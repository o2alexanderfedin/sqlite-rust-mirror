type __darwin_size_t = u64;
extern "C" fn read_file(z_filename_1: *const i8) -> *mut i8 {
    unsafe {
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut z: *mut i8 = core::ptr::null_mut();
        let mut n: i64 = 0 as i64;
        let mut got: u64 = 0 as u64;
        in_ =
            unsafe {
                fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if in_ == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"unable to open \'%s\' for reading\n".as_ptr() as *mut i8
                        as *const i8, z_filename_1)
            };
            unsafe { exit(1) };
        }
        unsafe { fseek(in_, 0 as i64, 2) };
        n = unsafe { ftell(in_) };
        unsafe { rewind(in_) };
        z = unsafe { malloc((n + 1 as i64) as u64) } as *mut i8;
        if z == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot allocate %d bytes to store \'%s\'\n".as_ptr() as
                            *mut i8 as *const i8, (n + 1 as i64) as i32, z_filename_1)
            };
            unsafe { exit(1) };
        }
        got = unsafe { fread(z as *mut (), 1 as u64, n as u64, in_) };
        unsafe { fclose(in_) };
        if got != n as u64 {
            unsafe {
                fprintf(__stderrp,
                    c"only read %d of %d bytes from \'%s\'\n".as_ptr() as
                            *mut i8 as *const i8, got as i32, n as i32, z_filename_1)
            };
            unsafe { exit(1) };
        }
        unsafe { *z.offset(n as isize) = 0 as i8 };
        return z;
    }
}
extern "C" fn has_side_effect(z: &[i8]) -> i32 {
    let mut i: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b0: loop {
            if !(i < z.len() as u32) { break '__b0; }
            '__c0: loop {
                if z[i as usize] as i32 == '/' as i32 &&
                        unsafe {
                                strncmp(&z[i as usize],
                                    c"/*side-effects-ok*/".as_ptr() as *mut i8 as *const i8,
                                    19 as u64)
                            } == 0 {
                    return 0;
                }
                if z[i as usize] as i32 == '=' as i32 && i > 0 as u32 &&
                                        z[(i - 1 as u32) as usize] as i32 != '=' as i32 &&
                                    z[(i - 1 as u32) as usize] as i32 != '>' as i32 &&
                                z[(i - 1 as u32) as usize] as i32 != '<' as i32 &&
                            z[(i - 1 as u32) as usize] as i32 != '!' as i32 &&
                        z[(i + 1 as u32) as usize] as i32 != '=' as i32 {
                    return 1;
                }
                if z[i as usize] as i32 == '+' as i32 &&
                        z[(i + 1 as u32) as usize] as i32 == '+' as i32 {
                    return 1;
                }
                if z[i as usize] as i32 == '-' as i32 &&
                        z[(i + 1 as u32) as usize] as i32 == '-' as i32 {
                    return 1;
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
extern "C" fn find_close_paren(z: *const i8) -> u32 {
    let mut n_open: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b1: loop {
            if !(unsafe { *z.add(i as usize) } != 0) { break '__b1; }
            '__c1: loop {
                if unsafe { *z.add(i as usize) } as i32 == '(' as i32 {
                    { let __p = &mut n_open; let __t = *__p; *__p += 1; __t };
                }
                if unsafe { *z.add(i as usize) } as i32 == ')' as i32 {
                    if n_open == 0 as u32 { break '__b1; }
                    { let __p = &mut n_open; let __t = *__p; *__p -= 1; __t };
                }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return i;
}
extern "C" fn find_all_side_effects(z: *const i8) -> u32 {
    unsafe {
        let mut lineno: u32 = 1 as u32;
        let mut i: u32 = 0 as u32;
        let mut n_err: u32 = 0 as u32;
        let mut c: i8 = 0 as i8;
        let mut prev_c: i8 = 0 as i8;
        {
            i = 0 as u32;
            '__b2: loop {
                if !({ c = unsafe { *z.add(i as usize) } as i8; c } as i32 !=
                                0) {
                    break '__b2;
                }
                '__c2: loop {
                    if c as i32 == '\n' as i32 {
                        { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
                        break '__c2;
                    }
                    if unsafe { isalpha(c as i32) } != 0 &&
                            (unsafe { isalpha(prev_c as i32) } == 0) as i32 != 0 {
                        if unsafe {
                                                strncmp(unsafe { &*z.add(i as usize) },
                                                    c"assert(".as_ptr() as *mut i8 as *const i8, 7 as u64)
                                            } == 0 ||
                                        unsafe {
                                                strncmp(unsafe { &*z.add(i as usize) },
                                                    c"ALWAYS(".as_ptr() as *mut i8 as *const i8, 7 as u64)
                                            } == 0 ||
                                    unsafe {
                                            strncmp(unsafe { &*z.add(i as usize) },
                                                c"NEVER(".as_ptr() as *mut i8 as *const i8, 6 as u64)
                                        } == 0 ||
                                unsafe {
                                        strncmp(unsafe { &*z.add(i as usize) },
                                            c"testcase(".as_ptr() as *mut i8 as *const i8, 9 as u64)
                                    } == 0 {
                            let mut n: u32 = 0 as u32;
                            let mut z2: *const i8 =
                                unsafe { &*z.add((i + 5 as u32) as usize) };
                            while unsafe { *z2.offset(0 as isize) } as i32 != '(' as i32
                                {
                                {
                                    let __p = &mut z2;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                            {
                                let __p = &mut z2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            n = find_close_paren(z2);
                            if has_side_effect(unsafe {
                                            let __p = z2 as *const i8;
                                            if __p.is_null() {
                                                &[]
                                            } else { core::slice::from_raw_parts(__p, n as usize) }
                                        }) != 0 {
                                { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
                                unsafe {
                                    fprintf(__stderrp,
                                        c"side-effect line %u: %.*s\n".as_ptr() as *mut i8 as
                                            *const i8, lineno,
                                        unsafe {
                                                    unsafe {
                                                        z2.add((n + 1 as u32) as
                                                                    usize).offset_from(unsafe { z.add(i as usize) })
                                                    }
                                                } as i64 as i32,
                                        unsafe { &raw const *z.add(i as usize) } as *const i8)
                                };
                            }
                        }
                    }
                    break '__c2;
                }
                {
                    ({ prev_c = c; prev_c }) as u32;
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        return n_err;
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut z: *mut i8 = core::ptr::null_mut();
        let mut n_err: u32 = 0 as u32;
        if argc != 2 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s FILENAME\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) })
            };
            return Err(1);
        }
        z = read_file(unsafe { *argv.offset(1 as isize) } as *const i8);
        n_err = find_all_side_effects(z as *const i8);
        unsafe { free(z as *mut ()) };
        if n_err != 0 {
            eprintln!("Found {} undesirable side-effects", n_err as u32);
            return Err(1);
        }
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
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn rewind(_: *mut FILE)
    -> ();
    fn malloc(__size: u64)
    -> *mut ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn isalpha(_c: i32)
    -> i32;
    fn free(_: *mut ())
    -> ();
    static mut __stderrp: *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;