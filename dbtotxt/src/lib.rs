type DarwinSizeT = u64;

extern "C" fn all_zero(a_line_1: *const u8) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b0: loop {
            if !(i < 16 &&
                            unsafe { *a_line_1.offset(i as isize) } as i32 == 0) {
                break '__b0;
            }
            '__c0: loop { break '__c0; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (i == 16) as i32;
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut pgsz: i32 = 0;
        let mut for_cli: i32 = 0;
        let mut b_sql: i32 = 0;
        let mut sz_file: i64 = 0 as i64;
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut n_sql: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut n_err: i32 = 0;
        let mut z_input_file: *const i8 = core::ptr::null();
        let mut z_base_name: *const i8 = core::ptr::null();
        let mut last_page: i32 = 0;
        let mut i_page: i32 = 0;
        let mut b_raw: i32 = 0;
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut a_line: *mut u8 = core::ptr::null_mut();
        let mut a_hdr: *const u8 = core::ptr::null();
        let mut b_show: [u8; 256] = [0; 256];
        unsafe {
            memset(&raw mut b_show[0 as usize] as *mut u8 as *mut (),
                '.' as i32, core::mem::size_of::<[u8; 256]>() as u64)
        };
        {
            i = ' ' as i32;
            '__b1: loop {
                if !(i <= '~' as i32) { break '__b1; }
                '__c1: loop {
                    if i != '{' as i32 && i != '}' as i32 && i != '\"' as i32 &&
                            i != '\\' as i32 {
                        b_show[i as usize] = i as u8;
                    }
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 1;
            '__b2: loop {
                if !(i < argc) { break '__b2; }
                '__c2: loop {
                    if unsafe {
                                    *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                } as i32 == '-' as i32 {
                        let mut z: *const i8 =
                            unsafe { *argv.offset(i as isize) } as *const i8;
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        if unsafe {
                                    strcmp(z, c"pagesize".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            pgsz =
                                unsafe {
                                    atoi(unsafe { *argv.offset(i as isize) } as *const i8)
                                };
                            if pgsz < 512 || pgsz > 65536 || pgsz & pgsz - 1 != 0 {
                                eprintln!("Page size must be a power of two between 512 and 65536.");
                                { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
                            }
                            break '__c2;
                        } else if unsafe {
                                    strcmp(z, c"for-cli".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            for_cli = 1;
                            break '__c2;
                        } else if unsafe {
                                    strcmp(z, c"script".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            for_cli = 1;
                            b_sql = 1;
                            break '__c2;
                        } else if unsafe {
                                    strcmp(z, c"raw".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            b_raw = 1;
                            break '__c2;
                        }
                        unsafe {
                            fprintf(__stderrp,
                                c"Unknown option: %s\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { *argv.offset(i as isize) })
                        };
                        { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
                    } else if !(z_input_file).is_null() {
                        unsafe {
                            fprintf(__stderrp,
                                c"Already using a different input file: [%s]\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
                    } else {
                        z_input_file =
                            unsafe { *argv.offset(i as isize) } as *const i8;
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if z_input_file == core::ptr::null() {
            eprintln!("No input file specified.");
            { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
        }
        if n_err != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s [--pagesize N] [--script] [--for-cli] FILENAME\n".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) })
            };
            unsafe { exit(1) };
        }
        in_ =
            unsafe {
                fopen(z_input_file, c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if in_ == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"Cannot open input file [%s]\n".as_ptr() as *mut i8 as
                        *const i8, z_input_file)
            };
            unsafe { exit(1) };
        }
        unsafe { fseek(in_, 0 as i64, 2) };
        sz_file = unsafe { ftell(in_) };
        unsafe { rewind(in_) };
        if sz_file < 100 as i64 && (b_raw == 0) as i32 != 0 {
            eprintln!("File too short. Minimum size is 100 bytes.");
            unsafe { exit(1) };
        }
        a_data = unsafe { malloc((sz_file + 16 as i64) as u64) } as *mut u8;
        if a_data == core::ptr::null_mut() {
            eprintln!("Failed to allocate {} bytes", sz_file as i64);
            unsafe { exit(1) };
        }
        if unsafe { fread(a_data as *mut (), sz_file as u64, 1 as u64, in_) }
                != 1 as u64 {
            eprintln!("Cannot read file info memory");
            unsafe { exit(1) };
        }
        unsafe {
            memset(unsafe { a_data.offset(sz_file as isize) } as *mut (), 0,
                16 as u64)
        };
        unsafe { fclose(in_) };
        if b_sql != 0 {
            {
                i = 0;
                '__b3: loop {
                    if !((i as i64) < sz_file &&
                                    unsafe { *a_data.offset(i as isize) } as i32 != 0) {
                        break '__b3;
                    }
                    '__c3: loop { break '__c3; }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i as i64 == sz_file {
                eprintln!("No zero terminator on SQL script");
                unsafe { exit(1) };
            }
            n_sql = i + 1;
            if (sz_file - n_sql as i64) < 100 as i64 &&
                    (b_raw == 0) as i32 != 0 {
                eprintln!("Less than 100 bytes in the database");
                unsafe { exit(1) };
            }
        } else { n_sql = 0; }
        a_hdr = unsafe { a_data.offset(n_sql as isize) };
        if pgsz == 0 {
            if b_raw != 0 {
                pgsz = 1024;
            } else {
                pgsz =
                    (unsafe { *a_hdr.offset(16 as isize) } as i32) << 8 |
                        unsafe { *a_hdr.offset(17 as isize) } as i32;
                if pgsz == 1 { pgsz = 65536; }
                if pgsz < 512 || pgsz & pgsz - 1 != 0 {
                    eprintln!("Invalid page size in header: {}", pgsz as i32);
                    unsafe { exit(1) };
                }
            }
        }
        z_base_name = z_input_file;
        {
            i = 0;
            '__b4: loop {
                if !(unsafe { *z_input_file.offset(i as isize) } != 0) {
                    break '__b4;
                }
                '__c4: loop {
                    if unsafe { *z_input_file.offset(i as isize) } as i32 ==
                                '/' as i32 &&
                            unsafe { *z_input_file.offset((i + 1) as isize) } as i32 !=
                                0 {
                        z_base_name =
                            unsafe {
                                unsafe {
                                    z_input_file.offset(i as isize).offset(1 as isize)
                                }
                            };
                    }
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if for_cli != 0 {
            unsafe {
                printf(c".open --hexdb\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            printf(c"| size %d pagesize %d filename %s\n".as_ptr() as *mut i8
                    as *const i8, sz_file as i32, pgsz, z_base_name)
        };
        {
            i = n_sql;
            '__b5: loop {
                if !((i as i64) < sz_file) { break '__b5; }
                '__c5: loop {
                    a_line = unsafe { a_data.offset(i as isize) };
                    if all_zero(a_line as *const u8) != 0 { break '__c5; }
                    i_page = i / pgsz + 1;
                    if last_page != i_page {
                        unsafe {
                            printf(c"| page %d offset %d\n".as_ptr() as *mut i8 as
                                    *const i8, i_page, (i_page - 1) * pgsz)
                        };
                        last_page = i_page;
                    }
                    unsafe {
                        printf(c"|  %5d:".as_ptr() as *mut i8 as *const i8,
                            i - (i_page - 1) * pgsz)
                    };
                    {
                        j = 0;
                        '__b6: loop {
                            if !(j < 16) { break '__b6; }
                            '__c6: loop {
                                unsafe {
                                    printf(c" %02x".as_ptr() as *mut i8 as *const i8,
                                        unsafe { *a_line.offset(j as isize) } as i32)
                                };
                                break '__c6;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { printf(c"   ".as_ptr() as *mut i8 as *const i8) };
                    {
                        j = 0;
                        '__b7: loop {
                            if !(j < 16) { break '__b7; }
                            '__c7: loop {
                                let c: u8 = unsafe { *a_line.offset(j as isize) } as u8;
                                unsafe { fputc(b_show[c as usize] as i32, __stdoutp) };
                                break '__c7;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { fputc('\n' as i32, __stdoutp) };
                    break '__c5;
                }
                i += 16;
            }
        }
        unsafe {
            printf(c"| end %s\n".as_ptr() as *mut i8 as *const i8,
                z_base_name)
        };
        if n_sql > 0 {
            unsafe {
                printf(c"%s\n".as_ptr() as *mut i8 as *const i8, a_data)
            };
        }
        unsafe { free(a_data as *mut ()) };
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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn atoi(_: *const i8)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
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
    fn printf(_: *const i8, ...)
    -> i32;
    fn fputc(_: i32, _: *mut FILE)
    -> i32;
    fn free(_: *mut ())
    -> ();
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
