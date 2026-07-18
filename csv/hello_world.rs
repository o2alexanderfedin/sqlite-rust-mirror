#![feature(c_variadic)]
type __darwin_size_t = u64;
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
#[repr(C)]
#[derive(Copy, Clone)]
struct CsvReader {
    in_: *mut FILE,
    z: *mut i8,
    n: i64,
    n_alloc: i64,
    n_line: i64,
    b_not_first: i32,
    c_term: i32,
    i_in: u64,
    n_in: u64,
    z_in: *mut i8,
    z_err: [i8; 200],
}
extern "C" fn csv_reader_init(p: &mut CsvReader) -> () {
    (*p).in_ = core::ptr::null_mut();
    (*p).z = core::ptr::null_mut();
    (*p).n = 0 as i64;
    (*p).n_alloc = 0 as i64;
    (*p).n_line = 0 as i64;
    (*p).b_not_first = 0;
    (*p).n_in = 0 as u64;
    (*p).z_in = core::ptr::null_mut();
    (*p).z_err[0 as usize] = 0 as i8;
}
extern "C" fn csv_reader_reset(p: *mut CsvReader) -> () {
    if !(unsafe { (*p).in_ }).is_null() {
        unsafe { fclose(unsafe { (*p).in_ }) };
        unsafe { sqlite3_free(unsafe { (*p).z_in } as *mut ()) };
    }
    unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
    csv_reader_init(unsafe { &mut *p });
}
unsafe extern "C" fn csv_errmsg(p: &mut CsvReader, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe {
        sqlite3_vsnprintf(200, &raw mut (*p).z_err[0 as usize] as *mut i8,
            z_format_1, ap)
    };
    ();
}
extern "C" fn csv_reader_open(p: *mut CsvReader, z_filename_1: *const i8,
    z_data_1: *const i8) -> i32 {
    if !(z_filename_1).is_null() {
        unsafe {
            (*p).z_in =
                unsafe { sqlite3_malloc64(1024 as sqlite3_uint64) } as *mut i8
        };
        if unsafe { (*p).z_in } == core::ptr::null_mut() {
            unsafe {
                csv_errmsg(unsafe { &mut *p },
                    c"out of memory".as_ptr() as *mut i8 as *const i8)
            };
            return 1;
        }
        unsafe {
            (*p).in_ =
                unsafe {
                    fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
                }
        };
        if unsafe { (*p).in_ } == core::ptr::null_mut() {
            unsafe { sqlite3_free(unsafe { (*p).z_in } as *mut ()) };
            csv_reader_reset(p);
            unsafe {
                csv_errmsg(unsafe { &mut *p },
                    c"cannot open \'%s\' for reading".as_ptr() as *mut i8 as
                        *const i8, z_filename_1)
            };
            return 1;
        }
    } else {
        if !(unsafe { (*p).in_ } == core::ptr::null_mut()) as i32 as i64 != 0
            {
            unsafe {
                __assert_rtn(c"csv_reader_open".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 145,
                    c"p->in==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p).z_in = z_data_1 as *mut i8 };
        unsafe { (*p).n_in = unsafe { strlen(z_data_1) } };
    }
    return 0;
}
extern "C" fn csv_getc_refill(p: &mut CsvReader) -> i32 {
    let mut got: u64 = 0 as u64;
    if !((*p).i_in >= (*p).n_in) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"csv_getc_refill".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 158,
                c"p->iIn>=p->nIn".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !((*p).in_ != core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"csv_getc_refill".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 159,
                c"p->in!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    got =
        unsafe {
            fread((*p).z_in as *mut (), 1 as u64, 1024 as u64, (*p).in_)
        };
    if got == 0 as u64 { return -1; }
    (*p).n_in = got;
    (*p).i_in = 1 as u64;
    return unsafe { *(*p).z_in.offset(0 as isize) } as i32;
}
extern "C" fn csv_getc(p: *mut CsvReader) -> i32 {
    if unsafe { (*p).i_in } >= unsafe { (*p).n_in } {
        if unsafe { (*p).in_ } != core::ptr::null_mut() {
            return csv_getc_refill(unsafe { &mut *p });
        }
        return -1;
    }
    return unsafe {
                *(unsafe { (*p).z_in } as
                            *mut u8).add({
                                let __p = unsafe { &mut (*p).i_in };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as usize)
            } as i32;
}
extern "C" fn csv_resize_and_append(p: *mut CsvReader, c: i8) -> i32 {
    let mut z_new: *mut i8 = core::ptr::null_mut();
    let n_new: i64 = unsafe { (*p).n_alloc } * 2 as i64 + 100 as i64;
    z_new =
        unsafe {
                sqlite3_realloc64(unsafe { (*p).z } as *mut (),
                    n_new as sqlite3_uint64)
            } as *mut i8;
    if !(z_new).is_null() {
        unsafe { (*p).z = z_new };
        unsafe { (*p).n_alloc = n_new };
        unsafe {
            *unsafe {
                        (*p).z.offset({
                                    let __p = unsafe { &mut (*p).n };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                    } = c
        };
        return 0;
    } else {
        unsafe {
            csv_errmsg(unsafe { &mut *p },
                c"out of memory".as_ptr() as *mut i8 as *const i8)
        };
        return 1;
    }
}
extern "C" fn csv_append(p: *mut CsvReader, c: i8) -> i32 {
    if unsafe { (*p).n } >= unsafe { (*p).n_alloc } - 1 as i64 {
        return csv_resize_and_append(p, c);
    }
    unsafe {
        *unsafe {
                    (*p).z.offset({
                                let __p = unsafe { &mut (*p).n };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize)
                } = c
    };
    return 0;
}
extern "C" fn csv_read_one_field(p: *mut CsvReader) -> *mut i8 {
    let mut c: i32 = 0;
    unsafe { (*p).n = 0 as i64 };
    c = csv_getc(p);
    if c == -1 { unsafe { (*p).c_term = -1 }; return core::ptr::null_mut(); }
    if c == '\"' as i32 {
        let mut pc: i32 = 0;
        let mut ppc: i32 = 0;
        let start_line: i64 = unsafe { (*p).n_line };
        pc = { ppc = 0; ppc };
        loop {
            c = csv_getc(p);
            if c <= '\"' as i32 || pc == '\"' as i32 {
                if c == '\n' as i32 {
                    {
                        let __p = unsafe { &mut (*p).n_line };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                if c == '\"' as i32 {
                    if pc == '\"' as i32 { pc = 0; continue; }
                }
                if c == ',' as i32 && pc == '\"' as i32 ||
                                c == '\n' as i32 && pc == '\"' as i32 ||
                            c == '\n' as i32 && pc == '\r' as i32 && ppc == '\"' as i32
                        || c == -1 && pc == '\"' as i32 {
                    '__b1: loop {
                        '__c1: loop {
                            {
                                let __p = unsafe { &mut (*p).n };
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            };
                            break '__c1;
                        }
                        if !(unsafe {
                                                *unsafe { (*p).z.offset(unsafe { (*p).n } as isize) }
                                            } as i32 != '\"' as i32) {
                            break '__b1;
                        }
                    }
                    unsafe { (*p).c_term = c as i8 as i32 };
                    break;
                }
                if pc == '\"' as i32 && c != '\r' as i32 {
                    unsafe {
                        csv_errmsg(unsafe { &mut *p },
                            c"line %d: unescaped %c character".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p).n_line }, '\"' as i32)
                    };
                    break;
                }
                if c == -1 {
                    unsafe {
                        csv_errmsg(unsafe { &mut *p },
                            c"line %d: unterminated %c-quoted field\n".as_ptr() as
                                    *mut i8 as *const i8, start_line, '\"' as i32)
                    };
                    unsafe { (*p).c_term = c as i8 as i32 };
                    break;
                }
            }
            if csv_append(p, c as i8) != 0 { return core::ptr::null_mut(); }
            ppc = pc;
            pc = c;
        }
    } else {
        if c & 255 == 239 && unsafe { (*p).b_not_first } == 0 {
            csv_append(p, c as i8);
            c = csv_getc(p);
            if c & 255 == 187 {
                csv_append(p, c as i8);
                c = csv_getc(p);
                if c & 255 == 191 {
                    unsafe { (*p).b_not_first = 1 };
                    unsafe { (*p).n = 0 as i64 };
                    return csv_read_one_field(p);
                }
            }
        }
        while c > ',' as i32 || c != -1 && c != ',' as i32 && c != '\n' as i32
            {
            if csv_append(p, c as i8) != 0 { return core::ptr::null_mut(); }
            c = csv_getc(p);
        }
        if c == '\n' as i32 {
            {
                let __p = unsafe { &mut (*p).n_line };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p).n } > 0 as i64 &&
                    unsafe {
                                *unsafe {
                                        (*p).z.offset((unsafe { (*p).n } - 1 as i64) as isize)
                                    }
                            } as i32 == '\r' as i32 {
                {
                    let __p = unsafe { &mut (*p).n };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
        }
        unsafe { (*p).c_term = c as i8 as i32 };
    }
    if !(unsafe { (*p).z } == core::ptr::null_mut() ||
                            unsafe { (*p).n } < unsafe { (*p).n_alloc }) as i32 as i64
            != 0 {
        unsafe {
            __assert_rtn(c"csv_read_one_field".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 287,
                c"p->z==0 || p->n<p->nAlloc".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p).z }).is_null() {
        unsafe {
            *unsafe { (*p).z.offset(unsafe { (*p).n } as isize) } = 0 as i8
        };
    }
    unsafe { (*p).b_not_first = 1 };
    return unsafe { (*p).z };
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CsvTable {
    base: sqlite3_vtab,
    z_filename: *mut i8,
    z_data: *mut i8,
    i_start: i64,
    n_col: i32,
    tst_flags: u32,
}
extern "C" fn csv_skip_whitespace(mut z: *const i8) -> *const i8 {
    while unsafe { isspace(unsafe { *z.offset(0 as isize) } as u8 as i32) } !=
            0 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return z;
}
extern "C" fn csv_parameter(z_tag_1: *const i8, n_tag_1: i32,
    mut z: *const i8) -> *const i8 {
    z = csv_skip_whitespace(z);
    if unsafe { strncmp(z_tag_1, z, n_tag_1 as u64) } != 0 {
        return core::ptr::null();
    }
    z = csv_skip_whitespace(unsafe { z.offset(n_tag_1 as isize) });
    if unsafe { *z.offset(0 as isize) } as i32 != '=' as i32 {
        return core::ptr::null();
    }
    return csv_skip_whitespace(unsafe { z.offset(1 as isize) });
}
extern "C" fn csv_trim_whitespace(z: *mut i8) -> () {
    let mut n: u64 = unsafe { strlen(z as *const i8) };
    while n > 0 as u64 &&
            unsafe { isspace(unsafe { *z.add(n as usize) } as u8 as i32) } !=
                0 {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    unsafe { *z.add(n as usize) = 0 as i8 };
}
extern "C" fn csv_dequote(z: *mut i8) -> () {
    let mut j: i32 = 0;
    let c_quote: i8 = unsafe { *z.offset(0 as isize) };
    let mut i: u64 = 0 as u64;
    let mut n: u64 = 0 as u64;
    if c_quote as i32 != '\'' as i32 && c_quote as i32 != '\"' as i32 {
        return;
    }
    n = unsafe { strlen(z as *const i8) };
    if n < 2 as u64 ||
            unsafe { *z.add((n - 1 as u64) as usize) } as i32 !=
                unsafe { *z.offset(0 as isize) } as i32 {
        return;
    }
    {
        { ({ i = 1 as u64; i }) as i32; j = 0 };
        '__b5: loop {
            if !(i < n - 1 as u64) { break '__b5; }
            '__c5: loop {
                if unsafe { *z.add(i as usize) } as i32 == c_quote as i32 &&
                        unsafe { *z.add((i + 1 as u64) as usize) } as i32 ==
                            c_quote as i32 {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
                unsafe {
                    *z.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = unsafe { *z.add(i as usize) }
                };
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z.offset(j as isize) = 0 as i8 };
}
extern "C" fn csv_string_parameter(p: *mut CsvReader, z_param_1: *const i8,
    z_arg_1: *const i8, pz_val_1: &mut *mut i8) -> i32 {
    let mut z_value: *const i8 = core::ptr::null();
    z_value =
        csv_parameter(z_param_1, unsafe { strlen(z_param_1) } as i32,
            z_arg_1);
    if z_value == core::ptr::null() { return 0; }
    unsafe { (*p).z_err[0 as usize] = 0 as i8 };
    if !(*pz_val_1).is_null() {
        unsafe {
            csv_errmsg(unsafe { &mut *p },
                c"more than one \'%s\' parameter".as_ptr() as *mut i8 as
                    *const i8, z_param_1)
        };
        return 1;
    }
    *pz_val_1 =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z_value)
        };
    if *pz_val_1 == core::ptr::null_mut() {
        unsafe {
            csv_errmsg(unsafe { &mut *p },
                c"out of memory".as_ptr() as *mut i8 as *const i8)
        };
        return 1;
    }
    csv_trim_whitespace(*pz_val_1);
    csv_dequote(*pz_val_1);
    return 1;
}
extern "C" fn csvtab_disconnect(p_vtab: *mut sqlite3_vtab) -> i32 {
    let p: *mut CsvTable = p_vtab as *mut CsvTable;
    unsafe { sqlite3_free(unsafe { (*p).z_filename } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).z_data } as *mut ()) };
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
extern "C" fn csv_boolean(z: *const i8) -> i32 {
    if unsafe { sqlite3_stricmp(c"yes".as_ptr() as *mut i8 as *const i8, z) }
                        == 0 ||
                    unsafe {
                            sqlite3_stricmp(c"on".as_ptr() as *mut i8 as *const i8, z)
                        } == 0 ||
                unsafe {
                        sqlite3_stricmp(c"true".as_ptr() as *mut i8 as *const i8, z)
                    } == 0 ||
            unsafe { *z.offset(0 as isize) } as i32 == '1' as i32 &&
                unsafe { *z.offset(1 as isize) } as i32 == 0 {
        return 1;
    }
    if unsafe { sqlite3_stricmp(c"no".as_ptr() as *mut i8 as *const i8, z) }
                        == 0 ||
                    unsafe {
                            sqlite3_stricmp(c"off".as_ptr() as *mut i8 as *const i8, z)
                        } == 0 ||
                unsafe {
                        sqlite3_stricmp(c"false".as_ptr() as *mut i8 as *const i8,
                            z)
                    } == 0 ||
            unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                unsafe { *z.offset(1 as isize) } as i32 == 0 {
        return 0;
    }
    return -1;
}
extern "C" fn csv_boolean_parameter(z_tag_1: *const i8, n_tag_1: i32,
    mut z: *const i8, p_value_1: &mut i32) -> i32 {
    let mut b: i32 = 0;
    z = csv_skip_whitespace(z);
    if unsafe { strncmp(z_tag_1, z, n_tag_1 as u64) } != 0 { return 0; }
    z = csv_skip_whitespace(unsafe { z.offset(n_tag_1 as isize) });
    if unsafe { *z.offset(0 as isize) } as i32 == 0 {
        *p_value_1 = 1;
        return 1;
    }
    if unsafe { *z.offset(0 as isize) } as i32 != '=' as i32 { return 0; }
    z = csv_skip_whitespace(unsafe { z.offset(1 as isize) });
    b = csv_boolean(z);
    if b >= 0 { *p_value_1 = b; return 1; }
    return 0;
}
extern "C" fn csvtab_connect(db: *mut sqlite3, p_aux: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab: *mut *mut sqlite3_vtab,
    pz_err: *mut *mut i8) -> i32 {
    unsafe {
        let mut p_new: *mut CsvTable = core::ptr::null_mut();
        let mut b_header: i32 = 0;
        let mut rc: i32 = 0;
        let mut i: u64 = 0 as u64;
        let mut j: u64 = 0 as u64;
        let mut b: i32 = 0;
        let mut n_col: i32 = 0;
        let mut s_rdr: CsvReader = unsafe { core::mem::zeroed() };
        let mut az_p_value: [*mut i8; 3] = [core::ptr::null_mut(); 3];
        let mut z: *const i8 = core::ptr::null();
        let mut z_value: *const i8 = core::ptr::null();
        let mut p_str: *mut sqlite3_str = core::ptr::null_mut();
        let mut z_sep: *mut i8 = core::ptr::null_mut();
        let mut i_col: i32 = 0;
        let mut z__1: *mut i8 = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s7:
                {
                match __state {
                    0 => { p_new = core::ptr::null_mut(); __state = 4; }
                    2 => { rc = 7; __state = 114; }
                    3 => {
                        if !(p_new).is_null() {
                            __state = 117;
                        } else { __state = 116; }
                    }
                    4 => { b_header = -1; __state = 5; }
                    5 => { rc = 0; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { n_col = -99; __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => {
                        if !(core::mem::size_of::<[*mut i8; 3]>() as u64 ==
                                                core::mem::size_of::<[*const i8; 3]>() as u64) as i32 as i64
                                != 0 {
                            unsafe {
                                __assert_rtn(c"csvtabConnect".as_ptr() as *const i8,
                                    c"csv.c".as_ptr() as *mut i8 as *const i8, 517,
                                    c"sizeof(azPValue)==sizeof(azParam)".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 13;
                    }
                    13 => {
                        unsafe {
                            memset(&raw mut s_rdr as *mut (), 0,
                                core::mem::size_of::<CsvReader>() as u64)
                        };
                        __state = 14;
                    }
                    14 => {
                        unsafe {
                            memset(&raw mut az_p_value[0 as usize] as *mut *mut i8 as
                                    *mut (), 0, core::mem::size_of::<[*mut i8; 3]>() as u64)
                        };
                        __state = 15;
                    }
                    15 => { i = 3 as u64; __state = 17; }
                    16 => {
                        if (az_p_value[0 as usize] == core::ptr::null_mut()) as i32
                                == (az_p_value[1 as usize] == core::ptr::null_mut()) as i32
                            {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    17 => {
                        if i < argc as u64 { __state = 18; } else { __state = 16; }
                    }
                    18 => {
                        z = unsafe { *argv.add(i as usize) };
                        __state = 20;
                    }
                    19 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 17;
                    }
                    20 => { __state = 21; }
                    21 => { j = 0 as u64; __state = 23; }
                    22 => {
                        if j <
                                core::mem::size_of::<[*const i8; 3]>() as u64 /
                                    core::mem::size_of::<*const i8>() as u64 {
                            __state = 27;
                        } else { __state = 28; }
                    }
                    23 => {
                        if j <
                                core::mem::size_of::<[*const i8; 3]>() as u64 /
                                    core::mem::size_of::<*const i8>() as u64 {
                            __state = 24;
                        } else { __state = 22; }
                    }
                    24 => {
                        if csv_string_parameter(&mut s_rdr, az_param[j as usize], z,
                                    &mut az_p_value[j as usize]) != 0 {
                            __state = 26;
                        } else { __state = 25; }
                    }
                    25 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 23;
                    }
                    26 => { __state = 22; }
                    27 => {
                        if s_rdr.z_err[0 as usize] != 0 {
                            __state = 29;
                        } else { __state = 19; }
                    }
                    28 => {
                        if csv_boolean_parameter(c"header".as_ptr() as *mut i8 as
                                        *const i8, 6, z, &mut b) != 0 {
                            __state = 30;
                        } else { __state = 31; }
                    }
                    29 => { __state = 3; }
                    30 => {
                        if b_header >= 0 { __state = 33; } else { __state = 32; }
                    }
                    31 => {
                        if {
                                    z_value =
                                        csv_parameter(c"columns".as_ptr() as *mut i8 as *const i8,
                                            7, z);
                                    z_value
                                } != core::ptr::null() {
                            __state = 35;
                        } else { __state = 36; }
                    }
                    32 => { b_header = b; __state = 19; }
                    33 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"more than one \'header\' parameter".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 34;
                    }
                    34 => { __state = 3; }
                    35 => {
                        if n_col > 0 { __state = 38; } else { __state = 37; }
                    }
                    36 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"bad parameter: \'%s\'".as_ptr() as *mut i8 as *const i8,
                                z)
                        };
                        __state = 46;
                    }
                    37 => { n_col = unsafe { atoi(z_value) }; __state = 40; }
                    38 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"more than one \'columns\' parameter".as_ptr() as *mut i8
                                    as *const i8)
                        };
                        __state = 39;
                    }
                    39 => { __state = 3; }
                    40 => {
                        if n_col <= 0 { __state = 41; } else { __state = 42; }
                    }
                    41 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"column= value must be positive".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 43;
                    }
                    42 => {
                        if n_col > unsafe { sqlite3_limit(db, 2, -1) } {
                            __state = 44;
                        } else { __state = 19; }
                    }
                    43 => { __state = 3; }
                    44 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"column= value too big, max %d".as_ptr() as *mut i8 as
                                    *const i8, unsafe { sqlite3_limit(db, 2, -1) })
                        };
                        __state = 45;
                    }
                    45 => { __state = 3; }
                    46 => { __state = 3; }
                    47 => {
                        if (n_col <= 0 || b_header == 1) &&
                                csv_reader_open(&mut s_rdr,
                                        az_p_value[0 as usize] as *const i8,
                                        az_p_value[1 as usize] as *const i8) != 0 {
                            __state = 51;
                        } else { __state = 50; }
                    }
                    48 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"must specify either filename= or data= but not both".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 49;
                    }
                    49 => { __state = 3; }
                    50 => {
                        p_new =
                            unsafe {
                                    sqlite3_malloc64(core::mem::size_of::<CsvTable>() as
                                            sqlite3_uint64)
                                } as *mut CsvTable;
                        __state = 52;
                    }
                    51 => { __state = 3; }
                    52 => {
                        unsafe { *pp_vtab = p_new as *mut sqlite3_vtab };
                        __state = 53;
                    }
                    53 => {
                        if p_new == core::ptr::null_mut() {
                            __state = 55;
                        } else { __state = 54; }
                    }
                    54 => {
                        unsafe {
                            memset(p_new as *mut (), 0,
                                core::mem::size_of::<CsvTable>() as u64)
                        };
                        __state = 56;
                    }
                    55 => { __state = 2; }
                    56 => {
                        if az_p_value[2 as usize] == core::ptr::null_mut() {
                            __state = 58;
                        } else { __state = 59; }
                    }
                    57 => {
                        unsafe { (*p_new).z_filename = az_p_value[0 as usize] };
                        __state = 94;
                    }
                    58 => {
                        p_str = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        __state = 60;
                    }
                    59 => {
                        if n_col < 0 { __state = 90; } else { __state = 91; }
                    }
                    60 => { z_sep = c"".as_ptr() as *mut i8; __state = 61; }
                    61 => { i_col = 0; __state = 62; }
                    62 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"CREATE TABLE x(".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 63;
                    }
                    63 => {
                        if n_col < 0 && b_header < 1 {
                            __state = 65;
                        } else { __state = 64; }
                    }
                    64 => {
                        if n_col > 0 && b_header < 1 {
                            __state = 70;
                        } else { __state = 71; }
                    }
                    65 => { n_col = 0; __state = 66; }
                    66 => { csv_read_one_field(&mut s_rdr); __state = 68; }
                    67 => {
                        if s_rdr.c_term == ',' as i32 {
                            __state = 66;
                        } else { __state = 64; }
                    }
                    68 => {
                        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                        __state = 67;
                    }
                    69 => { unsafe { (*p_new).n_col = n_col }; __state = 86; }
                    70 => { i_col = 0; __state = 72; }
                    71 => {
                        z__1 = csv_read_one_field(&mut s_rdr);
                        __state = 78;
                    }
                    72 => {
                        if i_col < n_col { __state = 73; } else { __state = 69; }
                    }
                    73 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"%sc%d TEXT".as_ptr() as *mut i8 as *const i8, z_sep,
                                i_col)
                        };
                        __state = 75;
                    }
                    74 => {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                        __state = 72;
                    }
                    75 => { z_sep = c",".as_ptr() as *mut i8; __state = 74; }
                    76 => {
                        if n_col < 0 { __state = 82; } else { __state = 83; }
                    }
                    77 => {
                        if s_rdr.c_term == ',' as i32 {
                            __state = 71;
                        } else { __state = 76; }
                    }
                    78 => {
                        if n_col > 0 && i_col < n_col || n_col < 0 && b_header != 0
                            {
                            __state = 79;
                        } else { __state = 77; }
                    }
                    79 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"%s\"%w\" TEXT".as_ptr() as *mut i8 as *const i8, z_sep,
                                z__1)
                        };
                        __state = 80;
                    }
                    80 => { z_sep = c",".as_ptr() as *mut i8; __state = 81; }
                    81 => {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                        __state = 77;
                    }
                    82 => { n_col = i_col; __state = 69; }
                    83 => {
                        if i_col < n_col { __state = 84; } else { __state = 69; }
                    }
                    84 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c"%sc%d TEXT".as_ptr() as *mut i8 as *const i8, z_sep,
                                { let __p = &mut i_col; *__p += 1; *__p })
                        };
                        __state = 85;
                    }
                    85 => { z_sep = c",".as_ptr() as *mut i8; __state = 83; }
                    86 => {
                        unsafe {
                            sqlite3_str_appendf(p_str,
                                c")".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 87;
                    }
                    87 => {
                        az_p_value[2 as usize] =
                            unsafe { sqlite3_str_finish(p_str) };
                        __state = 88;
                    }
                    88 => {
                        if az_p_value[2 as usize] == core::ptr::null_mut() {
                            __state = 89;
                        } else { __state = 57; }
                    }
                    89 => { __state = 2; }
                    90 => { csv_read_one_field(&mut s_rdr); __state = 93; }
                    91 => { unsafe { (*p_new).n_col = n_col }; __state = 57; }
                    92 => {
                        if s_rdr.c_term == ',' as i32 {
                            __state = 90;
                        } else { __state = 57; }
                    }
                    93 => {
                        {
                            let __p = unsafe { &mut (*p_new).n_col };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 92;
                    }
                    94 => {
                        az_p_value[0 as usize] = core::ptr::null_mut();
                        __state = 95;
                    }
                    95 => {
                        unsafe { (*p_new).z_data = az_p_value[1 as usize] };
                        __state = 96;
                    }
                    96 => {
                        az_p_value[1 as usize] = core::ptr::null_mut();
                        __state = 97;
                    }
                    97 => {
                        if b_header != 1 { __state = 99; } else { __state = 100; }
                    }
                    98 => { csv_reader_reset(&mut s_rdr); __state = 103; }
                    99 => {
                        unsafe { (*p_new).i_start = 0 as i64 };
                        __state = 98;
                    }
                    100 => {
                        if !(unsafe { (*p_new).z_data }).is_null() {
                            __state = 101;
                        } else { __state = 102; }
                    }
                    101 => {
                        unsafe { (*p_new).i_start = s_rdr.i_in as i32 as i64 };
                        __state = 98;
                    }
                    102 => {
                        unsafe {
                            (*p_new).i_start =
                                (unsafe { ftell(s_rdr.in_) } as u64 - s_rdr.n_in +
                                            s_rdr.i_in) as i32 as i64
                        };
                        __state = 98;
                    }
                    103 => {
                        rc =
                            unsafe {
                                sqlite3_declare_vtab(db,
                                    az_p_value[2 as usize] as *const i8)
                            };
                        __state = 104;
                    }
                    104 => {
                        if rc != 0 { __state = 106; } else { __state = 105; }
                    }
                    105 => { i = 0 as u64; __state = 109; }
                    106 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"bad schema: \'%s\' - %s".as_ptr() as *mut i8 as *const i8,
                                az_p_value[2 as usize], unsafe { sqlite3_errmsg(db) })
                        };
                        __state = 107;
                    }
                    107 => { __state = 3; }
                    108 => {
                        unsafe { sqlite3_vtab_config(db, 3) };
                        __state = 112;
                    }
                    109 => {
                        if i <
                                core::mem::size_of::<[*mut i8; 3]>() as u64 /
                                    core::mem::size_of::<*mut i8>() as u64 {
                            __state = 110;
                        } else { __state = 108; }
                    }
                    110 => {
                        unsafe { sqlite3_free(az_p_value[i as usize] as *mut ()) };
                        __state = 111;
                    }
                    111 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 109;
                    }
                    112 => { return 0; }
                    113 => { __state = 2; }
                    114 => {
                        unsafe {
                            csv_errmsg(&mut s_rdr,
                                c"out of memory".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 115;
                    }
                    115 => { __state = 3; }
                    116 => { i = 0 as u64; __state = 119; }
                    117 => {
                        csvtab_disconnect(unsafe { &mut (*p_new).base });
                        __state = 116;
                    }
                    118 => {
                        if s_rdr.z_err[0 as usize] != 0 {
                            __state = 123;
                        } else { __state = 122; }
                    }
                    119 => {
                        if i <
                                core::mem::size_of::<[*mut i8; 3]>() as u64 /
                                    core::mem::size_of::<*mut i8>() as u64 {
                            __state = 120;
                        } else { __state = 118; }
                    }
                    120 => {
                        unsafe { sqlite3_free(az_p_value[i as usize] as *mut ()) };
                        __state = 121;
                    }
                    121 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 119;
                    }
                    122 => { csv_reader_reset(&mut s_rdr); __state = 125; }
                    123 => {
                        unsafe { sqlite3_free(unsafe { *pz_err } as *mut ()) };
                        __state = 124;
                    }
                    124 => {
                        unsafe {
                            *pz_err =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        &raw mut s_rdr.z_err[0 as usize] as *mut i8)
                                }
                        };
                        __state = 122;
                    }
                    125 => {
                        if rc == 0 { __state = 127; } else { __state = 126; }
                    }
                    126 => { return rc; }
                    127 => { rc = 1; __state = 126; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn csvtab_create(db: *mut sqlite3, p_aux: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab: *mut *mut sqlite3_vtab,
    pz_err: *mut *mut i8) -> i32 {
    return csvtab_connect(db, p_aux, argc, argv, pp_vtab, pz_err);
}
extern "C" fn csvtab_best_index(tab: *mut sqlite3_vtab,
    p_idx_info: *mut sqlite3_index_info) -> i32 {
    unsafe { (*p_idx_info).estimated_cost = 1000000 as f64 };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CsvCursor {
    base: sqlite3_vtab_cursor,
    rdr: CsvReader,
    az_val: *mut *mut i8,
    a_len: *mut i64,
    i_rowid: sqlite3_int64,
}
extern "C" fn csv_xfer_error(p_tab_1: &mut CsvTable, p_rdr_1: &mut CsvReader)
    -> () {
    unsafe { sqlite3_free((*p_tab_1).base.z_err_msg as *mut ()) };
    (*p_tab_1).base.z_err_msg =
        unsafe {
            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                &raw mut (*p_rdr_1).z_err[0 as usize] as *mut i8)
        };
}
extern "C" fn csvtab_open(p: *mut sqlite3_vtab,
    pp_cursor: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let p_tab: *mut CsvTable = p as *mut CsvTable;
    let mut p_cur: *mut CsvCursor = core::ptr::null_mut();
    let mut n_byte: sqlite3_int64 =
        unsafe { (*p_tab).n_col } as sqlite3_int64;
    if !(unsafe { (*p_tab).n_col } < 32768) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"csvtabOpen".as_ptr() as *const i8,
                c"csv.c".as_ptr() as *mut i8 as *const i8, 717,
                c"pTab->nCol<32768".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n_byte =
        (core::mem::size_of::<CsvCursor>() as u64 +
                (core::mem::size_of::<*mut i8>() as u64 +
                        core::mem::size_of::<i64>() as u64) * n_byte as u64) as
            sqlite3_int64;
    p_cur =
        unsafe { sqlite3_malloc64(n_byte as sqlite3_uint64) } as
            *mut CsvCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe { memset(p_cur as *mut (), 0, n_byte as u64) };
    unsafe {
        (*p_cur).az_val =
            unsafe { &raw mut *p_cur.offset(1 as isize) } as *mut *mut i8
    };
    unsafe {
        (*p_cur).a_len =
            unsafe {
                    &raw mut *unsafe {
                                (*p_cur).az_val.offset(unsafe { (*p_tab).n_col } as isize)
                            }
                } as *mut i64
    };
    unsafe { *pp_cursor = unsafe { &mut (*p_cur).base } };
    if csv_reader_open(unsafe { &mut (*p_cur).rdr },
                unsafe { (*p_tab).z_filename } as *const i8,
                unsafe { (*p_tab).z_data } as *const i8) != 0 {
        csv_xfer_error(unsafe { &mut *p_tab }, unsafe { &mut (*p_cur).rdr });
        return 1;
    }
    return 0;
}
extern "C" fn csvtab_cursor_row_reset(p_cur_1: &mut CsvCursor) -> () {
    let p_tab: *const CsvTable =
        (*p_cur_1).base.p_vtab as *mut CsvTable as *const CsvTable;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b8: loop {
            if !(i < unsafe { (*p_tab).n_col }) { break '__b8; }
            '__c8: loop {
                unsafe {
                    sqlite3_free(unsafe {
                                *(*p_cur_1).az_val.offset(i as isize)
                            } as *mut ())
                };
                unsafe {
                    *(*p_cur_1).az_val.offset(i as isize) =
                        core::ptr::null_mut()
                };
                unsafe { *(*p_cur_1).a_len.offset(i as isize) = 0 as i64 };
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn csvtab_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut CsvCursor = cur as *mut CsvCursor;
    csvtab_cursor_row_reset(unsafe { &mut *p_cur });
    csv_reader_reset(unsafe { &mut (*p_cur).rdr });
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0;
}
extern "C" fn csvtab_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut CsvCursor = cur as *mut CsvCursor;
    let p_tab: *mut CsvTable = unsafe { (*cur).p_vtab } as *mut CsvTable;
    let mut i: i32 = 0;
    let mut z: *const i8 = core::ptr::null();
    '__b9: loop {
        '__c9: loop {
            z = csv_read_one_field(unsafe { &mut (*p_cur).rdr });
            if z == core::ptr::null_mut() { break '__b9; }
            if i < unsafe { (*p_tab).n_col } {
                if unsafe { *unsafe { (*p_cur).a_len.offset(i as isize) } } <
                        unsafe { (*p_cur).rdr.n } + 1 as i64 {
                    let z_new: *mut i8 =
                        unsafe {
                                sqlite3_realloc64(unsafe {
                                            *unsafe { (*p_cur).az_val.offset(i as isize) }
                                        } as *mut (),
                                    (unsafe { (*p_cur).rdr.n } + 1 as i64) as sqlite3_uint64)
                            } as *mut i8;
                    if z_new == core::ptr::null_mut() {
                        unsafe {
                            csv_errmsg(unsafe { &mut (*p_cur).rdr },
                                c"out of memory".as_ptr() as *mut i8 as *const i8)
                        };
                        csv_xfer_error(unsafe { &mut *p_tab },
                            unsafe { &mut (*p_cur).rdr });
                        break '__b9;
                    }
                    unsafe {
                        *unsafe { (*p_cur).az_val.offset(i as isize) } = z_new
                    };
                    unsafe {
                        *unsafe { (*p_cur).a_len.offset(i as isize) } =
                            unsafe { (*p_cur).rdr.n } + 1 as i64
                    };
                }
                unsafe {
                    memcpy(unsafe {
                                *unsafe { (*p_cur).az_val.offset(i as isize) }
                            } as *mut (), z as *const (),
                        (unsafe { (*p_cur).rdr.n } + 1 as i64) as u64)
                };
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
            break '__c9;
        }
        if !(unsafe { (*p_cur).rdr.c_term } == ',' as i32) { break '__b9; }
    }
    if z == core::ptr::null_mut() && i == 0 {
        unsafe { (*p_cur).i_rowid = -1 as sqlite3_int64 };
    } else {
        {
            let __p = unsafe { &mut (*p_cur).i_rowid };
            let __t = *__p;
            *__p += 1;
            __t
        };
        while i < unsafe { (*p_tab).n_col } {
            unsafe {
                sqlite3_free(unsafe {
                            *unsafe { (*p_cur).az_val.offset(i as isize) }
                        } as *mut ())
            };
            unsafe {
                *unsafe { (*p_cur).az_val.offset(i as isize) } =
                    core::ptr::null_mut()
            };
            unsafe {
                *unsafe { (*p_cur).a_len.offset(i as isize) } = 0 as i64
            };
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
extern "C" fn csvtab_filter(p_vtab_cursor: *mut sqlite3_vtab_cursor,
    idx_num: i32, idx_str: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let p_cur: *mut CsvCursor = p_vtab_cursor as *mut CsvCursor;
    let p_tab: *const CsvTable =
        unsafe { (*p_vtab_cursor).p_vtab } as *mut CsvTable as
            *const CsvTable;
    unsafe { (*p_cur).i_rowid = 0 as sqlite3_int64 };
    if csv_append(unsafe { &mut (*p_cur).rdr }, 0 as i8) != 0 { return 7; }
    if unsafe { (*p_cur).rdr.in_ } == core::ptr::null_mut() {
        if !(unsafe { (*p_cur).rdr.z_in } == unsafe { (*p_tab).z_data }) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"csvtabFilter".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 830,
                    c"pCur->rdr.zIn==pTab->zData".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !(unsafe { (*p_tab).i_start } >= 0 as i64) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"csvtabFilter".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 831,
                    c"pTab->iStart>=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(unsafe { (*p_tab).i_start } as u64 <=
                                unsafe { (*p_cur).rdr.n_in }) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"csvtabFilter".as_ptr() as *const i8,
                    c"csv.c".as_ptr() as *mut i8 as *const i8, 832,
                    c"(size_t)pTab->iStart<=pCur->rdr.nIn".as_ptr() as *mut i8
                        as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p_cur).rdr.i_in = unsafe { (*p_tab).i_start } as u64 };
    } else {
        unsafe {
            fseek(unsafe { (*p_cur).rdr.in_ }, unsafe { (*p_tab).i_start }, 0)
        };
        unsafe { (*p_cur).rdr.i_in = 0 as u64 };
        unsafe { (*p_cur).rdr.n_in = 0 as u64 };
    }
    return csvtab_next(p_vtab_cursor);
}
extern "C" fn csvtab_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const CsvCursor = cur as *mut CsvCursor as *const CsvCursor;
    return (unsafe { (*p_cur).i_rowid } < 0 as i64) as i32;
}
extern "C" fn csvtab_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_cur: *const CsvCursor = cur as *mut CsvCursor as *const CsvCursor;
    let p_tab: *const CsvTable =
        unsafe { (*cur).p_vtab } as *mut CsvTable as *const CsvTable;
    if i >= 0 && i < unsafe { (*p_tab).n_col } &&
            unsafe { *unsafe { (*p_cur).az_val.offset(i as isize) } } !=
                core::ptr::null_mut() {
        unsafe {
            sqlite3_result_text(ctx,
                unsafe { *unsafe { (*p_cur).az_val.offset(i as isize) } } as
                    *const i8, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
    return 0;
}
extern "C" fn csvtab_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid: *mut sqlite3_int64) -> i32 {
    let p_cur: *const CsvCursor = cur as *mut CsvCursor as *const CsvCursor;
    unsafe { *p_rowid = unsafe { (*p_cur).i_rowid } };
    return 0;
}
static mut csv_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: Some(csvtab_create),
        x_connect: Some(csvtab_connect),
        x_best_index: Some(csvtab_best_index),
        x_disconnect: Some(csvtab_disconnect),
        x_destroy: Some(csvtab_disconnect),
        x_open: Some(csvtab_open),
        x_close: Some(csvtab_close),
        x_filter: Some(csvtab_filter),
        x_next: Some(csvtab_next),
        x_eof: Some(csvtab_eof),
        x_column: Some(csvtab_column),
        x_rowid: Some(csvtab_rowid),
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
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_csv_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"csv".as_ptr() as *mut i8 as *const i8,
                    &raw mut csv_module as *const sqlite3_module,
                    core::ptr::null_mut())
            };
        return rc;
    }
}
static mut az_param: [*const i8; 3] =
    [c"filename".as_ptr() as *const i8, c"data".as_ptr() as *const i8,
            c"schema".as_ptr() as *const i8];
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
    fn fclose(_: *mut FILE)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn isspace(_c: i32)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn atoi(_: *const i8)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn __builtin_unreachable()
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
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;