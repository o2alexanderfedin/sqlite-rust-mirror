#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
type Int32T = i32;
type DarwinPidT = Int32T;
type PidT = DarwinPidT;
type DarwinSizeT = u64;
extern "C" fn get_process_id() -> i32 { return unsafe { getpid() } as i32; }
#[repr(C)]
#[derive(Copy, Clone)]
struct SLConn {
    is_err: i32,
    db: *mut Sqlite3,
    i_log: i32,
    fd: *mut FILE,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SLGlobal {
    mutex: *mut Sqlite3Mutex,
    n_conn: i32,
    b_conditional: i32,
    b_reuse: i32,
    z_prefix: [i8; 512],
    z_idx: [i8; 512],
    i_next_log: i32,
    i_next_db: i32,
    b_rec: i32,
    i_clock: i32,
    a_conn: [SLConn; 256],
}
static mut sqllogglobal: SLGlobal = unsafe { core::mem::zeroed() };
extern "C" fn sqllog_isspace(c: i8) -> i32 {
    return (c as i32 == ' ' as i32 || c as i32 == '\t' as i32 ||
                            c as i32 == '\n' as i32 || c as i32 == '\u{b}' as i32 ||
                    c as i32 == '\u{c}' as i32 || c as i32 == '\r' as i32) as
            i32;
}
extern "C" fn sqllog_tokenize(z: *const i8, pz: &mut *const i8, pn: &mut i32)
    -> () {
    let mut p: *const i8 = z;
    let mut n: i32 = 0;
    while sqllog_isspace(unsafe { *p }) != 0 {
        {
            let __p = &mut p;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    *pz = p;
    n = 0;
    while unsafe { *p.offset(n as isize) } as i32 >= 'a' as i32 &&
                unsafe { *p.offset(n as isize) } as i32 <= 'z' as i32 ||
            unsafe { *p.offset(n as isize) } as i32 >= 'A' as i32 &&
                unsafe { *p.offset(n as isize) } as i32 <= 'Z' as i32 {
        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
    }
    *pn = n;
}
extern "C" fn sqllog_find_file(z_file_1: *const i8) -> *mut i8 {
    unsafe {
        let mut z_ret: *mut i8 = core::ptr::null_mut();
        let mut fd: *mut FILE = core::ptr::null_mut();
        fd =
            unsafe {
                fopen(&raw mut sqllogglobal.z_idx[0 as usize] as *mut i8 as
                        *const i8, c"r".as_ptr() as *mut i8 as *const i8)
            };
        if fd == core::ptr::null_mut() {
            unsafe {
                sqlite3_log(10,
                    c"sqllogFindFile(): error in fopen()".as_ptr() as *mut i8 as
                        *const i8)
            };
            return core::ptr::null_mut();
        }
        while unsafe { feof(fd) } == 0 {
            let mut z_line: [i8; 1029] = [0; 1029];
            if !(unsafe {
                                fgets(&raw mut z_line[0 as usize] as *mut i8,
                                    core::mem::size_of::<[i8; 1029]>() as i32, fd)
                            }).is_null() {
                let mut n: i32 = 0;
                let mut z: *const i8 = core::ptr::null();
                z_line[(core::mem::size_of::<[i8; 1029]>() as u64 - 1 as u64)
                            as usize] = '\u{0}' as i32 as i8;
                z = &raw mut z_line[0 as usize] as *mut i8;
                while unsafe { *z } as i32 >= '0' as i32 &&
                        unsafe { *z } as i32 <= '9' as i32 {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                while unsafe { *z } as i32 == ' ' as i32 {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                n = unsafe { strlen(z as *const i8) } as i32;
                while n > 0 &&
                        sqllog_isspace(unsafe { *z.offset((n - 1) as isize) }) != 0
                    {
                    { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                }
                if n as u64 == unsafe { strlen(z_file_1) } &&
                        0 ==
                            unsafe {
                                memcmp(z_file_1 as *const (), z as *const (), n as u64)
                            } {
                    let mut z_buf: [i8; 16] = [0; 16];
                    unsafe {
                        memset(&raw mut z_buf[0 as usize] as *mut i8 as *mut (), 0,
                            core::mem::size_of::<[i8; 16]>() as u64)
                    };
                    z = &raw mut z_line[0 as usize] as *mut i8;
                    while unsafe { *z } as i32 >= '0' as i32 &&
                            unsafe { *z } as i32 <= '9' as i32 {
                        z_buf[unsafe {
                                            z.offset_from(&raw mut z_line[0 as usize] as *mut i8)
                                        } as i64 as usize] = unsafe { *z };
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    z_ret =
                        unsafe {
                            sqlite3_mprintf(c"%s_%s.db".as_ptr() as *mut i8 as
                                    *const i8,
                                &raw mut sqllogglobal.z_prefix[0 as usize] as *mut i8,
                                &raw mut z_buf[0 as usize] as *mut i8)
                        };
                    break;
                }
            }
        }
        if unsafe { ferror(fd) } != 0 {
            unsafe {
                sqlite3_log(10,
                    c"sqllogFindFile(): error reading index file".as_ptr() as
                            *mut i8 as *const i8)
            };
        }
        unsafe { fclose(fd) };
        return z_ret;
    }
}
extern "C" fn sqllog_find_attached(db: *mut Sqlite3, z_search_1: *const i8,
    z_name_1: *mut i8, z_file_1: *mut i8) -> i32 {
    unsafe {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        if !(sqllogglobal.b_rec == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqllogFindAttached".as_ptr() as *const i8,
                    c"test_sqllog.c".as_ptr() as *mut i8 as *const i8, 237,
                    c"sqllogglobal.bRec==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        sqllogglobal.b_rec = 1;
        rc =
            unsafe {
                sqlite3_prepare_v2(db,
                    c"PRAGMA database_list".as_ptr() as *mut i8 as *const i8,
                    -1, &mut p_stmt, core::ptr::null_mut())
            };
        if rc == 0 {
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                let mut z_val1: *const i8 = core::ptr::null();
                let mut n_val1: i32 = 0;
                let mut z_val2: *const i8 = core::ptr::null();
                let mut n_val2: i32 = 0;
                z_val1 =
                    unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
                n_val1 = unsafe { sqlite3_column_bytes(p_stmt, 1) };
                if !(z_name_1).is_null() {
                    unsafe {
                        memcpy(z_name_1 as *mut (), z_val1 as *const (),
                            (n_val1 + 1) as u64)
                    };
                }
                z_val2 =
                    unsafe { sqlite3_column_text(p_stmt, 2) } as *const i8;
                n_val2 = unsafe { sqlite3_column_bytes(p_stmt, 2) };
                unsafe {
                    memcpy(z_file_1 as *mut (), z_val2 as *const (),
                        (n_val2 + 1) as u64)
                };
                if !(z_search_1).is_null() &&
                            unsafe { strlen(z_search_1) } == n_val1 as u64 &&
                        0 == unsafe { sqlite3_strnicmp(z_search_1, z_val1, n_val1) }
                    {
                    break;
                }
            }
            rc = unsafe { sqlite3_finalize(p_stmt) };
        }
        sqllogglobal.b_rec = 0;
        if rc != 0 {
            unsafe {
                sqlite3_log(rc,
                    c"sqllogFindAttached(): error in \"PRAGMA database_list\"".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        return rc;
    }
}
extern "C" fn sqllog_copydb(p: &SLConn, z_search_1: *const i8, b_log_1: i32)
    -> () {
    unsafe {
        let mut z_name: [i8; 512] = [0; 512];
        let mut z_file: [i8; 512] = [0; 512];
        let mut z_free: *mut i8 = core::ptr::null_mut();
        let mut z_init: *mut i8 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        rc =
            sqllog_find_attached((*p).db, z_search_1,
                &raw mut z_name[0 as usize] as *mut i8,
                &raw mut z_file[0 as usize] as *mut i8);
        if rc != 0 { return; }
        if z_file[0 as usize] as i32 == '\u{0}' as i32 {
            z_init =
                unsafe {
                    sqlite3_mprintf(c"".as_ptr() as *mut i8 as *const i8)
                };
        } else {
            if sqllogglobal.b_reuse != 0 {
                z_init =
                    sqllog_find_file(&raw mut z_file[0 as usize] as *mut i8 as
                            *const i8);
            } else { z_init = core::ptr::null_mut(); }
            if z_init == core::ptr::null_mut() {
                let mut rc: i32 = 0;
                let mut copy: *mut Sqlite3 = core::ptr::null_mut();
                let mut i_db: i32 = 0;
                i_db =
                    {
                        let __p = &mut sqllogglobal.i_next_db;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                z_init =
                    unsafe {
                        sqlite3_mprintf(c"%s_%02d.db".as_ptr() as *mut i8 as
                                *const i8,
                            &raw mut sqllogglobal.z_prefix[0 as usize] as *mut i8, i_db)
                    };
                if !(sqllogglobal.b_rec == 0) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"sqllogCopydb".as_ptr() as *const i8,
                            c"test_sqllog.c".as_ptr() as *mut i8 as *const i8, 319,
                            c"sqllogglobal.bRec==0".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                sqllogglobal.b_rec = 1;
                rc = unsafe { sqlite3_open(z_init as *const i8, &mut copy) };
                if rc == 0 {
                    let mut p_bak: *mut Sqlite3Backup = core::ptr::null_mut();
                    unsafe {
                        sqlite3_exec(copy,
                            c"PRAGMA synchronous = 0".as_ptr() as *mut i8 as *const i8,
                            None, core::ptr::null_mut(), core::ptr::null_mut())
                    };
                    p_bak =
                        unsafe {
                            sqlite3_backup_init(copy,
                                c"main".as_ptr() as *mut i8 as *const i8, (*p).db,
                                &raw mut z_name[0 as usize] as *mut i8 as *const i8)
                        };
                    if !(p_bak).is_null() {
                        unsafe { sqlite3_backup_step(p_bak, -1) };
                        rc = unsafe { sqlite3_backup_finish(p_bak) };
                    } else { rc = unsafe { sqlite3_errcode(copy) }; }
                    unsafe { sqlite3_close(copy) };
                }
                sqllogglobal.b_rec = 0;
                if rc == 0 {
                    let fd: *mut FILE =
                        unsafe {
                            fopen(&raw mut sqllogglobal.z_idx[0 as usize] as *mut i8 as
                                    *const i8, c"a".as_ptr() as *mut i8 as *const i8)
                        };
                    if !(fd).is_null() {
                        unsafe {
                            fprintf(fd, c"%d %s\n".as_ptr() as *mut i8 as *const i8,
                                i_db, &raw mut z_file[0 as usize] as *mut i8)
                        };
                        unsafe { fclose(fd) };
                    }
                } else {
                    unsafe {
                        sqlite3_log(rc,
                            c"sqllogCopydb(): error backing up database".as_ptr() as
                                    *mut i8 as *const i8)
                    };
                }
            }
        }
        if b_log_1 != 0 {
            z_free =
                unsafe {
                    sqlite3_mprintf(c"ATTACH \'%q\' AS \'%q\'; -- clock=%d\n".as_ptr()
                                as *mut i8 as *const i8, z_init,
                        &raw mut z_name[0 as usize] as *mut i8,
                        {
                            let __p = &mut sqllogglobal.i_clock;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        })
                };
        } else {
            z_free =
                unsafe {
                    sqlite3_mprintf(c"-- Main database is \'%q\'\n".as_ptr() as
                                *mut i8 as *const i8, z_init)
                };
        }
        unsafe {
            fprintf((*p).fd, c"%s".as_ptr() as *mut i8 as *const i8, z_free)
        };
        unsafe { sqlite3_free(z_free as *mut ()) };
        unsafe { sqlite3_free(z_init as *mut ()) };
    }
}
extern "C" fn sqllog_openlog(p: &mut SLConn) -> () {
    unsafe {
        if (*p).fd == core::ptr::null_mut() {
            let mut z_log: *mut i8 = core::ptr::null_mut();
            if sqllogglobal.z_prefix[0 as usize] as i32 == 0 {
                let mut fd: *mut FILE = core::ptr::null_mut();
                let z_var: *mut i8 =
                    unsafe {
                        getenv(c"SQLITE_SQLLOG_DIR".as_ptr() as *mut i8 as
                                *const i8)
                    };
                if z_var == core::ptr::null_mut() ||
                        unsafe { strlen(z_var as *const i8) } + 10 as u64 >=
                            core::mem::size_of::<[i8; 512]>() as u64 {
                    return;
                }
                unsafe {
                    sqlite3_snprintf(core::mem::size_of::<[i8; 512]>() as i32,
                        &raw mut sqllogglobal.z_prefix[0 as usize] as *mut i8,
                        c"%s/sqllog_%05d".as_ptr() as *mut i8 as *const i8, z_var,
                        get_process_id())
                };
                unsafe {
                    sqlite3_snprintf(core::mem::size_of::<[i8; 512]>() as i32,
                        &raw mut sqllogglobal.z_idx[0 as usize] as *mut i8,
                        c"%s.idx".as_ptr() as *mut i8 as *const i8,
                        &raw mut sqllogglobal.z_prefix[0 as usize] as *mut i8)
                };
                if !(unsafe {
                                    getenv(c"SQLITE_SQLLOG_REUSE_FILES".as_ptr() as *mut i8 as
                                            *const i8)
                                }).is_null() {
                    sqllogglobal.b_reuse =
                        unsafe {
                            atoi(unsafe {
                                        getenv(c"SQLITE_SQLLOG_REUSE_FILES".as_ptr() as *mut i8 as
                                                *const i8)
                                    } as *const i8)
                        };
                }
                fd =
                    unsafe {
                        fopen(&raw mut sqllogglobal.z_idx[0 as usize] as *mut i8 as
                                *const i8, c"w".as_ptr() as *mut i8 as *const i8)
                    };
                if !(fd).is_null() { unsafe { fclose(fd) }; }
            }
            z_log =
                unsafe {
                    sqlite3_mprintf(c"%s_%05d.sql".as_ptr() as *mut i8 as
                            *const i8,
                        &raw mut sqllogglobal.z_prefix[0 as usize] as *mut i8,
                        (*p).i_log)
                };
            (*p).fd =
                unsafe {
                    fopen(z_log as *const i8,
                        c"w".as_ptr() as *mut i8 as *const i8)
                };
            unsafe { sqlite3_free(z_log as *mut ()) };
            if (*p).fd == core::ptr::null_mut() {
                unsafe {
                    sqlite3_log(10,
                        c"sqllogOpenlog(): Failed to open log file".as_ptr() as
                                *mut i8 as *const i8)
                };
            }
        }
    }
}
extern "C" fn test_sqllog_stmt(p: *mut SLConn, z_sql_1: *const i8) -> () {
    unsafe {
        let mut z_first: *const i8 = core::ptr::null();
        let mut n_first: i32 = 0;
        sqllog_tokenize(z_sql_1, &mut z_first, &mut n_first);
        if n_first != 6 ||
                0 !=
                    unsafe {
                        sqlite3_strnicmp(c"ATTACH".as_ptr() as *mut i8 as *const i8,
                            z_first, 6)
                    } {
            unsafe {
                fprintf(unsafe { (*p).fd },
                    c"%s; -- clock=%d\n".as_ptr() as *mut i8 as *const i8,
                    z_sql_1,
                    {
                        let __p = &mut sqllogglobal.i_clock;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    })
            };
        } else { sqllog_copydb(unsafe { &*p }, core::ptr::null(), 1); }
    }
}
extern "C" fn sqllog_trace_db(db: *mut Sqlite3) -> i32 {
    unsafe {
        let mut b_ret: i32 = 1;
        if sqllogglobal.b_conditional != 0 {
            let mut z_file: [i8; 512] = [0; 512];
            let rc: i32 =
                sqllog_find_attached(db,
                    c"main".as_ptr() as *mut i8 as *const i8,
                    core::ptr::null_mut(),
                    &raw mut z_file[0 as usize] as *mut i8);
            if rc == 0 {
                let n_file: i32 =
                    unsafe {
                            strlen(&raw mut z_file[0 as usize] as *mut i8 as *const i8)
                        } as i32;
                if 512 - n_file < 8 {
                    unsafe {
                        sqlite3_log(10,
                            c"sqllogTraceDb(): database name too long (%d bytes)".as_ptr()
                                    as *mut i8 as *const i8, n_file)
                    };
                    b_ret = 0;
                } else {
                    unsafe {
                        memcpy(&raw mut z_file[n_file as usize] as *mut (),
                            c"-sqllog".as_ptr() as *mut i8 as *const (), 8 as u64)
                    };
                    b_ret =
                        (unsafe {
                                        access(&raw mut z_file[0 as usize] as *mut i8 as *const i8,
                                            0)
                                    } == 0) as i32 as i32;
                }
            }
        }
        return b_ret;
    }
}
extern "C" fn test_sqllog(p_ctx_1: *mut (), db: *mut Sqlite3,
    z_sql_1: *const i8, e_type_1: i32) -> () {
    unsafe {
        let mut p: *mut SLConn = core::ptr::null_mut();
        let mainmtx: *mut Sqlite3Mutex = unsafe { sqlite3_mutex_alloc(2) };
        if !(e_type_1 == 0 || e_type_1 == 1 || e_type_1 == 2) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"testSqllog".as_ptr() as *const i8,
                    c"test_sqllog.c".as_ptr() as *mut i8 as *const i8, 472,
                    c"eType==0 || eType==1 || eType==2".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !((e_type_1 == 2) as i32 == (z_sql_1 == core::ptr::null()) as i32)
                        as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"testSqllog".as_ptr() as *const i8,
                    c"test_sqllog.c".as_ptr() as *mut i8 as *const i8, 473,
                    c"(eType==2)==(zSql==0)".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if e_type_1 == 0 {
            unsafe { sqlite3_mutex_enter(mainmtx) };
            if sqllogglobal.mutex == core::ptr::null_mut() {
                sqllogglobal.mutex = unsafe { sqlite3_mutex_alloc(1) };
            }
            unsafe { sqlite3_mutex_leave(mainmtx) };
            unsafe { sqlite3_mutex_enter(sqllogglobal.mutex) };
            if sqllogglobal.b_rec == 0 && sqllog_trace_db(db) != 0 {
                unsafe { sqlite3_mutex_enter(mainmtx) };
                p =
                    &mut sqllogglobal.a_conn[{
                                    let __p = &mut sqllogglobal.n_conn;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize];
                unsafe { (*p).fd = core::ptr::null_mut() };
                unsafe { (*p).db = db };
                unsafe {
                    (*p).i_log =
                        {
                            let __p = &mut sqllogglobal.i_next_log;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        }
                };
                unsafe { sqlite3_mutex_leave(mainmtx) };
                sqllog_openlog(unsafe { &mut *p });
                if !(unsafe { (*p).fd }).is_null() {
                    sqllog_copydb(unsafe { &*p },
                        c"main".as_ptr() as *mut i8 as *const i8, 0);
                }
            }
            unsafe { sqlite3_mutex_leave(sqllogglobal.mutex) };
        } else {
            let mut i: i32 = 0;
            {
                i = 0;
                '__b8: loop {
                    if !(i < sqllogglobal.n_conn) { break '__b8; }
                    '__c8: loop {
                        p = &mut sqllogglobal.a_conn[i as usize];
                        if unsafe { (*p).db } == db { break '__b8; }
                        break '__c8;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if e_type_1 == 2 {
                unsafe { sqlite3_mutex_enter(mainmtx) };
                if i < sqllogglobal.n_conn {
                    if !(unsafe { (*p).fd }).is_null() {
                        unsafe { fclose(unsafe { (*p).fd }) };
                    }
                    unsafe { (*p).db = core::ptr::null_mut() };
                    unsafe { (*p).fd = core::ptr::null_mut() };
                    {
                        let __p = &mut sqllogglobal.n_conn;
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                }
                if sqllogglobal.n_conn == 0 {
                    unsafe { sqlite3_mutex_free(sqllogglobal.mutex) };
                    sqllogglobal.mutex = core::ptr::null_mut();
                } else if i < sqllogglobal.n_conn {
                    let n_shift: i32 =
                        unsafe {
                                    (&raw mut sqllogglobal.a_conn[sqllogglobal.n_conn as usize]
                                            as *mut SLConn).offset_from(p)
                                } as i64 as i32;
                    if n_shift > 0 {
                        unsafe {
                            memmove(p as *mut (),
                                unsafe { &raw mut *p.offset(1 as isize) } as *const (),
                                n_shift as u64 * core::mem::size_of::<SLConn>() as u64)
                        };
                    }
                }
                unsafe { sqlite3_mutex_leave(mainmtx) };
            } else if i < sqllogglobal.n_conn &&
                    !(unsafe { (*p).fd }).is_null() {
                unsafe { sqlite3_mutex_enter(sqllogglobal.mutex) };
                if sqllogglobal.b_rec == 0 { test_sqllog_stmt(p, z_sql_1); }
                unsafe { sqlite3_mutex_leave(sqllogglobal.mutex) };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_init_sqllog() -> () {
    unsafe {
        if !(unsafe {
                            getenv(c"SQLITE_SQLLOG_DIR".as_ptr() as *mut i8 as
                                    *const i8)
                        }).is_null() {
            if 0 ==
                    unsafe {
                        sqlite3_config(21,
                            unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32)
                                            -> ()>(test_sqllog as *const ())
                            }, 0)
                    } {
                unsafe {
                    memset(&raw mut sqllogglobal as *mut (), 0,
                        core::mem::size_of::<SLGlobal>() as u64)
                };
                sqllogglobal.b_reuse = 1;
                if !(unsafe {
                                    getenv(c"SQLITE_SQLLOG_CONDITIONAL".as_ptr() as *mut i8 as
                                            *const i8)
                                }).is_null() {
                    sqllogglobal.b_conditional = 1;
                }
            }
        }
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
    fn getpid()
    -> PidT;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn feof(_: *mut FILE)
    -> i32;
    fn fgets(_: *mut i8, __size: i32, _: *mut FILE)
    -> *mut i8;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn ferror(_: *mut FILE)
    -> i32;
    fn fclose(_: *mut FILE)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn getenv(_: *const i8)
    -> *mut i8;
    fn atoi(_: *const i8)
    -> i32;
    fn access(_: *const i8, _: i32)
    -> i32;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;