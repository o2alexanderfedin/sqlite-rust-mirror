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
#[unsafe(no_mangle)]
pub static mut sqlite3_api: *const sqlite3_api_routines = core::ptr::null();
#[repr(C)]
#[derive(Copy, Clone)]
struct vt02_vtab {
    parent: sqlite3_vtab,
    db: *mut sqlite3,
    busy: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct vt02_cur {
    parent: sqlite3_vtab_cursor,
    i: sqlite3_int64,
    i_eof: sqlite3_int64,
    i_min: sqlite3_int64,
    i_incr: i32,
    m_d: u32,
}
#[unsafe(no_mangle)]
pub extern "C" fn vt02_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_v_tab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    unsafe {
        let mut p_vtab: *mut vt02_vtab = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let z_schema: *const i8 = p_aux_1 as *const i8;
        p_vtab =
            unsafe {
                    (unsafe {
                            (*sqlite3_api).malloc.unwrap()
                        })(core::mem::size_of::<vt02_vtab>() as i32)
                } as *mut vt02_vtab;
        if p_vtab == core::ptr::null_mut() {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        (unsafe {
                                (*sqlite3_api).mprintf.unwrap()
                            })(c"out of memory".as_ptr() as *mut i8 as *const i8)
                    }
            };
            return 7;
        }
        unsafe {
            memset(p_vtab as *mut (), 0,
                core::mem::size_of::<vt02_vtab>() as u64)
        };
        unsafe { (*p_vtab).db = db };
        rc =
            unsafe {
                (unsafe {
                        (*sqlite3_api).declare_vtab.unwrap()
                    })(db,
                    if !(z_schema).is_null() {
                        z_schema
                    } else { z_default_schema.as_ptr() as *const i8 })
            };
        if rc != 0 {
            unsafe {
                (unsafe { (*sqlite3_api).free.unwrap() })(p_vtab as *mut ())
            };
        } else { unsafe { *pp_v_tab_1 = unsafe { &mut (*p_vtab).parent } }; }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vt02_disconnect(p_v_tab_1: *mut sqlite3_vtab) -> i32 {
    unsafe {
        unsafe {
            (unsafe { (*sqlite3_api).free.unwrap() })(p_v_tab_1 as *mut ())
        };
        return 0;
    }
}
unsafe extern "C" fn vt02_err_msg(p_vtab_1: &mut sqlite3_vtab,
    z_format_1: *const i8, mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe {
            (unsafe {
                    (*sqlite3_api).free.unwrap()
                })((*p_vtab_1).z_err_msg as *mut ())
        };
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        (*p_vtab_1).z_err_msg =
            unsafe {
                (unsafe { (*sqlite3_api).vmprintf.unwrap() })(z_format_1, ap)
            };
        ();
    }
}
extern "C" fn vt02_open(p_v_tab_1: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    unsafe {
        let mut p_cur: *mut vt02_cur = core::ptr::null_mut();
        p_cur =
            unsafe {
                    (unsafe {
                            (*sqlite3_api).malloc.unwrap()
                        })(core::mem::size_of::<vt02_cur>() as i32)
                } as *mut vt02_cur;
        if p_cur == core::ptr::null_mut() {
            unsafe {
                vt02_err_msg(unsafe { &mut *p_v_tab_1 },
                    c"out of memory".as_ptr() as *mut i8 as *const i8)
            };
            return 7;
        }
        unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).parent } };
        unsafe { (*p_cur).i = -1 as sqlite3_int64 };
        return 0;
    }
}
extern "C" fn vt02_close(p_cursor_1: *mut sqlite3_vtab_cursor) -> i32 {
    unsafe {
        let p_cur: *mut vt02_cur = p_cursor_1 as *mut vt02_cur;
        unsafe {
            (unsafe { (*sqlite3_api).free.unwrap() })(p_cur as *mut ())
        };
        return 0;
    }
}
extern "C" fn vt02_eof(p_cursor_1: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const vt02_cur =
        p_cursor_1 as *mut vt02_cur as *const vt02_cur;
    return (unsafe { (*p_cur).i } < unsafe { (*p_cur).i_min } ||
                unsafe { (*p_cur).i } >= unsafe { (*p_cur).i_eof }) as i32;
}
extern "C" fn vt02_next(p_cursor_1: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut vt02_cur = p_cursor_1 as *mut vt02_cur;
    '__b0: loop {
        '__c0: loop {
            unsafe {
                (*p_cur).i += unsafe { (*p_cur).i_incr } as sqlite3_int64
            };
            if unsafe { (*p_cur).i } < unsafe { (*p_cur).i_min } ||
                    unsafe { (*p_cur).i } >= unsafe { (*p_cur).i_eof } {
                break '__b0;
            }
            break '__c0;
        }
        if !(unsafe { (*p_cur).m_d } &
                            (1 << unsafe { (*p_cur).i } % 10 as sqlite3_int64) as u32 ==
                        0 as u32) {
            break '__b0;
        }
    }
    return 0;
}
extern "C" fn vt02_filter(p_cursor_1: *mut sqlite3_vtab_cursor,
    mut idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    unsafe {
        let mut p_cur: *mut vt02_cur = core::ptr::null_mut();
        let mut b_use_offset: i32 = 0;
        let mut b_reverse: i32 = 0;
        let mut i_arg: i32 = 0;
        let mut i_orig_idx_num: i32 = 0;
        let mut i: i32 = 0;
        let mut e: i32 = 0;
        let mut m: i32 = 0;
        let mut v: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i__1: i32 = 0;
        let mut e__1: i32 = 0;
        let mut m__1: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_in: *mut sqlite3_value = core::ptr::null_mut();
        let mut p_val: *mut sqlite3_value = core::ptr::null_mut();
        let mut v__1: sqlite3_int64 = 0 as sqlite3_int64;
        let mut e_type: i32 = 0;
        let mut r: f64 = 0.0;
        let mut x: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_skip: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s2:
                {
                match __state {
                    0 => { p_cur = p_cursor_1 as *mut vt02_cur; __state = 3; }
                    2 => {
                        unsafe {
                            vt02_err_msg(unsafe {
                                    &mut *unsafe { (*p_cursor_1).p_vtab }
                                },
                                c"invalid idxNum for vt02: %d".as_ptr() as *mut i8 as
                                    *const i8, i_orig_idx_num)
                        };
                        __state = 104;
                    }
                    3 => { b_use_offset = 0; __state = 4; }
                    4 => { b_reverse = 0; __state = 5; }
                    5 => { i_arg = 0; __state = 6; }
                    6 => { i_orig_idx_num = idx_num_1; __state = 7; }
                    7 => { unsafe { (*p_cur).i_incr = 1 }; __state = 8; }
                    8 => {
                        unsafe { (*p_cur).i_min = 0 as sqlite3_int64 };
                        __state = 9;
                    }
                    9 => {
                        unsafe { (*p_cur).m_d = 1023 as u32 };
                        __state = 10;
                    }
                    10 => {
                        if idx_num_1 >= 1000 {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    11 => {
                        if idx_num_1 >= 100 { __state = 15; } else { __state = 14; }
                    }
                    12 => { b_reverse = 1; __state = 13; }
                    13 => { idx_num_1 -= 1000; __state = 11; }
                    14 => {
                        if idx_num_1 < 0 || idx_num_1 > 38 {
                            __state = 18;
                        } else { __state = 17; }
                    }
                    15 => { b_use_offset = 1; __state = 16; }
                    16 => { idx_num_1 -= 100; __state = 14; }
                    17 => {
                        if idx_num_1 >= 10 { __state = 20; } else { __state = 19; }
                    }
                    18 => { __state = 2; }
                    19 => {
                        if idx_num_1 == 0 { __state = 23; } else { __state = 24; }
                    }
                    20 => { unsafe { (*p_cur).i_incr *= 10 }; __state = 21; }
                    21 => { idx_num_1 -= 10; __state = 17; }
                    22 => {
                        if b_reverse != 0 { __state = 92; } else { __state = 91; }
                    }
                    23 => {
                        unsafe { (*p_cur).i = 0 as sqlite3_int64 };
                        __state = 25;
                    }
                    24 => {
                        if idx_num_1 == 1 { __state = 26; } else { __state = 27; }
                    }
                    25 => {
                        unsafe { (*p_cur).i_eof = 10000 as sqlite3_int64 };
                        __state = 22;
                    }
                    26 => {
                        unsafe {
                            (*p_cur).i =
                                unsafe {
                                    (unsafe {
                                            (*sqlite3_api).value_int64.unwrap()
                                        })(unsafe { *argv.offset(0 as isize) })
                                }
                        };
                        __state = 28;
                    }
                    27 => {
                        if idx_num_1 >= 2 && idx_num_1 <= 5 {
                            __state = 35;
                        } else { __state = 36; }
                    }
                    28 => {
                        if unsafe { (*p_cur).i } < 0 as i64 {
                            __state = 30;
                        } else { __state = 29; }
                    }
                    29 => {
                        if unsafe { (*p_cur).i } > 9999 as i64 {
                            __state = 32;
                        } else { __state = 31; }
                    }
                    30 => {
                        unsafe { (*p_cur).i = -1 as sqlite3_int64 };
                        __state = 29;
                    }
                    31 => {
                        unsafe {
                            (*p_cur).i_eof = unsafe { (*p_cur).i } + 1 as sqlite3_int64
                        };
                        __state = 33;
                    }
                    32 => {
                        unsafe { (*p_cur).i = 10000 as sqlite3_int64 };
                        __state = 31;
                    }
                    33 => {
                        if unsafe { (*p_cur).i } < 0 as i64 ||
                                unsafe { (*p_cur).i } > 9999 as i64 {
                            __state = 34;
                        } else { __state = 22; }
                    }
                    34 => {
                        unsafe { (*p_cur).i = unsafe { (*p_cur).i_eof } };
                        __state = 22;
                    }
                    35 => { __state = 37; }
                    36 => {
                        if idx_num_1 >= 6 && idx_num_1 <= 8 {
                            __state = 50;
                        } else { __state = 51; }
                    }
                    37 => { e = idx_num_1 - 2; __state = 38; }
                    38 => {
                        if !(e <= argc - 1) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"vt02Filter".as_ptr() as *const i8,
                                    c"vt02.c".as_ptr() as *mut i8 as *const i8, 400,
                                    c"e<=argc-1".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 39;
                    }
                    39 => {
                        unsafe { (*p_cur).i = 0 as sqlite3_int64 };
                        __state = 40;
                    }
                    40 => { { m = 1000; i = 0 }; __state = 41; }
                    41 => { if i <= e { __state = 42; } else { __state = 22; } }
                    42 => {
                        v =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).value_int64.unwrap()
                                    })(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            };
                        __state = 44;
                    }
                    43 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            m /= 10
                        };
                        __state = 41;
                    }
                    44 => {
                        if v < 0 as i64 { __state = 46; } else { __state = 45; }
                    }
                    45 => {
                        if v > 9 as i64 { __state = 48; } else { __state = 47; }
                    }
                    46 => { v = 0 as sqlite3_int64; __state = 45; }
                    47 => {
                        unsafe { (*p_cur).i += m as sqlite3_int64 * v };
                        __state = 49;
                    }
                    48 => { v = 9 as sqlite3_int64; __state = 47; }
                    49 => {
                        unsafe {
                            (*p_cur).i_eof = unsafe { (*p_cur).i } + m as sqlite3_int64
                        };
                        __state = 43;
                    }
                    50 => { __state = 52; }
                    51 => { __state = 2; }
                    52 => { __state = 53; }
                    53 => { e__1 = idx_num_1 - 6; __state = 54; }
                    54 => {
                        if !(e__1 <= argc - 2) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"vt02Filter".as_ptr() as *const i8,
                                    c"vt02.c".as_ptr() as *mut i8 as *const i8, 413,
                                    c"e<=argc-2".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 55;
                    }
                    55 => {
                        unsafe { (*p_cur).i = 0 as sqlite3_int64 };
                        __state = 56;
                    }
                    56 => { { m__1 = 1000; i__1 = 0 }; __state = 58; }
                    57 => { unsafe { (*p_cur).m_d = 0 as u32 }; __state = 72; }
                    58 => {
                        if i__1 <= e__1 { __state = 59; } else { __state = 57; }
                    }
                    59 => { __state = 61; }
                    60 => {
                        {
                            { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                            m__1 /= 10
                        };
                        __state = 58;
                    }
                    61 => { p_val = core::ptr::null_mut(); __state = 62; }
                    62 => {
                        if unsafe {
                                        (unsafe {
                                                (*sqlite3_api).vtab_in_first.unwrap()
                                            })(core::ptr::null_mut(), &mut p_val)
                                    } != 21 ||
                                unsafe {
                                        (unsafe {
                                                (*sqlite3_api).vtab_in_first.unwrap()
                                            })(unsafe { *argv.offset(i_arg as isize) }, &mut p_val)
                                    } != 1 {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    63 => {
                        v__1 =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).value_int64.unwrap()
                                    })(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            };
                        __state = 66;
                    }
                    64 => {
                        unsafe {
                            vt02_err_msg(unsafe {
                                    &mut *unsafe { (*p_cursor_1).p_vtab }
                                },
                                c"unexpected success from sqlite3_vtab_in_first()".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 65;
                    }
                    65 => { return 1; }
                    66 => {
                        if v__1 < 0 as i64 { __state = 68; } else { __state = 67; }
                    }
                    67 => {
                        if v__1 > 9 as i64 { __state = 70; } else { __state = 69; }
                    }
                    68 => { v__1 = 0 as sqlite3_int64; __state = 67; }
                    69 => {
                        unsafe { (*p_cur).i += m__1 as sqlite3_int64 * v__1 };
                        __state = 71;
                    }
                    70 => { v__1 = 9 as sqlite3_int64; __state = 69; }
                    71 => {
                        unsafe {
                            (*p_cur).i_eof =
                                unsafe { (*p_cur).i } + m__1 as sqlite3_int64
                        };
                        __state = 60;
                    }
                    72 => {
                        p_in =
                            unsafe {
                                *argv.offset({
                                                let __p = &mut i_arg;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            };
                        __state = 73;
                    }
                    73 => {
                        if !(unsafe {
                                                    (unsafe { (*sqlite3_api).value_type.unwrap() })(p_in)
                                                } == 5) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"vt02Filter".as_ptr() as *const i8,
                                    c"vt02.c".as_ptr() as *mut i8 as *const i8, 433,
                                    c"sqlite3_value_type(pIn)==SQLITE_NULL".as_ptr() as *mut i8
                                        as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 74;
                    }
                    74 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).vtab_in_first.unwrap()
                                    })(p_in, &mut p_val)
                            };
                        __state = 76;
                    }
                    75 => {
                        if rc != 0 && rc != 101 {
                            __state = 89;
                        } else { __state = 22; }
                    }
                    76 => {
                        if rc == 0 && p_val != core::ptr::null_mut() {
                            __state = 77;
                        } else { __state = 75; }
                    }
                    77 => {
                        e_type =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).value_numeric_type.unwrap()
                                    })(p_val)
                            };
                        __state = 79;
                    }
                    78 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).vtab_in_next.unwrap()
                                    })(p_in, &mut p_val)
                            };
                        __state = 76;
                    }
                    79 => {
                        if e_type == 2 { __state = 81; } else { __state = 82; }
                    }
                    80 => {
                        i__1 =
                            unsafe {
                                (unsafe { (*sqlite3_api).value_int.unwrap() })(p_val)
                            };
                        __state = 86;
                    }
                    81 => {
                        r =
                            unsafe {
                                (unsafe { (*sqlite3_api).value_double.unwrap() })(p_val)
                            };
                        __state = 83;
                    }
                    82 => {
                        if e_type != 1 { __state = 85; } else { __state = 80; }
                    }
                    83 => {
                        if r < 0.0 || r > 9.0 || r != r as i32 as f64 {
                            __state = 84;
                        } else { __state = 80; }
                    }
                    84 => { __state = 78; }
                    85 => { __state = 78; }
                    86 => {
                        if i__1 < 0 || i__1 > 9 {
                            __state = 88;
                        } else { __state = 87; }
                    }
                    87 => {
                        unsafe { (*p_cur).m_d |= (1 << i__1) as u32 };
                        __state = 78;
                    }
                    88 => { __state = 78; }
                    89 => {
                        unsafe {
                            vt02_err_msg(unsafe {
                                    &mut *unsafe { (*p_cursor_1).p_vtab }
                                },
                                c"Error from sqlite3_vtab_in_first/next()".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 90;
                    }
                    90 => { return rc; }
                    91 => {
                        if b_use_offset != 0 {
                            __state = 100;
                        } else { __state = 99; }
                    }
                    92 => { __state = 93; }
                    93 => {
                        x =
                            unsafe { (*p_cur).i } +
                                (unsafe { (*p_cur).i_eof } - unsafe { (*p_cur).i }) /
                                        unsafe { (*p_cur).i_incr } as sqlite3_int64 *
                                    unsafe { (*p_cur).i_incr } as sqlite3_int64;
                        __state = 94;
                    }
                    94 => {
                        if x >= unsafe { (*p_cur).i_eof } {
                            __state = 96;
                        } else { __state = 95; }
                    }
                    95 => {
                        unsafe { (*p_cur).i_incr = -unsafe { (*p_cur).i_incr } };
                        __state = 97;
                    }
                    96 => {
                        x -= unsafe { (*p_cur).i_incr } as sqlite3_int64;
                        __state = 95;
                    }
                    97 => {
                        unsafe { (*p_cur).i_min = unsafe { (*p_cur).i } };
                        __state = 98;
                    }
                    98 => { unsafe { (*p_cur).i = x }; __state = 91; }
                    99 => { return 0; }
                    100 => {
                        n_skip =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).value_int.unwrap()
                                    })(unsafe { *argv.offset(i_arg as isize) })
                            };
                        __state = 101;
                    }
                    101 => {
                        if { let __p = &mut n_skip; let __t = *__p; *__p -= 1; __t }
                                    > 0 && (vt02_eof(p_cursor_1) == 0) as i32 != 0 {
                            __state = 102;
                        } else { __state = 99; }
                    }
                    102 => { vt02_next(p_cursor_1); __state = 101; }
                    103 => { __state = 2; }
                    104 => { return 1; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn vt02_column(p_cursor_1: *mut sqlite3_vtab_cursor,
    context: *mut sqlite3_context, n_1: i32) -> i32 {
    unsafe {
        let p_cur: *const vt02_cur =
            p_cursor_1 as *mut vt02_cur as *const vt02_cur;
        let mut v: i32 = unsafe { (*p_cur).i } as i32;
        if n_1 == 0 {
            unsafe {
                (unsafe { (*sqlite3_api).result_int.unwrap() })(context, v)
            };
        } else if n_1 >= 1 && n_1 <= 4 {
            v = v / i_divisor[n_1 as usize] as i32 % 10;
            unsafe {
                (unsafe { (*sqlite3_api).result_int.unwrap() })(context, v)
            };
        }
        return 0;
    }
}
extern "C" fn vt02_rowid(p_cursor_1: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite3_int64) -> i32 {
    let p_cur: *const vt02_cur =
        p_cursor_1 as *mut vt02_cur as *const vt02_cur;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i } + 1 as sqlite3_int64 };
    return 0;
}
unsafe extern "C" fn sqlite3_run_sql(db: *mut sqlite3,
    p_v_tab_1: &mut sqlite3_vtab, z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        if z_format_1 == core::ptr::null() {
            z_sql =
                unsafe {
                    let __ap = &mut ap;
                    let __va_p = *__ap;
                    *__ap =
                        (*__ap).add((core::mem::size_of::<*mut i8>() + 7) & !7);
                    *(__va_p as *const *mut i8)
                };
        } else {
            z_sql =
                unsafe {
                    (unsafe {
                            (*sqlite3_api).vmprintf.unwrap()
                        })(z_format_1, ap)
                };
        }
        ();
        if !(z_sql).is_null() {
            let mut z_err_msg: *mut i8 = core::ptr::null_mut();
            {
                let _ =
                    unsafe {
                        (unsafe {
                                (*sqlite3_api).exec.unwrap()
                            })(db, z_sql as *const i8,
                            unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut (), i32, *mut *mut i8,
                                            *mut *mut i8) -> i32>(0 as *const ())
                            }, core::ptr::null_mut(), &mut z_err_msg)
                    };
            };
            if !(z_err_msg).is_null() {
                if (*p_v_tab_1).z_err_msg == core::ptr::null_mut() {
                    (*p_v_tab_1).z_err_msg =
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).mprintf.unwrap()
                                })(c"%s in [%s]".as_ptr() as *mut i8 as *const i8,
                                z_err_msg, z_sql)
                        };
                }
                unsafe {
                    (unsafe {
                            (*sqlite3_api).free.unwrap()
                        })(z_err_msg as *mut ())
                };
            }
            unsafe {
                (unsafe { (*sqlite3_api).free.unwrap() })(z_sql as *mut ())
            };
        }
    }
}
extern "C" fn sqlite3_best_index_log(p_info_1: *mut sqlite3_index_info,
    z_log_tab_1: *const i8, db: *mut sqlite3, az_colname_1: *const *const i8,
    p_v_tab_1: *mut sqlite3_vtab) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_str: *mut sqlite3_str = core::ptr::null_mut();
        let mut i_bi: i32 = 0;
        if unsafe {
                    (unsafe {
                            (*sqlite3_api).table_column_metadata.unwrap()
                        })(db, core::ptr::null(), z_log_tab_1, core::ptr::null(),
                        core::ptr::null_mut(), core::ptr::null_mut(),
                        core::ptr::null_mut(), core::ptr::null_mut(),
                        core::ptr::null_mut())
                } != 0 {
            unsafe {
                sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                    c"CREATE TABLE IF NOT EXISTS temp.\"%w\"(\n bi INT,          -- BestIndex call number\n vn TEXT,         -- Variable Name\n ix INT,          -- Index or value\n cn TEXT,         -- Column Name\n op INT,          -- Opcode or argvIndex\n ux INT,          -- usable for omit flag\n rx BOOLEAN,      -- Right-hand side value is available\n rhs ANY,         -- RHS value\n cs TEXT,         -- Collating Sequence\n inop BOOLEAN     -- IN operator capable of batch reads\n);".as_ptr()
                            as *mut i8 as *const i8, z_log_tab_1)
            };
            i_bi = 1;
        } else {
            let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
            let mut z_sql: *mut i8 = core::ptr::null_mut();
            z_sql =
                unsafe {
                    (unsafe {
                            (*sqlite3_api).mprintf.unwrap()
                        })(c"SELECT max(bi) FROM temp.\"%w\"".as_ptr() as *mut i8 as
                            *const i8, z_log_tab_1)
                };
            if z_sql == core::ptr::null_mut() {
                unsafe {
                    (unsafe {
                            (*sqlite3_api).free.unwrap()
                        })(unsafe { (*p_v_tab_1).z_err_msg } as *mut ())
                };
                unsafe {
                    (*p_v_tab_1).z_err_msg =
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).mprintf.unwrap()
                                })(c"out of memory".as_ptr() as *mut i8 as *const i8)
                        }
                };
                return;
            }
            rc =
                unsafe {
                    (unsafe {
                            (*sqlite3_api).prepare_v2.unwrap()
                        })(db, z_sql as *const i8, -1, &mut p_stmt,
                        core::ptr::null_mut())
                };
            unsafe {
                (unsafe { (*sqlite3_api).free.unwrap() })(z_sql as *mut ())
            };
            if rc != 0 {
                unsafe {
                    (unsafe {
                            (*sqlite3_api).free.unwrap()
                        })(unsafe { (*p_v_tab_1).z_err_msg } as *mut ())
                };
                unsafe {
                    (*p_v_tab_1).z_err_msg =
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).mprintf.unwrap()
                                })(c"%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { (unsafe { (*sqlite3_api).errmsg.unwrap() })(db) })
                        }
                };
                i_bi = 0;
            } else if unsafe {
                        (unsafe { (*sqlite3_api).step.unwrap() })(p_stmt)
                    } == 100 {
                i_bi =
                    unsafe {
                            (unsafe { (*sqlite3_api).column_int.unwrap() })(p_stmt, 0)
                        } + 1;
            } else { i_bi = 1; }
            unsafe { (unsafe { (*sqlite3_api).finalize.unwrap() })(p_stmt) };
        }
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix) VALUES(%d,\'nConstraint\',%d)RETURNING iif(bi=%d,\'ok\',RAISE(ABORT,\'wrong trigger\'))".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe { (*p_info_1).n_constraint }, i_bi)
        };
        {
            i = 0;
            '__b3: loop {
                if !(i < unsafe { (*p_info_1).n_constraint }) { break '__b3; }
                '__c3: loop {
                    let mut p_val: *mut sqlite3_value = core::ptr::null_mut();
                    let mut z_sql_1: *mut i8 = core::ptr::null_mut();
                    let i_col: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_info_1).a_constraint.offset(i as isize)
                                    }).i_column
                        };
                    let op: i32 =
                        unsafe {
                                (*unsafe { (*p_info_1).a_constraint.offset(i as isize) }).op
                            } as i32;
                    let mut z_col: *const i8 = core::ptr::null();
                    if op == 73 || op == 74 {
                        z_col = c"".as_ptr() as *mut i8 as *const i8;
                    } else if i_col < 0 {
                        z_col = c"rowid".as_ptr() as *mut i8 as *const i8;
                    } else {
                        z_col = unsafe { *az_colname_1.offset(i_col as isize) };
                    }
                    p_str =
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).str_new.unwrap()
                                })(core::ptr::null_mut())
                        };
                    unsafe {
                        (unsafe {
                                (*sqlite3_api).str_appendf.unwrap()
                            })(p_str,
                            c"INSERT INTO temp.\"%w\"(bi,vn,ix,cn,op,ux,rx,rhs,cs,inop)VALUES(%d,\'aConstraint\',%d,%Q,%d,%d".as_ptr()
                                    as *mut i8 as *const i8, z_log_tab_1, i_bi, i, z_col, op,
                            unsafe {
                                    (*unsafe {
                                                (*p_info_1).a_constraint.offset(i as isize)
                                            }).usable
                                } as i32)
                    };
                    p_val = core::ptr::null_mut();
                    rc =
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).vtab_rhs_value.unwrap()
                                })(p_info_1, i, &mut p_val)
                        };
                    if !(p_val != core::ptr::null_mut() || rc != 0) as i32 as
                                i64 != 0 {
                        unsafe {
                            __assert_rtn(c"sqlite3BestIndexLog".as_ptr() as *const i8,
                                c"vt02.c".as_ptr() as *mut i8 as *const i8, 660,
                                c"pVal!=0 || rc!=SQLITE_OK".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    if rc == 0 {
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).str_appendf.unwrap()
                                })(p_str, c",1,?1".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).str_appendf.unwrap()
                                })(p_str, c",0,NULL".as_ptr() as *mut i8 as *const i8)
                        };
                    }
                    unsafe {
                        (unsafe {
                                (*sqlite3_api).str_appendf.unwrap()
                            })(p_str, c",%Q,%d)".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).vtab_collation.unwrap()
                                    })(p_info_1, i)
                            },
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).vtab_in.unwrap()
                                    })(p_info_1, i, -1)
                            })
                    };
                    z_sql_1 =
                        unsafe {
                            (unsafe { (*sqlite3_api).str_finish.unwrap() })(p_str)
                        };
                    if z_sql_1 == core::ptr::null_mut() {
                        if unsafe { (*p_v_tab_1).z_err_msg } ==
                                core::ptr::null_mut() {
                            unsafe {
                                (*p_v_tab_1).z_err_msg =
                                    unsafe {
                                        (unsafe {
                                                (*sqlite3_api).mprintf.unwrap()
                                            })(c"out of memory".as_ptr() as *mut i8 as *const i8)
                                    }
                            };
                        }
                    } else {
                        let mut p_stmt_1: *mut sqlite3_stmt = core::ptr::null_mut();
                        rc =
                            unsafe {
                                (unsafe {
                                        (*sqlite3_api).prepare_v2.unwrap()
                                    })(db, z_sql_1 as *const i8, -1, &mut p_stmt_1,
                                    core::ptr::null_mut())
                            };
                        if rc != 0 {
                            if unsafe { (*p_v_tab_1).z_err_msg } ==
                                    core::ptr::null_mut() {
                                unsafe {
                                    (*p_v_tab_1).z_err_msg =
                                        unsafe {
                                            (unsafe {
                                                    (*sqlite3_api).mprintf.unwrap()
                                                })(c"%s".as_ptr() as *mut i8 as *const i8,
                                                unsafe { (unsafe { (*sqlite3_api).errmsg.unwrap() })(db) })
                                        }
                                };
                            }
                        } else {
                            if !(p_val).is_null() {
                                unsafe {
                                    (unsafe {
                                            (*sqlite3_api).bind_value.unwrap()
                                        })(p_stmt_1, 1, p_val as *const sqlite3_value)
                                };
                            }
                            unsafe {
                                (unsafe { (*sqlite3_api).step.unwrap() })(p_stmt_1)
                            };
                            rc =
                                unsafe {
                                    (unsafe { (*sqlite3_api).reset.unwrap() })(p_stmt_1)
                                };
                            if rc != 0 &&
                                    unsafe { (*p_v_tab_1).z_err_msg } == core::ptr::null_mut() {
                                unsafe {
                                    (*p_v_tab_1).z_err_msg =
                                        unsafe {
                                            (unsafe {
                                                    (*sqlite3_api).mprintf.unwrap()
                                                })(c"%s".as_ptr() as *mut i8 as *const i8,
                                                unsafe { (unsafe { (*sqlite3_api).errmsg.unwrap() })(db) })
                                        }
                                };
                            }
                        }
                        unsafe {
                            (unsafe { (*sqlite3_api).finalize.unwrap() })(p_stmt_1)
                        };
                        unsafe {
                            (unsafe {
                                    (*sqlite3_api).free.unwrap()
                                })(z_sql_1 as *mut ())
                        };
                    }
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix) VALUES(%d,\'nOrderBy\',%d)".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe { (*p_info_1).n_order_by })
        };
        {
            i = 0;
            '__b4: loop {
                if !(i < unsafe { (*p_info_1).n_order_by }) { break '__b4; }
                '__c4: loop {
                    let i_col_1: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_info_1).a_order_by.offset(i as isize)
                                    }).i_column
                        };
                    unsafe {
                        sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                            c"INSERT INTO temp.\"%w\"(bi,vn,ix,cn,op)VALUES(%d,\'aOrderBy\',%d,%Q,%d)".as_ptr()
                                    as *mut i8 as *const i8, z_log_tab_1, i_bi, i,
                            if i_col_1 >= 0 {
                                unsafe { *az_colname_1.offset(i_col_1 as isize) }
                            } else { c"rowid".as_ptr() as *mut i8 as *const i8 },
                            unsafe {
                                    (*unsafe { (*p_info_1).a_order_by.offset(i as isize) }).desc
                                } as i32)
                    };
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix) VALUES(%d,\'sqlite3_vtab_distinct\',%d)".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe {
                    (unsafe { (*sqlite3_api).vtab_distinct.unwrap() })(p_info_1)
                })
        };
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix) VALUES(%d,\'colUsed\',%lld)".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe { (*p_info_1).col_used })
        };
        {
            i = 0;
            '__b5: loop {
                if !(i < unsafe { (*p_info_1).n_constraint }) { break '__b5; }
                '__c5: loop {
                    let i_col_2: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_info_1).a_constraint.offset(i as isize)
                                    }).i_column
                        };
                    let op: i32 =
                        unsafe {
                                (*unsafe { (*p_info_1).a_constraint.offset(i as isize) }).op
                            } as i32;
                    let mut z_col_1: *const i8 = core::ptr::null();
                    if op == 73 || op == 74 {
                        z_col_1 = c"".as_ptr() as *mut i8 as *const i8;
                    } else if i_col_2 < 0 {
                        z_col_1 = c"rowid".as_ptr() as *mut i8 as *const i8;
                    } else {
                        z_col_1 = unsafe { *az_colname_1.offset(i_col_2 as isize) };
                    }
                    unsafe {
                        sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                            c"INSERT INTO temp.\"%w\"(bi,vn,ix,cn,op,ux)VALUES(%d,\'aConstraintUsage\',%d,%Q,%d,%d)".as_ptr()
                                    as *mut i8 as *const i8, z_log_tab_1, i_bi, i, z_col_1,
                            unsafe {
                                (*unsafe {
                                            (*p_info_1).a_constraint_usage.offset(i as isize)
                                        }).argv_index
                            },
                            unsafe {
                                    (*unsafe {
                                                (*p_info_1).a_constraint_usage.offset(i as isize)
                                            }).omit
                                } as i32)
                    };
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix)VALUES(%d,\'idxNum\',%d)".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe { (*p_info_1).idx_num })
        };
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix)VALUES(%d,\'estimatedCost\',%f)".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe { (*p_info_1).estimated_cost })
        };
        unsafe {
            sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                c"INSERT INTO temp.\"%w\"(bi,vn,ix)VALUES(%d,\'estimatedRows\',%lld)".as_ptr()
                        as *mut i8 as *const i8, z_log_tab_1, i_bi,
                unsafe { (*p_info_1).estimated_rows })
        };
        if !(unsafe { (*p_info_1).idx_str }).is_null() {
            unsafe {
                sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                    c"INSERT INTO temp.\"%w\"(bi,vn,ix)VALUES(%d,\'idxStr\',%Q)".as_ptr()
                            as *mut i8 as *const i8, z_log_tab_1, i_bi,
                    unsafe { (*p_info_1).idx_str })
            };
            unsafe {
                sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                    c"INSERT INTO temp.\"%w\"(bi,vn,ix)VALUES(%d,\'needToFreeIdxStr\',%d)".as_ptr()
                            as *mut i8 as *const i8, z_log_tab_1, i_bi,
                    unsafe { (*p_info_1).need_to_free_idx_str })
            };
        }
        if unsafe { (*p_info_1).n_order_by } != 0 {
            unsafe {
                sqlite3_run_sql(db, unsafe { &mut *p_v_tab_1 },
                    c"INSERT INTO temp.\"%w\"(bi,vn,ix)VALUES(%d,\'orderByConsumed\',%d)".as_ptr()
                            as *mut i8 as *const i8, z_log_tab_1, i_bi,
                    unsafe { (*p_info_1).order_by_consumed })
            };
        }
    }
}
extern "C" fn vt02_best_index(p_v_tab_1: *mut sqlite3_vtab,
    p_info_1: *mut sqlite3_index_info) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut is_eq: [i32; 5] = [0; 5];
        let mut is_used: [i32; 5] = [0; 5];
        let mut argv_index: i32 = 0;
        let mut i_offset: i32 = -1;
        let mut p_x: *mut () = core::ptr::null_mut();
        let mut flags: i32 = 0;
        let mut z_log_tab: *const i8 = core::ptr::null();
        let mut i_flag_term: i32 = -1;
        let mut i_log_term: i32 = -1;
        let mut i_in: i32 = -1;
        let mut p_self: *mut vt02_vtab = core::ptr::null_mut();
        p_self = p_v_tab_1 as *mut vt02_vtab;
        if unsafe { (*p_self).busy } != 0 {
            unsafe {
                vt02_err_msg(unsafe { &mut *p_v_tab_1 },
                    c"recursive use  of vt02 prohibited".as_ptr() as *mut i8 as
                        *const i8)
            };
            return 19;
        }
        {
            let __p = unsafe { &mut (*p_self).busy };
            let __t = *__p;
            *__p += 1;
            __t
        };
        {
            i = 0;
            '__b6: loop {
                if !(i < unsafe { (*p_info_1).n_constraint }) { break '__b6; }
                '__c6: loop {
                    let mut p_val: *mut sqlite3_value = core::ptr::null_mut();
                    if (unsafe {
                                        (*unsafe {
                                                    (*p_info_1).a_constraint.offset(i as isize)
                                                }).usable
                                    } == 0) as i32 != 0 {
                        break '__c6;
                    }
                    if unsafe {
                                    (*unsafe { (*p_info_1).a_constraint.offset(i as isize) }).op
                                } as i32 != 2 {
                        break '__c6;
                    }
                    '__s7:
                        {
                        match unsafe {
                                (*unsafe {
                                            (*p_info_1).a_constraint.offset(i as isize)
                                        }).i_column
                            } {
                            5 => {
                                if unsafe {
                                                (unsafe {
                                                        (*sqlite3_api).vtab_rhs_value.unwrap()
                                                    })(p_info_1, i, &mut p_val)
                                            } == 0 &&
                                        unsafe {
                                                (unsafe { (*sqlite3_api).value_type.unwrap() })(p_val)
                                            } == 1 {
                                    flags =
                                        unsafe {
                                            (unsafe { (*sqlite3_api).value_int.unwrap() })(p_val)
                                        };
                                }
                                i_flag_term = i;
                            }
                            6 => {
                                if unsafe {
                                                (unsafe {
                                                        (*sqlite3_api).vtab_rhs_value.unwrap()
                                                    })(p_info_1, i, &mut p_val)
                                            } == 0 &&
                                        unsafe {
                                                (unsafe { (*sqlite3_api).value_type.unwrap() })(p_val)
                                            } == 3 {
                                    z_log_tab =
                                        unsafe {
                                                (unsafe { (*sqlite3_api).value_text.unwrap() })(p_val)
                                            } as *const i8;
                                }
                                i_log_term = i;
                            }
                            _ => {}
                        }
                    }
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            memset(&raw mut is_eq[0 as usize] as *mut i32 as *mut (), 255,
                core::mem::size_of::<[i32; 5]>() as u64)
        };
        unsafe {
            memset(&raw mut is_used[0 as usize] as *mut i32 as *mut (), 255,
                core::mem::size_of::<[i32; 5]>() as u64)
        };
        {
            i = 0;
            '__b8: loop {
                if !(i < unsafe { (*p_info_1).n_constraint }) { break '__b8; }
                '__c8: loop {
                    let mut j: i32 =
                        unsafe {
                            (*unsafe {
                                        (*p_info_1).a_constraint.offset(i as isize)
                                    }).i_column
                        };
                    if j >= 5 { break '__c8; }
                    if unsafe {
                                        (*unsafe {
                                                    (*p_info_1).a_constraint.offset(i as isize)
                                                }).usable
                                    } as i32 == 0 && flags & 1 == 0 {
                        break '__c8;
                    }
                    if j < 0 { j = 0; }
                    '__s9:
                        {
                        match unsafe {
                                (*unsafe { (*p_info_1).a_constraint.offset(i as isize) }).op
                            } {
                            150 => { is_eq[j as usize] = i; }
                            2 => { is_eq[j as usize] = i; }
                            16 => { is_used[j as usize] = i; }
                            8 => { is_used[j as usize] = i; }
                            4 => { is_used[j as usize] = i; }
                            32 => { is_used[j as usize] = i; }
                            74 => { i_offset = i; }
                            _ => {}
                        }
                    }
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if is_eq[0 as usize] >= 0 {
            unsafe { (*p_info_1).estimated_cost = 1 as f64 };
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(is_eq[0 as usize] as
                                        isize)
                            }).argv_index =
                    { let __p = &mut argv_index; *__p += 1; *__p }
            };
            if flags & 32 != 0 {
                unsafe {
                    (*unsafe {
                                    (*p_info_1).a_constraint_usage.offset(is_eq[0 as usize] as
                                            isize)
                                }).omit = 1 as u8
                };
            }
            unsafe { (*p_info_1).idx_num = 1 };
        } else if is_eq[1 as usize] < 0 {
            unsafe { (*p_info_1).idx_num = 0 };
            unsafe { (*p_info_1).estimated_cost = 10000 as f64 };
        } else {
            let mut v: i32 = 1000;
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(is_eq[1 as usize] as
                                        isize)
                            }).argv_index =
                    { let __p = &mut argv_index; *__p += 1; *__p }
            };
            if flags & 32 != 0 {
                unsafe {
                    (*unsafe {
                                    (*p_info_1).a_constraint_usage.offset(is_eq[1 as usize] as
                                            isize)
                                }).omit = 1 as u8
                };
            }
            {
                i = 2;
                '__b10: loop {
                    if !(i <= 4 && is_eq[i as usize] >= 0) { break '__b10; }
                    '__c10: loop {
                        if i == 4 &&
                                unsafe {
                                        (unsafe {
                                                (*sqlite3_api).vtab_in.unwrap()
                                            })(p_info_1, is_eq[4 as usize], 0)
                                    } != 0 {
                            break '__b10;
                        }
                        unsafe {
                            (*unsafe {
                                            (*p_info_1).a_constraint_usage.offset(is_eq[i as usize] as
                                                    isize)
                                        }).argv_index =
                                { let __p = &mut argv_index; *__p += 1; *__p }
                        };
                        if flags & 32 != 0 {
                            unsafe {
                                (*unsafe {
                                                (*p_info_1).a_constraint_usage.offset(is_eq[i as usize] as
                                                        isize)
                                            }).omit = 1 as u8
                            };
                        }
                        v /= 10;
                        break '__c10;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_info_1).idx_num = i };
            if is_eq[4 as usize] >= 0 &&
                    unsafe {
                            (unsafe {
                                    (*sqlite3_api).vtab_in.unwrap()
                                })(p_info_1, is_eq[4 as usize], 1)
                        } != 0 {
                i_in = is_eq[4 as usize];
                unsafe {
                    (*unsafe {
                                    (*p_info_1).a_constraint_usage.offset(i_in as isize)
                                }).argv_index =
                        { let __p = &mut argv_index; *__p += 1; *__p }
                };
                if flags & 32 != 0 {
                    unsafe {
                        (*unsafe {
                                        (*p_info_1).a_constraint_usage.offset(i_in as isize)
                                    }).omit = 1 as u8
                    };
                }
                v /= 5;
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                unsafe { (*p_info_1).idx_num += 4 };
            }
            unsafe { (*p_info_1).estimated_cost = v as f64 };
        }
        unsafe {
            (*p_info_1).estimated_rows =
                unsafe { (*p_info_1).estimated_cost } as sqlite3_int64
        };
        if unsafe { (*p_info_1).n_order_by } > 0 && flags & 2 == 0 {
            let mut e_distinct: i32 =
                unsafe {
                    (unsafe { (*sqlite3_api).vtab_distinct.unwrap() })(p_info_1)
                };
            if unsafe { (*p_info_1).idx_num } == 1 {
                unsafe { (*p_info_1).order_by_consumed = 1 };
            } else if unsafe {
                        (*unsafe {
                                    (*p_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } <= 0 {
                if unsafe {
                            (*unsafe { (*p_info_1).a_order_by.offset(0 as isize) }).desc
                        } != 0 {
                    unsafe { (*p_info_1).idx_num += 1000 };
                }
                unsafe { (*p_info_1).order_by_consumed = 1 };
            } else if e_distinct >= 1 {
                let mut x: u32 = 0 as u32;
                let mut n_desc: i32 = 0;
                let mut n_asc: i32 = 0;
                {
                    i = 0;
                    '__b11: loop {
                        if !(i < unsafe { (*p_info_1).n_order_by }) {
                            break '__b11;
                        }
                        '__c11: loop {
                            let mut i_col: i32 =
                                unsafe {
                                    (*unsafe {
                                                (*p_info_1).a_order_by.offset(i as isize)
                                            }).i_column
                                };
                            if i_col < 0 { i_col = 0; }
                            if unsafe {
                                        (*unsafe { (*p_info_1).a_order_by.offset(i as isize) }).desc
                                    } != 0 {
                                { let __p = &mut n_desc; let __t = *__p; *__p += 1; __t };
                            } else {
                                { let __p = &mut n_asc; let __t = *__p; *__p += 1; __t };
                            }
                            x |= (1 << i_col) as u32;
                            break '__c11;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if n_desc > 0 && n_asc > 0 {
                    if e_distinct != 1 { e_distinct = -999; }
                } else if n_asc == 0 {
                    unsafe { (*p_info_1).idx_num += 1000 };
                }
                if e_distinct >= 2 && flags & 256 != 0 { e_distinct = 1; }
                if e_distinct >= 2 {
                    if x == 2 as u32 {
                        unsafe {
                            (*p_info_1).idx_num +=
                                if flags & 128 != 0 { 20 } else { 30 }
                        };
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x == 6 as u32 {
                        unsafe {
                            (*p_info_1).idx_num +=
                                if flags & 128 != 0 { 10 } else { 20 }
                        };
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x == 14 as u32 {
                        unsafe {
                            (*p_info_1).idx_num += if flags & 128 != 0 { 0 } else { 10 }
                        };
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x & 1 as u32 != 0 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x == 30 as u32 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    }
                } else if e_distinct == 1 {
                    if x == 2 as u32 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x == 6 as u32 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x == 14 as u32 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x & 1 as u32 != 0 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    } else if x == 30 as u32 {
                        unsafe { (*p_info_1).order_by_consumed = 1 };
                    }
                }
            } else {
                let mut n_desc_1: i32 = 0;
                let mut n_asc_1: i32 = 0;
                {
                    i = 0;
                    '__b12: loop {
                        if !(i < unsafe { (*p_info_1).n_order_by }) {
                            break '__b12;
                        }
                        '__c12: loop {
                            if unsafe {
                                        (*unsafe {
                                                    (*p_info_1).a_order_by.offset(i as isize)
                                                }).i_column
                                    } != i + 1 {
                                break '__b12;
                            }
                            if unsafe {
                                        (*unsafe { (*p_info_1).a_order_by.offset(i as isize) }).desc
                                    } != 0 {
                                { let __p = &mut n_desc_1; let __t = *__p; *__p += 1; __t };
                            } else {
                                { let __p = &mut n_asc_1; let __t = *__p; *__p += 1; __t };
                            }
                            break '__c12;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if i == unsafe { (*p_info_1).n_order_by } &&
                        (n_desc_1 == 0 || n_asc_1 == 0) {
                    unsafe { (*p_info_1).order_by_consumed = 1 };
                    if n_desc_1 != 0 { unsafe { (*p_info_1).idx_num += 1000 }; }
                }
            }
        }
        if flags & 8 != 0 {
            unsafe {
                (*p_info_1).idx_str =
                    unsafe {
                        (unsafe {
                                (*sqlite3_api).mprintf.unwrap()
                            })(c"test".as_ptr() as *mut i8 as *const i8)
                    }
            };
            unsafe { (*p_info_1).need_to_free_idx_str = 1 };
        }
        if flags & 16 != 0 { unsafe { (*p_info_1).idx_num += 10000 }; }
        if i_offset >= 0 {
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_offset as isize)
                            }).argv_index =
                    { let __p = &mut argv_index; *__p += 1; *__p }
            };
            if flags & 4 == 0 &&
                    (unsafe { (*p_info_1).n_order_by } == 0 ||
                        unsafe { (*p_info_1).order_by_consumed } != 0) {
                unsafe {
                    (*unsafe {
                                    (*p_info_1).a_constraint_usage.offset(i_offset as isize)
                                }).omit = 1 as u8
                };
                unsafe { (*p_info_1).idx_num += 100 };
            }
        }
        if i_flag_term >= 0 {
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_flag_term as isize)
                            }).omit = 1 as u8
            };
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_flag_term as isize)
                            }).argv_index =
                    { let __p = &mut argv_index; *__p += 1; *__p }
            };
        }
        if i_log_term >= 0 {
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_log_term as isize)
                            }).omit = 1 as u8
            };
            unsafe {
                (*unsafe {
                                (*p_info_1).a_constraint_usage.offset(i_log_term as isize)
                            }).argv_index =
                    { let __p = &mut argv_index; *__p += 1; *__p }
            };
        }
        if flags & 64 != 0 {
            {
                i = 0;
                '__b13: loop {
                    if !(i < unsafe { (*p_info_1).n_constraint }) {
                        break '__b13;
                    }
                    '__c13: loop {
                        if unsafe {
                                        (*unsafe {
                                                    (*p_info_1).a_constraint.offset(i as isize)
                                                }).usable
                                    } != 0 &&
                                unsafe {
                                        (*unsafe {
                                                    (*p_info_1).a_constraint_usage.offset(i as isize)
                                                }).argv_index
                                    } == 0 {
                            unsafe {
                                (*unsafe {
                                                (*p_info_1).a_constraint_usage.offset(i as isize)
                                            }).argv_index =
                                    { let __p = &mut argv_index; *__p += 1; *__p }
                            };
                            if flags & 32 != 0 {
                                unsafe {
                                    (*unsafe {
                                                    (*p_info_1).a_constraint_usage.offset(i as isize)
                                                }).omit = 1 as u8
                                };
                            }
                        }
                        break '__c13;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if !(z_log_tab).is_null() {
            let db: *mut sqlite3 =
                unsafe { (*(p_v_tab_1 as *mut vt02_vtab)).db };
            sqlite3_best_index_log(p_info_1, z_log_tab, db,
                &raw mut az_colname[0 as usize] as *mut *const i8 as
                    *const *const i8, p_v_tab_1);
        }
        {
            let __p = unsafe { &mut (*p_self).busy };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        p_x = unsafe { (unsafe { (*sqlite3_api).malloc.unwrap() })(800) };
        if p_x == core::ptr::null_mut() { return 7; }
        unsafe { (unsafe { (*sqlite3_api).free.unwrap() })(p_x) };
        return if unsafe { (*p_v_tab_1).z_err_msg } != core::ptr::null_mut() {
                1
            } else { 0 };
    }
}
static vt02_module: sqlite3_module =
    sqlite3_module {
        i_version: 2,
        x_create: None,
        x_connect: Some(vt02_connect),
        x_best_index: Some(vt02_best_index),
        x_disconnect: Some(vt02_disconnect),
        x_destroy: Some(vt02_disconnect),
        x_open: Some(vt02_open),
        x_close: Some(vt02_close),
        x_filter: Some(vt02_filter),
        x_next: Some(vt02_next),
        x_eof: Some(vt02_eof),
        x_column: Some(vt02_column),
        x_rowid: Some(vt02_rowid),
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
extern "C" fn vt02_core_init(db: *mut sqlite3) -> () {
    unsafe {
        unsafe {
            (unsafe {
                    (*sqlite3_api).create_module.unwrap()
                })(db, c"vt02".as_ptr() as *mut i8 as *const i8, &vt02_module,
                core::ptr::null_mut())
        };
        unsafe {
            (unsafe {
                    (*sqlite3_api).create_module.unwrap()
                })(db, c"vt02pkx".as_ptr() as *mut i8 as *const i8,
                &vt02_module, &raw const z_pk_x_schema[0 as usize] as *mut ())
        };
        unsafe {
            (unsafe {
                    (*sqlite3_api).create_module.unwrap()
                })(db, c"vt02pkabcd".as_ptr() as *mut i8 as *const i8,
                &vt02_module,
                &raw const z_pk_abcd_schema[0 as usize] as *mut ())
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vt02_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    unsafe { sqlite3_api = p_api_1; vt02_core_init(db); return 0; }
}
static z_default_schema: [i8; 89] =
    [67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            120 as i8, 40 as i8, 120 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 44 as i8, 32 as i8, 97 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 44 as i8, 32 as i8, 98 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 44 as i8, 32 as i8, 99 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 44 as i8, 32 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 44 as i8,
            32 as i8, 102 as i8, 108 as i8, 97 as i8, 103 as i8, 115 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 32 as i8, 72 as i8,
            73 as i8, 68 as i8, 68 as i8, 69 as i8, 78 as i8, 44 as i8,
            32 as i8, 108 as i8, 111 as i8, 103 as i8, 116 as i8, 97 as i8,
            98 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            32 as i8, 72 as i8, 73 as i8, 68 as i8, 68 as i8, 69 as i8,
            78 as i8, 41 as i8, 59 as i8, 0 as i8];
static i_divisor: [i32; 5] = [1, 1000, 100, 10, 1];
static mut az_colname: [*const i8; 7] =
    [c"x".as_ptr() as *const i8, c"a".as_ptr() as *const i8,
            c"b".as_ptr() as *const i8, c"c".as_ptr() as *const i8,
            c"d".as_ptr() as *const i8, c"flags".as_ptr() as *const i8,
            c"logtab".as_ptr() as *const i8];
static z_pk_x_schema: [i8; 110] =
    [67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            120 as i8, 40 as i8, 120 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 32 as i8, 78 as i8, 79 as i8, 84 as i8, 32 as i8,
            78 as i8, 85 as i8, 76 as i8, 76 as i8, 32 as i8, 80 as i8,
            82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8,
            32 as i8, 75 as i8, 69 as i8, 89 as i8, 44 as i8, 32 as i8,
            97 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 44 as i8,
            32 as i8, 98 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            44 as i8, 32 as i8, 99 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 44 as i8, 32 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 44 as i8, 32 as i8, 102 as i8, 108 as i8,
            97 as i8, 103 as i8, 115 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 32 as i8, 72 as i8, 73 as i8, 68 as i8, 68 as i8,
            69 as i8, 78 as i8, 44 as i8, 32 as i8, 108 as i8, 111 as i8,
            103 as i8, 116 as i8, 97 as i8, 98 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 32 as i8, 72 as i8, 73 as i8,
            68 as i8, 68 as i8, 69 as i8, 78 as i8, 41 as i8, 59 as i8,
            0 as i8];
static z_pk_abcd_schema: [i8; 147] =
    [67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            120 as i8, 40 as i8, 120 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 44 as i8, 32 as i8, 97 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 32 as i8, 78 as i8, 79 as i8, 84 as i8,
            32 as i8, 78 as i8, 85 as i8, 76 as i8, 76 as i8, 44 as i8,
            32 as i8, 98 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            32 as i8, 78 as i8, 79 as i8, 84 as i8, 32 as i8, 78 as i8,
            85 as i8, 76 as i8, 76 as i8, 44 as i8, 32 as i8, 99 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 32 as i8, 78 as i8,
            79 as i8, 84 as i8, 32 as i8, 78 as i8, 85 as i8, 76 as i8,
            76 as i8, 44 as i8, 32 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 32 as i8, 78 as i8, 79 as i8, 84 as i8,
            32 as i8, 78 as i8, 85 as i8, 76 as i8, 76 as i8, 44 as i8,
            32 as i8, 102 as i8, 108 as i8, 97 as i8, 103 as i8, 115 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 32 as i8, 72 as i8,
            73 as i8, 68 as i8, 68 as i8, 69 as i8, 78 as i8, 44 as i8,
            32 as i8, 108 as i8, 111 as i8, 103 as i8, 116 as i8, 97 as i8,
            98 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            32 as i8, 72 as i8, 73 as i8, 68 as i8, 68 as i8, 69 as i8,
            78 as i8, 44 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8,
            77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8,
            69 as i8, 89 as i8, 40 as i8, 97 as i8, 44 as i8, 98 as i8,
            44 as i8, 99 as i8, 44 as i8, 100 as i8, 41 as i8, 41 as i8,
            59 as i8, 0 as i8];
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn __builtin_unreachable()
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}