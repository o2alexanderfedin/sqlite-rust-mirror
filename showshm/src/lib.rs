type DarwinSizeT = u64;
type Int64T = i64;
type DarwinOffT = Int64T;
type OffT = DarwinOffT;
type DarwinSsizeT = i64;
static mut fd: i32 = -1;
extern "C" fn out_of_memory() -> () {
    eprintln!("Out of memory...");
    unsafe { exit(1) };
}
extern "C" fn get_content(ofst: i32, n_byte_1: i32) -> *mut u8 {
    unsafe {
        let mut a_data: *mut u8 = core::ptr::null_mut();
        a_data = unsafe { malloc(n_byte_1 as u64) } as *mut u8;
        if a_data == core::ptr::null_mut() { out_of_memory(); }
        unsafe { lseek(fd, ofst as OffT, 0) };
        unsafe { read(fd, a_data as *mut (), n_byte_1 as u64) };
        return a_data;
    }
}
extern "C" fn print_decode_line(a_data_1: *const u8, ofst: i32, n_byte_1: i32,
    flg: u32, z_msg_1: *const i8) -> () {
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
        '__b0: loop {
            if !(j < 4) { break '__b0; }
            '__c0: loop {
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
                break '__c0;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_byte_1 == 8 {
        {
            j = 4;
            '__b1: loop {
                if !(j < 8) { break '__b1; }
                '__c1: loop {
                    unsafe {
                        sprintf(&mut z_buf[i as usize],
                            c" %02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *a_data_1.offset((ofst + j) as isize) } as i32)
                    };
                    i +=
                        unsafe { strlen(&raw mut z_buf[i as usize] as *const i8) }
                            as i32;
                    break '__c1;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if flg & 2 as u32 != 0 {
        if !(n_byte_1 == 4) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"print_decode_line".as_ptr() as *const i8,
                    c"showshm.c".as_ptr() as *mut i8 as *const i8, 84,
                    c"nByte==4".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            memcpy(&raw mut val as *mut (),
                unsafe { a_data_1.offset(ofst as isize) } as *const (),
                4 as u64)
        };
    }
    unsafe {
        sprintf(&mut z_buf[i as usize],
            c"            ".as_ptr() as *mut i8 as *const i8)
    };
    i += 12;
    if flg & 4 as u32 != 0 {
        let mut sz: u16 = 0 as u16;
        unsafe {
            memcpy(&raw mut sz as *mut (),
                unsafe { a_data_1.offset(ofst as isize) } as *const (),
                2 as u64)
        };
        unsafe {
            sprintf(&mut z_buf[i as usize],
                c"   %9d".as_ptr() as *mut i8 as *const i8,
                if sz as i32 == 1 { 65536 } else { sz as i32 })
        };
    } else if flg & 1 as u32 != 0 {
        unsafe {
            sprintf(&mut z_buf[i as usize],
                c"  0x%08x".as_ptr() as *mut i8 as *const i8, val)
        };
    } else if n_byte_1 < 8 {
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
extern "C" fn print_index_hdr(a_data_1: *mut u8, ix: i32) -> () {
    let mut i: i32 = 0;
    if !(ix == 0 || ix == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"print_index_hdr".as_ptr() as *const i8,
                c"showshm.c".as_ptr() as *mut i8 as *const i8, 107,
                c"ix==0 || ix==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    i = if ix != 0 { 48 } else { 0 };
    print_decode_line(a_data_1 as *const u8, 0 + i, 4, 2 as u32,
        c"Wal-index version".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 4 + i, 4, 0 as u32,
        c"unused padding".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 8 + i, 4, 2 as u32,
        c"transaction counter".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 12 + i, 1, 0 as u32,
        c"1 when initialized".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 13 + i, 1, 0 as u32,
        c"true if WAL cksums are bigendian".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 14 + i, 2, 4 as u32,
        c"database page size".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 16 + i, 4, 2 as u32,
        c"mxFrame".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 20 + i, 4, 2 as u32,
        c"Size of database in pages".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 24 + i, 8, 0 as u32,
        c"Cksum of last frame in -wal".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 32 + i, 8, 0 as u32,
        c"Salt values from the -wal".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 40 + i, 8, 0 as u32,
        c"Cksum over all prior fields".as_ptr() as *mut i8 as *const i8);
}
extern "C" fn print_ckpt_info(a_data_1: *mut u8) -> () {
    let i: i32 = 96 as i32;
    let mut j: i32 = 0;
    print_decode_line(a_data_1 as *const u8, 0 + i as i32, 4, 2 as u32,
        c"nBackfill".as_ptr() as *mut i8 as *const i8);
    {
        j = 0;
        '__b2: loop {
            if !(j < 5) { break '__b2; }
            '__c2: loop {
                let mut z_label: [i8; 100] = [0; 100];
                unsafe {
                    sprintf(&raw mut z_label[0 as usize] as *mut i8,
                        c"aReadMark[%d]".as_ptr() as *mut i8 as *const i8, j)
                };
                print_decode_line(a_data_1 as *const u8, 4 * j + 4 + i as i32,
                    4, 2 as u32,
                    &raw mut z_label[0 as usize] as *mut i8 as *const i8);
                break '__c2;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    print_decode_line(a_data_1 as *const u8, 24 + i as i32, 8, 0 as u32,
        c"aLock".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 32 + i as i32, 4, 2 as u32,
        c"nBackfillAttempted".as_ptr() as *mut i8 as *const i8);
    print_decode_line(a_data_1 as *const u8, 36 + i as i32, 4, 2 as u32,
        c"notUsed0".as_ptr() as *mut i8 as *const i8);
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut a_data: *mut u8 = core::ptr::null_mut();
        if argc < 2 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s FILENAME\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) })
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
        a_data = get_content(0, 136);
        print_index_hdr(a_data, 0);
        print_index_hdr(a_data, 1);
        print_ckpt_info(a_data);
        unsafe { free(a_data as *mut ()) };
        unsafe { close(fd) };
        return Ok(());
    }
}
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
    fn sprintf(_: *mut i8, _: *const i8, ...)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn free(_: *mut ())
    -> ();
    fn close(_: i32)
    -> i32;
    static mut __stderrp: *mut FILE;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;