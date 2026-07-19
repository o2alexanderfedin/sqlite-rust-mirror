#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor, SqliteInt64,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct VtablogVtab {
    base: Sqlite3Vtab,
    z_db: *mut i8,
    z_name: *mut i8,
    n_row: i32,
    n_cursor: i32,
    i_consume_ob: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VtablogCursor {
    base: Sqlite3VtabCursor,
    i_cursor: i32,
    i_rowid: Sqlite3Int64,
}

/// Skip leading whitespace.  Return a pointer to the first non-whitespace
///* character, or to the zero terminator if the string has only whitespace
extern "C" fn vtablog_skip_whitespace(mut z: *const i8) -> *const i8 {
    while unsafe { isspace(unsafe { *z.offset(0 as isize) } as u8 as i32) } !=
            0 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return z;
}

/// Remove trailing whitespace from the end of string z[]
extern "C" fn vtablog_trim_whitespace(z: *mut i8) -> () {
    let mut n: u64 = unsafe { strlen(z as *const i8) };
    while n > 0 as u64 &&
            unsafe { isspace(unsafe { *z.add(n as usize) } as u8 as i32) } !=
                0 {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    unsafe { *z.add(n as usize) = 0 as i8 };
}

/// Dequote the string
extern "C" fn vtablog_dequote(z: *mut i8) -> () {
    let mut j: i32 = 0;
    let c_quote: i8 = unsafe { *z.offset(0 as isize) };
    let mut i: u64 = 0 as u64;
    let mut n: u64 = 0 as u64;
    if c_quote as i32 != '\'' as i32 && c_quote as i32 != '\"' as i32 {
        return;
    }
    n = unsafe { strlen(z as *const i8) };
    if n < 2 as u64 ||
            unsafe { *z.add((n - 1 as u64) as usize) } as i32 !=
                unsafe { *z.offset(0 as isize) } as i32 {
        return;
    }
    {
        { ({ i = 1 as u64; i }) as i32; j = 0 };
        '__b2: loop {
            if !(i < n - 1 as u64) { break '__b2; }
            '__c2: loop {
                if unsafe { *z.add(i as usize) } as i32 == c_quote as i32 &&
                        unsafe { *z.add((i + 1 as u64) as usize) } as i32 ==
                            c_quote as i32 {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
                unsafe {
                    *z.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = unsafe { *z.add(i as usize) }
                };
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z.offset(j as isize) = 0 as i8 };
}

/// Check to see if the string is of the form:  "TAG = VALUE" with optional
///* whitespace before and around tokens.  If it is, return a pointer to the
///* first character of VALUE.  If it is not, return NULL.
extern "C" fn vtablog_parameter(z_tag_1: *const i8, n_tag_1: i32,
    mut z: *const i8) -> *const i8 {
    z = vtablog_skip_whitespace(z);
    if unsafe { strncmp(z_tag_1, z, n_tag_1 as u64) } != 0 {
        return core::ptr::null();
    }
    z = vtablog_skip_whitespace(unsafe { z.offset(n_tag_1 as isize) });
    if unsafe { *z.offset(0 as isize) } as i32 != '=' as i32 {
        return core::ptr::null();
    }
    return vtablog_skip_whitespace(unsafe { z.offset(1 as isize) });
}

/// Decode a parameter that requires a dequoted string.
///*
///* Return non-zero on an error.
extern "C" fn vtablog_string_parameter(pz_err_1: &mut *mut i8,
    z_param_1: *const i8, z_arg_1: *const i8, pz_val_1: &mut *mut i8) -> i32 {
    let mut z_value: *const i8 = core::ptr::null();
    z_value =
        vtablog_parameter(z_param_1, unsafe { strlen(z_param_1) } as i32,
            z_arg_1);
    if z_value == core::ptr::null() { return 0; }
    if !(*pz_val_1).is_null() {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"more than one \'%s\' parameter".as_ptr() as
                            *mut i8 as *const i8, z_param_1)
            };
        return 1;
    }
    *pz_val_1 =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_value)
        };
    if *pz_val_1 == core::ptr::null_mut() {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"out of memory".as_ptr() as *mut i8 as
                        *const i8)
            };
        return 1;
    }
    vtablog_trim_whitespace(*pz_val_1);
    vtablog_dequote(*pz_val_1);
    return 0;
}

///* The vtablogConnect() method is invoked to create a new
///* vtablog_vtab that describes the vtablog virtual table.
///*
///* Think of this routine as the constructor for vtablog_vtab objects.
///*
///* All this routine needs to do is:
///*
///*    (1) Allocate the vtablog_vtab object and initialize all fields.
///*
///*    (2) Tell SQLite (via the sqlite3_declare_vtab() interface) what the
///*        result set of queries against vtablog will look like.
extern "C" fn vtablog_connect_create(db: *mut Sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: &mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8, is_create_1: i32) -> i32 {
    let mut p_new: *mut VtablogVtab = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut z_schema: *mut i8 = core::ptr::null_mut();
    let mut z_n_row: *mut i8 = core::ptr::null_mut();
    let mut z_consume_ob: *mut i8 = core::ptr::null_mut();
    let mut z: *const i8 = core::ptr::null();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s4:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe { sqlite3_free(z_schema as *mut ()) };
                    __state = 54;
                }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { z_schema = core::ptr::null_mut(); __state = 6; }
                6 => { z_n_row = core::ptr::null_mut(); __state = 7; }
                7 => { z_consume_ob = core::ptr::null_mut(); __state = 8; }
                8 => {
                    unsafe {
                        printf(c"%s.%s.%s():\n".as_ptr() as *mut i8 as *const i8,
                            unsafe { *argv.offset(1 as isize) },
                            unsafe { *argv.offset(2 as isize) },
                            if is_create_1 != 0 {
                                c"xCreate".as_ptr() as *mut i8
                            } else { c"xConnect".as_ptr() as *mut i8 })
                    };
                    __state = 9;
                }
                9 => {
                    unsafe {
                        printf(c"  argc=%d\n".as_ptr() as *mut i8 as *const i8,
                            argc)
                    };
                    __state = 10;
                }
                10 => { i = 0; __state = 12; }
                11 => { i = 3; __state = 19; }
                12 => { if i < argc { __state = 13; } else { __state = 11; } }
                13 => {
                    unsafe {
                        printf(c"  argv[%d] = ".as_ptr() as *mut i8 as *const i8, i)
                    };
                    __state = 15;
                }
                14 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 12;
                }
                15 => {
                    if !(unsafe { *argv.offset(i as isize) }).is_null() {
                        __state = 16;
                    } else { __state = 17; }
                }
                16 => {
                    unsafe {
                        printf(c"[%s]\n".as_ptr() as *mut i8 as *const i8,
                            unsafe { *argv.offset(i as isize) })
                    };
                    __state = 14;
                }
                17 => {
                    unsafe {
                        printf(c"NULL\n".as_ptr() as *mut i8 as *const i8)
                    };
                    __state = 14;
                }
                18 => {
                    if z_schema == core::ptr::null_mut() {
                        __state = 32;
                    } else { __state = 31; }
                }
                19 => { if i < argc { __state = 20; } else { __state = 18; } }
                20 => {
                    z = unsafe { *argv.offset(i as isize) };
                    __state = 22;
                }
                21 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 19;
                }
                22 => {
                    if vtablog_string_parameter(unsafe { &mut *pz_err_1 },
                                c"schema".as_ptr() as *mut i8 as *const i8, z,
                                &mut z_schema) != 0 {
                        __state = 24;
                    } else { __state = 23; }
                }
                23 => {
                    if vtablog_string_parameter(unsafe { &mut *pz_err_1 },
                                c"rows".as_ptr() as *mut i8 as *const i8, z, &mut z_n_row)
                            != 0 {
                        __state = 27;
                    } else { __state = 26; }
                }
                24 => { rc = 1; __state = 25; }
                25 => { __state = 2; }
                26 => {
                    if vtablog_string_parameter(unsafe { &mut *pz_err_1 },
                                c"consume_order_by".as_ptr() as *mut i8 as *const i8, z,
                                &mut z_consume_ob) != 0 {
                        __state = 29;
                    } else { __state = 21; }
                }
                27 => { rc = 1; __state = 28; }
                28 => { __state = 2; }
                29 => { rc = 1; __state = 30; }
                30 => { __state = 2; }
                31 => {
                    unsafe {
                        printf(c"  schema = \'%s\'\n".as_ptr() as *mut i8 as
                                *const i8, z_schema)
                    };
                    __state = 36;
                }
                32 => {
                    z_schema =
                        unsafe {
                            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                c"CREATE TABLE x(a,b);".as_ptr() as *mut i8)
                        };
                    __state = 33;
                }
                33 => {
                    if z_schema == core::ptr::null_mut() {
                        __state = 34;
                    } else { __state = 31; }
                }
                34 => { rc = 7; __state = 35; }
                35 => { __state = 2; }
                36 => {
                    rc =
                        unsafe { sqlite3_declare_vtab(db, z_schema as *const i8) };
                    __state = 37;
                }
                37 => { if rc == 0 { __state = 39; } else { __state = 38; } }
                38 => { __state = 2; }
                39 => {
                    p_new =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<VtablogVtab>() as
                                        Sqlite3Uint64)
                            } as *mut VtablogVtab;
                    __state = 40;
                }
                40 => {
                    *pp_vtab_1 = p_new as *mut Sqlite3Vtab;
                    __state = 41;
                }
                41 => {
                    if p_new == core::ptr::null_mut() {
                        __state = 43;
                    } else { __state = 42; }
                }
                42 => {
                    unsafe {
                        memset(p_new as *mut (), 0,
                            core::mem::size_of::<VtablogVtab>() as u64)
                    };
                    __state = 44;
                }
                43 => { return 7; }
                44 => { unsafe { (*p_new).n_row = 10 }; __state = 45; }
                45 => {
                    if !(z_n_row).is_null() {
                        __state = 47;
                    } else { __state = 46; }
                }
                46 => {
                    unsafe {
                        printf(c"  nrow = %d\n".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_new).n_row })
                    };
                    __state = 48;
                }
                47 => {
                    unsafe {
                        (*p_new).n_row = unsafe { atoi(z_n_row as *const i8) }
                    };
                    __state = 46;
                }
                48 => {
                    if !(z_consume_ob).is_null() {
                        __state = 50;
                    } else { __state = 49; }
                }
                49 => {
                    if unsafe { (*p_new).i_consume_ob } != 0 {
                        __state = 52;
                    } else { __state = 51; }
                }
                50 => {
                    unsafe {
                        (*p_new).i_consume_ob =
                            unsafe { atoi(z_consume_ob as *const i8) }
                    };
                    __state = 49;
                }
                51 => {
                    unsafe {
                        (*p_new).z_db =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(1 as isize) })
                            }
                    };
                    __state = 53;
                }
                52 => {
                    unsafe {
                        printf(c"  consume_order_by = %d\n".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_new).i_consume_ob })
                    };
                    __state = 51;
                }
                53 => {
                    unsafe {
                        (*p_new).z_name =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(2 as isize) })
                            }
                    };
                    __state = 38;
                }
                54 => {
                    unsafe { sqlite3_free(z_n_row as *mut ()) };
                    __state = 55;
                }
                55 => {
                    unsafe { sqlite3_free(z_consume_ob as *mut ()) };
                    __state = 56;
                }
                56 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}

extern "C" fn vtablog_create(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    return vtablog_connect_create(db, p_aux_1, argc, argv,
            unsafe { &mut *pp_vtab_1 }, pz_err_1, 1);
}

extern "C" fn vtablog_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    return vtablog_connect_create(db, p_aux_1, argc, argv,
            unsafe { &mut *pp_vtab_1 }, pz_err_1, 0);
}

///* This method is the destructor for vtablog_vtab objects.
extern "C" fn vtablog_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *const VtablogVtab =
        p_vtab_1 as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xDisconnect()\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    unsafe { sqlite3_free(unsafe { (*p_tab).z_db } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p_tab).z_name } as *mut ()) };
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}

///* This method is (also) the destructor for vtablog_vtab objects.
extern "C" fn vtablog_destroy(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *const VtablogVtab =
        p_vtab_1 as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xDestroy()\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    unsafe { sqlite3_free(unsafe { (*p_tab).z_db } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p_tab).z_name } as *mut ()) };
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}

///* Constructor for a new vtablog_cursor object.
extern "C" fn vtablog_open(p: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p_tab: *mut VtablogVtab = p as *mut VtablogVtab;
    let mut p_cur: *mut VtablogCursor = core::ptr::null_mut();
    unsafe {
        printf(c"%s.%s.xOpen(cursor=%d)\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name },
            { let __p = unsafe { &mut (*p_tab).n_cursor }; *__p += 1; *__p })
    };
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<VtablogCursor>() as
                        Sqlite3Uint64)
            } as *mut VtablogCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<VtablogCursor>() as u64)
    };
    unsafe { (*p_cur).i_cursor = unsafe { (*p_tab).n_cursor } };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}

///* Destructor for a vtablog_cursor.
extern "C" fn vtablog_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const VtablogCursor =
        cur as *mut VtablogCursor as *const VtablogCursor;
    let p_tab: *const VtablogVtab =
        unsafe { (*cur).p_vtab } as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xClose(cursor=%d)\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name },
            unsafe { (*p_cur).i_cursor })
    };
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0;
}

///* Advance a vtablog_cursor to its next row of output.
extern "C" fn vtablog_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut VtablogCursor = cur as *mut VtablogCursor;
    let p_tab: *const VtablogVtab =
        unsafe { (*cur).p_vtab } as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xNext(cursor=%d)  rowid %d -> %d\n".as_ptr() as *mut i8
                as *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, unsafe { (*p_cur).i_cursor },
            unsafe { (*p_cur).i_rowid } as i32,
            unsafe { (*p_cur).i_rowid } as i32 + 1)
    };
    {
        let __p = unsafe { &mut (*p_cur).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return 0;
}

///* Return values of columns for the row at which the vtablog_cursor
///* is currently pointing.
extern "C" fn vtablog_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const VtablogCursor =
        cur as *mut VtablogCursor as *const VtablogCursor;
    let p_tab: *const VtablogVtab =
        unsafe { (*cur).p_vtab } as *mut VtablogVtab as *const VtablogVtab;
    let mut z_val: [i8; 50] = [0; 50];
    if i < 26 {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 50]>() as i32,
                &raw mut z_val[0 as usize] as *mut i8,
                c"%c%d".as_ptr() as *mut i8 as *const i8,
                unsafe {
                        *(c"abcdefghijklmnopqrstuvwyz".as_ptr() as
                                    *mut i8).offset(i as isize)
                    } as i32, unsafe { (*p_cur).i_rowid })
        };
    } else {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 50]>() as i32,
                &raw mut z_val[0 as usize] as *mut i8,
                c"{%d}%d".as_ptr() as *mut i8 as *const i8, i,
                unsafe { (*p_cur).i_rowid })
        };
    }
    unsafe {
        printf(c"%s.%s.xColumn(cursor=%d, i=%d): [%s]\n".as_ptr() as *mut i8
                as *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, unsafe { (*p_cur).i_cursor }, i,
            &raw mut z_val[0 as usize] as *mut i8)
    };
    unsafe {
        sqlite3_result_text(ctx,
            &raw mut z_val[0 as usize] as *mut i8 as *const i8, -1,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
    return 0;
}

///* Return the rowid for the current row.  In this implementation, the
///* rowid is the same as the output value.
extern "C" fn vtablog_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const VtablogCursor =
        cur as *mut VtablogCursor as *const VtablogCursor;
    let p_tab: *const VtablogVtab =
        unsafe { (*cur).p_vtab } as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xRowid(cursor=%d): %d\n".as_ptr() as *mut i8 as
                *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, unsafe { (*p_cur).i_cursor },
            unsafe { (*p_cur).i_rowid } as i32)
    };
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}

///* Return TRUE if the cursor has been moved off of the last
///* row of output.
extern "C" fn vtablog_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const VtablogCursor =
        cur as *mut VtablogCursor as *const VtablogCursor;
    let p_tab: *const VtablogVtab =
        unsafe { (*cur).p_vtab } as *mut VtablogVtab as *const VtablogVtab;
    let rc: i32 =
        (unsafe { (*p_cur).i_rowid } >= unsafe { (*p_tab).n_row } as i64) as
            i32;
    unsafe {
        printf(c"%s.%s.xEof(cursor=%d): %d\n".as_ptr() as *mut i8 as
                *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, unsafe { (*p_cur).i_cursor }, rc)
    };
    return rc;
}

///* Output an sqlite3_value object's value as an SQL literal.
extern "C" fn vtablog_quote(p: *mut Sqlite3Value) -> () {
    let mut z: [i8; 50] = [0; 50];
    '__s5:
        {
        match unsafe { sqlite3_value_type(p) } {
            5 => {
                {
                    unsafe { printf(c"NULL".as_ptr() as *mut i8 as *const i8) };
                    break '__s5;
                }
                {
                    unsafe {
                        sqlite3_snprintf(50, &raw mut z[0 as usize] as *mut i8,
                            c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_int64(p) })
                    };
                    unsafe {
                        printf(c"%s".as_ptr() as *mut i8 as *const i8,
                            &raw mut z[0 as usize] as *mut i8)
                    };
                    break '__s5;
                }
                {
                    unsafe {
                        sqlite3_snprintf(50, &raw mut z[0 as usize] as *mut i8,
                            c"%!.20g".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_double(p) })
                    };
                    unsafe {
                        printf(c"%s".as_ptr() as *mut i8 as *const i8,
                            &raw mut z[0 as usize] as *mut i8)
                    };
                    break '__s5;
                }
                {
                    let n: i32 = unsafe { sqlite3_value_bytes(p) };
                    let mut z: *const u8 =
                        unsafe { sqlite3_value_blob(p) } as *const u8;
                    let mut i: i32 = 0;
                    unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                    {
                        i = 0;
                        '__b6: loop {
                            if !(i < n) { break '__b6; }
                            '__c6: loop {
                                unsafe {
                                    printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                        unsafe { *z.offset(i as isize) } as i32)
                                };
                                break '__c6;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    break '__s5;
                }
                {
                    let mut z: *const i8 =
                        unsafe { sqlite3_value_text(p) } as *const i8;
                    let mut i: i32 = 0;
                    let mut c: i8 = 0 as i8;
                    {
                        i = 0;
                        '__b7: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                    i32 != 0 && c as i32 != '\'' as i32) {
                                break '__b7;
                            }
                            '__c7: loop { break '__c7; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c as i32 == 0 {
                        unsafe {
                            printf(c"\'%s\'".as_ptr() as *mut i8 as *const i8, z)
                        };
                    } else {
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                        while unsafe { *z } != 0 {
                            {
                                i = 0;
                                '__b9: loop {
                                    if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                            i32 != 0 && c as i32 != '\'' as i32) {
                                        break '__b9;
                                    }
                                    '__c9: loop { break '__c9; }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if c as i32 == '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                            if i != 0 {
                                unsafe {
                                    printf(c"%.*s".as_ptr() as *mut i8 as *const i8, i, z)
                                };
                                {
                                    let __n = i;
                                    let __p = &mut z;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                            }
                            if c as i32 == '\'' as i32 {
                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                continue;
                            }
                            if c as i32 == 0 { break; }
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    }
                    break '__s5;
                }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_snprintf(50, &raw mut z[0 as usize] as *mut i8,
                            c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_int64(p) })
                    };
                    unsafe {
                        printf(c"%s".as_ptr() as *mut i8 as *const i8,
                            &raw mut z[0 as usize] as *mut i8)
                    };
                    break '__s5;
                }
                {
                    unsafe {
                        sqlite3_snprintf(50, &raw mut z[0 as usize] as *mut i8,
                            c"%!.20g".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_double(p) })
                    };
                    unsafe {
                        printf(c"%s".as_ptr() as *mut i8 as *const i8,
                            &raw mut z[0 as usize] as *mut i8)
                    };
                    break '__s5;
                }
                {
                    let n: i32 = unsafe { sqlite3_value_bytes(p) };
                    let mut z: *const u8 =
                        unsafe { sqlite3_value_blob(p) } as *const u8;
                    let mut i: i32 = 0;
                    unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                    {
                        i = 0;
                        '__b6: loop {
                            if !(i < n) { break '__b6; }
                            '__c6: loop {
                                unsafe {
                                    printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                        unsafe { *z.offset(i as isize) } as i32)
                                };
                                break '__c6;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    break '__s5;
                }
                {
                    let mut z: *const i8 =
                        unsafe { sqlite3_value_text(p) } as *const i8;
                    let mut i: i32 = 0;
                    let mut c: i8 = 0 as i8;
                    {
                        i = 0;
                        '__b7: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                    i32 != 0 && c as i32 != '\'' as i32) {
                                break '__b7;
                            }
                            '__c7: loop { break '__c7; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c as i32 == 0 {
                        unsafe {
                            printf(c"\'%s\'".as_ptr() as *mut i8 as *const i8, z)
                        };
                    } else {
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                        while unsafe { *z } != 0 {
                            {
                                i = 0;
                                '__b9: loop {
                                    if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                            i32 != 0 && c as i32 != '\'' as i32) {
                                        break '__b9;
                                    }
                                    '__c9: loop { break '__c9; }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if c as i32 == '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                            if i != 0 {
                                unsafe {
                                    printf(c"%.*s".as_ptr() as *mut i8 as *const i8, i, z)
                                };
                                {
                                    let __n = i;
                                    let __p = &mut z;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                            }
                            if c as i32 == '\'' as i32 {
                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                continue;
                            }
                            if c as i32 == 0 { break; }
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    }
                    break '__s5;
                }
            }
            2 => {
                {
                    unsafe {
                        sqlite3_snprintf(50, &raw mut z[0 as usize] as *mut i8,
                            c"%!.20g".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_double(p) })
                    };
                    unsafe {
                        printf(c"%s".as_ptr() as *mut i8 as *const i8,
                            &raw mut z[0 as usize] as *mut i8)
                    };
                    break '__s5;
                }
                {
                    let n: i32 = unsafe { sqlite3_value_bytes(p) };
                    let mut z: *const u8 =
                        unsafe { sqlite3_value_blob(p) } as *const u8;
                    let mut i: i32 = 0;
                    unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                    {
                        i = 0;
                        '__b6: loop {
                            if !(i < n) { break '__b6; }
                            '__c6: loop {
                                unsafe {
                                    printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                        unsafe { *z.offset(i as isize) } as i32)
                                };
                                break '__c6;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    break '__s5;
                }
                {
                    let mut z: *const i8 =
                        unsafe { sqlite3_value_text(p) } as *const i8;
                    let mut i: i32 = 0;
                    let mut c: i8 = 0 as i8;
                    {
                        i = 0;
                        '__b7: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                    i32 != 0 && c as i32 != '\'' as i32) {
                                break '__b7;
                            }
                            '__c7: loop { break '__c7; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c as i32 == 0 {
                        unsafe {
                            printf(c"\'%s\'".as_ptr() as *mut i8 as *const i8, z)
                        };
                    } else {
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                        while unsafe { *z } != 0 {
                            {
                                i = 0;
                                '__b9: loop {
                                    if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                            i32 != 0 && c as i32 != '\'' as i32) {
                                        break '__b9;
                                    }
                                    '__c9: loop { break '__c9; }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if c as i32 == '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                            if i != 0 {
                                unsafe {
                                    printf(c"%.*s".as_ptr() as *mut i8 as *const i8, i, z)
                                };
                                {
                                    let __n = i;
                                    let __p = &mut z;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                            }
                            if c as i32 == '\'' as i32 {
                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                continue;
                            }
                            if c as i32 == 0 { break; }
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    }
                    break '__s5;
                }
            }
            4 => {
                {
                    let n: i32 = unsafe { sqlite3_value_bytes(p) };
                    let mut z: *const u8 =
                        unsafe { sqlite3_value_blob(p) } as *const u8;
                    let mut i: i32 = 0;
                    unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                    {
                        i = 0;
                        '__b6: loop {
                            if !(i < n) { break '__b6; }
                            '__c6: loop {
                                unsafe {
                                    printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                        unsafe { *z.offset(i as isize) } as i32)
                                };
                                break '__c6;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    break '__s5;
                }
                {
                    let mut z: *const i8 =
                        unsafe { sqlite3_value_text(p) } as *const i8;
                    let mut i: i32 = 0;
                    let mut c: i8 = 0 as i8;
                    {
                        i = 0;
                        '__b7: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                    i32 != 0 && c as i32 != '\'' as i32) {
                                break '__b7;
                            }
                            '__c7: loop { break '__c7; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c as i32 == 0 {
                        unsafe {
                            printf(c"\'%s\'".as_ptr() as *mut i8 as *const i8, z)
                        };
                    } else {
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                        while unsafe { *z } != 0 {
                            {
                                i = 0;
                                '__b9: loop {
                                    if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                            i32 != 0 && c as i32 != '\'' as i32) {
                                        break '__b9;
                                    }
                                    '__c9: loop { break '__c9; }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if c as i32 == '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                            if i != 0 {
                                unsafe {
                                    printf(c"%.*s".as_ptr() as *mut i8 as *const i8, i, z)
                                };
                                {
                                    let __n = i;
                                    let __p = &mut z;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                            }
                            if c as i32 == '\'' as i32 {
                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                continue;
                            }
                            if c as i32 == 0 { break; }
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    }
                    break '__s5;
                }
            }
            3 => {
                {
                    let mut z: *const i8 =
                        unsafe { sqlite3_value_text(p) } as *const i8;
                    let mut i: i32 = 0;
                    let mut c: i8 = 0 as i8;
                    {
                        i = 0;
                        '__b7: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                    i32 != 0 && c as i32 != '\'' as i32) {
                                break '__b7;
                            }
                            '__c7: loop { break '__c7; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c as i32 == 0 {
                        unsafe {
                            printf(c"\'%s\'".as_ptr() as *mut i8 as *const i8, z)
                        };
                    } else {
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                        while unsafe { *z } != 0 {
                            {
                                i = 0;
                                '__b9: loop {
                                    if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                            i32 != 0 && c as i32 != '\'' as i32) {
                                        break '__b9;
                                    }
                                    '__c9: loop { break '__c9; }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if c as i32 == '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                            if i != 0 {
                                unsafe {
                                    printf(c"%.*s".as_ptr() as *mut i8 as *const i8, i, z)
                                };
                                {
                                    let __n = i;
                                    let __p = &mut z;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                            }
                            if c as i32 == '\'' as i32 {
                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                continue;
                            }
                            if c as i32 == 0 { break; }
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                    }
                    break '__s5;
                }
            }
            _ => {}
        }
    }
}

///* This method is called to "rewind" the vtablog_cursor object back
///* to the first row of output.  This method is always called at least
///* once prior to any call to vtablogColumn() or vtablogRowid() or 
///* vtablogEof().
extern "C" fn vtablog_filter(cur: *mut Sqlite3VtabCursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut VtablogCursor = cur as *mut VtablogCursor;
    let p_tab: *const VtablogVtab =
        unsafe { (*cur).p_vtab } as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xFilter(cursor=%d):\n".as_ptr() as *mut i8 as
                *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, unsafe { (*p_cur).i_cursor })
    };
    unsafe { (*p_cur).i_rowid = 0 as Sqlite3Int64 };
    return 0;
}

///* Return an sqlite3_index_info operator name in static space.
///* The name is possibly overwritten on subsequent calls.
extern "C" fn vtablog_op_name(op: u8) -> *mut i8 {
    unsafe {
        let mut z_out: *mut i8 = core::ptr::null_mut();
        '__s10:
            {
            match op {
                2 => { z_out = c"EQ".as_ptr() as *mut i8; }
                4 => { z_out = c"GT".as_ptr() as *mut i8; }
                8 => { z_out = c"LE".as_ptr() as *mut i8; }
                16 => { z_out = c"LT".as_ptr() as *mut i8; }
                32 => { z_out = c"GE".as_ptr() as *mut i8; }
                64 => { z_out = c"MATCH".as_ptr() as *mut i8; }
                65 => { z_out = c"LIKE".as_ptr() as *mut i8; }
                66 => { z_out = c"GLOB".as_ptr() as *mut i8; }
                67 => { z_out = c"REGEXP".as_ptr() as *mut i8; }
                68 => { z_out = c"NE".as_ptr() as *mut i8; }
                69 => { z_out = c"ISNOT".as_ptr() as *mut i8; }
                70 => { z_out = c"ISNOTNULL".as_ptr() as *mut i8; }
                71 => { z_out = c"ISNULL".as_ptr() as *mut i8; }
                72 => { z_out = c"IS".as_ptr() as *mut i8; }
                73 => { z_out = c"LIMIT".as_ptr() as *mut i8; }
                74 => { z_out = c"OFFSET".as_ptr() as *mut i8; }
                150 => { z_out = c"FUNCTION".as_ptr() as *mut i8; }
                _ => {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 30]>() as i32,
                            &raw mut z_unknown[0 as usize] as *mut i8,
                            c"%d".as_ptr() as *mut i8 as *const i8, op as i32)
                    };
                    z_out = &raw mut z_unknown[0 as usize] as *mut i8;
                }
            }
        }
        return z_out;
    }
}

///* SQLite will invoke this method one or more times while planning a query
///* that uses the vtablog virtual table.  This routine needs to create
///* a query plan for each invocation and compute an estimated cost for that
///* plan.
extern "C" fn vtablog_best_index(tab: *mut Sqlite3Vtab,
    p: *mut Sqlite3IndexInfo) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    let mut i: i32 = 0;
    unsafe {
        printf(c"%s.%s.xBestIndex():\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    unsafe {
        printf(c"  colUsed: 0x%016llx\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).col_used })
    };
    unsafe {
        printf(c"  nConstraint: %d\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).n_constraint })
    };
    {
        i = 0;
        '__b11: loop {
            if !(i < unsafe { (*p).n_constraint }) { break '__b11; }
            '__c11: loop {
                let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
                let rc: i32 =
                    unsafe { sqlite3_vtab_rhs_value(p, i, &mut p_val) };
                unsafe {
                    printf(c"  constraint[%d]: col=%d termid=%d op=%s usabled=%d coll=%s rhs=".as_ptr()
                                as *mut i8 as *const i8, i,
                        unsafe {
                            (*unsafe { (*p).a_constraint.offset(i as isize) }).i_column
                        },
                        unsafe {
                            (*unsafe {
                                        (*p).a_constraint.offset(i as isize)
                                    }).i_term_offset
                        },
                        vtablog_op_name(unsafe {
                                (*unsafe { (*p).a_constraint.offset(i as isize) }).op
                            }),
                        unsafe {
                                (*unsafe { (*p).a_constraint.offset(i as isize) }).usable
                            } as i32, unsafe { sqlite3_vtab_collation(p, i) })
                };
                if rc == 0 {
                    vtablog_quote(p_val);
                    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                } else {
                    unsafe {
                        printf(c"N/A\n".as_ptr() as *mut i8 as *const i8)
                    };
                }
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        printf(c"  nOrderBy: %d\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).n_order_by })
    };
    if unsafe { (*p).n_order_by } != 0 {
        {
            i = 0;
            '__b12: loop {
                if !(i < unsafe { (*p).n_order_by }) { break '__b12; }
                '__c12: loop {
                    unsafe {
                        printf(c"  orderby[%d]: col=%d desc=%d\n".as_ptr() as
                                    *mut i8 as *const i8, i,
                            unsafe {
                                (*unsafe { (*p).a_order_by.offset(i as isize) }).i_column
                            },
                            unsafe {
                                    (*unsafe { (*p).a_order_by.offset(i as isize) }).desc
                                } as i32)
                    };
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_tab).i_consume_ob } != 0 {
            let n: i32 =
                unsafe {
                        (*unsafe { (*p).a_order_by.offset(0 as isize) }).i_column
                    } + 1;
            if unsafe {
                                (*unsafe { (*p).a_order_by.offset(0 as isize) }).desc
                            } != 0 && n == -unsafe { (*p_tab).i_consume_ob } ||
                    (unsafe {
                                        (*unsafe { (*p).a_order_by.offset(0 as isize) }).desc
                                    } == 0) as i32 != 0 && n == unsafe { (*p_tab).i_consume_ob }
                {
                unsafe { (*p).order_by_consumed = 1 };
            }
        }
    }
    unsafe { (*p).estimated_cost = 500 as f64 };
    unsafe { (*p).estimated_rows = 500 as Sqlite3Int64 };
    unsafe {
        printf(c"  idxNum=%d\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).idx_num })
    };
    unsafe { printf(c"  idxStr=NULL\n".as_ptr() as *mut i8 as *const i8) };
    unsafe {
        printf(c"  sqlite3_vtab_distinct()=%d\n".as_ptr() as *mut i8 as
                *const i8, unsafe { sqlite3_vtab_distinct(p) })
    };
    unsafe {
        printf(c"  orderByConsumed=%d\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).order_by_consumed })
    };
    unsafe {
        printf(c"  estimatedCost=%g\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).estimated_cost })
    };
    unsafe {
        printf(c"  estimatedRows=%lld\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p).estimated_rows })
    };
    return 0;
}

///* SQLite invokes this method to INSERT, UPDATE, or DELETE content from
///* the table. 
///*
///* This implementation does not actually make any changes to the table
///* content.  It merely logs the fact that the method was invoked
extern "C" fn vtablog_update(tab: *mut Sqlite3Vtab, argc: i32,
    argv: *mut *mut Sqlite3Value, p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    let mut i: i32 = 0;
    unsafe {
        printf(c"%s.%s.xUpdate():\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    unsafe { printf(c"  argc=%d\n".as_ptr() as *mut i8 as *const i8, argc) };
    {
        i = 0;
        '__b13: loop {
            if !(i < argc) { break '__b13; }
            '__c13: loop {
                unsafe {
                    printf(c"  argv[%d]=".as_ptr() as *mut i8 as *const i8, i)
                };
                vtablog_quote(unsafe { *argv.offset(i as isize) });
                unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                break '__c13;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

extern "C" fn vtablog_begin(tab: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xBegin()\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    return 0;
}

extern "C" fn vtablog_sync(tab: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xSync()\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    return 0;
}

extern "C" fn vtablog_commit(tab: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xCommit()\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    return 0;
}

extern "C" fn vtablog_rollback(tab: *mut Sqlite3Vtab) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xRollback()\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name })
    };
    return 0;
}

extern "C" fn vtablog_savepoint(tab: *mut Sqlite3Vtab, n_1: i32) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xSavepoint(%d)\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name }, n_1)
    };
    return 0;
}

extern "C" fn vtablog_release(tab: *mut Sqlite3Vtab, n_1: i32) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xRelease(%d)\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name }, n_1)
    };
    return 0;
}

extern "C" fn vtablog_rollback_to(tab: *mut Sqlite3Vtab, n_1: i32) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xRollbackTo(%d)\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name }, n_1)
    };
    return 0;
}

extern "C" fn vtablog_find_method(tab: *mut Sqlite3Vtab, n_arg_1: i32,
    z_name_1: *const i8,
    px_func_1:
        *mut unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> (), pp_arg_1: *mut *mut ()) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xFindMethod(nArg=%d, zName=%s)\n".as_ptr() as *mut i8
                as *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, n_arg_1, z_name_1)
    };
    return 0;
}

extern "C" fn vtablog_rename(tab: *mut Sqlite3Vtab, z_new_1: *const i8)
    -> i32 {
    let p_tab: *mut VtablogVtab = tab as *mut VtablogVtab;
    unsafe {
        printf(c"%s.%s.xRename(\'%s\')\n".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_tab).z_db }, unsafe { (*p_tab).z_name }, z_new_1)
    };
    unsafe { sqlite3_free(unsafe { (*p_tab).z_name } as *mut ()) };
    unsafe {
        (*p_tab).z_name =
            unsafe {
                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                    z_new_1)
            }
    };
    return 0;
}

/// Any table name that contains the text "shadow" is seen as a
///* shadow table.  Nothing else is.
extern "C" fn vtablog_shadow_name(z_name_1: *const i8) -> i32 {
    unsafe {
        printf(c"vtablog.xShadowName(\'%s\')\n".as_ptr() as *mut i8 as
                *const i8, z_name_1)
    };
    return (unsafe {
                    sqlite3_strglob(c"*shadow*".as_ptr() as *mut i8 as
                            *const i8, z_name_1)
                } == 0) as i32;
}

extern "C" fn vtablog_integrity(tab: *mut Sqlite3Vtab, z_schema_1: *const i8,
    z_tab_name_1: *const i8, m_flags_1: i32, pz_err_1: *mut *mut i8) -> i32 {
    let p_tab: *const VtablogVtab =
        tab as *mut VtablogVtab as *const VtablogVtab;
    unsafe {
        printf(c"%s.%s.xIntegrity(mFlags=0x%x)\n".as_ptr() as *mut i8 as
                *const i8, unsafe { (*p_tab).z_db },
            unsafe { (*p_tab).z_name }, m_flags_1)
    };
    return 0;
}

///* This following structure defines all the methods for the 
///* vtablog virtual table.
static mut vtablog_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 4,
        x_create: Some(vtablog_create),
        x_connect: Some(vtablog_connect),
        x_best_index: Some(vtablog_best_index),
        x_disconnect: Some(vtablog_disconnect),
        x_destroy: Some(vtablog_destroy),
        x_open: Some(vtablog_open),
        x_close: Some(vtablog_close),
        x_filter: Some(vtablog_filter),
        x_next: Some(vtablog_next),
        x_eof: Some(vtablog_eof),
        x_column: Some(vtablog_column),
        x_rowid: Some(vtablog_rowid),
        x_update: Some(vtablog_update),
        x_begin: Some(vtablog_begin),
        x_sync: Some(vtablog_sync),
        x_commit: Some(vtablog_commit),
        x_rollback: Some(vtablog_rollback),
        x_find_function: Some(vtablog_find_method),
        x_rename: Some(vtablog_rename),
        x_savepoint: Some(vtablog_savepoint),
        x_release: Some(vtablog_release),
        x_rollback_to: Some(vtablog_rollback_to),
        x_shadow_name: Some(vtablog_shadow_name),
        x_integrity: Some(vtablog_integrity),
    };

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtablog_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"vtablog".as_ptr() as *mut i8 as *const i8,
                    &raw mut vtablog_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
        return rc;
    }
}

static mut z_unknown: [i8; 30] = unsafe { core::mem::zeroed() };

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
    fn isspace(_c: i32)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn atoi(_: *const i8)
    -> i32;
    fn __builtin_unreachable()
    -> ();
}
