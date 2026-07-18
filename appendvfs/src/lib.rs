#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;
type DarwinSizeT = u64;
type ApndVfs = Sqlite3Vfs;
#[repr(C)]
#[derive(Copy, Clone)]
struct ApndFile {
    base: Sqlite3File,
    i_pg_one: Sqlite3Int64,
    i_mark: Sqlite3Int64,
}
extern "C" fn apnd_close(mut p_file: *mut Sqlite3File) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_close.unwrap()
                })(p_file)
        };
}
extern "C" fn apnd_read(mut p_file: *mut Sqlite3File, z_buf: *mut (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let paf: *const ApndFile = p_file as *mut ApndFile as *const ApndFile;
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_read.unwrap()
                })(p_file, z_buf, i_amt, unsafe { (*paf).i_pg_one } + i_ofst)
        };
}
extern "C" fn apnd_write_mark(paf: *mut ApndFile, p_file_1: *mut Sqlite3File,
    mut i_write_end_1: SqliteInt64) -> i32 {
    let mut i_pg_one: SqliteInt64 = unsafe { (*paf).i_pg_one };
    let mut a: [u8; 25] = [0; 25];
    let mut i: i32 = 8;
    let mut rc: i32 = 0;
    if !(p_file_1 ==
                            unsafe { (paf as *mut ApndFile).offset(1 as isize) } as
                                *mut Sqlite3File) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"apndWriteMark".as_ptr() as *const i8,
                c"appendvfs.c".as_ptr() as *mut i8 as *const i8, 260,
                c"pFile == ORIGFILE(paf)".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        memcpy(&raw mut a[0 as usize] as *mut u8 as *mut (),
            c"Start-Of-SQLite3-".as_ptr() as *mut i8 as *const (), 17 as u64)
    };
    while { let __p = &mut i; *__p -= 1; *__p } >= 0 {
        a[(17 + i) as usize] = (i_pg_one & 255 as SqliteInt64) as u8;
        i_pg_one >>= 8 as SqliteInt64;
    }
    i_write_end_1 += unsafe { (*paf).i_pg_one };
    if 0 ==
            {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_file_1).p_methods }).x_write.unwrap()
                            })(p_file_1, &raw mut a[0 as usize] as *mut u8 as *const (),
                            17 + 8, i_write_end_1)
                    };
                rc
            } {
        unsafe { (*paf).i_mark = i_write_end_1 };
    }
    return rc;
}
extern "C" fn apnd_write(mut p_file: *mut Sqlite3File, z_buf: *const (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let paf: *mut ApndFile = p_file as *mut ApndFile;
    let i_write_end: SqliteInt64 = i_ofst + i_amt as SqliteInt64;
    if i_write_end >= 1073741824 as i64 { return 13; }
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if unsafe { (*paf).i_mark } < 0 as i64 ||
            unsafe { (*paf).i_pg_one } + i_write_end >
                unsafe { (*paf).i_mark } {
        let rc: i32 = apnd_write_mark(paf, p_file, i_write_end);
        if 0 != rc { return rc; }
    }
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_write.unwrap()
                })(p_file, z_buf, i_amt, unsafe { (*paf).i_pg_one } + i_ofst)
        };
}
extern "C" fn apnd_truncate(mut p_file: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    let paf: *mut ApndFile = p_file as *mut ApndFile;
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if 0 != apnd_write_mark(paf, p_file, size) { return 10; }
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_truncate.unwrap()
                })(p_file,
                unsafe { (*paf).i_mark } + (17 + 8) as Sqlite3Int64)
        };
}
extern "C" fn apnd_sync(mut p_file: *mut Sqlite3File, flags: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_sync.unwrap()
                })(p_file, flags)
        };
}
extern "C" fn apnd_file_size(p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let paf: *const ApndFile = p_file as *mut ApndFile as *const ApndFile;
    unsafe {
        *p_size =
            if unsafe { (*paf).i_mark } >= 0 as i64 {
                (unsafe { (*paf).i_mark }) - unsafe { (*paf).i_pg_one }
            } else { 0 as Sqlite3Int64 }
    };
    return 0;
}
extern "C" fn apnd_lock(mut p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_lock.unwrap()
                })(p_file, e_lock)
        };
}
extern "C" fn apnd_unlock(mut p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_unlock.unwrap()
                })(p_file, e_lock)
        };
}
extern "C" fn apnd_check_reserved_lock(mut p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_file).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(p_file, p_res_out)
        };
}
extern "C" fn apnd_file_control(mut p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    let paf: *const ApndFile = p_file as *mut ApndFile as *const ApndFile;
    let mut rc: i32 = 0;
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if op == 5 {
        unsafe {
            *(p_arg as *mut Sqlite3Int64) += unsafe { (*paf).i_pg_one }
        };
    }
    rc =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_file_control.unwrap()
                })(p_file, op, p_arg)
        };
    if rc == 0 && op == 12 {
        unsafe {
            *(p_arg as *mut *mut i8) =
                unsafe {
                    sqlite3_mprintf(c"apnd(%lld)/%z".as_ptr() as *mut i8 as
                            *const i8, unsafe { (*paf).i_pg_one },
                        unsafe { *(p_arg as *mut *mut i8) })
                }
        };
    }
    return rc;
}
extern "C" fn apnd_sector_size(mut p_file: *mut Sqlite3File) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_sector_size.unwrap()
                })(p_file)
        };
}
extern "C" fn apnd_device_characteristics(mut p_file: *mut Sqlite3File)
    -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_file).p_methods
                                    }).x_device_characteristics.unwrap()
                })(p_file)
        };
}
extern "C" fn apnd_shm_map(mut p_file: *mut Sqlite3File, i_pg: i32, pgsz: i32,
    b_extend: i32, pp: *mut *mut ()) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_map.unwrap()
                })(p_file, i_pg, pgsz, b_extend, pp)
        };
}
extern "C" fn apnd_shm_lock(mut p_file: *mut Sqlite3File, offset: i32, n: i32,
    flags: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_lock.unwrap()
                })(p_file, offset, n, flags)
        };
}
extern "C" fn apnd_shm_barrier(mut p_file: *mut Sqlite3File) -> () {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    unsafe {
        (unsafe {
                (*unsafe { (*p_file).p_methods }).x_shm_barrier.unwrap()
            })(p_file)
    };
}
extern "C" fn apnd_shm_unmap(mut p_file: *mut Sqlite3File, delete_flag: i32)
    -> i32 {
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_unmap.unwrap()
                })(p_file, delete_flag)
        };
}
extern "C" fn apnd_fetch(mut p_file: *mut Sqlite3File, i_ofst: Sqlite3Int64,
    i_amt: i32, pp: *mut *mut ()) -> i32 {
    let p: *const ApndFile = p_file as *mut ApndFile as *const ApndFile;
    if unsafe { (*p).i_mark } < 0 as i64 ||
            i_ofst + i_amt as Sqlite3Int64 > unsafe { (*p).i_mark } {
        return 10;
    }
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_fetch.unwrap()
                })(p_file, i_ofst + unsafe { (*p).i_pg_one }, i_amt, pp)
        };
}
extern "C" fn apnd_unfetch(mut p_file: *mut Sqlite3File, i_ofst: Sqlite3Int64,
    p_page: *mut ()) -> i32 {
    let p: *const ApndFile = p_file as *mut ApndFile as *const ApndFile;
    p_file =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_unfetch.unwrap()
                })(p_file, i_ofst + unsafe { (*p).i_pg_one }, p_page)
        };
}
static apnd_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 3,
        x_close: Some(apnd_close),
        x_read: Some(apnd_read),
        x_write: Some(apnd_write),
        x_truncate: Some(apnd_truncate),
        x_sync: Some(apnd_sync),
        x_file_size: Some(apnd_file_size),
        x_lock: Some(apnd_lock),
        x_unlock: Some(apnd_unlock),
        x_check_reserved_lock: Some(apnd_check_reserved_lock),
        x_file_control: Some(apnd_file_control),
        x_sector_size: Some(apnd_sector_size),
        x_device_characteristics: Some(apnd_device_characteristics),
        x_shm_map: Some(apnd_shm_map),
        x_shm_lock: Some(apnd_shm_lock),
        x_shm_barrier: Some(apnd_shm_barrier),
        x_shm_unmap: Some(apnd_shm_unmap),
        x_fetch: Some(apnd_fetch),
        x_unfetch: Some(apnd_unfetch),
    };
extern "C" fn apnd_read_mark(sz: Sqlite3Int64, p_file_1: *mut Sqlite3File)
    -> Sqlite3Int64 {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut i_mark: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut msbs: i32 = 8 * (8 - 1);
    let mut a: [u8; 25] = [0; 25];
    if (17 + 8) as Sqlite3Int64 != sz & 511 as Sqlite3Int64 {
        return -1 as Sqlite3Int64;
    }
    rc =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_file_1).p_methods }).x_read.unwrap()
                })(p_file_1, &raw mut a[0 as usize] as *mut u8 as *mut (),
                17 + 8, sz - (17 + 8) as Sqlite3Int64)
        };
    if rc != 0 { return -1 as Sqlite3Int64; }
    if unsafe {
                memcmp(&raw mut a[0 as usize] as *mut u8 as *const (),
                    c"Start-Of-SQLite3-".as_ptr() as *mut i8 as *const (),
                    17 as u64)
            } != 0 {
        return -1 as Sqlite3Int64;
    }
    i_mark = ((a[17 as usize] as i32 & 127) as Sqlite3Int64) << msbs;
    {
        i = 1;
        '__b1: loop {
            if !(i < 8) { break '__b1; }
            '__c1: loop {
                msbs -= 8;
                i_mark |= (a[(17 + i) as usize] as Sqlite3Int64) << msbs;
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i_mark > sz - (17 + 8) as Sqlite3Int64 - 512 as Sqlite3Int64 {
        return -1 as Sqlite3Int64;
    }
    if i_mark & 511 as Sqlite3Int64 != 0 { return -1 as Sqlite3Int64; }
    return i_mark;
}
static apvfs_sqlite_hdr: [i8; 16] =
    [83 as i8, 81 as i8, 76 as i8, 105 as i8, 116 as i8, 101 as i8, 32 as i8,
            102 as i8, 111 as i8, 114 as i8, 109 as i8, 97 as i8, 116 as i8,
            32 as i8, 51 as i8, 0 as i8];
extern "C" fn apnd_is_appendvfs_database(sz: Sqlite3Int64,
    p_file_1: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let mut z_hdr: [i8; 16] = [0; 16];
    let i_mark: Sqlite3Int64 = apnd_read_mark(sz, p_file_1);
    if i_mark >= 0 as i64 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_file_1).p_methods }).x_read.unwrap()
                    })(p_file_1,
                    &raw mut z_hdr[0 as usize] as *mut i8 as *mut (),
                    core::mem::size_of::<[i8; 16]>() as i32, i_mark)
            };
        if 0 == rc &&
                        unsafe {
                                memcmp(&raw mut z_hdr[0 as usize] as *mut i8 as *const (),
                                    &raw const apvfs_sqlite_hdr[0 as usize] as *const i8 as
                                        *const (), core::mem::size_of::<[i8; 16]>() as u64)
                            } == 0 && sz & 511 as Sqlite3Int64 == (17 + 8) as i64 &&
                sz >= (512 + (17 + 8)) as i64 {
            return 1;
        }
    }
    return 0;
}
extern "C" fn apnd_is_ordinary_database_file(sz: Sqlite3Int64,
    p_file_1: *mut Sqlite3File) -> i32 {
    let mut z_hdr: [i8; 16] = [0; 16];
    if apnd_is_appendvfs_database(sz, p_file_1) != 0 ||
                    sz & 511 as Sqlite3Int64 != 0 as i64 ||
                0 !=
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_file_1).p_methods }).x_read.unwrap()
                            })(p_file_1,
                            &raw mut z_hdr[0 as usize] as *mut i8 as *mut (),
                            core::mem::size_of::<[i8; 16]>() as i32, 0)
                    } ||
            unsafe {
                    memcmp(&raw mut z_hdr[0 as usize] as *mut i8 as *const (),
                        &raw const apvfs_sqlite_hdr[0 as usize] as *const i8 as
                            *const (), core::mem::size_of::<[i8; 16]>() as u64)
                } != 0 {
        return 0;
    } else { return 1; }
}
extern "C" fn apnd_open(p_apnd_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_file: *mut Sqlite3File, flags: i32, p_out_flags: *mut i32) -> i32 {
    let p_apnd_file: *mut ApndFile = p_file as *mut ApndFile;
    let p_base_file: *mut Sqlite3File =
        unsafe { (p_file as *mut ApndFile).offset(1 as isize) } as
            *mut Sqlite3File;
    let p_base_vfs: *mut Sqlite3Vfs =
        unsafe { (*p_apnd_vfs).p_app_data } as *mut Sqlite3Vfs;
    let mut rc: i32 = 0;
    let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
    if flags & 256 == 0 {
        return unsafe {
                (unsafe {
                        (*p_base_vfs).x_open.unwrap()
                    })(p_base_vfs, z_name, p_file, flags, p_out_flags)
            };
    }
    unsafe {
        memset(p_apnd_file as *mut (), 0,
            core::mem::size_of::<ApndFile>() as u64)
    };
    unsafe { (*p_file).p_methods = &apnd_io_methods };
    unsafe { (*p_apnd_file).i_mark = -1 as Sqlite3Int64 };
    rc =
        unsafe {
            (unsafe {
                    (*p_base_vfs).x_open.unwrap()
                })(p_base_vfs, z_name, p_base_file, flags, p_out_flags)
        };
    if rc == 0 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_base_file).p_methods }).x_file_size.unwrap()
                    })(p_base_file, &mut sz)
            };
        if rc != 0 {
            unsafe {
                (unsafe {
                        (*unsafe { (*p_base_file).p_methods }).x_close.unwrap()
                    })(p_base_file)
            };
        }
    }
    if rc != 0 {
        unsafe { (*p_file).p_methods = core::ptr::null() };
        return rc;
    }
    if apnd_is_ordinary_database_file(sz, p_base_file) != 0 {
        unsafe {
            memmove(p_apnd_file as *mut (), p_base_file as *const (),
                unsafe { (*p_base_vfs).sz_os_file } as u64)
        };
        return 0;
    }
    unsafe { (*p_apnd_file).i_pg_one = apnd_read_mark(sz, p_file) };
    if unsafe { (*p_apnd_file).i_pg_one } >= 0 as i64 {
        unsafe { (*p_apnd_file).i_mark = sz - (17 + 8) as Sqlite3Int64 };
        return 0;
    }
    if flags & 4 == 0 {
        unsafe {
            (unsafe {
                    (*unsafe { (*p_base_file).p_methods }).x_close.unwrap()
                })(p_base_file)
        };
        rc = 14;
        unsafe { (*p_file).p_methods = core::ptr::null() };
    } else {
        unsafe {
            (*p_apnd_file).i_pg_one =
                sz + (4096 - 1) as Sqlite3Int64 &
                    !((4096 - 1) as Sqlite3Int64)
        };
    }
    return rc;
}
extern "C" fn apnd_delete(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_delete.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path,
                dir_sync)
        };
}
extern "C" fn apnd_access(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    flags: i32, p_res_out: *mut i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_access.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path,
                flags, p_res_out)
        };
}
extern "C" fn apnd_full_pathname(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_full_pathname.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path,
                n_out, z_out)
        };
}
extern "C" fn apnd_dl_open(p_vfs: *mut Sqlite3Vfs, z_path: *const i8)
    -> *mut () {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_dl_open.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path)
        };
}
extern "C" fn apnd_dl_error(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    unsafe {
        (unsafe {
                (*(unsafe { (*p_vfs).p_app_data } as
                                    *mut Sqlite3Vfs)).x_dl_error.unwrap()
            })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, n_byte,
            z_err_msg)
    };
}
extern "C" fn apnd_dl_sym(p_vfs: *mut Sqlite3Vfs, p: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_dl_sym.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, p,
                z_sym)
        };
}
extern "C" fn apnd_dl_close(p_vfs: *mut Sqlite3Vfs, p_handle: *mut ()) -> () {
    unsafe {
        (unsafe {
                (*(unsafe { (*p_vfs).p_app_data } as
                                    *mut Sqlite3Vfs)).x_dl_close.unwrap()
            })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, p_handle)
    };
}
extern "C" fn apnd_randomness(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_randomness.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, n_byte,
                z_buf_out)
        };
}
extern "C" fn apnd_sleep(p_vfs: *mut Sqlite3Vfs, n_micro: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_sleep.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, n_micro)
        };
}
extern "C" fn apnd_current_time(p_vfs: *mut Sqlite3Vfs, p_time_out: *mut f64)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_current_time.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs,
                p_time_out)
        };
}
extern "C" fn apnd_get_last_error(p_vfs: *mut Sqlite3Vfs, a: i32, b: *mut i8)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_get_last_error.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, a, b)
        };
}
extern "C" fn apnd_current_time_int64(p_vfs: *mut Sqlite3Vfs,
    p: *mut Sqlite3Int64) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_current_time_int64.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, p)
        };
}
extern "C" fn apnd_set_system_call(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_call: unsafe extern "C" fn() -> ()) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_set_system_call.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_name,
                p_call)
        };
}
extern "C" fn apnd_get_system_call(p_vfs: *mut Sqlite3Vfs, z_name: *const i8)
    -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_get_system_call.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_name)
        };
}
extern "C" fn apnd_next_system_call(p_vfs: *mut Sqlite3Vfs, z_name: *const i8)
    -> *const i8 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_next_system_call.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_name)
        };
}
static mut apnd_vfs: Sqlite3Vfs =
    Sqlite3Vfs {
        i_version: 3,
        sz_os_file: 0,
        mx_pathname: 1024,
        p_next: core::ptr::null_mut(),
        z_name: c"apndvfs".as_ptr() as *const i8,
        p_app_data: core::ptr::null_mut(),
        x_open: Some(apnd_open),
        x_delete: Some(apnd_delete),
        x_access: Some(apnd_access),
        x_full_pathname: Some(apnd_full_pathname),
        x_dl_open: Some(apnd_dl_open),
        x_dl_error: Some(apnd_dl_error),
        x_dl_sym: Some(apnd_dl_sym),
        x_dl_close: Some(apnd_dl_close),
        x_randomness: Some(apnd_randomness),
        x_sleep: Some(apnd_sleep),
        x_current_time: Some(apnd_current_time),
        x_get_last_error: Some(apnd_get_last_error),
        x_current_time_int64: Some(apnd_current_time_int64),
        x_set_system_call: Some(apnd_set_system_call),
        x_get_system_call: Some(apnd_get_system_call),
        x_next_system_call: Some(apnd_next_system_call),
    };
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_appendvfs_init(db: *const Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_orig: *mut Sqlite3Vfs = core::ptr::null_mut();
        { let _ = p_api_1; };
        { let _ = pz_err_msg_1; };
        { let _ = db; };
        p_orig = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        if p_orig == core::ptr::null_mut() { return 1; }
        apnd_vfs.i_version = unsafe { (*p_orig).i_version };
        apnd_vfs.p_app_data = p_orig as *mut ();
        apnd_vfs.sz_os_file =
            (unsafe { (*p_orig).sz_os_file } as u64 +
                    core::mem::size_of::<ApndFile>() as u64) as i32;
        rc = unsafe { sqlite3_vfs_register(&mut apnd_vfs, 0) };
        if rc == 0 { rc = 0 | 1 << 8; }
        return rc;
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
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}