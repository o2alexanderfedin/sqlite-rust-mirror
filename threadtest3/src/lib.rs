#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;

type DarwinSizeT = u64;

type DarwinPthreadT = *mut OpaquePthreadT;

type PthreadT = DarwinPthreadT;

type DarwinSsizeT = i64;

type DarwinIntptrT = i64;

type Int32T = i32;

type DarwinDevT = Int32T;

type DevT = DarwinDevT;

type Uint16T = u16;

type DarwinModeT = Uint16T;

type ModeT = DarwinModeT;

type NlinkT = Uint16T;

type Uint64T = u64;

type DarwinIno64T = Uint64T;

type Uint32T = u32;

type DarwinUidT = Uint32T;

type UidT = DarwinUidT;

type DarwinGidT = Uint32T;

type GidT = DarwinGidT;

type Int64T = i64;

type DarwinOffT = Int64T;

type OffT = DarwinOffT;

type DarwinBlkcntT = Int64T;

type BlkcntT = DarwinBlkcntT;

type DarwinBlksizeT = Int32T;

type BlksizeT = DarwinBlksizeT;

type DarwinTimeT = i64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atimespec: Timespec,
    st_mtimespec: Timespec,
    st_ctimespec: Timespec,
    st_birthtimespec: Timespec,
    st_size: i64,
    st_blocks: i64,
    st_blksize: i32,
    st_flags: u32,
    st_gen: u32,
    st_lspare: i32,
    st_qspare: [i64; 2],
}

#[unsafe(no_mangle)]
pub static mut sqlite3_pending_byte: i32 = 1073741824;

#[repr(C)]
#[derive(Copy, Clone)]
struct MD5Context {
    is_init: i32,
    buf: [u32; 4],
    bits: [u32; 2],
    u: MD5ContextU0,
}

#[repr(C)]
#[derive(Copy, Clone)]
union MD5ContextU0 {
    in_: [u8; 64],
    in32: [u32; 16],
}

extern "C" fn byte_reverse(mut buf: *mut u8, mut longs: u32) -> () {
    let mut t: u32 = 0 as u32;
    '__b0: loop {
        '__c0: loop {
            t =
                (((unsafe { *buf.offset(3 as isize) } as u32) << 8 |
                                    unsafe { *buf.offset(2 as isize) } as u32) as u32) << 16 |
                    ((unsafe { *buf.offset(1 as isize) } as u32) << 8 |
                        unsafe { *buf.offset(0 as isize) } as u32);
            unsafe { *(buf as *mut u32) = t };
            {
                let __n = 4;
                let __p = &mut buf;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            break '__c0;
        }
        if !({ let __p = &mut longs; *__p -= 1; *__p } != 0) { break '__b0; }
    }
}

extern "C" fn md5_transform(buf: *mut u32, in__1: *const u32) -> () {
    let mut a: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    let mut c: u32 = 0 as u32;
    let mut d: u32 = 0 as u32;
    a = unsafe { *buf.offset(0 as isize) };
    b = unsafe { *buf.offset(1 as isize) };
    c = unsafe { *buf.offset(2 as isize) };
    d = unsafe { *buf.offset(3 as isize) };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(0 as isize) } as u32 + 3614090360u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(1 as isize) } as u32 + 3905402710u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(2 as isize) } as u32 +
                    606105819 as u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(3 as isize) } as u32 + 3250441966u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(4 as isize) } as u32 + 4118548399u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(5 as isize) } as u32 +
                    1200080426 as u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(6 as isize) } as u32 + 2821735955u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(7 as isize) } as u32 + 4249261313u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(8 as isize) } as u32 +
                    1770035416 as u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(9 as isize) } as u32 + 2336552879u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(10 as isize) } as u32 +
                    4294925233u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(11 as isize) } as u32 +
                    2304563134u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(12 as isize) } as u32 +
                    1804603682 as u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(13 as isize) } as u32 +
                    4254626195u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(14 as isize) } as u32 +
                    2792965006u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(15 as isize) } as u32 +
                    1236535329 as u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(1 as isize) } as u32 + 4129170786u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(6 as isize) } as u32 + 3225465664u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(11 as isize) } as u32 +
                    643717713 as u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(0 as isize) } as u32 + 3921069994u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(5 as isize) } as u32 + 3593408605u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(10 as isize) } as u32 +
                    38016083 as u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(15 as isize) } as u32 +
                    3634488961u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(4 as isize) } as u32 + 3889429448u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(9 as isize) } as u32 +
                    568446438 as u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(14 as isize) } as u32 +
                    3275163606u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(3 as isize) } as u32 + 4107603335u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(8 as isize) } as u32 +
                    1163531501 as u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(13 as isize) } as u32 +
                    2850285829u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(2 as isize) } as u32 + 4243563512u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(7 as isize) } as u32 +
                    1735328473 as u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(12 as isize) } as u32 +
                    2368359562u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(5 as isize) } as u32 +
                    4294588738u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(8 as isize) } as u32 +
                    2272392833u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(11 as isize) } as u32 +
                    1839030562 as u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(14 as isize) } as u32 +
                    4259657740u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(1 as isize) } as u32 +
                    2763975236u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(4 as isize) } as u32 +
                    1272893353 as u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(7 as isize) } as u32 +
                    4139469664u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(10 as isize) } as u32 +
                    3200236656u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(13 as isize) } as u32 +
                    681279174 as u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(0 as isize) } as u32 +
                    3936430074u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(3 as isize) } as u32 +
                    3572445317u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(6 as isize) } as u32 +
                    76029189 as u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(9 as isize) } as u32 +
                    3654602809u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(12 as isize) } as u32 +
                    3873151461u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(15 as isize) } as u32 +
                    530742520 as u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(2 as isize) } as u32 +
                    3299628645u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(0 as isize) } as u32 +
                    4096336452u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(7 as isize) } as u32 +
                    1126891415 as u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(14 as isize) } as u32
                    + 2878612391u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(5 as isize) } as u32 +
                    4237533241u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(12 as isize) } as u32
                    + 1700485571 as u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(3 as isize) } as u32 +
                    2399980690u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(10 as isize) } as u32
                    + 4293915773u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(1 as isize) } as u32 +
                    2240044497u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(8 as isize) } as u32 +
                    1873313359 as u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(15 as isize) } as u32
                    + 4264355552u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(6 as isize) } as u32 +
                    2734768916u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(13 as isize) } as u32
                    + 1309151649 as u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(4 as isize) } as u32 +
                    4149444226u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(11 as isize) } as u32
                    + 3174756917u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(2 as isize) } as u32 +
                    718787259 as u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(9 as isize) } as u32 +
                    3951481745u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    unsafe { *buf.offset(0 as isize) += a };
    unsafe { *buf.offset(1 as isize) += b };
    unsafe { *buf.offset(2 as isize) += c };
    unsafe { *buf.offset(3 as isize) += d };
}

extern "C" fn md5_init(ctx: &mut MD5Context) -> () {
    (*ctx).is_init = 1;
    (*ctx).buf[0 as usize] = 1732584193 as u32;
    (*ctx).buf[1 as usize] = 4023233417u32;
    (*ctx).buf[2 as usize] = 2562383102u32;
    (*ctx).buf[3 as usize] = 271733878 as u32;
    (*ctx).bits[0 as usize] = 0 as u32;
    (*ctx).bits[1 as usize] = 0 as u32;
}

extern "C" fn md5_update(ctx: &mut MD5Context, mut buf: *const u8,
    mut len: u32) -> () {
    unsafe {
        let mut t: u32 = 0 as u32;
        t = (*ctx).bits[0 as usize];
        if {
                    (*ctx).bits[0 as usize] = t + ((len as u32) << 3);
                    (*ctx).bits[0 as usize]
                } < t {
            {
                let __p = &mut (*ctx).bits[1 as usize];
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        (*ctx).bits[1 as usize] += len >> 29;
        t = t >> 3 & 63 as u32;
        if t != 0 {
            let p: *mut u8 =
                unsafe {
                    (&raw mut (*ctx).u.in_[0 as usize] as
                            *mut u8).add(t as usize)
                };
            t = 64 as u32 - t;
            if len < t {
                unsafe { memcpy(p as *mut (), buf as *const (), len as u64) };
                return;
            }
            unsafe { memcpy(p as *mut (), buf as *const (), t as u64) };
            byte_reverse(&raw mut (*ctx).u.in_[0 as usize] as *mut u8,
                16 as u32);
            md5_transform(&raw mut (*ctx).buf[0 as usize] as *mut u32,
                &raw mut (*ctx).u.in_[0 as usize] as *mut u32 as *const u32);
            {
                let __n = t;
                let __p = &mut buf;
                *__p = unsafe { (*__p).add(__n as usize) };
            };
            len -= t;
        }
        while len >= 64 as u32 {
            unsafe {
                memcpy(&raw mut (*ctx).u.in_[0 as usize] as *mut u8 as
                        *mut (), buf as *const (), 64 as u64)
            };
            byte_reverse(&raw mut (*ctx).u.in_[0 as usize] as *mut u8,
                16 as u32);
            md5_transform(&raw mut (*ctx).buf[0 as usize] as *mut u32,
                &raw mut (*ctx).u.in_[0 as usize] as *mut u32 as *const u32);
            {
                let __n = 64;
                let __p = &mut buf;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            len -= 64 as u32;
        }
        unsafe {
            memcpy(&raw mut (*ctx).u.in_[0 as usize] as *mut u8 as *mut (),
                buf as *const (), len as u64)
        };
    }
}

extern "C" fn md5_final(digest: *mut u8, ctx: *mut MD5Context) -> () {
    unsafe {
        let mut count: u32 = 0 as u32;
        let mut p: *mut u8 = core::ptr::null_mut();
        count = unsafe { (*ctx).bits[0 as usize] } >> 3 & 63 as u32;
        p =
            unsafe {
                (unsafe { &raw mut (*ctx).u.in_[0 as usize] } as
                        *mut u8).add(count as usize)
            };
        unsafe {
            *{
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = 128 as u8
        };
        count = (64 - 1) as u32 - count;
        if count < 8 as u32 {
            unsafe { memset(p as *mut (), 0, count as u64) };
            byte_reverse(unsafe { &raw mut (*ctx).u.in_[0 as usize] } as
                    *mut u8, 16 as u32);
            md5_transform(unsafe { &raw mut (*ctx).buf[0 as usize] } as
                    *mut u32,
                unsafe { &raw mut (*ctx).u.in_[0 as usize] } as *mut u32 as
                    *const u32);
            unsafe {
                memset(unsafe { &raw mut (*ctx).u.in_[0 as usize] } as *mut u8
                        as *mut (), 0, 56 as u64)
            };
        } else {
            unsafe { memset(p as *mut (), 0, (count - 8 as u32) as u64) };
        }
        byte_reverse(unsafe { &raw mut (*ctx).u.in_[0 as usize] } as *mut u8,
            14 as u32);
        unsafe {
            (*ctx).u.in32[14 as usize] = unsafe { (*ctx).bits[0 as usize] }
        };
        unsafe {
            (*ctx).u.in32[15 as usize] = unsafe { (*ctx).bits[1 as usize] }
        };
        md5_transform(unsafe { &raw mut (*ctx).buf[0 as usize] } as *mut u32,
            unsafe { &raw mut (*ctx).u.in_[0 as usize] } as *mut u32 as
                *const u32);
        byte_reverse(unsafe { &raw mut (*ctx).buf[0 as usize] } as *mut u8,
            4 as u32);
        unsafe {
            memcpy(digest as *mut (),
                unsafe { &raw mut (*ctx).buf[0 as usize] } as *mut u32 as
                    *const (), 16 as u64)
        };
        unsafe {
            memset(ctx as *mut (), 0,
                core::mem::size_of::<MD5Context>() as u64)
        };
    }
}

extern "C" fn md5_digest_to_base16(digest: *const u8, z_buf_1: *mut i8)
    -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = { i = 0; i };
        '__b2: loop {
            if !(i < 16) { break '__b2; }
            '__c2: loop {
                let a: i32 = unsafe { *digest.offset(i as isize) } as i32;
                unsafe {
                    *z_buf_1.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = z_encode[(a >> 4 & 15) as usize] as i8
                };
                unsafe {
                    *z_buf_1.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = z_encode[(a & 15) as usize] as i8
                };
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z_buf_1.offset(j as isize) = 0 as i8 };
}

extern "C" fn md5step(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut MD5Context = core::ptr::null_mut();
    let mut i: i32 = 0;
    if argc < 1 { return; }
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<MD5Context>() as i32)
            } as *mut MD5Context;
    if p == core::ptr::null_mut() { return; }
    if (unsafe { (*p).is_init } == 0) as i32 != 0 {
        md5_init(unsafe { &mut *p });
    }
    {
        i = 0;
        '__b3: loop {
            if !(i < argc) { break '__b3; }
            '__c3: loop {
                let z_data: *const i8 =
                    unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *mut i8 as *const i8;
                if !(z_data).is_null() {
                    md5_update(unsafe { &mut *p },
                        z_data as *mut u8 as *const u8,
                        unsafe { strlen(z_data) } as u32);
                }
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn md5finalize(context: *mut Sqlite3Context) -> () {
    let mut p: *mut MD5Context = core::ptr::null_mut();
    let mut digest: [u8; 16] = [0; 16];
    let mut z_buf: [i8; 33] = [0; 33];
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<MD5Context>() as i32)
            } as *mut MD5Context;
    md5_final(&raw mut digest[0 as usize] as *mut u8, p);
    md5_digest_to_base16(&raw mut digest[0 as usize] as *mut u8 as *const u8,
        &raw mut z_buf[0 as usize] as *mut i8);
    unsafe {
        sqlite3_result_text(context,
            &raw mut z_buf[0 as usize] as *mut i8 as *const i8, -1,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Error {
    rc: i32,
    i_line: i32,
    z_err: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite {
    db: *mut Sqlite3,
    p_cache: *mut Statement,
    n_text: i32,
    a_text: *mut *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Statement {
    p_stmt: *mut Sqlite3Stmt,
    p_next: *mut Statement,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Threadset {
    i_max_tid: i32,
    p_thread: *mut Thread,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Thread {
    i_tid: i32,
    p_arg: *mut (),
    tid: PthreadT,
    x_proc: Option<unsafe extern "C" fn(i32, *mut ()) -> *mut i8>,
    z_res: *mut i8,
    p_next: *mut Thread,
}

static mut n_global_err: i32 = 0;

extern "C" fn free_err(p: &mut Error) -> () {
    unsafe { sqlite3_free((*p).z_err as *mut ()) };
    (*p).z_err = core::ptr::null_mut();
    (*p).rc = 0;
}

extern "C" fn print_err(p: &Error) -> () {
    unsafe {
        if (*p).rc != 0 {
            let mut is_warn: i32 = 0;
            if (*p).rc == 17 { is_warn = 1; }
            if unsafe {
                        sqlite3_strglob(c"* - no such table: *".as_ptr() as *mut i8
                                as *const i8, (*p).z_err as *const i8)
                    } == 0 {
                is_warn = 1;
            }
            unsafe {
                printf(c"%s: (%d) \"%s\" at line %d\n".as_ptr() as *mut i8 as
                        *const i8,
                    if is_warn != 0 {
                        c"Warning".as_ptr() as *mut i8
                    } else { c"Error".as_ptr() as *mut i8 }, (*p).rc,
                    (*p).z_err, (*p).i_line)
            };
            if (is_warn == 0) as i32 != 0 {
                {
                    let __p = &mut n_global_err;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            unsafe { fflush(__stdoutp) };
        }
    }
}

extern "C" fn print_and_free_err(p: *mut Error) -> () {
    print_err(unsafe { &*p });
    free_err(unsafe { &mut *p });
}

extern "C" fn system_error(p_err_1: &mut Error, i_sys_1: i32) -> () {
    (*p_err_1).rc = i_sys_1;
    (*p_err_1).z_err = unsafe { sqlite3_malloc(512) } as *mut i8;
    unsafe { strerror_r(i_sys_1, (*p_err_1).z_err, 512 as u64) };
    unsafe { *(*p_err_1).z_err.offset(511 as isize) = '\u{0}' as i32 as i8 };
}

extern "C" fn sqlite_error(p_err_1: &mut Error, p_db_1: &Sqlite,
    z_func_1: *const i8) -> () {
    (*p_err_1).rc = unsafe { sqlite3_errcode((*p_db_1).db) };
    (*p_err_1).z_err =
        unsafe {
            sqlite3_mprintf(c"sqlite3_%s() - %s (%d)".as_ptr() as *mut i8 as
                    *const i8, z_func_1,
                unsafe { sqlite3_errmsg((*p_db_1).db) },
                unsafe { sqlite3_extended_errcode((*p_db_1).db) })
        };
}

extern "C" fn test_error_x(p_err_1: &mut Error, z_err_1: *mut i8) -> () {
    if (*p_err_1).rc == 0 {
        (*p_err_1).rc = 1;
        (*p_err_1).z_err = z_err_1;
    } else { unsafe { sqlite3_free(z_err_1 as *mut ()) }; }
}

extern "C" fn clear_error_x(p_err_1: &mut Error, rc: i32) -> () {
    if (*p_err_1).rc == rc {
        (*p_err_1).rc = 0;
        unsafe { sqlite3_free((*p_err_1).z_err as *mut ()) };
        (*p_err_1).z_err = core::ptr::null_mut();
    }
}

extern "C" fn busyhandler(p_arg_1: *mut (), n: i32) -> i32 {
    unsafe { sqlite3_sleep(10) };
    return 1;
}

extern "C" fn opendb_x(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    z_file_1: *const i8, b_delete_1: i32) -> () {
    if unsafe { (*p_err_1).rc } == 0 {
        let mut rc: i32 = 0;
        let flags: i32 = 4 | 2 | 64;
        if b_delete_1 != 0 { unsafe { unlink(z_file_1) }; }
        rc =
            unsafe {
                sqlite3_open_v2(z_file_1, unsafe { &mut (*p_db_1).db }, flags,
                    core::ptr::null())
            };
        if rc != 0 {
            sqlite_error(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"open".as_ptr() as *mut i8 as *const i8);
            unsafe { sqlite3_close(unsafe { (*p_db_1).db }) };
            unsafe { (*p_db_1).db = core::ptr::null_mut() };
        } else {
            unsafe {
                sqlite3_create_function(unsafe { (*p_db_1).db },
                    c"md5sum".as_ptr() as *mut i8 as *const i8, -1, 1,
                    core::ptr::null_mut(), None, Some(md5step),
                    Some(md5finalize))
            };
            unsafe {
                sqlite3_busy_handler(unsafe { (*p_db_1).db },
                    Some(busyhandler), core::ptr::null_mut())
            };
            unsafe {
                sqlite3_exec(unsafe { (*p_db_1).db },
                    c"PRAGMA synchronous=OFF".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
    }
}

extern "C" fn closedb_x(p_err_1: &mut Error, p_db_1: *mut Sqlite) -> () {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut p_iter: *mut Statement = core::ptr::null_mut();
    let mut p_next: *mut Statement = core::ptr::null_mut();
    {
        p_iter = unsafe { (*p_db_1).p_cache };
        '__b4: loop {
            if !(!(p_iter).is_null()) { break '__b4; }
            '__c4: loop {
                p_next = unsafe { (*p_iter).p_next };
                unsafe { sqlite3_finalize(unsafe { (*p_iter).p_stmt }) };
                unsafe { sqlite3_free(p_iter as *mut ()) };
                break '__c4;
            }
            p_iter = p_next;
        }
    }
    {
        i = 0;
        '__b5: loop {
            if !(i < unsafe { (*p_db_1).n_text }) { break '__b5; }
            '__c5: loop {
                unsafe {
                    sqlite3_free(unsafe {
                                *unsafe { (*p_db_1).a_text.offset(i as isize) }
                            } as *mut ())
                };
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p_db_1).a_text } as *mut ()) };
    rc = unsafe { sqlite3_close(unsafe { (*p_db_1).db }) };
    if rc != 0 && (*p_err_1).rc == 0 {
        (*p_err_1).z_err =
            unsafe {
                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(unsafe { (*p_db_1).db }) })
            };
    }
    unsafe {
        memset(p_db_1 as *mut (), 0, core::mem::size_of::<Sqlite>() as u64)
    };
}

extern "C" fn sql_script_x(p_err_1: &mut Error, p_db_1: &Sqlite,
    z_sql_1: *const i8) -> () {
    if (*p_err_1).rc == 0 {
        (*p_err_1).rc =
            unsafe {
                sqlite3_exec((*p_db_1).db, z_sql_1, None,
                    core::ptr::null_mut(), &mut (*p_err_1).z_err)
            };
    }
}

unsafe extern "C" fn sql_script_printf_x(p_err_1: &mut Error, p_db_1: &Sqlite,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    if (*p_err_1).rc == 0 {
        let z_sql: *mut i8 = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        (*p_err_1).rc =
            unsafe {
                sqlite3_exec((*p_db_1).db, z_sql as *const i8, None,
                    core::ptr::null_mut(), &mut (*p_err_1).z_err)
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    ();
}

extern "C" fn get_sql_statement(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    z_sql_1: *const i8) -> *mut Statement {
    let mut p_ret: *mut Statement = core::ptr::null_mut();
    let mut rc: i32 = 0;
    {
        p_ret = unsafe { (*p_db_1).p_cache };
        '__b6: loop {
            if !(!(p_ret).is_null()) { break '__b6; }
            '__c6: loop {
                if 0 ==
                        unsafe {
                            strcmp(unsafe { sqlite3_sql(unsafe { (*p_ret).p_stmt }) },
                                z_sql_1)
                        } {
                    return p_ret;
                }
                break '__c6;
            }
            p_ret = unsafe { (*p_ret).p_next };
        }
    }
    p_ret =
        unsafe { sqlite3_malloc(core::mem::size_of::<Statement>() as i32) } as
            *mut Statement;
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p_db_1).db }, z_sql_1, -1,
                unsafe { &mut (*p_ret).p_stmt }, core::ptr::null_mut())
        };
    if rc != 0 {
        sqlite_error(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"prepare_v2".as_ptr() as *mut i8 as *const i8);
        return core::ptr::null_mut();
    }
    if !(0 ==
                            unsafe {
                                strcmp(unsafe { sqlite3_sql(unsafe { (*p_ret).p_stmt }) },
                                    z_sql_1)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"getSqlStatement".as_ptr() as *const i8,
                c"threadtest3.c".as_ptr() as *mut i8 as *const i8, 639,
                c"0==strcmp(sqlite3_sql(pRet->pStmt), zSql)".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe { (*p_ret).p_next = unsafe { (*p_db_1).p_cache } };
    unsafe { (*p_db_1).p_cache = p_ret };
    return p_ret;
}

extern "C" fn get_and_bind_sql_statement(p_err_1: *mut Error,
    p_db_1: *mut Sqlite, mut ap: *const i8) -> *mut Sqlite3Stmt {
    let mut p_statement: *const Statement = core::ptr::null();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut i: i32 = 0;
    p_statement =
        get_sql_statement(p_err_1, p_db_1,
            unsafe {
                let __ap = &mut ap;
                let __va_p = *__ap;
                *__ap =
                    (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                *(__va_p as *const *const i8)
            });
    if (p_statement).is_null() as i32 != 0 { return core::ptr::null_mut(); }
    p_stmt = unsafe { (*p_statement).p_stmt };
    {
        i = 1;
        '__b7: loop {
            if !(i <= unsafe { sqlite3_bind_parameter_count(p_stmt) }) {
                break '__b7;
            }
            '__c7: loop {
                let z_name: *const i8 =
                    unsafe { sqlite3_bind_parameter_name(p_stmt, i) };
                let p_arg: *mut () =
                    unsafe {
                        let __ap = &mut ap;
                        let __va_p = *__ap;
                        *__ap =
                            (*__ap).add((core::mem::size_of::<*mut ()>() + 7) & !7);
                        *(__va_p as *const *mut ())
                    };
                '__s8:
                    {
                    match unsafe { *z_name.offset(1 as isize) } {
                        105 => {
                            unsafe {
                                sqlite3_bind_int64(p_stmt, i,
                                    unsafe { *(p_arg as *mut i64) })
                            };
                        }
                        _ => {
                            unsafe { (*p_err_1).rc = 1 };
                            unsafe {
                                (*p_err_1).z_err =
                                    unsafe {
                                        sqlite3_mprintf(c"Cannot discern type: \"%s\"".as_ptr() as
                                                    *mut i8 as *const i8, z_name)
                                    }
                            };
                            p_stmt = core::ptr::null_mut();
                        }
                    }
                }
                break '__c7;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return p_stmt;
}

unsafe extern "C" fn execsql_i64_x(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    mut __va0: ...) -> i64 {
    let mut i_ret: i64 = 0 as i64;
    if unsafe { (*p_err_1).rc } == 0 {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        p_stmt = get_and_bind_sql_statement(p_err_1, p_db_1, ap as *const i8);
        if !(p_stmt).is_null() {
            let mut first: i32 = 1;
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                if first != 0 && unsafe { sqlite3_column_count(p_stmt) } > 0 {
                    i_ret = unsafe { sqlite3_column_int64(p_stmt, 0) };
                }
                first = 0;
            }
            if 0 != unsafe { sqlite3_reset(p_stmt) } {
                sqlite_error(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                    c"reset".as_ptr() as *mut i8 as *const i8);
            }
        }
        ();
    }
    return i_ret;
}

unsafe extern "C" fn execsql_text_x(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i_slot_1: i32, mut __va0: ...) -> *mut i8 {
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    if i_slot_1 >= unsafe { (*p_db_1).n_text } {
        let n_byte: i32 =
            (core::mem::size_of::<*mut i8>() as u64 * (i_slot_1 + 1) as u64)
                as i32;
        unsafe {
            (*p_db_1).a_text =
                unsafe {
                        sqlite3_realloc(unsafe { (*p_db_1).a_text } as *mut (),
                            n_byte)
                    } as *mut *mut i8
        };
        unsafe {
            memset(unsafe {
                        &raw mut *unsafe {
                                    (*p_db_1).a_text.offset(unsafe { (*p_db_1).n_text } as
                                            isize)
                                }
                    } as *mut (), 0,
                core::mem::size_of::<*mut i8>() as u64 *
                    (i_slot_1 + 1 - unsafe { (*p_db_1).n_text }) as u64)
        };
        unsafe { (*p_db_1).n_text = i_slot_1 + 1 };
    }
    if unsafe { (*p_err_1).rc } == 0 {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        p_stmt = get_and_bind_sql_statement(p_err_1, p_db_1, ap as *const i8);
        if !(p_stmt).is_null() {
            let mut first: i32 = 1;
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                if first != 0 && unsafe { sqlite3_column_count(p_stmt) } > 0 {
                    z_ret =
                        unsafe {
                            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_column_text(p_stmt, 0) })
                        };
                    unsafe {
                        sqlite3_free(unsafe {
                                    *unsafe { (*p_db_1).a_text.offset(i_slot_1 as isize) }
                                } as *mut ())
                    };
                    unsafe {
                        *unsafe { (*p_db_1).a_text.offset(i_slot_1 as isize) } =
                            z_ret
                    };
                }
                first = 0;
            }
            if 0 != unsafe { sqlite3_reset(p_stmt) } {
                sqlite_error(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                    c"reset".as_ptr() as *mut i8 as *const i8);
            }
        }
        ();
    }
    return z_ret;
}

extern "C" fn integrity_check_x(p_err_1: *mut Error, p_db_1: *mut Sqlite)
    -> () {
    if unsafe { (*p_err_1).rc } == 0 {
        let mut p_statement: *const Statement = core::ptr::null();
        let mut z_err: *mut i8 = core::ptr::null_mut();
        p_statement =
            get_sql_statement(p_err_1, p_db_1,
                c"PRAGMA integrity_check".as_ptr() as *mut i8 as *const i8);
        if !(p_statement).is_null() {
            let p_stmt: *mut Sqlite3Stmt = unsafe { (*p_statement).p_stmt };
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                let z: *const i8 =
                    unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
                if unsafe {
                            strcmp(z, c"ok".as_ptr() as *mut i8 as *const i8)
                        } != 0 {
                    if z_err == core::ptr::null_mut() {
                        z_err =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z)
                            };
                    } else {
                        z_err =
                            unsafe {
                                sqlite3_mprintf(c"%z\n%s".as_ptr() as *mut i8 as *const i8,
                                    z_err, z)
                            };
                    }
                }
            }
            unsafe { sqlite3_reset(p_stmt) };
            if !(z_err).is_null() {
                unsafe { (*p_err_1).z_err = z_err };
                unsafe { (*p_err_1).rc = 1 };
            }
        }
    }
}

extern "C" fn launch_thread_main(p_arg_1: *mut ()) -> *mut () {
    let p: *mut Thread = p_arg_1 as *mut Thread;
    unsafe {
        (*p).z_res =
            unsafe {
                (unsafe {
                        (*p).x_proc.unwrap()
                    })(unsafe { (*p).i_tid }, unsafe { (*p).p_arg })
            }
    };
    return core::ptr::null_mut();
}

extern "C" fn launch_thread_x(p_err_1: *mut Error,
    p_threads_1: &mut Threadset,
    x_proc_1: Option<unsafe extern "C" fn(i32, *mut ()) -> *mut i8>,
    p_arg_1: *mut ()) -> () {
    if unsafe { (*p_err_1).rc } == 0 {
        let i_tid: i32 =
            { let __p = &mut (*p_threads_1).i_max_tid; *__p += 1; *__p };
        let mut p: *mut Thread = core::ptr::null_mut();
        let mut rc: i32 = 0;
        p =
            unsafe { sqlite3_malloc(core::mem::size_of::<Thread>() as i32) }
                as *mut Thread;
        unsafe {
            memset(p as *mut (), 0, core::mem::size_of::<Thread>() as u64)
        };
        unsafe { (*p).i_tid = i_tid };
        unsafe { (*p).p_arg = p_arg_1 };
        unsafe { (*p).x_proc = x_proc_1 };
        rc =
            unsafe {
                pthread_create(unsafe { &mut (*p).tid },
                    0 as *mut () as *const PthreadAttrT, launch_thread_main,
                    p as *mut ())
            };
        if rc != 0 {
            system_error(unsafe { &mut *p_err_1 }, rc);
            unsafe { sqlite3_free(p as *mut ()) };
        } else {
            unsafe { (*p).p_next = (*p_threads_1).p_thread };
            (*p_threads_1).p_thread = p;
        }
    }
}

extern "C" fn join_all_threads_x(p_err_1: *mut Error,
    p_threads_1: &mut Threadset) -> () {
    unsafe {
        let mut p: *mut Thread = core::ptr::null_mut();
        let mut p_next: *mut Thread = core::ptr::null_mut();
        {
            p = (*p_threads_1).p_thread;
            '__b12: loop {
                if !(!(p).is_null()) { break '__b12; }
                '__c12: loop {
                    let mut ret: *mut () = core::ptr::null_mut();
                    let mut rc: i32 = 0;
                    p_next = unsafe { (*p).p_next };
                    rc = unsafe { pthread_join(unsafe { (*p).tid }, &mut ret) };
                    if rc != 0 {
                        if unsafe { (*p_err_1).rc } == 0 {
                            system_error(unsafe { &mut *p_err_1 }, rc);
                        }
                    } else {
                        unsafe {
                            printf(c"Thread %d says: %s\n".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*p).i_tid },
                                if unsafe { (*p).z_res } == core::ptr::null_mut() {
                                    c"...".as_ptr() as *mut i8
                                } else { unsafe { (*p).z_res } })
                        };
                        unsafe { fflush(__stdoutp) };
                    }
                    unsafe { sqlite3_free(unsafe { (*p).z_res } as *mut ()) };
                    unsafe { sqlite3_free(p as *mut ()) };
                    break '__c12;
                }
                p = p_next;
            }
        }
        (*p_threads_1).p_thread = core::ptr::null_mut();
    }
}

extern "C" fn filesize_x(p_err_1: &Error, z_file_1: *const i8) -> i64 {
    let mut i_ret: i64 = 0 as i64;
    if (*p_err_1).rc == 0 {
        let mut s_stat: Stat = unsafe { core::mem::zeroed() };
        if unsafe { stat(z_file_1, &mut s_stat) } != 0 {
            i_ret = -1 as i64;
        } else { i_ret = s_stat.st_size; }
    }
    return i_ret;
}

extern "C" fn filecopy_x(p_err_1: *mut Error, z_from_1: *const i8,
    z_to_1: *const i8) -> () {
    if unsafe { (*p_err_1).rc } == 0 {
        let n_byte: i64 = filesize_x(unsafe { &*p_err_1 }, z_from_1);
        if n_byte < 0 as i64 {
            test_error_x(unsafe { &mut *p_err_1 },
                unsafe {
                    sqlite3_mprintf(c"no such file: %s".as_ptr() as *mut i8 as
                            *const i8, z_from_1)
                });
        } else {
            let mut i_off: i64 = 0 as i64;
            let mut a_buf: [i8; 1024] = [0; 1024];
            let mut fd1: i32 = 0;
            let mut fd2: i32 = 0;
            unsafe { unlink(z_to_1) };
            fd1 = unsafe { open(z_from_1, 0 | 0) };
            if fd1 < 0 {
                system_error(unsafe { &mut *p_err_1 },
                    unsafe { *unsafe { __error() } });
                return;
            }
            fd2 = unsafe { open(z_to_1, 2 | 512 | 2048 | 0, 420) };
            if fd2 < 0 {
                system_error(unsafe { &mut *p_err_1 },
                    unsafe { *unsafe { __error() } });
                unsafe { close(fd1) };
                return;
            }
            i_off = 0 as i64;
            while i_off < n_byte {
                let mut n_copy: i32 =
                    core::mem::size_of::<[i8; 1024]>() as i32;
                if n_copy as i64 + i_off > n_byte {
                    n_copy = (n_byte - i_off) as i32;
                }
                if n_copy as i64 !=
                        unsafe {
                            read(fd1, &raw mut a_buf[0 as usize] as *mut i8 as *mut (),
                                n_copy as u64)
                        } {
                    system_error(unsafe { &mut *p_err_1 },
                        unsafe { *unsafe { __error() } });
                    break;
                }
                if n_copy as i64 !=
                        unsafe {
                            write(fd2,
                                &raw mut a_buf[0 as usize] as *mut i8 as *const (),
                                n_copy as u64)
                        } {
                    system_error(unsafe { &mut *p_err_1 },
                        unsafe { *unsafe { __error() } });
                    break;
                }
                i_off += n_copy as i64;
            }
            unsafe { close(fd1) };
            unsafe { close(fd2) };
        }
    }
}

static mut timelimit: f64 = 0.0;

extern "C" fn current_time() -> f64 {
    unsafe {
        let mut t: f64 = 0.0;
        if p_timelimit_vfs == core::ptr::null_mut() {
            p_timelimit_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        }
        if unsafe { (*p_timelimit_vfs).i_version } >= 2 &&
                unsafe { (*p_timelimit_vfs).x_current_time_int64.is_some() } {
            let mut tm: Sqlite3Int64 = 0 as Sqlite3Int64;
            unsafe {
                (unsafe {
                        (*p_timelimit_vfs).x_current_time_int64.unwrap()
                    })(p_timelimit_vfs, &mut tm)
            };
            t = tm as f64 / 86400000.0;
        } else {
            unsafe {
                (unsafe {
                        (*p_timelimit_vfs).x_current_time.unwrap()
                    })(p_timelimit_vfs, &mut t)
            };
        }
        return t;
    }
}

extern "C" fn setstoptime_x(p_err_1: &Error, n_ms_1: i32) -> () {
    unsafe {
        if (*p_err_1).rc == 0 {
            let t: f64 = current_time();
            timelimit = t + n_ms_1 as f64 / (1000.0 * 60.0 * 60.0 * 24.0);
        }
    }
}

extern "C" fn timetostop_x(p_err_1: &Error) -> i32 {
    unsafe {
        let mut ret: i32 = 1;
        if (*p_err_1).rc == 0 {
            let t: f64 = current_time();
            ret = (t >= timelimit) as i32;
        }
        return ret;
    }
}

extern "C" fn walthread1_thread(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut n_iter: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 988 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 989 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let az_sql: [*const i8; 2] =
            [c"SELECT md5sum(x) FROM t1 WHERE rowid != (SELECT max(rowid) FROM t1)".as_ptr()
                        as *const i8,
                    c"SELECT x FROM t1 WHERE rowid = (SELECT max(rowid) FROM t1)".as_ptr()
                        as *const i8];
        let mut z1: *mut i8 = core::ptr::null_mut();
        let mut z2: *mut i8 = core::ptr::null_mut();
        let mut z3: *mut i8 = core::ptr::null_mut();
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 996 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"BEGIN".as_ptr() as *mut i8)
                    };
            }
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 997 }
            };
            integrity_check_x(&mut err, &mut db)
        };
        z1 =
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 998 }
                };
                unsafe {
                    execsql_text_x(&mut err, &mut db, 1, az_sql[0 as usize])
                }
            };
        z2 =
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 999 }
                };
                unsafe {
                    execsql_text_x(&mut err, &mut db, 2, az_sql[1 as usize])
                }
            };
        z3 =
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 1000 }
                };
                unsafe {
                    execsql_text_x(&mut err, &mut db, 3, az_sql[0 as usize])
                }
            };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1001 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"COMMIT".as_ptr() as *mut i8)
                    };
            }
        };
        if unsafe { strcmp(z1 as *const i8, z2 as *const i8) } != 0 ||
                unsafe { strcmp(z1 as *const i8, z3 as *const i8) } != 0 {
            test_error_x(&mut err,
                unsafe {
                    sqlite3_mprintf(c"Failed read: %s %s %s".as_ptr() as *mut i8
                            as *const i8, z1, z2, z3)
                });
        }
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1013 }
            };
            sql_script_x(&mut err, &db,
                c"BEGIN;INSERT INTO t1 VALUES(randomblob(100));INSERT INTO t1 VALUES(randomblob(100));INSERT INTO t1 SELECT md5sum(x) FROM t1;COMMIT;".as_ptr()
                        as *mut i8 as *const i8)
        };
        { let __p = &mut n_iter; let __t = *__p; *__p += 1; __t };
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1016 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"%d iterations".as_ptr() as *mut i8 as *const i8,
                n_iter)
        };
}

extern "C" fn walthread1_ckpt_thread(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut n_ckpt: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1027 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1028 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        unsafe { sqlite3_sleep(500) };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1030 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"PRAGMA wal_checkpoint".as_ptr() as *mut i8)
                    };
            }
        };
        if err.rc == 0 {
            { let __p = &mut n_ckpt; let __t = *__p; *__p += 1; __t };
        }
        clear_error_x(&mut err, 5);
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1034 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"%d checkpoints".as_ptr() as *mut i8 as
                    *const i8, n_ckpt)
        };
}

extern "C" fn walthread1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    let mut i: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1046 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1053 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA journal_mode = WAL;CREATE TABLE t1(x PRIMARY KEY);INSERT INTO t1 VALUES(randomblob(100));INSERT INTO t1 VALUES(randomblob(100));INSERT INTO t1 SELECT md5sum(x) FROM t1;".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1054 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1056 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    {
        i = 0;
        '__b16: loop {
            if !(i < 10) { break '__b16; }
            '__c16: loop {
                {
                    unsafe {
                        (*&mut err).i_line =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 1058 }
                    };
                    launch_thread_x(&mut err, &mut threads,
                        Some(walthread1_thread), core::ptr::null_mut())
                };
                break '__c16;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1060 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread1_ckpt_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1061 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    print_and_free_err(&mut err);
}

extern "C" fn walthread2_thread(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut an_trans: [i32; 2] = [0, 0];
    let i_arg: i32 = p_arg_1 as i64 as i32;
    let mut z_journal: *const i8 =
        c"PRAGMA journal_mode = WAL".as_ptr() as *mut i8 as *const i8;
    if i_arg != 0 {
        z_journal =
            c"PRAGMA journal_mode = DELETE".as_ptr() as *mut i8 as *const i8;
    }
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1075 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut journal_exists: i32 = 0;
        let mut wal_exists: i32 = 0;
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1079 }
            };
            opendb_x(&mut err, &mut db,
                c"test.db".as_ptr() as *mut i8 as *const i8, 0)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1081 }
            };
            sql_script_x(&mut err, &db, z_journal)
        };
        clear_error_x(&mut err, 5);
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1083 }
            };
            sql_script_x(&mut err, &db,
                c"BEGIN".as_ptr() as *mut i8 as *const i8)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1084 }
            };
            sql_script_x(&mut err, &db,
                c"INSERT INTO t1 VALUES(NULL, randomblob(100))".as_ptr() as
                        *mut i8 as *const i8)
        };
        journal_exists =
            ({
                        ({
                                let __v =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1086 };
                                unsafe { (*&mut err).i_line = __v };
                                __v
                            }) as i64;
                        filesize_x(&err,
                            c"test.db-journal".as_ptr() as *mut i8 as *const i8)
                    } >= 0 as i64) as i32;
        wal_exists =
            ({
                        ({
                                let __v =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1087 };
                                unsafe { (*&mut err).i_line = __v };
                                __v
                            }) as i64;
                        filesize_x(&err,
                            c"test.db-wal".as_ptr() as *mut i8 as *const i8)
                    } >= 0 as i64) as i32;
        if journal_exists + wal_exists != 1 {
            test_error_x(&mut err,
                unsafe {
                    sqlite3_mprintf(c"File system looks incorrect (%d, %d)".as_ptr()
                                as *mut i8 as *const i8, journal_exists, wal_exists)
                });
        }
        {
            let __p = &mut an_trans[journal_exists as usize];
            let __t = *__p;
            *__p += 1;
            __t
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1095 }
            };
            sql_script_x(&mut err, &db,
                c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1096 }
            };
            integrity_check_x(&mut err, &mut db)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1097 }
            };
            closedb_x(&mut err, &mut db)
        };
    }
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"W %d R %d".as_ptr() as *mut i8 as *const i8,
                an_trans[0 as usize], an_trans[1 as usize])
        };
}

extern "C" fn walthread2(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1109 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1110 }
        };
        sql_script_x(&mut err, &db,
            c"CREATE TABLE t1(x INTEGER PRIMARY KEY, y UNIQUE)".as_ptr() as
                    *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1111 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1113 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1114 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread2_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1115 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread2_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1116 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread2_thread),
            1 as *mut ())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1117 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread2_thread),
            1 as *mut ())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1118 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    print_and_free_err(&mut err);
}

extern "C" fn walthread3_thread(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i_next_write: i64 = 0 as i64;
    let i_arg: i32 = p_arg_1 as i64 as i32;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1129 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1130 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA wal_autocheckpoint = 10".as_ptr() as *mut i8 as
                *const i8)
    };
    i_next_write = (i_arg + 1) as i64;
    loop {
        let mut sum1: i64 = 0 as i64;
        let mut sum2: i64 = 0 as i64;
        let mut stop: i32 = 0;
        while 0 ==
                {
                    stop =
                        {
                            unsafe {
                                (*&mut err).i_line =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1138 }
                            };
                            timetostop_x(&err)
                        };
                    stop
                } {
            let i_max: i64 =
                {
                    ({
                            let __v =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1139 };
                            unsafe { (*&mut err).i_line = __v };
                            __v
                        }) as i64;
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"SELECT max(cnt) FROM t1".as_ptr() as *mut i8)
                    }
                };
            if i_max + 1 as i64 == i_next_write { break; }
        }
        if stop != 0 { break; }
        sum1 =
            {
                ({
                        let __v =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 1144 };
                        unsafe { (*&mut err).i_line = __v };
                        __v
                    }) as i64;
                unsafe {
                    execsql_i64_x(&mut err, &mut db,
                        c"SELECT sum(cnt) FROM t1".as_ptr() as *mut i8)
                }
            };
        sum2 =
            {
                ({
                        let __v =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 1145 };
                        unsafe { (*&mut err).i_line = __v };
                        __v
                    }) as i64;
                unsafe {
                    execsql_i64_x(&mut err, &mut db,
                        c"SELECT sum(sum1) FROM t1".as_ptr() as *mut i8)
                }
            };
        {
            ({
                    let __v =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 1149 };
                    unsafe { (*&mut err).i_line = __v };
                    __v
                }) as i64;
            unsafe {
                execsql_i64_x(&mut err, &mut db,
                    c"INSERT INTO t1 VALUES(:iNextWrite, :iSum1, :iSum2)".as_ptr()
                        as *mut i8, &raw mut i_next_write as *mut i64,
                    &raw mut sum1 as *mut i64, &raw mut sum2 as *mut i64)
            }
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1150 }
            };
            integrity_check_x(&mut err, &mut db)
        };
        i_next_write += 6 as i64;
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1155 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return core::ptr::null_mut();
}

extern "C" fn walthread3(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    let mut i: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1166 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1173 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA journal_mode = WAL;CREATE TABLE t1(cnt PRIMARY KEY, sum1, sum2);CREATE INDEX i1 ON t1(sum1);CREATE INDEX i2 ON t1(sum2);INSERT INTO t1 VALUES(0, 0, 0);".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1174 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1176 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    {
        i = 0;
        '__b20: loop {
            if !(i < 6) { break '__b20; }
            '__c20: loop {
                {
                    unsafe {
                        (*&mut err).i_line =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 1178 }
                    };
                    launch_thread_x(&mut err, &mut threads,
                        Some(walthread3_thread), i as i64 as *mut ())
                };
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1180 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    print_and_free_err(&mut err);
}

extern "C" fn walthread4_reader_thread(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1189 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1190 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 1191 }
            };
            integrity_check_x(&mut err, &mut db)
        };
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1193 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return core::ptr::null_mut();
}

extern "C" fn walthread4_writer_thread(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i_row: i64 = 1 as i64;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1204 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1205 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA wal_autocheckpoint = 15;".as_ptr() as *mut i8 as
                *const i8)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1206 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            ({
                    let __v =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 1209 };
                    unsafe { (*&mut err).i_line = __v };
                    __v
                }) as i64;
            unsafe {
                execsql_i64_x(&mut err, &mut db,
                    c"REPLACE INTO t1 VALUES(:iRow, randomblob(300))".as_ptr()
                        as *mut i8, &raw mut i_row as *mut i64)
            }
        };
        { let __p = &mut i_row; let __t = *__p; *__p += 1; __t };
        if i_row == 10 as i64 { i_row = 0 as i64; }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1213 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return core::ptr::null_mut();
}

extern "C" fn walthread4(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1224 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1228 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA journal_mode = WAL;CREATE TABLE t1(a INTEGER PRIMARY KEY, b UNIQUE);".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1229 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1231 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1232 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(walthread4_reader_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1233 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(walthread4_writer_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1234 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    print_and_free_err(&mut err);
}

extern "C" fn walthread5_thread(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut n_row: i64 = 0 as i64;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1244 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    n_row =
        {
            ({
                    let __v =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 1245 };
                    unsafe { (*&mut err).i_line = __v };
                    __v
                }) as i64;
            unsafe {
                execsql_i64_x(&mut err, &mut db,
                    c"SELECT count(*) FROM t1".as_ptr() as *mut i8)
            }
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1246 }
        };
        closedb_x(&mut err, &mut db)
    };
    if n_row != 65536 as i64 {
        test_error_x(&mut err,
            unsafe {
                sqlite3_mprintf(c"Bad row count: %d".as_ptr() as *mut i8 as
                        *const i8, n_row as i32)
            });
    }
    print_and_free_err(&mut err);
    return core::ptr::null_mut();
}

extern "C" fn walthread5(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1257 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1282 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA wal_autocheckpoint = 0;PRAGMA page_size = 1024;PRAGMA journal_mode = WAL;CREATE TABLE t1(x);BEGIN;INSERT INTO t1 VALUES(randomblob(900));INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*     2 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*     4 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*     8 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*    16 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*    32 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*    64 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*   128 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*   256 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*   512 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*  1024 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*  2048 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*  4096 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /*  8192 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /* 16384 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /* 32768 */INSERT INTO t1 SELECT randomblob(900) FROM t1;      /* 65536 */COMMIT;".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1283 }
        };
        filecopy_x(&mut err, c"test.db".as_ptr() as *mut i8 as *const i8,
            c"test_sv.db".as_ptr() as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1284 }
        };
        filecopy_x(&mut err, c"test.db-wal".as_ptr() as *mut i8 as *const i8,
            c"test_sv.db-wal".as_ptr() as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1285 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1287 }
        };
        filecopy_x(&mut err, c"test_sv.db".as_ptr() as *mut i8 as *const i8,
            c"test.db".as_ptr() as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1288 }
        };
        filecopy_x(&mut err,
            c"test_sv.db-wal".as_ptr() as *mut i8 as *const i8,
            c"test.db-wal".as_ptr() as *mut i8 as *const i8)
    };
    if err.rc == 0 {
        unsafe {
            printf(c"  WAL file is %d bytes,".as_ptr() as *mut i8 as
                    *const i8,
                {
                        ({
                                let __v =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1291 };
                                unsafe { (*&mut err).i_line = __v };
                                __v
                            }) as i64;
                        filesize_x(&err,
                            c"test.db-wal".as_ptr() as *mut i8 as *const i8)
                    } as i32)
        };
        unsafe {
            printf(c" DB file is %d.\n".as_ptr() as *mut i8 as *const i8,
                {
                        ({
                                let __v =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1292 };
                                unsafe { (*&mut err).i_line = __v };
                                __v
                            }) as i64;
                        filesize_x(&err,
                            c"test.db".as_ptr() as *mut i8 as *const i8)
                    } as i32)
        };
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1295 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1296 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread5_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1297 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread5_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1298 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread5_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1299 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread5_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1300 }
        };
        launch_thread_x(&mut err, &mut threads, Some(walthread5_thread),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1301 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    if err.rc == 0 {
        unsafe {
            printf(c"  WAL file is %d bytes,".as_ptr() as *mut i8 as
                    *const i8,
                {
                        ({
                                let __v =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1304 };
                                unsafe { (*&mut err).i_line = __v };
                                __v
                            }) as i64;
                        filesize_x(&err,
                            c"test.db-wal".as_ptr() as *mut i8 as *const i8)
                    } as i32)
        };
        unsafe {
            printf(c" DB file is %d.\n".as_ptr() as *mut i8 as *const i8,
                {
                        ({
                                let __v =
                                    if unsafe { (*&mut err).rc } != 0 {
                                        unsafe { (*&mut err).i_line }
                                    } else { 1305 };
                                unsafe { (*&mut err).i_line = __v };
                                __v
                            }) as i64;
                        filesize_x(&err,
                            c"test.db".as_ptr() as *mut i8 as *const i8)
                    } as i32)
        };
    }
    print_and_free_err(&mut err);
}

extern "C" fn cgt_pager_1_populate(p_err_1: *mut Error, p_db_1: *mut Sqlite)
    -> () {
    let z_insert: *const i8 =
        c"INSERT INTO t1 VALUES(:iRow, zeroblob(:iBlob))".as_ptr() as *mut i8
            as *const i8;
    let mut i_row: i64 = 0 as i64;
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 1318 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"BEGIN".as_ptr() as *mut i8 as *const i8)
    };
    {
        i_row = 1 as i64;
        '__b23: loop {
            if !(i_row <= 10000 as i64) { break '__b23; }
            '__c23: loop {
                let mut i_blob: i64 = 600 as i64 + i_row % 300 as i64;
                {
                    unsafe {
                        (*p_err_1).i_line =
                            if unsafe { (*p_err_1).rc } != 0 {
                                unsafe { (*p_err_1).i_line }
                            } else { 1321 }
                    };
                    {
                        let _ =
                            unsafe {
                                execsql_i64_x(p_err_1, p_db_1, z_insert,
                                    &raw mut i_row as *mut i64, &raw mut i_blob as *mut i64)
                            };
                    }
                };
                break '__c23;
            }
            { let __p = &mut i_row; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 1323 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"COMMIT".as_ptr() as *mut i8 as *const i8)
    };
}

extern "C" fn cgt_pager_1_update(p_err_1: *mut Error, p_db_1: *mut Sqlite)
    -> () {
    let z_update: *const i8 =
        c"UPDATE t1 SET b = zeroblob(:iBlob) WHERE a = :iRow".as_ptr() as
                *mut i8 as *const i8;
    let mut i_row: i64 = 0 as i64;
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 1328 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"BEGIN".as_ptr() as *mut i8 as *const i8)
    };
    {
        i_row = 1 as i64;
        '__b24: loop {
            if !(i_row <= 10000 as i64) { break '__b24; }
            '__c24: loop {
                let mut i_blob: i64 =
                    600 as i64 + (i_row + 100 as i64) % 300 as i64;
                {
                    unsafe {
                        (*p_err_1).i_line =
                            if unsafe { (*p_err_1).rc } != 0 {
                                unsafe { (*p_err_1).i_line }
                            } else { 1331 }
                    };
                    {
                        let _ =
                            unsafe {
                                execsql_i64_x(p_err_1, p_db_1, z_update,
                                    &raw mut i_blob as *mut i64, &raw mut i_row as *mut i64)
                            };
                    }
                };
                break '__c24;
            }
            { let __p = &mut i_row; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 1333 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"COMMIT".as_ptr() as *mut i8 as *const i8)
    };
}

extern "C" fn cgt_pager_1_read(p_err_1: *mut Error, p_db_1: *mut Sqlite)
    -> () {
    let mut i_row: i64 = 0 as i64;
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 1337 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"BEGIN".as_ptr() as *mut i8 as *const i8)
    };
    {
        i_row = 1 as i64;
        '__b25: loop {
            if !(i_row <= 10000 as i64) { break '__b25; }
            '__c25: loop {
                {
                    unsafe {
                        (*p_err_1).i_line =
                            if unsafe { (*p_err_1).rc } != 0 {
                                unsafe { (*p_err_1).i_line }
                            } else { 1339 }
                    };
                    {
                        let _ =
                            unsafe {
                                execsql_i64_x(p_err_1, p_db_1,
                                    c"SELECT * FROM t1 WHERE a = :iRow".as_ptr() as *mut i8,
                                    &raw mut i_row as *mut i64)
                            };
                    }
                };
                break '__c25;
            }
            { let __p = &mut i_row; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 1341 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"COMMIT".as_ptr() as *mut i8 as *const i8)
    };
}

extern "C" fn cgt_pager_1(n_ms_1: i32) -> () {
    let mut x_sub:
            Option<unsafe extern "C" fn(*mut Error, *mut Sqlite) -> ()> =
        None;
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1348 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1353 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA cache_size = 2000;PRAGMA page_size = 1024;CREATE TABLE t1(a INTEGER PRIMARY KEY, b BLOB);".as_ptr()
                    as *mut i8 as *const i8)
    };
    x_sub = Some(cgt_pager_1_populate);
    unsafe { x_sub.unwrap()(&mut err, &mut db) };
    x_sub = Some(cgt_pager_1_update);
    unsafe { x_sub.unwrap()(&mut err, &mut db) };
    x_sub = Some(cgt_pager_1_read);
    unsafe { x_sub.unwrap()(&mut err, &mut db) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1359 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
}

extern "C" fn dynamic_triggers_1(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut n_drop: i32 = 0;
    let mut n_create: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1377 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1378 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut i: i32 = 0;
        {
            i = 1;
            '__b27: loop {
                if !(i < 9) { break '__b27; }
                '__c27: loop {
                    let z_sql: *mut i8 =
                        unsafe {
                            sqlite3_mprintf(c"CREATE TRIGGER itr%d BEFORE INSERT ON t%d BEGIN INSERT INTO t%d VALUES(new.x, new.y);END;".as_ptr()
                                        as *mut i8 as *const i8, i, i, i + 1)
                        };
                    {
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1387 }
                        };
                        {
                            let _ = unsafe { execsql_i64_x(&mut err, &mut db, z_sql) };
                        }
                    };
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    { let __p = &mut n_create; let __t = *__p; *__p += 1; __t };
                    break '__c27;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 1;
            '__b28: loop {
                if !(i < 9) { break '__b28; }
                '__c28: loop {
                    let z_sql_1: *mut i8 =
                        unsafe {
                            sqlite3_mprintf(c"CREATE TRIGGER dtr%d BEFORE DELETE ON t%d BEGIN DELETE FROM t%d WHERE x = old.x; END;".as_ptr()
                                        as *mut i8 as *const i8, i, i, i + 1)
                        };
                    {
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1398 }
                        };
                        {
                            let _ =
                                unsafe { execsql_i64_x(&mut err, &mut db, z_sql_1) };
                        }
                    };
                    unsafe { sqlite3_free(z_sql_1 as *mut ()) };
                    { let __p = &mut n_create; let __t = *__p; *__p += 1; __t };
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 1;
            '__b29: loop {
                if !(i < 9) { break '__b29; }
                '__c29: loop {
                    let z_sql_2: *mut i8 =
                        unsafe {
                            sqlite3_mprintf(c"DROP TRIGGER itr%d".as_ptr() as *mut i8 as
                                    *const i8, i)
                        };
                    {
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1405 }
                        };
                        {
                            let _ =
                                unsafe { execsql_i64_x(&mut err, &mut db, z_sql_2) };
                        }
                    };
                    unsafe { sqlite3_free(z_sql_2 as *mut ()) };
                    { let __p = &mut n_drop; let __t = *__p; *__p += 1; __t };
                    break '__c29;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 1;
            '__b30: loop {
                if !(i < 9) { break '__b30; }
                '__c30: loop {
                    let z_sql_3: *mut i8 =
                        unsafe {
                            sqlite3_mprintf(c"DROP TRIGGER dtr%d".as_ptr() as *mut i8 as
                                    *const i8, i)
                        };
                    {
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1412 }
                        };
                        {
                            let _ =
                                unsafe { execsql_i64_x(&mut err, &mut db, z_sql_3) };
                        }
                    };
                    unsafe { sqlite3_free(z_sql_3 as *mut ()) };
                    { let __p = &mut n_drop; let __t = *__p; *__p += 1; __t };
                    break '__c30;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1417 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"%d created, %d dropped".as_ptr() as *mut i8 as
                    *const i8, n_create, n_drop)
        };
}

extern "C" fn dynamic_triggers_2(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i_val: i64 = 0 as i64;
    let mut n_insert: i32 = 0;
    let mut n_delete: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1430 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 1431 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        '__b32: loop {
            '__c32: loop {
                i_val = (i_val + 1 as i64) % 100 as i64;
                {
                    unsafe {
                        (*&mut err).i_line =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 1434 }
                    };
                    {
                        let _ =
                            unsafe {
                                execsql_i64_x(&mut err, &mut db,
                                    c"INSERT INTO t1 VALUES(:iX, :iY+1)".as_ptr() as *mut i8,
                                    &raw mut i_val as *mut i64, &raw mut i_val as *mut i64)
                            };
                    }
                };
                { let __p = &mut n_insert; let __t = *__p; *__p += 1; __t };
                break '__c32;
            }
            if !(i_val != 0) { break '__b32; }
        }
        '__b33: loop {
            '__c33: loop {
                i_val = (i_val + 1 as i64) % 100 as i64;
                {
                    unsafe {
                        (*&mut err).i_line =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 1440 }
                    };
                    {
                        let _ =
                            unsafe {
                                execsql_i64_x(&mut err, &mut db,
                                    c"DELETE FROM t1 WHERE x = :iX".as_ptr() as *mut i8,
                                    &raw mut i_val as *mut i64)
                            };
                    }
                };
                { let __p = &mut n_delete; let __t = *__p; *__p += 1; __t };
                break '__c33;
            }
            if !(i_val != 0) { break '__b33; }
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1444 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"%d inserts, %d deletes".as_ptr() as *mut i8 as
                    *const i8, n_insert, n_delete)
        };
}

extern "C" fn dynamic_triggers(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1455 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1468 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA page_size = 1024;PRAGMA journal_mode = WAL;CREATE TABLE t1(x, y);CREATE TABLE t2(x, y);CREATE TABLE t3(x, y);CREATE TABLE t4(x, y);CREATE TABLE t5(x, y);CREATE TABLE t6(x, y);CREATE TABLE t7(x, y);CREATE TABLE t8(x, y);CREATE TABLE t9(x, y);".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1469 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1471 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1474 }
        };
        launch_thread_x(&mut err, &mut threads, Some(dynamic_triggers_2),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1475 }
        };
        launch_thread_x(&mut err, &mut threads, Some(dynamic_triggers_2),
            core::ptr::null_mut())
    };
    unsafe { sqlite3_sleep(2 * 1000) };
    unsafe { sqlite3_enable_shared_cache(0) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1480 }
        };
        launch_thread_x(&mut err, &mut threads, Some(dynamic_triggers_2),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1481 }
        };
        launch_thread_x(&mut err, &mut threads, Some(dynamic_triggers_1),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 1483 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    print_and_free_err(&mut err);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CheckpointStarvationCtx {
    e_mode: i32,
    n_max_frame: i32,
}

extern "C" fn checkpoint_starvation_walhook(p_ctx_1: *mut (),
    db: *mut Sqlite3, z_db_1: *const i8, n_frame_1: i32) -> i32 {
    let p: *mut CheckpointStarvationCtx =
        p_ctx_1 as *mut CheckpointStarvationCtx;
    if n_frame_1 > unsafe { (*p).n_max_frame } {
        unsafe { (*p).n_max_frame = n_frame_1 };
    }
    if n_frame_1 >= 50 {
        unsafe {
            sqlite3_wal_checkpoint_v2(db, z_db_1, unsafe { (*p).e_mode },
                core::ptr::null_mut(), core::ptr::null_mut())
        };
    }
    return 0;
}

extern "C" fn checkpoint_starvation_reader(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 73 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 74 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut i_count1: i64 = 0 as i64;
        let mut i_count2: i64 = 0 as i64;
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 76 }
            };
            sql_script_x(&mut err, &db,
                c"BEGIN".as_ptr() as *mut i8 as *const i8)
        };
        i_count1 =
            {
                ({
                        let __v =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 77 };
                        unsafe { (*&mut err).i_line = __v };
                        __v
                    }) as i64;
                unsafe {
                    execsql_i64_x(&mut err, &mut db,
                        c"SELECT count(x) FROM t1".as_ptr() as *mut i8)
                }
            };
        unsafe { sqlite3_sleep(100) };
        i_count2 =
            {
                ({
                        let __v =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 79 };
                        unsafe { (*&mut err).i_line = __v };
                        __v
                    }) as i64;
                unsafe {
                    execsql_i64_x(&mut err, &mut db,
                        c"SELECT count(x) FROM t1".as_ptr() as *mut i8)
                }
            };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 80 }
            };
            sql_script_x(&mut err, &db,
                c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        if i_count1 != i_count2 {
            test_error_x(&mut err,
                unsafe {
                    sqlite3_mprintf(c"Isolation failure - %lld %lld".as_ptr() as
                                *mut i8 as *const i8, i_count1, i_count2)
                });
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 86 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return core::ptr::null_mut();
}

extern "C" fn checkpoint_starvation_main(n_ms_1: i32,
    p: *mut CheckpointStarvationCtx) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    let mut n_insert: i32 = 0;
    let mut i: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 99 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 104 }
        };
        sql_script_x(&mut err, &db,
            c"PRAGMA page_size = 1024;PRAGMA journal_mode = WAL;CREATE TABLE t1(x);".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 106 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    {
        i = 0;
        '__b35: loop {
            if !(i < 4) { break '__b35; }
            '__c35: loop {
                {
                    unsafe {
                        (*&mut err).i_line =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 109 }
                    };
                    launch_thread_x(&mut err, &mut threads,
                        Some(checkpoint_starvation_reader), core::ptr::null_mut())
                };
                unsafe { sqlite3_sleep(100 / 4) };
                break '__c35;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        sqlite3_wal_hook(db.db, Some(checkpoint_starvation_walhook),
            p as *mut ())
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 114 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 115 }
            };
            sql_script_x(&mut err, &db,
                c"INSERT INTO t1 VALUES(randomblob(1200))".as_ptr() as *mut i8
                    as *const i8)
        };
        { let __p = &mut n_insert; let __t = *__p; *__p += 1; __t };
    }
    unsafe {
        printf(c" Checkpoint mode  : %s\n".as_ptr() as *mut i8 as *const i8,
            if unsafe { (*p).e_mode } == 0 {
                c"PASSIVE".as_ptr() as *mut i8
            } else { c"RESTART".as_ptr() as *mut i8 })
    };
    unsafe {
        printf(c" Peak WAL         : %d frames\n".as_ptr() as *mut i8 as
                *const i8, unsafe { (*p).n_max_frame })
    };
    unsafe {
        printf(c" Transaction count: %d transactions\n".as_ptr() as *mut i8 as
                *const i8, n_insert)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 125 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 126 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
}

extern "C" fn checkpoint_starvation_1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut ctx: CheckpointStarvationCtx =
        CheckpointStarvationCtx { e_mode: 0, n_max_frame: 0 };
    checkpoint_starvation_main(n_ms_1, &mut ctx);
    if ctx.n_max_frame < 50 * 10 {
        test_error_x(&mut err,
            unsafe {
                sqlite3_mprintf(c"WAL failed to grow - %d frames".as_ptr() as
                            *mut i8 as *const i8, ctx.n_max_frame)
            });
    }
    print_and_free_err(&mut err);
}

extern "C" fn checkpoint_starvation_2(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut ctx: CheckpointStarvationCtx =
        CheckpointStarvationCtx { e_mode: 2, n_max_frame: 0 };
    checkpoint_starvation_main(n_ms_1, &mut ctx);
    if ctx.n_max_frame > 50 + 10 {
        test_error_x(&mut err,
            unsafe {
                sqlite3_mprintf(c"WAL grew too large - %d frames".as_ptr() as
                            *mut i8 as *const i8, ctx.n_max_frame)
            });
    }
    print_and_free_err(&mut err);
}

extern "C" fn create_drop_index_thread(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 21 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 22 }
            };
            opendb_x(&mut err, &mut db,
                c"test.db".as_ptr() as *mut i8 as *const i8, 0)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 39 }
            };
            sql_script_x(&mut err, &db,
                c"DROP INDEX IF EXISTS i1;DROP INDEX IF EXISTS i2;DROP INDEX IF EXISTS i3;DROP INDEX IF EXISTS i4;CREATE INDEX IF NOT EXISTS i1 ON t11(a);CREATE INDEX IF NOT EXISTS i2 ON t11(b);CREATE INDEX IF NOT EXISTS i3 ON t11(c);CREATE INDEX IF NOT EXISTS i4 ON t11(d);SELECT * FROM t11 ORDER BY a;SELECT * FROM t11 ORDER BY b;SELECT * FROM t11 ORDER BY c;SELECT * FROM t11 ORDER BY d;".as_ptr()
                        as *mut i8 as *const i8)
        };
        clear_error_x(&mut err, 6);
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 42 }
            };
            closedb_x(&mut err, &mut db)
        };
    }
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn create_drop_index_1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 54 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 59 }
        };
        sql_script_x(&mut err, &db,
            c"CREATE TABLE t11(a, b, c, d);WITH data(x) AS (SELECT 1 UNION ALL SELECT x+1 FROM data WHERE x<100) INSERT INTO t11 SELECT x,x,x,x FROM data;".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 60 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 62 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 65 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(create_drop_index_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 66 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(create_drop_index_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 67 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(create_drop_index_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 68 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(create_drop_index_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 69 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(create_drop_index_thread), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 71 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    unsafe { sqlite3_enable_shared_cache(0) };
    print_and_free_err(&mut err);
}

extern "C" fn lookaside1_thread_reader(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 25 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 27 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        unsafe {
            sqlite3_prepare_v2(db.db,
                c"SELECT 1 FROM t1".as_ptr() as *mut i8 as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
        while unsafe { sqlite3_step(p_stmt) } == 100 {
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 33 }
                };
                {
                    let _ =
                        unsafe {
                            execsql_i64_x(&mut err, &mut db,
                                c"SELECT length(x||y||z) FROM t2".as_ptr() as *mut i8)
                        };
                }
            };
        }
        rc = unsafe { sqlite3_finalize(p_stmt) };
        if err.rc == 0 && rc != 0 {
            sqlite_error(&mut err, &db,
                c"finalize".as_ptr() as *mut i8 as *const i8);
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 41 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn lookaside1_thread_writer(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 50 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    '__b40: loop {
        '__c40: loop {
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 57 }
                };
                sql_script_x(&mut err, &db,
                    c"BEGIN;UPDATE t3 SET i=i+1 WHERE x=1;ROLLBACK;".as_ptr() as
                            *mut i8 as *const i8)
            };
            break '__c40;
        }
        if !(({
                                    unsafe {
                                        (*&mut err).i_line =
                                            if unsafe { (*&mut err).rc } != 0 {
                                                unsafe { (*&mut err).i_line }
                                            } else { 58 }
                                    };
                                    timetostop_x(&err)
                                } == 0) as i32 != 0) {
            break '__b40;
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 60 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn lookaside1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 71 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 84 }
        };
        sql_script_x(&mut err, &db,
            c"CREATE TABLE t1(x PRIMARY KEY) WITHOUT ROWID;WITH data(x,y) AS (  SELECT 1, quote(randomblob(750)) UNION ALL   SELECT x*2, y||y FROM data WHERE x<5) INSERT INTO t1 SELECT y FROM data;CREATE TABLE t3(x PRIMARY KEY,i) WITHOUT ROWID;INSERT INTO t3 VALUES(1, 1);CREATE TABLE t2(x,y,z);INSERT INTO t2 VALUES(randomblob(50), randomblob(50), randomblob(50));".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 85 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 87 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 90 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(lookaside1_thread_reader), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 91 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(lookaside1_thread_reader), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 92 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(lookaside1_thread_reader), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 93 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(lookaside1_thread_reader), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 94 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(lookaside1_thread_reader), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 95 }
        };
        launch_thread_x(&mut err, &mut threads,
            Some(lookaside1_thread_writer), core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 96 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    unsafe { sqlite3_enable_shared_cache(0) };
    print_and_free_err(&mut err);
}

extern "C" fn vacuum1_thread_writer(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i: i64 = 0 as i64;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 28 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 29 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 36 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"WITH loop(i) AS (SELECT 1 UNION ALL SELECT i+1 FROM loop WHERE i<100) INSERT INTO t1 SELECT randomblob(50), randomblob(2500) FROM loop".as_ptr()
                                as *mut i8)
                    };
            }
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 39 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"DELETE FROM t1 WHERE rowid = :i".as_ptr() as *mut i8,
                            &raw mut i as *mut i64)
                    };
            }
        };
        clear_error_x(&mut err, 6);
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 43 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"SELECT * FROM t1 ORDER BY x".as_ptr() as *mut i8)
                    };
            }
        };
        clear_error_x(&mut err, 6);
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 47 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn vacuum1_thread_vacuumer(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 55 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    '__b42: loop {
        '__c42: loop {
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 58 }
                };
                sql_script_x(&mut err, &db,
                    c"VACUUM".as_ptr() as *mut i8 as *const i8)
            };
            clear_error_x(&mut err, 6);
            break '__c42;
        }
        if !(({
                                    unsafe {
                                        (*&mut err).i_line =
                                            if unsafe { (*&mut err).rc } != 0 {
                                                unsafe { (*&mut err).i_line }
                                            } else { 60 }
                                    };
                                    timetostop_x(&err)
                                } == 0) as i32 != 0) {
            break '__b42;
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 62 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn vacuum1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 72 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 76 }
        };
        sql_script_x(&mut err, &db,
            c"CREATE TABLE t1(x PRIMARY KEY, y BLOB);CREATE INDEX i1 ON t1(y);".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 77 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 79 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 82 }
        };
        launch_thread_x(&mut err, &mut threads, Some(vacuum1_thread_writer),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 83 }
        };
        launch_thread_x(&mut err, &mut threads, Some(vacuum1_thread_writer),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 84 }
        };
        launch_thread_x(&mut err, &mut threads, Some(vacuum1_thread_writer),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 85 }
        };
        launch_thread_x(&mut err, &mut threads, Some(vacuum1_thread_vacuumer),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 86 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    unsafe { sqlite3_enable_shared_cache(0) };
    print_and_free_err(&mut err);
}

extern "C" fn stress_thread_1(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 24 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 25 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 26 }
            };
            sql_script_x(&mut err, &db,
                c"CREATE TABLE IF NOT EXISTS t1(a PRIMARY KEY, b)".as_ptr() as
                        *mut i8 as *const i8)
        };
        clear_error_x(&mut err, 6);
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 28 }
            };
            sql_script_x(&mut err, &db,
                c"DROP TABLE IF EXISTS t1".as_ptr() as *mut i8 as *const i8)
        };
        clear_error_x(&mut err, 6);
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 31 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn stress_thread_2(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 42 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 43 }
            };
            opendb_x(&mut err, &mut db,
                c"test.db".as_ptr() as *mut i8 as *const i8, 0)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 44 }
            };
            sql_script_x(&mut err, &db,
                c"SELECT * FROM sqlite_schema;".as_ptr() as *mut i8 as
                    *const i8)
        };
        clear_error_x(&mut err, 6);
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 46 }
            };
            closedb_x(&mut err, &mut db)
        };
    }
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

extern "C" fn stress_thread_3(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 62 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 63 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 64 }
            };
            sql_script_x(&mut err, &db,
                c"SELECT * FROM t1 ORDER BY a;".as_ptr() as *mut i8 as
                    *const i8)
        };
        { let __p = &mut i1; let __t = *__p; *__p += 1; __t };
        if err.rc != 0 {
            { let __p = &mut i2; let __t = *__p; *__p += 1; __t };
        }
        clear_error_x(&mut err, 6);
        clear_error_x(&mut err, 1);
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 70 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"read t1 %d/%d attempts".as_ptr() as *mut i8 as
                    *const i8, i2, i1)
        };
}

extern "C" fn stress_thread_4(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let i_arg: i32 = p_arg_1 as i64 as i32;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 85 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 86 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        if i_arg != 0 {
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 88 }
                };
                closedb_x(&mut err, &mut db)
            };
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 89 }
                };
                opendb_x(&mut err, &mut db,
                    c"test.db".as_ptr() as *mut i8 as *const i8, 0)
            };
        }
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 94 }
            };
            sql_script_x(&mut err, &db,
                c"WITH loop(i) AS (SELECT 1 UNION ALL SELECT i+1 FROM loop LIMIT 200) INSERT INTO t1 VALUES(randomblob(60), randomblob(60));".as_ptr()
                        as *mut i8 as *const i8)
        };
        { let __p = &mut i1; let __t = *__p; *__p += 1; __t };
        if err.rc != 0 {
            { let __p = &mut i2; let __t = *__p; *__p += 1; __t };
        }
        clear_error_x(&mut err, 6);
        clear_error_x(&mut err, 1);
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 100 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"wrote t1 %d/%d attempts".as_ptr() as *mut i8 as
                    *const i8, i2, i1)
        };
}

extern "C" fn stress_thread_5(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let i_arg: i32 = p_arg_1 as i64 as i32;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 116 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 0)
    };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 117 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut i: i64 = (i1 % 4) as i64;
        if i_arg != 0 {
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 120 }
                };
                closedb_x(&mut err, &mut db)
            };
            {
                unsafe {
                    (*&mut err).i_line =
                        if unsafe { (*&mut err).rc } != 0 {
                            unsafe { (*&mut err).i_line }
                        } else { 121 }
                };
                opendb_x(&mut err, &mut db,
                    c"test.db".as_ptr() as *mut i8 as *const i8, 0)
            };
        }
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 123 }
            };
            {
                let _ =
                    unsafe {
                        execsql_i64_x(&mut err, &mut db,
                            c"DELETE FROM t1 WHERE (rowid % 4)==:i".as_ptr() as *mut i8,
                            &raw mut i as *mut i64)
                    };
            }
        };
        { let __p = &mut i1; let __t = *__p; *__p += 1; __t };
        if err.rc != 0 {
            { let __p = &mut i2; let __t = *__p; *__p += 1; __t };
        }
        clear_error_x(&mut err, 6);
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 128 }
        };
        closedb_x(&mut err, &mut db)
    };
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"deleted from t1 %d/%d attempts".as_ptr() as
                        *mut i8 as *const i8, i2, i1)
        };
}

extern "C" fn stress1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 138 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 141 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_1),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 142 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_1),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 144 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_2),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 145 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_2),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 147 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_3),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 148 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_3),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 150 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_4),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 151 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_4),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 153 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_5),
            core::ptr::null_mut())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 154 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress_thread_5),
            1 as *mut ())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 156 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    unsafe { sqlite3_enable_shared_cache(0) };
    print_and_free_err(&mut err);
}

extern "C" fn stress2_workload1(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    let i_tab: i32 = i % (5 - 1) + 1;
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 195 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"CREATE TABLE IF NOT EXISTS t%d(x PRIMARY KEY, y, z);".as_ptr()
                        as *mut i8 as *const i8, i_tab)
        }
    };
}

extern "C" fn stress2_workload2(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    let i_tab: i32 = i % (5 - 1) + 1;
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 200 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"DROP TABLE IF EXISTS t%d;".as_ptr() as *mut i8 as *const i8,
                i_tab)
        }
    };
}

extern "C" fn stress2_workload3(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 204 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"SELECT * FROM t0 WHERE z = \'small\'".as_ptr() as *mut i8 as
                *const i8)
    };
}

extern "C" fn stress2_workload4(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 208 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"SELECT * FROM t0 WHERE z = \'big\'".as_ptr() as *mut i8 as
                *const i8)
    };
}

extern "C" fn stress2_workload5(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 214 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"INSERT INTO t0 VALUES(hex(random()), hex(randomblob(200)), \'small\');".as_ptr()
                    as *mut i8 as *const i8)
    };
}

extern "C" fn stress2_workload6(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 220 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"INSERT INTO t0 VALUES(hex(random()), hex(randomblob(57)), \'big\');".as_ptr()
                    as *mut i8 as *const i8)
    };
}

extern "C" fn stress2_workload7(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 228 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"UPDATE t0 SET y = hex(randomblob(200)) WHERE x LIKE hex((%d %% 5)) AND z=\'small\';".as_ptr()
                        as *mut i8 as *const i8, i)
        }
    };
}

extern "C" fn stress2_workload8(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 235 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"UPDATE t0 SET y = hex(randomblob(57)) WHERE x LIKE hex(%d %% 5) AND z=\'big\';".as_ptr()
                        as *mut i8 as *const i8, i)
        }
    };
}

extern "C" fn stress2_workload9(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 241 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"DELETE FROM t0 WHERE x LIKE hex(%d %% 5) AND z=\'small\';".as_ptr()
                        as *mut i8 as *const i8, i)
        }
    };
}

extern "C" fn stress2_workload10(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 246 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"DELETE FROM t0 WHERE x LIKE hex(%d %% 5) AND z=\'big\';".as_ptr()
                        as *mut i8 as *const i8, i)
        }
    };
}

extern "C" fn stress2_workload11(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 250 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"VACUUM".as_ptr() as *mut i8 as *const i8)
    };
}

extern "C" fn stress2_workload14(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 254 }
        };
        sql_script_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
            c"PRAGMA integrity_check".as_ptr() as *mut i8 as *const i8)
    };
}

extern "C" fn stress2_workload17(p_err_1: *mut Error, p_db_1: *mut Sqlite,
    i: i32) -> () {
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 260 }
        };
        unsafe {
            sql_script_printf_x(unsafe { &mut *p_err_1 }, unsafe { &*p_db_1 },
                c"PRAGMA journal_mode = %q".as_ptr() as *mut i8 as *const i8,
                if i % 2 != 0 {
                    c"delete".as_ptr() as *mut i8
                } else { c"wal".as_ptr() as *mut i8 })
        }
    };
}

extern "C" fn stress2_workload19(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let z_db: *const i8 = p_arg_1 as *const i8;
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 267 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 268 }
            };
            opendb_x(&mut err, &mut db, z_db, 0)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 269 }
            };
            sql_script_x(&mut err, &db,
                c"SELECT * FROM sqlite_schema;".as_ptr() as *mut i8 as
                    *const i8)
        };
        clear_error_x(&mut err, 6);
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 271 }
            };
            closedb_x(&mut err, &mut db)
        };
    }
    print_and_free_err(&mut err);
    return unsafe { sqlite3_mprintf(c"ok".as_ptr() as *mut i8 as *const i8) };
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stress2Ctx {
    z_db: *const i8,
    x_proc: Option<unsafe extern "C" fn(*mut Error, *mut Sqlite, i32) -> ()>,
}

extern "C" fn stress2_thread_wrapper(i_tid_1: i32, p_arg_1: *mut ())
    -> *mut i8 {
    let p_ctx: *const Stress2Ctx =
        p_arg_1 as *mut Stress2Ctx as *const Stress2Ctx;
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 291 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut cnt: i32 = 0;
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 293 }
            };
            opendb_x(&mut err, &mut db, unsafe { (*p_ctx).z_db }, 0)
        };
        {
            cnt = 0;
            '__b50: loop {
                if !(err.rc == 0 && cnt < 5) { break '__b50; }
                '__c50: loop {
                    unsafe {
                        (unsafe { (*p_ctx).x_proc.unwrap() })(&mut err, &mut db, i1)
                    };
                    i2 += (err.rc == 0) as i32;
                    clear_error_x(&mut err, 6);
                    { let __p = &mut i1; let __t = *__p; *__p += 1; __t };
                    break '__c50;
                }
                { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 300 }
            };
            closedb_x(&mut err, &mut db)
        };
    }
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"ok %d/%d".as_ptr() as *mut i8 as *const i8, i2,
                i1)
        };
}

extern "C" fn stress2_launch_thread_loop(p_err_1: *mut Error,
    p_threads_1: *mut Threadset, z_db_1: *const i8,
    x: unsafe extern "C" fn(*mut Error, *mut Sqlite, i32) -> ()) -> () {
    let p_ctx: *mut Stress2Ctx =
        unsafe { sqlite3_malloc(core::mem::size_of::<Stress2Ctx>() as i32) }
            as *mut Stress2Ctx;
    unsafe { (*p_ctx).z_db = z_db_1 };
    unsafe {
        (*p_ctx).x_proc =
            if x !=
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut Error, *mut Sqlite, i32)
                                    -> ()>(0 as *const ())
                    } {
                Some(x)
            } else { None }
    };
    {
        unsafe {
            (*p_err_1).i_line =
                if unsafe { (*p_err_1).rc } != 0 {
                    unsafe { (*p_err_1).i_line }
                } else { 316 }
        };
        launch_thread_x(p_err_1, unsafe { &mut *p_threads_1 },
            Some(stress2_thread_wrapper), p_ctx as *mut ())
    };
}

extern "C" fn stress2(n_ms_1: i32) -> () {
    let a_task: [Stress2TaskN11Stress2Task; 13] =
        [Stress2TaskN11Stress2Task { x: Some(stress2_workload1) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload2) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload3) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload4) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload5) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload6) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload7) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload8) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload9) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload10) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload11) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload14) },
                Stress2TaskN11Stress2Task { x: Some(stress2_workload17) }];
    let z_db: *const i8 = c"test.db".as_ptr() as *mut i8 as *const i8;
    let mut i: i32 = 0;
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 345 }
        };
        opendb_x(&mut err, &mut db, z_db, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 349 }
        };
        sql_script_x(&mut err, &db,
            c"CREATE TABLE IF NOT EXISTS t0(x PRIMARY KEY, y, z);CREATE INDEX IF NOT EXISTS i0 ON t0(y);".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 350 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 352 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        i = 0;
        '__b51: loop {
            if !((i as u64) <
                            core::mem::size_of::<[Stress2TaskN11Stress2Task; 13]>() as
                                    u64 /
                                core::mem::size_of::<Stress2TaskN11Stress2Task>() as u64) {
                break '__b51;
            }
            '__c51: loop {
                stress2_launch_thread_loop(&mut err, &mut threads, z_db,
                    a_task[i as
                                    usize].x.unwrap_or(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut Error, *mut Sqlite, i32)
                                        -> ()>(0 as *const ())
                        }));
                break '__c51;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 358 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress2_workload19),
            z_db as *mut ())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 359 }
        };
        launch_thread_x(&mut err, &mut threads, Some(stress2_workload19),
            z_db as *mut ())
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 361 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    unsafe { sqlite3_enable_shared_cache(0) };
    print_and_free_err(&mut err);
}

extern "C" fn shared_thread1(i_tid_1: i32, p_arg_1: *mut ()) -> *mut i8 {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    while ({
                        unsafe {
                            (*&mut err).i_line =
                                if unsafe { (*&mut err).rc } != 0 {
                                    unsafe { (*&mut err).i_line }
                                } else { 22 }
                        };
                        timetostop_x(&err)
                    } == 0) as i32 != 0 {
        let mut db: Sqlite =
            Sqlite {
                db: core::ptr::null_mut(),
                p_cache: core::ptr::null_mut(),
                n_text: 0,
                a_text: core::ptr::null_mut(),
            };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 24 }
            };
            opendb_x(&mut err, &mut db,
                c"test.db".as_ptr() as *mut i8 as *const i8, 0)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 25 }
            };
            sql_script_x(&mut err, &db,
                c"SELECT * FROM t1".as_ptr() as *mut i8 as *const i8)
        };
        {
            unsafe {
                (*&mut err).i_line =
                    if unsafe { (*&mut err).rc } != 0 {
                        unsafe { (*&mut err).i_line }
                    } else { 26 }
            };
            closedb_x(&mut err, &mut db)
        };
    }
    print_and_free_err(&mut err);
    return unsafe {
            sqlite3_mprintf(c"done!".as_ptr() as *mut i8 as *const i8)
        };
}

extern "C" fn shared1(n_ms_1: i32) -> () {
    let mut err: Error =
        Error { rc: 0, i_line: 0, z_err: core::ptr::null_mut() };
    let mut db: Sqlite =
        Sqlite {
            db: core::ptr::null_mut(),
            p_cache: core::ptr::null_mut(),
            n_text: 0,
            a_text: core::ptr::null_mut(),
        };
    let mut threads: Threadset =
        Threadset { i_max_tid: 0, p_thread: core::ptr::null_mut() };
    let mut ii: i32 = 0;
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 39 }
        };
        opendb_x(&mut err, &mut db,
            c"test.db".as_ptr() as *mut i8 as *const i8, 1)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 40 }
        };
        sql_script_x(&mut err, &db,
            c"CREATE TABLE t1(x)".as_ptr() as *mut i8 as *const i8)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 41 }
        };
        closedb_x(&mut err, &mut db)
    };
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 43 }
        };
        setstoptime_x(&err, n_ms_1)
    };
    unsafe { sqlite3_enable_shared_cache(1) };
    {
        ii = 0;
        '__b53: loop {
            if !(ii < 5) { break '__b53; }
            '__c53: loop {
                {
                    unsafe {
                        (*&mut err).i_line =
                            if unsafe { (*&mut err).rc } != 0 {
                                unsafe { (*&mut err).i_line }
                            } else { 47 }
                    };
                    launch_thread_x(&mut err, &mut threads,
                        Some(shared_thread1), core::ptr::null_mut())
                };
                break '__c53;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        unsafe {
            (*&mut err).i_line =
                if unsafe { (*&mut err).rc } != 0 {
                    unsafe { (*&mut err).i_line }
                } else { 50 }
        };
        join_all_threads_x(&mut err, &mut threads)
    };
    unsafe { sqlite3_enable_shared_cache(0) };
    print_and_free_err(&mut err);
}

extern "C" fn __main_inner(mut argc: i32, mut argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let a_test: [ThreadTestN10ThreadTest; 15] =
            [ThreadTestN10ThreadTest {
                        x_test: Some(walthread1),
                        z_test: c"walthread1".as_ptr() as *const i8,
                        n_ms: 20000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(walthread2),
                        z_test: c"walthread2".as_ptr() as *const i8,
                        n_ms: 20000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(walthread3),
                        z_test: c"walthread3".as_ptr() as *const i8,
                        n_ms: 20000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(walthread4),
                        z_test: c"walthread4".as_ptr() as *const i8,
                        n_ms: 20000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(walthread5),
                        z_test: c"walthread5".as_ptr() as *const i8,
                        n_ms: 1000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(cgt_pager_1),
                        z_test: c"cgt_pager_1".as_ptr() as *const i8,
                        n_ms: 0,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(dynamic_triggers),
                        z_test: c"dynamic_triggers".as_ptr() as *const i8,
                        n_ms: 20000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(checkpoint_starvation_1),
                        z_test: c"checkpoint_starvation_1".as_ptr() as *const i8,
                        n_ms: 10000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(checkpoint_starvation_2),
                        z_test: c"checkpoint_starvation_2".as_ptr() as *const i8,
                        n_ms: 10000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(create_drop_index_1),
                        z_test: c"create_drop_index_1".as_ptr() as *const i8,
                        n_ms: 10000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(lookaside1),
                        z_test: c"lookaside1".as_ptr() as *const i8,
                        n_ms: 10000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(vacuum1),
                        z_test: c"vacuum1".as_ptr() as *const i8,
                        n_ms: 10000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(stress1),
                        z_test: c"stress1".as_ptr() as *const i8,
                        n_ms: 10000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(stress2),
                        z_test: c"stress2".as_ptr() as *const i8,
                        n_ms: 60000,
                    },
                    ThreadTestN10ThreadTest {
                        x_test: Some(shared1),
                        z_test: c"shared1".as_ptr() as *const i8,
                        n_ms: 10000,
                    }];
        let mut i: i32 = 0;
        let mut i_arg: i32 = 0;
        let mut n_testfound: i32 = 0;
        let mut z_arg: *const i8 = core::ptr::null();
        let mut rc: i32 = 0;
        let mut z: *const i8 = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s55:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe {
                            printf(c"Usage: %s [-multiplexor] [testname|testprefix*]...\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(0 as isize) })
                        };
                        __state = 48;
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { n_testfound = 0; __state = 6; }
                    6 => { unsafe { sqlite3_config(2) }; __state = 7; }
                    7 => { if argc < 2 { __state = 9; } else { __state = 8; } }
                    8 => { i_arg = 1; __state = 12; }
                    9 => { argc = 2; __state = 10; }
                    10 => {
                        argv = &raw mut subst_argv[0 as usize] as *mut *mut i8;
                        __state = 8;
                    }
                    11 => { i_arg = 1; __state = 31; }
                    12 => {
                        if i_arg < argc { __state = 13; } else { __state = 11; }
                    }
                    13 => {
                        z_arg =
                            unsafe { *argv.offset(i_arg as isize) } as *const i8;
                        __state = 15;
                    }
                    14 => {
                        { let __p = &mut i_arg; let __t = *__p; *__p += 1; __t };
                        __state = 12;
                    }
                    15 => {
                        if unsafe { *z_arg.offset(0 as isize) } as i32 == '-' as i32
                            {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => { i = 0; __state = 25; }
                    17 => {
                        if unsafe {
                                    sqlite3_stricmp(z_arg,
                                        c"-multiplexor".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 19;
                        } else { __state = 20; }
                    }
                    18 => { __state = 14; }
                    19 => {
                        rc =
                            unsafe {
                                sqlite3_multiplex_initialize(core::ptr::null(), 1)
                            };
                        __state = 21;
                    }
                    20 => { __state = 2; }
                    21 => {
                        if rc != 0 { __state = 22; } else { __state = 18; }
                    }
                    22 => {
                        eprintln!("Failed to install multiplexor VFS ({})", rc as i32);
                        __state = 23;
                    }
                    23 => { return Err(253); }
                    24 => {
                        if i as u64 >=
                                core::mem::size_of::<[ThreadTestN10ThreadTest; 15]>() as u64
                                    / core::mem::size_of::<ThreadTestN10ThreadTest>() as u64 {
                            __state = 29;
                        } else { __state = 14; }
                    }
                    25 => {
                        if (i as u64) <
                                core::mem::size_of::<[ThreadTestN10ThreadTest; 15]>() as u64
                                    / core::mem::size_of::<ThreadTestN10ThreadTest>() as u64 {
                            __state = 26;
                        } else { __state = 24; }
                    }
                    26 => {
                        if unsafe {
                                    sqlite3_strglob(z_arg, a_test[i as usize].z_test)
                                } == 0 {
                            __state = 28;
                        } else { __state = 27; }
                    }
                    27 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 25;
                    }
                    28 => { __state = 24; }
                    29 => { __state = 2; }
                    30 => {
                        if n_testfound == 0 { __state = 45; } else { __state = 44; }
                    }
                    31 => {
                        if i_arg < argc { __state = 32; } else { __state = 30; }
                    }
                    32 => {
                        if unsafe {
                                        *unsafe {
                                                (*argv.offset(i_arg as isize)).offset(0 as isize)
                                            }
                                    } as i32 == '-' as i32 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    33 => {
                        { let __p = &mut i_arg; let __t = *__p; *__p += 1; __t };
                        __state = 31;
                    }
                    34 => { i = 0; __state = 36; }
                    35 => { __state = 33; }
                    36 => {
                        if (i as u64) <
                                core::mem::size_of::<[ThreadTestN10ThreadTest; 15]>() as u64
                                    / core::mem::size_of::<ThreadTestN10ThreadTest>() as u64 {
                            __state = 37;
                        } else { __state = 33; }
                    }
                    37 => { z = a_test[i as usize].z_test; __state = 39; }
                    38 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 36;
                    }
                    39 => {
                        if unsafe {
                                    sqlite3_strglob(unsafe { *argv.offset(i_arg as isize) } as
                                            *const i8, z)
                                } == 0 {
                            __state = 40;
                        } else { __state = 38; }
                    }
                    40 => {
                        unsafe {
                            printf(c"Running %s for %d seconds...\n".as_ptr() as *mut i8
                                    as *const i8, z, a_test[i as usize].n_ms / 1000)
                        };
                        __state = 41;
                    }
                    41 => { unsafe { fflush(__stdoutp) }; __state = 42; }
                    42 => {
                        unsafe {
                            a_test[i as usize].x_test.unwrap()(a_test[i as usize].n_ms)
                        };
                        __state = 43;
                    }
                    43 => {
                        {
                            let __p = &mut n_testfound;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 38;
                    }
                    44 => {
                        unsafe {
                            printf(c"%d errors out of %d tests\n".as_ptr() as *mut i8 as
                                    *const i8, n_global_err, n_testfound)
                        };
                        __state = 46;
                    }
                    45 => { __state = 2; }
                    46 => {
                        return Err(if n_global_err > 0 { 255 } else { 0 });
                    }
                    47 => { __state = 2; }
                    48 => {
                        unsafe {
                            printf(c"Available tests are:\n".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 49;
                    }
                    49 => { i = 0; __state = 51; }
                    50 => { return Err(254); }
                    51 => {
                        if (i as u64) <
                                core::mem::size_of::<[ThreadTestN10ThreadTest; 15]>() as u64
                                    / core::mem::size_of::<ThreadTestN10ThreadTest>() as u64 {
                            __state = 52;
                        } else { __state = 50; }
                    }
                    52 => {
                        unsafe {
                            printf(c"   %s\n".as_ptr() as *mut i8 as *const i8,
                                a_test[i as usize].z_test)
                        };
                        __state = 53;
                    }
                    53 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 51;
                    }
                    _ => {}
                }
            }
        }
        unreachable!();
        return Ok(());
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stress2TaskN11Stress2Task {
    x: Option<unsafe extern "C" fn(*mut Error, *mut Sqlite, i32) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ThreadTestN10ThreadTest {
    x_test: Option<unsafe extern "C" fn(i32) -> ()>,
    z_test: *const i8,
    n_ms: i32,
}

static z_encode: [i8; 17] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 97 as i8, 98 as i8, 99 as i8,
            100 as i8, 101 as i8, 102 as i8, 0 as i8];

static mut p_timelimit_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();

static mut subst_argv: [*mut i8; 3] =
    [core::ptr::null_mut(), c"*".as_ptr() as *mut i8, core::ptr::null_mut()];

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
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
    fn sqlite3_multiplex_initialize(z_orig_vfs_name_1: *const i8,
    make_default_1: i32)
    -> i32;
    fn sqlite3_multiplex_shutdown(e_force_1: i32)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn strerror_r(__errnum: i32, __strerrbuf: *mut i8, __buflen: u64)
    -> i32;
    fn unlink(_: *const i8)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn pthread_create(_: *mut PthreadT, _: *const PthreadAttrT,
    _: unsafe extern "C" fn(*mut ()) -> *mut (), _: *mut ())
    -> i32;
    fn pthread_join(_: PthreadT, _: *mut *mut ())
    -> i32;
    fn stat(_: *const i8, _: *mut Stat)
    -> i32;
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn __error()
    -> *mut i32;
    fn close(_: i32)
    -> i32;
    fn read(_: i32, _: *mut (), __nbyte: u64)
    -> i64;
    fn write(__fd: i32, __buf: *const (), __nbyte: u64)
    -> i64;
    fn __builtin_unreachable()
    -> ();
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;

#[repr(C)]
#[derive(Copy, Clone)]
struct OpaquePthreadT {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct OpaquePthreadAttrT {
    _opaque: [u8; 0],
}

type PthreadAttrT = OpaquePthreadAttrT;
