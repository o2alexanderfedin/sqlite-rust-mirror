type DarwinSizeT = u64;
extern "C" fn hex_value(c: i8) -> i32 {
    if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
        return c as i32 - '0' as i32;
    }
    if c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32 {
        return c as i32 - 'a' as i32 + 10;
    }
    if c as i32 >= 'A' as i32 && c as i32 <= 'F' as i32 {
        return c as i32 - 'A' as i32 + 10;
    }
    return -1;
}
extern "C" fn to_hex(c: u8) -> i8 {
    return unsafe {
            *(c"0123456789abcdef".as_ptr() as
                        *mut i8).offset((c as i32 & 15) as isize)
        };
}
extern "C" fn put_varint(p: *mut u8, mut v: u64) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut buf: [u8; 10] = [0; 10];
    if v & (4278190080u32 as u64) << 32 != 0 {
        unsafe { *p.offset(8 as isize) = v as u8 };
        v >>= 8 as u64;
        {
            i = 7;
            '__b0: loop {
                if !(i >= 0) { break '__b0; }
                '__c0: loop {
                    unsafe {
                        *p.offset(i as isize) = (v & 127 as u64 | 128 as u64) as u8
                    };
                    v >>= 7 as u64;
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        return 9;
    }
    n = 0;
    '__b1: loop {
        '__c1: loop {
            buf[{ let __p = &mut n; let __t = *__p; *__p += 1; __t } as usize]
                = (v & 127 as u64 | 128 as u64) as u8;
            v >>= 7 as u64;
            break '__c1;
        }
        if !(v != 0 as u64) { break '__b1; }
    }
    buf[0 as usize] &= 127 as u8;
    {
        { i = 0; j = n - 1 };
        '__b2: loop {
            if !(j >= 0) { break '__b2; }
            '__c2: loop {
                unsafe { *p.offset(i as isize) = buf[j as usize] };
                break '__c2;
            }
            {
                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                { let __p = &mut i; let __t = *__p; *__p += 1; __t }
            };
        }
    }
    return n;
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut i: i32 = 0;
        let mut x: u64 = 0 as u64;
        let mut u_x: u64 = 0 as u64;
        let mut i_x: i64 = 0 as i64;
        let mut n: i32 = 0;
        let mut z_hex: [u8; 20] = [0; 20];
        if argc == 1 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage:\n  %s HH HH HH ...   Convert varint to decimal\n  %s DDDDD          Convert decimal to varint\n                    Add \'+\' or \'-\' before DDDDD to disambiguate.\n".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) },
                    unsafe { *argv.offset(0 as isize) })
            };
            unsafe { exit(1) };
        }
        if argc > 2 ||
                unsafe {
                                strlen(unsafe { *argv.offset(1 as isize) } as *const i8)
                            } == 2 as u64 &&
                        hex_value(unsafe {
                                    *unsafe { (*argv.offset(1 as isize)).offset(0 as isize) }
                                }) >= 0 &&
                    hex_value(unsafe {
                                *unsafe { (*argv.offset(1 as isize)).offset(1 as isize) }
                            }) >= 0 {
            {
                i = 1;
                '__b3: loop {
                    if !(i < argc && i < 9) { break '__b3; }
                    '__c3: loop {
                        if unsafe {
                                    strlen(unsafe { *argv.offset(i as isize) } as *const i8)
                                } != 2 as u64 {
                            unsafe {
                                fprintf(__stderrp,
                                    c"Not a hex byte: %s\n".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(i as isize) })
                            };
                            unsafe { exit(1) };
                        }
                        x =
                            ((hex_value(unsafe {
                                                *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                            }) << 4) +
                                    hex_value(unsafe {
                                            *unsafe { (*argv.offset(i as isize)).offset(1 as isize) }
                                        })) as u64;
                        u_x = (u_x << 7) + (x & 127 as u64);
                        if x & 128 as u64 == 0 as u64 { break '__b3; }
                        break '__c3;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i == 9 && i < argc {
                if unsafe {
                            strlen(unsafe { *argv.offset(i as isize) } as *const i8)
                        } != 2 as u64 {
                    unsafe {
                        fprintf(__stderrp,
                            c"Not a hex byte: %s\n".as_ptr() as *mut i8 as *const i8,
                            unsafe { *argv.offset(i as isize) })
                    };
                    unsafe { exit(1) };
                }
                x =
                    ((hex_value(unsafe {
                                        *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                    }) << 4) +
                            hex_value(unsafe {
                                    *unsafe { (*argv.offset(i as isize)).offset(1 as isize) }
                                })) as u64;
                u_x = (u_x << 8) + x;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            if i < argc {
                unsafe {
                    fprintf(__stderrp,
                        c"Extra arguments: %s...\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { *argv.offset(i as isize) })
                };
                unsafe { exit(1) };
            }
        } else {
            let mut z: *const i8 =
                unsafe { *argv.offset(1 as isize) } as *const i8;
            let mut sign: i32 = 1;
            if unsafe { *z.offset(0 as isize) } as i32 == '+' as i32 {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            } else if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                sign = -1;
            }
            u_x = 0 as u64;
            while unsafe { *z.offset(0 as isize) } != 0 {
                if (unsafe { *z.offset(0 as isize) } as i32) < '0' as i32 ||
                        unsafe { *z.offset(0 as isize) } as i32 > '9' as i32 {
                    unsafe {
                        fprintf(__stderrp,
                            c"Not a decimal number: %s".as_ptr() as *mut i8 as
                                *const i8, unsafe { *argv.offset(1 as isize) })
                    };
                    unsafe { exit(1) };
                }
                u_x =
                    u_x * 10 as u64 + unsafe { *z.offset(0 as isize) } as u64 -
                        '0' as i32 as u64;
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if sign < 0 {
                unsafe {
                    memcpy(&raw mut i_x as *mut (), &raw mut u_x as *const (),
                        8 as u64)
                };
                i_x = -i_x;
                unsafe {
                    memcpy(&raw mut u_x as *mut (), &raw mut i_x as *const (),
                        8 as u64)
                };
            }
        }
        n = put_varint(&raw mut z_hex[0 as usize] as *mut u8, u_x);
        unsafe {
            printf(c"%lld =".as_ptr() as *mut i8 as *const i8, u_x as i64)
        };
        {
            i = 0;
            '__b5: loop {
                if !(i < n) { break '__b5; }
                '__c5: loop {
                    unsafe {
                        printf(c" %c%c".as_ptr() as *mut i8 as *const i8,
                            to_hex((z_hex[i as usize] as i32 >> 4) as u8) as i32,
                            to_hex((z_hex[i as usize] as i32 & 15) as u8) as i32)
                    };
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
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
    fn exit(_: i32)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
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