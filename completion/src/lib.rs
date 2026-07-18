#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct CompletionVtab {
    base: Sqlite3Vtab,
    db: *mut Sqlite3,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CompletionCursor {
    base: Sqlite3VtabCursor,
    db: *mut Sqlite3,
    n_prefix: i32,
    n_line: i32,
    z_prefix: *mut i8,
    z_line: *mut i8,
    z_current_row: *const i8,
    sz_row: i32,
    p_stmt: *mut Sqlite3Stmt,
    i_rowid: Sqlite3Int64,
    e_phase: i32,
    j: i32,
}

extern "C" fn completion_connect(db: *mut Sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut CompletionVtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = p_aux_1; };
    { let _ = argc; };
    { let _ = argv; };
    { let _ = pz_err_1; };
    unsafe { sqlite3_vtab_config(db, 2) };
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(  candidate TEXT,  prefix TEXT HIDDEN,  wholeline TEXT HIDDEN,  phase INT HIDDEN)".as_ptr()
                        as *mut i8 as *const i8)
        };
    if rc == 0 {
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<CompletionVtab>() as
                            Sqlite3Uint64)
                } as *mut CompletionVtab;
        unsafe { *pp_vtab_1 = p_new as *mut Sqlite3Vtab };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<CompletionVtab>() as u64)
        };
        unsafe { (*p_new).db = db };
    }
    return rc;
}

extern "C" fn completion_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}

extern "C" fn completion_open(p: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_cur: *mut CompletionCursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<CompletionCursor>() as
                        Sqlite3Uint64)
            } as *mut CompletionCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<CompletionCursor>() as u64)
    };
    unsafe { (*p_cur).db = unsafe { (*(p as *mut CompletionVtab)).db } };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}

extern "C" fn completion_cursor_reset(p_cur_1: &mut CompletionCursor) -> () {
    unsafe { sqlite3_free((*p_cur_1).z_prefix as *mut ()) };
    (*p_cur_1).z_prefix = core::ptr::null_mut();
    (*p_cur_1).n_prefix = 0;
    unsafe { sqlite3_free((*p_cur_1).z_line as *mut ()) };
    (*p_cur_1).z_line = core::ptr::null_mut();
    (*p_cur_1).n_line = 0;
    unsafe { sqlite3_finalize((*p_cur_1).p_stmt) };
    (*p_cur_1).p_stmt = core::ptr::null_mut();
    (*p_cur_1).j = 0;
}

extern "C" fn completion_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    completion_cursor_reset(unsafe { &mut *(cur as *mut CompletionCursor) });
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0;
}

extern "C" fn completion_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut CompletionCursor = cur as *mut CompletionCursor;
    let mut e_next_phase: i32 = 0;
    let mut i_col: i32 = -1;
    let mut rc: i32 = 0;
    {
        let __p = unsafe { &mut (*p_cur).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    while unsafe { (*p_cur).e_phase } != 11 {
        '__s1:
            {
            match unsafe { (*p_cur).e_phase } {
                1 => {
                    {
                        if unsafe { (*p_cur).j } >=
                                unsafe { sqlite3_keyword_count() } {
                            unsafe { (*p_cur).z_current_row = core::ptr::null() };
                            unsafe { (*p_cur).e_phase = 7 };
                        } else {
                            unsafe {
                                sqlite3_keyword_name({
                                        let __p = unsafe { &mut (*p_cur).j };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    }, unsafe { &mut (*p_cur).z_current_row },
                                    unsafe { &mut (*p_cur).sz_row })
                            };
                        }
                        i_col = -1;
                        break '__s1;
                    }
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, unsafe { &mut (*p_cur).p_stmt }, core::ptr::null_mut())
                            };
                        }
                        i_col = 1;
                        e_next_phase = 8;
                        break '__s1;
                    }
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql: *mut i8 = core::ptr::null_mut();
                            let mut z_sep: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2) } == 100 {
                                let z_db: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str,
                                        c"%sSELECT name FROM \"%w\".sqlite_schema".as_ptr() as
                                                *mut i8 as *const i8, z_sep, z_db)
                                };
                                z_sep = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2) };
                            z_sql = unsafe { sqlite3_str_finish(p_str) };
                            if z_sql == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 9;
                        break '__s1;
                    }
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2_1: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str_1: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql_1: *mut i8 = core::ptr::null_mut();
                            let mut z_sep_1: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2_1, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2_1) } == 100 {
                                let z_db_1: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2_1, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str_1,
                                        c"%sSELECT pti.name FROM \"%w\".sqlite_schema AS sm JOIN pragma_table_xinfo(sm.name,%Q) AS pti WHERE sm.type=\'table\'".as_ptr()
                                                as *mut i8 as *const i8, z_sep_1, z_db_1, z_db_1)
                                };
                                z_sep_1 = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2_1) };
                            z_sql_1 = unsafe { sqlite3_str_finish(p_str_1) };
                            if z_sql_1 == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql_1 as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql_1 as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 11;
                        break '__s1;
                    }
                }
                7 => {
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, unsafe { &mut (*p_cur).p_stmt }, core::ptr::null_mut())
                            };
                        }
                        i_col = 1;
                        e_next_phase = 8;
                        break '__s1;
                    }
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql: *mut i8 = core::ptr::null_mut();
                            let mut z_sep: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2) } == 100 {
                                let z_db: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str,
                                        c"%sSELECT name FROM \"%w\".sqlite_schema".as_ptr() as
                                                *mut i8 as *const i8, z_sep, z_db)
                                };
                                z_sep = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2) };
                            z_sql = unsafe { sqlite3_str_finish(p_str) };
                            if z_sql == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 9;
                        break '__s1;
                    }
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2_1: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str_1: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql_1: *mut i8 = core::ptr::null_mut();
                            let mut z_sep_1: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2_1, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2_1) } == 100 {
                                let z_db_1: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2_1, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str_1,
                                        c"%sSELECT pti.name FROM \"%w\".sqlite_schema AS sm JOIN pragma_table_xinfo(sm.name,%Q) AS pti WHERE sm.type=\'table\'".as_ptr()
                                                as *mut i8 as *const i8, z_sep_1, z_db_1, z_db_1)
                                };
                                z_sep_1 = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2_1) };
                            z_sql_1 = unsafe { sqlite3_str_finish(p_str_1) };
                            if z_sql_1 == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql_1 as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql_1 as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 11;
                        break '__s1;
                    }
                }
                8 => {
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql: *mut i8 = core::ptr::null_mut();
                            let mut z_sep: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2) } == 100 {
                                let z_db: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str,
                                        c"%sSELECT name FROM \"%w\".sqlite_schema".as_ptr() as
                                                *mut i8 as *const i8, z_sep, z_db)
                                };
                                z_sep = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2) };
                            z_sql = unsafe { sqlite3_str_finish(p_str) };
                            if z_sql == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 9;
                        break '__s1;
                    }
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2_1: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str_1: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql_1: *mut i8 = core::ptr::null_mut();
                            let mut z_sep_1: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2_1, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2_1) } == 100 {
                                let z_db_1: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2_1, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str_1,
                                        c"%sSELECT pti.name FROM \"%w\".sqlite_schema AS sm JOIN pragma_table_xinfo(sm.name,%Q) AS pti WHERE sm.type=\'table\'".as_ptr()
                                                as *mut i8 as *const i8, z_sep_1, z_db_1, z_db_1)
                                };
                                z_sep_1 = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2_1) };
                            z_sql_1 = unsafe { sqlite3_str_finish(p_str_1) };
                            if z_sql_1 == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql_1 as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql_1 as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 11;
                        break '__s1;
                    }
                }
                9 => {
                    {
                        if unsafe { (*p_cur).p_stmt } == core::ptr::null_mut() {
                            let mut p_s2_1: *mut Sqlite3Stmt = core::ptr::null_mut();
                            let p_str_1: *mut Sqlite3Str =
                                unsafe { sqlite3_str_new(unsafe { (*p_cur).db }) };
                            let mut z_sql_1: *mut i8 = core::ptr::null_mut();
                            let mut z_sep_1: *const i8 =
                                c"".as_ptr() as *mut i8 as *const i8;
                            unsafe {
                                sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                                    -1, &mut p_s2_1, core::ptr::null_mut())
                            };
                            while unsafe { sqlite3_step(p_s2_1) } == 100 {
                                let z_db_1: *const i8 =
                                    unsafe { sqlite3_column_text(p_s2_1, 1) } as *const i8;
                                unsafe {
                                    sqlite3_str_appendf(p_str_1,
                                        c"%sSELECT pti.name FROM \"%w\".sqlite_schema AS sm JOIN pragma_table_xinfo(sm.name,%Q) AS pti WHERE sm.type=\'table\'".as_ptr()
                                                as *mut i8 as *const i8, z_sep_1, z_db_1, z_db_1)
                                };
                                z_sep_1 = c" UNION ".as_ptr() as *mut i8 as *const i8;
                            }
                            rc = unsafe { sqlite3_finalize(p_s2_1) };
                            z_sql_1 = unsafe { sqlite3_str_finish(p_str_1) };
                            if z_sql_1 == core::ptr::null_mut() { return 7; }
                            if rc == 0 {
                                unsafe {
                                    sqlite3_prepare_v2(unsafe { (*p_cur).db },
                                        z_sql_1 as *const i8, -1, unsafe { &mut (*p_cur).p_stmt },
                                        core::ptr::null_mut())
                                };
                            }
                            unsafe { sqlite3_free(z_sql_1 as *mut ()) };
                            if rc != 0 { return rc; }
                        }
                        i_col = 0;
                        e_next_phase = 11;
                        break '__s1;
                    }
                }
                _ => {}
            }
        }
        if i_col < 0 {
            if unsafe { (*p_cur).z_current_row } == core::ptr::null() {
                continue;
            }
        } else {
            if unsafe { sqlite3_step(unsafe { (*p_cur).p_stmt }) } == 100 {
                unsafe {
                    (*p_cur).z_current_row =
                        unsafe {
                                sqlite3_column_text(unsafe { (*p_cur).p_stmt }, i_col)
                            } as *const i8
                };
                unsafe {
                    (*p_cur).sz_row =
                        unsafe {
                            sqlite3_column_bytes(unsafe { (*p_cur).p_stmt }, i_col)
                        }
                };
            } else {
                rc = unsafe { sqlite3_finalize(unsafe { (*p_cur).p_stmt }) };
                unsafe { (*p_cur).p_stmt = core::ptr::null_mut() };
                unsafe { (*p_cur).e_phase = e_next_phase };
                if rc != 0 { return rc; }
                continue;
            }
        }
        if unsafe { (*p_cur).n_prefix } == 0 { break; }
        if unsafe { (*p_cur).n_prefix } <= unsafe { (*p_cur).sz_row } &&
                unsafe {
                        sqlite3_strnicmp(unsafe { (*p_cur).z_prefix } as *const i8,
                            unsafe { (*p_cur).z_current_row },
                            unsafe { (*p_cur).n_prefix })
                    } == 0 {
            break;
        }
    }
    return 0;
}

extern "C" fn completion_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const CompletionCursor =
        cur as *mut CompletionCursor as *const CompletionCursor;
    '__s4:
        {
        match i {
            0 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx, unsafe { (*p_cur).z_current_row },
                            unsafe { (*p_cur).sz_row },
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_cur).z_prefix } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_cur).z_line } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).e_phase })
                    };
                    break '__s4;
                }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_cur).z_prefix } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_cur).z_line } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).e_phase })
                    };
                    break '__s4;
                }
            }
            2 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_cur).z_line } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).e_phase })
                    };
                    break '__s4;
                }
            }
            3 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).e_phase })
                    };
                    break '__s4;
                }
            }
            _ => {}
        }
    }
    return 0;
}

extern "C" fn completion_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const CompletionCursor =
        cur as *mut CompletionCursor as *const CompletionCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}

extern "C" fn completion_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const CompletionCursor =
        cur as *mut CompletionCursor as *const CompletionCursor;
    return (unsafe { (*p_cur).e_phase } >= 11) as i32;
}

extern "C" fn completion_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut CompletionCursor =
        p_vtab_cursor_1 as *mut CompletionCursor;
    let mut i_arg: i32 = 0;
    { let _ = idx_str_1; };
    { let _ = argc; };
    completion_cursor_reset(unsafe { &mut *p_cur });
    if idx_num_1 & 1 != 0 {
        unsafe {
            (*p_cur).n_prefix =
                unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(i_arg as isize) })
                }
        };
        if unsafe { (*p_cur).n_prefix } > 0 {
            unsafe {
                (*p_cur).z_prefix =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i_arg as isize) })
                            })
                    }
            };
            if unsafe { (*p_cur).z_prefix } == core::ptr::null_mut() {
                return 7;
            }
            unsafe {
                (*p_cur).n_prefix =
                    unsafe { strlen(unsafe { (*p_cur).z_prefix } as *const i8) }
                        as i32
            };
        }
        i_arg = 1;
    }
    if idx_num_1 & 2 != 0 {
        unsafe {
            (*p_cur).n_line =
                unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(i_arg as isize) })
                }
        };
        if unsafe { (*p_cur).n_line } > 0 {
            unsafe {
                (*p_cur).z_line =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i_arg as isize) })
                            })
                    }
            };
            if unsafe { (*p_cur).z_line } == core::ptr::null_mut() {
                return 7;
            }
            unsafe {
                (*p_cur).n_line =
                    unsafe { strlen(unsafe { (*p_cur).z_line } as *const i8) }
                        as i32
            };
        }
    }
    if unsafe { (*p_cur).z_line } != core::ptr::null_mut() &&
            unsafe { (*p_cur).z_prefix } == core::ptr::null_mut() {
        let mut i: i32 = unsafe { (*p_cur).n_line };
        while i > 0 &&
                (unsafe {
                            isalnum(unsafe {
                                            *unsafe { (*p_cur).z_line.offset((i - 1) as isize) }
                                        } as u8 as i32)
                        } != 0 ||
                    unsafe {
                                *unsafe { (*p_cur).z_line.offset((i - 1) as isize) }
                            } as i32 == '_' as i32) {
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
        unsafe { (*p_cur).n_prefix = unsafe { (*p_cur).n_line } - i };
        if unsafe { (*p_cur).n_prefix } > 0 {
            unsafe {
                (*p_cur).z_prefix =
                    unsafe {
                        sqlite3_mprintf(c"%.*s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_cur).n_prefix },
                            unsafe { unsafe { (*p_cur).z_line.offset(i as isize) } })
                    }
            };
            if unsafe { (*p_cur).z_prefix } == core::ptr::null_mut() {
                return 7;
            }
            unsafe {
                (*p_cur).n_prefix =
                    unsafe { strlen(unsafe { (*p_cur).z_prefix } as *const i8) }
                        as i32
            };
        }
    }
    unsafe { (*p_cur).i_rowid = 0 as Sqlite3Int64 };
    unsafe { (*p_cur).e_phase = 1 };
    return completion_next(p_vtab_cursor_1);
}

extern "C" fn completion_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i: i32 = 0;
    let mut idx_num: i32 = 0;
    let mut prefix_idx: i32 = -1;
    let mut wholeline_idx: i32 = -1;
    let mut n_arg: i32 = 0;
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();
    { let _ = tab; };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b6: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) { break '__b6; }
            '__c6: loop {
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c6;
                }
                if unsafe { (*p_constraint).op } as i32 != 2 { break '__c6; }
                '__s7:
                    {
                    match unsafe { (*p_constraint).i_column } {
                        1 => { prefix_idx = i; idx_num |= 1; }
                        2 => { wholeline_idx = i; idx_num |= 2; }
                        _ => {}
                    }
                }
                break '__c6;
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
    if prefix_idx >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(prefix_idx as
                                    isize)
                        }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(prefix_idx as
                                    isize)
                        }).omit = 1 as u8
        };
    }
    if wholeline_idx >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(wholeline_idx as
                                    isize)
                        }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(wholeline_idx as
                                    isize)
                        }).omit = 1 as u8
        };
    }
    unsafe { (*p_idx_info_1).idx_num = idx_num };
    unsafe {
        (*p_idx_info_1).estimated_cost = 5000 as f64 - (1000 * n_arg) as f64
    };
    unsafe {
        (*p_idx_info_1).estimated_rows = (500 - 100 * n_arg) as Sqlite3Int64
    };
    return 0;
}

static mut completion_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: None,
        x_connect: Some(completion_connect),
        x_best_index: Some(completion_best_index),
        x_disconnect: Some(completion_disconnect),
        x_destroy: None,
        x_open: Some(completion_open),
        x_close: Some(completion_close),
        x_filter: Some(completion_filter),
        x_next: Some(completion_next),
        x_eof: Some(completion_eof),
        x_column: Some(completion_column),
        x_rowid: Some(completion_rowid),
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
pub extern "C" fn sqlite3_completion_vtab_init(db: *mut Sqlite3) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"completion".as_ptr() as *mut i8 as *const i8,
                    &raw mut completion_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
        return rc;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_completion_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    rc = sqlite3_completion_vtab_init(db);
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
    fn isalnum(_c: i32)
    -> i32;
}
