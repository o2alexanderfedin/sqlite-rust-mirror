#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3ApiRoutines, Sqlite3Backup, Sqlite3Blob, Sqlite3Context,
    Sqlite3File, Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64,
    Sqlite3IoMethods, Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry,
    Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str,
    Sqlite3Uint64, Sqlite3Value, Sqlite3Vfs,
};

type DarwinSizeT = u64;

///* Main recover handle structure.
#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3Recover {
    db_in: *mut Sqlite3,
    z_db: *mut i8,
    z_uri: *mut i8,
    p_sql_ctx: *mut (),
    x_sql: Option<unsafe extern "C" fn(*mut (), *const i8) -> i32>,
    z_state_db: *mut i8,
    z_lost_and_found: *mut i8,
    b_freelist_corrupt: i32,
    b_recover_rowid: i32,
    b_slow_indexes: i32,
    pgsz: i32,
    detected_pgsz: i32,
    n_reserve: i32,
    p_page1_disk: *mut u8,
    p_page1_cache: *mut u8,
    err_code: i32,
    z_err_msg: *mut i8,
    e_state: i32,
    b_close_transaction: i32,
    w1: RecoverStateW1,
    laf: RecoverStateLAF,
    db_out: *mut Sqlite3,
    p_get_page: *mut Sqlite3Stmt,
    p_tbl_list: *mut RecoverTable,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RecoverStateW1 {
    p_tbls: *mut Sqlite3Stmt,
    p_sel: *mut Sqlite3Stmt,
    p_insert: *mut Sqlite3Stmt,
    n_insert: i32,
    p_tab: *mut RecoverTable,
    n_max: i32,
    ap_val: *mut *mut Sqlite3Value,
    n_val: i32,
    b_have_rowid: i32,
    i_rowid: i64,
    i_prev_page: i64,
    i_prev_cell: i32,
}

///* When recovering rows of data that can be associated with table
///* definitions recovered from the sqlite_schema table, each table is
///* represented by an instance of the following object.
///*
///* iRoot:
///*   The root page in the original database. Not necessarily (and usually
///*   not) the same in the recovered database.
///*
///* zTab:
///*   Name of the table.
///*
///* nCol/aCol[]:
///*   aCol[] is an array of nCol columns. In the order in which they appear 
///*   in the table.
///*
///* bIntkey:
///*   Set to true for intkey tables, false for WITHOUT ROWID.
///*
///* iRowidBind:
///*   Each column in the aCol[] array has associated with it the index of
///*   the bind parameter its values will be bound to in the INSERT statement
///*   used to construct the output database. If the table does has a rowid
///*   but not an INTEGER PRIMARY KEY column, then iRowidBind contains the
///*   index of the bind paramater to which the rowid value should be bound.
///*   Otherwise, it contains -1. If the table does contain an INTEGER PRIMARY 
///*   KEY column, then the rowid value should be bound to the index associated
///*   with the column.
///*
///* pNext:
///*   All RecoverTable objects used by the recovery operation are allocated
///*   and populated as part of creating the recovered database schema in
///*   the output database, before any non-schema data are recovered. They
///*   are then stored in a singly-linked list linked by this variable beginning
///*   at sqlite3_recover.pTblList.
#[repr(C)]
#[derive(Copy, Clone)]
struct RecoverTable {
    i_root: u32,
    z_tab: *mut i8,
    n_col: i32,
    a_col: *mut RecoverColumn,
    b_intkey: i32,
    i_rowid_bind: i32,
    p_next: *mut RecoverTable,
}

///* Each database column is represented by an instance of the following object
///* stored in the RecoverTable.aCol[] array of the associated table.
///*
///* iField:
///*   The index of the associated field within database records. Or -1 if
///*   there is no associated field (e.g. for virtual generated columns).
///*
///* iBind:
///*   The bind index of the INSERT statement to bind this columns values
///*   to. Or 0 if there is no such index (iff (iField<0)).
///*
///* bIPK:
///*   True if this is the INTEGER PRIMARY KEY column.
///*
///* zCol:
///*   Name of column.
///*
///* eHidden:
///*   A RECOVER_EHIDDEN_* constant value (see below for interpretation of each).
#[repr(C)]
#[derive(Copy, Clone)]
struct RecoverColumn {
    i_field: i32,
    i_bind: i32,
    b_ipk: i32,
    z_col: *mut i8,
    e_hidden: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RecoverStateLAF {
    p_used: *mut RecoverBitmap,
    n_pg: i64,
    p_all_and_parent: *mut Sqlite3Stmt,
    p_map_insert: *mut Sqlite3Stmt,
    p_max_field: *mut Sqlite3Stmt,
    p_used_pages: *mut Sqlite3Stmt,
    p_find_root: *mut Sqlite3Stmt,
    p_insert: *mut Sqlite3Stmt,
    p_all_page: *mut Sqlite3Stmt,
    p_page_data: *mut Sqlite3Stmt,
    ap_val: *mut *mut Sqlite3Value,
    n_max_field: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RecoverBitmap {
    n_pg: i64,
    a_elem: [u32; 0],
}

///* Like strlen(). But handles NULL pointer arguments.
extern "C" fn recover_strlen(z_str_1: *const i8) -> i32 {
    if z_str_1 == core::ptr::null() { return 0; }
    return (unsafe { strlen(z_str_1) } & 2147483647 as u64) as i32;
}

///* This is a worker function that does the heavy lifting for both init
///* functions:
///*
///*     sqlite3_recover_init()
///*     sqlite3_recover_init_sql()
///*
///* All this function does is allocate space for the recover handle and
///* take copies of the input parameters. All the real work is done within
///* sqlite3_recover_run().
#[unsafe(no_mangle)]
pub extern "C" fn recover_init(db: *mut Sqlite3, mut z_db_1: *const i8,
    z_uri_1: *const i8,
    x_sql_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> i32>,
    p_sql_ctx_1: *mut ()) -> *mut Sqlite3Recover {
    let mut p_ret: *mut Sqlite3Recover = core::ptr::null_mut();
    let mut n_db: i32 = 0;
    let mut n_uri: i32 = 0;
    let mut n_byte: i32 = 0;
    if z_db_1 == core::ptr::null() {
        z_db_1 = c"main".as_ptr() as *mut i8 as *const i8;
    }
    n_db = recover_strlen(z_db_1);
    n_uri = recover_strlen(z_uri_1);
    n_byte =
        (core::mem::size_of::<Sqlite3Recover>() as u64 + n_db as u64 +
                        1 as u64 + n_uri as u64 + 1 as u64) as i32;
    p_ret = unsafe { sqlite3_malloc(n_byte) } as *mut Sqlite3Recover;
    if !(p_ret).is_null() {
        unsafe { memset(p_ret as *mut (), 0, n_byte as u64) };
        unsafe { (*p_ret).db_in = db };
        unsafe {
            (*p_ret).z_db =
                unsafe { &raw mut *p_ret.offset(1 as isize) } as *mut i8
        };
        unsafe {
            (*p_ret).z_uri =
                unsafe {
                    unsafe { (*p_ret).z_db.offset((n_db + 1) as isize) }
                }
        };
        unsafe {
            memcpy(unsafe { (*p_ret).z_db } as *mut (), z_db_1 as *const (),
                n_db as u64)
        };
        if n_uri > 0 && !(z_uri_1).is_null() {
            unsafe {
                memcpy(unsafe { (*p_ret).z_uri } as *mut (),
                    z_uri_1 as *const (), n_uri as u64)
            };
        }
        unsafe { (*p_ret).x_sql = x_sql_1 };
        unsafe { (*p_ret).p_sql_ctx = p_sql_ctx_1 };
        unsafe { (*p_ret).b_recover_rowid = 1 };
    }
    return p_ret;
}

/// 
///* These two APIs attempt to create and return a new sqlite3_recover object.
///* In both cases the first two arguments identify the (possibly
///* corrupt) database to recover data from. The first argument is an open
///* database handle and the second the name of a database attached to that
///* handle (i.e. "main", "temp" or the name of an attached database).
///*
///* If sqlite3_recover_init() is used to create the new sqlite3_recover
///* handle, then data is recovered into a new database, identified by
///* string parameter zUri. zUri may be an absolute or relative file path,
///* or may be an SQLite URI. If the identified database file already exists,
///* it is overwritten.
///*
///* If sqlite3_recover_init_sql() is invoked, then any recovered data will
///* be returned to the user as a series of SQL statements. Executing these
///* SQL statements results in the same database as would have been created
///* had sqlite3_recover_init() been used. For each SQL statement in the
///* output, the callback function passed as the third argument (xSql) is 
///* invoked once. The first parameter is a passed a copy of the fourth argument
///* to this function (pCtx) as its first parameter, and a pointer to a
///* nul-terminated buffer containing the SQL statement formated as UTF-8 as 
///* the second. If the xSql callback returns any value other than SQLITE_OK,
///* then processing is immediately abandoned and the value returned used as
///* the recover handle error code (see below).
///*
///* If an out-of-memory error occurs, NULL may be returned instead of
///* a valid handle. In all other cases, it is the responsibility of the
///* application to avoid resource leaks by ensuring that
///* sqlite3_recover_finish() is called on all allocated handles.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_init(db: *mut Sqlite3, z_db_1: *const i8,
    z_uri_1: *const i8) -> *mut Sqlite3Recover {
    return recover_init(db, z_db_1, z_uri_1, None, core::ptr::null_mut());
}

///* Initialize a recovery handle that returns recovered data in the
///* form of SQL statements via a callback.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_init_sql(db: *mut Sqlite3,
    z_db_1: *const i8,
    x_sql_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> i32>,
    p_sql_ctx_1: *mut ()) -> *mut Sqlite3Recover {
    return recover_init(db, z_db_1, core::ptr::null(), x_sql_1, p_sql_ctx_1);
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). NULL is returned in this case.
///*
///* Otherwise, an attempt is made to interpret zFmt as a printf() style
///* formatting string and the result of using the trailing arguments for
///* parameter substitution with it written into a buffer obtained from
///* sqlite3_malloc(). If successful, a pointer to the buffer is returned.
///* It is the responsibility of the caller to eventually free the buffer
///* using sqlite3_free().
///*
///* Or, if an error occurs, an error code and message is left in the recover
///* handle and NULL returned.
unsafe extern "C" fn recover_m_printf(p: &mut Sqlite3Recover,
    z_fmt_1: *const i8, mut __va0: ...) -> *mut i8 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    ();
    if (*p).err_code == 0 {
        if z == core::ptr::null_mut() { (*p).err_code = 7; }
    } else {
        unsafe { sqlite3_free(z as *mut ()) };
        z = core::ptr::null_mut();
    }
    return z;
}

///* Configure an sqlite3_recover object that has just been created using
///* sqlite3_recover_init() or sqlite3_recover_init_sql(). This function
///* may only be called before the first call to sqlite3_recover_step()
///* or sqlite3_recover_run() on the object.
///*
///* The second argument passed to this function must be one of the
///* SQLITE_RECOVER_* symbols defined below. Valid values for the third argument
///* depend on the specific SQLITE_RECOVER_* symbol in use.
///*
///* SQLITE_OK is returned if the configuration operation was successful,
///* or an SQLite error code otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_config(p: *mut Sqlite3Recover, op: i32,
    p_arg_1: *mut ()) -> i32 {
    let mut rc: i32 = 0;
    if p == core::ptr::null_mut() {
        rc = 7;
    } else if unsafe { (*p).e_state } != 0 {
        rc = 21;
    } else {
        '__s0:
            {
            match op {
                789 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p).z_state_db } as *mut ())
                    };
                    unsafe {
                        (*p).z_state_db =
                            unsafe {
                                recover_m_printf(unsafe { &mut *p },
                                    c"%s".as_ptr() as *mut i8 as *const i8, p_arg_1 as *mut i8)
                            }
                    };
                }
                1 => {
                    {
                        let z_arg: *const i8 = p_arg_1 as *const i8;
                        unsafe {
                            sqlite3_free(unsafe { (*p).z_lost_and_found } as *mut ())
                        };
                        if !(z_arg).is_null() {
                            unsafe {
                                (*p).z_lost_and_found =
                                    unsafe {
                                        recover_m_printf(unsafe { &mut *p },
                                            c"%s".as_ptr() as *mut i8 as *const i8, z_arg)
                                    }
                            };
                        } else {
                            unsafe { (*p).z_lost_and_found = core::ptr::null_mut() };
                        }
                        break '__s0;
                    }
                    unsafe {
                        (*p).b_freelist_corrupt = unsafe { *(p_arg_1 as *mut i32) }
                    };
                }
                2 => {
                    unsafe {
                        (*p).b_freelist_corrupt = unsafe { *(p_arg_1 as *mut i32) }
                    };
                }
                3 => {
                    unsafe {
                        (*p).b_recover_rowid = unsafe { *(p_arg_1 as *mut i32) }
                    };
                }
                4 => {
                    unsafe {
                        (*p).b_slow_indexes = unsafe { *(p_arg_1 as *mut i32) }
                    };
                }
                _ => { rc = 12; }
            }
        }
    }
    return rc;
}

///* Set the error code and error message for the recover handle passed as
///* the first argument. The error code is set to the value of parameter
///* errCode.
///*
///* Parameter zFmt must be a printf() style formatting string. The handle 
///* error message is set to the result of using any trailing arguments for 
///* parameter substitutions in the formatting string.
///*
///* For example:
///*
///*   recoverError(p, SQLITE_ERROR, "no such table: %s", zTablename);
unsafe extern "C" fn recover_error(p: &mut Sqlite3Recover, err_code_1: i32,
    z_fmt_1: *const i8, mut __va0: ...) -> i32 {
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    if !(z_fmt_1).is_null() { z = unsafe { sqlite3_vmprintf(z_fmt_1, ap) }; }
    ();
    unsafe { sqlite3_free((*p).z_err_msg as *mut ()) };
    (*p).z_err_msg = z;
    (*p).err_code = err_code_1;
    return err_code_1;
}

///* If this recover handle is not in SQL callback mode (i.e. was not created 
///* using sqlite3_recover_init_sql()) of if an error has already occurred, 
///* this function is a no-op. Otherwise, issue a callback with SQL statement
///* zSql as the parameter. 
///*
///* If the callback returns non-zero, set the recover handle error code to
///* the value returned (so that the caller will abandon processing).
extern "C" fn recover_sql_callback(p: *mut Sqlite3Recover, z_sql_1: *const i8)
    -> () {
    if unsafe { (*p).err_code } == 0 && unsafe { (*p).x_sql.is_some() } {
        let res: i32 =
            unsafe {
                (unsafe {
                        (*p).x_sql.unwrap()
                    })(unsafe { (*p).p_sql_ctx }, z_sql_1)
            };
        if res != 0 {
            unsafe {
                recover_error(unsafe { &mut *p }, 1,
                    c"callback returned an error - %d".as_ptr() as *mut i8 as
                        *const i8, res)
            };
        }
    }
}

extern "C" fn recover_enter_mutex() -> () {
    unsafe { sqlite3_mutex_enter(unsafe { sqlite3_mutex_alloc(9) }) };
}

///* Set the recover handle error to the error code and message returned by
///* calling sqlite3_errcode() and sqlite3_errmsg(), respectively, on database
///* handle db.
extern "C" fn recover_db_error(p: *mut Sqlite3Recover, db: *mut Sqlite3)
    -> i32 {
    return unsafe {
            recover_error(unsafe { &mut *p }, unsafe { sqlite3_errcode(db) },
                c"%s".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(db) })
        };
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). 
///*
///* Otherwise, it attempts to prepare the SQL statement in zSql against
///* database handle db. If successful, the statement handle is returned.
///* Or, if an error occurs, NULL is returned and an error left in the
///* recover handle.
extern "C" fn recover_prepare(p: *mut Sqlite3Recover, db: *mut Sqlite3,
    z_sql_1: *const i8) -> *mut Sqlite3Stmt {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    if unsafe { (*p).err_code } == 0 {
        if unsafe {
                    sqlite3_prepare_v2(db, z_sql_1, -1, &mut p_stmt,
                        core::ptr::null_mut())
                } != 0 {
            recover_db_error(p, db);
        }
    }
    return p_stmt;
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). 
///*
///* Otherwise, argument zFmt is used as a printf() style format string,
///* along with any trailing arguments, to create an SQL statement. This
///* SQL statement is prepared against database handle db and, if successful,
///* the statment handle returned. Or, if an error occurs - either during
///* the printf() formatting or when preparing the resulting SQL - an
///* error code and message are left in the recover handle.
unsafe extern "C" fn recover_prepare_printf(p: *mut Sqlite3Recover,
    db: *mut Sqlite3, z_fmt_1: *const i8, mut __va0: ...)
    -> *mut Sqlite3Stmt {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    if unsafe { (*p).err_code } == 0 {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
        ();
        if z == core::ptr::null_mut() {
            unsafe { (*p).err_code = 7 };
        } else {
            p_stmt = recover_prepare(p, db, z as *const i8);
            unsafe { sqlite3_free(z as *mut ()) };
        }
    }
    return p_stmt;
}

///* Finalize SQLite statement handle pStmt. If the call to sqlite3_reset() 
///* indicates that an error occurred, and there is not already an error
///* in the recover handle passed as the first argument, set the error
///* code and error message appropriately.
extern "C" fn recover_finalize(p: *mut Sqlite3Recover,
    p_stmt_1: *mut Sqlite3Stmt) -> () {
    let db: *mut Sqlite3 = unsafe { sqlite3_db_handle(p_stmt_1) };
    let rc: i32 = unsafe { sqlite3_finalize(p_stmt_1) };
    if rc != 0 && unsafe { (*p).err_code } == 0 { recover_db_error(p, db); }
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). Zero is returned in this case.
///*
///* Otherwise, execute "PRAGMA page_count" against the input database. If
///* successful, return the integer result. Or, if an error occurs, leave an
///* error code and error message in the sqlite3_recover handle and return
///* zero.
extern "C" fn recover_page_count(p: *mut Sqlite3Recover) -> i64 {
    let mut n_pg: i64 = 0 as i64;
    if unsafe { (*p).err_code } == 0 {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        p_stmt =
            unsafe {
                recover_prepare_printf(p, unsafe { (*p).db_in },
                    c"PRAGMA %Q.page_count".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).z_db })
            };
        if !(p_stmt).is_null() {
            unsafe { sqlite3_step(p_stmt) };
            n_pg = unsafe { sqlite3_column_int64(p_stmt, 0) };
        }
        recover_finalize(p, p_stmt);
    }
    return n_pg;
}

///* Reset SQLite statement handle pStmt. If the call to sqlite3_reset() 
///* indicates that an error occurred, and there is not already an error
///* in the recover handle passed as the first argument, set the error
///* code and error message appropriately.
///*
///* This function returns a copy of the statement handle pointer passed
///* as the second argument.
extern "C" fn recover_reset(p: *mut Sqlite3Recover,
    p_stmt_1: *mut Sqlite3Stmt) -> *mut Sqlite3Stmt {
    let rc: i32 = unsafe { sqlite3_reset(p_stmt_1) };
    if rc != 0 && rc != 19 && unsafe { (*p).err_code } == 0 {
        recover_db_error(p, unsafe { sqlite3_db_handle(p_stmt_1) });
    }
    return p_stmt_1;
}

///* The implementation of a user-defined SQL function invoked by the 
///* sqlite_dbdata and sqlite_dbptr virtual table modules to access pages
///* of the database being recovered.
///*
///* This function always takes a single integer argument. If the argument
///* is zero, then the value returned is the number of pages in the db being
///* recovered. If the argument is greater than zero, it is a page number. 
///* The value returned in this case is an SQL blob containing the data for 
///* the identified page of the db being recovered. e.g.
///*
///*     SELECT getpage(0);       -- return number of pages in db
///*     SELECT getpage(4);       -- return page 4 of db as a blob of data
extern "C" fn recover_get_page(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let p: *mut Sqlite3Recover =
        unsafe { sqlite3_user_data(p_ctx_1) } as *mut Sqlite3Recover;
    let pgno: i64 =
        unsafe {
            sqlite3_value_int64(unsafe { *ap_arg_1.offset(0 as isize) })
        };
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    if !(n_arg_1 == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverGetPage".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 722,
                c"nArg==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if pgno == 0 as i64 {
        let n_pg: i64 = recover_page_count(p);
        unsafe { sqlite3_result_int64(p_ctx_1, n_pg) };
        return;
    } else {
        if unsafe { (*p).p_get_page } == core::ptr::null_mut() {
            p_stmt =
                {
                    unsafe {
                        (*p).p_get_page =
                            unsafe {
                                recover_prepare_printf(p, unsafe { (*p).db_in },
                                    c"SELECT data FROM sqlite_dbpage(%Q) WHERE pgno=?".as_ptr()
                                            as *mut i8 as *const i8, unsafe { (*p).z_db })
                            }
                    };
                    unsafe { (*p).p_get_page }
                };
        } else if unsafe { (*p).err_code } == 0 {
            p_stmt = unsafe { (*p).p_get_page };
        }
        if !(p_stmt).is_null() {
            unsafe { sqlite3_bind_int64(p_stmt, 1, pgno) };
            if 100 == unsafe { sqlite3_step(p_stmt) } {
                let mut a_pg: *const u8 = core::ptr::null();
                let mut n_pg_1: i32 = 0;
                if !(unsafe { (*p).err_code } == 0) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"recoverGetPage".as_ptr() as *const i8,
                            c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 741,
                            c"p->errCode==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                a_pg = unsafe { sqlite3_column_blob(p_stmt, 0) } as *const u8;
                n_pg_1 = unsafe { sqlite3_column_bytes(p_stmt, 0) };
                if pgno == 1 as i64 && n_pg_1 == unsafe { (*p).pgsz } &&
                        0 ==
                            unsafe {
                                memcmp(unsafe { (*p).p_page1_cache } as *const (),
                                    a_pg as *const (), n_pg_1 as u64)
                            } {
                    a_pg = unsafe { (*p).p_page1_disk } as *const u8;
                }
                unsafe {
                    sqlite3_result_blob(p_ctx_1, a_pg as *const (),
                        n_pg_1 - unsafe { (*p).n_reserve },
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            }
            recover_reset(p, p_stmt);
        }
    }
    if unsafe { (*p).err_code } != 0 {
        if !(unsafe { (*p).z_err_msg }).is_null() {
            unsafe {
                sqlite3_result_error(p_ctx_1,
                    unsafe { (*p).z_err_msg } as *const i8, -1)
            };
        }
        unsafe {
            sqlite3_result_error_code(p_ctx_1, unsafe { (*p).err_code })
        };
    }
}

///* Query bitmap object pMap for the state of the bit associated with page
///* iPg. Return 1 if it is set, or 0 otherwise.
extern "C" fn recover_bitmap_query(p_map_1: &RecoverBitmap, i_pg_1: i64)
    -> i32 {
    let mut ret: i32 = 1;
    if i_pg_1 <= (*p_map_1).n_pg && i_pg_1 > 0 as i64 {
        let i_elem: i32 = (i_pg_1 / 32 as i64) as i32;
        let i_bit: i32 = (i_pg_1 % 32 as i64) as i32;
        ret =
            if unsafe {
                            *((*p_map_1).a_elem.as_ptr() as
                                        *mut u32).offset(i_elem as isize)
                        } & (1 as u32) << i_bit != 0 {
                1
            } else { 0 };
    }
    return ret;
}

///* Implementation of SQL scalar function "page_is_used". This function
///* is used as part of the procedure for locating orphan rows for the
///* lost-and-found table, and it depends on those routines having populated
///* the sqlite3_recover.laf.pUsed variable.
///*
///* The only argument to this function is a page-number. It returns true 
///* if the page has already been used somehow during data recovery, or false
///* otherwise.
///*
///*     SELECT page_is_used(<pgno>);
extern "C" fn recover_page_is_used(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let p: *const Sqlite3Recover =
        unsafe { sqlite3_user_data(p_ctx_1) } as *mut Sqlite3Recover as
            *const Sqlite3Recover;
    let pgno: i64 =
        unsafe {
            sqlite3_value_int64(unsafe { *ap_arg_1.offset(0 as isize) })
        };
    if !(n_arg_1 == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverPageIsUsed".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 695,
                c"nArg==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        sqlite3_result_int(p_ctx_1,
            recover_bitmap_query(unsafe { &*unsafe { (*p).laf.p_used } },
                pgno))
    };
}

///* Implementation of SQL scalar function "read_i32". The first argument to 
///* this function must be a blob. The second a non-negative integer. This 
///* function reads and returns a 32-bit big-endian integer from byte
///* offset (4*<arg2>) of the blob.
///*
///*     SELECT read_i32(<blob>, <idx>)
extern "C" fn recover_read_i32(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p_blob: *const u8 = core::ptr::null();
    let mut n_blob: i32 = 0;
    let mut i_int: i32 = 0;
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverReadI32".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 661,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n_blob =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    p_blob =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const u8;
    i_int =
        unsafe { sqlite3_value_int(unsafe { *argv.offset(1 as isize) }) } &
            65535;
    if (i_int + 1) * 4 <= n_blob {
        let a: *const u8 = unsafe { &*p_blob.offset((i_int * 4) as isize) };
        let i_val: i64 =
            ((unsafe { *a.offset(0 as isize) } as i64) << 24) +
                        ((unsafe { *a.offset(1 as isize) } as i64) << 16) +
                    ((unsafe { *a.offset(2 as isize) } as i64) << 8) +
                ((unsafe { *a.offset(3 as isize) } as i64) << 0);
        unsafe { sqlite3_result_int64(context, i_val) };
    }
}

///* Find a string that is not found anywhere in z[].  Return a pointer
///* to that string.
///*
///* Try to use zA and zB first.  If both of those are already found in z[]
///* then make up some string and store it in the buffer zBuf.
extern "C" fn recover_unused_string(z: *const i8, z_a_1: *const i8,
    z_b_1: *const i8, z_buf_1: *mut i8) -> *const i8 {
    let mut i: u32 = 0 as u32;
    if unsafe { strstr(z, z_a_1) } == core::ptr::null_mut() { return z_a_1; }
    if unsafe { strstr(z, z_b_1) } == core::ptr::null_mut() { return z_b_1; }
    '__b1: loop {
        '__c1: loop {
            unsafe {
                sqlite3_snprintf(20, z_buf_1,
                    c"(%s%u)".as_ptr() as *mut i8 as *const i8, z_a_1,
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t })
            };
            break '__c1;
        }
        if !(unsafe { strstr(z, z_buf_1 as *const i8) } !=
                        core::ptr::null_mut()) {
            break '__b1;
        }
    }
    return z_buf_1 as *const i8;
}

///* Implementation of scalar SQL function "escape_crlf".  The argument passed to
///* this function is the output of built-in function quote(). If the first
///* character of the input is "'", indicating that the value passed to quote()
///* was a text value, then this function searches the input for "\n" and "\r"
///* characters and adds a wrapper similar to the following:
///*
///*   replace(replace(<input>, '\n', char(10), '\r', char(13));
///*
///* Or, if the first character of the input is not "'", then a copy of the input
///* is returned.
extern "C" fn recover_escape_crlf(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let z_text: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    { let _ = argc; };
    if !(z_text).is_null() &&
            unsafe { *z_text.offset(0 as isize) } as i32 == '\'' as i32 {
        let n_text: i32 =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        let mut i: i32 = 0;
        let mut z_buf1: [i8; 20] = [0; 20];
        let mut z_buf2: [i8; 20] = [0; 20];
        let mut z_nl: *const i8 = core::ptr::null();
        let mut z_cr: *const i8 = core::ptr::null();
        let mut n_cr: i32 = 0;
        let mut n_nl: i32 = 0;
        {
            i = 0;
            '__b2: loop {
                if !(unsafe { *z_text.offset(i as isize) } != 0) {
                    break '__b2;
                }
                '__c2: loop {
                    if z_nl == core::ptr::null() &&
                            unsafe { *z_text.offset(i as isize) } as i32 == '\n' as i32
                        {
                        z_nl =
                            recover_unused_string(z_text,
                                c"\\n".as_ptr() as *mut i8 as *const i8,
                                c"\\012".as_ptr() as *mut i8 as *const i8,
                                &raw mut z_buf1[0 as usize] as *mut i8);
                        n_nl = unsafe { strlen(z_nl) } as i32;
                    }
                    if z_cr == core::ptr::null() &&
                            unsafe { *z_text.offset(i as isize) } as i32 == '\r' as i32
                        {
                        z_cr =
                            recover_unused_string(z_text,
                                c"\\r".as_ptr() as *mut i8 as *const i8,
                                c"\\015".as_ptr() as *mut i8 as *const i8,
                                &raw mut z_buf2[0 as usize] as *mut i8);
                        n_cr = unsafe { strlen(z_cr) } as i32;
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if !(z_nl).is_null() || !(z_cr).is_null() {
            let mut i_out: i32 = 0;
            let n_max: i64 = if n_nl > n_cr { n_nl } else { n_cr } as i64;
            let n_alloc: i64 =
                n_max * n_text as i64 + (n_max + 64 as i64) * 2 as i64;
            let z_out: *mut i8 =
                unsafe { sqlite3_malloc64(n_alloc as Sqlite3Uint64) } as
                    *mut i8;
            if z_out == core::ptr::null_mut() {
                unsafe { sqlite3_result_error_nomem(context) };
                return;
            }
            if !(z_nl).is_null() && !(z_cr).is_null() {
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (),
                        c"replace(replace(".as_ptr() as *mut i8 as *const (),
                        16 as u64)
                };
                i_out += 16;
            } else {
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), c"replace(".as_ptr() as *mut i8 as *const (),
                        8 as u64)
                };
                i_out += 8;
            }
            {
                i = 0;
                '__b3: loop {
                    if !(unsafe { *z_text.offset(i as isize) } != 0) {
                        break '__b3;
                    }
                    '__c3: loop {
                        if unsafe { *z_text.offset(i as isize) } as i32 ==
                                '\n' as i32 {
                            unsafe {
                                memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                                        *mut (), z_nl as *const (), n_nl as u64)
                            };
                            i_out += n_nl;
                        } else if unsafe { *z_text.offset(i as isize) } as i32 ==
                                '\r' as i32 {
                            unsafe {
                                memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                                        *mut (), z_cr as *const (), n_cr as u64)
                            };
                            i_out += n_cr;
                        } else {
                            unsafe {
                                *z_out.offset(i_out as isize) =
                                    unsafe { *z_text.offset(i as isize) } as i8
                            };
                            { let __p = &mut i_out; let __t = *__p; *__p += 1; __t };
                        }
                        break '__c3;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if !(z_nl).is_null() {
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), c",\'".as_ptr() as *mut i8 as *const (), 2 as u64)
                };
                i_out += 2;
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), z_nl as *const (), n_nl as u64)
                };
                i_out += n_nl;
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), c"\', char(10))".as_ptr() as *mut i8 as *const (),
                        12 as u64)
                };
                i_out += 12;
            }
            if !(z_cr).is_null() {
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), c",\'".as_ptr() as *mut i8 as *const (), 2 as u64)
                };
                i_out += 2;
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), z_cr as *const (), n_cr as u64)
                };
                i_out += n_cr;
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_out as isize) } as
                            *mut (), c"\', char(13))".as_ptr() as *mut i8 as *const (),
                        12 as u64)
                };
                i_out += 12;
            }
            unsafe {
                sqlite3_result_text(context, z_out as *const i8, i_out,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
            unsafe { sqlite3_free(z_out as *mut ()) };
            return;
        }
    }
    unsafe {
        sqlite3_result_value(context, unsafe { *argv.offset(0 as isize) })
    };
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). A copy of the error code is returned in
///* this case. 
///*
///* Otherwise, an attempt is made to open the output database, attach
///* and create the schema of the temporary database used to store
///* intermediate data, and to register all required user functions and
///* virtual table modules with the output handle.
///*
///* If no error occurs, SQLITE_OK is returned. Otherwise, an error code
///* and error message are left in the recover handle and a copy of the
///* error code returned.
#[allow(unused_doc_comments)]
extern "C" fn recover_open_output(p: *mut Sqlite3Recover) -> i32 {
    let a_func: [FuncN4Func; 4] =
        [FuncN4Func {
                    z_name: c"getpage".as_ptr() as *const i8,
                    n_arg: 1,
                    x_func: Some(recover_get_page),
                },
                FuncN4Func {
                    z_name: c"page_is_used".as_ptr() as *const i8,
                    n_arg: 1,
                    x_func: Some(recover_page_is_used),
                },
                FuncN4Func {
                    z_name: c"read_i32".as_ptr() as *const i8,
                    n_arg: 2,
                    x_func: Some(recover_read_i32),
                },
                FuncN4Func {
                    z_name: c"escape_crlf".as_ptr() as *const i8,
                    n_arg: 1,
                    x_func: Some(recover_escape_crlf),
                }];
    let flags: i32 = (64 | 4 | 2) as i32;
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    /// New database handle
    let mut ii: i32 = 0;

    /// For iterating through aFunc[]
    if !(unsafe { (*p).db_out } == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverOpenOutput".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1015,
                c"p->dbOut==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe {
                sqlite3_open_v2(unsafe { (*p).z_uri } as *const i8, &mut db,
                    flags, core::ptr::null())
            } != 0 {
        recover_db_error(p, db);
    }
    if unsafe { (*p).err_code } == 0 {
        unsafe {
            (*p).err_code =
                unsafe {
                    sqlite3_dbdata_init(db, core::ptr::null_mut(),
                        core::ptr::null())
                }
        };
    }
    {
        ii = 0;
        '__b4: loop {
            if !(unsafe { (*p).err_code } == 0 &&
                            ii <
                                (core::mem::size_of::<[FuncN4Func; 4]>() as u64 /
                                        core::mem::size_of::<FuncN4Func>() as u64) as i32) {
                break '__b4;
            }
            '__c4: loop {
                unsafe {
                    (*p).err_code =
                        unsafe {
                            sqlite3_create_function(db, a_func[ii as usize].z_name,
                                a_func[ii as usize].n_arg, 1, p as *mut (),
                                a_func[ii as usize].x_func, None, None)
                        }
                };
                break '__c4;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p).db_out = db };
    return unsafe { (*p).err_code };
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RecoverGlobal {
    p_methods: *const Sqlite3IoMethods,
    p: *mut Sqlite3Recover,
}

static mut recover_g: RecoverGlobal = unsafe { core::mem::zeroed() };

extern "C" fn recover_vfs_close(p_fd: *mut Sqlite3File) -> i32 {
    unsafe {
        if !(unsafe { (*p_fd).p_methods } !=
                                &raw mut recover_methods as *const Sqlite3IoMethods) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"recoverVfsClose".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 2247,
                    c"pFd->pMethods!=&recover_methods".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_fd).p_methods }).x_close.unwrap()
                    })(p_fd)
            };
    }
}

///* Decode and return an unsigned 16-bit big-endian integer value from 
///* buffer a[].
extern "C" fn recover_get_u16(a: *const u8) -> u32 {
    return ((unsafe { *a.offset(0 as isize) } as u32) << 8) +
            unsafe { *a.offset(1 as isize) } as u32;
}

///* Decode and return an unsigned 32-bit big-endian integer value from 
///* buffer a[].
extern "C" fn recover_get_u32(a: *const u8) -> u32 {
    return ((unsafe { *a.offset(0 as isize) } as u32) << 24) +
                    ((unsafe { *a.offset(1 as isize) } as u32) << 16) +
                ((unsafe { *a.offset(2 as isize) } as u32) << 8) +
            unsafe { *a.offset(3 as isize) } as u32;
}

///* Decode an SQLite varint from buffer a[]. Write the decoded value to (*pVal)
///* and return the number of bytes consumed.
extern "C" fn recover_get_varint(a: *const u8, p_val_1: &mut i64) -> i32 {
    let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b5: loop {
            if !(i < 8) { break '__b5; }
            '__c5: loop {
                u =
                    (u << 7) +
                        (unsafe { *a.offset(i as isize) } as i32 & 127) as
                            Sqlite3Uint64;
                if unsafe { *a.offset(i as isize) } as i32 & 128 == 0 {
                    *p_val_1 = u as Sqlite3Int64;
                    return i + 1;
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    u =
        (u << 8) +
            (unsafe { *a.offset(i as isize) } as i32 & 255) as Sqlite3Uint64;
    *p_val_1 = u as Sqlite3Int64;
    return 9;
}

///* The second argument points to a buffer n bytes in size. If this buffer
///* or a prefix thereof appears to contain a well-formed SQLite b-tree page, 
///* return the page-size in bytes. Otherwise, if the buffer does not 
///* appear to contain a well-formed b-tree page, return 0.
#[allow(unused_doc_comments)]
extern "C" fn recover_is_valid_page(a_tmp_1: *mut u8, a: &[u8]) -> i32 {
    let a_used: *mut u8 = a_tmp_1;
    let mut n_frag: i32 = 0;
    let mut n_actual: i32 = 0;
    let mut i_free: i32 = 0;
    let mut n_cell: i32 = 0;
    /// Number of cells on page
    let mut i_cell_off: i32 = 0;
    /// Offset of cell array in page
    let mut i_content: i32 = 0;
    let mut e_type: i32 = 0;
    let mut ii: i32 = 0;
    e_type = a[0 as usize] as i32;
    if e_type != 2 && e_type != 5 && e_type != 10 && e_type != 13 {
        return 0;
    }
    i_free = recover_get_u16(&a[1 as usize]) as i32;
    n_cell = recover_get_u16(&a[3 as usize]) as i32;
    i_content = recover_get_u16(&a[5 as usize]) as i32;
    if i_content == 0 { i_content = 65536; }
    n_frag = a[7 as usize] as i32;
    if i_content > a.len() as i32 { return 0; }
    unsafe { memset(a_used as *mut (), 0, a.len() as i32 as u64) };
    unsafe { memset(a_used as *mut (), 255, i_content as u64) };
    if i_free != 0 && i_free <= i_content { return 0; }
    while i_free != 0 {
        let mut i_next: i32 = 0;
        let mut n_byte: i32 = 0;
        if i_free > a.len() as i32 - 4 { return 0; }
        i_next = recover_get_u16(&a[i_free as usize]) as i32;
        n_byte = recover_get_u16(&a[(i_free + 2) as usize]) as i32;
        if i_free + n_byte > a.len() as i32 || n_byte < 4 { return 0; }
        if i_next != 0 && i_next < i_free + n_byte { return 0; }
        unsafe {
            memset(unsafe { &raw mut *a_used.offset(i_free as isize) } as
                    *mut (), 255, n_byte as u64)
        };
        i_free = i_next;
    }
    if e_type == 2 || e_type == 5 {
        i_cell_off = 12;
    } else { i_cell_off = 8; }
    if i_cell_off + 2 * n_cell > i_content { return 0; }
    {
        ii = 0;
        '__b7: loop {
            if !(ii < n_cell) { break '__b7; }
            '__c7: loop {
                let mut i_byte: i32 = 0;
                let mut n_payload: i64 = 0 as i64;
                let mut n_byte_1: i32 = 0;
                let i_off: i32 =
                    recover_get_u16(&a[(i_cell_off + 2 * ii) as usize]) as i32;
                if i_off < i_content || i_off > a.len() as i32 { return 0; }
                if e_type == 5 || e_type == 2 { n_byte_1 += 4; }
                n_byte_1 +=
                    recover_get_varint(&a[(i_off + n_byte_1) as usize],
                        &mut n_payload);
                if e_type == 13 {
                    let mut dummy: i64 = 0 as i64;
                    n_byte_1 +=
                        recover_get_varint(&a[(i_off + n_byte_1) as usize],
                            &mut dummy);
                }
                if e_type != 5 {
                    let x: i32 =
                        if e_type == 13 {
                            a.len() as i32 - 35
                        } else { (a.len() as i32 - 12) * 64 / 255 - 23 };
                    let m: i32 = (a.len() as i32 - 12) * 32 / 255 - 23;
                    let k: i32 =
                        (m as i64 +
                                (n_payload - m as i64) % (a.len() as i32 - 4) as i64) as
                            i32;
                    if n_payload < x as i64 {
                        n_byte_1 += n_payload as i32;
                    } else if k <= x {
                        n_byte_1 += k + 4;
                    } else { n_byte_1 += m + 4; }
                }
                if i_off + n_byte_1 > a.len() as i32 { return 0; }
                {
                    i_byte = i_off;
                    '__b8: loop {
                        if !(i_byte < i_off + n_byte_1) { break '__b8; }
                        '__c8: loop {
                            if unsafe { *a_used.offset(i_byte as isize) } as i32 != 0 {
                                return 0;
                            }
                            unsafe { *a_used.offset(i_byte as isize) = 255 as u8 };
                            break '__c8;
                        }
                        { let __p = &mut i_byte; let __t = *__p; *__p += 1; __t };
                    }
                }
                break '__c7;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    n_actual = 0;
    {
        ii = 0;
        '__b9: loop {
            if !(ii < a.len() as i32) { break '__b9; }
            '__c9: loop {
                if unsafe { *a_used.offset(ii as isize) } as i32 == 0 {
                    { let __p = &mut n_actual; let __t = *__p; *__p += 1; __t };
                }
                break '__c9;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return (n_actual == n_frag) as i32;
}

///* Detect the page-size of the database opened by file-handle pFd by 
///* searching the first part of the file for a well-formed SQLite b-tree 
///* page. If parameter nReserve is non-zero, then as well as searching for
///* a b-tree page with zero reserved bytes, this function searches for one
///* with nReserve reserved bytes at the end of it.
///*
///* If successful, set variable p->detected_pgsz to the detected page-size
///* in bytes and return SQLITE_OK. Or, if no error occurs but no valid page
///* can be found, return SQLITE_OK but leave p->detected_pgsz set to 0. Or,
///* if an error occurs (e.g. an IO or OOM error), then an SQLite error code
///* is returned. The final value of p->detected_pgsz is undefined in this
///* case.
extern "C" fn recover_vfs_detect_pagesize(p: &mut Sqlite3Recover,
    p_fd_1: *mut Sqlite3File, mut n_reserve_1: u32, n_sz_1: i64) -> i32 {
    let mut rc: i32 = 0;
    let n_min: i32 = 512 as i32;
    let n_max: i32 = 65536 as i32;
    let n_max_blk: i32 = 4 as i32;
    let mut pgsz: u32 = 0 as u32;
    let mut i_blk: i32 = 0;
    let mut a_pg: *mut u8 = core::ptr::null_mut();
    let mut a_tmp: *mut u8 = core::ptr::null_mut();
    let mut n_blk: i32 = 0;
    a_pg = unsafe { sqlite3_malloc(2 * n_max as i32) } as *mut u8;
    if a_pg == core::ptr::null_mut() { return 7; }
    a_tmp = unsafe { a_pg.offset(n_max as isize) };
    n_blk = ((n_sz_1 + n_max as i64 - 1 as i64) / n_max as i64) as i32;
    if n_blk > n_max_blk { n_blk = n_max_blk as i32; }
    '__b10: loop {
        '__c10: loop {
            {
                i_blk = 0;
                '__b11: loop {
                    if !(rc == 0 && i_blk < n_blk) { break '__b11; }
                    '__c11: loop {
                        let n_byte: i32 =
                            if n_sz_1 >= ((i_blk + 1) * n_max as i32) as i64 {
                                    n_max as i64
                                } else { n_sz_1 % n_max as i64 } as i32;
                        unsafe { memset(a_pg as *mut (), 0, n_max as u64) };
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_fd_1).p_methods }).x_read.unwrap()
                                    })(p_fd_1, a_pg as *mut (), n_byte,
                                    (i_blk * n_max as i32) as i64)
                            };
                        if rc == 0 {
                            let mut pgsz2: i32 = 0;
                            {
                                pgsz2 =
                                    if pgsz != 0 { pgsz * 2 as u32 } else { n_min as u32 } as
                                        i32;
                                '__b12: loop {
                                    if !(pgsz2 <= n_max) { break '__b12; }
                                    '__c12: loop {
                                        let mut i_off: i32 = 0;
                                        {
                                            i_off = 0;
                                            '__b13: loop {
                                                if !(i_off < n_max) { break '__b13; }
                                                '__c13: loop {
                                                    if recover_is_valid_page(a_tmp,
                                                                unsafe {
                                                                    let __p =
                                                                        unsafe { &mut *a_pg.offset(i_off as isize) } as *const u8;
                                                                    if __p.is_null() {
                                                                        &[]
                                                                    } else {
                                                                        core::slice::from_raw_parts(__p,
                                                                            (pgsz2 as u32 - n_reserve_1) as usize)
                                                                    }
                                                                }) != 0 {
                                                        pgsz = pgsz2 as u32;
                                                        break '__b13;
                                                    }
                                                    break '__c13;
                                                }
                                                i_off += pgsz2;
                                            }
                                        }
                                        break '__c12;
                                    }
                                    pgsz2 = pgsz2 * 2;
                                }
                            }
                        }
                        break '__c11;
                    }
                    { let __p = &mut i_blk; let __t = *__p; *__p += 1; __t };
                }
            }
            if pgsz > (*p).detected_pgsz as u32 {
                (*p).detected_pgsz = pgsz as i32;
                (*p).n_reserve = n_reserve_1 as i32;
            }
            if n_reserve_1 == 0 as u32 { break '__b10; }
            n_reserve_1 = 0 as u32;
            break '__c10;
        }
    }
    (*p).detected_pgsz = pgsz as i32;
    unsafe { sqlite3_free(a_pg as *mut ()) };
    return rc;
}

///* This function is a no-op if the recover handle passed as the first 
///* argument already contains an error (if p->errCode!=SQLITE_OK). 
///*
///* Otherwise, an attempt is made to allocate, zero and return a buffer nByte
///* bytes in size. If successful, a pointer to the new buffer is returned. Or,
///* if an OOM error occurs, NULL is returned and the handle error code
///* (p->errCode) set to SQLITE_NOMEM.
extern "C" fn recover_malloc(p: &mut Sqlite3Recover, n_byte_1: i64)
    -> *mut () {
    let mut p_ret: *mut () = core::ptr::null_mut();
    if !(n_byte_1 > 0 as i64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverMalloc".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 344,
                c"nByte>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if (*p).err_code == 0 {
        p_ret = unsafe { sqlite3_malloc64(n_byte_1 as Sqlite3Uint64) };
        if !(p_ret).is_null() {
            unsafe { memset(p_ret, 0, n_byte_1 as u64) };
        } else { (*p).err_code = 7; }
    }
    return p_ret;
}

///* Write value v to buffer a[] as a 32-bit big-endian unsigned integer.
extern "C" fn recover_put_u32(a: *mut u8, v: u32) -> () {
    unsafe { *a.offset(0 as isize) = (v >> 24 & 255 as u32) as u8 };
    unsafe { *a.offset(1 as isize) = (v >> 16 & 255 as u32) as u8 };
    unsafe { *a.offset(2 as isize) = (v >> 8 & 255 as u32) as u8 };
    unsafe { *a.offset(3 as isize) = (v >> 0 & 255 as u32) as u8 };
}

///* Write value v to buffer a[] as a 16-bit big-endian unsigned integer.
extern "C" fn recover_put_u16(a: *mut u8, v: u32) -> () {
    unsafe { *a.offset(0 as isize) = (v >> 8 & 255 as u32) as u8 };
    unsafe { *a.offset(1 as isize) = (v >> 0 & 255 as u32) as u8 };
}

///* The xRead() method of the wrapper VFS. This is used to intercept calls
///* to read page 1 of the input database.
#[allow(unused_doc_comments)]
extern "C" fn recover_vfs_read(p_fd: *mut Sqlite3File, a_buf: *mut (),
    n_byte: i32, i_off: Sqlite3Int64) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_read.unwrap()
                        })(p_fd, a_buf, n_byte, i_off)
                };
            if n_byte == 16 {
                unsafe { sqlite3_randomness(16, a_buf) };
            } else if rc == 0 && i_off == 0 as i64 && n_byte >= 108 {
                /// Ensure that the database has a valid header file. The only fields
                ///* that really matter to recovery are:
                ///*
                ///*   + Database page size (16-bits at offset 16)
                ///*   + Size of db in pages (32-bits at offset 28)
                ///*   + Database encoding (32-bits at offset 56)
                ///*
                ///* Also preserved are:
                ///*
                ///*   + first freelist page (32-bits at offset 32)
                ///*   + size of freelist (32-bits at offset 36)
                ///*   + the wal-mode flags (16-bits at offset 18)
                ///*
                ///* We also try to preserve the auto-vacuum, incr-value, user-version
                ///* and application-id fields - all 32 bit quantities at offsets 
                ///* 52, 60, 64 and 68. All other fields are set to known good values.
                ///*
                ///* Byte offset 105 should also contain the page-size as a 16-bit 
                ///* integer.
                let a_preserve: [i32; 6] = [32, 36, 52, 60, 64, 68];
                let mut a_hdr: [u8; 108] =
                    [83 as u8, 81 as u8, 76 as u8, 105 as u8, 116 as u8,
                            101 as u8, 32 as u8, 102 as u8, 111 as u8, 114 as u8,
                            109 as u8, 97 as u8, 116 as u8, 32 as u8, 51 as u8, 0 as u8,
                            255 as u8, 255 as u8, 1 as u8, 1 as u8, 0 as u8, 64 as u8,
                            32 as u8, 32 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
                            255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                            255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                            255 as u8, 255 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
                            0 as u8, 0 as u8, 0 as u8, 4 as u8, 0 as u8, 0 as u8,
                            16 as u8, 0 as u8, 255 as u8, 255 as u8, 255 as u8,
                            255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                            255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                            255 as u8, 255 as u8, 255 as u8, 255 as u8, 255 as u8,
                            255 as u8, 255 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
                            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
                            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
                            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
                            0 as u8, 0 as u8, 0 as u8, 46 as u8, 91 as u8, 48 as u8,
                            13 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 255 as u8,
                            255 as u8, 0 as u8];
                let a: *mut u8 = a_buf as *mut u8;
                let mut pgsz: u32 =
                    recover_get_u16(unsafe { &raw mut *a.offset(16 as isize) }
                            as *const u8);
                let mut n_reserve: u32 =
                    unsafe { *a.offset(20 as isize) } as u32;
                let mut enc: u32 =
                    recover_get_u32(unsafe { &raw mut *a.offset(56 as isize) }
                            as *const u8);
                let mut dbsz: u32 = 0 as u32;
                let mut db_file_size: i64 = 0 as i64;
                let mut ii: i32 = 0;
                let p: *mut Sqlite3Recover = recover_g.p;
                if pgsz == 1 as u32 { pgsz = 65536 as u32; }
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_fd).p_methods }).x_file_size.unwrap()
                            })(p_fd, &mut db_file_size)
                    };
                if rc == 0 && unsafe { (*p).detected_pgsz } == 0 {
                    rc =
                        recover_vfs_detect_pagesize(unsafe { &mut *p }, p_fd,
                            n_reserve, db_file_size);
                }
                if unsafe { (*p).detected_pgsz } != 0 {
                    pgsz = unsafe { (*p).detected_pgsz } as u32;
                    n_reserve = unsafe { (*p).n_reserve } as u32;
                }
                if pgsz != 0 { dbsz = (db_file_size / pgsz as i64) as u32; }
                if enc != 1 as u32 && enc != 3 as u32 && enc != 2 as u32 {
                    enc = 1 as u32;
                }
                unsafe {
                    sqlite3_free(unsafe { (*p).p_page1_cache } as *mut ())
                };
                unsafe { (*p).p_page1_cache = core::ptr::null_mut() };
                unsafe { (*p).p_page1_disk = core::ptr::null_mut() };
                unsafe { (*p).pgsz = n_byte };
                unsafe {
                    (*p).p_page1_cache =
                        recover_malloc(unsafe { &mut *p }, (n_byte * 2) as i64) as
                            *mut u8
                };
                if !(unsafe { (*p).p_page1_cache }).is_null() {
                    unsafe {
                        (*p).p_page1_disk =
                            unsafe {
                                unsafe { (*p).p_page1_cache.offset(n_byte as isize) }
                            }
                    };
                    unsafe {
                        memcpy(unsafe { (*p).p_page1_disk } as *mut (),
                            a_buf as *const (), n_byte as u64)
                    };
                    a_hdr[18 as usize] = unsafe { *a.offset(18 as isize) };
                    a_hdr[19 as usize] = unsafe { *a.offset(19 as isize) };
                    recover_put_u32(&mut a_hdr[28 as usize], dbsz);
                    recover_put_u32(&mut a_hdr[56 as usize], enc);
                    recover_put_u16(&mut a_hdr[105 as usize], pgsz - n_reserve);
                    if pgsz == 65536 as u32 { pgsz = 1 as u32; }
                    recover_put_u16(&mut a_hdr[16 as usize], pgsz);
                    a_hdr[20 as usize] = n_reserve as u8;
                    {
                        ii = 0;
                        '__b14: loop {
                            if !(ii <
                                            (core::mem::size_of::<[i32; 6]>() as u64 /
                                                    core::mem::size_of::<i32>() as u64) as i32) {
                                break '__b14;
                            }
                            '__c14: loop {
                                unsafe {
                                    memcpy(&raw mut a_hdr[a_preserve[ii as usize] as usize] as
                                            *mut (),
                                        unsafe {
                                                &raw mut *a.offset(a_preserve[ii as usize] as isize)
                                            } as *const (), 4 as u64)
                                };
                                break '__c14;
                            }
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        memcpy(a_buf,
                            &raw mut a_hdr[0 as usize] as *mut u8 as *const (),
                            core::mem::size_of::<[u8; 108]>() as u64)
                    };
                    unsafe {
                        memset(unsafe {
                                    &raw mut *(a_buf as
                                                    *mut u8).add(core::mem::size_of::<[u8; 108]>() as usize)
                                } as *mut (), 0,
                            n_byte as u64 - core::mem::size_of::<[u8; 108]>() as u64)
                    };
                    unsafe {
                        memcpy(unsafe { (*p).p_page1_cache } as *mut (),
                            a_buf as *const (), n_byte as u64)
                    };
                } else { rc = unsafe { (*p).err_code }; }
            }
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_read.unwrap()
                        })(p_fd, a_buf, n_byte, i_off)
                };
        }
        return rc;
    }
}

///* Methods of the wrapper VFS. All methods except for xRead() and xClose()
///* simply uninstall the sqlite3_io_methods wrapper, invoke the equivalent
///* method on the lower level VFS, then reinstall the wrapper before returning.
///* Those that return an integer value use the RECOVER_VFS_WRAPPER macro.
extern "C" fn recover_vfs_write(p_fd: *mut Sqlite3File, a_buf: *const (),
    n_byte: i32, i_off: Sqlite3Int64) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_write.unwrap()
                        })(p_fd, a_buf, n_byte, i_off)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_write.unwrap()
                        })(p_fd, a_buf, n_byte, i_off)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_truncate(p_fd: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_truncate.unwrap()
                        })(p_fd, size)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_truncate.unwrap()
                        })(p_fd, size)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_sync(p_fd: *mut Sqlite3File, flags: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_sync.unwrap()
                        })(p_fd, flags)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_sync.unwrap()
                        })(p_fd, flags)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_file_size(p_fd: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_file_size.unwrap()
                        })(p_fd, p_size)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_file_size.unwrap()
                        })(p_fd, p_size)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_lock(p_fd: *mut Sqlite3File, e_lock: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_lock.unwrap()
                        })(p_fd, e_lock)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_lock.unwrap()
                        })(p_fd, e_lock)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_unlock(p_fd: *mut Sqlite3File, e_lock: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_unlock.unwrap()
                        })(p_fd, e_lock)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_unlock.unwrap()
                        })(p_fd, e_lock)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_check_reserved_lock(p_fd: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*p_fd).p_methods
                                            }).x_check_reserved_lock.unwrap()
                        })(p_fd, p_res_out)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*p_fd).p_methods
                                            }).x_check_reserved_lock.unwrap()
                        })(p_fd, p_res_out)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_file_control(p_fd: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                if !(unsafe { (*p_fd).p_methods }).is_null() {
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_fd).p_methods }).x_file_control.unwrap()
                            })(p_fd, op, p_arg)
                    }
                } else { 12 };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                if !(unsafe { (*p_fd).p_methods }).is_null() {
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_fd).p_methods }).x_file_control.unwrap()
                            })(p_fd, op, p_arg)
                    }
                } else { 12 };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_sector_size(p_fd: *mut Sqlite3File) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_sector_size.unwrap()
                        })(p_fd)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_sector_size.unwrap()
                        })(p_fd)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_device_characteristics(p_fd: *mut Sqlite3File)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*p_fd).p_methods
                                            }).x_device_characteristics.unwrap()
                        })(p_fd)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*p_fd).p_methods
                                            }).x_device_characteristics.unwrap()
                        })(p_fd)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_shm_map(p_fd: *mut Sqlite3File, i_pg: i32,
    pgsz: i32, b_extend: i32, pp: *mut *mut ()) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_shm_map.unwrap()
                        })(p_fd, i_pg, pgsz, b_extend, pp)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_shm_map.unwrap()
                        })(p_fd, i_pg, pgsz, b_extend, pp)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_shm_lock(p_fd: *mut Sqlite3File, offset: i32,
    n: i32, flags: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_shm_lock.unwrap()
                        })(p_fd, offset, n, flags)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_shm_lock.unwrap()
                        })(p_fd, offset, n, flags)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_shm_barrier(p_fd: *mut Sqlite3File) -> () {
    unsafe {
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            unsafe {
                (unsafe {
                        (*unsafe { (*p_fd).p_methods }).x_shm_barrier.unwrap()
                    })(p_fd)
            };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            unsafe {
                (unsafe {
                        (*unsafe { (*p_fd).p_methods }).x_shm_barrier.unwrap()
                    })(p_fd)
            };
        }
    }
}

extern "C" fn recover_vfs_shm_unmap(p_fd: *mut Sqlite3File, delete_flag: i32)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if unsafe { (*p_fd).p_methods } ==
                &raw mut recover_methods as *const Sqlite3IoMethods {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_shm_unmap.unwrap()
                        })(p_fd, delete_flag)
                };
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_fd).p_methods }).x_shm_unmap.unwrap()
                        })(p_fd, delete_flag)
                };
        }
        return rc;
    }
}

extern "C" fn recover_vfs_fetch(p_fd: *mut Sqlite3File, i_off: Sqlite3Int64,
    i_amt: i32, pp: *mut *mut ()) -> i32 {
    { let _ = p_fd; };
    { let _ = i_off; };
    { let _ = i_amt; };
    unsafe { *pp = core::ptr::null_mut() };
    return 0;
}

extern "C" fn recover_vfs_unfetch(p_fd: *mut Sqlite3File, i_off: Sqlite3Int64,
    p: *mut ()) -> i32 {
    { let _ = p_fd; };
    { let _ = i_off; };
    { let _ = p; };
    return 0;
}

static mut recover_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 2,
        x_close: Some(recover_vfs_close),
        x_read: Some(recover_vfs_read),
        x_write: Some(recover_vfs_write),
        x_truncate: Some(recover_vfs_truncate),
        x_sync: Some(recover_vfs_sync),
        x_file_size: Some(recover_vfs_file_size),
        x_lock: Some(recover_vfs_lock),
        x_unlock: Some(recover_vfs_unlock),
        x_check_reserved_lock: Some(recover_vfs_check_reserved_lock),
        x_file_control: Some(recover_vfs_file_control),
        x_sector_size: Some(recover_vfs_sector_size),
        x_device_characteristics: Some(recover_vfs_device_characteristics),
        x_shm_map: Some(recover_vfs_shm_map),
        x_shm_lock: Some(recover_vfs_shm_lock),
        x_shm_barrier: Some(recover_vfs_shm_barrier),
        x_shm_unmap: Some(recover_vfs_shm_unmap),
        x_fetch: Some(recover_vfs_fetch),
        x_unfetch: Some(recover_vfs_unfetch),
    };

///* Install the VFS wrapper around the file-descriptor open on the input
///* database for recover handle p. Mutex RECOVER_MUTEX_ID must be held
///* when this function is called.
extern "C" fn recover_install_wrapper(p: *mut Sqlite3Recover) -> () {
    unsafe {
        let mut p_fd: *mut Sqlite3File = core::ptr::null_mut();
        if !(recover_g.p_methods == core::ptr::null()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"recoverInstallWrapper".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 2577,
                    c"recover_g.pMethods==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            sqlite3_file_control(unsafe { (*p).db_in },
                unsafe { (*p).z_db } as *const i8, 7,
                &raw mut p_fd as *mut ())
        };
        if !(p_fd == core::ptr::null_mut() ||
                                unsafe { (*p_fd).p_methods } !=
                                    &raw mut recover_methods as *const Sqlite3IoMethods) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"recoverInstallWrapper".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 2580,
                    c"pFd==0 || pFd->pMethods!=&recover_methods".as_ptr() as
                            *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(p_fd).is_null() && !(unsafe { (*p_fd).p_methods }).is_null() {
            let i_version: i32 =
                1 +
                    (unsafe { (*unsafe { (*p_fd).p_methods }).i_version } as i32
                                > 1 &&
                            unsafe {
                                (*unsafe { (*p_fd).p_methods }).x_shm_map.is_some()
                            }) as i32;
            recover_g.p_methods = unsafe { (*p_fd).p_methods };
            recover_g.p = p;
            recover_methods.i_version = i_version;
            unsafe {
                (*p_fd).p_methods =
                    &raw mut recover_methods as *const Sqlite3IoMethods
            };
        }
    }
}

///* Run a single SQL statement in zSql.  If zSql contains two or more
///* SQL statements separated by ';', only the first is run.
///*
///* Return the sqlite3_finalizer() or sqlite3_prepare() result code
///* from running the zSql statement.
extern "C" fn recover_one_stmt(db: *mut Sqlite3, z_sql_1: *const i8) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if z_sql_1 == core::ptr::null() { return 0; }
    rc =
        unsafe {
            sqlite3_prepare_v2(db, z_sql_1, -1, &mut p_stmt,
                core::ptr::null_mut())
        };
    if rc != 0 { unsafe { sqlite3_finalize(p_stmt) }; return rc; }
    while 100 == unsafe { sqlite3_step(p_stmt) } {}
    return unsafe { sqlite3_finalize(p_stmt) };
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). A copy of p->errCode is returned in this 
///* case.
///*
///* Otherwise, execute a single SQL statment in zSql.  Even if zSql contains
///* two or more SQL statements separated by ';', only execute the first one.
///* If successful, return SQLITE_OK.  Or, if an error occurs, leave an error
///* code and message in the recover handle and return a copy of the error code.
extern "C" fn recover_exec(p: *mut Sqlite3Recover, db: *mut Sqlite3,
    z_sql_1: *const i8) -> i32 {
    if unsafe { (*p).err_code } == 0 {
        let rc: i32 = recover_one_stmt(db, z_sql_1);
        if rc != 0 { recover_db_error(p, db); }
    }
    return unsafe { (*p).err_code };
}

///* Transfer the following settings from the input database to the output
///* database:
///*
///*   + page-size,
///*   + auto-vacuum settings,
///*   + database encoding,
///*   + user-version (PRAGMA user_version), and
///*   + application-id (PRAGMA application_id), and
extern "C" fn recover_transfer_settings(p: *mut Sqlite3Recover) -> () {
    let a_pragma: [*const i8; 5] =
        [c"encoding".as_ptr() as *const i8,
                c"page_size".as_ptr() as *const i8,
                c"auto_vacuum".as_ptr() as *const i8,
                c"user_version".as_ptr() as *const i8,
                c"application_id".as_ptr() as *const i8];
    let mut ii: i32 = 0;
    if unsafe { (*p).err_code } == 0 {
        let mut db2: *mut Sqlite3 = core::ptr::null_mut();
        let rc: i32 =
            unsafe {
                sqlite3_open(c"".as_ptr() as *mut i8 as *const i8, &mut db2)
            };
        if rc != 0 { recover_db_error(p, db2); return; }
        {
            ii = 0;
            '__b16: loop {
                if !(ii <
                                (core::mem::size_of::<[*const i8; 5]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64) as i32) {
                    break '__b16;
                }
                '__c16: loop {
                    let z_prag: *const i8 = a_pragma[ii as usize];
                    let mut p1: *mut Sqlite3Stmt = core::ptr::null_mut();
                    p1 =
                        unsafe {
                            recover_prepare_printf(p, unsafe { (*p).db_in },
                                c"PRAGMA %Q.%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p).z_db }, z_prag)
                        };
                    if unsafe { (*p).err_code } == 0 &&
                            unsafe { sqlite3_step(p1) } == 100 {
                        let z_arg: *const i8 =
                            unsafe { sqlite3_column_text(p1, 0) } as *const i8;
                        let z2: *mut i8 =
                            unsafe {
                                recover_m_printf(unsafe { &mut *p },
                                    c"PRAGMA %s = %Q".as_ptr() as *mut i8 as *const i8, z_prag,
                                    z_arg)
                            };
                        recover_sql_callback(p, z2 as *const i8);
                        recover_exec(p, db2, z2 as *const i8);
                        unsafe { sqlite3_free(z2 as *mut ()) };
                        if z_arg == core::ptr::null() {
                            unsafe {
                                recover_error(unsafe { &mut *p }, 7, core::ptr::null())
                            };
                        }
                    }
                    recover_finalize(p, p1);
                    break '__c16;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        recover_exec(p, db2,
            c"CREATE TABLE t1(a)".as_ptr() as *mut i8 as *const i8);
        recover_exec(p, db2,
            c"DROP TABLE t1".as_ptr() as *mut i8 as *const i8);
        if unsafe { (*p).err_code } == 0 {
            let db: *mut Sqlite3 = unsafe { (*p).db_out };
            let p_backup: *mut Sqlite3Backup =
                unsafe {
                    sqlite3_backup_init(db,
                        c"main".as_ptr() as *mut i8 as *const i8, db2,
                        c"main".as_ptr() as *mut i8 as *const i8)
                };
            if !(p_backup).is_null() {
                unsafe { sqlite3_backup_step(p_backup, -1) };
                unsafe {
                    (*p).err_code = unsafe { sqlite3_backup_finish(p_backup) }
                };
            } else { recover_db_error(p, db); }
        }
        unsafe { sqlite3_close(db2) };
    }
}

///* Attach the auxiliary database 'recovery' to the output database handle.
///* This temporary database is used during the recovery process and then 
///* discarded.
extern "C" fn recover_open_recovery(p: *mut Sqlite3Recover) -> () {
    let z_sql: *mut i8 =
        unsafe {
            recover_m_printf(unsafe { &mut *p },
                c"ATTACH %Q AS recovery;".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p).z_state_db })
        };
    recover_exec(p, unsafe { (*p).db_out }, z_sql as *const i8);
    unsafe { sqlite3_free(z_sql as *mut ()) };
    recover_exec(p, unsafe { (*p).db_out },
        c"PRAGMA writable_schema = 1".as_ptr() as *mut i8 as *const i8);
    recover_exec(p, unsafe { (*p).db_out },
        c"CREATE TABLE recovery.map(pgno INTEGER PRIMARY KEY, parent INT)".as_ptr()
                as *mut i8 as *const i8);
    recover_exec(p, unsafe { (*p).db_out },
        c"CREATE TABLE recovery.schema(type, name, tbl_name, rootpage, sql)".as_ptr()
                as *mut i8 as *const i8);
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). A copy of the error code is returned in
///* this case. 
///*
///* Otherwise, attempt to populate temporary table "recovery.schema" with the
///* parts of the database schema that can be extracted from the input database.
///*
///* If no error occurs, SQLITE_OK is returned. Otherwise, an error code
///* and error message are left in the recover handle and a copy of the
///* error code returned. It is not considered an error if part of all of
///* the database schema cannot be recovered due to corruption.
extern "C" fn recover_cache_schema(p: *mut Sqlite3Recover) -> i32 {
    return recover_exec(p, unsafe { (*p).db_out },
            c"WITH RECURSIVE pages(p) AS (  SELECT 1    UNION  SELECT child FROM sqlite_dbptr(\'getpage()\'), pages WHERE pgno=p)INSERT INTO recovery.schema SELECT  max(CASE WHEN field=0 THEN value ELSE NULL END),  max(CASE WHEN field=1 THEN value ELSE NULL END),  max(CASE WHEN field=2 THEN value ELSE NULL END),  max(CASE WHEN field=3 THEN value ELSE NULL END),  max(CASE WHEN field=4 THEN value ELSE NULL END)FROM sqlite_dbdata(\'getpage()\') WHERE pgno IN (  SELECT p FROM pages) GROUP BY pgno, cell".as_ptr()
                    as *mut i8 as *const i8);
}

///* Uninstall the VFS wrapper that was installed around the file-descriptor open
///* on the input database for recover handle p. Mutex RECOVER_MUTEX_ID must be
///* held when this function is called.
extern "C" fn recover_uninstall_wrapper(p: &Sqlite3Recover) -> () {
    unsafe {
        let mut p_fd: *mut Sqlite3File = core::ptr::null_mut();
        unsafe {
            sqlite3_file_control((*p).db_in, (*p).z_db as *const i8, 7,
                &raw mut p_fd as *mut ())
        };
        if !(p_fd).is_null() && !(unsafe { (*p_fd).p_methods }).is_null() {
            unsafe { (*p_fd).p_methods = recover_g.p_methods };
            recover_g.p_methods = core::ptr::null();
            recover_g.p = core::ptr::null_mut();
        }
    }
}

extern "C" fn recover_leave_mutex() -> () {
    unsafe { sqlite3_mutex_leave(unsafe { sqlite3_mutex_alloc(9) }) };
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK).
///*
///* Otherwise, argument zName must be the name of a table that has just been
///* created in the output database. This function queries the output db
///* for the schema of said table, and creates a RecoverTable object to
///* store the schema in memory. The new RecoverTable object is linked into
///* the list at sqlite3_recover.pTblList.
///*
///* Parameter iRoot must be the root page of table zName in the INPUT 
///* database.
extern "C" fn recover_add_table(p: *mut Sqlite3Recover, z_name_1: *const i8,
    i_root_1: i64) -> () {
    let mut p_stmt: *mut Sqlite3Stmt =
        unsafe {
            recover_prepare_printf(p, unsafe { (*p).db_out },
                c"PRAGMA table_xinfo(%Q)".as_ptr() as *mut i8 as *const i8,
                z_name_1)
        };
    if !(p_stmt).is_null() {
        let mut i_pk: i32 = -1;
        let mut i_bind: i32 = 1;
        let mut p_new: *mut RecoverTable = core::ptr::null_mut();
        let mut n_col: i32 = 0;
        let n_name: i32 = recover_strlen(z_name_1);
        let mut n_byte: i32 = 0;
        while unsafe { sqlite3_step(p_stmt) } == 100 {
            { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
            n_byte += unsafe { sqlite3_column_bytes(p_stmt, 1) } + 1;
        }
        n_byte +=
            (core::mem::size_of::<RecoverTable>() as u64 +
                            n_col as u64 * core::mem::size_of::<RecoverColumn>() as u64
                        + n_name as u64 + 1 as u64) as i32;
        recover_reset(p, p_stmt);
        p_new =
            recover_malloc(unsafe { &mut *p }, n_byte as i64) as
                *mut RecoverTable;
        if !(p_new).is_null() {
            let mut i: i32 = 0;
            let mut i_field: i32 = 0;
            let mut csr: *mut i8 = core::ptr::null_mut();
            unsafe {
                (*p_new).a_col =
                    unsafe { &raw mut *p_new.offset(1 as isize) } as
                        *mut RecoverColumn
            };
            unsafe {
                (*p_new).z_tab =
                    {
                        csr =
                            unsafe {
                                    &raw mut *unsafe { (*p_new).a_col.offset(n_col as isize) }
                                } as *mut i8;
                        csr
                    }
            };
            unsafe { (*p_new).n_col = n_col };
            unsafe { (*p_new).i_root = i_root_1 as u32 };
            unsafe {
                memcpy(csr as *mut (), z_name_1 as *const (), n_name as u64)
            };
            {
                let __n = n_name + 1;
                let __p = &mut csr;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            {
                i = 0;
                '__b18: loop {
                    if !(unsafe { sqlite3_step(p_stmt) } == 100) {
                        break '__b18;
                    }
                    '__c18: loop {
                        let i_pkf: i32 = unsafe { sqlite3_column_int(p_stmt, 5) };
                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, 1) };
                        let z: *const i8 =
                            unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
                        let z_type: *const i8 =
                            unsafe { sqlite3_column_text(p_stmt, 2) } as *const i8;
                        let e_hidden: i32 =
                            unsafe { sqlite3_column_int(p_stmt, 6) };
                        if i_pk == -1 && i_pkf == 1 &&
                                (unsafe {
                                                sqlite3_stricmp(c"integer".as_ptr() as *mut i8 as *const i8,
                                                    z_type)
                                            } == 0) as i32 != 0 {
                            i_pk = i;
                        }
                        if i_pkf > 1 { i_pk = -2; }
                        unsafe {
                            (*unsafe { (*p_new).a_col.offset(i as isize) }).z_col = csr
                        };
                        unsafe {
                            (*unsafe { (*p_new).a_col.offset(i as isize) }).e_hidden =
                                e_hidden
                        };
                        if e_hidden == 2 {
                            unsafe {
                                (*unsafe { (*p_new).a_col.offset(i as isize) }).i_field = -1
                            };
                        } else {
                            unsafe {
                                (*unsafe { (*p_new).a_col.offset(i as isize) }).i_field =
                                    { let __p = &mut i_field; let __t = *__p; *__p += 1; __t }
                            };
                        }
                        if e_hidden != 2 && e_hidden != 3 {
                            unsafe {
                                (*unsafe { (*p_new).a_col.offset(i as isize) }).i_bind =
                                    { let __p = &mut i_bind; let __t = *__p; *__p += 1; __t }
                            };
                        }
                        unsafe { memcpy(csr as *mut (), z as *const (), n as u64) };
                        {
                            let __n = n + 1;
                            let __p = &mut csr;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        break '__c18;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_new).p_next = unsafe { (*p).p_tbl_list } };
            unsafe { (*p).p_tbl_list = p_new };
            unsafe { (*p_new).b_intkey = 1 };
        }
        recover_finalize(p, p_stmt);
        p_stmt =
            unsafe {
                recover_prepare_printf(p, unsafe { (*p).db_out },
                    c"PRAGMA index_xinfo(%Q)".as_ptr() as *mut i8 as *const i8,
                    z_name_1)
            };
        while !(p_stmt).is_null() && unsafe { sqlite3_step(p_stmt) } == 100 {
            let i_field_1: i32 = unsafe { sqlite3_column_int(p_stmt, 0) };
            let i_col: i32 = unsafe { sqlite3_column_int(p_stmt, 1) };
            if !(i_col < unsafe { (*p_new).n_col }) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"recoverAddTable".as_ptr() as *const i8,
                        c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1144,
                        c"iCol<pNew->nCol".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            unsafe {
                (*unsafe { (*p_new).a_col.offset(i_col as isize) }).i_field =
                    i_field_1
            };
            unsafe { (*p_new).b_intkey = 0 };
            i_pk = -2;
        }
        recover_finalize(p, p_stmt);
        if unsafe { (*p).err_code } == 0 {
            if i_pk >= 0 {
                unsafe {
                    (*unsafe { (*p_new).a_col.offset(i_pk as isize) }).b_ipk = 1
                };
            } else if unsafe { (*p_new).b_intkey } != 0 {
                unsafe {
                    (*p_new).i_rowid_bind =
                        { let __p = &mut i_bind; let __t = *__p; *__p += 1; __t }
                };
            }
        }
    }
}

///* This function is called after recoverCacheSchema() has cached those parts
///* of the input database schema that could be recovered in temporary table
///* "recovery.schema". This function creates in the output database copies
///* of all parts of that schema that must be created before the tables can
///* be populated. Specifically, this means:
///*
///*     * all tables that are not VIRTUAL, and
///*     * UNIQUE indexes.
///*
///* If the recovery handle uses SQL callbacks, then callbacks containing
///* the associated "CREATE TABLE" and "CREATE INDEX" statements are made.
///*
///* Additionally, records are added to the sqlite_schema table of the
///* output database for any VIRTUAL tables. The CREATE VIRTUAL TABLE
///* records are written directly to sqlite_schema, not actually executed.
///* If the handle is in SQL callback mode, then callbacks are invoked 
///* with equivalent SQL statements.
extern "C" fn recover_write_schema1(p: *mut Sqlite3Recover) -> i32 {
    let mut p_select: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_tblname: *mut Sqlite3Stmt = core::ptr::null_mut();
    p_select =
        recover_prepare(p, unsafe { (*p).db_out },
            c"WITH dbschema(rootpage, name, sql, tbl, isVirtual, isIndex) AS (  SELECT rootpage, name, sql,     type=\'table\',     sql LIKE \'create virtual%\',    (type=\'index\' AND (sql LIKE \'%unique%\' OR ?1))  FROM recovery.schema)SELECT rootpage, tbl, isVirtual, name, sql FROM dbschema   WHERE (tbl OR isIndex) AND sql GLOB \'CREATE *\'  ORDER BY tbl DESC, name==\'sqlite_sequence\' DESC".as_ptr()
                    as *mut i8 as *const i8);
    p_tblname =
        recover_prepare(p, unsafe { (*p).db_out },
            c"SELECT name FROM sqlite_schema WHERE type=\'table\' ORDER BY rowid DESC LIMIT 1".as_ptr()
                    as *mut i8 as *const i8);
    if !(p_select).is_null() {
        unsafe {
            sqlite3_bind_int(p_select, 1, unsafe { (*p).b_slow_indexes })
        };
        while unsafe { sqlite3_step(p_select) } == 100 {
            let i_root: i64 = unsafe { sqlite3_column_int64(p_select, 0) };
            let b_table: i32 = unsafe { sqlite3_column_int(p_select, 1) };
            let b_virtual: i32 = unsafe { sqlite3_column_int(p_select, 2) };
            let z_name: *const i8 =
                unsafe { sqlite3_column_text(p_select, 3) } as *const i8;
            let mut z_sql: *const i8 =
                unsafe { sqlite3_column_text(p_select, 4) } as *const i8;
            let mut z_free: *mut i8 = core::ptr::null_mut();
            let mut rc: i32 = 0;
            if b_virtual != 0 {
                z_sql =
                    {
                            z_free =
                                unsafe {
                                    recover_m_printf(unsafe { &mut *p },
                                        c"INSERT INTO sqlite_schema VALUES(\'table\', %Q, %Q, 0, %Q)".as_ptr()
                                                as *mut i8 as *const i8, z_name, z_name, z_sql)
                                };
                            z_free
                        } as *const i8;
            }
            rc = recover_one_stmt(unsafe { (*p).db_out }, z_sql);
            if rc == 0 {
                recover_sql_callback(p, z_sql);
                if b_table != 0 && (b_virtual == 0) as i32 != 0 {
                    if 100 == unsafe { sqlite3_step(p_tblname) } {
                        let z_tbl: *const i8 =
                            unsafe { sqlite3_column_text(p_tblname, 0) } as *const i8;
                        if !(z_tbl).is_null() {
                            recover_add_table(p, z_tbl, i_root);
                        }
                    }
                    recover_reset(p, p_tblname);
                }
            } else if rc != 1 { recover_db_error(p, unsafe { (*p).db_out }); }
            unsafe { sqlite3_free(z_free as *mut ()) };
        }
    }
    recover_finalize(p, p_select);
    recover_finalize(p, p_tblname);
    return unsafe { (*p).err_code };
}

///* Initialize resources required in RECOVER_STATE_WRITING state - during which
///* tables recovered from the schema of the input database are populated with
///* recovered data.
#[allow(unused_doc_comments)]
extern "C" fn recover_write_data_init(p: *mut Sqlite3Recover) -> i32 {
    let p1: *mut RecoverStateW1 = unsafe { &mut (*p).w1 };
    let mut p_tbl: *const RecoverTable = core::ptr::null();
    let mut n_byte: i32 = 0;

    /// Figure out the maximum number of columns for any table in the schema
    if !(unsafe { (*p1).n_max } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverWriteDataInit".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1699,
                c"p1->nMax==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        p_tbl = unsafe { (*p).p_tbl_list };
        '__b21: loop {
            if !(!(p_tbl).is_null()) { break '__b21; }
            '__c21: loop {
                if unsafe { (*p_tbl).n_col } > unsafe { (*p1).n_max } {
                    unsafe { (*p1).n_max = unsafe { (*p_tbl).n_col } };
                }
                break '__c21;
            }
            p_tbl = unsafe { (*p_tbl).p_next };
        }
    }

    /// Allocate an array of (sqlite3_value*) in which to accumulate the values
    ///* that will be written to the output database in a single row.
    (n_byte =
        (core::mem::size_of::<*mut Sqlite3Value>() as u64 *
                (unsafe { (*p1).n_max } + 1) as u64) as i32);
    unsafe {
        (*p1).ap_val =
            recover_malloc(unsafe { &mut *p }, n_byte as i64) as
                *mut *mut Sqlite3Value
    };
    if unsafe { (*p1).ap_val } == core::ptr::null_mut() {
        return unsafe { (*p).err_code };
    }

    /// Prepare the SELECT to loop through schema tables (pTbls) and the SELECT
    ///* to loop through cells that appear to belong to a single table (pSel).
    unsafe {
        (*p1).p_tbls =
            recover_prepare(p, unsafe { (*p).db_out },
                c"SELECT rootpage FROM recovery.schema   WHERE type=\'table\' AND (sql NOT LIKE \'create virtual%\')  ORDER BY (tbl_name=\'sqlite_sequence\') ASC".as_ptr()
                        as *mut i8 as *const i8)
    };
    unsafe {
        (*p1).p_sel =
            recover_prepare(p, unsafe { (*p).db_out },
                c"WITH RECURSIVE pages(page) AS (  SELECT ?1    UNION  SELECT child FROM sqlite_dbptr(\'getpage()\'), pages     WHERE pgno=page) SELECT page, cell, field, value FROM sqlite_dbdata(\'getpage()\') d, pages p WHERE p.page=d.pgno UNION ALL SELECT 0, 0, 0, 0".as_ptr()
                        as *mut i8 as *const i8)
    };
    return unsafe { (*p).err_code };
}

///* Search the list of RecoverTable objects at p->pTblList for one that
///* has root page iRoot in the input database. If such an object is found,
///* return a pointer to it. Otherwise, return NULL.
extern "C" fn recover_find_table(p: &Sqlite3Recover, i_root_1: u32)
    -> *mut RecoverTable {
    let mut p_ret: *mut RecoverTable = core::ptr::null_mut();
    {
        p_ret = (*p).p_tbl_list;
        '__b22: loop {
            if !(!(p_ret).is_null() && unsafe { (*p_ret).i_root } != i_root_1)
                {
                break '__b22;
            }
            '__c22: loop { break '__c22; }
            p_ret = unsafe { (*p_ret).p_next };
        }
    }
    return p_ret;
}

///* This function is a no-op if recover handle p already contains an error
///* (if p->errCode!=SQLITE_OK). In this case it returns NULL.
///*
///* Otherwise, if the recover handle is configured to create an output
///* database (was created by sqlite3_recover_init()), then this function
///* prepares and returns an SQL statement to INSERT a new record into table
///* pTab, assuming the first nField fields of a record extracted from disk
///* are valid.
///*
///* For example, if table pTab is:
///*
///*     CREATE TABLE name(a, b GENERATED ALWAYS AS (a+1) STORED, c, d, e);
///*
///* And nField is 4, then the SQL statement prepared and returned is:
///*
///*     INSERT INTO (a, c, d) VALUES (?1, ?2, ?3);
///*
///* In this case even though 4 values were extracted from the input db,
///* only 3 are written to the output, as the generated STORED column 
///* cannot be written.
///*
///* If the recover handle is in SQL callback mode, then the SQL statement
///* prepared is such that evaluating it returns a single row containing
///* a single text value - itself an SQL statement similar to the above,
///* except with SQL literals in place of the variables. For example:
///*
///*     SELECT 'INSERT INTO (a, c, d) VALUES (' 
///*          || quote(?1) || ', '
///*          || quote(?2) || ', '
///*          || quote(?3) || ')';
///*
///* In either case, it is the responsibility of the caller to eventually
///* free the statement handle using sqlite3_finalize().
extern "C" fn recover_insert_stmt(p: *mut Sqlite3Recover,
    p_tab_1: &RecoverTable, n_field_1: i32) -> *mut Sqlite3Stmt {
    let mut p_ret: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut z_sep: *const i8 = c"".as_ptr() as *mut i8 as *const i8;
    let mut z_sql_sep: *const i8 = c"".as_ptr() as *mut i8 as *const i8;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut z_final: *mut i8 = core::ptr::null_mut();
    let mut z_bind: *mut i8 = core::ptr::null_mut();
    let mut ii: i32 = 0;
    let b_sql: i32 = if unsafe { (*p).x_sql.is_some() } { 1 } else { 0 };
    if n_field_1 <= 0 { return core::ptr::null_mut(); }
    if !(n_field_1 <= (*p_tab_1).n_col) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverInsertStmt".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1337,
                c"nField<=pTab->nCol".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    z_sql =
        unsafe {
            recover_m_printf(unsafe { &mut *p },
                c"INSERT OR IGNORE INTO %Q(".as_ptr() as *mut i8 as *const i8,
                (*p_tab_1).z_tab)
        };
    if (*p_tab_1).i_rowid_bind != 0 {
        if ((*p_tab_1).b_intkey == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"recoverInsertStmt".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1342,
                    c"pTab->bIntkey".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        z_sql =
            unsafe {
                recover_m_printf(unsafe { &mut *p },
                    c"%z_rowid_".as_ptr() as *mut i8 as *const i8, z_sql)
            };
        if b_sql != 0 {
            z_bind =
                unsafe {
                    recover_m_printf(unsafe { &mut *p },
                        c"%zquote(?%d)".as_ptr() as *mut i8 as *const i8, z_bind,
                        (*p_tab_1).i_rowid_bind)
                };
        } else {
            z_bind =
                unsafe {
                    recover_m_printf(unsafe { &mut *p },
                        c"%z?%d".as_ptr() as *mut i8 as *const i8, z_bind,
                        (*p_tab_1).i_rowid_bind)
                };
        }
        z_sql_sep = c"||\', \'||".as_ptr() as *mut i8 as *const i8;
        z_sep = c", ".as_ptr() as *mut i8 as *const i8;
    }
    {
        ii = 0;
        '__b23: loop {
            if !(ii < n_field_1) { break '__b23; }
            '__c23: loop {
                let e_hidden: i32 =
                    unsafe { (*(*p_tab_1).a_col.offset(ii as isize)).e_hidden };
                if e_hidden != 2 && e_hidden != 3 {
                    if !(unsafe {
                                                    (*(*p_tab_1).a_col.offset(ii as isize)).i_field
                                                } >= 0 &&
                                            unsafe { (*(*p_tab_1).a_col.offset(ii as isize)).i_bind } >=
                                                1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"recoverInsertStmt".as_ptr() as *const i8,
                                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1358,
                                c"pTab->aCol[ii].iField>=0 && pTab->aCol[ii].iBind>=1".as_ptr()
                                        as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    z_sql =
                        unsafe {
                            recover_m_printf(unsafe { &mut *p },
                                c"%z%s%Q".as_ptr() as *mut i8 as *const i8, z_sql, z_sep,
                                unsafe { (*(*p_tab_1).a_col.offset(ii as isize)).z_col })
                        };
                    if b_sql != 0 {
                        z_bind =
                            unsafe {
                                recover_m_printf(unsafe { &mut *p },
                                    c"%z%sescape_crlf(quote(?%d))".as_ptr() as *mut i8 as
                                        *const i8, z_bind, z_sql_sep,
                                    unsafe { (*(*p_tab_1).a_col.offset(ii as isize)).i_bind })
                            };
                        z_sql_sep = c"||\', \'||".as_ptr() as *mut i8 as *const i8;
                    } else {
                        z_bind =
                            unsafe {
                                recover_m_printf(unsafe { &mut *p },
                                    c"%z%s?%d".as_ptr() as *mut i8 as *const i8, z_bind, z_sep,
                                    unsafe { (*(*p_tab_1).a_col.offset(ii as isize)).i_bind })
                            };
                    }
                    z_sep = c", ".as_ptr() as *mut i8 as *const i8;
                }
                break '__c23;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if b_sql != 0 {
        z_final =
            unsafe {
                recover_m_printf(unsafe { &mut *p },
                    c"SELECT %Q || \') VALUES (\' || %s || \')\'".as_ptr() as
                            *mut i8 as *const i8, z_sql, z_bind)
            };
    } else {
        z_final =
            unsafe {
                recover_m_printf(unsafe { &mut *p },
                    c"%s) VALUES (%s)".as_ptr() as *mut i8 as *const i8, z_sql,
                    z_bind)
            };
    }
    p_ret = recover_prepare(p, unsafe { (*p).db_out }, z_final as *const i8);
    unsafe { sqlite3_free(z_sql as *mut ()) };
    unsafe { sqlite3_free(z_bind as *mut ()) };
    unsafe { sqlite3_free(z_final as *mut ()) };
    return p_ret;
}

///* Bind the value pVal to parameter iBind of statement pStmt. Leave an
///* error in the recover handle passed as the first argument if an error
///* (e.g. an OOM) occurs.
extern "C" fn recover_bind_value(p: *mut Sqlite3Recover,
    p_stmt_1: *mut Sqlite3Stmt, i_bind_1: i32, p_val_1: *const Sqlite3Value)
    -> () {
    if unsafe { (*p).err_code } == 0 {
        let rc: i32 =
            unsafe {
                sqlite3_bind_value(p_stmt_1, i_bind_1,
                    p_val_1 as *const Sqlite3Value)
            };
        if rc != 0 {
            unsafe {
                recover_error(unsafe { &mut *p }, rc, core::ptr::null())
            };
        }
    }
}

///* Perform one step (sqlite3_recover_step()) of work for the connection 
///* passed as the only argument, which is guaranteed to be in
///* RECOVER_STATE_WRITING state - during which tables recovered from the
///* schema of the input database are populated with recovered data.
#[allow(unused_doc_comments)]
extern "C" fn recover_write_data_step(p: *mut Sqlite3Recover) -> i32 {
    let p1: *mut RecoverStateW1 = unsafe { &mut (*p).w1 };
    let p_sel: *mut Sqlite3Stmt = unsafe { (*p1).p_sel };
    let ap_val: *mut *mut Sqlite3Value = unsafe { (*p1).ap_val };
    if unsafe { (*p).err_code } == 0 &&
            unsafe { (*p1).p_tab } == core::ptr::null_mut() {
        if unsafe { sqlite3_step(unsafe { (*p1).p_tbls }) } == 100 {
            let i_root: i64 =
                unsafe { sqlite3_column_int64(unsafe { (*p1).p_tbls }, 0) };
            unsafe {
                (*p1).p_tab =
                    recover_find_table(unsafe { &*p }, i_root as u32)
            };
            recover_finalize(p, unsafe { (*p1).p_insert });
            unsafe { (*p1).p_insert = core::ptr::null_mut() };
            if unsafe { (*p1).p_tab } == core::ptr::null_mut() {
                return unsafe { (*p).err_code };
            }
            if unsafe {
                        sqlite3_stricmp(c"sqlite_sequence".as_ptr() as *mut i8 as
                                *const i8,
                            unsafe { (*unsafe { (*p1).p_tab }).z_tab } as *const i8)
                    } == 0 {
                recover_exec(p, unsafe { (*p).db_out },
                    c"DELETE FROM sqlite_sequence".as_ptr() as *mut i8 as
                        *const i8);
                recover_sql_callback(p,
                    c"DELETE FROM sqlite_sequence".as_ptr() as *mut i8 as
                        *const i8);
            }

            /// Bind the root page of this table within the original database to 
            ///* SELECT statement p1->pSel. The SELECT statement will then iterate
            ///* through cells that look like they belong to table pTab.
            unsafe { sqlite3_bind_int64(p_sel, 1, i_root) };
            unsafe { (*p1).n_val = 0 };
            unsafe { (*p1).b_have_rowid = 0 };
            unsafe { (*p1).i_prev_page = -1 as i64 };
            unsafe { (*p1).i_prev_cell = -1 };
        } else { return 101; }
    }
    if !(unsafe { (*p).err_code } != 0 || !(unsafe { (*p1).p_tab }).is_null())
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverWriteDataStep".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1796,
                c"p->errCode!=SQLITE_OK || p1->pTab".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p).err_code } == 0 && unsafe { sqlite3_step(p_sel) } == 100
        {
        let p_tab: *mut RecoverTable = unsafe { (*p1).p_tab };
        let i_page: i64 = unsafe { sqlite3_column_int64(p_sel, 0) };
        let i_cell: i32 = unsafe { sqlite3_column_int(p_sel, 1) };
        let i_field: i32 = unsafe { sqlite3_column_int(p_sel, 2) };
        let p_val: *const Sqlite3Value =
            unsafe { sqlite3_column_value(p_sel, 3) } as *const Sqlite3Value;
        let b_new_cell: i32 =
            (unsafe { (*p1).i_prev_page } != i_page ||
                    unsafe { (*p1).i_prev_cell } != i_cell) as i32;
        if !(b_new_cell == 0 || (i_field == -1 || i_field == 0)) as i32 as i64
                != 0 {
            unsafe {
                __assert_rtn(c"recoverWriteDataStep".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1807,
                    c"bNewCell==0 || (iField==-1 || iField==0)".as_ptr() as
                            *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(b_new_cell != 0 || i_field == unsafe { (*p1).n_val } ||
                                unsafe { (*p1).n_val } == unsafe { (*p_tab).n_col }) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"recoverWriteDataStep".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1808,
                    c"bNewCell || iField==p1->nVal || p1->nVal==pTab->nCol".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if b_new_cell != 0 {
            let mut ii: i32 = 0;
            if unsafe { (*p1).n_val } >= 0 {
                if unsafe { (*p1).p_insert } == core::ptr::null_mut() ||
                        unsafe { (*p1).n_val } != unsafe { (*p1).n_insert } {
                    recover_finalize(p, unsafe { (*p1).p_insert });
                    unsafe {
                        (*p1).p_insert =
                            recover_insert_stmt(p, unsafe { &*p_tab },
                                unsafe { (*p1).n_val })
                    };
                    unsafe { (*p1).n_insert = unsafe { (*p1).n_val } };
                }
                if unsafe { (*p1).n_val } > 0 {
                    let p_insert: *mut Sqlite3Stmt = unsafe { (*p1).p_insert };
                    {
                        ii = 0;
                        '__b24: loop {
                            if !(ii < unsafe { (*p_tab).n_col }) { break '__b24; }
                            '__c24: loop {
                                let p_col: *const RecoverColumn =
                                    unsafe {
                                            &raw mut *unsafe { (*p_tab).a_col.offset(ii as isize) }
                                        } as *const RecoverColumn;
                                let i_bind: i32 = unsafe { (*p_col).i_bind };
                                if i_bind > 0 {
                                    if unsafe { (*p_col).b_ipk } != 0 {
                                        unsafe {
                                            sqlite3_bind_int64(p_insert, i_bind,
                                                unsafe { (*p1).i_rowid })
                                        };
                                    } else if unsafe { (*p_col).i_field } <
                                            unsafe { (*p1).n_val } {
                                        recover_bind_value(p, p_insert, i_bind,
                                            unsafe {
                                                    *ap_val.offset(unsafe { (*p_col).i_field } as isize)
                                                } as *const Sqlite3Value);
                                    }
                                }
                                break '__c24;
                            }
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { (*p).b_recover_rowid } != 0 &&
                                unsafe { (*p_tab).i_rowid_bind } > 0 &&
                            unsafe { (*p1).b_have_rowid } != 0 {
                        unsafe {
                            sqlite3_bind_int64(p_insert,
                                unsafe { (*p_tab).i_rowid_bind }, unsafe { (*p1).i_rowid })
                        };
                    }
                    if 100 == unsafe { sqlite3_step(p_insert) } {
                        let z: *const i8 =
                            unsafe { sqlite3_column_text(p_insert, 0) } as *const i8;
                        recover_sql_callback(p, z);
                    }
                    recover_reset(p, p_insert);
                    if !(unsafe { (*p).err_code } != 0 || !(p_insert).is_null())
                                    as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"recoverWriteDataStep".as_ptr() as *const i8,
                                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1839,
                                c"p->errCode || pInsert".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(p_insert).is_null() {
                        unsafe { sqlite3_clear_bindings(p_insert) };
                    }
                }
            }
            {
                ii = 0;
                '__b25: loop {
                    if !(ii < unsafe { (*p1).n_val }) { break '__b25; }
                    '__c25: loop {
                        unsafe {
                            sqlite3_value_free(unsafe { *ap_val.offset(ii as isize) })
                        };
                        unsafe {
                            *ap_val.offset(ii as isize) = core::ptr::null_mut()
                        };
                        break '__c25;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p1).n_val = -1 };
            unsafe { (*p1).b_have_rowid = 0 };
        }
        if i_page != 0 as i64 {
            if i_field < 0 {
                unsafe {
                    (*p1).i_rowid = unsafe { sqlite3_column_int64(p_sel, 3) }
                };
                if !(unsafe { (*p1).n_val } == -1) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"recoverWriteDataStep".as_ptr() as *const i8,
                            c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1855,
                            c"p1->nVal==-1".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                unsafe { (*p1).n_val = 0 };
                unsafe { (*p1).b_have_rowid = 1 };
            } else if i_field < unsafe { (*p_tab).n_col } {
                if !(unsafe { *ap_val.offset(i_field as isize) } ==
                                        core::ptr::null_mut()) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"recoverWriteDataStep".as_ptr() as *const i8,
                            c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1859,
                            c"apVal[iField]==0".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                unsafe {
                    *ap_val.offset(i_field as isize) =
                        unsafe { sqlite3_value_dup(p_val as *const Sqlite3Value) }
                };
                if unsafe { *ap_val.offset(i_field as isize) } ==
                        core::ptr::null_mut() {
                    unsafe {
                        recover_error(unsafe { &mut *p }, 7, core::ptr::null())
                    };
                }
                unsafe { (*p1).n_val = i_field + 1 };
            } else if unsafe { (*p_tab).n_col } == 0 {
                unsafe { (*p1).n_val = unsafe { (*p_tab).n_col } };
            }
            unsafe { (*p1).i_prev_cell = i_cell };
            unsafe { (*p1).i_prev_page = i_page };
        }
    } else {
        recover_reset(p, p_sel);
        unsafe { (*p1).p_tab = core::ptr::null_mut() };
    }
    return unsafe { (*p).err_code };
}

///* Clean up resources allocated by recoverWriteDataInit() (stuff in 
///* sqlite3_recover.w1).
extern "C" fn recover_write_data_cleanup(p: *mut Sqlite3Recover) -> () {
    let p1: *mut RecoverStateW1 = unsafe { &mut (*p).w1 };
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b26: loop {
            if !(ii < unsafe { (*p1).n_val }) { break '__b26; }
            '__c26: loop {
                unsafe {
                    sqlite3_value_free(unsafe {
                            *unsafe { (*p1).ap_val.offset(ii as isize) }
                        })
                };
                break '__c26;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p1).ap_val } as *mut ()) };
    recover_finalize(p, unsafe { (*p1).p_insert });
    recover_finalize(p, unsafe { (*p1).p_tbls });
    recover_finalize(p, unsafe { (*p1).p_sel });
    unsafe {
        memset(p1 as *mut (), 0,
            core::mem::size_of::<RecoverStateW1>() as u64)
    };
}

///* This function is a no-op if p->errCode is initially other than SQLITE_OK.
///* In this case it returns NULL.
///*
///* Otherwise, an attempt is made to allocate and return a bitmap object
///* large enough to store a bit for all page numbers between 1 and nPg,
///* inclusive. The bitmap is initially zeroed.
extern "C" fn recover_bitmap_alloc(p: *mut Sqlite3Recover, n_pg_1: i64)
    -> *mut RecoverBitmap {
    let n_elem: i32 = ((n_pg_1 + 1 as i64 + 31 as i64) / 32 as i64) as i32;
    let n_byte: i32 =
        (16 as u64 + n_elem as u64 * core::mem::size_of::<u32>() as u64) as
            i32;
    let p_ret: *mut RecoverBitmap =
        recover_malloc(unsafe { &mut *p }, n_byte as i64) as
            *mut RecoverBitmap;
    if !(p_ret).is_null() { unsafe { (*p_ret).n_pg = n_pg_1 }; }
    return p_ret;
}

///* Initialize resources required by sqlite3_recover_step() in
///* RECOVER_STATE_LOSTANDFOUND1 state - during which the set of pages not
///* already allocated to a recovered schema element is determined.
#[allow(unused_doc_comments)]
extern "C" fn recover_lost_and_found1_init(p: *mut Sqlite3Recover) -> () {
    let p_laf: *mut RecoverStateLAF = unsafe { &mut (*p).laf };
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    if !(unsafe { (*p).laf.p_used } == core::ptr::null_mut()) as i32 as i64 !=
            0 {
        unsafe {
            __assert_rtn(c"recoverLostAndFound1Init".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1888,
                c"p->laf.pUsed==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe { (*p_laf).n_pg = recover_page_count(p) };
    unsafe {
        (*p_laf).p_used = recover_bitmap_alloc(p, unsafe { (*p_laf).n_pg })
    };

    /// Prepare a statement to iterate through all pages that are part of any tree
    ///* in the recoverable part of the input database schema to the bitmap. And,
    ///* if !p->bFreelistCorrupt, add all pages that appear to be part of the
    ///* freelist.
    (p_stmt =
        recover_prepare(p, unsafe { (*p).db_out },
            c"WITH trunk(pgno) AS (  SELECT read_i32(getpage(1), 8) AS x WHERE x>0    UNION  SELECT read_i32(getpage(trunk.pgno), 0) AS x FROM trunk WHERE x>0),trunkdata(pgno, data) AS (  SELECT pgno, getpage(pgno) FROM trunk),freelist(data, n, freepgno) AS (  SELECT data, min(16384, read_i32(data, 1)-1), pgno FROM trunkdata    UNION ALL  SELECT data, n-1, read_i32(data, 2+n) FROM freelist WHERE n>=0),roots(r) AS (  SELECT 1 UNION ALL  SELECT rootpage FROM recovery.schema WHERE rootpage>0),used(page) AS (  SELECT r FROM roots    UNION  SELECT child FROM sqlite_dbptr(\'getpage()\'), used     WHERE pgno=page) SELECT page FROM used UNION ALL SELECT freepgno FROM freelist WHERE NOT ?".as_ptr()
                    as *mut i8 as *const i8));
    if !(p_stmt).is_null() {
        unsafe {
            sqlite3_bind_int(p_stmt, 1, unsafe { (*p).b_freelist_corrupt })
        };
    }
    unsafe { (*p_laf).p_used_pages = p_stmt };
}

///* Set the bit associated with page iPg in bitvec pMap.
extern "C" fn recover_bitmap_set(p_map_1: &mut RecoverBitmap, i_pg_1: i64)
    -> () {
    if i_pg_1 <= (*p_map_1).n_pg {
        let i_elem: i32 = (i_pg_1 / 32 as i64) as i32;
        let i_bit: i32 = (i_pg_1 % 32 as i64) as i32;
        unsafe {
            *((*p_map_1).a_elem.as_ptr() as *mut u32).offset(i_elem as isize)
                |= (1 as u32) << i_bit
        };
    }
}

///* Perform one step (sqlite3_recover_step()) of work for the connection 
///* passed as the only argument, which is guaranteed to be in
///* RECOVER_STATE_LOSTANDFOUND1 state - during which the set of pages not
///* already allocated to a recovered schema element is determined.
extern "C" fn recover_lost_and_found1_step(p: *mut Sqlite3Recover) -> i32 {
    let p_laf: *mut RecoverStateLAF = unsafe { &mut (*p).laf };
    let mut rc: i32 = unsafe { (*p).err_code };
    if rc == 0 {
        rc = unsafe { sqlite3_step(unsafe { (*p_laf).p_used_pages }) };
        if rc == 100 {
            let i_pg: i64 =
                unsafe {
                    sqlite3_column_int64(unsafe { (*p_laf).p_used_pages }, 0)
                };
            recover_bitmap_set(unsafe { &mut *unsafe { (*p_laf).p_used } },
                i_pg);
            rc = 0;
        } else {
            recover_finalize(p, unsafe { (*p_laf).p_used_pages });
            unsafe { (*p_laf).p_used_pages = core::ptr::null_mut() };
        }
    }
    return rc;
}

///* Initialize resources required by RECOVER_STATE_LOSTANDFOUND2 
///* state - during which the pages identified in RECOVER_STATE_LOSTANDFOUND1
///* are sorted into sets that likely belonged to the same database tree.
extern "C" fn recover_lost_and_found2_init(p: *mut Sqlite3Recover) -> () {
    let p_laf: *mut RecoverStateLAF = unsafe { &mut (*p).laf };
    if !(unsafe { (*p).laf.p_all_and_parent } == core::ptr::null_mut()) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverLostAndFound2Init".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1961,
                c"p->laf.pAllAndParent==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).laf.p_map_insert } == core::ptr::null_mut()) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverLostAndFound2Init".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1962,
                c"p->laf.pMapInsert==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).laf.p_max_field } == core::ptr::null_mut()) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverLostAndFound2Init".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1963,
                c"p->laf.pMaxField==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).laf.n_max_field } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverLostAndFound2Init".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1964,
                c"p->laf.nMaxField==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        (*p_laf).p_map_insert =
            recover_prepare(p, unsafe { (*p).db_out },
                c"INSERT OR IGNORE INTO recovery.map(pgno, parent) VALUES(?, ?)".as_ptr()
                        as *mut i8 as *const i8)
    };
    unsafe {
        (*p_laf).p_all_and_parent =
            unsafe {
                recover_prepare_printf(p, unsafe { (*p).db_out },
                    c"WITH RECURSIVE seq(ii) AS (  SELECT 1 UNION ALL SELECT ii+1 FROM seq WHERE ii<%lld)SELECT pgno, child FROM sqlite_dbptr(\'getpage()\')  UNION ALL SELECT NULL, ii FROM seq".as_ptr()
                            as *mut i8 as *const i8, unsafe { (*p).laf.n_pg })
            }
    };
    unsafe {
        (*p_laf).p_max_field =
            unsafe {
                recover_prepare_printf(p, unsafe { (*p).db_out },
                    c"SELECT max(field)+1 FROM sqlite_dbdata(\'getpage\') WHERE pgno = ?".as_ptr()
                            as *mut i8 as *const i8)
            }
    };
}

///* Perform one step (sqlite3_recover_step()) of work for the connection 
///* passed as the only argument, which is guaranteed to be in
///* RECOVER_STATE_LOSTANDFOUND2 state - during which the pages identified 
///* in RECOVER_STATE_LOSTANDFOUND1 are sorted into sets that likely belonged 
///* to the same database tree.
extern "C" fn recover_lost_and_found2_step(p: *mut Sqlite3Recover) -> i32 {
    let p_laf: *mut RecoverStateLAF = unsafe { &mut (*p).laf };
    if unsafe { (*p).err_code } == 0 {
        let res: i32 =
            unsafe { sqlite3_step(unsafe { (*p_laf).p_all_and_parent }) };
        if res == 100 {
            let i_child: i64 =
                unsafe {
                        sqlite3_column_int(unsafe { (*p_laf).p_all_and_parent }, 1)
                    } as i64;
            if recover_bitmap_query(unsafe { &*unsafe { (*p_laf).p_used } },
                        i_child) == 0 {
                unsafe {
                    sqlite3_bind_int64(unsafe { (*p_laf).p_map_insert }, 1,
                        i_child)
                };
                unsafe {
                    sqlite3_bind_value(unsafe { (*p_laf).p_map_insert }, 2,
                        unsafe {
                                sqlite3_column_value(unsafe { (*p_laf).p_all_and_parent },
                                    0)
                            } as *const Sqlite3Value)
                };
                unsafe { sqlite3_step(unsafe { (*p_laf).p_map_insert }) };
                recover_reset(p, unsafe { (*p_laf).p_map_insert });
                unsafe {
                    sqlite3_bind_int64(unsafe { (*p_laf).p_max_field }, 1,
                        i_child)
                };
                if 100 ==
                        unsafe { sqlite3_step(unsafe { (*p_laf).p_max_field }) } {
                    let n_max: i32 =
                        unsafe {
                            sqlite3_column_int(unsafe { (*p_laf).p_max_field }, 0)
                        };
                    if n_max > unsafe { (*p_laf).n_max_field } {
                        unsafe { (*p_laf).n_max_field = n_max };
                    }
                }
                recover_reset(p, unsafe { (*p_laf).p_max_field });
            }
        } else {
            recover_finalize(p, unsafe { (*p_laf).p_all_and_parent });
            unsafe { (*p_laf).p_all_and_parent = core::ptr::null_mut() };
            return 101;
        }
    }
    return unsafe { (*p).err_code };
}

///* This function attempts to create a lost and found table within the 
///* output db. If successful, it returns a pointer to a buffer containing
///* the name of the new table. It is the responsibility of the caller to
///* eventually free this buffer using sqlite3_free().
///*
///* If an error occurs, NULL is returned and an error code and error 
///* message left in the recover handle.
extern "C" fn recover_lost_and_found_create(p: *mut Sqlite3Recover,
    n_field_1: i32) -> *mut i8 {
    let mut z_tbl: *mut i8 = core::ptr::null_mut();
    let mut p_probe: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut ii: i32 = 0;
    p_probe =
        recover_prepare(p, unsafe { (*p).db_out },
            c"SELECT 1 FROM sqlite_schema WHERE name=?".as_ptr() as *mut i8 as
                *const i8);
    {
        ii = -1;
        '__b27: loop {
            if !(z_tbl == core::ptr::null_mut() &&
                                unsafe { (*p).err_code } == 0 && ii < 1000) {
                break '__b27;
            }
            '__c27: loop {
                let mut b_fail: i32 = 0;
                if ii < 0 {
                    z_tbl =
                        unsafe {
                            recover_m_printf(unsafe { &mut *p },
                                c"%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p).z_lost_and_found })
                        };
                } else {
                    z_tbl =
                        unsafe {
                            recover_m_printf(unsafe { &mut *p },
                                c"%s_%d".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p).z_lost_and_found }, ii)
                        };
                }
                if unsafe { (*p).err_code } == 0 {
                    unsafe {
                        sqlite3_bind_text(p_probe, 1, z_tbl as *const i8, -1, None)
                    };
                    if 100 == unsafe { sqlite3_step(p_probe) } { b_fail = 1; }
                    recover_reset(p, p_probe);
                }
                if b_fail != 0 {
                    unsafe { sqlite3_clear_bindings(p_probe) };
                    unsafe { sqlite3_free(z_tbl as *mut ()) };
                    z_tbl = core::ptr::null_mut();
                }
                break '__c27;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    recover_finalize(p, p_probe);
    if !(z_tbl).is_null() {
        let mut z_sep: *const i8 = core::ptr::null();
        let mut z_field: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        z_sep =
            c"rootpgno INTEGER, pgno INTEGER, nfield INTEGER, id INTEGER, ".as_ptr()
                    as *mut i8 as *const i8;
        {
            ii = 0;
            '__b28: loop {
                if !(unsafe { (*p).err_code } == 0 && ii < n_field_1) {
                    break '__b28;
                }
                '__c28: loop {
                    z_field =
                        unsafe {
                            recover_m_printf(unsafe { &mut *p },
                                c"%z%sc%d".as_ptr() as *mut i8 as *const i8, z_field, z_sep,
                                ii)
                        };
                    z_sep = c", ".as_ptr() as *mut i8 as *const i8;
                    break '__c28;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        z_sql =
            unsafe {
                recover_m_printf(unsafe { &mut *p },
                    c"CREATE TABLE %s(%s)".as_ptr() as *mut i8 as *const i8,
                    z_tbl, z_field)
            };
        unsafe { sqlite3_free(z_field as *mut ()) };
        recover_exec(p, unsafe { (*p).db_out }, z_sql as *const i8);
        recover_sql_callback(p, z_sql as *const i8);
        unsafe { sqlite3_free(z_sql as *mut ()) };
    } else if unsafe { (*p).err_code } == 0 {
        unsafe {
            recover_error(unsafe { &mut *p }, 1,
                c"failed to create %s output table".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).z_lost_and_found })
        };
    }
    return z_tbl;
}

///* Synthesize and prepare an INSERT statement to write to the lost_and_found
///* table in the output database. The name of the table is zTab, and it has
///* nField c* fields.
extern "C" fn recover_lost_and_found_insert(p: *mut Sqlite3Recover,
    z_tab_1: *const i8, n_field_1: i32) -> *mut Sqlite3Stmt {
    let n_total: i32 = n_field_1 + 4;
    let mut ii: i32 = 0;
    let mut z_bind: *mut i8 = core::ptr::null_mut();
    let mut p_ret: *mut Sqlite3Stmt = core::ptr::null_mut();
    if !unsafe { (*p).x_sql.is_some() } as i32 != 0 {
        {
            ii = 0;
            '__b29: loop {
                if !(ii < n_total) { break '__b29; }
                '__c29: loop {
                    z_bind =
                        unsafe {
                            recover_m_printf(unsafe { &mut *p },
                                c"%z%s?".as_ptr() as *mut i8 as *const i8, z_bind,
                                if !(z_bind).is_null() {
                                    c", ".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 }, ii)
                        };
                    break '__c29;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        p_ret =
            unsafe {
                recover_prepare_printf(p, unsafe { (*p).db_out },
                    c"INSERT INTO %s VALUES(%s)".as_ptr() as *mut i8 as
                        *const i8, z_tab_1, z_bind)
            };
    } else {
        let mut z_sep: *const i8 = c"".as_ptr() as *mut i8 as *const i8;
        {
            ii = 0;
            '__b30: loop {
                if !(ii < n_total) { break '__b30; }
                '__c30: loop {
                    z_bind =
                        unsafe {
                            recover_m_printf(unsafe { &mut *p },
                                c"%z%squote(?)".as_ptr() as *mut i8 as *const i8, z_bind,
                                z_sep)
                        };
                    z_sep = c"|| \', \' ||".as_ptr() as *mut i8 as *const i8;
                    break '__c30;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        p_ret =
            unsafe {
                recover_prepare_printf(p, unsafe { (*p).db_out },
                    c"SELECT \'INSERT INTO %s VALUES(\' || %s || \')\'".as_ptr()
                            as *mut i8 as *const i8, z_tab_1, z_bind)
            };
    }
    unsafe { sqlite3_free(z_bind as *mut ()) };
    return p_ret;
}

///* Initialize resources required in RECOVER_STATE_LOSTANDFOUND3 
///* state - during which the lost-and-found table of the output database 
///* is populated with recovered data that can not be assigned to any 
///* recovered schema object.
#[allow(unused_doc_comments)]
extern "C" fn recover_lost_and_found3_init(p: *mut Sqlite3Recover) -> () {
    let p_laf: *mut RecoverStateLAF = unsafe { &mut (*p).laf };
    if unsafe { (*p_laf).n_max_field } > 0 {
        let mut z_tab: *mut i8 = core::ptr::null_mut();

        /// Name of lost_and_found table
        (z_tab =
            recover_lost_and_found_create(p,
                unsafe { (*p_laf).n_max_field }));
        unsafe {
            (*p_laf).p_insert =
                recover_lost_and_found_insert(p, z_tab as *const i8,
                    unsafe { (*p_laf).n_max_field })
        };
        unsafe { sqlite3_free(z_tab as *mut ()) };
        unsafe {
            (*p_laf).p_all_page =
                unsafe {
                    recover_prepare_printf(p, unsafe { (*p).db_out },
                        c"WITH RECURSIVE seq(ii) AS (  SELECT 1 UNION ALL SELECT ii+1 FROM seq WHERE ii<%lld)SELECT ii FROM seq".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p).laf.n_pg })
                }
        };
        unsafe {
            (*p_laf).p_page_data =
                recover_prepare(p, unsafe { (*p).db_out },
                    c"SELECT cell, field, value FROM sqlite_dbdata(\'getpage()\') d WHERE d.pgno=? UNION ALL SELECT -1, -1, -1".as_ptr()
                            as *mut i8 as *const i8)
        };
        unsafe {
            (*p_laf).ap_val =
                recover_malloc(unsafe { &mut *p },
                        (unsafe { (*p_laf).n_max_field } as u64 *
                                core::mem::size_of::<*mut Sqlite3Value>() as u64) as i64) as
                    *mut *mut Sqlite3Value
        };
    }
}

///* Input database page iPg contains data that will be written to the
///* lost-and-found table of the output database. This function attempts
///* to identify the root page of the tree that page iPg belonged to.
///* If successful, it sets output variable (*piRoot) to the page number
///* of the root page and returns SQLITE_OK. Otherwise, if an error occurs,
///* an SQLite error code is returned and the final value of *piRoot 
///* undefined.
extern "C" fn recover_lost_and_found_find_root(p: *mut Sqlite3Recover,
    i_pg_1: i64, pi_root_1: &mut i64) -> i32 {
    let p_laf: *mut RecoverStateLAF = unsafe { &mut (*p).laf };
    if unsafe { (*p_laf).p_find_root } == core::ptr::null_mut() {
        unsafe {
            (*p_laf).p_find_root =
                recover_prepare(p, unsafe { (*p).db_out },
                    c"WITH RECURSIVE p(pgno) AS (  SELECT ?    UNION  SELECT parent FROM recovery.map AS m, p WHERE m.pgno=p.pgno) SELECT p.pgno FROM p, recovery.map m WHERE m.pgno=p.pgno     AND m.parent IS NULL".as_ptr()
                            as *mut i8 as *const i8)
        };
    }
    if unsafe { (*p).err_code } == 0 {
        unsafe {
            sqlite3_bind_int64(unsafe { (*p_laf).p_find_root }, 1, i_pg_1)
        };
        if unsafe { sqlite3_step(unsafe { (*p_laf).p_find_root }) } == 100 {
            *pi_root_1 =
                unsafe {
                    sqlite3_column_int64(unsafe { (*p_laf).p_find_root }, 0)
                };
        } else { *pi_root_1 = i_pg_1; }
        recover_reset(p, unsafe { (*p_laf).p_find_root });
    }
    return unsafe { (*p).err_code };
}

///* Recover data from page iPage of the input database and write it to
///* the lost-and-found table in the output database.
#[allow(unused_doc_comments)]
extern "C" fn recover_lost_and_found_one_page(p: *mut Sqlite3Recover,
    i_page_1: i64) -> () {
    let p_laf: *const RecoverStateLAF =
        unsafe { &raw mut (*p).laf } as *const RecoverStateLAF;
    let ap_val: *mut *mut Sqlite3Value = unsafe { (*p_laf).ap_val };
    let p_page_data: *mut Sqlite3Stmt = unsafe { (*p_laf).p_page_data };
    let p_insert: *mut Sqlite3Stmt = unsafe { (*p_laf).p_insert };
    let mut n_val: i32 = -1;
    let mut i_prev_cell: i32 = 0;
    let mut i_root: i64 = 0 as i64;
    let mut b_have_rowid: i32 = 0;
    let mut i_rowid: i64 = 0 as i64;
    let mut ii: i32 = 0;
    if recover_lost_and_found_find_root(p, i_page_1, &mut i_root) != 0 {
        return;
    }
    unsafe { sqlite3_bind_int64(p_page_data, 1, i_page_1) };
    while unsafe { (*p).err_code } == 0 &&
            100 == unsafe { sqlite3_step(p_page_data) } {
        let i_cell: i32 =
            unsafe { sqlite3_column_int64(p_page_data, 0) } as i32;
        let i_field: i32 =
            unsafe { sqlite3_column_int64(p_page_data, 1) } as i32;
        if i_prev_cell != i_cell && n_val >= 0 {

            /// Insert the new row
            unsafe { sqlite3_bind_int64(p_insert, 1, i_root) };

            /// rootpgno
            unsafe { sqlite3_bind_int64(p_insert, 2, i_page_1) };

            /// pgno
            unsafe { sqlite3_bind_int(p_insert, 3, n_val) };
            if b_have_rowid != 0 {
                unsafe { sqlite3_bind_int64(p_insert, 4, i_rowid) };
            }
            {
                ii = 0;
                '__b32: loop {
                    if !(ii < n_val) { break '__b32; }
                    '__c32: loop {
                        recover_bind_value(p, p_insert, 5 + ii,
                            unsafe { *ap_val.offset(ii as isize) } as
                                *const Sqlite3Value);
                        break '__c32;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            if unsafe { sqlite3_step(p_insert) } == 100 {
                recover_sql_callback(p,
                    unsafe { sqlite3_column_text(p_insert, 0) } as *const i8);
            }
            recover_reset(p, p_insert);
            {
                ii = 0;
                '__b33: loop {
                    if !(ii < n_val) { break '__b33; }
                    '__c33: loop {
                        unsafe {
                            sqlite3_value_free(unsafe { *ap_val.offset(ii as isize) })
                        };
                        unsafe {
                            *ap_val.offset(ii as isize) = core::ptr::null_mut()
                        };
                        break '__c33;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { sqlite3_clear_bindings(p_insert) };
            b_have_rowid = 0;
            n_val = -1;
        }
        if i_cell < 0 { break; }
        if i_field < 0 {
            if !(n_val == -1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"recoverLostAndFoundOnePage".as_ptr() as
                            *const i8,
                        c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1599,
                        c"nVal==-1".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            i_rowid = unsafe { sqlite3_column_int64(p_page_data, 2) };
            b_have_rowid = 1;
            n_val = 0;
        } else if i_field < unsafe { (*p_laf).n_max_field } {
            let p_val: *const Sqlite3Value =
                unsafe { sqlite3_column_value(p_page_data, 2) } as
                    *const Sqlite3Value;
            unsafe {
                *ap_val.offset(i_field as isize) =
                    unsafe { sqlite3_value_dup(p_val as *const Sqlite3Value) }
            };
            if !(i_field == n_val || n_val == -1 && i_field == 0) as i32 as
                        i64 != 0 {
                unsafe {
                    __assert_rtn(c"recoverLostAndFoundOnePage".as_ptr() as
                            *const i8,
                        c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 1606,
                        c"iField==nVal || (nVal==-1 && iField==0)".as_ptr() as
                                *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            n_val = i_field + 1;
            if unsafe { *ap_val.offset(i_field as isize) } ==
                    core::ptr::null_mut() {
                unsafe {
                    recover_error(unsafe { &mut *p }, 7, core::ptr::null())
                };
            }
        }
        i_prev_cell = i_cell;
    }
    recover_reset(p, p_page_data);
    {
        ii = 0;
        '__b34: loop {
            if !(ii < n_val) { break '__b34; }
            '__c34: loop {
                unsafe {
                    sqlite3_value_free(unsafe { *ap_val.offset(ii as isize) })
                };
                unsafe {
                    *ap_val.offset(ii as isize) = core::ptr::null_mut()
                };
                break '__c34;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Perform one step (sqlite3_recover_step()) of work for the connection 
///* passed as the only argument, which is guaranteed to be in
///* RECOVER_STATE_LOSTANDFOUND3 state - during which the lost-and-found 
///* table of the output database is populated with recovered data that can 
///* not be assigned to any recovered schema object.
extern "C" fn recover_lost_and_found3_step(p: *mut Sqlite3Recover) -> i32 {
    let p_laf: *const RecoverStateLAF =
        unsafe { &raw mut (*p).laf } as *const RecoverStateLAF;
    if unsafe { (*p).err_code } == 0 {
        if unsafe { (*p_laf).p_insert } == core::ptr::null_mut() {
            return 101;
        } else {
            if unsafe { (*p).err_code } == 0 {
                let res: i32 =
                    unsafe { sqlite3_step(unsafe { (*p_laf).p_all_page }) };
                if res == 100 {
                    let i_page: i64 =
                        unsafe {
                            sqlite3_column_int64(unsafe { (*p_laf).p_all_page }, 0)
                        };
                    if recover_bitmap_query(unsafe {
                                    &*unsafe { (*p_laf).p_used }
                                }, i_page) == 0 {
                        recover_lost_and_found_one_page(p, i_page);
                    }
                } else {
                    recover_reset(p, unsafe { (*p_laf).p_all_page });
                    return 101;
                }
            }
        }
    }
    return 0;
}

///* This function is called after the output database has been populated. It
///* adds all recovered schema elements that were not created in the output
///* database by recoverWriteSchema1() - everything except for tables and
///* UNIQUE indexes. Specifically:
///*
///*     * views,
///*     * triggers,
///*     * non-UNIQUE indexes.
///*
///* If the recover handle is in SQL callback mode, then equivalent callbacks
///* are issued to create the schema elements.
extern "C" fn recover_write_schema2(p: *mut Sqlite3Recover) -> i32 {
    let mut p_select: *mut Sqlite3Stmt = core::ptr::null_mut();
    p_select =
        recover_prepare(p, unsafe { (*p).db_out },
            if unsafe { (*p).b_slow_indexes } != 0 {
                    c"SELECT rootpage, sql FROM recovery.schema   WHERE type!=\'table\' AND type!=\'index\'    AND sql GLOB \'CREATE *\'".as_ptr()
                        as *mut i8
                } else {
                    c"SELECT rootpage, sql FROM recovery.schema   WHERE type!=\'table\' AND (type!=\'index\' OR sql NOT LIKE \'%unique%\')    AND sql GLOB \'CREATE *\'".as_ptr()
                        as *mut i8
                } as *const i8);
    if !(p_select).is_null() {
        while unsafe { sqlite3_step(p_select) } == 100 {
            let z_sql: *const i8 =
                unsafe { sqlite3_column_text(p_select, 1) } as *const i8;
            let rc: i32 = recover_one_stmt(unsafe { (*p).db_out }, z_sql);
            if rc == 0 {
                recover_sql_callback(p, z_sql);
            } else if rc != 1 { recover_db_error(p, unsafe { (*p).db_out }); }
        }
    }
    recover_finalize(p, p_select);
    return unsafe { (*p).err_code };
}

///* Free a bitmap object allocated by recoverBitmapAlloc().
extern "C" fn recover_bitmap_free(p_map_1: *mut RecoverBitmap) -> () {
    unsafe { sqlite3_free(p_map_1 as *mut ()) };
}

///* Free all resources allocated as part of sqlite3_recover_step() calls
///* in one of the RECOVER_STATE_LOSTANDFOUND[123] states.
extern "C" fn recover_lost_and_found_cleanup(p: &mut Sqlite3Recover) -> () {
    recover_bitmap_free((*p).laf.p_used);
    (*p).laf.p_used = core::ptr::null_mut();
    unsafe { sqlite3_finalize((*p).laf.p_used_pages) };
    unsafe { sqlite3_finalize((*p).laf.p_all_and_parent) };
    unsafe { sqlite3_finalize((*p).laf.p_map_insert) };
    unsafe { sqlite3_finalize((*p).laf.p_max_field) };
    unsafe { sqlite3_finalize((*p).laf.p_find_root) };
    unsafe { sqlite3_finalize((*p).laf.p_insert) };
    unsafe { sqlite3_finalize((*p).laf.p_all_page) };
    unsafe { sqlite3_finalize((*p).laf.p_page_data) };
    (*p).laf.p_used_pages = core::ptr::null_mut();
    (*p).laf.p_all_and_parent = core::ptr::null_mut();
    (*p).laf.p_map_insert = core::ptr::null_mut();
    (*p).laf.p_max_field = core::ptr::null_mut();
    (*p).laf.p_find_root = core::ptr::null_mut();
    (*p).laf.p_insert = core::ptr::null_mut();
    (*p).laf.p_all_page = core::ptr::null_mut();
    (*p).laf.p_page_data = core::ptr::null_mut();
    unsafe { sqlite3_free((*p).laf.ap_val as *mut ()) };
    (*p).laf.ap_val = core::ptr::null_mut();
}

///* Free all resources allocated as part of sqlite3_recover_step() calls.
extern "C" fn recover_final_cleanup(p: *mut Sqlite3Recover) -> () {
    let mut p_tab: *mut RecoverTable = core::ptr::null_mut();
    let mut p_next: *mut RecoverTable = core::ptr::null_mut();
    recover_write_data_cleanup(p);
    recover_lost_and_found_cleanup(unsafe { &mut *p });
    {
        p_tab = unsafe { (*p).p_tbl_list };
        '__b36: loop {
            if !(!(p_tab).is_null()) { break '__b36; }
            '__c36: loop {
                p_next = unsafe { (*p_tab).p_next };
                unsafe { sqlite3_free(p_tab as *mut ()) };
                break '__c36;
            }
            p_tab = p_next;
        }
    }
    unsafe { (*p).p_tbl_list = core::ptr::null_mut() };
    unsafe { sqlite3_finalize(unsafe { (*p).p_get_page }) };
    unsafe { (*p).p_get_page = core::ptr::null_mut() };
    unsafe {
        sqlite3_file_control(unsafe { (*p).db_in },
            unsafe { (*p).z_db } as *const i8, 42, core::ptr::null_mut())
    };
    {
        let res: i32 = unsafe { sqlite3_close(unsafe { (*p).db_out }) };
        if !(res == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"recoverFinalCleanup".as_ptr() as *const i8,
                    c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 2069,
                    c"res==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    unsafe { (*p).db_out = core::ptr::null_mut() };
}

///* This function does the work of a single sqlite3_recover_step() call. It
///* is guaranteed that the handle is not in an error state when this
///* function is called.
#[allow(unused_doc_comments)]
extern "C" fn recover_step(p: *mut Sqlite3Recover) -> () {
    if !(!(p).is_null() && unsafe { (*p).err_code } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"recoverStep".as_ptr() as *const i8,
                c"sqlite3recover.c".as_ptr() as *mut i8 as *const i8, 2612,
                c"p && p->errCode==SQLITE_OK".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    '__s37:
        {
        match unsafe { (*p).e_state } {
            0 => {
                {
                    let mut b_use_wrapper: i32 = 1;

                    /// This is the very first call to sqlite3_recover_step() on this object.
                    recover_sql_callback(p,
                        c"BEGIN".as_ptr() as *mut i8 as *const i8);
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = on".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"PRAGMA foreign_keys = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_enter_mutex();

                    /// Open the output database. And register required virtual tables and 
                    ///* user functions with the new handle.
                    recover_open_output(p);
                    if unsafe { (*p).err_code } == 0 {
                        '__b38: loop {
                            '__c38: loop {
                                unsafe { (*p).err_code = 0 };
                                if b_use_wrapper != 0 { recover_install_wrapper(p); }

                                /// Open a transaction on the input database.
                                unsafe {
                                    sqlite3_file_control(unsafe { (*p).db_in },
                                        unsafe { (*p).z_db } as *const i8, 42,
                                        core::ptr::null_mut())
                                };
                                recover_exec(p, unsafe { (*p).db_in },
                                    c"PRAGMA writable_schema = on".as_ptr() as *mut i8 as
                                        *const i8);
                                recover_exec(p, unsafe { (*p).db_in },
                                    c"BEGIN".as_ptr() as *mut i8 as *const i8);
                                if unsafe { (*p).err_code } == 0 {
                                    unsafe { (*p).b_close_transaction = 1 };
                                }
                                recover_exec(p, unsafe { (*p).db_in },
                                    c"SELECT 1 FROM sqlite_schema".as_ptr() as *mut i8 as
                                        *const i8);
                                recover_transfer_settings(p);
                                recover_open_recovery(p);
                                recover_cache_schema(p);
                                if b_use_wrapper != 0 {
                                    recover_uninstall_wrapper(unsafe { &*p });
                                }
                                break '__c38;
                            }
                            if !(unsafe { (*p).err_code } == 26 &&
                                                {
                                                        let __p = &mut b_use_wrapper;
                                                        let __t = *__p;
                                                        *__p -= 1;
                                                        __t
                                                    } != 0 &&
                                            0 ==
                                                recover_one_stmt(unsafe { (*p).db_in },
                                                    c"ROLLBACK".as_ptr() as *mut i8 as *const i8)) {
                                break '__b38;
                            }
                        }
                    }
                    recover_leave_mutex();
                    recover_exec(p, unsafe { (*p).db_out },
                        c"BEGIN".as_ptr() as *mut i8 as *const i8);
                    recover_write_schema1(p);
                    unsafe { (*p).e_state = 1 };
                    break '__s37;
                }
                {
                    if unsafe { (*p).w1.p_tbls } == core::ptr::null_mut() {
                        recover_write_data_init(p);
                    }
                    if 101 == recover_write_data_step(p) {
                        recover_write_data_cleanup(p);
                        if !(unsafe { (*p).z_lost_and_found }).is_null() {
                            unsafe { (*p).e_state = 2 };
                        } else { unsafe { (*p).e_state = 5 }; }
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_used } == core::ptr::null_mut() {
                        recover_lost_and_found1_init(p);
                    }
                    if 101 == recover_lost_and_found1_step(p) {
                        unsafe { (*p).e_state = 3 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_all_and_parent } ==
                            core::ptr::null_mut() {
                        recover_lost_and_found2_init(p);
                    }
                    if 101 == recover_lost_and_found2_step(p) {
                        unsafe { (*p).e_state = 4 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_insert } == core::ptr::null_mut() {
                        recover_lost_and_found3_init(p);
                    }
                    if 101 == recover_lost_and_found3_step(p) {
                        unsafe { (*p).e_state = 5 };
                    }
                    break '__s37;
                }
                {
                    let mut rc: i32 = 0;
                    recover_write_schema2(p);
                    unsafe { (*p).e_state = 6 };

                    /// If no error has occurred, commit the write transaction on the output
                    ///* database. Regardless of whether or not an error has occurred, make
                    ///* an attempt to end the read transaction on the input database.
                    recover_exec(p, unsafe { (*p).db_out },
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    rc =
                        recover_one_stmt(unsafe { (*p).db_in },
                            c"END".as_ptr() as *mut i8 as *const i8);
                    if unsafe { (*p).err_code } == 0 {
                        unsafe { (*p).err_code = rc };
                    }
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    unsafe { (*p).e_state = 6 };
                    recover_final_cleanup(p);
                    break '__s37;
                }
                {

                    /// no-op
                    break '__s37;
                }
            }
            1 => {
                {
                    if unsafe { (*p).w1.p_tbls } == core::ptr::null_mut() {
                        recover_write_data_init(p);
                    }
                    if 101 == recover_write_data_step(p) {
                        recover_write_data_cleanup(p);
                        if !(unsafe { (*p).z_lost_and_found }).is_null() {
                            unsafe { (*p).e_state = 2 };
                        } else { unsafe { (*p).e_state = 5 }; }
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_used } == core::ptr::null_mut() {
                        recover_lost_and_found1_init(p);
                    }
                    if 101 == recover_lost_and_found1_step(p) {
                        unsafe { (*p).e_state = 3 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_all_and_parent } ==
                            core::ptr::null_mut() {
                        recover_lost_and_found2_init(p);
                    }
                    if 101 == recover_lost_and_found2_step(p) {
                        unsafe { (*p).e_state = 4 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_insert } == core::ptr::null_mut() {
                        recover_lost_and_found3_init(p);
                    }
                    if 101 == recover_lost_and_found3_step(p) {
                        unsafe { (*p).e_state = 5 };
                    }
                    break '__s37;
                }
                {
                    let mut rc: i32 = 0;
                    recover_write_schema2(p);
                    unsafe { (*p).e_state = 6 };

                    /// If no error has occurred, commit the write transaction on the output
                    ///* database. Regardless of whether or not an error has occurred, make
                    ///* an attempt to end the read transaction on the input database.
                    recover_exec(p, unsafe { (*p).db_out },
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    rc =
                        recover_one_stmt(unsafe { (*p).db_in },
                            c"END".as_ptr() as *mut i8 as *const i8);
                    if unsafe { (*p).err_code } == 0 {
                        unsafe { (*p).err_code = rc };
                    }
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    unsafe { (*p).e_state = 6 };
                    recover_final_cleanup(p);
                    break '__s37;
                }
                {

                    /// no-op
                    break '__s37;
                }
            }
            2 => {
                {
                    if unsafe { (*p).laf.p_used } == core::ptr::null_mut() {
                        recover_lost_and_found1_init(p);
                    }
                    if 101 == recover_lost_and_found1_step(p) {
                        unsafe { (*p).e_state = 3 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_all_and_parent } ==
                            core::ptr::null_mut() {
                        recover_lost_and_found2_init(p);
                    }
                    if 101 == recover_lost_and_found2_step(p) {
                        unsafe { (*p).e_state = 4 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_insert } == core::ptr::null_mut() {
                        recover_lost_and_found3_init(p);
                    }
                    if 101 == recover_lost_and_found3_step(p) {
                        unsafe { (*p).e_state = 5 };
                    }
                    break '__s37;
                }
                {
                    let mut rc: i32 = 0;
                    recover_write_schema2(p);
                    unsafe { (*p).e_state = 6 };

                    /// If no error has occurred, commit the write transaction on the output
                    ///* database. Regardless of whether or not an error has occurred, make
                    ///* an attempt to end the read transaction on the input database.
                    recover_exec(p, unsafe { (*p).db_out },
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    rc =
                        recover_one_stmt(unsafe { (*p).db_in },
                            c"END".as_ptr() as *mut i8 as *const i8);
                    if unsafe { (*p).err_code } == 0 {
                        unsafe { (*p).err_code = rc };
                    }
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    unsafe { (*p).e_state = 6 };
                    recover_final_cleanup(p);
                    break '__s37;
                }
                {

                    /// no-op
                    break '__s37;
                }
            }
            3 => {
                {
                    if unsafe { (*p).laf.p_all_and_parent } ==
                            core::ptr::null_mut() {
                        recover_lost_and_found2_init(p);
                    }
                    if 101 == recover_lost_and_found2_step(p) {
                        unsafe { (*p).e_state = 4 };
                    }
                    break '__s37;
                }
                {
                    if unsafe { (*p).laf.p_insert } == core::ptr::null_mut() {
                        recover_lost_and_found3_init(p);
                    }
                    if 101 == recover_lost_and_found3_step(p) {
                        unsafe { (*p).e_state = 5 };
                    }
                    break '__s37;
                }
                {
                    let mut rc: i32 = 0;
                    recover_write_schema2(p);
                    unsafe { (*p).e_state = 6 };

                    /// If no error has occurred, commit the write transaction on the output
                    ///* database. Regardless of whether or not an error has occurred, make
                    ///* an attempt to end the read transaction on the input database.
                    recover_exec(p, unsafe { (*p).db_out },
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    rc =
                        recover_one_stmt(unsafe { (*p).db_in },
                            c"END".as_ptr() as *mut i8 as *const i8);
                    if unsafe { (*p).err_code } == 0 {
                        unsafe { (*p).err_code = rc };
                    }
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    unsafe { (*p).e_state = 6 };
                    recover_final_cleanup(p);
                    break '__s37;
                }
                {

                    /// no-op
                    break '__s37;
                }
            }
            4 => {
                {
                    if unsafe { (*p).laf.p_insert } == core::ptr::null_mut() {
                        recover_lost_and_found3_init(p);
                    }
                    if 101 == recover_lost_and_found3_step(p) {
                        unsafe { (*p).e_state = 5 };
                    }
                    break '__s37;
                }
                {
                    let mut rc: i32 = 0;
                    recover_write_schema2(p);
                    unsafe { (*p).e_state = 6 };

                    /// If no error has occurred, commit the write transaction on the output
                    ///* database. Regardless of whether or not an error has occurred, make
                    ///* an attempt to end the read transaction on the input database.
                    recover_exec(p, unsafe { (*p).db_out },
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    rc =
                        recover_one_stmt(unsafe { (*p).db_in },
                            c"END".as_ptr() as *mut i8 as *const i8);
                    if unsafe { (*p).err_code } == 0 {
                        unsafe { (*p).err_code = rc };
                    }
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    unsafe { (*p).e_state = 6 };
                    recover_final_cleanup(p);
                    break '__s37;
                }
                {

                    /// no-op
                    break '__s37;
                }
            }
            5 => {
                {
                    let mut rc: i32 = 0;
                    recover_write_schema2(p);
                    unsafe { (*p).e_state = 6 };

                    /// If no error has occurred, commit the write transaction on the output
                    ///* database. Regardless of whether or not an error has occurred, make
                    ///* an attempt to end the read transaction on the input database.
                    recover_exec(p, unsafe { (*p).db_out },
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    rc =
                        recover_one_stmt(unsafe { (*p).db_in },
                            c"END".as_ptr() as *mut i8 as *const i8);
                    if unsafe { (*p).err_code } == 0 {
                        unsafe { (*p).err_code = rc };
                    }
                    recover_sql_callback(p,
                        c"PRAGMA writable_schema = off".as_ptr() as *mut i8 as
                            *const i8);
                    recover_sql_callback(p,
                        c"COMMIT".as_ptr() as *mut i8 as *const i8);
                    unsafe { (*p).e_state = 6 };
                    recover_final_cleanup(p);
                    break '__s37;
                }
                {

                    /// no-op
                    break '__s37;
                }
            }
            6 => {
                {

                    /// no-op
                    break '__s37;
                }
            }
            _ => {}
        }
    }
}

///* Perform a unit of work towards the recovery operation. This function 
///* must normally be called multiple times to complete database recovery.
///*
///* If no error occurs but the recovery operation is not completed, this
///* function returns SQLITE_OK. If recovery has been completed successfully
///* then SQLITE_DONE is returned. If an error has occurred, then an SQLite
///* error code (e.g. SQLITE_IOERR or SQLITE_NOMEM) is returned. It is not
///* considered an error if some or all of the data cannot be recovered
///* due to database corruption.
///*
///* Once sqlite3_recover_step() has returned a value other than SQLITE_OK,
///* all further such calls on the same recover handle are no-ops that return
///* the same non-SQLITE_OK value.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_step(p: *mut Sqlite3Recover) -> i32 {
    if p == core::ptr::null_mut() { return 7; }
    if unsafe { (*p).err_code } == 0 { recover_step(p); }
    if unsafe { (*p).e_state } == 6 && unsafe { (*p).err_code } == 0 {
        return 101;
    }
    return unsafe { (*p).err_code };
}

///* If this function is called on an sqlite3_recover handle after
///* an error occurs, an SQLite error code is returned. Otherwise, SQLITE_OK.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_errcode(p: *const Sqlite3Recover) -> i32 {
    return if !(p).is_null() { unsafe { (*p).err_code } } else { 7 };
}

/// 
///* Run the recovery operation to completion. Return SQLITE_OK if successful,
///* or an SQLite error code otherwise. Calling this function is the same
///* as executing:
///*
///*     while( SQLITE_OK==sqlite3_recover_step(p) );
///*     return sqlite3_recover_errcode(p);
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_run(p: *mut Sqlite3Recover) -> i32 {
    while 0 == sqlite3_recover_step(p) {}
    return sqlite3_recover_errcode(p as *const Sqlite3Recover);
}

///* If an error has been encountered during a prior call to
///* sqlite3_recover_step(), then this function attempts to return a 
///* pointer to a buffer containing an English language explanation of 
///* the error. If no error message is available, or if an out-of memory 
///* error occurs while attempting to allocate a buffer in which to format
///* the error message, NULL is returned.
///*
///* The returned buffer remains valid until the sqlite3_recover handle is
///* destroyed using sqlite3_recover_finish().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_errmsg(p: *const Sqlite3Recover)
    -> *const i8 {
    return if !(p).is_null() && unsafe { (*p).err_code } != 7 {
                unsafe { (*p).z_err_msg }
            } else { c"out of memory".as_ptr() as *mut i8 } as *const i8;
}

/// 
///* Clean up a recovery object created by a call to sqlite3_recover_init().
///* The results of using a recovery object with any API after it has been
///* passed to this function are undefined.
///*
///* This function returns the same value as sqlite3_recover_errcode().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_recover_finish(p: *mut Sqlite3Recover) -> i32 {
    let mut rc: i32 = 0;
    if p == core::ptr::null_mut() {
        rc = 7;
    } else {
        recover_final_cleanup(p);
        if unsafe { (*p).b_close_transaction } != 0 &&
                unsafe { sqlite3_get_autocommit(unsafe { (*p).db_in }) } == 0
            {
            rc =
                recover_one_stmt(unsafe { (*p).db_in },
                    c"END".as_ptr() as *mut i8 as *const i8);
            if unsafe { (*p).err_code } == 0 {
                unsafe { (*p).err_code = rc };
            }
        }
        rc = unsafe { (*p).err_code };
        unsafe { sqlite3_free(unsafe { (*p).z_err_msg } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_state_db } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_lost_and_found } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).p_page1_cache } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FuncN4Func {
    z_name: *const i8,
    n_arg: i32,
    x_func: Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
        *mut *mut Sqlite3Value) -> ()>,
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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn sqlite3_dbdata_init(_: *mut Sqlite3, _: *mut *mut i8,
    _: *const Sqlite3ApiRoutines)
    -> i32;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
