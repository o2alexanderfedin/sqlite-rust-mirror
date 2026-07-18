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
struct UnionCsr {
    base: sqlite3_vtab_cursor,
    p_stmt: *mut sqlite3_stmt,
    i_max_rowid: sqlite3_int64,
    i_tab: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct UnionTab {
    base: sqlite3_vtab,
    db: *mut sqlite3,
    b_swarm: i32,
    i_pk: i32,
    n_src: i32,
    a_src: *mut UnionSrc,
    b_has_context: i32,
    z_source_str: *mut i8,
    p_not_found: *mut sqlite3_stmt,
    p_open_close: *mut sqlite3_stmt,
    p_closable: *mut UnionSrc,
    n_open: i32,
    n_max_open: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct UnionSrc {
    z_db: *mut i8,
    z_tab: *mut i8,
    i_min: sqlite3_int64,
    i_max: sqlite3_int64,
    z_file: *mut i8,
    z_context: *mut i8,
    n_user: i32,
    db: *mut sqlite3,
    p_next_closable: *mut UnionSrc,
}
extern "C" fn union_malloc(p_rc_1: &mut i32, n_byte_1: sqlite3_int64)
    -> *mut () {
    let mut p_ret: *mut () = core::ptr::null_mut();
    if !(n_byte_1 > 0 as i64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"unionMalloc".as_ptr() as *const i8,
                c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 255,
                c"nByte>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if *p_rc_1 == 0 {
        p_ret = unsafe { sqlite3_malloc64(n_byte_1 as sqlite3_uint64) };
        if !(p_ret).is_null() {
            unsafe { memset(p_ret, 0, n_byte_1 as u64) };
        } else { *p_rc_1 = 7; }
    } else { p_ret = core::ptr::null_mut(); }
    return p_ret;
}
extern "C" fn union_strdup(p_rc_1: *mut i32, z_in_1: *const i8) -> *mut i8 {
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    if !(z_in_1).is_null() {
        let n_byte: sqlite3_int64 =
            (unsafe { strlen(z_in_1) } + 1 as u64) as sqlite3_int64;
        z_ret = union_malloc(unsafe { &mut *p_rc_1 }, n_byte) as *mut i8;
        if !(z_ret).is_null() {
            unsafe {
                memcpy(z_ret as *mut (), z_in_1 as *const (), n_byte as u64)
            };
        }
    }
    return z_ret;
}
extern "C" fn union_dequote(z: *mut i8) -> () {
    if !(z).is_null() {
        let mut q: i8 = unsafe { *z.offset(0 as isize) };
        if q as i32 == '[' as i32 || q as i32 == '\'' as i32 ||
                    q as i32 == '\"' as i32 || q as i32 == '`' as i32 {
            let mut i_in: i32 = 1;
            let mut i_out: i32 = 0;
            if q as i32 == '[' as i32 { q = ']' as i32 as i8; }
            while if unsafe { *z.offset(i_in as isize) } != 0 {
                        1
                    } else {
                        {
                            if (0 == 0) as i32 as i64 != 0 {
                                unsafe {
                                    __assert_rtn(c"unionDequote".as_ptr() as *const i8,
                                        c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 306,
                                        c"0".as_ptr() as *mut i8 as *const i8)
                                }
                            } else { { let _ = 0; } };
                            0
                        }
                    } != 0 {
                if unsafe { *z.offset(i_in as isize) } as i32 == q as i32 {
                    if unsafe { *z.offset((i_in + 1) as isize) } as i32 !=
                            q as i32 {
                        { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                        break;
                    } else {
                        i_in += 2;
                        unsafe {
                            *z.offset({
                                                let __p = &mut i_out;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = q
                        };
                    }
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
}
extern "C" fn union_prepare(p_rc_1: &mut i32, db: *mut sqlite3,
    z_sql_1: *const i8, pz_err_1: *mut *mut i8) -> *mut sqlite3_stmt {
    let mut p_ret: *mut sqlite3_stmt = core::ptr::null_mut();
    if (pz_err_1).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"unionPrepare".as_ptr() as *const i8,
                c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 348,
                c"pzErr".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if *p_rc_1 == 0 {
        let rc: i32 =
            unsafe {
                sqlite3_prepare_v2(db, z_sql_1, -1, &mut p_ret,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"sql error: %s".as_ptr() as *mut i8 as
                                *const i8, unsafe { sqlite3_errmsg(db) })
                    }
            };
            *p_rc_1 = rc;
        }
    }
    return p_ret;
}
unsafe extern "C" fn union_prepare_printf(p_rc_1: *mut i32,
    pz_err_1: *mut *mut i8, db: *mut sqlite3, z_fmt_1: *const i8,
    mut __va0: ...) -> *mut sqlite3_stmt {
    let mut p_ret: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    if unsafe { *p_rc_1 } == 0 {
        if z_sql == core::ptr::null_mut() {
            unsafe { *p_rc_1 = 7 };
        } else {
            p_ret =
                union_prepare(unsafe { &mut *p_rc_1 }, db, z_sql as *const i8,
                    pz_err_1);
        }
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    ();
    return p_ret;
}
extern "C" fn union_finalize(p_rc_1: &mut i32, p_stmt_1: *mut sqlite3_stmt,
    pz_err_1: &mut *mut i8) -> () {
    let db: *mut sqlite3 = unsafe { sqlite3_db_handle(p_stmt_1) };
    let rc: i32 = unsafe { sqlite3_finalize(p_stmt_1) };
    if *p_rc_1 == 0 {
        *p_rc_1 = rc;
        if rc != 0 {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_errmsg(db) })
                };
        }
    }
}
extern "C" fn union_invoke_open_close(p_tab_1: &UnionTab, p_src_1: &UnionSrc,
    b_close_1: i32, pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    if !((*p_tab_1).p_open_close).is_null() {
        unsafe {
            sqlite3_bind_text((*p_tab_1).p_open_close, 1,
                (*p_src_1).z_file as *const i8, -1, None)
        };
        if (*p_tab_1).b_has_context != 0 {
            unsafe {
                sqlite3_bind_text((*p_tab_1).p_open_close, 2,
                    (*p_src_1).z_context as *const i8, -1, None)
            };
        }
        unsafe {
            sqlite3_bind_int((*p_tab_1).p_open_close,
                2 + (*p_tab_1).b_has_context, b_close_1)
        };
        unsafe { sqlite3_step((*p_tab_1).p_open_close) };
        if 0 != { rc = unsafe { sqlite3_reset((*p_tab_1).p_open_close) }; rc }
            {
            if !(pz_err_1).is_null() {
                unsafe {
                    *pz_err_1 =
                        unsafe {
                            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_errmsg((*p_tab_1).db) })
                        }
                };
            }
        }
    }
    return rc;
}
extern "C" fn union_close_sources(p_tab_1: *mut UnionTab, n_max_1: i32)
    -> () {
    while !(unsafe { (*p_tab_1).p_closable }).is_null() &&
            unsafe { (*p_tab_1).n_open } > n_max_1 {
        let mut p: *mut UnionSrc = core::ptr::null_mut();
        let mut pp: *mut *mut UnionSrc = core::ptr::null_mut();
        {
            pp = unsafe { &mut (*p_tab_1).p_closable };
            '__b2: loop {
                if !(!(unsafe {
                                            (*unsafe { *pp }).p_next_closable
                                        }).is_null()) {
                    break '__b2;
                }
                '__c2: loop { break '__c2; }
                pp = unsafe { &mut (*unsafe { *pp }).p_next_closable };
            }
        }
        p = unsafe { *pp };
        if (unsafe { (*p).db }).is_null() as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"unionCloseSources".as_ptr() as *const i8,
                    c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 469,
                    c"p->db".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { sqlite3_close(unsafe { (*p).db }) };
        unsafe { (*p).db = core::ptr::null_mut() };
        unsafe { *pp = core::ptr::null_mut() };
        {
            let __p = unsafe { &mut (*p_tab_1).n_open };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        union_invoke_open_close(unsafe { &*p_tab_1 }, unsafe { &*p }, 1,
            core::ptr::null_mut());
    }
}
extern "C" fn union_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    if !(p_vtab_1).is_null() {
        let p_tab: *mut UnionTab = p_vtab_1 as *mut UnionTab;
        let mut i: i32 = 0;
        {
            i = 0;
            '__b3: loop {
                if !(i < unsafe { (*p_tab).n_src }) { break '__b3; }
                '__c3: loop {
                    let p_src: *mut UnionSrc =
                        unsafe {
                            &mut *unsafe { (*p_tab).a_src.offset(i as isize) }
                        };
                    let b_have_src_db: i32 =
                        (unsafe { (*p_src).db } != core::ptr::null_mut()) as i32;
                    unsafe { sqlite3_close(unsafe { (*p_src).db }) };
                    if b_have_src_db != 0 {
                        union_invoke_open_close(unsafe { &*p_tab },
                            unsafe { &*p_src }, 1, core::ptr::null_mut());
                    }
                    unsafe {
                        sqlite3_free(unsafe { (*p_src).z_db } as *mut ())
                    };
                    unsafe {
                        sqlite3_free(unsafe { (*p_src).z_tab } as *mut ())
                    };
                    unsafe {
                        sqlite3_free(unsafe { (*p_src).z_file } as *mut ())
                    };
                    unsafe {
                        sqlite3_free(unsafe { (*p_src).z_context } as *mut ())
                    };
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_finalize(unsafe { (*p_tab).p_not_found }) };
        unsafe { sqlite3_finalize(unsafe { (*p_tab).p_open_close }) };
        unsafe { sqlite3_free(unsafe { (*p_tab).z_source_str } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p_tab).a_src } as *mut ()) };
        unsafe { sqlite3_free(p_tab as *mut ()) };
    }
    return 0;
}
extern "C" fn union_is_intkey_table(db: *mut sqlite3, p_src_1: &UnionSrc,
    pz_err_1: &mut *mut i8) -> i32 {
    let mut b_pk: i32 = 0;
    let mut z_type: *const i8 = core::ptr::null();
    let mut rc: i32 = 0;
    unsafe {
        sqlite3_table_column_metadata(db, (*p_src_1).z_db as *const i8,
            (*p_src_1).z_tab as *const i8,
            c"_rowid_".as_ptr() as *mut i8 as *const i8, &mut z_type,
            core::ptr::null_mut(), core::ptr::null_mut(), &mut b_pk,
            core::ptr::null_mut())
    };
    rc = unsafe { sqlite3_errcode(db) };
    if rc == 1 ||
            rc == 0 &&
                ((b_pk == 0) as i32 != 0 ||
                    unsafe {
                            sqlite3_stricmp(c"integer".as_ptr() as *mut i8 as *const i8,
                                z_type)
                        } != 0) {
        rc = 1;
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"no such rowid table: %s%s%s".as_ptr() as
                            *mut i8 as *const i8,
                    if !((*p_src_1).z_db).is_null() {
                        (*p_src_1).z_db
                    } else { c"".as_ptr() as *mut i8 },
                    if !((*p_src_1).z_db).is_null() {
                        c".".as_ptr() as *mut i8
                    } else { c"".as_ptr() as *mut i8 }, (*p_src_1).z_tab)
            };
    }
    return rc;
}
extern "C" fn union_source_to_str(p_rc_1: &mut i32, p_tab_1: &UnionTab,
    p_src_1: *mut UnionSrc, pz_err_1: *mut *mut i8) -> *mut i8 {
    let mut z_ret: *mut i8 = core::ptr::null_mut();
    if *p_rc_1 == 0 {
        let db: *mut sqlite3 =
            if (*p_tab_1).b_swarm != 0 {
                unsafe { (*p_src_1).db }
            } else { (*p_tab_1).db };
        let mut rc: i32 =
            union_is_intkey_table(db, unsafe { &*p_src_1 },
                unsafe { &mut *pz_err_1 });
        let p_stmt: *mut sqlite3_stmt =
            union_prepare(&mut rc, db,
                c"SELECT group_concat(quote(name) || \'.\' || quote(type)) FROM pragma_table_info(?, ?)".as_ptr()
                        as *mut i8 as *const i8, pz_err_1);
        if rc == 0 {
            unsafe {
                sqlite3_bind_text(p_stmt, 1,
                    unsafe { (*p_src_1).z_tab } as *const i8, -1, None)
            };
            unsafe {
                sqlite3_bind_text(p_stmt, 2,
                    unsafe { (*p_src_1).z_db } as *const i8, -1, None)
            };
            if 100 == unsafe { sqlite3_step(p_stmt) } {
                let z: *const i8 =
                    unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
                z_ret = union_strdup(&mut rc, z);
            }
            union_finalize(&mut rc, p_stmt, unsafe { &mut *pz_err_1 });
        }
        *p_rc_1 = rc;
    }
    return z_ret;
}
extern "C" fn union_source_check(p_tab_1: *mut UnionTab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut z0: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    if !(unsafe { *pz_err_1 } == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"unionSourceCheck".as_ptr() as *const i8,
                c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 597,
                c"*pzErr==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    z0 =
        union_source_to_str(&mut rc, unsafe { &*p_tab_1 },
            unsafe { &mut *unsafe { (*p_tab_1).a_src.offset(0 as isize) } },
            pz_err_1);
    {
        i = 1;
        '__b4: loop {
            if !(i < unsafe { (*p_tab_1).n_src }) { break '__b4; }
            '__c4: loop {
                let z: *mut i8 =
                    union_source_to_str(&mut rc, unsafe { &*p_tab_1 },
                        unsafe {
                            &mut *unsafe { (*p_tab_1).a_src.offset(i as isize) }
                        }, pz_err_1);
                if rc == 0 &&
                        unsafe { sqlite3_stricmp(z as *const i8, z0 as *const i8) }
                            != 0 {
                    unsafe {
                        *pz_err_1 =
                            unsafe {
                                sqlite3_mprintf(c"source table schema mismatch".as_ptr() as
                                            *mut i8 as *const i8)
                            }
                    };
                    rc = 1;
                }
                unsafe { sqlite3_free(z as *mut ()) };
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(z0 as *mut ()) };
    return rc;
}
extern "C" fn union_open_database_inner(p_tab_1: *mut UnionTab,
    p_src_1: *mut UnionSrc, pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    rc =
        union_invoke_open_close(unsafe { &*p_tab_1 }, unsafe { &*p_src_1 }, 0,
            pz_err_1);
    if rc != 0 { return rc; }
    rc =
        unsafe {
            sqlite3_open_v2(unsafe { (*p_src_1).z_file } as *const i8,
                unsafe { &mut (*p_src_1).db }, open_flags, core::ptr::null())
        };
    if rc == 0 { return rc; }
    if !(unsafe { (*p_tab_1).p_not_found }).is_null() {
        unsafe { sqlite3_close(unsafe { (*p_src_1).db }) };
        unsafe { (*p_src_1).db = core::ptr::null_mut() };
        unsafe {
            sqlite3_bind_text(unsafe { (*p_tab_1).p_not_found }, 1,
                unsafe { (*p_src_1).z_file } as *const i8, -1, None)
        };
        if unsafe { (*p_tab_1).b_has_context } != 0 {
            unsafe {
                sqlite3_bind_text(unsafe { (*p_tab_1).p_not_found }, 2,
                    unsafe { (*p_src_1).z_context } as *const i8, -1, None)
            };
        }
        unsafe { sqlite3_step(unsafe { (*p_tab_1).p_not_found }) };
        if 0 !=
                {
                    rc =
                        unsafe { sqlite3_reset(unsafe { (*p_tab_1).p_not_found }) };
                    rc
                } {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(unsafe { (*p_tab_1).db }) })
                    }
            };
            return rc;
        }
        rc =
            unsafe {
                sqlite3_open_v2(unsafe { (*p_src_1).z_file } as *const i8,
                    unsafe { &mut (*p_src_1).db }, open_flags,
                    core::ptr::null())
            };
    }
    if rc != 0 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_errmsg(unsafe { (*p_src_1).db }) })
                }
        };
    }
    return rc;
}
extern "C" fn union_open_database(p_tab_1: *mut UnionTab, i_src_1: i32,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let p_src: *mut UnionSrc =
        unsafe { &mut *unsafe { (*p_tab_1).a_src.offset(i_src_1 as isize) } };
    if !(unsafe { (*p_tab_1).b_swarm } != 0 &&
                            i_src_1 < unsafe { (*p_tab_1).n_src }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"unionOpenDatabase".as_ptr() as *const i8,
                c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 664,
                c"pTab->bSwarm && iSrc<pTab->nSrc".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_src).db } == core::ptr::null_mut() {
        union_close_sources(p_tab_1, unsafe { (*p_tab_1).n_max_open } - 1);
        rc = union_open_database_inner(p_tab_1, p_src, pz_err_1);
        if rc == 0 {
            let z: *mut i8 =
                union_source_to_str(&mut rc, unsafe { &*p_tab_1 }, p_src,
                    pz_err_1);
            if rc == 0 {
                if unsafe { (*p_tab_1).z_source_str } == core::ptr::null_mut()
                    {
                    unsafe { (*p_tab_1).z_source_str = z };
                } else {
                    if unsafe {
                                sqlite3_stricmp(z as *const i8,
                                    unsafe { (*p_tab_1).z_source_str } as *const i8)
                            } != 0 {
                        unsafe {
                            *pz_err_1 =
                                unsafe {
                                    sqlite3_mprintf(c"source table schema mismatch".as_ptr() as
                                                *mut i8 as *const i8)
                                }
                        };
                        rc = 1;
                    }
                    unsafe { sqlite3_free(z as *mut ()) };
                }
            }
        }
        if rc == 0 {
            unsafe {
                (*p_src).p_next_closable = unsafe { (*p_tab_1).p_closable }
            };
            unsafe { (*p_tab_1).p_closable = p_src };
            {
                let __p = unsafe { &mut (*p_tab_1).n_open };
                let __t = *__p;
                *__p += 1;
                __t
            };
        } else {
            unsafe { sqlite3_close(unsafe { (*p_src).db }) };
            unsafe { (*p_src).db = core::ptr::null_mut() };
            union_invoke_open_close(unsafe { &*p_tab_1 }, unsafe { &*p_src },
                1, core::ptr::null_mut());
        }
    }
    return rc;
}
extern "C" fn union_incr_refcount(p_tab_1: &mut UnionTab, i_tab_1: i32)
    -> () {
    if (*p_tab_1).b_swarm != 0 {
        let p_src: *mut UnionSrc =
            unsafe { &mut *(*p_tab_1).a_src.offset(i_tab_1 as isize) };
        if !(unsafe { (*p_src).n_user } >= 0 &&
                                !(unsafe { (*p_src).db }).is_null()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"unionIncrRefcount".as_ptr() as *const i8,
                    c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 707,
                    c"pSrc->nUser>=0 && pSrc->db".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if unsafe { (*p_src).n_user } == 0 {
            let mut pp: *mut *mut UnionSrc = core::ptr::null_mut();
            {
                pp = &mut (*p_tab_1).p_closable;
                '__b5: loop {
                    if !(unsafe { *pp } != p_src) { break '__b5; }
                    '__c5: loop { break '__c5; }
                    pp = unsafe { &mut (*unsafe { *pp }).p_next_closable };
                }
            }
            unsafe { *pp = unsafe { (*p_src).p_next_closable } };
            unsafe { (*p_src).p_next_closable = core::ptr::null_mut() };
        }
        {
            let __p = unsafe { &mut (*p_src).n_user };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}
extern "C" fn union_finalize_csr_stmt(p_csr_1: &mut UnionCsr) -> i32 {
    let mut rc: i32 = 0;
    if !((*p_csr_1).p_stmt).is_null() {
        let p_tab: *mut UnionTab = (*p_csr_1).base.p_vtab as *mut UnionTab;
        let p_src: *mut UnionSrc =
            unsafe {
                &mut *unsafe {
                            (*p_tab).a_src.offset((*p_csr_1).i_tab as isize)
                        }
            };
        rc = unsafe { sqlite3_finalize((*p_csr_1).p_stmt) };
        (*p_csr_1).p_stmt = core::ptr::null_mut();
        if unsafe { (*p_tab).b_swarm } != 0 {
            {
                let __p = unsafe { &mut (*p_src).n_user };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if !(unsafe { (*p_src).n_user } >= 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"unionFinalizeCsrStmt".as_ptr() as *const i8,
                        c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 735,
                        c"pSrc->nUser>=0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if unsafe { (*p_src).n_user } == 0 {
                unsafe {
                    (*p_src).p_next_closable = unsafe { (*p_tab).p_closable }
                };
                unsafe { (*p_tab).p_closable = p_src };
            }
            union_close_sources(p_tab, unsafe { (*p_tab).n_max_open });
        }
    }
    return rc;
}
extern "C" fn union_isspace(c: i8) -> i32 {
    return (c as i32 == ' ' as i32 || c as i32 == '\n' as i32 ||
                    c as i32 == '\r' as i32 || c as i32 == '\t' as i32) as i32;
}
extern "C" fn union_isidchar(c: i8) -> i32 {
    return (c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32 ||
                    c as i32 >= 'A' as i32 && (c as i32) < 'Z' as i32 ||
                c as i32 >= '0' as i32 && c as i32 <= '9' as i32) as i32;
}
extern "C" fn union_configure_vtab(p_rc_1: &mut i32, p_tab_1: &mut UnionTab,
    p_stmt_1: *mut sqlite3_stmt, az_arg_1: &[*const i8],
    pz_err_1: *mut *mut i8) -> () {
    let mut rc: i32 = *p_rc_1;
    let mut i: i32 = 0;
    if rc == 0 {
        (*p_tab_1).b_has_context =
            (unsafe { sqlite3_column_count(p_stmt_1) } > 4) as i32;
    }
    {
        i = 0;
        '__b6: loop {
            if !(rc == 0 && i < az_arg_1.len() as i32) { break '__b6; }
            '__c6: loop {
                let z_arg: *mut i8 =
                    union_strdup(&mut rc, az_arg_1[i as usize]);
                if !(z_arg).is_null() {
                    let mut n_opt: i32 = 0;
                    let mut z_opt: *mut i8 = core::ptr::null_mut();
                    let mut z_val: *mut i8 = core::ptr::null_mut();
                    union_dequote(z_arg);
                    z_opt = z_arg;
                    while union_isspace(unsafe { *z_opt }) != 0 {
                        {
                            let __p = &mut z_opt;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    z_val = z_opt;
                    if unsafe { *z_val } as i32 == ':' as i32 {
                        {
                            let __p = &mut z_val;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    while union_isidchar(unsafe { *z_val }) != 0 {
                        {
                            let __p = &mut z_val;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    n_opt = unsafe { z_val.offset_from(z_opt) } as i64 as i32;
                    while union_isspace(unsafe { *z_val }) != 0 {
                        {
                            let __p = &mut z_val;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe { *z_val } as i32 == '=' as i32 {
                        unsafe {
                            *z_opt.offset(n_opt as isize) = '\u{0}' as i32 as i8
                        };
                        {
                            let __p = &mut z_val;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        while union_isspace(unsafe { *z_val }) != 0 {
                            {
                                let __p = &mut z_val;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        z_val = union_strdup(&mut rc, z_val as *const i8);
                        if !(z_val).is_null() {
                            union_dequote(z_val);
                            if unsafe { *z_opt.offset(0 as isize) } as i32 == ':' as i32
                                {
                                let i_param: i32 =
                                    unsafe {
                                        sqlite3_bind_parameter_index(p_stmt_1, z_opt as *const i8)
                                    };
                                if i_param == 0 {
                                    unsafe {
                                        *pz_err_1 =
                                            unsafe {
                                                sqlite3_mprintf(c"swarmvtab: no such SQL parameter: %s".as_ptr()
                                                            as *mut i8 as *const i8, z_opt)
                                            }
                                    };
                                    rc = 1;
                                } else {
                                    rc =
                                        unsafe {
                                            sqlite3_bind_text(p_stmt_1, i_param, z_val as *const i8, -1,
                                                Some(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn(*mut ())
                                                                    -> ()>(-1 as isize as *const ())
                                                    }))
                                        };
                                }
                            } else if n_opt == 7 &&
                                    0 ==
                                        unsafe {
                                            sqlite3_strnicmp(z_opt as *const i8,
                                                c"maxopen".as_ptr() as *mut i8 as *const i8, 7)
                                        } {
                                (*p_tab_1).n_max_open = unsafe { atoi(z_val as *const i8) };
                                if (*p_tab_1).n_max_open <= 0 {
                                    unsafe {
                                        *pz_err_1 =
                                            unsafe {
                                                sqlite3_mprintf(c"swarmvtab: illegal maxopen value".as_ptr()
                                                            as *mut i8 as *const i8)
                                            }
                                    };
                                    rc = 1;
                                }
                            } else if n_opt == 7 &&
                                    0 ==
                                        unsafe {
                                            sqlite3_strnicmp(z_opt as *const i8,
                                                c"missing".as_ptr() as *mut i8 as *const i8, 7)
                                        } {
                                if !((*p_tab_1).p_not_found).is_null() {
                                    unsafe {
                                        *pz_err_1 =
                                            unsafe {
                                                sqlite3_mprintf(c"swarmvtab: duplicate \"missing\" option".as_ptr()
                                                            as *mut i8 as *const i8)
                                            }
                                    };
                                    rc = 1;
                                } else {
                                    (*p_tab_1).p_not_found =
                                        unsafe {
                                            union_prepare_printf(&mut rc, pz_err_1, (*p_tab_1).db,
                                                c"SELECT \"%w\"(?%s)".as_ptr() as *mut i8 as *const i8,
                                                z_val,
                                                if (*p_tab_1).b_has_context != 0 {
                                                    c",?".as_ptr() as *mut i8
                                                } else { c"".as_ptr() as *mut i8 })
                                        };
                                }
                            } else if n_opt == 9 &&
                                    0 ==
                                        unsafe {
                                            sqlite3_strnicmp(z_opt as *const i8,
                                                c"openclose".as_ptr() as *mut i8 as *const i8, 9)
                                        } {
                                if !((*p_tab_1).p_open_close).is_null() {
                                    unsafe {
                                        *pz_err_1 =
                                            unsafe {
                                                sqlite3_mprintf(c"swarmvtab: duplicate \"openclose\" option".as_ptr()
                                                            as *mut i8 as *const i8)
                                            }
                                    };
                                    rc = 1;
                                } else {
                                    (*p_tab_1).p_open_close =
                                        unsafe {
                                            union_prepare_printf(&mut rc, pz_err_1, (*p_tab_1).db,
                                                c"SELECT \"%w\"(?,?%s)".as_ptr() as *mut i8 as *const i8,
                                                z_val,
                                                if (*p_tab_1).b_has_context != 0 {
                                                    c",?".as_ptr() as *mut i8
                                                } else { c"".as_ptr() as *mut i8 })
                                        };
                                }
                            } else {
                                unsafe {
                                    *pz_err_1 =
                                        unsafe {
                                            sqlite3_mprintf(c"swarmvtab: unrecognized option: %s".as_ptr()
                                                        as *mut i8 as *const i8, z_opt)
                                        }
                                };
                                rc = 1;
                            }
                            unsafe { sqlite3_free(z_val as *mut ()) };
                        }
                    } else {
                        if i == 0 && az_arg_1.len() as i32 == 1 {
                            (*p_tab_1).p_not_found =
                                unsafe {
                                    union_prepare_printf(&mut rc, pz_err_1, (*p_tab_1).db,
                                        c"SELECT \"%w\"(?)".as_ptr() as *mut i8 as *const i8, z_arg)
                                };
                        } else {
                            unsafe {
                                *pz_err_1 =
                                    unsafe {
                                        sqlite3_mprintf(c"swarmvtab: parse error: %s".as_ptr() as
                                                    *mut i8 as *const i8, az_arg_1[i as usize])
                                    }
                            };
                            rc = 1;
                        }
                    }
                    unsafe { sqlite3_free(z_arg as *mut ()) };
                }
                break '__c6;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    *p_rc_1 = rc;
}
extern "C" fn union_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_tab: *mut UnionTab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let b_swarm: i32 = if p_aux_1 == core::ptr::null_mut() { 0 } else { 1 };
    let z_vtab: *const i8 =
        if b_swarm != 0 {
                c"swarmvtab".as_ptr() as *mut i8
            } else { c"unionvtab".as_ptr() as *mut i8 } as *const i8;
    if unsafe {
                sqlite3_stricmp(c"temp".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) })
            } != 0 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"%s tables must be created in TEMP schema".as_ptr()
                                as *mut i8 as *const i8, z_vtab)
                }
        };
        rc = 1;
    } else if argc < 4 || argc > 4 && b_swarm == 0 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"wrong number of arguments for %s".as_ptr()
                                as *mut i8 as *const i8, z_vtab)
                }
        };
        rc = 1;
    } else {
        let mut n_alloc: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let z_arg: *mut i8 =
            union_strdup(&mut rc, unsafe { *argv.offset(3 as isize) });
        union_dequote(z_arg);
        p_stmt =
            unsafe {
                union_prepare_printf(&mut rc, pz_err_1, db,
                    c"SELECT * FROM (%z) ORDER BY 3".as_ptr() as *mut i8 as
                        *const i8, z_arg)
            };
        p_tab =
            union_malloc(&mut rc,
                    core::mem::size_of::<UnionTab>() as sqlite3_int64) as
                *mut UnionTab;
        if !(p_tab).is_null() {
            if !(rc == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"unionConnect".as_ptr() as *const i8,
                        c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 919,
                        c"rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            unsafe { (*p_tab).db = db };
            unsafe { (*p_tab).b_swarm = b_swarm };
            unsafe { (*p_tab).n_max_open = 9 };
        }
        if b_swarm != 0 {
            union_configure_vtab(&mut rc, unsafe { &mut *p_tab }, p_stmt,
                unsafe {
                    let __p =
                        unsafe { &*argv.offset(4 as isize) } as *const *const i8;
                    if __p.is_null() {
                        &[]
                    } else {
                        core::slice::from_raw_parts(__p, (argc - 4) as usize)
                    }
                }, pz_err_1);
        }
        while rc == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
            let z_db: *const i8 =
                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
            let z_tab: *const i8 =
                unsafe { sqlite3_column_text(p_stmt, 1) } as *const i8;
            let i_min: sqlite3_int64 =
                unsafe { sqlite3_column_int64(p_stmt, 2) };
            let i_max: sqlite3_int64 =
                unsafe { sqlite3_column_int64(p_stmt, 3) };
            let mut p_src: *mut UnionSrc = core::ptr::null_mut();
            if n_alloc <= unsafe { (*p_tab).n_src } {
                let n_new: i32 = if n_alloc != 0 { n_alloc * 2 } else { 8 };
                let a_new: *mut UnionSrc =
                    unsafe {
                            sqlite3_realloc64(unsafe { (*p_tab).a_src } as *mut (),
                                n_new as u64 * core::mem::size_of::<UnionSrc>() as u64)
                        } as *mut UnionSrc;
                if a_new == core::ptr::null_mut() {
                    rc = 7;
                    break;
                } else {
                    unsafe {
                        memset(unsafe {
                                    &raw mut *a_new.offset(unsafe { (*p_tab).n_src } as isize)
                                } as *mut (), 0,
                            (n_new - unsafe { (*p_tab).n_src }) as u64 *
                                core::mem::size_of::<UnionSrc>() as u64)
                    };
                    unsafe { (*p_tab).a_src = a_new };
                    n_alloc = n_new;
                }
            }
            if i_max < i_min ||
                    unsafe { (*p_tab).n_src } > 0 &&
                        i_min <=
                            unsafe {
                                (*unsafe {
                                            (*p_tab).a_src.offset((unsafe { (*p_tab).n_src } - 1) as
                                                    isize)
                                        }).i_max
                            } {
                unsafe {
                    *pz_err_1 =
                        unsafe {
                            sqlite3_mprintf(c"rowid range mismatch error".as_ptr() as
                                        *mut i8 as *const i8)
                        }
                };
                rc = 1;
            }
            if rc == 0 {
                p_src =
                    unsafe {
                        unsafe {
                            (*p_tab).a_src.offset({
                                        let __p = unsafe { &mut (*p_tab).n_src };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                        }
                    };
                unsafe {
                    (*p_src).z_tab =
                        union_strdup(&mut rc,
                            if !(z_tab).is_null() {
                                z_tab
                            } else { c"".as_ptr() as *mut i8 as *const i8 })
                };
                unsafe { (*p_src).i_min = i_min };
                unsafe { (*p_src).i_max = i_max };
                if b_swarm != 0 {
                    unsafe { (*p_src).z_file = union_strdup(&mut rc, z_db) };
                } else {
                    unsafe { (*p_src).z_db = union_strdup(&mut rc, z_db) };
                }
                if unsafe { (*p_tab).b_has_context } != 0 {
                    let z_context: *const i8 =
                        unsafe { sqlite3_column_text(p_stmt, 4) } as *const i8;
                    unsafe {
                        (*p_src).z_context = union_strdup(&mut rc, z_context)
                    };
                }
            }
        }
        union_finalize(&mut rc, p_stmt, unsafe { &mut *pz_err_1 });
        p_stmt = core::ptr::null_mut();
        if rc == 0 && unsafe { (*p_tab).n_src } == 0 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"no source tables configured".as_ptr() as
                                    *mut i8 as *const i8)
                    }
            };
            rc = 1;
        }
        if rc == 0 {
            if b_swarm != 0 {
                rc = union_open_database(p_tab, 0, pz_err_1);
            } else { rc = union_source_check(p_tab, pz_err_1); }
        }
        if rc == 0 {
            let p_src_1: *const UnionSrc =
                unsafe {
                        &raw mut *unsafe { (*p_tab).a_src.offset(0 as isize) }
                    } as *const UnionSrc;
            let tdb: *mut sqlite3 =
                if unsafe { (*p_tab).b_swarm } != 0 {
                    unsafe { (*p_src_1).db }
                } else { unsafe { (*p_tab).db } };
            p_stmt =
                unsafe {
                    union_prepare_printf(&mut rc, pz_err_1, tdb,
                        c"SELECT \'CREATE TABLE xyz(\'    || group_concat(quote(name) || \' \' || type, \', \')    || \')\',max((cid+1) * (type=\'INTEGER\' COLLATE nocase AND pk=1))-1 FROM pragma_table_info(%Q, ?)".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p_src_1).z_tab },
                        unsafe { (*p_src_1).z_db })
                };
        }
        if rc == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
            let z_decl: *const i8 =
                unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
            rc = unsafe { sqlite3_declare_vtab(db, z_decl) };
            unsafe {
                (*p_tab).i_pk = unsafe { sqlite3_column_int(p_stmt, 1) }
            };
        }
        union_finalize(&mut rc, p_stmt, unsafe { &mut *pz_err_1 });
    }
    if rc != 0 {
        union_disconnect(p_tab as *mut sqlite3_vtab);
        p_tab = core::ptr::null_mut();
    }
    unsafe { *pp_vtab_1 = p_tab as *mut sqlite3_vtab };
    return rc;
}
extern "C" fn union_open(p: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let mut p_csr: *mut UnionCsr = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = p; };
    p_csr =
        union_malloc(&mut rc,
                core::mem::size_of::<UnionCsr>() as sqlite3_int64) as
            *mut UnionCsr;
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_csr).base } };
    return rc;
}
extern "C" fn union_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *mut UnionCsr = cur as *mut UnionCsr;
    union_finalize_csr_stmt(unsafe { &mut *p_csr });
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}
extern "C" fn do_union_next(p_csr_1: *mut UnionCsr) -> i32 {
    let mut rc: i32 = 0;
    if (unsafe { (*p_csr_1).p_stmt }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"doUnionNext".as_ptr() as *const i8,
                c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 1059,
                c"pCsr->pStmt".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_step(unsafe { (*p_csr_1).p_stmt }) } != 100 {
        let p_tab: *mut UnionTab =
            unsafe { (*p_csr_1).base.p_vtab } as *mut UnionTab;
        rc = union_finalize_csr_stmt(unsafe { &mut *p_csr_1 });
        if rc == 0 && unsafe { (*p_tab).b_swarm } != 0 {
            {
                let __p = unsafe { &mut (*p_csr_1).i_tab };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p_csr_1).i_tab } < unsafe { (*p_tab).n_src } {
                let p_src: *const UnionSrc =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_tab).a_src.offset(unsafe { (*p_csr_1).i_tab } as isize)
                                    }
                        } as *const UnionSrc;
                if unsafe { (*p_csr_1).i_max_rowid } >=
                        unsafe { (*p_src).i_min } {
                    rc =
                        union_open_database(p_tab, unsafe { (*p_csr_1).i_tab },
                            unsafe { &mut (*p_tab).base.z_err_msg });
                    unsafe {
                        (*p_csr_1).p_stmt =
                            unsafe {
                                union_prepare_printf(&mut rc,
                                    unsafe { &mut (*p_tab).base.z_err_msg },
                                    unsafe { (*p_src).db },
                                    c"SELECT rowid, * FROM %Q %s %lld".as_ptr() as *mut i8 as
                                        *const i8, unsafe { (*p_src).z_tab },
                                    if unsafe { (*p_src).i_max } >
                                            unsafe { (*p_csr_1).i_max_rowid } {
                                        c"WHERE _rowid_ <=".as_ptr() as *mut i8
                                    } else { c"-- ".as_ptr() as *mut i8 },
                                    unsafe { (*p_csr_1).i_max_rowid })
                            }
                    };
                    if rc == 0 {
                        if (unsafe { (*p_csr_1).p_stmt }).is_null() as i32 as i64 !=
                                0 {
                            unsafe {
                                __assert_rtn(c"doUnionNext".as_ptr() as *const i8,
                                    c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 1077,
                                    c"pCsr->pStmt".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        union_incr_refcount(unsafe { &mut *p_tab },
                            unsafe { (*p_csr_1).i_tab });
                        rc = 100;
                    }
                }
            }
        }
    }
    return rc;
}
extern "C" fn union_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut rc: i32 = 0;
    '__b12: loop {
        '__c12: loop {
            rc = do_union_next(cur as *mut UnionCsr);
            break '__c12;
        }
        if !(rc == 100) { break '__b12; }
    }
    return rc;
}
extern "C" fn union_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_csr: *const UnionCsr = cur as *mut UnionCsr as *const UnionCsr;
    unsafe {
        sqlite3_result_value(ctx,
            unsafe {
                sqlite3_column_value(unsafe { (*p_csr).p_stmt }, i + 1)
            })
    };
    return 0;
}
extern "C" fn union_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p_csr: *const UnionCsr = cur as *mut UnionCsr as *const UnionCsr;
    unsafe {
        *p_rowid_1 =
            unsafe { sqlite3_column_int64(unsafe { (*p_csr).p_stmt }, 0) }
    };
    return 0;
}
extern "C" fn union_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *const UnionCsr = cur as *mut UnionCsr as *const UnionCsr;
    return (unsafe { (*p_csr).p_stmt } == core::ptr::null_mut()) as i32;
}
extern "C" fn union_filter(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let p_tab: *mut UnionTab =
        unsafe { (*p_vtab_cursor_1).p_vtab } as *mut UnionTab;
    let p_csr: *mut UnionCsr = p_vtab_cursor_1 as *mut UnionCsr;
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut b_zero: i32 = 0;
    let mut i_min: sqlite3_int64 =
        -1 as sqlite3_int64 -
            (4294967295u32 as sqlite3_int64 |
                (2147483647 as sqlite3_int64) << 32);
    let mut i_max: sqlite3_int64 =
        4294967295u32 as sqlite3_int64 | (2147483647 as sqlite3_int64) << 32;
    if !(idx_num_1 == 0 || idx_num_1 == 2 || idx_num_1 == 8 || idx_num_1 == 32
                                    || idx_num_1 == 16 || idx_num_1 == 4 || idx_num_1 == 32 | 8)
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"unionFilter".as_ptr() as *const i8,
                c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 1155,
                c"idxNum==0 || idxNum==SQLITE_INDEX_CONSTRAINT_EQ || idxNum==SQLITE_INDEX_CONSTRAINT_LE || idxNum==SQLITE_INDEX_CONSTRAINT_GE || idxNum==SQLITE_INDEX_CONSTRAINT_LT || idxNum==SQLITE_INDEX_CONSTRAINT_GT || idxNum==(SQLITE_INDEX_CONSTRAINT_GE|SQLITE_INDEX_CONSTRAINT_LE)".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    { let _ = idx_str_1; };
    if idx_num_1 == 2 {
        if !(argc == 1) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"unionFilter".as_ptr() as *const i8,
                    c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 1160,
                    c"argc==1".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        i_min =
            {
                i_max =
                    unsafe {
                        sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                    };
                i_max
            };
    } else {
        if idx_num_1 & (8 | 16) != 0 {
            if !(argc >= 1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"unionFilter".as_ptr() as *const i8,
                        c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 1165,
                        c"argc>=1".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            i_max =
                unsafe {
                    sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
                };
            if idx_num_1 & 16 != 0 {
                if i_max ==
                        -1 as sqlite3_int64 -
                            (4294967295u32 as sqlite3_int64 |
                                (2147483647 as sqlite3_int64) << 32) {
                    b_zero = 1;
                } else {
                    { let __p = &mut i_max; let __t = *__p; *__p -= 1; __t };
                }
            }
        }
        if idx_num_1 & (32 | 4) != 0 {
            if !(argc >= 1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"unionFilter".as_ptr() as *const i8,
                        c"unionvtab.c".as_ptr() as *mut i8 as *const i8, 1177,
                        c"argc>=1".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            i_min =
                unsafe {
                    sqlite3_value_int64(unsafe {
                            *argv.offset((argc - 1) as isize)
                        })
                };
            if idx_num_1 & 4 != 0 {
                if i_min ==
                        4294967295u32 as sqlite3_int64 |
                            (2147483647 as sqlite3_int64) << 32 {
                    b_zero = 1;
                } else {
                    { let __p = &mut i_min; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
    union_finalize_csr_stmt(unsafe { &mut *p_csr });
    if b_zero != 0 { return 0; }
    {
        i = 0;
        '__b13: loop {
            if !(i < unsafe { (*p_tab).n_src }) { break '__b13; }
            '__c13: loop {
                let p_src: *const UnionSrc =
                    unsafe {
                            &raw mut *unsafe { (*p_tab).a_src.offset(i as isize) }
                        } as *const UnionSrc;
                if i_min > unsafe { (*p_src).i_max } ||
                        i_max < unsafe { (*p_src).i_min } {
                    break '__c13;
                }
                z_sql =
                    unsafe {
                        sqlite3_mprintf(c"%z%sSELECT rowid, * FROM %s%q%s%Q".as_ptr()
                                    as *mut i8 as *const i8, z_sql,
                            if !(z_sql).is_null() {
                                c" UNION ALL ".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 },
                            if !(unsafe { (*p_src).z_db }).is_null() {
                                c"\'".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 },
                            if !(unsafe { (*p_src).z_db }).is_null() {
                                unsafe { (*p_src).z_db }
                            } else { c"".as_ptr() as *mut i8 },
                            if !(unsafe { (*p_src).z_db }).is_null() {
                                c"\'.".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 },
                            unsafe { (*p_src).z_tab })
                    };
                if z_sql == core::ptr::null_mut() { rc = 7; break '__b13; }
                if i_min == i_max {
                    z_sql =
                        unsafe {
                            sqlite3_mprintf(c"%z WHERE rowid=%lld".as_ptr() as *mut i8
                                    as *const i8, z_sql, i_min)
                        };
                } else {
                    let mut z_where: *const i8 =
                        c"WHERE".as_ptr() as *mut i8 as *const i8;
                    if i_min !=
                                -1 as sqlite3_int64 -
                                    (4294967295u32 as sqlite3_int64 |
                                        (2147483647 as sqlite3_int64) << 32) &&
                            i_min > unsafe { (*p_src).i_min } {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"%z WHERE rowid>=%lld".as_ptr() as *mut i8
                                        as *const i8, z_sql, i_min)
                            };
                        z_where = c"AND".as_ptr() as *mut i8 as *const i8;
                    }
                    if i_max !=
                                4294967295u32 as sqlite3_int64 |
                                    (2147483647 as sqlite3_int64) << 32 &&
                            i_max < unsafe { (*p_src).i_max } {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"%z %s rowid<=%lld".as_ptr() as *mut i8 as
                                        *const i8, z_sql, z_where, i_max)
                            };
                    }
                }
                if unsafe { (*p_tab).b_swarm } != 0 {
                    unsafe { (*p_csr).i_tab = i };
                    unsafe { (*p_csr).i_max_rowid = i_max };
                    rc =
                        union_open_database(p_tab, i,
                            unsafe { &mut (*p_tab).base.z_err_msg });
                    break '__b13;
                }
                break '__c13;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if z_sql == core::ptr::null_mut() {
        return rc;
    } else {
        let db: *mut sqlite3 =
            if unsafe { (*p_tab).b_swarm } != 0 {
                unsafe {
                    (*unsafe {
                                    &mut *unsafe {
                                                (*p_tab).a_src.offset(unsafe { (*p_csr).i_tab } as isize)
                                            }
                                }).db
                }
            } else { unsafe { (*p_tab).db } };
        unsafe {
            (*p_csr).p_stmt =
                union_prepare(&mut rc, db, z_sql as *const i8,
                    unsafe { &mut (*p_tab).base.z_err_msg })
        };
        if !(unsafe { (*p_csr).p_stmt }).is_null() {
            union_incr_refcount(unsafe { &mut *p_tab },
                unsafe { (*p_csr).i_tab });
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
    if rc != 0 { return rc; }
    return union_next(p_vtab_cursor_1);
}
extern "C" fn union_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let p_tab: *const UnionTab = tab as *mut UnionTab as *const UnionTab;
    let mut i_eq: i32 = -1;
    let mut i_lt: i32 = -1;
    let mut i_gt: i32 = -1;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b14: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b14;
            }
            '__c14: loop {
                let p: *const sqlite3_index_constraint =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_idx_info_1).a_constraint.offset(i as isize)
                                    }
                        } as *const sqlite3_index_constraint;
                if unsafe { (*p).usable } != 0 &&
                        (unsafe { (*p).i_column } < 0 ||
                            unsafe { (*p).i_column } == unsafe { (*p_tab).i_pk }) {
                    '__s15:
                        {
                        match unsafe { (*p).op } {
                            2 => { i_eq = i; }
                            8 => { i_lt = i; }
                            16 => { i_lt = i; }
                            32 => { i_gt = i; }
                            4 => { i_gt = i; }
                            _ => {}
                        }
                    }
                }
                break '__c14;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i_eq >= 0 {
        unsafe { (*p_idx_info_1).estimated_rows = 1 as sqlite3_int64 };
        unsafe { (*p_idx_info_1).idx_flags = 1 };
        unsafe { (*p_idx_info_1).estimated_cost = 3.0 };
        unsafe { (*p_idx_info_1).idx_num = 2 };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_eq as isize)
                        }).argv_index = 1
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_eq as isize)
                        }).omit = 1 as u8
        };
    } else {
        let mut i_cons: i32 = 1;
        let mut idx_num: i32 = 0;
        let mut n_row: sqlite3_int64 = 1000000 as sqlite3_int64;
        if i_lt >= 0 {
            n_row = n_row / 2 as sqlite3_int64;
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_lt as isize)
                            }).argv_index =
                    { let __p = &mut i_cons; let __t = *__p; *__p += 1; __t }
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_lt as isize)
                            }).omit = 1 as u8
            };
            idx_num |=
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_constraint.offset(i_lt as isize)
                                }).op
                    } as i32;
        }
        if i_gt >= 0 {
            n_row = n_row / 2 as sqlite3_int64;
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_gt as isize)
                            }).argv_index =
                    { let __p = &mut i_cons; let __t = *__p; *__p += 1; __t }
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i_gt as isize)
                            }).omit = 1 as u8
            };
            idx_num |=
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_constraint.offset(i_gt as isize)
                                }).op
                    } as i32;
        }
        unsafe { (*p_idx_info_1).estimated_rows = n_row };
        unsafe { (*p_idx_info_1).estimated_cost = 3.0 * n_row as f64 };
        unsafe { (*p_idx_info_1).idx_num = idx_num };
    }
    return 0;
}
extern "C" fn create_union_vtab(db: *mut sqlite3) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"unionvtab".as_ptr() as *mut i8 as *const i8,
                    &raw mut union_module as *const sqlite3_module,
                    core::ptr::null_mut())
            };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_module(db,
                        c"swarmvtab".as_ptr() as *mut i8 as *const i8,
                        &raw mut union_module as *const sqlite3_module,
                        db as *mut ())
                };
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_unionvtab_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    let mut rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    rc = create_union_vtab(db);
    return rc;
}
static open_flags: i32 = (1 | 64) as i32;
static mut union_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: Some(union_connect),
        x_connect: Some(union_connect),
        x_best_index: Some(union_best_index),
        x_disconnect: Some(union_disconnect),
        x_destroy: Some(union_disconnect),
        x_open: Some(union_open),
        x_close: Some(union_close),
        x_filter: Some(union_filter),
        x_next: Some(union_next),
        x_eof: Some(union_eof),
        x_column: Some(union_column),
        x_rowid: Some(union_rowid),
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn atoi(_: *const i8)
    -> i32;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}