#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

type Int32T = i32;

type DarwinDevT = Int32T;

type DevT = DarwinDevT;

type Uint16T = u16;

type DarwinModeT = Uint16T;

type ModeT = DarwinModeT;

type NlinkT = Uint16T;

type Uint64T = u64;

type DarwinIno64T = Uint64T;

type Uint32T = u32;

type DarwinUidT = Uint32T;

type UidT = DarwinUidT;

type DarwinGidT = Uint32T;

type GidT = DarwinGidT;

type Int64T = i64;

type DarwinOffT = Int64T;

type OffT = DarwinOffT;

type DarwinBlkcntT = Int64T;

type BlkcntT = DarwinBlkcntT;

type DarwinBlksizeT = Int32T;

type BlksizeT = DarwinBlksizeT;

type DarwinTimeT = i64;

type TimeT = DarwinTimeT;

type DarwinSsizeT = i64;

type DarwinSusecondsT = Int32T;

type Uint8T = u8;

#[repr(C)]
#[derive(Copy, Clone)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atimespec: Timespec,
    st_mtimespec: Timespec,
    st_ctimespec: Timespec,
    st_birthtimespec: Timespec,
    st_size: i64,
    st_blocks: i64,
    st_blksize: i32,
    st_flags: u32,
    st_gen: u32,
    st_lspare: i32,
    st_qspare: [i64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Dirent {
    d_ino: u64,
    d_seekoff: u64,
    d_reclen: u16,
    d_namlen: u16,
    d_type: u8,
    d_name: [i8; 1024],
}

extern "C" fn read_file_contents(ctx: *mut Sqlite3Context,
    z_name_1: *const i8) -> () {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut n_in: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut p_buf: *mut () = core::ptr::null_mut();
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    let mut mx_blob: i32 = 0;
    in_ = unsafe { fopen(z_name_1, c"rb".as_ptr() as *mut i8 as *const i8) };
    if in_ == core::ptr::null_mut() { return; }
    unsafe { fseek(in_, 0 as i64, 2) };
    n_in = unsafe { ftell(in_) } as Sqlite3Int64;
    unsafe { rewind(in_) };
    db = unsafe { sqlite3_context_db_handle(ctx) };
    mx_blob = unsafe { sqlite3_limit(db, 0, -1) };
    if n_in > mx_blob as i64 {
        unsafe { sqlite3_result_error_code(ctx, 18) };
        unsafe { fclose(in_) };
        return;
    }
    p_buf =
        unsafe {
            sqlite3_malloc64(if n_in != 0 { n_in } else { 1 as Sqlite3Int64 }
                    as Sqlite3Uint64)
        };
    if p_buf == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(ctx) };
        unsafe { fclose(in_) };
        return;
    }
    if n_in ==
            unsafe { fread(p_buf, 1 as u64, n_in as u64, in_) } as
                Sqlite3Int64 {
        unsafe {
            sqlite3_result_blob64(ctx, p_buf as *const (),
                n_in as Sqlite3Uint64, Some(sqlite3_free))
        };
    } else {
        unsafe { sqlite3_result_error_code(ctx, 10) };
        unsafe { sqlite3_free(p_buf) };
    }
    unsafe { fclose(in_) };
}

extern "C" fn readfile_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_name: *const i8 = core::ptr::null();
    { let _ = argc; };
    z_name =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_name == core::ptr::null() { return; }
    read_file_contents(context, z_name);
}

unsafe extern "C" fn ctx_error_msg(ctx: *mut Sqlite3Context,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    unsafe { sqlite3_result_error(ctx, z_msg as *const i8, -1) };
    unsafe { sqlite3_free(z_msg as *mut ()) };
    ();
}

extern "C" fn file_stat(z_path_1: *const i8, p_stat_buf_1: *mut Stat) -> i32 {
    return unsafe { stat(z_path_1, p_stat_buf_1) };
}

extern "C" fn file_link_stat(z_path_1: *const i8, p_stat_buf_1: *mut Stat)
    -> i32 {
    return unsafe { lstat(z_path_1, p_stat_buf_1) };
}

extern "C" fn make_directory(z_file_1: *const i8) -> i32 {
    let z_copy: *mut i8 =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_file_1)
        };
    let mut rc: i32 = 0;
    if z_copy == core::ptr::null_mut() {
        rc = 7;
    } else {
        let n_copy: i32 = unsafe { strlen(z_copy as *const i8) } as i32;
        let mut i: i32 = 1;
        while rc == 0 {
            let mut s_stat: Stat = unsafe { core::mem::zeroed() };
            let mut rc2: i32 = 0;
            {
                '__b1: loop {
                    if !(unsafe { *z_copy.offset(i as isize) } as i32 !=
                                        '/' as i32 && i < n_copy) {
                        break '__b1;
                    }
                    '__c1: loop { break '__c1; }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i == n_copy { break; }
            unsafe { *z_copy.offset(i as isize) = '\u{0}' as i32 as i8 };
            rc2 = file_stat(z_copy as *const i8, &mut s_stat);
            if rc2 != 0 {
                if unsafe { mkdir(z_copy as *const i8, 511 as ModeT) } != 0 {
                    rc = 1;
                }
            } else {
                if !(s_stat.st_mode as i32 & 61440 == 16384) as i32 != 0 {
                    rc = 1;
                }
            }
            unsafe { *z_copy.offset(i as isize) = '/' as i32 as i8 };
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
        unsafe { sqlite3_free(z_copy as *mut ()) };
    }
    return rc;
}

extern "C" fn write_file(p_ctx_1: *mut Sqlite3Context, z_file_1: *const i8,
    p_data_1: *mut Sqlite3Value, mode: ModeT, mtime: Sqlite3Int64) -> i32 {
    if z_file_1 == core::ptr::null() { return 1; }
    if mode as i32 & 61440 == 40960 {
        let z_to: *const i8 =
            unsafe { sqlite3_value_text(p_data_1) } as *const i8;
        if z_to == core::ptr::null() { return 1; }
        unsafe { unlink(z_file_1) };
        if unsafe { symlink(z_to, z_file_1) } < 0 { return 1; }
    } else {
        if mode as i32 & 61440 == 16384 {
            if unsafe { mkdir(z_file_1, mode) } != 0 {
                let mut s_stat: Stat = unsafe { core::mem::zeroed() };
                if unsafe { *unsafe { __error() } } != 17 ||
                                0 != file_stat(z_file_1, &mut s_stat) ||
                            !(s_stat.st_mode as i32 & 61440 == 16384) as i32 != 0 ||
                        s_stat.st_mode as i32 & 511 != mode as i32 & 511 &&
                            0 !=
                                unsafe { chmod(z_file_1, (mode as i32 & 511) as ModeT) } {
                    return 1;
                }
            }
        } else {
            let mut n_write: Sqlite3Int64 = 0 as Sqlite3Int64;
            let mut z: *const i8 = core::ptr::null();
            let mut rc: i32 = 0;
            let out: *mut FILE =
                unsafe {
                    fopen(z_file_1, c"wb".as_ptr() as *mut i8 as *const i8)
                };
            if out == core::ptr::null_mut() { return 1; }
            z = unsafe { sqlite3_value_blob(p_data_1) } as *const i8;
            if !(z).is_null() {
                let n: Sqlite3Int64 =
                    unsafe {
                            fwrite(z as *const (), 1 as u64,
                                unsafe { sqlite3_value_bytes(p_data_1) } as u64, out)
                        } as Sqlite3Int64;
                n_write =
                    unsafe { sqlite3_value_bytes(p_data_1) } as Sqlite3Int64;
                if n_write != n { rc = 1; }
            }
            unsafe { fclose(out) };
            if rc == 0 && mode != 0 &&
                    unsafe { chmod(z_file_1, (mode as i32 & 511) as ModeT) } !=
                        0 {
                rc = 1;
            }
            if rc != 0 { return 2; }
            unsafe { sqlite3_result_int64(p_ctx_1, n_write) };
        }
    }
    if mtime >= 0 as i64 {
        if 0 == (mode as i32 & 61440 == 40960) as i32 {
            let mut times: [Timeval; 2] = unsafe { core::mem::zeroed() };
            times[0 as usize].tv_usec =
                { times[1 as usize].tv_usec = 0; times[1 as usize].tv_usec };
            times[0 as usize].tv_sec = unsafe { time(core::ptr::null_mut()) };
            times[1 as usize].tv_sec = mtime as DarwinTimeT;
            if unsafe {
                        utimes(z_file_1,
                            &raw mut times[0 as usize] as *mut Timeval as
                                *const Timeval)
                    } != 0 {
                return 1;
            }
        }
    }
    return 0;
}

extern "C" fn writefile_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_file: *const i8 = core::ptr::null();
    let mut mode: ModeT = 0 as ModeT;
    let mut res: i32 = 0;
    let mut mtime: Sqlite3Int64 = -1 as Sqlite3Int64;
    if argc < 2 || argc > 4 {
        unsafe {
            sqlite3_result_error(context,
                c"wrong number of arguments to function writefile()".as_ptr()
                        as *mut i8 as *const i8, -1)
        };
        return;
    }
    z_file =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_file == core::ptr::null() { return; }
    if argc >= 3 {
        mode =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(2 as isize) }) }
                as ModeT;
    }
    if argc == 4 {
        mtime =
            unsafe {
                sqlite3_value_int64(unsafe { *argv.offset(3 as isize) })
            };
    }
    res =
        write_file(context, z_file, unsafe { *argv.offset(1 as isize) }, mode,
            mtime);
    if res == 1 && unsafe { *unsafe { __error() } } == 2 {
        if make_directory(z_file) == 0 {
            res =
                write_file(context, z_file,
                    unsafe { *argv.offset(1 as isize) }, mode, mtime);
        }
    }
    if argc > 2 && res != 0 {
        if mode as i32 & 61440 == 40960 {
            unsafe {
                ctx_error_msg(context,
                    c"failed to create symlink: %s".as_ptr() as *mut i8 as
                        *const i8, z_file)
            };
        } else if mode as i32 & 61440 == 16384 {
            unsafe {
                ctx_error_msg(context,
                    c"failed to create directory: %s".as_ptr() as *mut i8 as
                        *const i8, z_file)
            };
        } else {
            unsafe {
                ctx_error_msg(context,
                    c"failed to write file: %s".as_ptr() as *mut i8 as
                        *const i8, z_file)
            };
        }
    }
}

extern "C" fn ls_mode_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut i: i32 = 0;
    let i_mode: i32 =
        unsafe { sqlite3_value_int(unsafe { *argv.offset(0 as isize) }) };
    let mut z: [i8; 16] = [0; 16];
    { let _ = argc; };
    if i_mode & 61440 == 40960 {
        z[0 as usize] = 'l' as i32 as i8;
    } else if i_mode & 61440 == 32768 {
        z[0 as usize] = '-' as i32 as i8;
    } else if i_mode & 61440 == 16384 {
        z[0 as usize] = 'd' as i32 as i8;
    } else { z[0 as usize] = '?' as i32 as i8; }
    {
        i = 0;
        '__b2: loop {
            if !(i < 3) { break '__b2; }
            '__c2: loop {
                let m: i32 = i_mode >> (2 - i) * 3;
                let a: *mut i8 = &mut z[(1 + i * 3) as usize];
                unsafe {
                    *a.offset(0 as isize) =
                        if m & 4 != 0 { 'r' as i32 } else { '-' as i32 } as i8
                };
                unsafe {
                    *a.offset(1 as isize) =
                        if m & 2 != 0 { 'w' as i32 } else { '-' as i32 } as i8
                };
                unsafe {
                    *a.offset(2 as isize) =
                        if m & 1 != 0 { 'x' as i32 } else { '-' as i32 } as i8
                };
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    z[10 as usize] = '\u{0}' as i32 as i8;
    unsafe {
        sqlite3_result_text(context,
            &raw mut z[0 as usize] as *mut i8 as *const i8, -1,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FsdirCursor {
    base: Sqlite3VtabCursor,
    n_lvl: i32,
    mx_lvl: i32,
    i_lvl: i32,
    a_lvl: *mut FsdirLevel,
    z_base: *const i8,
    n_base: i32,
    s_stat: Stat,
    z_path: *mut i8,
    i_rowid: Sqlite3Int64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FsdirLevel {
    p_dir: *mut DIR,
    z_dir: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FsdirTab {
    base: Sqlite3Vtab,
}

extern "C" fn fsdir_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut FsdirTab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = p_aux_1; };
    { let _ = argc; };
    { let _ = argv; };
    { let _ = pz_err_1; };
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(name,mode,mtime,data,level,path HIDDEN,dir HIDDEN)".as_ptr()
                        as *mut i8 as *const i8)
        };
    if rc == 0 {
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<FsdirTab>() as
                            Sqlite3Uint64)
                } as *mut FsdirTab;
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<FsdirTab>() as u64)
        };
        unsafe { sqlite3_vtab_config(db, 3) };
    }
    unsafe { *pp_vtab_1 = p_new as *mut Sqlite3Vtab };
    return rc;
}

extern "C" fn fsdir_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}

extern "C" fn fsdir_open(p: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_cur: *mut FsdirCursor = core::ptr::null_mut();
    { let _ = p; };
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<FsdirCursor>() as
                        Sqlite3Uint64)
            } as *mut FsdirCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<FsdirCursor>() as u64)
    };
    unsafe { (*p_cur).i_lvl = -1 };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}

extern "C" fn fsdir_reset_cursor(p_cur_1: &mut FsdirCursor) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b3: loop {
            if !(i <= (*p_cur_1).i_lvl) { break '__b3; }
            '__c3: loop {
                let p_lvl: *const FsdirLevel =
                    unsafe { &raw mut *(*p_cur_1).a_lvl.offset(i as isize) } as
                        *const FsdirLevel;
                if !(unsafe { (*p_lvl).p_dir }).is_null() {
                    unsafe { closedir(unsafe { (*p_lvl).p_dir }) };
                }
                unsafe { sqlite3_free(unsafe { (*p_lvl).z_dir } as *mut ()) };
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free((*p_cur_1).z_path as *mut ()) };
    unsafe { sqlite3_free((*p_cur_1).a_lvl as *mut ()) };
    (*p_cur_1).a_lvl = core::ptr::null_mut();
    (*p_cur_1).z_path = core::ptr::null_mut();
    (*p_cur_1).z_base = core::ptr::null();
    (*p_cur_1).n_base = 0;
    (*p_cur_1).n_lvl = 0;
    (*p_cur_1).i_lvl = -1;
    (*p_cur_1).i_rowid = 1 as Sqlite3Int64;
}

extern "C" fn fsdir_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut FsdirCursor = cur as *mut FsdirCursor;
    fsdir_reset_cursor(unsafe { &mut *p_cur });
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}

unsafe extern "C" fn fsdir_set_errmsg(p_cur_1: &FsdirCursor,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe {
        (*(*p_cur_1).base.p_vtab).z_err_msg =
            unsafe { sqlite3_vmprintf(z_fmt_1, ap) }
    };
    ();
}

extern "C" fn fsdir_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut FsdirCursor = cur as *mut FsdirCursor;
    let m: ModeT = unsafe { (*p_cur).s_stat.st_mode };
    {
        let __p = unsafe { &mut (*p_cur).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    if m as i32 & 61440 == 16384 &&
            unsafe { (*p_cur).i_lvl } + 3 < unsafe { (*p_cur).mx_lvl } {
        let i_new: i32 = unsafe { (*p_cur).i_lvl } + 1;
        let mut p_lvl: *mut FsdirLevel = core::ptr::null_mut();
        if i_new >= unsafe { (*p_cur).n_lvl } {
            let n_new: i32 = i_new + 1;
            let n_byte: Sqlite3Int64 =
                (n_new as u64 * core::mem::size_of::<FsdirLevel>() as u64) as
                    Sqlite3Int64;
            let a_new: *mut FsdirLevel =
                unsafe {
                        sqlite3_realloc64(unsafe { (*p_cur).a_lvl } as *mut (),
                            n_byte as Sqlite3Uint64)
                    } as *mut FsdirLevel;
            if a_new == core::ptr::null_mut() { return 7; }
            unsafe {
                memset(unsafe {
                            &raw mut *a_new.offset(unsafe { (*p_cur).n_lvl } as isize)
                        } as *mut (), 0,
                    core::mem::size_of::<FsdirLevel>() as u64 *
                        (n_new - unsafe { (*p_cur).n_lvl }) as u64)
            };
            unsafe { (*p_cur).a_lvl = a_new };
            unsafe { (*p_cur).n_lvl = n_new };
        }
        unsafe { (*p_cur).i_lvl = i_new };
        p_lvl = unsafe { unsafe { (*p_cur).a_lvl.offset(i_new as isize) } };
        unsafe { (*p_lvl).z_dir = unsafe { (*p_cur).z_path } };
        unsafe { (*p_cur).z_path = core::ptr::null_mut() };
        unsafe {
            (*p_lvl).p_dir =
                unsafe { opendir(unsafe { (*p_lvl).z_dir } as *const i8) }
        };
        if unsafe { (*p_lvl).p_dir } == core::ptr::null_mut() {
            unsafe {
                fsdir_set_errmsg(unsafe { &*p_cur },
                    c"cannot read directory: %s".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_lvl).z_dir })
            };
            return 1;
        }
    }
    while unsafe { (*p_cur).i_lvl } >= 0 {
        let p_lvl_1: *mut FsdirLevel =
            unsafe {
                &mut *unsafe {
                            (*p_cur).a_lvl.offset(unsafe { (*p_cur).i_lvl } as isize)
                        }
            };
        let p_entry: *mut Dirent =
            unsafe { readdir(unsafe { (*p_lvl_1).p_dir }) };
        if !(p_entry).is_null() {
            if unsafe { (*p_entry).d_name[0 as usize] } as i32 == '.' as i32 {
                if unsafe { (*p_entry).d_name[1 as usize] } as i32 ==
                            '.' as i32 &&
                        unsafe { (*p_entry).d_name[2 as usize] } as i32 ==
                            '\u{0}' as i32 {
                    continue;
                }
                if unsafe { (*p_entry).d_name[1 as usize] } as i32 ==
                        '\u{0}' as i32 {
                    continue;
                }
            }
            unsafe { sqlite3_free(unsafe { (*p_cur).z_path } as *mut ()) };
            unsafe {
                (*p_cur).z_path =
                    unsafe {
                        sqlite3_mprintf(c"%s/%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_lvl_1).z_dir },
                            unsafe { &raw mut (*p_entry).d_name[0 as usize] } as
                                *mut i8)
                    }
            };
            if unsafe { (*p_cur).z_path } == core::ptr::null_mut() {
                return 7;
            }
            if file_link_stat(unsafe { (*p_cur).z_path } as *const i8,
                        unsafe { &mut (*p_cur).s_stat }) != 0 {
                unsafe {
                    fsdir_set_errmsg(unsafe { &*p_cur },
                        c"cannot stat file: %s".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p_cur).z_path })
                };
                return 1;
            }
            return 0;
        }
        unsafe { closedir(unsafe { (*p_lvl_1).p_dir }) };
        unsafe { sqlite3_free(unsafe { (*p_lvl_1).z_dir } as *mut ()) };
        unsafe { (*p_lvl_1).p_dir = core::ptr::null_mut() };
        unsafe { (*p_lvl_1).z_dir = core::ptr::null_mut() };
        {
            let __p = unsafe { &mut (*p_cur).i_lvl };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
    unsafe { sqlite3_free(unsafe { (*p_cur).z_path } as *mut ()) };
    unsafe { (*p_cur).z_path = core::ptr::null_mut() };
    return 0;
}

extern "C" fn fsdir_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const FsdirCursor =
        cur as *mut FsdirCursor as *const FsdirCursor;
    '__s5:
        {
        match i {
            0 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_cur).z_path.offset(unsafe { (*p_cur).n_base } as isize)
                                            }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s5;
                }
                unsafe {
                    sqlite3_result_int64(ctx,
                        unsafe { (*p_cur).s_stat.st_mode } as Sqlite3Int64)
                };
            }
            1 => {
                unsafe {
                    sqlite3_result_int64(ctx,
                        unsafe { (*p_cur).s_stat.st_mode } as Sqlite3Int64)
                };
            }
            2 => {
                unsafe {
                    sqlite3_result_int64(ctx,
                        unsafe { (*p_cur).s_stat.st_mtimespec.tv_sec })
                };
            }
            3 => {
                {
                    let m: ModeT = unsafe { (*p_cur).s_stat.st_mode };
                    if m as i32 & 61440 == 16384 {
                        unsafe { sqlite3_result_null(ctx) };
                    } else if m as i32 & 61440 == 40960 {
                        let mut a_static: [i8; 64] = [0; 64];
                        let mut a_buf: *mut i8 =
                            &raw mut a_static[0 as usize] as *mut i8;
                        let mut n_buf: Sqlite3Int64 = 64 as Sqlite3Int64;
                        let mut n: i32 = 0;
                        loop {
                            n =
                                unsafe {
                                        readlink(unsafe { (*p_cur).z_path } as *const i8, a_buf,
                                            n_buf as u64)
                                    } as i32;
                            if (n as Sqlite3Int64) < n_buf { break; }
                            if a_buf != &raw mut a_static[0 as usize] as *mut i8 {
                                unsafe { sqlite3_free(a_buf as *mut ()) };
                            }
                            n_buf = n_buf * 2 as Sqlite3Int64;
                            a_buf =
                                unsafe { sqlite3_malloc64(n_buf as Sqlite3Uint64) } as
                                    *mut i8;
                            if a_buf == core::ptr::null_mut() {
                                unsafe { sqlite3_result_error_nomem(ctx) };
                                return 7;
                            }
                        }
                        unsafe {
                            sqlite3_result_text(ctx, a_buf as *const i8, n,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        if a_buf != &raw mut a_static[0 as usize] as *mut i8 {
                            unsafe { sqlite3_free(a_buf as *mut ()) };
                        }
                    } else {
                        read_file_contents(ctx,
                            unsafe { (*p_cur).z_path } as *const i8);
                    }
                    break '__s5;
                }
                unsafe {
                    sqlite3_result_int(ctx, unsafe { (*p_cur).i_lvl } + 2)
                };
            }
            4 => {
                unsafe {
                    sqlite3_result_int(ctx, unsafe { (*p_cur).i_lvl } + 2)
                };
            }
            5 => { { break '__s5; } }
            _ => { { break '__s5; } }
        }
    }
    return 0;
}

extern "C" fn fsdir_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const FsdirCursor =
        cur as *mut FsdirCursor as *const FsdirCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}

extern "C" fn fsdir_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const FsdirCursor =
        cur as *mut FsdirCursor as *const FsdirCursor;
    return (unsafe { (*p_cur).z_path } == core::ptr::null_mut()) as i32;
}

extern "C" fn fsdir_filter(cur: *mut Sqlite3VtabCursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut Sqlite3Value) -> i32 {
    let mut z_dir: *const i8 = core::ptr::null();
    let p_cur: *mut FsdirCursor = cur as *mut FsdirCursor;
    let mut i: i32 = 0;
    { let _ = idx_str_1; };
    fsdir_reset_cursor(unsafe { &mut *p_cur });
    if idx_num_1 == 0 {
        unsafe {
            fsdir_set_errmsg(unsafe { &*p_cur },
                c"table function fsdir requires an argument".as_ptr() as
                        *mut i8 as *const i8)
        };
        return 1;
    }
    if !(idx_num_1 & 1 != 0 && argc > 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsdirFilter".as_ptr() as *const i8,
                c"fileio.c".as_ptr() as *mut i8 as *const i8, 921,
                c"(idxNum & 0x01)!=0 && argc>0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    z_dir =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_dir == core::ptr::null() {
        unsafe {
            fsdir_set_errmsg(unsafe { &*p_cur },
                c"table function fsdir requires a non-NULL argument".as_ptr()
                        as *mut i8 as *const i8)
        };
        return 1;
    }
    i = 1;
    if idx_num_1 & 2 != 0 {
        if !(argc > i) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fsdirFilter".as_ptr() as *const i8,
                    c"fileio.c".as_ptr() as *mut i8 as *const i8, 929,
                    c"argc>i".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            (*p_cur).z_base =
                unsafe {
                        sqlite3_value_text(unsafe {
                                *argv.offset({
                                                let __p = &mut i;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            })
                    } as *const i8
        };
    }
    if idx_num_1 & 12 != 0 {
        if !(argc > i) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fsdirFilter".as_ptr() as *const i8,
                    c"fileio.c".as_ptr() as *mut i8 as *const i8, 933,
                    c"argc>i".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            (*p_cur).mx_lvl =
                unsafe {
                    sqlite3_value_int(unsafe {
                            *argv.offset({
                                            let __p = &mut i;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        })
                }
        };
        if idx_num_1 & 8 != 0 {
            {
                let __p = unsafe { &mut (*p_cur).mx_lvl };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        if unsafe { (*p_cur).mx_lvl } <= 0 {
            unsafe { (*p_cur).mx_lvl = 1000000000 };
        }
    } else { unsafe { (*p_cur).mx_lvl = 1000000000 }; }
    if !(unsafe { (*p_cur).z_base }).is_null() {
        unsafe {
            (*p_cur).n_base =
                unsafe { strlen(unsafe { (*p_cur).z_base }) } as i32 + 1
        };
        unsafe {
            (*p_cur).z_path =
                unsafe {
                    sqlite3_mprintf(c"%s/%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p_cur).z_base }, z_dir)
                }
        };
    } else {
        unsafe {
            (*p_cur).z_path =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_dir)
                }
        };
    }
    if unsafe { (*p_cur).z_path } == core::ptr::null_mut() { return 7; }
    if file_link_stat(unsafe { (*p_cur).z_path } as *const i8,
                unsafe { &mut (*p_cur).s_stat }) != 0 {
        unsafe {
            fsdir_set_errmsg(unsafe { &*p_cur },
                c"cannot stat file: %s".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p_cur).z_path })
        };
        return 1;
    }
    return 0;
}

extern "C" fn fsdir_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i: i32 = 0;
    let mut idx_path: i32 = -1;
    let mut idx_dir: i32 = -1;
    let mut idx_level: i32 = -1;
    let mut idx_level_eq: i32 = 0;
    let mut omit_level: i32 = 0;
    let mut seen_path: i32 = 0;
    let mut seen_dir: i32 = 0;
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();
    { let _ = tab; };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b7: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) { break '__b7; }
            '__c7: loop {
                if unsafe { (*p_constraint).op } as i32 == 2 {
                    '__s8:
                        {
                        match unsafe { (*p_constraint).i_column } {
                            5 => {
                                {
                                    if unsafe { (*p_constraint).usable } != 0 {
                                        idx_path = i;
                                        seen_path = 0;
                                    } else if idx_path < 0 { seen_path = 1; }
                                    break '__s8;
                                }
                                {
                                    if unsafe { (*p_constraint).usable } != 0 {
                                        idx_dir = i;
                                        seen_dir = 0;
                                    } else if idx_dir < 0 { seen_dir = 1; }
                                    break '__s8;
                                }
                                {
                                    if unsafe { (*p_constraint).usable } != 0 && idx_level < 0 {
                                        idx_level = i;
                                        idx_level_eq = 8;
                                        omit_level = 0;
                                    }
                                    break '__s8;
                                }
                            }
                            6 => {
                                {
                                    if unsafe { (*p_constraint).usable } != 0 {
                                        idx_dir = i;
                                        seen_dir = 0;
                                    } else if idx_dir < 0 { seen_dir = 1; }
                                    break '__s8;
                                }
                                {
                                    if unsafe { (*p_constraint).usable } != 0 && idx_level < 0 {
                                        idx_level = i;
                                        idx_level_eq = 8;
                                        omit_level = 0;
                                    }
                                    break '__s8;
                                }
                            }
                            4 => {
                                {
                                    if unsafe { (*p_constraint).usable } != 0 && idx_level < 0 {
                                        idx_level = i;
                                        idx_level_eq = 8;
                                        omit_level = 0;
                                    }
                                    break '__s8;
                                }
                            }
                            _ => {}
                        }
                    }
                } else if unsafe { (*p_constraint).i_column } as i32 == 4 &&
                            unsafe { (*p_constraint).usable } != 0 && idx_level < 0 {
                    if unsafe { (*p_constraint).op } as i32 == 8 {
                        idx_level = i;
                        idx_level_eq = 8;
                        omit_level = 1;
                    } else if unsafe { (*p_constraint).op } as i32 == 16 {
                        idx_level = i;
                        idx_level_eq = 4;
                        omit_level = 1;
                    }
                }
                break '__c7;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_constraint;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if seen_path != 0 || seen_dir != 0 { return 19; }
    if idx_path < 0 {
        unsafe { (*p_idx_info_1).idx_num = 0 };
        unsafe {
            (*p_idx_info_1).estimated_rows = 2147483647 as Sqlite3Int64
        };
    } else {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(idx_path as isize)
                        }).omit = 1 as u8
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(idx_path as isize)
                        }).argv_index = 1
        };
        unsafe { (*p_idx_info_1).idx_num = 1 };
        unsafe { (*p_idx_info_1).estimated_cost = 1000000000.0 };
        i = 2;
        if idx_dir >= 0 {
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(idx_dir as isize)
                            }).omit = 1 as u8
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(idx_dir as isize)
                            }).argv_index =
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
            };
            unsafe { (*p_idx_info_1).idx_num |= 2 };
            unsafe { (*p_idx_info_1).estimated_cost /= 10000.0 };
        }
        if idx_level >= 0 {
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(idx_level as
                                        isize)
                            }).omit = omit_level as u8
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(idx_level as
                                        isize)
                            }).argv_index =
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
            };
            unsafe { (*p_idx_info_1).idx_num |= idx_level_eq };
            unsafe { (*p_idx_info_1).estimated_cost /= 10000.0 };
        }
    }
    return 0;
}

extern "C" fn fsdir_register(db: *mut Sqlite3) -> i32 {
    unsafe {
        let rc: i32 =
            unsafe {
                sqlite3_create_module(db,
                    c"fsdir".as_ptr() as *mut i8 as *const i8,
                    &raw mut fsdir_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            };
        return rc;
    }
}

extern "C" fn portable_realpath(z_path_1: *const i8) -> *mut i8 {
    let mut z_out: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut z_buf: [i8; 1025] = [0; 1025];
    if z_path_1 == core::ptr::null() { return core::ptr::null_mut(); }
    z = unsafe { realpath(z_path_1, &raw mut z_buf[0 as usize] as *mut i8) };
    if !(z).is_null() {
        z_out =
            unsafe {
                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                    &raw mut z_buf[0 as usize] as *mut i8)
            };
    }
    if z_out == core::ptr::null_mut() {
        z = unsafe { realpath(z_path_1, 0 as *mut () as *mut i8) };
        if !(z).is_null() {
            z_out =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z)
                };
            unsafe { free(z as *mut ()) };
        }
    }
    return z_out;
}

extern "C" fn realpath_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_path: *const i8 = core::ptr::null();
    let mut z_copy: *mut i8 = core::ptr::null_mut();
    let mut z_out: *mut i8 = core::ptr::null_mut();
    let mut c_sep: i8 = 0 as i8;
    let mut len: u64 = 0 as u64;
    let is_win: i32 = 0 as i32;
    { let _ = argc; };
    z_path =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_path == core::ptr::null() { return; }
    if unsafe { *z_path.offset(0 as isize) } as i32 == 0 {
        z_path = c".".as_ptr() as *mut i8 as *const i8;
    }
    z_copy =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_path)
        };
    if z_copy == core::ptr::null_mut() { return; }
    len = unsafe { strlen(z_copy as *const i8) };
    while len > 1 as u64 &&
            (unsafe { *z_copy.add((len - 1 as u64) as usize) } as i32 ==
                    '/' as i32 ||
                is_win != 0 &&
                    unsafe { *z_copy.add((len - 1 as u64) as usize) } as i32 ==
                        '\\' as i32) {
        { let __p = &mut len; let __t = *__p; *__p -= 1; __t };
    }
    unsafe { *z_copy.add(len as usize) = 0 as i8 };
    loop {
        z_out = portable_realpath(z_copy as *const i8);
        unsafe { *z_copy.add(len as usize) = c_sep };
        if !(z_out).is_null() {
            if c_sep != 0 {
                z_out =
                    unsafe {
                        sqlite3_mprintf(c"%z%s".as_ptr() as *mut i8 as *const i8,
                            z_out,
                            unsafe { &raw mut *z_copy.add(len as usize) } as *mut i8)
                    };
            }
            break;
        } else {
            let mut i: u64 = len - 1 as u64;
            while i > 0 as u64 {
                if unsafe { *z_copy.add(i as usize) } as i32 == '/' as i32 ||
                        is_win != 0 &&
                            unsafe { *z_copy.add(i as usize) } as i32 == '\\' as i32 {
                    break;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
            if i <= 0 as u64 {
                if unsafe { *z_copy.offset(0 as isize) } as i32 == '/' as i32
                    {
                    z_out = z_copy;
                    z_copy = core::ptr::null_mut();
                } else if {
                            z_out =
                                portable_realpath(c".".as_ptr() as *mut i8 as *const i8);
                            z_out
                        } != core::ptr::null_mut() {
                    z_out =
                        unsafe {
                            sqlite3_mprintf(c"%z/%s".as_ptr() as *mut i8 as *const i8,
                                z_out, z_copy)
                        };
                }
                break;
            }
            c_sep = unsafe { *z_copy.add(i as usize) };
            unsafe { *z_copy.add(i as usize) = 0 as i8 };
            len = i;
        }
    }
    unsafe { sqlite3_free(z_copy as *mut ()) };
    if !(z_out).is_null() {
        let mut i: u64 = 0 as u64;
        let mut j: u64 = 0 as u64;
        let mut n: u64 = 0 as u64;
        n = unsafe { strlen(z_out as *const i8) };
        {
            i = { j = 0 as u64; j };
            '__b12: loop {
                if !(i < n) { break '__b12; }
                '__c12: loop {
                    if unsafe { *z_out.add(i as usize) } as i32 == '/' as i32 {
                        if unsafe { *z_out.add((i + 1 as u64) as usize) } as i32 ==
                                '/' as i32 {
                            break '__c12;
                        }
                        if unsafe { *z_out.add((i + 1 as u64) as usize) } as i32 ==
                                        '.' as i32 && (i + 2 as u64) < n &&
                                unsafe { *z_out.add((i + 2 as u64) as usize) } as i32 ==
                                    '/' as i32 {
                            i += 1 as u64;
                            break '__c12;
                        }
                        if unsafe { *z_out.add((i + 1 as u64) as usize) } as i32 ==
                                            '.' as i32 && (i + 3 as u64) < n &&
                                    unsafe { *z_out.add((i + 2 as u64) as usize) } as i32 ==
                                        '.' as i32 &&
                                unsafe { *z_out.add((i + 3 as u64) as usize) } as i32 ==
                                    '/' as i32 {
                            while j > 0 as u64 &&
                                    unsafe { *z_out.add((j - 1 as u64) as usize) } as i32 !=
                                        '/' as i32 {
                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                            }
                            if j > 0 as u64 {
                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                            }
                            i += 2 as u64;
                            break '__c12;
                        }
                    }
                    unsafe {
                        *z_out.add({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = unsafe { *z_out.add(i as usize) }
                    };
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { *z_out.add(j as usize) = 0 as i8 };
        unsafe {
            sqlite3_result_text(context, z_out as *const i8, -1,
                Some(sqlite3_free))
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fileio_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"readfile".as_ptr() as *mut i8 as *const i8, 1, 1 | 524288,
                core::ptr::null_mut(), Some(readfile_func), None, None)
        };
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"writefile".as_ptr() as *mut i8 as *const i8, -1,
                    1 | 524288, core::ptr::null_mut(), Some(writefile_func),
                    None, None)
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"lsmode".as_ptr() as *mut i8 as *const i8, 1, 1,
                    core::ptr::null_mut(), Some(ls_mode_func), None, None)
            };
    }
    if rc == 0 { rc = fsdir_register(db); }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"realpath".as_ptr() as *mut i8 as *const i8, 1, 1,
                    core::ptr::null_mut(), Some(realpath_func), None, None)
            };
    }
    return rc;
}

static mut fsdir_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: None,
        x_connect: Some(fsdir_connect),
        x_best_index: Some(fsdir_best_index),
        x_disconnect: Some(fsdir_disconnect),
        x_destroy: None,
        x_open: Some(fsdir_open),
        x_close: Some(fsdir_close),
        x_filter: Some(fsdir_filter),
        x_next: Some(fsdir_next),
        x_eof: Some(fsdir_eof),
        x_column: Some(fsdir_column),
        x_rowid: Some(fsdir_rowid),
        x_update: None,
        x_begin: None,
        x_sync: None,
        x_commit: None,
        x_rollback: None,
        x_find_function: None,
        x_rename: None,
        x_savepoint: None,
        x_release: None,
        x_rollback_to: None,
        x_shadow_name: None,
        x_integrity: None,
    };

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
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn rewind(_: *mut FILE)
    -> ();
    fn fclose(_: *mut FILE)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn stat(_: *const i8, _: *mut Stat)
    -> i32;
    fn lstat(_: *const i8, _: *mut Stat)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn mkdir(_: *const i8, _: ModeT)
    -> i32;
    fn unlink(_: *const i8)
    -> i32;
    fn symlink(_: *const i8, _: *const i8)
    -> i32;
    fn __error()
    -> *mut i32;
    fn chmod(_: *const i8, _: ModeT)
    -> i32;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn time(_: *mut TimeT)
    -> TimeT;
    fn utimes(_: *const i8, _: *const Timeval)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn closedir(_: *mut DIR)
    -> i32;
    fn opendir(_: *const i8)
    -> *mut DIR;
    fn readdir(_: *mut DIR)
    -> *mut Dirent;
    fn readlink(_: *const i8, _: *mut i8, __bufsize: u64)
    -> i64;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn realpath(_: *const i8, _: *mut i8)
    -> *mut i8;
    fn free(_: *mut ())
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DIR {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
