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
struct Decimal {
    sign: i8,
    oom: i8,
    is_null: i8,
    is_init: i8,
    n_digit: i32,
    n_frac: i32,
    a: *mut i8,
}
extern "C" fn decimal_clear(p: &Decimal) -> () {
    unsafe { sqlite3_free((*p).a as *mut ()) };
}
extern "C" fn decimal_free(p: *mut Decimal) -> () {
    if !(p).is_null() {
        decimal_clear(unsafe { &*p });
        unsafe { sqlite3_free(p as *mut ()) };
    }
}
extern "C" fn decimal_new_from_text(z_in_1: *const i8, n: i32)
    -> *mut Decimal {
    let mut p: *mut Decimal = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut i_exp: i32 = 0;
    let mut c: i8 = 0 as i8;
    let mut j: i32 = 0;
    let mut neg: i32 = 0;
    let mut a: *mut i8 = core::ptr::null_mut();
    let mut n_extra: i32 = 0;
    let mut a__1: *mut i8 = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s1:
            {
            match __state {
                0 => { p = core::ptr::null_mut(); __state = 3; }
                2 => {
                    if !(p).is_null() { __state = 101; } else { __state = 100; }
                }
                3 => { __state = 4; }
                4 => { i_exp = 0; __state = 5; }
                5 => {
                    if z_in_1 == core::ptr::null() {
                        __state = 7;
                    } else { __state = 6; }
                }
                6 => {
                    p =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<Decimal>() as
                                        sqlite3_uint64)
                            } as *mut Decimal;
                    __state = 8;
                }
                7 => { __state = 2; }
                8 => {
                    if p == core::ptr::null_mut() {
                        __state = 10;
                    } else { __state = 9; }
                }
                9 => { unsafe { (*p).sign = 0 as i8 }; __state = 11; }
                10 => { __state = 2; }
                11 => { unsafe { (*p).oom = 0 as i8 }; __state = 12; }
                12 => { unsafe { (*p).is_init = 1 as i8 }; __state = 13; }
                13 => { unsafe { (*p).is_null = 0 as i8 }; __state = 14; }
                14 => { unsafe { (*p).n_digit = 0 }; __state = 15; }
                15 => { unsafe { (*p).n_frac = 0 }; __state = 16; }
                16 => {
                    unsafe {
                        (*p).a =
                            unsafe { sqlite3_malloc64((n + 1) as sqlite3_uint64) } as
                                *mut i8
                    };
                    __state = 17;
                }
                17 => {
                    if unsafe { (*p).a } == core::ptr::null_mut() {
                        __state = 19;
                    } else { __state = 18; }
                }
                18 => { i = 0; __state = 21; }
                19 => { __state = 2; }
                20 => {
                    if i < n &&
                            unsafe { *z_in_1.offset(i as isize) } as i32 == '-' as i32 {
                        __state = 25;
                    } else { __state = 26; }
                }
                21 => {
                    if i < n &&
                            unsafe {
                                    isspace(unsafe { *z_in_1.offset(i as isize) } as u8 as i32)
                                } != 0 {
                        __state = 22;
                    } else { __state = 20; }
                }
                22 => { __state = 23; }
                23 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 21;
                }
                24 => {
                    if i < n &&
                            unsafe { *z_in_1.offset(i as isize) } as i32 == '0' as i32 {
                        __state = 30;
                    } else { __state = 29; }
                }
                25 => { unsafe { (*p).sign = 1 as i8 }; __state = 27; }
                26 => {
                    if i < n &&
                            unsafe { *z_in_1.offset(i as isize) } as i32 == '+' as i32 {
                        __state = 28;
                    } else { __state = 24; }
                }
                27 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 24;
                }
                28 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 24;
                }
                29 => { if i < n { __state = 32; } else { __state = 31; } }
                30 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 24;
                }
                31 => {
                    if unsafe { (*p).n_frac } != 0 {
                        __state = 56;
                    } else { __state = 55; }
                }
                32 => {
                    c = unsafe { *z_in_1.offset(i as isize) } as i8;
                    __state = 33;
                }
                33 => {
                    if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
                        __state = 35;
                    } else { __state = 36; }
                }
                34 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 29;
                }
                35 => {
                    unsafe {
                        *unsafe {
                                    (*p).a.offset({
                                                let __p = unsafe { &mut (*p).n_digit };
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                                } = (c as i32 - '0' as i32) as i8
                    };
                    __state = 34;
                }
                36 => {
                    if c as i32 == '.' as i32 {
                        __state = 37;
                    } else { __state = 38; }
                }
                37 => {
                    unsafe { (*p).n_frac = unsafe { (*p).n_digit } + 1 };
                    __state = 34;
                }
                38 => {
                    if c as i32 == 'e' as i32 || c as i32 == 'E' as i32 {
                        __state = 39;
                    } else { __state = 34; }
                }
                39 => { j = i + 1; __state = 40; }
                40 => { neg = 0; __state = 41; }
                41 => { if j >= n { __state = 43; } else { __state = 42; } }
                42 => {
                    if unsafe { *z_in_1.offset(j as isize) } as i32 ==
                            '-' as i32 {
                        __state = 45;
                    } else { __state = 46; }
                }
                43 => { __state = 31; }
                44 => {
                    if j < n && i_exp < 1000000 {
                        __state = 50;
                    } else { __state = 49; }
                }
                45 => { neg = 1; __state = 47; }
                46 => {
                    if unsafe { *z_in_1.offset(j as isize) } as i32 ==
                            '+' as i32 {
                        __state = 48;
                    } else { __state = 44; }
                }
                47 => {
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    __state = 44;
                }
                48 => {
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    __state = 44;
                }
                49 => { if neg != 0 { __state = 54; } else { __state = 53; } }
                50 => {
                    if unsafe { *z_in_1.offset(j as isize) } as i32 >=
                                '0' as i32 &&
                            unsafe { *z_in_1.offset(j as isize) } as i32 <= '9' as i32 {
                        __state = 52;
                    } else { __state = 51; }
                }
                51 => {
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    __state = 44;
                }
                52 => {
                    i_exp =
                        i_exp * 10 + unsafe { *z_in_1.offset(j as isize) } as i32 -
                            '0' as i32;
                    __state = 51;
                }
                53 => { __state = 31; }
                54 => { i_exp = -i_exp; __state = 53; }
                55 => {
                    if i_exp > 0 { __state = 58; } else { __state = 59; }
                }
                56 => {
                    unsafe {
                        (*p).n_frac =
                            unsafe { (*p).n_digit } - (unsafe { (*p).n_frac } - 1)
                    };
                    __state = 55;
                }
                57 => {
                    if unsafe { (*p).sign } != 0 {
                        __state = 91;
                    } else { __state = 90; }
                }
                58 => {
                    if unsafe { (*p).n_frac } > 0 {
                        __state = 61;
                    } else { __state = 60; }
                }
                59 => {
                    if i_exp < 0 { __state = 72; } else { __state = 57; }
                }
                60 => {
                    if i_exp > 0 { __state = 66; } else { __state = 57; }
                }
                61 => {
                    if i_exp <= unsafe { (*p).n_frac } {
                        __state = 62;
                    } else { __state = 63; }
                }
                62 => { unsafe { (*p).n_frac -= i_exp }; __state = 64; }
                63 => { i_exp -= unsafe { (*p).n_frac }; __state = 65; }
                64 => { i_exp = 0; __state = 60; }
                65 => { unsafe { (*p).n_frac = 0 }; __state = 60; }
                66 => {
                    a =
                        unsafe {
                                sqlite3_realloc64(unsafe { (*p).a } as *mut (),
                                    (unsafe { (*p).n_digit } as sqlite3_int64 +
                                                i_exp as sqlite3_int64 + 1 as sqlite3_int64) as
                                        sqlite3_uint64)
                            } as *mut i8;
                    __state = 67;
                }
                67 => {
                    if a == core::ptr::null_mut() {
                        __state = 69;
                    } else { __state = 68; }
                }
                68 => { unsafe { (*p).a = a }; __state = 70; }
                69 => { __state = 2; }
                70 => {
                    unsafe {
                        memset(unsafe {
                                    unsafe { (*p).a.offset(unsafe { (*p).n_digit } as isize) }
                                } as *mut (), 0, i_exp as u64)
                    };
                    __state = 71;
                }
                71 => { unsafe { (*p).n_digit += i_exp }; __state = 57; }
                72 => { __state = 73; }
                73 => { i_exp = -i_exp; __state = 74; }
                74 => {
                    n_extra =
                        unsafe { (*p).n_digit } - unsafe { (*p).n_frac } - 1;
                    __state = 75;
                }
                75 => {
                    if n_extra != 0 { __state = 77; } else { __state = 76; }
                }
                76 => {
                    if i_exp > 0 { __state = 82; } else { __state = 57; }
                }
                77 => {
                    if n_extra >= i_exp { __state = 78; } else { __state = 79; }
                }
                78 => { unsafe { (*p).n_frac += i_exp }; __state = 80; }
                79 => { i_exp -= n_extra; __state = 81; }
                80 => { i_exp = 0; __state = 76; }
                81 => {
                    unsafe { (*p).n_frac = unsafe { (*p).n_digit } - 1 };
                    __state = 76;
                }
                82 => {
                    a__1 =
                        unsafe {
                                sqlite3_realloc64(unsafe { (*p).a } as *mut (),
                                    (unsafe { (*p).n_digit } as sqlite3_int64 +
                                                i_exp as sqlite3_int64 + 1 as sqlite3_int64) as
                                        sqlite3_uint64)
                            } as *mut i8;
                    __state = 83;
                }
                83 => {
                    if a__1 == core::ptr::null_mut() {
                        __state = 85;
                    } else { __state = 84; }
                }
                84 => { unsafe { (*p).a = a__1 }; __state = 86; }
                85 => { __state = 2; }
                86 => {
                    unsafe {
                        memmove(unsafe { unsafe { (*p).a.offset(i_exp as isize) } }
                                as *mut (), unsafe { (*p).a } as *const (),
                            unsafe { (*p).n_digit } as u64)
                    };
                    __state = 87;
                }
                87 => {
                    unsafe {
                        memset(unsafe { (*p).a } as *mut (), 0, i_exp as u64)
                    };
                    __state = 88;
                }
                88 => { unsafe { (*p).n_digit += i_exp }; __state = 89; }
                89 => { unsafe { (*p).n_frac += i_exp }; __state = 57; }
                90 => {
                    if unsafe { (*p).n_digit } > 10000000 {
                        __state = 98;
                    } else { __state = 97; }
                }
                91 => { i = 0; __state = 93; }
                92 => {
                    if i >= unsafe { (*p).n_digit } {
                        __state = 96;
                    } else { __state = 90; }
                }
                93 => {
                    if i < unsafe { (*p).n_digit } &&
                            unsafe { *unsafe { (*p).a.offset(i as isize) } } as i32 == 0
                        {
                        __state = 94;
                    } else { __state = 92; }
                }
                94 => { __state = 95; }
                95 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 93;
                }
                96 => { unsafe { (*p).sign = 0 as i8 }; __state = 90; }
                97 => { return p; }
                98 => { __state = 2; }
                99 => { __state = 2; }
                100 => { return core::ptr::null_mut(); }
                101 => {
                    if !(unsafe { (*p).a }).is_null() {
                        __state = 103;
                    } else { __state = 102; }
                }
                102 => {
                    unsafe { sqlite3_free(p as *mut ()) };
                    __state = 100;
                }
                103 => {
                    unsafe { sqlite3_free(unsafe { (*p).a } as *mut ()) };
                    __state = 102;
                }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn decimal_mul(p_a_1: *mut Decimal, p_b_1: *const Decimal) -> () {
    let mut acc: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut min_frac: i32 = 0;
    let mut sum_digit: sqlite3_int64 = 0 as sqlite3_int64;
    let mut f: i8 = 0 as i8;
    let mut carry: i32 = 0;
    let mut x: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s3:
            {
            match __state {
                0 => { acc = core::ptr::null_mut(); __state = 3; }
                2 => { unsafe { sqlite3_free(acc as *mut ()) }; __state = 1; }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => {
                    if p_a_1 == core::ptr::null_mut() ||
                                            unsafe { (*p_a_1).oom } != 0 ||
                                        unsafe { (*p_a_1).is_null } != 0 ||
                                    p_b_1 == core::ptr::null_mut() ||
                                unsafe { (*p_b_1).oom } != 0 ||
                            unsafe { (*p_b_1).is_null } != 0 {
                        __state = 8;
                    } else { __state = 7; }
                }
                7 => {
                    sum_digit = unsafe { (*p_a_1).n_digit } as sqlite3_int64;
                    __state = 9;
                }
                8 => { __state = 2; }
                9 => {
                    sum_digit += unsafe { (*p_b_1).n_digit } as sqlite3_int64;
                    __state = 10;
                }
                10 => { sum_digit += 2 as sqlite3_int64; __state = 11; }
                11 => {
                    if sum_digit > 10000000 as i64 {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => {
                    acc =
                        unsafe { sqlite3_malloc64(sum_digit as sqlite3_uint64) } as
                            *mut i8;
                    __state = 15;
                }
                13 => { unsafe { (*p_a_1).oom = 1 as i8 }; __state = 14; }
                14 => { return; }
                15 => {
                    if acc == core::ptr::null_mut() {
                        __state = 17;
                    } else { __state = 16; }
                }
                16 => {
                    unsafe {
                        memset(acc as *mut (), 0,
                            (unsafe { (*p_a_1).n_digit } + unsafe { (*p_b_1).n_digit } +
                                    2) as u64)
                    };
                    __state = 19;
                }
                17 => { unsafe { (*p_a_1).oom = 1 as i8 }; __state = 18; }
                18 => { __state = 2; }
                19 => { min_frac = unsafe { (*p_a_1).n_frac }; __state = 20; }
                20 => {
                    if unsafe { (*p_b_1).n_frac } < min_frac {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => { i = unsafe { (*p_a_1).n_digit } - 1; __state = 24; }
                22 => { min_frac = unsafe { (*p_b_1).n_frac }; __state = 21; }
                23 => {
                    unsafe { sqlite3_free(unsafe { (*p_a_1).a } as *mut ()) };
                    __state = 37;
                }
                24 => { if i >= 0 { __state = 25; } else { __state = 23; } }
                25 => {
                    f = unsafe { *unsafe { (*p_a_1).a.offset(i as isize) } };
                    __state = 27;
                }
                26 => {
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                    __state = 24;
                }
                27 => { carry = 0; __state = 28; }
                28 => {
                    { j = unsafe { (*p_b_1).n_digit } - 1; k = i + j + 3 };
                    __state = 30;
                }
                29 => {
                    x = unsafe { *acc.offset(k as isize) } as i32 + carry;
                    __state = 35;
                }
                30 => { if j >= 0 { __state = 31; } else { __state = 29; } }
                31 => {
                    x =
                        unsafe { *acc.offset(k as isize) } as i32 +
                                f as i32 *
                                    unsafe { *unsafe { (*p_b_1).a.offset(j as isize) } } as i32
                            + carry;
                    __state = 33;
                }
                32 => {
                    {
                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        { let __p = &mut k; let __t = *__p; *__p -= 1; __t }
                    };
                    __state = 30;
                }
                33 => {
                    unsafe { *acc.offset(k as isize) = (x % 10) as i8 };
                    __state = 34;
                }
                34 => { carry = x / 10; __state = 32; }
                35 => {
                    unsafe { *acc.offset(k as isize) = (x % 10) as i8 };
                    __state = 36;
                }
                36 => {
                    unsafe { *acc.offset((k - 1) as isize) += (x / 10) as i8 };
                    __state = 26;
                }
                37 => { unsafe { (*p_a_1).a = acc }; __state = 38; }
                38 => { acc = core::ptr::null_mut(); __state = 39; }
                39 => {
                    unsafe {
                        (*p_a_1).n_digit += unsafe { (*p_b_1).n_digit } + 2
                    };
                    __state = 40;
                }
                40 => {
                    unsafe { (*p_a_1).n_frac += unsafe { (*p_b_1).n_frac } };
                    __state = 41;
                }
                41 => {
                    unsafe {
                        (*p_a_1).sign ^= unsafe { (*p_b_1).sign } as i32 as i8
                    };
                    __state = 42;
                }
                42 => {
                    if unsafe { (*p_a_1).n_frac } > min_frac &&
                            unsafe {
                                        *unsafe {
                                                (*p_a_1).a.offset((unsafe { (*p_a_1).n_digit } - 1) as
                                                        isize)
                                            }
                                    } as i32 == 0 {
                        __state = 44;
                    } else { __state = 43; }
                }
                43 => { __state = 2; }
                44 => {
                    {
                        let __p = unsafe { &mut (*p_a_1).n_frac };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    __state = 45;
                }
                45 => {
                    {
                        let __p = unsafe { &mut (*p_a_1).n_digit };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    __state = 42;
                }
                _ => {}
            }
        }
    }
}
extern "C" fn decimal_pow2(mut n_1: i32) -> *mut Decimal {
    let mut p_a: *mut Decimal = core::ptr::null_mut();
    let mut p_x: *mut Decimal = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s5:
            {
            match __state {
                0 => { p_a = core::ptr::null_mut(); __state = 3; }
                2 => { decimal_free(p_a); __state = 29; }
                3 => { p_x = core::ptr::null_mut(); __state = 4; }
                4 => {
                    if n_1 < -20000 || n_1 > 20000 {
                        __state = 6;
                    } else { __state = 5; }
                }
                5 => {
                    p_a =
                        decimal_new_from_text(c"1.0".as_ptr() as *mut i8 as
                                *const i8, 3);
                    __state = 7;
                }
                6 => { __state = 2; }
                7 => {
                    if p_a == core::ptr::null_mut() ||
                            unsafe { (*p_a).oom } != 0 {
                        __state = 9;
                    } else { __state = 8; }
                }
                8 => { if n_1 == 0 { __state = 11; } else { __state = 10; } }
                9 => { __state = 2; }
                10 => { if n_1 > 0 { __state = 13; } else { __state = 14; } }
                11 => { return p_a; }
                12 => {
                    if p_x == core::ptr::null_mut() ||
                            unsafe { (*p_x).oom } != 0 {
                        __state = 17;
                    } else { __state = 16; }
                }
                13 => {
                    p_x =
                        decimal_new_from_text(c"2.0".as_ptr() as *mut i8 as
                                *const i8, 3);
                    __state = 12;
                }
                14 => { n_1 = -n_1; __state = 15; }
                15 => {
                    p_x =
                        decimal_new_from_text(c"0.5".as_ptr() as *mut i8 as
                                *const i8, 3);
                    __state = 12;
                }
                16 => { if 1 != 0 { __state = 19; } else { __state = 18; } }
                17 => { __state = 2; }
                18 => { decimal_free(p_x); __state = 27; }
                19 => {
                    if n_1 & 1 != 0 { __state = 21; } else { __state = 20; }
                }
                20 => { n_1 >>= 1; __state = 24; }
                21 => {
                    decimal_mul(p_a, p_x as *const Decimal);
                    __state = 22;
                }
                22 => {
                    if unsafe { (*p_a).oom } != 0 {
                        __state = 23;
                    } else { __state = 20; }
                }
                23 => { __state = 2; }
                24 => { if n_1 == 0 { __state = 26; } else { __state = 25; } }
                25 => {
                    decimal_mul(p_x, p_x as *const Decimal);
                    __state = 16;
                }
                26 => { __state = 18; }
                27 => { return p_a; }
                28 => { __state = 2; }
                29 => { decimal_free(p_x); __state = 30; }
                30 => { return core::ptr::null_mut(); }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn decimal_from_double(mut r: f64) -> *mut Decimal {
    let mut m: sqlite3_int64 = 0 as sqlite3_int64;
    let mut a: sqlite3_int64 = 0 as sqlite3_int64;
    let mut e: i32 = 0;
    let mut is_neg: i32 = 0;
    let mut p_a: *mut Decimal = core::ptr::null_mut();
    let mut p_x: *mut Decimal = core::ptr::null_mut();
    let mut z_num: [i8; 100] = [0; 100];
    if r < 0.0 { is_neg = 1; r = -r; } else { is_neg = 0; }
    unsafe {
        memcpy(&raw mut a as *mut (), &raw mut r as *const (),
            core::mem::size_of::<sqlite3_int64>() as u64)
    };
    if a == 0 as i64 || a == 9223372036854775808u64 as sqlite3_int64 {
        e = 0;
        m = 0 as sqlite3_int64;
    } else {
        e = (a >> 52) as i32;
        m = a & ((1 as sqlite3_int64) << 52) - 1 as sqlite3_int64;
        if e == 0 {
            m <<= 1 as sqlite3_int64;
        } else { m |= (1 as sqlite3_int64) << 52; }
        while e < 1075 && m > 0 as i64 && m & 1 as sqlite3_int64 == 0 as i64 {
            m >>= 1 as sqlite3_int64;
            { let __p = &mut e; let __t = *__p; *__p += 1; __t };
        }
        if is_neg != 0 { m = -m; }
        e = e - 1075;
        if e > 971 { return core::ptr::null_mut(); }
    }
    unsafe {
        sqlite3_snprintf(core::mem::size_of::<[i8; 100]>() as i32,
            &raw mut z_num[0 as usize] as *mut i8,
            c"%lld".as_ptr() as *mut i8 as *const i8, m)
    };
    p_a =
        decimal_new_from_text(&raw mut z_num[0 as usize] as *mut i8 as
                *const i8,
            unsafe {
                    strlen(&raw mut z_num[0 as usize] as *mut i8 as *const i8)
                } as i32);
    p_x = decimal_pow2(e);
    decimal_mul(p_a, p_x as *const Decimal);
    decimal_free(p_x);
    return p_a;
}
extern "C" fn decimal_new(p_ctx_1: *mut sqlite3_context,
    p_in_1: *mut sqlite3_value, b_text_only_1: i32) -> *mut Decimal {
    let mut p: *mut Decimal = core::ptr::null_mut();
    let mut e_type: i32 = 0;
    let mut z_in: *const i8 = core::ptr::null();
    let mut n: i32 = 0;
    let mut x: *const u8 = core::ptr::null();
    let mut i: u32 = 0 as u32;
    let mut v: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut r: f64 = 0.0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s8:
            {
            match __state {
                0 => { p = core::ptr::null_mut(); __state = 3; }
                2 => {
                    if !(p_ctx_1).is_null() {
                        __state = 39;
                    } else { __state = 38; }
                }
                3 => {
                    e_type = unsafe { sqlite3_value_type(p_in_1) };
                    __state = 4;
                }
                4 => {
                    if b_text_only_1 != 0 && (e_type == 2 || e_type == 4) {
                        __state = 6;
                    } else { __state = 5; }
                }
                5 => {
                    '__s9:
                        {
                        match e_type {
                            3 => { __state = 8; }
                            1 => { __state = 9; }
                            2 => { __state = 10; }
                            4 => { __state = 11; }
                            5 => { __state = 12; }
                            _ => { __state = 7; }
                        }
                    }
                }
                6 => { e_type = 3; __state = 5; }
                7 => { return p; }
                8 => { __state = 9; }
                9 => {
                    z_in = unsafe { sqlite3_value_text(p_in_1) } as *const i8;
                    __state = 15;
                }
                10 => {
                    p =
                        decimal_from_double(unsafe {
                                sqlite3_value_double(p_in_1)
                            });
                    __state = 22;
                }
                11 => { __state = 24; }
                12 => { __state = 7; }
                13 => { __state = 8; }
                14 => { __state = 20; }
                15 => {
                    n = unsafe { sqlite3_value_bytes(p_in_1) };
                    __state = 16;
                }
                16 => { p = decimal_new_from_text(z_in, n); __state = 17; }
                17 => {
                    if p == core::ptr::null_mut() {
                        __state = 19;
                    } else { __state = 18; }
                }
                18 => { __state = 7; }
                19 => { __state = 2; }
                20 => { __state = 10; }
                21 => { __state = 11; }
                22 => { __state = 7; }
                23 => { __state = 12; }
                24 => { __state = 25; }
                25 => { v = 0 as sqlite3_uint64; __state = 26; }
                26 => { __state = 27; }
                27 => {
                    if unsafe { sqlite3_value_bytes(p_in_1) } as u64 !=
                            core::mem::size_of::<f64>() as u64 {
                        __state = 29;
                    } else { __state = 28; }
                }
                28 => {
                    x = unsafe { sqlite3_value_blob(p_in_1) } as *const u8;
                    __state = 30;
                }
                29 => { __state = 7; }
                30 => { i = 0 as u32; __state = 32; }
                31 => {
                    unsafe {
                        memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                            core::mem::size_of::<f64>() as u64)
                    };
                    __state = 35;
                }
                32 => {
                    if (i as u64) < core::mem::size_of::<f64>() as u64 {
                        __state = 33;
                    } else { __state = 31; }
                }
                33 => {
                    v =
                        v << 8 | unsafe { *x.add(i as usize) } as sqlite3_uint64;
                    __state = 34;
                }
                34 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 32;
                }
                35 => { p = decimal_from_double(r); __state = 36; }
                36 => { __state = 7; }
                37 => { __state = 2; }
                38 => { unsafe { sqlite3_free(p as *mut ()) }; __state = 40; }
                39 => {
                    unsafe { sqlite3_result_error_nomem(p_ctx_1) };
                    __state = 38;
                }
                40 => { return core::ptr::null_mut(); }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn decimal_result(p_ctx_1: *mut sqlite3_context, p: *mut Decimal)
    -> () {
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    if p == core::ptr::null_mut() || unsafe { (*p).oom } != 0 {
        unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        return;
    }
    if unsafe { (*p).is_null } != 0 {
        unsafe { sqlite3_result_null(p_ctx_1) };
        return;
    }
    z =
        unsafe {
                sqlite3_malloc64((unsafe { (*p).n_digit } as sqlite3_int64 +
                            8 as sqlite3_int64) as sqlite3_uint64)
            } as *mut i8;
    if z == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        return;
    }
    i = 0;
    if unsafe { (*p).n_digit } == 0 ||
            unsafe { (*p).n_digit } == 1 &&
                unsafe { *unsafe { (*p).a.offset(0 as isize) } } as i32 == 0 {
        unsafe { (*p).sign = 0 as i8 };
    }
    if unsafe { (*p).sign } != 0 {
        unsafe { *z.offset(0 as isize) = '-' as i32 as i8 };
        i = 1;
    }
    n = unsafe { (*p).n_digit } - unsafe { (*p).n_frac };
    if n <= 0 {
        unsafe {
            *z.offset({ let __p = &mut i; let __t = *__p; *__p += 1; __t } as
                            isize) = '0' as i32 as i8
        };
    }
    j = 0;
    while n > 1 &&
            unsafe { *unsafe { (*p).a.offset(j as isize) } } as i32 == 0 {
        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    while n > 0 {
        unsafe {
            *z.offset({ let __p = &mut i; let __t = *__p; *__p += 1; __t } as
                            isize) =
                (unsafe { *unsafe { (*p).a.offset(j as isize) } } as i32 +
                        '0' as i32) as i8
        };
        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    if unsafe { (*p).n_frac } != 0 {
        unsafe {
            *z.offset({ let __p = &mut i; let __t = *__p; *__p += 1; __t } as
                            isize) = '.' as i32 as i8
        };
        '__b12: loop {
            '__c12: loop {
                unsafe {
                    *z.offset({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) =
                        (unsafe { *unsafe { (*p).a.offset(j as isize) } } as i32 +
                                '0' as i32) as i8
                };
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                break '__c12;
            }
            if !(j < unsafe { (*p).n_digit }) { break '__b12; }
        }
    }
    unsafe { *z.offset(i as isize) = 0 as i8 };
    unsafe {
        sqlite3_result_text(p_ctx_1, z as *const i8, i, Some(sqlite3_free))
    };
}
extern "C" fn decimal_expand(p: *mut Decimal, n_digit: i32, n_frac: i32)
    -> () {
    let mut n_add_sig: i32 = 0;
    let mut n_add_frac: i32 = 0;
    let mut a: *mut i8 = core::ptr::null_mut();
    if p == core::ptr::null_mut() { return; }
    n_add_frac = n_frac - unsafe { (*p).n_frac };
    if !(n_add_frac >= 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"decimal_expand".as_ptr() as *const i8,
                c"decimal.c".as_ptr() as *mut i8 as *const i8, 483,
                c"nAddFrac>=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n_add_sig = n_digit - unsafe { (*p).n_digit } - n_add_frac;
    if n_add_frac == 0 && n_add_sig == 0 { return; }
    if n_digit + 1 > 10000000 { unsafe { (*p).oom = 1 as i8 }; return; }
    a =
        unsafe {
                sqlite3_realloc64(unsafe { (*p).a } as *mut (),
                    (n_digit + 1) as sqlite3_uint64)
            } as *mut i8;
    if a == core::ptr::null_mut() { unsafe { (*p).oom = 1 as i8 }; return; }
    unsafe { (*p).a = a };
    if n_add_sig != 0 {
        unsafe {
            memmove(unsafe { unsafe { (*p).a.offset(n_add_sig as isize) } } as
                    *mut (), unsafe { (*p).a } as *const (),
                unsafe { (*p).n_digit } as u64)
        };
        unsafe { memset(unsafe { (*p).a } as *mut (), 0, n_add_sig as u64) };
        unsafe { (*p).n_digit += n_add_sig };
    }
    if n_add_frac != 0 {
        unsafe {
            memset(unsafe {
                        unsafe { (*p).a.offset(unsafe { (*p).n_digit } as isize) }
                    } as *mut (), 0, n_add_frac as u64)
        };
        unsafe { (*p).n_digit += n_add_frac };
        unsafe { (*p).n_frac += n_add_frac };
    }
}
extern "C" fn decimal_round(p: *mut Decimal, mut n_1: i32) -> () {
    let mut i: i32 = 0;
    let mut n_zero: i32 = 0;
    if n_1 < 1 { return; }
    if p == core::ptr::null_mut() { return; }
    if unsafe { (*p).n_digit } <= n_1 { return; }
    {
        n_zero = 0;
        '__b13: loop {
            if !(n_zero < unsafe { (*p).n_digit } &&
                            unsafe { *unsafe { (*p).a.offset(n_zero as isize) } } as i32
                                == 0) {
                break '__b13;
            }
            '__c13: loop { break '__c13; }
            { let __p = &mut n_zero; let __t = *__p; *__p += 1; __t };
        }
    }
    n_1 += n_zero;
    if unsafe { (*p).n_digit } <= n_1 { return; }
    if unsafe { *unsafe { (*p).a.offset(n_1 as isize) } } as i32 >= 5 {
        {
            i = 0;
            '__b14: loop {
                if !(i < n_1 &&
                                unsafe { *unsafe { (*p).a.offset(i as isize) } } as i32 ==
                                    9) {
                    break '__b14;
                }
                '__c14: loop { break '__c14; }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if i == n_1 {
            decimal_expand(p, unsafe { (*p).n_digit } + 1,
                unsafe { (*p).n_frac });
            if unsafe { (*p).oom } != 0 { return; }
        }
        {
            let __p =
                unsafe { &mut *unsafe { (*p).a.offset((n_1 - 1) as isize) } };
            let __t = *__p;
            *__p += 1;
            __t
        };
        {
            i = n_1 - 1;
            '__b15: loop {
                if !(i > 0 &&
                                unsafe { *unsafe { (*p).a.offset(i as isize) } } as i32 > 9)
                    {
                    break '__b15;
                }
                '__c15: loop {
                    unsafe { *unsafe { (*p).a.offset(i as isize) } = 0 as i8 };
                    {
                        let __p =
                            unsafe { &mut *unsafe { (*p).a.offset((i - 1) as isize) } };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        if !(unsafe { *unsafe { (*p).a.offset(0 as isize) } } as i32 <= 9) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"decimal_round".as_ptr() as *const i8,
                    c"decimal.c".as_ptr() as *mut i8 as *const i8, 335,
                    c"p->a[0]<=9".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    unsafe {
        memset(unsafe { &raw mut *unsafe { (*p).a.offset(n_1 as isize) } } as
                *mut (), 0, (unsafe { (*p).n_digit } - n_1) as u64)
    };
}
extern "C" fn decimal_result_sci(p_ctx_1: *mut sqlite3_context,
    p: *const Decimal, mut n_1: i32) -> () {
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut n_zero: i32 = 0;
    let mut n_digit: i32 = 0;
    let mut n_frac: i32 = 0;
    let mut exp: i32 = 0;
    let mut zero: i8 = 0 as i8;
    let mut a: *const i8 = core::ptr::null();
    if p == core::ptr::null_mut() || unsafe { (*p).oom } != 0 {
        unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        return;
    }
    if unsafe { (*p).is_null } != 0 {
        unsafe { sqlite3_result_null(p_ctx_1) };
        return;
    }
    if n_1 < 1 { n_1 = 0; }
    {
        n_digit = unsafe { (*p).n_digit };
        '__b16: loop {
            if !(n_digit > n_1 &&
                            unsafe { *unsafe { (*p).a.offset((n_digit - 1) as isize) } }
                                    as i32 == 0) {
                break '__b16;
            }
            '__c16: loop { break '__c16; }
            { let __p = &mut n_digit; let __t = *__p; *__p -= 1; __t };
        }
    }
    {
        n_zero = 0;
        '__b17: loop {
            if !(n_zero < n_digit &&
                            unsafe { *unsafe { (*p).a.offset(n_zero as isize) } } as i32
                                == 0) {
                break '__b17;
            }
            '__c17: loop { break '__c17; }
            { let __p = &mut n_zero; let __t = *__p; *__p += 1; __t };
        }
    }
    n_frac = unsafe { (*p).n_frac } + (n_digit - unsafe { (*p).n_digit });
    n_digit -= n_zero;
    z =
        unsafe {
                sqlite3_malloc64((n_digit as sqlite3_int64 +
                            20 as sqlite3_int64) as sqlite3_uint64)
            } as *mut i8;
    if z == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        return;
    }
    if n_digit == 0 {
        zero = 0 as i8;
        a = &mut zero;
        n_digit = 1;
        n_frac = 0;
    } else { a = unsafe { unsafe { (*p).a.offset(n_zero as isize) } }; }
    if unsafe { (*p).sign } != 0 && n_digit > 0 {
        unsafe { *z.offset(0 as isize) = '-' as i32 as i8 };
    } else { unsafe { *z.offset(0 as isize) = '+' as i32 as i8 }; }
    unsafe {
        *z.offset(1 as isize) =
            (unsafe { *a.offset(0 as isize) } as i32 + '0' as i32) as i8
    };
    unsafe { *z.offset(2 as isize) = '.' as i32 as i8 };
    if n_digit == 1 {
        unsafe { *z.offset(3 as isize) = '0' as i32 as i8 };
        i = 4;
    } else {
        {
            i = 1;
            '__b18: loop {
                if !(i < n_digit) { break '__b18; }
                '__c18: loop {
                    unsafe {
                        *z.offset((2 + i) as isize) =
                            (unsafe { *a.offset(i as isize) } as i32 + '0' as i32) as i8
                    };
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        i = n_digit + 2;
    }
    exp = n_digit - n_frac - 1;
    unsafe {
        sqlite3_snprintf(n_digit + 20 - i,
            unsafe { &mut *z.offset(i as isize) },
            c"e%+03d".as_ptr() as *mut i8 as *const i8, exp)
    };
    unsafe {
        sqlite3_result_text(p_ctx_1, z as *const i8, -1, Some(sqlite3_free))
    };
}
extern "C" fn decimal_cmp(mut p_a_1: *mut Decimal, mut p_b_1: *mut Decimal)
    -> i32 {
    let mut n_a_sig: i32 = 0;
    let mut n_b_sig: i32 = 0;
    let mut rc: i32 = 0;
    let mut n: i32 = 0;
    while unsafe { (*p_a_1).n_frac } > 0 &&
            unsafe {
                        *unsafe {
                                (*p_a_1).a.offset((unsafe { (*p_a_1).n_digit } - 1) as
                                        isize)
                            }
                    } as i32 == 0 {
        {
            let __p = unsafe { &mut (*p_a_1).n_digit };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        {
            let __p = unsafe { &mut (*p_a_1).n_frac };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
    while unsafe { (*p_b_1).n_frac } > 0 &&
            unsafe {
                        *unsafe {
                                (*p_b_1).a.offset((unsafe { (*p_b_1).n_digit } - 1) as
                                        isize)
                            }
                    } as i32 == 0 {
        {
            let __p = unsafe { &mut (*p_b_1).n_digit };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        {
            let __p = unsafe { &mut (*p_b_1).n_frac };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
    if unsafe { (*p_a_1).sign } as i32 != unsafe { (*p_b_1).sign } as i32 {
        return if unsafe { (*p_a_1).sign } != 0 { -1 } else { 1 };
    }
    if unsafe { (*p_a_1).sign } != 0 {
        let p_temp: *mut Decimal = p_a_1;
        p_a_1 = p_b_1;
        p_b_1 = p_temp;
    }
    n_a_sig = unsafe { (*p_a_1).n_digit } - unsafe { (*p_a_1).n_frac };
    n_b_sig = unsafe { (*p_b_1).n_digit } - unsafe { (*p_b_1).n_frac };
    if n_a_sig != n_b_sig { return n_a_sig - n_b_sig; }
    n = unsafe { (*p_a_1).n_digit };
    if n > unsafe { (*p_b_1).n_digit } { n = unsafe { (*p_b_1).n_digit }; }
    rc =
        unsafe {
            memcmp(unsafe { (*p_a_1).a } as *const (),
                unsafe { (*p_b_1).a } as *const (), n as u64)
        };
    if rc == 0 {
        rc = unsafe { (*p_a_1).n_digit } - unsafe { (*p_b_1).n_digit };
    }
    return rc;
}
extern "C" fn decimal_cmp_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p_a: *mut Decimal = core::ptr::null_mut();
    let mut p_b: *mut Decimal = core::ptr::null_mut();
    '__b21: loop {
        '__c21: loop {
            let mut rc: i32 = 0;
            { let _ = argc; };
            p_a =
                decimal_new(context, unsafe { *argv.offset(0 as isize) }, 1);
            if p_a == core::ptr::null_mut() || unsafe { (*p_a).is_null } != 0
                {
                break '__b21;
            }
            p_b =
                decimal_new(context, unsafe { *argv.offset(1 as isize) }, 1);
            if p_b == core::ptr::null_mut() || unsafe { (*p_b).is_null } != 0
                {
                break '__b21;
            }
            rc = decimal_cmp(p_a, p_b);
            if rc < 0 { rc = -1; } else if rc > 0 { rc = 1; }
            unsafe { sqlite3_result_int(context, rc) };
            break '__c21;
        }
        if !(false) { break '__b21; }
    }
    decimal_free(p_a);
    decimal_free(p_b);
}
extern "C" fn decimal_add(p_a_1: *mut Decimal, p_b_1: *mut Decimal) -> () {
    let mut n_sig: i32 = 0;
    let mut n_frac: i32 = 0;
    let mut n_digit: i32 = 0;
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    if p_a_1 == core::ptr::null_mut() { return; }
    if unsafe { (*p_a_1).oom } != 0 || p_b_1 == core::ptr::null_mut() ||
            unsafe { (*p_b_1).oom } != 0 {
        unsafe { (*p_a_1).oom = 1 as i8 };
        return;
    }
    if unsafe { (*p_a_1).is_null } != 0 || unsafe { (*p_b_1).is_null } != 0 {
        unsafe { (*p_a_1).is_null = 1 as i8 };
        return;
    }
    n_sig = unsafe { (*p_a_1).n_digit } - unsafe { (*p_a_1).n_frac };
    if n_sig != 0 &&
            unsafe { *unsafe { (*p_a_1).a.offset(0 as isize) } } as i32 == 0 {
        { let __p = &mut n_sig; let __t = *__p; *__p -= 1; __t };
    }
    if n_sig < unsafe { (*p_b_1).n_digit } - unsafe { (*p_b_1).n_frac } {
        n_sig = unsafe { (*p_b_1).n_digit } - unsafe { (*p_b_1).n_frac };
    }
    n_frac = unsafe { (*p_a_1).n_frac };
    if n_frac < unsafe { (*p_b_1).n_frac } {
        n_frac = unsafe { (*p_b_1).n_frac };
    }
    n_digit = n_sig + n_frac + 1;
    decimal_expand(p_a_1, n_digit, n_frac);
    decimal_expand(p_b_1, n_digit, n_frac);
    if unsafe { (*p_a_1).oom } != 0 || unsafe { (*p_b_1).oom } != 0 {
        unsafe { (*p_a_1).oom = 1 as i8 };
    } else {
        if unsafe { (*p_a_1).sign } as i32 == unsafe { (*p_b_1).sign } as i32
            {
            let mut carry: i32 = 0;
            {
                i = n_digit - 1;
                '__b22: loop {
                    if !(i >= 0) { break '__b22; }
                    '__c22: loop {
                        let x: i32 =
                            unsafe { *unsafe { (*p_a_1).a.offset(i as isize) } } as i32
                                    +
                                    unsafe { *unsafe { (*p_b_1).a.offset(i as isize) } } as i32
                                + carry;
                        if x >= 10 {
                            carry = 1;
                            unsafe {
                                *unsafe { (*p_a_1).a.offset(i as isize) } = (x - 10) as i8
                            };
                        } else {
                            carry = 0;
                            unsafe {
                                *unsafe { (*p_a_1).a.offset(i as isize) } = x as i8
                            };
                        }
                        break '__c22;
                    }
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                }
            }
        } else {
            let mut a_a: *const i8 = core::ptr::null();
            let mut a_b: *const i8 = core::ptr::null();
            let mut borrow: i32 = 0;
            rc =
                unsafe {
                    memcmp(unsafe { (*p_a_1).a } as *const (),
                        unsafe { (*p_b_1).a } as *const (), n_digit as u64)
                };
            if rc < 0 {
                a_a = unsafe { (*p_b_1).a };
                a_b = unsafe { (*p_a_1).a };
                unsafe {
                    (*p_a_1).sign = (unsafe { (*p_a_1).sign } == 0) as i32 as i8
                };
            } else {
                a_a = unsafe { (*p_a_1).a };
                a_b = unsafe { (*p_b_1).a };
            }
            {
                i = n_digit - 1;
                '__b23: loop {
                    if !(i >= 0) { break '__b23; }
                    '__c23: loop {
                        let x: i32 =
                            unsafe { *a_a.offset(i as isize) } as i32 -
                                    unsafe { *a_b.offset(i as isize) } as i32 - borrow;
                        if x < 0 {
                            unsafe {
                                *unsafe { (*p_a_1).a.offset(i as isize) } = (x + 10) as i8
                            };
                            borrow = 1;
                        } else {
                            unsafe {
                                *unsafe { (*p_a_1).a.offset(i as isize) } = x as i8
                            };
                            borrow = 0;
                        }
                        break '__c23;
                    }
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                }
            }
        }
    }
}
extern "C" fn decimal_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let p: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(0 as isize) }, 0);
    let mut n: i32 = 0;
    if argc == 2 {
        n = unsafe { sqlite3_value_int(unsafe { *argv.offset(1 as isize) }) };
        if n > 0 { decimal_round(p, n); }
    } else { n = 0; }
    if !(p).is_null() {
        if unsafe { sqlite3_user_data(context) } != core::ptr::null_mut() {
            decimal_result_sci(context, p as *const Decimal, n);
        } else { decimal_result(context, p); }
        decimal_free(p);
    }
}
extern "C" fn decimal_coll_func(not_used_1: *mut (), n_key1_1: i32,
    p_key1_1: *const (), n_key2_1: i32, p_key2_1: *const ()) -> i32 {
    let z_a: *const u8 = p_key1_1 as *const u8;
    let z_b: *const u8 = p_key2_1 as *const u8;
    let p_a: *mut Decimal = decimal_new_from_text(z_a as *const i8, n_key1_1);
    let p_b: *mut Decimal = decimal_new_from_text(z_b as *const i8, n_key2_1);
    let mut rc: i32 = 0;
    { let _ = not_used_1; };
    if p_a == core::ptr::null_mut() || p_b == core::ptr::null_mut() {
        rc = 0;
    } else { rc = decimal_cmp(p_a, p_b); }
    decimal_free(p_a);
    decimal_free(p_b);
    return rc;
}
extern "C" fn decimal_add_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let p_a: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(0 as isize) }, 1);
    let p_b: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(1 as isize) }, 1);
    { let _ = argc; };
    decimal_add(p_a, p_b);
    decimal_result(context, p_a);
    decimal_free(p_a);
    decimal_free(p_b);
}
extern "C" fn decimal_sub_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let p_a: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(0 as isize) }, 1);
    let p_b: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(1 as isize) }, 1);
    { let _ = argc; };
    if !(p_b).is_null() {
        unsafe { (*p_b).sign = (unsafe { (*p_b).sign } == 0) as i32 as i8 };
        decimal_add(p_a, p_b);
        decimal_result(context, p_a);
    }
    decimal_free(p_a);
    decimal_free(p_b);
}
extern "C" fn decimal_sum_step(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut Decimal = core::ptr::null_mut();
    let mut p_arg: *mut Decimal = core::ptr::null_mut();
    { let _ = argc; };
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<Decimal>() as i32)
            } as *mut Decimal;
    if p == core::ptr::null_mut() { return; }
    if (unsafe { (*p).is_init } == 0) as i32 != 0 {
        unsafe { (*p).is_init = 1 as i8 };
        unsafe {
            (*p).a =
                unsafe { sqlite3_malloc64(2 as sqlite3_uint64) } as *mut i8
        };
        if unsafe { (*p).a } == core::ptr::null_mut() {
            unsafe { (*p).oom = 1 as i8 };
        } else { unsafe { *unsafe { (*p).a.offset(0 as isize) } = 0 as i8 }; }
        unsafe { (*p).n_digit = 1 };
        unsafe { (*p).n_frac = 0 };
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    p_arg = decimal_new(context, unsafe { *argv.offset(0 as isize) }, 1);
    decimal_add(p, p_arg);
    decimal_free(p_arg);
}
extern "C" fn decimal_sum_inverse(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut Decimal = core::ptr::null_mut();
    let mut p_arg: *mut Decimal = core::ptr::null_mut();
    { let _ = argc; };
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<Decimal>() as i32)
            } as *mut Decimal;
    if p == core::ptr::null_mut() { return; }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    p_arg = decimal_new(context, unsafe { *argv.offset(0 as isize) }, 1);
    if !(p_arg).is_null() {
        unsafe {
            (*p_arg).sign = (unsafe { (*p_arg).sign } == 0) as i32 as i8
        };
    }
    decimal_add(p, p_arg);
    decimal_free(p_arg);
}
extern "C" fn decimal_sum_value(context: *mut sqlite3_context) -> () {
    let p: *mut Decimal =
        unsafe { sqlite3_aggregate_context(context, 0) } as *mut Decimal;
    if p == core::ptr::null_mut() { return; }
    decimal_result(context, p);
}
extern "C" fn decimal_sum_finalize(context: *mut sqlite3_context) -> () {
    let p: *mut Decimal =
        unsafe { sqlite3_aggregate_context(context, 0) } as *mut Decimal;
    if p == core::ptr::null_mut() { return; }
    decimal_result(context, p);
    decimal_clear(unsafe { &*p });
}
extern "C" fn decimal_mul_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let p_a: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(0 as isize) }, 1);
    let p_b: *mut Decimal =
        decimal_new(context, unsafe { *argv.offset(1 as isize) }, 1);
    '__b24: loop {
        '__c24: loop {
            { let _ = argc; };
            if p_a == core::ptr::null_mut() || unsafe { (*p_a).oom } != 0 ||
                                unsafe { (*p_a).is_null } != 0 ||
                            p_b == core::ptr::null_mut() || unsafe { (*p_b).oom } != 0
                    || unsafe { (*p_b).is_null } != 0 {
                break '__b24;
            }
            decimal_mul(p_a, p_b as *const Decimal);
            if unsafe { (*p_a).oom } != 0 { break '__b24; }
            decimal_result(context, p_a);
            break '__c24;
        }
        if !(false) { break '__b24; }
    }
    decimal_free(p_a);
    decimal_free(p_b);
}
extern "C" fn decimal_pow2_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    { let _ = argc; };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 1
        {
        let p_a: *mut Decimal =
            decimal_pow2(unsafe {
                    sqlite3_value_int(unsafe { *argv.offset(0 as isize) })
                });
        decimal_result_sci(context, p_a as *const Decimal, 0);
        decimal_free(p_a);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_decimal_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i: u32 = 0 as u32;
        { let _ = pz_err_msg_1; };
        { let _ = p_api_1; };
        {
            i = 0 as u32;
            '__b25: loop {
                if !(i <
                                    (core::mem::size_of::<[sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0; 9]>()
                                                    as u64 / 24) as i32 as u32 && rc == 0) {
                    break '__b25;
                }
                '__c25: loop {
                    rc =
                        unsafe {
                            sqlite3_create_function(db, a_func[i as usize].z_func_name,
                                a_func[i as usize].n_arg, 1 | 2097152 | 2048,
                                if a_func[i as usize].i_arg != 0 {
                                        db
                                    } else { core::ptr::null_mut() } as *mut (),
                                a_func[i as usize].x_func, None, None)
                        };
                    break '__c25;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_window_function(db,
                        c"decimal_sum".as_ptr() as *mut i8 as *const i8, 1,
                        1 | 2097152 | 2048, core::ptr::null_mut(),
                        Some(decimal_sum_step), Some(decimal_sum_finalize),
                        Some(decimal_sum_value), Some(decimal_sum_inverse), None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_collation(db,
                        c"decimal".as_ptr() as *mut i8 as *const i8, 1,
                        core::ptr::null_mut(), Some(decimal_coll_func))
                };
        }
        return rc;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
    z_func_name: *const i8,
    n_arg: i32,
    i_arg: i32,
    x_func: Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
        *mut *mut sqlite3_value) -> ()>,
}
static mut a_func: [sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0; 9] =
    [sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal".as_ptr() as *const i8,
                n_arg: 1,
                i_arg: 0,
                x_func: Some(decimal_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal".as_ptr() as *const i8,
                n_arg: 2,
                i_arg: 0,
                x_func: Some(decimal_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_exp".as_ptr() as *const i8,
                n_arg: 1,
                i_arg: 1,
                x_func: Some(decimal_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_exp".as_ptr() as *const i8,
                n_arg: 2,
                i_arg: 1,
                x_func: Some(decimal_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_cmp".as_ptr() as *const i8,
                n_arg: 2,
                i_arg: 0,
                x_func: Some(decimal_cmp_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_add".as_ptr() as *const i8,
                n_arg: 2,
                i_arg: 0,
                x_func: Some(decimal_add_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_sub".as_ptr() as *const i8,
                n_arg: 2,
                i_arg: 0,
                x_func: Some(decimal_sub_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_mul".as_ptr() as *const i8,
                n_arg: 2,
                i_arg: 0,
                x_func: Some(decimal_mul_func),
            },
            sqlite3_decimal_init_s0_N23sqlite3_decimal_init_s0 {
                z_func_name: c"decimal_pow2".as_ptr() as *const i8,
                n_arg: 1,
                i_arg: 0,
                x_func: Some(decimal_pow2_func),
            }];
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
    fn isspace(_c: i32)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}