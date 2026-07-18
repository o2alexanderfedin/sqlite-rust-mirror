#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
type DarwinSizeT = u64;
extern "C" fn show_help(z_argv0_1: *const i8) -> () {
    unsafe {
        printf(c"Usage: %s [options] DATABASE ...\n".as_ptr() as *mut i8 as
                *const i8, z_argv0_1)
    };
    unsafe {
        printf(c"Read databases into an in-memory filesystem.  Run test SQL as specified\nby command-line arguments or from\n\n    SELECT group_concat(sql) FROM autoexec;\n\nOptions:\n  --help              Show this help text\n  -q|--quiet          Reduced output\n  --limit-mem N       Limit memory used by test SQLite instances to N bytes\n  --limit-vdbe        Panic if any test runs for more than 100,000 cycles\n  --no-lookaside      Disable the lookaside memory allocator\n  --timeout N         Timeout after N seconds.\n  --trace             Show the results of each SQL command\n  -v|--verbose        Increased output.  Repeat for more output.\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe { exit(0) };
}
unsafe extern "C" fn fatal_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_format_1, ap) };
        ();
        eprintln!("");
        unsafe { exit(1) };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VFile {
    z_filename: *mut i8,
    sz: i32,
    n_ref: i32,
    a: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VHandle {
    base: Sqlite3File,
    p_v_file: *mut VFile,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct GlobalVars {
    a_file: [VFile; 10],
}
static mut g: GlobalVars = unsafe { core::mem::zeroed() };
extern "C" fn format_vfs() -> () {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b0: loop {
                if !(i < 10) { break '__b0; }
                '__c0: loop {
                    g.a_file[i as usize].sz = -1;
                    g.a_file[i as usize].z_filename = core::ptr::null_mut();
                    g.a_file[i as usize].a = core::ptr::null_mut();
                    g.a_file[i as usize].n_ref = 0;
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
extern "C" fn reformat_vfs() -> () {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b1: loop {
                if !(i < 10) { break '__b1; }
                '__c1: loop {
                    if g.a_file[i as usize].sz < 0 { break '__c1; }
                    if !(g.a_file[i as usize].z_filename).is_null() {
                        unsafe { free(g.a_file[i as usize].z_filename as *mut ()) };
                        g.a_file[i as usize].z_filename = core::ptr::null_mut();
                    }
                    if g.a_file[i as usize].n_ref > 0 {
                        unsafe {
                            fatal_error(c"file %d still open.  nRef=%d".as_ptr() as
                                        *mut i8 as *const i8, i, g.a_file[i as usize].n_ref)
                        };
                    }
                    g.a_file[i as usize].sz = -1;
                    unsafe { free(g.a_file[i as usize].a as *mut ()) };
                    g.a_file[i as usize].a = core::ptr::null_mut();
                    g.a_file[i as usize].n_ref = 0;
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
extern "C" fn find_v_file(z_name_1: *const i8) -> *mut VFile {
    unsafe {
        let mut i: i32 = 0;
        if z_name_1 == core::ptr::null() { return core::ptr::null_mut(); }
        {
            i = 0;
            '__b2: loop {
                if !(i < 10) { break '__b2; }
                '__c2: loop {
                    if g.a_file[i as usize].z_filename == core::ptr::null_mut()
                        {
                        break '__c2;
                    }
                    if unsafe {
                                strcmp(g.a_file[i as usize].z_filename as *const i8,
                                    z_name_1)
                            } == 0 {
                        return &mut g.a_file[i as usize];
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return core::ptr::null_mut();
    }
}
extern "C" fn create_v_file(z_name_1: *const i8, z_disk_file_1: *const i8)
    -> *mut VFile {
    unsafe {
        let mut p_new: *mut VFile = find_v_file(z_name_1);
        let mut i: i32 = 0;
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut sz: i64 = 0 as i64;
        if !(p_new).is_null() { return p_new; }
        {
            i = 0;
            '__b3: loop {
                if !(i < 10 && g.a_file[i as usize].sz >= 0) { break '__b3; }
                '__c3: loop { break '__c3; }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if i >= 10 { return core::ptr::null_mut(); }
        if !(z_disk_file_1).is_null() {
            in_ =
                unsafe {
                    fopen(z_disk_file_1, c"rb".as_ptr() as *mut i8 as *const i8)
                };
            if in_ == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"no such file: \"%s\"".as_ptr() as *mut i8 as
                            *const i8, z_disk_file_1)
                };
            }
            unsafe { fseek(in_, 0 as i64, 2) };
            sz = unsafe { ftell(in_) };
            unsafe { rewind(in_) };
        }
        p_new = &mut g.a_file[i as usize];
        if !(z_name_1).is_null() {
            let n_name: i32 = unsafe { strlen(z_name_1) } as i32 + 1;
            unsafe {
                (*p_new).z_filename =
                    unsafe { malloc(n_name as u64) } as *mut i8
            };
            if unsafe { (*p_new).z_filename } == core::ptr::null_mut() {
                if !(in_).is_null() { unsafe { fclose(in_) }; }
                return core::ptr::null_mut();
            }
            unsafe {
                memcpy(unsafe { (*p_new).z_filename } as *mut (),
                    z_name_1 as *const (), n_name as u64)
            };
        } else { unsafe { (*p_new).z_filename = core::ptr::null_mut() }; }
        unsafe { (*p_new).n_ref = 0 };
        unsafe { (*p_new).sz = sz as i32 };
        unsafe { (*p_new).a = unsafe { malloc(sz as u64) } as *mut u8 };
        if sz > 0 as i64 {
            if unsafe { (*p_new).a } == core::ptr::null_mut() ||
                    unsafe {
                            fread(unsafe { (*p_new).a } as *mut (), sz as u64, 1 as u64,
                                in_)
                        } < 1 as u64 {
                unsafe { free(unsafe { (*p_new).z_filename } as *mut ()) };
                unsafe { free(unsafe { (*p_new).a } as *mut ()) };
                unsafe { (*p_new).a = core::ptr::null_mut() };
                unsafe { (*p_new).z_filename = core::ptr::null_mut() };
                unsafe { (*p_new).sz = -1 };
                p_new = core::ptr::null_mut();
            }
        }
        if !(in_).is_null() { unsafe { fclose(in_) }; }
        return p_new;
    }
}
extern "C" fn inmem_close(p_file_1: *mut Sqlite3File) -> i32 {
    let p: *const VHandle = p_file_1 as *mut VHandle as *const VHandle;
    let p_v_file: *mut VFile = unsafe { (*p).p_v_file };
    {
        let __p = unsafe { &mut (*p_v_file).n_ref };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    if unsafe { (*p_v_file).n_ref } == 0 &&
            unsafe { (*p_v_file).z_filename } == core::ptr::null_mut() {
        unsafe { (*p_v_file).sz = -1 };
        unsafe { free(unsafe { (*p_v_file).a } as *mut ()) };
        unsafe { (*p_v_file).a = core::ptr::null_mut() };
    }
    return 0;
}
extern "C" fn inmem_read(p_file_1: *mut Sqlite3File, p_data_1: *mut (),
    mut i_amt_1: i32, i_ofst_1: Sqlite3Int64) -> i32 {
    let p_handle: *const VHandle = p_file_1 as *mut VHandle as *const VHandle;
    let p_v_file: *const VFile =
        unsafe { (*p_handle).p_v_file } as *const VFile;
    if i_ofst_1 < 0 as i64 || i_ofst_1 >= unsafe { (*p_v_file).sz } as i64 {
        unsafe { memset(p_data_1, 0, i_amt_1 as u64) };
        return 10 | 2 << 8;
    }
    if i_ofst_1 + i_amt_1 as Sqlite3Int64 > unsafe { (*p_v_file).sz } as i64 {
        unsafe { memset(p_data_1, 0, i_amt_1 as u64) };
        i_amt_1 =
            (unsafe { (*p_v_file).sz } as Sqlite3Int64 - i_ofst_1) as i32;
        unsafe {
            memcpy(p_data_1, unsafe { (*p_v_file).a } as *const (),
                i_amt_1 as u64)
        };
        return 10 | 2 << 8;
    }
    unsafe {
        memcpy(p_data_1,
            unsafe { unsafe { (*p_v_file).a.offset(i_ofst_1 as isize) } } as
                *const (), i_amt_1 as u64)
    };
    return 0;
}
extern "C" fn inmem_write(p_file_1: *mut Sqlite3File, p_data_1: *const (),
    i_amt_1: i32, i_ofst_1: Sqlite3Int64) -> i32 {
    let p_handle: *const VHandle = p_file_1 as *mut VHandle as *const VHandle;
    let p_v_file: *mut VFile = unsafe { (*p_handle).p_v_file };
    if i_ofst_1 + i_amt_1 as Sqlite3Int64 > unsafe { (*p_v_file).sz } as i64 {
        let mut a_new: *mut u8 = core::ptr::null_mut();
        if i_ofst_1 + i_amt_1 as Sqlite3Int64 >= 1000000 as i64 { return 13; }
        a_new =
            unsafe {
                    realloc(unsafe { (*p_v_file).a } as *mut (),
                        (i_ofst_1 + i_amt_1 as Sqlite3Int64) as i32 as u64)
                } as *mut u8;
        if a_new == core::ptr::null_mut() { return 13; }
        unsafe { (*p_v_file).a = a_new };
        if i_ofst_1 > unsafe { (*p_v_file).sz } as i64 {
            unsafe {
                memset(unsafe {
                            unsafe {
                                (*p_v_file).a.offset(unsafe { (*p_v_file).sz } as isize)
                            }
                        } as *mut (), 0,
                    (i_ofst_1 - unsafe { (*p_v_file).sz } as Sqlite3Int64) as
                            i32 as u64)
            };
        }
        unsafe {
            (*p_v_file).sz = (i_ofst_1 + i_amt_1 as Sqlite3Int64) as i32
        };
    }
    unsafe {
        memcpy(unsafe { unsafe { (*p_v_file).a.offset(i_ofst_1 as isize) } }
                as *mut (), p_data_1, i_amt_1 as u64)
    };
    return 0;
}
extern "C" fn inmem_truncate(p_file_1: *mut Sqlite3File,
    i_size_1: Sqlite3Int64) -> i32 {
    let p_handle: *const VHandle = p_file_1 as *mut VHandle as *const VHandle;
    let p_v_file: *mut VFile = unsafe { (*p_handle).p_v_file };
    if unsafe { (*p_v_file).sz } as Sqlite3Int64 > i_size_1 &&
            i_size_1 >= 0 as i64 {
        unsafe { (*p_v_file).sz = i_size_1 as i32 };
    }
    return 0;
}
extern "C" fn inmem_sync(p_file_1: *mut Sqlite3File, flags: i32) -> i32 {
    return 0;
}
extern "C" fn inmem_file_size(p_file_1: *mut Sqlite3File,
    p_size_1: *mut Sqlite3Int64) -> i32 {
    unsafe {
        *p_size_1 =
            unsafe { (*unsafe { (*(p_file_1 as *mut VHandle)).p_v_file }).sz }
                as Sqlite3Int64
    };
    return 0;
}
extern "C" fn inmem_lock(p_file_1: *mut Sqlite3File, type__1: i32) -> i32 {
    return 0;
}
extern "C" fn inmem_unlock(p_file_1: *mut Sqlite3File, type__1: i32) -> i32 {
    return 0;
}
extern "C" fn inmem_check_reserved_lock(p_file_1: *mut Sqlite3File,
    p_out_1: *mut i32) -> i32 {
    unsafe { *p_out_1 = 0 };
    return 0;
}
extern "C" fn inmem_file_control(p_file_1: *mut Sqlite3File, op: i32,
    p_arg_1: *mut ()) -> i32 {
    return 12;
}
extern "C" fn inmem_sector_size(p_file_1: *mut Sqlite3File) -> i32 {
    return 512;
}
extern "C" fn inmem_device_characteristics(p_file_1: *mut Sqlite3File)
    -> i32 {
    return 512 | 2048 | 4096;
}
static mut v_handle_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(inmem_close),
        x_read: Some(inmem_read),
        x_write: Some(inmem_write),
        x_truncate: Some(inmem_truncate),
        x_sync: Some(inmem_sync),
        x_file_size: Some(inmem_file_size),
        x_lock: Some(inmem_lock),
        x_unlock: Some(inmem_unlock),
        x_check_reserved_lock: Some(inmem_check_reserved_lock),
        x_file_control: Some(inmem_file_control),
        x_sector_size: Some(inmem_sector_size),
        x_device_characteristics: Some(inmem_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };
extern "C" fn inmem_open(p_vfs_1: *mut Sqlite3Vfs, z_filename_1: *const i8,
    p_file_1: *mut Sqlite3File, open_flags_1: i32, p_out_flags_1: *mut i32)
    -> i32 {
    unsafe {
        let p_v_file: *mut VFile =
            create_v_file(z_filename_1, core::ptr::null());
        let p_handle: *mut VHandle = p_file_1 as *mut VHandle;
        if p_v_file == core::ptr::null_mut() { return 13; }
        unsafe { (*p_handle).p_v_file = p_v_file };
        {
            let __p = unsafe { &mut (*p_v_file).n_ref };
            let __t = *__p;
            *__p += 1;
            __t
        };
        unsafe {
            (*p_file_1).p_methods =
                &raw mut v_handle_methods as *const Sqlite3IoMethods
        };
        if !(p_out_flags_1).is_null() {
            unsafe { *p_out_flags_1 = open_flags_1 };
        }
        return 0;
    }
}
extern "C" fn inmem_delete(p_vfs_1: *mut Sqlite3Vfs, z_filename_1: *const i8,
    syncdir: i32) -> i32 {
    let p_v_file: *mut VFile = find_v_file(z_filename_1);
    if p_v_file == core::ptr::null_mut() { return 0; }
    if unsafe { (*p_v_file).n_ref } == 0 {
        unsafe { free(unsafe { (*p_v_file).z_filename } as *mut ()) };
        unsafe { (*p_v_file).z_filename = core::ptr::null_mut() };
        unsafe { (*p_v_file).sz = -1 };
        unsafe { free(unsafe { (*p_v_file).a } as *mut ()) };
        unsafe { (*p_v_file).a = core::ptr::null_mut() };
        return 0;
    }
    return 10 | 10 << 8;
}
extern "C" fn inmem_access(p_vfs_1: *mut Sqlite3Vfs, z_filename_1: *const i8,
    flags: i32, p_res_out_1: *mut i32) -> i32 {
    let p_v_file: *const VFile = find_v_file(z_filename_1) as *const VFile;
    unsafe { *p_res_out_1 = (p_v_file != core::ptr::null_mut()) as i32 };
    return 0;
}
extern "C" fn inmem_full_pathname(p_vfs_1: *mut Sqlite3Vfs,
    z_filename_1: *const i8, n_out_1: i32, z_out_1: *mut i8) -> i32 {
    unsafe {
        sqlite3_snprintf(n_out_1, z_out_1,
            c"%s".as_ptr() as *mut i8 as *const i8, z_filename_1)
    };
    return 0;
}
extern "C" fn inmem_vfs_register() -> () {
    unsafe {
        let p_default: *const Sqlite3Vfs =
            unsafe { sqlite3_vfs_find(core::ptr::null()) } as
                *const Sqlite3Vfs;
        inmem_vfs.i_version = 3;
        inmem_vfs.sz_os_file = core::mem::size_of::<VHandle>() as i32;
        inmem_vfs.mx_pathname = 200;
        inmem_vfs.z_name = c"inmem".as_ptr() as *mut i8 as *const i8;
        inmem_vfs.x_open = Some(inmem_open);
        inmem_vfs.x_delete = Some(inmem_delete);
        inmem_vfs.x_access = Some(inmem_access);
        inmem_vfs.x_full_pathname = Some(inmem_full_pathname);
        inmem_vfs.x_randomness = unsafe { (*p_default).x_randomness };
        inmem_vfs.x_sleep = unsafe { (*p_default).x_sleep };
        inmem_vfs.x_current_time_int64 =
            unsafe { (*p_default).x_current_time_int64 };
        unsafe { sqlite3_vfs_register(&mut inmem_vfs, 0) };
    }
}
extern "C" fn set_alarm(n_1: i32) -> () { { let _ = n_1; }; }
#[repr(C)]
#[derive(Copy, Clone)]
struct Str {
    z: *mut i8,
    n: Sqlite3Uint64,
    n_alloc: Sqlite3Uint64,
    oom_err: i32,
}
extern "C" fn str_init(p: *mut Str) -> () {
    unsafe { memset(p as *mut (), 0, core::mem::size_of::<Str>() as u64) };
}
extern "C" fn str_append(p: *mut Str, z: *const i8) -> () {
    let n: Sqlite3Uint64 = unsafe { strlen(z) } as Sqlite3Uint64;
    if unsafe { (*p).n } + n >= unsafe { (*p).n_alloc } {
        let mut z_new: *mut i8 = core::ptr::null_mut();
        let mut n_new: Sqlite3Uint64 = 0 as Sqlite3Uint64;
        if unsafe { (*p).oom_err } != 0 { return; }
        n_new =
            unsafe { (*p).n_alloc } * 2 as Sqlite3Uint64 +
                    100 as Sqlite3Uint64 + n;
        z_new =
            unsafe { sqlite3_realloc64(unsafe { (*p).z } as *mut (), n_new) }
                as *mut i8;
        if z_new == core::ptr::null_mut() {
            unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
            unsafe {
                memset(p as *mut (), 0, core::mem::size_of::<Str>() as u64)
            };
            unsafe { (*p).oom_err = 1 };
            return;
        }
        unsafe { (*p).z = z_new };
        unsafe { (*p).n_alloc = n_new };
    }
    unsafe {
        memcpy(unsafe { unsafe { (*p).z.add(unsafe { (*p).n } as usize) } } as
                *mut (), z as *const (), n as i32 as u64)
    };
    unsafe { (*p).n += n };
    unsafe { *unsafe { (*p).z.add(unsafe { (*p).n } as usize) } = 0 as i8 };
}
extern "C" fn str_str(p: &Str) -> *mut i8 { return (*p).z; }
extern "C" fn str_free(p: *mut Str) -> () {
    unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
    str_init(p);
}
extern "C" fn hex_digit_value(c: i8) -> i32 {
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
extern "C" fn integer_value(mut z_arg_1: *const i8) -> i32 {
    unsafe {
        let mut v: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i: i32 = 0;
        let mut is_neg: i32 = 0;
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '-' as i32 {
            is_neg = 1;
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '+' as i32
            {
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '0' as i32 &&
                unsafe { *z_arg_1.offset(1 as isize) } as i32 == 'x' as i32 {
            let mut x: i32 = 0;
            {
                let __n = 2;
                let __p = &mut z_arg_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            while {
                        x = hex_digit_value(unsafe { *z_arg_1.offset(0 as isize) });
                        x
                    } >= 0 {
                v = (v << 4) + x as Sqlite3Int64;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else {
            while unsafe {
                        isdigit(unsafe { *z_arg_1.offset(0 as isize) } as u8 as i32)
                    } != 0 {
                v =
                    v * 10 as Sqlite3Int64 +
                            unsafe { *z_arg_1.offset(0 as isize) } as Sqlite3Int64 -
                        '0' as i32 as Sqlite3Int64;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        {
            i = 0;
            '__b6: loop {
                if !((i as u64) <
                                core::mem::size_of::<[IntegerValueS0N16integerValueS0; 9]>()
                                        as u64 / 16) {
                    break '__b6;
                }
                '__c6: loop {
                    if unsafe {
                                sqlite3_stricmp(a_mult[i as usize].z_suffix as *const i8,
                                    z_arg_1)
                            } == 0 {
                        v *= a_mult[i as usize].i_mult as Sqlite3Int64;
                        break '__b6;
                    }
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if v > 2147483647 as i64 {
            unsafe {
                fatal_error(c"parameter too large - max 2147483648".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        return if is_neg != 0 { -v } else { v } as i32;
    }
}
extern "C" fn sql_log(p_not_used_1: *mut (), i_err_code_1: i32,
    z_msg_1: *const i8) -> () {
    unsafe {
        unsafe {
            printf(c"LOG: (%d) %s\n".as_ptr() as *mut i8 as *const i8,
                i_err_code_1, z_msg_1)
        };
        unsafe { fflush(__stdoutp) };
    }
}
extern "C" fn progress_handler(p_vdbe_limit_flag_1: *mut ()) -> i32 {
    if unsafe { *(p_vdbe_limit_flag_1 as *mut i32) } != 0 {
        unsafe {
            fatal_error(c"too many VDBE cycles".as_ptr() as *mut i8 as
                    *const i8)
        };
    }
    return 1;
}
extern "C" fn run_sql(db: *mut Sqlite3, mut z_sql_1: *const i8,
    run_flags_1: u32) -> () {
    let mut z_more: *const i8 = core::ptr::null();
    let z_end: *const i8 =
        unsafe { &*z_sql_1.add(unsafe { strlen(z_sql_1) } as usize) };
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    while !(z_sql_1).is_null() && unsafe { *z_sql_1.offset(0 as isize) } != 0
        {
        z_more = core::ptr::null();
        p_stmt = core::ptr::null_mut();
        unsafe {
            sqlite3_prepare_v2(db, z_sql_1, -1, &mut p_stmt, &mut z_more)
        };
        if !(z_more <= z_end) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"runSql".as_ptr() as *const i8,
                    c"dbfuzz.c".as_ptr() as *mut i8 as *const i8, 587,
                    c"zMore<=zEnd".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if z_more == z_sql_1 { break; }
        if run_flags_1 & 1 as u32 != 0 {
            let mut z: *const i8 = z_sql_1;
            let mut n: i32 = 0;
            while z < z_more &&
                    unsafe {
                            isspace(unsafe { *z.offset(0 as isize) } as u8 as i32)
                        } != 0 {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            n = unsafe { z_more.offset_from(z) } as i64 as i32;
            while n > 0 &&
                    unsafe {
                            isspace(unsafe { *z.offset((n - 1) as isize) } as u8 as i32)
                        } != 0 {
                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
            }
            if n == 0 { break; }
            if p_stmt == core::ptr::null_mut() {
                unsafe {
                    printf(c"TRACE: %.*s (error: %s)\n".as_ptr() as *mut i8 as
                            *const i8, n, z, unsafe { sqlite3_errmsg(db) })
                };
            } else {
                unsafe {
                    printf(c"TRACE: %.*s\n".as_ptr() as *mut i8 as *const i8, n,
                        z)
                };
            }
        }
        z_sql_1 = z_more;
        if !(p_stmt).is_null() {
            if run_flags_1 & 2 as u32 == 0 as u32 {
                while 100 == unsafe { sqlite3_step(p_stmt) } {}
            } else {
                let mut n_col: i32 = -1;
                let mut n_row: i32 = 0;
                {
                    n_row = 0;
                    '__b11: loop {
                        if !(100 == unsafe { sqlite3_step(p_stmt) }) {
                            break '__b11;
                        }
                        '__c11: loop {
                            let mut i: i32 = 0;
                            if n_col < 0 {
                                n_col = unsafe { sqlite3_column_count(p_stmt) };
                            }
                            {
                                i = 0;
                                '__b12: loop {
                                    if !(i < n_col) { break '__b12; }
                                    '__c12: loop {
                                        let e_type: i32 = unsafe { sqlite3_column_type(p_stmt, i) };
                                        unsafe {
                                            printf(c"ROW[%d].%s = ".as_ptr() as *mut i8 as *const i8,
                                                n_row, unsafe { sqlite3_column_name(p_stmt, i) })
                                        };
                                        '__s13:
                                            {
                                            match e_type {
                                                5 => {
                                                    {
                                                        unsafe {
                                                            printf(c"NULL\n".as_ptr() as *mut i8 as *const i8)
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"INT %s\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                                    *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                }
                                                1 => {
                                                    {
                                                        unsafe {
                                                            printf(c"INT %s\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                                    *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                }
                                                2 => {
                                                    {
                                                        unsafe {
                                                            printf(c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                                    *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                }
                                                3 => {
                                                    {
                                                        unsafe {
                                                            printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                                unsafe { sqlite3_column_text(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                    {
                                                        unsafe {
                                                            printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                                    *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                }
                                                4 => {
                                                    {
                                                        unsafe {
                                                            printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                                    *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                        };
                                                        break '__s13;
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        break '__c12;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            break '__c11;
                        }
                        { let __p = &mut n_row; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            unsafe { sqlite3_finalize(p_stmt) };
        }
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut i: i32 = 0;
        let mut n_db: i32 = 0;
        let mut az_db: *mut *mut i8 = core::ptr::null_mut();
        let mut verbose_flag: i32 = 0;
        let mut no_lookaside: i32 = 0;
        let mut vdbe_limit_flag: i32 = 0;
        let mut n_heap: i32 = 0;
        let mut i_timeout: i32 = 0;
        let mut rc: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut sql: Str = unsafe { core::mem::zeroed() };
        let mut run_flags: u32 = 0 as u32;
        {
            i = 1;
            '__b14: loop {
                if !(i < argc) { break '__b14; }
                '__c14: loop {
                    let mut z: *mut i8 = unsafe { *argv.offset(i as isize) };
                    if unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 {
                        az_db =
                            unsafe {
                                    realloc(az_db as *mut (),
                                        core::mem::size_of::<*mut i8>() as u64 * (n_db + 1) as u64)
                                } as *mut *mut i8;
                        if az_db == core::ptr::null_mut() {
                            unsafe {
                                fatal_error(c"out of memory".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                        }
                        unsafe {
                            *az_db.offset({
                                                let __p = &mut n_db;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = z
                        };
                        break '__c14;
                    }
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
                                strcmp(z as *const i8,
                                    c"help".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        show_help(unsafe { *argv.offset(0 as isize) } as *const i8);
                    } else if unsafe {
                                strcmp(z as *const i8,
                                    c"limit-mem".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        if i == argc - 1 {
                            unsafe {
                                fatal_error(c"missing argument to %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(i as isize) })
                            };
                        }
                        n_heap =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                    } else if unsafe {
                                strcmp(z as *const i8,
                                    c"no-lookaside".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        no_lookaside = 1;
                    } else if unsafe {
                                strcmp(z as *const i8,
                                    c"timeout".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        if i == argc - 1 {
                            unsafe {
                                fatal_error(c"missing argument to %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(i as isize) })
                            };
                        }
                        i_timeout =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                    } else if unsafe {
                                strcmp(z as *const i8,
                                    c"trace".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        run_flags |= (2 | 1) as u32;
                    } else if unsafe {
                                strcmp(z as *const i8,
                                    c"limit-vdbe".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        vdbe_limit_flag = 1;
                    } else if unsafe {
                                    strcmp(z as *const i8,
                                        c"v".as_ptr() as *mut i8 as *const i8)
                                } == 0 ||
                            unsafe {
                                    strcmp(z as *const i8,
                                        c"verbose".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                        verbose_flag = 1;
                        run_flags |= 1 as u32;
                    } else {
                        unsafe {
                            fatal_error(c"unknown command-line option: \"%s\"\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(i as isize) })
                        };
                    }
                    break '__c14;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_db == 0 {
            show_help(unsafe { *argv.offset(0 as isize) } as *const i8);
        }
        if verbose_flag != 0 {
            unsafe {
                sqlite3_config(16,
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut (), i32, *const i8)
                                    -> ()>(sql_log as *const ())
                    })
            };
        }
        if n_heap > 0 {
            let p_heap: *mut () = unsafe { malloc(n_heap as u64) };
            if p_heap == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"cannot allocate %d-byte heap\n".as_ptr() as
                                *mut i8 as *const i8, n_heap)
                };
            }
            rc = unsafe { sqlite3_config(8, p_heap, n_heap, 32) };
            if rc != 0 {
                unsafe {
                    fatal_error(c"heap configuration failed: %d\n".as_ptr() as
                                *mut i8 as *const i8, rc)
                };
            }
        }
        if no_lookaside != 0 { unsafe { sqlite3_config(13, 0, 0) }; }
        inmem_vfs_register();
        format_vfs();
        str_init(&mut sql);
        {
            i = 0;
            '__b15: loop {
                if !(i < n_db) { break '__b15; }
                '__c15: loop {
                    if verbose_flag != 0 && n_db > 1 {
                        unsafe {
                            printf(c"DATABASE-FILE: %s\n".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *az_db.offset(i as isize) })
                        };
                        unsafe { fflush(__stdoutp) };
                    }
                    if i_timeout != 0 { set_alarm(i_timeout); }
                    create_v_file(c"test.db".as_ptr() as *mut i8 as *const i8,
                        unsafe { *az_db.offset(i as isize) } as *const i8);
                    rc =
                        unsafe {
                            sqlite3_open_v2(c"test.db".as_ptr() as *mut i8 as *const i8,
                                &mut db, 2, c"inmem".as_ptr() as *mut i8 as *const i8)
                        };
                    if rc != 0 {
                        unsafe {
                            printf(c"cannot open test.db for \"%s\"\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *az_db.offset(i as isize) })
                        };
                        reformat_vfs();
                        break '__c15;
                    }
                    if vdbe_limit_flag != 0 {
                        unsafe {
                            sqlite3_progress_handler(db, 100000, Some(progress_handler),
                                &raw mut vdbe_limit_flag as *mut ())
                        };
                    }
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT sql FROM autoexec".as_ptr() as *mut i8 as
                                    *const i8, -1, &mut p_stmt, core::ptr::null_mut())
                        };
                    if rc == 0 {
                        while 100 == unsafe { sqlite3_step(p_stmt) } {
                            str_append(&mut sql,
                                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8);
                            str_append(&mut sql,
                                c"\n".as_ptr() as *mut i8 as *const i8);
                        }
                    }
                    unsafe { sqlite3_finalize(p_stmt) };
                    str_append(&mut sql,
                        c"PRAGMA integrity_check;\n".as_ptr() as *mut i8 as
                            *const i8);
                    run_sql(db, str_str(&sql) as *const i8, run_flags);
                    unsafe { sqlite3_close(db) };
                    reformat_vfs();
                    str_free(&mut sql);
                    if unsafe { sqlite3_memory_used() } > 0 as i64 {
                        unsafe { free(az_db as *mut ()) };
                        reformat_vfs();
                        unsafe {
                            fatal_error(c"memory leak of %lld bytes".as_ptr() as *mut i8
                                    as *const i8, unsafe { sqlite3_memory_used() })
                        };
                    }
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        str_free(&mut sql);
        reformat_vfs();
        return Ok(());
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IntegerValueS0N16integerValueS0 {
    z_suffix: *mut i8,
    i_mult: i32,
}
static mut inmem_vfs: Sqlite3Vfs = unsafe { core::mem::zeroed() };
static mut a_mult: [IntegerValueS0N16integerValueS0; 9] =
    [IntegerValueS0N16integerValueS0 {
                z_suffix: c"KiB".as_ptr() as *mut i8,
                i_mult: 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"MiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"GiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024 * 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"KB".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"MB".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"GB".as_ptr() as *mut i8,
                i_mult: 1000000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"K".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"M".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"G".as_ptr() as *mut i8,
                i_mult: 1000000000,
            }];
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
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
    fn printf(_: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn free(_: *mut ())
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn rewind(_: *mut FILE)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn malloc(__size: u64)
    -> *mut ();
    fn fclose(_: *mut FILE)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn realloc(__ptr: *mut (), __size: u64)
    -> *mut ();
    fn isdigit(_c: i32)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn isspace(_c: i32)
    -> i32;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;