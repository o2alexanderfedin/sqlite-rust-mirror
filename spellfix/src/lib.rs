#![feature(c_variadic)]
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

///* The following table gives the character class for non-initial ASCII
///* characters.
static mid_class: [u8; 128] =
    [12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 11 as u8, 12 as u8, 12 as u8, 11 as u8,
            11 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 11 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 0 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 10 as u8,
            10 as u8, 10 as u8, 10 as u8, 10 as u8, 10 as u8, 10 as u8,
            10 as u8, 10 as u8, 10 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 1 as u8, 2 as u8, 3 as u8,
            4 as u8, 1 as u8, 2 as u8, 3 as u8, 0 as u8, 1 as u8, 3 as u8,
            3 as u8, 6 as u8, 8 as u8, 8 as u8, 1 as u8, 2 as u8, 3 as u8,
            7 as u8, 3 as u8, 4 as u8, 1 as u8, 2 as u8, 2 as u8, 3 as u8,
            1 as u8, 3 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 1 as u8,
            2 as u8, 3 as u8, 0 as u8, 1 as u8, 3 as u8, 3 as u8, 6 as u8,
            8 as u8, 8 as u8, 1 as u8, 2 as u8, 3 as u8, 7 as u8, 3 as u8,
            4 as u8, 1 as u8, 2 as u8, 2 as u8, 3 as u8, 1 as u8, 3 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8];

/// 
///* This tables gives the character class for ASCII characters that form the
///* initial character of a word.  The only difference from midClass is with
///* the letters H, W, and Y.
static init_class: [u8; 128] =
    [12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 11 as u8, 12 as u8, 12 as u8, 11 as u8,
            11 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 11 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 10 as u8,
            10 as u8, 10 as u8, 10 as u8, 10 as u8, 10 as u8, 10 as u8,
            10 as u8, 10 as u8, 10 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 1 as u8, 2 as u8, 3 as u8,
            4 as u8, 1 as u8, 2 as u8, 3 as u8, 0 as u8, 1 as u8, 3 as u8,
            3 as u8, 6 as u8, 8 as u8, 8 as u8, 1 as u8, 2 as u8, 3 as u8,
            7 as u8, 3 as u8, 4 as u8, 1 as u8, 2 as u8, 2 as u8, 3 as u8,
            9 as u8, 3 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 1 as u8,
            2 as u8, 3 as u8, 0 as u8, 1 as u8, 3 as u8, 3 as u8, 6 as u8,
            8 as u8, 8 as u8, 1 as u8, 2 as u8, 3 as u8, 7 as u8, 3 as u8,
            4 as u8, 1 as u8, 2 as u8, 2 as u8, 3 as u8, 9 as u8, 3 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8];

///* Mapping from the character class number (0-13) to a symbol for each
///* character class.  Note that initClass[] can be used to map the class
///* symbol back into the class number.
static class_name: [u8; 14] =
    [46 as u8, 65 as u8, 66 as u8, 67 as u8, 68 as u8, 72 as u8, 76 as u8,
            82 as u8, 77 as u8, 89 as u8, 57 as u8, 32 as u8, 63 as u8,
            0 as u8];

///* Generate a "phonetic hash" from a string of ASCII characters
///* in zIn[0..nIn-1].
///*
///*   * Map characters by character class as defined above.
///*   * Omit double-letters
///*   * Omit vowels beside R and L
///*   * Omit T when followed by CH
///*   * Omit W when followed by R
///*   * Omit D when followed by J or G
///*   * Omit K in KN or G in GN at the beginning of a word
///*
///* Space to hold the result is obtained from sqlite3_malloc()
///*
///* Return NULL if memory allocation fails.
extern "C" fn phonetic_hash(mut z_in_1: *const u8, mut n_in_1: i32)
    -> *mut u8 {
    let z_out: *mut u8 =
        unsafe {
                sqlite3_malloc64((n_in_1 as i64 + 1 as i64) as Sqlite3Uint64)
            } as *mut u8;
    let mut i: i32 = 0;
    let mut n_out: i32 = 0;
    let mut c_prev: i8 = 119 as i8;
    let mut c_prev_x: i8 = 119 as i8;
    let mut a_class: *const u8 =
        &raw const init_class[0 as usize] as *const u8;
    if z_out == core::ptr::null_mut() { return core::ptr::null_mut(); }
    if n_in_1 > 2 {
        '__s0:
            {
            match unsafe { *z_in_1.offset(0 as isize) } {
                103 => {
                    {
                        if unsafe { *z_in_1.offset(1 as isize) } as i32 ==
                                'n' as i32 {
                            {
                                let __p = &mut z_in_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            { let __p = &mut n_in_1; let __t = *__p; *__p -= 1; __t };
                        }
                        break '__s0;
                    }
                }
                107 => {
                    {
                        if unsafe { *z_in_1.offset(1 as isize) } as i32 ==
                                'n' as i32 {
                            {
                                let __p = &mut z_in_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            { let __p = &mut n_in_1; let __t = *__p; *__p -= 1; __t };
                        }
                        break '__s0;
                    }
                }
                _ => {}
            }
        }
    }
    {
        i = 0;
        '__b1: loop {
            if !(i < n_in_1) { break '__b1; }
            '__c1: loop {
                let mut c: u8 = unsafe { *z_in_1.offset(i as isize) } as u8;
                if i + 1 < n_in_1 {
                    if c as i32 == 'w' as i32 &&
                            unsafe { *z_in_1.offset((i + 1) as isize) } as i32 ==
                                'r' as i32 {
                        break '__c1;
                    }
                    if c as i32 == 'd' as i32 &&
                            (unsafe { *z_in_1.offset((i + 1) as isize) } as i32 ==
                                    'j' as i32 ||
                                unsafe { *z_in_1.offset((i + 1) as isize) } as i32 ==
                                    'g' as i32) {
                        break '__c1;
                    }
                    if i + 2 < n_in_1 {
                        if c as i32 == 't' as i32 &&
                                    unsafe { *z_in_1.offset((i + 1) as isize) } as i32 ==
                                        'c' as i32 &&
                                unsafe { *z_in_1.offset((i + 2) as isize) } as i32 ==
                                    'h' as i32 {
                            break '__c1;
                        }
                    }
                }
                c =
                    unsafe { *a_class.offset((c as i32 & 127) as isize) } as u8;
                if c as i32 == 11 { break '__c1; }
                if c as i32 == 12 && c_prev as i32 != 10 { break '__c1; }
                a_class = &raw const mid_class[0 as usize] as *const u8;
                if c as i32 == 1 &&
                        (c_prev_x as i32 == 7 || c_prev_x as i32 == 6) {
                    break '__c1;
                }
                if (c as i32 == 7 || c as i32 == 6) && c_prev_x as i32 == 1 {
                    { let __p = &mut n_out; let __t = *__p; *__p -= 1; __t };
                }
                c_prev = c as i8;
                if c as i32 == 0 { break '__c1; }
                c_prev_x = c as i8;
                c = class_name[c as usize] as u8;
                { let _ = 0; };
                if n_out == 0 ||
                        c as i32 !=
                            unsafe { *z_out.offset((n_out - 1) as isize) } as i32 {
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut n_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = c
                    };
                }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z_out.offset(n_out as isize) = 0 as u8 };
    return z_out;
}

///* This is an SQL function wrapper around phoneticHash().  See
///* the description of phoneticHash() for additional information.
extern "C" fn phonetic_hash_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_in: *const u8 = core::ptr::null();
    let mut z_out: *mut u8 = core::ptr::null_mut();
    z_in = unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    if z_in == core::ptr::null() { return; }
    z_out =
        phonetic_hash(z_in,
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            });
    if z_out == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
    } else {
        unsafe {
            sqlite3_result_text(context, z_out as *mut i8 as *const i8, -1,
                Some(sqlite3_free))
        };
    }
}

///* Return the character class number for a character given its
///* context.
extern "C" fn character_class(c_prev_1: i8, c: i8) -> i8 {
    return if c_prev_1 as i32 == 0 {
                init_class[(c as i32 & 127) as usize] as i32
            } else { mid_class[(c as i32 & 127) as usize] as i32 } as i8;
}

///* Return the cost of inserting or deleting character c immediately
///* following character cPrev.  If cPrev==0, that means c is the first
///* character of the word.
#[allow(unused_doc_comments)]
extern "C" fn insert_or_delete_cost(c_prev_1: i8, c: i8, c_next_1: i8)
    -> i32 {
    let class_c: i8 = character_class(c_prev_1, c);
    let mut class_cprev: i8 = 0 as i8;
    if class_c as i32 == 0 {

        /// Insert or delete "silent" characters such as H or W
        return 1;
    }
    if c_prev_1 as i32 == c as i32 {

        /// Repeated characters, or miss a repeat
        return 10;
    }
    if class_c as i32 == 1 &&
            (c_prev_1 as i32 == 'r' as i32 || c_next_1 as i32 == 'r' as i32) {
        return 20;
    }
    class_cprev = character_class(c_prev_1, c_prev_1);
    if class_c as i32 == class_cprev as i32 {
        if class_c as i32 == 1 {

            /// Remove or add a new vowel to a vowel cluster
            return 15;
        } else {

            /// Remove or add a consonant not in the same class
            return 50;
        }
    }

    /// any other character insertion or deletion
    return 100;
}

///* Return the cost of substituting cTo in place of cFrom assuming
///* the previous character is cPrev.  If cPrev==0 then cTo is the first
///* character of the word.
#[allow(unused_doc_comments)]
extern "C" fn substitute_cost(c_prev_1: i8, c_from_1: i8, c_to_1: i8) -> i32 {
    let mut class_from: i8 = 0 as i8;
    let mut class_to: i8 = 0 as i8;
    if c_from_1 as i32 == c_to_1 as i32 {

        /// Exact match
        return 0;
    }
    if c_from_1 as i32 == c_to_1 as i32 ^ 32 &&
            (c_to_1 as i32 >= 'A' as i32 && c_to_1 as i32 <= 'Z' as i32 ||
                c_to_1 as i32 >= 'a' as i32 && c_to_1 as i32 <= 'z' as i32) {

        /// differ only in case
        return 0;
    }
    class_from = character_class(c_prev_1, c_from_1);
    class_to = character_class(c_prev_1, c_to_1);
    if class_from as i32 == class_to as i32 {

        /// Same character class
        return 40;
    }
    if class_from as i32 >= 2 && class_from as i32 <= 9 &&
                class_to as i32 >= 2 && class_to as i32 <= 9 {

        /// Convert from one consonant to another, but in a different class
        return 75;
    }

    /// Any other subsitution
    return 100;
}

///* Given two strings zA and zB which are pure ASCII, return the cost
///* of transforming zA into zB.  If zA ends with '*' assume that it is
///* a prefix of zB and give only minimal penalty for extra characters
///* on the end of zB.
///*
///* Smaller numbers mean a closer match.
///*
///* Negative values indicate an error:
///*    -1  One of the inputs is NULL
///*    -2  Non-ASCII characters on input
///*    -3  Unable to allocate memory
///*    -4  Inputs too large
///*
///* If pnMatch is not NULL, then *pnMatch is set to the number of bytes
///* of zB that matched the pattern in zA. If zA does not end with a '*',
///* then this value is always the number of bytes in zB (i.e. strlen(zB)).
///* If zA does end in a '*', then it is the number of bytes in the prefix
///* of zB that was deemed to match zA.
#[allow(unused_doc_comments)]
extern "C" fn editdist1(mut z_a_1: *const i8, mut z_b_1: *const i8,
    pn_match_1: *mut i32) -> i32 {
    let mut n_a: u32 = 0 as u32;
    let mut n_b: u32 = 0 as u32;
    /// Number of characters in zA[] and zB[]
    let mut x_a: u32 = 0 as u32;
    let mut x_b: u32 = 0 as u32;
    /// Loop counters for zA[] and zB[]
    let mut c_a: i8 = 0 as i8;
    let mut c_b: i8 = 0 as i8;
    /// Current character of zA and zB
    let mut c_aprev: i8 = 0 as i8;
    let mut c_bprev: i8 = 0 as i8;
    /// Previous character of zA and zB
    let mut c_anext: i8 = 0 as i8;
    let mut c_bnext: i8 = 0 as i8;
    /// Next character in zA and zB
    let mut d: i32 = 0;
    /// North-west cost value
    let mut dc: i32 = 0;
    /// North-west character value
    let mut res: i32 = 0;
    /// Final result
    let mut m: *mut i32 = core::ptr::null_mut();
    /// The cost matrix
    let mut cx: *mut i8 = core::ptr::null_mut();
    /// Corresponding character values
    let mut to_free: *mut i32 = core::ptr::null_mut();
    /// Malloced space
    let mut n_match: i32 = 0;
    let mut m_stack: [i32; 75] = [0; 75];
    if z_a_1 == core::ptr::null() || z_b_1 == core::ptr::null() { return -1; }
    while unsafe { *z_a_1.offset(0 as isize) } != 0 &&
            unsafe { *z_a_1.offset(0 as isize) } as i32 ==
                unsafe { *z_b_1.offset(0 as isize) } as i32 {
        dc = unsafe { *z_a_1.offset(0 as isize) } as i32;
        {
            let __p = &mut z_a_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        {
            let __p = &mut z_b_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        { let __p = &mut n_match; let __t = *__p; *__p += 1; __t };
    }
    if !(pn_match_1).is_null() { unsafe { *pn_match_1 = n_match }; }
    if unsafe { *z_a_1.offset(0 as isize) } as i32 == 0 &&
            unsafe { *z_b_1.offset(0 as isize) } as i32 == 0 {
        return 0;
    }
    {
        n_a = 0 as u32;
        '__b3: loop {
            if !(unsafe { *z_a_1.add(n_a as usize) } != 0) { break '__b3; }
            '__c3: loop {
                if unsafe { *z_a_1.add(n_a as usize) } as i32 & 128 != 0 {
                    return -2;
                }
                break '__c3;
            }
            { let __p = &mut n_a; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_a >= 100000 as u32 { return -4; }
    {
        n_b = 0 as u32;
        '__b4: loop {
            if !(unsafe { *z_b_1.add(n_b as usize) } != 0) { break '__b4; }
            '__c4: loop {
                if unsafe { *z_b_1.add(n_b as usize) } as i32 & 128 != 0 {
                    return -2;
                }
                break '__c4;
            }
            { let __p = &mut n_b; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_b >= 100000 as u32 { return -4; }
    if n_a == 0 as u32 {
        c_bprev = dc as i8;
        {
            x_b = { res = 0; res } as u32;
            '__b5: loop {
                if !({ c_b = unsafe { *z_b_1.add(x_b as usize) } as i8; c_b }
                                    as i32 != 0) {
                    break '__b5;
                }
                '__c5: loop {
                    res +=
                        insert_or_delete_cost(c_bprev, c_b,
                                unsafe { *z_b_1.add((x_b + 1 as u32) as usize) }) / 4;
                    c_bprev = c_b;
                    break '__c5;
                }
                { let __p = &mut x_b; let __t = *__p; *__p += 1; __t };
            }
        }
        return res;
    }
    if n_b == 0 as u32 {
        c_aprev = dc as i8;
        {
            x_a = { res = 0; res } as u32;
            '__b6: loop {
                if !({ c_a = unsafe { *z_a_1.add(x_a as usize) } as i8; c_a }
                                    as i32 != 0) {
                    break '__b6;
                }
                '__c6: loop {
                    res +=
                        insert_or_delete_cost(c_aprev, c_a,
                            unsafe { *z_a_1.add((x_a + 1 as u32) as usize) });
                    c_aprev = c_a;
                    break '__c6;
                }
                { let __p = &mut x_a; let __t = *__p; *__p += 1; __t };
            }
        }
        return res;
    }
    if unsafe { *z_a_1.offset(0 as isize) } as i32 == '*' as i32 &&
            unsafe { *z_a_1.offset(1 as isize) } as i32 == 0 {
        return 0;
    }
    if (n_b as u64) <
            core::mem::size_of::<[i32; 75]>() as u64 * 4 as u64 /
                (core::mem::size_of::<i32>() as u64 * 5 as u64) {
        m = &raw mut m_stack[0 as usize] as *mut i32;
    } else {
        m =
            {
                to_free =
                    unsafe {
                            sqlite3_malloc64(((n_b as i64 + 1 as i64) * 5) as u64 *
                                        core::mem::size_of::<i32>() as u64 / 4 as u64)
                        } as *mut i32;
                to_free
            };
        if m == core::ptr::null_mut() { return -3; }
    }
    cx = unsafe { &raw mut *m.add((n_b + 1 as u32) as usize) } as *mut i8;

    /// Compute the Wagner edit distance
    unsafe { *m.offset(0 as isize) = 0 };
    unsafe { *cx.offset(0 as isize) = dc as i8 };
    c_bprev = dc as i8;
    {
        x_b = 1 as u32;
        '__b7: loop {
            if !(x_b <= n_b) { break '__b7; }
            '__c7: loop {
                c_bnext = unsafe { *z_b_1.add(x_b as usize) } as i8;
                c_b = unsafe { *z_b_1.add((x_b - 1 as u32) as usize) } as i8;
                unsafe { *cx.add(x_b as usize) = c_b };
                unsafe {
                    *m.add(x_b as usize) =
                        unsafe { *m.add((x_b - 1 as u32) as usize) } +
                            insert_or_delete_cost(c_bprev, c_b, c_bnext)
                };
                c_bprev = c_b;
                break '__c7;
            }
            { let __p = &mut x_b; let __t = *__p; *__p += 1; __t };
        }
    }
    c_aprev = dc as i8;
    {
        x_a = 1 as u32;
        '__b8: loop {
            if !(x_a <= n_a) { break '__b8; }
            '__c8: loop {
                let last_a: i32 = (x_a == n_a) as i32;
                c_a = unsafe { *z_a_1.add((x_a - 1 as u32) as usize) } as i8;
                c_anext = unsafe { *z_a_1.add(x_a as usize) } as i8;
                if c_a as i32 == '*' as i32 && last_a != 0 { break '__b8; }
                d = unsafe { *m.offset(0 as isize) };
                dc = unsafe { *cx.offset(0 as isize) } as i32;
                unsafe {
                    *m.offset(0 as isize) =
                        d + insert_or_delete_cost(c_aprev, c_a, c_anext)
                };
                c_bprev = 0 as i8;
                {
                    x_b = 1 as u32;
                    '__b9: loop {
                        if !(x_b <= n_b) { break '__b9; }
                        '__c9: loop {
                            let mut total_cost: i32 = 0;
                            let mut ins_cost: i32 = 0;
                            let mut del_cost: i32 = 0;
                            let mut sub_cost: i32 = 0;
                            let mut ncx: i32 = 0;
                            c_b =
                                unsafe { *z_b_1.add((x_b - 1 as u32) as usize) } as i8;
                            c_bnext = unsafe { *z_b_1.add(x_b as usize) } as i8;

                            /// Cost to insert cB
                            (ins_cost =
                                insert_or_delete_cost(unsafe {
                                        *cx.add((x_b - 1 as u32) as usize)
                                    }, c_b, c_bnext));
                            if last_a != 0 { ins_cost /= 4; }

                            /// Cost to delete cA
                            (del_cost =
                                insert_or_delete_cost(unsafe { *cx.add(x_b as usize) }, c_a,
                                    c_bnext));

                            /// Cost to substitute cA->cB
                            (sub_cost =
                                substitute_cost(unsafe {
                                        *cx.add((x_b - 1 as u32) as usize)
                                    }, c_a, c_b));

                            /// Best cost
                            (total_cost =
                                ins_cost + unsafe { *m.add((x_b - 1 as u32) as usize) });
                            ncx = c_b as i32;
                            if del_cost + unsafe { *m.add(x_b as usize) } < total_cost {
                                total_cost = del_cost + unsafe { *m.add(x_b as usize) };
                                ncx = c_a as i32;
                            }
                            if sub_cost + d < total_cost { total_cost = sub_cost + d; }

                            /// Update the matrix
                            (d = unsafe { *m.add(x_b as usize) });
                            dc = unsafe { *cx.add(x_b as usize) } as i32;
                            unsafe { *m.add(x_b as usize) = total_cost };
                            unsafe { *cx.add(x_b as usize) = ncx as i8 };
                            c_bprev = c_b;
                            break '__c9;
                        }
                        { let __p = &mut x_b; let __t = *__p; *__p += 1; __t };
                    }
                }
                c_aprev = c_a;
                break '__c8;
            }
            { let __p = &mut x_a; let __t = *__p; *__p += 1; __t };
        }
    }
    if c_a as i32 == '*' as i32 {
        res = unsafe { *m.offset(1 as isize) };
        {
            x_b = 1 as u32;
            '__b10: loop {
                if !(x_b <= n_b) { break '__b10; }
                '__c10: loop {
                    if unsafe { *m.add(x_b as usize) } < res {
                        res = unsafe { *m.add(x_b as usize) };
                        if !(pn_match_1).is_null() {
                            unsafe { *pn_match_1 = (x_b + n_match as u32) as i32 };
                        }
                    }
                    break '__c10;
                }
                { let __p = &mut x_b; let __t = *__p; *__p += 1; __t };
            }
        }
    } else {
        res = unsafe { *m.add(n_b as usize) };

        /// In the current implementation, pnMatch is always NULL if zA does
        ///* not end in "*"
        { let _ = 0; };
    }
    unsafe { sqlite3_free(to_free as *mut ()) };
    return res;
}

///* Function:    editdist(A,B)
///*
///* Return the cost of transforming string A into string B.  Both strings
///* must be pure ASCII text.  If A ends with '*' then it is assumed to be
///* a prefix of B and extra characters on the end of B have minimal additional
///* cost.
extern "C" fn editdist_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let res: i32 =
        editdist1(unsafe {
                    sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                } as *const i8,
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8, core::ptr::null_mut());
    if res < 0 {
        if res == -3 {
            unsafe { sqlite3_result_error_nomem(context) };
        } else if res == -4 {
            unsafe { sqlite3_result_error_toobig(context) };
        } else if res == -2 {
            unsafe {
                sqlite3_result_error(context,
                    c"non-ASCII input to editdist()".as_ptr() as *mut i8 as
                        *const i8, -1)
            };
        } else {
            unsafe {
                sqlite3_result_error(context,
                    c"NULL input to editdist()".as_ptr() as *mut i8 as
                        *const i8, -1)
            };
        }
    } else { unsafe { sqlite3_result_int(context, res) }; }
}

///* An entry in the edit cost table
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3Cost {
    p_next: *mut EditDist3Cost,
    n_from: u8,
    n_to: u8,
    i_cost: u16,
    a: [i8; 4],
}

///* Complete configuration
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3Config {
    n_lang: i32,
    a: *mut EditDist3Lang,
}

///* Edit costs for a particular language ID
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3Lang {
    i_lang: i32,
    i_ins_cost: i32,
    i_del_cost: i32,
    i_sub_cost: i32,
    p_cost: *mut EditDist3Cost,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3Point {
    _opaque: [u8; 0],
}

///* Extra information about each character in the FROM string.
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3From {
    n_subst: i32,
    n_del: i32,
    n_byte: i32,
    ap_subst: *mut *mut EditDist3Cost,
    ap_del: *mut *mut EditDist3Cost,
}

///* A precompiled FROM string.
///
///* In the common case we expect the FROM string to be reused multiple times.
///* In other words, the common case will be to measure the edit distance
///* from a single origin string to multiple target strings.
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3FromString {
    z: *mut i8,
    n: i32,
    is_prefix: i32,
    a: *mut EditDist3From,
}

///* Extra information about each character in the TO string.
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3To {
    n_ins: i32,
    n_byte: i32,
    ap_ins: *mut *mut EditDist3Cost,
}

///* A precompiled FROM string
#[repr(C)]
#[derive(Copy, Clone)]
struct EditDist3ToString {
    z: *mut i8,
    n: i32,
    a: *mut EditDist3To,
}

///* The default EditDist3Lang object, with default costs.
static mut edit_dist3_lang: EditDist3Lang =
    EditDist3Lang {
        i_lang: 0,
        i_ins_cost: 100,
        i_del_cost: 100,
        i_sub_cost: 150,
        p_cost: core::ptr::null_mut(),
    };

///* Clear or delete an instance of the object that records all edit-distance
///* weights.
extern "C" fn edit_dist3_config_clear(p: *mut EditDist3Config) -> () {
    let mut i: i32 = 0;
    if p == core::ptr::null_mut() { return; }
    {
        i = 0;
        '__b11: loop {
            if !(i < unsafe { (*p).n_lang }) { break '__b11; }
            '__c11: loop {
                let mut p_cost: *mut EditDist3Cost = core::ptr::null_mut();
                let mut p_next: *mut EditDist3Cost = core::ptr::null_mut();
                p_cost =
                    unsafe { (*unsafe { (*p).a.offset(i as isize) }).p_cost };
                while !(p_cost).is_null() {
                    p_next = unsafe { (*p_cost).p_next };
                    unsafe { sqlite3_free(p_cost as *mut ()) };
                    p_cost = p_next;
                }
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p).a } as *mut ()) };
    unsafe {
        memset(p as *mut (), 0,
            core::mem::size_of::<EditDist3Config>() as u64)
    };
}

extern "C" fn edit_dist3_config_delete(p_in_1: *mut ()) -> () {
    let p: *mut EditDist3Config = p_in_1 as *mut EditDist3Config;
    edit_dist3_config_clear(p);
    unsafe { sqlite3_free(p as *mut ()) };
}

/// Compare the FROM values of two EditDist3Cost objects, for sorting.
///* Return negative, zero, or positive if the A is less than, equal to,
///* or greater than B.
extern "C" fn edit_dist3_cost_compare(p_a_1: &EditDist3Cost,
    p_b_1: &EditDist3Cost) -> i32 {
    let mut n: i32 = (*p_a_1).n_from as i32;
    let mut rc: i32 = 0;
    if n > (*p_b_1).n_from as i32 { n = (*p_b_1).n_from as i32; }
    rc =
        unsafe {
            strncmp(&raw const (*p_a_1).a[0 as usize] as *mut i8 as *const i8,
                &raw const (*p_b_1).a[0 as usize] as *mut i8 as *const i8,
                n as u64)
        };
    if rc == 0 { rc = (*p_a_1).n_from as i32 - (*p_b_1).n_from as i32; }
    return rc;
}

///* Merge together two sorted lists of EditDist3Cost objects, in order
///* of increasing FROM.
extern "C" fn edit_dist3_cost_merge(mut p_a_1: *mut EditDist3Cost,
    mut p_b_1: *mut EditDist3Cost) -> *mut EditDist3Cost {
    let mut p_head: *mut EditDist3Cost = core::ptr::null_mut();
    let mut pp_tail: *mut *mut EditDist3Cost = &mut p_head;
    let mut p: *mut EditDist3Cost = core::ptr::null_mut();
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if edit_dist3_cost_compare(unsafe { &*p_a_1 }, unsafe { &*p_b_1 }) <=
                0 {
            p = p_a_1;
            p_a_1 = unsafe { (*p_a_1).p_next };
        } else { p = p_b_1; p_b_1 = unsafe { (*p_b_1).p_next }; }
        unsafe { *pp_tail = p };
        pp_tail = unsafe { &mut (*p).p_next };
    }
    if !(p_a_1).is_null() {
        unsafe { *pp_tail = p_a_1 };
    } else { unsafe { *pp_tail = p_b_1 }; }
    return p_head;
}

///* Sort a list of EditDist3Cost objects into order of increasing FROM
extern "C" fn edit_dist3_cost_sort(mut p_list_1: *mut EditDist3Cost)
    -> *mut EditDist3Cost {
    let mut ap: [*mut EditDist3Cost; 60] = [core::ptr::null_mut(); 60];
    let mut p: *mut EditDist3Cost = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut mx: i32 = 0;
    ap[0 as usize] = core::ptr::null_mut();
    ap[1 as usize] = core::ptr::null_mut();
    while !(p_list_1).is_null() {
        p = p_list_1;
        p_list_1 = unsafe { (*p).p_next };
        unsafe { (*p).p_next = core::ptr::null_mut() };
        {
            i = 0;
            '__b15: loop {
                if !(!(ap[i as usize]).is_null()) { break '__b15; }
                '__c15: loop {
                    p = edit_dist3_cost_merge(ap[i as usize], p);
                    ap[i as usize] = core::ptr::null_mut();
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        ap[i as usize] = p;
        if i > mx { mx = i; ap[(i + 1) as usize] = core::ptr::null_mut(); }
    }
    p = core::ptr::null_mut();
    {
        i = 0;
        '__b16: loop {
            if !(i <= mx) { break '__b16; }
            '__c16: loop {
                if !(ap[i as usize]).is_null() {
                    p = edit_dist3_cost_merge(p, ap[i as usize]);
                }
                break '__c16;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return p;
}

///* Load all edit-distance weights from a table.
extern "C" fn edit_dist3_config_load(p: *mut EditDist3Config,
    db: *mut Sqlite3, z_table_1: *const i8) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut rc2: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut i_lang_prev: i32 = -9999;
    let mut p_lang: *mut EditDist3Lang = core::ptr::null_mut();
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT iLang, cFrom, cTo, iCost FROM \"%w\" WHERE iLang>=0 ORDER BY iLang".as_ptr()
                        as *mut i8 as *const i8, z_table_1)
        };
    if z_sql == core::ptr::null_mut() { return 7; }
    rc =
        unsafe {
            sqlite3_prepare(db, z_sql as *const i8, -1, &mut p_stmt,
                core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc != 0 { return rc; }
    edit_dist3_config_clear(p);
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        let i_lang: i32 = unsafe { sqlite3_column_int(p_stmt, 0) };
        let z_from: *const i8 =
            unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
        let n_from: i32 =
            if !(z_from).is_null() {
                unsafe { sqlite3_column_bytes(p_stmt, 1) }
            } else { 0 };
        let z_to: *const i8 =
            unsafe { sqlite3_column_text(p_stmt, 2) } as *const i8;
        let n_to: i32 =
            if !(z_to).is_null() {
                unsafe { sqlite3_column_bytes(p_stmt, 2) }
            } else { 0 };
        let i_cost: i32 = unsafe { sqlite3_column_int(p_stmt, 3) };
        { let _ = 0; };
        { let _ = 0; };
        if n_from > 100 || n_to > 100 { continue; }
        if i_cost < 0 { continue; }
        if i_cost >= 10000 { continue; }
        if p_lang == core::ptr::null_mut() || i_lang != i_lang_prev {
            let mut p_new: *mut EditDist3Lang = core::ptr::null_mut();
            p_new =
                unsafe {
                        sqlite3_realloc64(unsafe { (*p).a } as *mut (),
                            (unsafe { (*p).n_lang } as i64 + 1 as i64) as u64 *
                                core::mem::size_of::<EditDist3Lang>() as u64)
                    } as *mut EditDist3Lang;
            if p_new == core::ptr::null_mut() { rc = 7; break; }
            unsafe { (*p).a = p_new };
            p_lang =
                unsafe {
                    unsafe { (*p).a.offset(unsafe { (*p).n_lang } as isize) }
                };
            {
                let __p = unsafe { &mut (*p).n_lang };
                let __t = *__p;
                *__p += 1;
                __t
            };
            unsafe { (*p_lang).i_lang = i_lang };
            unsafe { (*p_lang).i_ins_cost = 100 };
            unsafe { (*p_lang).i_del_cost = 100 };
            unsafe { (*p_lang).i_sub_cost = 150 };
            unsafe { (*p_lang).p_cost = core::ptr::null_mut() };
            i_lang_prev = i_lang;
        }
        if n_from == 1 &&
                    unsafe { *z_from.offset(0 as isize) } as i32 == '?' as i32
                && n_to == 0 {
            unsafe { (*p_lang).i_del_cost = i_cost };
        } else if n_from == 0 && n_to == 1 &&
                unsafe { *z_to.offset(0 as isize) } as i32 == '?' as i32 {
            unsafe { (*p_lang).i_ins_cost = i_cost };
        } else if n_from == 1 && n_to == 1 &&
                    unsafe { *z_from.offset(0 as isize) } as i32 == '?' as i32
                && unsafe { *z_to.offset(0 as isize) } as i32 == '?' as i32 {
            unsafe { (*p_lang).i_sub_cost = i_cost };
        } else {
            let mut p_cost: *mut EditDist3Cost = core::ptr::null_mut();
            let mut n_extra: i32 = n_from + n_to - 4;
            if n_extra < 0 { n_extra = 0; }
            p_cost =
                unsafe {
                        sqlite3_malloc64(core::mem::size_of::<EditDist3Cost>() as
                                    u64 + n_extra as u64)
                    } as *mut EditDist3Cost;
            if p_cost == core::ptr::null_mut() { rc = 7; break; }
            unsafe { (*p_cost).n_from = n_from as u8 };
            unsafe { (*p_cost).n_to = n_to as u8 };
            unsafe { (*p_cost).i_cost = i_cost as u16 };
            unsafe {
                memcpy(unsafe { &raw mut (*p_cost).a[0 as usize] } as *mut i8
                        as *mut (), z_from as *const (), n_from as u64)
            };
            unsafe {
                memcpy(unsafe {
                            (unsafe { &raw mut (*p_cost).a[0 as usize] } as
                                    *mut i8).offset(n_from as isize)
                        } as *mut (), z_to as *const (), n_to as u64)
            };
            unsafe { (*p_cost).p_next = unsafe { (*p_lang).p_cost } };
            unsafe { (*p_lang).p_cost = p_cost };
        }
    }
    rc2 = unsafe { sqlite3_finalize(p_stmt) };
    if rc == 0 { rc = rc2; }
    if rc == 0 {
        let mut i_lang_1: i32 = 0;
        {
            i_lang_1 = 0;
            '__b18: loop {
                if !(i_lang_1 < unsafe { (*p).n_lang }) { break '__b18; }
                '__c18: loop {
                    unsafe {
                        (*unsafe { (*p).a.offset(i_lang_1 as isize) }).p_cost =
                            edit_dist3_cost_sort(unsafe {
                                    (*unsafe { (*p).a.offset(i_lang_1 as isize) }).p_cost
                                })
                    };
                    break '__c18;
                }
                { let __p = &mut i_lang_1; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return rc;
}

///* Return the length (in bytes) of a utf-8 character.  Or return a maximum
///* of N.
extern "C" fn utf8_len(c: u8, n_1: i32) -> i32 {
    let mut len: i32 = 1;
    if c as i32 > 127 {
        if c as i32 & 224 == 192 {
            len = 2;
        } else if c as i32 & 240 == 224 { len = 3; } else { len = 4; }
    }
    if len > n_1 { len = n_1; }
    return len;
}

///* Return TRUE (non-zero) if the To side of the given cost matches
///* the given string.
extern "C" fn match_to(p: &EditDist3Cost, z: *const i8, n: i32) -> i32 {
    { let _ = 0; };
    if (*p).a[(*p).n_from as usize] as i32 !=
            unsafe { *z.offset(0 as isize) } as i32 {
        return 0;
    }
    if (*p).n_to as i32 > n { return 0; }
    if unsafe {
                strncmp(unsafe {
                            (&raw const (*p).a[0 as usize] as
                                    *mut i8).add((*p).n_from as usize)
                        } as *const i8, z, (*p).n_to as u64)
            } != 0 {
        return 0;
    }
    return 1;
}

///* Return TRUE (non-zero) if the From side of the given cost matches
///* the given string.
extern "C" fn match_from(p: &EditDist3Cost, z: *const i8, n: i32) -> i32 {
    { let _ = 0; };
    if (*p).n_from != 0 {
        if (*p).a[0 as usize] as i32 !=
                unsafe { *z.offset(0 as isize) } as i32 {
            return 0;
        }
        if unsafe {
                    strncmp(&raw const (*p).a[0 as usize] as *mut i8 as
                            *const i8, z, (*p).n_from as u64)
                } != 0 {
            return 0;
        }
    }
    return 1;
}

///* Return TRUE (non-zero) of the next FROM character and the next TO
///* character are the same.
extern "C" fn match_from_to(p_str_1: &EditDist3FromString, n1: i32,
    z2: *const i8, n2: i32) -> i32 {
    let b1: i32 = unsafe { (*(*p_str_1).a.offset(n1 as isize)).n_byte };
    if b1 > n2 { return 0; }
    { let _ = 0; };
    if unsafe { *(*p_str_1).z.offset(n1 as isize) } as i32 !=
            unsafe { *z2.offset(0 as isize) } as i32 {
        return 0;
    }
    if unsafe {
                strncmp(unsafe { (*p_str_1).z.offset(n1 as isize) } as
                        *const i8, z2, b1 as u64)
            } != 0 {
        return 0;
    }
    return 1;
}

///* Delete an EditDist3FromString objecct
extern "C" fn edit_dist3_from_string_delete(p: *mut EditDist3FromString)
    -> () {
    let mut i: i32 = 0;
    if !(p).is_null() {
        {
            i = 0;
            '__b19: loop {
                if !(i < unsafe { (*p).n }) { break '__b19; }
                '__c19: loop {
                    unsafe {
                        sqlite3_free(unsafe {
                                    (*unsafe { (*p).a.offset(i as isize) }).ap_del
                                } as *mut ())
                    };
                    unsafe {
                        sqlite3_free(unsafe {
                                    (*unsafe { (*p).a.offset(i as isize) }).ap_subst
                                } as *mut ())
                    };
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

///* Create a EditDist3FromString object.
extern "C" fn edit_dist3_from_string_new(p_lang_1: &EditDist3Lang,
    z: *const i8, mut n: i32) -> *mut EditDist3FromString {
    let mut p_str: *mut EditDist3FromString = core::ptr::null_mut();
    let mut p: *mut EditDist3Cost = core::ptr::null_mut();
    let mut i: i32 = 0;
    if z == core::ptr::null() { return core::ptr::null_mut(); }
    if n < 0 { n = unsafe { strlen(z) } as i32; }
    p_str =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<EditDist3FromString>()
                                    as u64 +
                                core::mem::size_of::<EditDist3From>() as u64 * n as u64 +
                            n as i64 as u64 + 1 as u64)
            } as *mut EditDist3FromString;
    if p_str == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe {
        (*p_str).a =
            unsafe { &raw mut *p_str.offset(1 as isize) } as
                *mut EditDist3From
    };
    unsafe {
        memset(unsafe { (*p_str).a } as *mut (), 0,
            core::mem::size_of::<EditDist3From>() as u64 * n as u64)
    };
    unsafe { (*p_str).n = n };
    unsafe {
        (*p_str).z =
            unsafe { &raw mut *unsafe { (*p_str).a.offset(n as isize) } } as
                *mut i8
    };
    unsafe {
        memcpy(unsafe { (*p_str).z } as *mut (), z as *const (),
            (n + 1) as u64)
    };
    if n != 0 && unsafe { *z.offset((n - 1) as isize) } as i32 == '*' as i32 {
        unsafe { (*p_str).is_prefix = 1 };
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
        {
            let __p = unsafe { &mut (*p_str).n };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        unsafe { *unsafe { (*p_str).z.offset(n as isize) } = 0 as i8 };
    } else { unsafe { (*p_str).is_prefix = 0 }; }
    {
        i = 0;
        '__b20: loop {
            if !(i < n) { break '__b20; }
            '__c20: loop {
                let p_from: *mut EditDist3From =
                    unsafe { &mut *unsafe { (*p_str).a.offset(i as isize) } };
                unsafe {
                    memset(p_from as *mut (), 0,
                        core::mem::size_of::<EditDist3From>() as u64)
                };
                unsafe {
                    (*p_from).n_byte =
                        utf8_len(unsafe { *z.offset(i as isize) } as u8, n - i)
                };
                {
                    p = (*p_lang_1).p_cost;
                    '__b21: loop {
                        if !(!(p).is_null()) { break '__b21; }
                        '__c21: loop {
                            let mut ap_new: *mut *mut EditDist3Cost =
                                core::ptr::null_mut();
                            if i + unsafe { (*p).n_from } as i32 > n { break '__c21; }
                            if match_from(unsafe { &*p },
                                        unsafe { z.offset(i as isize) }, n - i) == 0 {
                                break '__c21;
                            }
                            if unsafe { (*p).n_to } as i32 == 0 {
                                ap_new =
                                    unsafe {
                                            sqlite3_realloc64(unsafe { (*p_from).ap_del } as *mut (),
                                                core::mem::size_of::<*mut EditDist3Cost>() as u64 *
                                                    (unsafe { (*p_from).n_del } as i64 + 1 as i64) as u64)
                                        } as *mut *mut EditDist3Cost;
                                if ap_new == core::ptr::null_mut() { break '__b21; }
                                unsafe { (*p_from).ap_del = ap_new };
                                unsafe {
                                    *ap_new.offset({
                                                        let __p = unsafe { &mut (*p_from).n_del };
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = p
                                };
                            } else {
                                ap_new =
                                    unsafe {
                                            sqlite3_realloc64(unsafe { (*p_from).ap_subst } as *mut (),
                                                core::mem::size_of::<*mut EditDist3Cost>() as u64 *
                                                    (unsafe { (*p_from).n_subst } as i64 + 1 as i64) as u64)
                                        } as *mut *mut EditDist3Cost;
                                if ap_new == core::ptr::null_mut() { break '__b21; }
                                unsafe { (*p_from).ap_subst = ap_new };
                                unsafe {
                                    *ap_new.offset({
                                                        let __p = unsafe { &mut (*p_from).n_subst };
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = p
                                };
                            }
                            break '__c21;
                        }
                        p = unsafe { (*p).p_next };
                    }
                }
                if !(p).is_null() {
                    edit_dist3_from_string_delete(p_str);
                    p_str = core::ptr::null_mut();
                    break '__b20;
                }
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return p_str;
}

///* Update entry m[i] such that it is the minimum of its current value
///* and m[j]+iCost.
extern "C" fn update_cost(m: *mut u32, i: i32, j: i32, i_cost_1: i32) -> () {
    let mut b: u32 = 0 as u32;
    { let _ = 0; };
    { let _ = 0; };
    b = unsafe { *m.offset(j as isize) } + i_cost_1 as u32;
    if b < unsafe { *m.offset(i as isize) } {
        unsafe { *m.offset(i as isize) = b };
    }
}

/// Compute the edit distance between two strings.
///*
///* If an error occurs, return a negative number which is the error code.
///*
///* If pnMatch is not NULL, then *pnMatch is set to the number of characters
///* (not bytes) in z2 that matched the search pattern in *pFrom. If pFrom does
///* not contain the pattern for a prefix-search, then this is always the number
///* of characters in z2. If pFrom does contain a prefix search pattern, then
///* it is the number of characters in the prefix of z2 that was deemed to 
///* match pFrom.
#[allow(unused_doc_comments)]
extern "C" fn edit_dist3_core(p_from_1: &EditDist3FromString, z2: *const i8,
    n2: i32, p_lang_1: &EditDist3Lang, pn_match_1: *mut i32) -> i32 {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut i1: i32 = 0;
    let mut b1: i32 = 0;
    let mut i2: i32 = 0;
    let mut b2: i32 = 0;
    let mut f: EditDist3FromString = unsafe { core::mem::zeroed() };
    let mut a2: *mut EditDist3To = core::ptr::null_mut();
    let mut m: *mut u32 = core::ptr::null_mut();
    let mut p_to_free: *mut u32 = core::ptr::null_mut();
    let mut sz_row: i32 = 0;
    let mut p: *mut EditDist3Cost = core::ptr::null_mut();
    let mut res: i32 = 0;
    let mut n_byte: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut stack_space: [u32; 256] = [0; 256];
    /// allocate the Wagner matrix and the aTo[] array for the TO string
    /// Out of memory
    /// Fill in the a1[] matrix for all characters of the TO string
    let mut ap_new: *mut *mut EditDist3Cost = core::ptr::null_mut();
    /// Out of memory
    /// Prepare to compute the minimum edit distance
    /// First fill in the top-row of the matrix with FROM deletion costs
    /// Fill in all subsequent rows, top-to-bottom, left-to-right
    let mut rx: i32 = 0;
    /// Starting index for current row
    let mut rxp: i32 = 0;
    /// Starting index for previous row
    let mut cx: i32 = 0;
    /// Index of current cell
    let mut cxp: i32 = 0;
    /// Index of cell immediately to the left
    let mut cxd: i32 = 0;
    /// Index of cell to the left and one row above
    let mut cxu: i32 = 0;
    /// Index of cell immediately above
    /// Enable for debugging
    /// Free memory allocations and return the result
    let mut b: i32 = 0;
    let mut n_extra: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s23:
            {
            match __state {
                0 => { __state = 3; }
                2 => { i2 = 0; __state = 127; }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { f = *p_from_1; __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { __state = 9; }
                9 => { __state = 10; }
                10 => { __state = 11; }
                11 => { __state = 12; }
                12 => { __state = 13; }
                13 => { __state = 14; }
                14 => {
                    if n2 > 10000 { __state = 16; } else { __state = 15; }
                }
                15 => {
                    if f.n > 10000 { __state = 18; } else { __state = 17; }
                }
                16 => { return -2; }
                17 => { n = (f.n + 1) * (n2 + 1); __state = 19; }
                18 => { return -2; }
                19 => { n = n + 1 & !1; __state = 20; }
                20 => {
                    n_byte =
                        (n as u64 * core::mem::size_of::<u32>() as u64 +
                                core::mem::size_of::<EditDist3To>() as u64 * n2 as u64) as
                            Sqlite3Uint64;
                    __state = 21;
                }
                21 => {
                    if n_byte <= core::mem::size_of::<[u32; 256]>() as u64 {
                        __state = 23;
                    } else { __state = 24; }
                }
                22 => {
                    a2 =
                        unsafe { &raw mut *m.offset(n as isize) } as
                            *mut EditDist3To;
                    __state = 28;
                }
                23 => {
                    m = &raw mut stack_space[0 as usize] as *mut u32;
                    __state = 25;
                }
                24 => {
                    m =
                        {
                            p_to_free = unsafe { sqlite3_malloc64(n_byte) } as *mut u32;
                            p_to_free
                        };
                    __state = 26;
                }
                25 => { p_to_free = core::ptr::null_mut(); __state = 22; }
                26 => {
                    if m == core::ptr::null_mut() {
                        __state = 27;
                    } else { __state = 22; }
                }
                27 => { return -1; }
                28 => {
                    unsafe {
                        memset(a2 as *mut (), 0,
                            core::mem::size_of::<EditDist3To>() as u64 * n2 as u64)
                    };
                    __state = 29;
                }
                29 => { i2 = 0; __state = 31; }
                30 => { sz_row = f.n + 1; __state = 53; }
                31 => { if i2 < n2 { __state = 32; } else { __state = 30; } }
                32 => {
                    unsafe {
                        (*a2.offset(i2 as isize)).n_byte =
                            utf8_len(unsafe { *z2.offset(i2 as isize) } as u8, n2 - i2)
                    };
                    __state = 34;
                }
                33 => {
                    { let __p = &mut i2; let __t = *__p; *__p += 1; __t };
                    __state = 31;
                }
                34 => { p = (*p_lang_1).p_cost; __state = 35; }
                35 => {
                    if !(p).is_null() { __state = 36; } else { __state = 33; }
                }
                36 => { __state = 38; }
                37 => { p = unsafe { (*p).p_next }; __state = 35; }
                38 => {
                    if unsafe { (*p).n_from } as i32 > 0 {
                        __state = 40;
                    } else { __state = 39; }
                }
                39 => {
                    if i2 + unsafe { (*p).n_to } as i32 > n2 {
                        __state = 42;
                    } else { __state = 41; }
                }
                40 => { __state = 33; }
                41 => {
                    if unsafe { (*p).a[0 as usize] } as i32 >
                            unsafe { *z2.offset(i2 as isize) } as i32 {
                        __state = 44;
                    } else { __state = 43; }
                }
                42 => { __state = 37; }
                43 => {
                    if match_to(unsafe { &*p },
                                unsafe { z2.offset(i2 as isize) }, n2 - i2) == 0 {
                        __state = 46;
                    } else { __state = 45; }
                }
                44 => { __state = 33; }
                45 => {
                    {
                        let __p = unsafe { &mut (*a2.offset(i2 as isize)).n_ins };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 47;
                }
                46 => { __state = 37; }
                47 => {
                    ap_new =
                        unsafe {
                                sqlite3_realloc64(unsafe {
                                            (*a2.offset(i2 as isize)).ap_ins
                                        } as *mut (),
                                    core::mem::size_of::<*mut EditDist3Cost>() as u64 *
                                        unsafe { (*a2.offset(i2 as isize)).n_ins } as u64)
                            } as *mut *mut EditDist3Cost;
                    __state = 48;
                }
                48 => {
                    if ap_new == core::ptr::null_mut() {
                        __state = 50;
                    } else { __state = 49; }
                }
                49 => {
                    unsafe { (*a2.offset(i2 as isize)).ap_ins = ap_new };
                    __state = 52;
                }
                50 => { res = -1; __state = 51; }
                51 => { __state = 2; }
                52 => {
                    unsafe {
                        *unsafe {
                                    (*a2.offset(i2 as
                                                        isize)).ap_ins.offset((unsafe {
                                                    (*a2.offset(i2 as isize)).n_ins
                                                } - 1) as isize)
                                } = p
                    };
                    __state = 37;
                }
                53 => {
                    unsafe {
                        memset(m as *mut (), 1,
                            ((n2 + 1) * sz_row) as u64 *
                                core::mem::size_of::<u32>() as u64)
                    };
                    __state = 54;
                }
                54 => {
                    unsafe { *m.offset(0 as isize) = 0 as u32 };
                    __state = 55;
                }
                55 => { i1 = 0; __state = 57; }
                56 => { i2 = 0; __state = 67; }
                57 => { if i1 < f.n { __state = 58; } else { __state = 56; } }
                58 => {
                    b1 = unsafe { (*f.a.offset(i1 as isize)).n_byte };
                    __state = 60;
                }
                59 => { i1 += b1; __state = 57; }
                60 => {
                    update_cost(m, i1 + b1, i1, (*p_lang_1).i_del_cost);
                    __state = 61;
                }
                61 => { k = 0; __state = 62; }
                62 => {
                    if k < unsafe { (*f.a.offset(i1 as isize)).n_del } {
                        __state = 63;
                    } else { __state = 59; }
                }
                63 => {
                    p =
                        unsafe {
                            *unsafe {
                                    (*f.a.offset(i1 as isize)).ap_del.offset(k as isize)
                                }
                        };
                    __state = 65;
                }
                64 => {
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    __state = 62;
                }
                65 => {
                    update_cost(m, i1 + unsafe { (*p).n_from } as i32, i1,
                        unsafe { (*p).i_cost } as i32);
                    __state = 64;
                }
                66 => {
                    res =
                        unsafe { *m.offset((sz_row * (n2 + 1) - 1) as isize) } as
                            i32;
                    __state = 108;
                }
                67 => { if i2 < n2 { __state = 68; } else { __state = 66; } }
                68 => { __state = 70; }
                69 => { i2 += b2; __state = 67; }
                70 => { __state = 71; }
                71 => {
                    b2 = unsafe { (*a2.offset(i2 as isize)).n_byte };
                    __state = 72;
                }
                72 => { rx = sz_row * (i2 + b2); __state = 73; }
                73 => { rxp = sz_row * i2; __state = 74; }
                74 => {
                    update_cost(m, rx, rxp, (*p_lang_1).i_ins_cost);
                    __state = 75;
                }
                75 => { k = 0; __state = 77; }
                76 => { i1 = 0; __state = 81; }
                77 => {
                    if k < unsafe { (*a2.offset(i2 as isize)).n_ins } {
                        __state = 78;
                    } else { __state = 76; }
                }
                78 => {
                    p =
                        unsafe {
                            *unsafe {
                                    (*a2.offset(i2 as isize)).ap_ins.offset(k as isize)
                                }
                        };
                    __state = 80;
                }
                79 => {
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    __state = 77;
                }
                80 => {
                    update_cost(m, sz_row * (i2 + unsafe { (*p).n_to } as i32),
                        rxp, unsafe { (*p).i_cost } as i32);
                    __state = 79;
                }
                81 => { if i1 < f.n { __state = 82; } else { __state = 69; } }
                82 => { __state = 84; }
                83 => { i1 += b1; __state = 81; }
                84 => { __state = 85; }
                85 => { __state = 86; }
                86 => { __state = 87; }
                87 => {
                    b1 = unsafe { (*f.a.offset(i1 as isize)).n_byte };
                    __state = 88;
                }
                88 => { cxp = rx + i1; __state = 89; }
                89 => { cx = cxp + b1; __state = 90; }
                90 => { cxd = rxp + i1; __state = 91; }
                91 => { cxu = cxd + b1; __state = 92; }
                92 => {
                    update_cost(m, cx, cxp, (*p_lang_1).i_del_cost);
                    __state = 93;
                }
                93 => { k = 0; __state = 95; }
                94 => {
                    update_cost(m, cx, cxu, (*p_lang_1).i_ins_cost);
                    __state = 99;
                }
                95 => {
                    if k < unsafe { (*f.a.offset(i1 as isize)).n_del } {
                        __state = 96;
                    } else { __state = 94; }
                }
                96 => {
                    p =
                        unsafe {
                            *unsafe {
                                    (*f.a.offset(i1 as isize)).ap_del.offset(k as isize)
                                }
                        };
                    __state = 98;
                }
                97 => {
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    __state = 95;
                }
                98 => {
                    update_cost(m, cxp + unsafe { (*p).n_from } as i32, cxp,
                        unsafe { (*p).i_cost } as i32);
                    __state = 97;
                }
                99 => {
                    if match_from_to(&f, i1, unsafe { z2.offset(i2 as isize) },
                                n2 - i2) != 0 {
                        __state = 101;
                    } else { __state = 100; }
                }
                100 => {
                    update_cost(m, cx, cxd, (*p_lang_1).i_sub_cost);
                    __state = 102;
                }
                101 => { update_cost(m, cx, cxd, 0); __state = 100; }
                102 => { k = 0; __state = 103; }
                103 => {
                    if k < unsafe { (*f.a.offset(i1 as isize)).n_subst } {
                        __state = 104;
                    } else { __state = 83; }
                }
                104 => {
                    p =
                        unsafe {
                            *unsafe {
                                    (*f.a.offset(i1 as isize)).ap_subst.offset(k as isize)
                                }
                        };
                    __state = 106;
                }
                105 => {
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    __state = 103;
                }
                106 => {
                    if match_to(unsafe { &*p },
                                unsafe { z2.offset(i2 as isize) }, n2 - i2) != 0 {
                        __state = 107;
                    } else { __state = 105; }
                }
                107 => {
                    update_cost(m,
                        cxd + unsafe { (*p).n_from } as i32 +
                            sz_row * unsafe { (*p).n_to } as i32, cxd,
                        unsafe { (*p).i_cost } as i32);
                    __state = 105;
                }
                108 => { n = n2; __state = 109; }
                109 => {
                    if f.is_prefix != 0 {
                        __state = 111;
                    } else { __state = 110; }
                }
                110 => {
                    if !(pn_match_1).is_null() {
                        __state = 119;
                    } else { __state = 118; }
                }
                111 => { i2 = 1; __state = 112; }
                112 => {
                    if i2 <= n2 { __state = 113; } else { __state = 110; }
                }
                113 => {
                    b = unsafe { *m.offset((sz_row * i2 - 1) as isize) } as i32;
                    __state = 115;
                }
                114 => {
                    { let __p = &mut i2; let __t = *__p; *__p += 1; __t };
                    __state = 112;
                }
                115 => {
                    if b <= res { __state = 116; } else { __state = 114; }
                }
                116 => { res = b; __state = 117; }
                117 => { n = i2 - 1; __state = 114; }
                118 => { __state = 2; }
                119 => { n_extra = 0; __state = 120; }
                120 => { k = 0; __state = 122; }
                121 => {
                    unsafe { *pn_match_1 = n - n_extra };
                    __state = 118;
                }
                122 => { if k < n { __state = 123; } else { __state = 121; } }
                123 => {
                    if unsafe { *z2.offset(k as isize) } as i32 & 192 == 128 {
                        __state = 125;
                    } else { __state = 124; }
                }
                124 => {
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    __state = 122;
                }
                125 => {
                    { let __p = &mut n_extra; let __t = *__p; *__p += 1; __t };
                    __state = 124;
                }
                126 => {
                    unsafe { sqlite3_free(p_to_free as *mut ()) };
                    __state = 130;
                }
                127 => {
                    if i2 < n2 { __state = 128; } else { __state = 126; }
                }
                128 => {
                    unsafe {
                        sqlite3_free(unsafe { (*a2.offset(i2 as isize)).ap_ins } as
                                *mut ())
                    };
                    __state = 129;
                }
                129 => {
                    { let __p = &mut i2; let __t = *__p; *__p += 1; __t };
                    __state = 127;
                }
                130 => { return res; }
                _ => {}
            }
        }
    }

    /// allocate the Wagner matrix and the aTo[] array for the TO string
    /// Out of memory
    /// Fill in the a1[] matrix for all characters of the TO string
    /// Out of memory
    /// Prepare to compute the minimum edit distance
    /// First fill in the top-row of the matrix with FROM deletion costs
    /// Fill in all subsequent rows, top-to-bottom, left-to-right
    /// Starting index for current row
    /// Starting index for previous row
    /// Index of current cell
    /// Index of cell immediately to the left
    /// Index of cell to the left and one row above
    /// Index of cell immediately above
    /// Enable for debugging
    /// Free memory allocations and return the result
    unreachable!();
}

///* Get an appropriate EditDist3Lang object.
extern "C" fn edit_dist3_find_lang(p_config_1: &EditDist3Config,
    i_lang_1: i32) -> *const EditDist3Lang {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b24: loop {
                if !(i < (*p_config_1).n_lang) { break '__b24; }
                '__c24: loop {
                    if unsafe { (*(*p_config_1).a.offset(i as isize)).i_lang }
                            == i_lang_1 {
                        return unsafe {
                                    &raw mut *(*p_config_1).a.offset(i as isize)
                                } as *const EditDist3Lang;
                    }
                    break '__c24;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return &edit_dist3_lang;
    }
}

///* Function:    editdist3(A,B,iLang)
///*              editdist3(tablename)
///*
///* Return the cost of transforming string A into string B using edit
///* weights for iLang.
///*
///* The second form loads edit weights into memory from a table.
extern "C" fn edit_dist3_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let p_config: *mut EditDist3Config =
        unsafe { sqlite3_user_data(context) } as *mut EditDist3Config;
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut rc: i32 = 0;
    if argc == 1 {
        let z_table: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        rc = edit_dist3_config_load(p_config, db, z_table);
        if rc != 0 { unsafe { sqlite3_result_error_code(context, rc) }; }
    } else {
        let z_a: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        let z_b: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8;
        let n_a: i32 =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        let n_b: i32 =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
            };
        let i_lang: i32 =
            if argc == 3 {
                unsafe {
                    sqlite3_value_int(unsafe { *argv.offset(2 as isize) })
                }
            } else { 0 };
        let p_lang: *const EditDist3Lang =
            edit_dist3_find_lang(unsafe { &*p_config }, i_lang);
        let mut p_from: *mut EditDist3FromString = core::ptr::null_mut();
        let mut dist: i32 = 0;
        p_from = edit_dist3_from_string_new(unsafe { &*p_lang }, z_a, n_a);
        if p_from == core::ptr::null_mut() {
            unsafe { sqlite3_result_error_nomem(context) };
            return;
        }
        dist =
            edit_dist3_core(unsafe { &*p_from }, z_b, n_b,
                unsafe { &*p_lang }, core::ptr::null_mut());
        edit_dist3_from_string_delete(p_from);
        if dist == -1 {
            unsafe { sqlite3_result_error_nomem(context) };
        } else if dist == -2 {
            unsafe { sqlite3_result_error_toobig(context) };
        } else { unsafe { sqlite3_result_int(context, dist) }; }
    }
}

///* Register the editDist3 function with SQLite
extern "C" fn edit_dist3_install(db: *mut Sqlite3) -> i32 {
    let mut rc: i32 = 0;
    let p_config: *mut EditDist3Config =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<EditDist3Config>() as
                        Sqlite3Uint64)
            } as *mut EditDist3Config;
    if p_config == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_config as *mut (), 0,
            core::mem::size_of::<EditDist3Config>() as u64)
    };
    rc =
        unsafe {
            sqlite3_create_function_v2(db,
                c"editdist3".as_ptr() as *mut i8 as *const i8, 2, 1 | 2048,
                p_config as *mut (), Some(edit_dist3_sql_func), None, None,
                None)
        };
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function_v2(db,
                    c"editdist3".as_ptr() as *mut i8 as *const i8, 3, 1 | 2048,
                    p_config as *mut (), Some(edit_dist3_sql_func), None, None,
                    None)
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function_v2(db,
                    c"editdist3".as_ptr() as *mut i8 as *const i8, 1, 1 | 2048,
                    p_config as *mut (), Some(edit_dist3_sql_func), None, None,
                    Some(edit_dist3_config_delete))
            };
    } else { unsafe { sqlite3_free(p_config as *mut ()) }; }
    return rc;
}

///* This lookup table is used to help decode the first byte of
///* a multi-byte UTF8 character.
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

///* Return the value of the first UTF-8 character in the string.
extern "C" fn utf8_read(z: &[u8], p_size_1: &mut i32) -> i32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    if z.len() as i32 == 0 {
        c = { i = 0; i };
    } else {
        c = z[0 as usize] as i32;
        i = 1;
        if c >= 192 {
            c = sqlite3_utf8_trans1[(c - 192) as usize] as i32;
            while i < z.len() as i32 && z[i as usize] as i32 & 192 == 128 {
                c =
                    (c << 6) +
                        (63 &
                            z[{ let __p = &mut i; let __t = *__p; *__p += 1; __t } as
                                        usize] as i32);
            }
        }
    }
    *p_size_1 = i;
    return c;
}

///* Return the number of characters in the utf-8 string in the nIn byte
///* buffer pointed to by zIn.
extern "C" fn utf8_charlen(z_in_1: &[i8]) -> i32 {
    let mut i: i32 = 0;
    let mut n_char: i32 = 0;
    {
        i = 0;
        '__b26: loop {
            if !(i < z_in_1.len() as i32) { break '__b26; }
            '__c26: loop {
                let mut sz: i32 = 0;
                utf8_read(unsafe {
                        let __p =
                            &raw const z_in_1[i as usize] as *const u8 as *const u8;
                        if __p.is_null() {
                            &[]
                        } else {
                            core::slice::from_raw_parts(__p,
                                (z_in_1.len() as i32 - i) as usize)
                        }
                    }, &mut sz);
                i += sz;
                break '__c26;
            }
            { let __p = &mut n_char; let __t = *__p; *__p += 1; __t };
        }
    }
    return n_char;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Transliteration {
    c_from: u16,
    c_to0: u8,
    c_to1: u8,
    c_to2: u8,
    c_to3: u8,
}

///* Table of translations from unicode characters into ASCII.
static translit: [Transliteration; 389] =
    [Transliteration {
                c_from: 160 as u16,
                c_to0: 32 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 181 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 192 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 193 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 194 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 195 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 196 as u16,
                c_to0: 65 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 197 as u16,
                c_to0: 65 as u8,
                c_to1: 97 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 198 as u16,
                c_to0: 65 as u8,
                c_to1: 69 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 199 as u16,
                c_to0: 67 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 200 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 201 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 202 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 203 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 204 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 205 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 206 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 207 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 208 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 209 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 210 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 211 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 212 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 213 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 214 as u16,
                c_to0: 79 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 215 as u16,
                c_to0: 120 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 216 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 217 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 218 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 219 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 220 as u16,
                c_to0: 85 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 221 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 222 as u16,
                c_to0: 84 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 223 as u16,
                c_to0: 115 as u8,
                c_to1: 115 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 224 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 225 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 226 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 227 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 228 as u16,
                c_to0: 97 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 229 as u16,
                c_to0: 97 as u8,
                c_to1: 97 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 230 as u16,
                c_to0: 97 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 231 as u16,
                c_to0: 99 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 232 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 233 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 234 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 235 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 236 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 237 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 238 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 239 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 240 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 241 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 242 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 243 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 244 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 245 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 246 as u16,
                c_to0: 111 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 247 as u16,
                c_to0: 58 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 248 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 249 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 250 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 251 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 252 as u16,
                c_to0: 117 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 253 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 254 as u16,
                c_to0: 116 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 255 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 256 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 257 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 258 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 259 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 260 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 261 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 262 as u16,
                c_to0: 67 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 263 as u16,
                c_to0: 99 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 264 as u16,
                c_to0: 67 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 265 as u16,
                c_to0: 99 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 266 as u16,
                c_to0: 67 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 267 as u16,
                c_to0: 99 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 268 as u16,
                c_to0: 67 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 269 as u16,
                c_to0: 99 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 270 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 271 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 272 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 273 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 274 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 275 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 276 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 277 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 278 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 279 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 280 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 281 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 282 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 283 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 284 as u16,
                c_to0: 71 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 285 as u16,
                c_to0: 103 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 286 as u16,
                c_to0: 71 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 287 as u16,
                c_to0: 103 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 288 as u16,
                c_to0: 71 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 289 as u16,
                c_to0: 103 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 290 as u16,
                c_to0: 71 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 291 as u16,
                c_to0: 103 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 292 as u16,
                c_to0: 72 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 293 as u16,
                c_to0: 104 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 294 as u16,
                c_to0: 72 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 295 as u16,
                c_to0: 104 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 296 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 297 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 298 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 299 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 300 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 301 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 302 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 303 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 304 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 305 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 306 as u16,
                c_to0: 73 as u8,
                c_to1: 74 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 307 as u16,
                c_to0: 105 as u8,
                c_to1: 106 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 308 as u16,
                c_to0: 74 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 309 as u16,
                c_to0: 106 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 310 as u16,
                c_to0: 75 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 311 as u16,
                c_to0: 107 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 312 as u16,
                c_to0: 107 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 313 as u16,
                c_to0: 76 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 314 as u16,
                c_to0: 108 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 315 as u16,
                c_to0: 76 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 316 as u16,
                c_to0: 108 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 317 as u16,
                c_to0: 76 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 318 as u16,
                c_to0: 108 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 319 as u16,
                c_to0: 76 as u8,
                c_to1: 46 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 320 as u16,
                c_to0: 108 as u8,
                c_to1: 46 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 321 as u16,
                c_to0: 76 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 322 as u16,
                c_to0: 108 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 323 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 324 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 325 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 326 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 327 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 328 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 329 as u16,
                c_to0: 39 as u8,
                c_to1: 110 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 330 as u16,
                c_to0: 78 as u8,
                c_to1: 71 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 331 as u16,
                c_to0: 110 as u8,
                c_to1: 103 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 332 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 333 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 334 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 335 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 336 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 337 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 338 as u16,
                c_to0: 79 as u8,
                c_to1: 69 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 339 as u16,
                c_to0: 111 as u8,
                c_to1: 101 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 340 as u16,
                c_to0: 82 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 341 as u16,
                c_to0: 114 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 342 as u16,
                c_to0: 82 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 343 as u16,
                c_to0: 114 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 344 as u16,
                c_to0: 82 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 345 as u16,
                c_to0: 114 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 346 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 347 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 348 as u16,
                c_to0: 83 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 349 as u16,
                c_to0: 115 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 350 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 351 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 352 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 353 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 354 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 355 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 356 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 357 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 358 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 359 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 360 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 361 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 362 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 363 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 364 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 365 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 366 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 367 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 368 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 369 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 370 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 371 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 372 as u16,
                c_to0: 87 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 373 as u16,
                c_to0: 119 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 374 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 375 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 376 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 377 as u16,
                c_to0: 90 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 378 as u16,
                c_to0: 122 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 379 as u16,
                c_to0: 90 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 380 as u16,
                c_to0: 122 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 381 as u16,
                c_to0: 90 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 382 as u16,
                c_to0: 122 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 383 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 402 as u16,
                c_to0: 102 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 536 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 537 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 538 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 539 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 902 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 904 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 905 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 906 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 908 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 910 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 911 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 912 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 913 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 914 as u16,
                c_to0: 66 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 915 as u16,
                c_to0: 71 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 916 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 917 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 918 as u16,
                c_to0: 90 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 919 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 920 as u16,
                c_to0: 84 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 921 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 922 as u16,
                c_to0: 75 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 923 as u16,
                c_to0: 76 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 924 as u16,
                c_to0: 77 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 925 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 926 as u16,
                c_to0: 88 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 927 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 928 as u16,
                c_to0: 80 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 929 as u16,
                c_to0: 82 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 931 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 932 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 933 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 934 as u16,
                c_to0: 70 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 935 as u16,
                c_to0: 67 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 936 as u16,
                c_to0: 80 as u8,
                c_to1: 115 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 937 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 938 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 939 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 940 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 941 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 942 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 943 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 945 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 946 as u16,
                c_to0: 98 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 947 as u16,
                c_to0: 103 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 948 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 949 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 950 as u16,
                c_to0: 122 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 951 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 952 as u16,
                c_to0: 116 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 953 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 954 as u16,
                c_to0: 107 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 955 as u16,
                c_to0: 108 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 956 as u16,
                c_to0: 109 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 957 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 958 as u16,
                c_to0: 120 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 959 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 960 as u16,
                c_to0: 112 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 961 as u16,
                c_to0: 114 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 963 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 964 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 965 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 966 as u16,
                c_to0: 102 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 967 as u16,
                c_to0: 99 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 968 as u16,
                c_to0: 112 as u8,
                c_to1: 115 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 969 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 970 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 971 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 972 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 973 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 974 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1024 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1025 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1026 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1027 as u16,
                c_to0: 71 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1028 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1029 as u16,
                c_to0: 90 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1030 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1031 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1032 as u16,
                c_to0: 74 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1033 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1034 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1035 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1036 as u16,
                c_to0: 75 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1037 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1038 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1039 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1040 as u16,
                c_to0: 65 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1041 as u16,
                c_to0: 66 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1042 as u16,
                c_to0: 86 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1043 as u16,
                c_to0: 71 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1044 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1045 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1046 as u16,
                c_to0: 90 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1047 as u16,
                c_to0: 90 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1048 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1049 as u16,
                c_to0: 73 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1050 as u16,
                c_to0: 75 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1051 as u16,
                c_to0: 76 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1052 as u16,
                c_to0: 77 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1053 as u16,
                c_to0: 78 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1054 as u16,
                c_to0: 79 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1055 as u16,
                c_to0: 80 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1056 as u16,
                c_to0: 82 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1057 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1058 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1059 as u16,
                c_to0: 85 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1060 as u16,
                c_to0: 70 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1061 as u16,
                c_to0: 75 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1062 as u16,
                c_to0: 84 as u8,
                c_to1: 99 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1063 as u16,
                c_to0: 67 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1064 as u16,
                c_to0: 83 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1065 as u16,
                c_to0: 83 as u8,
                c_to1: 104 as u8,
                c_to2: 99 as u8,
                c_to3: 104 as u8,
            },
            Transliteration {
                c_from: 1066 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1067 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1068 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1069 as u16,
                c_to0: 69 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1070 as u16,
                c_to0: 73 as u8,
                c_to1: 117 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1071 as u16,
                c_to0: 73 as u8,
                c_to1: 97 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1072 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1073 as u16,
                c_to0: 98 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1074 as u16,
                c_to0: 118 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1075 as u16,
                c_to0: 103 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1076 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1077 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1078 as u16,
                c_to0: 122 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1079 as u16,
                c_to0: 122 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1080 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1081 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1082 as u16,
                c_to0: 107 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1083 as u16,
                c_to0: 108 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1084 as u16,
                c_to0: 109 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1085 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1086 as u16,
                c_to0: 111 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1087 as u16,
                c_to0: 112 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1088 as u16,
                c_to0: 114 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1089 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1090 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1091 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1092 as u16,
                c_to0: 102 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1093 as u16,
                c_to0: 107 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1094 as u16,
                c_to0: 116 as u8,
                c_to1: 99 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1095 as u16,
                c_to0: 99 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1096 as u16,
                c_to0: 115 as u8,
                c_to1: 104 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1097 as u16,
                c_to0: 115 as u8,
                c_to1: 104 as u8,
                c_to2: 99 as u8,
                c_to3: 104 as u8,
            },
            Transliteration {
                c_from: 1098 as u16,
                c_to0: 97 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1099 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1100 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1101 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1102 as u16,
                c_to0: 105 as u8,
                c_to1: 117 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1103 as u16,
                c_to0: 105 as u8,
                c_to1: 97 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1104 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1105 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1106 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1107 as u16,
                c_to0: 103 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1108 as u16,
                c_to0: 101 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1109 as u16,
                c_to0: 122 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1110 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1111 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1112 as u16,
                c_to0: 106 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1113 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1114 as u16,
                c_to0: 110 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1115 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1116 as u16,
                c_to0: 107 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1117 as u16,
                c_to0: 105 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1118 as u16,
                c_to0: 117 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 1119 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7682 as u16,
                c_to0: 66 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7683 as u16,
                c_to0: 98 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7690 as u16,
                c_to0: 68 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7691 as u16,
                c_to0: 100 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7710 as u16,
                c_to0: 70 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7711 as u16,
                c_to0: 102 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7744 as u16,
                c_to0: 77 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7745 as u16,
                c_to0: 109 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7766 as u16,
                c_to0: 80 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7767 as u16,
                c_to0: 112 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7776 as u16,
                c_to0: 83 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7777 as u16,
                c_to0: 115 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7786 as u16,
                c_to0: 84 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7787 as u16,
                c_to0: 116 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7808 as u16,
                c_to0: 87 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7809 as u16,
                c_to0: 119 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7810 as u16,
                c_to0: 87 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7811 as u16,
                c_to0: 119 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7812 as u16,
                c_to0: 87 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7813 as u16,
                c_to0: 119 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7922 as u16,
                c_to0: 89 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 7923 as u16,
                c_to0: 121 as u8,
                c_to1: 0 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 64256 as u16,
                c_to0: 102 as u8,
                c_to1: 102 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 64257 as u16,
                c_to0: 102 as u8,
                c_to1: 105 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 64258 as u16,
                c_to0: 102 as u8,
                c_to1: 108 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 64261 as u16,
                c_to0: 115 as u8,
                c_to1: 116 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            },
            Transliteration {
                c_from: 64262 as u16,
                c_to0: 115 as u8,
                c_to1: 116 as u8,
                c_to2: 0 as u8,
                c_to3: 0 as u8,
            }];

extern "C" fn spellfix_find_translit(c: i32, px_top_1: &mut i32)
    -> *const Transliteration {
    *px_top_1 =
        (core::mem::size_of::<[Transliteration; 389]>() as u64 /
                    core::mem::size_of::<Transliteration>() as u64 - 1 as u64)
            as i32;
    return &raw const translit[0 as usize] as *const Transliteration;
}

///* Convert the input string from UTF-8 into pure ASCII by converting
///* all non-ASCII characters to some combination of characters in the
///* ASCII subset.
///*
///* The returned string might contain more characters than the input.
///*
///* Space to hold the returned string comes from sqlite3_malloc() and
///* should be freed by the caller.
extern "C" fn transliterate(mut z_in_1: *const u8, mut n_in_1: i32)
    -> *mut u8 {
    let z_out: *mut u8 =
        unsafe {
                sqlite3_malloc64((n_in_1 as i64 * 4 as i64 + 1 as i64) as
                        Sqlite3Uint64)
            } as *mut u8;
    let mut c: i32 = 0;
    let mut sz: i32 = 0;
    let mut n_out: i32 = 0;
    if z_out == core::ptr::null_mut() { return core::ptr::null_mut(); }
    n_out = 0;
    while n_in_1 > 0 {
        c =
            utf8_read(unsafe {
                    let __p = z_in_1 as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, n_in_1 as usize) }
                }, &mut sz);
        {
            let __n = sz;
            let __p = &mut z_in_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        n_in_1 -= sz;
        if c <= 127 {
            unsafe {
                *z_out.offset({
                                    let __p = &mut n_out;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize) = c as u8
            };
        } else {
            let mut x_top: i32 = 0;
            let mut x_btm: i32 = 0;
            let mut x: i32 = 0;
            let tbl: *const Transliteration =
                spellfix_find_translit(c, &mut x_top);
            x_btm = 0;
            while x_top >= x_btm {
                x = (x_top + x_btm) / 2;
                if unsafe { (*tbl.offset(x as isize)).c_from } as i32 == c {
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut n_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) =
                            unsafe { (*tbl.offset(x as isize)).c_to0 } as u8
                    };
                    if unsafe { (*tbl.offset(x as isize)).c_to1 } != 0 {
                        unsafe {
                            *z_out.offset({
                                                let __p = &mut n_out;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) =
                                unsafe { (*tbl.offset(x as isize)).c_to1 } as u8
                        };
                        if unsafe { (*tbl.offset(x as isize)).c_to2 } != 0 {
                            unsafe {
                                *z_out.offset({
                                                    let __p = &mut n_out;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) =
                                    unsafe { (*tbl.offset(x as isize)).c_to2 } as u8
                            };
                            if unsafe { (*tbl.offset(x as isize)).c_to3 } != 0 {
                                unsafe {
                                    *z_out.offset({
                                                        let __p = &mut n_out;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) =
                                        unsafe { (*tbl.offset(x as isize)).c_to3 } as u8
                                };
                            }
                        }
                    }
                    c = 0;
                    break;
                } else if unsafe { (*tbl.offset(x as isize)).c_from } as i32 >
                        c {
                    x_top = x - 1;
                } else { x_btm = x + 1; }
            }
            if c != 0 {
                unsafe {
                    *z_out.offset({
                                        let __p = &mut n_out;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = '?' as i32 as u8
                };
            }
        }
    }
    unsafe { *z_out.offset(n_out as isize) = 0 as u8 };
    return z_out;
}

///* Return the number of characters in the shortest prefix of the input
///* string that transliterates to an ASCII string nTrans bytes or longer.
///* Or, if the transliteration of the input string is less than nTrans
///* bytes in size, return the number of characters in the input string.
extern "C" fn translen_to_charlen(z_in_1: &[i8], n_trans_1: i32) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut sz: i32 = 0;
    let mut n_out: i32 = 0;
    let mut n_char: i32 = 0;
    i = { n_out = 0; n_out };
    {
        n_char = 0;
        '__b29: loop {
            if !(i < z_in_1.len() as i32 && n_out < n_trans_1) {
                break '__b29;
            }
            '__c29: loop {
                c =
                    utf8_read(unsafe {
                            let __p =
                                &raw const z_in_1[i as usize] as *const u8 as *const u8;
                            if __p.is_null() {
                                &[]
                            } else {
                                core::slice::from_raw_parts(__p,
                                    (z_in_1.len() as i32 - i) as usize)
                            }
                        }, &mut sz);
                i += sz;
                { let __p = &mut n_out; let __t = *__p; *__p += 1; __t };
                if c >= 128 {
                    let mut x_top: i32 = 0;
                    let mut x_btm: i32 = 0;
                    let mut x: i32 = 0;
                    let tbl: *const Transliteration =
                        spellfix_find_translit(c, &mut x_top);
                    x_btm = 0;
                    while x_top >= x_btm {
                        x = (x_top + x_btm) / 2;
                        if unsafe { (*tbl.offset(x as isize)).c_from } as i32 == c {
                            if unsafe { (*tbl.offset(x as isize)).c_to1 } != 0 {
                                { let __p = &mut n_out; let __t = *__p; *__p += 1; __t };
                                if unsafe { (*tbl.offset(x as isize)).c_to2 } != 0 {
                                    { let __p = &mut n_out; let __t = *__p; *__p += 1; __t };
                                    if unsafe { (*tbl.offset(x as isize)).c_to3 } != 0 {
                                        { let __p = &mut n_out; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                            break;
                        } else if unsafe { (*tbl.offset(x as isize)).c_from } as i32
                                > c {
                            x_top = x - 1;
                        } else { x_btm = x + 1; }
                    }
                }
                break '__c29;
            }
            { let __p = &mut n_char; let __t = *__p; *__p += 1; __t };
        }
    }
    return n_char;
}

///*    spellfix1_translit(X)
///*
///* Convert a string that contains non-ASCII Roman characters into 
///* pure ASCII.
extern "C" fn transliterate_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let z_in: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    let n_in: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    let z_out: *mut u8 = transliterate(z_in, n_in);
    if z_out == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
    } else {
        unsafe {
            sqlite3_result_text(context, z_out as *mut i8 as *const i8, -1,
                Some(sqlite3_free))
        };
    }
}

///*    spellfix1_scriptcode(X)
///*
///* Try to determine the dominant script used by the word X and return
///* its ISO 15924 numeric code.
///*
///* The current implementation only understands the following scripts:
///*
///*    215  (Latin)
///*    220  (Cyrillic)
///*    200  (Greek)
///*
///* This routine will return 998 if the input X contains characters from
///* two or more of the above scripts or 999 if X contains no characters
///* from any of the above scripts.
extern "C" fn script_code_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_in: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    let mut n_in: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    let mut c: i32 = 0;
    let mut sz: i32 = 0;
    let mut script_mask: i32 = 0;
    let mut res: i32 = 0;
    let mut seen_digit: i32 = 0;
    while n_in > 0 {
        c =
            utf8_read(unsafe {
                    let __p = z_in as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, n_in as usize) }
                }, &mut sz);
        {
            let __n = sz;
            let __p = &mut z_in;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        n_in -= sz;
        if c < 687 {
            if c >= 128 || (mid_class[(c & 127) as usize] as i32) < 10 {
                script_mask |= 1;
            } else if c >= '0' as i32 && c <= '9' as i32 { seen_digit = 1; }
        } else if c >= 1024 && c <= 1279 {
            script_mask |= 2;
        } else if c >= 902 && c <= 974 {
            script_mask |= 4;
        } else if c >= 1424 && c <= 1535 {
            script_mask |= 8;
        } else if c >= 1536 && c <= 1791 { script_mask |= 16; }
    }
    if script_mask == 0 && seen_digit != 0 { script_mask = 1; }
    '__s32:
        {
        match script_mask {
            0 => { res = 999; }
            1 => { res = 215; }
            2 => { res = 220; }
            4 => { res = 200; }
            8 => { res = 125; }
            16 => { res = 160; }
            _ => { res = 998; }
        }
    }
    unsafe { sqlite3_result_int(context, res) };
}

/// Fuzzy-search virtual table object
#[repr(C)]
#[derive(Copy, Clone)]
struct Spellfix1Vtab {
    base: Sqlite3Vtab,
    db: *mut Sqlite3,
    z_db_name: *mut i8,
    z_table_name: *mut i8,
    z_cost_table: *mut i8,
    p_config3: *mut EditDist3Config,
}

/// Fuzzy-search cursor object
#[repr(C)]
#[derive(Copy, Clone)]
struct Spellfix1Cursor {
    base: Sqlite3VtabCursor,
    p_v_tab: *mut Spellfix1Vtab,
    z_pattern: *mut i8,
    idx_num: i32,
    n_row: i32,
    n_alloc: i32,
    i_row: i32,
    i_lang: i32,
    i_top: i32,
    i_scope: i32,
    n_search: i32,
    p_full_scan: *mut Sqlite3Stmt,
    a: *mut Spellfix1Row,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Spellfix1Row {
    i_rowid: Sqlite3Int64,
    z_word: *mut i8,
    i_rank: i32,
    i_distance: i32,
    i_score: i32,
    i_matchlen: i32,
    z_hash: [i8; 32],
}

///* Construct one or more SQL statements from the format string given
///* and then evaluate those statements. The success code is written
///* into *pRc.
///*
///* If *pRc is initially non-zero then this routine is a no-op.
unsafe extern "C" fn spellfix1_db_exec(p_rc_1: &mut i32, db: *mut Sqlite3,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    if *p_rc_1 != 0 { return; }
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    if z_sql == core::ptr::null_mut() {
        *p_rc_1 = 7;
    } else {
        *p_rc_1 =
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
}

///* xDisconnect/xDestroy method for the fuzzy-search module.
extern "C" fn spellfix1_uninit(is_destroy_1: i32, p_v_tab_1: *mut Sqlite3Vtab)
    -> i32 {
    let p: *mut Spellfix1Vtab = p_v_tab_1 as *mut Spellfix1Vtab;
    let mut rc: i32 = 0;
    if is_destroy_1 != 0 {
        let db: *mut Sqlite3 = unsafe { (*p).db };
        unsafe {
            spellfix1_db_exec(&mut rc, db,
                c"DROP TABLE IF EXISTS \"%w\".\"%w_vocab\"".as_ptr() as
                        *mut i8 as *const i8, unsafe { (*p).z_db_name },
                unsafe { (*p).z_table_name })
        };
    }
    if rc == 0 {
        unsafe { sqlite3_free(unsafe { (*p).z_table_name } as *mut ()) };
        edit_dist3_config_delete(unsafe { (*p).p_config3 } as *mut ());
        unsafe { sqlite3_free(unsafe { (*p).z_cost_table } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
    return rc;
}

extern "C" fn spellfix1_disconnect(p_v_tab_1: *mut Sqlite3Vtab) -> i32 {
    return spellfix1_uninit(0, p_v_tab_1);
}

extern "C" fn spellfix1_destroy(p_v_tab_1: *mut Sqlite3Vtab) -> i32 {
    return spellfix1_uninit(1, p_v_tab_1);
}

///* Make a copy of a string.  Remove leading and trailing whitespace
///* and dequote it.
extern "C" fn spellfix1_dequote(mut z_in_1: *const i8) -> *mut i8 {
    let mut z_out: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: i8 = 0 as i8;
    while unsafe {
                isspace(unsafe { *z_in_1.offset(0 as isize) } as u8 as i32)
            } != 0 {
        {
            let __p = &mut z_in_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    z_out =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_in_1)
        };
    if z_out == core::ptr::null_mut() { return core::ptr::null_mut(); }
    i = unsafe { strlen(z_out as *const i8) } as i32;
    while i > 0 &&
            unsafe {
                    isspace(unsafe { *z_out.offset((i - 1) as isize) } as i32)
                } != 0 {
        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
    }
    unsafe { *z_out.offset(i as isize) = 0 as i8 };
    c = unsafe { *z_out.offset(0 as isize) };
    if c as i32 == '\'' as i32 || c as i32 == '\"' as i32 {
        {
            { i = 1; j = 0 };
            '__b35: loop {
                if !(unsafe { *z_out.offset(i as isize) } != 0) {
                    break '__b35;
                }
                '__c35: loop {
                    unsafe {
                        *z_out.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = unsafe { *z_out.offset(i as isize) }
                    };
                    if unsafe { *z_out.offset(i as isize) } as i32 == c as i32 {
                        if unsafe { *z_out.offset((i + 1) as isize) } as i32 ==
                                c as i32 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        } else {
                            unsafe { *z_out.offset((j - 1) as isize) = 0 as i8 };
                            break '__b35;
                        }
                    }
                    break '__c35;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return z_out;
}

///* xConnect/xCreate method for the spellfix1 module. Arguments are:
///*
///*   argv[0]   -> module name  ("spellfix1")
///*   argv[1]   -> database name
///*   argv[2]   -> table name
///*   argv[3].. -> optional arguments (i.e. "edit_cost_table" parameter)
#[allow(unused_doc_comments)]
extern "C" fn spellfix1_init(is_create_1: i32, db: *mut Sqlite3,
    p_aux_1: *mut (), argc: i32, argv: *const *const i8,
    pp_v_tab_1: &mut *mut Sqlite3Vtab, pz_err_1: &mut *mut i8) -> i32 {
    let mut p_new: *mut Spellfix1Vtab = core::ptr::null_mut();
    /// const char *zModule = argv[0]; // not used
    let z_db_name: *const i8 = unsafe { *argv.offset(1 as isize) };
    let z_table_name: *const i8 = unsafe { *argv.offset(2 as isize) };
    let mut n_db_name: i32 = 0;
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    n_db_name = unsafe { strlen(z_db_name) } as i32;
    p_new =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<Spellfix1Vtab>() as u64
                            + n_db_name as i64 as u64 + 1 as u64)
            } as *mut Spellfix1Vtab;
    if p_new == core::ptr::null_mut() {
        rc = 7;
    } else {
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<Spellfix1Vtab>() as u64)
        };
        unsafe {
            (*p_new).z_db_name =
                unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
        };
        unsafe {
            memcpy(unsafe { (*p_new).z_db_name } as *mut (),
                z_db_name as *const (), (n_db_name + 1) as u64)
        };
        unsafe {
            (*p_new).z_table_name =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_table_name)
                }
        };
        unsafe { (*p_new).db = db };
        if unsafe { (*p_new).z_table_name } == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe { sqlite3_vtab_config(db, 2) };
            rc =
                unsafe {
                    sqlite3_declare_vtab(db,
                        c"CREATE TABLE x(word,rank,distance,langid, score, matchlen, phonehash HIDDEN, top HIDDEN, scope HIDDEN, srchcnt HIDDEN, soundslike HIDDEN, command HIDDEN)".as_ptr()
                                as *mut i8 as *const i8)
                };
        }
        if rc == 0 && is_create_1 != 0 {
            unsafe {
                spellfix1_db_exec(&mut rc, db,
                    c"CREATE TABLE IF NOT EXISTS \"%w\".\"%w_vocab\"(\n  id INTEGER PRIMARY KEY,\n  rank INT,\n  langid INT,\n  word TEXT,\n  k1 TEXT,\n  k2 TEXT\n);\n".as_ptr()
                            as *mut i8 as *const i8, z_db_name, z_table_name)
            };
            unsafe {
                spellfix1_db_exec(&mut rc, db,
                    c"CREATE INDEX IF NOT EXISTS \"%w\".\"%w_vocab_index_langid_k2\" ON \"%w_vocab\"(langid,k2);".as_ptr()
                            as *mut i8 as *const i8, z_db_name, z_table_name,
                    z_table_name)
            };
        }
        {
            i = 3;
            '__b36: loop {
                if !(rc == 0 && i < argc) { break '__b36; }
                '__c36: loop {
                    if unsafe {
                                    strncmp(unsafe { *argv.offset(i as isize) },
                                        c"edit_cost_table=".as_ptr() as *mut i8 as *const i8,
                                        16 as u64)
                                } == 0 &&
                            unsafe { (*p_new).z_cost_table } == core::ptr::null_mut() {
                        unsafe {
                            (*p_new).z_cost_table =
                                spellfix1_dequote(unsafe {
                                        &*unsafe { (*argv.offset(i as isize)).offset(16 as isize) }
                                    })
                        };
                        if unsafe { (*p_new).z_cost_table } == core::ptr::null_mut()
                            {
                            rc = 7;
                        }
                        break '__c36;
                    }
                    *pz_err_1 =
                        unsafe {
                            sqlite3_mprintf(c"bad argument to spellfix1(): \"%s\"".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(i as isize) })
                        };
                    rc = 1;
                    break '__c36;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if rc != 0 && !(p_new).is_null() {
        *pp_v_tab_1 = core::ptr::null_mut();
        spellfix1_uninit(0, unsafe { &mut (*p_new).base });
    } else { *pp_v_tab_1 = p_new as *mut Sqlite3Vtab; }
    return rc;
}

///* The xConnect and xCreate methods
extern "C" fn spellfix1_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_v_tab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    return spellfix1_init(0, db, p_aux_1, argc, argv,
            unsafe { &mut *pp_v_tab_1 }, unsafe { &mut *pz_err_1 });
}

extern "C" fn spellfix1_create(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_v_tab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    return spellfix1_init(1, db, p_aux_1, argc, argv,
            unsafe { &mut *pp_v_tab_1 }, unsafe { &mut *pz_err_1 });
}

///* Clear all of the content from a cursor.
extern "C" fn spellfix1_reset_cursor(p_cur_1: &mut Spellfix1Cursor) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b37: loop {
            if !(i < (*p_cur_1).n_row) { break '__b37; }
            '__c37: loop {
                unsafe {
                    sqlite3_free(unsafe {
                                (*(*p_cur_1).a.offset(i as isize)).z_word
                            } as *mut ())
                };
                break '__c37;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_cur_1).n_row = 0;
    (*p_cur_1).i_row = 0;
    (*p_cur_1).n_search = 0;
    if !((*p_cur_1).p_full_scan).is_null() {
        unsafe { sqlite3_finalize((*p_cur_1).p_full_scan) };
        (*p_cur_1).p_full_scan = core::ptr::null_mut();
    }
}

///* Resize the cursor to hold up to N rows of content
extern "C" fn spellfix1_resize_cursor(p_cur_1: *mut Spellfix1Cursor, n_1: i32)
    -> () {
    let mut a_new: *mut Spellfix1Row = core::ptr::null_mut();
    { let _ = 0; };
    a_new =
        unsafe {
                sqlite3_realloc64(unsafe { (*p_cur_1).a } as *mut (),
                    core::mem::size_of::<Spellfix1Row>() as u64 * n_1 as u64)
            } as *mut Spellfix1Row;
    if a_new == core::ptr::null_mut() && n_1 > 0 {
        spellfix1_reset_cursor(unsafe { &mut *p_cur_1 });
        unsafe { sqlite3_free(unsafe { (*p_cur_1).a } as *mut ()) };
        unsafe { (*p_cur_1).n_alloc = 0 };
        unsafe { (*p_cur_1).a = core::ptr::null_mut() };
    } else {
        unsafe { (*p_cur_1).n_alloc = n_1 };
        unsafe { (*p_cur_1).a = a_new };
    }
}

///* Close a fuzzy-search cursor.
extern "C" fn spellfix1_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut Spellfix1Cursor = cur as *mut Spellfix1Cursor;
    spellfix1_reset_cursor(unsafe { &mut *p_cur });
    spellfix1_resize_cursor(p_cur, 0);
    unsafe { sqlite3_free(unsafe { (*p_cur).z_pattern } as *mut ()) };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}

///*
///* The plan number is a bitmask of the SPELLFIX_IDXNUM_* values defined
///* above.
///*
///* filter.argv[*] values contains $str, $langid, $top, $scope and $rowid
///* if specified and in that order.
extern "C" fn spellfix1_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i_plan: i32 = 0;
    let mut i_lang_term: i32 = -1;
    let mut i_top_term: i32 = -1;
    let mut i_scope_term: i32 = -1;
    let mut i_dist_term: i32 = -1;
    let mut i_rowid_term: i32 = -1;
    let mut i: i32 = 0;
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b38: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b38;
            }
            '__c38: loop {
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c38;
                }
                if i_plan & 1 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 0 &&
                        unsafe { (*p_constraint).op } as i32 == 64 {
                    i_plan |= 1;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = 1
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                }
                if i_plan & 2 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 3 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 2;
                    i_lang_term = i;
                }
                if i_plan & 4 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 7 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 4;
                    i_top_term = i;
                }
                if i_plan & 8 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 8 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 8;
                    i_scope_term = i;
                }
                if i_plan & (16 | 32) == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 2 &&
                        (unsafe { (*p_constraint).op } as i32 == 16 ||
                            unsafe { (*p_constraint).op } as i32 == 8) {
                    if unsafe { (*p_constraint).op } as i32 == 16 {
                        i_plan |= 16;
                    } else { i_plan |= 32; }
                    i_dist_term = i;
                }
                if i_plan & 64 == 0 &&
                            (unsafe { (*p_constraint).i_column } as i32) < 0 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 64;
                    i_rowid_term = i;
                }
                break '__c38;
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
    if i_plan & 1 != 0 {
        let mut idx: i32 = 2;
        unsafe { (*p_idx_info_1).idx_num = i_plan };
        if unsafe { (*p_idx_info_1).n_order_by } == 1 &&
                    unsafe {
                            (*unsafe {
                                        (*p_idx_info_1).a_order_by.offset(0 as isize)
                                    }).i_column
                        } == 4 &&
                unsafe {
                            (*unsafe {
                                        (*p_idx_info_1).a_order_by.offset(0 as isize)
                                    }).desc
                        } as i32 == 0 {
            unsafe { (*p_idx_info_1).order_by_consumed = 1 };
        }
        if i_plan & 2 != 0 {
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_lang_term as
                                        isize)
                            }).argv_index =
                    { let __p = &mut idx; let __t = *__p; *__p += 1; __t }
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_lang_term as
                                        isize)
                            }).omit = 1 as u8
            };
        }
        if i_plan & 4 != 0 {
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_top_term as
                                        isize)
                            }).argv_index =
                    { let __p = &mut idx; let __t = *__p; *__p += 1; __t }
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_top_term as
                                        isize)
                            }).omit = 1 as u8
            };
        }
        if i_plan & 8 != 0 {
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_scope_term as
                                        isize)
                            }).argv_index =
                    { let __p = &mut idx; let __t = *__p; *__p += 1; __t }
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_scope_term as
                                        isize)
                            }).omit = 1 as u8
            };
        }
        if i_plan & (16 | 32) != 0 {
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_dist_term as
                                        isize)
                            }).argv_index =
                    { let __p = &mut idx; let __t = *__p; *__p += 1; __t }
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_dist_term as
                                        isize)
                            }).omit = 1 as u8
            };
        }
        unsafe { (*p_idx_info_1).estimated_cost = 100000.0 };
    } else if i_plan & 64 != 0 {
        unsafe { (*p_idx_info_1).idx_num = 64 };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_rowid_term as
                                    isize)
                        }).argv_index = 1
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_rowid_term as
                                    isize)
                        }).omit = 1 as u8
        };
        unsafe { (*p_idx_info_1).estimated_cost = 5 as f64 };
    } else {
        unsafe { (*p_idx_info_1).idx_num = 0 };
        unsafe { (*p_idx_info_1).estimated_cost = 1e50 };
    }
    return 0;
}

///* Open a new fuzzy-search cursor.
extern "C" fn spellfix1_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p: *mut Spellfix1Vtab = p_v_tab_1 as *mut Spellfix1Vtab;
    let mut p_cur: *mut Spellfix1Cursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<Spellfix1Cursor>() as
                        Sqlite3Uint64)
            } as *mut Spellfix1Cursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<Spellfix1Cursor>() as u64)
    };
    unsafe { (*p_cur).p_v_tab = p };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}

///* Adjust a distance measurement by the words rank in order to show
///* preference to common words.
extern "C" fn spellfix1_score(i_distance_1: i32, mut i_rank_1: i32) -> i32 {
    let mut i_log2: i32 = 0;
    {
        i_log2 = 0;
        '__b39: loop {
            if !(i_rank_1 > 0) { break '__b39; }
            '__c39: loop { break '__c39; }
            {
                { let __p = &mut i_log2; let __t = *__p; *__p += 1; __t };
                i_rank_1 >>= 1
            };
        }
    }
    return i_distance_1 + 32 - i_log2;
}

///* Compare two spellfix1_row objects for sorting purposes in qsort() such
///* that they sort in order of increasing distance.
extern "C" fn spellfix1_row_compare(a_1: *const (), b_1: *const ()) -> i32 {
    let a: *const Spellfix1Row = a_1 as *const Spellfix1Row;
    let b: *const Spellfix1Row = b_1 as *const Spellfix1Row;
    return unsafe { (*a).i_score } - unsafe { (*b).i_score } as i32;
}

///* A structure used to pass information from spellfix1FilterForMatch()
///* into spellfix1RunQuery().
#[repr(C)]
#[derive(Copy, Clone)]
struct MatchQuery {
    p_cur: *mut Spellfix1Cursor,
    p_stmt: *mut Sqlite3Stmt,
    z_hash: [i8; 32],
    z_pattern: *const i8,
    n_pattern: i32,
    p_match_str3: *mut EditDist3FromString,
    p_config3: *mut EditDist3Config,
    p_lang: *const EditDist3Lang,
    i_lang: i32,
    i_scope: i32,
    i_max_dist: i32,
    rc: i32,
    n_run: i32,
    az_prior: [[i8; 32]; 1],
}

///* Run a query looking for the best matches against zPattern using
///* zHash as the character class seed hash.
#[allow(unused_doc_comments)]
extern "C" fn spellfix1_run_query(p: &mut MatchQuery, z_query_1: *const i8,
    n_query_1: i32) -> () {
    let mut z_k1: *const i8 = core::ptr::null();
    let mut z_word: *const i8 = core::ptr::null();
    let mut i_dist: i32 = 0;
    let mut i_rank: i32 = 0;
    let mut i_score: i32 = 0;
    let mut i_worst: i32 = 0;
    let mut idx: i32 = 0;
    let mut idx_worst: i32 = -1;
    let mut i: i32 = 0;
    let mut i_scope: i32 = (*p).i_scope;
    let p_cur: *mut Spellfix1Cursor = (*p).p_cur;
    let p_stmt: *mut Sqlite3Stmt = (*p).p_stmt;
    let mut z_hash1: [i8; 32] = [0; 32];
    let mut z_hash2: [i8; 32] = [0; 32];
    let mut z_class: *mut i8 = core::ptr::null_mut();
    let mut n_class: i32 = 0;
    let mut rc: i32 = 0;
    if unsafe { (*p_cur).a } == core::ptr::null_mut() || (*p).rc != 0 {
        return;
    }

    /// Prior memory allocation failure
    (z_class =
        phonetic_hash(z_query_1 as *mut u8 as *const u8, n_query_1) as
            *mut i8);
    if z_class == core::ptr::null_mut() { (*p).rc = 7; return; }
    n_class = unsafe { strlen(z_class as *const i8) } as i32;
    if n_class > 32 - 2 {
        n_class = 32 - 2;
        unsafe { *z_class.offset(n_class as isize) = 0 as i8 };
    }
    if n_class <= i_scope {
        if n_class > 2 { i_scope = n_class - 1; } else { i_scope = n_class; }
    }
    unsafe {
        memcpy(&raw mut z_hash1[0 as usize] as *mut i8 as *mut (),
            z_class as *const (), i_scope as u64)
    };
    unsafe { sqlite3_free(z_class as *mut ()) };
    z_hash1[i_scope as usize] = 0 as i8;
    unsafe {
        memcpy(&raw mut z_hash2[0 as usize] as *mut i8 as *mut (),
            &raw mut z_hash1[0 as usize] as *mut i8 as *const (),
            i_scope as u64)
    };
    z_hash2[i_scope as usize] = 'Z' as i32 as i8;
    z_hash2[(i_scope + 1) as usize] = 0 as i8;
    { let _ = 0; };
    unsafe {
        memcpy(&raw mut (*p).az_prior[{
                                        let __p = &mut (*p).n_run;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as usize][0 as usize] as *mut i8 as *mut (),
            &raw mut z_hash1[0 as usize] as *mut i8 as *const (),
            (i_scope + 1) as u64)
    };
    if unsafe {
                    sqlite3_bind_text(p_stmt, 1,
                        &raw mut z_hash1[0 as usize] as *mut i8 as *const i8, -1,
                        None)
                } == 7 ||
            unsafe {
                    sqlite3_bind_text(p_stmt, 2,
                        &raw mut z_hash2[0 as usize] as *mut i8 as *const i8, -1,
                        None)
                } == 7 {
        (*p).rc = 7;
        return;
    }
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        let mut i_matchlen: i32 = -1;
        i_rank = unsafe { sqlite3_column_int(p_stmt, 2) };
        if !((*p).p_match_str3).is_null() {
            let n_word: i32 = unsafe { sqlite3_column_bytes(p_stmt, 1) };
            z_word = unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
            i_dist =
                edit_dist3_core(unsafe { &*(*p).p_match_str3 }, z_word,
                    n_word, unsafe { &*(*p).p_lang }, &mut i_matchlen);
        } else {
            z_k1 = unsafe { sqlite3_column_text(p_stmt, 3) } as *const i8;
            if z_k1 == core::ptr::null() { continue; }
            i_dist = editdist1((*p).z_pattern, z_k1, core::ptr::null_mut());
        }
        if i_dist < 0 { (*p).rc = if i_dist == -4 { 18 } else { 7 }; break; }
        {
            let __p = unsafe { &mut (*p_cur).n_search };
            let __t = *__p;
            *__p += 1;
            __t
        };

        /// If there is a "distance < $dist" or "distance <= $dist" constraint,
        ///* check if this row meets it. If not, jump back up to the top of the
        ///* loop to process the next row. Otherwise, if the row does match the
        ///* distance constraint, check if the pCur->a[] array is already full.
        ///* If it is and no explicit "top = ?" constraint was present in the
        ///* query, grow the array to ensure there is room for the new entry.
        { let _ = 0; };
        if (*p).i_max_dist >= 0 {
            if i_dist > (*p).i_max_dist { continue; }
            if unsafe { (*p_cur).n_row } >= unsafe { (*p_cur).n_alloc } &&
                    unsafe { (*p_cur).idx_num } & 4 == 0 {
                spellfix1_resize_cursor(p_cur,
                    unsafe { (*p_cur).n_alloc } * 2 + 10);
                if unsafe { (*p_cur).a } == core::ptr::null_mut() { break; }
            }
        }
        i_score = spellfix1_score(i_dist, i_rank);
        if unsafe { (*p_cur).n_row } < unsafe { (*p_cur).n_alloc } {
            idx = unsafe { (*p_cur).n_row };
        } else if i_score < i_worst {
            idx = idx_worst;
            unsafe {
                sqlite3_free(unsafe {
                            (*unsafe { (*p_cur).a.offset(idx as isize) }).z_word
                        } as *mut ())
            };
        } else { continue; }
        unsafe {
            (*unsafe { (*p_cur).a.offset(idx as isize) }).z_word =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_column_text(p_stmt, 1) })
                }
        };
        if unsafe { (*unsafe { (*p_cur).a.offset(idx as isize) }).z_word } ==
                core::ptr::null_mut() {
            (*p).rc = 7;
            break;
        }
        unsafe {
            (*unsafe { (*p_cur).a.offset(idx as isize) }).i_rowid =
                unsafe { sqlite3_column_int64(p_stmt, 0) }
        };
        unsafe {
            (*unsafe { (*p_cur).a.offset(idx as isize) }).i_rank = i_rank
        };
        unsafe {
            (*unsafe { (*p_cur).a.offset(idx as isize) }).i_distance = i_dist
        };
        unsafe {
            (*unsafe { (*p_cur).a.offset(idx as isize) }).i_score = i_score
        };
        unsafe {
            (*unsafe { (*p_cur).a.offset(idx as isize) }).i_matchlen =
                i_matchlen
        };
        unsafe {
            memcpy(unsafe {
                            &raw mut (*unsafe {
                                                (*p_cur).a.offset(idx as isize)
                                            }).z_hash[0 as usize]
                        } as *mut i8 as *mut (),
                &raw mut z_hash1[0 as usize] as *mut i8 as *const (),
                (i_scope + 1) as u64)
        };
        if unsafe { (*p_cur).n_row } < unsafe { (*p_cur).n_alloc } {
            {
                let __p = unsafe { &mut (*p_cur).n_row };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        if unsafe { (*p_cur).n_row } == unsafe { (*p_cur).n_alloc } {
            i_worst =
                unsafe {
                    (*unsafe { (*p_cur).a.offset(0 as isize) }).i_score
                };
            idx_worst = 0;
            {
                i = 1;
                '__b41: loop {
                    if !(i < unsafe { (*p_cur).n_row }) { break '__b41; }
                    '__c41: loop {
                        i_score =
                            unsafe {
                                (*unsafe { (*p_cur).a.offset(i as isize) }).i_score
                            };
                        if i_worst < i_score { i_worst = i_score; idx_worst = i; }
                        break '__c41;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
    rc = unsafe { sqlite3_reset(p_stmt) };
    if rc != 0 { (*p).rc = rc; }
}

///* This version of the xFilter method work if the MATCH term is present
///* and we are doing a scan.
#[allow(unused_doc_comments)]
extern "C" fn spellfix1_filter_for_match(p_cur_1: *mut Spellfix1Cursor,
    argc: i32, argv: *const *mut Sqlite3Value) -> i32 {
    /// RHS of the MATCH operator
    let mut p_match_str3: *mut EditDist3FromString = core::ptr::null_mut();
    /// zMatchThis as an editdist string
    /// Transliteration of zMatchThis
    /// Length of zPattern
    /// Max number of rows of output
    /// Use this many characters of zClass
    /// Language code
    /// SQL of shadow table query
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    /// Shadow table query
    /// Result code
    /// Next available filter parameter
    /// The virtual table that owns pCur
    let mut x: MatchQuery = unsafe { core::mem::zeroed() };
    '__b42: loop {
        '__c42: loop {
            let idx_num: i32 = unsafe { (*p_cur_1).idx_num };
            let mut z_match_this: *const u8 = core::ptr::null();
            /// RHS of the MATCH operator
            /// zMatchThis as an editdist string
            let mut z_pattern: *mut i8 = core::ptr::null_mut();
            /// Transliteration of zMatchThis
            let mut n_pattern: i32 = 0;
            /// Length of zPattern
            let mut i_limit: i32 = 20;
            /// Max number of rows of output
            let i_scope: i32 = 3;
            /// Use this many characters of zClass
            let mut i_lang: i32 = 0;
            /// Language code
            let mut z_sql: *mut i8 = core::ptr::null_mut();
            /// SQL of shadow table query
            /// Shadow table query
            let mut rc: i32 = 0;
            /// Result code
            let mut idx: i32 = 1;
            /// Next available filter parameter
            let p: *mut Spellfix1Vtab = unsafe { (*p_cur_1).p_v_tab };
            if unsafe { (*p).z_cost_table } != core::ptr::null_mut() &&
                    unsafe { (*p).p_config3 } == core::ptr::null_mut() {
                unsafe {
                    (*p).p_config3 =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<EditDist3Config>() as
                                        Sqlite3Uint64)
                            } as *mut EditDist3Config
                };
                if unsafe { (*p).p_config3 } == core::ptr::null_mut() {
                    return 7;
                }
                unsafe {
                    memset(unsafe { (*p).p_config3 } as *mut (), 0,
                        core::mem::size_of::<EditDist3Config>() as u64)
                };
                rc =
                    edit_dist3_config_load(unsafe { (*p).p_config3 },
                        unsafe { (*p).db },
                        unsafe { (*p).z_cost_table } as *const i8);
                if rc != 0 { return rc; }
            }
            unsafe {
                memset(&raw mut x as *mut (), 0,
                    core::mem::size_of::<MatchQuery>() as u64)
            };
            x.i_scope = 3;

            /// Default scope if none specified by "WHERE scope=N"
            (x.i_max_dist = -1);
            if idx_num & 2 != 0 {
                i_lang =
                    unsafe {
                        sqlite3_value_int(unsafe {
                                *argv.offset({
                                                let __p = &mut idx;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            })
                    };
            }
            if idx_num & 4 != 0 {
                i_limit =
                    unsafe {
                        sqlite3_value_int(unsafe {
                                *argv.offset({
                                                let __p = &mut idx;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            })
                    };
                if i_limit < 1 { i_limit = 1; }
            }
            if idx_num & 8 != 0 {
                x.i_scope =
                    unsafe {
                        sqlite3_value_int(unsafe {
                                *argv.offset({
                                                let __p = &mut idx;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            })
                    };
                if x.i_scope < 1 { x.i_scope = 1; }
                if x.i_scope > 32 - 2 { x.i_scope = 32 - 2; }
            }
            if idx_num & (16 | 32) != 0 {
                x.i_max_dist =
                    unsafe {
                        sqlite3_value_int(unsafe {
                                *argv.offset({
                                                let __p = &mut idx;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            })
                    };
                if idx_num & 16 != 0 {
                    {
                        let __p = &mut x.i_max_dist;
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                }
                if x.i_max_dist < 0 { x.i_max_dist = 0; }
            }
            spellfix1_reset_cursor(unsafe { &mut *p_cur_1 });
            spellfix1_resize_cursor(p_cur_1, i_limit);
            z_match_this =
                unsafe {
                    sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                };
            if z_match_this == core::ptr::null() { return 0; }
            if !(unsafe { (*p).p_config3 }).is_null() {
                x.p_lang =
                    edit_dist3_find_lang(unsafe { &*unsafe { (*p).p_config3 } },
                        i_lang);
                p_match_str3 =
                    edit_dist3_from_string_new(unsafe { &*x.p_lang },
                        z_match_this as *const i8, -1);
                if p_match_str3 == core::ptr::null_mut() {
                    x.rc = 7;
                    break '__b42;
                }
            } else { x.p_lang = core::ptr::null(); }
            z_pattern =
                transliterate(z_match_this,
                        unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                        }) as *mut i8;
            unsafe {
                sqlite3_free(unsafe { (*p_cur_1).z_pattern } as *mut ())
            };
            unsafe { (*p_cur_1).z_pattern = z_pattern };
            if z_pattern == core::ptr::null_mut() { x.rc = 7; break '__b42; }
            n_pattern = unsafe { strlen(z_pattern as *const i8) } as i32;
            if n_pattern > 0 &&
                    unsafe { *z_pattern.offset((n_pattern - 1) as isize) } as
                            i32 == '*' as i32 {
                { let __p = &mut n_pattern; let __t = *__p; *__p -= 1; __t };
            }
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"SELECT id, word, rank, coalesce(k1,word)  FROM \"%w\".\"%w_vocab\" WHERE langid=%d AND k2>=?1 AND k2<?2".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p).z_db_name },
                        unsafe { (*p).z_table_name }, i_lang)
                };
            if z_sql == core::ptr::null_mut() {
                x.rc = 7;
                p_stmt = core::ptr::null_mut();
                break '__b42;
            }
            rc =
                unsafe {
                    sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8,
                        -1, &mut p_stmt, core::ptr::null_mut())
                };
            unsafe { sqlite3_free(z_sql as *mut ()) };
            unsafe { (*p_cur_1).i_lang = i_lang };
            x.p_cur = p_cur_1;
            x.p_stmt = p_stmt;
            x.z_pattern = z_pattern as *const i8;
            x.n_pattern = n_pattern;
            x.p_match_str3 = p_match_str3;
            x.i_lang = i_lang;
            x.rc = rc;
            x.p_config3 = unsafe { (*p).p_config3 };
            if x.rc == 0 {
                spellfix1_run_query(&mut x, z_pattern as *const i8,
                    n_pattern);
            }
            if !(unsafe { (*p_cur_1).a }).is_null() {
                unsafe {
                    qsort(unsafe { (*p_cur_1).a } as *mut (),
                        unsafe { (*p_cur_1).n_row } as u64,
                        core::mem::size_of::<Spellfix1Row>() as u64,
                        spellfix1_row_compare)
                };
                unsafe { (*p_cur_1).i_top = i_limit };
                unsafe { (*p_cur_1).i_scope = i_scope };
            } else { x.rc = 7; }
            break '__c42;
        }
        if !(false) { break '__b42; }
    }

    /// RHS of the MATCH operator
    /// zMatchThis as an editdist string
    /// Transliteration of zMatchThis
    /// Length of zPattern
    /// Max number of rows of output
    /// Use this many characters of zClass
    /// Language code
    /// SQL of shadow table query
    /// Shadow table query
    /// Result code
    /// Next available filter parameter
    /// The virtual table that owns pCur
    /// For passing info to RunQuery()
    /// Load the cost table if we have not already done so
    /// Default scope if none specified by "WHERE scope=N"
    /// Maximum allowed edit distance
    unsafe { sqlite3_finalize(p_stmt) };
    edit_dist3_from_string_delete(p_match_str3);
    return x.rc;
}

///* This version of xFilter handles a full-table scan case
extern "C" fn spellfix1_filter_for_full_scan(p_cur_1: *mut Spellfix1Cursor,
    argc: i32, argv: *const *mut Sqlite3Value) -> i32 {
    let mut rc: i32 = 0;
    let idx_num: i32 = unsafe { (*p_cur_1).idx_num };
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let p_v_tab: *const Spellfix1Vtab =
        unsafe { (*p_cur_1).p_v_tab } as *const Spellfix1Vtab;
    spellfix1_reset_cursor(unsafe { &mut *p_cur_1 });
    { let _ = 0; };
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT word, rank, NULL, langid, id FROM \"%w\".\"%w_vocab\"%s".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p_v_tab).z_db_name },
                unsafe { (*p_v_tab).z_table_name },
                if idx_num & 64 != 0 {
                    c" WHERE rowid=?".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 })
        };
    if z_sql == core::ptr::null_mut() { return 7; }
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p_v_tab).db }, z_sql as *const i8,
                -1, unsafe { &mut (*p_cur_1).p_full_scan },
                core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc == 0 && idx_num & 64 != 0 {
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_bind_value(unsafe { (*p_cur_1).p_full_scan }, 1,
                    unsafe { *argv.offset(0 as isize) } as *const Sqlite3Value)
            };
    }
    unsafe {
        (*p_cur_1).n_row =
            { unsafe { (*p_cur_1).i_row = 0 }; unsafe { (*p_cur_1).i_row } }
    };
    if rc == 0 {
        rc = unsafe { sqlite3_step(unsafe { (*p_cur_1).p_full_scan }) };
        if rc == 100 { unsafe { (*p_cur_1).i_row = -1 }; rc = 0; }
        if rc == 101 { rc = 0; }
    } else { unsafe { (*p_cur_1).i_row = 0 }; }
    return rc;
}

///* Called to "rewind" a cursor back to the beginning so that
///* it starts its output over again.  Always called at least once
///* prior to any spellfix1Column, spellfix1Rowid, or spellfix1Eof call.
extern "C" fn spellfix1_filter(cur: *mut Sqlite3VtabCursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut Spellfix1Cursor = cur as *mut Spellfix1Cursor;
    let mut rc: i32 = 0;
    unsafe { (*p_cur).idx_num = idx_num_1 };
    if idx_num_1 & 1 != 0 {
        rc =
            spellfix1_filter_for_match(p_cur, argc,
                argv as *const *mut Sqlite3Value);
    } else {
        rc =
            spellfix1_filter_for_full_scan(p_cur, argc,
                argv as *const *mut Sqlite3Value);
    }
    return rc;
}

///* Advance a cursor to its next row of output
extern "C" fn spellfix1_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut Spellfix1Cursor = cur as *mut Spellfix1Cursor;
    let mut rc: i32 = 0;
    if unsafe { (*p_cur).i_row } < unsafe { (*p_cur).n_row } {
        if !(unsafe { (*p_cur).p_full_scan }).is_null() {
            rc = unsafe { sqlite3_step(unsafe { (*p_cur).p_full_scan }) };
            if rc != 100 {
                unsafe { (*p_cur).i_row = unsafe { (*p_cur).n_row } };
            }
            if rc == 100 || rc == 101 { rc = 0; }
        } else {
            {
                let __p = unsafe { &mut (*p_cur).i_row };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return rc;
}

///* Return TRUE if we are at the end-of-file
extern "C" fn spellfix1_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const Spellfix1Cursor =
        cur as *mut Spellfix1Cursor as *const Spellfix1Cursor;
    return (unsafe { (*p_cur).i_row } >= unsafe { (*p_cur).n_row }) as i32;
}

///* Return columns from the current row.
extern "C" fn spellfix1_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const Spellfix1Cursor =
        cur as *mut Spellfix1Cursor as *const Spellfix1Cursor;
    if !(unsafe { (*p_cur).p_full_scan }).is_null() {
        if i <= 3 {
            unsafe {
                sqlite3_result_value(ctx,
                    unsafe {
                        sqlite3_column_value(unsafe { (*p_cur).p_full_scan }, i)
                    })
            };
        } else { unsafe { sqlite3_result_null(ctx) }; }
        return 0;
    }
    '__s43:
        {
        match i {
            0 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                    (*unsafe {
                                                (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                            }).z_word
                                } as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_rank
                            })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_distance
                            })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_score
                            })
                    };
                    break '__s43;
                }
                {
                    let mut i_matchlen: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                    }).i_matchlen
                        };
                    if i_matchlen < 0 {
                        let n_pattern: i32 =
                            unsafe {
                                    strlen(unsafe { (*p_cur).z_pattern } as *const i8)
                                } as i32;
                        let z_word: *mut i8 =
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).z_word
                            };
                        let n_word: i32 =
                            unsafe { strlen(z_word as *const i8) } as i32;
                        if n_pattern > 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*p_cur).z_pattern.offset((n_pattern - 1) as isize)
                                                }
                                        } as i32 == '*' as i32 {
                            let mut z_translit: *mut i8 = core::ptr::null_mut();
                            let mut res: i32 = 0;
                            z_translit =
                                transliterate(z_word as *mut u8 as *const u8, n_word) as
                                    *mut i8;
                            if (z_translit).is_null() as i32 != 0 { return 7; }
                            res =
                                editdist1(unsafe { (*p_cur).z_pattern } as *const i8,
                                    z_translit as *const i8, &mut i_matchlen);
                            unsafe { sqlite3_free(z_translit as *mut ()) };
                            if res < 0 { return if res == -4 { 18 } else { 7 }; }
                            i_matchlen =
                                translen_to_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    }, i_matchlen);
                        } else {
                            i_matchlen =
                                utf8_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    });
                        }
                    }
                    unsafe { sqlite3_result_int(ctx, i_matchlen) };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_rank
                            })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_distance
                            })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_score
                            })
                    };
                    break '__s43;
                }
                {
                    let mut i_matchlen: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                    }).i_matchlen
                        };
                    if i_matchlen < 0 {
                        let n_pattern: i32 =
                            unsafe {
                                    strlen(unsafe { (*p_cur).z_pattern } as *const i8)
                                } as i32;
                        let z_word: *mut i8 =
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).z_word
                            };
                        let n_word: i32 =
                            unsafe { strlen(z_word as *const i8) } as i32;
                        if n_pattern > 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*p_cur).z_pattern.offset((n_pattern - 1) as isize)
                                                }
                                        } as i32 == '*' as i32 {
                            let mut z_translit: *mut i8 = core::ptr::null_mut();
                            let mut res: i32 = 0;
                            z_translit =
                                transliterate(z_word as *mut u8 as *const u8, n_word) as
                                    *mut i8;
                            if (z_translit).is_null() as i32 != 0 { return 7; }
                            res =
                                editdist1(unsafe { (*p_cur).z_pattern } as *const i8,
                                    z_translit as *const i8, &mut i_matchlen);
                            unsafe { sqlite3_free(z_translit as *mut ()) };
                            if res < 0 { return if res == -4 { 18 } else { 7 }; }
                            i_matchlen =
                                translen_to_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    }, i_matchlen);
                        } else {
                            i_matchlen =
                                utf8_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    });
                        }
                    }
                    unsafe { sqlite3_result_int(ctx, i_matchlen) };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            2 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_distance
                            })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_score
                            })
                    };
                    break '__s43;
                }
                {
                    let mut i_matchlen: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                    }).i_matchlen
                        };
                    if i_matchlen < 0 {
                        let n_pattern: i32 =
                            unsafe {
                                    strlen(unsafe { (*p_cur).z_pattern } as *const i8)
                                } as i32;
                        let z_word: *mut i8 =
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).z_word
                            };
                        let n_word: i32 =
                            unsafe { strlen(z_word as *const i8) } as i32;
                        if n_pattern > 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*p_cur).z_pattern.offset((n_pattern - 1) as isize)
                                                }
                                        } as i32 == '*' as i32 {
                            let mut z_translit: *mut i8 = core::ptr::null_mut();
                            let mut res: i32 = 0;
                            z_translit =
                                transliterate(z_word as *mut u8 as *const u8, n_word) as
                                    *mut i8;
                            if (z_translit).is_null() as i32 != 0 { return 7; }
                            res =
                                editdist1(unsafe { (*p_cur).z_pattern } as *const i8,
                                    z_translit as *const i8, &mut i_matchlen);
                            unsafe { sqlite3_free(z_translit as *mut ()) };
                            if res < 0 { return if res == -4 { 18 } else { 7 }; }
                            i_matchlen =
                                translen_to_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    }, i_matchlen);
                        } else {
                            i_matchlen =
                                utf8_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    });
                        }
                    }
                    unsafe { sqlite3_result_int(ctx, i_matchlen) };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            3 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_score
                            })
                    };
                    break '__s43;
                }
                {
                    let mut i_matchlen: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                    }).i_matchlen
                        };
                    if i_matchlen < 0 {
                        let n_pattern: i32 =
                            unsafe {
                                    strlen(unsafe { (*p_cur).z_pattern } as *const i8)
                                } as i32;
                        let z_word: *mut i8 =
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).z_word
                            };
                        let n_word: i32 =
                            unsafe { strlen(z_word as *const i8) } as i32;
                        if n_pattern > 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*p_cur).z_pattern.offset((n_pattern - 1) as isize)
                                                }
                                        } as i32 == '*' as i32 {
                            let mut z_translit: *mut i8 = core::ptr::null_mut();
                            let mut res: i32 = 0;
                            z_translit =
                                transliterate(z_word as *mut u8 as *const u8, n_word) as
                                    *mut i8;
                            if (z_translit).is_null() as i32 != 0 { return 7; }
                            res =
                                editdist1(unsafe { (*p_cur).z_pattern } as *const i8,
                                    z_translit as *const i8, &mut i_matchlen);
                            unsafe { sqlite3_free(z_translit as *mut ()) };
                            if res < 0 { return if res == -4 { 18 } else { 7 }; }
                            i_matchlen =
                                translen_to_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    }, i_matchlen);
                        } else {
                            i_matchlen =
                                utf8_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    });
                        }
                    }
                    unsafe { sqlite3_result_int(ctx, i_matchlen) };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            4 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).i_score
                            })
                    };
                    break '__s43;
                }
                {
                    let mut i_matchlen: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                    }).i_matchlen
                        };
                    if i_matchlen < 0 {
                        let n_pattern: i32 =
                            unsafe {
                                    strlen(unsafe { (*p_cur).z_pattern } as *const i8)
                                } as i32;
                        let z_word: *mut i8 =
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).z_word
                            };
                        let n_word: i32 =
                            unsafe { strlen(z_word as *const i8) } as i32;
                        if n_pattern > 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*p_cur).z_pattern.offset((n_pattern - 1) as isize)
                                                }
                                        } as i32 == '*' as i32 {
                            let mut z_translit: *mut i8 = core::ptr::null_mut();
                            let mut res: i32 = 0;
                            z_translit =
                                transliterate(z_word as *mut u8 as *const u8, n_word) as
                                    *mut i8;
                            if (z_translit).is_null() as i32 != 0 { return 7; }
                            res =
                                editdist1(unsafe { (*p_cur).z_pattern } as *const i8,
                                    z_translit as *const i8, &mut i_matchlen);
                            unsafe { sqlite3_free(z_translit as *mut ()) };
                            if res < 0 { return if res == -4 { 18 } else { 7 }; }
                            i_matchlen =
                                translen_to_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    }, i_matchlen);
                        } else {
                            i_matchlen =
                                utf8_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    });
                        }
                    }
                    unsafe { sqlite3_result_int(ctx, i_matchlen) };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            5 => {
                {
                    let mut i_matchlen: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                    }).i_matchlen
                        };
                    if i_matchlen < 0 {
                        let n_pattern: i32 =
                            unsafe {
                                    strlen(unsafe { (*p_cur).z_pattern } as *const i8)
                                } as i32;
                        let z_word: *mut i8 =
                            unsafe {
                                (*unsafe {
                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                        }).z_word
                            };
                        let n_word: i32 =
                            unsafe { strlen(z_word as *const i8) } as i32;
                        if n_pattern > 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*p_cur).z_pattern.offset((n_pattern - 1) as isize)
                                                }
                                        } as i32 == '*' as i32 {
                            let mut z_translit: *mut i8 = core::ptr::null_mut();
                            let mut res: i32 = 0;
                            z_translit =
                                transliterate(z_word as *mut u8 as *const u8, n_word) as
                                    *mut i8;
                            if (z_translit).is_null() as i32 != 0 { return 7; }
                            res =
                                editdist1(unsafe { (*p_cur).z_pattern } as *const i8,
                                    z_translit as *const i8, &mut i_matchlen);
                            unsafe { sqlite3_free(z_translit as *mut ()) };
                            if res < 0 { return if res == -4 { 18 } else { 7 }; }
                            i_matchlen =
                                translen_to_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    }, i_matchlen);
                        } else {
                            i_matchlen =
                                utf8_charlen(unsafe {
                                        let __p = z_word as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n_word as usize) }
                                    });
                        }
                    }
                    unsafe { sqlite3_result_int(ctx, i_matchlen) };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            6 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                        &raw const (*unsafe {
                                                            (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                                                        }).z_hash[0 as usize]
                                    } as *mut i8 as *const i8, -1, None)
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            7 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_top })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            8 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_scope })
                    };
                    break '__s43;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            9 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_search })
                    };
                    break '__s43;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s43; }
            }
            _ => { { unsafe { sqlite3_result_null(ctx) }; break '__s43; } }
        }
    }
    return 0;
}

///* The rowid.
extern "C" fn spellfix1_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const Spellfix1Cursor =
        cur as *mut Spellfix1Cursor as *const Spellfix1Cursor;
    if !(unsafe { (*p_cur).p_full_scan }).is_null() {
        unsafe {
            *p_rowid_1 =
                unsafe {
                    sqlite3_column_int64(unsafe { (*p_cur).p_full_scan }, 4)
                }
        };
    } else {
        unsafe {
            *p_rowid_1 =
                unsafe {
                    (*unsafe {
                                (*p_cur).a.offset(unsafe { (*p_cur).i_row } as isize)
                            }).i_rowid
                }
        };
    }
    return 0;
}

///* This function is called by the xUpdate() method. It returns a string
///* containing the conflict mode that xUpdate() should use for the current
///* operation. One of: "ROLLBACK", "IGNORE", "ABORT" or "REPLACE".
#[allow(unused_doc_comments)]
extern "C" fn spellfix1_get_conflict(db: *mut Sqlite3) -> *const i8 {
    unsafe {
        /// Note: Instead of "FAIL" - "ABORT".
        let e_conflict: i32 = unsafe { sqlite3_vtab_on_conflict(db) };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        return az_conflict[(e_conflict - 1) as usize];
    }
}

///* The xUpdate() method.
#[allow(unused_doc_comments)]
extern "C" fn spellfix1_update(p_v_tab_1: *mut Sqlite3Vtab, argc: i32,
    argv: *mut *mut Sqlite3Value, p_rowid_1: *mut SqliteInt64) -> i32 {
    let mut rc: i32 = 0;
    let mut rowid: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut new_rowid: Sqlite3Int64 = 0 as Sqlite3Int64;
    let p: *mut Spellfix1Vtab = p_v_tab_1 as *mut Spellfix1Vtab;
    let db: *mut Sqlite3 = unsafe { (*p).db };
    if argc == 1 {

        /// A delete operation on the rowid given by argv[0]
        (rowid =
            {
                unsafe {
                    *p_rowid_1 =
                        unsafe {
                            sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                        }
                };
                unsafe { *p_rowid_1 }
            });
        unsafe {
            spellfix1_db_exec(&mut rc, db,
                c"DELETE FROM \"%w\".\"%w_vocab\"  WHERE id=%lld".as_ptr() as
                        *mut i8 as *const i8, unsafe { (*p).z_db_name },
                unsafe { (*p).z_table_name }, rowid)
        };
    } else {
        let z_word: *const u8 =
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset((0 + 2) as isize) })
            };
        let n_word: i32 =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset((0 + 2) as isize) })
            };
        let i_lang: i32 =
            unsafe {
                sqlite3_value_int(unsafe { *argv.offset((3 + 2) as isize) })
            };
        let mut i_rank: i32 =
            unsafe {
                sqlite3_value_int(unsafe { *argv.offset((1 + 2) as isize) })
            };
        let z_soundslike: *const u8 =
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset((10 + 2) as isize) })
            };
        let n_soundslike: i32 =
            unsafe {
                sqlite3_value_bytes(unsafe {
                        *argv.offset((10 + 2) as isize)
                    })
            };
        let mut z_k1: *mut i8 = core::ptr::null_mut();
        let mut z_k2: *mut i8 = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut c: i8 = 0 as i8;
        let z_conflict: *const i8 = spellfix1_get_conflict(db);
        if z_word == core::ptr::null() {
            /// Inserts of the form:  INSERT INTO table(command) VALUES('xyzzy');
            ///* cause zWord to be NULL, so we look at the "command" column to see
            ///* what special actions to take
            let z_cmd: *const i8 =
                unsafe {
                        sqlite3_value_text(unsafe {
                                *argv.offset((11 + 2) as isize)
                            })
                    } as *const i8;
            if z_cmd == core::ptr::null() {
                unsafe {
                    (*p_v_tab_1).z_err_msg =
                        unsafe {
                            sqlite3_mprintf(c"NOT NULL constraint failed: %s.word".as_ptr()
                                        as *mut i8 as *const i8, unsafe { (*p).z_table_name })
                        }
                };
                return 19 | 5 << 8;
            }
            if unsafe {
                        strcmp(z_cmd, c"reset".as_ptr() as *mut i8 as *const i8)
                    } == 0 {

                /// Reset the  edit cost table (if there is one).
                edit_dist3_config_delete(unsafe { (*p).p_config3 } as
                        *mut ());
                unsafe { (*p).p_config3 = core::ptr::null_mut() };
                return 0;
            }
            if unsafe {
                        strncmp(z_cmd,
                            c"edit_cost_table=".as_ptr() as *mut i8 as *const i8,
                            16 as u64)
                    } == 0 {
                edit_dist3_config_delete(unsafe { (*p).p_config3 } as
                        *mut ());
                unsafe { (*p).p_config3 = core::ptr::null_mut() };
                unsafe {
                    sqlite3_free(unsafe { (*p).z_cost_table } as *mut ())
                };
                unsafe {
                    (*p).z_cost_table =
                        spellfix1_dequote(unsafe { z_cmd.offset(16 as isize) })
                };
                if unsafe { (*p).z_cost_table } == core::ptr::null_mut() {
                    return 7;
                }
                if unsafe { *unsafe { (*p).z_cost_table.offset(0 as isize) } }
                                as i32 == 0 ||
                        unsafe {
                                sqlite3_stricmp(unsafe { (*p).z_cost_table } as *const i8,
                                    c"null".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                    unsafe {
                        sqlite3_free(unsafe { (*p).z_cost_table } as *mut ())
                    };
                    unsafe { (*p).z_cost_table = core::ptr::null_mut() };
                }
                return 0;
            }
            unsafe {
                (*p_v_tab_1).z_err_msg =
                    unsafe {
                        sqlite3_mprintf(c"unknown value for %s.command: \"%w\"".as_ptr()
                                    as *mut i8 as *const i8, unsafe { (*p).z_table_name },
                            z_cmd)
                    }
            };
            return 1;
        }
        if i_rank < 1 { i_rank = 1; }
        if !(z_soundslike).is_null() {
            z_k1 = transliterate(z_soundslike, n_soundslike) as *mut i8;
        } else { z_k1 = transliterate(z_word, n_word) as *mut i8; }
        if z_k1 == core::ptr::null_mut() { return 7; }
        {
            i = 0;
            '__b44: loop {
                if !({ c = unsafe { *z_k1.offset(i as isize) }; c } as i32 !=
                                0) {
                    break '__b44;
                }
                '__c44: loop {
                    if c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32 {
                        unsafe {
                            *z_k1.offset(i as isize) += ('a' as i32 - 'A' as i32) as i8
                        };
                    }
                    break '__c44;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        z_k2 = phonetic_hash(z_k1 as *const u8, i) as *mut i8;
        if z_k2 == core::ptr::null_mut() {
            unsafe { sqlite3_free(z_k1 as *mut ()) };
            return 7;
        }
        if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) }
                == 5 {
            if unsafe {
                        sqlite3_value_type(unsafe { *argv.offset(1 as isize) })
                    } == 5 {
                unsafe {
                    spellfix1_db_exec(&mut rc, db,
                        c"INSERT INTO \"%w\".\"%w_vocab\"(rank,langid,word,k1,k2) VALUES(%d,%d,%Q,nullif(%Q,%Q),%Q)".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p).z_db_name },
                        unsafe { (*p).z_table_name }, i_rank, i_lang, z_word, z_k1,
                        z_word, z_k2)
                };
            } else {
                new_rowid =
                    unsafe {
                        sqlite3_value_int64(unsafe { *argv.offset(1 as isize) })
                    };
                unsafe {
                    spellfix1_db_exec(&mut rc, db,
                        c"INSERT OR %s INTO \"%w\".\"%w_vocab\"(id,rank,langid,word,k1,k2) VALUES(%lld,%d,%d,%Q,nullif(%Q,%Q),%Q)".as_ptr()
                                as *mut i8 as *const i8, z_conflict,
                        unsafe { (*p).z_db_name }, unsafe { (*p).z_table_name },
                        new_rowid, i_rank, i_lang, z_word, z_k1, z_word, z_k2)
                };
            }
            unsafe { *p_rowid_1 = unsafe { sqlite3_last_insert_rowid(db) } };
        } else {
            rowid =
                unsafe {
                    sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                };
            new_rowid =
                {
                    unsafe {
                        *p_rowid_1 =
                            unsafe {
                                sqlite3_value_int64(unsafe { *argv.offset(1 as isize) })
                            }
                    };
                    unsafe { *p_rowid_1 }
                };
            unsafe {
                spellfix1_db_exec(&mut rc, db,
                    c"UPDATE OR %s \"%w\".\"%w_vocab\" SET id=%lld, rank=%d, langid=%d, word=%Q, k1=nullif(%Q,%Q), k2=%Q WHERE id=%lld".as_ptr()
                            as *mut i8 as *const i8, z_conflict,
                    unsafe { (*p).z_db_name }, unsafe { (*p).z_table_name },
                    new_rowid, i_rank, i_lang, z_word, z_k1, z_word, z_k2,
                    rowid)
            };
        }
        unsafe { sqlite3_free(z_k1 as *mut ()) };
        unsafe { sqlite3_free(z_k2 as *mut ()) };
    }
    return rc;
}

///* Rename the spellfix1 table.
extern "C" fn spellfix1_rename(p_v_tab_1: *mut Sqlite3Vtab,
    z_new_1: *const i8) -> i32 {
    let p: *mut Spellfix1Vtab = p_v_tab_1 as *mut Spellfix1Vtab;
    let db: *mut Sqlite3 = unsafe { (*p).db };
    let mut rc: i32 = 0;
    let z_new_name: *mut i8 =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_new_1)
        };
    if z_new_name == core::ptr::null_mut() { return 7; }
    unsafe {
        spellfix1_db_exec(&mut rc, db,
            c"ALTER TABLE \"%w\".\"%w_vocab\" RENAME TO \"%w_vocab\"".as_ptr()
                    as *mut i8 as *const i8, unsafe { (*p).z_db_name },
            unsafe { (*p).z_table_name }, z_new_name)
    };
    if rc == 0 {
        unsafe { sqlite3_free(unsafe { (*p).z_table_name } as *mut ()) };
        unsafe { (*p).z_table_name = z_new_name };
    } else { unsafe { sqlite3_free(z_new_name as *mut ()) }; }
    return rc;
}

///* A virtual table module that provides fuzzy search.
static mut spellfix1_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: Some(spellfix1_create),
        x_connect: Some(spellfix1_connect),
        x_best_index: Some(spellfix1_best_index),
        x_disconnect: Some(spellfix1_disconnect),
        x_destroy: Some(spellfix1_destroy),
        x_open: Some(spellfix1_open),
        x_close: Some(spellfix1_close),
        x_filter: Some(spellfix1_filter),
        x_next: Some(spellfix1_next),
        x_eof: Some(spellfix1_eof),
        x_column: Some(spellfix1_column),
        x_rowid: Some(spellfix1_rowid),
        x_update: Some(spellfix1_update),
        x_begin: None,
        x_sync: None,
        x_commit: None,
        x_rollback: None,
        x_find_function: None,
        x_rename: Some(spellfix1_rename),
        x_savepoint: None,
        x_release: None,
        x_rollback_to: None,
        x_shadow_name: None,
        x_integrity: None,
    };

///* Register the various functions and the virtual table.
extern "C" fn spellfix1_register(db: *mut Sqlite3) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i: u32 = 0 as u32;
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"spellfix1_translit".as_ptr() as *mut i8 as *const i8, 1,
                    1 | 2048, core::ptr::null_mut(),
                    Some(transliterate_sql_func), None, None)
            };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"spellfix1_editdist".as_ptr() as *mut i8 as *const i8, 2,
                        1 | 2048, core::ptr::null_mut(), Some(editdist_sql_func),
                        None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"spellfix1_phonehash".as_ptr() as *mut i8 as *const i8, 1,
                        1 | 2048, core::ptr::null_mut(),
                        Some(phonetic_hash_sql_func), None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"spellfix1_scriptcode".as_ptr() as *mut i8 as *const i8, 1,
                        1 | 2048, core::ptr::null_mut(), Some(script_code_sql_func),
                        None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_module(db,
                        c"spellfix1".as_ptr() as *mut i8 as *const i8,
                        &raw mut spellfix1_module as *const Sqlite3Module,
                        core::ptr::null_mut())
                };
        }
        if rc == 0 { rc = edit_dist3_install(db); }
        {
            i = 0 as u32;
            '__b45: loop {
                if !((i as u64) <
                                core::mem::size_of::<[Transliteration; 389]>() as u64 /
                                        core::mem::size_of::<Transliteration>() as u64 - 1 as u64) {
                    break '__b45;
                }
                '__c45: loop { { let _ = 0; }; break '__c45; }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return rc;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_spellfix_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    { let _ = p_api_1; };
    return spellfix1_register(db);
    return 0;
}

static mut az_conflict: [*const i8; 5] =
    [c"ROLLBACK".as_ptr() as *const i8, c"IGNORE".as_ptr() as *const i8,
            c"ABORT".as_ptr() as *const i8, c"ABORT".as_ptr() as *const i8,
            c"REPLACE".as_ptr() as *const i8];

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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn isspace(_c: i32)
    -> i32;
    fn qsort(__base: *mut (), __nel: u64, __width: u64,
    __compar: unsafe extern "C" fn(*const (), *const ()) -> i32)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
