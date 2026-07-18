#![feature(c_variadic)]
type __darwin_size_t = u64;
type __int32_t = i32;
type __darwin_dev_t = __int32_t;
type dev_t = __darwin_dev_t;
type __uint16_t = u16;
type __darwin_mode_t = __uint16_t;
type mode_t = __darwin_mode_t;
type nlink_t = __uint16_t;
type __uint64_t = u64;
type __darwin_ino64_t = __uint64_t;
type __uint32_t = u32;
type __darwin_uid_t = __uint32_t;
type uid_t = __darwin_uid_t;
type __darwin_gid_t = __uint32_t;
type gid_t = __darwin_gid_t;
type __int64_t = i64;
type __darwin_off_t = __int64_t;
type off_t = __darwin_off_t;
type __darwin_blkcnt_t = __int64_t;
type blkcnt_t = __darwin_blkcnt_t;
type __darwin_blksize_t = __int32_t;
type blksize_t = __darwin_blksize_t;
type __darwin_time_t = i64;
type time_t = __darwin_time_t;
type __darwin_ssize_t = i64;
type __darwin_suseconds_t = __int32_t;
type __uint8_t = u8;
#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atimespec: timespec,
    st_mtimespec: timespec,
    st_ctimespec: timespec,
    st_birthtimespec: timespec,
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
struct timeval {
    tv_sec: i64,
    tv_usec: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct dirent {
    d_ino: u64,
    d_seekoff: u64,
    d_reclen: u16,
    d_namlen: u16,
    d_type: u8,
    d_name: [i8; 1024],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3 {
    _opaque: [u8; 0],
}
type sqlite_int64 = i64;
type sqlite_uint64 = u64;
type sqlite3_int64 = sqlite_int64;
type sqlite3_uint64 = sqlite_uint64;
type sqlite3_callback =
    unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_file {
    p_methods: *const sqlite3_io_methods,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_io_methods {
    i_version: i32,
    x_close: Option<unsafe extern "C" fn(*mut sqlite3_file) -> i32>,
    x_read: Option<unsafe extern "C" fn(*mut sqlite3_file, *mut (), i32, i64)
        -> i32>,
    x_write: Option<unsafe extern "C" fn(*mut sqlite3_file, *const (), i32,
        i64) -> i32>,
    x_truncate: Option<unsafe extern "C" fn(*mut sqlite3_file, i64) -> i32>,
    x_sync: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_file_size: Option<unsafe extern "C" fn(*mut sqlite3_file, *mut i64)
        -> i32>,
    x_lock: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_unlock: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_check_reserved_lock: Option<unsafe extern "C" fn(*mut sqlite3_file,
        *mut i32) -> i32>,
    x_file_control: Option<unsafe extern "C" fn(*mut sqlite3_file, i32,
        *mut ()) -> i32>,
    x_sector_size: Option<unsafe extern "C" fn(*mut sqlite3_file) -> i32>,
    x_device_characteristics: Option<unsafe extern "C" fn(*mut sqlite3_file)
        -> i32>,
    x_shm_map: Option<unsafe extern "C" fn(*mut sqlite3_file, i32, i32, i32,
        *mut *mut ()) -> i32>,
    x_shm_lock: Option<unsafe extern "C" fn(*mut sqlite3_file, i32, i32, i32)
        -> i32>,
    x_shm_barrier: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>,
    x_shm_unmap: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_fetch: Option<unsafe extern "C" fn(*mut sqlite3_file, i64, i32,
        *mut *mut ()) -> i32>,
    x_unfetch: Option<unsafe extern "C" fn(*mut sqlite3_file, i64, *mut ())
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_mutex {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_api_routines {
    aggregate_context: Option<unsafe extern "C" fn(*mut sqlite3_context, i32)
        -> *mut ()>,
    aggregate_count: Option<unsafe extern "C" fn(*mut sqlite3_context)
        -> i32>,
    bind_blob: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, *const (),
        i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    bind_double: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, f64)
        -> i32>,
    bind_int: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, i32)
        -> i32>,
    bind_int64: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, i64)
        -> i32>,
    bind_null: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32) -> i32>,
    bind_parameter_count: Option<unsafe extern "C" fn(*mut sqlite3_stmt)
        -> i32>,
    bind_parameter_index: Option<unsafe extern "C" fn(*mut sqlite3_stmt,
        *const i8) -> i32>,
    bind_parameter_name: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const i8>,
    bind_text: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, *const i8,
        i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    bind_text16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    bind_value: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32,
        *const sqlite3_value) -> i32>,
    busy_handler: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), i32) -> i32, *mut ()) -> i32>,
    busy_timeout: Option<unsafe extern "C" fn(*mut sqlite3, i32) -> i32>,
    changes: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    close: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    collation_needed: Option<unsafe extern "C" fn(*mut sqlite3, *mut (),
        unsafe extern "C" fn(*mut (), *mut sqlite3, i32, *const i8) -> ())
        -> i32>,
    collation_needed16: Option<unsafe extern "C" fn(*mut sqlite3, *mut (),
        unsafe extern "C" fn(*mut (), *mut sqlite3, i32, *const ()) -> ())
        -> i32>,
    column_blob: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const ()>,
    column_bytes: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_bytes16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> i32>,
    column_count: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    column_database_name: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const i8>,
    column_database_name16: Option<unsafe extern "C" fn(*mut sqlite3_stmt,
        i32) -> *const ()>,
    column_decltype: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const i8>,
    column_decltype16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const ()>,
    column_double: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> f64>,
    column_int: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_int64: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32) -> i64>,
    column_name: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const i8>,
    column_name16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const ()>,
    column_origin_name: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const i8>,
    column_origin_name16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const ()>,
    column_table_name: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const i8>,
    column_table_name16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const ()>,
    column_text: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const u8>,
    column_text16: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *const ()>,
    column_type: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_value: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32)
        -> *mut sqlite3_value>,
    commit_hook: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut ()) -> i32, *mut ()) -> *mut ()>,
    complete: Option<unsafe extern "C" fn(*const i8) -> i32>,
    complete16: Option<unsafe extern "C" fn(*const ()) -> i32>,
    create_collation: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ()) -> i32)
        -> i32>,
    create_collation16: Option<unsafe extern "C" fn(*mut sqlite3, *const (),
        i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ()) -> i32)
        -> i32>,
    create_function: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, i32,
        i32, *mut (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context) -> ()) -> i32>,
    create_function16: Option<unsafe extern "C" fn(*mut sqlite3, *const (),
        i32, i32, *mut (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context) -> ()) -> i32>,
    create_module: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        *const sqlite3_module, *mut ()) -> i32>,
    data_count: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    db_handle: Option<unsafe extern "C" fn(*mut sqlite3_stmt)
        -> *mut sqlite3>,
    declare_vtab: Option<unsafe extern "C" fn(*mut sqlite3, *const i8)
        -> i32>,
    enable_shared_cache: Option<unsafe extern "C" fn(i32) -> i32>,
    errcode: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    errmsg: Option<unsafe extern "C" fn(*mut sqlite3) -> *const i8>,
    errmsg16: Option<unsafe extern "C" fn(*mut sqlite3) -> *const ()>,
    exec: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        *mut (), *mut *mut i8) -> i32>,
    expired: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    finalize: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    free: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    free_table: Option<unsafe extern "C" fn(*mut *mut i8) -> ()>,
    get_autocommit: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    get_auxdata: Option<unsafe extern "C" fn(*mut sqlite3_context, i32)
        -> *mut ()>,
    get_table: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        *mut *mut *mut i8, *mut i32, *mut i32, *mut *mut i8) -> i32>,
    global_recover: Option<unsafe extern "C" fn() -> i32>,
    interruptx: Option<unsafe extern "C" fn(*mut sqlite3) -> ()>,
    last_insert_rowid: Option<unsafe extern "C" fn(*mut sqlite3) -> i64>,
    libversion: Option<unsafe extern "C" fn() -> *const i8>,
    libversion_number: Option<unsafe extern "C" fn() -> i32>,
    malloc: Option<unsafe extern "C" fn(i32) -> *mut ()>,
    mprintf: Option<unsafe extern "C" fn(*const i8, ...) -> *mut i8>,
    open: Option<unsafe extern "C" fn(*const i8, *mut *mut sqlite3) -> i32>,
    open16: Option<unsafe extern "C" fn(*const (), *mut *mut sqlite3) -> i32>,
    prepare: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, i32,
        *mut *mut sqlite3_stmt, *mut *const i8) -> i32>,
    prepare16: Option<unsafe extern "C" fn(*mut sqlite3, *const (), i32,
        *mut *mut sqlite3_stmt, *mut *const ()) -> i32>,
    profile: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), *const i8, u64) -> (), *mut ())
        -> *mut ()>,
    progress_handler: Option<unsafe extern "C" fn(*mut sqlite3, i32,
        unsafe extern "C" fn(*mut ()) -> i32, *mut ()) -> ()>,
    realloc: Option<unsafe extern "C" fn(*mut (), i32) -> *mut ()>,
    reset: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    result_blob: Option<unsafe extern "C" fn(*mut sqlite3_context, *const (),
        i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    result_double: Option<unsafe extern "C" fn(*mut sqlite3_context, f64)
        -> ()>,
    result_error: Option<unsafe extern "C" fn(*mut sqlite3_context, *const i8,
        i32) -> ()>,
    result_error16: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *const (), i32) -> ()>,
    result_int: Option<unsafe extern "C" fn(*mut sqlite3_context, i32) -> ()>,
    result_int64: Option<unsafe extern "C" fn(*mut sqlite3_context, i64)
        -> ()>,
    result_null: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    result_text: Option<unsafe extern "C" fn(*mut sqlite3_context, *const i8,
        i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    result_text16: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    result_text16be: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    result_text16le: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    result_value: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *mut sqlite3_value) -> ()>,
    rollback_hook: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut ()) -> (), *mut ()) -> *mut ()>,
    set_authorizer: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, *const i8,
                *const i8) -> i32, *mut ()) -> i32>,
    set_auxdata: Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
        *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    xsnprintf: Option<unsafe extern "C" fn(i32, *mut i8, *const i8, ...)
        -> *mut i8>,
    step: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    table_column_metadata: Option<unsafe extern "C" fn(*mut sqlite3,
        *const i8, *const i8, *const i8, *mut *const i8, *mut *const i8,
        *mut i32, *mut i32, *mut i32) -> i32>,
    thread_cleanup: Option<unsafe extern "C" fn() -> ()>,
    total_changes: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    trace: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), *const i8) -> (), *mut ()) -> *mut ()>,
    transfer_bindings: Option<unsafe extern "C" fn(*mut sqlite3_stmt,
        *mut sqlite3_stmt) -> i32>,
    update_hook: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64) -> (),
        *mut ()) -> *mut ()>,
    user_data: Option<unsafe extern "C" fn(*mut sqlite3_context) -> *mut ()>,
    value_blob: Option<unsafe extern "C" fn(*mut sqlite3_value) -> *const ()>,
    value_bytes: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    value_bytes16: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    value_double: Option<unsafe extern "C" fn(*mut sqlite3_value) -> f64>,
    value_int: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    value_int64: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i64>,
    value_numeric_type: Option<unsafe extern "C" fn(*mut sqlite3_value)
        -> i32>,
    value_text: Option<unsafe extern "C" fn(*mut sqlite3_value) -> *const u8>,
    value_text16: Option<unsafe extern "C" fn(*mut sqlite3_value)
        -> *const ()>,
    value_text16be: Option<unsafe extern "C" fn(*mut sqlite3_value)
        -> *const ()>,
    value_text16le: Option<unsafe extern "C" fn(*mut sqlite3_value)
        -> *const ()>,
    value_type: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    vmprintf: Option<unsafe extern "C" fn(*const i8, *mut i8) -> *mut i8>,
    overload_function: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        i32) -> i32>,
    prepare_v2: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, i32,
        *mut *mut sqlite3_stmt, *mut *const i8) -> i32>,
    prepare16_v2: Option<unsafe extern "C" fn(*mut sqlite3, *const (), i32,
        *mut *mut sqlite3_stmt, *mut *const ()) -> i32>,
    clear_bindings: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    create_module_v2: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        *const sqlite3_module, *mut (), unsafe extern "C" fn(*mut ()) -> ())
        -> i32>,
    bind_zeroblob: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, i32)
        -> i32>,
    blob_bytes: Option<unsafe extern "C" fn(*mut sqlite3_blob) -> i32>,
    blob_close: Option<unsafe extern "C" fn(*mut sqlite3_blob) -> i32>,
    blob_open: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, *const i8,
        *const i8, i64, i32, *mut *mut sqlite3_blob) -> i32>,
    blob_read: Option<unsafe extern "C" fn(*mut sqlite3_blob, *mut (), i32,
        i32) -> i32>,
    blob_write: Option<unsafe extern "C" fn(*mut sqlite3_blob, *const (), i32,
        i32) -> i32>,
    create_collation_v2: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    file_control: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, i32,
        *mut ()) -> i32>,
    memory_highwater: Option<unsafe extern "C" fn(i32) -> i64>,
    memory_used: Option<unsafe extern "C" fn() -> i64>,
    mutex_alloc: Option<unsafe extern "C" fn(i32) -> *mut sqlite3_mutex>,
    mutex_enter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    mutex_free: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    mutex_leave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    mutex_try: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
    open_v2: Option<unsafe extern "C" fn(*const i8, *mut *mut sqlite3, i32,
        *const i8) -> i32>,
    release_memory: Option<unsafe extern "C" fn(i32) -> i32>,
    result_error_nomem: Option<unsafe extern "C" fn(*mut sqlite3_context)
        -> ()>,
    result_error_toobig: Option<unsafe extern "C" fn(*mut sqlite3_context)
        -> ()>,
    sleep: Option<unsafe extern "C" fn(i32) -> i32>,
    soft_heap_limit: Option<unsafe extern "C" fn(i32) -> ()>,
    vfs_find: Option<unsafe extern "C" fn(*const i8) -> *mut sqlite3_vfs>,
    vfs_register: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32) -> i32>,
    vfs_unregister: Option<unsafe extern "C" fn(*mut sqlite3_vfs) -> i32>,
    xthreadsafe: Option<unsafe extern "C" fn() -> i32>,
    result_zeroblob: Option<unsafe extern "C" fn(*mut sqlite3_context, i32)
        -> ()>,
    result_error_code: Option<unsafe extern "C" fn(*mut sqlite3_context, i32)
        -> ()>,
    test_control: Option<unsafe extern "C" fn(i32, ...) -> i32>,
    randomness: Option<unsafe extern "C" fn(i32, *mut ()) -> ()>,
    context_db_handle: Option<unsafe extern "C" fn(*mut sqlite3_context)
        -> *mut sqlite3>,
    extended_result_codes: Option<unsafe extern "C" fn(*mut sqlite3, i32)
        -> i32>,
    limit: Option<unsafe extern "C" fn(*mut sqlite3, i32, i32) -> i32>,
    next_stmt: Option<unsafe extern "C" fn(*mut sqlite3, *mut sqlite3_stmt)
        -> *mut sqlite3_stmt>,
    sql: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> *const i8>,
    status: Option<unsafe extern "C" fn(i32, *mut i32, *mut i32, i32) -> i32>,
    backup_finish: Option<unsafe extern "C" fn(*mut sqlite3_backup) -> i32>,
    backup_init: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        *mut sqlite3, *const i8) -> *mut sqlite3_backup>,
    backup_pagecount: Option<unsafe extern "C" fn(*mut sqlite3_backup)
        -> i32>,
    backup_remaining: Option<unsafe extern "C" fn(*mut sqlite3_backup)
        -> i32>,
    backup_step: Option<unsafe extern "C" fn(*mut sqlite3_backup, i32)
        -> i32>,
    compileoption_get: Option<unsafe extern "C" fn(i32) -> *const i8>,
    compileoption_used: Option<unsafe extern "C" fn(*const i8) -> i32>,
    create_function_v2: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        i32, i32, *mut (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context) -> (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    db_config: Option<unsafe extern "C" fn(*mut sqlite3, i32, ...) -> i32>,
    db_mutex: Option<unsafe extern "C" fn(*mut sqlite3)
        -> *mut sqlite3_mutex>,
    db_status: Option<unsafe extern "C" fn(*mut sqlite3, i32, *mut i32,
        *mut i32, i32) -> i32>,
    extended_errcode: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    log: Option<unsafe extern "C" fn(i32, *const i8, ...) -> ()>,
    soft_heap_limit64: Option<unsafe extern "C" fn(i64) -> i64>,
    sourceid: Option<unsafe extern "C" fn() -> *const i8>,
    stmt_status: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, i32)
        -> i32>,
    strnicmp: Option<unsafe extern "C" fn(*const i8, *const i8, i32) -> i32>,
    unlock_notify: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut *mut (), i32) -> (), *mut ()) -> i32>,
    wal_autocheckpoint: Option<unsafe extern "C" fn(*mut sqlite3, i32)
        -> i32>,
    wal_checkpoint: Option<unsafe extern "C" fn(*mut sqlite3, *const i8)
        -> i32>,
    wal_hook: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32,
        *mut ()) -> *mut ()>,
    blob_reopen: Option<unsafe extern "C" fn(*mut sqlite3_blob, i64) -> i32>,
    vtab_config: Option<unsafe extern "C" fn(*mut sqlite3, i32, ...) -> i32>,
    vtab_on_conflict: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    close_v2: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    db_filename: Option<unsafe extern "C" fn(*mut sqlite3, *const i8)
        -> *const i8>,
    db_readonly: Option<unsafe extern "C" fn(*mut sqlite3, *const i8) -> i32>,
    db_release_memory: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    errstr: Option<unsafe extern "C" fn(i32) -> *const i8>,
    stmt_busy: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    stmt_readonly: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    stricmp: Option<unsafe extern "C" fn(*const i8, *const i8) -> i32>,
    uri_boolean: Option<unsafe extern "C" fn(*const i8, *const i8, i32)
        -> i32>,
    uri_int64: Option<unsafe extern "C" fn(*const i8, *const i8, i64) -> i64>,
    uri_parameter: Option<unsafe extern "C" fn(*const i8, *const i8)
        -> *const i8>,
    xvsnprintf: Option<unsafe extern "C" fn(i32, *mut i8, *const i8, *mut i8)
        -> *mut i8>,
    wal_checkpoint_v2: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        i32, *mut i32, *mut i32) -> i32>,
    auto_extension: Option<unsafe extern "C" fn(unsafe extern "C" fn() -> ())
        -> i32>,
    bind_blob64: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32,
        *const (), u64, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    bind_text64: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32,
        *const i8, u64, unsafe extern "C" fn(*mut ()) -> (), u8) -> i32>,
    cancel_auto_extension: Option<unsafe extern "C" fn(unsafe extern "C" fn()
                -> ()) -> i32>,
    load_extension: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        *const i8, *mut *mut i8) -> i32>,
    malloc64: Option<unsafe extern "C" fn(u64) -> *mut ()>,
    msize: Option<unsafe extern "C" fn(*mut ()) -> u64>,
    realloc64: Option<unsafe extern "C" fn(*mut (), u64) -> *mut ()>,
    reset_auto_extension: Option<unsafe extern "C" fn() -> ()>,
    result_blob64: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *const (), u64, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    result_text64: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *const i8, u64, unsafe extern "C" fn(*mut ()) -> (), u8) -> ()>,
    strglob: Option<unsafe extern "C" fn(*const i8, *const i8) -> i32>,
    value_dup: Option<unsafe extern "C" fn(*const sqlite3_value)
        -> *mut sqlite3_value>,
    value_free: Option<unsafe extern "C" fn(*mut sqlite3_value) -> ()>,
    result_zeroblob64: Option<unsafe extern "C" fn(*mut sqlite3_context, u64)
        -> i32>,
    bind_zeroblob64: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, u64)
        -> i32>,
    value_subtype: Option<unsafe extern "C" fn(*mut sqlite3_value) -> u32>,
    result_subtype: Option<unsafe extern "C" fn(*mut sqlite3_context, u32)
        -> ()>,
    status64: Option<unsafe extern "C" fn(i32, *mut i64, *mut i64, i32)
        -> i32>,
    strlike: Option<unsafe extern "C" fn(*const i8, *const i8, u32) -> i32>,
    db_cacheflush: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    system_errno: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    trace_v2: Option<unsafe extern "C" fn(*mut sqlite3, u32,
        unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32, *mut ())
        -> i32>,
    expanded_sql: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> *mut i8>,
    set_last_insert_rowid: Option<unsafe extern "C" fn(*mut sqlite3, i64)
        -> ()>,
    prepare_v3: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, i32, u32,
        *mut *mut sqlite3_stmt, *mut *const i8) -> i32>,
    prepare16_v3: Option<unsafe extern "C" fn(*mut sqlite3, *const (), i32,
        u32, *mut *mut sqlite3_stmt, *mut *const ()) -> i32>,
    bind_pointer: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, *mut (),
        *const i8, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    result_pointer: Option<unsafe extern "C" fn(*mut sqlite3_context, *mut (),
        *const i8, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    value_pointer: Option<unsafe extern "C" fn(*mut sqlite3_value, *const i8)
        -> *mut ()>,
    vtab_nochange: Option<unsafe extern "C" fn(*mut sqlite3_context) -> i32>,
    value_nochange: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    vtab_collation: Option<unsafe extern "C" fn(*mut sqlite3_index_info, i32)
        -> *const i8>,
    keyword_count: Option<unsafe extern "C" fn() -> i32>,
    keyword_name: Option<unsafe extern "C" fn(i32, *mut *const i8, *mut i32)
        -> i32>,
    keyword_check: Option<unsafe extern "C" fn(*const i8, i32) -> i32>,
    str_new: Option<unsafe extern "C" fn(*mut sqlite3) -> *mut sqlite3_str>,
    str_finish: Option<unsafe extern "C" fn(*mut sqlite3_str) -> *mut i8>,
    str_appendf: Option<unsafe extern "C" fn(*mut sqlite3_str, *const i8, ...)
        -> ()>,
    str_vappendf: Option<unsafe extern "C" fn(*mut sqlite3_str, *const i8,
        *mut i8) -> ()>,
    str_append: Option<unsafe extern "C" fn(*mut sqlite3_str, *const i8, i32)
        -> ()>,
    str_appendall: Option<unsafe extern "C" fn(*mut sqlite3_str, *const i8)
        -> ()>,
    str_appendchar: Option<unsafe extern "C" fn(*mut sqlite3_str, i32, i8)
        -> ()>,
    str_reset: Option<unsafe extern "C" fn(*mut sqlite3_str) -> ()>,
    str_errcode: Option<unsafe extern "C" fn(*mut sqlite3_str) -> i32>,
    str_length: Option<unsafe extern "C" fn(*mut sqlite3_str) -> i32>,
    str_value: Option<unsafe extern "C" fn(*mut sqlite3_str) -> *mut i8>,
    create_window_function: Option<unsafe extern "C" fn(*mut sqlite3,
        *const i8, i32, i32, *mut (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut sqlite3_context) -> (),
        unsafe extern "C" fn(*mut sqlite3_context) -> (),
        unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    normalized_sql: Option<unsafe extern "C" fn(*mut sqlite3_stmt)
        -> *const i8>,
    stmt_isexplain: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> i32>,
    value_frombind: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    drop_modules: Option<unsafe extern "C" fn(*mut sqlite3, *mut *const i8)
        -> i32>,
    hard_heap_limit64: Option<unsafe extern "C" fn(i64) -> i64>,
    uri_key: Option<unsafe extern "C" fn(*const i8, i32) -> *const i8>,
    filename_database: Option<unsafe extern "C" fn(*const i8) -> *const i8>,
    filename_journal: Option<unsafe extern "C" fn(*const i8) -> *const i8>,
    filename_wal: Option<unsafe extern "C" fn(*const i8) -> *const i8>,
    create_filename: Option<unsafe extern "C" fn(*const i8, *const i8,
        *const i8, i32, *mut *const i8) -> *const i8>,
    free_filename: Option<unsafe extern "C" fn(*const i8) -> ()>,
    database_file_object: Option<unsafe extern "C" fn(*const i8)
        -> *mut sqlite3_file>,
    txn_state: Option<unsafe extern "C" fn(*mut sqlite3, *const i8) -> i32>,
    changes64: Option<unsafe extern "C" fn(*mut sqlite3) -> i64>,
    total_changes64: Option<unsafe extern "C" fn(*mut sqlite3) -> i64>,
    autovacuum_pages: Option<unsafe extern "C" fn(*mut sqlite3,
        unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32,
        *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    error_offset: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    vtab_rhs_value: Option<unsafe extern "C" fn(*mut sqlite3_index_info, i32,
        *mut *mut sqlite3_value) -> i32>,
    vtab_distinct: Option<unsafe extern "C" fn(*mut sqlite3_index_info)
        -> i32>,
    vtab_in: Option<unsafe extern "C" fn(*mut sqlite3_index_info, i32, i32)
        -> i32>,
    vtab_in_first: Option<unsafe extern "C" fn(*mut sqlite3_value,
        *mut *mut sqlite3_value) -> i32>,
    vtab_in_next: Option<unsafe extern "C" fn(*mut sqlite3_value,
        *mut *mut sqlite3_value) -> i32>,
    deserialize: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, *mut u8,
        i64, i64, u32) -> i32>,
    serialize: Option<unsafe extern "C" fn(*mut sqlite3, *const i8, *mut i64,
        u32) -> *mut u8>,
    db_name: Option<unsafe extern "C" fn(*mut sqlite3, i32) -> *const i8>,
    value_encoding: Option<unsafe extern "C" fn(*mut sqlite3_value) -> i32>,
    is_interrupted: Option<unsafe extern "C" fn(*mut sqlite3) -> i32>,
    stmt_explain: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32) -> i32>,
    get_clientdata: Option<unsafe extern "C" fn(*mut sqlite3, *const i8)
        -> *mut ()>,
    set_clientdata: Option<unsafe extern "C" fn(*mut sqlite3, *const i8,
        *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    setlk_timeout: Option<unsafe extern "C" fn(*mut sqlite3, i32, i32)
        -> i32>,
    set_errmsg: Option<unsafe extern "C" fn(*mut sqlite3, i32, *const i8)
        -> i32>,
    db_status64: Option<unsafe extern "C" fn(*mut sqlite3, i32, *mut i64,
        *mut i64, i32) -> i32>,
    str_truncate: Option<unsafe extern "C" fn(*mut sqlite3_str, i32) -> ()>,
    str_free: Option<unsafe extern "C" fn(*mut sqlite3_str) -> ()>,
    carray_bind: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32, *mut (),
        i32, i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    carray_bind_v2: Option<unsafe extern "C" fn(*mut sqlite3_stmt, i32,
        *mut (), i32, i32, unsafe extern "C" fn(*mut ()) -> (), *mut ())
        -> i32>,
    incomplete: Option<unsafe extern "C" fn(*const i8) -> i64>,
    result_str: Option<unsafe extern "C" fn(*mut sqlite3_context,
        *mut sqlite3_str, i32) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_context {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_stmt {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_value {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_module {
    i_version: i32,
    x_create: Option<unsafe extern "C" fn(*mut sqlite3, *mut (), i32,
        *const *const i8, *mut *mut sqlite3_vtab, *mut *mut i8) -> i32>,
    x_connect: Option<unsafe extern "C" fn(*mut sqlite3, *mut (), i32,
        *const *const i8, *mut *mut sqlite3_vtab, *mut *mut i8) -> i32>,
    x_best_index: Option<unsafe extern "C" fn(*mut sqlite3_vtab,
        *mut sqlite3_index_info) -> i32>,
    x_disconnect: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_destroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_open: Option<unsafe extern "C" fn(*mut sqlite3_vtab,
        *mut *mut sqlite3_vtab_cursor) -> i32>,
    x_close: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> i32>,
    x_filter: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor, i32,
        *const i8, i32, *mut *mut sqlite3_value) -> i32>,
    x_next: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> i32>,
    x_eof: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> i32>,
    x_column: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor,
        *mut sqlite3_context, i32) -> i32>,
    x_rowid: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor, *mut i64)
        -> i32>,
    x_update: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32,
        *mut *mut sqlite3_value, *mut i64) -> i32>,
    x_begin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_sync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_commit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_rollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_find_function: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32,
        *const i8,
        *mut unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (), *mut *mut ()) -> i32>,
    x_rename: Option<unsafe extern "C" fn(*mut sqlite3_vtab, *const i8)
        -> i32>,
    x_savepoint: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32) -> i32>,
    x_release: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32) -> i32>,
    x_rollback_to: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32)
        -> i32>,
    x_shadow_name: Option<unsafe extern "C" fn(*const i8) -> i32>,
    x_integrity: Option<unsafe extern "C" fn(*mut sqlite3_vtab, *const i8,
        *const i8, i32, *mut *mut i8) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vtab {
    p_module: *const sqlite3_module,
    n_ref: i32,
    z_err_msg: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_info {
    n_constraint: i32,
    a_constraint: *mut sqlite3_index_constraint,
    n_order_by: i32,
    a_order_by: *mut sqlite3_index_orderby,
    a_constraint_usage: *mut sqlite3_index_constraint_usage,
    idx_num: i32,
    idx_str: *mut i8,
    need_to_free_idx_str: i32,
    order_by_consumed: i32,
    estimated_cost: f64,
    estimated_rows: sqlite3_int64,
    idx_flags: i32,
    col_used: sqlite3_uint64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_constraint {
    i_column: i32,
    op: u8,
    usable: u8,
    i_term_offset: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_orderby {
    i_column: i32,
    desc: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_constraint_usage {
    argv_index: i32,
    omit: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vtab_cursor {
    p_vtab: *mut sqlite3_vtab,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_blob {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vfs {
    i_version: i32,
    sz_os_file: i32,
    mx_pathname: i32,
    p_next: *mut sqlite3_vfs,
    z_name: *const i8,
    p_app_data: *mut (),
    x_open: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8,
        *mut sqlite3_file, i32, *mut i32) -> i32>,
    x_delete: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8, i32)
        -> i32>,
    x_access: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8, i32,
        *mut i32) -> i32>,
    x_full_pathname: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8,
        i32, *mut i8) -> i32>,
    x_dl_open: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8)
        -> *mut ()>,
    x_dl_error: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32, *mut i8)
        -> ()>,
    x_dl_sym: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut (),
        *const i8) -> unsafe extern "C" fn() -> ()>,
    x_dl_close: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut ()) -> ()>,
    x_randomness: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32, *mut i8)
        -> i32>,
    x_sleep: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32) -> i32>,
    x_current_time: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut f64)
        -> i32>,
    x_get_last_error: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32,
        *mut i8) -> i32>,
    x_current_time_int64: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *mut i64) -> i32>,
    x_set_system_call: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *const i8, unsafe extern "C" fn() -> ()) -> i32>,
    x_get_system_call: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *const i8) -> unsafe extern "C" fn() -> ()>,
    x_next_system_call: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *const i8) -> *const i8>,
}
type sqlite3_filename = *const i8;
type sqlite3_syscall_ptr = unsafe extern "C" fn() -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_backup {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_str {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_mem_methods {
    x_malloc: Option<unsafe extern "C" fn(i32) -> *mut ()>,
    x_free: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_realloc: Option<unsafe extern "C" fn(*mut (), i32) -> *mut ()>,
    x_size: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_roundup: Option<unsafe extern "C" fn(i32) -> i32>,
    x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_app_data: *mut (),
}
type sqlite3_destructor_type = unsafe extern "C" fn(*mut ()) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_mutex_methods {
    x_mutex_init: Option<unsafe extern "C" fn() -> i32>,
    x_mutex_end: Option<unsafe extern "C" fn() -> i32>,
    x_mutex_alloc: Option<unsafe extern "C" fn(i32) -> *mut sqlite3_mutex>,
    x_mutex_free: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    x_mutex_enter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    x_mutex_try: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
    x_mutex_leave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    x_mutex_held: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
    x_mutex_notheld: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_page {
    p_buf: *mut (),
    p_extra: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_methods2 {
    i_version: i32,
    p_arg: *mut (),
    x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_create: Option<unsafe extern "C" fn(i32, i32, i32)
        -> *mut sqlite3_pcache>,
    x_cachesize: Option<unsafe extern "C" fn(*mut sqlite3_pcache, i32) -> ()>,
    x_pagecount: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> i32>,
    x_fetch: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32, i32)
        -> *mut sqlite3_pcache_page>,
    x_unpin: Option<unsafe extern "C" fn(*mut sqlite3_pcache,
        *mut sqlite3_pcache_page, i32) -> ()>,
    x_rekey: Option<unsafe extern "C" fn(*mut sqlite3_pcache,
        *mut sqlite3_pcache_page, u32, u32) -> ()>,
    x_truncate: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32) -> ()>,
    x_destroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
    x_shrink: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_methods {
    p_arg: *mut (),
    x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_create: Option<unsafe extern "C" fn(i32, i32) -> *mut sqlite3_pcache>,
    x_cachesize: Option<unsafe extern "C" fn(*mut sqlite3_pcache, i32) -> ()>,
    x_pagecount: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> i32>,
    x_fetch: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32, i32)
        -> *mut ()>,
    x_unpin: Option<unsafe extern "C" fn(*mut sqlite3_pcache, *mut (), i32)
        -> ()>,
    x_rekey: Option<unsafe extern "C" fn(*mut sqlite3_pcache, *mut (), u32,
        u32) -> ()>,
    x_truncate: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32) -> ()>,
    x_destroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_snapshot {
    hidden: [u8; 48],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_rtree_geometry {
    p_context: *mut (),
    n_param: i32,
    a_param: *mut sqlite3_rtree_dbl,
    p_user: *mut (),
    x_del_user: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}
type sqlite3_rtree_dbl = f64;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_rtree_query_info {
    p_context: *mut (),
    n_param: i32,
    a_param: *mut sqlite3_rtree_dbl,
    p_user: *mut (),
    x_del_user: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    a_coord: *mut sqlite3_rtree_dbl,
    an_queue: *mut u32,
    n_coord: i32,
    i_level: i32,
    mx_level: i32,
    i_rowid: sqlite3_int64,
    r_parent_score: sqlite3_rtree_dbl,
    e_parent_within: i32,
    e_within: i32,
    r_score: sqlite3_rtree_dbl,
    ap_sql_param: *mut *mut sqlite3_value,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExtensionApi {
    i_version: i32,
    x_user_data: Option<unsafe extern "C" fn(*mut Fts5Context) -> *mut ()>,
    x_column_count: Option<unsafe extern "C" fn(*mut Fts5Context) -> i32>,
    x_row_count: Option<unsafe extern "C" fn(*mut Fts5Context, *mut i64)
        -> i32>,
    x_column_total_size: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut i64) -> i32>,
    x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Context, *const i8, i32,
        *mut (),
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
    x_phrase_count: Option<unsafe extern "C" fn(*mut Fts5Context) -> i32>,
    x_phrase_size: Option<unsafe extern "C" fn(*mut Fts5Context, i32) -> i32>,
    x_inst_count: Option<unsafe extern "C" fn(*mut Fts5Context, *mut i32)
        -> i32>,
    x_inst: Option<unsafe extern "C" fn(*mut Fts5Context, i32, *mut i32,
        *mut i32, *mut i32) -> i32>,
    x_rowid: Option<unsafe extern "C" fn(*mut Fts5Context) -> i64>,
    x_column_text: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_column_size: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut i32) -> i32>,
    x_query_phrase: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut (),
        unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
                *mut ()) -> i32) -> i32>,
    x_set_auxdata: Option<unsafe extern "C" fn(*mut Fts5Context, *mut (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    x_get_auxdata: Option<unsafe extern "C" fn(*mut Fts5Context, i32)
        -> *mut ()>,
    x_phrase_first: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut Fts5PhraseIter, *mut i32, *mut i32) -> i32>,
    x_phrase_next: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut Fts5PhraseIter, *mut i32, *mut i32) -> ()>,
    x_phrase_first_column: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut Fts5PhraseIter, *mut i32) -> i32>,
    x_phrase_next_column: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut Fts5PhraseIter, *mut i32) -> ()>,
    x_query_token: Option<unsafe extern "C" fn(*mut Fts5Context, i32, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_inst_token: Option<unsafe extern "C" fn(*mut Fts5Context, i32, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_column_locale: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_tokenize_v2: Option<unsafe extern "C" fn(*mut Fts5Context, *const i8,
        i32, *const i8, i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Context {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5PhraseIter {
    a: *const u8,
    b: *const u8,
}
type fts5_extension_function =
    unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
        *mut sqlite3_context, i32, *mut *mut sqlite3_value) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Tokenizer {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fts5_tokenizer_v2 {
    i_version: i32,
    x_create: Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
        *mut *mut Fts5Tokenizer) -> i32>,
    x_delete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Tokenizer, *mut (), i32,
        *const i8, i32, *const i8, i32,
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fts5_tokenizer {
    x_create: Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
        *mut *mut Fts5Tokenizer) -> i32>,
    x_delete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Tokenizer, *mut (), i32,
        *const i8, i32,
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fts5_api {
    i_version: i32,
    x_create_tokenizer: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut (), *mut fts5_tokenizer, unsafe extern "C" fn(*mut ()) -> ())
        -> i32>,
    x_find_tokenizer: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut *mut (), *mut fts5_tokenizer) -> i32>,
    x_create_function: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut (),
        unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
                *mut sqlite3_context, i32, *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    x_create_tokenizer_v2: Option<unsafe extern "C" fn(*mut fts5_api,
        *const i8, *mut (), *mut fts5_tokenizer_v2,
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    x_find_tokenizer_v2: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut *mut (), *mut *mut fts5_tokenizer_v2) -> i32>,
}
type sqlite3_loadext_entry =
    unsafe extern "C" fn(*mut sqlite3, *mut *mut i8,
        *const sqlite3_api_routines) -> i32;
extern "C" fn read_file_contents(ctx: *mut sqlite3_context,
    z_name_1: *const i8) -> () {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut n_in: sqlite3_int64 = 0 as sqlite3_int64;
    let mut p_buf: *mut () = core::ptr::null_mut();
    let mut db: *mut sqlite3 = core::ptr::null_mut();
    let mut mx_blob: i32 = 0;
    in_ = unsafe { fopen(z_name_1, c"rb".as_ptr() as *mut i8 as *const i8) };
    if in_ == core::ptr::null_mut() { return; }
    unsafe { fseek(in_, 0 as i64, 2) };
    n_in = unsafe { ftell(in_) } as sqlite3_int64;
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
            sqlite3_malloc64(if n_in != 0 { n_in } else { 1 as sqlite3_int64 }
                    as sqlite3_uint64)
        };
    if p_buf == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(ctx) };
        unsafe { fclose(in_) };
        return;
    }
    if n_in ==
            unsafe { fread(p_buf, 1 as u64, n_in as u64, in_) } as
                sqlite3_int64 {
        unsafe {
            sqlite3_result_blob64(ctx, p_buf as *const (),
                n_in as sqlite3_uint64, Some(sqlite3_free))
        };
    } else {
        unsafe { sqlite3_result_error_code(ctx, 10) };
        unsafe { sqlite3_free(p_buf) };
    }
    unsafe { fclose(in_) };
}
extern "C" fn readfile_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut z_name: *const i8 = core::ptr::null();
    { let _ = argc; };
    z_name =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_name == core::ptr::null() { return; }
    read_file_contents(context, z_name);
}
unsafe extern "C" fn ctx_error_msg(ctx: *mut sqlite3_context,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    unsafe { sqlite3_result_error(ctx, z_msg as *const i8, -1) };
    unsafe { sqlite3_free(z_msg as *mut ()) };
    ();
}
extern "C" fn file_stat(z_path_1: *const i8, p_stat_buf_1: *mut stat) -> i32 {
    return unsafe { stat(z_path_1, p_stat_buf_1) };
}
extern "C" fn file_link_stat(z_path_1: *const i8, p_stat_buf_1: *mut stat)
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
            let mut s_stat: stat = unsafe { core::mem::zeroed() };
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
                if unsafe { mkdir(z_copy as *const i8, 511 as mode_t) } != 0 {
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
extern "C" fn write_file(p_ctx_1: *mut sqlite3_context, z_file_1: *const i8,
    p_data_1: *mut sqlite3_value, mode: mode_t, mtime: sqlite3_int64) -> i32 {
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
                let mut s_stat: stat = unsafe { core::mem::zeroed() };
                if unsafe { *unsafe { __error() } } != 17 ||
                                0 != file_stat(z_file_1, &mut s_stat) ||
                            !(s_stat.st_mode as i32 & 61440 == 16384) as i32 != 0 ||
                        s_stat.st_mode as i32 & 511 != mode as i32 & 511 &&
                            0 !=
                                unsafe { chmod(z_file_1, (mode as i32 & 511) as mode_t) } {
                    return 1;
                }
            }
        } else {
            let mut n_write: sqlite3_int64 = 0 as sqlite3_int64;
            let mut z: *const i8 = core::ptr::null();
            let mut rc: i32 = 0;
            let out: *mut FILE =
                unsafe {
                    fopen(z_file_1, c"wb".as_ptr() as *mut i8 as *const i8)
                };
            if out == core::ptr::null_mut() { return 1; }
            z = unsafe { sqlite3_value_blob(p_data_1) } as *const i8;
            if !(z).is_null() {
                let n: sqlite3_int64 =
                    unsafe {
                            fwrite(z as *const (), 1 as u64,
                                unsafe { sqlite3_value_bytes(p_data_1) } as u64, out)
                        } as sqlite3_int64;
                n_write =
                    unsafe { sqlite3_value_bytes(p_data_1) } as sqlite3_int64;
                if n_write != n { rc = 1; }
            }
            unsafe { fclose(out) };
            if rc == 0 && mode != 0 &&
                    unsafe { chmod(z_file_1, (mode as i32 & 511) as mode_t) } !=
                        0 {
                rc = 1;
            }
            if rc != 0 { return 2; }
            unsafe { sqlite3_result_int64(p_ctx_1, n_write) };
        }
    }
    if mtime >= 0 as i64 {
        if 0 == (mode as i32 & 61440 == 40960) as i32 {
            let mut times: [timeval; 2] = unsafe { core::mem::zeroed() };
            times[0 as usize].tv_usec =
                { times[1 as usize].tv_usec = 0; times[1 as usize].tv_usec };
            times[0 as usize].tv_sec = unsafe { time(core::ptr::null_mut()) };
            times[1 as usize].tv_sec = mtime as __darwin_time_t;
            if unsafe {
                        utimes(z_file_1,
                            &raw mut times[0 as usize] as *mut timeval as
                                *const timeval)
                    } != 0 {
                return 1;
            }
        }
    }
    return 0;
}
extern "C" fn writefile_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut z_file: *const i8 = core::ptr::null();
    let mut mode: mode_t = 0 as mode_t;
    let mut res: i32 = 0;
    let mut mtime: sqlite3_int64 = -1 as sqlite3_int64;
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
                as mode_t;
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
extern "C" fn ls_mode_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
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
struct fsdir_cursor {
    base: sqlite3_vtab_cursor,
    n_lvl: i32,
    mx_lvl: i32,
    i_lvl: i32,
    a_lvl: *mut FsdirLevel,
    z_base: *const i8,
    n_base: i32,
    s_stat: stat,
    z_path: *mut i8,
    i_rowid: sqlite3_int64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FsdirLevel {
    p_dir: *mut DIR,
    z_dir: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fsdir_tab {
    base: sqlite3_vtab,
}
extern "C" fn fsdir_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut fsdir_tab = core::ptr::null_mut();
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
                    sqlite3_malloc64(core::mem::size_of::<fsdir_tab>() as
                            sqlite3_uint64)
                } as *mut fsdir_tab;
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<fsdir_tab>() as u64)
        };
        unsafe { sqlite3_vtab_config(db, 3) };
    }
    unsafe { *pp_vtab_1 = p_new as *mut sqlite3_vtab };
    return rc;
}
extern "C" fn fsdir_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}
extern "C" fn fsdir_open(p: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let mut p_cur: *mut fsdir_cursor = core::ptr::null_mut();
    { let _ = p; };
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<fsdir_cursor>() as
                        sqlite3_uint64)
            } as *mut fsdir_cursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<fsdir_cursor>() as u64)
    };
    unsafe { (*p_cur).i_lvl = -1 };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}
extern "C" fn fsdir_reset_cursor(p_cur_1: &mut fsdir_cursor) -> () {
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
    (*p_cur_1).i_rowid = 1 as sqlite3_int64;
}
extern "C" fn fsdir_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut fsdir_cursor = cur as *mut fsdir_cursor;
    fsdir_reset_cursor(unsafe { &mut *p_cur });
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}
unsafe extern "C" fn fsdir_set_errmsg(p_cur_1: &fsdir_cursor,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe {
        (*(*p_cur_1).base.p_vtab).z_err_msg =
            unsafe { sqlite3_vmprintf(z_fmt_1, ap) }
    };
    ();
}
extern "C" fn fsdir_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut fsdir_cursor = cur as *mut fsdir_cursor;
    let m: mode_t = unsafe { (*p_cur).s_stat.st_mode };
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
            let n_byte: sqlite3_int64 =
                (n_new as u64 * core::mem::size_of::<FsdirLevel>() as u64) as
                    sqlite3_int64;
            let a_new: *mut FsdirLevel =
                unsafe {
                        sqlite3_realloc64(unsafe { (*p_cur).a_lvl } as *mut (),
                            n_byte as sqlite3_uint64)
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
        let p_entry: *mut dirent =
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
extern "C" fn fsdir_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_cur: *const fsdir_cursor =
        cur as *mut fsdir_cursor as *const fsdir_cursor;
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
                        unsafe { (*p_cur).s_stat.st_mode } as sqlite3_int64)
                };
            }
            1 => {
                unsafe {
                    sqlite3_result_int64(ctx,
                        unsafe { (*p_cur).s_stat.st_mode } as sqlite3_int64)
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
                    let m: mode_t = unsafe { (*p_cur).s_stat.st_mode };
                    if m as i32 & 61440 == 16384 {
                        unsafe { sqlite3_result_null(ctx) };
                    } else if m as i32 & 61440 == 40960 {
                        let mut a_static: [i8; 64] = [0; 64];
                        let mut a_buf: *mut i8 =
                            &raw mut a_static[0 as usize] as *mut i8;
                        let mut n_buf: sqlite3_int64 = 64 as sqlite3_int64;
                        let mut n: i32 = 0;
                        loop {
                            n =
                                unsafe {
                                        readlink(unsafe { (*p_cur).z_path } as *const i8, a_buf,
                                            n_buf as u64)
                                    } as i32;
                            if (n as sqlite3_int64) < n_buf { break; }
                            if a_buf != &raw mut a_static[0 as usize] as *mut i8 {
                                unsafe { sqlite3_free(a_buf as *mut ()) };
                            }
                            n_buf = n_buf * 2 as sqlite3_int64;
                            a_buf =
                                unsafe { sqlite3_malloc64(n_buf as sqlite3_uint64) } as
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
extern "C" fn fsdir_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p_cur: *const fsdir_cursor =
        cur as *mut fsdir_cursor as *const fsdir_cursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}
extern "C" fn fsdir_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const fsdir_cursor =
        cur as *mut fsdir_cursor as *const fsdir_cursor;
    return (unsafe { (*p_cur).z_path } == core::ptr::null_mut()) as i32;
}
extern "C" fn fsdir_filter(cur: *mut sqlite3_vtab_cursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut sqlite3_value) -> i32 {
    let mut z_dir: *const i8 = core::ptr::null();
    let p_cur: *mut fsdir_cursor = cur as *mut fsdir_cursor;
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
extern "C" fn fsdir_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i: i32 = 0;
    let mut idx_path: i32 = -1;
    let mut idx_dir: i32 = -1;
    let mut idx_level: i32 = -1;
    let mut idx_level_eq: i32 = 0;
    let mut omit_level: i32 = 0;
    let mut seen_path: i32 = 0;
    let mut seen_dir: i32 = 0;
    let mut p_constraint: *const sqlite3_index_constraint = core::ptr::null();
    { let _ = tab; };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const sqlite3_index_constraint;
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
            (*p_idx_info_1).estimated_rows = 2147483647 as sqlite3_int64
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
extern "C" fn fsdir_register(db: *mut sqlite3) -> i32 {
    unsafe {
        let rc: i32 =
            unsafe {
                sqlite3_create_module(db,
                    c"fsdir".as_ptr() as *mut i8 as *const i8,
                    &raw mut fsdir_module as *const sqlite3_module,
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
extern "C" fn realpath_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
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
pub extern "C" fn sqlite3_fileio_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
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
static mut fsdir_module: sqlite3_module =
    sqlite3_module {
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
    fn sqlite3_close(_: *mut sqlite3)
    -> i32;
    fn sqlite3_close_v2(_: *mut sqlite3)
    -> i32;
    fn sqlite3_exec(_: *mut sqlite3, sql: *const i8,
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
    fn sqlite3_db_config(_: *mut sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_extended_result_codes(_: *mut sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3)
    -> sqlite3_int64;
    fn sqlite3_set_last_insert_rowid(_: *mut sqlite3, _: sqlite3_int64)
    -> ();
    fn sqlite3_changes(_: *mut sqlite3)
    -> i32;
    fn sqlite3_changes64(_: *mut sqlite3)
    -> sqlite3_int64;
    fn sqlite3_total_changes(_: *mut sqlite3)
    -> i32;
    fn sqlite3_total_changes64(_: *mut sqlite3)
    -> sqlite3_int64;
    fn sqlite3_interrupt(_: *mut sqlite3)
    -> ();
    fn sqlite3_is_interrupted(_: *mut sqlite3)
    -> i32;
    fn sqlite3_complete(sql: *const i8)
    -> i32;
    fn sqlite3_complete16(sql: *const ())
    -> i32;
    fn sqlite3_incomplete(sql: *const i8)
    -> sqlite3_int64;
    fn sqlite3_busy_handler(_: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), i32) -> i32>, _: *mut ())
    -> i32;
    fn sqlite3_busy_timeout(_: *mut sqlite3, ms: i32)
    -> i32;
    fn sqlite3_setlk_timeout(_: *mut sqlite3, ms: i32, flags: i32)
    -> i32;
    fn sqlite3_get_table(db: *mut sqlite3, z_sql_1: *const i8,
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
    fn sqlite3_malloc64(_: sqlite3_uint64)
    -> *mut ();
    fn sqlite3_realloc(_: *mut (), _: i32)
    -> *mut ();
    fn sqlite3_realloc64(_: *mut (), _: sqlite3_uint64)
    -> *mut ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_msize(_: *mut ())
    -> sqlite3_uint64;
    fn sqlite3_memory_used()
    -> sqlite3_int64;
    fn sqlite3_memory_highwater(reset_flag_1: i32)
    -> sqlite3_int64;
    fn sqlite3_randomness(n_1: i32, p_1: *mut ())
    -> ();
    fn sqlite3_set_authorizer(_: *mut sqlite3,
    x_auth_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
            *const i8, *const i8) -> i32>, p_user_data_1: *mut ())
    -> i32;
    fn sqlite3_trace(_: *mut sqlite3,
    x_trace_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_profile(_: *mut sqlite3,
    x_profile_1: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_trace_v2(_: *mut sqlite3, u_mask_1: u32,
    x_callback_1:
        Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    p_ctx_1: *mut ())
    -> i32;
    fn sqlite3_progress_handler(_: *mut sqlite3, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_open(filename: *const i8, pp_db_1: *mut *mut sqlite3)
    -> i32;
    fn sqlite3_open16(filename: *const (), pp_db_1: *mut *mut sqlite3)
    -> i32;
    fn sqlite3_open_v2(filename: *const i8, pp_db_1: *mut *mut sqlite3,
    flags: i32, z_vfs_1: *const i8)
    -> i32;
    fn sqlite3_uri_parameter(z: sqlite3_filename, z_param_1: *const i8)
    -> *const i8;
    fn sqlite3_uri_boolean(z: sqlite3_filename, z_param_1: *const i8,
    b_default_1: i32)
    -> i32;
    fn sqlite3_uri_int64(_: sqlite3_filename, _: *const i8, _: sqlite3_int64)
    -> sqlite3_int64;
    fn sqlite3_uri_key(z: sqlite3_filename, n_1: i32)
    -> *const i8;
    fn sqlite3_filename_database(_: sqlite3_filename)
    -> *const i8;
    fn sqlite3_filename_journal(_: sqlite3_filename)
    -> *const i8;
    fn sqlite3_filename_wal(_: sqlite3_filename)
    -> *const i8;
    fn sqlite3_database_file_object(_: *const i8)
    -> *mut sqlite3_file;
    fn sqlite3_create_filename(z_database_1: *const i8,
    z_journal_1: *const i8, z_wal_1: *const i8, n_param_1: i32,
    az_param_1: *mut *const i8)
    -> sqlite3_filename;
    fn sqlite3_free_filename(_: sqlite3_filename)
    -> ();
    fn sqlite3_errcode(db: *mut sqlite3)
    -> i32;
    fn sqlite3_extended_errcode(db: *mut sqlite3)
    -> i32;
    fn sqlite3_errmsg(_: *mut sqlite3)
    -> *const i8;
    fn sqlite3_errmsg16(_: *mut sqlite3)
    -> *const ();
    fn sqlite3_errstr(_: i32)
    -> *const i8;
    fn sqlite3_error_offset(db: *mut sqlite3)
    -> i32;
    fn sqlite3_set_errmsg(db: *mut sqlite3, errcode: i32,
    z_err_msg_1: *const i8)
    -> i32;
    fn sqlite3_limit(_: *mut sqlite3, id: i32, new_val_1: i32)
    -> i32;
    fn sqlite3_prepare(db: *mut sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut sqlite3_stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v2(db: *mut sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut sqlite3_stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v3(db: *mut sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    prep_flags_1: u32, pp_stmt_1: *mut *mut sqlite3_stmt,
    pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare16(db: *mut sqlite3, z_sql_1: *const (), n_byte_1: i32,
    pp_stmt_1: *mut *mut sqlite3_stmt, pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v2(db: *mut sqlite3, z_sql_1: *const (),
    n_byte_1: i32, pp_stmt_1: *mut *mut sqlite3_stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v3(db: *mut sqlite3, z_sql_1: *const (),
    n_byte_1: i32, prep_flags_1: u32, pp_stmt_1: *mut *mut sqlite3_stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_sql(p_stmt_1: *mut sqlite3_stmt)
    -> *const i8;
    fn sqlite3_expanded_sql(p_stmt_1: *mut sqlite3_stmt)
    -> *mut i8;
    fn sqlite3_stmt_readonly(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_stmt_isexplain(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_stmt_explain(p_stmt_1: *mut sqlite3_stmt, e_mode_1: i32)
    -> i32;
    fn sqlite3_stmt_busy(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_bind_blob(_: *mut sqlite3_stmt, _: i32, _: *const (), n: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_blob64(_: *mut sqlite3_stmt, _: i32, _: *const (),
    _: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_double(_: *mut sqlite3_stmt, _: i32, _: f64)
    -> i32;
    fn sqlite3_bind_int(_: *mut sqlite3_stmt, _: i32, _: i32)
    -> i32;
    fn sqlite3_bind_int64(_: *mut sqlite3_stmt, _: i32, _: sqlite3_int64)
    -> i32;
    fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: i32)
    -> i32;
    fn sqlite3_bind_text(_: *mut sqlite3_stmt, _: i32, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text16(_: *mut sqlite3_stmt, _: i32, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text64(_: *mut sqlite3_stmt, _: i32, _: *const i8,
    _: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> i32;
    fn sqlite3_bind_value(_: *mut sqlite3_stmt, _: i32,
    _: *const sqlite3_value)
    -> i32;
    fn sqlite3_bind_pointer(_: *mut sqlite3_stmt, _: i32, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_zeroblob(_: *mut sqlite3_stmt, _: i32, n: i32)
    -> i32;
    fn sqlite3_bind_zeroblob64(_: *mut sqlite3_stmt, _: i32,
    _: sqlite3_uint64)
    -> i32;
    fn sqlite3_bind_parameter_count(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_bind_parameter_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_bind_parameter_index(_: *mut sqlite3_stmt, z_name_1: *const i8)
    -> i32;
    fn sqlite3_clear_bindings(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_column_count(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_column_name(_: *mut sqlite3_stmt, n_1: i32)
    -> *const i8;
    fn sqlite3_column_name16(_: *mut sqlite3_stmt, n_1: i32)
    -> *const ();
    fn sqlite3_column_database_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_database_name16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_column_table_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_table_name16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_column_origin_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_origin_name16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_column_decltype(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_decltype16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_step(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_data_count(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_column_blob(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_double(_: *mut sqlite3_stmt, i_col_1: i32)
    -> f64;
    fn sqlite3_column_int(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_int64(_: *mut sqlite3_stmt, i_col_1: i32)
    -> sqlite3_int64;
    fn sqlite3_column_text(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *const u8;
    fn sqlite3_column_text16(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_value(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *mut sqlite3_value;
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_bytes16(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_type(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_finalize(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_reset(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_create_function(db: *mut sqlite3, z_function_name_1: *const i8,
    n_arg_1: i32, e_text_rep_1: i32, p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>)
    -> i32;
    fn sqlite3_create_function16(db: *mut sqlite3,
    z_function_name_1: *const (), n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>)
    -> i32;
    fn sqlite3_create_function_v2(db: *mut sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_window_function(db: *mut sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_aggregate_count(_: *mut sqlite3_context)
    -> i32;
    fn sqlite3_expired(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_transfer_bindings(_: *mut sqlite3_stmt, _: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_global_recover()
    -> i32;
    fn sqlite3_thread_cleanup()
    -> ();
    fn sqlite3_memory_alarm(_:
        Option<unsafe extern "C" fn(*mut (), i64, i32) -> ()>, _: *mut (),
    _: sqlite3_int64)
    -> i32;
    fn sqlite3_value_blob(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_double(_: *mut sqlite3_value)
    -> f64;
    fn sqlite3_value_int(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_int64(_: *mut sqlite3_value)
    -> sqlite3_int64;
    fn sqlite3_value_pointer(_: *mut sqlite3_value, _: *const i8)
    -> *mut ();
    fn sqlite3_value_text(_: *mut sqlite3_value)
    -> *const u8;
    fn sqlite3_value_text16(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_text16le(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_text16be(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_bytes(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_bytes16(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_type(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_numeric_type(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_nochange(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_frombind(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_encoding(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_subtype(_: *mut sqlite3_value)
    -> u32;
    fn sqlite3_value_dup(_: *const sqlite3_value)
    -> *mut sqlite3_value;
    fn sqlite3_value_free(_: *mut sqlite3_value)
    -> ();
    fn sqlite3_aggregate_context(_: *mut sqlite3_context, n_bytes_1: i32)
    -> *mut ();
    fn sqlite3_user_data(_: *mut sqlite3_context)
    -> *mut ();
    fn sqlite3_context_db_handle(_: *mut sqlite3_context)
    -> *mut sqlite3;
    fn sqlite3_get_auxdata(_: *mut sqlite3_context, n_1: i32)
    -> *mut ();
    fn sqlite3_set_auxdata(_: *mut sqlite3_context, n_1: i32, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_get_clientdata(_: *mut sqlite3, _: *const i8)
    -> *mut ();
    fn sqlite3_set_clientdata(_: *mut sqlite3, _: *const i8, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_result_blob(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_blob64(_: *mut sqlite3_context, _: *const (),
    _: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_double(_: *mut sqlite3_context, _: f64)
    -> ();
    fn sqlite3_result_error(_: *mut sqlite3_context, _: *const i8, _: i32)
    -> ();
    fn sqlite3_result_error16(_: *mut sqlite3_context, _: *const (), _: i32)
    -> ();
    fn sqlite3_result_error_toobig(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: i32)
    -> ();
    fn sqlite3_result_int(_: *mut sqlite3_context, _: i32)
    -> ();
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64)
    -> ();
    fn sqlite3_result_null(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_result_text(_: *mut sqlite3_context, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text64(_: *mut sqlite3_context, z: *const i8,
    n: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> ();
    fn sqlite3_result_text16(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16le(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16be(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value)
    -> ();
    fn sqlite3_result_pointer(_: *mut sqlite3_context, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_zeroblob(_: *mut sqlite3_context, n: i32)
    -> ();
    fn sqlite3_result_zeroblob64(_: *mut sqlite3_context, n: sqlite3_uint64)
    -> i32;
    fn sqlite3_result_subtype(_: *mut sqlite3_context, _: u32)
    -> ();
    fn sqlite3_create_collation(_: *mut sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_create_collation_v2(_: *mut sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_collation16(_: *mut sqlite3, z_name_1: *const (),
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_collation_needed(_: *mut sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut sqlite3, i32, *const i8)
            -> ()>)
    -> i32;
    fn sqlite3_collation_needed16(_: *mut sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut sqlite3, i32, *const ())
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
    fn sqlite3_get_autocommit(_: *mut sqlite3)
    -> i32;
    fn sqlite3_db_handle(_: *mut sqlite3_stmt)
    -> *mut sqlite3;
    fn sqlite3_db_name(db: *mut sqlite3, n_1: i32)
    -> *const i8;
    fn sqlite3_db_filename(db: *mut sqlite3, z_db_name_1: *const i8)
    -> sqlite3_filename;
    fn sqlite3_db_readonly(db: *mut sqlite3, z_db_name_1: *const i8)
    -> i32;
    fn sqlite3_txn_state(_: *mut sqlite3, z_schema_1: *const i8)
    -> i32;
    fn sqlite3_next_stmt(p_db_1: *mut sqlite3, p_stmt_1: *mut sqlite3_stmt)
    -> *mut sqlite3_stmt;
    fn sqlite3_commit_hook(_: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_rollback_hook(_: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_autovacuum_pages(db: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    _: *mut (), _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_update_hook(_: *mut sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64)
            -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_enable_shared_cache(_: i32)
    -> i32;
    fn sqlite3_release_memory(_: i32)
    -> i32;
    fn sqlite3_db_release_memory(_: *mut sqlite3)
    -> i32;
    fn sqlite3_soft_heap_limit64(n_1: sqlite3_int64)
    -> sqlite3_int64;
    fn sqlite3_hard_heap_limit64(n_1: sqlite3_int64)
    -> sqlite3_int64;
    fn sqlite3_soft_heap_limit(n_1: i32)
    -> ();
    fn sqlite3_table_column_metadata(db: *mut sqlite3, z_db_name_1: *const i8,
    z_table_name_1: *const i8, z_column_name_1: *const i8,
    pz_data_type_1: *mut *const i8, pz_coll_seq_1: *mut *const i8,
    p_not_null_1: *mut i32, p_primary_key_1: *mut i32, p_autoinc_1: *mut i32)
    -> i32;
    fn sqlite3_load_extension(db: *mut sqlite3, z_file_1: *const i8,
    z_proc_1: *const i8, pz_err_msg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_enable_load_extension(db: *mut sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_cancel_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_reset_auto_extension()
    -> ();
    fn sqlite3_create_module(db: *mut sqlite3, z_name_1: *const i8,
    p: *const sqlite3_module, p_client_data_1: *mut ())
    -> i32;
    fn sqlite3_create_module_v2(db: *mut sqlite3, z_name_1: *const i8,
    p: *const sqlite3_module, p_client_data_1: *mut (),
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_drop_modules(db: *mut sqlite3, az_keep_1: *mut *const i8)
    -> i32;
    fn sqlite3_declare_vtab(_: *mut sqlite3, z_sql_1: *const i8)
    -> i32;
    fn sqlite3_overload_function(_: *mut sqlite3, z_func_name_1: *const i8,
    n_arg_1: i32)
    -> i32;
    fn sqlite3_blob_open(_: *mut sqlite3, z_db_1: *const i8,
    z_table_1: *const i8, z_column_1: *const i8, i_row_1: sqlite3_int64,
    flags: i32, pp_blob_1: *mut *mut sqlite3_blob)
    -> i32;
    fn sqlite3_blob_reopen(_: *mut sqlite3_blob, _: sqlite3_int64)
    -> i32;
    fn sqlite3_blob_close(_: *mut sqlite3_blob)
    -> i32;
    fn sqlite3_blob_bytes(_: *mut sqlite3_blob)
    -> i32;
    fn sqlite3_blob_read(_: *mut sqlite3_blob, z_1: *mut (), n_1: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_blob_write(_: *mut sqlite3_blob, z: *const (), n: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_vfs_find(z_vfs_name_1: *const i8)
    -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(_: *mut sqlite3_vfs, make_dflt_1: i32)
    -> i32;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs)
    -> i32;
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex)
    -> ();
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex)
    -> ();
    fn sqlite3_mutex_try(_: *mut sqlite3_mutex)
    -> i32;
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex)
    -> ();
    fn sqlite3_mutex_held(_: *mut sqlite3_mutex)
    -> i32;
    fn sqlite3_mutex_notheld(_: *mut sqlite3_mutex)
    -> i32;
    fn sqlite3_db_mutex(_: *mut sqlite3)
    -> *mut sqlite3_mutex;
    fn sqlite3_file_control(_: *mut sqlite3, z_db_name_1: *const i8, op: i32,
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
    fn sqlite3_str_new(_: *mut sqlite3)
    -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str)
    -> *mut i8;
    fn sqlite3_str_free(_: *mut sqlite3_str)
    -> ();
    fn sqlite3_result_str(_: *mut sqlite3_context, _: *mut sqlite3_str,
    _: i32)
    -> ();
    fn sqlite3_str_appendf(_: *mut sqlite3_str, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_str_vappendf(_: *mut sqlite3_str, z_format_1: *const i8,
    _: *mut i8)
    -> ();
    fn sqlite3_str_append(_: *mut sqlite3_str, z_in_1: *const i8, n_1: i32)
    -> ();
    fn sqlite3_str_appendall(_: *mut sqlite3_str, z_in_1: *const i8)
    -> ();
    fn sqlite3_str_appendchar(_: *mut sqlite3_str, n_1: i32, c_1: i8)
    -> ();
    fn sqlite3_str_reset(_: *mut sqlite3_str)
    -> ();
    fn sqlite3_str_truncate(_: *mut sqlite3_str, n_1: i32)
    -> ();
    fn sqlite3_str_errcode(_: *mut sqlite3_str)
    -> i32;
    fn sqlite3_str_length(_: *mut sqlite3_str)
    -> i32;
    fn sqlite3_str_value(_: *mut sqlite3_str)
    -> *mut i8;
    fn sqlite3_status(op: i32, p_current_1: *mut i32, p_highwater_1: *mut i32,
    reset_flag_1: i32)
    -> i32;
    fn sqlite3_status64(op: i32, p_current_1: *mut sqlite3_int64,
    p_highwater_1: *mut sqlite3_int64, reset_flag_1: i32)
    -> i32;
    fn sqlite3_db_status(_: *mut sqlite3, op: i32, p_cur_1: *mut i32,
    p_hiwtr_1: *mut i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_db_status64(_: *mut sqlite3, _: i32, _: *mut sqlite3_int64,
    _: *mut sqlite3_int64, _: i32)
    -> i32;
    fn sqlite3_stmt_status(_: *mut sqlite3_stmt, op: i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_backup_init(p_dest_1: *mut sqlite3, z_dest_name_1: *const i8,
    p_source_1: *mut sqlite3, z_source_name_1: *const i8)
    -> *mut sqlite3_backup;
    fn sqlite3_backup_step(p: *mut sqlite3_backup, n_page_1: i32)
    -> i32;
    fn sqlite3_backup_finish(p: *mut sqlite3_backup)
    -> i32;
    fn sqlite3_backup_remaining(p: *mut sqlite3_backup)
    -> i32;
    fn sqlite3_backup_pagecount(p: *mut sqlite3_backup)
    -> i32;
    fn sqlite3_unlock_notify(p_blocked_1: *mut sqlite3,
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
    fn sqlite3_wal_hook(_: *mut sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), *mut sqlite3, *const i8, i32)
            -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_wal_autocheckpoint(db: *mut sqlite3, n_1: i32)
    -> i32;
    fn sqlite3_wal_checkpoint(db: *mut sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_wal_checkpoint_v2(db: *mut sqlite3, z_db_1: *const i8,
    e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32;
    fn sqlite3_vtab_config(_: *mut sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_vtab_on_conflict(_: *mut sqlite3)
    -> i32;
    fn sqlite3_vtab_nochange(_: *mut sqlite3_context)
    -> i32;
    fn sqlite3_vtab_collation(_: *mut sqlite3_index_info, _: i32)
    -> *const i8;
    fn sqlite3_vtab_distinct(_: *mut sqlite3_index_info)
    -> i32;
    fn sqlite3_vtab_in(_: *mut sqlite3_index_info, i_cons_1: i32,
    b_handle_1: i32)
    -> i32;
    fn sqlite3_vtab_in_first(p_val_1: *mut sqlite3_value,
    pp_out_1: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_vtab_in_next(p_val_1: *mut sqlite3_value,
    pp_out_1: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_vtab_rhs_value(_: *mut sqlite3_index_info, _: i32,
    pp_val_1: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_stmt_scanstatus(p_stmt_1: *mut sqlite3_stmt, idx: i32,
    i_scan_status_op_1: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_v2(p_stmt_1: *mut sqlite3_stmt, idx: i32,
    i_scan_status_op_1: i32, flags: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_reset(_: *mut sqlite3_stmt)
    -> ();
    fn sqlite3_db_cacheflush(_: *mut sqlite3)
    -> i32;
    fn sqlite3_system_errno(_: *mut sqlite3)
    -> i32;
    fn sqlite3_snapshot_get(db: *mut sqlite3, z_schema_1: *const i8,
    pp_snapshot_1: *mut *mut sqlite3_snapshot)
    -> i32;
    fn sqlite3_snapshot_open(db: *mut sqlite3, z_schema_1: *const i8,
    p_snapshot_1: *mut sqlite3_snapshot)
    -> i32;
    fn sqlite3_snapshot_free(_: *mut sqlite3_snapshot)
    -> ();
    fn sqlite3_snapshot_cmp(p1: *mut sqlite3_snapshot,
    p2: *mut sqlite3_snapshot)
    -> i32;
    fn sqlite3_snapshot_recover(db: *mut sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_serialize(db: *mut sqlite3, z_schema_1: *const i8,
    pi_size_1: *mut sqlite3_int64, m_flags_1: u32)
    -> *mut u8;
    fn sqlite3_deserialize(db: *mut sqlite3, z_schema_1: *const i8,
    p_data_1: *mut u8, sz_db_1: sqlite3_int64, sz_buf_1: sqlite3_int64,
    m_flags_1: u32)
    -> i32;
    fn sqlite3_carray_bind_v2(p_stmt_1: *mut sqlite3_stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, p_del_1: *mut ())
    -> i32;
    fn sqlite3_carray_bind(p_stmt_1: *mut sqlite3_stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_rtree_geometry_callback(db: *mut sqlite3, z_geom_1: *const i8,
    x_geom_1:
        Option<unsafe extern "C" fn(*mut sqlite3_rtree_geometry, i32,
            *mut f64, *mut i32) -> i32>, p_context_1: *mut ())
    -> i32;
    fn sqlite3_rtree_query_callback(db: *mut sqlite3,
    z_query_func_1: *const i8,
    x_query_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_rtree_query_info) -> i32>,
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
    fn stat(_: *const i8, _: *mut stat)
    -> i32;
    fn lstat(_: *const i8, _: *mut stat)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn mkdir(_: *const i8, _: mode_t)
    -> i32;
    fn unlink(_: *const i8)
    -> i32;
    fn symlink(_: *const i8, _: *const i8)
    -> i32;
    fn __error()
    -> *mut i32;
    fn chmod(_: *const i8, _: mode_t)
    -> i32;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn time(_: *mut time_t)
    -> time_t;
    fn utimes(_: *const i8, _: *const timeval)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn closedir(_: *mut DIR)
    -> i32;
    fn opendir(_: *const i8)
    -> *mut DIR;
    fn readdir(_: *mut DIR)
    -> *mut dirent;
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
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;