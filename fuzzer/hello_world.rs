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
struct fuzzer_vtab {
    base: sqlite3_vtab,
    z_class_name: *mut i8,
    p_rule: *mut fuzzer_rule,
    n_cursor: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fuzzer_rule {
    p_next: *mut fuzzer_rule,
    z_from: *mut i8,
    r_cost: fuzzer_cost,
    n_from: fuzzer_len,
    n_to: fuzzer_len,
    i_ruleset: fuzzer_ruleid,
    z_to: [i8; 4],
}
type fuzzer_cost = i32;
type fuzzer_len = i8;
type fuzzer_ruleid = i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct fuzzer_cursor {
    base: sqlite3_vtab_cursor,
    i_rowid: sqlite3_int64,
    p_vtab: *mut fuzzer_vtab,
    r_limit: fuzzer_cost,
    p_stem: *mut fuzzer_stem,
    p_done: *mut fuzzer_stem,
    a_queue: [*mut fuzzer_stem; 20],
    mx_queue: i32,
    z_buf: *mut i8,
    n_buf: i32,
    n_stem: i32,
    i_ruleset: i32,
    null_rule: fuzzer_rule,
    ap_hash: [*mut fuzzer_stem; 4001],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fuzzer_stem {
    z_basis: *mut i8,
    p_rule: *const fuzzer_rule,
    p_next: *mut fuzzer_stem,
    p_hash: *mut fuzzer_stem,
    r_base_cost: fuzzer_cost,
    r_cost_x: fuzzer_cost,
    n_basis: fuzzer_len,
    n: fuzzer_len,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fuzzer_seen {
    _opaque: [u8; 0],
}
extern "C" fn fuzzer_merge_rules(mut p_a_1: *mut fuzzer_rule,
    mut p_b_1: *mut fuzzer_rule) -> *mut fuzzer_rule {
    let mut head: fuzzer_rule = unsafe { core::mem::zeroed() };
    let mut p_tail: *mut fuzzer_rule = core::ptr::null_mut();
    p_tail = &mut head;
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if unsafe { (*p_a_1).r_cost } <= unsafe { (*p_b_1).r_cost } {
            unsafe { (*p_tail).p_next = p_a_1 };
            p_tail = p_a_1;
            p_a_1 = unsafe { (*p_a_1).p_next };
        } else {
            unsafe { (*p_tail).p_next = p_b_1 };
            p_tail = p_b_1;
            p_b_1 = unsafe { (*p_b_1).p_next };
        }
    }
    if p_a_1 == core::ptr::null_mut() {
        unsafe { (*p_tail).p_next = p_b_1 };
    } else { unsafe { (*p_tail).p_next = p_a_1 }; }
    return head.p_next;
}
extern "C" fn fuzzer_load_one_rule(p: &fuzzer_vtab,
    p_stmt_1: *mut sqlite3_stmt, pp_rule_1: &mut *mut fuzzer_rule,
    pz_err_1: &mut *mut i8) -> i32 {
    let i_ruleset: sqlite3_int64 =
        unsafe { sqlite3_column_int64(p_stmt_1, 0) };
    let mut z_from: *const i8 =
        unsafe { sqlite3_column_text(p_stmt_1, 1) } as *const i8;
    let mut z_to: *const i8 =
        unsafe { sqlite3_column_text(p_stmt_1, 2) } as *const i8;
    let n_cost: i32 = unsafe { sqlite3_column_int(p_stmt_1, 3) };
    let mut rc: i32 = 0;
    let mut n_from: i32 = 0;
    let mut n_to: i32 = 0;
    let mut p_rule: *mut fuzzer_rule = core::ptr::null_mut();
    if z_from == core::ptr::null() {
        z_from = c"".as_ptr() as *mut i8 as *const i8;
    }
    if z_to == core::ptr::null() {
        z_to = c"".as_ptr() as *mut i8 as *const i8;
    }
    n_from = unsafe { strlen(z_from) } as i32;
    n_to = unsafe { strlen(z_to) } as i32;
    if unsafe { strcmp(z_from, z_to) } == 0 {
        *pp_rule_1 = core::ptr::null_mut();
        return 0;
    }
    if n_cost <= 0 || n_cost > 1000 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: cost must be between 1 and %d".as_ptr()
                            as *mut i8 as *const i8, (*p).z_class_name, 1000)
            };
        rc = 1;
    } else if n_from > 50 || n_to > 50 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: maximum string length is %d".as_ptr() as
                            *mut i8 as *const i8, (*p).z_class_name, 50)
            };
        rc = 1;
    } else if i_ruleset < 0 as i64 || i_ruleset > 2147483647 as i64 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: ruleset must be between 0 and %d".as_ptr()
                            as *mut i8 as *const i8, (*p).z_class_name, 2147483647)
            };
        rc = 1;
    } else {
        p_rule =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<fuzzer_rule>() as u64
                                + n_from as u64 + n_to as u64)
                } as *mut fuzzer_rule;
        if p_rule == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe {
                memset(p_rule as *mut (), 0,
                    core::mem::size_of::<fuzzer_rule>() as u64)
            };
            unsafe {
                (*p_rule).z_from =
                    unsafe { &raw mut (*p_rule).z_to[0 as usize] } as *mut i8
            };
            unsafe {
                {
                    let __n = n_to + 1;
                    let __p = &mut (*p_rule).z_from;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                }
            };
            unsafe { (*p_rule).n_from = n_from as fuzzer_len };
            unsafe {
                memcpy(unsafe { (*p_rule).z_from } as *mut (),
                    z_from as *const (), (n_from + 1) as u64)
            };
            unsafe {
                memcpy(unsafe { &raw mut (*p_rule).z_to[0 as usize] } as
                            *mut i8 as *mut (), z_to as *const (), (n_to + 1) as u64)
            };
            unsafe { (*p_rule).n_to = n_to as fuzzer_len };
            unsafe { (*p_rule).r_cost = n_cost };
            unsafe { (*p_rule).i_ruleset = i_ruleset as i32 };
        }
    }
    *pp_rule_1 = p_rule;
    return rc;
}
extern "C" fn fuzzer_load_rules(db: *mut sqlite3, p: *mut fuzzer_vtab,
    z_db_1: *const i8, z_data_1: *const i8, pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p_head: *mut fuzzer_rule = core::ptr::null_mut();
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT * FROM %Q.%Q".as_ptr() as *mut i8 as
                    *const i8, z_db_1, z_data_1)
        };
    if z_sql == core::ptr::null_mut() {
        rc = 7;
    } else {
        let mut rc2: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s: %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).z_class_name }, unsafe { sqlite3_errmsg(db) })
                    }
            };
        } else if unsafe { sqlite3_column_count(p_stmt) } != 4 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s: %s has %d columns, expected 4".as_ptr()
                                    as *mut i8 as *const i8, unsafe { (*p).z_class_name },
                            z_data_1, unsafe { sqlite3_column_count(p_stmt) })
                    }
            };
            rc = 1;
        } else {
            while rc == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
                let mut p_rule: *mut fuzzer_rule = core::ptr::null_mut();
                rc =
                    fuzzer_load_one_rule(unsafe { &*p }, p_stmt, &mut p_rule,
                        unsafe { &mut *pz_err_1 });
                if !(p_rule).is_null() {
                    unsafe { (*p_rule).p_next = p_head };
                    p_head = p_rule;
                }
            }
        }
        rc2 = unsafe { sqlite3_finalize(p_stmt) };
        if rc == 0 { rc = rc2; }
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc == 0 {
        let mut i: u32 = 0 as u32;
        let mut p_x: *mut fuzzer_rule = core::ptr::null_mut();
        let mut a: [*mut fuzzer_rule; 15] = [core::ptr::null_mut(); 15];
        {
            i = 0 as u32;
            '__b2: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*mut fuzzer_rule; 15]>() as u64 /
                                    core::mem::size_of::<*mut fuzzer_rule>() as u64) {
                    break '__b2;
                }
                '__c2: loop {
                    a[i as usize] = core::ptr::null_mut();
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        while { p_x = p_head; p_x } != core::ptr::null_mut() {
            p_head = unsafe { (*p_x).p_next };
            unsafe { (*p_x).p_next = core::ptr::null_mut() };
            {
                i = 0 as u32;
                '__b4: loop {
                    if !(!(a[i as usize]).is_null() &&
                                    (i as u64) <
                                        core::mem::size_of::<[*mut fuzzer_rule; 15]>() as u64 /
                                                core::mem::size_of::<*mut fuzzer_rule>() as u64 - 1 as u64)
                        {
                        break '__b4;
                    }
                    '__c4: loop {
                        p_x = fuzzer_merge_rules(a[i as usize], p_x);
                        a[i as usize] = core::ptr::null_mut();
                        break '__c4;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            a[i as usize] = fuzzer_merge_rules(a[i as usize], p_x);
        }
        {
            { p_x = a[0 as usize]; i = 1 as u32 };
            '__b5: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*mut fuzzer_rule; 15]>() as u64 /
                                    core::mem::size_of::<*mut fuzzer_rule>() as u64) {
                    break '__b5;
                }
                '__c5: loop {
                    p_x = fuzzer_merge_rules(a[i as usize], p_x);
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            (*p).p_rule = fuzzer_merge_rules(unsafe { (*p).p_rule }, p_x)
        };
    } else { { let _ = 0; }; unsafe { (*p).p_rule = p_head }; }
    return rc;
}
extern "C" fn fuzzer_dequote(z_in_1: *const i8) -> *mut i8 {
    let mut n_in: sqlite3_int64 = 0 as sqlite3_int64;
    let mut z_out: *mut i8 = core::ptr::null_mut();
    n_in = unsafe { strlen(z_in_1) } as sqlite3_int64;
    z_out =
        unsafe {
                sqlite3_malloc64((n_in + 1 as sqlite3_int64) as
                        sqlite3_uint64)
            } as *mut i8;
    if !(z_out).is_null() {
        let mut q: i8 = unsafe { *z_in_1.offset(0 as isize) } as i8;
        if q as i32 != '[' as i32 && q as i32 != '\'' as i32 &&
                    q as i32 != '\"' as i32 && q as i32 != '`' as i32 {
            unsafe {
                memcpy(z_out as *mut (), z_in_1 as *const (),
                    (n_in + 1 as sqlite3_int64) as u64)
            };
        } else {
            let mut i_out: i32 = 0;
            let mut i_in: i32 = 0;
            if q as i32 == '[' as i32 { q = ']' as i32 as i8; }
            {
                i_in = 1;
                '__b6: loop {
                    if !((i_in as sqlite3_int64) < n_in) { break '__b6; }
                    '__c6: loop {
                        if unsafe { *z_in_1.offset(i_in as isize) } as i32 ==
                                q as i32 {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                        }
                        unsafe {
                            *z_out.offset({
                                                let __p = &mut i_out;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = unsafe { *z_in_1.offset(i_in as isize) } as i8
                        };
                        break '__c6;
                    }
                    { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                }
            }
            { let _ = 0; };
            unsafe { *z_out.offset(i_out as isize) = 0 as i8 };
        }
        { let _ = 0; };
    }
    return z_out;
}
extern "C" fn fuzzer_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p: *mut fuzzer_vtab = p_vtab_1 as *mut fuzzer_vtab;
    { let _ = 0; };
    while !(unsafe { (*p).p_rule }).is_null() {
        let p_rule: *mut fuzzer_rule = unsafe { (*p).p_rule };
        unsafe { (*p).p_rule = unsafe { (*p_rule).p_next } };
        unsafe { sqlite3_free(p_rule as *mut ()) };
    }
    unsafe { sqlite3_free(p as *mut ()) };
    return 0;
}
extern "C" fn fuzzer_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut p_new: *mut fuzzer_vtab = core::ptr::null_mut();
    let z_module: *const i8 = unsafe { *argv.offset(0 as isize) };
    let z_db: *const i8 = unsafe { *argv.offset(1 as isize) };
    if argc != 4 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"%s: wrong number of CREATE VIRTUAL TABLE arguments".as_ptr()
                                as *mut i8 as *const i8, z_module)
                }
        };
        rc = 1;
    } else {
        let mut n_module: sqlite3_int64 = 0 as sqlite3_int64;
        n_module = unsafe { strlen(z_module) } as sqlite3_int64;
        p_new =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<fuzzer_vtab>() as u64
                                + n_module as u64 + 1 as u64)
                } as *mut fuzzer_vtab;
        if p_new == core::ptr::null_mut() {
            rc = 7;
        } else {
            let mut z_tab: *mut i8 = core::ptr::null_mut();
            unsafe {
                memset(p_new as *mut (), 0,
                    core::mem::size_of::<fuzzer_vtab>() as u64)
            };
            unsafe {
                (*p_new).z_class_name =
                    unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
            };
            unsafe {
                memcpy(unsafe { (*p_new).z_class_name } as *mut (),
                    z_module as *const (),
                    (n_module + 1 as sqlite3_int64) as u64)
            };
            z_tab = fuzzer_dequote(unsafe { *argv.offset(3 as isize) });
            if z_tab == core::ptr::null_mut() {
                rc = 7;
            } else {
                rc =
                    fuzzer_load_rules(db, p_new, z_db, z_tab as *const i8,
                        pz_err_1);
                unsafe { sqlite3_free(z_tab as *mut ()) };
            }
            if rc == 0 {
                rc =
                    unsafe {
                        sqlite3_declare_vtab(db,
                            c"CREATE TABLE x(word,distance,ruleset)".as_ptr() as *mut i8
                                as *const i8)
                    };
            }
            if rc != 0 {
                fuzzer_disconnect(p_new as *mut sqlite3_vtab);
                p_new = core::ptr::null_mut();
            } else { unsafe { sqlite3_vtab_config(db, 2) }; }
        }
    }
    unsafe { *pp_vtab_1 = p_new as *mut sqlite3_vtab };
    return rc;
}
extern "C" fn fuzzer_open(p_v_tab_1: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let p: *mut fuzzer_vtab = p_v_tab_1 as *mut fuzzer_vtab;
    let mut p_cur: *mut fuzzer_cursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<fuzzer_cursor>() as
                        sqlite3_uint64)
            } as *mut fuzzer_cursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<fuzzer_cursor>() as u64)
    };
    unsafe { (*p_cur).p_vtab = p };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    {
        let __p = unsafe { &mut (*p).n_cursor };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return 0;
}
extern "C" fn fuzzer_clear_stem_list(mut p_stem_1: *mut fuzzer_stem) -> () {
    while !(p_stem_1).is_null() {
        let p_next: *mut fuzzer_stem = unsafe { (*p_stem_1).p_next };
        unsafe { sqlite3_free(p_stem_1 as *mut ()) };
        p_stem_1 = p_next;
    }
}
extern "C" fn fuzzer_clear_cursor(p_cur_1: &mut fuzzer_cursor,
    clear_hash_1: i32) -> () {
    let mut i: i32 = 0;
    fuzzer_clear_stem_list((*p_cur_1).p_stem);
    fuzzer_clear_stem_list((*p_cur_1).p_done);
    {
        i = 0;
        '__b9: loop {
            if !(i < 20) { break '__b9; }
            '__c9: loop {
                fuzzer_clear_stem_list((*p_cur_1).a_queue[i as usize]);
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_cur_1).r_limit = 0 as fuzzer_cost;
    if clear_hash_1 != 0 && (*p_cur_1).n_stem != 0 {
        (*p_cur_1).mx_queue = 0;
        (*p_cur_1).p_stem = core::ptr::null_mut();
        (*p_cur_1).p_done = core::ptr::null_mut();
        unsafe {
            memset(&raw mut (*p_cur_1).a_queue[0 as usize] as
                        *mut *mut fuzzer_stem as *mut (), 0,
                core::mem::size_of::<[*mut fuzzer_stem; 20]>() as u64)
        };
        unsafe {
            memset(&raw mut (*p_cur_1).ap_hash[0 as usize] as
                        *mut *mut fuzzer_stem as *mut (), 0,
                core::mem::size_of::<[*mut fuzzer_stem; 4001]>() as u64)
        };
    }
    (*p_cur_1).n_stem = 0;
}
extern "C" fn fuzzer_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
    fuzzer_clear_cursor(unsafe { &mut *p_cur }, 0);
    unsafe { sqlite3_free(unsafe { (*p_cur).z_buf } as *mut ()) };
    {
        let __p = unsafe { &mut (*unsafe { (*p_cur).p_vtab }).n_cursor };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}
extern "C" fn fuzzer_render(p_stem_1: &fuzzer_stem, pz_buf_1: &mut *mut i8,
    pn_buf_1: &mut i32) -> i32 {
    let p_rule: *const fuzzer_rule = (*p_stem_1).p_rule;
    let mut n: sqlite3_int64 = 0 as sqlite3_int64;
    let mut z: *mut i8 = core::ptr::null_mut();
    n =
        ((*p_stem_1).n_basis as i32 + unsafe { (*p_rule).n_to } as i32 -
                unsafe { (*p_rule).n_from } as i32) as sqlite3_int64;
    if (*pn_buf_1 as sqlite3_int64) < n + 1 as sqlite3_int64 {
        *pz_buf_1 =
            unsafe {
                    sqlite3_realloc64(*pz_buf_1 as *mut (),
                        (n + 100 as sqlite3_int64) as sqlite3_uint64)
                } as *mut i8;
        if *pz_buf_1 == core::ptr::null_mut() { return 7; }
        *pn_buf_1 = (n + 100 as sqlite3_int64) as i32;
    }
    n = (*p_stem_1).n as sqlite3_int64;
    z = *pz_buf_1;
    if n < 0 as i64 {
        unsafe {
            memcpy(z as *mut (), (*p_stem_1).z_basis as *const (),
                ((*p_stem_1).n_basis as i32 + 1) as u64)
        };
    } else {
        unsafe {
            memcpy(z as *mut (), (*p_stem_1).z_basis as *const (), n as u64)
        };
        unsafe {
            memcpy(unsafe { &raw mut *z.offset(n as isize) } as *mut (),
                unsafe { &raw const (*p_rule).z_to[0 as usize] } as *const i8
                    as *const (), unsafe { (*p_rule).n_to } as u64)
        };
        unsafe {
            memcpy(unsafe {
                        &raw mut *z.offset((n +
                                            unsafe { (*p_rule).n_to } as sqlite3_int64) as isize)
                    } as *mut (),
                unsafe {
                        &raw mut *(*p_stem_1).z_basis.offset((n +
                                            unsafe { (*p_rule).n_from } as sqlite3_int64) as isize)
                    } as *const (),
                ((*p_stem_1).n_basis as sqlite3_int64 - n -
                            unsafe { (*p_rule).n_from } as sqlite3_int64 +
                        1 as sqlite3_int64) as u64)
        };
    }
    { let _ = 0; };
    return 0;
}
extern "C" fn fuzzer_hash(mut z: *const i8) -> u32 {
    let mut h: u32 = 0 as u32;
    while unsafe { *z } != 0 {
        h =
            h << 3 ^ h >> 29 ^
                unsafe {
                        *{
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as u32;
    }
    return h % 4001 as u32;
}
extern "C" fn fuzzer_cost_1(p_stem_1: &mut fuzzer_stem) -> fuzzer_cost {
    return {
            (*p_stem_1).r_cost_x =
                (*p_stem_1).r_base_cost +
                    unsafe { (*(*p_stem_1).p_rule).r_cost } as fuzzer_cost;
            (*p_stem_1).r_cost_x
        };
}
extern "C" fn fuzzer_seen_2(p_cur_1: &mut fuzzer_cursor,
    p_stem_1: *mut fuzzer_stem) -> i32 {
    let mut h: u32 = 0 as u32;
    let mut p_lookup: *const fuzzer_stem = core::ptr::null();
    if fuzzer_render(unsafe { &*p_stem_1 }, &mut (*p_cur_1).z_buf,
                &mut (*p_cur_1).n_buf) == 7 {
        return -1;
    }
    h = fuzzer_hash((*p_cur_1).z_buf as *const i8);
    p_lookup = (*p_cur_1).ap_hash[h as usize];
    while !(p_lookup).is_null() &&
            unsafe {
                    strcmp(unsafe { (*p_lookup).z_basis } as *const i8,
                        (*p_cur_1).z_buf as *const i8)
                } != 0 {
        p_lookup = unsafe { (*p_lookup).p_hash };
    }
    return (p_lookup != core::ptr::null_mut()) as i32;
}
extern "C" fn fuzzer_skip_rule(p_rule_1: *const fuzzer_rule,
    p_stem_1: &fuzzer_stem, i_ruleset_1: i32) -> i32 {
    return (!(p_rule_1).is_null() &&
                (unsafe { (*p_rule_1).i_ruleset } as fuzzer_ruleid !=
                        i_ruleset_1 ||
                    (*p_stem_1).n_basis as i32 +
                                unsafe { (*p_rule_1).n_to } as i32 -
                            unsafe { (*p_rule_1).n_from } as i32 > 100)) as i32;
}
extern "C" fn fuzzer_advance(p_cur_1: *mut fuzzer_cursor,
    p_stem_1: *mut fuzzer_stem) -> i32 {
    let mut p_rule: *const fuzzer_rule = core::ptr::null();
    while { p_rule = unsafe { (*p_stem_1).p_rule }; p_rule } !=
            core::ptr::null() {
        { let _ = 0; };
        while (unsafe { (*p_stem_1).n } as i32) <
                unsafe { (*p_stem_1).n_basis } as i32 -
                    unsafe { (*p_rule).n_from } as i32 {
            {
                let __p = unsafe { &mut (*p_stem_1).n };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p_rule).n_from } as i32 == 0 ||
                    unsafe {
                            memcmp(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_stem_1).z_basis.offset(unsafe { (*p_stem_1).n } as
                                                            isize)
                                                }
                                    } as *const (), unsafe { (*p_rule).z_from } as *const (),
                                unsafe { (*p_rule).n_from } as u64)
                        } == 0 {
                let rc: i32 =
                    fuzzer_seen_2(unsafe { &mut *p_cur_1 }, p_stem_1);
                if rc < 0 { return -1; }
                if rc == 0 {
                    fuzzer_cost_1(unsafe { &mut *p_stem_1 });
                    return 1;
                }
            }
        }
        unsafe { (*p_stem_1).n = -1 as fuzzer_len };
        '__b14: loop {
            '__c14: loop {
                p_rule = unsafe { (*p_rule).p_next } as *const fuzzer_rule;
                break '__c14;
            }
            if !(fuzzer_skip_rule(p_rule, unsafe { &*p_stem_1 },
                                unsafe { (*p_cur_1).i_ruleset }) != 0) {
                break '__b14;
            }
        }
        unsafe { (*p_stem_1).p_rule = p_rule };
        if !(p_rule).is_null() &&
                fuzzer_cost_1(unsafe { &mut *p_stem_1 }) >
                    unsafe { (*p_cur_1).r_limit } {
            unsafe { (*p_stem_1).p_rule = core::ptr::null() };
        }
    }
    return 0;
}
extern "C" fn fuzzer_merge_stems(mut p_a_1: *mut fuzzer_stem,
    mut p_b_1: *mut fuzzer_stem) -> *mut fuzzer_stem {
    let mut head: fuzzer_stem = unsafe { core::mem::zeroed() };
    let mut p_tail: *mut fuzzer_stem = core::ptr::null_mut();
    p_tail = &mut head;
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if unsafe { (*p_a_1).r_cost_x } <= unsafe { (*p_b_1).r_cost_x } {
            unsafe { (*p_tail).p_next = p_a_1 };
            p_tail = p_a_1;
            p_a_1 = unsafe { (*p_a_1).p_next };
        } else {
            unsafe { (*p_tail).p_next = p_b_1 };
            p_tail = p_b_1;
            p_b_1 = unsafe { (*p_b_1).p_next };
        }
    }
    if p_a_1 == core::ptr::null_mut() {
        unsafe { (*p_tail).p_next = p_b_1 };
    } else { unsafe { (*p_tail).p_next = p_a_1 }; }
    return head.p_next;
}
extern "C" fn fuzzer_lowest_cost_stem(p_cur_1: &mut fuzzer_cursor)
    -> *mut fuzzer_stem {
    let mut p_best: *mut fuzzer_stem = core::ptr::null_mut();
    let mut p_x: *mut fuzzer_stem = core::ptr::null_mut();
    let mut i_best: i32 = 0;
    let mut i: i32 = 0;
    if (*p_cur_1).p_stem == core::ptr::null_mut() {
        i_best = -1;
        p_best = core::ptr::null_mut();
        {
            i = 0;
            '__b16: loop {
                if !(i <= (*p_cur_1).mx_queue) { break '__b16; }
                '__c16: loop {
                    p_x = (*p_cur_1).a_queue[i as usize];
                    if p_x == core::ptr::null_mut() { break '__c16; }
                    if p_best == core::ptr::null_mut() ||
                            unsafe { (*p_best).r_cost_x } > unsafe { (*p_x).r_cost_x } {
                        p_best = p_x;
                        i_best = i;
                    }
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if !(p_best).is_null() {
            (*p_cur_1).a_queue[i_best as usize] = unsafe { (*p_best).p_next };
            unsafe { (*p_best).p_next = core::ptr::null_mut() };
            (*p_cur_1).p_stem = p_best;
        }
    }
    return (*p_cur_1).p_stem;
}
extern "C" fn fuzzer_insert(p_cur_1: *mut fuzzer_cursor,
    mut p_new_1: *mut fuzzer_stem) -> *mut fuzzer_stem {
    let mut p_x: *mut fuzzer_stem = core::ptr::null_mut();
    let mut i: i32 = 0;
    if { p_x = unsafe { (*p_cur_1).p_stem }; p_x } != core::ptr::null_mut() &&
            unsafe { (*p_x).r_cost_x } > unsafe { (*p_new_1).r_cost_x } {
        unsafe { (*p_new_1).p_next = core::ptr::null_mut() };
        unsafe { (*p_cur_1).p_stem = p_new_1 };
        p_new_1 = p_x;
    }
    unsafe { (*p_new_1).p_next = core::ptr::null_mut() };
    p_x = p_new_1;
    {
        i = 0;
        '__b17: loop {
            if !(i <= unsafe { (*p_cur_1).mx_queue }) { break '__b17; }
            '__c17: loop {
                if !(unsafe { (*p_cur_1).a_queue[i as usize] }).is_null() {
                    p_x =
                        fuzzer_merge_stems(p_x,
                            unsafe { (*p_cur_1).a_queue[i as usize] });
                    unsafe {
                        (*p_cur_1).a_queue[i as usize] = core::ptr::null_mut()
                    };
                } else {
                    unsafe { (*p_cur_1).a_queue[i as usize] = p_x };
                    break '__b17;
                }
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i > unsafe { (*p_cur_1).mx_queue } {
        if i < 20 {
            unsafe { (*p_cur_1).mx_queue = i };
            unsafe { (*p_cur_1).a_queue[i as usize] = p_x };
        } else {
            { let _ = 0; };
            p_x =
                fuzzer_merge_stems(p_x,
                    unsafe { (*p_cur_1).a_queue[(20 - 1) as usize] });
            unsafe { (*p_cur_1).a_queue[(20 - 1) as usize] = p_x };
        }
    }
    return fuzzer_lowest_cost_stem(unsafe { &mut *p_cur_1 });
}
extern "C" fn fuzzer_new_stem(p_cur_1: &mut fuzzer_cursor,
    z_word_1: *const i8, r_base_cost_1: fuzzer_cost) -> *mut fuzzer_stem {
    let mut p_new: *mut fuzzer_stem = core::ptr::null_mut();
    let mut p_rule: *const fuzzer_rule = core::ptr::null();
    let mut h: u32 = 0 as u32;
    p_new =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<fuzzer_stem>() as u64 +
                            unsafe { strlen(z_word_1) } + 1 as u64)
            } as *mut fuzzer_stem;
    if p_new == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe {
        memset(p_new as *mut (), 0,
            core::mem::size_of::<fuzzer_stem>() as u64)
    };
    unsafe {
        (*p_new).z_basis =
            unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
    };
    unsafe { (*p_new).n_basis = unsafe { strlen(z_word_1) } as fuzzer_len };
    unsafe {
        memcpy(unsafe { (*p_new).z_basis } as *mut (), z_word_1 as *const (),
            (unsafe { (*p_new).n_basis } as i32 + 1) as u64)
    };
    p_rule = unsafe { (*(*p_cur_1).p_vtab).p_rule };
    while fuzzer_skip_rule(p_rule as *const fuzzer_rule, unsafe { &*p_new },
                (*p_cur_1).i_ruleset) != 0 {
        p_rule = unsafe { (*p_rule).p_next };
    }
    unsafe { (*p_new).p_rule = p_rule as *const fuzzer_rule };
    unsafe { (*p_new).n = -1 as fuzzer_len };
    unsafe {
        (*p_new).r_base_cost =
            {
                unsafe { (*p_new).r_cost_x = r_base_cost_1 };
                unsafe { (*p_new).r_cost_x }
            }
    };
    h = fuzzer_hash(unsafe { (*p_new).z_basis } as *const i8);
    unsafe { (*p_new).p_hash = (*p_cur_1).ap_hash[h as usize] };
    (*p_cur_1).ap_hash[h as usize] = p_new;
    { let __p = &mut (*p_cur_1).n_stem; let __t = *__p; *__p += 1; __t };
    return p_new;
}
extern "C" fn fuzzer_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
    let mut rc: i32 = 0;
    let mut p_stem: *mut fuzzer_stem = core::ptr::null_mut();
    let mut p_new: *mut fuzzer_stem = core::ptr::null_mut();
    {
        let __p = unsafe { &mut (*p_cur).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    p_stem = unsafe { (*p_cur).p_stem };
    if unsafe { (*p_stem).r_cost_x } > 0 {
        rc =
            fuzzer_render(unsafe { &*p_stem }, unsafe { &mut (*p_cur).z_buf },
                unsafe { &mut (*p_cur).n_buf });
        if rc == 7 { return 7; }
        p_new =
            fuzzer_new_stem(unsafe { &mut *p_cur },
                unsafe { (*p_cur).z_buf } as *const i8,
                unsafe { (*p_stem).r_cost_x });
        if !(p_new).is_null() {
            if fuzzer_advance(p_cur, p_new) == 0 {
                unsafe { (*p_new).p_next = unsafe { (*p_cur).p_done } };
                unsafe { (*p_cur).p_done = p_new };
            } else { if fuzzer_insert(p_cur, p_new) == p_new { return 0; } }
        } else { return 7; }
    }
    while { p_stem = unsafe { (*p_cur).p_stem }; p_stem } !=
            core::ptr::null_mut() {
        let res: i32 = fuzzer_advance(p_cur, p_stem);
        if res < 0 {
            return 7;
        } else if res > 0 {
            unsafe { (*p_cur).p_stem = core::ptr::null_mut() };
            p_stem = fuzzer_insert(p_cur, p_stem);
            if { rc = fuzzer_seen_2(unsafe { &mut *p_cur }, p_stem); rc } != 0
                {
                if rc < 0 { return 7; }
                continue;
            }
            return 0;
        }
        unsafe { (*p_cur).p_stem = core::ptr::null_mut() };
        unsafe { (*p_stem).p_next = unsafe { (*p_cur).p_done } };
        unsafe { (*p_cur).p_done = p_stem };
        if !(fuzzer_lowest_cost_stem(unsafe { &mut *p_cur })).is_null() {
            rc =
                fuzzer_seen_2(unsafe { &mut *p_cur },
                    unsafe { (*p_cur).p_stem });
            if rc < 0 { return 7; }
            if rc == 0 { return 0; }
        }
    }
    unsafe { (*p_cur).r_limit = 0 as fuzzer_cost };
    return 0;
}
extern "C" fn fuzzer_filter(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let p_cur: *mut fuzzer_cursor = p_vtab_cursor_1 as *mut fuzzer_cursor;
    let mut z_word: *const i8 = c"".as_ptr() as *mut i8 as *const i8;
    let mut p_stem: *mut fuzzer_stem = core::ptr::null_mut();
    let mut idx: i32 = 0;
    fuzzer_clear_cursor(unsafe { &mut *p_cur }, 1);
    unsafe { (*p_cur).r_limit = 2147483647 };
    idx = 0;
    if idx_num_1 & 1 != 0 {
        z_word =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        if z_word == core::ptr::null() {
            z_word = c"".as_ptr() as *mut i8 as *const i8;
        }
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    if idx_num_1 & 2 != 0 {
        unsafe {
            (*p_cur).r_limit =
                unsafe {
                        sqlite3_value_int(unsafe { *argv.offset(idx as isize) })
                    } as fuzzer_cost
        };
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    if idx_num_1 & 4 != 0 {
        unsafe {
            (*p_cur).i_ruleset =
                unsafe {
                        sqlite3_value_int(unsafe { *argv.offset(idx as isize) })
                    } as fuzzer_cost
        };
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    unsafe {
        (*p_cur).null_rule.p_next =
            unsafe { (*unsafe { (*p_cur).p_vtab }).p_rule }
    };
    unsafe { (*p_cur).null_rule.r_cost = 0 };
    unsafe { (*p_cur).null_rule.n_from = 0 as fuzzer_len };
    unsafe { (*p_cur).null_rule.n_to = 0 as fuzzer_len };
    unsafe { (*p_cur).null_rule.z_from = c"".as_ptr() as *mut i8 };
    unsafe { (*p_cur).i_rowid = 1 as sqlite3_int64 };
    { let _ = 0; };
    if (unsafe { strlen(z_word) } as i32) < 100 {
        unsafe {
            (*p_cur).p_stem =
                {
                    p_stem =
                        fuzzer_new_stem(unsafe { &mut *p_cur }, z_word,
                            0 as fuzzer_cost);
                    p_stem
                }
        };
        if p_stem == core::ptr::null_mut() { return 7; }
        unsafe {
            (*p_stem).p_rule =
                unsafe { &raw mut (*p_cur).null_rule } as *const fuzzer_rule
        };
        unsafe { (*p_stem).n = unsafe { (*p_stem).n_basis } };
    } else { unsafe { (*p_cur).r_limit = 0 }; }
    return 0;
}
extern "C" fn fuzzer_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_cur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
    if i == 0 {
        if fuzzer_render(unsafe { &*unsafe { (*p_cur).p_stem } },
                    unsafe { &mut (*p_cur).z_buf },
                    unsafe { &mut (*p_cur).n_buf }) == 7 {
            return 7;
        }
        unsafe {
            sqlite3_result_text(ctx, unsafe { (*p_cur).z_buf } as *const i8,
                -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    } else if i == 1 {
        unsafe {
            sqlite3_result_int(ctx,
                unsafe { (*unsafe { (*p_cur).p_stem }).r_cost_x })
        };
    } else { unsafe { sqlite3_result_null(ctx) }; }
    return 0;
}
extern "C" fn fuzzer_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p_cur: *const fuzzer_cursor =
        cur as *mut fuzzer_cursor as *const fuzzer_cursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}
extern "C" fn fuzzer_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const fuzzer_cursor =
        cur as *mut fuzzer_cursor as *const fuzzer_cursor;
    return (unsafe { (*p_cur).r_limit } <= 0 as fuzzer_cost) as i32;
}
extern "C" fn fuzzer_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i_plan: i32 = 0;
    let mut i_dist_term: i32 = -1;
    let mut i_ruleset_term: i32 = -1;
    let mut i: i32 = 0;
    let mut seen_match: i32 = 0;
    let mut p_constraint: *const sqlite3_index_constraint = core::ptr::null();
    let mut r_cost: f64 = 1000000000000.0;
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const sqlite3_index_constraint;
    {
        i = 0;
        '__b20: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b20;
            }
            '__c20: loop {
                if unsafe { (*p_constraint).i_column } as i32 == 0 &&
                        unsafe { (*p_constraint).op } as i32 == 64 {
                    seen_match = 1;
                }
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c20;
                }
                if i_plan & 1 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 0 &&
                        unsafe { (*p_constraint).op } as i32 == 64 {
                    i_plan |= 1;
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
                    r_cost /= 1000000.0;
                }
                if i_plan & 2 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 1 &&
                        (unsafe { (*p_constraint).op } as i32 == 16 ||
                            unsafe { (*p_constraint).op } as i32 == 8) {
                    i_plan |= 2;
                    i_dist_term = i;
                    r_cost /= 10.0;
                }
                if i_plan & 4 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 2 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 4;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                    i_ruleset_term = i;
                    r_cost /= 10.0;
                }
                break '__c20;
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
    if i_plan & 2 != 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_dist_term as
                                    isize)
                        }).argv_index = 1 + (i_plan & 1 != 0) as i32
        };
    }
    if i_plan & 4 != 0 {
        let mut idx: i32 = 1;
        if i_plan & 1 != 0 {
            { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        }
        if i_plan & 2 != 0 {
            { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        }
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_ruleset_term as
                                    isize)
                        }).argv_index = idx
        };
    }
    unsafe { (*p_idx_info_1).idx_num = i_plan };
    if unsafe { (*p_idx_info_1).n_order_by } == 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 1 &&
            unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } as i32 == 0 {
        unsafe { (*p_idx_info_1).order_by_consumed = 1 };
    }
    if seen_match != 0 && i_plan & 1 == 0 { r_cost = 1e99; }
    unsafe { (*p_idx_info_1).estimated_cost = r_cost };
    return 0;
}
static mut fuzzer_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: Some(fuzzer_connect),
        x_connect: Some(fuzzer_connect),
        x_best_index: Some(fuzzer_best_index),
        x_disconnect: Some(fuzzer_disconnect),
        x_destroy: Some(fuzzer_disconnect),
        x_open: Some(fuzzer_open),
        x_close: Some(fuzzer_close),
        x_filter: Some(fuzzer_filter),
        x_next: Some(fuzzer_next),
        x_eof: Some(fuzzer_eof),
        x_column: Some(fuzzer_column),
        x_rowid: Some(fuzzer_rowid),
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
pub extern "C" fn sqlite3_fuzzer_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"fuzzer".as_ptr() as *mut i8 as *const i8,
                    &raw mut fuzzer_module as *const sqlite3_module,
                    core::ptr::null_mut())
            };
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
    fn strlen(__s: *const i8)
    -> u64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
}