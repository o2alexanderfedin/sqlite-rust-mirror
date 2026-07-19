type DarwinSizeT = u64;

///* Compute signature for a block of content.
///*
///* For blocks of 16 or fewer bytes, the signature is just a hex dump of
///* the entire block.
///*
///* For blocks of more than 16 bytes, the signature is a hex dump of the
///* first 8 bytes followed by a 64-bit hash of the entire block.
extern "C" fn vlog_signature(p: *mut u8, n: i32, z_cksum_1: *mut i8) -> () {
    let mut s0: u32 = 0 as u32;
    let mut s1: u32 = 0 as u32;
    let mut p_i: *const u32 = core::ptr::null();
    let mut i: i32 = 0;
    if n <= 16 {
        {
            i = 0;
            '__b0: loop {
                if !(i < n) { break '__b0; }
                '__c0: loop {
                    unsafe {
                        sprintf(unsafe { z_cksum_1.offset((i * 2) as isize) },
                            c"%02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *p.offset(i as isize) } as i32)
                    };
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    } else {
        p_i = p as *mut u32;
        {
            i = 0;
            '__b1: loop {
                if !(i < n - 7) { break '__b1; }
                '__c1: loop {
                    s0 += unsafe { *p_i.offset(0 as isize) } + s1;
                    s1 += unsafe { *p_i.offset(1 as isize) } + s0;
                    {
                        let __n = 2;
                        let __p = &mut p_i;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    break '__c1;
                }
                i += 8;
            }
        }
        {
            i = 0;
            '__b2: loop {
                if !(i < 8) { break '__b2; }
                '__c2: loop {
                    unsafe {
                        sprintf(unsafe { z_cksum_1.offset((i * 2) as isize) },
                            c"%02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *p.offset(i as isize) } as i32)
                    };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sprintf(unsafe { z_cksum_1.offset((i * 2) as isize) },
                c"-%08x%08x".as_ptr() as *mut i8 as *const i8, s0, s1)
        };
    }
}

///* Open a file.  Find its page size.  Read each page, and compute and
///* display the page signature.
extern "C" fn compute_sigs(z_filename_1: *const i8) -> () {
    unsafe {
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut pgsz: u32 = 0 as u32;
        let mut got: u64 = 0 as u64;
        let mut n: u32 = 0 as u32;
        let mut a_buf: [u8; 50] = [0; 50];
        let mut a_page: [u8; 65536] = [0; 65536];
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s4:
                {
                match __state {
                    0 => {
                        in_ =
                            unsafe {
                                fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
                            };
                        __state = 3;
                    }
                    2 => { unsafe { fclose(in_) }; __state = 1; }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => {
                        if in_ == core::ptr::null_mut() {
                            __state = 10;
                        } else { __state = 9; }
                    }
                    9 => {
                        got =
                            unsafe {
                                fread(&raw mut a_buf[0 as usize] as *mut u8 as *mut (),
                                    1 as u64, core::mem::size_of::<[u8; 50]>() as u64, in_)
                            };
                        __state = 12;
                    }
                    10 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"cannot open \"%s\"\n".as_ptr() as *mut i8 as *const i8,
                                z_filename_1)
                        };
                        __state = 11;
                    }
                    11 => { return; }
                    12 => {
                        if got != core::mem::size_of::<[u8; 50]>() as u64 {
                            __state = 14;
                        } else { __state = 13; }
                    }
                    13 => {
                        pgsz =
                            (a_buf[16 as usize] as i32 * 256 +
                                    a_buf[17 as usize] as i32) as u32;
                        __state = 15;
                    }
                    14 => { __state = 2; }
                    15 => {
                        if pgsz == 1 as u32 { __state = 17; } else { __state = 16; }
                    }
                    16 => {
                        if pgsz & pgsz - 1 as u32 != 0 as u32 {
                            __state = 19;
                        } else { __state = 18; }
                    }
                    17 => { pgsz = 65536 as u32; __state = 16; }
                    18 => { unsafe { rewind(in_) }; __state = 21; }
                    19 => {
                        eprintln!("invalid page size: {:02x}{:02x}", a_buf[16 as usize] as i32 as u32, a_buf[17 as usize] as i32 as u32);
                        __state = 20;
                    }
                    20 => { __state = 2; }
                    21 => { n = 1 as u32; __state = 23; }
                    22 => { __state = 2; }
                    23 => {
                        if {
                                    got =
                                        unsafe {
                                            fread(&raw mut a_page[0 as usize] as *mut u8 as *mut (),
                                                1 as u64, pgsz as u64, in_)
                                        };
                                    got
                                } == pgsz as u64 {
                            __state = 24;
                        } else { __state = 22; }
                    }
                    24 => {
                        vlog_signature(&raw mut a_page[0 as usize] as *mut u8,
                            pgsz as i32,
                            &raw mut a_buf[0 as usize] as *mut u8 as *mut i8);
                        __state = 26;
                    }
                    25 => {
                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        __state = 23;
                    }
                    26 => {
                        unsafe {
                            printf(c"%4d: %s\n".as_ptr() as *mut i8 as *const i8, n,
                                &raw mut a_buf[0 as usize] as *mut u8)
                        };
                        __state = 25;
                    }
                    _ => {}
                }
            }
        }
    }
}

///* Find page signatures for all named files.
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    let mut i: i32 = 0;
    {
        i = 1;
        '__b5: loop {
            if !(i < argc) { break '__b5; }
            '__c5: loop {
                compute_sigs(unsafe { *argv.offset(i as isize) } as
                        *const i8);
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return Ok(());
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
    fn sprintf(_: *mut i8, _: *const i8, ...)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fclose(_: *mut FILE)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn rewind(_: *mut FILE)
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    static mut __stderrp: *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
