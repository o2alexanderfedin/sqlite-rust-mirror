#![allow(unused_imports, dead_code)]

mod fts5_h;
mod fts5_int_h;
mod sqlite3_h;
mod sqlite3ext_h;
use crate::fts5_h::{Fts5Api, Fts5Tokenizer};
use crate::fts5_int_h::{
    Fts5Buffer, Fts5Colset, Fts5Config, Fts5Expr, Fts5ExprNearset,
    Fts5ExprNode, Fts5ExprPhrase, Fts5Global, Fts5Hash, Fts5Index,
    Fts5IndexIter, Fts5Parse, Fts5PoslistPopulator, Fts5PoslistReader,
    Fts5PoslistWriter, Fts5Storage, Fts5Table, Fts5Termset, Fts5Token,
    Fts5TokenizerConfig,
};
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexConstraint, Sqlite3IndexInfo, Sqlite3Int64,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor, SqliteInt64,
};

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5VocabTable {
    base: Sqlite3Vtab,
    z_fts5_tbl: *mut i8,
    z_fts5_db: *mut i8,
    db: *mut Sqlite3,
    p_global: *mut Fts5Global,
    e_type: i32,
    b_busy: u32,
}

///* Translate a string containing an fts5vocab table type to an 
///* FTS5_VOCAB_XXX constant. If successful, set *peType to the output
///* value and return SQLITE_OK. Otherwise, set *pzErr to an error message
///* and return SQLITE_ERROR.
extern "C" fn fts5_vocab_table_type(z_type_1: *const i8,
    pz_err_1: &mut *mut i8, pe_type_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    let z_copy: *mut i8 =
        unsafe { sqlite3_fts5_strndup(&mut rc, z_type_1, -1) };
    if rc == 0 {
        unsafe { sqlite3_fts5_dequote(z_copy) };
        if unsafe {
                    sqlite3_stricmp(z_copy as *const i8,
                        c"col".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            *pe_type_1 = 0;
        } else if unsafe {
                    sqlite3_stricmp(z_copy as *const i8,
                        c"row".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            *pe_type_1 = 1;
        } else if unsafe {
                    sqlite3_stricmp(z_copy as *const i8,
                        c"instance".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            *pe_type_1 = 2;
        } else {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"fts5vocab: unknown table type: %Q".as_ptr()
                                as *mut i8 as *const i8, z_copy)
                };
            rc = 1;
        }
        unsafe { sqlite3_free(z_copy as *mut ()) };
    }
    return rc;
}

///* This function is the implementation of both the xConnect and xCreate
///* methods of the FTS3 virtual table.
///*
///* The argv[] array contains the following:
///*
///*   argv[0]   -> module name  ("fts5vocab")
///*   argv[1]   -> database name
///*   argv[2]   -> table name
///*
///* then:
///*
///*   argv[3]   -> name of fts5 table
///*   argv[4]   -> type of fts5vocab table
///*
///* or, for tables in the TEMP schema only.
///*
///*   argv[3]   -> name of fts5 tables database
///*   argv[4]   -> name of fts5 table
///*   argv[5]   -> type of fts5vocab table
#[allow(unused_doc_comments)]
extern "C" fn fts5_vocab_init_vtab(db: *mut Sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_v_tab_1: &mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let az_schema: [*const i8; 3] =
        [c"CREATE TABlE vocab(term, col, doc, cnt)".as_ptr() as *const i8,
                c"CREATE TABlE vocab(term, doc, cnt)".as_ptr() as *const i8,
                c"CREATE TABlE vocab(term, doc, col, offset)".as_ptr() as
                    *const i8];
    let mut p_ret: *mut Fts5VocabTable = core::ptr::null_mut();
    let mut rc: i32 = 0;
    /// Return code
    let mut b_db: i32 = 0;
    b_db =
        (argc == 6 &&
                    unsafe { strlen(unsafe { *argv.offset(1 as isize) }) } ==
                        4 as u64 &&
                unsafe {
                        memcmp(c"temp".as_ptr() as *mut i8 as *const (),
                            unsafe { *argv.offset(1 as isize) } as *const (), 4 as u64)
                    } == 0) as i32;
    if argc != 5 && b_db == 0 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"wrong number of vtable arguments".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        rc = 1;
    } else {
        let mut n_byte: i64 = 0 as i64;
        /// Bytes of space to allocate
        let z_db: *const i8 =
            if b_db != 0 {
                unsafe { *argv.offset(3 as isize) }
            } else { unsafe { *argv.offset(1 as isize) } };
        let z_tab: *const i8 =
            if b_db != 0 {
                unsafe { *argv.offset(4 as isize) }
            } else { unsafe { *argv.offset(3 as isize) } };
        let z_type: *const i8 =
            if b_db != 0 {
                unsafe { *argv.offset(5 as isize) }
            } else { unsafe { *argv.offset(4 as isize) } };
        let n_db: i64 = (unsafe { strlen(z_db) } + 1 as u64) as i64;
        let n_tab: i64 = (unsafe { strlen(z_tab) } + 1 as u64) as i64;
        let mut e_type: i32 = 0;
        rc =
            fts5_vocab_table_type(z_type, unsafe { &mut *pz_err_1 },
                &mut e_type);
        if rc == 0 {
            if !(e_type >= 0 &&
                                    e_type <
                                        (core::mem::size_of::<[*const i8; 3]>() as u64 /
                                                core::mem::size_of::<*const i8>() as u64) as i32) as i32 as
                        i64 != 0 {
                unsafe {
                    __assert_rtn(c"fts5VocabInitVtab".as_ptr() as *const i8,
                        c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 206,
                        c"eType>=0 && eType<ArraySize(azSchema)".as_ptr() as *mut i8
                            as *const i8)
                }
            } else { { let _ = 0; } };
            rc =
                unsafe {
                    sqlite3_declare_vtab(db, az_schema[e_type as usize])
                };
        }
        n_byte =
            (core::mem::size_of::<Fts5VocabTable>() as u64 + n_db as u64 +
                    n_tab as u64) as i64;
        p_ret =
            unsafe { sqlite3_fts5_malloc_zero(&mut rc, n_byte) } as
                *mut Fts5VocabTable;
        if !(p_ret).is_null() {
            unsafe { (*p_ret).p_global = p_aux_1 as *mut Fts5Global };
            unsafe { (*p_ret).e_type = e_type };
            unsafe { (*p_ret).db = db };
            unsafe {
                (*p_ret).z_fts5_tbl =
                    unsafe { &raw mut *p_ret.offset(1 as isize) } as *mut i8
            };
            unsafe {
                (*p_ret).z_fts5_db =
                    unsafe {
                        unsafe { (*p_ret).z_fts5_tbl.offset(n_tab as isize) }
                    }
            };
            unsafe {
                memcpy(unsafe { (*p_ret).z_fts5_tbl } as *mut (),
                    z_tab as *const (), n_tab as u64)
            };
            unsafe {
                memcpy(unsafe { (*p_ret).z_fts5_db } as *mut (),
                    z_db as *const (), n_db as u64)
            };
            unsafe { sqlite3_fts5_dequote(unsafe { (*p_ret).z_fts5_tbl }) };
            unsafe { sqlite3_fts5_dequote(unsafe { (*p_ret).z_fts5_db }) };
        }
    }
    *pp_v_tab_1 = p_ret as *mut Sqlite3Vtab;
    return rc;
}

extern "C" fn fts5_vocab_create_method(db: *mut Sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    return fts5_vocab_init_vtab(db, p_aux_1, argc, argv,
            unsafe { &mut *pp_vtab_1 }, pz_err_1);
}

///* The xConnect() and xCreate() methods for the virtual table. All the
///* work is done in function fts5VocabInitVtab().
extern "C" fn fts5_vocab_connect_method(db: *mut Sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    return fts5_vocab_init_vtab(db, p_aux_1, argc, argv,
            unsafe { &mut *pp_vtab_1 }, pz_err_1);
}

/// 
///* Implementation of the xBestIndex method.
///*
///* Only constraints of the form:
///*
///*     term <= ?
///*     term == ?
///*     term >= ?
///*
///* are interpreted. Less-than and less-than-or-equal are treated 
///* identically, as are greater-than and greater-than-or-equal.
extern "C" fn fts5_vocab_best_index_method(p_unused_1: *mut Sqlite3Vtab,
    p_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i: i32 = 0;
    let mut i_term_eq: i32 = -1;
    let mut i_term_ge: i32 = -1;
    let mut i_term_le: i32 = -1;
    let mut idx_num: i32 = unsafe { (*p_info_1).col_used } as i32;
    let mut n_arg: i32 = 0;
    { let _ = p_unused_1; };
    if !(unsafe { (*p_info_1).col_used } & 255 as Sqlite3Uint64 ==
                            unsafe { (*p_info_1).col_used }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5VocabBestIndexMethod".as_ptr() as *const i8,
                c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 280,
                c"(pInfo->colUsed & FTS5_VOCAB_COLUSED_MASK)==pInfo->colUsed".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        i = 0;
        '__b0: loop {
            if !(i < unsafe { (*p_info_1).n_constraint }) { break '__b0; }
            '__c0: loop {
                let p: *const Sqlite3IndexConstraint =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_info_1).a_constraint.offset(i as isize)
                                    }
                        } as *const Sqlite3IndexConstraint;
                if unsafe { (*p).usable } as i32 == 0 { break '__c0; }
                if unsafe { (*p).i_column } == 0 {
                    if unsafe { (*p).op } as i32 == 2 { i_term_eq = i; }
                    if unsafe { (*p).op } as i32 == 8 { i_term_le = i; }
                    if unsafe { (*p).op } as i32 == 16 { i_term_le = i; }
                    if unsafe { (*p).op } as i32 == 32 { i_term_ge = i; }
                    if unsafe { (*p).op } as i32 == 4 { i_term_ge = i; }
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i_term_eq >= 0 {
        idx_num |= 256;
        unsafe {
            (*unsafe {
                            (*p_info_1).a_constraint_usage.offset(i_term_eq as isize)
                        }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
        };
        unsafe { (*p_info_1).estimated_cost = 100 as f64 };
    } else {
        unsafe { (*p_info_1).estimated_cost = 1000000 as f64 };
        if i_term_ge >= 0 {
            idx_num |= 512;
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_term_ge as isize)
                            }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
            };
            unsafe {
                (*p_info_1).estimated_cost =
                    unsafe { (*p_info_1).estimated_cost } / 2 as f64
            };
        }
        if i_term_le >= 0 {
            idx_num |= 1024;
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_term_le as isize)
                            }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
            };
            unsafe {
                (*p_info_1).estimated_cost =
                    unsafe { (*p_info_1).estimated_cost } / 2 as f64
            };
        }
    }
    if unsafe { (*p_info_1).n_order_by } == 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 0 &&
            unsafe {
                        (*unsafe { (*p_info_1).a_order_by.offset(0 as isize) }).desc
                    } as i32 == 0 {
        unsafe { (*p_info_1).order_by_consumed = 1 };
    }
    unsafe { (*p_info_1).idx_num = idx_num };
    return 0;
}

///* The xDisconnect() virtual table method.
extern "C" fn fts5_vocab_disconnect_method(p_vtab_1: *mut Sqlite3Vtab)
    -> i32 {
    let p_tab: *mut Fts5VocabTable = p_vtab_1 as *mut Fts5VocabTable;
    unsafe { sqlite3_free(p_tab as *mut ()) };
    return 0;
}

///* The xDestroy() virtual table method.
extern "C" fn fts5_vocab_destroy_method(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *mut Fts5VocabTable = p_vtab_1 as *mut Fts5VocabTable;
    unsafe { sqlite3_free(p_tab as *mut ()) };
    return 0;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5VocabCursor {
    base: Sqlite3VtabCursor,
    p_stmt: *mut Sqlite3Stmt,
    p_fts5: *mut Fts5Table,
    b_eof: i32,
    p_iter: *mut Fts5IndexIter,
    p_struct: *mut (),
    n_le_term: i32,
    z_le_term: *mut i8,
    col_used: i32,
    i_col: i32,
    a_cnt: *mut i64,
    a_doc: *mut i64,
    rowid: i64,
    term: Fts5Buffer,
    i_inst_pos: i64,
    i_inst_off: i32,
}

///* Implementation of xOpen method.
extern "C" fn fts5_vocab_open_method(p_v_tab_1: *mut Sqlite3Vtab,
    pp_csr_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p_tab: *mut Fts5VocabTable = p_v_tab_1 as *mut Fts5VocabTable;
    let mut p_fts5: *mut Fts5Table = core::ptr::null_mut();
    let mut p_csr: *mut Fts5VocabCursor = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    if unsafe { (*p_tab).b_busy } != 0 {
        unsafe {
            (*p_v_tab_1).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"recursive definition for %s.%s".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p_tab).z_fts5_db },
                        unsafe { (*p_tab).z_fts5_tbl })
                }
        };
        return 1;
    }
    z_sql =
        unsafe {
            sqlite3_fts5_mprintf(&mut rc,
                c"SELECT t.%Q FROM %Q.%Q AS t WHERE t.%Q MATCH \'*id\'".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p_tab).z_fts5_tbl },
                unsafe { (*p_tab).z_fts5_db }, unsafe { (*p_tab).z_fts5_tbl },
                unsafe { (*p_tab).z_fts5_tbl })
        };
    if !(z_sql).is_null() {
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p_tab).db }, z_sql as *const i8,
                    -1, &mut p_stmt, core::ptr::null_mut())
            };
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if !(rc == 0 || p_stmt == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5VocabOpenMethod".as_ptr() as *const i8,
                c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 356,
                c"rc==SQLITE_OK || pStmt==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if rc == 1 { rc = 0; }
    unsafe { (*p_tab).b_busy = 1 as u32 };
    if !(p_stmt).is_null() && unsafe { sqlite3_step(p_stmt) } == 100 {
        let i_id: i64 = unsafe { sqlite3_column_int64(p_stmt, 0) };
        p_fts5 =
            unsafe {
                sqlite3_fts5_table_from_csrid(unsafe { (*p_tab).p_global },
                    i_id)
            };
    }
    unsafe { (*p_tab).b_busy = 0 as u32 };
    if rc == 0 {
        if p_fts5 == core::ptr::null_mut() {
            rc = unsafe { sqlite3_finalize(p_stmt) };
            p_stmt = core::ptr::null_mut();
            if rc == 0 {
                unsafe {
                    (*p_v_tab_1).z_err_msg =
                        unsafe {
                            sqlite3_mprintf(c"no such fts5 table: %s.%s".as_ptr() as
                                        *mut i8 as *const i8, unsafe { (*p_tab).z_fts5_db },
                                unsafe { (*p_tab).z_fts5_tbl })
                        }
                };
                rc = 1;
            }
        } else { rc = unsafe { sqlite3_fts5_flush_to_disk(p_fts5) }; }
    }
    if rc == 0 {
        let n_byte: i64 =
            (unsafe { (*unsafe { (*p_fts5).p_config }).n_col } as u64 *
                            core::mem::size_of::<i64>() as u64 * 2 as u64 +
                    core::mem::size_of::<Fts5VocabCursor>() as u64) as i64;
        p_csr =
            unsafe { sqlite3_fts5_malloc_zero(&mut rc, n_byte) } as
                *mut Fts5VocabCursor;
    }
    if !(p_csr).is_null() {
        unsafe { (*p_csr).p_fts5 = p_fts5 };
        unsafe { (*p_csr).p_stmt = p_stmt };
        unsafe {
            (*p_csr).a_cnt =
                unsafe { &raw mut *p_csr.offset(1 as isize) } as *mut i64
        };
        unsafe {
            (*p_csr).a_doc =
                unsafe {
                    unsafe {
                        (*p_csr).a_cnt.offset(unsafe {
                                    (*unsafe { (*p_fts5).p_config }).n_col
                                } as isize)
                    }
                }
        };
    } else { unsafe { sqlite3_finalize(p_stmt) }; }
    unsafe { *pp_csr_1 = p_csr as *mut Sqlite3VtabCursor };
    return rc;
}

///* Restore cursor pCsr to the state it was in immediately after being
///* created by the xOpen() method.
extern "C" fn fts5_vocab_reset_cursor(p_csr_1: &mut Fts5VocabCursor) -> () {
    let n_col: i32 =
        unsafe { (*unsafe { (*(*p_csr_1).p_fts5).p_config }).n_col };
    (*p_csr_1).rowid = 0 as i64;
    unsafe { sqlite3_fts5_iter_close((*p_csr_1).p_iter) };
    unsafe { sqlite3_fts5_structure_release((*p_csr_1).p_struct) };
    (*p_csr_1).p_struct = core::ptr::null_mut();
    (*p_csr_1).p_iter = core::ptr::null_mut();
    unsafe { sqlite3_free((*p_csr_1).z_le_term as *mut ()) };
    (*p_csr_1).n_le_term = -1;
    (*p_csr_1).z_le_term = core::ptr::null_mut();
    (*p_csr_1).b_eof = 0;
    (*p_csr_1).i_col = 0;
    (*p_csr_1).i_inst_pos = 0 as i64;
    (*p_csr_1).i_inst_off = 0;
    (*p_csr_1).col_used = 0;
    unsafe {
        memset((*p_csr_1).a_cnt as *mut (), 0,
            core::mem::size_of::<i64>() as u64 * n_col as u64)
    };
    unsafe {
        memset((*p_csr_1).a_doc as *mut (), 0,
            core::mem::size_of::<i64>() as u64 * n_col as u64)
    };
}

///* Close the cursor.  For additional information see the documentation
///* on the xClose method of the virtual table interface.
extern "C" fn fts5_vocab_close_method(p_cursor_1: *mut Sqlite3VtabCursor)
    -> i32 {
    let p_csr: *mut Fts5VocabCursor = p_cursor_1 as *mut Fts5VocabCursor;
    fts5_vocab_reset_cursor(unsafe { &mut *p_csr });
    unsafe { sqlite3_fts5_buffer_free(unsafe { &mut (*p_csr).term }) };
    unsafe { sqlite3_finalize(unsafe { (*p_csr).p_stmt }) };
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}

extern "C" fn fts5_vocab_instance_new_term(p_csr_1: &mut Fts5VocabCursor)
    -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*(*p_csr_1).p_iter).b_eof } != 0 {
        (*p_csr_1).b_eof = 1;
    } else {
        let mut z_term: *const i8 = core::ptr::null();
        let mut n_term: i32 = 0;
        z_term =
            unsafe { sqlite3_fts5_iter_term((*p_csr_1).p_iter, &mut n_term) };
        if (*p_csr_1).n_le_term >= 0 {
            let n_cmp: i32 =
                if n_term < (*p_csr_1).n_le_term {
                    n_term
                } else { (*p_csr_1).n_le_term };
            let b_cmp: i32 =
                unsafe {
                    memcmp((*p_csr_1).z_le_term as *const (),
                        z_term as *const (), n_cmp as u64)
                };
            if b_cmp < 0 || b_cmp == 0 && (*p_csr_1).n_le_term < n_term {
                (*p_csr_1).b_eof = 1;
            }
        }
        unsafe {
            sqlite3_fts5_buffer_set(&mut rc, &mut (*p_csr_1).term, n_term,
                z_term as *const u8)
        };
    }
    return rc;
}

extern "C" fn fts5_vocab_instance_next(p_csr_1: *mut Fts5VocabCursor) -> i32 {
    let e_detail: i32 =
        unsafe {
            (*unsafe { (*unsafe { (*p_csr_1).p_fts5 }).p_config }).e_detail
        };
    let mut rc: i32 = 0;
    let p_iter: *const Fts5IndexIter =
        unsafe { (*p_csr_1).p_iter } as *const Fts5IndexIter;
    let pp: *mut i64 = unsafe { &mut (*p_csr_1).i_inst_pos };
    let po: *mut i32 = unsafe { &mut (*p_csr_1).i_inst_off };
    if !(unsafe { (*p_iter).b_eof } as i32 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5VocabInstanceNext".as_ptr() as *const i8,
                c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 464,
                c"sqlite3Fts5IterEof(pIter)==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p_csr_1).b_eof } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5VocabInstanceNext".as_ptr() as *const i8,
                c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 465,
                c"pCsr->bEof==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    while e_detail == 1 ||
            unsafe {
                    sqlite3_fts5_poslist_next64(unsafe { (*p_iter).p_data },
                        unsafe { (*p_iter).n_data }, po, pp)
                } != 0 {
        unsafe { (*p_csr_1).i_inst_pos = 0 as i64 };
        unsafe { (*p_csr_1).i_inst_off = 0 };
        rc =
            unsafe {
                sqlite3_fts5_iter_next_scan(unsafe { (*p_csr_1).p_iter })
            };
        if rc == 0 {
            rc = fts5_vocab_instance_new_term(unsafe { &mut *p_csr_1 });
            if unsafe { (*p_csr_1).b_eof } != 0 || e_detail == 1 { break; }
        }
        if rc != 0 { unsafe { (*p_csr_1).b_eof = 1 }; break; }
    }
    return rc;
}

///* Advance the cursor to the next row in the table.
#[allow(unused_doc_comments)]
extern "C" fn fts5_vocab_next_method(p_cursor_1: *mut Sqlite3VtabCursor)
    -> i32 {
    let p_csr: *mut Fts5VocabCursor = p_cursor_1 as *mut Fts5VocabCursor;
    let p_tab: *const Fts5VocabTable =
        unsafe { (*p_cursor_1).p_vtab } as *mut Fts5VocabTable as
            *const Fts5VocabTable;
    let n_col: i32 =
        unsafe { (*unsafe { (*unsafe { (*p_csr).p_fts5 }).p_config }).n_col };
    let mut rc: i32 = 0;
    rc =
        unsafe {
            sqlite3_fts5_structure_test(unsafe {
                    (*unsafe { (*p_csr).p_fts5 }).p_index
                }, unsafe { (*p_csr).p_struct })
        };
    if rc != 0 { return rc; }
    {
        let __p = unsafe { &mut (*p_csr).rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    if unsafe { (*p_tab).e_type } == 2 {
        return fts5_vocab_instance_next(p_csr);
    }
    if unsafe { (*p_tab).e_type } == 0 {
        {
            {
                let __p = unsafe { &mut (*p_csr).i_col };
                let __t = *__p;
                *__p += 1;
                __t
            };
            '__b2: loop {
                if !(unsafe { (*p_csr).i_col } < n_col) { break '__b2; }
                '__c2: loop {
                    if unsafe {
                                *unsafe {
                                        (*p_csr).a_doc.offset(unsafe { (*p_csr).i_col } as isize)
                                    }
                            } != 0 {
                        break '__b2;
                    }
                    break '__c2;
                }
                {
                    let __p = unsafe { &mut (*p_csr).i_col };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
        }
    }
    if unsafe { (*p_tab).e_type } != 0 || unsafe { (*p_csr).i_col } >= n_col {
        if unsafe { (*unsafe { (*p_csr).p_iter }).b_eof } != 0 {
            unsafe { (*p_csr).b_eof = 1 };
        } else {
            let mut z_term: *const i8 = core::ptr::null();
            let mut n_term: i32 = 0;
            z_term =
                unsafe {
                    sqlite3_fts5_iter_term(unsafe { (*p_csr).p_iter },
                        &mut n_term)
                };
            if !(n_term >= 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"fts5VocabNextMethod".as_ptr() as *const i8,
                        c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 517,
                        c"nTerm>=0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if unsafe { (*p_csr).n_le_term } >= 0 {
                let n_cmp: i32 =
                    if n_term < unsafe { (*p_csr).n_le_term } {
                        n_term
                    } else { unsafe { (*p_csr).n_le_term } };
                let b_cmp: i32 =
                    unsafe {
                        memcmp(unsafe { (*p_csr).z_le_term } as *const (),
                            z_term as *const (), n_cmp as u64)
                    };
                if b_cmp < 0 ||
                        b_cmp == 0 && unsafe { (*p_csr).n_le_term } < n_term {
                    unsafe { (*p_csr).b_eof = 1 };
                    return 0;
                }
            }
            unsafe {
                sqlite3_fts5_buffer_set(&mut rc,
                    unsafe { &mut (*p_csr).term }, n_term, z_term as *const u8)
            };
            unsafe {
                memset(unsafe { (*p_csr).a_cnt } as *mut (), 0,
                    n_col as u64 * core::mem::size_of::<i64>() as u64)
            };
            unsafe {
                memset(unsafe { (*p_csr).a_doc } as *mut (), 0,
                    n_col as u64 * core::mem::size_of::<i64>() as u64)
            };
            unsafe { (*p_csr).i_col = 0 };
            if !(unsafe { (*p_tab).e_type } == 0 ||
                                    unsafe { (*p_tab).e_type } == 1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"fts5VocabNextMethod".as_ptr() as *const i8,
                        c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 532,
                        c"pTab->eType==FTS5_VOCAB_COL || pTab->eType==FTS5_VOCAB_ROW".as_ptr()
                                as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            while rc == 0 {
                let e_detail: i32 =
                    unsafe {
                        (*unsafe {
                                        (*unsafe { (*p_csr).p_fts5 }).p_config
                                    }).e_detail
                    };
                let mut p_pos: *const u8 = core::ptr::null();
                let mut n_pos: i32 = 0;
                /// Position list
                let mut i_pos: i64 = 0 as i64;
                /// 64-bit position read from poslist
                let mut i_off: i32 = 0;

                /// Current offset within position list
                (p_pos = unsafe { (*unsafe { (*p_csr).p_iter }).p_data });
                n_pos = unsafe { (*unsafe { (*p_csr).p_iter }).n_data };
                '__s4:
                    {
                    match unsafe { (*p_tab).e_type } {
                        1 => {
                            if e_detail == 0 && unsafe { (*p_csr).col_used } & 4 != 0 {
                                while i_pos < n_pos as i64 {
                                    let mut ii: u32 = 0 as u32;
                                    {
                                        ii =
                                            unsafe {
                                                    *p_pos.offset({
                                                                    let __p = &mut i_pos;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize)
                                                } as u32;
                                        if ii & 128 as u32 != 0 {
                                            { let __p = &mut i_pos; let __t = *__p; *__p -= 1; __t };
                                            i_pos +=
                                                unsafe {
                                                        sqlite3_fts5_get_varint32(unsafe {
                                                                &*p_pos.offset(i_pos as isize)
                                                            }, &raw mut ii as *mut u32)
                                                    } as i64;
                                        }
                                    }
                                    if ii == 1 as u32 {
                                        {
                                            ii =
                                                unsafe {
                                                        *p_pos.offset({
                                                                        let __p = &mut i_pos;
                                                                        let __t = *__p;
                                                                        *__p += 1;
                                                                        __t
                                                                    } as isize)
                                                    } as u32;
                                            if ii & 128 as u32 != 0 {
                                                { let __p = &mut i_pos; let __t = *__p; *__p -= 1; __t };
                                                i_pos +=
                                                    unsafe {
                                                            sqlite3_fts5_get_varint32(unsafe {
                                                                    &*p_pos.offset(i_pos as isize)
                                                                }, &raw mut ii as *mut u32)
                                                        } as i64;
                                            }
                                        }
                                    } else {

                                        /// An instance - increment pCsr->aCnt[]
                                        {
                                            let __p =
                                                unsafe {
                                                    &mut *unsafe { (*p_csr).a_cnt.offset(0 as isize) }
                                                };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                }
                            }
                            {
                                let __p =
                                    unsafe {
                                        &mut *unsafe { (*p_csr).a_doc.offset(0 as isize) }
                                    };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        0 => {
                            if e_detail == 0 {
                                let mut i_col: i32 = -1;
                                while 0 ==
                                        unsafe {
                                            sqlite3_fts5_poslist_next64(p_pos, n_pos, &mut i_off,
                                                &mut i_pos)
                                        } {
                                    let mut ii: i32 = (i_pos >> 32 & 2147483647 as i64) as i32;
                                    if i_col != ii {
                                        if ii >= n_col { rc = 11 | 1 << 8; break; }
                                        {
                                            let __p =
                                                unsafe {
                                                    &mut *unsafe { (*p_csr).a_doc.offset(ii as isize) }
                                                };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                        i_col = ii;
                                    }
                                    {
                                        let __p =
                                            unsafe {
                                                &mut *unsafe { (*p_csr).a_cnt.offset(ii as isize) }
                                            };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                }
                            } else if e_detail == 2 {
                                while 0 ==
                                        unsafe {
                                            sqlite3_fts5_poslist_next64(p_pos, n_pos, &mut i_off,
                                                &mut i_pos)
                                        } {
                                    if !(i_pos >= 0 as i64 && i_pos < n_col as i64) as i32 as
                                                i64 != 0 {
                                        unsafe {
                                            __assert_rtn(c"fts5VocabNextMethod".as_ptr() as *const i8,
                                                c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 579,
                                                c"iPos>=0 && iPos<nCol".as_ptr() as *mut i8 as *const i8)
                                        }
                                    } else { { let _ = 0; } };
                                    if i_pos >= n_col as i64 { rc = 11 | 1 << 8; break; }
                                    {
                                        let __p =
                                            unsafe {
                                                &mut *unsafe { (*p_csr).a_doc.offset(i_pos as isize) }
                                            };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                }
                            } else {
                                if !(e_detail == 1) as i32 as i64 != 0 {
                                    unsafe {
                                        __assert_rtn(c"fts5VocabNextMethod".as_ptr() as *const i8,
                                            c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 587,
                                            c"eDetail==FTS5_DETAIL_NONE".as_ptr() as *mut i8 as
                                                *const i8)
                                    }
                                } else { { let _ = 0; } };
                                {
                                    let __p =
                                        unsafe {
                                            &mut *unsafe { (*p_csr).a_doc.offset(0 as isize) }
                                        };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        _ => {
                            if !(unsafe { (*p_tab).e_type } == 2) as i32 as i64 != 0 {
                                unsafe {
                                    __assert_rtn(c"fts5VocabNextMethod".as_ptr() as *const i8,
                                        c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 593,
                                        c"pTab->eType==FTS5_VOCAB_INSTANCE".as_ptr() as *mut i8 as
                                            *const i8)
                                }
                            } else { { let _ = 0; } };
                        }
                    }
                }
                if rc == 0 {
                    rc =
                        unsafe {
                            sqlite3_fts5_iter_next_scan(unsafe { (*p_csr).p_iter })
                        };
                }
                if unsafe { (*p_tab).e_type } == 2 { break; }
                if rc == 0 {
                    z_term =
                        unsafe {
                            sqlite3_fts5_iter_term(unsafe { (*p_csr).p_iter },
                                &mut n_term)
                        };
                    if n_term != unsafe { (*p_csr).term.n } ||
                            n_term > 0 &&
                                unsafe {
                                        memcmp(z_term as *const (),
                                            unsafe { (*p_csr).term.p } as *const (), n_term as u64)
                                    } != 0 {
                        break;
                    }
                    if unsafe { (*unsafe { (*p_csr).p_iter }).b_eof } != 0 {
                        break;
                    }
                }
            }
        }
    }
    if rc == 0 && unsafe { (*p_csr).b_eof } == 0 &&
            unsafe { (*p_tab).e_type } == 0 {
        {
            '__b8: loop {
                if !(unsafe { (*p_csr).i_col } < n_col &&
                                unsafe {
                                        *unsafe {
                                                (*p_csr).a_doc.offset(unsafe { (*p_csr).i_col } as isize)
                                            }
                                    } == 0 as i64) {
                    break '__b8;
                }
                '__c8: loop { break '__c8; }
                {
                    let __p = unsafe { &mut (*p_csr).i_col };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
        }
        if unsafe { (*p_csr).i_col } == n_col { rc = 11 | 1 << 8; }
    }
    return rc;
}

///* This is the xFilter implementation for the virtual table.
extern "C" fn fts5_vocab_filter_method(p_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, z_unused_1: *const i8, n_unused_1: i32,
    ap_val_1: *mut *mut Sqlite3Value) -> i32 {
    let p_tab: *const Fts5VocabTable =
        unsafe { (*p_cursor_1).p_vtab } as *mut Fts5VocabTable as
            *const Fts5VocabTable;
    let p_csr: *mut Fts5VocabCursor = p_cursor_1 as *mut Fts5VocabCursor;
    let e_type: i32 = unsafe { (*p_tab).e_type };
    let mut rc: i32 = 0;
    let mut i_val: i32 = 0;
    let mut f: i32 = 8;
    let mut z_term: *const i8 = core::ptr::null();
    let mut n_term: i32 = 0;
    let mut p_eq: *mut Sqlite3Value = core::ptr::null_mut();
    let mut p_ge: *mut Sqlite3Value = core::ptr::null_mut();
    let mut p_le: *mut Sqlite3Value = core::ptr::null_mut();
    { { let _ = z_unused_1; }; { let _ = n_unused_1; } };
    fts5_vocab_reset_cursor(unsafe { &mut *p_csr });
    if idx_num_1 & 256 != 0 {
        p_eq =
            unsafe {
                *ap_val_1.offset({
                                let __p = &mut i_val;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize)
            };
    }
    if idx_num_1 & 512 != 0 {
        p_ge =
            unsafe {
                *ap_val_1.offset({
                                let __p = &mut i_val;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize)
            };
    }
    if idx_num_1 & 1024 != 0 {
        p_le =
            unsafe {
                *ap_val_1.offset({
                                let __p = &mut i_val;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize)
            };
    }
    unsafe { (*p_csr).col_used = idx_num_1 & 255 };
    if !(p_eq).is_null() {
        z_term = unsafe { sqlite3_value_text(p_eq) } as *const i8;
        n_term = unsafe { sqlite3_value_bytes(p_eq) };
        f = 128;
    } else {
        if !(p_ge).is_null() {
            z_term = unsafe { sqlite3_value_text(p_ge) } as *const i8;
            n_term = unsafe { sqlite3_value_bytes(p_ge) };
        }
        if !(p_le).is_null() {
            let mut z_copy: *const i8 =
                unsafe { sqlite3_value_text(p_le) } as *const i8;
            if z_copy == core::ptr::null() {
                z_copy = c"".as_ptr() as *mut i8 as *const i8;
            }
            unsafe {
                (*p_csr).n_le_term = unsafe { sqlite3_value_bytes(p_le) }
            };
            unsafe {
                (*p_csr).z_le_term =
                    unsafe {
                            sqlite3_malloc64((unsafe { (*p_csr).n_le_term } as i64 +
                                        1 as i64) as Sqlite3Uint64)
                        } as *mut i8
            };
            if unsafe { (*p_csr).z_le_term } == core::ptr::null_mut() {
                rc = 7;
            } else {
                unsafe {
                    memcpy(unsafe { (*p_csr).z_le_term } as *mut (),
                        z_copy as *const (),
                        (unsafe { (*p_csr).n_le_term } + 1) as u64)
                };
            }
        }
    }
    if rc == 0 {
        let p_index: *mut Fts5Index =
            unsafe { (*unsafe { (*p_csr).p_fts5 }).p_index };
        rc =
            unsafe {
                sqlite3_fts5_index_query(p_index, z_term, n_term, f,
                    core::ptr::null_mut(), unsafe { &mut (*p_csr).p_iter })
            };
        if rc == 0 {
            unsafe {
                (*p_csr).p_struct =
                    unsafe { sqlite3_fts5_structure_ref(p_index) }
            };
        }
    }
    if rc == 0 && e_type == 2 {
        rc = fts5_vocab_instance_new_term(unsafe { &mut *p_csr });
    }
    if rc == 0 && (unsafe { (*p_csr).b_eof } == 0) as i32 != 0 &&
            (e_type != 2 ||
                unsafe {
                        (*unsafe {
                                        (*unsafe { (*p_csr).p_fts5 }).p_config
                                    }).e_detail
                    } != 1) {
        rc = fts5_vocab_next_method(p_cursor_1);
    }
    return rc;
}

/// 
///* This is the xEof method of the virtual table. SQLite calls this 
///* routine to find out if it has reached the end of a result set.
extern "C" fn fts5_vocab_eof_method(p_cursor_1: *mut Sqlite3VtabCursor)
    -> i32 {
    let p_csr: *const Fts5VocabCursor =
        p_cursor_1 as *mut Fts5VocabCursor as *const Fts5VocabCursor;
    return unsafe { (*p_csr).b_eof };
}

extern "C" fn fts5_vocab_column_method(p_cursor_1: *mut Sqlite3VtabCursor,
    p_ctx_1: *mut Sqlite3Context, i_col_1: i32) -> i32 {
    let p_csr: *const Fts5VocabCursor =
        p_cursor_1 as *mut Fts5VocabCursor as *const Fts5VocabCursor;
    let e_detail: i32 =
        unsafe {
            (*unsafe { (*unsafe { (*p_csr).p_fts5 }).p_config }).e_detail
        };
    let e_type: i32 =
        unsafe {
            (*(unsafe { (*p_cursor_1).p_vtab } as *mut Fts5VocabTable)).e_type
        };
    let mut i_val: i64 = 0 as i64;
    if i_col_1 == 0 {
        unsafe {
            sqlite3_result_text(p_ctx_1,
                unsafe { (*p_csr).term.p } as *const i8,
                unsafe { (*p_csr).term.n },
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    } else if e_type == 0 {
        if !(i_col_1 == 1 || i_col_1 == 2 || i_col_1 == 3) as i32 as i64 != 0
            {
            unsafe {
                __assert_rtn(c"fts5VocabColumnMethod".as_ptr() as *const i8,
                    c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 722,
                    c"iCol==1 || iCol==2 || iCol==3".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if i_col_1 == 1 {
            if e_detail != 1 {
                let z: *const i8 =
                    unsafe {
                            *unsafe {
                                    (*unsafe {
                                                        (*unsafe { (*p_csr).p_fts5 }).p_config
                                                    }).az_col.offset(unsafe { (*p_csr).i_col } as isize)
                                }
                        } as *const i8;
                unsafe { sqlite3_result_text(p_ctx_1, z, -1, None) };
            }
        } else if i_col_1 == 2 {
            i_val =
                unsafe {
                    *unsafe {
                            (*p_csr).a_doc.offset(unsafe { (*p_csr).i_col } as isize)
                        }
                };
        } else {
            i_val =
                unsafe {
                    *unsafe {
                            (*p_csr).a_cnt.offset(unsafe { (*p_csr).i_col } as isize)
                        }
                };
        }
    } else if e_type == 1 {
        if !(i_col_1 == 1 || i_col_1 == 2) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5VocabColumnMethod".as_ptr() as *const i8,
                    c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 734,
                    c"iCol==1 || iCol==2".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if i_col_1 == 1 {
            i_val = unsafe { *unsafe { (*p_csr).a_doc.offset(0 as isize) } };
        } else {
            i_val = unsafe { *unsafe { (*p_csr).a_cnt.offset(0 as isize) } };
        }
    } else {
        if !(e_type == 2) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5VocabColumnMethod".as_ptr() as *const i8,
                    c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 741,
                    c"eType==FTS5_VOCAB_INSTANCE".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        '__s9:
            {
            match i_col_1 {
                1 => {
                    unsafe {
                        sqlite3_result_int64(p_ctx_1,
                            unsafe { (*unsafe { (*p_csr).p_iter }).i_rowid })
                    };
                }
                2 => {
                    {
                        let mut ii: i32 = -1;
                        if e_detail == 0 {
                            ii =
                                (unsafe { (*p_csr).i_inst_pos } >> 32 & 2147483647 as i64)
                                    as i32;
                        } else if e_detail == 2 {
                            ii = unsafe { (*p_csr).i_inst_pos } as i32;
                        }
                        if ii >= 0 &&
                                ii <
                                    unsafe {
                                        (*unsafe { (*unsafe { (*p_csr).p_fts5 }).p_config }).n_col
                                    } {
                            let z: *const i8 =
                                unsafe {
                                        *unsafe {
                                                (*unsafe {
                                                                    (*unsafe { (*p_csr).p_fts5 }).p_config
                                                                }).az_col.offset(ii as isize)
                                            }
                                    } as *const i8;
                            unsafe { sqlite3_result_text(p_ctx_1, z, -1, None) };
                        }
                        break '__s9;
                    }
                    {
                        if !(i_col_1 == 3) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fts5VocabColumnMethod".as_ptr() as *const i8,
                                    c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 760,
                                    c"iCol==3".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        if e_detail == 0 {
                            let mut ii: i32 =
                                (unsafe { (*p_csr).i_inst_pos } & 2147483647 as i64) as i32;
                            unsafe { sqlite3_result_int(p_ctx_1, ii) };
                        }
                        break '__s9;
                    }
                }
                _ => {
                    {
                        if !(i_col_1 == 3) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fts5VocabColumnMethod".as_ptr() as *const i8,
                                    c"fts5_vocab.c".as_ptr() as *mut i8 as *const i8, 760,
                                    c"iCol==3".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        if e_detail == 0 {
                            let mut ii: i32 =
                                (unsafe { (*p_csr).i_inst_pos } & 2147483647 as i64) as i32;
                            unsafe { sqlite3_result_int(p_ctx_1, ii) };
                        }
                        break '__s9;
                    }
                }
            }
        }
    }
    if i_val > 0 as i64 { unsafe { sqlite3_result_int64(p_ctx_1, i_val) }; }
    return 0;
}

/// 
///* This is the xRowid method. The SQLite core calls this routine to
///* retrieve the rowid for the current row of the result set. The
///* rowid should be written to *pRowid.
extern "C" fn fts5_vocab_rowid_method(p_cursor_1: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_csr: *const Fts5VocabCursor =
        p_cursor_1 as *mut Fts5VocabCursor as *const Fts5VocabCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_csr).rowid } };
    return 0;
}

///***********************************************************************
///* Interface to code in fts5_vocab.c.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_fts5_vocab_init(p_global: *mut Fts5Global,
    db: *mut Sqlite3) -> i32 {
    /// iVersion
    /// xCreate
    /// xConnect
    /// xBestIndex
    /// xDisconnect
    /// xDestroy
    /// xOpen
    /// xClose
    /// xFilter
    /// xNext
    /// xEof
    /// xColumn
    /// xRowid
    /// xUpdate
    /// xBegin
    /// xSync
    /// xCommit
    /// xRollback
    /// xFindFunction
    /// xRename
    /// xSavepoint
    /// xRelease
    /// xRollbackTo
    /// xShadowName
    /// xIntegrity
    let p: *mut () = p_global as *mut ();
    return unsafe {
            sqlite3_create_module_v2(db,
                c"fts5vocab".as_ptr() as *mut i8 as *const i8, &fts5_vocab, p,
                None)
        };
}

static fts5_vocab: Sqlite3Module =
    Sqlite3Module {
        i_version: 2,
        x_create: Some(fts5_vocab_create_method),
        x_connect: Some(fts5_vocab_connect_method),
        x_best_index: Some(fts5_vocab_best_index_method),
        x_disconnect: Some(fts5_vocab_disconnect_method),
        x_destroy: Some(fts5_vocab_destroy_method),
        x_open: Some(fts5_vocab_open_method),
        x_close: Some(fts5_vocab_close_method),
        x_filter: Some(fts5_vocab_filter_method),
        x_next: Some(fts5_vocab_next_method),
        x_eof: Some(fts5_vocab_eof_method),
        x_column: Some(fts5_vocab_column_method),
        x_rowid: Some(fts5_vocab_rowid_method),
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
    fn sqlite3_fts5_storage_open(_: *mut Fts5Config, _: *mut Fts5Index,
    _: i32, _: *mut *mut Fts5Storage, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_storage_close(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_rename(_: *mut Fts5Storage, z_name_1: *const i8)
    -> i32;
    fn sqlite3_fts5_drop_all(_: *mut Fts5Config)
    -> i32;
    fn sqlite3_fts5_create_table(_: *mut Fts5Config, _: *const i8,
    _: *const i8, _: i32, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_storage_delete(p: *mut Fts5Storage, _: i64,
    _: *mut *mut Sqlite3Value, _: i32)
    -> i32;
    fn sqlite3_fts5_storage_content_insert(p: *mut Fts5Storage, _: i32,
    _: *mut *mut Sqlite3Value, _: *mut i64)
    -> i32;
    fn sqlite3_fts5_storage_index_insert(p: *mut Fts5Storage,
    _: *mut *mut Sqlite3Value, _: i64)
    -> i32;
    fn sqlite3_fts5_storage_integrity(p: *mut Fts5Storage, i_arg_1: i32)
    -> i32;
    fn sqlite3_fts5_storage_stmt(p: *mut Fts5Storage, e_stmt_1: i32,
    _: *mut *mut Sqlite3Stmt, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_storage_stmt_release(p: *mut Fts5Storage, e_stmt_1: i32,
    _: *mut Sqlite3Stmt)
    -> ();
    fn sqlite3_fts5_storage_docsize(p: *mut Fts5Storage, i_rowid_1: i64,
    a_col_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_storage_size(p: *mut Fts5Storage, i_col_1: i32,
    pn_avg_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_storage_row_count(p: *mut Fts5Storage, pn_row_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_storage_sync(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_rollback(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_config_value(p: *mut Fts5Storage, _: *const i8,
    _: *mut Sqlite3Value, _: i32)
    -> i32;
    fn sqlite3_fts5_storage_delete_all(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_rebuild(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_optimize(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_merge(p: *mut Fts5Storage, n_merge_1: i32)
    -> i32;
    fn sqlite3_fts5_storage_reset(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_release_delete_row(_: *mut Fts5Storage)
    -> ();
    fn sqlite3_fts5_storage_find_delete_row(p: *mut Fts5Storage, i_del_1: i64)
    -> i32;
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
    fn strlen(__s: *const i8)
    -> u64;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
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
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
