#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};

type DarwinSizeT = u64;

///* Global state information for this program.
#[repr(C)]
#[derive(Copy, Clone)]
struct GState {
    z_err: *mut i8,
    f: *mut FILE,
    sz_pg: i32,
    i_root: i32,
    i_col: i32,
    pgno: i32,
    a_page: *mut u8,
    a_stack: [*mut u8; 20],
    a_pgno: [i32; 20],
    n_stack: i32,
    b_trace: i32,
}

///* Write an error.
unsafe extern "C" fn ofst_error(p: &mut GState, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { sqlite3_free((*p).z_err as *mut ()) };
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    (*p).z_err = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
}

///* Write a trace message
unsafe extern "C" fn ofst_trace(p: *const GState, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    if unsafe { (*p).b_trace } != 0 {
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vprintf(z_format_1, ap) };
        ();
    }
}

///* Find the root page of the table and the column number of the column.
extern "C" fn ofst_root_and_column(p: *mut GState, z_file_1: *const i8,
    z_table_1: *const i8, z_column_1: *const i8) -> () {
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut z_col: *const i8 = core::ptr::null();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s1:
            {
            match __state {
                0 => { db = core::ptr::null_mut(); __state = 3; }
                2 => { unsafe { sqlite3_close(db) }; __state = 54; }
                3 => { p_stmt = core::ptr::null_mut(); __state = 4; }
                4 => { z_sql = core::ptr::null_mut(); __state = 5; }
                5 => { __state = 6; }
                6 => {
                    if !(unsafe { (*p).z_err }).is_null() {
                        __state = 8;
                    } else { __state = 7; }
                }
                7 => {
                    rc = unsafe { sqlite3_open(z_file_1, &mut db) };
                    __state = 9;
                }
                8 => { return; }
                9 => { if rc != 0 { __state = 11; } else { __state = 10; } }
                10 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"SELECT rootpage FROM sqlite_schema WHERE name=%Q".as_ptr()
                                        as *mut i8 as *const i8, z_table_1)
                        };
                    __state = 13;
                }
                11 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"cannot open database file \"%s\"".as_ptr() as *mut i8 as
                                *const i8, z_file_1)
                    };
                    __state = 12;
                }
                12 => { __state = 2; }
                13 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                                core::ptr::null_mut())
                        };
                    __state = 14;
                }
                14 => { if rc != 0 { __state = 16; } else { __state = 15; } }
                15 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 17;
                }
                16 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"%s: [%s]".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(db) }, z_sql)
                    };
                    __state = 15;
                }
                17 => {
                    if !(unsafe { (*p).z_err }).is_null() {
                        __state = 19;
                    } else { __state = 18; }
                }
                18 => {
                    if unsafe { sqlite3_step(p_stmt) } != 100 {
                        __state = 21;
                    } else { __state = 20; }
                }
                19 => { __state = 2; }
                20 => {
                    unsafe {
                        (*p).i_root = unsafe { sqlite3_column_int(p_stmt, 0) }
                    };
                    __state = 24;
                }
                21 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"cannot find table [%s]\n".as_ptr() as *mut i8 as
                                *const i8, z_table_1)
                    };
                    __state = 22;
                }
                22 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 23; }
                23 => { __state = 2; }
                24 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 25; }
                25 => { unsafe { (*p).i_col = -1 }; __state = 26; }
                26 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"PRAGMA table_info(%Q)".as_ptr() as *mut i8
                                    as *const i8, z_table_1)
                        };
                    __state = 27;
                }
                27 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                                core::ptr::null_mut())
                        };
                    __state = 28;
                }
                28 => { if rc != 0 { __state = 30; } else { __state = 29; } }
                29 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 31;
                }
                30 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"%s: [%s}".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(db) }, z_sql)
                    };
                    __state = 29;
                }
                31 => {
                    if !(unsafe { (*p).z_err }).is_null() {
                        __state = 33;
                    } else { __state = 32; }
                }
                32 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 35;
                    } else { __state = 34; }
                }
                33 => { __state = 2; }
                34 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 39; }
                35 => {
                    z_col =
                        unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
                    __state = 36;
                }
                36 => {
                    if unsafe { strlen(z_col) } == unsafe { strlen(z_column_1) }
                            &&
                            unsafe {
                                    sqlite3_strnicmp(z_col, z_column_1,
                                        unsafe { strlen(z_col) } as i32)
                                } == 0 {
                        __state = 37;
                    } else { __state = 32; }
                }
                37 => {
                    unsafe {
                        (*p).i_col = unsafe { sqlite3_column_int(p_stmt, 0) }
                    };
                    __state = 38;
                }
                38 => { __state = 34; }
                39 => {
                    if unsafe { (*p).i_col } < 0 {
                        __state = 41;
                    } else { __state = 40; }
                }
                40 => {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"PRAGMA page_size".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                    __state = 43;
                }
                41 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"no such column: %s.%s".as_ptr() as *mut i8 as *const i8,
                            z_table_1, z_column_1)
                    };
                    __state = 42;
                }
                42 => { __state = 2; }
                43 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                                core::ptr::null_mut())
                        };
                    __state = 44;
                }
                44 => { if rc != 0 { __state = 46; } else { __state = 45; } }
                45 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 47;
                }
                46 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"%s: [%s]".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(db) }, z_sql)
                    };
                    __state = 45;
                }
                47 => {
                    if !(unsafe { (*p).z_err }).is_null() {
                        __state = 49;
                    } else { __state = 48; }
                }
                48 => {
                    if unsafe { sqlite3_step(p_stmt) } != 100 {
                        __state = 51;
                    } else { __state = 52; }
                }
                49 => { __state = 2; }
                50 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 53; }
                51 => {
                    unsafe {
                        ofst_error(unsafe { &mut *p },
                            c"cannot find page size".as_ptr() as *mut i8 as *const i8)
                    };
                    __state = 50;
                }
                52 => {
                    unsafe {
                        (*p).sz_pg = unsafe { sqlite3_column_int(p_stmt, 0) }
                    };
                    __state = 50;
                }
                53 => { __state = 2; }
                54 => { return; }
                _ => {}
            }
        }
    }
}

///* Pop a page from the stack
extern "C" fn ofst_pop_page(p: &mut GState) -> () {
    if (*p).n_stack <= 0 { return; }
    { let __p = &mut (*p).n_stack; let __t = *__p; *__p -= 1; __t };
    unsafe { sqlite3_free((*p).a_stack[(*p).n_stack as usize] as *mut ()) };
    (*p).pgno = (*p).a_pgno[((*p).n_stack - 1) as usize];
    (*p).a_page = (*p).a_stack[((*p).n_stack - 1) as usize];
}

///* Push a new page onto the stack.
extern "C" fn ofst_push_page(p: *mut GState, pgno: i32) -> () {
    let mut p_page: *mut u8 = core::ptr::null_mut();
    let mut got: u64 = 0 as u64;
    if !(unsafe { (*p).z_err }).is_null() { return; }
    if unsafe { (*p).n_stack } as u64 >=
            core::mem::size_of::<[*mut u8; 20]>() as u64 /
                core::mem::size_of::<*mut u8>() as u64 {
        unsafe {
            ofst_error(unsafe { &mut *p },
                c"page stack overflow".as_ptr() as *mut i8 as *const i8)
        };
        return;
    }
    unsafe { (*p).a_pgno[unsafe { (*p).n_stack } as usize] = pgno };
    unsafe {
        (*p).a_stack[unsafe { (*p).n_stack } as usize] =
            {
                p_page =
                    unsafe { sqlite3_malloc(unsafe { (*p).sz_pg }) } as *mut u8;
                p_page
            }
    };
    if p_page == core::ptr::null_mut() {
        eprintln!("out of memory");
        unsafe { exit(1) };
    }
    {
        let __p = unsafe { &mut (*p).n_stack };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { (*p).a_page = p_page };
    unsafe { (*p).pgno = pgno };
    unsafe {
        fseek(unsafe { (*p).f }, ((pgno - 1) * unsafe { (*p).sz_pg }) as i64,
            0)
    };
    got =
        unsafe {
            fread(p_page as *mut (), 1 as u64, unsafe { (*p).sz_pg } as u64,
                unsafe { (*p).f })
        };
    if got != unsafe { (*p).sz_pg } as u64 {
        unsafe {
            ofst_error(unsafe { &mut *p },
                c"unable to read page %d".as_ptr() as *mut i8 as *const i8,
                pgno)
        };
        ofst_pop_page(unsafe { &mut *p });
    }
}

/// Read a two-byte integer at the given offset into the current page
extern "C" fn ofst2byte(p: &GState, ofst: i32) -> i32 {
    let x: i32 = unsafe { *(*p).a_page.offset(ofst as isize) } as i32;
    return (x << 8) +
            unsafe { *(*p).a_page.offset((ofst + 1) as isize) } as i32;
}

/// Read a four-byte integer at the given offset into the current page
extern "C" fn ofst4byte(p: &GState, ofst: i32) -> i32 {
    let mut x: i32 = unsafe { *(*p).a_page.offset(ofst as isize) } as i32;
    x = (x << 8) + unsafe { *(*p).a_page.offset((ofst + 1) as isize) } as i32;
    x = (x << 8) + unsafe { *(*p).a_page.offset((ofst + 2) as isize) } as i32;
    x = (x << 8) + unsafe { *(*p).a_page.offset((ofst + 3) as isize) } as i32;
    return x;
}

/// Read a variable-length integer.  Update the offset
extern "C" fn ofst_varint(p: &GState, p_ofst_1: &mut i32) -> Sqlite3Int64 {
    let mut x: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut a: *const u8 =
        unsafe { &raw mut *(*p).a_page.offset(*p_ofst_1 as isize) } as
            *const u8;
    let mut n: i32 = 0;
    while n < 8 && unsafe { *a.offset(0 as isize) } as i32 & 128 != 0 {
        x =
            (x << 7) +
                (unsafe { *a.offset(0 as isize) } as i32 & 127) as
                    Sqlite3Int64;
        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        {
            let __p = &mut a;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if n == 8 {
        x = (x << 8) + unsafe { *a.offset(0 as isize) } as Sqlite3Int64;
    } else {
        x = (x << 7) + unsafe { *a.offset(0 as isize) } as Sqlite3Int64;
    }
    *p_ofst_1 += n + 1;
    return x;
}

/// Return the absolute offset into a file for the given offset
///* into the current page
extern "C" fn ofst_in_file(p: &GState, ofst: i32) -> i32 {
    return (*p).sz_pg * ((*p).pgno - 1) + ofst;
}

/// Return the size (in bytes) of the data corresponding to the
///* given serial code
extern "C" fn ofst_serial_size(scode: i32) -> i32 {
    if scode < 5 { return scode; }
    if scode == 5 { return 6; }
    if scode < 8 { return 8; }
    if scode < 12 { return 0; }
    return (scode - 12) / 2;
}

/// Walk an interior btree page
extern "C" fn ofst_walk_interior_page(p: *mut GState) -> () {
    let mut n_cell: i32 = 0;
    let mut i: i32 = 0;
    let mut ofst: i32 = 0;
    let mut i_child: i32 = 0;
    n_cell = ofst2byte(unsafe { &*p }, 3);
    {
        i = 0;
        '__b3: loop {
            if !(i < n_cell) { break '__b3; }
            '__c3: loop {
                ofst = ofst2byte(unsafe { &*p }, 12 + i * 2);
                i_child = ofst4byte(unsafe { &*p }, ofst);
                unsafe { ofst_walk_page(p, i_child) };
                if !(unsafe { (*p).z_err }).is_null() { return; }
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { ofst_walk_page(p, ofst4byte(unsafe { &*p }, 8)) };
}

/// Walk a leaf btree page
extern "C" fn ofst_walk_leaf_page(p: *mut GState) -> () {
    let mut n_cell: i32 = 0;
    let mut i: i32 = 0;
    let mut ofst: i32 = 0;
    let mut n_payload: i32 = 0;
    let mut rowid: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_hdr: i32 = 0;
    let mut j: i32 = 0;
    let mut scode: i32 = 0;
    let mut sz: i32 = 0;
    let mut data_ofst: i32 = 0;
    let mut z_msg: [i8; 200] = [0; 200];
    n_cell = ofst2byte(unsafe { &*p }, 3);
    {
        i = 0;
        '__b4: loop {
            if !(i < n_cell) { break '__b4; }
            '__c4: loop {
                ofst = ofst2byte(unsafe { &*p }, 8 + i * 2);
                n_payload = ofst_varint(unsafe { &*p }, &mut ofst) as i32;
                rowid = ofst_varint(unsafe { &*p }, &mut ofst);
                if n_payload > unsafe { (*p).sz_pg } - 35 {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 200]>() as i32,
                            &raw mut z_msg[0 as usize] as *mut i8,
                            c"# overflow rowid %lld".as_ptr() as *mut i8 as *const i8,
                            rowid)
                    };
                    unsafe {
                        printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                            &raw mut z_msg[0 as usize] as *mut i8)
                    };
                    break '__c4;
                }
                data_ofst = ofst;
                n_hdr = ofst_varint(unsafe { &*p }, &mut ofst) as i32;
                data_ofst += n_hdr;
                {
                    j = 0;
                    '__b5: loop {
                        if !(j < unsafe { (*p).i_col }) { break '__b5; }
                        '__c5: loop {
                            scode = ofst_varint(unsafe { &*p }, &mut ofst) as i32;
                            data_ofst += ofst_serial_size(scode);
                            break '__c5;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                scode = ofst_varint(unsafe { &*p }, &mut ofst) as i32;
                sz = ofst_serial_size(scode);
                unsafe {
                    sqlite3_snprintf(core::mem::size_of::<[i8; 200]>() as i32,
                        &raw mut z_msg[0 as usize] as *mut i8,
                        c"rowid %12lld size %5d offset %8d".as_ptr() as *mut i8 as
                            *const i8, rowid, sz,
                        ofst_in_file(unsafe { &*p }, data_ofst))
                };
                unsafe {
                    printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                        &raw mut z_msg[0 as usize] as *mut i8)
                };
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

/// Forward reference
extern "C" fn ofst_walk_page(p: *mut GState, pgno: i32) -> () {
    if !(unsafe { (*p).z_err }).is_null() { return; }
    ofst_push_page(p, pgno);
    if !(unsafe { (*p).z_err }).is_null() { return; }
    if unsafe { *unsafe { (*p).a_page.offset(0 as isize) } } as i32 == 5 {
        ofst_walk_interior_page(p);
    } else if unsafe { *unsafe { (*p).a_page.offset(0 as isize) } } as i32 ==
            13 {
        ofst_walk_leaf_page(p);
    } else {
        unsafe {
            ofst_error(unsafe { &mut *p },
                c"page %d has a faulty type byte: %d".as_ptr() as *mut i8 as
                    *const i8, pgno,
                unsafe { *unsafe { (*p).a_page.offset(0 as isize) } } as i32)
        };
    }
    ofst_pop_page(unsafe { &mut *p });
}

extern "C" fn __main_inner(mut argc: i32, mut argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut g: GState = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut g as *mut (), 0,
                core::mem::size_of::<GState>() as u64)
        };
        if argc > 2 &&
                unsafe {
                        strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                            c"--trace".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
            g.b_trace = 1;
            { let __p = &mut argc; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut argv;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if argc != 4 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s DATABASE TABLE COLUMN\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { *argv })
            };
            unsafe { exit(1) };
        }
        ofst_root_and_column(&mut g,
            unsafe { *argv.offset(1 as isize) } as *const i8,
            unsafe { *argv.offset(2 as isize) } as *const i8,
            unsafe { *argv.offset(3 as isize) } as *const i8);
        if !(g.z_err).is_null() {
            unsafe {
                fprintf(__stderrp, c"%s\n".as_ptr() as *mut i8 as *const i8,
                    g.z_err)
            };
            unsafe { exit(1) };
        }
        unsafe {
            ofst_trace(&raw mut g as *const GState,
                c"# szPg = %d\n".as_ptr() as *mut i8 as *const i8, g.sz_pg)
        };
        unsafe {
            ofst_trace(&raw mut g as *const GState,
                c"# iRoot = %d\n".as_ptr() as *mut i8 as *const i8, g.i_root)
        };
        unsafe {
            ofst_trace(&raw mut g as *const GState,
                c"# iCol = %d\n".as_ptr() as *mut i8 as *const i8, g.i_col)
        };
        g.f =
            unsafe {
                fopen(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if g.f == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot open \"%s\"\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) })
            };
            unsafe { exit(1) };
        }
        ofst_walk_page(&mut g, g.i_root);
        if !(g.z_err).is_null() {
            unsafe {
                fprintf(__stderrp, c"%s\n".as_ptr() as *mut i8 as *const i8,
                    g.z_err)
            };
            unsafe { exit(1) };
        }
        return Ok(());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
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
    fn vprintf(_: *const i8, _: *mut i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn exit(_: i32)
    -> ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    static mut __stderrp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
