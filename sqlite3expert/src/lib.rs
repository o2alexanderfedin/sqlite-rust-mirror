#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexConstraint, Sqlite3IndexInfo, Sqlite3Int64,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor, SqliteInt64,
};

type DarwinSizeT = u64;

///* sqlite3expert object.
#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3expert {
    i_sample: i32,
    db: *mut Sqlite3,
    dbm: *mut Sqlite3,
    dbv: *mut Sqlite3,
    p_table: *mut IdxTable,
    p_scan: *mut IdxScan,
    p_write: *mut IdxWrite,
    p_statement: *mut IdxStatement,
    b_run: i32,
    pz_errmsg: *mut *mut i8,
    rc: i32,
    h_idx: IdxHash,
    z_candidates: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxTable {
    n_col: i32,
    z_name: *mut i8,
    a_col: *mut IdxColumn,
    p_next: *mut IdxTable,
}

///* Information regarding a single database table. Extracted from 
///* "PRAGMA table_info" by function idxGetTableInfo().
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxColumn {
    z_name: *mut i8,
    z_coll: *mut i8,
    i_pk: i32,
}

///* A single scan of a single table.
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxScan {
    p_tab: *mut IdxTable,
    i_db: i32,
    covering: i64,
    p_order: *mut IdxConstraint,
    p_eq: *mut IdxConstraint,
    p_range: *mut IdxConstraint,
    p_next_scan: *mut IdxScan,
}

///* A single constraint. Equivalent to either "col = ?" or "col < ?" (or
///* any other type of single-ended range constraint on a column).
///*
///* pLink:
///*   Used to temporarily link IdxConstraint objects into lists while
///*   creating candidate indexes.
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxConstraint {
    z_coll: *mut i8,
    b_range: i32,
    i_col: i32,
    b_flag: i32,
    b_desc: i32,
    p_next: *mut IdxConstraint,
    p_link: *mut IdxConstraint,
}

///* An object of the following type is created for each unique table/write-op
///* seen. The objects are stored in a singly-linked list beginning at
///* sqlite3expert.pWrite.
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxWrite {
    p_tab: *mut IdxTable,
    e_op: i32,
    p_next: *mut IdxWrite,
}

///* Each statement being analyzed is represented by an instance of this
///* structure.
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxStatement {
    i_id: i32,
    z_sql: *mut i8,
    z_idx: *mut i8,
    z_eqp: *mut i8,
    p_next: *mut IdxStatement,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxHash {
    p_first: *mut IdxHashEntry,
    a_hash: [*mut IdxHashEntry; 1023],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxHashEntry {
    z_key: *mut i8,
    z_val: *mut i8,
    z_val2: *mut i8,
    p_hash_next: *mut IdxHashEntry,
    p_next: *mut IdxHashEntry,
}

///* Allocate and return nByte bytes of zeroed memory using sqlite3_malloc(). 
///* If the allocation fails, set *pRc to SQLITE_NOMEM and return NULL.
extern "C" fn idx_malloc(p_rc_1: &mut i32, n_byte_1: i64) -> *mut () {
    let mut p_ret: *mut () = core::ptr::null_mut();
    if !(*p_rc_1 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxMalloc".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 177,
                c"*pRc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(n_byte_1 > 0 as i64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxMalloc".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 178,
                c"nByte>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_ret = unsafe { sqlite3_malloc64(n_byte_1 as Sqlite3Uint64) };
    if !(p_ret).is_null() {
        unsafe { memset(p_ret, 0, n_byte_1 as u64) };
    } else { *p_rc_1 = 7; }
    return p_ret;
}

///* Define and possibly pretend to use a useless collation sequence.
///* This pretense allows expert to accept SQL using custom collations.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn dummy_compare(up1: *mut (), up2: i32, up3: *const (),
    up4: i32, up5: *const ()) -> i32 {
    { let _ = up1; };
    { let _ = up2; };
    { let _ = up3; };
    { let _ = up4; };
    { let _ = up5; };
    if (0 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dummyCompare".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1913,
                c"0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };

    /// VDBE should never be run.
    return 0;
}

/// And a callback to register above upon actual need
#[unsafe(no_mangle)]
pub extern "C" fn use_dummy_cs(up1: *mut (), db: *mut Sqlite3, etr: i32,
    z_name_1: *const i8) -> () {
    { let _ = up1; };
    unsafe {
        sqlite3_create_collation_v2(db, z_name_1, etr, core::ptr::null_mut(),
            Some(dummy_compare), None)
    };
}

///* dummy functions for no-op implementation of UDFs during expert's work
#[unsafe(no_mangle)]
pub extern "C" fn dummy_udf(up1: *mut Sqlite3Context, up2: i32,
    up3: *mut *mut Sqlite3Value) -> () {
    { let _ = up1; };
    { let _ = up2; };
    { let _ = up3; };
    if (0 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dummyUDF".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1931,
                c"0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
}

#[unsafe(no_mangle)]
pub extern "C" fn dummy_ud_fvalue(up1: *mut Sqlite3Context) -> () {
    { let _ = up1; };
    if (0 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dummyUDFvalue".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1935,
                c"0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
}

///* Register UDFs from user database with another.
#[unsafe(no_mangle)]
pub extern "C" fn register_ud_fs(db_src_1: *mut Sqlite3,
    db_dst_1: *mut Sqlite3) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 =
        unsafe {
            sqlite3_prepare_v2(db_src_1,
                c"SELECT name,type,enc,narg,flags FROM pragma_function_list() WHERE builtin==0".as_ptr()
                        as *mut i8 as *const i8, -1, &mut p_stmt,
                core::ptr::null_mut())
        };
    if rc == 0 {
        while 100 == { rc = unsafe { sqlite3_step(p_stmt) }; rc } {
            let nargs: i32 = unsafe { sqlite3_column_int(p_stmt, 3) };
            let flags: i32 = unsafe { sqlite3_column_int(p_stmt, 4) };
            let name: *const i8 =
                unsafe { sqlite3_column_text(p_stmt, 0) } as *mut i8 as
                    *const i8;
            let type_: *const i8 =
                unsafe { sqlite3_column_text(p_stmt, 1) } as *mut i8 as
                    *const i8;
            let enc: *const i8 =
                unsafe { sqlite3_column_text(p_stmt, 2) } as *mut i8 as
                    *const i8;
            if name == core::ptr::null() || type_ == core::ptr::null() ||
                    enc == core::ptr::null()
                {} else {
                let mut ienc: i32 = 1;
                let mut rcf: i32 = 1;
                if unsafe {
                            strcmp(enc, c"utf16le".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    ienc = 2;
                } else if unsafe {
                            strcmp(enc, c"utf16be".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    ienc = 3;
                }
                ienc |= flags & (2048 | 524288);
                if unsafe {
                            strcmp(type_, c"w".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    rcf =
                        unsafe {
                            sqlite3_create_window_function(db_dst_1, name, nargs, ienc,
                                core::ptr::null_mut(), Some(dummy_udf),
                                Some(dummy_ud_fvalue), None, None, None)
                        };
                } else if unsafe {
                            strcmp(type_, c"a".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    rcf =
                        unsafe {
                            sqlite3_create_function(db_dst_1, name, nargs, ienc,
                                core::ptr::null_mut(), None, Some(dummy_udf),
                                Some(dummy_ud_fvalue))
                        };
                } else if unsafe {
                            strcmp(type_, c"s".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    rcf =
                        unsafe {
                            sqlite3_create_function(db_dst_1, name, nargs, ienc,
                                core::ptr::null_mut(), Some(dummy_udf), None, None)
                        };
                }
                if rcf != 0 { rc = rcf; break; }
            }
        }
        unsafe { sqlite3_finalize(p_stmt) };
        if rc == 101 { rc = 0; }
    }
    return rc;
}

///* An error associated with database handle db has just occurred. Pass
///* the error message to callback function xOut.
extern "C" fn idx_database_error(db: *mut Sqlite3, pz_errmsg_1: &mut *mut i8)
    -> () {
    *pz_errmsg_1 =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(db) })
        };
}

///* Prepare an SQL statement.
extern "C" fn idx_prepare_stmt(db: *mut Sqlite3,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_errmsg_1: *mut *mut i8,
    z_sql_1: *const i8) -> i32 {
    let rc: i32 =
        unsafe {
            sqlite3_prepare_v2(db, z_sql_1, -1, pp_stmt_1,
                core::ptr::null_mut())
        };
    if rc != 0 {
        unsafe { *pp_stmt_1 = core::ptr::null_mut() };
        idx_database_error(db, unsafe { &mut *pz_errmsg_1 });
    }
    return rc;
}

///* Prepare an SQL statement using the results of a printf() formatting.
unsafe extern "C" fn idx_printf_prepare_stmt(db: *mut Sqlite3,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_errmsg_1: *mut *mut i8,
    z_fmt_1: *const i8, mut __va0: ...) -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    if z_sql == core::ptr::null_mut() {
        rc = 7;
    } else {
        rc = idx_prepare_stmt(db, pp_stmt_1, pz_errmsg_1, z_sql as *const i8);
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    ();
    return rc;
}

///* This function tests if the schema of the main database of database handle
///* db contains an object named zTab. Assuming no error occurs, output parameter
///* (*pbContains) is set to true if zTab exists, or false if it does not.
///*
///* Or, if an error occurs, an SQLite error code is returned. The final value
///* of (*pbContains) is undefined in this case.
extern "C" fn expert_db_contains_object(db: *mut Sqlite3, z_tab_1: *const i8,
    pb_contains_1: &mut i32) -> i32 {
    let z_sql: *const i8 =
        c"SELECT 1 FROM sqlite_schema WHERE name = ?".as_ptr() as *mut i8 as
            *const i8;
    let mut p_sql: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut ret: i32 = 0;
    rc =
        unsafe {
            sqlite3_prepare_v2(db, z_sql, -1, &mut p_sql,
                core::ptr::null_mut())
        };
    if rc == 0 {
        unsafe { sqlite3_bind_text(p_sql, 1, z_tab_1, -1, None) };
        if 100 == unsafe { sqlite3_step(p_sql) } { ret = 1; }
        rc = unsafe { sqlite3_finalize(p_sql) };
    }
    *pb_contains_1 = ret;
    return rc;
}

///* Execute SQL command zSql using database handle db. If no error occurs,
///* set (*pzErr) to NULL and return SQLITE_OK. 
///*
///* If an error does occur, return an SQLite error code and set (*pzErr) to
///* point to a buffer containing an English language error message. Except,
///* if the error message begins with "no such module:", then ignore the
///* error and return as if the SQL statement had succeeded.
///*
///* This is used to copy as much of the database schema as possible while 
///* ignoring any errors related to missing virtual table modules.
extern "C" fn expert_schema_sql(db: *mut Sqlite3, z_sql_1: *const i8,
    pz_err_1: &mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut z_err: *mut i8 = core::ptr::null_mut();
    rc =
        unsafe {
            sqlite3_exec(db, z_sql_1, None, core::ptr::null_mut(), &mut z_err)
        };
    if rc != 0 && !(z_err).is_null() {
        let n_err: i32 = unsafe { strlen(z_err as *const i8) } as i32;
        if n_err >= 15 &&
                unsafe {
                        memcmp(z_err as *const (),
                            c"no such module:".as_ptr() as *mut i8 as *const (),
                            15 as u64)
                    } == 0 {
            unsafe { sqlite3_free(z_err as *mut ()) };
            rc = 0;
            z_err = core::ptr::null_mut();
        }
    }
    *pz_err_1 = z_err;
    return rc;
}

///* End of virtual table implementation.
///************************************************************************/
////*
///* Finalize SQL statement pStmt. If (*pRc) is SQLITE_OK when this function
///* is called, set it to the return value of sqlite3_finalize() before
///* returning. Otherwise, discard the sqlite3_finalize() return value.
extern "C" fn idx_finalize(p_rc_1: &mut i32, p_stmt_1: *mut Sqlite3Stmt)
    -> () {
    let rc: i32 = unsafe { sqlite3_finalize(p_stmt_1) };
    if *p_rc_1 == 0 { *p_rc_1 = rc; }
}

///**********************************************************************
///* Beginning of virtual table implementation.
#[repr(C)]
#[derive(Copy, Clone)]
struct ExpertVtab {
    base: Sqlite3Vtab,
    p_tab: *mut IdxTable,
    p_expert: *mut Sqlite3expert,
}

extern "C" fn expert_dequote(z_in_1: *const i8) -> *mut i8 {
    let n: i64 = unsafe { strlen(z_in_1) } as i32 as i64;
    let z_ret: *mut i8 =
        unsafe { sqlite3_malloc64(n as Sqlite3Uint64) } as *mut i8;
    if !(unsafe { *z_in_1.offset(0 as isize) } as i32 == '\'' as i32) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"expertDequote".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 384,
                c"zIn[0]==\'\\\'\'".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { *z_in_1.offset((n - 1 as i64) as isize) } as i32 ==
                            '\'' as i32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"expertDequote".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 385,
                c"zIn[n-1]==\'\\\'\'".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(z_ret).is_null() {
        let mut i_out: i64 = 0 as i64;
        let mut i_in: i64 = 0 as i64;
        {
            i_in = 1 as i64;
            '__b1: loop {
                if !(i_in < n - 1 as i64) { break '__b1; }
                '__c1: loop {
                    if unsafe { *z_in_1.offset(i_in as isize) } as i32 ==
                            '\'' as i32 {
                        if !(unsafe { *z_in_1.offset((i_in + 1 as i64) as isize) }
                                                    as i32 == '\'' as i32) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"expertDequote".as_ptr() as *const i8,
                                    c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 392,
                                    c"zIn[iIn+1]==\'\\\'\'".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                    }
                    unsafe {
                        *z_ret.offset({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = unsafe { *z_in_1.offset(i_in as isize) } as i8
                    };
                    break '__c1;
                }
                { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { *z_ret.offset(i_out as isize) = '\u{0}' as i32 as i8 };
    }
    return z_ret;
}

/// 
///* This function is the implementation of both the xConnect and xCreate
///* methods of the r-tree virtual table.
///*
///*   argv[0]   -> module name
///*   argv[1]   -> database name
///*   argv[2]   -> table name
///*   argv[...] -> column names...
extern "C" fn expert_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let p_expert: *mut Sqlite3expert = p_aux_1 as *mut Sqlite3expert;
    let mut p: *mut ExpertVtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if argc != 4 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"internal error!".as_ptr() as *mut i8 as
                            *const i8)
                }
        };
        rc = 1;
    } else {
        let z_create_table: *mut i8 =
            expert_dequote(unsafe { *argv.offset(3 as isize) });
        if !(z_create_table).is_null() {
            rc =
                unsafe {
                    sqlite3_declare_vtab(db, z_create_table as *const i8)
                };
            if rc == 0 {
                p =
                    idx_malloc(&mut rc,
                            core::mem::size_of::<ExpertVtab>() as i64) as
                        *mut ExpertVtab;
            }
            if rc == 0 {
                unsafe { (*p).p_expert = p_expert };
                unsafe { (*p).p_tab = unsafe { (*p_expert).p_table } };
                if !(unsafe {
                                            sqlite3_stricmp(unsafe { (*unsafe { (*p).p_tab }).z_name }
                                                    as *const i8, unsafe { *argv.offset(2 as isize) })
                                        } == 0) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"expertConnect".as_ptr() as *const i8,
                            c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 436,
                            c"sqlite3_stricmp(p->pTab->zName, argv[2])==0".as_ptr() as
                                    *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
            }
            unsafe { sqlite3_free(z_create_table as *mut ()) };
        } else { rc = 7; }
    }
    unsafe { *pp_vtab_1 = p as *mut Sqlite3Vtab };
    return rc;
}

///* Allocate and return a new IdxConstraint object. Set the IdxConstraint.zColl
///* variable to point to a copy of nul-terminated string zColl.
extern "C" fn idx_new_constraint(p_rc_1: *mut i32, z_coll_1: *const i8)
    -> *mut IdxConstraint {
    let mut p_new: *mut IdxConstraint = core::ptr::null_mut();
    let n_coll: i32 = unsafe { strlen(z_coll_1) } as i32;
    if !(unsafe { *p_rc_1 } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxNewConstraint".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 301,
                c"*pRc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_new =
        idx_malloc(unsafe { &mut *p_rc_1 },
                (core::mem::size_of::<IdxConstraint>() as u64 * n_coll as u64
                        + 1 as u64) as i64) as *mut IdxConstraint;
    if !(p_new).is_null() {
        unsafe {
            (*p_new).z_coll =
                unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
        };
        unsafe {
            memcpy(unsafe { (*p_new).z_coll } as *mut (),
                z_coll_1 as *const (), (n_coll + 1) as u64)
        };
    }
    return p_new;
}

#[allow(unused_doc_comments)]
extern "C" fn expert_best_index(p_vtab_1: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let p: *const ExpertVtab =
        p_vtab_1 as *mut ExpertVtab as *const ExpertVtab;
    let mut rc: i32 = 0;
    let mut n: i32 = 0;
    let mut p_scan: *mut IdxScan = core::ptr::null_mut();
    let opmask: i32 = (2 | 4 | 16 | 32 | 8) as i32;
    p_scan =
        idx_malloc(&mut rc, core::mem::size_of::<IdxScan>() as i64) as
            *mut IdxScan;
    if !(p_scan).is_null() {
        let mut i: i32 = 0;

        /// Link the new scan object into the list
        unsafe { (*p_scan).p_tab = unsafe { (*p).p_tab } };
        unsafe {
            (*p_scan).p_next_scan =
                unsafe { (*unsafe { (*p).p_expert }).p_scan }
        };
        unsafe { (*unsafe { (*p).p_expert }).p_scan = p_scan };
        {
            i = 0;
            '__b2: loop {
                if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                    break '__b2;
                }
                '__c2: loop {
                    let p_cons: *const Sqlite3IndexConstraint =
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_idx_info_1).a_constraint.offset(i as isize)
                                        }
                            } as *const Sqlite3IndexConstraint;
                    if unsafe { (*p_cons).usable } != 0 &&
                                    unsafe { (*p_cons).i_column } >= 0 &&
                                unsafe {
                                        (*unsafe {
                                                    (*unsafe {
                                                                        (*p).p_tab
                                                                    }).a_col.offset(unsafe { (*p_cons).i_column } as isize)
                                                }).i_pk
                                    } == 0 &&
                            unsafe { (*p_cons).op } as i32 & opmask as i32 != 0 {
                        let mut p_new: *mut IdxConstraint = core::ptr::null_mut();
                        let z_coll: *const i8 =
                            unsafe { sqlite3_vtab_collation(p_idx_info_1, i) };
                        p_new = idx_new_constraint(&mut rc, z_coll);
                        if !(p_new).is_null() {
                            unsafe { (*p_new).i_col = unsafe { (*p_cons).i_column } };
                            if unsafe { (*p_cons).op } as i32 == 2 {
                                unsafe { (*p_new).p_next = unsafe { (*p_scan).p_eq } };
                                unsafe { (*p_scan).p_eq = p_new };
                            } else {
                                unsafe { (*p_new).b_range = 1 };
                                unsafe { (*p_new).p_next = unsafe { (*p_scan).p_range } };
                                unsafe { (*p_scan).p_range = p_new };
                            }
                        }
                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        unsafe {
                            (*unsafe {
                                            (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                        }).argv_index = n
                        };
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = unsafe { (*p_idx_info_1).n_order_by } - 1;
            '__b3: loop {
                if !(i >= 0) { break '__b3; }
                '__c3: loop {
                    let i_col: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_idx_info_1).a_order_by.offset(i as isize)
                                    }).i_column
                        };
                    if i_col >= 0 {
                        let p_new_1: *mut IdxConstraint =
                            idx_new_constraint(&mut rc,
                                unsafe {
                                        (*unsafe {
                                                    (*unsafe { (*p).p_tab }).a_col.offset(i_col as isize)
                                                }).z_coll
                                    } as *const i8);
                        if !(p_new_1).is_null() {
                            unsafe { (*p_new_1).i_col = i_col };
                            unsafe {
                                (*p_new_1).b_desc =
                                    unsafe {
                                            (*unsafe {
                                                        (*p_idx_info_1).a_order_by.offset(i as isize)
                                                    }).desc
                                        } as i32
                            };
                            unsafe { (*p_new_1).p_next = unsafe { (*p_scan).p_order } };
                            unsafe { (*p_new_1).p_link = unsafe { (*p_scan).p_order } };
                            unsafe { (*p_scan).p_order = p_new_1 };
                            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
    }
    unsafe { (*p_idx_info_1).estimated_cost = 1000000.0 / (n + 1) as f64 };
    return rc;
}

extern "C" fn expert_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut ExpertVtab = p_vtab_1 as *mut ExpertVtab;
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ExpertCsr {
    base: Sqlite3VtabCursor,
    p_data: *mut Sqlite3Stmt,
}

/// 
///* Virtual table module xOpen method.
extern "C" fn expert_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut rc: i32 = 0;
    let mut p_csr: *mut ExpertCsr = core::ptr::null_mut();
    { let _ = p_v_tab_1; };
    p_csr =
        idx_malloc(&mut rc, core::mem::size_of::<ExpertCsr>() as i64) as
            *mut ExpertCsr;
    unsafe { *pp_cursor_1 = p_csr as *mut Sqlite3VtabCursor };
    return rc;
}

/// 
///* Virtual table module xClose method.
extern "C" fn expert_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *mut ExpertCsr = cur as *mut ExpertCsr;
    unsafe { sqlite3_finalize(unsafe { (*p_csr).p_data }) };
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}

/// 
///* Virtual table module xNext method.
extern "C" fn expert_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *mut ExpertCsr = cur as *mut ExpertCsr;
    let mut rc: i32 = 0;
    if (unsafe { (*p_csr).p_data }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"expertNext".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 574,
                c"pCsr->pData".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc = unsafe { sqlite3_step(unsafe { (*p_csr).p_data }) };
    if rc != 100 {
        rc = unsafe { sqlite3_finalize(unsafe { (*p_csr).p_data }) };
        unsafe { (*p_csr).p_data = core::ptr::null_mut() };
    } else { rc = 0; }
    return rc;
}

/// 
///* Virtual table module xFilter method.
extern "C" fn expert_filter(cur: *mut Sqlite3VtabCursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut Sqlite3Value) -> i32 {
    let p_csr: *mut ExpertCsr = cur as *mut ExpertCsr;
    let p_vtab: *mut ExpertVtab = unsafe { (*cur).p_vtab } as *mut ExpertVtab;
    let p_expert: *const Sqlite3expert =
        unsafe { (*p_vtab).p_expert } as *const Sqlite3expert;
    let mut rc: i32 = 0;
    { let _ = idx_num_1; };
    { let _ = idx_str_1; };
    { let _ = argc; };
    { let _ = argv; };
    rc = unsafe { sqlite3_finalize(unsafe { (*p_csr).p_data }) };
    unsafe { (*p_csr).p_data = core::ptr::null_mut() };
    if rc == 0 {
        rc =
            unsafe {
                idx_printf_prepare_stmt(unsafe { (*p_expert).db },
                    unsafe { &mut (*p_csr).p_data },
                    unsafe { &mut (*p_vtab).base.z_err_msg },
                    c"SELECT * FROM main.%Q WHERE sqlite_expert_sample()".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { (*unsafe { (*p_vtab).p_tab }).z_name })
            };
    }
    if rc == 0 { rc = expert_next(cur); }
    return rc;
}

///* Virtual table module xEof method.
///*
///* Return non-zero if the cursor does not currently point to a valid 
///* record (i.e if the scan has finished), or zero otherwise.
extern "C" fn expert_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *const ExpertCsr = cur as *mut ExpertCsr as *const ExpertCsr;
    return (unsafe { (*p_csr).p_data } == core::ptr::null_mut()) as i32;
}

/// 
///* Virtual table module xColumn method.
extern "C" fn expert_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_csr: *const ExpertCsr = cur as *mut ExpertCsr as *const ExpertCsr;
    let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
    p_val = unsafe { sqlite3_column_value(unsafe { (*p_csr).p_data }, i) };
    if !(p_val).is_null() { unsafe { sqlite3_result_value(ctx, p_val) }; }
    return 0;
}

/// 
///* Virtual table module xRowid method.
extern "C" fn expert_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    { let _ = cur; };
    unsafe { *p_rowid_1 = 0 as SqliteInt64 };
    return 0;
}

extern "C" fn expert_update(p_vtab_1: *mut Sqlite3Vtab, n_data_1: i32,
    az_data_1: *mut *mut Sqlite3Value, p_rowid_1: *mut SqliteInt64) -> i32 {
    { let _ = p_vtab_1; };
    { let _ = n_data_1; };
    { let _ = az_data_1; };
    { let _ = p_rowid_1; };
    return 0;
}

#[allow(unused_doc_comments)]
extern "C" fn idx_register_vtab(p: *mut Sqlite3expert) -> i32 {
    unsafe {

        /// iVersion
        /// xCreate - create a table
        /// xConnect - connect to an existing table
        /// xBestIndex - Determine search strategy
        /// xDisconnect - Disconnect from a table
        /// xDestroy - Drop a table
        /// xOpen - open a cursor
        /// xClose - close a cursor
        /// xFilter - configure scan constraints
        /// xNext - advance a cursor
        /// xEof
        /// xColumn - read data
        /// xRowid - read data
        /// xUpdate - write data
        /// xBegin - begin transaction
        /// xSync - sync transaction
        /// xCommit - commit transaction
        /// xRollback - rollback transaction
        /// xFindFunction - function overloading
        /// xRename - rename the table
        /// xSavepoint
        /// xRelease
        /// xRollbackTo
        /// xShadowName
        /// xIntegrity
        return unsafe {
                sqlite3_create_module(unsafe { (*p).dbv },
                    c"expert".as_ptr() as *mut i8 as *const i8,
                    &raw mut expert_module as *const Sqlite3Module,
                    p as *mut ())
            };
    }
}

///* Attempt to allocate an IdxTable structure corresponding to table zTab
///* in the main database of connection db. If successful, set (*ppOut) to
///* point to the new object and return SQLITE_OK. Otherwise, return an
///* SQLite error code and set (*ppOut) to NULL. In this case *pzErrmsg may be
///* set to point to an error string.
///*
///* It is the responsibility of the caller to eventually free either the
///* IdxTable object or error message using sqlite3_free().
extern "C" fn idx_get_table_info(db: *mut Sqlite3, z_tab_1: *const i8,
    pp_out_1: &mut *mut IdxTable, pz_errmsg_1: *mut *mut i8) -> i32 {
    let mut p1: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut n_col: i32 = 0;
    let mut n_tab: i32 = 0;
    let mut n_byte: i64 = 0 as i64;
    let mut p_new: *mut IdxTable = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut rc2: i32 = 0;
    let mut p_csr: *mut i8 = core::ptr::null_mut();
    let mut n_pk: i32 = 0;
    *pp_out_1 = core::ptr::null_mut();
    if z_tab_1 == core::ptr::null() { return 1; }
    n_tab = unsafe { strlen(z_tab_1) } as i32;
    n_byte =
        (core::mem::size_of::<IdxTable>() as u64 + n_tab as u64 + 1 as u64) as
            i64;
    rc =
        unsafe {
            idx_printf_prepare_stmt(db, &mut p1, pz_errmsg_1,
                c"PRAGMA table_xinfo=%Q".as_ptr() as *mut i8 as *const i8,
                z_tab_1)
        };
    while rc == 0 && 100 == unsafe { sqlite3_step(p1) } {
        let z_col: *const i8 =
            unsafe { sqlite3_column_text(p1, 1) } as *const i8;
        let mut z_col_seq: *const i8 = core::ptr::null();
        if z_col == core::ptr::null() { rc = 1; break; }
        n_byte += (1 + unsafe { strlen(z_col) } as i32) as i64;
        rc =
            unsafe {
                sqlite3_table_column_metadata(db,
                    c"main".as_ptr() as *mut i8 as *const i8, z_tab_1, z_col,
                    core::ptr::null_mut(), &mut z_col_seq,
                    core::ptr::null_mut(), core::ptr::null_mut(),
                    core::ptr::null_mut())
            };
        if z_col_seq == core::ptr::null() {
            z_col_seq = c"binary".as_ptr() as *mut i8 as *const i8;
        }
        n_byte += (1 + unsafe { strlen(z_col_seq) } as i32) as i64;
        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
        n_pk += (unsafe { sqlite3_column_int(p1, 5) } > 0) as i32;
    }
    rc2 = unsafe { sqlite3_reset(p1) };
    if rc == 0 { rc = rc2; }
    n_byte +=
        (core::mem::size_of::<IdxColumn>() as u64 * n_col as u64) as u64 as
            i64;
    if rc == 0 { p_new = idx_malloc(&mut rc, n_byte) as *mut IdxTable; }
    if rc == 0 {
        unsafe {
            (*p_new).a_col =
                unsafe { &raw mut *p_new.offset(1 as isize) } as
                    *mut IdxColumn
        };
        unsafe { (*p_new).n_col = n_col };
        p_csr =
            unsafe {
                    &raw mut *unsafe { (*p_new).a_col.offset(n_col as isize) }
                } as *mut i8;
    }
    n_col = 0;
    while rc == 0 && 100 == unsafe { sqlite3_step(p1) } {
        let z_col_1: *const i8 =
            unsafe { sqlite3_column_text(p1, 1) } as *const i8;
        let mut z_col_seq_1: *const i8 = core::ptr::null();
        let mut n_copy: i32 = 0;
        if z_col_1 == core::ptr::null() { continue; }
        n_copy = unsafe { strlen(z_col_1) } as i32 + 1;
        unsafe {
            (*unsafe { (*p_new).a_col.offset(n_col as isize) }).z_name = p_csr
        };
        unsafe {
            (*unsafe { (*p_new).a_col.offset(n_col as isize) }).i_pk =
                (unsafe { sqlite3_column_int(p1, 5) } == 1 && n_pk == 1) as
                    i32
        };
        unsafe {
            memcpy(p_csr as *mut (), z_col_1 as *const (), n_copy as u64)
        };
        {
            let __n = n_copy;
            let __p = &mut p_csr;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        rc =
            unsafe {
                sqlite3_table_column_metadata(db,
                    c"main".as_ptr() as *mut i8 as *const i8, z_tab_1, z_col_1,
                    core::ptr::null_mut(), &mut z_col_seq_1,
                    core::ptr::null_mut(), core::ptr::null_mut(),
                    core::ptr::null_mut())
            };
        if rc == 0 {
            if z_col_seq_1 == core::ptr::null() {
                z_col_seq_1 = c"binary".as_ptr() as *mut i8 as *const i8;
            }
            n_copy = unsafe { strlen(z_col_seq_1) } as i32 + 1;
            unsafe {
                (*unsafe { (*p_new).a_col.offset(n_col as isize) }).z_coll =
                    p_csr
            };
            unsafe {
                memcpy(p_csr as *mut (), z_col_seq_1 as *const (),
                    n_copy as u64)
            };
            {
                let __n = n_copy;
                let __p = &mut p_csr;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
    }
    idx_finalize(&mut rc, p1);
    if rc != 0 {
        unsafe { sqlite3_free(p_new as *mut ()) };
        p_new = core::ptr::null_mut();
    } else if if p_new != core::ptr::null_mut() {
                1
            } else {
                {
                    if (0 == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"idxGetTableInfo".as_ptr() as *const i8,
                                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 772,
                                c"0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    0
                }
            } != 0 {
        unsafe { (*p_new).z_name = p_csr };
        if if unsafe { (*p_new).z_name } != core::ptr::null_mut() {
                    1
                } else {
                    {
                        if (0 == 0) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"idxGetTableInfo".as_ptr() as *const i8,
                                    c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 774,
                                    c"0".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        0
                    }
                } != 0 {
            unsafe {
                memcpy(unsafe { (*p_new).z_name } as *mut (),
                    z_tab_1 as *const (), (n_tab + 1) as u64)
            };
        }
    }
    *pp_out_1 = p_new;
    return rc;
}

///* This function is a no-op if *pRc is set to anything other than 
///* SQLITE_OK when it is called.
///*
///* If *pRc is initially set to SQLITE_OK, then the text specified by
///* the printf() style arguments is appended to zIn and the result returned
///* in a buffer allocated by sqlite3_malloc(). sqlite3_free() is called on
///* zIn before returning.
unsafe extern "C" fn idx_append_text(p_rc_1: &mut i32, z_in_1: *mut i8,
    z_fmt_1: *const i8, mut __va0: ...) -> *mut i8 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_append: *mut i8 = core::ptr::null_mut();
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    let n_in: i64 =
        if !(z_in_1).is_null() {
                (unsafe { strlen(z_in_1 as *const i8) }) as i32
            } else { 0 } as i64;
    let mut n_append: i64 = 0 as i64;
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    if *p_rc_1 == 0 {
        z_append = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
        if !(z_append).is_null() {
            n_append = unsafe { strlen(z_append as *const i8) } as i32 as i64;
            z_ret =
                unsafe {
                        sqlite3_malloc64((n_in + n_append + 1 as i64) as
                                Sqlite3Uint64)
                    } as *mut i8;
        }
        if !(z_append).is_null() && !(z_ret).is_null() {
            if n_in != 0 {
                unsafe {
                    memcpy(z_ret as *mut (), z_in_1 as *const (), n_in as u64)
                };
            }
            unsafe {
                memcpy(unsafe { &raw mut *z_ret.offset(n_in as isize) } as
                        *mut (), z_append as *const (),
                    (n_append + 1 as i64) as u64)
            };
        } else {
            unsafe { sqlite3_free(z_ret as *mut ()) };
            z_ret = core::ptr::null_mut();
            *p_rc_1 = 7;
        }
        unsafe { sqlite3_free(z_append as *mut ()) };
        unsafe { sqlite3_free(z_in_1 as *mut ()) };
    }
    ();
    return z_ret;
}

#[allow(unused_doc_comments)]
extern "C" fn idx_create_vtab_schema(p: *mut Sqlite3expert,
    pz_errmsg_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = idx_register_vtab(p);
    let mut p_schema: *mut Sqlite3Stmt = core::ptr::null_mut();

    /// For each table in the main db schema:
    ///*
    ///*   1) Add an entry to the p->pTable list, and
    ///*   2) Create the equivalent virtual table in dbv.
    (rc =
        idx_prepare_stmt(unsafe { (*p).db }, &mut p_schema, pz_errmsg_1,
            c"SELECT type, name, sql, 1,        substr(sql,1,14)==\'create virtual\' COLLATE nocase FROM sqlite_schema WHERE type IN (\'table\',\'view\') AND       substr(name,1,7)!=\'sqlite_\' COLLATE nocase  UNION ALL SELECT type, name, sql, 2, 0 FROM sqlite_schema WHERE type = \'trigger\'  AND tbl_name IN(SELECT name FROM sqlite_schema WHERE type = \'view\') ORDER BY 4, 5 DESC, 1".as_ptr()
                    as *mut i8 as *const i8));
    while rc == 0 && 100 == unsafe { sqlite3_step(p_schema) } {
        let z_type: *const i8 =
            unsafe { sqlite3_column_text(p_schema, 0) } as *const i8;
        let z_name: *const i8 =
            unsafe { sqlite3_column_text(p_schema, 1) } as *const i8;
        let z_sql: *const i8 =
            unsafe { sqlite3_column_text(p_schema, 2) } as *const i8;
        let b_virtual: i32 = unsafe { sqlite3_column_int(p_schema, 4) };
        let mut b_exists: i32 = 0;
        if z_type == core::ptr::null() || z_name == core::ptr::null() {
            continue;
        }
        rc =
            expert_db_contains_object(unsafe { (*p).dbv }, z_name,
                &mut b_exists);
        if rc != 0 || b_exists != 0 { continue; }
        if unsafe { *z_type.offset(0 as isize) } as i32 == 'v' as i32 ||
                    unsafe { *z_type.offset(1 as isize) } as i32 == 'r' as i32
                || b_virtual != 0 {
            if !(z_sql).is_null() {
                rc =
                    expert_schema_sql(unsafe { (*p).dbv }, z_sql,
                        unsafe { &mut *pz_errmsg_1 });
            }
        } else {
            let mut p_tab: *mut IdxTable = core::ptr::null_mut();
            rc =
                idx_get_table_info(unsafe { (*p).db }, z_name, &mut p_tab,
                    pz_errmsg_1);
            if rc == 0 &&
                    if p_tab != core::ptr::null_mut() {
                            1
                        } else {
                            {
                                if (0 == 0) as i32 as i64 != 0 {
                                    unsafe {
                                        __assert_rtn(c"idxCreateVtabSchema".as_ptr() as *const i8,
                                            c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1494,
                                            c"0".as_ptr() as *mut i8 as *const i8)
                                    }
                                } else { { let _ = 0; } };
                                0
                            }
                        } != 0 {
                let mut i: i32 = 0;
                let mut z_inner: *mut i8 = core::ptr::null_mut();
                let mut z_outer: *mut i8 = core::ptr::null_mut();
                unsafe { (*p_tab).p_next = unsafe { (*p).p_table } };
                unsafe { (*p).p_table = p_tab };

                /// The statement the vtab will pass to sqlite3_declare_vtab()
                (z_inner =
                    unsafe {
                        idx_append_text(&mut rc, core::ptr::null_mut(),
                            c"CREATE TABLE x(".as_ptr() as *mut i8 as *const i8)
                    });
                {
                    i = 0;
                    '__b7: loop {
                        if !(i < unsafe { (*p_tab).n_col }) { break '__b7; }
                        '__c7: loop {
                            z_inner =
                                unsafe {
                                    idx_append_text(&mut rc, z_inner,
                                        c"%s%Q COLLATE %Q".as_ptr() as *mut i8 as *const i8,
                                        if i == 0 {
                                            c"".as_ptr() as *mut i8
                                        } else { c", ".as_ptr() as *mut i8 },
                                        unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).z_name
                                        },
                                        unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).z_coll
                                        })
                                };
                            break '__c7;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                z_inner =
                    unsafe {
                        idx_append_text(&mut rc, z_inner,
                            c")".as_ptr() as *mut i8 as *const i8)
                    };

                /// The CVT statement to create the vtab
                (z_outer =
                    unsafe {
                        idx_append_text(&mut rc, core::ptr::null_mut(),
                            c"CREATE VIRTUAL TABLE %Q USING expert(%Q)".as_ptr() as
                                    *mut i8 as *const i8, z_name, z_inner)
                    });
                if rc == 0 {
                    rc =
                        unsafe {
                            sqlite3_exec(unsafe { (*p).dbv }, z_outer as *const i8,
                                None, core::ptr::null_mut(), pz_errmsg_1)
                        };
                }
                unsafe { sqlite3_free(z_inner as *mut ()) };
                unsafe { sqlite3_free(z_outer as *mut ()) };
            }
        }
    }
    idx_finalize(&mut rc, p_schema);
    return rc;
}

extern "C" fn idx_auth_callback(p_ctx_1: *mut (), e_op_1: i32, z3: *const i8,
    z4: *const i8, z_db_1: *const i8, z_trigger_1: *const i8) -> i32 {
    let mut rc: i32 = 0;
    { let _ = z4; };
    { let _ = z_trigger_1; };
    if e_op_1 == 18 || e_op_1 == 23 || e_op_1 == 9 {
        if unsafe {
                    sqlite3_stricmp(z_db_1,
                        c"main".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            let p: *mut Sqlite3expert = p_ctx_1 as *mut Sqlite3expert;
            let mut p_tab: *mut IdxTable = core::ptr::null_mut();
            {
                p_tab = unsafe { (*p).p_table };
                '__b8: loop {
                    if !(!(p_tab).is_null()) { break '__b8; }
                    '__c8: loop {
                        if 0 ==
                                unsafe {
                                    sqlite3_stricmp(z3, unsafe { (*p_tab).z_name } as *const i8)
                                } {
                            break '__b8;
                        }
                        break '__c8;
                    }
                    p_tab = unsafe { (*p_tab).p_next };
                }
            }
            if !(p_tab).is_null() {
                let mut p_write: *mut IdxWrite = core::ptr::null_mut();
                {
                    p_write = unsafe { (*p).p_write };
                    '__b9: loop {
                        if !(!(p_write).is_null()) { break '__b9; }
                        '__c9: loop {
                            if unsafe { (*p_write).p_tab } == p_tab &&
                                    unsafe { (*p_write).e_op } == e_op_1 {
                                break '__b9;
                            }
                            break '__c9;
                        }
                        p_write = unsafe { (*p_write).p_next };
                    }
                }
                if p_write == core::ptr::null_mut() {
                    p_write =
                        idx_malloc(&mut rc, core::mem::size_of::<IdxWrite>() as i64)
                            as *mut IdxWrite;
                    if rc == 0 {
                        unsafe { (*p_write).p_tab = p_tab };
                        unsafe { (*p_write).e_op = e_op_1 };
                        unsafe { (*p_write).p_next = unsafe { (*p).p_write } };
                        unsafe { (*p).p_write = p_write };
                    }
                }
            }
        }
    }
    return rc;
}

///* Free all elements of the linked list starting at pConstraint.
extern "C" fn idx_constraint_free(p_constraint_1: *mut IdxConstraint) -> () {
    let mut p_next: *mut IdxConstraint = core::ptr::null_mut();
    let mut p: *mut IdxConstraint = core::ptr::null_mut();
    {
        p = p_constraint_1;
        '__b10: loop {
            if !(!(p).is_null()) { break '__b10; }
            '__c10: loop {
                p_next = unsafe { (*p).p_next };
                unsafe { sqlite3_free(p as *mut ()) };
                break '__c10;
            }
            p = p_next;
        }
    }
}

///* Free all elements of the linked list starting from pScan up until pLast
///* (pLast is not freed).
extern "C" fn idx_scan_free(p_scan_1: *mut IdxScan, p_last_1: *mut IdxScan)
    -> () {
    let mut p: *mut IdxScan = core::ptr::null_mut();
    let mut p_next: *mut IdxScan = core::ptr::null_mut();
    {
        p = p_scan_1;
        '__b11: loop {
            if !(p != p_last_1) { break '__b11; }
            '__c11: loop {
                p_next = unsafe { (*p).p_next_scan };
                idx_constraint_free(unsafe { (*p).p_order });
                idx_constraint_free(unsafe { (*p).p_eq });
                idx_constraint_free(unsafe { (*p).p_range });
                unsafe { sqlite3_free(p as *mut ()) };
                break '__c11;
            }
            p = p_next;
        }
    }
}

///* Free all elements of the linked list starting from pStatement up 
///* until pLast (pLast is not freed).
extern "C" fn idx_statement_free(p_statement_1: *mut IdxStatement,
    p_last_1: *mut IdxStatement) -> () {
    let mut p: *mut IdxStatement = core::ptr::null_mut();
    let mut p_next: *mut IdxStatement = core::ptr::null_mut();
    {
        p = p_statement_1;
        '__b12: loop {
            if !(p != p_last_1) { break '__b12; }
            '__c12: loop {
                p_next = unsafe { (*p).p_next };
                unsafe { sqlite3_free(unsafe { (*p).z_eqp } as *mut ()) };
                unsafe { sqlite3_free(unsafe { (*p).z_idx } as *mut ()) };
                unsafe { sqlite3_free(p as *mut ()) };
                break '__c12;
            }
            p = p_next;
        }
    }
}

///* Free the linked list of IdxTable objects starting at pTab.
extern "C" fn idx_table_free(p_tab_1: *mut IdxTable) -> () {
    let mut p_iter: *mut IdxTable = core::ptr::null_mut();
    let mut p_next: *mut IdxTable = core::ptr::null_mut();
    {
        p_iter = p_tab_1;
        '__b13: loop {
            if !(!(p_iter).is_null()) { break '__b13; }
            '__c13: loop {
                p_next = unsafe { (*p_iter).p_next };
                unsafe { sqlite3_free(p_iter as *mut ()) };
                break '__c13;
            }
            p_iter = p_next;
        }
    }
}

///* Free the linked list of IdxWrite objects starting at pTab.
extern "C" fn idx_write_free(p_tab_1: *mut IdxWrite) -> () {
    let mut p_iter: *mut IdxWrite = core::ptr::null_mut();
    let mut p_next: *mut IdxWrite = core::ptr::null_mut();
    {
        p_iter = p_tab_1;
        '__b14: loop {
            if !(!(p_iter).is_null()) { break '__b14; }
            '__c14: loop {
                p_next = unsafe { (*p_iter).p_next };
                unsafe { sqlite3_free(p_iter as *mut ()) };
                break '__c14;
            }
            p_iter = p_next;
        }
    }
}

///* Reset an IdxHash hash table.
extern "C" fn idx_hash_clear(p_hash_1: *mut IdxHash) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b15: loop {
            if !(i < 1023) { break '__b15; }
            '__c15: loop {
                let mut p_entry: *mut IdxHashEntry = core::ptr::null_mut();
                let mut p_next: *mut IdxHashEntry = core::ptr::null_mut();
                {
                    p_entry = unsafe { (*p_hash_1).a_hash[i as usize] };
                    '__b16: loop {
                        if !(!(p_entry).is_null()) { break '__b16; }
                        '__c16: loop {
                            p_next = unsafe { (*p_entry).p_hash_next };
                            unsafe {
                                sqlite3_free(unsafe { (*p_entry).z_val2 } as *mut ())
                            };
                            unsafe { sqlite3_free(p_entry as *mut ()) };
                            break '__c16;
                        }
                        p_entry = p_next;
                    }
                }
                break '__c15;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        memset(p_hash_1 as *mut (), 0, core::mem::size_of::<IdxHash>() as u64)
    };
}

///* Free an (sqlite3expert*) handle and all associated resources. There 
///* should be one call to this function for each successful call to 
///* sqlite3-expert_new().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expert_destroy(p: *mut Sqlite3expert) -> () {
    if !(p).is_null() {
        unsafe { sqlite3_close(unsafe { (*p).dbm }) };
        unsafe { sqlite3_close(unsafe { (*p).dbv }) };
        idx_scan_free(unsafe { (*p).p_scan }, core::ptr::null_mut());
        idx_statement_free(unsafe { (*p).p_statement },
            core::ptr::null_mut());
        idx_table_free(unsafe { (*p).p_table });
        idx_write_free(unsafe { (*p).p_write });
        idx_hash_clear(unsafe { &mut (*p).h_idx });
        unsafe { sqlite3_free(unsafe { (*p).z_candidates } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

///* Create a new sqlite3expert object.
///*
///* If successful, a pointer to the new object is returned and (*pzErr) set
///* to NULL. Or, if an error occurs, NULL is returned and (*pzErr) set to
///* an English-language error message. In this case it is the responsibility
///* of the caller to eventually free the error message buffer using
///* sqlite3_free().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expert_new(db: *mut Sqlite3,
    pz_errmsg_1: *mut *mut i8) -> *mut Sqlite3expert {
    let mut rc: i32 = 0;
    let mut p_new: *mut Sqlite3expert = core::ptr::null_mut();
    p_new =
        idx_malloc(&mut rc, core::mem::size_of::<Sqlite3expert>() as i64) as
            *mut Sqlite3expert;
    if rc == 0 {
        unsafe { (*p_new).db = db };
        unsafe { (*p_new).i_sample = 100 };
        rc =
            unsafe {
                sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                    unsafe { &mut (*p_new).dbv })
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                    unsafe { &mut (*p_new).dbm })
            };
        if rc == 0 {
            unsafe {
                sqlite3_db_config(unsafe { (*p_new).dbm }, 1008, 1,
                    0 as *mut i32)
            };
        }
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_collation_needed(unsafe { (*p_new).dbm },
                    core::ptr::null_mut(), Some(use_dummy_cs))
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_collation_needed(unsafe { (*p_new).dbv },
                    core::ptr::null_mut(), Some(use_dummy_cs))
            };
    }
    if rc == 0 {
        rc = register_ud_fs(unsafe { (*p_new).db }, unsafe { (*p_new).dbm });
    }
    if rc == 0 {
        rc = register_ud_fs(unsafe { (*p_new).db }, unsafe { (*p_new).dbv });
    }
    if rc == 0 {
        let mut p_sql: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            unsafe {
                idx_printf_prepare_stmt(unsafe { (*p_new).db }, &mut p_sql,
                    pz_errmsg_1,
                    c"SELECT sql, name, substr(sql,1,14)==\'create virtual\' COLLATE nocase FROM sqlite_schema WHERE substr(name,1,7)!=\'sqlite_\' COLLATE nocase ORDER BY 3 DESC, rowid".as_ptr()
                            as *mut i8 as *const i8)
            };
        while rc == 0 && 100 == unsafe { sqlite3_step(p_sql) } {
            let z_sql: *const i8 =
                unsafe { sqlite3_column_text(p_sql, 0) } as *const i8;
            let z_name: *const i8 =
                unsafe { sqlite3_column_text(p_sql, 1) } as *const i8;
            let mut b_exists: i32 = 0;
            rc =
                expert_db_contains_object(unsafe { (*p_new).dbm }, z_name,
                    &mut b_exists);
            if rc == 0 && !(z_sql).is_null() && b_exists == 0 {
                rc =
                    expert_schema_sql(unsafe { (*p_new).dbm }, z_sql,
                        unsafe { &mut *pz_errmsg_1 });
            }
        }
        idx_finalize(&mut rc, p_sql);
    }
    if rc == 0 { rc = idx_create_vtab_schema(p_new, pz_errmsg_1); }
    if rc == 0 {
        unsafe {
            sqlite3_set_authorizer(unsafe { (*p_new).dbv },
                Some(idx_auth_callback), p_new as *mut ())
        };
    }
    if rc != 0 {
        unsafe { sqlite3_expert_destroy(p_new) };
        p_new = core::ptr::null_mut();
    }
    return p_new;
}

///* Configure an sqlite3expert object.
///*
///* EXPERT_CONFIG_SAMPLE:
///*   By default, sqlite3_expert_analyze() generates sqlite_stat1 data for
///*   each candidate index. This involves scanning and sorting the entire
///*   contents of each user database table once for each candidate index
///*   associated with the table. For large databases, this can be 
///*   prohibitively slow. This option allows the sqlite3expert object to
///*   be configured so that sqlite_stat1 data is instead generated based on a
///*   subset of each table, or so that no sqlite_stat1 data is used at all.
///*
///*   A single integer argument is passed to this option. If the value is less
///*   than or equal to zero, then no sqlite_stat1 data is generated or used by
///*   the analysis - indexes are recommended based on the database schema only.
///*   Or, if the value is 100 or greater, complete sqlite_stat1 data is
///*   generated for each candidate index (this is the default). Finally, if the
///*   value falls between 0 and 100, then it represents the percentage of user
///*   table rows that should be considered when generating sqlite_stat1 data.
///*
///*   Examples:
///*
///*     // Do not generate any sqlite_stat1 data
///*     sqlite3_expert_config(pExpert, EXPERT_CONFIG_SAMPLE, 0);
///*
///*     // Generate sqlite_stat1 data based on 10% of the rows in each table.
///*     sqlite3_expert_config(pExpert, EXPERT_CONFIG_SAMPLE, 10);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_config(p: &mut Sqlite3expert, op: i32,
    mut __va0: ...) -> i32 {
    let mut rc: i32 = 0;
    let mut ap: *const i8 = core::ptr::null();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    '__s18:
        {
        match op {
            1 => {
                {
                    let mut i_val: i32 =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        };
                    if i_val < 0 { i_val = 0; }
                    if i_val > 100 { i_val = 100; }
                    (*p).i_sample = i_val;
                    break '__s18;
                }
                rc = 12;
            }
            _ => { rc = 12; }
        }
    }
    ();
    return rc;
}

///* Specify zero or more SQL statements to be included in the analysis.
///*
///* Buffer zSql must contain zero or more complete SQL statements. This
///* function parses all statements contained in the buffer and adds them
///* to the internal list of statements to analyze. If successful, SQLITE_OK
///* is returned and (*pzErr) set to NULL. Or, if an error occurs - for example
///* due to a error in the SQL - an SQLite error code is returned and (*pzErr)
///* may be set to point to an English language error message. In this case
///* the caller is responsible for eventually freeing the error message buffer
///* using sqlite3_free().
///*
///* If an error does occur while processing one of the statements in the
///* buffer passed as the second argument, none of the statements in the
///* buffer are added to the analysis.
///*
///* This function must be called before sqlite3_expert_analyze(). If a call
///* to this function is made on an sqlite3expert object that has already
///* been passed to sqlite3_expert_analyze() SQLITE_MISUSE is returned
///* immediately and no statements are added to the analysis.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expert_sql(p: &mut Sqlite3expert,
    z_sql_1: *const i8, pz_err_1: *mut *mut i8) -> i32 {
    let p_scan_orig: *mut IdxScan = (*p).p_scan;
    let p_stmt_orig: *mut IdxStatement = (*p).p_statement;
    let mut rc: i32 = 0;
    let mut z_stmt: *const i8 = z_sql_1;
    if (*p).b_run != 0 { return 21; }
    while rc == 0 && !(z_stmt).is_null() &&
            unsafe { *z_stmt.offset(0 as isize) } != 0 {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();

        /// Ensure that the provided statement compiles against user's DB.
        (rc = idx_prepare_stmt((*p).db, &mut p_stmt, pz_err_1, z_stmt));
        if rc != 0 { break; }
        unsafe { sqlite3_finalize(p_stmt) };
        rc =
            unsafe {
                sqlite3_prepare_v2((*p).dbv, z_stmt, -1, &mut p_stmt,
                    &mut z_stmt)
            };
        if rc == 0 {
            if !(p_stmt).is_null() {
                let mut p_new: *mut IdxStatement = core::ptr::null_mut();
                let z: *const i8 = unsafe { sqlite3_sql(p_stmt) };
                let n: i64 = unsafe { strlen(z) } as i32 as i64;
                p_new =
                    idx_malloc(&mut rc,
                            (core::mem::size_of::<IdxStatement>() as u64 + n as u64 +
                                    1 as u64) as i64) as *mut IdxStatement;
                if rc == 0 {
                    unsafe {
                        (*p_new).z_sql =
                            unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
                    };
                    unsafe {
                        memcpy(unsafe { (*p_new).z_sql } as *mut (), z as *const (),
                            (n + 1 as i64) as u64)
                    };
                    unsafe { (*p_new).p_next = (*p).p_statement };
                    if !((*p).p_statement).is_null() {
                        unsafe {
                            (*p_new).i_id = unsafe { (*(*p).p_statement).i_id } + 1
                        };
                    }
                    (*p).p_statement = p_new;
                }
                unsafe { sqlite3_finalize(p_stmt) };
            }
        } else { idx_database_error((*p).dbv, unsafe { &mut *pz_err_1 }); }
    }
    if rc != 0 {
        idx_scan_free((*p).p_scan, p_scan_orig);
        idx_statement_free((*p).p_statement, p_stmt_orig);
        (*p).p_scan = p_scan_orig;
        (*p).p_statement = p_stmt_orig;
    }
    return rc;
}

#[allow(unused_doc_comments)]
extern "C" fn idx_process_one_trigger(p: &Sqlite3expert, p_write_1: &IdxWrite,
    pz_err_1: *mut *mut i8) -> i32 {
    unsafe {
        let p_tab: *const IdxTable = (*p_write_1).p_tab as *const IdxTable;
        let z_tab: *const i8 = unsafe { (*p_tab).z_name } as *const i8;
        let z_sql: *const i8 =
            c"SELECT \'CREATE TEMP\' || substr(sql, 7) FROM sqlite_schema WHERE tbl_name = %Q AND type IN (\'table\', \'trigger\') ORDER BY type;".as_ptr()
                    as *mut i8 as *const i8;
        let mut p_select: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut z_write: *mut i8 = core::ptr::null_mut();

        /// Create the table and its triggers in the temp schema
        (rc =
            unsafe {
                idx_printf_prepare_stmt((*p).db, &mut p_select, pz_err_1,
                    z_sql, z_tab, z_tab)
            });
        while rc == 0 && 100 == unsafe { sqlite3_step(p_select) } {
            let z_create: *const i8 =
                unsafe { sqlite3_column_text(p_select, 0) } as *const i8;
            if z_create == core::ptr::null() { continue; }
            rc =
                unsafe {
                    sqlite3_exec((*p).dbv, z_create, None,
                        core::ptr::null_mut(), pz_err_1)
                };
        }
        idx_finalize(&mut rc, p_select);
        if rc == 0 {
            let z: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"ALTER TABLE temp.%Q RENAME TO %Q".as_ptr()
                                as *mut i8 as *const i8, z_tab, z_int)
                };
            if z == core::ptr::null_mut() {
                rc = 7;
            } else {
                rc =
                    unsafe {
                        sqlite3_exec((*p).dbv, z as *const i8, None,
                            core::ptr::null_mut(), pz_err_1)
                    };
                unsafe { sqlite3_free(z as *mut ()) };
            }
        }
        '__s21:
            {
            match (*p_write_1).e_op {
                18 => {
                    {
                        let mut i: i32 = 0;
                        z_write =
                            unsafe {
                                idx_append_text(&mut rc, z_write,
                                    c"INSERT INTO %Q VALUES(".as_ptr() as *mut i8 as *const i8,
                                    z_int)
                            };
                        {
                            i = 0;
                            '__b22: loop {
                                if !(i < unsafe { (*p_tab).n_col }) { break '__b22; }
                                '__c22: loop {
                                    z_write =
                                        unsafe {
                                            idx_append_text(&mut rc, z_write,
                                                c"%s?".as_ptr() as *mut i8 as *const i8,
                                                if i == 0 {
                                                    c"".as_ptr() as *mut i8
                                                } else { c", ".as_ptr() as *mut i8 })
                                        };
                                    break '__c22;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        z_write =
                            unsafe {
                                idx_append_text(&mut rc, z_write,
                                    c")".as_ptr() as *mut i8 as *const i8)
                            };
                        break '__s21;
                    }
                    {
                        let mut i: i32 = 0;
                        z_write =
                            unsafe {
                                idx_append_text(&mut rc, z_write,
                                    c"UPDATE %Q SET ".as_ptr() as *mut i8 as *const i8, z_int)
                            };
                        {
                            i = 0;
                            '__b23: loop {
                                if !(i < unsafe { (*p_tab).n_col }) { break '__b23; }
                                '__c23: loop {
                                    z_write =
                                        unsafe {
                                            idx_append_text(&mut rc, z_write,
                                                c"%s%Q=?".as_ptr() as *mut i8 as *const i8,
                                                if i == 0 {
                                                    c"".as_ptr() as *mut i8
                                                } else { c", ".as_ptr() as *mut i8 },
                                                unsafe {
                                                    (*unsafe { (*p_tab).a_col.offset(i as isize) }).z_name
                                                })
                                        };
                                    break '__c23;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        break '__s21;
                    }
                    {
                        if !((*p_write_1).e_op == 9) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"idxProcessOneTrigger".as_ptr() as *const i8,
                                    c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1353,
                                    c"pWrite->eOp==SQLITE_DELETE".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        if rc == 0 {
                            z_write =
                                unsafe {
                                    sqlite3_mprintf(c"DELETE FROM %Q".as_ptr() as *mut i8 as
                                            *const i8, z_int)
                                };
                            if z_write == core::ptr::null_mut() { rc = 7; }
                        }
                    }
                }
                23 => {
                    {
                        let mut i: i32 = 0;
                        z_write =
                            unsafe {
                                idx_append_text(&mut rc, z_write,
                                    c"UPDATE %Q SET ".as_ptr() as *mut i8 as *const i8, z_int)
                            };
                        {
                            i = 0;
                            '__b23: loop {
                                if !(i < unsafe { (*p_tab).n_col }) { break '__b23; }
                                '__c23: loop {
                                    z_write =
                                        unsafe {
                                            idx_append_text(&mut rc, z_write,
                                                c"%s%Q=?".as_ptr() as *mut i8 as *const i8,
                                                if i == 0 {
                                                    c"".as_ptr() as *mut i8
                                                } else { c", ".as_ptr() as *mut i8 },
                                                unsafe {
                                                    (*unsafe { (*p_tab).a_col.offset(i as isize) }).z_name
                                                })
                                        };
                                    break '__c23;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        break '__s21;
                    }
                    {
                        if !((*p_write_1).e_op == 9) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"idxProcessOneTrigger".as_ptr() as *const i8,
                                    c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1353,
                                    c"pWrite->eOp==SQLITE_DELETE".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        if rc == 0 {
                            z_write =
                                unsafe {
                                    sqlite3_mprintf(c"DELETE FROM %Q".as_ptr() as *mut i8 as
                                            *const i8, z_int)
                                };
                            if z_write == core::ptr::null_mut() { rc = 7; }
                        }
                    }
                }
                _ => {
                    {
                        if !((*p_write_1).e_op == 9) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"idxProcessOneTrigger".as_ptr() as *const i8,
                                    c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1353,
                                    c"pWrite->eOp==SQLITE_DELETE".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        if rc == 0 {
                            z_write =
                                unsafe {
                                    sqlite3_mprintf(c"DELETE FROM %Q".as_ptr() as *mut i8 as
                                            *const i8, z_int)
                                };
                            if z_write == core::ptr::null_mut() { rc = 7; }
                        }
                    }
                }
            }
        }
        if rc == 0 {
            let mut p_x: *mut Sqlite3Stmt = core::ptr::null_mut();
            rc =
                unsafe {
                    sqlite3_prepare_v2((*p).dbv, z_write as *const i8, -1,
                        &mut p_x, core::ptr::null_mut())
                };
            idx_finalize(&mut rc, p_x);
            if rc != 0 {
                idx_database_error((*p).dbv, unsafe { &mut *pz_err_1 });
            }
        }
        unsafe { sqlite3_free(z_write as *mut ()) };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_exec((*p).dbv, z_drop, None, core::ptr::null_mut(),
                        pz_err_1)
                };
        }
        return rc;
    }
}

extern "C" fn idx_process_triggers(p: *mut Sqlite3expert,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut p_end: *mut IdxWrite = core::ptr::null_mut();
    let mut p_first: *mut IdxWrite = unsafe { (*p).p_write };
    while rc == 0 && p_first != p_end {
        let mut p_iter: *mut IdxWrite = core::ptr::null_mut();
        {
            p_iter = p_first;
            '__b25: loop {
                if !(rc == 0 && p_iter != p_end) { break '__b25; }
                '__c25: loop {
                    rc =
                        idx_process_one_trigger(unsafe { &*p }, unsafe { &*p_iter },
                            pz_err_1);
                    break '__c25;
                }
                p_iter = unsafe { (*p_iter).p_next };
            }
        }
        p_end = p_first;
        p_first = unsafe { (*p).p_write };
    }
    return rc;
}

///* Return true if list pList (linked by IdxConstraint.pLink) contains
///* a constraint compatible with *p. Otherwise return false.
extern "C" fn idx_find_constraint(p_list_1: *mut IdxConstraint,
    p: &IdxConstraint) -> i32 {
    let mut p_cmp: *const IdxConstraint = core::ptr::null();
    {
        p_cmp = p_list_1;
        '__b26: loop {
            if !(!(p_cmp).is_null()) { break '__b26; }
            '__c26: loop {
                if (*p).i_col == unsafe { (*p_cmp).i_col } { return 1; }
                break '__c26;
            }
            p_cmp = unsafe { (*p_cmp).p_link };
        }
    }
    return 0;
}

///* Search database dbm for an index compatible with the one idxCreateFromCons()
///* would create from arguments pScan, pEq and pTail. If no error occurs and 
///* such an index is found, return non-zero. Or, if no such index is found,
///* return zero.
///*
///* If an error occurs, set *pRc to an SQLite error code and return zero.
#[allow(unused_doc_comments)]
extern "C" fn idx_find_compatible(p_rc_1: &mut i32, dbm: *mut Sqlite3,
    p_scan_1: &IdxScan, p_eq_1: *mut IdxConstraint,
    p_tail_1: *mut IdxConstraint) -> i32 {
    let z_tbl: *const i8 =
        unsafe { (*(*p_scan_1).p_tab).z_name } as *const i8;
    let mut p_idx_list: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_iter: *mut IdxConstraint = core::ptr::null_mut();
    let mut n_eq: i32 = 0;
    /// Number of elements in pEq
    let mut rc: i32 = 0;
    {
        p_iter = p_eq_1;
        '__b27: loop {
            if !(!(p_iter).is_null()) { break '__b27; }
            '__c27: loop {
                { let __p = &mut n_eq; let __t = *__p; *__p += 1; __t };
                break '__c27;
            }
            p_iter = unsafe { (*p_iter).p_link };
        }
    }
    rc =
        unsafe {
            idx_printf_prepare_stmt(dbm, &mut p_idx_list,
                core::ptr::null_mut(),
                c"PRAGMA index_list=%Q".as_ptr() as *mut i8 as *const i8,
                z_tbl)
        };
    while rc == 0 && unsafe { sqlite3_step(p_idx_list) } == 100 {
        let mut b_match: i32 = 1;
        let mut p_t: *const IdxConstraint = p_tail_1 as *const IdxConstraint;
        let mut p_info: *mut Sqlite3Stmt = core::ptr::null_mut();
        let z_idx: *const i8 =
            unsafe { sqlite3_column_text(p_idx_list, 1) } as *const i8;
        if z_idx == core::ptr::null() { continue; }
        {
            p_iter = p_eq_1;
            '__b29: loop {
                if !(!(p_iter).is_null()) { break '__b29; }
                '__c29: loop {
                    unsafe { (*p_iter).b_flag = 0 };
                    break '__c29;
                }
                p_iter = unsafe { (*p_iter).p_link };
            }
        }
        rc =
            unsafe {
                idx_printf_prepare_stmt(dbm, &mut p_info,
                    core::ptr::null_mut(),
                    c"PRAGMA index_xInfo=%Q".as_ptr() as *mut i8 as *const i8,
                    z_idx)
            };
        while rc == 0 && unsafe { sqlite3_step(p_info) } == 100 {
            let i_idx: i32 = unsafe { sqlite3_column_int(p_info, 0) };
            let i_col: i32 = unsafe { sqlite3_column_int(p_info, 1) };
            let z_coll: *const i8 =
                unsafe { sqlite3_column_text(p_info, 4) } as *const i8;
            if i_idx < n_eq {
                {
                    p_iter = p_eq_1;
                    '__b31: loop {
                        if !(!(p_iter).is_null()) { break '__b31; }
                        '__c31: loop {
                            if unsafe { (*p_iter).b_flag } != 0 { break '__c31; }
                            if unsafe { (*p_iter).i_col } != i_col { break '__c31; }
                            if unsafe {
                                        sqlite3_stricmp(unsafe { (*p_iter).z_coll } as *const i8,
                                            z_coll)
                                    } != 0 {
                                break '__c31;
                            }
                            unsafe { (*p_iter).b_flag = 1 };
                            break '__b31;
                            break '__c31;
                        }
                        p_iter = unsafe { (*p_iter).p_link };
                    }
                }
                if p_iter == core::ptr::null_mut() { b_match = 0; break; }
            } else {
                if !(p_t).is_null() {
                    if unsafe { (*p_t).i_col } != i_col ||
                            unsafe {
                                    sqlite3_stricmp(unsafe { (*p_t).z_coll } as *const i8,
                                        z_coll)
                                } != 0 {
                        b_match = 0;
                        break;
                    }
                    p_t = unsafe { (*p_t).p_link };
                }
            }
        }
        idx_finalize(&mut rc, p_info);
        if rc == 0 && b_match != 0 {
            unsafe { sqlite3_finalize(p_idx_list) };
            return 1;
        }
    }
    idx_finalize(&mut rc, p_idx_list);
    *p_rc_1 = rc;
    return 0;
}

///* Return true if zId must be quoted in order to use it as an SQL
///* identifier, or false otherwise.
extern "C" fn idx_identifier_requires_quotes(z_id_1: *const i8) -> i32 {
    let mut i: i32 = 0;
    let n_id: i32 = unsafe { strlen(z_id_1) } as i32;
    if unsafe { sqlite3_keyword_check(z_id_1, n_id) } != 0 { return 1; }
    {
        i = 0;
        '__b32: loop {
            if !(unsafe { *z_id_1.offset(i as isize) } != 0) { break '__b32; }
            '__c32: loop {
                if !(unsafe { *z_id_1.offset(i as isize) } as i32 ==
                                                '_' as i32) as i32 != 0 &&
                                !(unsafe { *z_id_1.offset(i as isize) } as i32 >= '0' as i32
                                                &&
                                                unsafe { *z_id_1.offset(i as isize) } as i32 <= '9' as i32)
                                        as i32 != 0 &&
                            !(unsafe { *z_id_1.offset(i as isize) } as i32 >= 'a' as i32
                                            &&
                                            unsafe { *z_id_1.offset(i as isize) } as i32 <= 'z' as i32)
                                    as i32 != 0 &&
                        !(unsafe { *z_id_1.offset(i as isize) } as i32 >= 'A' as i32
                                        &&
                                        unsafe { *z_id_1.offset(i as isize) } as i32 <= 'Z' as i32)
                                as i32 != 0 {
                    return 1;
                }
                break '__c32;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* This function appends an index column definition suitable for constraint
///* pCons to the string passed as zIn and returns the result.
extern "C" fn idx_append_col_defn(p_rc_1: *mut i32, z_in_1: *mut i8,
    p_tab_1: &IdxTable, p_cons_1: &IdxConstraint) -> *mut i8 {
    let mut z_ret: *mut i8 = z_in_1;
    let p: *const IdxColumn =
        unsafe {
                &raw mut *(*p_tab_1).a_col.offset((*p_cons_1).i_col as isize)
            } as *const IdxColumn;
    if !(z_ret).is_null() {
        z_ret =
            unsafe {
                idx_append_text(unsafe { &mut *p_rc_1 }, z_ret,
                    c", ".as_ptr() as *mut i8 as *const i8)
            };
    }
    if idx_identifier_requires_quotes(unsafe { (*p).z_name } as *const i8) !=
            0 {
        z_ret =
            unsafe {
                idx_append_text(unsafe { &mut *p_rc_1 }, z_ret,
                    c"%Q".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).z_name })
            };
    } else {
        z_ret =
            unsafe {
                idx_append_text(unsafe { &mut *p_rc_1 }, z_ret,
                    c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).z_name })
            };
    }
    if unsafe {
                sqlite3_stricmp(unsafe { (*p).z_coll } as *const i8,
                    (*p_cons_1).z_coll as *const i8)
            } != 0 {
        if idx_identifier_requires_quotes((*p_cons_1).z_coll as *const i8) !=
                0 {
            z_ret =
                unsafe {
                    idx_append_text(unsafe { &mut *p_rc_1 }, z_ret,
                        c" COLLATE %Q".as_ptr() as *mut i8 as *const i8,
                        (*p_cons_1).z_coll)
                };
        } else {
            z_ret =
                unsafe {
                    idx_append_text(unsafe { &mut *p_rc_1 }, z_ret,
                        c" COLLATE %s".as_ptr() as *mut i8 as *const i8,
                        (*p_cons_1).z_coll)
                };
        }
    }
    if (*p_cons_1).b_desc != 0 {
        z_ret =
            unsafe {
                idx_append_text(unsafe { &mut *p_rc_1 }, z_ret,
                    c" DESC".as_ptr() as *mut i8 as *const i8)
            };
    }
    return z_ret;
}

/// Callback for sqlite3_exec() with query with leading count(*) column.
///The first argument is expected to be an int*, referent to be incremented
///if that leading column is not exactly '0'.
extern "C" fn count_nonzeros(p_count_1: *mut (), nc: i32,
    az_results_1: *mut *mut i8, az_columns_1: *mut *mut i8) -> i32 {
    { let _ = az_columns_1; };
    if nc > 0 &&
            (unsafe {
                            *unsafe {
                                    (*az_results_1.offset(0 as isize)).offset(0 as isize)
                                }
                        } as i32 != '0' as i32 ||
                unsafe {
                            *unsafe {
                                    (*az_results_1.offset(0 as isize)).offset(1 as isize)
                                }
                        } as i32 != 0) {
        unsafe { *(p_count_1 as *mut i32) += 1 };
    }
    return 0;
}

///* Return the index of the hash bucket that the string specified by the
///* arguments to this function belongs.
extern "C" fn idx_hash_string(z: &[i8]) -> i32 {
    let mut ret: u32 = 0 as u32;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b33: loop {
            if !(i < z.len() as i32) { break '__b33; }
            '__c33: loop {
                ret += (ret << 3) + z[i as usize] as u8 as u32;
                break '__c33;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (ret % 1023 as u32) as i32;
}

///* If zKey is already present in the hash table, return non-zero and do
///* nothing. Otherwise, add an entry with key zKey and payload string zVal to
///* the hash table passed as the second argument.
extern "C" fn idx_hash_add(p_rc_1: *mut i32, p_hash_1: &mut IdxHash,
    z_key_1: *const i8, z_val_1: *const i8) -> i32 {
    let n_key: i32 = unsafe { strlen(z_key_1) } as i32;
    let i_hash: i32 =
        idx_hash_string(unsafe {
                let __p = z_key_1 as *const i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_key as usize) }
            });
    let n_val: i32 =
        if !(z_val_1).is_null() {
            (unsafe { strlen(z_val_1) }) as i32
        } else { 0 };
    let mut p_entry: *mut IdxHashEntry = core::ptr::null_mut();
    if !(i_hash >= 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxHashAdd".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 240,
                c"iHash>=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        p_entry = (*p_hash_1).a_hash[i_hash as usize];
        '__b34: loop {
            if !(!(p_entry).is_null()) { break '__b34; }
            '__c34: loop {
                if unsafe { strlen(unsafe { (*p_entry).z_key } as *const i8) }
                                as i32 == n_key &&
                        0 ==
                            unsafe {
                                memcmp(unsafe { (*p_entry).z_key } as *const (),
                                    z_key_1 as *const (), n_key as u64)
                            } {
                    return 1;
                }
                break '__c34;
            }
            p_entry = unsafe { (*p_entry).p_hash_next };
        }
    }
    p_entry =
        idx_malloc(unsafe { &mut *p_rc_1 },
                (core::mem::size_of::<IdxHashEntry>() as u64 +
                                    n_key as i64 as u64 + 1 as u64 + n_val as i64 as u64 +
                        1 as u64) as i64) as *mut IdxHashEntry;
    if !(p_entry).is_null() {
        unsafe {
            (*p_entry).z_key =
                unsafe { &raw mut *p_entry.offset(1 as isize) } as *mut i8
        };
        unsafe {
            memcpy(unsafe { (*p_entry).z_key } as *mut (),
                z_key_1 as *const (), n_key as u64)
        };
        if !(z_val_1).is_null() {
            unsafe {
                (*p_entry).z_val =
                    unsafe {
                        unsafe { (*p_entry).z_key.offset((n_key + 1) as isize) }
                    }
            };
            unsafe {
                memcpy(unsafe { (*p_entry).z_val } as *mut (),
                    z_val_1 as *const (), n_val as u64)
            };
        }
        unsafe {
            (*p_entry).p_hash_next = (*p_hash_1).a_hash[i_hash as usize]
        };
        (*p_hash_1).a_hash[i_hash as usize] = p_entry;
        unsafe { (*p_entry).p_next = (*p_hash_1).p_first };
        (*p_hash_1).p_first = p_entry;
    }
    return 0;
}

#[allow(unused_doc_comments)]
extern "C" fn idx_create_from_cons(p: &mut Sqlite3expert,
    p_scan_1: *mut IdxScan, p_eq_1: *mut IdxConstraint,
    p_tail_1: *mut IdxConstraint) -> i32 {
    let dbm: *mut Sqlite3 = (*p).dbm;
    let mut rc: i32 = 0;
    if (!(p_eq_1).is_null() || !(p_tail_1).is_null()) &&
            0 ==
                idx_find_compatible(&mut rc, dbm, unsafe { &*p_scan_1 },
                    p_eq_1, p_tail_1) {
        let p_tab: *mut IdxTable = unsafe { (*p_scan_1).p_tab };
        let mut z_cols: *mut i8 = core::ptr::null_mut();
        let mut z_idx: *mut i8 = core::ptr::null_mut();
        let mut p_cons: *mut IdxConstraint = core::ptr::null_mut();
        let mut h: u32 = 0 as u32;
        let mut z_fmt: *const i8 = core::ptr::null();
        {
            p_cons = p_eq_1;
            '__b35: loop {
                if !(!(p_cons).is_null()) { break '__b35; }
                '__c35: loop {
                    z_cols =
                        idx_append_col_defn(&mut rc, z_cols, unsafe { &*p_tab },
                            unsafe { &*p_cons });
                    break '__c35;
                }
                p_cons = unsafe { (*p_cons).p_link };
            }
        }
        {
            p_cons = p_tail_1;
            '__b36: loop {
                if !(!(p_cons).is_null()) { break '__b36; }
                '__c36: loop {
                    z_cols =
                        idx_append_col_defn(&mut rc, z_cols, unsafe { &*p_tab },
                            unsafe { &*p_cons });
                    break '__c36;
                }
                p_cons = unsafe { (*p_cons).p_link };
            }
        }
        if rc == 0 {
            /// Hash the list of columns to come up with a name for the index
            let z_table: *const i8 =
                unsafe { (*unsafe { (*p_scan_1).p_tab }).z_name } as
                    *const i8;
            let quote_table: i32 = idx_identifier_requires_quotes(z_table);
            let mut z_name: *mut i8 = core::ptr::null_mut();
            /// Index name
            let mut collisions: i32 = 0;
            '__b37: loop {
                '__c37: loop {
                    let mut i: i32 = 0;
                    let mut z_find: *mut i8 = core::ptr::null_mut();
                    {
                        i = 0;
                        '__b38: loop {
                            if !(unsafe { *z_cols.offset(i as isize) } != 0) {
                                break '__b38;
                            }
                            '__c38: loop {
                                h +=
                                    (h << 3) + unsafe { *z_cols.offset(i as isize) } as u32;
                                break '__c38;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { sqlite3_free(z_name as *mut ()) };
                    z_name =
                        unsafe {
                            sqlite3_mprintf(c"%s_idx_%08x".as_ptr() as *mut i8 as
                                    *const i8, z_table, h)
                        };
                    if z_name == core::ptr::null_mut() { break '__b37; }

                    /// Is is unique among table, view and index names?
                    (z_fmt =
                        c"SELECT count(*) FROM sqlite_schema WHERE name=%Q AND type in (\'index\',\'table\',\'view\')".as_ptr()
                                as *mut i8 as *const i8);
                    z_find = unsafe { sqlite3_mprintf(z_fmt, z_name) };
                    i = 0;
                    rc =
                        unsafe {
                            sqlite3_exec(dbm, z_find as *const i8, Some(count_nonzeros),
                                &raw mut i as *mut (), core::ptr::null_mut())
                        };
                    if !(rc == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"idxCreateFromCons".as_ptr() as *const i8,
                                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1007,
                                c"rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe { sqlite3_free(z_find as *mut ()) };
                    if i == 0 { collisions = 0; break '__b37; }
                    { let __p = &mut collisions; *__p += 1; *__p };
                    break '__c37;
                }
                if !(collisions < 50 && z_name != core::ptr::null_mut()) {
                    break '__b37;
                }
            }
            if collisions != 0 {

                /// This return means "Gave up trying to find a unique index name."
                (rc = 5 | 3 << 8);
            } else if z_name == core::ptr::null_mut() {
                rc = 7;
            } else {
                if quote_table != 0 {
                    z_fmt =
                        c"CREATE INDEX \"%w\" ON \"%w\"(%s)".as_ptr() as *mut i8 as
                            *const i8;
                } else {
                    z_fmt =
                        c"CREATE INDEX %s ON %s(%s)".as_ptr() as *mut i8 as
                            *const i8;
                }
                z_idx =
                    unsafe { sqlite3_mprintf(z_fmt, z_name, z_table, z_cols) };
                if (z_idx).is_null() as i32 != 0 {
                    rc = 7;
                } else {
                    rc =
                        unsafe {
                            sqlite3_exec(dbm, z_idx as *const i8, None,
                                core::ptr::null_mut(), (*p).pz_errmsg)
                        };
                    if rc != 0 {
                        rc = 5 | 3 << 8;
                    } else {
                        idx_hash_add(&mut rc, &mut (*p).h_idx, z_name as *const i8,
                            z_idx as *const i8);
                    }
                }
                unsafe { sqlite3_free(z_name as *mut ()) };
                unsafe { sqlite3_free(z_idx as *mut ()) };
            }
        }
        unsafe { sqlite3_free(z_cols as *mut ()) };
    }
    return rc;
}

#[allow(unused_doc_comments)]
extern "C" fn idx_create_from_where(p: *mut Sqlite3expert,
    p_scan_1: *mut IdxScan, p_tail_1: *mut IdxConstraint) -> i32 {
    let mut p1: *mut IdxConstraint = core::ptr::null_mut();
    let mut p_con: *mut IdxConstraint = core::ptr::null_mut();
    let mut rc: i32 = 0;
    {
        p_con = unsafe { (*p_scan_1).p_eq };
        '__b39: loop {
            if !(!(p_con).is_null()) { break '__b39; }
            '__c39: loop {
                if (idx_find_constraint(p1, unsafe { &*p_con }) == 0) as i32
                            != 0 &&
                        (idx_find_constraint(p_tail_1, unsafe { &*p_con }) == 0) as
                                i32 != 0 {
                    unsafe { (*p_con).p_link = p1 };
                    p1 = p_con;
                }
                break '__c39;
            }
            p_con = unsafe { (*p_con).p_next };
        }
    }

    /// Create an index using the == constraints collected above. And the
    ///* range constraint/ORDER BY terms passed in by the caller, if any.
    (rc = idx_create_from_cons(unsafe { &mut *p }, p_scan_1, p1, p_tail_1));
    if p_tail_1 == core::ptr::null_mut() {
        {
            p_con = unsafe { (*p_scan_1).p_range };
            '__b40: loop {
                if !(rc == 0 && !(p_con).is_null()) { break '__b40; }
                '__c40: loop {
                    if !(unsafe { (*p_con).p_link } == core::ptr::null_mut()) as
                                    i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"idxCreateFromWhere".as_ptr() as *const i8,
                                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1084,
                                c"pCon->pLink==0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if (idx_find_constraint(p1, unsafe { &*p_con }) == 0) as i32
                                != 0 &&
                            (idx_find_constraint(p_tail_1, unsafe { &*p_con }) == 0) as
                                    i32 != 0 {
                        rc =
                            idx_create_from_cons(unsafe { &mut *p }, p_scan_1, p1,
                                p_con);
                    }
                    break '__c40;
                }
                p_con = unsafe { (*p_con).p_next };
            }
        }
    }
    return rc;
}

///* Create candidate indexes in database [dbm] based on the data in 
///* linked-list pScan.
extern "C" fn idx_create_candidates(p: *mut Sqlite3expert) -> i32 {
    let mut rc: i32 = 0;
    let mut p_iter: *mut IdxScan = core::ptr::null_mut();
    {
        p_iter = unsafe { (*p).p_scan };
        '__b41: loop {
            if !(!(p_iter).is_null() && rc == 0) { break '__b41; }
            '__c41: loop {
                rc = idx_create_from_where(p, p_iter, core::ptr::null_mut());
                if rc == 0 && !(unsafe { (*p_iter).p_order }).is_null() {
                    rc =
                        idx_create_from_where(p, p_iter,
                            unsafe { (*p_iter).p_order });
                }
                break '__c41;
            }
            p_iter = unsafe { (*p_iter).p_next_scan };
        }
    }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxRemCtx {
    n_slot: i32,
    a_slot: [IdxRemSlot; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxRemSlot {
    e_type: i32,
    i_val: i64,
    r_val: f64,
    n_byte: i64,
    n: i64,
    z: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxSampleCtx {
    i_target: i32,
    target: f64,
    n_row: f64,
    n_ret: f64,
}

extern "C" fn idx_largest_index(db: *mut Sqlite3, pn_max_1: &mut i32,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let z_max: *const i8 =
        c"SELECT max(i.seqno) FROM   sqlite_schema AS s,   pragma_index_list(s.name) AS l,   pragma_index_info(l.name) AS i WHERE s.type = \'table\'".as_ptr()
                as *mut i8 as *const i8;
    let mut p_max: *mut Sqlite3Stmt = core::ptr::null_mut();
    *pn_max_1 = 0;
    rc = idx_prepare_stmt(db, &mut p_max, pz_err_1, z_max);
    if rc == 0 && 100 == unsafe { sqlite3_step(p_max) } {
        *pn_max_1 = unsafe { sqlite3_column_int(p_max, 0) } + 1;
    }
    idx_finalize(&mut rc, p_max);
    return rc;
}

///* Implementation of scalar function sqlite_expert_rem().
extern "C" fn idx_rem_func(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let p: *mut IdxRemCtx =
        unsafe { sqlite3_user_data(p_ctx_1) } as *mut IdxRemCtx;
    let mut p_slot: *mut IdxRemSlot = core::ptr::null_mut();
    let mut i_slot: i32 = 0;
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxRemFunc".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1582,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    i_slot =
        unsafe { sqlite3_value_int(unsafe { *argv.offset(0 as isize) }) };
    if !(i_slot < unsafe { (*p).n_slot }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxRemFunc".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1585,
                c"iSlot<p->nSlot".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_slot = unsafe { &mut (*p).a_slot[i_slot as usize] };
    '__s42:
        {
        match unsafe { (*p_slot).e_type } {
            5 => {}
            1 => {
                unsafe {
                    sqlite3_result_int64(p_ctx_1, unsafe { (*p_slot).i_val })
                };
            }
            2 => {
                unsafe {
                    sqlite3_result_double(p_ctx_1, unsafe { (*p_slot).r_val })
                };
            }
            4 => {
                if !(unsafe { (*p_slot).n } <= 2147483647 as i64) as i32 as
                            i64 != 0 {
                    unsafe {
                        __assert_rtn(c"idxRemFunc".as_ptr() as *const i8,
                            c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1602,
                            c"pSlot->n <= 0x7fffffff".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                unsafe {
                    sqlite3_result_blob(p_ctx_1,
                        unsafe { (*p_slot).z } as *const (),
                        unsafe { (*p_slot).n } as i32,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            }
            3 => {
                if !(unsafe { (*p_slot).n } <= 2147483647 as i64) as i32 as
                            i64 != 0 {
                    unsafe {
                        __assert_rtn(c"idxRemFunc".as_ptr() as *const i8,
                            c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1607,
                            c"pSlot->n <= 0x7fffffff".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                unsafe {
                    sqlite3_result_text(p_ctx_1,
                        unsafe { (*p_slot).z } as *const i8,
                        unsafe { (*p_slot).n } as i32,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            }
            _ => {}
        }
    }
    unsafe {
        (*p_slot).e_type =
            unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) }
    };
    '__s43:
        {
        match unsafe { (*p_slot).e_type } {
            5 => {}
            1 => {
                unsafe {
                    (*p_slot).i_val =
                        unsafe {
                            sqlite3_value_int64(unsafe { *argv.offset(1 as isize) })
                        }
                };
            }
            2 => {
                unsafe {
                    (*p_slot).r_val =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(1 as isize) })
                        }
                };
            }
            4 => {
                {
                    let n_byte: i64 =
                        unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                            } as i64;
                    let mut p_data: *const () = core::ptr::null();
                    if n_byte > unsafe { (*p_slot).n_byte } {
                        let z_new: *mut i8 =
                            unsafe {
                                    sqlite3_realloc64(unsafe { (*p_slot).z } as *mut (),
                                        (n_byte * 2 as i64) as Sqlite3Uint64)
                                } as *mut i8;
                        if z_new == core::ptr::null_mut() {
                            unsafe { sqlite3_result_error_nomem(p_ctx_1) };
                            return;
                        }
                        unsafe { (*p_slot).n_byte = n_byte * 2 as i64 };
                        unsafe { (*p_slot).z = z_new };
                    }
                    unsafe { (*p_slot).n = n_byte };
                    if unsafe { (*p_slot).e_type } == 4 {
                        p_data =
                            unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(1 as isize) })
                            };
                        if !(p_data).is_null() {
                            unsafe {
                                memcpy(unsafe { (*p_slot).z } as *mut (), p_data,
                                    n_byte as u64)
                            };
                        }
                    } else {
                        p_data =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                                } as *const ();
                        unsafe {
                            memcpy(unsafe { (*p_slot).z } as *mut (), p_data,
                                n_byte as u64)
                        };
                    }
                    break '__s43;
                }
            }
            3 => {
                {
                    let n_byte: i64 =
                        unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                            } as i64;
                    let mut p_data: *const () = core::ptr::null();
                    if n_byte > unsafe { (*p_slot).n_byte } {
                        let z_new: *mut i8 =
                            unsafe {
                                    sqlite3_realloc64(unsafe { (*p_slot).z } as *mut (),
                                        (n_byte * 2 as i64) as Sqlite3Uint64)
                                } as *mut i8;
                        if z_new == core::ptr::null_mut() {
                            unsafe { sqlite3_result_error_nomem(p_ctx_1) };
                            return;
                        }
                        unsafe { (*p_slot).n_byte = n_byte * 2 as i64 };
                        unsafe { (*p_slot).z = z_new };
                    }
                    unsafe { (*p_slot).n = n_byte };
                    if unsafe { (*p_slot).e_type } == 4 {
                        p_data =
                            unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(1 as isize) })
                            };
                        if !(p_data).is_null() {
                            unsafe {
                                memcpy(unsafe { (*p_slot).z } as *mut (), p_data,
                                    n_byte as u64)
                            };
                        }
                    } else {
                        p_data =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                                } as *const ();
                        unsafe {
                            memcpy(unsafe { (*p_slot).z } as *mut (), p_data,
                                n_byte as u64)
                        };
                    }
                    break '__s43;
                }
            }
            _ => {}
        }
    }
}

extern "C" fn idx_sample_func(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let p: *mut IdxSampleCtx =
        unsafe { sqlite3_user_data(p_ctx_1) } as *mut IdxSampleCtx;
    let mut b_ret: i32 = 0;
    { let _ = argv; };
    if !(argc == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxSampleFunc".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1542,
                c"argc==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p).n_row } == 0.0 {
        b_ret = 1;
    } else {
        b_ret =
            (unsafe { (*p).n_ret } / unsafe { (*p).n_row } <=
                    unsafe { (*p).target }) as i32;
        if b_ret == 0 {
            let mut rnd: u16 = 0 as u16;
            unsafe { sqlite3_randomness(2, &raw mut rnd as *mut ()) };
            b_ret = (rnd as i32 % 100 <= unsafe { (*p).i_target }) as i32;
        }
    }
    unsafe { sqlite3_result_int(p_ctx_1, b_ret) };
    unsafe { (*p).n_row += 1.0 };
    unsafe { (*p).n_ret += b_ret as f64 };
}

extern "C" fn idx_build_sample_table(p: &Sqlite3expert, z_tab_1: *const i8)
    -> i32 {
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    rc =
        unsafe {
            sqlite3_exec((*p).dbv,
                c"DROP TABLE IF EXISTS temp.t592690916721053953805701627921227776".as_ptr()
                        as *mut i8 as *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
    if rc != 0 { return rc; }
    z_sql =
        unsafe {
            sqlite3_mprintf(c"CREATE TABLE temp.t592690916721053953805701627921227776 AS SELECT * FROM %Q".as_ptr()
                        as *mut i8 as *const i8, z_tab_1)
        };
    if z_sql == core::ptr::null_mut() { return 7; }
    rc =
        unsafe {
            sqlite3_exec((*p).dbv, z_sql as *const i8, None,
                core::ptr::null_mut(), core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    return rc;
}

///* If zKey/nKey is present in the hash table, return a pointer to the 
///* hash-entry object.
extern "C" fn idx_hash_find(p_hash_1: &IdxHash, z_key_1: *const i8,
    mut n_key_1: i32) -> *mut IdxHashEntry {
    let mut i_hash: i32 = 0;
    let mut p_entry: *mut IdxHashEntry = core::ptr::null_mut();
    if n_key_1 < 0 { n_key_1 = unsafe { strlen(z_key_1) } as i32; }
    i_hash =
        idx_hash_string(unsafe {
                let __p = z_key_1 as *const i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_key_1 as usize) }
            });
    if !(i_hash >= 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxHashFind".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 272,
                c"iHash>=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        p_entry = (*p_hash_1).a_hash[i_hash as usize];
        '__b44: loop {
            if !(!(p_entry).is_null()) { break '__b44; }
            '__c44: loop {
                if unsafe { strlen(unsafe { (*p_entry).z_key } as *const i8) }
                                as i32 == n_key_1 &&
                        0 ==
                            unsafe {
                                memcmp(unsafe { (*p_entry).z_key } as *const (),
                                    z_key_1 as *const (), n_key_1 as u64)
                            } {
                    return p_entry;
                }
                break '__c44;
            }
            p_entry = unsafe { (*p_entry).p_hash_next };
        }
    }
    return core::ptr::null_mut();
}

#[allow(unused_doc_comments)]
extern "C" fn idx_populate_one_stat1(p: &mut Sqlite3expert,
    p_index_x_info_1: *mut Sqlite3Stmt, p_write_stat_1: *mut Sqlite3Stmt,
    z_tab_1: *const i8, z_idx_1: *const i8, pz_err_1: *mut *mut i8) -> i32 {
    let mut z_cols: *mut i8 = core::ptr::null_mut();
    let mut z_order: *mut i8 = core::ptr::null_mut();
    let mut z_query: *mut i8 = core::ptr::null_mut();
    let mut n_col: i32 = 0;
    let mut i: i32 = 0;
    let mut p_query: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut a_stat: *mut i64 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if !((*p).i_sample > 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"idxPopulateOneStat1".as_ptr() as *const i8,
                c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1689,
                c"p->iSample>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };

    /// Formulate the query text
    unsafe { sqlite3_bind_text(p_index_x_info_1, 1, z_idx_1, -1, None) };
    while 0 == rc && 100 == unsafe { sqlite3_step(p_index_x_info_1) } {
        let z_comma: *const i8 =
            if z_cols == core::ptr::null_mut() {
                    c"".as_ptr() as *mut i8
                } else { c", ".as_ptr() as *mut i8 } as *const i8;
        let z_name: *const i8 =
            unsafe { sqlite3_column_text(p_index_x_info_1, 0) } as *const i8;
        let z_coll: *const i8 =
            unsafe { sqlite3_column_text(p_index_x_info_1, 1) } as *const i8;
        if z_name == core::ptr::null() {

            /// This index contains an expression. Ignore it.
            unsafe { sqlite3_free(z_cols as *mut ()) };
            unsafe { sqlite3_free(z_order as *mut ()) };
            return unsafe { sqlite3_reset(p_index_x_info_1) };
        }
        z_cols =
            unsafe {
                idx_append_text(&mut rc, z_cols,
                    c"%sx.%Q IS sqlite_expert_rem(%d, x.%Q) COLLATE %Q".as_ptr()
                            as *mut i8 as *const i8, z_comma, z_name, n_col, z_name,
                    z_coll)
            };
        z_order =
            unsafe {
                idx_append_text(&mut rc, z_order,
                    c"%s%d".as_ptr() as *mut i8 as *const i8, z_comma,
                    { let __p = &mut n_col; *__p += 1; *__p })
            };
    }
    unsafe { sqlite3_reset(p_index_x_info_1) };
    if rc == 0 {
        if (*p).i_sample == 100 {
            z_query =
                unsafe {
                    sqlite3_mprintf(c"SELECT %s FROM %Q x ORDER BY %s".as_ptr()
                                as *mut i8 as *const i8, z_cols, z_tab_1, z_order)
                };
        } else {
            z_query =
                unsafe {
                    sqlite3_mprintf(c"SELECT %s FROM temp.t592690916721053953805701627921227776 x ORDER BY %s".as_ptr()
                                as *mut i8 as *const i8, z_cols, z_order)
                };
        }
    }
    unsafe { sqlite3_free(z_cols as *mut ()) };
    unsafe { sqlite3_free(z_order as *mut ()) };
    if rc == 0 {
        let dbrem: *mut Sqlite3 =
            if (*p).i_sample == 100 { (*p).db } else { (*p).dbv };
        rc =
            idx_prepare_stmt(dbrem, &mut p_query, pz_err_1,
                z_query as *const i8);
    }
    unsafe { sqlite3_free(z_query as *mut ()) };
    if rc == 0 {
        a_stat =
            idx_malloc(&mut rc,
                    (core::mem::size_of::<i64>() as u64 * (n_col + 1) as u64) as
                        i64) as *mut i64;
    }
    if rc == 0 && 100 == unsafe { sqlite3_step(p_query) } {
        let mut p_entry: *mut IdxHashEntry = core::ptr::null_mut();
        let mut z_stat: *mut i8 = core::ptr::null_mut();
        {
            i = 0;
            '__b46: loop {
                if !(i <= n_col) { break '__b46; }
                '__c46: loop {
                    unsafe { *a_stat.offset(i as isize) = 1 as i64 };
                    break '__c46;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        while rc == 0 && 100 == unsafe { sqlite3_step(p_query) } {
            {
                let __p = unsafe { &mut *a_stat.offset(0 as isize) };
                let __t = *__p;
                *__p += 1;
                __t
            };
            {
                i = 0;
                '__b48: loop {
                    if !(i < n_col) { break '__b48; }
                    '__c48: loop {
                        if unsafe { sqlite3_column_int(p_query, i) } == 0 {
                            break '__b48;
                        }
                        break '__c48;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                '__b49: loop {
                    if !(i < n_col) { break '__b49; }
                    '__c49: loop {
                        {
                            let __p = unsafe { &mut *a_stat.offset((i + 1) as isize) };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        break '__c49;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if rc == 0 {
            let s0: i64 = unsafe { *a_stat.offset(0 as isize) };
            z_stat =
                unsafe {
                    sqlite3_mprintf(c"%lld".as_ptr() as *mut i8 as *const i8,
                        s0)
                };
            if z_stat == core::ptr::null_mut() { rc = 7; }
            {
                i = 1;
                '__b50: loop {
                    if !(rc == 0 && i <= n_col) { break '__b50; }
                    '__c50: loop {
                        z_stat =
                            unsafe {
                                idx_append_text(&mut rc, z_stat,
                                    c" %lld".as_ptr() as *mut i8 as *const i8,
                                    (s0 + unsafe { *a_stat.offset(i as isize) } / 2 as i64) /
                                        unsafe { *a_stat.offset(i as isize) })
                            };
                        break '__c50;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if rc == 0 {
            unsafe {
                sqlite3_bind_text(p_write_stat_1, 1, z_tab_1, -1, None)
            };
            unsafe {
                sqlite3_bind_text(p_write_stat_1, 2, z_idx_1, -1, None)
            };
            unsafe {
                sqlite3_bind_text(p_write_stat_1, 3, z_stat as *const i8, -1,
                    None)
            };
            unsafe { sqlite3_step(p_write_stat_1) };
            rc = unsafe { sqlite3_reset(p_write_stat_1) };
        }
        p_entry =
            idx_hash_find(&(*p).h_idx, z_idx_1,
                unsafe { strlen(z_idx_1) } as i32);
        if !(p_entry).is_null() {
            if !(unsafe { (*p_entry).z_val2 } == core::ptr::null_mut()) as i32
                        as i64 != 0 {
                unsafe {
                    __assert_rtn(c"idxPopulateOneStat1".as_ptr() as *const i8,
                        c"sqlite3expert.c".as_ptr() as *mut i8 as *const i8, 1767,
                        c"pEntry->zVal2==0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            unsafe { (*p_entry).z_val2 = z_stat };
        } else { unsafe { sqlite3_free(z_stat as *mut ()) }; }
    }
    unsafe { sqlite3_free(a_stat as *mut ()) };
    idx_finalize(&mut rc, p_query);
    return rc;
}

///* This function is called as part of sqlite3_expert_analyze(). Candidate
///* indexes have already been created in database sqlite3expert.dbm, this
///* function populates sqlite_stat1 table in the same database.
///*
///* The stat1 data is generated by querying the
extern "C" fn idx_populate_stat1(p: *mut Sqlite3expert,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut n_max: i32 = 0;
    let mut p_ctx: *mut IdxRemCtx = core::ptr::null_mut();
    let mut samplectx: IdxSampleCtx = unsafe { core::mem::zeroed() };
    let mut i: i32 = 0;
    let mut i_prev: i64 = -100000 as i64;
    let mut p_all_index: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_index_x_info: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_write: *mut Sqlite3Stmt = core::ptr::null_mut();
    let z_all_index: *const i8 =
        c"SELECT s.rowid, s.name, l.name FROM   sqlite_schema AS s,   pragma_index_list(s.name) AS l WHERE s.type = \'table\'".as_ptr()
                as *mut i8 as *const i8;
    let z_index_x_info: *const i8 =
        c"SELECT name, coll FROM pragma_index_xinfo(?) WHERE key".as_ptr() as
                *mut i8 as *const i8;
    let z_write: *const i8 =
        c"INSERT INTO sqlite_stat1 VALUES(?, ?, ?)".as_ptr() as *mut i8 as
            *const i8;
    if unsafe { (*p).i_sample } == 0 { return 0; }
    rc = idx_largest_index(unsafe { (*p).dbm }, &mut n_max, pz_err_1);
    if n_max <= 0 || rc != 0 { return rc; }
    rc =
        unsafe {
            sqlite3_exec(unsafe { (*p).dbm },
                c"ANALYZE; PRAGMA writable_schema=1".as_ptr() as *mut i8 as
                    *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
    if rc == 0 {
        let n_byte: i64 =
            (core::mem::size_of::<IdxRemCtx>() as u64 +
                    core::mem::size_of::<IdxRemSlot>() as u64 * n_max as u64) as
                i64;
        p_ctx = idx_malloc(&mut rc, n_byte) as *mut IdxRemCtx;
    }
    if rc == 0 {
        let dbrem: *mut Sqlite3 =
            if unsafe { (*p).i_sample } == 100 {
                unsafe { (*p).db }
            } else { unsafe { (*p).dbv } };
        rc =
            unsafe {
                sqlite3_create_function(dbrem,
                    c"sqlite_expert_rem".as_ptr() as *mut i8 as *const i8, 2, 1,
                    p_ctx as *mut (), Some(idx_rem_func), None, None)
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(unsafe { (*p).db },
                    c"sqlite_expert_sample".as_ptr() as *mut i8 as *const i8, 0,
                    1, &raw mut samplectx as *mut (), Some(idx_sample_func),
                    None, None)
            };
    }
    if rc == 0 {
        unsafe { (*p_ctx).n_slot = (n_max as i64 + 1 as i64) as i32 };
        rc =
            idx_prepare_stmt(unsafe { (*p).dbm }, &mut p_all_index, pz_err_1,
                z_all_index);
    }
    if rc == 0 {
        rc =
            idx_prepare_stmt(unsafe { (*p).dbm }, &mut p_index_x_info,
                pz_err_1, z_index_x_info);
    }
    if rc == 0 {
        rc =
            idx_prepare_stmt(unsafe { (*p).dbm }, &mut p_write, pz_err_1,
                z_write);
    }
    while rc == 0 && 100 == unsafe { sqlite3_step(p_all_index) } {
        let i_rowid: i64 = unsafe { sqlite3_column_int64(p_all_index, 0) };
        let z_tab: *const i8 =
            unsafe { sqlite3_column_text(p_all_index, 1) } as *const i8;
        let z_idx: *const i8 =
            unsafe { sqlite3_column_text(p_all_index, 2) } as *const i8;
        if z_tab == core::ptr::null() || z_idx == core::ptr::null() {
            continue;
        }
        if unsafe { (*p).i_sample } < 100 && i_prev != i_rowid {
            samplectx.target = unsafe { (*p).i_sample } as f64 / 100.0;
            samplectx.i_target = unsafe { (*p).i_sample };
            samplectx.n_row = 0.0;
            samplectx.n_ret = 0.0;
            rc = idx_build_sample_table(unsafe { &*p }, z_tab);
            if rc != 0 { break; }
        }
        rc =
            idx_populate_one_stat1(unsafe { &mut *p }, p_index_x_info,
                p_write, z_tab, z_idx, pz_err_1);
        i_prev = i_rowid;
    }
    if rc == 0 && unsafe { (*p).i_sample } < 100 {
        rc =
            unsafe {
                sqlite3_exec(unsafe { (*p).dbv },
                    c"DROP TABLE IF EXISTS temp.t592690916721053953805701627921227776".as_ptr()
                            as *mut i8 as *const i8, None, core::ptr::null_mut(),
                    core::ptr::null_mut())
            };
    }
    idx_finalize(&mut rc, p_all_index);
    idx_finalize(&mut rc, p_index_x_info);
    idx_finalize(&mut rc, p_write);
    if !(p_ctx).is_null() {
        {
            i = 0;
            '__b52: loop {
                if !(i < unsafe { (*p_ctx).n_slot }) { break '__b52; }
                '__c52: loop {
                    unsafe {
                        sqlite3_free(unsafe { (*p_ctx).a_slot[i as usize].z } as
                                *mut ())
                    };
                    break '__c52;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(p_ctx as *mut ()) };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_exec(unsafe { (*p).dbm },
                    c"ANALYZE sqlite_schema".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
    }
    unsafe {
        sqlite3_create_function(unsafe { (*p).db },
            c"sqlite_expert_rem".as_ptr() as *mut i8 as *const i8, 2, 1,
            core::ptr::null_mut(), None, None, None)
    };
    unsafe {
        sqlite3_create_function(unsafe { (*p).db },
            c"sqlite_expert_sample".as_ptr() as *mut i8 as *const i8, 0, 1,
            core::ptr::null_mut(), None, None, None)
    };
    unsafe {
        sqlite3_exec(unsafe { (*p).db },
            c"DROP TABLE IF EXISTS temp.t592690916721053953805701627921227776".as_ptr()
                    as *mut i8 as *const i8, None, core::ptr::null_mut(),
            core::ptr::null_mut())
    };
    return rc;
}

///* Initialize an IdxHash hash table.
extern "C" fn idx_hash_init(p_hash_1: *mut IdxHash) -> () {
    unsafe {
        memset(p_hash_1 as *mut (), 0, core::mem::size_of::<IdxHash>() as u64)
    };
}

///* If the hash table contains an entry with a key equal to the string
///* passed as the final two arguments to this function, return a pointer
///* to the payload string. Otherwise, if zKey/nKey is not present in the
///* hash table, return NULL.
extern "C" fn idx_hash_search(p_hash_1: *mut IdxHash, z_key_1: *const i8,
    n_key_1: i32) -> *const i8 {
    let p_entry: *const IdxHashEntry =
        idx_hash_find(unsafe { &*p_hash_1 }, z_key_1, n_key_1) as
            *const IdxHashEntry;
    if !(p_entry).is_null() {
        return unsafe { (*p_entry).z_val } as *const i8;
    }
    return core::ptr::null();
}

///* This function is called after candidate indexes have been created. It
///* runs all the queries to see which indexes they prefer, and populates
///* IdxStatement.zIdx and IdxStatement.zEQP with the results.
#[allow(unused_doc_comments)]
extern "C" fn idx_find_indexes(p: &mut Sqlite3expert, pz_err_1: *mut *mut i8)
    -> i32 {
    let mut p_stmt: *mut IdxStatement = core::ptr::null_mut();
    let mut dbm: *mut Sqlite3 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut h_idx: IdxHash = unsafe { core::mem::zeroed() };
    let mut p_entry: *const IdxHashEntry = core::ptr::null();
    let mut p_explain: *mut Sqlite3Stmt = core::ptr::null_mut();
    /// int iId = sqlite3_column_int(pExplain, 0); */
    ///      /* int iParent = sqlite3_column_int(pExplain, 1); */
    ///      /* int iNotUsed = sqlite3_column_int(pExplain, 2);
    let mut z_detail: *const i8 = core::ptr::null();
    let mut n_detail: i32 = 0;
    let mut i: i32 = 0;
    let mut z_idx: *const i8 = core::ptr::null();
    let mut z_sql: *const i8 = core::ptr::null();
    let mut n_idx: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s54:
            {
            match __state {
                0 => { __state = 3; }
                2 => { idx_hash_clear(&mut h_idx); __state = 48; }
                3 => { dbm = (*p).dbm; __state = 4; }
                4 => { rc = 0; __state = 5; }
                5 => { __state = 6; }
                6 => { idx_hash_init(&mut h_idx); __state = 7; }
                7 => { p_stmt = (*p).p_statement; __state = 9; }
                8 => { __state = 2; }
                9 => {
                    if rc == 0 && !(p_stmt).is_null() {
                        __state = 10;
                    } else { __state = 8; }
                }
                10 => { __state = 12; }
                11 => { p_stmt = unsafe { (*p_stmt).p_next }; __state = 9; }
                12 => { p_explain = core::ptr::null_mut(); __state = 13; }
                13 => { idx_hash_clear(&mut h_idx); __state = 14; }
                14 => {
                    rc =
                        unsafe {
                            idx_printf_prepare_stmt(dbm, &mut p_explain, pz_err_1,
                                c"EXPLAIN QUERY PLAN %s".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_stmt).z_sql })
                        };
                    __state = 15;
                }
                15 => {
                    if rc == 0 && unsafe { sqlite3_step(p_explain) } == 100 {
                        __state = 17;
                    } else { __state = 16; }
                }
                16 => { p_entry = h_idx.p_first; __state = 45; }
                17 => {
                    z_detail =
                        unsafe { sqlite3_column_text(p_explain, 3) } as *const i8;
                    __state = 18;
                }
                18 => { __state = 19; }
                19 => { __state = 20; }
                20 => {
                    if (z_detail).is_null() as i32 != 0 {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => {
                    n_detail = unsafe { strlen(z_detail) } as i32;
                    __state = 23;
                }
                22 => { __state = 15; }
                23 => { i = 0; __state = 25; }
                24 => {
                    if unsafe { *z_detail.offset(0 as isize) } as i32 !=
                            '-' as i32 {
                        __state = 43;
                    } else { __state = 15; }
                }
                25 => {
                    if i < n_detail { __state = 26; } else { __state = 24; }
                }
                26 => { z_idx = core::ptr::null(); __state = 28; }
                27 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 25;
                }
                28 => {
                    if i + 13 < n_detail &&
                            unsafe {
                                    memcmp(unsafe { &raw const *z_detail.offset(i as isize) } as
                                            *const (),
                                        c" USING INDEX ".as_ptr() as *mut i8 as *const (),
                                        13 as u64)
                                } == 0 {
                        __state = 30;
                    } else { __state = 31; }
                }
                29 => {
                    if !(z_idx).is_null() {
                        __state = 33;
                    } else { __state = 27; }
                }
                30 => {
                    z_idx = unsafe { z_detail.offset((i + 13) as isize) };
                    __state = 29;
                }
                31 => {
                    if i + 22 < n_detail &&
                            unsafe {
                                    memcmp(unsafe { &raw const *z_detail.offset(i as isize) } as
                                            *const (),
                                        c" USING COVERING INDEX ".as_ptr() as *mut i8 as *const (),
                                        22 as u64)
                                } == 0 {
                        __state = 32;
                    } else { __state = 29; }
                }
                32 => {
                    z_idx = unsafe { z_detail.offset((i + 22) as isize) };
                    __state = 29;
                }
                33 => { __state = 34; }
                34 => { n_idx = 0; __state = 35; }
                35 => {
                    if unsafe { *z_idx.offset(n_idx as isize) } as i32 !=
                                '\u{0}' as i32 &&
                            (unsafe { *z_idx.offset(n_idx as isize) } as i32 !=
                                    ' ' as i32 ||
                                unsafe { *z_idx.offset((n_idx + 1) as isize) } as i32 !=
                                    '(' as i32) {
                        __state = 37;
                    } else { __state = 36; }
                }
                36 => {
                    z_sql = idx_hash_search(&mut (*p).h_idx, z_idx, n_idx);
                    __state = 38;
                }
                37 => {
                    { let __p = &mut n_idx; let __t = *__p; *__p += 1; __t };
                    __state = 35;
                }
                38 => {
                    if !(z_sql).is_null() {
                        __state = 40;
                    } else { __state = 39; }
                }
                39 => { __state = 24; }
                40 => {
                    idx_hash_add(&mut rc, &mut h_idx, z_sql, core::ptr::null());
                    __state = 41;
                }
                41 => { if rc != 0 { __state = 42; } else { __state = 39; } }
                42 => { __state = 2; }
                43 => {
                    unsafe {
                        (*p_stmt).z_eqp =
                            unsafe {
                                idx_append_text(&mut rc, unsafe { (*p_stmt).z_eqp },
                                    c"%s\n".as_ptr() as *mut i8 as *const i8, z_detail)
                            }
                    };
                    __state = 15;
                }
                44 => { idx_finalize(&mut rc, p_explain); __state = 11; }
                45 => {
                    if !(p_entry).is_null() {
                        __state = 46;
                    } else { __state = 44; }
                }
                46 => {
                    unsafe {
                        (*p_stmt).z_idx =
                            unsafe {
                                idx_append_text(&mut rc, unsafe { (*p_stmt).z_idx },
                                    c"%s;\n".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_entry).z_key })
                            }
                    };
                    __state = 47;
                }
                47 => {
                    p_entry = unsafe { (*p_entry).p_next };
                    __state = 45;
                }
                48 => { return rc; }
                _ => {}
            }
        }
    }

    /// int iId = sqlite3_column_int(pExplain, 0); */
    ///      /* int iParent = sqlite3_column_int(pExplain, 1); */
    ///      /* int iNotUsed = sqlite3_column_int(pExplain, 2);
    unreachable!();
}

///* This function is called after the sqlite3expert object has been configured
///* with all SQL statements using sqlite3_expert_sql() to actually perform
///* the analysis. Once this function has been called, it is not possible to
///* add further SQL statements to the analysis.
///*
///* If successful, SQLITE_OK is returned and (*pzErr) is set to NULL. Or, if
///* an error occurs, an SQLite error code is returned and (*pzErr) set to 
///* point to a buffer containing an English language error message. In this
///* case it is the responsibility of the caller to eventually free the buffer
///* using sqlite3_free().
///*
///* If an error does occur within this function, the sqlite3expert object
///* is no longer useful for any purpose. At that point it is no longer
///* possible to add further SQL statements to the object or to re-attempt
///* the analysis. The sqlite3expert object must still be freed using a call
///* sqlite3_expert_destroy().
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expert_analyze(p: *mut Sqlite3expert,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut p_entry: *const IdxHashEntry = core::ptr::null();

    /// Do trigger processing to collect any extra IdxScan structures
    (rc = idx_process_triggers(p, pz_err_1));
    if rc == 0 {
        rc = idx_create_candidates(p);
    } else if rc == 5 | 3 << 8 {
        if !(pz_err_1).is_null() {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"Cannot find a unique index name to propose.".as_ptr()
                                    as *mut i8 as *const i8)
                    }
            };
        }
        return rc;
    }
    if rc == 0 { rc = idx_populate_stat1(p, pz_err_1); }
    {
        p_entry = unsafe { (*p).h_idx.p_first };
        '__b55: loop {
            if !(!(p_entry).is_null()) { break '__b55; }
            '__c55: loop {
                unsafe {
                    (*p).z_candidates =
                        unsafe {
                            idx_append_text(&mut rc, unsafe { (*p).z_candidates },
                                c"%s;%s%s\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_entry).z_val },
                                if !(unsafe { (*p_entry).z_val2 }).is_null() {
                                    c" -- stat1: ".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 },
                                unsafe { (*p_entry).z_val2 })
                        }
                };
                break '__c55;
            }
            p_entry = unsafe { (*p_entry).p_next };
        }
    }
    if rc == 0 { rc = idx_find_indexes(unsafe { &mut *p }, pz_err_1); }
    if rc == 0 { unsafe { (*p).b_run = 1 }; }
    return rc;
}

///* Return the total number of statements loaded using sqlite3_expert_sql().
///* The total number of SQL statements may be different from the total number
///* to calls to sqlite3_expert_sql().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expert_count(p: &Sqlite3expert) -> i32 {
    let mut n_ret: i32 = 0;
    if !((*p).p_statement).is_null() {
        n_ret = unsafe { (*(*p).p_statement).i_id } + 1;
    }
    return n_ret;
}

///* Return a component of the report.
///*
///* This function is called after sqlite3_expert_analyze() to extract the
///* results of the analysis. Each call to this function returns either a
///* NULL pointer or a pointer to a buffer containing a nul-terminated string.
///* The value passed as the third argument must be one of the EXPERT_REPORT_*
///* #define constants defined below.
///*
///* For some EXPERT_REPORT_* parameters, the buffer returned contains 
///* information relating to a specific SQL statement. In these cases that
///* SQL statement is identified by the value passed as the second argument.
///* SQL statements are numbered from 0 in the order in which they are parsed.
///* If an out-of-range value (less than zero or equal to or greater than the
///* value returned by sqlite3_expert_count()) is passed as the second argument
///* along with such an EXPERT_REPORT_* parameter, NULL is always returned.
///*
///* EXPERT_REPORT_SQL:
///*   Return the text of SQL statement iStmt.
///*
///* EXPERT_REPORT_INDEXES:
///*   Return a buffer containing the CREATE INDEX statements for all recommended
///*   indexes for statement iStmt. If there are no new recommeded indexes, NULL 
///*   is returned.
///*
///* EXPERT_REPORT_PLAN:
///*   Return a buffer containing the EXPLAIN QUERY PLAN output for SQL query
///*   iStmt after the proposed indexes have been added to the database schema.
///*
///* EXPERT_REPORT_CANDIDATES:
///*   Return a pointer to a buffer containing the CREATE INDEX statements 
///*   for all indexes that were tested (for all SQL statements). The iStmt
///*   parameter is ignored for EXPERT_REPORT_CANDIDATES calls.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expert_report(p: &Sqlite3expert, i_stmt_1: i32,
    e_report_1: i32) -> *const i8 {
    let mut z_ret: *const i8 = core::ptr::null();
    let mut p_stmt: *const IdxStatement = core::ptr::null();
    if (*p).b_run == 0 { return core::ptr::null(); }
    {
        p_stmt = (*p).p_statement;
        '__b56: loop {
            if !(!(p_stmt).is_null() && unsafe { (*p_stmt).i_id } != i_stmt_1)
                {
                break '__b56;
            }
            '__c56: loop { break '__c56; }
            p_stmt = unsafe { (*p_stmt).p_next };
        }
    }
    '__s57:
        {
        match e_report_1 {
            1 => {
                if !(p_stmt).is_null() {
                    z_ret = unsafe { (*p_stmt).z_sql } as *const i8;
                }
            }
            2 => {
                if !(p_stmt).is_null() {
                    z_ret = unsafe { (*p_stmt).z_idx } as *const i8;
                }
            }
            3 => {
                if !(p_stmt).is_null() {
                    z_ret = unsafe { (*p_stmt).z_eqp } as *const i8;
                }
            }
            4 => { z_ret = (*p).z_candidates as *const i8; }
            _ => {}
        }
    }
    return z_ret;
}

static mut expert_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 2,
        x_create: Some(expert_connect),
        x_connect: Some(expert_connect),
        x_best_index: Some(expert_best_index),
        x_disconnect: Some(expert_disconnect),
        x_destroy: Some(expert_disconnect),
        x_open: Some(expert_open),
        x_close: Some(expert_close),
        x_filter: Some(expert_filter),
        x_next: Some(expert_next),
        x_eof: Some(expert_eof),
        x_column: Some(expert_column),
        x_rowid: Some(expert_rowid),
        x_update: Some(expert_update),
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

static mut z_int: *const i8 =
    c"t592690916721053953805701627921227776".as_ptr() as *mut i8 as *const i8;

static mut z_drop: *const i8 =
    c"DROP TABLE t592690916721053953805701627921227776".as_ptr() as *mut i8 as
        *const i8;

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
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
