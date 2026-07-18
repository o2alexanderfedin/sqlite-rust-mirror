#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct DiskUsed {
    db: *mut Sqlite3,
    context: *mut Sqlite3Context,
    p_out: *mut Sqlite3Str,
    z_su: *mut i8,
    z_schema: *const i8,
}

extern "C" fn diskused_reset(p: *mut DiskUsed) -> () {
    if !(unsafe { (*p).z_su }).is_null() {
        let z_sql: *mut i8 =
            unsafe {
                sqlite3_mprintf(c"DROP TABLE temp.%s;".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p).z_su })
            };
        if !(z_sql).is_null() {
            unsafe {
                sqlite3_exec(unsafe { (*p).db }, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
        }
    }
    unsafe { sqlite3_str_free(unsafe { (*p).p_out }) };
    unsafe { sqlite3_free(unsafe { (*p).z_su } as *mut ()) };
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<DiskUsed>() as u64)
    };
}

unsafe extern "C" fn diskused_error(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut z_err: *mut i8 = core::ptr::null_mut();
    if !(z_format_1).is_null() {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_err = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
    } else { z_err = core::ptr::null_mut(); }
    if z_err == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(unsafe { (*p).context }) };
    } else {
        unsafe {
            sqlite3_result_error(unsafe { (*p).context }, z_err as *const i8,
                -1)
        };
        unsafe { sqlite3_free(z_err as *mut ()) };
    }
    diskused_reset(p);
}

extern "C" fn diskused_v_prep(p: *mut DiskUsed, z_fmt_1: *const i8,
    ap: *mut i8) -> *mut Sqlite3Stmt {
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    z_sql = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    if z_sql == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return core::ptr::null_mut();
    }
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
    if rc != 0 {
        unsafe {
            diskused_error(p,
                c"SQL parse error: %s\nOriginal SQL: %s".as_ptr() as *mut i8
                    as *const i8, unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                z_sql)
        };
        unsafe { sqlite3_finalize(p_stmt) };
        diskused_reset(p);
        p_stmt = core::ptr::null_mut();
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    return p_stmt;
}

unsafe extern "C" fn diskused_prepare(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> *mut Sqlite3Stmt {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = diskused_v_prep(p, z_format_1, ap);
    ();
    return p_stmt;
}

extern "C" fn diskused_stmt_finish(p: *mut DiskUsed, mut rc: i32,
    p_stmt_1: *mut Sqlite3Stmt) -> i32 {
    if rc == 101 { rc = 0; }
    if rc != 0 || { rc = unsafe { sqlite3_reset(p_stmt_1) }; rc } != 0 {
        unsafe {
            diskused_error(p,
                c"SQL run-time error: %s\nOriginal SQL: %s".as_ptr() as
                        *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                unsafe { sqlite3_sql(p_stmt_1) })
        };
        diskused_reset(p);
    }
    unsafe { sqlite3_finalize(p_stmt_1) };
    return rc;
}

unsafe extern "C" fn diskused_sql(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = diskused_v_prep(p, z_format_1, ap);
    ();
    if p_stmt == core::ptr::null_mut() { return 1; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {}
    if rc == 101 {
        rc = 0;
    } else {
        unsafe {
            diskused_error(p,
                c"SQL run-time error: %s\nOriginal SQL: %s".as_ptr() as
                        *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                unsafe { sqlite3_sql(p_stmt) })
        };
        diskused_reset(p);
    }
    unsafe { sqlite3_finalize(p_stmt) };
    return rc;
}

unsafe extern "C" fn diskused_sql_int(p: *mut DiskUsed,
    pi_res_1: &mut Sqlite3Int64, z_format_1: *const i8, mut __va0: ...)
    -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = diskused_v_prep(p, z_format_1, ap);
    ();
    if p_stmt == core::ptr::null_mut() { return 1; }
    rc = unsafe { sqlite3_step(p_stmt) };
    if rc == 100 {
        *pi_res_1 = unsafe { sqlite3_column_int64(p_stmt, 0) };
        rc = 0;
    } else if rc == 101 {
        rc = 0;
    } else {
        if !(unsafe { (*p).db }).is_null() {
            unsafe {
                diskused_error(p,
                    c"SQL run-time error: %s\nOriginal SQL: %s".as_ptr() as
                            *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                    unsafe { sqlite3_sql(p_stmt) })
            };
        }
        diskused_reset(p);
    }
    unsafe { sqlite3_finalize(p_stmt) };
    return rc;
}

unsafe extern "C" fn diskused_title(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut z_first: *mut i8 = core::ptr::null_mut();
    let mut z_title: *mut i8 = core::ptr::null_mut();
    let mut n_title: u64 = 0 as u64;
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_title = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    if z_title == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return;
    }
    z_first =
        if unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } == 0 {
            c"/".as_ptr() as *mut i8
        } else { c"\n*".as_ptr() as *mut i8 };
    n_title = unsafe { strlen(z_title as *const i8) };
    if n_title >= 75 as u64 {
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s** %z\n\n".as_ptr() as *mut i8 as *const i8, z_first,
                z_title)
        };
    } else {
        let n_extra: i32 = 74 - n_title as i32;
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s** %z %.*c\n\n".as_ptr() as *mut i8 as *const i8, z_first,
                z_title, n_extra, '*' as i32)
        };
    }
}

unsafe extern "C" fn diskused_line(p: *mut DiskUsed, z_desc_1: *const i8,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut z_txt: *mut i8 = core::ptr::null_mut();
    let mut n_desc: u64 = 0 as u64;
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_txt = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    if z_txt == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return;
    }
    n_desc =
        if !(z_desc_1).is_null() {
            unsafe { strlen(z_desc_1) }
        } else { 0 as u64 };
    if n_desc >= 50 as u64 {
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s %z".as_ptr() as *mut i8 as *const i8, z_desc_1, z_txt)
        };
    } else {
        let n_extra: i32 = 50 - n_desc as i32;
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s%.*c %z".as_ptr() as *mut i8 as *const i8, z_desc_1,
                n_extra, '.' as i32, z_txt)
        };
    }
}

extern "C" fn diskused_percent(p: &DiskUsed, r: f64) -> () {
    let mut z_num: [i8; 100] = [0; 100];
    let mut z_dp: *const i8 = core::ptr::null();
    let mut n_leading_digit: i32 = 0;
    let mut sz: i32 = 0;
    unsafe {
        sqlite3_snprintf((core::mem::size_of::<[i8; 100]>() as u64 - 5 as u64)
                as i32, &raw mut z_num[0 as usize] as *mut i8,
            if r >= 10.0 {
                    c"%.3g".as_ptr() as *mut i8
                } else { c"%.2g".as_ptr() as *mut i8 } as *const i8, r)
    };
    sz =
        unsafe { strlen(&raw mut z_num[0 as usize] as *mut i8 as *const i8) }
            as i32;
    z_dp =
        unsafe {
            strchr(&raw mut z_num[0 as usize] as *mut i8 as *const i8,
                '.' as i32)
        };
    if z_dp == core::ptr::null_mut() {
        unsafe {
            memcpy(unsafe {
                        (&raw mut z_num[0 as usize] as *mut i8).offset(sz as isize)
                    } as *mut (), c".0".as_ptr() as *mut i8 as *const (),
                3 as u64)
        };
        n_leading_digit = sz;
        sz += 2;
    } else {
        n_leading_digit =
            unsafe { z_dp.offset_from(&raw mut z_num[0 as usize] as *mut i8) }
                    as i64 as i32;
    }
    if n_leading_digit < 3 {
        unsafe {
            sqlite3_str_appendchar((*p).p_out, 3 - n_leading_digit,
                ' ' as i32 as i8)
        };
    }
    unsafe {
        sqlite3_str_append((*p).p_out,
            &raw mut z_num[0 as usize] as *mut i8 as *const i8, sz)
    };
    unsafe {
        sqlite3_str_append((*p).p_out,
            c"%\n".as_ptr() as *mut i8 as *const i8, 2)
    };
}

extern "C" fn diskused_subreport(p: *mut DiskUsed, z_title_1: *mut i8,
    z_where_1: *mut i8, pgsz: Sqlite3Int64, n_page_1: Sqlite3Int64) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut nentry: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut payload: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut ovfl_payload: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut mx_payload: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut ovfl_cnt: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut leaf_pages: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut int_pages: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut ovfl_pages: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut leaf_unused: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut int_unused: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut ovfl_unused: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut int_cell: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut depth: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut cnt: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut storage: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut total_pages: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut total_unused: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut total_meta: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut rc: i32 = 0;
    if z_title_1 == core::ptr::null_mut() ||
            z_where_1 == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return 7;
    }
    p_stmt =
        unsafe {
            diskused_prepare(p,
                c"SELECT\n  sum(if(is_without_rowid OR is_index,nentry,leaf_entries)),\n  sum(payload),\n  sum(ovfl_payload),\n  max(mx_payload),\n  sum(ovfl_cnt),\n  sum(leaf_pages),\n  sum(int_pages),\n  sum(ovfl_pages),\n  sum(leaf_unused),\n  sum(int_unused),\n  sum(ovfl_unused),\n  max(depth),\n  count(*),\n  sum(int_entries)\n FROM temp.%s WHERE %s".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_su }, z_where_1)
        };
    if p_stmt == core::ptr::null_mut() { return 1; }
    rc = unsafe { sqlite3_step(p_stmt) };
    if rc == 100 {
        unsafe {
            diskused_title(p, c"%s".as_ptr() as *mut i8 as *const i8,
                z_title_1)
        };
        nentry = unsafe { sqlite3_column_int64(p_stmt, 0) };
        payload = unsafe { sqlite3_column_int64(p_stmt, 1) };
        ovfl_payload = unsafe { sqlite3_column_int64(p_stmt, 2) };
        mx_payload = unsafe { sqlite3_column_int64(p_stmt, 3) };
        ovfl_cnt = unsafe { sqlite3_column_int64(p_stmt, 4) };
        leaf_pages = unsafe { sqlite3_column_int64(p_stmt, 5) };
        int_pages = unsafe { sqlite3_column_int64(p_stmt, 6) };
        ovfl_pages = unsafe { sqlite3_column_int64(p_stmt, 7) };
        leaf_unused = unsafe { sqlite3_column_int64(p_stmt, 8) };
        int_unused = unsafe { sqlite3_column_int64(p_stmt, 9) };
        ovfl_unused = unsafe { sqlite3_column_int64(p_stmt, 10) };
        depth = unsafe { sqlite3_column_int64(p_stmt, 11) };
        cnt = unsafe { sqlite3_column_int64(p_stmt, 12) };
        int_cell = unsafe { sqlite3_column_int64(p_stmt, 13) };
        rc = 101;
        total_pages = leaf_pages + int_pages + ovfl_pages;
        unsafe {
            diskused_line(p,
                c"Percentage of total database".as_ptr() as *mut i8 as
                    *const i8, c"%.3g%%\n".as_ptr() as *mut i8 as *const i8,
                total_pages as f64 * 100.0 / n_page_1 as f64)
        };
        unsafe {
            diskused_line(p,
                c"Number of entries".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, nentry)
        };
        storage = total_pages * pgsz;
        unsafe {
            diskused_line(p,
                c"Bytes of storage consumed".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, storage)
        };
        unsafe {
            diskused_line(p,
                c"Bytes of payload".as_ptr() as *mut i8 as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, payload)
        };
        diskused_percent(unsafe { &*p },
            payload as f64 * 100.0 / storage as f64);
        if ovfl_cnt > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Bytes of payload in overflow".as_ptr() as *mut i8 as
                        *const i8, c"%-11lld ".as_ptr() as *mut i8 as *const i8,
                    ovfl_payload)
            };
            diskused_percent(unsafe { &*p },
                ovfl_payload as f64 * 100.0 / payload as f64);
        }
        total_unused = leaf_unused + int_unused + ovfl_unused;
        total_meta = storage - payload - total_unused;
        unsafe {
            diskused_line(p,
                c"Bytes of metadata".as_ptr() as *mut i8 as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, total_meta)
        };
        diskused_percent(unsafe { &*p },
            total_meta as f64 * 100.0 / storage as f64);
        if cnt == 1 as i64 {
            unsafe {
                diskused_line(p,
                    c"B-tree depth".as_ptr() as *mut i8 as *const i8,
                    c"%lld\n".as_ptr() as *mut i8 as *const i8, depth)
            };
            if int_cell > 1 as i64 {
                unsafe {
                    diskused_line(p,
                        c"Average fanout".as_ptr() as *mut i8 as *const i8,
                        c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                        (int_cell + int_pages) as f64 / int_pages as f64)
                };
            }
        }
        if nentry > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Average payload per entry".as_ptr() as *mut i8 as
                        *const i8, c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                    payload as f64 / nentry as f64)
            };
            unsafe {
                diskused_line(p,
                    c"Average unused bytes per entry".as_ptr() as *mut i8 as
                        *const i8, c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                    total_unused as f64 / nentry as f64)
            };
            unsafe {
                diskused_line(p,
                    c"Average metadata per entry".as_ptr() as *mut i8 as
                        *const i8, c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                    total_meta as f64 / nentry as f64)
            };
        }
        unsafe {
            diskused_line(p,
                c"Maximum single-entry payload".as_ptr() as *mut i8 as
                    *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                mx_payload)
        };
        if nentry > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Entries that use overflow".as_ptr() as *mut i8 as
                        *const i8, c"%-11lld ".as_ptr() as *mut i8 as *const i8,
                    ovfl_cnt)
            };
            diskused_percent(unsafe { &*p },
                ovfl_cnt as f64 * 100.0 / nentry as f64);
        }
        if int_pages > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Index pages used".as_ptr() as *mut i8 as *const i8,
                    c"%lld\n".as_ptr() as *mut i8 as *const i8, int_pages)
            };
        }
        unsafe {
            diskused_line(p,
                c"Primary pages used".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, leaf_pages)
        };
        if ovfl_cnt != 0 {
            unsafe {
                diskused_line(p,
                    c"Overflow pages used".as_ptr() as *mut i8 as *const i8,
                    c"%lld\n".as_ptr() as *mut i8 as *const i8, ovfl_pages)
            };
        }
        unsafe {
            diskused_line(p,
                c"Total pages used".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, total_pages)
        };
        if int_pages > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Unused bytes on index pages".as_ptr() as *mut i8 as
                        *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                    int_unused)
            };
        }
        unsafe {
            diskused_line(p,
                c"Unused bytes on primary pages".as_ptr() as *mut i8 as
                    *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                leaf_unused)
        };
        if ovfl_cnt != 0 {
            unsafe {
                diskused_line(p,
                    c"Unused bytes on overflow pages".as_ptr() as *mut i8 as
                        *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                    ovfl_unused)
            };
        }
        unsafe {
            diskused_line(p,
                c"Unused bytes on all pages".as_ptr() as *mut i8 as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, total_unused)
        };
        diskused_percent(unsafe { &*p },
            total_unused as f64 * 100.0 / storage as f64);
    }
    return diskused_stmt_finish(p, rc, p_stmt);
}

extern "C" fn diskused_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut rc: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut n: i32 = 0;
    let mut ii: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut pgsz: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_page: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_page_in_use: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_free_list: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_index: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_wo_rowid: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut s: DiskUsed = unsafe { core::mem::zeroed() };
    let mut r: [u64; 2] = [0; 2];
    { let _ = argc; };
    unsafe {
        memset(&raw mut s as *mut (), 0,
            core::mem::size_of::<DiskUsed>() as u64)
    };
    s.db = unsafe { sqlite3_context_db_handle(context) };
    s.context = context;
    s.p_out = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
    if unsafe { sqlite3_str_errcode(s.p_out) } != 0 {
        unsafe { diskused_error(&mut s, core::ptr::null()) };
        return;
    }
    s.z_schema =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if s.z_schema == core::ptr::null() {
        s.z_schema = c"main".as_ptr() as *mut i8 as *const i8;
    } else if unsafe {
                sqlite3_strlike(c"temp".as_ptr() as *mut i8 as *const i8,
                    s.z_schema, 0 as u32)
            } == 0 {
        diskused_reset(&mut s);
        unsafe {
            sqlite3_result_text(context,
                c"cannot analyze \"temp\"".as_ptr() as *mut i8 as *const i8,
                -1, None)
        };
        return;
    }
    ii = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT 1 FROM pragma_database_list WHERE name=%Q COLLATE nocase".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 || ii == 0 as i64 {
        diskused_reset(&mut s);
        unsafe {
            sqlite3_result_text(context,
                c"no such database".as_ptr() as *mut i8 as *const i8, -1,
                None)
        };
        return;
    }
    unsafe {
        sqlite3_randomness(core::mem::size_of::<[u64; 2]>() as i32,
            &raw mut r as *mut ())
    };
    s.z_su =
        unsafe {
            sqlite3_mprintf(c"diskused%016llx%016llx".as_ptr() as *mut i8 as
                    *const i8, r[0 as usize], r[1 as usize])
        };
    if s.z_su == core::ptr::null_mut() {
        unsafe { diskused_error(&mut s, core::ptr::null()) };
        return;
    }
    rc =
        unsafe {
            diskused_sql(&mut s,
                c"CREATE TABLE temp.%s(\n   name text,                -- A table or index\n   tblname text,             -- Table that owns name\n   is_index boolean,         -- TRUE if it is an index\n   is_without_rowid boolean, -- TRUE if WITHOUT ROWID table\n   nentry int,               -- Number of entries in the BTree\n   leaf_entries int,         -- Number of leaf entries\n   depth int,                -- Depth of the b-tree\n   payload int,              -- Total data stored in this table/index\n   ovfl_payload int,         -- Total data stored on overflow pages\n   ovfl_cnt int,             -- Number of entries that use overflow\n   mx_payload int,           -- Maximum payload size\n   int_pages int,            -- Interior pages used\n   leaf_pages int,           -- Leaf pages used\n   ovfl_pages int,           -- Overflow pages used\n   int_unused int,           -- Unused bytes on interior pages\n   leaf_unused int,          -- Unused bytes on primary pages\n   ovfl_unused int,          -- Unused bytes on overflow pages\n   int_entries int           -- Btree cells on internal pages\n);".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if rc != 0 { return; }
    rc =
        unsafe {
            diskused_sql(&mut s,
                c"WITH\n  allidx(idxname) AS (\n    SELECT name FROM \"%w\".sqlite_schema WHERE type=\'index\'\n  ),\n  allobj(allname,tblname,isidx,isworowid) AS (\n    SELECT \'sqlite_schema\',\n           \'sqlite_schema\',\n           0,\n           0\n    UNION ALL\n    SELECT name,\n           tbl_name,\n           type=\'index\',\n           EXISTS(SELECT 1\n                    FROM pragma_index_list(sqlite_schema.name,%Q)\n                   WHERE pragma_index_list.origin=\'pk\'\n                     AND pragma_index_list.name NOT IN allidx)\n      FROM \"%w\".sqlite_schema\n  )\nINSERT INTO temp.%s\n  SELECT\n    allname,\n    tblname,\n    isidx,\n    isworowid,\n    sum(ncell),\n    sum((pagetype=\'leaf\')*ncell),\n    max((length(if(path GLOB \'*+*\',\'\',path))+3)/4),\n    sum(payload),\n    sum((pagetype=\'overflow\')*payload),\n    sum(path GLOB \'*+000000\'),\n    max(mx_payload),\n    sum(pagetype=\'internal\'),\n    sum(pagetype=\'leaf\'),\n    sum(pagetype=\'overflow\'),\n    sum((pagetype=\'internal\')*unused),\n    sum((pagetype=\'leaf\')*unused),\n    sum((pagetype=\'overflow\')*unused),\n    sum(if(pagetype=\'internal\',ncell))\n  FROM allobj CROSS JOIN dbstat(%Q) \n  WHERE dbstat.name=allobj.allname\n  GROUP BY allname;\n".as_ptr()
                        as *mut i8 as *const i8, s.z_schema, s.z_schema, s.z_schema,
                s.z_su, s.z_schema)
        };
    if rc != 0 { return; }
    n_page = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_page,
                c"PRAGMA \"%w\".page_count".as_ptr() as *mut i8 as *const i8,
                s.z_schema)
        };
    if rc != 0 { return; }
    if n_page <= 0 as i64 {
        diskused_reset(&mut s);
        unsafe {
            sqlite3_result_text(context,
                c"empty database".as_ptr() as *mut i8 as *const i8, -1, None)
        };
        return;
    }
    unsafe {
        diskused_title(&mut s,
            c"Database storage utilization report".as_ptr() as *mut i8 as
                *const i8)
    };
    pgsz = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut pgsz,
                c"PRAGMA \"%w\".page_size".as_ptr() as *mut i8 as *const i8,
                s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Page size in bytes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, pgsz)
    };
    unsafe {
        diskused_line(&mut s,
            c"Pages in the database".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, n_page)
    };
    n_page_in_use = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_page_in_use,
                c"SELECT sum(leaf_pages+int_pages+ovfl_pages) FROM temp.%s".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Pages that store data".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, n_page_in_use)
    };
    diskused_percent(&s, n_page_in_use as f64 * 100.0 / n_page as f64);
    n_free_list = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_free_list,
                c"PRAGMA \"%w\".freelist_count".as_ptr() as *mut i8 as
                    *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Pages on the freelist".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, n_free_list)
    };
    diskused_percent(&s, n_free_list as f64 * 100.0 / n_page as f64);
    ii = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"PRAGMA \"%w\".auto_vacuum".as_ptr() as *mut i8 as *const i8,
                s.z_schema)
        };
    if rc != 0 { return; }
    if ii == 0 as i64 || n_page <= 1 as i64 {
        ii = 0 as Sqlite3Int64;
    } else {
        let r_ptrs_per_page: f64 = (pgsz / 5 as Sqlite3Int64) as f64;
        let r_av_page: f64 = (n_page as f64 - 1.0) / (r_ptrs_per_page + 1.0);
        ii = unsafe { ceil(r_av_page) } as Sqlite3Int64;
    }
    unsafe {
        diskused_line(&mut s,
            c"Pages of auto-vacuum overhead".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, ii)
    };
    diskused_percent(&s, ii as f64 * 100.0 / n_page as f64);
    ii = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT count(*)+1 FROM \"%w\".sqlite_schema WHERE type=\'table\'".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Number of tables".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, ii)
    };
    n_wo_rowid = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_wo_rowid,
                c"SELECT count(*) FROM \"%w\".pragma_table_list WHERE wr".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    if n_wo_rowid > 0 as i64 {
        unsafe {
            diskused_line(&mut s,
                c"Number of WITHOUT ROWID tables".as_ptr() as *mut i8 as
                    *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                n_wo_rowid)
        };
        unsafe {
            diskused_line(&mut s,
                c"Number of rowid tables".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, ii - n_wo_rowid)
        };
    }
    n_index = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_index,
                c"SELECT count(*) FROM \"%w\".sqlite_schema WHERE type=\'index\'".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Number of indexes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, n_index)
    };
    ii = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT count(*) FROM \"%w\".sqlite_schema WHERE name GLOB \'sqlite_autoindex_*\' AND type=\'index\'".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Number of defined indexes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, n_index - ii)
    };
    unsafe {
        diskused_line(&mut s,
            c"Number of implied indexes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, ii)
    };
    unsafe {
        diskused_line(&mut s,
            c"Size of the database in bytes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, pgsz * n_page)
    };
    ii = 0 as Sqlite3Int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT sum(payload) FROM temp.%s WHERE NOT is_index AND name NOT LIKE \'sqlite_schema\'".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Bytes of payload".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, ii)
    };
    diskused_percent(&s, ii as f64 * 100.0 / (pgsz * n_page) as f64);
    unsafe {
        diskused_title(&mut s,
            c"Page counts for all tables with their indexes".as_ptr() as
                    *mut i8 as *const i8)
    };
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT upper(tblname),\n       sum(int_pages+leaf_pages+ovfl_pages)\n  FROM temp.%s\n WHERE tblname IS NOT NULL\n GROUP BY 1\n ORDER BY 2 DESC, 1;".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        let nn: Sqlite3Int64 = unsafe { sqlite3_column_int64(p_stmt, 1) };
        unsafe {
            diskused_line(&mut s,
                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, nn)
        };
        diskused_percent(&s, nn as f64 * 100.0 / n_page as f64);
    }
    if diskused_stmt_finish(&mut s, rc, p_stmt) != 0 { return; }
    unsafe {
        diskused_title(&mut s,
            c"Page counts for all tables and indexes separately".as_ptr() as
                    *mut i8 as *const i8)
    };
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT upper(name),\n       sum(int_pages+leaf_pages+ovfl_pages)\n  FROM temp.%s\n WHERE name IS NOT NULL\n GROUP BY 1\n ORDER BY 2 DESC, 1;".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        let nn: Sqlite3Int64 = unsafe { sqlite3_column_int64(p_stmt, 1) };
        unsafe {
            diskused_line(&mut s,
                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, nn)
        };
        diskused_percent(&s, nn as f64 * 100.0 / n_page as f64);
    }
    if diskused_stmt_finish(&mut s, rc, p_stmt) != 0 { return; }
    rc =
        diskused_subreport(&mut s,
            c"All tables and indexes".as_ptr() as *mut i8,
            c"1".as_ptr() as *mut i8, pgsz, n_page);
    if rc != 0 { return; }
    rc =
        diskused_subreport(&mut s, c"All tables".as_ptr() as *mut i8,
            c"NOT is_index".as_ptr() as *mut i8, pgsz, n_page);
    if rc != 0 { return; }
    if n_wo_rowid > 0 as i64 {
        rc =
            diskused_subreport(&mut s,
                c"All WITHOUT ROWID tables".as_ptr() as *mut i8,
                c"is_without_rowid".as_ptr() as *mut i8, pgsz, n_page);
        if rc != 0 { return; }
        rc =
            diskused_subreport(&mut s,
                c"All rowid tables".as_ptr() as *mut i8,
                c"NOT is_without_rowid AND NOT is_index".as_ptr() as *mut i8,
                pgsz, n_page);
        if rc != 0 { return; }
    }
    rc =
        diskused_subreport(&mut s, c"All indexes".as_ptr() as *mut i8,
            c"is_index".as_ptr() as *mut i8, pgsz, n_page);
    if rc != 0 { return; }
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT upper(tblname), tblname, sum(is_index) FROM temp.%s GROUP BY 1 ORDER BY 1".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        let z_upper: *const i8 =
            unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
        let z_name: *const i8 =
            unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
        let n_sub_index: i32 = unsafe { sqlite3_column_int(p_stmt, 2) };
        if n_sub_index == 0 {
            let z_title: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"Table %s".as_ptr() as *mut i8 as
                            *const i8, z_upper)
                };
            let z_where: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"name=%Q".as_ptr() as *mut i8 as *const i8,
                        z_name)
                };
            rc = diskused_subreport(&mut s, z_title, z_where, pgsz, n_page);
            unsafe { sqlite3_free(z_title as *mut ()) };
            unsafe { sqlite3_free(z_where as *mut ()) };
            if rc != 0 { break; }
        } else {
            let mut p_s2: *mut Sqlite3Stmt = core::ptr::null_mut();
            let mut z_title_1: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"Table %s and all its indexes".as_ptr() as
                                *mut i8 as *const i8, z_upper)
                };
            let mut z_where_1: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"tblname=%Q".as_ptr() as *mut i8 as
                            *const i8, z_name)
                };
            rc =
                diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                    n_page);
            unsafe { sqlite3_free(z_title_1 as *mut ()) };
            unsafe { sqlite3_free(z_where_1 as *mut ()) };
            if rc != 0 { break; }
            z_title_1 =
                unsafe {
                    sqlite3_mprintf(c"Table %s w/o any indexes".as_ptr() as
                                *mut i8 as *const i8, z_upper)
                };
            z_where_1 =
                unsafe {
                    sqlite3_mprintf(c"name=%Q".as_ptr() as *mut i8 as *const i8,
                        z_name)
                };
            rc =
                diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                    n_page);
            unsafe { sqlite3_free(z_title_1 as *mut ()) };
            unsafe { sqlite3_free(z_where_1 as *mut ()) };
            if rc != 0 { break; }
            if n_sub_index > 1 {
                z_title_1 =
                    unsafe {
                        sqlite3_mprintf(c"All indexes of table %s".as_ptr() as
                                    *mut i8 as *const i8, z_upper)
                    };
                z_where_1 =
                    unsafe {
                        sqlite3_mprintf(c"tblname=%Q AND is_index".as_ptr() as
                                    *mut i8 as *const i8, z_name)
                    };
                rc =
                    diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                        n_page);
                unsafe { sqlite3_free(z_title_1 as *mut ()) };
                unsafe { sqlite3_free(z_where_1 as *mut ()) };
                if rc != 0 { break; }
            }
            p_s2 =
                unsafe {
                    diskused_prepare(&mut s,
                        c"SELECT name, upper(name) FROM temp.%s WHERE is_index AND tblname=%Q".as_ptr()
                                as *mut i8 as *const i8, s.z_su, z_name)
                };
            if p_s2 == core::ptr::null_mut() { rc = 7; break; }
            while { rc = unsafe { sqlite3_step(p_s2) }; rc } == 100 {
                let z_u: *const i8 =
                    unsafe { sqlite3_column_text(p_s2, 1) } as *const i8;
                let z_n: *const i8 =
                    unsafe { sqlite3_column_text(p_s2, 0) } as *const i8;
                z_title_1 =
                    unsafe {
                        sqlite3_mprintf(c"Index %s".as_ptr() as *mut i8 as
                                *const i8, z_u)
                    };
                z_where_1 =
                    unsafe {
                        sqlite3_mprintf(c"name=%Q".as_ptr() as *mut i8 as *const i8,
                            z_n)
                    };
                rc =
                    diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                        n_page);
                unsafe { sqlite3_free(z_title_1 as *mut ()) };
                unsafe { sqlite3_free(z_where_1 as *mut ()) };
                if rc != 0 { break; }
            }
            rc = diskused_stmt_finish(&mut s, rc, p_s2);
            if rc != 0 { break; }
        }
    }
    if diskused_stmt_finish(&mut s, rc, p_stmt) != 0 { return; }
    unsafe {
        diskused_title(&mut s,
            c"Raw data used to generate this report".as_ptr() as *mut i8 as
                *const i8)
    };
    unsafe {
        sqlite3_str_appendf(s.p_out,
            c"The following SQL will create a table named \"space_used\" which\ncontains most of the information used to generate the report above.\n*/\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe {
        sqlite3_str_appendf(s.p_out,
            c"BEGIN;\nCREATE TABLE space_used(\n   name text,                -- A table or index\n   tblname text,             -- Table that owns name\n   is_index boolean,         -- TRUE if it is an index\n   is_without_rowid boolean, -- TRUE if WITHOUT ROWID table\n   nentry int,               -- Number of entries in the BTree\n   leaf_entries int,         -- Number of leaf entries\n   depth int,                -- Depth of the b-tree\n   payload int,              -- Total data in this table/index\n   ovfl_payload int,         -- Total data on overflow pages\n   ovfl_cnt int,             -- Entries that use overflow\n   mx_payload int,           -- Maximum payload size\n   int_pages int,            -- Interior pages used\n   leaf_pages int,           -- Leaf pages used\n   ovfl_pages int,           -- Overflow pages used\n   int_unused int,           -- Unused bytes on interior pages\n   leaf_unused int,          -- Unused bytes on primary pages\n   ovfl_unused int,          -- Unused bytes on overflow pages\n   int_entries int           -- B-tree entries on internal pages\n);\nINSERT INTO space_used VALUES\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT quote(name), quote(tblname),\n       is_index, is_without_rowid, nentry, leaf_entries,\n       depth, payload, ovfl_payload, ovfl_cnt, mx_payload,\n       int_pages, leaf_pages, ovfl_pages, int_unused,\n       leaf_unused, ovfl_unused, int_entries\n  FROM temp.%s;".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    n = 0;
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        if { let __p = &mut n; let __t = *__p; *__p += 1; __t } != 0 {
            unsafe {
                sqlite3_str_appendf(s.p_out,
                    c",\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            sqlite3_str_appendf(s.p_out,
                c" (%s,%s,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld)".as_ptr()
                        as *mut i8 as *const i8,
                unsafe { sqlite3_column_text(p_stmt, 0) },
                unsafe { sqlite3_column_text(p_stmt, 1) },
                unsafe { sqlite3_column_int64(p_stmt, 2) },
                unsafe { sqlite3_column_int64(p_stmt, 3) },
                unsafe { sqlite3_column_int64(p_stmt, 4) },
                unsafe { sqlite3_column_int64(p_stmt, 5) },
                unsafe { sqlite3_column_int64(p_stmt, 6) },
                unsafe { sqlite3_column_int64(p_stmt, 7) },
                unsafe { sqlite3_column_int64(p_stmt, 8) },
                unsafe { sqlite3_column_int64(p_stmt, 9) },
                unsafe { sqlite3_column_int64(p_stmt, 10) },
                unsafe { sqlite3_column_int64(p_stmt, 11) },
                unsafe { sqlite3_column_int64(p_stmt, 12) },
                unsafe { sqlite3_column_int64(p_stmt, 13) },
                unsafe { sqlite3_column_int64(p_stmt, 14) },
                unsafe { sqlite3_column_int64(p_stmt, 15) },
                unsafe { sqlite3_column_int64(p_stmt, 16) },
                unsafe { sqlite3_column_int64(p_stmt, 17) })
        };
    }
    if rc != 101 {
        unsafe {
            diskused_error(&mut s,
                c"SQL run-time error: %s\nSQL: %s".as_ptr() as *mut i8 as
                    *const i8, unsafe { sqlite3_errmsg(s.db) },
                unsafe { sqlite3_sql(p_stmt) })
        };
        unsafe { sqlite3_finalize(p_stmt) };
        return;
    }
    unsafe {
        sqlite3_str_appendf(s.p_out,
            c";\nCOMMIT;".as_ptr() as *mut i8 as *const i8)
    };
    unsafe { sqlite3_finalize(p_stmt) };
    if unsafe { sqlite3_str_length(s.p_out) } != 0 {
        unsafe {
            sqlite3_result_text(context,
                unsafe { sqlite3_str_finish(s.p_out) } as *const i8, -1,
                Some(sqlite3_free))
        };
        s.p_out = core::ptr::null_mut();
    }
    diskused_reset(&mut s);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_diskused_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"diskused".as_ptr() as *mut i8 as *const i8, 1, 1 | 524288,
                core::ptr::null_mut(), Some(diskused_func), None, None)
        };
    return rc;
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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn ceil(_: f64)
    -> f64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
