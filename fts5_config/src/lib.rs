#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod fts5_h;
pub(crate) use crate::fts5_h::*;
mod fts5_int_h;
pub(crate) use crate::fts5_int_h::*;
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

extern "C" fn fts5_isopenquote(x: i8) -> i32 {
    return (x as i32 == '\"' as i32 || x as i32 == '\'' as i32 ||
                    x as i32 == '[' as i32 || x as i32 == '`' as i32) as i32;
}

extern "C" fn fts5_dequote(z: *mut i8) -> i32 {
    let mut q: i8 = 0 as i8;
    let mut i_in: i32 = 1;
    let mut i_out: i32 = 0;
    q = unsafe { *z.offset(0 as isize) };
    if !(q as i32 == '[' as i32 || q as i32 == '\'' as i32 ||
                                q as i32 == '\"' as i32 || q as i32 == '`' as i32) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5Dequote".as_ptr() as *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 152,
                c"q==\'[\' || q==\'\\\'\' || q==\'\"\' || q==\'`\'".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if q as i32 == '[' as i32 { q = ']' as i32 as i8; }
    while unsafe { *z.offset(i_in as isize) } != 0 {
        if unsafe { *z.offset(i_in as isize) } as i32 == q as i32 {
            if unsafe { *z.offset((i_in + 1) as isize) } as i32 != q as i32 {
                { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                break;
            } else {
                i_in += 2;
                unsafe {
                    *z.offset({
                                        let __p = &mut i_out;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = q
                };
            }
        } else {
            unsafe {
                *z.offset({
                                    let __p = &mut i_out;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize) =
                    unsafe {
                        *z.offset({
                                        let __p = &mut i_in;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                    }
            };
        }
    }
    unsafe { *z.offset(i_out as isize) = '\u{0}' as i32 as i8 };
    return i_in;
}

extern "C" fn fts5_config_skip_bareword(p_in_1: *const i8) -> *const i8 {
    let mut p: *const i8 = p_in_1;
    while unsafe { sqlite3_fts5_is_bareword(unsafe { *p }) } != 0 {
        {
            let __p = &mut p;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if p == p_in_1 { p = core::ptr::null(); }
    return p;
}

extern "C" fn fts5_config_gobble_word(p_rc_1: &mut i32, z_in_1: *const i8,
    pz_out_1: &mut *mut i8, pb_quoted_1: &mut i32) -> *const i8 {
    let mut z_ret: *const i8 = core::ptr::null();
    let n_in: Sqlite3Int64 = unsafe { strlen(z_in_1) } as Sqlite3Int64;
    let z_out: *mut i8 =
        unsafe {
                sqlite3_malloc64((n_in + 1 as Sqlite3Int64) as Sqlite3Uint64)
            } as *mut i8;
    if !(*p_rc_1 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ConfigGobbleWord".as_ptr() as *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 458,
                c"*pRc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    *pb_quoted_1 = 0;
    *pz_out_1 = core::ptr::null_mut();
    if z_out == core::ptr::null_mut() {
        *p_rc_1 = 7;
    } else {
        unsafe {
            memcpy(z_out as *mut (), z_in_1 as *const (),
                (n_in + 1 as Sqlite3Int64) as u64)
        };
        if fts5_isopenquote(unsafe { *z_out.offset(0 as isize) }) != 0 {
            let ii: i32 = fts5_dequote(z_out);
            z_ret = unsafe { z_in_1.offset(ii as isize) };
            *pb_quoted_1 = 1;
        } else {
            z_ret = fts5_config_skip_bareword(z_in_1);
            if !(z_ret).is_null() {
                unsafe {
                    *z_out.offset(unsafe { z_ret.offset_from(z_in_1) } as i64 as
                                    isize) = '\u{0}' as i32 as i8
                };
            }
        }
    }
    if z_ret == core::ptr::null() {
        unsafe { sqlite3_free(z_out as *mut ()) };
    } else { *pz_out_1 = z_out; }
    return z_ret;
}

extern "C" fn fts5_iswhitespace(x: i8) -> i32 {
    return (x as i32 == ' ' as i32) as i32;
}

extern "C" fn fts5_config_skip_whitespace(p_in_1: *const i8) -> *const i8 {
    let mut p: *const i8 = p_in_1;
    if !(p).is_null() {
        while fts5_iswhitespace(unsafe { *p }) != 0 {
            {
                let __p = &mut p;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
    return p;
}

extern "C" fn fts5_isdigit(a: i8) -> i32 {
    return (a as i32 >= '0' as i32 && a as i32 <= '9' as i32) as i32;
}

extern "C" fn fts5_config_skip_literal(p_in_1: *const i8) -> *const i8 {
    let mut p: *const i8 = p_in_1;
    '__s3:
        {
        match unsafe { *p } {
            110 => {
                if unsafe {
                            sqlite3_strnicmp(c"null".as_ptr() as *mut i8 as *const i8,
                                p, 4)
                        } == 0 {
                    p = unsafe { p.offset(4 as isize) };
                } else { p = core::ptr::null(); }
            }
            78 => {
                if unsafe {
                            sqlite3_strnicmp(c"null".as_ptr() as *mut i8 as *const i8,
                                p, 4)
                        } == 0 {
                    p = unsafe { p.offset(4 as isize) };
                } else { p = core::ptr::null(); }
            }
            120 => {
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                if unsafe { *p } as i32 == '\'' as i32 {
                    {
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    while unsafe { *p } as i32 >= 'a' as i32 &&
                                    unsafe { *p } as i32 <= 'f' as i32 ||
                                unsafe { *p } as i32 >= 'A' as i32 &&
                                    unsafe { *p } as i32 <= 'F' as i32 ||
                            unsafe { *p } as i32 >= '0' as i32 &&
                                unsafe { *p } as i32 <= '9' as i32 {
                        {
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe { *p } as i32 == '\'' as i32 &&
                            0 as i64 ==
                                unsafe { p.offset_from(p_in_1) } as i64 % 2 as i64 {
                        {
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    } else { p = core::ptr::null(); }
                } else { p = core::ptr::null(); }
            }
            88 => {
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                if unsafe { *p } as i32 == '\'' as i32 {
                    {
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    while unsafe { *p } as i32 >= 'a' as i32 &&
                                    unsafe { *p } as i32 <= 'f' as i32 ||
                                unsafe { *p } as i32 >= 'A' as i32 &&
                                    unsafe { *p } as i32 <= 'F' as i32 ||
                            unsafe { *p } as i32 >= '0' as i32 &&
                                unsafe { *p } as i32 <= '9' as i32 {
                        {
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe { *p } as i32 == '\'' as i32 &&
                            0 as i64 ==
                                unsafe { p.offset_from(p_in_1) } as i64 % 2 as i64 {
                        {
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    } else { p = core::ptr::null(); }
                } else { p = core::ptr::null(); }
            }
            39 => {
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                while !(p).is_null() {
                    if unsafe { *p } as i32 == '\'' as i32 {
                        {
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        if unsafe { *p } as i32 != '\'' as i32 { break; }
                    }
                    {
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    if unsafe { *p } as i32 == 0 { p = core::ptr::null(); }
                }
            }
            _ => {
                if unsafe { *p } as i32 == '+' as i32 ||
                        unsafe { *p } as i32 == '-' as i32 {
                    {
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                while fts5_isdigit(unsafe { *p }) != 0 {
                    {
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                if unsafe { *p } as i32 == '.' as i32 &&
                        fts5_isdigit(unsafe { *p.offset(1 as isize) }) != 0 {
                    {
                        let __n = 2;
                        let __p = &mut p;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    while fts5_isdigit(unsafe { *p }) != 0 {
                        {
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                }
                if p == p_in_1 { p = core::ptr::null(); }
            }
        }
    }
    return p;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_dequote(z: *mut i8) -> () {
    let mut quote: i8 = 0 as i8;
    if !(0 == fts5_iswhitespace(unsafe { *z.offset(0 as isize) })) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5Dequote".as_ptr() as *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 193,
                c"0==fts5_iswhitespace(z[0])".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    quote = unsafe { *z.offset(0 as isize) };
    if quote as i32 == '[' as i32 || quote as i32 == '\'' as i32 ||
                quote as i32 == '\"' as i32 || quote as i32 == '`' as i32 {
        fts5_dequote(z);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Enum {
    z_name: *const i8,
    e_val: i32,
}

extern "C" fn fts5_config_set_enum(a_enum_1: *const Fts5Enum,
    z_enum_1: *const i8, pe_val_1: &mut i32) -> i32 {
    let n_enum: i32 = unsafe { strlen(z_enum_1) } as i32;
    let mut i: i32 = 0;
    let mut i_val: i32 = -1;
    {
        i = 0;
        '__b8: loop {
            if !(!(unsafe {
                                        (*a_enum_1.offset(i as isize)).z_name
                                    }).is_null()) {
                break '__b8;
            }
            '__c8: loop {
                if unsafe {
                            sqlite3_strnicmp(unsafe {
                                    (*a_enum_1.offset(i as isize)).z_name
                                }, z_enum_1, n_enum)
                        } == 0 {
                    if i_val >= 0 { return 1; }
                    i_val =
                        unsafe { (*a_enum_1.offset(i as isize)).e_val } as i32;
                }
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    *pe_val_1 = i_val;
    return if i_val < 0 { 1 } else { 0 };
}

extern "C" fn fts5_config_parse_special(p_config_1: &mut Fts5Config,
    z_cmd_1: *const i8, z_arg_1: *const i8, pz_err_1: &mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let n_cmd: i32 = unsafe { strlen(z_cmd_1) } as i32;
    if unsafe {
                sqlite3_strnicmp(c"prefix".as_ptr() as *mut i8 as *const i8,
                    z_cmd_1, n_cmd)
            } == 0 {
        let n_byte: i32 =
            (core::mem::size_of::<i32>() as u64 * 31 as u64) as i32;
        let mut p: *const i8 = core::ptr::null();
        let mut b_first: i32 = 1;
        if (*p_config_1).a_prefix == core::ptr::null_mut() {
            (*p_config_1).a_prefix =
                unsafe {
                        sqlite3_fts5_malloc_zero(&mut rc, n_byte as Sqlite3Int64)
                    } as *mut i32;
            if rc != 0 { return rc; }
        }
        p = z_arg_1;
        loop {
            let mut n_pre: i32 = 0;
            while unsafe { *p.offset(0 as isize) } as i32 == ' ' as i32 {
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if b_first == 0 &&
                    unsafe { *p.offset(0 as isize) } as i32 == ',' as i32 {
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                while unsafe { *p.offset(0 as isize) } as i32 == ' ' as i32 {
                    {
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
            } else if unsafe { *p.offset(0 as isize) } as i32 ==
                    '\u{0}' as i32 {
                break;
            }
            if (unsafe { *p.offset(0 as isize) } as i32) < '0' as i32 ||
                    unsafe { *p.offset(0 as isize) } as i32 > '9' as i32 {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"malformed prefix=... directive".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                rc = 1;
                break;
            }
            if (*p_config_1).n_prefix == 31 {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"too many prefix indexes (max %d)".as_ptr()
                                    as *mut i8 as *const i8, 31)
                    };
                rc = 1;
                break;
            }
            while unsafe { *p.offset(0 as isize) } as i32 >= '0' as i32 &&
                        unsafe { *p.offset(0 as isize) } as i32 <= '9' as i32 &&
                    n_pre < 1000 {
                n_pre =
                    n_pre * 10 +
                        (unsafe { *p.offset(0 as isize) } as i32 - '0' as i32);
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if n_pre <= 0 || n_pre >= 1000 {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"prefix length out of range (max 999)".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                rc = 1;
                break;
            }
            unsafe {
                *(*p_config_1).a_prefix.offset((*p_config_1).n_prefix as
                                isize) = n_pre
            };
            {
                let __p = &mut (*p_config_1).n_prefix;
                let __t = *__p;
                *__p += 1;
                __t
            };
            b_first = 0;
        }
        if !((*p_config_1).n_prefix <= 31) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ConfigParseSpecial".as_ptr() as *const i8,
                    c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 294,
                    c"pConfig->nPrefix<=FTS5_MAX_PREFIX_INDEXES".as_ptr() as
                            *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"tokenize".as_ptr() as *mut i8 as *const i8,
                    z_cmd_1, n_cmd)
            } == 0 {
        let mut p: *const i8 = z_arg_1 as *const i8;
        let mut n_arg: Sqlite3Int64 =
            (unsafe { strlen(z_arg_1) } + 1 as u64) as Sqlite3Int64;
        let mut az_arg: *mut *mut i8 =
            unsafe {
                    sqlite3_fts5_malloc_zero(&mut rc,
                        ((core::mem::size_of::<*mut i8>() as u64 + 2 as u64) *
                                n_arg as u64) as Sqlite3Int64)
                } as *mut *mut i8;
        if !(az_arg).is_null() {
            let mut p_space: *mut i8 =
                unsafe { &raw mut *az_arg.offset(n_arg as isize) } as *mut i8;
            if !((*p_config_1).t.az_arg).is_null() {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"multiple tokenize=... directives".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                rc = 1;
            } else {
                {
                    n_arg = 0 as Sqlite3Int64;
                    '__b13: loop {
                        if !(!(p).is_null() && unsafe { *p } != 0) { break '__b13; }
                        '__c13: loop {
                            let p2: *const i8 = fts5_config_skip_whitespace(p);
                            if unsafe { *p2 } as i32 == '\'' as i32 {
                                p = fts5_config_skip_literal(p2);
                            } else { p = fts5_config_skip_bareword(p2); }
                            if !(p).is_null() {
                                unsafe {
                                    memcpy(p_space as *mut (), p2 as *const (),
                                        unsafe { p.offset_from(p2) } as i64 as u64)
                                };
                                unsafe { *az_arg.offset(n_arg as isize) = p_space };
                                sqlite3_fts5_dequote(p_space);
                                {
                                    let __n = unsafe { p.offset_from(p2) } as i64 + 1 as i64;
                                    let __p = &mut p_space;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                                p = fts5_config_skip_whitespace(p);
                            }
                            break '__c13;
                        }
                        { let __p = &mut n_arg; let __t = *__p; *__p += 1; __t };
                    }
                }
                if p == core::ptr::null() {
                    *pz_err_1 =
                        unsafe {
                            sqlite3_mprintf(c"parse error in tokenize directive".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    rc = 1;
                } else {
                    (*p_config_1).t.az_arg = az_arg as *mut *const i8;
                    (*p_config_1).t.n_arg = n_arg as i32;
                    az_arg = core::ptr::null_mut();
                }
            }
        }
        unsafe { sqlite3_free(az_arg as *mut ()) };
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"content".as_ptr() as *mut i8 as *const i8,
                    z_cmd_1, n_cmd)
            } == 0 {
        if (*p_config_1).e_content != 0 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"multiple content=... directives".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            if unsafe { *z_arg_1.offset(0 as isize) } != 0 {
                (*p_config_1).e_content = 2;
                (*p_config_1).z_content =
                    unsafe {
                        sqlite3_fts5_mprintf(&mut rc,
                            c"%Q.%Q".as_ptr() as *mut i8 as *const i8,
                            (*p_config_1).z_db, z_arg_1)
                    };
            } else { (*p_config_1).e_content = 1; }
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"contentless_delete".as_ptr() as *mut i8 as
                        *const i8, z_cmd_1, n_cmd)
            } == 0 {
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 != '0' as i32 &&
                    unsafe { *z_arg_1.offset(0 as isize) } as i32 != '1' as i32
                ||
                unsafe { *z_arg_1.offset(1 as isize) } as i32 !=
                    '\u{0}' as i32 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"malformed contentless_delete=... directive".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            (*p_config_1).b_contentless_delete =
                (unsafe { *z_arg_1.offset(0 as isize) } as i32 == '1' as i32)
                    as i32;
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"contentless_unindexed".as_ptr() as *mut i8
                        as *const i8, z_cmd_1, n_cmd)
            } == 0 {
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 != '0' as i32 &&
                    unsafe { *z_arg_1.offset(0 as isize) } as i32 != '1' as i32
                ||
                unsafe { *z_arg_1.offset(1 as isize) } as i32 !=
                    '\u{0}' as i32 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"malformed contentless_delete=... directive".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            (*p_config_1).b_contentless_unindexed =
                (unsafe { *z_arg_1.offset(0 as isize) } as i32 == '1' as i32)
                    as i32;
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"content_rowid".as_ptr() as *mut i8 as
                        *const i8, z_cmd_1, n_cmd)
            } == 0 {
        if !((*p_config_1).z_content_rowid).is_null() {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"multiple content_rowid=... directives".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            (*p_config_1).z_content_rowid =
                unsafe { sqlite3_fts5_strndup(&mut rc, z_arg_1, -1) };
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"columnsize".as_ptr() as *mut i8 as
                        *const i8, z_cmd_1, n_cmd)
            } == 0 {
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 != '0' as i32 &&
                    unsafe { *z_arg_1.offset(0 as isize) } as i32 != '1' as i32
                ||
                unsafe { *z_arg_1.offset(1 as isize) } as i32 !=
                    '\u{0}' as i32 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"malformed columnsize=... directive".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            (*p_config_1).b_columnsize =
                (unsafe { *z_arg_1.offset(0 as isize) } as i32 == '1' as i32)
                    as i32;
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"locale".as_ptr() as *mut i8 as *const i8,
                    z_cmd_1, n_cmd)
            } == 0 {
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 != '0' as i32 &&
                    unsafe { *z_arg_1.offset(0 as isize) } as i32 != '1' as i32
                ||
                unsafe { *z_arg_1.offset(1 as isize) } as i32 !=
                    '\u{0}' as i32 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"malformed locale=... directive".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            (*p_config_1).b_locale =
                (unsafe { *z_arg_1.offset(0 as isize) } as i32 == '1' as i32)
                    as i32;
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"detail".as_ptr() as *mut i8 as *const i8,
                    z_cmd_1, n_cmd)
            } == 0 {
        let a_detail: [Fts5Enum; 4] =
            [Fts5Enum { z_name: c"none".as_ptr() as *const i8, e_val: 1 },
                    Fts5Enum {
                        z_name: c"full".as_ptr() as *const i8,
                        e_val: 0,
                    },
                    Fts5Enum {
                        z_name: c"columns".as_ptr() as *const i8,
                        e_val: 2,
                    }, Fts5Enum { z_name: core::ptr::null(), e_val: 0 }];
        if {
                    rc =
                        fts5_config_set_enum(&raw const a_detail[0 as usize] as
                                *const Fts5Enum, z_arg_1, &mut (*p_config_1).e_detail);
                    rc
                } != 0 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"malformed detail=... directive".as_ptr()
                                as *mut i8 as *const i8)
                };
        }
        return rc;
    }
    if unsafe {
                sqlite3_strnicmp(c"tokendata".as_ptr() as *mut i8 as
                        *const i8, z_cmd_1, n_cmd)
            } == 0 {
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 != '0' as i32 &&
                    unsafe { *z_arg_1.offset(0 as isize) } as i32 != '1' as i32
                ||
                unsafe { *z_arg_1.offset(1 as isize) } as i32 !=
                    '\u{0}' as i32 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"malformed tokendata=... directive".as_ptr()
                                as *mut i8 as *const i8)
                };
            rc = 1;
        } else {
            (*p_config_1).b_tokendata =
                (unsafe { *z_arg_1.offset(0 as isize) } as i32 == '1' as i32)
                    as i32;
        }
        return rc;
    }
    *pz_err_1 =
        unsafe {
            sqlite3_mprintf(c"unrecognized option: \"%.*s\"".as_ptr() as
                        *mut i8 as *const i8, n_cmd, z_cmd_1)
        };
    return 1;
}

extern "C" fn fts5_config_parse_column(p: &mut Fts5Config, z_col_1: *mut i8,
    z_arg_1: *mut i8, pz_err_1: &mut *mut i8, pb_unindexed_1: &mut i32)
    -> i32 {
    let mut rc: i32 = 0;
    if 0 ==
                unsafe {
                    sqlite3_stricmp(z_col_1 as *const i8,
                        c"rank".as_ptr() as *mut i8 as *const i8)
                } ||
            0 ==
                unsafe {
                    sqlite3_stricmp(z_col_1 as *const i8,
                        c"rowid".as_ptr() as *mut i8 as *const i8)
                } {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"reserved fts5 column name: %s".as_ptr() as
                            *mut i8 as *const i8, z_col_1)
            };
        rc = 1;
    } else if !(z_arg_1).is_null() {
        if 0 ==
                unsafe {
                    sqlite3_stricmp(z_arg_1 as *const i8,
                        c"unindexed".as_ptr() as *mut i8 as *const i8)
                } {
            unsafe {
                *(*p).ab_unindexed.offset((*p).n_col as isize) = 1 as u8
            };
            *pb_unindexed_1 = 1;
        } else {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"unrecognized column option: %s".as_ptr()
                                as *mut i8 as *const i8, z_arg_1)
                };
            rc = 1;
        }
    }
    unsafe {
        *(*p).az_col.offset({
                            let __p = &mut (*p).n_col;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        } as isize) = z_col_1
    };
    return rc;
}

extern "C" fn fts5_config_make_exprlist(p: &mut Fts5Config) -> i32 {
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut buf: Fts5Buffer =
        Fts5Buffer { p: core::ptr::null_mut(), n: 0, n_space: 0 };
    unsafe {
        sqlite3_fts5_buffer_append_printf(&mut rc, &mut buf,
            c"T.%Q".as_ptr() as *mut i8, (*p).z_content_rowid)
    };
    if (*p).e_content != 1 {
        if !((*p).e_content == 2 || (*p).e_content == 0 ||
                                (*p).e_content == 3) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ConfigMakeExprlist".as_ptr() as *const i8,
                    c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 527,
                    c"p->eContent==FTS5_CONTENT_EXTERNAL || p->eContent==FTS5_CONTENT_NORMAL || p->eContent==FTS5_CONTENT_UNINDEXED".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        {
            i = 0;
            '__b14: loop {
                if !(i < (*p).n_col) { break '__b14; }
                '__c14: loop {
                    if (*p).e_content == 2 {
                        unsafe {
                            sqlite3_fts5_buffer_append_printf(&mut rc, &mut buf,
                                c", T.%Q".as_ptr() as *mut i8,
                                unsafe { *(*p).az_col.offset(i as isize) })
                        };
                    } else if (*p).e_content == 0 ||
                            unsafe { *(*p).ab_unindexed.offset(i as isize) } != 0 {
                        unsafe {
                            sqlite3_fts5_buffer_append_printf(&mut rc, &mut buf,
                                c", T.c%d".as_ptr() as *mut i8, i)
                        };
                    } else {
                        unsafe {
                            sqlite3_fts5_buffer_append_printf(&mut rc, &mut buf,
                                c", NULL".as_ptr() as *mut i8)
                        };
                    }
                    break '__c14;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if (*p).e_content == 0 && (*p).b_locale != 0 {
        {
            i = 0;
            '__b15: loop {
                if !(i < (*p).n_col) { break '__b15; }
                '__c15: loop {
                    if unsafe { *(*p).ab_unindexed.offset(i as isize) } as i32
                            == 0 {
                        unsafe {
                            sqlite3_fts5_buffer_append_printf(&mut rc, &mut buf,
                                c", T.l%d".as_ptr() as *mut i8, i)
                        };
                    } else {
                        unsafe {
                            sqlite3_fts5_buffer_append_printf(&mut rc, &mut buf,
                                c", NULL".as_ptr() as *mut i8)
                        };
                    }
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if !((*p).z_content_exprlist == core::ptr::null_mut()) as i32 as i64 != 0
        {
        unsafe {
            __assert_rtn(c"fts5ConfigMakeExprlist".as_ptr() as *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 548,
                c"p->zContentExprlist==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    (*p).z_content_exprlist = buf.p as *mut i8;
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_config_free(p_config: *mut Fts5Config) -> () {
    if !(p_config).is_null() {
        let mut i: i32 = 0;
        if !(unsafe { (*p_config).t.p_tok }).is_null() {
            if !(unsafe { (*p_config).t.p_api1 }).is_null() {
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_config).t.p_api1 }).x_delete.unwrap()
                        })(unsafe { (*p_config).t.p_tok })
                };
            } else {
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_config).t.p_api2 }).x_delete.unwrap()
                        })(unsafe { (*p_config).t.p_tok })
                };
            }
        }
        unsafe {
            sqlite3_free(unsafe { (*p_config).t.az_arg } as *mut i8 as
                    *mut ())
        };
        unsafe { sqlite3_free(unsafe { (*p_config).z_db } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p_config).z_name } as *mut ()) };
        {
            i = 0;
            '__b16: loop {
                if !(i < unsafe { (*p_config).n_col }) { break '__b16; }
                '__c16: loop {
                    unsafe {
                        sqlite3_free(unsafe {
                                    *unsafe { (*p_config).az_col.offset(i as isize) }
                                } as *mut ())
                    };
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(unsafe { (*p_config).az_col } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p_config).a_prefix } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p_config).z_rank } as *mut ()) };
        unsafe {
            sqlite3_free(unsafe { (*p_config).z_rank_args } as *mut ())
        };
        unsafe { sqlite3_free(unsafe { (*p_config).z_content } as *mut ()) };
        unsafe {
            sqlite3_free(unsafe { (*p_config).z_content_rowid } as *mut ())
        };
        unsafe {
            sqlite3_free(unsafe { (*p_config).z_content_exprlist } as *mut ())
        };
        unsafe { sqlite3_free(p_config as *mut ()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_config_parse(p_global: *mut Fts5Global,
    db: *mut Sqlite3, n_arg: i32, az_arg: *mut *const i8,
    pp_out: &mut *mut Fts5Config, pz_err: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut p_ret: *mut Fts5Config = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut b_unindexed: i32 = 0;
    *pp_out =
        {
            p_ret =
                unsafe {
                        sqlite3_malloc64(core::mem::size_of::<Fts5Config>() as
                                Sqlite3Uint64)
                    } as *mut Fts5Config;
            p_ret
        };
    if p_ret == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_ret as *mut (), 0, core::mem::size_of::<Fts5Config>() as u64)
    };
    unsafe { (*p_ret).p_global = p_global };
    unsafe { (*p_ret).db = db };
    unsafe { (*p_ret).i_cookie = -1 };
    n_byte =
        (n_arg as u64 *
                (core::mem::size_of::<*mut i8>() as u64 +
                    core::mem::size_of::<u8>() as u64)) as Sqlite3Int64;
    unsafe {
        (*p_ret).az_col =
            unsafe { sqlite3_fts5_malloc_zero(&mut rc, n_byte) } as
                *mut *mut i8
    };
    unsafe {
        (*p_ret).ab_unindexed =
            if !(unsafe { (*p_ret).az_col }).is_null() {
                (unsafe {
                        &raw mut *unsafe { (*p_ret).az_col.offset(n_arg as isize) }
                    }) as *mut u8
            } else { core::ptr::null_mut() }
    };
    unsafe {
        (*p_ret).z_db =
            unsafe {
                sqlite3_fts5_strndup(&mut rc,
                    unsafe { *az_arg.offset(1 as isize) }, -1)
            }
    };
    unsafe {
        (*p_ret).z_name =
            unsafe {
                sqlite3_fts5_strndup(&mut rc,
                    unsafe { *az_arg.offset(2 as isize) }, -1)
            }
    };
    unsafe { (*p_ret).b_columnsize = 1 };
    unsafe { (*p_ret).e_detail = 0 };
    if rc == 0 &&
            unsafe {
                    sqlite3_stricmp(unsafe { (*p_ret).z_name } as *const i8,
                        c"rank".as_ptr() as *mut i8 as *const i8)
                } == 0 {
        unsafe {
            *pz_err =
                unsafe {
                    sqlite3_mprintf(c"reserved fts5 table name: %s".as_ptr() as
                                *mut i8 as *const i8, unsafe { (*p_ret).z_name })
                }
        };
        rc = 1;
    }
    if !(!(unsafe { (*p_ret).ab_unindexed }).is_null() &&
                                !(unsafe { (*p_ret).az_col }).is_null() || rc != 0) as i32
                as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ConfigParse".as_ptr() as *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 601,
                c"(pRet->abUnindexed && pRet->azCol) || rc!=SQLITE_OK".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        i = 3;
        '__b17: loop {
            if !(rc == 0 && i < n_arg) { break '__b17; }
            '__c17: loop {
                let z_orig: *const i8 = unsafe { *az_arg.offset(i as isize) };
                let mut z: *const i8 = core::ptr::null();
                let mut z_one: *mut i8 = core::ptr::null_mut();
                let mut z_two: *mut i8 = core::ptr::null_mut();
                let mut b_option: i32 = 0;
                let mut b_must_be_col: i32 = 0;
                z =
                    fts5_config_gobble_word(&mut rc, z_orig, &mut z_one,
                        &mut b_must_be_col);
                z = fts5_config_skip_whitespace(z);
                if !(z).is_null() && unsafe { *z } as i32 == '=' as i32 {
                    b_option = 1;
                    if !(z_one != core::ptr::null_mut()) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"sqlite3Fts5ConfigParse".as_ptr() as
                                    *const i8,
                                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 614,
                                c"zOne!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    if b_must_be_col != 0 { z = core::ptr::null(); }
                }
                z = fts5_config_skip_whitespace(z);
                if !(z).is_null() && unsafe { *z.offset(0 as isize) } != 0 {
                    let mut b_dummy: i32 = 0;
                    z =
                        fts5_config_gobble_word(&mut rc, z, &mut z_two,
                            &mut b_dummy);
                    if !(z).is_null() && unsafe { *z.offset(0 as isize) } != 0 {
                        z = core::ptr::null();
                    }
                }
                if rc == 0 {
                    if z == core::ptr::null() {
                        unsafe {
                            *pz_err =
                                unsafe {
                                    sqlite3_mprintf(c"parse error in \"%s\"".as_ptr() as *mut i8
                                            as *const i8, z_orig)
                                }
                        };
                        rc = 1;
                    } else {
                        if b_option != 0 {
                            rc =
                                fts5_config_parse_special(unsafe { &mut *p_ret },
                                    if if !(z_one).is_null() {
                                                    1
                                                } else {
                                                    {
                                                        if (0 == 0) as i32 as i64 != 0 {
                                                            unsafe {
                                                                __assert_rtn(c"sqlite3Fts5ConfigParse".as_ptr() as
                                                                        *const i8,
                                                                    c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 632,
                                                                    c"0".as_ptr() as *mut i8 as *const i8)
                                                            }
                                                        } else { { let _ = 0; } };
                                                        0
                                                    }
                                                } != 0 {
                                            z_one
                                        } else { c"".as_ptr() as *mut i8 } as *const i8,
                                    if !(z_two).is_null() {
                                            z_two
                                        } else { c"".as_ptr() as *mut i8 } as *const i8,
                                    unsafe { &mut *pz_err });
                        } else {
                            rc =
                                fts5_config_parse_column(unsafe { &mut *p_ret }, z_one,
                                    z_two, unsafe { &mut *pz_err }, &mut b_unindexed);
                            z_one = core::ptr::null_mut();
                        }
                    }
                }
                unsafe { sqlite3_free(z_one as *mut ()) };
                unsafe { sqlite3_free(z_two as *mut ()) };
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 && unsafe { (*p_ret).b_contentless_delete } != 0 &&
            unsafe { (*p_ret).e_content } != 1 {
        unsafe {
            *pz_err =
                unsafe {
                    sqlite3_mprintf(c"contentless_delete=1 requires a contentless table".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        rc = 1;
    }
    if rc == 0 && unsafe { (*p_ret).b_contentless_delete } != 0 &&
            unsafe { (*p_ret).b_columnsize } == 0 {
        unsafe {
            *pz_err =
                unsafe {
                    sqlite3_mprintf(c"contentless_delete=1 is incompatible with columnsize=0".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        rc = 1;
    }
    if rc == 0 && unsafe { (*p_ret).b_contentless_unindexed } != 0 &&
            unsafe { (*p_ret).e_content } != 1 {
        unsafe {
            *pz_err =
                unsafe {
                    sqlite3_mprintf(c"contentless_unindexed=1 requires a contentless table".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        rc = 1;
    }
    if rc == 0 && unsafe { (*p_ret).z_content } == core::ptr::null_mut() {
        let mut z_tail: *const i8 = core::ptr::null();
        if !(unsafe { (*p_ret).e_content } == 0 ||
                                unsafe { (*p_ret).e_content } == 1) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ConfigParse".as_ptr() as *const i8,
                    c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 687,
                    c"pRet->eContent==FTS5_CONTENT_NORMAL || pRet->eContent==FTS5_CONTENT_NONE".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if unsafe { (*p_ret).e_content } == 0 {
            z_tail = c"content".as_ptr() as *mut i8 as *const i8;
        } else if b_unindexed != 0 &&
                unsafe { (*p_ret).b_contentless_unindexed } != 0 {
            unsafe { (*p_ret).e_content = 3 };
            z_tail = c"content".as_ptr() as *mut i8 as *const i8;
        } else if unsafe { (*p_ret).b_columnsize } != 0 {
            z_tail = c"docsize".as_ptr() as *mut i8 as *const i8;
        }
        if !(z_tail).is_null() {
            unsafe {
                (*p_ret).z_content =
                    unsafe {
                        sqlite3_fts5_mprintf(&mut rc,
                            c"%Q.\'%q_%s\'".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_ret).z_db }, unsafe { (*p_ret).z_name },
                            z_tail)
                    }
            };
        }
    }
    if rc == 0 && unsafe { (*p_ret).z_content_rowid } == core::ptr::null_mut()
        {
        unsafe {
            (*p_ret).z_content_rowid =
                unsafe {
                    sqlite3_fts5_strndup(&mut rc,
                        c"rowid".as_ptr() as *mut i8 as *const i8, -1)
                }
        };
    }
    if rc == 0 { rc = fts5_config_make_exprlist(unsafe { &mut *p_ret }); }
    if rc != 0 {
        sqlite3_fts5_config_free(p_ret);
        *pp_out = core::ptr::null_mut();
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_config_declare_vtab(p_config: &Fts5Config)
    -> i32 {
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    z_sql =
        unsafe {
            sqlite3_fts5_mprintf(&mut rc,
                c"CREATE TABLE x(".as_ptr() as *mut i8 as *const i8)
        };
    {
        i = 0;
        '__b18: loop {
            if !(!(z_sql).is_null() && i < (*p_config).n_col) {
                break '__b18;
            }
            '__c18: loop {
                let z_sep: *const i8 =
                    if i == 0 {
                            c"".as_ptr() as *mut i8
                        } else { c", ".as_ptr() as *mut i8 } as *const i8;
                z_sql =
                    unsafe {
                        sqlite3_fts5_mprintf(&mut rc,
                            c"%z%s%Q".as_ptr() as *mut i8 as *const i8, z_sql, z_sep,
                            unsafe { *(*p_config).az_col.offset(i as isize) })
                    };
                break '__c18;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    z_sql =
        unsafe {
            sqlite3_fts5_mprintf(&mut rc,
                c"%z, %Q HIDDEN, %s HIDDEN)".as_ptr() as *mut i8 as *const i8,
                z_sql, (*p_config).z_name, c"rank".as_ptr() as *mut i8)
        };
    if !(!(z_sql).is_null() || rc == 7) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ConfigDeclareVtab".as_ptr() as
                    *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 769,
                c"zSql || rc==SQLITE_NOMEM".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(z_sql).is_null() {
        rc =
            unsafe {
                sqlite3_declare_vtab((*p_config).db, z_sql as *const i8)
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_tokenize(p_config: *mut Fts5Config, flags: i32,
    p_text: *const i8, n_text: i32, p_ctx: *mut (),
    x_token:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32)
            -> i32>) -> i32 {
    let mut rc: i32 = 0;
    if !(p_text).is_null() {
        if unsafe { (*p_config).t.p_tok } == core::ptr::null_mut() {
            rc = unsafe { sqlite3_fts5_load_tokenizer(p_config) };
        }
        if rc == 0 {
            if !(unsafe { (*p_config).t.p_api1 }).is_null() {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_config).t.p_api1 }).x_tokenize.unwrap()
                            })(unsafe { (*p_config).t.p_tok }, p_ctx, flags, p_text,
                            n_text,
                            x_token.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32)
                                                -> i32>(0 as *const ())
                                }))
                    };
            } else {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_config).t.p_api2 }).x_tokenize.unwrap()
                            })(unsafe { (*p_config).t.p_tok }, p_ctx, flags, p_text,
                            n_text, unsafe { (*p_config).t.p_locale },
                            unsafe { (*p_config).t.n_locale },
                            x_token.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32)
                                                -> i32>(0 as *const ())
                                }))
                    };
            }
        }
    }
    return rc;
}

extern "C" fn fts5_config_skip_args(p_in_1: *const i8) -> *const i8 {
    let mut p: *const i8 = p_in_1;
    loop {
        p = fts5_config_skip_whitespace(p);
        p = fts5_config_skip_literal(p);
        p = fts5_config_skip_whitespace(p);
        if p == core::ptr::null() || unsafe { *p } as i32 == ')' as i32 {
            break;
        }
        if unsafe { *p } as i32 != ',' as i32 {
            p = core::ptr::null();
            break;
        }
        {
            let __p = &mut p;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return p;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_config_parse_rank(z_in: *const i8,
    pz_rank: &mut *mut i8, pz_rank_args: &mut *mut i8) -> i32 {
    let mut p: *const i8 = z_in;
    let mut p_rank: *const i8 = core::ptr::null();
    let mut z_rank: *mut i8 = core::ptr::null_mut();
    let mut z_rank_args: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    *pz_rank = core::ptr::null_mut();
    *pz_rank_args = core::ptr::null_mut();
    if p == core::ptr::null() {
        rc = 1;
    } else {
        p = fts5_config_skip_whitespace(p);
        p_rank = p;
        p = fts5_config_skip_bareword(p);
        if !(p).is_null() {
            z_rank =
                unsafe {
                        sqlite3_fts5_malloc_zero(&mut rc,
                            unsafe {
                                    unsafe { p.offset(1 as isize).offset_from(p_rank) }
                                } as i64)
                    } as *mut i8;
            if !(z_rank).is_null() {
                unsafe {
                    memcpy(z_rank as *mut (), p_rank as *const (),
                        unsafe { p.offset_from(p_rank) } as i64 as u64)
                };
            }
        } else { rc = 1; }
        if rc == 0 {
            p = fts5_config_skip_whitespace(p);
            if unsafe { *p } as i32 != '(' as i32 { rc = 1; }
            {
                let __p = &mut p;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if rc == 0 {
            let mut p_args: *const i8 = core::ptr::null();
            p = fts5_config_skip_whitespace(p);
            p_args = p;
            if unsafe { *p } as i32 != ')' as i32 {
                p = fts5_config_skip_args(p);
                if p == core::ptr::null() {
                    rc = 1;
                } else {
                    z_rank_args =
                        unsafe {
                                sqlite3_fts5_malloc_zero(&mut rc,
                                    unsafe {
                                            unsafe { p.offset(1 as isize).offset_from(p_args) }
                                        } as i64)
                            } as *mut i8;
                    if !(z_rank_args).is_null() {
                        unsafe {
                            memcpy(z_rank_args as *mut (), p_args as *const (),
                                unsafe { p.offset_from(p_args) } as i64 as u64)
                        };
                    }
                }
            }
        }
    }
    if rc != 0 {
        unsafe { sqlite3_free(z_rank as *mut ()) };
        if !(z_rank_args == core::ptr::null_mut()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ConfigParseRank".as_ptr() as
                        *const i8,
                    c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 912,
                    c"zRankArgs==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    } else { *pz_rank = z_rank; *pz_rank_args = z_rank_args; }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_config_set_value(p_config: &mut Fts5Config,
    z_key: *const i8, p_val: *mut Sqlite3Value, pb_badkey: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"pgsz".as_ptr() as *mut i8 as *const i8)
            } {
        let mut pgsz: i32 = 0;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            pgsz = unsafe { sqlite3_value_int(p_val) };
        }
        if pgsz < 32 || pgsz > 64 * 1024 {
            *pb_badkey = 1;
        } else { (*p_config).pgsz = pgsz; }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"hashsize".as_ptr() as *mut i8 as *const i8)
            } {
        let mut n_hash_size: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            n_hash_size = unsafe { sqlite3_value_int(p_val) };
        }
        if n_hash_size <= 0 {
            *pb_badkey = 1;
        } else { (*p_config).n_hash_size = n_hash_size; }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"automerge".as_ptr() as *mut i8 as *const i8)
            } {
        let mut n_automerge: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            n_automerge = unsafe { sqlite3_value_int(p_val) };
        }
        if n_automerge < 0 || n_automerge > 64 {
            *pb_badkey = 1;
        } else {
            if n_automerge == 1 { n_automerge = 4; }
            (*p_config).n_automerge = n_automerge;
        }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"usermerge".as_ptr() as *mut i8 as *const i8)
            } {
        let mut n_usermerge: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            n_usermerge = unsafe { sqlite3_value_int(p_val) };
        }
        if n_usermerge < 2 || n_usermerge > 16 {
            *pb_badkey = 1;
        } else { (*p_config).n_usermerge = n_usermerge; }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"crisismerge".as_ptr() as *mut i8 as *const i8)
            } {
        let mut n_crisis_merge: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            n_crisis_merge = unsafe { sqlite3_value_int(p_val) };
        }
        if n_crisis_merge < 0 {
            *pb_badkey = 1;
        } else {
            if n_crisis_merge <= 1 { n_crisis_merge = 16; }
            if n_crisis_merge >= 2000 { n_crisis_merge = 2000 - 1; }
            (*p_config).n_crisis_merge = n_crisis_merge;
        }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"deletemerge".as_ptr() as *mut i8 as *const i8)
            } {
        let mut n_val: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            n_val = unsafe { sqlite3_value_int(p_val) };
        } else { *pb_badkey = 1; }
        if n_val < 0 { n_val = 10; }
        if n_val > 100 { n_val = 0; }
        (*p_config).n_delete_merge = n_val;
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"rank".as_ptr() as *mut i8 as *const i8)
            } {
        let z_in: *const i8 =
            unsafe { sqlite3_value_text(p_val) } as *const i8;
        let mut z_rank: *mut i8 = core::ptr::null_mut();
        let mut z_rank_args: *mut i8 = core::ptr::null_mut();
        rc =
            sqlite3_fts5_config_parse_rank(z_in, &mut z_rank,
                &mut z_rank_args);
        if rc == 0 {
            unsafe { sqlite3_free((*p_config).z_rank as *mut ()) };
            unsafe { sqlite3_free((*p_config).z_rank_args as *mut ()) };
            (*p_config).z_rank = z_rank;
            (*p_config).z_rank_args = z_rank_args;
        } else if rc == 1 { rc = 0; *pb_badkey = 1; }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"secure-delete".as_ptr() as *mut i8 as *const i8)
            } {
        let mut b_val: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            b_val = unsafe { sqlite3_value_int(p_val) };
        }
        if b_val < 0 {
            *pb_badkey = 1;
        } else {
            (*p_config).b_secure_delete = if b_val != 0 { 1 } else { 0 };
        }
    } else if 0 ==
            unsafe {
                sqlite3_stricmp(z_key,
                    c"insttoken".as_ptr() as *mut i8 as *const i8)
            } {
        let mut b_val_1: i32 = -1;
        if 1 == unsafe { sqlite3_value_numeric_type(p_val) } {
            b_val_1 = unsafe { sqlite3_value_int(p_val) };
        }
        if b_val_1 < 0 {
            *pb_badkey = 1;
        } else {
            (*p_config).b_prefix_insttoken = if b_val_1 != 0 { 1 } else { 0 };
        }
    } else { *pb_badkey = 1; }
    return rc;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_fts5_config_errmsg(p_config: &Fts5Config,
    z_fmt: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg = unsafe { sqlite3_vmprintf(z_fmt, ap) };
    if !((*p_config).pz_errmsg).is_null() {
        if !(unsafe { *(*p_config).pz_errmsg } == core::ptr::null_mut()) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ConfigErrmsg".as_ptr() as *const i8,
                    c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 1118,
                    c"*pConfig->pzErrmsg==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { *(*p_config).pz_errmsg = z_msg };
    } else { unsafe { sqlite3_free(z_msg as *mut ()) }; }
    ();
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_config_load(p_config: *mut Fts5Config,
    i_cookie: i32) -> i32 {
    let z_select: *const i8 =
        c"SELECT k, v FROM %Q.\'%q_config\'".as_ptr() as *mut i8 as *const i8;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut i_version: i32 = 0;
    unsafe { (*p_config).pgsz = 4050 };
    unsafe { (*p_config).n_automerge = 4 };
    unsafe { (*p_config).n_usermerge = 4 };
    unsafe { (*p_config).n_crisis_merge = 16 };
    unsafe { (*p_config).n_hash_size = 1024 * 1024 };
    unsafe { (*p_config).n_delete_merge = 10 };
    z_sql =
        unsafe {
            sqlite3_fts5_mprintf(&mut rc, z_select,
                unsafe { (*p_config).z_db }, unsafe { (*p_config).z_name })
        };
    if !(z_sql).is_null() {
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p_config).db },
                    z_sql as *const i8, -1, &mut p, core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    if !(rc == 0 || p == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ConfigLoad".as_ptr() as *const i8,
                c"fts5_config.c".as_ptr() as *mut i8 as *const i8, 1072,
                c"rc==SQLITE_OK || p==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if rc == 0 {
        while 100 == unsafe { sqlite3_step(p) } {
            let z_k: *const i8 =
                unsafe { sqlite3_column_text(p, 0) } as *const i8;
            let p_val: *mut Sqlite3Value =
                unsafe { sqlite3_column_value(p, 1) };
            if 0 ==
                    unsafe {
                        sqlite3_stricmp(z_k,
                            c"version".as_ptr() as *mut i8 as *const i8)
                    } {
                i_version = unsafe { sqlite3_value_int(p_val) };
            } else {
                let mut b_dummy: i32 = 0;
                sqlite3_fts5_config_set_value(unsafe { &mut *p_config }, z_k,
                    p_val, &mut b_dummy);
            }
        }
        rc = unsafe { sqlite3_finalize(p) };
    }
    if rc == 0 && i_version != 4 && i_version != 5 {
        rc = 1;
        unsafe {
            sqlite3_fts5_config_errmsg(unsafe { &*p_config },
                c"invalid fts5 file format (found %d, expected %d or %d) - run \'rebuild\'".as_ptr()
                        as *mut i8 as *const i8, i_version, 4, 5)
        };
    } else { unsafe { (*p_config).i_version = i_version }; }
    if rc == 0 { unsafe { (*p_config).i_cookie = i_cookie }; }
    return rc;
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
    fn sqlite3_fts5_malloc_zero(p_rc_1: *mut i32, n_byte_1: Sqlite3Int64)
    -> *mut ();
    fn sqlite3_fts5_strndup(p_rc_1: *mut i32, p_in_1: *const i8, n_in_1: i32)
    -> *mut i8;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn sqlite3_fts5_is_bareword(t: i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_fts5_mprintf(p_rc_1: *mut i32, z_fmt_1: *const i8, ...)
    -> *mut i8;
    fn sqlite3_fts5_buffer_append_printf(_: *mut i32, _: *mut Fts5Buffer,
    z_fmt_1: *mut i8, ...)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_fts5_load_tokenizer(p_config_1: *mut Fts5Config)
    -> i32;
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
    fn sqlite3_fts5_vocab_init(_: *mut Fts5Global, _: *mut Sqlite3)
    -> i32;
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
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
