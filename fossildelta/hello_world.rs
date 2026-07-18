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
type s16 = i16;
#[repr(C)]
#[derive(Copy, Clone)]
struct hash {
    a: u16,
    b: u16,
    i: u16,
    z: [i8; 16],
}
extern "C" fn hash_init(p_hash_1: &mut hash, z: *const i8) -> () {
    let mut a: u16 = 0 as u16;
    let mut b: u16 = 0 as u16;
    let mut i: u16 = 0 as u16;
    a = { b = unsafe { *z.offset(0 as isize) } as u16; b };
    {
        i = 1 as u16;
        '__b0: loop {
            if !((i as i32) < 16) { break '__b0; }
            '__c0: loop {
                a += unsafe { *z.add(i as usize) } as i32 as u16;
                b += a as i32 as u16;
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        memcpy(&raw mut (*p_hash_1).z[0 as usize] as *mut i8 as *mut (),
            z as *const (), 16 as u64)
    };
    (*p_hash_1).a = (a as i32 & 65535) as u16;
    (*p_hash_1).b = (b as i32 & 65535) as u16;
    (*p_hash_1).i = 0 as u16;
}
extern "C" fn hash_next(p_hash_1: &mut hash, c: i32) -> () {
    let old: u16 = (*p_hash_1).z[(*p_hash_1).i as usize] as u16;
    (*p_hash_1).z[(*p_hash_1).i as usize] = c as i8;
    (*p_hash_1).i = ((*p_hash_1).i as i32 + 1 & 16 - 1) as u16;
    (*p_hash_1).a = ((*p_hash_1).a as i32 - old as i32 + c) as u16;
    (*p_hash_1).b =
        ((*p_hash_1).b as i32 - 16 * old as i32 + (*p_hash_1).a as i32) as
            u16;
}
extern "C" fn hash_32bit(p_hash_1: &hash) -> u32 {
    return ((*p_hash_1).a as i32 & 65535) as u32 |
            (((*p_hash_1).b as i32 & 65535) as u32) << 16;
}
extern "C" fn hash_once(z: *const i8) -> u32 {
    let mut a: u16 = 0 as u16;
    let mut b: u16 = 0 as u16;
    let mut i: u16 = 0 as u16;
    a = { b = unsafe { *z.offset(0 as isize) } as u16; b };
    {
        i = 1 as u16;
        '__b1: loop {
            if !((i as i32) < 16) { break '__b1; }
            '__c1: loop {
                a += unsafe { *z.add(i as usize) } as i32 as u16;
                b += a as i32 as u16;
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return a as u32 | (b as u32) << 16;
}
extern "C" fn put_int(mut v: u32, pz: &mut *mut i8) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut z_buf: [i8; 20] = [0; 20];
    if v == 0 as u32 {
        unsafe {
            *{
                        let __p = &mut *pz;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = '0' as i32 as i8
        };
        return;
    }
    {
        i = 0;
        '__b2: loop {
            if !(v > 0 as u32) { break '__b2; }
            '__c2: loop {
                z_buf[i as usize] = z_digits[(v & 63 as u32) as usize] as i8;
                break '__c2;
            }
            {
                ({ let __p = &mut i; let __t = *__p; *__p += 1; __t }) as u32;
                v >>= 6 as u32
            };
        }
    }
    {
        j = i - 1;
        '__b3: loop {
            if !(j >= 0) { break '__b3; }
            '__c3: loop {
                unsafe {
                    *{
                                let __p = &mut *pz;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = z_buf[j as usize]
                };
                break '__c3;
            }
            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
        }
    }
}
extern "C" fn delta_get_int(pz: &mut *const i8, p_len_1: &mut i32) -> u32 {
    let mut v: u32 = 0 as u32;
    let mut c: i32 = 0;
    let mut z: *mut u8 = *pz as *mut u8;
    let z_end: *mut u8 = unsafe { z.offset(*p_len_1 as isize) };
    while z < z_end && { c = z_value[unsafe { *z } as usize] as i32; c } >= 0
        {
        v = (v << 6) + c as u32;
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    *p_len_1 -= unsafe { z.offset_from(*pz as *mut u8) } as i64 as i32;
    *pz = z as *mut i8 as *const i8;
    return v;
}
extern "C" fn digit_count(v: i32) -> i32 {
    let mut i: u32 = 0 as u32;
    let mut x: u32 = 0 as u32;
    {
        { i = 1 as u32; x = 64 as u32 };
        '__b5: loop {
            if !(v as u32 >= x) { break '__b5; }
            '__c5: loop { break '__c5; }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                x <<= 6 as u32
            };
        }
    }
    return i as i32;
}
extern "C" fn checksum(z_in_1: *const i8, mut n_1: u64) -> u32 {
    let mut z: *const u8 = z_in_1 as *const u8;
    let z_end: *const u8 =
        unsafe { &raw const *z_in_1.add((n_1 & !3 as u64) as usize) } as
            *const u8;
    let mut sum: u32 = 0 as u32;
    if !(unsafe { z.offset_from(0 as *const u8) } as i64 % 4 as i64 ==
                            0 as i64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"checksum".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 232,
                c"(z - (const unsigned char*)0)%4==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if 0 == unsafe { *(&raw const byte_order_test as *mut i8) } as i32 {
        while z < z_end {
            sum += unsafe { *(z as *mut u32) };
            {
                let __n = 4;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
    } else {
        let mut sum0: u32 = 0 as u32;
        let mut sum1: u32 = 0 as u32;
        let mut sum2: u32 = 0 as u32;
        while n_1 >= 16 as u64 {
            sum0 +=
                unsafe { *z.offset(0 as isize) } as u32 +
                            unsafe { *z.offset(4 as isize) } as u32 +
                        unsafe { *z.offset(8 as isize) } as u32 +
                    unsafe { *z.offset(12 as isize) } as u32;
            sum1 +=
                unsafe { *z.offset(1 as isize) } as u32 +
                            unsafe { *z.offset(5 as isize) } as u32 +
                        unsafe { *z.offset(9 as isize) } as u32 +
                    unsafe { *z.offset(13 as isize) } as u32;
            sum2 +=
                unsafe { *z.offset(2 as isize) } as u32 +
                            unsafe { *z.offset(6 as isize) } as u32 +
                        unsafe { *z.offset(10 as isize) } as u32 +
                    unsafe { *z.offset(14 as isize) } as u32;
            sum +=
                unsafe { *z.offset(3 as isize) } as u32 +
                            unsafe { *z.offset(7 as isize) } as u32 +
                        unsafe { *z.offset(11 as isize) } as u32 +
                    unsafe { *z.offset(15 as isize) } as u32;
            {
                let __n = 16;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n_1 -= 16 as u64;
        }
        while n_1 >= 4 as u64 {
            sum0 += unsafe { *z.offset(0 as isize) } as u32;
            sum1 += unsafe { *z.offset(1 as isize) } as u32;
            sum2 += unsafe { *z.offset(2 as isize) } as u32;
            sum += unsafe { *z.offset(3 as isize) } as u32;
            {
                let __n = 4;
                let __p = &mut z;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n_1 -= 4 as u64;
        }
        sum += (sum2 << 8) + (sum1 << 16) + (sum0 << 24);
    }
    '__s9:
        {
        match n_1 & 3 as u64 {
            3 => {
                sum +=
                    ((unsafe { *z.offset(2 as isize) } as i32) << 8) as u32;
                sum +=
                    ((unsafe { *z.offset(1 as isize) } as i32) << 16) as u32;
                sum +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            2 => {
                sum +=
                    ((unsafe { *z.offset(1 as isize) } as i32) << 16) as u32;
                sum +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            1 => {
                sum +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            _ => {}
        }
    }
    return sum;
}
extern "C" fn delta_create(z_src_1: *const i8, len_src_1: u32,
    z_out_1: *const i8, len_out_1: u32, mut z_delta_1: *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut base: i32 = 0;
    let z_orig_delta: *const i8 = z_delta_1 as *const i8;
    let mut h: hash = unsafe { core::mem::zeroed() };
    let mut n_hash: i32 = 0;
    let mut landmark: *mut i32 = core::ptr::null_mut();
    let mut collide: *mut i32 = core::ptr::null_mut();
    let mut last_read: i32 = -1;
    put_int(len_out_1, &mut z_delta_1);
    unsafe {
        *{
                    let __p = &mut z_delta_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = '\n' as i32 as i8
    };
    if len_src_1 <= 16 as u32 {
        put_int(len_out_1, &mut z_delta_1);
        unsafe {
            *{
                        let __p = &mut z_delta_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = ':' as i32 as i8
        };
        unsafe {
            memcpy(z_delta_1 as *mut (), z_out_1 as *const (),
                len_out_1 as u64)
        };
        {
            let __n = len_out_1;
            let __p = &mut z_delta_1;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        put_int(checksum(z_out_1, len_out_1 as u64), &mut z_delta_1);
        unsafe {
            *{
                        let __p = &mut z_delta_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = ';' as i32 as i8
        };
        return unsafe { z_delta_1.offset_from(z_orig_delta) } as i64 as i32;
    }
    n_hash = (len_src_1 / 16 as u32) as i32;
    collide =
        unsafe {
                sqlite3_malloc64((n_hash as sqlite3_int64 *
                                2 as sqlite3_int64) as u64 *
                        core::mem::size_of::<i32>() as u64)
            } as *mut i32;
    if collide == core::ptr::null_mut() { return 0; }
    unsafe {
        memset(collide as *mut (), -1,
            (n_hash * 2) as u64 * core::mem::size_of::<i32>() as u64)
    };
    landmark = unsafe { collide.offset(n_hash as isize) };
    {
        i = 0;
        '__b10: loop {
            if !((i as u32) < len_src_1 - 16 as u32) { break '__b10; }
            '__c10: loop {
                let mut hv: i32 =
                    (hash_once(unsafe { &*z_src_1.offset(i as isize) }) %
                            n_hash as u32) as i32;
                unsafe {
                    *collide.offset((i / 16) as isize) =
                        unsafe { *landmark.offset(hv as isize) }
                };
                unsafe { *landmark.offset(hv as isize) = i / 16 };
                break '__c10;
            }
            i += 16;
        }
    }
    base = 0;
    while ((base + 16) as u32) < len_out_1 {
        let mut i_src: i32 = 0;
        let mut i_block: i32 = 0;
        let mut best_cnt: u32 = 0 as u32;
        let mut best_ofst: u32 = 0 as u32;
        let mut best_litsz: u32 = 0 as u32;
        hash_init(&mut h, unsafe { &*z_out_1.offset(base as isize) });
        i = 0;
        best_cnt = 0 as u32;
        loop {
            let mut hv: i32 = 0;
            let mut limit: i32 = 250;
            hv = (hash_32bit(&h) % n_hash as u32) as i32;
            i_block = unsafe { *landmark.offset(hv as isize) };
            while i_block >= 0 &&
                    { let __p = &mut limit; let __t = *__p; *__p -= 1; __t } > 0
                {
                let mut cnt: i32 = 0;
                let mut ofst: i32 = 0;
                let mut litsz: i32 = 0;
                let mut j: i32 = 0;
                let mut k: i32 = 0;
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                let mut sz: i32 = 0;
                let mut limit_x: i32 = 0;
                i_src = i_block * 16;
                y = base + i;
                limit_x =
                    if len_src_1 - i_src as u32 <= len_out_1 - y as u32 {
                            len_src_1
                        } else { i_src as u32 + len_out_1 - y as u32 } as i32;
                {
                    x = i_src;
                    '__b14: loop {
                        if !(x < limit_x) { break '__b14; }
                        '__c14: loop {
                            if unsafe { *z_src_1.offset(x as isize) } as i32 !=
                                    unsafe { *z_out_1.offset(y as isize) } as i32 {
                                break '__b14;
                            }
                            break '__c14;
                        }
                        {
                            { let __p = &mut x; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut y; let __t = *__p; *__p += 1; __t }
                        };
                    }
                }
                j = x - i_src - 1;
                {
                    k = 1;
                    '__b15: loop {
                        if !(k < i_src && k <= i) { break '__b15; }
                        '__c15: loop {
                            if unsafe { *z_src_1.offset((i_src - k) as isize) } as i32
                                    !=
                                    unsafe { *z_out_1.offset((base + i - k) as isize) } as i32 {
                                break '__b15;
                            }
                            break '__c15;
                        }
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    }
                }
                { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                ofst = i_src - k;
                cnt = j + k + 1;
                litsz = i - k;
                sz =
                    digit_count(i - k) + digit_count(cnt) + digit_count(ofst) +
                        3;
                if cnt >= sz && cnt as u32 > best_cnt {
                    best_cnt = cnt as u32;
                    best_ofst = (i_src - k) as u32;
                    best_litsz = litsz as u32;
                }
                i_block = unsafe { *collide.offset(i_block as isize) };
            }
            if best_cnt > 0 as u32 {
                if best_litsz > 0 as u32 {
                    put_int(best_litsz, &mut z_delta_1);
                    unsafe {
                        *{
                                    let __p = &mut z_delta_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = ':' as i32 as i8
                    };
                    unsafe {
                        memcpy(z_delta_1 as *mut (),
                            unsafe { &raw const *z_out_1.offset(base as isize) } as
                                *const (), best_litsz as u64)
                    };
                    {
                        let __n = best_litsz;
                        let __p = &mut z_delta_1;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    base += best_litsz as i32;
                }
                base += best_cnt as i32;
                put_int(best_cnt, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = '@' as i32 as i8
                };
                put_int(best_ofst, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = ',' as i32 as i8
                };
                if best_ofst + best_cnt - 1 as u32 > last_read as u32 {
                    last_read = (best_ofst + best_cnt - 1 as u32) as i32;
                }
                best_cnt = 0 as u32;
                break;
            }
            if (base + i + 16) as u32 >= len_out_1 {
                put_int(len_out_1 - base as u32, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = ':' as i32 as i8
                };
                unsafe {
                    memcpy(z_delta_1 as *mut (),
                        unsafe { &raw const *z_out_1.offset(base as isize) } as
                            *const (), (len_out_1 - base as u32) as u64)
                };
                {
                    let __n = len_out_1 - base as u32;
                    let __p = &mut z_delta_1;
                    *__p = unsafe { (*__p).add(__n as usize) };
                };
                base = len_out_1 as i32;
                break;
            }
            hash_next(&mut h,
                unsafe { *z_out_1.offset((base + i + 16) as isize) } as i32);
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (base as u32) < len_out_1 {
        put_int(len_out_1 - base as u32, &mut z_delta_1);
        unsafe {
            *{
                        let __p = &mut z_delta_1;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = ':' as i32 as i8
        };
        unsafe {
            memcpy(z_delta_1 as *mut (),
                unsafe { &raw const *z_out_1.offset(base as isize) } as
                    *const (), (len_out_1 - base as u32) as u64)
        };
        {
            let __n = len_out_1 - base as u32;
            let __p = &mut z_delta_1;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
    }
    put_int(checksum(z_out_1, len_out_1 as u64), &mut z_delta_1);
    unsafe {
        *{
                    let __p = &mut z_delta_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = ';' as i32 as i8
    };
    unsafe { sqlite3_free(collide as *mut ()) };
    return unsafe { z_delta_1.offset_from(z_orig_delta) } as i64 as i32;
}
extern "C" fn delta_output_size(mut z_delta_1: *const i8,
    mut len_delta_1: i32) -> i32 {
    let mut size: i32 = 0;
    size = delta_get_int(&mut z_delta_1, &mut len_delta_1) as i32;
    if len_delta_1 <= 0 || unsafe { *z_delta_1 } as i32 != '\n' as i32 {
        return -1;
    }
    return size;
}
extern "C" fn delta_apply(z_src_1: *const i8, len_src_1: i32,
    mut z_delta_1: *const i8, mut len_delta_1: i32, mut z_out_1: *mut i8)
    -> i32 {
    let mut limit: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut total: sqlite3_uint64 = 0 as sqlite3_uint64;
    limit = delta_get_int(&mut z_delta_1, &mut len_delta_1) as sqlite3_uint64;
    if len_delta_1 <= 0 || unsafe { *z_delta_1 } as i32 != '\n' as i32 {
        return -1;
    }
    {
        let __p = &mut z_delta_1;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    { let __p = &mut len_delta_1; let __t = *__p; *__p -= 1; __t };
    while unsafe { *z_delta_1 } != 0 && len_delta_1 > 0 {
        let mut cnt: u32 = 0 as u32;
        let mut ofst: u32 = 0 as u32;
        cnt = delta_get_int(&mut z_delta_1, &mut len_delta_1);
        if len_delta_1 <= 0 { return -1; }
        '__s17:
            {
            match unsafe { *z_delta_1.offset(0 as isize) } {
                64 => {
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        ofst = delta_get_int(&mut z_delta_1, &mut len_delta_1);
                        if len_delta_1 > 0 &&
                                unsafe { *z_delta_1.offset(0 as isize) } as i32 !=
                                    ',' as i32 {
                            return -1;
                        }
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        total += cnt as sqlite3_uint64;
                        if total > limit { return -1; }
                        if ofst as u64 + cnt as u64 > len_src_1 as u64 {
                            return -1;
                        }
                        unsafe {
                            memcpy(z_out_1 as *mut (),
                                unsafe { &raw const *z_src_1.add(ofst as usize) } as
                                    *const (), cnt as u64)
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_out_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        break '__s17;
                    }
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        total += cnt as sqlite3_uint64;
                        if total > limit { return -1; }
                        if cnt > len_delta_1 as u32 { return -1; }
                        unsafe {
                            memcpy(z_out_1 as *mut (), z_delta_1 as *const (),
                                cnt as u64)
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_out_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_delta_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        len_delta_1 -= cnt as i32;
                        break '__s17;
                    }
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { *z_out_1.offset(0 as isize) = 0 as i8 };
                        if total != limit { return -1; }
                        return total as i32;
                    }
                    { return -1; }
                }
                58 => {
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        total += cnt as sqlite3_uint64;
                        if total > limit { return -1; }
                        if cnt > len_delta_1 as u32 { return -1; }
                        unsafe {
                            memcpy(z_out_1 as *mut (), z_delta_1 as *const (),
                                cnt as u64)
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_out_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        {
                            let __n = cnt;
                            let __p = &mut z_delta_1;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        len_delta_1 -= cnt as i32;
                        break '__s17;
                    }
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { *z_out_1.offset(0 as isize) = 0 as i8 };
                        if total != limit { return -1; }
                        return total as i32;
                    }
                    { return -1; }
                }
                59 => {
                    {
                        {
                            let __p = &mut z_delta_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        {
                            let __p = &mut len_delta_1;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { *z_out_1.offset(0 as isize) = 0 as i8 };
                        if total != limit { return -1; }
                        return total as i32;
                    }
                    { return -1; }
                }
                _ => { { return -1; } }
            }
        }
    }
    return -1;
}
extern "C" fn delta_create_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut a_orig: *const i8 = core::ptr::null();
    let mut n_orig: i32 = 0;
    let mut a_new: *const i8 = core::ptr::null();
    let mut n_new: i32 = 0;
    let mut a_out: *mut i8 = core::ptr::null_mut();
    let mut n_out: i32 = 0;
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"deltaCreateFunc".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 664,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) } == 5
        {
        return;
    }
    n_orig =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    a_orig =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    n_new =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) }) };
    a_new =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(1 as isize) }) } as
            *const i8;
    a_out =
        unsafe { sqlite3_malloc64((n_new + 70) as sqlite3_uint64) } as
            *mut i8;
    if a_out == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
    } else {
        n_out =
            delta_create(a_orig, n_orig as u32, a_new, n_new as u32, a_out);
        if n_out < 0 {
            unsafe { sqlite3_free(a_out as *mut ()) };
            unsafe {
                sqlite3_result_error(context,
                    c"cannot create fossil delta".as_ptr() as *mut i8 as
                        *const i8, -1)
            };
        } else {
            unsafe {
                sqlite3_result_blob(context, a_out as *const (), n_out,
                    Some(sqlite3_free))
            };
        }
    }
}
extern "C" fn delta_apply_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut a_orig: *const i8 = core::ptr::null();
    let mut n_orig: i32 = 0;
    let mut a_delta: *const i8 = core::ptr::null();
    let mut n_delta: i32 = 0;
    let mut a_out: *mut i8 = core::ptr::null_mut();
    let mut n_out: i32 = 0;
    let mut n_out2: i32 = 0;
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"deltaApplyFunc".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 699,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) } == 5
        {
        return;
    }
    n_orig =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    a_orig =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    n_delta =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(1 as isize) }) };
    a_delta =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(1 as isize) }) } as
            *const i8;
    n_out = delta_output_size(a_delta, n_delta);
    if n_out < 0 {
        unsafe {
            sqlite3_result_error(context,
                c"corrupt fossil delta".as_ptr() as *mut i8 as *const i8, -1)
        };
        return;
    }
    a_out =
        unsafe {
                sqlite3_malloc64((n_out as sqlite3_int64 + 1 as sqlite3_int64)
                        as sqlite3_uint64)
            } as *mut i8;
    if a_out == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(context) };
    } else {
        n_out2 = delta_apply(a_orig, n_orig, a_delta, n_delta, a_out);
        if n_out2 != n_out {
            unsafe { sqlite3_free(a_out as *mut ()) };
            unsafe {
                sqlite3_result_error(context,
                    c"corrupt fossil delta".as_ptr() as *mut i8 as *const i8,
                    -1)
            };
        } else {
            unsafe {
                sqlite3_result_blob(context, a_out as *const (), n_out,
                    Some(sqlite3_free))
            };
        }
    }
}
extern "C" fn delta_output_size_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut a_delta: *const i8 = core::ptr::null();
    let mut n_delta: i32 = 0;
    let mut n_out: i32 = 0;
    if !(argc == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"deltaOutputSizeFunc".as_ptr() as *const i8,
                c"fossildelta.c".as_ptr() as *mut i8 as *const i8, 740,
                c"argc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } == 5
        {
        return;
    }
    n_delta =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    a_delta =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    n_out = delta_output_size(a_delta, n_delta);
    if n_out < 0 {
        unsafe {
            sqlite3_result_error(context,
                c"corrupt fossil delta".as_ptr() as *mut i8 as *const i8, -1)
        };
        return;
    } else { unsafe { sqlite3_result_int(context, n_out) }; }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct deltaparsevtab_vtab {
    base: sqlite3_vtab,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct deltaparsevtab_cursor {
    base: sqlite3_vtab_cursor,
    a_delta: *mut i8,
    i_cursor: i64,
    i_next: i64,
    n_delta: i64,
    e_op: i32,
    a1: u32,
    a2: u32,
}
static mut az_op: [*const i8; 6] =
    [c"SIZE".as_ptr() as *const i8, c"COPY".as_ptr() as *const i8,
            c"INSERT".as_ptr() as *const i8,
            c"CHECKSUM".as_ptr() as *const i8, c"ERROR".as_ptr() as *const i8,
            c"EOF".as_ptr() as *const i8];
extern "C" fn deltaparsevtab_connect(db: *mut sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut deltaparsevtab_vtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(op,a1,a2,delta HIDDEN)".as_ptr() as *mut i8
                    as *const i8)
        };
    if rc == 0 {
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<deltaparsevtab_vtab>()
                            as sqlite3_uint64)
                } as *mut deltaparsevtab_vtab;
        unsafe { *pp_vtab_1 = p_new as *mut sqlite3_vtab };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<deltaparsevtab_vtab>() as u64)
        };
        unsafe { sqlite3_vtab_config(db, 2) };
    }
    return rc;
}
extern "C" fn deltaparsevtab_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p: *mut deltaparsevtab_vtab = p_vtab_1 as *mut deltaparsevtab_vtab;
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
extern "C" fn deltaparsevtab_open(p: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let mut p_cur: *mut deltaparsevtab_cursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<deltaparsevtab_cursor>()
                        as sqlite3_uint64)
            } as *mut deltaparsevtab_cursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<deltaparsevtab_cursor>() as u64)
    };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}
extern "C" fn deltaparsevtab_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut deltaparsevtab_cursor = cur as *mut deltaparsevtab_cursor;
    unsafe { sqlite3_free(unsafe { (*p_cur).a_delta } as *mut ()) };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}
extern "C" fn deltaparsevtab_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut deltaparsevtab_cursor = cur as *mut deltaparsevtab_cursor;
    let mut z: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    unsafe { (*p_cur).i_cursor = unsafe { (*p_cur).i_next } };
    if unsafe { (*p_cur).i_cursor } >= unsafe { (*p_cur).n_delta } {
        unsafe { (*p_cur).e_op = 4 };
        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
        return 0;
    }
    z =
        unsafe {
                unsafe {
                    (*p_cur).a_delta.offset(unsafe { (*p_cur).i_cursor } as
                            isize)
                }
            } as *const i8;
    unsafe { (*p_cur).a1 = delta_get_int(&mut z, &mut i) };
    '__s18:
        {
        match if i > 0 {
                (unsafe { *z.offset(0 as isize) }) as i32
            } else { 0 } {
            64 => {
                {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    if unsafe { (*p_cur).i_next } >= unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                        break '__s18;
                    }
                    unsafe { (*p_cur).a2 = delta_get_int(&mut z, &mut i) };
                    unsafe { (*p_cur).e_op = 1 };
                    unsafe {
                        (*p_cur).i_next =
                            unsafe {
                                        unsafe {
                                            z.offset(1 as
                                                        isize).offset_from(unsafe { (*p_cur).a_delta })
                                        }
                                    } as i64 as i64
                    };
                    break '__s18;
                }
                {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    unsafe {
                        (*p_cur).a2 =
                            unsafe { z.offset_from(unsafe { (*p_cur).a_delta }) } as i64
                                as u32
                    };
                    unsafe { (*p_cur).e_op = 2 };
                    unsafe {
                        (*p_cur).i_next =
                            unsafe {
                                        unsafe {
                                            z.add(unsafe { (*p_cur).a1 } as
                                                        usize).offset_from(unsafe { (*p_cur).a_delta })
                                        }
                                    } as i64 as i64
                    };
                    break '__s18;
                }
                {
                    unsafe { (*p_cur).e_op = 3 };
                    unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    break '__s18;
                }
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
            58 => {
                {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    unsafe {
                        (*p_cur).a2 =
                            unsafe { z.offset_from(unsafe { (*p_cur).a_delta }) } as i64
                                as u32
                    };
                    unsafe { (*p_cur).e_op = 2 };
                    unsafe {
                        (*p_cur).i_next =
                            unsafe {
                                        unsafe {
                                            z.add(unsafe { (*p_cur).a1 } as
                                                        usize).offset_from(unsafe { (*p_cur).a_delta })
                                        }
                                    } as i64 as i64
                    };
                    break '__s18;
                }
                {
                    unsafe { (*p_cur).e_op = 3 };
                    unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    break '__s18;
                }
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
            59 => {
                {
                    unsafe { (*p_cur).e_op = 3 };
                    unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    break '__s18;
                }
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
            _ => {
                {
                    if unsafe { (*p_cur).i_next } == unsafe { (*p_cur).n_delta }
                        {
                        unsafe { (*p_cur).e_op = 5 };
                    } else {
                        unsafe { (*p_cur).e_op = 4 };
                        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
                    }
                    break '__s18;
                }
            }
        }
    }
    return 0;
}
extern "C" fn deltaparsevtab_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    unsafe {
        let p_cur: *const deltaparsevtab_cursor =
            cur as *mut deltaparsevtab_cursor as *const deltaparsevtab_cursor;
        '__s19:
            {
            match i {
                0 => {
                    {
                        unsafe {
                            sqlite3_result_text(ctx,
                                az_op[unsafe { (*p_cur).e_op } as usize], -1, None)
                        };
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_int(ctx, unsafe { (*p_cur).a1 } as i32)
                        };
                        break '__s19;
                    }
                    {
                        if unsafe { (*p_cur).e_op } == 1 {
                            unsafe {
                                sqlite3_result_int(ctx, unsafe { (*p_cur).a2 } as i32)
                            };
                        } else if unsafe { (*p_cur).e_op } == 2 {
                            if unsafe { (*p_cur).a2 } as i64 +
                                        unsafe { (*p_cur).a1 } as i64 > unsafe { (*p_cur).n_delta }
                                {
                                unsafe {
                                    sqlite3_result_zeroblob(ctx, unsafe { (*p_cur).a1 } as i32)
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).a_delta.add(unsafe { (*p_cur).a2 } as usize)
                                                }
                                            } as *const (), unsafe { (*p_cur).a1 } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                1 => {
                    {
                        unsafe {
                            sqlite3_result_int(ctx, unsafe { (*p_cur).a1 } as i32)
                        };
                        break '__s19;
                    }
                    {
                        if unsafe { (*p_cur).e_op } == 1 {
                            unsafe {
                                sqlite3_result_int(ctx, unsafe { (*p_cur).a2 } as i32)
                            };
                        } else if unsafe { (*p_cur).e_op } == 2 {
                            if unsafe { (*p_cur).a2 } as i64 +
                                        unsafe { (*p_cur).a1 } as i64 > unsafe { (*p_cur).n_delta }
                                {
                                unsafe {
                                    sqlite3_result_zeroblob(ctx, unsafe { (*p_cur).a1 } as i32)
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).a_delta.add(unsafe { (*p_cur).a2 } as usize)
                                                }
                                            } as *const (), unsafe { (*p_cur).a1 } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                2 => {
                    {
                        if unsafe { (*p_cur).e_op } == 1 {
                            unsafe {
                                sqlite3_result_int(ctx, unsafe { (*p_cur).a2 } as i32)
                            };
                        } else if unsafe { (*p_cur).e_op } == 2 {
                            if unsafe { (*p_cur).a2 } as i64 +
                                        unsafe { (*p_cur).a1 } as i64 > unsafe { (*p_cur).n_delta }
                                {
                                unsafe {
                                    sqlite3_result_zeroblob(ctx, unsafe { (*p_cur).a1 } as i32)
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).a_delta.add(unsafe { (*p_cur).a2 } as usize)
                                                }
                                            } as *const (), unsafe { (*p_cur).a1 } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                        break '__s19;
                    }
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                3 => {
                    {
                        unsafe {
                            sqlite3_result_blob(ctx,
                                unsafe { (*p_cur).a_delta } as *const (),
                                unsafe { (*p_cur).n_delta } as i32,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        break '__s19;
                    }
                }
                _ => {}
            }
        }
        return 0;
    }
}
extern "C" fn deltaparsevtab_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p_cur: *const deltaparsevtab_cursor =
        cur as *mut deltaparsevtab_cursor as *const deltaparsevtab_cursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_cursor } };
    return 0;
}
extern "C" fn deltaparsevtab_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const deltaparsevtab_cursor =
        cur as *mut deltaparsevtab_cursor as *const deltaparsevtab_cursor;
    return (unsafe { (*p_cur).e_op } == 5 ||
                unsafe { (*p_cur).i_cursor } >= unsafe { (*p_cur).n_delta })
            as i32;
}
extern "C" fn deltaparsevtab_filter(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let p_cur: *mut deltaparsevtab_cursor =
        p_vtab_cursor_1 as *mut deltaparsevtab_cursor;
    let mut a: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    unsafe { (*p_cur).e_op = 4 };
    if idx_num_1 != 1 { return 0; }
    unsafe {
        (*p_cur).n_delta =
            unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                } as i64
    };
    a =
        unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if unsafe { (*p_cur).n_delta } == 0 as i64 || a == core::ptr::null() {
        return 0;
    }
    unsafe {
        (*p_cur).a_delta =
            unsafe {
                    sqlite3_malloc64((unsafe { (*p_cur).n_delta } + 1 as i64) as
                            sqlite3_uint64)
                } as *mut i8
    };
    if unsafe { (*p_cur).a_delta } == core::ptr::null_mut() {
        unsafe { (*p_cur).n_delta = 0 as i64 };
        return 7;
    }
    unsafe {
        memcpy(unsafe { (*p_cur).a_delta } as *mut (), a as *const (),
            unsafe { (*p_cur).n_delta } as u64)
    };
    unsafe {
        *unsafe {
                    (*p_cur).a_delta.offset(unsafe { (*p_cur).n_delta } as
                            isize)
                } = 0 as i8
    };
    a = unsafe { (*p_cur).a_delta } as *const i8;
    unsafe { (*p_cur).e_op = 0 };
    unsafe { (*p_cur).a1 = delta_get_int(&mut a, &mut i) };
    if i <= 0 || unsafe { *a.offset(0 as isize) } as i32 != '\n' as i32 {
        unsafe { (*p_cur).e_op = 4 };
        unsafe {
            (*p_cur).a1 =
                { unsafe { (*p_cur).a2 = 0 as u32 }; unsafe { (*p_cur).a2 } }
        };
        unsafe { (*p_cur).i_next = unsafe { (*p_cur).n_delta } };
        return 0;
    }
    {
        let __p = &mut a;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    unsafe {
        (*p_cur).i_next =
            unsafe { a.offset_from(unsafe { (*p_cur).a_delta }) } as i64 as
                i64
    };
    return 0;
}
extern "C" fn deltaparsevtab_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b20: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b20;
            }
            '__c20: loop {
                if unsafe {
                            (*unsafe {
                                        (*p_idx_info_1).a_constraint.offset(i as isize)
                                    }).i_column
                        } != 3 {
                    break '__c20;
                }
                if unsafe {
                                (*unsafe {
                                            (*p_idx_info_1).a_constraint.offset(i as isize)
                                        }).usable
                            } as i32 == 0 {
                    break '__c20;
                }
                if unsafe {
                                (*unsafe {
                                            (*p_idx_info_1).a_constraint.offset(i as isize)
                                        }).op
                            } as i32 != 2 {
                    break '__c20;
                }
                unsafe {
                    (*unsafe {
                                    (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                }).argv_index = 1
                };
                unsafe {
                    (*unsafe {
                                    (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                }).omit = 1 as u8
                };
                unsafe { (*p_idx_info_1).estimated_cost = 1 as f64 };
                unsafe {
                    (*p_idx_info_1).estimated_rows = 10 as sqlite3_int64
                };
                unsafe { (*p_idx_info_1).idx_num = 1 };
                return 0;
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p_idx_info_1).idx_num = 0 };
    unsafe { (*p_idx_info_1).estimated_cost = 2147483647 as f64 };
    unsafe { (*p_idx_info_1).estimated_rows = 2147483647 as sqlite3_int64 };
    return 19;
}
static mut deltaparsevtab_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: None,
        x_connect: Some(deltaparsevtab_connect),
        x_best_index: Some(deltaparsevtab_best_index),
        x_disconnect: Some(deltaparsevtab_disconnect),
        x_destroy: None,
        x_open: Some(deltaparsevtab_open),
        x_close: Some(deltaparsevtab_close),
        x_filter: Some(deltaparsevtab_filter),
        x_next: Some(deltaparsevtab_next),
        x_eof: Some(deltaparsevtab_eof),
        x_column: Some(deltaparsevtab_column),
        x_rowid: Some(deltaparsevtab_rowid),
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
pub extern "C" fn sqlite3_fossildelta_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        { let _ = pz_err_msg_1; };
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"delta_create".as_ptr() as *mut i8 as *const i8, 2, enc,
                    core::ptr::null_mut(), Some(delta_create_func), None, None)
            };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"delta_apply".as_ptr() as *mut i8 as *const i8, 2, enc,
                        core::ptr::null_mut(), Some(delta_apply_func), None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"delta_output_size".as_ptr() as *mut i8 as *const i8, 1,
                        enc, core::ptr::null_mut(), Some(delta_output_size_func),
                        None, None)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_module(db,
                        c"delta_parse".as_ptr() as *mut i8 as *const i8,
                        &raw mut deltaparsevtab_module as *const sqlite3_module,
                        core::ptr::null_mut())
                };
        }
        return rc;
    }
}
static z_digits: [i8; 65] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 65 as i8, 66 as i8, 67 as i8,
            68 as i8, 69 as i8, 70 as i8, 71 as i8, 72 as i8, 73 as i8,
            74 as i8, 75 as i8, 76 as i8, 77 as i8, 78 as i8, 79 as i8,
            80 as i8, 81 as i8, 82 as i8, 83 as i8, 84 as i8, 85 as i8,
            86 as i8, 87 as i8, 88 as i8, 89 as i8, 90 as i8, 95 as i8,
            97 as i8, 98 as i8, 99 as i8, 100 as i8, 101 as i8, 102 as i8,
            103 as i8, 104 as i8, 105 as i8, 106 as i8, 107 as i8, 108 as i8,
            109 as i8, 110 as i8, 111 as i8, 112 as i8, 113 as i8, 114 as i8,
            115 as i8, 116 as i8, 117 as i8, 118 as i8, 119 as i8, 120 as i8,
            121 as i8, 122 as i8, 126 as i8, 0 as i8];
static z_value: [i8; 256] =
    [-1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, 0 as i8,
            1 as i8, 2 as i8, 3 as i8, 4 as i8, 5 as i8, 6 as i8, 7 as i8,
            8 as i8, 9 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, 10 as i8, 11 as i8, 12 as i8,
            13 as i8, 14 as i8, 15 as i8, 16 as i8, 17 as i8, 18 as i8,
            19 as i8, 20 as i8, 21 as i8, 22 as i8, 23 as i8, 24 as i8,
            25 as i8, 26 as i8, 27 as i8, 28 as i8, 29 as i8, 30 as i8,
            31 as i8, 32 as i8, 33 as i8, 34 as i8, 35 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, 36 as i8, -1 as i8, 37 as i8,
            38 as i8, 39 as i8, 40 as i8, 41 as i8, 42 as i8, 43 as i8,
            44 as i8, 45 as i8, 46 as i8, 47 as i8, 48 as i8, 49 as i8,
            50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8,
            56 as i8, 57 as i8, 58 as i8, 59 as i8, 60 as i8, 61 as i8,
            62 as i8, -1 as i8, -1 as i8, -1 as i8, 63 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8];
static byte_order_test: i32 = 1 as i32;
static enc: i32 = (1 | 2097152) as i32;
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
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}