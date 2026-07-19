#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};

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

#[repr(C)]
#[derive(Copy, Clone)]
struct GlobalData {
    pagesize: i64,
    usablesize: i64,
    dbfd: i32,
    mx_page: u32,
    n_res: u32,
    per_line: i32,
    b_raw: i32,
    b_csv: i32,
    b_tmstmp: i32,
    p_fd: *mut Sqlite3File,
    p_db: *mut Sqlite3,
    z_page_use: *mut *mut i8,
    a_page_tag: *mut TmstmpTag,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TmstmpTag {
    a: [u8; 16],
}

static mut g: GlobalData =
    GlobalData {
        pagesize: 4096 as i64,
        usablesize: 4096 as i64,
        dbfd: -1,
        mx_page: 0 as u32,
        n_res: 0 as u32,
        per_line: 16,
        b_raw: 0,
        b_csv: 0,
        b_tmstmp: 0,
        p_fd: core::ptr::null_mut(),
        p_db: core::ptr::null_mut(),
        z_page_use: core::ptr::null_mut(),
        a_page_tag: core::ptr::null_mut(),
    };

///* Convert the var-int format into i64.  Return the number of bytes
///* in the var-int.  Write the var-int value into *pVal.
extern "C" fn decode_varint(z: *const u8, p_val_1: &mut i64) -> i32 {
    let mut v: i64 = 0 as i64;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b0: loop {
            if !(i < 8) { break '__b0; }
            '__c0: loop {
                v =
                    (v << 7) +
                        (unsafe { *z.offset(i as isize) } as i32 & 127) as i64;
                if unsafe { *z.offset(i as isize) } as i32 & 128 == 0 {
                    *p_val_1 = v;
                    return i + 1;
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    v = (v << 8) + (unsafe { *z.offset(i as isize) } as i32 & 255) as i64;
    *p_val_1 = v;
    return 9;
}

///* Extract a big-endian 32-bit integer
extern "C" fn decode_int32(z: *const u8) -> u32 {
    return (((unsafe { *z.offset(0 as isize) } as i32) << 24) +
                        ((unsafe { *z.offset(1 as isize) } as i32) << 16) +
                    ((unsafe { *z.offset(2 as isize) } as i32) << 8) +
                unsafe { *z.offset(3 as isize) } as i32) as u32;
}

/// Report an out-of-memory error and die.
extern "C" fn out_of_memory() -> () {
    eprintln!("Out of memory...");
    unsafe { exit(1) };
}

///* Open a database connection.
extern "C" fn open_database(z_prg_1: *const i8, z_name_1: *const i8)
    -> *mut Sqlite3 {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let flags: i32 = 2 | 64;
        let rc: i32 =
            unsafe {
                sqlite3_open_v2(z_name_1, &mut db, flags, core::ptr::null())
            };
        if rc != 0 {
            let z_err: *const i8 = unsafe { sqlite3_errmsg(db) };
            unsafe {
                fprintf(__stderrp,
                    c"%s: can\'t open %s (%s)\n".as_ptr() as *mut i8 as
                        *const i8, z_prg_1, z_name_1, z_err)
            };
            unsafe { sqlite3_close(db) };
            unsafe { exit(1) };
        }
        return db;
    }
}

///* Open the database file.
extern "C" fn file_open(z_prg_1: *const i8, z_name_1: *const i8) -> () {
    unsafe {
        if !(g.dbfd < 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fileOpen".as_ptr() as *const i8,
                    c"showdb.c".as_ptr() as *mut i8 as *const i8, 110,
                    c"g.dbfd<0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if g.b_raw == 0 {
            let mut rc: i32 = 0;
            let p_arg: *mut () = &raw mut g.p_fd as *mut ();
            g.p_db = open_database(z_prg_1, z_name_1);
            rc =
                unsafe {
                    sqlite3_file_control(g.p_db,
                        c"main".as_ptr() as *mut i8 as *const i8, 7, p_arg)
                };
            if rc != 0 {
                unsafe {
                    fprintf(__stderrp,
                        c"%s: failed to obtain fd for %s (SQLite too old?)\n".as_ptr()
                                as *mut i8 as *const i8, z_prg_1, z_name_1)
                };
                unsafe { exit(1) };
            }
        } else {
            g.dbfd = unsafe { open(z_name_1, 0) };
            if g.dbfd < 0 {
                unsafe {
                    fprintf(__stderrp,
                        c"%s: can\'t open %s\n".as_ptr() as *mut i8 as *const i8,
                        z_prg_1, z_name_1)
                };
                unsafe { exit(1) };
            }
        }
    }
}

///* Close the database file opened by fileOpen()
extern "C" fn file_close() -> () {
    unsafe {
        if g.b_raw == 0 {
            unsafe { sqlite3_close(g.p_db) };
            g.p_db = core::ptr::null_mut();
            g.p_fd = core::ptr::null_mut();
        } else { unsafe { close(g.dbfd) }; g.dbfd = -1; }
    }
}

///* Read content from the file.
///*
///* Space to hold the content is obtained from sqlite3_malloc() and needs 
///* to be freed by the caller.
extern "C" fn file_read(ofst: Sqlite3Int64, n_byte_1: i32) -> *mut u8 {
    unsafe {
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut got: i32 = 0;
        let mut rc: i32 = 0;
        a_data =
            unsafe {
                    sqlite3_malloc64((32 as i64 + n_byte_1 as i64) as
                            Sqlite3Uint64)
                } as *mut u8;
        if a_data == core::ptr::null_mut() { out_of_memory(); }
        unsafe { memset(a_data as *mut (), 0, (n_byte_1 + 32) as u64) };
        if g.b_raw == 0 {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*g.p_fd).p_methods }).x_read.unwrap()
                        })(g.p_fd, a_data as *mut (), n_byte_1, ofst)
                };
            if rc != 0 && rc != 10 | 2 << 8 {
                eprintln!("error in xRead() - {}", rc as i32);
                unsafe { exit(1) };
            }
        } else {
            unsafe { lseek(g.dbfd, ofst as i64, 0) };
            got =
                unsafe { read(g.dbfd, a_data as *mut (), n_byte_1 as u64) } as
                    i32;
            if got == n_byte_1 {
                rc = 0;
            } else if got > 0 && got < n_byte_1 {
                unsafe {
                    memset(unsafe { a_data.offset(got as isize) } as *mut (), 0,
                        (n_byte_1 - got) as u64)
                };
                rc = 10 | 2 << 8;
            } else {
                unsafe { memset(a_data as *mut (), 0, n_byte_1 as u64) };
                rc = 10;
            }
        }
        if !(g.a_page_tag).is_null() && n_byte_1 == g.pagesize as i32 {
            let pgno: u32 = (ofst / g.pagesize) as u32 + 1 as u32;
            if pgno > 0 as u32 && pgno <= g.mx_page {
                unsafe {
                    memcpy(unsafe {
                                    &raw mut (*g.a_page_tag.add(pgno as usize)).a[0 as usize]
                                } as *mut u8 as *mut (),
                        unsafe { &raw mut *a_data.offset((n_byte_1 - 16) as isize) }
                            as *const (), 16 as u64)
                };
            }
        }
        return a_data;
    }
}

///* Return the size of the file in byte.
extern "C" fn file_getsize() -> i64 {
    unsafe {
        let mut res: i64 = 0 as i64;
        if g.b_raw == 0 {
            let rc: i32 =
                unsafe {
                    (unsafe {
                            (*unsafe { (*g.p_fd).p_methods }).x_file_size.unwrap()
                        })(g.p_fd, &mut res)
                };
            if rc != 0 {
                eprintln!("error in xFileSize() - {}", rc as i32);
                unsafe { exit(1) };
            }
        } else {
            let mut sbuf: Stat = unsafe { core::mem::zeroed() };
            unsafe { fstat(g.dbfd, &mut sbuf) };
            res = sbuf.st_size as Sqlite3Int64;
        }
        return res;
    }
}

///* Print a range of bytes as hex and as ascii.
extern "C" fn print_byte_range(ofst: Sqlite3Int64, n_byte_1: i32,
    print_ofst_1: i32) -> *mut u8 {
    unsafe {
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut z_ofst_fmt: *const i8 = core::ptr::null();
        if print_ofst_1 + n_byte_1 & !4095 == 0 {
            z_ofst_fmt = c" %03x: ".as_ptr() as *mut i8 as *const i8;
        } else if print_ofst_1 + n_byte_1 & !65535 == 0 {
            z_ofst_fmt = c" %04x: ".as_ptr() as *mut i8 as *const i8;
        } else if print_ofst_1 + n_byte_1 & !1048575 == 0 {
            z_ofst_fmt = c" %05x: ".as_ptr() as *mut i8 as *const i8;
        } else if print_ofst_1 + n_byte_1 & !16777215 == 0 {
            z_ofst_fmt = c" %06x: ".as_ptr() as *mut i8 as *const i8;
        } else { z_ofst_fmt = c" %08x: ".as_ptr() as *mut i8 as *const i8; }
        a_data = file_read(ofst, n_byte_1);
        {
            i = 0;
            '__b1: loop {
                if !(i < n_byte_1) { break '__b1; }
                '__c1: loop {
                    let mut go: i32 = 0;
                    {
                        j = 0;
                        '__b2: loop {
                            if !(j < g.per_line) { break '__b2; }
                            '__c2: loop {
                                if i + j > n_byte_1 { break '__b2; }
                                if unsafe { *a_data.offset((i + j) as isize) } != 0 {
                                    go = 1;
                                    break '__b2;
                                }
                                break '__c2;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if (go == 0) as i32 != 0 && i > 0 &&
                            i + g.per_line < n_byte_1 {
                        break '__c1;
                    }
                    unsafe { fprintf(__stdoutp, z_ofst_fmt, i + print_ofst_1) };
                    {
                        j = 0;
                        '__b3: loop {
                            if !(j < g.per_line) { break '__b3; }
                            '__c3: loop {
                                if i + j > n_byte_1 {
                                    unsafe {
                                        fprintf(__stdoutp, c"   ".as_ptr() as *mut i8 as *const i8)
                                    };
                                } else {
                                    unsafe {
                                        fprintf(__stdoutp,
                                            c"%02x ".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *a_data.offset((i + j) as isize) } as i32)
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
                            if !(j < g.per_line) { break '__b4; }
                            '__c4: loop {
                                if i + j > n_byte_1 {
                                    unsafe {
                                        fprintf(__stdoutp, c" ".as_ptr() as *mut i8 as *const i8)
                                    };
                                } else {
                                    unsafe {
                                        fprintf(__stdoutp, c"%c".as_ptr() as *mut i8 as *const i8,
                                            if unsafe {
                                                        isprint(unsafe { *a_data.offset((i + j) as isize) } as u8 as
                                                                i32)
                                                    } != 0 {
                                                (unsafe { *a_data.offset((i + j) as isize) }) as i32
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
                    break '__c1;
                }
                i += g.per_line;
            }
        }
        return a_data;
    }
}

///* Print an entire page of content as hex
extern "C" fn print_page(i_pg_1: u32) -> () {
    unsafe {
        let mut i_start: i64 = 0 as i64;
        let mut a_data: *mut u8 = core::ptr::null_mut();
        i_start = (i_pg_1 - 1 as u32) as i64 * g.pagesize;
        unsafe {
            fprintf(__stdoutp,
                c"Page %u:   (offsets 0x%llx..0x%llx)\n".as_ptr() as *mut i8
                    as *const i8, i_pg_1, i_start,
                i_start + g.pagesize - 1 as i64)
        };
        a_data = print_byte_range(i_start, g.pagesize as i32, 0);
        unsafe { sqlite3_free(a_data as *mut ()) };
    }
}

/// Print a line of decoded output showing a 4-byte unsigned integer.
extern "C" fn print_decode_line(a_data_1: *const u8, ofst: i32, n_byte_1: i32,
    z_msg_1: *const i8) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut val: u32 = unsafe { *a_data_1.offset(ofst as isize) } as u32;
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
                        val * 256 as u32 +
                            unsafe { *a_data_1.offset((ofst + j) as isize) } as u32;
                }
                i +=
                    unsafe { strlen(&raw mut z_buf[i as usize] as *const i8) }
                        as i32;
                break '__c5;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        sprintf(&mut z_buf[i as usize],
            c"   %10u".as_ptr() as *mut i8 as *const i8, val)
    };
    unsafe {
        printf(c"%s  %s\n".as_ptr() as *mut i8 as *const i8,
            &raw mut z_buf[0 as usize] as *mut i8, z_msg_1)
    };
}

///* Decode the database header.
extern "C" fn print_db_header() -> () {
    let mut a_data: *mut u8 = core::ptr::null_mut();
    a_data = print_byte_range(0 as Sqlite3Int64, 100, 0);
    unsafe { printf(c"Decoded:\n".as_ptr() as *mut i8 as *const i8) };
    print_decode_line(a_data as *const u8, 16, 2,
        c"Database page size".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 18, 1,
        c"File format write version".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 19, 1,
        c"File format read version".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 20, 1,
        c"Reserved space at end of page".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 24, 4,
        c"File change counter".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 28, 4,
        c"Size of database in pages".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 32, 4,
        c"Page number of first freelist page".as_ptr() as *mut i8 as
            *const i8);
    print_decode_line(a_data as *const u8, 36, 4,
        c"Number of freelist pages".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 40, 4,
        c"Schema cookie".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 44, 4,
        c"Schema format version".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 48, 4,
        c"Default page cache size".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 52, 4,
        c"Largest auto-vac root page".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 56, 4,
        c"Text encoding".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 60, 4,
        c"User version".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 64, 4,
        c"Incremental-vacuum mode".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 68, 4,
        c"Application ID".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 72, 4,
        c"meta[8]".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 76, 4,
        c"meta[9]".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 80, 4,
        c"meta[10]".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 84, 4,
        c"meta[11]".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 88, 4,
        c"meta[12]".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data as *const u8, 92, 4,
        c"Change counter for version number".as_ptr() as *mut i8 as
            *const i8);
    print_decode_line(a_data as *const u8, 96, 4,
        c"SQLite version number".as_ptr() as *mut i8 as *const i8);
    unsafe { sqlite3_free(a_data as *mut ()) };
}

///* Describe cell content.
extern "C" fn describe_content(mut a: *mut u8, mut n_local_1: i64,
    mut z_desc_1: *mut i8) -> i64 {
    let mut n_desc: i64 = 0 as i64;
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
        n_desc += j as i64;
    }
    return n_desc;
}

///* Compute the local payload size given the total payload size and
///* the page size.
#[allow(unused_doc_comments)]
extern "C" fn local_payload(n_payload_1: i64, c_type_1: i8) -> i64 {
    unsafe {
        let mut max_local: i64 = 0 as i64;
        let mut min_local: i64 = 0 as i64;
        let mut surplus: i64 = 0 as i64;
        let mut n_local: i64 = 0 as i64;
        if c_type_1 as i32 == 13 {

            /// Table leaf
            (max_local = g.usablesize - 35 as i64);
            min_local =
                (g.usablesize - 12 as i64) * 32 as i64 / 255 as i64 -
                    23 as i64;
        } else {
            max_local =
                (g.usablesize - 12 as i64) * 64 as i64 / 255 as i64 -
                    23 as i64;
            min_local =
                (g.usablesize - 12 as i64) * 32 as i64 / 255 as i64 -
                    23 as i64;
        }
        if n_payload_1 > max_local {
            surplus =
                min_local +
                    (n_payload_1 - min_local) % (g.usablesize - 4 as i64);
            if surplus <= max_local {
                n_local = surplus;
            } else { n_local = min_local; }
        } else { n_local = n_payload_1; }
        return n_local;
    }
}

///* Create a description for a single cell.
///*
///* The return value is the local cell size.
extern "C" fn describe_cell(c_type_1: u8, mut a: *mut u8,
    show_cell_content_1: i32, pz_desc_1: &mut *mut i8) -> i64 {
    unsafe {
        let mut i: i32 = 0;
        let mut n_desc: i64 = 0 as i64;
        let mut n: i32 = 0;
        let mut left_child: u32 = 0 as u32;
        let mut n_payload: i64 = 0 as i64;
        let mut rowid: i64 = 0 as i64;
        let mut n_local: i64 = 0 as i64;
        i = 0;
        if c_type_1 as i32 <= 5 {
            left_child =
                (((unsafe { *a.offset(0 as isize) } as i32 * 256 +
                                        unsafe { *a.offset(1 as isize) } as i32) * 256 +
                                unsafe { *a.offset(2 as isize) } as i32) * 256 +
                        unsafe { *a.offset(3 as isize) } as i32) as u32;
            {
                let __n = 4;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += 4;
            unsafe {
                sprintf(&raw mut z_desc_2[0 as usize] as *mut i8,
                    c"lx: %u ".as_ptr() as *mut i8 as *const i8, left_child)
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
            let mut ovfl: u32 = 0 as u32;
            let b: *const u8 =
                unsafe { &raw mut *a.offset(n_local as isize) } as *const u8;
            ovfl =
                (((unsafe { *b.offset(0 as isize) } as i32 * 256 +
                                        unsafe { *b.offset(1 as isize) } as i32) * 256 +
                                unsafe { *b.offset(2 as isize) } as i32) * 256 +
                        unsafe { *b.offset(3 as isize) } as i32) as u32;
            unsafe {
                sprintf(&mut z_desc_2[n_desc as usize],
                    c"ov: %u ".as_ptr() as *mut i8 as *const i8, ovfl)
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

/// Print an offset followed by nByte bytes.  Add extra white-space
///* at the end so that subsequent text is aligned.
extern "C" fn print_bytes(a_data_1: *mut u8, a_start_1: *mut u8,
    n_byte_1: i32) -> () {
    let mut j: i32 = 0;
    unsafe {
        printf(c" %03x: ".as_ptr() as *mut i8 as *const i8,
            unsafe { a_start_1.offset_from(a_data_1) } as i64 as i32)
    };
    {
        j = 0;
        '__b8: loop {
            if !(j < 9) { break '__b8; }
            '__c8: loop {
                if j >= n_byte_1 {
                    unsafe { printf(c"   ".as_ptr() as *mut i8 as *const i8) };
                } else {
                    unsafe {
                        printf(c"%02x ".as_ptr() as *mut i8 as *const i8,
                            unsafe { *a_start_1.offset(j as isize) } as i32)
                    };
                }
                break '__c8;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Write a full decode on stdout for the cell at a[ofst].
///* Assume the page contains a header of size szPgHdr bytes.
extern "C" fn decode_cell(a: *mut u8, pgno: u32, i_cell_1: i32,
    sz_pg_hdr_1: i32, ofst: i32) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut left_child: u32 = 0 as u32;
    let mut k: i64 = 0 as i64;
    let mut n_payload: i64 = 0 as i64;
    let mut rowid: i64 = 0 as i64;
    let mut n_hdr: i64 = 0 as i64;
    let mut i_type: i64 = 0 as i64;
    let mut n_local: i64 = 0 as i64;
    let mut x: *mut u8 = unsafe { a.offset(ofst as isize) };
    let mut end: *mut u8 = core::ptr::null_mut();
    let c_type: u8 = unsafe { *a.offset(0 as isize) };
    let mut n_col: i32 = 0;
    let mut sz_col: [i32; 2000] = [0; 2000];
    let mut ofst_col: [i32; 2000] = [0; 2000];
    let mut type_col: [i32; 2000] = [0; 2000];
    unsafe {
        printf(c"Cell[%d]:\n".as_ptr() as *mut i8 as *const i8, i_cell_1)
    };
    if c_type as i32 <= 5 {
        left_child =
            (((unsafe { *x.offset(0 as isize) } as i32 * 256 +
                                    unsafe { *x.offset(1 as isize) } as i32) * 256 +
                            unsafe { *x.offset(2 as isize) } as i32) * 256 +
                    unsafe { *x.offset(3 as isize) } as i32) as u32;
        print_bytes(a, x, 4);
        unsafe {
            printf(c"left child page:: %u\n".as_ptr() as *mut i8 as *const i8,
                left_child)
        };
        {
            let __n = 4;
            let __p = &mut x;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    if c_type as i32 != 5 {
        i = decode_varint(x as *const u8, &mut n_payload);
        print_bytes(a, x, i);
        n_local = local_payload(n_payload, c_type as i8);
        if n_local == n_payload {
            unsafe {
                printf(c"payload-size: %lld\n".as_ptr() as *mut i8 as
                        *const i8, n_payload)
            };
        } else {
            unsafe {
                printf(c"payload-size: %lld (%lld local, %lld overflow)\n".as_ptr()
                            as *mut i8 as *const i8, n_payload, n_local,
                    n_payload - n_local)
            };
        }
        {
            let __n = i;
            let __p = &mut x;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    } else { n_payload = { n_local = 0 as i64; n_local }; }
    end = unsafe { x.offset(n_local as isize) };
    if c_type as i32 == 5 || c_type as i32 == 13 {
        i = decode_varint(x as *const u8, &mut rowid);
        print_bytes(a, x, i);
        unsafe {
            printf(c"rowid: %lld\n".as_ptr() as *mut i8 as *const i8, rowid)
        };
        {
            let __n = i;
            let __p = &mut x;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    if n_local > 0 as i64 {
        i = decode_varint(x as *const u8, &mut n_hdr);
        print_bytes(a, x, i);
        unsafe {
            printf(c"record-header-size: %d\n".as_ptr() as *mut i8 as
                    *const i8, n_hdr as i32)
        };
        j = i;
        n_col = 0;
        k = n_hdr;
        while unsafe { x.offset(j as isize) } <= end && (j as i64) < n_hdr {
            let mut z_type_name: *const i8 = core::ptr::null();
            let mut sz: i32 = 0;
            let mut z_nm: [i8; 30] = [0; 30];
            i =
                decode_varint(unsafe { x.offset(j as isize) } as *const u8,
                    &mut i_type);
            print_bytes(a, unsafe { x.offset(j as isize) }, i);
            unsafe {
                printf(c"typecode[%d]: %d - ".as_ptr() as *mut i8 as
                        *const i8, n_col, i_type as i32)
            };
            '__s10:
                {
                match i_type {
                    0 => {
                        z_type_name = c"NULL".as_ptr() as *mut i8 as *const i8;
                        sz = 0;
                    }
                    1 => {
                        z_type_name = c"int8".as_ptr() as *mut i8 as *const i8;
                        sz = 1;
                    }
                    2 => {
                        z_type_name = c"int16".as_ptr() as *mut i8 as *const i8;
                        sz = 2;
                    }
                    3 => {
                        z_type_name = c"int24".as_ptr() as *mut i8 as *const i8;
                        sz = 3;
                    }
                    4 => {
                        z_type_name = c"int32".as_ptr() as *mut i8 as *const i8;
                        sz = 4;
                    }
                    5 => {
                        z_type_name = c"int48".as_ptr() as *mut i8 as *const i8;
                        sz = 6;
                    }
                    6 => {
                        z_type_name = c"int64".as_ptr() as *mut i8 as *const i8;
                        sz = 8;
                    }
                    7 => {
                        z_type_name = c"double".as_ptr() as *mut i8 as *const i8;
                        sz = 8;
                    }
                    8 => {
                        z_type_name = c"zero".as_ptr() as *mut i8 as *const i8;
                        sz = 0;
                    }
                    9 => {
                        z_type_name = c"one".as_ptr() as *mut i8 as *const i8;
                        sz = 0;
                    }
                    10 => {
                        z_type_name = c"error".as_ptr() as *mut i8 as *const i8;
                        sz = 0;
                    }
                    11 => {
                        z_type_name = c"error".as_ptr() as *mut i8 as *const i8;
                        sz = 0;
                    }
                    _ => {
                        {
                            sz = (i_type - 12 as i64) as i32 / 2;
                            unsafe {
                                sprintf(&raw mut z_nm[0 as usize] as *mut i8,
                                    if i_type & 1 as i64 == 0 as i64 {
                                            c"blob(%d)".as_ptr() as *mut i8
                                        } else { c"text(%d)".as_ptr() as *mut i8 } as *const i8, sz)
                            };
                            z_type_name =
                                &raw mut z_nm[0 as usize] as *mut i8 as *const i8;
                            break '__s10;
                        }
                    }
                }
            }
            unsafe {
                printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_type_name)
            };
            sz_col[n_col as usize] = sz;
            ofst_col[n_col as usize] = k as i32;
            type_col[n_col as usize] = i_type as i32;
            k += sz as i64;
            { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
            j += i;
        }
        {
            i = 0;
            '__b11: loop {
                if !(i < n_col &&
                                (ofst_col[i as usize] + sz_col[i as usize]) as i64 <=
                                    n_local) {
                    break '__b11;
                }
                '__c11: loop {
                    let s: i32 = ofst_col[i as usize];
                    let mut v: i64 = 0 as i64;
                    let mut p_data: *const u8 = core::ptr::null();
                    if sz_col[i as usize] == 0 { break '__c11; }
                    print_bytes(a, unsafe { x.offset(s as isize) },
                        sz_col[i as usize]);
                    unsafe {
                        printf(c"data[%d]: ".as_ptr() as *mut i8 as *const i8, i)
                    };
                    p_data = unsafe { x.offset(s as isize) } as *const u8;
                    if type_col[i as usize] <= 7 {
                        v = unsafe { *p_data.offset(0 as isize) } as i8 as i64;
                        {
                            k = 1 as i64;
                            '__b12: loop {
                                if !(k < sz_col[i as usize] as i64) { break '__b12; }
                                '__c12: loop {
                                    v = (v << 8) + unsafe { *p_data.offset(k as isize) } as i64;
                                    break '__c12;
                                }
                                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if type_col[i as usize] == 7 {
                            let mut r: f64 = 0.0;
                            unsafe {
                                memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                    core::mem::size_of::<f64>() as u64)
                            };
                            unsafe {
                                printf(c"%#g\n".as_ptr() as *mut i8 as *const i8, r)
                            };
                        } else {
                            unsafe {
                                printf(c"%lld\n".as_ptr() as *mut i8 as *const i8, v)
                            };
                        }
                    } else {
                        let mut ii: i32 = 0;
                        let mut jj: i32 = 0;
                        let mut z_const: [i8; 32] = [0; 32];
                        if type_col[i as usize] & 1 == 0 {
                            z_const[0 as usize] = 'x' as i32 as i8;
                            z_const[1 as usize] = '\'' as i32 as i8;
                            {
                                { ii = 2; jj = 0 };
                                '__b13: loop {
                                    if !(jj < sz_col[i as usize] && ii < 24) { break '__b13; }
                                    '__c13: loop {
                                        unsafe {
                                            sprintf(unsafe {
                                                    (&raw mut z_const[0 as usize] as
                                                            *mut i8).offset(ii as isize)
                                                }, c"%02x".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *p_data.offset(jj as isize) } as i32)
                                        };
                                        break '__c13;
                                    }
                                    {
                                        { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                                        ii += 2
                                    };
                                }
                            }
                        } else {
                            z_const[0 as usize] = '\'' as i32 as i8;
                            {
                                { ii = 1; jj = 0 };
                                '__b14: loop {
                                    if !(jj < sz_col[i as usize] && ii < 24) { break '__b14; }
                                    '__c14: loop {
                                        z_const[ii as usize] =
                                            if unsafe {
                                                            isprint(unsafe { *p_data.offset(jj as isize) } as u8 as i32)
                                                        } != 0 {
                                                    (unsafe { *p_data.offset(jj as isize) }) as i32
                                                } else { '.' as i32 } as i8;
                                        break '__c14;
                                    }
                                    {
                                        { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t }
                                    };
                                }
                            }
                            z_const[ii as usize] = 0 as i8;
                        }
                        if jj < sz_col[i as usize] {
                            unsafe {
                                memcpy(unsafe {
                                            (&raw mut z_const[0 as usize] as
                                                    *mut i8).offset(ii as isize)
                                        } as *mut (), c"...\'".as_ptr() as *mut i8 as *const (),
                                    5 as u64)
                            };
                        } else {
                            unsafe {
                                memcpy(unsafe {
                                            (&raw mut z_const[0 as usize] as
                                                    *mut i8).offset(ii as isize)
                                        } as *mut (), c"\'".as_ptr() as *mut i8 as *const (),
                                    2 as u64)
                            };
                        }
                        unsafe {
                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                                &raw mut z_const[0 as usize] as *mut i8)
                        };
                    }
                    j = ofst_col[i as usize] + sz_col[i as usize];
                    break '__c11;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if (j as i64) < n_local {
        print_bytes(a, unsafe { x.offset(j as isize) }, 0);
        unsafe {
            printf(c"... %lld bytes of content ...\n".as_ptr() as *mut i8 as
                    *const i8, n_local - j as i64)
        };
    }
    if n_local < n_payload {
        print_bytes(a, unsafe { x.offset(n_local as isize) }, 4);
        unsafe {
            printf(c"overflow-page: %u\n".as_ptr() as *mut i8 as *const i8,
                decode_int32(unsafe { x.offset(n_local as isize) } as
                        *const u8))
        };
    }
}

///* Decode a btree page
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
        let mut cell_to_decode: i32 = -2;
        let mut z_map: *mut i8 = core::ptr::null_mut();
        '__s15:
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
            '__s17:
                {
                match unsafe { *z_args_1.offset(0 as isize) } {
                    99 => { show_cell_content = 1; }
                    109 => { show_map = 1; }
                    100 => {
                        {
                            if (unsafe {
                                                isdigit(unsafe { *z_args_1.offset(1 as isize) } as u8 as
                                                        i32)
                                            } == 0) as i32 != 0 {
                                cell_to_decode = -1;
                            } else {
                                cell_to_decode = 0;
                                while unsafe {
                                            isdigit(unsafe { *z_args_1.offset(1 as isize) } as u8 as
                                                    i32)
                                        } != 0 {
                                    {
                                        let __p = &mut z_args_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                    cell_to_decode =
                                        cell_to_decode * 10 +
                                                unsafe { *z_args_1.offset(0 as isize) } as i32 - '0' as i32;
                                }
                            }
                            break '__s17;
                        }
                    }
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
        n_cell =
            unsafe { *a.offset(3 as isize) } as i32 * 256 +
                unsafe { *a.offset(4 as isize) } as i32;
        i_cell_ptr =
            if unsafe { *a.offset(0 as isize) } as i32 == 2 ||
                    unsafe { *a.offset(0 as isize) } as i32 == 5 {
                12
            } else { 8 };
        if cell_to_decode >= n_cell {
            unsafe {
                printf(c"Page %d has only %d cells\n".as_ptr() as *mut i8 as
                        *const i8, pgno, n_cell)
            };
            return;
        }
        unsafe {
            printf(c"Header on btree page %d:\n".as_ptr() as *mut i8 as
                    *const i8, pgno)
        };
        print_decode_line(a as *const u8, 0, 1, z_type);
        print_decode_line(a as *const u8, 1, 2,
            c"Offset to first freeblock".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a as *const u8, 3, 2,
            c"Number of cells on this page".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a as *const u8, 5, 2,
            c"Offset to cell content area".as_ptr() as *mut i8 as *const i8);
        print_decode_line(a as *const u8, 7, 1,
            c"Fragmented byte count".as_ptr() as *mut i8 as *const i8);
        if unsafe { *a.offset(0 as isize) } as i32 == 2 ||
                unsafe { *a.offset(0 as isize) } as i32 == 5 {
            print_decode_line(a as *const u8, 8, 4,
                c"Right child".as_ptr() as *mut i8 as *const i8);
        }
        if cell_to_decode == -2 && n_cell > 0 {
            unsafe {
                printf(c" key: lx=left-child n=payload-size r=rowid\n".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        if show_map != 0 {
            z_map = unsafe { sqlite3_malloc(g.pagesize as i32) } as *mut i8;
            unsafe {
                memset(z_map as *mut (), '.' as i32, g.pagesize as u64)
            };
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
            '__b19: loop {
                if !(i < n_cell) { break '__b19; }
                '__c19: loop {
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
                    if cell_to_decode == -2 {
                        unsafe {
                            printf(c" %03x: cell[%d] %s\n".as_ptr() as *mut i8 as
                                    *const i8, cofst, i, z_desc)
                        };
                    } else if cell_to_decode == -1 || cell_to_decode == i {
                        decode_cell(a, pgno as u32, i, hdr_size_1,
                            cofst - hdr_size_1);
                    }
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if show_map != 0 {
            unsafe {
                printf(c"Page map:  (H=header P=cell-index 1=page-1-header .=free-space)\n".as_ptr()
                            as *mut i8 as *const i8)
            };
            {
                i = 0;
                '__b20: loop {
                    if !((i as u32 as i64) < g.pagesize) { break '__b20; }
                    '__c20: loop {
                        unsafe {
                            printf(c" %03x: %.64s\n".as_ptr() as *mut i8 as *const i8,
                                i, unsafe { &raw mut *z_map.offset(i as isize) } as *mut i8)
                        };
                        break '__c20;
                    }
                    i += 64;
                }
            }
            unsafe { sqlite3_free(z_map as *mut ()) };
        }
    }
}

///* Decode a freelist trunk page.
extern "C" fn decode_trunk_page(mut pgno: u32, detail: i32, recursive: i32)
    -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut a: *mut u8 = core::ptr::null_mut();
        while pgno > 0 as u32 {
            a =
                file_read((pgno - 1 as u32) as i64 * g.pagesize,
                    g.pagesize as i32);
            unsafe {
                printf(c"Decode of freelist trunk page %d:\n".as_ptr() as
                            *mut i8 as *const i8, pgno)
            };
            print_decode_line(a as *const u8, 0, 4,
                c"Next freelist trunk page".as_ptr() as *mut i8 as *const i8);
            print_decode_line(a as *const u8, 4, 4,
                c"Number of entries on this page".as_ptr() as *mut i8 as
                    *const i8);
            if detail != 0 {
                n =
                    decode_int32(unsafe { &raw mut *a.offset(4 as isize) } as
                            *const u8);
                {
                    i = 0 as u32;
                    '__b22: loop {
                        if !(i < n && (i as i64) < g.usablesize / 4 as i64) {
                            break '__b22;
                        }
                        '__c22: loop {
                            let x: u32 =
                                decode_int32(unsafe {
                                            &raw mut *a.add((8 as u32 + 4 as u32 * i) as usize)
                                        } as *const u8);
                            let mut z_idx: [i8; 13] = [0; 13];
                            unsafe {
                                sprintf(&raw mut z_idx[0 as usize] as *mut i8,
                                    c"[%d]".as_ptr() as *mut i8 as *const i8, i)
                            };
                            unsafe {
                                printf(c"  %5s %7u".as_ptr() as *mut i8 as *const i8,
                                    &raw mut z_idx[0 as usize] as *mut i8, x)
                            };
                            if i % 5 as u32 == 4 as u32 {
                                unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                            }
                            break '__c22;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if i % 5 as u32 != 0 as u32 {
                    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                }
            }
            if (recursive == 0) as i32 != 0 {
                pgno = 0 as u32;
            } else {
                pgno =
                    decode_int32(unsafe { &raw mut *a.offset(0 as isize) } as
                            *const u8);
            }
            unsafe { sqlite3_free(a as *mut ()) };
        }
    }
}

///* Add a comment on the use of a page.
unsafe extern "C" fn page_usage_msg(pgno: u32, z_format_1: *const i8,
    mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_msg: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_msg = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        if pgno <= 0 as u32 || pgno > g.mx_page {
            unsafe {
                printf(c"ERROR: page %d out of range 1..%u: %s\n".as_ptr() as
                            *mut i8 as *const i8, pgno, g.mx_page, z_msg)
            };
            unsafe { sqlite3_free(z_msg as *mut ()) };
            return;
        }
        if unsafe { *g.z_page_use.add(pgno as usize) } !=
                core::ptr::null_mut() {
            unsafe {
                printf(c"ERROR: page %d used multiple times:\n".as_ptr() as
                            *mut i8 as *const i8, pgno)
            };
            unsafe {
                printf(c"ERROR:    previous: %s\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { *g.z_page_use.add(pgno as usize) })
            };
            unsafe {
                printf(c"ERROR:    current:  %s\n".as_ptr() as *mut i8 as
                        *const i8, z_msg)
            };
            unsafe {
                sqlite3_free(unsafe { *g.z_page_use.add(pgno as usize) } as
                        *mut ())
            };
        }
        unsafe { *g.z_page_use.add(pgno as usize) = z_msg };
    }
}

///* Find overflow pages of a cell and describe their usage.
extern "C" fn page_usage_cell(c_type_1: u8, mut a: *mut u8, pgno: u32,
    cellno: i32) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut n_payload: i64 = 0 as i64;
        let mut rowid: i64 = 0 as i64;
        let mut n_local: i64 = 0 as i64;
        i = 0;
        if c_type_1 as i32 <= 5 {
            {
                let __n = 4;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += 4;
        }
        if c_type_1 as i32 != 5 {
            i = decode_varint(a as *const u8, &mut n_payload);
            {
                let __n = i;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n += i;
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
        }
        if n_local < n_payload {
            let mut ovfl: u32 =
                decode_int32(unsafe { a.offset(n_local as isize) } as
                        *const u8);
            let mut cnt: u32 = 0 as u32;
            while ovfl != 0 &&
                    { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } <
                        g.mx_page {
                unsafe {
                    page_usage_msg(ovfl,
                        c"overflow %d from cell %d of page %u".as_ptr() as *mut i8
                            as *const i8, cnt, cellno, pgno)
                };
                a =
                    file_read((ovfl - 1 as u32) as Sqlite3Int64 *
                            g.pagesize as Sqlite3Int64, g.pagesize as i32);
                ovfl = decode_int32(a as *const u8);
                unsafe { sqlite3_free(a as *mut ()) };
            }
        }
    }
}

///* True if the memory is all zeros
extern "C" fn all_zero(mut a: *const u8, mut n: i32) -> i32 {
    while n != 0 &&
            unsafe {
                        *{
                                    let __p = &mut a;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }.offset(0 as isize)
                    } as i32 == 0 {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    return (n == 0) as i32;
}

///* Describe the usages of a b-tree page.
///*
///* If parent==0, then this is the root of a btree.  If parent<0 then
///* this is an orphan page.
extern "C" fn page_usage_btree(pgno: u32, parent: i32, idx: i32,
    z_name_1: *const i8) -> () {
    unsafe {
        let mut a: *mut u8 = core::ptr::null_mut();
        let mut z_type: *const i8 =
            c"corrupt node".as_ptr() as *mut i8 as *const i8;
        let mut n_cell: i32 = 0;
        let mut i: i32 = 0;
        let hdr: i32 = if pgno == 1 as u32 { 100 } else { 0 };
        let mut z_entry: [i8; 30] = [0; 30];
        if pgno <= 0 as u32 || pgno > g.mx_page { return; }
        a =
            file_read((pgno - 1 as u32) as i64 * g.pagesize,
                g.pagesize as i32);
        '__s25:
            {
            match unsafe { *a.offset(hdr as isize) } {
                0 => {
                    {
                        if all_zero(a as *const u8, g.pagesize as i32) != 0 {
                            z_type = c"zeroed page".as_ptr() as *mut i8 as *const i8;
                        } else if parent < 0 {
                            return;
                        } else {
                            z_type = c"corrupt node".as_ptr() as *mut i8 as *const i8;
                        }
                        break '__s25;
                    }
                    z_type =
                        c"interior node of index".as_ptr() as *mut i8 as *const i8;
                }
                2 => {
                    z_type =
                        c"interior node of index".as_ptr() as *mut i8 as *const i8;
                }
                5 => {
                    z_type =
                        c"interior node of table".as_ptr() as *mut i8 as *const i8;
                }
                10 => {
                    z_type = c"leaf of index".as_ptr() as *mut i8 as *const i8;
                }
                13 => {
                    z_type = c"leaf of table".as_ptr() as *mut i8 as *const i8;
                }
                _ => {
                    {
                        if parent < 0 { return; }
                        z_type = c"corrupt node".as_ptr() as *mut i8 as *const i8;
                    }
                }
            }
        }
        n_cell =
            unsafe { *a.offset((hdr + 3) as isize) } as i32 * 256 +
                unsafe { *a.offset((hdr + 4) as isize) } as i32;
        if n_cell == 1 {
            unsafe {
                sqlite3_snprintf(core::mem::size_of::<[i8; 30]>() as i32,
                    &raw mut z_entry[0 as usize] as *mut i8,
                    c"1 row".as_ptr() as *mut i8 as *const i8)
            };
        } else {
            unsafe {
                sqlite3_snprintf(core::mem::size_of::<[i8; 30]>() as i32,
                    &raw mut z_entry[0 as usize] as *mut i8,
                    c"%d rows".as_ptr() as *mut i8 as *const i8, n_cell)
            };
        }
        if parent > 0 {
            unsafe {
                page_usage_msg(pgno,
                    c"%s [%s], child %d of page %d, %s".as_ptr() as *mut i8 as
                        *const i8, z_type, z_name_1, idx, parent,
                    &raw mut z_entry[0 as usize] as *mut i8)
            };
        } else if parent == 0 {
            unsafe {
                page_usage_msg(pgno,
                    c"root %s [%s], %s".as_ptr() as *mut i8 as *const i8,
                    z_type, z_name_1, &raw mut z_entry[0 as usize] as *mut i8)
            };
        } else {
            unsafe {
                page_usage_msg(pgno,
                    c"orphaned %s, %s".as_ptr() as *mut i8 as *const i8, z_type,
                    &raw mut z_entry[0 as usize] as *mut i8)
            };
        }
        if unsafe { *a.offset(hdr as isize) } as i32 == 2 ||
                unsafe { *a.offset(hdr as isize) } as i32 == 5 {
            let cellstart: i32 = hdr + 12;
            let mut child: u32 = 0 as u32;
            {
                i = 0;
                '__b26: loop {
                    if !(i < n_cell) { break '__b26; }
                    '__c26: loop {
                        let mut cellidx: u32 = 0 as u32;
                        let mut ofst: u32 = 0 as u32;
                        cellidx = (cellstart + i * 2) as u32;
                        if (cellidx + 1 as u32) as i64 >= g.usablesize {
                            unsafe {
                                printf(c"ERROR: page %d too many cells (%d)\n".as_ptr() as
                                            *mut i8 as *const i8, pgno, n_cell)
                            };
                            break '__b26;
                        }
                        ofst =
                            (unsafe { *a.add(cellidx as usize) } as i32 * 256 +
                                    unsafe { *a.add((cellidx + 1 as u32) as usize) } as i32) as
                                u32;
                        if ofst < cellidx + 2 as u32 ||
                                (ofst + 4 as u32) as i64 >= g.usablesize {
                            unsafe {
                                printf(c"ERROR: page %d cell %d out of bounds\n".as_ptr() as
                                            *mut i8 as *const i8, pgno, i)
                            };
                            break '__c26;
                        }
                        child =
                            decode_int32(unsafe { a.add(ofst as usize) } as *const u8);
                        page_usage_btree(child, pgno as i32, i, z_name_1);
                        break '__c26;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            child =
                decode_int32(unsafe {
                            unsafe {
                                a.offset(cellstart as isize).offset(-(4 as isize))
                            }
                        } as *const u8);
            page_usage_btree(child, pgno as i32, i, z_name_1);
        }
        if unsafe { *a.offset(hdr as isize) } as i32 == 2 ||
                    unsafe { *a.offset(hdr as isize) } as i32 == 10 ||
                unsafe { *a.offset(hdr as isize) } as i32 == 13 {
            let cellstart: i32 =
                hdr + 8 +
                    4 * (unsafe { *a.offset(hdr as isize) } as i32 <= 5) as i32;
            {
                i = 0;
                '__b27: loop {
                    if !(i < n_cell) { break '__b27; }
                    '__c27: loop {
                        let mut ofst: i32 = 0;
                        ofst = cellstart + i * 2;
                        ofst =
                            unsafe { *a.offset(ofst as isize) } as i32 * 256 +
                                unsafe { *a.offset((ofst + 1) as isize) } as i32;
                        page_usage_cell(unsafe { *a.offset(hdr as isize) },
                            unsafe { a.offset(ofst as isize) }, pgno, i);
                        break '__c27;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        unsafe { sqlite3_free(a as *mut ()) };
    }
}

///* Determine page usage by the freelist
extern "C" fn page_usage_freelist(mut pgno: u32) -> () {
    unsafe {
        let mut a: *mut u8 = core::ptr::null_mut();
        let mut cnt: i32 = 0;
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut i_next: i32 = 0;
        let mut parent: i32 = 1;
        while pgno > 0 as u32 && pgno <= g.mx_page &&
                ({ let __p = &mut cnt; let __t = *__p; *__p += 1; __t } as
                            u32) < g.mx_page {
            unsafe {
                page_usage_msg(pgno,
                    c"freelist trunk #%d child of %d".as_ptr() as *mut i8 as
                        *const i8, cnt, parent)
            };
            a =
                file_read((pgno - 1 as u32) as i64 * g.pagesize,
                    g.pagesize as i32);
            i_next = decode_int32(a as *const u8) as i32;
            n =
                decode_int32(unsafe { a.offset(4 as isize) } as *const u8) as
                    i32;
            if n as i64 > (g.usablesize - 8 as i64) / 4 as i64 {
                unsafe {
                    printf(c"ERROR: page %d too many freelist entries (%d)\n".as_ptr()
                                as *mut i8 as *const i8, pgno, n)
                };
                n = ((g.usablesize - 8 as i64) / 4 as i64) as i32;
            }
            {
                i = 0;
                '__b29: loop {
                    if !(i < n) { break '__b29; }
                    '__c29: loop {
                        let child: i32 =
                            decode_int32(unsafe { a.offset((i * 4 + 8) as isize) } as
                                        *const u8) as i32;
                        unsafe {
                            page_usage_msg(child as u32,
                                c"freelist leaf, child %d of trunk page %d".as_ptr() as
                                        *mut i8 as *const i8, i, pgno)
                        };
                        break '__c29;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { sqlite3_free(a as *mut ()) };
            parent = pgno as i32;
            pgno = i_next as u32;
        }
    }
}

///* Determine pages used as PTRMAP pages
extern "C" fn page_usage_ptrmap(a: *const u8) -> () {
    unsafe {
        if decode_int32(unsafe { a.offset(52 as isize) } as *const u8) != 0 {
            let usable: i32 =
                (g.pagesize - unsafe { *a.offset(20 as isize) } as i64) as
                    i32;
            let mut pgno: u64 = 2 as u64;
            let per_page: i32 = usable / 5;
            while pgno <= g.mx_page as u64 {
                unsafe {
                    page_usage_msg(pgno as u32,
                        c"PTRMAP page covering %llu..%llu".as_ptr() as *mut i8 as
                            *const i8, pgno + 1 as u64, pgno + per_page as u64)
                };
                pgno += (per_page + 1) as u64;
            }
        }
    }
}

///* The six bytes at a[] are a big-endian unsigned integer which is the
///* number of milliseconds since 1970.  Decode that value into an ISO 8601
///* date/time string stored in static space and return a pointer to that
///* string.
#[allow(unused_doc_comments)]
extern "C" fn decode_timestamp(a: *const u8) -> *const i8 {
    unsafe {
        let mut ms: u64 = 0 as u64;
        /// Milliseconds since 1970
        let mut days: u64 = 0 as u64;
        /// Days since 1970-01-01
        let mut sod: u64 = 0 as u64;
        /// Start of date specified by ms
        let mut z: u64 = 0 as u64;
        /// Days since 0000-03-01
        let mut era: u64 = 0 as u64;
        /// 400-year era
        let mut i: i32 = 0;
        /// Loop counter
        let mut h: i32 = 0;
        /// hour
        let mut m: i32 = 0;
        /// minute
        let mut s: i32 = 0;
        /// second
        let mut f: i32 = 0;
        /// millisecond
        let mut y: i32 = 0;
        /// year
        let mut m: i32 = 0;
        /// month
        let mut d: i32 = 0;
        /// day
        let mut y: i32 = 0;
        /// year assuming March is first month
        let mut doe: u32 = 0 as u32;
        /// day of 400-year era
        let mut yoe: u32 = 0 as u32;
        /// year of 400-year era
        let mut doy: u32 = 0 as u32;
        /// day of year
        let mut mp: u32 = 0 as u32;
        {
            { ({ ms = 0 as u64; ms }) as i32; i = 0 };
            '__b31: loop {
                if !(i <= 5) { break '__b31; }
                '__c31: loop {
                    ms = (ms << 8) + unsafe { *a.offset(i as isize) } as u64;
                    break '__c31;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if ms == 0 as u64 {
            return c"                       ".as_ptr() as *mut i8 as
                    *const i8;
        } else if ms > 4102444800000i64 as u64 {

            /// 2100-01-01 */
            ///        /*  YYYY-MM-DD HH:MM:SS.SSS
            return c"      (bad date)       ".as_ptr() as *mut i8 as
                    *const i8;
        }
        days = ms / 86400000 as u64;
        sod = ms % 86400000 as u64 / 1000 as u64;
        f = (ms % 1000 as u64) as i32;
        h = (sod / 3600 as u64) as i32;
        m = (sod % 3600 as u64 / 60 as u64) as i32;
        s = (sod % 60 as u64) as i32;
        z = days + 719468 as u64;
        era = z / 146097 as u64;
        doe = (z - era * 146097 as u64) as u32;
        yoe =
            (doe - doe / 1460 as u32 + doe / 36524 as u32 -
                    doe / 146096 as u32) / 365 as u32;
        y = (yoe as i32 as u64 + era * 400 as u64) as i32;
        doy = doe - (365 as u32 * yoe + yoe / 4 as u32 - yoe / 100 as u32);
        mp = (5 as u32 * doy + 2 as u32) / 153 as u32;
        d = (doy - (153 as u32 * mp + 2 as u32) / 5 as u32 + 1 as u32) as i32;
        m = (mp + if mp < 10 as u32 { 3 } else { -9 } as u32) as i32;
        y = y + (m <= 2) as i32;
        unsafe {
            snprintf(&raw mut z_out[0 as usize] as *mut i8,
                core::mem::size_of::<[i8; 50]>() as u64,
                c"%04d-%02d-%02d %02d:%02d:%02d.%03d".as_ptr() as *mut i8 as
                    *const i8, y, m, d, h, m, s, f)
        };
        return &raw const z_out[0 as usize] as *const i8;
    }
}

///* Try to figure out how every page in the database file is being used.
#[allow(unused_doc_comments)]
extern "C" fn page_usage_report(z_prg_1: *const i8, z_db_name_1: *const i8)
    -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        let mut rc: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut a: *mut u8 = core::ptr::null_mut();
        let mut z_query: [i8; 200] = [0; 200];
        if g.mx_page < 1 as u32 {
            unsafe {
                printf(c"empty database\n".as_ptr() as *mut i8 as *const i8)
            };
            return;
        }

        /// Open the database file
        (db = open_database(z_prg_1, z_db_name_1));

        /// Set up global variables g.zPageUse[] and g.mxPage to record page
        ///* usages
        (g.z_page_use =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<*mut i8>() as u64 *
                            (g.mx_page + 1 as u32) as u64)
                } as *mut *mut i8);
        if g.z_page_use == core::ptr::null_mut() { out_of_memory(); }
        unsafe {
            memset(g.z_page_use as *mut (), 0,
                core::mem::size_of::<*mut i8>() as u64 *
                    (g.mx_page + 1 as u32) as u64)
        };

        /// Discover the usage of each page
        (a = file_read(0 as Sqlite3Int64, 100));
        if g.b_tmstmp != 0 && unsafe { *a.offset(20 as isize) } as i32 == 16 {
            g.a_page_tag =
                unsafe {
                        sqlite3_malloc64(core::mem::size_of::<TmstmpTag>() as u64 *
                                (g.mx_page + 1 as u32) as u64)
                    } as *mut TmstmpTag;
            if g.a_page_tag == core::ptr::null_mut() { out_of_memory(); }
            unsafe {
                memset(g.a_page_tag as *mut (), 0,
                    core::mem::size_of::<TmstmpTag>() as u64 *
                        (g.mx_page + 1 as u32) as u64)
            };
        } else { g.b_tmstmp = 0; g.a_page_tag = core::ptr::null_mut(); }
        page_usage_freelist(decode_int32(unsafe { a.offset(32 as isize) } as
                    *const u8));
        page_usage_ptrmap(a as *const u8);
        unsafe { sqlite3_free(a as *mut ()) };
        page_usage_btree(1 as u32, 0, 0,
            c"sqlite_schema".as_ptr() as *mut i8 as *const i8);
        unsafe {
            sqlite3_exec(db,
                c"PRAGMA writable_schema=ON".as_ptr() as *mut i8 as *const i8,
                None, core::ptr::null_mut(), core::ptr::null_mut())
        };
        {
            j = 0 as u32;
            '__b32: loop {
                if !(j < 2 as u32) { break '__b32; }
                '__c32: loop {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 200]>() as i32,
                            &raw mut z_query[0 as usize] as *mut i8,
                            c"SELECT type, name, rootpage FROM SQLITE_MASTER WHERE rootpage ORDER BY rowid %s".as_ptr()
                                    as *mut i8 as *const i8,
                            if j != 0 {
                                c"DESC".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 })
                    };
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db,
                                &raw mut z_query[0 as usize] as *mut i8 as *const i8, -1,
                                &mut p_stmt, core::ptr::null_mut())
                        };
                    if rc == 0 {
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 2) } as u32;
                            page_usage_btree(pgno, 0, 0,
                                unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8);
                        }
                    } else {
                        unsafe {
                            printf(c"ERROR: cannot query database: %s\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                    }
                    rc = unsafe { sqlite3_finalize(p_stmt) };
                    if rc == 0 { break '__b32; }
                    break '__c32;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_close(db) };
        if g.b_csv != 0 {
            if g.b_tmstmp != 0 {
                unsafe {
                    printf(c"pgno,tm,frame,flg,salt,parent,child,ovfl,txt\r\n".as_ptr()
                                as *mut i8 as *const i8)
                };
            } else {
                unsafe {
                    printf(c"pgno,parent,child,ovfl,txt\r\n".as_ptr() as *mut i8
                            as *const i8)
                };
            }
        }
        {
            i = 1 as u32;
            '__b34: loop {
                if !(i <= g.mx_page) { break '__b34; }
                '__c34: loop {
                    if unsafe { *g.z_page_use.add(i as usize) } ==
                            core::ptr::null_mut() {
                        unsafe {
                            *g.z_page_use.add(i as usize) =
                                unsafe {
                                    sqlite3_mprintf(c"???".as_ptr() as *mut i8 as *const i8)
                                }
                        };
                        if unsafe { *g.z_page_use.add(i as usize) } ==
                                core::ptr::null_mut() {
                            break '__c34;
                        }
                    }
                    if g.b_csv != 0 {
                        let z: *const i8 =
                            unsafe { *g.z_page_use.add(i as usize) } as *const i8;
                        let mut s: *const i8 = core::ptr::null();
                        unsafe {
                            printf(c"%u,".as_ptr() as *mut i8 as *const i8, i)
                        };
                        if g.b_tmstmp != 0 {
                            let mut a: *const u8 =
                                unsafe {
                                        &raw const (*g.a_page_tag.add(i as usize)).a[0 as usize]
                                    } as *const u8;
                            let mut tm: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                            let mut x: u32 = 0 as u32;
                            let mut k: i32 = 0;
                            {
                                k = 2;
                                '__b35: loop {
                                    if !(k <= 7) { break '__b35; }
                                    '__c35: loop {
                                        tm =
                                            (tm << 8) +
                                                unsafe { *a.offset(k as isize) } as Sqlite3Uint64;
                                        break '__c35;
                                    }
                                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                printf(c"%llu.%03u,".as_ptr() as *mut i8 as *const i8,
                                    tm / 1000 as Sqlite3Uint64,
                                    (tm % 1000 as Sqlite3Uint64) as u32)
                            };
                            {
                                { ({ x = 0 as u32; x }) as i32; k = 8 };
                                '__b36: loop {
                                    if !(k <= 11) { break '__b36; }
                                    '__c36: loop {
                                        x = (x << 8) + unsafe { *a.offset(k as isize) } as u32;
                                        break '__c36;
                                    }
                                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                printf(c"%u,".as_ptr() as *mut i8 as *const i8, x)
                            };
                            unsafe {
                                printf(c"%u,".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *a.offset(12 as isize) } as i32)
                            };
                            {
                                { ({ x = 0 as u32; x }) as i32; k = 13 };
                                '__b37: loop {
                                    if !(k <= 15) { break '__b37; }
                                    '__c37: loop {
                                        x = (x << 8) + unsafe { *a.offset(k as isize) } as u32;
                                        break '__c37;
                                    }
                                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                printf(c"%u,".as_ptr() as *mut i8 as *const i8, x)
                            };
                        }
                        if {
                                    s =
                                        unsafe {
                                                strstr(z, c" of page ".as_ptr() as *mut i8 as *const i8)
                                            } as *const i8;
                                    s
                                } != core::ptr::null() {
                            unsafe {
                                printf(c"%d,".as_ptr() as *mut i8 as *const i8,
                                    unsafe { atoi(unsafe { s.offset(9 as isize) }) })
                            };
                        } else if {
                                    s =
                                        unsafe {
                                                strstr(z,
                                                    c" of trunk page ".as_ptr() as *mut i8 as *const i8)
                                            } as *const i8;
                                    s
                                } != core::ptr::null() {
                            unsafe {
                                printf(c"%d,".as_ptr() as *mut i8 as *const i8,
                                    unsafe { atoi(unsafe { s.offset(15 as isize) }) })
                            };
                        } else {
                            unsafe { printf(c"0,".as_ptr() as *mut i8 as *const i8) };
                        }
                        if {
                                    s =
                                        unsafe {
                                                strstr(z, c"], child ".as_ptr() as *mut i8 as *const i8)
                                            } as *const i8;
                                    s
                                } != core::ptr::null() {
                            unsafe {
                                printf(c"%d,".as_ptr() as *mut i8 as *const i8,
                                    unsafe { atoi(unsafe { s.offset(9 as isize) }) })
                            };
                        } else if {
                                    s =
                                        unsafe {
                                                strstr(z, c" from cell ".as_ptr() as *mut i8 as *const i8)
                                            } as *const i8;
                                    s
                                } != core::ptr::null() {
                            unsafe {
                                printf(c"%d,".as_ptr() as *mut i8 as *const i8,
                                    unsafe { atoi(unsafe { s.offset(12 as isize) }) })
                            };
                        } else {
                            unsafe { printf(c"-1,".as_ptr() as *mut i8 as *const i8) };
                        }
                        if unsafe {
                                    strncmp(z, c"overflow ".as_ptr() as *mut i8 as *const i8,
                                        9 as u64)
                                } == 0 {
                            unsafe {
                                printf(c"%d,".as_ptr() as *mut i8 as *const i8,
                                    unsafe { atoi(unsafe { z.offset(9 as isize) }) })
                            };
                        } else {
                            unsafe { printf(c"-1,".as_ptr() as *mut i8 as *const i8) };
                        }
                        unsafe {
                            printf(c"\"%s\"\r\n".as_ptr() as *mut i8 as *const i8, z)
                        };
                    } else if g.b_tmstmp != 0 {
                        unsafe {
                            printf(c"%5u: %s %s\n".as_ptr() as *mut i8 as *const i8, i,
                                decode_timestamp(unsafe {
                                            &raw mut (*g.a_page_tag.add(i as usize)).a[2 as usize]
                                        } as *const u8), unsafe { *g.z_page_use.add(i as usize) })
                        };
                    } else {
                        unsafe {
                            printf(c"%5u: %s\n".as_ptr() as *mut i8 as *const i8, i,
                                unsafe { *g.z_page_use.add(i as usize) })
                        };
                    }
                    break '__c34;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 1 as u32;
            '__b38: loop {
                if !(i <= g.mx_page) { break '__b38; }
                '__c38: loop {
                    unsafe {
                        sqlite3_free(unsafe { *g.z_page_use.add(i as usize) } as
                                *mut ())
                    };
                    break '__c38;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(g.z_page_use as *mut ()) };
        g.z_page_use = core::ptr::null_mut();
    }
}

///* Try to figure out how every page in the database file is being used.
#[allow(unused_doc_comments)]
extern "C" fn ptrmap_coverage_report(z_db_name_1: *const i8) -> () {
    unsafe {
        let mut pgno: u64 = 0 as u64;
        let mut a_hdr: *mut u8 = core::ptr::null_mut();
        let mut a: *mut u8 = core::ptr::null_mut();
        let mut usable: i32 = 0;
        let mut per_page: i32 = 0;
        let mut i: i32 = 0;
        if g.mx_page < 1 as u32 {
            unsafe {
                printf(c"empty database\n".as_ptr() as *mut i8 as *const i8)
            };
            return;
        }

        /// Make sure PTRMAPs are used in this database
        (a_hdr = file_read(0 as Sqlite3Int64, 100));
        if unsafe { *a_hdr.offset(55 as isize) } as i32 == 0 {
            unsafe {
                printf(c"database does not use PTRMAP pages\n".as_ptr() as
                            *mut i8 as *const i8)
            };
            return;
        }
        usable =
            (g.pagesize - unsafe { *a_hdr.offset(20 as isize) } as i64) as
                i32;
        per_page = usable / 5;
        unsafe { sqlite3_free(a_hdr as *mut ()) };
        unsafe {
            printf(c"%5d: root of sqlite_schema\n".as_ptr() as *mut i8 as
                    *const i8, 1)
        };
        {
            pgno = 2 as u64;
            '__b39: loop {
                if !(pgno <= g.mx_page as u64) { break '__b39; }
                '__c39: loop {
                    unsafe {
                        printf(c"%5llu: PTRMAP page covering %llu..%llu\n".as_ptr()
                                    as *mut i8 as *const i8, pgno, pgno + 1 as u64,
                            pgno + per_page as u64)
                    };
                    a =
                        file_read(((pgno - 1 as u64) * g.pagesize as u64) as
                                Sqlite3Int64, g.pagesize as i32);
                    {
                        i = 0;
                        '__b40: loop {
                            if !(i + 5 <= usable) { break '__b40; }
                            '__c40: loop {
                                let mut z_type: *const i8 = core::ptr::null();
                                let i_from: u32 =
                                    decode_int32(unsafe { &raw mut *a.offset((i + 1) as isize) }
                                            as *const u8);
                                let z_extra: *const i8 =
                                    if pgno + 1 as u64 + (i / 5) as u64 > g.mx_page as u64 {
                                            c" (off end of DB)".as_ptr() as *mut i8
                                        } else { c"".as_ptr() as *mut i8 } as *const i8;
                                '__s41:
                                    {
                                    match unsafe { *a.offset(i as isize) } {
                                        1 => {
                                            z_type =
                                                c"b-tree root page".as_ptr() as *mut i8 as *const i8;
                                        }
                                        2 => {
                                            z_type = c"freelist page".as_ptr() as *mut i8 as *const i8;
                                        }
                                        3 => {
                                            z_type =
                                                c"first page of overflow".as_ptr() as *mut i8 as *const i8;
                                        }
                                        4 => {
                                            z_type =
                                                c"later page of overflow".as_ptr() as *mut i8 as *const i8;
                                        }
                                        5 => {
                                            z_type =
                                                c"b-tree non-root page".as_ptr() as *mut i8 as *const i8;
                                        }
                                        _ => {
                                            {
                                                if unsafe { *z_extra.offset(0 as isize) } as i32 == 0 {
                                                    unsafe {
                                                        printf(c"%5llu: invalid (0x%02x), parent=%u\n".as_ptr() as
                                                                    *mut i8 as *const i8, pgno + 1 as u64 + (i / 5) as u64,
                                                            unsafe { *a.offset(i as isize) } as i32, i_from)
                                                    };
                                                }
                                                z_type = core::ptr::null();
                                                break '__s41;
                                            }
                                        }
                                    }
                                }
                                if !(z_type).is_null() {
                                    unsafe {
                                        printf(c"%5llu: %s, parent=%u%s\n".as_ptr() as *mut i8 as
                                                *const i8, pgno + 1 as u64 + (i / 5) as u64, z_type, i_from,
                                            z_extra)
                                    };
                                }
                                break '__c40;
                            }
                            i += 5;
                        }
                    }
                    unsafe { sqlite3_free(a as *mut ()) };
                    break '__c39;
                }
                pgno += (per_page + 1) as u64;
            }
        }
    }
}

///* Check the range validity for a page number.  Print an error and
///* exit if the page is out of range.
extern "C" fn check_page_validity(i_page_1: u32) -> () {
    unsafe {
        if i_page_1 < 1 as u32 || i_page_1 > g.mx_page {
            eprintln!("Invalid page number {}:  valid range is 1..{}", i_page_1 as i32, g.mx_page as i32);
            unsafe { exit(1) };
        }
    }
}

///* Print a usage comment
extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        unsafe {
            fprintf(__stderrp,
                c"Usage %s ?--uri? FILENAME ?args...?\n\n".as_ptr() as *mut i8
                    as *const i8, argv0)
        };
        eprintln!("switches:\n    --csv           CSV output for \"pgidx\"\n    --raw           Read db file directly, bypassing SQLite VFS\n    --tmstmp        Interpret tmstmpvfs tags\nargs:\n    dbheader        Show database header\n    pgidx           Index of how each page is used\n    ptrmap          Show all PTRMAP page content\n    NNN..MMM        Show hex of pages NNN through MMM\n    NNN..end        Show hex of pages NNN through end of file\n    NNNb            Decode btree page NNN\n    NNNbc           Decode btree page NNN and show content\n    NNNbm           Decode btree page NNN and show a layout map\n    NNNbdCCC        Decode cell CCC on btree page NNN\n    NNNt            Decode freelist trunk page NNN\n    NNNtd           Show leaf freelist pages on the decode\n    NNNtr           Recursively decode freelist starting at NNN");
    }
}

#[allow(unused_doc_comments)]
extern "C" fn __main_inner(argc: i32, argv: *mut *mut i8) -> Result<(), i32> {
    unsafe {
        let mut sz_file: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut z_pg_sz: *mut u8 = core::ptr::null_mut();
        let z_prg: *const i8 =
            unsafe { *argv.offset(0 as isize) } as *const i8;
        /// Name of this executable
        let mut az_arg: *const *mut i8 = argv as *const *mut i8;
        let mut n_arg: i32 = argc;
        while n_arg > 1 &&
                unsafe {
                            *unsafe { (*az_arg.offset(1 as isize)).offset(0 as isize) }
                        } as i32 == '-' as i32 {
            let mut z: *const i8 =
                unsafe { *az_arg.offset(1 as isize) } as *const i8;
            if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 &&
                    unsafe { *z.offset(2 as isize) } as i32 != 0 {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if unsafe {
                        sqlite3_stricmp(c"-raw".as_ptr() as *mut i8 as *const i8, z)
                    } == 0 {
                g.b_raw = 1;
                {
                    let __p = &mut az_arg;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_arg; let __t = *__p; *__p -= 1; __t };
            } else if unsafe {
                        strcmp(c"-csv".as_ptr() as *mut i8 as *const i8, z)
                    } == 0 {
                g.b_csv = 1;
                {
                    let __p = &mut az_arg;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_arg; let __t = *__p; *__p -= 1; __t };
            } else if unsafe {
                        strcmp(c"-tmstmp".as_ptr() as *mut i8 as *const i8, z)
                    } == 0 {
                g.b_tmstmp = 1;
                {
                    let __p = &mut az_arg;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_arg; let __t = *__p; *__p -= 1; __t };
            } else { usage(z_prg); unsafe { exit(1) }; }
        }
        if n_arg < 2 { usage(z_prg); unsafe { exit(1) }; }
        file_open(z_prg, unsafe { *az_arg.offset(1 as isize) } as *const i8);
        sz_file = file_getsize();
        z_pg_sz = file_read(0 as Sqlite3Int64, 24);
        g.pagesize =
            (unsafe { *z_pg_sz.offset(16 as isize) } as i32 * 256 +
                    unsafe { *z_pg_sz.offset(17 as isize) } as i32 * 65536) as
                i64;
        if g.pagesize == 0 as i64 { g.pagesize = 4096 as i64; }
        g.n_res = unsafe { *z_pg_sz.offset(20 as isize) } as u32;
        g.usablesize = g.pagesize - g.n_res as i64;
        unsafe { sqlite3_free(z_pg_sz as *mut ()) };
        g.mx_page =
            ((sz_file + g.pagesize - 1 as Sqlite3Int64) / g.pagesize) as u32;
        if (g.b_csv == 0) as i32 != 0 {
            unsafe {
                printf(c"Pagesize: %d\n".as_ptr() as *mut i8 as *const i8,
                    g.pagesize as i32)
            };
            if g.n_res != 0 {
                unsafe {
                    printf(c"Useable-size: %d\n".as_ptr() as *mut i8 as
                            *const i8, g.usablesize as i32)
                };
            }
            unsafe {
                printf(c"Available pages: 1..%u\n".as_ptr() as *mut i8 as
                        *const i8, g.mx_page)
            };
        }
        if n_arg == 2 {
            let mut i: u32 = 0 as u32;
            {
                i = 1 as u32;
                '__b43: loop {
                    if !(i <= g.mx_page) { break '__b43; }
                    '__c43: loop { print_page(i); break '__c43; }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else {
            let mut i: i32 = 0;
            {
                i = 2;
                '__b44: loop {
                    if !(i < n_arg) { break '__b44; }
                    '__c44: loop {
                        let mut i_start: u32 = 0 as u32;
                        let mut i_end: u32 = 0 as u32;
                        let mut z_left: *mut i8 = core::ptr::null_mut();
                        if unsafe {
                                    strcmp(unsafe { *az_arg.offset(i as isize) } as *const i8,
                                        c"dbheader".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            print_db_header();
                            break '__c44;
                        }
                        if unsafe {
                                    strcmp(unsafe { *az_arg.offset(i as isize) } as *const i8,
                                        c"pgidx".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            page_usage_report(z_prg,
                                unsafe { *az_arg.offset(1 as isize) } as *const i8);
                            break '__c44;
                        }
                        if unsafe {
                                    strcmp(unsafe { *az_arg.offset(i as isize) } as *const i8,
                                        c"ptrmap".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            ptrmap_coverage_report(unsafe { *az_arg.offset(1 as isize) }
                                    as *const i8);
                            break '__c44;
                        }
                        if unsafe {
                                    strcmp(unsafe { *az_arg.offset(i as isize) } as *const i8,
                                        c"help".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            usage(z_prg);
                            break '__c44;
                        }
                        if (unsafe {
                                            isdigit(unsafe {
                                                            *unsafe { (*az_arg.offset(i as isize)).offset(0 as isize) }
                                                        } as u8 as i32)
                                        } == 0) as i32 != 0 {
                            unsafe {
                                fprintf(__stderrp,
                                    c"%s: unknown option: [%s]\n".as_ptr() as *mut i8 as
                                        *const i8, z_prg, unsafe { *az_arg.offset(i as isize) })
                            };
                            break '__c44;
                        }
                        i_start =
                            unsafe {
                                    strtoul(unsafe { *az_arg.offset(i as isize) } as *const i8,
                                        &mut z_left, 0)
                                } as u32;
                        check_page_validity(i_start);
                        if !(z_left).is_null() &&
                                unsafe {
                                        strcmp(z_left as *const i8,
                                            c"..end".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            i_end = g.mx_page;
                        } else if !(z_left).is_null() &&
                                    unsafe { *z_left.offset(0 as isize) } as i32 == '.' as i32
                                &&
                                unsafe { *z_left.offset(1 as isize) } as i32 == '.' as i32 {
                            i_end =
                                unsafe {
                                        strtol(unsafe { &raw mut *z_left.offset(2 as isize) } as
                                                *const i8, core::ptr::null_mut(), 0)
                                    } as u32;
                            check_page_validity(i_end);
                        } else if !(z_left).is_null() &&
                                unsafe { *z_left.offset(0 as isize) } as i32 == 'b' as i32 {
                            let mut ofst: i64 = 0 as i64;
                            let mut n_byte: i32 = 0;
                            let mut hdr_size: i32 = 0;
                            let mut a: *mut u8 = core::ptr::null_mut();
                            if i_start == 1 as u32 {
                                ofst = { hdr_size = 100; hdr_size } as i64;
                                n_byte = (g.pagesize - 100 as i64) as i32;
                            } else {
                                hdr_size = 0;
                                ofst = (i_start - 1 as u32) as i64 * g.pagesize;
                                n_byte = g.pagesize as i32;
                            }
                            a = file_read(ofst, n_byte);
                            decode_btree_page(a, i_start as i32, hdr_size,
                                unsafe { &raw mut *z_left.offset(1 as isize) } as
                                    *const i8);
                            unsafe { sqlite3_free(a as *mut ()) };
                            break '__c44;
                        } else if !(z_left).is_null() &&
                                unsafe { *z_left.offset(0 as isize) } as i32 == 't' as i32 {
                            let mut detail: i32 = 0;
                            let mut recursive: i32 = 0;
                            let mut j: i32 = 0;
                            {
                                j = 1;
                                '__b45: loop {
                                    if !(unsafe { *z_left.offset(j as isize) } != 0) {
                                        break '__b45;
                                    }
                                    '__c45: loop {
                                        if unsafe { *z_left.offset(j as isize) } as i32 ==
                                                'r' as i32 {
                                            recursive = 1;
                                        }
                                        if unsafe { *z_left.offset(j as isize) } as i32 ==
                                                'd' as i32 {
                                            detail = 1;
                                        }
                                        break '__c45;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            decode_trunk_page(i_start, detail, recursive);
                            break '__c44;
                        } else { i_end = i_start; }
                        if i_start < 1 as u32 || i_end < i_start ||
                                i_end > g.mx_page {
                            eprintln!("Page argument should be LOWER?..UPPER?.  Range 1 to {}", g.mx_page as i32);
                            unsafe { exit(1) };
                        }
                        while i_start <= i_end {
                            print_page(i_start);
                            { let __p = &mut i_start; let __t = *__p; *__p += 1; __t };
                        }
                        break '__c44;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        file_close();
        return Ok(());
    }
}

static mut z_desc_2: [i8; 1000] = unsafe { core::mem::zeroed() };

static mut z_out: [i8; 50] = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
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
    fn exit(_: i32)
    -> ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn close(_: i32)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn lseek(_: i32, _: OffT, _: i32)
    -> OffT;
    fn read(_: i32, _: *mut (), __nbyte: u64)
    -> i64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fstat(_: i32, _: *mut Stat)
    -> i32;
    fn isprint(_c: i32)
    -> i32;
    fn sprintf(_: *mut i8, _: *const i8, ...)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn isdigit(_c: i32)
    -> i32;
    fn snprintf(__str: *mut i8, __size: u64, __format: *const i8, ...)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn atoi(_: *const i8)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strtoul(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> u64;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
