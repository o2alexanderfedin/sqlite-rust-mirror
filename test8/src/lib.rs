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
type ClientData = *mut ();
type TclCommand = *mut Tcl_Command_;
#[repr(C)]
#[derive(Copy, Clone)]
struct TclDString {
    string: *mut i8,
    length: i32,
    spaceAvl: i32,
    staticSpace: [i8; 200],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct TclCmdInfo {
    isNativeObjectProc: i32,
    objProc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp, i32,
        *const *mut TclObj) -> i32>,
    objClientData: *mut (),
    proc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp, i32,
        *mut *const i8) -> i32>,
    clientData: *mut (),
    deleteProc: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    deleteData: *mut (),
    namespacePtr: *mut TclNamespace,
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
struct EchoVtab {
    base: Sqlite3Vtab,
    interp: *mut TclInterp,
    db: *mut Sqlite3,
    is_pattern: i32,
    in_transaction: i32,
    z_this: *mut i8,
    z_table_name: *mut i8,
    z_log_name: *mut i8,
    n_col: i32,
    a_index: *mut i32,
    a_col: *mut *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct EchoCursor {
    base: Sqlite3VtabCursor,
    p_stmt: *mut Sqlite3Stmt,
}
extern "C" fn simulate_vtab_error(p: &mut EchoVtab, z_method_1: *const i8)
    -> i32 {
    let mut z_err: *const i8 = core::ptr::null();
    let mut z_varname: [i8; 128] = [0; 128];
    z_varname[127 as usize] = '\u{0}' as i32 as i8;
    unsafe {
        sqlite3_snprintf(127, &raw mut z_varname[0 as usize] as *mut i8,
            c"echo_module_fail(%s,%s)".as_ptr() as *mut i8 as *const i8,
            z_method_1, (*p).z_table_name)
    };
    z_err =
        unsafe {
            Tcl_GetVar((*p).interp,
                &raw mut z_varname[0 as usize] as *mut i8 as *const i8, 1)
        };
    if !(z_err).is_null() {
        (*p).base.z_err_msg =
            unsafe {
                sqlite3_mprintf(c"echo-vtab-error: %s".as_ptr() as *mut i8 as
                        *const i8, z_err)
            };
    }
    return (z_err != core::ptr::null()) as i32;
}
extern "C" fn dequote_string(z: *mut i8) -> () {
    let mut quote: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if z == core::ptr::null_mut() { return; }
    quote = unsafe { *z.offset(0 as isize) } as i32;
    '__s0:
        {
        match quote {
            39 => {}
            34 => {}
            96 => {}
            91 => { quote = ']' as i32; }
            _ => { return; }
        }
    }
    {
        { i = 1; j = 0 };
        '__b1: loop {
            if !(unsafe { *z.offset(i as isize) } != 0) { break '__b1; }
            '__c1: loop {
                if unsafe { *z.offset(i as isize) } as i32 == quote {
                    if unsafe { *z.offset((i + 1) as isize) } as i32 == quote {
                        unsafe {
                            *z.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = quote as i8
                        };
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    } else {
                        unsafe {
                            *z.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = 0 as i8
                        };
                        break '__b1;
                    }
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
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn get_column_names(db: *mut Sqlite3, z_tab_1: *const i8,
    pa_col_1: &mut *mut *mut i8, pn_col_1: &mut i32) -> i32 {
    let mut a_col: *mut *mut i8 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut n_col: i32 = 0;
    let mut ii: i32 = 0;
    let mut n_bytes: i32 = 0;
    let mut z_space: *mut i8 = core::ptr::null_mut();
    let mut z_name: *const i8 = core::ptr::null();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s3:
            {
            match __state {
                0 => { a_col = core::ptr::null_mut(); __state = 3; }
                2 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 42; }
                3 => { __state = 4; }
                4 => { p_stmt = core::ptr::null_mut(); __state = 5; }
                5 => { rc = 0; __state = 6; }
                6 => { n_col = 0; __state = 7; }
                7 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"SELECT * FROM %Q".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1)
                        };
                    __state = 8;
                }
                8 => {
                    if (z_sql).is_null() as i32 != 0 {
                        __state = 10;
                    } else { __state = 9; }
                }
                9 => {
                    rc =
                        unsafe {
                            sqlite3_prepare(db, z_sql as *const i8, -1, &mut p_stmt,
                                core::ptr::null_mut())
                        };
                    __state = 12;
                }
                10 => { rc = 7; __state = 11; }
                11 => { __state = 2; }
                12 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 13;
                }
                13 => { if rc == 0 { __state = 15; } else { __state = 14; } }
                14 => { *pa_col_1 = a_col; __state = 40; }
                15 => { __state = 16; }
                16 => { __state = 17; }
                17 => { __state = 18; }
                18 => {
                    n_col = unsafe { sqlite3_column_count(p_stmt) };
                    __state = 19;
                }
                19 => {
                    n_bytes =
                        (core::mem::size_of::<*mut i8>() as u64 * n_col as u64) as
                            i32;
                    __state = 20;
                }
                20 => { ii = 0; __state = 22; }
                21 => {
                    a_col =
                        unsafe { sqlite3_malloc_zero(n_bytes as u64) } as
                            *mut *mut i8;
                    __state = 29;
                }
                22 => {
                    if ii < n_col { __state = 23; } else { __state = 21; }
                }
                23 => {
                    z_name = unsafe { sqlite3_column_name(p_stmt, ii) };
                    __state = 25;
                }
                24 => {
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    __state = 22;
                }
                25 => {
                    if (z_name).is_null() as i32 != 0 {
                        __state = 27;
                    } else { __state = 26; }
                }
                26 => {
                    n_bytes += unsafe { strlen(z_name) } as i32 + 1;
                    __state = 24;
                }
                27 => { rc = 7; __state = 28; }
                28 => { __state = 2; }
                29 => {
                    if (a_col).is_null() as i32 != 0 {
                        __state = 31;
                    } else { __state = 30; }
                }
                30 => {
                    z_space =
                        unsafe { &raw mut *a_col.offset(n_col as isize) } as
                            *mut i8;
                    __state = 33;
                }
                31 => { rc = 7; __state = 32; }
                32 => { __state = 2; }
                33 => { ii = 0; __state = 35; }
                34 => { { let _ = 0; }; __state = 14; }
                35 => {
                    if ii < n_col { __state = 36; } else { __state = 34; }
                }
                36 => {
                    unsafe { *a_col.offset(ii as isize) = z_space };
                    __state = 38;
                }
                37 => {
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    __state = 35;
                }
                38 => {
                    unsafe {
                        sqlite3_snprintf(n_bytes, z_space,
                            c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_column_name(p_stmt, ii) })
                    };
                    __state = 39;
                }
                39 => {
                    {
                        let __n =
                            unsafe { strlen(z_space as *const i8) } as i32 + 1;
                        let __p = &mut z_space;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 37;
                }
                40 => { *pn_col_1 = n_col; __state = 41; }
                41 => { __state = 2; }
                42 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn get_index_array(db: *mut Sqlite3, z_tab_1: *const i8,
    n_col_1: i32, pa_index_1: &mut *mut i32) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut a_index: *mut i32 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut z_idx: *const i8 = core::ptr::null();
    let mut p_stmt2: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut cid: i32 = 0;
    let mut rc2: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s5:
            {
            match __state {
                0 => { p_stmt = core::ptr::null_mut(); __state = 3; }
                2 => {
                    if !(p_stmt).is_null() {
                        __state = 37;
                    } else { __state = 36; }
                }
                3 => { a_index = core::ptr::null_mut(); __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => {
                    a_index =
                        unsafe {
                                sqlite3_malloc_zero(core::mem::size_of::<i32>() as u64 *
                                        n_col_1 as u64)
                            } as *mut i32;
                    __state = 7;
                }
                7 => {
                    if (a_index).is_null() as i32 != 0 {
                        __state = 9;
                    } else { __state = 8; }
                }
                8 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"PRAGMA index_list(%s)".as_ptr() as *mut i8
                                    as *const i8, z_tab_1)
                        };
                    __state = 11;
                }
                9 => { rc = 7; __state = 10; }
                10 => { __state = 2; }
                11 => {
                    if (z_sql).is_null() as i32 != 0 {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => {
                    rc =
                        unsafe {
                            sqlite3_prepare(db, z_sql as *const i8, -1, &mut p_stmt,
                                core::ptr::null_mut())
                        };
                    __state = 15;
                }
                13 => { rc = 7; __state = 14; }
                14 => { __state = 2; }
                15 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 16;
                }
                16 => {
                    if !(p_stmt).is_null() &&
                            unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 18;
                    } else { __state = 17; }
                }
                17 => { __state = 2; }
                18 => {
                    z_idx =
                        unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
                    __state = 19;
                }
                19 => { p_stmt2 = core::ptr::null_mut(); __state = 20; }
                20 => {
                    if z_idx == core::ptr::null() {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"PRAGMA index_info(%s)".as_ptr() as *mut i8
                                    as *const i8, z_idx)
                        };
                    __state = 23;
                }
                22 => { __state = 16; }
                23 => {
                    if (z_sql).is_null() as i32 != 0 {
                        __state = 25;
                    } else { __state = 24; }
                }
                24 => {
                    rc =
                        unsafe {
                            sqlite3_prepare(db, z_sql as *const i8, -1, &mut p_stmt2,
                                core::ptr::null_mut())
                        };
                    __state = 27;
                }
                25 => { rc = 7; __state = 26; }
                26 => { __state = 2; }
                27 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 28;
                }
                28 => {
                    if !(p_stmt2).is_null() &&
                            unsafe { sqlite3_step(p_stmt2) } == 100 {
                        __state = 30;
                    } else { __state = 29; }
                }
                29 => {
                    if !(p_stmt2).is_null() {
                        __state = 34;
                    } else { __state = 33; }
                }
                30 => {
                    cid = unsafe { sqlite3_column_int(p_stmt2, 1) };
                    __state = 31;
                }
                31 => { { let _ = 0; }; __state = 32; }
                32 => {
                    unsafe { *a_index.offset(cid as isize) = 1 };
                    __state = 29;
                }
                33 => { if rc != 0 { __state = 35; } else { __state = 16; } }
                34 => {
                    rc = unsafe { sqlite3_finalize(p_stmt2) };
                    __state = 33;
                }
                35 => { __state = 2; }
                36 => { if rc != 0 { __state = 41; } else { __state = 40; } }
                37 => {
                    rc2 = unsafe { sqlite3_finalize(p_stmt) };
                    __state = 38;
                }
                38 => { if rc == 0 { __state = 39; } else { __state = 36; } }
                39 => { rc = rc2; __state = 36; }
                40 => { *pa_index_1 = a_index; __state = 43; }
                41 => {
                    unsafe { sqlite3_free(a_index as *mut ()) };
                    __state = 42;
                }
                42 => { a_index = core::ptr::null_mut(); __state = 40; }
                43 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn append_to_echo_module(interp: *mut TclInterp,
    z_arg_1: *const i8) -> () {
    let flags: i32 = 4 | 8 | 1;
    unsafe {
        Tcl_SetVar(interp, c"echo_module".as_ptr() as *mut i8 as *const i8,
            if !(z_arg_1).is_null() {
                z_arg_1
            } else { c"".as_ptr() as *mut i8 as *const i8 }, flags)
    };
}
extern "C" fn echo_declare_vtab(p_vtab_1: &mut EchoVtab, db: *mut Sqlite3)
    -> i32 {
    let mut rc: i32 = 0;
    if !((*p_vtab_1).z_table_name).is_null() {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_prepare(db,
                    c"SELECT sql FROM sqlite_schema WHERE type = \'table\' AND name = ?".as_ptr()
                            as *mut i8 as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc == 0 {
            unsafe {
                sqlite3_bind_text(p_stmt, 1,
                    (*p_vtab_1).z_table_name as *const i8, -1, None)
            };
            if unsafe { sqlite3_step(p_stmt) } == 100 {
                let mut rc2: i32 = 0;
                let z_create_table: *const i8 =
                    unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
                rc = unsafe { sqlite3_declare_vtab(db, z_create_table) };
                rc2 = unsafe { sqlite3_finalize(p_stmt) };
                if rc == 0 { rc = rc2; }
            } else {
                rc = unsafe { sqlite3_finalize(p_stmt) };
                if rc == 0 { rc = 1; }
            }
            if rc == 0 {
                rc =
                    get_column_names(db, (*p_vtab_1).z_table_name as *const i8,
                        &mut (*p_vtab_1).a_col, &mut (*p_vtab_1).n_col);
            }
            if rc == 0 {
                rc =
                    get_index_array(db, (*p_vtab_1).z_table_name as *const i8,
                        (*p_vtab_1).n_col, &mut (*p_vtab_1).a_index);
            }
        }
    }
    return rc;
}
extern "C" fn echo_destructor(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut EchoVtab = p_vtab_1 as *mut EchoVtab;
    unsafe { sqlite3_free(unsafe { (*p).a_index } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).a_col } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).z_this } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).z_table_name } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).z_log_name } as *mut ()) };
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct EchoModule {
    interp: *mut TclInterp,
    db: *mut Sqlite3,
}
extern "C" fn echo_constructor(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: &mut *mut Sqlite3Vtab,
    pz_err_1: *const *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut p_vtab: *mut EchoVtab = core::ptr::null_mut();
    p_vtab =
        unsafe {
                sqlite3_malloc_zero(core::mem::size_of::<EchoVtab>() as u64)
            } as *mut EchoVtab;
    if (p_vtab).is_null() as i32 != 0 { return 7; }
    unsafe {
        (*p_vtab).interp = unsafe { (*(p_aux_1 as *mut EchoModule)).interp }
    };
    unsafe { (*p_vtab).db = db };
    unsafe {
        (*p_vtab).z_this =
            unsafe {
                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(2 as isize) })
            }
    };
    if (unsafe { (*p_vtab).z_this }).is_null() as i32 != 0 {
        echo_destructor(p_vtab as *mut Sqlite3Vtab);
        return 7;
    }
    if argc > 3 {
        unsafe {
            (*p_vtab).z_table_name =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { *argv.offset(3 as isize) })
                }
        };
        dequote_string(unsafe { (*p_vtab).z_table_name });
        if !(unsafe { (*p_vtab).z_table_name }).is_null() &&
                unsafe {
                            *unsafe { (*p_vtab).z_table_name.offset(0 as isize) }
                        } as i32 == '*' as i32 {
            let z: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"%s%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { *argv.offset(2 as isize) },
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_vtab).z_table_name.offset(1 as isize)
                                        }
                            } as *mut i8)
                };
            unsafe {
                sqlite3_free(unsafe { (*p_vtab).z_table_name } as *mut ())
            };
            unsafe { (*p_vtab).z_table_name = z };
            unsafe { (*p_vtab).is_pattern = 1 };
        }
        if (unsafe { (*p_vtab).z_table_name }).is_null() as i32 != 0 {
            echo_destructor(p_vtab as *mut Sqlite3Vtab);
            return 7;
        }
    }
    {
        i = 0;
        '__b6: loop {
            if !(i < argc) { break '__b6; }
            '__c6: loop {
                append_to_echo_module(unsafe { (*p_vtab).interp },
                    unsafe { *argv.offset(i as isize) });
                break '__c6;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    rc = echo_declare_vtab(unsafe { &mut *p_vtab }, db);
    if rc != 0 { echo_destructor(p_vtab as *mut Sqlite3Vtab); return rc; }
    *pp_vtab_1 = unsafe { &mut (*p_vtab).base };
    return 0;
}
extern "C" fn echo_create(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    append_to_echo_module(unsafe { (*(p_aux_1 as *mut EchoModule)).interp },
        c"xCreate".as_ptr() as *mut i8 as *const i8);
    rc =
        echo_constructor(db, p_aux_1, argc, argv, unsafe { &mut *pp_vtab_1 },
            pz_err_1 as *const *mut i8);
    if rc == 0 && argc == 5 {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let p_vtab: *mut EchoVtab =
            unsafe { *(pp_vtab_1 as *mut *mut EchoVtab) };
        unsafe {
            (*p_vtab).z_log_name =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { *argv.offset(4 as isize) })
                }
        };
        z_sql =
            unsafe {
                sqlite3_mprintf(c"CREATE TABLE %Q(logmsg)".as_ptr() as *mut i8
                        as *const i8, unsafe { (*p_vtab).z_log_name })
            };
        rc =
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if rc != 0 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(db) })
                    }
            };
        }
    }
    if !(unsafe { *pp_vtab_1 }).is_null() && rc != 0 {
        echo_destructor(unsafe { *pp_vtab_1 });
        unsafe { *pp_vtab_1 = core::ptr::null_mut() };
    }
    if rc == 0 {
        unsafe {
            (*unsafe { *(pp_vtab_1 as *mut *mut EchoVtab) }).in_transaction =
                1
        };
    }
    return rc;
}
extern "C" fn echo_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    append_to_echo_module(unsafe { (*(p_aux_1 as *mut EchoModule)).interp },
        c"xConnect".as_ptr() as *mut i8 as *const i8);
    return echo_constructor(db, p_aux_1, argc, argv,
            unsafe { &mut *pp_vtab_1 }, pz_err_1 as *const *mut i8);
}
extern "C" fn echo_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    append_to_echo_module(unsafe { (*(p_vtab_1 as *mut EchoVtab)).interp },
        c"xDisconnect".as_ptr() as *mut i8 as *const i8);
    return echo_destructor(p_vtab_1);
}
extern "C" fn echo_destroy(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let mut rc: i32 = 0;
    let p: *const EchoVtab = p_vtab_1 as *mut EchoVtab as *const EchoVtab;
    append_to_echo_module(unsafe { (*(p_vtab_1 as *mut EchoVtab)).interp },
        c"xDestroy".as_ptr() as *mut i8 as *const i8);
    if !(p).is_null() && !(unsafe { (*p).z_log_name }).is_null() {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        z_sql =
            unsafe {
                sqlite3_mprintf(c"DROP TABLE %Q".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p).z_log_name })
            };
        rc =
            unsafe {
                sqlite3_exec(unsafe { (*p).db }, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    if rc == 0 { rc = echo_destructor(p_vtab_1); }
    return rc;
}
extern "C" fn echo_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_cur: *mut EchoCursor = core::ptr::null_mut();
    if simulate_vtab_error(unsafe { &mut *(p_v_tab_1 as *mut EchoVtab) },
                c"xOpen".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    p_cur =
        unsafe {
                sqlite3_malloc_zero(core::mem::size_of::<EchoCursor>() as u64)
            } as *mut EchoCursor;
    unsafe { *pp_cursor_1 = p_cur as *mut Sqlite3VtabCursor };
    return if !(p_cur).is_null() { 0 } else { 7 };
}
extern "C" fn echo_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let mut rc: i32 = 0;
    let p_cur: *mut EchoCursor = cur as *mut EchoCursor;
    let p_stmt: *mut Sqlite3Stmt = unsafe { (*p_cur).p_stmt };
    unsafe { (*p_cur).p_stmt = core::ptr::null_mut() };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    rc = unsafe { sqlite3_finalize(p_stmt) };
    return rc;
}
extern "C" fn echo_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    return if !(unsafe { (*(cur as *mut EchoCursor)).p_stmt }).is_null() {
            0
        } else { 1 };
}
extern "C" fn echo_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_cur: *mut EchoCursor = cur as *mut EchoCursor;
        if simulate_vtab_error(unsafe {
                        &mut *(unsafe { (*cur).p_vtab } as *mut EchoVtab)
                    }, c"xNext".as_ptr() as *mut i8 as *const i8) != 0 {
            return 1;
        }
        if !(unsafe { (*p_cur).p_stmt }).is_null() {
            rc = unsafe { sqlite3_step(unsafe { (*p_cur).p_stmt }) };
            if rc == 100 {
                rc = 0;
            } else {
                rc = unsafe { sqlite3_finalize(unsafe { (*p_cur).p_stmt }) };
                unsafe { (*p_cur).p_stmt = core::ptr::null_mut() };
            }
        }
        return rc;
    }
}
extern "C" fn echo_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    unsafe {
        let i_col: i32 = i + 1;
        let p_stmt: *mut Sqlite3Stmt =
            unsafe { (*(cur as *mut EchoCursor)).p_stmt };
        if simulate_vtab_error(unsafe {
                        &mut *(unsafe { (*cur).p_vtab } as *mut EchoVtab)
                    }, c"xColumn".as_ptr() as *mut i8 as *const i8) != 0 {
            return 1;
        }
        if (p_stmt).is_null() as i32 != 0 {
            unsafe { sqlite3_result_null(ctx) };
        } else {
            { let _ = 0; };
            unsafe {
                sqlite3_result_value(ctx,
                    unsafe { sqlite3_column_value(p_stmt, i_col) })
            };
        }
        return 0;
    }
}
extern "C" fn echo_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    unsafe {
        let p_stmt: *mut Sqlite3Stmt =
            unsafe { (*(cur as *mut EchoCursor)).p_stmt };
        if simulate_vtab_error(unsafe {
                        &mut *(unsafe { (*cur).p_vtab } as *mut EchoVtab)
                    }, c"xRowid".as_ptr() as *mut i8 as *const i8) != 0 {
            return 1;
        }
        unsafe { *p_rowid_1 = unsafe { sqlite3_column_int64(p_stmt, 0) } };
        return 0;
    }
}
extern "C" fn hash_string(z_string_1: *const i8) -> i32 {
    let mut val: u32 = 0 as u32;
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b7: loop {
            if !(unsafe { *z_string_1.offset(ii as isize) } != 0) {
                break '__b7;
            }
            '__c7: loop {
                val =
                    (val << 3) +
                        unsafe { *z_string_1.offset(ii as isize) } as i32 as u32;
                break '__c7;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return (val & 2147483647 as u32) as i32;
}
extern "C" fn echo_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i: i32 = 0;
        let p_cur: *mut EchoCursor = p_vtab_cursor_1 as *mut EchoCursor;
        let p_vtab: *mut EchoVtab =
            unsafe { (*p_vtab_cursor_1).p_vtab } as *mut EchoVtab;
        let db: *mut Sqlite3 = unsafe { (*p_vtab).db };
        if simulate_vtab_error(unsafe { &mut *p_vtab },
                    c"xFilter".as_ptr() as *mut i8 as *const i8) != 0 {
            return 1;
        }
        { let _ = 0; };
        append_to_echo_module(unsafe { (*p_vtab).interp },
            c"xFilter".as_ptr() as *mut i8 as *const i8);
        append_to_echo_module(unsafe { (*p_vtab).interp }, idx_str_1);
        {
            i = 0;
            '__b8: loop {
                if !(i < argc) { break '__b8; }
                '__c8: loop {
                    append_to_echo_module(unsafe { (*p_vtab).interp },
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const i8);
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_finalize(unsafe { (*p_cur).p_stmt }) };
        unsafe { (*p_cur).p_stmt = core::ptr::null_mut() };
        rc =
            unsafe {
                sqlite3_prepare(db, idx_str_1, -1,
                    unsafe { &mut (*p_cur).p_stmt }, core::ptr::null_mut())
            };
        { let _ = 0; };
        {
            i = 0;
            '__b9: loop {
                if !(rc == 0 && i < argc) { break '__b9; }
                '__c9: loop {
                    rc =
                        unsafe {
                            sqlite3_bind_value(unsafe { (*p_cur).p_stmt }, i + 1,
                                unsafe { *argv.offset(i as isize) } as *const Sqlite3Value)
                        };
                    break '__c9;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 { rc = echo_next(p_vtab_cursor_1); }
        return rc;
    }
}
extern "C" fn string_concat(pz_str_1: *mut *mut i8, z_append_1: *mut i8,
    do_free_1: i32, p_rc_1: &mut i32) -> () {
    let mut z_in: *mut i8 = unsafe { *pz_str_1 };
    if (z_append_1).is_null() as i32 != 0 && do_free_1 != 0 && *p_rc_1 == 0 {
        *p_rc_1 = 7;
    }
    if *p_rc_1 != 0 {
        unsafe { sqlite3_free(z_in as *mut ()) };
        z_in = core::ptr::null_mut();
    } else {
        if !(z_in).is_null() {
            let z_temp: *mut i8 = z_in;
            z_in =
                unsafe {
                    sqlite3_mprintf(c"%s%s".as_ptr() as *mut i8 as *const i8,
                        z_in, z_append_1)
                };
            unsafe { sqlite3_free(z_temp as *mut ()) };
        } else {
            z_in =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_append_1)
                };
        }
        if (z_in).is_null() as i32 != 0 { *p_rc_1 = 7; }
    }
    unsafe { *pz_str_1 = z_in };
    if do_free_1 != 0 { unsafe { sqlite3_free(z_append_1 as *mut ()) }; }
}
extern "C" fn echo_select_list(p_tab_1: &EchoVtab,
    p_idx_info_1: &Sqlite3IndexInfo) -> *mut i8 {
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    if unsafe { sqlite3_libversion_number() } < 3010000 {
        z_ret =
            unsafe {
                sqlite3_mprintf(c", *".as_ptr() as *mut i8 as *const i8)
            };
    } else {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b10: loop {
                if !(i < (*p_tab_1).n_col) { break '__b10; }
                '__c10: loop {
                    if (*p_idx_info_1).col_used &
                                (1 as Sqlite3Uint64) << if i >= 63 { 63 } else { i } != 0 {
                        z_ret =
                            unsafe {
                                sqlite3_mprintf(c"%z, %s".as_ptr() as *mut i8 as *const i8,
                                    z_ret, unsafe { *(*p_tab_1).a_col.offset(i as isize) })
                            };
                    } else {
                        z_ret =
                            unsafe {
                                sqlite3_mprintf(c"%z, NULL".as_ptr() as *mut i8 as
                                        *const i8, z_ret)
                            };
                    }
                    if (z_ret).is_null() as i32 != 0 { break '__b10; }
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return z_ret;
}
extern "C" fn echo_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut ii: i32 = 0;
    let mut z_query: *mut i8 = core::ptr::null_mut();
    let mut z_col: *mut i8 = core::ptr::null_mut();
    let mut z_new: *mut i8 = core::ptr::null_mut();
    let mut n_arg: i32 = 0;
    let mut z_sep: *const i8 = c"WHERE".as_ptr() as *mut i8 as *const i8;
    let p_vtab: *mut EchoVtab = tab as *mut EchoVtab;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let interp: *mut TclInterp = unsafe { (*p_vtab).interp };
    let mut n_row: i32 = 0;
    let mut use_idx: i32 = 0;
    let mut rc: i32 = 0;
    let mut use_cost: i32 = 0;
    let mut cost: f64 = 0 as f64;
    let mut is_ignore_usable: i32 = 0;
    if !(unsafe {
                        Tcl_GetVar(interp,
                            c"echo_module_ignore_usable".as_ptr() as *mut i8 as
                                *const i8, 1)
                    }).is_null() {
        is_ignore_usable = 1;
    }
    if simulate_vtab_error(unsafe { &mut *p_vtab },
                c"xBestIndex".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    if !(unsafe {
                        Tcl_GetVar(interp,
                            c"echo_module_cost".as_ptr() as *mut i8 as *const i8, 1)
                    }).is_null() {
        cost =
            unsafe {
                atof(unsafe {
                        Tcl_GetVar(interp,
                            c"echo_module_cost".as_ptr() as *mut i8 as *const i8, 1)
                    })
            };
        use_cost = 1;
    } else {
        z_query =
            unsafe {
                sqlite3_mprintf(c"SELECT count(*) FROM %Q".as_ptr() as *mut i8
                        as *const i8, unsafe { (*p_vtab).z_table_name })
            };
        if (z_query).is_null() as i32 != 0 { return 7; }
        rc =
            unsafe {
                sqlite3_prepare(unsafe { (*p_vtab).db }, z_query as *const i8,
                    -1, &mut p_stmt, core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_query as *mut ()) };
        if rc != 0 { return rc; }
        unsafe { sqlite3_step(p_stmt) };
        n_row = unsafe { sqlite3_column_int(p_stmt, 0) };
        rc = unsafe { sqlite3_finalize(p_stmt) };
        if rc != 0 { return rc; }
    }
    z_col = echo_select_list(unsafe { &*p_vtab }, unsafe { &*p_idx_info_1 });
    if (z_col).is_null() as i32 != 0 { return 7; }
    z_query =
        unsafe {
            sqlite3_mprintf(c"SELECT rowid%z FROM %Q".as_ptr() as *mut i8 as
                    *const i8, z_col, unsafe { (*p_vtab).z_table_name })
        };
    if (z_query).is_null() as i32 != 0 { return 7; }
    {
        ii = 0;
        '__b11: loop {
            if !(ii < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b11;
            }
            '__c11: loop {
                let mut p_constraint: *const Sqlite3IndexConstraint =
                    core::ptr::null();
                let mut p_usage: *mut Sqlite3IndexConstraintUsage =
                    core::ptr::null_mut();
                let mut i_col: i32 = 0;
                p_constraint =
                    unsafe {
                            unsafe { (*p_idx_info_1).a_constraint.offset(ii as isize) }
                        } as *const Sqlite3IndexConstraint;
                p_usage =
                    unsafe {
                        unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(ii as isize)
                        }
                    };
                if (is_ignore_usable == 0) as i32 != 0 &&
                        (unsafe { (*p_constraint).usable } == 0) as i32 != 0 {
                    break '__c11;
                }
                i_col = unsafe { (*p_constraint).i_column } as i32;
                if i_col < 0 ||
                        unsafe {
                                *unsafe { (*p_vtab).a_index.offset(i_col as isize) }
                            } != 0 {
                    let z_new_col: *mut i8 =
                        if i_col >= 0 {
                            unsafe {
                                *unsafe { (*p_vtab).a_col.offset(i_col as isize) }
                            }
                        } else { c"rowid".as_ptr() as *mut i8 };
                    let mut z_op: *mut i8 = core::ptr::null_mut();
                    use_idx = 1;
                    '__s12:
                        {
                        match unsafe { (*p_constraint).op } {
                            2 => { z_op = c"=".as_ptr() as *mut i8; }
                            16 => { z_op = c"<".as_ptr() as *mut i8; }
                            4 => { z_op = c">".as_ptr() as *mut i8; }
                            8 => { z_op = c"<=".as_ptr() as *mut i8; }
                            32 => { z_op = c">=".as_ptr() as *mut i8; }
                            64 => { z_op = c"LIKE".as_ptr() as *mut i8; }
                            65 => { z_op = c"like".as_ptr() as *mut i8; }
                            66 => { z_op = c"glob".as_ptr() as *mut i8; }
                            67 => { z_op = c"regexp".as_ptr() as *mut i8; }
                            _ => {}
                        }
                    }
                    if !(z_op).is_null() {
                        if unsafe { *z_op.offset(0 as isize) } as i32 == 'L' as i32
                            {
                            z_new =
                                unsafe {
                                    sqlite3_mprintf(c" %s %s LIKE (SELECT \'%%\'||?||\'%%\')".as_ptr()
                                                as *mut i8 as *const i8, z_sep, z_new_col)
                                };
                        } else {
                            z_new =
                                unsafe {
                                    sqlite3_mprintf(c" %s %s %s ?".as_ptr() as *mut i8 as
                                            *const i8, z_sep, z_new_col, z_op)
                                };
                        }
                        string_concat(&mut z_query, z_new, 1, &mut rc);
                        z_sep = c"AND".as_ptr() as *mut i8 as *const i8;
                        unsafe {
                            (*p_usage).argv_index =
                                { let __p = &mut n_arg; *__p += 1; *__p }
                        };
                        unsafe { (*p_usage).omit = 1 as u8 };
                    }
                }
                break '__c11;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if unsafe { (*p_idx_info_1).n_order_by } == 1 &&
            (unsafe { (*unsafe { (*p_idx_info_1).a_order_by }).i_column } < 0
                ||
                unsafe {
                        *unsafe {
                                (*p_vtab).a_index.offset(unsafe {
                                            (*unsafe { (*p_idx_info_1).a_order_by }).i_column
                                        } as isize)
                            }
                    } != 0) {
        let i_col_1: i32 =
            unsafe { (*unsafe { (*p_idx_info_1).a_order_by }).i_column };
        let z_new_col_1: *mut i8 =
            if i_col_1 >= 0 {
                unsafe {
                    *unsafe { (*p_vtab).a_col.offset(i_col_1 as isize) }
                }
            } else { c"rowid".as_ptr() as *mut i8 };
        let z_dir: *mut i8 =
            if unsafe { (*unsafe { (*p_idx_info_1).a_order_by }).desc } != 0 {
                c"DESC".as_ptr() as *mut i8
            } else { c"ASC".as_ptr() as *mut i8 };
        z_new =
            unsafe {
                sqlite3_mprintf(c" ORDER BY %s %s".as_ptr() as *mut i8 as
                        *const i8, z_new_col_1, z_dir)
            };
        string_concat(&mut z_query, z_new, 1, &mut rc);
        unsafe { (*p_idx_info_1).order_by_consumed = 1 };
    }
    append_to_echo_module(unsafe { (*p_vtab).interp },
        c"xBestIndex".as_ptr() as *mut i8 as *const i8);
    append_to_echo_module(unsafe { (*p_vtab).interp }, z_query as *const i8);
    if (z_query).is_null() as i32 != 0 { return rc; }
    unsafe { (*p_idx_info_1).idx_num = hash_string(z_query as *const i8) };
    unsafe { (*p_idx_info_1).idx_str = z_query };
    unsafe { (*p_idx_info_1).need_to_free_idx_str = 1 };
    if use_cost != 0 {
        unsafe { (*p_idx_info_1).estimated_cost = cost };
    } else if use_idx != 0 {
        {
            ii = 0;
            '__b13: loop {
                if !((ii as u64) <
                                core::mem::size_of::<i32>() as u64 * 8 as u64 - 1 as u64) {
                    break '__b13;
                }
                '__c13: loop {
                    if n_row & 1 << ii != 0 {
                        unsafe { (*p_idx_info_1).estimated_cost = ii as f64 };
                    }
                    break '__c13;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
    } else { unsafe { (*p_idx_info_1).estimated_cost = n_row as f64 }; }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn echo_update(tab: *mut Sqlite3Vtab, n_data_1: i32,
    ap_data_1: *mut *mut Sqlite3Value, p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_vtab: *mut EchoVtab = tab as *mut EchoVtab;
    let db: *mut Sqlite3 = unsafe { (*p_vtab).db };
    let mut rc: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut bind_arg_zero: i32 = 0;
    let mut bind_arg_one: i32 = 0;
    let mut i: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if simulate_vtab_error(unsafe { &mut *p_vtab },
                c"xUpdate".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    if n_data_1 > 1 &&
            unsafe {
                    sqlite3_value_type(unsafe { *ap_data_1.offset(0 as isize) })
                } == 1 {
        let mut z_sep: *mut i8 = c" SET".as_ptr() as *mut i8;
        z =
            unsafe {
                sqlite3_mprintf(c"UPDATE %Q".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_vtab).z_table_name })
            };
        if (z).is_null() as i32 != 0 { rc = 7; }
        bind_arg_one =
            (!(unsafe { *ap_data_1.offset(1 as isize) }).is_null() &&
                    unsafe {
                            sqlite3_value_type(unsafe { *ap_data_1.offset(1 as isize) })
                        } == 1) as i32;
        bind_arg_zero = 1;
        if bind_arg_one != 0 {
            string_concat(&mut z, c" SET rowid=?1 ".as_ptr() as *mut i8, 0,
                &mut rc);
            z_sep = c",".as_ptr() as *mut i8;
        }
        {
            i = 2;
            '__b14: loop {
                if !(i < n_data_1) { break '__b14; }
                '__c14: loop {
                    if unsafe { *ap_data_1.offset(i as isize) } ==
                            core::ptr::null_mut() {
                        break '__c14;
                    }
                    string_concat(&mut z,
                        unsafe {
                            sqlite3_mprintf(c"%s %Q=?%d".as_ptr() as *mut i8 as
                                    *const i8, z_sep,
                                unsafe {
                                    *unsafe { (*p_vtab).a_col.offset((i - 2) as isize) }
                                }, i)
                        }, 1, &mut rc);
                    z_sep = c",".as_ptr() as *mut i8;
                    break '__c14;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        string_concat(&mut z,
            unsafe {
                sqlite3_mprintf(c" WHERE rowid=?%d".as_ptr() as *mut i8 as
                        *const i8, n_data_1)
            }, 1, &mut rc);
    } else if n_data_1 == 1 &&
            unsafe {
                    sqlite3_value_type(unsafe { *ap_data_1.offset(0 as isize) })
                } == 1 {
        z =
            unsafe {
                sqlite3_mprintf(c"DELETE FROM %Q WHERE rowid = ?1".as_ptr() as
                            *mut i8 as *const i8, unsafe { (*p_vtab).z_table_name })
            };
        if (z).is_null() as i32 != 0 { rc = 7; }
        bind_arg_zero = 1;
    } else if n_data_1 > 2 &&
            unsafe {
                    sqlite3_value_type(unsafe { *ap_data_1.offset(0 as isize) })
                } == 5 {
        let mut ii: i32 = 0;
        let mut z_insert: *mut i8 = core::ptr::null_mut();
        let mut z_values: *mut i8 = core::ptr::null_mut();
        z_insert =
            unsafe {
                sqlite3_mprintf(c"INSERT INTO %Q (".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_vtab).z_table_name })
            };
        if (z_insert).is_null() as i32 != 0 { rc = 7; }
        if unsafe {
                    sqlite3_value_type(unsafe { *ap_data_1.offset(1 as isize) })
                } == 1 {
            bind_arg_one = 1;
            z_values =
                unsafe {
                    sqlite3_mprintf(c"?".as_ptr() as *mut i8 as *const i8)
                };
            string_concat(&mut z_insert, c"rowid".as_ptr() as *mut i8, 0,
                &mut rc);
        }
        { let _ = 0; };
        {
            ii = 2;
            '__b15: loop {
                if !(ii < n_data_1) { break '__b15; }
                '__c15: loop {
                    string_concat(&mut z_insert,
                        unsafe {
                            sqlite3_mprintf(c"%s%Q".as_ptr() as *mut i8 as *const i8,
                                if !(z_values).is_null() {
                                    c", ".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 },
                                unsafe {
                                    *unsafe { (*p_vtab).a_col.offset((ii - 2) as isize) }
                                })
                        }, 1, &mut rc);
                    string_concat(&mut z_values,
                        unsafe {
                            sqlite3_mprintf(c"%s?%d".as_ptr() as *mut i8 as *const i8,
                                if !(z_values).is_null() {
                                    c", ".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 }, ii)
                        }, 1, &mut rc);
                    break '__c15;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        string_concat(&mut z, z_insert, 1, &mut rc);
        string_concat(&mut z, c") VALUES(".as_ptr() as *mut i8, 0, &mut rc);
        string_concat(&mut z, z_values, 1, &mut rc);
        string_concat(&mut z, c")".as_ptr() as *mut i8, 0, &mut rc);
    } else { { let _ = 0; }; return 1; }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_prepare(db, z as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
    }
    { let _ = 0; };
    unsafe { sqlite3_free(z as *mut ()) };
    if rc == 0 {
        if bind_arg_zero != 0 {
            unsafe {
                sqlite3_bind_value(p_stmt, n_data_1,
                    unsafe { *ap_data_1.offset(0 as isize) } as
                        *const Sqlite3Value)
            };
        }
        if bind_arg_one != 0 {
            unsafe {
                sqlite3_bind_value(p_stmt, 1,
                    unsafe { *ap_data_1.offset(1 as isize) } as
                        *const Sqlite3Value)
            };
        }
        {
            i = 2;
            '__b16: loop {
                if !(i < n_data_1 && rc == 0) { break '__b16; }
                '__c16: loop {
                    if !(unsafe { *ap_data_1.offset(i as isize) }).is_null() {
                        rc =
                            unsafe {
                                sqlite3_bind_value(p_stmt, i,
                                    unsafe { *ap_data_1.offset(i as isize) } as
                                        *const Sqlite3Value)
                            };
                    }
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 {
            unsafe { sqlite3_step(p_stmt) };
            rc = unsafe { sqlite3_finalize(p_stmt) };
        } else { unsafe { sqlite3_finalize(p_stmt) }; }
    }
    if !(p_rowid_1).is_null() && rc == 0 {
        unsafe { *p_rowid_1 = unsafe { sqlite3_last_insert_rowid(db) } };
    }
    if rc != 0 {
        unsafe {
            (*tab).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"echo-vtab-error: %s".as_ptr() as *mut i8
                            as *const i8, unsafe { sqlite3_errmsg(db) })
                }
        };
    }
    return rc;
}
extern "C" fn echo_transaction_call(tab: *mut Sqlite3Vtab,
    z_call_1: *const i8) -> i32 {
    let mut z: *mut i8 = core::ptr::null_mut();
    let p_vtab: *const EchoVtab = tab as *mut EchoVtab as *const EchoVtab;
    z =
        unsafe {
            sqlite3_mprintf(c"echo(%s)".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p_vtab).z_table_name })
        };
    if z == core::ptr::null_mut() { return 7; }
    append_to_echo_module(unsafe { (*p_vtab).interp }, z_call_1);
    append_to_echo_module(unsafe { (*p_vtab).interp }, z as *const i8);
    unsafe { sqlite3_free(z as *mut ()) };
    return 0;
}
extern "C" fn echo_begin(tab: *mut Sqlite3Vtab) -> i32 {
    let mut rc: i32 = 0;
    let p_vtab: *mut EchoVtab = tab as *mut EchoVtab;
    let interp: *mut TclInterp = unsafe { (*p_vtab).interp };
    let mut z_val: *const i8 = core::ptr::null();
    { let _ = 0; };
    if simulate_vtab_error(unsafe { &mut *p_vtab },
                c"xBegin".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    rc =
        echo_transaction_call(tab,
            c"xBegin".as_ptr() as *mut i8 as *const i8);
    if rc == 0 {
        z_val =
            unsafe {
                Tcl_GetVar(interp,
                    c"echo_module_begin_fail".as_ptr() as *mut i8 as *const i8,
                    1)
            };
        if !(z_val).is_null() &&
                0 ==
                    unsafe {
                        strcmp(z_val,
                            unsafe { (*p_vtab).z_table_name } as *const i8)
                    } {
            rc = 1;
        }
    }
    if rc == 0 { unsafe { (*p_vtab).in_transaction = 1 }; }
    return rc;
}
extern "C" fn echo_sync(tab: *mut Sqlite3Vtab) -> i32 {
    let mut rc: i32 = 0;
    let p_vtab: *mut EchoVtab = tab as *mut EchoVtab;
    let interp: *mut TclInterp = unsafe { (*p_vtab).interp };
    let mut z_val: *const i8 = core::ptr::null();
    { let _ = 0; };
    if simulate_vtab_error(unsafe { &mut *p_vtab },
                c"xSync".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    rc =
        echo_transaction_call(tab, c"xSync".as_ptr() as *mut i8 as *const i8);
    if rc == 0 {
        z_val =
            unsafe {
                Tcl_GetVar(interp,
                    c"echo_module_sync_fail".as_ptr() as *mut i8 as *const i8,
                    1)
            };
        if !(z_val).is_null() &&
                0 ==
                    unsafe {
                        strcmp(z_val,
                            unsafe { (*p_vtab).z_table_name } as *const i8)
                    } {
            rc = -1;
        }
    }
    return rc;
}
extern "C" fn echo_commit(tab: *mut Sqlite3Vtab) -> i32 {
    let p_vtab: *mut EchoVtab = tab as *mut EchoVtab;
    let mut rc: i32 = 0;
    { let _ = 0; };
    if simulate_vtab_error(unsafe { &mut *p_vtab },
                c"xCommit".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    unsafe { sqlite3_begin_benign_malloc() };
    rc =
        echo_transaction_call(tab,
            c"xCommit".as_ptr() as *mut i8 as *const i8);
    unsafe { sqlite3_end_benign_malloc() };
    unsafe { (*p_vtab).in_transaction = 0 };
    return rc;
}
extern "C" fn echo_rollback(tab: *mut Sqlite3Vtab) -> i32 {
    let mut rc: i32 = 0;
    let p_vtab: *mut EchoVtab = tab as *mut EchoVtab;
    { let _ = 0; };
    rc =
        echo_transaction_call(tab,
            c"xRollback".as_ptr() as *mut i8 as *const i8);
    unsafe { (*p_vtab).in_transaction = 0 };
    return rc;
}
extern "C" fn overloaded_glob_function(p_context_1: *mut Sqlite3Context,
    n_arg_1: i32, ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let interp: *mut TclInterp =
        unsafe { sqlite3_user_data(p_context_1) } as *mut TclInterp;
    let mut str: TclDString = unsafe { core::mem::zeroed() };
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    unsafe { Tcl_DStringInit(&mut str) };
    unsafe {
        Tcl_DStringAppendElement(&mut str,
            c"::echo_glob_overload".as_ptr() as *mut i8 as *const i8)
    };
    {
        i = 0;
        '__b17: loop {
            if !(i < n_arg_1) { break '__b17; }
            '__c17: loop {
                unsafe {
                    Tcl_DStringAppendElement(&mut str,
                        unsafe {
                                    sqlite3_value_text(unsafe { *ap_arg_1.offset(i as isize) })
                                } as *mut i8 as *const i8)
                };
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    rc =
        unsafe {
            Tcl_Eval(interp, unsafe { (*&mut str).string } as *const i8)
        };
    unsafe { Tcl_DStringFree(&mut str) };
    if rc != 0 {
        unsafe {
            sqlite3_result_error(p_context_1,
                unsafe { Tcl_GetStringResult(interp) }, -1)
        };
    } else {
        unsafe {
            sqlite3_result_text(p_context_1,
                unsafe { Tcl_GetStringResult(interp) }, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
    unsafe { Tcl_ResetResult(interp) };
}
extern "C" fn echo_find_function(vtab: *mut Sqlite3Vtab, n_arg_1: i32,
    z_func_name_1: *const i8,
    px_func_1:
        *mut unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> (), pp_arg_1: *mut *mut ()) -> i32 {
    let p_vtab: *const EchoVtab = vtab as *mut EchoVtab as *const EchoVtab;
    let interp: *mut TclInterp = unsafe { (*p_vtab).interp };
    let mut info: TclCmdInfo = unsafe { core::mem::zeroed() };
    if unsafe {
                strcmp(z_func_name_1,
                    c"glob".as_ptr() as *mut i8 as *const i8)
            } != 0 {
        return 0;
    }
    if unsafe {
                Tcl_GetCommandInfo(interp,
                    c"::echo_glob_overload".as_ptr() as *mut i8 as *const i8,
                    &mut info)
            } == 0 {
        return 0;
    }
    unsafe { *px_func_1 = overloaded_glob_function };
    unsafe { *pp_arg_1 = interp as *mut () };
    return 1;
}
extern "C" fn echo_rename(vtab: *mut Sqlite3Vtab, z_new_name_1: *const i8)
    -> i32 {
    let mut rc: i32 = 0;
    let p: *mut EchoVtab = vtab as *mut EchoVtab;
    if simulate_vtab_error(unsafe { &mut *p },
                c"xRename".as_ptr() as *mut i8 as *const i8) != 0 {
        return 1;
    }
    if unsafe { (*p).is_pattern } != 0 {
        let n_this: i32 =
            unsafe { strlen(unsafe { (*p).z_this } as *const i8) } as i32;
        let z_sql: *mut i8 =
            unsafe {
                sqlite3_mprintf(c"ALTER TABLE %s RENAME TO %s%s".as_ptr() as
                            *mut i8 as *const i8, unsafe { (*p).z_table_name },
                    z_new_name_1,
                    unsafe {
                            &raw mut *unsafe {
                                        (*p).z_table_name.offset(n_this as isize)
                                    }
                        } as *mut i8)
            };
        rc =
            unsafe {
                sqlite3_exec(unsafe { (*p).db }, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    return rc;
}
extern "C" fn echo_savepoint(p_v_tab_1: *mut Sqlite3Vtab, i_savepoint_1: i32)
    -> i32 {
    { let _ = 0; };
    return 0;
}
extern "C" fn echo_release(p_v_tab_1: *mut Sqlite3Vtab, i_savepoint_1: i32)
    -> i32 {
    { let _ = 0; };
    return 0;
}
extern "C" fn echo_rollback_to(p_v_tab_1: *mut Sqlite3Vtab,
    i_savepoint_1: i32) -> i32 {
    { let _ = 0; };
    return 0;
}
static mut echo_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 1,
        x_create: Some(echo_create),
        x_connect: Some(echo_connect),
        x_best_index: Some(echo_best_index),
        x_disconnect: Some(echo_disconnect),
        x_destroy: Some(echo_destroy),
        x_open: Some(echo_open),
        x_close: Some(echo_close),
        x_filter: Some(echo_filter),
        x_next: Some(echo_next),
        x_eof: Some(echo_eof),
        x_column: Some(echo_column),
        x_rowid: Some(echo_rowid),
        x_update: Some(echo_update),
        x_begin: Some(echo_begin),
        x_sync: Some(echo_sync),
        x_commit: Some(echo_commit),
        x_rollback: Some(echo_rollback),
        x_find_function: Some(echo_find_function),
        x_rename: Some(echo_rename),
        x_savepoint: None,
        x_release: None,
        x_rollback_to: None,
        x_shadow_name: None,
        x_integrity: None,
    };
static mut echo_module_v2: Sqlite3Module =
    Sqlite3Module {
        i_version: 2,
        x_create: Some(echo_create),
        x_connect: Some(echo_connect),
        x_best_index: Some(echo_best_index),
        x_disconnect: Some(echo_disconnect),
        x_destroy: Some(echo_destroy),
        x_open: Some(echo_open),
        x_close: Some(echo_close),
        x_filter: Some(echo_filter),
        x_next: Some(echo_next),
        x_eof: Some(echo_eof),
        x_column: Some(echo_column),
        x_rowid: Some(echo_rowid),
        x_update: Some(echo_update),
        x_begin: Some(echo_begin),
        x_sync: Some(echo_sync),
        x_commit: Some(echo_commit),
        x_rollback: Some(echo_rollback),
        x_find_function: Some(echo_find_function),
        x_rename: Some(echo_rename),
        x_savepoint: Some(echo_savepoint),
        x_release: Some(echo_release),
        x_rollback_to: Some(echo_rollback_to),
        x_shadow_name: None,
        x_integrity: None,
    };
extern "C" fn module_destroy(p: *mut ()) -> () {
    let p_mod: *const EchoModule = p as *mut EchoModule as *const EchoModule;
    unsafe {
        sqlite3_create_function(unsafe { (*p_mod).db },
            c"function_that_does_not_exist_0982ma98".as_ptr() as *mut i8 as
                *const i8, 5, 1, core::ptr::null_mut(), None, None, None)
    };
    unsafe { sqlite3_free(p) };
}
extern "C" fn register_echo_module(client_data_1: ClientData,
    interp: *mut TclInterp, objc: i32, objv: *const *mut TclObj) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_mod: *mut EchoModule = core::ptr::null_mut();
        if objc != 2 {
            unsafe {
                Tcl_WrongNumArgs(interp, 1, objv,
                    c"DB".as_ptr() as *mut i8 as *const i8)
            };
            return 1;
        }
        if unsafe {
                    get_db_pointer(interp,
                        unsafe {
                                Tcl_GetString(unsafe { *objv.offset(1 as isize) })
                            } as *const i8, &mut db)
                } != 0 {
            return 1;
        }
        p_mod =
            unsafe {
                    sqlite3_malloc(core::mem::size_of::<EchoModule>() as i32)
                } as *mut EchoModule;
        unsafe { (*p_mod).interp = interp };
        unsafe { (*p_mod).db = db };
        rc =
            unsafe {
                sqlite3_create_module_v2(db,
                    c"echo".as_ptr() as *mut i8 as *const i8,
                    &raw mut echo_module as *const Sqlite3Module,
                    p_mod as *mut (), Some(module_destroy))
            };
        if rc == 0 {
            p_mod =
                unsafe {
                        sqlite3_malloc(core::mem::size_of::<EchoModule>() as i32)
                    } as *mut EchoModule;
            unsafe { (*p_mod).interp = interp };
            unsafe { (*p_mod).db = db };
            rc =
                unsafe {
                    sqlite3_create_module_v2(db,
                        c"echo_v2".as_ptr() as *mut i8 as *const i8,
                        &raw mut echo_module_v2 as *const Sqlite3Module,
                        p_mod as *mut (), Some(module_destroy))
                };
        }
        unsafe {
            Tcl_SetResult(interp, unsafe { sqlite3_err_name(rc) } as *mut i8,
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut i8) -> ()>(0 as *const ())
                })
        };
        return 0;
    }
}
extern "C" fn declare_vtab(client_data_1: ClientData, interp: *mut TclInterp,
    objc: i32, objv: *const *mut TclObj) -> i32 {
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if objc != 3 {
        unsafe {
            Tcl_WrongNumArgs(interp, 1, objv,
                c"DB SQL".as_ptr() as *mut i8 as *const i8)
        };
        return 1;
    }
    if unsafe {
                get_db_pointer(interp,
                    unsafe {
                            Tcl_GetString(unsafe { *objv.offset(1 as isize) })
                        } as *const i8, &mut db)
            } != 0 {
        return 1;
    }
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                unsafe { Tcl_GetString(unsafe { *objv.offset(2 as isize) }) }
                    as *const i8)
        };
    if rc != 0 {
        unsafe {
            Tcl_SetResult(interp, unsafe { sqlite3_errmsg(db) } as *mut i8,
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
                })
        };
        return 1;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlitetest8_init(interp: *mut TclInterp) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b18: loop {
                if !((i as u64) <
                                core::mem::size_of::<[Sqlitetest8InitS0N19sqlitetest8InitS0; 2]>()
                                        as u64 / 24) {
                    break '__b18;
                }
                '__c18: loop {
                    unsafe {
                        Tcl_CreateObjCommand(interp,
                            a_obj_cmd[i as usize].z_name as *const i8,
                            a_obj_cmd[i as
                                            usize].x_proc.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut (), *mut TclInterp, i32,
                                                *const *mut TclObj) -> i32>(0 as *const ())
                                }), a_obj_cmd[i as usize].client_data,
                            unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
                            })
                    };
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlitetest8InitS0N19sqlitetest8InitS0 {
    z_name: *mut i8,
    x_proc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp, i32,
        *const *mut TclObj) -> i32>,
    client_data: *mut (),
}
static mut a_obj_cmd: [Sqlitetest8InitS0N19sqlitetest8InitS0; 2] =
    [Sqlitetest8InitS0N19sqlitetest8InitS0 {
                z_name: c"register_echo_module".as_ptr() as *mut i8,
                x_proc: Some(register_echo_module),
                client_data: core::ptr::null_mut(),
            },
            Sqlitetest8InitS0N19sqlitetest8InitS0 {
                z_name: c"sqlite3_declare_vtab".as_ptr() as *mut i8,
                x_proc: Some(declare_vtab),
                client_data: core::ptr::null_mut(),
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
    fn Tcl_GetVar(interp: *mut TclInterp, varName: *const i8, flags: i32)
    -> *const i8;
    fn strlen(__s: *const i8)
    -> u64;
    fn Tcl_SetVar(interp: *mut TclInterp, varName: *const i8,
    newValue: *const i8, flags: i32)
    -> *const i8;
    fn atof(_: *const i8)
    -> f64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn Tcl_DStringInit(dsPtr: *mut TclDString)
    -> ();
    fn Tcl_DStringAppendElement(dsPtr: *mut TclDString, element: *const i8)
    -> *mut i8;
    fn Tcl_Eval(interp: *mut TclInterp, script: *const i8)
    -> i32;
    fn Tcl_DStringFree(dsPtr: *mut TclDString)
    -> ();
    fn Tcl_GetStringResult(interp: *mut TclInterp)
    -> *const i8;
    fn Tcl_ResetResult(interp: *mut TclInterp)
    -> ();
    fn Tcl_GetCommandInfo(interp: *mut TclInterp, cmdName: *const i8,
    infoPtr: *mut TclCmdInfo)
    -> i32;
    fn get_db_pointer(interp: *mut TclInterp, z_a_1: *const i8,
    pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_err_name(_: i32)
    -> *const i8;
    fn Tcl_WrongNumArgs(interp: *mut TclInterp, objc: i32,
    objv: *const *mut TclObj, message: *const i8)
    -> ();
    fn Tcl_GetString(objPtr: *mut TclObj)
    -> *mut i8;
    fn Tcl_SetResult(interp: *mut TclInterp, result: *mut i8,
    freeProc: unsafe extern "C" fn(*mut i8) -> ())
    -> ();
    fn Tcl_CreateObjCommand(interp: *mut TclInterp, cmdName: *const i8,
    proc:
        unsafe extern "C" fn(*mut (), *mut TclInterp, i32, *const *mut TclObj)
            -> i32, clientData: ClientData,
    deleteProc: unsafe extern "C" fn(*mut ()) -> ())
    -> TclCommand;
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
struct Tcl_Command_ {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct TclInterp {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct TclNamespace {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct TclObj {
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