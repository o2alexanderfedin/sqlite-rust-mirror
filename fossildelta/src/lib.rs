#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor, SqliteInt64,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

///* Must be a 16-bit value
type S16 = i16;

#[repr(C)]
#[derive(Copy, Clone)]
struct Hash {
    a: u16,
    b: u16,
    i: u16,
    z: [i8; 16],
}

///* Initialize the rolling hash using the first NHASH characters of z[]
extern "C" fn hash_init(p_hash_1: &mut Hash, z: *const i8) -> () {
    let mut a: u16 = 0 as u16;
    let mut b: u16 = 0 as u16;
    let mut i: u16 = 0 as u16;
    a = { b = unsafe { *z.offset(0 as isize) } as u16; b };
    {
        i = 1 as u16;
        '__b0: loop {
            if !((i as i32) < 16) { break '__b0; }
            '__c0: loop {
                a += unsafe { *z.add(i as usize) } as i32 as u16;
                b += a as i32 as u16;
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        memcpy(&raw mut (*p_hash_1).z[0 as usize] as *mut i8 as *mut (),
            z as *const (), 16 as u64)
    };
    (*p_hash_1).a = (a as i32 & 65535) as u16;
    (*p_hash_1).b = (b as i32 & 65535) as u16;
    (*p_hash_1).i = 0 as u16;
}

///* Advance the rolling hash by a single character "c"
extern "C" fn hash_next(p_hash_1: &mut Hash, c: i32) -> () {
    let old: u16 = (*p_hash_1).z[(*p_hash_1).i as usize] as u16;
    (*p_hash_1).z[(*p_hash_1).i as usize] = c as i8;
    (*p_hash_1).i = ((*p_hash_1).i as i32 + 1 & 16 - 1) as u16;
    (*p_hash_1).a = ((*p_hash_1).a as i32 - old as i32 + c) as u16;
    (*p_hash_1).b =
        ((*p_hash_1).b as i32 - 16 * old as i32 + (*p_hash_1).a as i32) as
            u16;
}

///* Return a 32-bit hash value
extern "C" fn hash_32bit(p_hash_1: &Hash) -> u32 {
    return ((*p_hash_1).a as i32 & 65535) as u32 |
            (((*p_hash_1).b as i32 & 65535) as u32) << 16;
}

///* Compute a hash on NHASH bytes.
///*
///* This routine is intended to be equivalent to:
///*    hash h;
///*    hash_init(&h, zInput);
///*    return hash_32bit(&h);
extern "C" fn hash_once(z: *const i8) -> u32 {
    let mut a: u16 = 0 as u16;
    let mut b: u16 = 0 as u16;
    let mut i: u16 = 0 as u16;
    a = { b = unsafe { *z.offset(0 as isize) } as u16; b };
    {
        i = 1 as u16;
        '__b1: loop {
            if !((i as i32) < 16) { break '__b1; }
            '__c1: loop {
                a += unsafe { *z.add(i as usize) } as i32 as u16;
                b += a as i32 as u16;
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return a as u32 | (b as u32) << 16;
}

///* Write an base-64 integer into the given buffer.
#[allow(unused_doc_comments)]
extern "C" fn put_int(mut v: u32, pz: &mut *mut i8) -> () {
    ///  123456789 123456789 123456789 123456789 123456789 123456789 123
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut z_buf: [i8; 20] = [0; 20];
    if v == 0 as u32 {
        unsafe {
            *{
                        let __p = &mut *pz;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = '0' as i32 as i8
        };
        return;
    }
    {
        i = 0;
        '__b2: loop {
            if !(v > 0 as u32) { break '__b2; }
            '__c2: loop {
                z_buf[i as usize] = z_digits[(v & 63 as u32) as usize] as i8;
                break '__c2;
            }
            {
                ({ let __p = &mut i; let __t = *__p; *__p += 1; __t }) as u32;
                v >>= 6 as u32
            };
        }
    }
    {
        j = i - 1;
        '__b3: loop {
            if !(j >= 0) { break '__b3; }
            '__c3: loop {
                unsafe {
                    *{
                                let __p = &mut *pz;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = z_buf[j as usize]
                };
                break '__c3;
            }
            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
        }
    }
}

///* Read bytes from *pz and convert them into a positive integer.  When
///* finished, leave *pz pointing to the first character past the end of
///* the integer.  The *pLen parameter holds the length of the string
///* in *pz and is decremented once for each character in the integer.
extern "C" fn delta_get_int(pz: &mut *const i8, p_len_1: &mut i32) -> u32 {
    let mut v: u32 = 0 as u32;
    let mut c: i32 = 0;
    let mut z: *mut u8 = *pz as *mut u8;
    let z_end: *mut u8 = unsafe { z.offset(*p_len_1 as isize) };
    while z < z_end && { c = z_value[unsafe { *z } as usize] as i32; c } >= 0
        {
        v = (v << 6) + c as u32;
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    *p_len_1 -= unsafe { z.offset_from(*pz as *mut u8) } as i64 as i32;
    *pz = z as *mut i8 as *const i8;
    return v;
}

///* Return the number digits in the base-64 representation of a positive integer
extern "C" fn digit_count(v: i32) -> i32 {
    let mut i: u32 = 0 as u32;
    let mut x: u32 = 0 as u32;
    {
        { i = 1 as u32; x = 64 as u32 };
        '__b5: loop {
            if !(v as u32 >= x) { break '__b5; }
            '__c5: loop { break '__c5; }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                x <<= 6 as u32
            };
        }
    }
    return i as i32;
}

///* Compute a 32-bit big-endian checksum on the N-byte buffer.  If the
///* buffer is not a multiple of 4 bytes length, compute the sum that would
///* have occurred if the buffer was padded with zeros to the next multiple
///* of four bytes.
#[allow(unused_doc_comments)]
extern "C" fn checksum(z_in_1: *const i8, mut n_1: u64) -> u32 {
    let mut z: *const u8 = z_in_1 as *const u8;
    let z_end: *const u8 =
        unsafe { &raw const *z_in_1.add((n_1 & !3 as u64) as usize) } as
            *const u8;
    let mut sum: u32 = 0 as u32;
    if !(unsafe { z.offset_from(0 as *const u8) } as i64 % 4 as i64 ==
                            0 as i64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"checksum".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 232,
                c"(z - (const unsigned char*)0)%4==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if 0 == unsafe { *(&raw const byte_order_test as *mut i8) } as i32 {
        while z < z_end {
            sum += unsafe { *(z as *mut u32) };
            {
                let __n = 4;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
    } else {
        /// A little-endian machine
        let mut sum0: u32 = 0 as u32;
        let mut sum1: u32 = 0 as u32;
        let mut sum2: u32 = 0 as u32;
        while n_1 >= 16 as u64 {
            sum0 +=
                unsafe { *z.offset(0 as isize) } as u32 +
                            unsafe { *z.offset(4 as isize) } as u32 +
                        unsafe { *z.offset(8 as isize) } as u32 +
                    unsafe { *z.offset(12 as isize) } as u32;
            sum1 +=
                unsafe { *z.offset(1 as isize) } as u32 +
                            unsafe { *z.offset(5 as isize) } as u32 +
                        unsafe { *z.offset(9 as isize) } as u32 +
                    unsafe { *z.offset(13 as isize) } as u32;
            sum2 +=
                unsafe { *z.offset(2 as isize) } as u32 +
                            unsafe { *z.offset(6 as isize) } as u32 +
                        unsafe { *z.offset(10 as isize) } as u32 +
                    unsafe { *z.offset(14 as isize) } as u32;
            sum +=
                unsafe { *z.offset(3 as isize) } as u32 +
                            unsafe { *z.offset(7 as isize) } as u32 +
                        unsafe { *z.offset(11 as isize) } as u32 +
                    unsafe { *z.offset(15 as isize) } as u32;
            {
                let __n = 16;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n_1 -= 16 as u64;
        }
        while n_1 >= 4 as u64 {
            sum0 += unsafe { *z.offset(0 as isize) } as u32;
            sum1 += unsafe { *z.offset(1 as isize) } as u32;
            sum2 += unsafe { *z.offset(2 as isize) } as u32;
            sum += unsafe { *z.offset(3 as isize) } as u32;
            {
                let __n = 4;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n_1 -= 4 as u64;
        }
        sum += (sum2 << 8) + (sum1 << 16) + (sum0 << 24);
    }
    '__s9:
        {
        match n_1 & 3 as u64 {
            3 => {
                sum +=
                    ((unsafe { *z.offset(2 as isize) } as i32) << 8) as u32;
                sum +=
                    ((unsafe { *z.offset(1 as isize) } as i32) << 16) as u32;
                sum +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            2 => {
                sum +=
                    ((unsafe { *z.offset(1 as isize) } as i32) << 16) as u32;
                sum +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            1 => {
                sum +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            _ => {}
        }
    }
    return sum;
}

///* Create a new delta.
///*
///* The delta is written into a preallocated buffer, zDelta, which
///* should be at least 60 bytes longer than the target file, zOut.
///* The delta string will be NUL-terminated, but it might also contain
///* embedded NUL characters if either the zSrc or zOut files are
///* binary.  This function returns the length of the delta string
///* in bytes, excluding the final NUL terminator character.  It returns
///* 0 if an OOM occurs.
///*
///* Output Format:
///*
///* The delta begins with a base64 number followed by a newline.  This
///* number is the number of bytes in the TARGET file.  Thus, given a
///* delta file z, a program can compute the size of the output file
///* simply by reading the first line and decoding the base-64 number
///* found there.  The delta_output_size() routine does exactly this.
///*
///* After the initial size number, the delta consists of a series of
///* literal text segments and commands to copy from the SOURCE file.
///* A copy command looks like this:
///*
///*     NNN@MMM,
///*
///* where NNN is the number of bytes to be copied and MMM is the offset
///* into the source file of the first byte (both base-64).   If NNN is 0
///* it means copy the rest of the input file.  Literal text is like this:
///*
///*     NNN:TTTTT
///*
///* where NNN is the number of bytes of text (base-64) and TTTTT is the text.
///*
///* The last term is of the form
///*
///*     NNN;
///*
///* In this case, NNN is a 32-bit bigendian checksum of the output file
///* that can be used to verify that the delta applied correctly.  All
///* numbers are in base-64.
///*
///* Pure text files generate a pure text delta.  Binary files generate a
///* delta that may contain some binary data.
///*
///* Algorithm:
///*
///* The encoder first builds a hash table to help it find matching
///* patterns in the source file.  16-byte chunks of the source file
///* sampled at evenly spaced intervals are used to populate the hash
///* table.
///*
///* Next we begin scanning the target file using a sliding 16-byte
///* window.  The hash of the 16-byte window in the target is used to
///* search for a matching section in the source file.  When a match
///* is found, a copy command is added to the delta.  An effort is
///* made to extend the matching section to regions that come before
///* and after the 16-byte hash window.  A copy command is only issued
///* if the result would use less space that just quoting the text
///* literally. Literal text is added to the delta for sections that
///* do not match or which can not be encoded efficiently using copy
///* commands.
#[allow(unused_doc_comments)]
extern "C" fn delta_create(z_src_1: *const i8, len_src_1: u32,
    z_out_1: *const i8, len_out_1: u32, mut z_delta_1: *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut base: i32 = 0;
    let z_orig_delta: *const i8 = z_delta_1 as *const i8;
    let mut h: Hash = unsafe { core::mem::zeroed() };
    let mut n_hash: i32 = 0;
    /// Number of hash table entries
    let mut landmark: *mut i32 = core::ptr::null_mut();
    /// Primary hash table
    let mut collide: *mut i32 = core::ptr::null_mut();
    /// Collision chain
    let mut last_read: i32 = -1;

    /// Last byte of zSrc read by a COPY command
    /// Add the target file size to the beginning of the delta
    put_int(len_out_1, &mut z_delta_1);
    unsafe {
        *{
                    let __p = &mut z_delta_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = '\n' as i32 as i8
    };
    if len_src_1 <= 16 as u32 {
        put_int(len_out_1, &mut z_delta_1);
        unsafe {
            *{
                        let __p = &mut z_delta_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = ':' as i32 as i8
        };
        unsafe {
            memcpy(z_delta_1 as *mut (), z_out_1 as *const (),
                len_out_1 as u64)
        };
        {
            let __n = len_out_1;
            let __p = &mut z_delta_1;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        put_int(checksum(z_out_1, len_out_1 as u64), &mut z_delta_1);
        unsafe {
            *{
                        let __p = &mut z_delta_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = ';' as i32 as i8
        };
        return unsafe { z_delta_1.offset_from(z_orig_delta) } as i64 as i32;
    }

    /// Compute the hash table used to locate matching sections in the
    ///* source file.
    (n_hash = (len_src_1 / 16 as u32) as i32);
    collide =
        unsafe {
                sqlite3_malloc64((n_hash as Sqlite3Int64 * 2 as Sqlite3Int64)
                            as u64 * core::mem::size_of::<i32>() as u64)
            } as *mut i32;
    if collide == core::ptr::null_mut() { return 0; }
    unsafe {
        memset(collide as *mut (), -1,
            (n_hash * 2) as u64 * core::mem::size_of::<i32>() as u64)
    };
    landmark = unsafe { collide.offset(n_hash as isize) };
    {
        i = 0;
        '__b10: loop {
            if !((i as u32) < len_src_1 - 16 as u32) { break '__b10; }
            '__c10: loop {
                let mut hv: i32 =
                    (hash_once(unsafe { &*z_src_1.offset(i as isize) }) %
                            n_hash as u32) as i32;
                unsafe {
                    *collide.offset((i / 16) as isize) =
                        unsafe { *landmark.offset(hv as isize) }
                };
                unsafe { *landmark.offset(hv as isize) = i / 16 };
                break '__c10;
            }
            i += 16;
        }
    }

    /// Begin scanning the target file and generating copy commands and
    ///* literal sections of the delta.
    (base = 0);
    while ((base + 16) as u32) < len_out_1 {
        let mut i_src: i32 = 0;
        let mut i_block: i32 = 0;
        let mut best_cnt: u32 = 0 as u32;
        let mut best_ofst: u32 = 0 as u32;
        let mut best_litsz: u32 = 0 as u32;
        hash_init(&mut h, unsafe { &*z_out_1.offset(base as isize) });
        i = 0;

        /// Trying to match a landmark against zOut[base+i]
        (best_cnt = 0 as u32);
        loop {
            let mut hv: i32 = 0;
            let mut limit: i32 = 250;
            hv = (hash_32bit(&h) % n_hash as u32) as i32;
            i_block = unsafe { *landmark.offset(hv as isize) };
            while i_block >= 0 &&
                    { let __p = &mut limit; let __t = *__p; *__p -= 1; __t } > 0
                {
                ///* The hash window has identified a potential match against
                ///* landmark block iBlock.  But we need to investigate further.
                ///*
                ///* Look for a region in zOut that matches zSrc. Anchor the search
                ///* at zSrc[iSrc] and zOut[base+i].  Do not include anything prior to
                ///* zOut[base] or after zOut[outLen] nor anything after zSrc[srcLen].
                ///*
                ///* Set cnt equal to the length of the match and set ofst so that
                ///* zSrc[ofst] is the first element of the match.  litsz is the number
                ///* of characters between zOut[base] and the beginning of the match.
                ///* sz will be the overhead (in bytes) needed to encode the copy
                ///* command.  Only generate copy command if the overhead of the
                ///* copy command is less than the amount of literal text to be copied.
                let mut cnt: i32 = 0;
                let mut ofst: i32 = 0;
                let mut litsz: i32 = 0;
                let mut j: i32 = 0;
                let mut k: i32 = 0;
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                let mut sz: i32 = 0;
                let mut limit_x: i32 = 0;

                /// Beginning at iSrc, match forwards as far as we can.  j counts
                ///* the number of characters that match
                (i_src = i_block * 16);
                y = base + i;
                limit_x =
                    if len_src_1 - i_src as u32 <= len_out_1 - y as u32 {
                            len_src_1
                        } else { i_src as u32 + len_out_1 - y as u32 } as i32;
                {
                    x = i_src;
                    '__b14: loop {
                        if !(x < limit_x) { break '__b14; }
                        '__c14: loop {
                            if unsafe { *z_src_1.offset(x as isize) } as i32 !=
                                    unsafe { *z_out_1.offset(y as isize) } as i32 {
                                break '__b14;
                            }
                            break '__c14;
                        }
                        {
                            { let __p = &mut x; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut y; let __t = *__p; *__p += 1; __t }
                        };
                    }
                }
                j = x - i_src - 1;
                {
                    k = 1;
                    '__b15: loop {
                        if !(k < i_src && k <= i) { break '__b15; }
                        '__c15: loop {
                            if unsafe { *z_src_1.offset((i_src - k) as isize) } as i32
                                    !=
                                    unsafe { *z_out_1.offset((base + i - k) as isize) } as i32 {
                                break '__b15;
                            }
                            break '__c15;
                        }
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    }
                }
                { let __p = &mut k; let __t = *__p; *__p -= 1; __t };

                /// Compute the offset and size of the matching region
                (ofst = i_src - k);
                cnt = j + k + 1;
                litsz = i - k;

                /// Number of bytes of literal text before the copy */
                ///        /* sz will hold the number of bytes needed to encode the "insert"
                ///* command and the copy command, not counting the "insert" text
                (sz =
                    digit_count(i - k) + digit_count(cnt) + digit_count(ofst) +
                        3);
                if cnt >= sz && cnt as u32 > best_cnt {

                    /// Remember this match only if it is the best so far and it
                    ///* does not increase the file size
                    (best_cnt = cnt as u32);
                    best_ofst = (i_src - k) as u32;
                    best_litsz = litsz as u32;
                }

                /// Check the next matching block
                (i_block = unsafe { *collide.offset(i_block as isize) });
            }
            if best_cnt > 0 as u32 {
                if best_litsz > 0 as u32 {

                    /// Add an insert command before the copy
                    put_int(best_litsz, &mut z_delta_1);
                    unsafe {
                        *{
                                    let __p = &mut z_delta_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = ':' as i32 as i8
                    };
                    unsafe {
                        memcpy(z_delta_1 as *mut (),
                            unsafe { &raw const *z_out_1.offset(base as isize) } as
                                *const (), best_litsz as u64)
                    };
                    {
                        let __n = best_litsz;
                        let __p = &mut z_delta_1;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    base += best_litsz as i32;
                }
                base += best_cnt as i32;
                put_int(best_cnt, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = '@' as i32 as i8
                };
                put_int(best_ofst, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = ',' as i32 as i8
                };
                if best_ofst + best_cnt - 1 as u32 > last_read as u32 {
                    last_read = (best_ofst + best_cnt - 1 as u32) as i32;
                }
                best_cnt = 0 as u32;
                break;
            }
            if (base + i + 16) as u32 >= len_out_1 {

                /// We have reached the end of the file and have not found any
                ///* matches.  Do an "insert" for everything that does not match
                put_int(len_out_1 - base as u32, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = ':' as i32 as i8
                };
                unsafe {
                    memcpy(z_delta_1 as *mut (),
                        unsafe { &raw const *z_out_1.offset(base as isize) } as
                            *const (), (len_out_1 - base as u32) as u64)
                };
                {
                    let __n = len_out_1 - base as u32;
                    let __p = &mut z_delta_1;
                    *__p = unsafe { (*__p).add(__n as usize) };
                };
                base = len_out_1 as i32;
                break;
            }

            /// Advance the hash by one character.  Keep looking for a match
            hash_next(&mut h,
                unsafe { *z_out_1.offset((base + i + 16) as isize) } as i32);
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (base as u32) < len_out_1 {
        put_int(len_out_1 - base as u32, &mut z_delta_1);
        unsafe {
            *{
                        let __p = &mut z_delta_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = ':' as i32 as i8
        };
        unsafe {
            memcpy(z_delta_1 as *mut (),
                unsafe { &raw const *z_out_1.offset(base as isize) } as
                    *const (), (len_out_1 - base as u32) as u64)
        };
        {
            let __n = len_out_1 - base as u32;
            let __p = &mut z_delta_1;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
    }

    /// Output the final checksum record.
    put_int(checksum(z_out_1, len_out_1 as u64), &mut z_delta_1);
    unsafe {
        *{
                    let __p = &mut z_delta_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = ';' as i32 as i8
    };
    unsafe { sqlite3_free(collide as *mut ()) };
    return unsafe { z_delta_1.offset_from(z_orig_delta) } as i64 as i32;
}

///* Return the size (in bytes) of the output from applying
///* a delta.
///*
///* This routine is provided so that an procedure that is able
///* to call delta_apply() can learn how much space is required
///* for the output and hence allocate nor more space that is really
///* needed.
#[allow(unused_doc_comments)]
extern "C" fn delta_output_size(mut z_delta_1: *const i8,
    mut len_delta_1: i32) -> i32 {
    let mut size: i32 = 0;
    size = delta_get_int(&mut z_delta_1, &mut len_delta_1) as i32;
    if len_delta_1 <= 0 || unsafe { *z_delta_1 } as i32 != '\n' as i32 {

        /// ERROR: size integer not terminated by "\n"
        return -1;
    }
    return size;
}

///* Apply a delta.
///*
///* The output buffer should be big enough to hold the whole output
///* file and a NUL terminator at the end.  The delta_output_size()
///* routine will determine this size for you.
///*
///* The delta string should be null-terminated.  But the delta string
///* may contain embedded NUL characters (if the input and output are
///* binary files) so we also have to pass in the length of the delta in
///* the lenDelta parameter.
///*
///* This function returns the size of the output file in bytes (excluding
///* the final NUL terminator character).  Except, if the delta string is
///* malformed or intended for use with a source file other than zSrc,
///* then this routine returns -1.
///*
///* Refer to the delta_create() documentation above for a description
///* of the delta file format.
#[allow(unused_doc_comments)]
extern "C" fn delta_apply(z_src_1: *const i8, len_src_1: i32,
    mut z_delta_1: *const i8, mut len_delta_1: i32, mut z_out_1: *mut i8)
    -> i32 {
    let mut limit: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut total: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    limit = delta_get_int(&mut z_delta_1, &mut len_delta_1) as Sqlite3Uint64;
    if len_delta_1 <= 0 || unsafe { *z_delta_1 } as i32 != '\n' as i32 {

        /// ERROR: size integer not terminated by "\n"
        return -1;
    }
    {
        let __p = &mut z_delta_1;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    { let __p = &mut len_delta_1; let __t = *__p; *__p -= 1; __t };
    while unsafe { *z_delta_1 } != 0 && len_delta_1 > 0 {
        let mut cnt: u32 = 0 as u32;
        let mut ofst: u32 = 0 as u32;
        cnt = delta_get_int(&mut z_delta_1, &mut len_delta_1);
        if len_delta_1 <= 0 { return -1; }
        '__s17:
            {
            match unsafe { *z_delta_1.offset(0 as isize) } {
                64 => {
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        ofst = delta_get_int(&mut z_delta_1, &mut len_delta_1);
                        if len_delta_1 > 0 &&
                                unsafe { *z_delta_1.offset(0 as isize) } as i32 !=
                                    ',' as i32 {

                            /// ERROR: copy command not terminated by ','
                            return -1;
                        }
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        total += cnt as Sqlite3Uint64;
                        if total > limit {

                            /// ERROR: copy exceeds output file size
                            return -1;
                        }
                        if ofst as u64 + cnt as u64 > len_src_1 as u64 {

                            /// ERROR: copy extends past end of input
                            return -1;
                        }
                        unsafe {
                            memcpy(z_out_1 as *mut (),
                                unsafe { &raw const *z_src_1.add(ofst as usize) } as
                                    *const (), cnt as u64)
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_out_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        break '__s17;
                    }
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        total += cnt as Sqlite3Uint64;
                        if total > limit {

                            /// ERROR:  insert command gives an output larger than predicted
                            return -1;
                        }
                        if cnt > len_delta_1 as u32 {

                            /// ERROR: insert count exceeds size of delta
                            return -1;
                        }
                        unsafe {
                            memcpy(z_out_1 as *mut (), z_delta_1 as *const (),
                                cnt as u64)
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_out_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_delta_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        len_delta_1 -= cnt as i32;
                        break '__s17;
                    }
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { *z_out_1.offset(0 as isize) = 0 as i8 };
                        if total != limit {

                            /// ERROR: generated size does not match predicted size
                            return -1;
                        }
                        return total as i32;
                    }
                    {

                        /// ERROR: unknown delta operator
                        return -1;
                    }
                }
                58 => {
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        total += cnt as Sqlite3Uint64;
                        if total > limit {

                            /// ERROR:  insert command gives an output larger than predicted
                            return -1;
                        }
                        if cnt > len_delta_1 as u32 {

                            /// ERROR: insert count exceeds size of delta
                            return -1;
                        }
                        unsafe {
                            memcpy(z_out_1 as *mut (), z_delta_1 as *const (),
                                cnt as u64)
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_out_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_delta_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        len_delta_1 -= cnt as i32;
                        break '__s17;
                    }
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { *z_out_1.offset(0 as isize) = 0 as i8 };
                        if total != limit {

                            /// ERROR: generated size does not match predicted size
                            return -1;
                        }
                        return total as i32;
                    }
                    {

                        /// ERROR: unknown delta operator
                        return -1;
                    }
                }
                59 => {
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { *z_out_1.offset(0 as isize) = 0 as i8 };
                        if total != limit {

                            /// ERROR: generated size does not match predicted size
                            return -1;
                        }
                        return total as i32;
                    }
                    {

                        /// ERROR: unknown delta operator
                        return -1;
                    }
                }
                _ => {
                    {

                        /// ERROR: unknown delta operator
                        return -1;
                    }
                }
            }
        }
    }

    /// ERROR: unterminated delta
    return -1;
}

///* SQL functions:  delta_create(X,Y)
///*
///* Return a delta for carrying X into Y.
#[allow(unused_doc_comments)]
extern "C" fn delta_create_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut a_orig: *const i8 = core::ptr::null();
    let mut n_orig: i32 = 0;
    /// old blob
    let mut a_new: *const i8 = core::ptr::null();
    let mut n_new: i32 = 0;
    /// new blob
    let mut a_out: *mut i8 = core::ptr::null_mut();
    let mut n_out: i32 = 0;

    /// output delta
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"deltaCreateFunc".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 664,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) } == 5
        {
        return;
    }
    n_orig =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    a_orig =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    n_new =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) }) };
    a_new =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(1 as isize) }) } as
            *const i8;
    a_out =
        unsafe { sqlite3_malloc64((n_new + 70) as Sqlite3Uint64) } as *mut i8;
    if a_out == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
    } else {
        n_out =
            delta_create(a_orig, n_orig as u32, a_new, n_new as u32, a_out);
        if n_out < 0 {
            unsafe { sqlite3_free(a_out as *mut ()) };
            unsafe {
                sqlite3_result_error(context,
                    c"cannot create fossil delta".as_ptr() as *mut i8 as
                        *const i8, -1)
            };
        } else {
            unsafe {
                sqlite3_result_blob(context, a_out as *const (), n_out,
                    Some(sqlite3_free))
            };
        }
    }
}

///* SQL functions:  delta_apply(X,D)
///*
///* Return the result of applying delta D to input X.
#[allow(unused_doc_comments)]
extern "C" fn delta_apply_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut a_orig: *const i8 = core::ptr::null();
    let mut n_orig: i32 = 0;
    /// The X input
    let mut a_delta: *const i8 = core::ptr::null();
    let mut n_delta: i32 = 0;
    /// The input delta (D)
    let mut a_out: *mut i8 = core::ptr::null_mut();
    let mut n_out: i32 = 0;
    let mut n_out2: i32 = 0;

    /// The output
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"deltaApplyFunc".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 699,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) } == 5
        {
        return;
    }
    n_orig =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    a_orig =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    n_delta =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) }) };
    a_delta =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(1 as isize) }) } as
            *const i8;

    /// Figure out the size of the output
    (n_out = delta_output_size(a_delta, n_delta));
    if n_out < 0 {
        unsafe {
            sqlite3_result_error(context,
                c"corrupt fossil delta".as_ptr() as *mut i8 as *const i8, -1)
        };
        return;
    }
    a_out =
        unsafe {
                sqlite3_malloc64((n_out as Sqlite3Int64 + 1 as Sqlite3Int64)
                        as Sqlite3Uint64)
            } as *mut i8;
    if a_out == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
    } else {
        n_out2 = delta_apply(a_orig, n_orig, a_delta, n_delta, a_out);
        if n_out2 != n_out {
            unsafe { sqlite3_free(a_out as *mut ()) };
            unsafe {
                sqlite3_result_error(context,
                    c"corrupt fossil delta".as_ptr() as *mut i8 as *const i8,
                    -1)
            };
        } else {
            unsafe {
                sqlite3_result_blob(context, a_out as *const (), n_out,
                    Some(sqlite3_free))
            };
        }
    }
}

///* SQL functions:  delta_output_size(D)
///*
///* Return the size of the output that results from applying delta D.
#[allow(unused_doc_comments)]
extern "C" fn delta_output_size_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut a_delta: *const i8 = core::ptr::null();
    let mut n_delta: i32 = 0;
    /// The input delta (D)
    let mut n_out: i32 = 0;

    /// Size of output
    if !(argc == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"deltaOutputSizeFunc".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 740,
                c"argc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    n_delta =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    a_delta =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;

    /// Figure out the size of the output
    (n_out = delta_output_size(a_delta, n_delta));
    if n_out < 0 {
        unsafe {
            sqlite3_result_error(context,
                c"corrupt fossil delta".as_ptr() as *mut i8 as *const i8, -1)
        };
        return;
    } else { unsafe { sqlite3_result_int(context, n_out) }; }
}

///**************************************************************************
///* Table-valued SQL function:   delta_parse(DELTA)
///*
///* Schema:
///*
///*     CREATE TABLE delta_parse(
///*       op TEXT,
///*       a1 INT,
///*       a2 ANY,
///*       delta HIDDEN BLOB
///*     );
///*
///* Given an input DELTA, this function parses the delta and returns
///* rows for each entry in the delta.  The op column has one of the
///* values SIZE, COPY, INSERT, CHECKSUM, ERROR.
///*
///* Assuming no errors, the first row has op='SIZE'.  a1 is the size of
///* the output in bytes and a2 is NULL.
///*
///* After the initial SIZE row, there are zero or more 'COPY' and/or 'INSERT'
///* rows.  A COPY row means content is copied from the source into the
///* output.  Column a1 is the number of bytes to copy and a2 is the offset
///* into source from which to begin copying.  An INSERT row means to
///* insert text into the output stream.  Column a1 is the number of bytes
///* to insert and column is a BLOB that contains the text to be inserted.
///*
///* The last row of a well-formed delta will have an op value of 'CHECKSUM'.
///* The a1 column will be the value of the checksum and a2 will be NULL.
///*
///* If the input delta is not well-formed, then a row with an op value
///* of 'ERROR' is returned.  The a1 value of the ERROR row is the offset
///* into the delta where the error was encountered and a2 is NULL.
#[repr(C)]
#[derive(Copy, Clone)]
struct DeltaparsevtabVtab {
    base: Sqlite3Vtab,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DeltaparsevtabCursor {
    base: Sqlite3VtabCursor,
    a_delta: *mut i8,
    i_cursor: i64,
    i_next: i64,
    n_delta: i64,
    e_op: i32,
    a1: u32,
    a2: u32,
}

/// Operator names:
static mut az_op: [*const i8; 6] =
    [c"SIZE".as_ptr() as *const i8, c"COPY".as_ptr() as *const i8,
            c"INSERT".as_ptr() as *const i8,
            c"CHECKSUM".as_ptr() as *const i8, c"ERROR".as_ptr() as *const i8,
            c"EOF".as_ptr() as *const i8];

///* The deltaparsevtabConnect() method is invoked to create a new
///* deltaparse virtual table.
///*
///* Think of this routine as the constructor for deltaparsevtab_vtab objects.
///*
///* All this routine needs to do is:
///*
///*    (1) Allocate the deltaparsevtab_vtab object and initialize all fields.
///*
///*    (2) Tell SQLite (via the sqlite3_declare_vtab() interface) what the
///*        result set of queries against the virtual table will look like.
extern "C" fn deltaparsevtab_connect(db: *mut Sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut DeltaparsevtabVtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(op,a1,a2,delta HIDDEN)".as_ptr() as *mut i8
                    as *const i8)
        };
    if rc == 0 {
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<DeltaparsevtabVtab>()
                            as Sqlite3Uint64)
                } as *mut DeltaparsevtabVtab;
        unsafe { *pp_vtab_1 = p_new as *mut Sqlite3Vtab };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<DeltaparsevtabVtab>() as u64)
        };
        unsafe { sqlite3_vtab_config(db, 2) };
    }
    return rc;
}

///* This method is the destructor for deltaparsevtab_vtab objects.
extern "C" fn deltaparsevtab_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut DeltaparsevtabVtab = p_vtab_1 as *mut DeltaparsevtabVtab;
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}

///* Constructor for a new deltaparsevtab_cursor object.
extern "C" fn deltaparsevtab_open(p: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_cur: *mut DeltaparsevtabCursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<DeltaparsevtabCursor>()
                        as Sqlite3Uint64)
            } as *mut DeltaparsevtabCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<DeltaparsevtabCursor>() as u64)
    };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}

///* Destructor for a deltaparsevtab_cursor.
extern "C" fn deltaparsevtab_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut DeltaparsevtabCursor = cur as *mut DeltaparsevtabCursor;
    unsafe { sqlite3_free(unsafe { (*p_cur).a_delta } as *mut ()) };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}

///* Advance a deltaparsevtab_cursor to its next row of output.
extern "C" fn deltaparsevtab_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut DeltaparsevtabCursor = cur as *mut DeltaparsevtabCursor;
    let mut z: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    unsafe { (*p_cur).i_cursor = unsafe { (*p_cur).i_next } };
    if unsafe { (*p_cur).i_cursor } >= unsafe { (*p_cur).n_delta } {
        unsafe { (*p_cur).e_op = 4 };
        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
        return 0;
    }
    z =
        unsafe {
                unsafe {
                    (*p_cur).a_delta.offset(unsafe { (*p_cur).i_cursor } as
                            isize)
                }
            } as *const i8;
    unsafe { (*p_cur).a1 = delta_get_int(&mut z, &mut i) };
    '__s18:
        {
        match if i > 0 {
                (unsafe { *z.offset(0 as isize) }) as i32
            } else { 0 } {
            64 => {
                {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    if unsafe { (*p_cur).i_next } >= unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                        break '__s18;
                    }
                    unsafe { (*p_cur).a2 = delta_get_int(&mut z, &mut i) };
                    unsafe { (*p_cur).e_op = 1 };
                    unsafe {
                        (*p_cur).i_next =
                            unsafe {
                                        unsafe {
                                            z.offset(1 as
                                                        isize).offset_from(unsafe { (*p_cur).a_delta })
                                        }
                                    } as i64 as i64
                    };
                    break '__s18;
                }
                {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    unsafe {
                        (*p_cur).a2 =
                            unsafe { z.offset_from(unsafe { (*p_cur).a_delta }) } as i64
                                as u32
                    };
                    unsafe { (*p_cur).e_op = 2 };
                    unsafe {
                        (*p_cur).i_next =
                            unsafe {
                                        unsafe {
                                            z.add(unsafe { (*p_cur).a1 } as
                                                        usize).offset_from(unsafe { (*p_cur).a_delta })
                                        }
                                    } as i64 as i64
                    };
                    break '__s18;
                }
                {
                    unsafe { (*p_cur).e_op = 3 };
                    unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    break '__s18;
                }
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
            58 => {
                {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    unsafe {
                        (*p_cur).a2 =
                            unsafe { z.offset_from(unsafe { (*p_cur).a_delta }) } as i64
                                as u32
                    };
                    unsafe { (*p_cur).e_op = 2 };
                    unsafe {
                        (*p_cur).i_next =
                            unsafe {
                                        unsafe {
                                            z.add(unsafe { (*p_cur).a1 } as
                                                        usize).offset_from(unsafe { (*p_cur).a_delta })
                                        }
                                    } as i64 as i64
                    };
                    break '__s18;
                }
                {
                    unsafe { (*p_cur).e_op = 3 };
                    unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    break '__s18;
                }
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
            59 => {
                {
                    unsafe { (*p_cur).e_op = 3 };
                    unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    break '__s18;
                }
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
            _ => {
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
        }
    }
    return 0;
}

///* Return values of columns for the row at which the deltaparsevtab_cursor
///* is currently pointing.
extern "C" fn deltaparsevtab_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    unsafe {
        let p_cur: *const DeltaparsevtabCursor =
            cur as *mut DeltaparsevtabCursor as *const DeltaparsevtabCursor;
        '__s19:
            {
            match i {
                0 => {
                    {
                        unsafe {
                            sqlite3_result_text(ctx,
                                az_op[unsafe { (*p_cur).e_op } as usize], -1, None)
                        };
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_int(ctx, unsafe { (*p_cur).a1 } as i32)
                        };
                        break '__s19;
                    }
                    {
                        if unsafe { (*p_cur).e_op } == 1 {
                            unsafe {
                                sqlite3_result_int(ctx, unsafe { (*p_cur).a2 } as i32)
                            };
                        } else if unsafe { (*p_cur).e_op } == 2 {
                            if unsafe { (*p_cur).a2 } as i64 +
                                        unsafe { (*p_cur).a1 } as i64 > unsafe { (*p_cur).n_delta }
                                {
                                unsafe {
                                    sqlite3_result_zeroblob(ctx, unsafe { (*p_cur).a1 } as i32)
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).a_delta.add(unsafe { (*p_cur).a2 } as usize)
                                                }
                                            } as *const (), unsafe { (*p_cur).a1 } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                1 => {
                    {
                        unsafe {
                            sqlite3_result_int(ctx, unsafe { (*p_cur).a1 } as i32)
                        };
                        break '__s19;
                    }
                    {
                        if unsafe { (*p_cur).e_op } == 1 {
                            unsafe {
                                sqlite3_result_int(ctx, unsafe { (*p_cur).a2 } as i32)
                            };
                        } else if unsafe { (*p_cur).e_op } == 2 {
                            if unsafe { (*p_cur).a2 } as i64 +
                                        unsafe { (*p_cur).a1 } as i64 > unsafe { (*p_cur).n_delta }
                                {
                                unsafe {
                                    sqlite3_result_zeroblob(ctx, unsafe { (*p_cur).a1 } as i32)
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).a_delta.add(unsafe { (*p_cur).a2 } as usize)
                                                }
                                            } as *const (), unsafe { (*p_cur).a1 } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                2 => {
                    {
                        if unsafe { (*p_cur).e_op } == 1 {
                            unsafe {
                                sqlite3_result_int(ctx, unsafe { (*p_cur).a2 } as i32)
                            };
                        } else if unsafe { (*p_cur).e_op } == 2 {
                            if unsafe { (*p_cur).a2 } as i64 +
                                        unsafe { (*p_cur).a1 } as i64 > unsafe { (*p_cur).n_delta }
                                {
                                unsafe {
                                    sqlite3_result_zeroblob(ctx, unsafe { (*p_cur).a1 } as i32)
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).a_delta.add(unsafe { (*p_cur).a2 } as usize)
                                                }
                                            } as *const (), unsafe { (*p_cur).a1 } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                3 => {
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                _ => {}
            }
        }
        return 0;
    }
}

///* Return the rowid for the current row.  In this implementation, the
///* rowid is the same as the output value.
extern "C" fn deltaparsevtab_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const DeltaparsevtabCursor =
        cur as *mut DeltaparsevtabCursor as *const DeltaparsevtabCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_cursor } };
    return 0;
}

///* Return TRUE if the cursor has been moved off of the last
///* row of output.
extern "C" fn deltaparsevtab_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const DeltaparsevtabCursor =
        cur as *mut DeltaparsevtabCursor as *const DeltaparsevtabCursor;
    return (unsafe { (*p_cur).e_op } == 5 ||
                unsafe { (*p_cur).i_cursor } >= unsafe { (*p_cur).n_delta })
            as i32;
}

///* This method is called to "rewind" the deltaparsevtab_cursor object back
///* to the first row of output.  This method is always called at least
///* once prior to any call to deltaparsevtabColumn() or deltaparsevtabRowid() or
///* deltaparsevtabEof().
extern "C" fn deltaparsevtab_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut DeltaparsevtabCursor =
        p_vtab_cursor_1 as *mut DeltaparsevtabCursor;
    let mut a: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    unsafe { (*p_cur).e_op = 4 };
    if idx_num_1 != 1 { return 0; }
    unsafe {
        (*p_cur).n_delta =
            unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                } as i64
    };
    a =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if unsafe { (*p_cur).n_delta } == 0 as i64 || a == core::ptr::null() {
        return 0;
    }
    unsafe {
        (*p_cur).a_delta =
            unsafe {
                    sqlite3_malloc64((unsafe { (*p_cur).n_delta } + 1 as i64) as
                            Sqlite3Uint64)
                } as *mut i8
    };
    if unsafe { (*p_cur).a_delta } == core::ptr::null_mut() {
        unsafe { (*p_cur).n_delta = 0 as i64 };
        return 7;
    }
    unsafe {
        memcpy(unsafe { (*p_cur).a_delta } as *mut (), a as *const (),
            unsafe { (*p_cur).n_delta } as u64)
    };
    unsafe {
        *unsafe {
                    (*p_cur).a_delta.offset(unsafe { (*p_cur).n_delta } as
                            isize)
                } = 0 as i8
    };
    a = unsafe { (*p_cur).a_delta } as *const i8;
    unsafe { (*p_cur).e_op = 0 };
    unsafe { (*p_cur).a1 = delta_get_int(&mut a, &mut i) };
    if i <= 0 || unsafe { *a.offset(0 as isize) } as i32 != '\n' as i32 {
        unsafe { (*p_cur).e_op = 4 };
        unsafe {
            (*p_cur).a1 =
                { unsafe { (*p_cur).a2 = 0 as u32 }; unsafe { (*p_cur).a2 } }
        };
        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
        return 0;
    }
    {
        let __p = &mut a;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    unsafe {
        (*p_cur).i_next =
            unsafe { a.offset_from(unsafe { (*p_cur).a_delta }) } as i64 as
                i64
    };
    return 0;
}

///* SQLite will invoke this method one or more times while planning a query
///* that uses the virtual table.  This routine needs to create
///* a query plan for each invocation and compute an estimated cost for that
///* plan.
extern "C" fn deltaparsevtab_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b20: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b20;
            }
            '__c20: loop {
                if unsafe {
                            (*unsafe {
                                        (*p_idx_info_1).a_constraint.offset(i as isize)
                                    }).i_column
                        } != 3 {
                    break '__c20;
                }
                if unsafe {
                                (*unsafe {
                                            (*p_idx_info_1).a_constraint.offset(i as isize)
                                        }).usable
                            } as i32 == 0 {
                    break '__c20;
                }
                if unsafe {
                                (*unsafe {
                                            (*p_idx_info_1).a_constraint.offset(i as isize)
                                        }).op
                            } as i32 != 2 {
                    break '__c20;
                }
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
                unsafe { (*p_idx_info_1).estimated_cost = 1 as f64 };
                unsafe {
                    (*p_idx_info_1).estimated_rows = 10 as Sqlite3Int64
                };
                unsafe { (*p_idx_info_1).idx_num = 1 };
                return 0;
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p_idx_info_1).idx_num = 0 };
    unsafe { (*p_idx_info_1).estimated_cost = 2147483647 as f64 };
    unsafe { (*p_idx_info_1).estimated_rows = 2147483647 as Sqlite3Int64 };
    return 19;
}

///* This following structure defines all the methods for the
///* virtual table.
static mut deltaparsevtab_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: None,
        x_connect: Some(deltaparsevtab_connect),
        x_best_index: Some(deltaparsevtab_best_index),
        x_disconnect: Some(deltaparsevtab_disconnect),
        x_destroy: None,
        x_open: Some(deltaparsevtab_open),
        x_close: Some(deltaparsevtab_close),
        x_filter: Some(deltaparsevtab_filter),
        x_next: Some(deltaparsevtab_next),
        x_eof: Some(deltaparsevtab_eof),
        x_column: Some(deltaparsevtab_column),
        x_rowid: Some(deltaparsevtab_rowid),
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
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_fossildelta_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        { let _ = pz_err_msg_1; };

        /// Unused parameter
        (rc =
            unsafe {
                sqlite3_create_function(db,
                    c"delta_create".as_ptr() as *mut i8 as *const i8, 2, enc,
                    core::ptr::null_mut(), Some(delta_create_func), None, None)
            });
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"delta_apply".as_ptr() as *mut i8 as *const i8, 2, enc,
                        core::ptr::null_mut(), Some(delta_apply_func), None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"delta_output_size".as_ptr() as *mut i8 as *const i8, 1,
                        enc, core::ptr::null_mut(), Some(delta_output_size_func),
                        None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_module(db,
                        c"delta_parse".as_ptr() as *mut i8 as *const i8,
                        &raw mut deltaparsevtab_module as *const Sqlite3Module,
                        core::ptr::null_mut())
                };
        }
        return rc;
    }
}

static z_digits: [i8; 65] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 65 as i8, 66 as i8, 67 as i8,
            68 as i8, 69 as i8, 70 as i8, 71 as i8, 72 as i8, 73 as i8,
            74 as i8, 75 as i8, 76 as i8, 77 as i8, 78 as i8, 79 as i8,
            80 as i8, 81 as i8, 82 as i8, 83 as i8, 84 as i8, 85 as i8,
            86 as i8, 87 as i8, 88 as i8, 89 as i8, 90 as i8, 95 as i8,
            97 as i8, 98 as i8, 99 as i8, 100 as i8, 101 as i8, 102 as i8,
            103 as i8, 104 as i8, 105 as i8, 106 as i8, 107 as i8, 108 as i8,
            109 as i8, 110 as i8, 111 as i8, 112 as i8, 113 as i8, 114 as i8,
            115 as i8, 116 as i8, 117 as i8, 118 as i8, 119 as i8, 120 as i8,
            121 as i8, 122 as i8, 126 as i8, 0 as i8];

static z_value: [i8; 256] =
    [-1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, 0 as i8,
            1 as i8, 2 as i8, 3 as i8, 4 as i8, 5 as i8, 6 as i8, 7 as i8,
            8 as i8, 9 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, 10 as i8, 11 as i8, 12 as i8,
            13 as i8, 14 as i8, 15 as i8, 16 as i8, 17 as i8, 18 as i8,
            19 as i8, 20 as i8, 21 as i8, 22 as i8, 23 as i8, 24 as i8,
            25 as i8, 26 as i8, 27 as i8, 28 as i8, 29 as i8, 30 as i8,
            31 as i8, 32 as i8, 33 as i8, 34 as i8, 35 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, 36 as i8, -1 as i8, 37 as i8,
            38 as i8, 39 as i8, 40 as i8, 41 as i8, 42 as i8, 43 as i8,
            44 as i8, 45 as i8, 46 as i8, 47 as i8, 48 as i8, 49 as i8,
            50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8,
            56 as i8, 57 as i8, 58 as i8, 59 as i8, 60 as i8, 61 as i8,
            62 as i8, -1 as i8, -1 as i8, -1 as i8, 63 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8];

static byte_order_test: i32 = 1 as i32;

static enc: i32 = (1 | 2097152) as i32;

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
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
