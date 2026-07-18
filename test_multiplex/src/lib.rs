#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;
type DarwinSizeT = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct AnonS0 {
    p_orig_vfs: *mut Sqlite3Vfs,
    s_this_vfs: Sqlite3Vfs,
    s_io_methods_v1: Sqlite3IoMethods,
    s_io_methods_v2: Sqlite3IoMethods,
    is_initialized: i32,
}
static mut g_multiplex: AnonS0 = unsafe { core::mem::zeroed() };
#[repr(C)]
#[derive(Copy, Clone)]
struct MultiplexConn {
    base: Sqlite3File,
    p_group: *mut MultiplexGroup,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct MultiplexGroup {
    a_real: *mut MultiplexReal,
    n_real: i32,
    z_name: *mut i8,
    n_name: i32,
    flags: i32,
    sz_chunk: u32,
    b_enabled: u8,
    b_truncate: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct MultiplexReal {
    p: *mut Sqlite3File,
    z: *mut i8,
}
extern "C" fn multiplex_strlen30(z: *const i8) -> i32 {
    let mut z2: *const i8 = z;
    if z == core::ptr::null() { return 0; }
    while unsafe { *z2 } != 0 {
        {
            let __p = &mut z2;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return 1073741823 & unsafe { z2.offset_from(z) } as i64 as i32;
}
extern "C" fn multiplex_filename(z_base_1: *const i8, n_base_1: i32,
    flags: i32, i_chunk_1: i32, z_out_1: *mut i8) -> () {
    let mut n: i32 = n_base_1;
    unsafe {
        memcpy(z_out_1 as *mut (), z_base_1 as *const (), (n + 1) as u64)
    };
    if i_chunk_1 != 0 && i_chunk_1 <= 299 {
        unsafe {
            sqlite3_snprintf(4, unsafe { &mut *z_out_1.offset(n as isize) },
                c"%03d".as_ptr() as *mut i8 as *const i8, i_chunk_1)
        };
        n += 3;
    }
    if !(unsafe { *z_out_1.offset(n as isize) } as i32 == '\u{0}' as i32) as
                    i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"multiplexFilename".as_ptr() as *const i8,
                c"test_multiplex.c".as_ptr() as *mut i8 as *const i8, 250,
                c"zOut[n]==\'\\0\'".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe { *z_out_1.offset((n + 1) as isize) = '\u{0}' as i32 as i8 };
}
extern "C" fn multiplex_sub_filename(p_group_1: &mut MultiplexGroup,
    i_chunk_1: i32) -> i32 {
    if i_chunk_1 >= (*p_group_1).n_real {
        let mut p: *mut MultiplexReal = core::ptr::null_mut();
        p =
            unsafe {
                    sqlite3_realloc64((*p_group_1).a_real as *mut (),
                        (i_chunk_1 + 1) as u64 *
                            core::mem::size_of::<MultiplexReal>() as u64)
                } as *mut MultiplexReal;
        if p == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(unsafe { &raw mut *p.offset((*p_group_1).n_real as isize) }
                    as *mut (), 0,
                core::mem::size_of::<MultiplexReal>() as u64 *
                    (i_chunk_1 + 1 - (*p_group_1).n_real) as u64)
        };
        (*p_group_1).a_real = p;
        (*p_group_1).n_real = i_chunk_1 + 1;
    }
    if !((*p_group_1).z_name).is_null() &&
            unsafe { (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).z } ==
                core::ptr::null_mut() {
        let mut z: *mut i8 = core::ptr::null_mut();
        let n: i32 = (*p_group_1).n_name;
        z = unsafe { sqlite3_malloc64((n + 5) as Sqlite3Uint64) } as *mut i8;
        if z == core::ptr::null_mut() { return 7; }
        multiplex_filename((*p_group_1).z_name as *const i8,
            (*p_group_1).n_name, (*p_group_1).flags, i_chunk_1, z);
        unsafe {
            (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).z =
                unsafe {
                        sqlite3_create_filename(z as *const i8,
                            c"".as_ptr() as *mut i8 as *const i8,
                            c"".as_ptr() as *mut i8 as *const i8, 0,
                            core::ptr::null_mut())
                    } as *mut i8
        };
        unsafe { sqlite3_free(z as *mut ()) };
        if unsafe { (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).z } ==
                core::ptr::null_mut() {
            return 7;
        }
    }
    return 0;
}
extern "C" fn multiplex_sub_open(p_group_1: *mut MultiplexGroup,
    i_chunk_1: i32, rc: &mut i32, p_out_flags_1: *mut i32, create_flag_1: i32)
    -> *mut Sqlite3File {
    unsafe {
        let mut p_sub_open: *mut Sqlite3File = core::ptr::null_mut();
        let p_orig_vfs: *mut Sqlite3Vfs = g_multiplex.p_orig_vfs;
        *rc = multiplex_sub_filename(unsafe { &mut *p_group_1 }, i_chunk_1);
        if *rc == 0 &&
                {
                        p_sub_open =
                            unsafe {
                                (*unsafe {
                                            (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                        }).p
                            };
                        p_sub_open
                    } == core::ptr::null_mut() {
            let mut flags: i32 = 0;
            let mut b_exists: i32 = 0;
            flags = unsafe { (*p_group_1).flags };
            if create_flag_1 != 0 {
                flags |= 4;
            } else if i_chunk_1 == 0
                {} else if unsafe {
                        (*unsafe {
                                    (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                }).z
                    } == core::ptr::null_mut() {
                return core::ptr::null_mut();
            } else {
                *rc =
                    unsafe {
                        (unsafe {
                                (*p_orig_vfs).x_access.unwrap()
                            })(p_orig_vfs,
                            unsafe {
                                    (*unsafe {
                                                (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                            }).z
                                } as *const i8, 0, &mut b_exists)
                    };
                if *rc != 0 || (b_exists == 0) as i32 != 0 {
                    if *rc != 0 {
                        unsafe {
                            sqlite3_log(*rc,
                                c"multiplexor.xAccess failure on %s".as_ptr() as *mut i8 as
                                    *const i8,
                                unsafe {
                                    (*unsafe {
                                                (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                            }).z
                                })
                        };
                    }
                    return core::ptr::null_mut();
                }
                flags &= !4;
            }
            p_sub_open =
                unsafe {
                        sqlite3_malloc64(unsafe { (*p_orig_vfs).sz_os_file } as
                                Sqlite3Uint64)
                    } as *mut Sqlite3File;
            if p_sub_open == core::ptr::null_mut() {
                *rc = 10 | 12 << 8;
                return core::ptr::null_mut();
            }
            unsafe {
                (*unsafe { (*p_group_1).a_real.offset(i_chunk_1 as isize) }).p
                    = p_sub_open
            };
            *rc =
                unsafe {
                    (unsafe {
                            (*p_orig_vfs).x_open.unwrap()
                        })(p_orig_vfs,
                        unsafe {
                                (*unsafe {
                                            (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                        }).z
                            } as *const i8, p_sub_open, flags, p_out_flags_1)
                };
            if *rc != 0 {
                unsafe {
                    sqlite3_log(*rc,
                        c"multiplexor.xOpen failure on %s".as_ptr() as *mut i8 as
                            *const i8,
                        unsafe {
                            (*unsafe {
                                        (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                    }).z
                        })
                };
                unsafe { sqlite3_free(p_sub_open as *mut ()) };
                unsafe {
                    (*unsafe {
                                    (*p_group_1).a_real.offset(i_chunk_1 as isize)
                                }).p = core::ptr::null_mut()
                };
                return core::ptr::null_mut();
            }
        }
        return p_sub_open;
    }
}
extern "C" fn multiplex_sub_size(p_group_1: *mut MultiplexGroup,
    i_chunk_1: i32, rc: *mut i32) -> Sqlite3Int64 {
    let mut p_sub: *mut Sqlite3File = core::ptr::null_mut();
    let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
    if unsafe { *rc } != 0 { return 0 as Sqlite3Int64; }
    p_sub =
        multiplex_sub_open(p_group_1, i_chunk_1, unsafe { &mut *rc },
            0 as *mut () as *mut i32, 0);
    if p_sub == core::ptr::null_mut() { return 0 as Sqlite3Int64; }
    unsafe {
        *rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_sub).p_methods }).x_file_size.unwrap()
                    })(p_sub, &mut sz)
            }
    };
    return sz;
}
extern "C" fn multiplex_sub_close(p_group_1: &MultiplexGroup, i_chunk_1: i32,
    p_orig_vfs_1: *mut Sqlite3Vfs) -> () {
    let p_sub_open: *mut Sqlite3File =
        unsafe { (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).p };
    if !(p_sub_open).is_null() {
        unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_close.unwrap()
                })(p_sub_open)
        };
        if !(p_orig_vfs_1).is_null() &&
                !(unsafe {
                                (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).z
                            }).is_null() {
            unsafe {
                (unsafe {
                        (*p_orig_vfs_1).x_delete.unwrap()
                    })(p_orig_vfs_1,
                    unsafe {
                            (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).z
                        } as *const i8, 0)
            };
        }
        unsafe {
            sqlite3_free(unsafe {
                        (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).p
                    } as *mut ())
        };
    }
    unsafe {
        sqlite3_free_filename(unsafe {
                    (*(*p_group_1).a_real.offset(i_chunk_1 as isize)).z
                } as Sqlite3Filename)
    };
    unsafe {
        memset(unsafe {
                    &raw mut *(*p_group_1).a_real.offset(i_chunk_1 as isize)
                } as *mut (), 0, core::mem::size_of::<MultiplexReal>() as u64)
    };
}
extern "C" fn multiplex_free_components(p_group_1: *mut MultiplexGroup)
    -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b1: loop {
            if !(i < unsafe { (*p_group_1).n_real }) { break '__b1; }
            '__c1: loop {
                multiplex_sub_close(unsafe { &*p_group_1 }, i,
                    core::ptr::null_mut());
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p_group_1).a_real } as *mut ()) };
    unsafe { (*p_group_1).a_real = core::ptr::null_mut() };
    unsafe { (*p_group_1).n_real = 0 };
}
extern "C" fn multiplex_open(p_vfs_1: *mut Sqlite3Vfs, z_name_1: *const i8,
    p_conn_1: *mut Sqlite3File, flags: i32, p_out_flags_1: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_multiplex_open: *mut MultiplexConn = core::ptr::null_mut();
        let mut p_group: *mut MultiplexGroup = core::ptr::null_mut();
        let mut p_sub_open: *mut Sqlite3File = core::ptr::null_mut();
        let p_orig_vfs: *mut Sqlite3Vfs = g_multiplex.p_orig_vfs;
        let mut n_name: i32 = 0;
        let mut sz: i32 = 0;
        let z_to_free: *mut i8 = core::ptr::null_mut();
        { let _ = p_vfs_1; };
        unsafe {
            memset(p_conn_1 as *mut (), 0,
                unsafe { (*p_vfs_1).sz_os_file } as u64)
        };
        if !(!(z_name_1).is_null() || flags & 8 != 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"multiplexOpen".as_ptr() as *const i8,
                    c"test_multiplex.c".as_ptr() as *mut i8 as *const i8, 487,
                    c"zName || (flags & SQLITE_OPEN_DELETEONCLOSE)".as_ptr() as
                            *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        p_multiplex_open = p_conn_1 as *mut MultiplexConn;
        if rc == 0 {
            n_name =
                if !(z_name_1).is_null() {
                    multiplex_strlen30(z_name_1)
                } else { 0 };
            sz =
                (core::mem::size_of::<MultiplexGroup>() as u64 + n_name as u64
                        + 1 as u64) as i32;
            p_group =
                unsafe { sqlite3_malloc64(sz as Sqlite3Uint64) } as
                    *mut MultiplexGroup;
            if p_group == core::ptr::null_mut() { rc = 7; }
        }
        if rc == 0 {
            let z_uri: *const i8 =
                if flags & 64 != 0 { z_name_1 } else { core::ptr::null() };
            unsafe { memset(p_group as *mut (), 0, sz as u64) };
            unsafe { (*p_multiplex_open).p_group = p_group };
            unsafe { (*p_group).b_enabled = -1i32 as u8 };
            unsafe {
                (*p_group).b_truncate =
                    unsafe {
                            sqlite3_uri_boolean(z_uri,
                                c"truncate".as_ptr() as *mut i8 as *const i8,
                                (flags & 256 == 0) as i32)
                        } as u8
            };
            unsafe {
                (*p_group).sz_chunk =
                    unsafe {
                                sqlite3_uri_int64(z_uri,
                                    c"chunksize".as_ptr() as *mut i8 as *const i8,
                                    2147418112 as Sqlite3Int64)
                            } as i32 as u32
            };
            unsafe {
                (*p_group).sz_chunk =
                    unsafe { (*p_group).sz_chunk } + 65535 as u32 &
                        !65535 as u32
            };
            if !(z_name_1).is_null() {
                let p: *mut i8 =
                    unsafe { &raw mut *p_group.offset(1 as isize) } as *mut i8;
                unsafe { (*p_group).z_name = p };
                unsafe {
                    memcpy(unsafe { (*p_group).z_name } as *mut (),
                        z_name_1 as *const (), (n_name + 1) as u64)
                };
                unsafe { (*p_group).n_name = n_name };
            }
            if unsafe { (*p_group).b_enabled } != 0 {
                while sqlite3_pending_byte as u32 %
                            unsafe { (*p_group).sz_chunk } >=
                        unsafe { (*p_group).sz_chunk } - 65536 as u32 {
                    unsafe { (*p_group).sz_chunk += 65536 as u32 };
                }
            }
            unsafe { (*p_group).flags = flags & !64 };
            rc = multiplex_sub_filename(unsafe { &mut *p_group }, 1);
            if rc == 0 {
                p_sub_open =
                    multiplex_sub_open(p_group, 0, &mut rc, p_out_flags_1, 0);
                if p_sub_open == core::ptr::null_mut() && rc == 0 { rc = 14; }
            }
            if rc == 0 {
                let mut sz64: Sqlite3Int64 = 0 as Sqlite3Int64;
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_sub_open).p_methods }).x_file_size.unwrap()
                            })(p_sub_open, &mut sz64)
                    };
                if rc == 0 && !(z_name_1).is_null() {
                    let mut b_exists: i32 = 0;
                    if flags & 16384 != 0 {
                        unsafe { (*p_group).b_enabled = 0 as u8 };
                    } else if sz64 == 0 as i64 {
                        if flags & 2048 != 0 {
                            let mut i_chunk: i32 = 1;
                            '__b3: loop {
                                '__c3: loop {
                                    rc =
                                        unsafe {
                                            (unsafe {
                                                    (*p_orig_vfs).x_access.unwrap()
                                                })(p_orig_vfs,
                                                unsafe {
                                                        (*unsafe { (*p_group).a_real.offset(i_chunk as isize) }).z
                                                    } as *const i8, 0, &mut b_exists)
                                        };
                                    if rc == 0 && b_exists != 0 {
                                        rc =
                                            unsafe {
                                                (unsafe {
                                                        (*p_orig_vfs).x_delete.unwrap()
                                                    })(p_orig_vfs,
                                                    unsafe {
                                                            (*unsafe { (*p_group).a_real.offset(i_chunk as isize) }).z
                                                        } as *const i8, 0)
                                            };
                                        if rc == 0 {
                                            rc =
                                                multiplex_sub_filename(unsafe { &mut *p_group },
                                                    { let __p = &mut i_chunk; *__p += 1; *__p });
                                        }
                                    }
                                    break '__c3;
                                }
                                if !(rc == 0 && b_exists != 0) { break '__b3; }
                            }
                        }
                    } else {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*p_orig_vfs).x_access.unwrap()
                                    })(p_orig_vfs,
                                    unsafe {
                                            (*unsafe { (*p_group).a_real.offset(1 as isize) }).z
                                        } as *const i8, 0, &mut b_exists)
                            };
                        b_exists =
                            (multiplex_sub_size(p_group, 1, &mut rc) > 0 as i64) as i32;
                        if rc == 0 && b_exists != 0 &&
                                        sz64 == sz64 & 4294901760u32 as Sqlite3Int64 &&
                                    sz64 > 0 as i64 &&
                                sz64 != unsafe { (*p_group).sz_chunk } as i64 {
                            unsafe { (*p_group).sz_chunk = sz64 as i32 as u32 };
                        } else if rc == 0 && (b_exists == 0) as i32 != 0 &&
                                sz64 > unsafe { (*p_group).sz_chunk } as i64 {
                            unsafe { (*p_group).b_enabled = 0 as u8 };
                        }
                    }
                }
            }
            if rc == 0 {
                if unsafe { (*unsafe { (*p_sub_open).p_methods }).i_version }
                            as i32 == 1 {
                    unsafe {
                        (*p_conn_1).p_methods =
                            &raw mut g_multiplex.s_io_methods_v1 as
                                *const Sqlite3IoMethods
                    };
                } else {
                    unsafe {
                        (*p_conn_1).p_methods =
                            &raw mut g_multiplex.s_io_methods_v2 as
                                *const Sqlite3IoMethods
                    };
                }
            } else {
                multiplex_free_components(p_group);
                unsafe { sqlite3_free(p_group as *mut ()) };
            }
        }
        unsafe { sqlite3_free(z_to_free as *mut ()) };
        return rc;
    }
}
extern "C" fn multiplex_delete(p_vfs_1: *mut Sqlite3Vfs, z_name_1: *const i8,
    sync_dir_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_orig_vfs: *mut Sqlite3Vfs = g_multiplex.p_orig_vfs;
        rc =
            unsafe {
                (unsafe {
                        (*p_orig_vfs).x_delete.unwrap()
                    })(p_orig_vfs, z_name_1, sync_dir_1)
            };
        if rc == 0 {
            let n_name: i32 = unsafe { strlen(z_name_1) } as i32;
            let mut z: *mut i8 = core::ptr::null_mut();
            z =
                unsafe { sqlite3_malloc64((n_name + 5) as Sqlite3Uint64) } as
                    *mut i8;
            if z == core::ptr::null_mut() {
                rc = 10 | 12 << 8;
            } else {
                let mut i_chunk: i32 = 0;
                let mut b_exists: i32 = 0;
                '__b4: loop {
                    '__c4: loop {
                        multiplex_filename(z_name_1, n_name, 2048,
                            { let __p = &mut i_chunk; *__p += 1; *__p }, z);
                        rc =
                            unsafe {
                                (unsafe {
                                        (*p_orig_vfs).x_access.unwrap()
                                    })(p_orig_vfs, z as *const i8, 0, &mut b_exists)
                            };
                        break '__c4;
                    }
                    if !(rc == 0 && b_exists != 0) { break '__b4; }
                }
                while rc == 0 && i_chunk > 1 {
                    multiplex_filename(z_name_1, n_name, 2048,
                        { let __p = &mut i_chunk; *__p -= 1; *__p }, z);
                    rc =
                        unsafe {
                            (unsafe {
                                    (*p_orig_vfs).x_delete.unwrap()
                                })(p_orig_vfs, z as *const i8, sync_dir_1)
                        };
                }
                if rc == 0 {
                    i_chunk = 0;
                    '__b6: loop {
                        '__c6: loop {
                            multiplex_filename(z_name_1, n_name, 524288,
                                { let __p = &mut i_chunk; *__p += 1; *__p }, z);
                            rc =
                                unsafe {
                                    (unsafe {
                                            (*p_orig_vfs).x_access.unwrap()
                                        })(p_orig_vfs, z as *const i8, 0, &mut b_exists)
                                };
                            break '__c6;
                        }
                        if !(rc == 0 && b_exists != 0) { break '__b6; }
                    }
                    while rc == 0 && i_chunk > 1 {
                        multiplex_filename(z_name_1, n_name, 524288,
                            { let __p = &mut i_chunk; *__p -= 1; *__p }, z);
                        rc =
                            unsafe {
                                (unsafe {
                                        (*p_orig_vfs).x_delete.unwrap()
                                    })(p_orig_vfs, z as *const i8, sync_dir_1)
                            };
                    }
                }
            }
            unsafe { sqlite3_free(z as *mut ()) };
        }
        return rc;
    }
}
extern "C" fn multiplex_access(a: *mut Sqlite3Vfs, b: *const i8, c: i32,
    d: *mut i32) -> i32 {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_access.unwrap()
                    })(g_multiplex.p_orig_vfs, b, c, d)
            };
    }
}
extern "C" fn multiplex_full_pathname(a: *mut Sqlite3Vfs, b: *const i8,
    c: i32, d: *mut i8) -> i32 {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_full_pathname.unwrap()
                    })(g_multiplex.p_orig_vfs, b, c, d)
            };
    }
}
extern "C" fn multiplex_dl_open(a: *mut Sqlite3Vfs, b: *const i8) -> *mut () {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_dl_open.unwrap()
                    })(g_multiplex.p_orig_vfs, b)
            };
    }
}
extern "C" fn multiplex_dl_error(a: *mut Sqlite3Vfs, b: i32, c: *mut i8)
    -> () {
    unsafe {
        unsafe {
            (unsafe {
                    (*g_multiplex.p_orig_vfs).x_dl_error.unwrap()
                })(g_multiplex.p_orig_vfs, b, c)
        };
    }
}
extern "C" fn multiplex_dl_sym(a: *mut Sqlite3Vfs, b: *mut (), c: *const i8)
    -> unsafe extern "C" fn() -> () {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_dl_sym.unwrap()
                    })(g_multiplex.p_orig_vfs, b, c)
            };
    }
}
extern "C" fn multiplex_dl_close(a: *mut Sqlite3Vfs, b: *mut ()) -> () {
    unsafe {
        unsafe {
            (unsafe {
                    (*g_multiplex.p_orig_vfs).x_dl_close.unwrap()
                })(g_multiplex.p_orig_vfs, b)
        };
    }
}
extern "C" fn multiplex_randomness(a: *mut Sqlite3Vfs, b: i32, c: *mut i8)
    -> i32 {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_randomness.unwrap()
                    })(g_multiplex.p_orig_vfs, b, c)
            };
    }
}
extern "C" fn multiplex_sleep(a: *mut Sqlite3Vfs, b: i32) -> i32 {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_sleep.unwrap()
                    })(g_multiplex.p_orig_vfs, b)
            };
    }
}
extern "C" fn multiplex_current_time(a: *mut Sqlite3Vfs, b: *mut f64) -> i32 {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_current_time.unwrap()
                    })(g_multiplex.p_orig_vfs, b)
            };
    }
}
extern "C" fn multiplex_get_last_error(a: *mut Sqlite3Vfs, b: i32, c: *mut i8)
    -> i32 {
    unsafe {
        if unsafe { (*g_multiplex.p_orig_vfs).x_get_last_error.is_some() } {
            return unsafe {
                    (unsafe {
                            (*g_multiplex.p_orig_vfs).x_get_last_error.unwrap()
                        })(g_multiplex.p_orig_vfs, b, c)
                };
        } else { return 0; }
    }
}
extern "C" fn multiplex_current_time_int64(a: *mut Sqlite3Vfs,
    b: *mut Sqlite3Int64) -> i32 {
    unsafe {
        return unsafe {
                (unsafe {
                        (*g_multiplex.p_orig_vfs).x_current_time_int64.unwrap()
                    })(g_multiplex.p_orig_vfs, b)
            };
    }
}
extern "C" fn multiplex_close(p_conn_1: *mut Sqlite3File) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let p_group: *mut MultiplexGroup = unsafe { (*p).p_group };
    let rc: i32 = 0;
    multiplex_free_components(p_group);
    unsafe { sqlite3_free(p_group as *mut ()) };
    return rc;
}
extern "C" fn multiplex_read(p_conn_1: *mut Sqlite3File, mut p_buf_1: *mut (),
    mut i_amt_1: i32, mut i_ofst_1: Sqlite3Int64) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let p_group: *mut MultiplexGroup = unsafe { (*p).p_group };
    let mut rc: i32 = 0;
    if (unsafe { (*p_group).b_enabled } == 0) as i32 != 0 {
        let p_sub_open: *mut Sqlite3File =
            multiplex_sub_open(p_group, 0, &mut rc, 0 as *mut () as *mut i32,
                0);
        if p_sub_open == core::ptr::null_mut() {
            rc = 10 | 1 << 8;
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_sub_open).p_methods }).x_read.unwrap()
                        })(p_sub_open, p_buf_1, i_amt_1, i_ofst_1)
                };
        }
    } else {
        while i_amt_1 > 0 {
            let i: i32 =
                (i_ofst_1 / unsafe { (*p_group).sz_chunk } as Sqlite3Int64) as
                    i32;
            let mut p_sub_open_1: *mut Sqlite3File = core::ptr::null_mut();
            p_sub_open_1 =
                multiplex_sub_open(p_group, i, &mut rc,
                    0 as *mut () as *mut i32, 1);
            if !(p_sub_open_1).is_null() {
                let mut extra: i32 =
                    (((i_ofst_1 %
                                            unsafe { (*p_group).sz_chunk } as Sqlite3Int64) as i32 +
                                    i_amt_1) as u32 - unsafe { (*p_group).sz_chunk }) as i32;
                if extra < 0 { extra = 0; }
                i_amt_1 -= extra;
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_sub_open_1).p_methods }).x_read.unwrap()
                            })(p_sub_open_1, p_buf_1, i_amt_1,
                            i_ofst_1 % unsafe { (*p_group).sz_chunk } as Sqlite3Int64)
                    };
                if rc != 0 { break; }
                p_buf_1 =
                    unsafe { (p_buf_1 as *mut i8).offset(i_amt_1 as isize) } as
                        *mut ();
                i_ofst_1 += i_amt_1 as Sqlite3Int64;
                i_amt_1 = extra;
            } else { rc = 10 | 1 << 8; break; }
        }
    }
    return rc;
}
extern "C" fn multiplex_write(p_conn_1: *mut Sqlite3File,
    mut p_buf_1: *const (), mut i_amt_1: i32, mut i_ofst_1: Sqlite3Int64)
    -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let p_group: *mut MultiplexGroup = unsafe { (*p).p_group };
    let mut rc: i32 = 0;
    if (unsafe { (*p_group).b_enabled } == 0) as i32 != 0 {
        let p_sub_open: *mut Sqlite3File =
            multiplex_sub_open(p_group, 0, &mut rc, 0 as *mut () as *mut i32,
                0);
        if p_sub_open == core::ptr::null_mut() {
            rc = 10 | 3 << 8;
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_sub_open).p_methods }).x_write.unwrap()
                        })(p_sub_open, p_buf_1, i_amt_1, i_ofst_1)
                };
        }
    } else {
        while rc == 0 && i_amt_1 > 0 {
            let i: i32 =
                (i_ofst_1 / unsafe { (*p_group).sz_chunk } as Sqlite3Int64) as
                    i32;
            let p_sub_open_1: *mut Sqlite3File =
                multiplex_sub_open(p_group, i, &mut rc,
                    0 as *mut () as *mut i32, 1);
            if !(p_sub_open_1).is_null() {
                let mut extra: i32 =
                    (((i_ofst_1 %
                                            unsafe { (*p_group).sz_chunk } as Sqlite3Int64) as i32 +
                                    i_amt_1) as u32 - unsafe { (*p_group).sz_chunk }) as i32;
                if extra < 0 { extra = 0; }
                i_amt_1 -= extra;
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_sub_open_1).p_methods }).x_write.unwrap()
                            })(p_sub_open_1, p_buf_1, i_amt_1,
                            i_ofst_1 % unsafe { (*p_group).sz_chunk } as Sqlite3Int64)
                    };
                p_buf_1 =
                    unsafe { (p_buf_1 as *mut i8).offset(i_amt_1 as isize) } as
                        *const ();
                i_ofst_1 += i_amt_1 as Sqlite3Int64;
                i_amt_1 = extra;
            }
        }
    }
    return rc;
}
extern "C" fn multiplex_truncate(p_conn_1: *mut Sqlite3File,
    size: Sqlite3Int64) -> i32 {
    unsafe {
        let p: *const MultiplexConn =
            p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
        let p_group: *mut MultiplexGroup = unsafe { (*p).p_group };
        let mut rc: i32 = 0;
        if (unsafe { (*p_group).b_enabled } == 0) as i32 != 0 {
            let p_sub_open: *mut Sqlite3File =
                multiplex_sub_open(p_group, 0, &mut rc,
                    0 as *mut () as *mut i32, 0);
            if p_sub_open == core::ptr::null_mut() {
                rc = 10 | 6 << 8;
            } else {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_sub_open).p_methods }).x_truncate.unwrap()
                            })(p_sub_open, size)
                    };
            }
        } else {
            let mut i: i32 = 0;
            let i_base_group: i32 =
                (size / unsafe { (*p_group).sz_chunk } as Sqlite3Int64) as
                    i32;
            let mut p_sub_open_1: *mut Sqlite3File = core::ptr::null_mut();
            let p_orig_vfs: *mut Sqlite3Vfs = g_multiplex.p_orig_vfs;
            {
                i = unsafe { (*p_group).n_real } - 1;
                '__b10: loop {
                    if !(i > i_base_group && rc == 0) { break '__b10; }
                    '__c10: loop {
                        if unsafe { (*p_group).b_truncate } != 0 {
                            multiplex_sub_close(unsafe { &*p_group }, i, p_orig_vfs);
                        } else {
                            p_sub_open_1 =
                                multiplex_sub_open(p_group, i, &mut rc,
                                    core::ptr::null_mut(), 0);
                            if !(p_sub_open_1).is_null() {
                                rc =
                                    unsafe {
                                        (unsafe {
                                                (*unsafe { (*p_sub_open_1).p_methods }).x_truncate.unwrap()
                                            })(p_sub_open_1, 0)
                                    };
                            }
                        }
                        break '__c10;
                    }
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                }
            }
            if rc == 0 {
                p_sub_open_1 =
                    multiplex_sub_open(p_group, i_base_group, &mut rc,
                        core::ptr::null_mut(), 0);
                if !(p_sub_open_1).is_null() {
                    rc =
                        unsafe {
                            (unsafe {
                                    (*unsafe { (*p_sub_open_1).p_methods }).x_truncate.unwrap()
                                })(p_sub_open_1,
                                size % unsafe { (*p_group).sz_chunk } as Sqlite3Int64)
                        };
                }
            }
            if rc != 0 { rc = 10 | 6 << 8; }
        }
        return rc;
    }
}
extern "C" fn multiplex_sync(p_conn_1: *mut Sqlite3File, flags: i32) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let p_group: *const MultiplexGroup =
        unsafe { (*p).p_group } as *const MultiplexGroup;
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b11: loop {
            if !(i < unsafe { (*p_group).n_real }) { break '__b11; }
            '__c11: loop {
                let p_sub_open: *mut Sqlite3File =
                    unsafe {
                        (*unsafe { (*p_group).a_real.offset(i as isize) }).p
                    };
                if !(p_sub_open).is_null() {
                    let rc2: i32 =
                        unsafe {
                            (unsafe {
                                    (*unsafe { (*p_sub_open).p_methods }).x_sync.unwrap()
                                })(p_sub_open, flags)
                        };
                    if rc2 != 0 { rc = rc2; }
                }
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return rc;
}
extern "C" fn multiplex_file_size(p_conn_1: *mut Sqlite3File,
    p_size_1: *mut Sqlite3Int64) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let p_group: *mut MultiplexGroup = unsafe { (*p).p_group };
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    if (unsafe { (*p_group).b_enabled } == 0) as i32 != 0 {
        let p_sub_open: *mut Sqlite3File =
            multiplex_sub_open(p_group, 0, &mut rc, 0 as *mut () as *mut i32,
                0);
        if p_sub_open == core::ptr::null_mut() {
            rc = 10 | 7 << 8;
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_sub_open).p_methods }).x_file_size.unwrap()
                        })(p_sub_open, p_size_1)
                };
        }
    } else {
        unsafe { *p_size_1 = 0 as Sqlite3Int64 };
        {
            i = 0;
            '__b12: loop {
                if !(rc == 0) { break '__b12; }
                '__c12: loop {
                    let sz: Sqlite3Int64 =
                        multiplex_sub_size(p_group, i, &mut rc);
                    if sz == 0 as i64 { break '__b12; }
                    unsafe {
                        *p_size_1 =
                            i as Sqlite3Int64 *
                                    unsafe { (*p_group).sz_chunk } as Sqlite3Int64 + sz
                    };
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return rc;
}
extern "C" fn multiplex_lock(p_conn_1: *mut Sqlite3File, lock: i32) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_lock.unwrap()
                    })(p_sub_open, lock)
            };
    }
    return 5;
}
extern "C" fn multiplex_unlock(p_conn_1: *mut Sqlite3File, lock: i32) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_unlock.unwrap()
                    })(p_sub_open, lock)
            };
    }
    return 10 | 8 << 8;
}
extern "C" fn multiplex_check_reserved_lock(p_conn_1: *mut Sqlite3File,
    p_res_out_1: *mut i32) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe {
                                            (*p_sub_open).p_methods
                                        }).x_check_reserved_lock.unwrap()
                    })(p_sub_open, p_res_out_1)
            };
    }
    return 10 | 14 << 8;
}
extern "C" fn multiplex_file_control(p_conn_1: *mut Sqlite3File, op: i32,
    p_arg_1: *mut ()) -> i32 {
    unsafe {
        let p: *const MultiplexConn =
            p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
        let p_group: *mut MultiplexGroup = unsafe { (*p).p_group };
        let mut rc: i32 = 1;
        let mut p_sub_open: *mut Sqlite3File = core::ptr::null_mut();
        if (g_multiplex.is_initialized == 0) as i32 != 0 { return 21; }
        '__s13:
            {
            match op {
                214014 => {
                    if !(p_arg_1).is_null() {
                        let b_enabled: i32 = unsafe { *(p_arg_1 as *mut i32) };
                        unsafe { (*p_group).b_enabled = b_enabled as u8 };
                        rc = 0;
                    }
                }
                214015 => {
                    if !(p_arg_1).is_null() {
                        let mut sz_chunk: u32 = unsafe { *(p_arg_1 as *mut u32) };
                        if sz_chunk < 1 as u32 {
                            rc = 21;
                        } else {
                            sz_chunk = sz_chunk + (65536 - 1) as u32;
                            sz_chunk &= !(65536 - 1) as u32;
                            unsafe { (*p_group).sz_chunk = sz_chunk };
                            rc = 0;
                        }
                    }
                }
                214016 => { rc = 0; }
                5 => { rc = 0; }
                6 => { rc = 0; }
                14 => {
                    {
                        let a_fcntl: *mut *mut i8 = p_arg_1 as *mut *mut i8;
                        if !(unsafe { *a_fcntl.offset(1 as isize) }).is_null() &&
                                unsafe {
                                        sqlite3_strnicmp(unsafe { *a_fcntl.offset(1 as isize) } as
                                                *const i8, c"multiplex_".as_ptr() as *mut i8 as *const i8,
                                            10)
                                    } == 0 {
                            let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
                            { let _ = multiplex_file_size(p_conn_1, &mut sz); };
                            if unsafe {
                                        sqlite3_stricmp(unsafe { *a_fcntl.offset(1 as isize) } as
                                                *const i8,
                                            c"multiplex_truncate".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                if !(unsafe { *a_fcntl.offset(2 as isize) }).is_null() &&
                                        unsafe {
                                                *unsafe { (*a_fcntl.offset(2 as isize)).offset(0 as isize) }
                                            } != 0 {
                                    if unsafe {
                                                    sqlite3_stricmp(unsafe { *a_fcntl.offset(2 as isize) } as
                                                            *const i8, c"on".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                            unsafe {
                                                    sqlite3_stricmp(unsafe { *a_fcntl.offset(2 as isize) } as
                                                            *const i8, c"1".as_ptr() as *mut i8 as *const i8)
                                                } == 0 {
                                        unsafe { (*p_group).b_truncate = 1 as u8 };
                                    } else if unsafe {
                                                    sqlite3_stricmp(unsafe { *a_fcntl.offset(2 as isize) } as
                                                            *const i8, c"off".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                            unsafe {
                                                    sqlite3_stricmp(unsafe { *a_fcntl.offset(2 as isize) } as
                                                            *const i8, c"0".as_ptr() as *mut i8 as *const i8)
                                                } == 0 {
                                        unsafe { (*p_group).b_truncate = 0 as u8 };
                                    }
                                }
                                unsafe {
                                    *a_fcntl.offset(0 as isize) =
                                        unsafe {
                                            sqlite3_mprintf(if unsafe { (*p_group).b_truncate } != 0 {
                                                        c"on".as_ptr() as *mut i8
                                                    } else { c"off".as_ptr() as *mut i8 } as *const i8)
                                        }
                                };
                                rc = 0;
                                break '__s13;
                            }
                            if unsafe {
                                        sqlite3_stricmp(unsafe { *a_fcntl.offset(1 as isize) } as
                                                *const i8,
                                            c"multiplex_enabled".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                unsafe {
                                    *a_fcntl.offset(0 as isize) =
                                        unsafe {
                                            sqlite3_mprintf(c"%d".as_ptr() as *mut i8 as *const i8,
                                                (unsafe { (*p_group).b_enabled } as i32 != 0) as i32)
                                        }
                                };
                                rc = 0;
                                break '__s13;
                            }
                            if unsafe {
                                            sqlite3_stricmp(unsafe { *a_fcntl.offset(1 as isize) } as
                                                    *const i8,
                                                c"multiplex_chunksize".as_ptr() as *mut i8 as *const i8)
                                        } == 0 && unsafe { (*p_group).b_enabled } != 0 {
                                unsafe {
                                    *a_fcntl.offset(0 as isize) =
                                        unsafe {
                                            sqlite3_mprintf(c"%u".as_ptr() as *mut i8 as *const i8,
                                                unsafe { (*p_group).sz_chunk })
                                        }
                                };
                                rc = 0;
                                break '__s13;
                            }
                            if unsafe {
                                        sqlite3_stricmp(unsafe { *a_fcntl.offset(1 as isize) } as
                                                *const i8,
                                            c"multiplex_filecount".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                let mut n: i32 = 0;
                                let mut ii: i32 = 0;
                                {
                                    ii = 0;
                                    '__b14: loop {
                                        if !(ii < unsafe { (*p_group).n_real }) { break '__b14; }
                                        '__c14: loop {
                                            if unsafe {
                                                        (*unsafe { (*p_group).a_real.offset(ii as isize) }).p
                                                    } != core::ptr::null_mut() {
                                                { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                            }
                                            break '__c14;
                                        }
                                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                unsafe {
                                    *a_fcntl.offset(0 as isize) =
                                        unsafe {
                                            sqlite3_mprintf(c"%d".as_ptr() as *mut i8 as *const i8, n)
                                        }
                                };
                                rc = 0;
                                break '__s13;
                            }
                        }
                    }
                    p_sub_open =
                        multiplex_sub_open(p_group, 0, &mut rc,
                            0 as *mut () as *mut i32, 0);
                    if !(p_sub_open).is_null() {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe {
                                                            (*p_sub_open).p_methods
                                                        }).x_file_control.unwrap()
                                    })(p_sub_open, op, p_arg_1)
                            };
                        if op == 12 && rc == 0 {
                            unsafe {
                                *(p_arg_1 as *mut *mut i8) =
                                    unsafe {
                                        sqlite3_mprintf(c"multiplex/%z".as_ptr() as *mut i8 as
                                                *const i8, unsafe { *(p_arg_1 as *mut *mut i8) })
                                    }
                            };
                        }
                    }
                }
                _ => {
                    p_sub_open =
                        multiplex_sub_open(p_group, 0, &mut rc,
                            0 as *mut () as *mut i32, 0);
                    if !(p_sub_open).is_null() {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe {
                                                            (*p_sub_open).p_methods
                                                        }).x_file_control.unwrap()
                                    })(p_sub_open, op, p_arg_1)
                            };
                        if op == 12 && rc == 0 {
                            unsafe {
                                *(p_arg_1 as *mut *mut i8) =
                                    unsafe {
                                        sqlite3_mprintf(c"multiplex/%z".as_ptr() as *mut i8 as
                                                *const i8, unsafe { *(p_arg_1 as *mut *mut i8) })
                                    }
                            };
                        }
                    }
                }
            }
        }
        return rc;
    }
}
extern "C" fn multiplex_sector_size(p_conn_1: *mut Sqlite3File) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() &&
            unsafe {
                (*unsafe { (*p_sub_open).p_methods }).x_sector_size.is_some()
            } {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_sector_size.unwrap()
                    })(p_sub_open)
            };
    }
    return 4096;
}
extern "C" fn multiplex_device_characteristics(p_conn_1: *mut Sqlite3File)
    -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe {
                                            (*p_sub_open).p_methods
                                        }).x_device_characteristics.unwrap()
                    })(p_sub_open)
            };
    }
    return 0;
}
extern "C" fn multiplex_shm_map(p_conn_1: *mut Sqlite3File, i_region_1: i32,
    sz_region_1: i32, b_extend_1: i32, pp: *mut *mut ()) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_shm_map.unwrap()
                    })(p_sub_open, i_region_1, sz_region_1, b_extend_1, pp)
            };
    }
    return 10;
}
extern "C" fn multiplex_shm_lock(p_conn_1: *mut Sqlite3File, ofst: i32,
    n: i32, flags: i32) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_shm_lock.unwrap()
                    })(p_sub_open, ofst, n, flags)
            };
    }
    return 5;
}
extern "C" fn multiplex_shm_barrier(p_conn_1: *mut Sqlite3File) -> () {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_shm_barrier.unwrap()
                })(p_sub_open)
        };
    }
}
extern "C" fn multiplex_shm_unmap(p_conn_1: *mut Sqlite3File,
    delete_flag_1: i32) -> i32 {
    let p: *const MultiplexConn =
        p_conn_1 as *mut MultiplexConn as *const MultiplexConn;
    let mut rc: i32 = 0;
    let p_sub_open: *mut Sqlite3File =
        multiplex_sub_open(unsafe { (*p).p_group }, 0, &mut rc,
            0 as *mut () as *mut i32, 0);
    if !(p_sub_open).is_null() {
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_shm_unmap.unwrap()
                    })(p_sub_open, delete_flag_1)
            };
    }
    return 0;
}
extern "C" fn multiplex_control_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut rc: i32 = 0;
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut op: i32 = 0;
    let mut i_val: i32 = 0;
    if (db).is_null() as i32 != 0 || argc != 2 {
        rc = 1;
    } else {
        op =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(0 as isize) }) };
        i_val =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(1 as isize) }) };
        '__s15:
            {
            match op {
                1 => { op = 214014; }
                2 => { op = 214015; }
                3 => { op = 214016; }
                _ => { rc = 12; }
            }
        }
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_file_control(db, core::ptr::null(), op,
                    &raw mut i_val as *mut ())
            };
    }
    unsafe { sqlite3_result_error_code(context, rc) };
}
extern "C" fn multiplex_func_init(db: *mut Sqlite3,
    pz_err_msg_1: *mut *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let mut rc: i32 = 0;
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"multiplex_control".as_ptr() as *mut i8 as *const i8, 2, 5,
                core::ptr::null_mut(), Some(multiplex_control_func), None,
                None)
        };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_multiplex_initialize(z_orig_vfs_name: *const i8,
    make_default: i32) -> i32 {
    unsafe {
        let mut p_orig_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
        if g_multiplex.is_initialized != 0 { return 21; }
        p_orig_vfs = unsafe { sqlite3_vfs_find(z_orig_vfs_name) };
        if p_orig_vfs == core::ptr::null_mut() { return 1; }
        if !(p_orig_vfs != &raw mut g_multiplex.s_this_vfs as *mut Sqlite3Vfs)
                        as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3_multiplex_initialize".as_ptr() as
                        *const i8,
                    c"test_multiplex.c".as_ptr() as *mut i8 as *const i8, 1155,
                    c"pOrigVfs!=&gMultiplex.sThisVfs".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        g_multiplex.is_initialized = 1;
        g_multiplex.p_orig_vfs = p_orig_vfs;
        g_multiplex.s_this_vfs = unsafe { core::ptr::read(p_orig_vfs) };
        g_multiplex.s_this_vfs.sz_os_file +=
            core::mem::size_of::<MultiplexConn>() as i32;
        g_multiplex.s_this_vfs.z_name =
            c"multiplex".as_ptr() as *mut i8 as *const i8;
        g_multiplex.s_this_vfs.x_open = Some(multiplex_open);
        g_multiplex.s_this_vfs.x_delete = Some(multiplex_delete);
        g_multiplex.s_this_vfs.x_access = Some(multiplex_access);
        g_multiplex.s_this_vfs.x_full_pathname =
            Some(multiplex_full_pathname);
        g_multiplex.s_this_vfs.x_dl_open = Some(multiplex_dl_open);
        g_multiplex.s_this_vfs.x_dl_error = Some(multiplex_dl_error);
        g_multiplex.s_this_vfs.x_dl_sym = Some(multiplex_dl_sym);
        g_multiplex.s_this_vfs.x_dl_close = Some(multiplex_dl_close);
        g_multiplex.s_this_vfs.x_randomness = Some(multiplex_randomness);
        g_multiplex.s_this_vfs.x_sleep = Some(multiplex_sleep);
        g_multiplex.s_this_vfs.x_current_time = Some(multiplex_current_time);
        g_multiplex.s_this_vfs.x_get_last_error =
            Some(multiplex_get_last_error);
        g_multiplex.s_this_vfs.x_current_time_int64 =
            Some(multiplex_current_time_int64);
        g_multiplex.s_io_methods_v1.i_version = 1;
        g_multiplex.s_io_methods_v1.x_close = Some(multiplex_close);
        g_multiplex.s_io_methods_v1.x_read = Some(multiplex_read);
        g_multiplex.s_io_methods_v1.x_write = Some(multiplex_write);
        g_multiplex.s_io_methods_v1.x_truncate = Some(multiplex_truncate);
        g_multiplex.s_io_methods_v1.x_sync = Some(multiplex_sync);
        g_multiplex.s_io_methods_v1.x_file_size = Some(multiplex_file_size);
        g_multiplex.s_io_methods_v1.x_lock = Some(multiplex_lock);
        g_multiplex.s_io_methods_v1.x_unlock = Some(multiplex_unlock);
        g_multiplex.s_io_methods_v1.x_check_reserved_lock =
            Some(multiplex_check_reserved_lock);
        g_multiplex.s_io_methods_v1.x_file_control =
            Some(multiplex_file_control);
        g_multiplex.s_io_methods_v1.x_sector_size =
            Some(multiplex_sector_size);
        g_multiplex.s_io_methods_v1.x_device_characteristics =
            Some(multiplex_device_characteristics);
        g_multiplex.s_io_methods_v2 = g_multiplex.s_io_methods_v1;
        g_multiplex.s_io_methods_v2.i_version = 2;
        g_multiplex.s_io_methods_v2.x_shm_map = Some(multiplex_shm_map);
        g_multiplex.s_io_methods_v2.x_shm_lock = Some(multiplex_shm_lock);
        g_multiplex.s_io_methods_v2.x_shm_barrier =
            Some(multiplex_shm_barrier);
        g_multiplex.s_io_methods_v2.x_shm_unmap = Some(multiplex_shm_unmap);
        unsafe {
            sqlite3_vfs_register(&mut g_multiplex.s_this_vfs, make_default)
        };
        unsafe {
            sqlite3_auto_extension(Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn()
                                    -> ()>(multiplex_func_init as *const ())
                    }))
        };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_multiplex_shutdown(e_force: i32) -> i32 {
    unsafe {
        let rc: i32 = 0;
        if g_multiplex.is_initialized == 0 { return 21; }
        g_multiplex.is_initialized = 0;
        unsafe { sqlite3_vfs_unregister(&mut g_multiplex.s_this_vfs) };
        unsafe { memset(&raw mut g_multiplex as *mut (), 0, 488) };
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
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    static mut sqlite3_pending_byte: i32;
}