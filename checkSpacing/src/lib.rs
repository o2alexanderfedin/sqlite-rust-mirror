extern "C" fn check_spacing(z_file_1: *const i8, flags: u32) -> () {
    let in_: *mut FILE =
        unsafe { fopen(z_file_1, c"rb".as_ptr() as *mut i8 as *const i8) };
    let mut i: i32 = 0;
    let mut seen_space: i32 = 0;
    let mut seen_tab: i32 = 0;
    let mut ln: i32 = 0;
    let mut last_nonspace: i32 = 0;
    let mut z_line: [i8; 2000] = [0; 2000];
    if in_ == core::ptr::null_mut() {
        unsafe {
            printf(c"cannot open %s\n".as_ptr() as *mut i8 as *const i8,
                z_file_1)
        };
        return;
    }
    while !(unsafe {
                        fgets(&raw mut z_line[0 as usize] as *mut i8,
                            core::mem::size_of::<[i8; 2000]>() as i32, in_)
                    }).is_null() {
        seen_space = 0;
        seen_tab = 0;
        { let __p = &mut ln; let __t = *__p; *__p += 1; __t };
        {
            i = 0;
            '__b1: loop {
                if !(z_line[i as usize] != 0) { break '__b1; }
                '__c1: loop {
                    if z_line[i as usize] as i32 == '\t' as i32 && seen_tab == 0
                        {
                        unsafe {
                            printf(c"%s:%d: tab (\\t) character\n".as_ptr() as *mut i8
                                    as *const i8, z_file_1, ln)
                        };
                        seen_tab = 1;
                    } else if z_line[i as usize] as i32 == '\r' as i32 {
                        if flags & 1 as u32 == 0 as u32 {
                            unsafe {
                                printf(c"%s:%d: carriage-return (\\r) character\n".as_ptr()
                                            as *mut i8 as *const i8, z_file_1, ln)
                            };
                        }
                    } else if z_line[i as usize] as i32 == ' ' as i32 {
                        seen_space = 1;
                    } else if z_line[i as usize] as i32 != '\n' as i32 {
                        last_nonspace = ln;
                        seen_space = 0;
                    }
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if seen_space != 0 && flags & 2 as u32 == 0 as u32 {
            unsafe {
                printf(c"%s:%d: whitespace at end-of-line\n".as_ptr() as
                            *mut i8 as *const i8, z_file_1, ln)
            };
        }
    }
    unsafe { fclose(in_) };
    if last_nonspace < ln {
        unsafe {
            printf(c"%s:%d: blank lines at end of file (%d)\n".as_ptr() as
                        *mut i8 as *const i8, z_file_1, ln, ln - last_nonspace)
        };
    }
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    let mut i: i32 = 0;
    let mut flags: u32 = 2 as u32;
    {
        i = 1;
        '__b2: loop {
            if !(i < argc) { break '__b2; }
            '__c2: loop {
                let mut z: *const i8 =
                    unsafe { *argv.offset(i as isize) } as *const i8;
                if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                    while unsafe { *z.offset(0 as isize) } as i32 == '-' as i32
                        {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe {
                                strcmp(z, c"crok".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        flags |= 1 as u32;
                    } else if unsafe {
                                strcmp(z, c"wseol".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        flags &= !2 as u32;
                    } else if unsafe {
                                strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        unsafe {
                            printf(c"Usage: %s [options] FILE ...\n".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(0 as isize) })
                        };
                        unsafe {
                            printf(c"  --crok      Do not report on carriage-returns\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        unsafe {
                            printf(c"  --wseol     Complain about whitespace at end-of-line\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        unsafe {
                            printf(c"  --help      This message\n".as_ptr() as *mut i8
                                    as *const i8)
                        };
                    } else {
                        unsafe {
                            printf(c"unknown command-line option: [%s]\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        unsafe {
                            printf(c"use --help for additional information\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    }
                } else {
                    check_spacing(unsafe { *argv.offset(i as isize) } as
                            *const i8, flags);
                }
                break '__c2;
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
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn printf(_: *const i8, ...)
    -> i32;
    fn fgets(_: *mut i8, __size: i32, _: *mut FILE)
    -> *mut i8;
    fn fclose(_: *mut FILE)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
