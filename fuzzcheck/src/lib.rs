#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3recover_h;
pub(crate) use crate::sqlite3recover_h::*;
type DarwinSizeT = u64;
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
struct Blob {
    p_next: *mut Blob,
    id: i32,
    seq: i32,
    sz: i32,
    a: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct GlobalVars {
    z_argv0: *const i8,
    z_db_file: *const i8,
    a_file: [VFile; 10],
    n_db: i32,
    p_first_db: *mut Blob,
    n_sql: i32,
    p_first_sql: *mut Blob,
    u_random: u32,
    n_invariant: u32,
    z_test_name: [i8; 100],
}
static mut g: GlobalVars = unsafe { core::mem::zeroed() };
unsafe extern "C" fn fatal_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe {
            fprintf(__stderrp, c"%s".as_ptr() as *mut i8 as *const i8,
                g.z_argv0)
        };
        if !(g.z_db_file).is_null() {
            unsafe {
                fprintf(__stderrp, c" %s".as_ptr() as *mut i8 as *const i8,
                    g.z_db_file)
            };
        }
        if g.z_test_name[0 as usize] != 0 {
            unsafe {
                fprintf(__stderrp, c" (%s)".as_ptr() as *mut i8 as *const i8,
                    &raw mut g.z_test_name[0 as usize] as *mut i8)
            };
        }
        eprint!(": ");
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_format_1, ap) };
        ();
        eprintln!("");
        unsafe { exit(1) };
    }
}
extern "C" fn set_alarm(n_1: i32) -> () { { let _ = n_1; }; }
extern "C" fn progress_handler_1(p_vdbe_limit_flag_1: *mut ()) -> i32 {
    if unsafe { *(p_vdbe_limit_flag_1 as *mut i32) } != 0 {
        unsafe {
            fatal_error(c"too many VDBE cycles".as_ptr() as *mut i8 as
                    *const i8)
        };
    }
    return 1;
}
extern "C" fn safe_realloc(p_old_1: *mut (), sz_new_1: i32) -> *mut () {
    let p_new: *mut () =
        unsafe {
            realloc(p_old_1, if sz_new_1 <= 0 { 1 } else { sz_new_1 } as u64)
        };
    if p_new == core::ptr::null_mut() {
        unsafe {
            fatal_error(c"unable to realloc for %d bytes".as_ptr() as *mut i8
                    as *const i8, sz_new_1)
        };
    }
    return p_new;
}
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
extern "C" fn create_v_file(z_name_1: *const i8, sz: i32, p_data_1: *const u8)
    -> *mut VFile {
    unsafe {
        let mut p_new: *mut VFile = find_v_file(z_name_1);
        let mut i: i32 = 0;
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
        p_new = &mut g.a_file[i as usize];
        if !(z_name_1).is_null() {
            let n_name: i32 = unsafe { strlen(z_name_1) } as i32 + 1;
            unsafe {
                (*p_new).z_filename =
                    safe_realloc(core::ptr::null_mut(), n_name) as *mut i8
            };
            unsafe {
                memcpy(unsafe { (*p_new).z_filename } as *mut (),
                    z_name_1 as *const (), n_name as u64)
            };
        } else { unsafe { (*p_new).z_filename = core::ptr::null_mut() }; }
        unsafe { (*p_new).n_ref = 0 };
        unsafe { (*p_new).sz = sz };
        unsafe {
            (*p_new).a = safe_realloc(core::ptr::null_mut(), sz) as *mut u8
        };
        if sz > 0 {
            unsafe {
                memcpy(unsafe { (*p_new).a } as *mut (),
                    p_data_1 as *const (), sz as u64)
            };
        }
        return p_new;
    }
}
extern "C" fn all_zero(a_line_1: *const u8) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b4: loop {
            if !(i < 16 &&
                            unsafe { *a_line_1.offset(i as isize) } as i32 == 0) {
                break '__b4;
            }
            '__c4: loop { break '__c4; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return (i == 16) as i32;
}
extern "C" fn render_db_sql_for_cli(out: *mut FILE, z_file_1: *const i8,
    a_db_1: *mut u8, n_db_1: i32, z_sql_1: *mut u8, n_sql_1: i32) -> () {
    unsafe {
        unsafe {
            fprintf(out,
                c".print ******* %s *******\n".as_ptr() as *mut i8 as
                    *const i8, z_file_1)
        };
        if n_db_1 > 100 {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut pgsz: i32 = 0;
            let mut last_page: i32 = 0;
            let mut i_page: i32 = 0;
            let mut a_line: *mut u8 = core::ptr::null_mut();
            let mut buf: [u8; 16] = [0; 16];
            let mut b_show: [u8; 256] = [0; 256];
            unsafe {
                memset(&raw mut b_show[0 as usize] as *mut u8 as *mut (),
                    '.' as i32, core::mem::size_of::<[u8; 256]>() as u64)
            };
            {
                i = ' ' as i32;
                '__b5: loop {
                    if !(i <= '~' as i32) { break '__b5; }
                    '__c5: loop {
                        if i != '{' as i32 && i != '}' as i32 && i != '\"' as i32 &&
                                i != '\\' as i32 {
                            b_show[i as usize] = i as u8;
                        }
                        break '__c5;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            pgsz =
                (unsafe { *a_db_1.offset(16 as isize) } as i32) << 8 |
                    unsafe { *a_db_1.offset(17 as isize) } as i32;
            if pgsz == 0 { pgsz = 65536; }
            if pgsz < 512 || pgsz & pgsz - 1 != 0 { pgsz = 4096; }
            unsafe {
                fprintf(out,
                    c".open --hexdb\n".as_ptr() as *mut i8 as *const i8)
            };
            unsafe {
                fprintf(out,
                    c"| size %d pagesize %d filename %s\n".as_ptr() as *mut i8
                        as *const i8, n_db_1, pgsz, z_file_1)
            };
            {
                i = 0;
                '__b6: loop {
                    if !(i < n_db_1) { break '__b6; }
                    '__c6: loop {
                        if i + 16 > n_db_1 {
                            unsafe {
                                memset(&raw mut buf[0 as usize] as *mut u8 as *mut (), 0,
                                    core::mem::size_of::<[u8; 16]>() as u64)
                            };
                            unsafe {
                                memcpy(&raw mut buf[0 as usize] as *mut u8 as *mut (),
                                    unsafe { a_db_1.offset(i as isize) } as *const (),
                                    (n_db_1 - i) as u64)
                            };
                            a_line = &raw mut buf[0 as usize] as *mut u8;
                        } else { a_line = unsafe { a_db_1.offset(i as isize) }; }
                        if all_zero(a_line as *const u8) != 0 { break '__c6; }
                        i_page = i / pgsz + 1;
                        if last_page != i_page {
                            unsafe {
                                fprintf(out,
                                    c"| page %d offset %d\n".as_ptr() as *mut i8 as *const i8,
                                    i_page, (i_page - 1) * pgsz)
                            };
                            last_page = i_page;
                        }
                        unsafe {
                            fprintf(out, c"|  %5d:".as_ptr() as *mut i8 as *const i8,
                                i - (i_page - 1) * pgsz)
                        };
                        {
                            j = 0;
                            '__b7: loop {
                                if !(j < 16) { break '__b7; }
                                '__c7: loop {
                                    unsafe {
                                        fprintf(out, c" %02x".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *a_line.offset(j as isize) } as i32)
                                    };
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"   ".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            j = 0;
                            '__b8: loop {
                                if !(j < 16) { break '__b8; }
                                '__c8: loop {
                                    let c: u8 = unsafe { *a_line.offset(j as isize) } as u8;
                                    unsafe { fputc(b_show[c as usize] as i32, __stdoutp) };
                                    break '__c8;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { fputc('\n' as i32, __stdoutp) };
                        break '__c6;
                    }
                    i += 16;
                }
            }
            unsafe {
                fprintf(out, c"| end %s\n".as_ptr() as *mut i8 as *const i8,
                    z_file_1)
            };
        } else {
            unsafe {
                fprintf(out,
                    c".open :memory:\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            fprintf(out,
                c".testctrl prng_seed 1 db\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            fprintf(out,
                c".testctrl internal_functions\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            fprintf(out, c"%.*s".as_ptr() as *mut i8 as *const i8, n_sql_1,
                z_sql_1)
        };
        if n_sql_1 > 0 &&
                unsafe { *z_sql_1.offset((n_sql_1 - 1) as isize) } as i32 !=
                    '\n' as i32 {
            unsafe { fprintf(out, c"\n".as_ptr() as *mut i8 as *const i8) };
        }
    }
}
extern "C" fn path_tail(mut z_path_1: *const i8) -> *const i8 {
    let mut z_tail: *const i8 = z_path_1;
    while unsafe { *z_path_1.offset(0 as isize) } != 0 {
        if unsafe { *z_path_1.offset(0 as isize) } as i32 == '/' as i32 &&
                unsafe { *z_path_1.offset(1 as isize) } as i32 != 0 {
            z_tail = unsafe { z_path_1.offset(1 as isize) };
        }
        if unsafe { *z_path_1.offset(0 as isize) } as i32 == '\\' as i32 &&
                unsafe { *z_path_1.offset(1 as isize) } as i32 != 0 {
            z_tail = unsafe { z_path_1.offset(1 as isize) };
        }
        {
            let __p = &mut z_path_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return z_tail;
}
extern "C" fn read_file(z_filename_1: *const i8, sz: &mut i64) -> *mut i8 {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut n_in: i64 = 0 as i64;
    let mut p_buf: *mut u8 = core::ptr::null_mut();
    *sz = 0 as i64;
    if z_filename_1 == core::ptr::null() { return core::ptr::null_mut(); }
    in_ =
        unsafe {
            fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
        };
    if in_ == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe { fseek(in_, 0 as i64, 2) };
    *sz = { n_in = unsafe { ftell(in_) }; n_in };
    unsafe { rewind(in_) };
    p_buf =
        unsafe { sqlite3_malloc64((n_in + 1 as i64) as Sqlite3Uint64) } as
            *mut u8;
    if !(p_buf).is_null() &&
            1 as u64 ==
                unsafe { fread(p_buf as *mut (), n_in as u64, 1 as u64, in_) }
        {
        unsafe { *p_buf.offset(n_in as isize) = 0 as u8 };
        unsafe { fclose(in_) };
        return p_buf as *mut i8;
    }
    unsafe { sqlite3_free(p_buf as *mut ()) };
    *sz = 0 as i64;
    unsafe { fclose(in_) };
    return core::ptr::null_mut();
}
extern "C" fn readfile_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut n_in: i64 = 0 as i64;
    let mut p_buf: *mut () = core::ptr::null_mut();
    let z_name: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_name == core::ptr::null() { return; }
    p_buf = read_file(z_name, &mut n_in) as *mut ();
    if !(p_buf).is_null() {
        unsafe {
            sqlite3_result_blob(context, p_buf as *const (), n_in as i32,
                Some(sqlite3_free))
        };
    }
}
extern "C" fn readtextfile_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_name: *const i8 = core::ptr::null();
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut n_in: i64 = 0 as i64;
    let mut p_buf: *mut i8 = core::ptr::null_mut();
    z_name =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_name == core::ptr::null() { return; }
    in_ = unsafe { fopen(z_name, c"rb".as_ptr() as *mut i8 as *const i8) };
    if in_ == core::ptr::null_mut() { return; }
    unsafe { fseek(in_, 0 as i64, 2) };
    n_in = unsafe { ftell(in_) };
    unsafe { rewind(in_) };
    p_buf =
        unsafe { sqlite3_malloc64((n_in + 1 as i64) as Sqlite3Uint64) } as
            *mut i8;
    if !(p_buf).is_null() &&
            1 as u64 ==
                unsafe { fread(p_buf as *mut (), n_in as u64, 1 as u64, in_) }
        {
        unsafe { *p_buf.offset(n_in as isize) = 0 as i8 };
        unsafe {
            sqlite3_result_text(context, p_buf as *const i8, -1,
                Some(sqlite3_free))
        };
    } else { unsafe { sqlite3_free(p_buf as *mut ()) }; }
    unsafe { fclose(in_) };
}
extern "C" fn writefile_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut out: *mut FILE = core::ptr::null_mut();
    let mut z: *const i8 = core::ptr::null();
    let mut rc: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut z_file: *const i8 = core::ptr::null();
    { let _ = argc; };
    z_file =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_file == core::ptr::null() { return; }
    out = unsafe { fopen(z_file, c"wb".as_ptr() as *mut i8 as *const i8) };
    if out == core::ptr::null_mut() { return; }
    z =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(1 as isize) }) } as
            *const i8;
    if z == core::ptr::null() {
        rc = 0 as Sqlite3Int64;
    } else {
        rc =
            unsafe {
                    fwrite(z as *const (), 1 as u64,
                        unsafe {
                                sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) })
                            } as u64, out)
                } as Sqlite3Int64;
    }
    unsafe { fclose(out) };
    unsafe { sqlite3_result_int64(context, rc) };
}
extern "C" fn blob_list_load_from_db(db: *mut Sqlite3, z_sql_1: *const i8,
    first_id_1: i32, last_id_1: i32, p_n_1: &mut i32,
    pp_list_1: &mut *mut Blob) -> () {
    unsafe {
        let mut head: *mut Blob = core::ptr::null_mut();
        let mut p: *mut Blob = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut n: i32 = 0;
        let mut rc: i32 = 0;
        let mut z2: *mut i8 = core::ptr::null_mut();
        let mut u_blob: BlobListLoadFromDbU0N25blobListLoadFromDbU0 =
            unsafe { core::mem::zeroed() };
        head = &mut u_blob.s_blob;
        if first_id_1 > 0 {
            z2 =
                unsafe {
                    sqlite3_mprintf(c"%s WHERE rowid BETWEEN %d AND %d".as_ptr()
                                as *mut i8 as *const i8, z_sql_1, first_id_1, last_id_1)
                };
        } else {
            z2 =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_sql_1)
                };
        }
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z2 as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z2 as *mut ()) };
        if rc != 0 {
            unsafe {
                fatal_error(c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(db) })
            };
        }
        unsafe { (*head).p_next = core::ptr::null_mut() };
        p = head;
        while 100 == unsafe { sqlite3_step(p_stmt) } {
            let sz: i32 = unsafe { sqlite3_column_bytes(p_stmt, 1) };
            let p_new: *mut Blob =
                safe_realloc(core::ptr::null_mut(),
                        (core::mem::offset_of!(Blob, a) as u64 +
                                (sz + 1 + 7 & !7) as u64) as i32) as *mut Blob;
            unsafe { (*p_new).id = unsafe { sqlite3_column_int(p_stmt, 0) } };
            unsafe { (*p_new).sz = sz };
            unsafe {
                (*p_new).seq =
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t }
            };
            unsafe { (*p_new).p_next = core::ptr::null_mut() };
            unsafe {
                memcpy(unsafe { (*p_new).a.as_ptr() } as *mut u8 as *mut (),
                    unsafe { sqlite3_column_blob(p_stmt, 1) }, sz as u64)
            };
            unsafe {
                *(unsafe { (*p_new).a.as_ptr() } as
                                *mut u8).offset(sz as isize) = 0 as u8
            };
            unsafe { (*p).p_next = p_new };
            p = p_new;
        }
        unsafe { sqlite3_finalize(p_stmt) };
        *p_n_1 = n;
        *pp_list_1 = unsafe { (*head).p_next };
    }
}
extern "C" fn blob_list_free(mut p: *mut Blob) -> () {
    let mut p_next: *mut Blob = core::ptr::null_mut();
    while !(p).is_null() {
        p_next = unsafe { (*p).p_next };
        unsafe { free(p as *mut ()) };
        p = p_next;
    }
}
extern "C" fn time_of_day() -> Sqlite3Int64 {
    unsafe {
        let mut t: Sqlite3Int64 = 0 as Sqlite3Int64;
        if clock_vfs == core::ptr::null_mut() {
            clock_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
            if clock_vfs == core::ptr::null_mut() {
                return 0 as Sqlite3Int64;
            }
        }
        if unsafe { (*clock_vfs).i_version } >= 1 &&
                unsafe { (*clock_vfs).x_current_time_int64.is_some() } {
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time_int64.unwrap()
                    })(clock_vfs, &mut t)
            };
        } else {
            let mut r: f64 = 0.0;
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time.unwrap()
                    })(clock_vfs, &mut r)
            };
            t = (r * 86400000.0) as Sqlite3Int64;
        }
        return t;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzCtx {
    db: *mut Sqlite3,
    i_cutoff_time: Sqlite3Int64,
    i_last_cb: Sqlite3Int64,
    mx_interval: Sqlite3Int64,
    n_cb: u32,
    mx_cb: u32,
    exec_cnt: u32,
    timeout_hit: i32,
}
static mut e_verbosity: i32 = 0;
static mut b_vdbe_debug: i32 = 0;
static mut gi_timeout: i32 = 10000;
static mut mx_progress_cb: u32 = 2000 as u32;
static mut length_limit: i32 = 1000000;
static mut depth_limit: i32 = 500;
static mut heap_limit: Sqlite3Int64 = 100000000 as Sqlite3Int64;
static mut vdbe_op_limit: i32 = 25000;
static mut max_db_size: Sqlite3Int64 = 104857600 as Sqlite3Int64;
static mut oom_counter: u32 = 0 as u32;
static mut oom_repeat: u32 = 0 as u32;
static mut default_malloc: Option<unsafe extern "C" fn(i32) -> *mut ()> =
    None;
static mut b_no_recover: i32 = 0;
#[unsafe(no_mangle)]
pub extern "C" fn oom_fault() -> () {
    unsafe {
        if e_verbosity != 0 {
            unsafe {
                printf(c"Simulated OOM fault\n".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        if oom_repeat > 0 as u32 {
            { let __p = &mut oom_repeat; let __t = *__p; *__p -= 1; __t };
        } else {
            { let __p = &mut oom_counter; let __t = *__p; *__p -= 1; __t };
        }
    }
}
extern "C" fn oom_malloc(n_byte_1: i32) -> *mut () {
    unsafe {
        if oom_counter != 0 {
            if oom_counter == 1 as u32 {
                oom_fault();
                return core::ptr::null_mut();
            } else {
                {
                    let __p = &mut oom_counter;
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
        }
        return unsafe { default_malloc.unwrap()(n_byte_1) };
    }
}
extern "C" fn register_oom_simulator() -> () {
    unsafe {
        let mut mem: Sqlite3MemMethods = unsafe { core::mem::zeroed() };
        unsafe { sqlite3_shutdown() };
        unsafe { sqlite3_config(5, &raw mut mem as *mut Sqlite3MemMethods) };
        default_malloc = mem.x_malloc;
        mem.x_malloc = Some(oom_malloc);
        unsafe { sqlite3_config(4, &raw mut mem as *mut Sqlite3MemMethods) };
    }
}
extern "C" fn disable_oom() -> () {
    unsafe { oom_counter = 0 as u32; oom_repeat = 0 as u32; }
}
extern "C" fn hex_to_int(mut h: u32) -> u8 {
    h += 9 as u32 * (1 as u32 & h >> 6);
    return (h & 15 as u32) as u8;
}
extern "C" fn is_offset(z_in_1: &[u8], p_k_1: &mut u32, p_i_1: *mut u32)
    -> i32 {
    let mut i: i32 = 0;
    let mut k: u32 = 0 as u32;
    let mut c: u8 = 0 as u8;
    {
        i = 1;
        '__b12: loop {
            if !(i < z_in_1.len() as i32 &&
                            { c = z_in_1[i as usize] as u8; c } as i32 != ']' as i32) {
                break '__b12;
            }
            '__c12: loop {
                if (unsafe { isxdigit(c as i32) } == 0) as i32 != 0 {
                    return 0;
                }
                k = k * 16 as u32 + hex_to_int(c as u32) as u32;
                break '__c12;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i == z_in_1.len() as i32 { return 0; }
    *p_k_1 = 2 as u32 * k;
    unsafe { *p_i_1 += i as u32 };
    return 1;
}
extern "C" fn decode_database(z_in_1: *const u8, n_in_1: i32,
    pa_decode_1: &mut *mut u8, pn_decode_1: &mut i32) -> i32 {
    unsafe {
        let mut a: *mut u8 = core::ptr::null_mut();
        let mut a_new: *mut u8 = core::ptr::null_mut();
        let mut mx: i32 = 0;
        let mut n_alloc: Sqlite3Uint64 = 4096 as Sqlite3Uint64;
        let mut i: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        let mut k: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut b: u8 = 0 as u8;
        if n_in_1 < 4 { return -1; }
        n = n_in_1 as u32;
        a = unsafe { sqlite3_malloc64(n_alloc) } as *mut u8;
        if a == core::ptr::null_mut() {
            eprintln!("Out of memory!");
            unsafe { exit(1) };
        }
        unsafe { memset(a as *mut (), 0, n_alloc as u64) };
        {
            i = { k = 0 as u32; k };
            '__b13: loop {
                if !(i < n) { break '__b13; }
                '__c13: loop {
                    let c: u8 = unsafe { *z_in_1.add(i as usize) } as u8;
                    if unsafe { isxdigit(c as i32) } != 0 {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        if k & 1 as u32 != 0 {
                            b = (hex_to_int(c as u32) as i32 * 16) as u8;
                        } else {
                            b += hex_to_int(c as u32) as i32 as u8;
                            j = k / 2 as u32 - 1 as u32;
                            if j as Sqlite3Uint64 >= n_alloc {
                                let mut new_size: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                                if n_alloc == 10000000 as u64 || j >= 10000000 as u32 {
                                    if e_verbosity != 0 {
                                        eprintln!("Input database too big: max {} bytes", 10000000 as i32);
                                    }
                                    unsafe { sqlite3_free(a as *mut ()) };
                                    return -1;
                                }
                                new_size = n_alloc * 2 as Sqlite3Uint64;
                                if new_size <= j as u64 {
                                    new_size =
                                        (j + 4096 as u32 & !4095 as u32) as Sqlite3Uint64;
                                }
                                if new_size > 10000000 as u64 {
                                    if j >= 10000000 as u32 {
                                        unsafe { sqlite3_free(a as *mut ()) };
                                        return -1;
                                    }
                                    new_size = 10000000 as Sqlite3Uint64;
                                }
                                a_new =
                                    unsafe { sqlite3_realloc64(a as *mut (), new_size) } as
                                        *mut u8;
                                if a_new == core::ptr::null_mut() {
                                    unsafe { sqlite3_free(a as *mut ()) };
                                    return -1;
                                }
                                a = a_new;
                                if !(new_size > n_alloc) as i32 as i64 != 0 {
                                    unsafe {
                                        __assert_rtn(c"decodeDatabase".as_ptr() as *const i8,
                                            c"fuzzcheck.c".as_ptr() as *mut i8 as *const i8, 831,
                                            c"newSize > nAlloc".as_ptr() as *mut i8 as *const i8)
                                    }
                                } else { { let _ = 0; } };
                                unsafe {
                                    memset(unsafe { a.add(n_alloc as usize) } as *mut (), 0,
                                        (new_size - n_alloc) as u64)
                                };
                                n_alloc = new_size;
                            }
                            if j >= mx as u32 {
                                mx = (j + 4095 as u32 & !4095 as u32) as i32;
                                if mx > 10000000 { mx = 10000000; }
                            }
                            if !((j as Sqlite3Uint64) < n_alloc) as i32 as i64 != 0 {
                                unsafe {
                                    __assert_rtn(c"decodeDatabase".as_ptr() as *const i8,
                                        c"fuzzcheck.c".as_ptr() as *mut i8 as *const i8, 839,
                                        c"j<nAlloc".as_ptr() as *mut i8 as *const i8)
                                }
                            } else { { let _ = 0; } };
                            unsafe { *a.add(j as usize) = b };
                        }
                    } else if unsafe { *z_in_1.add(i as usize) } as i32 ==
                                    '[' as i32 && i < n - 3 as u32 &&
                            is_offset(unsafe {
                                        let __p = unsafe { z_in_1.add(i as usize) } as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else {
                                            core::slice::from_raw_parts(__p,
                                                (n_in_1 as u32 - i) as usize)
                                        }
                                    }, &mut k, &mut i) != 0 {
                        break '__c13;
                    } else if unsafe { *z_in_1.add(i as usize) } as i32 ==
                                    '\n' as i32 && i < n - 4 as u32 &&
                            unsafe {
                                    memcmp(unsafe { z_in_1.add(i as usize) } as *const (),
                                        c"\n--\n".as_ptr() as *mut i8 as *const (), 4 as u64)
                                } == 0 {
                        i += 4 as u32;
                        break '__b13;
                    }
                    break '__c13;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        *pn_decode_1 = mx;
        *pa_decode_1 = a;
        return i as i32;
    }
}
extern "C" fn progress_handler(p_client_data_1: *mut ()) -> i32 {
    unsafe {
        let p: *mut FuzzCtx = p_client_data_1 as *mut FuzzCtx;
        let i_now: Sqlite3Int64 = time_of_day();
        let mut rc: i32 = (i_now >= unsafe { (*p).i_cutoff_time }) as i32;
        let i_diff: Sqlite3Int64 = i_now - unsafe { (*p).i_last_cb };
        if i_diff > unsafe { (*p).mx_interval } {
            unsafe { (*p).mx_interval = i_diff };
        }
        {
            let __p = unsafe { &mut (*p).n_cb };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if rc == 0 && unsafe { (*p).mx_cb } > 0 as u32 &&
                unsafe { (*p).mx_cb } <= unsafe { (*p).n_cb } {
            rc = 1;
        }
        if rc != 0 && (unsafe { (*p).timeout_hit } == 0) as i32 != 0 &&
                e_verbosity >= 2 {
            unsafe {
                printf(c"Timeout on progress callback %d\n".as_ptr() as
                            *mut i8 as *const i8, unsafe { (*p).n_cb })
            };
            unsafe { fflush(__stdoutp) };
            unsafe { (*p).timeout_hit = 1 };
        }
        return rc;
    }
}
extern "C" fn block_troublesome_sql(p_client_data_1: *mut (), e_code_1: i32,
    z_arg1_1: *const i8, z_arg2_1: *const i8, z_arg3_1: *const i8,
    z_arg4_1: *const i8) -> i32 {
    unsafe {
        let p_bts_flags: *mut u32 = p_client_data_1 as *mut u32;
        { let _ = z_arg3_1; };
        { let _ = z_arg4_1; };
        '__s14:
            {
            match e_code_1 {
                19 => {
                    {
                        if unsafe {
                                        sqlite3_stricmp(c"busy_timeout".as_ptr() as *mut i8 as
                                                *const i8, z_arg1_1)
                                    } == 0 &&
                                (z_arg2_1 == core::ptr::null() ||
                                        unsafe { strtoll(z_arg2_1, core::ptr::null_mut(), 0) } >
                                            100 as i64 ||
                                    unsafe { strtoll(z_arg2_1, core::ptr::null_mut(), 10) } >
                                        100 as i64) {
                            return 1;
                        } else if unsafe {
                                        sqlite3_stricmp(c"hard_heap_limit".as_ptr() as *mut i8 as
                                                *const i8, z_arg1_1)
                                    } == 0 ||
                                unsafe {
                                        sqlite3_stricmp(c"reverse_unordered_selects".as_ptr() as
                                                    *mut i8 as *const i8, z_arg1_1)
                                    } == 0 {
                            unsafe { *p_bts_flags |= 8 as u32 };
                        } else if e_verbosity == 0 {
                            if unsafe {
                                                sqlite3_strnicmp(c"vdbe_".as_ptr() as *mut i8 as *const i8,
                                                    z_arg1_1, 5)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(c"parser_trace".as_ptr() as *mut i8 as
                                                        *const i8, z_arg1_1)
                                            } == 0 ||
                                    unsafe {
                                            sqlite3_stricmp(c"temp_store_directory".as_ptr() as *mut i8
                                                    as *const i8, z_arg1_1)
                                        } == 0 {
                                return 1;
                            }
                        } else if unsafe {
                                            sqlite3_stricmp(c"oom".as_ptr() as *mut i8 as *const i8,
                                                z_arg1_1)
                                        } == 0 && z_arg2_1 != core::ptr::null() &&
                                unsafe { *z_arg2_1.offset(0 as isize) } as i32 != 0 {
                            oom_counter = unsafe { atoi(z_arg2_1) } as u32;
                        }
                        unsafe { *p_bts_flags |= 2 as u32 };
                        break '__s14;
                    }
                    {
                        unsafe { *p_bts_flags |= 2 as u32 };
                        if z_arg1_1 == core::ptr::null() { return 1; }
                        if unsafe {
                                    strcmp(z_arg1_1,
                                        c":memory:".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            return 0;
                        }
                        if unsafe {
                                        sqlite3_strglob(c"file:*[?]vfs=memdb".as_ptr() as *mut i8 as
                                                *const i8, z_arg1_1)
                                    } == 0 &&
                                unsafe {
                                        sqlite3_strglob(c"file:*[^/a-zA-Z0-9_.]*[?]vfs=memdb".as_ptr()
                                                    as *mut i8 as *const i8, z_arg1_1)
                                    } != 0 {
                            return 0;
                        }
                        return 1;
                    }
                    { unsafe { *p_bts_flags |= 1 as u32 }; break '__s14; }
                    {
                        let mut first: i32 = 0;
                        let mut last: i32 = 0;
                        first = 0;
                        last =
                            (core::mem::size_of::<[*const i8; 35]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64 - 1 as u64) as i32;
                        '__b15: loop {
                            '__c15: loop {
                                let mid: i32 = (first + last) / 2;
                                let c: i32 =
                                    unsafe {
                                        sqlite3_stricmp(az_bad_funcs[mid as usize], z_arg2_1)
                                    };
                                if c < 0 {
                                    first = mid + 1;
                                } else if c > 0 {
                                    last = mid - 1;
                                } else {
                                    unsafe { *p_bts_flags |= 4 as u32 };
                                    break '__b15;
                                }
                                break '__c15;
                            }
                            if !(first <= last) { break '__b15; }
                        }
                        break '__s14;
                    }
                    { break '__s14; }
                    { unsafe { *p_bts_flags |= 2 as u32 }; }
                }
                24 => {
                    {
                        unsafe { *p_bts_flags |= 2 as u32 };
                        if z_arg1_1 == core::ptr::null() { return 1; }
                        if unsafe {
                                    strcmp(z_arg1_1,
                                        c":memory:".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            return 0;
                        }
                        if unsafe {
                                        sqlite3_strglob(c"file:*[?]vfs=memdb".as_ptr() as *mut i8 as
                                                *const i8, z_arg1_1)
                                    } == 0 &&
                                unsafe {
                                        sqlite3_strglob(c"file:*[^/a-zA-Z0-9_.]*[?]vfs=memdb".as_ptr()
                                                    as *mut i8 as *const i8, z_arg1_1)
                                    } != 0 {
                            return 0;
                        }
                        return 1;
                    }
                    { unsafe { *p_bts_flags |= 1 as u32 }; break '__s14; }
                    {
                        let mut first: i32 = 0;
                        let mut last: i32 = 0;
                        first = 0;
                        last =
                            (core::mem::size_of::<[*const i8; 35]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64 - 1 as u64) as i32;
                        '__b15: loop {
                            '__c15: loop {
                                let mid: i32 = (first + last) / 2;
                                let c: i32 =
                                    unsafe {
                                        sqlite3_stricmp(az_bad_funcs[mid as usize], z_arg2_1)
                                    };
                                if c < 0 {
                                    first = mid + 1;
                                } else if c > 0 {
                                    last = mid - 1;
                                } else {
                                    unsafe { *p_bts_flags |= 4 as u32 };
                                    break '__b15;
                                }
                                break '__c15;
                            }
                            if !(first <= last) { break '__b15; }
                        }
                        break '__s14;
                    }
                    { break '__s14; }
                    { unsafe { *p_bts_flags |= 2 as u32 }; }
                }
                21 => {
                    { unsafe { *p_bts_flags |= 1 as u32 }; break '__s14; }
                    {
                        let mut first: i32 = 0;
                        let mut last: i32 = 0;
                        first = 0;
                        last =
                            (core::mem::size_of::<[*const i8; 35]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64 - 1 as u64) as i32;
                        '__b15: loop {
                            '__c15: loop {
                                let mid: i32 = (first + last) / 2;
                                let c: i32 =
                                    unsafe {
                                        sqlite3_stricmp(az_bad_funcs[mid as usize], z_arg2_1)
                                    };
                                if c < 0 {
                                    first = mid + 1;
                                } else if c > 0 {
                                    last = mid - 1;
                                } else {
                                    unsafe { *p_bts_flags |= 4 as u32 };
                                    break '__b15;
                                }
                                break '__c15;
                            }
                            if !(first <= last) { break '__b15; }
                        }
                        break '__s14;
                    }
                    { break '__s14; }
                    { unsafe { *p_bts_flags |= 2 as u32 }; }
                }
                31 => {
                    {
                        let mut first: i32 = 0;
                        let mut last: i32 = 0;
                        first = 0;
                        last =
                            (core::mem::size_of::<[*const i8; 35]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64 - 1 as u64) as i32;
                        '__b15: loop {
                            '__c15: loop {
                                let mid: i32 = (first + last) / 2;
                                let c: i32 =
                                    unsafe {
                                        sqlite3_stricmp(az_bad_funcs[mid as usize], z_arg2_1)
                                    };
                                if c < 0 {
                                    first = mid + 1;
                                } else if c > 0 {
                                    last = mid - 1;
                                } else {
                                    unsafe { *p_bts_flags |= 4 as u32 };
                                    break '__b15;
                                }
                                break '__c15;
                            }
                            if !(first <= last) { break '__b15; }
                        }
                        break '__s14;
                    }
                    { break '__s14; }
                    { unsafe { *p_bts_flags |= 2 as u32 }; }
                }
                20 => {
                    { break '__s14; }
                    { unsafe { *p_bts_flags |= 2 as u32 }; }
                }
                _ => { { unsafe { *p_bts_flags |= 2 as u32 }; } }
            }
        }
        return 0;
    }
}
extern "C" fn recover_sql_cb(p_ctx_1: *mut (), z_sql_1: *const i8) -> i32 {
    unsafe {
        if e_verbosity >= 2 && !(z_sql_1).is_null() {
            unsafe {
                printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_sql_1)
            };
        }
        return 0;
    }
}
extern "C" fn recover_database(db: *mut Sqlite3) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let z_recovery_db: *const i8 = c"".as_ptr() as *mut i8 as *const i8;
        let z_laf: *const i8 =
            c"lost_and_found".as_ptr() as *mut i8 as *const i8;
        let mut b_freelist: i32 = 1;
        let mut b_rowids: i32 = 1;
        let mut p: *mut Sqlite3Recover = core::ptr::null_mut();
        p =
            unsafe {
                sqlite3_recover_init_sql(db,
                    c"main".as_ptr() as *mut i8 as *const i8,
                    Some(recover_sql_cb), core::ptr::null_mut())
            };
        unsafe { sqlite3_recover_config(p, 789, z_recovery_db as *mut ()) };
        unsafe { sqlite3_recover_config(p, 1, z_laf as *mut ()) };
        unsafe { sqlite3_recover_config(p, 3, &raw mut b_rowids as *mut ()) };
        unsafe {
            sqlite3_recover_config(p, 2, &raw mut b_freelist as *mut ())
        };
        unsafe { sqlite3_recover_run(p) };
        if unsafe { sqlite3_recover_errcode(p) } != 0 {
            let z_err: *const i8 = unsafe { sqlite3_recover_errmsg(p) };
            let err_code: i32 = unsafe { sqlite3_recover_errcode(p) };
            if e_verbosity > 0 {
                unsafe {
                    printf(c"recovery error: %s (%d)\n".as_ptr() as *mut i8 as
                            *const i8, z_err, err_code)
                };
            }
        }
        rc = unsafe { sqlite3_recover_finish(p) };
        if e_verbosity > 0 && rc != 0 {
            unsafe {
                printf(c"recovery returns error code %d\n".as_ptr() as *mut i8
                        as *const i8, rc)
            };
        }
        return rc;
    }
}
extern "C" fn bind_debug_parameters(p_stmt_1: *mut Sqlite3Stmt) -> () {
    let n_var: i32 = unsafe { sqlite3_bind_parameter_count(p_stmt_1) };
    let mut i: i32 = 0;
    {
        i = 1;
        '__b16: loop {
            if !(i <= n_var) { break '__b16; }
            '__c16: loop {
                let z_var: *const i8 =
                    unsafe { sqlite3_bind_parameter_name(p_stmt_1, i) };
                if z_var == core::ptr::null() { break '__c16; }
                if unsafe {
                            strncmp(z_var, c"$int_".as_ptr() as *mut i8 as *const i8,
                                5 as u64)
                        } == 0 {
                    unsafe {
                        sqlite3_bind_int(p_stmt_1, i,
                            unsafe { atoi(unsafe { &*z_var.offset(5 as isize) }) })
                    };
                } else if unsafe {
                            strncmp(z_var, c"$text_".as_ptr() as *mut i8 as *const i8,
                                6 as u64)
                        } == 0 {
                    let sz_var: u64 = unsafe { strlen(z_var) };
                    let z_buf: *mut i8 =
                        unsafe { sqlite3_malloc64(sz_var - 5 as u64) } as *mut i8;
                    if !(z_buf).is_null() {
                        unsafe {
                            memcpy(z_buf as *mut (),
                                unsafe { &raw const *z_var.offset(6 as isize) } as
                                    *const (), sz_var - 5 as u64)
                        };
                        unsafe {
                            sqlite3_bind_text64(p_stmt_1, i, z_buf as *const i8,
                                sz_var - 6 as u64, Some(sqlite3_free), 1 as u8)
                        };
                    }
                }
                break '__c16;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn run_db_sql(db: *mut Sqlite3, mut z_sql_1: *const i8,
    p_bts_flags_1: &mut u32, db_opt_1: u32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut b_corrupt: i32 = 0;
        while unsafe {
                    isspace(unsafe { *z_sql_1.offset(0 as isize) } as i32 & 127)
                } != 0 {
            {
                let __p = &mut z_sql_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z_sql_1.offset(0 as isize) } as i32 == 0 { return 0; }
        if e_verbosity >= 4 {
            unsafe {
                printf(c"RUNNING-SQL: [%s]\n".as_ptr() as *mut i8 as
                        *const i8, z_sql_1)
            };
            unsafe { fflush(__stdoutp) };
        }
        *p_bts_flags_1 &= 8 as u32;
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql_1, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc == 0 {
            let mut n_row: i32 = 0;
            bind_debug_parameters(p_stmt);
            while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
                { let __p = &mut n_row; let __t = *__p; *__p += 1; __t };
                if e_verbosity >= 4 {
                    let mut j: i32 = 0;
                    {
                        j = 0;
                        '__b19: loop {
                            if !(j < unsafe { sqlite3_column_count(p_stmt) }) {
                                break '__b19;
                            }
                            '__c19: loop {
                                if j != 0 {
                                    unsafe { printf(c",".as_ptr() as *mut i8 as *const i8) };
                                }
                                '__s20:
                                    {
                                    match unsafe { sqlite3_column_type(p_stmt, j) } {
                                        5 => {
                                            {
                                                unsafe { printf(c"NULL".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, j) })
                                                };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b21: loop {
                                                        if !(i < n) { break '__b21; }
                                                        '__c21: loop {
                                                            unsafe {
                                                                printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                                                    unsafe { *a.offset(i as isize) } as i32)
                                                            };
                                                            break '__c21;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b22: loop {
                                                        if !(i < n) { break '__b22; }
                                                        '__c22: loop {
                                                            if unsafe { *a.offset(i as isize) } as i32 == '\'' as i32 {
                                                                unsafe { printf(c"\'\'".as_ptr() as *mut i8 as *const i8) };
                                                            } else {
                                                                unsafe { putchar(unsafe { *a.offset(i as isize) } as i32) };
                                                            }
                                                            break '__c22;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                        }
                                        1 => {
                                            {
                                                unsafe {
                                                    printf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, j) })
                                                };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b21: loop {
                                                        if !(i < n) { break '__b21; }
                                                        '__c21: loop {
                                                            unsafe {
                                                                printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                                                    unsafe { *a.offset(i as isize) } as i32)
                                                            };
                                                            break '__c21;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b22: loop {
                                                        if !(i < n) { break '__b22; }
                                                        '__c22: loop {
                                                            if unsafe { *a.offset(i as isize) } as i32 == '\'' as i32 {
                                                                unsafe { printf(c"\'\'".as_ptr() as *mut i8 as *const i8) };
                                                            } else {
                                                                unsafe { putchar(unsafe { *a.offset(i as isize) } as i32) };
                                                            }
                                                            break '__c22;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                        }
                                        2 => {
                                            {
                                                unsafe {
                                                    printf(c"%s".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, j) })
                                                };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b21: loop {
                                                        if !(i < n) { break '__b21; }
                                                        '__c21: loop {
                                                            unsafe {
                                                                printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                                                    unsafe { *a.offset(i as isize) } as i32)
                                                            };
                                                            break '__c21;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b22: loop {
                                                        if !(i < n) { break '__b22; }
                                                        '__c22: loop {
                                                            if unsafe { *a.offset(i as isize) } as i32 == '\'' as i32 {
                                                                unsafe { printf(c"\'\'".as_ptr() as *mut i8 as *const i8) };
                                                            } else {
                                                                unsafe { putchar(unsafe { *a.offset(i as isize) } as i32) };
                                                            }
                                                            break '__c22;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                        }
                                        4 => {
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"x\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b21: loop {
                                                        if !(i < n) { break '__b21; }
                                                        '__c21: loop {
                                                            unsafe {
                                                                printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                                                    unsafe { *a.offset(i as isize) } as i32)
                                                            };
                                                            break '__c21;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b22: loop {
                                                        if !(i < n) { break '__b22; }
                                                        '__c22: loop {
                                                            if unsafe { *a.offset(i as isize) } as i32 == '\'' as i32 {
                                                                unsafe { printf(c"\'\'".as_ptr() as *mut i8 as *const i8) };
                                                            } else {
                                                                unsafe { putchar(unsafe { *a.offset(i as isize) } as i32) };
                                                            }
                                                            break '__c22;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                        }
                                        3 => {
                                            {
                                                let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, j) };
                                                let mut i: i32 = 0;
                                                let mut a: *const u8 = core::ptr::null();
                                                a = unsafe { sqlite3_column_blob(p_stmt, j) } as *const u8;
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                {
                                                    i = 0;
                                                    '__b22: loop {
                                                        if !(i < n) { break '__b22; }
                                                        '__c22: loop {
                                                            if unsafe { *a.offset(i as isize) } as i32 == '\'' as i32 {
                                                                unsafe { printf(c"\'\'".as_ptr() as *mut i8 as *const i8) };
                                                            } else {
                                                                unsafe { putchar(unsafe { *a.offset(i as isize) } as i32) };
                                                            }
                                                            break '__c22;
                                                        }
                                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                    }
                                                }
                                                unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                                break '__s20;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                break '__c19;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                    unsafe { fflush(__stdoutp) };
                }
            }
            if rc == 101 {
                if *p_bts_flags_1 == 1 as u32 &&
                            (unsafe { sqlite3_stmt_isexplain(p_stmt) } == 0) as i32 != 0
                        && n_row > 0 {
                    let mut i_row: i32 = 0;
                    unsafe { sqlite3_reset(p_stmt) };
                    while unsafe { sqlite3_step(p_stmt) } == 100 {
                        let mut i_cnt: i32 = 0;
                        { let __p = &mut i_row; let __t = *__p; *__p += 1; __t };
                        {
                            i_cnt = 0;
                            '__b24: loop {
                                if !(i_cnt < 99999) { break '__b24; }
                                '__c24: loop {
                                    rc =
                                        unsafe {
                                            fuzz_invariant(db, p_stmt, i_cnt, i_row, n_row,
                                                &mut b_corrupt, e_verbosity, db_opt_1)
                                        };
                                    if rc == 101 { break '__b24; }
                                    if rc != 1 {
                                        {
                                            let __p = &mut g.n_invariant;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    if e_verbosity > 0 {
                                        if rc == 0 {
                                            unsafe {
                                                printf(c"invariant-check: ok\n".as_ptr() as *mut i8 as
                                                        *const i8)
                                            };
                                        } else if rc == 11 {
                                            unsafe {
                                                printf(c"invariant-check: failed due to database corruption\n".as_ptr()
                                                            as *mut i8 as *const i8)
                                            };
                                        }
                                    }
                                    break '__c24;
                                }
                                { let __p = &mut i_cnt; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                }
            } else if e_verbosity >= 4 {
                unsafe {
                    printf(c"SQL-ERROR: (%d) %s\n".as_ptr() as *mut i8 as
                            *const i8, rc, unsafe { sqlite3_errmsg(db) })
                };
                unsafe { fflush(__stdoutp) };
            }
        } else if e_verbosity >= 4 {
            unsafe {
                printf(c"SQL-ERROR (%d): %s\n".as_ptr() as *mut i8 as
                        *const i8, rc, unsafe { sqlite3_errmsg(db) })
            };
            unsafe { fflush(__stdoutp) };
        }
        return unsafe { sqlite3_finalize(p_stmt) };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AnonS0 {
    mask: u32,
    i_setting: i32,
    z_name: *mut i8,
}
static mut a_db_config_settings: [AnonS0; 13] =
    [AnonS0 {
                mask: 1 as u32,
                i_setting: 1002,
                z_name: c"enable_fkey".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 2 as u32,
                i_setting: 1003,
                z_name: c"enable_trigger".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 4 as u32,
                i_setting: 1015,
                z_name: c"enable_view".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 8 as u32,
                i_setting: 1007,
                z_name: c"enable_qpsg".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 16 as u32,
                i_setting: 1008,
                z_name: c"trigger_eqp".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 32 as u32,
                i_setting: 1010,
                z_name: c"defensive".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 64 as u32,
                i_setting: 1011,
                z_name: c"writable_schema".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 128 as u32,
                i_setting: 1012,
                z_name: c"legacy_alter_table".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 256 as u32,
                i_setting: 1018,
                z_name: c"stmt_scanstatus".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 512 as u32,
                i_setting: 1019,
                z_name: c"reverse_scanorder".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 2048 as u32,
                i_setting: 1013,
                z_name: c"dqs_dml".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 4096 as u32,
                i_setting: 1014,
                z_name: c"dqs_ddl".as_ptr() as *mut i8,
            },
            AnonS0 {
                mask: 8192 as u32,
                i_setting: 1017,
                z_name: c"trusted_schema".as_ptr() as *mut i8,
            }];
extern "C" fn toggle_db_config(db: *mut Sqlite3, i_setting_1: i32) -> () {
    let mut v: i32 = 0;
    unsafe { sqlite3_db_config(db, i_setting_1, -1, &raw mut v as *mut i32) };
    v = (v == 0) as i32 as i32;
    unsafe { sqlite3_db_config(db, i_setting_1, v, 0) };
}
#[unsafe(no_mangle)]
pub extern "C" fn run_combined_db_sql_input(a_data_1: *const u8,
    n_byte_1: u64, i_timeout_1: i32, b_script_1: i32, i_sql_id_1: i32)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i_sql: i32 = 0;
        let mut a_db: *mut u8 = core::ptr::null_mut();
        let mut n_db: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut n_sql: i32 = 0;
        let mut cx: FuzzCtx = unsafe { core::mem::zeroed() };
        let mut bts_flags: u32 = 0 as u32;
        let mut db_flags: u32 = 0 as u32;
        let mut db_opt: u32 = 0 as u32;
        let mut n_alloc: i32 = 0;
        let mut n_not_used: i32 = 0;
        let mut z_name: [i8; 100] = [0; 100];
        let mut x: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut c_saved: i8 = 0 as i8;
        let mut n_alloc_1: i32 = 0;
        let mut n_not_used_1: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s26:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 118;
                    }
                    3 => { __state = 4; }
                    4 => { a_db = core::ptr::null_mut(); __state = 5; }
                    5 => { n_db = 0; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { z_sql = core::ptr::null_mut(); __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { bts_flags = 0 as u32; __state = 12; }
                    12 => { db_flags = 0 as u32; __state = 13; }
                    13 => { db_opt = 0 as u32; __state = 14; }
                    14 => {
                        if n_byte_1 < 10 as u64 {
                            __state = 16;
                        } else { __state = 15; }
                    }
                    15 => {
                        if unsafe { sqlite3_initialize() } != 0 {
                            __state = 18;
                        } else { __state = 17; }
                    }
                    16 => { return 0; }
                    17 => {
                        if unsafe { sqlite3_memory_used() } != 0 as i64 {
                            __state = 20;
                        } else { __state = 19; }
                    }
                    18 => { return 0; }
                    19 => {
                        unsafe {
                            memset(&raw mut cx as *mut (), 0,
                                core::mem::size_of::<FuzzCtx>() as u64)
                        };
                        __state = 25;
                    }
                    20 => { n_alloc = 0; __state = 21; }
                    21 => { n_not_used = 0; __state = 22; }
                    22 => {
                        unsafe {
                            sqlite3_status(9, &mut n_alloc, &mut n_not_used, 0)
                        };
                        __state = 23;
                    }
                    23 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"memory leak prior to test start: %lld bytes in %d allocations\n".as_ptr()
                                        as *mut i8 as *const i8, unsafe { sqlite3_memory_used() },
                                n_alloc)
                        };
                        __state = 24;
                    }
                    24 => { unsafe { exit(1) }; __state = 19; }
                    25 => {
                        i_sql =
                            decode_database(a_data_1 as *mut u8 as *const u8,
                                n_byte_1 as i32, &mut a_db, &mut n_db);
                        __state = 26;
                    }
                    26 => {
                        if i_sql < 0 { __state = 28; } else { __state = 27; }
                    }
                    27 => {
                        if n_db >= 75 { __state = 30; } else { __state = 29; }
                    }
                    28 => { return 0; }
                    29 => {
                        if n_db >= 79 { __state = 32; } else { __state = 31; }
                    }
                    30 => {
                        db_flags =
                            ((unsafe { *a_db.offset(72 as isize) } as u32) << 24) +
                                        ((unsafe { *a_db.offset(73 as isize) } as u32) << 16) +
                                    ((unsafe { *a_db.offset(74 as isize) } as u32) << 8) +
                                unsafe { *a_db.offset(75 as isize) } as u32;
                        __state = 29;
                    }
                    31 => {
                        n_sql = (n_byte_1 - i_sql as u64) as i32;
                        __state = 33;
                    }
                    32 => {
                        db_opt =
                            ((unsafe { *a_db.offset(76 as isize) } as u32) << 24) +
                                        ((unsafe { *a_db.offset(77 as isize) } as u32) << 16) +
                                    ((unsafe { *a_db.offset(78 as isize) } as u32) << 8) +
                                unsafe { *a_db.offset(79 as isize) } as u32;
                        __state = 31;
                    }
                    33 => {
                        if b_script_1 != 0 { __state = 35; } else { __state = 34; }
                    }
                    34 => {
                        if e_verbosity >= 3 { __state = 41; } else { __state = 40; }
                    }
                    35 => { __state = 36; }
                    36 => {
                        unsafe {
                            sqlite3_snprintf(core::mem::size_of::<[i8; 100]>() as i32,
                                &raw mut z_name[0 as usize] as *mut i8,
                                c"dbsql%06d.db".as_ptr() as *mut i8 as *const i8,
                                i_sql_id_1)
                        };
                        __state = 37;
                    }
                    37 => {
                        render_db_sql_for_cli(__stdoutp,
                            &raw mut z_name[0 as usize] as *mut i8 as *const i8, a_db,
                            n_db, unsafe { a_data_1.offset(i_sql as isize) } as *mut u8,
                            n_sql);
                        __state = 38;
                    }
                    38 => {
                        unsafe { sqlite3_free(a_db as *mut ()) };
                        __state = 39;
                    }
                    39 => { return 0; }
                    40 => {
                        rc = unsafe { sqlite3_open(core::ptr::null(), &mut cx.db) };
                        __state = 43;
                    }
                    41 => {
                        unsafe {
                            printf(c"****** %d-byte input, %d-byte database, %d-byte script ******\n".as_ptr()
                                        as *mut i8 as *const i8, n_byte_1 as i32, n_db, n_sql)
                        };
                        __state = 42;
                    }
                    42 => { unsafe { fflush(__stdoutp) }; __state = 40; }
                    43 => {
                        if rc != 0 { __state = 45; } else { __state = 44; }
                    }
                    44 => {
                        unsafe { sqlite3_test_control(15, cx.db, db_opt) };
                        __state = 47;
                    }
                    45 => {
                        unsafe { sqlite3_free(a_db as *mut ()) };
                        __state = 46;
                    }
                    46 => { return 1; }
                    47 => { i = 0; __state = 49; }
                    48 => {
                        if b_vdbe_debug != 0 {
                            __state = 54;
                        } else { __state = 53; }
                    }
                    49 => {
                        if (i as u64) <
                                core::mem::size_of::<[AnonS0; 13]>() as u64 / 16 {
                            __state = 50;
                        } else { __state = 48; }
                    }
                    50 => {
                        if db_flags & a_db_config_settings[i as usize].mask as u32
                                != 0 {
                            __state = 52;
                        } else { __state = 51; }
                    }
                    51 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 49;
                    }
                    52 => {
                        toggle_db_config(cx.db,
                            a_db_config_settings[i as usize].i_setting);
                        __state = 51;
                    }
                    53 => { cx.i_last_cb = time_of_day(); __state = 55; }
                    54 => {
                        unsafe {
                            sqlite3_exec(cx.db,
                                c"PRAGMA vdbe_debug=ON".as_ptr() as *mut i8 as *const i8,
                                None, core::ptr::null_mut(), core::ptr::null_mut())
                        };
                        __state = 53;
                    }
                    55 => {
                        cx.i_cutoff_time =
                            cx.i_last_cb +
                                if i_timeout_1 < gi_timeout {
                                        i_timeout_1
                                    } else { gi_timeout } as Sqlite3Int64;
                        __state = 56;
                    }
                    56 => { cx.mx_cb = mx_progress_cb; __state = 57; }
                    57 => {
                        unsafe {
                            sqlite3_progress_handler(cx.db, 10, Some(progress_handler),
                                &raw mut cx as *mut ())
                        };
                        __state = 58;
                    }
                    58 => {
                        if vdbe_op_limit > 0 {
                            __state = 60;
                        } else { __state = 59; }
                    }
                    59 => {
                        if length_limit > 0 { __state = 62; } else { __state = 61; }
                    }
                    60 => {
                        unsafe { sqlite3_limit(cx.db, 5, vdbe_op_limit) };
                        __state = 59;
                    }
                    61 => {
                        if depth_limit > 0 { __state = 64; } else { __state = 63; }
                    }
                    62 => {
                        unsafe { sqlite3_limit(cx.db, 0, length_limit) };
                        __state = 61;
                    }
                    63 => {
                        unsafe { sqlite3_limit(cx.db, 8, 100) };
                        __state = 65;
                    }
                    64 => {
                        unsafe { sqlite3_limit(cx.db, 3, depth_limit) };
                        __state = 63;
                    }
                    65 => {
                        unsafe { sqlite3_hard_heap_limit64(heap_limit) };
                        __state = 66;
                    }
                    66 => { rc = 1; __state = 67; }
                    67 => {
                        unsafe {
                            sqlite3_test_control(14, &raw mut rc as *mut i32)
                        };
                        __state = 68;
                    }
                    68 => {
                        if n_db >= 20 &&
                                    unsafe { *a_db.offset(18 as isize) } as i32 == 2 &&
                                unsafe { *a_db.offset(19 as isize) } as i32 == 2 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        rc =
                            unsafe {
                                sqlite3_deserialize(cx.db,
                                    c"main".as_ptr() as *mut i8 as *const i8, a_db,
                                    n_db as Sqlite3Int64, n_db as Sqlite3Int64, (2 | 1) as u32)
                            };
                        __state = 71;
                    }
                    70 => {
                        unsafe {
                            *a_db.offset(18 as isize) =
                                {
                                    unsafe { *a_db.offset(19 as isize) = 1 as u8 };
                                    unsafe { *a_db.offset(19 as isize) }
                                }
                        };
                        __state = 69;
                    }
                    71 => {
                        if rc != 0 { __state = 73; } else { __state = 72; }
                    }
                    72 => {
                        if max_db_size > 0 as i64 {
                            __state = 76;
                        } else { __state = 75; }
                    }
                    73 => {
                        eprintln!("sqlite3_deserialize() failed with {}", rc as i32);
                        __state = 74;
                    }
                    74 => { __state = 2; }
                    75 => {
                        if e_verbosity >= 5 { __state = 79; } else { __state = 78; }
                    }
                    76 => { x = max_db_size; __state = 77; }
                    77 => {
                        unsafe {
                            sqlite3_file_control(cx.db,
                                c"main".as_ptr() as *mut i8 as *const i8, 36,
                                &raw mut x as *mut ())
                        };
                        __state = 75;
                    }
                    78 => {
                        unsafe {
                            sqlite3_set_authorizer(cx.db, Some(block_troublesome_sql),
                                &raw mut bts_flags as *mut ())
                        };
                        __state = 80;
                    }
                    79 => {
                        unsafe {
                            sqlite3_exec(cx.db,
                                c"PRAGMA vdbe_debug=ON;".as_ptr() as *mut i8 as *const i8,
                                None, core::ptr::null_mut(), core::ptr::null_mut())
                        };
                        __state = 78;
                    }
                    80 => {
                        unsafe {
                            sqlite3_vt02_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 81;
                    }
                    81 => {
                        unsafe {
                            sqlite3_randomjson_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 82;
                    }
                    82 => {
                        unsafe {
                            sqlite3_series_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 83;
                    }
                    83 => {
                        unsafe {
                            sqlite3_base64_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 84;
                    }
                    84 => {
                        unsafe {
                            sqlite3_base85_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 85;
                    }
                    85 => {
                        unsafe {
                            sqlite3_completion_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 86;
                    }
                    86 => {
                        unsafe {
                            sqlite3_decimal_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 87;
                    }
                    87 => {
                        unsafe {
                            sqlite3_diskused_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 88;
                    }
                    88 => {
                        unsafe {
                            sqlite3_ieee_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 89;
                    }
                    89 => {
                        unsafe {
                            sqlite3_regexp_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 90;
                    }
                    90 => {
                        unsafe {
                            sqlite3_shathree_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 91;
                    }
                    91 => {
                        unsafe {
                            sqlite3_sha_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 92;
                    }
                    92 => {
                        unsafe {
                            sqlite3_stmtrand_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 93;
                    }
                    93 => {
                        unsafe {
                            sqlite3_dbdata_init(cx.db, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 94;
                    }
                    94 => {
                        unsafe {
                            sqlite3_table_column_metadata(cx.db, core::ptr::null(),
                                c"x".as_ptr() as *mut i8 as *const i8, core::ptr::null(),
                                core::ptr::null_mut(), core::ptr::null_mut(),
                                core::ptr::null_mut(), core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 95;
                    }
                    95 => {
                        unsafe { sqlite3_test_control(28, 1, cx.db) };
                        __state = 96;
                    }
                    96 => {
                        if (b_no_recover == 0) as i32 != 0 {
                            __state = 98;
                        } else { __state = 97; }
                    }
                    97 => {
                        z_sql = unsafe { sqlite3_malloc(n_sql + 1) } as *mut i8;
                        __state = 99;
                    }
                    98 => { recover_database(cx.db); __state = 97; }
                    99 => {
                        if z_sql == core::ptr::null_mut() {
                            __state = 101;
                        } else { __state = 102; }
                    }
                    100 => { __state = 2; }
                    101 => { eprintln!("Out of memory!"); __state = 100; }
                    102 => {
                        unsafe {
                            memcpy(z_sql as *mut (),
                                unsafe { a_data_1.offset(i_sql as isize) } as *const (),
                                n_sql as u64)
                        };
                        __state = 103;
                    }
                    103 => {
                        unsafe { *z_sql.offset(n_sql as isize) = 0 as i8 };
                        __state = 104;
                    }
                    104 => { i = { j = 0; j }; __state = 106; }
                    105 => {
                        if j < i { __state = 117; } else { __state = 100; }
                    }
                    106 => {
                        if unsafe { *z_sql.offset(i as isize) } != 0 {
                            __state = 107;
                        } else { __state = 105; }
                    }
                    107 => {
                        if unsafe { *z_sql.offset(i as isize) } as i32 == ';' as i32
                            {
                            __state = 109;
                        } else { __state = 108; }
                    }
                    108 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 106;
                    }
                    109 => {
                        c_saved = unsafe { *z_sql.offset((i + 1) as isize) };
                        __state = 110;
                    }
                    110 => {
                        unsafe { *z_sql.offset((i + 1) as isize) = 0 as i8 };
                        __state = 111;
                    }
                    111 => {
                        if unsafe {
                                    sqlite3_complete(unsafe { z_sql.offset(j as isize) } as
                                            *const i8)
                                } != 0 {
                            __state = 113;
                        } else { __state = 112; }
                    }
                    112 => {
                        unsafe { *z_sql.offset((i + 1) as isize) = c_saved };
                        __state = 115;
                    }
                    113 => {
                        rc =
                            run_db_sql(cx.db,
                                unsafe { z_sql.offset(j as isize) } as *const i8,
                                &mut bts_flags, db_opt);
                        __state = 114;
                    }
                    114 => { j = i + 1; __state = 112; }
                    115 => {
                        if rc == 9 || progress_handler(&raw mut cx as *mut ()) != 0
                            {
                            __state = 116;
                        } else { __state = 108; }
                    }
                    116 => { __state = 2; }
                    117 => {
                        run_db_sql(cx.db,
                            unsafe { z_sql.offset(j as isize) } as *const i8,
                            &mut bts_flags, db_opt);
                        __state = 100;
                    }
                    118 => {
                        rc = unsafe { sqlite3_close(cx.db) };
                        __state = 119;
                    }
                    119 => {
                        if rc != 0 { __state = 121; } else { __state = 120; }
                    }
                    120 => {
                        if e_verbosity >= 2 && (b_script_1 == 0) as i32 != 0 {
                            __state = 123;
                        } else { __state = 122; }
                    }
                    121 => {
                        unsafe {
                            fprintf(__stdoutp,
                                c"sqlite3_close() returns %d\n".as_ptr() as *mut i8 as
                                    *const i8, rc)
                        };
                        __state = 120;
                    }
                    122 => {
                        if unsafe { sqlite3_memory_used() } != 0 as i64 {
                            __state = 125;
                        } else { __state = 124; }
                    }
                    123 => {
                        unsafe {
                            fprintf(__stdoutp,
                                c"Peak memory usages: %f MB\n".as_ptr() as *mut i8 as
                                    *const i8,
                                unsafe { sqlite3_memory_highwater(1) } as f64 / 1000000.0)
                        };
                        __state = 122;
                    }
                    124 => {
                        unsafe { sqlite3_hard_heap_limit64(0 as Sqlite3Int64) };
                        __state = 130;
                    }
                    125 => { n_alloc_1 = 0; __state = 126; }
                    126 => { n_not_used_1 = 0; __state = 127; }
                    127 => {
                        unsafe {
                            sqlite3_status(9, &mut n_alloc_1, &mut n_not_used_1, 0)
                        };
                        __state = 128;
                    }
                    128 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"Memory leak: %lld bytes in %d allocations\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { sqlite3_memory_used() },
                                n_alloc_1)
                        };
                        __state = 129;
                    }
                    129 => { unsafe { exit(1) }; __state = 124; }
                    130 => {
                        unsafe { sqlite3_soft_heap_limit64(0 as Sqlite3Int64) };
                        __state = 131;
                    }
                    131 => { return 0; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn is_db_sql(mut a: *const u8, mut n: i32) -> i32 {
    let mut buf: [u8; 12] = [0; 12];
    let mut i: i32 = 0;
    if n > 4 &&
            unsafe {
                    memcmp(a as *const (),
                        c"\n--\n".as_ptr() as *mut i8 as *const (), 4 as u64)
                } == 0 {
        return 1;
    }
    while n > 0 &&
            unsafe { isspace(unsafe { *a.offset(0 as isize) } as i32) } != 0 {
        {
            let __p = &mut a;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    {
        i = 0;
        '__b28: loop {
            if !(n > 0 && i < 8) { break '__b28; }
            '__c28: loop {
                if unsafe {
                            isxdigit(unsafe { *a.offset(0 as isize) } as i32)
                        } != 0 {
                    buf[{ let __p = &mut i; let __t = *__p; *__p += 1; __t } as
                                usize] = unsafe { *a.offset(0 as isize) };
                }
                break '__c28;
            }
            {
                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                {
                    let __p = &mut a;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if i == 8 &&
            unsafe {
                    memcmp(&raw mut buf[0 as usize] as *mut u8 as *const (),
                        c"53514c69".as_ptr() as *mut i8 as *const (), 8 as u64)
                } == 0 {
        return 1;
    }
    return 0;
}
extern "C" fn is_db_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let n: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    let a: *mut u8 =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *mut u8;
    unsafe {
        sqlite3_result_int(context,
            (a != core::ptr::null_mut() && n > 0 &&
                    is_db_sql(a as *const u8, n) != 0) as i32)
    };
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
            memcpy(p_data_1,
                unsafe { unsafe { (*p_v_file).a.offset(i_ofst_1 as isize) } }
                    as *const (), i_amt_1 as u64)
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
        if i_ofst_1 + i_amt_1 as Sqlite3Int64 >= 10000000 as i64 {
            return 13;
        }
        unsafe {
            (*p_v_file).a =
                safe_realloc(unsafe { (*p_v_file).a } as *mut (),
                        (i_ofst_1 + i_amt_1 as Sqlite3Int64) as i32) as *mut u8
        };
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
            create_v_file(z_filename_1, 0,
                c"".as_ptr() as *mut u8 as *const u8);
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
extern "C" fn inmem_randomness(not_used_1: *mut Sqlite3Vfs, n_buf_1: i32,
    z_buf_1: *mut i8) -> i32 {
    unsafe {
        unsafe { memset(z_buf_1 as *mut (), 0, n_buf_1 as u64) };
        unsafe {
            memcpy(z_buf_1 as *mut (), &raw mut g.u_random as *const (),
                if (n_buf_1 as u64) < core::mem::size_of::<u32>() as u64 {
                    n_buf_1 as u64
                } else { core::mem::size_of::<u32>() as u64 })
        };
        return n_buf_1;
    }
}
extern "C" fn inmem_vfs_register(make_default_1: i32) -> () {
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
        inmem_vfs.x_randomness = Some(inmem_randomness);
        inmem_vfs.x_sleep = unsafe { (*p_default).x_sleep };
        inmem_vfs.x_current_time_int64 =
            unsafe { (*p_default).x_current_time_int64 };
        unsafe { sqlite3_vfs_register(&mut inmem_vfs, make_default_1) };
    }
}
extern "C" fn run_sql(db: *mut Sqlite3, mut z_sql_1: *const i8,
    run_flags_1: u32) -> () {
    let mut z_more: *const i8 = core::ptr::null();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    while !(z_sql_1).is_null() && unsafe { *z_sql_1.offset(0 as isize) } != 0
        {
        z_more = core::ptr::null();
        p_stmt = core::ptr::null_mut();
        unsafe {
            sqlite3_prepare_v2(db, z_sql_1, -1, &mut p_stmt, &mut z_more)
        };
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
                while 100 == unsafe { sqlite3_step(p_stmt) } {
                    let mut i: i32 = 0;
                    if n_col < 0 {
                        n_col = unsafe { sqlite3_column_count(p_stmt) };
                    } else if n_col > 0 {
                        unsafe {
                            printf(c"--------------------------------------------\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    }
                    {
                        i = 0;
                        '__b34: loop {
                            if !(i < n_col) { break '__b34; }
                            '__c34: loop {
                                let e_type: i32 = unsafe { sqlite3_column_type(p_stmt, i) };
                                unsafe {
                                    printf(c"%s = ".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_name(p_stmt, i) })
                                };
                                '__s35:
                                    {
                                    match e_type {
                                        5 => {
                                            {
                                                unsafe {
                                                    printf(c"NULL\n".as_ptr() as *mut i8 as *const i8)
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"INT %s\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                            *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                        }
                                        1 => {
                                            {
                                                unsafe {
                                                    printf(c"INT %s\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                            *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                        }
                                        2 => {
                                            {
                                                unsafe {
                                                    printf(c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                            *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                        }
                                        3 => {
                                            {
                                                unsafe {
                                                    printf(c"TEXT [%s]\n".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { sqlite3_column_text(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                            *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                        }
                                        4 => {
                                            {
                                                unsafe {
                                                    printf(c"BLOB (%d bytes)\n".as_ptr() as *mut i8 as
                                                            *const i8, unsafe { sqlite3_column_bytes(p_stmt, i) })
                                                };
                                                break '__s35;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                break '__c34;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
            }
            unsafe { sqlite3_finalize(p_stmt) };
        }
    }
}
extern "C" fn rebuild_database(db: *mut Sqlite3, db_sql_only_1: i32) -> () {
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    z_sql =
        unsafe {
            sqlite3_mprintf(c"BEGIN;\nCREATE TEMP TABLE dbx AS SELECT DISTINCT dbcontent FROM db;\nDELETE FROM db;\nINSERT INTO db(dbid, dbcontent)  SELECT NULL, dbcontent FROM dbx ORDER BY 2;\nDROP TABLE dbx;\nCREATE TEMP TABLE sx AS SELECT DISTINCT sqltext FROM xsql %s;\nDELETE FROM xsql;\nINSERT INTO xsql(sqlid,sqltext)  SELECT NULL, sqltext FROM sx ORDER BY 2;\nDROP TABLE sx;\nCOMMIT;\nPRAGMA page_size=1024;\nVACUUM;\n".as_ptr()
                        as *mut i8 as *const i8,
                if db_sql_only_1 != 0 {
                    c" WHERE isdbsql(sqltext)".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 })
        };
    rc =
        unsafe {
            sqlite3_exec(db, z_sql as *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc != 0 {
        unsafe {
            fatal_error(c"cannot rebuild: %s".as_ptr() as *mut i8 as
                    *const i8, unsafe { sqlite3_errmsg(db) })
        };
    }
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
            '__b38: loop {
                if !((i as u64) <
                                core::mem::size_of::<[IntegerValueS0N16integerValueS0; 9]>()
                                        as u64 / 16) {
                    break '__b38;
                }
                '__c38: loop {
                    if unsafe {
                                sqlite3_stricmp(a_mult[i as usize].z_suffix as *const i8,
                                    z_arg_1)
                            } == 0 {
                        v *= a_mult[i as usize].i_mult as Sqlite3Int64;
                        break '__b38;
                    }
                    break '__c38;
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
extern "C" fn number_of_v_char(mut z: *const i8) -> i32 {
    let mut n: i32 = 0;
    while unsafe { *z.offset(0 as isize) } != 0 &&
            unsafe { *z.offset(0 as isize) } as i32 == 'v' as i32 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
    }
    return if unsafe { *z.offset(0 as isize) } as i32 == 0 { n } else { 0 };
}
extern "C" fn show_help() -> () {
    unsafe {
        unsafe {
            printf(c"Usage: %s [options] SOURCE-DB ?ARGS...?\n".as_ptr() as
                        *mut i8 as *const i8, g.z_argv0)
        };
        unsafe {
            printf(c"Read databases and SQL scripts from SOURCE-DB and execute each script against\neach database, checking for crashes and memory leaks.\nOptions:\n  --brief              Output only a summary of results at the end\n  --cell-size-check    Set the PRAGMA cell_size_check=ON\n  --dbid M..N          Use only the databases where dbid between M and N\n                       \"M..\" for M and afterwards. Just \"M\" for M only\n  --export-db DIR      Write databases to files(s) in DIR. Works with --dbid\n  --export-sql DIR     Write SQL to file(s) in DIR. Also works with --sqlid\n  --help               Show this help text\n  --info               Show information about SOURCE-DB w/o running tests\n  --limit-depth N      Limit expression depth to N.  Default: 500\n  --limit-heap N       Limit heap memory to N.  Default: 100M\n  --limit-mem N        Limit memory used by test SQLite instance to N bytes\n  --limit-vdbe         Panic if any test runs for more than 100,000 cycles\n  --load-sql   FILE..  Load SQL scripts fron files into SOURCE-DB\n  --load-db    FILE..  Load template databases from files into SOURCE_DB\n  --load-dbsql FILE..  Load dbsqlfuzz outputs into the xsql table\n               ^^^^------ Use \"-\" for FILE to read filenames from stdin\n  -m TEXT              Add a description to the database\n  --native-vfs         Use the native VFS for initially empty database files\n  --native-malloc      Turn off MEMSYS3/5 and Lookaside\n  --no-recover         Do not run recovery on dbsqlfuzz databases\n  --oss-fuzz           Enable OSS-FUZZ testing\n  --prng-seed N        Seed value for the PRGN inside of SQLite\n  -q|--quiet           Reduced output\n  --rebuild            Rebuild and vacuum the database file\n  --result-trace       Show the results of each SQL command\n  --script             Output CLI script instead of running tests\n  --skip N             Skip the first N test cases\n  --slice M N          Run only the M-th out of each group of N tests\n  --spinner            Use a spinner to show progress\n  --sqlid M..N         Use only SQL where sqlid between M..N\n                       \"M..\" for M and afterwards. Just \"M\" for M only\n  --timeout N          Maximum time for any one test in N millseconds\n  -v|--verbose         Increased output.  Repeat for more output.\n  --vdbe-debug         Activate VDBE debugging.\n  --wait N             Wait N seconds before continuing - useful for\n                          attaching an MSVC debugging.\n".as_ptr()
                        as *mut i8 as *const i8)
        };
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut i_begin: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut quiet_flag: i32 = 0;
        let mut brief_flag: i32 = 0;
        let mut verbose_flag: i32 = 0;
        let mut z_ins_sql: *mut i8 = core::ptr::null_mut();
        let mut i_first_ins_arg: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut p_sql: *mut Blob = core::ptr::null_mut();
        let mut p_db: *mut Blob = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut db_sql_only: i32 = 0;
        let mut first_sqlid: i32 = 0;
        let mut last_sqlid: i32 = 0;
        let mut first_dbid: i32 = 0;
        let mut last_dbid: i32 = 0;
        let mut native_flag: i32 = 0;
        let mut rebuild_flag: i32 = 0;
        let mut vdbe_limit_flag: i32 = 0;
        let mut info_flag: i32 = 0;
        let mut n_skip: i32 = 0;
        let mut b_script: i32 = 0;
        let mut b_spinner: i32 = 0;
        let mut timeout_test: i32 = 0;
        let mut run_flags: i32 = 0;
        let mut z_msg: *mut i8 = core::ptr::null_mut();
        let mut n_src_db: i32 = 0;
        let mut az_src_db: *mut *mut i8 = core::ptr::null_mut();
        let mut i_src_db: i32 = 0;
        let mut n_test: i32 = 0;
        let mut n_slice_skip: i32 = 0;
        let mut z_db_name: *mut i8 = core::ptr::null_mut();
        let mut z_fail_code: *const i8 = core::ptr::null();
        let mut cell_sz_ck_flag: i32 = 0;
        let mut sql_fuzz: i32 = 0;
        let mut i_timeout: i32 = 0;
        let mut n_mem: i32 = 0;
        let mut n_mem_this_db: i32 = 0;
        let mut z_exp_db: *const i8 = core::ptr::null();
        let mut z_exp_sql: *const i8 = core::ptr::null();
        let mut p_heap: *mut () = core::ptr::null_mut();
        let mut oss_fuzz: i32 = 0;
        let mut oss_fuzz_this_db: i32 = 0;
        let mut native_malloc: i32 = 0;
        let mut p_dflt_vfs: *const Sqlite3Vfs = core::ptr::null();
        let mut open_flags4_data: i32 = 0;
        let mut b_timer: i32 = 0;
        let mut n_v: i32 = 0;
        let mut tm_start: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i_est_time: i32 = 0;
        let mut i_slice_sz: i32 = 0;
        let mut i_slice_idx: i32 = 0;
        let mut z: *const i8 = core::ptr::null();
        let mut z_dot_dot: *const i8 = core::ptr::null();
        let mut z_dot_dot_1: *const i8 = core::ptr::null();
        let mut ii: i32 = 0;
        let mut zz: *const i8 = core::ptr::null();
        let mut i_delay: i32 = 0;
        let mut n_data: i64 = 0 as i64;
        let mut a_data: *mut i8 = core::ptr::null_mut();
        let mut z_raw_data: *mut i8 = core::ptr::null_mut();
        let mut n_raw_data: i64 = 0 as i64;
        let mut n: i32 = 0;
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut z_name: *const i8 = core::ptr::null();
        let mut z_line: [i8; 2000] = [0; 2000];
        let mut kk: u64 = 0 as u64;
        let mut z_ex_db: *const i8 = core::ptr::null();
        let mut z_ex_sql: *const i8 = core::ptr::null();
        let mut n_total: i32 = 0;
        let mut idx: i32 = 0;
        let mut i_to_go: i32 = 0;
        let mut hr: i32 = 0;
        let mut min: i32 = 0;
        let mut sec: i32 = 0;
        let mut idx__1: i32 = 0;
        let mut amt: i32 = 0;
        let mut tm_end: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut open_flags: i32 = 0;
        let mut z_vfs: *const i8 = core::ptr::null();
        let mut n_total_1: i32 = 0;
        let mut idx__2: i32 = 0;
        let mut idx__3: i32 = 0;
        let mut amt__1: i32 = 0;
        let mut z_name_1: [i8; 100] = [0; 100];
        let mut tm_end_1: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i_elapse: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut n_total_2: i32 = 0;
        let mut i_elapse_1: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s41:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { blob_list_free(g.p_first_sql); __state = 602; }
                    3 => { quiet_flag = 0; __state = 4; }
                    4 => { brief_flag = 0; __state = 5; }
                    5 => { verbose_flag = 0; __state = 6; }
                    6 => { z_ins_sql = core::ptr::null_mut(); __state = 7; }
                    7 => { i_first_ins_arg = 0; __state = 8; }
                    8 => { db = core::ptr::null_mut(); __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => { db_sql_only = 0; __state = 15; }
                    15 => { first_sqlid = -1; __state = 16; }
                    16 => { last_sqlid = 2147483647; __state = 17; }
                    17 => { first_dbid = -1; __state = 18; }
                    18 => { last_dbid = 2147483647; __state = 19; }
                    19 => { native_flag = 0; __state = 20; }
                    20 => { rebuild_flag = 0; __state = 21; }
                    21 => { vdbe_limit_flag = 0; __state = 22; }
                    22 => { info_flag = 0; __state = 23; }
                    23 => { n_skip = 0; __state = 24; }
                    24 => { b_script = 0; __state = 25; }
                    25 => { b_spinner = 0; __state = 26; }
                    26 => { timeout_test = 0; __state = 27; }
                    27 => { run_flags = 0; __state = 28; }
                    28 => { z_msg = core::ptr::null_mut(); __state = 29; }
                    29 => { n_src_db = 0; __state = 30; }
                    30 => { az_src_db = core::ptr::null_mut(); __state = 31; }
                    31 => { __state = 32; }
                    32 => { n_test = 0; __state = 33; }
                    33 => { n_slice_skip = 0; __state = 34; }
                    34 => { z_db_name = c"".as_ptr() as *mut i8; __state = 35; }
                    35 => { z_fail_code = core::ptr::null(); __state = 36; }
                    36 => { cell_sz_ck_flag = 0; __state = 37; }
                    37 => { sql_fuzz = 0; __state = 38; }
                    38 => { i_timeout = 120000; __state = 39; }
                    39 => { n_mem = 0; __state = 40; }
                    40 => { n_mem_this_db = 0; __state = 41; }
                    41 => { z_exp_db = core::ptr::null(); __state = 42; }
                    42 => { z_exp_sql = core::ptr::null(); __state = 43; }
                    43 => { p_heap = core::ptr::null_mut(); __state = 44; }
                    44 => { oss_fuzz = 0; __state = 45; }
                    45 => { oss_fuzz_this_db = 0; __state = 46; }
                    46 => { native_malloc = 0; __state = 47; }
                    47 => { __state = 48; }
                    48 => { __state = 49; }
                    49 => { b_timer = 0; __state = 50; }
                    50 => { __state = 51; }
                    51 => { __state = 52; }
                    52 => { i_est_time = 0; __state = 53; }
                    53 => { i_slice_sz = 0; __state = 54; }
                    54 => { i_slice_idx = 0; __state = 55; }
                    55 => { unsafe { sqlite3_config(17, 1) }; __state = 56; }
                    56 => { unsafe { sqlite3_config(9, 1) }; __state = 57; }
                    57 => { register_oom_simulator(); __state = 58; }
                    58 => { unsafe { sqlite3_initialize() }; __state = 59; }
                    59 => { i_begin = time_of_day(); __state = 60; }
                    60 => {
                        g.z_argv0 =
                            unsafe { *argv.offset(0 as isize) } as *const i8;
                        __state = 61;
                    }
                    61 => { open_flags4_data = 1; __state = 62; }
                    62 => {
                        z_fail_code =
                            unsafe {
                                    getenv(c"TEST_FAILURE".as_ptr() as *mut i8 as *const i8)
                                } as *const i8;
                        __state = 63;
                    }
                    63 => {
                        p_dflt_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
                        __state = 64;
                    }
                    64 => { inmem_vfs_register(1); __state = 65; }
                    65 => { i = 1; __state = 67; }
                    66 => {
                        if n_src_db == 0 { __state = 255; } else { __state = 254; }
                    }
                    67 => {
                        if i < argc { __state = 68; } else { __state = 66; }
                    }
                    68 => {
                        z = unsafe { *argv.offset(i as isize) } as *const i8;
                        __state = 70;
                    }
                    69 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 67;
                    }
                    70 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            __state = 71;
                        } else { __state = 72; }
                    }
                    71 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 73;
                    }
                    72 => {
                        { let __p = &mut n_src_db; let __t = *__p; *__p += 1; __t };
                        __state = 252;
                    }
                    73 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            __state = 75;
                        } else { __state = 74; }
                    }
                    74 => {
                        if unsafe {
                                    strcmp(z, c"brief".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 76;
                        } else { __state = 77; }
                    }
                    75 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 74;
                    }
                    76 => { brief_flag = 1; __state = 78; }
                    77 => {
                        if unsafe {
                                    strcmp(z,
                                        c"cell-size-check".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 81;
                        } else { __state = 82; }
                    }
                    78 => { quiet_flag = 1; __state = 79; }
                    79 => { verbose_flag = 1; __state = 80; }
                    80 => { e_verbosity = 0; __state = 69; }
                    81 => { cell_sz_ck_flag = 1; __state = 69; }
                    82 => {
                        if unsafe {
                                    strcmp(z, c"dbid".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 83;
                        } else { __state = 84; }
                    }
                    83 => { __state = 85; }
                    84 => {
                        if unsafe {
                                    strcmp(z, c"export-db".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 94;
                        } else { __state = 95; }
                    }
                    85 => {
                        if i >= argc - 1 { __state = 87; } else { __state = 86; }
                    }
                    86 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 88;
                    }
                    87 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 86;
                    }
                    88 => {
                        z_dot_dot =
                            unsafe {
                                    strstr(unsafe { *argv.offset(i as isize) } as *const i8,
                                        c"..".as_ptr() as *mut i8 as *const i8)
                                } as *const i8;
                        __state = 89;
                    }
                    89 => {
                        if !(z_dot_dot).is_null() {
                            __state = 90;
                        } else { __state = 91; }
                    }
                    90 => {
                        first_dbid =
                            unsafe {
                                atoi(unsafe { *argv.offset(i as isize) } as *const i8)
                            };
                        __state = 92;
                    }
                    91 => {
                        last_dbid =
                            {
                                first_dbid =
                                    integer_value(unsafe { *argv.offset(i as isize) } as
                                            *const i8);
                                first_dbid
                            };
                        __state = 69;
                    }
                    92 => {
                        if unsafe { *z_dot_dot.offset(2 as isize) } != 0 {
                            __state = 93;
                        } else { __state = 69; }
                    }
                    93 => {
                        last_dbid =
                            unsafe { atoi(unsafe { &*z_dot_dot.offset(2 as isize) }) };
                        __state = 69;
                    }
                    94 => {
                        if i >= argc - 1 { __state = 97; } else { __state = 96; }
                    }
                    95 => {
                        if unsafe {
                                        strcmp(z, c"export-sql".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe {
                                        strcmp(z, c"export-dbsql".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            __state = 98;
                        } else { __state = 99; }
                    }
                    96 => {
                        z_exp_db =
                            unsafe {
                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                            };
                        __state = 69;
                    }
                    97 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 96;
                    }
                    98 => {
                        if i >= argc - 1 { __state = 101; } else { __state = 100; }
                    }
                    99 => {
                        if unsafe {
                                    strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 102;
                        } else { __state = 103; }
                    }
                    100 => {
                        z_exp_sql =
                            unsafe {
                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                            };
                        __state = 69;
                    }
                    101 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 100;
                    }
                    102 => { show_help(); __state = 104; }
                    103 => {
                        if unsafe {
                                    strcmp(z, c"info".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 105;
                        } else { __state = 106; }
                    }
                    104 => { return Ok(()); }
                    105 => { info_flag = 1; __state = 69; }
                    106 => {
                        if unsafe {
                                    strcmp(z, c"limit-depth".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 107;
                        } else { __state = 108; }
                    }
                    107 => {
                        if i >= argc - 1 { __state = 110; } else { __state = 109; }
                    }
                    108 => {
                        if unsafe {
                                    strcmp(z, c"limit-heap".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 111;
                        } else { __state = 112; }
                    }
                    109 => {
                        depth_limit =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                        __state = 69;
                    }
                    110 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 109;
                    }
                    111 => {
                        if i >= argc - 1 { __state = 114; } else { __state = 113; }
                    }
                    112 => {
                        if unsafe {
                                    strcmp(z, c"limit-mem".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 115;
                        } else { __state = 116; }
                    }
                    113 => {
                        heap_limit =
                            integer_value(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8) as Sqlite3Int64;
                        __state = 69;
                    }
                    114 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 113;
                    }
                    115 => {
                        if i >= argc - 1 { __state = 118; } else { __state = 117; }
                    }
                    116 => {
                        if unsafe {
                                    strcmp(z, c"limit-vdbe".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 119;
                        } else { __state = 120; }
                    }
                    117 => {
                        n_mem =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                        __state = 69;
                    }
                    118 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 117;
                    }
                    119 => { vdbe_limit_flag = 1; __state = 69; }
                    120 => {
                        if unsafe {
                                    strcmp(z, c"load-sql".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 121;
                        } else { __state = 122; }
                    }
                    121 => {
                        z_ins_sql =
                            c"INSERT INTO xsql(sqltext)VALUES(CAST(readtextfile(?1) AS text))".as_ptr()
                                as *mut i8;
                        __state = 123;
                    }
                    122 => {
                        if unsafe {
                                    strcmp(z, c"load-db".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 126;
                        } else { __state = 127; }
                    }
                    123 => { i_first_ins_arg = i + 1; __state = 124; }
                    124 => { open_flags4_data = 2 | 4; __state = 125; }
                    125 => { __state = 66; }
                    126 => {
                        z_ins_sql =
                            c"INSERT INTO db(dbcontent) VALUES(readfile(?1))".as_ptr()
                                as *mut i8;
                        __state = 128;
                    }
                    127 => {
                        if unsafe {
                                    strcmp(z, c"load-dbsql".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 131;
                        } else { __state = 132; }
                    }
                    128 => { i_first_ins_arg = i + 1; __state = 129; }
                    129 => { open_flags4_data = 2 | 4; __state = 130; }
                    130 => { __state = 66; }
                    131 => {
                        z_ins_sql =
                            c"INSERT INTO xsql(sqltext)VALUES(readfile(?1))".as_ptr() as
                                *mut i8;
                        __state = 133;
                    }
                    132 => {
                        if unsafe {
                                    strcmp(z, c"m".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 137;
                        } else { __state = 138; }
                    }
                    133 => { i_first_ins_arg = i + 1; __state = 134; }
                    134 => { open_flags4_data = 2 | 4; __state = 135; }
                    135 => { db_sql_only = 1; __state = 136; }
                    136 => { __state = 66; }
                    137 => {
                        if i >= argc - 1 { __state = 140; } else { __state = 139; }
                    }
                    138 => {
                        if unsafe {
                                    strcmp(z, c"native-malloc".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 142;
                        } else { __state = 143; }
                    }
                    139 => {
                        z_msg =
                            unsafe {
                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                            };
                        __state = 141;
                    }
                    140 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 139;
                    }
                    141 => { open_flags4_data = 2 | 4; __state = 69; }
                    142 => { native_malloc = 1; __state = 69; }
                    143 => {
                        if unsafe {
                                    strcmp(z, c"native-vfs".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 144;
                        } else { __state = 145; }
                    }
                    144 => { native_flag = 1; __state = 69; }
                    145 => {
                        if unsafe {
                                    strcmp(z, c"no-recover".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 146;
                        } else { __state = 147; }
                    }
                    146 => { b_no_recover = 1; __state = 69; }
                    147 => {
                        if unsafe {
                                    strcmp(z, c"oss-fuzz".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 148;
                        } else { __state = 149; }
                    }
                    148 => { oss_fuzz = 1; __state = 69; }
                    149 => {
                        if unsafe {
                                    strcmp(z, c"prng-seed".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 150;
                        } else { __state = 151; }
                    }
                    150 => {
                        if i >= argc - 1 { __state = 153; } else { __state = 152; }
                    }
                    151 => {
                        if unsafe {
                                        strcmp(z, c"quiet".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe { strcmp(z, c"q".as_ptr() as *mut i8 as *const i8) }
                                    == 0 {
                            __state = 154;
                        } else { __state = 155; }
                    }
                    152 => {
                        g.u_random =
                            unsafe {
                                    atoi(unsafe {
                                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                            } as *const i8)
                                } as u32;
                        __state = 69;
                    }
                    153 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 152;
                    }
                    154 => { brief_flag = 0; __state = 156; }
                    155 => {
                        if unsafe {
                                    strcmp(z, c"rebuild".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 159;
                        } else { __state = 160; }
                    }
                    156 => { quiet_flag = 1; __state = 157; }
                    157 => { verbose_flag = 0; __state = 158; }
                    158 => { e_verbosity = 0; __state = 69; }
                    159 => { rebuild_flag = 1; __state = 161; }
                    160 => {
                        if unsafe {
                                    strcmp(z, c"result-trace".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 162;
                        } else { __state = 163; }
                    }
                    161 => { open_flags4_data = 2; __state = 69; }
                    162 => { run_flags |= 2; __state = 69; }
                    163 => {
                        if unsafe {
                                    strcmp(z, c"script".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 164;
                        } else { __state = 165; }
                    }
                    164 => { b_script = 1; __state = 69; }
                    165 => {
                        if unsafe {
                                    strcmp(z, c"skip".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 166;
                        } else { __state = 167; }
                    }
                    166 => {
                        if i >= argc - 1 { __state = 169; } else { __state = 168; }
                    }
                    167 => {
                        if unsafe {
                                    strcmp(z, c"slice".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 170;
                        } else { __state = 171; }
                    }
                    168 => {
                        n_skip =
                            unsafe {
                                atoi(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8)
                            };
                        __state = 69;
                    }
                    169 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 168;
                    }
                    170 => {
                        if i >= argc - 2 { __state = 173; } else { __state = 172; }
                    }
                    171 => {
                        if unsafe {
                                    strcmp(z, c"spinner".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 179;
                        } else { __state = 180; }
                    }
                    172 => {
                        i_slice_idx =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                        __state = 174;
                    }
                    173 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 172;
                    }
                    174 => {
                        i_slice_sz =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                        __state = 175;
                    }
                    175 => { brief_flag = 1; __state = 176; }
                    176 => { quiet_flag = 1; __state = 177; }
                    177 => { verbose_flag = 1; __state = 178; }
                    178 => { e_verbosity = 0; __state = 69; }
                    179 => { b_spinner = 1; __state = 69; }
                    180 => {
                        if unsafe {
                                    strcmp(z, c"sqlid".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 181;
                        } else { __state = 182; }
                    }
                    181 => { __state = 183; }
                    182 => {
                        if unsafe {
                                    strcmp(z, c"timeout".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 193;
                        } else { __state = 194; }
                    }
                    183 => {
                        if i >= argc - 1 { __state = 185; } else { __state = 184; }
                    }
                    184 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 186;
                    }
                    185 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 184;
                    }
                    186 => {
                        z_dot_dot_1 =
                            unsafe {
                                    strstr(unsafe { *argv.offset(i as isize) } as *const i8,
                                        c"..".as_ptr() as *mut i8 as *const i8)
                                } as *const i8;
                        __state = 187;
                    }
                    187 => {
                        if !(z_dot_dot_1).is_null() {
                            __state = 188;
                        } else { __state = 189; }
                    }
                    188 => {
                        first_sqlid =
                            unsafe {
                                atoi(unsafe { *argv.offset(i as isize) } as *const i8)
                            };
                        __state = 190;
                    }
                    189 => {
                        first_sqlid =
                            integer_value(unsafe { *argv.offset(i as isize) } as
                                    *const i8);
                        __state = 192;
                    }
                    190 => {
                        if unsafe { *z_dot_dot_1.offset(2 as isize) } != 0 {
                            __state = 191;
                        } else { __state = 69; }
                    }
                    191 => {
                        last_sqlid =
                            unsafe {
                                atoi(unsafe { &*z_dot_dot_1.offset(2 as isize) })
                            };
                        __state = 69;
                    }
                    192 => { last_sqlid = first_sqlid; __state = 69; }
                    193 => {
                        if i >= argc - 1 { __state = 196; } else { __state = 195; }
                    }
                    194 => {
                        if unsafe {
                                    strcmp(z, c"timeout-test".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 197;
                        } else { __state = 198; }
                    }
                    195 => {
                        i_timeout =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                        __state = 69;
                    }
                    196 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 195;
                    }
                    197 => { timeout_test = 1; __state = 199; }
                    198 => {
                        if unsafe {
                                    strcmp(z, c"timer".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 200;
                        } else { __state = 201; }
                    }
                    199 => {
                        unsafe {
                            fatal_error(c"timeout is not available on non-unix systems".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 69;
                    }
                    200 => { b_timer = 1; __state = 69; }
                    201 => {
                        if unsafe {
                                    strcmp(z, c"vdbe-debug".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 202;
                        } else { __state = 203; }
                    }
                    202 => { b_vdbe_debug = 1; __state = 69; }
                    203 => {
                        if unsafe {
                                    strcmp(z, c"verbose".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 204;
                        } else { __state = 205; }
                    }
                    204 => { brief_flag = 0; __state = 206; }
                    205 => {
                        if { n_v = number_of_v_char(z); n_v } >= 1 {
                            __state = 211;
                        } else { __state = 212; }
                    }
                    206 => { quiet_flag = 0; __state = 207; }
                    207 => {
                        {
                            let __p = &mut verbose_flag;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 208;
                    }
                    208 => {
                        {
                            let __p = &mut e_verbosity;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 209;
                    }
                    209 => {
                        if verbose_flag > 2 {
                            __state = 210;
                        } else { __state = 69; }
                    }
                    210 => { run_flags |= 1; __state = 69; }
                    211 => { quiet_flag = 0; __state = 213; }
                    212 => {
                        if unsafe {
                                    strcmp(z, c"version".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 217;
                        } else { __state = 218; }
                    }
                    213 => { verbose_flag += n_v; __state = 214; }
                    214 => { e_verbosity += n_v; __state = 215; }
                    215 => {
                        if verbose_flag > 2 {
                            __state = 216;
                        } else { __state = 69; }
                    }
                    216 => { run_flags |= 1; __state = 69; }
                    217 => { __state = 219; }
                    218 => {
                        if unsafe {
                                    strcmp(z, c"wait".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 226;
                        } else { __state = 227; }
                    }
                    219 => { __state = 220; }
                    220 => {
                        unsafe {
                            printf(c"SQLite %s %s (%d-bit)\n".as_ptr() as *mut i8 as
                                    *const i8, unsafe { sqlite3_libversion() },
                                unsafe { sqlite3_sourceid() },
                                8 * core::mem::size_of::<*mut i8>() as i32)
                        };
                        __state = 221;
                    }
                    221 => { ii = 0; __state = 223; }
                    222 => { return Ok(()); }
                    223 => {
                        if { zz = unsafe { sqlite3_compileoption_get(ii) }; zz } !=
                                core::ptr::null() {
                            __state = 224;
                        } else { __state = 222; }
                    }
                    224 => {
                        unsafe {
                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8, zz)
                        };
                        __state = 225;
                    }
                    225 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 223;
                    }
                    226 => { __state = 228; }
                    227 => {
                        if unsafe {
                                    strcmp(z, c"is-dbsql".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 242;
                        } else { __state = 243; }
                    }
                    228 => {
                        if i >= argc - 1 { __state = 230; } else { __state = 229; }
                    }
                    229 => {
                        i_delay =
                            integer_value(unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8);
                        __state = 231;
                    }
                    230 => {
                        unsafe {
                            fatal_error(c"missing arguments on %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 229;
                    }
                    231 => {
                        unsafe {
                            printf(c"Waiting %d seconds:".as_ptr() as *mut i8 as
                                    *const i8, i_delay)
                        };
                        __state = 232;
                    }
                    232 => { unsafe { fflush(__stdoutp) }; __state = 233; }
                    233 => {
                        if 1 != 0 { __state = 235; } else { __state = 234; }
                    }
                    234 => {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        __state = 241;
                    }
                    235 => { unsafe { sqlite3_sleep(1000) }; __state = 236; }
                    236 => {
                        { let __p = &mut i_delay; let __t = *__p; *__p -= 1; __t };
                        __state = 237;
                    }
                    237 => {
                        if i_delay <= 0 { __state = 239; } else { __state = 238; }
                    }
                    238 => {
                        unsafe {
                            printf(c" %d".as_ptr() as *mut i8 as *const i8, i_delay)
                        };
                        __state = 240;
                    }
                    239 => { __state = 234; }
                    240 => { unsafe { fflush(__stdoutp) }; __state = 233; }
                    241 => { unsafe { fflush(__stdoutp) }; __state = 69; }
                    242 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 244;
                    }
                    243 => {
                        unsafe {
                            fatal_error(c"unknown option: %s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 69;
                    }
                    244 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 246;
                    }
                    245 => { unsafe { exit(0) }; __state = 69; }
                    246 => {
                        if i < argc { __state = 247; } else { __state = 245; }
                    }
                    247 => { __state = 249; }
                    248 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 246;
                    }
                    249 => {
                        a_data =
                            read_file(unsafe { *argv.offset(i as isize) } as *const i8,
                                &mut n_data);
                        __state = 250;
                    }
                    250 => {
                        unsafe {
                            printf(c"%d %s\n".as_ptr() as *mut i8 as *const i8,
                                is_db_sql(a_data as *mut u8 as *const u8, n_data as i32),
                                unsafe { *argv.offset(i as isize) })
                        };
                        __state = 251;
                    }
                    251 => {
                        unsafe { sqlite3_free(a_data as *mut ()) };
                        __state = 248;
                    }
                    252 => {
                        az_src_db =
                            safe_realloc(az_src_db as *mut (),
                                    (n_src_db as u64 * core::mem::size_of::<*mut i8>() as u64)
                                        as i32) as *mut *mut i8;
                        __state = 253;
                    }
                    253 => {
                        unsafe {
                            *az_src_db.offset((n_src_db - 1) as isize) =
                                unsafe { *argv.offset(i as isize) }
                        };
                        __state = 69;
                    }
                    254 => {
                        if n_src_db > 1 { __state = 257; } else { __state = 256; }
                    }
                    255 => {
                        unsafe {
                            fatal_error(c"no source database specified".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 254;
                    }
                    256 => {
                        if i_slice_sz <= i_slice_idx || i_slice_sz <= 0 ||
                                i_slice_idx < 0 {
                            __state = 262;
                        } else { __state = 261; }
                    }
                    257 => {
                        if !(z_msg).is_null() {
                            __state = 259;
                        } else { __state = 258; }
                    }
                    258 => {
                        if !(z_ins_sql).is_null() {
                            __state = 260;
                        } else { __state = 256; }
                    }
                    259 => {
                        unsafe {
                            fatal_error(c"cannot change the description of more than one database".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 258;
                    }
                    260 => {
                        unsafe {
                            fatal_error(c"cannot import into more than one database".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 256;
                    }
                    261 => { i_src_db = 0; __state = 264; }
                    262 => {
                        i_slice_sz = { i_slice_idx = 0; i_slice_idx };
                        __state = 261;
                    }
                    263 => {
                        if (quiet_flag == 0) as i32 != 0 &&
                                (b_script == 0) as i32 != 0 {
                            __state = 605;
                        } else { __state = 604; }
                    }
                    264 => {
                        if i_src_db < n_src_db {
                            __state = 265;
                        } else { __state = 263; }
                    }
                    265 => {
                        z_raw_data = core::ptr::null_mut();
                        __state = 267;
                    }
                    266 => {
                        { let __p = &mut i_src_db; let __t = *__p; *__p += 1; __t };
                        __state = 264;
                    }
                    267 => { n_raw_data = 0 as i64; __state = 268; }
                    268 => {
                        g.z_db_file =
                            unsafe { *az_src_db.offset(i_src_db as isize) } as
                                *const i8;
                        __state = 269;
                    }
                    269 => {
                        rc =
                            unsafe {
                                sqlite3_open_v2(unsafe {
                                            *az_src_db.offset(i_src_db as isize)
                                        } as *const i8, &mut db, open_flags4_data,
                                    unsafe { (*p_dflt_vfs).z_name })
                            };
                        __state = 270;
                    }
                    270 => {
                        if rc == 0 { __state = 272; } else { __state = 271; }
                    }
                    271 => {
                        if rc != 0 { __state = 274; } else { __state = 273; }
                    }
                    272 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db,
                                    c"SELECT count(*) FROM sqlite_schema".as_ptr() as *mut i8 as
                                        *const i8, None, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        __state = 271;
                    }
                    273 => {
                        if info_flag != 0 { __state = 280; } else { __state = 279; }
                    }
                    274 => { unsafe { sqlite3_close(db) }; __state = 275; }
                    275 => {
                        z_raw_data =
                            read_file(unsafe { *az_src_db.offset(i_src_db as isize) } as
                                    *const i8, &mut n_raw_data);
                        __state = 276;
                    }
                    276 => {
                        if z_raw_data == core::ptr::null_mut() {
                            __state = 278;
                        } else { __state = 277; }
                    }
                    277 => {
                        unsafe {
                            sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                                &mut db)
                        };
                        __state = 273;
                    }
                    278 => {
                        unsafe {
                            fatal_error(c"input file \"%s\" is not recognized\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *az_src_db.offset(i_src_db as isize) })
                        };
                        __state = 277;
                    }
                    279 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db,
                                    c"CREATE TABLE IF NOT EXISTS db(\n  dbid INTEGER PRIMARY KEY, -- database id\n  dbcontent BLOB            -- database disk file image\n);\nCREATE TABLE IF NOT EXISTS xsql(\n  sqlid INTEGER PRIMARY KEY,   -- SQL script id\n  sqltext TEXT                 -- Text of SQL statements to run\n);CREATE TABLE IF NOT EXISTS readme(\n  msg TEXT -- Human-readable description of this file\n);".as_ptr()
                                            as *mut i8 as *const i8, None, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        __state = 303;
                    }
                    280 => { __state = 281; }
                    281 => {
                        z_db_name = unsafe { *az_src_db.offset(i_src_db as isize) };
                        __state = 282;
                    }
                    282 => {
                        i = unsafe { strlen(z_db_name as *const i8) } as i32 - 1;
                        __state = 283;
                    }
                    283 => {
                        if i > 0 &&
                                    unsafe { *z_db_name.offset((i - 1) as isize) } as i32 !=
                                        '/' as i32 &&
                                unsafe { *z_db_name.offset((i - 1) as isize) } as i32 !=
                                    '\\' as i32 {
                            __state = 285;
                        } else { __state = 284; }
                    }
                    284 => {
                        {
                            let __n = i;
                            let __p = &mut z_db_name;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 286;
                    }
                    285 => {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        __state = 283;
                    }
                    286 => {
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT msg FROM readme".as_ptr() as *mut i8 as *const i8,
                                -1, &mut p_stmt, core::ptr::null_mut())
                        };
                        __state = 287;
                    }
                    287 => {
                        if !(p_stmt).is_null() &&
                                unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 289;
                        } else { __state = 290; }
                    }
                    288 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 291;
                    }
                    289 => {
                        unsafe {
                            printf(c"%s: %s".as_ptr() as *mut i8 as *const i8,
                                z_db_name, unsafe { sqlite3_column_text(p_stmt, 0) })
                        };
                        __state = 288;
                    }
                    290 => {
                        unsafe {
                            printf(c"%s: (empty \"readme\")".as_ptr() as *mut i8 as
                                    *const i8, z_db_name)
                        };
                        __state = 288;
                    }
                    291 => {
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT count(*) FROM db".as_ptr() as *mut i8 as *const i8,
                                -1, &mut p_stmt, core::ptr::null_mut())
                        };
                        __state = 292;
                    }
                    292 => {
                        if !(p_stmt).is_null() &&
                                    unsafe { sqlite3_step(p_stmt) } == 100 &&
                                { n = unsafe { sqlite3_column_int(p_stmt, 0) }; n } > 0 {
                            __state = 294;
                        } else { __state = 293; }
                    }
                    293 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 295;
                    }
                    294 => {
                        unsafe {
                            printf(c" - %d DBs".as_ptr() as *mut i8 as *const i8, n)
                        };
                        __state = 293;
                    }
                    295 => {
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT count(*) FROM xsql".as_ptr() as *mut i8 as
                                    *const i8, -1, &mut p_stmt, core::ptr::null_mut())
                        };
                        __state = 296;
                    }
                    296 => {
                        if !(p_stmt).is_null() &&
                                    unsafe { sqlite3_step(p_stmt) } == 100 &&
                                { n = unsafe { sqlite3_column_int(p_stmt, 0) }; n } > 0 {
                            __state = 298;
                        } else { __state = 297; }
                    }
                    297 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 299;
                    }
                    298 => {
                        unsafe {
                            printf(c" - %d scripts".as_ptr() as *mut i8 as *const i8, n)
                        };
                        __state = 297;
                    }
                    299 => {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        __state = 300;
                    }
                    300 => { unsafe { sqlite3_close(db) }; __state = 301; }
                    301 => {
                        unsafe { sqlite3_free(z_raw_data as *mut ()) };
                        __state = 302;
                    }
                    302 => { __state = 266; }
                    303 => {
                        if rc != 0 { __state = 305; } else { __state = 304; }
                    }
                    304 => {
                        if !(z_msg).is_null() {
                            __state = 307;
                        } else { __state = 306; }
                    }
                    305 => {
                        unsafe {
                            fatal_error(c"cannot create schema: %s".as_ptr() as *mut i8
                                    as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 304;
                    }
                    306 => {
                        if !(z_raw_data).is_null() {
                            __state = 314;
                        } else { __state = 313; }
                    }
                    307 => { __state = 308; }
                    308 => {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"DELETE FROM readme; INSERT INTO readme(msg) VALUES(%Q)".as_ptr()
                                            as *mut i8 as *const i8, z_msg)
                            };
                        __state = 309;
                    }
                    309 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db, z_sql as *const i8, None,
                                    core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        __state = 310;
                    }
                    310 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 311;
                    }
                    311 => {
                        if rc != 0 { __state = 312; } else { __state = 306; }
                    }
                    312 => {
                        unsafe {
                            fatal_error(c"cannot change description: %s".as_ptr() as
                                        *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 306;
                    }
                    313 => { oss_fuzz_this_db = oss_fuzz; __state = 328; }
                    314 => {
                        z_ins_sql =
                            c"INSERT INTO xsql(sqltext) VALUES(?1)".as_ptr() as *mut i8;
                        __state = 315;
                    }
                    315 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db, z_ins_sql as *const i8, -1,
                                    &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 316;
                    }
                    316 => {
                        if rc != 0 { __state = 318; } else { __state = 317; }
                    }
                    317 => {
                        unsafe {
                            sqlite3_bind_text(p_stmt, 1, z_raw_data as *const i8,
                                n_raw_data as i32, None)
                        };
                        __state = 319;
                    }
                    318 => {
                        unsafe {
                            fatal_error(c"cannot prepare statement [%s]: %s".as_ptr() as
                                        *mut i8 as *const i8, z_ins_sql,
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 317;
                    }
                    319 => { unsafe { sqlite3_step(p_stmt) }; __state = 320; }
                    320 => {
                        rc = unsafe { sqlite3_reset(p_stmt) };
                        __state = 321;
                    }
                    321 => {
                        if rc != 0 { __state = 323; } else { __state = 322; }
                    }
                    322 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 324;
                    }
                    323 => {
                        unsafe {
                            fatal_error(c"insert failed for %s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 322;
                    }
                    324 => { rebuild_database(db, db_sql_only); __state = 325; }
                    325 => { z_ins_sql = core::ptr::null_mut(); __state = 326; }
                    326 => {
                        unsafe { sqlite3_free(z_raw_data as *mut ()) };
                        __state = 327;
                    }
                    327 => {
                        z_raw_data = core::ptr::null_mut();
                        __state = 313;
                    }
                    328 => {
                        if unsafe {
                                    sqlite3_table_column_metadata(db, core::ptr::null(),
                                        c"config".as_ptr() as *mut i8 as *const i8,
                                        core::ptr::null(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                } == 0 {
                            __state = 330;
                        } else { __state = 329; }
                    }
                    329 => {
                        if !(z_ins_sql).is_null() {
                            __state = 347;
                        } else { __state = 346; }
                    }
                    330 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db,
                                    c"SELECT name, value FROM config".as_ptr() as *mut i8 as
                                        *const i8, -1, &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 331;
                    }
                    331 => {
                        if rc != 0 { __state = 333; } else { __state = 332; }
                    }
                    332 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 335;
                        } else { __state = 334; }
                    }
                    333 => {
                        unsafe {
                            fatal_error(c"cannot prepare query of CONFIG table: %s".as_ptr()
                                        as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 332;
                    }
                    334 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 329;
                    }
                    335 => {
                        z_name =
                            unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
                        __state = 336;
                    }
                    336 => {
                        if z_name == core::ptr::null() {
                            __state = 338;
                        } else { __state = 337; }
                    }
                    337 => {
                        if unsafe {
                                    strcmp(z_name, c"oss-fuzz".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 340;
                        } else { __state = 339; }
                    }
                    338 => { __state = 332; }
                    339 => {
                        if unsafe {
                                    strcmp(z_name,
                                        c"limit-mem".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 343;
                        } else { __state = 332; }
                    }
                    340 => {
                        oss_fuzz_this_db = unsafe { sqlite3_column_int(p_stmt, 1) };
                        __state = 341;
                    }
                    341 => {
                        if verbose_flag > 1 {
                            __state = 342;
                        } else { __state = 339; }
                    }
                    342 => {
                        unsafe {
                            printf(c"Config: oss-fuzz=%d\n".as_ptr() as *mut i8 as
                                    *const i8, oss_fuzz_this_db)
                        };
                        __state = 339;
                    }
                    343 => {
                        n_mem_this_db = unsafe { sqlite3_column_int(p_stmt, 1) };
                        __state = 344;
                    }
                    344 => {
                        if verbose_flag > 1 {
                            __state = 345;
                        } else { __state = 332; }
                    }
                    345 => {
                        unsafe {
                            printf(c"Config: limit-mem=%d\n".as_ptr() as *mut i8 as
                                    *const i8, n_mem_this_db)
                        };
                        __state = 332;
                    }
                    346 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db,
                                    c"PRAGMA query_only=1;".as_ptr() as *mut i8 as *const i8,
                                    None, core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        __state = 386;
                    }
                    347 => {
                        unsafe {
                            sqlite3_create_function(db,
                                c"readfile".as_ptr() as *mut i8 as *const i8, 1, 1,
                                core::ptr::null_mut(), Some(readfile_func), None, None)
                        };
                        __state = 348;
                    }
                    348 => {
                        unsafe {
                            sqlite3_create_function(db,
                                c"readtextfile".as_ptr() as *mut i8 as *const i8, 1, 1,
                                core::ptr::null_mut(), Some(readtextfile_func), None, None)
                        };
                        __state = 349;
                    }
                    349 => {
                        unsafe {
                            sqlite3_create_function(db,
                                c"isdbsql".as_ptr() as *mut i8 as *const i8, 1, 1,
                                core::ptr::null_mut(), Some(is_db_sql_func), None, None)
                        };
                        __state = 350;
                    }
                    350 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db, z_ins_sql as *const i8, -1,
                                    &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 351;
                    }
                    351 => {
                        if rc != 0 { __state = 353; } else { __state = 352; }
                    }
                    352 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db, c"BEGIN".as_ptr() as *mut i8 as *const i8,
                                    None, core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        __state = 354;
                    }
                    353 => {
                        unsafe {
                            fatal_error(c"cannot prepare statement [%s]: %s".as_ptr() as
                                        *mut i8 as *const i8, z_ins_sql,
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 352;
                    }
                    354 => {
                        if rc != 0 { __state = 356; } else { __state = 355; }
                    }
                    355 => { i = i_first_ins_arg; __state = 358; }
                    356 => {
                        unsafe {
                            fatal_error(c"cannot start a transaction".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 355;
                    }
                    357 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 380;
                    }
                    358 => {
                        if i < argc { __state = 359; } else { __state = 357; }
                    }
                    359 => {
                        if unsafe {
                                    strcmp(unsafe { *argv.offset(i as isize) } as *const i8,
                                        c"-".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 361;
                        } else { __state = 362; }
                    }
                    360 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 358;
                    }
                    361 => { __state = 363; }
                    362 => {
                        unsafe {
                            sqlite3_bind_text(p_stmt, 1,
                                unsafe { *argv.offset(i as isize) } as *const i8, -1, None)
                        };
                        __state = 374;
                    }
                    363 => {
                        if rc == 0 &&
                                unsafe {
                                        fgets(&raw mut z_line[0 as usize] as *mut i8,
                                            core::mem::size_of::<[i8; 2000]>() as i32, __stdinp)
                                    } != core::ptr::null_mut() {
                            __state = 364;
                        } else { __state = 360; }
                    }
                    364 => {
                        kk =
                            unsafe {
                                strlen(&raw mut z_line[0 as usize] as *mut i8 as *const i8)
                            };
                        __state = 365;
                    }
                    365 => {
                        if kk > 0 as u64 &&
                                z_line[(kk - 1 as u64) as usize] as i32 <= ' ' as i32 {
                            __state = 367;
                        } else { __state = 366; }
                    }
                    366 => {
                        unsafe {
                            sqlite3_bind_text(p_stmt, 1,
                                &raw mut z_line[0 as usize] as *mut i8 as *const i8,
                                kk as i32, None)
                        };
                        __state = 368;
                    }
                    367 => {
                        { let __p = &mut kk; let __t = *__p; *__p -= 1; __t };
                        __state = 365;
                    }
                    368 => {
                        if verbose_flag > 1 {
                            __state = 370;
                        } else { __state = 369; }
                    }
                    369 => { unsafe { sqlite3_step(p_stmt) }; __state = 371; }
                    370 => {
                        unsafe {
                            printf(c"loading %.*s\n".as_ptr() as *mut i8 as *const i8,
                                kk as i32, &raw mut z_line[0 as usize] as *mut i8)
                        };
                        __state = 369;
                    }
                    371 => {
                        rc = unsafe { sqlite3_reset(p_stmt) };
                        __state = 372;
                    }
                    372 => {
                        if rc != 0 { __state = 373; } else { __state = 363; }
                    }
                    373 => {
                        unsafe {
                            fatal_error(c"insert failed for %s".as_ptr() as *mut i8 as
                                    *const i8, &raw mut z_line[0 as usize] as *mut i8)
                        };
                        __state = 363;
                    }
                    374 => {
                        if verbose_flag > 1 {
                            __state = 376;
                        } else { __state = 375; }
                    }
                    375 => { unsafe { sqlite3_step(p_stmt) }; __state = 377; }
                    376 => {
                        unsafe {
                            printf(c"loading %s\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { *argv.offset(i as isize) })
                        };
                        __state = 375;
                    }
                    377 => {
                        rc = unsafe { sqlite3_reset(p_stmt) };
                        __state = 378;
                    }
                    378 => {
                        if rc != 0 { __state = 379; } else { __state = 360; }
                    }
                    379 => {
                        unsafe {
                            fatal_error(c"insert failed for %s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        __state = 360;
                    }
                    380 => {
                        rc =
                            unsafe {
                                sqlite3_exec(db, c"COMMIT".as_ptr() as *mut i8 as *const i8,
                                    None, core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        __state = 381;
                    }
                    381 => {
                        if rc != 0 { __state = 383; } else { __state = 382; }
                    }
                    382 => { rebuild_database(db, db_sql_only); __state = 384; }
                    383 => {
                        unsafe {
                            fatal_error(c"cannot commit the transaction: %s".as_ptr() as
                                        *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 382;
                    }
                    384 => { unsafe { sqlite3_close(db) }; __state = 385; }
                    385 => { return Ok(()); }
                    386 => {
                        if rc != 0 { __state = 388; } else { __state = 387; }
                    }
                    387 => {
                        if z_exp_db != core::ptr::null_mut() ||
                                z_exp_sql != core::ptr::null_mut() {
                            __state = 390;
                        } else { __state = 389; }
                    }
                    388 => {
                        unsafe {
                            fatal_error(c"cannot set database to query-only".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 387;
                    }
                    389 => {
                        blob_list_load_from_db(db,
                            c"SELECT sqlid, sqltext FROM xsql".as_ptr() as *mut i8 as
                                *const i8, first_sqlid, last_sqlid, &mut g.n_sql,
                            &mut g.p_first_sql);
                        __state = 415;
                    }
                    390 => {
                        unsafe {
                            sqlite3_create_function(db,
                                c"writefile".as_ptr() as *mut i8 as *const i8, 2, 1,
                                core::ptr::null_mut(), Some(writefile_func), None, None)
                        };
                        __state = 391;
                    }
                    391 => {
                        if z_exp_db != core::ptr::null_mut() {
                            __state = 393;
                        } else { __state = 392; }
                    }
                    392 => {
                        if z_exp_sql != core::ptr::null_mut() {
                            __state = 404;
                        } else { __state = 403; }
                    }
                    393 => {
                        z_ex_db =
                            c"SELECT writefile(printf(\'%s/db%06d.db\',?1,dbid),dbcontent),       dbid, printf(\'%s/db%06d.db\',?1,dbid), length(dbcontent)  FROM db WHERE dbid BETWEEN ?2 AND ?3;".as_ptr()
                                    as *mut i8 as *const i8;
                        __state = 394;
                    }
                    394 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db, z_ex_db, -1, &mut p_stmt,
                                    core::ptr::null_mut())
                            };
                        __state = 395;
                    }
                    395 => {
                        if rc != 0 { __state = 397; } else { __state = 396; }
                    }
                    396 => {
                        unsafe {
                            sqlite3_bind_text64(p_stmt, 1, z_exp_db as *const i8,
                                unsafe { strlen(z_exp_db as *const i8) }, None, 1 as u8)
                        };
                        __state = 398;
                    }
                    397 => {
                        unsafe {
                            fatal_error(c"cannot prepare statement [%s]: %s".as_ptr() as
                                        *mut i8 as *const i8, z_ex_db,
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 396;
                    }
                    398 => {
                        unsafe { sqlite3_bind_int(p_stmt, 2, first_dbid) };
                        __state = 399;
                    }
                    399 => {
                        unsafe { sqlite3_bind_int(p_stmt, 3, last_dbid) };
                        __state = 400;
                    }
                    400 => {
                        if unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 402;
                        } else { __state = 401; }
                    }
                    401 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 392;
                    }
                    402 => {
                        unsafe {
                            printf(c"write db-%d (%d bytes) into %s\n".as_ptr() as
                                        *mut i8 as *const i8,
                                unsafe { sqlite3_column_int(p_stmt, 1) },
                                unsafe { sqlite3_column_int(p_stmt, 3) },
                                unsafe { sqlite3_column_text(p_stmt, 2) })
                        };
                        __state = 400;
                    }
                    403 => { unsafe { sqlite3_close(db) }; __state = 414; }
                    404 => {
                        z_ex_sql =
                            c"SELECT writefile(printf(\'%s/sql%06d.txt\',?1,sqlid),sqltext),       sqlid, printf(\'%s/sql%06d.txt\',?1,sqlid), length(sqltext)  FROM xsql WHERE sqlid BETWEEN ?2 AND ?3;".as_ptr()
                                    as *mut i8 as *const i8;
                        __state = 405;
                    }
                    405 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2(db, z_ex_sql, -1, &mut p_stmt,
                                    core::ptr::null_mut())
                            };
                        __state = 406;
                    }
                    406 => {
                        if rc != 0 { __state = 408; } else { __state = 407; }
                    }
                    407 => {
                        unsafe {
                            sqlite3_bind_text64(p_stmt, 1, z_exp_sql as *const i8,
                                unsafe { strlen(z_exp_sql as *const i8) }, None, 1 as u8)
                        };
                        __state = 409;
                    }
                    408 => {
                        unsafe {
                            fatal_error(c"cannot prepare statement [%s]: %s".as_ptr() as
                                        *mut i8 as *const i8, z_ex_sql,
                                unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 407;
                    }
                    409 => {
                        unsafe { sqlite3_bind_int(p_stmt, 2, first_sqlid) };
                        __state = 410;
                    }
                    410 => {
                        unsafe { sqlite3_bind_int(p_stmt, 3, last_sqlid) };
                        __state = 411;
                    }
                    411 => {
                        if unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 413;
                        } else { __state = 412; }
                    }
                    412 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 403;
                    }
                    413 => {
                        unsafe {
                            printf(c"write sql-%d (%d bytes) into %s\n".as_ptr() as
                                        *mut i8 as *const i8,
                                unsafe { sqlite3_column_int(p_stmt, 1) },
                                unsafe { sqlite3_column_int(p_stmt, 3) },
                                unsafe { sqlite3_column_text(p_stmt, 2) })
                        };
                        __state = 411;
                    }
                    414 => { return Ok(()); }
                    415 => {
                        if g.n_sql == 0 { __state = 417; } else { __state = 416; }
                    }
                    416 => {
                        blob_list_load_from_db(db,
                            c"SELECT dbid, dbcontent FROM db".as_ptr() as *mut i8 as
                                *const i8, first_dbid, last_dbid, &mut g.n_db,
                            &mut g.p_first_db);
                        __state = 418;
                    }
                    417 => {
                        unsafe {
                            fatal_error(c"need at least one SQL script".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 416;
                    }
                    418 => {
                        if g.n_db == 0 { __state = 420; } else { __state = 419; }
                    }
                    419 => {
                        if (quiet_flag == 0) as i32 != 0 &&
                                (b_script == 0) as i32 != 0 {
                            __state = 427;
                        } else { __state = 426; }
                    }
                    420 => {
                        g.p_first_db =
                            safe_realloc(core::ptr::null_mut(),
                                    core::mem::size_of::<Blob>() as i32) as *mut Blob;
                        __state = 421;
                    }
                    421 => {
                        unsafe {
                            memset(g.p_first_db as *mut (), 0,
                                core::mem::size_of::<Blob>() as u64)
                        };
                        __state = 422;
                    }
                    422 => { unsafe { (*g.p_first_db).id = 1 }; __state = 423; }
                    423 => {
                        unsafe { (*g.p_first_db).seq = 0 };
                        __state = 424;
                    }
                    424 => { g.n_db = 1; __state = 425; }
                    425 => { sql_fuzz = 1; __state = 419; }
                    426 => {
                        if rebuild_flag != 0 {
                            __state = 438;
                        } else { __state = 437; }
                    }
                    427 => {
                        z_db_name = unsafe { *az_src_db.offset(i_src_db as isize) };
                        __state = 428;
                    }
                    428 => {
                        i = unsafe { strlen(z_db_name as *const i8) } as i32 - 1;
                        __state = 429;
                    }
                    429 => {
                        if i > 0 &&
                                    unsafe { *z_db_name.offset((i - 1) as isize) } as i32 !=
                                        '/' as i32 &&
                                unsafe { *z_db_name.offset((i - 1) as isize) } as i32 !=
                                    '\\' as i32 {
                            __state = 431;
                        } else { __state = 430; }
                    }
                    430 => {
                        {
                            let __n = i;
                            let __p = &mut z_db_name;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 432;
                    }
                    431 => {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        __state = 429;
                    }
                    432 => {
                        if verbose_flag != 0 {
                            __state = 433;
                        } else { __state = 426; }
                    }
                    433 => {
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT msg FROM readme".as_ptr() as *mut i8 as *const i8,
                                -1, &mut p_stmt, core::ptr::null_mut())
                        };
                        __state = 434;
                    }
                    434 => {
                        if !(p_stmt).is_null() &&
                                unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 436;
                        } else { __state = 435; }
                    }
                    435 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 426;
                    }
                    436 => {
                        unsafe {
                            printf(c"%s: %s\n".as_ptr() as *mut i8 as *const i8,
                                z_db_name, unsafe { sqlite3_column_text(p_stmt, 0) })
                        };
                        __state = 435;
                    }
                    437 => { unsafe { sqlite3_close(db) }; __state = 444; }
                    438 => {
                        if (quiet_flag == 0) as i32 != 0 {
                            __state = 440;
                        } else { __state = 439; }
                    }
                    439 => { rebuild_database(db, 0); __state = 442; }
                    440 => {
                        unsafe {
                            printf(c"%s: rebuilding... ".as_ptr() as *mut i8 as
                                    *const i8, z_db_name)
                        };
                        __state = 441;
                    }
                    441 => { unsafe { fflush(__stdoutp) }; __state = 439; }
                    442 => {
                        if (quiet_flag == 0) as i32 != 0 {
                            __state = 443;
                        } else { __state = 437; }
                    }
                    443 => {
                        unsafe {
                            printf(c"done\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 437;
                    }
                    444 => {
                        if unsafe { sqlite3_memory_used() } > 0 as i64 {
                            __state = 446;
                        } else { __state = 445; }
                    }
                    445 => { unsafe { sqlite3_shutdown() }; __state = 447; }
                    446 => {
                        unsafe {
                            fatal_error(c"SQLite has memory in use before the start of testing".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 445;
                    }
                    447 => {
                        if n_mem_this_db > 0 && n_mem == 0 {
                            __state = 449;
                        } else { __state = 450; }
                    }
                    448 => {
                        if native_malloc != 0 {
                            __state = 457;
                        } else { __state = 456; }
                    }
                    449 => {
                        if (native_malloc == 0) as i32 != 0 {
                            __state = 451;
                        } else { __state = 452; }
                    }
                    450 => {
                        unsafe { sqlite3_hard_heap_limit64(0 as Sqlite3Int64) };
                        __state = 448;
                    }
                    451 => {
                        p_heap = unsafe { realloc(p_heap, n_mem_this_db as u64) };
                        __state = 453;
                    }
                    452 => {
                        unsafe {
                            sqlite3_hard_heap_limit64(n_mem_this_db as Sqlite3Int64)
                        };
                        __state = 448;
                    }
                    453 => {
                        if p_heap == core::ptr::null_mut() {
                            __state = 455;
                        } else { __state = 454; }
                    }
                    454 => {
                        unsafe { sqlite3_config(8, p_heap, n_mem_this_db, 128) };
                        __state = 448;
                    }
                    455 => {
                        unsafe {
                            fatal_error(c"failed to allocate %d bytes of heap memory".as_ptr()
                                        as *mut i8 as *const i8, n_mem)
                        };
                        __state = 454;
                    }
                    456 => { format_vfs(); __state = 458; }
                    457 => {
                        unsafe { sqlite3_config(13, 0, 0) };
                        __state = 456;
                    }
                    458 => {
                        if verbose_flag < 2 && (quiet_flag == 0) as i32 != 0 &&
                                    (b_spinner == 0) as i32 != 0 && (b_script == 0) as i32 != 0
                            {
                            __state = 460;
                        } else { __state = 459; }
                    }
                    459 => { p_sql = g.p_first_sql; __state = 462; }
                    460 => {
                        unsafe {
                            printf(c"%s:".as_ptr() as *mut i8 as *const i8, z_db_name)
                        };
                        __state = 459;
                    }
                    461 => {
                        if brief_flag != 0 {
                            __state = 590;
                        } else { __state = 591; }
                    }
                    462 => {
                        if !(p_sql).is_null() {
                            __state = 463;
                        } else { __state = 461; }
                    }
                    463 => { tm_start = time_of_day(); __state = 465; }
                    464 => {
                        p_sql = unsafe { (*p_sql).p_next };
                        __state = 462;
                    }
                    465 => {
                        if is_db_sql(unsafe { (*p_sql).a.as_ptr() } as *mut u8 as
                                        *const u8, unsafe { (*p_sql).sz }) != 0 {
                            __state = 467;
                        } else { __state = 466; }
                    }
                    466 => { p_db = g.p_first_db; __state = 512; }
                    467 => {
                        if i_slice_sz > 0 && n_test % i_slice_sz != i_slice_idx {
                            __state = 469;
                        } else { __state = 468; }
                    }
                    468 => {
                        unsafe {
                            sqlite3_snprintf(core::mem::size_of::<[i8; 100]>() as i32,
                                &raw mut g.z_test_name[0 as usize] as *mut i8,
                                c"sqlid=%d".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_sql).id })
                        };
                        __state = 472;
                    }
                    469 => {
                        { let __p = &mut n_test; let __t = *__p; *__p += 1; __t };
                        __state = 470;
                    }
                    470 => {
                        {
                            let __p = &mut n_slice_skip;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 471;
                    }
                    471 => { __state = 464; }
                    472 => {
                        if b_script != 0 { __state = 474; } else { __state = 475; }
                    }
                    473 => {
                        if n_skip > 0 { __state = 504; } else { __state = 505; }
                    }
                    474 => { __state = 473; }
                    475 => {
                        if b_spinner != 0 { __state = 476; } else { __state = 477; }
                    }
                    476 => { n_total = g.n_sql; __state = 478; }
                    477 => {
                        if verbose_flag > 1 {
                            __state = 493;
                        } else { __state = 494; }
                    }
                    478 => { idx = unsafe { (*p_sql).seq }; __state = 479; }
                    479 => {
                        if n_src_db == 1 && n_total > idx && idx >= 20 {
                            __state = 481;
                        } else { __state = 482; }
                    }
                    480 => { unsafe { fflush(__stdoutp) }; __state = 473; }
                    481 => {
                        i_to_go =
                            ((time_of_day() - i_begin) * (n_total - idx) as Sqlite3Int64
                                    / (idx * 1000) as Sqlite3Int64) as i32;
                        __state = 483;
                    }
                    482 => {
                        unsafe {
                            printf(c"\r%s: %d/%d           ".as_ptr() as *mut i8 as
                                    *const i8, z_db_name, idx, n_total)
                        };
                        __state = 480;
                    }
                    483 => { __state = 484; }
                    484 => {
                        if idx == 20 { __state = 486; } else { __state = 487; }
                    }
                    485 => { hr = i_est_time / 3600; __state = 488; }
                    486 => { i_est_time = i_to_go; __state = 485; }
                    487 => {
                        i_est_time = (i_to_go + 7 * i_est_time) / 8;
                        __state = 485;
                    }
                    488 => { min = i_est_time / 60 % 60; __state = 489; }
                    489 => { sec = i_est_time % 60; __state = 490; }
                    490 => {
                        if hr > 0 { __state = 491; } else { __state = 492; }
                    }
                    491 => {
                        unsafe {
                            printf(c"\r%s: %d/%d ETC %d:%02d:%02d  ".as_ptr() as *mut i8
                                    as *const i8, z_db_name, idx, n_total, hr, min, sec)
                        };
                        __state = 480;
                    }
                    492 => {
                        unsafe {
                            printf(c"\r%s: %d/%d ETC %02d:%02d    ".as_ptr() as *mut i8
                                    as *const i8, z_db_name, idx, n_total, min, sec)
                        };
                        __state = 480;
                    }
                    493 => {
                        unsafe {
                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                                &raw mut g.z_test_name[0 as usize] as *mut i8)
                        };
                        __state = 495;
                    }
                    494 => {
                        if (quiet_flag == 0) as i32 != 0 {
                            __state = 496;
                        } else { __state = 473; }
                    }
                    495 => { unsafe { fflush(__stdoutp) }; __state = 473; }
                    496 => { __state = 497; }
                    497 => { idx__1 = unsafe { (*p_sql).seq }; __state = 498; }
                    498 => { amt = idx__1 * 10 / g.n_sql; __state = 499; }
                    499 => {
                        if amt != prev_amt {
                            __state = 500;
                        } else { __state = 473; }
                    }
                    500 => {
                        unsafe {
                            printf(c" %d%%".as_ptr() as *mut i8 as *const i8, amt * 10)
                        };
                        __state = 501;
                    }
                    501 => { unsafe { fflush(__stdoutp) }; __state = 502; }
                    502 => { prev_amt = amt; __state = 473; }
                    503 => {
                        { let __p = &mut n_test; let __t = *__p; *__p += 1; __t };
                        __state = 506;
                    }
                    504 => {
                        { let __p = &mut n_skip; let __t = *__p; *__p -= 1; __t };
                        __state = 503;
                    }
                    505 => {
                        run_combined_db_sql_input(unsafe { (*p_sql).a.as_ptr() } as
                                    *mut u8 as *const u8, unsafe { (*p_sql).sz } as u64,
                            i_timeout, b_script, unsafe { (*p_sql).id });
                        __state = 503;
                    }
                    506 => {
                        if b_timer != 0 && (b_script == 0) as i32 != 0 {
                            __state = 508;
                        } else { __state = 507; }
                    }
                    507 => {
                        g.z_test_name[0 as usize] = 0 as i8;
                        __state = 510;
                    }
                    508 => { tm_end = time_of_day(); __state = 509; }
                    509 => {
                        unsafe {
                            printf(c"%lld %s\n".as_ptr() as *mut i8 as *const i8,
                                tm_end - tm_start,
                                &raw mut g.z_test_name[0 as usize] as *mut i8)
                        };
                        __state = 507;
                    }
                    510 => { disable_oom(); __state = 511; }
                    511 => { __state = 464; }
                    512 => {
                        if !(p_db).is_null() {
                            __state = 513;
                        } else { __state = 464; }
                    }
                    513 => { __state = 515; }
                    514 => { p_db = unsafe { (*p_db).p_next }; __state = 512; }
                    515 => {
                        z_vfs = c"inmem".as_ptr() as *mut i8 as *const i8;
                        __state = 516;
                    }
                    516 => {
                        if i_slice_sz > 0 && n_test % i_slice_sz != i_slice_idx {
                            __state = 518;
                        } else { __state = 517; }
                    }
                    517 => {
                        unsafe {
                            sqlite3_snprintf(core::mem::size_of::<[i8; 100]>() as i32,
                                &raw mut g.z_test_name[0 as usize] as *mut i8,
                                c"sqlid=%d,dbid=%d".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_sql).id }, unsafe { (*p_db).id })
                        };
                        __state = 521;
                    }
                    518 => {
                        { let __p = &mut n_test; let __t = *__p; *__p += 1; __t };
                        __state = 519;
                    }
                    519 => {
                        {
                            let __p = &mut n_slice_skip;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 520;
                    }
                    520 => { __state = 514; }
                    521 => {
                        if b_script != 0 { __state = 523; } else { __state = 524; }
                    }
                    522 => {
                        if n_skip > 0 { __state = 541; } else { __state = 540; }
                    }
                    523 => { __state = 522; }
                    524 => {
                        if b_spinner != 0 { __state = 525; } else { __state = 526; }
                    }
                    525 => { n_total_1 = g.n_db * g.n_sql; __state = 527; }
                    526 => {
                        if verbose_flag > 1 {
                            __state = 530;
                        } else { __state = 531; }
                    }
                    527 => {
                        idx__2 =
                            unsafe { (*p_sql).seq } * g.n_db + unsafe { (*p_db).id } -
                                1;
                        __state = 528;
                    }
                    528 => {
                        unsafe {
                            printf(c"\r%s: %d/%d            ".as_ptr() as *mut i8 as
                                    *const i8, z_db_name, idx__2, n_total_1)
                        };
                        __state = 529;
                    }
                    529 => { unsafe { fflush(__stdoutp) }; __state = 522; }
                    530 => {
                        unsafe {
                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                                &raw mut g.z_test_name[0 as usize] as *mut i8)
                        };
                        __state = 532;
                    }
                    531 => {
                        if (quiet_flag == 0) as i32 != 0 {
                            __state = 533;
                        } else { __state = 522; }
                    }
                    532 => { unsafe { fflush(__stdoutp) }; __state = 522; }
                    533 => { __state = 534; }
                    534 => {
                        idx__3 =
                            unsafe { (*p_sql).seq } * g.n_db + unsafe { (*p_db).id } -
                                1;
                        __state = 535;
                    }
                    535 => {
                        amt__1 = idx__3 * 10 / (g.n_db * g.n_sql);
                        __state = 536;
                    }
                    536 => {
                        if amt__1 != prev_amt_1 {
                            __state = 537;
                        } else { __state = 522; }
                    }
                    537 => {
                        unsafe {
                            printf(c" %d%%".as_ptr() as *mut i8 as *const i8,
                                amt__1 * 10)
                        };
                        __state = 538;
                    }
                    538 => { unsafe { fflush(__stdoutp) }; __state = 539; }
                    539 => { prev_amt_1 = amt__1; __state = 522; }
                    540 => {
                        if b_script != 0 { __state = 544; } else { __state = 543; }
                    }
                    541 => {
                        { let __p = &mut n_skip; let __t = *__p; *__p -= 1; __t };
                        __state = 542;
                    }
                    542 => { __state = 514; }
                    543 => {
                        create_v_file(c"main.db".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_db).sz },
                            unsafe { (*p_db).a.as_ptr() } as *mut u8 as *const u8);
                        __state = 548;
                    }
                    544 => { __state = 545; }
                    545 => {
                        unsafe {
                            sqlite3_snprintf(core::mem::size_of::<[i8; 100]>() as i32,
                                &raw mut z_name_1[0 as usize] as *mut i8,
                                c"db%06d.db".as_ptr() as *mut i8 as *const i8,
                                if unsafe { (*p_db).id } > 1 {
                                    unsafe { (*p_db).id }
                                } else { unsafe { (*p_sql).id } })
                        };
                        __state = 546;
                    }
                    546 => {
                        render_db_sql_for_cli(__stdoutp,
                            &raw mut z_name_1[0 as usize] as *mut i8 as *const i8,
                            unsafe { (*p_db).a.as_ptr() } as *mut u8,
                            unsafe { (*p_db).sz },
                            unsafe { (*p_sql).a.as_ptr() } as *mut u8,
                            unsafe { (*p_sql).sz });
                        __state = 547;
                    }
                    547 => { __state = 514; }
                    548 => {
                        unsafe { sqlite3_randomness(0, core::ptr::null_mut()) };
                        __state = 549;
                    }
                    549 => {
                        if oss_fuzz_this_db != 0 {
                            __state = 551;
                        } else { __state = 552; }
                    }
                    550 => {
                        if unsafe { sqlite3_memory_used() } > 0 as i64 {
                            __state = 576;
                        } else { __state = 575; }
                    }
                    551 => {
                        unsafe {
                            fatal_error(c"--oss-fuzz not supported: recompile with -DSQLITE_OSS_FUZZ".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 550;
                    }
                    552 => { open_flags = 4 | 2; __state = 553; }
                    553 => {
                        if native_flag != 0 && unsafe { (*p_db).sz } == 0 {
                            __state = 555;
                        } else { __state = 554; }
                    }
                    554 => {
                        rc =
                            unsafe {
                                sqlite3_open_v2(c"main.db".as_ptr() as *mut i8 as *const i8,
                                    &mut db, open_flags, z_vfs)
                            };
                        __state = 557;
                    }
                    555 => { open_flags |= 128; __state = 556; }
                    556 => { z_vfs = core::ptr::null(); __state = 554; }
                    557 => {
                        if rc != 0 { __state = 559; } else { __state = 558; }
                    }
                    558 => {
                        unsafe { sqlite3_limit(db, 0, 100000000) };
                        __state = 560;
                    }
                    559 => {
                        unsafe {
                            fatal_error(c"cannot open inmem database".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 558;
                    }
                    560 => {
                        unsafe { sqlite3_limit(db, 8, 50) };
                        __state = 561;
                    }
                    561 => {
                        if cell_sz_ck_flag != 0 {
                            __state = 563;
                        } else { __state = 562; }
                    }
                    562 => {
                        set_alarm((i_timeout + 999) / 1000);
                        __state = 564;
                    }
                    563 => {
                        run_sql(db,
                            c"PRAGMA cell_size_check=ON".as_ptr() as *mut i8 as
                                *const i8, run_flags as u32);
                        __state = 562;
                    }
                    564 => {
                        unsafe { sqlite3_test_control(17, db) };
                        __state = 565;
                    }
                    565 => {
                        if sql_fuzz != 0 || vdbe_limit_flag != 0 {
                            __state = 567;
                        } else { __state = 566; }
                    }
                    566 => {
                        unsafe { sqlite3_test_control(28, 1, db) };
                        __state = 568;
                    }
                    567 => {
                        unsafe {
                            sqlite3_progress_handler(db, 100000,
                                Some(progress_handler_1),
                                &raw mut vdbe_limit_flag as *mut ())
                        };
                        __state = 566;
                    }
                    568 => {
                        if b_vdbe_debug != 0 {
                            __state = 570;
                        } else { __state = 569; }
                    }
                    569 => {
                        run_sql(db,
                            unsafe { (*p_sql).a.as_ptr() } as *mut i8 as *const i8,
                            run_flags as u32);
                        __state = 572;
                    }
                    570 => {
                        unsafe {
                            sqlite3_exec(db,
                                c"PRAGMA vdbe_debug=ON".as_ptr() as *mut i8 as *const i8,
                                None, core::ptr::null_mut(), core::ptr::null_mut())
                        };
                        __state = 569;
                    }
                    571 => { set_alarm(0); __state = 573; }
                    572 => {
                        if timeout_test != 0 {
                            __state = 569;
                        } else { __state = 571; }
                    }
                    573 => {
                        unsafe {
                            sqlite3_exec(db,
                                c"PRAGMA temp_store_directory=\'\'".as_ptr() as *mut i8 as
                                    *const i8, None, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        };
                        __state = 574;
                    }
                    574 => { unsafe { sqlite3_close(db) }; __state = 550; }
                    575 => { reformat_vfs(); __state = 577; }
                    576 => {
                        unsafe {
                            fatal_error(c"memory leak: %lld bytes outstanding".as_ptr()
                                        as *mut i8 as *const i8, unsafe { sqlite3_memory_used() })
                        };
                        __state = 575;
                    }
                    577 => {
                        { let __p = &mut n_test; let __t = *__p; *__p += 1; __t };
                        __state = 578;
                    }
                    578 => {
                        if b_timer != 0 { __state = 580; } else { __state = 579; }
                    }
                    579 => {
                        g.z_test_name[0 as usize] = 0 as i8;
                        __state = 582;
                    }
                    580 => { tm_end_1 = time_of_day(); __state = 581; }
                    581 => {
                        unsafe {
                            printf(c"%lld %s\n".as_ptr() as *mut i8 as *const i8,
                                tm_end_1 - tm_start,
                                &raw mut g.z_test_name[0 as usize] as *mut i8)
                        };
                        __state = 579;
                    }
                    582 => {
                        if !(z_fail_code).is_null() {
                            __state = 583;
                        } else { __state = 514; }
                    }
                    583 => {
                        if unsafe { *z_fail_code.offset(0 as isize) } as i32 ==
                                    '5' as i32 &&
                                unsafe { *z_fail_code.offset(1 as isize) } as i32 == 0 {
                            __state = 584;
                        } else { __state = 585; }
                    }
                    584 => {
                        unsafe {
                            fatal_error(c"simulated failure".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 514;
                    }
                    585 => {
                        if unsafe { *z_fail_code.offset(0 as isize) } as i32 != 0 {
                            __state = 586;
                        } else { __state = 514; }
                    }
                    586 => {
                        unsafe {
                            printf(c"\nExit early due to TEST_FAILURE being set\n".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 587;
                    }
                    587 => { i_src_db = n_src_db - 1; __state = 588; }
                    588 => { __state = 2; }
                    589 => { __state = 2; }
                    590 => {
                        i_elapse = time_of_day() - i_begin;
                        __state = 592;
                    }
                    591 => {
                        if b_script != 0 { __state = 596; } else { __state = 597; }
                    }
                    592 => {
                        if i_slice_sz > 0 { __state = 594; } else { __state = 595; }
                    }
                    593 => { i_begin = time_of_day(); __state = 589; }
                    594 => {
                        unsafe {
                            printf(c"%s %s: 0 errors out of %d tests, slice %d/%d, %d.%03d seconds\n".as_ptr()
                                        as *mut i8 as *const i8,
                                path_tail(unsafe { *argv.offset(0 as isize) } as *const i8),
                                path_tail(g.z_db_file), n_test - n_slice_skip, i_slice_idx,
                                i_slice_sz, (i_elapse / 1000 as Sqlite3Int64) as i32,
                                (i_elapse % 1000 as Sqlite3Int64) as i32)
                        };
                        __state = 593;
                    }
                    595 => {
                        unsafe {
                            printf(c"%s %s: 0 errors out of %d tests, %d.%03d seconds\n".as_ptr()
                                        as *mut i8 as *const i8,
                                path_tail(unsafe { *argv.offset(0 as isize) } as *const i8),
                                path_tail(g.z_db_file), n_test,
                                (i_elapse / 1000 as Sqlite3Int64) as i32,
                                (i_elapse % 1000 as Sqlite3Int64) as i32)
                        };
                        __state = 593;
                    }
                    596 => { __state = 589; }
                    597 => {
                        if b_spinner != 0 { __state = 598; } else { __state = 599; }
                    }
                    598 => { n_total_2 = g.n_db * g.n_sql; __state = 600; }
                    599 => {
                        if (quiet_flag == 0) as i32 != 0 && verbose_flag < 2 {
                            __state = 601;
                        } else { __state = 589; }
                    }
                    600 => {
                        unsafe {
                            printf(c"\r%s: %d/%d          \n".as_ptr() as *mut i8 as
                                    *const i8, z_db_name, n_total_2, n_total_2)
                        };
                        __state = 589;
                    }
                    601 => {
                        unsafe {
                            printf(c" 100%% - %d tests\n".as_ptr() as *mut i8 as
                                    *const i8, g.n_db * g.n_sql)
                        };
                        __state = 589;
                    }
                    602 => { blob_list_free(g.p_first_db); __state = 603; }
                    603 => { reformat_vfs(); __state = 266; }
                    604 => {
                        unsafe { free(az_src_db as *mut ()) };
                        __state = 609;
                    }
                    605 => {
                        i_elapse_1 = time_of_day() - i_begin;
                        __state = 606;
                    }
                    606 => {
                        if g.n_invariant != 0 {
                            __state = 608;
                        } else { __state = 607; }
                    }
                    607 => {
                        unsafe {
                            printf(c"fuzzcheck: 0 errors out of %d tests in %d.%03d seconds\nSQLite %s %s (%d-bit)\n".as_ptr()
                                        as *mut i8 as *const i8, n_test,
                                (i_elapse_1 / 1000 as Sqlite3Int64) as i32,
                                (i_elapse_1 % 1000 as Sqlite3Int64) as i32,
                                unsafe { sqlite3_libversion() },
                                unsafe { sqlite3_sourceid() },
                                8 * core::mem::size_of::<*mut i8>() as i32)
                        };
                        __state = 604;
                    }
                    608 => {
                        unsafe {
                            printf(c"fuzzcheck: %u query invariants checked\n".as_ptr()
                                        as *mut i8 as *const i8, g.n_invariant)
                        };
                        __state = 607;
                    }
                    609 => { unsafe { free(p_heap) }; __state = 610; }
                    610 => { return Ok(()); }
                    _ => {}
                }
            }
        }
        unreachable!();
        return Ok(());
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
union BlobListLoadFromDbU0N25blobListLoadFromDbU0 {
    s_blob: Blob,
    tmp: [u8; 28],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IntegerValueS0N16integerValueS0 {
    z_suffix: *mut i8,
    i_mult: i32,
}
static mut clock_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
static mut az_bad_funcs: [*const i8; 35] =
    [c"avg".as_ptr() as *const i8, c"count".as_ptr() as *const i8,
            c"cume_dist".as_ptr() as *const i8,
            c"current_date".as_ptr() as *const i8,
            c"current_time".as_ptr() as *const i8,
            c"current_timestamp".as_ptr() as *const i8,
            c"date".as_ptr() as *const i8, c"datetime".as_ptr() as *const i8,
            c"decimal_sum".as_ptr() as *const i8,
            c"dense_rank".as_ptr() as *const i8,
            c"first_value".as_ptr() as *const i8,
            c"geopoly_group_bbox".as_ptr() as *const i8,
            c"group_concat".as_ptr() as *const i8,
            c"implies_nonnull_row".as_ptr() as *const i8,
            c"json_group_array".as_ptr() as *const i8,
            c"json_group_object".as_ptr() as *const i8,
            c"julianday".as_ptr() as *const i8, c"lag".as_ptr() as *const i8,
            c"last_value".as_ptr() as *const i8,
            c"lead".as_ptr() as *const i8, c"max".as_ptr() as *const i8,
            c"min".as_ptr() as *const i8, c"nth_value".as_ptr() as *const i8,
            c"ntile".as_ptr() as *const i8,
            c"percent_rank".as_ptr() as *const i8,
            c"random".as_ptr() as *const i8,
            c"randomblob".as_ptr() as *const i8,
            c"rank".as_ptr() as *const i8,
            c"row_number".as_ptr() as *const i8,
            c"sqlite_offset".as_ptr() as *const i8,
            c"strftime".as_ptr() as *const i8, c"sum".as_ptr() as *const i8,
            c"time".as_ptr() as *const i8, c"total".as_ptr() as *const i8,
            c"unixepoch".as_ptr() as *const i8];
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
static mut prev_amt: i32 = -1;
static mut prev_amt_1: i32 = -1;
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
    fn sqlite3_recover_init(db: *mut Sqlite3, z_db_1: *const i8,
    z_uri_1: *const i8)
    -> *mut Sqlite3Recover;
    fn sqlite3_recover_init_sql(db: *mut Sqlite3, z_db_1: *const i8,
    x_sql_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> i32>,
    p_ctx_1: *mut ())
    -> *mut Sqlite3Recover;
    fn sqlite3_recover_config(_: *mut Sqlite3Recover, op: i32,
    p_arg_1: *mut ())
    -> i32;
    fn sqlite3_recover_step(_: *mut Sqlite3Recover)
    -> i32;
    fn sqlite3_recover_run(_: *mut Sqlite3Recover)
    -> i32;
    fn sqlite3_recover_errmsg(_: *mut Sqlite3Recover)
    -> *const i8;
    fn sqlite3_recover_errcode(_: *mut Sqlite3Recover)
    -> i32;
    fn sqlite3_recover_finish(_: *mut Sqlite3Recover)
    -> i32;
    fn sqlite3_vt02_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_randomjson_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_series_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_base64_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_base85_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_completion_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_decimal_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_diskused_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_ieee_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_regexp_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_shathree_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_sha_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn sqlite3_stmtrand_init(_: *mut Sqlite3, _: *mut *mut i8, _: *mut ())
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn realloc(__ptr: *mut (), __size: u64)
    -> *mut ();
    fn free(_: *mut ())
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn fputc(_: i32, _: *mut FILE)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn rewind(_: *mut FILE)
    -> ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn isxdigit(_c: i32)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn strtoll(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    fn atoi(_: *const i8)
    -> i32;
    fn fuzz_invariant(db: *mut Sqlite3, p_stmt_1: *mut Sqlite3Stmt,
    i_cnt_1: i32, i_row_1: i32, n_row_1: i32, pb_corrupt_1: *mut i32,
    e_verbosity_1: i32, db_opt_1: u32)
    -> i32;
    fn sqlite3_dbdata_init(_: *mut Sqlite3, _: *mut *const i8, _: *mut ())
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn isspace(_c: i32)
    -> i32;
    fn putchar(_: i32)
    -> i32;
    fn isdigit(_c: i32)
    -> i32;
    fn getenv(_: *const i8)
    -> *mut i8;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn fgets(_: *mut i8, __size: i32, _: *mut FILE)
    -> *mut i8;
    fn __builtin_unreachable()
    -> ();
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stdinp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;