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
mod vdbe_int_h;
pub(crate) use crate::vdbe_int_h::*;

type DarwinIntptrT = i64;

type DarwinSizeT = u64;

impl Vdbe {
    fn expired(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x3u32) as i32 }
    fn set_expired(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x3u32) | ((val & 0x3u32) << 0u32);
    }
    fn explain(&self) -> i32 { ((self._bitfield_1 >> 2u32) & 0x3u32) as i32 }
    fn set_explain(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x3u32 << 2u32)) | ((val & 0x3u32) << 2u32);
    }
    fn change_cnt_on(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_change_cnt_on(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn uses_stmt_journal(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_uses_stmt_journal(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn read_only(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_read_only(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn b_is_reader(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_b_is_reader(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn have_eqp_ops(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_have_eqp_ops(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
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

impl VdbeCursor {
    fn is_ephemeral(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_is_ephemeral(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn use_random_rowid(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_use_random_rowid(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn is_ordered(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_is_ordered(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn no_reuse(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_no_reuse(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn col_cache(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_col_cache(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
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

#[repr(C)]
#[derive(Copy, Clone)]
struct CompareInfo {
    match_all: u8,
    match_one: u8,
    match_set: u8,
    no_case: u8,
}

extern "C" fn pattern_compare(mut z_pattern_1: *const u8,
    mut z_string_1: *const u8, p_info_1: *const CompareInfo,
    match_other_1: u32) -> i32 {
    unsafe {
        let mut c: u32 = 0 as u32;
        let mut c2: u32 = 0 as u32;
        let match_one: u32 = unsafe { (*p_info_1).match_one } as u32;
        let match_all: u32 = unsafe { (*p_info_1).match_all } as u32;
        let no_case: u8 = unsafe { (*p_info_1).no_case } as u8;
        let mut z_escaped: *const u8 = core::ptr::null();
        while {
                    c =
                        if (unsafe { *z_pattern_1.offset(0 as isize) } as i32) < 128
                            {
                            (unsafe {
                                    *{
                                            let __p = &mut z_pattern_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                }) as u32
                        } else { unsafe { sqlite3_utf8_read(&mut z_pattern_1) } };
                    c
                } != 0 as u32 {
            if c == match_all {
                while {
                                c =
                                    if (unsafe { *z_pattern_1.offset(0 as isize) } as i32) < 128
                                        {
                                        (unsafe {
                                                *{
                                                        let __p = &mut z_pattern_1;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            }) as u32
                                    } else { unsafe { sqlite3_utf8_read(&mut z_pattern_1) } };
                                c
                            } == match_all || c == match_one && match_one != 0 as u32 {
                    if c == match_one &&
                            unsafe { sqlite3_utf8_read(&mut z_string_1) } == 0 as u32 {
                        return 2;
                    }
                }
                if c == 0 as u32 {
                    return 0;
                } else if c == match_other_1 {
                    if unsafe { (*p_info_1).match_set } as i32 == 0 {
                        c = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                        if c == 0 as u32 { return 2; }
                    } else {
                        { let _ = 0; };
                        while unsafe { *z_string_1 } != 0 {
                            let b_match: i32 =
                                pattern_compare(unsafe {
                                        &*z_pattern_1.offset(-1 as isize)
                                    }, z_string_1, p_info_1, match_other_1);
                            if b_match != 1 { return b_match; }
                            {
                                if unsafe {
                                                *{
                                                        let __p = &mut z_string_1;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32 >= 192 {
                                    while unsafe { *z_string_1 } as i32 & 192 == 128 {
                                        {
                                            let __p = &mut z_string_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                }
                            }
                        }
                        return 2;
                    }
                }
                if c < 128 as u32 {
                    let mut z_stop: [i8; 3] = [0; 3];
                    let mut b_match_1: i32 = 0;
                    if no_case != 0 {
                        z_stop[0 as usize] =
                            (c &
                                    !(unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(c as u8 as usize)
                                                    } as i32 & 32) as u32) as i8;
                        z_stop[1 as usize] =
                            unsafe {
                                    *(sqlite3_upper_to_lower.as_ptr() as
                                                *const u8).add(c as u8 as usize)
                                } as i8;
                        z_stop[2 as usize] = 0 as i8;
                    } else {
                        z_stop[0 as usize] = c as i8;
                        z_stop[1 as usize] = 0 as i8;
                    }
                    loop {
                        {
                            let __n =
                                unsafe {
                                    strcspn(z_string_1 as *const i8,
                                        &raw mut z_stop[0 as usize] as *mut i8 as *const i8)
                                };
                            let __p = &mut z_string_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        if unsafe { *z_string_1.offset(0 as isize) } as i32 == 0 {
                            break;
                        }
                        {
                            let __p = &mut z_string_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        b_match_1 =
                            pattern_compare(z_pattern_1, z_string_1, p_info_1,
                                match_other_1);
                        if b_match_1 != 1 { return b_match_1; }
                    }
                } else {
                    let mut b_match_2: i32 = 0;
                    while {
                                c2 =
                                    if (unsafe { *z_string_1.offset(0 as isize) } as i32) < 128
                                        {
                                        (unsafe {
                                                *{
                                                        let __p = &mut z_string_1;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            }) as u32
                                    } else { unsafe { sqlite3_utf8_read(&mut z_string_1) } };
                                c2
                            } != 0 as u32 {
                        if c2 != c { continue; }
                        b_match_2 =
                            pattern_compare(z_pattern_1, z_string_1, p_info_1,
                                match_other_1);
                        if b_match_2 != 1 { return b_match_2; }
                    }
                }
                return 2;
            }
            if c == match_other_1 {
                if unsafe { (*p_info_1).match_set } as i32 == 0 {
                    c = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                    if c == 0 as u32 { return 1; }
                    z_escaped = z_pattern_1;
                } else {
                    let mut prior_c: u32 = 0 as u32;
                    let mut seen: i32 = 0;
                    let mut invert: i32 = 0;
                    c = unsafe { sqlite3_utf8_read(&mut z_string_1) };
                    if c == 0 as u32 { return 1; }
                    c2 = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                    if c2 == '^' as i32 as u32 {
                        invert = 1;
                        c2 = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                    }
                    if c2 == ']' as i32 as u32 {
                        if c == ']' as i32 as u32 { seen = 1; }
                        c2 = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                    }
                    while c2 != 0 && c2 != ']' as i32 as u32 {
                        if c2 == '-' as i32 as u32 &&
                                        unsafe { *z_pattern_1.offset(0 as isize) } as i32 !=
                                            ']' as i32 &&
                                    unsafe { *z_pattern_1.offset(0 as isize) } as i32 != 0 &&
                                prior_c > 0 as u32 {
                            c2 = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                            if c >= prior_c && c <= c2 { seen = 1; }
                            prior_c = 0 as u32;
                        } else { if c == c2 { seen = 1; } prior_c = c2; }
                        c2 = unsafe { sqlite3_utf8_read(&mut z_pattern_1) };
                    }
                    if c2 == 0 as u32 || seen ^ invert == 0 { return 1; }
                    continue;
                }
            }
            c2 =
                if (unsafe { *z_string_1.offset(0 as isize) } as i32) < 128 {
                    (unsafe {
                            *{
                                    let __p = &mut z_string_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        }) as u32
                } else { unsafe { sqlite3_utf8_read(&mut z_string_1) } };
            if c == c2 { continue; }
            if no_case != 0 &&
                            unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).add(c as u8 as usize)
                                    } as i32 ==
                                unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).add(c2 as u8 as usize)
                                    } as i32 && c < 128 as u32 && c2 < 128 as u32 {
                continue;
            }
            if c == match_one && z_pattern_1 != z_escaped && c2 != 0 as u32 {
                continue;
            }
            return 1;
        }
        return if unsafe { *z_string_1 } as i32 == 0 { 0 } else { 1 };
    }
}

static glob_info: CompareInfo =
    CompareInfo {
        match_all: '*' as i32 as u8,
        match_one: '?' as i32 as u8,
        match_set: '[' as i32 as u8,
        no_case: 0 as u8,
    };

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_strglob(z_glob_pattern: *const i8,
    z_string: *const i8) -> i32 {
    if z_string == core::ptr::null() {
        return (z_glob_pattern != core::ptr::null()) as i32;
    } else if z_glob_pattern == core::ptr::null() {
        return 1;
    } else {
        return pattern_compare(z_glob_pattern as *mut u8 as *const u8,
                z_string as *mut u8 as *const u8, &glob_info,
                '[' as i32 as u32);
    }
}

static like_info_norm: CompareInfo =
    CompareInfo {
        match_all: '%' as i32 as u8,
        match_one: '_' as i32 as u8,
        match_set: 0 as u8,
        no_case: 1 as u8,
    };

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_strlike(z_pattern: *const i8, z_str: *const i8,
    esc: u32) -> i32 {
    if z_str == core::ptr::null() {
        return (z_pattern != core::ptr::null()) as i32;
    } else if z_pattern == core::ptr::null() {
        return 1;
    } else {
        return pattern_compare(z_pattern as *mut u8 as *const u8,
                z_str as *mut u8 as *const u8, &like_info_norm, esc);
    }
}

static hexdigits: [i8; 16] =
    ['0' as i32 as i8, '1' as i32 as i8, '2' as i32 as i8, '3' as i32 as i8,
            '4' as i32 as i8, '5' as i32 as i8, '6' as i32 as i8,
            '7' as i32 as i8, '8' as i32 as i8, '9' as i32 as i8,
            'A' as i32 as i8, 'B' as i32 as i8, 'C' as i32 as i8,
            'D' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8];

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quote_value(p_str_1: *mut StrAccum,
    p_value_1: *mut Sqlite3Value, b_escape_1: i32) -> () {
    { let _ = 0; };
    '__s7:
        {
        match unsafe { sqlite3_value_type(p_value_1) } {
            2 => {
                {
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            c"%!0.17g".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_double(p_value_1) })
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_int64(p_value_1) })
                    };
                    break '__s7;
                }
                {
                    let z_blob: *const i8 =
                        unsafe { sqlite3_value_blob(p_value_1) } as *const i8;
                    let n_blob: i64 =
                        unsafe { sqlite3_value_bytes(p_value_1) } as i64;
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_accum_enlarge(p_str_1,
                            n_blob * 2 as i64 + 4 as i64)
                    };
                    if unsafe { (*p_str_1).acc_error } as i32 == 0 {
                        let z_text: *mut i8 = unsafe { (*p_str_1).z_text };
                        let mut i: i32 = 0;
                        {
                            i = 0;
                            '__b8: loop {
                                if !((i as i64) < n_blob) { break '__b8; }
                                '__c8: loop {
                                    unsafe {
                                        *z_text.offset((i * 2 + 2) as isize) =
                                            hexdigits[(unsafe { *z_blob.offset(i as isize) } as i32 >> 4
                                                            & 15) as usize] as i8
                                    };
                                    unsafe {
                                        *z_text.offset((i * 2 + 3) as isize) =
                                            hexdigits[(unsafe { *z_blob.offset(i as isize) } as i32 &
                                                            15) as usize] as i8
                                    };
                                    break '__c8;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            *z_text.offset((n_blob * 2 as i64 + 2 as i64) as isize) =
                                '\'' as i32 as i8
                        };
                        unsafe {
                            *z_text.offset((n_blob * 2 as i64 + 3 as i64) as isize) =
                                '\u{0}' as i32 as i8
                        };
                        unsafe { *z_text.offset(0 as isize) = 'X' as i32 as i8 };
                        unsafe { *z_text.offset(1 as isize) = '\'' as i32 as i8 };
                        unsafe {
                            (*p_str_1).n_char = (n_blob * 2 as i64 + 3 as i64) as u32
                        };
                    }
                    break '__s7;
                }
                {
                    let z_arg: *const u8 =
                        unsafe { sqlite3_value_text(p_value_1) };
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            if b_escape_1 != 0 {
                                    c"%#Q".as_ptr() as *mut i8
                                } else { c"%Q".as_ptr() as *mut i8 } as *const i8, z_arg)
                    };
                    break '__s7;
                }
                {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                    };
                    break '__s7;
                }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_int64(p_value_1) })
                    };
                    break '__s7;
                }
                {
                    let z_blob: *const i8 =
                        unsafe { sqlite3_value_blob(p_value_1) } as *const i8;
                    let n_blob: i64 =
                        unsafe { sqlite3_value_bytes(p_value_1) } as i64;
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_accum_enlarge(p_str_1,
                            n_blob * 2 as i64 + 4 as i64)
                    };
                    if unsafe { (*p_str_1).acc_error } as i32 == 0 {
                        let z_text: *mut i8 = unsafe { (*p_str_1).z_text };
                        let mut i: i32 = 0;
                        {
                            i = 0;
                            '__b8: loop {
                                if !((i as i64) < n_blob) { break '__b8; }
                                '__c8: loop {
                                    unsafe {
                                        *z_text.offset((i * 2 + 2) as isize) =
                                            hexdigits[(unsafe { *z_blob.offset(i as isize) } as i32 >> 4
                                                            & 15) as usize] as i8
                                    };
                                    unsafe {
                                        *z_text.offset((i * 2 + 3) as isize) =
                                            hexdigits[(unsafe { *z_blob.offset(i as isize) } as i32 &
                                                            15) as usize] as i8
                                    };
                                    break '__c8;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            *z_text.offset((n_blob * 2 as i64 + 2 as i64) as isize) =
                                '\'' as i32 as i8
                        };
                        unsafe {
                            *z_text.offset((n_blob * 2 as i64 + 3 as i64) as isize) =
                                '\u{0}' as i32 as i8
                        };
                        unsafe { *z_text.offset(0 as isize) = 'X' as i32 as i8 };
                        unsafe { *z_text.offset(1 as isize) = '\'' as i32 as i8 };
                        unsafe {
                            (*p_str_1).n_char = (n_blob * 2 as i64 + 3 as i64) as u32
                        };
                    }
                    break '__s7;
                }
                {
                    let z_arg: *const u8 =
                        unsafe { sqlite3_value_text(p_value_1) };
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            if b_escape_1 != 0 {
                                    c"%#Q".as_ptr() as *mut i8
                                } else { c"%Q".as_ptr() as *mut i8 } as *const i8, z_arg)
                    };
                    break '__s7;
                }
                {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                    };
                    break '__s7;
                }
            }
            4 => {
                {
                    let z_blob: *const i8 =
                        unsafe { sqlite3_value_blob(p_value_1) } as *const i8;
                    let n_blob: i64 =
                        unsafe { sqlite3_value_bytes(p_value_1) } as i64;
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_accum_enlarge(p_str_1,
                            n_blob * 2 as i64 + 4 as i64)
                    };
                    if unsafe { (*p_str_1).acc_error } as i32 == 0 {
                        let z_text: *mut i8 = unsafe { (*p_str_1).z_text };
                        let mut i: i32 = 0;
                        {
                            i = 0;
                            '__b8: loop {
                                if !((i as i64) < n_blob) { break '__b8; }
                                '__c8: loop {
                                    unsafe {
                                        *z_text.offset((i * 2 + 2) as isize) =
                                            hexdigits[(unsafe { *z_blob.offset(i as isize) } as i32 >> 4
                                                            & 15) as usize] as i8
                                    };
                                    unsafe {
                                        *z_text.offset((i * 2 + 3) as isize) =
                                            hexdigits[(unsafe { *z_blob.offset(i as isize) } as i32 &
                                                            15) as usize] as i8
                                    };
                                    break '__c8;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            *z_text.offset((n_blob * 2 as i64 + 2 as i64) as isize) =
                                '\'' as i32 as i8
                        };
                        unsafe {
                            *z_text.offset((n_blob * 2 as i64 + 3 as i64) as isize) =
                                '\u{0}' as i32 as i8
                        };
                        unsafe { *z_text.offset(0 as isize) = 'X' as i32 as i8 };
                        unsafe { *z_text.offset(1 as isize) = '\'' as i32 as i8 };
                        unsafe {
                            (*p_str_1).n_char = (n_blob * 2 as i64 + 3 as i64) as u32
                        };
                    }
                    break '__s7;
                }
                {
                    let z_arg: *const u8 =
                        unsafe { sqlite3_value_text(p_value_1) };
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            if b_escape_1 != 0 {
                                    c"%#Q".as_ptr() as *mut i8
                                } else { c"%Q".as_ptr() as *mut i8 } as *const i8, z_arg)
                    };
                    break '__s7;
                }
                {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                    };
                    break '__s7;
                }
            }
            3 => {
                {
                    let z_arg: *const u8 =
                        unsafe { sqlite3_value_text(p_value_1) };
                    unsafe {
                        sqlite3_str_appendf(p_str_1 as *mut Sqlite3Str,
                            if b_escape_1 != 0 {
                                    c"%#Q".as_ptr() as *mut i8
                                } else { c"%Q".as_ptr() as *mut i8 } as *const i8, z_arg)
                    };
                    break '__s7;
                }
                {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                    };
                    break '__s7;
                }
            }
            _ => {
                {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_str_append(p_str_1 as *mut Sqlite3Str,
                            c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                    };
                    break '__s7;
                }
            }
        }
    }
}

extern "C" fn version_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    unsafe {
        sqlite3_result_text(context, unsafe { sqlite3_libversion() }, -1,
            None)
    };
}

extern "C" fn load_ext(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let z_file: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    let mut z_proc: *const i8 = core::ptr::null();
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut z_err_msg: *mut i8 = core::ptr::null_mut();
    if unsafe { (*db).flags } & 131072 as u64 == 0 as u64 {
        unsafe {
            sqlite3_result_error(context,
                c"not authorized".as_ptr() as *mut i8 as *const i8, -1)
        };
        return;
    }
    if argc == 2 {
        z_proc =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8;
    } else { z_proc = core::ptr::null(); }
    if !(z_file).is_null() &&
            unsafe {
                    sqlite3_load_extension(db, z_file, z_proc, &mut z_err_msg)
                } != 0 {
        unsafe { sqlite3_result_error(context, z_err_msg as *const i8, -1) };
        unsafe { sqlite3_free(z_err_msg as *mut ()) };
    }
}

extern "C" fn compileoptionused_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_opt_name: *const i8 = core::ptr::null();
    { let _ = 0; };
    { let _ = argc; };
    if {
                z_opt_name =
                    unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        } as *const i8;
                z_opt_name
            } != core::ptr::null() {
        unsafe {
            sqlite3_result_int(context,
                unsafe { sqlite3_compileoption_used(z_opt_name) })
        };
    }
}

extern "C" fn compileoptionget_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut n: i32 = 0;
    { let _ = 0; };
    { let _ = argc; };
    n = unsafe { sqlite3_value_int(unsafe { *argv.offset(0 as isize) }) };
    unsafe {
        sqlite3_result_text(context, unsafe { sqlite3_compileoption_get(n) },
            -1, None)
    };
}

extern "C" fn context_malloc(context: *mut Sqlite3Context, n_byte_1: i64)
    -> *mut () {
    let mut z: *mut i8 = core::ptr::null_mut();
    let db: *const Sqlite3 =
        unsafe { sqlite3_context_db_handle(context) } as *const Sqlite3;
    { let _ = 0; };
    if n_byte_1 > unsafe { (*db).a_limit[0 as usize] } as i64 {
        unsafe { sqlite3_result_error_toobig(context) };
        z = core::ptr::null_mut();
    } else {
        z = unsafe { sqlite3Malloc(n_byte_1 as u64) } as *mut i8;
        if (z).is_null() as i32 != 0 {
            unsafe { sqlite3_result_error_nomem(context) };
        }
    }
    return z as *mut ();
}

extern "C" fn trim_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut z_in: *const u8 = core::ptr::null();
        let mut z_char_set: *const u8 = core::ptr::null();
        let mut n_in: u32 = 0 as u32;
        let mut flags: i32 = 0;
        let mut i: i32 = 0;
        let mut a_len: *mut u32 = core::ptr::null_mut();
        let mut az_char: *mut *mut u8 = core::ptr::null_mut();
        let mut n_char: i32 = 0;
        if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) }
                == 5 {
            return;
        }
        z_in =
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
            };
        if z_in == core::ptr::null() { return; }
        n_in =
            unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                } as u32;
        { let _ = 0; };
        if argc == 1 {
            n_char = 1;
            a_len = &raw const len_one[0 as usize] as *mut u32;
            az_char = &raw const az_one[0 as usize] as *mut *mut u8;
            z_char_set = core::ptr::null();
        } else if {
                    z_char_set =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                        };
                    z_char_set
                } == core::ptr::null() {
            return;
        } else {
            let mut z: *const u8 = core::ptr::null();
            {
                { z = z_char_set; n_char = 0 };
                '__b9: loop {
                    if !(unsafe { *z } != 0) { break '__b9; }
                    '__c9: loop {
                        {
                            if unsafe {
                                            *{
                                                    let __p = &mut z;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }
                                        } as i32 >= 192 {
                                while unsafe { *z } as i32 & 192 == 128 {
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                            }
                        }
                        break '__c9;
                    }
                    { let __p = &mut n_char; let __t = *__p; *__p += 1; __t };
                }
            }
            if n_char > 0 {
                az_char =
                    context_malloc(context,
                            (n_char as i64 as u64 *
                                    (core::mem::size_of::<*mut i8>() as u64 +
                                            core::mem::size_of::<u32>() as u64) as u64) as i64) as
                        *mut *mut u8;
                if az_char == core::ptr::null_mut() { return; }
                a_len =
                    unsafe { &raw mut *az_char.offset(n_char as isize) } as
                        *mut u32;
                {
                    { z = z_char_set; n_char = 0 };
                    '__b11: loop {
                        if !(unsafe { *z } != 0) { break '__b11; }
                        '__c11: loop {
                            unsafe { *az_char.offset(n_char as isize) = z as *mut u8 };
                            {
                                if unsafe {
                                                *{
                                                        let __p = &mut z;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32 >= 192 {
                                    while unsafe { *z } as i32 & 192 == 128 {
                                        {
                                            let __p = &mut z;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                }
                            }
                            unsafe {
                                *a_len.offset(n_char as isize) =
                                    unsafe {
                                                z.offset_from(unsafe { *az_char.offset(n_char as isize) })
                                            } as i64 as u32
                            };
                            break '__c11;
                        }
                        { let __p = &mut n_char; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }
        if n_char > 0 {
            flags = unsafe { sqlite3_user_data(context) } as i64 as i32;
            if flags & 1 != 0 {
                while n_in > 0 as u32 {
                    let mut len: u32 = 0 as u32;
                    {
                        i = 0;
                        '__b14: loop {
                            if !(i < n_char) { break '__b14; }
                            '__c14: loop {
                                len = unsafe { *a_len.offset(i as isize) };
                                if len <= n_in &&
                                        unsafe {
                                                memcmp(z_in as *const (),
                                                    unsafe { *az_char.offset(i as isize) } as *const (),
                                                    len as u64)
                                            } == 0 {
                                    break '__b14;
                                }
                                break '__c14;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if i >= n_char { break; }
                    {
                        let __n = len;
                        let __p = &mut z_in;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    n_in -= len;
                }
            }
            if flags & 2 != 0 {
                while n_in > 0 as u32 {
                    let mut len: u32 = 0 as u32;
                    {
                        i = 0;
                        '__b16: loop {
                            if !(i < n_char) { break '__b16; }
                            '__c16: loop {
                                len = unsafe { *a_len.offset(i as isize) };
                                if len <= n_in &&
                                        unsafe {
                                                memcmp(unsafe {
                                                            &raw const *z_in.add((n_in - len) as usize)
                                                        } as *const (),
                                                    unsafe { *az_char.offset(i as isize) } as *const (),
                                                    len as u64)
                                            } == 0 {
                                    break '__b16;
                                }
                                break '__c16;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if i >= n_char { break; }
                    n_in -= len;
                }
            }
            if !(z_char_set).is_null() {
                unsafe { sqlite3_free(az_char as *mut ()) };
            }
        }
        unsafe {
            sqlite3_result_text(context, z_in as *mut i8 as *const i8,
                n_in as i32,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
}

extern "C" fn sqlite3_get_func_coll_seq(context: &Sqlite3Context)
    -> *mut CollSeq {
    unsafe {
        let mut p_op: *const VdbeOp = core::ptr::null();
        { let _ = 0; };
        p_op =
            unsafe {
                    unsafe {
                        (*(*context).p_vdbe).a_op.offset(((*context).i_op - 1) as
                                isize)
                    }
                } as *mut VdbeOp;
        { let _ = 0; };
        { let _ = 0; };
        return unsafe { (*p_op).p4.p_coll };
    }
}

extern "C" fn minmax_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut i: i32 = 0;
    let mut mask: i32 = 0;
    let mut i_best: i32 = 0;
    let mut p_coll: *const CollSeq = core::ptr::null();
    { let _ = 0; };
    mask =
        if unsafe { sqlite3_user_data(context) } == core::ptr::null_mut() {
            0
        } else { -1 };
    p_coll = sqlite3_get_func_coll_seq(unsafe { &*context });
    { let _ = 0; };
    { let _ = 0; };
    i_best = 0;
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    {
        i = 1;
        '__b17: loop {
            if !(i < argc) { break '__b17; }
            '__c17: loop {
                if unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                        } == 5 {
                    return;
                }
                if unsafe {
                                sqlite3_mem_compare(unsafe { *argv.offset(i_best as isize) }
                                        as *const Mem,
                                    unsafe { *argv.offset(i as isize) } as *const Mem,
                                    p_coll as *const CollSeq)
                            } ^ mask >= 0 {
                    i_best = i;
                }
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        sqlite3_result_value(context,
            unsafe { *argv.offset(i_best as isize) })
    };
}

extern "C" fn sqlite3_skip_accumulator_load(context: &mut Sqlite3Context)
    -> () {
    { let _ = 0; };
    (*context).is_error = -1;
    (*context).skip_flag = 1 as u8;
}

extern "C" fn minmax_step(context: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let p_arg: *mut Mem = unsafe { *argv.offset(0 as isize) } as *mut Mem;
    let mut p_best: *mut Mem = core::ptr::null_mut();
    { let _ = not_used_1; };
    p_best =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<Mem>() as i32)
            } as *mut Mem;
    if (p_best).is_null() as i32 != 0 { return; }
    if unsafe { sqlite3_value_type(p_arg as *mut Sqlite3Value) } == 5 {
        if unsafe { (*p_best).flags } != 0 {
            sqlite3_skip_accumulator_load(unsafe { &mut *context });
        }
    } else if unsafe { (*p_best).flags } != 0 {
        let mut max: i32 = 0;
        let mut cmp: i32 = 0;
        let p_coll: *const CollSeq =
            sqlite3_get_func_coll_seq(unsafe { &*context }) as *const CollSeq;
        max =
            (unsafe { sqlite3_user_data(context) } != core::ptr::null_mut())
                as i32;
        cmp =
            unsafe {
                sqlite3_mem_compare(p_best as *const Mem, p_arg as *const Mem,
                    p_coll as *const CollSeq)
            };
        if max != 0 && cmp < 0 || (max == 0) as i32 != 0 && cmp > 0 {
            unsafe { sqlite3_vdbe_mem_copy(p_best, p_arg as *const Mem) };
        } else { sqlite3_skip_accumulator_load(unsafe { &mut *context }); }
    } else {
        unsafe {
            (*p_best).db = unsafe { sqlite3_context_db_handle(context) }
        };
        unsafe { sqlite3_vdbe_mem_copy(p_best, p_arg as *const Mem) };
    }
}

extern "C" fn min_max_value_finalize(context: *mut Sqlite3Context,
    b_value_1: i32) -> () {
    let mut p_res: *mut Sqlite3Value = core::ptr::null_mut();
    p_res =
        unsafe { sqlite3_aggregate_context(context, 0) } as *mut Sqlite3Value;
    if !(p_res).is_null() {
        if unsafe { (*p_res).flags } != 0 {
            unsafe { sqlite3_result_value(context, p_res) };
        }
        if b_value_1 == 0 {
            unsafe { sqlite3_vdbe_mem_release(p_res as *mut Mem) };
        }
    }
}

extern "C" fn min_max_finalize(context: *mut Sqlite3Context) -> () {
    min_max_value_finalize(context, 0);
}

extern "C" fn min_max_value(context: *mut Sqlite3Context) -> () {
    min_max_value_finalize(context, 1);
}

extern "C" fn typeof_func(context: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let i: i32 =
            unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) }
                - 1;
        { let _ = not_used_1; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_result_text(context, az_type[i as usize], -1, None)
        };
    }
}

extern "C" fn subtype_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = argc; };
    unsafe {
        sqlite3_result_int(context,
            unsafe {
                    sqlite3_value_subtype(unsafe { *argv.offset(0 as isize) })
                } as i32)
    };
}

extern "C" fn length_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = 0; };
    { let _ = argc; };
    '__s18:
        {
        match unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } {
            4 => {
                {
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s18;
                }
                {
                    let mut z: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    let mut z0: *const u8 = core::ptr::null();
                    if z == core::ptr::null() { return; }
                    z0 = z;
                    loop {
                        if ((unsafe { *z.offset(0 as isize) } as i32 - 1) as u8 as
                                        i32) < 128 - 1 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        } else if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                            break;
                        } else {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            while unsafe { *z.offset(0 as isize) } as i32 & 192 == 128 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                {
                                    let __p = &mut z0;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                        }
                    }
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe { z.offset_from(z0) } as i64 as i32)
                    };
                    break '__s18;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s18; }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s18;
                }
                {
                    let mut z: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    let mut z0: *const u8 = core::ptr::null();
                    if z == core::ptr::null() { return; }
                    z0 = z;
                    loop {
                        if ((unsafe { *z.offset(0 as isize) } as i32 - 1) as u8 as
                                        i32) < 128 - 1 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        } else if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                            break;
                        } else {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            while unsafe { *z.offset(0 as isize) } as i32 & 192 == 128 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                {
                                    let __p = &mut z0;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                        }
                    }
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe { z.offset_from(z0) } as i64 as i32)
                    };
                    break '__s18;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s18; }
            }
            2 => {
                {
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s18;
                }
                {
                    let mut z: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    let mut z0: *const u8 = core::ptr::null();
                    if z == core::ptr::null() { return; }
                    z0 = z;
                    loop {
                        if ((unsafe { *z.offset(0 as isize) } as i32 - 1) as u8 as
                                        i32) < 128 - 1 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        } else if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                            break;
                        } else {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            while unsafe { *z.offset(0 as isize) } as i32 & 192 == 128 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                {
                                    let __p = &mut z0;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                        }
                    }
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe { z.offset_from(z0) } as i64 as i32)
                    };
                    break '__s18;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s18; }
            }
            3 => {
                {
                    let mut z: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    let mut z0: *const u8 = core::ptr::null();
                    if z == core::ptr::null() { return; }
                    z0 = z;
                    loop {
                        if ((unsafe { *z.offset(0 as isize) } as i32 - 1) as u8 as
                                        i32) < 128 - 1 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        } else if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                            break;
                        } else {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            while unsafe { *z.offset(0 as isize) } as i32 & 192 == 128 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                {
                                    let __p = &mut z0;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                        }
                    }
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe { z.offset_from(z0) } as i64 as i32)
                    };
                    break '__s18;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s18; }
            }
            _ => {
                { unsafe { sqlite3_result_null(context) }; break '__s18; }
            }
        }
    }
}

extern "C" fn bytelength_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = 0; };
    { let _ = argc; };
    '__s21:
        {
        match unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } {
            4 => {
                {
                    unsafe {
                        sqlite3_result_int(context,
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s21;
                }
                {
                    let m: i64 =
                        if unsafe {
                                            (*unsafe { sqlite3_context_db_handle(context) }).enc
                                        } as i32 <= 1 {
                                1
                            } else { 2 } as i64;
                    unsafe {
                        sqlite3_result_int64(context,
                            unsafe {
                                        sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                    } as i64 * m)
                    };
                    break '__s21;
                }
                {
                    if unsafe {
                                sqlite3_value_encoding(unsafe { *argv.offset(0 as isize) })
                            } <= 1 {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    } else {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes16(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    }
                    break '__s21;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s21; }
            }
            1 => {
                {
                    let m: i64 =
                        if unsafe {
                                            (*unsafe { sqlite3_context_db_handle(context) }).enc
                                        } as i32 <= 1 {
                                1
                            } else { 2 } as i64;
                    unsafe {
                        sqlite3_result_int64(context,
                            unsafe {
                                        sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                    } as i64 * m)
                    };
                    break '__s21;
                }
                {
                    if unsafe {
                                sqlite3_value_encoding(unsafe { *argv.offset(0 as isize) })
                            } <= 1 {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    } else {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes16(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    }
                    break '__s21;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s21; }
            }
            2 => {
                {
                    let m: i64 =
                        if unsafe {
                                            (*unsafe { sqlite3_context_db_handle(context) }).enc
                                        } as i32 <= 1 {
                                1
                            } else { 2 } as i64;
                    unsafe {
                        sqlite3_result_int64(context,
                            unsafe {
                                        sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                    } as i64 * m)
                    };
                    break '__s21;
                }
                {
                    if unsafe {
                                sqlite3_value_encoding(unsafe { *argv.offset(0 as isize) })
                            } <= 1 {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    } else {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes16(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    }
                    break '__s21;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s21; }
            }
            3 => {
                {
                    if unsafe {
                                sqlite3_value_encoding(unsafe { *argv.offset(0 as isize) })
                            } <= 1 {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    } else {
                        unsafe {
                            sqlite3_result_int(context,
                                unsafe {
                                    sqlite3_value_bytes16(unsafe { *argv.offset(0 as isize) })
                                })
                        };
                    }
                    break '__s21;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s21; }
            }
            _ => {
                { unsafe { sqlite3_result_null(context) }; break '__s21; }
            }
        }
    }
}

extern "C" fn instr_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_haystack: *const u8 = core::ptr::null();
    let mut z_needle: *const u8 = core::ptr::null();
    let mut n_haystack: i32 = 0;
    let mut n_needle: i32 = 0;
    let mut type_haystack: i32 = 0;
    let mut type_needle: i32 = 0;
    let mut n: i32 = 0;
    let mut is_text: i32 = 0;
    let mut first_char: u8 = 0 as u8;
    let mut p_c1: *mut Sqlite3Value = core::ptr::null_mut();
    let mut p_c2: *mut Sqlite3Value = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s23:
            {
            match __state {
                0 => { __state = 4; }
                2 => { unsafe { sqlite3_value_free(p_c1) }; __state = 52; }
                3 => {
                    unsafe { sqlite3_result_error_nomem(context) };
                    __state = 55;
                }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { n = 1; __state = 9; }
                9 => { __state = 10; }
                10 => { __state = 11; }
                11 => { p_c1 = core::ptr::null_mut(); __state = 12; }
                12 => { p_c2 = core::ptr::null_mut(); __state = 13; }
                13 => { { let _ = argc; }; __state = 14; }
                14 => {
                    type_haystack =
                        unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                        };
                    __state = 15;
                }
                15 => {
                    type_needle =
                        unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(1 as isize) })
                        };
                    __state = 16;
                }
                16 => {
                    if type_haystack == 5 || type_needle == 5 {
                        __state = 18;
                    } else { __state = 17; }
                }
                17 => {
                    n_haystack =
                        unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                        };
                    __state = 19;
                }
                18 => { return; }
                19 => {
                    n_needle =
                        unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                        };
                    __state = 20;
                }
                20 => {
                    if n_needle > 0 { __state = 22; } else { __state = 21; }
                }
                21 => {
                    unsafe { sqlite3_result_int(context, n) };
                    __state = 51;
                }
                22 => {
                    if type_haystack == 4 && type_needle == 4 {
                        __state = 24;
                    } else { __state = 25; }
                }
                23 => {
                    if z_needle == core::ptr::null() ||
                            n_haystack != 0 && z_haystack == core::ptr::null() {
                        __state = 43;
                    } else { __state = 42; }
                }
                24 => {
                    z_haystack =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    __state = 26;
                }
                25 => {
                    if type_haystack != 4 && type_needle != 4 {
                        __state = 28;
                    } else { __state = 29; }
                }
                26 => {
                    z_needle =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(1 as isize) })
                            } as *const u8;
                    __state = 27;
                }
                27 => { is_text = 0; __state = 23; }
                28 => {
                    z_haystack =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    __state = 30;
                }
                29 => {
                    p_c1 =
                        unsafe {
                            sqlite3_value_dup(unsafe { *argv.offset(0 as isize) } as
                                    *const Sqlite3Value)
                        };
                    __state = 32;
                }
                30 => {
                    z_needle =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                        };
                    __state = 31;
                }
                31 => { is_text = 1; __state = 23; }
                32 => {
                    z_haystack = unsafe { sqlite3_value_text(p_c1) };
                    __state = 33;
                }
                33 => {
                    if z_haystack == core::ptr::null() {
                        __state = 35;
                    } else { __state = 34; }
                }
                34 => {
                    n_haystack = unsafe { sqlite3_value_bytes(p_c1) };
                    __state = 36;
                }
                35 => { __state = 3; }
                36 => {
                    p_c2 =
                        unsafe {
                            sqlite3_value_dup(unsafe { *argv.offset(1 as isize) } as
                                    *const Sqlite3Value)
                        };
                    __state = 37;
                }
                37 => {
                    z_needle = unsafe { sqlite3_value_text(p_c2) };
                    __state = 38;
                }
                38 => {
                    if z_needle == core::ptr::null() {
                        __state = 40;
                    } else { __state = 39; }
                }
                39 => {
                    n_needle = unsafe { sqlite3_value_bytes(p_c2) };
                    __state = 41;
                }
                40 => { __state = 3; }
                41 => { is_text = 1; __state = 23; }
                42 => {
                    first_char = unsafe { *z_needle.offset(0 as isize) } as u8;
                    __state = 44;
                }
                43 => { __state = 3; }
                44 => {
                    if n_needle <= n_haystack &&
                            (unsafe { *z_haystack.offset(0 as isize) } as i32 !=
                                    first_char as i32 ||
                                unsafe {
                                        memcmp(z_haystack as *const (), z_needle as *const (),
                                            n_needle as u64)
                                    } != 0) {
                        __state = 46;
                    } else { __state = 45; }
                }
                45 => {
                    if n_needle > n_haystack {
                        __state = 50;
                    } else { __state = 21; }
                }
                46 => {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                    __state = 47;
                }
                47 => {
                    {
                        let __p = &mut n_haystack;
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    __state = 49;
                }
                48 => {
                    if is_text != 0 &&
                            unsafe { *z_haystack.offset(0 as isize) } as i32 & 192 ==
                                128 {
                        __state = 47;
                    } else { __state = 44; }
                }
                49 => {
                    {
                        let __p = &mut z_haystack;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 48;
                }
                50 => { n = 0; __state = 21; }
                51 => { __state = 2; }
                52 => { unsafe { sqlite3_value_free(p_c2) }; __state = 53; }
                53 => { return; }
                54 => { __state = 3; }
                55 => { __state = 2; }
                _ => {}
            }
        }
    }
}

extern "C" fn printf_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut x: PrintfArguments = unsafe { core::mem::zeroed() };
    let mut str: StrAccum = unsafe { core::mem::zeroed() };
    let mut z_format: *const i8 = core::ptr::null();
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    if argc >= 1 &&
            {
                    z_format =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                            } as *const i8;
                    z_format
                } != core::ptr::null() {
        x.n_arg = argc - 1;
        x.n_used = 0;
        x.ap_arg = unsafe { argv.offset(1 as isize) };
        unsafe {
            sqlite3_str_accum_init(&mut str, db, core::ptr::null_mut(), 0,
                unsafe { (*db).a_limit[0 as usize] })
        };
        str.printf_flags = 2 as u8;
        unsafe {
            sqlite3_str_appendf(&raw mut str as *mut Sqlite3Str, z_format,
                &raw mut x as *mut PrintfArguments)
        };
        unsafe {
            sqlite3_result_str(context, &raw mut str as *mut Sqlite3Str, 1)
        };
    }
}

extern "C" fn unicode_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    { let _ = argc; };
    if !(z).is_null() && unsafe { *z.offset(0 as isize) } != 0 {
        unsafe {
            sqlite3_result_int(context,
                unsafe { sqlite3_utf8_read(&mut z) } as i32)
        };
    }
}

extern "C" fn char_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z: *mut u8 = core::ptr::null_mut();
    let mut z_out: *mut u8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    z_out =
        {
            z =
                unsafe { sqlite3_malloc64((argc * 4 + 1) as Sqlite3Uint64) }
                    as *mut u8;
            z
        };
    if z == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
        return;
    }
    {
        i = 0;
        '__b24: loop {
            if !(i < argc) { break '__b24; }
            '__c24: loop {
                let mut x: Sqlite3Int64 = 0 as Sqlite3Int64;
                let mut c: u32 = 0 as u32;
                x =
                    unsafe {
                        sqlite3_value_int64(unsafe { *argv.offset(i as isize) })
                    };
                if x < 0 as i64 || x > 1114111 as i64 {
                    x = 65533 as Sqlite3Int64;
                }
                c = (x & 2097151 as Sqlite3Int64) as u32;
                if c < 128 as u32 {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (c & 255 as u32) as u8
                    };
                } else if c < 2048 as u32 {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (192 + (c >> 6 & 31 as u32) as u8 as i32) as u8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (c & 63 as u32) as u8 as i32) as u8
                    };
                } else if c < 65536 as u32 {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (224 + (c >> 12 & 15 as u32) as u8 as i32) as u8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (c >> 6 & 63 as u32) as u8 as i32) as u8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (c & 63 as u32) as u8 as i32) as u8
                    };
                } else {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (240 + (c >> 18 & 7 as u32) as u8 as i32) as u8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (c >> 12 & 63 as u32) as u8 as i32) as u8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (c >> 6 & 63 as u32) as u8 as i32) as u8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (c & 63 as u32) as u8 as i32) as u8
                    };
                }
                break '__c24;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z_out = 0 as u8 };
    unsafe {
        sqlite3_result_text64(context, z as *mut i8 as *const i8,
            unsafe { z_out.offset_from(z) } as i64 as Sqlite3Uint64,
            Some(sqlite3_free), 16 as u8)
    };
}

extern "C" fn abs_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = 0; };
    { let _ = argc; };
    '__s25:
        {
        match unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } {
            1 => {
                {
                    let mut i_val: i64 =
                        unsafe {
                            sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                        };
                    if i_val < 0 as i64 {
                        if i_val ==
                                -1 as i64 -
                                    (4294967295u32 as i64 | (2147483647 as i64) << 32) {
                            unsafe {
                                sqlite3_result_error(context,
                                    c"integer overflow".as_ptr() as *mut i8 as *const i8, -1)
                            };
                            return;
                        }
                        i_val = -i_val;
                    }
                    unsafe { sqlite3_result_int64(context, i_val) };
                    break '__s25;
                }
                { unsafe { sqlite3_result_null(context) }; break '__s25; }
                {
                    let mut r_val: f64 =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        };
                    if r_val < 0 as f64 { r_val = -r_val; }
                    unsafe { sqlite3_result_double(context, r_val) };
                    break '__s25;
                }
            }
            5 => {
                { unsafe { sqlite3_result_null(context) }; break '__s25; }
                {
                    let mut r_val: f64 =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        };
                    if r_val < 0 as f64 { r_val = -r_val; }
                    unsafe { sqlite3_result_double(context, r_val) };
                    break '__s25;
                }
            }
            _ => {
                {
                    let mut r_val: f64 =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        };
                    if r_val < 0 as f64 { r_val = -r_val; }
                    unsafe { sqlite3_result_double(context, r_val) };
                    break '__s25;
                }
            }
        }
    }
}

extern "C" fn round_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut n: i64 = 0 as i64;
    let mut r: f64 = 0.0;
    let mut z_buf: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    if argc == 2 {
        if 5 ==
                unsafe {
                    sqlite3_value_type(unsafe { *argv.offset(1 as isize) })
                } {
            return;
        }
        n =
            unsafe {
                sqlite3_value_int64(unsafe { *argv.offset(1 as isize) })
            };
        if n > 30 as i64 { n = 30 as i64; }
        if n < 0 as i64 { n = 0 as i64; }
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    r = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    if r < -4503599627370496.0 || r > 4503599627370496.0
        {} else if n == 0 as i64 {
        r = (r + if r < 0 as f64 { -0.5 } else { 0.5 }) as SqliteInt64 as f64;
    } else {
        z_buf =
            unsafe {
                sqlite3_mprintf(c"%!.*f".as_ptr() as *mut i8 as *const i8,
                    n as i32, r)
            };
        if z_buf == core::ptr::null_mut() {
            unsafe { sqlite3_result_error_nomem(context) };
            return;
        }
        unsafe { sqlite3_ato_f(z_buf as *const i8, &mut r) };
        unsafe { sqlite3_free(z_buf as *mut ()) };
    }
    unsafe { sqlite3_result_double(context, r) };
}

extern "C" fn upper_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut z1: *mut i8 = core::ptr::null_mut();
        let mut z2: *const i8 = core::ptr::null();
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        { let _ = argc; };
        z2 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                    as *mut i8 as *const i8;
        n =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        { let _ = 0; };
        if !(z2).is_null() {
            z1 = context_malloc(context, n as i64 + 1 as i64) as *mut i8;
            if !(z1).is_null() {
                {
                    i = 0;
                    '__b26: loop {
                        if !(i < n) { break '__b26; }
                        '__c26: loop {
                            unsafe {
                                *z1.offset(i as isize) =
                                    (unsafe { *z2.offset(i as isize) } as i32 &
                                            !(unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2.offset(i as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 32)) as i8
                            };
                            break '__c26;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    sqlite3_result_text(context, z1 as *const i8, n,
                        Some(sqlite3_free))
                };
            }
        }
    }
}

extern "C" fn lower_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut z1: *mut i8 = core::ptr::null_mut();
        let mut z2: *const i8 = core::ptr::null();
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        { let _ = argc; };
        z2 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                    as *mut i8 as *const i8;
        n =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        { let _ = 0; };
        if !(z2).is_null() {
            z1 = context_malloc(context, n as i64 + 1 as i64) as *mut i8;
            if !(z1).is_null() {
                {
                    i = 0;
                    '__b27: loop {
                        if !(i < n) { break '__b27; }
                        '__c27: loop {
                            unsafe {
                                *z1.offset(i as isize) =
                                    unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z2.offset(i as isize) } as u8 as
                                                        usize)
                                        } as i8
                            };
                            break '__c27;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    sqlite3_result_text(context, z1 as *const i8, n,
                        Some(sqlite3_free))
                };
            }
        }
    }
}

extern "C" fn hex_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut p_blob: *const u8 = core::ptr::null();
    let mut z_hex: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = argc; };
    p_blob =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const u8;
    n = unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    { let _ = 0; };
    z =
        {
            z_hex =
                context_malloc(context, n as i64 * 2 as i64 + 1 as i64) as
                    *mut i8;
            z_hex
        };
    if !(z_hex).is_null() {
        {
            i = 0;
            '__b28: loop {
                if !(i < n) { break '__b28; }
                '__c28: loop {
                    let c: u8 = unsafe { *p_blob } as u8;
                    unsafe {
                        *{
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = hexdigits[(c as i32 >> 4 & 15) as usize] as i8
                    };
                    unsafe {
                        *{
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = hexdigits[(c as i32 & 15) as usize] as i8
                    };
                    break '__c28;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_blob;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        unsafe { *z = 0 as i8 };
        unsafe {
            sqlite3_result_text64(context, z_hex as *const i8,
                unsafe { z.offset_from(z_hex) } as i64 as u64,
                Some(sqlite3_free), 16 as u8)
        };
    }
}

extern "C" fn str_contains_char(z_str_1: *const u8, n_str_1: i32, ch: u32)
    -> i32 {
    let z_end: *const u8 = unsafe { &*z_str_1.offset(n_str_1 as isize) };
    let mut z: *const u8 = z_str_1;
    while z < z_end {
        let tst: u32 =
            if (unsafe { *z.offset(0 as isize) } as i32) < 128 {
                (unsafe {
                        *{
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    }) as u32
            } else { unsafe { sqlite3_utf8_read(&mut z) } };
        if tst == ch { return 1; }
    }
    return 0;
}

extern "C" fn unhex_func(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut z_pass: *const u8 = core::ptr::null();
        let mut n_pass: i32 = 0;
        let mut z_hex: *const u8 = core::ptr::null();
        let mut n_hex: i32 = 0;
        let mut p_blob: *mut u8 = core::ptr::null_mut();
        let mut p: *mut u8 = core::ptr::null_mut();
        let mut c: u8 = 0 as u8;
        let mut d: u8 = 0 as u8;
        let mut ch: u32 = 0 as u32;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s31:
                {
                match __state {
                    0 => { z_pass = c"".as_ptr() as *const u8; __state = 4; }
                    2 => {
                        unsafe {
                            sqlite3_result_blob(p_ctx_1, p_blob as *const (),
                                unsafe { p.offset_from(p_blob) } as i64 as i32,
                                Some(sqlite3_free))
                        };
                        __state = 36;
                    }
                    3 => {
                        unsafe { sqlite3_free(p_blob as *mut ()) };
                        __state = 38;
                    }
                    4 => { n_pass = 0; __state = 5; }
                    5 => {
                        z_hex =
                            unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                            };
                        __state = 6;
                    }
                    6 => {
                        n_hex =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        __state = 7;
                    }
                    7 => { p_blob = core::ptr::null_mut(); __state = 8; }
                    8 => { p = core::ptr::null_mut(); __state = 9; }
                    9 => { { let _ = 0; }; __state = 10; }
                    10 => {
                        if argc == 2 { __state = 12; } else { __state = 11; }
                    }
                    11 => {
                        if (z_hex).is_null() as i32 != 0 ||
                                (z_pass).is_null() as i32 != 0 {
                            __state = 15;
                        } else { __state = 14; }
                    }
                    12 => {
                        z_pass =
                            unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                            };
                        __state = 13;
                    }
                    13 => {
                        n_pass =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                            };
                        __state = 11;
                    }
                    14 => {
                        p =
                            {
                                p_blob =
                                    context_malloc(p_ctx_1, (n_hex / 2 + 1) as i64) as *mut u8;
                                p_blob
                            };
                        __state = 16;
                    }
                    15 => { return; }
                    16 => {
                        if !(p_blob).is_null() {
                            __state = 18;
                        } else { __state = 17; }
                    }
                    17 => { __state = 2; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => {
                        if { c = unsafe { *z_hex } as u8; c } as i32 != 0 {
                            __state = 21;
                        } else { __state = 17; }
                    }
                    21 => {
                        if (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(c as u8 as usize)
                                                } as i32 & 8 == 0) as i32 != 0 {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => {
                        {
                            let __p = &mut z_hex;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 30;
                    }
                    23 => {
                        ch =
                            if (unsafe { *z_hex.offset(0 as isize) } as i32) < 128 {
                                (unsafe {
                                        *{
                                                let __p = &mut z_hex;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                    }) as u32
                            } else { unsafe { sqlite3_utf8_read(&mut z_hex) } };
                        __state = 24;
                    }
                    24 => { { let _ = 0; }; __state = 25; }
                    25 => {
                        if (str_contains_char(z_pass, n_pass, ch) == 0) as i32 != 0
                            {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => { c = unsafe { *z_hex } as u8; __state = 28; }
                    27 => { __state = 3; }
                    28 => {
                        if c as i32 == 0 { __state = 29; } else { __state = 21; }
                    }
                    29 => { __state = 2; }
                    30 => { { let _ = 0; }; __state = 31; }
                    31 => { { let _ = 0; }; __state = 32; }
                    32 => {
                        d =
                            unsafe {
                                    *{
                                            let __p = &mut z_hex;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as u8;
                        __state = 33;
                    }
                    33 => {
                        if (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(d as u8 as usize)
                                                } as i32 & 8 == 0) as i32 != 0 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    34 => {
                        unsafe {
                            *{
                                        let __p = &mut p;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } =
                                ((unsafe { sqlite3_hex_to_int(c as i32) } as i32) << 4 |
                                        unsafe { sqlite3_hex_to_int(d as i32) } as i32) as u8
                        };
                        __state = 20;
                    }
                    35 => { __state = 3; }
                    36 => { return; }
                    37 => { __state = 3; }
                    38 => { return; }
                    _ => {}
                }
            }
        }
    }
}

extern "C" fn concat_func_core(context: *mut Sqlite3Context, argc: i32,
    argv: *const *mut Sqlite3Value, n_sep_1: i32, z_sep_1: *const i8) -> () {
    let mut j: i64 = 0 as i64;
    let mut n: i64 = 0 as i64;
    let mut i: i32 = 0;
    let mut b_not_null: i32 = 0;
    let mut z: *mut i8 = core::ptr::null_mut();
    {
        i = 0;
        '__b32: loop {
            if !(i < argc) { break '__b32; }
            '__c32: loop {
                n +=
                    unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(i as isize) })
                        } as i64;
                break '__c32;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    n += (argc - 1) as i64 * n_sep_1 as i64;
    z =
        unsafe { sqlite3_malloc64((n + 1 as i64) as Sqlite3Uint64) } as
            *mut i8;
    if z == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
        return;
    }
    j = 0 as i64;
    {
        i = 0;
        '__b33: loop {
            if !(i < argc) { break '__b33; }
            '__c33: loop {
                if unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                        } != 5 {
                    let k: i32 =
                        unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(i as isize) })
                        };
                    let v: *const i8 =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const i8;
                    if v != core::ptr::null() {
                        if b_not_null != 0 && n_sep_1 > 0 {
                            unsafe {
                                memcpy(unsafe { &raw mut *z.offset(j as isize) } as *mut (),
                                    z_sep_1 as *const (), n_sep_1 as u64)
                            };
                            j += n_sep_1 as i64;
                        }
                        unsafe {
                            memcpy(unsafe { &raw mut *z.offset(j as isize) } as *mut (),
                                v as *const (), k as u64)
                        };
                        j += k as i64;
                        b_not_null = 1;
                    }
                }
                break '__c33;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z.offset(j as isize) = 0 as i8 };
    { let _ = 0; };
    unsafe {
        sqlite3_result_text64(context, z as *const i8, j as Sqlite3Uint64,
            Some(sqlite3_free), 16 as u8)
    };
}

extern "C" fn concat_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    concat_func_core(context, argc, argv as *const *mut Sqlite3Value, 0,
        c"".as_ptr() as *mut i8 as *const i8);
}

extern "C" fn concatws_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let n_sep: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    let z_sep: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_sep == core::ptr::null() { return; }
    concat_func_core(context, argc - 1,
        unsafe { argv.offset(1 as isize) } as *const *mut Sqlite3Value, n_sep,
        z_sep);
}

extern "C" fn random_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    let mut r: SqliteInt64 = 0 as SqliteInt64;
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    unsafe {
        sqlite3_randomness(core::mem::size_of::<SqliteInt64>() as i32,
            &raw mut r as *mut ())
    };
    if r < 0 as i64 {
        r = -(r & (4294967295u32 as i64 | (2147483647 as i64) << 32));
    }
    unsafe { sqlite3_result_int64(context, r) };
}

extern "C" fn random_blob(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut n: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut p: *mut u8 = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = argc; };
    n = unsafe { sqlite3_value_int64(unsafe { *argv.offset(0 as isize) }) };
    if n < 1 as i64 { n = 1 as Sqlite3Int64; }
    p = context_malloc(context, n) as *mut u8;
    if !(p).is_null() {
        unsafe { sqlite3_randomness(n as i32, p as *mut ()) };
        unsafe {
            sqlite3_result_blob(context, p as *mut i8 as *const (), n as i32,
                Some(sqlite3_free))
        };
    }
}

extern "C" fn nullif_func(context: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let p_coll: *const CollSeq =
        sqlite3_get_func_coll_seq(unsafe { &*context }) as *const CollSeq;
    { let _ = not_used_1; };
    if unsafe {
                sqlite3_mem_compare(unsafe { *argv.offset(0 as isize) } as
                        *const Mem,
                    unsafe { *argv.offset(1 as isize) } as *const Mem,
                    p_coll as *const CollSeq)
            } != 0 {
        unsafe {
            sqlite3_result_value(context, unsafe { *argv.offset(0 as isize) })
        };
    }
}

extern "C" fn sourceid_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    unsafe {
        sqlite3_result_text(context, unsafe { sqlite3_sourceid() }, -1, None)
    };
}

extern "C" fn errlog_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = argc; };
    { let _ = context; };
    unsafe {
        sqlite3_log(unsafe {
                sqlite3_value_int(unsafe { *argv.offset(0 as isize) })
            }, c"%s".as_ptr() as *mut i8 as *const i8,
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
            })
    };
}

extern "C" fn is_n_hex(z: *const i8, n_1: i32, p_val_1: &mut u32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut v: u32 = 0 as u32;
        {
            i = 0;
            '__b34: loop {
                if !(i < n_1) { break '__b34; }
                '__c34: loop {
                    if (unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.offset(i as isize) } as u8 as
                                                            usize)
                                            } as i32 & 8 == 0) as i32 != 0 {
                        return 0;
                    }
                    v =
                        (v << 4) +
                            unsafe {
                                    sqlite3_hex_to_int(unsafe { *z.offset(i as isize) } as i32)
                                } as u32;
                    break '__c34;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        *p_val_1 = v;
        return 1;
    }
}

extern "C" fn unistr_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut z_out: *mut i8 = core::ptr::null_mut();
        let mut z_in: *const i8 = core::ptr::null();
        let mut n_in: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut n: i32 = 0;
        let mut v: u32 = 0 as u32;
        let mut z: *const i8 = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s36:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe { sqlite3_free(z_out as *mut ()) };
                        __state = 58;
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { { let _ = 0; }; __state = 8; }
                    8 => { { let _ = argc; }; __state = 9; }
                    9 => {
                        z_in =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                                } as *const i8;
                        __state = 10;
                    }
                    10 => {
                        if z_in == core::ptr::null() {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    11 => {
                        n_in =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        __state = 13;
                    }
                    12 => { return; }
                    13 => {
                        z_out =
                            unsafe { sqlite3_malloc64((n_in + 1) as Sqlite3Uint64) } as
                                *mut i8;
                        __state = 14;
                    }
                    14 => {
                        if z_out == core::ptr::null_mut() {
                            __state = 16;
                        } else { __state = 15; }
                    }
                    15 => { i = { j = 0; j }; __state = 18; }
                    16 => {
                        unsafe { sqlite3_result_error_nomem(context) };
                        __state = 17;
                    }
                    17 => { return; }
                    18 => {
                        if i < n_in { __state = 20; } else { __state = 19; }
                    }
                    19 => {
                        unsafe { *z_out.offset(j as isize) = 0 as i8 };
                        __state = 55;
                    }
                    20 => {
                        z =
                            unsafe {
                                    strchr(unsafe { &*z_in.offset(i as isize) }, '\\' as i32)
                                } as *const i8;
                        __state = 21;
                    }
                    21 => {
                        if z == core::ptr::null() {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => {
                        n =
                            unsafe { z.offset_from(unsafe { z_in.offset(i as isize) }) }
                                    as i64 as i32;
                        __state = 27;
                    }
                    23 => { n = n_in - i; __state = 24; }
                    24 => {
                        unsafe {
                            memmove(unsafe { &raw mut *z_out.offset(j as isize) } as
                                    *mut (),
                                unsafe { &raw const *z_in.offset(i as isize) } as *const (),
                                n as u64)
                        };
                        __state = 25;
                    }
                    25 => { j += n; __state = 26; }
                    26 => { __state = 19; }
                    27 => { if n > 0 { __state = 29; } else { __state = 28; } }
                    28 => {
                        if unsafe { *z_in.offset((i + 1) as isize) } as i32 ==
                                '\\' as i32 {
                            __state = 32;
                        } else { __state = 33; }
                    }
                    29 => {
                        unsafe {
                            memmove(unsafe { &raw mut *z_out.offset(j as isize) } as
                                    *mut (),
                                unsafe { &raw const *z_in.offset(i as isize) } as *const (),
                                n as u64)
                        };
                        __state = 30;
                    }
                    30 => { j += n; __state = 31; }
                    31 => { i += n; __state = 28; }
                    32 => { i += 2; __state = 34; }
                    33 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_in.offset((i + 1) as isize) } as
                                                            u8 as usize)
                                        } as i32 & 8 != 0 {
                            __state = 35;
                        } else { __state = 36; }
                    }
                    34 => {
                        unsafe {
                            *z_out.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '\\' as i32 as i8
                        };
                        __state = 18;
                    }
                    35 => {
                        if (is_n_hex(unsafe { &*z_in.offset((i + 1) as isize) }, 4,
                                            &mut v) == 0) as i32 != 0 {
                            __state = 38;
                        } else { __state = 37; }
                    }
                    36 => {
                        if unsafe { *z_in.offset((i + 1) as isize) } as i32 ==
                                '+' as i32 {
                            __state = 40;
                        } else { __state = 41; }
                    }
                    37 => { i += 5; __state = 39; }
                    38 => { __state = 2; }
                    39 => {
                        j +=
                            unsafe {
                                sqlite3_append_one_utf8_character(unsafe {
                                        &mut *z_out.offset(j as isize)
                                    }, v)
                            };
                        __state = 18;
                    }
                    40 => {
                        if (is_n_hex(unsafe { &*z_in.offset((i + 2) as isize) }, 6,
                                            &mut v) == 0) as i32 != 0 {
                            __state = 43;
                        } else { __state = 42; }
                    }
                    41 => {
                        if unsafe { *z_in.offset((i + 1) as isize) } as i32 ==
                                'u' as i32 {
                            __state = 45;
                        } else { __state = 46; }
                    }
                    42 => { i += 8; __state = 44; }
                    43 => { __state = 2; }
                    44 => {
                        j +=
                            unsafe {
                                sqlite3_append_one_utf8_character(unsafe {
                                        &mut *z_out.offset(j as isize)
                                    }, v)
                            };
                        __state = 18;
                    }
                    45 => {
                        if (is_n_hex(unsafe { &*z_in.offset((i + 2) as isize) }, 4,
                                            &mut v) == 0) as i32 != 0 {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    46 => {
                        if unsafe { *z_in.offset((i + 1) as isize) } as i32 ==
                                'U' as i32 {
                            __state = 50;
                        } else { __state = 51; }
                    }
                    47 => { i += 6; __state = 49; }
                    48 => { __state = 2; }
                    49 => {
                        j +=
                            unsafe {
                                sqlite3_append_one_utf8_character(unsafe {
                                        &mut *z_out.offset(j as isize)
                                    }, v)
                            };
                        __state = 18;
                    }
                    50 => {
                        if (is_n_hex(unsafe { &*z_in.offset((i + 2) as isize) }, 8,
                                            &mut v) == 0) as i32 != 0 {
                            __state = 53;
                        } else { __state = 52; }
                    }
                    51 => { __state = 2; }
                    52 => { i += 10; __state = 54; }
                    53 => { __state = 2; }
                    54 => {
                        j +=
                            unsafe {
                                sqlite3_append_one_utf8_character(unsafe {
                                        &mut *z_out.offset(j as isize)
                                    }, v)
                            };
                        __state = 18;
                    }
                    55 => {
                        unsafe {
                            sqlite3_result_text64(context, z_out as *const i8,
                                j as Sqlite3Uint64, Some(sqlite3_free), 16 as u8)
                        };
                        __state = 56;
                    }
                    56 => { return; }
                    57 => { __state = 2; }
                    58 => {
                        unsafe {
                            sqlite3_result_error(context,
                                c"invalid Unicode escape".as_ptr() as *mut i8 as *const i8,
                                -1)
                        };
                        __state = 59;
                    }
                    59 => { return; }
                    _ => {}
                }
            }
        }
    }
}

extern "C" fn quote_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut str: Sqlite3Str = unsafe { core::mem::zeroed() };
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    { let _ = 0; };
    { let _ = argc; };
    unsafe {
        sqlite3_str_accum_init(&raw mut str as *mut StrAccum, db,
            core::ptr::null_mut(), 0, unsafe { (*db).a_limit[0 as usize] })
    };
    sqlite3_quote_value(&raw mut str as *mut StrAccum,
        unsafe { *argv.offset(0 as isize) },
        unsafe { sqlite3_user_data(context) } as i64 as i32);
    unsafe { sqlite3_result_str(context, &mut str, 1) };
}

extern "C" fn last_insert_rowid(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    unsafe {
        sqlite3_result_int64(context,
            unsafe { sqlite3_last_insert_rowid(db) })
    };
}

extern "C" fn changes(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    unsafe {
        sqlite3_result_int64(context, unsafe { sqlite3_changes64(db) })
    };
}

extern "C" fn total_changes(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    unsafe {
        sqlite3_result_int64(context, unsafe { sqlite3_total_changes64(db) })
    };
}

extern "C" fn replace_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_str: *const u8 = core::ptr::null();
    let mut z_pattern: *const u8 = core::ptr::null();
    let mut z_rep: *const u8 = core::ptr::null();
    let mut z_out: *mut u8 = core::ptr::null_mut();
    let mut n_str: i32 = 0;
    let mut n_pattern: i32 = 0;
    let mut n_rep: i32 = 0;
    let mut n_out: i64 = 0 as i64;
    let mut loop_limit: i32 = 0;
    let mut i: i64 = 0 as i64;
    let mut j: i64 = 0 as i64;
    let mut cnt_expand: u32 = 0 as u32;
    let db: *const Sqlite3 =
        unsafe { sqlite3_context_db_handle(context) } as *const Sqlite3;
    { let _ = 0; };
    { let _ = argc; };
    z_str =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    if z_str == core::ptr::null() { return; }
    n_str =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    { let _ = 0; };
    z_pattern =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) };
    if z_pattern == core::ptr::null() { { let _ = 0; }; return; }
    if unsafe { *z_pattern.offset(0 as isize) } as i32 == 0 {
        { let _ = 0; };
        unsafe {
            sqlite3_result_text(context, z_str as *const i8, n_str,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
        return;
    }
    n_pattern =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) }) };
    { let _ = 0; };
    z_rep =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(2 as isize) }) };
    if z_rep == core::ptr::null() { return; }
    n_rep =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(2 as isize) }) };
    { let _ = 0; };
    n_out = (n_str + 1) as i64;
    { let _ = 0; };
    z_out = context_malloc(context, n_out) as *mut u8;
    if z_out == core::ptr::null_mut() { return; }
    loop_limit = n_str - n_pattern;
    cnt_expand = 0 as u32;
    {
        i = { j = 0 as i64; j };
        '__b37: loop {
            if !(i <= loop_limit as i64) { break '__b37; }
            '__c37: loop {
                if unsafe { *z_str.offset(i as isize) } as i32 !=
                            unsafe { *z_pattern.offset(0 as isize) } as i32 ||
                        unsafe {
                                memcmp(unsafe { &raw const *z_str.offset(i as isize) } as
                                        *const (), z_pattern as *const (), n_pattern as u64)
                            } != 0 {
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = unsafe { *z_str.offset(i as isize) } as u8
                    };
                } else {
                    if n_rep > n_pattern {
                        n_out += (n_rep - n_pattern) as i64;
                        if n_out - 1 as i64 >
                                unsafe { (*db).a_limit[0 as usize] } as i64 {
                            unsafe { sqlite3_result_error_toobig(context) };
                            unsafe { sqlite3_free(z_out as *mut ()) };
                            return;
                        }
                        {
                            let __p = &mut cnt_expand;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        if cnt_expand & cnt_expand - 1 as u32 == 0 as u32 {
                            let mut z_old: *mut u8 = core::ptr::null_mut();
                            z_old = z_out;
                            z_out =
                                unsafe {
                                        sqlite3Realloc(z_out as *mut (),
                                            (n_out as i32 as i64 + (n_out - n_str as i64 - 1 as i64)) as
                                                u64)
                                    } as *mut u8;
                            if z_out == core::ptr::null_mut() {
                                unsafe { sqlite3_result_error_nomem(context) };
                                unsafe { sqlite3_free(z_old as *mut ()) };
                                return;
                            }
                        }
                    }
                    unsafe {
                        memcpy(unsafe { &raw mut *z_out.offset(j as isize) } as
                                *mut (), z_rep as *const (), n_rep as u64)
                    };
                    j += n_rep as i64;
                    i += (n_pattern - 1) as i64;
                }
                break '__c37;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    unsafe {
        memcpy(unsafe { &raw mut *z_out.offset(j as isize) } as *mut (),
            unsafe { &raw const *z_str.offset(i as isize) } as *const (),
            (n_str as i64 - i) as u64)
    };
    j += n_str as i64 - i;
    { let _ = 0; };
    unsafe { *z_out.offset(j as isize) = 0 as u8 };
    unsafe {
        sqlite3_result_text(context, z_out as *mut i8 as *const i8, j as i32,
            Some(sqlite3_free))
    };
}

extern "C" fn zeroblob_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut n: i64 = 0 as i64;
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = argc; };
    n = unsafe { sqlite3_value_int64(unsafe { *argv.offset(0 as isize) }) };
    if n < 0 as i64 { n = 0 as i64; }
    rc = unsafe { sqlite3_result_zeroblob64(context, n as Sqlite3Uint64) };
    if rc != 0 { unsafe { sqlite3_result_error_code(context, rc) }; }
}

extern "C" fn substr_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z: *const u8 = core::ptr::null();
    let mut z2: *const u8 = core::ptr::null();
    let mut len: i32 = 0;
    let mut p0type: i32 = 0;
    let mut p1: i64 = 0 as i64;
    let mut p2: i64 = 0 as i64;
    { let _ = 0; };
    p0type =
        unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) };
    p1 = unsafe { sqlite3_value_int64(unsafe { *argv.offset(1 as isize) }) };
    if p0type == 4 {
        len =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        z =
            unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) }
                as *const u8;
        if z == core::ptr::null() { return; }
        { let _ = 0; };
    } else {
        z =
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
            };
        if z == core::ptr::null() { return; }
        len = 0;
        if p1 < 0 as i64 {
            {
                z2 = z;
                '__b38: loop {
                    if !(unsafe { *z2 } != 0) { break '__b38; }
                    '__c38: loop {
                        {
                            if unsafe {
                                            *{
                                                    let __p = &mut z2;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }
                                        } as i32 >= 192 {
                                while unsafe { *z2 } as i32 & 192 == 128 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                            }
                        }
                        break '__c38;
                    }
                    { let __p = &mut len; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
    if argc == 3 {
        p2 =
            unsafe {
                sqlite3_value_int64(unsafe { *argv.offset(2 as isize) })
            };
        if p2 == 0 as i64 &&
                unsafe {
                        sqlite3_value_type(unsafe { *argv.offset(2 as isize) })
                    } == 5 {
            return;
        }
    } else {
        p2 =
            unsafe {
                    (*unsafe {
                                        sqlite3_context_db_handle(context)
                                    }).a_limit[0 as usize]
                } as i64;
    }
    if p1 == 0 as i64 {
        if unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) }
                == 5 {
            return;
        }
    }
    if p1 < 0 as i64 {
        p1 += len as i64;
        if p1 < 0 as i64 {
            if p2 < 0 as i64 { p2 = 0 as i64; } else { p2 += p1; }
            p1 = 0 as i64;
        }
    } else if p1 > 0 as i64 {
        { let __p = &mut p1; let __t = *__p; *__p -= 1; __t };
    } else if p2 > 0 as i64 {
        { let __p = &mut p2; let __t = *__p; *__p -= 1; __t };
    }
    if p2 < 0 as i64 { if p2 < -p1 { p2 = p1; } else { p2 = -p2; } p1 -= p2; }
    { let _ = 0; };
    if p0type != 4 {
        {
            '__b40: loop {
                if !(p1 > 0 as i64) { break '__b40; }
                '__c40: loop {
                    if ((unsafe { *z.offset(0 as isize) } as i32 - 1) as u8 as
                                    i32) < 128 - 1 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    } else if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                        break '__b40;
                    } else {
                        '__b41: loop {
                            '__c41: loop {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                break '__c41;
                            }
                            if !(unsafe { *z.offset(0 as isize) } as i32 & 192 == 128) {
                                break '__b41;
                            }
                        }
                    }
                    break '__c40;
                }
                { let __p = &mut p1; let __t = *__p; *__p -= 1; __t };
            }
        }
        {
            z2 = z;
            '__b42: loop {
                if !(p2 > 0 as i64) { break '__b42; }
                '__c42: loop {
                    if ((unsafe { *z2.offset(0 as isize) } as i32 - 1) as u8 as
                                    i32) < 128 - 1 {
                        {
                            let __p = &mut z2;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    } else if unsafe { *z2.offset(0 as isize) } as i32 == 0 {
                        break '__b42;
                    } else {
                        '__b43: loop {
                            '__c43: loop {
                                {
                                    let __p = &mut z2;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                break '__c43;
                            }
                            if !(unsafe { *z2.offset(0 as isize) } as i32 & 192 == 128)
                                {
                                break '__b43;
                            }
                        }
                    }
                    break '__c42;
                }
                { let __p = &mut p2; let __t = *__p; *__p -= 1; __t };
            }
        }
        unsafe {
            sqlite3_result_text64(context, z as *mut i8 as *const i8,
                unsafe { z2.offset_from(z) } as i64 as Sqlite3Uint64,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }), 1 as u8)
        };
    } else {
        if p1 >= len as i64 {
            p1 = { p2 = 0 as i64; p2 };
        } else if p2 > len as i64 - p1 {
            p2 = len as i64 - p1;
            { let _ = 0; };
        }
        unsafe {
            sqlite3_result_blob64(context,
                unsafe { &raw const *z.offset(p1 as isize) } as *mut i8 as
                    *const (), p2 as u64,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SumCtx {
    r_sum: f64,
    r_err: f64,
    i_sum: i64,
    cnt: i64,
    approx: u8,
    ovrfl: u8,
}

extern "C" fn kahan_babuska_neumaier_init(p: *mut SumCtx, i_val_1: i64)
    -> () {
    if i_val_1 <= -4503599627370496i64 || i_val_1 >= 4503599627370496i64 {
        let i_sm: i64 = i_val_1 % 16384 as i64;
        unsafe { (*p).r_sum = (i_val_1 - i_sm) as f64 };
        unsafe { (*p).r_err = i_sm as f64 };
    } else {
        unsafe { (*p).r_sum = i_val_1 as f64 };
        unsafe { (*p).r_err = 0.0 };
    }
}

extern "C" fn kahan_babuska_neumaier_step(p_sum_1: *mut SumCtx, r: f64)
    -> () {
    let s: f64 = unsafe { (*p_sum_1).r_sum } as f64;
    let t: f64 = (s + r as f64) as f64;
    if unsafe { fabs(s) } > unsafe { fabs(r) } {
        unsafe { (*p_sum_1).r_err += s - t as f64 + r as f64 };
    } else { unsafe { (*p_sum_1).r_err += r - t as f64 + s as f64 }; }
    unsafe { (*p_sum_1).r_sum = t as f64 };
}

extern "C" fn kahan_babuska_neumaier_step_int64(p_sum_1: *mut SumCtx,
    i_val_1: i64) -> () {
    if i_val_1 <= -4503599627370496i64 || i_val_1 >= 4503599627370496i64 {
        let mut i_big: i64 = 0 as i64;
        let mut i_sm: i64 = 0 as i64;
        i_sm = i_val_1 % 16384 as i64;
        i_big = i_val_1 - i_sm;
        kahan_babuska_neumaier_step(p_sum_1, i_big as f64);
        kahan_babuska_neumaier_step(p_sum_1, i_sm as f64);
    } else { kahan_babuska_neumaier_step(p_sum_1, i_val_1 as f64 as f64); }
}

extern "C" fn sum_step(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut SumCtx = core::ptr::null_mut();
    let mut type_: i32 = 0;
    { let _ = 0; };
    { let _ = argc; };
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<SumCtx>() as i32)
            } as *mut SumCtx;
    type_ =
        unsafe {
            sqlite3_value_numeric_type(unsafe { *argv.offset(0 as isize) })
        };
    if !(p).is_null() && type_ != 5 {
        {
            let __p = unsafe { &mut (*p).cnt };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if unsafe { (*p).approx } as i32 == 0 {
            if type_ != 1 {
                kahan_babuska_neumaier_init(p, unsafe { (*p).i_sum });
                unsafe { (*p).approx = 1 as u8 };
                kahan_babuska_neumaier_step(p,
                    unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        } as f64);
            } else {
                let mut x: i64 = unsafe { (*p).i_sum };
                if unsafe {
                            sqlite3_add_int64(&mut x,
                                unsafe {
                                    sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                                })
                        } == 0 {
                    unsafe { (*p).i_sum = x };
                } else {
                    unsafe { (*p).ovrfl = 1 as u8 };
                    kahan_babuska_neumaier_init(p, unsafe { (*p).i_sum });
                    unsafe { (*p).approx = 1 as u8 };
                    kahan_babuska_neumaier_step_int64(p,
                        unsafe {
                            sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                        });
                }
            }
        } else {
            if type_ == 1 {
                kahan_babuska_neumaier_step_int64(p,
                    unsafe {
                        sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                    });
            } else {
                unsafe { (*p).ovrfl = 0 as u8 };
                kahan_babuska_neumaier_step(p,
                    unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        } as f64);
            }
        }
    }
}

extern "C" fn sum_finalize(context: *mut Sqlite3Context) -> () {
    let mut p: *const SumCtx = core::ptr::null();
    p = unsafe { sqlite3_aggregate_context(context, 0) } as *mut SumCtx;
    if !(p).is_null() && unsafe { (*p).cnt } > 0 as i64 {
        if unsafe { (*p).approx } != 0 {
            if unsafe { (*p).ovrfl } != 0 {
                unsafe {
                    sqlite3_result_error(context,
                        c"integer overflow".as_ptr() as *mut i8 as *const i8, -1)
                };
            } else if (unsafe { sqlite3_is_overflow(unsafe { (*p).r_err }) }
                            == 0) as i32 != 0 {
                unsafe {
                    sqlite3_result_double(context,
                        unsafe { (*p).r_sum } + unsafe { (*p).r_err })
                };
            } else {
                unsafe {
                    sqlite3_result_double(context, unsafe { (*p).r_sum })
                };
            }
        } else {
            unsafe { sqlite3_result_int64(context, unsafe { (*p).i_sum }) };
        }
    }
}

extern "C" fn sum_inverse(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut SumCtx = core::ptr::null_mut();
    let mut type_: i32 = 0;
    { let _ = 0; };
    { let _ = argc; };
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<SumCtx>() as i32)
            } as *mut SumCtx;
    type_ =
        unsafe {
            sqlite3_value_numeric_type(unsafe { *argv.offset(0 as isize) })
        };
    if !(p).is_null() && type_ != 5 {
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).cnt };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if (unsafe { (*p).approx } == 0) as i32 != 0 {
            let mut x: i64 = unsafe { (*p).i_sum };
            if unsafe {
                        sqlite3_sub_int64(&mut x,
                            unsafe {
                                sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                            })
                    } == 0 {
                unsafe { (*p).i_sum = x };
                return;
            }
            unsafe { (*p).ovrfl = 1 as u8 };
            unsafe { (*p).approx = 1 as u8 };
            kahan_babuska_neumaier_init(p, unsafe { (*p).i_sum });
        }
        if type_ == 1 {
            let i_val: i64 =
                unsafe {
                    sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                };
            if i_val !=
                    -1 as i64 -
                        (4294967295u32 as i64 | (2147483647 as i64) << 32) {
                kahan_babuska_neumaier_step_int64(p, -i_val);
            } else {
                kahan_babuska_neumaier_step_int64(p,
                    4294967295u32 as i64 | (2147483647 as i64) << 32);
                kahan_babuska_neumaier_step_int64(p, 1 as i64);
            }
        } else {
            kahan_babuska_neumaier_step(p,
                -unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        } as f64);
        }
    }
}

extern "C" fn total_finalize(context: *mut Sqlite3Context) -> () {
    let mut p: *const SumCtx = core::ptr::null();
    let mut r: f64 = 0.0;
    p = unsafe { sqlite3_aggregate_context(context, 0) } as *mut SumCtx;
    if !(p).is_null() {
        if unsafe { (*p).approx } != 0 {
            r = unsafe { (*p).r_sum };
            if (unsafe { sqlite3_is_overflow(unsafe { (*p).r_err }) } == 0) as
                        i32 != 0 {
                r += unsafe { (*p).r_err };
            }
        } else { r = unsafe { (*p).i_sum } as f64; }
    }
    unsafe { sqlite3_result_double(context, r) };
}

extern "C" fn avg_finalize(context: *mut Sqlite3Context) -> () {
    let mut p: *const SumCtx = core::ptr::null();
    p = unsafe { sqlite3_aggregate_context(context, 0) } as *mut SumCtx;
    if !(p).is_null() && unsafe { (*p).cnt } > 0 as i64 {
        let mut r: f64 = 0.0;
        if unsafe { (*p).approx } != 0 {
            r = unsafe { (*p).r_sum };
            if (unsafe { sqlite3_is_overflow(unsafe { (*p).r_err }) } == 0) as
                        i32 != 0 {
                r += unsafe { (*p).r_err };
            }
        } else { r = unsafe { (*p).i_sum } as f64; }
        unsafe {
            sqlite3_result_double(context, r / unsafe { (*p).cnt } as f64)
        };
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CountCtx {
    n: i64,
}

extern "C" fn count_step(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut p: *mut CountCtx = core::ptr::null_mut();
        p =
            unsafe {
                    sqlite3_aggregate_context(context,
                        core::mem::size_of::<CountCtx>() as i32)
                } as *mut CountCtx;
        if (argc == 0 ||
                    5 !=
                        unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                        }) && !(p).is_null() {
            {
                let __p = unsafe { &mut (*p).n };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        { let _ = 0; };
    }
}

extern "C" fn count_finalize(context: *mut Sqlite3Context) -> () {
    unsafe {
        let mut p: *const CountCtx = core::ptr::null();
        p = unsafe { sqlite3_aggregate_context(context, 0) } as *mut CountCtx;
        unsafe {
            sqlite3_result_int64(context,
                if !(p).is_null() { unsafe { (*p).n } } else { 0 as i64 })
        };
    }
}

extern "C" fn count_inverse(ctx: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut p: *mut CountCtx = core::ptr::null_mut();
        p =
            unsafe {
                    sqlite3_aggregate_context(ctx,
                        core::mem::size_of::<CountCtx>() as i32)
                } as *mut CountCtx;
        if (argc == 0 ||
                    5 !=
                        unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                        }) && !(p).is_null() {
            {
                let __p = unsafe { &mut (*p).n };
                let __t = *__p;
                *__p -= 1;
                __t
            };
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct GroupConcatCtx {
    str: StrAccum,
    n_accum: i32,
    n_first_sep_length: i32,
    pn_sep_lengths: *mut i32,
}

extern "C" fn group_concat_step(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_val: *const i8 = core::ptr::null();
    let mut p_gcc: *mut GroupConcatCtx = core::ptr::null_mut();
    let mut z_sep: *const i8 = core::ptr::null();
    let mut n_val: i32 = 0;
    let mut n_sep: i32 = 0;
    { let _ = 0; };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    p_gcc =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<GroupConcatCtx>() as i32)
            } as *mut GroupConcatCtx;
    if !(p_gcc).is_null() {
        let db: *const Sqlite3 =
            unsafe { sqlite3_context_db_handle(context) } as *const Sqlite3;
        let first_term: i32 =
            (unsafe { (*p_gcc).str.mx_alloc } == 0 as u32) as i32;
        unsafe {
            (*p_gcc).str.mx_alloc =
                unsafe { (*db).a_limit[0 as usize] } as u32
        };
        if argc == 1 {
            if (first_term == 0) as i32 != 0 {
                unsafe {
                    sqlite3_str_appendchar(unsafe { &raw mut (*p_gcc).str } as
                            *mut Sqlite3Str, 1, ',' as i32 as i8)
                };
            } else { unsafe { (*p_gcc).n_first_sep_length = 1 }; }
        } else if (first_term == 0) as i32 != 0 {
            z_sep =
                unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                        } as *mut i8 as *const i8;
            n_sep =
                unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                };
            if !(z_sep).is_null() {
                unsafe {
                    sqlite3_str_append(unsafe { &raw mut (*p_gcc).str } as
                            *mut Sqlite3Str, z_sep, n_sep)
                };
            } else { n_sep = 0; }
            if n_sep != unsafe { (*p_gcc).n_first_sep_length } ||
                    unsafe { (*p_gcc).pn_sep_lengths } != core::ptr::null_mut()
                {
                let mut pnsl: *mut i32 = unsafe { (*p_gcc).pn_sep_lengths };
                if pnsl == core::ptr::null_mut() {
                    pnsl =
                        unsafe {
                                sqlite3_malloc64((unsafe { (*p_gcc).n_accum } + 1) as u64 *
                                        core::mem::size_of::<i32>() as u64)
                            } as *mut i32;
                    if pnsl != core::ptr::null_mut() {
                        let mut i: i32 = 0;
                        let n_a: i32 = unsafe { (*p_gcc).n_accum } - 1;
                        while i < n_a {
                            unsafe {
                                *pnsl.offset({
                                                    let __p = &mut i;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) = unsafe { (*p_gcc).n_first_sep_length }
                            };
                        }
                    }
                } else {
                    pnsl =
                        unsafe {
                                sqlite3_realloc64(pnsl as *mut (),
                                    unsafe { (*p_gcc).n_accum } as u64 *
                                        core::mem::size_of::<i32>() as u64)
                            } as *mut i32;
                }
                if pnsl != core::ptr::null_mut() {
                    if unsafe { (*p_gcc).n_accum } > 0 {
                        unsafe {
                            *pnsl.offset((unsafe { (*p_gcc).n_accum } - 1) as isize) =
                                n_sep
                        };
                    }
                    unsafe { (*p_gcc).pn_sep_lengths = pnsl };
                } else {
                    unsafe {
                        sqlite3_str_accum_set_error(unsafe { &mut (*p_gcc).str },
                            7 as u8)
                    };
                }
            }
        } else {
            unsafe {
                (*p_gcc).n_first_sep_length =
                    unsafe {
                        sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                    }
            };
        }
        unsafe { (*p_gcc).n_accum += 1 };
        z_val =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                    as *mut i8 as *const i8;
        n_val =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        if !(z_val).is_null() {
            unsafe {
                sqlite3_str_append(unsafe { &raw mut (*p_gcc).str } as
                        *mut Sqlite3Str, z_val, n_val)
            };
        }
    }
}

extern "C" fn group_concat_finalize(context: *mut Sqlite3Context) -> () {
    let p_gcc: *mut GroupConcatCtx =
        unsafe { sqlite3_aggregate_context(context, 0) } as
            *mut GroupConcatCtx;
    if !(p_gcc).is_null() {
        unsafe {
            sqlite3_result_str(context,
                unsafe { &raw mut (*p_gcc).str } as *mut Sqlite3Str, 1)
        };
        unsafe {
            sqlite3_free(unsafe { (*p_gcc).pn_sep_lengths } as *mut ())
        };
    }
}

extern "C" fn group_concat_value(context: *mut Sqlite3Context) -> () {
    let p_gcc: *mut GroupConcatCtx =
        unsafe { sqlite3_aggregate_context(context, 0) } as
            *mut GroupConcatCtx;
    if !(p_gcc).is_null() && unsafe { (*p_gcc).n_accum } > 0 {
        unsafe {
            sqlite3_result_str(context,
                unsafe { &raw mut (*p_gcc).str } as *mut Sqlite3Str, 0)
        };
    }
}

extern "C" fn group_concat_inverse(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p_gcc: *mut GroupConcatCtx = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = argc; };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    p_gcc =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<GroupConcatCtx>() as i32)
            } as *mut GroupConcatCtx;
    if !(p_gcc).is_null() {
        let mut n_vs: i32 = 0;
        {
            let _ =
                unsafe {
                    sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                };
        };
        n_vs =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        unsafe { (*p_gcc).n_accum -= 1 };
        if unsafe { (*p_gcc).pn_sep_lengths } != core::ptr::null_mut() {
            { let _ = 0; };
            if unsafe { (*p_gcc).n_accum } > 0 {
                n_vs += unsafe { *unsafe { (*p_gcc).pn_sep_lengths } };
                unsafe {
                    memmove(unsafe { (*p_gcc).pn_sep_lengths } as *mut (),
                        unsafe {
                                unsafe { (*p_gcc).pn_sep_lengths.offset(1 as isize) }
                            } as *const (),
                        (unsafe { (*p_gcc).n_accum } - 1) as u64 *
                            core::mem::size_of::<i32>() as u64)
                };
            }
        } else { n_vs += unsafe { (*p_gcc).n_first_sep_length }; }
        if n_vs >= unsafe { (*p_gcc).str.n_char } as i32 {
            unsafe { (*p_gcc).str.n_char = 0 as u32 };
        } else {
            unsafe { (*p_gcc).str.n_char -= n_vs as u32 };
            unsafe {
                memmove(unsafe { (*p_gcc).str.z_text } as *mut (),
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_gcc).str.z_text.offset(n_vs as isize)
                                    }
                        } as *const (), unsafe { (*p_gcc).str.n_char } as u64)
            };
        }
        if unsafe { (*p_gcc).str.n_char } == 0 as u32 {
            unsafe { (*p_gcc).str.mx_alloc = 0 as u32 };
            unsafe {
                sqlite3_free(unsafe { (*p_gcc).pn_sep_lengths } as *mut ())
            };
            unsafe { (*p_gcc).pn_sep_lengths = core::ptr::null_mut() };
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Percentile {
    n_alloc: u64,
    n_used: u64,
    b_sorted: i8,
    b_keep_sorted: i8,
    b_pct_valid: i8,
    r_pct: f64,
    a: *mut f64,
}

unsafe extern "C" fn percent_error(p_ctx_1: *mut Sqlite3Context,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut z_msg1: *mut i8 = core::ptr::null_mut();
    let mut z_msg2: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg1 = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    z_msg2 =
        if !(z_msg1).is_null() {
            unsafe {
                sqlite3_mprintf(z_msg1 as *const i8,
                    unsafe {
                        sqlite3_vdbe_func_name(p_ctx_1 as *const Sqlite3Context)
                    })
            }
        } else { core::ptr::null_mut() };
    unsafe { sqlite3_result_error(p_ctx_1, z_msg2 as *const i8, -1) };
    unsafe { sqlite3_free(z_msg1 as *mut ()) };
    unsafe { sqlite3_free(z_msg2 as *mut ()) };
}

extern "C" fn percent_same_value(mut a: f64, b: f64) -> i32 {
    a -= b;
    return (a >= -0.001 && a <= 0.001) as i32;
}

extern "C" fn percent_is_infinity(mut r: f64) -> i32 {
    let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    { let _ = 0; };
    unsafe {
        memcpy(&raw mut u as *mut (), &raw mut r as *const (),
            core::mem::size_of::<Sqlite3Uint64>() as u64)
    };
    return (u >> 52 & 2047 as Sqlite3Uint64 == 2047 as u64) as i32;
}

extern "C" fn percent_binary_search(p: &Percentile, y: f64, b_exact_1: i32)
    -> i64 {
    let mut i_first: i64 = 0 as i64;
    let mut i_last: i64 = (*p).n_used as i64 - 1 as i64;
    while i_last >= i_first {
        let i_mid: i64 = (i_first + i_last) / 2 as i64;
        let x: f64 = unsafe { *(*p).a.offset(i_mid as isize) };
        if x < y {
            i_first = i_mid + 1 as i64;
        } else if x > y { i_last = i_mid - 1 as i64; } else { return i_mid; }
    }
    if b_exact_1 != 0 { return -1 as i64; }
    return i_first;
}

extern "C" fn percent_step(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut Percentile = core::ptr::null_mut();
    let mut r_pct: f64 = 0.0;
    let mut e_type: i32 = 0;
    let mut y: f64 = 0.0;
    { let _ = 0; };
    if argc == 1 {
        r_pct = 0.5;
    } else {
        let mx_frac: f64 =
            if unsafe { sqlite3_user_data(p_ctx_1) } as i64 as i32 & 2 != 0 {
                100.0
            } else { 1.0 };
        e_type =
            unsafe {
                sqlite3_value_numeric_type(unsafe {
                        *argv.offset(1 as isize)
                    })
            };
        r_pct =
            unsafe {
                    sqlite3_value_double(unsafe { *argv.offset(1 as isize) })
                } / mx_frac;
        if e_type != 1 && e_type != 2 || r_pct < 0.0 || r_pct > 1.0 {
            unsafe {
                percent_error(p_ctx_1,
                    c"the fraction argument to %%s() is not between 0.0 and %.1f".as_ptr()
                            as *mut i8 as *const i8, mx_frac as f64)
            };
            return;
        }
    }
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<Percentile>() as i32)
            } as *mut Percentile;
    if p == core::ptr::null_mut() { return; }
    if (unsafe { (*p).b_pct_valid } == 0) as i32 != 0 {
        unsafe { (*p).r_pct = r_pct };
        unsafe { (*p).b_pct_valid = 1 as i8 };
    } else if (percent_same_value(unsafe { (*p).r_pct }, r_pct) == 0) as i32
            != 0 {
        unsafe {
            percent_error(p_ctx_1,
                c"the fraction argument to %%s() is not the same for all input rows".as_ptr()
                        as *mut i8 as *const i8)
        };
        return;
    }
    e_type =
        unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) };
    if e_type == 5 { return; }
    if e_type != 1 && e_type != 2 {
        unsafe {
            percent_error(p_ctx_1,
                c"input to %%s() is not numeric".as_ptr() as *mut i8 as
                    *const i8)
        };
        return;
    }
    y = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    if percent_is_infinity(y) != 0 {
        unsafe {
            percent_error(p_ctx_1,
                c"Inf input to %%s()".as_ptr() as *mut i8 as *const i8)
        };
        return;
    }
    if unsafe { (*p).n_used } >= unsafe { (*p).n_alloc } {
        let n: u64 = unsafe { (*p).n_alloc } * 2 as u64 + 250 as u64;
        let a: *mut f64 =
            unsafe {
                    sqlite3_realloc64(unsafe { (*p).a } as *mut (),
                        core::mem::size_of::<f64>() as u64 * n)
                } as *mut f64;
        if a == core::ptr::null_mut() {
            unsafe { sqlite3_free(unsafe { (*p).a } as *mut ()) };
            unsafe {
                memset(p as *mut (), 0,
                    core::mem::size_of::<Percentile>() as u64)
            };
            unsafe { sqlite3_result_error_nomem(p_ctx_1) };
            return;
        }
        unsafe { (*p).n_alloc = n };
        unsafe { (*p).a = a };
    }
    if unsafe { (*p).n_used } == 0 as u64 {
        unsafe {
            *unsafe {
                        (*p).a.add({
                                    let __p = unsafe { &mut (*p).n_used };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = y
        };
        unsafe { (*p).b_sorted = 1 as i8 };
    } else if (unsafe { (*p).b_sorted } == 0) as i32 != 0 ||
            y >=
                unsafe {
                    *unsafe {
                            (*p).a.add((unsafe { (*p).n_used } - 1 as u64) as usize)
                        }
                } {
        unsafe {
            *unsafe {
                        (*p).a.add({
                                    let __p = unsafe { &mut (*p).n_used };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = y
        };
    } else if unsafe { (*p).b_keep_sorted } != 0 {
        let mut i: i64 = 0 as i64;
        i = percent_binary_search(unsafe { &*p }, y, 0);
        if i < unsafe { (*p).n_used } as i32 as i64 {
            unsafe {
                memmove(unsafe {
                            &raw mut *unsafe { (*p).a.offset((i + 1 as i64) as isize) }
                        } as *mut (),
                    unsafe { &raw mut *unsafe { (*p).a.offset(i as isize) } } as
                        *const (),
                    (unsafe { (*p).n_used } - i as u64) *
                        core::mem::size_of::<f64>() as u64)
            };
        }
        unsafe { *unsafe { (*p).a.offset(i as isize) } = y };
        {
            let __p = unsafe { &mut (*p).n_used };
            let __t = *__p;
            *__p += 1;
            __t
        };
    } else {
        unsafe {
            *unsafe {
                        (*p).a.add({
                                    let __p = unsafe { &mut (*p).n_used };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = y
        };
        unsafe { (*p).b_sorted = 0 as i8 };
    }
}

extern "C" fn percent_sort(mut a: *mut f64, mut n: u32, mut i_req_1: i32)
    -> () {
    let mut i_lt: i32 = 0;
    let mut i_gt: i32 = 0;
    let mut i: i32 = 0;
    let mut r_pivot: f64 = 0.0;
    { let _ = 0; };
    '__b46: loop {
        '__c46: loop {
            if unsafe { *a.offset(0 as isize) } >
                    unsafe { *a.add((n - 1 as u32) as usize) } {
                {
                    let ttt: f64 = unsafe { *a.offset(0 as isize) };
                    unsafe {
                        *a.offset(0 as isize) =
                            unsafe { *a.add((n - 1 as u32) as usize) }
                    };
                    unsafe { *a.add((n - 1 as u32) as usize) = ttt };
                }
            }
            if n == 2 as u32 { return; }
            i_gt = (n - 1 as u32) as i32;
            i = (n / 2 as u32) as i32;
            if unsafe { *a.offset(0 as isize) } >
                    unsafe { *a.offset(i as isize) } {
                {
                    let ttt: f64 = unsafe { *a.offset(0 as isize) };
                    unsafe {
                        *a.offset(0 as isize) = unsafe { *a.offset(i as isize) }
                    };
                    unsafe { *a.offset(i as isize) = ttt };
                }
            } else if unsafe { *a.offset(i as isize) } >
                    unsafe { *a.offset(i_gt as isize) } {
                {
                    let ttt: f64 = unsafe { *a.offset(i as isize) };
                    unsafe {
                        *a.offset(i as isize) = unsafe { *a.offset(i_gt as isize) }
                    };
                    unsafe { *a.offset(i_gt as isize) = ttt };
                }
            }
            if n == 3 as u32 { return; }
            r_pivot = unsafe { *a.offset(i as isize) };
            i_lt = { i = 1; i };
            '__b47: loop {
                '__c47: loop {
                    if unsafe { *a.offset(i as isize) } < r_pivot {
                        if i > i_lt {
                            let ttt: f64 = unsafe { *a.offset(i as isize) };
                            unsafe {
                                *a.offset(i as isize) = unsafe { *a.offset(i_lt as isize) }
                            };
                            unsafe { *a.offset(i_lt as isize) = ttt };
                        }
                        { let __p = &mut i_lt; let __t = *__p; *__p += 1; __t };
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    } else if unsafe { *a.offset(i as isize) } > r_pivot {
                        '__b48: loop {
                            '__c48: loop {
                                { let __p = &mut i_gt; let __t = *__p; *__p -= 1; __t };
                                break '__c48;
                            }
                            if !(i_gt > i &&
                                            unsafe { *a.offset(i_gt as isize) } > r_pivot) {
                                break '__b48;
                            }
                        }
                        {
                            let ttt: f64 = unsafe { *a.offset(i as isize) };
                            unsafe {
                                *a.offset(i as isize) = unsafe { *a.offset(i_gt as isize) }
                            };
                            unsafe { *a.offset(i_gt as isize) = ttt };
                        }
                    } else {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c47;
                }
                if !(i < i_gt) { break '__b47; }
            }
            { let _ = 0; };
            { let _ = 0; };
            if i_req_1 >= 0 {
                if i_req_1 < i_lt {
                    n = i_lt as u32;
                } else {
                    {
                        let __n = i_gt;
                        let __p = &mut a;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    n -= i_gt as u32;
                    i_req_1 =
                        if 0 > i_req_1 - i_gt { 0 } else { i_req_1 - i_gt };
                }
            } else {
                if i_lt > (n / 2 as u32) as i32 {
                    if n - i_gt as u32 >= 2 as u32 {
                        percent_sort(unsafe { a.offset(i_gt as isize) },
                            n - i_gt as u32, -1);
                    }
                    n = i_lt as u32;
                } else {
                    if i_lt >= 2 { percent_sort(a, i_lt as u32, -1); }
                    {
                        let __n = i_gt;
                        let __p = &mut a;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    n -= i_gt as u32;
                }
            }
            break '__c46;
        }
        if !(n >= 2 as u32) { break '__b46; }
    }
}

extern "C" fn percent_compute(p_ctx_1: *mut Sqlite3Context, b_is_final_1: i32)
    -> () {
    let mut p: *mut Percentile = core::ptr::null_mut();
    let settings: i32 =
        unsafe { sqlite3_user_data(p_ctx_1) } as i64 as i32 & 1;
    let mut i1: u32 = 0 as u32;
    let mut i2: u32 = 0 as u32;
    let mut v1: f64 = 0.0;
    let mut v2: f64 = 0.0;
    let mut ix: f64 = 0.0;
    let mut vx: f64 = 0.0;
    p = unsafe { sqlite3_aggregate_context(p_ctx_1, 0) } as *mut Percentile;
    if p == core::ptr::null_mut() { return; }
    if unsafe { (*p).a } == core::ptr::null_mut() { return; }
    if unsafe { (*p).n_used } != 0 {
        ix =
            unsafe { (*p).r_pct } *
                (unsafe { (*p).n_used } - 1 as u64) as f64;
        i1 = ix as u32;
        if unsafe { (*p).b_sorted } as i32 == 0 {
            { let _ = 0; };
            percent_sort(unsafe { (*p).a }, unsafe { (*p).n_used } as u32,
                if b_is_final_1 != 0 { i1 as i32 } else { -1 });
            unsafe { (*p).b_sorted = 1 as i8 };
        }
        if settings & 1 != 0 {
            vx = unsafe { *unsafe { (*p).a.add(i1 as usize) } };
        } else {
            i2 =
                if ix == i1 as f64 ||
                        i1 as u64 == unsafe { (*p).n_used } - 1 as u64 {
                    i1
                } else { i1 + 1 as u32 };
            v1 = unsafe { *unsafe { (*p).a.add(i1 as usize) } };
            v2 = unsafe { *unsafe { (*p).a.add(i2 as usize) } };
            vx = v1 + (v2 - v1) * (ix - i1 as f64);
        }
        unsafe { sqlite3_result_double(p_ctx_1, vx) };
    }
    if b_is_final_1 != 0 {
        unsafe { sqlite3_free(unsafe { (*p).a } as *mut ()) };
        unsafe {
            memset(p as *mut (), 0, core::mem::size_of::<Percentile>() as u64)
        };
    } else { unsafe { (*p).b_keep_sorted = 1 as i8 }; }
}

extern "C" fn percent_final(p_ctx_1: *mut Sqlite3Context) -> () {
    percent_compute(p_ctx_1, 1);
}

extern "C" fn percent_value(p_ctx_1: *mut Sqlite3Context) -> () {
    percent_compute(p_ctx_1, 0);
}

extern "C" fn percent_inverse(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut Percentile = core::ptr::null_mut();
    let mut e_type: i32 = 0;
    let mut y: f64 = 0.0;
    let mut i: i64 = 0 as i64;
    { let _ = 0; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<Percentile>() as i32)
            } as *mut Percentile;
    { let _ = 0; };
    e_type =
        unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) };
    if e_type == 5 { return; }
    if e_type != 1 && e_type != 2 { return; }
    y = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    if percent_is_infinity(y) != 0 { return; }
    if unsafe { (*p).b_sorted } as i32 == 0 {
        { let _ = 0; };
        percent_sort(unsafe { (*p).a }, unsafe { (*p).n_used } as u32, -1);
        unsafe { (*p).b_sorted = 1 as i8 };
    }
    unsafe { (*p).b_keep_sorted = 1 as i8 };
    i = percent_binary_search(unsafe { &*p }, y, 1);
    if i >= 0 as i64 {
        {
            let __p = unsafe { &mut (*p).n_used };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if i < unsafe { (*p).n_used } as i32 as i64 {
            unsafe {
                memmove(unsafe {
                            &raw mut *unsafe { (*p).a.offset(i as isize) }
                        } as *mut (),
                    unsafe {
                            &raw mut *unsafe { (*p).a.offset((i + 1 as i64) as isize) }
                        } as *const (),
                    (unsafe { (*p).n_used } - i as u64) *
                        core::mem::size_of::<f64>() as u64)
            };
        }
    }
}

extern "C" fn like_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_a: *const u8 = core::ptr::null();
    let mut z_b: *const u8 = core::ptr::null();
    let mut escape: u32 = 0 as u32;
    let mut n_pat: i32 = 0;
    let db: *const Sqlite3 =
        unsafe { sqlite3_context_db_handle(context) } as *const Sqlite3;
    let mut p_info: *mut CompareInfo =
        unsafe { sqlite3_user_data(context) } as *mut CompareInfo;
    let mut backup_info: CompareInfo = unsafe { core::mem::zeroed() };
    n_pat =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    if n_pat > unsafe { (*db).a_limit[8 as usize] } {
        unsafe {
            sqlite3_result_error(context,
                c"LIKE or GLOB pattern too complex".as_ptr() as *mut i8 as
                    *const i8, -1)
        };
        return;
    }
    if argc == 3 {
        let mut z_esc: *const u8 =
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(2 as isize) })
            };
        if z_esc == core::ptr::null() { return; }
        if unsafe { sqlite3_utf8_char_len(z_esc as *mut i8 as *const i8, -1) }
                != 1 {
            unsafe {
                sqlite3_result_error(context,
                    c"ESCAPE expression must be a single character".as_ptr() as
                            *mut i8 as *const i8, -1)
            };
            return;
        }
        escape = unsafe { sqlite3_utf8_read(&mut z_esc) };
        if escape == unsafe { (*p_info).match_all } as u32 ||
                escape == unsafe { (*p_info).match_one } as u32 {
            unsafe {
                memcpy(&raw mut backup_info as *mut (), p_info as *const (),
                    core::mem::size_of::<CompareInfo>() as u64)
            };
            p_info = &mut backup_info;
            if escape == unsafe { (*p_info).match_all } as u32 {
                unsafe { (*p_info).match_all = 0 as u8 };
            }
            if escape == unsafe { (*p_info).match_one } as u32 {
                unsafe { (*p_info).match_one = 0 as u8 };
            }
        }
    } else { escape = unsafe { (*p_info).match_set } as u32; }
    z_b = unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    z_a = unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) };
    if !(z_a).is_null() && !(z_b).is_null() {
        unsafe {
            sqlite3_result_int(context,
                (pattern_compare(z_b, z_a, p_info as *const CompareInfo,
                            escape) == 0) as i32)
        };
    }
}

extern "C" fn x_ceil(x: f64) -> f64 { return unsafe { ceil(x) }; }

extern "C" fn ceiling_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = 0; };
    '__s49:
        {
        match unsafe {
                sqlite3_value_numeric_type(unsafe {
                        *argv.offset(0 as isize)
                    })
            } {
            1 => {
                {
                    unsafe {
                        sqlite3_result_int64(context,
                            unsafe {
                                sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s49;
                }
                {
                    let x: Option<unsafe extern "C" fn(f64) -> f64> =
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(f64)
                                            -> f64>(unsafe { sqlite3_user_data(context) } as *const ())
                            });
                    unsafe {
                        sqlite3_result_double(context,
                            unsafe {
                                x.unwrap()(unsafe {
                                        sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                                    })
                            })
                    };
                    break '__s49;
                }
                { break '__s49; }
            }
            2 => {
                {
                    let x: Option<unsafe extern "C" fn(f64) -> f64> =
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(f64)
                                            -> f64>(unsafe { sqlite3_user_data(context) } as *const ())
                            });
                    unsafe {
                        sqlite3_result_double(context,
                            unsafe {
                                x.unwrap()(unsafe {
                                        sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                                    })
                            })
                    };
                    break '__s49;
                }
                { break '__s49; }
            }
            _ => { { break '__s49; } }
        }
    }
}

extern "C" fn x_floor(x: f64) -> f64 { return unsafe { floor(x) }; }

extern "C" fn log_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut x: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut ans: f64 = 0.0;
    { let _ = 0; };
    '__s50:
        {
        match unsafe {
                sqlite3_value_numeric_type(unsafe {
                        *argv.offset(0 as isize)
                    })
            } {
            1 => {
                x =
                    unsafe {
                        sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                    };
                if x <= 0.0 { return; }
            }
            2 => {
                x =
                    unsafe {
                        sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                    };
                if x <= 0.0 { return; }
            }
            _ => { return; }
        }
    }
    if argc == 2 {
        '__s51:
            {
            match unsafe {
                    sqlite3_value_numeric_type(unsafe {
                            *argv.offset(0 as isize)
                        })
                } {
                1 => {
                    b = unsafe { log(x) };
                    if b <= 0.0 { return; }
                    x =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(1 as isize) })
                        };
                    if x <= 0.0 { return; }
                }
                2 => {
                    b = unsafe { log(x) };
                    if b <= 0.0 { return; }
                    x =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(1 as isize) })
                        };
                    if x <= 0.0 { return; }
                }
                _ => { return; }
            }
        }
        ans = unsafe { log(x) } / b;
    } else {
        '__s52:
            {
            match unsafe { sqlite3_user_data(context) } as i64 as i32 {
                1 => { ans = unsafe { log10(x) }; }
                2 => { ans = unsafe { log2(x) }; }
                _ => { ans = unsafe { log(x) }; }
            }
        }
    }
    unsafe { sqlite3_result_double(context, ans) };
}

extern "C" fn math1_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut type0: i32 = 0;
    let mut v0: f64 = 0.0;
    let mut ans: f64 = 0.0;
    let mut x: Option<unsafe extern "C" fn(f64) -> f64> = None;
    { let _ = 0; };
    type0 =
        unsafe {
            sqlite3_value_numeric_type(unsafe { *argv.offset(0 as isize) })
        };
    if type0 != 1 && type0 != 2 { return; }
    v0 = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    x =
        Some(unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(f64)
                            -> f64>(unsafe { sqlite3_user_data(context) } as *const ())
            });
    ans = unsafe { x.unwrap()(v0) };
    unsafe { sqlite3_result_double(context, ans) };
}

extern "C" fn math2_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut type0: i32 = 0;
    let mut type1: i32 = 0;
    let mut v0: f64 = 0.0;
    let mut v1: f64 = 0.0;
    let mut ans: f64 = 0.0;
    let mut x: Option<unsafe extern "C" fn(f64, f64) -> f64> = None;
    { let _ = 0; };
    type0 =
        unsafe {
            sqlite3_value_numeric_type(unsafe { *argv.offset(0 as isize) })
        };
    if type0 != 1 && type0 != 2 { return; }
    type1 =
        unsafe {
            sqlite3_value_numeric_type(unsafe { *argv.offset(1 as isize) })
        };
    if type1 != 1 && type1 != 2 { return; }
    v0 = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    v1 = unsafe { sqlite3_value_double(unsafe { *argv.offset(1 as isize) }) };
    x =
        Some(unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(f64, f64)
                            -> f64>(unsafe { sqlite3_user_data(context) } as *const ())
            });
    ans = unsafe { x.unwrap()(v0, v1) };
    unsafe { sqlite3_result_double(context, ans) };
}

extern "C" fn deg_to_rad(x: f64) -> f64 {
    return x * (3.141592653589793 / 180.0);
}

extern "C" fn rad_to_deg(x: f64) -> f64 {
    return x * (180.0 / 3.141592653589793);
}

extern "C" fn pi_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    { let _ = 0; };
    { let _ = argv; };
    unsafe { sqlite3_result_double(context, 3.141592653589793) };
}

extern "C" fn sign_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut type0: i32 = 0;
    let mut x: f64 = 0.0;
    { let _ = argc; };
    { let _ = 0; };
    type0 =
        unsafe {
            sqlite3_value_numeric_type(unsafe { *argv.offset(0 as isize) })
        };
    if type0 != 1 && type0 != 2 { return; }
    x = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    unsafe {
        sqlite3_result_int(context,
            if x < 0.0 { -1 } else { if x > 0.0 { 1 } else { 0 } })
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_builtin_functions() -> () {
    unsafe {
        unsafe { sqlite3_alter_functions() };
        unsafe { sqlite3_window_functions() };
        unsafe { sqlite3_register_date_time_functions() };
        unsafe { sqlite3_register_json_functions() };
        unsafe {
            sqlite3_insert_builtin_funcs(&raw mut a_builtin_func[0 as usize]
                    as *mut FuncDef,
                (core::mem::size_of::<[FuncDef; 108]>() as u64 /
                        core::mem::size_of::<FuncDef>() as u64) as i32)
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_per_connection_builtin_functions(db:
        *mut Sqlite3) -> () {
    let rc: i32 =
        unsafe {
            sqlite3_overload_function(db,
                c"MATCH".as_ptr() as *mut i8 as *const i8, 2)
        };
    { let _ = 0; };
    if rc == 7 { unsafe { sqlite3_oom_fault(db) }; }
}

static like_info_alt: CompareInfo =
    CompareInfo {
        match_all: '%' as i32 as u8,
        match_one: '_' as i32 as u8,
        match_set: 0 as u8,
        no_case: 0 as u8,
    };

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_like_functions(db: *mut Sqlite3,
    case_sensitive_1: i32) -> () {
    let mut p_def: *mut FuncDef = core::ptr::null_mut();
    let mut p_info: *mut CompareInfo = core::ptr::null_mut();
    let mut flags: i32 = 0;
    let mut n_arg: i32 = 0;
    if case_sensitive_1 != 0 {
        p_info = &raw const like_info_alt as *mut CompareInfo;
        flags = 4 | 8;
    } else {
        p_info = &raw const like_info_norm as *mut CompareInfo;
        flags = 4;
    }
    {
        n_arg = 2;
        '__b53: loop {
            if !(n_arg <= 3) { break '__b53; }
            '__c53: loop {
                unsafe {
                    sqlite3_create_func(db,
                        c"like".as_ptr() as *mut i8 as *const i8, n_arg, 1,
                        p_info as *mut (), Some(like_func), None, None, None, None,
                        core::ptr::null_mut())
                };
                p_def =
                    unsafe {
                        sqlite3_find_function(db,
                            c"like".as_ptr() as *mut i8 as *const i8, n_arg, 1 as u8,
                            0 as u8)
                    };
                { let _ = 0; };
                unsafe { (*p_def).func_flags |= flags as u32 };
                unsafe { (*p_def).func_flags &= !2097152 as u32 };
                break '__c53;
            }
            { let __p = &mut n_arg; let __t = *__p; *__p += 1; __t };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_is_like_function(db: *mut Sqlite3, p_expr_1: &Expr,
    p_is_nocase_1: &mut i32, a_wc_1: *mut i8) -> i32 {
    unsafe {
        let mut p_def: *const FuncDef = core::ptr::null();
        let mut n_expr: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if ((*p_expr_1).x.p_list).is_null() as i32 != 0 { return 0; }
        n_expr = unsafe { (*(*p_expr_1).x.p_list).n_expr };
        { let _ = 0; };
        p_def =
            unsafe {
                sqlite3_find_function(db, (*p_expr_1).u.z_token as *const i8,
                    n_expr, 1 as u8, 0 as u8)
            };
        if p_def == core::ptr::null_mut() ||
                unsafe { (*p_def).func_flags } & 4 as u32 == 0 as u32 {
            return 0;
        }
        unsafe {
            memcpy(a_wc_1 as *mut (),
                unsafe { (*p_def).p_user_data } as *const (), 3 as u64)
        };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if n_expr < 3 {
            unsafe { *a_wc_1.offset(3 as isize) = 0 as i8 };
        } else {
            let p_escape: *const Expr =
                unsafe {
                        (*(unsafe { (*(*p_expr_1).x.p_list).a.as_ptr() } as
                                        *mut ExprListItem).offset(2 as isize)).p_expr
                    } as *const Expr;
            let mut z_escape: *const i8 = core::ptr::null();
            if unsafe { (*p_escape).op } as i32 != 118 { return 0; }
            { let _ = 0; };
            z_escape = unsafe { (*p_escape).u.z_token };
            if unsafe { *z_escape.offset(0 as isize) } as i32 == 0 ||
                    unsafe { *z_escape.offset(1 as isize) } as i32 != 0 {
                return 0;
            }
            if unsafe { *z_escape.offset(0 as isize) } as i32 ==
                    unsafe { *a_wc_1.offset(0 as isize) } as i32 {
                return 0;
            }
            if unsafe { *z_escape.offset(0 as isize) } as i32 ==
                    unsafe { *a_wc_1.offset(1 as isize) } as i32 {
                return 0;
            }
            unsafe {
                *a_wc_1.offset(3 as isize) =
                    unsafe { *z_escape.offset(0 as isize) }
            };
        }
        *p_is_nocase_1 =
            (unsafe { (*p_def).func_flags } & 8 as u32 == 0 as u32) as i32;
        return 1;
    }
}

static len_one: [u32; 1] = [1 as u32];

static mut az_one: [*mut u8; 1] = [c" ".as_ptr() as *mut u8];

static mut az_type: [*const i8; 5] =
    [c"integer".as_ptr() as *const i8, c"real".as_ptr() as *const i8,
            c"text".as_ptr() as *const i8, c"blob".as_ptr() as *const i8,
            c"null".as_ptr() as *const i8];

static mut a_builtin_func: [FuncDef; 108] =
    [FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 262144 | 16384 | 4194304 | 2048 |
                        0) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"implies_nonnull_row".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 262144 | 16384 | 4194304 | 2048 |
                        0) as u32,
                p_user_data: 3 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"expr_compare".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 262144 | 16384 | 4194304 | 2048 |
                        0) as u32,
                p_user_data: 2 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"expr_implies_expr".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 262144 | 16384 | 4194304 | 2048 |
                        0) as u32,
                p_user_data: 4 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"affinity".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 524288 | 2097152) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(load_ext),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"load_extension".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 524288 | 2097152) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(load_ext),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"load_extension".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(compileoptionused_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_compileoption_used".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(compileoptionget_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_compileoption_get".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 1024) as u32,
                p_user_data: 99 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unlikely".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 1024) as u32,
                p_user_data: 99 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"likelihood".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 1024) as u32,
                p_user_data: 99 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"likely".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(trim_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"ltrim".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(trim_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"ltrim".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 2 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(trim_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"rtrim".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 2 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(trim_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"rtrim".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 3 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(trim_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"trim".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 3 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(trim_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"trim".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -3 as i16,
                func_flags: (8388608 | 2048 | 1 | 1 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(minmax_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"min".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 1 * 32 | 4096 | 134217728) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(minmax_step),
                x_finalize: Some(min_max_finalize),
                x_value: Some(min_max_value),
                x_inverse: None,
                z_name: c"min".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -3 as i16,
                func_flags: (8388608 | 2048 | 1 | 1 * 32) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(minmax_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"max".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 1 * 32 | 4096 | 134217728) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(minmax_step),
                x_finalize: Some(min_max_finalize),
                x_value: Some(min_max_value),
                x_inverse: None,
                z_name: c"max".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32 | 128) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(typeof_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"typeof".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32 | 128 | 1048576) as
                    u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(subtype_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"subtype".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32 | 64) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(length_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"length".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32 | 192) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(bytelength_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"octet_length".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(instr_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"instr".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(printf_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"printf".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(printf_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"format".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(unicode_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unicode".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(char_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"char".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(abs_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"abs".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(round_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"round".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(round_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"round".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(upper_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"upper".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(lower_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"lower".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(hex_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"hex".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(unhex_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unhex".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(unhex_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unhex".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -3 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(concat_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"concat".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -4 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(concatws_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"concat_ws".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"ifnull".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(random_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"random".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(random_blob),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"randomblob".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 1 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(nullif_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"nullif".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_version".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(sourceid_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_source_id".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 524288 | 2097152) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(errlog_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_log".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(unistr_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unistr".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(quote_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"quote".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(quote_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unistr_quote".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(last_insert_rowid),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"last_insert_rowid".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(changes),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"changes".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(total_changes),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"total_changes".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(replace_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"replace".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(zeroblob_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"zeroblob".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(substr_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"substr".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(substr_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"substr".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(substr_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"substring".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(substr_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"substring".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(sum_step),
                x_finalize: Some(sum_finalize),
                x_value: Some(sum_finalize),
                x_inverse: Some(sum_inverse),
                z_name: c"sum".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(sum_step),
                x_finalize: Some(total_finalize),
                x_value: Some(total_finalize),
                x_inverse: Some(sum_inverse),
                z_name: c"total".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(sum_step),
                x_finalize: Some(avg_finalize),
                x_value: Some(avg_finalize),
                x_inverse: Some(sum_inverse),
                z_name: c"avg".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 256 | 134217728) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(count_step),
                x_finalize: Some(count_finalize),
                x_value: Some(count_finalize),
                x_inverse: Some(count_inverse),
                z_name: c"count".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 134217728) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(count_step),
                x_finalize: Some(count_finalize),
                x_value: Some(count_finalize),
                x_inverse: Some(count_inverse),
                z_name: c"count".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(group_concat_step),
                x_finalize: Some(group_concat_finalize),
                x_value: Some(group_concat_value),
                x_inverse: Some(group_concat_inverse),
                z_name: c"group_concat".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(group_concat_step),
                x_finalize: Some(group_concat_finalize),
                x_value: Some(group_concat_value),
                x_inverse: Some(group_concat_inverse),
                z_name: c"group_concat".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(group_concat_step),
                x_finalize: Some(group_concat_finalize),
                x_value: Some(group_concat_value),
                x_inverse: Some(group_concat_inverse),
                z_name: c"string_agg".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 2097152 | 33554432) as
                    u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(percent_step),
                x_finalize: Some(percent_final),
                x_value: Some(percent_value),
                x_inverse: Some(percent_inverse),
                z_name: c"median".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 2097152 | 33554432) as
                    u32,
                p_user_data: 2 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(percent_step),
                x_finalize: Some(percent_final),
                x_value: Some(percent_value),
                x_inverse: Some(percent_inverse),
                z_name: c"percentile".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 2097152 | 33554432) as
                    u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(percent_step),
                x_finalize: Some(percent_final),
                x_value: Some(percent_value),
                x_inverse: Some(percent_inverse),
                z_name: c"percentile_cont".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 2097152 | 33554432) as
                    u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(percent_step),
                x_finalize: Some(percent_final),
                x_value: Some(percent_value),
                x_inverse: Some(percent_inverse),
                z_name: c"percentile_disc".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 4 | 8) as u32,
                p_user_data: &raw const glob_info as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(like_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"glob".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 4) as u32,
                p_user_data: &raw const like_info_norm as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(like_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"like".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 2048 | 1 | 4) as u32,
                p_user_data: &raw const like_info_norm as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(like_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"like".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: x_ceil as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ceiling_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"ceil".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: x_ceil as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ceiling_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"ceiling".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: x_floor as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ceiling_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"floor".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: trunc as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ceiling_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"trunc".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(log_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"ln".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(log_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"log".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 1 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(log_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"log10".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 2 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(log_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"log2".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(log_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"log".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: exp as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"exp".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: pow as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math2_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"pow".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: pow as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math2_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"power".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: fmod as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math2_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"mod".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: acos as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"acos".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: asin as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"asin".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: atan as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"atan".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: atan2 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math2_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"atan2".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: cos as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"cos".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: sin as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sin".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: tan as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"tan".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: cosh as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"cosh".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: sinh as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sinh".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: tanh as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"tanh".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: acosh as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"acosh".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: asinh as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"asinh".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: atanh as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"atanh".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: sqrt as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqrt".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: deg_to_rad as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"radians".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: rad_to_deg as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(math1_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"degrees".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 2048 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(pi_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"pi".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 1 | 0 * 32) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(sign_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sign".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -4 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 0) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"coalesce".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -4 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 0) as u32,
                p_user_data: 5 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"iif".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -4 as i16,
                func_flags: (8388608 | 1 | 4194304 | 2048 | 0) as u32,
                p_user_data: 5 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(version_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"if".as_ptr() as *const i8,
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
    fn sqlite3_utf8_read(_: *mut *const u8)
    -> u32;
    static sqlite3_ctype_map: [u8; 0];
    static sqlite3_upper_to_lower: [u8; 0];
    fn strcspn(__s: *const i8, __charset: *const i8)
    -> u64;
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
    fn sqlite3_str_accum_enlarge(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_vdbe_mem_copy(_: *mut Mem, _: *const Mem)
    -> i32;
    fn sqlite3_vdbe_mem_release(p: *mut Mem)
    -> ();
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut Sqlite3, _: *mut i8,
    _: i32, _: i32)
    -> ();
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
    -> i32;
    fn sqlite3_hex_to_int(h: i32)
    -> u8;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn fabs(_: f64)
    -> f64;
    fn sqlite3_add_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_sub_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_str_accum_set_error(_: *mut StrAccum, _: u8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_utf8_char_len(p_data_1: *const i8, n_byte_1: i32)
    -> i32;
    fn ceil(_: f64)
    -> f64;
    fn floor(_: f64)
    -> f64;
    fn log(_: f64)
    -> f64;
    fn log10(_: f64)
    -> f64;
    fn log2(_: f64)
    -> f64;
    fn sqlite3_alter_functions()
    -> ();
    fn sqlite3_register_date_time_functions()
    -> ();
    fn sqlite3_register_json_functions()
    -> ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
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
    fn sqlite3_get_int32(_: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_get_u_int32(_: *const i8, _: *mut u32)
    -> i32;
    fn sqlite3_atoi(_: *const i8)
    -> i32;
    fn sqlite3_utf16_byte_len(p_data_1: *const (), n_byte_1: i32,
    n_char_1: i32)
    -> i32;
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
    fn sqlite3_str_accum_enlarge_if_needed(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_finish(_: *mut StrAccum)
    -> *mut i8;
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
    static sqlite3_small_type_sizes: [u8; 0];
    fn sqlite3_vdbe_error(_: *mut Vdbe, _: *const i8, ...)
    -> ();
    fn sqlite3_vdbe_free_cursor(_: *mut Vdbe, _: *mut VdbeCursor)
    -> ();
    fn sqlite3_vdbe_free_cursor_nn(_: *mut Vdbe, _: *mut VdbeCursor)
    -> ();
    fn sqlite_vdbe_pop_stack(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_handle_moved_cursor(p: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_finish_moveto(_: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_cursor_restore(_: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_serial_type_len(_: u32)
    -> u32;
    fn sqlite3_vdbe_one_byte_serial_type_len(_: u8)
    -> u8;
    fn sqlite3_vdbe_serial_get(_: *const u8, _: u32, _: *mut Mem)
    -> ();
    fn sqlite3_vdbe_delete_aux_data(_: *mut Sqlite3, _: *mut *mut AuxData,
    _: i32, _: i32)
    -> ();
    fn sqlite2_btree_key_compare(_: *mut BtCursor, _: *const (), _: i32,
    _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_idx_key_compare(_: *mut Sqlite3, _: *mut VdbeCursor,
    _: *mut UnpackedRecord, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_idx_rowid(_: *mut Sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_vdbe_exec(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_next_opcode(_: *mut Vdbe, _: *mut Mem, _: i32,
    _: *mut i32, _: *mut i32, _: *mut *mut Op)
    -> i32;
    fn sqlite3_vdbe_display_p4(_: *mut Sqlite3, _: *mut Op)
    -> *mut i8;
    fn sqlite3_vdbe_list(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_halt(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_change_encoding(_: *mut Mem, _: i32)
    -> i32;
    fn sqlite3_vdbe_mem_too_big(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_shallow_copy(_: *mut Mem, _: *const Mem, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_move(_: *mut Mem, _: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_nul_terminate(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_set_str(_: *mut Mem, _: *const i8, _: i64, _: u8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_mem_set_text(_: *mut Mem, _: *const i8, _: i64,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_mem_set_int64(_: *mut Mem, _: i64)
    -> ();
    fn sqlite3_vdbe_mem_set_double(_: *mut Mem, _: f64)
    -> ();
    fn sqlite3_vdbe_mem_set_pointer(_: *mut Mem, _: *mut (), _: *const i8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_vdbe_mem_init(_: *mut Mem, _: *mut Sqlite3, _: u16)
    -> ();
    fn sqlite3_vdbe_mem_set_null(_: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_set_zero_blob(_: *mut Mem, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_set_row_set(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_zero_terminate_if_able(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_make_writeable(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_stringify(_: *mut Mem, _: u8, _: u8)
    -> i32;
    fn sqlite3_int_float_compare(_: i64, _: f64)
    -> i32;
    fn sqlite3_vdbe_int_value(_: *const Mem)
    -> i64;
    fn sqlite3_vdbe_mem_integerify(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_real_value(_: *mut Mem)
    -> f64;
    fn sqlite3_mem_real_value_rc(_: *mut Mem, _: *mut f64)
    -> i32;
    fn sqlite3_vdbe_boolean_value(_: *mut Mem, if_null_1: i32)
    -> i32;
    fn sqlite3_vdbe_integer_affinity(_: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_realify(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_numerify(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_cast(_: *mut Mem, _: u8, _: u8)
    -> i32;
    fn sqlite3_vdbe_mem_from_btree(_: *mut BtCursor, _: u32, _: u32,
    _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_from_btree_zero_offset(_: *mut BtCursor, _: u32,
    _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_release_malloc(p: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_finalize(_: *mut Mem, _: *mut FuncDef)
    -> i32;
    fn sqlite3_vdbe_mem_agg_value(_: *mut Mem, _: *mut Mem, _: *mut FuncDef)
    -> i32;
    fn sqlite3_opcode_name(_: i32)
    -> *const i8;
    fn sqlite3_vdbe_mem_grow(p_mem_1: *mut Mem, n: i32, preserve: i32)
    -> i32;
    fn sqlite3_vdbe_mem_clear_and_resize(p_mem_1: *mut Mem, n: i32)
    -> i32;
    fn sqlite3_vdbe_close_statement(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_frame_mem_del(_: *mut ())
    -> ();
    fn sqlite3_vdbe_frame_delete(_: *mut VdbeFrame)
    -> ();
    fn sqlite3_vdbe_frame_restore(_: *mut VdbeFrame)
    -> i32;
    fn sqlite3_vdbe_transfer_error(p: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_find_index_key(_: *mut BtCursor, _: *mut Index,
    _: *mut UnpackedRecord, _: *mut i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_sorter_init(_: *mut Sqlite3, _: i32, _: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_sorter_reset(_: *mut Sqlite3, _: *mut VdbeSorter)
    -> ();
    fn sqlite3_vdbe_sorter_close(_: *mut Sqlite3, _: *mut VdbeCursor)
    -> ();
    fn sqlite3_vdbe_sorter_rowkey(_: *const VdbeCursor, _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_sorter_next(_: *mut Sqlite3, _: *const VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_sorter_rewind(_: *const VdbeCursor, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_sorter_write(_: *const VdbeCursor, _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_sorter_compare(_: *const VdbeCursor, _: *mut Mem, _: i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_value_list_free(_: *mut ())
    -> ();
    fn sqlite3_vdbe_enter(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_leave(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_check_fk_immediate(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_check_fk_deferred(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_mem_translate(_: *mut Mem, _: u8)
    -> i32;
    fn sqlite3_vdbe_mem_handle_bom(p_mem_1: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_expand_blob(_: *mut Mem)
    -> i32;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn trunc(_: f64)
    -> f64;
    fn exp(_: f64)
    -> f64;
    fn pow(_: f64, _: f64)
    -> f64;
    fn fmod(_: f64, _: f64)
    -> f64;
    fn acos(_: f64)
    -> f64;
    fn asin(_: f64)
    -> f64;
    fn atan(_: f64)
    -> f64;
    fn atan2(_: f64, _: f64)
    -> f64;
    fn cos(_: f64)
    -> f64;
    fn sin(_: f64)
    -> f64;
    fn tan(_: f64)
    -> f64;
    fn cosh(_: f64)
    -> f64;
    fn sinh(_: f64)
    -> f64;
    fn tanh(_: f64)
    -> f64;
    fn acosh(_: f64)
    -> f64;
    fn asinh(_: f64)
    -> f64;
    fn atanh(_: f64)
    -> f64;
    fn sqrt(_: f64)
    -> f64;
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
