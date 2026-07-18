#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;
type DarwinSizeT = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct Percentile {
    n_alloc: u32,
    n_used: u32,
    b_sorted: i8,
    b_keep_sorted: i8,
    b_pct_valid: i8,
    r_pct: f64,
    a: *mut f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PercentileFunc {
    z_name: *const i8,
    n_arg: i8,
    mx_frac: i8,
    b_discrete: i8,
}
static mut a_percent_func: [PercentileFunc; 4] =
    [PercentileFunc {
                z_name: c"median".as_ptr() as *const i8,
                n_arg: 1 as i8,
                mx_frac: 1 as i8,
                b_discrete: 0 as i8,
            },
            PercentileFunc {
                z_name: c"percentile".as_ptr() as *const i8,
                n_arg: 2 as i8,
                mx_frac: 100 as i8,
                b_discrete: 0 as i8,
            },
            PercentileFunc {
                z_name: c"percentile_cont".as_ptr() as *const i8,
                n_arg: 2 as i8,
                mx_frac: 1 as i8,
                b_discrete: 0 as i8,
            },
            PercentileFunc {
                z_name: c"percentile_disc".as_ptr() as *const i8,
                n_arg: 2 as i8,
                mx_frac: 1 as i8,
                b_discrete: 1 as i8,
            }];
extern "C" fn percent_is_infinity(mut r: f64) -> i32 {
    let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    if !(core::mem::size_of::<Sqlite3Uint64>() as u64 ==
                            core::mem::size_of::<f64>() as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"percentIsInfinity".as_ptr() as *const i8,
                c"percentile.c".as_ptr() as *mut i8 as *const i8, 163,
                c"sizeof(u)==sizeof(r)".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        memcpy(&raw mut u as *mut (), &raw mut r as *const (),
            core::mem::size_of::<Sqlite3Uint64>() as u64)
    };
    return (u >> 52 & 2047 as Sqlite3Uint64 == 2047 as u64) as i32;
}
extern "C" fn percent_same_value(mut a: f64, b: f64) -> i32 {
    a -= b;
    return (a >= -0.001 && a <= 0.001) as i32;
}
extern "C" fn percent_binary_search(p: &Percentile, y: f64, b_exact_1: i32)
    -> i32 {
    let mut i_first: i32 = 0;
    let mut i_last: i32 = ((*p).n_used - 1 as u32) as i32;
    while i_last >= i_first {
        let i_mid: i32 = (i_first + i_last) / 2;
        let x: f64 = unsafe { *(*p).a.offset(i_mid as isize) };
        if x < y {
            i_first = i_mid + 1;
        } else if x > y { i_last = i_mid - 1; } else { return i_mid; }
    }
    if b_exact_1 != 0 { return -1; }
    return i_first;
}
unsafe extern "C" fn percent_error(p_ctx_1: *mut Sqlite3Context,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let p_func: *const PercentileFunc =
        unsafe { sqlite3_user_data(p_ctx_1) } as *mut PercentileFunc as
            *const PercentileFunc;
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
                    unsafe { (*p_func).z_name })
            }
        } else { core::ptr::null_mut() };
    unsafe { sqlite3_result_error(p_ctx_1, z_msg2 as *const i8, -1) };
    unsafe { sqlite3_free(z_msg1 as *mut ()) };
    unsafe { sqlite3_free(z_msg2 as *mut ()) };
}
extern "C" fn percent_step(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut Percentile = core::ptr::null_mut();
    let mut r_pct: f64 = 0.0;
    let mut e_type: i32 = 0;
    let mut y: f64 = 0.0;
    if !(argc == 2 || argc == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"percentStep".as_ptr() as *const i8,
                c"percentile.c".as_ptr() as *mut i8 as *const i8, 236,
                c"argc==2 || argc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if argc == 1 {
        r_pct = 0.5;
    } else {
        let p_func: *const PercentileFunc =
            unsafe { sqlite3_user_data(p_ctx_1) } as *mut PercentileFunc as
                *const PercentileFunc;
        e_type =
            unsafe {
                sqlite3_value_numeric_type(unsafe {
                        *argv.offset(1 as isize)
                    })
            };
        r_pct =
            unsafe {
                    sqlite3_value_double(unsafe { *argv.offset(1 as isize) })
                } / unsafe { (*p_func).mx_frac } as f64;
        if e_type != 1 && e_type != 2 || r_pct < 0.0 || r_pct > 1.0 {
            unsafe {
                percent_error(p_ctx_1,
                    c"the fraction argument to %%s() is not between 0.0 and %.1f".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { (*p_func).mx_frac } as f64)
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
        let n: u32 = unsafe { (*p).n_alloc } * 2 as u32 + 250 as u32;
        let a: *mut f64 =
            unsafe {
                    sqlite3_realloc64(unsafe { (*p).a } as *mut (),
                        core::mem::size_of::<f64>() as u64 * n as u64)
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
    if unsafe { (*p).n_used } == 0 as u32 {
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
                            (*p).a.add((unsafe { (*p).n_used } - 1 as u32) as usize)
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
        let mut i: i32 = 0;
        i = percent_binary_search(unsafe { &*p }, y, 0);
        if i < unsafe { (*p).n_used } as i32 {
            unsafe {
                memmove(unsafe {
                            &raw mut *unsafe { (*p).a.offset((i + 1) as isize) }
                        } as *mut (),
                    unsafe { &raw mut *unsafe { (*p).a.offset(i as isize) } } as
                        *const (),
                    (unsafe { (*p).n_used } - i as u32) as u64 *
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
extern "C" fn percent_sort(mut a: *mut f64, mut n: u32) -> () {
    let mut i_lt: i32 = 0;
    let mut i_gt: i32 = 0;
    let mut i: i32 = 0;
    let mut r_pivot: f64 = 0.0;
    while n >= 2 as u32 {
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
        if unsafe { *a.offset(0 as isize) } > unsafe { *a.offset(i as isize) }
            {
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
        '__b2: loop {
            '__c2: loop {
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
                    '__b3: loop {
                        '__c3: loop {
                            { let __p = &mut i_gt; let __t = *__p; *__p -= 1; __t };
                            break '__c3;
                        }
                        if !(i_gt > i &&
                                        unsafe { *a.offset(i_gt as isize) } > r_pivot) {
                            break '__b3;
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
                break '__c2;
            }
            if !(i < i_gt) { break '__b2; }
        }
        if i_lt as u32 > n / 2 as u32 {
            if n - i_gt as u32 >= 2 as u32 {
                percent_sort(unsafe { a.offset(i_gt as isize) },
                    n - i_gt as u32);
            }
            n = i_lt as u32;
        } else {
            if i_lt >= 2 { percent_sort(a, i_lt as u32); }
            {
                let __n = i_gt;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n -= i_gt as u32;
        }
    }
}
extern "C" fn percent_inverse(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut Percentile = core::ptr::null_mut();
    let mut e_type: i32 = 0;
    let mut y: f64 = 0.0;
    let mut i: i32 = 0;
    if !(argc == 2 || argc == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"percentInverse".as_ptr() as *const i8,
                c"percentile.c".as_ptr() as *mut i8 as *const i8, 394,
                c"argc==2 || argc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<Percentile>() as i32)
            } as *mut Percentile;
    if !(p != core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"percentInverse".as_ptr() as *const i8,
                c"percentile.c".as_ptr() as *mut i8 as *const i8, 398,
                c"p!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    e_type =
        unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) };
    if e_type == 5 { return; }
    if e_type != 1 && e_type != 2 { return; }
    y = unsafe { sqlite3_value_double(unsafe { *argv.offset(0 as isize) }) };
    if percent_is_infinity(y) != 0 { return; }
    if unsafe { (*p).b_sorted } as i32 == 0 {
        if !(unsafe { (*p).n_used } > 1 as u32) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"percentInverse".as_ptr() as *const i8,
                    c"percentile.c".as_ptr() as *mut i8 as *const i8, 416,
                    c"p->nUsed>1".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        percent_sort(unsafe { (*p).a }, unsafe { (*p).n_used });
        unsafe { (*p).b_sorted = 1 as i8 };
    }
    unsafe { (*p).b_keep_sorted = 1 as i8 };
    i = percent_binary_search(unsafe { &*p }, y, 1);
    if i >= 0 {
        {
            let __p = unsafe { &mut (*p).n_used };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if i < unsafe { (*p).n_used } as i32 {
            unsafe {
                memmove(unsafe {
                            &raw mut *unsafe { (*p).a.offset(i as isize) }
                        } as *mut (),
                    unsafe {
                            &raw mut *unsafe { (*p).a.offset((i + 1) as isize) }
                        } as *const (),
                    (unsafe { (*p).n_used } - i as u32) as u64 *
                        core::mem::size_of::<f64>() as u64)
            };
        }
    }
}
extern "C" fn percent_compute(p_ctx_1: *mut Sqlite3Context, b_is_final_1: i32)
    -> () {
    let mut p: *mut Percentile = core::ptr::null_mut();
    let p_func: *const PercentileFunc =
        unsafe { sqlite3_user_data(p_ctx_1) } as *mut PercentileFunc as
            *const PercentileFunc;
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
        if unsafe { (*p).b_sorted } as i32 == 0 {
            if !(unsafe { (*p).n_used } > 1 as u32) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"percentCompute".as_ptr() as *const i8,
                        c"percentile.c".as_ptr() as *mut i8 as *const i8, 447,
                        c"p->nUsed>1".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            percent_sort(unsafe { (*p).a }, unsafe { (*p).n_used });
            unsafe { (*p).b_sorted = 1 as i8 };
        }
        ix =
            unsafe { (*p).r_pct } *
                (unsafe { (*p).n_used } - 1 as u32) as f64;
        i1 = ix as u32;
        if unsafe { (*p_func).b_discrete } != 0 {
            vx = unsafe { *unsafe { (*p).a.add(i1 as usize) } };
        } else {
            i2 =
                if ix == i1 as f64 || i1 == unsafe { (*p).n_used } - 1 as u32
                    {
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
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_percentile_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i: u32 = 0 as u32;
        { let _ = p_api_1; };
        { let _ = pz_err_msg_1; };
        {
            i = 0 as u32;
            '__b4: loop {
                if !((i as u64) <
                                core::mem::size_of::<[PercentileFunc; 4]>() as u64 /
                                    core::mem::size_of::<PercentileFunc>() as u64) {
                    break '__b4;
                }
                '__c4: loop {
                    rc =
                        unsafe {
                            sqlite3_create_window_function(db,
                                a_percent_func[i as usize].z_name,
                                a_percent_func[i as usize].n_arg as i32,
                                1 | 2097152 | 33554432,
                                &raw const a_percent_func[i as usize] as *mut (),
                                Some(percent_step), Some(percent_final),
                                Some(percent_value), Some(percent_inverse), None)
                        };
                    if rc != 0 { break '__b4; }
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}