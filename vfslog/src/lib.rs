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

/// There is a pair (an array of size 2) of the following objects for
///* each database file being logged.  The first contains the filename
///* and is used to log I/O with the main database.  The second has
///* a NULL filename and is used to log I/O for the journal.  Both
///* out pointers are the same.
#[repr(C)]
#[derive(Copy, Clone)]
struct VLogLog {
    p_next: *mut VLogLog,
    pp_prev: *mut *mut VLogLog,
    n_ref: i32,
    n_filename: i32,
    z_filename: *mut i8,
    out: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VLogVfs {
    base: Sqlite3Vfs,
    p_vfs: *mut Sqlite3Vfs,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VLogFile {
    base: Sqlite3File,
    p_real: *mut Sqlite3File,
    p_log: *mut VLogLog,
}

extern "C" fn vlog_time() -> Sqlite3Uint64 { return 0 as Sqlite3Uint64; }

///* Write a message to the log file
extern "C" fn vlog_log_print(p_log_1: *const VLogLog, t_start_1: Sqlite3Int64,
    t_elapse_1: Sqlite3Int64, z_op_1: *const i8, i_arg1_1: Sqlite3Int64,
    i_arg2_1: Sqlite3Int64, z_arg3_1: *const i8, i_res_1: i32) -> () {
    let mut z1: [i8; 40] = [0; 40];
    let mut z2: [i8; 40] = [0; 40];
    let mut z3: [i8; 2000] = [0; 2000];
    if p_log_1 == core::ptr::null_mut() { return; }
    if i_arg1_1 >= 0 as i64 {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 40]>() as i32,
                &raw mut z1[0 as usize] as *mut i8,
                c"%lld".as_ptr() as *mut i8 as *const i8, i_arg1_1)
        };
    } else { z1[0 as usize] = 0 as i8; }
    if i_arg2_1 >= 0 as i64 {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 40]>() as i32,
                &raw mut z2[0 as usize] as *mut i8,
                c"%lld".as_ptr() as *mut i8 as *const i8, i_arg2_1)
        };
    } else { z2[0 as usize] = 0 as i8; }
    if !(z_arg3_1).is_null() {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 2000]>() as i32,
                &raw mut z3[0 as usize] as *mut i8,
                c"\"%.*w\"".as_ptr() as *mut i8 as *const i8,
                core::mem::size_of::<[i8; 2000]>() as u64 - 4 as u64,
                z_arg3_1)
        };
    } else { z3[0 as usize] = 0 as i8; }
    unsafe {
        fprintf(unsafe { (*p_log_1).out },
            c"%lld,%lld,%s,%d,%s,%s,%s,%d\n".as_ptr() as *mut i8 as *const i8,
            t_start_1, t_elapse_1, z_op_1,
            (unsafe { (*p_log_1).z_filename } == core::ptr::null_mut()) as
                i32, &raw mut z1[0 as usize] as *mut i8,
            &raw mut z2[0 as usize] as *mut i8,
            &raw mut z3[0 as usize] as *mut i8, i_res_1)
    };
}

///* Close a VLogLog object
extern "C" fn vlog_log_close(p: *mut VLogLog) -> () {
    if !(p).is_null() {
        let mut p_mutex: *mut Sqlite3Mutex = core::ptr::null_mut();
        {
            let __p = unsafe { &mut (*p).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p).n_ref } > 0 ||
                unsafe { (*p).z_filename } == core::ptr::null_mut() {
            return;
        }
        p_mutex = unsafe { sqlite3_mutex_alloc(2) };
        unsafe { sqlite3_mutex_enter(p_mutex) };
        unsafe { *unsafe { (*p).pp_prev } = unsafe { (*p).p_next } };
        if !(unsafe { (*p).p_next }).is_null() {
            unsafe {
                (*unsafe { (*p).p_next }).pp_prev = unsafe { (*p).pp_prev }
            };
        }
        unsafe { sqlite3_mutex_leave(p_mutex) };
        unsafe { fclose(unsafe { (*p).out }) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

///* Methods for VLogFile
extern "C" fn vlog_close(p_file: *mut Sqlite3File) -> i32 {
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut rc: i32 = 0;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
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
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"CLOSE".as_ptr() as *mut i8 as *const i8, -1 as Sqlite3Int64,
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    vlog_log_close(unsafe { (*p).p_log });
    return rc;
}

///* Compute signature for a block of content.
///*
///* For blocks of 16 or fewer bytes, the signature is just a hex dump of
///* the entire block.
///*
///* For blocks of more than 16 bytes, the signature is a hex dump of the
///* first 8 bytes followed by a 64-bit has of the entire block.
extern "C" fn vlog_signature(p: *mut u8, n: i32, z_cksum_1: *mut i8) -> () {
    let mut s0: u32 = 0 as u32;
    let mut s1: u32 = 0 as u32;
    let mut p_i: *const u32 = core::ptr::null();
    let mut i: i32 = 0;
    if n <= 16 {
        {
            i = 0;
            '__b0: loop {
                if !(i < n) { break '__b0; }
                '__c0: loop {
                    unsafe {
                        sqlite3_snprintf(3,
                            unsafe { z_cksum_1.offset((i * 2) as isize) },
                            c"%02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *p.offset(i as isize) } as i32)
                    };
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    } else {
        p_i = p as *mut u32;
        {
            i = 0;
            '__b1: loop {
                if !(i < n - 7) { break '__b1; }
                '__c1: loop {
                    s0 += unsafe { *p_i.offset(0 as isize) } + s1;
                    s1 += unsafe { *p_i.offset(1 as isize) } + s0;
                    {
                        let __n = 2;
                        let __p = &mut p_i;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    break '__c1;
                }
                i += 8;
            }
        }
        {
            i = 0;
            '__b2: loop {
                if !(i < 8) { break '__b2; }
                '__c2: loop {
                    unsafe {
                        sqlite3_snprintf(3,
                            unsafe { z_cksum_1.offset((i * 2) as isize) },
                            c"%02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *p.offset(i as isize) } as i32)
                    };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_snprintf(18,
                unsafe { z_cksum_1.offset((i * 2) as isize) },
                c"-%08x%08x".as_ptr() as *mut i8 as *const i8, s0, s1)
        };
    }
}

///* Convert a big-endian 32-bit integer into a native integer
extern "C" fn big_to_native(x: *const u8) -> i32 {
    return ((unsafe { *x.offset(0 as isize) } as i32) << 24) +
                    ((unsafe { *x.offset(1 as isize) } as i32) << 16) +
                ((unsafe { *x.offset(2 as isize) } as i32) << 8) +
            unsafe { *x.offset(3 as isize) } as i32;
}

///* Read data from an vlog-file.
extern "C" fn vlog_read(p_file: *mut Sqlite3File, z_buf: *mut (), i_amt: i32,
    i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    let mut z_sig: [i8; 40] = [0; 40];
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_read.unwrap()
                })(unsafe { (*p).p_real }, z_buf, i_amt, i_ofst)
        };
    t_elapse = vlog_time() - t_start;
    if rc == 0 {
        vlog_signature(z_buf as *mut u8, i_amt,
            &raw mut z_sig[0 as usize] as *mut i8);
    } else { z_sig[0 as usize] = 0 as i8; }
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"READ".as_ptr() as *mut i8 as *const i8, i_amt as Sqlite3Int64,
        i_ofst, &raw mut z_sig[0 as usize] as *mut i8 as *const i8, rc);
    if rc == 0 && !(unsafe { (*p).p_log }).is_null() &&
                    !(unsafe { (*unsafe { (*p).p_log }).z_filename }).is_null()
                && i_ofst <= 24 as i64 &&
            i_ofst + i_amt as SqliteInt64 >= 28 as i64 {
        let x: *const u8 =
            unsafe {
                    (z_buf as
                            *mut u8).offset((24 as SqliteInt64 - i_ofst) as isize)
                } as *const u8;
        let mut i_ctr: u32 = 0 as u32;
        let mut n_free: u32 = -1i32 as u32;
        let mut z_free: *const i8 = core::ptr::null();
        let mut z_str: [i8; 12] = [0; 12];
        i_ctr = big_to_native(x as *const u8) as u32;
        if i_ofst + i_amt as SqliteInt64 >= 40 as i64 {
            z_free = &raw mut z_str[0 as usize] as *mut i8;
            unsafe {
                sqlite3_snprintf(core::mem::size_of::<[i8; 12]>() as i32,
                    &raw mut z_str[0 as usize] as *mut i8,
                    c"%d".as_ptr() as *mut i8 as *const i8,
                    big_to_native(unsafe { x.offset(8 as isize) } as *const u8))
            };
            n_free =
                big_to_native(unsafe { x.offset(12 as isize) } as *const u8)
                    as u32;
        }
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, 0 as Sqlite3Int64,
            c"CHNGCTR-READ".as_ptr() as *mut i8 as *const i8,
            i_ctr as Sqlite3Int64, n_free as Sqlite3Int64,
            z_free as *const i8, 0);
    }
    return rc;
}

///* Write data to an vlog-file.
extern "C" fn vlog_write(p_file: *mut Sqlite3File, z: *const (), i_amt: i32,
    i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    let mut z_sig: [i8; 40] = [0; 40];
    t_start = vlog_time();
    vlog_signature(z as *mut u8, i_amt,
        &raw mut z_sig[0 as usize] as *mut i8);
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_write.unwrap()
                })(unsafe { (*p).p_real }, z, i_amt, i_ofst)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"WRITE".as_ptr() as *mut i8 as *const i8, i_amt as Sqlite3Int64,
        i_ofst, &raw mut z_sig[0 as usize] as *mut i8 as *const i8, rc);
    if rc == 0 && !(unsafe { (*p).p_log }).is_null() &&
                    !(unsafe { (*unsafe { (*p).p_log }).z_filename }).is_null()
                && i_ofst <= 24 as i64 &&
            i_ofst + i_amt as SqliteInt64 >= 28 as i64 {
        let x: *const u8 =
            unsafe {
                    (z as *mut u8).offset((24 as SqliteInt64 - i_ofst) as isize)
                } as *const u8;
        let mut i_ctr: u32 = 0 as u32;
        let mut n_free: u32 = -1i32 as u32;
        let mut z_free: *const i8 = core::ptr::null();
        let mut z_str: [i8; 12] = [0; 12];
        i_ctr = big_to_native(x as *const u8) as u32;
        if i_ofst + i_amt as SqliteInt64 >= 40 as i64 {
            z_free = &raw mut z_str[0 as usize] as *mut i8;
            unsafe {
                sqlite3_snprintf(core::mem::size_of::<[i8; 12]>() as i32,
                    &raw mut z_str[0 as usize] as *mut i8,
                    c"%d".as_ptr() as *mut i8 as *const i8,
                    big_to_native(unsafe { x.offset(8 as isize) } as *const u8))
            };
            n_free =
                big_to_native(unsafe { x.offset(12 as isize) } as *const u8)
                    as u32;
        }
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, 0 as Sqlite3Int64,
            c"CHNGCTR-WRITE".as_ptr() as *mut i8 as *const i8,
            i_ctr as Sqlite3Int64, n_free as Sqlite3Int64,
            z_free as *const i8, 0);
    }
    return rc;
}

///* Truncate an vlog-file.
extern "C" fn vlog_truncate(p_file: *mut Sqlite3File, size: Sqlite3Int64)
    -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_truncate.unwrap()
                })(unsafe { (*p).p_real }, size)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"TRUNCATE".as_ptr() as *mut i8 as *const i8, size,
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    return rc;
}

///* Sync an vlog-file.
extern "C" fn vlog_sync(p_file: *mut Sqlite3File, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_sync.unwrap()
                })(unsafe { (*p).p_real }, flags)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"SYNC".as_ptr() as *mut i8 as *const i8, flags as Sqlite3Int64,
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    return rc;
}

///* Return the current file-size of an vlog-file.
extern "C" fn vlog_file_size(p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_file_size.unwrap()
                })(unsafe { (*p).p_real }, p_size)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"FILESIZE".as_ptr() as *mut i8 as *const i8, unsafe { *p_size },
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    return rc;
}

///* Lock an vlog-file.
extern "C" fn vlog_lock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_lock.unwrap()
                })(unsafe { (*p).p_real }, e_lock)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"LOCK".as_ptr() as *mut i8 as *const i8, e_lock as Sqlite3Int64,
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    return rc;
}

///* Unlock an vlog-file.
extern "C" fn vlog_unlock(p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, 0 as Sqlite3Int64,
        c"UNLOCK".as_ptr() as *mut i8 as *const i8, e_lock as Sqlite3Int64,
        -1 as Sqlite3Int64, core::ptr::null(), 0);
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_unlock.unwrap()
                })(unsafe { (*p).p_real }, e_lock)
        };
    return rc;
}

///* Check if another file-handle holds a RESERVED lock on an vlog-file.
extern "C" fn vlog_check_reserved_lock(p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(unsafe { (*p).p_real }, p_res_out)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"CHECKRESERVEDLOCK".as_ptr() as *mut i8 as *const i8,
        unsafe { *p_res_out } as Sqlite3Int64, -1 as Sqlite3Int64,
        c"".as_ptr() as *mut i8 as *const i8, rc);
    return rc;
}

///* File control method. For custom operations on an vlog-file.
extern "C" fn vlog_file_control(p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut rc: i32 = 0;
    t_start = vlog_time();
    rc =
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
                    sqlite3_mprintf(c"vlog/%z".as_ptr() as *mut i8 as *const i8,
                        unsafe { *(p_arg as *mut *mut i8) })
                }
        };
    }
    t_elapse = vlog_time() - t_start;
    if op == 19 {
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
            c"TRACE".as_ptr() as *mut i8 as *const i8, op as Sqlite3Int64,
            -1 as Sqlite3Int64, p_arg as *const i8, rc);
    } else if op == 14 {
        let az_arg: *const *const i8 =
            p_arg as *mut *const i8 as *const *const i8;
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
            c"FILECONTROL".as_ptr() as *mut i8 as *const i8,
            op as Sqlite3Int64, -1 as Sqlite3Int64,
            unsafe { *az_arg.offset(1 as isize) }, rc);
    } else if op == 5 {
        let sz: Sqlite3Int64 =
            unsafe { core::ptr::read(p_arg as *mut Sqlite3Int64) };
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
            c"FILECONTROL".as_ptr() as *mut i8 as *const i8,
            op as Sqlite3Int64, sz, core::ptr::null(), rc);
    } else {
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
            c"FILECONTROL".as_ptr() as *mut i8 as *const i8,
            op as Sqlite3Int64, -1 as Sqlite3Int64, core::ptr::null(), rc);
    }
    return rc;
}

///* Return the sector-size in bytes for an vlog-file.
extern "C" fn vlog_sector_size(p_file: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_sector_size.unwrap()
                })(unsafe { (*p).p_real })
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"SECTORSIZE".as_ptr() as *mut i8 as *const i8, -1 as Sqlite3Int64,
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    return rc;
}

///* Return the device characteristic flags supported by an vlog-file.
extern "C" fn vlog_device_characteristics(p_file: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_device_characteristics.unwrap()
                })(unsafe { (*p).p_real })
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
        c"DEVCHAR".as_ptr() as *mut i8 as *const i8, -1 as Sqlite3Int64,
        -1 as Sqlite3Int64, core::ptr::null(), rc);
    return rc;
}

///* List of all active log connections.  Protected by the master mutex.
static mut all_logs: *mut VLogLog = core::ptr::null_mut();

///* Open a VLogLog object on the given file
extern "C" fn vlog_log_open(z_filename_1: *const i8) -> *mut VLogLog {
    unsafe {
        let mut n_name: i32 = unsafe { strlen(z_filename_1) } as i32;
        let mut is_journal: i32 = 0;
        let mut p_mutex: *mut Sqlite3Mutex = core::ptr::null_mut();
        let mut p_log: *mut VLogLog = core::ptr::null_mut();
        let mut p_temp: *mut VLogLog = core::ptr::null_mut();
        let mut t_now: Sqlite3Int64 = 0 as Sqlite3Int64;
        if n_name > 4 &&
                unsafe {
                        strcmp(unsafe {
                                unsafe {
                                    z_filename_1.offset(n_name as isize).offset(-(4 as isize))
                                }
                            }, c"-wal".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
            return core::ptr::null_mut();
        } else if n_name > 8 &&
                unsafe {
                        strcmp(unsafe {
                                unsafe {
                                    z_filename_1.offset(n_name as isize).offset(-(8 as isize))
                                }
                            }, c"-journal".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
            n_name -= 8;
            is_journal = 1;
        } else if n_name > 12 &&
                unsafe {
                        sqlite3_strglob(c"-mj??????9??".as_ptr() as *mut i8 as
                                *const i8,
                            unsafe {
                                unsafe {
                                    z_filename_1.offset(n_name as isize).offset(-(12 as isize))
                                }
                            })
                    } == 0 {
            return core::ptr::null_mut();
        }
        p_temp =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<VLogLog>() as u64 *
                                    2 as u64 + n_name as u64 + 60 as u64)
                } as *mut VLogLog;
        if p_temp == core::ptr::null_mut() { return core::ptr::null_mut(); }
        p_mutex = unsafe { sqlite3_mutex_alloc(2) };
        unsafe { sqlite3_mutex_enter(p_mutex) };
        {
            p_log = all_logs;
            '__b3: loop {
                if !(!(p_log).is_null()) { break '__b3; }
                '__c3: loop {
                    if unsafe { (*p_log).n_filename } == n_name &&
                            (unsafe {
                                            memcmp(unsafe { (*p_log).z_filename } as *const (),
                                                z_filename_1 as *const (), n_name as u64)
                                        } == 0) as i32 != 0 {
                        break '__b3;
                    }
                    break '__c3;
                }
                p_log = unsafe { (*p_log).p_next };
            }
        }
        if p_log == core::ptr::null_mut() {
            p_log = p_temp;
            p_temp = core::ptr::null_mut();
            unsafe {
                memset(p_log as *mut (), 0,
                    core::mem::size_of::<VLogLog>() as u64 * 2 as u64)
            };
            unsafe {
                (*p_log).z_filename =
                    unsafe { &raw mut *p_log.offset(2 as isize) } as *mut i8
            };
            t_now = vlog_time() as Sqlite3Int64;
            unsafe {
                sqlite3_snprintf(n_name + 60, unsafe { (*p_log).z_filename },
                    c"%.*s-debuglog-%lld".as_ptr() as *mut i8 as *const i8,
                    n_name, z_filename_1, t_now)
            };
            unsafe {
                (*p_log).out =
                    unsafe {
                        fopen(unsafe { (*p_log).z_filename } as *const i8,
                            c"a".as_ptr() as *mut i8 as *const i8)
                    }
            };
            if unsafe { (*p_log).out } == core::ptr::null_mut() {
                unsafe { sqlite3_mutex_leave(p_mutex) };
                unsafe { sqlite3_free(p_log as *mut ()) };
                return core::ptr::null_mut();
            }
            unsafe { (*p_log).n_filename = n_name };
            unsafe {
                (*p_log.offset(1 as isize)).out =
                    unsafe { (*p_log.offset(0 as isize)).out }
            };
            unsafe { (*p_log).pp_prev = &mut all_logs };
            if !(all_logs).is_null() {
                unsafe {
                    (*all_logs).pp_prev = unsafe { &mut (*p_log).p_next }
                };
            }
            unsafe { (*p_log).p_next = all_logs };
            all_logs = p_log;
        }
        unsafe { sqlite3_mutex_leave(p_mutex) };
        if !(p_temp).is_null() {
            unsafe { sqlite3_free(p_temp as *mut ()) };
        } else {}
        if !(p_log).is_null() && is_journal != 0 {
            {
                let __p = &mut p_log;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        {
            let __p = unsafe { &mut (*p_log).n_ref };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return p_log;
    }
}

static mut vlog_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(vlog_close),
        x_read: Some(vlog_read),
        x_write: Some(vlog_write),
        x_truncate: Some(vlog_truncate),
        x_sync: Some(vlog_sync),
        x_file_size: Some(vlog_file_size),
        x_lock: Some(vlog_lock),
        x_unlock: Some(vlog_unlock),
        x_check_reserved_lock: Some(vlog_check_reserved_lock),
        x_file_control: Some(vlog_file_control),
        x_sector_size: Some(vlog_sector_size),
        x_device_characteristics: Some(vlog_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };

///* Methods for VLogVfs
extern "C" fn vlog_open(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_file: *mut Sqlite3File, flags: i32, p_out_flags: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
        let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
        let mut i_arg2: Sqlite3Int64 = 0 as Sqlite3Int64;
        let p: *mut VLogFile = p_file as *mut VLogFile;
        unsafe {
            (*p).p_real =
                unsafe { &raw mut *p.offset(1 as isize) } as *mut Sqlite3File
        };
        if flags & (256 | 2048) != 0 {
            unsafe { (*p).p_log = vlog_log_open(z_name) };
        } else { unsafe { (*p).p_log = core::ptr::null_mut() }; }
        t_start = vlog_time();
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*(p_vfs as *mut VLogVfs)).p_vfs
                                        }).x_open.unwrap()
                    })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_name,
                    unsafe { (*p).p_real }, flags, p_out_flags)
            };
        t_elapse = vlog_time() - t_start;
        i_arg2 =
            if !(p_out_flags).is_null() {
                    unsafe { *p_out_flags }
                } else { -1 } as Sqlite3Int64;
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as Sqlite3Int64, t_elapse as Sqlite3Int64,
            c"OPEN".as_ptr() as *mut i8 as *const i8, flags as Sqlite3Int64,
            i_arg2, core::ptr::null(), rc);
        if rc == 0 {
            unsafe {
                (*p_file).p_methods =
                    &raw mut vlog_io_methods as *const Sqlite3IoMethods
            };
        } else {
            if !(unsafe { (*p).p_log }).is_null() {
                vlog_log_close(unsafe { (*p).p_log });
            }
            unsafe { (*p).p_log = core::ptr::null_mut() };
        }
        return rc;
    }
}

///* Delete the file located at zPath. If the dirSync argument is true,
///* ensure the file-system modifications are synced to disk before
///* returning.
extern "C" fn vlog_delete(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut p_log: *mut VLogLog = core::ptr::null_mut();
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_delete.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path,
                dir_sync)
        };
    t_elapse = vlog_time() - t_start;
    p_log = vlog_log_open(z_path);
    vlog_log_print(p_log as *const VLogLog, t_start as Sqlite3Int64,
        t_elapse as Sqlite3Int64, c"DELETE".as_ptr() as *mut i8 as *const i8,
        dir_sync as Sqlite3Int64, -1 as Sqlite3Int64, core::ptr::null(), rc);
    vlog_log_close(p_log);
    return rc;
}

///* Test for access permissions. Return true if the requested permission
///* is available, or false otherwise.
extern "C" fn vlog_access(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    flags: i32, p_res_out: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut t_elapse: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    let mut p_log: *mut VLogLog = core::ptr::null_mut();
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_access.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path, flags,
                p_res_out)
        };
    t_elapse = vlog_time() - t_start;
    p_log = vlog_log_open(z_path);
    vlog_log_print(p_log as *const VLogLog, t_start as Sqlite3Int64,
        t_elapse as Sqlite3Int64, c"ACCESS".as_ptr() as *mut i8 as *const i8,
        flags as Sqlite3Int64, unsafe { *p_res_out } as Sqlite3Int64,
        core::ptr::null(), rc);
    vlog_log_close(p_log);
    return rc;
}

///* Populate buffer zOut with the full canonical pathname corresponding
///* to the pathname in zPath. zOut is guaranteed to point to a buffer
///* of at least (INST_MAX_PATHNAME+1) bytes.
extern "C" fn vlog_full_pathname(p_vfs: *mut Sqlite3Vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_full_pathname.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path, n_out,
                z_out)
        };
}

///* Open the dynamic library located at zPath and return a handle.
extern "C" fn vlog_dl_open(p_vfs: *mut Sqlite3Vfs, z_path: *const i8)
    -> *mut () {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_dl_open.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path)
        };
}

///* Populate the buffer zErrMsg (size nByte bytes) with a human readable
///* utf-8 string describing the most recent error encountered associated 
///* with dynamic libraries.
extern "C" fn vlog_dl_error(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*(p_vfs as *mut VLogVfs)).p_vfs
                                }).x_dl_error.unwrap()
            })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, n_byte, z_err_msg)
    };
}

///* Return a pointer to the symbol zSymbol in the dynamic library pHandle.
extern "C" fn vlog_dl_sym(p_vfs: *mut Sqlite3Vfs, p: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_dl_sym.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p, z_sym)
        };
}

///* Close the dynamic library handle pHandle.
extern "C" fn vlog_dl_close(p_vfs: *mut Sqlite3Vfs, p_handle: *mut ()) -> () {
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*(p_vfs as *mut VLogVfs)).p_vfs
                                }).x_dl_close.unwrap()
            })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p_handle)
    };
}

///* Populate the buffer pointed to by zBufOut with nByte bytes of 
///* random data.
extern "C" fn vlog_randomness(p_vfs: *mut Sqlite3Vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_randomness.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, n_byte,
                z_buf_out)
        };
}

///* Sleep for nMicro microseconds. Return the number of microseconds 
///* actually slept.
extern "C" fn vlog_sleep(p_vfs: *mut Sqlite3Vfs, n_micro: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_sleep.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, n_micro)
        };
}

///* Return the current time as a Julian Day number in *pTimeOut.
extern "C" fn vlog_current_time(p_vfs: *mut Sqlite3Vfs, p_time_out: *mut f64)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_current_time.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p_time_out)
        };
}

extern "C" fn vlog_get_last_error(p_vfs: *mut Sqlite3Vfs, a: i32, b: *mut i8)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_get_last_error.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, a, b)
        };
}

extern "C" fn vlog_current_time_int64(p_vfs: *mut Sqlite3Vfs,
    p: *mut Sqlite3Int64) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_current_time_int64.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p)
        };
}

static mut vlog_vfs: VLogVfs =
    VLogVfs {
        base: Sqlite3Vfs {
            i_version: 1,
            sz_os_file: 0,
            mx_pathname: 1024,
            p_next: core::ptr::null_mut(),
            z_name: c"vfslog".as_ptr() as *const i8,
            p_app_data: core::ptr::null_mut(),
            x_open: Some(vlog_open),
            x_delete: Some(vlog_delete),
            x_access: Some(vlog_access),
            x_full_pathname: Some(vlog_full_pathname),
            x_dl_open: Some(vlog_dl_open),
            x_dl_error: Some(vlog_dl_error),
            x_dl_sym: Some(vlog_dl_sym),
            x_dl_close: Some(vlog_dl_close),
            x_randomness: Some(vlog_randomness),
            x_sleep: Some(vlog_sleep),
            x_current_time: Some(vlog_current_time),
            x_get_last_error: Some(vlog_get_last_error),
            x_current_time_int64: Some(vlog_current_time_int64),
            x_set_system_call: None,
            x_get_system_call: None,
            x_next_system_call: None,
        },
        p_vfs: core::ptr::null_mut(),
    };

///* Register debugvfs as the default VFS for this process.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_vfslog(z_arg_1: *const i8) -> i32 {
    unsafe {
        vlog_vfs.p_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        if vlog_vfs.p_vfs == core::ptr::null_mut() { return 1; }
        vlog_vfs.base.sz_os_file =
            (core::mem::size_of::<VLogFile>() as u64 +
                    unsafe { (*vlog_vfs.p_vfs).sz_os_file } as u64) as i32;
        return unsafe { sqlite3_vfs_register(&mut vlog_vfs.base, 1) };
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn fclose(_: *mut FILE)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
