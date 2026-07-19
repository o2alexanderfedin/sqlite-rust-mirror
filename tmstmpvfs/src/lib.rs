#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3IoMethods,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, SqliteInt64,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

type Int32T = i32;

type DarwinPidT = Int32T;

type PidT = DarwinPidT;

///* Forward declaration of objects used by this utility
type TmstmpVfs = Sqlite3Vfs;

/// An open WAL or DB file
#[repr(C)]
#[derive(Copy, Clone)]
struct TmstmpFile {
    base: Sqlite3File,
    u_magic: u32,
    salt1: u32,
    i_frame: u32,
    pgno: u32,
    pgsz: u32,
    is_wal: u8,
    is_db: u8,
    is_commit: u8,
    has_correct_reserve: u8,
    in_ckpt: u8,
    p_log: *mut TmstmpLog,
    p_partner: *mut TmstmpFile,
    i_ofst: Sqlite3Int64,
    p_sub_vfs: *mut Sqlite3Vfs,
}

/// Information for the tmstmp log file.
#[repr(C)]
#[derive(Copy, Clone)]
struct TmstmpLog {
    z_logname: *mut i8,
    log: *mut FILE,
    n: i32,
    a: [u8; 96],
}

/// Free a TmstmpLog object
extern "C" fn tmstmp_log_free(p_log_1: *mut TmstmpLog) -> () {
    if p_log_1 == core::ptr::null_mut() { return; }
    if !(unsafe { (*p_log_1).log }).is_null() {
        unsafe { fclose(unsafe { (*p_log_1).log }) };
    }
    unsafe { sqlite3_free(unsafe { (*p_log_1).z_logname } as *mut ()) };
    unsafe { sqlite3_free(p_log_1 as *mut ()) };
}

/// Flush log content.  Open the file if necessary. Return the
///* number of errors.
extern "C" fn tmstmp_log_flush(p: &mut TmstmpFile) -> i32 {
    let p_log: *mut TmstmpLog = (*p).p_log;
    if !(p_log != core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"tmstmpLogFlush".as_ptr() as *const i8,
                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 496,
                c"pLog!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_log).log } == core::ptr::null_mut() {
        unsafe {
            (*p_log).log =
                unsafe {
                    fopen(unsafe { (*p_log).z_logname } as *const i8,
                        c"wb".as_ptr() as *mut i8 as *const i8)
                }
        };
        if unsafe { (*p_log).log } == core::ptr::null_mut() {
            tmstmp_log_free(p_log);
            (*p).p_log = core::ptr::null_mut();
            return 1;
        }
    }
    {
        let _ =
            unsafe {
                fwrite(unsafe { &raw mut (*p_log).a[0 as usize] } as *mut u8
                        as *const (), unsafe { (*p_log).n } as u64, 1 as u64,
                    unsafe { (*p_log).log })
            };
    };
    unsafe { fflush(unsafe { (*p_log).log }) };
    unsafe { (*p_log).n = 0 };
    return 0;
}

///* Write a 6-byte millisecond timestamp into aOut[]
extern "C" fn tmstmp_put_ts(p: &TmstmpFile, a_out_1: *mut u8) -> () {
    let mut tm: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    unsafe {
        (unsafe {
                (*(*p).p_sub_vfs).x_current_time_int64.unwrap()
            })((*p).p_sub_vfs, &raw mut tm as *mut Sqlite3Int64)
    };
    tm -= 210866760000000i64 as Sqlite3Uint64;
    unsafe {
        *a_out_1.offset(0 as isize) = (tm >> 40 & 255 as Sqlite3Uint64) as u8
    };
    unsafe {
        *a_out_1.offset(1 as isize) = (tm >> 32 & 255 as Sqlite3Uint64) as u8
    };
    unsafe {
        *a_out_1.offset(2 as isize) = (tm >> 24 & 255 as Sqlite3Uint64) as u8
    };
    unsafe {
        *a_out_1.offset(3 as isize) = (tm >> 16 & 255 as Sqlite3Uint64) as u8
    };
    unsafe {
        *a_out_1.offset(4 as isize) = (tm >> 8 & 255 as Sqlite3Uint64) as u8
    };
    unsafe {
        *a_out_1.offset(5 as isize) = (tm & 255 as Sqlite3Uint64) as u8
    };
}

/// Write a 32-bit integer as big-ending into a[]
extern "C" fn tmstmp_put_u32(v: u32, a: *mut u8) -> () {
    unsafe { *a.offset(0 as isize) = (v >> 24 & 255 as u32) as u8 };
    unsafe { *a.offset(1 as isize) = (v >> 16 & 255 as u32) as u8 };
    unsafe { *a.offset(2 as isize) = (v >> 8 & 255 as u32) as u8 };
    unsafe { *a.offset(3 as isize) = (v & 255 as u32) as u8 };
}

///* Write a record onto the event log
extern "C" fn tmstmp_event(mut p: *mut TmstmpFile, op: u8, a1: u8, a2: u32,
    a3: u32, p_ts_1: *const u8) -> () {
    let mut a: *mut u8 = core::ptr::null_mut();
    let mut p_log: *mut TmstmpLog = core::ptr::null_mut();
    if unsafe { (*p).is_wal } != 0 {
        p = unsafe { (*p).p_partner };
        if !(p != core::ptr::null_mut()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"tmstmpEvent".as_ptr() as *const i8,
                    c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 526,
                    c"p!=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if (unsafe { (*p).is_db } == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"tmstmpEvent".as_ptr() as *const i8,
                    c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 527,
                    c"p->isDb".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    p_log = unsafe { (*p).p_log };
    if p_log == core::ptr::null_mut() { return; }
    if unsafe { (*p_log).n } >= core::mem::size_of::<[u8; 96]>() as i32 {
        if tmstmp_log_flush(unsafe { &mut *p }) != 0 { return; }
    }
    a =
        unsafe {
            (unsafe { &raw mut (*p_log).a[0 as usize] } as
                    *mut u8).offset(unsafe { (*p_log).n } as isize)
        };
    unsafe { *a.offset(0 as isize) = op };
    unsafe { *a.offset(1 as isize) = a1 };
    if !(p_ts_1).is_null() {
        unsafe {
            memcpy(unsafe { a.offset(2 as isize) } as *mut (),
                p_ts_1 as *const (), 6 as u64)
        };
    } else { tmstmp_put_ts(unsafe { &*p }, unsafe { a.offset(2 as isize) }); }
    tmstmp_put_u32(a2, unsafe { a.offset(8 as isize) });
    tmstmp_put_u32(a3, unsafe { a.offset(12 as isize) });
    unsafe { (*p_log).n += 16 };
    if !(unsafe { (*p_log).log }).is_null() ||
            op as i32 >= 3 && op as i32 <= 8 {
        { let _ = tmstmp_log_flush(unsafe { &mut *p }); };
    }
}

///* Methods for TmstmpFile
extern "C" fn tmstmp_close(mut p_file: *mut Sqlite3File) -> i32 {
    let p: *mut TmstmpFile = p_file as *mut TmstmpFile;
    if unsafe { (*p).has_correct_reserve } != 0 {
        tmstmp_event(p,
            if unsafe { (*p).is_db } != 0 { 15 } else { 14 } as u8, 0 as u8,
            0 as u32, 0 as u32, core::ptr::null());
    }
    tmstmp_log_free(unsafe { (*p).p_log });
    if !(unsafe { (*p).p_partner }).is_null() {
        if !(unsafe { (*unsafe { (*p).p_partner }).p_partner } == p) as i32 as
                    i64 != 0 {
            unsafe {
                __assert_rtn(c"tmstmpClose".as_ptr() as *const i8,
                    c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 560,
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
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_close.unwrap()
                })(p_file)
        };
}

///* Read bytes from a file
extern "C" fn tmstmp_read(mut p_file: *mut Sqlite3File, z_buf: *mut (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut TmstmpFile = p_file as *mut TmstmpFile;
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    rc =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_read.unwrap()
                })(p_file, z_buf, i_amt, i_ofst)
        };
    if rc != 0 { return rc; }
    if unsafe { (*p).is_db } != 0 && i_ofst == 0 as i64 && i_amt >= 100 {
        let a: *const u8 = z_buf as *mut u8 as *const u8;
        unsafe {
            (*p).has_correct_reserve =
                (unsafe { *a.offset(20 as isize) } as i32 == 16) as u8
        };
        unsafe {
            (*p).pgsz =
                (((unsafe { *a.offset(16 as isize) } as i32) << 8) +
                        unsafe { *a.offset(17 as isize) } as i32) as u32
        };
        if unsafe { (*p).pgsz } == 1 as u32 {
            unsafe { (*p).pgsz = 65536 as u32 };
        }
        if !(unsafe { (*p).p_partner }).is_null() {
            unsafe {
                (*unsafe { (*p).p_partner }).has_correct_reserve =
                    unsafe { (*p).has_correct_reserve }
            };
            unsafe {
                (*unsafe { (*p).p_partner }).pgsz = unsafe { (*p).pgsz }
            };
        }
    }
    if unsafe { (*p).is_wal } != 0 && unsafe { (*p).in_ckpt } != 0 &&
                    i_amt >= 512 && i_amt <= 65535 && i_amt & i_amt - 1 == 0 {
        unsafe {
            (*unsafe { (*p).p_partner }).i_frame =
                ((i_ofst - 56 as SqliteInt64) /
                            (unsafe { (*p).pgsz } + 24 as u32) as SqliteInt64 +
                        1 as SqliteInt64) as u32
        };
    }
    return rc;
}

///* Read a 32-bit big-endian unsigned integer and return it.
extern "C" fn tmstmp_get_u32(a: *const u8) -> u32 {
    return (((unsafe { *a.offset(0 as isize) } as i32) << 24) +
                        ((unsafe { *a.offset(1 as isize) } as i32) << 16) +
                    ((unsafe { *a.offset(2 as isize) } as i32) << 8) +
                unsafe { *a.offset(3 as isize) } as i32) as u32;
}

///* Write data to a tmstmp-file.
#[allow(unused_doc_comments)]
extern "C" fn tmstmp_write(p_file: *mut Sqlite3File, z_buf: *const (),
    i_amt: i32, i_ofst: Sqlite3Int64) -> i32 {
    let p: *mut TmstmpFile = p_file as *mut TmstmpFile;
    let p_sub: *mut Sqlite3File =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    if (unsafe { (*p).has_correct_reserve } == 0) as i32 != 0
        {} else if unsafe { (*p).is_wal } != 0 {
        if i_amt == 24 {
            /// A frame header
            let mut x: u32 = 0 as u32;
            unsafe {
                (*p).i_frame =
                    ((i_ofst - 32 as SqliteInt64) /
                                (unsafe { (*p).pgsz } + 24 as u32) as SqliteInt64 +
                            1 as SqliteInt64) as u32
            };
            unsafe { (*p).pgno = tmstmp_get_u32(z_buf as *const u8) };
            unsafe {
                (*p).salt1 =
                    tmstmp_get_u32(unsafe {
                            (z_buf as *const u8).offset(8 as isize)
                        })
            };
            unsafe {
                memcpy(&raw mut x as *mut (),
                    unsafe { (z_buf as *const u8).offset(4 as isize) } as
                        *const (), 4 as u64)
            };
            unsafe { (*p).is_commit = (x != 0 as u32) as u8 };
            unsafe { (*p).i_ofst = i_ofst };
        } else if i_amt >= 512 &&
                i_ofst == unsafe { (*p).i_ofst } + 24 as Sqlite3Int64 {
            let mut s: [u8; 16] = [0; 16];
            unsafe {
                memset(&raw mut s[0 as usize] as *mut u8 as *mut (), 0,
                    16 as u64)
            };
            tmstmp_put_ts(unsafe { &*p },
                unsafe {
                    (&raw mut s[0 as usize] as *mut u8).offset(2 as isize)
                });
            tmstmp_event(p, 3 as u8, unsafe { (*p).is_commit },
                unsafe { (*p).pgno }, unsafe { (*p).i_frame },
                unsafe {
                        (&raw mut s[0 as usize] as *mut u8).offset(2 as isize)
                    } as *const u8);
        } else if i_amt == 32 && i_ofst == 0 as i64 {
            unsafe {
                (*p).salt1 =
                    tmstmp_get_u32(unsafe {
                            (z_buf as *const u8).offset(16 as isize)
                        })
            };
            tmstmp_event(p, 8 as u8, 0 as u8, 0 as u32, unsafe { (*p).salt1 },
                core::ptr::null());
        }
    } else if unsafe { (*p).in_ckpt } != 0 {
        let mut s: *mut u8 =
            unsafe {
                unsafe {
                    (z_buf as
                                *mut u8).offset(i_amt as isize).offset(-(16 as isize))
                }
            };
        unsafe { memset(s as *mut (), 0, 16 as u64) };
        tmstmp_put_ts(unsafe { &*p }, unsafe { s.offset(2 as isize) });
        tmstmp_put_u32(unsafe { (*p).i_frame },
            unsafe { s.offset(8 as isize) });
        tmstmp_put_u32(unsafe { (*unsafe { (*p).p_partner }).salt1 } &
                16777215 as u32, unsafe { s.offset(12 as isize) });
        if !(unsafe { (*p).pgsz } > 0 as u32) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"tmstmpWrite".as_ptr() as *const i8,
                    c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 643,
                    c"p->pgsz>0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        tmstmp_event(p, 6 as u8, 0 as u8,
            (i_ofst / unsafe { (*p).pgsz } as SqliteInt64 + 1 as SqliteInt64)
                as u32, unsafe { (*p).i_frame }, core::ptr::null());
    } else if unsafe { (*p).p_partner } == core::ptr::null_mut() {
        /// Writing into a database in rollback mode
        let mut s: *mut u8 =
            unsafe {
                unsafe {
                    (z_buf as
                                *mut u8).offset(i_amt as isize).offset(-(16 as isize))
                }
            };
        unsafe { memset(s as *mut (), 0, 16 as u64) };
        tmstmp_put_ts(unsafe { &*p }, unsafe { s.offset(2 as isize) });
        unsafe { *s.offset(12 as isize) = 2 as u8 };
        if !(unsafe { (*p).pgsz } > 0 as u32) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"tmstmpWrite".as_ptr() as *const i8,
                    c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 651,
                    c"p->pgsz>0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        tmstmp_event(p, 4 as u8, 0 as u8,
            (i_ofst / unsafe { (*p).pgsz } as SqliteInt64) as u32 + 1 as u32,
            0 as u32, unsafe { s.offset(2 as isize) } as *const u8);
    }
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub).p_methods }).x_write.unwrap()
                })(p_sub, z_buf, i_amt, i_ofst)
        };
}

///* Truncate a tmstmp-file.
extern "C" fn tmstmp_truncate(mut p_file: *mut Sqlite3File,
    size: Sqlite3Int64) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_truncate.unwrap()
                })(p_file, size)
        };
}

///* Sync a tmstmp-file.
extern "C" fn tmstmp_sync(mut p_file: *mut Sqlite3File, flags: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_sync.unwrap()
                })(p_file, flags)
        };
}

///* Return the current file-size of a tmstmp-file.
extern "C" fn tmstmp_file_size(mut p_file: *mut Sqlite3File,
    p_size: *mut Sqlite3Int64) -> i32 {
    let p: *mut TmstmpFile = p_file as *mut TmstmpFile;
    p_file =
        unsafe { (p as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_file_size.unwrap()
                })(p_file, p_size)
        };
}

///* Lock a tmstmp-file.
extern "C" fn tmstmp_lock(mut p_file: *mut Sqlite3File, e_lock: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_lock.unwrap()
                })(p_file, e_lock)
        };
}

///* Unlock a tmstmp-file.
extern "C" fn tmstmp_unlock(mut p_file: *mut Sqlite3File, e_lock: i32)
    -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_unlock.unwrap()
                })(p_file, e_lock)
        };
}

///* Check if another file-handle holds a RESERVED lock on a tmstmp-file.
extern "C" fn tmstmp_check_reserved_lock(mut p_file: *mut Sqlite3File,
    p_res_out: *mut i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_file).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(p_file, p_res_out)
        };
}

///* File control method. For custom operations on a tmstmp-file.
extern "C" fn tmstmp_file_control(mut p_file: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut TmstmpFile = p_file as *mut TmstmpFile;
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    rc =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_file_control.unwrap()
                })(p_file, op, p_arg)
        };
    '__s0:
        {
        match op {
            12 => {
                {
                    if unsafe { (*p).has_correct_reserve } != 0 && rc == 0 {
                        unsafe {
                            *(p_arg as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"tmstmp/%z".as_ptr() as *mut i8 as
                                            *const i8, unsafe { *(p_arg as *mut *mut i8) })
                                }
                        };
                    }
                    break '__s0;
                }
                {
                    unsafe { (*p).in_ckpt = 1 as u8 };
                    if (unsafe { (*p).is_db } == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 723,
                                c"p->isDb".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(unsafe { (*p).p_partner } != core::ptr::null_mut()) as
                                    i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 724,
                                c"p->pPartner!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe { (*unsafe { (*p).p_partner }).in_ckpt = 1 as u8 };
                    if unsafe { (*p).has_correct_reserve } != 0 {
                        tmstmp_event(p, 5 as u8, 0 as u8, 0 as u32, 0 as u32,
                            core::ptr::null());
                    }
                    rc = 0;
                    break '__s0;
                }
                {
                    unsafe { (*p).in_ckpt = 0 as u8 };
                    if (unsafe { (*p).is_db } == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 734,
                                c"p->isDb".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(unsafe { (*p).p_partner } != core::ptr::null_mut()) as
                                    i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 735,
                                c"p->pPartner!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe { (*unsafe { (*p).p_partner }).in_ckpt = 0 as u8 };
                    if unsafe { (*p).has_correct_reserve } != 0 {
                        tmstmp_event(p, 7 as u8, 0 as u8, 0 as u32, 0 as u32,
                            core::ptr::null());
                    }
                    rc = 0;
                    break '__s0;
                }
            }
            39 => {
                {
                    unsafe { (*p).in_ckpt = 1 as u8 };
                    if (unsafe { (*p).is_db } == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 723,
                                c"p->isDb".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(unsafe { (*p).p_partner } != core::ptr::null_mut()) as
                                    i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 724,
                                c"p->pPartner!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe { (*unsafe { (*p).p_partner }).in_ckpt = 1 as u8 };
                    if unsafe { (*p).has_correct_reserve } != 0 {
                        tmstmp_event(p, 5 as u8, 0 as u8, 0 as u32, 0 as u32,
                            core::ptr::null());
                    }
                    rc = 0;
                    break '__s0;
                }
                {
                    unsafe { (*p).in_ckpt = 0 as u8 };
                    if (unsafe { (*p).is_db } == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 734,
                                c"p->isDb".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(unsafe { (*p).p_partner } != core::ptr::null_mut()) as
                                    i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 735,
                                c"p->pPartner!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe { (*unsafe { (*p).p_partner }).in_ckpt = 0 as u8 };
                    if unsafe { (*p).has_correct_reserve } != 0 {
                        tmstmp_event(p, 7 as u8, 0 as u8, 0 as u32, 0 as u32,
                            core::ptr::null());
                    }
                    rc = 0;
                    break '__s0;
                }
            }
            37 => {
                {
                    unsafe { (*p).in_ckpt = 0 as u8 };
                    if (unsafe { (*p).is_db } == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 734,
                                c"p->isDb".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if !(unsafe { (*p).p_partner } != core::ptr::null_mut()) as
                                    i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"tmstmpFileControl".as_ptr() as *const i8,
                                c"tmstmpvfs.c".as_ptr() as *mut i8 as *const i8, 735,
                                c"p->pPartner!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe { (*unsafe { (*p).p_partner }).in_ckpt = 0 as u8 };
                    if unsafe { (*p).has_correct_reserve } != 0 {
                        tmstmp_event(p, 7 as u8, 0 as u8, 0 as u32, 0 as u32,
                            core::ptr::null());
                    }
                    rc = 0;
                    break '__s0;
                }
            }
            _ => {}
        }
    }
    return rc;
}

///* Return the sector-size in bytes for a tmstmp-file.
extern "C" fn tmstmp_sector_size(mut p_file: *mut Sqlite3File) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_sector_size.unwrap()
                })(p_file)
        };
}

///* Return the device characteristic flags supported by a tmstmp-file.
extern "C" fn tmstmp_device_characteristics(mut p_file: *mut Sqlite3File)
    -> i32 {
    let mut devchar: i32 = 0;
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
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
extern "C" fn tmstmp_shm_map(mut p_file: *mut Sqlite3File, i_pg: i32,
    pgsz: i32, b_extend: i32, pp: *mut *mut ()) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_map.unwrap()
                })(p_file, i_pg, pgsz, b_extend, pp)
        };
}

/// Perform locking on a shared-memory segment
extern "C" fn tmstmp_shm_lock(mut p_file: *mut Sqlite3File, offset: i32,
    n: i32, flags: i32) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_lock.unwrap()
                })(p_file, offset, n, flags)
        };
}

/// Memory barrier operation on shared memory
extern "C" fn tmstmp_shm_barrier(mut p_file: *mut Sqlite3File) -> () {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    unsafe {
        (unsafe {
                (*unsafe { (*p_file).p_methods }).x_shm_barrier.unwrap()
            })(p_file)
    };
}

/// Unmap a shared memory segment
extern "C" fn tmstmp_shm_unmap(mut p_file: *mut Sqlite3File, delete_flag: i32)
    -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_shm_unmap.unwrap()
                })(p_file, delete_flag)
        };
}

/// Fetch a page of a memory-mapped file
extern "C" fn tmstmp_fetch(mut p_file: *mut Sqlite3File, i_ofst: Sqlite3Int64,
    i_amt: i32, pp: *mut *mut ()) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_fetch.unwrap()
                })(p_file, i_ofst, i_amt, pp)
        };
}

/// Release a memory-mapped page
extern "C" fn tmstmp_unfetch(mut p_file: *mut Sqlite3File,
    i_ofst: Sqlite3Int64, p_page: *mut ()) -> i32 {
    p_file =
        unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
            *mut Sqlite3File;
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_file).p_methods }).x_unfetch.unwrap()
                })(p_file, i_ofst, p_page)
        };
}

static tmstmp_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 3,
        x_close: Some(tmstmp_close),
        x_read: Some(tmstmp_read),
        x_write: Some(tmstmp_write),
        x_truncate: Some(tmstmp_truncate),
        x_sync: Some(tmstmp_sync),
        x_file_size: Some(tmstmp_file_size),
        x_lock: Some(tmstmp_lock),
        x_unlock: Some(tmstmp_unlock),
        x_check_reserved_lock: Some(tmstmp_check_reserved_lock),
        x_file_control: Some(tmstmp_file_control),
        x_sector_size: Some(tmstmp_sector_size),
        x_device_characteristics: Some(tmstmp_device_characteristics),
        x_shm_map: Some(tmstmp_shm_map),
        x_shm_lock: Some(tmstmp_shm_lock),
        x_shm_barrier: Some(tmstmp_shm_barrier),
        x_shm_unmap: Some(tmstmp_shm_unmap),
        x_fetch: Some(tmstmp_fetch),
        x_unfetch: Some(tmstmp_unfetch),
    };

///* Methods for TmstmpVfs
#[allow(unused_doc_comments)]
extern "C" fn tmstmp_open(p_vfs: *mut Sqlite3Vfs, z_name: *const i8,
    p_file: *mut Sqlite3File, flags: i32, p_out_flags: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    '__b1: loop {
        '__c1: loop {
            let mut p: *mut TmstmpFile = core::ptr::null_mut();
            let mut p_db: *mut TmstmpFile = core::ptr::null_mut();
            let mut p_sub_file: *mut Sqlite3File = core::ptr::null_mut();
            let mut p_sub_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
            p_sub_vfs = unsafe { (*p_vfs).p_app_data } as *mut Sqlite3Vfs;
            if flags & (256 | 524288) == 0 {

                /// If the file is not a persistent database or a WAL file, then
                ///* bypass the timestamp logic all together
                return unsafe {
                        (unsafe {
                                (*p_sub_vfs).x_open.unwrap()
                            })(p_sub_vfs, z_name, p_file, flags, p_out_flags)
                    };
            }
            if flags & 524288 != 0 {
                p_db =
                    unsafe { sqlite3_database_file_object(z_name) } as
                        *mut TmstmpFile;
                if p_db == core::ptr::null_mut() ||
                                unsafe { (*p_db).u_magic } != 713537325 as u32 ||
                            (unsafe { (*p_db).is_db } == 0) as i32 != 0 ||
                        unsafe { (*p_db).p_partner } != core::ptr::null_mut() {
                    return unsafe {
                            (unsafe {
                                    (*p_sub_vfs).x_open.unwrap()
                                })(p_sub_vfs, z_name, p_file, flags, p_out_flags)
                        };
                }
            } else { p_db = core::ptr::null_mut(); }
            p = p_file as *mut TmstmpFile;
            unsafe {
                memset(p as *mut (), 0,
                    core::mem::size_of::<TmstmpFile>() as u64)
            };
            p_sub_file =
                unsafe { (p_file as *mut TmstmpFile).offset(1 as isize) } as
                    *mut Sqlite3File;
            unsafe { (*p_file).p_methods = &tmstmp_io_methods };
            unsafe { (*p).p_sub_vfs = p_sub_vfs };
            unsafe { (*p).u_magic = 713537325 as u32 };
            rc =
                unsafe {
                    (unsafe {
                            (*p_sub_vfs).x_open.unwrap()
                        })(p_sub_vfs, z_name, p_sub_file, flags, p_out_flags)
                };
            if rc != 0 { break '__b1; }
            if p_db != core::ptr::null_mut() {
                unsafe { (*p).is_wal = 1 as u8 };
                unsafe { (*p).p_partner = p_db };
                unsafe { (*p_db).p_partner = p };
            } else {
                let mut r2: u32 = 0 as u32;
                let mut pid: u32 = 0 as u32;
                let mut p_log: *mut TmstmpLog = core::ptr::null_mut();
                let mut r1: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                /// Milliseconds since 1970-01-01
                let mut days: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                /// Days since 1970-01-01
                let mut sod: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                /// Start of date specified by r1
                let mut z: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                /// Days since 0000-03-01
                let mut era: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                /// 400-year era
                let mut h: i32 = 0;
                /// hour
                let mut m: i32 = 0;
                /// minute
                let mut s: i32 = 0;
                /// second
                let mut f: i32 = 0;
                /// millisecond
                let mut y: i32 = 0;
                /// year
                let mut m: i32 = 0;
                /// month
                let mut d: i32 = 0;
                /// day
                let mut y: i32 = 0;
                /// year assuming March is first month
                let mut doe: u32 = 0 as u32;
                /// day of 400-year era
                let mut yoe: u32 = 0 as u32;
                /// year of 400-year era
                let mut doy: u32 = 0 as u32;
                /// day of year
                let mut mp: u32 = 0 as u32;

                /// month with March==0
                unsafe { (*p).is_db = 1 as u8 };
                r1 = 0 as Sqlite3Uint64;
                p_log =
                    unsafe {
                            sqlite3_malloc64(core::mem::size_of::<TmstmpLog>() as
                                    Sqlite3Uint64)
                        } as *mut TmstmpLog;
                if p_log == core::ptr::null_mut() {
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_sub_file).p_methods }).x_close.unwrap()
                            })(p_sub_file)
                    };
                    rc = 7;
                    break '__b1;
                }
                unsafe {
                    memset(p_log as *mut (), 0,
                        core::mem::size_of::<TmstmpLog>() as u64)
                };
                unsafe { (*p).p_log = p_log };
                unsafe {
                    (unsafe {
                            (*unsafe { (*p).p_sub_vfs }).x_current_time_int64.unwrap()
                        })(unsafe { (*p).p_sub_vfs },
                        &raw mut r1 as *mut Sqlite3Int64)
                };
                r1 -= 210866760000000i64 as Sqlite3Uint64;
                days = r1 / 86400000 as Sqlite3Uint64;
                sod = r1 % 86400000 as Sqlite3Uint64 / 1000 as Sqlite3Uint64;
                f = (r1 % 1000 as Sqlite3Uint64) as i32;
                h = (sod / 3600 as Sqlite3Uint64) as i32;
                m =
                    (sod % 3600 as Sqlite3Uint64 / 60 as Sqlite3Uint64) as i32;
                s = (sod % 60 as Sqlite3Uint64) as i32;
                z = days + 719468 as Sqlite3Uint64;
                era = z / 146097 as Sqlite3Uint64;
                doe = (z - era * 146097 as Sqlite3Uint64) as u32;
                yoe =
                    (doe - doe / 1460 as u32 + doe / 36524 as u32 -
                            doe / 146096 as u32) / 365 as u32;
                y =
                    (yoe as i32 as Sqlite3Uint64 + era * 400 as Sqlite3Uint64)
                        as i32;
                doy =
                    doe -
                        (365 as u32 * yoe + yoe / 4 as u32 - yoe / 100 as u32);
                mp = (5 as u32 * doy + 2 as u32) / 153 as u32;
                d =
                    (doy - (153 as u32 * mp + 2 as u32) / 5 as u32 + 1 as u32)
                        as i32;
                m = (mp + if mp < 10 as u32 { 3 } else { -9 } as u32) as i32;
                y = y + (m <= 2) as i32;
                unsafe {
                    sqlite3_randomness(core::mem::size_of::<u32>() as i32,
                        &raw mut r2 as *mut ())
                };
                pid = unsafe { getpid() } as u32;
                unsafe {
                    (*p_log).z_logname =
                        unsafe {
                            sqlite3_mprintf(c"%s-tmstmp/%04d%02d%02dT%02d%02d%02d%03d-%08d-%08x".as_ptr()
                                        as *mut i8 as *const i8, z_name, y, m, d, h, m, s, f, pid,
                                r2)
                        }
                };
            }
            tmstmp_event(p,
                if unsafe { (*p).is_wal } != 0 { 2 } else { 1 } as u8,
                0 as u8, unsafe { getpid() } as u32, 0 as u32,
                core::ptr::null());
            break '__c1;
        }
        if !(false) { break '__b1; }
    }
    if rc != 0 { unsafe { (*p_file).p_methods = core::ptr::null() }; }
    return rc;
}

///* All VFS interfaces other than xOpen are passed down into the Sub-VFS.
extern "C" fn tmstmp_delete(p: *mut Sqlite3Vfs, z_name: *const i8,
    sync_dir: i32) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_delete.unwrap() })(p_sub, z_name, sync_dir)
        };
}

extern "C" fn tmstmp_access(p: *mut Sqlite3Vfs, z_name: *const i8, flags: i32,
    p_r: *mut i32) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_access.unwrap() })(p_sub, z_name, flags, p_r)
        };
}

extern "C" fn tmstmp_full_pathname(p: *mut Sqlite3Vfs, z_name: *const i8,
    n: i32, z_out: *mut i8) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe {
                    (*p_sub).x_full_pathname.unwrap()
                })(p_sub, z_name, n, z_out)
        };
}

extern "C" fn tmstmp_dl_open(p: *mut Sqlite3Vfs, z_filename: *const i8)
    -> *mut () {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_dl_open.unwrap() })(p_sub, z_filename)
        };
}

extern "C" fn tmstmp_dl_error(p: *mut Sqlite3Vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe {
                    (*p_sub).x_dl_error.unwrap()
                })(p_sub, n_byte, z_err_msg)
        };
}

extern "C" fn tmstmp_dl_sym(p: *mut Sqlite3Vfs, p_dl: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_dl_sym.unwrap() })(p_sub, p_dl, z_sym)
        };
}

extern "C" fn tmstmp_dl_close(p: *mut Sqlite3Vfs, p_dl: *mut ()) -> () {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe { (unsafe { (*p_sub).x_dl_close.unwrap() })(p_sub, p_dl) };
}

extern "C" fn tmstmp_randomness(p: *mut Sqlite3Vfs, n_byte: i32,
    z_out: *mut i8) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_randomness.unwrap() })(p_sub, n_byte, z_out)
        };
}

extern "C" fn tmstmp_sleep(p: *mut Sqlite3Vfs, microseconds: i32) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_sleep.unwrap() })(p_sub, microseconds)
        };
}

extern "C" fn tmstmp_current_time(p: *mut Sqlite3Vfs, pr_now: *mut f64)
    -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_current_time.unwrap() })(p_sub, pr_now)
        };
}

extern "C" fn tmstmp_get_last_error(p: *mut Sqlite3Vfs, a: i32, b: *mut i8)
    -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_get_last_error.unwrap() })(p_sub, a, b)
        };
}

extern "C" fn tmstmp_current_time_int64(p: *mut Sqlite3Vfs,
    pi_now: *mut Sqlite3Int64) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_current_time_int64.unwrap() })(p_sub, pi_now)
        };
}

extern "C" fn tmstmp_set_system_call(p: *mut Sqlite3Vfs, z_name: *const i8,
    x: unsafe extern "C" fn() -> ()) -> i32 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_set_system_call.unwrap() })(p_sub, z_name, x)
        };
}

extern "C" fn tmstmp_get_system_call(p: *mut Sqlite3Vfs, z: *const i8)
    -> unsafe extern "C" fn() -> () {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_get_system_call.unwrap() })(p_sub, z)
        };
}

extern "C" fn tmstmp_next_system_call(p: *mut Sqlite3Vfs, z_name: *const i8)
    -> *const i8 {
    let p_sub: *mut Sqlite3Vfs =
        unsafe { (*p).p_app_data } as *mut Sqlite3Vfs;
    return unsafe {
            (unsafe { (*p_sub).x_next_system_call.unwrap() })(p_sub, z_name)
        };
}

static mut tmstmp_vfs: Sqlite3Vfs =
    Sqlite3Vfs {
        i_version: 3,
        sz_os_file: 0,
        mx_pathname: 1024,
        p_next: core::ptr::null_mut(),
        z_name: c"tmstmpvfs".as_ptr() as *const i8,
        p_app_data: core::ptr::null_mut(),
        x_open: Some(tmstmp_open),
        x_delete: Some(tmstmp_delete),
        x_access: Some(tmstmp_access),
        x_full_pathname: Some(tmstmp_full_pathname),
        x_dl_open: Some(tmstmp_dl_open),
        x_dl_error: Some(tmstmp_dl_error),
        x_dl_sym: Some(tmstmp_dl_sym),
        x_dl_close: Some(tmstmp_dl_close),
        x_randomness: Some(tmstmp_randomness),
        x_sleep: Some(tmstmp_sleep),
        x_current_time: Some(tmstmp_current_time),
        x_get_last_error: Some(tmstmp_get_last_error),
        x_current_time_int64: Some(tmstmp_current_time_int64),
        x_set_system_call: Some(tmstmp_set_system_call),
        x_get_system_call: Some(tmstmp_get_system_call),
        x_next_system_call: Some(tmstmp_next_system_call),
    };

///* Register the tmstmp VFS as the default VFS for the system.
extern "C" fn tmstmp_register_vfs() -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_orig: *mut Sqlite3Vfs =
            unsafe { sqlite3_vfs_find(core::ptr::null()) };
        if p_orig == core::ptr::null_mut() { return 1; }
        if p_orig == &raw mut tmstmp_vfs as *mut Sqlite3Vfs { return 0; }
        tmstmp_vfs.i_version = unsafe { (*p_orig).i_version };
        tmstmp_vfs.p_app_data = p_orig as *mut ();
        tmstmp_vfs.sz_os_file =
            (unsafe { (*p_orig).sz_os_file } as u64 +
                    core::mem::size_of::<TmstmpFile>() as u64) as i32;
        rc = unsafe { sqlite3_vfs_register(&mut tmstmp_vfs, 1) };
        return rc;
    }
}

/// 
///* This routine is called by sqlite3_load_extension() when the
///* extension is first loaded.
///*
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_tmstmpvfs_init(db: *const Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };

    /// not used
    { let _ = db; };

    /// not used
    /// not used
    (rc = tmstmp_register_vfs());
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
    fn fclose(_: *mut FILE)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn fflush(_: *mut FILE)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn getpid()
    -> PidT;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
