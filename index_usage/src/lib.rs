//!* 2018-12-04
//!*
//!* The author disclaims copyright to this source code.  In place of
//!* a legal notice, here is a blessing:
//!*
//!*    May you do good and not evil.
//!*    May you find forgiveness for yourself and forgive others.
//!*    May you share freely, never taking more than you give.
//!*
//!************************************************************************
//!* 
//!* This file implements a utility program used to help determine which
//!* indexes in a database schema are used and unused, and how often specific
//!* indexes are used.
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};

extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        printf(c"Usage: %s [OPTIONS] DATABASE LOG\n\n".as_ptr() as *mut i8 as
                *const i8, argv0)
    };
    unsafe {
        printf(c"DATABASE is an SQLite database against which various statements\nhave been run.  The SQL text is stored in LOG.  LOG is an SQLite\ndatabase with this schema:\n\n    CREATE TABLE sqllog(sql TEXT);\n\nThis utility program analyzes statements contained in LOG and prints\na report showing how many times each index in DATABASE is used by the\nstatements in LOG.\n\nDATABASE only needs to contain the schema used by the statements in\nLOG. The content can be removed from DATABASE.\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe {
        printf(c"\nOPTIONS:\n\n    --progress N   Show a progress message after every N input rows\n    -q             Omit error message when parsing log entries\n    --using NAME   Print SQL statements that use index NAME\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe {
        printf(c"\nAnalysis will be done by SQLite version %s dated %.20s\ncheckin number %.40s. Different versions\nof SQLite might use different indexes.\n".as_ptr()
                    as *mut i8 as *const i8, unsafe { sqlite3_libversion() },
            unsafe { sqlite3_sourceid() },
            unsafe { unsafe { sqlite3_sourceid().offset(21 as isize) } })
    };
    unsafe { exit(1) };
}

#[allow(unused_doc_comments)]
extern "C" fn __main_inner(mut argc: i32, argv: *mut *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The main database
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        /// a query
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut n_err: i32 = 0;
        let mut rc: i32 = 0;
        let mut b_quiet: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut z_using: *const i8 = core::ptr::null();
        let mut p_incr_cnt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut n_row: i32 = 0;
        let mut i_progress: i32 = 0;
        let mut z: *const i8 = core::ptr::null();
        /// Open the LOG database
        /// Update the counts based on LOG
        let mut z_log: *const i8 = core::ptr::null();
        let mut p_s2: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut z_explain: *const i8 = core::ptr::null();
        let mut z1: *const i8 = core::ptr::null();
        let mut z2: *const i8 = core::ptr::null();
        let mut n: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s1:
                {
                match __state {
                    0 => { db = core::ptr::null_mut(); __state = 3; }
                    2 => {
                        unsafe { sqlite3_finalize(p_incr_cnt) };
                        __state = 121;
                    }
                    3 => { p_stmt = core::ptr::null_mut(); __state = 4; }
                    4 => { __state = 5; }
                    5 => { n_err = 0; __state = 6; }
                    6 => { __state = 7; }
                    7 => { b_quiet = 0; __state = 8; }
                    8 => { __state = 9; }
                    9 => { z_using = core::ptr::null(); __state = 10; }
                    10 => { p_incr_cnt = core::ptr::null_mut(); __state = 11; }
                    11 => { n_row = 0; __state = 12; }
                    12 => { i_progress = 0; __state = 13; }
                    13 => { i = { j = 1; j }; __state = 15; }
                    14 => { argc = j; __state = 43; }
                    15 => {
                        if i < argc { __state = 16; } else { __state = 14; }
                    }
                    16 => {
                        z = unsafe { *argv.offset(i as isize) } as *const i8;
                        __state = 18;
                    }
                    17 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 15;
                    }
                    18 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            __state = 19;
                        } else { __state = 20; }
                    }
                    19 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 21;
                    }
                    20 => { if j < i { __state = 42; } else { __state = 17; } }
                    21 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => {
                        if unsafe {
                                    strcmp(z, c"progress".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 25;
                        } else { __state = 24; }
                    }
                    23 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 22;
                    }
                    24 => {
                        if unsafe {
                                    strcmp(z, c"q".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 31;
                        } else { __state = 30; }
                    }
                    25 => {
                        if i + 1 < argc { __state = 27; } else { __state = 26; }
                    }
                    26 => {
                        unsafe {
                            printf(c"The --progress option requires an argument\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 29;
                    }
                    27 => {
                        i_progress =
                            unsafe {
                                    strtol(unsafe {
                                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                            } as *const i8, core::ptr::null_mut(), 0)
                                } as i32;
                        __state = 28;
                    }
                    28 => { __state = 17; }
                    29 => { unsafe { exit(0) }; __state = 24; }
                    30 => {
                        if unsafe {
                                    strcmp(z, c"using".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 34;
                        } else { __state = 33; }
                    }
                    31 => { b_quiet = 1; __state = 32; }
                    32 => { __state = 17; }
                    33 => {
                        if unsafe {
                                        strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe { strcmp(z, c"?".as_ptr() as *mut i8 as *const i8) }
                                    == 0 {
                            __state = 40;
                        } else { __state = 39; }
                    }
                    34 => {
                        if i + 1 < argc { __state = 36; } else { __state = 35; }
                    }
                    35 => {
                        unsafe {
                            printf(c"The --using option requires an argument\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 38;
                    }
                    36 => {
                        z_using =
                            unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8;
                        __state = 37;
                    }
                    37 => { __state = 17; }
                    38 => { unsafe { exit(0) }; __state = 33; }
                    39 => {
                        unsafe {
                            printf(c"Unknown command-line option: \"%s\"\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 41;
                    }
                    40 => {
                        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                        __state = 39;
                    }
                    41 => { unsafe { exit(0) }; __state = 17; }
                    42 => {
                        unsafe {
                            *argv.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = unsafe { *argv.offset(i as isize) }
                        };
                        __state = 17;
                    }
                    43 => {
                        if argc != 3 { __state = 45; } else { __state = 44; }
                    }
                    44 => {
                        rc =
                            unsafe {
                                sqlite3_open_v2(unsafe { *argv.offset(1 as isize) } as
                                        *const i8, &mut db, 1, core::ptr::null())
                            };
                        __state = 46;
                    }
                    45 => {
                        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                        __state = 44;
                    }
                    46 => {
                        if rc != 0 { __state = 48; } else { __state = 47; }
                    }
                    47 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db,
                                    c"SELECT * FROM sqlite_schema".as_ptr() as *mut i8 as
                                        *const i8, -1, &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 50;
                    }
                    48 => {
                        unsafe {
                            printf(c"Cannot open \"%s\" for reading: %s\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(1 as isize) },
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 49;
                    }
                    49 => { __state = 2; }
                    50 => {
                        if rc != 0 { __state = 52; } else { __state = 51; }
                    }
                    51 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 54; }
                    52 => {
                        unsafe {
                            printf(c"Cannot read the schema from \"%s\" - %s\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(1 as isize) },
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 53;
                    }
                    53 => { __state = 2; }
                    54 => { p_stmt = core::ptr::null_mut(); __state = 55; }
                    55 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db,
                                    c"CREATE TABLE temp.idxu(\n  tbl TEXT COLLATE nocase,\n  idx TEXT COLLATE nocase,\n  cnt INT,\n  PRIMARY KEY(idx)\n) WITHOUT ROWID;".as_ptr()
                                            as *mut i8 as *const i8, None, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        __state = 56;
                    }
                    56 => {
                        if rc != 0 { __state = 58; } else { __state = 57; }
                    }
                    57 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db,
                                    c"INSERT INTO temp.idxu(tbl,idx,cnt) SELECT tbl_name, name, 0 FROM sqlite_schema WHERE type=\'index\' AND sql IS NOT NULL".as_ptr()
                                            as *mut i8 as *const i8, None, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        __state = 60;
                    }
                    58 => {
                        unsafe {
                            printf(c"Cannot create the result table - %s\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 59;
                    }
                    59 => { __state = 2; }
                    60 => {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"ATTACH %Q AS log".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(2 as isize) })
                            };
                        __state = 61;
                    }
                    61 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db, z_sql as *const i8, None,
                                    core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        __state = 62;
                    }
                    62 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 63;
                    }
                    63 => {
                        if rc != 0 { __state = 65; } else { __state = 64; }
                    }
                    64 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db,
                                    c"SELECT sql, rowid FROM log.sqllog WHERE upper(substr(sql,1,5)) NOT IN (\'BEGIN\',\'COMMI\',\'ROLLB\',\'PRAGM\')".as_ptr()
                                            as *mut i8 as *const i8, -1, &mut p_stmt,
                                    core::ptr::null_mut())
                            };
                        __state = 67;
                    }
                    65 => {
                        unsafe {
                            printf(c"Cannot open the LOG database \"%s\" - %s\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(2 as isize) },
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 66;
                    }
                    66 => { __state = 2; }
                    67 => {
                        if rc != 0 { __state = 69; } else { __state = 68; }
                    }
                    68 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db,
                                    c"UPDATE temp.idxu SET cnt=cnt+1 WHERE idx=?1".as_ptr() as
                                            *mut i8 as *const i8, -1, &mut p_incr_cnt,
                                    core::ptr::null_mut())
                            };
                        __state = 71;
                    }
                    69 => {
                        unsafe {
                            printf(c"Cannot read the SQLLOG table in the LOG database \"%s\" - %s\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(2 as isize) },
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 70;
                    }
                    70 => { __state = 2; }
                    71 => {
                        if rc != 0 { __state = 73; } else { __state = 72; }
                    }
                    72 => {
                        if unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 76;
                        } else { __state = 75; }
                    }
                    73 => {
                        unsafe {
                            printf(c"Cannot prepare a statement to increment a counter for indexes used\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 74;
                    }
                    74 => { __state = 2; }
                    75 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 112;
                    }
                    76 => {
                        z_log =
                            unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
                        __state = 77;
                    }
                    77 => { __state = 78; }
                    78 => {
                        if z_log == core::ptr::null() {
                            __state = 80;
                        } else { __state = 79; }
                    }
                    79 => {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"EXPLAIN QUERY PLAN %s".as_ptr() as *mut i8
                                        as *const i8, z_log)
                            };
                        __state = 81;
                    }
                    80 => { __state = 72; }
                    81 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_s2,
                                    core::ptr::null_mut())
                            };
                        __state = 82;
                    }
                    82 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 83;
                    }
                    83 => {
                        if rc != 0 { __state = 85; } else { __state = 86; }
                    }
                    84 => { unsafe { sqlite3_finalize(p_s2) }; __state = 72; }
                    85 => {
                        if (b_quiet == 0) as i32 != 0 {
                            __state = 88;
                        } else { __state = 87; }
                    }
                    86 => {
                        { let __p = &mut n_row; let __t = *__p; *__p += 1; __t };
                        __state = 90;
                    }
                    87 => {
                        { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
                        __state = 84;
                    }
                    88 => {
                        unsafe {
                            printf(c"Cannot compile LOG entry %d (%s): %s\n".as_ptr() as
                                        *mut i8 as *const i8,
                                unsafe { sqlite3_column_int(p_stmt, 1) }, z_log,
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 89;
                    }
                    89 => { unsafe { fflush(__stdoutp) }; __state = 87; }
                    90 => {
                        if i_progress > 0 && n_row % i_progress == 0 {
                            __state = 92;
                        } else { __state = 91; }
                    }
                    91 => {
                        if unsafe { sqlite3_step(p_s2) } == 100 {
                            __state = 94;
                        } else { __state = 84; }
                    }
                    92 => {
                        unsafe {
                            printf(c"%d...\n".as_ptr() as *mut i8 as *const i8, n_row)
                        };
                        __state = 93;
                    }
                    93 => { unsafe { fflush(__stdoutp) }; __state = 91; }
                    94 => {
                        z_explain =
                            unsafe { sqlite3_column_text(p_s2, 3) } as *const i8;
                        __state = 95;
                    }
                    95 => { __state = 96; }
                    96 => { __state = 97; }
                    97 => {
                        z1 =
                            unsafe {
                                    strstr(z_explain,
                                        c" USING INDEX ".as_ptr() as *mut i8 as *const i8)
                                } as *const i8;
                        __state = 98;
                    }
                    98 => {
                        if z1 == core::ptr::null() {
                            __state = 100;
                        } else { __state = 99; }
                    }
                    99 => {
                        {
                            let __n = 13;
                            let __p = &mut z1;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 101;
                    }
                    100 => { __state = 91; }
                    101 => {
                        z2 = unsafe { z1.offset(1 as isize) };
                        __state = 103;
                    }
                    102 => {
                        n = unsafe { z2.offset_from(z1) } as i64 as i32;
                        __state = 106;
                    }
                    103 => {
                        if unsafe { *z2.offset(0 as isize) } != 0 &&
                                unsafe { *z2.offset(1 as isize) } as i32 != '(' as i32 {
                            __state = 104;
                        } else { __state = 102; }
                    }
                    104 => { __state = 105; }
                    105 => {
                        {
                            let __p = &mut z2;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 103;
                    }
                    106 => {
                        if !(z_using).is_null() &&
                                unsafe { sqlite3_strnicmp(z_using, z1, n) } == 0 {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    107 => {
                        unsafe { sqlite3_bind_text(p_incr_cnt, 1, z1, n, None) };
                        __state = 110;
                    }
                    108 => {
                        unsafe {
                            printf(c"Using %s:\n%s\n".as_ptr() as *mut i8 as *const i8,
                                z_using, z_log)
                        };
                        __state = 109;
                    }
                    109 => { unsafe { fflush(__stdoutp) }; __state = 107; }
                    110 => {
                        unsafe { sqlite3_step(p_incr_cnt) };
                        __state = 111;
                    }
                    111 => {
                        unsafe { sqlite3_reset(p_incr_cnt) };
                        __state = 91;
                    }
                    112 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db,
                                    c"SELECT tbl, idx, cnt,    (SELECT group_concat(name,\',\') FROM pragma_index_info(idx)) FROM temp.idxu, main.sqlite_schema WHERE temp.idxu.tbl=main.sqlite_schema.tbl_name   AND temp.idxu.idx=main.sqlite_schema.name ORDER BY cnt DESC, tbl, idx".as_ptr()
                                            as *mut i8 as *const i8, -1, &mut p_stmt,
                                    core::ptr::null_mut())
                            };
                        __state = 113;
                    }
                    113 => {
                        if rc != 0 { __state = 115; } else { __state = 114; }
                    }
                    114 => {
                        if unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 118;
                        } else { __state = 117; }
                    }
                    115 => {
                        unsafe {
                            printf(c"Cannot query the result table - %s\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 116;
                    }
                    116 => { __state = 2; }
                    117 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 119;
                    }
                    118 => {
                        unsafe {
                            printf(c"%10d %s on %s(%s)\n".as_ptr() as *mut i8 as
                                    *const i8, unsafe { sqlite3_column_int(p_stmt, 2) },
                                unsafe { sqlite3_column_text(p_stmt, 1) },
                                unsafe { sqlite3_column_text(p_stmt, 0) },
                                unsafe { sqlite3_column_text(p_stmt, 3) })
                        };
                        __state = 114;
                    }
                    119 => { p_stmt = core::ptr::null_mut(); __state = 120; }
                    120 => { __state = 2; }
                    121 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 122;
                    }
                    122 => { unsafe { sqlite3_close(db) }; __state = 123; }
                    123 => { return Err(n_err); }
                    _ => {}
                }
            }
        }

        /// The main database
        /// a query
        /// Open the LOG database
        /// Update the counts based on LOG
        /// printf("EXPLAIN: %s\n", zExplain);
        /// Generate the report
        unreachable!();
        return Ok(());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
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
    fn printf(_: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    fn fflush(_: *mut FILE)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn __builtin_unreachable()
    -> ();
    static mut __stdoutp: *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
