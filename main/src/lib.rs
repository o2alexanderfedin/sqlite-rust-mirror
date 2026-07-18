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
type DarwinVaList = *mut i8;
type DarwinIntptrT = i64;
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_libversion() -> *const i8 {
    return &raw const sqlite3_version[0 as usize] as *const i8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sourceid() -> *const i8 {
    return c"2026-06-29 15:11:58 007f0496619e571092382a3668b6bbcc17919908895dc4be71f8d1b6ec2aeeed".as_ptr()
                as *mut i8 as *const i8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_libversion_number() -> i32 { return 3054000; }
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_compileoption_used(mut z_opt_name: *const i8)
    -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut n_opt: i32 = 0;
    let mut az_compile_opt: *const *const i8 = core::ptr::null();
    az_compile_opt = unsafe { sqlite3_compile_options(&mut n_opt) };
    if unsafe {
                sqlite3_strnicmp(z_opt_name,
                    c"SQLITE_".as_ptr() as *mut i8 as *const i8, 7)
            } == 0 {
        {
            let __n = 7;
            let __p = &mut z_opt_name;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    n = unsafe { sqlite3_strlen30(z_opt_name) };
    {
        i = 0;
        '__b0: loop {
            if !(i < n_opt) { break '__b0; }
            '__c0: loop {
                if unsafe {
                                sqlite3_strnicmp(z_opt_name,
                                    unsafe { *az_compile_opt.offset(i as isize) }, n)
                            } == 0 &&
                        unsafe {
                                sqlite3_is_id_char(unsafe {
                                            *unsafe {
                                                    (*az_compile_opt.offset(i as isize)).offset(n as isize)
                                                }
                                        } as u8)
                            } == 0 {
                    return 1;
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_compileoption_get(n: i32) -> *const i8 {
    let mut n_opt: i32 = 0;
    let mut az_compile_opt: *const *const i8 = core::ptr::null();
    az_compile_opt = unsafe { sqlite3_compile_options(&mut n_opt) };
    if n >= 0 && n < n_opt {
        return unsafe { *az_compile_opt.offset(n as isize) };
    }
    return core::ptr::null();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_threadsafe() -> i32 { return 1; }
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
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_report_error(i_err: i32, lineno: i32,
    z_type: *const i8) -> i32 {
    unsafe {
        sqlite3_log(i_err,
            c"%s at line %d of [%.10s]".as_ptr() as *mut i8 as *const i8,
            z_type, lineno, unsafe { sqlite3_sourceid().offset(20 as isize) })
    };
    return i_err;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_misuse_error(lineno: i32) -> i32 {
    return sqlite3_report_error(21, lineno,
            c"misuse".as_ptr() as *mut i8 as *const i8);
}
extern "C" fn disconnect_all_vtab(db: *mut Sqlite3) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut p: *mut HashElem = core::ptr::null_mut();
        unsafe { sqlite3_btree_enter_all(db) };
        {
            i = 0;
            '__b1: loop {
                if !(i < unsafe { (*db).n_db }) { break '__b1; }
                '__c1: loop {
                    let p_schema: *mut Schema =
                        unsafe {
                            (*unsafe { (*db).a_db.offset(i as isize) }).p_schema
                        };
                    if !(p_schema).is_null() {
                        {
                            p =
                                unsafe { (*unsafe { &mut (*p_schema).tbl_hash }).first };
                            '__b2: loop {
                                if !(!(p).is_null()) { break '__b2; }
                                '__c2: loop {
                                    let p_tab: *mut Table = unsafe { (*p).data } as *mut Table;
                                    if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                                        unsafe { sqlite3_vtab_disconnect(db, p_tab) };
                                    }
                                    break '__c2;
                                }
                                p = unsafe { (*p).next };
                            }
                        }
                    }
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            p = unsafe { (*unsafe { &mut (*db).a_module }).first };
            '__b3: loop {
                if !(!(p).is_null()) { break '__b3; }
                '__c3: loop {
                    let p_mod: *const Module =
                        unsafe { (*p).data } as *mut Module as *const Module;
                    if !(unsafe { (*p_mod).p_epo_tab }).is_null() {
                        unsafe {
                            sqlite3_vtab_disconnect(db, unsafe { (*p_mod).p_epo_tab })
                        };
                    }
                    break '__c3;
                }
                p = unsafe { (*p).next };
            }
        }
        unsafe { sqlite3_vtab_unlock_list(db) };
        unsafe { sqlite3_btree_leave_all(db) };
    }
}
extern "C" fn connection_is_busy(db: &Sqlite3) -> i32 {
    let mut j: i32 = 0;
    { let _ = 0; };
    if !((*db).p_vdbe).is_null() { return 1; }
    {
        j = 0;
        '__b4: loop {
            if !(j < (*db).n_db) { break '__b4; }
            '__c4: loop {
                let p_bt: *mut Btree =
                    unsafe { (*(*db).a_db.offset(j as isize)).p_bt };
                if !(p_bt).is_null() &&
                        unsafe { sqlite3_btree_is_in_backup(p_bt) } != 0 {
                    return 1;
                }
                break '__c4;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rollback_all(db: *mut Sqlite3, trip_code_1: i32)
    -> () {
    let mut i: i32 = 0;
    let mut in_trans: i32 = 0;
    let mut schema_change: i32 = 0;
    { let _ = 0; };
    unsafe { sqlite3_begin_benign_malloc() };
    unsafe { sqlite3_btree_enter_all(db) };
    schema_change =
        (unsafe { (*db).m_db_flags } & 1 as u32 != 0 as u32 &&
                unsafe { (*db).init.busy } as i32 == 0) as i32;
    {
        i = 0;
        '__b5: loop {
            if !(i < unsafe { (*db).n_db }) { break '__b5; }
            '__c5: loop {
                let p: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                if !(p).is_null() {
                    if unsafe { sqlite3_btree_txn_state(p) } == 2 {
                        in_trans = 1;
                    }
                    unsafe {
                        sqlite3_btree_rollback(p, trip_code_1,
                            (schema_change == 0) as i32 as i32)
                    };
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_vtab_rollback(db) };
    unsafe { sqlite3_end_benign_malloc() };
    if schema_change != 0 {
        unsafe { sqlite3_expire_prepared_statements(db, 0) };
        unsafe { sqlite3_reset_all_schemas_of_connection(db) };
    }
    unsafe { sqlite3_btree_leave_all(db) };
    unsafe { (*db).n_deferred_cons = 0 as i64 };
    unsafe { (*db).n_deferred_imm_cons = 0 as i64 };
    unsafe { (*db).flags &= !((524288 as u64 | (2 as u64) << 32) as u64) };
    if unsafe { (*db).x_rollback_callback.is_some() } &&
            (in_trans != 0 || (unsafe { (*db).auto_commit } == 0) as i32 != 0)
        {
        unsafe {
            (unsafe {
                    (*db).x_rollback_callback.unwrap()
                })(unsafe { (*db).p_rollback_arg })
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_close_savepoints(db: *mut Sqlite3) -> () {
    while !(unsafe { (*db).p_savepoint }).is_null() {
        let p_tmp: *mut Savepoint = unsafe { (*db).p_savepoint };
        unsafe { (*db).p_savepoint = unsafe { (*p_tmp).p_next } };
        unsafe { sqlite3_db_free(db, p_tmp as *mut ()) };
    }
    unsafe { (*db).n_savepoint = 0 };
    unsafe { (*db).n_statement = 0 };
    unsafe { (*db).is_transaction_savepoint = 0 as u8 };
}
extern "C" fn function_destroy(db: *mut Sqlite3, p: &FuncDef) -> () {
    unsafe {
        let mut p_destructor: *mut FuncDestructor = core::ptr::null_mut();
        { let _ = 0; };
        p_destructor = (*p).u.p_destructor;
        if !(p_destructor).is_null() {
            {
                let __p = unsafe { &mut (*p_destructor).n_ref };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if unsafe { (*p_destructor).n_ref } == 0 {
                unsafe {
                    (unsafe {
                            (*p_destructor).x_destroy.unwrap()
                        })(unsafe { (*p_destructor).p_user_data })
                };
                unsafe { sqlite3_db_free(db, p_destructor as *mut ()) };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_leave_mutex_and_close_zombie(db: *mut Sqlite3)
    -> () {
    unsafe {
        let mut i: *mut HashElem = core::ptr::null_mut();
        let mut j: i32 = 0;
        if unsafe { (*db).e_open_state } as i32 != 167 ||
                connection_is_busy(unsafe { &*db }) != 0 {
            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
            return;
        }
        unsafe { sqlite3_rollback_all(db, 0) };
        sqlite3_close_savepoints(db);
        {
            j = 0;
            '__b7: loop {
                if !(j < unsafe { (*db).n_db }) { break '__b7; }
                '__c7: loop {
                    let p_db: *mut Db =
                        unsafe { &mut *unsafe { (*db).a_db.offset(j as isize) } };
                    if !(unsafe { (*p_db).p_bt }).is_null() {
                        unsafe { sqlite3_btree_close(unsafe { (*p_db).p_bt }) };
                        unsafe { (*p_db).p_bt = core::ptr::null_mut() };
                        if j != 1 {
                            unsafe { (*p_db).p_schema = core::ptr::null_mut() };
                        }
                    }
                    break '__c7;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        if !(unsafe {
                            (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                        }).is_null() {
            unsafe {
                sqlite3_schema_clear(unsafe {
                            (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                        } as *mut ())
            };
            { let _ = 0; };
        }
        unsafe { sqlite3_vtab_unlock_list(db) };
        unsafe { sqlite3_collapse_database_array(db) };
        { let _ = 0; };
        { let _ = 0; };
        {
            i = unsafe { (*unsafe { &mut (*db).a_func }).first };
            '__b8: loop {
                if !(!(i).is_null()) { break '__b8; }
                '__c8: loop {
                    let mut p_next: *mut FuncDef = core::ptr::null_mut();
                    let mut p: *mut FuncDef = core::ptr::null_mut();
                    p = unsafe { (*i).data } as *mut FuncDef;
                    '__b9: loop {
                        '__c9: loop {
                            function_destroy(db, unsafe { &*p });
                            p_next = unsafe { (*p).p_next };
                            unsafe { sqlite3_db_free(db, p as *mut ()) };
                            p = p_next;
                            break '__c9;
                        }
                        if !(!(p).is_null()) { break '__b9; }
                    }
                    break '__c8;
                }
                i = unsafe { (*i).next };
            }
        }
        unsafe { sqlite3_hash_clear(unsafe { &mut (*db).a_func }) };
        {
            i = unsafe { (*unsafe { &mut (*db).a_coll_seq }).first };
            '__b10: loop {
                if !(!(i).is_null()) { break '__b10; }
                '__c10: loop {
                    let p_coll: *mut CollSeq =
                        unsafe { (*i).data } as *mut CollSeq;
                    {
                        j = 0;
                        '__b11: loop {
                            if !(j < 3) { break '__b11; }
                            '__c11: loop {
                                if unsafe { (*p_coll.offset(j as isize)).x_del.is_some() } {
                                    unsafe {
                                        (unsafe {
                                                (*p_coll.offset(j as isize)).x_del.unwrap()
                                            })(unsafe { (*p_coll.offset(j as isize)).p_user })
                                    };
                                }
                                break '__c11;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { sqlite3_db_free(db, p_coll as *mut ()) };
                    break '__c10;
                }
                i = unsafe { (*i).next };
            }
        }
        unsafe { sqlite3_hash_clear(unsafe { &mut (*db).a_coll_seq }) };
        {
            i = unsafe { (*unsafe { &mut (*db).a_module }).first };
            '__b12: loop {
                if !(!(i).is_null()) { break '__b12; }
                '__c12: loop {
                    let p_mod: *mut Module =
                        unsafe { (*i).data } as *mut Module;
                    unsafe { sqlite3_vtab_eponymous_table_clear(db, p_mod) };
                    unsafe { sqlite3_vtab_module_unref(db, p_mod) };
                    break '__c12;
                }
                i = unsafe { (*i).next };
            }
        }
        unsafe { sqlite3_hash_clear(unsafe { &mut (*db).a_module }) };
        unsafe { sqlite3_error(db, 0) };
        unsafe { sqlite3ValueFree(unsafe { (*db).p_err }) };
        unsafe { sqlite3_close_extensions(db) };
        unsafe { (*db).e_open_state = 213 as u8 };
        unsafe {
            sqlite3_db_free(db,
                unsafe {
                        (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                    } as *mut ())
        };
        if unsafe { (*db).x_autovac_destr.is_some() } {
            unsafe {
                (unsafe {
                        (*db).x_autovac_destr.unwrap()
                    })(unsafe { (*db).p_autovac_pages_arg })
            };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        unsafe { (*db).e_open_state = 206 as u8 };
        unsafe { sqlite3_mutex_free(unsafe { (*db).mutex }) };
        { let _ = 0; };
        if unsafe { (*db).lookaside.b_malloced } != 0 {
            unsafe { sqlite3_free(unsafe { (*db).lookaside.p_start }) };
        }
        unsafe { sqlite3_free(db as *mut ()) };
    }
}
extern "C" fn sqlite3_close_2(db: *mut Sqlite3, force_zombie_1: i32) -> i32 {
    unsafe {
        if (db).is_null() as i32 != 0 { return 0; }
        if (unsafe { sqlite3_safety_check_sick_or_ok(db) } == 0) as i32 != 0 {
            return sqlite3_misuse_error(1274);
        }
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if unsafe { (*db).m_trace } as i32 & 8 != 0 {
            unsafe {
                (unsafe {
                        (*db).trace.x_v2.unwrap()
                    })(8, unsafe { (*db).p_trace_arg }, db as *mut (),
                    core::ptr::null_mut())
            };
        }
        disconnect_all_vtab(db);
        unsafe { sqlite3_vtab_rollback(db) };
        if (force_zombie_1 == 0) as i32 != 0 &&
                connection_is_busy(unsafe { &*db }) != 0 {
            unsafe {
                sqlite3_error_with_msg(db, 5,
                    c"unable to close due to unfinalized statements or unfinished backups".as_ptr()
                            as *mut i8 as *const i8)
            };
            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
            return 5;
        }
        while !(unsafe { (*db).p_db_data }).is_null() {
            let p: *mut DbClientData = unsafe { (*db).p_db_data };
            unsafe { (*db).p_db_data = unsafe { (*p).p_next } };
            { let _ = 0; };
            if unsafe { (*p).x_destructor.is_some() } {
                unsafe {
                    (unsafe {
                            (*p).x_destructor.unwrap()
                        })(unsafe { (*p).p_data })
                };
            }
            unsafe { sqlite3_free(p as *mut ()) };
        }
        unsafe { (*db).e_open_state = 167 as u8 };
        unsafe { sqlite3_leave_mutex_and_close_zombie(db) };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_close(db: *mut Sqlite3) -> i32 {
    return sqlite3_close_2(db, 0);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_close_v2(db: *mut Sqlite3) -> i32 {
    return sqlite3_close_2(db, 1);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_initialize() -> i32 {
    unsafe {
        let mut p_main_mtx: *mut Sqlite3Mutex = core::ptr::null_mut();
        let mut rc: i32 = 0;
        { let _ = 0; };
        if sqlite3Config.is_init != 0 {
            unsafe { sqlite3_memory_barrier() };
            return 0;
        }
        rc = unsafe { sqlite3_mutex_init() };
        if rc != 0 { return rc; }
        p_main_mtx = unsafe { sqlite3MutexAlloc(2) };
        unsafe { sqlite3_mutex_enter(p_main_mtx) };
        sqlite3Config.is_mutex_init = 1;
        if (sqlite3Config.is_malloc_init == 0) as i32 != 0 {
            rc = unsafe { sqlite3_malloc_init() };
        }
        if rc == 0 {
            sqlite3Config.is_malloc_init = 1;
            if (sqlite3Config.p_init_mutex).is_null() as i32 != 0 {
                sqlite3Config.p_init_mutex = unsafe { sqlite3MutexAlloc(1) };
                if sqlite3Config.b_core_mutex != 0 &&
                        (sqlite3Config.p_init_mutex).is_null() as i32 != 0 {
                    rc = 7;
                }
            }
        }
        if rc == 0 {
            {
                let __p = &mut sqlite3Config.n_ref_init_mutex;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        unsafe { sqlite3_mutex_leave(p_main_mtx) };
        if rc != 0 { return rc; }
        unsafe { sqlite3_mutex_enter(sqlite3Config.p_init_mutex) };
        if sqlite3Config.is_init == 0 && sqlite3Config.in_progress == 0 {
            sqlite3Config.in_progress = 1;
            unsafe {
                memset(&raw mut sqlite3_builtin_functions as *mut (), 0,
                    core::mem::size_of::<FuncDefHash>() as u64)
            };
            unsafe { sqlite3_register_builtin_functions() };
            if sqlite3Config.is_p_cache_init == 0 {
                rc = unsafe { sqlite3_pcache_initialize() };
            }
            if rc == 0 {
                sqlite3Config.is_p_cache_init = 1;
                rc = unsafe { sqlite3OsInit() };
            }
            if rc == 0 { rc = unsafe { sqlite3_memdb_init() }; }
            if rc == 0 {
                unsafe {
                    sqlite3_p_cache_buffer_setup(sqlite3Config.p_page,
                        sqlite3Config.sz_page, sqlite3Config.n_page)
                };
            }
            if rc == 0 {
                unsafe { sqlite3_memory_barrier() };
                sqlite3Config.is_init = 1;
            }
            sqlite3Config.in_progress = 0;
        }
        unsafe { sqlite3_mutex_leave(sqlite3Config.p_init_mutex) };
        unsafe { sqlite3_mutex_enter(p_main_mtx) };
        {
            let __p = &mut sqlite3Config.n_ref_init_mutex;
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if sqlite3Config.n_ref_init_mutex <= 0 {
            { let _ = 0; };
            unsafe { sqlite3_mutex_free(sqlite3Config.p_init_mutex) };
            sqlite3Config.p_init_mutex = core::ptr::null_mut();
        }
        unsafe { sqlite3_mutex_leave(p_main_mtx) };
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_shutdown() -> i32 {
    unsafe {
        if sqlite3Config.is_init != 0 {
            unsafe { sqlite3_os_end() };
            unsafe { sqlite3_reset_auto_extension() };
            sqlite3Config.is_init = 0;
        }
        if sqlite3Config.is_p_cache_init != 0 {
            unsafe { sqlite3_pcache_shutdown() };
            sqlite3Config.is_p_cache_init = 0;
        }
        if sqlite3Config.is_malloc_init != 0 {
            unsafe { sqlite3_malloc_end() };
            sqlite3Config.is_malloc_init = 0;
            sqlite3_data_directory = core::ptr::null_mut();
            sqlite3_temp_directory = core::ptr::null_mut();
        }
        if sqlite3Config.is_mutex_init != 0 {
            unsafe { sqlite3_mutex_end() };
            sqlite3Config.is_mutex_init = 0;
        }
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_config(op: i32, mut __va0: ...) -> i32 {
    unsafe {
        let mut ap: *const i8 = core::ptr::null();
        let mut rc: i32 = 0;
        if sqlite3Config.is_init != 0 {
            if op < 0 || op > 63 ||
                    (1 as u64) << op & m_anytime_config_option as u64 ==
                        0 as u64 {
                return sqlite3_misuse_error(440);
            }
        }
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        '__s14:
            {
            match op {
                1 => {
                    {
                        sqlite3Config.b_core_mutex = 0 as u8;
                        sqlite3Config.b_full_mutex = 0 as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_core_mutex = 1 as u8;
                        sqlite3Config.b_full_mutex = 0 as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_core_mutex = 1 as u8;
                        sqlite3Config.b_full_mutex = 1 as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.mutex =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    } = sqlite3Config.mutex
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.m =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                2 => {
                    {
                        sqlite3Config.b_core_mutex = 1 as u8;
                        sqlite3Config.b_full_mutex = 0 as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_core_mutex = 1 as u8;
                        sqlite3Config.b_full_mutex = 1 as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.mutex =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    } = sqlite3Config.mutex
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.m =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                3 => {
                    {
                        sqlite3Config.b_core_mutex = 1 as u8;
                        sqlite3Config.b_full_mutex = 1 as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.mutex =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    } = sqlite3Config.mutex
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.m =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                10 => {
                    {
                        sqlite3Config.mutex =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    } = sqlite3Config.mutex
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.m =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                11 => {
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MutexMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MutexMethods)
                                    } = sqlite3Config.mutex
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.m =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                4 => {
                    {
                        sqlite3Config.m =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                5 => {
                    {
                        if !sqlite3Config.m.x_malloc.is_some() as i32 != 0 {
                            unsafe { sqlite3_mem_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3MemMethods>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3MemMethods)
                                    } = sqlite3Config.m
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                9 => {
                    {
                        { let _ = 0; };
                        sqlite3Config.b_memstat =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                27 => {
                    {
                        sqlite3Config.b_small_malloc =
                            (unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } != 0) as u8;
                        break '__s14;
                    }
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                7 => {
                    {
                        sqlite3Config.p_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        sqlite3Config.sz_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_page =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                24 => {
                    {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } =
                                unsafe { sqlite3_header_size_btree() } +
                                        unsafe { sqlite3_header_size_pcache() } +
                                    unsafe { sqlite3_header_size_pcache1() }
                        };
                        break '__s14;
                    }
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                14 => {
                    { break '__s14; }
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                15 => {
                    { rc = 1; break '__s14; }
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                18 => {
                    {
                        sqlite3Config.pcache2 =
                            unsafe {
                                core::ptr::read(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    })
                            };
                        break '__s14;
                    }
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                19 => {
                    {
                        if !sqlite3Config.pcache2.x_init.is_some() as i32 != 0 {
                            unsafe { sqlite3_p_cache_set_default() };
                        }
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3PcacheMethods2>()
                                                            + 7) & !7);
                                        *(__va_p as *const *mut Sqlite3PcacheMethods2)
                                    } = sqlite3Config.pcache2
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                13 => {
                    {
                        sqlite3Config.sz_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        sqlite3Config.n_lookaside =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                16 => {
                    {
                        let x_log:
                                Option<unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()> =
                            Some(unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*mut (),
                                                                    i32, *const i8) -> ()>() + 7) & !7);
                                    *(__va_p as
                                            *const unsafe extern "C" fn(*mut (), i32, *const i8) -> ())
                                });
                        let p_log_arg: *mut () =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                                *(__va_p as *const *mut ())
                            };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.x_log
                                        as
                                        *mut *mut ()).store(x_log.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), i32, *const i8)
                                                        -> ()>(0 as *const ())
                                        }) as *mut (), std::sync::atomic::Ordering::Relaxed)
                        };
                        unsafe {
                            std::sync::atomic::AtomicPtr::from_ptr(&raw mut sqlite3Config.p_log_arg
                                        as
                                        *mut *mut ()).store(p_log_arg as *mut (),
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                17 => {
                    {
                        let b_open_uri: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        unsafe {
                            std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                        as
                                        *mut u8).store(b_open_uri as u8,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        break '__s14;
                    }
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                20 => {
                    {
                        sqlite3Config.b_use_cis =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as u8;
                        break '__s14;
                    }
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                22 => {
                    {
                        let mut sz_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        let mut mx_mmap: Sqlite3Int64 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        if mx_mmap < 0 as i64 || mx_mmap > 2147418112 as i64 {
                            mx_mmap = 2147418112 as Sqlite3Int64;
                        }
                        if sz_mmap < 0 as i64 { sz_mmap = 0 as Sqlite3Int64; }
                        if sz_mmap > mx_mmap { sz_mmap = mx_mmap; }
                        sqlite3Config.mx_mmap = mx_mmap;
                        sqlite3Config.sz_mmap = sz_mmap;
                        break '__s14;
                    }
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                25 => {
                    {
                        sqlite3Config.sz_pma =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                26 => {
                    {
                        sqlite3Config.n_stmt_spill =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        break '__s14;
                    }
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                29 => {
                    {
                        sqlite3Config.mx_memdb_size =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<Sqlite3Int64>() + 7) &
                                            !7);
                                *(__va_p as *const Sqlite3Int64)
                            };
                        break '__s14;
                    }
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                30 => {
                    {
                        let p_val: *mut i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                *(__va_p as *const *mut i32)
                            };
                        unsafe { *p_val = 0 };
                        break '__s14;
                    }
                    { rc = 1; break '__s14; }
                }
                _ => { { rc = 1; break '__s14; } }
            }
        }
        ();
        return rc;
    }
}
extern "C" fn setup_lookaside(db: *mut Sqlite3, p_buf_1: *mut (), mut sz: i32,
    mut cnt: i32) -> i32 {
    unsafe {
        let mut p_start: *mut () = core::ptr::null_mut();
        let mut sz_alloc: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut n_big: i32 = 0;
        let mut n_sm: i32 = 0;
        if unsafe { sqlite3_lookaside_used(db, core::ptr::null_mut()) } > 0 {
            return 5;
        }
        if unsafe { (*db).lookaside.b_malloced } != 0 {
            unsafe { sqlite3_free(unsafe { (*db).lookaside.p_start }) };
        }
        sz = sz & !7;
        if sz <= core::mem::size_of::<*mut LookasideSlot>() as i32 { sz = 0; }
        if sz > 65528 { sz = 65528; }
        if cnt < 1 { cnt = 0; }
        if sz > 0 && cnt > 2147418112 / sz { cnt = 2147418112 / sz; }
        sz_alloc = sz as i64 * cnt as i64;
        if sz_alloc == 0 as i64 {
            sz = 0;
            p_start = core::ptr::null_mut();
        } else if p_buf_1 == core::ptr::null_mut() {
            unsafe { sqlite3_begin_benign_malloc() };
            p_start = unsafe { sqlite3Malloc(sz_alloc as u64) };
            unsafe { sqlite3_end_benign_malloc() };
            if !(p_start).is_null() {
                sz_alloc =
                    unsafe { sqlite3_malloc_size(p_start as *const ()) } as
                        Sqlite3Int64;
            }
        } else { p_start = p_buf_1; }
        if sz >= 128 * 3 {
            n_big = (sz_alloc / (3 * 128 + sz) as Sqlite3Int64) as i32;
            n_sm =
                ((sz_alloc - sz as i64 * n_big as i64) / 128 as SqliteInt64)
                    as i32;
        } else if sz >= 128 * 2 {
            n_big = (sz_alloc / (128 + sz) as Sqlite3Int64) as i32;
            n_sm =
                ((sz_alloc - sz as i64 * n_big as i64) / 128 as SqliteInt64)
                    as i32;
        } else if sz > 0 {
            n_big = (sz_alloc / sz as Sqlite3Int64) as i32;
            n_sm = 0;
        } else { n_big = { n_sm = 0; n_sm }; }
        unsafe { (*db).lookaside.p_start = p_start };
        unsafe { (*db).lookaside.p_init = core::ptr::null_mut() };
        unsafe { (*db).lookaside.p_free = core::ptr::null_mut() };
        unsafe { (*db).lookaside.sz = sz as u16 };
        unsafe { (*db).lookaside.sz_true = sz as u16 };
        if !(p_start).is_null() {
            let mut i: i32 = 0;
            let mut p: *mut LookasideSlot = core::ptr::null_mut();
            { let _ = 0; };
            p = p_start as *mut LookasideSlot;
            {
                i = 0;
                '__b15: loop {
                    if !(i < n_big) { break '__b15; }
                    '__c15: loop {
                        unsafe { (*p).p_next = unsafe { (*db).lookaside.p_init } };
                        unsafe { (*db).lookaside.p_init = p };
                        p =
                            unsafe { &raw mut *(p as *mut u8).offset(sz as isize) } as
                                *mut LookasideSlot;
                        break '__c15;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*db).lookaside.p_small_init = core::ptr::null_mut() };
            unsafe { (*db).lookaside.p_small_free = core::ptr::null_mut() };
            unsafe { (*db).lookaside.p_middle = p as *mut () };
            {
                i = 0;
                '__b16: loop {
                    if !(i < n_sm) { break '__b16; }
                    '__c16: loop {
                        unsafe {
                            (*p).p_next = unsafe { (*db).lookaside.p_small_init }
                        };
                        unsafe { (*db).lookaside.p_small_init = p };
                        p =
                            unsafe { &raw mut *(p as *mut u8).offset(128 as isize) } as
                                *mut LookasideSlot;
                        break '__c16;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            { let _ = 0; };
            unsafe { (*db).lookaside.p_end = p as *mut () };
            unsafe { (*db).lookaside.b_disable = 0 as u32 };
            unsafe {
                (*db).lookaside.b_malloced =
                    if p_buf_1 == core::ptr::null_mut() { 1 } else { 0 } as u8
            };
            unsafe { (*db).lookaside.n_slot = (n_big + n_sm) as u32 };
        } else {
            unsafe { (*db).lookaside.p_start = core::ptr::null_mut() };
            unsafe { (*db).lookaside.p_small_init = core::ptr::null_mut() };
            unsafe { (*db).lookaside.p_small_free = core::ptr::null_mut() };
            unsafe { (*db).lookaside.p_middle = core::ptr::null_mut() };
            unsafe { (*db).lookaside.p_end = core::ptr::null_mut() };
            unsafe { (*db).lookaside.b_disable = 1 as u32 };
            unsafe { (*db).lookaside.sz = 0 as u16 };
            unsafe { (*db).lookaside.b_malloced = 0 as u8 };
            unsafe { (*db).lookaside.n_slot = 0 as u32 };
        }
        unsafe {
            (*db).lookaside.p_true_end = unsafe { (*db).lookaside.p_end }
        };
        { let _ = 0; };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_db_config(db: *mut Sqlite3, op: i32,
    mut __va0: ...) -> i32 {
    let mut ap: *const i8 = core::ptr::null();
    let mut rc: i32 = 0;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    '__s17:
        {
        match op {
            1000 => {
                {
                    unsafe {
                        (*unsafe { (*db).a_db.offset(0 as isize) }).z_db_s_name =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i8>() + 7) & !7);
                                *(__va_p as *const *mut i8)
                            }
                    };
                    rc = 0;
                    break '__s17;
                }
                {
                    let p_buf: *mut () =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap =
                                (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                            *(__va_p as *const *mut ())
                        };
                    let sz: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    let cnt: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    rc = setup_lookaside(db, p_buf, sz, cnt);
                    break '__s17;
                }
                {
                    let n_in: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    let p_out: *mut i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap =
                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                            *(__va_p as *const *mut i32)
                        };
                    if n_in > 3 && n_in < 24 {
                        unsafe { (*db).n_fp_digit = n_in as u8 };
                    }
                    if !(p_out).is_null() {
                        unsafe { *p_out = unsafe { (*db).n_fp_digit } as i32 };
                    }
                    rc = 0;
                    break '__s17;
                }
                {
                    let mut i: u32 = 0 as u32;
                    rc = 1;
                    {
                        i = 0 as u32;
                        '__b18: loop {
                            if !(i <
                                            (core::mem::size_of::<[Sqlite3DbConfigS0N20sqlite3DbConfigS0; 21]>()
                                                            as u64 / 16) as i32 as u32) {
                                break '__b18;
                            }
                            '__c18: loop {
                                if a_flag_op[i as usize].op as i32 == op {
                                    let onoff: i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                            *(__va_p as *const i32)
                                        };
                                    let p_res: *mut i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                            *(__va_p as *const *mut i32)
                                        };
                                    let old_flags: u64 = unsafe { (*db).flags };
                                    if onoff > 0 {
                                        unsafe { (*db).flags |= a_flag_op[i as usize].mask as u64 };
                                    } else if onoff == 0 {
                                        unsafe {
                                            (*db).flags &= !(a_flag_op[i as usize].mask as u64)
                                        };
                                    }
                                    if old_flags != unsafe { (*db).flags } {
                                        unsafe { sqlite3_expire_prepared_statements(db, 0) };
                                    }
                                    if !(p_res).is_null() {
                                        unsafe {
                                            *p_res =
                                                (unsafe { (*db).flags } & a_flag_op[i as usize].mask as u64
                                                        != 0 as u64) as i32
                                        };
                                    }
                                    rc = 0;
                                    break '__b18;
                                }
                                break '__c18;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s17;
                }
            }
            1001 => {
                {
                    let p_buf: *mut () =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap =
                                (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                            *(__va_p as *const *mut ())
                        };
                    let sz: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    let cnt: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    rc = setup_lookaside(db, p_buf, sz, cnt);
                    break '__s17;
                }
                {
                    let n_in: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    let p_out: *mut i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap =
                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                            *(__va_p as *const *mut i32)
                        };
                    if n_in > 3 && n_in < 24 {
                        unsafe { (*db).n_fp_digit = n_in as u8 };
                    }
                    if !(p_out).is_null() {
                        unsafe { *p_out = unsafe { (*db).n_fp_digit } as i32 };
                    }
                    rc = 0;
                    break '__s17;
                }
                {
                    let mut i: u32 = 0 as u32;
                    rc = 1;
                    {
                        i = 0 as u32;
                        '__b18: loop {
                            if !(i <
                                            (core::mem::size_of::<[Sqlite3DbConfigS0N20sqlite3DbConfigS0; 21]>()
                                                            as u64 / 16) as i32 as u32) {
                                break '__b18;
                            }
                            '__c18: loop {
                                if a_flag_op[i as usize].op as i32 == op {
                                    let onoff: i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                            *(__va_p as *const i32)
                                        };
                                    let p_res: *mut i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                            *(__va_p as *const *mut i32)
                                        };
                                    let old_flags: u64 = unsafe { (*db).flags };
                                    if onoff > 0 {
                                        unsafe { (*db).flags |= a_flag_op[i as usize].mask as u64 };
                                    } else if onoff == 0 {
                                        unsafe {
                                            (*db).flags &= !(a_flag_op[i as usize].mask as u64)
                                        };
                                    }
                                    if old_flags != unsafe { (*db).flags } {
                                        unsafe { sqlite3_expire_prepared_statements(db, 0) };
                                    }
                                    if !(p_res).is_null() {
                                        unsafe {
                                            *p_res =
                                                (unsafe { (*db).flags } & a_flag_op[i as usize].mask as u64
                                                        != 0 as u64) as i32
                                        };
                                    }
                                    rc = 0;
                                    break '__b18;
                                }
                                break '__c18;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s17;
                }
            }
            1023 => {
                {
                    let n_in: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    let p_out: *mut i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap =
                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                            *(__va_p as *const *mut i32)
                        };
                    if n_in > 3 && n_in < 24 {
                        unsafe { (*db).n_fp_digit = n_in as u8 };
                    }
                    if !(p_out).is_null() {
                        unsafe { *p_out = unsafe { (*db).n_fp_digit } as i32 };
                    }
                    rc = 0;
                    break '__s17;
                }
                {
                    let mut i: u32 = 0 as u32;
                    rc = 1;
                    {
                        i = 0 as u32;
                        '__b18: loop {
                            if !(i <
                                            (core::mem::size_of::<[Sqlite3DbConfigS0N20sqlite3DbConfigS0; 21]>()
                                                            as u64 / 16) as i32 as u32) {
                                break '__b18;
                            }
                            '__c18: loop {
                                if a_flag_op[i as usize].op as i32 == op {
                                    let onoff: i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                            *(__va_p as *const i32)
                                        };
                                    let p_res: *mut i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                            *(__va_p as *const *mut i32)
                                        };
                                    let old_flags: u64 = unsafe { (*db).flags };
                                    if onoff > 0 {
                                        unsafe { (*db).flags |= a_flag_op[i as usize].mask as u64 };
                                    } else if onoff == 0 {
                                        unsafe {
                                            (*db).flags &= !(a_flag_op[i as usize].mask as u64)
                                        };
                                    }
                                    if old_flags != unsafe { (*db).flags } {
                                        unsafe { sqlite3_expire_prepared_statements(db, 0) };
                                    }
                                    if !(p_res).is_null() {
                                        unsafe {
                                            *p_res =
                                                (unsafe { (*db).flags } & a_flag_op[i as usize].mask as u64
                                                        != 0 as u64) as i32
                                        };
                                    }
                                    rc = 0;
                                    break '__b18;
                                }
                                break '__c18;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s17;
                }
            }
            _ => {
                {
                    let mut i: u32 = 0 as u32;
                    rc = 1;
                    {
                        i = 0 as u32;
                        '__b18: loop {
                            if !(i <
                                            (core::mem::size_of::<[Sqlite3DbConfigS0N20sqlite3DbConfigS0; 21]>()
                                                            as u64 / 16) as i32 as u32) {
                                break '__b18;
                            }
                            '__c18: loop {
                                if a_flag_op[i as usize].op as i32 == op {
                                    let onoff: i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                            *(__va_p as *const i32)
                                        };
                                    let p_res: *mut i32 =
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                            *(__va_p as *const *mut i32)
                                        };
                                    let old_flags: u64 = unsafe { (*db).flags };
                                    if onoff > 0 {
                                        unsafe { (*db).flags |= a_flag_op[i as usize].mask as u64 };
                                    } else if onoff == 0 {
                                        unsafe {
                                            (*db).flags &= !(a_flag_op[i as usize].mask as u64)
                                        };
                                    }
                                    if old_flags != unsafe { (*db).flags } {
                                        unsafe { sqlite3_expire_prepared_statements(db, 0) };
                                    }
                                    if !(p_res).is_null() {
                                        unsafe {
                                            *p_res =
                                                (unsafe { (*db).flags } & a_flag_op[i as usize].mask as u64
                                                        != 0 as u64) as i32
                                        };
                                    }
                                    rc = 0;
                                    break '__b18;
                                }
                                break '__c18;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s17;
                }
            }
        }
    }
    ();
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_extended_result_codes(db: &mut Sqlite3, onoff: i32)
    -> i32 {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    (*db).err_mask =
        if onoff != 0 { 4294967295u32 } else { 255 as u32 } as i32;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_last_insert_rowid(db: &Sqlite3) -> Sqlite3Int64 {
    let mut i_ret: i64 = 0 as i64;
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    i_ret = (*db).last_rowid;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return i_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_set_last_insert_rowid(db: &mut Sqlite3,
    i_rowid_1: Sqlite3Int64) -> () {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    (*db).last_rowid = i_rowid_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_changes64(db: &Sqlite3) -> Sqlite3Int64 {
    let mut i_ret: i64 = 0 as i64;
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    i_ret = (*db).n_change;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return i_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_changes(db: *mut Sqlite3) -> i32 {
    return sqlite3_changes64(unsafe { &*db }) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_total_changes64(db: &Sqlite3) -> Sqlite3Int64 {
    let mut i_ret: i64 = 0 as i64;
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    i_ret = (*db).n_total_change;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return i_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_total_changes(db: *mut Sqlite3) -> i32 {
    return sqlite3_total_changes64(unsafe { &*db }) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_interrupt(db: &mut Sqlite3) -> () {
    unsafe {
        unsafe {
            std::sync::atomic::AtomicI32::from_ptr(&raw mut (*db).u1.is_interrupted
                        as
                        *mut i32).store(1 as i32,
                std::sync::atomic::Ordering::Relaxed)
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_is_interrupted(db: &mut Sqlite3) -> i32 {
    unsafe {
        return (unsafe {
                        std::sync::atomic::AtomicI32::from_ptr(&raw mut (*db).u1.is_interrupted
                                    as *mut i32).load(std::sync::atomic::Ordering::Relaxed)
                    } != 0) as i32;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_busy_handler(db: &mut Sqlite3,
    x_busy_1: Option<unsafe extern "C" fn(*mut (), i32) -> i32>,
    p_arg_1: *mut ()) -> i32 {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    (*db).busy_handler.x_busy_handler = x_busy_1;
    (*db).busy_handler.p_busy_arg = p_arg_1;
    (*db).busy_handler.n_busy = 0;
    (*db).busy_timeout = 0;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return 0;
}
extern "C" fn sqlite_default_busy_callback(ptr: *mut (), count: i32) -> i32 {
    let db: *const Sqlite3 = ptr as *mut Sqlite3 as *const Sqlite3;
    let tmout: i32 = unsafe { (*db).busy_timeout };
    let mut delay: i32 = 0;
    let mut prior: i32 = 0;
    { let _ = 0; };
    if count <
            (core::mem::size_of::<[u8; 12]>() as u64 /
                    core::mem::size_of::<u8>() as u64) as i32 {
        delay = delays[count as usize] as i32;
        prior = totals[count as usize] as i32;
    } else {
        delay =
            delays[((core::mem::size_of::<[u8; 12]>() as u64 /
                                    core::mem::size_of::<u8>() as u64) as i32 - 1) as usize] as
                i32;
        prior =
            totals[((core::mem::size_of::<[u8; 12]>() as u64 /
                                        core::mem::size_of::<u8>() as u64) as i32 - 1) as usize] as
                    i32 +
                delay *
                    (count -
                        ((core::mem::size_of::<[u8; 12]>() as u64 /
                                    core::mem::size_of::<u8>() as u64) as i32 - 1));
    }
    if prior + delay > tmout {
        delay = tmout - prior;
        if delay <= 0 { return 0; }
    }
    unsafe { sqlite3_os_sleep(unsafe { (*db).p_vfs }, delay * 1000) };
    return 1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_busy_timeout(db: *mut Sqlite3, ms: i32) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if ms > 0 {
        sqlite3_busy_handler(unsafe { &mut *db },
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut (), i32)
                                -> i32>(sqlite_default_busy_callback as *const ())
                }), db as *mut ());
        unsafe { (*db).busy_timeout = ms };
    } else {
        sqlite3_busy_handler(unsafe { &mut *db }, None,
            core::ptr::null_mut());
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_setlk_timeout(db: *const Sqlite3, ms: i32,
    flags: i32) -> i32 {
    if ms < -1 { return 25; }
    { let _ = db; };
    { let _ = flags; };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trace(db: &mut Sqlite3,
    x_trace_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    p_arg_1: *mut ()) -> *mut () {
    unsafe {
        let mut p_old: *mut () = core::ptr::null_mut();
        unsafe { sqlite3_mutex_enter((*db).mutex) };
        p_old = (*db).p_trace_arg;
        (*db).m_trace = if x_trace_1.is_some() { 64 } else { 0 } as u8;
        (*db).trace.x_legacy = x_trace_1;
        (*db).p_trace_arg = p_arg_1;
        unsafe { sqlite3_mutex_leave((*db).mutex) };
        return p_old;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_profile(db: &mut Sqlite3,
    x_profile_1: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    p_arg_1: *mut ()) -> *mut () {
    let mut p_old: *mut () = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    p_old = (*db).p_profile_arg;
    (*db).x_profile = x_profile_1;
    (*db).p_profile_arg = p_arg_1;
    (*db).m_trace &= 15 as u8;
    if (*db).x_profile.is_some() { (*db).m_trace |= 128 as u8; }
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return p_old;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trace_v2(db: &mut Sqlite3, mut m_trace_1: u32,
    mut x_trace_1:
        Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    p_arg_1: *mut ()) -> i32 {
    unsafe {
        unsafe { sqlite3_mutex_enter((*db).mutex) };
        if m_trace_1 == 0 as u32 { x_trace_1 = None; }
        if !x_trace_1.is_some() as i32 != 0 { m_trace_1 = 0 as u32; }
        (*db).m_trace = m_trace_1 as u8;
        (*db).trace.x_v2 = x_trace_1;
        (*db).p_trace_arg = p_arg_1;
        unsafe { sqlite3_mutex_leave((*db).mutex) };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_progress_handler(db: &mut Sqlite3, n_ops_1: i32,
    x_progress_1: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_arg_1: *mut ()) -> () {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    if n_ops_1 > 0 {
        (*db).x_progress = x_progress_1;
        (*db).n_progress_ops = n_ops_1 as u32;
        (*db).p_progress_arg = p_arg_1;
    } else {
        (*db).x_progress = None;
        (*db).n_progress_ops = 0 as u32;
        (*db).p_progress_arg = core::ptr::null_mut();
    }
    unsafe { sqlite3_mutex_leave((*db).mutex) };
}
static a_hard_limit: [i32; 13] =
    [1000000000, 1000000000, 2000, 1000, 500, 250000000, 1000, 10, 50000,
            32766, 1000, 8, 2500];
extern "C" fn create_collation(db: *mut Sqlite3, z_name_1: *const i8, enc: u8,
    p_ctx_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32 {
    let mut p_coll: *mut CollSeq = core::ptr::null_mut();
    let mut enc2: i32 = 0;
    { let _ = 0; };
    enc2 = enc as i32;
    if enc2 == 4 || enc2 == 8 { enc2 = 2; }
    if enc2 < 1 || enc2 > 3 { return sqlite3_misuse_error(2911); }
    p_coll = unsafe { sqlite3_find_coll_seq(db, enc2 as u8, z_name_1, 0) };
    if !(p_coll).is_null() && unsafe { (*p_coll).x_cmp.is_some() } {
        if unsafe { (*db).n_vdbe_active } != 0 {
            unsafe {
                sqlite3_error_with_msg(db, 5,
                    c"unable to delete/modify collation sequence due to active statements".as_ptr()
                            as *mut i8 as *const i8)
            };
            return 5;
        }
        unsafe { sqlite3_expire_prepared_statements(db, 0) };
        if unsafe { (*p_coll).enc } as i32 & !8 == enc2 {
            let a_coll: *mut CollSeq =
                unsafe {
                        sqlite3_hash_find(unsafe { &raw mut (*db).a_coll_seq } as
                                *const Hash, z_name_1)
                    } as *mut CollSeq;
            let mut j: i32 = 0;
            {
                j = 0;
                '__b19: loop {
                    if !(j < 3) { break '__b19; }
                    '__c19: loop {
                        let p: *mut CollSeq =
                            unsafe { &mut *a_coll.offset(j as isize) };
                        if unsafe { (*p).enc } as i32 ==
                                unsafe { (*p_coll).enc } as i32 {
                            if unsafe { (*p).x_del.is_some() } {
                                unsafe {
                                    (unsafe { (*p).x_del.unwrap() })(unsafe { (*p).p_user })
                                };
                            }
                            unsafe { (*p).x_cmp = None };
                        }
                        break '__c19;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
    p_coll = unsafe { sqlite3_find_coll_seq(db, enc2 as u8, z_name_1, 1) };
    if p_coll == core::ptr::null_mut() { return 7; }
    unsafe { (*p_coll).x_cmp = x_compare_1 };
    unsafe { (*p_coll).p_user = p_ctx_1 };
    unsafe { (*p_coll).x_del = x_del_1 };
    unsafe { (*p_coll).enc = (enc2 | enc as i32 & 8) as u8 };
    unsafe { sqlite3_error(db, 0) };
    return 0;
}
extern "C" fn bin_coll_func(not_used_1: *mut (), n_key1_1: i32,
    p_key1_1: *const (), n_key2_1: i32, p_key2_1: *const ()) -> i32 {
    let mut rc: i32 = 0;
    let mut n: i32 = 0;
    { let _ = not_used_1; };
    n = if n_key1_1 < n_key2_1 { n_key1_1 } else { n_key2_1 };
    { let _ = 0; };
    rc = unsafe { memcmp(p_key1_1, p_key2_1, n as u64) };
    if rc == 0 { rc = n_key1_1 - n_key2_1; }
    return rc;
}
extern "C" fn nocase_collating_func(not_used_1: *mut (), n_key1_1: i32,
    p_key1_1: *const (), n_key2_1: i32, p_key2_1: *const ()) -> i32 {
    let mut r: i32 =
        unsafe {
            sqlite3_strnicmp(p_key1_1 as *const i8, p_key2_1 as *const i8,
                if n_key1_1 < n_key2_1 { n_key1_1 } else { n_key2_1 })
        };
    { let _ = not_used_1; };
    if 0 == r { r = n_key1_1 - n_key2_1; }
    return r;
}
extern "C" fn rtrim_coll_func(p_user_1: *mut (), mut n_key1_1: i32,
    p_key1_1: *const (), mut n_key2_1: i32, p_key2_1: *const ()) -> i32 {
    let p_k1: *const u8 = p_key1_1 as *const u8;
    let p_k2: *const u8 = p_key2_1 as *const u8;
    while n_key1_1 != 0 &&
            unsafe { *p_k1.offset((n_key1_1 - 1) as isize) } as i32 ==
                ' ' as i32 {
        { let __p = &mut n_key1_1; let __t = *__p; *__p -= 1; __t };
    }
    while n_key2_1 != 0 &&
            unsafe { *p_k2.offset((n_key2_1 - 1) as isize) } as i32 ==
                ' ' as i32 {
        { let __p = &mut n_key2_1; let __t = *__p; *__p -= 1; __t };
    }
    return bin_coll_func(p_user_1, n_key1_1, p_key1_1, n_key2_1, p_key2_1);
}
extern "C" fn database_name(mut z_name_1: *const i8) -> *const i8 {
    while unsafe { *z_name_1.offset(-1 as isize) } as i32 != 0 ||
                    unsafe { *z_name_1.offset(-2 as isize) } as i32 != 0 ||
                unsafe { *z_name_1.offset(-3 as isize) } as i32 != 0 ||
            unsafe { *z_name_1.offset(-4 as isize) } as i32 != 0 {
        {
            let __p = &mut z_name_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(-1) };
            __t
        };
    }
    return z_name_1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_free_filename(mut p: Sqlite3Filename) -> () {
    if p == core::ptr::null() { return; }
    p = database_name(p);
    unsafe {
        sqlite3_free(unsafe { (p as *mut i8).offset(-(4 as isize)) } as
                *mut ())
    };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_parse_uri(z_default_vfs_1: *const i8,
    z_uri_1: *const i8, p_flags_1: &mut u32, pp_vfs_1: &mut *mut Sqlite3Vfs,
    pz_file_1: &mut *mut i8, pz_err_msg_1: &mut *mut i8) -> i32 {
    unsafe {
        unsafe {
            let mut rc: i32 = 0;
            let mut flags: u32 = 0 as u32;
            let mut z_vfs: *const i8 = core::ptr::null();
            let mut z_file: *mut i8 = core::ptr::null_mut();
            let mut c: i8 = 0 as i8;
            let mut n_uri: i64 = 0 as i64;
            let mut z_opt: *mut i8 = core::ptr::null_mut();
            let mut e_state: i32 = 0;
            let mut i_in: i64 = 0 as i64;
            let mut i_out: i64 = 0 as i64;
            let mut n_byte: u64 = 0 as u64;
            let mut octet: i32 = 0;
            let mut n_opt: i64 = 0 as i64;
            let mut z_val: *mut i8 = core::ptr::null_mut();
            let mut n_val: i64 = 0 as i64;
            let mut a_mode: *const OpenModeN8OpenMode = core::ptr::null();
            let mut z_mode_type: *mut i8 = core::ptr::null_mut();
            let mut mask: i32 = 0;
            let mut limit: i32 = 0;
            let mut i: i32 = 0;
            let mut mode: i32 = 0;
            let mut z: *const i8 = core::ptr::null();
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s24:
                    {
                    match __state {
                        0 => { rc = 0; __state = 3; }
                        2 => {
                            if rc != 0 { __state = 120; } else { __state = 119; }
                        }
                        3 => { flags = *p_flags_1; __state = 4; }
                        4 => { z_vfs = z_default_vfs_1; __state = 5; }
                        5 => { __state = 6; }
                        6 => { __state = 7; }
                        7 => {
                            n_uri = unsafe { strlen(z_uri_1) } as i64;
                            __state = 8;
                        }
                        8 => { { let _ = 0; }; __state = 9; }
                        9 => {
                            if (flags & 64 as u32 != 0 ||
                                            unsafe {
                                                    std::sync::atomic::AtomicU8::from_ptr(&raw mut sqlite3Config.b_open_uri
                                                                as *mut u8).load(std::sync::atomic::Ordering::Relaxed)
                                                } != 0) && n_uri >= 5 as i64 &&
                                    unsafe {
                                            memcmp(z_uri_1 as *const (),
                                                c"file:".as_ptr() as *mut i8 as *const (), 5 as u64)
                                        } == 0 {
                                __state = 11;
                            } else { __state = 12; }
                        }
                        10 => {
                            *pp_vfs_1 = unsafe { sqlite3_vfs_find(z_vfs) };
                            __state = 115;
                        }
                        11 => { __state = 13; }
                        12 => {
                            z_file =
                                unsafe {
                                        sqlite3_malloc64((n_uri + 8 as i64) as Sqlite3Uint64)
                                    } as *mut i8;
                            __state = 107;
                        }
                        13 => { __state = 14; }
                        14 => { __state = 15; }
                        15 => { i_out = 0 as i64; __state = 16; }
                        16 => { n_byte = (n_uri + 8 as i64) as u64; __state = 17; }
                        17 => { flags |= 64 as u32; __state = 18; }
                        18 => { i_in = 0 as i64; __state = 20; }
                        19 => {
                            z_file = unsafe { sqlite3_malloc64(n_byte) } as *mut i8;
                            __state = 23;
                        }
                        20 => {
                            if i_in < n_uri { __state = 21; } else { __state = 19; }
                        }
                        21 => {
                            n_byte +=
                                (unsafe { *z_uri_1.offset(i_in as isize) } as i32 ==
                                        '&' as i32) as u64;
                            __state = 22;
                        }
                        22 => {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                            __state = 20;
                        }
                        23 => {
                            if (z_file).is_null() as i32 != 0 {
                                __state = 25;
                            } else { __state = 24; }
                        }
                        24 => {
                            unsafe { memset(z_file as *mut (), 0, 4 as u64) };
                            __state = 26;
                        }
                        25 => { return 7; }
                        26 => {
                            {
                                let __n = 4;
                                let __p = &mut z_file;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            __state = 27;
                        }
                        27 => { i_in = 5 as i64; __state = 28; }
                        28 => {
                            if unsafe { *z_uri_1.offset(5 as isize) } as i32 ==
                                        '/' as i32 &&
                                    unsafe { *z_uri_1.offset(6 as isize) } as i32 == '/' as i32
                                {
                                __state = 30;
                            } else { __state = 29; }
                        }
                        29 => { e_state = 0; __state = 37; }
                        30 => { i_in = 7 as i64; __state = 31; }
                        31 => {
                            if unsafe { *z_uri_1.offset(i_in as isize) } != 0 &&
                                    unsafe { *z_uri_1.offset(i_in as isize) } as i32 !=
                                        '/' as i32 {
                                __state = 33;
                            } else { __state = 32; }
                        }
                        32 => {
                            if i_in != 7 as i64 &&
                                    (i_in != 16 as i64 ||
                                        unsafe {
                                                memcmp(c"localhost".as_ptr() as *mut i8 as *const (),
                                                    unsafe { &raw const *z_uri_1.offset(7 as isize) } as
                                                        *const (), 9 as u64)
                                            } != 0) {
                                __state = 34;
                            } else { __state = 29; }
                        }
                        33 => {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                            __state = 31;
                        }
                        34 => {
                            *pz_err_msg_1 =
                                unsafe {
                                    sqlite3_mprintf(c"invalid uri authority: %.*s".as_ptr() as
                                                *mut i8 as *const i8, (i_in - 7 as i64) as i32,
                                        unsafe { &raw const *z_uri_1.offset(7 as isize) } as
                                            *const i8)
                                };
                            __state = 35;
                        }
                        35 => { rc = 1; __state = 36; }
                        36 => { __state = 2; }
                        37 => {
                            if {
                                                c = unsafe { *z_uri_1.offset(i_in as isize) } as i8;
                                                c
                                            } as i32 != 0 && c as i32 != '#' as i32 {
                                __state = 39;
                            } else { __state = 38; }
                        }
                        38 => {
                            if e_state == 1 { __state = 63; } else { __state = 62; }
                        }
                        39 => {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                            __state = 40;
                        }
                        40 => {
                            if c as i32 == '%' as i32 &&
                                        unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z_uri_1.offset(i_in as isize) } as
                                                                        u8 as usize)
                                                    } as i32 & 8 != 0 &&
                                    unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe {
                                                                        *z_uri_1.offset((i_in + 1 as i64) as isize)
                                                                    } as u8 as usize)
                                                } as i32 & 8 != 0 {
                                __state = 42;
                            } else { __state = 43; }
                        }
                        41 => {
                            unsafe {
                                *z_file.offset({
                                                    let __p = &mut i_out;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) = c
                            };
                            __state = 37;
                        }
                        42 => {
                            octet =
                                (unsafe {
                                                sqlite3_hex_to_int(unsafe {
                                                            *z_uri_1.offset({
                                                                            let __p = &mut i_in;
                                                                            let __t = *__p;
                                                                            *__p += 1;
                                                                            __t
                                                                        } as isize)
                                                        } as i32)
                                            } as i32) << 4;
                            __state = 44;
                        }
                        43 => {
                            if e_state == 1 &&
                                    (c as i32 == '&' as i32 || c as i32 == '=' as i32) {
                                __state = 51;
                            } else { __state = 52; }
                        }
                        44 => {
                            octet +=
                                unsafe {
                                        sqlite3_hex_to_int(unsafe {
                                                    *z_uri_1.offset({
                                                                    let __p = &mut i_in;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize)
                                                } as i32)
                                    } as i32;
                            __state = 45;
                        }
                        45 => { { let _ = 0; }; __state = 46; }
                        46 => {
                            if octet == 0 { __state = 48; } else { __state = 47; }
                        }
                        47 => { c = octet as i8; __state = 41; }
                        48 => {
                            if {
                                                            c = unsafe { *z_uri_1.offset(i_in as isize) } as i8;
                                                            c
                                                        } as i32 != 0 && c as i32 != '#' as i32 &&
                                            (e_state != 0 || c as i32 != '?' as i32) &&
                                        (e_state != 1 ||
                                            c as i32 != '=' as i32 && c as i32 != '&' as i32) &&
                                    (e_state != 2 || c as i32 != '&' as i32) {
                                __state = 50;
                            } else { __state = 49; }
                        }
                        49 => { __state = 37; }
                        50 => {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                            __state = 48;
                        }
                        51 => {
                            if unsafe { *z_file.offset((i_out - 1 as i64) as isize) } as
                                        i32 == 0 {
                                __state = 54;
                            } else { __state = 53; }
                        }
                        52 => {
                            if e_state == 0 && c as i32 == '?' as i32 ||
                                    e_state == 2 && c as i32 == '&' as i32 {
                                __state = 60;
                            } else { __state = 41; }
                        }
                        53 => {
                            if c as i32 == '&' as i32 {
                                __state = 58;
                            } else { __state = 59; }
                        }
                        54 => {
                            if unsafe { *z_uri_1.offset(i_in as isize) } != 0 &&
                                        unsafe { *z_uri_1.offset(i_in as isize) } as i32 !=
                                            '#' as i32 &&
                                    unsafe { *z_uri_1.offset((i_in - 1 as i64) as isize) } as
                                            i32 != '&' as i32 {
                                __state = 56;
                            } else { __state = 55; }
                        }
                        55 => { __state = 37; }
                        56 => {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                            __state = 54;
                        }
                        57 => { c = 0 as i8; __state = 41; }
                        58 => {
                            unsafe {
                                *z_file.offset({
                                                    let __p = &mut i_out;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) = '\u{0}' as i32 as i8
                            };
                            __state = 57;
                        }
                        59 => { e_state = 2; __state = 57; }
                        60 => { c = 0 as i8; __state = 61; }
                        61 => { e_state = 1; __state = 41; }
                        62 => {
                            unsafe {
                                memset(unsafe { z_file.offset(i_out as isize) } as *mut (),
                                    0, 4 as u64)
                            };
                            __state = 64;
                        }
                        63 => {
                            unsafe {
                                *z_file.offset({
                                                    let __p = &mut i_out;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) = '\u{0}' as i32 as i8
                            };
                            __state = 62;
                        }
                        64 => {
                            z_opt =
                                unsafe {
                                    z_file.add((unsafe { strlen(z_file as *const i8) } +
                                                1 as u64) as usize)
                                };
                            __state = 65;
                        }
                        65 => {
                            if unsafe { *z_opt.offset(0 as isize) } != 0 {
                                __state = 66;
                            } else { __state = 10; }
                        }
                        66 => {
                            n_opt = unsafe { strlen(z_opt as *const i8) } as i64;
                            __state = 67;
                        }
                        67 => {
                            z_val =
                                unsafe { z_opt.offset((n_opt + 1 as i64) as isize) };
                            __state = 68;
                        }
                        68 => {
                            n_val = unsafe { strlen(z_val as *const i8) } as i64;
                            __state = 69;
                        }
                        69 => {
                            if n_opt == 3 as i64 &&
                                    unsafe {
                                            memcmp(c"vfs".as_ptr() as *mut i8 as *const (),
                                                z_opt as *const (), 3 as u64)
                                        } == 0 {
                                __state = 71;
                            } else { __state = 72; }
                        }
                        70 => {
                            z_opt =
                                unsafe { z_val.offset((n_val + 1 as i64) as isize) };
                            __state = 65;
                        }
                        71 => { z_vfs = z_val as *const i8; __state = 70; }
                        72 => { a_mode = core::ptr::null(); __state = 73; }
                        73 => { z_mode_type = core::ptr::null_mut(); __state = 74; }
                        74 => { mask = 0; __state = 75; }
                        75 => { limit = 0; __state = 76; }
                        76 => {
                            if n_opt == 5 as i64 &&
                                    unsafe {
                                            memcmp(c"cache".as_ptr() as *mut i8 as *const (),
                                                z_opt as *const (), 5 as u64)
                                        } == 0 {
                                __state = 78;
                            } else { __state = 77; }
                        }
                        77 => {
                            if n_opt == 4 as i64 &&
                                    unsafe {
                                            memcmp(c"mode".as_ptr() as *mut i8 as *const (),
                                                z_opt as *const (), 4 as u64)
                                        } == 0 {
                                __state = 84;
                            } else { __state = 83; }
                        }
                        78 => { __state = 79; }
                        79 => { mask = 131072 | 262144; __state = 80; }
                        80 => {
                            a_mode =
                                &raw mut a_cache_mode[0 as usize] as
                                    *mut OpenModeN8OpenMode;
                            __state = 81;
                        }
                        81 => { limit = mask; __state = 82; }
                        82 => {
                            z_mode_type = c"cache".as_ptr() as *mut i8;
                            __state = 77;
                        }
                        83 => {
                            if !(a_mode).is_null() {
                                __state = 89;
                            } else { __state = 70; }
                        }
                        84 => { __state = 85; }
                        85 => { mask = 1 | 2 | 4 | 128; __state = 86; }
                        86 => {
                            a_mode =
                                &raw mut a_open_mode[0 as usize] as *mut OpenModeN8OpenMode;
                            __state = 87;
                        }
                        87 => {
                            limit = (mask as u32 & flags) as i32;
                            __state = 88;
                        }
                        88 => {
                            z_mode_type = c"access".as_ptr() as *mut i8;
                            __state = 83;
                        }
                        89 => { __state = 90; }
                        90 => { mode = 0; __state = 91; }
                        91 => { i = 0; __state = 93; }
                        92 => {
                            if mode == 0 { __state = 100; } else { __state = 99; }
                        }
                        93 => {
                            if !(unsafe { (*a_mode.offset(i as isize)).z }).is_null() {
                                __state = 94;
                            } else { __state = 92; }
                        }
                        94 => {
                            z = unsafe { (*a_mode.offset(i as isize)).z };
                            __state = 96;
                        }
                        95 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 93;
                        }
                        96 => {
                            if n_val == unsafe { strlen(z) } as i64 &&
                                    0 ==
                                        unsafe {
                                            memcmp(z_val as *const (), z as *const (), n_val as u64)
                                        } {
                                __state = 97;
                            } else { __state = 95; }
                        }
                        97 => {
                            mode = unsafe { (*a_mode.offset(i as isize)).mode };
                            __state = 98;
                        }
                        98 => { __state = 92; }
                        99 => {
                            if mode & !128 > limit {
                                __state = 104;
                            } else { __state = 103; }
                        }
                        100 => {
                            *pz_err_msg_1 =
                                unsafe {
                                    sqlite3_mprintf(c"no such %s mode: %s".as_ptr() as *mut i8
                                            as *const i8, z_mode_type, z_val)
                                };
                            __state = 101;
                        }
                        101 => { rc = 1; __state = 102; }
                        102 => { __state = 2; }
                        103 => {
                            flags = flags & !mask as u32 | mode as u32;
                            __state = 70;
                        }
                        104 => {
                            *pz_err_msg_1 =
                                unsafe {
                                    sqlite3_mprintf(c"%s mode not allowed: %s".as_ptr() as
                                                *mut i8 as *const i8, z_mode_type, z_val)
                                };
                            __state = 105;
                        }
                        105 => { rc = 3; __state = 106; }
                        106 => { __state = 2; }
                        107 => {
                            if (z_file).is_null() as i32 != 0 {
                                __state = 109;
                            } else { __state = 108; }
                        }
                        108 => {
                            unsafe { memset(z_file as *mut (), 0, 4 as u64) };
                            __state = 110;
                        }
                        109 => { return 7; }
                        110 => {
                            {
                                let __n = 4;
                                let __p = &mut z_file;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            __state = 111;
                        }
                        111 => {
                            if n_uri != 0 { __state = 113; } else { __state = 112; }
                        }
                        112 => {
                            unsafe {
                                memset(unsafe { z_file.offset(n_uri as isize) } as *mut (),
                                    0, 4 as u64)
                            };
                            __state = 114;
                        }
                        113 => {
                            unsafe {
                                memcpy(z_file as *mut (), z_uri_1 as *const (),
                                    n_uri as u64)
                            };
                            __state = 112;
                        }
                        114 => { flags &= !64 as u32; __state = 10; }
                        115 => {
                            if *pp_vfs_1 == core::ptr::null_mut() {
                                __state = 117;
                            } else { __state = 116; }
                        }
                        116 => { __state = 2; }
                        117 => {
                            *pz_err_msg_1 =
                                unsafe {
                                    sqlite3_mprintf(c"no such vfs: %s".as_ptr() as *mut i8 as
                                            *const i8, z_vfs)
                                };
                            __state = 118;
                        }
                        118 => { rc = 1; __state = 116; }
                        119 => { *p_flags_1 = flags; __state = 122; }
                        120 => {
                            sqlite3_free_filename(z_file as Sqlite3Filename);
                            __state = 121;
                        }
                        121 => { z_file = core::ptr::null_mut(); __state = 119; }
                        122 => { *pz_file_1 = z_file; __state = 123; }
                        123 => { return rc; }
                        _ => {}
                    }
                }
            }
            unreachable!();
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_errcode(db: *mut Sqlite3) -> i32 {
    let mut i_ret: i32 = 0;
    if (db).is_null() as i32 != 0 { return 7; }
    if (unsafe { sqlite3_safety_check_sick_or_ok(db) } == 0) as i32 != 0 {
        return sqlite3_misuse_error(2838);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if unsafe { (*db).malloc_failed } != 0 {
        i_ret = 7;
    } else { i_ret = unsafe { (*db).err_code } & unsafe { (*db).err_mask }; }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return i_ret;
}
extern "C" fn sqlite3_test_ext_init(db: *mut Sqlite3) -> i32 {
    { let _ = db; };
    return unsafe { sqlite3_fault_sim(500) };
}
static sqlite3_builtin_extensions:
    [unsafe extern "C" fn(*mut Sqlite3) -> i32; 1] =
    [sqlite3_test_ext_init];
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_hook(db: &mut Sqlite3,
    x_callback_1:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32)
            -> i32>, p_arg_1: *mut ()) -> *mut () {
    let mut p_ret: *mut () = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    p_ret = (*db).p_wal_arg;
    (*db).x_wal_callback = x_callback_1;
    (*db).p_wal_arg = p_arg_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return p_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_checkpoint(db: &Sqlite3, i_db_1: i32, e_mode_1: i32,
    mut pn_log_1: *mut i32, mut pn_ckpt_1: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut b_busy: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        i = 0;
        '__b25: loop {
            if !(i < (*db).n_db && rc == 0) { break '__b25; }
            '__c25: loop {
                if i == i_db_1 || i_db_1 == 10 + 2 {
                    rc =
                        unsafe {
                            sqlite3_btree_checkpoint(unsafe {
                                    (*(*db).a_db.offset(i as isize)).p_bt
                                }, e_mode_1, pn_log_1, pn_ckpt_1)
                        };
                    pn_log_1 = core::ptr::null_mut();
                    pn_ckpt_1 = core::ptr::null_mut();
                    if rc == 5 { b_busy = 1; rc = 0; }
                }
                break '__c25;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return if rc == 0 && b_busy != 0 { 5 } else { rc };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_checkpoint_v2(db: *mut Sqlite3,
    z_db_1: *const i8, e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i_db: i32 = 0;
        if !(pn_log_1).is_null() { unsafe { *pn_log_1 = -1 }; }
        if !(pn_ckpt_1).is_null() { unsafe { *pn_ckpt_1 = -1 }; }
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if e_mode_1 < -1 || e_mode_1 > 3 {
            return sqlite3_misuse_error(2596);
        }
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if !(z_db_1).is_null() && unsafe { *z_db_1.offset(0 as isize) } != 0 {
            i_db = unsafe { sqlite3_find_db_name(db, z_db_1) };
        } else { i_db = 10 + 2; }
        if i_db < 0 {
            rc = 1;
            unsafe {
                sqlite3_error_with_msg(db, 1,
                    c"unknown database: %s".as_ptr() as *mut i8 as *const i8,
                    z_db_1)
            };
        } else {
            unsafe { (*db).busy_handler.n_busy = 0 };
            rc =
                unsafe {
                    sqlite3_checkpoint(unsafe { &*db }, i_db, e_mode_1,
                        pn_log_1, pn_ckpt_1)
                };
            unsafe { sqlite3_error(db, rc) };
        }
        rc = unsafe { sqlite3_api_exit(db, rc) };
        if unsafe { (*db).n_vdbe_active } == 0 {
            unsafe {
                std::sync::atomic::AtomicI32::from_ptr(unsafe {
                                &raw mut (*db).u1.is_interrupted
                            } as
                            *mut i32).store(0 as i32,
                    std::sync::atomic::Ordering::Relaxed)
            };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_checkpoint(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32 {
    return sqlite3_wal_checkpoint_v2(db, z_db_1, 0, core::ptr::null_mut(),
            core::ptr::null_mut());
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_default_hook(p_client_data_1: *mut (),
    db: *mut Sqlite3, z_db_1: *const i8, n_frame_1: i32) -> i32 {
    if n_frame_1 >= p_client_data_1 as i64 as i32 {
        unsafe { sqlite3_begin_benign_malloc() };
        unsafe { sqlite3_wal_checkpoint(db, z_db_1) };
        unsafe { sqlite3_end_benign_malloc() };
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_autocheckpoint(db: *mut Sqlite3, n_frame_1: i32)
    -> i32 {
    if n_frame_1 > 0 {
        unsafe {
            sqlite3_wal_hook(unsafe { &mut *db },
                Some(sqlite3_wal_default_hook), n_frame_1 as i64 as *mut ())
        };
    } else {
        unsafe {
            sqlite3_wal_hook(unsafe { &mut *db }, None, core::ptr::null_mut())
        };
    }
    return 0;
}
extern "C" fn open_database(mut z_filename_1: *const i8,
    pp_db_1: &mut *mut Sqlite3, mut flags: u32, z_vfs_1: *const i8) -> i32 {
    unsafe {
        unsafe {
            let mut db: *mut Sqlite3 = core::ptr::null_mut();
            let mut rc: i32 = 0;
            let mut is_threadsafe: i32 = 0;
            let mut z_open: *mut i8 = core::ptr::null_mut();
            let mut z_err_msg: *mut i8 = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s27:
                    {
                    match __state {
                        0 => { __state = 3; }
                        2 => {
                            if !(db).is_null() {
                                __state = 117;
                            } else { __state = 116; }
                        }
                        3 => { __state = 4; }
                        4 => { __state = 5; }
                        5 => { z_open = core::ptr::null_mut(); __state = 6; }
                        6 => { z_err_msg = core::ptr::null_mut(); __state = 7; }
                        7 => { __state = 8; }
                        8 => { *pp_db_1 = core::ptr::null_mut(); __state = 9; }
                        9 => { rc = sqlite3_initialize(); __state = 10; }
                        10 => {
                            if rc != 0 { __state = 12; } else { __state = 11; }
                        }
                        11 => {
                            if sqlite3Config.b_core_mutex as i32 == 0 {
                                __state = 14;
                            } else { __state = 15; }
                        }
                        12 => { return rc; }
                        13 => {
                            if flags & 262144 as u32 != 0 {
                                __state = 21;
                            } else { __state = 22; }
                        }
                        14 => { is_threadsafe = 0; __state = 13; }
                        15 => {
                            if flags & 32768 as u32 != 0 {
                                __state = 16;
                            } else { __state = 17; }
                        }
                        16 => { is_threadsafe = 0; __state = 13; }
                        17 => {
                            if flags & 65536 as u32 != 0 {
                                __state = 18;
                            } else { __state = 19; }
                        }
                        18 => { is_threadsafe = 1; __state = 13; }
                        19 => {
                            is_threadsafe = sqlite3Config.b_full_mutex as i32;
                            __state = 13;
                        }
                        20 => {
                            flags &=
                                !(8 | 16 | 256 | 512 | 1024 | 2048 | 4096 | 8192 | 16384 |
                                                    32768 | 65536 | 524288) as u32;
                            __state = 24;
                        }
                        21 => { flags &= !131072 as u32; __state = 20; }
                        22 => {
                            if sqlite3Config.shared_cache_enabled != 0 {
                                __state = 23;
                            } else { __state = 20; }
                        }
                        23 => { flags |= 131072 as u32; __state = 20; }
                        24 => {
                            db =
                                unsafe {
                                        sqlite3_malloc_zero(core::mem::size_of::<Sqlite3>() as u64)
                                    } as *mut Sqlite3;
                            __state = 25;
                        }
                        25 => {
                            if db == core::ptr::null_mut() {
                                __state = 27;
                            } else { __state = 26; }
                        }
                        26 => {
                            if is_threadsafe != 0 {
                                __state = 29;
                            } else { __state = 28; }
                        }
                        27 => { __state = 2; }
                        28 => {
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            __state = 36;
                        }
                        29 => {
                            unsafe { (*db).mutex = unsafe { sqlite3MutexAlloc(1) } };
                            __state = 30;
                        }
                        30 => {
                            if unsafe { (*db).mutex } == core::ptr::null_mut() {
                                __state = 32;
                            } else { __state = 31; }
                        }
                        31 => {
                            if is_threadsafe == 0 {
                                __state = 35;
                            } else { __state = 28; }
                        }
                        32 => {
                            unsafe { sqlite3_free(db as *mut ()) };
                            __state = 33;
                        }
                        33 => { db = core::ptr::null_mut(); __state = 34; }
                        34 => { __state = 2; }
                        35 => { __state = 28; }
                        36 => {
                            unsafe {
                                (*db).err_mask =
                                    if flags & 33554432 as u32 != 0 as u32 {
                                            4294967295u32
                                        } else { 255 as u32 } as i32
                            };
                            __state = 37;
                        }
                        37 => { unsafe { (*db).n_db = 2 }; __state = 38; }
                        38 => {
                            unsafe { (*db).e_open_state = 109 as u8 };
                            __state = 39;
                        }
                        39 => {
                            unsafe {
                                (*db).a_db =
                                    unsafe { &raw mut (*db).a_db_static[0 as usize] } as *mut Db
                            };
                            __state = 40;
                        }
                        40 => {
                            unsafe { (*db).lookaside.b_disable = 1 as u32 };
                            __state = 41;
                        }
                        41 => {
                            unsafe { (*db).lookaside.sz = 0 as u16 };
                            __state = 42;
                        }
                        42 => {
                            unsafe { (*db).n_fp_digit = 17 as u8 };
                            __state = 43;
                        }
                        43 => { { let _ = 0; }; __state = 44; }
                        44 => {
                            unsafe {
                                memcpy(unsafe { &raw mut (*db).a_limit[0 as usize] } as
                                            *mut i32 as *mut (),
                                    &raw const a_hard_limit[0 as usize] as *const i32 as
                                        *const (), core::mem::size_of::<[i32; 13]>() as u64)
                            };
                            __state = 45;
                        }
                        45 => {
                            unsafe { (*db).a_limit[11 as usize] = 0 };
                            __state = 46;
                        }
                        46 => {
                            unsafe { (*db).auto_commit = 1 as u8 };
                            __state = 47;
                        }
                        47 => {
                            unsafe { (*db).next_autovac = -1 as i8 };
                            __state = 48;
                        }
                        48 => {
                            unsafe { (*db).sz_mmap = sqlite3Config.sz_mmap };
                            __state = 49;
                        }
                        49 => { unsafe { (*db).next_pagesize = 0 }; __state = 50; }
                        50 => {
                            unsafe {
                                (*db).init.az_init =
                                    sqlite3_std_type.as_ptr() as *mut *const i8
                            };
                            __state = 51;
                        }
                        51 => {
                            unsafe {
                                (*db).flags |=
                                    ((64 | 262144) as u32 | 2147483648u32 | 32 as u32) as u64 |
                                                                (16 as u64) << 32 | (32 as u64) << 32 | (64 as u64) << 32 |
                                                    128 as u64 | 1073741824 as u64 | 536870912 as u64 |
                                        32768 as u64
                            };
                            __state = 52;
                        }
                        52 => {
                            unsafe {
                                sqlite3_hash_init(unsafe { &mut (*db).a_coll_seq })
                            };
                            __state = 53;
                        }
                        53 => {
                            unsafe {
                                sqlite3_hash_init(unsafe { &mut (*db).a_module })
                            };
                            __state = 54;
                        }
                        54 => {
                            create_collation(db,
                                sqlite3_str_binary.as_ptr() as *const i8, 1 as u8,
                                core::ptr::null_mut(), Some(bin_coll_func), None);
                            __state = 55;
                        }
                        55 => {
                            create_collation(db,
                                sqlite3_str_binary.as_ptr() as *const i8, 3 as u8,
                                core::ptr::null_mut(), Some(bin_coll_func), None);
                            __state = 56;
                        }
                        56 => {
                            create_collation(db,
                                sqlite3_str_binary.as_ptr() as *const i8, 2 as u8,
                                core::ptr::null_mut(), Some(bin_coll_func), None);
                            __state = 57;
                        }
                        57 => {
                            create_collation(db,
                                c"NOCASE".as_ptr() as *mut i8 as *const i8, 1 as u8,
                                core::ptr::null_mut(), Some(nocase_collating_func), None);
                            __state = 58;
                        }
                        58 => {
                            create_collation(db,
                                c"RTRIM".as_ptr() as *mut i8 as *const i8, 1 as u8,
                                core::ptr::null_mut(), Some(rtrim_coll_func), None);
                            __state = 59;
                        }
                        59 => {
                            if unsafe { (*db).malloc_failed } != 0 {
                                __state = 61;
                            } else { __state = 60; }
                        }
                        60 => { unsafe { (*db).open_flags = flags }; __state = 62; }
                        61 => { __state = 2; }
                        62 => { { let _ = 0; }; __state = 63; }
                        63 => { { let _ = 0; }; __state = 64; }
                        64 => { { let _ = 0; }; __state = 65; }
                        65 => { __state = 66; }
                        66 => { __state = 67; }
                        67 => { __state = 68; }
                        68 => {
                            if 1 << (flags & 7 as u32) & 70 == 0 {
                                __state = 70;
                            } else { __state = 71; }
                        }
                        69 => {
                            if rc != 0 { __state = 75; } else { __state = 74; }
                        }
                        70 => { rc = sqlite3_misuse_error(3594); __state = 69; }
                        71 => {
                            if z_filename_1 == core::ptr::null() {
                                __state = 73;
                            } else { __state = 72; }
                        }
                        72 => {
                            rc =
                                sqlite3_parse_uri(z_vfs_1, z_filename_1, &mut flags,
                                    unsafe { &mut (*db).p_vfs }, &mut z_open, &mut z_err_msg);
                            __state = 69;
                        }
                        73 => {
                            z_filename_1 = c":memory:".as_ptr() as *mut i8 as *const i8;
                            __state = 72;
                        }
                        74 => { { let _ = 0; }; __state = 80; }
                        75 => {
                            if rc == 7 { __state = 77; } else { __state = 76; }
                        }
                        76 => {
                            unsafe {
                                sqlite3_error_with_msg(db, rc,
                                    if !(z_err_msg).is_null() {
                                            c"%s".as_ptr() as *mut i8
                                        } else { core::ptr::null_mut() } as *const i8, z_err_msg)
                            };
                            __state = 78;
                        }
                        77 => { unsafe { sqlite3_oom_fault(db) }; __state = 76; }
                        78 => {
                            unsafe { sqlite3_free(z_err_msg as *mut ()) };
                            __state = 79;
                        }
                        79 => { __state = 2; }
                        80 => {
                            rc =
                                unsafe {
                                    sqlite3_btree_open(unsafe { (*db).p_vfs },
                                        z_open as *const i8, db,
                                        unsafe {
                                            &mut (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt
                                        }, 0, (flags | 256 as u32) as i32)
                                };
                            __state = 81;
                        }
                        81 => {
                            if rc != 0 { __state = 83; } else { __state = 82; }
                        }
                        82 => {
                            unsafe {
                                sqlite3_btree_enter(unsafe {
                                        (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt
                                    })
                            };
                            __state = 87;
                        }
                        83 => {
                            if rc == 10 | 12 << 8 {
                                __state = 85;
                            } else { __state = 84; }
                        }
                        84 => { unsafe { sqlite3_error(db, rc) }; __state = 86; }
                        85 => { rc = 7; __state = 84; }
                        86 => { __state = 2; }
                        87 => {
                            unsafe {
                                (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema =
                                    unsafe {
                                        sqlite3_schema_get(db,
                                            unsafe { (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt })
                                    }
                            };
                            __state = 88;
                        }
                        88 => {
                            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                                __state = 90;
                            } else { __state = 89; }
                        }
                        89 => {
                            unsafe {
                                sqlite3_btree_leave(unsafe {
                                        (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt
                                    })
                            };
                            __state = 91;
                        }
                        90 => {
                            unsafe {
                                sqlite3_set_text_encoding(db,
                                    unsafe {
                                        (*unsafe {
                                                        (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                                                    }).enc
                                    })
                            };
                            __state = 89;
                        }
                        91 => {
                            unsafe {
                                (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema =
                                    unsafe { sqlite3_schema_get(db, core::ptr::null_mut()) }
                            };
                            __state = 92;
                        }
                        92 => {
                            unsafe {
                                (*unsafe { (*db).a_db.offset(0 as isize) }).z_db_s_name =
                                    c"main".as_ptr() as *mut i8
                            };
                            __state = 93;
                        }
                        93 => {
                            unsafe {
                                (*unsafe { (*db).a_db.offset(0 as isize) }).safety_level =
                                    (2 + 1) as u8
                            };
                            __state = 94;
                        }
                        94 => {
                            unsafe {
                                (*unsafe { (*db).a_db.offset(1 as isize) }).z_db_s_name =
                                    c"temp".as_ptr() as *mut i8
                            };
                            __state = 95;
                        }
                        95 => {
                            unsafe {
                                (*unsafe { (*db).a_db.offset(1 as isize) }).safety_level =
                                    1 as u8
                            };
                            __state = 96;
                        }
                        96 => {
                            unsafe { (*db).e_open_state = 118 as u8 };
                            __state = 97;
                        }
                        97 => {
                            if unsafe { (*db).malloc_failed } != 0 {
                                __state = 99;
                            } else { __state = 98; }
                        }
                        98 => { unsafe { sqlite3_error(db, 0) }; __state = 100; }
                        99 => { __state = 2; }
                        100 => {
                            unsafe {
                                sqlite3_register_per_connection_builtin_functions(db)
                            };
                            __state = 101;
                        }
                        101 => { rc = sqlite3_errcode(db); __state = 102; }
                        102 => { i = 0; __state = 104; }
                        103 => {
                            if rc == 0 { __state = 108; } else { __state = 107; }
                        }
                        104 => {
                            if rc == 0 &&
                                    i <
                                        (core::mem::size_of::<[unsafe extern "C" fn(*mut Sqlite3)
                                                                -> i32; 1]>() as u64 /
                                                core::mem::size_of::<unsafe extern "C" fn(*mut Sqlite3)
                                                                -> i32>() as u64) as i32 {
                                __state = 105;
                            } else { __state = 103; }
                        }
                        105 => {
                            rc = unsafe { sqlite3_builtin_extensions[i as usize](db) };
                            __state = 106;
                        }
                        106 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 104;
                        }
                        107 => {
                            if rc != 0 { __state = 113; } else { __state = 112; }
                        }
                        108 => {
                            unsafe { sqlite3_auto_load_extensions(db) };
                            __state = 109;
                        }
                        109 => { rc = sqlite3_errcode(db); __state = 110; }
                        110 => {
                            if rc != 0 { __state = 111; } else { __state = 107; }
                        }
                        111 => { __state = 2; }
                        112 => {
                            setup_lookaside(db, core::ptr::null_mut(),
                                sqlite3Config.sz_lookaside, sqlite3Config.n_lookaside);
                            __state = 114;
                        }
                        113 => { unsafe { sqlite3_error(db, rc) }; __state = 112; }
                        114 => {
                            sqlite3_wal_autocheckpoint(db, 1000);
                            __state = 115;
                        }
                        115 => { __state = 2; }
                        116 => { rc = sqlite3_errcode(db); __state = 119; }
                        117 => { { let _ = 0; }; __state = 118; }
                        118 => {
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            __state = 116;
                        }
                        119 => { { let _ = 0; }; __state = 120; }
                        120 => {
                            if rc & 255 == 7 { __state = 122; } else { __state = 123; }
                        }
                        121 => { *pp_db_1 = db; __state = 126; }
                        122 => { sqlite3_close(db); __state = 124; }
                        123 => {
                            if rc != 0 { __state = 125; } else { __state = 121; }
                        }
                        124 => { db = core::ptr::null_mut(); __state = 121; }
                        125 => {
                            unsafe { (*db).e_open_state = 186 as u8 };
                            __state = 121;
                        }
                        126 => {
                            sqlite3_free_filename(z_open as Sqlite3Filename);
                            __state = 127;
                        }
                        127 => { return rc; }
                        _ => {}
                    }
                }
            }
            unreachable!();
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open(z_filename_1: *const i8,
    pp_db_1: *mut *mut Sqlite3) -> i32 {
    return open_database(z_filename_1, unsafe { &mut *pp_db_1 },
            (2 | 4) as u32, core::ptr::null());
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open16(mut z_filename_1: *const (),
    pp_db_1: *mut *mut Sqlite3) -> i32 {
    unsafe {
        let mut z_filename8: *const i8 = core::ptr::null();
        let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
        let mut rc: i32 = 0;
        unsafe { *pp_db_1 = core::ptr::null_mut() };
        rc = sqlite3_initialize();
        if rc != 0 { return rc; }
        if z_filename_1 == core::ptr::null() {
            z_filename_1 = b"\x00\x00\x00".as_ptr() as *mut i8 as *const ();
        }
        p_val = unsafe { sqlite3_value_new(core::ptr::null_mut()) };
        unsafe {
            sqlite3_value_set_str(p_val, -1, z_filename_1, 2 as u8, None)
        };
        z_filename8 =
            unsafe { sqlite3ValueText(p_val, 1 as u8) } as *const i8;
        if !(z_filename8).is_null() {
            rc =
                open_database(z_filename8, unsafe { &mut *pp_db_1 },
                    (2 | 4) as u32, core::ptr::null());
            { let _ = 0; };
            if rc == 0 &&
                    !(unsafe {
                                                (*unsafe {
                                                                (*unsafe {
                                                                            (*unsafe { *pp_db_1 }).a_db.offset(0 as isize)
                                                                        }).p_schema
                                                            }).schema_flags
                                            } as i32 & 1 == 1) as i32 != 0 {
                unsafe {
                    (*unsafe {
                                        (*unsafe {
                                                    (*unsafe { *pp_db_1 }).a_db.offset(0 as isize)
                                                }).p_schema
                                    }).enc =
                        {
                            unsafe { (*unsafe { *pp_db_1 }).enc = 2 as u8 };
                            unsafe { (*unsafe { *pp_db_1 }).enc }
                        }
                };
            }
        } else { rc = 7; }
        unsafe { sqlite3ValueFree(p_val) };
        return rc & 255;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open_v2(filename: *const i8,
    pp_db_1: *mut *mut Sqlite3, flags: i32, z_vfs_1: *const i8) -> i32 {
    return open_database(filename, unsafe { &mut *pp_db_1 }, flags as u32,
            z_vfs_1);
}
extern "C" fn uri_parameter(mut z_filename_1: *const i8, z_param_1: *const i8)
    -> *const i8 {
    {
        let __n = unsafe { sqlite3_strlen30(z_filename_1) } + 1;
        let __p = &mut z_filename_1;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    while z_filename_1 != core::ptr::null() &&
            unsafe { *z_filename_1.offset(0 as isize) } != 0 {
        let x: i32 = unsafe { strcmp(z_filename_1, z_param_1) };
        {
            let __n = unsafe { sqlite3_strlen30(z_filename_1) } + 1;
            let __p = &mut z_filename_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if x == 0 { return z_filename_1; }
        {
            let __n = unsafe { sqlite3_strlen30(z_filename_1) } + 1;
            let __p = &mut z_filename_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return core::ptr::null();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_uri_parameter(mut z_filename: Sqlite3Filename,
    z_param: *const i8) -> *const i8 {
    if z_filename == core::ptr::null() || z_param == core::ptr::null() {
        return core::ptr::null();
    }
    z_filename = database_name(z_filename);
    return uri_parameter(z_filename, z_param);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_uri_boolean(z_filename: Sqlite3Filename,
    z_param: *const i8, mut b_dflt: i32) -> i32 {
    let z: *const i8 = sqlite3_uri_parameter(z_filename, z_param);
    b_dflt = (b_dflt != 0) as i32;
    return if !(z).is_null() {
            (unsafe { sqlite3_get_boolean(z, b_dflt as u8) }) as i32
        } else { b_dflt };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_uri_int64(z_filename: Sqlite3Filename,
    z_param: *const i8, mut b_dflt: Sqlite3Int64) -> Sqlite3Int64 {
    let z: *const i8 = sqlite3_uri_parameter(z_filename, z_param);
    let mut v: Sqlite3Int64 = 0 as Sqlite3Int64;
    if !(z).is_null() && unsafe { sqlite3_dec_or_hex_to_i64(z, &mut v) } == 0
        {
        b_dflt = v;
    }
    return b_dflt;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_uri_key(mut z_filename: Sqlite3Filename, mut n: i32)
    -> *const i8 {
    if z_filename == core::ptr::null() || n < 0 { return core::ptr::null(); }
    z_filename = database_name(z_filename);
    {
        let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
        let __p = &mut z_filename;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    while !(z_filename).is_null() &&
                unsafe { *z_filename.offset(0 as isize) } != 0 &&
            { let __p = &mut n; let __t = *__p; *__p -= 1; __t } > 0 {
        {
            let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
            let __p = &mut z_filename;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        {
            let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
            let __p = &mut z_filename;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return if unsafe { *z_filename.offset(0 as isize) } != 0 {
            z_filename
        } else { core::ptr::null() };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_filename_database(z_filename: Sqlite3Filename)
    -> *const i8 {
    if z_filename == core::ptr::null() { return core::ptr::null(); }
    return database_name(z_filename);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_filename_journal(mut z_filename: Sqlite3Filename)
    -> *const i8 {
    if z_filename == core::ptr::null() { return core::ptr::null(); }
    z_filename = database_name(z_filename);
    {
        let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
        let __p = &mut z_filename;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    while !(z_filename).is_null() &&
            unsafe { *z_filename.offset(0 as isize) } != 0 {
        {
            let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
            let __p = &mut z_filename;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        {
            let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
            let __p = &mut z_filename;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return unsafe { z_filename.offset(1 as isize) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_filename_wal(mut z_filename: Sqlite3Filename)
    -> *const i8 {
    z_filename = sqlite3_filename_journal(z_filename);
    if !(z_filename).is_null() {
        {
            let __n = unsafe { sqlite3_strlen30(z_filename) } + 1;
            let __p = &mut z_filename;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return z_filename;
}
extern "C" fn append_text(p: *mut i8, z: *const i8) -> *mut i8 {
    let n: u64 = unsafe { strlen(z) };
    unsafe { memcpy(p as *mut (), z as *const (), n + 1 as u64) };
    return unsafe { unsafe { p.add(n as usize).offset(1 as isize) } };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_filename(z_database: *const i8,
    z_journal: *const i8, z_wal: *const i8, n_param: i32,
    az_param: *mut *const i8) -> Sqlite3Filename {
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut i: i32 = 0;
    let mut p_result: *mut i8 = core::ptr::null_mut();
    let mut p: *mut i8 = core::ptr::null_mut();
    n_byte =
        (unsafe { strlen(z_database) } + unsafe { strlen(z_journal) } +
                    unsafe { strlen(z_wal) } + 10 as u64) as Sqlite3Int64;
    {
        i = 0;
        '__b31: loop {
            if !(i < n_param * 2) { break '__b31; }
            '__c31: loop {
                n_byte +=
                    (unsafe { strlen(unsafe { *az_param.offset(i as isize) }) }
                                + 1 as u64) as u64 as Sqlite3Int64;
                break '__c31;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    p_result =
        {
            p =
                unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                    *mut i8;
            p
        };
    if p == core::ptr::null_mut() { return 0 as Sqlite3Filename; }
    unsafe { memset(p as *mut (), 0, 4 as u64) };
    {
        let __n = 4;
        let __p = &mut p;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    p = append_text(p, z_database);
    {
        i = 0;
        '__b32: loop {
            if !(i < n_param * 2) { break '__b32; }
            '__c32: loop {
                p = append_text(p, unsafe { *az_param.offset(i as isize) });
                break '__c32;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        *{
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = 0 as i8
    };
    p = append_text(p, z_journal);
    p = append_text(p, z_wal);
    unsafe {
        *{
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = 0 as i8
    };
    unsafe {
        *{
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = 0 as i8
    };
    { let _ = 0; };
    return unsafe { p_result.offset(4 as isize) } as Sqlite3Filename;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_extended_errcode(db: *mut Sqlite3) -> i32 {
    let mut i_ret: i32 = 0;
    if (db).is_null() as i32 != 0 { return 7; }
    if (unsafe { sqlite3_safety_check_sick_or_ok(db) } == 0) as i32 != 0 {
        return sqlite3_misuse_error(2853);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if unsafe { (*db).malloc_failed } != 0 {
        i_ret = 7;
    } else { i_ret = unsafe { (*db).err_code }; }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return i_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_err_str(mut rc: i32) -> *const i8 {
    unsafe {
        let mut z_err: *const i8 =
            c"unknown error".as_ptr() as *mut i8 as *const i8;
        '__s33:
            {
            match rc {
                516 => {
                    {
                        z_err =
                            c"abort due to ROLLBACK".as_ptr() as *mut i8 as *const i8;
                        break '__s33;
                    }
                    {
                        z_err =
                            c"another row available".as_ptr() as *mut i8 as *const i8;
                        break '__s33;
                    }
                    {
                        z_err =
                            c"no more rows available".as_ptr() as *mut i8 as *const i8;
                        break '__s33;
                    }
                    {
                        rc &= 255;
                        if rc >= 0 &&
                                    rc <
                                        (core::mem::size_of::<[*const i8; 29]>() as u64 /
                                                core::mem::size_of::<*const i8>() as u64) as i32 &&
                                a_msg[rc as usize] != core::ptr::null() {
                            z_err = a_msg[rc as usize];
                        }
                        break '__s33;
                    }
                }
                100 => {
                    {
                        z_err =
                            c"another row available".as_ptr() as *mut i8 as *const i8;
                        break '__s33;
                    }
                    {
                        z_err =
                            c"no more rows available".as_ptr() as *mut i8 as *const i8;
                        break '__s33;
                    }
                    {
                        rc &= 255;
                        if rc >= 0 &&
                                    rc <
                                        (core::mem::size_of::<[*const i8; 29]>() as u64 /
                                                core::mem::size_of::<*const i8>() as u64) as i32 &&
                                a_msg[rc as usize] != core::ptr::null() {
                            z_err = a_msg[rc as usize];
                        }
                        break '__s33;
                    }
                }
                101 => {
                    {
                        z_err =
                            c"no more rows available".as_ptr() as *mut i8 as *const i8;
                        break '__s33;
                    }
                    {
                        rc &= 255;
                        if rc >= 0 &&
                                    rc <
                                        (core::mem::size_of::<[*const i8; 29]>() as u64 /
                                                core::mem::size_of::<*const i8>() as u64) as i32 &&
                                a_msg[rc as usize] != core::ptr::null() {
                            z_err = a_msg[rc as usize];
                        }
                        break '__s33;
                    }
                }
                _ => {
                    {
                        rc &= 255;
                        if rc >= 0 &&
                                    rc <
                                        (core::mem::size_of::<[*const i8; 29]>() as u64 /
                                                core::mem::size_of::<*const i8>() as u64) as i32 &&
                                a_msg[rc as usize] != core::ptr::null() {
                            z_err = a_msg[rc as usize];
                        }
                        break '__s33;
                    }
                }
            }
        }
        return z_err;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_errmsg(db: *mut Sqlite3) -> *const i8 {
    let mut z: *const i8 = core::ptr::null();
    if (db).is_null() as i32 != 0 { return sqlite3_err_str(7); }
    if (unsafe { sqlite3_safety_check_sick_or_ok(db) } == 0) as i32 != 0 {
        return sqlite3_err_str(sqlite3_misuse_error(2732));
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if unsafe { (*db).malloc_failed } != 0 {
        z = sqlite3_err_str(7);
    } else {
        z =
            if unsafe { (*db).err_code } != 0 {
                    (unsafe { sqlite3_value_text(unsafe { (*db).p_err }) }) as
                        *mut i8
                } else { core::ptr::null_mut() } as *const i8;
        { let _ = 0; };
        if z == core::ptr::null() {
            z = sqlite3_err_str(unsafe { (*db).err_code });
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return z;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_errmsg16(db: *mut Sqlite3) -> *const () {
    let mut z: *const () = core::ptr::null();
    if (db).is_null() as i32 != 0 {
        return &raw const out_of_mem[0 as usize] as *mut () as *const ();
    }
    if (unsafe { sqlite3_safety_check_sick_or_ok(db) } == 0) as i32 != 0 {
        return &raw const misuse[0 as usize] as *mut () as *const ();
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if unsafe { (*db).malloc_failed } != 0 {
        z = &raw const out_of_mem[0 as usize] as *mut () as *const ();
    } else {
        z = unsafe { sqlite3_value_text16(unsafe { (*db).p_err }) };
        if z == core::ptr::null() {
            unsafe {
                sqlite3_error_with_msg(db, unsafe { (*db).err_code },
                    sqlite3_err_str(unsafe { (*db).err_code }))
            };
            z = unsafe { sqlite3_value_text16(unsafe { (*db).p_err }) };
        }
        unsafe { sqlite3_oom_clear(db) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return z;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_errstr(rc: i32) -> *const i8 {
    return sqlite3_err_str(rc);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_error_offset(db: *mut Sqlite3) -> i32 {
    let mut i_offset: i32 = -1;
    if !(db).is_null() && unsafe { sqlite3_safety_check_sick_or_ok(db) } != 0
        {
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if unsafe { (*db).err_code } != 0 {
            i_offset = unsafe { (*db).err_byte_offset };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    }
    return i_offset;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_set_errmsg(db: *mut Sqlite3, errcode: i32,
    z_msg_1: *const i8) -> i32 {
    let mut rc: i32 = 0;
    if (unsafe { sqlite3_safety_check_ok(db) } == 0) as i32 != 0 {
        return sqlite3_misuse_error(2759);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if !(z_msg_1).is_null() {
        unsafe {
            sqlite3_error_with_msg(db, errcode,
                c"%s".as_ptr() as *mut i8 as *const i8, z_msg_1)
        };
    } else { unsafe { sqlite3_error(db, errcode) }; }
    rc = unsafe { sqlite3_api_exit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_limit(db: &mut Sqlite3, limit_id_1: i32,
    mut new_limit_1: i32) -> i32 {
    let mut old_limit: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if limit_id_1 < 0 || limit_id_1 >= 12 + 1 { return -1; }
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    old_limit = (*db).a_limit[limit_id_1 as usize];
    if new_limit_1 >= 0 {
        if new_limit_1 > a_hard_limit[limit_id_1 as usize] {
            new_limit_1 = a_hard_limit[limit_id_1 as usize] as i32;
        } else if new_limit_1 < 30 && limit_id_1 == 0 { new_limit_1 = 30; }
        (*db).a_limit[limit_id_1 as usize] = new_limit_1;
    }
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return old_limit;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_func(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, mut enc: i32,
    p_user_data_1: *mut (),
    x_s_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    p_destructor_1: *mut FuncDestructor) -> i32 {
    unsafe {
        let mut p: *mut FuncDef = core::ptr::null_mut();
        let mut extra_flags: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if z_function_name_1 == core::ptr::null() ||
                                x_s_func_1.is_some() && x_final_1.is_some() ||
                            !x_final_1.is_some() as i32 as i32 !=
                                !x_step_1.is_some() as i32 as i32 ||
                        !x_value_1.is_some() as i32 as i32 !=
                            !x_inverse_1.is_some() as i32 as i32 ||
                    (n_arg_1 < -1 || n_arg_1 > 1000) ||
                255 < unsafe { sqlite3_strlen30(z_function_name_1) } {
            return sqlite3_misuse_error(1971);
        }
        { let _ = 0; };
        { let _ = 0; };
        extra_flags =
            enc & (2048 | 524288 | 1048576 | 2097152 | 16777216 | 33554432);
        enc &= 3 | 5;
        { let _ = 0; };
        extra_flags ^= 2097152;
        '__s34:
            {
            match enc {
                4 => { enc = 2; }
                5 => {
                    {
                        let mut rc: i32 = 0;
                        rc =
                            sqlite3_create_func(db, z_function_name_1, n_arg_1,
                                (1 | extra_flags) ^ 2097152, p_user_data_1, x_s_func_1,
                                x_step_1, x_final_1, x_value_1, x_inverse_1,
                                p_destructor_1);
                        if rc == 0 {
                            rc =
                                sqlite3_create_func(db, z_function_name_1, n_arg_1,
                                    (2 | extra_flags) ^ 2097152, p_user_data_1, x_s_func_1,
                                    x_step_1, x_final_1, x_value_1, x_inverse_1,
                                    p_destructor_1);
                        }
                        if rc != 0 { return rc; }
                        enc = 3;
                        break '__s34;
                    }
                    break '__s34;
                    enc = 1;
                }
                1 => { break '__s34; enc = 1; }
                2 => { break '__s34; enc = 1; }
                3 => { break '__s34; enc = 1; }
                _ => { enc = 1; }
            }
        }
        p =
            unsafe {
                sqlite3_find_function(db, z_function_name_1, n_arg_1,
                    enc as u8, 0 as u8)
            };
        if !(p).is_null() &&
                    unsafe { (*p).func_flags } & 3 as u32 == enc as u32 &&
                unsafe { (*p).n_arg } as i32 == n_arg_1 {
            if unsafe { (*db).n_vdbe_active } != 0 {
                unsafe {
                    sqlite3_error_with_msg(db, 5,
                        c"unable to delete/modify user-function due to active statements".as_ptr()
                                as *mut i8 as *const i8)
                };
                { let _ = 0; };
                return 5;
            } else { unsafe { sqlite3_expire_prepared_statements(db, 0) }; }
        } else if !x_s_func_1.is_some() as i32 != 0 &&
                !x_final_1.is_some() as i32 != 0 {
            return 0;
        }
        p =
            unsafe {
                sqlite3_find_function(db, z_function_name_1, n_arg_1,
                    enc as u8, 1 as u8)
            };
        { let _ = 0; };
        if (p).is_null() as i32 != 0 { return 7; }
        function_destroy(db, unsafe { &*p });
        if !(p_destructor_1).is_null() {
            {
                let __p = unsafe { &mut (*p_destructor_1).n_ref };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        unsafe { (*p).u.p_destructor = p_destructor_1 };
        unsafe {
            (*p).func_flags =
                unsafe { (*p).func_flags } & 3 as u32 | extra_flags as u32
        };
        unsafe {
            (*p).x_s_func =
                if x_s_func_1.is_some() { x_s_func_1 } else { x_step_1 }
        };
        unsafe { (*p).x_finalize = x_final_1 };
        unsafe { (*p).x_value = x_value_1 };
        unsafe { (*p).x_inverse = x_inverse_1 };
        unsafe { (*p).p_user_data = p_user_data_1 };
        unsafe { (*p).n_arg = n_arg_1 as u16 as i16 };
        return 0;
    }
}
extern "C" fn create_function_api(db: *mut Sqlite3, z_func_1: *const i8,
    n_arg_1: i32, enc: i32, p: *mut (),
    x_s_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    let mut rc: i32 = 1;
    '__b35: loop {
        '__c35: loop {
            let mut p_arg: *mut FuncDestructor = core::ptr::null_mut();
            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
            if x_destroy_1.is_some() {
                p_arg =
                    unsafe {
                            sqlite3Malloc(core::mem::size_of::<FuncDestructor>() as u64)
                        } as *mut FuncDestructor;
                if (p_arg).is_null() as i32 != 0 {
                    unsafe { sqlite3_oom_fault(db) };
                    unsafe { x_destroy_1.unwrap()(p) };
                    break '__b35;
                }
                unsafe { (*p_arg).n_ref = 0 };
                unsafe { (*p_arg).x_destroy = x_destroy_1 };
                unsafe { (*p_arg).p_user_data = p };
            }
            rc =
                sqlite3_create_func(db, z_func_1, n_arg_1, enc, p, x_s_func_1,
                    x_step_1, x_final_1, x_value_1, x_inverse_1, p_arg);
            if !(p_arg).is_null() && unsafe { (*p_arg).n_ref } == 0 {
                { let _ = 0; };
                unsafe { x_destroy_1.unwrap()(p) };
                unsafe { sqlite3_free(p_arg as *mut ()) };
            }
            break '__c35;
        }
        if !(false) { break '__b35; }
    }
    rc = unsafe { sqlite3_api_exit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_function(db: *mut Sqlite3,
    z_func_1: *const i8, n_arg_1: i32, enc: i32, p: *mut (),
    x_s_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32 {
    return create_function_api(db, z_func_1, n_arg_1, enc, p, x_s_func_1,
            x_step_1, x_final_1, None, None, None);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_function16(db: *mut Sqlite3,
    z_function_name_1: *const (), n_arg_1: i32, e_text_rep_1: i32, p: *mut (),
    x_s_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32 {
    let mut rc: i32 = 0;
    let mut z_func8: *mut i8 = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    { let _ = 0; };
    z_func8 = unsafe { sqlite3_utf16to8(db, z_function_name_1, -1, 2 as u8) };
    rc =
        sqlite3_create_func(db, z_func8 as *const i8, n_arg_1, e_text_rep_1,
            p, x_s_func_1, x_step_1, x_final_1, None, None,
            core::ptr::null_mut());
    unsafe { sqlite3_db_free(db, z_func8 as *mut ()) };
    rc = unsafe { sqlite3_api_exit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_function_v2(db: *mut Sqlite3,
    z_func_1: *const i8, n_arg_1: i32, enc: i32, p: *mut (),
    x_s_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    return create_function_api(db, z_func_1, n_arg_1, enc, p, x_s_func_1,
            x_step_1, x_final_1, None, None, x_destroy_1);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_window_function(db: *mut Sqlite3,
    z_func_1: *const i8, n_arg_1: i32, enc: i32, p: *mut (),
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    return create_function_api(db, z_func_1, n_arg_1, enc, p, None, x_step_1,
            x_final_1, x_value_1, x_inverse_1, x_destroy_1);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_global_recover() -> i32 { return 0; }
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_thread_cleanup() -> () {}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_clientdata(db: &Sqlite3, z_name_1: *const i8)
    -> *mut () {
    let mut p: *const DbClientData = core::ptr::null();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    {
        p = (*db).p_db_data;
        '__b36: loop {
            if !(!(p).is_null()) { break '__b36; }
            '__c36: loop {
                if unsafe {
                            strcmp(unsafe { (*p).z_name.as_ptr() } as *mut i8 as
                                    *const i8, z_name_1)
                        } == 0 {
                    let p_result: *mut () = unsafe { (*p).p_data };
                    unsafe { sqlite3_mutex_leave((*db).mutex) };
                    return p_result;
                }
                break '__c36;
            }
            p = unsafe { (*p).p_next };
        }
    }
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return core::ptr::null_mut();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_set_clientdata(db: &mut Sqlite3,
    z_name_1: *const i8, p_data_1: *mut (),
    x_destructor_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    let mut p: *mut DbClientData = core::ptr::null_mut();
    let mut pp: *mut *mut DbClientData = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    pp = &mut (*db).p_db_data;
    {
        p = (*db).p_db_data;
        '__b37: loop {
            if !(!(p).is_null() &&
                            unsafe {
                                    strcmp(unsafe { (*p).z_name.as_ptr() } as *mut i8 as
                                            *const i8, z_name_1)
                                } != 0) {
                break '__b37;
            }
            '__c37: loop { pp = unsafe { &mut (*p).p_next }; break '__c37; }
            p = unsafe { (*p).p_next };
        }
    }
    if !(p).is_null() {
        { let _ = 0; };
        if unsafe { (*p).x_destructor.is_some() } {
            unsafe {
                (unsafe {
                        (*p).x_destructor.unwrap()
                    })(unsafe { (*p).p_data })
            };
        }
        if p_data_1 == core::ptr::null_mut() {
            unsafe { *pp = unsafe { (*p).p_next } };
            unsafe { sqlite3_free(p as *mut ()) };
            unsafe { sqlite3_mutex_leave((*db).mutex) };
            return 0;
        }
    } else if p_data_1 == core::ptr::null_mut() {
        unsafe { sqlite3_mutex_leave((*db).mutex) };
        return 0;
    } else {
        let n: u64 = unsafe { strlen(z_name_1) };
        p =
            unsafe {
                    sqlite3_malloc64(core::mem::offset_of!(DbClientData, z_name)
                                as u64 + (n + 1 as u64))
                } as *mut DbClientData;
        if p == core::ptr::null_mut() {
            if x_destructor_1.is_some() {
                unsafe { x_destructor_1.unwrap()(p_data_1) };
            }
            unsafe { sqlite3_mutex_leave((*db).mutex) };
            return 7;
        }
        unsafe {
            memcpy(unsafe { (*p).z_name.as_ptr() } as *mut i8 as *mut (),
                z_name_1 as *const (), n + 1 as u64)
        };
        unsafe { (*p).p_next = (*db).p_db_data };
        (*db).p_db_data = p;
    }
    unsafe { (*p).p_data = p_data_1 };
    unsafe { (*p).x_destructor = x_destructor_1 };
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_collation_v2(db: *mut Sqlite3,
    z_name_1: *const i8, enc: i32, p_ctx_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    { let _ = 0; };
    rc =
        create_collation(db, z_name_1, enc as u8, p_ctx_1, x_compare_1,
            x_del_1);
    rc = unsafe { sqlite3_api_exit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_collation(db: *mut Sqlite3,
    z_name_1: *const i8, enc: i32, p_ctx_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>) -> i32 {
    return unsafe {
            sqlite3_create_collation_v2(db, z_name_1, enc, p_ctx_1,
                x_compare_1, None)
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_collation16(db: *mut Sqlite3,
    z_name_1: *const (), enc: i32, p_ctx_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>) -> i32 {
    let mut rc: i32 = 0;
    let mut z_name8: *mut i8 = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    { let _ = 0; };
    z_name8 = unsafe { sqlite3_utf16to8(db, z_name_1, -1, 2 as u8) };
    if !(z_name8).is_null() {
        rc =
            create_collation(db, z_name8 as *const i8, enc as u8, p_ctx_1,
                x_compare_1, None);
        unsafe { sqlite3_db_free(db, z_name8 as *mut ()) };
    }
    rc = unsafe { sqlite3_api_exit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_collation_needed(db: &mut Sqlite3,
    p_coll_needed_arg_1: *mut (),
    x_coll_needed_1:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const i8)
            -> ()>) -> i32 {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    (*db).x_coll_needed = x_coll_needed_1;
    (*db).x_coll_needed16 = None;
    (*db).p_coll_needed_arg = p_coll_needed_arg_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_collation_needed16(db: &mut Sqlite3,
    p_coll_needed_arg_1: *mut (),
    x_coll_needed16_1:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const ())
            -> ()>) -> i32 {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    (*db).x_coll_needed = None;
    (*db).x_coll_needed16 = x_coll_needed16_1;
    (*db).p_coll_needed_arg = p_coll_needed_arg_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sleep(ms: i32) -> i32 {
    let mut p_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
    let mut rc: i32 = 0;
    p_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
    if p_vfs == core::ptr::null_mut() { return 0; }
    rc =
        unsafe { sqlite3_os_sleep(p_vfs, if ms < 0 { 0 } else { 1000 * ms }) }
            / 1000;
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_autocommit(db: &Sqlite3) -> i32 {
    let mut i_ret: i32 = 0;
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    i_ret = (*db).auto_commit as i32;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return i_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_name(db: &Sqlite3, n_1: i32) -> *const i8 {
    let mut z_ret: *const i8 = core::ptr::null();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    if n_1 >= 0 && n_1 < (*db).n_db {
        z_ret =
            unsafe { (*(*db).a_db.offset(n_1 as isize)).z_db_s_name } as
                *const i8;
    }
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return z_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_name_to_btree(db: *mut Sqlite3,
    z_db_name_1: *const i8) -> *mut Btree {
    let i_db: i32 =
        if !(z_db_name_1).is_null() {
            unsafe { sqlite3_find_db_name(db, z_db_name_1) }
        } else { 0 };
    return if i_db < 0 {
            core::ptr::null_mut()
        } else {
            unsafe { (*unsafe { (*db).a_db.offset(i_db as isize) }).p_bt }
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_filename(db: *mut Sqlite3,
    z_db_name_1: *const i8) -> Sqlite3Filename {
    let mut p_bt: *mut Btree = core::ptr::null_mut();
    p_bt = sqlite3_db_name_to_btree(db, z_db_name_1);
    return if !(p_bt).is_null() {
            unsafe { sqlite3_btree_get_filename(p_bt) }
        } else { core::ptr::null() };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_readonly(db: *mut Sqlite3,
    z_db_name_1: *const i8) -> i32 {
    let mut p_bt: *mut Btree = core::ptr::null_mut();
    p_bt = sqlite3_db_name_to_btree(db, z_db_name_1);
    return if !(p_bt).is_null() {
            unsafe { sqlite3_btree_is_readonly(p_bt) }
        } else { -1 };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_txn_state(db: *mut Sqlite3, z_schema_1: *const i8)
    -> i32 {
    let mut i_db: i32 = 0;
    let mut n_db: i32 = 0;
    let mut i_txn: i32 = -1;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if !(z_schema_1).is_null() {
        n_db =
            { i_db = unsafe { sqlite3_find_db_name(db, z_schema_1) }; i_db };
        if i_db < 0 {
            { let __p = &mut n_db; let __t = *__p; *__p -= 1; __t };
        }
    } else { i_db = 0; n_db = unsafe { (*db).n_db } - 1; }
    {
        '__b38: loop {
            if !(i_db <= n_db) { break '__b38; }
            '__c38: loop {
                let p_bt: *mut Btree =
                    unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).p_bt
                    };
                let x: i32 =
                    if p_bt != core::ptr::null_mut() {
                        unsafe { sqlite3_btree_txn_state(p_bt) }
                    } else { 0 };
                if x > i_txn { i_txn = x; }
                break '__c38;
            }
            { let __p = &mut i_db; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return i_txn;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_commit_hook(db: &mut Sqlite3,
    x_callback_1: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_arg_1: *mut ()) -> *mut () {
    let mut p_old: *mut () = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    p_old = (*db).p_commit_arg;
    (*db).x_commit_callback = x_callback_1;
    (*db).p_commit_arg = p_arg_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return p_old;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rollback_hook(db: &mut Sqlite3,
    x_callback_1: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_arg_1: *mut ()) -> *mut () {
    let mut p_ret: *mut () = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    p_ret = (*db).p_rollback_arg;
    (*db).x_rollback_callback = x_callback_1;
    (*db).p_rollback_arg = p_arg_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return p_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_autovacuum_pages(db: &mut Sqlite3,
    x_callback_1:
        Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32)
            -> u32>, p_arg_1: *mut (),
    x_destructor_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    if (*db).x_autovac_destr.is_some() {
        unsafe { (*db).x_autovac_destr.unwrap()((*db).p_autovac_pages_arg) };
    }
    (*db).x_autovac_pages = x_callback_1;
    (*db).p_autovac_pages_arg = p_arg_1;
    (*db).x_autovac_destr = x_destructor_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_update_hook(db: &mut Sqlite3,
    x_callback_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64)
            -> ()>, p_arg_1: *mut ()) -> *mut () {
    let mut p_ret: *mut () = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*db).mutex) };
    p_ret = (*db).p_update_arg;
    (*db).x_update_callback = x_callback_1;
    (*db).p_update_arg = p_arg_1;
    unsafe { sqlite3_mutex_leave((*db).mutex) };
    return p_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_release_memory(db: *mut Sqlite3) -> i32 {
    let mut i: i32 = 0;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe { sqlite3_btree_enter_all(db) };
    {
        i = 0;
        '__b39: loop {
            if !(i < unsafe { (*db).n_db }) { break '__b39; }
            '__c39: loop {
                let p_bt: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                if !(p_bt).is_null() {
                    let p_pager: *mut Pager =
                        unsafe { sqlite3_btree_pager(p_bt) };
                    unsafe { sqlite3_pager_shrink(p_pager) };
                }
                break '__c39;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_btree_leave_all(db) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_table_column_metadata(db: *mut Sqlite3,
    z_db_name_1: *const i8, z_table_name_1: *const i8,
    z_column_name_1: *const i8, pz_data_type_1: *mut *const i8,
    pz_coll_seq_1: *mut *const i8, p_not_null_1: *mut i32,
    p_primary_key_1: *mut i32, p_autoinc_1: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut z_err_msg: *mut i8 = core::ptr::null_mut();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut z_data_type: *const i8 = core::ptr::null();
        let mut z_coll_seq: *const i8 = core::ptr::null();
        let mut notnull: i32 = 0;
        let mut primarykey: i32 = 0;
        let mut autoinc: i32 = 0;
        '__b40: loop {
            '__c40: loop {
                let mut p_col: *mut Column = core::ptr::null_mut();
                let mut i_col: i32 = 0;
                unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                unsafe { sqlite3_btree_enter_all(db) };
                rc = unsafe { sqlite3_init(db, &mut z_err_msg) };
                if 0 != rc { break '__b40; }
                p_tab =
                    unsafe {
                        sqlite3_find_table(db, z_table_name_1, z_db_name_1)
                    };
                if (p_tab).is_null() as i32 != 0 ||
                        unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                    p_tab = core::ptr::null_mut();
                    break '__b40;
                }
                if z_column_name_1 == core::ptr::null()
                    {} else {
                    i_col =
                        unsafe { sqlite3_column_index(p_tab, z_column_name_1) };
                    if i_col >= 0 {
                        p_col =
                            unsafe { unsafe { (*p_tab).a_col.offset(i_col as isize) } };
                    } else {
                        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 &&
                                unsafe { sqlite3_is_rowid(z_column_name_1) } != 0 {
                            i_col = unsafe { (*p_tab).i_p_key } as i32;
                            p_col =
                                if i_col >= 0 {
                                    unsafe {
                                        &mut *unsafe { (*p_tab).a_col.offset(i_col as isize) }
                                    }
                                } else { core::ptr::null_mut() };
                        } else { p_tab = core::ptr::null_mut(); break '__b40; }
                    }
                }
                if !(p_col).is_null() {
                    z_data_type =
                        unsafe { sqlite3ColumnType(p_col, core::ptr::null_mut()) }
                            as *const i8;
                    z_coll_seq = unsafe { sqlite3_column_coll(p_col) };
                    notnull =
                        (unsafe { (*p_col).not_null() } as i32 != 0) as i32;
                    primarykey =
                        (unsafe { (*p_col).col_flags } as i32 & 1 != 0) as i32;
                    autoinc =
                        (unsafe { (*p_tab).i_p_key } as i32 == i_col &&
                                unsafe { (*p_tab).tab_flags } & 8 as u32 != 0 as u32) as
                            i32;
                } else {
                    z_data_type = c"INTEGER".as_ptr() as *mut i8 as *const i8;
                    primarykey = 1;
                }
                if (z_coll_seq).is_null() as i32 != 0 {
                    z_coll_seq = sqlite3_str_binary.as_ptr() as *const i8;
                }
                break '__c40;
            }
            if !(false) { break '__b40; }
        }
        unsafe { sqlite3_btree_leave_all(db) };
        if !(pz_data_type_1).is_null() {
            unsafe { *pz_data_type_1 = z_data_type };
        }
        if !(pz_coll_seq_1).is_null() {
            unsafe { *pz_coll_seq_1 = z_coll_seq };
        }
        if !(p_not_null_1).is_null() { unsafe { *p_not_null_1 = notnull }; }
        if !(p_primary_key_1).is_null() {
            unsafe { *p_primary_key_1 = primarykey };
        }
        if !(p_autoinc_1).is_null() { unsafe { *p_autoinc_1 = autoinc }; }
        if 0 == rc && (p_tab).is_null() as i32 != 0 {
            unsafe { sqlite3_db_free(db, z_err_msg as *mut ()) };
            z_err_msg =
                unsafe {
                    sqlite3_m_printf(db,
                        c"no such table column: %s.%s".as_ptr() as *mut i8 as
                            *const i8, z_table_name_1, z_column_name_1)
                };
            rc = 1;
        }
        unsafe {
            sqlite3_error_with_msg(db, rc,
                if !(z_err_msg).is_null() {
                        c"%s".as_ptr() as *mut i8
                    } else { core::ptr::null_mut() } as *const i8, z_err_msg)
        };
        unsafe { sqlite3_db_free(db, z_err_msg as *mut ()) };
        rc = unsafe { sqlite3_api_exit(db, rc) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return rc;
    }
}
extern "C" fn sqlite3_invalid_function(context: *mut Sqlite3Context,
    not_used_1: i32, not_used2_1: *mut *mut Sqlite3Value) -> () {
    let z_name: *const i8 =
        unsafe { sqlite3_user_data(context) } as *const i8;
    let mut z_err: *mut i8 = core::ptr::null_mut();
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    z_err =
        unsafe {
            sqlite3_mprintf(c"unable to use function %s in the requested context".as_ptr()
                        as *mut i8 as *const i8, z_name)
        };
    unsafe { sqlite3_result_error(context, z_err as *const i8, -1) };
    unsafe { sqlite3_free(z_err as *mut ()) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_overload_function(db: *mut Sqlite3,
    z_name_1: *const i8, n_arg_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut z_copy: *mut i8 = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    rc =
        (unsafe {
                    sqlite3_find_function(db, z_name_1, n_arg_1, 1 as u8,
                        0 as u8)
                } != core::ptr::null_mut()) as i32;
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    if rc != 0 { return 0; }
    z_copy =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_name_1)
        };
    if z_copy == core::ptr::null_mut() { return 7; }
    return sqlite3_create_function_v2(db, z_name_1, n_arg_1, 1,
            z_copy as *mut (), Some(sqlite3_invalid_function), None, None,
            Some(sqlite3_free));
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_mutex(db: &Sqlite3) -> *mut Sqlite3Mutex {
    return (*db).mutex;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_file_control(db: *mut Sqlite3,
    z_db_name_1: *const i8, op: i32, p_arg_1: *mut ()) -> i32 {
    let mut rc: i32 = 1;
    let mut p_btree: *mut Btree = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    p_btree = unsafe { sqlite3_db_name_to_btree(db, z_db_name_1) };
    if !(p_btree).is_null() {
        let mut p_pager: *mut Pager = core::ptr::null_mut();
        let mut fd: *mut Sqlite3File = core::ptr::null_mut();
        unsafe { sqlite3_btree_enter(p_btree) };
        p_pager = unsafe { sqlite3_btree_pager(p_btree) };
        { let _ = 0; };
        fd = unsafe { sqlite3_pager_file(p_pager) };
        { let _ = 0; };
        if op == 7 {
            unsafe { *(p_arg_1 as *mut *mut Sqlite3File) = fd };
            rc = 0;
        } else if op == 27 {
            unsafe {
                *(p_arg_1 as *mut *mut Sqlite3Vfs) =
                    unsafe { sqlite3_pager_vfs(p_pager) }
            };
            rc = 0;
        } else if op == 28 {
            unsafe {
                *(p_arg_1 as *mut *mut Sqlite3File) =
                    unsafe { sqlite3_pager_jrnl_file(p_pager) }
            };
            rc = 0;
        } else if op == 35 {
            unsafe {
                *(p_arg_1 as *mut u32) =
                    unsafe { sqlite3_pager_data_version(p_pager) }
            };
            rc = 0;
        } else if op == 38 {
            let i_new: i32 = unsafe { *(p_arg_1 as *mut i32) };
            unsafe {
                *(p_arg_1 as *mut i32) =
                    unsafe { sqlite3_btree_get_requested_reserve(p_btree) }
            };
            if i_new >= 0 && i_new <= 255 {
                unsafe { sqlite3_btree_set_page_size(p_btree, 0, i_new, 0) };
            }
            rc = 0;
        } else if op == 42 {
            unsafe { sqlite3_btree_clear_cache(p_btree) };
            rc = 0;
        } else {
            let n_save: i32 = unsafe { (*db).busy_handler.n_busy };
            rc = unsafe { sqlite3_os_file_control(fd, op, p_arg_1) };
            unsafe { (*db).busy_handler.n_busy = n_save };
        }
        unsafe { sqlite3_btree_leave(p_btree) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_test_control(op: i32, mut __va0: ...)
    -> i32 {
    unsafe {
        unsafe {
            let mut rc: i32 = 0;
            let mut ap: *const i8 = core::ptr::null();
            unsafe { ap = core::mem::transmute_copy(&__va0) };
            '__s41:
                {
                match op {
                    5 => {
                        { unsafe { sqlite3_prng_save_state() }; break '__s41; }
                        { unsafe { sqlite3_prng_restore_state() }; break '__s41; }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let mut y: i32 = 0;
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            { let _ = 0; };
                            if !(db).is_null() &&
                                    {
                                            y =
                                                unsafe {
                                                    (*unsafe {
                                                                    (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                                                                }).schema_cookie
                                                };
                                            y
                                        } != 0 {
                                x = y;
                            }
                            sqlite3Config.i_prng_seed = x as u32;
                            unsafe { sqlite3_randomness(0, core::ptr::null_mut()) };
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let b: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if b != 0 {
                                unsafe { (*db).flags |= (8 as u64) << 32 };
                            } else { unsafe { (*db).flags &= !((8 as u64) << 32) }; }
                            break '__s41;
                        }
                        {
                            let sz: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let a_prog: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            rc = unsafe { sqlite3_bitvec_builtin_test(sz, a_prog) };
                            break '__s41;
                        }
                        {
                            sqlite3Config.x_test_callback =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(i32)
                                                                        -> i32>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn(i32) -> i32)
                                    });
                            rc = unsafe { sqlite3_fault_sim(0) };
                            break '__s41;
                        }
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    6 => {
                        { unsafe { sqlite3_prng_restore_state() }; break '__s41; }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let mut y: i32 = 0;
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            { let _ = 0; };
                            if !(db).is_null() &&
                                    {
                                            y =
                                                unsafe {
                                                    (*unsafe {
                                                                    (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                                                                }).schema_cookie
                                                };
                                            y
                                        } != 0 {
                                x = y;
                            }
                            sqlite3Config.i_prng_seed = x as u32;
                            unsafe { sqlite3_randomness(0, core::ptr::null_mut()) };
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let b: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if b != 0 {
                                unsafe { (*db).flags |= (8 as u64) << 32 };
                            } else { unsafe { (*db).flags &= !((8 as u64) << 32) }; }
                            break '__s41;
                        }
                        {
                            let sz: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let a_prog: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            rc = unsafe { sqlite3_bitvec_builtin_test(sz, a_prog) };
                            break '__s41;
                        }
                        {
                            sqlite3Config.x_test_callback =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(i32)
                                                                        -> i32>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn(i32) -> i32)
                                    });
                            rc = unsafe { sqlite3_fault_sim(0) };
                            break '__s41;
                        }
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    28 => {
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let mut y: i32 = 0;
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            { let _ = 0; };
                            if !(db).is_null() &&
                                    {
                                            y =
                                                unsafe {
                                                    (*unsafe {
                                                                    (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                                                                }).schema_cookie
                                                };
                                            y
                                        } != 0 {
                                x = y;
                            }
                            sqlite3Config.i_prng_seed = x as u32;
                            unsafe { sqlite3_randomness(0, core::ptr::null_mut()) };
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let b: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if b != 0 {
                                unsafe { (*db).flags |= (8 as u64) << 32 };
                            } else { unsafe { (*db).flags &= !((8 as u64) << 32) }; }
                            break '__s41;
                        }
                        {
                            let sz: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let a_prog: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            rc = unsafe { sqlite3_bitvec_builtin_test(sz, a_prog) };
                            break '__s41;
                        }
                        {
                            sqlite3Config.x_test_callback =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(i32)
                                                                        -> i32>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn(i32) -> i32)
                                    });
                            rc = unsafe { sqlite3_fault_sim(0) };
                            break '__s41;
                        }
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    7 => {
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let b: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if b != 0 {
                                unsafe { (*db).flags |= (8 as u64) << 32 };
                            } else { unsafe { (*db).flags &= !((8 as u64) << 32) }; }
                            break '__s41;
                        }
                        {
                            let sz: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let a_prog: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            rc = unsafe { sqlite3_bitvec_builtin_test(sz, a_prog) };
                            break '__s41;
                        }
                        {
                            sqlite3Config.x_test_callback =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(i32)
                                                                        -> i32>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn(i32) -> i32)
                                    });
                            rc = unsafe { sqlite3_fault_sim(0) };
                            break '__s41;
                        }
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    8 => {
                        {
                            let sz: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let a_prog: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            rc = unsafe { sqlite3_bitvec_builtin_test(sz, a_prog) };
                            break '__s41;
                        }
                        {
                            sqlite3Config.x_test_callback =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(i32)
                                                                        -> i32>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn(i32) -> i32)
                                    });
                            rc = unsafe { sqlite3_fault_sim(0) };
                            break '__s41;
                        }
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    9 => {
                        {
                            sqlite3Config.x_test_callback =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(i32)
                                                                        -> i32>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn(i32) -> i32)
                                    });
                            rc = unsafe { sqlite3_fault_sim(0) };
                            break '__s41;
                        }
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    10 => {
                        {
                            let mut x_benign_begin:
                                    Option<unsafe extern "C" fn() -> ()> = None;
                            let mut x_benign_end: Option<unsafe extern "C" fn() -> ()> =
                                None;
                            x_benign_begin =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            x_benign_end =
                                Some(unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<unsafe extern "C" fn()
                                                                        -> ()>() + 7) & !7);
                                        *(__va_p as *const unsafe extern "C" fn() -> ())
                                    });
                            unsafe {
                                sqlite3_benign_malloc_hooks(x_benign_begin, x_benign_end)
                            };
                            break '__s41;
                        }
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    11 => {
                        {
                            rc = sqlite3_pending_byte;
                            {
                                let new_val: u32 =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    };
                                if new_val != 0 { sqlite3_pending_byte = new_val as i32; }
                            }
                            break '__s41;
                        }
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    12 => {
                        {
                            let mut x: i32 = 0 as i32;
                            { let _ = 0; };
                            rc = x as i32;
                            break '__s41;
                        }
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    13 => {
                        {
                            let mut x: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            rc = if x != 0 { x } else { 0 };
                            break '__s41;
                        }
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    22 => {
                        { rc = 1234 * 100 + 1 * 10 + 0; break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    15 => {
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).db_opt_flags =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                        *(__va_p as *const u32)
                                    }
                            };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    16 => {
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let p_n: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_n = unsafe { (*db).db_opt_flags } as i32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    18 => {
                        {
                            sqlite3Config.b_localtime_fault =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            if sqlite3Config.b_localtime_fault == 2 {
                                sqlite3Config.x_alt_localtime =
                                    Some(unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<unsafe extern "C" fn(*const (),
                                                                            *mut ()) -> i32>() + 7) & !7);
                                            *(__va_p as
                                                    *const unsafe extern "C" fn(*const (), *mut ()) -> i32)
                                        });
                            } else { sqlite3Config.x_alt_localtime = None; }
                            break '__s41;
                        }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    17 => {
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe { (*db).m_db_flags ^= 32 as u32 };
                            break '__s41;
                        }
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    20 => {
                        {
                            sqlite3Config.never_corrupt =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    29 => {
                        {
                            sqlite3Config.b_extra_schema_checks =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    } as u8;
                            break '__s41;
                        }
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    19 => {
                        {
                            sqlite3Config.i_once_reset_threshold =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            break '__s41;
                        }
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    21 => {
                        { break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    24 => {
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            unsafe {
                                (*db).n_max_sorter_mmap =
                                    unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                        *(__va_p as *const i32)
                                    }
                            };
                            break '__s41;
                        }
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    23 => {
                        { if sqlite3Config.is_init == 0 { rc = 1; } break '__s41; }
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    25 => {
                        {
                            let db: *mut Sqlite3 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                !7);
                                    *(__va_p as *const *mut Sqlite3)
                                };
                            let mut i_db: i32 = 0;
                            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                            i_db =
                                unsafe {
                                    sqlite3_find_db_name(db,
                                        unsafe {
                                            let __ap = &mut ap;
                                            let __va_p = *__ap;
                                            *__ap =
                                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                            *(__va_p as *const *const i8)
                                        })
                                };
                            if i_db >= 0 {
                                unsafe { (*db).init.i_db = i_db as u8 };
                                unsafe {
                                    (*db).init.busy =
                                        {
                                                unsafe {
                                                    (*db).init.set_imposter_table(unsafe {
                                                                    let __ap = &mut ap;
                                                                    let __va_p = *__ap;
                                                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                                    *(__va_p as *const i32)
                                                                } as u32 as u32)
                                                };
                                                unsafe { (*db).init.imposter_table() }
                                            } as u8
                                };
                                unsafe {
                                    (*db).init.new_tnum =
                                        unsafe {
                                                let __ap = &mut ap;
                                                let __va_p = *__ap;
                                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                                *(__va_p as *const i32)
                                            } as Pgno
                                };
                                if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*db).init.new_tnum } > 0 as u32 {
                                    unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                                }
                            }
                            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                            break '__s41;
                        }
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    27 => {
                        {
                            let p_ctx: *mut Sqlite3Context =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Context>() +
                                                        7) & !7);
                                    *(__va_p as *const *mut Sqlite3Context)
                                };
                            unsafe { sqlite3_result_int_real(p_ctx) };
                            break '__s41;
                        }
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    30 => {
                        {
                            let db: *const Sqlite3 =
                                unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut Sqlite3>() + 7) &
                                                    !7);
                                        *(__va_p as *const *mut Sqlite3)
                                    } as *const Sqlite3;
                            let pn: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Sqlite3Uint64>() + 7)
                                                & !7);
                                    *(__va_p as *const *mut Sqlite3Uint64)
                                };
                            unsafe { *pn = 0 as u64 };
                            { let _ = db; };
                            break '__s41;
                        }
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    31 => {
                        {
                            let op_trace: i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                };
                            let ptr: *mut u32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u32>() + 7) & !7);
                                    *(__va_p as *const *mut u32)
                                };
                            '__s42:
                                {
                                match op_trace {
                                    0 => { unsafe { *ptr = sqlite3_tree_trace }; }
                                    1 => { sqlite3_tree_trace = unsafe { *ptr }; }
                                    2 => { unsafe { *ptr = sqlite3_where_trace }; }
                                    3 => { sqlite3_where_trace = unsafe { *ptr }; }
                                    _ => {}
                                }
                            }
                            break '__s41;
                        }
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    33 => {
                        {
                            let r_in: f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                    *(__va_p as *const f64)
                                };
                            let r_log_est: LogEst =
                                unsafe { sqlite3_log_est_from_double(r_in) };
                            let p_i1: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            let p_u64: *mut u64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut u64>() + 7) & !7);
                                    *(__va_p as *const *mut u64)
                                };
                            let p_i2: *mut i32 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                    *(__va_p as *const *mut i32)
                                };
                            unsafe { *p_i1 = r_log_est as i32 };
                            unsafe {
                                *p_u64 = unsafe { sqlite3_log_est_to_int(r_log_est) }
                            };
                            unsafe {
                                *p_i2 = unsafe { sqlite3_log_est(unsafe { *p_u64 }) } as i32
                            };
                            break '__s41;
                        }
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    34 => {
                        {
                            let z: *const i8 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                    *(__va_p as *const *const i8)
                                };
                            let p_r: *mut f64 =
                                unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut f64>() + 7) & !7);
                                    *(__va_p as *const *mut f64)
                                };
                            rc = unsafe { sqlite3_ato_f(z, p_r) };
                            break '__s41;
                        }
                        { break '__s41; }
                    }
                    14 => { { break '__s41; } }
                    _ => {}
                }
            }
            ();
            return rc;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_cacheflush(db: *mut Sqlite3) -> i32 {
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut b_seen_busy: i32 = 0;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe { sqlite3_btree_enter_all(db) };
    {
        i = 0;
        '__b43: loop {
            if !(rc == 0 && i < unsafe { (*db).n_db }) { break '__b43; }
            '__c43: loop {
                let p_bt: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                if !(p_bt).is_null() &&
                        unsafe { sqlite3_btree_txn_state(p_bt) } == 2 {
                    let p_pager: *mut Pager =
                        unsafe { sqlite3_btree_pager(p_bt) };
                    rc = unsafe { sqlite3_pager_flush(p_pager) };
                    if rc == 5 { b_seen_busy = 1; rc = 0; }
                }
                break '__c43;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_btree_leave_all(db) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return if rc == 0 && b_seen_busy != 0 { 5 } else { rc };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_system_errno(db: *const Sqlite3) -> i32 {
    let mut i_ret: i32 = 0;
    if !(db).is_null() {
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        i_ret = unsafe { (*db).i_sys_errno };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    }
    return i_ret;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_corrupt_error(lineno: i32) -> i32 {
    return sqlite3_report_error(11, lineno,
            c"database corruption".as_ptr() as *mut i8 as *const i8);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_cantopen_error(lineno: i32) -> i32 {
    return sqlite3_report_error(14, lineno,
            c"cannot open file".as_ptr() as *mut i8 as *const i8);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_is_binary(p: *const CollSeq) -> i32 {
    { let _ = 0; };
    return (p == core::ptr::null() ||
                unsafe { (*p).x_cmp } == Some(bin_coll_func)) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_invoke_busy_handler(p: &mut BusyHandler) -> i32 {
    let mut rc: i32 = 0;
    if !(*p).x_busy_handler.is_some() as i32 != 0 || (*p).n_busy < 0 {
        return 0;
    }
    rc =
        unsafe { (*p).x_busy_handler.unwrap()((*p).p_busy_arg, (*p).n_busy) };
    if rc == 0 {
        (*p).n_busy = -1;
    } else { { let __p = &mut (*p).n_busy; let __t = *__p; *__p += 1; __t }; }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_temp_in_memory(db: &Sqlite3) -> i32 {
    return ((*db).temp_store as i32 == 2) as i32;
}
type LOGFUNCT = unsafe extern "C" fn(*mut (), i32, *const i8) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
    op: i32,
    mask: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct OpenModeN8OpenMode {
    z: *const i8,
    mode: i32,
}
type Sqlite3FaultFuncType = unsafe extern "C" fn(i32) -> i32;
type VoidFunction = unsafe extern "C" fn() -> ();
type Sqlite3LocaltimeType = unsafe extern "C" fn(*const (), *mut ()) -> i32;
static m_anytime_config_option: u64 =
    (0 as u64 | (1 as u64) << 16 | (1 as u64) << 24) as u64;
static a_flag_op: [Sqlite3DbConfigS0N20sqlite3DbConfigS0; 21] =
    [Sqlite3DbConfigS0N20sqlite3DbConfigS0 { op: 1002, mask: 16384 as u64 },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1003,
                mask: 262144 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1015,
                mask: 2147483648u32 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1004,
                mask: 4194304 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1005,
                mask: 65536 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1006,
                mask: 2048 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1007,
                mask: 8388608 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1008,
                mask: 16777216 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1009,
                mask: 33554432 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1010,
                mask: 268435456 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1011,
                mask: (1 | 134217728) as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1012,
                mask: 67108864 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1014,
                mask: 536870912 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1013,
                mask: 1073741824 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1016,
                mask: 2 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1017,
                mask: 128 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1018,
                mask: 1024 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1019,
                mask: 4096 as u64,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1020,
                mask: (16 as u64) << 32,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1021,
                mask: (32 as u64) << 32,
            },
            Sqlite3DbConfigS0N20sqlite3DbConfigS0 {
                op: 1022,
                mask: (64 as u64) << 32,
            }];
static delays: [u8; 12] =
    [1 as u8, 2 as u8, 5 as u8, 10 as u8, 15 as u8, 20 as u8, 25 as u8,
            25 as u8, 25 as u8, 50 as u8, 50 as u8, 100 as u8];
static totals: [u8; 12] =
    [0 as u8, 1 as u8, 3 as u8, 8 as u8, 18 as u8, 33 as u8, 53 as u8,
            78 as u8, 103 as u8, 128 as u8, 178 as u8, 228 as u8];
static mut a_cache_mode: [OpenModeN8OpenMode; 3] =
    [OpenModeN8OpenMode { z: c"shared".as_ptr() as *const i8, mode: 131072 },
            OpenModeN8OpenMode {
                z: c"private".as_ptr() as *const i8,
                mode: 262144,
            }, OpenModeN8OpenMode { z: core::ptr::null(), mode: 0 }];
static mut a_open_mode: [OpenModeN8OpenMode; 5] =
    [OpenModeN8OpenMode { z: c"ro".as_ptr() as *const i8, mode: 1 },
            OpenModeN8OpenMode { z: c"rw".as_ptr() as *const i8, mode: 2 },
            OpenModeN8OpenMode {
                z: c"rwc".as_ptr() as *const i8,
                mode: 2 | 4,
            },
            OpenModeN8OpenMode {
                z: c"memory".as_ptr() as *const i8,
                mode: 128,
            }, OpenModeN8OpenMode { z: core::ptr::null(), mode: 0 }];
static mut a_msg: [*const i8; 29] =
    [c"not an error".as_ptr() as *const i8,
            c"SQL logic error".as_ptr() as *const i8, core::ptr::null(),
            c"access permission denied".as_ptr() as *const i8,
            c"query aborted".as_ptr() as *const i8,
            c"database is locked".as_ptr() as *const i8,
            c"database table is locked".as_ptr() as *const i8,
            c"out of memory".as_ptr() as *const i8,
            c"attempt to write a readonly database".as_ptr() as *const i8,
            c"interrupted".as_ptr() as *const i8,
            c"disk I/O error".as_ptr() as *const i8,
            c"database disk image is malformed".as_ptr() as *const i8,
            c"unknown operation".as_ptr() as *const i8,
            c"database or disk is full".as_ptr() as *const i8,
            c"unable to open database file".as_ptr() as *const i8,
            c"locking protocol".as_ptr() as *const i8, core::ptr::null(),
            c"database schema has changed".as_ptr() as *const i8,
            c"string or blob too big".as_ptr() as *const i8,
            c"constraint failed".as_ptr() as *const i8,
            c"datatype mismatch".as_ptr() as *const i8,
            c"bad parameter or other API misuse".as_ptr() as *const i8,
            core::ptr::null(), c"authorization denied".as_ptr() as *const i8,
            core::ptr::null(),
            c"column index out of range".as_ptr() as *const i8,
            c"file is not a database".as_ptr() as *const i8,
            c"notification message".as_ptr() as *const i8,
            c"warning message".as_ptr() as *const i8];
static out_of_mem: [u16; 14] =
    ['o' as i32 as u16, 'u' as i32 as u16, 't' as i32 as u16,
            ' ' as i32 as u16, 'o' as i32 as u16, 'f' as i32 as u16,
            ' ' as i32 as u16, 'm' as i32 as u16, 'e' as i32 as u16,
            'm' as i32 as u16, 'o' as i32 as u16, 'r' as i32 as u16,
            'y' as i32 as u16, 0 as u16];
static misuse: [u16; 34] =
    ['b' as i32 as u16, 'a' as i32 as u16, 'd' as i32 as u16,
            ' ' as i32 as u16, 'p' as i32 as u16, 'a' as i32 as u16,
            'r' as i32 as u16, 'a' as i32 as u16, 'm' as i32 as u16,
            'e' as i32 as u16, 't' as i32 as u16, 'e' as i32 as u16,
            'r' as i32 as u16, ' ' as i32 as u16, 'o' as i32 as u16,
            'r' as i32 as u16, ' ' as i32 as u16, 'o' as i32 as u16,
            't' as i32 as u16, 'h' as i32 as u16, 'e' as i32 as u16,
            'r' as i32 as u16, ' ' as i32 as u16, 'A' as i32 as u16,
            'P' as i32 as u16, 'I' as i32 as u16, ' ' as i32 as u16,
            'm' as i32 as u16, 'i' as i32 as u16, 's' as i32 as u16,
            'u' as i32 as u16, 's' as i32 as u16, 'e' as i32 as u16,
            0 as u16];
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn sqlite3_compile_options(pn_opt_1: *mut i32)
    -> *mut *const i8;
    fn sqlite3_strnicmp(_: *const i8, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3_is_id_char(_: u8)
    -> i32;
    fn sqlite3_safety_check_sick_or_ok(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_log(i_err_code_1: i32, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_mutex_enter(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_btree_enter_all(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vtab_disconnect(db: *mut Sqlite3, p: *mut Table)
    -> ();
    fn sqlite3_vtab_unlock_list(_: *mut Sqlite3)
    -> ();
    fn sqlite3_btree_leave_all(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vtab_rollback(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_btree_is_in_backup(_: *mut Btree)
    -> i32;
    fn sqlite3_error_with_msg(_: *mut Sqlite3, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_mutex_leave(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_begin_benign_malloc()
    -> ();
    fn sqlite3_btree_txn_state(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_rollback(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_end_benign_malloc()
    -> ();
    fn sqlite3_expire_prepared_statements(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_reset_all_schemas_of_connection(_: *mut Sqlite3)
    -> ();
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_btree_close(_: *mut Btree)
    -> i32;
    fn sqlite3_schema_clear(_: *mut ())
    -> ();
    fn sqlite3_collapse_database_array(_: *mut Sqlite3)
    -> ();
    fn sqlite3_hash_clear(_: *mut Hash)
    -> ();
    fn sqlite3_vtab_eponymous_table_clear(_: *mut Sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_module_unref(_: *mut Sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_close_extensions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_mutex_free(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_exec(_: *mut Sqlite3, sql: *const i8,
    callback:
        Option<unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8)
            -> i32>, _: *mut (), errmsg: *mut *mut i8)
    -> i32;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_memory_barrier()
    -> ();
    fn sqlite3_mutex_init()
    -> i32;
    fn sqlite3MutexAlloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_malloc_init()
    -> i32;
    static mut sqlite3_builtin_functions: FuncDefHash;
    fn sqlite3_register_builtin_functions()
    -> ();
    fn sqlite3_pcache_initialize()
    -> i32;
    fn sqlite3OsInit()
    -> i32;
    fn sqlite3_memdb_init()
    -> i32;
    fn sqlite3_p_cache_buffer_setup(_: *mut (), sz: i32, n: i32)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_os_end()
    -> i32;
    fn sqlite3_reset_auto_extension()
    -> ();
    fn sqlite3_pcache_shutdown()
    -> ();
    fn sqlite3_malloc_end()
    -> ();
    fn sqlite3_mutex_end()
    -> i32;
    fn sqlite3_os_init()
    -> i32;
    fn sqlite3_mem_set_default()
    -> ();
    fn sqlite3_header_size_btree()
    -> i32;
    fn sqlite3_header_size_pcache()
    -> i32;
    fn sqlite3_header_size_pcache1()
    -> i32;
    fn sqlite3_p_cache_set_default()
    -> ();
    fn sqlite3_lookaside_used(_: *mut Sqlite3, _: *mut i32)
    -> i32;
    fn sqlite3Malloc(_: u64)
    -> *mut ();
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_complete(sql: *const i8)
    -> i32;
    fn sqlite3_complete16(sql: *const ())
    -> i32;
    fn sqlite3_incomplete(sql: *const i8)
    -> Sqlite3Int64;
    fn sqlite3_os_sleep(_: *mut Sqlite3Vfs, _: i32)
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
    fn sqlite3_malloc_zero(_: u64)
    -> *mut ();
    static mut sqlite3_std_type: [*const i8; 0];
    fn sqlite3_hash_init(_: *mut Hash)
    -> ();
    fn sqlite3_find_coll_seq(_: *mut Sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_hash_find(_: *const Hash, p_key_1: *const i8)
    -> *mut ();
    static sqlite3_str_binary: [i8; 0];
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3_hex_to_int(h: i32)
    -> u8;
    fn sqlite3_vfs_find(z_vfs_name_1: *const i8)
    -> *mut Sqlite3Vfs;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_btree_open(p_vfs_1: *mut Sqlite3Vfs, z_filename_1: *const i8,
    db: *mut Sqlite3, pp_btree_1: *mut *mut Btree, flags: i32,
    vfs_flags_1: i32)
    -> i32;
    fn sqlite3_btree_enter(_: *mut Btree)
    -> ();
    fn sqlite3_schema_get(_: *mut Sqlite3, _: *mut Btree)
    -> *mut Schema;
    fn sqlite3_set_text_encoding(db: *mut Sqlite3, _: u8)
    -> ();
    fn sqlite3_btree_leave(_: *mut Btree)
    -> ();
    fn sqlite3_register_per_connection_builtin_functions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_fault_sim(_: i32)
    -> i32;
    fn sqlite3_auto_load_extensions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_find_db_name(_: *mut Sqlite3, _: *const i8)
    -> i32;
    fn sqlite3_btree_checkpoint(_: *mut Btree, _: i32, _: *mut i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_api_exit(db: *mut Sqlite3, _: i32)
    -> i32;
    fn sqlite3_value_new(_: *mut Sqlite3)
    -> *mut Sqlite3Value;
    fn sqlite3_value_set_str(_: *mut Sqlite3Value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3ValueText(_: *mut Sqlite3Value, _: u8)
    -> *const ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_get_boolean(z: *const i8, _: u8)
    -> u8;
    fn sqlite3_dec_or_hex_to_i64(_: *const i8, _: *mut i64)
    -> i32;
    fn sqlite3_database_file_object(_: *const i8)
    -> *mut Sqlite3File;
    fn sqlite3_value_text(_: *mut Sqlite3Value)
    -> *const u8;
    fn sqlite3_value_text16(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_oom_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_safety_check_ok(_: *mut Sqlite3)
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
    fn sqlite3_find_function(_: *mut Sqlite3, _: *const i8, _: i32, _: u8,
    _: u8)
    -> *mut FuncDef;
    fn sqlite3_utf16to8(_: *mut Sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_aggregate_count(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_expired(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_transfer_bindings(_: *mut Sqlite3Stmt, _: *mut Sqlite3Stmt)
    -> i32;
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
    fn sqlite3_win32_set_directory(type__1: u64, z_value_1: *mut ())
    -> i32;
    fn sqlite3_win32_set_directory8(type__1: u64, z_value_1: *const i8)
    -> i32;
    fn sqlite3_win32_set_directory16(type__1: u64, z_value_1: *const ())
    -> i32;
    fn sqlite3_db_handle(_: *mut Sqlite3Stmt)
    -> *mut Sqlite3;
    fn sqlite3_btree_get_filename(_: *mut Btree)
    -> *const i8;
    fn sqlite3_btree_is_readonly(p_bt_1: *mut Btree)
    -> i32;
    fn sqlite3_next_stmt(p_db_1: *mut Sqlite3, p_stmt_1: *mut Sqlite3Stmt)
    -> *mut Sqlite3Stmt;
    fn sqlite3_enable_shared_cache(_: i32)
    -> i32;
    fn sqlite3_release_memory(_: i32)
    -> i32;
    fn sqlite3_btree_pager(_: *mut Btree)
    -> *mut Pager;
    fn sqlite3_pager_shrink(_: *mut Pager)
    -> ();
    fn sqlite3_soft_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_hard_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_soft_heap_limit(n_1: i32)
    -> ();
    fn sqlite3_init(_: *mut Sqlite3, _: *mut *mut i8)
    -> i32;
    fn sqlite3_find_table(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut Table;
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
    -> i32;
    fn sqlite3ColumnType(_: *mut Column, _: *mut i8)
    -> *mut i8;
    fn sqlite3_column_coll(_: *mut Column)
    -> *const i8;
    fn sqlite3_m_printf(_: *mut Sqlite3, _: *const i8, ...)
    -> *mut i8;
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
    fn sqlite3_vfs_register(_: *mut Sqlite3Vfs, make_dflt_1: i32)
    -> i32;
    fn sqlite3_vfs_unregister(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_mutex_try(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_held(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_notheld(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_pager_file(_: *mut Pager)
    -> *mut Sqlite3File;
    fn sqlite3_pager_vfs(_: *mut Pager)
    -> *mut Sqlite3Vfs;
    fn sqlite3_pager_jrnl_file(_: *mut Pager)
    -> *mut Sqlite3File;
    fn sqlite3_pager_data_version(_: *mut Pager)
    -> u32;
    fn sqlite3_btree_get_requested_reserve(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_set_page_size(p: *mut Btree, n_pagesize_1: i32,
    n_reserve_1: i32, e_fix_1: i32)
    -> i32;
    fn sqlite3_btree_clear_cache(_: *mut Btree)
    -> ();
    fn sqlite3_os_file_control(_: *mut Sqlite3File, _: i32, _: *mut ())
    -> i32;
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_bitvec_builtin_test(_: i32, _: *mut i32)
    -> i32;
    fn sqlite3_benign_malloc_hooks(_: Option<unsafe extern "C" fn() -> ()>,
    _: Option<unsafe extern "C" fn() -> ()>)
    -> ();
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_result_int_real(_: *mut Sqlite3Context)
    -> ();
    static mut sqlite3_tree_trace: u32;
    static mut sqlite3_where_trace: u32;
    fn sqlite3_log_est_from_double(_: f64)
    -> LogEst;
    fn sqlite3_log_est_to_int(_: LogEst)
    -> u64;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
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
    fn sqlite3_strglob(z_glob_1: *const i8, z_str_1: *const i8)
    -> i32;
    fn sqlite3_strlike(z_glob_1: *const i8, z_str_1: *const i8, c_esc_1: u32)
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
    fn sqlite3_pager_flush(_: *mut Pager)
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
    fn sqlite3_hash_insert(_: *mut Hash, p_key_1: *const i8,
    p_data_1: *mut ())
    -> *mut ();
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
    fn sqlite3_pager_mem_used(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_filename(_: *const Pager, _: i32)
    -> *const i8;
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
    fn sqlite3_btree_set_cache_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_spill_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_mmap_limit(_: *mut Btree, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_btree_set_pager_flags(_: *mut Btree, _: u32)
    -> i32;
    fn sqlite3_btree_get_page_size(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_max_page_count(_: *mut Btree, _: Pgno)
    -> Pgno;
    fn sqlite3_btree_last_page(_: *mut Btree)
    -> Pgno;
    fn sqlite3_btree_secure_delete(_: *mut Btree, _: i32)
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
    fn sqlite3_btree_begin_stmt(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_create_table(_: *mut Btree, _: *mut Pgno, flags: i32)
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
    fn sqlite3_btree_cursor_is_valid_nn(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_count(_: *mut Sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_btree_transfer_row(_: *mut BtCursor, _: *mut BtCursor, _: i64)
    -> i32;
    fn sqlite3_btree_sharable(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_enter_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_connection_count(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_leave_cursor(_: *mut BtCursor)
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
    fn sqlite3_str_i_cmp(_: *const i8, _: *const i8)
    -> i32;
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
    fn sqlite3_db_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_malloc_size(_: *mut Sqlite3, _: *const ())
    -> i32;
    fn sqlite3_page_malloc(_: i32)
    -> *mut ();
    fn sqlite3_page_free(_: *mut ())
    -> ();
    fn sqlite3_heap_nearly_full()
    -> i32;
    fn sqlite3_default_mutex()
    -> *const Sqlite3MutexMethods;
    fn sqlite3_noop_mutex()
    -> *const Sqlite3MutexMethods;
    fn sqlite3_status_value(_: i32)
    -> Sqlite3Int64;
    fn sqlite3_status_up(_: i32, _: i32)
    -> ();
    fn sqlite3_status_down(_: i32, _: i32)
    -> ();
    fn sqlite3_status_highwater(_: i32, _: i32)
    -> ();
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
    fn sqlite3_reset_one_schema(_: *mut Sqlite3, _: i32)
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
    fn sqlite3_quote_value(_: *mut StrAccum, _: *mut Sqlite3Value, _: i32)
    -> ();
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn sqlite3_register_date_time_functions()
    -> ();
    fn sqlite3_register_json_functions()
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
    fn sqlite3_log_est_add(_: LogEst, _: LogEst)
    -> LogEst;
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
    fn sqlite3_error_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_system_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_hex_to_blob(_: *mut Sqlite3, z: *const i8, n: i32)
    -> *mut ();
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
    fn sqlite3_is_memdb(_: *const Sqlite3Vfs)
    -> i32;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
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
    fn sqlite3_value_is_of_class(_: *const Sqlite3Value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3ValueBytes(_: *mut Sqlite3Value, _: u8)
    -> i32;
    fn sqlite3_value_set_null(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_apply_affinity(_: *mut Sqlite3Value, _: u8, _: u8)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    static sqlite3_std_type_len: [u8; 0];
    static sqlite3_std_type_affinity: [i8; 0];
    static sqlite3_upper_to_lower: [u8; 0];
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
    static sqlite3_oom_str: Sqlite3Str;
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
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_vtab_clear(db: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_sync(db: *mut Sqlite3, _: *mut Vdbe)
    -> i32;
    fn sqlite3_vtab_commit(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_lock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_unlock(_: *mut VTable)
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
    fn sqlite3_journal_modename(_: i32)
    -> *const i8;
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
    fn __builtin_unreachable()
    -> ();
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