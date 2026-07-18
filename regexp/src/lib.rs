#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;
type DarwinSizeT = u64;
type ReStateNumber = u16;
#[repr(C)]
#[derive(Copy, Clone)]
struct ReStateSet {
    n_state: u32,
    a_state: *mut ReStateNumber,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ReInput {
    z: *const u8,
    i: i32,
    mx: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ReCompiled {
    s_in: ReInput,
    z_err: *const i8,
    a_op: *mut i8,
    a_arg: *mut i32,
    x_next_char: Option<unsafe extern "C" fn(*mut ReInput) -> u32>,
    z_init: [u8; 12],
    n_init: i32,
    n_state: u32,
    n_alloc: u32,
    mx_alloc: u32,
}
extern "C" fn re_add_state(p_set_1: &mut ReStateSet, new_state_1: i32) -> () {
    let mut i: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b0: loop {
            if !(i < (*p_set_1).n_state) { break '__b0; }
            '__c0: loop {
                if unsafe { *(*p_set_1).a_state.add(i as usize) } as i32 ==
                        new_state_1 {
                    return;
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        *(*p_set_1).a_state.add({
                            let __p = &mut (*p_set_1).n_state;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        } as usize) = new_state_1 as ReStateNumber
    };
}
extern "C" fn re_next_char(p: *mut ReInput) -> u32 {
    let mut c: u32 = 0 as u32;
    if unsafe { (*p).i } >= unsafe { (*p).mx } { return 0 as u32; }
    c =
        unsafe {
                *unsafe {
                        (*p).z.offset({
                                    let __p = unsafe { &mut (*p).i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                    }
            } as u32;
    if c >= 128 as u32 {
        if c & 224 as u32 == 192 as u32 &&
                    unsafe { (*p).i } < unsafe { (*p).mx } &&
                unsafe {
                                *unsafe { (*p).z.offset(unsafe { (*p).i } as isize) }
                            } as i32 & 192 == 128 {
            c =
                (c & 31 as u32) << 6 |
                    (unsafe {
                                    *unsafe {
                                            (*p).z.offset({
                                                        let __p = unsafe { &mut (*p).i };
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                        }
                                } as i32 & 63) as u32;
            if c < 128 as u32 { c = 65533 as u32; }
        } else if c & 240 as u32 == 224 as u32 &&
                        unsafe { (*p).i } + 1 < unsafe { (*p).mx } &&
                    unsafe {
                                    *unsafe { (*p).z.offset(unsafe { (*p).i } as isize) }
                                } as i32 & 192 == 128 &&
                unsafe {
                                *unsafe { (*p).z.offset((unsafe { (*p).i } + 1) as isize) }
                            } as i32 & 192 == 128 {
            c =
                (c & 15 as u32) << 12 |
                        ((unsafe {
                                            *unsafe { (*p).z.offset(unsafe { (*p).i } as isize) }
                                        } as i32 & 63) << 6) as u32 |
                    (unsafe {
                                    *unsafe { (*p).z.offset((unsafe { (*p).i } + 1) as isize) }
                                } as i32 & 63) as u32;
            unsafe { (*p).i += 2 };
            if c <= 2047 as u32 || c >= 55296 as u32 && c <= 57343 as u32 {
                c = 65533 as u32;
            }
        } else if c & 248 as u32 == 240 as u32 &&
                            unsafe { (*p).i } + 2 < unsafe { (*p).mx } &&
                        unsafe {
                                        *unsafe { (*p).z.offset(unsafe { (*p).i } as isize) }
                                    } as i32 & 192 == 128 &&
                    unsafe {
                                    *unsafe { (*p).z.offset((unsafe { (*p).i } + 1) as isize) }
                                } as i32 & 192 == 128 &&
                unsafe {
                                *unsafe { (*p).z.offset((unsafe { (*p).i } + 2) as isize) }
                            } as i32 & 192 == 128 {
            c =
                (c & 7 as u32) << 18 |
                            ((unsafe {
                                                *unsafe { (*p).z.offset(unsafe { (*p).i } as isize) }
                                            } as i32 & 63) << 12) as u32 |
                        ((unsafe {
                                            *unsafe { (*p).z.offset((unsafe { (*p).i } + 1) as isize) }
                                        } as i32 & 63) << 6) as u32 |
                    (unsafe {
                                    *unsafe { (*p).z.offset((unsafe { (*p).i } + 2) as isize) }
                                } as i32 & 63) as u32;
            unsafe { (*p).i += 3 };
            if c <= 65535 as u32 || c > 1114111 as u32 { c = 65533 as u32; }
        } else { c = 65533 as u32; }
    }
    return c;
}
extern "C" fn re_next_char_nocase(p: *mut ReInput) -> u32 {
    let mut c: u32 = re_next_char(p);
    if c >= 'A' as i32 as u32 && c <= 'Z' as i32 as u32 {
        c += ('a' as i32 - 'A' as i32) as u32;
    }
    return c;
}
extern "C" fn re_word_char(c: i32) -> i32 {
    return (c >= '0' as i32 && c <= '9' as i32 ||
                        c >= 'a' as i32 && c <= 'z' as i32 ||
                    c >= 'A' as i32 && c <= 'Z' as i32 || c == '_' as i32) as
            i32;
}
extern "C" fn re_digit_char(c: i32) -> i32 {
    return (c >= '0' as i32 && c <= '9' as i32) as i32;
}
extern "C" fn re_space_char(c: i32) -> i32 {
    return (c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 ||
                        c == '\r' as i32 || c == '\u{b}' as i32 ||
                c == '\u{c}' as i32) as i32;
}
extern "C" fn sqlite3re_match(p_re_1: &ReCompiled, z_in_1: *const u8,
    n_in_1: i32) -> i32 {
    let mut a_state_set: [ReStateSet; 2] = unsafe { core::mem::zeroed() };
    let mut p_this: *mut ReStateSet = core::ptr::null_mut();
    let mut p_next: *mut ReStateSet = core::ptr::null_mut();
    let mut a_space: [u16; 100] = [0; 100];
    let mut p_to_free: *mut ReStateNumber = core::ptr::null_mut();
    let mut i: u32 = 0 as u32;
    let mut i_swap: u32 = 0 as u32;
    let mut c: i32 = 0;
    let mut c_prev: i32 = 0;
    let mut rc: i32 = 0;
    let mut in_: ReInput = unsafe { core::mem::zeroed() };
    let mut x: u8 = 0 as u8;
    let mut x__1: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut hit: i32 = 0;
    let mut x__2: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s2:
            {
            match __state {
                0 => { __state = 4; }
                2 => { j = 1; __state = 106; }
                3 => {
                    unsafe { sqlite3_free(p_to_free as *mut ()) };
                    __state = 133;
                }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { i = 0 as u32; __state = 7; }
                7 => { i_swap = 0 as u32; __state = 8; }
                8 => { c = 268435455; __state = 9; }
                9 => { c_prev = 0; __state = 10; }
                10 => { rc = 0; __state = 11; }
                11 => { __state = 12; }
                12 => { in_.z = z_in_1; __state = 13; }
                13 => { in_.i = 0; __state = 14; }
                14 => {
                    in_.mx =
                        if n_in_1 >= 0 {
                            n_in_1
                        } else { (unsafe { strlen(z_in_1 as *const i8) }) as i32 };
                    __state = 15;
                }
                15 => {
                    if (*p_re_1).n_init != 0 {
                        __state = 17;
                    } else { __state = 16; }
                }
                16 => {
                    if (*p_re_1).n_state as u64 <=
                            core::mem::size_of::<[u16; 100]>() as u64 /
                                (core::mem::size_of::<ReStateNumber>() as u64 * 2 as u64) {
                        __state = 24;
                    } else { __state = 25; }
                }
                17 => { x = (*p_re_1).z_init[0 as usize]; __state = 18; }
                18 => {
                    if in_.i + (*p_re_1).n_init <= in_.mx &&
                            (unsafe { *z_in_1.offset(in_.i as isize) } as i32 !=
                                    x as i32 ||
                                unsafe {
                                        strncmp(unsafe {
                                                (z_in_1 as *const i8).offset(in_.i as isize)
                                            }, &raw const (*p_re_1).z_init[0 as usize] as *const i8,
                                            (*p_re_1).n_init as u64)
                                    } != 0) {
                        __state = 20;
                    } else { __state = 19; }
                }
                19 => {
                    if in_.i + (*p_re_1).n_init > in_.mx {
                        __state = 22;
                    } else { __state = 21; }
                }
                20 => {
                    { let __p = &mut in_.i; let __t = *__p; *__p += 1; __t };
                    __state = 18;
                }
                21 => { c = 268435455 - 1; __state = 16; }
                22 => { return 0; }
                23 => {
                    a_state_set[1 as usize].a_state =
                        unsafe {
                            a_state_set[0 as
                                            usize].a_state.add((*p_re_1).n_state as usize)
                        };
                    __state = 30;
                }
                24 => { p_to_free = core::ptr::null_mut(); __state = 26; }
                25 => {
                    p_to_free =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<ReStateNumber>() as
                                                u64 * 2 as u64 * (*p_re_1).n_state as u64)
                            } as *mut ReStateNumber;
                    __state = 27;
                }
                26 => {
                    a_state_set[0 as usize].a_state =
                        &raw mut a_space[0 as usize] as *mut ReStateNumber;
                    __state = 23;
                }
                27 => {
                    if p_to_free == core::ptr::null_mut() {
                        __state = 29;
                    } else { __state = 28; }
                }
                28 => {
                    a_state_set[0 as usize].a_state = p_to_free;
                    __state = 23;
                }
                29 => { return -1; }
                30 => { p_next = &mut a_state_set[1 as usize]; __state = 31; }
                31 => {
                    unsafe { (*p_next).n_state = 0 as u32 };
                    __state = 32;
                }
                32 => {
                    re_add_state(unsafe { &mut *p_next }, 0);
                    __state = 33;
                }
                33 => {
                    if c != 0 && unsafe { (*p_next).n_state } > 0 as u32 {
                        __state = 35;
                    } else { __state = 34; }
                }
                34 => { i = 0 as u32; __state = 125; }
                35 => { c_prev = c; __state = 36; }
                36 => {
                    c =
                        unsafe { (*p_re_1).x_next_char.unwrap()(&mut in_) } as i32;
                    __state = 37;
                }
                37 => { p_this = p_next; __state = 38; }
                38 => {
                    p_next = &mut a_state_set[i_swap as usize];
                    __state = 39;
                }
                39 => { i_swap = 1 as u32 - i_swap; __state = 40; }
                40 => {
                    unsafe { (*p_next).n_state = 0 as u32 };
                    __state = 41;
                }
                41 => { i = 0 as u32; __state = 42; }
                42 => {
                    if i < unsafe { (*p_this).n_state } {
                        __state = 43;
                    } else { __state = 33; }
                }
                43 => {
                    x__1 =
                        unsafe { *unsafe { (*p_this).a_state.add(i as usize) } } as
                            i32;
                    __state = 45;
                }
                44 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 42;
                }
                45 => {
                    '__s3:
                        {
                        match unsafe { *(*p_re_1).a_op.offset(x__1 as isize) } {
                            1 => { __state = 46; }
                            18 => { __state = 47; }
                            2 => { __state = 48; }
                            11 => { __state = 49; }
                            12 => { __state = 50; }
                            13 => { __state = 51; }
                            14 => { __state = 52; }
                            15 => { __state = 53; }
                            16 => { __state = 54; }
                            17 => { __state = 55; }
                            3 => { __state = 56; }
                            4 => { __state = 57; }
                            5 => { __state = 58; }
                            6 => { __state = 59; }
                            8 => { __state = 60; }
                            7 => { __state = 61; }
                            _ => { __state = 44; }
                        }
                    }
                }
                46 => {
                    if unsafe { *(*p_re_1).a_arg.offset(x__1 as isize) } == c {
                        __state = 65;
                    } else { __state = 64; }
                }
                47 => {
                    if c_prev == 268435455 {
                        __state = 68;
                    } else { __state = 67; }
                }
                48 => { if c != 0 { __state = 71; } else { __state = 70; } }
                49 => {
                    if re_word_char(c) != 0 {
                        __state = 74;
                    } else { __state = 73; }
                }
                50 => {
                    if (re_word_char(c) == 0) as i32 != 0 && c != 0 {
                        __state = 77;
                    } else { __state = 76; }
                }
                51 => {
                    if re_digit_char(c) != 0 {
                        __state = 80;
                    } else { __state = 79; }
                }
                52 => {
                    if (re_digit_char(c) == 0) as i32 != 0 && c != 0 {
                        __state = 83;
                    } else { __state = 82; }
                }
                53 => {
                    if re_space_char(c) != 0 {
                        __state = 86;
                    } else { __state = 85; }
                }
                54 => {
                    if (re_space_char(c) == 0) as i32 != 0 && c != 0 {
                        __state = 89;
                    } else { __state = 88; }
                }
                55 => {
                    if re_word_char(c) != re_word_char(c_prev) {
                        __state = 92;
                    } else { __state = 91; }
                }
                56 => {
                    re_add_state(unsafe { &mut *p_next }, x__1);
                    __state = 94;
                }
                57 => {
                    re_add_state(unsafe { &mut *p_this },
                        x__1 + unsafe { *(*p_re_1).a_arg.offset(x__1 as isize) });
                    __state = 97;
                }
                58 => {
                    re_add_state(unsafe { &mut *p_this },
                        x__1 + unsafe { *(*p_re_1).a_arg.offset(x__1 as isize) });
                    __state = 100;
                }
                59 => { rc = 1; __state = 102; }
                60 => { if c == 0 { __state = 105; } else { __state = 104; } }
                61 => { __state = 2; }
                62 => { __state = 46; }
                63 => { __state = 47; }
                64 => { __state = 44; }
                65 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 64;
                }
                66 => { __state = 48; }
                67 => { __state = 44; }
                68 => {
                    re_add_state(unsafe { &mut *p_this }, x__1 + 1);
                    __state = 67;
                }
                69 => { __state = 49; }
                70 => { __state = 44; }
                71 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 70;
                }
                72 => { __state = 50; }
                73 => { __state = 44; }
                74 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 73;
                }
                75 => { __state = 51; }
                76 => { __state = 44; }
                77 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 76;
                }
                78 => { __state = 52; }
                79 => { __state = 44; }
                80 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 79;
                }
                81 => { __state = 53; }
                82 => { __state = 44; }
                83 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 82;
                }
                84 => { __state = 54; }
                85 => { __state = 44; }
                86 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 85;
                }
                87 => { __state = 55; }
                88 => { __state = 44; }
                89 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + 1);
                    __state = 88;
                }
                90 => { __state = 56; }
                91 => { __state = 44; }
                92 => {
                    re_add_state(unsafe { &mut *p_this }, x__1 + 1);
                    __state = 91;
                }
                93 => { __state = 57; }
                94 => {
                    re_add_state(unsafe { &mut *p_this }, x__1 + 1);
                    __state = 95;
                }
                95 => { __state = 44; }
                96 => { __state = 58; }
                97 => {
                    re_add_state(unsafe { &mut *p_this }, x__1 + 1);
                    __state = 98;
                }
                98 => { __state = 44; }
                99 => { __state = 59; }
                100 => { __state = 44; }
                101 => { __state = 60; }
                102 => { __state = 3; }
                103 => { __state = 61; }
                104 => { __state = 2; }
                105 => { __state = 44; }
                106 => {
                    n = unsafe { *(*p_re_1).a_arg.offset(x__1 as isize) };
                    __state = 107;
                }
                107 => { hit = 0; __state = 108; }
                108 => { j = 1; __state = 110; }
                109 => {
                    if unsafe { *(*p_re_1).a_op.offset(x__1 as isize) } as i32
                            == 8 {
                        __state = 121;
                    } else { __state = 120; }
                }
                110 => {
                    if j > 0 && j < n { __state = 111; } else { __state = 109; }
                }
                111 => {
                    if unsafe { *(*p_re_1).a_op.offset((x__1 + j) as isize) } as
                                i32 == 9 {
                        __state = 113;
                    } else { __state = 114; }
                }
                112 => {
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    __state = 110;
                }
                113 => {
                    if unsafe { *(*p_re_1).a_arg.offset((x__1 + j) as isize) }
                            == c {
                        __state = 115;
                    } else { __state = 112; }
                }
                114 => {
                    if unsafe { *(*p_re_1).a_arg.offset((x__1 + j) as isize) }
                                <= c &&
                            unsafe { *(*p_re_1).a_arg.offset((x__1 + j + 1) as isize) }
                                >= c {
                        __state = 117;
                    } else { __state = 118; }
                }
                115 => { hit = 1; __state = 116; }
                116 => { j = -1; __state = 112; }
                117 => { hit = 1; __state = 119; }
                118 => {
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    __state = 112;
                }
                119 => { j = -1; __state = 112; }
                120 => {
                    if hit != 0 { __state = 123; } else { __state = 122; }
                }
                121 => { hit = (hit == 0) as i32 as i32; __state = 120; }
                122 => { __state = 44; }
                123 => {
                    re_add_state(unsafe { &mut *p_next }, x__1 + n);
                    __state = 122;
                }
                124 => { __state = 3; }
                125 => {
                    if i < unsafe { (*p_next).n_state } {
                        __state = 126;
                    } else { __state = 124; }
                }
                126 => {
                    x__2 =
                        unsafe { *unsafe { (*p_next).a_state.add(i as usize) } } as
                            i32;
                    __state = 128;
                }
                127 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 125;
                }
                128 => {
                    if unsafe { *(*p_re_1).a_op.offset(x__2 as isize) } as i32
                            == 5 {
                        __state = 130;
                    } else { __state = 129; }
                }
                129 => {
                    if unsafe { *(*p_re_1).a_op.offset(x__2 as isize) } as i32
                            == 6 {
                        __state = 131;
                    } else { __state = 127; }
                }
                130 => {
                    x__2 += unsafe { *(*p_re_1).a_arg.offset(x__2 as isize) };
                    __state = 128;
                }
                131 => { rc = 1; __state = 132; }
                132 => { __state = 124; }
                133 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn re_resize(p: &mut ReCompiled, n_1: u32) -> i32 {
    let mut a_op: *mut i8 = core::ptr::null_mut();
    let mut a_arg: *mut i32 = core::ptr::null_mut();
    if n_1 > (*p).mx_alloc {
        (*p).z_err =
            c"REGEXP pattern too big".as_ptr() as *mut i8 as *const i8;
        return 1;
    }
    a_op =
        unsafe {
                sqlite3_realloc64((*p).a_op as *mut (),
                    n_1 as u64 * core::mem::size_of::<i8>() as u64)
            } as *mut i8;
    if a_op == core::ptr::null_mut() {
        (*p).z_err = c"out of memory".as_ptr() as *mut i8 as *const i8;
        return 1;
    }
    (*p).a_op = a_op;
    a_arg =
        unsafe {
                sqlite3_realloc64((*p).a_arg as *mut (),
                    n_1 as u64 * core::mem::size_of::<i32>() as u64)
            } as *mut i32;
    if a_arg == core::ptr::null_mut() {
        (*p).z_err = c"out of memory".as_ptr() as *mut i8 as *const i8;
        return 1;
    }
    (*p).a_arg = a_arg;
    (*p).n_alloc = n_1;
    return 0;
}
extern "C" fn re_insert(p: *mut ReCompiled, i_before_1: i32, op: i32,
    arg: i32) -> i32 {
    let mut i: i32 = 0;
    if unsafe { (*p).n_alloc } <= unsafe { (*p).n_state } &&
            re_resize(unsafe { &mut *p }, unsafe { (*p).n_alloc } * 2 as u32)
                != 0 {
        return 0;
    }
    {
        i = unsafe { (*p).n_state } as i32;
        '__b4: loop {
            if !(i > i_before_1) { break '__b4; }
            '__c4: loop {
                unsafe {
                    *unsafe { (*p).a_op.offset(i as isize) } =
                        unsafe { *unsafe { (*p).a_op.offset((i - 1) as isize) } }
                };
                unsafe {
                    *unsafe { (*p).a_arg.offset(i as isize) } =
                        unsafe { *unsafe { (*p).a_arg.offset((i - 1) as isize) } }
                };
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    {
        let __p = unsafe { &mut (*p).n_state };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { *unsafe { (*p).a_op.offset(i_before_1 as isize) } = op as i8 };
    unsafe { *unsafe { (*p).a_arg.offset(i_before_1 as isize) } = arg };
    return i_before_1;
}
extern "C" fn re_append(p: *mut ReCompiled, op: i32, arg: i32) -> i32 {
    return re_insert(p, unsafe { (*p).n_state } as i32, op, arg);
}
extern "C" fn re_copy(p: *mut ReCompiled, i_start_1: i32, n_1: u32) -> () {
    if unsafe { (*p).n_state } + n_1 >= unsafe { (*p).n_alloc } &&
            re_resize(unsafe { &mut *p },
                    unsafe { (*p).n_alloc } * 2 as u32 + n_1) != 0 {
        return;
    }
    unsafe {
        memcpy(unsafe {
                    &raw mut *unsafe {
                                (*p).a_op.add(unsafe { (*p).n_state } as usize)
                            }
                } as *mut (),
            unsafe {
                    &raw mut *unsafe { (*p).a_op.offset(i_start_1 as isize) }
                } as *const (),
            n_1 as u64 * core::mem::size_of::<i8>() as u64)
    };
    unsafe {
        memcpy(unsafe {
                    &raw mut *unsafe {
                                (*p).a_arg.add(unsafe { (*p).n_state } as usize)
                            }
                } as *mut (),
            unsafe {
                    &raw mut *unsafe { (*p).a_arg.offset(i_start_1 as isize) }
                } as *const (),
            n_1 as u64 * core::mem::size_of::<i32>() as u64)
    };
    unsafe { (*p).n_state += n_1 };
}
extern "C" fn re_hex(mut c: i32, p_v_1: &mut i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        c -= '0' as i32;
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        c -= 'a' as i32 - 10;
    } else if c >= 'A' as i32 && c <= 'F' as i32 {
        c -= 'A' as i32 - 10;
    } else { return 0; }
    *p_v_1 = *p_v_1 * 16 + (c & 255);
    return 1;
}
extern "C" fn re_esc_char(p: &mut ReCompiled) -> u32 {
    let mut i: i32 = 0;
    let mut v: i32 = 0;
    let mut c: i8 = 0 as i8;
    if (*p).s_in.i >= (*p).s_in.mx { return 0 as u32; }
    c = unsafe { *(*p).s_in.z.offset((*p).s_in.i as isize) } as i8;
    if c as i32 == 'u' as i32 && (*p).s_in.i + 4 < (*p).s_in.mx {
        let z_in: *const u8 =
            unsafe { (*p).s_in.z.offset((*p).s_in.i as isize) };
        if re_hex(unsafe { *z_in.offset(1 as isize) } as i32, &mut v) != 0 &&
                        re_hex(unsafe { *z_in.offset(2 as isize) } as i32, &mut v)
                            != 0 &&
                    re_hex(unsafe { *z_in.offset(3 as isize) } as i32, &mut v)
                        != 0 &&
                re_hex(unsafe { *z_in.offset(4 as isize) } as i32, &mut v) !=
                    0 {
            (*p).s_in.i += 5;
            return v as u32;
        }
    }
    if c as i32 == 'x' as i32 && (*p).s_in.i + 2 < (*p).s_in.mx {
        let z_in_1: *const u8 =
            unsafe { (*p).s_in.z.offset((*p).s_in.i as isize) };
        if re_hex(unsafe { *z_in_1.offset(1 as isize) } as i32, &mut v) != 0
                &&
                re_hex(unsafe { *z_in_1.offset(2 as isize) } as i32, &mut v)
                    != 0 {
            (*p).s_in.i += 3;
            return v as u32;
        }
    }
    {
        i = 0;
        '__b5: loop {
            if !(z_esc[i as usize] != 0 &&
                            z_esc[i as usize] as i32 != c as i32) {
                break '__b5;
            }
            '__c5: loop { break '__c5; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if z_esc[i as usize] != 0 {
        if i < 6 { c = z_trans[i as usize] as i8; }
        { let __p = &mut (*p).s_in.i; let __t = *__p; *__p += 1; __t };
    } else {
        (*p).z_err = c"unknown \\ escape".as_ptr() as *mut i8 as *const i8;
    }
    return c as u32;
}
extern "C" fn re_peek(p: &ReCompiled) -> u8 {
    return if (*p).s_in.i < (*p).s_in.mx {
                (unsafe { *(*p).s_in.z.offset((*p).s_in.i as isize) }) as i32
            } else { 0 } as u8;
}
extern "C" fn re_subcompile_re(p: *mut ReCompiled) -> *const i8 {
    let mut z_err: *const i8 = core::ptr::null();
    let mut i_start: i32 = 0;
    let mut i_end: i32 = 0;
    let mut i_goto: i32 = 0;
    i_start = unsafe { (*p).n_state } as i32;
    z_err = re_subcompile_string(p);
    if !(z_err).is_null() { return z_err; }
    while re_peek(unsafe { &*p }) as i32 == '|' as i32 {
        i_end = unsafe { (*p).n_state } as i32;
        re_insert(p, i_start, 4, i_end + 2 - i_start);
        i_goto = re_append(p, 5, 0);
        {
            let __p = unsafe { &mut (*p).s_in.i };
            let __t = *__p;
            *__p += 1;
            __t
        };
        z_err = re_subcompile_string(p);
        if !(z_err).is_null() { return z_err; }
        unsafe {
            *unsafe { (*p).a_arg.offset(i_goto as isize) } =
                (unsafe { (*p).n_state } - i_goto as u32) as i32
        };
    }
    return core::ptr::null();
}
extern "C" fn re_subcompile_string(p: *mut ReCompiled) -> *const i8 {
    let mut i_prev: i32 = -1;
    let mut i_start: i32 = 0;
    let mut c: u32 = 0 as u32;
    let mut z_err: *const i8 = core::ptr::null();
    while {
                c =
                    unsafe {
                        (unsafe {
                                (*p).x_next_char.unwrap()
                            })(unsafe { &mut (*p).s_in })
                    };
                c
            } != 0 as u32 {
        i_start = unsafe { (*p).n_state } as i32;
        '__s8:
            {
            match c {
                124 => {
                    {
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        return core::ptr::null();
                    }
                    {
                        z_err = re_subcompile_re(p);
                        if !(z_err).is_null() { return z_err; }
                        if re_peek(unsafe { &*p }) as i32 != ')' as i32 {
                            return c"unmatched \'(\'".as_ptr() as *mut i8 as *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        break '__s8;
                    }
                    {
                        if re_peek(unsafe { &*p }) as i32 == '*' as i32 {
                            re_append(p, 3, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 2, 0); }
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'*\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 5,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state } + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'+\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state }) as i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                41 => {
                    {
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        return core::ptr::null();
                    }
                    {
                        z_err = re_subcompile_re(p);
                        if !(z_err).is_null() { return z_err; }
                        if re_peek(unsafe { &*p }) as i32 != ')' as i32 {
                            return c"unmatched \'(\'".as_ptr() as *mut i8 as *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        break '__s8;
                    }
                    {
                        if re_peek(unsafe { &*p }) as i32 == '*' as i32 {
                            re_append(p, 3, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 2, 0); }
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'*\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 5,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state } + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'+\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state }) as i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                40 => {
                    {
                        z_err = re_subcompile_re(p);
                        if !(z_err).is_null() { return z_err; }
                        if re_peek(unsafe { &*p }) as i32 != ')' as i32 {
                            return c"unmatched \'(\'".as_ptr() as *mut i8 as *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        break '__s8;
                    }
                    {
                        if re_peek(unsafe { &*p }) as i32 == '*' as i32 {
                            re_append(p, 3, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 2, 0); }
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'*\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 5,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state } + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'+\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state }) as i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                46 => {
                    {
                        if re_peek(unsafe { &*p }) as i32 == '*' as i32 {
                            re_append(p, 3, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 2, 0); }
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'*\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 5,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state } + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'+\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state }) as i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                42 => {
                    {
                        if i_prev < 0 {
                            return c"\'*\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 5,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state } + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'+\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state }) as i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                43 => {
                    {
                        if i_prev < 0 {
                            return c"\'+\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_append(p, 4,
                            (i_prev as u32 - unsafe { (*p).n_state }) as i32);
                        break '__s8;
                    }
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                63 => {
                    {
                        if i_prev < 0 {
                            return c"\'?\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        re_insert(p, i_prev, 4,
                            (unsafe { (*p).n_state } - i_prev as u32 + 1 as u32) as
                                i32);
                        break '__s8;
                    }
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                36 => {
                    { re_append(p, 1, 0); break '__s8; }
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                94 => {
                    { re_append(p, 18, 0); break '__s8; }
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                123 => {
                    {
                        let mut m: u32 = 0 as u32;
                        let mut n: u32 = 0 as u32;
                        let mut sz: u32 = 0 as u32;
                        let mut j: u32 = 0 as u32;
                        if i_prev < 0 {
                            return c"\'{m,n}\' without operand".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                    '0' as i32 as u32 && c <= '9' as i32 as u32 {
                            m = m * 10 as u32 + c - '0' as i32 as u32;
                            if m * 2 as u32 > unsafe { (*p).mx_alloc } {
                                return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                        *const i8;
                            }
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        n = m;
                        if c == ',' as i32 as u32 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            n = 0 as u32;
                            while { c = re_peek(unsafe { &*p }) as u32; c } >=
                                        '0' as i32 as u32 && c <= '9' as i32 as u32 {
                                n = n * 10 as u32 + c - '0' as i32 as u32;
                                if n * 2 as u32 > unsafe { (*p).mx_alloc } {
                                    return c"REGEXP pattern too big".as_ptr() as *mut i8 as
                                            *const i8;
                                }
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                        }
                        if c != '}' as i32 as u32 {
                            return c"unmatched \'{\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if n < m {
                            return c"n less than m in \'{m,n}\'".as_ptr() as *mut i8 as
                                    *const i8;
                        }
                        {
                            let __p = unsafe { &mut (*p).s_in.i };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        sz = unsafe { (*p).n_state } - i_prev as u32;
                        if m == 0 as u32 {
                            if n == 0 as u32 {
                                return c"both m and n are zero in \'{m,n}\'".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            re_insert(p, i_prev, 4, (sz + 1 as u32) as i32);
                            { let __p = &mut i_prev; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            {
                                j = 1 as u32;
                                '__b11: loop {
                                    if !(j < m) { break '__b11; }
                                    '__c11: loop { re_copy(p, i_prev, sz); break '__c11; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        {
                            j = m;
                            '__b12: loop {
                                if !(j < n) { break '__b12; }
                                '__c12: loop {
                                    re_append(p, 4, (sz + 1 as u32) as i32);
                                    re_copy(p, i_prev, sz);
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 as u32 && m > 0 as u32 {
                            re_append(p, 4, -(sz as i32));
                        }
                        break '__s8;
                    }
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                91 => {
                    {
                        let i_first: u32 = unsafe { (*p).n_state };
                        if re_peek(unsafe { &*p }) as i32 == '^' as i32 {
                            re_append(p, 8, 0);
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        } else { re_append(p, 7, 0); }
                        while {
                                    c =
                                        unsafe {
                                            (unsafe {
                                                    (*p).x_next_char.unwrap()
                                                })(unsafe { &mut (*p).s_in })
                                        };
                                    c
                                } != 0 as u32 {
                            if c == '[' as i32 as u32 &&
                                    re_peek(unsafe { &*p }) as i32 == ':' as i32 {
                                return c"POSIX character classes not supported".as_ptr() as
                                            *mut i8 as *const i8;
                            }
                            if c == '\\' as i32 as u32 {
                                c = re_esc_char(unsafe { &mut *p });
                            }
                            if re_peek(unsafe { &*p }) as i32 == '-' as i32 {
                                re_append(p, 10, c as i32);
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                c =
                                    unsafe {
                                        (unsafe {
                                                (*p).x_next_char.unwrap()
                                            })(unsafe { &mut (*p).s_in })
                                    };
                                if c == '\\' as i32 as u32 {
                                    c = re_esc_char(unsafe { &mut *p });
                                }
                                re_append(p, 10, c as i32);
                            } else { re_append(p, 9, c as i32); }
                            if re_peek(unsafe { &*p }) as i32 == ']' as i32 {
                                {
                                    let __p = unsafe { &mut (*p).s_in.i };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break;
                            }
                        }
                        if c == 0 as u32 {
                            return c"unclosed \'[\'".as_ptr() as *mut i8 as *const i8;
                        }
                        if unsafe { (*p).n_state } > i_first {
                            unsafe {
                                *unsafe { (*p).a_arg.add(i_first as usize) } =
                                    (unsafe { (*p).n_state } - i_first) as i32
                            };
                        }
                        break '__s8;
                    }
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                92 => {
                    {
                        let mut special_op: i32 = 0;
                        '__s14:
                            {
                            match re_peek(unsafe { &*p }) {
                                98 => { special_op = 17; }
                                100 => { special_op = 13; }
                                68 => { special_op = 14; }
                                115 => { special_op = 15; }
                                83 => { special_op = 16; }
                                119 => { special_op = 11; }
                                87 => { special_op = 12; }
                                _ => {}
                            }
                        }
                        if special_op != 0 {
                            {
                                let __p = unsafe { &mut (*p).s_in.i };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            re_append(p, special_op, 0);
                        } else {
                            c = re_esc_char(unsafe { &mut *p });
                            re_append(p, 1, c as i32);
                        }
                        break '__s8;
                    }
                    { re_append(p, 1, c as i32); break '__s8; }
                }
                _ => { { re_append(p, 1, c as i32); break '__s8; } }
            }
        }
        i_prev = i_start;
    }
    return core::ptr::null();
}
extern "C" fn sqlite3re_free(p_re_1: *mut ReCompiled) -> () {
    if !(p_re_1).is_null() {
        unsafe { sqlite3_free(unsafe { (*p_re_1).a_op } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p_re_1).a_arg } as *mut ()) };
        unsafe { sqlite3_free(p_re_1 as *mut ()) };
    }
}
extern "C" fn re_free_voidptr(p: *mut ()) -> () {
    sqlite3re_free(p as *mut ReCompiled);
}
extern "C" fn sqlite3re_compile(pp_re_1: &mut *mut ReCompiled,
    mut z_in_1: *const i8, mx_re_1: i32, no_case_1: i32) -> *const i8 {
    let mut p_re: *mut ReCompiled = core::ptr::null_mut();
    let mut z_err: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    *pp_re_1 = core::ptr::null_mut();
    p_re =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<ReCompiled>() as
                        Sqlite3Uint64)
            } as *mut ReCompiled;
    if p_re == core::ptr::null_mut() {
        return c"out of memory".as_ptr() as *mut i8 as *const i8;
    }
    unsafe {
        memset(p_re as *mut (), 0, core::mem::size_of::<ReCompiled>() as u64)
    };
    unsafe {
        (*p_re).x_next_char =
            if no_case_1 != 0 {
                Some(re_next_char_nocase)
            } else { Some(re_next_char) }
    };
    unsafe { (*p_re).mx_alloc = mx_re_1 as u32 };
    if re_resize(unsafe { &mut *p_re }, 30 as u32) != 0 {
        z_err = unsafe { (*p_re).z_err };
        sqlite3re_free(p_re);
        return z_err;
    }
    if unsafe { *z_in_1.offset(0 as isize) } as i32 == '^' as i32 {
        {
            let __p = &mut z_in_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    } else { re_append(p_re, 3, 0); }
    unsafe { (*p_re).s_in.z = z_in_1 as *mut u8 as *const u8 };
    unsafe { (*p_re).s_in.i = 0 };
    unsafe { (*p_re).s_in.mx = unsafe { strlen(z_in_1) } as i32 };
    z_err = re_subcompile_re(p_re);
    if !(z_err).is_null() { sqlite3re_free(p_re); return z_err; }
    if unsafe { (*p_re).s_in.i } >= unsafe { (*p_re).s_in.mx } {
        re_append(p_re, 6, 0);
        *pp_re_1 = p_re;
    } else {
        sqlite3re_free(p_re);
        return c"unrecognized character".as_ptr() as *mut i8 as *const i8;
    }
    if unsafe { *unsafe { (*p_re).a_op.offset(0 as isize) } } as i32 == 3 &&
            (no_case_1 == 0) as i32 != 0 {
        {
            { j = 0; i = 1 };
            '__b15: loop {
                if !(j < core::mem::size_of::<[u8; 12]>() as i32 - 2 &&
                                unsafe { *unsafe { (*p_re).a_op.offset(i as isize) } } as
                                        i32 == 1) {
                    break '__b15;
                }
                '__c15: loop {
                    let x: u32 =
                        unsafe { *unsafe { (*p_re).a_arg.offset(i as isize) } } as
                            u32;
                    if x <= 127 as u32 {
                        unsafe {
                            (*p_re).z_init[{
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize] = x as u8
                        };
                    } else if x <= 2047 as u32 {
                        unsafe {
                            (*p_re).z_init[{
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize] = (192 as u32 | x >> 6) as u8
                        };
                        unsafe {
                            (*p_re).z_init[{
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize] = (128 as u32 | x & 63 as u32) as u8
                        };
                    } else if x <= 65535 as u32 {
                        unsafe {
                            (*p_re).z_init[{
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize] = (224 as u32 | x >> 12) as u8
                        };
                        unsafe {
                            (*p_re).z_init[{
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize] = (128 as u32 | x >> 6 & 63 as u32) as u8
                        };
                        unsafe {
                            (*p_re).z_init[{
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize] = (128 as u32 | x & 63 as u32) as u8
                        };
                    } else { break '__b15; }
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if j > 0 && unsafe { (*p_re).z_init[(j - 1) as usize] } as i32 == 0 {
            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
        }
        unsafe { (*p_re).n_init = j };
    }
    return unsafe { (*p_re).z_err };
}
extern "C" fn re_maxlen(context: *mut Sqlite3Context) -> i32 {
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    return unsafe { sqlite3_limit(db, 8, -1) };
}
extern "C" fn re_maxnfa(mxlen: i32) -> i32 { return 75 + mxlen / 2; }
extern "C" fn re_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p_re: *mut ReCompiled = core::ptr::null_mut();
    let mut z_pattern: *const i8 = core::ptr::null();
    let mut z_str: *const u8 = core::ptr::null();
    let mut z_err: *const i8 = core::ptr::null();
    let mut set_aux: i32 = 0;
    { let _ = argc; };
    p_re = unsafe { sqlite3_get_auxdata(context, 0) } as *mut ReCompiled;
    if p_re == core::ptr::null_mut() {
        let mx_len: i32 = re_maxlen(context);
        let mut n_pattern: i32 = 0;
        z_pattern =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        if z_pattern == core::ptr::null() { return; }
        n_pattern =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        if n_pattern > mx_len {
            z_err =
                c"REGEXP pattern too big".as_ptr() as *mut i8 as *const i8;
        } else {
            z_err =
                sqlite3re_compile(&mut p_re, z_pattern, re_maxnfa(mx_len),
                    (unsafe { sqlite3_user_data(context) } !=
                            core::ptr::null_mut()) as i32);
        }
        if !(z_err).is_null() {
            sqlite3re_free(p_re);
            unsafe { sqlite3_result_error(context, z_err, -1) };
            return;
        }
        if p_re == core::ptr::null_mut() {
            unsafe { sqlite3_result_error_nomem(context) };
            return;
        }
        set_aux = 1;
    }
    z_str =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) } as
            *const u8;
    if z_str != core::ptr::null() {
        unsafe {
            sqlite3_result_int(context,
                sqlite3re_match(unsafe { &*p_re }, z_str, -1))
        };
    }
    if set_aux != 0 {
        unsafe {
            sqlite3_set_auxdata(context, 0, p_re as *mut (),
                Some(re_free_voidptr))
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_regexp_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"regexp".as_ptr() as *mut i8 as *const i8, 2,
                1 | 2097152 | 2048, core::ptr::null_mut(), Some(re_sql_func),
                None, None)
        };
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"regexpi".as_ptr() as *mut i8 as *const i8, 2,
                    1 | 2097152 | 2048, db as *mut (), Some(re_sql_func), None,
                    None)
            };
    }
    return rc;
}
static z_esc: [i8; 22] =
    [97 as i8, 102 as i8, 110 as i8, 114 as i8, 116 as i8, 118 as i8,
            92 as i8, 40 as i8, 41 as i8, 42 as i8, 46 as i8, 43 as i8,
            63 as i8, 91 as i8, 36 as i8, 94 as i8, 123 as i8, 124 as i8,
            125 as i8, 93 as i8, 45 as i8, 0 as i8];
static z_trans: [i8; 7] =
    [7 as i8, 12 as i8, 10 as i8, 13 as i8, 9 as i8, 11 as i8, 0 as i8];
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
    fn strlen(__s: *const i8)
    -> u64;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn __builtin_unreachable()
    -> ();
}