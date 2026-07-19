#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

static totype_one: i32 = 1 as i32;

///* Return TRUE if character c is a whitespace character
extern "C" fn totype_isspace(c: u8) -> i32 {
    return (c as i32 == ' ' as i32 || c as i32 == '\t' as i32 ||
                            c as i32 == '\n' as i32 || c as i32 == '\u{b}' as i32 ||
                    c as i32 == '\u{c}' as i32 || c as i32 == '\r' as i32) as
            i32;
}

///* Return TRUE if character c is a digit
extern "C" fn totype_isdigit(c: u8) -> i32 {
    return (c as i32 >= '0' as i32 && c as i32 <= '9' as i32) as i32;
}

///* Compare the 19-character string zNum against the text representation
///* value 2^63:  9223372036854775808.  Return negative, zero, or positive
///* if zNum is less than, equal to, or greater than the string.
///* Note that zNum must contain exactly 19 characters.
///*
///* Unlike memcmp() this routine is guaranteed to return the difference
///* in the values of the last digit if the only difference is in the
///* last digit.  So, for example,
///*
///*      totypeCompare2pow63("9223372036854775800")
///*
///* will return -8.
#[allow(unused_doc_comments)]
extern "C" fn totype_compare2pow63(z_num_1: *const i8) -> i32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    /// 012345678901234567
    let pow63: *const i8 =
        c"922337203685477580".as_ptr() as *mut i8 as *const i8;
    {
        i = 0;
        '__b0: loop {
            if !(c == 0 && i < 18) { break '__b0; }
            '__c0: loop {
                c =
                    (unsafe { *z_num_1.offset(i as isize) } as i32 -
                            unsafe { *pow63.offset(i as isize) } as i32) * 10;
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if c == 0 {
        c = unsafe { *z_num_1.offset(18 as isize) } as i32 - '8' as i32;
    }
    return c;
}

///* Convert zNum to a 64-bit signed integer.
///*
///* If the zNum value is representable as a 64-bit twos-complement
///* integer, then write that value into *pNum and return 0.
///*
///* If zNum is exactly 9223372036854665808, return 2.  This special
///* case is broken out because while 9223372036854665808 cannot be a
///* signed 64-bit integer, its negative -9223372036854665808 can be.
///*
///* If zNum is too big for a 64-bit integer and is not
///* 9223372036854665808  or if zNum contains any non-numeric text,
///* then return 1.
///*
///* The string is not necessarily zero-terminated.
#[allow(unused_doc_comments)]
extern "C" fn totype_atoi64(mut z_num_1: *const i8,
    p_num_1: &mut Sqlite3Int64, length: i32) -> i32 {
    let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut neg: i32 = 0;
    /// assume positive
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let non_num: i32 = 0;
    let mut z_start: *const i8 = core::ptr::null();
    let z_end: *const i8 = unsafe { z_num_1.offset(length as isize) };
    while z_num_1 < z_end && totype_isspace(unsafe { *z_num_1 } as u8) != 0 {
        {
            let __p = &mut z_num_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if z_num_1 < z_end {
        if unsafe { *z_num_1 } as i32 == '-' as i32 {
            neg = 1;
            {
                let __p = &mut z_num_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_num_1 } as i32 == '+' as i32 {
            {
                let __p = &mut z_num_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
    z_start = z_num_1;
    while z_num_1 < z_end &&
            unsafe { *z_num_1.offset(0 as isize) } as i32 == '0' as i32 {
        {
            let __p = &mut z_num_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    {
        i = 0;
        '__b3: loop {
            if !(unsafe { z_num_1.offset(i as isize) } < z_end &&
                                { c = unsafe { *z_num_1.offset(i as isize) } as i32; c } >=
                                    '0' as i32 && c <= '9' as i32) {
                break '__b3;
            }
            '__c3: loop {
                u =
                    u * 10 as Sqlite3Uint64 + c as Sqlite3Uint64 -
                        '0' as i32 as Sqlite3Uint64;
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if u >
            (4294967295u32 as Sqlite3Int64 |
                    (2147483647 as Sqlite3Int64) << 32) as u64 {
        *p_num_1 =
            -1 as Sqlite3Int64 -
                (4294967295u32 as Sqlite3Int64 |
                    (2147483647 as Sqlite3Int64) << 32);
    } else if neg != 0 {
        *p_num_1 = -(u as Sqlite3Int64);
    } else { *p_num_1 = u as Sqlite3Int64; }
    if c != 0 && unsafe { z_num_1.offset(i as isize) } < z_end ||
                    i == 0 && z_start == z_num_1 || i > 19 || non_num != 0 {

        /// zNum is empty or contains non-numeric text or is longer
        ///* than 19 digits (thus guaranteeing that it is too large)
        return 1;
    } else if i < 19 {

        /// Less than 19 digits, so we know that it fits in 64 bits
        if !(u <=
                                (4294967295u32 as Sqlite3Int64 |
                                        (2147483647 as Sqlite3Int64) << 32) as u64) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"totypeAtoi64".as_ptr() as *const i8,
                    c"totype.c".as_ptr() as *mut i8 as *const i8, 161,
                    c"u<=LARGEST_INT64".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        return 0;
    } else {

        /// zNum is a 19-digit numbers.  Compare it against 9223372036854775808.
        (c = totype_compare2pow63(z_num_1));
        if c < 0 {

            /// zNum is less than 9223372036854775808 so it fits
            if !(u <=
                                    (4294967295u32 as Sqlite3Int64 |
                                            (2147483647 as Sqlite3Int64) << 32) as u64) as i32 as i64 !=
                    0 {
                unsafe {
                    __assert_rtn(c"totypeAtoi64".as_ptr() as *const i8,
                        c"totype.c".as_ptr() as *mut i8 as *const i8, 168,
                        c"u<=LARGEST_INT64".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            return 0;
        } else if c > 0 {

            /// zNum is greater than 9223372036854775808 so it overflows
            return 1;
        } else {

            /// zNum is exactly 9223372036854775808.  Fits if negative.  The
            ///* special case 2 overflow if positive
            if !(u - 1 as Sqlite3Uint64 ==
                                    (4294967295u32 as Sqlite3Int64 |
                                            (2147483647 as Sqlite3Int64) << 32) as u64) as i32 as i64 !=
                    0 {
                unsafe {
                    __assert_rtn(c"totypeAtoi64".as_ptr() as *const i8,
                        c"totype.c".as_ptr() as *mut i8 as *const i8, 176,
                        c"u-1==LARGEST_INT64".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if !(*p_num_1 ==
                                    -1 as Sqlite3Int64 -
                                        (4294967295u32 as Sqlite3Int64 |
                                            (2147483647 as Sqlite3Int64) << 32)) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"totypeAtoi64".as_ptr() as *const i8,
                        c"totype.c".as_ptr() as *mut i8 as *const i8, 177,
                        c"(*pNum)==SMALLEST_INT64".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            return if neg != 0 { 0 } else { 2 };
        }
    }
}

///* The string z[] is an text representation of a real number.
///* Convert this string to a double and write it into *pResult.
///*
///* The string is not necessarily zero-terminated.
///*
///* Return TRUE if the result is a valid real number (or integer) and FALSE
///* if the string is empty or contains extraneous text.  Valid numbers
///* are in one of these formats:
///*
///*    [+-]digits[E[+-]digits]
///*    [+-]digits.[digits][E[+-]digits]
///*    [+-].digits[E[+-]digits]
///*
///* Leading and trailing whitespace is ignored for the purpose of determining
///* validity.
///*
///* If some prefix of the input string is a valid number, this routine
///* returns FALSE but it still converts the prefix and writes the result
///* into *pResult.
#[allow(unused_doc_comments)]
extern "C" fn totype_ato_f(mut z: *const i8, p_result_1: &mut f64,
    length: i32) -> i32 {
    let mut z_end: *const i8 = core::ptr::null();
    /// sign * significand * (10 ^ (esign * exponent))
    let mut sign: i32 = 0;
    /// sign of significand
    let mut s: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// significand
    let mut d: i32 = 0;
    /// adjust exponent for shifting decimal point
    let mut esign: i32 = 0;
    /// sign of exponent
    let mut e: i32 = 0;
    /// exponent
    let mut e_valid: i32 = 0;
    /// True exponent is either not used or is well-formed
    let mut result: f64 = 0.0;
    let mut n_digits: i32 = 0;
    let mut non_num: i32 = 0;
    /// Default return value, in case of an error
    /// skip leading spaces
    /// get sign of significand
    /// skip leading zeroes
    /// copy max significant digits to significand
    /// skip non-significant significand digits
    ///* (increase exponent by d to shift decimal left)
    /// if decimal point is present
    /// copy digits from after decimal to significand
    ///* (decrease exponent by d to shift decimal right)
    /// skip non-significant digits
    /// if exponent is present
    /// get sign of exponent
    /// copy digits to exponent
    /// skip trailing spaces
    /// adjust exponent by d, and update sign
    /// if 0 significand
    /// In the IEEE 754 standard, zero is signed.
    ///* Add the sign if we've seen at least one digit
    /// attempt to reduce exponent
    /// adjust the sign of significand
    /// if exponent, scale significand as appropriate
    ///* and store in result.
    let mut scale: f64 = 0.0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s5:
            {
            match __state {
                0 => {
                    z_end = unsafe { z.offset(length as isize) };
                    __state = 3;
                }
                2 => { e = e * esign + d; __state = 58; }
                3 => { sign = 1; __state = 4; }
                4 => { s = 0 as Sqlite3Int64; __state = 5; }
                5 => { d = 0; __state = 6; }
                6 => { esign = 1; __state = 7; }
                7 => { e = 0; __state = 8; }
                8 => { e_valid = 1; __state = 9; }
                9 => { __state = 10; }
                10 => { n_digits = 0; __state = 11; }
                11 => { non_num = 0; __state = 12; }
                12 => { *p_result_1 = 0.0; __state = 13; }
                13 => {
                    if z < z_end && totype_isspace(unsafe { *z } as u8) != 0 {
                        __state = 15;
                    } else { __state = 14; }
                }
                14 => {
                    if z >= z_end { __state = 17; } else { __state = 16; }
                }
                15 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 13;
                }
                16 => {
                    if unsafe { *z } as i32 == '-' as i32 {
                        __state = 19;
                    } else { __state = 20; }
                }
                17 => { return 0; }
                18 => {
                    if z < z_end &&
                            unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 {
                        __state = 24;
                    } else { __state = 23; }
                }
                19 => { sign = -1; __state = 21; }
                20 => {
                    if unsafe { *z } as i32 == '+' as i32 {
                        __state = 22;
                    } else { __state = 18; }
                }
                21 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 18;
                }
                22 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 18;
                }
                23 => {
                    if z < z_end && totype_isdigit(unsafe { *z } as u8) != 0 &&
                            s <
                                ((4294967295u32 as Sqlite3Int64 |
                                            (2147483647 as Sqlite3Int64) << 32) - 9 as Sqlite3Int64) /
                                    10 as Sqlite3Int64 {
                        __state = 26;
                    } else { __state = 25; }
                }
                24 => {
                    {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        { let __p = &mut n_digits; let __t = *__p; *__p += 1; __t }
                    };
                    __state = 18;
                }
                25 => {
                    if z < z_end && totype_isdigit(unsafe { *z } as u8) != 0 {
                        __state = 29;
                    } else { __state = 28; }
                }
                26 => {
                    s =
                        s * 10 as Sqlite3Int64 +
                            (unsafe { *z } as i32 - '0' as i32) as Sqlite3Int64;
                    __state = 27;
                }
                27 => {
                    {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        { let __p = &mut n_digits; let __t = *__p; *__p += 1; __t }
                    };
                    __state = 23;
                }
                28 => {
                    if z >= z_end { __state = 31; } else { __state = 30; }
                }
                29 => {
                    {
                        {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            { let __p = &mut n_digits; let __t = *__p; *__p += 1; __t }
                        };
                        { let __p = &mut d; let __t = *__p; *__p += 1; __t }
                    };
                    __state = 25;
                }
                30 => {
                    if unsafe { *z } as i32 == '.' as i32 {
                        __state = 33;
                    } else { __state = 32; }
                }
                31 => { __state = 2; }
                32 => {
                    if z >= z_end { __state = 40; } else { __state = 39; }
                }
                33 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 34;
                }
                34 => {
                    if z < z_end && totype_isdigit(unsafe { *z } as u8) != 0 &&
                            s <
                                ((4294967295u32 as Sqlite3Int64 |
                                            (2147483647 as Sqlite3Int64) << 32) - 9 as Sqlite3Int64) /
                                    10 as Sqlite3Int64 {
                        __state = 36;
                    } else { __state = 35; }
                }
                35 => {
                    if z < z_end && totype_isdigit(unsafe { *z } as u8) != 0 {
                        __state = 38;
                    } else { __state = 32; }
                }
                36 => {
                    s =
                        s * 10 as Sqlite3Int64 +
                            (unsafe { *z } as i32 - '0' as i32) as Sqlite3Int64;
                    __state = 37;
                }
                37 => {
                    {
                        {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            { let __p = &mut n_digits; let __t = *__p; *__p += 1; __t }
                        };
                        { let __p = &mut d; let __t = *__p; *__p -= 1; __t }
                    };
                    __state = 34;
                }
                38 => {
                    {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        { let __p = &mut n_digits; let __t = *__p; *__p += 1; __t }
                    };
                    __state = 35;
                }
                39 => {
                    if unsafe { *z } as i32 == 'e' as i32 ||
                            unsafe { *z } as i32 == 'E' as i32 {
                        __state = 42;
                    } else { __state = 41; }
                }
                40 => { __state = 2; }
                41 => {
                    if n_digits != 0 && e_valid != 0 {
                        __state = 56;
                    } else { __state = 55; }
                }
                42 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 43;
                }
                43 => { e_valid = 0; __state = 44; }
                44 => {
                    if z >= z_end { __state = 46; } else { __state = 45; }
                }
                45 => {
                    if unsafe { *z } as i32 == '-' as i32 {
                        __state = 48;
                    } else { __state = 49; }
                }
                46 => { __state = 2; }
                47 => {
                    if z < z_end && totype_isdigit(unsafe { *z } as u8) != 0 {
                        __state = 52;
                    } else { __state = 41; }
                }
                48 => { esign = -1; __state = 50; }
                49 => {
                    if unsafe { *z } as i32 == '+' as i32 {
                        __state = 51;
                    } else { __state = 47; }
                }
                50 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 47;
                }
                51 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 47;
                }
                52 => {
                    e =
                        if e < 10000 {
                            e * 10 + (unsafe { *z } as i32 - '0' as i32)
                        } else { 10000 };
                    __state = 53;
                }
                53 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 54;
                }
                54 => { e_valid = 1; __state = 47; }
                55 => { __state = 2; }
                56 => {
                    if z < z_end && totype_isspace(unsafe { *z } as u8) != 0 {
                        __state = 57;
                    } else { __state = 55; }
                }
                57 => {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 56;
                }
                58 => { if e < 0 { __state = 60; } else { __state = 61; } }
                59 => {
                    if (s == 0) as i32 != 0 {
                        __state = 64;
                    } else { __state = 65; }
                }
                60 => { esign = -1; __state = 62; }
                61 => { esign = 1; __state = 59; }
                62 => { e *= -1; __state = 59; }
                63 => { *p_result_1 = result; __state = 96; }
                64 => {
                    result =
                        if sign < 0 && n_digits != 0 {
                            -(0 as f64)
                        } else { 0 as f64 };
                    __state = 63;
                }
                65 => {
                    if esign > 0 { __state = 67; } else { __state = 68; }
                }
                66 => { s = if sign < 0 { -s } else { s }; __state = 71; }
                67 => {
                    if s <
                                (4294967295u32 as Sqlite3Int64 |
                                        (2147483647 as Sqlite3Int64) << 32) / 10 as Sqlite3Int64 &&
                            e > 0 {
                        __state = 69;
                    } else { __state = 66; }
                }
                68 => {
                    if (s % 10 as Sqlite3Int64 == 0) as i32 != 0 && e > 0 {
                        __state = 70;
                    } else { __state = 66; }
                }
                69 => {
                    {
                        ({ let __p = &mut e; let __t = *__p; *__p -= 1; __t }) as
                            Sqlite3Int64;
                        s *= 10 as Sqlite3Int64
                    };
                    __state = 67;
                }
                70 => {
                    {
                        ({ let __p = &mut e; let __t = *__p; *__p -= 1; __t }) as
                            Sqlite3Int64;
                        s /= 10 as Sqlite3Int64
                    };
                    __state = 68;
                }
                71 => { if e != 0 { __state = 72; } else { __state = 73; } }
                72 => { scale = 1.0; __state = 74; }
                73 => { result = s as f64; __state = 63; }
                74 => {
                    if e > 307 && e < 342 {
                        __state = 75;
                    } else { __state = 76; }
                }
                75 => {
                    if e % 308 != 0 { __state = 78; } else { __state = 77; }
                }
                76 => { if e >= 342 { __state = 84; } else { __state = 85; } }
                77 => {
                    if esign < 0 { __state = 80; } else { __state = 81; }
                }
                78 => { scale *= 10.0; __state = 79; }
                79 => { e -= 1; __state = 75; }
                80 => { result = s as f64 / scale; __state = 82; }
                81 => { result = s as f64 * scale; __state = 83; }
                82 => { result /= 1e308; __state = 63; }
                83 => { result *= 1e308; __state = 63; }
                84 => {
                    if esign < 0 { __state = 86; } else { __state = 87; }
                }
                85 => {
                    if e % 22 != 0 { __state = 89; } else { __state = 88; }
                }
                86 => { result = 0.0 * s as f64; __state = 63; }
                87 => { result = 1e308 * 1e308 * s as f64; __state = 63; }
                88 => { if e > 0 { __state = 92; } else { __state = 91; } }
                89 => { scale *= 10.0; __state = 90; }
                90 => { e -= 1; __state = 85; }
                91 => {
                    if esign < 0 { __state = 94; } else { __state = 95; }
                }
                92 => { scale *= 1e22; __state = 93; }
                93 => { e -= 22; __state = 88; }
                94 => { result = s as f64 / scale; __state = 63; }
                95 => { result = s as f64 * scale; __state = 63; }
                96 => {
                    return (z >= z_end && n_digits > 0 && e_valid != 0 &&
                                non_num == 0) as i32;
                }
                _ => {}
            }
        }
    }

    /// sign * significand * (10 ^ (esign * exponent))
    /// sign of significand
    /// significand
    /// adjust exponent for shifting decimal point
    /// sign of exponent
    /// exponent
    /// True exponent is either not used or is well-formed
    /// Default return value, in case of an error
    /// skip leading spaces
    /// get sign of significand
    /// skip leading zeroes
    /// copy max significant digits to significand
    /// skip non-significant significand digits
    ///* (increase exponent by d to shift decimal left)
    /// if decimal point is present
    /// copy digits from after decimal to significand
    ///* (decrease exponent by d to shift decimal right)
    /// skip non-significant digits
    /// if exponent is present
    /// get sign of exponent
    /// copy digits to exponent
    /// skip trailing spaces
    /// adjust exponent by d, and update sign
    /// if 0 significand
    /// In the IEEE 754 standard, zero is signed.
    ///* Add the sign if we've seen at least one digit
    /// attempt to reduce exponent
    /// adjust the sign of significand
    /// if exponent, scale significand as appropriate
    ///* and store in result.
    /// attempt to handle extremely small/large numbers better
    /// Infinity
    /// 1.0e+22 is the largest power of 10 than can be
    ///* represented exactly.
    /// store the result
    /// return true if number and no extra non-whitespace characters after
    unreachable!();
}

/// 
///* Convert a floating point value to an integer. Or, if this cannot be
///* done in a way that avoids 'outside the range of representable values' 
///* warnings from UBSAN, return 0.
///*
///* This function is a modified copy of internal SQLite function
///* sqlite3RealToI64().
extern "C" fn totype_double_to_int(r: f64) -> Sqlite3Int64 {
    if r < -9.223372036854775e18 { return 0 as Sqlite3Int64; }
    if r > 9.223372036854775e18 { return 0 as Sqlite3Int64; }
    return r as Sqlite3Int64;
}

///* tointeger(X):  If X is any value (integer, double, blob, or string) that
///* can be losslessly converted into an integer, then make the conversion and
///* return the result.  Otherwise, return NULL.
extern "C" fn tointeger_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    if !(argc == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"tointegerFunc".as_ptr() as *const i8,
                c"totype.c".as_ptr() as *mut i8 as *const i8, 377,
                c"argc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    { let _ = argc; };
    '__s6:
        {
        match unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } {
            2 => {
                {
                    let r_val: f64 =
                        unsafe {
                            sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                        };
                    let i_val: Sqlite3Int64 = totype_double_to_int(r_val);
                    if r_val == i_val as f64 {
                        unsafe { sqlite3_result_int64(context, i_val) };
                    }
                    break '__s6;
                }
                {
                    unsafe {
                        sqlite3_result_int64(context,
                            unsafe {
                                sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s6;
                }
                {
                    let z_blob: *const u8 =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    if !(z_blob).is_null() {
                        let n_blob: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_blob as u64 ==
                                core::mem::size_of::<Sqlite3Int64>() as u64 {
                            let mut i_val_1: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if unsafe { *(&raw const totype_one as *mut i8) } as i32 ==
                                    0 {
                                let mut i: i32 = 0;
                                let mut z_blob_rev: [u8; 8] = [0; 8];
                                {
                                    i = 0;
                                    '__b7: loop {
                                        if !((i as u64) <
                                                        core::mem::size_of::<Sqlite3Int64>() as u64) {
                                            break '__b7;
                                        }
                                        '__c7: loop {
                                            z_blob_rev[i as usize] =
                                                unsafe {
                                                        *z_blob.add((core::mem::size_of::<Sqlite3Int64>() as u64 -
                                                                            1 as u64 - i as u64) as usize)
                                                    } as u8;
                                            break '__c7;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    memcpy(&raw mut i_val_1 as *mut (),
                                        &raw mut z_blob_rev[0 as usize] as *mut u8 as *const (),
                                        core::mem::size_of::<Sqlite3Int64>() as u64)
                                };
                            } else {
                                unsafe {
                                    memcpy(&raw mut i_val_1 as *mut (), z_blob as *const (),
                                        core::mem::size_of::<Sqlite3Int64>() as u64)
                                };
                            }
                            unsafe { sqlite3_result_int64(context, i_val_1) };
                        }
                    }
                    break '__s6;
                }
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                        as i32 != 0 {
                            let mut i_val_2: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if (totype_atoi64(z_str as *const i8, &mut i_val_2, n_str)
                                            == 0) as i32 != 0 {
                                unsafe { sqlite3_result_int64(context, i_val_2) };
                            }
                        }
                    }
                    break '__s6;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tointegerFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 427,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s6;
                }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_result_int64(context,
                            unsafe {
                                sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s6;
                }
                {
                    let z_blob: *const u8 =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    if !(z_blob).is_null() {
                        let n_blob: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_blob as u64 ==
                                core::mem::size_of::<Sqlite3Int64>() as u64 {
                            let mut i_val_1: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if unsafe { *(&raw const totype_one as *mut i8) } as i32 ==
                                    0 {
                                let mut i: i32 = 0;
                                let mut z_blob_rev: [u8; 8] = [0; 8];
                                {
                                    i = 0;
                                    '__b7: loop {
                                        if !((i as u64) <
                                                        core::mem::size_of::<Sqlite3Int64>() as u64) {
                                            break '__b7;
                                        }
                                        '__c7: loop {
                                            z_blob_rev[i as usize] =
                                                unsafe {
                                                        *z_blob.add((core::mem::size_of::<Sqlite3Int64>() as u64 -
                                                                            1 as u64 - i as u64) as usize)
                                                    } as u8;
                                            break '__c7;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    memcpy(&raw mut i_val_1 as *mut (),
                                        &raw mut z_blob_rev[0 as usize] as *mut u8 as *const (),
                                        core::mem::size_of::<Sqlite3Int64>() as u64)
                                };
                            } else {
                                unsafe {
                                    memcpy(&raw mut i_val_1 as *mut (), z_blob as *const (),
                                        core::mem::size_of::<Sqlite3Int64>() as u64)
                                };
                            }
                            unsafe { sqlite3_result_int64(context, i_val_1) };
                        }
                    }
                    break '__s6;
                }
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                        as i32 != 0 {
                            let mut i_val_2: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if (totype_atoi64(z_str as *const i8, &mut i_val_2, n_str)
                                            == 0) as i32 != 0 {
                                unsafe { sqlite3_result_int64(context, i_val_2) };
                            }
                        }
                    }
                    break '__s6;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tointegerFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 427,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s6;
                }
            }
            4 => {
                {
                    let z_blob: *const u8 =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    if !(z_blob).is_null() {
                        let n_blob: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_blob as u64 ==
                                core::mem::size_of::<Sqlite3Int64>() as u64 {
                            let mut i_val_1: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if unsafe { *(&raw const totype_one as *mut i8) } as i32 ==
                                    0 {
                                let mut i: i32 = 0;
                                let mut z_blob_rev: [u8; 8] = [0; 8];
                                {
                                    i = 0;
                                    '__b7: loop {
                                        if !((i as u64) <
                                                        core::mem::size_of::<Sqlite3Int64>() as u64) {
                                            break '__b7;
                                        }
                                        '__c7: loop {
                                            z_blob_rev[i as usize] =
                                                unsafe {
                                                        *z_blob.add((core::mem::size_of::<Sqlite3Int64>() as u64 -
                                                                            1 as u64 - i as u64) as usize)
                                                    } as u8;
                                            break '__c7;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    memcpy(&raw mut i_val_1 as *mut (),
                                        &raw mut z_blob_rev[0 as usize] as *mut u8 as *const (),
                                        core::mem::size_of::<Sqlite3Int64>() as u64)
                                };
                            } else {
                                unsafe {
                                    memcpy(&raw mut i_val_1 as *mut (), z_blob as *const (),
                                        core::mem::size_of::<Sqlite3Int64>() as u64)
                                };
                            }
                            unsafe { sqlite3_result_int64(context, i_val_1) };
                        }
                    }
                    break '__s6;
                }
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                        as i32 != 0 {
                            let mut i_val_2: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if (totype_atoi64(z_str as *const i8, &mut i_val_2, n_str)
                                            == 0) as i32 != 0 {
                                unsafe { sqlite3_result_int64(context, i_val_2) };
                            }
                        }
                    }
                    break '__s6;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tointegerFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 427,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s6;
                }
            }
            3 => {
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                        as i32 != 0 {
                            let mut i_val_2: Sqlite3Int64 = 0 as Sqlite3Int64;
                            if (totype_atoi64(z_str as *const i8, &mut i_val_2, n_str)
                                            == 0) as i32 != 0 {
                                unsafe { sqlite3_result_int64(context, i_val_2) };
                            }
                        }
                    }
                    break '__s6;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tointegerFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 427,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s6;
                }
            }
            _ => {
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tointegerFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 427,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s6;
                }
            }
        }
    }
}

extern "C" fn toreal_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    if !(argc == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"torealFunc".as_ptr() as *const i8,
                c"totype.c".as_ptr() as *mut i8 as *const i8, 447,
                c"argc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    { let _ = argc; };
    '__s8:
        {
        match unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } {
            2 => {
                {
                    unsafe {
                        sqlite3_result_double(context,
                            unsafe {
                                sqlite3_value_double(unsafe { *argv.offset(0 as isize) })
                            })
                    };
                    break '__s8;
                }
                {
                    let i_val: Sqlite3Int64 =
                        unsafe {
                            sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                        };
                    let r_val: f64 = i_val as f64;
                    if i_val == totype_double_to_int(r_val) {
                        unsafe { sqlite3_result_double(context, r_val) };
                    }
                    break '__s8;
                }
                {
                    let z_blob: *const u8 =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    if !(z_blob).is_null() {
                        let n_blob: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_blob as u64 == core::mem::size_of::<f64>() as u64 {
                            let mut r_val_1: f64 = 0.0;
                            if unsafe { *(&raw const totype_one as *mut i8) } as i32 ==
                                    1 {
                                let mut i: i32 = 0;
                                let mut z_blob_rev: [u8; 8] = [0; 8];
                                {
                                    i = 0;
                                    '__b9: loop {
                                        if !((i as u64) < core::mem::size_of::<f64>() as u64) {
                                            break '__b9;
                                        }
                                        '__c9: loop {
                                            z_blob_rev[i as usize] =
                                                unsafe {
                                                        *z_blob.add((core::mem::size_of::<f64>() as u64 - 1 as u64 -
                                                                        i as u64) as usize)
                                                    } as u8;
                                            break '__c9;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    memcpy(&raw mut r_val_1 as *mut (),
                                        &raw mut z_blob_rev[0 as usize] as *mut u8 as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                            } else {
                                unsafe {
                                    memcpy(&raw mut r_val_1 as *mut (), z_blob as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                            }
                            unsafe { sqlite3_result_double(context, r_val_1) };
                        }
                    }
                    break '__s8;
                }
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                    (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                            as i32 != 0 &&
                                (totype_isspace(unsafe {
                                                    *z_str.offset((n_str - 1) as isize)
                                                }) == 0) as i32 != 0 {
                            let mut r_val_2: f64 = 0.0;
                            if totype_ato_f(z_str as *const i8, &mut r_val_2, n_str) !=
                                    0 {
                                unsafe { sqlite3_result_double(context, r_val_2) };
                                return;
                            }
                        }
                    }
                    break '__s8;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"torealFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 498,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s8;
                }
            }
            1 => {
                {
                    let i_val: Sqlite3Int64 =
                        unsafe {
                            sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                        };
                    let r_val: f64 = i_val as f64;
                    if i_val == totype_double_to_int(r_val) {
                        unsafe { sqlite3_result_double(context, r_val) };
                    }
                    break '__s8;
                }
                {
                    let z_blob: *const u8 =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    if !(z_blob).is_null() {
                        let n_blob: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_blob as u64 == core::mem::size_of::<f64>() as u64 {
                            let mut r_val_1: f64 = 0.0;
                            if unsafe { *(&raw const totype_one as *mut i8) } as i32 ==
                                    1 {
                                let mut i: i32 = 0;
                                let mut z_blob_rev: [u8; 8] = [0; 8];
                                {
                                    i = 0;
                                    '__b9: loop {
                                        if !((i as u64) < core::mem::size_of::<f64>() as u64) {
                                            break '__b9;
                                        }
                                        '__c9: loop {
                                            z_blob_rev[i as usize] =
                                                unsafe {
                                                        *z_blob.add((core::mem::size_of::<f64>() as u64 - 1 as u64 -
                                                                        i as u64) as usize)
                                                    } as u8;
                                            break '__c9;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    memcpy(&raw mut r_val_1 as *mut (),
                                        &raw mut z_blob_rev[0 as usize] as *mut u8 as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                            } else {
                                unsafe {
                                    memcpy(&raw mut r_val_1 as *mut (), z_blob as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                            }
                            unsafe { sqlite3_result_double(context, r_val_1) };
                        }
                    }
                    break '__s8;
                }
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                    (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                            as i32 != 0 &&
                                (totype_isspace(unsafe {
                                                    *z_str.offset((n_str - 1) as isize)
                                                }) == 0) as i32 != 0 {
                            let mut r_val_2: f64 = 0.0;
                            if totype_ato_f(z_str as *const i8, &mut r_val_2, n_str) !=
                                    0 {
                                unsafe { sqlite3_result_double(context, r_val_2) };
                                return;
                            }
                        }
                    }
                    break '__s8;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"torealFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 498,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s8;
                }
            }
            4 => {
                {
                    let z_blob: *const u8 =
                        unsafe {
                                sqlite3_value_blob(unsafe { *argv.offset(0 as isize) })
                            } as *const u8;
                    if !(z_blob).is_null() {
                        let n_blob: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_blob as u64 == core::mem::size_of::<f64>() as u64 {
                            let mut r_val_1: f64 = 0.0;
                            if unsafe { *(&raw const totype_one as *mut i8) } as i32 ==
                                    1 {
                                let mut i: i32 = 0;
                                let mut z_blob_rev: [u8; 8] = [0; 8];
                                {
                                    i = 0;
                                    '__b9: loop {
                                        if !((i as u64) < core::mem::size_of::<f64>() as u64) {
                                            break '__b9;
                                        }
                                        '__c9: loop {
                                            z_blob_rev[i as usize] =
                                                unsafe {
                                                        *z_blob.add((core::mem::size_of::<f64>() as u64 - 1 as u64 -
                                                                        i as u64) as usize)
                                                    } as u8;
                                            break '__c9;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    memcpy(&raw mut r_val_1 as *mut (),
                                        &raw mut z_blob_rev[0 as usize] as *mut u8 as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                            } else {
                                unsafe {
                                    memcpy(&raw mut r_val_1 as *mut (), z_blob as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                            }
                            unsafe { sqlite3_result_double(context, r_val_1) };
                        }
                    }
                    break '__s8;
                }
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                    (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                            as i32 != 0 &&
                                (totype_isspace(unsafe {
                                                    *z_str.offset((n_str - 1) as isize)
                                                }) == 0) as i32 != 0 {
                            let mut r_val_2: f64 = 0.0;
                            if totype_ato_f(z_str as *const i8, &mut r_val_2, n_str) !=
                                    0 {
                                unsafe { sqlite3_result_double(context, r_val_2) };
                                return;
                            }
                        }
                    }
                    break '__s8;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"torealFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 498,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s8;
                }
            }
            3 => {
                {
                    let z_str: *const u8 =
                        unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        };
                    if !(z_str).is_null() {
                        let n_str: i32 =
                            unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                            };
                        if n_str != 0 &&
                                    (totype_isspace(unsafe { *z_str.offset(0 as isize) }) == 0)
                                            as i32 != 0 &&
                                (totype_isspace(unsafe {
                                                    *z_str.offset((n_str - 1) as isize)
                                                }) == 0) as i32 != 0 {
                            let mut r_val_2: f64 = 0.0;
                            if totype_ato_f(z_str as *const i8, &mut r_val_2, n_str) !=
                                    0 {
                                unsafe { sqlite3_result_double(context, r_val_2) };
                                return;
                            }
                        }
                    }
                    break '__s8;
                }
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"torealFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 498,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s8;
                }
            }
            _ => {
                {
                    if !(unsafe {
                                                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
                                            } == 5) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"torealFunc".as_ptr() as *const i8,
                                c"totype.c".as_ptr() as *mut i8 as *const i8, 498,
                                c"sqlite3_value_type(argv[0])==SQLITE_NULL".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__s8;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_totype_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };

    /// Unused parameter
    (rc =
        unsafe {
            sqlite3_create_function(db,
                c"tointeger".as_ptr() as *mut i8 as *const i8, 1,
                1 | 2048 | 2097152, core::ptr::null_mut(),
                Some(tointeger_func), None, None)
        });
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"toreal".as_ptr() as *mut i8 as *const i8, 1,
                    1 | 2048 | 2097152, core::ptr::null_mut(),
                    Some(toreal_func), None, None)
            };
    }
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
