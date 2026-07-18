#[repr(C)]
#[derive(Copy, Clone)]
struct U256 {
    a: [u64; 4],
}
extern "C" fn u256_times_10(p_x_1: &mut U256) -> () {
    let mut carry: u64 = 0 as u64;
    let mut i: i32 = 0;
    {
        i = 3;
        '__b0: loop {
            if !(i >= 0) { break '__b0; }
            '__c0: loop {
                let y: u128 =
                    10 as u128 * (*p_x_1).a[i as usize] as u128 + carry as u128;
                (*p_x_1).a[i as usize] = y as u64;
                carry = (y >> 64) as u64;
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
}
extern "C" fn u256_times_2(p_x_1: &mut U256) -> () {
    let mut carry: u64 = 0 as u64;
    let mut i: i32 = 0;
    {
        i = 3;
        '__b1: loop {
            if !(i >= 0) { break '__b1; }
            '__c1: loop {
                let y: u64 = (*p_x_1).a[i as usize];
                (*p_x_1).a[i as usize] = y << 1 | carry;
                carry = y >> 63;
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
}
extern "C" fn u256_div_10(p_x_1: &mut U256) -> () {
    let mut rem: u64 = 0 as u64;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b2: loop {
            if !(i < 4) { break '__b2; }
            '__c2: loop {
                let acc: u128 =
                    (rem as u128) << 64 | (*p_x_1).a[i as usize] as u128;
                (*p_x_1).a[i as usize] = (acc / 10 as u128) as u64;
                rem = (acc % 10 as u128) as u64;
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn u256_div_2(p_x_1: &mut U256) -> () {
    let mut rem: u64 = 0 as u64;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b3: loop {
            if !(i < 4) { break '__b3; }
            '__c3: loop {
                let y: u64 = (*p_x_1).a[i as usize];
                (*p_x_1).a[i as usize] = y >> 1 | rem << 63;
                rem = y & 1 as u64;
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut i_next: i32 = 0;
        let mut e: i32 = 0;
        let mut b_round: i32 = 0;
        let mut b_truth: i32 = 0;
        let top: u64 = ((1 as u64) << 63) as u64;
        let mut v: U256 = unsafe { core::mem::zeroed() };
        let mut a_hi: [u64; 699] = [0; 699];
        let mut a_lo: [u64; 699] = [0; 699];
        let mut a_e: [i32; 699] = [0; 699];
        {
            i = 1;
            '__b4: loop {
                if !(i < argc) { break '__b4; }
                '__c4: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 &&
                                unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 &&
                            unsafe { *z.offset(2 as isize) } as i32 != 0 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe {
                                strcmp(z, c"-round".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        b_round = 1;
                    } else if unsafe {
                                strcmp(z, c"-truth".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        b_truth = 1;
                    } else {
                        unsafe {
                            fprintf(__stderrp,
                                c"unknown option: \"%s\"\n".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        unsafe { exit(1) };
                    }
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        v.a[0 as usize] = top as u64;
        v.a[1 as usize] = 0 as u64;
        v.a[2 as usize] = 0 as u64;
        v.a[3 as usize] = 0 as u64;
        {
            { i = 351; e = 63 };
            '__b5: loop {
                if !(i >= 0) { break '__b5; }
                '__c5: loop {
                    a_hi[i as usize] = v.a[0 as usize];
                    a_lo[i as usize] = v.a[1 as usize];
                    a_e[i as usize] = e;
                    u256_div_10(&mut v);
                    while v.a[0 as usize] < top {
                        { let __p = &mut e; let __t = *__p; *__p += 1; __t };
                        u256_times_2(&mut v);
                    }
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        v.a[0 as usize] = 0 as u64;
        v.a[1 as usize] = top as u64;
        v.a[2 as usize] = 0 as u64;
        v.a[3 as usize] = 0 as u64;
        {
            { i = 351 + 1; e = 63 };
            '__b7: loop {
                if !(i < 347 - -351 + 1) { break '__b7; }
                '__c7: loop {
                    u256_times_10(&mut v);
                    while v.a[0 as usize] > 0 as u64 {
                        { let __p = &mut e; let __t = *__p; *__p -= 1; __t };
                        u256_div_2(&mut v);
                    }
                    a_hi[i as usize] = v.a[1 as usize];
                    a_lo[i as usize] = v.a[2 as usize];
                    a_e[i as usize] = e;
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if b_truth != 0 {
            unsafe {
                printf(c"  /* Powers of ten, accurate to 128 bits each */\n".as_ptr()
                            as *mut i8 as *const i8)
            };
            unsafe {
                printf(c"  static const struct {u64 hi; u64 lo;} aTruth[] = {\n".as_ptr()
                            as *mut i8 as *const i8)
            };
            {
                i = 0;
                '__b9: loop {
                    if !(i < 347 - -351 + 1) { break '__b9; }
                    '__c9: loop {
                        let mut x: u64 = a_hi[i as usize];
                        let y: u64 = a_lo[i as usize];
                        let mut e: i32 = a_e[i as usize];
                        let mut z_op: *mut i8 = core::ptr::null_mut();
                        if e > 0 {
                            z_op = c"<<".as_ptr() as *mut i8;
                        } else { e = -e; z_op = c">>".as_ptr() as *mut i8; }
                        unsafe {
                            printf(c"   {0x%016llx, 0x%016llx}, /* %2d: 1.0e%+d %s %d */\n".as_ptr()
                                        as *mut i8 as *const i8, x, y, i, i + -351, z_op, e)
                        };
                        break '__c9;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { printf(c"  };\n".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe {
            printf(c"  static const u64 aBase[] = {\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        {
            { i = 351; j = 0 };
            '__b10: loop {
                if !(i < 351 + 27) { break '__b10; }
                '__c10: loop {
                    let mut x: u64 = a_hi[i as usize];
                    let mut e: i32 = a_e[i as usize];
                    let mut z_op_1: *mut i8 = core::ptr::null_mut();
                    if e > 0 {
                        z_op_1 = c"<<".as_ptr() as *mut i8;
                    } else { e = -e; z_op_1 = c">>".as_ptr() as *mut i8; }
                    unsafe {
                        printf(c"    UINT64_C(0x%016llx), /* %2d: 1.0e%+d %s %d */\n".as_ptr()
                                    as *mut i8 as *const i8, x, j, i + -351, z_op_1, e)
                    };
                    break '__c10;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        unsafe { printf(c"  };\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"  static const u64 aScale[] = {\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        {
            i = { j = 0; j };
            '__b11: loop {
                if !(i < 347 - -351 + 1) { break '__b11; }
                '__c11: loop {
                    let mut z_extra: *const i8 =
                        c"".as_ptr() as *mut i8 as *const i8;
                    i_next = i + 27;
                    if i == 351 {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        z_extra =
                            c" (special case)".as_ptr() as *mut i8 as *const i8;
                    }
                    let mut x: u64 = a_hi[i as usize];
                    let mut e: i32 = a_e[i as usize];
                    let mut z_op_2: *mut i8 = core::ptr::null_mut();
                    if e > 0 {
                        z_op_2 = c"<<".as_ptr() as *mut i8;
                    } else { e = -e; z_op_2 = c">>".as_ptr() as *mut i8; }
                    unsafe {
                        printf(c"    UINT64_C(0x%016llx), /* %2d: 1.0e%+d %s %d%s */\n".as_ptr()
                                    as *mut i8 as *const i8, x, j, i + -351, z_op_2, e, z_extra)
                    };
                    break '__c11;
                }
                {
                    i = i_next;
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        unsafe { printf(c"  };\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"  static const unsigned int aScaleLo[] = {\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        {
            i = { j = 0; j };
            '__b12: loop {
                if !(i < 347 - -351 + 1) { break '__b12; }
                '__c12: loop {
                    let mut z_extra_1: *const i8 =
                        c"".as_ptr() as *mut i8 as *const i8;
                    i_next = i + 27;
                    if i == 351 {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        z_extra_1 =
                            c" (special case)".as_ptr() as *mut i8 as *const i8;
                    }
                    let mut x: u64 = a_lo[i as usize];
                    let mut e: i32 = a_e[i as usize];
                    let mut z_op_3: *mut i8 = core::ptr::null_mut();
                    if b_round != 0 && x & (1 as u64) << 31 != 0 as u64 &&
                            i != 351 - 1 {
                        x += (1 as u64) << 32;
                    }
                    if e > 0 {
                        z_op_3 = c"<<".as_ptr() as *mut i8;
                    } else { e = -e; z_op_3 = c">>".as_ptr() as *mut i8; }
                    unsafe {
                        printf(c"    0x%08llx, /* %2d: 1.0e%+d %s %d%s */\n".as_ptr()
                                    as *mut i8 as *const i8, x >> 32, j, i + -351, z_op_3, e,
                            z_extra_1)
                    };
                    break '__c12;
                }
                {
                    i = i_next;
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        unsafe { printf(c"  };\n".as_ptr() as *mut i8 as *const i8) };
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
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
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