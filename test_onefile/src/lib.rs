#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3IoMethods,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, SqliteInt64,
};

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct FsRealFile {
    p_file: *mut Sqlite3File,
    z_name: *const i8,
    n_database: i32,
    n_journal: i32,
    n_blob: i32,
    n_ref: i32,
    p_next: *mut FsRealFile,
    pp_this: *mut *mut FsRealFile,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FsFile {
    base: Sqlite3File,
    e_type: i32,
    p_real: *mut FsRealFile,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TmpFile {
    base: Sqlite3File,
    n_size: i32,
    n_alloc: i32,
    z_alloc: *mut i8,
}

///* Method declarations for fs_file.
#[allow(unused_doc_comments)]
extern "C" fn fs_close(p_file: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let p: *const FsFile = p_file as *mut FsFile as *const FsFile;
    let p_real: *mut FsRealFile = unsafe { (*p).p_real };

    /// Decrement the real_file ref-count.
    {
        let __p = unsafe { &mut (*p_real).n_ref };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    if !(unsafe { (*p_real).n_ref } >= 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsClose".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 382,
                c"pReal->nRef>=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_real).n_ref } == 0 {
        unsafe {
            *unsafe { (*p_real).pp_this } = unsafe { (*p_real).p_next }
        };
        if !(unsafe { (*p_real).p_next }).is_null() {
            unsafe {
                (*unsafe { (*p_real).p_next }).pp_this =
                    unsafe { (*p_real).pp_this }
            };
        }
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*unsafe { (*p_real).p_file }).p_methods
                                        }).x_close.unwrap()
                    })(unsafe { (*p_real).p_file })
            };
        unsafe { sqlite3_free(p_real as *mut ()) };
    }
    return rc;
}

///* Read data from an fs-file.
#[allow(unused_doc_comments)]
extern "C" fn fs_read(p_file: *mut Sqlite3File, z_buf: *mut (), i_amt: i32,
    i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *const FsFile = p_file as *mut FsFile as *const FsFile;
    let p_real: *const FsRealFile =
        unsafe { (*p).p_real } as *const FsRealFile;
    let p_f: *mut Sqlite3File = unsafe { (*p_real).p_file };
    if unsafe { (*p).e_type } == 1 &&
                i_amt as SqliteInt64 + i_ofst >
                    unsafe { (*p_real).n_database } as i64 ||
            unsafe { (*p).e_type } == 2 &&
                i_amt as SqliteInt64 + i_ofst >
                    unsafe { (*p_real).n_journal } as i64 {
        rc = 10 | 2 << 8;
    } else if unsafe { (*p).e_type } == 1 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_f).p_methods }).x_read.unwrap()
                    })(p_f, z_buf, i_amt, i_ofst + 512 as SqliteInt64)
            };
    } else {
        /// Journal file.
        let mut i_rem: i32 = i_amt;
        let mut i_buf: i32 = 0;
        let mut ii: i32 = i_ofst as i32;
        while i_rem > 0 && rc == 0 {
            let i_real_off: i32 =
                unsafe { (*p_real).n_blob } - 512 * (ii / 512 + 1) + ii % 512;
            let i_real_amt: i32 =
                if i_rem < 512 - i_real_off % 512 {
                    i_rem
                } else { 512 - i_real_off % 512 };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_f).p_methods }).x_read.unwrap()
                        })(p_f,
                        unsafe {
                                &raw mut *(z_buf as *mut i8).offset(i_buf as isize)
                            } as *mut (), i_real_amt, i_real_off as i64)
                };
            ii += i_real_amt;
            i_buf += i_real_amt;
            i_rem -= i_real_amt;
        }
    }
    return rc;
}

///* Write data to an fs-file.
#[allow(unused_doc_comments)]
extern "C" fn fs_write(p_file: *mut Sqlite3File, z_buf: *const (), i_amt: i32,
    i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *const FsFile = p_file as *mut FsFile as *const FsFile;
    let p_real: *mut FsRealFile = unsafe { (*p).p_real };
    let p_f: *mut Sqlite3File = unsafe { (*p_real).p_file };
    if unsafe { (*p).e_type } == 1 {
        if i_amt as SqliteInt64 + i_ofst + 512 as SqliteInt64 >
                (unsafe { (*p_real).n_blob } - unsafe { (*p_real).n_journal })
                    as i64 {
            rc = 13;
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_f).p_methods }).x_write.unwrap()
                        })(p_f, z_buf, i_amt, i_ofst + 512 as SqliteInt64)
                };
            if rc == 0 {
                unsafe {
                    (*p_real).n_database =
                        if unsafe { (*p_real).n_database } as SqliteInt64 >
                                    i_amt as SqliteInt64 + i_ofst {
                                (unsafe { (*p_real).n_database }) as SqliteInt64
                            } else { i_amt as SqliteInt64 + i_ofst } as i32
                };
            }
        }
    } else {
        /// Journal file.
        let mut i_rem: i32 = i_amt;
        let mut i_buf: i32 = 0;
        let mut ii: i32 = i_ofst as i32;
        while i_rem > 0 && rc == 0 {
            let i_real_off: i32 =
                unsafe { (*p_real).n_blob } - 512 * (ii / 512 + 1) + ii % 512;
            let i_real_amt: i32 =
                if i_rem < 512 - i_real_off % 512 {
                    i_rem
                } else { 512 - i_real_off % 512 };
            if i_real_off < unsafe { (*p_real).n_database } + 512 {
                rc = 13;
            } else {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_f).p_methods }).x_write.unwrap()
                            })(p_f,
                            unsafe {
                                    &raw mut *(z_buf as *mut i8).offset(i_buf as isize)
                                } as *const (), i_real_amt, i_real_off as i64)
                    };
                ii += i_real_amt;
                i_buf += i_real_amt;
                i_rem -= i_real_amt;
            }
        }
        if rc == 0 {
            unsafe {
                (*p_real).n_journal =
                    if unsafe { (*p_real).n_journal } as SqliteInt64 >
                                i_amt as SqliteInt64 + i_ofst {
                            (unsafe { (*p_real).n_journal }) as SqliteInt64
                        } else { i_amt as SqliteInt64 + i_ofst } as i32
            };
        }
    }
    return rc;
}

///* Truncate an fs-file.
extern "C" fn fs_truncate(p_file: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    let p: *const FsFile = p_file as *mut FsFile as *const FsFile;
    let p_real: *mut FsRealFile = unsafe { (*p).p_real };
    if unsafe { (*p).e_type } == 1 {
        unsafe {
            (*p_real).n_database =
                if (unsafe { (*p_real).n_database } as SqliteInt64) < size {
                        (unsafe { (*p_real).n_database }) as SqliteInt64
                    } else { size } as i32
        };
    } else {
        unsafe {
            (*p_real).n_journal =
                if (unsafe { (*p_real).n_journal } as SqliteInt64) < size {
                        (unsafe { (*p_real).n_journal }) as SqliteInt64
                    } else { size } as i32
        };
    }
    return 0;
}

///* Sync an fs-file.
extern "C" fn fs_sync(p_file: *mut Sqlite3File, flags: i32) -> i32 {
    let p: *const FsFile = p_file as *mut FsFile as *const FsFile;
    let p_real: *const FsRealFile =
        unsafe { (*p).p_real } as *const FsRealFile;
    let p_real_file: *mut Sqlite3File = unsafe { (*p_real).p_file };
    let mut rc: i32 = 0;
    if unsafe { (*p).e_type } == 1 {
        let mut z_size: [u8; 4] = [0; 4];
        z_size[0 as usize] =
            ((unsafe { (*p_real).n_database } as u32 & 4278190080u32) >> 24)
                as u8;
        z_size[1 as usize] =
            ((unsafe { (*p_real).n_database } & 16711680) >> 16) as u8;
        z_size[2 as usize] =
            ((unsafe { (*p_real).n_database } & 65280) >> 8) as u8;
        z_size[3 as usize] = (unsafe { (*p_real).n_database } & 255) as u8;
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_real_file).p_methods }).x_write.unwrap()
                    })(p_real_file,
                    &raw mut z_size[0 as usize] as *mut u8 as *const (), 4, 0)
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_real_file).p_methods }).x_sync.unwrap()
                    })(p_real_file, flags & !16)
            };
    }
    return rc;
}

///* Return the current file-size of an fs-file.
extern "C" fn fs_file_size(p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let p: *const FsFile = p_file as *mut FsFile as *const FsFile;
    let p_real: *const FsRealFile =
        unsafe { (*p).p_real } as *const FsRealFile;
    if unsafe { (*p).e_type } == 1 {
        unsafe { *p_size = unsafe { (*p_real).n_database } as SqliteInt64 };
    } else {
        unsafe { *p_size = unsafe { (*p_real).n_journal } as SqliteInt64 };
    }
    return 0;
}

///* Lock an fs-file.
extern "C" fn fs_lock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    return 0;
}

///* Unlock an fs-file.
extern "C" fn fs_unlock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    return 0;
}

///* Check if another file-handle holds a RESERVED lock on an fs-file.
extern "C" fn fs_check_reserved_lock(p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    unsafe { *p_res_out = 0 };
    return 0;
}

///* File control method. For custom operations on an fs-file.
extern "C" fn fs_file_control(p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    if op == 14 { return 12; }
    return 0;
}

///* Return the sector-size in bytes for an fs-file.
extern "C" fn fs_sector_size(p_file: *mut Sqlite3File) -> i32 { return 512; }

///* Return the device characteristic flags supported by an fs-file.
extern "C" fn fs_device_characteristics(p_file: *mut Sqlite3File) -> i32 {
    return 0;
}

///* Method declarations for tmp_file.
extern "C" fn tmp_close(p_file: *mut Sqlite3File) -> i32 {
    let p_tmp: *const TmpFile = p_file as *mut TmpFile as *const TmpFile;
    unsafe { sqlite3_free(unsafe { (*p_tmp).z_alloc } as *mut ()) };
    return 0;
}

///* Read data from a tmp-file.
extern "C" fn tmp_read(p_file: *mut Sqlite3File, z_buf: *mut (), i_amt: i32,
    i_ofst: Sqlite3Int64) -> i32 {
    let p_tmp: *const TmpFile = p_file as *mut TmpFile as *const TmpFile;
    if i_amt as SqliteInt64 + i_ofst > unsafe { (*p_tmp).n_size } as i64 {
        return 10 | 2 << 8;
    }
    unsafe {
        memcpy(z_buf,
            unsafe {
                    &raw mut *unsafe {
                                (*p_tmp).z_alloc.offset(i_ofst as isize)
                            }
                } as *const (), i_amt as u64)
    };
    return 0;
}

///* Write data to a tmp-file.
extern "C" fn tmp_write(p_file: *mut Sqlite3File, z_buf: *const (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let p_tmp: *mut TmpFile = p_file as *mut TmpFile;
    if i_amt as SqliteInt64 + i_ofst > unsafe { (*p_tmp).n_alloc } as i64 {
        let n_new: i32 =
            (2 as SqliteInt64 *
                    (i_amt as SqliteInt64 + i_ofst +
                        unsafe { (*p_tmp).n_alloc } as SqliteInt64)) as i32;
        let z_new: *mut i8 =
            unsafe {
                    sqlite3_realloc(unsafe { (*p_tmp).z_alloc } as *mut (),
                        n_new)
                } as *mut i8;
        if (z_new).is_null() as i32 != 0 { return 7; }
        unsafe { (*p_tmp).z_alloc = z_new };
        unsafe { (*p_tmp).n_alloc = n_new };
    }
    unsafe {
        memcpy(unsafe {
                    &raw mut *unsafe {
                                (*p_tmp).z_alloc.offset(i_ofst as isize)
                            }
                } as *mut (), z_buf, i_amt as u64)
    };
    unsafe {
        (*p_tmp).n_size =
            if unsafe { (*p_tmp).n_size } as SqliteInt64 >
                        i_ofst + i_amt as SqliteInt64 {
                    (unsafe { (*p_tmp).n_size }) as SqliteInt64
                } else { i_ofst + i_amt as SqliteInt64 } as i32
    };
    return 0;
}

///* Truncate a tmp-file.
extern "C" fn tmp_truncate(p_file: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    let p_tmp: *mut TmpFile = p_file as *mut TmpFile;
    unsafe {
        (*p_tmp).n_size =
            if (unsafe { (*p_tmp).n_size } as SqliteInt64) < size {
                    (unsafe { (*p_tmp).n_size }) as SqliteInt64
                } else { size } as i32
    };
    return 0;
}

///* Sync a tmp-file.
extern "C" fn tmp_sync(p_file: *mut Sqlite3File, flags: i32) -> i32 {
    return 0;
}

///* Return the current file-size of a tmp-file.
extern "C" fn tmp_file_size(p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let p_tmp: *const TmpFile = p_file as *mut TmpFile as *const TmpFile;
    unsafe { *p_size = unsafe { (*p_tmp).n_size } as SqliteInt64 };
    return 0;
}

///* Lock a tmp-file.
extern "C" fn tmp_lock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    return 0;
}

///* Unlock a tmp-file.
extern "C" fn tmp_unlock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    return 0;
}

///* Check if another file-handle holds a RESERVED lock on a tmp-file.
extern "C" fn tmp_check_reserved_lock(p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    unsafe { *p_res_out = 0 };
    return 0;
}

///* File control method. For custom operations on a tmp-file.
extern "C" fn tmp_file_control(p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    return 0;
}

///* Return the sector-size in bytes for a tmp-file.
extern "C" fn tmp_sector_size(p_file: *mut Sqlite3File) -> i32 { return 0; }

///* Return the device characteristic flags supported by a tmp-file.
extern "C" fn tmp_device_characteristics(p_file: *mut Sqlite3File) -> i32 {
    return 0;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FsVfsT {
    base: Sqlite3Vfs,
    p_file_list: *mut FsRealFile,
    p_parent: *mut Sqlite3Vfs,
}

static mut tmp_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(tmp_close),
        x_read: Some(tmp_read),
        x_write: Some(tmp_write),
        x_truncate: Some(tmp_truncate),
        x_sync: Some(tmp_sync),
        x_file_size: Some(tmp_file_size),
        x_lock: Some(tmp_lock),
        x_unlock: Some(tmp_unlock),
        x_check_reserved_lock: Some(tmp_check_reserved_lock),
        x_file_control: Some(tmp_file_control),
        x_sector_size: Some(tmp_sector_size),
        x_device_characteristics: Some(tmp_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };

static mut fs_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(fs_close),
        x_read: Some(fs_read),
        x_write: Some(fs_write),
        x_truncate: Some(fs_truncate),
        x_sync: Some(fs_sync),
        x_file_size: Some(fs_file_size),
        x_lock: Some(fs_lock),
        x_unlock: Some(fs_unlock),
        x_check_reserved_lock: Some(fs_check_reserved_lock),
        x_file_control: Some(fs_file_control),
        x_sector_size: Some(fs_sector_size),
        x_device_characteristics: Some(fs_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };

///* Method declarations for fs_vfs.
extern "C" fn fs_open(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_file: *mut Sqlite3File, flags: i32, p_out_flags: *mut i32) -> i32 {
    unsafe {
        let mut p_fs_vfs: *mut FsVfsT = core::ptr::null_mut();
        let mut p: *mut FsFile = core::ptr::null_mut();
        let mut p_real: *mut FsRealFile = core::ptr::null_mut();
        let mut e_type: i32 = 0;
        let mut n_name: i32 = 0;
        let mut rc: i32 = 0;
        let mut p2: *mut TmpFile = core::ptr::null_mut();
        let mut real_flags: i32 = 0;
        let mut size: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut p_real_file: *mut Sqlite3File = core::ptr::null_mut();
        let mut p_parent: *mut Sqlite3Vfs = core::ptr::null_mut();
        let mut z_s: [u8; 4] = [0; 4];
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s3:
                {
                match __state {
                    0 => { p_fs_vfs = p_vfs as *mut FsVfsT; __state = 3; }
                    2 => {
                        if !(p_real).is_null() {
                            __state = 62;
                        } else { __state = 61; }
                    }
                    3 => { p = p_file as *mut FsFile; __state = 4; }
                    4 => { p_real = core::ptr::null_mut(); __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { rc = 0; __state = 8; }
                    8 => {
                        if 0 == flags & (256 | 2048) {
                            __state = 10;
                        } else { __state = 9; }
                    }
                    9 => {
                        e_type = if flags & 256 != 0 { 1 } else { 2 };
                        __state = 14;
                    }
                    10 => { p2 = p_file as *mut TmpFile; __state = 11; }
                    11 => {
                        unsafe {
                            memset(p2 as *mut (), 0,
                                core::mem::size_of::<TmpFile>() as u64)
                        };
                        __state = 12;
                    }
                    12 => {
                        unsafe {
                            (*p2).base.p_methods =
                                &raw mut tmp_io_methods as *const Sqlite3IoMethods
                        };
                        __state = 13;
                    }
                    13 => { return 0; }
                    14 => {
                        unsafe {
                            (*p).base.p_methods =
                                &raw mut fs_io_methods as *const Sqlite3IoMethods
                        };
                        __state = 15;
                    }
                    15 => { unsafe { (*p).e_type = e_type }; __state = 16; }
                    16 => {
                        if !(unsafe {
                                                    strlen(c"-journal".as_ptr() as *mut i8 as *const i8)
                                                } == 8 as u64) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fsOpen".as_ptr() as *const i8,
                                    c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 609,
                                    c"strlen(\"-journal\")==8".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 17;
                    }
                    17 => {
                        n_name =
                            unsafe { strlen(z_name) } as i32 -
                                if e_type == 2 { 8 } else { 0 };
                        __state = 18;
                    }
                    18 => {
                        p_real = unsafe { (*p_fs_vfs).p_file_list };
                        __state = 19;
                    }
                    19 => { __state = 21; }
                    20 => {
                        if (p_real).is_null() as i32 != 0 {
                            __state = 25;
                        } else { __state = 24; }
                    }
                    21 => {
                        if !(p_real).is_null() &&
                                unsafe {
                                        strncmp(unsafe { (*p_real).z_name }, z_name, n_name as u64)
                                    } != 0 {
                            __state = 22;
                        } else { __state = 20; }
                    }
                    22 => { __state = 23; }
                    23 => {
                        p_real = unsafe { (*p_real).p_next };
                        __state = 21;
                    }
                    24 => { __state = 2; }
                    25 => { real_flags = flags & !256 | 512; __state = 26; }
                    26 => { __state = 27; }
                    27 => { __state = 28; }
                    28 => {
                        p_parent = unsafe { (*p_fs_vfs).p_parent };
                        __state = 29;
                    }
                    29 => {
                        if !(e_type == 1) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fsOpen".as_ptr() as *const i8,
                                    c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 619,
                                    c"eType==DATABASE_FILE".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 30;
                    }
                    30 => {
                        p_real =
                            unsafe {
                                    sqlite3_malloc((core::mem::size_of::<FsRealFile>() as u64 +
                                                unsafe { (*p_parent).sz_os_file } as u64) as i32)
                                } as *mut FsRealFile;
                        __state = 31;
                    }
                    31 => {
                        if (p_real).is_null() as i32 != 0 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        unsafe {
                            memset(p_real as *mut (), 0,
                                core::mem::size_of::<FsRealFile>() as u64 +
                                    unsafe { (*p_parent).sz_os_file } as u64)
                        };
                        __state = 35;
                    }
                    33 => { rc = 7; __state = 34; }
                    34 => { __state = 2; }
                    35 => {
                        unsafe { (*p_real).z_name = z_name };
                        __state = 36;
                    }
                    36 => {
                        unsafe {
                            (*p_real).p_file =
                                unsafe { &raw mut *p_real.offset(1 as isize) } as
                                    *mut Sqlite3File
                        };
                        __state = 37;
                    }
                    37 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*p_parent).x_open.unwrap()
                                    })(p_parent, z_name, unsafe { (*p_real).p_file },
                                    real_flags, p_out_flags)
                            };
                        __state = 38;
                    }
                    38 => {
                        if rc != 0 { __state = 40; } else { __state = 39; }
                    }
                    39 => {
                        p_real_file = unsafe { (*p_real).p_file };
                        __state = 41;
                    }
                    40 => { __state = 2; }
                    41 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_file_size.unwrap()
                                    })(p_real_file, &mut size)
                            };
                        __state = 42;
                    }
                    42 => {
                        if rc != 0 { __state = 44; } else { __state = 43; }
                    }
                    43 => {
                        if size == 0 as i64 { __state = 46; } else { __state = 47; }
                    }
                    44 => { __state = 2; }
                    45 => {
                        if rc == 0 { __state = 56; } else { __state = 24; }
                    }
                    46 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_write.unwrap()
                                    })(p_real_file,
                                    b"\x00\x00".as_ptr() as *mut i8 as *const (), 1,
                                    (10485760 - 1) as i64)
                            };
                        __state = 48;
                    }
                    47 => { __state = 49; }
                    48 => {
                        unsafe { (*p_real).n_blob = 10485760 };
                        __state = 45;
                    }
                    49 => {
                        unsafe { (*p_real).n_blob = size as i32 };
                        __state = 50;
                    }
                    50 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_read.unwrap()
                                    })(p_real_file,
                                    &raw mut z_s[0 as usize] as *mut u8 as *mut (), 4, 0)
                            };
                        __state = 51;
                    }
                    51 => {
                        unsafe {
                            (*p_real).n_database =
                                ((z_s[0 as usize] as i32) << 24) +
                                            ((z_s[1 as usize] as i32) << 16) +
                                        ((z_s[2 as usize] as i32) << 8) + z_s[3 as usize] as i32
                        };
                        __state = 52;
                    }
                    52 => {
                        if rc == 0 { __state = 53; } else { __state = 45; }
                    }
                    53 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_read.unwrap()
                                    })(p_real_file,
                                    &raw mut z_s[0 as usize] as *mut u8 as *mut (), 4,
                                    (unsafe { (*p_real).n_blob } - 4) as i64)
                            };
                        __state = 54;
                    }
                    54 => {
                        if z_s[0 as usize] != 0 || z_s[1 as usize] != 0 ||
                                    z_s[2 as usize] != 0 || z_s[3 as usize] != 0 {
                            __state = 55;
                        } else { __state = 45; }
                    }
                    55 => {
                        unsafe {
                            (*p_real).n_journal = unsafe { (*p_real).n_blob }
                        };
                        __state = 45;
                    }
                    56 => {
                        unsafe {
                            (*p_real).p_next = unsafe { (*p_fs_vfs).p_file_list }
                        };
                        __state = 57;
                    }
                    57 => {
                        if !(unsafe { (*p_real).p_next }).is_null() {
                            __state = 59;
                        } else { __state = 58; }
                    }
                    58 => {
                        unsafe {
                            (*p_real).pp_this = unsafe { &mut (*p_fs_vfs).p_file_list }
                        };
                        __state = 60;
                    }
                    59 => {
                        unsafe {
                            (*unsafe { (*p_real).p_next }).pp_this =
                                unsafe { &mut (*p_real).p_next }
                        };
                        __state = 58;
                    }
                    60 => {
                        unsafe { (*p_fs_vfs).p_file_list = p_real };
                        __state = 24;
                    }
                    61 => { return rc; }
                    62 => {
                        if rc == 0 { __state = 63; } else { __state = 64; }
                    }
                    63 => { unsafe { (*p).p_real = p_real }; __state = 65; }
                    64 => {
                        if !(unsafe {
                                            (*unsafe { (*p_real).p_file }).p_methods
                                        }).is_null() {
                            __state = 67;
                        } else { __state = 66; }
                    }
                    65 => {
                        {
                            let __p = unsafe { &mut (*p_real).n_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 61;
                    }
                    66 => {
                        unsafe { sqlite3_free(p_real as *mut ()) };
                        __state = 61;
                    }
                    67 => {
                        unsafe {
                            (unsafe {
                                    (*unsafe {
                                                        (*unsafe { (*p_real).p_file }).p_methods
                                                    }).x_close.unwrap()
                                })(unsafe { (*p_real).p_file })
                        };
                        __state = 66;
                    }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}

///* Delete the file located at zPath. If the dirSync argument is true,
///* ensure the file-system modifications are synced to disk before
///* returning.
extern "C" fn fs_delete(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    let mut rc: i32 = 0;
    let p_fs_vfs: *const FsVfsT = p_vfs as *mut FsVfsT as *const FsVfsT;
    let mut p_real: *mut FsRealFile = core::ptr::null_mut();
    let mut p_f: *mut Sqlite3File = core::ptr::null_mut();
    let n_name: i32 = unsafe { strlen(z_path) } as i32 - 8;
    if !(unsafe { strlen(c"-journal".as_ptr() as *mut i8 as *const i8) } ==
                            8 as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsDelete".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 693,
                c"strlen(\"-journal\")==8".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                                strcmp(c"-journal".as_ptr() as *mut i8 as *const i8,
                                    unsafe { &*z_path.offset(n_name as isize) })
                            } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsDelete".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 694,
                c"strcmp(\"-journal\", &zPath[nName])==0".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    p_real = unsafe { (*p_fs_vfs).p_file_list };
    {
        '__b4: loop {
            if !(!(p_real).is_null() &&
                            unsafe {
                                    strncmp(unsafe { (*p_real).z_name }, z_path, n_name as u64)
                                } != 0) {
                break '__b4;
            }
            '__c4: loop { break '__c4; }
            p_real = unsafe { (*p_real).p_next };
        }
    }
    if !(p_real).is_null() {
        p_f = unsafe { (*p_real).p_file };
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_f).p_methods }).x_write.unwrap()
                    })(p_f,
                    b"\x00\x00\x00\x00\x00".as_ptr() as *mut i8 as *const (), 4,
                    (unsafe { (*p_real).n_blob } - 512) as i64)
            };
        if rc == 0 { unsafe { (*p_real).n_journal = 0 }; }
    }
    return rc;
}

///* Test for access permissions. Return true if the requested permission
///* is available, or false otherwise.
extern "C" fn fs_access(p_vfs: *mut Sqlite3Vfs, z_path: *const i8, flags: i32,
    p_res_out: *mut i32) -> i32 {
    let p_fs_vfs: *const FsVfsT = p_vfs as *mut FsVfsT as *const FsVfsT;
    let mut p_real: *const FsRealFile = core::ptr::null();
    let mut is_journal: i32 = 0;
    let mut n_name: i32 = unsafe { strlen(z_path) } as i32;
    if flags != 0 {
        let p_parent: *mut Sqlite3Vfs =
            unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
        return unsafe {
                (unsafe {
                        (*p_parent).x_access.unwrap()
                    })(p_parent, z_path, flags, p_res_out)
            };
    }
    if !(unsafe { strlen(c"-journal".as_ptr() as *mut i8 as *const i8) } ==
                            8 as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsAccess".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 728,
                c"strlen(\"-journal\")==8".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if n_name > 8 &&
            unsafe {
                    strcmp(c"-journal".as_ptr() as *mut i8 as *const i8,
                        unsafe { &*z_path.offset((n_name - 8) as isize) })
                } == 0 {
        n_name -= 8;
        is_journal = 1;
    }
    p_real = unsafe { (*p_fs_vfs).p_file_list };
    {
        '__b5: loop {
            if !(!(p_real).is_null() &&
                            unsafe {
                                    strncmp(unsafe { (*p_real).z_name }, z_path, n_name as u64)
                                } != 0) {
                break '__b5;
            }
            '__c5: loop { break '__c5; }
            p_real = unsafe { (*p_real).p_next };
        }
    }
    unsafe {
        *p_res_out =
            (!(p_real).is_null() &&
                    ((is_journal == 0) as i32 != 0 ||
                        unsafe { (*p_real).n_journal } > 0)) as i32
    };
    return 0;
}

///* Populate buffer zOut with the full canonical pathname corresponding
///* to the pathname in zPath. zOut is guaranteed to point to a buffer
///* of at least (FS_MAX_PATHNAME+1) bytes.
extern "C" fn fs_full_pathname(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    return unsafe {
            (unsafe {
                    (*p_parent).x_full_pathname.unwrap()
                })(p_parent, z_path, n_out, z_out)
        };
}

///* Open the dynamic library located at zPath and return a handle.
extern "C" fn fs_dl_open(p_vfs: *mut Sqlite3Vfs, z_path: *const i8)
    -> *mut () {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    return unsafe {
            (unsafe { (*p_parent).x_dl_open.unwrap() })(p_parent, z_path)
        };
}

///* Populate the buffer zErrMsg (size nByte bytes) with a human readable
///* utf-8 string describing the most recent error encountered associated 
///* with dynamic libraries.
extern "C" fn fs_dl_error(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    unsafe {
        (unsafe {
                (*p_parent).x_dl_error.unwrap()
            })(p_parent, n_byte, z_err_msg)
    };
}

///* Return a pointer to the symbol zSymbol in the dynamic library pHandle.
extern "C" fn fs_dl_sym(p_vfs: *mut Sqlite3Vfs, p_h: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    return unsafe {
            (unsafe { (*p_parent).x_dl_sym.unwrap() })(p_parent, p_h, z_sym)
        };
}

///* Close the dynamic library handle pHandle.
extern "C" fn fs_dl_close(p_vfs: *mut Sqlite3Vfs, p_handle: *mut ()) -> () {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    unsafe {
        (unsafe { (*p_parent).x_dl_close.unwrap() })(p_parent, p_handle)
    };
}

///* Populate the buffer pointed to by zBufOut with nByte bytes of 
///* random data.
extern "C" fn fs_randomness(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    return unsafe {
            (unsafe {
                    (*p_parent).x_randomness.unwrap()
                })(p_parent, n_byte, z_buf_out)
        };
}

///* Sleep for nMicro microseconds. Return the number of microseconds 
///* actually slept.
extern "C" fn fs_sleep(p_vfs: *mut Sqlite3Vfs, n_micro: i32) -> i32 {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    return unsafe {
            (unsafe { (*p_parent).x_sleep.unwrap() })(p_parent, n_micro)
        };
}

///* Return the current time as a Julian Day number in *pTimeOut.
extern "C" fn fs_current_time(p_vfs: *mut Sqlite3Vfs, p_time_out: *mut f64)
    -> i32 {
    let p_parent: *mut Sqlite3Vfs =
        unsafe { (*(p_vfs as *mut FsVfsT)).p_parent };
    return unsafe {
            (unsafe {
                    (*p_parent).x_current_time.unwrap()
                })(p_parent, p_time_out)
        };
}

static mut fs_vfs: FsVfsT =
    FsVfsT {
        base: Sqlite3Vfs {
            i_version: 1,
            sz_os_file: 0,
            mx_pathname: 0,
            p_next: core::ptr::null_mut(),
            z_name: c"fs".as_ptr() as *const i8,
            p_app_data: core::ptr::null_mut(),
            x_open: Some(fs_open),
            x_delete: Some(fs_delete),
            x_access: Some(fs_access),
            x_full_pathname: Some(fs_full_pathname),
            x_dl_open: Some(fs_dl_open),
            x_dl_error: Some(fs_dl_error),
            x_dl_sym: Some(fs_dl_sym),
            x_dl_close: Some(fs_dl_close),
            x_randomness: Some(fs_randomness),
            x_sleep: Some(fs_sleep),
            x_current_time: Some(fs_current_time),
            x_get_last_error: None,
            x_current_time_int64: None,
            x_set_system_call: None,
            x_get_system_call: None,
            x_next_system_call: None,
        },
        p_file_list: core::ptr::null_mut(),
        p_parent: core::ptr::null_mut(),
    };

///* This procedure registers the fs vfs with SQLite. If the argument is
///* true, the fs vfs becomes the new default vfs. It is the only publicly
///* available function in this file.
#[unsafe(no_mangle)]
pub extern "C" fn fs_register() -> i32 {
    unsafe {
        if !(fs_vfs.p_parent).is_null() { return 0; }
        fs_vfs.p_parent = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        fs_vfs.base.mx_pathname = unsafe { (*fs_vfs.p_parent).mx_pathname };
        fs_vfs.base.sz_os_file =
            if core::mem::size_of::<TmpFile>() as u64 >
                        core::mem::size_of::<FsFile>() as u64 {
                    core::mem::size_of::<TmpFile>() as u64
                } else { core::mem::size_of::<FsFile>() as u64 } as i32;
        return unsafe { sqlite3_vfs_register(&mut fs_vfs.base, 0) };
    }
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
