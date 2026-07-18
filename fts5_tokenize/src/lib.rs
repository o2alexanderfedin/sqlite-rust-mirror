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

#[repr(C)]
#[derive(Copy, Clone)]
struct Unicode61Tokenizer {
    a_token_char: [u8; 128],
    a_fold: *mut i8,
    n_fold: i32,
    e_remove_diacritic: i32,
    n_exception: i32,
    ai_exception: *mut i32,
    a_category: [u8; 32],
}

extern "C" fn unicode_set_categories(p: &mut Unicode61Tokenizer,
    z_cat_1: *const i8) -> i32 {
    let mut z: *const i8 = z_cat_1;
    while unsafe { *z } != 0 {
        while unsafe { *z } as i32 == ' ' as i32 ||
                unsafe { *z } as i32 == '\t' as i32 {
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z } != 0 &&
                unsafe {
                        sqlite3_fts5_unicode_cat_parse(z,
                            &raw mut (*p).a_category[0 as usize] as *mut u8)
                    } != 0 {
            return 1;
        }
        while unsafe { *z } as i32 != ' ' as i32 &&
                    unsafe { *z } as i32 != '\t' as i32 &&
                unsafe { *z } as i32 != '\u{0}' as i32 {
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
    unsafe {
        sqlite3_fts5_unicode_ascii(&raw mut (*p).a_category[0 as usize] as
                *mut u8, &raw mut (*p).a_token_char[0 as usize] as *mut u8)
    };
    return 0;
}

static sqlite3_utf8_trans1: [u8; 64] =
    [0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8,
            8 as u8, 9 as u8, 10 as u8, 11 as u8, 12 as u8, 13 as u8,
            14 as u8, 15 as u8, 16 as u8, 17 as u8, 18 as u8, 19 as u8,
            20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
            26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8,
            0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8,
            7 as u8, 8 as u8, 9 as u8, 10 as u8, 11 as u8, 12 as u8, 13 as u8,
            14 as u8, 15 as u8, 0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8,
            5 as u8, 6 as u8, 7 as u8, 0 as u8, 1 as u8, 2 as u8, 3 as u8,
            0 as u8, 1 as u8, 0 as u8, 0 as u8];

extern "C" fn fts5_unicode_add_exceptions(p: &mut Unicode61Tokenizer,
    z: *const i8, b_token_chars_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let n: i32 = unsafe { strlen(z) } as i32;
    let mut a_new: *mut i32 = core::ptr::null_mut();
    if n > 0 {
        a_new =
            unsafe {
                    sqlite3_realloc64((*p).ai_exception as *mut (),
                        (n + (*p).n_exception) as u64 *
                            core::mem::size_of::<i32>() as u64)
                } as *mut i32;
        if !(a_new).is_null() {
            let mut n_new: i32 = (*p).n_exception;
            let mut z_csr: *const u8 = z as *const u8;
            let z_term: *const u8 =
                unsafe { &raw const *z.offset(n as isize) } as *const u8;
            while z_csr < z_term {
                let mut i_code: u32 = 0 as u32;
                let mut b_token: i32 = 0;
                i_code =
                    unsafe {
                            *{
                                    let __p = &mut z_csr;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as u32;
                if i_code >= 192 as u32 {
                    i_code =
                        sqlite3_utf8_trans1[(i_code - 192 as u32) as usize] as u32;
                    while z_csr < z_term &&
                            unsafe { *z_csr } as i32 & 192 == 128 {
                        i_code =
                            (i_code << 6) +
                                (63 &
                                        unsafe {
                                                *{
                                                        let __p = &mut z_csr;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32) as u32;
                    }
                    if i_code < 128 as u32 ||
                                i_code & 4294965248u32 == 55296 as u32 ||
                            i_code & 4294967294u32 == 65534 as u32 {
                        i_code = 65533 as u32;
                    }
                }
                if i_code < 128 as u32 {
                    (*p).a_token_char[i_code as usize] = b_token_chars_1 as u8;
                } else {
                    b_token =
                        (*p).a_category[unsafe {
                                        sqlite3_fts5_unicode_category(i_code)
                                    } as usize] as i32;
                    if !(b_token == 0 || b_token == 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5UnicodeAddExceptions".as_ptr() as
                                    *const i8,
                                c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 279,
                                c"(bToken==0 || bToken==1)".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(b_token_chars_1 == 0 || b_token_chars_1 == 1) as i32 as
                                i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5UnicodeAddExceptions".as_ptr() as
                                    *const i8,
                                c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 280,
                                c"(bTokenChars==0 || bTokenChars==1)".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    if b_token != b_token_chars_1 &&
                            unsafe { sqlite3_fts5_unicode_isdiacritic(i_code as i32) }
                                == 0 {
                        let mut i: i32 = 0;
                        {
                            i = 0;
                            '__b5: loop {
                                if !(i < n_new) { break '__b5; }
                                '__c5: loop {
                                    if unsafe { *a_new.offset(i as isize) } as u32 > i_code {
                                        break '__b5;
                                    }
                                    break '__c5;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            memmove(unsafe { &raw mut *a_new.offset((i + 1) as isize) }
                                    as *mut (),
                                unsafe { &raw mut *a_new.offset(i as isize) } as *const (),
                                (n_new - i) as u64 * core::mem::size_of::<i32>() as u64)
                        };
                        unsafe { *a_new.offset(i as isize) = i_code as i32 };
                        { let __p = &mut n_new; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            (*p).ai_exception = a_new;
            (*p).n_exception = n_new;
        } else { rc = 7; }
    }
    return rc;
}

extern "C" fn fts5_unicode_delete(p_tok_1: *mut Fts5Tokenizer) -> () {
    if !(p_tok_1).is_null() {
        let p: *mut Unicode61Tokenizer = p_tok_1 as *mut Unicode61Tokenizer;
        unsafe { sqlite3_free(unsafe { (*p).ai_exception } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).a_fold } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
    return;
}

extern "C" fn fts5_unicode_create(p_unused_1: *mut (),
    az_arg_1: *mut *const i8, n_arg_1: i32, pp_out_1: *mut *mut Fts5Tokenizer)
    -> i32 {
    let mut rc: i32 = 0;
    let mut p: *mut Unicode61Tokenizer = core::ptr::null_mut();
    { let _ = p_unused_1; };
    if n_arg_1 % 2 != 0 {
        rc = 1;
    } else {
        p =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<Unicode61Tokenizer>()
                            as Sqlite3Uint64)
                } as *mut Unicode61Tokenizer;
        if !(p).is_null() {
            let mut z_cat: *const i8 =
                c"L* N* Co".as_ptr() as *mut i8 as *const i8;
            let mut i: i32 = 0;
            unsafe {
                memset(p as *mut (), 0,
                    core::mem::size_of::<Unicode61Tokenizer>() as u64)
            };
            unsafe { (*p).e_remove_diacritic = 1 };
            unsafe { (*p).n_fold = 64 };
            unsafe {
                (*p).a_fold =
                    unsafe {
                            sqlite3_malloc64(unsafe { (*p).n_fold } as u64 *
                                    core::mem::size_of::<i8>() as u64)
                        } as *mut i8
            };
            if unsafe { (*p).a_fold } == core::ptr::null_mut() { rc = 7; }
            {
                i = 0;
                '__b6: loop {
                    if !(rc == 0 && i < n_arg_1) { break '__b6; }
                    '__c6: loop {
                        if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"categories".as_ptr() as *mut i8 as *const i8)
                                } {
                            z_cat = unsafe { *az_arg_1.offset((i + 1) as isize) };
                        }
                        break '__c6;
                    }
                    i += 2;
                }
            }
            if rc == 0 {
                rc = unicode_set_categories(unsafe { &mut *p }, z_cat);
            }
            {
                i = 0;
                '__b7: loop {
                    if !(rc == 0 && i < n_arg_1) { break '__b7; }
                    '__c7: loop {
                        let z_arg: *const i8 =
                            unsafe { *az_arg_1.offset((i + 1) as isize) };
                        if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"remove_diacritics".as_ptr() as *mut i8 as *const i8)
                                } {
                            if unsafe { *z_arg.offset(0 as isize) } as i32 != '0' as i32
                                            && unsafe { *z_arg.offset(0 as isize) } as i32 != '1' as i32
                                        && unsafe { *z_arg.offset(0 as isize) } as i32 != '2' as i32
                                    || unsafe { *z_arg.offset(1 as isize) } != 0 {
                                rc = 1;
                            } else {
                                unsafe {
                                    (*p).e_remove_diacritic =
                                        unsafe { *z_arg.offset(0 as isize) } as i32 - '0' as i32
                                };
                                if !(unsafe { (*p).e_remove_diacritic } == 0 ||
                                                            unsafe { (*p).e_remove_diacritic } == 1 ||
                                                        unsafe { (*p).e_remove_diacritic } == 2) as i32 as i64 != 0
                                    {
                                    unsafe {
                                        __assert_rtn(c"fts5UnicodeCreate".as_ptr() as *const i8,
                                            c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 403,
                                            c"p->eRemoveDiacritic==FTS5_REMOVE_DIACRITICS_NONE || p->eRemoveDiacritic==FTS5_REMOVE_DIACRITICS_SIMPLE || p->eRemoveDiacritic==FTS5_REMOVE_DIACRITICS_COMPLEX".as_ptr()
                                                    as *mut i8 as *const i8)
                                    }
                                } else { { let _ = 0; } };
                            }
                        } else if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"tokenchars".as_ptr() as *mut i8 as *const i8)
                                } {
                            rc =
                                fts5_unicode_add_exceptions(unsafe { &mut *p }, z_arg, 1);
                        } else if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"separators".as_ptr() as *mut i8 as *const i8)
                                } {
                            rc =
                                fts5_unicode_add_exceptions(unsafe { &mut *p }, z_arg, 0);
                        } else if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"categories".as_ptr() as *mut i8 as *const i8)
                                } {} else { rc = 1; }
                        break '__c7;
                    }
                    i += 2;
                }
            }
        } else { rc = 7; }
        if rc != 0 {
            fts5_unicode_delete(p as *mut Fts5Tokenizer);
            p = core::ptr::null_mut();
        }
        unsafe { *pp_out_1 = p as *mut Fts5Tokenizer };
    }
    return rc;
}

extern "C" fn fts5_unicode_is_exception(p: &Unicode61Tokenizer, i_code_1: i32)
    -> i32 {
    if (*p).n_exception > 0 {
        let a: *const i32 = (*p).ai_exception as *const i32;
        let mut i_lo: i32 = 0;
        let mut i_hi: i32 = (*p).n_exception - 1;
        while i_hi >= i_lo {
            let i_test: i32 = (i_hi + i_lo) / 2;
            if i_code_1 == unsafe { *a.offset(i_test as isize) } {
                return 1;
            } else if i_code_1 > unsafe { *a.offset(i_test as isize) } {
                i_lo = i_test + 1;
            } else { i_hi = i_test - 1; }
        }
    }
    return 0;
}

extern "C" fn fts5_unicode_is_alnum(p: *mut Unicode61Tokenizer, i_code_1: i32)
    -> i32 {
    return unsafe {
                    (*p).a_category[unsafe {
                                sqlite3_fts5_unicode_category(i_code_1 as u32)
                            } as usize]
                } as i32 ^
            fts5_unicode_is_exception(unsafe { &*p }, i_code_1);
}

extern "C" fn fts5_unicode_tokenize(p_tokenizer_1: *mut Fts5Tokenizer,
    p_ctx_1: *mut (), i_unused_1: i32, p_text_1: *const i8, n_text_1: i32,
    x_token_1:
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
    -> i32 {
    let mut p: *mut Unicode61Tokenizer = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut a: *const u8 = core::ptr::null();
    let mut z_term: *mut u8 = core::ptr::null_mut();
    let mut z_csr: *mut u8 = core::ptr::null_mut();
    let mut a_fold: *mut i8 = core::ptr::null_mut();
    let mut n_fold: i32 = 0;
    let mut p_end: *const i8 = core::ptr::null();
    let mut i_code: u32 = 0 as u32;
    let mut z_out: *mut i8 = core::ptr::null_mut();
    let mut is: i32 = 0;
    let mut ie: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s10:
            {
            match __state {
                0 => {
                    p = p_tokenizer_1 as *mut Unicode61Tokenizer;
                    __state = 5;
                }
                2 => {
                    i_code =
                        unsafe {
                                sqlite3_fts5_unicode_fold(i_code as i32,
                                    unsafe { (*p).e_remove_diacritic })
                            } as u32;
                    __state = 65;
                }
                3 => {
                    if unsafe { *z_csr } as i32 >= 'A' as i32 &&
                            unsafe { *z_csr } as i32 <= 'Z' as i32 {
                        __state = 83;
                    } else { __state = 84; }
                }
                4 => { if rc == 101 { __state = 86; } else { __state = 85; } }
                5 => { rc = 0; __state = 6; }
                6 => {
                    a =
                        unsafe { &raw mut (*p).a_token_char[0 as usize] } as *mut u8
                            as *const u8;
                    __state = 7;
                }
                7 => {
                    z_term =
                        unsafe { &raw const *p_text_1.offset(n_text_1 as isize) } as
                            *mut u8;
                    __state = 8;
                }
                8 => { z_csr = p_text_1 as *mut u8; __state = 9; }
                9 => { a_fold = unsafe { (*p).a_fold }; __state = 10; }
                10 => { n_fold = unsafe { (*p).n_fold }; __state = 11; }
                11 => {
                    p_end =
                        unsafe { a_fold.offset((n_fold - 6) as isize) } as
                            *const i8;
                    __state = 12;
                }
                12 => { { let _ = i_unused_1; }; __state = 13; }
                13 => { if rc == 0 { __state = 15; } else { __state = 14; } }
                14 => { __state = 4; }
                15 => { __state = 16; }
                16 => { z_out = a_fold; __state = 17; }
                17 => { __state = 18; }
                18 => { __state = 19; }
                19 => { if 1 != 0 { __state = 21; } else { __state = 20; } }
                20 => {
                    if z_csr < z_term { __state = 40; } else { __state = 39; }
                }
                21 => {
                    if z_csr >= z_term { __state = 23; } else { __state = 22; }
                }
                22 => {
                    if unsafe { *z_csr } as i32 & 128 != 0 {
                        __state = 24;
                    } else { __state = 25; }
                }
                23 => { __state = 4; }
                24 => {
                    is =
                        unsafe { z_csr.offset_from(p_text_1 as *mut u8) } as i64 as
                            i32;
                    __state = 26;
                }
                25 => {
                    if unsafe { *a.add(unsafe { *z_csr } as usize) } != 0 {
                        __state = 37;
                    } else { __state = 36; }
                }
                26 => {
                    i_code =
                        unsafe {
                                *{
                                        let __p = &mut z_csr;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as u32;
                    __state = 27;
                }
                27 => {
                    if i_code >= 192 as u32 {
                        __state = 29;
                    } else { __state = 28; }
                }
                28 => { __state = 34; }
                29 => {
                    i_code =
                        sqlite3_utf8_trans1[(i_code - 192 as u32) as usize] as u32;
                    __state = 30;
                }
                30 => {
                    if z_csr < z_term && unsafe { *z_csr } as i32 & 192 == 128 {
                        __state = 32;
                    } else { __state = 31; }
                }
                31 => {
                    if i_code < 128 as u32 ||
                                i_code & 4294965248u32 == 55296 as u32 ||
                            i_code & 4294967294u32 == 65534 as u32 {
                        __state = 33;
                    } else { __state = 28; }
                }
                32 => {
                    i_code =
                        (i_code << 6) +
                            (63 &
                                    unsafe {
                                            *{
                                                    let __p = &mut z_csr;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }
                                        } as i32) as u32;
                    __state = 30;
                }
                33 => { i_code = 65533 as u32; __state = 28; }
                34 => {
                    if fts5_unicode_is_alnum(p, i_code as i32) != 0 {
                        __state = 35;
                    } else { __state = 19; }
                }
                35 => { __state = 2; }
                36 => {
                    {
                        let __p = &mut z_csr;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 19;
                }
                37 => {
                    is =
                        unsafe { z_csr.offset_from(p_text_1 as *mut u8) } as i64 as
                            i32;
                    __state = 38;
                }
                38 => { __state = 3; }
                39 => {
                    rc =
                        unsafe {
                            x_token_1(p_ctx_1, 0, a_fold as *const i8,
                                unsafe { z_out.offset_from(a_fold) } as i64 as i32, is, ie)
                        };
                    __state = 13;
                }
                40 => {
                    if z_out as *const i8 > p_end {
                        __state = 42;
                    } else { __state = 41; }
                }
                41 => {
                    if unsafe { *z_csr } as i32 & 128 != 0 {
                        __state = 53;
                    } else { __state = 54; }
                }
                42 => {
                    a_fold =
                        unsafe {
                                sqlite3_malloc64((n_fold as Sqlite3Int64 *
                                            2 as Sqlite3Int64) as Sqlite3Uint64)
                            } as *mut i8;
                    __state = 43;
                }
                43 => {
                    if a_fold == core::ptr::null_mut() {
                        __state = 45;
                    } else { __state = 44; }
                }
                44 => {
                    z_out =
                        unsafe {
                            a_fold.offset(unsafe {
                                            z_out.offset_from(unsafe { (*p).a_fold })
                                        } as i64 as isize)
                        };
                    __state = 47;
                }
                45 => { rc = 7; __state = 46; }
                46 => { __state = 4; }
                47 => {
                    unsafe {
                        memcpy(a_fold as *mut (),
                            unsafe { (*p).a_fold } as *const (), n_fold as u64)
                    };
                    __state = 48;
                }
                48 => {
                    unsafe { sqlite3_free(unsafe { (*p).a_fold } as *mut ()) };
                    __state = 49;
                }
                49 => { unsafe { (*p).a_fold = a_fold }; __state = 50; }
                50 => {
                    unsafe { (*p).n_fold = { n_fold = n_fold * 2; n_fold } };
                    __state = 51;
                }
                51 => {
                    p_end =
                        unsafe { a_fold.offset((n_fold - 6) as isize) } as
                            *const i8;
                    __state = 41;
                }
                52 => {
                    ie =
                        unsafe { z_csr.offset_from(p_text_1 as *mut u8) } as i64 as
                            i32;
                    __state = 20;
                }
                53 => {
                    i_code =
                        unsafe {
                                *{
                                        let __p = &mut z_csr;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as u32;
                    __state = 55;
                }
                54 => {
                    if unsafe { *a.add(unsafe { *z_csr } as usize) } as i32 == 0
                        {
                        __state = 80;
                    } else { __state = 81; }
                }
                55 => {
                    if i_code >= 192 as u32 {
                        __state = 57;
                    } else { __state = 56; }
                }
                56 => { __state = 62; }
                57 => {
                    i_code =
                        sqlite3_utf8_trans1[(i_code - 192 as u32) as usize] as u32;
                    __state = 58;
                }
                58 => {
                    if z_csr < z_term && unsafe { *z_csr } as i32 & 192 == 128 {
                        __state = 60;
                    } else { __state = 59; }
                }
                59 => {
                    if i_code < 128 as u32 ||
                                i_code & 4294965248u32 == 55296 as u32 ||
                            i_code & 4294967294u32 == 65534 as u32 {
                        __state = 61;
                    } else { __state = 56; }
                }
                60 => {
                    i_code =
                        (i_code << 6) +
                            (63 &
                                    unsafe {
                                            *{
                                                    let __p = &mut z_csr;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }
                                        } as i32) as u32;
                    __state = 58;
                }
                61 => { i_code = 65533 as u32; __state = 56; }
                62 => {
                    if fts5_unicode_is_alnum(p, i_code as i32) != 0 ||
                            unsafe { sqlite3_fts5_unicode_isdiacritic(i_code as i32) }
                                != 0 {
                        __state = 63;
                    } else { __state = 64; }
                }
                63 => { __state = 2; }
                64 => { __state = 39; }
                65 => {
                    if i_code != 0 { __state = 67; } else { __state = 66; }
                }
                66 => { __state = 52; }
                67 => {
                    if i_code < 128 as u32 {
                        __state = 68;
                    } else { __state = 69; }
                }
                68 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (i_code & 255 as u32) as u8 as i8
                    };
                    __state = 66;
                }
                69 => {
                    if i_code < 2048 as u32 {
                        __state = 70;
                    } else { __state = 71; }
                }
                70 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (192 + (i_code >> 6 & 31 as u32) as u8 as i32) as i8
                    };
                    __state = 72;
                }
                71 => {
                    if i_code < 65536 as u32 {
                        __state = 73;
                    } else { __state = 74; }
                }
                72 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                    };
                    __state = 66;
                }
                73 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (224 + (i_code >> 12 & 15 as u32) as u8 as i32) as i8
                    };
                    __state = 75;
                }
                74 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (240 + (i_code >> 18 & 7 as u32) as u8 as i32) as i8
                    };
                    __state = 77;
                }
                75 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (i_code >> 6 & 63 as u32) as u8 as i32) as i8
                    };
                    __state = 76;
                }
                76 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                    };
                    __state = 66;
                }
                77 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (i_code >> 12 & 63 as u32) as u8 as i32) as i8
                    };
                    __state = 78;
                }
                78 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (i_code >> 6 & 63 as u32) as u8 as i32) as i8
                    };
                    __state = 79;
                }
                79 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                    };
                    __state = 66;
                }
                80 => { __state = 39; }
                81 => { __state = 3; }
                82 => {
                    {
                        let __p = &mut z_csr;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 52;
                }
                83 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = (unsafe { *z_csr } as i32 + 32) as i8
                    };
                    __state = 82;
                }
                84 => {
                    unsafe {
                        *{
                                    let __p = &mut z_out;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = unsafe { *z_csr } as i8
                    };
                    __state = 82;
                }
                85 => { return rc; }
                86 => { rc = 0; __state = 85; }
                _ => {}
            }
        }
    }
    unreachable!();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct AsciiTokenizer {
    a_token_char: [u8; 128],
}

static mut a_ascii_token_char: [u8; 128] =
    [0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8];

extern "C" fn fts5_ascii_add_exceptions(p: &mut AsciiTokenizer,
    z_arg_1: *const i8, b_token_chars_1: i32) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b11: loop {
            if !(unsafe { *z_arg_1.offset(i as isize) } != 0) {
                break '__b11;
            }
            '__c11: loop {
                if unsafe { *z_arg_1.offset(i as isize) } as i32 & 128 == 0 {
                    (*p).a_token_char[unsafe { *z_arg_1.offset(i as isize) } as
                                    i32 as usize] = b_token_chars_1 as u8;
                }
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn fts5_ascii_delete(p: *mut Fts5Tokenizer) -> () {
    unsafe { sqlite3_free(p as *mut ()) };
}

extern "C" fn fts5_ascii_create(p_unused_1: *mut (), az_arg_1: *mut *const i8,
    n_arg_1: i32, pp_out_1: *mut *mut Fts5Tokenizer) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p: *mut AsciiTokenizer = core::ptr::null_mut();
        { let _ = p_unused_1; };
        if n_arg_1 % 2 != 0 {
            rc = 1;
        } else {
            p =
                unsafe {
                        sqlite3_malloc64(core::mem::size_of::<AsciiTokenizer>() as
                                Sqlite3Uint64)
                    } as *mut AsciiTokenizer;
            if p == core::ptr::null_mut() {
                rc = 7;
            } else {
                let mut i: i32 = 0;
                unsafe {
                    memset(p as *mut (), 0,
                        core::mem::size_of::<AsciiTokenizer>() as u64)
                };
                unsafe {
                    memcpy(unsafe { &raw mut (*p).a_token_char[0 as usize] } as
                                *mut u8 as *mut (),
                        &raw mut a_ascii_token_char[0 as usize] as *mut u8 as
                            *const (), core::mem::size_of::<[u8; 128]>() as u64)
                };
                {
                    i = 0;
                    '__b12: loop {
                        if !(rc == 0 && i < n_arg_1) { break '__b12; }
                        '__c12: loop {
                            let z_arg: *const i8 =
                                unsafe { *az_arg_1.offset((i + 1) as isize) };
                            if 0 ==
                                    unsafe {
                                        sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                            c"tokenchars".as_ptr() as *mut i8 as *const i8)
                                    } {
                                fts5_ascii_add_exceptions(unsafe { &mut *p }, z_arg, 1);
                            } else if 0 ==
                                    unsafe {
                                        sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                            c"separators".as_ptr() as *mut i8 as *const i8)
                                    } {
                                fts5_ascii_add_exceptions(unsafe { &mut *p }, z_arg, 0);
                            } else { rc = 1; }
                            break '__c12;
                        }
                        i += 2;
                    }
                }
                if rc != 0 {
                    fts5_ascii_delete(p as *mut Fts5Tokenizer);
                    p = core::ptr::null_mut();
                }
            }
        }
        unsafe { *pp_out_1 = p as *mut Fts5Tokenizer };
        return rc;
    }
}

extern "C" fn ascii_fold(a_out_1: *mut i8, a_in_1: &[i8]) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b13: loop {
            if !(i < a_in_1.len() as i32) { break '__b13; }
            '__c13: loop {
                let mut c: i8 = a_in_1[i as usize] as i8;
                if c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32 {
                    c += 32 as i8;
                }
                unsafe { *a_out_1.offset(i as isize) = c };
                break '__c13;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn fts5_ascii_tokenize(p_tokenizer_1: *mut Fts5Tokenizer,
    p_ctx_1: *mut (), i_unused_1: i32, p_text_1: *const i8, n_text_1: i32,
    x_token_1:
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
    -> i32 {
    let p: *mut AsciiTokenizer = p_tokenizer_1 as *mut AsciiTokenizer;
    let mut rc: i32 = 0;
    let mut ie: i32 = 0;
    let mut is: i32 = 0;
    let mut a_fold: [i8; 64] = [0; 64];
    let mut n_fold: i32 = core::mem::size_of::<[i8; 64]>() as i32;
    let mut p_fold: *mut i8 = &raw mut a_fold[0 as usize] as *mut i8;
    let a: *const u8 =
        unsafe { &raw const (*p).a_token_char[0 as usize] } as *const u8;
    { let _ = i_unused_1; };
    while is < n_text_1 && rc == 0 {
        let mut n_byte: i32 = 0;
        while is < n_text_1 &&
                (unsafe { *p_text_1.offset(is as isize) } as i32 & 128 == 0 &&
                    unsafe {
                                *a.offset(unsafe { *p_text_1.offset(is as isize) } as i32 as
                                            isize)
                            } as i32 == 0) {
            { let __p = &mut is; let __t = *__p; *__p += 1; __t };
        }
        if is == n_text_1 { break; }
        ie = is + 1;
        while ie < n_text_1 &&
                (unsafe { *p_text_1.offset(ie as isize) } as i32 & 128 != 0 ||
                    unsafe {
                            *a.offset(unsafe { *p_text_1.offset(ie as isize) } as i32 as
                                        isize)
                        } != 0) {
            { let __p = &mut ie; let __t = *__p; *__p += 1; __t };
        }
        n_byte = ie - is;
        if n_byte > n_fold {
            if p_fold != &raw mut a_fold[0 as usize] as *mut i8 {
                unsafe { sqlite3_free(p_fold as *mut ()) };
            }
            p_fold =
                unsafe {
                        sqlite3_malloc64((n_byte as Sqlite3Int64 *
                                    2 as Sqlite3Int64) as Sqlite3Uint64)
                    } as *mut i8;
            if p_fold == core::ptr::null_mut() { rc = 7; break; }
            n_fold = n_byte * 2;
        }
        ascii_fold(p_fold,
            unsafe {
                let __p =
                    unsafe { &*p_text_1.offset(is as isize) } as *const i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_byte as usize) }
            });
        rc =
            unsafe {
                x_token_1(p_ctx_1, 0, p_fold as *const i8, n_byte, is, ie)
            };
        is = ie + 1;
    }
    if p_fold != &raw mut a_fold[0 as usize] as *mut i8 {
        unsafe { sqlite3_free(p_fold as *mut ()) };
    }
    if rc == 101 { rc = 0; }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TrigramTokenizer {
    b_fold: i32,
    i_fold_param: i32,
}

extern "C" fn fts5_tri_delete(p: *mut Fts5Tokenizer) -> () {
    unsafe { sqlite3_free(p as *mut ()) };
}

extern "C" fn fts5_tri_create(p_unused_1: *mut (), az_arg_1: *mut *const i8,
    n_arg_1: i32, pp_out_1: *mut *mut Fts5Tokenizer) -> i32 {
    let mut rc: i32 = 0;
    let mut p_new: *mut TrigramTokenizer = core::ptr::null_mut();
    { let _ = p_unused_1; };
    if n_arg_1 % 2 != 0 {
        rc = 1;
    } else {
        let mut i: i32 = 0;
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<TrigramTokenizer>() as
                            Sqlite3Uint64)
                } as *mut TrigramTokenizer;
        if p_new == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe { (*p_new).b_fold = 1 };
            unsafe { (*p_new).i_fold_param = 0 };
            {
                i = 0;
                '__b17: loop {
                    if !(rc == 0 && i < n_arg_1) { break '__b17; }
                    '__c17: loop {
                        let z_arg: *const i8 =
                            unsafe { *az_arg_1.offset((i + 1) as isize) };
                        if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"case_sensitive".as_ptr() as *mut i8 as *const i8)
                                } {
                            if unsafe { *z_arg.offset(0 as isize) } as i32 != '0' as i32
                                        && unsafe { *z_arg.offset(0 as isize) } as i32 != '1' as i32
                                    || unsafe { *z_arg.offset(1 as isize) } != 0 {
                                rc = 1;
                            } else {
                                unsafe {
                                    (*p_new).b_fold =
                                        (unsafe { *z_arg.offset(0 as isize) } as i32 == '0' as i32)
                                            as i32
                                };
                            }
                        } else if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { *az_arg_1.offset(i as isize) },
                                        c"remove_diacritics".as_ptr() as *mut i8 as *const i8)
                                } {
                            if unsafe { *z_arg.offset(0 as isize) } as i32 != '0' as i32
                                            && unsafe { *z_arg.offset(0 as isize) } as i32 != '1' as i32
                                        && unsafe { *z_arg.offset(0 as isize) } as i32 != '2' as i32
                                    || unsafe { *z_arg.offset(1 as isize) } != 0 {
                                rc = 1;
                            } else {
                                unsafe {
                                    (*p_new).i_fold_param =
                                        if unsafe { *z_arg.offset(0 as isize) } as i32 != '0' as i32
                                            {
                                            2
                                        } else { 0 }
                                };
                            }
                        } else { rc = 1; }
                        break '__c17;
                    }
                    i += 2;
                }
            }
            if unsafe { (*p_new).i_fold_param } != 0 &&
                    unsafe { (*p_new).b_fold } == 0 {
                rc = 1;
            }
            if rc != 0 {
                fts5_tri_delete(p_new as *mut Fts5Tokenizer);
                p_new = core::ptr::null_mut();
            }
        }
    }
    unsafe { *pp_out_1 = p_new as *mut Fts5Tokenizer };
    return rc;
}

extern "C" fn fts5_tri_tokenize(p_tok_1: *mut Fts5Tokenizer, p_ctx_1: *mut (),
    unused_flags_1: i32, p_text_1: *const i8, n_text_1: i32,
    x_token_1:
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
    -> i32 {
    let p: *const TrigramTokenizer =
        p_tok_1 as *mut TrigramTokenizer as *const TrigramTokenizer;
    let mut rc: i32 = 0;
    let mut a_buf: [i8; 32] = [0; 32];
    let mut z_out: *mut i8 = &raw mut a_buf[0 as usize] as *mut i8;
    let mut ii: i32 = 0;
    let mut z_in: *const u8 = p_text_1 as *const u8;
    let z_eof: *const u8 =
        if !(z_in).is_null() {
            unsafe { &*z_in.offset(n_text_1 as isize) }
        } else { core::ptr::null() };
    let mut i_code: u32 = 0 as u32;
    let mut a_start: [i32; 3] = [0; 3];
    { let _ = unused_flags_1; };
    {
        ii = 0;
        '__b18: loop {
            if !(ii < 3) { break '__b18; }
            '__c18: loop {
                '__b19: loop {
                    '__c19: loop {
                        a_start[ii as usize] =
                            unsafe { z_in.offset_from(p_text_1 as *const u8) } as i64 as
                                i32;
                        if z_in >= z_eof { return 0; }
                        i_code =
                            unsafe {
                                    *{
                                            let __p = &mut z_in;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as u32;
                        if i_code >= 192 as u32 {
                            i_code =
                                sqlite3_utf8_trans1[(i_code - 192 as u32) as usize] as u32;
                            while z_in < z_eof && unsafe { *z_in } as i32 & 192 == 128 {
                                i_code =
                                    (i_code << 6) +
                                        (63 &
                                                unsafe {
                                                        *{
                                                                let __p = &mut z_in;
                                                                let __t = *__p;
                                                                *__p = unsafe { (*__p).offset(1) };
                                                                __t
                                                            }
                                                    } as i32) as u32;
                            }
                            if i_code < 128 as u32 ||
                                        i_code & 4294965248u32 == 55296 as u32 ||
                                    i_code & 4294967294u32 == 65534 as u32 {
                                i_code = 65533 as u32;
                            }
                        }
                        if unsafe { (*p).b_fold } != 0 {
                            i_code =
                                unsafe {
                                        sqlite3_fts5_unicode_fold(i_code as i32,
                                            unsafe { (*p).i_fold_param })
                                    } as u32;
                        }
                        break '__c19;
                    }
                    if !(i_code == 0 as u32) { break '__b19; }
                }
                {
                    if i_code < 128 as u32 {
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (i_code & 255 as u32) as u8 as i8
                        };
                    } else if i_code < 2048 as u32 {
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (192 + (i_code >> 6 & 31 as u32) as u8 as i32) as i8
                        };
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                        };
                    } else if i_code < 65536 as u32 {
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (224 + (i_code >> 12 & 15 as u32) as u8 as i32) as i8
                        };
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (128 + (i_code >> 6 & 63 as u32) as u8 as i32) as i8
                        };
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                        };
                    } else {
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (240 + (i_code >> 18 & 7 as u32) as u8 as i32) as i8
                        };
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (128 + (i_code >> 12 & 63 as u32) as u8 as i32) as i8
                        };
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (128 + (i_code >> 6 & 63 as u32) as u8 as i32) as i8
                        };
                        unsafe {
                            *{
                                        let __p = &mut z_out;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                        };
                    }
                }
                break '__c18;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if !(z_in <= z_eof) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5TriTokenize".as_ptr() as *const i8,
                c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 1386,
                c"zIn<=zEof".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    loop {
        let mut i_next: i32 = 0;
        let mut z1: *const i8 = core::ptr::null();
        '__b22: loop {
            '__c22: loop {
                i_next =
                    unsafe { z_in.offset_from(p_text_1 as *const u8) } as i64 as
                        i32;
                if z_in >= z_eof { i_code = 0 as u32; break '__b22; }
                i_code =
                    unsafe {
                            *{
                                    let __p = &mut z_in;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as u32;
                if i_code >= 192 as u32 {
                    i_code =
                        sqlite3_utf8_trans1[(i_code - 192 as u32) as usize] as u32;
                    while z_in < z_eof && unsafe { *z_in } as i32 & 192 == 128 {
                        i_code =
                            (i_code << 6) +
                                (63 &
                                        unsafe {
                                                *{
                                                        let __p = &mut z_in;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32) as u32;
                    }
                    if i_code < 128 as u32 ||
                                i_code & 4294965248u32 == 55296 as u32 ||
                            i_code & 4294967294u32 == 65534 as u32 {
                        i_code = 65533 as u32;
                    }
                }
                if unsafe { (*p).b_fold } != 0 {
                    i_code =
                        unsafe {
                                sqlite3_fts5_unicode_fold(i_code as i32,
                                    unsafe { (*p).i_fold_param })
                            } as u32;
                }
                break '__c22;
            }
            if !(i_code == 0 as u32) { break '__b22; }
        }
        rc =
            unsafe {
                x_token_1(p_ctx_1, 0,
                    &raw mut a_buf[0 as usize] as *mut i8 as *const i8,
                    unsafe {
                                z_out.offset_from(&raw mut a_buf[0 as usize] as *mut i8)
                            } as i64 as i32, a_start[0 as usize], i_next)
            };
        if i_code == 0 as u32 || rc != 0 { break; }
        z1 = &raw mut a_buf[0 as usize] as *mut i8 as *const i8;
        {
            if unsafe {
                                *{
                                        let __p = &mut z1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as u8 as i32 >= 192 {
                while unsafe { *z1 } as u8 as i32 & 192 == 128 {
                    {
                        let __p = &mut z1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
            }
        }
        unsafe {
            memmove(&raw mut a_buf[0 as usize] as *mut i8 as *mut (),
                z1 as *const (),
                unsafe { z_out.offset_from(z1) } as i64 as u64)
        };
        {
            let __n =
                unsafe {
                        z1.offset_from(&raw mut a_buf[0 as usize] as *mut i8)
                    } as i64;
            let __p = &mut z_out;
            *__p = unsafe { (*__p).offset(-(__n as isize)) };
        };
        {
            if i_code < 128 as u32 {
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (i_code & 255 as u32) as u8 as i8
                };
            } else if i_code < 2048 as u32 {
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (192 + (i_code >> 6 & 31 as u32) as u8 as i32) as i8
                };
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                };
            } else if i_code < 65536 as u32 {
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (224 + (i_code >> 12 & 15 as u32) as u8 as i32) as i8
                };
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (128 + (i_code >> 6 & 63 as u32) as u8 as i32) as i8
                };
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                };
            } else {
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (240 + (i_code >> 18 & 7 as u32) as u8 as i32) as i8
                };
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (128 + (i_code >> 12 & 63 as u32) as u8 as i32) as i8
                };
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (128 + (i_code >> 6 & 63 as u32) as u8 as i32) as i8
                };
                unsafe {
                    *{
                                let __p = &mut z_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = (128 + (i_code & 63 as u32) as u8 as i32) as i8
                };
            }
        }
        a_start[0 as usize] = a_start[1 as usize];
        a_start[1 as usize] = a_start[2 as usize];
        a_start[2 as usize] = i_next;
    }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct PorterTokenizer {
    tokenizer_v2: Fts5TokenizerV2,
    p_tokenizer: *mut Fts5Tokenizer,
    a_buf: [i8; 128],
}

extern "C" fn fts5_porter_delete(p_tok_1: *mut Fts5Tokenizer) -> () {
    if !(p_tok_1).is_null() {
        let p: *mut PorterTokenizer = p_tok_1 as *mut PorterTokenizer;
        if !(unsafe { (*p).p_tokenizer }).is_null() {
            unsafe {
                (unsafe {
                        (*p).tokenizer_v2.x_delete.unwrap()
                    })(unsafe { (*p).p_tokenizer })
            };
        }
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

extern "C" fn fts5_porter_create(p_ctx_1: *mut (),
    mut az_arg_1: *mut *const i8, mut n_arg_1: i32,
    pp_out_1: *mut *mut Fts5Tokenizer) -> i32 {
    let p_api: *mut Fts5Api = p_ctx_1 as *mut Fts5Api;
    let mut rc: i32 = 0;
    let mut p_ret: *mut PorterTokenizer = core::ptr::null_mut();
    let mut p_userdata: *mut () = core::ptr::null_mut();
    let mut z_base: *const i8 = c"unicode61".as_ptr() as *mut i8 as *const i8;
    let mut p_v2: *mut Fts5TokenizerV2 = core::ptr::null_mut();
    while n_arg_1 > 0 {
        if unsafe {
                    sqlite3_stricmp(unsafe { *az_arg_1.offset(0 as isize) },
                        c"porter".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            { let __p = &mut n_arg_1; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut az_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else { z_base = unsafe { *az_arg_1.offset(0 as isize) }; break; }
    }
    p_ret =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<PorterTokenizer>() as
                        Sqlite3Uint64)
            } as *mut PorterTokenizer;
    if !(p_ret).is_null() {
        unsafe {
            memset(p_ret as *mut (), 0,
                core::mem::size_of::<PorterTokenizer>() as u64)
        };
        rc =
            unsafe {
                (unsafe {
                        (*p_api).x_find_tokenizer_v2.unwrap()
                    })(p_api, z_base, &mut p_userdata, &mut p_v2)
            };
    } else { rc = 7; }
    if rc == 0 {
        let n_arg2: i32 = if n_arg_1 > 0 { n_arg_1 - 1 } else { 0 };
        let az2: *mut *const i8 =
            if n_arg2 != 0 {
                unsafe { &mut *az_arg_1.offset(1 as isize) }
            } else { core::ptr::null_mut() };
        unsafe {
            memcpy(unsafe { &raw mut (*p_ret).tokenizer_v2 } as *mut (),
                p_v2 as *const (),
                core::mem::size_of::<Fts5TokenizerV2>() as u64)
        };
        rc =
            unsafe {
                (unsafe {
                        (*p_ret).tokenizer_v2.x_create.unwrap()
                    })(p_userdata, az2, n_arg2,
                    unsafe { &mut (*p_ret).p_tokenizer })
            };
    }
    if rc != 0 {
        fts5_porter_delete(p_ret as *mut Fts5Tokenizer);
        p_ret = core::ptr::null_mut();
    }
    unsafe { *pp_out_1 = p_ret as *mut Fts5Tokenizer };
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct PorterContext {
    p_ctx: *mut (),
    x_token: Option<unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32,
        i32) -> i32>,
    a_buf: *mut i8,
}

extern "C" fn fts5_porter_step1_a(a_buf_1: *const i8, pn_buf_1: &mut i32)
    -> () {
    let n_buf: i32 = *pn_buf_1;
    if unsafe { *a_buf_1.offset((n_buf - 1) as isize) } as i32 == 's' as i32 {
        if unsafe { *a_buf_1.offset((n_buf - 2) as isize) } as i32 ==
                'e' as i32 {
            if n_buf > 4 &&
                            unsafe { *a_buf_1.offset((n_buf - 4) as isize) } as i32 ==
                                's' as i32 &&
                        unsafe { *a_buf_1.offset((n_buf - 3) as isize) } as i32 ==
                            's' as i32 ||
                    n_buf > 3 &&
                        unsafe { *a_buf_1.offset((n_buf - 3) as isize) } as i32 ==
                            'i' as i32 {
                *pn_buf_1 = n_buf - 2;
            } else { *pn_buf_1 = n_buf - 1; }
        } else if unsafe { *a_buf_1.offset((n_buf - 2) as isize) } as i32 !=
                's' as i32 {
            *pn_buf_1 = n_buf - 1;
        }
    }
}

extern "C" fn fts5_porter_is_vowel(c: i8, b_y_is_vowel_1: i32) -> i32 {
    return (c as i32 == 'a' as i32 || c as i32 == 'e' as i32 ||
                            c as i32 == 'i' as i32 || c as i32 == 'o' as i32 ||
                    c as i32 == 'u' as i32 ||
                b_y_is_vowel_1 != 0 && c as i32 == 'y' as i32) as i32;
}

extern "C" fn fts5_porter_gobble_vc(z_stem_1: &[i8], b_prev_cons_1: i32)
    -> i32 {
    let mut i: i32 = 0;
    let mut b_cons: i32 = b_prev_cons_1;
    {
        i = 0;
        '__b26: loop {
            if !(i < z_stem_1.len() as i32) { break '__b26; }
            '__c26: loop {
                if 0 ==
                        {
                            b_cons =
                                (fts5_porter_is_vowel(z_stem_1[i as usize], b_cons) == 0) as
                                        i32 as i32;
                            b_cons
                        } {
                    break '__b26;
                }
                break '__c26;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        '__b27: loop {
            if !(i < z_stem_1.len() as i32) { break '__b27; }
            '__c27: loop {
                if {
                            b_cons =
                                (fts5_porter_is_vowel(z_stem_1[i as usize], b_cons) == 0) as
                                        i32 as i32;
                            b_cons
                        } != 0 {
                    return i + 1;
                }
                break '__c27;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

extern "C" fn fts5_porter_m_gt0(z_stem_1: *mut i8, n_stem_1: i32) -> i32 {
    return ((fts5_porter_gobble_vc(unsafe {
                                    let __p = z_stem_1 as *const i8;
                                    if __p.is_null() {
                                        &[]
                                    } else {
                                        core::slice::from_raw_parts(__p, n_stem_1 as usize)
                                    }
                                }, 0) == 0) as i32 == 0) as i32 as i32;
}

extern "C" fn fts5_porter_vowel(z_stem_1: &[i8]) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b28: loop {
            if !(i < z_stem_1.len() as i32) { break '__b28; }
            '__c28: loop {
                if fts5_porter_is_vowel(z_stem_1[i as usize], (i > 0) as i32)
                        != 0 {
                    return 1;
                }
                break '__c28;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

extern "C" fn fts5_porter_step1_b(a_buf_1: *mut i8, pn_buf_1: &mut i32)
    -> i32 {
    let mut ret: i32 = 0;
    let n_buf: i32 = *pn_buf_1;
    '__s29:
        {
        match unsafe { *a_buf_1.offset((n_buf - 2) as isize) } {
            101 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"eed".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 3) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 3) as isize)
                                    } as *mut (), c"ee".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 3 + 2;
                    }
                } else if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"ed".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    if fts5_porter_vowel(unsafe {
                                    let __p = a_buf_1 as *const i8;
                                    if __p.is_null() {
                                        &[]
                                    } else {
                                        core::slice::from_raw_parts(__p, (n_buf - 2) as usize)
                                    }
                                }) != 0 {
                        *pn_buf_1 = n_buf - 2;
                        ret = 1;
                    }
                }
            }
            110 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ing".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_vowel(unsafe {
                                    let __p = a_buf_1 as *const i8;
                                    if __p.is_null() {
                                        &[]
                                    } else {
                                        core::slice::from_raw_parts(__p, (n_buf - 3) as usize)
                                    }
                                }) != 0 {
                        *pn_buf_1 = n_buf - 3;
                        ret = 1;
                    }
                }
            }
            _ => {}
        }
    }
    return ret;
}

extern "C" fn fts5_porter_step1_b2(a_buf_1: *mut i8, pn_buf_1: &mut i32)
    -> i32 {
    let mut ret: i32 = 0;
    let n_buf: i32 = *pn_buf_1;
    '__s30:
        {
        match unsafe { *a_buf_1.offset((n_buf - 2) as isize) } {
            97 => {
                if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"at".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    unsafe {
                        memcpy(unsafe {
                                    &raw mut *a_buf_1.offset((n_buf - 2) as isize)
                                } as *mut (), c"ate".as_ptr() as *mut i8 as *const (),
                            3 as u64)
                    };
                    *pn_buf_1 = n_buf - 2 + 3;
                    ret = 1;
                }
            }
            98 => {
                if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"bl".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    unsafe {
                        memcpy(unsafe {
                                    &raw mut *a_buf_1.offset((n_buf - 2) as isize)
                                } as *mut (), c"ble".as_ptr() as *mut i8 as *const (),
                            3 as u64)
                    };
                    *pn_buf_1 = n_buf - 2 + 3;
                    ret = 1;
                }
            }
            105 => {
                if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"iz".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    unsafe {
                        memcpy(unsafe {
                                    &raw mut *a_buf_1.offset((n_buf - 2) as isize)
                                } as *mut (), c"ize".as_ptr() as *mut i8 as *const (),
                            3 as u64)
                    };
                    *pn_buf_1 = n_buf - 2 + 3;
                    ret = 1;
                }
            }
            _ => {}
        }
    }
    return ret;
}

extern "C" fn fts5_porter_m_eq1(z_stem_1: *mut i8, n_stem_1: i32) -> i32 {
    let mut n: i32 = 0;
    n =
        fts5_porter_gobble_vc(unsafe {
                let __p = z_stem_1 as *const i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_stem_1 as usize) }
            }, 0);
    if n != 0 &&
            0 ==
                fts5_porter_gobble_vc(unsafe {
                        let __p =
                            unsafe { &mut *z_stem_1.offset(n as isize) } as *const i8;
                        if __p.is_null() {
                            &[]
                        } else {
                            core::slice::from_raw_parts(__p, (n_stem_1 - n) as usize)
                        }
                    }, 1) {
        return 1;
    }
    return 0;
}

extern "C" fn fts5_porter_ostar(z_stem_1: &[i8]) -> i32 {
    if z_stem_1[(z_stem_1.len() as i32 - 1) as usize] as i32 == 'w' as i32 ||
                z_stem_1[(z_stem_1.len() as i32 - 1) as usize] as i32 ==
                    'x' as i32 ||
            z_stem_1[(z_stem_1.len() as i32 - 1) as usize] as i32 ==
                'y' as i32 {
        return 0;
    } else {
        let mut i: i32 = 0;
        let mut mask: i32 = 0;
        let mut b_cons: i32 = 0;
        {
            i = 0;
            '__b31: loop {
                if !(i < z_stem_1.len() as i32) { break '__b31; }
                '__c31: loop {
                    b_cons =
                        (fts5_porter_is_vowel(z_stem_1[i as usize], b_cons) == 0) as
                                i32 as i32;
                    if !(b_cons == 0 || b_cons == 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5Porter_Ostar".as_ptr() as *const i8,
                                c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 720,
                                c"bCons==0 || bCons==1".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    mask = (mask << 1) + b_cons & 7;
                    break '__c31;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return (mask == 5) as i32;
    }
}

extern "C" fn fts5_porter_step2(a_buf_1: *mut i8, pn_buf_1: &mut i32) -> i32 {
    let ret: i32 = 0;
    let n_buf: i32 = *pn_buf_1;
    '__s32:
        {
        match unsafe { *a_buf_1.offset((n_buf - 2) as isize) } {
            97 => {
                if n_buf > 7 &&
                        0 ==
                            unsafe {
                                memcmp(c"ational".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 7) as isize) } as
                                        *const (), 7 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 7) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 7) as isize)
                                    } as *mut (), c"ate".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 7 + 3;
                    }
                } else if n_buf > 6 &&
                        0 ==
                            unsafe {
                                memcmp(c"tional".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 6) as isize) } as
                                        *const (), 6 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 6) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 6) as isize)
                                    } as *mut (), c"tion".as_ptr() as *mut i8 as *const (),
                                4 as u64)
                        };
                        *pn_buf_1 = n_buf - 6 + 4;
                    }
                }
            }
            99 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"enci".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"ence".as_ptr() as *mut i8 as *const (),
                                4 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 4;
                    }
                } else if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"anci".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"ance".as_ptr() as *mut i8 as *const (),
                                4 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 4;
                    }
                }
            }
            101 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"izer".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"ize".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 3;
                    }
                }
            }
            103 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"logi".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"log".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 3;
                    }
                }
            }
            108 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"bli".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 3) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 3) as isize)
                                    } as *mut (), c"ble".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 3 + 3;
                    }
                } else if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"alli".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"al".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 2;
                    }
                } else if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"entli".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"ent".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 3;
                    }
                } else if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"eli".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 3) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 3) as isize)
                                    } as *mut (), c"e".as_ptr() as *mut i8 as *const (),
                                1 as u64)
                        };
                        *pn_buf_1 = n_buf - 3 + 1;
                    }
                } else if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"ousli".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"ous".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 3;
                    }
                }
            }
            111 => {
                if n_buf > 7 &&
                        0 ==
                            unsafe {
                                memcmp(c"ization".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 7) as isize) } as
                                        *const (), 7 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 7) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 7) as isize)
                                    } as *mut (), c"ize".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 7 + 3;
                    }
                } else if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"ation".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"ate".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 3;
                    }
                } else if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ator".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"ate".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 3;
                    }
                }
            }
            115 => {
                if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"alism".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"al".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 2;
                    }
                } else if n_buf > 7 &&
                        0 ==
                            unsafe {
                                memcmp(c"iveness".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 7) as isize) } as
                                        *const (), 7 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 7) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 7) as isize)
                                    } as *mut (), c"ive".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 7 + 3;
                    }
                } else if n_buf > 7 &&
                        0 ==
                            unsafe {
                                memcmp(c"fulness".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 7) as isize) } as
                                        *const (), 7 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 7) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 7) as isize)
                                    } as *mut (), c"ful".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 7 + 3;
                    }
                } else if n_buf > 7 &&
                        0 ==
                            unsafe {
                                memcmp(c"ousness".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 7) as isize) } as
                                        *const (), 7 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 7) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 7) as isize)
                                    } as *mut (), c"ous".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 7 + 3;
                    }
                }
            }
            116 => {
                if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"aliti".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"al".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 2;
                    }
                } else if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"iviti".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"ive".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 3;
                    }
                } else if n_buf > 6 &&
                        0 ==
                            unsafe {
                                memcmp(c"biliti".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 6) as isize) } as
                                        *const (), 6 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 6) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 6) as isize)
                                    } as *mut (), c"ble".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        };
                        *pn_buf_1 = n_buf - 6 + 3;
                    }
                }
            }
            _ => {}
        }
    }
    return ret;
}

extern "C" fn fts5_porter_step3(a_buf_1: *mut i8, pn_buf_1: &mut i32) -> i32 {
    let ret: i32 = 0;
    let n_buf: i32 = *pn_buf_1;
    '__s33:
        {
        match unsafe { *a_buf_1.offset((n_buf - 2) as isize) } {
            97 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ical".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 4) as isize)
                                    } as *mut (), c"ic".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 4 + 2;
                    }
                }
            }
            115 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ness".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 4) != 0 {
                        *pn_buf_1 = n_buf - 4;
                    }
                }
            }
            116 => {
                if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"icate".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"ic".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 2;
                    }
                } else if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"iciti".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"ic".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 2;
                    }
                }
            }
            117 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ful".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            118 => {
                if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"ative".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        *pn_buf_1 = n_buf - 5;
                    }
                }
            }
            122 => {
                if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"alize".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt0(a_buf_1, n_buf - 5) != 0 {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_buf_1.offset((n_buf - 5) as isize)
                                    } as *mut (), c"al".as_ptr() as *mut i8 as *const (),
                                2 as u64)
                        };
                        *pn_buf_1 = n_buf - 5 + 2;
                    }
                }
            }
            _ => {}
        }
    }
    return ret;
}

extern "C" fn fts5_porter_m_gt1(z_stem_1: *mut i8, n_stem_1: i32) -> i32 {
    let mut n: i32 = 0;
    n =
        fts5_porter_gobble_vc(unsafe {
                let __p = z_stem_1 as *const i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_stem_1 as usize) }
            }, 0);
    if n != 0 &&
            fts5_porter_gobble_vc(unsafe {
                        let __p =
                            unsafe { &mut *z_stem_1.offset(n as isize) } as *const i8;
                        if __p.is_null() {
                            &[]
                        } else {
                            core::slice::from_raw_parts(__p, (n_stem_1 - n) as usize)
                        }
                    }, 1) != 0 {
        return 1;
    }
    return 0;
}

extern "C" fn fts5_porter_m_gt1_and_s_or_t(z_stem_1: *mut i8, n_stem_1: i32)
    -> i32 {
    if !(n_stem_1 > 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5Porter_MGt1_and_S_or_T".as_ptr() as *const i8,
                c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 729,
                c"nStem>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return ((unsafe { *z_stem_1.offset((n_stem_1 - 1) as isize) } as i32 ==
                        's' as i32 ||
                    unsafe { *z_stem_1.offset((n_stem_1 - 1) as isize) } as i32
                        == 't' as i32) &&
                fts5_porter_m_gt1(z_stem_1, n_stem_1) != 0) as i32;
}

extern "C" fn fts5_porter_step4(a_buf_1: *mut i8, pn_buf_1: &mut i32) -> i32 {
    let ret: i32 = 0;
    let n_buf: i32 = *pn_buf_1;
    '__s34:
        {
        match unsafe { *a_buf_1.offset((n_buf - 2) as isize) } {
            97 => {
                if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"al".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 2) != 0 {
                        *pn_buf_1 = n_buf - 2;
                    }
                }
            }
            99 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ance".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 4) != 0 {
                        *pn_buf_1 = n_buf - 4;
                    }
                } else if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ence".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 4) != 0 {
                        *pn_buf_1 = n_buf - 4;
                    }
                }
            }
            101 => {
                if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"er".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 2) != 0 {
                        *pn_buf_1 = n_buf - 2;
                    }
                }
            }
            105 => {
                if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"ic".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 2) != 0 {
                        *pn_buf_1 = n_buf - 2;
                    }
                }
            }
            108 => {
                if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"able".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 4) != 0 {
                        *pn_buf_1 = n_buf - 4;
                    }
                } else if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ible".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 4) != 0 {
                        *pn_buf_1 = n_buf - 4;
                    }
                }
            }
            110 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ant".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                } else if n_buf > 5 &&
                        0 ==
                            unsafe {
                                memcmp(c"ement".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 5) as isize) } as
                                        *const (), 5 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 5) != 0 {
                        *pn_buf_1 = n_buf - 5;
                    }
                } else if n_buf > 4 &&
                        0 ==
                            unsafe {
                                memcmp(c"ment".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 4) as isize) } as
                                        *const (), 4 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 4) != 0 {
                        *pn_buf_1 = n_buf - 4;
                    }
                } else if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ent".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            111 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ion".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1_and_s_or_t(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                } else if n_buf > 2 &&
                        0 ==
                            unsafe {
                                memcmp(c"ou".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 2) as isize) } as
                                        *const (), 2 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 2) != 0 {
                        *pn_buf_1 = n_buf - 2;
                    }
                }
            }
            115 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ism".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            116 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ate".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                } else if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"iti".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            117 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ous".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            118 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ive".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            122 => {
                if n_buf > 3 &&
                        0 ==
                            unsafe {
                                memcmp(c"ize".as_ptr() as *mut i8 as *const (),
                                    unsafe { &raw mut *a_buf_1.offset((n_buf - 3) as isize) } as
                                        *const (), 3 as u64)
                            } {
                    if fts5_porter_m_gt1(a_buf_1, n_buf - 3) != 0 {
                        *pn_buf_1 = n_buf - 3;
                    }
                }
            }
            _ => {}
        }
    }
    return ret;
}

extern "C" fn fts5_porter_cb(p_ctx_1: *mut (), tflags: i32,
    p_token_1: *const i8, n_token_1: i32, i_start_1: i32, i_end_1: i32)
    -> i32 {
    let p: *const PorterContext =
        p_ctx_1 as *mut PorterContext as *const PorterContext;
    '__b35: loop {
        '__c35: loop {
            let mut a_buf: *mut i8 = core::ptr::null_mut();
            let mut n_buf: i32 = 0;
            if n_token_1 > 64 || n_token_1 < 3 { break '__b35; }
            a_buf = unsafe { (*p).a_buf };
            n_buf = n_token_1;
            unsafe {
                memcpy(a_buf as *mut (), p_token_1 as *const (), n_buf as u64)
            };
            fts5_porter_step1_a(a_buf as *const i8, &mut n_buf);
            if fts5_porter_step1_b(a_buf, &mut n_buf) != 0 {
                if fts5_porter_step1_b2(a_buf, &mut n_buf) == 0 {
                    let c: i8 = unsafe { *a_buf.offset((n_buf - 1) as isize) };
                    if fts5_porter_is_vowel(c, 0) == 0 && c as i32 != 'l' as i32
                                    && c as i32 != 's' as i32 && c as i32 != 'z' as i32 &&
                            c as i32 ==
                                unsafe { *a_buf.offset((n_buf - 2) as isize) } as i32 {
                        { let __p = &mut n_buf; let __t = *__p; *__p -= 1; __t };
                    } else if fts5_porter_m_eq1(a_buf, n_buf) != 0 &&
                            fts5_porter_ostar(unsafe {
                                        let __p = a_buf as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_buf as usize) }
                                    }) != 0 {
                        unsafe {
                            *a_buf.offset({
                                                let __p = &mut n_buf;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = 'e' as i32 as i8
                        };
                    }
                }
            }
            if unsafe { *a_buf.offset((n_buf - 1) as isize) } as i32 ==
                        'y' as i32 &&
                    fts5_porter_vowel(unsafe {
                                let __p = a_buf as *const i8;
                                if __p.is_null() {
                                    &[]
                                } else {
                                    core::slice::from_raw_parts(__p, (n_buf - 1) as usize)
                                }
                            }) != 0 {
                unsafe {
                    *a_buf.offset((n_buf - 1) as isize) = 'i' as i32 as i8
                };
            }
            fts5_porter_step2(a_buf, &mut n_buf);
            fts5_porter_step3(a_buf, &mut n_buf);
            fts5_porter_step4(a_buf, &mut n_buf);
            if !(n_buf > 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"fts5PorterCb".as_ptr() as *const i8,
                        c"fts5_tokenize.c".as_ptr() as *mut i8 as *const i8, 1231,
                        c"nBuf>0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if unsafe { *a_buf.offset((n_buf - 1) as isize) } as i32 ==
                    'e' as i32 {
                if fts5_porter_m_gt1(a_buf, n_buf - 1) != 0 ||
                        fts5_porter_m_eq1(a_buf, n_buf - 1) != 0 &&
                            (fts5_porter_ostar(unsafe {
                                                let __p = a_buf as *const i8;
                                                if __p.is_null() {
                                                    &[]
                                                } else {
                                                    core::slice::from_raw_parts(__p, (n_buf - 1) as usize)
                                                }
                                            }) == 0) as i32 != 0 {
                    { let __p = &mut n_buf; let __t = *__p; *__p -= 1; __t };
                }
            }
            if n_buf > 1 &&
                            unsafe { *a_buf.offset((n_buf - 1) as isize) } as i32 ==
                                'l' as i32 &&
                        unsafe { *a_buf.offset((n_buf - 2) as isize) } as i32 ==
                            'l' as i32 && fts5_porter_m_gt1(a_buf, n_buf - 1) != 0 {
                { let __p = &mut n_buf; let __t = *__p; *__p -= 1; __t };
            }
            return unsafe {
                    (unsafe {
                            (*p).x_token.unwrap()
                        })(unsafe { (*p).p_ctx }, tflags, a_buf as *const i8, n_buf,
                        i_start_1, i_end_1)
                };
            break '__c35;
        }
        if !(false) { break '__b35; }
    }
    return unsafe {
            (unsafe {
                    (*p).x_token.unwrap()
                })(unsafe { (*p).p_ctx }, tflags, p_token_1, n_token_1,
                i_start_1, i_end_1)
        };
}

extern "C" fn fts5_porter_tokenize(p_tokenizer_1: *mut Fts5Tokenizer,
    p_ctx_1: *mut (), flags: i32, p_text_1: *const i8, n_text_1: i32,
    p_loc_1: *const i8, n_loc_1: i32,
    x_token_1:
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
    -> i32 {
    let p: *mut PorterTokenizer = p_tokenizer_1 as *mut PorterTokenizer;
    let mut s_ctx: PorterContext = unsafe { core::mem::zeroed() };
    s_ctx.x_token =
        if x_token_1 !=
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32)
                                -> i32>(0 as *const ())
                } {
            Some(x_token_1)
        } else { None };
    s_ctx.p_ctx = p_ctx_1;
    s_ctx.a_buf = unsafe { &raw mut (*p).a_buf[0 as usize] } as *mut i8;
    return unsafe {
            (unsafe {
                    (*p).tokenizer_v2.x_tokenize.unwrap()
                })(unsafe { (*p).p_tokenizer }, &raw mut s_ctx as *mut (),
                flags, p_text_1, n_text_1, p_loc_1, n_loc_1, fts5_porter_cb)
        };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_tokenizer_init(p_api: *mut Fts5Api) -> i32 {
    let mut a_builtin: [BuiltinTokenizerN16BuiltinTokenizer; 3] =
        [BuiltinTokenizerN16BuiltinTokenizer {
                    z_name: c"unicode61".as_ptr() as *const i8,
                    x: fts5_tokenizer {
                        x_create: Some(fts5_unicode_create),
                        x_delete: Some(fts5_unicode_delete),
                        x_tokenize: Some(fts5_unicode_tokenize),
                    },
                },
                BuiltinTokenizerN16BuiltinTokenizer {
                    z_name: c"ascii".as_ptr() as *const i8,
                    x: fts5_tokenizer {
                        x_create: Some(fts5_ascii_create),
                        x_delete: Some(fts5_ascii_delete),
                        x_tokenize: Some(fts5_ascii_tokenize),
                    },
                },
                BuiltinTokenizerN16BuiltinTokenizer {
                    z_name: c"trigram".as_ptr() as *const i8,
                    x: fts5_tokenizer {
                        x_create: Some(fts5_tri_create),
                        x_delete: Some(fts5_tri_delete),
                        x_tokenize: Some(fts5_tri_tokenize),
                    },
                }];
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b36: loop {
            if !(rc == 0 &&
                            i <
                                (core::mem::size_of::<[BuiltinTokenizerN16BuiltinTokenizer; 3]>()
                                            as u64 /
                                        core::mem::size_of::<BuiltinTokenizerN16BuiltinTokenizer>()
                                            as u64) as i32) {
                break '__b36;
            }
            '__c36: loop {
                rc =
                    unsafe {
                        (unsafe {
                                (*p_api).x_create_tokenizer.unwrap()
                            })(p_api, a_builtin[i as usize].z_name, p_api as *mut (),
                            &mut a_builtin[i as usize].x,
                            unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
                            })
                    };
                break '__c36;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 {
        let mut s_porter: Fts5TokenizerV2 =
            Fts5TokenizerV2 {
                i_version: 2,
                x_create: Some(fts5_porter_create),
                x_delete: Some(fts5_porter_delete),
                x_tokenize: Some(fts5_porter_tokenize),
            };
        rc =
            unsafe {
                (unsafe {
                        (*p_api).x_create_tokenizer_v2.unwrap()
                    })(p_api, c"porter".as_ptr() as *mut i8 as *const i8,
                    p_api as *mut (), &mut s_porter,
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
                    })
            };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_tokenizer_pattern(x_create:
        Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
            *mut *mut Fts5Tokenizer) -> i32>, p_tok: *mut Fts5Tokenizer)
    -> i32 {
    if x_create == Some(fts5_tri_create) {
        let p: *const TrigramTokenizer =
            p_tok as *mut TrigramTokenizer as *const TrigramTokenizer;
        if unsafe { (*p).i_fold_param } == 0 {
            return if unsafe { (*p).b_fold } != 0 { 65 } else { 66 };
        }
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_tokenizer_preload(p: &Fts5TokenizerConfig)
    -> i32 {
    return ((*p).n_arg >= 1 &&
                0 ==
                    unsafe {
                        sqlite3_stricmp(unsafe { *(*p).az_arg.offset(0 as isize) },
                            c"trigram".as_ptr() as *mut i8 as *const i8)
                    }) as i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct PorterRule {
    z_suffix: *const i8,
    n_suffix: i32,
    x_cond: Option<unsafe extern "C" fn(*mut i8, i32) -> i32>,
    z_output: *const i8,
    n_output: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct BuiltinTokenizerN16BuiltinTokenizer {
    z_name: *const i8,
    x: fts5_tokenizer,
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
    fn sqlite3_fts5_unicode_cat_parse(_: *const i8, _: *mut u8)
    -> i32;
    fn sqlite3_fts5_unicode_ascii(_: *mut u8, _: *mut u8)
    -> ();
    fn sqlite3_fts5_unicode_category(i_code_1: u32)
    -> i32;
    fn sqlite3_fts5_unicode_isdiacritic(c: i32)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_fts5_unicode_fold(c: i32, b_remove_diacritic_1: i32)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_fts5_vocab_init(_: *mut Fts5Global, _: *mut Sqlite3)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
