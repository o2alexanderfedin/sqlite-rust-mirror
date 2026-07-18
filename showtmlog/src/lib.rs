type DarwinSizeT = u64;
extern "C" fn decode_timestamp(a: *const u8) -> *const i8 {
    unsafe {
        let mut ms: u64 = 0 as u64;
        let mut days: u64 = 0 as u64;
        let mut sod: u64 = 0 as u64;
        let mut z: u64 = 0 as u64;
        let mut era: u64 = 0 as u64;
        let mut i: i32 = 0;
        let mut h: i32 = 0;
        let mut m: i32 = 0;
        let mut s: i32 = 0;
        let mut f: i32 = 0;
        let mut y: i32 = 0;
        let mut m: i32 = 0;
        let mut d: i32 = 0;
        let mut y: i32 = 0;
        let mut doe: u32 = 0 as u32;
        let mut yoe: u32 = 0 as u32;
        let mut doy: u32 = 0 as u32;
        let mut mp: u32 = 0 as u32;
        {
            { ({ ms = 0 as u64; ms }) as i32; i = 0 };
            '__b0: loop {
                if !(i <= 5) { break '__b0; }
                '__c0: loop {
                    ms = (ms << 8) + unsafe { *a.offset(i as isize) } as u64;
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if ms == 0 as u64 {
            return c"                       ".as_ptr() as *mut i8 as
                    *const i8;
        } else if ms > 4102444800000i64 as u64 {
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
extern "C" fn render_csv(i_file_1: i32, a: *const u8) -> () {
    let mut a2: u32 = 0 as u32;
    let mut a3: u32 = 0 as u32;
    let mut j: i32 = 0;
    let mut ms: u64 = 0 as u64;
    {
        { ({ ms = 0 as u64; ms }) as i32; j = 2 };
        '__b1: loop {
            if !(j <= 7) { break '__b1; }
            '__c1: loop {
                ms = (ms << 8) + unsafe { *a.offset(j as isize) } as u64;
                break '__c1;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        printf(c"%u.%03u,%d,".as_ptr() as *mut i8 as *const i8,
            (ms / 1000 as u64) as u32, (ms % 1000 as u64) as u32, i_file_1)
    };
    {
        { ({ a2 = 0 as u32; a2 }) as i32; j = 8 };
        '__b2: loop {
            if !(j <= 11) { break '__b2; }
            '__c2: loop {
                a2 = (a2 << 8) + unsafe { *a.offset(j as isize) } as u32;
                break '__c2;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        { ({ a3 = 0 as u32; a3 }) as i32; j = 12 };
        '__b3: loop {
            if !(j <= 15) { break '__b3; }
            '__c3: loop {
                a3 = (a3 << 8) + unsafe { *a.offset(j as isize) } as u32;
                break '__c3;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    '__s4:
        {
        match unsafe { *a.offset(0 as isize) } {
            1 => {
                {
                    unsafe {
                        printf(c"\"open-db\",%u,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"open-wal\",%u,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-page\",,%u,%u,,%d\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3, unsafe { *a.offset(1 as isize) } as i32)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"db-page\",,%u,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-start\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-page\",,%u,%u,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            2 => {
                {
                    unsafe {
                        printf(c"\"open-wal\",%u,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-page\",,%u,%u,,%d\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3, unsafe { *a.offset(1 as isize) } as i32)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"db-page\",,%u,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-start\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-page\",,%u,%u,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            3 => {
                {
                    unsafe {
                        printf(c"\"wal-page\",,%u,%u,,%d\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3, unsafe { *a.offset(1 as isize) } as i32)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"db-page\",,%u,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-start\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-page\",,%u,%u,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            4 => {
                {
                    unsafe {
                        printf(c"\"db-page\",,%u,,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-start\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-page\",,%u,%u,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            5 => {
                {
                    unsafe {
                        printf(c"\"ckpt-start\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-page\",,%u,%u,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            6 => {
                {
                    unsafe {
                        printf(c"\"ckpt-page\",,%u,%u,,\r\n".as_ptr() as *mut i8 as
                                *const i8, a2, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            7 => {
                {
                    unsafe {
                        printf(c"\"ckpt-end\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            8 => {
                {
                    unsafe {
                        printf(c"\"wal-reset\",,,,%u,\r\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            14 => {
                {
                    unsafe {
                        printf(c"\"close-wal\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            15 => {
                {
                    unsafe {
                        printf(c"\"close-db\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
            _ => {
                {
                    unsafe {
                        printf(c"\"invalid-record\",,,,,\r\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    break '__s4;
                }
            }
        }
    }
}
extern "C" fn render_text(a: *const u8) -> () {
    let mut a2: u32 = 0 as u32;
    let mut a3: u32 = 0 as u32;
    let mut j: i32 = 0;
    unsafe {
        printf(c"%s ".as_ptr() as *mut i8 as *const i8,
            decode_timestamp(unsafe { a.offset(2 as isize) } as *const u8))
    };
    {
        { ({ a2 = 0 as u32; a2 }) as i32; j = 8 };
        '__b5: loop {
            if !(j <= 11) { break '__b5; }
            '__c5: loop {
                a2 = (a2 << 8) + unsafe { *a.offset(j as isize) } as u32;
                break '__c5;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        { ({ a3 = 0 as u32; a3 }) as i32; j = 12 };
        '__b6: loop {
            if !(j <= 15) { break '__b6; }
            '__c6: loop {
                a3 = (a3 << 8) + unsafe { *a.offset(j as isize) } as u32;
                break '__c6;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    '__s7:
        {
        match unsafe { *a.offset(0 as isize) } {
            1 => {
                {
                    unsafe {
                        printf(c"open-db   pid %u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"open-wal  pid %u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-page  pgno %-8u frame %-8u%s\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3,
                            if unsafe { *a.offset(1 as isize) } as i32 == 1 {
                                c" txn".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 })
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"db-page   pgno %-8u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-start\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-page pgno %-8u frame %-8u\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            2 => {
                {
                    unsafe {
                        printf(c"open-wal  pid %u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-page  pgno %-8u frame %-8u%s\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3,
                            if unsafe { *a.offset(1 as isize) } as i32 == 1 {
                                c" txn".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 })
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"db-page   pgno %-8u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-start\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-page pgno %-8u frame %-8u\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            3 => {
                {
                    unsafe {
                        printf(c"wal-page  pgno %-8u frame %-8u%s\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3,
                            if unsafe { *a.offset(1 as isize) } as i32 == 1 {
                                c" txn".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 })
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"db-page   pgno %-8u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-start\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-page pgno %-8u frame %-8u\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            4 => {
                {
                    unsafe {
                        printf(c"db-page   pgno %-8u\n".as_ptr() as *mut i8 as
                                *const i8, a2)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-start\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-page pgno %-8u frame %-8u\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            5 => {
                {
                    unsafe {
                        printf(c"ckpt-start\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-page pgno %-8u frame %-8u\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            6 => {
                {
                    unsafe {
                        printf(c"ckpt-page pgno %-8u frame %-8u\n".as_ptr() as
                                    *mut i8 as *const i8, a2, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            7 => {
                {
                    unsafe {
                        printf(c"ckpt-end\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            8 => {
                {
                    unsafe {
                        printf(c"wal-reset salt1 0x%08x\n".as_ptr() as *mut i8 as
                                *const i8, a3)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            14 => {
                {
                    unsafe {
                        printf(c"close-wal\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            15 => {
                {
                    unsafe {
                        printf(c"close-db\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
            _ => {
                {
                    unsafe {
                        printf(c"invalid-record\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s7;
                }
            }
        }
    }
}
extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        printf(c"Usage: %s [--csv] LOGFILE ...\n".as_ptr() as *mut i8 as
                *const i8, argv0)
    };
    unsafe {
        printf(c"Decode one or more tmstmpvfs log files and display the results\non stdout.  Render as CSV if the --csv option is used.\n".as_ptr()
                    as *mut i8 as *const i8)
    };
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    let mut i: i32 = 0;
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut a: [u8; 16] = [0; 16];
    let mut b_csv: i32 = 0;
    let mut z: *const i8 = core::ptr::null();
    let mut n_file: i32 = 0;
    let mut i_file: i32 = 0;
    {
        i = 1;
        '__b8: loop {
            if !(i < argc) { break '__b8; }
            '__c8: loop {
                z = unsafe { *argv.offset(i as isize) } as *const i8;
                if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                    if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe {
                                strcmp(z, c"-csv".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        b_csv = 1;
                    } else if unsafe {
                                    strcmp(z, c"-help".as_ptr() as *mut i8 as *const i8)
                                } == 0 ||
                            unsafe { strcmp(z, c"-?".as_ptr() as *mut i8 as *const i8) }
                                == 0 {
                        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                        return Ok(());
                    } else {
                        unsafe {
                            printf(c"unknown command-line option: \"%s\"\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                        return Err(1);
                    }
                } else {
                    { let __p = &mut n_file; let __t = *__p; *__p += 1; __t };
                }
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_file == 0 {
        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
        return Err(1);
    }
    i_file = 0;
    if b_csv != 0 {
        unsafe {
            printf(c"tmstmp,fileno,op,pid,pgno,frame,salt,txn\r\n".as_ptr() as
                        *mut i8 as *const i8)
        };
    }
    {
        i = 1;
        '__b9: loop {
            if !(i < argc) { break '__b9; }
            '__c9: loop {
                z = unsafe { *argv.offset(i as isize) } as *const i8;
                if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                    break '__c9;
                }
                in_ =
                    unsafe { fopen(z, c"rb".as_ptr() as *mut i8 as *const i8) };
                if in_ == core::ptr::null_mut() {
                    unsafe {
                        printf(c"%s: can\'t open\n".as_ptr() as *mut i8 as
                                *const i8, z)
                    };
                    break '__c9;
                }
                { let __p = &mut i_file; let __t = *__p; *__p += 1; __t };
                if n_file > 1 && (b_csv == 0) as i32 != 0 {
                    unsafe {
                        printf(c"*** %s ***\n".as_ptr() as *mut i8 as *const i8, z)
                    };
                }
                while 16 as u64 ==
                        unsafe {
                            fread(&raw mut a[0 as usize] as *mut u8 as *mut (),
                                1 as u64, 16 as u64, in_)
                        } {
                    if b_csv != 0 {
                        render_csv(i_file,
                            &raw mut a[0 as usize] as *mut u8 as *const u8);
                    } else {
                        render_text(&raw mut a[0 as usize] as *mut u8 as *const u8);
                    }
                }
                unsafe { fclose(in_) };
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return Ok(());
}
static mut z_out: [i8; 50] = unsafe { core::mem::zeroed() };
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn snprintf(__str: *mut i8, __size: u64, __format: *const i8, ...)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;