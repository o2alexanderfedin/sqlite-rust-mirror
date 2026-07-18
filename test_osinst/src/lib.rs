#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
type DarwinSizeT = u64;
type DarwinTimeT = i64;
type Int32T = i32;
type DarwinSusecondsT = Int32T;
#[repr(C)]
#[derive(Copy, Clone)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VfslogVfs {
    base: Sqlite3Vfs,
    p_vfs: *mut Sqlite3Vfs,
    i_next_file_id: i32,
    p_log: *mut Sqlite3File,
    i_offset: Sqlite3Int64,
    n_buf: i32,
    a_buf: [i8; 8192],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VfslogFile {
    base: Sqlite3File,
    p_real: *mut Sqlite3File,
    p_vfslog: *mut Sqlite3Vfs,
    i_file_id: i32,
}
extern "C" fn vfslog_time() -> Sqlite3Uint64 {
    let mut s_time: Timeval = unsafe { core::mem::zeroed() };
    unsafe { gettimeofday(&mut s_time, core::ptr::null_mut()) };
    return s_time.tv_usec as Sqlite3Uint64 +
            s_time.tv_sec as Sqlite3Uint64 * 1000000 as Sqlite3Uint64;
}
extern "C" fn vfslog_flush(p: &mut VfslogVfs) -> () {
    if (*p).n_buf != 0 {
        unsafe {
            (unsafe {
                    (*unsafe { (*(*p).p_log).p_methods }).x_write.unwrap()
                })((*p).p_log,
                &raw mut (*p).a_buf[0 as usize] as *mut i8 as *const (),
                (*p).n_buf, (*p).i_offset)
        };
        (*p).i_offset += (*p).n_buf as Sqlite3Int64;
        (*p).n_buf = 0;
    }
}
extern "C" fn put32bits(p: *mut u8, v: u32) -> () {
    unsafe { *p.offset(0 as isize) = (v >> 24) as u8 };
    unsafe { *p.offset(1 as isize) = (v >> 16) as u8 };
    unsafe { *p.offset(2 as isize) = (v >> 8) as u8 };
    unsafe { *p.offset(3 as isize) = v as u8 };
}
extern "C" fn vfslog_call(p_vfs: *mut Sqlite3Vfs, e_event: i32, i_fileid: i32,
    n_click: Sqlite3Int64, return_code: i32, size: i32, offset: i32) -> () {
    let p: *mut VfslogVfs = p_vfs as *mut VfslogVfs;
    let mut z_rec: *mut u8 = core::ptr::null_mut();
    if (24 + unsafe { (*p).n_buf }) as u64 >
            core::mem::size_of::<[i8; 8192]>() as u64 {
        vfslog_flush(unsafe { &mut *p });
    }
    z_rec =
        unsafe { &raw mut (*p).a_buf[unsafe { (*p).n_buf } as usize] } as
            *mut u8;
    put32bits(unsafe { &mut *z_rec.offset(0 as isize) }, e_event as u32);
    put32bits(unsafe { &mut *z_rec.offset(4 as isize) }, i_fileid as u32);
    put32bits(unsafe { &mut *z_rec.offset(8 as isize) },
        (n_click & 65535 as Sqlite3Int64) as u32);
    put32bits(unsafe { &mut *z_rec.offset(12 as isize) }, return_code as u32);
    put32bits(unsafe { &mut *z_rec.offset(16 as isize) }, size as u32);
    put32bits(unsafe { &mut *z_rec.offset(20 as isize) }, offset as u32);
    unsafe { (*p).n_buf += 24 };
}
extern "C" fn vfslog_close(p_file: *mut Sqlite3File) -> i32 {
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut rc: i32 = 0;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    if !(unsafe { (*unsafe { (*p).p_real }).p_methods }).is_null() {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*unsafe { (*p).p_real }).p_methods
                                        }).x_close.unwrap()
                    })(unsafe { (*p).p_real })
            };
    }
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 3, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, 0);
    return rc;
}
extern "C" fn vfslog_read(p_file: *mut Sqlite3File, z_buf: *mut (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_read.unwrap()
                })(unsafe { (*p).p_real }, z_buf, i_amt, i_ofst)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 14, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, i_amt, i_ofst as i32);
    return rc;
}
extern "C" fn vfslog_write(p_file: *mut Sqlite3File, z: *const (), i_amt: i32,
    i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_write.unwrap()
                })(unsafe { (*p).p_real }, z, i_amt, i_ofst)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 20, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, i_amt, i_ofst as i32);
    return rc;
}
extern "C" fn vfslog_truncate(p_file: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_truncate.unwrap()
                })(unsafe { (*p).p_real }, size)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 18, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, size as i32);
    return rc;
}
extern "C" fn vfslog_sync(p_file: *mut Sqlite3File, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_sync.unwrap()
                })(unsafe { (*p).p_real }, flags)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 17, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, flags, 0);
    return rc;
}
extern "C" fn vfslog_file_size(p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_file_size.unwrap()
                })(unsafe { (*p).p_real }, p_size)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 8, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, unsafe { *p_size } as i32);
    return rc;
}
extern "C" fn vfslog_lock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_lock.unwrap()
                })(unsafe { (*p).p_real }, e_lock)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 11, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, e_lock, 0);
    return rc;
}
extern "C" fn vfslog_unlock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_unlock.unwrap()
                })(unsafe { (*p).p_real }, e_lock)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 19, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, e_lock, 0);
    return rc;
}
extern "C" fn vfslog_check_reserved_lock(p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(unsafe { (*p).p_real }, p_res_out)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 2, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, unsafe { *p_res_out }, 0);
    return rc;
}
extern "C" fn vfslog_file_control(p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    let rc: i32 =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_file_control.unwrap()
                })(unsafe { (*p).p_real }, op, p_arg)
        };
    if op == 12 && rc == 0 {
        unsafe {
            *(p_arg as *mut *mut i8) =
                unsafe {
                    sqlite3_mprintf(c"vfslog/%z".as_ptr() as *mut i8 as
                            *const i8, unsafe { *(p_arg as *mut *mut i8) })
                }
        };
    }
    return rc;
}
extern "C" fn vfslog_sector_size(p_file: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_sector_size.unwrap()
                })(unsafe { (*p).p_real })
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 15, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, 0);
    return rc;
}
extern "C" fn vfslog_device_characteristics(p_file: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_device_characteristics.unwrap()
                })(unsafe { (*p).p_real })
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 6, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, 0);
    return rc;
}
extern "C" fn vfslog_shm_lock(p_file: *mut Sqlite3File, ofst: i32, n: i32,
    flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_shm_lock.unwrap()
                })(unsafe { (*p).p_real }, ofst, n, flags)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 25, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, 0);
    return rc;
}
extern "C" fn vfslog_shm_map(p_file: *mut Sqlite3File, i_region: i32,
    sz_region: i32, is_write: i32, pp: *mut *mut ()) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_shm_map.unwrap()
                })(unsafe { (*p).p_real }, i_region, sz_region, is_write, pp)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 23, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, 0);
    return rc;
}
extern "C" fn vfslog_shm_barrier(p_file: *mut Sqlite3File) -> () {
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*unsafe { (*p).p_real }).p_methods
                                }).x_shm_barrier.unwrap()
            })(unsafe { (*p).p_real })
    };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 26, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, 0, 0, 0);
}
extern "C" fn vfslog_shm_unmap(p_file: *mut Sqlite3File, delete_flag: i32)
    -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VfslogFile = p_file as *mut VfslogFile as *const VfslogFile;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_shm_unmap.unwrap()
                })(unsafe { (*p).p_real }, delete_flag)
        };
    t = vfslog_time() - t;
    vfslog_call(unsafe { (*p).p_vfslog }, 22, unsafe { (*p).i_file_id },
        t as Sqlite3Int64, rc, 0, 0);
    return rc;
}
static mut vfslog_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 2,
        x_close: Some(vfslog_close),
        x_read: Some(vfslog_read),
        x_write: Some(vfslog_write),
        x_truncate: Some(vfslog_truncate),
        x_sync: Some(vfslog_sync),
        x_file_size: Some(vfslog_file_size),
        x_lock: Some(vfslog_lock),
        x_unlock: Some(vfslog_unlock),
        x_check_reserved_lock: Some(vfslog_check_reserved_lock),
        x_file_control: Some(vfslog_file_control),
        x_sector_size: Some(vfslog_sector_size),
        x_device_characteristics: Some(vfslog_device_characteristics),
        x_shm_map: Some(vfslog_shm_map),
        x_shm_lock: Some(vfslog_shm_lock),
        x_shm_barrier: Some(vfslog_shm_barrier),
        x_shm_unmap: Some(vfslog_shm_unmap),
        x_fetch: None,
        x_unfetch: None,
    };
extern "C" fn vfslog_string(p_vfs: *mut Sqlite3Vfs, z_str: *const i8) -> () {
    let p: *mut VfslogVfs = p_vfs as *mut VfslogVfs;
    let mut z_rec: *mut u8 = core::ptr::null_mut();
    let n_str: i32 =
        if !(z_str).is_null() {
            (unsafe { strlen(z_str) }) as i32
        } else { 0 };
    if (4 + n_str + unsafe { (*p).n_buf }) as u64 >
            core::mem::size_of::<[i8; 8192]>() as u64 {
        vfslog_flush(unsafe { &mut *p });
    }
    z_rec =
        unsafe { &raw mut (*p).a_buf[unsafe { (*p).n_buf } as usize] } as
            *mut u8;
    put32bits(unsafe { &mut *z_rec.offset(0 as isize) }, n_str as u32);
    if !(z_str).is_null() {
        unsafe {
            memcpy(unsafe { &raw mut *z_rec.offset(4 as isize) } as *mut (),
                z_str as *const (), n_str as u64)
        };
    }
    unsafe { (*p).n_buf += 4 + n_str };
}
extern "C" fn vfslog_open(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_file: *mut Sqlite3File, flags: i32, p_out_flags: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
        let p: *mut VfslogFile = p_file as *mut VfslogFile;
        let p_log: *mut VfslogVfs = p_vfs as *mut VfslogVfs;
        unsafe {
            (*p_file).p_methods =
                &raw mut vfslog_io_methods as *const Sqlite3IoMethods
        };
        unsafe {
            (*p).p_real =
                unsafe { &raw mut *p.offset(1 as isize) } as *mut Sqlite3File
        };
        unsafe { (*p).p_vfslog = p_vfs };
        unsafe {
            (*p).i_file_id =
                {
                    let __p = unsafe { &mut (*p_log).i_next_file_id };
                    *__p += 1;
                    *__p
                }
        };
        t = vfslog_time();
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*(p_vfs as *mut VfslogVfs)).p_vfs
                                        }).x_open.unwrap()
                    })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, z_name,
                    unsafe { (*p).p_real }, flags, p_out_flags)
            };
        t = vfslog_time() - t;
        vfslog_call(p_vfs, 12, unsafe { (*p).i_file_id }, t as Sqlite3Int64,
            rc, 0, 0);
        vfslog_string(p_vfs, z_name);
        return rc;
    }
}
extern "C" fn vfslog_delete(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_delete.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, z_path,
                dir_sync)
        };
    t = vfslog_time() - t;
    vfslog_call(p_vfs, 5, 0, t as Sqlite3Int64, rc, dir_sync, 0);
    vfslog_string(p_vfs, z_path);
    return rc;
}
extern "C" fn vfslog_access(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    flags: i32, p_res_out: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    t = vfslog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_access.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, z_path,
                flags, p_res_out)
        };
    t = vfslog_time() - t;
    vfslog_call(p_vfs, 1, 0, t as Sqlite3Int64, rc, flags,
        unsafe { *p_res_out });
    vfslog_string(p_vfs, z_path);
    return rc;
}
extern "C" fn vfslog_full_pathname(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_full_pathname.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, z_path,
                n_out, z_out)
        };
}
extern "C" fn vfslog_dl_open(p_vfs: *mut Sqlite3Vfs, z_path: *const i8)
    -> *mut () {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_dl_open.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, z_path)
        };
}
extern "C" fn vfslog_dl_error(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*(p_vfs as *mut VfslogVfs)).p_vfs
                                }).x_dl_error.unwrap()
            })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, n_byte,
            z_err_msg)
    };
}
extern "C" fn vfslog_dl_sym(p_vfs: *mut Sqlite3Vfs, p: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_dl_sym.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, p, z_sym)
        };
}
extern "C" fn vfslog_dl_close(p_vfs: *mut Sqlite3Vfs, p_handle: *mut ())
    -> () {
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*(p_vfs as *mut VfslogVfs)).p_vfs
                                }).x_dl_close.unwrap()
            })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, p_handle)
    };
}
extern "C" fn vfslog_randomness(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_randomness.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, n_byte,
                z_buf_out)
        };
}
extern "C" fn vfslog_sleep(p_vfs: *mut Sqlite3Vfs, n_micro: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_sleep.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, n_micro)
        };
}
extern "C" fn vfslog_current_time(p_vfs: *mut Sqlite3Vfs,
    p_time_out: *mut f64) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_current_time.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, p_time_out)
        };
}
extern "C" fn vfslog_get_last_error(p_vfs: *mut Sqlite3Vfs, a: i32,
    b: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_get_last_error.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, a, b)
        };
}
extern "C" fn vfslog_current_time_int64(p_vfs: *mut Sqlite3Vfs,
    p: *mut Sqlite3Int64) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VfslogVfs)).p_vfs
                                    }).x_current_time_int64.unwrap()
                })(unsafe { (*(p_vfs as *mut VfslogVfs)).p_vfs }, p)
        };
}
static mut vfslog_vfs: Sqlite3Vfs =
    Sqlite3Vfs {
        i_version: 1,
        sz_os_file: core::mem::size_of::<VfslogFile>() as i32,
        mx_pathname: 512,
        p_next: core::ptr::null_mut(),
        z_name: core::ptr::null(),
        p_app_data: core::ptr::null_mut(),
        x_open: Some(vfslog_open),
        x_delete: Some(vfslog_delete),
        x_access: Some(vfslog_access),
        x_full_pathname: Some(vfslog_full_pathname),
        x_dl_open: Some(vfslog_dl_open),
        x_dl_error: Some(vfslog_dl_error),
        x_dl_sym: Some(vfslog_dl_sym),
        x_dl_close: Some(vfslog_dl_close),
        x_randomness: Some(vfslog_randomness),
        x_sleep: Some(vfslog_sleep),
        x_current_time: Some(vfslog_current_time),
        x_get_last_error: Some(vfslog_get_last_error),
        x_current_time_int64: Some(vfslog_current_time_int64),
        x_set_system_call: None,
        x_get_system_call: None,
        x_next_system_call: None,
    };
extern "C" fn vfslog_finalize(p: *mut VfslogVfs) -> () {
    if !(unsafe { (*unsafe { (*p).p_log }).p_methods }).is_null() {
        vfslog_flush(unsafe { &mut *p });
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_log }).p_methods
                                    }).x_close.unwrap()
                })(unsafe { (*p).p_log })
        };
    }
    unsafe { sqlite3_free(p as *mut ()) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vfslog_finalize(z_vfs_1: *const i8) -> i32 {
    let mut p_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
    p_vfs = unsafe { sqlite3_vfs_find(z_vfs_1) };
    if (p_vfs).is_null() as i32 != 0 ||
            unsafe { (*p_vfs).x_open } != Some(vfslog_open) {
        return 1;
    }
    unsafe { sqlite3_vfs_unregister(p_vfs) };
    vfslog_finalize(p_vfs as *mut VfslogVfs);
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vfslog_new(z_vfs_1: *const i8,
    z_parent_vfs_1: *const i8, z_log_1: *const i8) -> i32 {
    unsafe {
        let mut p: *mut VfslogVfs = core::ptr::null_mut();
        let mut p_parent: *mut Sqlite3Vfs = core::ptr::null_mut();
        let mut n_byte: i32 = 0;
        let mut flags: i32 = 0;
        let mut rc: i32 = 0;
        let mut z_file: *mut i8 = core::ptr::null_mut();
        let mut n_vfs: i32 = 0;
        p_parent = unsafe { sqlite3_vfs_find(z_parent_vfs_1) };
        if (p_parent).is_null() as i32 != 0 { return 1; }
        n_vfs = unsafe { strlen(z_vfs_1) } as i32;
        n_byte =
            (core::mem::size_of::<VfslogVfs>() as u64 +
                                    unsafe { (*p_parent).sz_os_file } as u64 + n_vfs as u64 +
                            1 as u64 + unsafe { (*p_parent).mx_pathname } as u64 +
                    1 as u64) as i32;
        p = unsafe { sqlite3_malloc(n_byte) } as *mut VfslogVfs;
        unsafe { memset(p as *mut (), 0, n_byte as u64) };
        unsafe { (*p).p_vfs = p_parent };
        unsafe {
            (*p).p_log =
                unsafe { &raw mut *p.offset(1 as isize) } as *mut Sqlite3File
        };
        unsafe {
            memcpy(unsafe { &raw mut (*p).base } as *mut (),
                &raw mut vfslog_vfs as *const (),
                core::mem::size_of::<Sqlite3Vfs>() as u64)
        };
        unsafe {
            (*p).base.z_name =
                unsafe {
                        (unsafe { (*p).p_log } as
                                *mut i8).offset(unsafe { (*p_parent).sz_os_file } as isize)
                    } as *const i8
        };
        unsafe { (*p).base.sz_os_file += unsafe { (*p_parent).sz_os_file } };
        unsafe {
            memcpy(unsafe { (*p).base.z_name } as *mut i8 as *mut (),
                z_vfs_1 as *const (), n_vfs as u64)
        };
        z_file =
            unsafe {
                    &raw const *unsafe {
                                (*p).base.z_name.offset((n_vfs + 1) as isize)
                            }
                } as *mut i8;
        unsafe {
            (unsafe {
                    (*p_parent).x_full_pathname.unwrap()
                })(p_parent, z_log_1, unsafe { (*p_parent).mx_pathname },
                z_file)
        };
        flags = 2 | 4 | 16384;
        unsafe {
            (unsafe {
                    (*p_parent).x_delete.unwrap()
                })(p_parent, z_file as *const i8, 0)
        };
        rc =
            unsafe {
                (unsafe {
                        (*p_parent).x_open.unwrap()
                    })(p_parent, z_file as *const i8, unsafe { (*p).p_log },
                    flags, &mut flags)
            };
        if rc == 0 {
            unsafe {
                memcpy(unsafe { &raw mut (*p).a_buf[0 as usize] } as *mut i8
                        as *mut (),
                    c"sqlite_ostrace1.....".as_ptr() as *mut i8 as *const (),
                    20 as u64)
            };
            unsafe { (*p).i_offset = 0 as Sqlite3Int64 };
            unsafe { (*p).n_buf = 20 };
            rc = unsafe { sqlite3_vfs_register(p as *mut Sqlite3Vfs, 1) };
        }
        if rc != 0 { vfslog_finalize(p); }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vfslog_annotate(z_vfs_1: *const i8,
    z_msg_1: *const i8) -> i32 {
    let mut p_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
    p_vfs = unsafe { sqlite3_vfs_find(z_vfs_1) };
    if (p_vfs).is_null() as i32 != 0 ||
            unsafe { (*p_vfs).x_open } != Some(vfslog_open) {
        return 1;
    }
    vfslog_call(p_vfs, 28, 0, 0 as Sqlite3Int64, 0, 0, 0);
    vfslog_string(p_vfs, z_msg_1);
    return 0;
}
extern "C" fn vfslog_eventname(e_event_1: i32) -> *const i8 {
    let mut z_event: *const i8 = core::ptr::null();
    '__s0:
        {
        match e_event_1 {
            3 => { z_event = c"xClose".as_ptr() as *mut i8 as *const i8; }
            14 => { z_event = c"xRead".as_ptr() as *mut i8 as *const i8; }
            20 => { z_event = c"xWrite".as_ptr() as *mut i8 as *const i8; }
            18 => { z_event = c"xTruncate".as_ptr() as *mut i8 as *const i8; }
            17 => { z_event = c"xSync".as_ptr() as *mut i8 as *const i8; }
            8 => { z_event = c"xFilesize".as_ptr() as *mut i8 as *const i8; }
            11 => { z_event = c"xLock".as_ptr() as *mut i8 as *const i8; }
            19 => { z_event = c"xUnlock".as_ptr() as *mut i8 as *const i8; }
            2 => {
                z_event = c"xCheckResLock".as_ptr() as *mut i8 as *const i8;
            }
            7 => {
                z_event = c"xFileControl".as_ptr() as *mut i8 as *const i8;
            }
            15 => {
                z_event = c"xSectorSize".as_ptr() as *mut i8 as *const i8;
            }
            6 => {
                z_event = c"xDeviceChar".as_ptr() as *mut i8 as *const i8;
            }
            12 => { z_event = c"xOpen".as_ptr() as *mut i8 as *const i8; }
            5 => { z_event = c"xDelete".as_ptr() as *mut i8 as *const i8; }
            1 => { z_event = c"xAccess".as_ptr() as *mut i8 as *const i8; }
            9 => {
                z_event = c"xFullPathname".as_ptr() as *mut i8 as *const i8;
            }
            13 => {
                z_event = c"xRandomness".as_ptr() as *mut i8 as *const i8;
            }
            16 => { z_event = c"xSleep".as_ptr() as *mut i8 as *const i8; }
            4 => {
                z_event = c"xCurrentTime".as_ptr() as *mut i8 as *const i8;
            }
            22 => { z_event = c"xShmUnmap".as_ptr() as *mut i8 as *const i8; }
            25 => { z_event = c"xShmLock".as_ptr() as *mut i8 as *const i8; }
            26 => {
                z_event = c"xShmBarrier".as_ptr() as *mut i8 as *const i8;
            }
            23 => { z_event = c"xShmMap".as_ptr() as *mut i8 as *const i8; }
            28 => {
                z_event = c"annotation".as_ptr() as *mut i8 as *const i8;
            }
            _ => {}
        }
    }
    return z_event;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VfslogVtab {
    base: Sqlite3Vtab,
    p_fd: *mut Sqlite3File,
    n_byte: Sqlite3Int64,
    z_file: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VfslogCsr {
    base: Sqlite3VtabCursor,
    i_rowid: Sqlite3Int64,
    i_offset: Sqlite3Int64,
    z_transient: *mut i8,
    n_file: i32,
    az_file: *mut *mut i8,
    a_buf: [u8; 1024],
}
extern "C" fn get32bits(p: *const u8) -> u32 {
    return (((unsafe { *p.offset(0 as isize) } as i32) << 24) +
                        ((unsafe { *p.offset(1 as isize) } as i32) << 16) +
                    ((unsafe { *p.offset(2 as isize) } as i32) << 8) +
                unsafe { *p.offset(3 as isize) } as i32) as u32;
}
extern "C" fn dequote(z: *mut i8) -> () {
    let mut quote: i8 = 0 as i8;
    quote = unsafe { *z.offset(0 as isize) };
    if quote as i32 == '[' as i32 || quote as i32 == '\'' as i32 ||
                quote as i32 == '\"' as i32 || quote as i32 == '`' as i32 {
        let mut i_in: i32 = 1;
        let mut i_out: i32 = 0;
        if quote as i32 == '[' as i32 { quote = ']' as i32 as i8; }
        while unsafe { *z.offset(i_in as isize) } != 0 {
            if unsafe { *z.offset(i_in as isize) } as i32 == quote as i32 {
                if unsafe { *z.offset((i_in + 1) as isize) } as i32 !=
                        quote as i32 {
                    break;
                }
                unsafe {
                    *z.offset({
                                        let __p = &mut i_out;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = quote
                };
                i_in += 2;
            } else {
                unsafe {
                    *z.offset({
                                        let __p = &mut i_out;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) =
                        unsafe {
                            *z.offset({
                                            let __p = &mut i_in;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        }
                };
            }
        }
        unsafe { *z.offset(i_out as isize) = '\u{0}' as i32 as i8 };
    }
}
extern "C" fn vlog_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
    let mut flags: i32 = 0;
    let mut p: *mut VfslogVtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut n_byte: i32 = 0;
    let mut z_file: *mut i8 = core::ptr::null_mut();
    unsafe { *pp_vtab_1 = core::ptr::null_mut() };
    p_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
    n_byte =
        (core::mem::size_of::<VfslogVtab>() as u64 +
                    unsafe { (*p_vfs).sz_os_file } as u64 +
                unsafe { (*p_vfs).mx_pathname } as u64) as i32;
    p = unsafe { sqlite3_malloc(n_byte) } as *mut VfslogVtab;
    if p == core::ptr::null_mut() { return 7; }
    unsafe { memset(p as *mut (), 0, n_byte as u64) };
    unsafe {
        (*p).p_fd =
            unsafe { &raw mut *p.offset(1 as isize) } as *mut Sqlite3File
    };
    unsafe {
        (*p).z_file =
            unsafe {
                (unsafe { (*p).p_fd } as
                        *mut i8).offset(unsafe { (*p_vfs).sz_os_file } as isize)
            }
    };
    z_file =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                unsafe { *argv.offset(3 as isize) })
        };
    if (z_file).is_null() as i32 != 0 {
        unsafe { sqlite3_free(p as *mut ()) };
        return 7;
    }
    dequote(z_file);
    unsafe {
        (unsafe {
                (*p_vfs).x_full_pathname.unwrap()
            })(p_vfs, z_file as *const i8, unsafe { (*p_vfs).mx_pathname },
            unsafe { (*p).z_file })
    };
    unsafe { sqlite3_free(z_file as *mut ()) };
    flags = 2 | 16384;
    rc =
        unsafe {
            (unsafe {
                    (*p_vfs).x_open.unwrap()
                })(p_vfs, unsafe { (*p).z_file } as *const i8,
                unsafe { (*p).p_fd }, flags, &mut flags)
        };
    if rc == 0 {
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_fd }).p_methods
                                    }).x_file_size.unwrap()
                })(unsafe { (*p).p_fd }, unsafe { &mut (*p).n_byte })
        };
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE xxx(event, file, click, rc, size, offset)".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { *pp_vtab_1 = unsafe { &mut (*p).base } };
    } else { unsafe { sqlite3_free(p as *mut ()) }; }
    return rc;
}
extern "C" fn vlog_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    unsafe { (*p_idx_info_1).estimated_cost = 10.0 };
    return 0;
}
extern "C" fn vlog_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut VfslogVtab = p_vtab_1 as *mut VfslogVtab;
    if !(unsafe { (*unsafe { (*p).p_fd }).p_methods }).is_null() {
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_fd }).p_methods
                                    }).x_close.unwrap()
                })(unsafe { (*p).p_fd })
        };
        unsafe { (*unsafe { (*p).p_fd }).p_methods = core::ptr::null() };
    }
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
extern "C" fn vlog_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let mut p_csr: *mut VfslogCsr = core::ptr::null_mut();
    p_csr =
        unsafe { sqlite3_malloc(core::mem::size_of::<VfslogCsr>() as i32) } as
            *mut VfslogCsr;
    if (p_csr).is_null() as i32 != 0 { return 7; }
    unsafe {
        memset(p_csr as *mut (), 0, core::mem::size_of::<VfslogCsr>() as u64)
    };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_csr).base } };
    return 0;
}
extern "C" fn vlog_close(p_cursor_1: *mut Sqlite3VtabCursor) -> i32 {
    let p: *mut VfslogCsr = p_cursor_1 as *mut VfslogCsr;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b2: loop {
            if !(i < unsafe { (*p).n_file }) { break '__b2; }
            '__c2: loop {
                unsafe {
                    sqlite3_free(unsafe {
                                *unsafe { (*p).az_file.offset(i as isize) }
                            } as *mut ())
                };
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p).az_file } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).z_transient } as *mut ()) };
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
extern "C" fn vlog_next(p_cursor_1: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *mut VfslogCsr = p_cursor_1 as *mut VfslogCsr;
    let p: *const VfslogVtab =
        unsafe { (*p_cursor_1).p_vtab } as *mut VfslogVtab as
            *const VfslogVtab;
    let mut rc: i32 = 0;
    let mut n_read: i32 = 0;
    unsafe { sqlite3_free(unsafe { (*p_csr).z_transient } as *mut ()) };
    unsafe { (*p_csr).z_transient = core::ptr::null_mut() };
    n_read = 24;
    if unsafe { (*p_csr).i_offset } + n_read as Sqlite3Int64 <=
            unsafe { (*p).n_byte } {
        let mut e_event: i32 = 0;
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*unsafe { (*p).p_fd }).p_methods
                                        }).x_read.unwrap()
                    })(unsafe { (*p).p_fd },
                    unsafe { &raw mut (*p_csr).a_buf[0 as usize] } as *mut u8 as
                        *mut (), n_read, unsafe { (*p_csr).i_offset })
            };
        e_event =
            get32bits(unsafe { &raw mut (*p_csr).a_buf[0 as usize] } as
                            *mut u8 as *const u8) as i32;
        if rc == 0 && (e_event == 12 || e_event == 5 || e_event == 1) {
            let mut buf: [i8; 4] = [0; 4];
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*unsafe { (*p).p_fd }).p_methods
                                            }).x_read.unwrap()
                        })(unsafe { (*p).p_fd },
                        &raw mut buf[0 as usize] as *mut i8 as *mut (), 4,
                        unsafe { (*p_csr).i_offset } + n_read as Sqlite3Int64)
                };
            n_read += 4;
            if rc == 0 {
                let n_str: i32 =
                    get32bits(&raw mut buf[0 as usize] as *mut u8 as *const u8)
                        as i32;
                let z_str: *mut i8 =
                    unsafe { sqlite3_malloc(n_str + 1) } as *mut i8;
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe {
                                                    (*unsafe { (*p).p_fd }).p_methods
                                                }).x_read.unwrap()
                            })(unsafe { (*p).p_fd }, z_str as *mut (), n_str,
                            unsafe { (*p_csr).i_offset } + n_read as Sqlite3Int64)
                    };
                unsafe {
                    *z_str.offset(n_str as isize) = '\u{0}' as i32 as i8
                };
                n_read += n_str;
                if e_event == 12 {
                    let i_fileid: i32 =
                        get32bits(unsafe { &raw mut (*p_csr).a_buf[4 as usize] } as
                                    *const u8) as i32;
                    if i_fileid >= unsafe { (*p_csr).n_file } {
                        let mut n_new: i32 =
                            (core::mem::size_of::<*mut i8>() as u64 *
                                    (i_fileid + 1) as u64) as i32;
                        unsafe {
                            (*p_csr).az_file =
                                unsafe {
                                        sqlite3_realloc(unsafe { (*p_csr).az_file } as *mut (),
                                            n_new)
                                    } as *mut *mut i8
                        };
                        n_new -=
                            (core::mem::size_of::<*mut i8>() as u64 *
                                    unsafe { (*p_csr).n_file } as u64) as i32;
                        unsafe {
                            memset(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_csr).az_file.offset(unsafe { (*p_csr).n_file } as isize)
                                                }
                                    } as *mut (), 0, n_new as u64)
                        };
                        unsafe { (*p_csr).n_file = i_fileid + 1 };
                    }
                    unsafe {
                        sqlite3_free(unsafe {
                                    *unsafe { (*p_csr).az_file.offset(i_fileid as isize) }
                                } as *mut ())
                    };
                    unsafe {
                        *unsafe { (*p_csr).az_file.offset(i_fileid as isize) } =
                            z_str
                    };
                } else { unsafe { (*p_csr).z_transient = z_str }; }
            }
        }
    }
    unsafe { (*p_csr).i_rowid += 1 as Sqlite3Int64 };
    unsafe { (*p_csr).i_offset += n_read as Sqlite3Int64 };
    return rc;
}
extern "C" fn vlog_eof(p_cursor_1: *mut Sqlite3VtabCursor) -> i32 {
    let p_csr: *const VfslogCsr =
        p_cursor_1 as *mut VfslogCsr as *const VfslogCsr;
    let p: *const VfslogVtab =
        unsafe { (*p_cursor_1).p_vtab } as *mut VfslogVtab as
            *const VfslogVtab;
    return (unsafe { (*p_csr).i_offset } >= unsafe { (*p).n_byte }) as i32;
}
extern "C" fn vlog_filter(p_cursor_1: *mut Sqlite3VtabCursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut Sqlite3Value) -> i32 {
    let p_csr: *mut VfslogCsr = p_cursor_1 as *mut VfslogCsr;
    unsafe { (*p_csr).i_rowid = 0 as Sqlite3Int64 };
    unsafe { (*p_csr).i_offset = 20 as Sqlite3Int64 };
    return vlog_next(p_cursor_1);
}
extern "C" fn vlog_column(p_cursor_1: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let mut val: u32 = 0 as u32;
    let p_csr: *mut VfslogCsr = p_cursor_1 as *mut VfslogCsr;
    if !(i < 7) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"vlogColumn".as_ptr() as *const i8,
                c"test_osinst.c".as_ptr() as *mut i8 as *const i8, 1040,
                c"i<7".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    val =
        get32bits(unsafe { &raw mut (*p_csr).a_buf[(4 * i) as usize] } as
                *const u8);
    '__s3:
        {
        match i {
            0 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx, vfslog_eventname(val as i32), -1,
                            None)
                    };
                    break '__s3;
                }
                {
                    let mut z_str: *const i8 =
                        unsafe { (*p_csr).z_transient } as *const i8;
                    if val != 0 as u32 &&
                            val < unsafe { (*p_csr).n_file } as u32 {
                        z_str =
                            unsafe { *unsafe { (*p_csr).az_file.add(val as usize) } };
                    }
                    unsafe {
                        sqlite3_result_text(ctx, z_str as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s3;
                }
                unsafe { sqlite3_result_int(ctx, val as i32) };
            }
            1 => {
                {
                    let mut z_str: *const i8 =
                        unsafe { (*p_csr).z_transient } as *const i8;
                    if val != 0 as u32 &&
                            val < unsafe { (*p_csr).n_file } as u32 {
                        z_str =
                            unsafe { *unsafe { (*p_csr).az_file.add(val as usize) } };
                    }
                    unsafe {
                        sqlite3_result_text(ctx, z_str as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s3;
                }
                unsafe { sqlite3_result_int(ctx, val as i32) };
            }
            _ => { unsafe { sqlite3_result_int(ctx, val as i32) }; }
        }
    }
    return 0;
}
extern "C" fn vlog_rowid(p_cursor_1: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_csr: *const VfslogCsr =
        p_cursor_1 as *mut VfslogCsr as *const VfslogCsr;
    unsafe { *p_rowid_1 = unsafe { (*p_csr).i_rowid } };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vfslog_register(db: *mut Sqlite3) -> i32 {
    unsafe {
        unsafe {
            sqlite3_create_module(db,
                c"vfslog".as_ptr() as *mut i8 as *const i8,
                &raw mut vfslog_module as *const Sqlite3Module,
                core::ptr::null_mut())
        };
        return 0;
    }
}
static mut vfslog_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: Some(vlog_connect),
        x_connect: Some(vlog_connect),
        x_best_index: Some(vlog_best_index),
        x_disconnect: Some(vlog_disconnect),
        x_destroy: Some(vlog_disconnect),
        x_open: Some(vlog_open),
        x_close: Some(vlog_close),
        x_filter: Some(vlog_filter),
        x_next: Some(vlog_next),
        x_eof: Some(vlog_eof),
        x_column: Some(vlog_column),
        x_rowid: Some(vlog_rowid),
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
    fn gettimeofday(_: *mut Timeval, _: *mut ())
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}