type DarwinSizeT = u64;
type Int64T = i64;
type DarwinOffT = Int64T;
type OffT = DarwinOffT;
type DarwinSsizeT = i64;
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
static mut pagesize: i32 = 1024;
static mut fd: i32 = -1;
static mut mx_frame: i32 = 0;
static mut per_line: i32 = 16;
#[repr(C)]
#[derive(Copy, Clone)]
struct Cksum {
    b_swap: i32,
    s0: u32,
    s1: u32,
}
extern "C" fn get_int32(a: *const u8) -> u32 {
    let x: u32 =
        (((unsafe { *a.offset(0 as isize) } as i32) << 24) +
                        ((unsafe { *a.offset(1 as isize) } as i32) << 16) +
                    ((unsafe { *a.offset(2 as isize) } as i32) << 8) +
                unsafe { *a.offset(3 as isize) } as i32) as u32;
    return x;
}
extern "C" fn swab32(x: u32) -> u32 {
    return ((x & 255 as u32) << 24) + ((x & 65280 as u32) << 8) +
                ((x & 16711680 as u32) >> 8) + ((x & 4278190080u32) >> 24);
}
extern "C" fn extend_cksum(p_cksum_1: &mut Cksum, a_data_1: *mut u8,
    mut n_byte_1: u32, b_init_1: i32) -> () {
    let mut a32: *const u32 = core::ptr::null();
    if b_init_1 != 0 {
        let mut a: i32 = 0;
        unsafe { *(&raw mut a as *mut i8) = 1 as i8 };
        if a == 1 {
            (*p_cksum_1).b_swap =
                (get_int32(a_data_1 as *const u8) != 931071618 as u32) as i32;
        } else {
            (*p_cksum_1).b_swap =
                (get_int32(a_data_1 as *const u8) != 931071619 as u32) as i32;
        }
        (*p_cksum_1).s0 = 0 as u32;
        (*p_cksum_1).s1 = 0 as u32;
    }
    a32 = a_data_1 as *mut u32;
    while n_byte_1 > 0 as u32 {
        let mut x0: u32 = unsafe { *a32.offset(0 as isize) };
        let mut x1: u32 = unsafe { *a32.offset(1 as isize) };
        if (*p_cksum_1).b_swap != 0 { x0 = swab32(x0); x1 = swab32(x1); }
        (*p_cksum_1).s0 += x0 + (*p_cksum_1).s1;
        (*p_cksum_1).s1 += x1 + (*p_cksum_1).s0;
        n_byte_1 -= 8 as u32;
        {
            let __n = 2;
            let __p = &mut a32;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
}
extern "C" fn decode_varint(z: *const u8, p_val_1: &mut i64) -> i32 {
    let mut v: i64 = 0 as i64;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b1: loop {
            if !(i < 8) { break '__b1; }
            '__c1: loop {
                v =
                    (v << 7) +
                        (unsafe { *z.offset(i as isize) } as i32 & 127) as i64;
                if unsafe { *z.offset(i as isize) } as i32 & 128 == 0 {
                    *p_val_1 = v;
                    return i + 1;
                }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    v = (v << 8) + (unsafe { *z.offset(i as isize) } as i32 & 255) as i64;
    *p_val_1 = v;
    return 9;
}
extern "C" fn out_of_memory() -> () {
    eprintln!("Out of memory...");
    unsafe { exit(1) };
}
extern "C" fn get_content(ofst: i64, n_byte_1: i32) -> *mut u8 {
    unsafe {
        let mut a_data: *mut u8 = core::ptr::null_mut();
        a_data = unsafe { malloc(n_byte_1 as u64) } as *mut u8;
        if a_data == core::ptr::null_mut() { out_of_memory(); }
        unsafe { lseek(fd, ofst, 0) };
        unsafe { read(fd, a_data as *mut (), n_byte_1 as u64) };
        return a_data;
    }
}
extern "C" fn print_byte_range(ofst: i32, a_data_1: &[u8], print_ofst_1: i32)
    -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut z_ofst_fmt: *const i8 = core::ptr::null();
        if print_ofst_1 + a_data_1.len() as i32 & !4095 == 0 {
            z_ofst_fmt = c" %03x: ".as_ptr() as *mut i8 as *const i8;
        } else if print_ofst_1 + a_data_1.len() as i32 & !65535 == 0 {
            z_ofst_fmt = c" %04x: ".as_ptr() as *mut i8 as *const i8;
        } else if print_ofst_1 + a_data_1.len() as i32 & !1048575 == 0 {
            z_ofst_fmt = c" %05x: ".as_ptr() as *mut i8 as *const i8;
        } else if print_ofst_1 + a_data_1.len() as i32 & !16777215 == 0 {
            z_ofst_fmt = c" %06x: ".as_ptr() as *mut i8 as *const i8;
        } else { z_ofst_fmt = c" %08x: ".as_ptr() as *mut i8 as *const i8; }
        {
            i = 0;
            '__b2: loop {
                if !(i < a_data_1.len() as i32) { break '__b2; }
                '__c2: loop {
                    unsafe { fprintf(__stdoutp, z_ofst_fmt, i + print_ofst_1) };
                    {
                        j = 0;
                        '__b3: loop {
                            if !(j < per_line) { break '__b3; }
                            '__c3: loop {
                                if i + j > a_data_1.len() as i32 {
                                    unsafe {
                                        fprintf(__stdoutp, c"   ".as_ptr() as *mut i8 as *const i8)
                                    };
                                } else {
                                    unsafe {
                                        fprintf(__stdoutp,
                                            c"%02x ".as_ptr() as *mut i8 as *const i8,
                                            a_data_1[(i + j) as usize] as i32)
                                    };
                                }
                                break '__c3;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    {
                        j = 0;
                        '__b4: loop {
                            if !(j < per_line) { break '__b4; }
                            '__c4: loop {
                                if i + j > a_data_1.len() as i32 {
                                    unsafe {
                                        fprintf(__stdoutp, c" ".as_ptr() as *mut i8 as *const i8)
                                    };
                                } else {
                                    unsafe {
                                        fprintf(__stdoutp, c"%c".as_ptr() as *mut i8 as *const i8,
                                            if unsafe {
                                                        isprint(a_data_1[(i + j) as usize] as u8 as i32)
                                                    } != 0 {
                                                a_data_1[(i + j) as usize] as i32
                                            } else { '.' as i32 })
                                    };
                                }
                                break '__c4;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        fprintf(__stdoutp, c"\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__c2;
                }
                i += per_line;
            }
        }
    }
}
extern "C" fn print_decode_line(a_data_1: *const u8, ofst: i32, n_byte_1: i32,
    as_hex_1: i32, z_msg_1: *const i8) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut val: i32 = unsafe { *a_data_1.offset(ofst as isize) } as i32;
    let mut z_buf: [i8; 100] = [0; 100];
    unsafe {
        sprintf(&raw mut z_buf[0 as usize] as *mut i8,
            c" %03x: %02x".as_ptr() as *mut i8 as *const i8, ofst,
            unsafe { *a_data_1.offset(ofst as isize) } as i32)
    };
    i =
        unsafe { strlen(&raw mut z_buf[0 as usize] as *mut i8 as *const i8) }
            as i32;
    {
        j = 1;
        '__b5: loop {
            if !(j < 4) { break '__b5; }
            '__c5: loop {
                if j >= n_byte_1 {
                    unsafe {
                        sprintf(&mut z_buf[i as usize],
                            c"   ".as_ptr() as *mut i8 as *const i8)
                    };
                } else {
                    unsafe {
                        sprintf(&mut z_buf[i as usize],
                            c" %02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *a_data_1.offset((ofst + j) as isize) } as i32)
                    };
                    val =
                        val * 256 +
                            unsafe { *a_data_1.offset((ofst + j) as isize) } as i32;
                }
                i +=
                    unsafe { strlen(&raw mut z_buf[i as usize] as *const i8) }
                        as i32;
                break '__c5;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    if as_hex_1 != 0 {
        unsafe {
            sprintf(&mut z_buf[i as usize],
                c"  0x%08x".as_ptr() as *mut i8 as *const i8, val)
        };
    } else {
        unsafe {
            sprintf(&mut z_buf[i as usize],
                c"   %9d".as_ptr() as *mut i8 as *const i8, val)
        };
    }
    unsafe {
        printf(c"%s  %s\n".as_ptr() as *mut i8 as *const i8,
            &raw mut z_buf[0 as usize] as *mut i8, z_msg_1)
    };
}
extern "C" fn print_frame(i_frame_1: i32) -> () {
    unsafe {
        let mut i_start: i64 = 0 as i64;
        let mut a_data: *mut u8 = core::ptr::null_mut();
        i_start = 32 as i64 + (i_frame_1 - 1) as i64 * (pagesize + 24) as i64;
        unsafe {
            fprintf(__stdoutp,
                c"Frame %d:   (offsets 0x%llx..0x%llx)\n".as_ptr() as *mut i8
                    as *const i8, i_frame_1, i_start,
                i_start + pagesize as i64 + 24 as i64)
        };
        a_data = get_content(i_start, pagesize + 24);
        print_decode_line(a_data as *const u8, 0, 4, 0,
            c"Page number".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a_data as *const u8, 4, 4, 0,
            c"DB size, or 0 for non-commit".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a_data as *const u8, 8, 4, 1,
            c"Salt-1".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a_data as *const u8, 12, 4, 1,
            c"Salt-2".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a_data as *const u8, 16, 4, 1,
            c"Checksum-1".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a_data as *const u8, 20, 4, 1,
            c"Checksum-2".as_ptr() as *mut i8 as *const i8);
        print_byte_range((i_start + 24 as i64) as i32,
            unsafe {
                let __p = unsafe { a_data.offset(24 as isize) } as *const u8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, pagesize as usize) }
            }, 0);
        unsafe { free(a_data as *mut ()) };
    }
}
extern "C" fn print_oneline_frame(i_frame_1: i32, p_cksum_1: *mut Cksum)
    -> () {
    unsafe {
        let mut i_start: i64 = 0 as i64;
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut s0: u32 = 0 as u32;
        let mut s1: u32 = 0 as u32;
        i_start = 32 as i64 + (i_frame_1 - 1) as i64 * (pagesize + 24) as i64;
        a_data = get_content(i_start, 24);
        extend_cksum(unsafe { &mut *p_cksum_1 }, a_data, 8 as u32, 0);
        extend_cksum(unsafe { &mut *p_cksum_1 },
            get_content(i_start + 24 as i64, pagesize), pagesize as u32, 0);
        s0 = get_int32(unsafe { a_data.offset(16 as isize) } as *const u8);
        s1 = get_int32(unsafe { a_data.offset(20 as isize) } as *const u8);
        unsafe {
            fprintf(__stdoutp,
                c"Frame %4d: %6d %6d 0x%08x,%08x 0x%08x,%08x".as_ptr() as
                        *mut i8 as *const i8, i_frame_1,
                get_int32(a_data as *const u8),
                get_int32(unsafe { a_data.offset(4 as isize) } as *const u8),
                get_int32(unsafe { a_data.offset(8 as isize) } as *const u8),
                get_int32(unsafe { a_data.offset(12 as isize) } as *const u8),
                s0, s1)
        };
        if s0 == unsafe { (*p_cksum_1).s0 } &&
                s1 == unsafe { (*p_cksum_1).s1 } {
            unsafe {
                fprintf(__stdoutp, c"\n".as_ptr() as *mut i8 as *const i8)
            };
        } else {
            unsafe {
                fprintf(__stdoutp,
                    c" should be 0x%08x,%08x\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_cksum_1).s0 },
                    unsafe { (*p_cksum_1).s1 })
            };
        }
        unsafe { (*p_cksum_1).s0 = s0 };
        unsafe { (*p_cksum_1).s1 = s1 };
        unsafe { free(a_data as *mut ()) };
    }
}
extern "C" fn print_wal_header(p_cksum_1: *mut Cksum) -> () {
    let mut a_data: *mut u8 = core::ptr::null_mut();
    a_data = get_content(0 as i64, 32);
    if !(p_cksum_1).is_null() {
        extend_cksum(unsafe { &mut *p_cksum_1 }, a_data, 24 as u32, 1);
        unsafe {
            printf(c"Checksum byte order: %s\n".as_ptr() as *mut i8 as
                    *const i8,
                if unsafe { (*p_cksum_1).b_swap } != 0 {
                    c"swapped".as_ptr() as *mut i8
                } else { c"native".as_ptr() as *mut i8 })
        };
    }
    unsafe { printf(c"WAL Header:\n".as_ptr() as *mut i8 as *const i8) };
    print_decode_line(a_data as *const u8, 0, 4, 1,
        c"Magic.  0x377f0682 (le) or 0x377f0683 (be)".as_ptr() as *mut i8 as
            *const i8);
    print_decode_line(a_data as *const u8, 4, 4, 0,
        c"File format".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 8, 4, 0,
        c"Database page size".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 12, 4, 0,
        c"Checkpoint sequence number".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 16, 4, 1,
        c"Salt-1".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 20, 4, 1,
        c"Salt-2".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 24, 4, 1,
        c"Checksum-1".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 28, 4, 1,
        c"Checksum-2".as_ptr() as *mut i8 as *const i8);
    if !(p_cksum_1).is_null() {
        if unsafe { (*p_cksum_1).s0 } !=
                get_int32(unsafe { a_data.offset(24 as isize) } as *const u8)
            {
            unsafe {
                printf(c"**** cksum-1 mismatch: 0x%08x\n".as_ptr() as *mut i8
                        as *const i8, unsafe { (*p_cksum_1).s0 })
            };
        }
        if unsafe { (*p_cksum_1).s1 } !=
                get_int32(unsafe { a_data.offset(28 as isize) } as *const u8)
            {
            unsafe {
                printf(c"**** cksum-2 mismatch: 0x%08x\n".as_ptr() as *mut i8
                        as *const i8, unsafe { (*p_cksum_1).s1 })
            };
        }
    }
    unsafe { free(a_data as *mut ()) };
}
extern "C" fn describe_content(mut a: *mut u8, mut n_local_1: i64,
    mut z_desc_1: *mut i8) -> i64 {
    let mut n_desc: i32 = 0;
    let mut n: i32 = 0;
    let mut j: i32 = 0;
    let mut i: i64 = 0 as i64;
    let mut x: i64 = 0 as i64;
    let mut v: i64 = 0 as i64;
    let mut p_data: *const u8 = core::ptr::null();
    let mut p_limit: *const u8 = core::ptr::null();
    let mut sep: i8 = ' ' as i32 as i8;
    p_limit = unsafe { a.offset(n_local_1 as isize) } as *const u8;
    n = decode_varint(a as *const u8, &mut x);
    p_data = unsafe { a.offset(x as isize) } as *const u8;
    {
        let __n = n;
        let __p = &mut a;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    i = x - n as i64;
    while i > 0 as i64 && p_data <= p_limit {
        n = decode_varint(a as *const u8, &mut x);
        {
            let __n = n;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        i -= n as i64;
        n_local_1 -= n as i64;
        unsafe { *z_desc_1.offset(0 as isize) = sep };
        sep = ',' as i32 as i8;
        { let __p = &mut n_desc; let __t = *__p; *__p += 1; __t };
        {
            let __p = &mut z_desc_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        if x == 0 as i64 {
            unsafe {
                sprintf(z_desc_1, c"*".as_ptr() as *mut i8 as *const i8)
            };
        } else if x >= 1 as i64 && x <= 6 as i64 {
            v = unsafe { *p_data.offset(0 as isize) } as i8 as i64;
            {
                let __p = &mut p_data;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            '__s7:
                {
                match x {
                    6 => {
                        v =
                            (v << 16) +
                                    ((unsafe { *p_data.offset(0 as isize) } as i32) << 8) as i64
                                + unsafe { *p_data.offset(1 as isize) } as i64;
                        {
                            let __n = 2;
                            let __p = &mut p_data;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        v =
                            (v << 16) +
                                    ((unsafe { *p_data.offset(0 as isize) } as i32) << 8) as i64
                                + unsafe { *p_data.offset(1 as isize) } as i64;
                        {
                            let __n = 2;
                            let __p = &mut p_data;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    5 => {
                        v =
                            (v << 16) +
                                    ((unsafe { *p_data.offset(0 as isize) } as i32) << 8) as i64
                                + unsafe { *p_data.offset(1 as isize) } as i64;
                        {
                            let __n = 2;
                            let __p = &mut p_data;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    4 => {
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    3 => {
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    2 => {
                        v = (v << 8) + unsafe { *p_data.offset(0 as isize) } as i64;
                        {
                            let __p = &mut p_data;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    _ => {}
                }
            }
            unsafe {
                sprintf(z_desc_1, c"%lld".as_ptr() as *mut i8 as *const i8, v)
            };
        } else if x == 7 as i64 {
            unsafe {
                sprintf(z_desc_1, c"real".as_ptr() as *mut i8 as *const i8)
            };
            {
                let __n = 8;
                let __p = &mut p_data;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        } else if x == 8 as i64 {
            unsafe {
                sprintf(z_desc_1, c"0".as_ptr() as *mut i8 as *const i8)
            };
        } else if x == 9 as i64 {
            unsafe {
                sprintf(z_desc_1, c"1".as_ptr() as *mut i8 as *const i8)
            };
        } else if x >= 12 as i64 {
            let size: i64 = (x - 12 as i64) / 2 as i64;
            if x & 1 as i64 == 0 as i64 {
                unsafe {
                    sprintf(z_desc_1,
                        c"blob(%lld)".as_ptr() as *mut i8 as *const i8, size)
                };
            } else {
                unsafe {
                    sprintf(z_desc_1,
                        c"txt(%lld)".as_ptr() as *mut i8 as *const i8, size)
                };
            }
            {
                let __n = size;
                let __p = &mut p_data;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        j = unsafe { strlen(z_desc_1 as *const i8) } as i32;
        {
            let __n = j;
            let __p = &mut z_desc_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        n_desc += j;
    }
    return n_desc as i64;
}
extern "C" fn local_payload(n_payload_1: i64, c_type_1: i8) -> i64 {
    unsafe {
        let mut max_local: i64 = 0 as i64;
        let mut min_local: i64 = 0 as i64;
        let mut surplus: i64 = 0 as i64;
        let mut n_local: i64 = 0 as i64;
        if c_type_1 as i32 == 13 {
            max_local = (pagesize - 35) as i64;
            min_local = ((pagesize - 12) * 32 / 255 - 23) as i64;
        } else {
            max_local = ((pagesize - 12) * 64 / 255 - 23) as i64;
            min_local = ((pagesize - 12) * 32 / 255 - 23) as i64;
        }
        if n_payload_1 > max_local {
            surplus =
                min_local + (n_payload_1 - min_local) % (pagesize - 4) as i64;
            if surplus <= max_local {
                n_local = surplus;
            } else { n_local = min_local; }
        } else { n_local = n_payload_1; }
        return n_local;
    }
}
extern "C" fn describe_cell(c_type_1: u8, mut a: *mut u8,
    show_cell_content_1: i32, pz_desc_1: &mut *mut i8) -> i64 {
    unsafe {
        let mut i: i32 = 0;
        let mut n_desc: i64 = 0 as i64;
        let mut n: i32 = 0;
        let mut left_child: i32 = 0;
        let mut n_payload: i64 = 0 as i64;
        let mut rowid: i64 = 0 as i64;
        let mut n_local: i64 = 0 as i64;
        i = 0;
        if c_type_1 as i32 <= 5 {
            left_child =
                ((unsafe { *a.offset(0 as isize) } as i32 * 256 +
                                    unsafe { *a.offset(1 as isize) } as i32) * 256 +
                            unsafe { *a.offset(2 as isize) } as i32) * 256 +
                    unsafe { *a.offset(3 as isize) } as i32;
            {
                let __n = 4;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += 4;
            unsafe {
                sprintf(&raw mut z_desc_2[0 as usize] as *mut i8,
                    c"lx: %d ".as_ptr() as *mut i8 as *const i8, left_child)
            };
            n_desc =
                unsafe {
                        strlen(&raw mut z_desc_2[0 as usize] as *mut i8 as
                                *const i8)
                    } as i64;
        }
        if c_type_1 as i32 != 5 {
            i = decode_varint(a as *const u8, &mut n_payload);
            {
                let __n = i;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += i;
            unsafe {
                sprintf(&mut z_desc_2[n_desc as usize],
                    c"n: %lld ".as_ptr() as *mut i8 as *const i8, n_payload)
            };
            n_desc +=
                unsafe {
                            strlen(&raw mut z_desc_2[n_desc as usize] as *const i8)
                        } as u64 as i64;
            n_local = local_payload(n_payload, c_type_1 as i8);
        } else { n_payload = { n_local = 0 as i64; n_local }; }
        if c_type_1 as i32 == 5 || c_type_1 as i32 == 13 {
            i = decode_varint(a as *const u8, &mut rowid);
            {
                let __n = i;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += i;
            unsafe {
                sprintf(&mut z_desc_2[n_desc as usize],
                    c"r: %lld ".as_ptr() as *mut i8 as *const i8, rowid)
            };
            n_desc +=
                unsafe {
                            strlen(&raw mut z_desc_2[n_desc as usize] as *const i8)
                        } as u64 as i64;
        }
        if n_local < n_payload {
            let mut ovfl: i32 = 0;
            let b: *const u8 =
                unsafe { &raw mut *a.offset(n_local as isize) } as *const u8;
            ovfl =
                ((unsafe { *b.offset(0 as isize) } as i32 * 256 +
                                    unsafe { *b.offset(1 as isize) } as i32) * 256 +
                            unsafe { *b.offset(2 as isize) } as i32) * 256 +
                    unsafe { *b.offset(3 as isize) } as i32;
            unsafe {
                sprintf(&mut z_desc_2[n_desc as usize],
                    c"ov: %d ".as_ptr() as *mut i8 as *const i8, ovfl)
            };
            n_desc +=
                unsafe {
                            strlen(&raw mut z_desc_2[n_desc as usize] as *const i8)
                        } as u64 as i64;
            n += 4;
        }
        if show_cell_content_1 != 0 && c_type_1 as i32 != 5 {
            n_desc +=
                describe_content(a, n_local,
                    &mut z_desc_2[(n_desc - 1 as i64) as usize]);
        }
        *pz_desc_1 = &raw mut z_desc_2[0 as usize] as *mut i8;
        return n_local + n as i64;
    }
}
extern "C" fn decode_btree_page(a: *mut u8, pgno: i32, hdr_size_1: i32,
    mut z_args_1: *const i8) -> () {
    unsafe {
        let mut z_type: *const i8 =
            c"unknown".as_ptr() as *mut i8 as *const i8;
        let mut n_cell: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut i_cell_ptr: i32 = 0;
        let mut show_cell_content: i32 = 0;
        let mut show_map: i32 = 0;
        let mut z_map: *mut i8 = core::ptr::null_mut();
        '__s8:
            {
            match unsafe { *a.offset(0 as isize) } {
                2 => {
                    z_type =
                        c"index interior node".as_ptr() as *mut i8 as *const i8;
                }
                5 => {
                    z_type =
                        c"table interior node".as_ptr() as *mut i8 as *const i8;
                }
                10 => {
                    z_type = c"index leaf".as_ptr() as *mut i8 as *const i8;
                }
                13 => {
                    z_type = c"table leaf".as_ptr() as *mut i8 as *const i8;
                }
                _ => {}
            }
        }
        while unsafe { *z_args_1.offset(0 as isize) } != 0 {
            '__s10:
                {
                match unsafe { *z_args_1.offset(0 as isize) } {
                    99 => { show_cell_content = 1; }
                    109 => { show_map = 1; }
                    _ => {}
                }
            }
            {
                let __p = &mut z_args_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        unsafe {
            printf(c"Decode of btree page %d:\n".as_ptr() as *mut i8 as
                    *const i8, pgno)
        };
        print_decode_line(a as *const u8, 0, 1, 0, z_type);
        print_decode_line(a as *const u8, 1, 2, 0,
            c"Offset to first freeblock".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a as *const u8, 3, 2, 0,
            c"Number of cells on this page".as_ptr() as *mut i8 as *const i8);
        n_cell =
            unsafe { *a.offset(3 as isize) } as i32 * 256 +
                unsafe { *a.offset(4 as isize) } as i32;
        print_decode_line(a as *const u8, 5, 2, 0,
            c"Offset to cell content area".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a as *const u8, 7, 1, 0,
            c"Fragmented byte count".as_ptr() as *mut i8 as *const i8);
        if unsafe { *a.offset(0 as isize) } as i32 == 2 ||
                unsafe { *a.offset(0 as isize) } as i32 == 5 {
            print_decode_line(a as *const u8, 8, 4, 0,
                c"Right child".as_ptr() as *mut i8 as *const i8);
            i_cell_ptr = 12;
        } else { i_cell_ptr = 8; }
        if n_cell > 0 {
            unsafe {
                printf(c" key: lx=left-child n=payload-size r=rowid\n".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        if show_map != 0 {
            z_map = unsafe { malloc(pagesize as u64) } as *mut i8;
            unsafe { memset(z_map as *mut (), '.' as i32, pagesize as u64) };
            unsafe {
                memset(z_map as *mut (), '1' as i32, hdr_size_1 as u64)
            };
            unsafe {
                memset(unsafe { &raw mut *z_map.offset(hdr_size_1 as isize) }
                        as *mut (), 'H' as i32, i_cell_ptr as u64)
            };
            unsafe {
                memset(unsafe {
                            &raw mut *z_map.offset((hdr_size_1 + i_cell_ptr) as isize)
                        } as *mut (), 'P' as i32, (2 * n_cell) as u64)
            };
        }
        {
            i = 0;
            '__b11: loop {
                if !(i < n_cell) { break '__b11; }
                '__c11: loop {
                    let mut cofst: i32 = i_cell_ptr + i * 2;
                    let mut z_desc: *mut i8 = core::ptr::null_mut();
                    let mut n: i64 = 0 as i64;
                    cofst =
                        unsafe { *a.offset(cofst as isize) } as i32 * 256 +
                            unsafe { *a.offset((cofst + 1) as isize) } as i32;
                    n =
                        describe_cell(unsafe { *a.offset(0 as isize) },
                            unsafe { &mut *a.offset((cofst - hdr_size_1) as isize) },
                            show_cell_content, &mut z_desc);
                    if show_map != 0 {
                        let mut z_buf: [i8; 30] = [0; 30];
                        unsafe {
                            memset(unsafe { &raw mut *z_map.offset(cofst as isize) } as
                                    *mut (), '*' as i32, n as u64)
                        };
                        unsafe { *z_map.offset(cofst as isize) = '[' as i32 as i8 };
                        unsafe {
                            *z_map.offset((cofst as i64 + n - 1 as i64) as isize) =
                                ']' as i32 as i8
                        };
                        unsafe {
                            sprintf(&raw mut z_buf[0 as usize] as *mut i8,
                                c"%d".as_ptr() as *mut i8 as *const i8, i)
                        };
                        j =
                            unsafe {
                                    strlen(&raw mut z_buf[0 as usize] as *mut i8 as *const i8)
                                } as i32;
                        if j as i64 <= n - 2 as i64 {
                            unsafe {
                                memcpy(unsafe {
                                            &raw mut *z_map.offset((cofst + 1) as isize)
                                        } as *mut (),
                                    &raw mut z_buf[0 as usize] as *mut i8 as *const (),
                                    j as u64)
                            };
                        }
                    }
                    unsafe {
                        printf(c" %03x: cell[%d] %s\n".as_ptr() as *mut i8 as
                                *const i8, cofst, i, z_desc)
                    };
                    break '__c11;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if show_map != 0 {
            {
                i = 0;
                '__b12: loop {
                    if !(i < pagesize) { break '__b12; }
                    '__c12: loop {
                        unsafe {
                            printf(c" %03x: %.64s\n".as_ptr() as *mut i8 as *const i8,
                                i, unsafe { &raw mut *z_map.offset(i as isize) } as *mut i8)
                        };
                        break '__c12;
                    }
                    i += 64;
                }
            }
            unsafe { free(z_map as *mut ()) };
        }
    }
}
extern "C" fn check_page_validity(i_page_1: i32, mx_page_1: i32) -> () {
    if i_page_1 < 1 || i_page_1 > mx_page_1 {
        eprintln!("Invalid page number {}:  valid range is 1..{}", i_page_1 as i32, mx_page_1 as i32);
        unsafe { exit(1) };
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut sbuf: Stat = unsafe { core::mem::zeroed() };
        let mut z_pg_sz: [u8; 4] = [0; 4];
        if argc < 2 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s FILENAME ?PAGE? ...\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { *argv.offset(0 as isize) })
            };
            unsafe { exit(1) };
        }
        fd =
            unsafe {
                open(unsafe { *argv.offset(1 as isize) } as *const i8, 0)
            };
        if fd < 0 {
            unsafe {
                fprintf(__stderrp,
                    c"%s: can\'t open %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) },
                    unsafe { *argv.offset(1 as isize) })
            };
            unsafe { exit(1) };
        }
        z_pg_sz[0 as usize] = 0 as u8;
        z_pg_sz[1 as usize] = 0 as u8;
        unsafe { fstat(fd, &mut sbuf) };
        if sbuf.st_size < 32 as i64 {
            unsafe {
                printf(c"%s: file too small to be a WAL - only %d bytes\n".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) }, sbuf.st_size as i32)
            };
            return Ok(());
        }
        if unsafe { lseek(fd, 8 as OffT, 0) } != 8 as i64 {
            unsafe {
                printf(c"\"%s\" seems to not be a valid WAL file\n".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) })
            };
            return Err(1);
        }
        if unsafe {
                    read(fd, &raw mut z_pg_sz[0 as usize] as *mut u8 as *mut (),
                        4 as u64)
                } != 4 as i64 {
            unsafe {
                printf(c"\"%s\": cannot read the page size\n".as_ptr() as
                            *mut i8 as *const i8, unsafe { *argv.offset(1 as isize) })
            };
            return Err(1);
        }
        pagesize =
            z_pg_sz[1 as usize] as i32 * 65536 +
                    z_pg_sz[2 as usize] as i32 * 256 +
                z_pg_sz[3 as usize] as i32;
        if pagesize == 0 { pagesize = 1024; }
        unsafe {
            printf(c"Pagesize: %d\n".as_ptr() as *mut i8 as *const i8,
                pagesize)
        };
        if pagesize & pagesize - 1 != 0 || pagesize < 512 || pagesize > 65536
            {
            unsafe {
                printf(c"\"%s\": invalid page size.\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { *argv.offset(1 as isize) })
            };
            return Err(1);
        }
        mx_frame =
            ((sbuf.st_size - 32 as OffT) / (pagesize + 24) as OffT) as i32;
        unsafe {
            printf(c"Available pages: 1..%d\n".as_ptr() as *mut i8 as
                    *const i8, mx_frame)
        };
        if argc == 2 {
            let mut i: i32 = 0;
            let mut x: Cksum = unsafe { core::mem::zeroed() };
            print_wal_header(&mut x);
            {
                i = 1;
                '__b13: loop {
                    if !(i <= mx_frame) { break '__b13; }
                    '__c13: loop {
                        print_oneline_frame(i, &mut x);
                        break '__c13;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else {
            let mut i: i32 = 0;
            {
                i = 2;
                '__b14: loop {
                    if !(i < argc) { break '__b14; }
                    '__c14: loop {
                        let mut i_start: i32 = 0;
                        let mut i_end: i32 = 0;
                        let mut z_left: *mut i8 = core::ptr::null_mut();
                        if unsafe {
                                    strcmp(unsafe { *argv.offset(i as isize) } as *const i8,
                                        c"header".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            print_wal_header(core::ptr::null_mut());
                            break '__c14;
                        }
                        if (unsafe {
                                            isdigit(unsafe {
                                                            *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                                        } as u8 as i32)
                                        } == 0) as i32 != 0 {
                            unsafe {
                                fprintf(__stderrp,
                                    c"%s: unknown option: [%s]\n".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(0 as isize) },
                                    unsafe { *argv.offset(i as isize) })
                            };
                            break '__c14;
                        }
                        i_start =
                            unsafe {
                                    strtol(unsafe { *argv.offset(i as isize) } as *const i8,
                                        &mut z_left, 0)
                                } as i32;
                        check_page_validity(i_start, mx_frame);
                        if !(z_left).is_null() &&
                                unsafe {
                                        strcmp(z_left as *const i8,
                                            c"..end".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            i_end = mx_frame;
                        } else if !(z_left).is_null() &&
                                    unsafe { *z_left.offset(0 as isize) } as i32 == '.' as i32
                                &&
                                unsafe { *z_left.offset(1 as isize) } as i32 == '.' as i32 {
                            i_end =
                                unsafe {
                                        strtol(unsafe { &raw mut *z_left.offset(2 as isize) } as
                                                *const i8, core::ptr::null_mut(), 0)
                                    } as i32;
                            check_page_validity(i_end, mx_frame);
                        } else if !(z_left).is_null() &&
                                unsafe { *z_left.offset(0 as isize) } as i32 == 'b' as i32 {
                            let mut ofst: i64 = 0 as i64;
                            let mut n_byte: i32 = 0;
                            let mut hdr_size: i32 = 0;
                            let mut a: *mut u8 = core::ptr::null_mut();
                            if i_start == 1 {
                                hdr_size = 100;
                                ofst = { hdr_size = 100; hdr_size } as i64;
                                n_byte = pagesize - 100;
                            } else {
                                hdr_size = 0;
                                ofst = (i_start - 1) as i64 * pagesize as i64;
                                n_byte = pagesize;
                            }
                            ofst =
                                (32 + hdr_size) as i64 +
                                        (i_start - 1) as i64 * (pagesize + 24) as i64 + 24 as i64;
                            a = get_content(ofst, n_byte);
                            decode_btree_page(a, i_start, hdr_size,
                                unsafe { z_left.offset(1 as isize) } as *const i8);
                            unsafe { free(a as *mut ()) };
                            break '__c14;
                        } else if !(z_left).is_null() &&
                                unsafe {
                                        strcmp(z_left as *const i8,
                                            c"truncate".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            let new_size: OffT =
                                (32 + i_start * (pagesize + 24)) as OffT;
                            unsafe {
                                truncate(unsafe { *argv.offset(1 as isize) } as *const i8,
                                    new_size)
                            };
                            break '__c14;
                        } else { i_end = i_start; }
                        if i_start < 1 || i_end < i_start || i_end > mx_frame {
                            eprintln!("Page argument should be LOWER?..UPPER?.  Range 1 to {}", mx_frame as i32);
                            unsafe { exit(1) };
                        }
                        while i_start <= i_end {
                            print_frame(i_start);
                            { let __p = &mut i_start; let __t = *__p; *__p += 1; __t };
                        }
                        break '__c14;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        unsafe { close(fd) };
        return Ok(());
    }
}
static mut z_desc_2: [i8; 1000] = unsafe { core::mem::zeroed() };
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn exit(_: i32)
    -> ();
    fn malloc(__size: u64)
    -> *mut ();
    fn lseek(_: i32, _: OffT, _: i32)
    -> OffT;
    fn read(_: i32, _: *mut (), __nbyte: u64)
    -> i64;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn isprint(_c: i32)
    -> i32;
    fn sprintf(_: *mut i8, _: *const i8, ...)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn free(_: *mut ())
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn fstat(_: i32, _: *mut Stat)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn isdigit(_c: i32)
    -> i32;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    fn truncate(_: *const i8, _: OffT)
    -> i32;
    fn close(_: i32)
    -> i32;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;