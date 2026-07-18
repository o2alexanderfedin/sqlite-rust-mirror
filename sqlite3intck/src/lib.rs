#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3Intck {
    db: *mut Sqlite3,
    z_db: *const i8,
    z_obj: *mut i8,
    p_check: *mut Sqlite3Stmt,
    z_key: *mut i8,
    n_key_val: i32,
    z_message: *mut i8,
    b_corrupt_schema: i32,
    rc: i32,
    z_err: *mut i8,
    z_test_sql: *mut i8,
}

extern "C" fn intck_get_token(z: *const i8) -> i32 {
    let c: i8 = unsafe { *z.offset(0 as isize) } as i8;
    let mut i_ret: i32 = 1;
    if c as i32 == '\'' as i32 || c as i32 == '\"' as i32 ||
            c as i32 == '`' as i32 {
        while unsafe { *z.offset(i_ret as isize) } != 0 {
            if unsafe { *z.offset(i_ret as isize) } as i32 == c as i32 {
                { let __p = &mut i_ret; let __t = *__p; *__p += 1; __t };
                if unsafe { *z.offset(i_ret as isize) } as i32 != c as i32 {
                    break;
                }
            }
            { let __p = &mut i_ret; let __t = *__p; *__p += 1; __t };
        }
    } else if c as i32 == '[' as i32 {
        while unsafe { *z.offset(i_ret as isize) } != 0 &&
                unsafe {
                            *z.offset({
                                            let __p = &mut i_ret;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        } as i32 != ']' as i32 {}
    } else if c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32 ||
            c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32 {
        while unsafe { *z.offset(i_ret as isize) } as i32 >= 'A' as i32 &&
                    unsafe { *z.offset(i_ret as isize) } as i32 <= 'Z' as i32 ||
                unsafe { *z.offset(i_ret as isize) } as i32 >= 'a' as i32 &&
                    unsafe { *z.offset(i_ret as isize) } as i32 <= 'z' as i32 {
            { let __p = &mut i_ret; let __t = *__p; *__p += 1; __t };
        }
    }
    return i_ret;
}

extern "C" fn intck_is_space(c: i8) -> i32 {
    return (c as i32 == ' ' as i32 || c as i32 == '\t' as i32 ||
                    c as i32 == '\n' as i32 || c as i32 == '\r' as i32) as i32;
}

extern "C" fn intck_parse_create_index(z: *const i8, i_col_1: i32,
    pn_byte_1: &mut i32) -> *const i8 {
    let mut i_off: i32 = 0;
    let mut i_this_col: i32 = 0;
    let mut i_start: i32 = 0;
    let mut n_open: i32 = 0;
    let mut z_ret: *const i8 = core::ptr::null();
    let mut n_ret: i32 = 0;
    let mut i_end_of_col: i32 = 0;
    while unsafe { *z.offset(i_off as isize) } as i32 != '(' as i32 {
        i_off += intck_get_token(unsafe { &*z.offset(i_off as isize) });
        if unsafe { *z.offset(i_off as isize) } as i32 == '\u{0}' as i32 {
            return core::ptr::null();
        }
    }
    if !(unsafe { *z.offset(i_off as isize) } as i32 == '(' as i32) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"intckParseCreateIndex".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 380,
                c"z[iOff]==\'(\'".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n_open = 1;
    { let __p = &mut i_off; let __t = *__p; *__p += 1; __t };
    i_start = i_off;
    while unsafe { *z.offset(i_off as isize) } != 0 {
        let z_token: *const i8 = unsafe { &*z.offset(i_off as isize) };
        let mut n_token: i32 = 0;
        if n_open == 1 {
            if unsafe { *z.offset(i_off as isize) } as i32 == ',' as i32 ||
                    unsafe { *z.offset(i_off as isize) } as i32 == ')' as i32 {
                if i_col_1 == i_this_col {
                    let i_end: i32 =
                        if i_end_of_col != 0 { i_end_of_col } else { i_off };
                    n_ret = i_end - i_start;
                    z_ret = unsafe { z.offset(i_start as isize) };
                    break;
                }
                i_start = i_off + 1;
                while intck_is_space(unsafe { *z.offset(i_start as isize) })
                        != 0 {
                    { let __p = &mut i_start; let __t = *__p; *__p += 1; __t };
                }
                { let __p = &mut i_this_col; let __t = *__p; *__p += 1; __t };
            }
            if unsafe { *z.offset(i_off as isize) } as i32 == ')' as i32 {
                break;
            }
        }
        if unsafe { *z.offset(i_off as isize) } as i32 == '(' as i32 {
            { let __p = &mut n_open; let __t = *__p; *__p += 1; __t };
        }
        if unsafe { *z.offset(i_off as isize) } as i32 == ')' as i32 {
            { let __p = &mut n_open; let __t = *__p; *__p -= 1; __t };
        }
        n_token = intck_get_token(z_token);
        if n_token == 3 &&
                    0 ==
                        unsafe {
                            sqlite3_strnicmp(z_token,
                                c"ASC".as_ptr() as *mut i8 as *const i8, n_token)
                        } ||
                n_token == 4 &&
                    0 ==
                        unsafe {
                            sqlite3_strnicmp(z_token,
                                c"DESC".as_ptr() as *mut i8 as *const i8, n_token)
                        } {
            i_end_of_col = i_off;
        } else if 0 == intck_is_space(unsafe { *z_token.offset(0 as isize) })
            {
            i_end_of_col = 0;
        }
        i_off += n_token;
    }
    while z_ret == core::ptr::null() &&
            unsafe { *z.offset(i_off as isize) } != 0 {
        let n: i32 = intck_get_token(unsafe { &*z.offset(i_off as isize) });
        if n == 5 &&
                0 ==
                    unsafe {
                        sqlite3_strnicmp(unsafe { &*z.offset(i_off as isize) },
                            c"where".as_ptr() as *mut i8 as *const i8, 5)
                    } {
            z_ret = unsafe { z.offset((i_off + 5) as isize) };
            n_ret = unsafe { strlen(z_ret) } as i32;
        }
        i_off += n;
    }
    if !(z_ret).is_null() {
        while intck_is_space(unsafe { *z_ret.offset(0 as isize) }) != 0 {
            { let __p = &mut n_ret; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut z_ret;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        while n_ret > 0 &&
                intck_is_space(unsafe { *z_ret.offset((n_ret - 1) as isize) })
                    != 0 {
            { let __p = &mut n_ret; let __t = *__p; *__p -= 1; __t };
        }
    }
    *pn_byte_1 = n_ret;
    return z_ret;
}

extern "C" fn intck_parse_create_index_func(p_ctx_1: *mut Sqlite3Context,
    n_val_1: i32, ap_val_1: *mut *mut Sqlite3Value) -> () {
    let z_sql: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *ap_val_1.offset(0 as isize) }) }
            as *const i8;
    let idx: i32 =
        unsafe { sqlite3_value_int(unsafe { *ap_val_1.offset(1 as isize) }) };
    let mut z_res: *const i8 = core::ptr::null();
    let mut n_res: i32 = 0;
    if !(n_val_1 == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"intckParseCreateIndexFunc".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 459,
                c"nVal==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(z_sql).is_null() {
        z_res = intck_parse_create_index(z_sql, idx, &mut n_res);
    }
    unsafe {
        sqlite3_result_text(p_ctx_1, z_res, n_res,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_close(p: *mut Sqlite3Intck) -> () {
    if !(p).is_null() {
        unsafe { sqlite3_finalize(unsafe { (*p).p_check }) };
        unsafe {
            sqlite3_create_function(unsafe { (*p).db },
                c"parse_create_index".as_ptr() as *mut i8 as *const i8, 1, 1,
                core::ptr::null_mut(), None, None, None)
        };
        unsafe { sqlite3_free(unsafe { (*p).z_obj } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_key } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_test_sql } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_err } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_message } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_open(db: *mut Sqlite3, z_db_arg: *const i8,
    pp_out: &mut *mut Sqlite3Intck) -> i32 {
    let mut p_new: *mut Sqlite3Intck = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let z_db: *const i8 =
        if !(z_db_arg).is_null() {
            z_db_arg
        } else { c"main".as_ptr() as *mut i8 as *const i8 };
    let n_db: i32 = unsafe { strlen(z_db) } as i32;
    p_new =
        unsafe {
                sqlite3_malloc((core::mem::size_of::<Sqlite3Intck>() as u64 +
                                n_db as u64 + 1 as u64) as i32)
            } as *mut Sqlite3Intck;
    if p_new == core::ptr::null_mut() {
        rc = 7;
    } else {
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<Sqlite3Intck>() as u64)
        };
        unsafe { (*p_new).db = db };
        unsafe {
            (*p_new).z_db =
                unsafe { &raw mut *p_new.offset(1 as isize) } as *const i8
        };
        unsafe {
            memcpy(unsafe { &raw mut *p_new.offset(1 as isize) } as *mut (),
                z_db as *const (), (n_db + 1) as u64)
        };
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"parse_create_index".as_ptr() as *mut i8 as *const i8, 2,
                    1, core::ptr::null_mut(),
                    Some(intck_parse_create_index_func), None, None)
            };
        if rc != 0 {
            sqlite3_intck_close(p_new);
            p_new = core::ptr::null_mut();
        }
    }
    *pp_out = p_new;
    return rc;
}

extern "C" fn intck_save_errmsg(p: &mut Sqlite3Intck) -> () {
    (*p).rc = unsafe { sqlite3_errcode((*p).db) };
    unsafe { sqlite3_free((*p).z_err as *mut ()) };
    (*p).z_err =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_errmsg((*p).db) })
        };
}

extern "C" fn intck_prepare(p: *mut Sqlite3Intck, z_sql_1: *const i8)
    -> *mut Sqlite3Stmt {
    let mut p_ret: *mut Sqlite3Stmt = core::ptr::null_mut();
    if unsafe { (*p).rc } == 0 {
        unsafe {
            (*p).rc =
                unsafe {
                    sqlite3_prepare_v2(unsafe { (*p).db }, z_sql_1, -1,
                        &mut p_ret, core::ptr::null_mut())
                }
        };
        if unsafe { (*p).rc } != 0 {
            intck_save_errmsg(unsafe { &mut *p });
            if !(p_ret == core::ptr::null_mut()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"intckPrepare".as_ptr() as *const i8,
                        c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 82,
                        c"pRet==0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
        }
    }
    return p_ret;
}

unsafe extern "C" fn intck_prepare_fmt(p: *mut Sqlite3Intck,
    z_fmt_1: *const i8, mut __va0: ...) -> *mut Sqlite3Stmt {
    let mut p_ret: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    if unsafe { (*p).rc } == 0 && z_sql == core::ptr::null_mut() {
        unsafe { (*p).rc = 7 };
    }
    p_ret = intck_prepare(p, z_sql as *const i8);
    unsafe { sqlite3_free(z_sql as *mut ()) };
    ();
    return p_ret;
}

unsafe extern "C" fn intck_mprintf(p: &mut Sqlite3Intck, z_fmt_1: *const i8,
    mut __va0: ...) -> *mut i8 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_ret = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    if (*p).rc == 0 {
        if z_ret == core::ptr::null_mut() { (*p).rc = 7; }
    } else {
        unsafe { sqlite3_free(z_ret as *mut ()) };
        z_ret = core::ptr::null_mut();
    }
    ();
    return z_ret;
}

extern "C" fn intck_finalize(p: *mut Sqlite3Intck, p_stmt_1: *mut Sqlite3Stmt)
    -> () {
    let rc: i32 = unsafe { sqlite3_finalize(p_stmt_1) };
    if unsafe { (*p).rc } == 0 && rc != 0 {
        intck_save_errmsg(unsafe { &mut *p });
    }
}

extern "C" fn intck_find_object(p: *mut Sqlite3Intck) -> () {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let z_prev: *mut i8 = unsafe { (*p).z_obj };
    unsafe { (*p).z_obj = core::ptr::null_mut() };
    if !(unsafe { (*p).rc } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"intckFindObject".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 277,
                c"p->rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).p_check } == core::ptr::null_mut()) as i32 as i64 != 0
        {
        unsafe {
            __assert_rtn(c"intckFindObject".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 278,
                c"p->pCheck==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_stmt =
        unsafe {
            intck_prepare_fmt(p,
                c"WITH tables(table_name) AS (  SELECT name  FROM %Q.sqlite_schema WHERE (type=\'table\' OR type=\'index\') AND rootpage  UNION ALL   SELECT \'sqlite_schema\')SELECT table_name FROM tables WHERE ?1 IS NULL OR table_name%s?1 ORDER BY 1".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_db },
                if !(unsafe { (*p).z_key }).is_null() {
                    c">=".as_ptr() as *mut i8
                } else { c">".as_ptr() as *mut i8 })
        };
    if unsafe { (*p).rc } == 0 {
        unsafe {
            sqlite3_bind_text(p_stmt, 1, z_prev as *const i8, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            unsafe {
                (*p).z_obj =
                    unsafe {
                        intck_mprintf(unsafe { &mut *p },
                            c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8)
                    }
            };
        }
    }
    intck_finalize(p, p_stmt);
    if unsafe {
                sqlite3_stricmp(unsafe { (*p).z_obj } as *const i8,
                    z_prev as *const i8)
            } != 0 {
        unsafe { sqlite3_free(unsafe { (*p).z_key } as *mut ()) };
        unsafe { (*p).z_key = core::ptr::null_mut() };
    }
    unsafe { sqlite3_free(z_prev as *mut ()) };
}

extern "C" fn intck_step(p: &Sqlite3Intck, p_stmt_1: *mut Sqlite3Stmt)
    -> i32 {
    if (*p).rc != 0 { return (*p).rc; }
    return unsafe { sqlite3_step(p_stmt_1) };
}

extern "C" fn intck_get_auto_index(p: *mut Sqlite3Intck) -> i32 {
    let mut b_ret: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    p_stmt =
        intck_prepare(p,
            c"PRAGMA automatic_index".as_ptr() as *mut i8 as *const i8);
    if 100 == intck_step(unsafe { &*p }, p_stmt) {
        b_ret = unsafe { sqlite3_column_int(p_stmt, 0) };
    }
    intck_finalize(p, p_stmt);
    return b_ret;
}

extern "C" fn intck_exec(p: *mut Sqlite3Intck, z_sql_1: *const i8) -> () {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    p_stmt = intck_prepare(p, z_sql_1);
    intck_step(unsafe { &*p }, p_stmt);
    intck_finalize(p, p_stmt);
}

extern "C" fn intck_is_index(p: *mut Sqlite3Intck, z_obj_1: *const i8)
    -> i32 {
    let mut b_ret: i32 = 0;
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    p_stmt =
        unsafe {
            intck_prepare_fmt(p,
                c"SELECT 1 FROM %Q.sqlite_schema WHERE name=%Q AND type=\'index\'".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_db }, z_obj_1)
        };
    if unsafe { (*p).rc } == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
        b_ret = 1;
    }
    intck_finalize(p, p_stmt);
    return b_ret;
}

extern "C" fn intck_check_object_sql(p: *mut Sqlite3Intck, z_obj_1: *const i8,
    z_prev_1: *const i8, pn_key_val_1: *mut i32) -> *mut i8 {
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut b_auto_index: i32 = 0;
    let mut b_is_index: i32 = 0;
    let z_common: *const i8 =
        c", without_rowid(b) AS (  SELECT EXISTS (    SELECT 1 FROM tabname, pragma_index_list(tab, db) AS l      WHERE origin=\'pk\'       AND NOT EXISTS (SELECT 1 FROM sqlite_schema WHERE name=l.name)  )), idx_cols(idx_name, idx_ispk, col_name, col_expr, col_alias) AS (  SELECT l.name, (l.origin==\'pk\' AND w.b), i.name, COALESCE((    SELECT parse_create_index(sql, i.seqno) FROM     sqlite_schema WHERE name = l.name  ), format(\'\"%w\"\', i.name) || \' COLLATE \' || quote(i.coll)),  \'c\' || row_number() OVER ()  FROM       tabname t,      without_rowid w,      pragma_index_list(t.tab, t.db) l,      pragma_index_xinfo(l.name) i      WHERE i.key  UNION ALL  SELECT \'\', 1, \'_rowid_\', \'_rowid_\', \'r1\' FROM without_rowid WHERE b=0), tabpk(db, tab, idx, o_pk, i_pk, q_pk, eq_pk, ps_pk, pk_pk, n_pk) AS (    WITH pkfields(f, a) AS (      SELECT i.col_name, i.col_alias FROM idx_cols i WHERE i.idx_ispk    )    SELECT t.db, t.tab, t.idx,            group_concat(a, \', \'),            group_concat(\'i.\'||quote(f), \', \'),            group_concat(\'quote(o.\'||a||\')\', \' || \'\',\'\' || \'),             format(\'(%s)==(%s)\',               group_concat(\'o.\'||a, \', \'),                group_concat(format(\'\"%w\"\', f), \', \')           ),           group_concat(\'%s\', \',\'),           group_concat(\'quote(\'||a||\')\', \', \'),             count(*)    FROM tabname t, pkfields), idx(name, match_expr, partial, partial_alias, idx_ps, idx_idx) AS (  SELECT idx_name,    format(\'(%s,%s) IS (%s,%s)\',            group_concat(i.col_expr, \', \'), i_pk,           group_concat(\'o.\'||i.col_alias, \', \'), o_pk    ),     parse_create_index(        (SELECT sql FROM sqlite_schema WHERE name=idx_name), -1    ),    \'cond\' || row_number() OVER ()    , group_concat(\'%s\', \',\')    , group_concat(\'quote(\'||i.col_alias||\')\', \', \')  FROM tabpk t,        without_rowid w,       idx_cols i  WHERE i.idx_ispk==0   GROUP BY idx_name), wrapper_with(s) AS (  SELECT \'intck_wrapper AS (\n  SELECT\n    \' || (      WITH f(a, b) AS (        SELECT col_expr, col_alias FROM idx_cols          UNION ALL         SELECT partial, partial_alias FROM idx WHERE partial IS NOT NULL      )      SELECT group_concat(format(\'%s AS %s\', a, b), \',\n    \') FROM f    )    || format(\'\n  FROM %Q.%Q \', t.db, t.tab)    || CASE WHEN t.idx IS NULL THEN         \'NOT INDEXED\'       ELSE        format(\'INDEXED BY %Q%s\', t.idx, \' WHERE \'||i.partial)       END    || \'\n)\'    FROM tabname t LEFT JOIN idx i ON (i.name=t.idx))".as_ptr()
                as *mut i8 as *const i8;
    b_auto_index = intck_get_auto_index(p);
    if b_auto_index != 0 {
        intck_exec(p,
            c"PRAGMA automatic_index = 0".as_ptr() as *mut i8 as *const i8);
    }
    b_is_index = intck_is_index(p, z_obj_1);
    if b_is_index != 0 {
        p_stmt =
            unsafe {
                intck_prepare_fmt(p,
                    c"WITH tabname(db, tab, idx) AS (  SELECT %Q, (SELECT tbl_name FROM %Q.sqlite_schema WHERE name=%Q), %Q ), whereclause(w_c) AS (%s)%s, case_statement(c) AS (  SELECT     \'CASE WHEN (\' || group_concat(col_alias, \', \') || \', 1) IS (\n\'     || \'      SELECT \' || group_concat(col_expr, \', \') || \', 1 FROM \'    || format(\'%%Q.%%Q NOT INDEXED WHERE %%s\n\', t.db, t.tab, p.eq_pk)    || \'    )\n  THEN NULL\n    \'    || \'ELSE format(\'\'surplus entry (\'    ||   group_concat(\'%%s\', \',\') || \',\' || p.ps_pk    || \') in index \' || t.idx || \'\'\', \'     ||   group_concat(\'quote(\'||i.col_alias||\')\', \', \') || \', \' || p.pk_pk    || \')\'    || \'\n  END AS error_message\'  FROM tabname t, tabpk p, idx_cols i WHERE i.idx_name=t.idx), thiskey(k, n) AS (    SELECT group_concat(i.col_alias, \', \') || \', \' || p.o_pk,            count(*) + p.n_pk     FROM tabpk p, idx_cols i WHERE i.idx_name=p.idx), main_select(m, n) AS (  SELECT format(      \'WITH %%s\n\' ||      \', idx_checker AS (\n\' ||      \'  SELECT %%s,\n\' ||      \'  %%s\n\' ||       \'  FROM intck_wrapper AS o\n\' ||      \')\n\',      ww.s, c, t.k  ), t.n  FROM case_statement, wrapper_with ww, thiskey t)SELECT m ||     group_concat(\'SELECT * FROM idx_checker \' || w_c, \' UNION ALL \'), n FROM main_select, whereclause ".as_ptr()
                            as *mut i8 as *const i8, unsafe { (*p).z_db },
                    unsafe { (*p).z_db }, z_obj_1, z_obj_1,
                    if !(z_prev_1).is_null() {
                        z_prev_1
                    } else { c"VALUES(\'\')".as_ptr() as *mut i8 as *const i8 },
                    z_common)
            };
    } else {
        p_stmt =
            unsafe {
                intck_prepare_fmt(p,
                    c"WITH tabname(db, tab, idx, prev) AS (SELECT %Q, %Q, NULL, %Q)%s, expr(e, p) AS (  SELECT format(\'CASE WHEN EXISTS \n    (SELECT 1 FROM %%Q.%%Q AS i INDEXED BY %%Q WHERE %%s%%s)\n    THEN NULL\n    ELSE format(\'\'entry (%%s,%%s) missing from index %%s\'\', %%s, %%s)\n  END\n\'    , t.db, t.tab, i.name, i.match_expr, \' AND (\' || partial || \')\',      i.idx_ps, t.ps_pk, i.name, i.idx_idx, t.pk_pk),    CASE WHEN partial IS NULL THEN NULL ELSE i.partial_alias END  FROM tabpk t, idx i), numbered(ii, cond, e) AS (  SELECT 0, \'n.ii=0\', \'NULL\'    UNION ALL   SELECT row_number() OVER (),      \'(n.ii=\'||row_number() OVER ()||COALESCE(\' AND \'||p||\')\', \')\'), e  FROM expr), counter_with(w) AS (    SELECT \'WITH intck_counter(ii) AS (\n  \' ||        group_concat(\'SELECT \'||ii, \' UNION ALL\n  \')     || \'\n)\' FROM numbered), case_statement(c) AS (    SELECT \'CASE \' ||     group_concat(format(\'\n  WHEN %%s THEN (%%s)\', cond, e), \'\') ||    \'\nEND AS error_message\'    FROM numbered), thiskey(k, n) AS (    SELECT o_pk || \', ii\', n_pk+1 FROM tabpk), whereclause(w_c) AS (    SELECT CASE WHEN prev!=\'\' THEN     \'\nWHERE (\' || o_pk ||\', n.ii) > \' || prev    ELSE \'\'    END    FROM tabpk, tabname), main_select(m, n) AS (  SELECT format(      \'%%s, %%s\nSELECT %%s,\n%%s\nFROM intck_wrapper AS o, intck_counter AS n%%s\nORDER BY %%s\',       w, ww.s, c, thiskey.k, whereclause.w_c, t.o_pk  ), thiskey.n  FROM case_statement, tabpk t, counter_with,        wrapper_with ww, thiskey, whereclause)SELECT m, n FROM main_select".as_ptr()
                            as *mut i8 as *const i8, unsafe { (*p).z_db }, z_obj_1,
                    z_prev_1, z_common)
            };
    }
    while unsafe { (*p).rc } == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
        z_ret =
            unsafe {
                intck_mprintf(unsafe { &mut *p },
                    c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8)
            };
        if !(pn_key_val_1).is_null() {
            unsafe {
                *pn_key_val_1 = unsafe { sqlite3_column_int(p_stmt, 1) }
            };
        }
    }
    intck_finalize(p, p_stmt);
    if b_auto_index != 0 {
        intck_exec(p,
            c"PRAGMA automatic_index = 1".as_ptr() as *mut i8 as *const i8);
    }
    return z_ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_step(p: *mut Sqlite3Intck) -> i32 {
    if unsafe { (*p).rc } == 0 {
        if !(unsafe { (*p).z_message }).is_null() {
            unsafe { sqlite3_free(unsafe { (*p).z_message } as *mut ()) };
            unsafe { (*p).z_message = core::ptr::null_mut() };
        }
        if unsafe { (*p).b_corrupt_schema } != 0 {
            unsafe { (*p).rc = 101 };
        } else if unsafe { (*p).p_check } == core::ptr::null_mut() {
            intck_find_object(p);
            if unsafe { (*p).rc } == 0 {
                if !(unsafe { (*p).z_obj }).is_null() {
                    let mut z_sql: *mut i8 = core::ptr::null_mut();
                    z_sql =
                        intck_check_object_sql(p,
                            unsafe { (*p).z_obj } as *const i8,
                            unsafe { (*p).z_key } as *const i8,
                            unsafe { &mut (*p).n_key_val });
                    unsafe {
                        (*p).p_check = intck_prepare(p, z_sql as *const i8)
                    };
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    unsafe { sqlite3_free(unsafe { (*p).z_key } as *mut ()) };
                    unsafe { (*p).z_key = core::ptr::null_mut() };
                } else { unsafe { (*p).rc = 101 }; }
            } else if unsafe { (*p).rc } == 11 {
                unsafe { (*p).rc = 0 };
                unsafe {
                    (*p).z_message =
                        unsafe {
                            intck_mprintf(unsafe { &mut *p },
                                c"%s".as_ptr() as *mut i8 as *const i8,
                                c"corruption found while reading database schema".as_ptr()
                                    as *mut i8)
                        }
                };
                unsafe { (*p).b_corrupt_schema = 1 };
            }
        }
        if !(unsafe { (*p).p_check }).is_null() {
            if !(unsafe { (*p).rc } == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3_intck_step".as_ptr() as *const i8,
                        c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 867,
                        c"p->rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if unsafe { sqlite3_step(unsafe { (*p).p_check }) } == 100
                {} else {
                intck_finalize(p, unsafe { (*p).p_check });
                unsafe { (*p).p_check = core::ptr::null_mut() };
                unsafe { (*p).n_key_val = 0 };
                if unsafe { (*p).rc } == 11 {
                    unsafe { (*p).rc = 0 };
                    unsafe {
                        (*p).z_message =
                            unsafe {
                                intck_mprintf(unsafe { &mut *p },
                                    c"corruption found while scanning database object %s".as_ptr()
                                            as *mut i8 as *const i8, unsafe { (*p).z_obj })
                            }
                    };
                }
            }
        }
    }
    return unsafe { (*p).rc };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_message(p: &Sqlite3Intck) -> *const i8 {
    if !((*p).p_check == core::ptr::null_mut() ||
                            (*p).z_message == core::ptr::null_mut()) as i32 as i64 != 0
        {
        unsafe {
            __assert_rtn(c"sqlite3_intck_message".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 892,
                c"p->pCheck==0 || p->zMessage==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !((*p).z_message).is_null() { return (*p).z_message as *const i8; }
    if !((*p).p_check).is_null() {
        return unsafe { sqlite3_column_text((*p).p_check, 0) } as *const i8;
    }
    return core::ptr::null();
}

extern "C" fn intck_save_key(p: *mut Sqlite3Intck) -> () {
    let mut ii: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_xinfo: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut z_dir: *const i8 = core::ptr::null();
    if (unsafe { (*p).p_check }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"intckSaveKey".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 179,
                c"p->pCheck".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).z_key } == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"intckSaveKey".as_ptr() as *const i8,
                c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 180,
                c"p->zKey==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_xinfo =
        unsafe {
            intck_prepare_fmt(p,
                c"SELECT group_concat(desc, \'\') FROM %Q.sqlite_schema s, pragma_index_xinfo(%Q, %Q) WHERE s.type=\'index\' AND s.name=%Q".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_db },
                unsafe { (*p).z_obj }, unsafe { (*p).z_db },
                unsafe { (*p).z_obj })
        };
    if unsafe { (*p).rc } == 0 && 100 == unsafe { sqlite3_step(p_xinfo) } {
        z_dir = unsafe { sqlite3_column_text(p_xinfo, 0) } as *const i8;
    }
    if z_dir == core::ptr::null() {
        let mut z_sep: *const i8 =
            c"SELECT \'(\' || ".as_ptr() as *mut i8 as *const i8;
        {
            ii = 0;
            '__b10: loop {
                if !(ii < unsafe { (*p).n_key_val }) { break '__b10; }
                '__c10: loop {
                    z_sql =
                        unsafe {
                            intck_mprintf(unsafe { &mut *p },
                                c"%z%squote(?)".as_ptr() as *mut i8 as *const i8, z_sql,
                                z_sep)
                        };
                    z_sep = c" || \', \' || ".as_ptr() as *mut i8 as *const i8;
                    break '__c10;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        z_sql =
            unsafe {
                intck_mprintf(unsafe { &mut *p },
                    c"%z || \')\'".as_ptr() as *mut i8 as *const i8, z_sql)
            };
    } else {
        if !(unsafe { (*p).n_key_val } > 1) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"intckSaveKey".as_ptr() as *const i8,
                    c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 204,
                    c"p->nKeyVal>1".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        {
            ii = unsafe { (*p).n_key_val };
            '__b11: loop {
                if !(ii > 0) { break '__b11; }
                '__c11: loop {
                    let b_last_is_desc: i32 =
                        (unsafe { *z_dir.offset((ii - 1) as isize) } as i32 ==
                                '1' as i32) as i32;
                    let b_last_is_null: i32 =
                        (unsafe { sqlite3_column_type(unsafe { (*p).p_check }, ii) }
                                == 5) as i32;
                    let z_last: *const i8 =
                        unsafe { sqlite3_column_name(unsafe { (*p).p_check }, ii) };
                    let mut z_lhs: *mut i8 = core::ptr::null_mut();
                    let mut z_rhs: *mut i8 = core::ptr::null_mut();
                    let mut z_where: *mut i8 = core::ptr::null_mut();
                    if b_last_is_null != 0 {
                        if b_last_is_desc != 0 { break '__c11; }
                        z_where =
                            unsafe {
                                intck_mprintf(unsafe { &mut *p },
                                    c"\'%s IS NOT NULL\'".as_ptr() as *mut i8 as *const i8,
                                    z_last)
                            };
                    } else {
                        let z_op: *const i8 =
                            if b_last_is_desc != 0 {
                                    c"<".as_ptr() as *mut i8
                                } else { c">".as_ptr() as *mut i8 } as *const i8;
                        z_where =
                            unsafe {
                                intck_mprintf(unsafe { &mut *p },
                                    c"\'%s %s \' || quote(?%d)".as_ptr() as *mut i8 as
                                        *const i8, z_last, z_op, ii)
                            };
                    }
                    if ii > 1 {
                        let mut z_lhs_sep: *const i8 =
                            c"".as_ptr() as *mut i8 as *const i8;
                        let mut z_rhs_sep: *const i8 =
                            c"".as_ptr() as *mut i8 as *const i8;
                        let mut jj: i32 = 0;
                        {
                            jj = 0;
                            '__b12: loop {
                                if !(jj < ii - 1) { break '__b12; }
                                '__c12: loop {
                                    let z_alias: *const i8 =
                                        unsafe {
                                                sqlite3_column_name(unsafe { (*p).p_check }, jj + 1)
                                            } as *const i8;
                                    z_lhs =
                                        unsafe {
                                            intck_mprintf(unsafe { &mut *p },
                                                c"%z%s%s".as_ptr() as *mut i8 as *const i8, z_lhs,
                                                z_lhs_sep, z_alias)
                                        };
                                    z_rhs =
                                        unsafe {
                                            intck_mprintf(unsafe { &mut *p },
                                                c"%z%squote(?%d)".as_ptr() as *mut i8 as *const i8, z_rhs,
                                                z_rhs_sep, jj + 1)
                                        };
                                    z_lhs_sep = c",".as_ptr() as *mut i8 as *const i8;
                                    z_rhs_sep =
                                        c" || \',\' || ".as_ptr() as *mut i8 as *const i8;
                                    break '__c12;
                                }
                                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        z_where =
                            unsafe {
                                intck_mprintf(unsafe { &mut *p },
                                    c"\'(%z) IS (\' || %z || \') AND \' || %z".as_ptr() as
                                            *mut i8 as *const i8, z_lhs, z_rhs, z_where)
                            };
                    }
                    z_where =
                        unsafe {
                            intck_mprintf(unsafe { &mut *p },
                                c"\'WHERE \' || %z".as_ptr() as *mut i8 as *const i8,
                                z_where)
                        };
                    z_sql =
                        unsafe {
                            intck_mprintf(unsafe { &mut *p },
                                c"%z%s(quote( %z ) )".as_ptr() as *mut i8 as *const i8,
                                z_sql,
                                if z_sql == core::ptr::null_mut() {
                                    c"VALUES".as_ptr() as *mut i8
                                } else { c",\n      ".as_ptr() as *mut i8 }, z_where)
                        };
                    break '__c11;
                }
                { let __p = &mut ii; let __t = *__p; *__p -= 1; __t };
            }
        }
        z_sql =
            unsafe {
                intck_mprintf(unsafe { &mut *p },
                    c"WITH wc(q) AS (\n%z\n)SELECT \'VALUES\' || group_concat(\'(\' || q || \')\', \',\n      \') FROM wc".as_ptr()
                            as *mut i8 as *const i8, z_sql)
            };
    }
    p_stmt = intck_prepare(p, z_sql as *const i8);
    if unsafe { (*p).rc } == 0 {
        {
            ii = 0;
            '__b13: loop {
                if !(ii < unsafe { (*p).n_key_val }) { break '__b13; }
                '__c13: loop {
                    unsafe {
                        sqlite3_bind_value(p_stmt, ii + 1,
                            unsafe {
                                    sqlite3_column_value(unsafe { (*p).p_check }, ii + 1)
                                } as *const Sqlite3Value)
                    };
                    break '__c13;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        if 100 == unsafe { sqlite3_step(p_stmt) } {
            unsafe {
                (*p).z_key =
                    unsafe {
                        intck_mprintf(unsafe { &mut *p },
                            c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8)
                    }
            };
        }
        intck_finalize(p, p_stmt);
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    intck_finalize(p, p_xinfo);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_unlock(p: *mut Sqlite3Intck) -> i32 {
    if unsafe { (*p).rc } == 0 && !(unsafe { (*p).p_check }).is_null() {
        if !(unsafe { (*p).z_key } == core::ptr::null_mut() &&
                                unsafe { (*p).n_key_val } > 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3_intck_unlock".as_ptr() as *const i8,
                    c"sqlite3intck.c".as_ptr() as *mut i8 as *const i8, 916,
                    c"p->zKey==0 && p->nKeyVal>0".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        intck_save_key(p);
        intck_finalize(p, unsafe { (*p).p_check });
        unsafe { (*p).p_check = core::ptr::null_mut() };
    }
    return unsafe { (*p).rc };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_error(p: &Sqlite3Intck,
    pz_err: *mut *const i8) -> i32 {
    if !(pz_err).is_null() { unsafe { *pz_err = (*p).z_err as *const i8 }; }
    return if (*p).rc == 101 { 0 } else { (*p).rc };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_intck_test_sql(p: *mut Sqlite3Intck,
    z_obj: *const i8) -> *const i8 {
    unsafe { sqlite3_free(unsafe { (*p).z_test_sql } as *mut ()) };
    if !(z_obj).is_null() {
        unsafe {
            (*p).z_test_sql =
                intck_check_object_sql(p, z_obj, core::ptr::null(),
                    core::ptr::null_mut())
        };
    } else {
        if !(unsafe { (*p).z_obj }).is_null() {
            unsafe {
                (*p).z_test_sql =
                    intck_check_object_sql(p,
                        unsafe { (*p).z_obj } as *const i8,
                        unsafe { (*p).z_key } as *const i8, core::ptr::null_mut())
            };
        } else {
            unsafe { sqlite3_free(unsafe { (*p).z_test_sql } as *mut ()) };
            unsafe { (*p).z_test_sql = core::ptr::null_mut() };
        }
    }
    return unsafe { (*p).z_test_sql } as *const i8;
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
    fn strlen(__s: *const i8)
    -> u64;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
