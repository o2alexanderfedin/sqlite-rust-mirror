#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod fts5_h;
pub(crate) use crate::fts5_h::*;
mod fts5_int_h;
pub(crate) use crate::fts5_int_h::*;
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Storage {
    p_config: *mut Fts5Config,
    p_index: *mut Fts5Index,
    b_totals_valid: i32,
    n_total_row: i64,
    a_total_size: *mut i64,
    p_saved_row: *mut Sqlite3Stmt,
    a_stmt: [*mut Sqlite3Stmt; 12],
}

unsafe extern "C" fn fts5_exec_printf(db: *mut Sqlite3,
    pz_err_1: *mut *mut i8, z_format_1: *const i8, mut __va0: ...) -> i32 {
    let mut rc: i32 = 0;
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    if z_sql == core::ptr::null_mut() {
        rc = 7;
    } else {
        rc =
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), pz_err_1)
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    ();
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_create_table(p_config: &Fts5Config,
    z_post: *const i8, z_defn: *const i8, b_without: i32,
    pz_err: &mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut z_err: *mut i8 = core::ptr::null_mut();
    rc =
        unsafe {
            fts5_exec_printf((*p_config).db, &mut z_err,
                c"CREATE TABLE %Q.\'%q_%q\'(%s)%s".as_ptr() as *mut i8 as
                    *const i8, (*p_config).z_db, (*p_config).z_name, z_post,
                z_defn,
                if b_without != 0 {
                    c" WITHOUT ROWID".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 })
        };
    if !(z_err).is_null() {
        *pz_err =
            unsafe {
                sqlite3_mprintf(c"fts5: error creating shadow table %q_%s: %s".as_ptr()
                            as *mut i8 as *const i8, (*p_config).z_name, z_post, z_err)
            };
        unsafe { sqlite3_free(z_err as *mut ()) };
    }
    return rc;
}

extern "C" fn fts5_storage_get_stmt(p: &mut Fts5Storage, e_stmt_1: i32,
    pp_stmt_1: &mut *mut Sqlite3Stmt, pz_err_msg_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    if !(unsafe { (*(*p).p_config).b_columnsize } != 0 ||
                            e_stmt_1 != 7 && e_stmt_1 != 8 && e_stmt_1 != 9) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageGetStmt".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 96,
                c"p->pConfig->bColumnsize || ( eStmt!=FTS5_STMT_REPLACE_DOCSIZE && eStmt!=FTS5_STMT_DELETE_DOCSIZE && eStmt!=FTS5_STMT_LOOKUP_DOCSIZE )".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(e_stmt_1 >= 0 &&
                            e_stmt_1 <
                                (core::mem::size_of::<[*mut Sqlite3Stmt; 12]>() as u64 /
                                        core::mem::size_of::<*mut Sqlite3Stmt>() as u64) as i32) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageGetStmt".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 98,
                c"eStmt>=0 && eStmt<ArraySize(p->aStmt)".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    if (*p).a_stmt[e_stmt_1 as usize] == core::ptr::null_mut() {
        let az_stmt: [*const i8; 12] =
            [c"SELECT %s FROM %s T WHERE T.%Q >= ? AND T.%Q <= ? ORDER BY T.%Q ASC".as_ptr()
                        as *const i8,
                    c"SELECT %s FROM %s T WHERE T.%Q <= ? AND T.%Q >= ? ORDER BY T.%Q DESC".as_ptr()
                        as *const i8,
                    c"SELECT %s FROM %s T WHERE T.%Q=?".as_ptr() as *const i8,
                    c"SELECT %s FROM %s T WHERE T.%Q=?".as_ptr() as *const i8,
                    c"INSERT INTO %Q.\'%q_content\' VALUES(%s)".as_ptr() as
                        *const i8,
                    c"REPLACE INTO %Q.\'%q_content\' VALUES(%s)".as_ptr() as
                        *const i8,
                    c"DELETE FROM %Q.\'%q_content\' WHERE id=?".as_ptr() as
                        *const i8,
                    c"REPLACE INTO %Q.\'%q_docsize\' VALUES(?,?%s)".as_ptr() as
                        *const i8,
                    c"DELETE FROM %Q.\'%q_docsize\' WHERE id=?".as_ptr() as
                        *const i8,
                    c"SELECT sz%s FROM %Q.\'%q_docsize\' WHERE id=?".as_ptr() as
                        *const i8,
                    c"REPLACE INTO %Q.\'%q_config\' VALUES(?,?)".as_ptr() as
                        *const i8, c"SELECT %s FROM %s AS T".as_ptr() as *const i8];
        let p_c: *const Fts5Config = (*p).p_config as *const Fts5Config;
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        if !((core::mem::size_of::<[*const i8; 12]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64) as i32 ==
                                (core::mem::size_of::<[*mut Sqlite3Stmt; 12]>() as u64 /
                                        core::mem::size_of::<*mut Sqlite3Stmt>() as u64) as i32) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5StorageGetStmt".as_ptr() as *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 120,
                    c"ArraySize(azStmt)==ArraySize(p->aStmt)".as_ptr() as
                            *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        '__s0:
            {
            match e_stmt_1 {
                11 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_content_exprlist },
                                unsafe { (*p_c).z_content })
                        };
                }
                0 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_content_exprlist },
                                unsafe { (*p_c).z_content },
                                unsafe { (*p_c).z_content_rowid },
                                unsafe { (*p_c).z_content_rowid },
                                unsafe { (*p_c).z_content_rowid })
                        };
                }
                1 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_content_exprlist },
                                unsafe { (*p_c).z_content },
                                unsafe { (*p_c).z_content_rowid },
                                unsafe { (*p_c).z_content_rowid },
                                unsafe { (*p_c).z_content_rowid })
                        };
                }
                2 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_content_exprlist },
                                unsafe { (*p_c).z_content },
                                unsafe { (*p_c).z_content_rowid })
                        };
                }
                3 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_content_exprlist },
                                unsafe { (*p_c).z_content },
                                unsafe { (*p_c).z_content_rowid })
                        };
                }
                4 => {
                    {
                        let mut z_bind: *mut i8 = core::ptr::null_mut();
                        let mut i: i32 = 0;
                        if !(unsafe { (*p_c).e_content } == 0 ||
                                                unsafe { (*p_c).e_content } == 3) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fts5StorageGetStmt".as_ptr() as *const i8,
                                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 151,
                                    c"pC->eContent==FTS5_CONTENT_NORMAL || pC->eContent==FTS5_CONTENT_UNINDEXED".as_ptr()
                                            as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        {
                            i = 0;
                            '__b1: loop {
                                if !(rc == 0 && i < unsafe { (*p_c).n_col } + 1) {
                                    break '__b1;
                                }
                                '__c1: loop {
                                    if (i == 0) as i32 != 0 || unsafe { (*p_c).e_content } == 0
                                            ||
                                            unsafe {
                                                    *unsafe { (*p_c).ab_unindexed.offset((i - 1) as isize) }
                                                } != 0 {
                                        z_bind =
                                            unsafe {
                                                sqlite3_fts5_mprintf(&mut rc,
                                                    c"%z%s?%d".as_ptr() as *mut i8 as *const i8, z_bind,
                                                    if !(z_bind).is_null() {
                                                        c",".as_ptr() as *mut i8
                                                    } else { c"".as_ptr() as *mut i8 }, i + 1)
                                            };
                                    }
                                    break '__c1;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { (*p_c).b_locale } != 0 &&
                                unsafe { (*p_c).e_content } == 0 {
                            {
                                i = 0;
                                '__b2: loop {
                                    if !(rc == 0 && i < unsafe { (*p_c).n_col }) {
                                        break '__b2;
                                    }
                                    '__c2: loop {
                                        if unsafe {
                                                        *unsafe { (*p_c).ab_unindexed.offset(i as isize) }
                                                    } as i32 == 0 {
                                            z_bind =
                                                unsafe {
                                                    sqlite3_fts5_mprintf(&mut rc,
                                                        c"%z,?%d".as_ptr() as *mut i8 as *const i8, z_bind,
                                                        unsafe { (*p_c).n_col } + i + 2)
                                                };
                                        }
                                        break '__c2;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        z_sql =
                            unsafe {
                                sqlite3_fts5_mprintf(&mut rc, az_stmt[e_stmt_1 as usize],
                                    unsafe { (*p_c).z_db }, unsafe { (*p_c).z_name }, z_bind)
                            };
                        unsafe { sqlite3_free(z_bind as *mut ()) };
                        break '__s0;
                    }
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_db }, unsafe { (*p_c).z_name },
                                if unsafe { (*p_c).b_contentless_delete } != 0 {
                                    c",?".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 })
                        };
                }
                5 => {
                    {
                        let mut z_bind: *mut i8 = core::ptr::null_mut();
                        let mut i: i32 = 0;
                        if !(unsafe { (*p_c).e_content } == 0 ||
                                                unsafe { (*p_c).e_content } == 3) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fts5StorageGetStmt".as_ptr() as *const i8,
                                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 151,
                                    c"pC->eContent==FTS5_CONTENT_NORMAL || pC->eContent==FTS5_CONTENT_UNINDEXED".as_ptr()
                                            as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        {
                            i = 0;
                            '__b1: loop {
                                if !(rc == 0 && i < unsafe { (*p_c).n_col } + 1) {
                                    break '__b1;
                                }
                                '__c1: loop {
                                    if (i == 0) as i32 != 0 || unsafe { (*p_c).e_content } == 0
                                            ||
                                            unsafe {
                                                    *unsafe { (*p_c).ab_unindexed.offset((i - 1) as isize) }
                                                } != 0 {
                                        z_bind =
                                            unsafe {
                                                sqlite3_fts5_mprintf(&mut rc,
                                                    c"%z%s?%d".as_ptr() as *mut i8 as *const i8, z_bind,
                                                    if !(z_bind).is_null() {
                                                        c",".as_ptr() as *mut i8
                                                    } else { c"".as_ptr() as *mut i8 }, i + 1)
                                            };
                                    }
                                    break '__c1;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { (*p_c).b_locale } != 0 &&
                                unsafe { (*p_c).e_content } == 0 {
                            {
                                i = 0;
                                '__b2: loop {
                                    if !(rc == 0 && i < unsafe { (*p_c).n_col }) {
                                        break '__b2;
                                    }
                                    '__c2: loop {
                                        if unsafe {
                                                        *unsafe { (*p_c).ab_unindexed.offset(i as isize) }
                                                    } as i32 == 0 {
                                            z_bind =
                                                unsafe {
                                                    sqlite3_fts5_mprintf(&mut rc,
                                                        c"%z,?%d".as_ptr() as *mut i8 as *const i8, z_bind,
                                                        unsafe { (*p_c).n_col } + i + 2)
                                                };
                                        }
                                        break '__c2;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        z_sql =
                            unsafe {
                                sqlite3_fts5_mprintf(&mut rc, az_stmt[e_stmt_1 as usize],
                                    unsafe { (*p_c).z_db }, unsafe { (*p_c).z_name }, z_bind)
                            };
                        unsafe { sqlite3_free(z_bind as *mut ()) };
                        break '__s0;
                    }
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_db }, unsafe { (*p_c).z_name },
                                if unsafe { (*p_c).b_contentless_delete } != 0 {
                                    c",?".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 })
                        };
                }
                7 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_db }, unsafe { (*p_c).z_name },
                                if unsafe { (*p_c).b_contentless_delete } != 0 {
                                    c",?".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 })
                        };
                }
                9 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                if unsafe { (*p_c).b_contentless_delete } != 0 {
                                    c",origin".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 }, unsafe { (*p_c).z_db },
                                unsafe { (*p_c).z_name })
                        };
                }
                _ => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(az_stmt[e_stmt_1 as usize],
                                unsafe { (*p_c).z_db }, unsafe { (*p_c).z_name })
                        };
                }
            }
        }
        if z_sql == core::ptr::null_mut() {
            rc = 7;
        } else {
            let mut f: i32 = 1;
            if e_stmt_1 > 3 { f |= 4; }
            {
                let __p = unsafe { &mut (*(*p).p_config).b_lock };
                let __t = *__p;
                *__p += 1;
                __t
            };
            rc =
                unsafe {
                    sqlite3_prepare_v3(unsafe { (*p_c).db }, z_sql as *const i8,
                        -1, f as u32, &mut (*p).a_stmt[e_stmt_1 as usize],
                        core::ptr::null_mut())
                };
            {
                let __p = unsafe { &mut (*(*p).p_config).b_lock };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
            if rc != 0 && !(pz_err_msg_1).is_null() {
                unsafe {
                    *pz_err_msg_1 =
                        unsafe {
                            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_errmsg(unsafe { (*p_c).db }) })
                        }
                };
            }
            if rc == 1 && e_stmt_1 > 3 && e_stmt_1 < 11 { rc = 11; }
        }
    }
    *pp_stmt_1 = (*p).a_stmt[e_stmt_1 as usize];
    unsafe { sqlite3_reset(*pp_stmt_1) };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_config_value(p: *mut Fts5Storage,
    z: *const i8, p_val: *mut Sqlite3Value, i_val: i32) -> i32 {
    let mut p_replace: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 =
        fts5_storage_get_stmt(unsafe { &mut *p }, 10, &mut p_replace,
            core::ptr::null_mut());
    if rc == 0 {
        unsafe { sqlite3_bind_text(p_replace, 1, z, -1, None) };
        if !(p_val).is_null() {
            unsafe {
                sqlite3_bind_value(p_replace, 2, p_val as *const Sqlite3Value)
            };
        } else { unsafe { sqlite3_bind_int(p_replace, 2, i_val) }; }
        unsafe { sqlite3_step(p_replace) };
        rc = unsafe { sqlite3_reset(p_replace) };
        unsafe { sqlite3_bind_null(p_replace, 1) };
    }
    if rc == 0 && !(p_val).is_null() {
        let i_new: i32 = unsafe { (*unsafe { (*p).p_config }).i_cookie } + 1;
        rc =
            unsafe {
                sqlite3_fts5_index_set_cookie(unsafe { (*p).p_index }, i_new)
            };
        if rc == 0 {
            unsafe { (*unsafe { (*p).p_config }).i_cookie = i_new };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_close(p: *mut Fts5Storage) -> i32 {
    let rc: i32 = 0;
    if !(p).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b3: loop {
                if !(i <
                                (core::mem::size_of::<[*mut Sqlite3Stmt; 12]>() as u64 /
                                        core::mem::size_of::<*mut Sqlite3Stmt>() as u64) as i32) {
                    break '__b3;
                }
                '__c3: loop {
                    unsafe {
                        sqlite3_finalize(unsafe { (*p).a_stmt[i as usize] })
                    };
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(p as *mut ()) };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_open(p_config: *mut Fts5Config,
    p_index: *mut Fts5Index, b_create: i32, pp: &mut *mut Fts5Storage,
    pz_err: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut p: *mut Fts5Storage = core::ptr::null_mut();
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    n_byte =
        (core::mem::size_of::<Fts5Storage>() as u64 +
                unsafe { (*p_config).n_col } as u64 *
                    core::mem::size_of::<i64>() as u64) as Sqlite3Int64;
    *pp =
        {
            p =
                unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                    *mut Fts5Storage;
            p
        };
    if (p).is_null() as i32 != 0 { return 7; }
    unsafe { memset(p as *mut (), 0, n_byte as u64) };
    unsafe {
        (*p).a_total_size =
            unsafe { &raw mut *p.offset(1 as isize) } as *mut i64
    };
    unsafe { (*p).p_config = p_config };
    unsafe { (*p).p_index = p_index };
    if b_create != 0 {
        if unsafe { (*p_config).e_content } == 0 ||
                unsafe { (*p_config).e_content } == 3 {
            let mut i: i32 = 0;
            let mut z_defn: *mut i8 = core::ptr::null_mut();
            let p_defn: *mut Sqlite3Str =
                unsafe { sqlite3_str_new(unsafe { (*p_config).db }) };
            unsafe {
                sqlite3_str_appendf(p_defn,
                    c"id INTEGER PRIMARY KEY".as_ptr() as *mut i8 as *const i8)
            };
            {
                i = 0;
                '__b4: loop {
                    if !(i < unsafe { (*p_config).n_col }) { break '__b4; }
                    '__c4: loop {
                        if unsafe { (*p_config).e_content } == 0 ||
                                unsafe {
                                        *unsafe { (*p_config).ab_unindexed.offset(i as isize) }
                                    } != 0 {
                            unsafe {
                                sqlite3_str_appendf(p_defn,
                                    c", c%d".as_ptr() as *mut i8 as *const i8, i)
                            };
                        }
                        break '__c4;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if unsafe { (*p_config).b_locale } != 0 {
                {
                    i = 0;
                    '__b5: loop {
                        if !(i < unsafe { (*p_config).n_col }) { break '__b5; }
                        '__c5: loop {
                            if unsafe {
                                            *unsafe { (*p_config).ab_unindexed.offset(i as isize) }
                                        } as i32 == 0 {
                                unsafe {
                                    sqlite3_str_appendf(p_defn,
                                        c", l%d".as_ptr() as *mut i8 as *const i8, i)
                                };
                            }
                            break '__c5;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            z_defn = unsafe { sqlite3_str_finish(p_defn) };
            if !(z_defn).is_null() {
                rc =
                    sqlite3_fts5_create_table(unsafe { &*p_config },
                        c"content".as_ptr() as *mut i8 as *const i8,
                        z_defn as *const i8, 0, unsafe { &mut *pz_err });
                unsafe { sqlite3_free(z_defn as *mut ()) };
            } else { rc = 7; }
        }
        if rc == 0 && unsafe { (*p_config).b_columnsize } != 0 {
            let mut z_cols: *const i8 =
                c"id INTEGER PRIMARY KEY, sz BLOB".as_ptr() as *mut i8 as
                    *const i8;
            if unsafe { (*p_config).b_contentless_delete } != 0 {
                z_cols =
                    c"id INTEGER PRIMARY KEY, sz BLOB, origin INTEGER".as_ptr()
                            as *mut i8 as *const i8;
            }
            rc =
                sqlite3_fts5_create_table(unsafe { &*p_config },
                    c"docsize".as_ptr() as *mut i8 as *const i8, z_cols, 0,
                    unsafe { &mut *pz_err });
        }
        if rc == 0 {
            rc =
                sqlite3_fts5_create_table(unsafe { &*p_config },
                    c"config".as_ptr() as *mut i8 as *const i8,
                    c"k PRIMARY KEY, v".as_ptr() as *mut i8 as *const i8, 1,
                    unsafe { &mut *pz_err });
        }
        if rc == 0 {
            rc =
                sqlite3_fts5_storage_config_value(p,
                    c"version".as_ptr() as *mut i8 as *const i8,
                    core::ptr::null_mut(), 4);
        }
    }
    if rc != 0 { sqlite3_fts5_storage_close(p); *pp = core::ptr::null_mut(); }
    return rc;
}

extern "C" fn fts5_storage_save_totals(p: &Fts5Storage) -> i32 {
    let n_col: i32 = unsafe { (*(*p).p_config).n_col };
    let mut i: i32 = 0;
    let mut buf: Fts5Buffer = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    unsafe {
        memset(&raw mut buf as *mut (), 0,
            core::mem::size_of::<Fts5Buffer>() as u64)
    };
    unsafe {
        sqlite3_fts5_buffer_append_varint(&mut rc, &mut buf, (*p).n_total_row)
    };
    {
        i = 0;
        '__b6: loop {
            if !(i < n_col) { break '__b6; }
            '__c6: loop {
                unsafe {
                    sqlite3_fts5_buffer_append_varint(&mut rc, &mut buf,
                        unsafe { *(*p).a_total_size.offset(i as isize) })
                };
                break '__c6;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_fts5_index_set_averages((*p).p_index,
                    buf.p as *const u8, buf.n)
            };
    }
    unsafe { sqlite3_free(buf.p as *mut ()) };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_sync(p: *mut Fts5Storage) -> i32 {
    let mut rc: i32 = 0;
    let i_last_rowid: i64 =
        unsafe {
            sqlite3_last_insert_rowid(unsafe {
                    (*unsafe { (*p).p_config }).db
                })
        };
    if unsafe { (*p).b_totals_valid } != 0 {
        rc = fts5_storage_save_totals(unsafe { &*p });
        if rc == 0 { unsafe { (*p).b_totals_valid = 0 }; }
    }
    if rc == 0 {
        rc = unsafe { sqlite3_fts5_index_sync(unsafe { (*p).p_index }) };
    }
    unsafe {
        sqlite3_set_last_insert_rowid(unsafe {
                (*unsafe { (*p).p_config }).db
            }, i_last_rowid)
    };
    return rc;
}

extern "C" fn fts5_storage_rename_one(p_config_1: &Fts5Config,
    p_rc_1: &mut i32, z_tail_1: *const i8, z_name_1: *const i8) -> () {
    if *p_rc_1 == 0 {
        *p_rc_1 =
            unsafe {
                fts5_exec_printf((*p_config_1).db, core::ptr::null_mut(),
                    c"ALTER TABLE %Q.\'%q_%s\' RENAME TO \'%q_%s\';".as_ptr() as
                            *mut i8 as *const i8, (*p_config_1).z_db,
                    (*p_config_1).z_name, z_tail_1, z_name_1, z_tail_1)
            };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_rename(p_storage: *mut Fts5Storage,
    z_name: *const i8) -> i32 {
    let p_config: *mut Fts5Config = unsafe { (*p_storage).p_config };
    let mut rc: i32 = sqlite3_fts5_storage_sync(p_storage);
    fts5_storage_rename_one(unsafe { &*p_config }, &mut rc,
        c"data".as_ptr() as *mut i8 as *const i8, z_name);
    fts5_storage_rename_one(unsafe { &*p_config }, &mut rc,
        c"idx".as_ptr() as *mut i8 as *const i8, z_name);
    fts5_storage_rename_one(unsafe { &*p_config }, &mut rc,
        c"config".as_ptr() as *mut i8 as *const i8, z_name);
    if unsafe { (*p_config).b_columnsize } != 0 {
        fts5_storage_rename_one(unsafe { &*p_config }, &mut rc,
            c"docsize".as_ptr() as *mut i8 as *const i8, z_name);
    }
    if unsafe { (*p_config).e_content } == 0 {
        fts5_storage_rename_one(unsafe { &*p_config }, &mut rc,
            c"content".as_ptr() as *mut i8 as *const i8, z_name);
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_drop_all(p_config: &Fts5Config) -> i32 {
    let mut rc: i32 =
        unsafe {
            fts5_exec_printf((*p_config).db, core::ptr::null_mut(),
                c"DROP TABLE IF EXISTS %Q.\'%q_data\';DROP TABLE IF EXISTS %Q.\'%q_idx\';DROP TABLE IF EXISTS %Q.\'%q_config\';".as_ptr()
                        as *mut i8 as *const i8, (*p_config).z_db,
                (*p_config).z_name, (*p_config).z_db, (*p_config).z_name,
                (*p_config).z_db, (*p_config).z_name)
        };
    if rc == 0 && (*p_config).b_columnsize != 0 {
        rc =
            unsafe {
                fts5_exec_printf((*p_config).db, core::ptr::null_mut(),
                    c"DROP TABLE IF EXISTS %Q.\'%q_docsize\';".as_ptr() as
                            *mut i8 as *const i8, (*p_config).z_db, (*p_config).z_name)
            };
    }
    if rc == 0 && (*p_config).e_content == 0 {
        rc =
            unsafe {
                fts5_exec_printf((*p_config).db, core::ptr::null_mut(),
                    c"DROP TABLE IF EXISTS %Q.\'%q_content\';".as_ptr() as
                            *mut i8 as *const i8, (*p_config).z_db, (*p_config).z_name)
            };
    }
    return rc;
}

extern "C" fn fts5_storage_load_totals(p: &mut Fts5Storage, b_cache_1: i32)
    -> i32 {
    let mut rc: i32 = 0;
    if (*p).b_totals_valid == 0 {
        rc =
            unsafe {
                sqlite3_fts5_index_get_averages((*p).p_index,
                    &mut (*p).n_total_row, (*p).a_total_size)
            };
        (*p).b_totals_valid = b_cache_1;
    }
    return rc;
}

extern "C" fn fts5_storage_contentless_delete(p: *mut Fts5Storage,
    i_del_1: i64) -> i32 {
    let mut i_origin: i64 = 0 as i64;
    let mut p_lookup: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if (unsafe { (*unsafe { (*p).p_config }).b_contentless_delete } == 0) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageContentlessDelete".as_ptr() as
                    *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 630,
                c"p->pConfig->bContentlessDelete".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*unsafe { (*p).p_config }).e_content } == 1 ||
                            unsafe { (*unsafe { (*p).p_config }).e_content } == 3) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageContentlessDelete".as_ptr() as
                    *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 633,
                c"p->pConfig->eContent==FTS5_CONTENT_NONE || p->pConfig->eContent==FTS5_CONTENT_UNINDEXED".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc =
        fts5_storage_get_stmt(unsafe { &mut *p }, 9, &mut p_lookup,
            core::ptr::null_mut());
    if rc == 0 {
        unsafe { sqlite3_bind_int64(p_lookup, 1, i_del_1) };
        if 100 == unsafe { sqlite3_step(p_lookup) } {
            i_origin = unsafe { sqlite3_column_int64(p_lookup, 1) };
        }
        rc = unsafe { sqlite3_reset(p_lookup) };
    }
    if rc == 0 && i_origin != 0 as i64 {
        rc =
            unsafe {
                sqlite3_fts5_index_contentless_delete(unsafe { (*p).p_index },
                    i_origin, i_del_1)
            };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_find_delete_row(p: *mut Fts5Storage,
    i_del: i64) -> i32 {
    let mut rc: i32 = 0;
    let mut p_seek: *mut Sqlite3Stmt = core::ptr::null_mut();
    if !(unsafe { (*p).p_saved_row } == core::ptr::null_mut()) as i32 as i64
            != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5StorageFindDeleteRow".as_ptr() as
                    *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 479,
                c"p->pSavedRow==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc =
        fts5_storage_get_stmt(unsafe { &mut *p }, 2 + 1, &mut p_seek,
            core::ptr::null_mut());
    if rc == 0 {
        unsafe { sqlite3_bind_int64(p_seek, 1, i_del) };
        if unsafe { sqlite3_step(p_seek) } != 100 {
            rc = unsafe { sqlite3_reset(p_seek) };
        } else { unsafe { (*p).p_saved_row = p_seek }; }
    }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5InsertCtx {
    p_storage: *mut Fts5Storage,
    i_col: i32,
    sz_col: i32,
}

extern "C" fn fts5_storage_insert_callback(p_context_1: *mut (), tflags: i32,
    p_token_1: *const i8, mut n_token_1: i32, i_unused1_1: i32,
    i_unused2_1: i32) -> i32 {
    let p_ctx: *mut Fts5InsertCtx = p_context_1 as *mut Fts5InsertCtx;
    let p_idx: *mut Fts5Index =
        unsafe { (*unsafe { (*p_ctx).p_storage }).p_index };
    { { let _ = i_unused1_1; }; { let _ = i_unused2_1; } };
    if n_token_1 > 32768 { n_token_1 = 32768; }
    if tflags & 1 == 0 || unsafe { (*p_ctx).sz_col } == 0 {
        {
            let __p = unsafe { &mut (*p_ctx).sz_col };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    return unsafe {
            sqlite3_fts5_index_write(p_idx, unsafe { (*p_ctx).i_col },
                unsafe { (*p_ctx).sz_col } - 1, p_token_1, n_token_1)
        };
}

extern "C" fn fts5_storage_delete_from_index(p: *mut Fts5Storage,
    i_del_1: i64, ap_val_1: *const *mut Sqlite3Value, b_save_row_1: i32)
    -> i32 {
    let p_config: *mut Fts5Config = unsafe { (*p).p_config };
    let mut p_seek: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut rc2: i32 = 0;
    let mut i_col: i32 = 0;
    let mut ctx: Fts5InsertCtx = unsafe { core::mem::zeroed() };
    if !(b_save_row_1 == 0 || ap_val_1 == core::ptr::null_mut()) as i32 as i64
            != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageDeleteFromIndex".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 516,
                c"bSaveRow==0 || apVal==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(b_save_row_1 == 0 || b_save_row_1 == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageDeleteFromIndex".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 517,
                c"bSaveRow==0 || bSaveRow==1".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(3 == 2 + 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5StorageDeleteFromIndex".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 518,
                c"FTS5_STMT_LOOKUP2==FTS5_STMT_LOOKUP+1".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    if ap_val_1 == core::ptr::null_mut() {
        if !(unsafe { (*p).p_saved_row }).is_null() && b_save_row_1 != 0 {
            p_seek = unsafe { (*p).p_saved_row };
            unsafe { (*p).p_saved_row = core::ptr::null_mut() };
        } else {
            rc =
                fts5_storage_get_stmt(unsafe { &mut *p }, 2 + b_save_row_1,
                    &mut p_seek, core::ptr::null_mut());
            if rc != 0 { return rc; }
            unsafe { sqlite3_bind_int64(p_seek, 1, i_del_1) };
            if unsafe { sqlite3_step(p_seek) } != 100 {
                return unsafe { sqlite3_reset(p_seek) };
            }
        }
    }
    ctx.p_storage = p;
    ctx.i_col = -1;
    {
        i_col = 1;
        '__b7: loop {
            if !(rc == 0 && i_col <= unsafe { (*p_config).n_col }) {
                break '__b7;
            }
            '__c7: loop {
                if unsafe {
                                *unsafe {
                                        (*p_config).ab_unindexed.offset((i_col - 1) as isize)
                                    }
                            } as i32 == 0 {
                    let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
                    let mut p_free: *mut Sqlite3Value = core::ptr::null_mut();
                    let mut p_text: *const i8 = core::ptr::null();
                    let mut n_text: i32 = 0;
                    let mut p_loc: *const i8 = core::ptr::null();
                    let mut n_loc: i32 = 0;
                    if !(p_seek == core::ptr::null_mut() ||
                                            ap_val_1 == core::ptr::null_mut()) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5StorageDeleteFromIndex".as_ptr() as
                                    *const i8,
                                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 545,
                                c"pSeek==0 || apVal==0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(p_seek != core::ptr::null_mut() ||
                                            ap_val_1 != core::ptr::null_mut()) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5StorageDeleteFromIndex".as_ptr() as
                                    *const i8,
                                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 546,
                                c"pSeek!=0 || apVal!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(p_seek).is_null() {
                        p_val = unsafe { sqlite3_column_value(p_seek, i_col) };
                    } else {
                        p_val = unsafe { *ap_val_1.offset((i_col - 1) as isize) };
                    }
                    if unsafe { (*p_config).b_locale } != 0 &&
                            unsafe { sqlite3_fts5_is_locale_value(p_config, p_val) } !=
                                0 {
                        rc =
                            unsafe {
                                sqlite3_fts5_decode_locale_value(p_val, &mut p_text,
                                    &mut n_text, &mut p_loc, &mut n_loc)
                            };
                    } else {
                        if unsafe { sqlite3_value_type(p_val) } != 3 {
                            p_free =
                                {
                                    p_val =
                                        unsafe { sqlite3_value_dup(p_val as *const Sqlite3Value) };
                                    p_val
                                };
                            if p_val == core::ptr::null_mut() { rc = 7; }
                        }
                        if rc == 0 {
                            p_text = unsafe { sqlite3_value_text(p_val) } as *const i8;
                            n_text = unsafe { sqlite3_value_bytes(p_val) };
                            if unsafe { (*p_config).b_locale } != 0 &&
                                    !(p_seek).is_null() {
                                p_loc =
                                    unsafe {
                                            sqlite3_column_text(p_seek,
                                                i_col + unsafe { (*p_config).n_col })
                                        } as *const i8;
                                n_loc =
                                    unsafe {
                                        sqlite3_column_bytes(p_seek,
                                            i_col + unsafe { (*p_config).n_col })
                                    };
                            }
                        }
                    }
                    if rc == 0 {
                        unsafe { sqlite3_fts5_set_locale(p_config, p_loc, n_loc) };
                        ctx.sz_col = 0;
                        rc =
                            unsafe {
                                sqlite3_fts5_tokenize(p_config, 4, p_text, n_text,
                                    &raw mut ctx as *mut (), Some(fts5_storage_insert_callback))
                            };
                        unsafe {
                            *unsafe { (*p).a_total_size.offset((i_col - 1) as isize) }
                                -= ctx.sz_col as i64
                        };
                        if rc == 0 &&
                                unsafe {
                                        *unsafe { (*p).a_total_size.offset((i_col - 1) as isize) }
                                    } < 0 as i64 {
                            rc = 11 | 1 << 8;
                        }
                        unsafe { sqlite3_fts5_clear_locale(p_config) };
                    }
                    unsafe { sqlite3_value_free(p_free) };
                }
                break '__c7;
            }
            { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 && unsafe { (*p).n_total_row } < 1 as i64 {
        rc = 11 | 1 << 8;
    } else {
        {
            let __p = unsafe { &mut (*p).n_total_row };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
    if rc == 0 && b_save_row_1 != 0 {
        if !(unsafe { (*p).p_saved_row } == core::ptr::null_mut()) as i32 as
                    i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5StorageDeleteFromIndex".as_ptr() as
                        *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 597,
                    c"p->pSavedRow==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p).p_saved_row = p_seek };
    } else {
        rc2 = unsafe { sqlite3_reset(p_seek) };
        if rc == 0 { rc = rc2; }
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_delete(p: *mut Fts5Storage, i_del: i64,
    ap_val: *mut *mut Sqlite3Value, b_save_row: i32) -> i32 {
    let p_config: *const Fts5Config =
        unsafe { (*p).p_config } as *const Fts5Config;
    let mut rc: i32 = 0;
    let mut p_del: *mut Sqlite3Stmt = core::ptr::null_mut();
    if !(unsafe { (*p_config).e_content } != 0 ||
                            ap_val == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5StorageDelete".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 746,
                c"pConfig->eContent!=FTS5_CONTENT_NORMAL || apVal==0".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc = fts5_storage_load_totals(unsafe { &mut *p }, 1);
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_fts5_index_begin_write(unsafe { (*p).p_index }, 1,
                    i_del)
            };
    }
    if rc == 0 {
        if unsafe { (*unsafe { (*p).p_config }).b_contentless_delete } != 0 {
            rc = fts5_storage_contentless_delete(p, i_del);
            if rc == 0 && b_save_row != 0 &&
                    unsafe { (*unsafe { (*p).p_config }).e_content } == 3 {
                rc = sqlite3_fts5_storage_find_delete_row(p, i_del);
            }
        } else {
            rc =
                fts5_storage_delete_from_index(p, i_del,
                    ap_val as *const *mut Sqlite3Value, b_save_row);
        }
    }
    if rc == 0 && unsafe { (*p_config).b_columnsize } != 0 {
        rc =
            fts5_storage_get_stmt(unsafe { &mut *p }, 8, &mut p_del,
                core::ptr::null_mut());
        if rc == 0 {
            unsafe { sqlite3_bind_int64(p_del, 1, i_del) };
            unsafe { sqlite3_step(p_del) };
            rc = unsafe { sqlite3_reset(p_del) };
        }
    }
    if unsafe { (*p_config).e_content } == 0 ||
            unsafe { (*p_config).e_content } == 3 {
        if rc == 0 {
            rc =
                fts5_storage_get_stmt(unsafe { &mut *p }, 6, &mut p_del,
                    core::ptr::null_mut());
        }
        if rc == 0 {
            unsafe { sqlite3_bind_int64(p_del, 1, i_del) };
            unsafe { sqlite3_step(p_del) };
            rc = unsafe { sqlite3_reset(p_del) };
        }
    }
    return rc;
}

extern "C" fn fts5_storage_new_rowid(p: *mut Fts5Storage,
    pi_rowid_1: &mut i64) -> i32 {
    let mut rc: i32 = 20;
    if unsafe { (*unsafe { (*p).p_config }).b_columnsize } != 0 {
        let mut p_replace: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            fts5_storage_get_stmt(unsafe { &mut *p }, 7, &mut p_replace,
                core::ptr::null_mut());
        if rc == 0 {
            unsafe { sqlite3_bind_null(p_replace, 1) };
            unsafe { sqlite3_bind_null(p_replace, 2) };
            unsafe { sqlite3_step(p_replace) };
            rc = unsafe { sqlite3_reset(p_replace) };
        }
        if rc == 0 {
            *pi_rowid_1 =
                unsafe {
                    sqlite3_last_insert_rowid(unsafe {
                            (*unsafe { (*p).p_config }).db
                        })
                };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_content_insert(p: *mut Fts5Storage,
    b_replace: i32, ap_val: *mut *mut Sqlite3Value, pi_rowid: *mut i64)
    -> i32 {
    let p_config: *mut Fts5Config = unsafe { (*p).p_config };
    let mut rc: i32 = 0;
    if unsafe { (*p_config).e_content } != 0 &&
            unsafe { (*p_config).e_content } != 3 {
        if unsafe {
                    sqlite3_value_type(unsafe { *ap_val.offset(1 as isize) })
                } == 1 {
            unsafe {
                *pi_rowid =
                    unsafe {
                        sqlite3_value_int64(unsafe { *ap_val.offset(1 as isize) })
                    }
            };
        } else { rc = fts5_storage_new_rowid(p, unsafe { &mut *pi_rowid }); }
    } else {
        let mut p_insert: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut i: i32 = 0;
        if !(4 + 1 == 5) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5StorageContentInsert".as_ptr() as
                        *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 975,
                    c"FTS5_STMT_INSERT_CONTENT+1==FTS5_STMT_REPLACE_CONTENT".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(b_replace == 0 || b_replace == 1) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5StorageContentInsert".as_ptr() as
                        *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 976,
                    c"bReplace==0 || bReplace==1".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        rc =
            fts5_storage_get_stmt(unsafe { &mut *p }, 4 + b_replace,
                &mut p_insert, core::ptr::null_mut());
        if !(p_insert).is_null() {
            unsafe { sqlite3_clear_bindings(p_insert) };
        }
        unsafe {
            sqlite3_bind_value(p_insert, 1,
                unsafe { *ap_val.offset(1 as isize) } as *const Sqlite3Value)
        };
        {
            i = 2;
            '__b8: loop {
                if !(rc == 0 && i <= unsafe { (*p_config).n_col } + 1) {
                    break '__b8;
                }
                '__c8: loop {
                    let b_unindexed: i32 =
                        unsafe {
                                *unsafe {
                                        (*p_config).ab_unindexed.offset((i - 2) as isize)
                                    }
                            } as i32;
                    if unsafe { (*p_config).e_content } == 0 || b_unindexed != 0
                        {
                        let mut p_val: *mut Sqlite3Value =
                            unsafe { *ap_val.offset(i as isize) };
                        if unsafe { sqlite3_value_nochange(p_val) } != 0 &&
                                !(unsafe { (*p).p_saved_row }).is_null() {
                            p_val =
                                unsafe {
                                    sqlite3_column_value(unsafe { (*p).p_saved_row }, i - 1)
                                };
                            if unsafe { (*p_config).b_locale } != 0 && b_unindexed == 0
                                {
                                unsafe {
                                    sqlite3_bind_value(p_insert,
                                        unsafe { (*p_config).n_col } + i,
                                        unsafe {
                                                sqlite3_column_value(unsafe { (*p).p_saved_row },
                                                    unsafe { (*p_config).n_col } + i - 1)
                                            } as *const Sqlite3Value)
                                };
                            }
                        } else if unsafe {
                                    sqlite3_fts5_is_locale_value(p_config, p_val)
                                } != 0 {
                            let mut p_text: *const i8 = core::ptr::null();
                            let mut p_loc: *const i8 = core::ptr::null();
                            let mut n_text: i32 = 0;
                            let mut n_loc: i32 = 0;
                            if (unsafe { (*p_config).b_locale } == 0) as i32 as i64 != 0
                                {
                                unsafe {
                                    __assert_rtn(c"sqlite3Fts5StorageContentInsert".as_ptr() as
                                            *const i8,
                                        c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1004,
                                        c"pConfig->bLocale".as_ptr() as *mut i8 as *const i8)
                                }
                            } else { { let _ = 0; } };
                            rc =
                                unsafe {
                                    sqlite3_fts5_decode_locale_value(p_val, &mut p_text,
                                        &mut n_text, &mut p_loc, &mut n_loc)
                                };
                            if rc == 0 {
                                unsafe {
                                    sqlite3_bind_text(p_insert, i, p_text, n_text,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                                if b_unindexed == 0 {
                                    let i_loc: i32 = unsafe { (*p_config).n_col } + i;
                                    unsafe {
                                        sqlite3_bind_text(p_insert, i_loc, p_loc, n_loc,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                }
                            }
                            break '__c8;
                        }
                        rc =
                            unsafe {
                                sqlite3_bind_value(p_insert, i,
                                    p_val as *const Sqlite3Value)
                            };
                    }
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 {
            unsafe { sqlite3_step(p_insert) };
            rc = unsafe { sqlite3_reset(p_insert) };
        }
        unsafe {
            *pi_rowid =
                unsafe {
                    sqlite3_last_insert_rowid(unsafe { (*p_config).db })
                }
        };
    }
    return rc;
}

extern "C" fn fts5_storage_insert_docsize(p: *mut Fts5Storage, i_rowid_1: i64,
    p_buf_1: &Fts5Buffer) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*unsafe { (*p).p_config }).b_columnsize } != 0 {
        let mut p_replace: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            fts5_storage_get_stmt(unsafe { &mut *p }, 7, &mut p_replace,
                core::ptr::null_mut());
        if rc == 0 {
            unsafe { sqlite3_bind_int64(p_replace, 1, i_rowid_1) };
            if unsafe { (*unsafe { (*p).p_config }).b_contentless_delete } !=
                    0 {
                let mut i_origin: i64 = 0 as i64;
                rc =
                    unsafe {
                        sqlite3_fts5_index_get_origin(unsafe { (*p).p_index },
                            &mut i_origin)
                    };
                unsafe { sqlite3_bind_int64(p_replace, 3, i_origin) };
            }
        }
        if rc == 0 {
            unsafe {
                sqlite3_bind_blob(p_replace, 2, (*p_buf_1).p as *const (),
                    (*p_buf_1).n, None)
            };
            unsafe { sqlite3_step(p_replace) };
            rc = unsafe { sqlite3_reset(p_replace) };
            unsafe { sqlite3_bind_null(p_replace, 2) };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_index_insert(p: *mut Fts5Storage,
    ap_val: *mut *mut Sqlite3Value, i_rowid: i64) -> i32 {
    let p_config: *mut Fts5Config = unsafe { (*p).p_config };
    let mut rc: i32 = 0;
    let mut ctx: Fts5InsertCtx = unsafe { core::mem::zeroed() };
    let mut buf: Fts5Buffer = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut buf as *mut (), 0,
            core::mem::size_of::<Fts5Buffer>() as u64)
    };
    ctx.p_storage = p;
    rc = fts5_storage_load_totals(unsafe { &mut *p }, 1);
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_fts5_index_begin_write(unsafe { (*p).p_index }, 0,
                    i_rowid)
            };
    }
    {
        ctx.i_col = 0;
        '__b9: loop {
            if !(rc == 0 && ctx.i_col < unsafe { (*p_config).n_col }) {
                break '__b9;
            }
            '__c9: loop {
                ctx.sz_col = 0;
                if unsafe {
                                *unsafe {
                                        (*p_config).ab_unindexed.offset(ctx.i_col as isize)
                                    }
                            } as i32 == 0 {
                    let mut n_text: i32 = 0;
                    let mut p_text: *const i8 = core::ptr::null();
                    let mut n_loc: i32 = 0;
                    let mut p_loc: *const i8 = core::ptr::null();
                    let mut p_val: *mut Sqlite3Value =
                        unsafe { *ap_val.offset((ctx.i_col + 2) as isize) };
                    if !(unsafe { (*p).p_saved_row }).is_null() &&
                            unsafe { sqlite3_value_nochange(p_val) } != 0 {
                        p_val =
                            unsafe {
                                sqlite3_column_value(unsafe { (*p).p_saved_row },
                                    ctx.i_col + 1)
                            };
                        if unsafe { (*p_config).e_content } == 0 &&
                                unsafe { (*p_config).b_locale } != 0 {
                            let i_col: i32 =
                                ctx.i_col + 1 + unsafe { (*p_config).n_col };
                            p_loc =
                                unsafe {
                                        sqlite3_column_text(unsafe { (*p).p_saved_row }, i_col)
                                    } as *const i8;
                            n_loc =
                                unsafe {
                                    sqlite3_column_bytes(unsafe { (*p).p_saved_row }, i_col)
                                };
                        }
                    } else {
                        p_val = unsafe { *ap_val.offset((ctx.i_col + 2) as isize) };
                    }
                    if unsafe { (*p_config).b_locale } != 0 &&
                            unsafe { sqlite3_fts5_is_locale_value(p_config, p_val) } !=
                                0 {
                        rc =
                            unsafe {
                                sqlite3_fts5_decode_locale_value(p_val, &mut p_text,
                                    &mut n_text, &mut p_loc, &mut n_loc)
                            };
                    } else {
                        p_text = unsafe { sqlite3_value_text(p_val) } as *const i8;
                        n_text = unsafe { sqlite3_value_bytes(p_val) };
                    }
                    if rc == 0 {
                        unsafe { sqlite3_fts5_set_locale(p_config, p_loc, n_loc) };
                        rc =
                            unsafe {
                                sqlite3_fts5_tokenize(p_config, 4, p_text, n_text,
                                    &raw mut ctx as *mut (), Some(fts5_storage_insert_callback))
                            };
                        unsafe { sqlite3_fts5_clear_locale(p_config) };
                    }
                }
                unsafe {
                    sqlite3_fts5_buffer_append_varint(&mut rc, &mut buf,
                        ctx.sz_col as i64)
                };
                unsafe {
                    *unsafe { (*p).a_total_size.offset(ctx.i_col as isize) } +=
                        ctx.sz_col as i64
                };
                break '__c9;
            }
            { let __p = &mut ctx.i_col; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        let __p = unsafe { &mut (*p).n_total_row };
        let __t = *__p;
        *__p += 1;
        __t
    };
    if rc == 0 { rc = fts5_storage_insert_docsize(p, i_rowid, &buf); }
    unsafe { sqlite3_free(buf.p as *mut ()) };
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5IntegrityCtx {
    i_rowid: i64,
    i_col: i32,
    sz_col: i32,
    cksum: u64,
    p_termset: *mut Fts5Termset,
    p_config: *mut Fts5Config,
}

extern "C" fn fts5_storage_decode_size_array(mut a_col_1: &mut [i32],
    a_blob_1: &[u8]) -> i32 {
    let mut i: i32 = 0;
    let mut i_off: i32 = 0;
    {
        i = 0;
        '__b10: loop {
            if !(i < a_col_1.len() as i32) { break '__b10; }
            '__c10: loop {
                if i_off >= a_blob_1.len() as i32 { return 1; }
                i_off +=
                    unsafe {
                        sqlite3_fts5_get_varint32(&a_blob_1[i_off as usize],
                            &raw mut a_col_1[i as usize] as *mut u32)
                    };
                break '__c10;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (i_off != a_blob_1.len() as i32) as i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_docsize(p: *mut Fts5Storage,
    i_rowid: i64, a_col: *mut i32) -> i32 {
    let n_col: i32 = unsafe { (*unsafe { (*p).p_config }).n_col };
    let mut p_lookup: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if (unsafe { (*unsafe { (*p).p_config }).b_columnsize } == 0) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5StorageDocsize".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1420,
                c"p->pConfig->bColumnsize".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc =
        fts5_storage_get_stmt(unsafe { &mut *p }, 9, &mut p_lookup,
            core::ptr::null_mut());
    if !(p_lookup).is_null() {
        let mut b_corrupt: i32 = 1;
        if !(rc == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5StorageDocsize".as_ptr() as
                        *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1424,
                    c"rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { sqlite3_bind_int64(p_lookup, 1, i_rowid) };
        if 100 == unsafe { sqlite3_step(p_lookup) } {
            let a_blob: *const u8 =
                unsafe { sqlite3_column_blob(p_lookup, 0) } as *const u8;
            let n_blob: i32 = unsafe { sqlite3_column_bytes(p_lookup, 0) };
            if 0 ==
                    fts5_storage_decode_size_array(unsafe {
                            let __p = a_col as *mut i32;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(__p, n_col as usize)
                            }
                        },
                        unsafe {
                            let __p = a_blob as *const u8;
                            if __p.is_null() {
                                &[]
                            } else { core::slice::from_raw_parts(__p, n_blob as usize) }
                        }) {
                b_corrupt = 0;
            }
        }
        rc = unsafe { sqlite3_reset(p_lookup) };
        if b_corrupt != 0 && rc == 0 { rc = 11 | 1 << 8; }
    } else {
        if !(rc != 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5StorageDocsize".as_ptr() as
                        *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1438,
                    c"rc!=SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    return rc;
}

extern "C" fn fts5_storage_integrity_callback(p_context_1: *mut (),
    tflags: i32, p_token_1: *const i8, mut n_token_1: i32, i_unused1_1: i32,
    i_unused2_1: i32) -> i32 {
    let p_ctx: *mut Fts5IntegrityCtx = p_context_1 as *mut Fts5IntegrityCtx;
    let p_termset: *mut Fts5Termset = unsafe { (*p_ctx).p_termset };
    let mut b_present: i32 = 0;
    let mut ii: i32 = 0;
    let mut rc: i32 = 0;
    let mut i_pos: i32 = 0;
    let mut i_col: i32 = 0;
    { { let _ = i_unused1_1; }; { let _ = i_unused2_1; } };
    if n_token_1 > 32768 { n_token_1 = 32768; }
    if tflags & 1 == 0 || unsafe { (*p_ctx).sz_col } == 0 {
        {
            let __p = unsafe { &mut (*p_ctx).sz_col };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    '__s11:
        {
        match unsafe { (*unsafe { (*p_ctx).p_config }).e_detail } {
            0 => {
                i_pos = unsafe { (*p_ctx).sz_col } - 1;
                i_col = unsafe { (*p_ctx).i_col };
            }
            2 => { i_pos = unsafe { (*p_ctx).i_col }; i_col = 0; }
            _ => {
                if !(unsafe { (*unsafe { (*p_ctx).p_config }).e_detail } == 1)
                                as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"fts5StorageIntegrityCallback".as_ptr() as
                                *const i8,
                            c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1178,
                            c"pCtx->pConfig->eDetail==FTS5_DETAIL_NONE".as_ptr() as
                                    *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                i_pos = 0;
                i_col = 0;
            }
        }
    }
    rc =
        unsafe {
            sqlite3_fts5_termset_add(p_termset, 0, p_token_1, n_token_1,
                &mut b_present)
        };
    if rc == 0 && b_present == 0 {
        unsafe {
            (*p_ctx).cksum ^=
                unsafe {
                    sqlite3_fts5_index_entry_cksum(unsafe { (*p_ctx).i_rowid },
                        i_col, i_pos, 0, p_token_1, n_token_1)
                }
        };
    }
    {
        ii = 0;
        '__b12: loop {
            if !(rc == 0 &&
                            ii < unsafe { (*unsafe { (*p_ctx).p_config }).n_prefix }) {
                break '__b12;
            }
            '__c12: loop {
                let n_char: i32 =
                    unsafe {
                            *unsafe {
                                    (*unsafe { (*p_ctx).p_config }).a_prefix.offset(ii as isize)
                                }
                        } as i32;
                let n_byte: i32 =
                    unsafe {
                        sqlite3_fts5_index_charlen_to_bytelen(p_token_1, n_token_1,
                            n_char)
                    };
                if n_byte != 0 {
                    rc =
                        unsafe {
                            sqlite3_fts5_termset_add(p_termset, ii + 1, p_token_1,
                                n_byte, &mut b_present)
                        };
                    if b_present == 0 {
                        unsafe {
                            (*p_ctx).cksum ^=
                                unsafe {
                                    sqlite3_fts5_index_entry_cksum(unsafe { (*p_ctx).i_rowid },
                                        i_col, i_pos, ii + 1, p_token_1, n_byte)
                                }
                        };
                    }
                }
                break '__c12;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return rc;
}

extern "C" fn fts5_storage_count(p: &Fts5Storage, z_suffix_1: *const i8,
    pn_row_1: &mut i64) -> i32 {
    let p_config: *const Fts5Config = (*p).p_config as *const Fts5Config;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT count(*) FROM %Q.\'%q_%s\'".as_ptr() as
                        *mut i8 as *const i8, unsafe { (*p_config).z_db },
                unsafe { (*p_config).z_name }, z_suffix_1)
        };
    if z_sql == core::ptr::null_mut() {
        rc = 7;
    } else {
        let mut p_cnt: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p_config).db },
                    z_sql as *const i8, -1, &mut p_cnt, core::ptr::null_mut())
            };
        if rc == 0 {
            if 100 == unsafe { sqlite3_step(p_cnt) } {
                *pn_row_1 = unsafe { sqlite3_column_int64(p_cnt, 0) };
            }
            rc = unsafe { sqlite3_finalize(p_cnt) };
        }
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_integrity(p: *mut Fts5Storage,
    i_arg: i32) -> i32 {
    let p_config: *mut Fts5Config = unsafe { (*p).p_config };
    let mut rc: i32 = 0;
    let mut a_col_size: *mut i32 = core::ptr::null_mut();
    let mut a_total_size: *mut i64 = core::ptr::null_mut();
    let mut ctx: Fts5IntegrityCtx = unsafe { core::mem::zeroed() };
    let mut p_scan: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut b_use_cksum: i32 = 0;
    unsafe {
        memset(&raw mut ctx as *mut (), 0,
            core::mem::size_of::<Fts5IntegrityCtx>() as u64)
    };
    ctx.p_config = unsafe { (*p).p_config };
    a_total_size =
        unsafe {
                sqlite3_malloc64(unsafe { (*p_config).n_col } as u64 *
                        (core::mem::size_of::<i32>() as u64 +
                            core::mem::size_of::<i64>() as u64))
            } as *mut i64;
    if (a_total_size).is_null() as i32 != 0 { return 7; }
    a_col_size =
        unsafe {
                &raw mut *a_total_size.offset(unsafe { (*p_config).n_col } as
                                isize)
            } as *mut i32;
    unsafe {
        memset(a_total_size as *mut (), 0,
            core::mem::size_of::<i64>() as u64 *
                unsafe { (*p_config).n_col } as u64)
    };
    b_use_cksum =
        (unsafe { (*p_config).e_content } == 0 ||
                unsafe { (*p_config).e_content } == 2 && i_arg != 0) as i32;
    if b_use_cksum != 0 {
        rc =
            fts5_storage_get_stmt(unsafe { &mut *p }, 11, &mut p_scan,
                core::ptr::null_mut());
        if rc == 0 {
            let mut rc2: i32 = 0;
            while 100 == unsafe { sqlite3_step(p_scan) } {
                let mut i: i32 = 0;
                ctx.i_rowid = unsafe { sqlite3_column_int64(p_scan, 0) };
                ctx.sz_col = 0;
                if unsafe { (*p_config).b_columnsize } != 0 {
                    rc =
                        sqlite3_fts5_storage_docsize(p, ctx.i_rowid, a_col_size);
                }
                if rc == 0 && unsafe { (*p_config).e_detail } == 1 {
                    rc =
                        unsafe { sqlite3_fts5_termset_new(&mut ctx.p_termset) };
                }
                {
                    i = 0;
                    '__b14: loop {
                        if !(rc == 0 && i < unsafe { (*p_config).n_col }) {
                            break '__b14;
                        }
                        '__c14: loop {
                            if unsafe {
                                            *unsafe { (*p_config).ab_unindexed.offset(i as isize) }
                                        } as i32 == 0 {
                                let mut p_text: *const i8 = core::ptr::null();
                                let mut n_text: i32 = 0;
                                let mut p_loc: *const i8 = core::ptr::null();
                                let mut n_loc: i32 = 0;
                                let p_val: *mut Sqlite3Value =
                                    unsafe { sqlite3_column_value(p_scan, i + 1) };
                                if unsafe { (*p_config).e_content } == 2 &&
                                        unsafe { sqlite3_fts5_is_locale_value(p_config, p_val) } !=
                                            0 {
                                    rc =
                                        unsafe {
                                            sqlite3_fts5_decode_locale_value(p_val, &mut p_text,
                                                &mut n_text, &mut p_loc, &mut n_loc)
                                        };
                                } else {
                                    if unsafe { (*p_config).e_content } == 0 &&
                                            unsafe { (*p_config).b_locale } != 0 {
                                        let i_col: i32 = i + 1 + unsafe { (*p_config).n_col };
                                        p_loc =
                                            unsafe { sqlite3_column_text(p_scan, i_col) } as *const i8;
                                        n_loc = unsafe { sqlite3_column_bytes(p_scan, i_col) };
                                    }
                                    p_text = unsafe { sqlite3_value_text(p_val) } as *const i8;
                                    n_text = unsafe { sqlite3_value_bytes(p_val) };
                                }
                                ctx.i_col = i;
                                ctx.sz_col = 0;
                                if rc == 0 && unsafe { (*p_config).e_detail } == 2 {
                                    rc =
                                        unsafe { sqlite3_fts5_termset_new(&mut ctx.p_termset) };
                                }
                                if rc == 0 {
                                    unsafe { sqlite3_fts5_set_locale(p_config, p_loc, n_loc) };
                                    rc =
                                        unsafe {
                                            sqlite3_fts5_tokenize(p_config, 4, p_text, n_text,
                                                &raw mut ctx as *mut (),
                                                Some(fts5_storage_integrity_callback))
                                        };
                                    unsafe { sqlite3_fts5_clear_locale(p_config) };
                                }
                                if rc == 0 && unsafe { (*p_config).b_columnsize } != 0 &&
                                        ctx.sz_col != unsafe { *a_col_size.offset(i as isize) } {
                                    rc = 11 | 1 << 8;
                                }
                                unsafe {
                                    *a_total_size.offset(i as isize) += ctx.sz_col as i64
                                };
                                if unsafe { (*p_config).e_detail } == 2 {
                                    unsafe { sqlite3_fts5_termset_free(ctx.p_termset) };
                                    ctx.p_termset = core::ptr::null_mut();
                                }
                            }
                            break '__c14;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { sqlite3_fts5_termset_free(ctx.p_termset) };
                ctx.p_termset = core::ptr::null_mut();
                if rc != 0 { break; }
            }
            rc2 = unsafe { sqlite3_reset(p_scan) };
            if rc == 0 { rc = rc2; }
        }
        if rc == 0 {
            let mut i: i32 = 0;
            rc = fts5_storage_load_totals(unsafe { &mut *p }, 0);
            {
                i = 0;
                '__b15: loop {
                    if !(rc == 0 && i < unsafe { (*p_config).n_col }) {
                        break '__b15;
                    }
                    '__c15: loop {
                        if unsafe {
                                    *unsafe { (*p).a_total_size.offset(i as isize) }
                                } != unsafe { *a_total_size.offset(i as isize) } {
                            rc = 11 | 1 << 8;
                        }
                        break '__c15;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if rc == 0 && unsafe { (*p_config).e_content } == 0 {
            let mut n_row: i64 = 0 as i64;
            rc =
                fts5_storage_count(unsafe { &*p },
                    c"content".as_ptr() as *mut i8 as *const i8, &mut n_row);
            if rc == 0 && n_row != unsafe { (*p).n_total_row } {
                rc = 11 | 1 << 8;
            }
        }
        if rc == 0 && unsafe { (*p_config).b_columnsize } != 0 {
            let mut n_row_1: i64 = 0 as i64;
            rc =
                fts5_storage_count(unsafe { &*p },
                    c"docsize".as_ptr() as *mut i8 as *const i8, &mut n_row_1);
            if rc == 0 && n_row_1 != unsafe { (*p).n_total_row } {
                rc = 11 | 1 << 8;
            }
        }
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_fts5_index_integrity_check(unsafe { (*p).p_index },
                    ctx.cksum, b_use_cksum)
            };
    }
    unsafe { sqlite3_free(a_total_size as *mut ()) };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_stmt(p: *mut Fts5Storage, e_stmt: i32,
    pp: *mut *mut Sqlite3Stmt, pz_err_msg: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    if !(e_stmt == 0 || e_stmt == 1 || e_stmt == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5StorageStmt".as_ptr() as *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1363,
                c"eStmt==FTS5_STMT_SCAN_ASC || eStmt==FTS5_STMT_SCAN_DESC || eStmt==FTS5_STMT_LOOKUP".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc =
        fts5_storage_get_stmt(unsafe { &mut *p }, e_stmt, unsafe { &mut *pp },
            pz_err_msg);
    if rc == 0 {
        if !(unsafe { (*p).a_stmt[e_stmt as usize] } == unsafe { *pp }) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5StorageStmt".as_ptr() as *const i8,
                    c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1366,
                    c"p->aStmt[eStmt]==*pp".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p).a_stmt[e_stmt as usize] = core::ptr::null_mut() };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_stmt_release(p: &mut Fts5Storage,
    e_stmt: i32, p_stmt: *mut Sqlite3Stmt) -> () {
    if !(e_stmt == 0 || e_stmt == 1 || e_stmt == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5StorageStmtRelease".as_ptr() as
                    *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 1385,
                c"eStmt==FTS5_STMT_SCAN_ASC || eStmt==FTS5_STMT_SCAN_DESC || eStmt==FTS5_STMT_LOOKUP".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if (*p).a_stmt[e_stmt as usize] == core::ptr::null_mut() {
        unsafe { sqlite3_reset(p_stmt) };
        (*p).a_stmt[e_stmt as usize] = p_stmt;
    } else { unsafe { sqlite3_finalize(p_stmt) }; }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_size(p: *mut Fts5Storage, i_col: i32,
    pn_token: &mut i64) -> i32 {
    let mut rc: i32 = fts5_storage_load_totals(unsafe { &mut *p }, 0);
    if rc == 0 {
        *pn_token = 0 as i64;
        if i_col < 0 {
            let mut i: i32 = 0;
            {
                i = 0;
                '__b16: loop {
                    if !(i < unsafe { (*unsafe { (*p).p_config }).n_col }) {
                        break '__b16;
                    }
                    '__c16: loop {
                        *pn_token +=
                            unsafe { *unsafe { (*p).a_total_size.offset(i as isize) } };
                        break '__c16;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else if i_col < unsafe { (*unsafe { (*p).p_config }).n_col } {
            *pn_token =
                unsafe {
                    *unsafe { (*p).a_total_size.offset(i_col as isize) }
                };
        } else { rc = 25; }
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_row_count(p: *mut Fts5Storage,
    pn_row: &mut i64) -> i32 {
    let mut rc: i32 = fts5_storage_load_totals(unsafe { &mut *p }, 0);
    if rc == 0 {
        *pn_row = unsafe { (*p).n_total_row };
        if unsafe { (*p).n_total_row } <= 0 as i64 { rc = 11 | 1 << 8; }
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_rollback(p: &mut Fts5Storage) -> i32 {
    (*p).b_totals_valid = 0;
    return unsafe { sqlite3_fts5_index_rollback((*p).p_index) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_delete_all(p: *mut Fts5Storage)
    -> i32 {
    let p_config: *const Fts5Config =
        unsafe { (*p).p_config } as *const Fts5Config;
    let mut rc: i32 = 0;
    unsafe { (*p).b_totals_valid = 0 };
    rc =
        unsafe {
            fts5_exec_printf(unsafe { (*p_config).db }, core::ptr::null_mut(),
                c"DELETE FROM %Q.\'%q_data\';DELETE FROM %Q.\'%q_idx\';".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p_config).z_db },
                unsafe { (*p_config).z_name }, unsafe { (*p_config).z_db },
                unsafe { (*p_config).z_name })
        };
    if rc == 0 && unsafe { (*p_config).b_columnsize } != 0 {
        rc =
            unsafe {
                fts5_exec_printf(unsafe { (*p_config).db },
                    core::ptr::null_mut(),
                    c"DELETE FROM %Q.\'%q_docsize\';".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_config).z_db },
                    unsafe { (*p_config).z_name })
            };
    }
    if rc == 0 && unsafe { (*p_config).e_content } == 3 {
        rc =
            unsafe {
                fts5_exec_printf(unsafe { (*p_config).db },
                    core::ptr::null_mut(),
                    c"DELETE FROM %Q.\'%q_content\';".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_config).z_db },
                    unsafe { (*p_config).z_name })
            };
    }
    if rc == 0 {
        rc = unsafe { sqlite3_fts5_index_reinit(unsafe { (*p).p_index }) };
    }
    if rc == 0 {
        rc =
            sqlite3_fts5_storage_config_value(p,
                c"version".as_ptr() as *mut i8 as *const i8,
                core::ptr::null_mut(), 4);
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_rebuild(p: *mut Fts5Storage) -> i32 {
    let mut buf: Fts5Buffer =
        Fts5Buffer { p: core::ptr::null_mut(), n: 0, n_space: 0 };
    let p_config: *mut Fts5Config = unsafe { (*p).p_config };
    let mut p_scan: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut ctx: Fts5InsertCtx = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut rc2: i32 = 0;
    unsafe {
        memset(&raw mut ctx as *mut (), 0,
            core::mem::size_of::<Fts5InsertCtx>() as u64)
    };
    ctx.p_storage = p;
    rc = sqlite3_fts5_storage_delete_all(p);
    if rc == 0 { rc = fts5_storage_load_totals(unsafe { &mut *p }, 1); }
    if rc == 0 {
        rc =
            fts5_storage_get_stmt(unsafe { &mut *p }, 11, &mut p_scan,
                unsafe { (*p_config).pz_errmsg });
    }
    while rc == 0 && 100 == unsafe { sqlite3_step(p_scan) } {
        let i_rowid: i64 = unsafe { sqlite3_column_int64(p_scan, 0) };
        unsafe { sqlite3_fts5_buffer_zero(&mut buf) };
        rc =
            unsafe {
                sqlite3_fts5_index_begin_write(unsafe { (*p).p_index }, 0,
                    i_rowid)
            };
        {
            ctx.i_col = 0;
            '__b18: loop {
                if !(rc == 0 && ctx.i_col < unsafe { (*p_config).n_col }) {
                    break '__b18;
                }
                '__c18: loop {
                    ctx.sz_col = 0;
                    if unsafe {
                                    *unsafe {
                                            (*p_config).ab_unindexed.offset(ctx.i_col as isize)
                                        }
                                } as i32 == 0 {
                        let mut n_text: i32 = 0;
                        let mut p_text: *const i8 = core::ptr::null();
                        let mut n_loc: i32 = 0;
                        let mut p_loc: *const i8 = core::ptr::null();
                        let p_val: *mut Sqlite3Value =
                            unsafe { sqlite3_column_value(p_scan, ctx.i_col + 1) };
                        if unsafe { (*p_config).e_content } == 2 &&
                                unsafe { sqlite3_fts5_is_locale_value(p_config, p_val) } !=
                                    0 {
                            rc =
                                unsafe {
                                    sqlite3_fts5_decode_locale_value(p_val, &mut p_text,
                                        &mut n_text, &mut p_loc, &mut n_loc)
                                };
                        } else {
                            p_text = unsafe { sqlite3_value_text(p_val) } as *const i8;
                            n_text = unsafe { sqlite3_value_bytes(p_val) };
                            if unsafe { (*p_config).b_locale } != 0 {
                                let i_col: i32 =
                                    ctx.i_col + 1 + unsafe { (*p_config).n_col };
                                p_loc =
                                    unsafe { sqlite3_column_text(p_scan, i_col) } as *const i8;
                                n_loc = unsafe { sqlite3_column_bytes(p_scan, i_col) };
                            }
                        }
                        if rc == 0 {
                            unsafe { sqlite3_fts5_set_locale(p_config, p_loc, n_loc) };
                            rc =
                                unsafe {
                                    sqlite3_fts5_tokenize(p_config, 4, p_text, n_text,
                                        &raw mut ctx as *mut (), Some(fts5_storage_insert_callback))
                                };
                            unsafe { sqlite3_fts5_clear_locale(p_config) };
                        }
                    }
                    unsafe {
                        sqlite3_fts5_buffer_append_varint(&mut rc, &mut buf,
                            ctx.sz_col as i64)
                    };
                    unsafe {
                        *unsafe { (*p).a_total_size.offset(ctx.i_col as isize) } +=
                            ctx.sz_col as i64
                    };
                    break '__c18;
                }
                { let __p = &mut ctx.i_col; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            let __p = unsafe { &mut (*p).n_total_row };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if rc == 0 { rc = fts5_storage_insert_docsize(p, i_rowid, &buf); }
    }
    unsafe { sqlite3_free(buf.p as *mut ()) };
    rc2 = unsafe { sqlite3_reset(p_scan) };
    if rc == 0 { rc = rc2; }
    if rc == 0 { rc = fts5_storage_save_totals(unsafe { &*p }); }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_optimize(p: &Fts5Storage) -> i32 {
    return unsafe { sqlite3_fts5_index_optimize((*p).p_index) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_merge(p: &Fts5Storage, n_merge: i32)
    -> i32 {
    return unsafe { sqlite3_fts5_index_merge((*p).p_index, n_merge) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_reset(p: &Fts5Storage) -> i32 {
    return unsafe { sqlite3_fts5_index_reset((*p).p_index) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_storage_release_delete_row(p_storage:
        &mut Fts5Storage) -> () {
    if !((*p_storage).p_saved_row == core::ptr::null_mut() ||
                            (*p_storage).p_saved_row == (*p_storage).a_stmt[3 as usize])
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5StorageReleaseDeleteRow".as_ptr() as
                    *const i8,
                c"fts5_storage.c".as_ptr() as *mut i8 as *const i8, 614,
                c"pStorage->pSavedRow==0 || pStorage->pSavedRow==pStorage->aStmt[FTS5_STMT_LOOKUP2]".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe { sqlite3_reset((*p_storage).p_saved_row) };
    (*p_storage).p_saved_row = core::ptr::null_mut();
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
    fn sqlite3_fts5_config_parse(_: *mut Fts5Global, _: *mut Sqlite3, _: i32,
    _: *mut *const i8, _: *mut *mut Fts5Config, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_config_free(_: *mut Fts5Config)
    -> ();
    fn sqlite3_fts5_config_declare_vtab(p_config_1: *mut Fts5Config)
    -> i32;
    fn sqlite3_fts5_tokenize(p_config_1: *mut Fts5Config, flags: i32,
    p_text_1: *const i8, n_text_1: i32, p_ctx_1: *mut (),
    x_token_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32)
            -> i32>)
    -> i32;
    fn sqlite3_fts5_dequote(z: *mut i8)
    -> ();
    fn sqlite3_fts5_config_load(_: *mut Fts5Config, _: i32)
    -> i32;
    fn sqlite3_fts5_config_set_value(_: *mut Fts5Config, _: *const i8,
    _: *mut Sqlite3Value, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_config_parse_rank(_: *const i8, _: *mut *mut i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_config_errmsg(p_config_1: *mut Fts5Config,
    z_fmt_1: *const i8, ...)
    -> ();
    fn sqlite3_fts5_buffer_size(_: *mut i32, _: *mut Fts5Buffer, _: u32)
    -> i32;
    fn sqlite3_fts5_buffer_append_varint(_: *mut i32, _: *mut Fts5Buffer,
    _: i64)
    -> ();
    fn sqlite3_fts5_buffer_append_blob(_: *mut i32, _: *mut Fts5Buffer,
    _: u32, _: *const u8)
    -> ();
    fn sqlite3_fts5_buffer_append_string(_: *mut i32, _: *mut Fts5Buffer,
    _: *const i8)
    -> ();
    fn sqlite3_fts5_buffer_free(_: *mut Fts5Buffer)
    -> ();
    fn sqlite3_fts5_buffer_zero(_: *mut Fts5Buffer)
    -> ();
    fn sqlite3_fts5_buffer_set(_: *mut i32, _: *mut Fts5Buffer, _: i32,
    _: *const u8)
    -> ();
    fn sqlite3_fts5_buffer_append_printf(_: *mut i32, _: *mut Fts5Buffer,
    z_fmt_1: *mut i8, ...)
    -> ();
    fn sqlite3_fts5_mprintf(p_rc_1: *mut i32, z_fmt_1: *const i8, ...)
    -> *mut i8;
    fn sqlite3_fts5_put32(_: *mut u8, _: i32)
    -> ();
    fn sqlite3_fts5_get32(_: *const u8)
    -> i32;
    fn sqlite3_fts5_poslist_reader_init(a: *const u8, n: i32,
    p_iter_1: *mut Fts5PoslistReader)
    -> i32;
    fn sqlite3_fts5_poslist_reader_next(_: *mut Fts5PoslistReader)
    -> i32;
    fn sqlite3_fts5_poslist_writer_append(_: *mut Fts5Buffer,
    _: *mut Fts5PoslistWriter, _: i64)
    -> i32;
    fn sqlite3_fts5_poslist_safe_append(_: *mut Fts5Buffer, _: *mut i64,
    _: i64)
    -> ();
    fn sqlite3_fts5_poslist_next64(a: *const u8, n: i32, pi: *mut i32,
    pi_off_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_malloc_zero(p_rc_1: *mut i32, n_byte_1: Sqlite3Int64)
    -> *mut ();
    fn sqlite3_fts5_strndup(p_rc_1: *mut i32, p_in_1: *const i8, n_in_1: i32)
    -> *mut i8;
    fn sqlite3_fts5_is_bareword(t: i8)
    -> i32;
    fn sqlite3_fts5_termset_new(_: *mut *mut Fts5Termset)
    -> i32;
    fn sqlite3_fts5_termset_add(_: *mut Fts5Termset, _: i32, _: *const i8,
    _: i32, pb_present_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_termset_free(_: *mut Fts5Termset)
    -> ();
    fn sqlite3_fts5_index_open(p_config_1: *mut Fts5Config, b_create_1: i32,
    _: *mut *mut Fts5Index, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_index_close(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_entry_cksum(i_rowid_1: i64, i_col_1: i32,
    i_pos_1: i32, i_idx_1: i32, p_term_1: *const i8, n_term_1: i32)
    -> u64;
    fn sqlite3_fts5_index_charlen_to_bytelen(p: *const i8, n_byte_1: i32,
    n_char_1: i32)
    -> i32;
    fn sqlite3_fts5_index_query(p: *mut Fts5Index, p_token_1: *const i8,
    n_token_1: i32, flags: i32, p_colset_1: *mut Fts5Colset,
    pp_iter_1: *mut *mut Fts5IndexIter)
    -> i32;
    fn sqlite3_fts5_iter_next(_: *mut Fts5IndexIter)
    -> i32;
    fn sqlite3_fts5_iter_next_from(_: *mut Fts5IndexIter, i_match_1: i64)
    -> i32;
    fn sqlite3_fts5_iter_close(_: *mut Fts5IndexIter)
    -> ();
    fn sqlite3_fts5_index_close_reader(_: *mut Fts5Index)
    -> ();
    fn sqlite3_fts5_iter_term(_: *mut Fts5IndexIter, _: *mut i32)
    -> *const i8;
    fn sqlite3_fts5_iter_next_scan(_: *mut Fts5IndexIter)
    -> i32;
    fn sqlite3_fts5_structure_ref(_: *mut Fts5Index)
    -> *mut ();
    fn sqlite3_fts5_structure_release(_: *mut ())
    -> ();
    fn sqlite3_fts5_structure_test(_: *mut Fts5Index, _: *mut ())
    -> i32;
    fn sqlite3_fts5_iter_token(p_index_iter_1: *mut Fts5IndexIter,
    p_token_1: *const i8, n_token_1: i32, i_rowid_1: i64, i_col_1: i32,
    i_off_1: i32, pp_out_1: *mut *const i8, pn_out_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_index_write(p: *mut Fts5Index, i_col_1: i32, i_pos_1: i32,
    p_token_1: *const i8, n_token_1: i32)
    -> i32;
    fn sqlite3_fts5_index_begin_write(p: *mut Fts5Index, b_delete_1: i32,
    i_docid_1: i64)
    -> i32;
    fn sqlite3_fts5_index_sync(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_rollback(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_get_averages(p: *mut Fts5Index, pn_row_1: *mut i64,
    an_size_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_index_set_averages(p: *mut Fts5Index, _: *const u8,
    _: i32)
    -> i32;
    fn sqlite3_fts5_index_integrity_check(_: *mut Fts5Index, cksum: u64,
    b_use_cksum_1: i32)
    -> i32;
    fn sqlite3_fts5_index_init(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_fts5_index_set_cookie(_: *mut Fts5Index, _: i32)
    -> i32;
    fn sqlite3_fts5_index_reads(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_reinit(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_optimize(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_merge(p: *mut Fts5Index, n_merge_1: i32)
    -> i32;
    fn sqlite3_fts5_index_reset(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_load_config(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_get_origin(p: *mut Fts5Index, pi_origin_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_index_contentless_delete(p: *mut Fts5Index,
    i_origin_1: i64, i_rowid_1: i64)
    -> i32;
    fn sqlite3_fts5_index_iter_clear_tokendata(_: *mut Fts5IndexIter)
    -> ();
    fn sqlite3_fts5_index_iter_write_tokendata(_: *mut Fts5IndexIter,
    _: *const i8, _: i32, i_rowid_1: i64, i_col_1: i32, i_off_1: i32)
    -> i32;
    fn sqlite3_fts5_get_varint32(p: *const u8, v: *mut u32)
    -> i32;
    fn sqlite3_fts5_get_varint_len(i_val_1: u32)
    -> i32;
    fn sqlite3_fts5_get_varint(_: *const u8, _: *mut u64)
    -> u8;
    fn sqlite3_fts5_put_varint(p: *mut u8, v: u64)
    -> i32;
    fn sqlite3_fts5_load_tokenizer(p_config_1: *mut Fts5Config)
    -> i32;
    fn sqlite3_fts5_table_from_csrid(_: *mut Fts5Global, _: i64)
    -> *mut Fts5Table;
    fn sqlite3_fts5_flush_to_disk(_: *mut Fts5Table)
    -> i32;
    fn sqlite3_fts5_clear_locale(p_config_1: *mut Fts5Config)
    -> ();
    fn sqlite3_fts5_set_locale(p_config_1: *mut Fts5Config,
    p_loc_1: *const i8, n_loc_1: i32)
    -> ();
    fn sqlite3_fts5_is_locale_value(p_config_1: *mut Fts5Config,
    p_val_1: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_fts5_decode_locale_value(p_val_1: *mut Sqlite3Value,
    pp_text_1: *mut *const i8, pn_text_1: *mut i32, pp_loc_1: *mut *const i8,
    pn_loc_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_hash_new(_: *mut Fts5Config, _: *mut *mut Fts5Hash,
    pn_size_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_hash_free(_: *mut Fts5Hash)
    -> ();
    fn sqlite3_fts5_hash_write(_: *mut Fts5Hash, i_rowid_1: i64, i_col_1: i32,
    i_pos_1: i32, b_byte_1: i8, p_token_1: *const i8, n_token_1: i32)
    -> i32;
    fn sqlite3_fts5_hash_clear(_: *mut Fts5Hash)
    -> ();
    fn sqlite3_fts5_hash_is_empty(_: *mut Fts5Hash)
    -> i32;
    fn sqlite3_fts5_hash_query(_: *mut Fts5Hash, n_pre_1: i32,
    p_term_1: *const i8, n_term_1: i32, pp_obj_1: *mut *mut (),
    pn_doclist_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_hash_scan_init(_: *mut Fts5Hash, p_term_1: *const i8,
    n_term_1: i32)
    -> i32;
    fn sqlite3_fts5_hash_scan_next(_: *mut Fts5Hash)
    -> ();
    fn sqlite3_fts5_hash_scan_eof(_: *mut Fts5Hash)
    -> i32;
    fn sqlite3_fts5_hash_scan_entry(_: *mut Fts5Hash,
    pz_term_1: *mut *const i8, pn_term_1: *mut i32,
    pp_doclist_1: *mut *const u8, pn_doclist_1: *mut i32)
    -> ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_fts5_expr_new(p_config_1: *mut Fts5Config,
    b_phrase_to_and_1: i32, i_col_1: i32, z_expr_1: *const i8,
    pp_new_1: *mut *mut Fts5Expr, pz_err_1: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_expr_pattern(p_config_1: *mut Fts5Config, b_glob_1: i32,
    i_col_1: i32, z_text_1: *const i8, pp: *mut *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_first(_: *mut Fts5Expr, p_idx_1: *mut Fts5Index,
    i_min_1: i64, _: i64, b_desc_1: i32)
    -> i32;
    fn sqlite3_fts5_expr_next(_: *mut Fts5Expr, i_max_1: i64)
    -> i32;
    fn sqlite3_fts5_expr_eof(_: *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_rowid(_: *mut Fts5Expr)
    -> i64;
    fn sqlite3_fts5_expr_free(_: *mut Fts5Expr)
    -> ();
    fn sqlite3_fts5_expr_and(pp1: *mut *mut Fts5Expr, p2: *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_init(_: *mut Fts5Global, _: *mut Sqlite3)
    -> i32;
    fn sqlite3_fts5_expr_phrase_count(_: *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_phrase_size(_: *mut Fts5Expr, i_phrase_1: i32)
    -> i32;
    fn sqlite3_fts5_expr_poslist(_: *mut Fts5Expr, _: i32, _: *mut *const u8)
    -> i32;
    fn sqlite3_fts5_expr_clear_poslists(_: *mut Fts5Expr, _: i32)
    -> *mut Fts5PoslistPopulator;
    fn sqlite3_fts5_expr_populate_poslists(_: *mut Fts5Config,
    _: *mut Fts5Expr, _: *mut Fts5PoslistPopulator, _: i32, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_fts5_expr_check_poslists(_: *mut Fts5Expr, _: i64)
    -> ();
    fn sqlite3_fts5_expr_clone_phrase(_: *mut Fts5Expr, _: i32,
    _: *mut *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_phrase_collist(_: *mut Fts5Expr, _: i32,
    _: *mut *const u8, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_expr_query_token(_: *mut Fts5Expr, _: i32, _: i32,
    _: *mut *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_expr_inst_token(_: *mut Fts5Expr, _: i64, _: i32, _: i32,
    _: i32, _: i32, _: *mut *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_expr_clear_tokens(_: *mut Fts5Expr)
    -> ();
    fn sqlite3_fts5_parse_error(p_parse_1: *mut Fts5Parse, z_fmt_1: *const i8,
    ...)
    -> ();
    fn sqlite3_fts5_parse_node(p_parse_1: *mut Fts5Parse, e_type_1: i32,
    p_left_1: *mut Fts5ExprNode, p_right_1: *mut Fts5ExprNode,
    p_near_1: *mut Fts5ExprNearset)
    -> *mut Fts5ExprNode;
    fn sqlite3_fts5_parse_implicit_and(p_parse_1: *mut Fts5Parse,
    p_left_1: *mut Fts5ExprNode, p_right_1: *mut Fts5ExprNode)
    -> *mut Fts5ExprNode;
    fn sqlite3_fts5_parse_term(p_parse_1: *mut Fts5Parse,
    p_phrase_1: *mut Fts5ExprPhrase, p_token_1: *mut Fts5Token,
    b_prefix_1: i32)
    -> *mut Fts5ExprPhrase;
    fn sqlite3_fts5_parse_set_caret(_: *mut Fts5ExprPhrase)
    -> ();
    fn sqlite3_fts5_parse_nearset(_: *mut Fts5Parse, _: *mut Fts5ExprNearset,
    _: *mut Fts5ExprPhrase)
    -> *mut Fts5ExprNearset;
    fn sqlite3_fts5_parse_colset(_: *mut Fts5Parse, _: *mut Fts5Colset,
    _: *mut Fts5Token)
    -> *mut Fts5Colset;
    fn sqlite3_fts5_parse_phrase_free(_: *mut Fts5ExprPhrase)
    -> ();
    fn sqlite3_fts5_parse_nearset_free(_: *mut Fts5ExprNearset)
    -> ();
    fn sqlite3_fts5_parse_node_free(_: *mut Fts5ExprNode)
    -> ();
    fn sqlite3_fts5_parse_set_distance(_: *mut Fts5Parse,
    _: *mut Fts5ExprNearset, _: *mut Fts5Token)
    -> ();
    fn sqlite3_fts5_parse_set_colset(_: *mut Fts5Parse, _: *mut Fts5ExprNode,
    _: *mut Fts5Colset)
    -> ();
    fn sqlite3_fts5_parse_colset_invert(_: *mut Fts5Parse, _: *mut Fts5Colset)
    -> *mut Fts5Colset;
    fn sqlite3_fts5_parse_finished(p_parse_1: *mut Fts5Parse,
    p: *mut Fts5ExprNode)
    -> ();
    fn sqlite3_fts5_parse_near(p_parse_1: *mut Fts5Parse, _: *mut Fts5Token)
    -> ();
    fn sqlite3_fts5_aux_init(_: *mut Fts5Api)
    -> i32;
    fn sqlite3_fts5_tokenizer_init(_: *mut Fts5Api)
    -> i32;
    fn sqlite3_fts5_tokenizer_pattern(x_create_1:
        Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
            *mut *mut Fts5Tokenizer) -> i32>, p_tok_1: *mut Fts5Tokenizer)
    -> i32;
    fn sqlite3_fts5_tokenizer_preload(_: *mut Fts5TokenizerConfig)
    -> i32;
    fn sqlite3_fts5_vocab_init(_: *mut Fts5Global, _: *mut Sqlite3)
    -> i32;
    fn sqlite3_fts5_unicode_isdiacritic(c: i32)
    -> i32;
    fn sqlite3_fts5_unicode_fold(c: i32, b_remove_diacritic_1: i32)
    -> i32;
    fn sqlite3_fts5_unicode_cat_parse(_: *const i8, _: *mut u8)
    -> i32;
    fn sqlite3_fts5_unicode_category(i_code_1: u32)
    -> i32;
    fn sqlite3_fts5_unicode_ascii(_: *mut u8, _: *mut u8)
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
