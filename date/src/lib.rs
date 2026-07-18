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
type DarwinTimeT = i64;
type TimeT = DarwinTimeT;
#[repr(C)]
#[derive(Copy, Clone)]
struct Tm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
    tm_gmtoff: i64,
    tm_zone: *mut i8,
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
struct DateTime {
    i_jd: Sqlite3Int64,
    y: i32,
    m_1: i32,
    d: i32,
    h: i32,
    m: i32,
    tz: i32,
    s: f64,
    valid_jd: i8,
    valid_ymd: i8,
    valid_hms: i8,
    n_floor: i8,
    _bitfield_1: u32,
}
impl DateTime {
    fn raw_s(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x1u32) as i32 }
    fn set_raw_s(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn is_error(&self) -> i32 { ((self._bitfield_1 >> 1u32) & 0x1u32) as i32 }
    fn set_is_error(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn use_subsec(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_use_subsec(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn is_utc(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_is_utc(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn is_local(&self) -> i32 { ((self._bitfield_1 >> 4u32) & 0x1u32) as i32 }
    fn set_is_local(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
}
extern "C" fn clear_ymd_hms_tz(p: &mut DateTime) -> () {
    (*p).valid_ymd = 0 as i8;
    (*p).valid_hms = 0 as i8;
    (*p).tz = 0;
}
extern "C" fn set_date_time_to_current(context: *mut Sqlite3Context,
    p: *mut DateTime) -> i32 {
    unsafe { (*p).i_jd = unsafe { sqlite3_stmt_current_time(context) } };
    if unsafe { (*p).i_jd } > 0 as i64 {
        unsafe { (*p).valid_jd = 1 as i8 };
        unsafe { (*p).set_is_utc(1 as u32 as u32) };
        unsafe { (*p).set_is_local(0 as u32 as u32) };
        unsafe { clear_ymd_hms_tz(unsafe { &mut *p }) };
        return 0;
    } else { return 1; }
}
extern "C" fn set_raw_date_number(p: &mut DateTime, r: f64) -> () {
    (*p).s = r;
    (*p).set_raw_s(1 as u32 as u32);
    if r >= 0.0 && r < 5373484.5 {
        (*p).i_jd = (r * 86400000.0 + 0.5) as Sqlite3Int64;
        (*p).valid_jd = 1 as i8;
    }
}
extern "C" fn get_digits(mut z_date_1: *const i8, mut z_format_1: *const i8,
    __va: &[*mut i32]) -> i32 {
    unsafe {
        let mut __cursor: u64 = 0 as u64;
        let mut cnt: i32 = 0;
        let mut next_c: i8 = 0 as i8;
        let mut n: i8 = 0 as i8;
        let mut min: i8 = 0 as i8;
        let mut val: i32 = 0;
        let mut max: u16 = 0 as u16;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s1:
                {
                match __state {
                    0 => { __cursor = 0; __state = 3; }
                    2 => { (); __state = 27; }
                    3 => { __state = 4; }
                    4 => { cnt = 0; __state = 5; }
                    5 => { __state = 6; }
                    6 => {
                        n =
                            (unsafe { *z_format_1.offset(0 as isize) } as i32 -
                                    '0' as i32) as i8;
                        __state = 9;
                    }
                    7 => { __state = 2; }
                    8 => {
                        if next_c != 0 { __state = 6; } else { __state = 7; }
                    }
                    9 => {
                        min =
                            (unsafe { *z_format_1.offset(1 as isize) } as i32 -
                                    '0' as i32) as i8;
                        __state = 10;
                    }
                    10 => { val = 0; __state = 11; }
                    11 => { __state = 12; }
                    12 => { { let _ = 0; }; __state = 13; }
                    13 => {
                        max =
                            a_mx[(unsafe { *z_format_1.offset(2 as isize) } as i32 -
                                            'a' as i32) as usize] as u16;
                        __state = 14;
                    }
                    14 => {
                        next_c = unsafe { *z_format_1.offset(3 as isize) } as i8;
                        __state = 15;
                    }
                    15 => { val = 0; __state = 16; }
                    16 => {
                        if { let __p = &mut n; let __t = *__p; *__p -= 1; __t } != 0
                            {
                            __state = 18;
                        } else { __state = 17; }
                    }
                    17 => {
                        if val < min as i32 || val > max as i32 ||
                                next_c as i32 != 0 &&
                                    next_c as i32 != unsafe { *z_date_1 } as i32 {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    18 => {
                        if (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z_date_1 } as u8 as usize)
                                                } as i32 & 4 == 0) as i32 != 0 {
                            __state = 20;
                        } else { __state = 19; }
                    }
                    19 => {
                        val = val * 10 + unsafe { *z_date_1 } as i32 - '0' as i32;
                        __state = 21;
                    }
                    20 => { __state = 2; }
                    21 => {
                        {
                            let __p = &mut z_date_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 16;
                    }
                    22 => {
                        unsafe {
                            *__va[{
                                                let __p = &mut __cursor;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as usize] = val
                        };
                        __state = 24;
                    }
                    23 => { __state = 2; }
                    24 => {
                        {
                            let __p = &mut z_date_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 25;
                    }
                    25 => {
                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                        __state = 26;
                    }
                    26 => {
                        {
                            let __n = 4;
                            let __p = &mut z_format_1;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 8;
                    }
                    27 => { return cnt; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn parse_timezone(mut z_date_1: *const i8, p: &mut DateTime)
    -> i32 {
    unsafe {
        let mut sgn: i32 = 0;
        let mut n_hr: i32 = 0;
        let mut n_mn: i32 = 0;
        let mut c: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s3:
                {
                match __state {
                    0 => { sgn = 0; __state = 3; }
                    2 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_date_1 } as u8 as usize)
                                        } as i32 & 1 != 0 {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_date_1 } as u8 as usize)
                                        } as i32 & 1 != 0 {
                            __state = 7;
                        } else { __state = 6; }
                    }
                    6 => { (*p).tz = 0; __state = 8; }
                    7 => {
                        {
                            let __p = &mut z_date_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 5;
                    }
                    8 => { c = unsafe { *z_date_1 } as i32; __state = 9; }
                    9 => {
                        if c == '-' as i32 { __state = 11; } else { __state = 12; }
                    }
                    10 => {
                        {
                            let __p = &mut z_date_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 20;
                    }
                    11 => { sgn = -1; __state = 10; }
                    12 => {
                        if c == '+' as i32 { __state = 13; } else { __state = 14; }
                    }
                    13 => { sgn = 1; __state = 10; }
                    14 => {
                        if c == 'Z' as i32 || c == 'z' as i32 {
                            __state = 15;
                        } else { __state = 16; }
                    }
                    15 => {
                        {
                            let __p = &mut z_date_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 17;
                    }
                    16 => { return (c != 0) as i32; }
                    17 => { (*p).set_is_local(0 as u32 as u32); __state = 18; }
                    18 => { (*p).set_is_utc(1 as u32 as u32); __state = 19; }
                    19 => { __state = 2; }
                    20 => {
                        if get_digits(z_date_1,
                                    c"20b:20e".as_ptr() as *mut i8 as *const i8,
                                    &[&mut n_hr, &mut n_mn]) != 2 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        {
                            let __n = 5;
                            let __p = &mut z_date_1;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 23;
                    }
                    22 => { return 1; }
                    23 => { (*p).tz = sgn * (n_mn + n_hr * 60); __state = 24; }
                    24 => {
                        if (*p).tz == 0 { __state = 26; } else { __state = 25; }
                    }
                    25 => { __state = 2; }
                    26 => { (*p).set_is_local(0 as u32 as u32); __state = 27; }
                    27 => { (*p).set_is_utc(1 as u32 as u32); __state = 25; }
                    28 => { return (unsafe { *z_date_1 } as i32 != 0) as i32; }
                    29 => {
                        {
                            let __p = &mut z_date_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 2;
                    }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn parse_hh_mm_ss(mut z_date_1: *const i8, p: *mut DateTime)
    -> i32 {
    unsafe {
        let mut h: i32 = 0;
        let mut m: i32 = 0;
        let mut s: i32 = 0;
        let mut ms: f64 = 0.0;
        if get_digits(z_date_1, c"20c:20e".as_ptr() as *mut i8 as *const i8,
                    &[&mut h, &mut m]) != 2 {
            return 1;
        }
        {
            let __n = 5;
            let __p = &mut z_date_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if unsafe { *z_date_1 } as i32 == ':' as i32 {
            {
                let __p = &mut z_date_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            if get_digits(z_date_1, c"20e".as_ptr() as *mut i8 as *const i8,
                        &[&mut s]) != 1 {
                return 1;
            }
            {
                let __n = 2;
                let __p = &mut z_date_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            if unsafe { *z_date_1 } as i32 == '.' as i32 &&
                    unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z_date_1.offset(1 as isize) } as u8
                                                as usize)
                                } as i32 & 4 != 0 {
                let mut r_scale: f64 = 1.0;
                {
                    let __p = &mut z_date_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                while unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z_date_1 } as u8 as usize)
                                } as i32 & 4 != 0 {
                    ms =
                        ms * 10.0 + unsafe { *z_date_1 } as f64 - '0' as i32 as f64;
                    r_scale *= 10.0;
                    {
                        let __p = &mut z_date_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                ms /= r_scale;
                if ms > 0.999 { ms = 0.999; }
            }
        } else { s = 0; }
        unsafe { (*p).valid_jd = 0 as i8 };
        unsafe { (*p).set_raw_s(0 as u32 as u32) };
        unsafe { (*p).valid_hms = 1 as i8 };
        unsafe { (*p).h = h };
        unsafe { (*p).m = m };
        unsafe { (*p).s = s as f64 + ms };
        if parse_timezone(z_date_1, unsafe { &mut *p }) != 0 { return 1; }
        return 0;
    }
}
extern "C" fn compute_floor(p: &mut DateTime) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if (*p).d <= 28 {
            (*p).n_floor = 0 as i8;
        } else if 1 << (*p).m_1 & 5546 != 0 {
            (*p).n_floor = 0 as i8;
        } else if (*p).m_1 != 2 {
            (*p).n_floor = ((*p).d == 31) as i8;
        } else if (*p).y % 4 != 0 || (*p).y % 100 == 0 && (*p).y % 400 != 0 {
            (*p).n_floor = ((*p).d - 28) as i8;
        } else { (*p).n_floor = ((*p).d - 29) as i8; }
    }
}
extern "C" fn datetime_error(p: *mut DateTime) -> () {
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<DateTime>() as u64)
    };
    unsafe { (*p).set_is_error(1 as u32 as u32) };
}
extern "C" fn compute_jd(p: *mut DateTime) -> () {
    unsafe {
        let mut y: i32 = 0;
        let mut m: i32 = 0;
        let mut d: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut x1: i32 = 0;
        let mut x2: i32 = 0;
        if unsafe { (*p).valid_jd } != 0 { return; }
        if unsafe { (*p).valid_ymd } != 0 {
            y = unsafe { (*p).y };
            m = unsafe { (*p).m_1 };
            d = unsafe { (*p).d };
        } else { y = 2000; m = 1; d = 1; }
        if y < -4713 || y > 9999 || unsafe { (*p).raw_s() } != 0 {
            datetime_error(p);
            return;
        }
        if m <= 2 {
            { let __p = &mut y; let __t = *__p; *__p -= 1; __t };
            m += 12;
        }
        a = (y + 4800) / 100;
        b = 38 - a + a / 4;
        x1 = 36525 * (y + 4716) / 100;
        x2 = 306001 * (m + 1) / 10000;
        unsafe {
            (*p).i_jd =
                (((x1 + x2 + d + b) as f64 - 1524.5) * 86400000 as f64) as
                    Sqlite3Int64
        };
        unsafe { (*p).valid_jd = 1 as i8 };
        if unsafe { (*p).valid_hms } != 0 {
            unsafe {
                (*p).i_jd +=
                    (unsafe { (*p).h } * 3600000 + unsafe { (*p).m } * 60000) as
                            Sqlite3Int64 +
                        (unsafe { (*p).s } * 1000 as f64 + 0.5) as Sqlite3Int64
            };
            if unsafe { (*p).tz } != 0 {
                unsafe {
                    (*p).i_jd -= (unsafe { (*p).tz } * 60000) as Sqlite3Int64
                };
                unsafe { (*p).valid_ymd = 0 as i8 };
                unsafe { (*p).valid_hms = 0 as i8 };
                unsafe { (*p).tz = 0 };
                unsafe { (*p).set_is_utc(1 as u32 as u32) };
                unsafe { (*p).set_is_local(0 as u32 as u32) };
            }
        }
    }
}
extern "C" fn parse_yyyy_mm_dd(mut z_date_1: *const i8, p: *mut DateTime)
    -> i32 {
    unsafe {
        unsafe {
            let mut y: i32 = 0;
            let mut m: i32 = 0;
            let mut d: i32 = 0;
            let mut neg: i32 = 0;
            if unsafe { *z_date_1.offset(0 as isize) } as i32 == '-' as i32 {
                {
                    let __p = &mut z_date_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                neg = 1;
            } else { neg = 0; }
            if get_digits(z_date_1,
                        c"40f-21a-21d".as_ptr() as *mut i8 as *const i8,
                        &[&mut y, &mut m, &mut d]) != 3 {
                return 1;
            }
            {
                let __n = 10;
                let __p = &mut z_date_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            while unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z_date_1 } as u8 as usize)
                                } as i32 & 1 != 0 ||
                    'T' as i32 == unsafe { *(z_date_1 as *mut u8) } as i32 {
                {
                    let __p = &mut z_date_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if parse_hh_mm_ss(z_date_1, p) == 0
                {} else if unsafe { *z_date_1 } as i32 == 0 {
                unsafe { (*p).valid_hms = 0 as i8 };
            } else { return 1; }
            unsafe { (*p).valid_jd = 0 as i8 };
            unsafe { (*p).valid_ymd = 1 as i8 };
            unsafe { (*p).y = if neg != 0 { -y } else { y } };
            unsafe { (*p).m_1 = m };
            unsafe { (*p).d = d };
            compute_floor(unsafe { &mut *p });
            if unsafe { (*p).tz } != 0 { compute_jd(p); }
            return 0;
        }
    }
}
extern "C" fn parse_date_or_time(context: *mut Sqlite3Context,
    z_date_1: *const i8, p: *mut DateTime) -> i32 {
    let mut r: f64 = 0.0;
    if parse_yyyy_mm_dd(z_date_1, p) == 0 {
        return 0;
    } else if parse_hh_mm_ss(z_date_1, p) == 0 {
        return 0;
    } else if unsafe {
                    sqlite3_str_i_cmp(z_date_1,
                        c"now".as_ptr() as *mut i8 as *const i8)
                } == 0 && unsafe { sqlite3_not_pure_func(context) } != 0 {
        return set_date_time_to_current(context, p);
    } else if unsafe { sqlite3_ato_f(z_date_1, &mut r) } > 0 {
        set_raw_date_number(unsafe { &mut *p }, r);
        return 0;
    } else if (unsafe {
                        sqlite3_str_i_cmp(z_date_1,
                            c"subsec".as_ptr() as *mut i8 as *const i8)
                    } == 0 ||
                unsafe {
                        sqlite3_str_i_cmp(z_date_1,
                            c"subsecond".as_ptr() as *mut i8 as *const i8)
                    } == 0) && unsafe { sqlite3_not_pure_func(context) } != 0 {
        unsafe { (*p).set_use_subsec(1 as u32 as u32) };
        return set_date_time_to_current(context, p);
    }
    return 1;
}
extern "C" fn auto_adjust_date(p: *mut DateTime) -> () {
    if (unsafe { (*p).raw_s() } == 0) as i32 != 0 ||
            unsafe { (*p).valid_jd } != 0 {
        unsafe { (*p).set_raw_s(0 as u32 as u32) };
    } else if unsafe { (*p).s } >= (-21086676 as i64 * 10000 as i64) as f64 &&
            unsafe { (*p).s } <=
                (25340230 as i64 * 10000 as i64 + 799 as i64) as f64 {
        let r: f64 = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
        clear_ymd_hms_tz(unsafe { &mut *p });
        unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
        unsafe { (*p).valid_jd = 1 as i8 };
        unsafe { (*p).set_raw_s(0 as u32 as u32) };
    }
}
extern "C" fn valid_julian_day(i_jd_1: Sqlite3Int64) -> i32 {
    return (i_jd_1 >= 0 as i64 &&
                i_jd_1 <= (108096 as i64) << 32 | 275971583 as i64) as i32;
}
extern "C" fn compute_ymd(p: *mut DateTime) -> () {
    unsafe {
        let mut z: i32 = 0;
        let mut alpha: i32 = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut x1: i32 = 0;
        if unsafe { (*p).valid_ymd } != 0 { return; }
        if (unsafe { (*p).valid_jd } == 0) as i32 != 0 {
            unsafe { (*p).y = 2000 };
            unsafe { (*p).m_1 = 1 };
            unsafe { (*p).d = 1 };
        } else if (valid_julian_day(unsafe { (*p).i_jd }) == 0) as i32 != 0 {
            datetime_error(p);
            return;
        } else {
            z =
                ((unsafe { (*p).i_jd } + 43200000 as Sqlite3Int64) /
                        86400000 as Sqlite3Int64) as i32;
            alpha = ((z as f64 + 32044.75) / 36524.25) as i32 - 52;
            a = z + 1 + alpha - (alpha + 100) / 4 + 25;
            b = a + 1524;
            c = ((b as f64 - 122.1) / 365.25) as i32;
            d = 36525 * (c & 32767) / 100;
            e = ((b - d) as f64 / 30.6001) as i32;
            x1 = (30.6001 * e as f64) as i32;
            unsafe { (*p).d = b - d - x1 };
            unsafe { (*p).m_1 = if e < 14 { e - 1 } else { e - 13 } };
            unsafe {
                (*p).y =
                    if unsafe { (*p).m_1 } > 2 { c - 4716 } else { c - 4715 }
            };
        }
        unsafe { (*p).valid_ymd = 1 as i8 };
    }
}
extern "C" fn compute_hms(p: *mut DateTime) -> () {
    let mut day_ms: i32 = 0;
    let mut day_min: i32 = 0;
    if unsafe { (*p).valid_hms } != 0 { return; }
    compute_jd(p);
    day_ms =
        ((unsafe { (*p).i_jd } + 43200000 as Sqlite3Int64) %
                86400000 as Sqlite3Int64) as i32;
    unsafe { (*p).s = (day_ms % 60000) as f64 / 1000.0 };
    day_min = day_ms / 60000;
    unsafe { (*p).m = day_min % 60 };
    unsafe { (*p).h = day_min / 60 };
    unsafe { (*p).set_raw_s(0 as u32 as u32) };
    unsafe { (*p).valid_hms = 1 as i8 };
}
extern "C" fn compute_ymd_hms(p: *mut DateTime) -> () {
    compute_ymd(p);
    compute_hms(p);
}
extern "C" fn os_localtime(t: *const TimeT, p_tm_1: *mut Tm) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if sqlite3Config.b_localtime_fault != 0 {
            if sqlite3Config.x_alt_localtime.is_some() {
                return unsafe {
                        sqlite3Config.x_alt_localtime.unwrap()(t as *const (),
                            p_tm_1 as *mut ())
                    };
            } else { return 1; }
        }
        rc =
            (unsafe { localtime_r(t as *const TimeT, p_tm_1) } ==
                    core::ptr::null_mut()) as i32;
        return rc;
    }
}
extern "C" fn to_localtime(p: *mut DateTime, p_ctx_1: *mut Sqlite3Context)
    -> i32 {
    unsafe {
        let mut t: TimeT = 0 as TimeT;
        let mut s_local: Tm = unsafe { core::mem::zeroed() };
        let mut i_year_diff: i32 = 0;
        unsafe {
            memset(&raw mut s_local as *mut (), 0,
                core::mem::size_of::<Tm>() as u64)
        };
        compute_jd(p);
        if unsafe { (*p).i_jd } < 2108667600 as i64 * 100000 as i64 ||
                unsafe { (*p).i_jd } > 2130141456 as i64 * 100000 as i64 {
            let mut x: DateTime = unsafe { core::ptr::read(p) };
            compute_ymd_hms(&mut x);
            i_year_diff = 2000 + x.y % 4 - x.y;
            x.y += i_year_diff;
            x.valid_jd = 0 as i8;
            compute_jd(&mut x);
            t =
                (x.i_jd / 1000 as Sqlite3Int64 -
                        21086676 as i64 * 10000 as i64) as TimeT;
        } else {
            i_year_diff = 0;
            t =
                (unsafe { (*p).i_jd } / 1000 as Sqlite3Int64 -
                        21086676 as i64 * 10000 as i64) as TimeT;
        }
        if os_localtime(&raw mut t as *const TimeT, &mut s_local) != 0 {
            unsafe {
                sqlite3_result_error(p_ctx_1,
                    c"local time unavailable".as_ptr() as *mut i8 as *const i8,
                    -1)
            };
            return 1;
        }
        unsafe { (*p).y = s_local.tm_year + 1900 - i_year_diff };
        unsafe { (*p).m_1 = s_local.tm_mon + 1 };
        unsafe { (*p).d = s_local.tm_mday };
        unsafe { (*p).h = s_local.tm_hour };
        unsafe { (*p).m = s_local.tm_min };
        unsafe {
            (*p).s =
                s_local.tm_sec as f64 +
                    (unsafe { (*p).i_jd } % 1000 as Sqlite3Int64) as f64 * 0.001
        };
        unsafe { (*p).valid_ymd = 1 as i8 };
        unsafe { (*p).valid_hms = 1 as i8 };
        unsafe { (*p).valid_jd = 0 as i8 };
        unsafe { (*p).set_raw_s(0 as u32 as u32) };
        unsafe { (*p).tz = 0 };
        unsafe { (*p).set_is_error(0 as u32 as u32) };
        return 0;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AnonS0 {
    n_name: u8,
    z_name: [i8; 7],
    r_limit: f32,
    r_xform: f32,
}
static a_xform_type: [AnonS0; 6] =
    [AnonS0 {
                n_name: 6 as u8,
                z_name: [115 as i8, 101 as i8, 99 as i8, 111 as i8, 110 as i8,
                        100 as i8, 0 as i8],
                r_limit: 464270000000000.0,
                r_xform: 1.0,
            },
            AnonS0 {
                n_name: 6 as u8,
                z_name: [109 as i8, 105 as i8, 110 as i8, 117 as i8,
                        116 as i8, 101 as i8, 0 as i8],
                r_limit: 7737900000000.0,
                r_xform: 60.0,
            },
            AnonS0 {
                n_name: 4 as u8,
                z_name: [104 as i8, 111 as i8, 117 as i8, 114 as i8, 0 as i8,
                        0 as i8, 0 as i8],
                r_limit: 128970000000.0,
                r_xform: 3600.0,
            },
            AnonS0 {
                n_name: 3 as u8,
                z_name: [100 as i8, 97 as i8, 121 as i8, 0 as i8, 0 as i8,
                        0 as i8, 0 as i8],
                r_limit: 5373485.0,
                r_xform: 86400.0,
            },
            AnonS0 {
                n_name: 5 as u8,
                z_name: [109 as i8, 111 as i8, 110 as i8, 116 as i8,
                        104 as i8, 0 as i8, 0 as i8],
                r_limit: 176546.0,
                r_xform: 2592000.0,
            },
            AnonS0 {
                n_name: 4 as u8,
                z_name: [121 as i8, 101 as i8, 97 as i8, 114 as i8, 0 as i8,
                        0 as i8, 0 as i8],
                r_limit: 14713.0,
                r_xform: 31536000.0,
            }];
extern "C" fn parse_modifier(p_ctx_1: *mut Sqlite3Context, mut z: *const i8,
    mut n: i32, p: *mut DateTime, idx: i32) -> i32 {
    unsafe {
        unsafe {
            let mut rc: i32 = 1;
            let mut r: f64 = 0.0;
            '__s6:
                {
                match unsafe {
                        *(sqlite3_upper_to_lower.as_ptr() as
                                    *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                    usize)
                    } {
                    97 => {
                        {
                            if unsafe {
                                        sqlite3_stricmp(z, c"auto".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                if idx > 1 { return 1; }
                                auto_adjust_date(p);
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"ceiling".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                                unsafe { (*p).n_floor = 0 as i8 };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"floor".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                compute_jd(p);
                                unsafe {
                                    (*p).i_jd -=
                                        (unsafe { (*p).n_floor } as i32 * 86400000) as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"julianday".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                if idx > 1 { return 1; }
                                if unsafe { (*p).valid_jd } != 0 &&
                                        unsafe { (*p).raw_s() } != 0 {
                                    rc = 0;
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                }
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"localtime".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                rc =
                                    if unsafe { (*p).is_local() } != 0 {
                                        0
                                    } else { to_localtime(p, p_ctx_1) };
                                unsafe { (*p).set_is_utc(0 as u32 as u32) };
                                unsafe { (*p).set_is_local(1 as u32 as u32) };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"unixepoch".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p).raw_s() } != 0 {
                                if idx > 1 { return 1; }
                                r = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
                                if r >= 0.0 && r < 464269060800000.0 {
                                    clear_ymd_hms_tz(unsafe { &mut *p });
                                    unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                    rc = 0;
                                }
                            } else if unsafe {
                                            sqlite3_stricmp(z, c"utc".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                if unsafe { (*p).is_utc() } as i32 == 0 {
                                    let mut i_orig_jd: i64 = 0 as i64;
                                    let mut i_guess: i64 = 0 as i64;
                                    let mut cnt: i32 = 0;
                                    let mut i_err: i64 = 0 as i64;
                                    compute_jd(p);
                                    i_guess = { i_orig_jd = unsafe { (*p).i_jd }; i_orig_jd };
                                    i_err = 0 as i64;
                                    '__b7: loop {
                                        '__c7: loop {
                                            let mut new: DateTime = unsafe { core::mem::zeroed() };
                                            unsafe {
                                                memset(&raw mut new as *mut (), 0,
                                                    core::mem::size_of::<DateTime>() as u64)
                                            };
                                            i_guess -= i_err;
                                            new.i_jd = i_guess;
                                            new.valid_jd = 1 as i8;
                                            rc = to_localtime(&mut new, p_ctx_1);
                                            if rc != 0 { return rc; }
                                            compute_jd(&mut new);
                                            i_err = new.i_jd - i_orig_jd;
                                            break '__c7;
                                        }
                                        if !(i_err != 0 &&
                                                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 3)
                                            {
                                            break '__b7;
                                        }
                                    }
                                    unsafe {
                                        memset(p as *mut (), 0,
                                            core::mem::size_of::<DateTime>() as u64)
                                    };
                                    unsafe { (*p).i_jd = i_guess };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_is_utc(1 as u32 as u32) };
                                    unsafe { (*p).set_is_local(0 as u32 as u32) };
                                }
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    99 => {
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"ceiling".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                                unsafe { (*p).n_floor = 0 as i8 };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"floor".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                compute_jd(p);
                                unsafe {
                                    (*p).i_jd -=
                                        (unsafe { (*p).n_floor } as i32 * 86400000) as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"julianday".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                if idx > 1 { return 1; }
                                if unsafe { (*p).valid_jd } != 0 &&
                                        unsafe { (*p).raw_s() } != 0 {
                                    rc = 0;
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                }
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"localtime".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                rc =
                                    if unsafe { (*p).is_local() } != 0 {
                                        0
                                    } else { to_localtime(p, p_ctx_1) };
                                unsafe { (*p).set_is_utc(0 as u32 as u32) };
                                unsafe { (*p).set_is_local(1 as u32 as u32) };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"unixepoch".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p).raw_s() } != 0 {
                                if idx > 1 { return 1; }
                                r = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
                                if r >= 0.0 && r < 464269060800000.0 {
                                    clear_ymd_hms_tz(unsafe { &mut *p });
                                    unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                    rc = 0;
                                }
                            } else if unsafe {
                                            sqlite3_stricmp(z, c"utc".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                if unsafe { (*p).is_utc() } as i32 == 0 {
                                    let mut i_orig_jd: i64 = 0 as i64;
                                    let mut i_guess: i64 = 0 as i64;
                                    let mut cnt: i32 = 0;
                                    let mut i_err: i64 = 0 as i64;
                                    compute_jd(p);
                                    i_guess = { i_orig_jd = unsafe { (*p).i_jd }; i_orig_jd };
                                    i_err = 0 as i64;
                                    '__b7: loop {
                                        '__c7: loop {
                                            let mut new: DateTime = unsafe { core::mem::zeroed() };
                                            unsafe {
                                                memset(&raw mut new as *mut (), 0,
                                                    core::mem::size_of::<DateTime>() as u64)
                                            };
                                            i_guess -= i_err;
                                            new.i_jd = i_guess;
                                            new.valid_jd = 1 as i8;
                                            rc = to_localtime(&mut new, p_ctx_1);
                                            if rc != 0 { return rc; }
                                            compute_jd(&mut new);
                                            i_err = new.i_jd - i_orig_jd;
                                            break '__c7;
                                        }
                                        if !(i_err != 0 &&
                                                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 3)
                                            {
                                            break '__b7;
                                        }
                                    }
                                    unsafe {
                                        memset(p as *mut (), 0,
                                            core::mem::size_of::<DateTime>() as u64)
                                    };
                                    unsafe { (*p).i_jd = i_guess };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_is_utc(1 as u32 as u32) };
                                    unsafe { (*p).set_is_local(0 as u32 as u32) };
                                }
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    102 => {
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"floor".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                compute_jd(p);
                                unsafe {
                                    (*p).i_jd -=
                                        (unsafe { (*p).n_floor } as i32 * 86400000) as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"julianday".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                if idx > 1 { return 1; }
                                if unsafe { (*p).valid_jd } != 0 &&
                                        unsafe { (*p).raw_s() } != 0 {
                                    rc = 0;
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                }
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"localtime".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                rc =
                                    if unsafe { (*p).is_local() } != 0 {
                                        0
                                    } else { to_localtime(p, p_ctx_1) };
                                unsafe { (*p).set_is_utc(0 as u32 as u32) };
                                unsafe { (*p).set_is_local(1 as u32 as u32) };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"unixepoch".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p).raw_s() } != 0 {
                                if idx > 1 { return 1; }
                                r = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
                                if r >= 0.0 && r < 464269060800000.0 {
                                    clear_ymd_hms_tz(unsafe { &mut *p });
                                    unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                    rc = 0;
                                }
                            } else if unsafe {
                                            sqlite3_stricmp(z, c"utc".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                if unsafe { (*p).is_utc() } as i32 == 0 {
                                    let mut i_orig_jd: i64 = 0 as i64;
                                    let mut i_guess: i64 = 0 as i64;
                                    let mut cnt: i32 = 0;
                                    let mut i_err: i64 = 0 as i64;
                                    compute_jd(p);
                                    i_guess = { i_orig_jd = unsafe { (*p).i_jd }; i_orig_jd };
                                    i_err = 0 as i64;
                                    '__b7: loop {
                                        '__c7: loop {
                                            let mut new: DateTime = unsafe { core::mem::zeroed() };
                                            unsafe {
                                                memset(&raw mut new as *mut (), 0,
                                                    core::mem::size_of::<DateTime>() as u64)
                                            };
                                            i_guess -= i_err;
                                            new.i_jd = i_guess;
                                            new.valid_jd = 1 as i8;
                                            rc = to_localtime(&mut new, p_ctx_1);
                                            if rc != 0 { return rc; }
                                            compute_jd(&mut new);
                                            i_err = new.i_jd - i_orig_jd;
                                            break '__c7;
                                        }
                                        if !(i_err != 0 &&
                                                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 3)
                                            {
                                            break '__b7;
                                        }
                                    }
                                    unsafe {
                                        memset(p as *mut (), 0,
                                            core::mem::size_of::<DateTime>() as u64)
                                    };
                                    unsafe { (*p).i_jd = i_guess };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_is_utc(1 as u32 as u32) };
                                    unsafe { (*p).set_is_local(0 as u32 as u32) };
                                }
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    106 => {
                        {
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"julianday".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                if idx > 1 { return 1; }
                                if unsafe { (*p).valid_jd } != 0 &&
                                        unsafe { (*p).raw_s() } != 0 {
                                    rc = 0;
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                }
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"localtime".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                rc =
                                    if unsafe { (*p).is_local() } != 0 {
                                        0
                                    } else { to_localtime(p, p_ctx_1) };
                                unsafe { (*p).set_is_utc(0 as u32 as u32) };
                                unsafe { (*p).set_is_local(1 as u32 as u32) };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"unixepoch".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p).raw_s() } != 0 {
                                if idx > 1 { return 1; }
                                r = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
                                if r >= 0.0 && r < 464269060800000.0 {
                                    clear_ymd_hms_tz(unsafe { &mut *p });
                                    unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                    rc = 0;
                                }
                            } else if unsafe {
                                            sqlite3_stricmp(z, c"utc".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                if unsafe { (*p).is_utc() } as i32 == 0 {
                                    let mut i_orig_jd: i64 = 0 as i64;
                                    let mut i_guess: i64 = 0 as i64;
                                    let mut cnt: i32 = 0;
                                    let mut i_err: i64 = 0 as i64;
                                    compute_jd(p);
                                    i_guess = { i_orig_jd = unsafe { (*p).i_jd }; i_orig_jd };
                                    i_err = 0 as i64;
                                    '__b7: loop {
                                        '__c7: loop {
                                            let mut new: DateTime = unsafe { core::mem::zeroed() };
                                            unsafe {
                                                memset(&raw mut new as *mut (), 0,
                                                    core::mem::size_of::<DateTime>() as u64)
                                            };
                                            i_guess -= i_err;
                                            new.i_jd = i_guess;
                                            new.valid_jd = 1 as i8;
                                            rc = to_localtime(&mut new, p_ctx_1);
                                            if rc != 0 { return rc; }
                                            compute_jd(&mut new);
                                            i_err = new.i_jd - i_orig_jd;
                                            break '__c7;
                                        }
                                        if !(i_err != 0 &&
                                                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 3)
                                            {
                                            break '__b7;
                                        }
                                    }
                                    unsafe {
                                        memset(p as *mut (), 0,
                                            core::mem::size_of::<DateTime>() as u64)
                                    };
                                    unsafe { (*p).i_jd = i_guess };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_is_utc(1 as u32 as u32) };
                                    unsafe { (*p).set_is_local(0 as u32 as u32) };
                                }
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    108 => {
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"localtime".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                rc =
                                    if unsafe { (*p).is_local() } != 0 {
                                        0
                                    } else { to_localtime(p, p_ctx_1) };
                                unsafe { (*p).set_is_utc(0 as u32 as u32) };
                                unsafe { (*p).set_is_local(1 as u32 as u32) };
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"unixepoch".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p).raw_s() } != 0 {
                                if idx > 1 { return 1; }
                                r = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
                                if r >= 0.0 && r < 464269060800000.0 {
                                    clear_ymd_hms_tz(unsafe { &mut *p });
                                    unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                    rc = 0;
                                }
                            } else if unsafe {
                                            sqlite3_stricmp(z, c"utc".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                if unsafe { (*p).is_utc() } as i32 == 0 {
                                    let mut i_orig_jd: i64 = 0 as i64;
                                    let mut i_guess: i64 = 0 as i64;
                                    let mut cnt: i32 = 0;
                                    let mut i_err: i64 = 0 as i64;
                                    compute_jd(p);
                                    i_guess = { i_orig_jd = unsafe { (*p).i_jd }; i_orig_jd };
                                    i_err = 0 as i64;
                                    '__b7: loop {
                                        '__c7: loop {
                                            let mut new: DateTime = unsafe { core::mem::zeroed() };
                                            unsafe {
                                                memset(&raw mut new as *mut (), 0,
                                                    core::mem::size_of::<DateTime>() as u64)
                                            };
                                            i_guess -= i_err;
                                            new.i_jd = i_guess;
                                            new.valid_jd = 1 as i8;
                                            rc = to_localtime(&mut new, p_ctx_1);
                                            if rc != 0 { return rc; }
                                            compute_jd(&mut new);
                                            i_err = new.i_jd - i_orig_jd;
                                            break '__c7;
                                        }
                                        if !(i_err != 0 &&
                                                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 3)
                                            {
                                            break '__b7;
                                        }
                                    }
                                    unsafe {
                                        memset(p as *mut (), 0,
                                            core::mem::size_of::<DateTime>() as u64)
                                    };
                                    unsafe { (*p).i_jd = i_guess };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_is_utc(1 as u32 as u32) };
                                    unsafe { (*p).set_is_local(0 as u32 as u32) };
                                }
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    117 => {
                        {
                            if unsafe {
                                            sqlite3_stricmp(z,
                                                c"unixepoch".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p).raw_s() } != 0 {
                                if idx > 1 { return 1; }
                                r = unsafe { (*p).s } * 1000.0 + 210866760000000.0;
                                if r >= 0.0 && r < 464269060800000.0 {
                                    clear_ymd_hms_tz(unsafe { &mut *p });
                                    unsafe { (*p).i_jd = (r + 0.5) as Sqlite3Int64 };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_raw_s(0 as u32 as u32) };
                                    rc = 0;
                                }
                            } else if unsafe {
                                            sqlite3_stricmp(z, c"utc".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { sqlite3_not_pure_func(p_ctx_1) } != 0 {
                                if unsafe { (*p).is_utc() } as i32 == 0 {
                                    let mut i_orig_jd: i64 = 0 as i64;
                                    let mut i_guess: i64 = 0 as i64;
                                    let mut cnt: i32 = 0;
                                    let mut i_err: i64 = 0 as i64;
                                    compute_jd(p);
                                    i_guess = { i_orig_jd = unsafe { (*p).i_jd }; i_orig_jd };
                                    i_err = 0 as i64;
                                    '__b7: loop {
                                        '__c7: loop {
                                            let mut new: DateTime = unsafe { core::mem::zeroed() };
                                            unsafe {
                                                memset(&raw mut new as *mut (), 0,
                                                    core::mem::size_of::<DateTime>() as u64)
                                            };
                                            i_guess -= i_err;
                                            new.i_jd = i_guess;
                                            new.valid_jd = 1 as i8;
                                            rc = to_localtime(&mut new, p_ctx_1);
                                            if rc != 0 { return rc; }
                                            compute_jd(&mut new);
                                            i_err = new.i_jd - i_orig_jd;
                                            break '__c7;
                                        }
                                        if !(i_err != 0 &&
                                                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 3)
                                            {
                                            break '__b7;
                                        }
                                    }
                                    unsafe {
                                        memset(p as *mut (), 0,
                                            core::mem::size_of::<DateTime>() as u64)
                                    };
                                    unsafe { (*p).i_jd = i_guess };
                                    unsafe { (*p).valid_jd = 1 as i8 };
                                    unsafe { (*p).set_is_utc(1 as u32 as u32) };
                                    unsafe { (*p).set_is_local(0 as u32 as u32) };
                                }
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    119 => {
                        {
                            if unsafe {
                                                        sqlite3_strnicmp(z,
                                                            c"weekday ".as_ptr() as *mut i8 as *const i8, 8)
                                                    } == 0 &&
                                                unsafe {
                                                        sqlite3_ato_f(unsafe { &*z.offset(8 as isize) }, &mut r)
                                                    } > 0 && r >= 0.0 && r < 7.0 &&
                                    { n = r as i32; n } as f64 == r {
                                let mut z: Sqlite3Int64 = 0 as Sqlite3Int64;
                                compute_ymd_hms(p);
                                unsafe { (*p).tz = 0 };
                                unsafe { (*p).valid_jd = 0 as i8 };
                                compute_jd(p);
                                z =
                                    (unsafe { (*p).i_jd } + 129600000 as Sqlite3Int64) /
                                            86400000 as Sqlite3Int64 % 7 as Sqlite3Int64;
                                if z > n as i64 { z -= 7 as Sqlite3Int64; }
                                unsafe {
                                    (*p).i_jd +=
                                        (n as Sqlite3Int64 - z) * 86400000 as Sqlite3Int64
                                };
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    115 => {
                        {
                            if unsafe {
                                        sqlite3_strnicmp(z,
                                            c"start of ".as_ptr() as *mut i8 as *const i8, 9)
                                    } != 0 {
                                if unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsec".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z,
                                                    c"subsecond".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                    unsafe { (*p).set_use_subsec(1 as u32 as u32) };
                                    rc = 0;
                                }
                                break '__s6;
                            }
                            if (unsafe { (*p).valid_jd } == 0) as i32 != 0 &&
                                        (unsafe { (*p).valid_ymd } == 0) as i32 != 0 &&
                                    (unsafe { (*p).valid_hms } == 0) as i32 != 0 {
                                break '__s6;
                            }
                            {
                                let __n = 9;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            compute_ymd(p);
                            unsafe { (*p).valid_hms = 1 as i8 };
                            unsafe {
                                (*p).h = { unsafe { (*p).m = 0 }; unsafe { (*p).m } }
                            };
                            unsafe { (*p).s = 0.0 };
                            unsafe { (*p).set_raw_s(0 as u32 as u32) };
                            unsafe { (*p).tz = 0 };
                            unsafe { (*p).valid_jd = 0 as i8 };
                            if unsafe {
                                        sqlite3_stricmp(z,
                                            c"month".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"year".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe { (*p).m_1 = 1 };
                                unsafe { (*p).d = 1 };
                                rc = 0;
                            } else if unsafe {
                                        sqlite3_stricmp(z, c"day".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                rc = 0;
                            }
                            break '__s6;
                        }
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    43 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    45 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    48 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    49 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    50 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    51 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    52 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    53 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    54 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    55 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    56 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    57 => {
                        {
                            let mut r_rounder: f64 = 0.0;
                            let mut i: i32 = 0;
                            let mut rx: i32 = 0;
                            let mut y: i32 = 0;
                            let mut m: i32 = 0;
                            let mut d: i32 = 0;
                            let mut h: i32 = 0;
                            let mut m: i32 = 0;
                            let mut x: i32 = 0;
                            let mut z2: *const i8 = z;
                            let mut z_copy: *mut i8 = core::ptr::null_mut();
                            let db: *mut Sqlite3 =
                                unsafe { sqlite3_context_db_handle(p_ctx_1) };
                            let z0: i8 = unsafe { *z.offset(0 as isize) } as i8;
                            {
                                n = 1;
                                '__b8: loop {
                                    if !(unsafe { *z.offset(n as isize) } != 0) { break '__b8; }
                                    '__c8: loop {
                                        if unsafe { *z.offset(n as isize) } as i32 == ':' as i32 {
                                            break '__b8;
                                        }
                                        if unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(n as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 1 != 0 {
                                            break '__b8;
                                        }
                                        if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                            if n == 5 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"40f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                            if n == 6 &&
                                                    get_digits(unsafe { &*z.offset(1 as isize) },
                                                            c"50f".as_ptr() as *mut i8 as *const i8, &[&mut y]) == 1 {
                                                break '__b8;
                                            }
                                        }
                                        break '__c8;
                                    }
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            z_copy = unsafe { sqlite3_db_str_n_dup(db, z, n as u64) };
                            if z_copy == core::ptr::null_mut() { break '__s6; }
                            rx =
                                (unsafe { sqlite3_ato_f(z_copy as *const i8, &mut r) } <= 0)
                                    as i32;
                            unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
                            if rx != 0 { { let _ = 0; }; break '__s6; }
                            if unsafe { *z.offset(n as isize) } as i32 == '-' as i32 {
                                if z0 as i32 != '+' as i32 && z0 as i32 != '-' as i32 {
                                    break '__s6;
                                }
                                if n == 5 {
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"40f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                } else {
                                    { let _ = 0; };
                                    if get_digits(unsafe { &*z.offset(1 as isize) },
                                                c"50f-20a-20d".as_ptr() as *mut i8 as *const i8,
                                                &[&mut y, &mut m, &mut d]) != 3 {
                                        break '__s6;
                                    }
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                if m >= 12 { break '__s6; }
                                if d >= 31 { break '__s6; }
                                compute_ymd_hms(p);
                                unsafe { (*p).valid_jd = 0 as i8 };
                                if z0 as i32 == '-' as i32 {
                                    unsafe { (*p).y -= y };
                                    unsafe { (*p).m_1 -= m };
                                    d = -d;
                                } else { unsafe { (*p).y += y }; unsafe { (*p).m_1 += m }; }
                                x =
                                    if unsafe { (*p).m_1 } > 0 {
                                        (unsafe { (*p).m_1 } - 1) / 12
                                    } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                unsafe { (*p).y += x };
                                unsafe { (*p).m_1 -= x * 12 };
                                compute_floor(unsafe { &mut *p });
                                compute_jd(p);
                                unsafe { (*p).valid_hms = 0 as i8 };
                                unsafe { (*p).valid_ymd = 0 as i8 };
                                unsafe { (*p).i_jd += d as i64 * 86400000 as i64 };
                                if unsafe { *z.offset(11 as isize) } as i32 == 0 {
                                    rc = 0;
                                    break '__s6;
                                }
                                if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(11 as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 1 != 0 &&
                                        get_digits(unsafe { &*z.offset(12 as isize) },
                                                c"20c:20e".as_ptr() as *mut i8 as *const i8,
                                                &[&mut h, &mut m]) == 2 {
                                    z2 = unsafe { z.offset(12 as isize) };
                                    n = 2;
                                } else { break '__s6; }
                            }
                            if unsafe { *z2.offset(n as isize) } as i32 == ':' as i32 {
                                let mut tx: DateTime = unsafe { core::mem::zeroed() };
                                let mut day: Sqlite3Int64 = 0 as Sqlite3Int64;
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z2 } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                }
                                unsafe {
                                    memset(&raw mut tx as *mut (), 0,
                                        core::mem::size_of::<DateTime>() as u64)
                                };
                                if parse_hh_mm_ss(z2, &mut tx) != 0 { break '__s6; }
                                compute_jd(&mut tx);
                                tx.i_jd -= 43200000 as Sqlite3Int64;
                                day = tx.i_jd / 86400000 as Sqlite3Int64;
                                tx.i_jd -= day * 86400000 as Sqlite3Int64;
                                if z0 as i32 == '-' as i32 { tx.i_jd = -tx.i_jd; }
                                compute_jd(p);
                                clear_ymd_hms_tz(unsafe { &mut *p });
                                unsafe { (*p).i_jd += tx.i_jd };
                                rc = 0;
                                break '__s6;
                            }
                            {
                                let __n = n;
                                let __p = &mut z;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            while unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z } as u8 as usize)
                                            } as i32 & 1 != 0 {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            n = unsafe { sqlite3_strlen30(z) };
                            if n < 3 || n > 10 { break '__s6; }
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                        as usize)
                                        } as i32 == 's' as i32 {
                                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            }
                            compute_jd(p);
                            { let _ = 0; };
                            r_rounder = if r < 0 as f64 { -0.5 } else { 0.5 };
                            unsafe { (*p).n_floor = 0 as i8 };
                            {
                                i = 0;
                                '__b10: loop {
                                    if !(i <
                                                    (core::mem::size_of::<[AnonS0; 6]>() as u64 / 16) as i32) {
                                        break '__b10;
                                    }
                                    '__c10: loop {
                                        if a_xform_type[i as usize].n_name as i32 == n &&
                                                        unsafe {
                                                                sqlite3_strnicmp(&raw const a_xform_type[i as
                                                                                            usize].z_name[0 as usize] as *const i8, z, n)
                                                            } == 0 && r > -a_xform_type[i as usize].r_limit as f64 &&
                                                r < a_xform_type[i as usize].r_limit as f64 {
                                            '__s11:
                                                {
                                                match i {
                                                    4 => {
                                                        {
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            unsafe { (*p).m_1 += r as i32 };
                                                            x =
                                                                if unsafe { (*p).m_1 } > 0 {
                                                                    (unsafe { (*p).m_1 } - 1) / 12
                                                                } else { (unsafe { (*p).m_1 } - 12) / 12 };
                                                            unsafe { (*p).y += x };
                                                            unsafe { (*p).m_1 -= x * 12 };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    5 => {
                                                        {
                                                            let mut y: i32 = r as i32;
                                                            { let _ = 0; };
                                                            compute_ymd_hms(p);
                                                            { let _ = 0; };
                                                            unsafe { (*p).y += y };
                                                            compute_floor(unsafe { &mut *p });
                                                            unsafe { (*p).valid_jd = 0 as i8 };
                                                            r -= r as i32 as f64;
                                                            break '__s11;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            compute_jd(p);
                                            unsafe {
                                                (*p).i_jd +=
                                                    (r * 1000.0 * a_xform_type[i as usize].r_xform as f64 +
                                                            r_rounder) as Sqlite3Int64
                                            };
                                            rc = 0;
                                            break '__b10;
                                        }
                                        break '__c10;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            clear_ymd_hms_tz(unsafe { &mut *p });
                            break '__s6;
                        }
                        { break '__s6; }
                    }
                    _ => { { break '__s6; } }
                }
            }
            return rc;
        }
    }
}
extern "C" fn is_date(context: *mut Sqlite3Context, argc: i32,
    argv: *const *mut Sqlite3Value, p: *mut DateTime) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut z: *const u8 = core::ptr::null();
        let mut e_type: i32 = 0;
        unsafe {
            memset(p as *mut (), 0, core::mem::size_of::<DateTime>() as u64)
        };
        if argc == 0 {
            if (unsafe { sqlite3_not_pure_func(context) } == 0) as i32 != 0 {
                return 1;
            }
            return set_date_time_to_current(context, p);
        }
        if {
                        e_type =
                            unsafe {
                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                            };
                        e_type
                    } == 2 || e_type == 1 {
            set_raw_date_number(unsafe { &mut *p },
                unsafe {
                    sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                });
        } else {
            z =
                unsafe {
                    sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                };
            if (z).is_null() as i32 != 0 ||
                    parse_date_or_time(context, z as *mut i8 as *const i8, p) !=
                        0 {
                return 1;
            }
        }
        {
            i = 1;
            '__b12: loop {
                if !(i < argc) { break '__b12; }
                '__c12: loop {
                    z =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                        };
                    n =
                        unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(i as isize) })
                        };
                    if z == core::ptr::null() ||
                            parse_modifier(context, z as *mut i8 as *const i8, n, p, i)
                                != 0 {
                        return 1;
                    }
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        compute_jd(p);
        if unsafe { (*p).is_error() } != 0 ||
                (valid_julian_day(unsafe { (*p).i_jd }) == 0) as i32 != 0 {
            return 1;
        }
        if argc == 1 && unsafe { (*p).valid_ymd } != 0 &&
                unsafe { (*p).d } > 28 {
            { let _ = 0; };
            unsafe { (*p).valid_ymd = 0 as i8 };
        }
        return 0;
    }
}
extern "C" fn julianday_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut x: DateTime = unsafe { core::mem::zeroed() };
    if is_date(context, argc, argv as *const *mut Sqlite3Value, &mut x) == 0 {
        compute_jd(&mut x);
        unsafe { sqlite3_result_double(context, x.i_jd as f64 / 86400000.0) };
    }
}
extern "C" fn unixepoch_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut x: DateTime = unsafe { core::mem::zeroed() };
    if is_date(context, argc, argv as *const *mut Sqlite3Value, &mut x) == 0 {
        compute_jd(&mut x);
        if x.use_subsec() != 0 {
            unsafe {
                sqlite3_result_double(context,
                    (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                        1000.0)
            };
        } else {
            unsafe {
                sqlite3_result_int64(context,
                    x.i_jd / 1000 as Sqlite3Int64 -
                        21086676 as i64 * 10000 as i64)
            };
        }
    }
}
extern "C" fn date_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut x: DateTime = unsafe { core::mem::zeroed() };
        if is_date(context, argc, argv as *const *mut Sqlite3Value, &mut x) ==
                0 {
            let mut y: i32 = 0;
            let mut z_buf: [i8; 16] = [0; 16];
            compute_ymd(&mut x);
            y = x.y;
            if y < 0 { y = -y; }
            z_buf[1 as usize] = ('0' as i32 + y / 1000 % 10) as i8;
            z_buf[2 as usize] = ('0' as i32 + y / 100 % 10) as i8;
            z_buf[3 as usize] = ('0' as i32 + y / 10 % 10) as i8;
            z_buf[4 as usize] = ('0' as i32 + y % 10) as i8;
            z_buf[5 as usize] = '-' as i32 as i8;
            z_buf[6 as usize] = ('0' as i32 + x.m_1 / 10 % 10) as i8;
            z_buf[7 as usize] = ('0' as i32 + x.m_1 % 10) as i8;
            z_buf[8 as usize] = '-' as i32 as i8;
            z_buf[9 as usize] = ('0' as i32 + x.d / 10 % 10) as i8;
            z_buf[10 as usize] = ('0' as i32 + x.d % 10) as i8;
            z_buf[11 as usize] = 0 as i8;
            if x.y < 0 {
                z_buf[0 as usize] = '-' as i32 as i8;
                unsafe {
                    sqlite3_result_text(context,
                        &raw mut z_buf[0 as usize] as *mut i8 as *const i8, 11,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            } else {
                unsafe {
                    sqlite3_result_text(context,
                        &raw mut z_buf[1 as usize] as *const i8, 10,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            }
        }
    }
}
extern "C" fn time_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut x: DateTime = unsafe { core::mem::zeroed() };
    if is_date(context, argc, argv as *const *mut Sqlite3Value, &mut x) == 0 {
        let mut s: i32 = 0;
        let mut n: i32 = 0;
        let mut z_buf: [i8; 16] = [0; 16];
        compute_hms(&mut x);
        z_buf[0 as usize] = ('0' as i32 + x.h / 10 % 10) as i8;
        z_buf[1 as usize] = ('0' as i32 + x.h % 10) as i8;
        z_buf[2 as usize] = ':' as i32 as i8;
        z_buf[3 as usize] = ('0' as i32 + x.m / 10 % 10) as i8;
        z_buf[4 as usize] = ('0' as i32 + x.m % 10) as i8;
        z_buf[5 as usize] = ':' as i32 as i8;
        if x.use_subsec() != 0 {
            s = (1000.0 * x.s + 0.5) as i32;
            z_buf[6 as usize] = ('0' as i32 + s / 10000 % 10) as i8;
            z_buf[7 as usize] = ('0' as i32 + s / 1000 % 10) as i8;
            z_buf[8 as usize] = '.' as i32 as i8;
            z_buf[9 as usize] = ('0' as i32 + s / 100 % 10) as i8;
            z_buf[10 as usize] = ('0' as i32 + s / 10 % 10) as i8;
            z_buf[11 as usize] = ('0' as i32 + s % 10) as i8;
            z_buf[12 as usize] = 0 as i8;
            n = 12;
        } else {
            s = x.s as i32;
            z_buf[6 as usize] = ('0' as i32 + s / 10 % 10) as i8;
            z_buf[7 as usize] = ('0' as i32 + s % 10) as i8;
            z_buf[8 as usize] = 0 as i8;
            n = 8;
        }
        unsafe {
            sqlite3_result_text(context,
                &raw mut z_buf[0 as usize] as *mut i8 as *const i8, n,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
}
extern "C" fn datetime_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut x: DateTime = unsafe { core::mem::zeroed() };
        if is_date(context, argc, argv as *const *mut Sqlite3Value, &mut x) ==
                0 {
            let mut y: i32 = 0;
            let mut s: i32 = 0;
            let mut n: i32 = 0;
            let mut z_buf: [i8; 32] = [0; 32];
            compute_ymd_hms(&mut x);
            y = x.y;
            if y < 0 { y = -y; }
            z_buf[1 as usize] = ('0' as i32 + y / 1000 % 10) as i8;
            z_buf[2 as usize] = ('0' as i32 + y / 100 % 10) as i8;
            z_buf[3 as usize] = ('0' as i32 + y / 10 % 10) as i8;
            z_buf[4 as usize] = ('0' as i32 + y % 10) as i8;
            z_buf[5 as usize] = '-' as i32 as i8;
            z_buf[6 as usize] = ('0' as i32 + x.m_1 / 10 % 10) as i8;
            z_buf[7 as usize] = ('0' as i32 + x.m_1 % 10) as i8;
            z_buf[8 as usize] = '-' as i32 as i8;
            z_buf[9 as usize] = ('0' as i32 + x.d / 10 % 10) as i8;
            z_buf[10 as usize] = ('0' as i32 + x.d % 10) as i8;
            z_buf[11 as usize] = ' ' as i32 as i8;
            z_buf[12 as usize] = ('0' as i32 + x.h / 10 % 10) as i8;
            z_buf[13 as usize] = ('0' as i32 + x.h % 10) as i8;
            z_buf[14 as usize] = ':' as i32 as i8;
            z_buf[15 as usize] = ('0' as i32 + x.m / 10 % 10) as i8;
            z_buf[16 as usize] = ('0' as i32 + x.m % 10) as i8;
            z_buf[17 as usize] = ':' as i32 as i8;
            if x.use_subsec() != 0 {
                s = (1000.0 * x.s + 0.5) as i32;
                z_buf[18 as usize] = ('0' as i32 + s / 10000 % 10) as i8;
                z_buf[19 as usize] = ('0' as i32 + s / 1000 % 10) as i8;
                z_buf[20 as usize] = '.' as i32 as i8;
                z_buf[21 as usize] = ('0' as i32 + s / 100 % 10) as i8;
                z_buf[22 as usize] = ('0' as i32 + s / 10 % 10) as i8;
                z_buf[23 as usize] = ('0' as i32 + s % 10) as i8;
                z_buf[24 as usize] = 0 as i8;
                n = 24;
            } else {
                s = x.s as i32;
                z_buf[18 as usize] = ('0' as i32 + s / 10 % 10) as i8;
                z_buf[19 as usize] = ('0' as i32 + s % 10) as i8;
                z_buf[20 as usize] = 0 as i8;
                n = 20;
            }
            if x.y < 0 {
                z_buf[0 as usize] = '-' as i32 as i8;
                unsafe {
                    sqlite3_result_text(context,
                        &raw mut z_buf[0 as usize] as *mut i8 as *const i8, n,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            } else {
                unsafe {
                    sqlite3_result_text(context,
                        &raw mut z_buf[1 as usize] as *const i8, n - 1,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            }
        }
    }
}
extern "C" fn days_after_monday(p_date_1: &DateTime) -> i32 {
    { let _ = 0; };
    return (((*p_date_1).i_jd + 43200000 as Sqlite3Int64) /
                    86400000 as Sqlite3Int64) as i32 % 7;
}
extern "C" fn days_after_jan01(p_date_1: &DateTime) -> i32 {
    unsafe {
        let mut jan01: DateTime = *p_date_1;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        jan01.valid_jd = 0 as i8;
        jan01.m_1 = 1;
        jan01.d = 1;
        compute_jd(&mut jan01);
        return (((*p_date_1).i_jd - jan01.i_jd + 43200000 as Sqlite3Int64) /
                    86400000 as Sqlite3Int64) as i32;
    }
}
extern "C" fn days_after_sunday(p_date_1: &DateTime) -> i32 {
    { let _ = 0; };
    return (((*p_date_1).i_jd + 129600000 as Sqlite3Int64) /
                    86400000 as Sqlite3Int64) as i32 % 7;
}
extern "C" fn strftime_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut x: DateTime = unsafe { core::mem::zeroed() };
        let mut i: u64 = 0 as u64;
        let mut j: u64 = 0 as u64;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut z_fmt: *const i8 = core::ptr::null();
        let mut p_res: *mut Sqlite3Str = core::ptr::null_mut();
        if argc == 0 { return; }
        z_fmt =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        if z_fmt == core::ptr::null() ||
                is_date(context, argc - 1,
                        unsafe { argv.offset(1 as isize) } as
                            *const *mut Sqlite3Value, &mut x) != 0 {
            return;
        }
        db = unsafe { sqlite3_context_db_handle(context) };
        p_res = unsafe { sqlite3_str_new(db) };
        compute_jd(&mut x);
        compute_ymd_hms(&mut x);
        {
            i = { j = 0 as u64; j };
            '__b13: loop {
                if !(unsafe { *z_fmt.add(i as usize) } != 0) { break '__b13; }
                '__c13: loop {
                    let mut cf: i8 = 0 as i8;
                    if unsafe { *z_fmt.add(i as usize) } as i32 != '%' as i32 {
                        break '__c13;
                    }
                    if j < i {
                        unsafe {
                            sqlite3_str_append(p_res, unsafe { z_fmt.add(j as usize) },
                                (i - j) as i32)
                        };
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    j = i + 1 as u64;
                    cf = unsafe { *z_fmt.add(i as usize) } as i8;
                    '__s14:
                        {
                        match cf {
                            100 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'd' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.d)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut s: f64 = x.s;
                                    if s > 59.999 { s = 59.999; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%06.3f".as_ptr() as *mut i8 as *const i8, s)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d-%02d-%02d".as_ptr() as *mut i8 as *const i8, x.y,
                                            x.m_1, x.d)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    if cf as i32 == 'g' as i32 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%02d".as_ptr() as *mut i8 as *const i8, y.y % 100)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%04d".as_ptr() as *mut i8 as *const i8, y.y)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            101 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'd' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.d)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut s: f64 = x.s;
                                    if s > 59.999 { s = 59.999; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%06.3f".as_ptr() as *mut i8 as *const i8, s)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d-%02d-%02d".as_ptr() as *mut i8 as *const i8, x.y,
                                            x.m_1, x.d)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    if cf as i32 == 'g' as i32 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%02d".as_ptr() as *mut i8 as *const i8, y.y % 100)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%04d".as_ptr() as *mut i8 as *const i8, y.y)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            102 => {
                                {
                                    let mut s: f64 = x.s;
                                    if s > 59.999 { s = 59.999; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%06.3f".as_ptr() as *mut i8 as *const i8, s)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d-%02d-%02d".as_ptr() as *mut i8 as *const i8, x.y,
                                            x.m_1, x.d)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    if cf as i32 == 'g' as i32 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%02d".as_ptr() as *mut i8 as *const i8, y.y % 100)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%04d".as_ptr() as *mut i8 as *const i8, y.y)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            70 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d-%02d-%02d".as_ptr() as *mut i8 as *const i8, x.y,
                                            x.m_1, x.d)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    if cf as i32 == 'g' as i32 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%02d".as_ptr() as *mut i8 as *const i8, y.y % 100)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%04d".as_ptr() as *mut i8 as *const i8, y.y)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            71 => {
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    if cf as i32 == 'g' as i32 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%02d".as_ptr() as *mut i8 as *const i8, y.y % 100)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%04d".as_ptr() as *mut i8 as *const i8, y.y)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            103 => {
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    if cf as i32 == 'g' as i32 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%02d".as_ptr() as *mut i8 as *const i8, y.y % 100)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%04d".as_ptr() as *mut i8 as *const i8, y.y)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            72 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            107 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'H' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, x.h)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            73 => {
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            108 => {
                                {
                                    let mut h: i32 = x.h;
                                    if h > 12 { h -= 12; }
                                    if h == 0 { h = 12; }
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            if cf as i32 == 'I' as i32 {
                                                    c"%02d".as_ptr() as *mut i8
                                                } else { c"%2d".as_ptr() as *mut i8 } as *const i8, h)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            106 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%03d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&x) + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            74 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%.16g".as_ptr() as *mut i8 as *const i8,
                                            x.i_jd as f64 / 86400000.0)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            109 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m_1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            77 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            112 => {
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            80 => {
                                {
                                    if x.h >= 12 {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"PM".as_ptr() as *mut i8
                                                    } else { c"pm".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_append(p_res,
                                                if cf as i32 == 'p' as i32 {
                                                        c"AM".as_ptr() as *mut i8
                                                    } else { c"am".as_ptr() as *mut i8 } as *const i8, 2)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            82 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h, x.m)
                                    };
                                    break '__s14;
                                }
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            115 => {
                                {
                                    if x.use_subsec() != 0 {
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%.3f".as_ptr() as *mut i8 as *const i8,
                                                (x.i_jd - 21086676 as i64 * 10000000 as i64) as f64 /
                                                    1000.0)
                                        };
                                    } else {
                                        let i_s: i64 =
                                            (x.i_jd / 1000 as Sqlite3Int64 -
                                                    21086676 as i64 * 10000 as i64) as i64;
                                        unsafe {
                                            sqlite3_str_appendf(p_res,
                                                c"%lld".as_ptr() as *mut i8 as *const i8, i_s)
                                        };
                                    }
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            83 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            84 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d:%02d:%02d".as_ptr() as *mut i8 as *const i8, x.h,
                                            x.m, x.s as i32)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            117 => {
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            119 => {
                                {
                                    let mut c: i8 =
                                        (days_after_sunday(&x) as i8 as i32 + '0' as i32) as i8;
                                    if c as i32 == '0' as i32 && cf as i32 == 'u' as i32 {
                                        c = '7' as i32 as i8;
                                    }
                                    unsafe { sqlite3_str_appendchar(p_res, 1, c) };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            85 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_sunday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            86 => {
                                {
                                    let mut y: DateTime = x;
                                    { let _ = 0; };
                                    y.i_jd +=
                                        ((3 - days_after_monday(&x)) * 86400000) as Sqlite3Int64;
                                    y.valid_ymd = 0 as i8;
                                    compute_ymd(&mut y);
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            days_after_jan01(&y) / 7 + 1)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            87 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%02d".as_ptr() as *mut i8 as *const i8,
                                            (days_after_jan01(&x) - days_after_monday(&x) + 7) / 7)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            89 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendf(p_res,
                                            c"%04d".as_ptr() as *mut i8 as *const i8, x.y)
                                    };
                                    break '__s14;
                                }
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            37 => {
                                {
                                    unsafe {
                                        sqlite3_str_appendchar(p_res, 1, '%' as i32 as i8)
                                    };
                                    break '__s14;
                                }
                                { unsafe { sqlite3_str_free(p_res) }; return; }
                            }
                            _ => { { unsafe { sqlite3_str_free(p_res) }; return; } }
                        }
                    }
                    break '__c13;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if j < i {
            unsafe {
                sqlite3_str_append(p_res, unsafe { z_fmt.add(j as usize) },
                    (i - j) as i32)
            };
        }
        unsafe { sqlite3_result_str(context, p_res, 2) };
    }
}
extern "C" fn timediff_func(context: *mut Sqlite3Context, not_used1_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut sign: i8 = 0 as i8;
        let mut y: i32 = 0;
        let mut m: i32 = 0;
        let mut d1: DateTime = unsafe { core::mem::zeroed() };
        let mut d2: DateTime = unsafe { core::mem::zeroed() };
        let mut s_res: Sqlite3Str = unsafe { core::mem::zeroed() };
        { let _ = not_used1_1; };
        if is_date(context, 1,
                    unsafe { &raw mut *argv.offset(0 as isize) } as
                        *const *mut Sqlite3Value, &mut d1) != 0 {
            return;
        }
        if is_date(context, 1,
                    unsafe { &raw mut *argv.offset(1 as isize) } as
                        *const *mut Sqlite3Value, &mut d2) != 0 {
            return;
        }
        compute_ymd_hms(&mut d1);
        compute_ymd_hms(&mut d2);
        if d1.i_jd >= d2.i_jd {
            sign = '+' as i32 as i8;
            y = d1.y - d2.y;
            if y != 0 {
                d2.y = d1.y;
                d2.valid_jd = 0 as i8;
                compute_jd(&mut d2);
            }
            m = d1.m_1 - d2.m_1;
            if m < 0 {
                { let __p = &mut y; let __t = *__p; *__p -= 1; __t };
                m += 12;
            }
            if m != 0 {
                d2.m_1 = d1.m_1;
                d2.valid_jd = 0 as i8;
                compute_jd(&mut d2);
            }
            while d1.i_jd < d2.i_jd {
                { let __p = &mut m; let __t = *__p; *__p -= 1; __t };
                if m < 0 {
                    m = 11;
                    { let __p = &mut y; let __t = *__p; *__p -= 1; __t };
                }
                { let __p = &mut d2.m_1; let __t = *__p; *__p -= 1; __t };
                if d2.m_1 < 1 {
                    d2.m_1 = 12;
                    { let __p = &mut d2.y; let __t = *__p; *__p -= 1; __t };
                }
                d2.valid_jd = 0 as i8;
                compute_jd(&mut d2);
            }
            d1.i_jd -= d2.i_jd;
            d1.i_jd += (1486995408 as u64 * 100000 as u64) as Sqlite3Int64;
        } else {
            sign = '-' as i32 as i8;
            y = d2.y - d1.y;
            if y != 0 {
                d2.y = d1.y;
                d2.valid_jd = 0 as i8;
                compute_jd(&mut d2);
            }
            m = d2.m_1 - d1.m_1;
            if m < 0 {
                { let __p = &mut y; let __t = *__p; *__p -= 1; __t };
                m += 12;
            }
            if m != 0 {
                d2.m_1 = d1.m_1;
                d2.valid_jd = 0 as i8;
                compute_jd(&mut d2);
            }
            while d1.i_jd > d2.i_jd {
                { let __p = &mut m; let __t = *__p; *__p -= 1; __t };
                if m < 0 {
                    m = 11;
                    { let __p = &mut y; let __t = *__p; *__p -= 1; __t };
                }
                { let __p = &mut d2.m_1; let __t = *__p; *__p += 1; __t };
                if d2.m_1 > 12 {
                    d2.m_1 = 1;
                    { let __p = &mut d2.y; let __t = *__p; *__p += 1; __t };
                }
                d2.valid_jd = 0 as i8;
                compute_jd(&mut d2);
            }
            d1.i_jd = d2.i_jd - d1.i_jd;
            d1.i_jd += (1486995408 as u64 * 100000 as u64) as Sqlite3Int64;
        }
        clear_ymd_hms_tz(&mut d1);
        compute_ymd_hms(&mut d1);
        unsafe {
            sqlite3_str_accum_init(&raw mut s_res as *mut StrAccum,
                core::ptr::null_mut(), core::ptr::null_mut(), 0, 100)
        };
        unsafe {
            sqlite3_str_appendf(&mut s_res,
                c"%c%04d-%02d-%02d %02d:%02d:%06.3f".as_ptr() as *mut i8 as
                    *const i8, sign as i32, y, m, d1.d - 1, d1.h, d1.m, d1.s)
        };
        unsafe { sqlite3_result_str(context, &mut s_res, 1) };
    }
}
extern "C" fn ctime_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    time_func(context, 0, core::ptr::null_mut());
}
extern "C" fn ctimestamp_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    datetime_func(context, 0, core::ptr::null_mut());
}
extern "C" fn cdate_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    date_func(context, 0, core::ptr::null_mut());
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_date_time_functions() -> () {
    unsafe {
        unsafe {
            sqlite3_insert_builtin_funcs(&raw mut a_date_time_funcs[0 as
                                usize] as *mut FuncDef,
                (core::mem::size_of::<[FuncDef; 10]>() as u64 /
                        core::mem::size_of::<FuncDef>() as u64) as i32)
        };
    }
}
static a_mx: [u16; 6] =
    [12 as u16, 14 as u16, 24 as u16, 31 as u16, 59 as u16, 14712 as u16];
static mut a_date_time_funcs: [FuncDef; 10] =
    [FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(julianday_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"julianday".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(unixepoch_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"unixepoch".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(date_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"date".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(time_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"time".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(datetime_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"datetime".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(strftime_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"strftime".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 8192 | 1 | 2048) as u32,
                p_user_data: &raw mut sqlite3Config as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(timediff_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"timediff".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ctime_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"current_time".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ctimestamp_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"current_timestamp".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 8192 | 1) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(cdate_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"current_date".as_ptr() as *const i8,
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
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_stmt_current_time(_: *mut Sqlite3Context)
    -> Sqlite3Int64;
    static sqlite3_ctype_map: [u8; 0];
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
    -> i32;
    static sqlite3_upper_to_lower: [u8; 0];
    fn localtime_r(_: *const TimeT, _: *mut Tm)
    -> *mut Tm;
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut Sqlite3, _: *mut i8,
    _: i32, _: i32)
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
    fn __builtin_unreachable()
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