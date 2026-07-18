#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;
type DarwinSizeT = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzerVtab {
    base: Sqlite3Vtab,
    z_class_name: *mut i8,
    p_rule: *mut FuzzerRule,
    n_cursor: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzerRule {
    p_next: *mut FuzzerRule,
    z_from: *mut i8,
    r_cost: FuzzerCost,
    n_from: FuzzerLen,
    n_to: FuzzerLen,
    i_ruleset: FuzzerRuleid,
    z_to: [i8; 4],
}
type FuzzerCost = i32;
type FuzzerLen = i8;
type FuzzerRuleid = i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzerCursor {
    base: Sqlite3VtabCursor,
    i_rowid: Sqlite3Int64,
    p_vtab: *mut FuzzerVtab,
    r_limit: FuzzerCost,
    p_stem: *mut FuzzerStem,
    p_done: *mut FuzzerStem,
    a_queue: [*mut FuzzerStem; 20],
    mx_queue: i32,
    z_buf: *mut i8,
    n_buf: i32,
    n_stem: i32,
    i_ruleset: i32,
    null_rule: FuzzerRule,
    ap_hash: [*mut FuzzerStem; 4001],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzerStem {
    z_basis: *mut i8,
    p_rule: *const FuzzerRule,
    p_next: *mut FuzzerStem,
    p_hash: *mut FuzzerStem,
    r_base_cost: FuzzerCost,
    r_cost_x: FuzzerCost,
    n_basis: FuzzerLen,
    n: FuzzerLen,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzerSeen {
    _opaque: [u8; 0],
}
extern "C" fn fuzzer_merge_rules(mut p_a_1: *mut FuzzerRule,
    mut p_b_1: *mut FuzzerRule) -> *mut FuzzerRule {
    let mut head: FuzzerRule = unsafe { core::mem::zeroed() };
    let mut p_tail: *mut FuzzerRule = core::ptr::null_mut();
    p_tail = &mut head;
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if unsafe { (*p_a_1).r_cost } <= unsafe { (*p_b_1).r_cost } {
            unsafe { (*p_tail).p_next = p_a_1 };
            p_tail = p_a_1;
            p_a_1 = unsafe { (*p_a_1).p_next };
        } else {
            unsafe { (*p_tail).p_next = p_b_1 };
            p_tail = p_b_1;
            p_b_1 = unsafe { (*p_b_1).p_next };
        }
    }
    if p_a_1 == core::ptr::null_mut() {
        unsafe { (*p_tail).p_next = p_b_1 };
    } else { unsafe { (*p_tail).p_next = p_a_1 }; }
    return head.p_next;
}
extern "C" fn fuzzer_load_one_rule(p: &FuzzerVtab, p_stmt_1: *mut Sqlite3Stmt,
    pp_rule_1: &mut *mut FuzzerRule, pz_err_1: &mut *mut i8) -> i32 {
    let i_ruleset: Sqlite3Int64 =
        unsafe { sqlite3_column_int64(p_stmt_1, 0) };
    let mut z_from: *const i8 =
        unsafe { sqlite3_column_text(p_stmt_1, 1) } as *const i8;
    let mut z_to: *const i8 =
        unsafe { sqlite3_column_text(p_stmt_1, 2) } as *const i8;
    let n_cost: i32 = unsafe { sqlite3_column_int(p_stmt_1, 3) };
    let mut rc: i32 = 0;
    let mut n_from: i32 = 0;
    let mut n_to: i32 = 0;
    let mut p_rule: *mut FuzzerRule = core::ptr::null_mut();
    if z_from == core::ptr::null() {
        z_from = c"".as_ptr() as *mut i8 as *const i8;
    }
    if z_to == core::ptr::null() {
        z_to = c"".as_ptr() as *mut i8 as *const i8;
    }
    n_from = unsafe { strlen(z_from) } as i32;
    n_to = unsafe { strlen(z_to) } as i32;
    if unsafe { strcmp(z_from, z_to) } == 0 {
        *pp_rule_1 = core::ptr::null_mut();
        return 0;
    }
    if n_cost <= 0 || n_cost > 1000 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: cost must be between 1 and %d".as_ptr()
                            as *mut i8 as *const i8, (*p).z_class_name, 1000)
            };
        rc = 1;
    } else if n_from > 50 || n_to > 50 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: maximum string length is %d".as_ptr() as
                            *mut i8 as *const i8, (*p).z_class_name, 50)
            };
        rc = 1;
    } else if i_ruleset < 0 as i64 || i_ruleset > 2147483647 as i64 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: ruleset must be between 0 and %d".as_ptr()
                            as *mut i8 as *const i8, (*p).z_class_name, 2147483647)
            };
        rc = 1;
    } else {
        p_rule =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<FuzzerRule>() as u64 +
                                n_from as u64 + n_to as u64)
                } as *mut FuzzerRule;
        if p_rule == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe {
                memset(p_rule as *mut (), 0,
                    core::mem::size_of::<FuzzerRule>() as u64)
            };
            unsafe {
                (*p_rule).z_from =
                    unsafe { &raw mut (*p_rule).z_to[0 as usize] } as *mut i8
            };
            unsafe {
                {
                    let __n = n_to + 1;
                    let __p = &mut (*p_rule).z_from;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                }
            };
            unsafe { (*p_rule).n_from = n_from as FuzzerLen };
            unsafe {
                memcpy(unsafe { (*p_rule).z_from } as *mut (),
                    z_from as *const (), (n_from + 1) as u64)
            };
            unsafe {
                memcpy(unsafe { &raw mut (*p_rule).z_to[0 as usize] } as
                            *mut i8 as *mut (), z_to as *const (), (n_to + 1) as u64)
            };
            unsafe { (*p_rule).n_to = n_to as FuzzerLen };
            unsafe { (*p_rule).r_cost = n_cost };
            unsafe { (*p_rule).i_ruleset = i_ruleset as i32 };
        }
    }
    *pp_rule_1 = p_rule;
    return rc;
}
extern "C" fn fuzzer_load_rules(db: *mut Sqlite3, p: *mut FuzzerVtab,
    z_db_1: *const i8, z_data_1: *const i8, pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p_head: *mut FuzzerRule = core::ptr::null_mut();
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT * FROM %Q.%Q".as_ptr() as *mut i8 as
                    *const i8, z_db_1, z_data_1)
        };
    if z_sql == core::ptr::null_mut() {
        rc = 7;
    } else {
        let mut rc2: i32 = 0;
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s: %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).z_class_name }, unsafe { sqlite3_errmsg(db) })
                    }
            };
        } else if unsafe { sqlite3_column_count(p_stmt) } != 4 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s: %s has %d columns, expected 4".as_ptr()
                                    as *mut i8 as *const i8, unsafe { (*p).z_class_name },
                            z_data_1, unsafe { sqlite3_column_count(p_stmt) })
                    }
            };
            rc = 1;
        } else {
            while rc == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
                let mut p_rule: *mut FuzzerRule = core::ptr::null_mut();
                rc =
                    fuzzer_load_one_rule(unsafe { &*p }, p_stmt, &mut p_rule,
                        unsafe { &mut *pz_err_1 });
                if !(p_rule).is_null() {
                    unsafe { (*p_rule).p_next = p_head };
                    p_head = p_rule;
                }
            }
        }
        rc2 = unsafe { sqlite3_finalize(p_stmt) };
        if rc == 0 { rc = rc2; }
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc == 0 {
        let mut i: u32 = 0 as u32;
        let mut p_x: *mut FuzzerRule = core::ptr::null_mut();
        let mut a: [*mut FuzzerRule; 15] = [core::ptr::null_mut(); 15];
        {
            i = 0 as u32;
            '__b2: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*mut FuzzerRule; 15]>() as u64 /
                                    core::mem::size_of::<*mut FuzzerRule>() as u64) {
                    break '__b2;
                }
                '__c2: loop {
                    a[i as usize] = core::ptr::null_mut();
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        while { p_x = p_head; p_x } != core::ptr::null_mut() {
            p_head = unsafe { (*p_x).p_next };
            unsafe { (*p_x).p_next = core::ptr::null_mut() };
            {
                i = 0 as u32;
                '__b4: loop {
                    if !(!(a[i as usize]).is_null() &&
                                    (i as u64) <
                                        core::mem::size_of::<[*mut FuzzerRule; 15]>() as u64 /
                                                core::mem::size_of::<*mut FuzzerRule>() as u64 - 1 as u64) {
                        break '__b4;
                    }
                    '__c4: loop {
                        p_x = fuzzer_merge_rules(a[i as usize], p_x);
                        a[i as usize] = core::ptr::null_mut();
                        break '__c4;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            a[i as usize] = fuzzer_merge_rules(a[i as usize], p_x);
        }
        {
            { p_x = a[0 as usize]; i = 1 as u32 };
            '__b5: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*mut FuzzerRule; 15]>() as u64 /
                                    core::mem::size_of::<*mut FuzzerRule>() as u64) {
                    break '__b5;
                }
                '__c5: loop {
                    p_x = fuzzer_merge_rules(a[i as usize], p_x);
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            (*p).p_rule = fuzzer_merge_rules(unsafe { (*p).p_rule }, p_x)
        };
    } else { { let _ = 0; }; unsafe { (*p).p_rule = p_head }; }
    return rc;
}
extern "C" fn fuzzer_dequote(z_in_1: *const i8) -> *mut i8 {
    let mut n_in: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut z_out: *mut i8 = core::ptr::null_mut();
    n_in = unsafe { strlen(z_in_1) } as Sqlite3Int64;
    z_out =
        unsafe {
                sqlite3_malloc64((n_in + 1 as Sqlite3Int64) as Sqlite3Uint64)
            } as *mut i8;
    if !(z_out).is_null() {
        let mut q: i8 = unsafe { *z_in_1.offset(0 as isize) } as i8;
        if q as i32 != '[' as i32 && q as i32 != '\'' as i32 &&
                    q as i32 != '\"' as i32 && q as i32 != '`' as i32 {
            unsafe {
                memcpy(z_out as *mut (), z_in_1 as *const (),
                    (n_in + 1 as Sqlite3Int64) as u64)
            };
        } else {
            let mut i_out: i32 = 0;
            let mut i_in: i32 = 0;
            if q as i32 == '[' as i32 { q = ']' as i32 as i8; }
            {
                i_in = 1;
                '__b6: loop {
                    if !((i_in as Sqlite3Int64) < n_in) { break '__b6; }
                    '__c6: loop {
                        if unsafe { *z_in_1.offset(i_in as isize) } as i32 ==
                                q as i32 {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                        }
                        unsafe {
                            *z_out.offset({
                                                let __p = &mut i_out;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = unsafe { *z_in_1.offset(i_in as isize) } as i8
                        };
                        break '__c6;
                    }
                    { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                }
            }
            { let _ = 0; };
            unsafe { *z_out.offset(i_out as isize) = 0 as i8 };
        }
        { let _ = 0; };
    }
    return z_out;
}
extern "C" fn fuzzer_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut FuzzerVtab = p_vtab_1 as *mut FuzzerVtab;
    { let _ = 0; };
    while !(unsafe { (*p).p_rule }).is_null() {
        let p_rule: *mut FuzzerRule = unsafe { (*p).p_rule };
        unsafe { (*p).p_rule = unsafe { (*p_rule).p_next } };
        unsafe { sqlite3_free(p_rule as *mut ()) };
    }
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
extern "C" fn fuzzer_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut p_new: *mut FuzzerVtab = core::ptr::null_mut();
    let z_module: *const i8 = unsafe { *argv.offset(0 as isize) };
    let z_db: *const i8 = unsafe { *argv.offset(1 as isize) };
    if argc != 4 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"%s: wrong number of CREATE VIRTUAL TABLE arguments".as_ptr()
                                as *mut i8 as *const i8, z_module)
                }
        };
        rc = 1;
    } else {
        let mut n_module: Sqlite3Int64 = 0 as Sqlite3Int64;
        n_module = unsafe { strlen(z_module) } as Sqlite3Int64;
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<FuzzerVtab>() as u64 +
                                n_module as u64 + 1 as u64)
                } as *mut FuzzerVtab;
        if p_new == core::ptr::null_mut() {
            rc = 7;
        } else {
            let mut z_tab: *mut i8 = core::ptr::null_mut();
            unsafe {
                memset(p_new as *mut (), 0,
                    core::mem::size_of::<FuzzerVtab>() as u64)
            };
            unsafe {
                (*p_new).z_class_name =
                    unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
            };
            unsafe {
                memcpy(unsafe { (*p_new).z_class_name } as *mut (),
                    z_module as *const (),
                    (n_module + 1 as Sqlite3Int64) as u64)
            };
            z_tab = fuzzer_dequote(unsafe { *argv.offset(3 as isize) });
            if z_tab == core::ptr::null_mut() {
                rc = 7;
            } else {
                rc =
                    fuzzer_load_rules(db, p_new, z_db, z_tab as *const i8,
                        pz_err_1);
                unsafe { sqlite3_free(z_tab as *mut ()) };
            }
            if rc == 0 {
                rc =
                    unsafe {
                        sqlite3_declare_vtab(db,
                            c"CREATE TABLE x(word,distance,ruleset)".as_ptr() as *mut i8
                                as *const i8)
                    };
            }
            if rc != 0 {
                fuzzer_disconnect(p_new as *mut Sqlite3Vtab);
                p_new = core::ptr::null_mut();
            } else { unsafe { sqlite3_vtab_config(db, 2) }; }
        }
    }
    unsafe { *pp_vtab_1 = p_new as *mut Sqlite3Vtab };
    return rc;
}
extern "C" fn fuzzer_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p: *mut FuzzerVtab = p_v_tab_1 as *mut FuzzerVtab;
    let mut p_cur: *mut FuzzerCursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<FuzzerCursor>() as
                        Sqlite3Uint64)
            } as *mut FuzzerCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<FuzzerCursor>() as u64)
    };
    unsafe { (*p_cur).p_vtab = p };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    {
        let __p = unsafe { &mut (*p).n_cursor };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return 0;
}
extern "C" fn fuzzer_clear_stem_list(mut p_stem_1: *mut FuzzerStem) -> () {
    while !(p_stem_1).is_null() {
        let p_next: *mut FuzzerStem = unsafe { (*p_stem_1).p_next };
        unsafe { sqlite3_free(p_stem_1 as *mut ()) };
        p_stem_1 = p_next;
    }
}
extern "C" fn fuzzer_clear_cursor(p_cur_1: &mut FuzzerCursor,
    clear_hash_1: i32) -> () {
    let mut i: i32 = 0;
    fuzzer_clear_stem_list((*p_cur_1).p_stem);
    fuzzer_clear_stem_list((*p_cur_1).p_done);
    {
        i = 0;
        '__b9: loop {
            if !(i < 20) { break '__b9; }
            '__c9: loop {
                fuzzer_clear_stem_list((*p_cur_1).a_queue[i as usize]);
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_cur_1).r_limit = 0 as FuzzerCost;
    if clear_hash_1 != 0 && (*p_cur_1).n_stem != 0 {
        (*p_cur_1).mx_queue = 0;
        (*p_cur_1).p_stem = core::ptr::null_mut();
        (*p_cur_1).p_done = core::ptr::null_mut();
        unsafe {
            memset(&raw mut (*p_cur_1).a_queue[0 as usize] as
                        *mut *mut FuzzerStem as *mut (), 0,
                core::mem::size_of::<[*mut FuzzerStem; 20]>() as u64)
        };
        unsafe {
            memset(&raw mut (*p_cur_1).ap_hash[0 as usize] as
                        *mut *mut FuzzerStem as *mut (), 0,
                core::mem::size_of::<[*mut FuzzerStem; 4001]>() as u64)
        };
    }
    (*p_cur_1).n_stem = 0;
}
extern "C" fn fuzzer_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut FuzzerCursor = cur as *mut FuzzerCursor;
    fuzzer_clear_cursor(unsafe { &mut *p_cur }, 0);
    unsafe { sqlite3_free(unsafe { (*p_cur).z_buf } as *mut ()) };
    {
        let __p = unsafe { &mut (*unsafe { (*p_cur).p_vtab }).n_cursor };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}
extern "C" fn fuzzer_render(p_stem_1: &FuzzerStem, pz_buf_1: &mut *mut i8,
    pn_buf_1: &mut i32) -> i32 {
    let p_rule: *const FuzzerRule = (*p_stem_1).p_rule;
    let mut n: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut z: *mut i8 = core::ptr::null_mut();
    n =
        ((*p_stem_1).n_basis as i32 + unsafe { (*p_rule).n_to } as i32 -
                unsafe { (*p_rule).n_from } as i32) as Sqlite3Int64;
    if (*pn_buf_1 as Sqlite3Int64) < n + 1 as Sqlite3Int64 {
        *pz_buf_1 =
            unsafe {
                    sqlite3_realloc64(*pz_buf_1 as *mut (),
                        (n + 100 as Sqlite3Int64) as Sqlite3Uint64)
                } as *mut i8;
        if *pz_buf_1 == core::ptr::null_mut() { return 7; }
        *pn_buf_1 = (n + 100 as Sqlite3Int64) as i32;
    }
    n = (*p_stem_1).n as Sqlite3Int64;
    z = *pz_buf_1;
    if n < 0 as i64 {
        unsafe {
            memcpy(z as *mut (), (*p_stem_1).z_basis as *const (),
                ((*p_stem_1).n_basis as i32 + 1) as u64)
        };
    } else {
        unsafe {
            memcpy(z as *mut (), (*p_stem_1).z_basis as *const (), n as u64)
        };
        unsafe {
            memcpy(unsafe { &raw mut *z.offset(n as isize) } as *mut (),
                unsafe { &raw const (*p_rule).z_to[0 as usize] } as *const i8
                    as *const (), unsafe { (*p_rule).n_to } as u64)
        };
        unsafe {
            memcpy(unsafe {
                        &raw mut *z.offset((n +
                                            unsafe { (*p_rule).n_to } as Sqlite3Int64) as isize)
                    } as *mut (),
                unsafe {
                        &raw mut *(*p_stem_1).z_basis.offset((n +
                                            unsafe { (*p_rule).n_from } as Sqlite3Int64) as isize)
                    } as *const (),
                ((*p_stem_1).n_basis as Sqlite3Int64 - n -
                            unsafe { (*p_rule).n_from } as Sqlite3Int64 +
                        1 as Sqlite3Int64) as u64)
        };
    }
    { let _ = 0; };
    return 0;
}
extern "C" fn fuzzer_hash(mut z: *const i8) -> u32 {
    let mut h: u32 = 0 as u32;
    while unsafe { *z } != 0 {
        h =
            h << 3 ^ h >> 29 ^
                unsafe {
                        *{
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as u32;
    }
    return h % 4001 as u32;
}
extern "C" fn fuzzer_cost_1(p_stem_1: &mut FuzzerStem) -> FuzzerCost {
    return {
            (*p_stem_1).r_cost_x =
                (*p_stem_1).r_base_cost +
                    unsafe { (*(*p_stem_1).p_rule).r_cost } as FuzzerCost;
            (*p_stem_1).r_cost_x
        };
}
extern "C" fn fuzzer_seen_2(p_cur_1: &mut FuzzerCursor,
    p_stem_1: *mut FuzzerStem) -> i32 {
    let mut h: u32 = 0 as u32;
    let mut p_lookup: *const FuzzerStem = core::ptr::null();
    if fuzzer_render(unsafe { &*p_stem_1 }, &mut (*p_cur_1).z_buf,
                &mut (*p_cur_1).n_buf) == 7 {
        return -1;
    }
    h = fuzzer_hash((*p_cur_1).z_buf as *const i8);
    p_lookup = (*p_cur_1).ap_hash[h as usize];
    while !(p_lookup).is_null() &&
            unsafe {
                    strcmp(unsafe { (*p_lookup).z_basis } as *const i8,
                        (*p_cur_1).z_buf as *const i8)
                } != 0 {
        p_lookup = unsafe { (*p_lookup).p_hash };
    }
    return (p_lookup != core::ptr::null_mut()) as i32;
}
extern "C" fn fuzzer_skip_rule(p_rule_1: *const FuzzerRule,
    p_stem_1: &FuzzerStem, i_ruleset_1: i32) -> i32 {
    return (!(p_rule_1).is_null() &&
                (unsafe { (*p_rule_1).i_ruleset } as FuzzerRuleid !=
                        i_ruleset_1 ||
                    (*p_stem_1).n_basis as i32 +
                                unsafe { (*p_rule_1).n_to } as i32 -
                            unsafe { (*p_rule_1).n_from } as i32 > 100)) as i32;
}
extern "C" fn fuzzer_advance(p_cur_1: *mut FuzzerCursor,
    p_stem_1: *mut FuzzerStem) -> i32 {
    let mut p_rule: *const FuzzerRule = core::ptr::null();
    while { p_rule = unsafe { (*p_stem_1).p_rule }; p_rule } !=
            core::ptr::null() {
        { let _ = 0; };
        while (unsafe { (*p_stem_1).n } as i32) <
                unsafe { (*p_stem_1).n_basis } as i32 -
                    unsafe { (*p_rule).n_from } as i32 {
            {
                let __p = unsafe { &mut (*p_stem_1).n };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p_rule).n_from } as i32 == 0 ||
                    unsafe {
                            memcmp(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_stem_1).z_basis.offset(unsafe { (*p_stem_1).n } as
                                                            isize)
                                                }
                                    } as *const (), unsafe { (*p_rule).z_from } as *const (),
                                unsafe { (*p_rule).n_from } as u64)
                        } == 0 {
                let rc: i32 =
                    fuzzer_seen_2(unsafe { &mut *p_cur_1 }, p_stem_1);
                if rc < 0 { return -1; }
                if rc == 0 {
                    fuzzer_cost_1(unsafe { &mut *p_stem_1 });
                    return 1;
                }
            }
        }
        unsafe { (*p_stem_1).n = -1 as FuzzerLen };
        '__b14: loop {
            '__c14: loop {
                p_rule = unsafe { (*p_rule).p_next } as *const FuzzerRule;
                break '__c14;
            }
            if !(fuzzer_skip_rule(p_rule, unsafe { &*p_stem_1 },
                                unsafe { (*p_cur_1).i_ruleset }) != 0) {
                break '__b14;
            }
        }
        unsafe { (*p_stem_1).p_rule = p_rule };
        if !(p_rule).is_null() &&
                fuzzer_cost_1(unsafe { &mut *p_stem_1 }) >
                    unsafe { (*p_cur_1).r_limit } {
            unsafe { (*p_stem_1).p_rule = core::ptr::null() };
        }
    }
    return 0;
}
extern "C" fn fuzzer_merge_stems(mut p_a_1: *mut FuzzerStem,
    mut p_b_1: *mut FuzzerStem) -> *mut FuzzerStem {
    let mut head: FuzzerStem = unsafe { core::mem::zeroed() };
    let mut p_tail: *mut FuzzerStem = core::ptr::null_mut();
    p_tail = &mut head;
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if unsafe { (*p_a_1).r_cost_x } <= unsafe { (*p_b_1).r_cost_x } {
            unsafe { (*p_tail).p_next = p_a_1 };
            p_tail = p_a_1;
            p_a_1 = unsafe { (*p_a_1).p_next };
        } else {
            unsafe { (*p_tail).p_next = p_b_1 };
            p_tail = p_b_1;
            p_b_1 = unsafe { (*p_b_1).p_next };
        }
    }
    if p_a_1 == core::ptr::null_mut() {
        unsafe { (*p_tail).p_next = p_b_1 };
    } else { unsafe { (*p_tail).p_next = p_a_1 }; }
    return head.p_next;
}
extern "C" fn fuzzer_lowest_cost_stem(p_cur_1: &mut FuzzerCursor)
    -> *mut FuzzerStem {
    let mut p_best: *mut FuzzerStem = core::ptr::null_mut();
    let mut p_x: *mut FuzzerStem = core::ptr::null_mut();
    let mut i_best: i32 = 0;
    let mut i: i32 = 0;
    if (*p_cur_1).p_stem == core::ptr::null_mut() {
        i_best = -1;
        p_best = core::ptr::null_mut();
        {
            i = 0;
            '__b16: loop {
                if !(i <= (*p_cur_1).mx_queue) { break '__b16; }
                '__c16: loop {
                    p_x = (*p_cur_1).a_queue[i as usize];
                    if p_x == core::ptr::null_mut() { break '__c16; }
                    if p_best == core::ptr::null_mut() ||
                            unsafe { (*p_best).r_cost_x } > unsafe { (*p_x).r_cost_x } {
                        p_best = p_x;
                        i_best = i;
                    }
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if !(p_best).is_null() {
            (*p_cur_1).a_queue[i_best as usize] = unsafe { (*p_best).p_next };
            unsafe { (*p_best).p_next = core::ptr::null_mut() };
            (*p_cur_1).p_stem = p_best;
        }
    }
    return (*p_cur_1).p_stem;
}
extern "C" fn fuzzer_insert(p_cur_1: *mut FuzzerCursor,
    mut p_new_1: *mut FuzzerStem) -> *mut FuzzerStem {
    let mut p_x: *mut FuzzerStem = core::ptr::null_mut();
    let mut i: i32 = 0;
    if { p_x = unsafe { (*p_cur_1).p_stem }; p_x } != core::ptr::null_mut() &&
            unsafe { (*p_x).r_cost_x } > unsafe { (*p_new_1).r_cost_x } {
        unsafe { (*p_new_1).p_next = core::ptr::null_mut() };
        unsafe { (*p_cur_1).p_stem = p_new_1 };
        p_new_1 = p_x;
    }
    unsafe { (*p_new_1).p_next = core::ptr::null_mut() };
    p_x = p_new_1;
    {
        i = 0;
        '__b17: loop {
            if !(i <= unsafe { (*p_cur_1).mx_queue }) { break '__b17; }
            '__c17: loop {
                if !(unsafe { (*p_cur_1).a_queue[i as usize] }).is_null() {
                    p_x =
                        fuzzer_merge_stems(p_x,
                            unsafe { (*p_cur_1).a_queue[i as usize] });
                    unsafe {
                        (*p_cur_1).a_queue[i as usize] = core::ptr::null_mut()
                    };
                } else {
                    unsafe { (*p_cur_1).a_queue[i as usize] = p_x };
                    break '__b17;
                }
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i > unsafe { (*p_cur_1).mx_queue } {
        if i < 20 {
            unsafe { (*p_cur_1).mx_queue = i };
            unsafe { (*p_cur_1).a_queue[i as usize] = p_x };
        } else {
            { let _ = 0; };
            p_x =
                fuzzer_merge_stems(p_x,
                    unsafe { (*p_cur_1).a_queue[(20 - 1) as usize] });
            unsafe { (*p_cur_1).a_queue[(20 - 1) as usize] = p_x };
        }
    }
    return fuzzer_lowest_cost_stem(unsafe { &mut *p_cur_1 });
}
extern "C" fn fuzzer_new_stem(p_cur_1: &mut FuzzerCursor, z_word_1: *const i8,
    r_base_cost_1: FuzzerCost) -> *mut FuzzerStem {
    let mut p_new: *mut FuzzerStem = core::ptr::null_mut();
    let mut p_rule: *const FuzzerRule = core::ptr::null();
    let mut h: u32 = 0 as u32;
    p_new =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<FuzzerStem>() as u64 +
                            unsafe { strlen(z_word_1) } + 1 as u64)
            } as *mut FuzzerStem;
    if p_new == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe {
        memset(p_new as *mut (), 0, core::mem::size_of::<FuzzerStem>() as u64)
    };
    unsafe {
        (*p_new).z_basis =
            unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
    };
    unsafe { (*p_new).n_basis = unsafe { strlen(z_word_1) } as FuzzerLen };
    unsafe {
        memcpy(unsafe { (*p_new).z_basis } as *mut (), z_word_1 as *const (),
            (unsafe { (*p_new).n_basis } as i32 + 1) as u64)
    };
    p_rule = unsafe { (*(*p_cur_1).p_vtab).p_rule };
    while fuzzer_skip_rule(p_rule as *const FuzzerRule, unsafe { &*p_new },
                (*p_cur_1).i_ruleset) != 0 {
        p_rule = unsafe { (*p_rule).p_next };
    }
    unsafe { (*p_new).p_rule = p_rule as *const FuzzerRule };
    unsafe { (*p_new).n = -1 as FuzzerLen };
    unsafe {
        (*p_new).r_base_cost =
            {
                unsafe { (*p_new).r_cost_x = r_base_cost_1 };
                unsafe { (*p_new).r_cost_x }
            }
    };
    h = fuzzer_hash(unsafe { (*p_new).z_basis } as *const i8);
    unsafe { (*p_new).p_hash = (*p_cur_1).ap_hash[h as usize] };
    (*p_cur_1).ap_hash[h as usize] = p_new;
    { let __p = &mut (*p_cur_1).n_stem; let __t = *__p; *__p += 1; __t };
    return p_new;
}
extern "C" fn fuzzer_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut FuzzerCursor = cur as *mut FuzzerCursor;
    let mut rc: i32 = 0;
    let mut p_stem: *mut FuzzerStem = core::ptr::null_mut();
    let mut p_new: *mut FuzzerStem = core::ptr::null_mut();
    {
        let __p = unsafe { &mut (*p_cur).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    p_stem = unsafe { (*p_cur).p_stem };
    if unsafe { (*p_stem).r_cost_x } > 0 {
        rc =
            fuzzer_render(unsafe { &*p_stem }, unsafe { &mut (*p_cur).z_buf },
                unsafe { &mut (*p_cur).n_buf });
        if rc == 7 { return 7; }
        p_new =
            fuzzer_new_stem(unsafe { &mut *p_cur },
                unsafe { (*p_cur).z_buf } as *const i8,
                unsafe { (*p_stem).r_cost_x });
        if !(p_new).is_null() {
            if fuzzer_advance(p_cur, p_new) == 0 {
                unsafe { (*p_new).p_next = unsafe { (*p_cur).p_done } };
                unsafe { (*p_cur).p_done = p_new };
            } else { if fuzzer_insert(p_cur, p_new) == p_new { return 0; } }
        } else { return 7; }
    }
    while { p_stem = unsafe { (*p_cur).p_stem }; p_stem } !=
            core::ptr::null_mut() {
        let res: i32 = fuzzer_advance(p_cur, p_stem);
        if res < 0 {
            return 7;
        } else if res > 0 {
            unsafe { (*p_cur).p_stem = core::ptr::null_mut() };
            p_stem = fuzzer_insert(p_cur, p_stem);
            if { rc = fuzzer_seen_2(unsafe { &mut *p_cur }, p_stem); rc } != 0
                {
                if rc < 0 { return 7; }
                continue;
            }
            return 0;
        }
        unsafe { (*p_cur).p_stem = core::ptr::null_mut() };
        unsafe { (*p_stem).p_next = unsafe { (*p_cur).p_done } };
        unsafe { (*p_cur).p_done = p_stem };
        if !(fuzzer_lowest_cost_stem(unsafe { &mut *p_cur })).is_null() {
            rc =
                fuzzer_seen_2(unsafe { &mut *p_cur },
                    unsafe { (*p_cur).p_stem });
            if rc < 0 { return 7; }
            if rc == 0 { return 0; }
        }
    }
    unsafe { (*p_cur).r_limit = 0 as FuzzerCost };
    return 0;
}
extern "C" fn fuzzer_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut FuzzerCursor = p_vtab_cursor_1 as *mut FuzzerCursor;
    let mut z_word: *const i8 = c"".as_ptr() as *mut i8 as *const i8;
    let mut p_stem: *mut FuzzerStem = core::ptr::null_mut();
    let mut idx: i32 = 0;
    fuzzer_clear_cursor(unsafe { &mut *p_cur }, 1);
    unsafe { (*p_cur).r_limit = 2147483647 };
    idx = 0;
    if idx_num_1 & 1 != 0 {
        z_word =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        if z_word == core::ptr::null() {
            z_word = c"".as_ptr() as *mut i8 as *const i8;
        }
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    if idx_num_1 & 2 != 0 {
        unsafe {
            (*p_cur).r_limit =
                unsafe {
                        sqlite3_value_int(unsafe { *argv.offset(idx as isize) })
                    } as FuzzerCost
        };
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    if idx_num_1 & 4 != 0 {
        unsafe {
            (*p_cur).i_ruleset =
                unsafe {
                        sqlite3_value_int(unsafe { *argv.offset(idx as isize) })
                    } as FuzzerCost
        };
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    unsafe {
        (*p_cur).null_rule.p_next =
            unsafe { (*unsafe { (*p_cur).p_vtab }).p_rule }
    };
    unsafe { (*p_cur).null_rule.r_cost = 0 };
    unsafe { (*p_cur).null_rule.n_from = 0 as FuzzerLen };
    unsafe { (*p_cur).null_rule.n_to = 0 as FuzzerLen };
    unsafe { (*p_cur).null_rule.z_from = c"".as_ptr() as *mut i8 };
    unsafe { (*p_cur).i_rowid = 1 as Sqlite3Int64 };
    { let _ = 0; };
    if (unsafe { strlen(z_word) } as i32) < 100 {
        unsafe {
            (*p_cur).p_stem =
                {
                    p_stem =
                        fuzzer_new_stem(unsafe { &mut *p_cur }, z_word,
                            0 as FuzzerCost);
                    p_stem
                }
        };
        if p_stem == core::ptr::null_mut() { return 7; }
        unsafe {
            (*p_stem).p_rule =
                unsafe { &raw mut (*p_cur).null_rule } as *const FuzzerRule
        };
        unsafe { (*p_stem).n = unsafe { (*p_stem).n_basis } };
    } else { unsafe { (*p_cur).r_limit = 0 }; }
    return 0;
}
extern "C" fn fuzzer_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *mut FuzzerCursor = cur as *mut FuzzerCursor;
    if i == 0 {
        if fuzzer_render(unsafe { &*unsafe { (*p_cur).p_stem } },
                    unsafe { &mut (*p_cur).z_buf },
                    unsafe { &mut (*p_cur).n_buf }) == 7 {
            return 7;
        }
        unsafe {
            sqlite3_result_text(ctx, unsafe { (*p_cur).z_buf } as *const i8,
                -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    } else if i == 1 {
        unsafe {
            sqlite3_result_int(ctx,
                unsafe { (*unsafe { (*p_cur).p_stem }).r_cost_x })
        };
    } else { unsafe { sqlite3_result_null(ctx) }; }
    return 0;
}
extern "C" fn fuzzer_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const FuzzerCursor =
        cur as *mut FuzzerCursor as *const FuzzerCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}
extern "C" fn fuzzer_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const FuzzerCursor =
        cur as *mut FuzzerCursor as *const FuzzerCursor;
    return (unsafe { (*p_cur).r_limit } <= 0 as FuzzerCost) as i32;
}
extern "C" fn fuzzer_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i_plan: i32 = 0;
    let mut i_dist_term: i32 = -1;
    let mut i_ruleset_term: i32 = -1;
    let mut i: i32 = 0;
    let mut seen_match: i32 = 0;
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();
    let mut r_cost: f64 = 1000000000000.0;
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b20: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b20;
            }
            '__c20: loop {
                if unsafe { (*p_constraint).i_column } as i32 == 0 &&
                        unsafe { (*p_constraint).op } as i32 == 64 {
                    seen_match = 1;
                }
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c20;
                }
                if i_plan & 1 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 0 &&
                        unsafe { (*p_constraint).op } as i32 == 64 {
                    i_plan |= 1;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = 1
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                    r_cost /= 1000000.0;
                }
                if i_plan & 2 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 1 &&
                        (unsafe { (*p_constraint).op } as i32 == 16 ||
                            unsafe { (*p_constraint).op } as i32 == 8) {
                    i_plan |= 2;
                    i_dist_term = i;
                    r_cost /= 10.0;
                }
                if i_plan & 4 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 2 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 4;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                    i_ruleset_term = i;
                    r_cost /= 10.0;
                }
                break '__c20;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_constraint;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if i_plan & 2 != 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_dist_term as
                                    isize)
                        }).argv_index = 1 + (i_plan & 1 != 0) as i32
        };
    }
    if i_plan & 4 != 0 {
        let mut idx: i32 = 1;
        if i_plan & 1 != 0 {
            { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        }
        if i_plan & 2 != 0 {
            { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        }
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_ruleset_term as
                                    isize)
                        }).argv_index = idx
        };
    }
    unsafe { (*p_idx_info_1).idx_num = i_plan };
    if unsafe { (*p_idx_info_1).n_order_by } == 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 1 &&
            unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } as i32 == 0 {
        unsafe { (*p_idx_info_1).order_by_consumed = 1 };
    }
    if seen_match != 0 && i_plan & 1 == 0 { r_cost = 1e99; }
    unsafe { (*p_idx_info_1).estimated_cost = r_cost };
    return 0;
}
static mut fuzzer_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: Some(fuzzer_connect),
        x_connect: Some(fuzzer_connect),
        x_best_index: Some(fuzzer_best_index),
        x_disconnect: Some(fuzzer_disconnect),
        x_destroy: Some(fuzzer_disconnect),
        x_open: Some(fuzzer_open),
        x_close: Some(fuzzer_close),
        x_filter: Some(fuzzer_filter),
        x_next: Some(fuzzer_next),
        x_eof: Some(fuzzer_eof),
        x_column: Some(fuzzer_column),
        x_rowid: Some(fuzzer_rowid),
        x_update: None,
        x_begin: None,
        x_sync: None,
        x_commit: None,
        x_rollback: None,
        x_find_function: None,
        x_rename: None,
        x_savepoint: None,
        x_release: None,
        x_rollback_to: None,
        x_shadow_name: None,
        x_integrity: None,
    };
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fuzzer_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"fuzzer".as_ptr() as *mut i8 as *const i8,
                    &raw mut fuzzer_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
        return rc;
    }
}
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
    fn strlen(__s: *const i8)
    -> u64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
}