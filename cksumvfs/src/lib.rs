#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3IoMethods,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

///* Forward declaration of objects used by this utility
type CksmVfs = Sqlite3Vfs;

/// An open file
#[repr(C)]
#[derive(Copy, Clone)]
struct CksmFile {
    base: Sqlite3File,
    z_f_name: *const i8,
    compute_cksm: i8,
    verify_cksm: i8,
    p_partner: *mut CksmFile,
}

///* Methods for CksmFile
extern "C" fn cksm_close(mut p_file: *mut Sqlite3File) -> i32 {
    let p: *mut CksmFile = p_file as *mut CksmFile;
    if !(unsafe { (*p).p_partner }).is_null() {
        if !(unsafe { (*unsafe { (*p).p_partner }).p_partner } == p) as i32 as
                    i64 != 0 {
            unsafe {
                __assert_rtn(c"cksmClose".as_ptr() as *const i8,
                    c"cksumvfs.c".as_ptr() as *mut i8 as *const i8, 398,
                    c"p->pPartner->pPartner==p".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            (*unsafe { (*p).p_partner }).p_partner = core::ptr::null_mut()
        };
        unsafe { (*p).p_partner = core::ptr::null_mut() };
    }
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_close.unwrap()
                })(p_file)
        };
}

///* Set the computeCkSm and verifyCksm flags, if they need to be
///* changed.
extern "C" fn cksm_set_flags(p: &mut CksmFile,
    has_correct_reserve_size_1: i32) -> () {
    if has_correct_reserve_size_1 != (*p).compute_cksm as i32 {
        (*p).compute_cksm =
            {
                (*p).verify_cksm = has_correct_reserve_size_1 as i8;
                (*p).verify_cksm
            };
        if !((*p).p_partner).is_null() {
            unsafe {
                (*(*p).p_partner).verify_cksm =
                    has_correct_reserve_size_1 as i8
            };
            unsafe {
                (*(*p).p_partner).compute_cksm =
                    has_correct_reserve_size_1 as i8
            };
        }
    }
}

/// Compute a checksum on a buffer
extern "C" fn cksm_compute(a: *mut u8, n_byte_1: i32, a_out_1: *mut u8)
    -> () {
    let mut s1: u32 = 0 as u32;
    let mut s2: u32 = 0 as u32;
    let mut a_data: *mut u32 = a as *mut u32;
    let a_end: *mut u32 =
        unsafe { &raw mut *a.offset(n_byte_1 as isize) } as *mut u32;
    let mut x: u32 = 1 as u32;
    if !(n_byte_1 >= 8) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"cksmCompute".as_ptr() as *const i8,
                c"cksumvfs.c".as_ptr() as *mut i8 as *const i8, 309,
                c"nByte>=8".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(n_byte_1 & 7 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"cksmCompute".as_ptr() as *const i8,
                c"cksumvfs.c".as_ptr() as *mut i8 as *const i8, 310,
                c"(nByte&0x00000007)==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(n_byte_1 <= 65536) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"cksmCompute".as_ptr() as *const i8,
                c"cksumvfs.c".as_ptr() as *mut i8 as *const i8, 311,
                c"nByte<=65536".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if 1 == unsafe { *(&raw mut x as *mut u8) } as i32 {
        '__b0: loop {
            '__c0: loop {
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                break '__c0;
            }
            if !(a_data < a_end) { break '__b0; }
        }
    } else {
        '__b1: loop {
            '__c1: loop {
                s1 +=
                    ((unsafe { *a_data.offset(0 as isize) } & 255 as u32) << 24)
                                    +
                                    ((unsafe { *a_data.offset(0 as isize) } & 65280 as u32) <<
                                        8) +
                                ((unsafe { *a_data.offset(0 as isize) } & 16711680 as u32)
                                    >> 8) +
                            ((unsafe { *a_data.offset(0 as isize) } & 4278190080u32) >>
                                24) + s2;
                s2 +=
                    ((unsafe { *a_data.offset(1 as isize) } & 255 as u32) << 24)
                                    +
                                    ((unsafe { *a_data.offset(1 as isize) } & 65280 as u32) <<
                                        8) +
                                ((unsafe { *a_data.offset(1 as isize) } & 16711680 as u32)
                                    >> 8) +
                            ((unsafe { *a_data.offset(1 as isize) } & 4278190080u32) >>
                                24) + s1;
                {
                    let __n = 2;
                    let __p = &mut a_data;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                break '__c1;
            }
            if !(a_data < a_end) { break '__b1; }
        }
        s1 =
            ((s1 & 255 as u32) << 24) + ((s1 & 65280 as u32) << 8) +
                    ((s1 & 16711680 as u32) >> 8) +
                ((s1 & 4278190080u32) >> 24);
        s2 =
            ((s2 & 255 as u32) << 24) + ((s2 & 65280 as u32) << 8) +
                    ((s2 & 16711680 as u32) >> 8) +
                ((s2 & 4278190080u32) >> 24);
    }
    unsafe { memcpy(a_out_1 as *mut (), &raw mut s1 as *const (), 4 as u64) };
    unsafe {
        memcpy(unsafe { a_out_1.offset(4 as isize) } as *mut (),
            &raw mut s2 as *const (), 4 as u64)
    };
}

///* Read data from a cksm-file.
extern "C" fn cksm_read(mut p_file: *mut Sqlite3File, z_buf: *mut (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut CksmFile = p_file as *mut CksmFile;
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    rc =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_read.unwrap()
                })(p_file, z_buf, i_amt, i_ofst)
        };
    if rc == 0 {
        if i_ofst == 0 as i64 && i_amt >= 100 &&
                (unsafe {
                            memcmp(z_buf as *const (),
                                c"SQLite format 3".as_ptr() as *mut i8 as *const (),
                                16 as u64)
                        } == 0 ||
                    unsafe {
                            memcmp(z_buf as *const (),
                                c"ZV-".as_ptr() as *mut i8 as *const (), 3 as u64)
                        } == 0) {
            let d: *const u8 = z_buf as *mut u8 as *const u8;
            let has_correct_reserve_size: i8 =
                (unsafe { *d.offset(20 as isize) } as i32 == 8) as i8;
            cksm_set_flags(unsafe { &mut *p },
                has_correct_reserve_size as i32);
        }
        if i_amt >= 512 && i_amt & i_amt - 1 == 0 &&
                unsafe { (*p).verify_cksm } != 0 {
            let mut cksum: [u8; 8] = [0; 8];
            cksm_compute(z_buf as *mut u8, i_amt - 8,
                &raw mut cksum[0 as usize] as *mut u8);
            if unsafe {
                        memcmp(unsafe {
                                    unsafe {
                                        (z_buf as
                                                    *mut u8).offset(i_amt as isize).offset(-(8 as isize))
                                    }
                                } as *const (),
                            &raw mut cksum[0 as usize] as *mut u8 as *const (),
                            8 as u64)
                    } != 0 {
                unsafe {
                    sqlite3_log(10 | 32 << 8,
                        c"checksum fault offset %lld of \"%s\"".as_ptr() as *mut i8
                            as *const i8, i_ofst, unsafe { (*p).z_f_name })
                };
                rc = 10 | 32 << 8;
            }
        }
    }
    return rc;
}

///* Write data to a cksm-file.
extern "C" fn cksm_write(mut p_file: *mut Sqlite3File, z_buf: *const (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let p: *mut CksmFile = p_file as *mut CksmFile;
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if i_ofst == 0 as i64 && i_amt >= 100 &&
            (unsafe {
                        memcmp(z_buf,
                            c"SQLite format 3".as_ptr() as *mut i8 as *const (),
                            16 as u64)
                    } == 0 ||
                unsafe {
                        memcmp(z_buf, c"ZV-".as_ptr() as *mut i8 as *const (),
                            3 as u64)
                    } == 0) {
        let d: *const u8 = z_buf as *mut u8 as *const u8;
        let has_correct_reserve_size: i8 =
            (unsafe { *d.offset(20 as isize) } as i32 == 8) as i8;
        cksm_set_flags(unsafe { &mut *p }, has_correct_reserve_size as i32);
    }
    if i_amt >= 512 && unsafe { (*p).compute_cksm } != 0 {
        cksm_compute(z_buf as *mut u8, i_amt - 8,
            unsafe {
                unsafe {
                    (z_buf as
                                *mut u8).offset(i_amt as isize).offset(-(8 as isize))
                }
            });
    }
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_write.unwrap()
                })(p_file, z_buf, i_amt, i_ofst)
        };
}

///* Truncate a cksm-file.
extern "C" fn cksm_truncate(mut p_file: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_truncate.unwrap()
                })(p_file, size)
        };
}

///* Sync a cksm-file.
extern "C" fn cksm_sync(mut p_file: *mut Sqlite3File, flags: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_sync.unwrap()
                })(p_file, flags)
        };
}

///* Return the current file-size of a cksm-file.
extern "C" fn cksm_file_size(mut p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let p: *mut CksmFile = p_file as *mut CksmFile;
    p_file =
        unsafe { (p as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_file_size.unwrap()
                })(p_file, p_size)
        };
}

///* Lock a cksm-file.
extern "C" fn cksm_lock(mut p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_lock.unwrap()
                })(p_file, e_lock)
        };
}

///* Unlock a cksm-file.
extern "C" fn cksm_unlock(mut p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_unlock.unwrap()
                })(p_file, e_lock)
        };
}

///* Check if another file-handle holds a RESERVED lock on a cksm-file.
extern "C" fn cksm_check_reserved_lock(mut p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_file).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(p_file, p_res_out)
        };
}

///* File control method. For custom operations on a cksm-file.
#[allow(unused_doc_comments)]
extern "C" fn cksm_file_control(mut p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut CksmFile = p_file as *mut CksmFile;
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if op == 14 {
        let az_arg: *mut *mut i8 = p_arg as *mut *mut i8;
        if !(unsafe { *az_arg.offset(1 as isize) } != core::ptr::null_mut())
                        as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"cksmFileControl".as_ptr() as *const i8,
                    c"cksumvfs.c".as_ptr() as *mut i8 as *const i8, 552,
                    c"azArg[1]!=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if unsafe {
                    sqlite3_stricmp(unsafe { *az_arg.offset(1 as isize) } as
                            *const i8,
                        c"checksum_verification".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            let z_arg: *const i8 =
                unsafe { *az_arg.offset(2 as isize) } as *const i8;
            if z_arg != core::ptr::null_mut() {
                if unsafe { *z_arg.offset(0 as isize) } as i32 >= '1' as i32
                                    && unsafe { *z_arg.offset(0 as isize) } as i32 <= '9' as i32
                                ||
                                unsafe {
                                        sqlite3_strlike(c"enable%".as_ptr() as *mut i8 as *const i8,
                                            z_arg as *const i8, 0 as u32)
                                    } == 0 ||
                            unsafe {
                                    sqlite3_stricmp(c"yes".as_ptr() as *mut i8 as *const i8,
                                        z_arg as *const i8)
                                } == 0 ||
                        unsafe {
                                sqlite3_stricmp(c"on".as_ptr() as *mut i8 as *const i8,
                                    z_arg as *const i8)
                            } == 0 {
                    unsafe { (*p).verify_cksm = unsafe { (*p).compute_cksm } };
                } else { unsafe { (*p).verify_cksm = 0 as i8 }; }
                if !(unsafe { (*p).p_partner }).is_null() {
                    unsafe {
                        (*unsafe { (*p).p_partner }).verify_cksm =
                            unsafe { (*p).verify_cksm }
                    };
                }
            }
            unsafe {
                *az_arg.offset(0 as isize) =
                    unsafe {
                        sqlite3_mprintf(c"%d".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).verify_cksm } as i32)
                    }
            };
            return 0;
        } else if unsafe { (*p).compute_cksm } != 0 &&
                    unsafe { *az_arg.offset(2 as isize) } !=
                        core::ptr::null_mut() &&
                unsafe {
                        sqlite3_stricmp(unsafe { *az_arg.offset(1 as isize) } as
                                *const i8, c"page_size".as_ptr() as *mut i8 as *const i8)
                    } == 0 {

            /// Do not allow page size changes on a checksum database
            return 0;
        }
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
                    sqlite3_mprintf(c"cksm/%z".as_ptr() as *mut i8 as *const i8,
                        unsafe { *(p_arg as *mut *mut i8) })
                }
        };
    }
    return rc;
}

///* Return the sector-size in bytes for a cksm-file.
extern "C" fn cksm_sector_size(mut p_file: *mut Sqlite3File) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_sector_size.unwrap()
                })(p_file)
        };
}

///* Return the device characteristic flags supported by a cksm-file.
extern "C" fn cksm_device_characteristics(mut p_file: *mut Sqlite3File)
    -> i32 {
    let mut devchar: i32 = 0;
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    devchar =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_file).p_methods
                                    }).x_device_characteristics.unwrap()
                })(p_file)
        };
    return devchar & !32768;
}

/// Create a shared memory file mapping
extern "C" fn cksm_shm_map(mut p_file: *mut Sqlite3File, i_pg: i32, pgsz: i32,
    b_extend: i32, pp: *mut *mut ()) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_map.unwrap()
                })(p_file, i_pg, pgsz, b_extend, pp)
        };
}

/// Perform locking on a shared-memory segment
extern "C" fn cksm_shm_lock(mut p_file: *mut Sqlite3File, offset: i32, n: i32,
    flags: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_lock.unwrap()
                })(p_file, offset, n, flags)
        };
}

/// Memory barrier operation on shared memory
extern "C" fn cksm_shm_barrier(mut p_file: *mut Sqlite3File) -> () {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    unsafe {
        (unsafe {
                (*unsafe { (*p_file).p_methods }).x_shm_barrier.unwrap()
            })(p_file)
    };
}

/// Unmap a shared memory segment
extern "C" fn cksm_shm_unmap(mut p_file: *mut Sqlite3File, delete_flag: i32)
    -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_unmap.unwrap()
                })(p_file, delete_flag)
        };
}

/// Fetch a page of a memory-mapped file
extern "C" fn cksm_fetch(mut p_file: *mut Sqlite3File, i_ofst: Sqlite3Int64,
    i_amt: i32, pp: *mut *mut ()) -> i32 {
    let p: *const CksmFile = p_file as *mut CksmFile as *const CksmFile;
    if unsafe { (*p).compute_cksm } != 0 {
        unsafe { *pp = core::ptr::null_mut() };
        return 0;
    }
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if unsafe { (*unsafe { (*p_file).p_methods }).i_version } as i32 > 2 &&
            unsafe { (*unsafe { (*p_file).p_methods }).x_fetch.is_some() } {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_file).p_methods }).x_fetch.unwrap()
                    })(p_file, i_ofst, i_amt, pp)
            };
    }
    unsafe { *pp = core::ptr::null_mut() };
    return 0;
}

/// Release a memory-mapped page
extern "C" fn cksm_unfetch(mut p_file: *mut Sqlite3File, i_ofst: Sqlite3Int64,
    p_page: *mut ()) -> i32 {
    p_file =
        unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if unsafe { (*unsafe { (*p_file).p_methods }).i_version } as i32 > 2 &&
            unsafe { (*unsafe { (*p_file).p_methods }).x_unfetch.is_some() } {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_file).p_methods }).x_unfetch.unwrap()
                    })(p_file, i_ofst, p_page)
            };
    }
    return 0;
}

static cksm_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 3,
        x_close: Some(cksm_close),
        x_read: Some(cksm_read),
        x_write: Some(cksm_write),
        x_truncate: Some(cksm_truncate),
        x_sync: Some(cksm_sync),
        x_file_size: Some(cksm_file_size),
        x_lock: Some(cksm_lock),
        x_unlock: Some(cksm_unlock),
        x_check_reserved_lock: Some(cksm_check_reserved_lock),
        x_file_control: Some(cksm_file_control),
        x_sector_size: Some(cksm_sector_size),
        x_device_characteristics: Some(cksm_device_characteristics),
        x_shm_map: Some(cksm_shm_map),
        x_shm_lock: Some(cksm_shm_lock),
        x_shm_barrier: Some(cksm_shm_barrier),
        x_shm_unmap: Some(cksm_shm_unmap),
        x_fetch: Some(cksm_fetch),
        x_unfetch: Some(cksm_unfetch),
    };

///* Methods for CksmVfs
extern "C" fn cksm_open(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_file: *mut Sqlite3File, flags: i32, p_out_flags: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    '__b2: loop {
        '__c2: loop {
            let mut p: *mut CksmFile = core::ptr::null_mut();
            let mut p_sub_file: *mut Sqlite3File = core::ptr::null_mut();
            let mut p_sub_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
            p_sub_vfs = unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs;
            if flags & 256 == 0 {
                return unsafe {
                        (unsafe {
                                (*p_sub_vfs).x_open.unwrap()
                            })(p_sub_vfs, z_name, p_file, flags, p_out_flags)
                    };
            }
            p = p_file as *mut CksmFile;
            unsafe {
                memset(p as *mut (), 0,
                    core::mem::size_of::<CksmFile>() as u64)
            };
            p_sub_file =
                unsafe { (p_file as *mut CksmFile).offset(1 as isize) } as
                    *mut Sqlite3File;
            unsafe { (*p_file).p_methods = &cksm_io_methods };
            rc =
                unsafe {
                    (unsafe {
                            (*p_sub_vfs).x_open.unwrap()
                        })(p_sub_vfs, z_name, p_sub_file, flags, p_out_flags)
                };
            if rc != 0 { break '__b2; }
            unsafe { (*p).z_f_name = z_name };
            break '__c2;
        }
        if !(false) { break '__b2; }
    }
    if rc != 0 { unsafe { (*p_file).p_methods = core::ptr::null() }; }
    return rc;
}

///* All other VFS methods are pass-thrus.
extern "C" fn cksm_delete(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_delete.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path,
                dir_sync)
        };
}

extern "C" fn cksm_access(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    flags: i32, p_res_out: *mut i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_access.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path,
                flags, p_res_out)
        };
}

extern "C" fn cksm_full_pathname(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_full_pathname.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path,
                n_out, z_out)
        };
}

extern "C" fn cksm_dl_open(p_vfs: *mut Sqlite3Vfs, z_path: *const i8)
    -> *mut () {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_dl_open.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_path)
        };
}

extern "C" fn cksm_dl_error(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    unsafe {
        (unsafe {
                (*(unsafe { (*p_vfs).p_app_data } as
                                    *mut Sqlite3Vfs)).x_dl_error.unwrap()
            })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, n_byte,
            z_err_msg)
    };
}

extern "C" fn cksm_dl_sym(p_vfs: *mut Sqlite3Vfs, p: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_dl_sym.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, p,
                z_sym)
        };
}

extern "C" fn cksm_dl_close(p_vfs: *mut Sqlite3Vfs, p_handle: *mut ()) -> () {
    unsafe {
        (unsafe {
                (*(unsafe { (*p_vfs).p_app_data } as
                                    *mut Sqlite3Vfs)).x_dl_close.unwrap()
            })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, p_handle)
    };
}

extern "C" fn cksm_randomness(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_randomness.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, n_byte,
                z_buf_out)
        };
}

extern "C" fn cksm_sleep(p_vfs: *mut Sqlite3Vfs, n_micro: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_sleep.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, n_micro)
        };
}

extern "C" fn cksm_current_time(p_vfs: *mut Sqlite3Vfs, p_time_out: *mut f64)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_current_time.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs,
                p_time_out)
        };
}

extern "C" fn cksm_get_last_error(p_vfs: *mut Sqlite3Vfs, a: i32, b: *mut i8)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_get_last_error.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, a, b)
        };
}

extern "C" fn cksm_current_time_int64(p_vfs: *mut Sqlite3Vfs,
    p: *mut Sqlite3Int64) -> i32 {
    let p_orig: *mut Sqlite3Vfs =
        unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs;
    let mut rc: i32 = 0;
    if !(unsafe { (*p_orig).i_version } >= 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"cksmCurrentTimeInt64".as_ptr() as *const i8,
                c"cksumvfs.c".as_ptr() as *mut i8 as *const i8, 738,
                c"pOrig->iVersion>=2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_orig).x_current_time_int64.is_some() } {
        rc =
            unsafe {
                (unsafe {
                        (*p_orig).x_current_time_int64.unwrap()
                    })(p_orig, p)
            };
    } else {
        let mut r: f64 = 0.0;
        rc =
            unsafe {
                (unsafe { (*p_orig).x_current_time.unwrap() })(p_orig, &mut r)
            };
        unsafe { *p = (r * 86400000.0) as Sqlite3Int64 };
    }
    return rc;
}

extern "C" fn cksm_set_system_call(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_call: unsafe extern "C" fn() -> ()) -> i32 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_set_system_call.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_name,
                p_call)
        };
}

extern "C" fn cksm_get_system_call(p_vfs: *mut Sqlite3Vfs, z_name: *const i8)
    -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_get_system_call.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_name)
        };
}

extern "C" fn cksm_next_system_call(p_vfs: *mut Sqlite3Vfs, z_name: *const i8)
    -> *const i8 {
    return unsafe {
            (unsafe {
                    (*(unsafe { (*p_vfs).p_app_data } as
                                        *mut Sqlite3Vfs)).x_next_system_call.unwrap()
                })(unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs, z_name)
        };
}

static mut cksm_vfs: Sqlite3Vfs =
    Sqlite3Vfs {
        i_version: 3,
        sz_os_file: 0,
        mx_pathname: 1024,
        p_next: core::ptr::null_mut(),
        z_name: c"cksmvfs".as_ptr() as *const i8,
        p_app_data: core::ptr::null_mut(),
        x_open: Some(cksm_open),
        x_delete: Some(cksm_delete),
        x_access: Some(cksm_access),
        x_full_pathname: Some(cksm_full_pathname),
        x_dl_open: Some(cksm_dl_open),
        x_dl_error: Some(cksm_dl_error),
        x_dl_sym: Some(cksm_dl_sym),
        x_dl_close: Some(cksm_dl_close),
        x_randomness: Some(cksm_randomness),
        x_sleep: Some(cksm_sleep),
        x_current_time: Some(cksm_current_time),
        x_get_last_error: Some(cksm_get_last_error),
        x_current_time_int64: Some(cksm_current_time_int64),
        x_set_system_call: Some(cksm_set_system_call),
        x_get_system_call: Some(cksm_get_system_call),
        x_next_system_call: Some(cksm_next_system_call),
    };

///* SQL function:    verify_checksum(BLOB)
///*
///* Return 0 or 1 if the checksum is invalid or valid.  Or return
///* NULL if the input is not a BLOB that is the right size for a
///* database page.
extern "C" fn cksm_verify_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut n_byte: i32 = 0;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut cksum: [u8; 8] = [0; 8];
    data =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *mut u8;
    if data == core::ptr::null_mut() { return; }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } != 4
        {
        return;
    }
    n_byte =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    if n_byte < 512 || n_byte > 65536 || n_byte & n_byte - 1 != 0 { return; }
    cksm_compute(data, n_byte - 8, &raw mut cksum[0 as usize] as *mut u8);
    unsafe {
        sqlite3_result_int(context,
            (unsafe {
                        memcmp(unsafe {
                                    unsafe {
                                        data.offset(n_byte as isize).offset(-(8 as isize))
                                    }
                                } as *const (),
                            &raw mut cksum[0 as usize] as *mut u8 as *const (),
                            8 as u64)
                    } == 0) as i32)
    };
}

/// Register the verify_checksum() SQL function.
extern "C" fn cksm_register_func(db: *mut Sqlite3, pz_err_msg_1: *mut *mut i8,
    p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    if db == core::ptr::null_mut() { return 0; }
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"verify_checksum".as_ptr() as *mut i8 as *const i8, 1,
                1 | 2097152 | 2048, core::ptr::null_mut(),
                Some(cksm_verify_func), None, None)
        };
    return rc;
}

///* Register the cksum VFS as the default VFS for the system.
///* Also make arrangements to automatically register the "verify_checksum()"
///* SQL function on each new database connection.
extern "C" fn cksm_register_vfs() -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_orig: *mut Sqlite3Vfs =
            unsafe { sqlite3_vfs_find(core::ptr::null()) };
        if p_orig == core::ptr::null_mut() { return 1; }
        cksm_vfs.i_version = unsafe { (*p_orig).i_version };
        cksm_vfs.p_app_data = p_orig as *mut ();
        cksm_vfs.sz_os_file =
            (unsafe { (*p_orig).sz_os_file } as u64 +
                    core::mem::size_of::<CksmFile>() as u64) as i32;
        rc = unsafe { sqlite3_vfs_register(&mut cksm_vfs, 1) };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_auto_extension(Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn()
                                            -> ()>(cksm_register_func as *const ())
                            }))
                };
        }
        return rc;
    }
}

/// 
///* This routine is called by sqlite3_load_extension() when the
///* extension is first loaded.
///*
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_cksumvfs_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };

    /// not used
    (rc = cksm_register_func(db, core::ptr::null_mut(), core::ptr::null()));
    if rc == 0 { rc = cksm_register_vfs(); }
    if rc == 0 { rc = 0 | 1 << 8; }
    return rc;
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
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
