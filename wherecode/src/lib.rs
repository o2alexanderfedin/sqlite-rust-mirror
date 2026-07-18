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
mod where_int_h;
pub(crate) use crate::where_int_h::*;

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

impl WhereInfo {
    fn b_deferred_seek(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_b_deferred_seek(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn untested_terms(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_untested_terms(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn b_ordered_inner_loop(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_b_ordered_inner_loop(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn sorted(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_sorted(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn b_star_done(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_b_star_done(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn b_star_used(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_b_star_used(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
}

impl WhereLoopU0S1 {
    fn need_free(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_need_free(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn b_omit_offset(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_b_omit_offset(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn b_idx_num_hex(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_b_idx_num_hex(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
}

extern "C" fn explain_index_column_name(p_idx_1: &Index, mut i: i32)
    -> *const i8 {
    i = unsafe { *(*p_idx_1).ai_column.offset(i as isize) } as i32;
    if i == -2 { return c"<expr>".as_ptr() as *mut i8 as *const i8; }
    if i == -1 { return c"rowid".as_ptr() as *mut i8 as *const i8; }
    return unsafe {
                (*unsafe {
                            (*(*p_idx_1).p_table).a_col.offset(i as isize)
                        }).z_cn_name
            } as *const i8;
}

extern "C" fn explain_append_term(p_str_1: *mut StrAccum, p_idx_1: *mut Index,
    n_term_1: i32, i_term_1: i32, b_and_1: i32, z_op_1: *const i8) -> () {
    let mut i: i32 = 0;
    { let _ = 0; };
    if b_and_1 != 0 {
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c" AND ".as_ptr() as *mut i8 as *const i8, 5)
        };
    }
    if n_term_1 > 1 {
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c"(".as_ptr() as *mut i8 as *const i8, 1)
        };
    }
    {
        i = 0;
        '__b0: loop {
            if !(i < n_term_1) { break '__b0; }
            '__c0: loop {
                if i != 0 {
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c",".as_ptr() as *mut i8 as *const i8, 1)
                    };
                }
                unsafe {
                    sqlite3_str_appendall(p_str_1 as *mut Sqlite3Str,
                        explain_index_column_name(unsafe { &*p_idx_1 },
                            i_term_1 + i))
                };
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_term_1 > 1 {
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c")".as_ptr() as *mut i8 as *const i8, 1)
        };
    }
    unsafe { sqlite3_str_append(p_str_1 as *mut Sqlite3Str, z_op_1, 1) };
    if n_term_1 > 1 {
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c"(".as_ptr() as *mut i8 as *const i8, 1)
        };
    }
    {
        i = 0;
        '__b1: loop {
            if !(i < n_term_1) { break '__b1; }
            '__c1: loop {
                if i != 0 {
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c",".as_ptr() as *mut i8 as *const i8, 1)
                    };
                }
                unsafe {
                    sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                        c"?".as_ptr() as *mut i8 as *const i8, 1)
                };
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_term_1 > 1 {
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c")".as_ptr() as *mut i8 as *const i8, 1)
        };
    }
}

extern "C" fn explain_index_range(p_str_1: *mut StrAccum,
    p_loop_1: &WhereLoop) -> () {
    unsafe {
        let p_index: *mut Index = (*p_loop_1).u.btree.p_index;
        let n_eq: u16 = (*p_loop_1).u.btree.n_eq;
        let n_skip: u16 = (*p_loop_1).n_skip;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if n_eq as i32 == 0 &&
                (*p_loop_1).ws_flags & (32 | 16) as u32 == 0 as u32 {
            return;
        }
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c" (".as_ptr() as *mut i8 as *const i8, 2)
        };
        {
            i = 0;
            '__b2: loop {
                if !(i < n_eq as i32) { break '__b2; }
                '__c2: loop {
                    let z: *const i8 =
                        explain_index_column_name(unsafe { &*p_index }, i);
                    if i != 0 {
                        unsafe {
                            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                                c" AND ".as_ptr() as *mut i8 as *const i8, 5)
                        };
                    }
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            if i >= n_skip as i32 {
                                    c"%s=?".as_ptr() as *mut i8
                                } else { c"ANY(%s)".as_ptr() as *mut i8 } as *const i8, z)
                    };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        j = i;
        if (*p_loop_1).ws_flags & 32 as u32 != 0 {
            explain_append_term(p_str_1, p_index,
                (*p_loop_1).u.btree.n_btm as i32, j, i,
                c">".as_ptr() as *mut i8 as *const i8);
            i = 1;
        }
        if (*p_loop_1).ws_flags & 16 as u32 != 0 {
            explain_append_term(p_str_1, p_index,
                (*p_loop_1).u.btree.n_top as i32, j, i,
                c"<".as_ptr() as *mut i8 as *const i8);
        }
        unsafe {
            sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                c")".as_ptr() as *mut i8 as *const i8, 1)
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_add_explain_text(p_parse: *mut Parse,
    addr: i32, p_tab_list: &mut SrcList, p_level: &WhereLevel,
    wctrl_flags: u16) -> () {
    unsafe {
        if unsafe {
                            (*if !(unsafe { (*p_parse).p_toplevel }).is_null() {
                                            unsafe { (*p_parse).p_toplevel }
                                        } else { p_parse }).explain
                        } as i32 == 2 || 0 != 0 {
            let p_op: *mut VdbeOp =
                unsafe {
                    sqlite3_vdbe_get_op(unsafe { (*p_parse).p_vdbe }, addr)
                };
            let p_item: *mut SrcItem =
                unsafe {
                    &mut *((*p_tab_list).a.as_ptr() as
                                    *mut SrcItem).add((*p_level).i_from as usize)
                };
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            let mut is_search: i32 = 0;
            let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
            let mut flags: u32 = 0 as u32;
            let mut str: StrAccum = unsafe { core::mem::zeroed() };
            let mut z_buf: [i8; 100] = [0; 100];
            if unsafe { (*db).malloc_failed } != 0 { return; }
            p_loop = (*p_level).p_w_loop;
            flags = unsafe { (*p_loop).ws_flags };
            is_search =
                (flags & (32 | 16) as u32 != 0 as u32 ||
                            flags & 1024 as u32 == 0 as u32 &&
                                unsafe { (*p_loop).u.btree.n_eq } as i32 > 0 ||
                        wctrl_flags as i32 & (1 | 2) != 0) as i32;
            unsafe {
                sqlite3_str_accum_init(&mut str, db,
                    &raw mut z_buf[0 as usize] as *mut i8,
                    core::mem::size_of::<[i8; 100]>() as i32, 1000000000)
            };
            str.printf_flags = 1 as u8;
            unsafe {
                sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                    c"%s %S%s".as_ptr() as *mut i8 as *const i8,
                    if is_search != 0 {
                        c"SEARCH".as_ptr() as *mut i8
                    } else { c"SCAN".as_ptr() as *mut i8 }, p_item,
                    if unsafe { (*p_item).fg.from_exists() } != 0 {
                        c" EXISTS".as_ptr() as *mut i8
                    } else { c"".as_ptr() as *mut i8 })
            };
            if flags & (256 | 1024) as u32 == 0 as u32 {
                let mut z_fmt: *const i8 = core::ptr::null();
                let mut p_idx: *const Index = core::ptr::null();
                { let _ = 0; };
                p_idx = unsafe { (*p_loop).u.btree.p_index };
                { let _ = 0; };
                if !(unsafe { (*unsafe { (*p_item).p_s_tab }).tab_flags } &
                                            128 as u32 == 0 as u32) as i32 != 0 &&
                        unsafe { (*p_idx).idx_type() } as i32 == 2 {
                    if is_search != 0 {
                        z_fmt = c"PRIMARY KEY".as_ptr() as *mut i8 as *const i8;
                    }
                } else if flags & 131072 as u32 != 0 {
                    z_fmt =
                        c"AUTOMATIC PARTIAL COVERING INDEX".as_ptr() as *mut i8 as
                            *const i8;
                } else if flags & 16384 as u32 != 0 {
                    z_fmt =
                        c"AUTOMATIC COVERING INDEX".as_ptr() as *mut i8 as
                            *const i8;
                } else if flags & (64 | 67108864) as u32 != 0 {
                    z_fmt =
                        c"COVERING INDEX %s".as_ptr() as *mut i8 as *const i8;
                } else {
                    z_fmt = c"INDEX %s".as_ptr() as *mut i8 as *const i8;
                }
                if !(z_fmt).is_null() {
                    unsafe {
                        sqlite3_str_append(&raw mut str as *mut Sqlite3Str,
                            c" USING ".as_ptr() as *mut i8 as *const i8, 7)
                    };
                    unsafe {
                        sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str, z_fmt,
                            unsafe { (*p_idx).z_name })
                    };
                    explain_index_range(&mut str, unsafe { &*p_loop });
                }
            } else if flags & 256 as u32 != 0 as u32 &&
                    flags & 15 as u32 != 0 as u32 {
                let mut c_range_op: i8 = 0 as i8;
                let z_rowid: *const i8 =
                    c"rowid".as_ptr() as *mut i8 as *const i8;
                unsafe {
                    sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                        c" USING INTEGER PRIMARY KEY (%s".as_ptr() as *mut i8 as
                            *const i8, z_rowid)
                };
                if flags & (1 | 4) as u32 != 0 {
                    c_range_op = '=' as i32 as i8;
                } else if flags & 48 as u32 == 48 as u32 {
                    unsafe {
                        sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                            c">? AND %s".as_ptr() as *mut i8 as *const i8, z_rowid)
                    };
                    c_range_op = '<' as i32 as i8;
                } else if flags & 32 as u32 != 0 {
                    c_range_op = '>' as i32 as i8;
                } else { { let _ = 0; }; c_range_op = '<' as i32 as i8; }
                unsafe {
                    sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                        c"%c?)".as_ptr() as *mut i8 as *const i8, c_range_op as i32)
                };
            } else if flags & 1024 as u32 != 0 as u32 {
                unsafe {
                    sqlite3_str_appendall(&raw mut str as *mut Sqlite3Str,
                        c" VIRTUAL TABLE INDEX ".as_ptr() as *mut i8 as *const i8)
                };
                unsafe {
                    sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                        if unsafe { (*p_loop).u.vtab.b_idx_num_hex() } != 0 {
                                c"0x%x:%s".as_ptr() as *mut i8
                            } else { c"%d:%s".as_ptr() as *mut i8 } as *const i8,
                        unsafe { (*p_loop).u.vtab.idx_num },
                        unsafe { (*p_loop).u.vtab.idx_str })
                };
            }
            if unsafe { (*p_item).fg.jointype } as i32 & 8 != 0 {
                unsafe {
                    sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                        c" LEFT-JOIN".as_ptr() as *mut i8 as *const i8)
                };
            }
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                sqlite3_db_free(db, unsafe { (*p_op).p4.z } as *mut ())
            };
            unsafe { (*p_op).p4type = -7 as i8 };
            unsafe {
                (*p_op).p4.z = unsafe { sqlite3_str_accum_finish(&mut str) }
            };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_explain_one_scan(p_parse: *mut Parse,
    p_tab_list: *mut SrcList, p_level: *mut WhereLevel, wctrl_flags: u16)
    -> i32 {
    let mut ret: i32 = 0;
    if unsafe {
                        (*if !(unsafe { (*p_parse).p_toplevel }).is_null() {
                                        unsafe { (*p_parse).p_toplevel }
                                    } else { p_parse }).explain
                    } as i32 == 2 || 0 != 0 {
        if unsafe { (*unsafe { (*p_level).p_w_loop }).ws_flags } & 8192 as u32
                    == 0 as u32 && wctrl_flags as i32 & 32 == 0 {
            let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) };
            ret =
                unsafe {
                    sqlite3_vdbe_add_op3(v, 190, addr,
                        unsafe { (*p_parse).addr_explain },
                        unsafe { (*unsafe { (*p_level).p_w_loop }).r_run } as i32)
                };
            sqlite3_where_add_explain_text(p_parse, addr,
                unsafe { &mut *p_tab_list }, unsafe { &*p_level },
                wctrl_flags);
        }
    }
    return ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_explain_bloom_filter(p_parse: &Parse,
    p_w_info: &WhereInfo, p_level: &WhereLevel) -> i32 {
    unsafe {
        let mut ret: i32 = 0;
        let p_item: *mut SrcItem =
            unsafe {
                &mut *(unsafe { (*(*p_w_info).p_tab_list).a.as_ptr() } as
                                *mut SrcItem).add((*p_level).i_from as usize)
            };
        let v: *mut Vdbe = (*p_parse).p_vdbe;
        let db: *mut Sqlite3 = (*p_parse).db;
        let mut z_msg: *const i8 = core::ptr::null();
        let mut i: i32 = 0;
        let mut p_loop: *const WhereLoop = core::ptr::null();
        let mut str: StrAccum = unsafe { core::mem::zeroed() };
        let mut z_buf: [i8; 100] = [0; 100];
        unsafe {
            sqlite3_str_accum_init(&mut str, db,
                &raw mut z_buf[0 as usize] as *mut i8,
                core::mem::size_of::<[i8; 100]>() as i32, 1000000000)
        };
        str.printf_flags = 1 as u8;
        unsafe {
            sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                c"BLOOM FILTER ON %S (".as_ptr() as *mut i8 as *const i8,
                p_item)
        };
        p_loop = (*p_level).p_w_loop;
        if unsafe { (*p_loop).ws_flags } & 256 as u32 != 0 {
            let p_tab: *const Table =
                unsafe { (*p_item).p_s_tab } as *const Table;
            if unsafe { (*p_tab).i_p_key } as i32 >= 0 {
                unsafe {
                    sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                        c"%s=?".as_ptr() as *mut i8 as *const i8,
                        unsafe {
                            (*unsafe {
                                        (*p_tab).a_col.offset(unsafe { (*p_tab).i_p_key } as isize)
                                    }).z_cn_name
                        })
                };
            } else {
                unsafe {
                    sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                        c"rowid=?".as_ptr() as *mut i8 as *const i8)
                };
            }
        } else {
            {
                i = unsafe { (*p_loop).n_skip } as i32;
                '__b3: loop {
                    if !(i < unsafe { (*p_loop).u.btree.n_eq } as i32) {
                        break '__b3;
                    }
                    '__c3: loop {
                        let z: *const i8 =
                            explain_index_column_name(unsafe {
                                    &*unsafe { (*p_loop).u.btree.p_index }
                                }, i);
                        if i > unsafe { (*p_loop).n_skip } as i32 {
                            unsafe {
                                sqlite3_str_append(&raw mut str as *mut Sqlite3Str,
                                    c" AND ".as_ptr() as *mut i8 as *const i8, 5)
                            };
                        }
                        unsafe {
                            sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str,
                                c"%s=?".as_ptr() as *mut i8 as *const i8, z)
                        };
                        break '__c3;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        unsafe {
            sqlite3_str_append(&raw mut str as *mut Sqlite3Str,
                c")".as_ptr() as *mut i8 as *const i8, 1)
        };
        z_msg = unsafe { sqlite3_str_accum_finish(&mut str) };
        ret =
            unsafe {
                sqlite3_vdbe_add_op4(v, 190,
                    unsafe { sqlite3_vdbe_current_addr(v) },
                    (*p_parse).addr_explain, 0, z_msg as *const i8, -7)
            };
        return ret;
    }
}

extern "C" fn disable_term(p_level_1: &WhereLevel,
    mut p_term_1: *mut WhereTerm) -> () {
    let mut n_loop: i32 = 0;
    { let _ = 0; };
    while unsafe { (*p_term_1).wt_flags } as i32 & 4 == 0 &&
                ((*p_level_1).i_left_join == 0 ||
                    unsafe { (*unsafe { (*p_term_1).p_expr }).flags } & 1 as u32
                        != 0 as u32) &&
            (*p_level_1).not_ready & unsafe { (*p_term_1).prereq_all } ==
                0 as u64 {
        if n_loop != 0 && unsafe { (*p_term_1).wt_flags } as i32 & 1024 != 0 {
            unsafe { (*p_term_1).wt_flags |= 512 as u16 };
        } else { unsafe { (*p_term_1).wt_flags |= 4 as u16 }; }
        if unsafe { (*p_term_1).i_parent } < 0 { break; }
        p_term_1 =
            unsafe {
                unsafe {
                    (*unsafe {
                                        (*p_term_1).p_wc
                                    }).a.offset(unsafe { (*p_term_1).i_parent } as isize)
                }
            };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p_term_1).n_child };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p_term_1).n_child } as i32 != 0 { break; }
        { let __p = &mut n_loop; let __t = *__p; *__p += 1; __t };
    }
}

extern "C" fn adjust_order_by_col(p_order_by_1: *mut ExprList,
    p_e_list_1: &ExprList) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if p_order_by_1 == core::ptr::null_mut() { return; }
        {
            i = 0;
            '__b5: loop {
                if !(i < unsafe { (*p_order_by_1).n_expr }) { break '__b5; }
                '__c5: loop {
                    let t: i32 =
                        unsafe {
                                (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                            } as i32;
                    if t == 0 { break '__c5; }
                    {
                        j = 0;
                        '__b6: loop {
                            if !(j < (*p_e_list_1).n_expr) { break '__b6; }
                            '__c6: loop {
                                if unsafe {
                                                (*((*p_e_list_1).a.as_ptr() as
                                                                        *mut ExprListItem).offset(j as isize)).u.x.i_order_by_col
                                            } as i32 == t {
                                    unsafe {
                                        (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col =
                                            (j + 1) as u16
                                    };
                                    break '__b6;
                                }
                                break '__c6;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if j >= (*p_e_list_1).n_expr {
                        unsafe {
                            (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col =
                                0 as u16
                        };
                    }
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn remove_unindexable_in_clause_terms(p_parse_1: *mut Parse,
    i_eq_1: i32, p_loop_1: &WhereLoop, p_x_1: *mut Expr) -> *mut Expr {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let mut p_select: *mut Select = core::ptr::null_mut();
        let mut p_new: *mut Expr = core::ptr::null_mut();
        p_new = unsafe { sqlite3_expr_dup(db, p_x_1 as *const Expr, 0) };
        if unsafe { (*db).malloc_failed } as i32 == 0 {
            {
                p_select = unsafe { (*p_new).x.p_select };
                '__b7: loop {
                    if !(!(p_select).is_null()) { break '__b7; }
                    '__c7: loop {
                        let mut p_orig_rhs: *mut ExprList = core::ptr::null_mut();
                        let mut p_orig_lhs: *mut ExprList = core::ptr::null_mut();
                        let mut p_rhs: *mut ExprList = core::ptr::null_mut();
                        let mut p_lhs: *mut ExprList = core::ptr::null_mut();
                        let mut i: i32 = 0;
                        { let _ = 0; };
                        p_orig_rhs = unsafe { (*p_select).p_e_list };
                        { let _ = 0; };
                        { let _ = 0; };
                        if p_select == unsafe { (*p_new).x.p_select } {
                            p_orig_lhs =
                                unsafe { (*unsafe { (*p_new).p_left }).x.p_list };
                        }
                        {
                            i = i_eq_1;
                            '__b8: loop {
                                if !(i < (*p_loop_1).n_l_term as i32) { break '__b8; }
                                '__c8: loop {
                                    if unsafe {
                                                (*unsafe {
                                                                *(*p_loop_1).a_l_term.offset(i as isize)
                                                            }).p_expr
                                            } == p_x_1 {
                                        let mut i_field: i32 = 0;
                                        { let _ = 0; };
                                        i_field =
                                            unsafe {
                                                    (*unsafe {
                                                                            *(*p_loop_1).a_l_term.offset(i as isize)
                                                                        }).u.x.i_field
                                                } - 1;
                                        if unsafe {
                                                    (*(unsafe { (*p_orig_rhs).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i_field as isize)).p_expr
                                                } == core::ptr::null_mut() {
                                            break '__c8;
                                        }
                                        p_rhs =
                                            unsafe {
                                                sqlite3_expr_list_append(p_parse_1, p_rhs,
                                                    unsafe {
                                                        (*(unsafe { (*p_orig_rhs).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(i_field as isize)).p_expr
                                                    })
                                            };
                                        unsafe {
                                            (*(unsafe { (*p_orig_rhs).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i_field as isize)).p_expr =
                                                core::ptr::null_mut()
                                        };
                                        if !(p_rhs).is_null() {
                                            unsafe {
                                                (*(unsafe { (*p_rhs).a.as_ptr() } as
                                                                            *mut ExprListItem).offset((unsafe { (*p_rhs).n_expr } - 1)
                                                                            as isize)).u.x.i_order_by_col = (i_field + 1) as u16
                                            };
                                        }
                                        if !(p_orig_lhs).is_null() {
                                            { let _ = 0; };
                                            p_lhs =
                                                unsafe {
                                                    sqlite3_expr_list_append(p_parse_1, p_lhs,
                                                        unsafe {
                                                            (*(unsafe { (*p_orig_lhs).a.as_ptr() } as
                                                                            *mut ExprListItem).offset(i_field as isize)).p_expr
                                                        })
                                                };
                                            unsafe {
                                                (*(unsafe { (*p_orig_lhs).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i_field as isize)).p_expr =
                                                    core::ptr::null_mut()
                                            };
                                        }
                                    }
                                    break '__c8;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_expr_list_delete(db, p_orig_rhs) };
                        if !(p_orig_lhs).is_null() {
                            unsafe { sqlite3_expr_list_delete(db, p_orig_lhs) };
                            unsafe { (*unsafe { (*p_new).p_left }).x.p_list = p_lhs };
                        }
                        unsafe { (*p_select).p_e_list = p_rhs };
                        unsafe {
                            (*p_select).sel_id =
                                {
                                        let __p = unsafe { &mut (*p_parse_1).n_select };
                                        *__p += 1;
                                        *__p
                                    } as u32
                        };
                        if !(p_lhs).is_null() && unsafe { (*p_lhs).n_expr } == 1 {
                            let p: *mut Expr =
                                unsafe {
                                    (*(unsafe { (*p_lhs).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                };
                            unsafe {
                                (*(unsafe { (*p_lhs).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr =
                                    core::ptr::null_mut()
                            };
                            unsafe {
                                sqlite3_expr_delete(db, unsafe { (*p_new).p_left })
                            };
                            unsafe { (*p_new).p_left = p };
                        }
                        { let _ = 0; };
                        if !(p_rhs).is_null() {
                            adjust_order_by_col(unsafe { (*p_select).p_order_by },
                                unsafe { &*p_rhs });
                            adjust_order_by_col(unsafe { (*p_select).p_group_by },
                                unsafe { &*p_rhs });
                            {
                                i = 0;
                                '__b9: loop {
                                    if !(i < unsafe { (*p_rhs).n_expr }) { break '__b9; }
                                    '__c9: loop {
                                        unsafe {
                                            (*(unsafe { (*p_rhs).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col =
                                                0 as u16
                                        };
                                        break '__c9;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        break '__c7;
                    }
                    p_select = unsafe { (*p_select).p_prior };
                }
            }
        }
        return p_new;
    }
}

extern "C" fn code_in_term(p_parse_1: *mut Parse, p_term_1: *mut WhereTerm,
    p_level_1: *mut WhereLevel, i_eq_1: i32, mut b_rev_1: i32,
    i_target_1: i32) -> () {
    unsafe {
        let p_x: *mut Expr = unsafe { (*p_term_1).p_expr };
        let mut e_type: i32 = 5;
        let mut i_tab: i32 = 0;
        let mut p_in: *mut InLoop = core::ptr::null_mut();
        let p_loop: *mut WhereLoop = unsafe { (*p_level_1).p_w_loop };
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut n_eq: i32 = 0;
        let mut ai_map: *mut i32 = core::ptr::null_mut();
        if unsafe { (*p_loop).ws_flags } & 1024 as u32 == 0 as u32 &&
                    unsafe { (*p_loop).u.btree.p_index } !=
                        core::ptr::null_mut() &&
                unsafe {
                        *unsafe {
                                (*unsafe {
                                                    (*p_loop).u.btree.p_index
                                                }).a_sort_order.offset(i_eq_1 as isize)
                            }
                    } != 0 {
            b_rev_1 = (b_rev_1 == 0) as i32 as i32;
        }
        { let _ = 0; };
        {
            i = 0;
            '__b10: loop {
                if !(i < i_eq_1) { break '__b10; }
                '__c10: loop {
                    if !(unsafe {
                                            *unsafe { (*p_loop).a_l_term.offset(i as isize) }
                                        }).is_null() &&
                            unsafe {
                                    (*unsafe {
                                                    *unsafe { (*p_loop).a_l_term.offset(i as isize) }
                                                }).p_expr
                                } == p_x {
                        disable_term(unsafe { &*p_level_1 }, p_term_1);
                        return;
                    }
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = i_eq_1;
            '__b11: loop {
                if !(i < unsafe { (*p_loop).n_l_term } as i32) {
                    break '__b11;
                }
                '__c11: loop {
                    { let _ = 0; };
                    if unsafe {
                                (*unsafe {
                                                *unsafe { (*p_loop).a_l_term.offset(i as isize) }
                                            }).p_expr
                            } == p_x {
                        { let __p = &mut n_eq; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c11;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        i_tab = 0;
        if !(unsafe { (*p_x).flags } & 4096 as u32 != 0 as u32) as i32 != 0 ||
                unsafe {
                        (*unsafe {
                                        (*unsafe { (*p_x).x.p_select }).p_e_list
                                    }).n_expr
                    } == 1 {
            e_type =
                unsafe {
                    sqlite3_find_in_index(p_parse_1, p_x, 4 as u32,
                        core::ptr::null_mut(), core::ptr::null_mut(), &mut i_tab)
                };
        } else {
            let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
            let p_x_mod: *mut Expr =
                remove_unindexable_in_clause_terms(p_parse_1, i_eq_1,
                    unsafe { &*p_loop }, p_x);
            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                ai_map =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                core::mem::size_of::<i32>() as u64 * n_eq as u64)
                        } as *mut i32;
                e_type =
                    unsafe {
                        sqlite3_find_in_index(p_parse_1, p_x_mod, 4 as u32,
                            core::ptr::null_mut(), ai_map, &mut i_tab)
                    };
            }
            unsafe { sqlite3_expr_delete(db, p_x_mod) };
        }
        if e_type == 4 { b_rev_1 = (b_rev_1 == 0) as i32 as i32; }
        unsafe {
            sqlite3_vdbe_add_op2(v, if b_rev_1 != 0 { 32 } else { 36 }, i_tab,
                0)
        };
        { let _ = 0; };
        unsafe { (*p_loop).ws_flags |= 2048 as u32 };
        if unsafe { (*p_level_1).u.in_.n_in } == 0 {
            unsafe {
                (*p_level_1).addr_nxt =
                    unsafe { sqlite3_vdbe_make_label(p_parse_1) }
            };
        }
        if i_eq_1 > 0 &&
                unsafe { (*p_loop).ws_flags } & 1048576 as u32 == 0 as u32 {
            unsafe { (*p_loop).ws_flags |= 262144 as u32 };
        }
        i = unsafe { (*p_level_1).u.in_.n_in };
        unsafe { (*p_level_1).u.in_.n_in += n_eq };
        unsafe {
            (*p_level_1).u.in_.a_in_loop =
                unsafe {
                        sqlite3_where_realloc(unsafe {
                                (*unsafe { (*p_term_1).p_wc }).p_w_info
                            }, unsafe { (*p_level_1).u.in_.a_in_loop } as *mut (),
                            core::mem::size_of::<InLoop>() as u64 *
                                unsafe { (*p_level_1).u.in_.n_in } as u64)
                    } as *mut InLoop
        };
        p_in = unsafe { (*p_level_1).u.in_.a_in_loop };
        if !(p_in).is_null() {
            let mut i_map: i32 = 0;
            {
                let __n = i;
                let __p = &mut p_in;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            {
                i = i_eq_1;
                '__b12: loop {
                    if !(i < unsafe { (*p_loop).n_l_term } as i32) {
                        break '__b12;
                    }
                    '__c12: loop {
                        if unsafe {
                                    (*unsafe {
                                                    *unsafe { (*p_loop).a_l_term.offset(i as isize) }
                                                }).p_expr
                                } == p_x {
                            let i_out: i32 = i_target_1 + i - i_eq_1;
                            if e_type == 1 {
                                unsafe {
                                    (*p_in).addr_in_top =
                                        unsafe { sqlite3_vdbe_add_op2(v, 137, i_tab, i_out) }
                                };
                            } else {
                                let i_col: i32 =
                                    if !(ai_map).is_null() {
                                        unsafe {
                                            *ai_map.offset({
                                                            let __p = &mut i_map;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize)
                                        }
                                    } else { 0 };
                                unsafe {
                                    (*p_in).addr_in_top =
                                        unsafe { sqlite3_vdbe_add_op3(v, 96, i_tab, i_col, i_out) }
                                };
                            }
                            unsafe { sqlite3_vdbe_add_op1(v, 51, i_out) };
                            if i == i_eq_1 {
                                unsafe { (*p_in).i_cur = i_tab };
                                unsafe {
                                    (*p_in).e_end_loop_op =
                                        if b_rev_1 != 0 { 39 } else { 40 } as u8
                                };
                                if i_eq_1 > 0 {
                                    unsafe { (*p_in).i_base = i_target_1 - i };
                                    unsafe { (*p_in).n_prefix = i };
                                } else { unsafe { (*p_in).n_prefix = 0 }; }
                            } else { unsafe { (*p_in).e_end_loop_op = 189 as u8 }; }
                            {
                                let __p = &mut p_in;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        break '__c12;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i_eq_1 > 0 &&
                    unsafe { (*p_loop).ws_flags } & (1048576 | 1024) as u32 ==
                        0 as u32 {
                unsafe {
                    sqlite3_vdbe_add_op3(v, 127,
                        unsafe { (*p_level_1).i_idx_cur }, 0, i_eq_1)
                };
            }
        } else { unsafe { (*p_level_1).u.in_.n_in = 0 }; }
        unsafe {
            sqlite3_db_free(unsafe { (*p_parse_1).db }, ai_map as *mut ())
        };
    }
}

extern "C" fn code_equality_term(p_parse_1: *mut Parse,
    p_term_1: *mut WhereTerm, p_level_1: *mut WhereLevel, i_eq_1: i32,
    b_rev_1: i32, i_target_1: i32) -> i32 {
    let p_x: *const Expr = unsafe { (*p_term_1).p_expr } as *const Expr;
    let mut i_reg: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_x).op } as i32 == 54 || unsafe { (*p_x).op } as i32 == 45
        {
        i_reg =
            unsafe {
                sqlite3_expr_code_target(p_parse_1, unsafe { (*p_x).p_right },
                    i_target_1)
            };
    } else if unsafe { (*p_x).op } as i32 == 51 {
        i_reg = i_target_1;
        unsafe {
            sqlite3_vdbe_add_op2(unsafe { (*p_parse_1).p_vdbe }, 77, 0, i_reg)
        };
    } else {
        { let _ = 0; };
        i_reg = i_target_1;
        code_in_term(p_parse_1, p_term_1, p_level_1, i_eq_1, b_rev_1,
            i_target_1);
    }
    if unsafe { (*unsafe { (*p_level_1).p_w_loop }).ws_flags } &
                    2097152 as u32 == 0 as u32 ||
            unsafe { (*p_term_1).e_operator } as i32 & 2048 == 0 {
        disable_term(unsafe { &*p_level_1 }, p_term_1);
    }
    return i_reg;
}

extern "C" fn code_expr_or_vector(p_parse_1: *mut Parse, p: *mut Expr,
    i_reg_1: i32, n_reg_1: i32) -> () {
    unsafe {
        { let _ = 0; };
        if !(p).is_null() &&
                unsafe { sqlite3_expr_is_vector(p as *const Expr) } != 0 {
            if unsafe { (*p).flags } & 4096 as u32 != 0 as u32 {
                let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
                let mut i_select: i32 = 0;
                { let _ = 0; };
                i_select = unsafe { sqlite3_code_subselect(p_parse_1, p) };
                unsafe {
                    sqlite3_vdbe_add_op3(v, 82, i_select, i_reg_1, n_reg_1 - 1)
                };
            } else {
                let mut i: i32 = 0;
                let mut p_list: *const ExprList = core::ptr::null();
                { let _ = 0; };
                p_list = unsafe { (*p).x.p_list } as *const ExprList;
                { let _ = 0; };
                {
                    i = 0;
                    '__b13: loop {
                        if !(i < n_reg_1) { break '__b13; }
                        '__c13: loop {
                            unsafe {
                                sqlite3_expr_code(p_parse_1,
                                    unsafe {
                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *const ExprListItem).offset(i as isize)).p_expr
                                    }, i_reg_1 + i)
                            };
                            break '__c13;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        } else {
            { let _ = 0; };
            unsafe { sqlite3_expr_code(p_parse_1, p, i_reg_1) };
        }
    }
}

extern "C" fn code_all_equality_terms(p_parse_1: *mut Parse,
    p_level_1: *mut WhereLevel, b_rev_1: i32, n_extra_reg_1: i32,
    pz_aff_1: &mut *mut i8) -> i32 {
    unsafe {
        let mut n_eq: u16 = 0 as u16;
        let mut n_skip: u16 = 0 as u16;
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut p_idx: *mut Index = core::ptr::null_mut();
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        let mut p_loop: *const WhereLoop = core::ptr::null();
        let mut j: i32 = 0;
        let mut reg_base: i32 = 0;
        let mut n_reg: i32 = 0;
        let mut z_aff: *mut i8 = core::ptr::null_mut();
        p_loop = unsafe { (*p_level_1).p_w_loop };
        { let _ = 0; };
        n_eq = unsafe { (*p_loop).u.btree.n_eq };
        n_skip = unsafe { (*p_loop).n_skip };
        p_idx = unsafe { (*p_loop).u.btree.p_index };
        { let _ = 0; };
        reg_base = unsafe { (*p_parse_1).n_mem } + 1;
        n_reg = n_eq as i32 + n_extra_reg_1;
        unsafe { (*p_parse_1).n_mem += n_reg };
        z_aff =
            unsafe {
                sqlite3_db_str_dup(unsafe { (*p_parse_1).db },
                    unsafe {
                        sqlite3_index_affinity_str(unsafe { (*p_parse_1).db },
                            p_idx)
                    })
            };
        { let _ = 0; };
        if n_skip != 0 {
            let i_idx_cur: i32 = unsafe { (*p_level_1).i_idx_cur };
            unsafe {
                sqlite3_vdbe_add_op3(v, 77, 0, reg_base,
                    reg_base + n_skip as i32 - 1)
            };
            unsafe {
                sqlite3_vdbe_add_op1(v, if b_rev_1 != 0 { 32 } else { 36 },
                    i_idx_cur)
            };
            j = unsafe { sqlite3_vdbe_add_op0(v, 9) };
            { let _ = 0; };
            unsafe {
                (*p_level_1).addr_skip =
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v,
                            if b_rev_1 != 0 { 21 } else { 24 }, i_idx_cur, 0, reg_base,
                            n_skip as i32)
                    }
            };
            unsafe { sqlite3_vdbe_jump_here(v, j) };
            {
                j = 0;
                '__b14: loop {
                    if !(j < n_skip as i32) { break '__b14; }
                    '__c14: loop {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_idx_cur, j, reg_base + j)
                        };
                        break '__c14;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        { let _ = 0; };
        {
            j = n_skip as i32;
            '__b15: loop {
                if !(j < n_eq as i32) { break '__b15; }
                '__c15: loop {
                    let mut r1: i32 = 0;
                    p_term =
                        unsafe {
                            *unsafe { (*p_loop).a_l_term.offset(j as isize) }
                        };
                    { let _ = 0; };
                    r1 =
                        code_equality_term(p_parse_1, p_term, p_level_1, j, b_rev_1,
                            reg_base + j);
                    if r1 != reg_base + j {
                        if n_reg == 1 {
                            unsafe { sqlite3_release_temp_reg(p_parse_1, reg_base) };
                            reg_base = r1;
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 82, r1, reg_base + j) };
                        }
                    }
                    if unsafe { (*p_term).e_operator } as i32 & 1 != 0 {
                        if unsafe { (*unsafe { (*p_term).p_expr }).flags } &
                                    4096 as u32 != 0 {
                            if !(z_aff).is_null() {
                                unsafe { *z_aff.offset(j as isize) = 65 as i8 };
                            }
                        }
                    } else if unsafe { (*p_term).e_operator } as i32 & 256 == 0
                        {
                        let p_right: *const Expr =
                            unsafe { (*unsafe { (*p_term).p_expr }).p_right } as
                                *const Expr;
                        if unsafe { (*p_term).wt_flags } as i32 & 2048 == 0 &&
                                unsafe { sqlite3_expr_can_be_null(p_right as *const Expr) }
                                    != 0 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_base + j,
                                    unsafe { (*p_level_1).addr_brk })
                            };
                        }
                        if unsafe { (*p_parse_1).n_err } == 0 {
                            { let _ = 0; };
                            if unsafe {
                                            sqlite3_compare_affinity(p_right as *const Expr,
                                                unsafe { *z_aff.offset(j as isize) })
                                        } as i32 == 65 {
                                unsafe { *z_aff.offset(j as isize) = 65 as i8 };
                            }
                            if unsafe {
                                        sqlite3_expr_needs_no_affinity_change(p_right as
                                                *const Expr, unsafe { *z_aff.offset(j as isize) })
                                    } != 0 {
                                unsafe { *z_aff.offset(j as isize) = 65 as i8 };
                            }
                        }
                    }
                    break '__c15;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        *pz_aff_1 = z_aff;
        return reg_base;
    }
}

extern "C" fn code_apply_affinity(p_parse_1: &Parse, mut base: i32,
    mut n: i32, mut z_aff_1: *const i8) -> () {
    let v: *mut Vdbe = (*p_parse_1).p_vdbe;
    if z_aff_1 == core::ptr::null_mut() { { let _ = 0; }; return; }
    { let _ = 0; };
    { let _ = 0; };
    while n > 0 && unsafe { *z_aff_1.offset(0 as isize) } as i32 <= 65 {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
        { let __p = &mut base; let __t = *__p; *__p += 1; __t };
        {
            let __p = &mut z_aff_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    while n > 1 && unsafe { *z_aff_1.offset((n - 1) as isize) } as i32 <= 65 {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    if n > 0 {
        unsafe {
            sqlite3_vdbe_add_op4(v, 98, base, n, 0, z_aff_1 as *const i8, n)
        };
    }
}

extern "C" fn filter_pull_down(p_parse_1: *mut Parse,
    p_w_info_1: &mut WhereInfo, mut i_level_1: i32, addr_nxt_1: i32,
    not_ready_1: Bitmask) -> () {
    unsafe {
        let mut saved_addr_brk: i32 = 0;
        while { let __p = &mut i_level_1; *__p += 1; *__p } <
                (*p_w_info_1).n_level as i32 {
            let p_level: *mut WhereLevel =
                unsafe {
                    &mut *((*p_w_info_1).a.as_ptr() as
                                    *mut WhereLevel).offset(i_level_1 as isize)
                };
            let p_loop: *const WhereLoop =
                unsafe { (*p_level).p_w_loop } as *const WhereLoop;
            if unsafe { (*p_level).reg_filter } == 0 { continue; }
            if unsafe { (*unsafe { (*p_level).p_w_loop }).n_skip } != 0 {
                continue;
            }
            if unsafe { (*p_loop).prereq } & not_ready_1 != 0 { continue; }
            saved_addr_brk = unsafe { (*p_level).addr_brk };
            unsafe { (*p_level).addr_brk = addr_nxt_1 };
            if unsafe { (*p_loop).ws_flags } & 256 as u32 != 0 {
                let p_term: *mut WhereTerm =
                    unsafe {
                        *unsafe { (*p_loop).a_l_term.offset(0 as isize) }
                    };
                let mut reg_rowid: i32 = 0;
                { let _ = 0; };
                { let _ = 0; };
                reg_rowid = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                reg_rowid =
                    code_equality_term(p_parse_1, p_term, p_level, 0, 0,
                        reg_rowid);
                unsafe {
                    sqlite3_vdbe_add_op2(unsafe { (*p_parse_1).p_vdbe }, 13,
                        reg_rowid, addr_nxt_1)
                };
                unsafe {
                    sqlite3_vdbe_add_op4_int(unsafe { (*p_parse_1).p_vdbe }, 66,
                        unsafe { (*p_level).reg_filter }, addr_nxt_1, reg_rowid, 1)
                };
            } else {
                let n_eq: u16 = unsafe { (*p_loop).u.btree.n_eq };
                let mut r1: i32 = 0;
                let mut z_start_aff: *mut i8 = core::ptr::null_mut();
                { let _ = 0; };
                { let _ = 0; };
                r1 =
                    code_all_equality_terms(p_parse_1, p_level, 0, 0,
                        &mut z_start_aff);
                code_apply_affinity(unsafe { &*p_parse_1 }, r1, n_eq as i32,
                    z_start_aff as *const i8);
                unsafe {
                    sqlite3_db_free(unsafe { (*p_parse_1).db },
                        z_start_aff as *mut ())
                };
                unsafe {
                    sqlite3_vdbe_add_op4_int(unsafe { (*p_parse_1).p_vdbe }, 66,
                        unsafe { (*p_level).reg_filter }, addr_nxt_1, r1,
                        n_eq as i32)
                };
            }
            unsafe { (*p_level).reg_filter = 0 };
            unsafe { (*p_level).addr_brk = saved_addr_brk };
        }
    }
}

extern "C" fn where_like_optimization_string_fixup(v: *mut Vdbe,
    p_level_1: &WhereLevel, p_term_1: &WhereTerm) -> () {
    if (*p_term_1).wt_flags as i32 & 256 != 0 {
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        { let _ = 0; };
        p_op = unsafe { sqlite3_vdbe_get_last_op(v) };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_op).p3 = ((*p_level_1).i_like_rep_cntr >> 1) as i32 };
        unsafe {
            (*p_op).p5 =
                ((*p_level_1).i_like_rep_cntr & 1 as u32) as u8 as u16
        };
    }
}

extern "C" fn update_range_affinity_str(p_right_1: *mut Expr,
    mut z_aff_1: &mut [i8]) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b19: loop {
            if !(i < z_aff_1.len() as i32) { break '__b19; }
            '__c19: loop {
                let p: *const Expr =
                    unsafe { sqlite3_vector_field_subexpr(p_right_1, i) } as
                        *const Expr;
                if unsafe {
                                    sqlite3_compare_affinity(p as *const Expr,
                                        z_aff_1[i as usize])
                                } as i32 == 65 ||
                        unsafe {
                                sqlite3_expr_needs_no_affinity_change(p as *const Expr,
                                    z_aff_1[i as usize])
                            } != 0 {
                    z_aff_1[i as usize] = 65 as i8;
                }
                break '__c19;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn code_deferred_seek(p_w_info_1: &mut WhereInfo, p_idx_1: &Index,
    i_cur_1: i32, i_idx_cur_1: i32) -> () {
    let p_parse: *const Parse = (*p_w_info_1).p_parse as *const Parse;
    let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
    { let _ = 0; };
    { let _ = 0; };
    (*p_w_info_1).set_b_deferred_seek(1 as u32 as u32);
    unsafe { sqlite3_vdbe_add_op3(v, 143, i_idx_cur_1, 0, i_cur_1) };
    if (*p_w_info_1).wctrl_flags as i32 & (32 | 4096) != 0 &&
            unsafe {
                    (*if !(unsafe { (*p_parse).p_toplevel }).is_null() {
                                    unsafe { (*p_parse).p_toplevel }
                                } else { p_parse }).write_mask
                } == 0 as u32 {
        let mut i: i32 = 0;
        let p_tab: *mut Table = (*p_idx_1).p_table;
        let ai: *mut u32 =
            unsafe {
                    sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                        core::mem::size_of::<u32>() as u64 *
                            (unsafe { (*p_tab).n_col } as i32 + 1) as u64)
                } as *mut u32;
        if !(ai).is_null() {
            unsafe {
                *ai.offset(0 as isize) = unsafe { (*p_tab).n_col } as u32
            };
            {
                i = 0;
                '__b20: loop {
                    if !(i < (*p_idx_1).n_column as i32 - 1) { break '__b20; }
                    '__c20: loop {
                        let mut x1: i32 = 0;
                        let mut x2: i32 = 0;
                        { let _ = 0; };
                        x1 =
                            unsafe { *(*p_idx_1).ai_column.offset(i as isize) } as i32;
                        x2 =
                            unsafe { sqlite3_table_column_to_storage(p_tab, x1 as i16) }
                                as i32;
                        if x1 >= 0 {
                            unsafe { *ai.offset((x2 + 1) as isize) = (i + 1) as u32 };
                        }
                        break '__c20;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe {
                sqlite3_vdbe_change_p4(v, -1, ai as *mut i8 as *const i8, -15)
            };
        }
    }
}

extern "C" fn where_apply_partial_index_constraints(mut p_truth_1:
        *const Expr, i_tab_cur_1: i32, p_wc_1: *mut WhereClause) -> () {
    let mut i: i32 = 0;
    let mut p_term: *mut WhereTerm = core::ptr::null_mut();
    while unsafe { (*p_truth_1).op } as i32 == 44 {
        where_apply_partial_index_constraints(unsafe { (*p_truth_1).p_left }
                as *const Expr, i_tab_cur_1, p_wc_1);
        p_truth_1 = unsafe { (*p_truth_1).p_right };
    }
    {
        { i = 0; p_term = unsafe { (*p_wc_1).a } };
        '__b22: loop {
            if !(i < unsafe { (*p_wc_1).n_term }) { break '__b22; }
            '__c22: loop {
                let mut p_expr: *const Expr = core::ptr::null();
                if unsafe { (*p_term).wt_flags } as i32 & 4 != 0 {
                    break '__c22;
                }
                p_expr = unsafe { (*p_term).p_expr };
                if unsafe {
                            sqlite3_expr_compare(core::ptr::null(),
                                p_expr as *const Expr, p_truth_1 as *const Expr,
                                i_tab_cur_1)
                        } == 0 {
                    unsafe { (*p_term).wt_flags |= 4 as u16 };
                }
                break '__c22;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_term;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
}

extern "C" fn where_loop_is_one_row(p_loop_1: &WhereLoop) -> i32 {
    unsafe {
        if unsafe { (*(*p_loop_1).u.btree.p_index).on_error } != 0 &&
                    (*p_loop_1).n_skip as i32 == 0 &&
                (*p_loop_1).u.btree.n_eq as i32 ==
                    unsafe { (*(*p_loop_1).u.btree.p_index).n_key_col } as i32 {
            let mut ii: i32 = 0;
            {
                ii = 0;
                '__b23: loop {
                    if !(ii < (*p_loop_1).u.btree.n_eq as i32) { break '__b23; }
                    '__c23: loop {
                        if unsafe {
                                            (*unsafe {
                                                            *(*p_loop_1).a_l_term.offset(ii as isize)
                                                        }).e_operator
                                        } as i32 & (128 | 256) != 0 {
                            return 0;
                        }
                        break '__c23;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            return 1;
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_code_one_loop_start(p_parse: *mut Parse,
    v: *mut Vdbe, p_w_info: *mut WhereInfo, i_level: i32,
    p_level: *mut WhereLevel, not_ready: Bitmask) -> Bitmask {
    unsafe {
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut i_cur: i32 = 0;
        let mut addr_nxt: i32 = 0;
        let mut b_rev: i32 = 0;
        let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
        let mut p_wc: *mut WhereClause = core::ptr::null_mut();
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_tab_item: *const SrcItem = core::ptr::null();
        let mut addr_brk: i32 = 0;
        let mut addr_cont: i32 = 0;
        let mut i_rowid_reg: i32 = 0;
        let mut i_release_reg: i32 = 0;
        let mut p_idx: *mut Index = core::ptr::null_mut();
        let mut i_loop: i32 = 0;
        let mut reg_yield: i32 = 0;
        let mut p_subq: *const Subquery = core::ptr::null();
        let mut i_reg: i32 = 0;
        let mut addr_not_found: i32 = 0;
        let mut n_constraint: i32 = 0;
        let mut i_target: i32 = 0;
        let mut i_tab: i32 = 0;
        let mut i_cache: i32 = 0;
        let mut p_right: *mut Expr = core::ptr::null_mut();
        let mut p_compare: *mut Expr = core::ptr::null_mut();
        let mut p_right_1: *mut Expr = core::ptr::null_mut();
        let mut p_op: *const VdbeOp = core::ptr::null();
        let mut i_in: i32 = 0;
        let mut i_fld: i32 = 0;
        let mut p_left: *mut Expr = core::ptr::null_mut();
        let mut test_op: i32 = 0;
        let mut start: i32 = 0;
        let mut mem_end_value: i32 = 0;
        let mut p_start: *mut WhereTerm = core::ptr::null_mut();
        let mut p_end: *mut WhereTerm = core::ptr::null_mut();
        let mut p_x: *const Expr = core::ptr::null();
        let mut r1: i32 = 0;
        let mut r_temp: i32 = 0;
        let mut op: i32 = 0;
        let a_move_op: [u8; 4] = [24 as u8, 22 as u8, 21 as u8, 23 as u8];
        let mut p_x_1: *const Expr = core::ptr::null();
        let mut n_eq: u16 = 0 as u16;
        let mut n_btm: u16 = 0 as u16;
        let mut n_top: u16 = 0 as u16;
        let mut reg_base: i32 = 0;
        let mut p_range_start: *mut WhereTerm = core::ptr::null_mut();
        let mut p_range_end: *mut WhereTerm = core::ptr::null_mut();
        let mut start_eq: i32 = 0;
        let mut end_eq: i32 = 0;
        let mut start_constraints: i32 = 0;
        let mut n_constraint_1: i32 = 0;
        let mut i_idx_cur: i32 = 0;
        let mut n_extra_reg: i32 = 0;
        let mut op__1: i32 = 0;
        let mut z_start_aff: *mut i8 = core::ptr::null_mut();
        let mut z_end_aff: *mut i8 = core::ptr::null_mut();
        let mut b_seek_past_null: u8 = 0 as u8;
        let mut b_stop_at_null: u8 = 0 as u8;
        let mut omit_table: i32 = 0;
        let mut reg_bignull: i32 = 0;
        let mut addr_seek_scan: i32 = 0;
        let mut t: *mut WhereTerm = core::ptr::null_mut();
        let mut t__1: u8 = 0 as u8;
        let mut t__2: u8 = 0 as u8;
        let mut p_right_2: *mut Expr = core::ptr::null_mut();
        let mut p_right_3: *mut Expr = core::ptr::null_mut();
        let mut p_pk: *const Index = core::ptr::null();
        let mut p_or_wc: *const WhereClause = core::ptr::null();
        let mut p_or_tab: *mut SrcList = core::ptr::null_mut();
        let mut p_cov: *mut Index = core::ptr::null_mut();
        let mut i_cov_cur: i32 = 0;
        let mut reg_return: i32 = 0;
        let mut reg_rowset: i32 = 0;
        let mut reg_rowid: i32 = 0;
        let mut i_loop_body: i32 = 0;
        let mut i_ret_init: i32 = 0;
        let mut untested_terms: i32 = 0;
        let mut ii: i32 = 0;
        let mut p_and_expr: *mut Expr = core::ptr::null_mut();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut n_not_ready: i32 = 0;
        let mut orig_src: *mut SrcItem = core::ptr::null_mut();
        let mut p_pk_1: *mut Index = core::ptr::null_mut();
        let mut i_term: i32 = 0;
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let mut p_or_term: *const WhereTerm = core::ptr::null();
        let mut p_sub_w_info: *mut WhereInfo = core::ptr::null_mut();
        let mut p_or_expr: *mut Expr = core::ptr::null_mut();
        let mut p_delete: *mut Expr = core::ptr::null_mut();
        let mut jmp1: i32 = 0;
        let mut p_sub_loop: *const WhereLoop = core::ptr::null();
        let mut addr_explain: i32 = 0;
        let mut i_set: i32 = 0;
        let mut p_pk_2: *const Index = core::ptr::null();
        let mut n_pk: i32 = 0;
        let mut i_pk: i32 = 0;
        let mut r: i32 = 0;
        let mut i_col: i32 = 0;
        let mut i_next: i32 = 0;
        let mut p_e: *mut Expr = core::ptr::null_mut();
        let mut skip_like_addr: i32 = 0;
        let mut m: Bitmask = 0 as Bitmask;
        let mut x: u32 = 0 as u32;
        let mut p_e_1: *const Expr = core::ptr::null();
        let mut s_e_alt: Expr = unsafe { core::mem::zeroed() };
        let mut p_alt: *mut WhereTerm = core::ptr::null_mut();
        let mut p_tab_1: *mut Table = core::ptr::null_mut();
        let mut n_pk_1: i32 = 0;
        let mut r__1: i32 = 0;
        let mut jmp1__1: i32 = 0;
        let mut p_rj: *const WhereRightJoin = core::ptr::null();
        let mut i_pk_1: i32 = 0;
        let mut p_pk_3: *const Index = core::ptr::null();
        let mut i_col_1: i32 = 0;
        let mut p_rj_1: *mut WhereRightJoin = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s25:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        { p_term = unsafe { (*p_wc).a }; j = 0 };
                        __state = 776;
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { i_rowid_reg = 0; __state = 14; }
                    14 => { i_release_reg = 0; __state = 15; }
                    15 => { p_idx = core::ptr::null_mut(); __state = 16; }
                    16 => { __state = 17; }
                    17 => {
                        p_wc = unsafe { &mut (*p_w_info).s_wc };
                        __state = 18;
                    }
                    18 => { db = unsafe { (*p_parse).db }; __state = 19; }
                    19 => {
                        p_loop = unsafe { (*p_level).p_w_loop };
                        __state = 20;
                    }
                    20 => {
                        p_tab_item =
                            unsafe {
                                &mut *(unsafe {
                                                    (*unsafe { (*p_w_info).p_tab_list }).a.as_ptr()
                                                } as
                                                *mut SrcItem).add(unsafe { (*p_level).i_from } as usize)
                            };
                        __state = 21;
                    }
                    21 => {
                        i_cur = unsafe { (*p_tab_item).i_cursor };
                        __state = 22;
                    }
                    22 => {
                        unsafe {
                            (*p_level).not_ready =
                                not_ready &
                                    !unsafe {
                                            sqlite3_where_get_mask(unsafe {
                                                    &mut (*p_w_info).s_mask_set
                                                }, i_cur)
                                        }
                        };
                        __state = 23;
                    }
                    23 => {
                        b_rev =
                            (unsafe { (*p_w_info).rev_mask } >> i_level & 1 as Bitmask)
                                as i32;
                        __state = 24;
                    }
                    24 => { __state = 25; }
                    25 => {
                        addr_brk =
                            {
                                unsafe {
                                    (*p_level).addr_nxt = unsafe { (*p_level).addr_brk }
                                };
                                unsafe { (*p_level).addr_nxt }
                            };
                        __state = 26;
                    }
                    26 => {
                        addr_cont =
                            {
                                unsafe {
                                    (*p_level).addr_cont =
                                        unsafe { sqlite3_vdbe_make_label(p_parse) }
                                };
                                unsafe { (*p_level).addr_cont }
                            };
                        __state = 27;
                    }
                    27 => { { let _ = 0; }; __state = 28; }
                    28 => {
                        if unsafe { (*p_level).i_from } as i32 > 0 &&
                                unsafe { (*p_tab_item.offset(0 as isize)).fg.jointype } as
                                            i32 & 8 != 0 {
                            __state = 30;
                        } else { __state = 29; }
                    }
                    29 => {
                        if unsafe { (*p_tab_item).fg.via_coroutine() } != 0 {
                            __state = 34;
                        } else { __state = 35; }
                    }
                    30 => {
                        unsafe {
                            (*p_level).i_left_join =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 31;
                    }
                    31 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 0,
                                unsafe { (*p_level).i_left_join })
                        };
                        __state = 32;
                    }
                    32 => { __state = 29; }
                    33 => {
                        i_loop = if !(p_idx).is_null() { 1 } else { 2 };
                        __state = 654;
                    }
                    34 => { __state = 36; }
                    35 => {
                        if unsafe { (*p_loop).ws_flags } & 1024 as u32 != 0 as u32 {
                            __state = 45;
                        } else { __state = 46; }
                    }
                    36 => { __state = 37; }
                    37 => { { let _ = 0; }; __state = 38; }
                    38 => {
                        p_subq = unsafe { (*p_tab_item).u4.p_subq };
                        __state = 39;
                    }
                    39 => {
                        reg_yield = unsafe { (*p_subq).reg_return };
                        __state = 40;
                    }
                    40 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 11, reg_yield, 0,
                                unsafe { (*p_subq).addr_fill_sub })
                        };
                        __state = 41;
                    }
                    41 => {
                        unsafe {
                            (*p_level).p2 =
                                unsafe { sqlite3_vdbe_add_op2(v, 12, reg_yield, addr_brk) }
                        };
                        __state = 42;
                    }
                    42 => { __state = 43; }
                    43 => { __state = 44; }
                    44 => { unsafe { (*p_level).op = 9 as u8 }; __state = 33; }
                    45 => { __state = 47; }
                    46 => {
                        if unsafe { (*p_loop).ws_flags } & 256 as u32 != 0 as u32 &&
                                unsafe { (*p_loop).ws_flags } & (4 | 1) as u32 != 0 as u32 {
                            __state = 122;
                        } else { __state = 123; }
                    }
                    47 => { __state = 48; }
                    48 => {
                        n_constraint = unsafe { (*p_loop).n_l_term } as i32;
                        __state = 49;
                    }
                    49 => {
                        i_reg =
                            unsafe {
                                sqlite3_get_temp_range(p_parse, n_constraint + 2)
                            };
                        __state = 50;
                    }
                    50 => {
                        addr_not_found = unsafe { (*p_level).addr_brk };
                        __state = 51;
                    }
                    51 => { j = 0; __state = 53; }
                    52 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                unsafe { (*p_loop).u.vtab.idx_num }, i_reg)
                        };
                        __state = 75;
                    }
                    53 => {
                        if j < n_constraint { __state = 54; } else { __state = 52; }
                    }
                    54 => { i_target = i_reg + j + 2; __state = 56; }
                    55 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 53;
                    }
                    56 => {
                        p_term =
                            unsafe {
                                *unsafe { (*p_loop).a_l_term.offset(j as isize) }
                            };
                        __state = 57;
                    }
                    57 => {
                        if p_term == core::ptr::null_mut() {
                            __state = 59;
                        } else { __state = 58; }
                    }
                    58 => {
                        if unsafe { (*p_term).e_operator } as i32 & 1 != 0 {
                            __state = 60;
                        } else { __state = 61; }
                    }
                    59 => { __state = 55; }
                    60 => {
                        if if j <= 31 { (1 as u32) << j } else { 0 as u32 } &
                                    unsafe { (*p_loop).u.vtab.m_handle_in } != 0 {
                            __state = 62;
                        } else { __state = 63; }
                    }
                    61 => {
                        p_right = unsafe { (*unsafe { (*p_term).p_expr }).p_right };
                        __state = 68;
                    }
                    62 => {
                        i_tab =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 64;
                    }
                    63 => {
                        code_equality_term(p_parse, p_term, p_level, j, b_rev,
                            i_target);
                        __state = 67;
                    }
                    64 => {
                        i_cache =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 65;
                    }
                    65 => {
                        unsafe {
                            sqlite3_code_rhs_of_in(p_parse, unsafe { (*p_term).p_expr },
                                i_tab, 0)
                        };
                        __state = 66;
                    }
                    66 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 177, i_tab, i_target, i_cache)
                        };
                        __state = 55;
                    }
                    67 => {
                        addr_not_found = unsafe { (*p_level).addr_nxt };
                        __state = 55;
                    }
                    68 => {
                        code_expr_or_vector(p_parse, p_right, i_target, 1);
                        __state = 69;
                    }
                    69 => {
                        if unsafe { (*p_term).e_match_op } as i32 == 74 &&
                                unsafe { (*p_loop).u.vtab.b_omit_offset() } != 0 {
                            __state = 70;
                        } else { __state = 55; }
                    }
                    70 => { { let _ = 0; }; __state = 71; }
                    71 => { { let _ = 0; }; __state = 72; }
                    72 => { { let _ = 0; }; __state = 73; }
                    73 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 0,
                                unsafe { (*unsafe { (*p_w_info).p_select }).i_offset })
                        };
                        __state = 74;
                    }
                    74 => { __state = 55; }
                    75 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, n_constraint, i_reg + 1)
                        };
                        __state = 76;
                    }
                    76 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 6, i_cur, addr_not_found, i_reg,
                                unsafe { (*p_loop).u.vtab.idx_str } as *const i8,
                                if unsafe { (*p_loop).u.vtab.need_free() } != 0 {
                                    -7
                                } else { -1 })
                        };
                        __state = 77;
                    }
                    77 => { __state = 78; }
                    78 => {
                        unsafe { (*p_loop).u.vtab.set_need_free(0 as u32 as u32) };
                        __state = 79;
                    }
                    79 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 81;
                        } else { __state = 80; }
                    }
                    80 => { unsafe { (*p_level).p1 = i_cur }; __state = 82; }
                    81 => {
                        unsafe { (*p_loop).u.vtab.idx_str = core::ptr::null_mut() };
                        __state = 80;
                    }
                    82 => {
                        unsafe {
                            (*p_level).op =
                                if unsafe { (*p_w_info).e_one_pass } != 0 {
                                        189
                                    } else { 65 } as u8
                        };
                        __state = 83;
                    }
                    83 => {
                        unsafe {
                            (*p_level).p2 = unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 84;
                    }
                    84 => { { let _ = 0; }; __state = 85; }
                    85 => { j = 0; __state = 86; }
                    86 => {
                        if j < n_constraint { __state = 87; } else { __state = 33; }
                    }
                    87 => {
                        p_term =
                            unsafe {
                                *unsafe { (*p_loop).a_l_term.offset(j as isize) }
                            };
                        __state = 89;
                    }
                    88 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 86;
                    }
                    89 => {
                        if j < 16 &&
                                unsafe { (*p_loop).u.vtab.omit_mask } as i32 >> j & 1 != 0 {
                            __state = 91;
                        } else { __state = 90; }
                    }
                    90 => {
                        if unsafe { (*p_term).e_operator } as i32 & 1 != 0 &&
                                    if j <= 31 { (1 as u32) << j } else { 0 as u32 } &
                                            unsafe { (*p_loop).u.vtab.m_handle_in } == 0 as u32 &&
                                (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                            __state = 93;
                        } else { __state = 88; }
                    }
                    91 => {
                        disable_term(unsafe { &*p_level }, p_term);
                        __state = 92;
                    }
                    92 => { __state = 88; }
                    93 => { __state = 94; }
                    94 => { __state = 95; }
                    95 => { __state = 96; }
                    96 => { __state = 97; }
                    97 => { i_in = 0; __state = 99; }
                    98 => {
                        p_compare =
                            unsafe {
                                sqlite3_p_expr(p_parse, 54, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        __state = 106;
                    }
                    99 => {
                        if i_in < unsafe { (*p_level).u.in_.n_in } {
                            __state = 100;
                        } else { __state = 98; }
                    }
                    100 => {
                        p_op =
                            unsafe {
                                sqlite3_vdbe_get_op(v,
                                    unsafe {
                                        (*unsafe {
                                                    (*p_level).u.in_.a_in_loop.offset(i_in as isize)
                                                }).addr_in_top
                                    })
                            };
                        __state = 102;
                    }
                    101 => {
                        { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                        __state = 99;
                    }
                    102 => {
                        if unsafe { (*p_op).opcode } as i32 == 96 &&
                                    unsafe { (*p_op).p3 } == i_reg + j + 2 ||
                                unsafe { (*p_op).opcode } as i32 == 137 &&
                                    unsafe { (*p_op).p2 } == i_reg + j + 2 {
                            __state = 103;
                        } else { __state = 101; }
                    }
                    103 => { __state = 104; }
                    104 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, unsafe { (*p_op).opcode } as i32,
                                unsafe { (*p_op).p1 }, unsafe { (*p_op).p2 },
                                unsafe { (*p_op).p3 })
                        };
                        __state = 105;
                    }
                    105 => { __state = 98; }
                    106 => {
                        if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    107 => {
                        unsafe { sqlite3_expr_delete(db, p_compare) };
                        __state = 88;
                    }
                    108 => {
                        i_fld = unsafe { (*p_term).u.x.i_field };
                        __state = 109;
                    }
                    109 => {
                        p_left = unsafe { (*unsafe { (*p_term).p_expr }).p_left };
                        __state = 110;
                    }
                    110 => { { let _ = 0; }; __state = 111; }
                    111 => {
                        if i_fld > 0 { __state = 113; } else { __state = 114; }
                    }
                    112 => {
                        unsafe {
                            (*p_compare).p_right =
                                {
                                    p_right_1 =
                                        unsafe { sqlite3_expr(db, 176, core::ptr::null()) };
                                    p_right_1
                                }
                        };
                        __state = 118;
                    }
                    113 => { { let _ = 0; }; __state = 115; }
                    114 => {
                        unsafe { (*p_compare).p_left = p_left };
                        __state = 112;
                    }
                    115 => { { let _ = 0; }; __state = 116; }
                    116 => { { let _ = 0; }; __state = 117; }
                    117 => {
                        unsafe {
                            (*p_compare).p_left =
                                unsafe {
                                    (*(unsafe { (*unsafe { (*p_left).x.p_list }).a.as_ptr() } as
                                                    *mut ExprListItem).offset((i_fld - 1) as isize)).p_expr
                                }
                        };
                        __state = 112;
                    }
                    118 => {
                        if !(p_right_1).is_null() {
                            __state = 120;
                        } else { __state = 119; }
                    }
                    119 => {
                        unsafe { (*p_compare).p_left = core::ptr::null_mut() };
                        __state = 107;
                    }
                    120 => {
                        unsafe { (*p_right_1).i_table = i_reg + j + 2 };
                        __state = 121;
                    }
                    121 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_compare,
                                unsafe { (*p_level).addr_cont }, 16)
                        };
                        __state = 119;
                    }
                    122 => { { let _ = 0; }; __state = 124; }
                    123 => {
                        if unsafe { (*p_loop).ws_flags } & 256 as u32 != 0 as u32 &&
                                unsafe { (*p_loop).ws_flags } & 2 as u32 != 0 as u32 {
                            __state = 142;
                        } else { __state = 143; }
                    }
                    124 => {
                        p_term =
                            unsafe {
                                *unsafe { (*p_loop).a_l_term.offset(0 as isize) }
                            };
                        __state = 125;
                    }
                    125 => { { let _ = 0; }; __state = 126; }
                    126 => { { let _ = 0; }; __state = 127; }
                    127 => { __state = 128; }
                    128 => {
                        i_release_reg =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 129;
                    }
                    129 => {
                        i_rowid_reg =
                            code_equality_term(p_parse, p_term, p_level, 0, b_rev,
                                i_release_reg);
                        __state = 130;
                    }
                    130 => {
                        if i_rowid_reg != i_release_reg {
                            __state = 132;
                        } else { __state = 131; }
                    }
                    131 => {
                        addr_nxt = unsafe { (*p_level).addr_nxt };
                        __state = 133;
                    }
                    132 => {
                        unsafe { sqlite3_release_temp_reg(p_parse, i_release_reg) };
                        __state = 131;
                    }
                    133 => {
                        if unsafe { (*p_level).reg_filter } != 0 {
                            __state = 135;
                        } else { __state = 134; }
                    }
                    134 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 30, i_cur, addr_nxt, i_rowid_reg)
                        };
                        __state = 140;
                    }
                    135 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 13, i_rowid_reg, addr_nxt)
                        };
                        __state = 136;
                    }
                    136 => { __state = 137; }
                    137 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 66,
                                unsafe { (*p_level).reg_filter }, addr_nxt, i_rowid_reg, 1)
                        };
                        __state = 138;
                    }
                    138 => { __state = 139; }
                    139 => {
                        filter_pull_down(p_parse, unsafe { &mut *p_w_info },
                            i_level, addr_nxt, not_ready);
                        __state = 134;
                    }
                    140 => { __state = 141; }
                    141 => {
                        unsafe { (*p_level).op = 189 as u8 };
                        __state = 33;
                    }
                    142 => { test_op = 189; __state = 144; }
                    143 => {
                        if unsafe { (*p_loop).ws_flags } & 512 as u32 != 0 {
                            __state = 225;
                        } else { __state = 226; }
                    }
                    144 => { __state = 145; }
                    145 => { mem_end_value = 0; __state = 146; }
                    146 => { __state = 147; }
                    147 => { j = 0; __state = 148; }
                    148 => {
                        p_start = { p_end = core::ptr::null_mut(); p_end };
                        __state = 149;
                    }
                    149 => {
                        if unsafe { (*p_loop).ws_flags } & 32 as u32 != 0 {
                            __state = 151;
                        } else { __state = 150; }
                    }
                    150 => {
                        if unsafe { (*p_loop).ws_flags } & 16 as u32 != 0 {
                            __state = 153;
                        } else { __state = 152; }
                    }
                    151 => {
                        p_start =
                            unsafe {
                                *unsafe {
                                        (*p_loop).a_l_term.offset({
                                                    let __p = &mut j;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                    }
                            };
                        __state = 150;
                    }
                    152 => { { let _ = 0; }; __state = 154; }
                    153 => {
                        p_end =
                            unsafe {
                                *unsafe {
                                        (*p_loop).a_l_term.offset({
                                                    let __p = &mut j;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                    }
                            };
                        __state = 152;
                    }
                    154 => {
                        if b_rev != 0 { __state = 156; } else { __state = 155; }
                    }
                    155 => { __state = 159; }
                    156 => { p_term = p_start; __state = 157; }
                    157 => { p_start = p_end; __state = 158; }
                    158 => { p_end = p_term; __state = 155; }
                    159 => {
                        if !(p_start).is_null() {
                            __state = 161;
                        } else { __state = 162; }
                    }
                    160 => {
                        if !(p_end).is_null() {
                            __state = 199;
                        } else { __state = 198; }
                    }
                    161 => { __state = 163; }
                    162 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, if b_rev != 0 { 32 } else { 36 },
                                i_cur, unsafe { (*p_level).addr_halt })
                        };
                        __state = 196;
                    }
                    163 => { __state = 164; }
                    164 => { __state = 165; }
                    165 => { __state = 166; }
                    166 => { { let _ = 0; }; __state = 167; }
                    167 => { { let _ = 0; }; __state = 168; }
                    168 => { { let _ = 0; }; __state = 169; }
                    169 => { { let _ = 0; }; __state = 170; }
                    170 => { __state = 171; }
                    171 => {
                        p_x = unsafe { (*p_start).p_expr };
                        __state = 172;
                    }
                    172 => { { let _ = 0; }; __state = 173; }
                    173 => { __state = 174; }
                    174 => {
                        if unsafe {
                                    sqlite3_expr_is_vector(unsafe { (*p_x).p_right } as
                                            *const Expr)
                                } != 0 {
                            __state = 176;
                        } else { __state = 177; }
                    }
                    175 => {
                        unsafe { sqlite3_vdbe_add_op3(v, op, i_cur, addr_brk, r1) };
                        __state = 190;
                    }
                    176 => {
                        r1 =
                            {
                                r_temp = unsafe { sqlite3_get_temp_reg(p_parse) };
                                r_temp
                            };
                        __state = 178;
                    }
                    177 => {
                        r1 =
                            unsafe {
                                sqlite3_expr_code_temp(p_parse, unsafe { (*p_x).p_right },
                                    &mut r_temp)
                            };
                        __state = 188;
                    }
                    178 => {
                        code_expr_or_vector(p_parse, unsafe { (*p_x).p_right }, r1,
                            1);
                        __state = 179;
                    }
                    179 => { __state = 180; }
                    180 => { __state = 181; }
                    181 => { __state = 182; }
                    182 => { __state = 183; }
                    183 => {
                        op =
                            a_move_op[(unsafe { (*p_x).op } as i32 - 55 - 1 & 3 | 1) as
                                        usize] as i32;
                        __state = 184;
                    }
                    184 => { { let _ = 0; }; __state = 185; }
                    185 => { { let _ = 0; }; __state = 186; }
                    186 => { { let _ = 0; }; __state = 187; }
                    187 => { { let _ = 0; }; __state = 175; }
                    188 => {
                        disable_term(unsafe { &*p_level }, p_start);
                        __state = 189;
                    }
                    189 => {
                        op =
                            a_move_op[(unsafe { (*p_x).op } as i32 - 55) as usize] as
                                i32;
                        __state = 175;
                    }
                    190 => { __state = 191; }
                    191 => { __state = 192; }
                    192 => { __state = 193; }
                    193 => { __state = 194; }
                    194 => { __state = 195; }
                    195 => {
                        unsafe { sqlite3_release_temp_reg(p_parse, r_temp) };
                        __state = 160;
                    }
                    196 => { __state = 197; }
                    197 => { __state = 160; }
                    198 => {
                        start = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 212;
                    }
                    199 => { __state = 200; }
                    200 => {
                        p_x_1 = unsafe { (*p_end).p_expr };
                        __state = 201;
                    }
                    201 => { { let _ = 0; }; __state = 202; }
                    202 => { { let _ = 0; }; __state = 203; }
                    203 => { __state = 204; }
                    204 => { __state = 205; }
                    205 => {
                        mem_end_value =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 206;
                    }
                    206 => {
                        code_expr_or_vector(p_parse, unsafe { (*p_x_1).p_right },
                            mem_end_value, 1);
                        __state = 207;
                    }
                    207 => {
                        if 0 ==
                                    unsafe {
                                        sqlite3_expr_is_vector(unsafe { (*p_x_1).p_right } as
                                                *const Expr)
                                    } &&
                                (unsafe { (*p_x_1).op } as i32 == 57 ||
                                    unsafe { (*p_x_1).op } as i32 == 55) {
                            __state = 209;
                        } else { __state = 210; }
                    }
                    208 => {
                        if 0 ==
                                unsafe {
                                    sqlite3_expr_is_vector(unsafe { (*p_x_1).p_right } as
                                            *const Expr)
                                } {
                            __state = 211;
                        } else { __state = 198; }
                    }
                    209 => {
                        test_op = if b_rev != 0 { 56 } else { 58 };
                        __state = 208;
                    }
                    210 => {
                        test_op = if b_rev != 0 { 57 } else { 55 };
                        __state = 208;
                    }
                    211 => {
                        disable_term(unsafe { &*p_level }, p_end);
                        __state = 198;
                    }
                    212 => {
                        unsafe {
                            (*p_level).op = if b_rev != 0 { 39 } else { 40 } as u8
                        };
                        __state = 213;
                    }
                    213 => { unsafe { (*p_level).p1 = i_cur }; __state = 214; }
                    214 => { unsafe { (*p_level).p2 = start }; __state = 215; }
                    215 => { { let _ = 0; }; __state = 216; }
                    216 => {
                        if test_op != 189 { __state = 217; } else { __state = 33; }
                    }
                    217 => {
                        i_rowid_reg =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 218;
                    }
                    218 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 137, i_cur, i_rowid_reg) };
                        __state = 219;
                    }
                    219 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, test_op, mem_end_value, addr_brk,
                                i_rowid_reg)
                        };
                        __state = 220;
                    }
                    220 => { __state = 221; }
                    221 => { __state = 222; }
                    222 => { __state = 223; }
                    223 => { __state = 224; }
                    224 => {
                        unsafe { sqlite3_vdbe_change_p5(v, (67 | 16) as u16) };
                        __state = 33;
                    }
                    225 => { __state = 227; }
                    226 => {
                        if unsafe { (*p_loop).ws_flags } & 8192 as u32 != 0 {
                            __state = 488;
                        } else { __state = 489; }
                    }
                    227 => { __state = 228; }
                    228 => {
                        n_eq = unsafe { (*p_loop).u.btree.n_eq };
                        __state = 229;
                    }
                    229 => {
                        n_btm = unsafe { (*p_loop).u.btree.n_btm };
                        __state = 230;
                    }
                    230 => {
                        n_top = unsafe { (*p_loop).u.btree.n_top };
                        __state = 231;
                    }
                    231 => { __state = 232; }
                    232 => {
                        p_range_start = core::ptr::null_mut();
                        __state = 233;
                    }
                    233 => {
                        p_range_end = core::ptr::null_mut();
                        __state = 234;
                    }
                    234 => { __state = 235; }
                    235 => { __state = 236; }
                    236 => { __state = 237; }
                    237 => { __state = 238; }
                    238 => { __state = 239; }
                    239 => { n_extra_reg = 0; __state = 240; }
                    240 => { __state = 241; }
                    241 => { __state = 242; }
                    242 => { z_end_aff = core::ptr::null_mut(); __state = 243; }
                    243 => { b_seek_past_null = 0 as u8; __state = 244; }
                    244 => { b_stop_at_null = 0 as u8; __state = 245; }
                    245 => { __state = 246; }
                    246 => { reg_bignull = 0; __state = 247; }
                    247 => { addr_seek_scan = 0; __state = 248; }
                    248 => {
                        p_idx = unsafe { (*p_loop).u.btree.p_index };
                        __state = 249;
                    }
                    249 => {
                        i_idx_cur = unsafe { (*p_level).i_idx_cur };
                        __state = 250;
                    }
                    250 => { { let _ = 0; }; __state = 251; }
                    251 => { j = n_eq as i32; __state = 252; }
                    252 => {
                        if unsafe { (*p_loop).ws_flags } & 32 as u32 != 0 {
                            __state = 254;
                        } else { __state = 253; }
                    }
                    253 => {
                        if unsafe { (*p_loop).ws_flags } & 16 as u32 != 0 {
                            __state = 258;
                        } else { __state = 257; }
                    }
                    254 => {
                        p_range_start =
                            unsafe {
                                *unsafe {
                                        (*p_loop).a_l_term.offset({
                                                    let __p = &mut j;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                    }
                            };
                        __state = 255;
                    }
                    255 => {
                        n_extra_reg =
                            if n_extra_reg > unsafe { (*p_loop).u.btree.n_btm } as i32 {
                                n_extra_reg
                            } else { (unsafe { (*p_loop).u.btree.n_btm }) as i32 };
                        __state = 256;
                    }
                    256 => { { let _ = 0; }; __state = 253; }
                    257 => { { let _ = 0; }; __state = 276; }
                    258 => {
                        p_range_end =
                            unsafe {
                                *unsafe {
                                        (*p_loop).a_l_term.offset({
                                                    let __p = &mut j;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                    }
                            };
                        __state = 259;
                    }
                    259 => {
                        n_extra_reg =
                            if n_extra_reg > unsafe { (*p_loop).u.btree.n_top } as i32 {
                                n_extra_reg
                            } else { (unsafe { (*p_loop).u.btree.n_top }) as i32 };
                        __state = 260;
                    }
                    260 => {
                        if unsafe { (*p_range_end).wt_flags } as i32 & 256 != 0 {
                            __state = 262;
                        } else { __state = 261; }
                    }
                    261 => {
                        if p_range_start == core::ptr::null_mut() {
                            __state = 273;
                        } else { __state = 257; }
                    }
                    262 => { { let _ = 0; }; __state = 263; }
                    263 => { { let _ = 0; }; __state = 264; }
                    264 => {
                        unsafe {
                            (*p_level).i_like_rep_cntr =
                                {
                                        let __p = unsafe { &mut (*p_parse).n_mem };
                                        *__p += 1;
                                        *__p
                                    } as u32
                        };
                        __state = 265;
                    }
                    265 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1,
                                unsafe { (*p_level).i_like_rep_cntr } as i32)
                        };
                        __state = 266;
                    }
                    266 => { __state = 267; }
                    267 => {
                        unsafe {
                            (*p_level).addr_like_rep =
                                unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 268;
                    }
                    268 => { __state = 269; }
                    269 => { __state = 270; }
                    270 => { { let _ = 0; }; __state = 271; }
                    271 => {
                        unsafe { (*p_level).i_like_rep_cntr <<= 1 as u32 };
                        __state = 272;
                    }
                    272 => {
                        unsafe {
                            (*p_level).i_like_rep_cntr |=
                                (b_rev ^
                                        (unsafe {
                                                        *unsafe { (*p_idx).a_sort_order.add(n_eq as usize) }
                                                    } as i32 == 1) as i32) as u32
                        };
                        __state = 261;
                    }
                    273 => {
                        j =
                            unsafe { *unsafe { (*p_idx).ai_column.add(n_eq as usize) } }
                                as i32;
                        __state = 274;
                    }
                    274 => {
                        if j >= 0 &&
                                    unsafe {
                                                (*unsafe {
                                                            (*unsafe { (*p_idx).p_table }).a_col.offset(j as isize)
                                                        }).not_null()
                                            } as i32 == 0 || j == -2 {
                            __state = 275;
                        } else { __state = 257; }
                    }
                    275 => { b_seek_past_null = 1 as u8; __state = 257; }
                    276 => {
                        if unsafe { (*p_loop).ws_flags } & (16 | 32) as u32 ==
                                    0 as u32 &&
                                unsafe { (*p_loop).ws_flags } & 524288 as u32 != 0 as u32 {
                            __state = 278;
                        } else { __state = 277; }
                    }
                    277 => {
                        if (n_eq as i32) < unsafe { (*p_idx).n_column } as i32 &&
                                b_rev ==
                                    (unsafe {
                                                    *unsafe { (*p_idx).a_sort_order.add(n_eq as usize) }
                                                } as i32 == 0) as i32 {
                            __state = 288;
                        } else { __state = 287; }
                    }
                    278 => { { let _ = 0; }; __state = 279; }
                    279 => { { let _ = 0; }; __state = 280; }
                    280 => { __state = 281; }
                    281 => { n_extra_reg = 1; __state = 282; }
                    282 => { b_seek_past_null = 1 as u8; __state = 283; }
                    283 => {
                        unsafe {
                            (*p_level).reg_bignull =
                                {
                                    reg_bignull =
                                        {
                                            let __p = unsafe { &mut (*p_parse).n_mem };
                                            *__p += 1;
                                            *__p
                                        };
                                    reg_bignull
                                }
                        };
                        __state = 284;
                    }
                    284 => {
                        if unsafe { (*p_level).i_left_join } != 0 {
                            __state = 286;
                        } else { __state = 285; }
                    }
                    285 => {
                        unsafe {
                            (*p_level).addr_bignull =
                                unsafe { sqlite3_vdbe_make_label(p_parse) }
                        };
                        __state = 277;
                    }
                    286 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_bignull) };
                        __state = 285;
                    }
                    287 => {
                        if i_level > 0 &&
                                unsafe { (*p_loop).ws_flags } & 1048576 as u32 != 0 as u32 {
                            __state = 301;
                        } else { __state = 300; }
                    }
                    288 => { t = p_range_end; __state = 290; }
                    289 => { __state = 292; }
                    290 => { p_range_end = p_range_start; __state = 291; }
                    291 => { p_range_start = t; __state = 289; }
                    292 => { t__1 = b_seek_past_null; __state = 294; }
                    293 => { __state = 296; }
                    294 => { b_seek_past_null = b_stop_at_null; __state = 295; }
                    295 => { b_stop_at_null = t__1; __state = 293; }
                    296 => { t__2 = n_btm as u8; __state = 298; }
                    297 => { __state = 287; }
                    298 => { n_btm = n_top; __state = 299; }
                    299 => { n_top = t__2 as u16; __state = 297; }
                    300 => { __state = 302; }
                    301 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 138, i_idx_cur) };
                        __state = 300;
                    }
                    302 => {
                        reg_base =
                            code_all_equality_terms(p_parse, p_level, b_rev,
                                n_extra_reg, &mut z_start_aff);
                        __state = 303;
                    }
                    303 => { { let _ = 0; }; __state = 304; }
                    304 => {
                        if !(z_start_aff).is_null() && n_top != 0 {
                            __state = 306;
                        } else { __state = 305; }
                    }
                    305 => {
                        addr_nxt =
                            if reg_bignull != 0 {
                                unsafe { (*p_level).addr_bignull }
                            } else { unsafe { (*p_level).addr_nxt } };
                        __state = 307;
                    }
                    306 => {
                        z_end_aff =
                            unsafe {
                                sqlite3_db_str_dup(db,
                                    unsafe { &raw mut *z_start_aff.add(n_eq as usize) } as
                                        *const i8)
                            };
                        __state = 305;
                    }
                    307 => { __state = 308; }
                    308 => { __state = 309; }
                    309 => { __state = 310; }
                    310 => { __state = 311; }
                    311 => {
                        start_eq =
                            ((p_range_start).is_null() as i32 != 0 ||
                                    unsafe { (*p_range_start).e_operator } as i32 &
                                            (2 << 56 - 54 | 2 << 58 - 54) != 0) as i32;
                        __state = 312;
                    }
                    312 => {
                        end_eq =
                            ((p_range_end).is_null() as i32 != 0 ||
                                    unsafe { (*p_range_end).e_operator } as i32 &
                                            (2 << 56 - 54 | 2 << 58 - 54) != 0) as i32;
                        __state = 313;
                    }
                    313 => {
                        start_constraints =
                            (!(p_range_start).is_null() || n_eq as i32 > 0) as i32;
                        __state = 314;
                    }
                    314 => { n_constraint_1 = n_eq as i32; __state = 315; }
                    315 => {
                        if !(p_range_start).is_null() {
                            __state = 317;
                        } else { __state = 318; }
                    }
                    316 => {
                        code_apply_affinity(unsafe { &*p_parse }, reg_base,
                            n_constraint_1 - b_seek_past_null as i32,
                            z_start_aff as *const i8);
                        __state = 340;
                    }
                    317 => {
                        p_right_2 =
                            unsafe { (*unsafe { (*p_range_start).p_expr }).p_right };
                        __state = 319;
                    }
                    318 => {
                        if b_seek_past_null != 0 {
                            __state = 332;
                        } else { __state = 333; }
                    }
                    319 => {
                        code_expr_or_vector(p_parse, p_right_2,
                            reg_base + n_eq as i32, n_btm as i32);
                        __state = 320;
                    }
                    320 => {
                        where_like_optimization_string_fixup(v,
                            unsafe { &*p_level }, unsafe { &*p_range_start });
                        __state = 321;
                    }
                    321 => {
                        if unsafe { (*p_range_start).wt_flags } as i32 & 128 == 0 &&
                                unsafe {
                                        sqlite3_expr_can_be_null(p_right_2 as *const Expr)
                                    } != 0 {
                            __state = 323;
                        } else { __state = 322; }
                    }
                    322 => {
                        if !(z_start_aff).is_null() {
                            __state = 326;
                        } else { __state = 325; }
                    }
                    323 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 51, reg_base + n_eq as i32,
                                addr_nxt)
                        };
                        __state = 324;
                    }
                    324 => { __state = 322; }
                    325 => { n_constraint_1 += n_btm as i32; __state = 327; }
                    326 => {
                        update_range_affinity_str(p_right_2,
                            unsafe {
                                let __p =
                                    unsafe { &mut *z_start_aff.add(n_eq as usize) } as *mut i8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, n_btm as usize)
                                }
                            });
                        __state = 325;
                    }
                    327 => { __state = 328; }
                    328 => {
                        if unsafe {
                                    sqlite3_expr_is_vector(p_right_2 as *const Expr)
                                } == 0 {
                            __state = 330;
                        } else { __state = 331; }
                    }
                    329 => { b_seek_past_null = 0 as u8; __state = 316; }
                    330 => {
                        disable_term(unsafe { &*p_level }, p_range_start);
                        __state = 329;
                    }
                    331 => { start_eq = 1; __state = 329; }
                    332 => { start_eq = 0; __state = 334; }
                    333 => {
                        if reg_bignull != 0 {
                            __state = 337;
                        } else { __state = 316; }
                    }
                    334 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 77, 0, reg_base + n_eq as i32)
                        };
                        __state = 335;
                    }
                    335 => { start_constraints = 1; __state = 336; }
                    336 => {
                        {
                            let __p = &mut n_constraint_1;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 316;
                    }
                    337 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 77, 0, reg_base + n_eq as i32)
                        };
                        __state = 338;
                    }
                    338 => { start_constraints = 1; __state = 339; }
                    339 => {
                        {
                            let __p = &mut n_constraint_1;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 316;
                    }
                    340 => {
                        if unsafe { (*p_loop).n_skip } as i32 > 0 &&
                                n_constraint_1 == unsafe { (*p_loop).n_skip } as i32 {
                            __state = 342;
                        } else { __state = 343; }
                    }
                    341 => { n_constraint_1 = n_eq as i32; __state = 392; }
                    342 => { __state = 341; }
                    343 => {
                        if reg_bignull != 0 {
                            __state = 345;
                        } else { __state = 344; }
                    }
                    344 => {
                        if unsafe { (*p_level).reg_filter } != 0 {
                            __state = 348;
                        } else { __state = 347; }
                    }
                    345 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, reg_bignull) };
                        __state = 346;
                    }
                    346 => { __state = 344; }
                    347 => {
                        op__1 =
                            a_start_op[((start_constraints << 2) + (start_eq << 1) +
                                            b_rev) as usize] as i32;
                        __state = 351;
                    }
                    348 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 66,
                                unsafe { (*p_level).reg_filter }, addr_nxt, reg_base,
                                n_eq as i32)
                        };
                        __state = 349;
                    }
                    349 => { __state = 350; }
                    350 => {
                        filter_pull_down(p_parse, unsafe { &mut *p_w_info },
                            i_level, addr_nxt, not_ready);
                        __state = 347;
                    }
                    351 => { { let _ = 0; }; __state = 352; }
                    352 => {
                        if unsafe { (*p_loop).ws_flags } & 1048576 as u32 !=
                                    0 as u32 && op__1 == 23 {
                            __state = 354;
                        } else { __state = 353; }
                    }
                    353 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, op__1, i_idx_cur, addr_nxt,
                                reg_base, n_constraint_1)
                        };
                        __state = 361;
                    }
                    354 => { { let _ = 0; }; __state = 355; }
                    355 => {
                        addr_seek_scan =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 126,
                                    (unsafe {
                                                    *unsafe { (*p_idx).ai_row_log_est.offset(0 as isize) }
                                                } as i32 + 9) / 10)
                            };
                        __state = 356;
                    }
                    356 => {
                        if !(p_range_start).is_null() || !(p_range_end).is_null() {
                            __state = 358;
                        } else { __state = 357; }
                    }
                    357 => { __state = 353; }
                    358 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                        __state = 359;
                    }
                    359 => {
                        unsafe {
                            sqlite3_vdbe_change_p2(v, addr_seek_scan,
                                unsafe { sqlite3_vdbe_current_addr(v) } + 1)
                        };
                        __state = 360;
                    }
                    360 => { addr_seek_scan = 0; __state = 357; }
                    361 => { __state = 362; }
                    362 => { __state = 363; }
                    363 => { __state = 364; }
                    364 => { __state = 365; }
                    365 => { __state = 366; }
                    366 => { __state = 367; }
                    367 => { __state = 368; }
                    368 => { __state = 369; }
                    369 => { __state = 370; }
                    370 => { __state = 371; }
                    371 => { __state = 372; }
                    372 => { __state = 373; }
                    373 => { __state = 374; }
                    374 => { { let _ = 0; }; __state = 375; }
                    375 => {
                        if reg_bignull != 0 {
                            __state = 376;
                        } else { __state = 341; }
                    }
                    376 => { { let _ = 0; }; __state = 377; }
                    377 => { { let _ = 0; }; __state = 378; }
                    378 => { { let _ = 0; }; __state = 379; }
                    379 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 9, 0,
                                unsafe { sqlite3_vdbe_current_addr(v) } + 2)
                        };
                        __state = 380;
                    }
                    380 => {
                        op__1 =
                            a_start_op[((n_constraint_1 > 1) as i32 * 4 + 2 + b_rev) as
                                        usize] as i32;
                        __state = 381;
                    }
                    381 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, op__1, i_idx_cur, addr_nxt,
                                reg_base, n_constraint_1 - start_eq)
                        };
                        __state = 382;
                    }
                    382 => { __state = 383; }
                    383 => { __state = 384; }
                    384 => { __state = 385; }
                    385 => { __state = 386; }
                    386 => { __state = 387; }
                    387 => { __state = 388; }
                    388 => { __state = 389; }
                    389 => { __state = 390; }
                    390 => { __state = 391; }
                    391 => { { let _ = 0; }; __state = 341; }
                    392 => { { let _ = 0; }; __state = 393; }
                    393 => {
                        if !(p_range_end).is_null() {
                            __state = 395;
                        } else { __state = 396; }
                    }
                    394 => {
                        if !(z_start_aff).is_null() {
                            __state = 417;
                        } else { __state = 416; }
                    }
                    395 => {
                        p_right_3 =
                            unsafe { (*unsafe { (*p_range_end).p_expr }).p_right };
                        __state = 397;
                    }
                    396 => {
                        if b_stop_at_null != 0 {
                            __state = 412;
                        } else { __state = 394; }
                    }
                    397 => { { let _ = 0; }; __state = 398; }
                    398 => {
                        code_expr_or_vector(p_parse, p_right_3,
                            reg_base + n_eq as i32, n_top as i32);
                        __state = 399;
                    }
                    399 => {
                        where_like_optimization_string_fixup(v,
                            unsafe { &*p_level }, unsafe { &*p_range_end });
                        __state = 400;
                    }
                    400 => {
                        if unsafe { (*p_range_end).wt_flags } as i32 & 128 == 0 &&
                                unsafe {
                                        sqlite3_expr_can_be_null(p_right_3 as *const Expr)
                                    } != 0 {
                            __state = 402;
                        } else { __state = 401; }
                    }
                    401 => {
                        if !(z_end_aff).is_null() {
                            __state = 405;
                        } else { __state = 406; }
                    }
                    402 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 51, reg_base + n_eq as i32,
                                addr_nxt)
                        };
                        __state = 403;
                    }
                    403 => { __state = 401; }
                    404 => { n_constraint_1 += n_top as i32; __state = 408; }
                    405 => {
                        update_range_affinity_str(p_right_3,
                            unsafe {
                                let __p = z_end_aff as *mut i8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, n_top as usize)
                                }
                            });
                        __state = 407;
                    }
                    406 => { { let _ = 0; }; __state = 404; }
                    407 => {
                        code_apply_affinity(unsafe { &*p_parse },
                            reg_base + n_eq as i32, n_top as i32,
                            z_end_aff as *const i8);
                        __state = 404;
                    }
                    408 => { __state = 409; }
                    409 => {
                        if unsafe {
                                    sqlite3_expr_is_vector(p_right_3 as *const Expr)
                                } == 0 {
                            __state = 410;
                        } else { __state = 411; }
                    }
                    410 => {
                        disable_term(unsafe { &*p_level }, p_range_end);
                        __state = 394;
                    }
                    411 => { end_eq = 1; __state = 394; }
                    412 => {
                        if reg_bignull == 0 {
                            __state = 414;
                        } else { __state = 413; }
                    }
                    413 => {
                        {
                            let __p = &mut n_constraint_1;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 394;
                    }
                    414 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 77, 0, reg_base + n_eq as i32)
                        };
                        __state = 415;
                    }
                    415 => { end_eq = 0; __state = 413; }
                    416 => {
                        if !(z_end_aff).is_null() {
                            __state = 419;
                        } else { __state = 418; }
                    }
                    417 => {
                        unsafe {
                            sqlite3_db_nn_free_nn(db, z_start_aff as *mut ())
                        };
                        __state = 416;
                    }
                    418 => {
                        unsafe {
                            (*p_level).p2 = unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 420;
                    }
                    419 => {
                        unsafe { sqlite3_db_nn_free_nn(db, z_end_aff as *mut ()) };
                        __state = 418;
                    }
                    420 => {
                        if n_constraint_1 != 0 {
                            __state = 422;
                        } else { __state = 421; }
                    }
                    421 => {
                        if reg_bignull != 0 {
                            __state = 439;
                        } else { __state = 438; }
                    }
                    422 => {
                        if reg_bignull != 0 {
                            __state = 424;
                        } else { __state = 423; }
                    }
                    423 => {
                        op__1 = a_end_op[(b_rev * 2 + end_eq) as usize] as i32;
                        __state = 427;
                    }
                    424 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 17, reg_bignull,
                                unsafe { sqlite3_vdbe_current_addr(v) } + 3)
                        };
                        __state = 425;
                    }
                    425 => { __state = 426; }
                    426 => { __state = 423; }
                    427 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, op__1, i_idx_cur, addr_nxt,
                                reg_base, n_constraint_1)
                        };
                        __state = 428;
                    }
                    428 => { __state = 429; }
                    429 => { __state = 430; }
                    430 => { __state = 431; }
                    431 => { __state = 432; }
                    432 => { __state = 433; }
                    433 => { __state = 434; }
                    434 => { __state = 435; }
                    435 => { __state = 436; }
                    436 => {
                        if addr_seek_scan != 0 {
                            __state = 437;
                        } else { __state = 421; }
                    }
                    437 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_seek_scan) };
                        __state = 421;
                    }
                    438 => {
                        if unsafe { (*p_loop).ws_flags } & 262144 as u32 != 0 as u32
                            {
                            __state = 456;
                        } else { __state = 455; }
                    }
                    439 => { { let _ = 0; }; __state = 440; }
                    440 => { { let _ = 0; }; __state = 441; }
                    441 => { { let _ = 0; }; __state = 442; }
                    442 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 16, reg_bignull,
                                unsafe { sqlite3_vdbe_current_addr(v) } + 2)
                        };
                        __state = 443;
                    }
                    443 => { __state = 444; }
                    444 => { __state = 445; }
                    445 => {
                        op__1 =
                            a_end_op[(b_rev * 2 + b_seek_past_null as i32) as usize] as
                                i32;
                        __state = 446;
                    }
                    446 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, op__1, i_idx_cur, addr_nxt,
                                reg_base, n_constraint_1 + b_seek_past_null as i32)
                        };
                        __state = 447;
                    }
                    447 => { __state = 448; }
                    448 => { __state = 449; }
                    449 => { __state = 450; }
                    450 => { __state = 451; }
                    451 => { __state = 452; }
                    452 => { __state = 453; }
                    453 => { __state = 454; }
                    454 => { __state = 438; }
                    455 => {
                        omit_table =
                            (unsafe { (*p_loop).ws_flags } & 64 as u32 != 0 as u32 &&
                                    unsafe { (*p_w_info).wctrl_flags } as i32 & (32 | 4096) ==
                                        0) as i32;
                        __state = 457;
                    }
                    456 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 127, i_idx_cur, n_eq as i32,
                                n_eq as i32)
                        };
                        __state = 455;
                    }
                    457 => {
                        if omit_table != 0 {
                            __state = 459;
                        } else { __state = 460; }
                    }
                    458 => {
                        if unsafe { (*p_level).i_left_join } == 0 {
                            __state = 473;
                        } else { __state = 474; }
                    }
                    459 => { __state = 458; }
                    460 => {
                        if unsafe { (*unsafe { (*p_idx).p_table }).tab_flags } &
                                    128 as u32 == 0 as u32 {
                            __state = 461;
                        } else { __state = 462; }
                    }
                    461 => {
                        code_deferred_seek(unsafe { &mut *p_w_info },
                            unsafe { &*p_idx }, i_cur, i_idx_cur);
                        __state = 458;
                    }
                    462 => {
                        if i_cur != i_idx_cur {
                            __state = 463;
                        } else { __state = 458; }
                    }
                    463 => {
                        p_pk =
                            unsafe {
                                    sqlite3_primary_key_index(unsafe { (*p_idx).p_table })
                                } as *const Index;
                        __state = 464;
                    }
                    464 => {
                        i_rowid_reg =
                            unsafe {
                                sqlite3_get_temp_range(p_parse,
                                    unsafe { (*p_pk).n_key_col } as i32)
                            };
                        __state = 465;
                    }
                    465 => { j = 0; __state = 467; }
                    466 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_cur, addr_cont,
                                i_rowid_reg, unsafe { (*p_pk).n_key_col } as i32)
                        };
                        __state = 471;
                    }
                    467 => {
                        if j < unsafe { (*p_pk).n_key_col } as i32 {
                            __state = 468;
                        } else { __state = 466; }
                    }
                    468 => {
                        k =
                            unsafe {
                                sqlite3_table_column_to_index(p_idx,
                                    unsafe { *unsafe { (*p_pk).ai_column.offset(j as isize) } }
                                        as i32)
                            };
                        __state = 470;
                    }
                    469 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 467;
                    }
                    470 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_idx_cur, k, i_rowid_reg + j)
                        };
                        __state = 469;
                    }
                    471 => { __state = 458; }
                    472 => {
                        if unsafe { (*p_loop).ws_flags } & 4096 as u32 != 0 ||
                                unsafe { (*p_level).u.in_.n_in } != 0 && reg_bignull == 0 &&
                                    where_loop_is_one_row(unsafe { &*p_loop }) != 0 {
                            __state = 478;
                        } else { __state = 479; }
                    }
                    473 => {
                        if !(unsafe { (*p_idx).p_part_idx_where }).is_null() &&
                                unsafe { (*p_level).p_rj } == core::ptr::null_mut() {
                            __state = 475;
                        } else { __state = 472; }
                    }
                    474 => { __state = 476; }
                    475 => {
                        where_apply_partial_index_constraints(unsafe {
                                    (*p_idx).p_part_idx_where
                                } as *const Expr, i_cur, p_wc);
                        __state = 472;
                    }
                    476 => { { let _ = 0; }; __state = 472; }
                    477 => {
                        unsafe { (*p_level).p1 = i_idx_cur };
                        __state = 482;
                    }
                    478 => {
                        unsafe { (*p_level).op = 189 as u8 };
                        __state = 477;
                    }
                    479 => {
                        if b_rev != 0 { __state = 480; } else { __state = 481; }
                    }
                    480 => {
                        unsafe { (*p_level).op = 39 as u8 };
                        __state = 477;
                    }
                    481 => {
                        unsafe { (*p_level).op = 40 as u8 };
                        __state = 477;
                    }
                    482 => {
                        unsafe {
                            (*p_level).p3 =
                                if unsafe { (*p_loop).ws_flags } & 65536 as u32 != 0 as u32
                                        {
                                        1
                                    } else { 0 } as u8
                        };
                        __state = 483;
                    }
                    483 => {
                        if unsafe { (*p_loop).ws_flags } & 15 as u32 == 0 as u32 {
                            __state = 485;
                        } else { __state = 486; }
                    }
                    484 => {
                        if omit_table != 0 { __state = 487; } else { __state = 33; }
                    }
                    485 => {
                        unsafe { (*p_level).p5 = 1 as u8 };
                        __state = 484;
                    }
                    486 => { { let _ = 0; }; __state = 484; }
                    487 => { p_idx = core::ptr::null_mut(); __state = 33; }
                    488 => { __state = 490; }
                    489 => { __state = 643; }
                    490 => { __state = 491; }
                    491 => { p_cov = core::ptr::null_mut(); __state = 492; }
                    492 => {
                        i_cov_cur =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 493;
                    }
                    493 => {
                        reg_return =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 494;
                    }
                    494 => { reg_rowset = 0; __state = 495; }
                    495 => { reg_rowid = 0; __state = 496; }
                    496 => {
                        i_loop_body = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 497;
                    }
                    497 => { __state = 498; }
                    498 => { untested_terms = 0; __state = 499; }
                    499 => { __state = 500; }
                    500 => {
                        p_and_expr = core::ptr::null_mut();
                        __state = 501;
                    }
                    501 => {
                        p_tab = unsafe { (*p_tab_item).p_s_tab };
                        __state = 502;
                    }
                    502 => {
                        p_term =
                            unsafe {
                                *unsafe { (*p_loop).a_l_term.offset(0 as isize) }
                            };
                        __state = 503;
                    }
                    503 => { { let _ = 0; }; __state = 504; }
                    504 => { { let _ = 0; }; __state = 505; }
                    505 => { { let _ = 0; }; __state = 506; }
                    506 => {
                        p_or_wc =
                            unsafe { &mut (*unsafe { (*p_term).u.p_or_info }).wc };
                        __state = 507;
                    }
                    507 => {
                        unsafe { (*p_level).op = 69 as u8 };
                        __state = 508;
                    }
                    508 => {
                        unsafe { (*p_level).p1 = reg_return };
                        __state = 509;
                    }
                    509 => {
                        if unsafe { (*p_w_info).n_level } as i32 > 1 ||
                                unsafe { (*p_tab_item).fg.from_exists() } != 0 {
                            __state = 511;
                        } else { __state = 512; }
                    }
                    510 => {
                        if unsafe { (*p_w_info).wctrl_flags } as i32 & 16 == 0 {
                            __state = 528;
                        } else { __state = 527; }
                    }
                    511 => { __state = 513; }
                    512 => {
                        p_or_tab = unsafe { (*p_w_info).p_tab_list };
                        __state = 510;
                    }
                    513 => { __state = 514; }
                    514 => {
                        n_not_ready =
                            unsafe { (*p_w_info).n_level } as i32 - i_level - 1;
                        __state = 515;
                    }
                    515 => {
                        p_or_tab =
                            unsafe {
                                    sqlite3_db_malloc_raw_nn(db,
                                        core::mem::offset_of!(SrcList, a) as u64 +
                                            (n_not_ready + 1) as u64 *
                                                core::mem::size_of::<SrcItem>() as u64)
                                } as *mut SrcList;
                        __state = 516;
                    }
                    516 => {
                        if p_or_tab == core::ptr::null_mut() {
                            __state = 518;
                        } else { __state = 517; }
                    }
                    517 => {
                        unsafe {
                            (*p_or_tab).n_alloc = (n_not_ready + 1) as u8 as u32
                        };
                        __state = 519;
                    }
                    518 => { return not_ready; }
                    519 => {
                        unsafe {
                            (*p_or_tab).n_src = unsafe { (*p_or_tab).n_alloc } as i32
                        };
                        __state = 520;
                    }
                    520 => {
                        unsafe {
                            memcpy(unsafe { (*p_or_tab).a.as_ptr() } as *mut SrcItem as
                                    *mut (), p_tab_item as *const (),
                                core::mem::size_of::<SrcItem>() as u64)
                        };
                        __state = 521;
                    }
                    521 => {
                        orig_src =
                            unsafe { (*unsafe { (*p_w_info).p_tab_list }).a.as_ptr() }
                                as *mut SrcItem;
                        __state = 522;
                    }
                    522 => { k = 1; __state = 524; }
                    523 => {
                        unsafe {
                            (*(unsafe { (*p_or_tab).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as
                                                isize)).fg.set_from_exists(0 as u32 as u32)
                        };
                        __state = 510;
                    }
                    524 => {
                        if k <= n_not_ready {
                            __state = 525;
                        } else { __state = 523; }
                    }
                    525 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *(unsafe { (*p_or_tab).a.as_ptr() } as
                                                        *mut SrcItem).offset(k as isize)
                                    } as *mut (),
                                unsafe {
                                        &raw mut *orig_src.add(unsafe {
                                                            (*p_level.offset(k as isize)).i_from
                                                        } as usize)
                                    } as *const (), core::mem::size_of::<SrcItem>() as u64)
                        };
                        __state = 526;
                    }
                    526 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 524;
                    }
                    527 => {
                        i_ret_init =
                            unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_return) };
                        __state = 536;
                    }
                    528 => {
                        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                            __state = 530;
                        } else { __state = 531; }
                    }
                    529 => {
                        reg_rowid =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 527;
                    }
                    530 => {
                        reg_rowset =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 532;
                    }
                    531 => {
                        p_pk_1 = unsafe { sqlite3_primary_key_index(p_tab) };
                        __state = 533;
                    }
                    532 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg_rowset) };
                        __state = 529;
                    }
                    533 => {
                        reg_rowset =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 534;
                    }
                    534 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 120, reg_rowset,
                                unsafe { (*p_pk_1).n_key_col } as i32)
                        };
                        __state = 535;
                    }
                    535 => {
                        unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_pk_1) };
                        __state = 529;
                    }
                    536 => {
                        if unsafe { (*p_wc).n_term } > 1 {
                            __state = 538;
                        } else { __state = 537; }
                    }
                    537 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 1 as u8,
                                c"MULTI-INDEX OR".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 558;
                    }
                    538 => { __state = 539; }
                    539 => { i_term = 0; __state = 541; }
                    540 => {
                        if !(p_and_expr).is_null() {
                            __state = 557;
                        } else { __state = 537; }
                    }
                    541 => {
                        if i_term < unsafe { (*p_wc).n_term } {
                            __state = 542;
                        } else { __state = 540; }
                    }
                    542 => {
                        p_expr =
                            unsafe {
                                (*unsafe { (*p_wc).a.offset(i_term as isize) }).p_expr
                            };
                        __state = 544;
                    }
                    543 => {
                        { let __p = &mut i_term; let __t = *__p; *__p += 1; __t };
                        __state = 541;
                    }
                    544 => {
                        if unsafe { unsafe { (*p_wc).a.offset(i_term as isize) } }
                                == p_term {
                            __state = 546;
                        } else { __state = 545; }
                    }
                    545 => { __state = 547; }
                    546 => { __state = 543; }
                    547 => { __state = 548; }
                    548 => { __state = 549; }
                    549 => {
                        if unsafe {
                                            (*unsafe { (*p_wc).a.offset(i_term as isize) }).wt_flags
                                        } as i32 & (2 | 4 | 32768) != 0 {
                            __state = 551;
                        } else { __state = 550; }
                    }
                    550 => {
                        if unsafe {
                                            (*unsafe { (*p_wc).a.offset(i_term as isize) }).e_operator
                                        } as i32 & 16383 == 0 {
                            __state = 553;
                        } else { __state = 552; }
                    }
                    551 => { __state = 543; }
                    552 => {
                        if unsafe { (*p_expr).flags } & 4194304 as u32 != 0 as u32 {
                            __state = 555;
                        } else { __state = 554; }
                    }
                    553 => { __state = 543; }
                    554 => {
                        p_expr =
                            unsafe { sqlite3_expr_dup(db, p_expr as *const Expr, 0) };
                        __state = 556;
                    }
                    555 => { __state = 543; }
                    556 => {
                        p_and_expr =
                            unsafe { sqlite3_expr_and(p_parse, p_and_expr, p_expr) };
                        __state = 543;
                    }
                    557 => {
                        p_and_expr =
                            unsafe {
                                sqlite3_p_expr(p_parse, 44 | 65536, core::ptr::null_mut(),
                                    p_and_expr)
                            };
                        __state = 537;
                    }
                    558 => { ii = 0; __state = 560; }
                    559 => {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
                        __state = 625;
                    }
                    560 => {
                        if ii < unsafe { (*p_or_wc).n_term } {
                            __state = 561;
                        } else { __state = 559; }
                    }
                    561 => {
                        p_or_term =
                            unsafe { unsafe { (*p_or_wc).a.offset(ii as isize) } } as
                                *const WhereTerm;
                        __state = 563;
                    }
                    562 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 560;
                    }
                    563 => {
                        if unsafe { (*p_or_term).left_cursor } == i_cur ||
                                unsafe { (*p_or_term).e_operator } as i32 & 1024 != 0 {
                            __state = 564;
                        } else { __state = 562; }
                    }
                    564 => { __state = 565; }
                    565 => {
                        p_or_expr = unsafe { (*p_or_term).p_expr };
                        __state = 566;
                    }
                    566 => { __state = 567; }
                    567 => { jmp1 = 0; __state = 568; }
                    568 => { __state = 569; }
                    569 => {
                        p_delete =
                            {
                                p_or_expr =
                                    unsafe {
                                        sqlite3_expr_dup(db, p_or_expr as *const Expr, 0)
                                    };
                                p_or_expr
                            };
                        __state = 570;
                    }
                    570 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 572;
                        } else { __state = 571; }
                    }
                    571 => {
                        if !(p_and_expr).is_null() {
                            __state = 575;
                        } else { __state = 574; }
                    }
                    572 => {
                        unsafe { sqlite3_expr_delete(db, p_delete) };
                        __state = 573;
                    }
                    573 => { __state = 562; }
                    574 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 1 as u8,
                                c"INDEX %d".as_ptr() as *mut i8 as *const i8, ii + 1)
                        };
                        __state = 577;
                    }
                    575 => {
                        unsafe { (*p_and_expr).p_left = p_or_expr };
                        __state = 576;
                    }
                    576 => { p_or_expr = p_and_expr; __state = 574; }
                    577 => { __state = 578; }
                    578 => {
                        p_sub_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_or_tab, p_or_expr,
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    core::ptr::null_mut(), 32 as u16, i_cov_cur)
                            };
                        __state = 579;
                    }
                    579 => { { let _ = 0; }; __state = 580; }
                    580 => {
                        if !(p_sub_w_info).is_null() {
                            __state = 582;
                        } else { __state = 581; }
                    }
                    581 => {
                        unsafe { sqlite3_expr_delete(db, p_delete) };
                        __state = 562;
                    }
                    582 => { __state = 583; }
                    583 => {
                        addr_explain =
                            sqlite3_where_explain_one_scan(p_parse, p_or_tab,
                                unsafe {
                                    &mut *(unsafe { (*p_sub_w_info).a.as_ptr() } as
                                                    *mut WhereLevel).offset(0 as isize)
                                }, 0 as u16);
                        __state = 584;
                    }
                    584 => { { let _ = addr_explain; }; __state = 585; }
                    585 => {
                        if unsafe { (*p_w_info).wctrl_flags } as i32 & 16 == 0 {
                            __state = 587;
                        } else { __state = 586; }
                    }
                    586 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_return, i_loop_body)
                        };
                        __state = 611;
                    }
                    587 => {
                        i_set =
                            if ii == unsafe { (*p_or_wc).n_term } - 1 {
                                -1
                            } else { ii };
                        __state = 588;
                    }
                    588 => {
                        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                            __state = 589;
                        } else { __state = 590; }
                    }
                    589 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_cur, -1,
                                reg_rowid)
                        };
                        __state = 591;
                    }
                    590 => {
                        p_pk_2 =
                            unsafe { sqlite3_primary_key_index(p_tab) } as *const Index;
                        __state = 593;
                    }
                    591 => {
                        jmp1 =
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 49, reg_rowset, 0, reg_rowid,
                                    i_set)
                            };
                        __state = 592;
                    }
                    592 => { __state = 586; }
                    593 => {
                        n_pk = unsafe { (*p_pk_2).n_key_col } as i32;
                        __state = 594;
                    }
                    594 => { __state = 595; }
                    595 => { __state = 596; }
                    596 => {
                        r = unsafe { sqlite3_get_temp_range(p_parse, n_pk) };
                        __state = 597;
                    }
                    597 => { i_pk = 0; __state = 599; }
                    598 => {
                        if i_set != 0 { __state = 604; } else { __state = 603; }
                    }
                    599 => {
                        if i_pk < n_pk { __state = 600; } else { __state = 598; }
                    }
                    600 => {
                        i_col =
                            unsafe {
                                    *unsafe { (*p_pk_2).ai_column.offset(i_pk as isize) }
                                } as i32;
                        __state = 602;
                    }
                    601 => {
                        { let __p = &mut i_pk; let __t = *__p; *__p += 1; __t };
                        __state = 599;
                    }
                    602 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_cur,
                                i_col, r + i_pk)
                        };
                        __state = 601;
                    }
                    603 => {
                        if i_set >= 0 { __state = 607; } else { __state = 606; }
                    }
                    604 => {
                        jmp1 =
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, reg_rowset, 0, r, n_pk)
                            };
                        __state = 605;
                    }
                    605 => { __state = 603; }
                    606 => {
                        unsafe { sqlite3_release_temp_range(p_parse, r, n_pk) };
                        __state = 586;
                    }
                    607 => {
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r, n_pk, reg_rowid) };
                        __state = 608;
                    }
                    608 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, reg_rowset, reg_rowid, r,
                                n_pk)
                        };
                        __state = 609;
                    }
                    609 => {
                        if i_set != 0 { __state = 610; } else { __state = 606; }
                    }
                    610 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        __state = 606;
                    }
                    611 => {
                        if jmp1 != 0 { __state = 613; } else { __state = 612; }
                    }
                    612 => {
                        if unsafe { (*p_sub_w_info).untested_terms() } != 0 {
                            __state = 615;
                        } else { __state = 614; }
                    }
                    613 => {
                        unsafe { sqlite3_vdbe_jump_here(v, jmp1) };
                        __state = 612;
                    }
                    614 => {
                        p_sub_loop =
                            unsafe {
                                (*(unsafe { (*p_sub_w_info).a.as_ptr() } as
                                                *mut WhereLevel).offset(0 as isize)).p_w_loop
                            };
                        __state = 616;
                    }
                    615 => { untested_terms = 1; __state = 614; }
                    616 => { { let _ = 0; }; __state = 617; }
                    617 => {
                        if unsafe { (*p_sub_loop).ws_flags } & 512 as u32 !=
                                        0 as u32 &&
                                    (ii == 0 ||
                                        unsafe { (*p_sub_loop).u.btree.p_index } == p_cov) &&
                                (unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 ||
                                    !(unsafe {
                                                            (*unsafe { (*p_sub_loop).u.btree.p_index }).idx_type()
                                                        } as i32 == 2) as i32 != 0) {
                            __state = 619;
                        } else { __state = 620; }
                    }
                    618 => {
                        if unsafe { sqlite3_where_uses_deferred_seek(p_sub_w_info) }
                                != 0 {
                            __state = 623;
                        } else { __state = 622; }
                    }
                    619 => { { let _ = 0; }; __state = 621; }
                    620 => { p_cov = core::ptr::null_mut(); __state = 618; }
                    621 => {
                        p_cov = unsafe { (*p_sub_loop).u.btree.p_index };
                        __state = 618;
                    }
                    622 => {
                        unsafe { sqlite3_where_end(p_sub_w_info) };
                        __state = 624;
                    }
                    623 => {
                        unsafe { (*p_w_info).set_b_deferred_seek(1 as u32 as u32) };
                        __state = 622;
                    }
                    624 => {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
                        __state = 581;
                    }
                    625 => { { let _ = 0; }; __state = 626; }
                    626 => { { let _ = 0; }; __state = 627; }
                    627 => { { let _ = 0; }; __state = 628; }
                    628 => {
                        unsafe { (*p_level).u.p_covering_idx = p_cov };
                        __state = 629;
                    }
                    629 => {
                        if !(p_cov).is_null() {
                            __state = 631;
                        } else { __state = 630; }
                    }
                    630 => {
                        if !(p_and_expr).is_null() {
                            __state = 633;
                        } else { __state = 632; }
                    }
                    631 => {
                        unsafe { (*p_level).i_idx_cur = i_cov_cur };
                        __state = 630;
                    }
                    632 => {
                        unsafe {
                            sqlite3_vdbe_change_p1(v, i_ret_init,
                                unsafe { sqlite3_vdbe_current_addr(v) })
                        };
                        __state = 635;
                    }
                    633 => {
                        unsafe { (*p_and_expr).p_left = core::ptr::null_mut() };
                        __state = 634;
                    }
                    634 => {
                        unsafe { sqlite3_expr_delete(db, p_and_expr) };
                        __state = 632;
                    }
                    635 => {
                        unsafe {
                            sqlite3_vdbe_goto(v, unsafe { (*p_level).addr_brk })
                        };
                        __state = 636;
                    }
                    636 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_loop_body) };
                        __state = 637;
                    }
                    637 => { { let _ = 0; }; __state = 638; }
                    638 => {
                        unsafe {
                            (*p_level).p2 = unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 639;
                    }
                    639 => {
                        if unsafe { (*p_w_info).p_tab_list } != p_or_tab {
                            __state = 641;
                        } else { __state = 640; }
                    }
                    640 => {
                        if (untested_terms == 0) as i32 != 0 {
                            __state = 642;
                        } else { __state = 33; }
                    }
                    641 => {
                        unsafe { sqlite3_db_free_nn(db, p_or_tab as *mut ()) };
                        __state = 640;
                    }
                    642 => {
                        disable_term(unsafe { &*p_level }, p_term);
                        __state = 33;
                    }
                    643 => { __state = 644; }
                    644 => { { let _ = 0; }; __state = 645; }
                    645 => {
                        if unsafe { (*p_tab_item).fg.is_recursive() } != 0 {
                            __state = 646;
                        } else { __state = 647; }
                    }
                    646 => {
                        unsafe { (*p_level).op = 189 as u8 };
                        __state = 33;
                    }
                    647 => { __state = 648; }
                    648 => {
                        unsafe { (*p_level).op = a_step[b_rev as usize] as u8 };
                        __state = 649;
                    }
                    649 => { unsafe { (*p_level).p1 = i_cur }; __state = 650; }
                    650 => {
                        unsafe {
                            (*p_level).p2 =
                                1 +
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, a_start[b_rev as usize] as i32,
                                            i_cur, unsafe { (*p_level).addr_halt })
                                    }
                        };
                        __state = 651;
                    }
                    651 => { __state = 652; }
                    652 => { __state = 653; }
                    653 => { unsafe { (*p_level).p5 = 1 as u8 }; __state = 33; }
                    654 => { i_next = 0; __state = 657; }
                    655 => {
                        {
                            p_term = unsafe { (*p_wc).a };
                            j = unsafe { (*p_wc).n_base }
                        };
                        __state = 699;
                    }
                    656 => {
                        if i_loop > 0 { __state = 654; } else { __state = 655; }
                    }
                    657 => {
                        {
                            p_term = unsafe { (*p_wc).a };
                            j = unsafe { (*p_wc).n_term }
                        };
                        __state = 659;
                    }
                    658 => { i_loop = i_next; __state = 656; }
                    659 => {
                        if j > 0 { __state = 660; } else { __state = 658; }
                    }
                    660 => { __state = 662; }
                    661 => {
                        {
                            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 659;
                    }
                    662 => { skip_like_addr = 0; __state = 663; }
                    663 => { __state = 664; }
                    664 => { __state = 665; }
                    665 => {
                        if unsafe { (*p_term).wt_flags } as i32 & (2 | 4) != 0 {
                            __state = 667;
                        } else { __state = 666; }
                    }
                    666 => {
                        if unsafe { (*p_term).prereq_all } &
                                    unsafe { (*p_level).not_ready } != 0 as u64 {
                            __state = 669;
                        } else { __state = 668; }
                    }
                    667 => { __state = 661; }
                    668 => { p_e = unsafe { (*p_term).p_expr }; __state = 672; }
                    669 => { __state = 670; }
                    670 => {
                        unsafe { (*p_w_info).set_untested_terms(1 as u32 as u32) };
                        __state = 671;
                    }
                    671 => { __state = 661; }
                    672 => { { let _ = 0; }; __state = 673; }
                    673 => {
                        if unsafe { (*p_tab_item).fg.jointype } as i32 &
                                    (8 | 64 | 16) != 0 {
                            __state = 675;
                        } else { __state = 674; }
                    }
                    674 => {
                        if i_loop == 1 &&
                                (unsafe {
                                                sqlite3_expr_covered_by_index(p_e,
                                                    unsafe { (*p_level).i_tab_cur }, p_idx)
                                            } == 0) as i32 != 0 {
                            __state = 683;
                        } else { __state = 682; }
                    }
                    675 => {
                        if !(unsafe { (*p_e).flags } & (1 | 2) as u32 != 0 as u32)
                                    as i32 != 0 {
                            __state = 676;
                        } else { __state = 677; }
                    }
                    676 => { __state = 661; }
                    677 => {
                        if unsafe { (*p_tab_item).fg.jointype } as i32 & 8 == 8 &&
                                !(unsafe { (*p_e).flags } & 1 as u32 != 0 as u32) as i32 !=
                                    0 {
                            __state = 678;
                        } else { __state = 679; }
                    }
                    678 => { __state = 661; }
                    679 => {
                        m =
                            unsafe {
                                sqlite3_where_get_mask(unsafe {
                                        &mut (*p_w_info).s_mask_set
                                    }, unsafe { (*p_e).w.i_join })
                            };
                        __state = 680;
                    }
                    680 => {
                        if m & unsafe { (*p_level).not_ready } != 0 {
                            __state = 681;
                        } else { __state = 674; }
                    }
                    681 => { __state = 661; }
                    682 => {
                        if i_loop < 3 &&
                                unsafe { (*p_term).wt_flags } as i32 & 4096 != 0 {
                            __state = 686;
                        } else { __state = 685; }
                    }
                    683 => { i_next = 2; __state = 684; }
                    684 => { __state = 661; }
                    685 => {
                        if unsafe { (*p_term).wt_flags } as i32 & 512 != 0 {
                            __state = 690;
                        } else { __state = 689; }
                    }
                    686 => {
                        if i_next == 0 { __state = 688; } else { __state = 687; }
                    }
                    687 => { __state = 661; }
                    688 => { i_next = 3; __state = 687; }
                    689 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_e, addr_cont, 16)
                        };
                        __state = 695;
                    }
                    690 => {
                        x = unsafe { (*p_level).i_like_rep_cntr };
                        __state = 691;
                    }
                    691 => {
                        if x > 0 as u32 { __state = 692; } else { __state = 689; }
                    }
                    692 => {
                        skip_like_addr =
                            unsafe {
                                sqlite3_vdbe_add_op1(v,
                                    if x & 1 as u32 != 0 { 17 } else { 16 }, (x >> 1) as i32)
                            };
                        __state = 693;
                    }
                    693 => { __state = 694; }
                    694 => { __state = 689; }
                    695 => {
                        if skip_like_addr != 0 {
                            __state = 697;
                        } else { __state = 696; }
                    }
                    696 => {
                        unsafe { (*p_term).wt_flags |= 4 as u16 };
                        __state = 661;
                    }
                    697 => {
                        unsafe { sqlite3_vdbe_jump_here(v, skip_like_addr) };
                        __state = 696;
                    }
                    698 => {
                        if !(unsafe { (*p_level).p_rj }).is_null() {
                            __state = 735;
                        } else { __state = 734; }
                    }
                    699 => {
                        if j > 0 { __state = 700; } else { __state = 698; }
                    }
                    700 => { __state = 702; }
                    701 => {
                        {
                            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 699;
                    }
                    702 => { __state = 703; }
                    703 => {
                        if unsafe { (*p_term).wt_flags } as i32 & (2 | 4) != 0 {
                            __state = 705;
                        } else { __state = 704; }
                    }
                    704 => {
                        if unsafe { (*p_term).e_operator } as i32 & (2 | 128) == 0 {
                            __state = 707;
                        } else { __state = 706; }
                    }
                    705 => { __state = 701; }
                    706 => {
                        if unsafe { (*p_term).e_operator } as i32 & 2048 == 0 {
                            __state = 709;
                        } else { __state = 708; }
                    }
                    707 => { __state = 701; }
                    708 => {
                        if unsafe { (*p_term).left_cursor } != i_cur {
                            __state = 711;
                        } else { __state = 710; }
                    }
                    709 => { __state = 701; }
                    710 => {
                        if unsafe { (*p_tab_item).fg.jointype } as i32 &
                                    (8 | 64 | 16) != 0 {
                            __state = 713;
                        } else { __state = 712; }
                    }
                    711 => { __state = 701; }
                    712 => {
                        p_e_1 = unsafe { (*p_term).p_expr };
                        __state = 714;
                    }
                    713 => { __state = 701; }
                    714 => { { let _ = 0; }; __state = 715; }
                    715 => { { let _ = 0; }; __state = 716; }
                    716 => { { let _ = 0; }; __state = 717; }
                    717 => {
                        p_alt =
                            unsafe {
                                sqlite3_where_find_term(p_wc, i_cur,
                                    unsafe { (*p_term).u.x.left_column }, not_ready,
                                    (2 | 1 | 128) as u32, core::ptr::null_mut())
                            };
                        __state = 718;
                    }
                    718 => {
                        if p_alt == core::ptr::null_mut() {
                            __state = 720;
                        } else { __state = 719; }
                    }
                    719 => {
                        if unsafe { (*p_alt).wt_flags } as i32 & 4 != 0 {
                            __state = 722;
                        } else { __state = 721; }
                    }
                    720 => { __state = 701; }
                    721 => {
                        if unsafe { (*unsafe { (*p_alt).p_expr }).flags } &
                                    512 as u32 != 0 as u32 {
                            __state = 724;
                        } else { __state = 723; }
                    }
                    722 => { __state = 701; }
                    723 => {
                        if unsafe { (*p_alt).e_operator } as i32 & 1 != 0 &&
                                    unsafe { (*unsafe { (*p_alt).p_expr }).flags } & 4096 as u32
                                        != 0 as u32 &&
                                unsafe {
                                        (*unsafe {
                                                        (*unsafe {
                                                                        (*unsafe { (*p_alt).p_expr }).x.p_select
                                                                    }).p_e_list
                                                    }).n_expr
                                    } > 1 {
                            __state = 726;
                        } else { __state = 725; }
                    }
                    724 => { __state = 701; }
                    725 => { __state = 727; }
                    726 => { __state = 701; }
                    727 => { __state = 728; }
                    728 => { __state = 729; }
                    729 => { __state = 730; }
                    730 => {
                        s_e_alt =
                            unsafe { core::ptr::read(unsafe { (*p_alt).p_expr }) };
                        __state = 731;
                    }
                    731 => {
                        s_e_alt.p_left = unsafe { (*p_e_1).p_left };
                        __state = 732;
                    }
                    732 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, &mut s_e_alt, addr_cont, 16)
                        };
                        __state = 733;
                    }
                    733 => {
                        unsafe { (*p_alt).wt_flags |= 4 as u16 };
                        __state = 701;
                    }
                    734 => {
                        if unsafe { (*p_level).i_left_join } != 0 {
                            __state = 764;
                        } else { __state = 763; }
                    }
                    735 => { __state = 736; }
                    736 => { __state = 737; }
                    737 => { __state = 738; }
                    738 => { jmp1__1 = 0; __state = 739; }
                    739 => {
                        p_rj = unsafe { (*p_level).p_rj } as *const WhereRightJoin;
                        __state = 740;
                    }
                    740 => {
                        p_tab_1 =
                            unsafe {
                                (*(unsafe {
                                                    (*unsafe { (*p_w_info).p_tab_list }).a.as_ptr()
                                                } as
                                                *mut SrcItem).add(unsafe { (*p_level).i_from } as
                                                usize)).p_s_tab
                            };
                        __state = 741;
                    }
                    741 => {
                        if unsafe { (*p_tab_1).tab_flags } & 128 as u32 == 0 as u32
                            {
                            __state = 743;
                        } else { __state = 744; }
                    }
                    742 => {
                        jmp1__1 =
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, unsafe { (*p_rj).i_match },
                                    0, r__1 + 1, n_pk_1)
                            };
                        __state = 755;
                    }
                    743 => {
                        r__1 = unsafe { sqlite3_get_temp_range(p_parse, 2) };
                        __state = 745;
                    }
                    744 => { __state = 747; }
                    745 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab_1,
                                unsafe { (*p_level).i_tab_cur }, -1, r__1 + 1)
                        };
                        __state = 746;
                    }
                    746 => { n_pk_1 = 1; __state = 742; }
                    747 => {
                        p_pk_3 =
                            unsafe { sqlite3_primary_key_index(p_tab_1) } as
                                *const Index;
                        __state = 748;
                    }
                    748 => {
                        n_pk_1 = unsafe { (*p_pk_3).n_key_col } as i32;
                        __state = 749;
                    }
                    749 => {
                        r__1 =
                            unsafe { sqlite3_get_temp_range(p_parse, n_pk_1 + 1) };
                        __state = 750;
                    }
                    750 => { i_pk_1 = 0; __state = 751; }
                    751 => {
                        if i_pk_1 < n_pk_1 {
                            __state = 752;
                        } else { __state = 742; }
                    }
                    752 => {
                        i_col_1 =
                            unsafe {
                                    *unsafe { (*p_pk_3).ai_column.offset(i_pk_1 as isize) }
                                } as i32;
                        __state = 754;
                    }
                    753 => {
                        { let __p = &mut i_pk_1; let __t = *__p; *__p += 1; __t };
                        __state = 751;
                    }
                    754 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab_1, i_cur,
                                i_col_1, r__1 + 1 + i_pk_1)
                        };
                        __state = 753;
                    }
                    755 => { __state = 756; }
                    756 => { __state = 757; }
                    757 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, r__1 + 1, n_pk_1, r__1)
                        };
                        __state = 758;
                    }
                    758 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, unsafe { (*p_rj).i_match },
                                r__1, r__1 + 1, n_pk_1)
                        };
                        __state = 759;
                    }
                    759 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 185,
                                unsafe { (*p_rj).reg_bloom }, 0, r__1 + 1, n_pk_1)
                        };
                        __state = 760;
                    }
                    760 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        __state = 761;
                    }
                    761 => {
                        unsafe { sqlite3_vdbe_jump_here(v, jmp1__1) };
                        __state = 762;
                    }
                    762 => {
                        unsafe {
                            sqlite3_release_temp_range(p_parse, r__1, n_pk_1 + 1)
                        };
                        __state = 734;
                    }
                    763 => {
                        if !(unsafe { (*p_level).p_rj }).is_null() {
                            __state = 770;
                        } else { __state = 769; }
                    }
                    764 => {
                        unsafe {
                            (*p_level).addr_first =
                                unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 765;
                    }
                    765 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1,
                                unsafe { (*p_level).i_left_join })
                        };
                        __state = 766;
                    }
                    766 => { __state = 767; }
                    767 => {
                        if unsafe { (*p_level).p_rj } == core::ptr::null_mut() {
                            __state = 768;
                        } else { __state = 763; }
                    }
                    768 => { __state = 2; }
                    769 => { return unsafe { (*p_level).not_ready }; }
                    770 => {
                        p_rj_1 = unsafe { (*p_level).p_rj };
                        __state = 771;
                    }
                    771 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 76, 0,
                                unsafe { (*p_rj_1).reg_return })
                        };
                        __state = 772;
                    }
                    772 => {
                        unsafe {
                            (*p_rj_1).addr_subrtn =
                                unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 773;
                    }
                    773 => { { let _ = 0; }; __state = 774; }
                    774 => {
                        {
                            let __p = unsafe { &mut (*p_parse).within_rj_subrtn };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 775;
                    }
                    775 => { __state = 2; }
                    776 => {
                        if j < unsafe { (*p_wc).n_base } {
                            __state = 777;
                        } else { __state = 769; }
                    }
                    777 => { __state = 779; }
                    778 => {
                        {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 776;
                    }
                    779 => { __state = 780; }
                    780 => {
                        if unsafe { (*p_term).wt_flags } as i32 & (2 | 4) != 0 {
                            __state = 782;
                        } else { __state = 781; }
                    }
                    781 => {
                        if unsafe { (*p_term).prereq_all } &
                                    unsafe { (*p_level).not_ready } != 0 as u64 {
                            __state = 784;
                        } else { __state = 783; }
                    }
                    782 => { __state = 778; }
                    783 => {
                        if unsafe { (*p_tab_item).fg.jointype } as i32 & 64 != 0 {
                            __state = 787;
                        } else { __state = 786; }
                    }
                    784 => { { let _ = 0; }; __state = 785; }
                    785 => { __state = 778; }
                    786 => { { let _ = 0; }; __state = 788; }
                    787 => { __state = 778; }
                    788 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, unsafe { (*p_term).p_expr },
                                addr_cont, 16)
                        };
                        __state = 789;
                    }
                    789 => {
                        unsafe { (*p_term).wt_flags |= 4 as u16 };
                        __state = 778;
                    }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_right_join_loop(p_w_info: &mut WhereInfo,
    i_level: i32, p_level: &WhereLevel) -> () {
    unsafe {
        let p_parse: *mut Parse = (*p_w_info).p_parse;
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        let p_rj: *const WhereRightJoin =
            (*p_level).p_rj as *const WhereRightJoin;
        let mut p_sub_where: *mut Expr = core::ptr::null_mut();
        let p_wc: *const WhereClause =
            &raw mut (*p_w_info).s_wc as *const WhereClause;
        let mut p_sub_w_info: *mut WhereInfo = core::ptr::null_mut();
        let p_loop: *const WhereLoop =
            (*p_level).p_w_loop as *const WhereLoop;
        let p_tab_item: *const SrcItem =
            unsafe {
                    &raw mut *(unsafe { (*(*p_w_info).p_tab_list).a.as_ptr() }
                                    as *mut SrcItem).add((*p_level).i_from as usize)
                } as *const SrcItem;
        let mut p_from: *mut SrcList = core::ptr::null_mut();
        let mut u_src:
                Sqlite3WhereRightJoinLoopU0N32sqlite3WhereRightJoinLoopU0 =
            unsafe { core::mem::zeroed() };
        let mut m_all: Bitmask = 0 as Bitmask;
        let mut k: i32 = 0;
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"RIGHT-JOIN %s".as_ptr() as *mut i8 as *const i8,
                unsafe { (*unsafe { (*p_tab_item).p_s_tab }).z_name })
        };
        {
            k = 0;
            '__b26: loop {
                if !(k < i_level) { break '__b26; }
                '__c26: loop {
                    let mut i_idx_cur: i32 = 0;
                    let mut p_right: *const SrcItem = core::ptr::null();
                    { let _ = 0; };
                    p_right =
                        unsafe {
                            &mut *(unsafe { (*(*p_w_info).p_tab_list).a.as_ptr() } as
                                            *mut SrcItem).add(unsafe {
                                                (*((*p_w_info).a.as_ptr() as
                                                                *mut WhereLevel).offset(k as isize)).i_from
                                            } as usize)
                        };
                    m_all |=
                        unsafe {
                            (*unsafe {
                                            (*((*p_w_info).a.as_ptr() as
                                                            *mut WhereLevel).offset(k as isize)).p_w_loop
                                        }).mask_self
                        };
                    if unsafe { (*p_right).fg.via_coroutine() } != 0 {
                        let mut p_subq: *const Subquery = core::ptr::null();
                        { let _ = 0; };
                        p_subq = unsafe { (*p_right).u4.p_subq };
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 77, 0,
                                unsafe { (*p_subq).reg_result },
                                unsafe { (*p_subq).reg_result } +
                                        unsafe {
                                            (*unsafe {
                                                            (*unsafe { (*p_subq).p_select }).p_e_list
                                                        }).n_expr
                                        } - 1)
                        };
                    }
                    unsafe {
                        sqlite3_vdbe_add_op1(v, 138,
                            unsafe {
                                (*((*p_w_info).a.as_ptr() as
                                                *mut WhereLevel).offset(k as isize)).i_tab_cur
                            })
                    };
                    i_idx_cur =
                        unsafe {
                            (*((*p_w_info).a.as_ptr() as
                                            *mut WhereLevel).offset(k as isize)).i_idx_cur
                        };
                    if i_idx_cur != 0 {
                        unsafe { sqlite3_vdbe_add_op1(v, 138, i_idx_cur) };
                    }
                    break '__c26;
                }
                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_tab_item).fg.jointype } as i32 & 64 == 0 {
            m_all |= unsafe { (*p_loop).mask_self };
            {
                k = 0;
                '__b27: loop {
                    if !(k < unsafe { (*p_wc).n_term }) { break '__b27; }
                    '__c27: loop {
                        let p_term: *const WhereTerm =
                            unsafe { &raw mut *unsafe { (*p_wc).a.offset(k as isize) } }
                                as *const WhereTerm;
                        if unsafe { (*p_term).wt_flags } as i32 & (2 | 32768) != 0
                                && unsafe { (*p_term).e_operator } as i32 != 8192 {
                            break '__b27;
                        }
                        if unsafe { (*p_term).prereq_all } & !m_all != 0 {
                            break '__c27;
                        }
                        if unsafe { (*unsafe { (*p_term).p_expr }).flags } &
                                    (1 | 2) as u32 != 0 as u32 {
                            break '__c27;
                        }
                        p_sub_where =
                            unsafe {
                                sqlite3_expr_and(p_parse, p_sub_where,
                                    unsafe {
                                        sqlite3_expr_dup(unsafe { (*p_parse).db },
                                            unsafe { (*p_term).p_expr } as *const Expr, 0)
                                    })
                            };
                        break '__c27;
                    }
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if (*p_level).i_idx_cur != 0 {
            unsafe { sqlite3_vdbe_add_op1(v, 138, (*p_level).i_idx_cur) };
        }
        p_from = &mut u_src.s_src;
        unsafe { (*p_from).n_src = 1 };
        unsafe { (*p_from).n_alloc = 1 as u32 };
        unsafe {
            memcpy(unsafe {
                        &raw mut *(unsafe { (*p_from).a.as_ptr() } as
                                        *mut SrcItem).offset(0 as isize)
                    } as *mut (), p_tab_item as *const (),
                core::mem::size_of::<SrcItem>() as u64)
        };
        unsafe {
            (*(unsafe { (*p_from).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).fg.jointype = 0 as u8
        };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p_parse).within_rj_subrtn };
            let __t = *__p;
            *__p += 1;
            __t
        };
        p_sub_w_info =
            unsafe {
                sqlite3_where_begin(p_parse, p_from, p_sub_where,
                    core::ptr::null_mut(), core::ptr::null_mut(),
                    core::ptr::null_mut(), 4096 as u16, 0)
            };
        if !(p_sub_w_info).is_null() {
            let i_cur: i32 = (*p_level).i_tab_cur;
            let r: i32 =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
            let mut n_pk: i32 = 0;
            let mut jmp: i32 = 0;
            let addr_cont: i32 =
                unsafe { sqlite3_where_continue_label(p_sub_w_info) };
            let p_tab: *mut Table = unsafe { (*p_tab_item).p_s_tab };
            if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                unsafe {
                    sqlite3_expr_code_get_column_of_table(v, p_tab, i_cur, -1,
                        r)
                };
                n_pk = 1;
            } else {
                let mut i_pk: i32 = 0;
                let p_pk: *const Index =
                    unsafe { sqlite3_primary_key_index(p_tab) } as *const Index;
                n_pk = unsafe { (*p_pk).n_key_col } as i32;
                unsafe { (*p_parse).n_mem += n_pk - 1 };
                {
                    i_pk = 0;
                    '__b28: loop {
                        if !(i_pk < n_pk) { break '__b28; }
                        '__c28: loop {
                            let i_col: i32 =
                                unsafe {
                                        *unsafe { (*p_pk).ai_column.offset(i_pk as isize) }
                                    } as i32;
                            unsafe {
                                sqlite3_expr_code_get_column_of_table(v, p_tab, i_cur,
                                    i_col, r + i_pk)
                            };
                            break '__c28;
                        }
                        { let __p = &mut i_pk; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            jmp =
                unsafe {
                    sqlite3_vdbe_add_op4_int(v, 66,
                        unsafe { (*p_rj).reg_bloom }, 0, r, n_pk)
                };
            unsafe {
                sqlite3_vdbe_add_op4_int(v, 29, unsafe { (*p_rj).i_match },
                    addr_cont, r, n_pk)
            };
            unsafe { sqlite3_vdbe_jump_here(v, jmp) };
            unsafe {
                sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_rj).reg_return },
                    unsafe { (*p_rj).addr_subrtn })
            };
            unsafe { sqlite3_where_end(p_sub_w_info) };
        }
        unsafe { sqlite3_expr_delete(unsafe { (*p_parse).db }, p_sub_where) };
        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p_parse).within_rj_subrtn };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
union Sqlite3WhereRightJoinLoopU0N32sqlite3WhereRightJoinLoopU0 {
    s_src: SrcList,
    from_space: [u8; 80],
}

static a_start_op: [u8; 8] =
    [0 as u8, 0 as u8, 36 as u8, 32 as u8, 24 as u8, 21 as u8, 23 as u8,
            22 as u8];

static a_end_op: [u8; 4] = [46 as u8, 42 as u8, 41 as u8, 45 as u8];

static a_step: [u8; 2] = [40 as u8, 39 as u8];

static a_start: [u8; 2] = [36 as u8, 32 as u8];

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
    fn sqlite3_where_get_mask(_: *mut WhereMaskSet, _: i32)
    -> Bitmask;
    fn sqlite3_where_find_term(p_wc_1: *mut WhereClause, i_cur_1: i32,
    i_column_1: i32, not_ready_1: Bitmask, op: u32, p_idx_1: *mut Index)
    -> *mut WhereTerm;
    fn sqlite3_where_malloc(p_w_info_1: *mut WhereInfo, n_byte_1: u64)
    -> *mut ();
    fn sqlite3_where_realloc(p_w_info_1: *mut WhereInfo, p_old_1: *mut (),
    n_byte_1: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_where_clause_init(_: *mut WhereClause, _: *mut WhereInfo)
    -> ();
    fn sqlite3_where_clause_clear(_: *mut WhereClause)
    -> ();
    fn sqlite3_where_split(_: *mut WhereClause, _: *mut Expr, _: u8)
    -> ();
    fn sqlite3_where_add_limit(_: *mut WhereClause, _: *mut Select)
    -> ();
    fn sqlite3_where_expr_usage(_: *mut WhereMaskSet, _: *mut Expr)
    -> Bitmask;
    fn sqlite3_where_expr_usage_nn(_: *mut WhereMaskSet, _: *mut Expr)
    -> Bitmask;
    fn sqlite3_where_expr_list_usage(_: *mut WhereMaskSet, _: *mut ExprList)
    -> Bitmask;
    fn sqlite3_where_expr_analyze(_: *mut SrcList, _: *mut WhereClause)
    -> ();
    fn sqlite3_where_tab_func_args(_: *mut Parse, _: *mut SrcItem,
    _: *mut WhereClause)
    -> ();
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
