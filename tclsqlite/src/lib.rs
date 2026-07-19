#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module,
    Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, SqliteInt64, SqliteUint64,
};

type TclWideInt = i64;

type TclChannel = *mut Tcl_Channel_;

type ClientData = *mut ();

type TclChannelTypeVersion = *mut Tcl_ChannelTypeVersion_;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct TclChannelType {
    typeName: *mut i8,
    version: *mut Tcl_ChannelTypeVersion_,
    closeProc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp) -> i32>,
    inputProc: Option<unsafe extern "C" fn(*mut (), *mut i8, i32, *mut i32)
        -> i32>,
    outputProc: Option<unsafe extern "C" fn(*mut (), *const i8, i32, *mut i32)
        -> i32>,
    seekProc: Option<unsafe extern "C" fn(*mut (), i64, i32, *mut i32)
        -> i32>,
    setOptionProc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp,
        *const i8, *const i8) -> i32>,
    getOptionProc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp,
        *const i8, *mut TclDString) -> i32>,
    watchProc: Option<unsafe extern "C" fn(*mut (), i32) -> ()>,
    getHandleProc: Option<unsafe extern "C" fn(*mut (), i32, *mut *mut ())
        -> i32>,
    close2Proc: Option<unsafe extern "C" fn(*mut (), *mut TclInterp, i32)
        -> i32>,
    blockModeProc: Option<unsafe extern "C" fn(*mut (), i32) -> i32>,
    flushProc: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    handlerProc: Option<unsafe extern "C" fn(*mut (), i32) -> i32>,
    wideSeekProc: Option<unsafe extern "C" fn(*mut (), i64, i32, *mut i32)
        -> i64>,
    threadActionProc: Option<unsafe extern "C" fn(*mut (), i32) -> ()>,
    truncateProc: Option<unsafe extern "C" fn(*mut (), i64) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TclDString {
    string: *mut i8,
    length: i32,
    spaceAvl: i32,
    staticSpace: [i8; 200],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TclObj {
    refCount: i32,
    bytes: *mut i8,
    length: i32,
    typePtr: *mut TclObjType,
    internalRep: TclObjU0,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TclObjType {
    name: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
union TclObjU0 {
    longValue: i64,
    doubleValue: f64,
    otherValuePtr: *mut (),
    wideValue: i64,
    twoPtrValue: TclObjU0S0,
    ptrAndLongRep: TclObjU0S1,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TclObjU0S0 {
    ptr1: *mut (),
    ptr2: *mut (),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TclObjU0S1 {
    ptr: *mut (),
    value: u64,
}

type TclSize = i32;

type Uptr = u64;

///* There is one instance of this structure for each SQLite database
///* that has been opened by the SQLite TCL interface.
///*
///* If this module is built with SQLITE_TEST defined (to create the SQLite
///* testfixture executable), then it may be configured to use either
///* sqlite3_prepare_v2() or sqlite3_prepare() to prepare SQL statements.
///* If SqliteDb.bLegacyPrepare is true, sqlite3_prepare() is used.
#[repr(C)]
#[derive(Copy, Clone)]
struct SqliteDb {
    db: *mut Sqlite3,
    interp: *mut TclInterp,
    z_busy: *mut i8,
    z_commit: *mut i8,
    z_trace: *mut i8,
    z_trace_v2: *mut i8,
    z_profile: *mut i8,
    z_progress: *mut i8,
    z_bind_fallback: *mut i8,
    z_auth: *mut i8,
    disable_auth: i32,
    z_null: *mut i8,
    p_func: *mut SqlFunc,
    p_update_hook: *mut TclObj,
    p_pre_update_hook: *mut TclObj,
    p_rollback_hook: *mut TclObj,
    p_wal_hook: *mut TclObj,
    p_unlock_notify: *mut TclObj,
    p_collate: *mut SqlCollate,
    rc: i32,
    p_collate_needed: *mut TclObj,
    stmt_list: *mut SqlPreparedStmt,
    stmt_last: *mut SqlPreparedStmt,
    max_stmt: i32,
    n_stmt: i32,
    p_incrblob: *mut IncrblobChannel,
    n_step: i32,
    n_sort: i32,
    n_index: i32,
    n_vm_step: i32,
    n_transaction: i32,
    open_flags: i32,
    n_ref: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SqlFunc {
    interp: *mut TclInterp,
    p_script: *mut TclObj,
    p_db: *mut SqliteDb,
    use_eval_objv: i32,
    e_type: i32,
    z_name: *mut i8,
    p_next: *mut SqlFunc,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SqlCollate {
    interp: *mut TclInterp,
    z_script: *mut i8,
    p_next: *mut SqlCollate,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SqlPreparedStmt {
    p_next: *mut SqlPreparedStmt,
    p_prev: *mut SqlPreparedStmt,
    p_stmt: *mut Sqlite3Stmt,
    n_sql: i32,
    z_sql: *const i8,
    n_parm: i32,
    ap_parm: *mut *mut TclObj,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IncrblobChannel {
    p_blob: *mut Sqlite3Blob,
    p_db: *mut SqliteDb,
    i_seek: Sqlite3Int64,
    is_closed: u32,
    channel: TclChannel,
    p_next: *mut IncrblobChannel,
    p_prev: *mut IncrblobChannel,
}

///* Compute a string length that is limited to what can be stored in
///* lower 30 bits of a 32-bit signed integer.
extern "C" fn strlen30(z: *const i8) -> i32 {
    let mut z2: *const i8 = z;
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

///* Close all incrblob channels opened using database connection pDb.
///* This is called when shutting down the database connection.
#[allow(unused_doc_comments)]
extern "C" fn close_incrblob_channels(p_db_1: &SqliteDb) -> () {
    let mut p: *const IncrblobChannel = core::ptr::null();
    let mut p_next: *mut IncrblobChannel = core::ptr::null_mut();
    {
        p = (*p_db_1).p_incrblob;
        '__b1: loop {
            if !(!(p).is_null()) { break '__b1; }
            '__c1: loop {
                p_next = unsafe { (*p).p_next };

                /// Note: Calling unregister here call Tcl_Close on the incrblob channel,
                ///* which deletes the IncrblobChannel structure at *p. So do not
                ///* call Tcl_Free() here.
                unsafe {
                    Tcl_UnregisterChannel((*p_db_1).interp,
                        unsafe { (*p).channel })
                };
                break '__c1;
            }
            p = p_next;
        }
    }
}

///* Close an incremental blob channel.
#[allow(unused_doc_comments)]
extern "C" fn incrblob_close2(instance_data_1: ClientData,
    interp: *mut TclInterp, flags: i32) -> i32 {
    let p: *mut IncrblobChannel = instance_data_1 as *mut IncrblobChannel;
    let mut rc: i32 = 0;
    let db: *mut Sqlite3 = unsafe { (*unsafe { (*p).p_db }).db };
    if flags != 0 { unsafe { (*p).is_closed |= flags as u32 }; return 0; }

    /// If we reach this point, then we really do need to close the channel
    (rc = unsafe { sqlite3_blob_close(unsafe { (*p).p_blob }) });
    if !(unsafe { (*p).p_next }).is_null() {
        unsafe { (*unsafe { (*p).p_next }).p_prev = unsafe { (*p).p_prev } };
    }
    if !(unsafe { (*p).p_prev }).is_null() {
        unsafe { (*unsafe { (*p).p_prev }).p_next = unsafe { (*p).p_next } };
    }
    if unsafe { (*unsafe { (*p).p_db }).p_incrblob } == p {
        unsafe {
            (*unsafe { (*p).p_db }).p_incrblob = unsafe { (*p).p_next }
        };
    }

    /// Free the IncrblobChannel structure
    unsafe { Tcl_Free(p as *mut i8) };
    if rc != 0 {
        unsafe {
            Tcl_SetResult(interp, unsafe { sqlite3_errmsg(db) } as *mut i8,
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
                })
        };
        return 1;
    }
    return 0;
}

extern "C" fn incrblob_close(instance_data_1: ClientData,
    interp: *mut TclInterp) -> i32 {
    return incrblob_close2(instance_data_1, interp, 0);
}

///* Read data from an incremental blob channel.
#[allow(unused_doc_comments)]
extern "C" fn incrblob_input(instance_data_1: ClientData, buf: *mut i8,
    buf_size_1: i32, error_code_ptr_1: *mut i32) -> i32 {
    let p: *mut IncrblobChannel = instance_data_1 as *mut IncrblobChannel;
    let mut n_read: Sqlite3Int64 = buf_size_1 as Sqlite3Int64;
    /// Number of bytes to read
    let mut n_blob: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Total size of the blob
    let mut rc: i32 = 0;

    /// sqlite error code
    (n_blob =
        unsafe { sqlite3_blob_bytes(unsafe { (*p).p_blob }) } as
            Sqlite3Int64);
    if unsafe { (*p).i_seek } + n_read > n_blob {
        n_read = n_blob - unsafe { (*p).i_seek };
    }
    if n_read <= 0 as i64 { return 0; }
    rc =
        unsafe {
            sqlite3_blob_read(unsafe { (*p).p_blob }, buf as *mut (),
                n_read as i32, unsafe { (*p).i_seek } as i32)
        };
    if rc != 0 { unsafe { *error_code_ptr_1 = rc }; return -1; }
    unsafe { (*p).i_seek += n_read };
    return n_read as i32;
}

///* Write data to an incremental blob channel.
#[allow(unused_doc_comments)]
extern "C" fn incrblob_output(instance_data_1: ClientData, buf: *const i8,
    to_write_1: i32, error_code_ptr_1: *mut i32) -> i32 {
    let p: *mut IncrblobChannel = instance_data_1 as *mut IncrblobChannel;
    let n_write: Sqlite3Int64 = to_write_1 as Sqlite3Int64;
    /// Number of bytes to write
    let mut n_blob: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Total size of the blob
    let mut rc: i32 = 0;

    /// sqlite error code
    (n_blob =
        unsafe { sqlite3_blob_bytes(unsafe { (*p).p_blob }) } as
            Sqlite3Int64);
    if unsafe { (*p).i_seek } + n_write > n_blob {
        unsafe { *error_code_ptr_1 = 22 };
        return -1;
    }
    if n_write <= 0 as i64 { return 0; }
    rc =
        unsafe {
            sqlite3_blob_write(unsafe { (*p).p_blob },
                buf as *mut () as *const (), n_write as i32,
                unsafe { (*p).i_seek } as i32)
        };
    if rc != 0 { unsafe { *error_code_ptr_1 = 5 }; return -1; }
    unsafe { (*p).i_seek += n_write };
    return n_write as i32;
}

///* Seek an incremental blob channel.
extern "C" fn incrblob_wide_seek(instance_data_1: ClientData,
    offset: TclWideInt, seek_mode_1: i32, error_code_ptr_1: *mut i32)
    -> TclWideInt {
    let p: *mut IncrblobChannel = instance_data_1 as *mut IncrblobChannel;
    '__s2:
        {
        match seek_mode_1 {
            0 => { unsafe { (*p).i_seek = offset as Sqlite3Int64 }; }
            1 => { unsafe { (*p).i_seek += offset as Sqlite3Int64 }; }
            2 => {
                unsafe {
                    (*p).i_seek =
                        (unsafe { sqlite3_blob_bytes(unsafe { (*p).p_blob }) } as
                                    TclWideInt + offset) as Sqlite3Int64
                };
            }
            _ => {
                if (0 == 0) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"incrblobWideSeek".as_ptr() as *const i8,
                            c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 416,
                            c"!\"Bad seekMode\"".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
            }
        }
    }
    return unsafe { (*p).i_seek } as TclWideInt;
}

extern "C" fn incrblob_seek(instance_data_1: ClientData, offset: i64,
    seek_mode_1: i32, error_code_ptr_1: *mut i32) -> i32 {
    return incrblob_wide_seek(instance_data_1, offset, seek_mode_1,
                error_code_ptr_1) as i32;
}

extern "C" fn incrblob_watch(instance_data_1: ClientData, mode: i32) -> () {}

extern "C" fn incrblob_handle(instance_data_1: ClientData, dir: i32,
    h_ptr_1: *mut ClientData) -> i32 {
    return 1;
}

static mut incrblob_channel_type: TclChannelType =
    TclChannelType {
        typeName: c"incrblob".as_ptr() as *mut i8,
        version: 5 as TclChannelTypeVersion,
        closeProc: Some(incrblob_close),
        inputProc: Some(incrblob_input),
        outputProc: Some(incrblob_output),
        seekProc: Some(incrblob_seek),
        setOptionProc: None,
        getOptionProc: None,
        watchProc: Some(incrblob_watch),
        getHandleProc: Some(incrblob_handle),
        close2Proc: Some(incrblob_close2),
        blockModeProc: None,
        flushProc: None,
        handlerProc: None,
        wideSeekProc: Some(incrblob_wide_seek),
        threadActionProc: None,
        truncateProc: None,
    };

///* Create a new incrblob channel.
#[allow(unused_doc_comments)]
extern "C" fn create_incrblob_channel(interp: *mut TclInterp,
    p_db_1: *mut SqliteDb, z_db_1: *const i8, z_table_1: *const i8,
    z_column_1: *const i8, i_row_1: SqliteInt64, is_readonly_1: i32) -> i32 {
    unsafe {
        let mut p: *mut IncrblobChannel = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_db_1).db };
        let mut p_blob: *mut Sqlite3Blob = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let flags: i32 = 1 << 1 | if is_readonly_1 != 0 { 0 } else { 1 << 2 };
        let mut z_channel: [i8; 64] = [0; 64];
        rc =
            unsafe {
                sqlite3_blob_open(db, z_db_1, z_table_1, z_column_1, i_row_1,
                    (is_readonly_1 == 0) as i32 as i32, &mut p_blob)
            };
        if rc != 0 {
            unsafe {
                Tcl_SetResult(interp,
                    unsafe { sqlite3_errmsg(unsafe { (*p_db_1).db }) } as
                        *mut i8,
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
                    })
            };
            return 1;
        }
        p =
            unsafe {
                    Tcl_Alloc(core::mem::size_of::<IncrblobChannel>() as u32)
                } as *mut IncrblobChannel;
        unsafe {
            memset(p as *mut (), 0,
                core::mem::size_of::<IncrblobChannel>() as u64)
        };
        unsafe { (*p).p_blob = p_blob };
        if flags & 1 << 2 == 0 {
            unsafe { (*p).is_closed |= (1 << 2) as u32 };
        }
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 64]>() as i32,
                &raw mut z_channel[0 as usize] as *mut i8,
                c"incrblob_%d".as_ptr() as *mut i8 as *const i8,
                { let __p = &mut count; *__p += 1; *__p })
        };
        unsafe {
            (*p).channel =
                unsafe {
                    Tcl_CreateChannel(&mut incrblob_channel_type,
                        &raw mut z_channel[0 as usize] as *mut i8 as *const i8,
                        p as ClientData, flags)
                }
        };
        unsafe { Tcl_RegisterChannel(interp, unsafe { (*p).channel }) };

        /// Link the new channel into the SqliteDb.pIncrblob list.
        unsafe { (*p).p_next = unsafe { (*p_db_1).p_incrblob } };
        unsafe { (*p).p_prev = core::ptr::null_mut() };
        if !(unsafe { (*p).p_next }).is_null() {
            unsafe { (*unsafe { (*p).p_next }).p_prev = p };
        }
        unsafe { (*p_db_1).p_incrblob = p };
        unsafe { (*p).p_db = p_db_1 };
        unsafe {
            Tcl_SetResult(interp,
                unsafe { Tcl_GetChannelName(unsafe { (*p).channel }) } as
                    *mut i8,
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
                })
        };
        return 0;
    }
}

///* Look at the script prefix in pCmd.  We will be executing this script
///* after first appending one or more arguments.  This routine analyzes
///* the script to see if it is safe to use Tcl_EvalObjv() on the script
///* rather than the more general Tcl_EvalEx().  Tcl_EvalObjv() is much
///* faster.
///*
///* Scripts that are safe to use with Tcl_EvalObjv() consists of a
///* command name followed by zero or more arguments with no [...] or $
///* or {...} or ; to be seen anywhere.  Most callback scripts consist
///* of just a single procedure name and they meet this requirement.
#[allow(unused_doc_comments)]
extern "C" fn safe_to_use_eval_objv(p_cmd_1: *mut TclObj) -> i32 {
    /// We could try to do something with Tcl_Parse().  But we will instead
    ///* just do a search for forbidden characters.  If any of the forbidden
    ///* characters appear in pCmd, we will report the string as unsafe.
    let mut z: *const i8 = core::ptr::null();
    let mut n: TclSize = 0;
    z = unsafe { Tcl_GetStringFromObj(p_cmd_1, &mut n) } as *const i8;
    while { let __p = &mut n; let __t = *__p; *__p -= 1; __t } > 0 {
        let c: i32 =
            unsafe {
                    *{
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                } as i32;
        if c == '$' as i32 || c == '[' as i32 || c == ';' as i32 { return 0; }
    }
    return 1;
}

///* Find an SqlFunc structure with the given name.  Or create a new
///* one if an existing one cannot be found.  Return a pointer to the
///* structure.
extern "C" fn find_sql_func(p_db_1: *mut SqliteDb, z_name_1: *const i8)
    -> *mut SqlFunc {
    let mut p: *mut SqlFunc = core::ptr::null_mut();
    let mut p_new: *mut SqlFunc = core::ptr::null_mut();
    let n_name: i32 = strlen30(z_name_1);
    p_new =
        unsafe {
                Tcl_Alloc((core::mem::size_of::<SqlFunc>() as u64 +
                                n_name as u64 + 1 as u64) as u32)
            } as *mut SqlFunc;
    unsafe {
        (*p_new).z_name =
            unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
    };
    unsafe {
        memcpy(unsafe { (*p_new).z_name } as *mut (), z_name_1 as *const (),
            (n_name + 1) as u64)
    };
    {
        p = unsafe { (*p_db_1).p_func };
        '__b4: loop {
            if !(!(p).is_null()) { break '__b4; }
            '__c4: loop {
                if unsafe {
                            sqlite3_stricmp(unsafe { (*p).z_name } as *const i8,
                                unsafe { (*p_new).z_name } as *const i8)
                        } == 0 {
                    unsafe { Tcl_Free(p_new as *mut i8) };
                    return p;
                }
                break '__c4;
            }
            p = unsafe { (*p).p_next };
        }
    }
    unsafe { (*p_new).interp = unsafe { (*p_db_1).interp } };
    unsafe { (*p_new).p_db = p_db_1 };
    unsafe { (*p_new).p_script = core::ptr::null_mut() };
    unsafe { (*p_new).p_next = unsafe { (*p_db_1).p_func } };
    unsafe { (*p_db_1).p_func = p_new };
    return p_new;
}

///* Free a single SqlPreparedStmt object.
extern "C" fn db_free_stmt(p_stmt_1: *mut SqlPreparedStmt) -> () {
    unsafe { sqlite3_finalize(unsafe { (*p_stmt_1).p_stmt }) };
    unsafe { Tcl_Free(p_stmt_1 as *mut i8) };
}

///* Finalize and free a list of prepared statements
extern "C" fn flush_stmt_cache(p_db_1: &mut SqliteDb) -> () {
    let mut p_pre_stmt: *mut SqlPreparedStmt = core::ptr::null_mut();
    let mut p_next: *mut SqlPreparedStmt = core::ptr::null_mut();
    {
        p_pre_stmt = (*p_db_1).stmt_list;
        '__b5: loop {
            if !(!(p_pre_stmt).is_null()) { break '__b5; }
            '__c5: loop {
                p_next = unsafe { (*p_pre_stmt).p_next };
                db_free_stmt(p_pre_stmt);
                break '__c5;
            }
            p_pre_stmt = p_next;
        }
    }
    (*p_db_1).n_stmt = 0;
    (*p_db_1).stmt_last = core::ptr::null_mut();
    (*p_db_1).stmt_list = core::ptr::null_mut();
}

///* Increment the reference counter on the SqliteDb object. The reference
///* should be released by calling delDatabaseRef().
extern "C" fn add_database_ref(p_db_1: &mut SqliteDb) -> () {
    { let __p = &mut (*p_db_1).n_ref; let __t = *__p; *__p += 1; __t };
}

///* Decrement the reference counter associated with the SqliteDb object.
///* If it reaches zero, delete the object.
extern "C" fn del_database_ref(p_db_1: *mut SqliteDb) -> () {
    if !(unsafe { (*p_db_1).n_ref } > 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"delDatabaseRef".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 610,
                c"pDb->nRef>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        let __p = unsafe { &mut (*p_db_1).n_ref };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    if unsafe { (*p_db_1).n_ref } == 0 {
        flush_stmt_cache(unsafe { &mut *p_db_1 });
        close_incrblob_channels(unsafe { &*p_db_1 });
        unsafe { sqlite3_close(unsafe { (*p_db_1).db }) };
        while !(unsafe { (*p_db_1).p_func }).is_null() {
            let p_func: *mut SqlFunc = unsafe { (*p_db_1).p_func };
            unsafe { (*p_db_1).p_func = unsafe { (*p_func).p_next } };
            if !(unsafe { (*p_func).p_db } == p_db_1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"delDatabaseRef".as_ptr() as *const i8,
                        c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 619,
                        c"pFunc->pDb==pDb".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            '__b7: loop {
                '__c7: loop {
                    if {
                                let __p =
                                    unsafe { &mut (*unsafe { (*p_func).p_script }).refCount };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(unsafe { (*p_func).p_script }) };
                    }
                    break '__c7;
                }
                if !(0 != 0) { break '__b7; }
            }
            unsafe { Tcl_Free(p_func as *mut i8) };
        }
        while !(unsafe { (*p_db_1).p_collate }).is_null() {
            let p_collate: *mut SqlCollate = unsafe { (*p_db_1).p_collate };
            unsafe { (*p_db_1).p_collate = unsafe { (*p_collate).p_next } };
            unsafe { Tcl_Free(p_collate as *mut i8) };
        }
        if !(unsafe { (*p_db_1).z_busy }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_busy }) };
        }
        if !(unsafe { (*p_db_1).z_trace }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_trace }) };
        }
        if !(unsafe { (*p_db_1).z_trace_v2 }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_trace_v2 }) };
        }
        if !(unsafe { (*p_db_1).z_profile }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_profile }) };
        }
        if !(unsafe { (*p_db_1).z_bind_fallback }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_bind_fallback }) };
        }
        if !(unsafe { (*p_db_1).z_auth }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_auth }) };
        }
        if !(unsafe { (*p_db_1).z_null }).is_null() {
            unsafe { Tcl_Free(unsafe { (*p_db_1).z_null }) };
        }
        if !(unsafe { (*p_db_1).p_update_hook }).is_null() {
            '__b9: loop {
                '__c9: loop {
                    if {
                                let __p =
                                    unsafe {
                                        &mut (*unsafe { (*p_db_1).p_update_hook }).refCount
                                    };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(unsafe { (*p_db_1).p_update_hook }) };
                    }
                    break '__c9;
                }
                if !(0 != 0) { break '__b9; }
            }
        }
        if !(unsafe { (*p_db_1).p_pre_update_hook }).is_null() {
            '__b10: loop {
                '__c10: loop {
                    if {
                                let __p =
                                    unsafe {
                                        &mut (*unsafe { (*p_db_1).p_pre_update_hook }).refCount
                                    };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe {
                            TclFreeObj(unsafe { (*p_db_1).p_pre_update_hook })
                        };
                    }
                    break '__c10;
                }
                if !(0 != 0) { break '__b10; }
            }
        }
        if !(unsafe { (*p_db_1).p_rollback_hook }).is_null() {
            '__b11: loop {
                '__c11: loop {
                    if {
                                let __p =
                                    unsafe {
                                        &mut (*unsafe { (*p_db_1).p_rollback_hook }).refCount
                                    };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(unsafe { (*p_db_1).p_rollback_hook }) };
                    }
                    break '__c11;
                }
                if !(0 != 0) { break '__b11; }
            }
        }
        if !(unsafe { (*p_db_1).p_wal_hook }).is_null() {
            '__b12: loop {
                '__c12: loop {
                    if {
                                let __p =
                                    unsafe { &mut (*unsafe { (*p_db_1).p_wal_hook }).refCount };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(unsafe { (*p_db_1).p_wal_hook }) };
                    }
                    break '__c12;
                }
                if !(0 != 0) { break '__b12; }
            }
        }
        if !(unsafe { (*p_db_1).p_collate_needed }).is_null() {
            '__b13: loop {
                '__c13: loop {
                    if {
                                let __p =
                                    unsafe {
                                        &mut (*unsafe { (*p_db_1).p_collate_needed }).refCount
                                    };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe {
                            TclFreeObj(unsafe { (*p_db_1).p_collate_needed })
                        };
                    }
                    break '__c13;
                }
                if !(0 != 0) { break '__b13; }
            }
        }
        unsafe { Tcl_Free(p_db_1 as *mut i8) };
    }
}

///* TCL calls this procedure when an sqlite3 database command is
///* deleted.
extern "C" fn db_delete_cmd(db: *mut ()) -> () {
    let p_db: *mut SqliteDb = db as *mut SqliteDb;
    del_database_ref(p_db);
}

///* This routine is called when a database file is locked while trying
///* to execute SQL.
extern "C" fn db_busy_handler(cd: *mut (), n_tries_1: i32) -> i32 {
    let p_db: *const SqliteDb = cd as *mut SqliteDb as *const SqliteDb;
    let mut rc: i32 = 0;
    let mut z_val: [i8; 30] = [0; 30];
    unsafe {
        sqlite3_snprintf(core::mem::size_of::<[i8; 30]>() as i32,
            &raw mut z_val[0 as usize] as *mut i8,
            c"%d".as_ptr() as *mut i8 as *const i8, n_tries_1)
    };
    rc =
        unsafe {
            Tcl_VarEval(unsafe { (*p_db).interp }, unsafe { (*p_db).z_busy },
                c" ".as_ptr() as *mut i8,
                &raw mut z_val[0 as usize] as *mut i8, 0 as *mut i8)
        };
    if rc != 0 ||
            unsafe {
                    atoi(unsafe {
                            Tcl_GetStringResult(unsafe { (*p_db).interp })
                        })
                } != 0 {
        return 0;
    }
    return 1;
}

///* This routine is invoked as the 'progress callback' for the database.
extern "C" fn db_progress_handler(cd: *mut ()) -> i32 {
    let p_db: *const SqliteDb = cd as *mut SqliteDb as *const SqliteDb;
    let mut rc: i32 = 0;
    if (unsafe { (*p_db).z_progress }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"DbProgressHandler".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 702,
                c"pDb->zProgress".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    rc =
        unsafe {
            Tcl_Eval(unsafe { (*p_db).interp },
                unsafe { (*p_db).z_progress } as *const i8)
        };
    if rc != 0 ||
            unsafe {
                    atoi(unsafe {
                            Tcl_GetStringResult(unsafe { (*p_db).interp })
                        })
                } != 0 {
        return 1;
    }
    return 0;
}

///* This routine is called by the SQLite trace handler whenever a new
///* block of SQL is executed.  The TCL script in pDb->zTrace is executed.
extern "C" fn db_trace_handler(cd: *mut (), z_sql_1: *const i8) -> () {
    let p_db: *const SqliteDb = cd as *mut SqliteDb as *const SqliteDb;
    let mut str: TclDString = unsafe { core::mem::zeroed() };
    unsafe { Tcl_DStringInit(&mut str) };
    unsafe {
        Tcl_DStringAppend(&mut str, unsafe { (*p_db).z_trace } as *const i8,
            -1)
    };
    unsafe { Tcl_DStringAppendElement(&mut str, z_sql_1) };
    unsafe {
        Tcl_Eval(unsafe { (*p_db).interp },
            unsafe { (*&mut str).string } as *const i8)
    };
    unsafe { Tcl_DStringFree(&mut str) };
    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
}

///* This routine is called by the SQLite trace_v2 handler whenever a new
///* supported event is generated.  Unsupported event types are ignored.
///* The TCL script in pDb->zTraceV2 is executed, with the arguments for
///* the event appended to it (as list elements).
extern "C" fn db_trace_v2_handler(type__1: u32, cd: *mut (), pd: *mut (),
    xd: *mut ()) -> i32 {
    let p_db: *const SqliteDb = cd as *mut SqliteDb as *const SqliteDb;
    let mut p_cmd: *mut TclObj = core::ptr::null_mut();
    '__s14:
        {
        match type__1 {
            1 => {
                {
                    let p_stmt: *const Sqlite3Stmt =
                        pd as *mut Sqlite3Stmt as *const Sqlite3Stmt;
                    let z_sql: *const i8 = xd as *mut i8 as *const i8;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(p_stmt as Uptr as TclWideInt) })
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewStringObj(z_sql as *const i8, -1) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b15: loop {
                        '__c15: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c15;
                        }
                        if !(0 != 0) { break '__b15; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
                {
                    let p_stmt_1: *const Sqlite3Stmt =
                        pd as *mut Sqlite3Stmt as *const Sqlite3Stmt;
                    let ns: Sqlite3Int64 =
                        unsafe { core::ptr::read(xd as *mut Sqlite3Int64) };
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe {
                                Tcl_NewWideIntObj(p_stmt_1 as Uptr as TclWideInt)
                            })
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(ns as TclWideInt) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b16: loop {
                        '__c16: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c16;
                        }
                        if !(0 != 0) { break '__b16; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
                {
                    let p_stmt_2: *const Sqlite3Stmt =
                        pd as *mut Sqlite3Stmt as *const Sqlite3Stmt;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe {
                                Tcl_NewWideIntObj(p_stmt_2 as Uptr as TclWideInt)
                            })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b17: loop {
                        '__c17: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c17;
                        }
                        if !(0 != 0) { break '__b17; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
                {
                    let db: *const Sqlite3 =
                        pd as *mut Sqlite3 as *const Sqlite3;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(db as Uptr as TclWideInt) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b18: loop {
                        '__c18: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c18;
                        }
                        if !(0 != 0) { break '__b18; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
            }
            2 => {
                {
                    let p_stmt_1: *const Sqlite3Stmt =
                        pd as *mut Sqlite3Stmt as *const Sqlite3Stmt;
                    let ns: Sqlite3Int64 =
                        unsafe { core::ptr::read(xd as *mut Sqlite3Int64) };
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe {
                                Tcl_NewWideIntObj(p_stmt_1 as Uptr as TclWideInt)
                            })
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(ns as TclWideInt) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b16: loop {
                        '__c16: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c16;
                        }
                        if !(0 != 0) { break '__b16; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
                {
                    let p_stmt_2: *const Sqlite3Stmt =
                        pd as *mut Sqlite3Stmt as *const Sqlite3Stmt;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe {
                                Tcl_NewWideIntObj(p_stmt_2 as Uptr as TclWideInt)
                            })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b17: loop {
                        '__c17: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c17;
                        }
                        if !(0 != 0) { break '__b17; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
                {
                    let db: *const Sqlite3 =
                        pd as *mut Sqlite3 as *const Sqlite3;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(db as Uptr as TclWideInt) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b18: loop {
                        '__c18: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c18;
                        }
                        if !(0 != 0) { break '__b18; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
            }
            4 => {
                {
                    let p_stmt_2: *const Sqlite3Stmt =
                        pd as *mut Sqlite3Stmt as *const Sqlite3Stmt;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe {
                                Tcl_NewWideIntObj(p_stmt_2 as Uptr as TclWideInt)
                            })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b17: loop {
                        '__c17: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c17;
                        }
                        if !(0 != 0) { break '__b17; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
                {
                    let db: *const Sqlite3 =
                        pd as *mut Sqlite3 as *const Sqlite3;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(db as Uptr as TclWideInt) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b18: loop {
                        '__c18: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c18;
                        }
                        if !(0 != 0) { break '__b18; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
            }
            8 => {
                {
                    let db: *const Sqlite3 =
                        pd as *mut Sqlite3 as *const Sqlite3;
                    p_cmd =
                        unsafe {
                            Tcl_NewStringObj(unsafe { (*p_db).z_trace_v2 } as *const i8,
                                -1)
                        };
                    {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p += 1;
                        *__p
                    };
                    unsafe {
                        Tcl_ListObjAppendElement(unsafe { (*p_db).interp }, p_cmd,
                            unsafe { Tcl_NewWideIntObj(db as Uptr as TclWideInt) })
                    };
                    unsafe {
                        Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144)
                    };
                    '__b18: loop {
                        '__c18: loop {
                            if {
                                        let __p = unsafe { &mut (*p_cmd).refCount };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe { TclFreeObj(p_cmd) };
                            }
                            break '__c18;
                        }
                        if !(0 != 0) { break '__b18; }
                    }
                    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
                    break '__s14;
                }
            }
            _ => {}
        }
    }
    return 0;
}

///* This routine is called by the SQLite profile handler after a statement
///* SQL has executed.  The TCL script in pDb->zProfile is evaluated.
extern "C" fn db_profile_handler(cd: *mut (), z_sql_1: *const i8,
    tm: SqliteUint64) -> () {
    let p_db: *const SqliteDb = cd as *mut SqliteDb as *const SqliteDb;
    let mut str: TclDString = unsafe { core::mem::zeroed() };
    let mut z_tm: [i8; 100] = [0; 100];
    unsafe {
        sqlite3_snprintf((core::mem::size_of::<[i8; 100]>() as u64 - 1 as u64)
                as i32, &raw mut z_tm[0 as usize] as *mut i8,
            c"%lld".as_ptr() as *mut i8 as *const i8, tm)
    };
    unsafe { Tcl_DStringInit(&mut str) };
    unsafe {
        Tcl_DStringAppend(&mut str, unsafe { (*p_db).z_profile } as *const i8,
            -1)
    };
    unsafe { Tcl_DStringAppendElement(&mut str, z_sql_1) };
    unsafe {
        Tcl_DStringAppendElement(&mut str,
            &raw mut z_tm[0 as usize] as *mut i8 as *const i8)
    };
    unsafe {
        Tcl_Eval(unsafe { (*p_db).interp },
            unsafe { (*&mut str).string } as *const i8)
    };
    unsafe { Tcl_DStringFree(&mut str) };
    unsafe { Tcl_ResetResult(unsafe { (*p_db).interp }) };
}

///* This routine is called when a transaction is committed.  The
///* TCL script in pDb->zCommit is executed.  If it returns non-zero or
///* if it throws an exception, the transaction is rolled back instead
///* of being committed.
extern "C" fn db_commit_handler(cd: *mut ()) -> i32 {
    let p_db: *const SqliteDb = cd as *mut SqliteDb as *const SqliteDb;
    let mut rc: i32 = 0;
    rc =
        unsafe {
            Tcl_Eval(unsafe { (*p_db).interp },
                unsafe { (*p_db).z_commit } as *const i8)
        };
    if rc != 0 ||
            unsafe {
                    atoi(unsafe {
                            Tcl_GetStringResult(unsafe { (*p_db).interp })
                        })
                } != 0 {
        return 1;
    }
    return 0;
}

extern "C" fn db_rollback_handler(client_data_1: *mut ()) -> () {
    let p_db: *const SqliteDb =
        client_data_1 as *mut SqliteDb as *const SqliteDb;
    if (unsafe { (*p_db).p_rollback_hook }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"DbRollbackHandler".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 847,
                c"pDb->pRollbackHook".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if 0 !=
            unsafe {
                Tcl_EvalObjEx(unsafe { (*p_db).interp },
                    unsafe { (*p_db).p_rollback_hook }, 0)
            } {
        unsafe { Tcl_BackgroundError(unsafe { (*p_db).interp }) };
    }
}

///* This procedure handles wal_hook callbacks.
extern "C" fn db_wal_handler(client_data_1: *mut (), db: *mut Sqlite3,
    z_db_1: *const i8, n_entry_1: i32) -> i32 {
    let mut ret: i32 = 0;
    let mut p: *mut TclObj = core::ptr::null_mut();
    let p_db: *const SqliteDb =
        client_data_1 as *mut SqliteDb as *const SqliteDb;
    let interp: *mut TclInterp = unsafe { (*p_db).interp };
    if (unsafe { (*p_db).p_wal_hook }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"DbWalHandler".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 866,
                c"pDb->pWalHook".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(db == unsafe { (*p_db).db }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"DbWalHandler".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 868,
                c"db==pDb->db".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p = unsafe { Tcl_DuplicateObj(unsafe { (*p_db).p_wal_hook }) };
    { let __p = unsafe { &mut (*p).refCount }; *__p += 1; *__p };
    unsafe {
        Tcl_ListObjAppendElement(interp, p,
            unsafe { Tcl_NewStringObj(z_db_1, -1) })
    };
    unsafe {
        Tcl_ListObjAppendElement(interp, p,
            unsafe { Tcl_NewIntObj(n_entry_1) })
    };
    if 0 != unsafe { Tcl_EvalObjEx(interp, p, 0) } ||
            0 !=
                unsafe {
                    Tcl_GetIntFromObj(interp,
                        unsafe { Tcl_GetObjResult(interp) }, &mut ret)
                } {
        unsafe { Tcl_BackgroundError(interp) };
    }
    '__b19: loop {
        '__c19: loop {
            if { let __p = unsafe { &mut (*p).refCount }; *__p -= 1; *__p } <=
                    0 {
                unsafe { TclFreeObj(p) };
            }
            break '__c19;
        }
        if !(0 != 0) { break '__b19; }
    }
    return ret;
}

extern "C" fn db_update_handler(p: *mut (), op: i32, z_db_1: *const i8,
    z_tbl_1: *const i8, rowid: SqliteInt64) -> () {
    unsafe {
        let p_db: *const SqliteDb = p as *mut SqliteDb as *const SqliteDb;
        let mut p_cmd: *mut TclObj = core::ptr::null_mut();
        if !((9 - 1) / 9 == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"DbUpdateHandler".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 957,
                    c"(SQLITE_DELETE-1)/9 == 0".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !((18 - 1) / 9 == 1) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"DbUpdateHandler".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 958,
                    c"(SQLITE_INSERT-1)/9 == 1".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !((23 - 1) / 9 == 2) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"DbUpdateHandler".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 959,
                    c"(SQLITE_UPDATE-1)/9 == 2".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if (unsafe { (*p_db).p_update_hook }).is_null() as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"DbUpdateHandler".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 961,
                    c"pDb->pUpdateHook".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(op == 18 || op == 23 || op == 9) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"DbUpdateHandler".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 962,
                    c"op==SQLITE_INSERT || op==SQLITE_UPDATE || op==SQLITE_DELETE".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        p_cmd = unsafe { Tcl_DuplicateObj(unsafe { (*p_db).p_update_hook }) };
        { let __p = unsafe { &mut (*p_cmd).refCount }; *__p += 1; *__p };
        unsafe {
            Tcl_ListObjAppendElement(core::ptr::null_mut(), p_cmd,
                unsafe {
                    Tcl_NewStringObj(az_str[((op - 1) / 9) as usize], -1)
                })
        };
        unsafe {
            Tcl_ListObjAppendElement(core::ptr::null_mut(), p_cmd,
                unsafe { Tcl_NewStringObj(z_db_1, -1) })
        };
        unsafe {
            Tcl_ListObjAppendElement(core::ptr::null_mut(), p_cmd,
                unsafe { Tcl_NewStringObj(z_tbl_1, -1) })
        };
        unsafe {
            Tcl_ListObjAppendElement(core::ptr::null_mut(), p_cmd,
                unsafe { Tcl_NewWideIntObj(rowid) })
        };
        unsafe { Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_cmd, 262144) };
        '__b20: loop {
            '__c20: loop {
                if {
                            let __p = unsafe { &mut (*p_cmd).refCount };
                            *__p -= 1;
                            *__p
                        } <= 0 {
                    unsafe { TclFreeObj(p_cmd) };
                }
                break '__c20;
            }
            if !(0 != 0) { break '__b20; }
        }
    }
}

extern "C" fn tcl_collate_needed(p_ctx_1: *mut (), db: *mut Sqlite3, enc: i32,
    z_name_1: *const i8) -> () {
    let p_db: *const SqliteDb = p_ctx_1 as *mut SqliteDb as *const SqliteDb;
    let p_script: *mut TclObj =
        unsafe { Tcl_DuplicateObj(unsafe { (*p_db).p_collate_needed }) };
    { let __p = unsafe { &mut (*p_script).refCount }; *__p += 1; *__p };
    unsafe {
        Tcl_ListObjAppendElement(core::ptr::null_mut(), p_script,
            unsafe { Tcl_NewStringObj(z_name_1, -1) })
    };
    unsafe { Tcl_EvalObjEx(unsafe { (*p_db).interp }, p_script, 0) };
    '__b21: loop {
        '__c21: loop {
            if {
                        let __p = unsafe { &mut (*p_script).refCount };
                        *__p -= 1;
                        *__p
                    } <= 0 {
                unsafe { TclFreeObj(p_script) };
            }
            break '__c21;
        }
        if !(0 != 0) { break '__b21; }
    }
}

///* This routine is called to evaluate an SQL collation function implemented
///* using TCL script.
extern "C" fn tcl_sql_collate(p_ctx_1: *mut (), n_a_1: i32, z_a_1: *const (),
    n_b_1: i32, z_b_1: *const ()) -> i32 {
    let p: *const SqlCollate =
        p_ctx_1 as *mut SqlCollate as *const SqlCollate;
    let mut p_cmd: *mut TclObj = core::ptr::null_mut();
    p_cmd =
        unsafe {
            Tcl_NewStringObj(unsafe { (*p).z_script } as *const i8, -1)
        };
    { let __p = unsafe { &mut (*p_cmd).refCount }; *__p += 1; *__p };
    unsafe {
        Tcl_ListObjAppendElement(unsafe { (*p).interp }, p_cmd,
            unsafe { Tcl_NewStringObj(z_a_1 as *const i8, n_a_1) })
    };
    unsafe {
        Tcl_ListObjAppendElement(unsafe { (*p).interp }, p_cmd,
            unsafe { Tcl_NewStringObj(z_b_1 as *const i8, n_b_1) })
    };
    unsafe { Tcl_EvalObjEx(unsafe { (*p).interp }, p_cmd, 262144) };
    '__b22: loop {
        '__c22: loop {
            if {
                        let __p = unsafe { &mut (*p_cmd).refCount };
                        *__p -= 1;
                        *__p
                    } <= 0 {
                unsafe { TclFreeObj(p_cmd) };
            }
            break '__c22;
        }
        if !(0 != 0) { break '__b22; }
    }
    return unsafe {
            atoi(unsafe { Tcl_GetStringResult(unsafe { (*p).interp }) })
        };
}

///* This routine is called to evaluate an SQL function implemented
///* using TCL script.
#[allow(unused_doc_comments)]
extern "C" fn tcl_sql_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let p: *const SqlFunc =
        unsafe { sqlite3_user_data(context) } as *const SqlFunc;
    let mut p_cmd: *mut TclObj = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    if argc == 0 {

        /// If there are no arguments to the function, call Tcl_EvalObjEx on the
        ///* script object directly.  This allows the TCL compiler to generate
        ///* bytecode for the command on the first invocation and thus make
        ///* subsequent invocations much faster.
        (p_cmd = unsafe { (*p).p_script });
        { let __p = unsafe { &mut (*p_cmd).refCount }; *__p += 1; *__p };
        rc = unsafe { Tcl_EvalObjEx(unsafe { (*p).interp }, p_cmd, 0) };
        '__b23: loop {
            '__c23: loop {
                if {
                            let __p = unsafe { &mut (*p_cmd).refCount };
                            *__p -= 1;
                            *__p
                        } <= 0 {
                    unsafe { TclFreeObj(p_cmd) };
                }
                break '__c23;
            }
            if !(0 != 0) { break '__b23; }
        }
    } else {
        /// If there are arguments to the function, make a shallow copy of the
        ///* script object, lappend the arguments, then evaluate the copy.
        ///*
        ///* By "shallow" copy, we mean only the outer list Tcl_Obj is duplicated.
        ///* The new Tcl_Obj contains pointers to the original list elements.
        ///* That way, when Tcl_EvalObjv() is run and shimmers the first element
        ///* of the list to tclCmdNameType, that alternate representation will
        ///* be preserved and reused on the next invocation.
        let mut a_arg: *mut *mut TclObj = core::ptr::null_mut();
        let mut n_arg: TclSize = 0;
        if unsafe {
                    Tcl_ListObjGetElements(unsafe { (*p).interp },
                        unsafe { (*p).p_script }, &mut n_arg, &mut a_arg)
                } != 0 {
            unsafe {
                sqlite3_result_error(context,
                    unsafe { Tcl_GetStringResult(unsafe { (*p).interp }) }, -1)
            };
            return;
        }
        p_cmd = unsafe { Tcl_NewListObj(n_arg, a_arg as *const *mut TclObj) };
        { let __p = unsafe { &mut (*p_cmd).refCount }; *__p += 1; *__p };
        {
            i = 0;
            '__b24: loop {
                if !(i < argc) { break '__b24; }
                '__c24: loop {
                    let p_in: *mut Sqlite3Value =
                        unsafe { *argv.offset(i as isize) };
                    let mut p_val: *mut TclObj = core::ptr::null_mut();
                    '__s25:
                        {
                        match unsafe { sqlite3_value_type(p_in) } {
                            4 => {
                                {
                                    let bytes: i32 = unsafe { sqlite3_value_bytes(p_in) };
                                    p_val =
                                        unsafe {
                                            Tcl_NewByteArrayObj(unsafe { sqlite3_value_blob(p_in) } as
                                                    *const u8, bytes)
                                        };
                                    break '__s25;
                                }
                                {
                                    let mut v: SqliteInt64 =
                                        unsafe { sqlite3_value_int64(p_in) };
                                    if v >= -2147483647 as i64 && v <= 2147483647 as i64 {
                                        p_val = unsafe { Tcl_NewIntObj(v as i32) };
                                    } else { p_val = unsafe { Tcl_NewWideIntObj(v) }; }
                                    break '__s25;
                                }
                                {
                                    let mut r: f64 = unsafe { sqlite3_value_double(p_in) };
                                    p_val = unsafe { Tcl_NewDoubleObj(r) };
                                    break '__s25;
                                }
                                {
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { (*unsafe { (*p).p_db }).z_null }
                                                    as *const i8, -1)
                                        };
                                    break '__s25;
                                }
                                {
                                    let bytes: i32 = unsafe { sqlite3_value_bytes(p_in) };
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { sqlite3_value_text(p_in) } as
                                                        *mut i8 as *const i8, bytes)
                                        };
                                    break '__s25;
                                }
                            }
                            1 => {
                                {
                                    let mut v: SqliteInt64 =
                                        unsafe { sqlite3_value_int64(p_in) };
                                    if v >= -2147483647 as i64 && v <= 2147483647 as i64 {
                                        p_val = unsafe { Tcl_NewIntObj(v as i32) };
                                    } else { p_val = unsafe { Tcl_NewWideIntObj(v) }; }
                                    break '__s25;
                                }
                                {
                                    let mut r: f64 = unsafe { sqlite3_value_double(p_in) };
                                    p_val = unsafe { Tcl_NewDoubleObj(r) };
                                    break '__s25;
                                }
                                {
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { (*unsafe { (*p).p_db }).z_null }
                                                    as *const i8, -1)
                                        };
                                    break '__s25;
                                }
                                {
                                    let bytes: i32 = unsafe { sqlite3_value_bytes(p_in) };
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { sqlite3_value_text(p_in) } as
                                                        *mut i8 as *const i8, bytes)
                                        };
                                    break '__s25;
                                }
                            }
                            2 => {
                                {
                                    let mut r: f64 = unsafe { sqlite3_value_double(p_in) };
                                    p_val = unsafe { Tcl_NewDoubleObj(r) };
                                    break '__s25;
                                }
                                {
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { (*unsafe { (*p).p_db }).z_null }
                                                    as *const i8, -1)
                                        };
                                    break '__s25;
                                }
                                {
                                    let bytes: i32 = unsafe { sqlite3_value_bytes(p_in) };
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { sqlite3_value_text(p_in) } as
                                                        *mut i8 as *const i8, bytes)
                                        };
                                    break '__s25;
                                }
                            }
                            5 => {
                                {
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { (*unsafe { (*p).p_db }).z_null }
                                                    as *const i8, -1)
                                        };
                                    break '__s25;
                                }
                                {
                                    let bytes: i32 = unsafe { sqlite3_value_bytes(p_in) };
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { sqlite3_value_text(p_in) } as
                                                        *mut i8 as *const i8, bytes)
                                        };
                                    break '__s25;
                                }
                            }
                            _ => {
                                {
                                    let bytes: i32 = unsafe { sqlite3_value_bytes(p_in) };
                                    p_val =
                                        unsafe {
                                            Tcl_NewStringObj(unsafe { sqlite3_value_text(p_in) } as
                                                        *mut i8 as *const i8, bytes)
                                        };
                                    break '__s25;
                                }
                            }
                        }
                    }
                    rc =
                        unsafe {
                            Tcl_ListObjAppendElement(unsafe { (*p).interp }, p_cmd,
                                p_val)
                        };
                    if rc != 0 {
                        '__b26: loop {
                            '__c26: loop {
                                if {
                                            let __p = unsafe { &mut (*p_cmd).refCount };
                                            *__p -= 1;
                                            *__p
                                        } <= 0 {
                                    unsafe { TclFreeObj(p_cmd) };
                                }
                                break '__c26;
                            }
                            if !(0 != 0) { break '__b26; }
                        }
                        unsafe {
                            sqlite3_result_error(context,
                                unsafe { Tcl_GetStringResult(unsafe { (*p).interp }) }, -1)
                        };
                        return;
                    }
                    break '__c24;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if (unsafe { (*p).use_eval_objv } == 0) as i32 != 0 {

            /// Tcl_EvalObjEx() will automatically call Tcl_EvalObjv() if pCmd
            ///* is a list without a string representation.  To prevent this from
            ///* happening, make sure pCmd has a valid string representation
            unsafe { Tcl_GetString(p_cmd) };
        }
        rc = unsafe { Tcl_EvalObjEx(unsafe { (*p).interp }, p_cmd, 262144) };
        '__b27: loop {
            '__c27: loop {
                if {
                            let __p = unsafe { &mut (*p_cmd).refCount };
                            *__p -= 1;
                            *__p
                        } <= 0 {
                    unsafe { TclFreeObj(p_cmd) };
                }
                break '__c27;
            }
            if !(0 != 0) { break '__b27; }
        }
    }
    if 3 == rc {
        unsafe { sqlite3_result_null(context) };
    } else if rc != 0 && rc != 2 {
        unsafe {
            sqlite3_result_error(context,
                unsafe { Tcl_GetStringResult(unsafe { (*p).interp }) }, -1)
        };
    } else {
        let p_var: *mut TclObj =
            unsafe { Tcl_GetObjResult(unsafe { (*p).interp }) };
        let mut n: TclSize = 0;
        let mut data: *mut u8 = core::ptr::null_mut();
        let z_type: *const i8 =
            if !(unsafe { (*p_var).typePtr }).is_null() {
                    unsafe { (*unsafe { (*p_var).typePtr }).name }
                } else { c"".as_ptr() as *mut i8 } as *const i8;
        let c: i8 = unsafe { *z_type.offset(0 as isize) } as i8;
        let mut e_type: i32 = unsafe { (*p).e_type };
        if e_type == 5 {
            if c as i32 == 'b' as i32 &&
                        unsafe {
                                strcmp(z_type,
                                    c"bytearray".as_ptr() as *mut i8 as *const i8)
                            } == 0 && unsafe { (*p_var).bytes } == core::ptr::null_mut()
                {

                /// Only return a BLOB type if the Tcl variable is a bytearray and
                ///* has no string representation.
                (e_type = 4);
            } else if c as i32 == 'b' as i32 &&
                                    unsafe { (*p_var).bytes } == core::ptr::null_mut() &&
                                unsafe {
                                        strcmp(z_type, c"boolean".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                            c as i32 == 'b' as i32 &&
                                    unsafe { (*p_var).bytes } == core::ptr::null_mut() &&
                                unsafe {
                                        strcmp(z_type,
                                            c"booleanString".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                        c as i32 == 'w' as i32 &&
                            unsafe {
                                    strcmp(z_type, c"wideInt".as_ptr() as *mut i8 as *const i8)
                                } == 0 ||
                    c as i32 == 'i' as i32 &&
                        unsafe {
                                strcmp(z_type, c"int".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                e_type = 1;
            } else if c as i32 == 'd' as i32 &&
                    unsafe {
                            strcmp(z_type, c"double".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                e_type = 2;
            } else { e_type = 3; }
        }
        '__s28:
            {
            match e_type {
                4 => {
                    {
                        data = unsafe { Tcl_GetByteArrayFromObj(p_var, &mut n) };
                        unsafe {
                            sqlite3_result_blob(context, data as *const (), n,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s28;
                    }
                    {
                        let mut v: TclWideInt = 0 as TclWideInt;
                        if 0 ==
                                unsafe {
                                    Tcl_GetWideIntFromObj(core::ptr::null_mut(), p_var, &mut v)
                                } {
                            unsafe { sqlite3_result_int64(context, v) };
                            break '__s28;
                        }
                    }
                    {
                        let mut r: f64 = 0.0;
                        if 0 ==
                                unsafe {
                                    Tcl_GetDoubleFromObj(core::ptr::null_mut(), p_var, &mut r)
                                } {
                            unsafe { sqlite3_result_double(context, r) };
                            break '__s28;
                        }
                    }
                    {
                        data =
                            unsafe { Tcl_GetStringFromObj(p_var, &mut n) } as *mut u8;
                        unsafe {
                            sqlite3_result_text64(context, data as *mut i8 as *const i8,
                                n as Sqlite3Uint64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), 1 as u8)
                        };
                        break '__s28;
                    }
                }
                1 => {
                    {
                        let mut v: TclWideInt = 0 as TclWideInt;
                        if 0 ==
                                unsafe {
                                    Tcl_GetWideIntFromObj(core::ptr::null_mut(), p_var, &mut v)
                                } {
                            unsafe { sqlite3_result_int64(context, v) };
                            break '__s28;
                        }
                    }
                    {
                        let mut r: f64 = 0.0;
                        if 0 ==
                                unsafe {
                                    Tcl_GetDoubleFromObj(core::ptr::null_mut(), p_var, &mut r)
                                } {
                            unsafe { sqlite3_result_double(context, r) };
                            break '__s28;
                        }
                    }
                    {
                        data =
                            unsafe { Tcl_GetStringFromObj(p_var, &mut n) } as *mut u8;
                        unsafe {
                            sqlite3_result_text64(context, data as *mut i8 as *const i8,
                                n as Sqlite3Uint64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), 1 as u8)
                        };
                        break '__s28;
                    }
                }
                2 => {
                    {
                        let mut r: f64 = 0.0;
                        if 0 ==
                                unsafe {
                                    Tcl_GetDoubleFromObj(core::ptr::null_mut(), p_var, &mut r)
                                } {
                            unsafe { sqlite3_result_double(context, r) };
                            break '__s28;
                        }
                    }
                    {
                        data =
                            unsafe { Tcl_GetStringFromObj(p_var, &mut n) } as *mut u8;
                        unsafe {
                            sqlite3_result_text64(context, data as *mut i8 as *const i8,
                                n as Sqlite3Uint64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), 1 as u8)
                        };
                        break '__s28;
                    }
                }
                _ => {
                    {
                        data =
                            unsafe { Tcl_GetStringFromObj(p_var, &mut n) } as *mut u8;
                        unsafe {
                            sqlite3_result_text64(context, data as *mut i8 as *const i8,
                                n as Sqlite3Uint64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), 1 as u8)
                        };
                        break '__s28;
                    }
                }
            }
        }
    }
}

///* This is the authentication function.  It appends the authentication
///* type code and the two arguments to zCmd[] then invokes the result
///* on the interpreter.  The reply is examined to determine if the
///* authentication fails or succeeds.
#[allow(unused_doc_comments)]
extern "C" fn auth_callback(p_arg_1: *mut (), code: i32, z_arg1_1: *const i8,
    z_arg2_1: *const i8, z_arg3_1: *const i8, z_arg4_1: *const i8) -> i32 {
    let mut z_code: *const i8 = core::ptr::null();
    let mut str: TclDString = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut z_reply: *const i8 = core::ptr::null();
    /// EVIDENCE-OF: R-38590-62769 The first parameter to the authorizer
    ///* callback is a copy of the third parameter to the
    ///* sqlite3_set_authorizer() interface.
    let p_db: *const SqliteDb = p_arg_1 as *mut SqliteDb as *const SqliteDb;
    if unsafe { (*p_db).disable_auth } != 0 { return 0; }
    '__s29:
        {
        match code {
            0 => { z_code = c"SQLITE_COPY".as_ptr() as *mut i8 as *const i8; }
            1 => {
                z_code =
                    c"SQLITE_CREATE_INDEX".as_ptr() as *mut i8 as *const i8;
            }
            2 => {
                z_code =
                    c"SQLITE_CREATE_TABLE".as_ptr() as *mut i8 as *const i8;
            }
            3 => {
                z_code =
                    c"SQLITE_CREATE_TEMP_INDEX".as_ptr() as *mut i8 as
                        *const i8;
            }
            4 => {
                z_code =
                    c"SQLITE_CREATE_TEMP_TABLE".as_ptr() as *mut i8 as
                        *const i8;
            }
            5 => {
                z_code =
                    c"SQLITE_CREATE_TEMP_TRIGGER".as_ptr() as *mut i8 as
                        *const i8;
            }
            6 => {
                z_code =
                    c"SQLITE_CREATE_TEMP_VIEW".as_ptr() as *mut i8 as *const i8;
            }
            7 => {
                z_code =
                    c"SQLITE_CREATE_TRIGGER".as_ptr() as *mut i8 as *const i8;
            }
            8 => {
                z_code =
                    c"SQLITE_CREATE_VIEW".as_ptr() as *mut i8 as *const i8;
            }
            9 => {
                z_code = c"SQLITE_DELETE".as_ptr() as *mut i8 as *const i8;
            }
            10 => {
                z_code =
                    c"SQLITE_DROP_INDEX".as_ptr() as *mut i8 as *const i8;
            }
            11 => {
                z_code =
                    c"SQLITE_DROP_TABLE".as_ptr() as *mut i8 as *const i8;
            }
            12 => {
                z_code =
                    c"SQLITE_DROP_TEMP_INDEX".as_ptr() as *mut i8 as *const i8;
            }
            13 => {
                z_code =
                    c"SQLITE_DROP_TEMP_TABLE".as_ptr() as *mut i8 as *const i8;
            }
            14 => {
                z_code =
                    c"SQLITE_DROP_TEMP_TRIGGER".as_ptr() as *mut i8 as
                        *const i8;
            }
            15 => {
                z_code =
                    c"SQLITE_DROP_TEMP_VIEW".as_ptr() as *mut i8 as *const i8;
            }
            16 => {
                z_code =
                    c"SQLITE_DROP_TRIGGER".as_ptr() as *mut i8 as *const i8;
            }
            17 => {
                z_code = c"SQLITE_DROP_VIEW".as_ptr() as *mut i8 as *const i8;
            }
            18 => {
                z_code = c"SQLITE_INSERT".as_ptr() as *mut i8 as *const i8;
            }
            19 => {
                z_code = c"SQLITE_PRAGMA".as_ptr() as *mut i8 as *const i8;
            }
            20 => {
                z_code = c"SQLITE_READ".as_ptr() as *mut i8 as *const i8;
            }
            21 => {
                z_code = c"SQLITE_SELECT".as_ptr() as *mut i8 as *const i8;
            }
            22 => {
                z_code =
                    c"SQLITE_TRANSACTION".as_ptr() as *mut i8 as *const i8;
            }
            23 => {
                z_code = c"SQLITE_UPDATE".as_ptr() as *mut i8 as *const i8;
            }
            24 => {
                z_code = c"SQLITE_ATTACH".as_ptr() as *mut i8 as *const i8;
            }
            25 => {
                z_code = c"SQLITE_DETACH".as_ptr() as *mut i8 as *const i8;
            }
            26 => {
                z_code =
                    c"SQLITE_ALTER_TABLE".as_ptr() as *mut i8 as *const i8;
            }
            27 => {
                z_code = c"SQLITE_REINDEX".as_ptr() as *mut i8 as *const i8;
            }
            28 => {
                z_code = c"SQLITE_ANALYZE".as_ptr() as *mut i8 as *const i8;
            }
            29 => {
                z_code =
                    c"SQLITE_CREATE_VTABLE".as_ptr() as *mut i8 as *const i8;
            }
            30 => {
                z_code =
                    c"SQLITE_DROP_VTABLE".as_ptr() as *mut i8 as *const i8;
            }
            31 => {
                z_code = c"SQLITE_FUNCTION".as_ptr() as *mut i8 as *const i8;
            }
            32 => {
                z_code = c"SQLITE_SAVEPOINT".as_ptr() as *mut i8 as *const i8;
            }
            33 => {
                z_code = c"SQLITE_RECURSIVE".as_ptr() as *mut i8 as *const i8;
            }
            _ => { z_code = c"????".as_ptr() as *mut i8 as *const i8; }
        }
    }
    unsafe { Tcl_DStringInit(&mut str) };
    unsafe {
        Tcl_DStringAppend(&mut str, unsafe { (*p_db).z_auth } as *const i8,
            -1)
    };
    unsafe { Tcl_DStringAppendElement(&mut str, z_code) };
    unsafe {
        Tcl_DStringAppendElement(&mut str,
            if !(z_arg1_1).is_null() {
                z_arg1_1
            } else { c"".as_ptr() as *mut i8 as *const i8 })
    };
    unsafe {
        Tcl_DStringAppendElement(&mut str,
            if !(z_arg2_1).is_null() {
                z_arg2_1
            } else { c"".as_ptr() as *mut i8 as *const i8 })
    };
    unsafe {
        Tcl_DStringAppendElement(&mut str,
            if !(z_arg3_1).is_null() {
                z_arg3_1
            } else { c"".as_ptr() as *mut i8 as *const i8 })
    };
    unsafe {
        Tcl_DStringAppendElement(&mut str,
            if !(z_arg4_1).is_null() {
                z_arg4_1
            } else { c"".as_ptr() as *mut i8 as *const i8 })
    };
    rc =
        unsafe {
            Tcl_GlobalEval(unsafe { (*p_db).interp },
                unsafe { (*&mut str).string } as *const i8)
        };
    unsafe { Tcl_DStringFree(&mut str) };
    z_reply =
        if rc == 0 {
            unsafe { Tcl_GetStringResult(unsafe { (*p_db).interp }) }
        } else { c"SQLITE_DENY".as_ptr() as *mut i8 as *const i8 };
    if unsafe {
                strcmp(z_reply, c"SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        rc = 0;
    } else if unsafe {
                strcmp(z_reply,
                    c"SQLITE_DENY".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        rc = 1;
    } else if unsafe {
                strcmp(z_reply,
                    c"SQLITE_IGNORE".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        rc = 2;
    } else { rc = 999; }
    return rc;
}

///* This function is part of the implementation of the command:
///*
///*   $db transaction [-deferred|-immediate|-exclusive] SCRIPT
///*
///* It is invoked after evaluating the script SCRIPT to commit or rollback
///* the transaction or savepoint opened by the [transaction] command.
#[allow(unused_doc_comments)]
extern "C" fn db_trans_post_cmd(data: *mut ClientData, interp: *mut TclInterp,
    result: i32) -> i32 {
    unsafe {
        /// rc==TCL_ERROR, nTransaction!=0
        /// rc!=TCL_ERROR, nTransaction==0
        /// rc==TCL_ERROR, nTransaction==0
        let p_db: *mut SqliteDb =
            unsafe { *data.offset(0 as isize) } as *mut SqliteDb;
        let mut rc: i32 = result;
        let mut z_end: *const i8 = core::ptr::null();
        {
            let __p = unsafe { &mut (*p_db).n_transaction };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        z_end =
            az_end[((rc == 1) as i32 * 2 +
                        (unsafe { (*p_db).n_transaction } == 0) as i32) as usize];
        {
            let __p = unsafe { &mut (*p_db).disable_auth };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if unsafe {
                    sqlite3_exec(unsafe { (*p_db).db }, z_end, None,
                        core::ptr::null_mut(), core::ptr::null_mut())
                } != 0 {
            if rc != 1 {
                unsafe {
                    Tcl_AppendResult(interp,
                        unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                        0 as *mut i8)
                };
                rc = 1;
            }
            unsafe {
                sqlite3_exec(unsafe { (*p_db).db },
                    c"ROLLBACK".as_ptr() as *mut i8 as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
        {
            let __p = unsafe { &mut (*p_db).disable_auth };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        del_database_ref(p_db);
        return rc;
    }
}

///* Unless SQLITE_TEST is defined, this function is a simple wrapper around
///* sqlite3_prepare_v2(). If SQLITE_TEST is defined, then it uses either
///* sqlite3_prepare_v2() or legacy interface sqlite3_prepare(), depending
///* on whether or not the [db_use_legacy_prepare] command has been used to
///* configure the connection.
extern "C" fn db_prepare(p_db_1: &SqliteDb, z_sql_1: *const i8,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_out_1: *mut *const i8) -> i32 {
    let mut prep_flags: u32 = 0 as u32;
    if (*p_db_1).max_stmt > 5 { prep_flags = 1 as u32; }
    return unsafe {
            sqlite3_prepare_v3((*p_db_1).db, z_sql_1, -1, prep_flags,
                pp_stmt_1, pz_out_1)
        };
}

///* Search the cache for a prepared-statement object that implements the
///* first SQL statement in the buffer pointed to by parameter zIn. If
///* no such prepared-statement can be found, allocate and prepare a new
///* one. In either case, bind the current values of the relevant Tcl
///* variables to any $var, :var or @var variables in the statement. Before
///* returning, set *ppPreStmt to point to the prepared-statement object.
///*
///* Output parameter *pzOut is set to point to the next SQL statement in
///* buffer zIn, or to the '\0' byte at the end of zIn if there is no
///* next statement.
///*
///* If successful, TCL_OK is returned. Otherwise, TCL_ERROR is returned
///* and an error message loaded into interpreter pDb->interp.
#[allow(unused_doc_comments)]
extern "C" fn db_prepare_and_bind(p_db_1: *mut SqliteDb, z_in_1: *const i8,
    pz_out_1: *mut *const i8, pp_pre_stmt_1: &mut *mut SqlPreparedStmt)
    -> i32 {
    let mut z_sql: *const i8 = z_in_1;
    /// Pointer to first SQL statement in zIn
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    /// Prepared statement object
    let mut p_pre_stmt: *mut SqlPreparedStmt = core::ptr::null_mut();
    /// Pointer to cached statement
    let mut n_sql: i32 = 0;
    /// Length of zSql in bytes
    let mut n_var: i32 = 0;
    /// Number of variables in statement
    let mut i_parm: i32 = 0;
    /// Next free entry in apParm
    let mut c: i8 = 0 as i8;
    let mut i: i32 = 0;
    let mut need_result_reset: i32 = 0;
    /// Need to invoke Tcl_ResetResult()
    let mut rc: i32 = 0;
    /// Value to return
    let interp: *mut TclInterp = unsafe { (*p_db_1).interp };
    *pp_pre_stmt_1 = core::ptr::null_mut();
    while { c = unsafe { *z_sql.offset(0 as isize) } as i8; c } as i32 ==
                        ' ' as i32 || c as i32 == '\t' as i32 ||
                c as i32 == '\r' as i32 || c as i32 == '\n' as i32 {
        {
            let __p = &mut z_sql;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    n_sql = strlen30(z_sql);
    {
        p_pre_stmt = unsafe { (*p_db_1).stmt_list };
        '__b31: loop {
            if !(!(p_pre_stmt).is_null()) { break '__b31; }
            '__c31: loop {
                let mut n: i32 = unsafe { (*p_pre_stmt).n_sql };
                if n_sql >= n &&
                            unsafe {
                                    memcmp(unsafe { (*p_pre_stmt).z_sql } as *const (),
                                        z_sql as *const (), n as u64)
                                } == 0 &&
                        (unsafe { *z_sql.offset(n as isize) } as i32 == 0 ||
                            unsafe { *z_sql.offset((n - 1) as isize) } as i32 ==
                                ';' as i32) {
                    p_stmt = unsafe { (*p_pre_stmt).p_stmt };
                    unsafe {
                        *pz_out_1 =
                            unsafe {
                                z_sql.offset(unsafe { (*p_pre_stmt).n_sql } as isize)
                            }
                    };
                    if !(unsafe { (*p_pre_stmt).p_prev }).is_null() {
                        unsafe {
                            (*unsafe { (*p_pre_stmt).p_prev }).p_next =
                                unsafe { (*p_pre_stmt).p_next }
                        };
                    } else {
                        unsafe {
                            (*p_db_1).stmt_list = unsafe { (*p_pre_stmt).p_next }
                        };
                    }
                    if !(unsafe { (*p_pre_stmt).p_next }).is_null() {
                        unsafe {
                            (*unsafe { (*p_pre_stmt).p_next }).p_prev =
                                unsafe { (*p_pre_stmt).p_prev }
                        };
                    } else {
                        unsafe {
                            (*p_db_1).stmt_last = unsafe { (*p_pre_stmt).p_prev }
                        };
                    }
                    {
                        let __p = unsafe { &mut (*p_db_1).n_stmt };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    n_var = unsafe { sqlite3_bind_parameter_count(p_stmt) };
                    break '__b31;
                }
                break '__c31;
            }
            p_pre_stmt = unsafe { (*p_pre_stmt).p_next };
        }
    }
    if p_pre_stmt == core::ptr::null_mut() {
        let mut n_byte: i32 = 0;
        if 0 != db_prepare(unsafe { &*p_db_1 }, z_sql, &mut p_stmt, pz_out_1)
            {
            unsafe {
                Tcl_SetObjResult(interp,
                    unsafe {
                        Tcl_NewStringObj(unsafe {
                                sqlite3_errmsg(unsafe { (*p_db_1).db })
                            }, -1)
                    })
            };
            return 1;
        }
        if p_stmt == core::ptr::null_mut() {
            if 0 != unsafe { sqlite3_errcode(unsafe { (*p_db_1).db }) } {

                /// A compile-time error in the statement.
                unsafe {
                    Tcl_SetObjResult(interp,
                        unsafe {
                            Tcl_NewStringObj(unsafe {
                                    sqlite3_errmsg(unsafe { (*p_db_1).db })
                                }, -1)
                        })
                };
                return 1;
            } else {

                /// The statement was a no-op.  Continue to the next statement
                ///* in the SQL string.
                return 0;
            }
        }
        if !(p_pre_stmt == core::ptr::null_mut()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"dbPrepareAndBind".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 1467,
                    c"pPreStmt==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        n_var = unsafe { sqlite3_bind_parameter_count(p_stmt) };
        n_byte =
            (core::mem::size_of::<SqlPreparedStmt>() as u64 +
                    n_var as u64 * core::mem::size_of::<*mut TclObj>() as u64)
                as i32;
        p_pre_stmt =
            unsafe { Tcl_Alloc(n_byte as u32) } as *mut SqlPreparedStmt;
        unsafe { memset(p_pre_stmt as *mut (), 0, n_byte as u64) };
        unsafe { (*p_pre_stmt).p_stmt = p_stmt };
        unsafe {
            (*p_pre_stmt).n_sql =
                unsafe { unsafe { (*pz_out_1).offset_from(z_sql) } } as i64 as
                    i32
        };
        unsafe { (*p_pre_stmt).z_sql = unsafe { sqlite3_sql(p_stmt) } };
        unsafe {
            (*p_pre_stmt).ap_parm =
                unsafe { &raw mut *p_pre_stmt.offset(1 as isize) } as
                    *mut *mut TclObj
        };
    }
    if (p_pre_stmt).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dbPrepareAndBind".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 1486,
                c"pPreStmt".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(strlen30(unsafe { (*p_pre_stmt).z_sql }) ==
                            unsafe { (*p_pre_stmt).n_sql }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dbPrepareAndBind".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 1487,
                c"strlen30(pPreStmt->zSql)==pPreStmt->nSql".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(0 ==
                            unsafe {
                                memcmp(unsafe { (*p_pre_stmt).z_sql } as *const (),
                                    z_sql as *const (), unsafe { (*p_pre_stmt).n_sql } as u64)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dbPrepareAndBind".as_ptr() as *const i8,
                c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 1488,
                c"0==memcmp(pPreStmt->zSql, zSql, pPreStmt->nSql)".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        i = 1;
        '__b32: loop {
            if !(i <= n_var) { break '__b32; }
            '__c32: loop {
                let z_var: *const i8 =
                    unsafe { sqlite3_bind_parameter_name(p_stmt, i) };
                if z_var != core::ptr::null() &&
                        (unsafe { *z_var.offset(0 as isize) } as i32 == '$' as i32
                                || unsafe { *z_var.offset(0 as isize) } as i32 == ':' as i32
                            ||
                            unsafe { *z_var.offset(0 as isize) } as i32 == '@' as i32) {
                    let mut p_var: *mut TclObj =
                        unsafe {
                            Tcl_GetVar2Ex(interp, unsafe { &*z_var.offset(1 as isize) },
                                core::ptr::null(), 0)
                        };
                    if p_var == core::ptr::null_mut() &&
                            unsafe { (*p_db_1).z_bind_fallback } !=
                                core::ptr::null_mut() {
                        let mut p_cmd: *mut TclObj = core::ptr::null_mut();
                        let mut rx: i32 = 0;
                        p_cmd =
                            unsafe {
                                Tcl_NewStringObj(unsafe { (*p_db_1).z_bind_fallback } as
                                        *const i8, -1)
                            };
                        {
                            let __p = unsafe { &mut (*p_cmd).refCount };
                            *__p += 1;
                            *__p
                        };
                        unsafe {
                            Tcl_ListObjAppendElement(interp, p_cmd,
                                unsafe { Tcl_NewStringObj(z_var, -1) })
                        };
                        if need_result_reset != 0 {
                            unsafe { Tcl_ResetResult(interp) };
                        }
                        need_result_reset = 1;
                        rx = unsafe { Tcl_EvalObjEx(interp, p_cmd, 262144) };
                        '__b33: loop {
                            '__c33: loop {
                                if {
                                            let __p = unsafe { &mut (*p_cmd).refCount };
                                            *__p -= 1;
                                            *__p
                                        } <= 0 {
                                    unsafe { TclFreeObj(p_cmd) };
                                }
                                break '__c33;
                            }
                            if !(0 != 0) { break '__b33; }
                        }
                        if rx == 0 {
                            p_var = unsafe { Tcl_GetObjResult(interp) };
                        } else if rx == 1 {
                            rc = 1;
                            break '__b32;
                        } else { p_var = core::ptr::null_mut(); }
                    }
                    if !(p_var).is_null() {
                        let mut n: TclSize = 0;
                        let mut data: *mut u8 = core::ptr::null_mut();
                        let z_type: *const i8 =
                            if !(unsafe { (*p_var).typePtr }).is_null() {
                                    unsafe { (*unsafe { (*p_var).typePtr }).name }
                                } else { c"".as_ptr() as *mut i8 } as *const i8;
                        c = unsafe { *z_type.offset(0 as isize) } as i8;
                        if unsafe { *z_var.offset(0 as isize) } as i32 == '@' as i32
                                ||
                                c as i32 == 'b' as i32 &&
                                        unsafe {
                                                strcmp(z_type,
                                                    c"bytearray".as_ptr() as *mut i8 as *const i8)
                                            } == 0 && unsafe { (*p_var).bytes } == core::ptr::null_mut()
                            {

                            /// Load a BLOB type if the Tcl variable is a bytearray and
                            ///* it has no string representation or the host
                            ///* parameter name begins with "@".
                            (data = unsafe { Tcl_GetByteArrayFromObj(p_var, &mut n) });
                            unsafe {
                                sqlite3_bind_blob(p_stmt, i, data as *const (), n, None)
                            };
                            {
                                let __p = unsafe { &mut (*p_var).refCount };
                                *__p += 1;
                                *__p
                            };
                            unsafe {
                                *unsafe {
                                            (*p_pre_stmt).ap_parm.offset({
                                                        let __p = &mut i_parm;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                        } = p_var
                            };
                        } else if c as i32 == 'b' as i32 &&
                                    unsafe { (*p_var).bytes } == core::ptr::null_mut() &&
                                (unsafe {
                                            strcmp(z_type,
                                                c"booleanString".as_ptr() as *mut i8 as *const i8)
                                        } == 0 ||
                                    unsafe {
                                            strcmp(z_type, c"boolean".as_ptr() as *mut i8 as *const i8)
                                        } == 0) {
                            let mut nn: i32 = 0;
                            unsafe { Tcl_GetBooleanFromObj(interp, p_var, &mut nn) };
                            unsafe { sqlite3_bind_int(p_stmt, i, nn) };
                        } else if c as i32 == 'd' as i32 &&
                                unsafe {
                                        strcmp(z_type, c"double".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            let mut r: f64 = 0.0;
                            unsafe { Tcl_GetDoubleFromObj(interp, p_var, &mut r) };
                            unsafe { sqlite3_bind_double(p_stmt, i, r) };
                        } else if c as i32 == 'w' as i32 &&
                                    unsafe {
                                            strcmp(z_type, c"wideInt".as_ptr() as *mut i8 as *const i8)
                                        } == 0 ||
                                c as i32 == 'i' as i32 &&
                                    unsafe {
                                            strcmp(z_type, c"int".as_ptr() as *mut i8 as *const i8)
                                        } == 0 {
                            let mut v: TclWideInt = 0 as TclWideInt;
                            unsafe { Tcl_GetWideIntFromObj(interp, p_var, &mut v) };
                            unsafe { sqlite3_bind_int64(p_stmt, i, v) };
                        } else {
                            data =
                                unsafe { Tcl_GetStringFromObj(p_var, &mut n) } as *mut u8;
                            unsafe {
                                sqlite3_bind_text64(p_stmt, i, data as *mut i8 as *const i8,
                                    n as Sqlite3Uint64, None, 1 as u8)
                            };
                            {
                                let __p = unsafe { &mut (*p_var).refCount };
                                *__p += 1;
                                *__p
                            };
                            unsafe {
                                *unsafe {
                                            (*p_pre_stmt).ap_parm.offset({
                                                        let __p = &mut i_parm;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                        } = p_var
                            };
                        }
                    } else { unsafe { sqlite3_bind_null(p_stmt, i) }; }
                    if need_result_reset != 0 {
                        unsafe { Tcl_ResetResult(unsafe { (*p_db_1).interp }) };
                    }
                }
                break '__c32;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p_pre_stmt).n_parm = i_parm };
    *pp_pre_stmt_1 = p_pre_stmt;
    if need_result_reset != 0 && rc == 0 {
        unsafe { Tcl_ResetResult(unsafe { (*p_db_1).interp }) };
    }
    return rc;
}

///* Release a statement reference obtained by calling dbPrepareAndBind().
///* There should be exactly one call to this function for each call to
///* dbPrepareAndBind().
///*
///* If the discard parameter is non-zero, then the statement is deleted
///* immediately. Otherwise it is added to the LRU list and may be returned
///* by a subsequent call to dbPrepareAndBind().
#[allow(unused_doc_comments)]
extern "C" fn db_release_stmt(p_db_1: &mut SqliteDb,
    p_pre_stmt_1: *mut SqlPreparedStmt, discard: i32) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b34: loop {
            if !(i < unsafe { (*p_pre_stmt_1).n_parm }) { break '__b34; }
            '__c34: loop {
                '__b35: loop {
                    '__c35: loop {
                        if {
                                    let __p =
                                        unsafe {
                                            &mut (*unsafe {
                                                                *unsafe { (*p_pre_stmt_1).ap_parm.offset(i as isize) }
                                                            }).refCount
                                        };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            unsafe {
                                TclFreeObj(unsafe {
                                        *unsafe { (*p_pre_stmt_1).ap_parm.offset(i as isize) }
                                    })
                            };
                        }
                        break '__c35;
                    }
                    if !(0 != 0) { break '__b35; }
                }
                break '__c34;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p_pre_stmt_1).n_parm = 0 };
    if (*p_db_1).max_stmt <= 0 || discard != 0 {

        /// If the cache is turned off, deallocated the statement
        db_free_stmt(p_pre_stmt_1);
    } else {

        /// Add the prepared statement to the beginning of the cache list.
        unsafe { (*p_pre_stmt_1).p_next = (*p_db_1).stmt_list };
        unsafe { (*p_pre_stmt_1).p_prev = core::ptr::null_mut() };
        if !((*p_db_1).stmt_list).is_null() {
            unsafe { (*(*p_db_1).stmt_list).p_prev = p_pre_stmt_1 };
        }
        (*p_db_1).stmt_list = p_pre_stmt_1;
        if (*p_db_1).stmt_last == core::ptr::null_mut() {
            if !((*p_db_1).n_stmt == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"dbReleaseStmt".as_ptr() as *const i8,
                        c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 1598,
                        c"pDb->nStmt==0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            (*p_db_1).stmt_last = p_pre_stmt_1;
        } else {
            if !((*p_db_1).n_stmt > 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"dbReleaseStmt".as_ptr() as *const i8,
                        c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 1601,
                        c"pDb->nStmt>0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
        }
        { let __p = &mut (*p_db_1).n_stmt; let __t = *__p; *__p += 1; __t };
        while (*p_db_1).n_stmt > (*p_db_1).max_stmt {
            let p_last: *mut SqlPreparedStmt = (*p_db_1).stmt_last;
            (*p_db_1).stmt_last = unsafe { (*p_last).p_prev };
            unsafe { (*(*p_db_1).stmt_last).p_next = core::ptr::null_mut() };
            {
                let __p = &mut (*p_db_1).n_stmt;
                let __t = *__p;
                *__p -= 1;
                __t
            };
            db_free_stmt(p_last);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DbEvalContext {
    p_db: *mut SqliteDb,
    p_sql: *mut TclObj,
    z_sql: *const i8,
    p_pre_stmt: *mut SqlPreparedStmt,
    n_col: i32,
    eval_flags: i32,
    p_var_name: *mut TclObj,
    ap_col_name: *mut *mut TclObj,
}

///* Release any cache of column names currently held as part of
///* the DbEvalContext structure passed as the first argument.
extern "C" fn db_release_column_names(p: &mut DbEvalContext) -> () {
    if !((*p).ap_col_name).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b37: loop {
                if !(i < (*p).n_col) { break '__b37; }
                '__c37: loop {
                    '__b38: loop {
                        '__c38: loop {
                            if {
                                        let __p =
                                            unsafe {
                                                &mut (*unsafe {
                                                                    *(*p).ap_col_name.offset(i as isize)
                                                                }).refCount
                                            };
                                        *__p -= 1;
                                        *__p
                                    } <= 0 {
                                unsafe {
                                    TclFreeObj(unsafe { *(*p).ap_col_name.offset(i as isize) })
                                };
                            }
                            break '__c38;
                        }
                        if !(0 != 0) { break '__b38; }
                    }
                    break '__c37;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { Tcl_Free((*p).ap_col_name as *mut i8) };
        (*p).ap_col_name = core::ptr::null_mut();
    }
    (*p).n_col = 0;
}

///* Initialize a DbEvalContext structure.
///*
///* If pVarName is not NULL, then it contains the name of a Tcl array
///* variable. The "*" member of this array is set to a list containing
///* the names of the columns returned by the statement as part of each
///* call to dbEvalStep(), in order from left to right. e.g. if the names
///* of the returned columns are a, b and c, it does the equivalent of the
///* tcl command:
///*
///*     set ${pVarName}(*) {a b c}
extern "C" fn db_eval_init(p: *mut DbEvalContext, p_db_1: *mut SqliteDb,
    p_sql_1: *mut TclObj, p_var_name_1: *mut TclObj, eval_flags_1: i32)
    -> () {
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<DbEvalContext>() as u64)
    };
    unsafe { (*p).p_db = p_db_1 };
    unsafe { (*p).z_sql = unsafe { Tcl_GetString(p_sql_1) } as *const i8 };
    unsafe { (*p).p_sql = p_sql_1 };
    { let __p = unsafe { &mut (*p_sql_1).refCount }; *__p += 1; *__p };
    if !(p_var_name_1).is_null() {
        unsafe { (*p).p_var_name = p_var_name_1 };
        {
            let __p = unsafe { &mut (*p_var_name_1).refCount };
            *__p += 1;
            *__p
        };
    }
    unsafe { (*p).eval_flags = eval_flags_1 };
    add_database_ref(unsafe { &mut *unsafe { (*p).p_db } });
}

///* Obtain information about the row that the DbEvalContext passed as the
///* first argument currently points to.
#[allow(unused_doc_comments)]
extern "C" fn db_eval_row_info(p: &mut DbEvalContext, pn_col_1: *mut i32,
    pap_col_name_1: *mut *mut *mut TclObj) -> () {
    if core::ptr::null_mut() == (*p).ap_col_name {
        let p_stmt: *mut Sqlite3Stmt = unsafe { (*(*p).p_pre_stmt).p_stmt };
        let mut i: i32 = 0;
        /// Iterator variable
        let mut n_col: i32 = 0;
        /// Number of columns returned by pStmt
        let mut ap_col_name: *mut *mut TclObj = core::ptr::null_mut();

        /// Array of column names
        ((*p).n_col =
            { n_col = unsafe { sqlite3_column_count(p_stmt) }; n_col });
        if n_col > 0 &&
                (!(pap_col_name_1).is_null() || !((*p).p_var_name).is_null())
            {
            ap_col_name =
                unsafe {
                        Tcl_Alloc((core::mem::size_of::<*mut TclObj>() as u64 *
                                    n_col as u64) as u32)
                    } as *mut *mut TclObj;
            {
                i = 0;
                '__b39: loop {
                    if !(i < n_col) { break '__b39; }
                    '__c39: loop {
                        unsafe {
                            *ap_col_name.offset(i as isize) =
                                unsafe {
                                    Tcl_NewStringObj(unsafe { sqlite3_column_name(p_stmt, i) },
                                        -1)
                                }
                        };
                        {
                            let __p =
                                unsafe {
                                    &mut (*unsafe { *ap_col_name.offset(i as isize) }).refCount
                                };
                            *__p += 1;
                            *__p
                        };
                        break '__c39;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            (*p).ap_col_name = ap_col_name;
        }
        if !((*p).p_var_name).is_null() {
            let interp: *mut TclInterp = unsafe { (*(*p).p_db).interp };
            let p_col_list: *mut TclObj = unsafe { Tcl_NewObj() };
            let p_star: *mut TclObj =
                unsafe {
                    Tcl_NewStringObj(c"*".as_ptr() as *mut i8 as *const i8, -1)
                };
            {
                let __p = unsafe { &mut (*p_col_list).refCount };
                *__p += 1;
                *__p
            };
            { let __p = unsafe { &mut (*p_star).refCount }; *__p += 1; *__p };
            {
                i = 0;
                '__b40: loop {
                    if !(i < n_col) { break '__b40; }
                    '__c40: loop {
                        unsafe {
                            Tcl_ListObjAppendElement(interp, p_col_list,
                                unsafe { *ap_col_name.offset(i as isize) })
                        };
                        break '__c40;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if 0 == 2 & (*p).eval_flags {
                unsafe {
                    Tcl_ObjSetVar2(interp, (*p).p_var_name, p_star, p_col_list,
                        0)
                };
            } else {
                let mut p_dict: *mut TclObj =
                    unsafe {
                        Tcl_ObjGetVar2(interp, (*p).p_var_name,
                            0 as *mut () as *mut TclObj, 0)
                    };
                if (p_dict).is_null() as i32 != 0 {
                    p_dict = unsafe { Tcl_NewDictObj() };
                } else if unsafe { (*p_dict).refCount } > 1 {
                    p_dict = unsafe { Tcl_DuplicateObj(p_dict) };
                }
                if unsafe {
                            Tcl_DictObjPut(interp, p_dict, p_star, p_col_list)
                        } == 0 {
                    unsafe {
                        Tcl_ObjSetVar2(interp, (*p).p_var_name,
                            0 as *mut () as *mut TclObj, p_dict, 0)
                    };
                }
                {
                    let __p = unsafe { &mut (*p_dict).refCount };
                    *__p += 1;
                    *__p
                };
                '__b41: loop {
                    '__c41: loop {
                        if {
                                    let __p = unsafe { &mut (*p_dict).refCount };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            unsafe { TclFreeObj(p_dict) };
                        }
                        break '__c41;
                    }
                    if !(0 != 0) { break '__b41; }
                }
            }
            '__b42: loop {
                '__c42: loop {
                    if {
                                let __p = unsafe { &mut (*p_star).refCount };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(p_star) };
                    }
                    break '__c42;
                }
                if !(0 != 0) { break '__b42; }
            }
            '__b43: loop {
                '__c43: loop {
                    if {
                                let __p = unsafe { &mut (*p_col_list).refCount };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(p_col_list) };
                    }
                    break '__c43;
                }
                if !(0 != 0) { break '__b43; }
            }
        }
    }
    if !(pap_col_name_1).is_null() {
        unsafe { *pap_col_name_1 = (*p).ap_col_name };
    }
    if !(pn_col_1).is_null() { unsafe { *pn_col_1 = (*p).n_col }; }
}

///* Return one of TCL_OK, TCL_BREAK or TCL_ERROR. If TCL_ERROR is
///* returned, then an error message is stored in the interpreter before
///* returning.
///*
///* A return value of TCL_OK means there is a row of data available. The
///* data may be accessed using dbEvalRowInfo() and dbEvalColumnValue(). This
///* is analogous to a return of SQLITE_ROW from sqlite3_step(). If TCL_BREAK
///* is returned, then the SQL script has finished executing and there are
///* no further rows available. This is similar to SQLITE_DONE.
#[allow(unused_doc_comments)]
extern "C" fn db_eval_step(p: *mut DbEvalContext) -> i32 {
    let mut z_prev_sql: *const i8 = core::ptr::null();
    while unsafe { *unsafe { (*p).z_sql.offset(0 as isize) } } != 0 ||
            !(unsafe { (*p).p_pre_stmt }).is_null() {
        let mut rc: i32 = 0;
        if unsafe { (*p).p_pre_stmt } == core::ptr::null_mut() {
            z_prev_sql =
                if unsafe { (*p).z_sql } == z_prev_sql {
                    core::ptr::null()
                } else { unsafe { (*p).z_sql } };
            rc =
                db_prepare_and_bind(unsafe { (*p).p_db },
                    unsafe { (*p).z_sql }, unsafe { &mut (*p).z_sql },
                    unsafe { &mut (*p).p_pre_stmt });
            if rc != 0 { return rc; }
        } else {
            let mut rcs: i32 = 0;
            let p_db: *mut SqliteDb = unsafe { (*p).p_db };
            let p_pre_stmt: *mut SqlPreparedStmt = unsafe { (*p).p_pre_stmt };
            let p_stmt: *mut Sqlite3Stmt = unsafe { (*p_pre_stmt).p_stmt };
            rcs = unsafe { sqlite3_step(p_stmt) };
            if rcs == 100 { return 0; }
            if !(unsafe { (*p).p_var_name }).is_null() {
                db_eval_row_info(unsafe { &mut *p }, core::ptr::null_mut(),
                    core::ptr::null_mut());
            }
            rcs = unsafe { sqlite3_reset(p_stmt) };
            unsafe {
                (*p_db).n_step = unsafe { sqlite3_stmt_status(p_stmt, 1, 1) }
            };
            unsafe {
                (*p_db).n_sort = unsafe { sqlite3_stmt_status(p_stmt, 2, 1) }
            };
            unsafe {
                (*p_db).n_index = unsafe { sqlite3_stmt_status(p_stmt, 3, 1) }
            };
            unsafe {
                (*p_db).n_vm_step =
                    unsafe { sqlite3_stmt_status(p_stmt, 4, 1) }
            };
            db_release_column_names(unsafe { &mut *p });
            unsafe { (*p).p_pre_stmt = core::ptr::null_mut() };
            if rcs != 0 {

                /// If a run-time error occurs, report the error and stop reading
                ///* the SQL.
                db_release_stmt(unsafe { &mut *p_db }, p_pre_stmt, 1);
                unsafe {
                    Tcl_SetObjResult(unsafe { (*p_db).interp },
                        unsafe {
                            Tcl_NewStringObj(unsafe {
                                    sqlite3_errmsg(unsafe { (*p_db).db })
                                }, -1)
                        })
                };
                return 1;
            } else { db_release_stmt(unsafe { &mut *p_db }, p_pre_stmt, 0); }
        }
    }

    /// Finished
    return 3;
}

///* Free all resources currently held by the DbEvalContext structure passed
///* as the first argument. There should be exactly one call to this function
///* for each call to dbEvalInit().
extern "C" fn db_eval_finalize(p: *mut DbEvalContext) -> () {
    if !(unsafe { (*p).p_pre_stmt }).is_null() {
        unsafe {
            sqlite3_reset(unsafe { (*unsafe { (*p).p_pre_stmt }).p_stmt })
        };
        db_release_stmt(unsafe { &mut *unsafe { (*p).p_db } },
            unsafe { (*p).p_pre_stmt }, 0);
        unsafe { (*p).p_pre_stmt = core::ptr::null_mut() };
    }
    if !(unsafe { (*p).p_var_name }).is_null() {
        '__b45: loop {
            '__c45: loop {
                if {
                            let __p =
                                unsafe { &mut (*unsafe { (*p).p_var_name }).refCount };
                            *__p -= 1;
                            *__p
                        } <= 0 {
                    unsafe { TclFreeObj(unsafe { (*p).p_var_name }) };
                }
                break '__c45;
            }
            if !(0 != 0) { break '__b45; }
        }
        unsafe { (*p).p_var_name = core::ptr::null_mut() };
    }
    '__b46: loop {
        '__c46: loop {
            if {
                        let __p = unsafe { &mut (*unsafe { (*p).p_sql }).refCount };
                        *__p -= 1;
                        *__p
                    } <= 0 {
                unsafe { TclFreeObj(unsafe { (*p).p_sql }) };
            }
            break '__c46;
        }
        if !(0 != 0) { break '__b46; }
    }
    db_release_column_names(unsafe { &mut *p });
    del_database_ref(unsafe { (*p).p_db });
}

///* Return a pointer to a Tcl_Obj structure with ref-count 0 that contains
///* the value for the iCol'th column of the row currently pointed to by
///* the DbEvalContext structure passed as the first argument.
extern "C" fn db_eval_column_value(p: &DbEvalContext, i_col_1: i32)
    -> *mut TclObj {
    let p_stmt: *mut Sqlite3Stmt = unsafe { (*(*p).p_pre_stmt).p_stmt };
    '__s47:
        {
        match unsafe { sqlite3_column_type(p_stmt, i_col_1) } {
            4 => {
                {
                    let mut bytes: i32 =
                        unsafe { sqlite3_column_bytes(p_stmt, i_col_1) };
                    let z_blob: *const i8 =
                        unsafe { sqlite3_column_blob(p_stmt, i_col_1) } as
                            *const i8;
                    if (z_blob).is_null() as i32 != 0 { bytes = 0; }
                    return unsafe {
                            Tcl_NewByteArrayObj(z_blob as *mut u8 as *const u8, bytes)
                        };
                }
                {
                    let v: SqliteInt64 =
                        unsafe { sqlite3_column_int64(p_stmt, i_col_1) };
                    if v >= -2147483647 as i64 && v <= 2147483647 as i64 {
                        return unsafe { Tcl_NewIntObj(v as i32) };
                    } else { return unsafe { Tcl_NewWideIntObj(v) }; }
                }
                {
                    return unsafe {
                            Tcl_NewDoubleObj(unsafe {
                                    sqlite3_column_double(p_stmt, i_col_1)
                                })
                        };
                }
                {
                    return unsafe {
                            Tcl_NewStringObj(unsafe { (*(*p).p_db).z_null } as
                                    *const i8, -1)
                        };
                }
            }
            1 => {
                {
                    let v: SqliteInt64 =
                        unsafe { sqlite3_column_int64(p_stmt, i_col_1) };
                    if v >= -2147483647 as i64 && v <= 2147483647 as i64 {
                        return unsafe { Tcl_NewIntObj(v as i32) };
                    } else { return unsafe { Tcl_NewWideIntObj(v) }; }
                }
                {
                    return unsafe {
                            Tcl_NewDoubleObj(unsafe {
                                    sqlite3_column_double(p_stmt, i_col_1)
                                })
                        };
                }
                {
                    return unsafe {
                            Tcl_NewStringObj(unsafe { (*(*p).p_db).z_null } as
                                    *const i8, -1)
                        };
                }
            }
            2 => {
                {
                    return unsafe {
                            Tcl_NewDoubleObj(unsafe {
                                    sqlite3_column_double(p_stmt, i_col_1)
                                })
                        };
                }
                {
                    return unsafe {
                            Tcl_NewStringObj(unsafe { (*(*p).p_db).z_null } as
                                    *const i8, -1)
                        };
                }
            }
            5 => {
                {
                    return unsafe {
                            Tcl_NewStringObj(unsafe { (*(*p).p_db).z_null } as
                                    *const i8, -1)
                        };
                }
            }
            _ => {}
        }
    }
    return unsafe {
            Tcl_NewStringObj(unsafe { sqlite3_column_text(p_stmt, i_col_1) }
                        as *mut i8 as *const i8, -1)
        };
}

///* This function is part of the implementation of the command:
///*
///*   $db eval SQL ?TGT-NAME? SCRIPT
#[allow(unused_doc_comments)]
extern "C" fn db_eval_next_cmd(data: *mut ClientData, interp: *mut TclInterp,
    result: i32) -> i32 {
    let mut rc: i32 = result;
    /// Return code
    /// The first element of the data[] array is a pointer to a DbEvalContext
    ///* structure allocated using Tcl_Alloc(). The second element of data[]
    ///* is a pointer to a Tcl_Obj containing the script to run for each row
    ///* returned by the queries encapsulated in data[0].
    let p: *mut DbEvalContext =
        unsafe { *data.offset(0 as isize) } as *mut DbEvalContext;
    let p_script: *mut TclObj =
        unsafe { *data.offset(1 as isize) } as *mut TclObj;
    let p_var_name: *mut TclObj = unsafe { (*p).p_var_name };
    while (rc == 0 || rc == 4) && 0 == { rc = db_eval_step(p); rc } {
        let mut i: i32 = 0;
        let mut n_col: i32 = 0;
        let mut ap_col_name: *mut *mut TclObj = core::ptr::null_mut();
        db_eval_row_info(unsafe { &mut *p }, &mut n_col, &mut ap_col_name);
        {
            i = 0;
            '__b49: loop {
                if !(i < n_col) { break '__b49; }
                '__c49: loop {
                    if p_var_name == core::ptr::null_mut() {
                        unsafe {
                            Tcl_ObjSetVar2(interp,
                                unsafe { *ap_col_name.offset(i as isize) },
                                core::ptr::null_mut(),
                                db_eval_column_value(unsafe { &*p }, i), 0)
                        };
                    } else if unsafe { (*p).eval_flags } & 1 != 0 &&
                            unsafe {
                                    sqlite3_column_type(unsafe {
                                            (*unsafe { (*p).p_pre_stmt }).p_stmt
                                        }, i)
                                } == 5 {
                        if 0 == 2 & unsafe { (*p).eval_flags } {

                            /// Target is an array
                            unsafe {
                                Tcl_UnsetVar2(interp,
                                    unsafe { Tcl_GetString(p_var_name) } as *const i8,
                                    unsafe {
                                            Tcl_GetString(unsafe { *ap_col_name.offset(i as isize) })
                                        } as *const i8, 0)
                            };
                        } else {
                            /// Target is a dict
                            let mut p_dict: *mut TclObj =
                                unsafe {
                                    Tcl_ObjGetVar2(interp, p_var_name,
                                        0 as *mut () as *mut TclObj, 0)
                                };
                            if !(p_dict).is_null() {
                                if unsafe { (*p_dict).refCount } > 1 {
                                    p_dict = unsafe { Tcl_DuplicateObj(p_dict) };
                                }
                                if unsafe {
                                            Tcl_DictObjRemove(interp, p_dict,
                                                unsafe { *ap_col_name.offset(i as isize) })
                                        } == 0 {
                                    unsafe {
                                        Tcl_ObjSetVar2(interp, p_var_name,
                                            0 as *mut () as *mut TclObj, p_dict, 0)
                                    };
                                }
                                {
                                    let __p = unsafe { &mut (*p_dict).refCount };
                                    *__p += 1;
                                    *__p
                                };
                                '__b50: loop {
                                    '__c50: loop {
                                        if {
                                                    let __p = unsafe { &mut (*p_dict).refCount };
                                                    *__p -= 1;
                                                    *__p
                                                } <= 0 {
                                            unsafe { TclFreeObj(p_dict) };
                                        }
                                        break '__c50;
                                    }
                                    if !(0 != 0) { break '__b50; }
                                }
                            }
                        }
                    } else if 0 == 2 & unsafe { (*p).eval_flags } {

                        /// Target is an array: set target(colName) = colValue
                        unsafe {
                            Tcl_ObjSetVar2(interp, p_var_name,
                                unsafe { *ap_col_name.offset(i as isize) },
                                db_eval_column_value(unsafe { &*p }, i), 0)
                        };
                    } else {
                        /// Target is a dict: set target(colName) = colValue
                        let mut p_dict_1: *mut TclObj =
                            unsafe {
                                Tcl_ObjGetVar2(interp, p_var_name,
                                    0 as *mut () as *mut TclObj, 0)
                            };
                        if (p_dict_1).is_null() as i32 != 0 {
                            p_dict_1 = unsafe { Tcl_NewDictObj() };
                        } else if unsafe { (*p_dict_1).refCount } > 1 {
                            p_dict_1 = unsafe { Tcl_DuplicateObj(p_dict_1) };
                        }
                        if unsafe {
                                    Tcl_DictObjPut(interp, p_dict_1,
                                        unsafe { *ap_col_name.offset(i as isize) },
                                        db_eval_column_value(unsafe { &*p }, i))
                                } == 0 {
                            unsafe {
                                Tcl_ObjSetVar2(interp, p_var_name,
                                    0 as *mut () as *mut TclObj, p_dict_1, 0)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p_dict_1).refCount };
                            *__p += 1;
                            *__p
                        };
                        '__b51: loop {
                            '__c51: loop {
                                if {
                                            let __p = unsafe { &mut (*p_dict_1).refCount };
                                            *__p -= 1;
                                            *__p
                                        } <= 0 {
                                    unsafe { TclFreeObj(p_dict_1) };
                                }
                                break '__c51;
                            }
                            if !(0 != 0) { break '__b51; }
                        }
                    }
                    break '__c49;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if 0 != 0 {
            { let _ = 0; };
            return 0;
        } else { rc = unsafe { Tcl_EvalObjEx(interp, p_script, 0) }; }
    }
    '__b52: loop {
        '__c52: loop {
            if {
                        let __p = unsafe { &mut (*p_script).refCount };
                        *__p -= 1;
                        *__p
                    } <= 0 {
                unsafe { TclFreeObj(p_script) };
            }
            break '__c52;
        }
        if !(0 != 0) { break '__b52; }
    }
    db_eval_finalize(p);
    unsafe { Tcl_Free(p as *mut i8) };
    if rc == 0 || rc == 3 { unsafe { Tcl_ResetResult(interp) }; rc = 0; }
    return rc;
}

///* This function is used by the implementations of the following database
///* handle sub-commands:
///*
///*   $db update_hook ?SCRIPT?
///*   $db wal_hook ?SCRIPT?
///*   $db commit_hook ?SCRIPT?
///*   $db preupdate hook ?SCRIPT?
extern "C" fn db_hook_cmd(interp: *mut TclInterp, p_db_1: *mut SqliteDb,
    p_arg_1: *mut TclObj, pp_hook_1: &mut *mut TclObj) -> () {
    let db: *mut Sqlite3 = unsafe { (*p_db_1).db };
    if !(*pp_hook_1).is_null() {
        unsafe { Tcl_SetObjResult(interp, *pp_hook_1) };
        if !(p_arg_1).is_null() {
            '__b53: loop {
                '__c53: loop {
                    if {
                                let __p = unsafe { &mut (**pp_hook_1).refCount };
                                *__p -= 1;
                                *__p
                            } <= 0 {
                        unsafe { TclFreeObj(*pp_hook_1) };
                    }
                    break '__c53;
                }
                if !(0 != 0) { break '__b53; }
            }
            *pp_hook_1 = core::ptr::null_mut();
        }
    }
    if !(p_arg_1).is_null() {
        if ((*pp_hook_1).is_null() as i32 == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"DbHookCmd".as_ptr() as *const i8,
                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 2032,
                    c"!(*ppHook)".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if unsafe { *unsafe { Tcl_GetString(p_arg_1).offset(0 as isize) } } !=
                0 {
            *pp_hook_1 = p_arg_1;
            {
                let __p = unsafe { &mut (**pp_hook_1).refCount };
                *__p += 1;
                *__p
            };
        }
    }
    unsafe {
        sqlite3_update_hook(db,
            Some(if !(unsafe { (*p_db_1).p_update_hook }).is_null() {
                    db_update_handler
                } else {
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                    i64) -> ()>(0 as *const ())
                    }
                }), p_db_1 as *mut ())
    };
    unsafe {
        sqlite3_rollback_hook(db,
            Some(if !(unsafe { (*p_db_1).p_rollback_hook }).is_null() {
                    db_rollback_handler
                } else {
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
                    }
                }), p_db_1 as *mut ())
    };
    unsafe {
        sqlite3_wal_hook(db,
            Some(if !(unsafe { (*p_db_1).p_wal_hook }).is_null() {
                    db_wal_handler
                } else {
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32)
                                    -> i32>(0 as *const ())
                    }
                }), p_db_1 as *mut ())
    };
}

///* Implementation of the "db format" command.
///*
///* Based on provided options, format the results of the SQL statement(s)
///* provided into human-readable form using the Query Result Formatter (QRF)
///* and return the resuling text.
///*
///* Syntax:    db format OPTIONS SQL
///*
///* OPTIONS may be:
///*
///*     -style ("auto"|"box"|"column"|...)      Output style
///*     -esc ("auto"|"off"|"ascii"|"symbol")    How to deal with ctrl chars
///*     -text ("auto"|"off"|"sql"|"csv"|...)    How to escape TEXT values
///*     -title ("auto"|"off"|"sql"|...|"off")   How to escape column names
///*     -blob ("auto"|"text"|"sql"|...)         How to escape BLOB values
///*     -wordwrap ("auto"|"off"|"on")           Try to wrap at word boundry?
///*     -textjsonb ("auto"|"off"|"on")          Auto-convert JSONB to text?
///*     -splitcolumn ("auto"|"off"|"on")        Enable split-column mode
///*     -defaultalign ("auto"|"left"|...)       Default alignment
///*     -titalalign ("auto"|"left"|"right"|...) Default column name alignment
///*     -border ("auto"|"off"|"on")             Border for box and table styles
///*     -wrap NUMBER                            Max width of any single column
///*     -screenwidth NUMBER                     Width of the display TTY
///*     -linelimit NUMBER                       Max lines for any cell
///*     -charlimit NUMBER                       Content truncated to this size
///*     -titlelimit NUMBER                      Max width of column titles
///*     -multiinsert NUMBER                     Multi-row INSERT byte size
///*     -align LIST-OF-ALIGNMENT                Alignment of columns
///*     -widths LIST-OF-NUMBERS                 Widths for individual columns
///*     -columnsep TEXT                         Column separator text
///*     -rowsep TEXT                            Row separator text
///*     -tablename TEXT                         Table name for style "insert"
///*     -null TEXT                              Text for NULL values
///*
///* A mapping from TCL "format" command options to sqlite3_qrf_spec fields
///* is below.  Use this to reference the QRF documentation:
///*
///*     TCL Option        spec field
///*     ----------        ----------
///*     -style            eStyle
///*     -esc              eEsc
///*     -text             eText
///*     -title            eTitle, bTitle
///*     -blob             eBlob
///*     -wordwrap         bWordWrap
///*     -textjsonb        bTextJsonb
///*     -splitcolumn      bSplitColumn
///*     -defaultalign     eDfltAlign
///*     -titlealign       eTitleAlign
///*     -border           bBorder
///*     -wrap             nWrap
///*     -screenwidth      nScreenWidth
///*     -linelimit        nLineLimit
///*     -charlimit        nCharLimit
///*     -titlelimit       nTitleLimit
///*     -multiinsert      nMultiInsert
///*     -align            nAlign, aAlign
///*     -widths           nWidth, aWidth
///*     -columnsep        zColumnSep
///*     -rowsep           zRowSep
///*     -tablename        zTableName
///*     -null             zNull
extern "C" fn db_qrf(p_db_1: &SqliteDb, objc: i32, objv: *const *mut TclObj)
    -> i32 {
    unsafe {
        Tcl_SetResult((*p_db_1).interp,
            c"QRF not available in this build".as_ptr() as *mut i8,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
            })
    };
    return 1;
}

///* The "sqlite" command below creates a new Tcl command for each
///* connection it opens to an SQLite database.  This routine is invoked
///* whenever one of those connection-specific commands is executed
///* in Tcl.  For example, if you run Tcl code like this:
///*
///*       sqlite3 db1  "my_database"
///*       db1 close
///*
///* The first command opens a connection to the "my_database" database
///* and calls that connection "db1".  The second command causes this
///* subroutine to be invoked.
#[allow(unused_doc_comments)]
extern "C" fn db_obj_cmd(mut cd: *mut (), interp: *mut TclInterp,
    mut objc: i32, mut objv: *const *mut TclObj) -> i32 {
    unsafe {
        let mut p_db: *mut SqliteDb = core::ptr::null_mut();
        let mut choice: i32 = 0;
        let mut rc: i32 = 0;
        /// don't leave trailing commas on DB_enum, it confuses the AIX xlc compiler
        ///    $db authorizer ?CALLBACK?
        ///*
        ///* Invoke the given callback to authorize each SQL operation as it is
        ///* compiled.  5 arguments are appended to the callback before it is
        ///* invoked:
        ///*
        ///*   (1) The authorization type (ex: SQLITE_CREATE_TABLE, SQLITE_INSERT, ...)
        ///*   (2) First descriptive name (depends on authorization type)
        ///*   (3) Second descriptive name
        ///*   (4) Name of the database (ex: "main", "temp")
        ///*   (5) Name of trigger that is doing the access
        ///*
        ///* The callback should return one of the following strings: SQLITE_OK,
        ///* SQLITE_IGNORE, or SQLITE_DENY.  Any other return value is an error.
        ///*
        ///* If this method is invoked with no arguments, the current authorization
        ///* callback string is returned.
        let mut z_auth: *const i8 = core::ptr::null();
        let mut len: TclSize = 0;
        ///    $db backup ?DATABASE? FILENAME
        ///*
        ///* Open or create a database file named FILENAME.  Transfer the
        ///* content of local database DATABASE (default: "main") into the
        ///* FILENAME database.
        let mut z_dest_file: *const i8 = core::ptr::null();
        let mut z_src_db: *const i8 = core::ptr::null();
        let mut p_dest: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_backup: *mut Sqlite3Backup = core::ptr::null_mut();
        ///    $db bind_fallback ?CALLBACK?
        ///*
        ///* When resolving bind parameters in an SQL statement, if the parameter
        ///* cannot be associated with a TCL variable then invoke CALLBACK with a
        ///* single argument that is the name of the parameter and use the return
        ///* value of the CALLBACK as the binding.  If CALLBACK returns something
        ///* other than TCL_OK or TCL_ERROR then bind a NULL.
        ///*
        ///* If CALLBACK is an empty string, then revert to the default behavior 
        ///* which is to set the binding to NULL.
        ///*
        ///* If CALLBACK returns an error, that causes the statement execution to
        ///* abort.  Hence, to configure a connection so that it throws an error
        ///* on an attempt to bind an unknown variable, do something like this:
        ///*
        ///*     proc bind_error {name} {error "no such variable: $name"}
        ///*     db bind_fallback bind_error
        let mut z_callback: *const i8 = core::ptr::null();
        let mut len__1: TclSize = 0;
        ///    $db busy ?CALLBACK?
        ///*
        ///* Invoke the given callback if an SQL statement attempts to open
        ///* a locked database file.
        let mut z_busy: *const i8 = core::ptr::null();
        let mut len__2: TclSize = 0;
        ///     $db cache flush
        ///*     $db cache size n
        ///*
        ///* Flush the prepared statement cache, or set the maximum number of
        ///* cached statements.
        let mut sub_cmd: *const i8 = core::ptr::null();
        let mut n: i32 = 0;
        ///     $db changes
        ///*
        ///* Return the number of rows that were modified, inserted, or deleted by
        ///* the most recent INSERT, UPDATE or DELETE statement, not including
        ///* any changes made by trigger programs.
        let mut p_result: *mut TclObj = core::ptr::null_mut();
        ///    $db close
        ///*
        ///* Shutdown the database
        ///*     $db collate NAME SCRIPT
        ///*
        ///* Create a new SQL collation function called NAME.  Whenever
        ///* that function is called, invoke SCRIPT to evaluate the function.
        let mut p_collate: *mut SqlCollate = core::ptr::null_mut();
        let mut z_name: *const i8 = core::ptr::null();
        let mut z_script: *const i8 = core::ptr::null();
        let mut n_script: TclSize = 0;
        ///*     $db collation_needed SCRIPT
        ///*
        ///* Create a new SQL collation function called NAME.  Whenever
        ///* that function is called, invoke SCRIPT to evaluate the function.
        ///    $db commit_hook ?CALLBACK?
        ///*
        ///* Invoke the given callback just before committing every SQL transaction.
        ///* If the callback throws an exception or returns non-zero, then the
        ///* transaction is aborted.  If CALLBACK is an empty string, the callback
        ///* is disabled.
        let mut z_commit: *const i8 = core::ptr::null();
        let mut len__3: TclSize = 0;
        ///    $db complete SQL
        ///*
        ///* Return TRUE if SQL is a complete SQL statement.  Return FALSE if
        ///* additional lines of input are needed.  This is similar to the
        ///* built-in "info complete" command of Tcl.
        let mut p_result_1: *mut TclObj = core::ptr::null_mut();
        let mut is_complete: i32 = 0;
        let mut p_result_2: *mut TclObj = core::ptr::null_mut();
        let mut ii: i32 = 0;
        /// With no arguments, list all configuration options and with the
        ///* current value
        let mut v: i32 = 0;
        let mut z_opt: *const i8 = core::ptr::null();
        let mut onoff: i32 = 0;
        let mut v__1: i32 = 0;
        ///    $db copy conflict-algorithm table filename ?SEPARATOR? ?NULLINDICATOR?
        ///*
        ///* Copy data into table from filename, optionally using SEPARATOR
        ///* as column separators.  If a column contains a null string, or the
        ///* value of NULLINDICATOR, a NULL is inserted for the column.
        ///* conflict-algorithm is one of the sqlite conflict algorithms:
        ///*    rollback, abort, fail, ignore, replace
        ///* On success, return the number of lines processed, not necessarily same
        ///* as 'db changes' due to conflict-algorithm selected.
        ///*
        ///* This code is basically an implementation/enhancement of
        ///* the sqlite3 shell.c ".import" command.
        ///*
        ///* This command usage is equivalent to the sqlite2.x COPY statement,
        ///* which imports file data into a table using the PostgreSQL COPY file format:
        ///*   $db copy $conflict_algorithm $table_name $filename \t \\N
        let mut z_table: *mut i8 = core::ptr::null_mut();
        /// Insert data into this table
        let mut z_file: *mut i8 = core::ptr::null_mut();
        /// The file from which to extract data
        let mut z_conflict: *mut i8 = core::ptr::null_mut();
        /// The conflict algorithm to use
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        /// A statement
        let mut n_col: i32 = 0;
        /// Number of columns in the table
        let mut n_byte: i32 = 0;
        /// Number of bytes in an SQL string
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        /// Loop counters
        let mut n_sep: i32 = 0;
        /// Number of bytes in zSep[]
        let mut n_null: i32 = 0;
        /// Number of bytes in zNull[]
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        /// An SQL statement
        let mut z_line: *mut i8 = core::ptr::null_mut();
        /// A single line of input from the file
        let mut az_col: *mut *mut i8 = core::ptr::null_mut();
        /// zLine[] broken up into columns
        let mut z_commit_1: *const i8 = core::ptr::null();
        /// How to commit changes
        let mut in_: TclChannel = core::ptr::null_mut();
        /// The input file
        let mut lineno: i32 = 0;
        /// Line number of input file
        let mut z_line_num: [i8; 80] = [0; 80];
        /// Line number print buffer
        let mut str: *mut TclObj = core::ptr::null_mut();
        let mut p_result_3: *mut TclObj = core::ptr::null_mut();
        /// interp result
        let mut z_sep: *const i8 = core::ptr::null();
        let mut z_null: *const i8 = core::ptr::null();
        let mut z: *mut i8 = core::ptr::null_mut();
        let mut byte_len: TclSize = 0;
        let mut z_err: *mut i8 = core::ptr::null_mut();
        let mut n_err: i32 = 0;
        /// check for null data, if so, bind as null
        /// success, set result as number of lines processed
        /// failure, append lineno where failed
        ///*     $db deserialize ?-maxsize N? ?-readonly BOOL? ?DATABASE? VALUE
        ///*
        ///* Reopen DATABASE (default "main") using the content in $VALUE
        let mut z_schema: *const i8 = core::ptr::null();
        let mut p_value: *mut TclObj = core::ptr::null_mut();
        let mut p_ba: *const u8 = core::ptr::null();
        let mut p_data: *mut u8 = core::ptr::null_mut();
        let mut len__4: TclSize = 0;
        let mut xrc: i32 = 0;
        let mut mx_size: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i__1: i32 = 0;
        let mut is_readonly: i32 = 0;
        let mut z__1: *const i8 = core::ptr::null();
        let mut x: TclWideInt = 0 as TclWideInt;
        let mut flags: i32 = 0;
        ///*    $db enable_load_extension BOOLEAN
        ///*
        ///* Turn the extension loading feature on or off.  It if off by
        ///* default.
        let mut onoff__1: i32 = 0;
        ///*    $db errorcode
        ///*
        ///* Return the numeric error code that was returned by the most recent
        ///* call to sqlite3_exec().
        ///*    $db erroroffset
        ///*
        ///* Return the numeric error code that was returned by the most recent
        ///* call to sqlite3_exec().
        ///*    $db exists $sql
        ///*    $db onecolumn $sql
        ///*
        ///* The onecolumn method is the equivalent of:
        ///*     lindex [$db eval $sql] 0
        let mut p_result_4: *mut TclObj = core::ptr::null_mut();
        let mut s_eval: DbEvalContext = unsafe { core::mem::zeroed() };
        ///*    $db eval ?options? $sql ?varName? ?{  ...code... }?
        ///*
        ///* The SQL statement in $sql is evaluated.  For each row, the values
        ///* are placed in elements of the array or dict named $varName and
        ///* ...code... is executed.  If $varName and $code are omitted, then
        ///* no callback is ever invoked.  If $varName is an empty string,
        ///* then the values are placed in variables that have the same name
        ///* as the fields extracted by the query, and those variables are
        ///* accessible during the eval of $code.
        let mut eval_flags: i32 = 0;
        let mut z_opt_1: *const i8 = core::ptr::null();
        let mut s_eval_1: DbEvalContext = unsafe { core::mem::zeroed() };
        let mut p_ret: *mut TclObj = core::ptr::null_mut();
        let mut i__2: i32 = 0;
        let mut n_col_1: i32 = 0;
        let mut cd2: [*mut (); 2] = [core::ptr::null_mut(); 2];
        let mut p: *mut DbEvalContext = core::ptr::null_mut();
        let mut p_var_name: *mut TclObj = core::ptr::null_mut();
        let mut p_script: *mut TclObj = core::ptr::null_mut();
        ///*     $db format [OPTIONS] SQL
        ///*
        ///* Run the SQL statement(s) given as the final argument.  Use the
        ///* Query Result Formatter extension of SQLite to format the output as
        ///* text and return that text.
        ///*     $db function NAME [OPTIONS] SCRIPT
        ///*
        ///* Create a new SQL function called NAME.  Whenever that function is
        ///* called, invoke SCRIPT to evaluate the function.
        ///*
        ///* Options:
        ///*         --argcount N           Function has exactly N arguments
        ///*         --deterministic        The function is pure
        ///*         --directonly           Prohibit use inside triggers and views
        ///*         --innocuous            Has no side effects or information leaks
        ///*         --returntype TYPE      Specify the return type of the function
        let mut flags__1: i32 = 0;
        let mut p_func: *mut SqlFunc = core::ptr::null_mut();
        let mut p_script_1: *mut TclObj = core::ptr::null_mut();
        let mut z_name_1: *const i8 = core::ptr::null();
        let mut n_arg: i32 = 0;
        let mut i__3: i32 = 0;
        let mut e_type: i32 = 0;
        let mut z__2: *const i8 = core::ptr::null();
        let mut n__1: i32 = 0;
        let mut az_type: [*const i8; 6] =
            [c"integer".as_ptr() as *const i8, c"real".as_ptr() as *const i8,
                    c"text".as_ptr() as *const i8,
                    c"blob".as_ptr() as *const i8, c"any".as_ptr() as *const i8,
                    core::ptr::null()];
        ///*     $db incrblob ?-readonly? ?DB? TABLE COLUMN ROWID
        let mut is_readonly_1: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut z_table_1: *const i8 = core::ptr::null();
        let mut z_column: *const i8 = core::ptr::null();
        let mut i_row: TclWideInt = 0 as TclWideInt;
        /// Check for the -readonly option
        ///*     $db interrupt
        ///*
        ///* Interrupt the execution of the inner-most SQL interpreter.  This
        ///* causes the SQL statement to return an error of SQLITE_INTERRUPT.
        ///*     $db nullvalue ?STRING?
        ///*
        ///* Change text used when a NULL comes back from the database. If ?STRING?
        ///* is not present, then the current string used for NULL is returned.
        ///* If STRING is present, then STRING is returned.
        ///*
        let mut len__5: TclSize = 0;
        let mut z_null_1: *const i8 = core::ptr::null();
        ///*     $db last_insert_rowid
        ///*
        ///* Return an integer which is the ROWID for the most recent insert.
        let mut p_result_5: *mut TclObj = core::ptr::null_mut();
        let mut rowid: TclWideInt = 0 as TclWideInt;
        ///* The DB_ONECOLUMN method is implemented together with DB_EXISTS.
        ///    $db progress ?N CALLBACK?
        ///*
        ///* Invoke the given callback every N virtual machine opcodes while executing
        ///* queries.
        let mut z_progress: *const i8 = core::ptr::null();
        let mut len__6: TclSize = 0;
        let mut n__2: i32 = 0;
        ///    $db profile ?CALLBACK?
        ///*
        ///* Make arrangements to invoke the CALLBACK routine after each SQL statement
        ///* that has run.  The text of the SQL and the amount of elapse time are
        ///* appended to CALLBACK before the script is run.
        let mut z_profile: *const i8 = core::ptr::null();
        let mut len__7: TclSize = 0;
        ///*     $db rekey KEY
        ///*
        ///* Change the encryption key on the currently open database.
        ///    $db restore ?DATABASE? FILENAME
        ///*
        ///* Open a database file named FILENAME.  Transfer the content
        ///* of FILENAME into the local database DATABASE (default: "main").
        let mut z_src_file: *const i8 = core::ptr::null();
        let mut z_dest_db: *const i8 = core::ptr::null();
        let mut p_src: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_backup_1: *mut Sqlite3Backup = core::ptr::null_mut();
        let mut n_timeout: i32 = 0;
        ///*     $db serialize ?DATABASE?
        ///*
        ///* Return a serialization of a database.
        let mut z_schema_1: *const i8 = core::ptr::null();
        let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut p_data_1: *mut u8 = core::ptr::null_mut();
        let mut need_free: i32 = 0;
        ///*     $db status (step|sort|autoindex|vmstep)
        ///*
        ///* Display SQLITE_STMTSTATUS_FULLSCAN_STEP or
        ///* SQLITE_STMTSTATUS_SORT for the most recent eval.
        let mut v__2: i32 = 0;
        let mut z_op: *const i8 = core::ptr::null();
        ///*     $db timeout MILLESECONDS
        ///*
        ///* Delay for the number of milliseconds specified when a file is locked.
        let mut ms: i32 = 0;
        ///*     $db total_changes
        ///*
        ///* Return the number of rows that were modified, inserted, or deleted
        ///* since the database handle was created.
        let mut p_result_6: *mut TclObj = core::ptr::null_mut();
        ///    $db trace ?CALLBACK?
        ///*
        ///* Make arrangements to invoke the CALLBACK routine for each SQL statement
        ///* that is executed.  The text of the SQL is appended to CALLBACK before
        ///* it is executed.
        let mut z_trace: *const i8 = core::ptr::null();
        let mut len__8: TclSize = 0;
        ///    $db trace_v2 ?CALLBACK? ?MASK?
        ///*
        ///* Make arrangements to invoke the CALLBACK routine for each trace event
        ///* matching the mask that is generated.  The parameters are appended to
        ///* CALLBACK before it is executed.
        let mut z_trace_v2: *const i8 = core::ptr::null();
        let mut len__9: TclSize = 0;
        let mut w_mask: TclWideInt = 0 as TclWideInt;
        let mut i__4: TclSize = 0;
        let mut p_obj: *mut TclObj = core::ptr::null_mut();
        let mut ttype: i32 = 0;
        let mut w_type: TclWideInt = 0 as TclWideInt;
        let mut p_error: *mut TclObj = core::ptr::null_mut();
        /// use the "legacy" default
        ///    $db transaction [-deferred|-immediate|-exclusive] SCRIPT
        ///*
        ///* Start a new transaction (if we are not already in the midst of a
        ///* transaction) and execute the TCL script SCRIPT.  After SCRIPT
        ///* completes, either commit the transaction or roll it back if SCRIPT
        ///* throws an exception.  Or if no new transaction was started, do nothing.
        ///* pass the exception on up the stack.
        ///*
        ///* This command was inspired by Dave Thomas's talk on Ruby at the
        ///* 2005 O'Reilly Open Source Convention (OSCON).
        let mut p_script_2: *mut TclObj = core::ptr::null_mut();
        let mut z_begin: *const i8 = core::ptr::null();
        let mut ttype__1: i32 = 0;
        /// no-op
        /// Run the SQLite BEGIN command to open a transaction or savepoint.
        /// If using NRE, schedule a callback to invoke the script pScript, then
        ///* a second callback to commit (or rollback) the transaction or savepoint
        ///* opened above. If not using NRE, evaluate the script directly, then
        ///* call function DbTransPostCmd() to commit (or rollback) the transaction
        ///* or savepoint.
        /// DbTransPostCmd() calls delDatabaseRef()
        ///*    $db unlock_notify ?script?
        ///*    $db preupdate_hook count
        ///*    $db preupdate_hook hook ?SCRIPT?
        ///*    $db preupdate_hook new INDEX
        ///*    $db preupdate_hook old INDEX
        /// SQLITE_ENABLE_PREUPDATE_HOOK
        ///*    $db wal_hook ?script?
        ///*    $db update_hook ?script?
        ///*    $db rollback_hook ?script?
        /// set ppHook to point at pUpdateHook or pRollbackHook, depending on
        ///* whether [$db update_hook] or [$db rollback_hook] was invoked.
        let mut pp_hook: *mut *mut TclObj = core::ptr::null_mut();
        ///    $db version
        ///*
        ///* Return the version string for this database.
        let mut i__5: i32 = 0;
        let mut z_arg: *const i8 = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s55:
                {
                match __state {
                    0 => { p_db = cd as *mut SqliteDb; __state = 3; }
                    2 => { __state = 13; }
                    3 => { __state = 4; }
                    4 => { rc = 0; __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { if objc < 2 { __state = 9; } else { __state = 8; } }
                    8 => {
                        if unsafe {
                                    Tcl_GetIndexFromObj(interp,
                                        unsafe { *objv.offset(1 as isize) },
                                        &raw mut db_strs[0 as usize] as *mut *const i8,
                                        c"option".as_ptr() as *mut i8 as *const i8, 0, &mut choice)
                                } != 0 {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    9 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 1, objv,
                                c"SUBCOMMAND ...".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 10;
                    }
                    10 => { return 1; }
                    11 => {
                        '__s56:
                            {
                            match choice as u32 {
                                DB_AUTHORIZER => { __state = 14; }
                                DB_BACKUP => { __state = 15; }
                                DB_BIND_FALLBACK => { __state = 16; }
                                DB_BUSY => { __state = 17; }
                                DB_CACHE => { __state = 18; }
                                DB_CHANGES => { __state = 19; }
                                DB_CLOSE => { __state = 20; }
                                DB_COLLATE => { __state = 21; }
                                DB_COLLATION_NEEDED => { __state = 22; }
                                DB_COMMIT_HOOK => { __state = 23; }
                                DB_COMPLETE => { __state = 24; }
                                DB_CONFIG => { __state = 25; }
                                DB_COPY => { __state = 26; }
                                DB_DESERIALIZE => { __state = 27; }
                                DB_ENABLE_LOAD_EXTENSION => { __state = 28; }
                                DB_ERRORCODE => { __state = 29; }
                                DB_ERROROFFSET => { __state = 30; }
                                DB_EXISTS => { __state = 31; }
                                DB_ONECOLUMN => { __state = 32; }
                                DB_EVAL => { __state = 33; }
                                DB_FORMAT => { __state = 34; }
                                DB_FUNCTION => { __state = 35; }
                                DB_INCRBLOB => { __state = 36; }
                                DB_INTERRUPT => { __state = 37; }
                                DB_NULLVALUE => { __state = 38; }
                                DB_LAST_INSERT_ROWID => { __state = 39; }
                                DB_PROGRESS => { __state = 40; }
                                DB_PROFILE => { __state = 41; }
                                DB_REKEY => { __state = 42; }
                                DB_RESTORE => { __state = 43; }
                                DB_SERIALIZE => { __state = 44; }
                                DB_STATUS => { __state = 45; }
                                DB_TIMEOUT => { __state = 46; }
                                DB_TOTAL_CHANGES => { __state = 47; }
                                DB_TRACE => { __state = 48; }
                                DB_TRACE_V2 => { __state = 49; }
                                DB_TRANSACTION => { __state = 50; }
                                DB_UNLOCK_NOTIFY => { __state = 51; }
                                DB_PREUPDATE => { __state = 52; }
                                DB_WAL_HOOK => { __state = 53; }
                                DB_UPDATE_HOOK => { __state = 54; }
                                DB_ROLLBACK_HOOK => { __state = 55; }
                                DB_VERSION => { __state = 56; }
                                _ => { __state = 13; }
                            }
                        }
                    }
                    12 => { return 1; }
                    13 => { return rc; }
                    14 => {
                        if objc > 3 { __state = 60; } else { __state = 61; }
                    }
                    15 => { __state = 80; }
                    16 => {
                        if objc > 3 { __state = 112; } else { __state = 113; }
                    }
                    17 => {
                        if objc > 3 { __state = 128; } else { __state = 129; }
                    }
                    18 => { __state = 147; }
                    19 => { __state = 174; }
                    20 => {
                        unsafe {
                            Tcl_DeleteCommand(interp,
                                unsafe {
                                        Tcl_GetStringFromObj(unsafe { *objv.offset(0 as isize) },
                                            core::ptr::null_mut())
                                    } as *const i8)
                        };
                        __state = 181;
                    }
                    21 => { __state = 183; }
                    22 => {
                        if objc != 3 { __state = 205; } else { __state = 204; }
                    }
                    23 => {
                        if objc > 3 { __state = 216; } else { __state = 217; }
                    }
                    24 => { __state = 235; }
                    25 => { __state = 244; }
                    26 => { __state = 279; }
                    27 => { z_schema = core::ptr::null(); __state = 419; }
                    28 => { __state = 474; }
                    29 => {
                        unsafe {
                            Tcl_SetObjResult(interp,
                                unsafe {
                                    Tcl_NewIntObj(unsafe {
                                            sqlite3_errcode(unsafe { (*p_db).db })
                                        })
                                })
                        };
                        __state = 482;
                    }
                    30 => {
                        unsafe {
                            Tcl_SetObjResult(interp,
                                unsafe {
                                    Tcl_NewIntObj(unsafe {
                                            sqlite3_error_offset(unsafe { (*p_db).db })
                                        })
                                })
                        };
                        __state = 484;
                    }
                    31 => { __state = 32; }
                    32 => { p_result_4 = core::ptr::null_mut(); __state = 486; }
                    33 => { eval_flags = 0; __state = 507; }
                    34 => {
                        rc = db_qrf(unsafe { &*p_db }, objc, objv);
                        __state = 555;
                    }
                    35 => { flags__1 = 1; __state = 557; }
                    36 => { is_readonly_1 = 0; __state = 619; }
                    37 => {
                        unsafe { sqlite3_interrupt(unsafe { (*p_db).db }) };
                        __state = 637;
                    }
                    38 => {
                        if objc != 2 && objc != 3 {
                            __state = 640;
                        } else { __state = 639; }
                    }
                    39 => { __state = 654; }
                    40 => {
                        if objc == 2 { __state = 664; } else { __state = 665; }
                    }
                    41 => {
                        if objc > 3 { __state = 689; } else { __state = 690; }
                    }
                    42 => {
                        if objc != 3 { __state = 709; } else { __state = 708; }
                    }
                    43 => { __state = 712; }
                    44 => {
                        z_schema_1 =
                            if objc >= 3 {
                                    unsafe {
                                        Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                    }
                                } else { c"main".as_ptr() as *mut i8 } as *const i8;
                        __state = 750;
                    }
                    45 => { __state = 766; }
                    46 => { __state = 784; }
                    47 => { __state = 792; }
                    48 => {
                        if objc > 3 { __state = 800; } else { __state = 801; }
                    }
                    49 => {
                        if objc > 4 { __state = 820; } else { __state = 821; }
                    }
                    50 => { __state = 881; }
                    51 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"unlock_notify not available in this build".as_ptr() as
                                    *mut i8, 0 as *mut i8)
                        };
                        __state = 916;
                    }
                    52 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"preupdate_hook was omitted at compile-time".as_ptr() as
                                    *mut i8, 0 as *mut i8)
                        };
                        __state = 919;
                    }
                    53 => { __state = 54; }
                    54 => { __state = 55; }
                    55 => { pp_hook = core::ptr::null_mut(); __state = 922; }
                    56 => { __state = 934; }
                    57 => { __state = 14; }
                    58 => { __state = 15; }
                    59 => { __state = 13; }
                    60 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?CALLBACK?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 62;
                    }
                    61 => {
                        if objc == 2 { __state = 63; } else { __state = 64; }
                    }
                    62 => { return 1; }
                    63 => {
                        if !(unsafe { (*p_db).z_auth }).is_null() {
                            __state = 65;
                        } else { __state = 59; }
                    }
                    64 => { __state = 66; }
                    65 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_auth },
                                0 as *mut i8)
                        };
                        __state = 59;
                    }
                    66 => { __state = 67; }
                    67 => {
                        if !(unsafe { (*p_db).z_auth }).is_null() {
                            __state = 69;
                        } else { __state = 68; }
                    }
                    68 => {
                        z_auth =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    &mut len)
                            };
                        __state = 70;
                    }
                    69 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_auth }) };
                        __state = 68;
                    }
                    70 => {
                        if !(z_auth).is_null() && len > 0 {
                            __state = 72;
                        } else { __state = 73; }
                    }
                    71 => {
                        if !(unsafe { (*p_db).z_auth }).is_null() {
                            __state = 75;
                        } else { __state = 76; }
                    }
                    72 => {
                        unsafe {
                            (*p_db).z_auth = unsafe { Tcl_Alloc((len + 1) as u32) }
                        };
                        __state = 74;
                    }
                    73 => {
                        unsafe { (*p_db).z_auth = core::ptr::null_mut() };
                        __state = 71;
                    }
                    74 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_auth } as *mut (),
                                z_auth as *const (), (len + 1) as u64)
                        };
                        __state = 71;
                    }
                    75 => { __state = 77; }
                    76 => {
                        unsafe {
                            sqlite3_set_authorizer(unsafe { (*p_db).db }, None,
                                core::ptr::null_mut())
                        };
                        __state = 59;
                    }
                    77 => { unsafe { (*p_db).interp = interp }; __state = 78; }
                    78 => {
                        unsafe {
                            sqlite3_set_authorizer(unsafe { (*p_db).db },
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                                    *const i8, *const i8) -> i32>(auth_callback as *const ())
                                    }), p_db as *mut ())
                        };
                        __state = 59;
                    }
                    79 => { __state = 16; }
                    80 => { __state = 81; }
                    81 => { __state = 82; }
                    82 => { __state = 83; }
                    83 => {
                        if objc == 3 { __state = 85; } else { __state = 86; }
                    }
                    84 => {
                        rc =
                            unsafe {
                                sqlite3_open_v2(z_dest_file, &mut p_dest,
                                    2 | 4 | unsafe { (*p_db).open_flags }, core::ptr::null())
                            };
                        __state = 92;
                    }
                    85 => {
                        z_src_db = c"main".as_ptr() as *mut i8 as *const i8;
                        __state = 87;
                    }
                    86 => {
                        if objc == 4 { __state = 88; } else { __state = 89; }
                    }
                    87 => {
                        z_dest_file =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                } as *const i8;
                        __state = 84;
                    }
                    88 => {
                        z_src_db =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                } as *const i8;
                        __state = 90;
                    }
                    89 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?DATABASE? FILENAME".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 91;
                    }
                    90 => {
                        z_dest_file =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(3 as isize) })
                                } as *const i8;
                        __state = 84;
                    }
                    91 => { return 1; }
                    92 => {
                        if rc != 0 { __state = 94; } else { __state = 93; }
                    }
                    93 => {
                        p_backup =
                            unsafe {
                                sqlite3_backup_init(p_dest,
                                    c"main".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_db).db }, z_src_db)
                            };
                        __state = 97;
                    }
                    94 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"cannot open target database: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(p_dest) }, 0 as *mut i8)
                        };
                        __state = 95;
                    }
                    95 => { unsafe { sqlite3_close(p_dest) }; __state = 96; }
                    96 => { return 1; }
                    97 => {
                        if p_backup == core::ptr::null_mut() {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    98 => {
                        if {
                                    rc = unsafe { sqlite3_backup_step(p_backup, 100) };
                                    rc
                                } == 0 {
                            __state = 103;
                        } else { __state = 102; }
                    }
                    99 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"backup failed: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(p_dest) }, 0 as *mut i8)
                        };
                        __state = 100;
                    }
                    100 => { unsafe { sqlite3_close(p_dest) }; __state = 101; }
                    101 => { return 1; }
                    102 => {
                        unsafe { sqlite3_backup_finish(p_backup) };
                        __state = 104;
                    }
                    103 => { __state = 98; }
                    104 => {
                        if rc == 101 { __state = 106; } else { __state = 107; }
                    }
                    105 => { unsafe { sqlite3_close(p_dest) }; __state = 109; }
                    106 => { rc = 0; __state = 105; }
                    107 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"backup failed: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(p_dest) }, 0 as *mut i8)
                        };
                        __state = 108;
                    }
                    108 => { rc = 1; __state = 105; }
                    109 => { __state = 13; }
                    110 => { __state = 17; }
                    111 => { __state = 13; }
                    112 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?CALLBACK?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 114;
                    }
                    113 => {
                        if objc == 2 { __state = 115; } else { __state = 116; }
                    }
                    114 => { return 1; }
                    115 => {
                        if !(unsafe { (*p_db).z_bind_fallback }).is_null() {
                            __state = 117;
                        } else { __state = 111; }
                    }
                    116 => { __state = 118; }
                    117 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_bind_fallback },
                                0 as *mut i8)
                        };
                        __state = 111;
                    }
                    118 => { __state = 119; }
                    119 => {
                        if !(unsafe { (*p_db).z_bind_fallback }).is_null() {
                            __state = 121;
                        } else { __state = 120; }
                    }
                    120 => {
                        z_callback =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    &mut len__1)
                            };
                        __state = 122;
                    }
                    121 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_bind_fallback }) };
                        __state = 120;
                    }
                    122 => {
                        if !(z_callback).is_null() && len__1 > 0 {
                            __state = 123;
                        } else { __state = 124; }
                    }
                    123 => {
                        unsafe {
                            (*p_db).z_bind_fallback =
                                unsafe { Tcl_Alloc((len__1 + 1) as u32) }
                        };
                        __state = 125;
                    }
                    124 => {
                        unsafe { (*p_db).z_bind_fallback = core::ptr::null_mut() };
                        __state = 111;
                    }
                    125 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_bind_fallback } as *mut (),
                                z_callback as *const (), (len__1 + 1) as u64)
                        };
                        __state = 111;
                    }
                    126 => { __state = 18; }
                    127 => { __state = 13; }
                    128 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"CALLBACK".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 130;
                    }
                    129 => {
                        if objc == 2 { __state = 131; } else { __state = 132; }
                    }
                    130 => { return 1; }
                    131 => {
                        if !(unsafe { (*p_db).z_busy }).is_null() {
                            __state = 133;
                        } else { __state = 127; }
                    }
                    132 => { __state = 134; }
                    133 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_busy },
                                0 as *mut i8)
                        };
                        __state = 127;
                    }
                    134 => { __state = 135; }
                    135 => {
                        if !(unsafe { (*p_db).z_busy }).is_null() {
                            __state = 137;
                        } else { __state = 136; }
                    }
                    136 => {
                        z_busy =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    &mut len__2)
                            };
                        __state = 138;
                    }
                    137 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_busy }) };
                        __state = 136;
                    }
                    138 => {
                        if !(z_busy).is_null() && len__2 > 0 {
                            __state = 140;
                        } else { __state = 141; }
                    }
                    139 => {
                        if !(unsafe { (*p_db).z_busy }).is_null() {
                            __state = 143;
                        } else { __state = 144; }
                    }
                    140 => {
                        unsafe {
                            (*p_db).z_busy = unsafe { Tcl_Alloc((len__2 + 1) as u32) }
                        };
                        __state = 142;
                    }
                    141 => {
                        unsafe { (*p_db).z_busy = core::ptr::null_mut() };
                        __state = 139;
                    }
                    142 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_busy } as *mut (),
                                z_busy as *const (), (len__2 + 1) as u64)
                        };
                        __state = 139;
                    }
                    143 => {
                        unsafe { (*p_db).interp = interp };
                        __state = 145;
                    }
                    144 => {
                        unsafe {
                            sqlite3_busy_handler(unsafe { (*p_db).db }, None,
                                core::ptr::null_mut())
                        };
                        __state = 127;
                    }
                    145 => {
                        unsafe {
                            sqlite3_busy_handler(unsafe { (*p_db).db },
                                Some(db_busy_handler), p_db as *mut ())
                        };
                        __state = 127;
                    }
                    146 => { __state = 19; }
                    147 => { __state = 148; }
                    148 => {
                        if objc <= 2 { __state = 150; } else { __state = 149; }
                    }
                    149 => {
                        sub_cmd =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    core::ptr::null_mut())
                            };
                        __state = 152;
                    }
                    150 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 1, objv,
                                c"cache option ?arg?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 151;
                    }
                    151 => { return 1; }
                    152 => {
                        if unsafe { *sub_cmd } as i32 == 'f' as i32 &&
                                unsafe {
                                        strcmp(sub_cmd as *const i8,
                                            c"flush".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            __state = 154;
                        } else { __state = 155; }
                    }
                    153 => { __state = 13; }
                    154 => {
                        if objc != 3 { __state = 156; } else { __state = 157; }
                    }
                    155 => {
                        if unsafe { *sub_cmd } as i32 == 's' as i32 &&
                                unsafe {
                                        strcmp(sub_cmd as *const i8,
                                            c"size".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            __state = 159;
                        } else { __state = 160; }
                    }
                    156 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"flush".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 158;
                    }
                    157 => {
                        flush_stmt_cache(unsafe { &mut *p_db });
                        __state = 153;
                    }
                    158 => { return 1; }
                    159 => {
                        if objc != 4 { __state = 161; } else { __state = 162; }
                    }
                    160 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"bad option \"".as_ptr() as *mut i8,
                                unsafe {
                                    Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                        core::ptr::null_mut())
                                }, c"\": must be flush or size".as_ptr() as *mut i8,
                                0 as *mut i8)
                        };
                        __state = 172;
                    }
                    161 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"size n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 163;
                    }
                    162 => {
                        if 1 ==
                                unsafe {
                                    Tcl_GetIntFromObj(interp,
                                        unsafe { *objv.offset(3 as isize) }, &mut n)
                                } {
                            __state = 164;
                        } else { __state = 165; }
                    }
                    163 => { return 1; }
                    164 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"cannot convert \"".as_ptr() as *mut i8,
                                unsafe {
                                    Tcl_GetStringFromObj(unsafe { *objv.offset(3 as isize) },
                                        core::ptr::null_mut())
                                }, c"\" to integer".as_ptr() as *mut i8, 0 as *mut i8)
                        };
                        __state = 166;
                    }
                    165 => {
                        if n < 0 { __state = 168; } else { __state = 169; }
                    }
                    166 => { return 1; }
                    167 => { unsafe { (*p_db).max_stmt = n }; __state = 153; }
                    168 => {
                        flush_stmt_cache(unsafe { &mut *p_db });
                        __state = 170;
                    }
                    169 => {
                        if n > 100 { __state = 171; } else { __state = 167; }
                    }
                    170 => { n = 0; __state = 167; }
                    171 => { n = 100; __state = 167; }
                    172 => { return 1; }
                    173 => { __state = 20; }
                    174 => {
                        if objc != 2 { __state = 176; } else { __state = 175; }
                    }
                    175 => {
                        p_result = unsafe { Tcl_GetObjResult(interp) };
                        __state = 178;
                    }
                    176 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 177;
                    }
                    177 => { return 1; }
                    178 => {
                        unsafe {
                            Tcl_SetWideIntObj(p_result,
                                unsafe { sqlite3_changes64(unsafe { (*p_db).db }) })
                        };
                        __state = 179;
                    }
                    179 => { __state = 13; }
                    180 => { __state = 21; }
                    181 => { __state = 13; }
                    182 => { __state = 22; }
                    183 => { __state = 184; }
                    184 => { __state = 185; }
                    185 => { __state = 186; }
                    186 => {
                        if objc != 4 { __state = 188; } else { __state = 187; }
                    }
                    187 => {
                        z_name =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    core::ptr::null_mut())
                            };
                        __state = 190;
                    }
                    188 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"NAME SCRIPT".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 189;
                    }
                    189 => { return 1; }
                    190 => {
                        z_script =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(3 as isize) },
                                    &mut n_script)
                            };
                        __state = 191;
                    }
                    191 => {
                        p_collate =
                            unsafe {
                                    Tcl_Alloc((core::mem::size_of::<SqlCollate>() as u64 +
                                                    n_script as u64 + 1 as u64) as u32)
                                } as *mut SqlCollate;
                        __state = 192;
                    }
                    192 => {
                        if p_collate == core::ptr::null_mut() {
                            __state = 194;
                        } else { __state = 193; }
                    }
                    193 => {
                        unsafe { (*p_collate).interp = interp };
                        __state = 195;
                    }
                    194 => { return 1; }
                    195 => {
                        unsafe {
                            (*p_collate).p_next = unsafe { (*p_db).p_collate }
                        };
                        __state = 196;
                    }
                    196 => {
                        unsafe {
                            (*p_collate).z_script =
                                unsafe { &raw mut *p_collate.offset(1 as isize) } as *mut i8
                        };
                        __state = 197;
                    }
                    197 => {
                        unsafe { (*p_db).p_collate = p_collate };
                        __state = 198;
                    }
                    198 => {
                        unsafe {
                            memcpy(unsafe { (*p_collate).z_script } as *mut (),
                                z_script as *const (), (n_script + 1) as u64)
                        };
                        __state = 199;
                    }
                    199 => {
                        if unsafe {
                                    sqlite3_create_collation(unsafe { (*p_db).db },
                                        z_name as *const i8, 1, p_collate as *mut (),
                                        Some(tcl_sql_collate))
                                } != 0 {
                            __state = 201;
                        } else { __state = 200; }
                    }
                    200 => { __state = 13; }
                    201 => {
                        unsafe {
                            Tcl_SetResult(interp,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) } as *mut i8,
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
                                })
                        };
                        __state = 202;
                    }
                    202 => { return 1; }
                    203 => { __state = 23; }
                    204 => {
                        if !(unsafe { (*p_db).p_collate_needed }).is_null() {
                            __state = 208;
                        } else { __state = 207; }
                    }
                    205 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"SCRIPT".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 206;
                    }
                    206 => { return 1; }
                    207 => {
                        unsafe {
                            (*p_db).p_collate_needed =
                                unsafe {
                                    Tcl_DuplicateObj(unsafe { *objv.offset(2 as isize) })
                                }
                        };
                        __state = 211;
                    }
                    208 => {
                        if {
                                    let __p =
                                        unsafe {
                                            &mut (*unsafe { (*p_db).p_collate_needed }).refCount
                                        };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            __state = 210;
                        } else { __state = 209; }
                    }
                    209 => {
                        if 0 != 0 { __state = 208; } else { __state = 207; }
                    }
                    210 => {
                        unsafe { TclFreeObj(unsafe { (*p_db).p_collate_needed }) };
                        __state = 209;
                    }
                    211 => {
                        {
                            let __p =
                                unsafe {
                                    &mut (*unsafe { (*p_db).p_collate_needed }).refCount
                                };
                            *__p += 1;
                            *__p
                        };
                        __state = 212;
                    }
                    212 => {
                        unsafe {
                            sqlite3_collation_needed(unsafe { (*p_db).db },
                                p_db as *mut (), Some(tcl_collate_needed))
                        };
                        __state = 213;
                    }
                    213 => { __state = 13; }
                    214 => { __state = 24; }
                    215 => { __state = 13; }
                    216 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?CALLBACK?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 218;
                    }
                    217 => {
                        if objc == 2 { __state = 219; } else { __state = 220; }
                    }
                    218 => { return 1; }
                    219 => {
                        if !(unsafe { (*p_db).z_commit }).is_null() {
                            __state = 221;
                        } else { __state = 215; }
                    }
                    220 => { __state = 222; }
                    221 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_commit },
                                0 as *mut i8)
                        };
                        __state = 215;
                    }
                    222 => { __state = 223; }
                    223 => {
                        if !(unsafe { (*p_db).z_commit }).is_null() {
                            __state = 225;
                        } else { __state = 224; }
                    }
                    224 => {
                        z_commit =
                            unsafe {
                                    Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                        &mut len__3)
                                } as *const i8;
                        __state = 226;
                    }
                    225 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_commit }) };
                        __state = 224;
                    }
                    226 => {
                        if !(z_commit).is_null() && len__3 > 0 {
                            __state = 228;
                        } else { __state = 229; }
                    }
                    227 => {
                        if !(unsafe { (*p_db).z_commit }).is_null() {
                            __state = 231;
                        } else { __state = 232; }
                    }
                    228 => {
                        unsafe {
                            (*p_db).z_commit = unsafe { Tcl_Alloc((len__3 + 1) as u32) }
                        };
                        __state = 230;
                    }
                    229 => {
                        unsafe { (*p_db).z_commit = core::ptr::null_mut() };
                        __state = 227;
                    }
                    230 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_commit } as *mut (),
                                z_commit as *const (), (len__3 + 1) as u64)
                        };
                        __state = 227;
                    }
                    231 => {
                        unsafe { (*p_db).interp = interp };
                        __state = 233;
                    }
                    232 => {
                        unsafe {
                            sqlite3_commit_hook(unsafe { (*p_db).db }, None,
                                core::ptr::null_mut())
                        };
                        __state = 215;
                    }
                    233 => {
                        unsafe {
                            sqlite3_commit_hook(unsafe { (*p_db).db },
                                Some(db_commit_handler), p_db as *mut ())
                        };
                        __state = 215;
                    }
                    234 => { __state = 25; }
                    235 => { __state = 236; }
                    236 => {
                        if objc != 3 { __state = 238; } else { __state = 237; }
                    }
                    237 => {
                        is_complete =
                            unsafe {
                                sqlite3_complete(unsafe {
                                            Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                                core::ptr::null_mut())
                                        } as *const i8)
                            };
                        __state = 240;
                    }
                    238 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"SQL".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 239;
                    }
                    239 => { return 1; }
                    240 => {
                        p_result_1 = unsafe { Tcl_GetObjResult(interp) };
                        __state = 241;
                    }
                    241 => {
                        unsafe { Tcl_SetBooleanObj(p_result_1, is_complete) };
                        __state = 242;
                    }
                    242 => { __state = 13; }
                    243 => { __state = 26; }
                    244 => { __state = 245; }
                    245 => { __state = 246; }
                    246 => {
                        if objc > 4 { __state = 248; } else { __state = 247; }
                    }
                    247 => {
                        if objc == 2 { __state = 251; } else { __state = 252; }
                    }
                    248 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?OPTION? ?BOOLEAN?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 249;
                    }
                    249 => { return 1; }
                    250 => {
                        unsafe { Tcl_SetObjResult(interp, p_result_2) };
                        __state = 277;
                    }
                    251 => {
                        p_result_2 =
                            unsafe { Tcl_NewListObj(0, core::ptr::null()) };
                        __state = 253;
                    }
                    252 => {
                        z_opt =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                } as *const i8;
                        __state = 260;
                    }
                    253 => { ii = 0; __state = 254; }
                    254 => {
                        if (ii as u64) <
                                core::mem::size_of::<[DbConfigChoicesN15DbConfigChoices; 16]>()
                                        as u64 /
                                    core::mem::size_of::<DbConfigChoicesN15DbConfigChoices>() as
                                        u64 {
                            __state = 255;
                        } else { __state = 250; }
                    }
                    255 => { v = 0; __state = 257; }
                    256 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 254;
                    }
                    257 => {
                        unsafe {
                            sqlite3_db_config(unsafe { (*p_db).db },
                                a_db_config[ii as usize].op, -1, &raw mut v as *mut i32)
                        };
                        __state = 258;
                    }
                    258 => {
                        unsafe {
                            Tcl_ListObjAppendElement(interp, p_result_2,
                                unsafe {
                                    Tcl_NewStringObj(a_db_config[ii as usize].z_name, -1)
                                })
                        };
                        __state = 259;
                    }
                    259 => {
                        unsafe {
                            Tcl_ListObjAppendElement(interp, p_result_2,
                                unsafe { Tcl_NewIntObj(v) })
                        };
                        __state = 256;
                    }
                    260 => { onoff = -1; __state = 261; }
                    261 => { v__1 = 0; __state = 262; }
                    262 => {
                        if unsafe { *z_opt.offset(0 as isize) } as i32 == '-' as i32
                            {
                            __state = 264;
                        } else { __state = 263; }
                    }
                    263 => { ii = 0; __state = 266; }
                    264 => {
                        {
                            let __p = &mut z_opt;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 263;
                    }
                    265 => {
                        if ii as u64 >=
                                core::mem::size_of::<[DbConfigChoicesN15DbConfigChoices; 16]>()
                                        as u64 /
                                    core::mem::size_of::<DbConfigChoicesN15DbConfigChoices>() as
                                        u64 {
                            __state = 271;
                        } else { __state = 270; }
                    }
                    266 => {
                        if (ii as u64) <
                                core::mem::size_of::<[DbConfigChoicesN15DbConfigChoices; 16]>()
                                        as u64 /
                                    core::mem::size_of::<DbConfigChoicesN15DbConfigChoices>() as
                                        u64 {
                            __state = 267;
                        } else { __state = 265; }
                    }
                    267 => {
                        if unsafe { strcmp(a_db_config[ii as usize].z_name, z_opt) }
                                == 0 {
                            __state = 269;
                        } else { __state = 268; }
                    }
                    268 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 266;
                    }
                    269 => { __state = 265; }
                    270 => {
                        if objc == 4 { __state = 274; } else { __state = 273; }
                    }
                    271 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"unknown config option: \"".as_ptr() as *mut i8, z_opt,
                                c"\"".as_ptr() as *mut i8, 0 as *mut ())
                        };
                        __state = 272;
                    }
                    272 => { return 1; }
                    273 => {
                        unsafe {
                            sqlite3_db_config(unsafe { (*p_db).db },
                                a_db_config[ii as usize].op, onoff,
                                &raw mut v__1 as *mut i32)
                        };
                        __state = 276;
                    }
                    274 => {
                        if unsafe {
                                    Tcl_GetBooleanFromObj(interp,
                                        unsafe { *objv.offset(3 as isize) }, &mut onoff)
                                } != 0 {
                            __state = 275;
                        } else { __state = 273; }
                    }
                    275 => { return 1; }
                    276 => {
                        p_result_2 = unsafe { Tcl_NewIntObj(v__1) };
                        __state = 250;
                    }
                    277 => { __state = 13; }
                    278 => { __state = 27; }
                    279 => { __state = 280; }
                    280 => { __state = 281; }
                    281 => { __state = 282; }
                    282 => { __state = 283; }
                    283 => { __state = 284; }
                    284 => { __state = 285; }
                    285 => { __state = 286; }
                    286 => { __state = 287; }
                    287 => { __state = 288; }
                    288 => { __state = 289; }
                    289 => { __state = 290; }
                    290 => { __state = 291; }
                    291 => { __state = 292; }
                    292 => { lineno = 0; __state = 293; }
                    293 => { __state = 294; }
                    294 => { __state = 295; }
                    295 => { __state = 296; }
                    296 => { __state = 297; }
                    297 => { __state = 298; }
                    298 => {
                        if objc < 5 || objc > 7 {
                            __state = 300;
                        } else { __state = 299; }
                    }
                    299 => {
                        if objc >= 6 { __state = 303; } else { __state = 304; }
                    }
                    300 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"CONFLICT-ALGORITHM TABLE FILENAME ?SEPARATOR? ?NULLINDICATOR?".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 301;
                    }
                    301 => { return 1; }
                    302 => {
                        if objc >= 7 { __state = 306; } else { __state = 307; }
                    }
                    303 => {
                        z_sep =
                            unsafe {
                                    Tcl_GetStringFromObj(unsafe { *objv.offset(5 as isize) },
                                        core::ptr::null_mut())
                                } as *const i8;
                        __state = 302;
                    }
                    304 => {
                        z_sep = c"\t".as_ptr() as *mut i8 as *const i8;
                        __state = 302;
                    }
                    305 => {
                        z_conflict =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    core::ptr::null_mut())
                            };
                        __state = 308;
                    }
                    306 => {
                        z_null =
                            unsafe {
                                    Tcl_GetStringFromObj(unsafe { *objv.offset(6 as isize) },
                                        core::ptr::null_mut())
                                } as *const i8;
                        __state = 305;
                    }
                    307 => {
                        z_null = c"".as_ptr() as *mut i8 as *const i8;
                        __state = 305;
                    }
                    308 => {
                        z_table =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(3 as isize) },
                                    core::ptr::null_mut())
                            };
                        __state = 309;
                    }
                    309 => {
                        z_file =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(4 as isize) },
                                    core::ptr::null_mut())
                            };
                        __state = 310;
                    }
                    310 => { n_sep = strlen30(z_sep); __state = 311; }
                    311 => { n_null = strlen30(z_null); __state = 312; }
                    312 => {
                        if n_sep == 0 { __state = 314; } else { __state = 313; }
                    }
                    313 => {
                        if unsafe {
                                                    strcmp(z_conflict as *const i8,
                                                        c"rollback".as_ptr() as *mut i8 as *const i8)
                                                } != 0 &&
                                            unsafe {
                                                    strcmp(z_conflict as *const i8,
                                                        c"abort".as_ptr() as *mut i8 as *const i8)
                                                } != 0 &&
                                        unsafe {
                                                strcmp(z_conflict as *const i8,
                                                    c"fail".as_ptr() as *mut i8 as *const i8)
                                            } != 0 &&
                                    unsafe {
                                            strcmp(z_conflict as *const i8,
                                                c"ignore".as_ptr() as *mut i8 as *const i8)
                                        } != 0 &&
                                unsafe {
                                        strcmp(z_conflict as *const i8,
                                            c"replace".as_ptr() as *mut i8 as *const i8)
                                    } != 0 {
                            __state = 317;
                        } else { __state = 316; }
                    }
                    314 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"Error: non-null separator required for copy".as_ptr() as
                                    *mut i8, 0 as *mut i8)
                        };
                        __state = 315;
                    }
                    315 => { return 1; }
                    316 => {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"SELECT * FROM \'%q\'".as_ptr() as *mut i8
                                        as *const i8, z_table)
                            };
                        __state = 319;
                    }
                    317 => {
                        unsafe {
                            Tcl_AppendResult(interp, c"Error: \"".as_ptr() as *mut i8,
                                z_conflict,
                                c"\", conflict-algorithm must be one of: rollback, abort, fail, ignore, or replace".as_ptr()
                                    as *mut i8, 0 as *mut i8)
                        };
                        __state = 318;
                    }
                    318 => { return 1; }
                    319 => {
                        if z_sql == core::ptr::null_mut() {
                            __state = 321;
                        } else { __state = 320; }
                    }
                    320 => {
                        n_byte = strlen30(z_sql as *const i8);
                        __state = 323;
                    }
                    321 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"Error: no such table: ".as_ptr() as *mut i8, z_table,
                                0 as *mut i8)
                        };
                        __state = 322;
                    }
                    322 => { return 1; }
                    323 => {
                        rc =
                            unsafe {
                                sqlite3_prepare(unsafe { (*p_db).db }, z_sql as *const i8,
                                    -1, &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 324;
                    }
                    324 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 325;
                    }
                    325 => {
                        if rc != 0 { __state = 327; } else { __state = 328; }
                    }
                    326 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 330;
                    }
                    327 => {
                        unsafe {
                            Tcl_AppendResult(interp, c"Error: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                                0 as *mut i8)
                        };
                        __state = 329;
                    }
                    328 => {
                        n_col = unsafe { sqlite3_column_count(p_stmt) };
                        __state = 326;
                    }
                    329 => { n_col = 0; __state = 326; }
                    330 => {
                        if n_col == 0 { __state = 332; } else { __state = 331; }
                    }
                    331 => {
                        z_sql =
                            unsafe { malloc((n_byte + 50 + n_col * 2) as u64) } as
                                *mut i8;
                        __state = 333;
                    }
                    332 => { return 1; }
                    333 => {
                        if z_sql == core::ptr::null_mut() {
                            __state = 335;
                        } else { __state = 334; }
                    }
                    334 => {
                        unsafe {
                            sqlite3_snprintf(n_byte + 50, z_sql,
                                c"INSERT OR %q INTO \'%q\' VALUES(?".as_ptr() as *mut i8 as
                                    *const i8, z_conflict, z_table)
                        };
                        __state = 337;
                    }
                    335 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"Error: can\'t malloc()".as_ptr() as *mut i8, 0 as *mut i8)
                        };
                        __state = 336;
                    }
                    336 => { return 1; }
                    337 => { j = strlen30(z_sql as *const i8); __state = 338; }
                    338 => { i = 1; __state = 340; }
                    339 => {
                        unsafe {
                            *z_sql.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = ')' as i32 as i8
                        };
                        __state = 344;
                    }
                    340 => {
                        if i < n_col { __state = 341; } else { __state = 339; }
                    }
                    341 => {
                        unsafe {
                            *z_sql.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = ',' as i32 as i8
                        };
                        __state = 343;
                    }
                    342 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 340;
                    }
                    343 => {
                        unsafe {
                            *z_sql.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '?' as i32 as i8
                        };
                        __state = 342;
                    }
                    344 => {
                        unsafe { *z_sql.offset(j as isize) = 0 as i8 };
                        __state = 345;
                    }
                    345 => {
                        rc =
                            unsafe {
                                sqlite3_prepare(unsafe { (*p_db).db }, z_sql as *const i8,
                                    -1, &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 346;
                    }
                    346 => { unsafe { free(z_sql as *mut ()) }; __state = 347; }
                    347 => {
                        if rc != 0 { __state = 349; } else { __state = 348; }
                    }
                    348 => {
                        in_ =
                            unsafe {
                                Tcl_OpenFileChannel(interp, z_file as *const i8,
                                    c"rb".as_ptr() as *mut i8 as *const i8, 438)
                            };
                        __state = 352;
                    }
                    349 => {
                        unsafe {
                            Tcl_AppendResult(interp, c"Error: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                                0 as *mut i8)
                        };
                        __state = 350;
                    }
                    350 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 351;
                    }
                    351 => { return 1; }
                    352 => {
                        if in_ == core::ptr::null_mut() {
                            __state = 354;
                        } else { __state = 353; }
                    }
                    353 => {
                        unsafe {
                            Tcl_SetChannelOption(0 as *mut () as *mut TclInterp, in_,
                                c"-translation".as_ptr() as *mut i8 as *const i8,
                                c"auto".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 356;
                    }
                    354 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 355;
                    }
                    355 => { return 1; }
                    356 => {
                        az_col =
                            unsafe {
                                    malloc(core::mem::size_of::<*mut i8>() as u64 *
                                            (n_col + 1) as u64)
                                } as *mut *mut i8;
                        __state = 357;
                    }
                    357 => {
                        if az_col == core::ptr::null_mut() {
                            __state = 359;
                        } else { __state = 358; }
                    }
                    358 => { str = unsafe { Tcl_NewObj() }; __state = 362; }
                    359 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"Error: can\'t malloc()".as_ptr() as *mut i8, 0 as *mut i8)
                        };
                        __state = 360;
                    }
                    360 => { unsafe { Tcl_Close(interp, in_) }; __state = 361; }
                    361 => { return 1; }
                    362 => {
                        {
                            let __p = unsafe { &mut (*str).refCount };
                            *__p += 1;
                            *__p
                        };
                        __state = 363;
                    }
                    363 => {
                        {
                            let _ =
                                unsafe {
                                    sqlite3_exec(unsafe { (*p_db).db },
                                        c"BEGIN".as_ptr() as *mut i8 as *const i8, None,
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                };
                        };
                        __state = 364;
                    }
                    364 => {
                        z_commit_1 = c"COMMIT".as_ptr() as *mut i8 as *const i8;
                        __state = 365;
                    }
                    365 => {
                        if unsafe { Tcl_GetsObj(in_, str) } >= 0 {
                            __state = 367;
                        } else { __state = 366; }
                    }
                    366 => {
                        if {
                                    let __p = unsafe { &mut (*str).refCount };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            __state = 406;
                        } else { __state = 405; }
                    }
                    367 => { __state = 368; }
                    368 => { __state = 369; }
                    369 => {
                        { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
                        __state = 370;
                    }
                    370 => {
                        z_line =
                            unsafe { Tcl_GetByteArrayFromObj(str, &mut byte_len) } as
                                *mut i8;
                        __state = 371;
                    }
                    371 => {
                        unsafe { *az_col.offset(0 as isize) = z_line };
                        __state = 372;
                    }
                    372 => { { i = 0; z = z_line }; __state = 374; }
                    373 => {
                        if i + 1 != n_col { __state = 383; } else { __state = 382; }
                    }
                    374 => {
                        if unsafe { *z } != 0 {
                            __state = 375;
                        } else { __state = 373; }
                    }
                    375 => {
                        if unsafe { *z } as i32 ==
                                    unsafe { *z_sep.offset(0 as isize) } as i32 &&
                                unsafe { strncmp(z as *const i8, z_sep, n_sep as u64) } == 0
                            {
                            __state = 377;
                        } else { __state = 376; }
                    }
                    376 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 374;
                    }
                    377 => { unsafe { *z = 0 as i8 }; __state = 378; }
                    378 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 379;
                    }
                    379 => {
                        if i < n_col { __state = 380; } else { __state = 376; }
                    }
                    380 => {
                        unsafe {
                            *az_col.offset(i as isize) =
                                unsafe { z.offset(n_sep as isize) }
                        };
                        __state = 381;
                    }
                    381 => {
                        {
                            let __n = n_sep - 1;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 376;
                    }
                    382 => { i = 0; __state = 393; }
                    383 => { __state = 384; }
                    384 => {
                        n_err = strlen30(z_file as *const i8) + 200;
                        __state = 385;
                    }
                    385 => {
                        z_err = unsafe { malloc(n_err as u64) } as *mut i8;
                        __state = 386;
                    }
                    386 => {
                        if !(z_err).is_null() {
                            __state = 388;
                        } else { __state = 387; }
                    }
                    387 => {
                        z_commit_1 = c"ROLLBACK".as_ptr() as *mut i8 as *const i8;
                        __state = 391;
                    }
                    388 => {
                        unsafe {
                            sqlite3_snprintf(n_err, z_err,
                                c"Error: %s line %d: expected %d columns of data but found %d".as_ptr()
                                        as *mut i8 as *const i8, z_file, lineno, n_col, i + 1)
                        };
                        __state = 389;
                    }
                    389 => {
                        unsafe { Tcl_AppendResult(interp, z_err, 0 as *mut i8) };
                        __state = 390;
                    }
                    390 => { unsafe { free(z_err as *mut ()) }; __state = 387; }
                    391 => { __state = 366; }
                    392 => { unsafe { sqlite3_step(p_stmt) }; __state = 398; }
                    393 => {
                        if i < n_col { __state = 394; } else { __state = 392; }
                    }
                    394 => {
                        if n_null > 0 &&
                                    unsafe {
                                            strcmp(unsafe { *az_col.offset(i as isize) } as *const i8,
                                                z_null)
                                        } == 0 ||
                                strlen30(unsafe { *az_col.offset(i as isize) } as *const i8)
                                    == 0 {
                            __state = 396;
                        } else { __state = 397; }
                    }
                    395 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 393;
                    }
                    396 => {
                        unsafe { sqlite3_bind_null(p_stmt, i + 1) };
                        __state = 395;
                    }
                    397 => {
                        unsafe {
                            sqlite3_bind_text(p_stmt, i + 1,
                                unsafe { *az_col.offset(i as isize) } as *const i8, -1,
                                None)
                        };
                        __state = 395;
                    }
                    398 => {
                        rc = unsafe { sqlite3_reset(p_stmt) };
                        __state = 399;
                    }
                    399 => {
                        unsafe { Tcl_SetObjLength(str, 0) };
                        __state = 400;
                    }
                    400 => {
                        if rc != 0 { __state = 401; } else { __state = 365; }
                    }
                    401 => {
                        unsafe {
                            Tcl_AppendResult(interp, c"Error: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                                0 as *mut i8)
                        };
                        __state = 402;
                    }
                    402 => {
                        z_commit_1 = c"ROLLBACK".as_ptr() as *mut i8 as *const i8;
                        __state = 403;
                    }
                    403 => { __state = 366; }
                    404 => {
                        unsafe { free(az_col as *mut ()) };
                        __state = 407;
                    }
                    405 => {
                        if 0 != 0 { __state = 366; } else { __state = 404; }
                    }
                    406 => { unsafe { TclFreeObj(str) }; __state = 405; }
                    407 => { unsafe { Tcl_Close(interp, in_) }; __state = 408; }
                    408 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 409;
                    }
                    409 => {
                        {
                            let _ =
                                unsafe {
                                    sqlite3_exec(unsafe { (*p_db).db }, z_commit_1, None,
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                };
                        };
                        __state = 410;
                    }
                    410 => {
                        if unsafe { *z_commit_1.offset(0 as isize) } as i32 ==
                                'C' as i32 {
                            __state = 412;
                        } else { __state = 413; }
                    }
                    411 => { __state = 13; }
                    412 => {
                        p_result_3 = unsafe { Tcl_GetObjResult(interp) };
                        __state = 414;
                    }
                    413 => {
                        unsafe {
                            sqlite3_snprintf(core::mem::size_of::<[i8; 80]>() as i32,
                                &raw mut z_line_num[0 as usize] as *mut i8,
                                c"%d".as_ptr() as *mut i8 as *const i8, lineno)
                        };
                        __state = 416;
                    }
                    414 => {
                        unsafe { Tcl_SetIntObj(p_result_3, lineno) };
                        __state = 415;
                    }
                    415 => { rc = 0; __state = 411; }
                    416 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c", failed while processing line: ".as_ptr() as *mut i8,
                                &raw mut z_line_num[0 as usize] as *mut i8, 0 as *mut i8)
                        };
                        __state = 417;
                    }
                    417 => { rc = 1; __state = 411; }
                    418 => { __state = 472; }
                    419 => { p_value = core::ptr::null_mut(); __state = 420; }
                    420 => { __state = 421; }
                    421 => { __state = 422; }
                    422 => { __state = 423; }
                    423 => { __state = 424; }
                    424 => { mx_size = 0 as Sqlite3Int64; __state = 425; }
                    425 => { __state = 426; }
                    426 => { is_readonly = 0; __state = 427; }
                    427 => {
                        if objc < 3 { __state = 429; } else { __state = 428; }
                    }
                    428 => { i__1 = 2; __state = 433; }
                    429 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?DATABASE? VALUE".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 430;
                    }
                    430 => { rc = 1; __state = 431; }
                    431 => { __state = 13; }
                    432 => {
                        p_value = unsafe { *objv.offset((objc - 1) as isize) };
                        __state = 454;
                    }
                    433 => {
                        if i__1 < objc - 1 {
                            __state = 434;
                        } else { __state = 432; }
                    }
                    434 => {
                        z__1 =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(i__1 as isize) })
                                } as *const i8;
                        __state = 436;
                    }
                    435 => {
                        { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                        __state = 433;
                    }
                    436 => {
                        if unsafe {
                                        strcmp(z__1, c"-maxsize".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i__1 < objc - 2 {
                            __state = 438;
                        } else { __state = 437; }
                    }
                    437 => {
                        if unsafe {
                                        strcmp(z__1, c"-readonly".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i__1 < objc - 2 {
                            __state = 445;
                        } else { __state = 444; }
                    }
                    438 => { __state = 439; }
                    439 => {
                        rc =
                            unsafe {
                                Tcl_GetWideIntFromObj(interp,
                                    unsafe {
                                        *objv.offset({ let __p = &mut i__1; *__p += 1; *__p } as
                                                    isize)
                                    }, &mut x)
                            };
                        __state = 440;
                    }
                    440 => {
                        if rc != 0 { __state = 442; } else { __state = 441; }
                    }
                    441 => { mx_size = x as Sqlite3Int64; __state = 443; }
                    442 => { __state = 2; }
                    443 => { __state = 435; }
                    444 => {
                        if z_schema == core::ptr::null() && i__1 == objc - 2 &&
                                unsafe { *z__1.offset(0 as isize) } as i32 != '-' as i32 {
                            __state = 450;
                        } else { __state = 449; }
                    }
                    445 => {
                        rc =
                            unsafe {
                                Tcl_GetBooleanFromObj(interp,
                                    unsafe {
                                        *objv.offset({ let __p = &mut i__1; *__p += 1; *__p } as
                                                    isize)
                                    }, &mut is_readonly)
                            };
                        __state = 446;
                    }
                    446 => {
                        if rc != 0 { __state = 448; } else { __state = 447; }
                    }
                    447 => { __state = 435; }
                    448 => { __state = 2; }
                    449 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"unknown option: ".as_ptr() as *mut i8, z__1, 0 as *mut i8)
                        };
                        __state = 452;
                    }
                    450 => { z_schema = z__1; __state = 451; }
                    451 => { __state = 435; }
                    452 => { rc = 1; __state = 453; }
                    453 => { __state = 2; }
                    454 => {
                        p_ba =
                            unsafe { Tcl_GetByteArrayFromObj(p_value, &mut len__4) };
                        __state = 455;
                    }
                    455 => {
                        p_data =
                            unsafe { sqlite3_malloc64(len__4 as Sqlite3Uint64) } as
                                *mut u8;
                        __state = 456;
                    }
                    456 => {
                        if p_data == core::ptr::null_mut() && len__4 > 0 {
                            __state = 458;
                        } else { __state = 459; }
                    }
                    457 => { __state = 2; }
                    458 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"out of memory".as_ptr() as *mut i8, 0 as *mut i8)
                        };
                        __state = 460;
                    }
                    459 => { __state = 461; }
                    460 => { rc = 1; __state = 457; }
                    461 => {
                        if len__4 > 0 { __state = 463; } else { __state = 462; }
                    }
                    462 => {
                        if is_readonly != 0 {
                            __state = 465;
                        } else { __state = 466; }
                    }
                    463 => {
                        unsafe {
                            memcpy(p_data as *mut (), p_ba as *const (), len__4 as u64)
                        };
                        __state = 462;
                    }
                    464 => {
                        xrc =
                            unsafe {
                                sqlite3_deserialize(unsafe { (*p_db).db }, z_schema, p_data,
                                    len__4 as Sqlite3Int64, len__4 as Sqlite3Int64,
                                    flags as u32)
                            };
                        __state = 467;
                    }
                    465 => { flags = 1 | 4; __state = 464; }
                    466 => { flags = 1 | 2; __state = 464; }
                    467 => {
                        if xrc != 0 { __state = 469; } else { __state = 468; }
                    }
                    468 => {
                        if mx_size > 0 as i64 {
                            __state = 471;
                        } else { __state = 457; }
                    }
                    469 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"unable to set MEMDB content".as_ptr() as *mut i8,
                                0 as *mut i8)
                        };
                        __state = 470;
                    }
                    470 => { rc = 1; __state = 468; }
                    471 => {
                        unsafe {
                            sqlite3_file_control(unsafe { (*p_db).db }, z_schema, 36,
                                &raw mut mx_size as *mut ())
                        };
                        __state = 457;
                    }
                    472 => { __state = 28; }
                    473 => { __state = 29; }
                    474 => {
                        if objc != 3 { __state = 476; } else { __state = 475; }
                    }
                    475 => {
                        if unsafe {
                                    Tcl_GetBooleanFromObj(interp,
                                        unsafe { *objv.offset(2 as isize) }, &mut onoff__1)
                                } != 0 {
                            __state = 479;
                        } else { __state = 478; }
                    }
                    476 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"BOOLEAN".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 477;
                    }
                    477 => { return 1; }
                    478 => {
                        unsafe {
                            sqlite3_enable_load_extension(unsafe { (*p_db).db },
                                onoff__1)
                        };
                        __state = 480;
                    }
                    479 => { return 1; }
                    480 => { __state = 13; }
                    481 => { __state = 30; }
                    482 => { __state = 13; }
                    483 => { __state = 31; }
                    484 => { __state = 13; }
                    485 => { __state = 505; }
                    486 => { __state = 487; }
                    487 => {
                        if objc != 3 { __state = 489; } else { __state = 488; }
                    }
                    488 => {
                        db_eval_init(&mut s_eval, p_db,
                            unsafe { *objv.offset(2 as isize) }, core::ptr::null_mut(),
                            0);
                        __state = 491;
                    }
                    489 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"SQL".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 490;
                    }
                    490 => { return 1; }
                    491 => { rc = db_eval_step(&mut s_eval); __state = 492; }
                    492 => {
                        if choice == DB_ONECOLUMN as i32 {
                            __state = 494;
                        } else { __state = 495; }
                    }
                    493 => { db_eval_finalize(&mut s_eval); __state = 500; }
                    494 => {
                        if rc == 0 { __state = 496; } else { __state = 497; }
                    }
                    495 => {
                        if rc == 3 || rc == 0 {
                            __state = 499;
                        } else { __state = 493; }
                    }
                    496 => {
                        p_result_4 = db_eval_column_value(&s_eval, 0);
                        __state = 493;
                    }
                    497 => {
                        if rc == 3 { __state = 498; } else { __state = 493; }
                    }
                    498 => {
                        unsafe { Tcl_ResetResult(interp) };
                        __state = 493;
                    }
                    499 => {
                        p_result_4 = unsafe { Tcl_NewBooleanObj((rc == 0) as i32) };
                        __state = 493;
                    }
                    500 => {
                        if !(p_result_4).is_null() {
                            __state = 502;
                        } else { __state = 501; }
                    }
                    501 => {
                        if rc == 3 { __state = 504; } else { __state = 503; }
                    }
                    502 => {
                        unsafe { Tcl_SetObjResult(interp, p_result_4) };
                        __state = 501;
                    }
                    503 => { __state = 13; }
                    504 => { rc = 0; __state = 503; }
                    505 => { __state = 33; }
                    506 => { __state = 34; }
                    507 => { __state = 508; }
                    508 => {
                        if objc > 3 &&
                                    {
                                            z_opt_1 =
                                                unsafe {
                                                        Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                                    } as *const i8;
                                            z_opt_1
                                        } != core::ptr::null() &&
                                unsafe { *z_opt_1.offset(0 as isize) } as i32 == '-' as i32
                            {
                            __state = 510;
                        } else { __state = 509; }
                    }
                    509 => {
                        if objc < 3 || objc > 5 {
                            __state = 519;
                        } else { __state = 518; }
                    }
                    510 => {
                        if unsafe {
                                    strcmp(z_opt_1,
                                        c"-withoutnulls".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 512;
                        } else { __state = 513; }
                    }
                    511 => {
                        { let __p = &mut objc; let __t = *__p; *__p -= 1; __t };
                        __state = 517;
                    }
                    512 => { eval_flags |= 1; __state = 511; }
                    513 => {
                        if unsafe {
                                    strcmp(z_opt_1, c"-asdict".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 514;
                        } else { __state = 515; }
                    }
                    514 => { eval_flags |= 2; __state = 511; }
                    515 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"unknown option: \"".as_ptr() as *mut i8, z_opt_1,
                                c"\"".as_ptr() as *mut i8, 0 as *mut ())
                        };
                        __state = 516;
                    }
                    516 => { return 1; }
                    517 => {
                        {
                            let __p = &mut objv;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 508;
                    }
                    518 => {
                        if objc == 3 { __state = 522; } else { __state = 523; }
                    }
                    519 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?OPTIONS? SQL ?VAR-NAME? ?SCRIPT?".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 520;
                    }
                    520 => { return 1; }
                    521 => { __state = 13; }
                    522 => { __state = 524; }
                    523 => { __state = 542; }
                    524 => { p_ret = unsafe { Tcl_NewObj() }; __state = 525; }
                    525 => {
                        {
                            let __p = unsafe { &mut (*p_ret).refCount };
                            *__p += 1;
                            *__p
                        };
                        __state = 526;
                    }
                    526 => {
                        db_eval_init(&mut s_eval_1, p_db,
                            unsafe { *objv.offset(2 as isize) }, core::ptr::null_mut(),
                            0);
                        __state = 527;
                    }
                    527 => {
                        if 0 == { rc = db_eval_step(&mut s_eval_1); rc } {
                            __state = 529;
                        } else { __state = 528; }
                    }
                    528 => { db_eval_finalize(&mut s_eval_1); __state = 536; }
                    529 => { __state = 530; }
                    530 => { __state = 531; }
                    531 => {
                        db_eval_row_info(&mut s_eval_1, &mut n_col_1,
                            core::ptr::null_mut());
                        __state = 532;
                    }
                    532 => { i__2 = 0; __state = 533; }
                    533 => {
                        if i__2 < n_col_1 { __state = 534; } else { __state = 527; }
                    }
                    534 => {
                        unsafe {
                            Tcl_ListObjAppendElement(interp, p_ret,
                                db_eval_column_value(&s_eval_1, i__2))
                        };
                        __state = 535;
                    }
                    535 => {
                        { let __p = &mut i__2; let __t = *__p; *__p += 1; __t };
                        __state = 533;
                    }
                    536 => {
                        if rc == 3 { __state = 538; } else { __state = 537; }
                    }
                    537 => {
                        if {
                                    let __p = unsafe { &mut (*p_ret).refCount };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            __state = 541;
                        } else { __state = 540; }
                    }
                    538 => {
                        unsafe { Tcl_SetObjResult(interp, p_ret) };
                        __state = 539;
                    }
                    539 => { rc = 0; __state = 537; }
                    540 => {
                        if 0 != 0 { __state = 537; } else { __state = 521; }
                    }
                    541 => { unsafe { TclFreeObj(p_ret) }; __state = 540; }
                    542 => { __state = 543; }
                    543 => {
                        p_var_name = core::ptr::null_mut();
                        __state = 544;
                    }
                    544 => { __state = 545; }
                    545 => {
                        if objc >= 5 &&
                                unsafe {
                                        *(unsafe {
                                                    Tcl_GetString(unsafe { *objv.offset(3 as isize) })
                                                } as *mut i8)
                                    } != 0 {
                            __state = 547;
                        } else { __state = 546; }
                    }
                    546 => {
                        p_script = unsafe { *objv.offset((objc - 1) as isize) };
                        __state = 548;
                    }
                    547 => {
                        p_var_name = unsafe { *objv.offset(3 as isize) };
                        __state = 546;
                    }
                    548 => {
                        {
                            let __p = unsafe { &mut (*p_script).refCount };
                            *__p += 1;
                            *__p
                        };
                        __state = 549;
                    }
                    549 => {
                        p =
                            unsafe {
                                    Tcl_Alloc(core::mem::size_of::<DbEvalContext>() as u32)
                                } as *mut DbEvalContext;
                        __state = 550;
                    }
                    550 => {
                        db_eval_init(p, p_db, unsafe { *objv.offset(2 as isize) },
                            p_var_name, eval_flags);
                        __state = 551;
                    }
                    551 => { cd2[0 as usize] = p as *mut (); __state = 552; }
                    552 => {
                        cd2[1 as usize] = p_script as *mut ();
                        __state = 553;
                    }
                    553 => {
                        rc =
                            db_eval_next_cmd(&raw mut cd2[0 as usize] as
                                    *mut ClientData, interp, 0);
                        __state = 521;
                    }
                    554 => { __state = 35; }
                    555 => { __state = 13; }
                    556 => { __state = 36; }
                    557 => { __state = 558; }
                    558 => { __state = 559; }
                    559 => { __state = 560; }
                    560 => { n_arg = -1; __state = 561; }
                    561 => { __state = 562; }
                    562 => { e_type = 5; __state = 563; }
                    563 => {
                        if objc < 4 { __state = 565; } else { __state = 564; }
                    }
                    564 => { i__3 = 3; __state = 568; }
                    565 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"NAME ?SWITCHES? SCRIPT".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 566;
                    }
                    566 => { return 1; }
                    567 => {
                        p_script_1 = unsafe { *objv.offset((objc - 1) as isize) };
                        __state = 601;
                    }
                    568 => {
                        if i__3 < objc - 1 {
                            __state = 569;
                        } else { __state = 567; }
                    }
                    569 => {
                        z__2 =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(i__3 as isize) })
                                } as *const i8;
                        __state = 571;
                    }
                    570 => {
                        { let __p = &mut i__3; let __t = *__p; *__p += 1; __t };
                        __state = 568;
                    }
                    571 => { n__1 = strlen30(z__2); __state = 572; }
                    572 => {
                        if n__1 > 1 &&
                                unsafe {
                                        strncmp(z__2, c"-argcount".as_ptr() as *mut i8 as *const i8,
                                            n__1 as u64)
                                    } == 0 {
                            __state = 573;
                        } else { __state = 574; }
                    }
                    573 => {
                        if i__3 == objc - 2 {
                            __state = 576;
                        } else { __state = 575; }
                    }
                    574 => {
                        if n__1 > 1 &&
                                unsafe {
                                        strncmp(z__2,
                                            c"-deterministic".as_ptr() as *mut i8 as *const i8,
                                            n__1 as u64)
                                    } == 0 {
                            __state = 583;
                        } else { __state = 584; }
                    }
                    575 => {
                        if unsafe {
                                    Tcl_GetIntFromObj(interp,
                                        unsafe { *objv.offset((i__3 + 1) as isize) }, &mut n_arg)
                                } != 0 {
                            __state = 579;
                        } else { __state = 578; }
                    }
                    576 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"option requires an argument: ".as_ptr() as *mut i8, z__2,
                                0 as *mut i8)
                        };
                        __state = 577;
                    }
                    577 => { return 1; }
                    578 => {
                        if n_arg < 0 { __state = 581; } else { __state = 580; }
                    }
                    579 => { return 1; }
                    580 => {
                        { let __p = &mut i__3; let __t = *__p; *__p += 1; __t };
                        __state = 570;
                    }
                    581 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"number of arguments must be non-negative".as_ptr() as
                                    *mut i8, 0 as *mut i8)
                        };
                        __state = 582;
                    }
                    582 => { return 1; }
                    583 => { flags__1 |= 2048; __state = 570; }
                    584 => {
                        if n__1 > 1 &&
                                unsafe {
                                        strncmp(z__2,
                                            c"-directonly".as_ptr() as *mut i8 as *const i8,
                                            n__1 as u64)
                                    } == 0 {
                            __state = 585;
                        } else { __state = 586; }
                    }
                    585 => { flags__1 |= 524288; __state = 570; }
                    586 => {
                        if n__1 > 1 &&
                                unsafe {
                                        strncmp(z__2,
                                            c"-innocuous".as_ptr() as *mut i8 as *const i8, n__1 as u64)
                                    } == 0 {
                            __state = 587;
                        } else { __state = 588; }
                    }
                    587 => { flags__1 |= 2097152; __state = 570; }
                    588 => {
                        if n__1 > 1 &&
                                unsafe {
                                        strncmp(z__2,
                                            c"-returntype".as_ptr() as *mut i8 as *const i8,
                                            n__1 as u64)
                                    } == 0 {
                            __state = 589;
                        } else { __state = 590; }
                    }
                    589 => { __state = 591; }
                    590 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"bad option \"".as_ptr() as *mut i8, z__2,
                                c"\": must be -argcount, -deterministic, -directonly, -innocuous, or -returntype".as_ptr()
                                    as *mut i8, 0 as *mut i8)
                        };
                        __state = 600;
                    }
                    591 => {
                        if !(1 == 1 && 2 == 2 && 3 == 3) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"DbObjCmd".as_ptr() as *const i8,
                                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 3425,
                                    c"SQLITE_INTEGER==1 && SQLITE_FLOAT==2 && SQLITE_TEXT==3".as_ptr()
                                            as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 592;
                    }
                    592 => {
                        if !(4 == 4 && 5 == 5) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"DbObjCmd".as_ptr() as *const i8,
                                    c"tclsqlite.c".as_ptr() as *mut i8 as *const i8, 3426,
                                    c"SQLITE_BLOB==4 && SQLITE_NULL==5".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 593;
                    }
                    593 => {
                        if i__3 == objc - 2 {
                            __state = 595;
                        } else { __state = 594; }
                    }
                    594 => {
                        { let __p = &mut i__3; let __t = *__p; *__p += 1; __t };
                        __state = 597;
                    }
                    595 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"option requires an argument: ".as_ptr() as *mut i8, z__2,
                                0 as *mut i8)
                        };
                        __state = 596;
                    }
                    596 => { return 1; }
                    597 => {
                        if unsafe {
                                    Tcl_GetIndexFromObj(interp,
                                        unsafe { *objv.offset(i__3 as isize) },
                                        &raw mut az_type[0 as usize] as *mut *const i8,
                                        c"type".as_ptr() as *mut i8 as *const i8, 0, &mut e_type)
                                } != 0 {
                            __state = 599;
                        } else { __state = 598; }
                    }
                    598 => {
                        { let __p = &mut e_type; let __t = *__p; *__p += 1; __t };
                        __state = 570;
                    }
                    599 => { return 1; }
                    600 => { return 1; }
                    601 => {
                        z_name_1 =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    core::ptr::null_mut())
                            };
                        __state = 602;
                    }
                    602 => {
                        p_func = find_sql_func(p_db, z_name_1 as *const i8);
                        __state = 603;
                    }
                    603 => {
                        if p_func == core::ptr::null_mut() {
                            __state = 605;
                        } else { __state = 604; }
                    }
                    604 => {
                        if !(unsafe { (*p_func).p_script }).is_null() {
                            __state = 607;
                        } else { __state = 606; }
                    }
                    605 => { return 1; }
                    606 => {
                        unsafe { (*p_func).p_script = p_script_1 };
                        __state = 610;
                    }
                    607 => {
                        if {
                                    let __p =
                                        unsafe { &mut (*unsafe { (*p_func).p_script }).refCount };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            __state = 609;
                        } else { __state = 608; }
                    }
                    608 => {
                        if 0 != 0 { __state = 607; } else { __state = 606; }
                    }
                    609 => {
                        unsafe { TclFreeObj(unsafe { (*p_func).p_script }) };
                        __state = 608;
                    }
                    610 => {
                        {
                            let __p = unsafe { &mut (*p_script_1).refCount };
                            *__p += 1;
                            *__p
                        };
                        __state = 611;
                    }
                    611 => {
                        unsafe {
                            (*p_func).use_eval_objv = safe_to_use_eval_objv(p_script_1)
                        };
                        __state = 612;
                    }
                    612 => {
                        unsafe { (*p_func).e_type = e_type };
                        __state = 613;
                    }
                    613 => {
                        rc =
                            unsafe {
                                sqlite3_create_function(unsafe { (*p_db).db },
                                    z_name_1 as *const i8, n_arg, flags__1, p_func as *mut (),
                                    Some(tcl_sql_func), None, None)
                            };
                        __state = 614;
                    }
                    614 => {
                        if rc != 0 { __state = 616; } else { __state = 615; }
                    }
                    615 => { __state = 13; }
                    616 => { rc = 1; __state = 617; }
                    617 => {
                        unsafe {
                            Tcl_SetResult(interp,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) } as *mut i8,
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut i8) -> ()>(1 as *const ())
                                })
                        };
                        __state = 615;
                    }
                    618 => { __state = 37; }
                    619 => {
                        z_db = c"main".as_ptr() as *mut i8 as *const i8;
                        __state = 620;
                    }
                    620 => { __state = 621; }
                    621 => { __state = 622; }
                    622 => { __state = 623; }
                    623 => {
                        if objc > 3 &&
                                unsafe {
                                        strcmp(unsafe {
                                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                                } as *const i8,
                                            c"-readonly".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            __state = 625;
                        } else { __state = 624; }
                    }
                    624 => {
                        if objc != 5 + is_readonly_1 && objc != 6 + is_readonly_1 {
                            __state = 627;
                        } else { __state = 626; }
                    }
                    625 => { is_readonly_1 = 1; __state = 624; }
                    626 => {
                        if objc == 6 + is_readonly_1 {
                            __state = 630;
                        } else { __state = 629; }
                    }
                    627 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?-readonly? ?DB? TABLE COLUMN ROWID".as_ptr() as *mut i8
                                    as *const i8)
                        };
                        __state = 628;
                    }
                    628 => { return 1; }
                    629 => {
                        z_table_1 =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset((objc - 3) as isize) })
                                } as *const i8;
                        __state = 631;
                    }
                    630 => {
                        z_db =
                            unsafe {
                                    Tcl_GetString(unsafe {
                                            *objv.offset((2 + is_readonly_1) as isize)
                                        })
                                } as *const i8;
                        __state = 629;
                    }
                    631 => {
                        z_column =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset((objc - 2) as isize) })
                                } as *const i8;
                        __state = 632;
                    }
                    632 => {
                        rc =
                            unsafe {
                                Tcl_GetWideIntFromObj(interp,
                                    unsafe { *objv.offset((objc - 1) as isize) }, &mut i_row)
                            };
                        __state = 633;
                    }
                    633 => {
                        if rc == 0 { __state = 635; } else { __state = 634; }
                    }
                    634 => { __state = 13; }
                    635 => {
                        rc =
                            create_incrblob_channel(interp, p_db, z_db, z_table_1,
                                z_column, i_row as Sqlite3Int64, is_readonly_1);
                        __state = 634;
                    }
                    636 => { __state = 38; }
                    637 => { __state = 13; }
                    638 => { __state = 39; }
                    639 => {
                        if objc == 3 { __state = 643; } else { __state = 642; }
                    }
                    640 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"NULLVALUE".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 641;
                    }
                    641 => { return 1; }
                    642 => {
                        unsafe {
                            Tcl_SetObjResult(interp,
                                unsafe {
                                    Tcl_NewStringObj(unsafe { (*p_db).z_null } as *const i8, -1)
                                })
                        };
                        __state = 652;
                    }
                    643 => { __state = 644; }
                    644 => {
                        z_null_1 =
                            unsafe {
                                    Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                        &mut len__5)
                                } as *const i8;
                        __state = 645;
                    }
                    645 => {
                        if !(unsafe { (*p_db).z_null }).is_null() {
                            __state = 647;
                        } else { __state = 646; }
                    }
                    646 => {
                        if !(z_null_1).is_null() && len__5 > 0 {
                            __state = 648;
                        } else { __state = 649; }
                    }
                    647 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_null }) };
                        __state = 646;
                    }
                    648 => {
                        unsafe {
                            (*p_db).z_null = unsafe { Tcl_Alloc((len__5 + 1) as u32) }
                        };
                        __state = 650;
                    }
                    649 => {
                        unsafe { (*p_db).z_null = core::ptr::null_mut() };
                        __state = 642;
                    }
                    650 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_null } as *mut (),
                                z_null_1 as *const (), len__5 as u64)
                        };
                        __state = 651;
                    }
                    651 => {
                        unsafe {
                            *unsafe { (*p_db).z_null.offset(len__5 as isize) } =
                                '\u{0}' as i32 as i8
                        };
                        __state = 642;
                    }
                    652 => { __state = 13; }
                    653 => { __state = 40; }
                    654 => { __state = 655; }
                    655 => {
                        if objc != 2 { __state = 657; } else { __state = 656; }
                    }
                    656 => {
                        rowid =
                            unsafe { sqlite3_last_insert_rowid(unsafe { (*p_db).db }) }
                                as TclWideInt;
                        __state = 659;
                    }
                    657 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 658;
                    }
                    658 => { return 1; }
                    659 => {
                        p_result_5 = unsafe { Tcl_GetObjResult(interp) };
                        __state = 660;
                    }
                    660 => {
                        unsafe { Tcl_SetWideIntObj(p_result_5, rowid) };
                        __state = 661;
                    }
                    661 => { __state = 13; }
                    662 => { __state = 41; }
                    663 => { __state = 13; }
                    664 => {
                        if !(unsafe { (*p_db).z_progress }).is_null() {
                            __state = 667;
                        } else { __state = 666; }
                    }
                    665 => {
                        if objc == 4 { __state = 668; } else { __state = 669; }
                    }
                    666 => {
                        unsafe {
                            sqlite3_progress_handler(unsafe { (*p_db).db }, 0, None,
                                core::ptr::null_mut())
                        };
                        __state = 663;
                    }
                    667 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_progress },
                                0 as *mut i8)
                        };
                        __state = 666;
                    }
                    668 => { __state = 670; }
                    669 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"N CALLBACK".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 686;
                    }
                    670 => { __state = 671; }
                    671 => { __state = 672; }
                    672 => {
                        if 0 !=
                                unsafe {
                                    Tcl_GetIntFromObj(interp,
                                        unsafe { *objv.offset(2 as isize) }, &mut n__2)
                                } {
                            __state = 674;
                        } else { __state = 673; }
                    }
                    673 => { __state = 675; }
                    674 => { return 1; }
                    675 => {
                        if !(unsafe { (*p_db).z_progress }).is_null() {
                            __state = 677;
                        } else { __state = 676; }
                    }
                    676 => {
                        z_progress =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(3 as isize) },
                                    &mut len__6)
                            };
                        __state = 678;
                    }
                    677 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_progress }) };
                        __state = 676;
                    }
                    678 => {
                        if !(z_progress).is_null() && len__6 > 0 {
                            __state = 680;
                        } else { __state = 681; }
                    }
                    679 => {
                        if !(unsafe { (*p_db).z_progress }).is_null() {
                            __state = 683;
                        } else { __state = 684; }
                    }
                    680 => {
                        unsafe {
                            (*p_db).z_progress =
                                unsafe { Tcl_Alloc((len__6 + 1) as u32) }
                        };
                        __state = 682;
                    }
                    681 => {
                        unsafe { (*p_db).z_progress = core::ptr::null_mut() };
                        __state = 679;
                    }
                    682 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_progress } as *mut (),
                                z_progress as *const (), (len__6 + 1) as u64)
                        };
                        __state = 679;
                    }
                    683 => {
                        unsafe { (*p_db).interp = interp };
                        __state = 685;
                    }
                    684 => {
                        unsafe {
                            sqlite3_progress_handler(unsafe { (*p_db).db }, 0, None,
                                core::ptr::null_mut())
                        };
                        __state = 663;
                    }
                    685 => {
                        unsafe {
                            sqlite3_progress_handler(unsafe { (*p_db).db }, n__2,
                                Some(db_progress_handler), p_db as *mut ())
                        };
                        __state = 663;
                    }
                    686 => { return 1; }
                    687 => { __state = 42; }
                    688 => { __state = 13; }
                    689 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?CALLBACK?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 691;
                    }
                    690 => {
                        if objc == 2 { __state = 692; } else { __state = 693; }
                    }
                    691 => { return 1; }
                    692 => {
                        if !(unsafe { (*p_db).z_profile }).is_null() {
                            __state = 694;
                        } else { __state = 688; }
                    }
                    693 => { __state = 695; }
                    694 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_profile },
                                0 as *mut i8)
                        };
                        __state = 688;
                    }
                    695 => { __state = 696; }
                    696 => {
                        if !(unsafe { (*p_db).z_profile }).is_null() {
                            __state = 698;
                        } else { __state = 697; }
                    }
                    697 => {
                        z_profile =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    &mut len__7)
                            };
                        __state = 699;
                    }
                    698 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_profile }) };
                        __state = 697;
                    }
                    699 => {
                        if !(z_profile).is_null() && len__7 > 0 {
                            __state = 701;
                        } else { __state = 702; }
                    }
                    700 => {
                        if !(unsafe { (*p_db).z_profile }).is_null() {
                            __state = 704;
                        } else { __state = 705; }
                    }
                    701 => {
                        unsafe {
                            (*p_db).z_profile =
                                unsafe { Tcl_Alloc((len__7 + 1) as u32) }
                        };
                        __state = 703;
                    }
                    702 => {
                        unsafe { (*p_db).z_profile = core::ptr::null_mut() };
                        __state = 700;
                    }
                    703 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_profile } as *mut (),
                                z_profile as *const (), (len__7 + 1) as u64)
                        };
                        __state = 700;
                    }
                    704 => {
                        unsafe { (*p_db).interp = interp };
                        __state = 706;
                    }
                    705 => {
                        unsafe {
                            sqlite3_profile(unsafe { (*p_db).db }, None,
                                core::ptr::null_mut())
                        };
                        __state = 688;
                    }
                    706 => {
                        unsafe {
                            sqlite3_profile(unsafe { (*p_db).db },
                                Some(db_profile_handler), p_db as *mut ())
                        };
                        __state = 688;
                    }
                    707 => { __state = 43; }
                    708 => { __state = 13; }
                    709 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"KEY".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 710;
                    }
                    710 => { return 1; }
                    711 => { __state = 44; }
                    712 => { __state = 713; }
                    713 => { __state = 714; }
                    714 => { __state = 715; }
                    715 => { n_timeout = 0; __state = 716; }
                    716 => {
                        if objc == 3 { __state = 718; } else { __state = 719; }
                    }
                    717 => {
                        rc =
                            unsafe {
                                sqlite3_open_v2(z_src_file, &mut p_src,
                                    1 | unsafe { (*p_db).open_flags }, core::ptr::null())
                            };
                        __state = 725;
                    }
                    718 => {
                        z_dest_db = c"main".as_ptr() as *mut i8 as *const i8;
                        __state = 720;
                    }
                    719 => {
                        if objc == 4 { __state = 721; } else { __state = 722; }
                    }
                    720 => {
                        z_src_file =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                } as *const i8;
                        __state = 717;
                    }
                    721 => {
                        z_dest_db =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                } as *const i8;
                        __state = 723;
                    }
                    722 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?DATABASE? FILENAME".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 724;
                    }
                    723 => {
                        z_src_file =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(3 as isize) })
                                } as *const i8;
                        __state = 717;
                    }
                    724 => { return 1; }
                    725 => {
                        if rc != 0 { __state = 727; } else { __state = 726; }
                    }
                    726 => {
                        p_backup_1 =
                            unsafe {
                                sqlite3_backup_init(unsafe { (*p_db).db }, z_dest_db, p_src,
                                    c"main".as_ptr() as *mut i8 as *const i8)
                            };
                        __state = 730;
                    }
                    727 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"cannot open source database: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(p_src) }, 0 as *mut i8)
                        };
                        __state = 728;
                    }
                    728 => { unsafe { sqlite3_close(p_src) }; __state = 729; }
                    729 => { return 1; }
                    730 => {
                        if p_backup_1 == core::ptr::null_mut() {
                            __state = 732;
                        } else { __state = 731; }
                    }
                    731 => {
                        if {
                                        rc = unsafe { sqlite3_backup_step(p_backup_1, 100) };
                                        rc
                                    } == 0 || rc == 5 {
                            __state = 736;
                        } else { __state = 735; }
                    }
                    732 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"restore failed: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                                0 as *mut i8)
                        };
                        __state = 733;
                    }
                    733 => { unsafe { sqlite3_close(p_src) }; __state = 734; }
                    734 => { return 1; }
                    735 => {
                        unsafe { sqlite3_backup_finish(p_backup_1) };
                        __state = 740;
                    }
                    736 => {
                        if rc == 5 { __state = 737; } else { __state = 731; }
                    }
                    737 => {
                        if {
                                    let __p = &mut n_timeout;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } >= 3 {
                            __state = 739;
                        } else { __state = 738; }
                    }
                    738 => { unsafe { sqlite3_sleep(100) }; __state = 731; }
                    739 => { __state = 735; }
                    740 => {
                        if rc == 101 { __state = 742; } else { __state = 743; }
                    }
                    741 => { unsafe { sqlite3_close(p_src) }; __state = 748; }
                    742 => { rc = 0; __state = 741; }
                    743 => {
                        if rc == 5 || rc == 6 {
                            __state = 744;
                        } else { __state = 745; }
                    }
                    744 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"restore failed: source database busy".as_ptr() as *mut i8,
                                0 as *mut i8)
                        };
                        __state = 746;
                    }
                    745 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"restore failed: ".as_ptr() as *mut i8,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                                0 as *mut i8)
                        };
                        __state = 747;
                    }
                    746 => { rc = 1; __state = 741; }
                    747 => { rc = 1; __state = 741; }
                    748 => { __state = 13; }
                    749 => { __state = 45; }
                    750 => { sz = 0 as Sqlite3Int64; __state = 751; }
                    751 => { __state = 752; }
                    752 => {
                        if objc != 2 && objc != 3 {
                            __state = 754;
                        } else { __state = 755; }
                    }
                    753 => { __state = 13; }
                    754 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?DATABASE?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 756;
                    }
                    755 => { __state = 757; }
                    756 => { rc = 1; __state = 753; }
                    757 => {
                        p_data_1 =
                            unsafe {
                                sqlite3_serialize(unsafe { (*p_db).db }, z_schema_1,
                                    &mut sz, 1 as u32)
                            };
                        __state = 758;
                    }
                    758 => {
                        if !(p_data_1).is_null() {
                            __state = 760;
                        } else { __state = 761; }
                    }
                    759 => {
                        unsafe {
                            Tcl_SetObjResult(interp,
                                unsafe {
                                    Tcl_NewByteArrayObj(p_data_1 as *const u8, sz as i32)
                                })
                        };
                        __state = 763;
                    }
                    760 => { need_free = 0; __state = 759; }
                    761 => {
                        p_data_1 =
                            unsafe {
                                sqlite3_serialize(unsafe { (*p_db).db }, z_schema_1,
                                    &mut sz, 0 as u32)
                            };
                        __state = 762;
                    }
                    762 => { need_free = 1; __state = 759; }
                    763 => {
                        if need_free != 0 { __state = 764; } else { __state = 753; }
                    }
                    764 => {
                        unsafe { sqlite3_free(p_data_1 as *mut ()) };
                        __state = 753;
                    }
                    765 => { __state = 46; }
                    766 => { __state = 767; }
                    767 => {
                        if objc != 3 { __state = 769; } else { __state = 768; }
                    }
                    768 => {
                        z_op =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(2 as isize) })
                                } as *const i8;
                        __state = 771;
                    }
                    769 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"(step|sort|autoindex)".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 770;
                    }
                    770 => { return 1; }
                    771 => {
                        if unsafe {
                                    strcmp(z_op, c"step".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 773;
                        } else { __state = 774; }
                    }
                    772 => {
                        unsafe {
                            Tcl_SetObjResult(interp, unsafe { Tcl_NewIntObj(v__2) })
                        };
                        __state = 782;
                    }
                    773 => { v__2 = unsafe { (*p_db).n_step }; __state = 772; }
                    774 => {
                        if unsafe {
                                    strcmp(z_op, c"sort".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 775;
                        } else { __state = 776; }
                    }
                    775 => { v__2 = unsafe { (*p_db).n_sort }; __state = 772; }
                    776 => {
                        if unsafe {
                                    strcmp(z_op, c"autoindex".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 777;
                        } else { __state = 778; }
                    }
                    777 => { v__2 = unsafe { (*p_db).n_index }; __state = 772; }
                    778 => {
                        if unsafe {
                                    strcmp(z_op, c"vmstep".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            __state = 779;
                        } else { __state = 780; }
                    }
                    779 => {
                        v__2 = unsafe { (*p_db).n_vm_step };
                        __state = 772;
                    }
                    780 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"bad argument: should be autoindex, step, sort or vmstep".as_ptr()
                                    as *mut i8, 0 as *mut i8)
                        };
                        __state = 781;
                    }
                    781 => { return 1; }
                    782 => { __state = 13; }
                    783 => { __state = 47; }
                    784 => {
                        if objc != 3 { __state = 786; } else { __state = 785; }
                    }
                    785 => {
                        if unsafe {
                                    Tcl_GetIntFromObj(interp,
                                        unsafe { *objv.offset(2 as isize) }, &mut ms)
                                } != 0 {
                            __state = 789;
                        } else { __state = 788; }
                    }
                    786 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"MILLISECONDS".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 787;
                    }
                    787 => { return 1; }
                    788 => {
                        unsafe { sqlite3_busy_timeout(unsafe { (*p_db).db }, ms) };
                        __state = 790;
                    }
                    789 => { return 1; }
                    790 => { __state = 13; }
                    791 => { __state = 48; }
                    792 => {
                        if objc != 2 { __state = 794; } else { __state = 793; }
                    }
                    793 => {
                        p_result_6 = unsafe { Tcl_GetObjResult(interp) };
                        __state = 796;
                    }
                    794 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 795;
                    }
                    795 => { return 1; }
                    796 => {
                        unsafe {
                            Tcl_SetWideIntObj(p_result_6,
                                unsafe { sqlite3_total_changes64(unsafe { (*p_db).db }) })
                        };
                        __state = 797;
                    }
                    797 => { __state = 13; }
                    798 => { __state = 49; }
                    799 => { __state = 13; }
                    800 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?CALLBACK?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 802;
                    }
                    801 => {
                        if objc == 2 { __state = 803; } else { __state = 804; }
                    }
                    802 => { return 1; }
                    803 => {
                        if !(unsafe { (*p_db).z_trace }).is_null() {
                            __state = 805;
                        } else { __state = 799; }
                    }
                    804 => { __state = 806; }
                    805 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_trace },
                                0 as *mut i8)
                        };
                        __state = 799;
                    }
                    806 => { __state = 807; }
                    807 => {
                        if !(unsafe { (*p_db).z_trace }).is_null() {
                            __state = 809;
                        } else { __state = 808; }
                    }
                    808 => {
                        z_trace =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    &mut len__8)
                            };
                        __state = 810;
                    }
                    809 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_trace }) };
                        __state = 808;
                    }
                    810 => {
                        if !(z_trace).is_null() && len__8 > 0 {
                            __state = 812;
                        } else { __state = 813; }
                    }
                    811 => {
                        if !(unsafe { (*p_db).z_trace }).is_null() {
                            __state = 815;
                        } else { __state = 816; }
                    }
                    812 => {
                        unsafe {
                            (*p_db).z_trace = unsafe { Tcl_Alloc((len__8 + 1) as u32) }
                        };
                        __state = 814;
                    }
                    813 => {
                        unsafe { (*p_db).z_trace = core::ptr::null_mut() };
                        __state = 811;
                    }
                    814 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_trace } as *mut (),
                                z_trace as *const (), (len__8 + 1) as u64)
                        };
                        __state = 811;
                    }
                    815 => {
                        unsafe { (*p_db).interp = interp };
                        __state = 817;
                    }
                    816 => {
                        unsafe {
                            sqlite3_trace(unsafe { (*p_db).db }, None,
                                core::ptr::null_mut())
                        };
                        __state = 799;
                    }
                    817 => {
                        unsafe {
                            sqlite3_trace(unsafe { (*p_db).db }, Some(db_trace_handler),
                                p_db as *mut ())
                        };
                        __state = 799;
                    }
                    818 => { __state = 50; }
                    819 => { __state = 13; }
                    820 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?CALLBACK? ?MASK?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 822;
                    }
                    821 => {
                        if objc == 2 { __state = 823; } else { __state = 824; }
                    }
                    822 => { return 1; }
                    823 => {
                        if !(unsafe { (*p_db).z_trace_v2 }).is_null() {
                            __state = 825;
                        } else { __state = 819; }
                    }
                    824 => { __state = 826; }
                    825 => {
                        unsafe {
                            Tcl_AppendResult(interp, unsafe { (*p_db).z_trace_v2 },
                                0 as *mut i8)
                        };
                        __state = 819;
                    }
                    826 => { __state = 827; }
                    827 => { w_mask = 0 as TclWideInt; __state = 828; }
                    828 => {
                        if objc == 4 { __state = 830; } else { __state = 831; }
                    }
                    829 => {
                        if !(unsafe { (*p_db).z_trace_v2 }).is_null() {
                            __state = 871;
                        } else { __state = 870; }
                    }
                    830 => { __state = 832; }
                    831 => { w_mask = 1 as TclWideInt; __state = 829; }
                    832 => { __state = 833; }
                    833 => { __state = 834; }
                    834 => {
                        if 0 !=
                                unsafe {
                                    Tcl_ListObjLength(interp,
                                        unsafe { *objv.offset(3 as isize) }, &mut len__9)
                                } {
                            __state = 836;
                        } else { __state = 835; }
                    }
                    835 => { i__4 = 0; __state = 837; }
                    836 => { return 1; }
                    837 => {
                        if i__4 < len__9 { __state = 838; } else { __state = 829; }
                    }
                    838 => { __state = 840; }
                    839 => {
                        { let __p = &mut i__4; let __t = *__p; *__p += 1; __t };
                        __state = 837;
                    }
                    840 => { __state = 841; }
                    841 => {
                        if 0 !=
                                unsafe {
                                    Tcl_ListObjIndex(interp,
                                        unsafe { *objv.offset(3 as isize) }, i__4, &mut p_obj)
                                } {
                            __state = 843;
                        } else { __state = 842; }
                    }
                    842 => {
                        if unsafe {
                                    Tcl_GetIndexFromObj(interp, p_obj,
                                        &raw mut ttype_strs[0 as usize] as *mut *const i8,
                                        c"trace type".as_ptr() as *mut i8 as *const i8, 0,
                                        &mut ttype)
                                } != 0 {
                            __state = 844;
                        } else { __state = 845; }
                    }
                    843 => { return 1; }
                    844 => { __state = 846; }
                    845 => {
                        '__s57:
                            {
                            match ttype as u32 {
                                TTYPE_STMT => { __state = 858; }
                                TTYPE_PROFILE => { __state = 859; }
                                TTYPE_ROW => { __state = 860; }
                                TTYPE_CLOSE => { __state = 861; }
                                _ => { __state = 839; }
                            }
                        }
                    }
                    846 => {
                        p_error =
                            unsafe {
                                Tcl_DuplicateObj(unsafe { Tcl_GetObjResult(interp) })
                            };
                        __state = 847;
                    }
                    847 => {
                        {
                            let __p = unsafe { &mut (*p_error).refCount };
                            *__p += 1;
                            *__p
                        };
                        __state = 848;
                    }
                    848 => {
                        if 0 ==
                                unsafe { Tcl_GetWideIntFromObj(interp, p_obj, &mut w_type) }
                            {
                            __state = 849;
                        } else { __state = 850; }
                    }
                    849 => {
                        if {
                                    let __p = unsafe { &mut (*p_error).refCount };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            __state = 853;
                        } else { __state = 852; }
                    }
                    850 => {
                        unsafe { Tcl_SetObjResult(interp, p_error) };
                        __state = 854;
                    }
                    851 => { w_mask |= w_type; __state = 839; }
                    852 => {
                        if 0 != 0 { __state = 849; } else { __state = 851; }
                    }
                    853 => { unsafe { TclFreeObj(p_error) }; __state = 852; }
                    854 => {
                        if {
                                    let __p = unsafe { &mut (*p_error).refCount };
                                    *__p -= 1;
                                    *__p
                                } <= 0 {
                            __state = 857;
                        } else { __state = 856; }
                    }
                    855 => { return 1; }
                    856 => {
                        if 0 != 0 { __state = 854; } else { __state = 855; }
                    }
                    857 => { unsafe { TclFreeObj(p_error) }; __state = 856; }
                    858 => { w_mask |= 1 as TclWideInt; __state = 863; }
                    859 => { w_mask |= 2 as TclWideInt; __state = 865; }
                    860 => { w_mask |= 4 as TclWideInt; __state = 867; }
                    861 => { w_mask |= 8 as TclWideInt; __state = 869; }
                    862 => { __state = 858; }
                    863 => { __state = 839; }
                    864 => { __state = 859; }
                    865 => { __state = 839; }
                    866 => { __state = 860; }
                    867 => { __state = 839; }
                    868 => { __state = 861; }
                    869 => { __state = 839; }
                    870 => {
                        z_trace_v2 =
                            unsafe {
                                Tcl_GetStringFromObj(unsafe { *objv.offset(2 as isize) },
                                    &mut len__9)
                            };
                        __state = 872;
                    }
                    871 => {
                        unsafe { Tcl_Free(unsafe { (*p_db).z_trace_v2 }) };
                        __state = 870;
                    }
                    872 => {
                        if !(z_trace_v2).is_null() && len__9 > 0 {
                            __state = 874;
                        } else { __state = 875; }
                    }
                    873 => {
                        if !(unsafe { (*p_db).z_trace_v2 }).is_null() {
                            __state = 877;
                        } else { __state = 878; }
                    }
                    874 => {
                        unsafe {
                            (*p_db).z_trace_v2 =
                                unsafe { Tcl_Alloc((len__9 + 1) as u32) }
                        };
                        __state = 876;
                    }
                    875 => {
                        unsafe { (*p_db).z_trace_v2 = core::ptr::null_mut() };
                        __state = 873;
                    }
                    876 => {
                        unsafe {
                            memcpy(unsafe { (*p_db).z_trace_v2 } as *mut (),
                                z_trace_v2 as *const (), (len__9 + 1) as u64)
                        };
                        __state = 873;
                    }
                    877 => {
                        unsafe { (*p_db).interp = interp };
                        __state = 879;
                    }
                    878 => {
                        unsafe {
                            sqlite3_trace_v2(unsafe { (*p_db).db }, 0 as u32, None,
                                core::ptr::null_mut())
                        };
                        __state = 819;
                    }
                    879 => {
                        unsafe {
                            sqlite3_trace_v2(unsafe { (*p_db).db }, w_mask as u32,
                                Some(db_trace_v2_handler), p_db as *mut ())
                        };
                        __state = 819;
                    }
                    880 => { __state = 51; }
                    881 => {
                        z_begin =
                            c"SAVEPOINT _tcl_transaction".as_ptr() as *mut i8 as
                                *const i8;
                        __state = 882;
                    }
                    882 => {
                        if objc != 3 && objc != 4 {
                            __state = 884;
                        } else { __state = 883; }
                    }
                    883 => {
                        if unsafe { (*p_db).n_transaction } == 0 && objc == 4 {
                            __state = 887;
                        } else { __state = 886; }
                    }
                    884 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"[TYPE] SCRIPT".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 885;
                    }
                    885 => { return 1; }
                    886 => {
                        p_script_2 = unsafe { *objv.offset((objc - 1) as isize) };
                        __state = 902;
                    }
                    887 => { __state = 888; }
                    888 => { __state = 889; }
                    889 => { __state = 890; }
                    890 => {
                        if unsafe {
                                    Tcl_GetIndexFromObj(interp,
                                        unsafe { *objv.offset(2 as isize) },
                                        &raw mut ttype_strs_1[0 as usize] as *mut *const i8,
                                        c"transaction type".as_ptr() as *mut i8 as *const i8, 0,
                                        &mut ttype__1)
                                } != 0 {
                            __state = 892;
                        } else { __state = 891; }
                    }
                    891 => {
                        '__s58:
                            {
                            match ttype__1 as u32 {
                                TTYPE_DEFERRED => { __state = 893; }
                                TTYPE_EXCLUSIVE => { __state = 894; }
                                TTYPE_IMMEDIATE => { __state = 895; }
                                _ => { __state = 886; }
                            }
                        }
                    }
                    892 => { return 1; }
                    893 => { __state = 897; }
                    894 => {
                        z_begin =
                            c"BEGIN EXCLUSIVE".as_ptr() as *mut i8 as *const i8;
                        __state = 899;
                    }
                    895 => {
                        z_begin =
                            c"BEGIN IMMEDIATE".as_ptr() as *mut i8 as *const i8;
                        __state = 901;
                    }
                    896 => { __state = 893; }
                    897 => { __state = 886; }
                    898 => { __state = 894; }
                    899 => { __state = 886; }
                    900 => { __state = 895; }
                    901 => { __state = 886; }
                    902 => {
                        {
                            let __p = unsafe { &mut (*p_db).disable_auth };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 903;
                    }
                    903 => {
                        rc =
                            unsafe {
                                sqlite3_exec(unsafe { (*p_db).db }, z_begin, None,
                                    core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        __state = 904;
                    }
                    904 => {
                        {
                            let __p = unsafe { &mut (*p_db).disable_auth };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 905;
                    }
                    905 => {
                        if rc != 0 { __state = 907; } else { __state = 906; }
                    }
                    906 => {
                        {
                            let __p = unsafe { &mut (*p_db).n_transaction };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 909;
                    }
                    907 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                unsafe { sqlite3_errmsg(unsafe { (*p_db).db }) },
                                0 as *mut i8)
                        };
                        __state = 908;
                    }
                    908 => { return 1; }
                    909 => {
                        add_database_ref(unsafe { &mut *p_db });
                        __state = 910;
                    }
                    910 => {
                        if 0 != 0 { __state = 912; } else { __state = 913; }
                    }
                    911 => { __state = 13; }
                    912 => { { let _ = 0; }; __state = 914; }
                    913 => {
                        rc =
                            db_trans_post_cmd(&raw mut cd as *mut ClientData, interp,
                                unsafe { Tcl_EvalObjEx(interp, p_script_2, 0) });
                        __state = 911;
                    }
                    914 => { { let _ = 0; }; __state = 911; }
                    915 => { __state = 52; }
                    916 => { rc = 1; __state = 917; }
                    917 => { __state = 13; }
                    918 => { __state = 53; }
                    919 => { rc = 1; __state = 920; }
                    920 => { __state = 13; }
                    921 => { __state = 933; }
                    922 => {
                        if choice == DB_WAL_HOOK as i32 {
                            __state = 924;
                        } else { __state = 923; }
                    }
                    923 => {
                        if choice == DB_UPDATE_HOOK as i32 {
                            __state = 926;
                        } else { __state = 925; }
                    }
                    924 => {
                        pp_hook = unsafe { &mut (*p_db).p_wal_hook };
                        __state = 923;
                    }
                    925 => {
                        if choice == DB_ROLLBACK_HOOK as i32 {
                            __state = 928;
                        } else { __state = 927; }
                    }
                    926 => {
                        pp_hook = unsafe { &mut (*p_db).p_update_hook };
                        __state = 925;
                    }
                    927 => {
                        if objc > 3 { __state = 930; } else { __state = 929; }
                    }
                    928 => {
                        pp_hook = unsafe { &mut (*p_db).p_rollback_hook };
                        __state = 927;
                    }
                    929 => {
                        db_hook_cmd(interp, p_db,
                            if objc == 3 {
                                unsafe { *objv.offset(2 as isize) }
                            } else { core::ptr::null_mut() }, unsafe { &mut *pp_hook });
                        __state = 932;
                    }
                    930 => {
                        unsafe {
                            Tcl_WrongNumArgs(interp, 2, objv,
                                c"?SCRIPT?".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 931;
                    }
                    931 => { return 1; }
                    932 => { __state = 13; }
                    933 => { __state = 56; }
                    934 => { i__5 = 2; __state = 936; }
                    935 => {
                        if i__5 == 2 { __state = 942; } else { __state = 941; }
                    }
                    936 => {
                        if i__5 < objc { __state = 937; } else { __state = 935; }
                    }
                    937 => {
                        z_arg =
                            unsafe {
                                    Tcl_GetString(unsafe { *objv.offset(i__5 as isize) })
                                } as *const i8;
                        __state = 939;
                    }
                    938 => {
                        { let __p = &mut i__5; let __t = *__p; *__p += 1; __t };
                        __state = 936;
                    }
                    939 => {
                        unsafe {
                            Tcl_AppendResult(interp,
                                c"unknown argument: ".as_ptr() as *mut i8, z_arg,
                                0 as *mut i8)
                        };
                        __state = 940;
                    }
                    940 => { return 1; }
                    941 => { __state = 13; }
                    942 => {
                        unsafe {
                            Tcl_SetResult(interp,
                                unsafe { sqlite3_libversion() } as *mut i8,
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut i8) -> ()>(0 as *const ())
                                })
                        };
                        __state = 941;
                    }
                    _ => {}
                }
            }
        }

        /// don't leave trailing commas on DB_enum, it confuses the AIX xlc compiler
        ///    $db authorizer ?CALLBACK?
        ///*
        ///* Invoke the given callback to authorize each SQL operation as it is
        ///* compiled.  5 arguments are appended to the callback before it is
        ///* invoked:
        ///*
        ///*   (1) The authorization type (ex: SQLITE_CREATE_TABLE, SQLITE_INSERT, ...)
        ///*   (2) First descriptive name (depends on authorization type)
        ///*   (3) Second descriptive name
        ///*   (4) Name of the database (ex: "main", "temp")
        ///*   (5) Name of trigger that is doing the access
        ///*
        ///* The callback should return one of the following strings: SQLITE_OK,
        ///* SQLITE_IGNORE, or SQLITE_DENY.  Any other return value is an error.
        ///*
        ///* If this method is invoked with no arguments, the current authorization
        ///* callback string is returned.
        ///    $db backup ?DATABASE? FILENAME
        ///*
        ///* Open or create a database file named FILENAME.  Transfer the
        ///* content of local database DATABASE (default: "main") into the
        ///* FILENAME database.
        ///    $db bind_fallback ?CALLBACK?
        ///*
        ///* When resolving bind parameters in an SQL statement, if the parameter
        ///* cannot be associated with a TCL variable then invoke CALLBACK with a
        ///* single argument that is the name of the parameter and use the return
        ///* value of the CALLBACK as the binding.  If CALLBACK returns something
        ///* other than TCL_OK or TCL_ERROR then bind a NULL.
        ///*
        ///* If CALLBACK is an empty string, then revert to the default behavior 
        ///* which is to set the binding to NULL.
        ///*
        ///* If CALLBACK returns an error, that causes the statement execution to
        ///* abort.  Hence, to configure a connection so that it throws an error
        ///* on an attempt to bind an unknown variable, do something like this:
        ///*
        ///*     proc bind_error {name} {error "no such variable: $name"}
        ///*     db bind_fallback bind_error
        ///    $db busy ?CALLBACK?
        ///*
        ///* Invoke the given callback if an SQL statement attempts to open
        ///* a locked database file.
        ///     $db cache flush
        ///*     $db cache size n
        ///*
        ///* Flush the prepared statement cache, or set the maximum number of
        ///* cached statements.
        ///     $db changes
        ///*
        ///* Return the number of rows that were modified, inserted, or deleted by
        ///* the most recent INSERT, UPDATE or DELETE statement, not including
        ///* any changes made by trigger programs.
        ///    $db close
        ///*
        ///* Shutdown the database
        ///*     $db collate NAME SCRIPT
        ///*
        ///* Create a new SQL collation function called NAME.  Whenever
        ///* that function is called, invoke SCRIPT to evaluate the function.
        ///*     $db collation_needed SCRIPT
        ///*
        ///* Create a new SQL collation function called NAME.  Whenever
        ///* that function is called, invoke SCRIPT to evaluate the function.
        ///    $db commit_hook ?CALLBACK?
        ///*
        ///* Invoke the given callback just before committing every SQL transaction.
        ///* If the callback throws an exception or returns non-zero, then the
        ///* transaction is aborted.  If CALLBACK is an empty string, the callback
        ///* is disabled.
        ///    $db complete SQL
        ///*
        ///* Return TRUE if SQL is a complete SQL statement.  Return FALSE if
        ///* additional lines of input are needed.  This is similar to the
        ///* built-in "info complete" command of Tcl.
        ///    $db config ?OPTION? ?BOOLEAN?
        ///*
        ///* Configure the database connection using the sqlite3_db_config()
        ///* interface.
        /// With no arguments, list all configuration options and with the
        ///* current value
        ///    $db copy conflict-algorithm table filename ?SEPARATOR? ?NULLINDICATOR?
        ///*
        ///* Copy data into table from filename, optionally using SEPARATOR
        ///* as column separators.  If a column contains a null string, or the
        ///* value of NULLINDICATOR, a NULL is inserted for the column.
        ///* conflict-algorithm is one of the sqlite conflict algorithms:
        ///*    rollback, abort, fail, ignore, replace
        ///* On success, return the number of lines processed, not necessarily same
        ///* as 'db changes' due to conflict-algorithm selected.
        ///*
        ///* This code is basically an implementation/enhancement of
        ///* the sqlite3 shell.c ".import" command.
        ///*
        ///* This command usage is equivalent to the sqlite2.x COPY statement,
        ///* which imports file data into a table using the PostgreSQL COPY file format:
        ///*   $db copy $conflict_algorithm $table_name $filename \t \\N
        /// Insert data into this table
        /// The file from which to extract data
        /// The conflict algorithm to use
        /// A statement
        /// Number of columns in the table
        /// Number of bytes in an SQL string
        /// Loop counters
        /// Number of bytes in zSep[]
        /// Number of bytes in zNull[]
        /// An SQL statement
        /// A single line of input from the file
        /// zLine[] broken up into columns
        /// How to commit changes
        /// The input file
        /// Line number of input file
        /// Line number print buffer
        /// interp result
        /// check for null data, if so, bind as null
        /// success, set result as number of lines processed
        /// failure, append lineno where failed
        ///*     $db deserialize ?-maxsize N? ?-readonly BOOL? ?DATABASE? VALUE
        ///*
        ///* Reopen DATABASE (default "main") using the content in $VALUE
        ///*    $db enable_load_extension BOOLEAN
        ///*
        ///* Turn the extension loading feature on or off.  It if off by
        ///* default.
        ///*    $db errorcode
        ///*
        ///* Return the numeric error code that was returned by the most recent
        ///* call to sqlite3_exec().
        ///*    $db erroroffset
        ///*
        ///* Return the numeric error code that was returned by the most recent
        ///* call to sqlite3_exec().
        ///*    $db exists $sql
        ///*    $db onecolumn $sql
        ///*
        ///* The onecolumn method is the equivalent of:
        ///*     lindex [$db eval $sql] 0
        ///*    $db eval ?options? $sql ?varName? ?{  ...code... }?
        ///*
        ///* The SQL statement in $sql is evaluated.  For each row, the values
        ///* are placed in elements of the array or dict named $varName and
        ///* ...code... is executed.  If $varName and $code are omitted, then
        ///* no callback is ever invoked.  If $varName is an empty string,
        ///* then the values are placed in variables that have the same name
        ///* as the fields extracted by the query, and those variables are
        ///* accessible during the eval of $code.
        ///*     $db format [OPTIONS] SQL
        ///*
        ///* Run the SQL statement(s) given as the final argument.  Use the
        ///* Query Result Formatter extension of SQLite to format the output as
        ///* text and return that text.
        ///*     $db function NAME [OPTIONS] SCRIPT
        ///*
        ///* Create a new SQL function called NAME.  Whenever that function is
        ///* called, invoke SCRIPT to evaluate the function.
        ///*
        ///* Options:
        ///*         --argcount N           Function has exactly N arguments
        ///*         --deterministic        The function is pure
        ///*         --directonly           Prohibit use inside triggers and views
        ///*         --innocuous            Has no side effects or information leaks
        ///*         --returntype TYPE      Specify the return type of the function
        ///*     $db incrblob ?-readonly? ?DB? TABLE COLUMN ROWID
        /// Check for the -readonly option
        ///*     $db interrupt
        ///*
        ///* Interrupt the execution of the inner-most SQL interpreter.  This
        ///* causes the SQL statement to return an error of SQLITE_INTERRUPT.
        ///*     $db nullvalue ?STRING?
        ///*
        ///* Change text used when a NULL comes back from the database. If ?STRING?
        ///* is not present, then the current string used for NULL is returned.
        ///* If STRING is present, then STRING is returned.
        ///*
        ///*     $db last_insert_rowid
        ///*
        ///* Return an integer which is the ROWID for the most recent insert.
        ///* The DB_ONECOLUMN method is implemented together with DB_EXISTS.
        ///    $db progress ?N CALLBACK?
        ///*
        ///* Invoke the given callback every N virtual machine opcodes while executing
        ///* queries.
        ///    $db profile ?CALLBACK?
        ///*
        ///* Make arrangements to invoke the CALLBACK routine after each SQL statement
        ///* that has run.  The text of the SQL and the amount of elapse time are
        ///* appended to CALLBACK before the script is run.
        ///*     $db rekey KEY
        ///*
        ///* Change the encryption key on the currently open database.
        ///    $db restore ?DATABASE? FILENAME
        ///*
        ///* Open a database file named FILENAME.  Transfer the content
        ///* of FILENAME into the local database DATABASE (default: "main").
        ///*     $db serialize ?DATABASE?
        ///*
        ///* Return a serialization of a database.
        ///*     $db status (step|sort|autoindex|vmstep)
        ///*
        ///* Display SQLITE_STMTSTATUS_FULLSCAN_STEP or
        ///* SQLITE_STMTSTATUS_SORT for the most recent eval.
        ///*     $db timeout MILLESECONDS
        ///*
        ///* Delay for the number of milliseconds specified when a file is locked.
        ///*     $db total_changes
        ///*
        ///* Return the number of rows that were modified, inserted, or deleted
        ///* since the database handle was created.
        ///    $db trace ?CALLBACK?
        ///*
        ///* Make arrangements to invoke the CALLBACK routine for each SQL statement
        ///* that is executed.  The text of the SQL is appended to CALLBACK before
        ///* it is executed.
        ///    $db trace_v2 ?CALLBACK? ?MASK?
        ///*
        ///* Make arrangements to invoke the CALLBACK routine for each trace event
        ///* matching the mask that is generated.  The parameters are appended to
        ///* CALLBACK before it is executed.
        /// use the "legacy" default
        ///    $db transaction [-deferred|-immediate|-exclusive] SCRIPT
        ///*
        ///* Start a new transaction (if we are not already in the midst of a
        ///* transaction) and execute the TCL script SCRIPT.  After SCRIPT
        ///* completes, either commit the transaction or roll it back if SCRIPT
        ///* throws an exception.  Or if no new transaction was started, do nothing.
        ///* pass the exception on up the stack.
        ///*
        ///* This command was inspired by Dave Thomas's talk on Ruby at the
        ///* 2005 O'Reilly Open Source Convention (OSCON).
        /// no-op
        /// Run the SQLite BEGIN command to open a transaction or savepoint.
        /// If using NRE, schedule a callback to invoke the script pScript, then
        ///* a second callback to commit (or rollback) the transaction or savepoint
        ///* opened above. If not using NRE, evaluate the script directly, then
        ///* call function DbTransPostCmd() to commit (or rollback) the transaction
        ///* or savepoint.
        /// DbTransPostCmd() calls delDatabaseRef()
        ///*    $db unlock_notify ?script?
        ///*    $db preupdate_hook count
        ///*    $db preupdate_hook hook ?SCRIPT?
        ///*    $db preupdate_hook new INDEX
        ///*    $db preupdate_hook old INDEX
        /// SQLITE_ENABLE_PREUPDATE_HOOK
        ///*    $db wal_hook ?script?
        ///*    $db update_hook ?script?
        ///*    $db rollback_hook ?script?
        /// set ppHook to point at pUpdateHook or pRollbackHook, depending on
        ///* whether [$db update_hook] or [$db rollback_hook] was invoked.
        ///    $db version
        ///*
        ///* Return the version string for this database.
        /// Optional arguments to $db version are used for testing purpose
        /// SQLITE_TEST
        /// End of the SWITCH statement
        unreachable!();
    }
}

///* Issue the usage message when the "sqlite3" command arguments are
///* incorrect.
extern "C" fn sqlite_cmd_usage(interp: *mut TclInterp,
    objv: *const *mut TclObj) -> i32 {
    unsafe {
        Tcl_WrongNumArgs(interp, 1, objv,
            c"HANDLE ?FILENAME? ?-vfs VFSNAME? ?-readonly BOOLEAN? ?-create BOOLEAN? ?-nofollow BOOLEAN? ?-nomutex BOOLEAN? ?-fullmutex BOOLEAN? ?-uri BOOLEAN?".as_ptr()
                    as *mut i8 as *const i8)
    };
    return 1;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_unload(interp: *const TclInterp, flags: i32)
    -> i32 {
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn tclsqlite3_unload(interp: *const TclInterp, flags: i32)
    -> i32 {
    return 0;
}

/// Because it accesses the file-system and uses persistent state, SQLite
///* is not considered appropriate for safe interpreters.  Hence, we cause
///* the _SafeInit() interfaces return TCL_ERROR.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_safe_init(interp: *const TclInterp) -> i32 {
    return 1;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_safe_unload(interp: *const TclInterp, flags: i32)
    -> i32 {
    return 1;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite_unload(interp: *const TclInterp, flags: i32) -> i32 {
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn tclsqlite_unload(interp: *const TclInterp, flags: i32)
    -> i32 {
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite_safe_init(interp: *const TclInterp) -> i32 {
    return 1;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite_safe_unload(interp: *const TclInterp, flags: i32)
    -> i32 {
    return 1;
}

const DB_AUTHORIZER: u32 = 0;

const DB_BACKUP: u32 = 1;

const DB_BIND_FALLBACK: u32 = 2;

const DB_BUSY: u32 = 3;

const DB_CACHE: u32 = 4;

const DB_CHANGES: u32 = 5;

const DB_CLOSE: u32 = 6;

const DB_COLLATE: u32 = 7;

const DB_COLLATION_NEEDED: u32 = 8;

const DB_COMMIT_HOOK: u32 = 9;

const DB_COMPLETE: u32 = 10;

const DB_CONFIG: u32 = 11;

const DB_COPY: u32 = 12;

const DB_DESERIALIZE: u32 = 13;

const DB_ENABLE_LOAD_EXTENSION: u32 = 14;

const DB_ERRORCODE: u32 = 15;

const DB_ERROROFFSET: u32 = 16;

const DB_EXISTS: u32 = 18;

const DB_ONECOLUMN: u32 = 25;

const DB_EVAL: u32 = 17;

const DB_FORMAT: u32 = 19;

const DB_FUNCTION: u32 = 20;

const DB_INCRBLOB: u32 = 21;

const DB_INTERRUPT: u32 = 22;

const DB_NULLVALUE: u32 = 24;

const DB_LAST_INSERT_ROWID: u32 = 23;

const DB_PROGRESS: u32 = 28;

const DB_PROFILE: u32 = 27;

const DB_REKEY: u32 = 29;

const DB_RESTORE: u32 = 30;

const DB_SERIALIZE: u32 = 32;

const DB_STATUS: u32 = 33;

const DB_TIMEOUT: u32 = 34;

const DB_TOTAL_CHANGES: u32 = 35;

const DB_TRACE: u32 = 36;

const DB_TRACE_V2: u32 = 37;

const DB_TRANSACTION: u32 = 38;

const DB_UNLOCK_NOTIFY: u32 = 39;

const DB_PREUPDATE: u32 = 26;

const DB_WAL_HOOK: u32 = 42;

const DB_UPDATE_HOOK: u32 = 40;

const DB_ROLLBACK_HOOK: u32 = 31;

const DB_VERSION: u32 = 41;

type Sqlite3AuthCb =
    unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, *const i8,
        *const i8) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
struct DbConfigChoicesN15DbConfigChoices {
    z_name: *const i8,
    op: i32,
}

const TTYPE_STMT: u32 = 0;

const TTYPE_PROFILE: u32 = 1;

const TTYPE_ROW: u32 = 2;

const TTYPE_CLOSE: u32 = 3;

const TTYPE_DEFERRED: u32 = 0;

const TTYPE_EXCLUSIVE: u32 = 1;

const TTYPE_IMMEDIATE: u32 = 2;

static mut count: i32 = 0;

static mut az_str: [*const i8; 3] =
    [c"DELETE".as_ptr() as *const i8, c"INSERT".as_ptr() as *const i8,
            c"UPDATE".as_ptr() as *const i8];

static mut az_end: [*const i8; 4] =
    [c"RELEASE _tcl_transaction".as_ptr() as *const i8,
            c"COMMIT".as_ptr() as *const i8,
            c"ROLLBACK TO _tcl_transaction ; RELEASE _tcl_transaction".as_ptr()
                as *const i8, c"ROLLBACK".as_ptr() as *const i8];

static mut db_strs: [*const i8; 44] =
    [c"authorizer".as_ptr() as *const i8, c"backup".as_ptr() as *const i8,
            c"bind_fallback".as_ptr() as *const i8,
            c"busy".as_ptr() as *const i8, c"cache".as_ptr() as *const i8,
            c"changes".as_ptr() as *const i8, c"close".as_ptr() as *const i8,
            c"collate".as_ptr() as *const i8,
            c"collation_needed".as_ptr() as *const i8,
            c"commit_hook".as_ptr() as *const i8,
            c"complete".as_ptr() as *const i8,
            c"config".as_ptr() as *const i8, c"copy".as_ptr() as *const i8,
            c"deserialize".as_ptr() as *const i8,
            c"enable_load_extension".as_ptr() as *const i8,
            c"errorcode".as_ptr() as *const i8,
            c"erroroffset".as_ptr() as *const i8,
            c"eval".as_ptr() as *const i8, c"exists".as_ptr() as *const i8,
            c"format".as_ptr() as *const i8,
            c"function".as_ptr() as *const i8,
            c"incrblob".as_ptr() as *const i8,
            c"interrupt".as_ptr() as *const i8,
            c"last_insert_rowid".as_ptr() as *const i8,
            c"nullvalue".as_ptr() as *const i8,
            c"onecolumn".as_ptr() as *const i8,
            c"preupdate".as_ptr() as *const i8,
            c"profile".as_ptr() as *const i8,
            c"progress".as_ptr() as *const i8, c"rekey".as_ptr() as *const i8,
            c"restore".as_ptr() as *const i8,
            c"rollback_hook".as_ptr() as *const i8,
            c"serialize".as_ptr() as *const i8,
            c"status".as_ptr() as *const i8, c"timeout".as_ptr() as *const i8,
            c"total_changes".as_ptr() as *const i8,
            c"trace".as_ptr() as *const i8, c"trace_v2".as_ptr() as *const i8,
            c"transaction".as_ptr() as *const i8,
            c"unlock_notify".as_ptr() as *const i8,
            c"update_hook".as_ptr() as *const i8,
            c"version".as_ptr() as *const i8,
            c"wal_hook".as_ptr() as *const i8, core::ptr::null()];

static mut a_db_config: [DbConfigChoicesN15DbConfigChoices; 16] =
    [DbConfigChoicesN15DbConfigChoices {
                z_name: c"defensive".as_ptr() as *const i8,
                op: 1010,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"dqs_ddl".as_ptr() as *const i8,
                op: 1014,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"dqs_dml".as_ptr() as *const i8,
                op: 1013,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"enable_fkey".as_ptr() as *const i8,
                op: 1002,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"enable_qpsg".as_ptr() as *const i8,
                op: 1007,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"enable_trigger".as_ptr() as *const i8,
                op: 1003,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"enable_view".as_ptr() as *const i8,
                op: 1015,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"fts3_tokenizer".as_ptr() as *const i8,
                op: 1004,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"legacy_alter_table".as_ptr() as *const i8,
                op: 1012,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"legacy_file_format".as_ptr() as *const i8,
                op: 1016,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"load_extension".as_ptr() as *const i8,
                op: 1005,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"no_ckpt_on_close".as_ptr() as *const i8,
                op: 1006,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"reset_database".as_ptr() as *const i8,
                op: 1009,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"trigger_eqp".as_ptr() as *const i8,
                op: 1008,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"trusted_schema".as_ptr() as *const i8,
                op: 1017,
            },
            DbConfigChoicesN15DbConfigChoices {
                z_name: c"writable_schema".as_ptr() as *const i8,
                op: 1011,
            }];

static mut ttype_strs: [*const i8; 5] =
    [c"statement".as_ptr() as *const i8, c"profile".as_ptr() as *const i8,
            c"row".as_ptr() as *const i8, c"close".as_ptr() as *const i8,
            core::ptr::null()];

static mut ttype_strs_1: [*const i8; 4] =
    [c"deferred".as_ptr() as *const i8, c"exclusive".as_ptr() as *const i8,
            c"immediate".as_ptr() as *const i8, core::ptr::null()];

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
    fn Tcl_UnregisterChannel(interp: *mut TclInterp, chan: TclChannel)
    -> i32;
    fn Tcl_Free(ptr: *mut i8)
    -> ();
    fn Tcl_SetResult(interp: *mut TclInterp, result: *mut i8,
    freeProc: unsafe extern "C" fn(*mut i8) -> ())
    -> ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn Tcl_Alloc(size: u32)
    -> *mut i8;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn Tcl_CreateChannel(typePtr: *mut TclChannelType, chanName: *const i8,
    instanceData: ClientData, mask: i32)
    -> TclChannel;
    fn Tcl_RegisterChannel(interp: *mut TclInterp, chan: TclChannel)
    -> ();
    fn Tcl_GetChannelName(chan: TclChannel)
    -> *const i8;
    fn Tcl_GetStringFromObj(objPtr: *mut TclObj, lengthPtr: *mut i32)
    -> *mut i8;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn TclFreeObj(objPtr: *mut TclObj)
    -> ();
    fn Tcl_VarEval(interp: *mut TclInterp, ...)
    -> i32;
    fn Tcl_GetStringResult(interp: *mut TclInterp)
    -> *const i8;
    fn atoi(_: *const i8)
    -> i32;
    fn Tcl_Eval(interp: *mut TclInterp, script: *const i8)
    -> i32;
    fn Tcl_DStringInit(dsPtr: *mut TclDString)
    -> ();
    fn Tcl_DStringAppend(dsPtr: *mut TclDString, bytes: *const i8,
    length: i32)
    -> *mut i8;
    fn Tcl_DStringAppendElement(dsPtr: *mut TclDString, element: *const i8)
    -> *mut i8;
    fn Tcl_DStringFree(dsPtr: *mut TclDString)
    -> ();
    fn Tcl_ResetResult(interp: *mut TclInterp)
    -> ();
    fn Tcl_NewStringObj(bytes: *const i8, length: i32)
    -> *mut TclObj;
    fn Tcl_NewWideIntObj(wideValue: TclWideInt)
    -> *mut TclObj;
    fn Tcl_ListObjAppendElement(interp: *mut TclInterp, listPtr: *mut TclObj,
    objPtr: *mut TclObj)
    -> i32;
    fn Tcl_EvalObjEx(interp: *mut TclInterp, objPtr: *mut TclObj, flags: i32)
    -> i32;
    fn Tcl_BackgroundError(interp: *mut TclInterp)
    -> ();
    fn Tcl_DuplicateObj(objPtr: *mut TclObj)
    -> *mut TclObj;
    fn Tcl_NewIntObj(intValue: i32)
    -> *mut TclObj;
    fn Tcl_GetObjResult(interp: *mut TclInterp)
    -> *mut TclObj;
    fn Tcl_GetIntFromObj(interp: *mut TclInterp, objPtr: *mut TclObj,
    intPtr: *mut i32)
    -> i32;
    fn Tcl_ListObjGetElements(interp: *mut TclInterp, listPtr: *mut TclObj,
    objcPtr: *mut i32, objvPtr: *mut *mut *mut TclObj)
    -> i32;
    fn Tcl_NewListObj(objc: i32, objv: *const *mut TclObj)
    -> *mut TclObj;
    fn Tcl_NewByteArrayObj(bytes: *const u8, length: i32)
    -> *mut TclObj;
    fn Tcl_NewDoubleObj(doubleValue: f64)
    -> *mut TclObj;
    fn Tcl_GetString(objPtr: *mut TclObj)
    -> *mut i8;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn Tcl_GetByteArrayFromObj(objPtr: *mut TclObj, lengthPtr: *mut i32)
    -> *mut u8;
    fn Tcl_GetWideIntFromObj(interp: *mut TclInterp, objPtr: *mut TclObj,
    widePtr: *mut TclWideInt)
    -> i32;
    fn Tcl_GetDoubleFromObj(interp: *mut TclInterp, objPtr: *mut TclObj,
    doublePtr: *mut f64)
    -> i32;
    fn Tcl_GlobalEval(interp: *mut TclInterp, command: *const i8)
    -> i32;
    fn Tcl_AppendResult(interp: *mut TclInterp, ...)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn Tcl_SetObjResult(interp: *mut TclInterp, resultObjPtr: *mut TclObj)
    -> ();
    fn Tcl_GetVar2Ex(interp: *mut TclInterp, part1: *const i8,
    part2: *const i8, flags: i32)
    -> *mut TclObj;
    fn Tcl_GetBooleanFromObj(interp: *mut TclInterp, objPtr: *mut TclObj,
    boolPtr: *mut i32)
    -> i32;
    fn Tcl_NewObj()
    -> *mut TclObj;
    fn Tcl_ObjSetVar2(interp: *mut TclInterp, part1Ptr: *mut TclObj,
    part2Ptr: *mut TclObj, newValuePtr: *mut TclObj, flags: i32)
    -> *mut TclObj;
    fn Tcl_ObjGetVar2(interp: *mut TclInterp, part1Ptr: *mut TclObj,
    part2Ptr: *mut TclObj, flags: i32)
    -> *mut TclObj;
    fn Tcl_NewDictObj()
    -> *mut TclObj;
    fn Tcl_DictObjPut(interp: *mut TclInterp, dictPtr: *mut TclObj,
    keyPtr: *mut TclObj, valuePtr: *mut TclObj)
    -> i32;
    fn Tcl_UnsetVar2(interp: *mut TclInterp, part1: *const i8,
    part2: *const i8, flags: i32)
    -> i32;
    fn Tcl_DictObjRemove(interp: *mut TclInterp, dictPtr: *mut TclObj,
    keyPtr: *mut TclObj)
    -> i32;
    fn Tcl_GetIndexFromObj(interp: *mut TclInterp, objPtr: *mut TclObj,
    tablePtr: *mut *const i8, msg: *const i8, flags: i32, indexPtr: *mut i32)
    -> i32;
    fn Tcl_WrongNumArgs(interp: *mut TclInterp, objc: i32,
    objv: *const *mut TclObj, message: *const i8)
    -> ();
    fn Tcl_DeleteCommand(interp: *mut TclInterp, cmdName: *const i8)
    -> i32;
    fn Tcl_SetWideIntObj(objPtr: *mut TclObj, wideValue: TclWideInt)
    -> ();
    fn Tcl_SetBooleanObj(objPtr: *mut TclObj, boolValue: i32)
    -> ();
    fn malloc(__size: u64)
    -> *mut ();
    fn free(_: *mut ())
    -> ();
    fn Tcl_OpenFileChannel(interp: *mut TclInterp, fileName: *const i8,
    modeString: *const i8, permissions: i32)
    -> TclChannel;
    fn Tcl_SetChannelOption(interp: *mut TclInterp, chan: TclChannel,
    optionName: *const i8, newValue: *const i8)
    -> i32;
    fn Tcl_Close(interp: *mut TclInterp, chan: TclChannel)
    -> i32;
    fn Tcl_GetsObj(chan: TclChannel, objPtr: *mut TclObj)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn Tcl_SetObjLength(objPtr: *mut TclObj, length: i32)
    -> ();
    fn Tcl_SetIntObj(objPtr: *mut TclObj, intValue: i32)
    -> ();
    fn Tcl_NewBooleanObj(boolValue: i32)
    -> *mut TclObj;
    fn Tcl_ListObjLength(interp: *mut TclInterp, listPtr: *mut TclObj,
    lengthPtr: *mut i32)
    -> i32;
    fn Tcl_ListObjIndex(interp: *mut TclInterp, listPtr: *mut TclObj,
    index: i32, objPtrPtr: *mut *mut TclObj)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Tcl_ChannelTypeVersion_ {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Tcl_Channel_ {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TclInterp {
    _opaque: [u8; 0],
}
