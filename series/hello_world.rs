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
struct series_cursor {
    base: sqlite3_vtab_cursor,
    i_o_base: sqlite3_int64,
    i_o_term: sqlite3_int64,
    i_o_step: sqlite3_int64,
    i_base: sqlite3_int64,
    i_term: sqlite3_int64,
    i_step: sqlite3_uint64,
    i_value: sqlite3_int64,
    b_desc: u8,
    b_done: u8,
}
extern "C" fn span64(mut a: sqlite3_int64, mut b: sqlite3_int64)
    -> sqlite3_uint64 {
    if !(a >= b) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"span64".as_ptr() as *const i8,
                c"series.c".as_ptr() as *mut i8 as *const i8, 186,
                c"a>=b".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return unsafe { *(&raw mut a as *mut sqlite3_uint64) } -
            unsafe { *(&raw mut b as *mut sqlite3_uint64) };
}
extern "C" fn add64(mut a: sqlite3_int64, b: sqlite3_uint64)
    -> sqlite3_int64 {
    let mut x: sqlite3_uint64 =
        unsafe { core::ptr::read(&raw mut a as *mut sqlite3_uint64) };
    x += b;
    return unsafe { *(&raw mut x as *mut sqlite3_int64) };
}
extern "C" fn sub64(mut a: sqlite3_int64, b: sqlite3_uint64)
    -> sqlite3_int64 {
    let mut x: sqlite3_uint64 =
        unsafe { core::ptr::read(&raw mut a as *mut sqlite3_uint64) };
    x -= b;
    return unsafe { *(&raw mut x as *mut sqlite3_int64) };
}
extern "C" fn series_connect(db: *mut sqlite3, p_unused_1: *mut (),
    argc_unused_1: i32, argv_unused_1: *const *const i8,
    pp_vtab_1: *mut *mut sqlite3_vtab, pz_err_unused_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut sqlite3_vtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = p_unused_1; };
    { let _ = argc_unused_1; };
    { let _ = argv_unused_1; };
    { let _ = pz_err_unused_1; };
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(value,start hidden,stop hidden,step hidden)".as_ptr()
                        as *mut i8 as *const i8)
        };
    if rc == 0 {
        p_new =
            {
                unsafe {
                    *pp_vtab_1 =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<sqlite3_vtab>() as
                                        sqlite3_uint64)
                            } as *mut sqlite3_vtab
                };
                unsafe { *pp_vtab_1 }
            };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<sqlite3_vtab>() as u64)
        };
        unsafe { sqlite3_vtab_config(db, 2) };
    }
    return rc;
}
extern "C" fn series_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}
extern "C" fn series_open(p_unused_1: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let mut p_cur: *mut series_cursor = core::ptr::null_mut();
    { let _ = p_unused_1; };
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<series_cursor>() as
                        sqlite3_uint64)
            } as *mut series_cursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<series_cursor>() as u64)
    };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}
extern "C" fn series_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0;
}
extern "C" fn series_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *mut series_cursor = cur as *mut series_cursor;
    if unsafe { (*p_cur).i_value } == unsafe { (*p_cur).i_term } {
        unsafe { (*p_cur).b_done = 1 as u8 };
    } else if unsafe { (*p_cur).b_desc } != 0 {
        unsafe {
            (*p_cur).i_value =
                sub64(unsafe { (*p_cur).i_value }, unsafe { (*p_cur).i_step })
        };
        if !(unsafe { (*p_cur).i_value } >= unsafe { (*p_cur).i_term }) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesNext".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 289,
                    c"pCur->iValue>=pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
    } else {
        unsafe {
            (*p_cur).i_value =
                add64(unsafe { (*p_cur).i_value }, unsafe { (*p_cur).i_step })
        };
        if !(unsafe { (*p_cur).i_value } <= unsafe { (*p_cur).i_term }) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesNext".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 292,
                    c"pCur->iValue<=pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
    }
    return 0;
}
extern "C" fn series_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_cur: *const series_cursor =
        cur as *mut series_cursor as *const series_cursor;
    let mut x: sqlite3_int64 = 0 as sqlite3_int64;
    '__s0:
        {
        match i {
            1 => { x = unsafe { (*p_cur).i_o_base }; }
            2 => { x = unsafe { (*p_cur).i_o_term }; }
            3 => { x = unsafe { (*p_cur).i_o_step }; }
            _ => { x = unsafe { (*p_cur).i_value }; }
        }
    }
    unsafe { sqlite3_result_int64(ctx, x) };
    return 0;
}
extern "C" fn series_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p_cur: *const series_cursor =
        cur as *mut series_cursor as *const series_cursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_value } };
    return 0;
}
extern "C" fn series_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const series_cursor =
        cur as *mut series_cursor as *const series_cursor;
    return unsafe { (*p_cur).b_done } as i32;
}
extern "C" fn series_steps(p_cur_1: &series_cursor) -> sqlite3_uint64 {
    if (*p_cur_1).b_desc != 0 {
        if !((*p_cur_1).i_base >= (*p_cur_1).i_term) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesSteps".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 356,
                    c"pCur->iBase >= pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        return span64((*p_cur_1).i_base, (*p_cur_1).i_term) /
                (*p_cur_1).i_step;
    } else {
        if !((*p_cur_1).i_base <= (*p_cur_1).i_term) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"seriesSteps".as_ptr() as *const i8,
                    c"series.c".as_ptr() as *mut i8 as *const i8, 359,
                    c"pCur->iBase <= pCur->iTerm".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        return span64((*p_cur_1).i_term, (*p_cur_1).i_base) /
                (*p_cur_1).i_step;
    }
}
extern "C" fn series_ceil(r: f64) -> f64 { return unsafe { ceil(r) }; }
extern "C" fn series_floor(r: f64) -> f64 { return unsafe { floor(r) }; }
extern "C" fn series_real_to_i64(r: f64) -> sqlite3_int64 {
    if r < -9.223372036854775e18 {
        return 9223372036854775808u64 as sqlite3_int64;
    }
    if r > 9.223372036854775e18 {
        return 9223372036854775807i64 as sqlite3_int64;
    }
    return r as sqlite3_int64;
}
extern "C" fn series_filter(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_unused_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let mut p_cur: *mut series_cursor = core::ptr::null_mut();
    let mut i_arg: i32 = 0;
    let mut i: i32 = 0;
    let mut i_min: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_max: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_limit: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_offset: sqlite3_int64 = 0 as sqlite3_int64;
    let mut r: f64 = 0.0;
    let mut r__1: f64 = 0.0;
    let mut r__2: f64 = 0.0;
    let mut span: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut span__1: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut tmp: sqlite3_int64 = 0 as sqlite3_int64;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s2:
            {
            match __state {
                0 => {
                    p_cur = p_vtab_cursor_1 as *mut series_cursor;
                    __state = 3;
                }
                2 => {
                    unsafe { (*p_cur).i_base = 0 as sqlite3_int64 };
                    __state = 136;
                }
                3 => { i_arg = 0; __state = 4; }
                4 => { __state = 5; }
                5 => {
                    i_min = 9223372036854775808u64 as sqlite3_int64;
                    __state = 6;
                }
                6 => {
                    i_max = 9223372036854775807i64 as sqlite3_int64;
                    __state = 7;
                }
                7 => { i_limit = 0 as sqlite3_int64; __state = 8; }
                8 => { i_offset = 0 as sqlite3_int64; __state = 9; }
                9 => { { let _ = idx_str_unused_1; }; __state = 10; }
                10 => { i = 0; __state = 12; }
                11 => {
                    if idx_num_1 & 1 != 0 {
                        __state = 17;
                    } else { __state = 18; }
                }
                12 => { if i < argc { __state = 13; } else { __state = 11; } }
                13 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                            } == 5 {
                        __state = 15;
                    } else { __state = 14; }
                }
                14 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 12;
                }
                15 => { __state = 2; }
                16 => {
                    if idx_num_1 & 2 != 0 {
                        __state = 20;
                    } else { __state = 21; }
                }
                17 => {
                    unsafe {
                        (*p_cur).i_o_base =
                            unsafe {
                                sqlite3_value_int64(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            }
                    };
                    __state = 16;
                }
                18 => {
                    unsafe { (*p_cur).i_o_base = 0 as sqlite3_int64 };
                    __state = 16;
                }
                19 => {
                    if idx_num_1 & 4 != 0 {
                        __state = 23;
                    } else { __state = 24; }
                }
                20 => {
                    unsafe {
                        (*p_cur).i_o_term =
                            unsafe {
                                sqlite3_value_int64(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            }
                    };
                    __state = 19;
                }
                21 => {
                    unsafe {
                        (*p_cur).i_o_term = 4294967295u32 as sqlite3_int64
                    };
                    __state = 19;
                }
                22 => {
                    if idx_num_1 & 5 == 0 && idx_num_1 & 896 != 0 {
                        __state = 28;
                    } else { __state = 27; }
                }
                23 => {
                    unsafe {
                        (*p_cur).i_o_step =
                            unsafe {
                                sqlite3_value_int64(unsafe {
                                        *argv.offset({
                                                        let __p = &mut i_arg;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                    })
                            }
                    };
                    __state = 25;
                }
                24 => {
                    unsafe { (*p_cur).i_o_step = 1 as sqlite3_int64 };
                    __state = 22;
                }
                25 => {
                    if unsafe { (*p_cur).i_o_step } == 0 as i64 {
                        __state = 26;
                    } else { __state = 22; }
                }
                26 => {
                    unsafe { (*p_cur).i_o_step = 1 as sqlite3_int64 };
                    __state = 22;
                }
                27 => {
                    if idx_num_1 & 6 == 0 && idx_num_1 & 12416 != 0 {
                        __state = 30;
                    } else { __state = 29; }
                }
                28 => {
                    unsafe {
                        (*p_cur).i_o_base = 9223372036854775808u64 as sqlite3_int64
                    };
                    __state = 27;
                }
                29 => {
                    unsafe { (*p_cur).i_base = unsafe { (*p_cur).i_o_base } };
                    __state = 31;
                }
                30 => {
                    unsafe {
                        (*p_cur).i_o_term = 9223372036854775807i64 as sqlite3_int64
                    };
                    __state = 29;
                }
                31 => {
                    unsafe { (*p_cur).i_term = unsafe { (*p_cur).i_o_term } };
                    __state = 32;
                }
                32 => {
                    if unsafe { (*p_cur).i_o_step } > 0 as i64 {
                        __state = 34;
                    } else { __state = 35; }
                }
                33 => {
                    unsafe {
                        (*p_cur).b_desc =
                            (unsafe { (*p_cur).i_o_step } < 0 as i64) as u8
                    };
                    __state = 39;
                }
                34 => {
                    unsafe {
                        (*p_cur).i_step =
                            unsafe { (*p_cur).i_o_step } as sqlite3_uint64
                    };
                    __state = 33;
                }
                35 => {
                    if unsafe { (*p_cur).i_o_step } >
                            9223372036854775808u64 as sqlite3_int64 {
                        __state = 36;
                    } else { __state = 37; }
                }
                36 => {
                    unsafe {
                        (*p_cur).i_step =
                            -unsafe { (*p_cur).i_o_step } as sqlite3_uint64
                    };
                    __state = 33;
                }
                37 => {
                    unsafe {
                        (*p_cur).i_step =
                            9223372036854775807i64 as sqlite3_int64 as sqlite3_uint64
                    };
                    __state = 38;
                }
                38 => {
                    {
                        let __p = unsafe { &mut (*p_cur).i_step };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 33;
                }
                39 => {
                    if unsafe { (*p_cur).b_desc } as i32 == 0 &&
                            unsafe { (*p_cur).i_base } > unsafe { (*p_cur).i_term } {
                        __state = 41;
                    } else { __state = 40; }
                }
                40 => {
                    if unsafe { (*p_cur).b_desc } as i32 != 0 &&
                            unsafe { (*p_cur).i_base } < unsafe { (*p_cur).i_term } {
                        __state = 43;
                    } else { __state = 42; }
                }
                41 => { __state = 2; }
                42 => {
                    if idx_num_1 & 32 != 0 {
                        __state = 45;
                    } else { __state = 44; }
                }
                43 => { __state = 2; }
                44 => {
                    if idx_num_1 & 13184 != 0 {
                        __state = 49;
                    } else { __state = 48; }
                }
                45 => {
                    i_limit =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 46;
                }
                46 => {
                    if idx_num_1 & 64 != 0 {
                        __state = 47;
                    } else { __state = 44; }
                }
                47 => {
                    i_offset =
                        unsafe {
                            sqlite3_value_int64(unsafe {
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
                48 => {
                    if unsafe { (*p_cur).b_desc } as i32 == 0 {
                        __state = 112;
                    } else { __state = 113; }
                }
                49 => {
                    if idx_num_1 & 128 != 0 {
                        __state = 51;
                    } else { __state = 52; }
                }
                50 => {
                    if unsafe { (*p_cur).b_desc } as i32 == 0 {
                        __state = 93;
                    } else { __state = 94; }
                }
                51 => {
                    if unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *argv.offset(i_arg as isize)
                                    })
                            } == 2 {
                        __state = 53;
                    } else { __state = 54; }
                }
                52 => {
                    if idx_num_1 & 768 != 0 {
                        __state = 59;
                    } else { __state = 58; }
                }
                53 => {
                    r =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 55;
                }
                54 => {
                    i_min =
                        {
                            i_max =
                                unsafe {
                                    sqlite3_value_int64(unsafe {
                                            *argv.offset({
                                                            let __p = &mut i_arg;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize)
                                        })
                                };
                            i_max
                        };
                    __state = 50;
                }
                55 => {
                    if r == series_ceil(r) &&
                                r >= 9223372036854775808u64 as sqlite3_int64 as f64 &&
                            r <= 9223372036854775807i64 as sqlite3_int64 as f64 {
                        __state = 56;
                    } else { __state = 57; }
                }
                56 => {
                    i_min = { i_max = series_real_to_i64(r); i_max };
                    __state = 50;
                }
                57 => { __state = 2; }
                58 => {
                    if idx_num_1 & 12288 != 0 {
                        __state = 76;
                    } else { __state = 75; }
                }
                59 => {
                    if unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *argv.offset(i_arg as isize)
                                    })
                            } == 2 {
                        __state = 60;
                    } else { __state = 61; }
                }
                60 => {
                    r__1 =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 62;
                }
                61 => {
                    i_min =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 71;
                }
                62 => {
                    if r__1 <= 9223372036854775808u64 as sqlite3_int64 as f64 {
                        __state = 63;
                    } else { __state = 64; }
                }
                63 => {
                    i_min = 9223372036854775808u64 as sqlite3_int64;
                    __state = 58;
                }
                64 => {
                    if r__1 > 9223372036854775807i64 as sqlite3_int64 as f64 {
                        __state = 65;
                    } else { __state = 66; }
                }
                65 => { __state = 2; }
                66 => {
                    i_min = series_real_to_i64(series_ceil(r__1));
                    __state = 67;
                }
                67 => {
                    if idx_num_1 & 512 != 0 && r__1 == series_ceil(r__1) {
                        __state = 68;
                    } else { __state = 58; }
                }
                68 => {
                    if i_min == 9223372036854775807i64 as sqlite3_int64 {
                        __state = 70;
                    } else { __state = 69; }
                }
                69 => {
                    { let __p = &mut i_min; let __t = *__p; *__p += 1; __t };
                    __state = 58;
                }
                70 => { __state = 2; }
                71 => {
                    if idx_num_1 & 512 != 0 {
                        __state = 72;
                    } else { __state = 58; }
                }
                72 => {
                    if i_min == 9223372036854775807i64 as sqlite3_int64 {
                        __state = 73;
                    } else { __state = 74; }
                }
                73 => { __state = 2; }
                74 => {
                    { let __p = &mut i_min; let __t = *__p; *__p += 1; __t };
                    __state = 58;
                }
                75 => {
                    if i_min > i_max { __state = 92; } else { __state = 50; }
                }
                76 => {
                    if unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *argv.offset(i_arg as isize)
                                    })
                            } == 2 {
                        __state = 77;
                    } else { __state = 78; }
                }
                77 => {
                    r__2 =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 79;
                }
                78 => {
                    i_max =
                        unsafe {
                            sqlite3_value_int64(unsafe {
                                    *argv.offset({
                                                    let __p = &mut i_arg;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize)
                                })
                        };
                    __state = 88;
                }
                79 => {
                    if r__2 >= 9223372036854775807i64 as sqlite3_int64 as f64 {
                        __state = 80;
                    } else { __state = 81; }
                }
                80 => {
                    i_max = 9223372036854775807i64 as sqlite3_int64;
                    __state = 75;
                }
                81 => {
                    if r__2 <= 9223372036854775808u64 as sqlite3_int64 as f64 {
                        __state = 82;
                    } else { __state = 83; }
                }
                82 => { __state = 2; }
                83 => {
                    i_max = series_real_to_i64(series_floor(r__2));
                    __state = 84;
                }
                84 => {
                    if idx_num_1 & 8192 != 0 && r__2 == series_floor(r__2) {
                        __state = 85;
                    } else { __state = 75; }
                }
                85 => {
                    if i_max == 9223372036854775808u64 as sqlite3_int64 {
                        __state = 87;
                    } else { __state = 86; }
                }
                86 => {
                    { let __p = &mut i_max; let __t = *__p; *__p -= 1; __t };
                    __state = 75;
                }
                87 => { __state = 2; }
                88 => {
                    if idx_num_1 & 8192 != 0 {
                        __state = 89;
                    } else { __state = 75; }
                }
                89 => {
                    if i_max == 9223372036854775808u64 as sqlite3_int64 {
                        __state = 90;
                    } else { __state = 91; }
                }
                90 => { __state = 2; }
                91 => {
                    { let __p = &mut i_max; let __t = *__p; *__p -= 1; __t };
                    __state = 75;
                }
                92 => { __state = 2; }
                93 => {
                    if unsafe { (*p_cur).i_base } < i_min {
                        __state = 96;
                    } else { __state = 95; }
                }
                94 => {
                    if unsafe { (*p_cur).i_base } > i_max {
                        __state = 104;
                    } else { __state = 103; }
                }
                95 => {
                    if unsafe { (*p_cur).i_term } > i_max {
                        __state = 102;
                    } else { __state = 48; }
                }
                96 => {
                    span = span64(i_min, unsafe { (*p_cur).i_base });
                    __state = 97;
                }
                97 => {
                    unsafe {
                        (*p_cur).i_base =
                            add64(unsafe { (*p_cur).i_base },
                                span / unsafe { (*p_cur).i_step } *
                                    unsafe { (*p_cur).i_step })
                    };
                    __state = 98;
                }
                98 => {
                    if unsafe { (*p_cur).i_base } < i_min {
                        __state = 99;
                    } else { __state = 95; }
                }
                99 => {
                    if unsafe { (*p_cur).i_base } >
                            sub64(9223372036854775807i64 as sqlite3_int64,
                                unsafe { (*p_cur).i_step }) {
                        __state = 101;
                    } else { __state = 100; }
                }
                100 => {
                    unsafe {
                        (*p_cur).i_base =
                            add64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step })
                    };
                    __state = 95;
                }
                101 => { __state = 2; }
                102 => { unsafe { (*p_cur).i_term = i_max }; __state = 48; }
                103 => {
                    if unsafe { (*p_cur).i_term } < i_min {
                        __state = 110;
                    } else { __state = 48; }
                }
                104 => {
                    span__1 = span64(unsafe { (*p_cur).i_base }, i_max);
                    __state = 105;
                }
                105 => {
                    unsafe {
                        (*p_cur).i_base =
                            sub64(unsafe { (*p_cur).i_base },
                                span__1 / unsafe { (*p_cur).i_step } *
                                    unsafe { (*p_cur).i_step })
                    };
                    __state = 106;
                }
                106 => {
                    if unsafe { (*p_cur).i_base } > i_max {
                        __state = 107;
                    } else { __state = 103; }
                }
                107 => {
                    if unsafe { (*p_cur).i_base } <
                            add64(9223372036854775808u64 as sqlite3_int64,
                                unsafe { (*p_cur).i_step }) {
                        __state = 109;
                    } else { __state = 108; }
                }
                108 => {
                    unsafe {
                        (*p_cur).i_base =
                            sub64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step })
                    };
                    __state = 103;
                }
                109 => { __state = 2; }
                110 => { unsafe { (*p_cur).i_term = i_min }; __state = 48; }
                111 => {
                    if idx_num_1 & 8 != 0 &&
                                unsafe { (*p_cur).b_desc } as i32 == 0 ||
                            idx_num_1 & 16 != 0 &&
                                unsafe { (*p_cur).b_desc } as i32 != 0 {
                        __state = 119;
                    } else { __state = 118; }
                }
                112 => {
                    if unsafe { (*p_cur).i_base } > unsafe { (*p_cur).i_term } {
                        __state = 115;
                    } else { __state = 114; }
                }
                113 => {
                    if unsafe { (*p_cur).i_base } < unsafe { (*p_cur).i_term } {
                        __state = 117;
                    } else { __state = 116; }
                }
                114 => {
                    unsafe {
                        (*p_cur).i_term =
                            sub64(unsafe { (*p_cur).i_term },
                                span64(unsafe { (*p_cur).i_term },
                                        unsafe { (*p_cur).i_base }) % unsafe { (*p_cur).i_step })
                    };
                    __state = 111;
                }
                115 => { __state = 2; }
                116 => {
                    unsafe {
                        (*p_cur).i_term =
                            add64(unsafe { (*p_cur).i_term },
                                span64(unsafe { (*p_cur).i_base },
                                        unsafe { (*p_cur).i_term }) % unsafe { (*p_cur).i_step })
                    };
                    __state = 111;
                }
                117 => { __state = 2; }
                118 => {
                    if !(unsafe { (*p_cur).i_step } != 0 as u64) as i32 as i64
                            != 0 {
                        unsafe {
                            __assert_rtn(c"seriesFilter".as_ptr() as *const i8,
                                c"series.c".as_ptr() as *mut i8 as *const i8, 661,
                                c"pCur->iStep!=0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 123;
                }
                119 => { tmp = unsafe { (*p_cur).i_base }; __state = 120; }
                120 => {
                    unsafe { (*p_cur).i_base = unsafe { (*p_cur).i_term } };
                    __state = 121;
                }
                121 => { unsafe { (*p_cur).i_term = tmp }; __state = 122; }
                122 => {
                    unsafe {
                        (*p_cur).b_desc =
                            (unsafe { (*p_cur).b_desc } == 0) as i32 as u8
                    };
                    __state = 118;
                }
                123 => {
                    if idx_num_1 & 32 != 0 {
                        __state = 125;
                    } else { __state = 124; }
                }
                124 => {
                    unsafe { (*p_cur).i_value = unsafe { (*p_cur).i_base } };
                    __state = 133;
                }
                125 => {
                    if i_offset > 0 as i64 {
                        __state = 127;
                    } else { __state = 126; }
                }
                126 => {
                    if i_limit >= 0 as i64 &&
                            series_steps(unsafe { &*p_cur }) > i_limit as sqlite3_uint64
                        {
                        __state = 132;
                    } else { __state = 124; }
                }
                127 => {
                    if series_steps(unsafe { &*p_cur }) <
                            i_offset as sqlite3_uint64 {
                        __state = 128;
                    } else { __state = 129; }
                }
                128 => { __state = 2; }
                129 => {
                    if unsafe { (*p_cur).b_desc } != 0 {
                        __state = 130;
                    } else { __state = 131; }
                }
                130 => {
                    unsafe {
                        (*p_cur).i_base =
                            sub64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step } * i_offset as sqlite3_uint64)
                    };
                    __state = 126;
                }
                131 => {
                    unsafe {
                        (*p_cur).i_base =
                            add64(unsafe { (*p_cur).i_base },
                                unsafe { (*p_cur).i_step } * i_offset as sqlite3_uint64)
                    };
                    __state = 126;
                }
                132 => {
                    unsafe {
                        (*p_cur).i_term =
                            add64(unsafe { (*p_cur).i_base },
                                (i_limit - 1 as sqlite3_int64) as sqlite3_uint64 *
                                    unsafe { (*p_cur).i_step })
                    };
                    __state = 124;
                }
                133 => {
                    unsafe { (*p_cur).b_done = 0 as u8 };
                    __state = 134;
                }
                134 => { return 0; }
                135 => { __state = 2; }
                136 => {
                    unsafe { (*p_cur).i_term = 0 as sqlite3_int64 };
                    __state = 137;
                }
                137 => {
                    unsafe { (*p_cur).i_step = 1 as sqlite3_uint64 };
                    __state = 138;
                }
                138 => {
                    unsafe { (*p_cur).b_desc = 0 as u8 };
                    __state = 139;
                }
                139 => {
                    unsafe { (*p_cur).b_done = 1 as u8 };
                    __state = 140;
                }
                140 => { return 0; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn series_best_index(p_v_tab_1: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut idx_num: i32 = 0;
    let mut b_start_seen: i32 = 0;
    let mut unusable_mask: i32 = 0;
    let mut n_arg: i32 = 0;
    let mut a_idx: [i32; 7] = [0; 7];
    let mut p_constraint: *const sqlite3_index_constraint = core::ptr::null();
    if !(2 == 1 + 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                c"series.c".as_ptr() as *mut i8 as *const i8, 748,
                c"SERIES_COLUMN_STOP == SERIES_COLUMN_START+1".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(3 == 1 + 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                c"series.c".as_ptr() as *mut i8 as *const i8, 749,
                c"SERIES_COLUMN_STEP == SERIES_COLUMN_START+2".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    a_idx[0 as usize] =
        {
            a_idx[1 as usize] =
                {
                    a_idx[2 as usize] =
                        {
                            a_idx[3 as usize] =
                                {
                                    a_idx[4 as usize] =
                                        {
                                            a_idx[5 as usize] =
                                                { a_idx[6 as usize] = -1; a_idx[6 as usize] };
                                            a_idx[5 as usize]
                                        };
                                    a_idx[4 as usize]
                                };
                            a_idx[3 as usize]
                        };
                    a_idx[2 as usize]
                };
            a_idx[1 as usize]
        };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const sqlite3_index_constraint;
    {
        i = 0;
        '__b3: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) { break '__b3; }
            '__c3: loop {
                let mut i_col: i32 = 0;
                let mut i_mask: i32 = 0;
                let op: i32 = unsafe { (*p_constraint).op } as i32;
                if op >= 73 && op <= 74 {
                    if unsafe { (*p_constraint).usable } as i32 == 0
                        {} else if op == 73 {
                        a_idx[3 as usize] = i;
                        idx_num |= 32;
                    } else {
                        if !(op == 74) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                                    c"series.c".as_ptr() as *mut i8 as *const i8, 766,
                                    c"op==SQLITE_INDEX_CONSTRAINT_OFFSET".as_ptr() as *mut i8 as
                                        *const i8)
                            }
                        } else { { let _ = 0; } };
                        a_idx[4 as usize] = i;
                        idx_num |= 64;
                    }
                    break '__c3;
                }
                if (unsafe { (*p_constraint).i_column } as i32) < 1 {
                    if (unsafe { (*p_constraint).i_column } as i32 == 0 ||
                                unsafe { (*p_constraint).i_column } as i32 == -1) &&
                            unsafe { (*p_constraint).usable } != 0 {
                        '__s4:
                            {
                            match op {
                                2 => {
                                    {
                                        idx_num |= 128;
                                        idx_num &= !13056;
                                        a_idx[5 as usize] = i;
                                        a_idx[6 as usize] = -1;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 256;
                                        idx_num &= !512;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                72 => {
                                    {
                                        idx_num |= 128;
                                        idx_num &= !13056;
                                        a_idx[5 as usize] = i;
                                        a_idx[6 as usize] = -1;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 256;
                                        idx_num &= !512;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                32 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 256;
                                        idx_num &= !512;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                4 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 512;
                                        idx_num &= !256;
                                        a_idx[5 as usize] = i;
                                        b_start_seen = 1;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                8 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 4096;
                                        idx_num &= !8192;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                16 => {
                                    {
                                        if idx_num & 128 != 0 { break '__s4; }
                                        idx_num |= 8192;
                                        idx_num &= !4096;
                                        a_idx[6 as usize] = i;
                                        break '__s4;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    break '__c3;
                }
                i_col = unsafe { (*p_constraint).i_column } - 1;
                if !(i_col >= 0 && i_col <= 2) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"seriesBestIndex".as_ptr() as *const i8,
                            c"series.c".as_ptr() as *mut i8 as *const i8, 828,
                            c"iCol>=0 && iCol<=2".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                i_mask = 1 << i_col;
                if i_col == 0 && op == 2 { b_start_seen = 1; }
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    unusable_mask |= i_mask;
                    break '__c3;
                } else if op == 2 {
                    idx_num |= i_mask;
                    a_idx[i_col as usize] = i;
                }
                break '__c3;
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
    if a_idx[3 as usize] == 0 { idx_num &= !96; a_idx[4 as usize] = 0; }
    {
        i = 0;
        '__b5: loop {
            if !(i < 7) { break '__b5; }
            '__c5: loop {
                if { j = a_idx[i as usize]; j } >= 0 {
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                                    }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                                    }).omit = ((0 == 0) as i32 != 0 || i >= 3) as u8
                    };
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (b_start_seen == 0) as i32 != 0 {
        unsafe { sqlite3_free(unsafe { (*p_v_tab_1).z_err_msg } as *mut ()) };
        unsafe {
            (*p_v_tab_1).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"first argument to \"generate_series()\" missing or unusable".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        return 1;
    }
    if unusable_mask & !idx_num != 0 { return 19; }
    if idx_num & 3 == 3 {
        unsafe {
            (*p_idx_info_1).estimated_cost =
                (2 - (idx_num & 4 != 0) as i32) as f64
        };
        unsafe { (*p_idx_info_1).estimated_rows = 1000 as sqlite3_int64 };
        if unsafe { (*p_idx_info_1).n_order_by } >= 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 0 {
            if unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } != 0 {
                idx_num |= 8;
            } else { idx_num |= 16; }
            unsafe { (*p_idx_info_1).order_by_consumed = 1 };
        }
    } else if idx_num & 33 == 33 {
        unsafe { (*p_idx_info_1).estimated_rows = 2500 as sqlite3_int64 };
    } else {
        unsafe {
            (*p_idx_info_1).estimated_rows = 2147483647 as sqlite3_int64
        };
    }
    unsafe { (*p_idx_info_1).idx_num = idx_num };
    unsafe { (*p_idx_info_1).idx_flags = 2 };
    return 0;
}
static mut series_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: None,
        x_connect: Some(series_connect),
        x_best_index: Some(series_best_index),
        x_disconnect: Some(series_disconnect),
        x_destroy: None,
        x_open: Some(series_open),
        x_close: Some(series_close),
        x_filter: Some(series_filter),
        x_next: Some(series_next),
        x_eof: Some(series_eof),
        x_column: Some(series_column),
        x_rowid: Some(series_rowid),
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
pub extern "C" fn sqlite3_series_init(db: *mut sqlite3,
    pz_err_msg_1: *mut *mut i8, p_api_1: *const sqlite3_api_routines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        if unsafe { sqlite3_libversion_number() } < 3008012 &&
                pz_err_msg_1 != core::ptr::null_mut() {
            unsafe {
                *pz_err_msg_1 =
                    unsafe {
                        sqlite3_mprintf(c"generate_series() requires SQLite 3.8.12 or later".as_ptr()
                                    as *mut i8 as *const i8)
                    }
            };
            return 1;
        }
        rc =
            unsafe {
                sqlite3_create_module(db,
                    c"generate_series".as_ptr() as *mut i8 as *const i8,
                    &raw mut series_module as *const sqlite3_module,
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn ceil(_: f64)
    -> f64;
    fn floor(_: f64)
    -> f64;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}