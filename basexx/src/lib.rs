#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;
extern "C" fn init_api_ptr(p_api_1: *const Sqlite3ApiRoutines) -> () {
    { let _ = p_api_1; };
}
static b64_digit_values: [u8; 128] =
    [130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8, 130 as u8, 130 as u8, 129 as u8, 129 as u8, 129 as u8,
            129 as u8, 129 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8, 130 as u8, 129 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8, 62 as u8, 130 as u8, 130 as u8, 130 as u8, 63 as u8,
            52 as u8, 53 as u8, 54 as u8, 55 as u8, 56 as u8, 57 as u8,
            58 as u8, 59 as u8, 60 as u8, 61 as u8, 130 as u8, 130 as u8,
            130 as u8, 128 as u8, 130 as u8, 130 as u8, 130 as u8, 0 as u8,
            1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8,
            8 as u8, 9 as u8, 10 as u8, 11 as u8, 12 as u8, 13 as u8,
            14 as u8, 15 as u8, 16 as u8, 17 as u8, 18 as u8, 19 as u8,
            20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
            130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8,
            32 as u8, 33 as u8, 34 as u8, 35 as u8, 36 as u8, 37 as u8,
            38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
            44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8,
            50 as u8, 51 as u8, 130 as u8, 130 as u8, 130 as u8, 130 as u8,
            130 as u8];
static b64_numerals: [i8; 65] =
    [65 as i8, 66 as i8, 67 as i8, 68 as i8, 69 as i8, 70 as i8, 71 as i8,
            72 as i8, 73 as i8, 74 as i8, 75 as i8, 76 as i8, 77 as i8,
            78 as i8, 79 as i8, 80 as i8, 81 as i8, 82 as i8, 83 as i8,
            84 as i8, 85 as i8, 86 as i8, 87 as i8, 88 as i8, 89 as i8,
            90 as i8, 97 as i8, 98 as i8, 99 as i8, 100 as i8, 101 as i8,
            102 as i8, 103 as i8, 104 as i8, 105 as i8, 106 as i8, 107 as i8,
            108 as i8, 109 as i8, 110 as i8, 111 as i8, 112 as i8, 113 as i8,
            114 as i8, 115 as i8, 116 as i8, 117 as i8, 118 as i8, 119 as i8,
            120 as i8, 121 as i8, 122 as i8, 48 as i8, 49 as i8, 50 as i8,
            51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8, 56 as i8,
            57 as i8, 43 as i8, 47 as i8, 0 as i8];
extern "C" fn to_base64(mut p_in_1: *const u8, mut nb_in_1: i32,
    mut p_out_1: *mut i8) -> *mut i8 {
    let mut n_col: i32 = 0;
    while nb_in_1 >= 3 {
        unsafe {
            *p_out_1.offset(0 as isize) =
                b64_numerals[(unsafe { *p_in_1.offset(0 as isize) } as i32 >>
                                    2) as u8 as usize] as i8
        };
        unsafe {
            *p_out_1.offset(1 as isize) =
                b64_numerals[(((unsafe { *p_in_1.offset(0 as isize) } as i32)
                                            << 4 | unsafe { *p_in_1.offset(1 as isize) } as i32 >> 4) &
                                    63) as u8 as usize] as i8
        };
        unsafe {
            *p_out_1.offset(2 as isize) =
                b64_numerals[((unsafe { *p_in_1.offset(1 as isize) } as i32 &
                                            15) << 2 |
                                    unsafe { *p_in_1.offset(2 as isize) } as i32 >> 6) as u8 as
                            usize] as i8
        };
        unsafe {
            *p_out_1.offset(3 as isize) =
                b64_numerals[(unsafe { *p_in_1.offset(2 as isize) } as i32 &
                                    63) as u8 as usize] as i8
        };
        {
            let __n = 4;
            let __p = &mut p_out_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        nb_in_1 -= 3;
        {
            let __n = 3;
            let __p = &mut p_in_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if { n_col += 4; n_col } >= 72 || nb_in_1 <= 0 {
            unsafe {
                *{
                            let __p = &mut p_out_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        } = '\n' as i32 as i8
            };
            n_col = 0;
        }
    }
    if nb_in_1 > 0 {
        let nco: i8 = (nb_in_1 + 1) as i8;
        let mut nbe: i32 = 0;
        let mut qv: u64 =
            unsafe {
                    *{
                            let __p = &mut p_in_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                } as u64;
        {
            nbe = 1;
            '__b1: loop {
                if !(nbe < 3) { break '__b1; }
                '__c1: loop {
                    qv <<= 8 as u64;
                    if nbe < nb_in_1 {
                        qv |=
                            unsafe {
                                    *{
                                            let __p = &mut p_in_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as u64;
                    }
                    break '__c1;
                }
                { let __p = &mut nbe; *__p += 1; *__p };
            }
        }
        {
            nbe = 3;
            '__b2: loop {
                if !(nbe >= 0) { break '__b2; }
                '__c2: loop {
                    let ce: i8 =
                        if nbe < nco as i32 {
                                b64_numerals[(qv & 63 as u64) as u8 as u8 as usize] as i32
                            } else { '=' as i32 } as i8;
                    qv >>= 6 as u64;
                    unsafe { *p_out_1.offset(nbe as isize) = ce };
                    break '__c2;
                }
                { let __p = &mut nbe; *__p -= 1; *__p };
            }
        }
        {
            let __n = 4;
            let __p = &mut p_out_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        unsafe {
            *{
                        let __p = &mut p_out_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = '\n' as i32 as i8
        };
    }
    unsafe { *p_out_1 = 0 as i8 };
    return p_out_1;
}
extern "C" fn skip_non_b64(mut s: *mut i8, mut nc: i32) -> *mut i8 {
    let mut c: i8 = 0 as i8;
    while { let __p = &mut nc; let __t = *__p; *__p -= 1; __t } > 0 &&
                { c = unsafe { *s }; c } != 0 &&
            !((if (c as u8 as i32) < 128 {
                                            b64_digit_values[c as u8 as usize] as u8 as i32
                                        } else { 128 } as u8 as i32) < 128) as i32 != 0 {
        { let __p = &mut s; *__p = unsafe { (*__p).offset(1) }; *__p };
    }
    return s;
}
extern "C" fn from_base64(mut p_in_1: *mut i8, mut nc_in_1: i32,
    mut p_out_1: *mut u8) -> *mut u8 {
    unsafe {
        if nc_in_1 > 0 &&
                unsafe { *p_in_1.offset((nc_in_1 - 1) as isize) } as i32 ==
                    '\n' as i32 {
            { let __p = &mut nc_in_1; *__p -= 1; *__p };
        }
        while nc_in_1 > 0 && unsafe { *p_in_1 } as i32 != '=' as i32 {
            let p_use: *mut i8 = skip_non_b64(p_in_1, nc_in_1);
            let mut qv: u64 = 0 as u64;
            let mut nti: i32 = 0;
            let mut nbo: i32 = 0;
            let mut nac: i32 = 0;
            nc_in_1 -= unsafe { p_use.offset_from(p_in_1) } as i64 as i32;
            p_in_1 = p_use;
            nti = if nc_in_1 > 4 { 4 } else { nc_in_1 };
            nc_in_1 -= nti;
            nbo = nboi_1[nti as usize] as i32;
            if nbo == 0 { break; }
            {
                nac = 0;
                '__b5: loop {
                    if !(nac < 4) { break '__b5; }
                    '__c5: loop {
                        let c: i8 =
                            if nac < nti {
                                    (unsafe {
                                            *{
                                                    let __p = &mut p_in_1;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }
                                        }) as i32
                                } else { b64_numerals[0 as usize] as i32 } as i8;
                        let mut bdp: u8 =
                            if (c as u8 as i32) < 128 {
                                    b64_digit_values[c as u8 as usize] as u8 as i32
                                } else { 128 } as u8;
                        '__s6:
                            {
                            match bdp {
                                130 => {
                                    nc_in_1 = 0;
                                    nti = nac;
                                    bdp = 0 as u8;
                                    { let __p = &mut nbo; *__p -= 1; *__p };
                                    qv = qv << 6 | bdp as u64;
                                }
                                129 => {
                                    nti = nac;
                                    bdp = 0 as u8;
                                    { let __p = &mut nbo; *__p -= 1; *__p };
                                    qv = qv << 6 | bdp as u64;
                                }
                                128 => {
                                    bdp = 0 as u8;
                                    { let __p = &mut nbo; *__p -= 1; *__p };
                                    qv = qv << 6 | bdp as u64;
                                }
                                _ => { qv = qv << 6 | bdp as u64; }
                            }
                        }
                        break '__c5;
                    }
                    { let __p = &mut nac; *__p += 1; *__p };
                }
            }
            '__s7:
                {
                match nbo {
                    3 => {
                        unsafe {
                            *p_out_1.offset(2 as isize) = (qv & 255 as u64) as u8
                        };
                        unsafe {
                            *p_out_1.offset(1 as isize) = (qv >> 8 & 255 as u64) as u8
                        };
                        unsafe {
                            *p_out_1.offset(0 as isize) = (qv >> 16 & 255 as u64) as u8
                        };
                    }
                    2 => {
                        unsafe {
                            *p_out_1.offset(1 as isize) = (qv >> 8 & 255 as u64) as u8
                        };
                        unsafe {
                            *p_out_1.offset(0 as isize) = (qv >> 16 & 255 as u64) as u8
                        };
                    }
                    1 => {
                        unsafe {
                            *p_out_1.offset(0 as isize) = (qv >> 16 & 255 as u64) as u8
                        };
                    }
                    _ => {}
                }
            }
            {
                let __n = nbo;
                let __p = &mut p_out_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        return p_out_1;
    }
}
extern "C" fn base64(context: *mut Sqlite3Context, na: i32,
    av: *mut *mut Sqlite3Value) -> () {
    let mut nb: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut nv: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut nc: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut nv_max: i32 = 0;
    let mut c_buf: *mut i8 = core::ptr::null_mut();
    let mut b_buf: *mut u8 = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s9:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"base64 OOM".as_ptr() as *mut i8 as *const i8, -1)
                    };
                    __state = 1;
                }
                3 => {
                    nv =
                        unsafe {
                                sqlite3_value_bytes(unsafe { *av.offset(0 as isize) })
                            } as Sqlite3Int64;
                    __state = 4;
                }
                4 => { __state = 5; }
                5 => {
                    nv_max =
                        unsafe {
                            sqlite3_limit(unsafe { sqlite3_context_db_handle(context) },
                                0, -1)
                        };
                    __state = 6;
                }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => {
                    if !(na == 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"base64".as_ptr() as *const i8,
                                c"base64.c".as_ptr() as *mut i8 as *const i8, 217,
                                c"na==1".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 9;
                }
                9 => {
                    '__s10:
                        {
                        match unsafe {
                                sqlite3_value_type(unsafe { *av.offset(0 as isize) })
                            } {
                            4 => { __state = 11; }
                            3 => { __state = 12; }
                            _ => { __state = 13; }
                        }
                    }
                }
                10 => { return; }
                11 => { nb = nv; __state = 15; }
                12 => { nc = nv; __state = 33; }
                13 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"base64 accepts only blob or text".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    __state = 52;
                }
                14 => { __state = 11; }
                15 => {
                    nc =
                        4 as Sqlite3Int64 *
                            ((nv + 2 as Sqlite3Int64) / 3 as Sqlite3Int64);
                    __state = 16;
                }
                16 => {
                    nc +=
                        (nc + (72 - 1) as Sqlite3Int64) / 72 as Sqlite3Int64 +
                            1 as Sqlite3Int64;
                    __state = 17;
                }
                17 => {
                    if (nv_max as Sqlite3Int64) < nc {
                        __state = 19;
                    } else { __state = 18; }
                }
                18 => {
                    b_buf =
                        unsafe {
                                sqlite3_value_blob(unsafe { *av.offset(0 as isize) })
                            } as *mut u8;
                    __state = 21;
                }
                19 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"blob expanded to base64 too big".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    __state = 20;
                }
                20 => { return; }
                21 => {
                    if (b_buf).is_null() as i32 != 0 {
                        __state = 23;
                    } else { __state = 22; }
                }
                22 => {
                    c_buf =
                        unsafe { sqlite3_malloc64(nc as Sqlite3Uint64) } as *mut i8;
                    __state = 27;
                }
                23 => {
                    if 7 ==
                            unsafe {
                                sqlite3_errcode(unsafe {
                                        sqlite3_context_db_handle(context)
                                    })
                            } {
                        __state = 25;
                    } else { __state = 24; }
                }
                24 => {
                    unsafe {
                        sqlite3_result_text(context,
                            c"".as_ptr() as *mut i8 as *const i8, -1, None)
                    };
                    __state = 26;
                }
                25 => { __state = 2; }
                26 => { __state = 10; }
                27 => {
                    if (c_buf).is_null() as i32 != 0 {
                        __state = 29;
                    } else { __state = 28; }
                }
                28 => {
                    nc =
                        unsafe {
                                        to_base64(b_buf as *const u8, nb as i32,
                                                c_buf).offset_from(c_buf)
                                    } as i64 as i32 as Sqlite3Int64;
                    __state = 30;
                }
                29 => { __state = 2; }
                30 => {
                    unsafe {
                        sqlite3_result_text(context, c_buf as *const i8, nc as i32,
                            Some(sqlite3_free))
                    };
                    __state = 31;
                }
                31 => { __state = 10; }
                32 => { __state = 12; }
                33 => {
                    nb =
                        3 as Sqlite3Int64 *
                            ((nv + 3 as Sqlite3Int64) / 4 as Sqlite3Int64);
                    __state = 34;
                }
                34 => {
                    if (nv_max as Sqlite3Int64) < nb {
                        __state = 36;
                    } else { __state = 37; }
                }
                35 => {
                    c_buf =
                        unsafe {
                                sqlite3_value_text(unsafe { *av.offset(0 as isize) })
                            } as *mut i8;
                    __state = 40;
                }
                36 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"blob from base64 may be too big".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    __state = 38;
                }
                37 => {
                    if nb < 1 as i64 { __state = 39; } else { __state = 35; }
                }
                38 => { return; }
                39 => { nb = 1 as Sqlite3Int64; __state = 35; }
                40 => {
                    if (c_buf).is_null() as i32 != 0 {
                        __state = 42;
                    } else { __state = 41; }
                }
                41 => {
                    b_buf =
                        unsafe { sqlite3_malloc64(nb as Sqlite3Uint64) } as *mut u8;
                    __state = 46;
                }
                42 => {
                    if 7 ==
                            unsafe {
                                sqlite3_errcode(unsafe {
                                        sqlite3_context_db_handle(context)
                                    })
                            } {
                        __state = 44;
                    } else { __state = 43; }
                }
                43 => {
                    unsafe { sqlite3_result_zeroblob(context, 0) };
                    __state = 45;
                }
                44 => { __state = 2; }
                45 => { __state = 10; }
                46 => {
                    if (b_buf).is_null() as i32 != 0 {
                        __state = 48;
                    } else { __state = 47; }
                }
                47 => {
                    nb =
                        unsafe {
                                        from_base64(c_buf, nc as i32, b_buf).offset_from(b_buf)
                                    } as i64 as i32 as Sqlite3Int64;
                    __state = 49;
                }
                48 => { __state = 2; }
                49 => {
                    unsafe {
                        sqlite3_result_blob(context, b_buf as *const (), nb as i32,
                            Some(sqlite3_free))
                    };
                    __state = 50;
                }
                50 => { __state = 10; }
                51 => { __state = 13; }
                52 => { return; }
                53 => { __state = 2; }
                _ => {}
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_base64_init(db: *mut Sqlite3,
    pz_err_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    { let _ = p_api_1; };
    { let _ = pz_err_1; };
    return unsafe {
            sqlite3_create_function(db,
                c"base64".as_ptr() as *mut i8 as *const i8, 1,
                2048 | 2097152 | 524288 | 1, core::ptr::null_mut(),
                Some(base64), None, None)
        };
}
static mut b85_c_offset: [u8; 5] =
    [0 as u8, '#' as i32 as u8, 0 as u8, ('*' as i32 - 4) as u8, 0 as u8];
extern "C" fn skip_non_b85(mut s: *mut i8, mut nc: i32) -> *mut i8 {
    let mut c: i8 = 0 as i8;
    while { let __p = &mut nc; let __t = *__p; *__p -= 1; __t } > 0 &&
                { c = unsafe { *s }; c } != 0 &&
            ((c as i32 >= '#' as i32) as i32 + (c as i32 > '&' as i32) as i32
                                    + (c as i32 >= '*' as i32) as i32 +
                                (c as i32 > 'z' as i32) as i32 & 1 == 0) as i32 != 0 {
        { let __p = &mut s; *__p = unsafe { (*__p).offset(1) }; *__p };
    }
    return s;
}
extern "C" fn putcs(mut pc: *mut i8, mut s: *const i8) -> *mut i8 {
    let mut c: i8 = 0 as i8;
    while {
                    c =
                        unsafe {
                            *{
                                    let __p = &mut s;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        };
                    c
                } as i32 != 0 {
        unsafe {
            *{
                        let __p = &mut pc;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = c
        };
    }
    return pc;
}
extern "C" fn to_base85(mut p_in_1: *const u8, mut nb_in_1: i32,
    mut p_out_1: *mut i8, p_sep_1: *mut i8) -> *mut i8 {
    let mut n_col: i32 = 0;
    while nb_in_1 >= 4 {
        let mut nco: i32 = 5;
        let mut qbv: u64 =
            (unsafe { *p_in_1.offset(0 as isize) } as u64) << 24 |
                        ((unsafe { *p_in_1.offset(1 as isize) } as i32) << 16) as
                            u64 |
                    ((unsafe { *p_in_1.offset(2 as isize) } as i32) << 8) as u64
                | unsafe { *p_in_1.offset(3 as isize) } as u64;
        while nco > 0 {
            let nqv: u32 = (qbv / 85) as u32;
            let dv: u8 = (qbv - 85 * nqv as u64) as u8;
            qbv = nqv as u64;
            unsafe {
                *p_out_1.offset({ let __p = &mut nco; *__p -= 1; *__p } as
                                isize) =
                    if (dv as i32) < 4 {
                            (dv as i32 + '#' as i32) as i8 as i32
                        } else { (dv as i32 - 4 + '*' as i32) as i8 as i32 } as i8
            };
        }
        nb_in_1 -= 4;
        {
            let __n = 4;
            let __p = &mut p_in_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        {
            let __n = 5;
            let __p = &mut p_out_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if !(p_sep_1).is_null() && { n_col += 5; n_col } >= 80 {
            p_out_1 = putcs(p_out_1, p_sep_1 as *const i8);
            n_col = 0;
        }
    }
    if nb_in_1 > 0 {
        let mut nco: i32 = nb_in_1 + 1;
        let mut qv: u64 =
            unsafe {
                    *{
                            let __p = &mut p_in_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                } as u64;
        let mut nbe: i32 = 1;
        while { let __p = &mut nbe; let __t = *__p; *__p += 1; __t } < nb_in_1
            {
            qv =
                qv << 8 |
                    unsafe {
                            *{
                                    let __p = &mut p_in_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as u64;
        }
        n_col += nco;
        while nco > 0 {
            let dv: u8 = (qv % 85 as u64) as u8;
            qv /= 85 as u64;
            unsafe {
                *p_out_1.offset({ let __p = &mut nco; *__p -= 1; *__p } as
                                isize) =
                    if (dv as i32) < 4 {
                            (dv as i32 + '#' as i32) as i8 as i32
                        } else { (dv as i32 - 4 + '*' as i32) as i8 as i32 } as i8
            };
        }
        {
            let __n = nb_in_1 + 1;
            let __p = &mut p_out_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    if !(p_sep_1).is_null() && n_col > 0 {
        p_out_1 = putcs(p_out_1, p_sep_1 as *const i8);
    }
    unsafe { *p_out_1 = 0 as i8 };
    return p_out_1;
}
extern "C" fn from_base85(mut p_in_1: *mut i8, mut nc_in_1: i32,
    mut p_out_1: *mut u8) -> *mut u8 {
    unsafe {
        if nc_in_1 > 0 &&
                unsafe { *p_in_1.offset((nc_in_1 - 1) as isize) } as i32 ==
                    '\n' as i32 {
            { let __p = &mut nc_in_1; *__p -= 1; *__p };
        }
        while nc_in_1 > 0 {
            let p_use: *mut i8 = skip_non_b85(p_in_1, nc_in_1);
            let mut qv: u64 = 0 as u64;
            let mut nti: i32 = 0;
            let mut nbo: i32 = 0;
            nc_in_1 -= unsafe { p_use.offset_from(p_in_1) } as i64 as i32;
            p_in_1 = p_use;
            nti = if nc_in_1 > 5 { 5 } else { nc_in_1 };
            nbo = nboi_2[nti as usize] as i32;
            if nbo == 0 { break; }
            while nti > 0 {
                let c: i8 =
                    unsafe {
                        *{
                                let __p = &mut p_in_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    };
                let cdo: u8 =
                    b85_c_offset[((c as i32 >= '#' as i32) as i32 +
                                        (c as i32 > '&' as i32) as i32 +
                                    (c as i32 >= '*' as i32) as i32 +
                                (c as i32 > 'z' as i32) as i32) as usize];
                { let __p = &mut nc_in_1; *__p -= 1; *__p };
                if cdo as i32 == 0 { break; }
                qv = 85 as u64 * qv + (c as i32 - cdo as i32) as u64;
                { let __p = &mut nti; *__p -= 1; *__p };
            }
            nbo -= nti;
            '__s19:
                {
                match nbo {
                    4 => {
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv >> 24 & 255 as u64) as u8
                        };
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv >> 16 & 255 as u64) as u8
                        };
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv >> 8 & 255 as u64) as u8
                        };
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv & 255 as u64) as u8
                        };
                    }
                    3 => {
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv >> 16 & 255 as u64) as u8
                        };
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv >> 8 & 255 as u64) as u8
                        };
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv & 255 as u64) as u8
                        };
                    }
                    2 => {
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv >> 8 & 255 as u64) as u8
                        };
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv & 255 as u64) as u8
                        };
                    }
                    1 => {
                        unsafe {
                            *{
                                        let __p = &mut p_out_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (qv & 255 as u64) as u8
                        };
                    }
                    0 => {}
                    _ => {}
                }
            }
        }
        return p_out_1;
    }
}
extern "C" fn all_base85(mut p: *const i8, mut len: i32) -> i32 {
    let mut c: i8 = 0 as i8;
    while { let __p = &mut len; let __t = *__p; *__p -= 1; __t } > 0 &&
            {
                        c =
                            unsafe {
                                *{
                                        let __p = &mut p;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            };
                        c
                    } as i32 != 0 {
        if ((c as i32 >= '#' as i32) as i32 + (c as i32 > '&' as i32) as i32 +
                                        (c as i32 >= '*' as i32) as i32 +
                                    (c as i32 > 'z' as i32) as i32 & 1 == 0) as i32 != 0 &&
                (unsafe { isspace(c as i32) } == 0) as i32 != 0 {
            return 0;
        }
    }
    return 1;
}
extern "C" fn is_base85(context: *mut Sqlite3Context, na: i32,
    av: *mut *mut Sqlite3Value) -> () {
    if !(na == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"is_base85".as_ptr() as *const i8,
                c"base85.c".as_ptr() as *mut i8 as *const i8, 268,
                c"na==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    '__s21:
        {
        match unsafe { sqlite3_value_type(unsafe { *av.offset(0 as isize) }) }
            {
            3 => {
                {
                    let rv: i32 =
                        all_base85(unsafe {
                                        sqlite3_value_text(unsafe { *av.offset(0 as isize) })
                                    } as *mut i8 as *const i8,
                            unsafe {
                                sqlite3_value_bytes(unsafe { *av.offset(0 as isize) })
                            });
                    unsafe { sqlite3_result_int(context, rv) };
                }
            }
            5 => { unsafe { sqlite3_result_null(context) }; }
            _ => {
                unsafe {
                    sqlite3_result_error(context,
                        c"is_base85 accepts only text or NULL".as_ptr() as *mut i8
                            as *const i8, -1)
                };
                return;
            }
        }
    }
}
extern "C" fn base85(context: *mut Sqlite3Context, na: i32,
    av: *mut *mut Sqlite3Value) -> () {
    let mut nb: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut nc: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut nv: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut nv_max: i32 = 0;
    let mut c_buf: *mut i8 = core::ptr::null_mut();
    let mut b_buf: *mut u8 = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s23:
            {
            match __state {
                0 => {
                    nv =
                        unsafe {
                                sqlite3_value_bytes(unsafe { *av.offset(0 as isize) })
                            } as Sqlite3Int64;
                    __state = 3;
                }
                2 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"base85 OOM".as_ptr() as *mut i8 as *const i8, -1)
                    };
                    __state = 1;
                }
                3 => {
                    nv_max =
                        unsafe {
                            sqlite3_limit(unsafe { sqlite3_context_db_handle(context) },
                                0, -1)
                        };
                    __state = 4;
                }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => {
                    if !(na == 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"base85".as_ptr() as *const i8,
                                c"base85.c".as_ptr() as *mut i8 as *const i8, 294,
                                c"na==1".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 7;
                }
                7 => {
                    '__s24:
                        {
                        match unsafe {
                                sqlite3_value_type(unsafe { *av.offset(0 as isize) })
                            } {
                            4 => { __state = 9; }
                            3 => { __state = 10; }
                            _ => { __state = 11; }
                        }
                    }
                }
                8 => { return; }
                9 => { nb = nv; __state = 13; }
                10 => { nc = nv; __state = 30; }
                11 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"base85 accepts only blob or text.".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    __state = 49;
                }
                12 => { __state = 9; }
                13 => {
                    nc =
                        5 as Sqlite3Int64 * (nv / 4 as Sqlite3Int64) +
                                        nv % 4 as Sqlite3Int64 + nv / 64 as Sqlite3Int64 +
                                1 as Sqlite3Int64 + 2 as Sqlite3Int64;
                    __state = 14;
                }
                14 => {
                    if (nv_max as Sqlite3Int64) < nc {
                        __state = 16;
                    } else { __state = 15; }
                }
                15 => {
                    b_buf =
                        unsafe {
                                sqlite3_value_blob(unsafe { *av.offset(0 as isize) })
                            } as *mut u8;
                    __state = 18;
                }
                16 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"blob expanded to base85 too big".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    __state = 17;
                }
                17 => { return; }
                18 => {
                    if (b_buf).is_null() as i32 != 0 {
                        __state = 20;
                    } else { __state = 19; }
                }
                19 => {
                    c_buf =
                        unsafe { sqlite3_malloc64(nc as Sqlite3Uint64) } as *mut i8;
                    __state = 24;
                }
                20 => {
                    if 7 ==
                            unsafe {
                                sqlite3_errcode(unsafe {
                                        sqlite3_context_db_handle(context)
                                    })
                            } {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => {
                    unsafe {
                        sqlite3_result_text(context,
                            c"".as_ptr() as *mut i8 as *const i8, -1, None)
                    };
                    __state = 23;
                }
                22 => { __state = 2; }
                23 => { __state = 8; }
                24 => {
                    if (c_buf).is_null() as i32 != 0 {
                        __state = 26;
                    } else { __state = 25; }
                }
                25 => {
                    nc =
                        unsafe {
                                        to_base85(b_buf as *const u8, nb as i32, c_buf,
                                                c"\n".as_ptr() as *mut i8).offset_from(c_buf)
                                    } as i64 as i32 as Sqlite3Int64;
                    __state = 27;
                }
                26 => { __state = 2; }
                27 => {
                    unsafe {
                        sqlite3_result_text(context, c_buf as *const i8, nc as i32,
                            Some(sqlite3_free))
                    };
                    __state = 28;
                }
                28 => { __state = 8; }
                29 => { __state = 10; }
                30 => {
                    nb =
                        4 as Sqlite3Int64 * (nv / 5 as Sqlite3Int64) +
                            nv % 5 as Sqlite3Int64;
                    __state = 31;
                }
                31 => {
                    if (nv_max as Sqlite3Int64) < nb {
                        __state = 33;
                    } else { __state = 34; }
                }
                32 => {
                    c_buf =
                        unsafe {
                                sqlite3_value_text(unsafe { *av.offset(0 as isize) })
                            } as *mut i8;
                    __state = 37;
                }
                33 => {
                    unsafe {
                        sqlite3_result_error(context,
                            c"blob from base85 may be too big".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    __state = 35;
                }
                34 => {
                    if nb < 1 as i64 { __state = 36; } else { __state = 32; }
                }
                35 => { return; }
                36 => { nb = 1 as Sqlite3Int64; __state = 32; }
                37 => {
                    if (c_buf).is_null() as i32 != 0 {
                        __state = 39;
                    } else { __state = 38; }
                }
                38 => {
                    b_buf =
                        unsafe { sqlite3_malloc64(nb as Sqlite3Uint64) } as *mut u8;
                    __state = 43;
                }
                39 => {
                    if 7 ==
                            unsafe {
                                sqlite3_errcode(unsafe {
                                        sqlite3_context_db_handle(context)
                                    })
                            } {
                        __state = 41;
                    } else { __state = 40; }
                }
                40 => {
                    unsafe { sqlite3_result_zeroblob(context, 0) };
                    __state = 42;
                }
                41 => { __state = 2; }
                42 => { __state = 8; }
                43 => {
                    if (b_buf).is_null() as i32 != 0 {
                        __state = 45;
                    } else { __state = 44; }
                }
                44 => {
                    nb =
                        unsafe {
                                        from_base85(c_buf, nc as i32, b_buf).offset_from(b_buf)
                                    } as i64 as i32 as Sqlite3Int64;
                    __state = 46;
                }
                45 => { __state = 2; }
                46 => {
                    unsafe {
                        sqlite3_result_blob(context, b_buf as *const (), nb as i32,
                            Some(sqlite3_free))
                    };
                    __state = 47;
                }
                47 => { __state = 8; }
                48 => { __state = 11; }
                49 => { return; }
                50 => { __state = 2; }
                _ => {}
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_base85_init(db: *mut Sqlite3,
    pz_err_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    { let _ = p_api_1; };
    { let _ = pz_err_1; };
    {
        let rc: i32 =
            unsafe {
                sqlite3_create_function(db,
                    c"is_base85".as_ptr() as *mut i8 as *const i8, 1,
                    2048 | 2097152 | 1, core::ptr::null_mut(), Some(is_base85),
                    None, None)
            };
        if rc != 0 { return rc; }
    }
    return unsafe {
            sqlite3_create_function(db,
                c"base85".as_ptr() as *mut i8 as *const i8, 1,
                2048 | 2097152 | 524288 | 1, core::ptr::null_mut(),
                Some(base85), None, None)
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_basexx_init(db: *mut Sqlite3,
    pz_err_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc1: i32 = 0;
    let mut rc2: i32 = 0;
    init_api_ptr(p_api_1);
    rc1 = sqlite3_base64_init(db, core::ptr::null(), core::ptr::null());
    rc2 = sqlite3_base85_init(db, core::ptr::null(), core::ptr::null());
    if rc1 == 0 && rc2 == 0 { return 0; } else { return 1; }
}
static mut nboi_1: [i8; 5] = [0 as i8, 0 as i8, 1 as i8, 2 as i8, 3 as i8];
static mut nboi_2: [i8; 6] =
    [0 as i8, 0 as i8, 1 as i8, 2 as i8, 3 as i8, 4 as i8];
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
    fn isspace(_c: i32)
    -> i32;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}