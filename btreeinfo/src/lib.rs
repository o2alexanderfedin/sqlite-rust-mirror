#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct BinfoTable {
    base: Sqlite3Vtab,
    db: *mut Sqlite3,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct BinfoCursor {
    base: Sqlite3VtabCursor,
    p_stmt: *mut Sqlite3Stmt,
    rc: i32,
    has_rowid: i32,
    n_entry: Sqlite3Int64,
    n_page: i32,
    depth: i32,
    sz_page: i32,
    z_schema: *mut i8,
}

extern "C" fn binfo_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_tab: *mut BinfoTable = core::ptr::null_mut();
    let mut rc: i32 = 0;
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(\n type TEXT,\n name TEXT,\n tbl_name TEXT,\n rootpage INT,\n sql TEXT,\n hasRowid BOOLEAN,\n nEntry INT,\n nPage INT,\n depth INT,\n szPage INT,\n zSchema TEXT HIDDEN\n)".as_ptr()
                        as *mut i8 as *const i8)
        };
    if rc == 0 {
        p_tab =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<BinfoTable>() as
                            Sqlite3Uint64)
                } as *mut BinfoTable;
        if p_tab == core::ptr::null_mut() { rc = 7; }
    }
    if !(rc == 0 || p_tab == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"binfoConnect".as_ptr() as *const i8,
                c"btreeinfo.c".as_ptr() as *mut i8 as *const i8, 144,
                c"rc==SQLITE_OK || pTab==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(p_tab).is_null() { unsafe { (*p_tab).db = db }; }
    unsafe { *pp_vtab_1 = p_tab as *mut Sqlite3Vtab };
    return rc;
}

extern "C" fn binfo_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}

extern "C" fn binfo_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i: i32 = 0;
    unsafe { (*p_idx_info_1).estimated_cost = 10000.0 };
    unsafe { (*p_idx_info_1).estimated_rows = 100 as Sqlite3Int64 };
    {
        i = 0;
        '__b0: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) { break '__b0; }
            '__c0: loop {
                let p: *const Sqlite3IndexConstraint =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_idx_info_1).a_constraint.offset(i as isize)
                                    }
                        } as *const Sqlite3IndexConstraint;
                if unsafe { (*p).usable } != 0 &&
                            unsafe { (*p).i_column } == 10 &&
                        unsafe { (*p).op } as i32 == 2 {
                    unsafe { (*p_idx_info_1).estimated_cost = 1000.0 };
                    unsafe { (*p_idx_info_1).idx_num = 1 };
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
                    break '__b0;
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

extern "C" fn binfo_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_csr: *mut BinfoCursor = core::ptr::null_mut();
    p_csr =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<BinfoCursor>() as
                        Sqlite3Uint64)
            } as *mut BinfoCursor;
    if p_csr == core::ptr::null_mut() {
        return 7;
    } else {
        unsafe {
            memset(p_csr as *mut (), 0,
                core::mem::size_of::<BinfoCursor>() as u64)
        };
        unsafe { (*p_csr).base.p_vtab = p_v_tab_1 };
    }
    unsafe { *pp_cursor_1 = p_csr as *mut Sqlite3VtabCursor };
    return 0;
}

extern "C" fn binfo_close(p_cursor_1: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *mut BinfoCursor = p_cursor_1 as *mut BinfoCursor;
    unsafe { sqlite3_finalize(unsafe { (*p_csr).p_stmt }) };
    unsafe { sqlite3_free(unsafe { (*p_csr).z_schema } as *mut ()) };
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}

extern "C" fn binfo_next(p_cursor_1: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *mut BinfoCursor = p_cursor_1 as *mut BinfoCursor;
    unsafe {
        (*p_csr).rc = unsafe { sqlite3_step(unsafe { (*p_csr).p_stmt }) }
    };
    unsafe { (*p_csr).has_rowid = -1 };
    return if unsafe { (*p_csr).rc } == 1 { 1 } else { 0 };
}

extern "C" fn binfo_eof(p_cursor_1: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *const BinfoCursor =
        p_cursor_1 as *mut BinfoCursor as *const BinfoCursor;
    return (unsafe { (*p_csr).rc } != 100) as i32;
}

extern "C" fn binfo_filter(p_cursor_1: *mut Sqlite3VtabCursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut Sqlite3Value) -> i32 {
    let p_csr: *mut BinfoCursor = p_cursor_1 as *mut BinfoCursor;
    let p_tab: *const BinfoTable =
        unsafe { (*p_cursor_1).p_vtab } as *mut BinfoTable as
            *const BinfoTable;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    unsafe { sqlite3_free(unsafe { (*p_csr).z_schema } as *mut ()) };
    if idx_num_1 == 1 &&
            unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) }
                != 5 {
        unsafe {
            (*p_csr).z_schema =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        })
                }
        };
    } else {
        unsafe {
            (*p_csr).z_schema =
                unsafe {
                    sqlite3_mprintf(c"main".as_ptr() as *mut i8 as *const i8)
                }
        };
    }
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT 0, \'table\',\'sqlite_schema\',\'sqlite_schema\',1,NULL UNION ALL SELECT rowid, type, name, tbl_name, rootpage, sql FROM \"%w\".sqlite_schema WHERE rootpage>=1".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p_csr).z_schema })
        };
    unsafe { sqlite3_finalize(unsafe { (*p_csr).p_stmt }) };
    unsafe { (*p_csr).p_stmt = core::ptr::null_mut() };
    unsafe { (*p_csr).has_rowid = -1 };
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p_tab).db }, z_sql as *const i8, -1,
                unsafe { &mut (*p_csr).p_stmt }, core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc == 0 { rc = binfo_next(p_cursor_1); }
    return rc;
}

extern "C" fn get_uint16(a: *const u8) -> u32 {
    return ((unsafe { *a.offset(0 as isize) } as i32) << 8 |
                unsafe { *a.offset(1 as isize) } as i32) as u32;
}

extern "C" fn get_uint32(a: *const u8) -> u32 {
    return ((unsafe { *a.offset(0 as isize) } as i32) << 24 |
                        (unsafe { *a.offset(1 as isize) } as i32) << 16 |
                    (unsafe { *a.offset(2 as isize) } as i32) << 8 |
                unsafe { *a.offset(3 as isize) } as i32) as u32;
}

extern "C" fn binfo_compute(db: *mut Sqlite3, mut pgno: i32,
    p_csr_1: &mut BinfoCursor) -> i32 {
    let mut n_entry: Sqlite3Int64 = 1 as Sqlite3Int64;
    let mut n_page: i32 = 1;
    let mut a_data: *mut u8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut pgsz: i32 = 0;
    let mut n_cell: i32 = 0;
    let mut i_cell: i32 = 0;
    rc =
        unsafe {
            sqlite3_prepare_v2(db,
                c"SELECT data FROM sqlite_dbpage(\'main\') WHERE pgno=?1".as_ptr()
                        as *mut i8 as *const i8, -1, &mut p_stmt,
                core::ptr::null_mut())
        };
    if rc != 0 { return rc; }
    (*p_csr_1).depth = 1;
    loop {
        unsafe { sqlite3_bind_int(p_stmt, 1, pgno) };
        if (*p_csr_1).depth > 25 {
            unsafe {
                sqlite3_set_errmsg(db, 11,
                    c"btree nested too deep".as_ptr() as *mut i8 as *const i8)
            };
            rc = 1;
            break;
        }
        rc = unsafe { sqlite3_step(p_stmt) };
        if rc != 100 { rc = 1; break; }
        (*p_csr_1).sz_page =
            { pgsz = unsafe { sqlite3_column_bytes(p_stmt, 0) }; pgsz };
        a_data = unsafe { sqlite3_column_blob(p_stmt, 0) } as *mut u8;
        if a_data == core::ptr::null_mut() { rc = 7; break; }
        if pgno == 1 {
            {
                let __n = 100;
                let __p = &mut a_data;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            pgsz -= 100;
        }
        (*p_csr_1).has_rowid =
            (unsafe { *a_data.offset(0 as isize) } as i32 != 2 &&
                    unsafe { *a_data.offset(0 as isize) } as i32 != 10) as i32;
        n_cell =
            get_uint16(unsafe { a_data.offset(3 as isize) } as *const u8) as
                i32;
        n_entry *= (n_cell + 1) as Sqlite3Int64;
        if unsafe { *a_data.offset(0 as isize) } as i32 == 10 ||
                unsafe { *a_data.offset(0 as isize) } as i32 == 13 {
            break;
        }
        n_page *= n_cell + 1;
        if 14 + 2 * (n_cell / 2) >= pgsz { rc = 11; break; }
        if n_cell <= 1 {
            pgno =
                get_uint32(unsafe { a_data.offset(8 as isize) } as *const u8)
                    as i32;
        } else {
            i_cell =
                get_uint16(unsafe {
                                unsafe {
                                    a_data.offset(12 as
                                                isize).offset((2 * (n_cell / 2)) as isize)
                                }
                            } as *const u8) as i32;
            if pgno == 1 { i_cell -= 100; }
            if i_cell <= 12 || i_cell >= pgsz - 4 { rc = 11; break; }
            pgno =
                get_uint32(unsafe { a_data.offset(i_cell as isize) } as
                            *const u8) as i32;
        }
        { let __p = &mut (*p_csr_1).depth; let __t = *__p; *__p += 1; __t };
        unsafe { sqlite3_reset(p_stmt) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    (*p_csr_1).n_page = n_page;
    (*p_csr_1).n_entry = n_entry;
    if rc == 100 { rc = 0; }
    return rc;
}

extern "C" fn binfo_column(p_cursor_1: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_csr: *mut BinfoCursor = p_cursor_1 as *mut BinfoCursor;
    if i >= 5 && i <= 9 && unsafe { (*p_csr).has_rowid } < 0 {
        let pgno: i32 =
            unsafe { sqlite3_column_int(unsafe { (*p_csr).p_stmt }, 3 + 1) };
        let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(ctx) };
        let rc: i32 = binfo_compute(db, pgno, unsafe { &mut *p_csr });
        if rc != 0 {
            unsafe {
                (*unsafe { (*p_cursor_1).p_vtab }).z_err_msg =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errstr(rc) })
                    }
            };
            return 1;
        }
    }
    '__s2:
        {
        match i {
            1 => {
                {
                    unsafe {
                        sqlite3_result_value(ctx,
                            unsafe {
                                sqlite3_column_value(unsafe { (*p_csr).p_stmt }, i + 1)
                            })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).has_rowid })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            0 => {
                {
                    unsafe {
                        sqlite3_result_value(ctx,
                            unsafe {
                                sqlite3_column_value(unsafe { (*p_csr).p_stmt }, i + 1)
                            })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).has_rowid })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            2 => {
                {
                    unsafe {
                        sqlite3_result_value(ctx,
                            unsafe {
                                sqlite3_column_value(unsafe { (*p_csr).p_stmt }, i + 1)
                            })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).has_rowid })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            3 => {
                {
                    unsafe {
                        sqlite3_result_value(ctx,
                            unsafe {
                                sqlite3_column_value(unsafe { (*p_csr).p_stmt }, i + 1)
                            })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).has_rowid })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            4 => {
                {
                    unsafe {
                        sqlite3_result_value(ctx,
                            unsafe {
                                sqlite3_column_value(unsafe { (*p_csr).p_stmt }, i + 1)
                            })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).has_rowid })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            5 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).has_rowid })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            6 => {
                {
                    unsafe {
                        sqlite3_result_int64(ctx, unsafe { (*p_csr).n_entry })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            7 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).n_page })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            8 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).depth })
                    };
                    break '__s2;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            10 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe { (*p_csr).z_schema } as *const i8, -1, None)
                    };
                    break '__s2;
                }
            }
            _ => {}
        }
    }
    return 0;
}

extern "C" fn binfo_rowid(p_cursor_1: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_csr: *const BinfoCursor =
        p_cursor_1 as *mut BinfoCursor as *const BinfoCursor;
    unsafe {
        *p_rowid_1 =
            unsafe { sqlite3_column_int64(unsafe { (*p_csr).p_stmt }, 0) }
    };
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_binfo_register(db: *mut Sqlite3) -> i32 {
    unsafe {
        return unsafe {
                sqlite3_create_module(db,
                    c"sqlite_btreeinfo".as_ptr() as *mut i8 as *const i8,
                    &raw mut binfo_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btreeinfo_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    { let _ = p_api_1; };
    return sqlite3_binfo_register(db);
}

static mut binfo_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: None,
        x_connect: Some(binfo_connect),
        x_best_index: Some(binfo_best_index),
        x_disconnect: Some(binfo_disconnect),
        x_destroy: None,
        x_open: Some(binfo_open),
        x_close: Some(binfo_close),
        x_filter: Some(binfo_filter),
        x_next: Some(binfo_next),
        x_eof: Some(binfo_eof),
        x_column: Some(binfo_column),
        x_rowid: Some(binfo_rowid),
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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
