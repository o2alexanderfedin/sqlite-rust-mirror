type __darwin_size_t = u64;
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut z_end: *mut i8 = core::ptr::null_mut();
        let mut to_append: i64 = 0 as i64;
        let mut current_sz: i64 = 0 as i64;
        let mut new_sz: i64 = 0 as i64;
        let mut f: *mut FILE = core::ptr::null_mut();
        let mut got: u64 = 0 as u64;
        let mut pgsz: i32 = 0;
        let mut zero: i8 = 0 as i8;
        let mut buf: [u8; 100] = [0; 100];
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s1:
                {
                match __state {
                    0 => { __state = 4; }
                    2 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"not a valid database: %s\n".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *argv.offset(1 as isize) })
                        };
                        __state = 49;
                    }
                    3 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"Usage: %s DATABASE N\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { *argv.offset(0 as isize) })
                        };
                        __state = 51;
                    }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { zero = 0 as i8; __state = 11; }
                    11 => { __state = 12; }
                    12 => {
                        if argc != 3 { __state = 14; } else { __state = 13; }
                    }
                    13 => {
                        to_append =
                            unsafe {
                                strtoll(unsafe { *argv.offset(2 as isize) } as *const i8,
                                    &mut z_end, 0)
                            };
                        __state = 15;
                    }
                    14 => { __state = 3; }
                    15 => {
                        if z_end == unsafe { *argv.offset(2 as isize) } ||
                                unsafe { *z_end.offset(0 as isize) } != 0 {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => {
                        if to_append < 1 as i64 {
                            __state = 19;
                        } else { __state = 18; }
                    }
                    17 => { __state = 3; }
                    18 => {
                        f =
                            unsafe {
                                fopen(unsafe { *argv.offset(1 as isize) } as *const i8,
                                    c"r+b".as_ptr() as *mut i8 as *const i8)
                            };
                        __state = 21;
                    }
                    19 => { eprintln!("N must be at least 1"); __state = 20; }
                    20 => { unsafe { exit(1) }; __state = 18; }
                    21 => {
                        if f == core::ptr::null_mut() {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => {
                        got =
                            unsafe {
                                fread(&raw mut buf[0 as usize] as *mut u8 as *mut (),
                                    1 as u64, core::mem::size_of::<[u8; 100]>() as u64, f)
                            };
                        __state = 25;
                    }
                    23 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"cannot open \"%s\" for reading and writing\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(1 as isize) })
                        };
                        __state = 24;
                    }
                    24 => { unsafe { exit(1) }; __state = 22; }
                    25 => {
                        if got != core::mem::size_of::<[u8; 100]>() as u64 {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => {
                        if unsafe {
                                    strcmp(&raw mut buf[0 as usize] as *mut i8 as *const i8,
                                        c"SQLite format 3".as_ptr() as *mut i8 as *const i8)
                                } != 0 {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    27 => { __state = 2; }
                    28 => {
                        pgsz =
                            ((buf[16 as usize] as i32) << 8) + buf[17 as usize] as i32;
                        __state = 30;
                    }
                    29 => { __state = 2; }
                    30 => {
                        if pgsz == 1 { __state = 32; } else { __state = 31; }
                    }
                    31 => {
                        if pgsz < 512 || pgsz > 65536 || pgsz & pgsz - 1 != 0 {
                            __state = 34;
                        } else { __state = 33; }
                    }
                    32 => { pgsz = 65536; __state = 31; }
                    33 => {
                        current_sz =
                            (((buf[28 as usize] as i32) << 24) +
                                            ((buf[29 as usize] as i32) << 16) +
                                        ((buf[30 as usize] as i32) << 8) + buf[31 as usize] as i32)
                                as i64;
                        __state = 35;
                    }
                    34 => { __state = 2; }
                    35 => { new_sz = current_sz + to_append; __state = 36; }
                    36 => {
                        if new_sz > 4294967295u32 as i64 {
                            __state = 38;
                        } else { __state = 37; }
                    }
                    37 => {
                        buf[28 as usize] = (new_sz >> 24 & 255 as i64) as u8;
                        __state = 39;
                    }
                    38 => { new_sz = 4294967295u32 as i64; __state = 37; }
                    39 => {
                        buf[29 as usize] = (new_sz >> 16 & 255 as i64) as u8;
                        __state = 40;
                    }
                    40 => {
                        buf[30 as usize] = (new_sz >> 8 & 255 as i64) as u8;
                        __state = 41;
                    }
                    41 => {
                        buf[31 as usize] = (new_sz & 255 as i64) as u8;
                        __state = 42;
                    }
                    42 => { unsafe { fseek(f, 28 as i64, 0) }; __state = 43; }
                    43 => {
                        unsafe {
                            fwrite(&raw mut buf[28 as usize] as *const (), 4 as u64,
                                1 as u64, f)
                        };
                        __state = 44;
                    }
                    44 => {
                        unsafe {
                            fseek(f, (new_sz * pgsz as i64 - 1 as i64) as i64, 0)
                        };
                        __state = 45;
                    }
                    45 => {
                        unsafe {
                            fwrite(&raw mut zero as *const (), 1 as u64, 1 as u64, f)
                        };
                        __state = 46;
                    }
                    46 => { unsafe { fclose(f) }; __state = 47; }
                    47 => { return Ok(()); }
                    48 => { __state = 2; }
                    49 => { unsafe { exit(1) }; __state = 50; }
                    50 => { __state = 3; }
                    51 => { unsafe { exit(1) }; __state = 1; }
                    _ => {}
                }
            }
        }
        unreachable!();
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn strtoll(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn exit(_: i32)
    -> ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    static mut __stderrp: *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;