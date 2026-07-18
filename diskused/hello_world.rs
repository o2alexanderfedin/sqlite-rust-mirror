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
struct DiskUsed {
    db: *mut sqlite3,
    context: *mut sqlite3_context,
    p_out: *mut sqlite3_str,
    z_su: *mut i8,
    z_schema: *const i8,
}
extern "C" fn diskused_reset(p: *mut DiskUsed) -> () {
    if !(unsafe { (*p).z_su }).is_null() {
        let z_sql: *mut i8 =
            unsafe {
                sqlite3_mprintf(c"DROP TABLE temp.%s;".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p).z_su })
            };
        if !(z_sql).is_null() {
            unsafe {
                sqlite3_exec(unsafe { (*p).db }, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
        }
    }
    unsafe { sqlite3_str_free(unsafe { (*p).p_out }) };
    unsafe { sqlite3_free(unsafe { (*p).z_su } as *mut ()) };
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<DiskUsed>() as u64)
    };
}
unsafe extern "C" fn diskused_error(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut z_err: *mut i8 = core::ptr::null_mut();
    if !(z_format_1).is_null() {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_err = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
    } else { z_err = core::ptr::null_mut(); }
    if z_err == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(unsafe { (*p).context }) };
    } else {
        unsafe {
            sqlite3_result_error(unsafe { (*p).context }, z_err as *const i8,
                -1)
        };
        unsafe { sqlite3_free(z_err as *mut ()) };
    }
    diskused_reset(p);
}
extern "C" fn diskused_v_prep(p: *mut DiskUsed, z_fmt_1: *const i8,
    ap: *mut i8) -> *mut sqlite3_stmt {
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    z_sql = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    if z_sql == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return core::ptr::null_mut();
    }
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
    if rc != 0 {
        unsafe {
            diskused_error(p,
                c"SQL parse error: %s\nOriginal SQL: %s".as_ptr() as *mut i8
                    as *const i8, unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                z_sql)
        };
        unsafe { sqlite3_finalize(p_stmt) };
        diskused_reset(p);
        p_stmt = core::ptr::null_mut();
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    return p_stmt;
}
unsafe extern "C" fn diskused_prepare(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> *mut sqlite3_stmt {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = diskused_v_prep(p, z_format_1, ap);
    ();
    return p_stmt;
}
extern "C" fn diskused_stmt_finish(p: *mut DiskUsed, mut rc: i32,
    p_stmt_1: *mut sqlite3_stmt) -> i32 {
    if rc == 101 { rc = 0; }
    if rc != 0 || { rc = unsafe { sqlite3_reset(p_stmt_1) }; rc } != 0 {
        unsafe {
            diskused_error(p,
                c"SQL run-time error: %s\nOriginal SQL: %s".as_ptr() as
                        *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                unsafe { sqlite3_sql(p_stmt_1) })
        };
        diskused_reset(p);
    }
    unsafe { sqlite3_finalize(p_stmt_1) };
    return rc;
}
unsafe extern "C" fn diskused_sql(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = diskused_v_prep(p, z_format_1, ap);
    ();
    if p_stmt == core::ptr::null_mut() { return 1; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {}
    if rc == 101 {
        rc = 0;
    } else {
        unsafe {
            diskused_error(p,
                c"SQL run-time error: %s\nOriginal SQL: %s".as_ptr() as
                        *mut i8 as *const i8,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                unsafe { sqlite3_sql(p_stmt) })
        };
        diskused_reset(p);
    }
    unsafe { sqlite3_finalize(p_stmt) };
    return rc;
}
unsafe extern "C" fn diskused_sql_int(p: *mut DiskUsed,
    pi_res_1: &mut sqlite3_int64, z_format_1: *const i8, mut __va0: ...)
    -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = diskused_v_prep(p, z_format_1, ap);
    ();
    if p_stmt == core::ptr::null_mut() { return 1; }
    rc = unsafe { sqlite3_step(p_stmt) };
    if rc == 100 {
        *pi_res_1 = unsafe { sqlite3_column_int64(p_stmt, 0) };
        rc = 0;
    } else if rc == 101 {
        rc = 0;
    } else {
        if !(unsafe { (*p).db }).is_null() {
            unsafe {
                diskused_error(p,
                    c"SQL run-time error: %s\nOriginal SQL: %s".as_ptr() as
                            *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                    unsafe { sqlite3_sql(p_stmt) })
            };
        }
        diskused_reset(p);
    }
    unsafe { sqlite3_finalize(p_stmt) };
    return rc;
}
unsafe extern "C" fn diskused_title(p: *mut DiskUsed, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut z_first: *mut i8 = core::ptr::null_mut();
    let mut z_title: *mut i8 = core::ptr::null_mut();
    let mut n_title: u64 = 0 as u64;
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_title = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    if z_title == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return;
    }
    z_first =
        if unsafe { sqlite3_str_length(unsafe { (*p).p_out }) } == 0 {
            c"/".as_ptr() as *mut i8
        } else { c"\n*".as_ptr() as *mut i8 };
    n_title = unsafe { strlen(z_title as *const i8) };
    if n_title >= 75 as u64 {
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s** %z\n\n".as_ptr() as *mut i8 as *const i8, z_first,
                z_title)
        };
    } else {
        let n_extra: i32 = 74 - n_title as i32;
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s** %z %.*c\n\n".as_ptr() as *mut i8 as *const i8, z_first,
                z_title, n_extra, '*' as i32)
        };
    }
}
unsafe extern "C" fn diskused_line(p: *mut DiskUsed, z_desc_1: *const i8,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut z_txt: *mut i8 = core::ptr::null_mut();
    let mut n_desc: u64 = 0 as u64;
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_txt = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    if z_txt == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return;
    }
    n_desc =
        if !(z_desc_1).is_null() {
            unsafe { strlen(z_desc_1) }
        } else { 0 as u64 };
    if n_desc >= 50 as u64 {
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s %z".as_ptr() as *mut i8 as *const i8, z_desc_1, z_txt)
        };
    } else {
        let n_extra: i32 = 50 - n_desc as i32;
        unsafe {
            sqlite3_str_appendf(unsafe { (*p).p_out },
                c"%s%.*c %z".as_ptr() as *mut i8 as *const i8, z_desc_1,
                n_extra, '.' as i32, z_txt)
        };
    }
}
extern "C" fn diskused_percent(p: &DiskUsed, r: f64) -> () {
    let mut z_num: [i8; 100] = [0; 100];
    let mut z_dp: *const i8 = core::ptr::null();
    let mut n_leading_digit: i32 = 0;
    let mut sz: i32 = 0;
    unsafe {
        sqlite3_snprintf((core::mem::size_of::<[i8; 100]>() as u64 - 5 as u64)
                as i32, &raw mut z_num[0 as usize] as *mut i8,
            if r >= 10.0 {
                    c"%.3g".as_ptr() as *mut i8
                } else { c"%.2g".as_ptr() as *mut i8 } as *const i8, r)
    };
    sz =
        unsafe { strlen(&raw mut z_num[0 as usize] as *mut i8 as *const i8) }
            as i32;
    z_dp =
        unsafe {
            strchr(&raw mut z_num[0 as usize] as *mut i8 as *const i8,
                '.' as i32)
        };
    if z_dp == core::ptr::null_mut() {
        unsafe {
            memcpy(unsafe {
                        (&raw mut z_num[0 as usize] as *mut i8).offset(sz as isize)
                    } as *mut (), c".0".as_ptr() as *mut i8 as *const (),
                3 as u64)
        };
        n_leading_digit = sz;
        sz += 2;
    } else {
        n_leading_digit =
            unsafe { z_dp.offset_from(&raw mut z_num[0 as usize] as *mut i8) }
                    as i64 as i32;
    }
    if n_leading_digit < 3 {
        unsafe {
            sqlite3_str_appendchar((*p).p_out, 3 - n_leading_digit,
                ' ' as i32 as i8)
        };
    }
    unsafe {
        sqlite3_str_append((*p).p_out,
            &raw mut z_num[0 as usize] as *mut i8 as *const i8, sz)
    };
    unsafe {
        sqlite3_str_append((*p).p_out,
            c"%\n".as_ptr() as *mut i8 as *const i8, 2)
    };
}
extern "C" fn diskused_subreport(p: *mut DiskUsed, z_title_1: *mut i8,
    z_where_1: *mut i8, pgsz: sqlite3_int64, n_page_1: sqlite3_int64) -> i32 {
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut nentry: sqlite3_int64 = 0 as sqlite3_int64;
    let mut payload: sqlite3_int64 = 0 as sqlite3_int64;
    let mut ovfl_payload: sqlite3_int64 = 0 as sqlite3_int64;
    let mut mx_payload: sqlite3_int64 = 0 as sqlite3_int64;
    let mut ovfl_cnt: sqlite3_int64 = 0 as sqlite3_int64;
    let mut leaf_pages: sqlite3_int64 = 0 as sqlite3_int64;
    let mut int_pages: sqlite3_int64 = 0 as sqlite3_int64;
    let mut ovfl_pages: sqlite3_int64 = 0 as sqlite3_int64;
    let mut leaf_unused: sqlite3_int64 = 0 as sqlite3_int64;
    let mut int_unused: sqlite3_int64 = 0 as sqlite3_int64;
    let mut ovfl_unused: sqlite3_int64 = 0 as sqlite3_int64;
    let mut int_cell: sqlite3_int64 = 0 as sqlite3_int64;
    let mut depth: sqlite3_int64 = 0 as sqlite3_int64;
    let mut cnt: sqlite3_int64 = 0 as sqlite3_int64;
    let mut storage: sqlite3_int64 = 0 as sqlite3_int64;
    let mut total_pages: sqlite3_int64 = 0 as sqlite3_int64;
    let mut total_unused: sqlite3_int64 = 0 as sqlite3_int64;
    let mut total_meta: sqlite3_int64 = 0 as sqlite3_int64;
    let mut rc: i32 = 0;
    if z_title_1 == core::ptr::null_mut() ||
            z_where_1 == core::ptr::null_mut() {
        unsafe { diskused_error(p, core::ptr::null()) };
        return 7;
    }
    p_stmt =
        unsafe {
            diskused_prepare(p,
                c"SELECT\n  sum(if(is_without_rowid OR is_index,nentry,leaf_entries)),\n  sum(payload),\n  sum(ovfl_payload),\n  max(mx_payload),\n  sum(ovfl_cnt),\n  sum(leaf_pages),\n  sum(int_pages),\n  sum(ovfl_pages),\n  sum(leaf_unused),\n  sum(int_unused),\n  sum(ovfl_unused),\n  max(depth),\n  count(*),\n  sum(int_entries)\n FROM temp.%s WHERE %s".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_su }, z_where_1)
        };
    if p_stmt == core::ptr::null_mut() { return 1; }
    rc = unsafe { sqlite3_step(p_stmt) };
    if rc == 100 {
        unsafe {
            diskused_title(p, c"%s".as_ptr() as *mut i8 as *const i8,
                z_title_1)
        };
        nentry = unsafe { sqlite3_column_int64(p_stmt, 0) };
        payload = unsafe { sqlite3_column_int64(p_stmt, 1) };
        ovfl_payload = unsafe { sqlite3_column_int64(p_stmt, 2) };
        mx_payload = unsafe { sqlite3_column_int64(p_stmt, 3) };
        ovfl_cnt = unsafe { sqlite3_column_int64(p_stmt, 4) };
        leaf_pages = unsafe { sqlite3_column_int64(p_stmt, 5) };
        int_pages = unsafe { sqlite3_column_int64(p_stmt, 6) };
        ovfl_pages = unsafe { sqlite3_column_int64(p_stmt, 7) };
        leaf_unused = unsafe { sqlite3_column_int64(p_stmt, 8) };
        int_unused = unsafe { sqlite3_column_int64(p_stmt, 9) };
        ovfl_unused = unsafe { sqlite3_column_int64(p_stmt, 10) };
        depth = unsafe { sqlite3_column_int64(p_stmt, 11) };
        cnt = unsafe { sqlite3_column_int64(p_stmt, 12) };
        int_cell = unsafe { sqlite3_column_int64(p_stmt, 13) };
        rc = 101;
        total_pages = leaf_pages + int_pages + ovfl_pages;
        unsafe {
            diskused_line(p,
                c"Percentage of total database".as_ptr() as *mut i8 as
                    *const i8, c"%.3g%%\n".as_ptr() as *mut i8 as *const i8,
                total_pages as f64 * 100.0 / n_page_1 as f64)
        };
        unsafe {
            diskused_line(p,
                c"Number of entries".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, nentry)
        };
        storage = total_pages * pgsz;
        unsafe {
            diskused_line(p,
                c"Bytes of storage consumed".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, storage)
        };
        unsafe {
            diskused_line(p,
                c"Bytes of payload".as_ptr() as *mut i8 as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, payload)
        };
        diskused_percent(unsafe { &*p },
            payload as f64 * 100.0 / storage as f64);
        if ovfl_cnt > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Bytes of payload in overflow".as_ptr() as *mut i8 as
                        *const i8, c"%-11lld ".as_ptr() as *mut i8 as *const i8,
                    ovfl_payload)
            };
            diskused_percent(unsafe { &*p },
                ovfl_payload as f64 * 100.0 / payload as f64);
        }
        total_unused = leaf_unused + int_unused + ovfl_unused;
        total_meta = storage - payload - total_unused;
        unsafe {
            diskused_line(p,
                c"Bytes of metadata".as_ptr() as *mut i8 as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, total_meta)
        };
        diskused_percent(unsafe { &*p },
            total_meta as f64 * 100.0 / storage as f64);
        if cnt == 1 as i64 {
            unsafe {
                diskused_line(p,
                    c"B-tree depth".as_ptr() as *mut i8 as *const i8,
                    c"%lld\n".as_ptr() as *mut i8 as *const i8, depth)
            };
            if int_cell > 1 as i64 {
                unsafe {
                    diskused_line(p,
                        c"Average fanout".as_ptr() as *mut i8 as *const i8,
                        c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                        (int_cell + int_pages) as f64 / int_pages as f64)
                };
            }
        }
        if nentry > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Average payload per entry".as_ptr() as *mut i8 as
                        *const i8, c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                    payload as f64 / nentry as f64)
            };
            unsafe {
                diskused_line(p,
                    c"Average unused bytes per entry".as_ptr() as *mut i8 as
                        *const i8, c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                    total_unused as f64 / nentry as f64)
            };
            unsafe {
                diskused_line(p,
                    c"Average metadata per entry".as_ptr() as *mut i8 as
                        *const i8, c"%.1f\n".as_ptr() as *mut i8 as *const i8,
                    total_meta as f64 / nentry as f64)
            };
        }
        unsafe {
            diskused_line(p,
                c"Maximum single-entry payload".as_ptr() as *mut i8 as
                    *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                mx_payload)
        };
        if nentry > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Entries that use overflow".as_ptr() as *mut i8 as
                        *const i8, c"%-11lld ".as_ptr() as *mut i8 as *const i8,
                    ovfl_cnt)
            };
            diskused_percent(unsafe { &*p },
                ovfl_cnt as f64 * 100.0 / nentry as f64);
        }
        if int_pages > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Index pages used".as_ptr() as *mut i8 as *const i8,
                    c"%lld\n".as_ptr() as *mut i8 as *const i8, int_pages)
            };
        }
        unsafe {
            diskused_line(p,
                c"Primary pages used".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, leaf_pages)
        };
        if ovfl_cnt != 0 {
            unsafe {
                diskused_line(p,
                    c"Overflow pages used".as_ptr() as *mut i8 as *const i8,
                    c"%lld\n".as_ptr() as *mut i8 as *const i8, ovfl_pages)
            };
        }
        unsafe {
            diskused_line(p,
                c"Total pages used".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, total_pages)
        };
        if int_pages > 0 as i64 {
            unsafe {
                diskused_line(p,
                    c"Unused bytes on index pages".as_ptr() as *mut i8 as
                        *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                    int_unused)
            };
        }
        unsafe {
            diskused_line(p,
                c"Unused bytes on primary pages".as_ptr() as *mut i8 as
                    *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                leaf_unused)
        };
        if ovfl_cnt != 0 {
            unsafe {
                diskused_line(p,
                    c"Unused bytes on overflow pages".as_ptr() as *mut i8 as
                        *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                    ovfl_unused)
            };
        }
        unsafe {
            diskused_line(p,
                c"Unused bytes on all pages".as_ptr() as *mut i8 as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, total_unused)
        };
        diskused_percent(unsafe { &*p },
            total_unused as f64 * 100.0 / storage as f64);
    }
    return diskused_stmt_finish(p, rc, p_stmt);
}
extern "C" fn diskused_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut rc: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut n: i32 = 0;
    let mut ii: sqlite3_int64 = 0 as sqlite3_int64;
    let mut pgsz: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_page: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_page_in_use: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_free_list: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_index: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_wo_rowid: sqlite3_int64 = 0 as sqlite3_int64;
    let mut s: DiskUsed = unsafe { core::mem::zeroed() };
    let mut r: [u64; 2] = [0; 2];
    { let _ = argc; };
    unsafe {
        memset(&raw mut s as *mut (), 0,
            core::mem::size_of::<DiskUsed>() as u64)
    };
    s.db = unsafe { sqlite3_context_db_handle(context) };
    s.context = context;
    s.p_out = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
    if unsafe { sqlite3_str_errcode(s.p_out) } != 0 {
        unsafe { diskused_error(&mut s, core::ptr::null()) };
        return;
    }
    s.z_schema =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if s.z_schema == core::ptr::null() {
        s.z_schema = c"main".as_ptr() as *mut i8 as *const i8;
    } else if unsafe {
                sqlite3_strlike(c"temp".as_ptr() as *mut i8 as *const i8,
                    s.z_schema, 0 as u32)
            } == 0 {
        diskused_reset(&mut s);
        unsafe {
            sqlite3_result_text(context,
                c"cannot analyze \"temp\"".as_ptr() as *mut i8 as *const i8,
                -1, None)
        };
        return;
    }
    ii = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT 1 FROM pragma_database_list WHERE name=%Q COLLATE nocase".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 || ii == 0 as i64 {
        diskused_reset(&mut s);
        unsafe {
            sqlite3_result_text(context,
                c"no such database".as_ptr() as *mut i8 as *const i8, -1,
                None)
        };
        return;
    }
    unsafe {
        sqlite3_randomness(core::mem::size_of::<[u64; 2]>() as i32,
            &raw mut r as *mut ())
    };
    s.z_su =
        unsafe {
            sqlite3_mprintf(c"diskused%016llx%016llx".as_ptr() as *mut i8 as
                    *const i8, r[0 as usize], r[1 as usize])
        };
    if s.z_su == core::ptr::null_mut() {
        unsafe { diskused_error(&mut s, core::ptr::null()) };
        return;
    }
    rc =
        unsafe {
            diskused_sql(&mut s,
                c"CREATE TABLE temp.%s(\n   name text,                -- A table or index\n   tblname text,             -- Table that owns name\n   is_index boolean,         -- TRUE if it is an index\n   is_without_rowid boolean, -- TRUE if WITHOUT ROWID table\n   nentry int,               -- Number of entries in the BTree\n   leaf_entries int,         -- Number of leaf entries\n   depth int,                -- Depth of the b-tree\n   payload int,              -- Total data stored in this table/index\n   ovfl_payload int,         -- Total data stored on overflow pages\n   ovfl_cnt int,             -- Number of entries that use overflow\n   mx_payload int,           -- Maximum payload size\n   int_pages int,            -- Interior pages used\n   leaf_pages int,           -- Leaf pages used\n   ovfl_pages int,           -- Overflow pages used\n   int_unused int,           -- Unused bytes on interior pages\n   leaf_unused int,          -- Unused bytes on primary pages\n   ovfl_unused int,          -- Unused bytes on overflow pages\n   int_entries int           -- Btree cells on internal pages\n);".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if rc != 0 { return; }
    rc =
        unsafe {
            diskused_sql(&mut s,
                c"WITH\n  allidx(idxname) AS (\n    SELECT name FROM \"%w\".sqlite_schema WHERE type=\'index\'\n  ),\n  allobj(allname,tblname,isidx,isworowid) AS (\n    SELECT \'sqlite_schema\',\n           \'sqlite_schema\',\n           0,\n           0\n    UNION ALL\n    SELECT name,\n           tbl_name,\n           type=\'index\',\n           EXISTS(SELECT 1\n                    FROM pragma_index_list(sqlite_schema.name,%Q)\n                   WHERE pragma_index_list.origin=\'pk\'\n                     AND pragma_index_list.name NOT IN allidx)\n      FROM \"%w\".sqlite_schema\n  )\nINSERT INTO temp.%s\n  SELECT\n    allname,\n    tblname,\n    isidx,\n    isworowid,\n    sum(ncell),\n    sum((pagetype=\'leaf\')*ncell),\n    max((length(if(path GLOB \'*+*\',\'\',path))+3)/4),\n    sum(payload),\n    sum((pagetype=\'overflow\')*payload),\n    sum(path GLOB \'*+000000\'),\n    max(mx_payload),\n    sum(pagetype=\'internal\'),\n    sum(pagetype=\'leaf\'),\n    sum(pagetype=\'overflow\'),\n    sum((pagetype=\'internal\')*unused),\n    sum((pagetype=\'leaf\')*unused),\n    sum((pagetype=\'overflow\')*unused),\n    sum(if(pagetype=\'internal\',ncell))\n  FROM allobj CROSS JOIN dbstat(%Q) \n  WHERE dbstat.name=allobj.allname\n  GROUP BY allname;\n".as_ptr()
                        as *mut i8 as *const i8, s.z_schema, s.z_schema, s.z_schema,
                s.z_su, s.z_schema)
        };
    if rc != 0 { return; }
    n_page = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_page,
                c"PRAGMA \"%w\".page_count".as_ptr() as *mut i8 as *const i8,
                s.z_schema)
        };
    if rc != 0 { return; }
    if n_page <= 0 as i64 {
        diskused_reset(&mut s);
        unsafe {
            sqlite3_result_text(context,
                c"empty database".as_ptr() as *mut i8 as *const i8, -1, None)
        };
        return;
    }
    unsafe {
        diskused_title(&mut s,
            c"Database storage utilization report".as_ptr() as *mut i8 as
                *const i8)
    };
    pgsz = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut pgsz,
                c"PRAGMA \"%w\".page_size".as_ptr() as *mut i8 as *const i8,
                s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Page size in bytes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, pgsz)
    };
    unsafe {
        diskused_line(&mut s,
            c"Pages in the database".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, n_page)
    };
    n_page_in_use = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_page_in_use,
                c"SELECT sum(leaf_pages+int_pages+ovfl_pages) FROM temp.%s".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Pages that store data".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, n_page_in_use)
    };
    diskused_percent(&s, n_page_in_use as f64 * 100.0 / n_page as f64);
    n_free_list = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_free_list,
                c"PRAGMA \"%w\".freelist_count".as_ptr() as *mut i8 as
                    *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Pages on the freelist".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, n_free_list)
    };
    diskused_percent(&s, n_free_list as f64 * 100.0 / n_page as f64);
    ii = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"PRAGMA \"%w\".auto_vacuum".as_ptr() as *mut i8 as *const i8,
                s.z_schema)
        };
    if rc != 0 { return; }
    if ii == 0 as i64 || n_page <= 1 as i64 {
        ii = 0 as sqlite3_int64;
    } else {
        let r_ptrs_per_page: f64 = (pgsz / 5 as sqlite3_int64) as f64;
        let r_av_page: f64 = (n_page as f64 - 1.0) / (r_ptrs_per_page + 1.0);
        ii = unsafe { ceil(r_av_page) } as sqlite3_int64;
    }
    unsafe {
        diskused_line(&mut s,
            c"Pages of auto-vacuum overhead".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, ii)
    };
    diskused_percent(&s, ii as f64 * 100.0 / n_page as f64);
    ii = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT count(*)+1 FROM \"%w\".sqlite_schema WHERE type=\'table\'".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Number of tables".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, ii)
    };
    n_wo_rowid = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_wo_rowid,
                c"SELECT count(*) FROM \"%w\".pragma_table_list WHERE wr".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    if n_wo_rowid > 0 as i64 {
        unsafe {
            diskused_line(&mut s,
                c"Number of WITHOUT ROWID tables".as_ptr() as *mut i8 as
                    *const i8, c"%lld\n".as_ptr() as *mut i8 as *const i8,
                n_wo_rowid)
        };
        unsafe {
            diskused_line(&mut s,
                c"Number of rowid tables".as_ptr() as *mut i8 as *const i8,
                c"%lld\n".as_ptr() as *mut i8 as *const i8, ii - n_wo_rowid)
        };
    }
    n_index = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut n_index,
                c"SELECT count(*) FROM \"%w\".sqlite_schema WHERE type=\'index\'".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Number of indexes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, n_index)
    };
    ii = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT count(*) FROM \"%w\".sqlite_schema WHERE name GLOB \'sqlite_autoindex_*\' AND type=\'index\'".as_ptr()
                        as *mut i8 as *const i8, s.z_schema)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Number of defined indexes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, n_index - ii)
    };
    unsafe {
        diskused_line(&mut s,
            c"Number of implied indexes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, ii)
    };
    unsafe {
        diskused_line(&mut s,
            c"Size of the database in bytes".as_ptr() as *mut i8 as *const i8,
            c"%lld\n".as_ptr() as *mut i8 as *const i8, pgsz * n_page)
    };
    ii = 0 as sqlite3_int64;
    rc =
        unsafe {
            diskused_sql_int(&mut s, &mut ii,
                c"SELECT sum(payload) FROM temp.%s WHERE NOT is_index AND name NOT LIKE \'sqlite_schema\'".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if rc != 0 { return; }
    unsafe {
        diskused_line(&mut s,
            c"Bytes of payload".as_ptr() as *mut i8 as *const i8,
            c"%-11lld ".as_ptr() as *mut i8 as *const i8, ii)
    };
    diskused_percent(&s, ii as f64 * 100.0 / (pgsz * n_page) as f64);
    unsafe {
        diskused_title(&mut s,
            c"Page counts for all tables with their indexes".as_ptr() as
                    *mut i8 as *const i8)
    };
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT upper(tblname),\n       sum(int_pages+leaf_pages+ovfl_pages)\n  FROM temp.%s\n WHERE tblname IS NOT NULL\n GROUP BY 1\n ORDER BY 2 DESC, 1;".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        let nn: sqlite3_int64 = unsafe { sqlite3_column_int64(p_stmt, 1) };
        unsafe {
            diskused_line(&mut s,
                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, nn)
        };
        diskused_percent(&s, nn as f64 * 100.0 / n_page as f64);
    }
    if diskused_stmt_finish(&mut s, rc, p_stmt) != 0 { return; }
    unsafe {
        diskused_title(&mut s,
            c"Page counts for all tables and indexes separately".as_ptr() as
                    *mut i8 as *const i8)
    };
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT upper(name),\n       sum(int_pages+leaf_pages+ovfl_pages)\n  FROM temp.%s\n WHERE name IS NOT NULL\n GROUP BY 1\n ORDER BY 2 DESC, 1;".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        let nn: sqlite3_int64 = unsafe { sqlite3_column_int64(p_stmt, 1) };
        unsafe {
            diskused_line(&mut s,
                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8,
                c"%-11lld ".as_ptr() as *mut i8 as *const i8, nn)
        };
        diskused_percent(&s, nn as f64 * 100.0 / n_page as f64);
    }
    if diskused_stmt_finish(&mut s, rc, p_stmt) != 0 { return; }
    rc =
        diskused_subreport(&mut s,
            c"All tables and indexes".as_ptr() as *mut i8,
            c"1".as_ptr() as *mut i8, pgsz, n_page);
    if rc != 0 { return; }
    rc =
        diskused_subreport(&mut s, c"All tables".as_ptr() as *mut i8,
            c"NOT is_index".as_ptr() as *mut i8, pgsz, n_page);
    if rc != 0 { return; }
    if n_wo_rowid > 0 as i64 {
        rc =
            diskused_subreport(&mut s,
                c"All WITHOUT ROWID tables".as_ptr() as *mut i8,
                c"is_without_rowid".as_ptr() as *mut i8, pgsz, n_page);
        if rc != 0 { return; }
        rc =
            diskused_subreport(&mut s,
                c"All rowid tables".as_ptr() as *mut i8,
                c"NOT is_without_rowid AND NOT is_index".as_ptr() as *mut i8,
                pgsz, n_page);
        if rc != 0 { return; }
    }
    rc =
        diskused_subreport(&mut s, c"All indexes".as_ptr() as *mut i8,
            c"is_index".as_ptr() as *mut i8, pgsz, n_page);
    if rc != 0 { return; }
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT upper(tblname), tblname, sum(is_index) FROM temp.%s GROUP BY 1 ORDER BY 1".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        let z_upper: *const i8 =
            unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
        let z_name: *const i8 =
            unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
        let n_sub_index: i32 = unsafe { sqlite3_column_int(p_stmt, 2) };
        if n_sub_index == 0 {
            let z_title: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"Table %s".as_ptr() as *mut i8 as
                            *const i8, z_upper)
                };
            let z_where: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"name=%Q".as_ptr() as *mut i8 as *const i8,
                        z_name)
                };
            rc = diskused_subreport(&mut s, z_title, z_where, pgsz, n_page);
            unsafe { sqlite3_free(z_title as *mut ()) };
            unsafe { sqlite3_free(z_where as *mut ()) };
            if rc != 0 { break; }
        } else {
            let mut p_s2: *mut sqlite3_stmt = core::ptr::null_mut();
            let mut z_title_1: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"Table %s and all its indexes".as_ptr() as
                                *mut i8 as *const i8, z_upper)
                };
            let mut z_where_1: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"tblname=%Q".as_ptr() as *mut i8 as
                            *const i8, z_name)
                };
            rc =
                diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                    n_page);
            unsafe { sqlite3_free(z_title_1 as *mut ()) };
            unsafe { sqlite3_free(z_where_1 as *mut ()) };
            if rc != 0 { break; }
            z_title_1 =
                unsafe {
                    sqlite3_mprintf(c"Table %s w/o any indexes".as_ptr() as
                                *mut i8 as *const i8, z_upper)
                };
            z_where_1 =
                unsafe {
                    sqlite3_mprintf(c"name=%Q".as_ptr() as *mut i8 as *const i8,
                        z_name)
                };
            rc =
                diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                    n_page);
            unsafe { sqlite3_free(z_title_1 as *mut ()) };
            unsafe { sqlite3_free(z_where_1 as *mut ()) };
            if rc != 0 { break; }
            if n_sub_index > 1 {
                z_title_1 =
                    unsafe {
                        sqlite3_mprintf(c"All indexes of table %s".as_ptr() as
                                    *mut i8 as *const i8, z_upper)
                    };
                z_where_1 =
                    unsafe {
                        sqlite3_mprintf(c"tblname=%Q AND is_index".as_ptr() as
                                    *mut i8 as *const i8, z_name)
                    };
                rc =
                    diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                        n_page);
                unsafe { sqlite3_free(z_title_1 as *mut ()) };
                unsafe { sqlite3_free(z_where_1 as *mut ()) };
                if rc != 0 { break; }
            }
            p_s2 =
                unsafe {
                    diskused_prepare(&mut s,
                        c"SELECT name, upper(name) FROM temp.%s WHERE is_index AND tblname=%Q".as_ptr()
                                as *mut i8 as *const i8, s.z_su, z_name)
                };
            if p_s2 == core::ptr::null_mut() { rc = 7; break; }
            while { rc = unsafe { sqlite3_step(p_s2) }; rc } == 100 {
                let z_u: *const i8 =
                    unsafe { sqlite3_column_text(p_s2, 1) } as *const i8;
                let z_n: *const i8 =
                    unsafe { sqlite3_column_text(p_s2, 0) } as *const i8;
                z_title_1 =
                    unsafe {
                        sqlite3_mprintf(c"Index %s".as_ptr() as *mut i8 as
                                *const i8, z_u)
                    };
                z_where_1 =
                    unsafe {
                        sqlite3_mprintf(c"name=%Q".as_ptr() as *mut i8 as *const i8,
                            z_n)
                    };
                rc =
                    diskused_subreport(&mut s, z_title_1, z_where_1, pgsz,
                        n_page);
                unsafe { sqlite3_free(z_title_1 as *mut ()) };
                unsafe { sqlite3_free(z_where_1 as *mut ()) };
                if rc != 0 { break; }
            }
            rc = diskused_stmt_finish(&mut s, rc, p_s2);
            if rc != 0 { break; }
        }
    }
    if diskused_stmt_finish(&mut s, rc, p_stmt) != 0 { return; }
    unsafe {
        diskused_title(&mut s,
            c"Raw data used to generate this report".as_ptr() as *mut i8 as
                *const i8)
    };
    unsafe {
        sqlite3_str_appendf(s.p_out,
            c"The following SQL will create a table named \"space_used\" which\ncontains most of the information used to generate the report above.\n*/\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe {
        sqlite3_str_appendf(s.p_out,
            c"BEGIN;\nCREATE TABLE space_used(\n   name text,                -- A table or index\n   tblname text,             -- Table that owns name\n   is_index boolean,         -- TRUE if it is an index\n   is_without_rowid boolean, -- TRUE if WITHOUT ROWID table\n   nentry int,               -- Number of entries in the BTree\n   leaf_entries int,         -- Number of leaf entries\n   depth int,                -- Depth of the b-tree\n   payload int,              -- Total data in this table/index\n   ovfl_payload int,         -- Total data on overflow pages\n   ovfl_cnt int,             -- Entries that use overflow\n   mx_payload int,           -- Maximum payload size\n   int_pages int,            -- Interior pages used\n   leaf_pages int,           -- Leaf pages used\n   ovfl_pages int,           -- Overflow pages used\n   int_unused int,           -- Unused bytes on interior pages\n   leaf_unused int,          -- Unused bytes on primary pages\n   ovfl_unused int,          -- Unused bytes on overflow pages\n   int_entries int           -- B-tree entries on internal pages\n);\nINSERT INTO space_used VALUES\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    p_stmt =
        unsafe {
            diskused_prepare(&mut s,
                c"SELECT quote(name), quote(tblname),\n       is_index, is_without_rowid, nentry, leaf_entries,\n       depth, payload, ovfl_payload, ovfl_cnt, mx_payload,\n       int_pages, leaf_pages, ovfl_pages, int_unused,\n       leaf_unused, ovfl_unused, int_entries\n  FROM temp.%s;".as_ptr()
                        as *mut i8 as *const i8, s.z_su)
        };
    if p_stmt == core::ptr::null_mut() { return; }
    n = 0;
    while { rc = unsafe { sqlite3_step(p_stmt) }; rc } == 100 {
        if { let __p = &mut n; let __t = *__p; *__p += 1; __t } != 0 {
            unsafe {
                sqlite3_str_appendf(s.p_out,
                    c",\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            sqlite3_str_appendf(s.p_out,
                c" (%s,%s,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld)".as_ptr()
                        as *mut i8 as *const i8,
                unsafe { sqlite3_column_text(p_stmt, 0) },
                unsafe { sqlite3_column_text(p_stmt, 1) },
                unsafe { sqlite3_column_int64(p_stmt, 2) },
                unsafe { sqlite3_column_int64(p_stmt, 3) },
                unsafe { sqlite3_column_int64(p_stmt, 4) },
                unsafe { sqlite3_column_int64(p_stmt, 5) },
                unsafe { sqlite3_column_int64(p_stmt, 6) },
                unsafe { sqlite3_column_int64(p_stmt, 7) },
                unsafe { sqlite3_column_int64(p_stmt, 8) },
                unsafe { sqlite3_column_int64(p_stmt, 9) },
                unsafe { sqlite3_column_int64(p_stmt, 10) },
                unsafe { sqlite3_column_int64(p_stmt, 11) },
                unsafe { sqlite3_column_int64(p_stmt, 12) },
                unsafe { sqlite3_column_int64(p_stmt, 13) },
                unsafe { sqlite3_column_int64(p_stmt, 14) },
                unsafe { sqlite3_column_int64(p_stmt, 15) },
                unsafe { sqlite3_column_int64(p_stmt, 16) },
                unsafe { sqlite3_column_int64(p_stmt, 17) })
        };
    }
    if rc != 101 {
        unsafe {
            diskused_error(&mut s,
                c"SQL run-time error: %s\nSQL: %s".as_ptr() as *mut i8 as
                    *const i8, unsafe { sqlite3_errmsg(s.db) },
                unsafe { sqlite3_sql(p_stmt) })
        };
        unsafe { sqlite3_finalize(p_stmt) };
        return;
    }
    unsafe {
        sqlite3_str_appendf(s.p_out,
            c";\nCOMMIT;".as_ptr() as *mut i8 as *const i8)
    };
    unsafe { sqlite3_finalize(p_stmt) };
    if unsafe { sqlite3_str_length(s.p_out) } != 0 {
        unsafe {
            sqlite3_result_text(context,
                unsafe { sqlite3_str_finish(s.p_out) } as *const i8, -1,
                Some(sqlite3_free))
        };
        s.p_out = core::ptr::null_mut();
    }
    diskused_reset(&mut s);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_diskused_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"diskused".as_ptr() as *mut i8 as *const i8, 1, 1 | 524288,
                core::ptr::null_mut(), Some(diskused_func), None, None)
        };
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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn ceil(_: f64)
    -> f64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}