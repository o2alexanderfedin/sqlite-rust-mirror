#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct CsvReader {
    in_: *mut FILE,
    z: *mut i8,
    n: i64,
    n_alloc: i64,
    n_line: i64,
    b_not_first: i32,
    c_term: i32,
    i_in: u64,
    n_in: u64,
    z_in: *mut i8,
    z_err: [i8; 200],
}

/// Initialize a CsvReader object
extern "C" fn csv_reader_init(p: &mut CsvReader) -> () {
    (*p).in_ = core::ptr::null_mut();
    (*p).z = core::ptr::null_mut();
    (*p).n = 0 as i64;
    (*p).n_alloc = 0 as i64;
    (*p).n_line = 0 as i64;
    (*p).b_not_first = 0;
    (*p).n_in = 0 as u64;
    (*p).z_in = core::ptr::null_mut();
    (*p).z_err[0 as usize] = 0 as i8;
}

/// Close and reset a CsvReader object
extern "C" fn csv_reader_reset(p: *mut CsvReader) -> () {
    if !(unsafe { (*p).in_ }).is_null() {
        unsafe { fclose(unsafe { (*p).in_ }) };
        unsafe { sqlite3_free(unsafe { (*p).z_in } as *mut ()) };
    }
    unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
    csv_reader_init(unsafe { &mut *p });
}

/// Report an error on a CsvReader
unsafe extern "C" fn csv_errmsg(p: &mut CsvReader, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe {
        sqlite3_vsnprintf(200, &raw mut (*p).z_err[0 as usize] as *mut i8,
            z_format_1, ap)
    };
    ();
}

/// Open the file associated with a CsvReader
///* Return the number of errors.
extern "C" fn csv_reader_open(p: *mut CsvReader, z_filename_1: *const i8,
    z_data_1: *const i8) -> i32 {
    if !(z_filename_1).is_null() {
        unsafe {
            (*p).z_in =
                unsafe { sqlite3_malloc64(1024 as Sqlite3Uint64) } as *mut i8
        };
        if unsafe { (*p).z_in } == core::ptr::null_mut() {
            unsafe {
                csv_errmsg(unsafe { &mut *p },
                    c"out of memory".as_ptr() as *mut i8 as *const i8)
            };
            return 1;
        }
        unsafe {
            (*p).in_ =
                unsafe {
                    fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
                }
        };
        if unsafe { (*p).in_ } == core::ptr::null_mut() {
            unsafe { sqlite3_free(unsafe { (*p).z_in } as *mut ()) };
            csv_reader_reset(p);
            unsafe {
                csv_errmsg(unsafe { &mut *p },
                    c"cannot open \'%s\' for reading".as_ptr() as *mut i8 as
                        *const i8, z_filename_1)
            };
            return 1;
        }
    } else {
        if !(unsafe { (*p).in_ } == core::ptr::null_mut()) as i32 as i64 != 0
            {
            unsafe {
                __assert_rtn(c"csv_reader_open".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 145,
                    c"p->in==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p).z_in = z_data_1 as *mut i8 };
        unsafe { (*p).n_in = unsafe { strlen(z_data_1) } };
    }
    return 0;
}

/// The input buffer has overflowed.  Refill the input buffer, then
///* return the next character
#[allow(unused_doc_comments)]
extern "C" fn csv_getc_refill(p: &mut CsvReader) -> i32 {
    let mut got: u64 = 0 as u64;
    if !((*p).i_in >= (*p).n_in) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"csv_getc_refill".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 158,
                c"p->iIn>=p->nIn".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };

    /// Only called on an empty input buffer
    if !((*p).in_ != core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"csv_getc_refill".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 159,
                c"p->in!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };

    /// Only called if reading froma file
    (got =
        unsafe {
            fread((*p).z_in as *mut (), 1 as u64, 1024 as u64, (*p).in_)
        });
    if got == 0 as u64 { return -1; }
    (*p).n_in = got;
    (*p).i_in = 1 as u64;
    return unsafe { *(*p).z_in.offset(0 as isize) } as i32;
}

/// Return the next character of input.  Return EOF at end of input.
extern "C" fn csv_getc(p: *mut CsvReader) -> i32 {
    if unsafe { (*p).i_in } >= unsafe { (*p).n_in } {
        if unsafe { (*p).in_ } != core::ptr::null_mut() {
            return csv_getc_refill(unsafe { &mut *p });
        }
        return -1;
    }
    return unsafe {
                *(unsafe { (*p).z_in } as
                            *mut u8).add({
                                let __p = unsafe { &mut (*p).i_in };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as usize)
            } as i32;
}

/// Increase the size of p->z and append character c to the end. 
///* Return 0 on success and non-zero if there is an OOM error
extern "C" fn csv_resize_and_append(p: *mut CsvReader, c: i8) -> i32 {
    let mut z_new: *mut i8 = core::ptr::null_mut();
    let n_new: i64 = unsafe { (*p).n_alloc } * 2 as i64 + 100 as i64;
    z_new =
        unsafe {
                sqlite3_realloc64(unsafe { (*p).z } as *mut (),
                    n_new as Sqlite3Uint64)
            } as *mut i8;
    if !(z_new).is_null() {
        unsafe { (*p).z = z_new };
        unsafe { (*p).n_alloc = n_new };
        unsafe {
            *unsafe {
                        (*p).z.offset({
                                    let __p = unsafe { &mut (*p).n };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                    } = c
        };
        return 0;
    } else {
        unsafe {
            csv_errmsg(unsafe { &mut *p },
                c"out of memory".as_ptr() as *mut i8 as *const i8)
        };
        return 1;
    }
}

/// Append a single character to the CsvReader.z[] array.
///* Return 0 on success and non-zero if there is an OOM error
extern "C" fn csv_append(p: *mut CsvReader, c: i8) -> i32 {
    if unsafe { (*p).n } >= unsafe { (*p).n_alloc } - 1 as i64 {
        return csv_resize_and_append(p, c);
    }
    unsafe {
        *unsafe {
                    (*p).z.offset({
                                let __p = unsafe { &mut (*p).n };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize)
                } = c
    };
    return 0;
}

/// Read a single field of CSV text.  Compatible with rfc4180 and extended
///* with the option of having a separator other than ",".
///*
///*   +  Input comes from p->in.
///*   +  Store results in p->z of length p->n.  Space to hold p->z comes
///*      from sqlite3_malloc64().
///*   +  Keep track of the line number in p->nLine.
///*   +  Store the character that terminates the field in p->cTerm.  Store
///*      EOF on end-of-file.
///*
///* Return 0 at EOF or on OOM.  On EOF, the p->cTerm character will have
///* been set to EOF.
extern "C" fn csv_read_one_field(p: *mut CsvReader) -> *mut i8 {
    let mut c: i32 = 0;
    unsafe { (*p).n = 0 as i64 };
    c = csv_getc(p);
    if c == -1 { unsafe { (*p).c_term = -1 }; return core::ptr::null_mut(); }
    if c == '\"' as i32 {
        let mut pc: i32 = 0;
        let mut ppc: i32 = 0;
        let start_line: i64 = unsafe { (*p).n_line };
        pc = { ppc = 0; ppc };
        loop {
            c = csv_getc(p);
            if c <= '\"' as i32 || pc == '\"' as i32 {
                if c == '\n' as i32 {
                    {
                        let __p = unsafe { &mut (*p).n_line };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                if c == '\"' as i32 {
                    if pc == '\"' as i32 { pc = 0; continue; }
                }
                if c == ',' as i32 && pc == '\"' as i32 ||
                                c == '\n' as i32 && pc == '\"' as i32 ||
                            c == '\n' as i32 && pc == '\r' as i32 && ppc == '\"' as i32
                        || c == -1 && pc == '\"' as i32 {
                    '__b1: loop {
                        '__c1: loop {
                            {
                                let __p = unsafe { &mut (*p).n };
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            };
                            break '__c1;
                        }
                        if !(unsafe {
                                                *unsafe { (*p).z.offset(unsafe { (*p).n } as isize) }
                                            } as i32 != '\"' as i32) {
                            break '__b1;
                        }
                    }
                    unsafe { (*p).c_term = c as i8 as i32 };
                    break;
                }
                if pc == '\"' as i32 && c != '\r' as i32 {
                    unsafe {
                        csv_errmsg(unsafe { &mut *p },
                            c"line %d: unescaped %c character".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p).n_line }, '\"' as i32)
                    };
                    break;
                }
                if c == -1 {
                    unsafe {
                        csv_errmsg(unsafe { &mut *p },
                            c"line %d: unterminated %c-quoted field\n".as_ptr() as
                                    *mut i8 as *const i8, start_line, '\"' as i32)
                    };
                    unsafe { (*p).c_term = c as i8 as i32 };
                    break;
                }
            }
            if csv_append(p, c as i8) != 0 { return core::ptr::null_mut(); }
            ppc = pc;
            pc = c;
        }
    } else {
        if c & 255 == 239 && unsafe { (*p).b_not_first } == 0 {
            csv_append(p, c as i8);
            c = csv_getc(p);
            if c & 255 == 187 {
                csv_append(p, c as i8);
                c = csv_getc(p);
                if c & 255 == 191 {
                    unsafe { (*p).b_not_first = 1 };
                    unsafe { (*p).n = 0 as i64 };
                    return csv_read_one_field(p);
                }
            }
        }
        while c > ',' as i32 || c != -1 && c != ',' as i32 && c != '\n' as i32
            {
            if csv_append(p, c as i8) != 0 { return core::ptr::null_mut(); }
            c = csv_getc(p);
        }
        if c == '\n' as i32 {
            {
                let __p = unsafe { &mut (*p).n_line };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p).n } > 0 as i64 &&
                    unsafe {
                                *unsafe {
                                        (*p).z.offset((unsafe { (*p).n } - 1 as i64) as isize)
                                    }
                            } as i32 == '\r' as i32 {
                {
                    let __p = unsafe { &mut (*p).n };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
        }
        unsafe { (*p).c_term = c as i8 as i32 };
    }
    if !(unsafe { (*p).z } == core::ptr::null_mut() ||
                            unsafe { (*p).n } < unsafe { (*p).n_alloc }) as i32 as i64
            != 0 {
        unsafe {
            __assert_rtn(c"csv_read_one_field".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 287,
                c"p->z==0 || p->n<p->nAlloc".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).z }).is_null() {
        unsafe {
            *unsafe { (*p).z.offset(unsafe { (*p).n } as isize) } = 0 as i8
        };
    }
    unsafe { (*p).b_not_first = 1 };
    return unsafe { (*p).z };
}

/// An instance of the CSV virtual table
#[repr(C)]
#[derive(Copy, Clone)]
struct CsvTable {
    base: Sqlite3Vtab,
    z_filename: *mut i8,
    z_data: *mut i8,
    i_start: i64,
    n_col: i32,
    tst_flags: u32,
}

/// Skip leading whitespace.  Return a pointer to the first non-whitespace
///* character, or to the zero terminator if the string has only whitespace
extern "C" fn csv_skip_whitespace(mut z: *const i8) -> *const i8 {
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

/// Check to see if the string is of the form:  "TAG = VALUE" with optional
///* whitespace before and around tokens.  If it is, return a pointer to the
///* first character of VALUE.  If it is not, return NULL.
extern "C" fn csv_parameter(z_tag_1: *const i8, n_tag_1: i32,
    mut z: *const i8) -> *const i8 {
    z = csv_skip_whitespace(z);
    if unsafe { strncmp(z_tag_1, z, n_tag_1 as u64) } != 0 {
        return core::ptr::null();
    }
    z = csv_skip_whitespace(unsafe { z.offset(n_tag_1 as isize) });
    if unsafe { *z.offset(0 as isize) } as i32 != '=' as i32 {
        return core::ptr::null();
    }
    return csv_skip_whitespace(unsafe { z.offset(1 as isize) });
}

/// Remove trailing whitespace from the end of string z[]
extern "C" fn csv_trim_whitespace(z: *mut i8) -> () {
    let mut n: u64 = unsafe { strlen(z as *const i8) };
    while n > 0 as u64 &&
            unsafe { isspace(unsafe { *z.add(n as usize) } as u8 as i32) } !=
                0 {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    unsafe { *z.add(n as usize) = 0 as i8 };
}

/// Dequote the string
extern "C" fn csv_dequote(z: *mut i8) -> () {
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
        '__b5: loop {
            if !(i < n - 1 as u64) { break '__b5; }
            '__c5: loop {
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
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z.offset(j as isize) = 0 as i8 };
}

/// Decode a parameter that requires a dequoted string.
///*
///* Return 1 if the parameter is seen, or 0 if not.  1 is returned
///* even if there is an error.  If an error occurs, then an error message
///* is left in p->zErr.  If there are no errors, p->zErr[0]==0.
extern "C" fn csv_string_parameter(p: *mut CsvReader, z_param_1: *const i8,
    z_arg_1: *const i8, pz_val_1: &mut *mut i8) -> i32 {
    let mut z_value: *const i8 = core::ptr::null();
    z_value =
        csv_parameter(z_param_1, unsafe { strlen(z_param_1) } as i32,
            z_arg_1);
    if z_value == core::ptr::null() { return 0; }
    unsafe { (*p).z_err[0 as usize] = 0 as i8 };
    if !(*pz_val_1).is_null() {
        unsafe {
            csv_errmsg(unsafe { &mut *p },
                c"more than one \'%s\' parameter".as_ptr() as *mut i8 as
                    *const i8, z_param_1)
        };
        return 1;
    }
    *pz_val_1 =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_value)
        };
    if *pz_val_1 == core::ptr::null_mut() {
        unsafe {
            csv_errmsg(unsafe { &mut *p },
                c"out of memory".as_ptr() as *mut i8 as *const i8)
        };
        return 1;
    }
    csv_trim_whitespace(*pz_val_1);
    csv_dequote(*pz_val_1);
    return 1;
}

///* This method is the destructor fo a CsvTable object.
extern "C" fn csvtab_disconnect(p_vtab: *mut Sqlite3Vtab) -> i32 {
    let p: *mut CsvTable = p_vtab as *mut CsvTable;
    unsafe { sqlite3_free(unsafe { (*p).z_filename } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).z_data } as *mut ()) };
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}

/// Return 0 if the argument is false and 1 if it is true.  Return -1 if
///* we cannot really tell.
extern "C" fn csv_boolean(z: *const i8) -> i32 {
    if unsafe { sqlite3_stricmp(c"yes".as_ptr() as *mut i8 as *const i8, z) }
                        == 0 ||
                    unsafe {
                            sqlite3_stricmp(c"on".as_ptr() as *mut i8 as *const i8, z)
                        } == 0 ||
                unsafe {
                        sqlite3_stricmp(c"true".as_ptr() as *mut i8 as *const i8, z)
                    } == 0 ||
            unsafe { *z.offset(0 as isize) } as i32 == '1' as i32 &&
                unsafe { *z.offset(1 as isize) } as i32 == 0 {
        return 1;
    }
    if unsafe { sqlite3_stricmp(c"no".as_ptr() as *mut i8 as *const i8, z) }
                        == 0 ||
                    unsafe {
                            sqlite3_stricmp(c"off".as_ptr() as *mut i8 as *const i8, z)
                        } == 0 ||
                unsafe {
                        sqlite3_stricmp(c"false".as_ptr() as *mut i8 as *const i8,
                            z)
                    } == 0 ||
            unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                unsafe { *z.offset(1 as isize) } as i32 == 0 {
        return 0;
    }
    return -1;
}

/// Check to see if the string is of the form:  "TAG = BOOLEAN" or just "TAG".
///* If it is, set *pValue to be the value of the boolean ("true" if there is
///* not "= BOOLEAN" component) and return non-zero.  If the input string
///* does not begin with TAG, return zero.
extern "C" fn csv_boolean_parameter(z_tag_1: *const i8, n_tag_1: i32,
    mut z: *const i8, p_value_1: &mut i32) -> i32 {
    let mut b: i32 = 0;
    z = csv_skip_whitespace(z);
    if unsafe { strncmp(z_tag_1, z, n_tag_1 as u64) } != 0 { return 0; }
    z = csv_skip_whitespace(unsafe { z.offset(n_tag_1 as isize) });
    if unsafe { *z.offset(0 as isize) } as i32 == 0 {
        *p_value_1 = 1;
        return 1;
    }
    if unsafe { *z.offset(0 as isize) } as i32 != '=' as i32 { return 0; }
    z = csv_skip_whitespace(unsafe { z.offset(1 as isize) });
    b = csv_boolean(z);
    if b >= 0 { *p_value_1 = b; return 1; }
    return 0;
}

///* Parameters:
///*    filename=FILENAME          Name of file containing CSV content
///*    data=TEXT                  Direct CSV content.
///*    schema=SCHEMA              Alternative CSV schema.
///*    header=YES|NO              First row of CSV defines the names of
///*                               columns if "yes".  Default "no".
///*    columns=N                  Assume the CSV file contains N columns.
///*
///* Only available if compiled with SQLITE_TEST:
///*    
///*    testflags=N                Bitmask of test flags.  Optional
///*
///* If schema= is omitted, then the columns are named "c0", "c1", "c2",
///* and so forth.  If columns=N is omitted, then the file is opened and
///* the number of columns in the first row is counted to determine the
///* column count.  If header=YES, then the first row is skipped.
#[allow(unused_doc_comments)]
extern "C" fn csvtab_connect(db: *mut Sqlite3, p_aux: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab: *mut *mut Sqlite3Vtab,
    pz_err: *mut *mut i8) -> i32 {
    unsafe {
        let mut p_new: *mut CsvTable = core::ptr::null_mut();
        /// The CsvTable object to construct
        let mut b_header: i32 = 0;
        /// header= flags.  -1 means not seen yet
        let mut rc: i32 = 0;
        /// Result code from this routine
        let mut i: u64 = 0 as u64;
        let mut j: u64 = 0 as u64;
        /// Loop counters
        let mut b: i32 = 0;
        /// Value of a boolean parameter
        let mut n_col: i32 = 0;
        /// Value of the columns= parameter
        let mut s_rdr: CsvReader = unsafe { core::mem::zeroed() };
        let mut az_p_value: [*mut i8; 3] = [core::ptr::null_mut(); 3];
        /// Parameter values
        let mut z: *const i8 = core::ptr::null();
        let mut z_value: *const i8 = core::ptr::null();
        let mut p_str: *mut Sqlite3Str = core::ptr::null_mut();
        let mut z_sep: *mut i8 = core::ptr::null_mut();
        let mut i_col: i32 = 0;
        let mut z__1: *mut i8 = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s7:
                {
                match __state {
                    0 => { p_new = core::ptr::null_mut(); __state = 4; }
                    2 => { rc = 7; __state = 114; }
                    3 => {
                        if !(p_new).is_null() {
                            __state = 117;
                        } else { __state = 116; }
                    }
                    4 => { b_header = -1; __state = 5; }
                    5 => { rc = 0; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { n_col = -99; __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => {
                        if !(core::mem::size_of::<[*mut i8; 3]>() as u64 ==
                                                core::mem::size_of::<[*const i8; 3]>() as u64) as i32 as i64
                                != 0 {
                            unsafe {
                                __assert_rtn(c"csvtabConnect".as_ptr() as *const i8,
                                    c"csv.c".as_ptr() as *mut i8 as *const i8, 517,
                                    c"sizeof(azPValue)==sizeof(azParam)".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 13;
                    }
                    13 => {
                        unsafe {
                            memset(&raw mut s_rdr as *mut (), 0,
                                core::mem::size_of::<CsvReader>() as u64)
                        };
                        __state = 14;
                    }
                    14 => {
                        unsafe {
                            memset(&raw mut az_p_value[0 as usize] as *mut *mut i8 as
                                    *mut (), 0, core::mem::size_of::<[*mut i8; 3]>() as u64)
                        };
                        __state = 15;
                    }
                    15 => { i = 3 as u64; __state = 17; }
                    16 => {
                        if (az_p_value[0 as usize] == core::ptr::null_mut()) as i32
                                == (az_p_value[1 as usize] == core::ptr::null_mut()) as i32
                            {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    17 => {
                        if i < argc as u64 { __state = 18; } else { __state = 16; }
                    }
                    18 => {
                        z = unsafe { *argv.add(i as usize) };
                        __state = 20;
                    }
                    19 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 17;
                    }
                    20 => { __state = 21; }
                    21 => { j = 0 as u64; __state = 23; }
                    22 => {
                        if j <
                                core::mem::size_of::<[*const i8; 3]>() as u64 /
                                    core::mem::size_of::<*const i8>() as u64 {
                            __state = 27;
                        } else { __state = 28; }
                    }
                    23 => {
                        if j <
                                core::mem::size_of::<[*const i8; 3]>() as u64 /
                                    core::mem::size_of::<*const i8>() as u64 {
                            __state = 24;
                        } else { __state = 22; }
                    }
                    24 => {
                        if csv_string_parameter(&mut s_rdr, az_param[j as usize], z,
                                    &mut az_p_value[j as usize]) != 0 {
                            __state = 26;
                        } else { __state = 25; }
                    }
                    25 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 23;
                    }
                    26 => { __state = 22; }
                    27 => {
                        if s_rdr.z_err[0 as usize] != 0 {
                            __state = 29;
                        } else { __state = 19; }
                    }
                    28 => {
                        if csv_boolean_parameter(c"header".as_ptr() as *mut i8 as
                                        *const i8, 6, z, &mut b) != 0 {
                            __state = 30;
                        } else { __state = 31; }
                    }
                    29 => { __state = 3; }
                    30 => {
                        if b_header >= 0 { __state = 33; } else { __state = 32; }
                    }
                    31 => {
                        if {
                                    z_value =
                                        csv_parameter(c"columns".as_ptr() as *mut i8 as *const i8,
                                            7, z);
                                    z_value
                                } != core::ptr::null() {
                            __state = 35;
                        } else { __state = 36; }
                    }
                    32 => { b_header = b; __state = 19; }
                    33 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"more than one \'header\' parameter".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 34;
                    }
                    34 => { __state = 3; }
                    35 => {
                        if n_col > 0 { __state = 38; } else { __state = 37; }
                    }
                    36 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"bad parameter: \'%s\'".as_ptr() as *mut i8 as *const i8,
                                z)
                        };
                        __state = 46;
                    }
                    37 => { n_col = unsafe { atoi(z_value) }; __state = 40; }
                    38 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"more than one \'columns\' parameter".as_ptr() as *mut i8
                                    as *const i8)
                        };
                        __state = 39;
                    }
                    39 => { __state = 3; }
                    40 => {
                        if n_col <= 0 { __state = 41; } else { __state = 42; }
                    }
                    41 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"column= value must be positive".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 43;
                    }
                    42 => {
                        if n_col > unsafe { sqlite3_limit(db, 2, -1) } {
                            __state = 44;
                        } else { __state = 19; }
                    }
                    43 => { __state = 3; }
                    44 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"column= value too big, max %d".as_ptr() as *mut i8 as
                                    *const i8, unsafe { sqlite3_limit(db, 2, -1) })
                        };
                        __state = 45;
                    }
                    45 => { __state = 3; }
                    46 => { __state = 3; }
                    47 => {
                        if (n_col <= 0 || b_header == 1) &&
                                csv_reader_open(&mut s_rdr,
                                        az_p_value[0 as usize] as *const i8,
                                        az_p_value[1 as usize] as *const i8) != 0 {
                            __state = 51;
                        } else { __state = 50; }
                    }
                    48 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"must specify either filename= or data= but not both".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 49;
                    }
                    49 => { __state = 3; }
                    50 => {
                        p_new =
                            unsafe {
                                    sqlite3_malloc64(core::mem::size_of::<CsvTable>() as
                                            Sqlite3Uint64)
                                } as *mut CsvTable;
                        __state = 52;
                    }
                    51 => { __state = 3; }
                    52 => {
                        unsafe { *pp_vtab = p_new as *mut Sqlite3Vtab };
                        __state = 53;
                    }
                    53 => {
                        if p_new == core::ptr::null_mut() {
                            __state = 55;
                        } else { __state = 54; }
                    }
                    54 => {
                        unsafe {
                            memset(p_new as *mut (), 0,
                                core::mem::size_of::<CsvTable>() as u64)
                        };
                        __state = 56;
                    }
                    55 => { __state = 2; }
                    56 => {
                        if az_p_value[2 as usize] == core::ptr::null_mut() {
                            __state = 58;
                        } else { __state = 59; }
                    }
                    57 => {
                        unsafe { (*p_new).z_filename = az_p_value[0 as usize] };
                        __state = 94;
                    }
                    58 => {
                        p_str = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        __state = 60;
                    }
                    59 => {
                        if n_col < 0 { __state = 90; } else { __state = 91; }
                    }
                    60 => { z_sep = c"".as_ptr() as *mut i8; __state = 61; }
                    61 => { i_col = 0; __state = 62; }
                    62 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"CREATE TABLE x(".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 63;
                    }
                    63 => {
                        if n_col < 0 && b_header < 1 {
                            __state = 65;
                        } else { __state = 64; }
                    }
                    64 => {
                        if n_col > 0 && b_header < 1 {
                            __state = 70;
                        } else { __state = 71; }
                    }
                    65 => { n_col = 0; __state = 66; }
                    66 => { csv_read_one_field(&mut s_rdr); __state = 68; }
                    67 => {
                        if s_rdr.c_term == ',' as i32 {
                            __state = 66;
                        } else { __state = 64; }
                    }
                    68 => {
                        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                        __state = 67;
                    }
                    69 => { unsafe { (*p_new).n_col = n_col }; __state = 86; }
                    70 => { i_col = 0; __state = 72; }
                    71 => {
                        z__1 = csv_read_one_field(&mut s_rdr);
                        __state = 78;
                    }
                    72 => {
                        if i_col < n_col { __state = 73; } else { __state = 69; }
                    }
                    73 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"%sc%d TEXT".as_ptr() as *mut i8 as *const i8, z_sep,
                                i_col)
                        };
                        __state = 75;
                    }
                    74 => {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                        __state = 72;
                    }
                    75 => { z_sep = c",".as_ptr() as *mut i8; __state = 74; }
                    76 => {
                        if n_col < 0 { __state = 82; } else { __state = 83; }
                    }
                    77 => {
                        if s_rdr.c_term == ',' as i32 {
                            __state = 71;
                        } else { __state = 76; }
                    }
                    78 => {
                        if n_col > 0 && i_col < n_col || n_col < 0 && b_header != 0
                            {
                            __state = 79;
                        } else { __state = 77; }
                    }
                    79 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"%s\"%w\" TEXT".as_ptr() as *mut i8 as *const i8, z_sep,
                                z__1)
                        };
                        __state = 80;
                    }
                    80 => { z_sep = c",".as_ptr() as *mut i8; __state = 81; }
                    81 => {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                        __state = 77;
                    }
                    82 => { n_col = i_col; __state = 69; }
                    83 => {
                        if i_col < n_col { __state = 84; } else { __state = 69; }
                    }
                    84 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"%sc%d TEXT".as_ptr() as *mut i8 as *const i8, z_sep,
                                { let __p = &mut i_col; *__p += 1; *__p })
                        };
                        __state = 85;
                    }
                    85 => { z_sep = c",".as_ptr() as *mut i8; __state = 83; }
                    86 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c")".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 87;
                    }
                    87 => {
                        az_p_value[2 as usize] =
                            unsafe { sqlite3_str_finish(p_str) };
                        __state = 88;
                    }
                    88 => {
                        if az_p_value[2 as usize] == core::ptr::null_mut() {
                            __state = 89;
                        } else { __state = 57; }
                    }
                    89 => { __state = 2; }
                    90 => { csv_read_one_field(&mut s_rdr); __state = 93; }
                    91 => { unsafe { (*p_new).n_col = n_col }; __state = 57; }
                    92 => {
                        if s_rdr.c_term == ',' as i32 {
                            __state = 90;
                        } else { __state = 57; }
                    }
                    93 => {
                        {
                            let __p = unsafe { &mut (*p_new).n_col };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 92;
                    }
                    94 => {
                        az_p_value[0 as usize] = core::ptr::null_mut();
                        __state = 95;
                    }
                    95 => {
                        unsafe { (*p_new).z_data = az_p_value[1 as usize] };
                        __state = 96;
                    }
                    96 => {
                        az_p_value[1 as usize] = core::ptr::null_mut();
                        __state = 97;
                    }
                    97 => {
                        if b_header != 1 { __state = 99; } else { __state = 100; }
                    }
                    98 => { csv_reader_reset(&mut s_rdr); __state = 103; }
                    99 => {
                        unsafe { (*p_new).i_start = 0 as i64 };
                        __state = 98;
                    }
                    100 => {
                        if !(unsafe { (*p_new).z_data }).is_null() {
                            __state = 101;
                        } else { __state = 102; }
                    }
                    101 => {
                        unsafe { (*p_new).i_start = s_rdr.i_in as i32 as i64 };
                        __state = 98;
                    }
                    102 => {
                        unsafe {
                            (*p_new).i_start =
                                (unsafe { ftell(s_rdr.in_) } as u64 - s_rdr.n_in +
                                            s_rdr.i_in) as i32 as i64
                        };
                        __state = 98;
                    }
                    103 => {
                        rc =
                            unsafe {
                                sqlite3_declare_vtab(db,
                                    az_p_value[2 as usize] as *const i8)
                            };
                        __state = 104;
                    }
                    104 => {
                        if rc != 0 { __state = 106; } else { __state = 105; }
                    }
                    105 => { i = 0 as u64; __state = 109; }
                    106 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"bad schema: \'%s\' - %s".as_ptr() as *mut i8 as *const i8,
                                az_p_value[2 as usize], unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 107;
                    }
                    107 => { __state = 3; }
                    108 => {
                        unsafe { sqlite3_vtab_config(db, 3) };
                        __state = 112;
                    }
                    109 => {
                        if i <
                                core::mem::size_of::<[*mut i8; 3]>() as u64 /
                                    core::mem::size_of::<*mut i8>() as u64 {
                            __state = 110;
                        } else { __state = 108; }
                    }
                    110 => {
                        unsafe { sqlite3_free(az_p_value[i as usize] as *mut ()) };
                        __state = 111;
                    }
                    111 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 109;
                    }
                    112 => { return 0; }
                    113 => { __state = 2; }
                    114 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"out of memory".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 115;
                    }
                    115 => { __state = 3; }
                    116 => { i = 0 as u64; __state = 119; }
                    117 => {
                        csvtab_disconnect(unsafe { &mut (*p_new).base });
                        __state = 116;
                    }
                    118 => {
                        if s_rdr.z_err[0 as usize] != 0 {
                            __state = 123;
                        } else { __state = 122; }
                    }
                    119 => {
                        if i <
                                core::mem::size_of::<[*mut i8; 3]>() as u64 /
                                    core::mem::size_of::<*mut i8>() as u64 {
                            __state = 120;
                        } else { __state = 118; }
                    }
                    120 => {
                        unsafe { sqlite3_free(az_p_value[i as usize] as *mut ()) };
                        __state = 121;
                    }
                    121 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 119;
                    }
                    122 => { csv_reader_reset(&mut s_rdr); __state = 125; }
                    123 => {
                        unsafe { sqlite3_free(unsafe { *pz_err } as *mut ()) };
                        __state = 124;
                    }
                    124 => {
                        unsafe {
                            *pz_err =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        &raw mut s_rdr.z_err[0 as usize] as *mut i8)
                                }
                        };
                        __state = 122;
                    }
                    125 => {
                        if rc == 0 { __state = 127; } else { __state = 126; }
                    }
                    126 => { return rc; }
                    127 => { rc = 1; __state = 126; }
                    _ => {}
                }
            }
        }

        /// The CsvTable object to construct
        /// header= flags.  -1 means not seen yet
        /// Result code from this routine
        /// Loop counters
        /// Value of a boolean parameter
        /// Value of the columns= parameter
        /// A CSV file reader used to store an error
        ///* message and/or to count the number of columns
        /// Parameter values
        /// Rationale for DIRECTONLY:
        ///* An attacker who controls a database schema could use this vtab
        ///* to exfiltrate sensitive data from other files in the filesystem.
        ///* And, recommended practice is to put all CSV virtual tables in the
        ///* TEMP namespace, so they should still be usable from within TEMP
        ///* views, so there shouldn't be a serious loss of functionality by
        ///* prohibiting the use of this vtab from persistent triggers and views.
        unreachable!();
    }
}

/// Forward references to the various virtual table methods implemented
///* in this file.
extern "C" fn csvtab_create(db: *mut Sqlite3, p_aux: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab: *mut *mut Sqlite3Vtab,
    pz_err: *mut *mut i8) -> i32 {
    return csvtab_connect(db, p_aux, argc, argv, pp_vtab, pz_err);
}

///* Only a forward full table scan is supported.  xBestIndex is mostly
///* a no-op.  If CSVTEST_FIDX is set, then the presence of equality
///* constraints lowers the estimated cost, which is fiction, but is useful
///* for testing certain kinds of virtual table behavior.
extern "C" fn csvtab_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info: *mut Sqlite3IndexInfo) -> i32 {
    unsafe { (*p_idx_info).estimated_cost = 1000000 as f64 };
    return 0;
}

/// A cursor for the CSV virtual table
#[repr(C)]
#[derive(Copy, Clone)]
struct CsvCursor {
    base: Sqlite3VtabCursor,
    rdr: CsvReader,
    az_val: *mut *mut i8,
    a_len: *mut i64,
    i_rowid: Sqlite3Int64,
}

/// Transfer error message text from a reader into a CsvTable
extern "C" fn csv_xfer_error(p_tab_1: &mut CsvTable, p_rdr_1: &mut CsvReader)
    -> () {
    unsafe { sqlite3_free((*p_tab_1).base.z_err_msg as *mut ()) };
    (*p_tab_1).base.z_err_msg =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                &raw mut (*p_rdr_1).z_err[0 as usize] as *mut i8)
        };
}

///* Constructor for a new CsvTable cursor object.
extern "C" fn csvtab_open(p: *mut Sqlite3Vtab,
    pp_cursor: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p_tab: *mut CsvTable = p as *mut CsvTable;
    let mut p_cur: *mut CsvCursor = core::ptr::null_mut();
    let mut n_byte: Sqlite3Int64 = unsafe { (*p_tab).n_col } as Sqlite3Int64;
    if !(unsafe { (*p_tab).n_col } < 32768) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"csvtabOpen".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 717,
                c"pTab->nCol<32768".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n_byte =
        (core::mem::size_of::<CsvCursor>() as u64 +
                (core::mem::size_of::<*mut i8>() as u64 +
                        core::mem::size_of::<i64>() as u64) * n_byte as u64) as
            Sqlite3Int64;
    p_cur =
        unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
            *mut CsvCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe { memset(p_cur as *mut (), 0, n_byte as u64) };
    unsafe {
        (*p_cur).az_val =
            unsafe { &raw mut *p_cur.offset(1 as isize) } as *mut *mut i8
    };
    unsafe {
        (*p_cur).a_len =
            unsafe {
                    &raw mut *unsafe {
                                (*p_cur).az_val.offset(unsafe { (*p_tab).n_col } as isize)
                            }
                } as *mut i64
    };
    unsafe { *pp_cursor = unsafe { &mut (*p_cur).base } };
    if csv_reader_open(unsafe { &mut (*p_cur).rdr },
                unsafe { (*p_tab).z_filename } as *const i8,
                unsafe { (*p_tab).z_data } as *const i8) != 0 {
        csv_xfer_error(unsafe { &mut *p_tab }, unsafe { &mut (*p_cur).rdr });
        return 1;
    }
    return 0;
}

///* Reset the current row content held by a CsvCursor.
extern "C" fn csvtab_cursor_row_reset(p_cur_1: &mut CsvCursor) -> () {
    let p_tab: *const CsvTable =
        (*p_cur_1).base.p_vtab as *mut CsvTable as *const CsvTable;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b8: loop {
            if !(i < unsafe { (*p_tab).n_col }) { break '__b8; }
            '__c8: loop {
                unsafe {
                    sqlite3_free(unsafe {
                                *(*p_cur_1).az_val.offset(i as isize)
                            } as *mut ())
                };
                unsafe {
                    *(*p_cur_1).az_val.offset(i as isize) =
                        core::ptr::null_mut()
                };
                unsafe { *(*p_cur_1).a_len.offset(i as isize) = 0 as i64 };
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Destructor for a CsvCursor.
extern "C" fn csvtab_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut CsvCursor = cur as *mut CsvCursor;
    csvtab_cursor_row_reset(unsafe { &mut *p_cur });
    csv_reader_reset(unsafe { &mut (*p_cur).rdr });
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0;
}

///* Advance a CsvCursor to its next row of input.
///* Set the EOF marker if we reach the end of input.
extern "C" fn csvtab_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut CsvCursor = cur as *mut CsvCursor;
    let p_tab: *mut CsvTable = unsafe { (*cur).p_vtab } as *mut CsvTable;
    let mut i: i32 = 0;
    let mut z: *const i8 = core::ptr::null();
    '__b9: loop {
        '__c9: loop {
            z = csv_read_one_field(unsafe { &mut (*p_cur).rdr });
            if z == core::ptr::null_mut() { break '__b9; }
            if i < unsafe { (*p_tab).n_col } {
                if unsafe { *unsafe { (*p_cur).a_len.offset(i as isize) } } <
                        unsafe { (*p_cur).rdr.n } + 1 as i64 {
                    let z_new: *mut i8 =
                        unsafe {
                                sqlite3_realloc64(unsafe {
                                            *unsafe { (*p_cur).az_val.offset(i as isize) }
                                        } as *mut (),
                                    (unsafe { (*p_cur).rdr.n } + 1 as i64) as Sqlite3Uint64)
                            } as *mut i8;
                    if z_new == core::ptr::null_mut() {
                        unsafe {
                            csv_errmsg(unsafe { &mut (*p_cur).rdr },
                                c"out of memory".as_ptr() as *mut i8 as *const i8)
                        };
                        csv_xfer_error(unsafe { &mut *p_tab },
                            unsafe { &mut (*p_cur).rdr });
                        break '__b9;
                    }
                    unsafe {
                        *unsafe { (*p_cur).az_val.offset(i as isize) } = z_new
                    };
                    unsafe {
                        *unsafe { (*p_cur).a_len.offset(i as isize) } =
                            unsafe { (*p_cur).rdr.n } + 1 as i64
                    };
                }
                unsafe {
                    memcpy(unsafe {
                                *unsafe { (*p_cur).az_val.offset(i as isize) }
                            } as *mut (), z as *const (),
                        (unsafe { (*p_cur).rdr.n } + 1 as i64) as u64)
                };
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
            break '__c9;
        }
        if !(unsafe { (*p_cur).rdr.c_term } == ',' as i32) { break '__b9; }
    }
    if z == core::ptr::null_mut() && i == 0 {
        unsafe { (*p_cur).i_rowid = -1 as Sqlite3Int64 };
    } else {
        {
            let __p = unsafe { &mut (*p_cur).i_rowid };
            let __t = *__p;
            *__p += 1;
            __t
        };
        while i < unsafe { (*p_tab).n_col } {
            unsafe {
                sqlite3_free(unsafe {
                            *unsafe { (*p_cur).az_val.offset(i as isize) }
                        } as *mut ())
            };
            unsafe {
                *unsafe { (*p_cur).az_val.offset(i as isize) } =
                    core::ptr::null_mut()
            };
            unsafe {
                *unsafe { (*p_cur).a_len.offset(i as isize) } = 0 as i64
            };
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Only a full table scan is supported.  So xFilter simply rewinds to
///* the beginning.
extern "C" fn csvtab_filter(p_vtab_cursor: *mut Sqlite3VtabCursor,
    idx_num: i32, idx_str: *const i8, argc: i32, argv: *mut *mut Sqlite3Value)
    -> i32 {
    let p_cur: *mut CsvCursor = p_vtab_cursor as *mut CsvCursor;
    let p_tab: *const CsvTable =
        unsafe { (*p_vtab_cursor).p_vtab } as *mut CsvTable as
            *const CsvTable;
    unsafe { (*p_cur).i_rowid = 0 as Sqlite3Int64 };
    if csv_append(unsafe { &mut (*p_cur).rdr }, 0 as i8) != 0 { return 7; }
    if unsafe { (*p_cur).rdr.in_ } == core::ptr::null_mut() {
        if !(unsafe { (*p_cur).rdr.z_in } == unsafe { (*p_tab).z_data }) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"csvtabFilter".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 830,
                    c"pCur->rdr.zIn==pTab->zData".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !(unsafe { (*p_tab).i_start } >= 0 as i64) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"csvtabFilter".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 831,
                    c"pTab->iStart>=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(unsafe { (*p_tab).i_start } as u64 <=
                                unsafe { (*p_cur).rdr.n_in }) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"csvtabFilter".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 832,
                    c"(size_t)pTab->iStart<=pCur->rdr.nIn".as_ptr() as *mut i8
                        as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p_cur).rdr.i_in = unsafe { (*p_tab).i_start } as u64 };
    } else {
        unsafe {
            fseek(unsafe { (*p_cur).rdr.in_ }, unsafe { (*p_tab).i_start }, 0)
        };
        unsafe { (*p_cur).rdr.i_in = 0 as u64 };
        unsafe { (*p_cur).rdr.n_in = 0 as u64 };
    }
    return csvtab_next(p_vtab_cursor);
}

///* Return TRUE if the cursor has been moved off of the last
///* row of output.
extern "C" fn csvtab_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const CsvCursor = cur as *mut CsvCursor as *const CsvCursor;
    return (unsafe { (*p_cur).i_rowid } < 0 as i64) as i32;
}

///* Return values of columns for the row at which the CsvCursor
///* is currently pointing.
extern "C" fn csvtab_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const CsvCursor = cur as *mut CsvCursor as *const CsvCursor;
    let p_tab: *const CsvTable =
        unsafe { (*cur).p_vtab } as *mut CsvTable as *const CsvTable;
    if i >= 0 && i < unsafe { (*p_tab).n_col } &&
            unsafe { *unsafe { (*p_cur).az_val.offset(i as isize) } } !=
                core::ptr::null_mut() {
        unsafe {
            sqlite3_result_text(ctx,
                unsafe { *unsafe { (*p_cur).az_val.offset(i as isize) } } as
                    *const i8, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
    return 0;
}

///* Return the rowid for the current row.
extern "C" fn csvtab_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid: *mut Sqlite3Int64) -> i32 {
    let p_cur: *const CsvCursor = cur as *mut CsvCursor as *const CsvCursor;
    unsafe { *p_rowid = unsafe { (*p_cur).i_rowid } };
    return 0;
}

static mut csv_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: Some(csvtab_create),
        x_connect: Some(csvtab_connect),
        x_best_index: Some(csvtab_best_index),
        x_disconnect: Some(csvtab_disconnect),
        x_destroy: Some(csvtab_disconnect),
        x_open: Some(csvtab_open),
        x_close: Some(csvtab_close),
        x_filter: Some(csvtab_filter),
        x_next: Some(csvtab_next),
        x_eof: Some(csvtab_eof),
        x_column: Some(csvtab_column),
        x_rowid: Some(csvtab_rowid),
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

/// 
///* This routine is called when the extension is loaded.  The new
///* CSV virtual table module is registered with the calling database
///* connection.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_csv_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"csv".as_ptr() as *mut i8 as *const i8,
                    &raw mut csv_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
        return rc;
    }
}

static mut az_param: [*const i8; 3] =
    [c"filename".as_ptr() as *const i8, c"data".as_ptr() as *const i8,
            c"schema".as_ptr() as *const i8];

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
    fn fclose(_: *mut FILE)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn isspace(_c: i32)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn atoi(_: *const i8)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
