#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexConstraint, Sqlite3IndexInfo, Sqlite3Int64,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor, SqliteInt64,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct SeriesCursor {
    base: Sqlite3VtabCursor,
    i_o_base: Sqlite3Int64,
    i_o_term: Sqlite3Int64,
    i_o_step: Sqlite3Int64,
    i_base: Sqlite3Int64,
    i_term: Sqlite3Int64,
    i_step: Sqlite3Uint64,
    i_value: Sqlite3Int64,
    b_desc: u8,
    b_done: u8,
}

///* Computed the difference between two 64-bit signed integers using a
///* convoluted computation designed to work around the silly restriction
///* against signed integer overflow in C.
extern "C" fn span64(mut a: Sqlite3Int64, mut b: Sqlite3Int64)
    -> Sqlite3Uint64 {
    if !(a >= b) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"span64".as_ptr() as *const i8,
                c"series.c".as_ptr() as *mut i8 as *const i8, 186,
                c"a>=b".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return unsafe { *(&raw mut a as *mut Sqlite3Uint64) } -
            unsafe { *(&raw mut b as *mut Sqlite3Uint64) };
}

///* Add or substract an unsigned 64-bit integer from a signed 64-bit integer
///* and return the new signed 64-bit integer.
extern "C" fn add64(mut a: Sqlite3Int64, b: Sqlite3Uint64) -> Sqlite3Int64 {
    let mut x: Sqlite3Uint64 =
        unsafe { core::ptr::read(&raw mut a as *mut Sqlite3Uint64) };
    x += b;
    return unsafe { *(&raw mut x as *mut Sqlite3Int64) };
}

extern "C" fn sub64(mut a: Sqlite3Int64, b: Sqlite3Uint64) -> Sqlite3Int64 {
    let mut x: Sqlite3Uint64 =
        unsafe { core::ptr::read(&raw mut a as *mut Sqlite3Uint64) };
    x -= b;
    return unsafe { *(&raw mut x as *mut Sqlite3Int64) };
}

///* The seriesConnect() method is invoked to create a new
///* series_vtab that describes the generate_series virtual table.
///*
///* Think of this routine as the constructor for series_vtab objects.
///*
///* All this routine needs to do is:
///*
///*    (1) Allocate the series_vtab object and initialize all fields.
///*
///*    (2) Tell SQLite (via the sqlite3_declare_vtab() interface) what the
///*        result set of queries against generate_series will look like.
#[allow(unused_doc_comments)]
extern "C" fn series_connect(db: *mut Sqlite3, p_unused_1: *mut (),
    argc_unused_1: i32, argv_unused_1: *const *const i8,
    pp_vtab_1: *mut *mut Sqlite3Vtab, pz_err_unused_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut Sqlite3Vtab = core::ptr::null_mut();
    let mut rc: i32 = 0;

    /// Column numbers
    { let _ = p_unused_1; };

    /// Column numbers
    { let _ = argc_unused_1; };

    /// Column numbers
    { let _ = argv_unused_1; };

    /// Column numbers
    { let _ = pz_err_unused_1; };

    /// Column numbers
    (rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(value,start hidden,stop hidden,step hidden)".as_ptr()
                        as *mut i8 as *const i8)
        });
    if rc == 0 {
        p_new =
            {
                unsafe {
                    *pp_vtab_1 =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<Sqlite3Vtab>() as
                                        Sqlite3Uint64)
                            } as *mut Sqlite3Vtab
                };
                unsafe { *pp_vtab_1 }
            };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<Sqlite3Vtab>() as u64)
        };
        unsafe { sqlite3_vtab_config(db, 2) };
    }
    return rc;
}

///* This method is the destructor for series_cursor objects.
extern "C" fn series_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}

///* Constructor for a new series_cursor object.
extern "C" fn series_open(p_unused_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_cur: *mut SeriesCursor = core::ptr::null_mut();
    { let _ = p_unused_1; };
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<SeriesCursor>() as
                        Sqlite3Uint64)
            } as *mut SeriesCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<SeriesCursor>() as u64)
    };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}

///* Destructor for a series_cursor.
extern "C" fn series_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0;
}

///* Advance a series_cursor to its next row of output.
extern "C" fn series_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut SeriesCursor = cur as *mut SeriesCursor;
    if unsafe { (*p_cur).i_value } == unsafe { (*p_cur).i_term } {
        unsafe { (*p_cur).b_done = 1 as u8 };
    } else if unsafe { (*p_cur).b_desc } != 0 {
        unsafe {
            (*p_cur).i_value =
                sub64(unsafe { (*p_cur).i_value }, unsafe { (*p_cur).i_step })
        };
        if !(unsafe { (*p_cur).i_value } >= unsafe { (*p_cur).i_term }) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesNext".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 289,
                    c"pCur->iValue>=pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
    } else {
        unsafe {
            (*p_cur).i_value =
                add64(unsafe { (*p_cur).i_value }, unsafe { (*p_cur).i_step })
        };
        if !(unsafe { (*p_cur).i_value } <= unsafe { (*p_cur).i_term }) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesNext".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 292,
                    c"pCur->iValue<=pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
    }
    return 0;
}

///* Return values of columns for the row at which the series_cursor
///* is currently pointing.
extern "C" fn series_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const SeriesCursor =
        cur as *mut SeriesCursor as *const SeriesCursor;
    let mut x: Sqlite3Int64 = 0 as Sqlite3Int64;
    '__s0:
        {
        match i {
            1 => { x = unsafe { (*p_cur).i_o_base }; }
            2 => { x = unsafe { (*p_cur).i_o_term }; }
            3 => { x = unsafe { (*p_cur).i_o_step }; }
            _ => { x = unsafe { (*p_cur).i_value }; }
        }
    }
    unsafe { sqlite3_result_int64(ctx, x) };
    return 0;
}

///* The rowid is the same as the value.
extern "C" fn series_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const SeriesCursor =
        cur as *mut SeriesCursor as *const SeriesCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_value } };
    return 0;
}

///* Return TRUE if the cursor has been moved off of the last
///* row of output.
extern "C" fn series_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const SeriesCursor =
        cur as *mut SeriesCursor as *const SeriesCursor;
    return unsafe { (*p_cur).b_done } as i32;
}

///* Return the number of steps between pCur->iBase and pCur->iTerm if
///* the step width is pCur->iStep.
extern "C" fn series_steps(p_cur_1: &SeriesCursor) -> Sqlite3Uint64 {
    if (*p_cur_1).b_desc != 0 {
        if !((*p_cur_1).i_base >= (*p_cur_1).i_term) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesSteps".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 356,
                    c"pCur->iBase >= pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        return span64((*p_cur_1).i_base, (*p_cur_1).i_term) /
                (*p_cur_1).i_step;
    } else {
        if !((*p_cur_1).i_base <= (*p_cur_1).i_term) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesSteps".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 359,
                    c"pCur->iBase <= pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        return span64((*p_cur_1).i_term, (*p_cur_1).i_base) /
                (*p_cur_1).i_step;
    }
}

///* Case 1 (the most common case):
///* The standard math library is available so use ceil() and floor() from there.
extern "C" fn series_ceil(r: f64) -> f64 { return unsafe { ceil(r) }; }

extern "C" fn series_floor(r: f64) -> f64 { return unsafe { floor(r) }; }

/// Convert a floating point value to its closest integer.  Do so in
///* a way that avoids 'outside the range of representable values' warnings
///* from UBSAN.
extern "C" fn series_real_to_i64(r: f64) -> Sqlite3Int64 {
    if r < -9.223372036854775e18 {
        return 9223372036854775808u64 as Sqlite3Int64;
    }
    if r > 9.223372036854775e18 {
        return 9223372036854775807i64 as Sqlite3Int64;
    }
    return r as Sqlite3Int64;
}

///* This method is called to "rewind" the series_cursor object back
///* to the first row of output.  This method is always called at least
///* once prior to any call to seriesColumn() or seriesRowid() or
///* seriesEof().
///*
///* The query plan selected by seriesBestIndex is passed in the idxNum
///* parameter.  (idxStr is not used in this implementation.)  idxNum
///* is a bitmask showing which constraints are available:
///*
///*   0x0001:    start=VALUE
///*   0x0002:    stop=VALUE
///*   0x0004:    step=VALUE
///*   0x0008:    descending order
///*   0x0010:    ascending order
///*   0x0020:    LIMIT  VALUE
///*   0x0040:    OFFSET  VALUE
///*   0x0080:    value=VALUE
///*   0x0100:    value>=VALUE
///*   0x0200:    value>VALUE
///*   0x1000:    value<=VALUE
///*   0x2000:    value<VALUE
///*
///* This routine should initialize the cursor and position it so that it
///* is pointing at the first row, or pointing off the end of the table
///* (so that seriesEof() will return true) if the table is empty.
#[allow(unused_doc_comments)]
extern "C" fn series_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_unused_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    let mut p_cur: *mut SeriesCursor = core::ptr::null_mut();
    let mut i_arg: i32 = 0;
    /// Arguments used so far
    let mut i: i32 = 0;
    /// Loop counter
    let mut i_min: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Smallest allowed output value
    let mut i_max: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Largest allowed output value
    let mut i_limit: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// if >0, the value of the LIMIT
    let mut i_offset: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// if >0, the value of the OFFSET
    /// If any constraints have a NULL value, then return no rows.
    ///* See ticket https://sqlite.org/src/info/fac496b61722daf2
    /// Capture the three HIDDEN parameters to the virtual table and insert
    ///* default values for any parameters that are omitted.
    /// If there are constraints on the value column but there are
    ///* no constraints on  the start, stop, and step columns, then
    ///* initialize the default range to be the entire range of 64-bit signed
    ///* integers.  This range will contracted by the value column constraints
    ///* further below.
    /// Extract the LIMIT and OFFSET values, but do not apply them yet.
    ///* The range must first be constrained by the limits on value.
    /// Narrow the range of iMin and iMax (the minimum and maximum outputs)
    ///* based on equality and inequality constraints on the "value" column.
    /// value=X
    let mut r: f64 = 0.0;
    /// value>X (0x200) or value>=X (0x100)
    let mut r__1: f64 = 0.0;
    /// value<X (0x2000) or value<=X (0x1000)
    let mut r__2: f64 = 0.0;
    /// Try to reduce the range of values to be generated based on
    ///* constraints on the "value" column.
    let mut span: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut span__1: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    /// Adjust iTerm so that it is exactly the last value of the series.
    /// Transform the series generator to output values in the requested
    ///* order.
    let mut tmp: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s2:
            {
            match __state {
                0 => {
                    p_cur = p_vtab_cursor_1 as *mut SeriesCursor;
                    __state = 3;
                }
                2 => {
                    unsafe { (*p_cur).i_base = 0 as Sqlite3Int64 };
                    __state = 136;
                }
                3 => { i_arg = 0; __state = 4; }
                4 => { __state = 5; }
                5 => {
                    i_min = 9223372036854775808u64 as Sqlite3Int64;
                    __state = 6;
                }
                6 => {
                    i_max = 9223372036854775807i64 as Sqlite3Int64;
                    __state = 7;
                }
                7 => { i_limit = 0 as Sqlite3Int64; __state = 8; }
                8 => { i_offset = 0 as Sqlite3Int64; __state = 9; }
                9 => { { let _ = idx_str_unused_1; }; __state = 10; }
                10 => { i = 0; __state = 12; }
                11 => {
                    if idx_num_1 & 1 != 0 {
                        __state = 17;
                    } else { __state = 18; }
                }
                12 => { if i < argc { __state = 13; } else { __state = 11; } }
                13 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                            } == 5 {
                        __state = 15;
                    } else { __state = 14; }
                }
                14 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 12;
                }
                15 => { __state = 2; }
                16 => {
                    if idx_num_1 & 2 != 0 {
                        __state = 20;
                    } else { __state = 21; }
                }
                17 => {
                    unsafe {
                        (*p_cur).i_o_base =
                            unsafe {
                                sqlite3_value_int64(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            }
                    };
                    __state = 16;
                }
                18 => {
                    unsafe { (*p_cur).i_o_base = 0 as Sqlite3Int64 };
                    __state = 16;
                }
                19 => {
                    if idx_num_1 & 4 != 0 {
                        __state = 23;
                    } else { __state = 24; }
                }
                20 => {
                    unsafe {
                        (*p_cur).i_o_term =
                            unsafe {
                                sqlite3_value_int64(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            }
                    };
                    __state = 19;
                }
                21 => {
                    unsafe {
                        (*p_cur).i_o_term = 4294967295u32 as Sqlite3Int64
                    };
                    __state = 19;
                }
                22 => {
                    if idx_num_1 & 5 == 0 && idx_num_1 & 896 != 0 {
                        __state = 28;
                    } else { __state = 27; }
                }
                23 => {
                    unsafe {
                        (*p_cur).i_o_step =
                            unsafe {
                                sqlite3_value_int64(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            }
                    };
                    __state = 25;
                }
                24 => {
                    unsafe { (*p_cur).i_o_step = 1 as Sqlite3Int64 };
                    __state = 22;
                }
                25 => {
                    if unsafe { (*p_cur).i_o_step } == 0 as i64 {
                        __state = 26;
                    } else { __state = 22; }
                }
                26 => {
                    unsafe { (*p_cur).i_o_step = 1 as Sqlite3Int64 };
                    __state = 22;
                }
                27 => {
                    if idx_num_1 & 6 == 0 && idx_num_1 & 12416 != 0 {
                        __state = 30;
                    } else { __state = 29; }
                }
                28 => {
                    unsafe {
                        (*p_cur).i_o_base = 9223372036854775808u64 as Sqlite3Int64
                    };
                    __state = 27;
                }
                29 => {
                    unsafe { (*p_cur).i_base = unsafe { (*p_cur).i_o_base } };
                    __state = 31;
                }
                30 => {
                    unsafe {
                        (*p_cur).i_o_term = 9223372036854775807i64 as Sqlite3Int64
                    };
                    __state = 29;
                }
                31 => {
                    unsafe { (*p_cur).i_term = unsafe { (*p_cur).i_o_term } };
                    __state = 32;
                }
                32 => {
                    if unsafe { (*p_cur).i_o_step } > 0 as i64 {
                        __state = 34;
                    } else { __state = 35; }
                }
                33 => {
                    unsafe {
                        (*p_cur).b_desc =
                            (unsafe { (*p_cur).i_o_step } < 0 as i64) as u8
                    };
                    __state = 39;
                }
                34 => {
                    unsafe {
                        (*p_cur).i_step =
                            unsafe { (*p_cur).i_o_step } as Sqlite3Uint64
                    };
                    __state = 33;
                }
                35 => {
                    if unsafe { (*p_cur).i_o_step } >
                            9223372036854775808u64 as Sqlite3Int64 {
                        __state = 36;
                    } else { __state = 37; }
                }
                36 => {
                    unsafe {
                        (*p_cur).i_step =
                            -unsafe { (*p_cur).i_o_step } as Sqlite3Uint64
                    };
                    __state = 33;
                }
                37 => {
                    unsafe {
                        (*p_cur).i_step =
                            9223372036854775807i64 as Sqlite3Int64 as Sqlite3Uint64
                    };
                    __state = 38;
                }
                38 => {
                    {
                        let __p = unsafe { &mut (*p_cur).i_step };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 33;
                }
                39 => {
                    if unsafe { (*p_cur).b_desc } as i32 == 0 &&
                            unsafe { (*p_cur).i_base } > unsafe { (*p_cur).i_term } {
                        __state = 41;
                    } else { __state = 40; }
                }
                40 => {
                    if unsafe { (*p_cur).b_desc } as i32 != 0 &&
                            unsafe { (*p_cur).i_base } < unsafe { (*p_cur).i_term } {
                        __state = 43;
                    } else { __state = 42; }
                }
                41 => { __state = 2; }
                42 => {
                    if idx_num_1 & 32 != 0 {
                        __state = 45;
                    } else { __state = 44; }
                }
                43 => { __state = 2; }
                44 => {
                    if idx_num_1 & 13184 != 0 {
                        __state = 49;
                    } else { __state = 48; }
                }
                45 => {
                    i_limit =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 46;
                }
                46 => {
                    if idx_num_1 & 64 != 0 {
                        __state = 47;
                    } else { __state = 44; }
                }
                47 => {
                    i_offset =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 44;
                }
                48 => {
                    if unsafe { (*p_cur).b_desc } as i32 == 0 {
                        __state = 112;
                    } else { __state = 113; }
                }
                49 => {
                    if idx_num_1 & 128 != 0 {
                        __state = 51;
                    } else { __state = 52; }
                }
                50 => {
                    if unsafe { (*p_cur).b_desc } as i32 == 0 {
                        __state = 93;
                    } else { __state = 94; }
                }
                51 => {
                    if unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *argv.offset(i_arg as isize)
                                    })
                            } == 2 {
                        __state = 53;
                    } else { __state = 54; }
                }
                52 => {
                    if idx_num_1 & 768 != 0 {
                        __state = 59;
                    } else { __state = 58; }
                }
                53 => {
                    r =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 55;
                }
                54 => {
                    i_min =
                        {
                            i_max =
                                unsafe {
                                    sqlite3_value_int64(unsafe {
                                            *argv.offset({
                                                            let __p = &mut i_arg;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize)
                                        })
                                };
                            i_max
                        };
                    __state = 50;
                }
                55 => {
                    if r == series_ceil(r) &&
                                r >= 9223372036854775808u64 as Sqlite3Int64 as f64 &&
                            r <= 9223372036854775807i64 as Sqlite3Int64 as f64 {
                        __state = 56;
                    } else { __state = 57; }
                }
                56 => {
                    i_min = { i_max = series_real_to_i64(r); i_max };
                    __state = 50;
                }
                57 => { __state = 2; }
                58 => {
                    if idx_num_1 & 12288 != 0 {
                        __state = 76;
                    } else { __state = 75; }
                }
                59 => {
                    if unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *argv.offset(i_arg as isize)
                                    })
                            } == 2 {
                        __state = 60;
                    } else { __state = 61; }
                }
                60 => {
                    r__1 =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 62;
                }
                61 => {
                    i_min =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 71;
                }
                62 => {
                    if r__1 <= 9223372036854775808u64 as Sqlite3Int64 as f64 {
                        __state = 63;
                    } else { __state = 64; }
                }
                63 => {
                    i_min = 9223372036854775808u64 as Sqlite3Int64;
                    __state = 58;
                }
                64 => {
                    if r__1 > 9223372036854775807i64 as Sqlite3Int64 as f64 {
                        __state = 65;
                    } else { __state = 66; }
                }
                65 => { __state = 2; }
                66 => {
                    i_min = series_real_to_i64(series_ceil(r__1));
                    __state = 67;
                }
                67 => {
                    if idx_num_1 & 512 != 0 && r__1 == series_ceil(r__1) {
                        __state = 68;
                    } else { __state = 58; }
                }
                68 => {
                    if i_min == 9223372036854775807i64 as Sqlite3Int64 {
                        __state = 70;
                    } else { __state = 69; }
                }
                69 => {
                    { let __p = &mut i_min; let __t = *__p; *__p += 1; __t };
                    __state = 58;
                }
                70 => { __state = 2; }
                71 => {
                    if idx_num_1 & 512 != 0 {
                        __state = 72;
                    } else { __state = 58; }
                }
                72 => {
                    if i_min == 9223372036854775807i64 as Sqlite3Int64 {
                        __state = 73;
                    } else { __state = 74; }
                }
                73 => { __state = 2; }
                74 => {
                    { let __p = &mut i_min; let __t = *__p; *__p += 1; __t };
                    __state = 58;
                }
                75 => {
                    if i_min > i_max { __state = 92; } else { __state = 50; }
                }
                76 => {
                    if unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *argv.offset(i_arg as isize)
                                    })
                            } == 2 {
                        __state = 77;
                    } else { __state = 78; }
                }
                77 => {
                    r__2 =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 79;
                }
                78 => {
                    i_max =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 88;
                }
                79 => {
                    if r__2 >= 9223372036854775807i64 as Sqlite3Int64 as f64 {
                        __state = 80;
                    } else { __state = 81; }
                }
                80 => {
                    i_max = 9223372036854775807i64 as Sqlite3Int64;
                    __state = 75;
                }
                81 => {
                    if r__2 <= 9223372036854775808u64 as Sqlite3Int64 as f64 {
                        __state = 82;
                    } else { __state = 83; }
                }
                82 => { __state = 2; }
                83 => {
                    i_max = series_real_to_i64(series_floor(r__2));
                    __state = 84;
                }
                84 => {
                    if idx_num_1 & 8192 != 0 && r__2 == series_floor(r__2) {
                        __state = 85;
                    } else { __state = 75; }
                }
                85 => {
                    if i_max == 9223372036854775808u64 as Sqlite3Int64 {
                        __state = 87;
                    } else { __state = 86; }
                }
                86 => {
                    { let __p = &mut i_max; let __t = *__p; *__p -= 1; __t };
                    __state = 75;
                }
                87 => { __state = 2; }
                88 => {
                    if idx_num_1 & 8192 != 0 {
                        __state = 89;
                    } else { __state = 75; }
                }
                89 => {
                    if i_max == 9223372036854775808u64 as Sqlite3Int64 {
                        __state = 90;
                    } else { __state = 91; }
                }
                90 => { __state = 2; }
                91 => {
                    { let __p = &mut i_max; let __t = *__p; *__p -= 1; __t };
                    __state = 75;
                }
                92 => { __state = 2; }
                93 => {
                    if unsafe { (*p_cur).i_base } < i_min {
                        __state = 96;
                    } else { __state = 95; }
                }
                94 => {
                    if unsafe { (*p_cur).i_base } > i_max {
                        __state = 104;
                    } else { __state = 103; }
                }
                95 => {
                    if unsafe { (*p_cur).i_term } > i_max {
                        __state = 102;
                    } else { __state = 48; }
                }
                96 => {
                    span = span64(i_min, unsafe { (*p_cur).i_base });
                    __state = 97;
                }
                97 => {
                    unsafe {
                        (*p_cur).i_base =
                            add64(unsafe { (*p_cur).i_base },
                                span / unsafe { (*p_cur).i_step } *
                                    unsafe { (*p_cur).i_step })
                    };
                    __state = 98;
                }
                98 => {
                    if unsafe { (*p_cur).i_base } < i_min {
                        __state = 99;
                    } else { __state = 95; }
                }
                99 => {
                    if unsafe { (*p_cur).i_base } >
                            sub64(9223372036854775807i64 as Sqlite3Int64,
                                unsafe { (*p_cur).i_step }) {
                        __state = 101;
                    } else { __state = 100; }
                }
                100 => {
                    unsafe {
                        (*p_cur).i_base =
                            add64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step })
                    };
                    __state = 95;
                }
                101 => { __state = 2; }
                102 => { unsafe { (*p_cur).i_term = i_max }; __state = 48; }
                103 => {
                    if unsafe { (*p_cur).i_term } < i_min {
                        __state = 110;
                    } else { __state = 48; }
                }
                104 => {
                    span__1 = span64(unsafe { (*p_cur).i_base }, i_max);
                    __state = 105;
                }
                105 => {
                    unsafe {
                        (*p_cur).i_base =
                            sub64(unsafe { (*p_cur).i_base },
                                span__1 / unsafe { (*p_cur).i_step } *
                                    unsafe { (*p_cur).i_step })
                    };
                    __state = 106;
                }
                106 => {
                    if unsafe { (*p_cur).i_base } > i_max {
                        __state = 107;
                    } else { __state = 103; }
                }
                107 => {
                    if unsafe { (*p_cur).i_base } <
                            add64(9223372036854775808u64 as Sqlite3Int64,
                                unsafe { (*p_cur).i_step }) {
                        __state = 109;
                    } else { __state = 108; }
                }
                108 => {
                    unsafe {
                        (*p_cur).i_base =
                            sub64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step })
                    };
                    __state = 103;
                }
                109 => { __state = 2; }
                110 => { unsafe { (*p_cur).i_term = i_min }; __state = 48; }
                111 => {
                    if idx_num_1 & 8 != 0 &&
                                unsafe { (*p_cur).b_desc } as i32 == 0 ||
                            idx_num_1 & 16 != 0 &&
                                unsafe { (*p_cur).b_desc } as i32 != 0 {
                        __state = 119;
                    } else { __state = 118; }
                }
                112 => {
                    if unsafe { (*p_cur).i_base } > unsafe { (*p_cur).i_term } {
                        __state = 115;
                    } else { __state = 114; }
                }
                113 => {
                    if unsafe { (*p_cur).i_base } < unsafe { (*p_cur).i_term } {
                        __state = 117;
                    } else { __state = 116; }
                }
                114 => {
                    unsafe {
                        (*p_cur).i_term =
                            sub64(unsafe { (*p_cur).i_term },
                                span64(unsafe { (*p_cur).i_term },
                                        unsafe { (*p_cur).i_base }) % unsafe { (*p_cur).i_step })
                    };
                    __state = 111;
                }
                115 => { __state = 2; }
                116 => {
                    unsafe {
                        (*p_cur).i_term =
                            add64(unsafe { (*p_cur).i_term },
                                span64(unsafe { (*p_cur).i_base },
                                        unsafe { (*p_cur).i_term }) % unsafe { (*p_cur).i_step })
                    };
                    __state = 111;
                }
                117 => { __state = 2; }
                118 => {
                    if !(unsafe { (*p_cur).i_step } != 0 as u64) as i32 as i64
                            != 0 {
                        unsafe {
                            __assert_rtn(c"seriesFilter".as_ptr() as *const i8,
                                c"series.c".as_ptr() as *mut i8 as *const i8, 661,
                                c"pCur->iStep!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 123;
                }
                119 => { tmp = unsafe { (*p_cur).i_base }; __state = 120; }
                120 => {
                    unsafe { (*p_cur).i_base = unsafe { (*p_cur).i_term } };
                    __state = 121;
                }
                121 => { unsafe { (*p_cur).i_term = tmp }; __state = 122; }
                122 => {
                    unsafe {
                        (*p_cur).b_desc =
                            (unsafe { (*p_cur).b_desc } == 0) as i32 as u8
                    };
                    __state = 118;
                }
                123 => {
                    if idx_num_1 & 32 != 0 {
                        __state = 125;
                    } else { __state = 124; }
                }
                124 => {
                    unsafe { (*p_cur).i_value = unsafe { (*p_cur).i_base } };
                    __state = 133;
                }
                125 => {
                    if i_offset > 0 as i64 {
                        __state = 127;
                    } else { __state = 126; }
                }
                126 => {
                    if i_limit >= 0 as i64 &&
                            series_steps(unsafe { &*p_cur }) > i_limit as Sqlite3Uint64
                        {
                        __state = 132;
                    } else { __state = 124; }
                }
                127 => {
                    if series_steps(unsafe { &*p_cur }) <
                            i_offset as Sqlite3Uint64 {
                        __state = 128;
                    } else { __state = 129; }
                }
                128 => { __state = 2; }
                129 => {
                    if unsafe { (*p_cur).b_desc } != 0 {
                        __state = 130;
                    } else { __state = 131; }
                }
                130 => {
                    unsafe {
                        (*p_cur).i_base =
                            sub64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step } * i_offset as Sqlite3Uint64)
                    };
                    __state = 126;
                }
                131 => {
                    unsafe {
                        (*p_cur).i_base =
                            add64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step } * i_offset as Sqlite3Uint64)
                    };
                    __state = 126;
                }
                132 => {
                    unsafe {
                        (*p_cur).i_term =
                            add64(unsafe { (*p_cur).i_base },
                                (i_limit - 1 as Sqlite3Int64) as Sqlite3Uint64 *
                                    unsafe { (*p_cur).i_step })
                    };
                    __state = 124;
                }
                133 => {
                    unsafe { (*p_cur).b_done = 0 as u8 };
                    __state = 134;
                }
                134 => { return 0; }
                135 => { __state = 2; }
                136 => {
                    unsafe { (*p_cur).i_term = 0 as Sqlite3Int64 };
                    __state = 137;
                }
                137 => {
                    unsafe { (*p_cur).i_step = 1 as Sqlite3Uint64 };
                    __state = 138;
                }
                138 => {
                    unsafe { (*p_cur).b_desc = 0 as u8 };
                    __state = 139;
                }
                139 => {
                    unsafe { (*p_cur).b_done = 1 as u8 };
                    __state = 140;
                }
                140 => { return 0; }
                _ => {}
            }
        }
    }

    /// Arguments used so far
    /// Loop counter
    /// Smallest allowed output value
    /// Largest allowed output value
    /// if >0, the value of the LIMIT
    /// if >0, the value of the OFFSET
    /// If any constraints have a NULL value, then return no rows.
    ///* See ticket https://sqlite.org/src/info/fac496b61722daf2
    /// Capture the three HIDDEN parameters to the virtual table and insert
    ///* default values for any parameters that are omitted.
    /// If there are constraints on the value column but there are
    ///* no constraints on  the start, stop, and step columns, then
    ///* initialize the default range to be the entire range of 64-bit signed
    ///* integers.  This range will contracted by the value column constraints
    ///* further below.
    /// Extract the LIMIT and OFFSET values, but do not apply them yet.
    ///* The range must first be constrained by the limits on value.
    /// Narrow the range of iMin and iMax (the minimum and maximum outputs)
    ///* based on equality and inequality constraints on the "value" column.
    /// value=X
    /// value>X (0x200) or value>=X (0x100)
    /// value<X (0x2000) or value<=X (0x1000)
    /// Try to reduce the range of values to be generated based on
    ///* constraints on the "value" column.
    /// Adjust iTerm so that it is exactly the last value of the series.
    /// Transform the series generator to output values in the requested
    ///* order.
    /// Apply LIMIT and OFFSET constraints, if any
    unreachable!();
}

///* SQLite will invoke this method one or more times while planning a query
///* that uses the generate_series virtual table.  This routine needs to create
///* a query plan for each invocation and compute an estimated cost for that
///* plan.
///*
///* In this implementation idxNum is used to represent the
///* query plan.  idxStr is unused.
///*
///* The query plan is represented by bits in idxNum:
///*
///*   0x0001  start = $num
///*   0x0002  stop = $num
///*   0x0004  step = $num
///*   0x0008  output is in descending order
///*   0x0010  output is in ascending order
///*   0x0020  LIMIT $num
///*   0x0040  OFFSET $num
///*   0x0080  value = $num
///*   0x0100  value >= $num
///*   0x0200  value > $num
///*   0x1000  value <= $num
///*   0x2000  value < $num
///*
///* Only one of 0x0100 or 0x0200 will be returned.  Similarly, only
///* one of 0x1000 or 0x2000 will be returned.  If the 0x0080 is set, then
///* none of the 0xff00 bits will be set.
///*
///* The order of parameters passed to xFilter is as follows:
///*
///*    * The argument to start= if bit 0x0001 is in the idxNum mask
///*    * The argument to stop= if bit 0x0002 is in the idxNum mask
///*    * The argument to step= if bit 0x0004 is in the idxNum mask
///*    * The argument to LIMIT if bit 0x0020 is in the idxNum mask
///*    * The argument to OFFSET if bit 0x0040 is in the idxNum mask
///*    * The argument to value=, or value>= or value> if any of
///*      bits 0x0380 are in the idxNum mask
///*    * The argument to value<= or value< if either of bits 0x3000
///*      are in the mask
///*
#[allow(unused_doc_comments)]
extern "C" fn series_best_index(p_v_tab_1: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    /// Loop over constraints
    let mut idx_num: i32 = 0;
    /// The query plan bitmask
    let mut b_start_seen: i32 = 0;
    /// EQ constraint seen on the START column
    let mut unusable_mask: i32 = 0;
    /// Mask of unusable constraints
    let mut n_arg: i32 = 0;
    /// Number of arguments that seriesFilter() expects
    let mut a_idx: [i32; 7] = [0; 7];
    /// Constraints on start, stop, step, LIMIT, OFFSET,
    ///* and value.  aIdx[5] covers value=, value>=, and
    ///* value>,  aIdx[6] covers value<= and value<
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();

    /// This implementation assumes that the start, stop, and step columns
    ///* are the last three columns in the virtual table.
    if !(2 == 1 + 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                c"series.c".as_ptr() as *mut i8 as *const i8, 748,
                c"SERIES_COLUMN_STOP == SERIES_COLUMN_START+1".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(3 == 1 + 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                c"series.c".as_ptr() as *mut i8 as *const i8, 749,
                c"SERIES_COLUMN_STEP == SERIES_COLUMN_START+2".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    a_idx[0 as usize] =
        {
            a_idx[1 as usize] =
                {
                    a_idx[2 as usize] =
                        {
                            a_idx[3 as usize] =
                                {
                                    a_idx[4 as usize] =
                                        {
                                            a_idx[5 as usize] =
                                                { a_idx[6 as usize] = -1; a_idx[6 as usize] };
                                            a_idx[5 as usize]
                                        };
                                    a_idx[4 as usize]
                                };
                            a_idx[3 as usize]
                        };
                    a_idx[2 as usize]
                };
            a_idx[1 as usize]
        };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b3: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) { break '__b3; }
            '__c3: loop {
                let mut i_col: i32 = 0;
                /// 0 for start, 1 for stop, 2 for step
                let mut i_mask: i32 = 0;
                /// bitmask for those column
                let op: i32 = unsafe { (*p_constraint).op } as i32;
                if op >= 73 && op <= 74 {
                    if unsafe { (*p_constraint).usable } as i32 == 0
                        {} else if op == 73 {
                        a_idx[3 as usize] = i;
                        idx_num |= 32;
                    } else {
                        if !(op == 74) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                                    c"series.c".as_ptr() as *mut i8 as *const i8, 766,
                                    c"op==SQLITE_INDEX_CONSTRAINT_OFFSET".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        a_idx[4 as usize] = i;
                        idx_num |= 64;
                    }
                    break '__c3;
                }
                if (unsafe { (*p_constraint).i_column } as i32) < 1 {
                    if (unsafe { (*p_constraint).i_column } as i32 == 0 ||
                                unsafe { (*p_constraint).i_column } as i32 == -1) &&
                            unsafe { (*p_constraint).usable } != 0 {
                        '__s4:
                            {
                            match op {
                                2 => {
                                    {
                                        idx_num |= 128;
                                        idx_num &= !13056;
                                        a_idx[5 as usize] = i;
                                        a_idx[6 as usize] = -1;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 256;
                                        idx_num &= !512;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                72 => {
                                    {
                                        idx_num |= 128;
                                        idx_num &= !13056;
                                        a_idx[5 as usize] = i;
                                        a_idx[6 as usize] = -1;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 256;
                                        idx_num &= !512;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                32 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 256;
                                        idx_num &= !512;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                4 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                8 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                16 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    break '__c3;
                }
                i_col = unsafe { (*p_constraint).i_column } - 1;
                if !(i_col >= 0 && i_col <= 2) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                            c"series.c".as_ptr() as *mut i8 as *const i8, 828,
                            c"iCol>=0 && iCol<=2".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                i_mask = 1 << i_col;
                if i_col == 0 && op == 2 { b_start_seen = 1; }
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    unusable_mask |= i_mask;
                    break '__c3;
                } else if op == 2 {
                    idx_num |= i_mask;
                    a_idx[i_col as usize] = i;
                }
                break '__c3;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_constraint;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if a_idx[3 as usize] == 0 {

        /// Ignore OFFSET if LIMIT is omitted
        (idx_num &= !96);
        a_idx[4 as usize] = 0;
    }
    {
        i = 0;
        '__b5: loop {
            if !(i < 7) { break '__b5; }
            '__c5: loop {
                if { j = a_idx[i as usize]; j } >= 0 {
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                                    }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                                    }).omit = ((0 == 0) as i32 != 0 || i >= 3) as u8
                    };
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (b_start_seen == 0) as i32 != 0 {
        unsafe { sqlite3_free(unsafe { (*p_v_tab_1).z_err_msg } as *mut ()) };
        unsafe {
            (*p_v_tab_1).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"first argument to \"generate_series()\" missing or unusable".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        return 1;
    }
    if unusable_mask & !idx_num != 0 {

        /// The start, stop, and step columns are inputs.  Therefore if there
        ///* are unusable constraints on any of start, stop, or step then
        ///* this plan is unusable
        return 19;
    }
    if idx_num & 3 == 3 {

        /// Both start= and stop= boundaries are available.  This is the 
        ///* the preferred case
        unsafe {
            (*p_idx_info_1).estimated_cost =
                (2 - (idx_num & 4 != 0) as i32) as f64
        };

        /// Both start= and stop= boundaries are available.  This is the 
        ///* the preferred case
        unsafe { (*p_idx_info_1).estimated_rows = 1000 as Sqlite3Int64 };
        if unsafe { (*p_idx_info_1).n_order_by } >= 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 0 {
            if unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } != 0 {
                idx_num |= 8;
            } else { idx_num |= 16; }
            unsafe { (*p_idx_info_1).order_by_consumed = 1 };
        }
    } else if idx_num & 33 == 33 {

        /// We have start= and LIMIT
        unsafe { (*p_idx_info_1).estimated_rows = 2500 as Sqlite3Int64 };
    } else {

        /// If either boundary is missing, we have to generate a huge span
        ///* of numbers.  Make this case very expensive so that the query
        ///* planner will work hard to avoid it.
        unsafe {
            (*p_idx_info_1).estimated_rows = 2147483647 as Sqlite3Int64
        };
    }
    unsafe { (*p_idx_info_1).idx_num = idx_num };
    unsafe { (*p_idx_info_1).idx_flags = 2 };
    return 0;
}

///* This following structure defines all the methods for the 
///* generate_series virtual table.
static mut series_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: None,
        x_connect: Some(series_connect),
        x_best_index: Some(series_best_index),
        x_disconnect: Some(series_disconnect),
        x_destroy: None,
        x_open: Some(series_open),
        x_close: Some(series_close),
        x_filter: Some(series_filter),
        x_next: Some(series_next),
        x_eof: Some(series_eof),
        x_column: Some(series_column),
        x_rowid: Some(series_rowid),
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

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_series_init(db: *mut Sqlite3,
    pz_err_msg_1: *mut *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        if unsafe { sqlite3_libversion_number() } < 3008012 &&
                pz_err_msg_1 != core::ptr::null_mut() {
            unsafe {
                *pz_err_msg_1 =
                    unsafe {
                        sqlite3_mprintf(c"generate_series() requires SQLite 3.8.12 or later".as_ptr()
                                    as *mut i8 as *const i8)
                    }
            };
            return 1;
        }
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"generate_series".as_ptr() as *mut i8 as *const i8,
                    &raw mut series_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
        return rc;
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn ceil(_: f64)
    -> f64;
    fn floor(_: f64)
    -> f64;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
