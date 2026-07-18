#![feature(c_variadic)]
type __darwin_size_t = u64;
type Byte = u8;
type uInt = u32;
#[repr(C)]
#[derive(Copy, Clone)]
struct z_stream_s {
    next_in: *mut u8,
    avail_in: u32,
    total_in: u64,
    next_out: *mut u8,
    avail_out: u32,
    total_out: u64,
    msg: *mut i8,
    state: *mut internal_state,
    zalloc: Option<unsafe extern "C" fn(*mut (), u32, u32) -> *mut ()>,
    zfree: Option<unsafe extern "C" fn(*mut (), *mut ()) -> ()>,
    opaque: *mut (),
    data_type: i32,
    adler: u64,
    reserved: u64,
}
type z_stream = z_stream_s;
type z_streamp = *mut z_stream;
type Bytef = Byte;
type uLong = u64;
type voidpf = *mut ();
type alloc_func = unsafe extern "C" fn(*mut (), u32, u32) -> *mut ();
type free_func = unsafe extern "C" fn(*mut (), *mut ()) -> ();
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
static zipfile_schema: [i8; 91] =
    [67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            121 as i8, 40 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8,
            32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8,
            82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8,
            44 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8, 44 as i8,
            109 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8, 44 as i8,
            115 as i8, 122 as i8, 44 as i8, 114 as i8, 97 as i8, 119 as i8,
            100 as i8, 97 as i8, 116 as i8, 97 as i8, 44 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 44 as i8, 109 as i8, 101 as i8,
            116 as i8, 104 as i8, 111 as i8, 100 as i8, 44 as i8, 122 as i8,
            32 as i8, 72 as i8, 73 as i8, 68 as i8, 68 as i8, 69 as i8,
            78 as i8, 41 as i8, 32 as i8, 87 as i8, 73 as i8, 84 as i8,
            72 as i8, 79 as i8, 85 as i8, 84 as i8, 32 as i8, 82 as i8,
            79 as i8, 87 as i8, 73 as i8, 68 as i8, 59 as i8, 0 as i8];
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileEOCD {
    i_disk: u16,
    i_first_disk: u16,
    n_entry: u16,
    n_entry_total: u16,
    n_size: u32,
    i_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileCDS {
    i_version_made_by: u16,
    i_version_extract: u16,
    flags: u16,
    i_compression: u16,
    m_time: u16,
    m_date: u16,
    crc32: u32,
    sz_compressed: u32,
    sz_uncompressed: u32,
    n_file: u16,
    n_extra: u16,
    n_comment: u16,
    i_disk_start: u16,
    i_internal_attr: u16,
    i_external_attr: u32,
    i_offset: u32,
    z_file: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileLFH {
    i_version_extract: u16,
    flags: u16,
    i_compression: u16,
    m_time: u16,
    m_date: u16,
    crc32: u32,
    sz_compressed: u32,
    sz_uncompressed: u32,
    n_file: u16,
    n_extra: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileEntry {
    cds: ZipfileCDS,
    m_unix_time: u32,
    a_extra: *mut u8,
    i_data_off: i64,
    a_data: *mut u8,
    p_next: *mut ZipfileEntry,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileCsr {
    base: sqlite3_vtab_cursor,
    i_id: i64,
    b_eof: u8,
    b_noop: u8,
    p_file: *mut FILE,
    i_next_off: i64,
    eocd: ZipfileEOCD,
    p_free_entry: *mut ZipfileEntry,
    p_current: *mut ZipfileEntry,
    p_csr_next: *mut ZipfileCsr,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileTab {
    base: sqlite3_vtab,
    z_file: *mut i8,
    db: *mut sqlite3,
    a_buffer: *mut u8,
    p_csr_list: *mut ZipfileCsr,
    i_next_csrid: i64,
    p_first_entry: *mut ZipfileEntry,
    p_last_entry: *mut ZipfileEntry,
    p_write_fd: *mut FILE,
    sz_current: i64,
    sz_orig: i64,
}
unsafe extern "C" fn zipfile_ctx_error_msg(ctx: *mut sqlite3_context,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    unsafe { sqlite3_result_error(ctx, z_msg as *const i8, -1) };
    unsafe { sqlite3_free(z_msg as *mut ()) };
    ();
}
extern "C" fn zipfile_dequote(z_in_1: *mut i8) -> () {
    let mut q: i8 = unsafe { *z_in_1.offset(0 as isize) };
    if q as i32 == '\"' as i32 || q as i32 == '\'' as i32 ||
                q as i32 == '`' as i32 || q as i32 == '[' as i32 {
        let mut i_in: i32 = 1;
        let mut i_out: i32 = 0;
        if q as i32 == '[' as i32 { q = ']' as i32 as i8; }
        while if unsafe { *z_in_1.offset(i_in as isize) } != 0 {
                    1
                } else {
                    {
                        if (0 == 0) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"zipfileDequote".as_ptr() as *const i8,
                                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 343,
                                    c"0".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        0
                    }
                } != 0 {
            let c: i8 =
                unsafe {
                    *z_in_1.offset({
                                    let __p = &mut i_in;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                };
            if c as i32 == q as i32 &&
                    unsafe {
                                *z_in_1.offset({
                                                let __p = &mut i_in;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize)
                            } as i32 != q as i32 {
                break;
            }
            unsafe {
                *z_in_1.offset({
                                    let __p = &mut i_out;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize) = c
            };
        }
        unsafe { *z_in_1.offset(i_out as isize) = '\u{0}' as i32 as i8 };
    }
}
extern "C" fn zipfile_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let n_byte: i32 =
        (core::mem::size_of::<ZipfileTab>() as u64 + (200 * 1024) as u64) as
            i32;
    let mut n_file: i32 = 0;
    let mut z_file: *const i8 = core::ptr::null();
    let mut p_new: *mut ZipfileTab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = p_aux_1; };
    if !(0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe { *argv.offset(0 as isize) },
                                    c"zipfile".as_ptr() as *mut i8 as *const i8)
                            }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileConnect".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 383,
                c"0==sqlite3_stricmp(argv[0], \"zipfile\")".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if 0 !=
                    unsafe {
                        sqlite3_stricmp(unsafe { *argv.offset(2 as isize) },
                            c"zipfile".as_ptr() as *mut i8 as *const i8)
                    } && argc < 4 || argc > 4 {
        unsafe {
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"zipfile constructor requires one argument".as_ptr()
                                as *mut i8 as *const i8)
                }
        };
        return 1;
    }
    if argc > 3 {
        z_file = unsafe { *argv.offset(3 as isize) };
        n_file = unsafe { strlen(z_file) } as i32 + 1;
    }
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                &raw const zipfile_schema[0 as usize] as *const i8)
        };
    if rc == 0 {
        p_new =
            unsafe {
                    sqlite3_malloc64((n_byte as i64 + n_file as i64) as
                            sqlite3_uint64)
                } as *mut ZipfileTab;
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe { memset(p_new as *mut (), 0, (n_byte + n_file) as u64) };
        unsafe { (*p_new).db = db };
        unsafe {
            (*p_new).a_buffer =
                unsafe { &raw mut *p_new.offset(1 as isize) } as *mut u8
        };
        if !(z_file).is_null() {
            unsafe {
                (*p_new).z_file =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_new).a_buffer.offset((200 * 1024) as isize)
                                    }
                        } as *mut i8
            };
            unsafe {
                memcpy(unsafe { (*p_new).z_file } as *mut (),
                    z_file as *const (), n_file as u64)
            };
            zipfile_dequote(unsafe { (*p_new).z_file });
        }
    }
    unsafe { sqlite3_vtab_config(db, 3) };
    unsafe { *pp_vtab_1 = p_new as *mut sqlite3_vtab };
    return rc;
}
extern "C" fn zipfile_entry_free(p: *mut ZipfileEntry) -> () {
    if !(p).is_null() {
        unsafe { sqlite3_free(unsafe { (*p).cds.z_file } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}
extern "C" fn zipfile_cleanup_transaction(p_tab_1: &mut ZipfileTab) -> () {
    let mut p_entry: *mut ZipfileEntry = core::ptr::null_mut();
    let mut p_next: *mut ZipfileEntry = core::ptr::null_mut();
    if !((*p_tab_1).p_write_fd).is_null() {
        unsafe { fclose((*p_tab_1).p_write_fd) };
        (*p_tab_1).p_write_fd = core::ptr::null_mut();
    }
    {
        p_entry = (*p_tab_1).p_first_entry;
        '__b1: loop {
            if !(!(p_entry).is_null()) { break '__b1; }
            '__c1: loop {
                p_next = unsafe { (*p_entry).p_next };
                zipfile_entry_free(p_entry);
                break '__c1;
            }
            p_entry = p_next;
        }
    }
    (*p_tab_1).p_first_entry = core::ptr::null_mut();
    (*p_tab_1).p_last_entry = core::ptr::null_mut();
    (*p_tab_1).sz_current = 0 as i64;
    (*p_tab_1).sz_orig = 0 as i64;
}
extern "C" fn zipfile_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    zipfile_cleanup_transaction(unsafe {
            &mut *(p_vtab_1 as *mut ZipfileTab)
        });
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}
extern "C" fn zipfile_open(p: *mut sqlite3_vtab,
    pp_csr_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let p_tab: *mut ZipfileTab = p as *mut ZipfileTab;
    let mut p_csr: *mut ZipfileCsr = core::ptr::null_mut();
    p_csr =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<ZipfileCsr>() as
                        sqlite3_uint64)
            } as *mut ZipfileCsr;
    unsafe { *pp_csr_1 = p_csr as *mut sqlite3_vtab_cursor };
    if p_csr == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_csr as *mut (), 0, core::mem::size_of::<ZipfileCsr>() as u64)
    };
    unsafe {
        (*p_csr).i_id =
            {
                let __p = unsafe { &mut (*p_tab).i_next_csrid };
                *__p += 1;
                *__p
            }
    };
    unsafe { (*p_csr).p_csr_next = unsafe { (*p_tab).p_csr_list } };
    unsafe { (*p_tab).p_csr_list = p_csr };
    return 0;
}
extern "C" fn zipfile_reset_cursor(p_csr_1: &mut ZipfileCsr) -> () {
    let mut p: *mut ZipfileEntry = core::ptr::null_mut();
    let mut p_next: *mut ZipfileEntry = core::ptr::null_mut();
    (*p_csr_1).b_eof = 0 as u8;
    if !((*p_csr_1).p_file).is_null() {
        unsafe { fclose((*p_csr_1).p_file) };
        (*p_csr_1).p_file = core::ptr::null_mut();
        zipfile_entry_free((*p_csr_1).p_current);
        (*p_csr_1).p_current = core::ptr::null_mut();
    }
    {
        p = (*p_csr_1).p_free_entry;
        '__b2: loop {
            if !(!(p).is_null()) { break '__b2; }
            '__c2: loop {
                p_next = unsafe { (*p).p_next };
                zipfile_entry_free(p);
                break '__c2;
            }
            p = p_next;
        }
    }
    (*p_csr_1).p_free_entry = core::ptr::null_mut();
}
extern "C" fn zipfile_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
    let p_tab: *mut ZipfileTab =
        unsafe { (*p_csr).base.p_vtab } as *mut ZipfileTab;
    let mut pp: *mut *mut ZipfileCsr = core::ptr::null_mut();
    zipfile_reset_cursor(unsafe { &mut *p_csr });
    {
        pp = unsafe { &mut (*p_tab).p_csr_list };
        '__b3: loop {
            if !(unsafe { *pp } != p_csr) { break '__b3; }
            '__c3: loop { break '__c3; }
            pp = unsafe { &mut (*unsafe { *pp }).p_csr_next };
        }
    }
    unsafe { *pp = unsafe { (*p_csr).p_csr_next } };
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}
unsafe extern "C" fn zipfile_table_err(p_tab_1: &mut ZipfileTab,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe { sqlite3_free((*p_tab_1).base.z_err_msg as *mut ()) };
    (*p_tab_1).base.z_err_msg = unsafe { sqlite3_vmprintf(z_fmt_1, ap) };
    ();
}
unsafe extern "C" fn zipfile_cursor_err(p_csr_1: &ZipfileCsr,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe {
        sqlite3_free(unsafe { (*(*p_csr_1).base.p_vtab).z_err_msg } as
                *mut ())
    };
    unsafe {
        (*(*p_csr_1).base.p_vtab).z_err_msg =
            unsafe { sqlite3_vmprintf(z_fmt_1, ap) }
    };
    ();
}
extern "C" fn zipfile_read_data(p_file_1: *mut FILE, a_read_1: *mut u8,
    n_read_1: i64, i_off_1: i64, pz_errmsg_1: &mut *mut i8) -> i32 {
    let mut n: u64 = 0 as u64;
    unsafe { fseek(p_file_1, i_off_1 as i64, 0) };
    n =
        unsafe {
            fread(a_read_1 as *mut (), 1 as u64, n_read_1 as i64 as u64,
                p_file_1)
        };
    if n != n_read_1 as u64 {
        unsafe { sqlite3_free(*pz_errmsg_1 as *mut ()) };
        *pz_errmsg_1 =
            unsafe {
                sqlite3_mprintf(c"error in fread()".as_ptr() as *mut i8 as
                        *const i8)
            };
        return 1;
    }
    return 0;
}
extern "C" fn zipfile_append_data(p_tab_1: *mut ZipfileTab,
    a_write_1: *const u8, n_write_1: i32) -> i32 {
    if n_write_1 > 0 {
        let mut n: u64 = n_write_1 as u64;
        unsafe {
            fseek(unsafe { (*p_tab_1).p_write_fd },
                unsafe { (*p_tab_1).sz_current } as i64, 0)
        };
        n =
            unsafe {
                fwrite(a_write_1 as *const (), 1 as u64, n_write_1 as u64,
                    unsafe { (*p_tab_1).p_write_fd })
            };
        if n as i32 != n_write_1 {
            unsafe {
                zipfile_table_err(unsafe { &mut *p_tab_1 },
                    c"error in fwrite()".as_ptr() as *mut i8 as *const i8)
            };
            return 1;
        }
        unsafe { (*p_tab_1).sz_current += n_write_1 as i64 };
    }
    return 0;
}
extern "C" fn zipfile_get_u16(a_buf_1: *const u8) -> u16 {
    return (((unsafe { *a_buf_1.offset(1 as isize) } as i32) << 8) +
                unsafe { *a_buf_1.offset(0 as isize) } as i32) as u16;
}
extern "C" fn zipfile_get_u32(a_buf_1: *const u8) -> u32 {
    if a_buf_1 == core::ptr::null() { return 0 as u32; }
    return ((unsafe { *a_buf_1.offset(3 as isize) } as u32) << 24) +
                    ((unsafe { *a_buf_1.offset(2 as isize) } as u32) << 16) +
                ((unsafe { *a_buf_1.offset(1 as isize) } as u32) << 8) +
            ((unsafe { *a_buf_1.offset(0 as isize) } as u32) << 0);
}
extern "C" fn zipfile_put_u16(a_buf_1: *mut u8, val: u16) -> () {
    unsafe { *a_buf_1.offset(0 as isize) = (val as i32 & 255) as u8 };
    unsafe { *a_buf_1.offset(1 as isize) = (val as i32 >> 8 & 255) as u8 };
}
extern "C" fn zipfile_put_u32(a_buf_1: *mut u8, val: u32) -> () {
    unsafe { *a_buf_1.offset(0 as isize) = (val & 255 as u32) as u8 };
    unsafe { *a_buf_1.offset(1 as isize) = (val >> 8 & 255 as u32) as u8 };
    unsafe { *a_buf_1.offset(2 as isize) = (val >> 16 & 255 as u32) as u8 };
    unsafe { *a_buf_1.offset(3 as isize) = (val >> 24 & 255 as u32) as u8 };
}
extern "C" fn zipfile_read_cds(a_buf_1: *mut u8, p_cds_1: &mut ZipfileCDS)
    -> i32 {
    let mut a_read: *mut u8 = a_buf_1;
    let sig: u32 =
        {
            {
                let __n = 4;
                let __p = &mut a_read;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                    *const u8)
        };
    let mut rc: i32 = 0;
    if sig != 33639248 as u32 {
        rc = 1;
    } else {
        (*p_cds_1).i_version_made_by =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).i_version_extract =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).flags =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).i_compression =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).m_time =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).m_date =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).crc32 =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).sz_compressed =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).sz_uncompressed =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        if !(a_read == unsafe { a_buf_1.offset(28 as isize) }) as i32 as i64
                != 0 {
            unsafe {
                __assert_rtn(c"zipfileReadCDS".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 644,
                    c"aRead==&aBuf[ZIPFILE_CDS_NFILE_OFF]".as_ptr() as *mut i8
                        as *const i8)
            }
        } else { { let _ = 0; } };
        (*p_cds_1).n_file =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).n_extra =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).n_comment =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).i_disk_start =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).i_internal_attr =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).i_external_attr =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        (*p_cds_1).i_offset =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        if !(a_read == unsafe { a_buf_1.offset(46 as isize) }) as i32 as i64
                != 0 {
            unsafe {
                __assert_rtn(c"zipfileReadCDS".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 652,
                    c"aRead==&aBuf[ZIPFILE_CDS_FIXED_SZ]".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
    }
    return rc;
}
extern "C" fn zipfile_read_lfh(a_buffer_1: *mut u8, p_lfh_1: &mut ZipfileLFH)
    -> i32 {
    let mut a_read: *const u8 = a_buffer_1 as *const u8;
    let mut rc: i32 = 0;
    let sig: u32 =
        {
            {
                let __n = 4;
                let __p = &mut a_read;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                    *const u8)
        };
    if sig != 67324752 as u32 {
        rc = 1;
    } else {
        (*p_lfh_1).i_version_extract =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).flags =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).i_compression =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).m_time =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).m_date =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).crc32 =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).sz_compressed =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).sz_uncompressed =
            {
                {
                    let __n = 4;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).n_file =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        (*p_lfh_1).n_extra =
            {
                {
                    let __n = 2;
                    let __p = &mut a_read;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                        *const u8)
            };
        if (*p_lfh_1).n_file as i32 > 250 { rc = 1; }
    }
    return rc;
}
extern "C" fn zipfile_scan_extra(a_extra_1: *mut u8, n_extra_1: i32,
    pm_time_1: &mut u32) -> i32 {
    let mut ret: i32 = 0;
    let mut p: *mut u8 = a_extra_1;
    let p_end: *mut u8 =
        unsafe { &mut *a_extra_1.offset(n_extra_1 as isize) };
    while unsafe {
                p.add((2 as u64 * core::mem::size_of::<u16>() as u64 +
                                1 as u64 + core::mem::size_of::<u32>() as u64) as usize)
            } <= p_end {
        let id: u16 =
            {
                {
                    let __n = 2;
                    let __p = &mut p;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { p.offset(-(2 as isize)) } as
                        *const u8)
            };
        let n_byte: u16 =
            {
                {
                    let __n = 2;
                    let __p = &mut p;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                zipfile_get_u16(unsafe { p.offset(-(2 as isize)) } as
                        *const u8)
            };
        '__s5:
            {
            match id {
                21589 => {
                    {
                        let b: u8 = unsafe { *p.offset(0 as isize) };
                        if b as i32 & 1 != 0 {
                            *pm_time_1 =
                                zipfile_get_u32(unsafe { &raw mut *p.offset(1 as isize) } as
                                        *const u8);
                            ret = 1;
                        }
                        break '__s5;
                    }
                }
                _ => {}
            }
        }
        {
            let __n = n_byte;
            let __p = &mut p;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
    }
    return ret;
}
extern "C" fn zipfile_mtime(p_cds_1: &ZipfileCDS) -> u32 {
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut sec: i32 = 0;
    let mut min: i32 = 0;
    let mut hr: i32 = 0;
    let mut j_dsec: i64 = 0 as i64;
    y = 1980 + ((*p_cds_1).m_date as i32 >> 9 & 127);
    m = (*p_cds_1).m_date as i32 >> 5 & 15;
    d = (*p_cds_1).m_date as i32 & 31;
    sec = ((*p_cds_1).m_time as i32 & 31) * 2;
    min = (*p_cds_1).m_time as i32 >> 5 & 63;
    hr = (*p_cds_1).m_time as i32 >> 11 & 31;
    if m <= 2 {
        { let __p = &mut y; let __t = *__p; *__p -= 1; __t };
        m += 12;
    }
    x1 = 36525 * (y + 4716) / 100;
    x2 = 306001 * (m + 1) / 10000;
    a = y / 100;
    b = 2 - a + a / 4;
    j_dsec =
        (((x1 + x2 + d + b) as f64 - 1524.5) * 86400 as f64) as i64 +
                    (hr * 3600) as i64 + (min * 60) as i64 + sec as i64;
    return (j_dsec - 24405875 as i64 * 8640 as i64) as u32;
}
extern "C" fn zipfile_mtime_to_dos(p_cds_1: *mut ZipfileCDS,
    m_unix_time_1: u32) -> () {
    let jd: i64 =
        2440588 as i64 + (m_unix_time_1 / (24 * 60 * 60) as u32) as i64;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    let mut e: i32 = 0;
    let mut yr: i32 = 0;
    let mut mon: i32 = 0;
    let mut day: i32 = 0;
    let mut hr: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0;
    a = ((jd as f64 - 1867216.25) / 36524.25) as i32;
    a = (jd + 1 as i64 + a as i64 - (a / 4) as i64) as i32;
    b = a + 1524;
    c = ((b as f64 - 122.1) / 365.25) as i32;
    d = 36525 * (c & 32767) / 100;
    e = ((b - d) as f64 / 30.6001) as i32;
    day = b - d - (30.6001 * e as f64) as i32;
    mon = if e < 14 { e - 1 } else { e - 13 };
    yr = if mon > 2 { c - 4716 } else { c - 4715 };
    hr = (m_unix_time_1 % (24 * 60 * 60) as u32 / (60 * 60) as u32) as i32;
    min = (m_unix_time_1 % (60 * 60) as u32 / 60 as u32) as i32;
    sec = (m_unix_time_1 % 60 as u32) as i32;
    if yr >= 1980 {
        unsafe {
            (*p_cds_1).m_date = (day + (mon << 5) + (yr - 1980 << 9)) as u16
        };
        unsafe {
            (*p_cds_1).m_time = (sec / 2 + (min << 5) + (hr << 11)) as u16
        };
    } else {
        unsafe {
            (*p_cds_1).m_date =
                {
                    unsafe { (*p_cds_1).m_time = 0 as u16 };
                    unsafe { (*p_cds_1).m_time }
                }
        };
    }
    if !(m_unix_time_1 < 315507600 as u32 ||
                                m_unix_time_1 == zipfile_mtime(unsafe { &*p_cds_1 }) ||
                            m_unix_time_1 % 2 as u32 != 0 &&
                                m_unix_time_1 - 1 as u32 ==
                                    zipfile_mtime(unsafe { &*p_cds_1 })) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileMtimeToDos".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 812,
                c"mUnixTime<315507600 || mUnixTime==zipfileMtime(pCds) || ((mUnixTime % 2) && mUnixTime-1==zipfileMtime(pCds))".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
}
extern "C" fn zipfile_corrupt(pz_err_1: &mut *mut i8) -> i32 {
    unsafe { sqlite3_free(*pz_err_1 as *mut ()) };
    *pz_err_1 =
        unsafe {
            sqlite3_mprintf(c"zip archive is corrupt".as_ptr() as *mut i8 as
                    *const i8)
        };
    return 11;
}
extern "C" fn zipfile_get_entry(p_tab_1: *mut ZipfileTab, a_blob_1: *const u8,
    n_blob_1: i64, p_file_1: *mut FILE, i_off_1: i64,
    pp_entry_1: &mut *mut ZipfileEntry) -> i32 {
    let mut a_read: *mut u8 = core::ptr::null_mut();
    let pz_err: *mut *mut i8 = unsafe { &mut (*p_tab_1).base.z_err_msg };
    let mut rc: i32 = 0;
    if a_blob_1 == core::ptr::null() {
        a_read = unsafe { (*p_tab_1).a_buffer };
        rc =
            zipfile_read_data(p_file_1, a_read, 46 as i64, i_off_1,
                unsafe { &mut *pz_err });
    } else {
        if i_off_1 + 46 as i64 > n_blob_1 {
            return zipfile_corrupt(unsafe { &mut *pz_err });
        }
        a_read =
            unsafe { &raw const *a_blob_1.offset(i_off_1 as isize) } as
                *mut u8;
    }
    if rc == 0 {
        let mut n_alloc: sqlite3_int64 = 0 as sqlite3_int64;
        let mut p_new: *mut ZipfileEntry = core::ptr::null_mut();
        let n_file: i32 =
            zipfile_get_u16(unsafe { &raw mut *a_read.offset(28 as isize) } as
                        *const u8) as i32;
        let mut n_extra: i32 =
            zipfile_get_u16(unsafe {
                            &raw mut *a_read.offset((28 + 2) as isize)
                        } as *const u8) as i32;
        n_extra +=
            zipfile_get_u16(unsafe {
                            &raw mut *a_read.offset((28 + 4) as isize)
                        } as *const u8) as i32;
        n_alloc =
            (core::mem::size_of::<ZipfileEntry>() as u64 + n_extra as u64) as
                sqlite3_int64;
        if !(a_blob_1).is_null() {
            n_alloc +=
                zipfile_get_u32(unsafe {
                                &raw mut *a_read.offset(20 as isize)
                            } as *const u8) as sqlite3_int64;
        }
        p_new =
            unsafe { sqlite3_malloc64(n_alloc as sqlite3_uint64) } as
                *mut ZipfileEntry;
        if p_new == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe {
                memset(p_new as *mut (), 0,
                    core::mem::size_of::<ZipfileEntry>() as u64)
            };
            rc = zipfile_read_cds(a_read, unsafe { &mut (*p_new).cds });
            if rc != 0 {
                unsafe {
                    zipfile_table_err(unsafe { &mut *p_tab_1 },
                        c"failed to read CDS at offset %lld".as_ptr() as *mut i8 as
                            *const i8, i_off_1)
                };
            } else if a_blob_1 == core::ptr::null() {
                rc =
                    zipfile_read_data(p_file_1, a_read,
                        (n_extra + n_file) as i64, i_off_1 + 46 as i64,
                        unsafe { &mut *pz_err });
            } else {
                a_read =
                    unsafe {
                            &raw const *a_blob_1.offset((i_off_1 + 46 as i64) as isize)
                        } as *mut u8;
                if i_off_1 + 46 as i64 + n_file as i64 + n_extra as i64 >
                        n_blob_1 {
                    rc = zipfile_corrupt(unsafe { &mut *pz_err });
                }
            }
        }
        if rc == 0 {
            let pt: *mut u32 = unsafe { &mut (*p_new).m_unix_time };
            unsafe {
                (*p_new).cds.z_file =
                    unsafe { sqlite3_malloc64((n_file + 1) as sqlite3_uint64) }
                        as *mut i8
            };
            if unsafe { (*p_new).cds.z_file } != core::ptr::null_mut() {
                unsafe {
                    memcpy(unsafe { (*p_new).cds.z_file } as *mut (),
                        a_read as *const (), n_file as u64)
                };
                unsafe {
                    *unsafe { (*p_new).cds.z_file.offset(n_file as isize) } =
                        0 as i8
                };
            }
            unsafe {
                (*p_new).a_extra =
                    unsafe { &raw mut *p_new.offset(1 as isize) } as *mut u8
            };
            unsafe {
                memcpy(unsafe { (*p_new).a_extra } as *mut (),
                    unsafe { &raw mut *a_read.offset(n_file as isize) } as
                        *const (), n_extra as u64)
            };
            if unsafe { (*p_new).cds.z_file } == core::ptr::null_mut() {
                rc = 7;
            } else if 0 ==
                    zipfile_scan_extra(unsafe {
                            &mut *a_read.offset(n_file as isize)
                        }, unsafe { (*p_new).cds.n_extra } as i32,
                        unsafe { &mut *pt }) {
                unsafe {
                    (*p_new).m_unix_time =
                        zipfile_mtime(unsafe { &(*p_new).cds })
                };
            }
        }
        if rc == 0 {
            let mut lfh: ZipfileLFH = unsafe { core::mem::zeroed() };
            if !(p_file_1).is_null() {
                rc =
                    zipfile_read_data(p_file_1, a_read, sz_fix as i64,
                        unsafe { (*p_new).cds.i_offset } as i64,
                        unsafe { &mut *pz_err });
            } else {
                a_read =
                    unsafe {
                            &raw const *a_blob_1.add(unsafe { (*p_new).cds.i_offset } as
                                            usize)
                        } as *mut u8;
                if unsafe { (*p_new).cds.i_offset } as i64 + 30 as i64 >
                        n_blob_1 {
                    rc = zipfile_corrupt(unsafe { &mut *pz_err });
                }
            }
            unsafe {
                memset(&raw mut lfh as *mut (), 0,
                    core::mem::size_of::<ZipfileLFH>() as u64)
            };
            if rc == 0 { rc = zipfile_read_lfh(a_read, &mut lfh); }
            if rc == 0 {
                unsafe {
                    (*p_new).i_data_off =
                        unsafe { (*p_new).cds.i_offset } as i64 + 30 as i64
                };
                unsafe {
                    (*p_new).i_data_off +=
                        (lfh.n_file as i32 + lfh.n_extra as i32) as i64
                };
                if !(a_blob_1).is_null() &&
                        unsafe { (*p_new).cds.sz_compressed } != 0 {
                    if unsafe { (*p_new).i_data_off } +
                                unsafe { (*p_new).cds.sz_compressed } as i64 > n_blob_1 {
                        rc = zipfile_corrupt(unsafe { &mut *pz_err });
                    } else {
                        unsafe {
                            (*p_new).a_data =
                                unsafe {
                                    unsafe { (*p_new).a_extra.offset(n_extra as isize) }
                                }
                        };
                        unsafe {
                            memcpy(unsafe { (*p_new).a_data } as *mut (),
                                unsafe {
                                        &raw const *a_blob_1.offset(unsafe { (*p_new).i_data_off }
                                                        as isize)
                                    } as *const (),
                                unsafe { (*p_new).cds.sz_compressed } as u64)
                        };
                    }
                }
            } else {
                unsafe {
                    zipfile_table_err(unsafe { &mut *p_tab_1 },
                        c"failed to read LFH at offset %d".as_ptr() as *mut i8 as
                            *const i8, unsafe { (*p_new).cds.i_offset } as i32)
                };
            }
        }
        if rc != 0 {
            zipfile_entry_free(p_new);
        } else { *pp_entry_1 = p_new; }
    }
    return rc;
}
extern "C" fn zipfile_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
    let mut rc: i32 = 0;
    if !(unsafe { (*p_csr).p_file }).is_null() {
        let i_eof: i64 =
            unsafe { (*p_csr).eocd.i_offset } as i64 +
                unsafe { (*p_csr).eocd.n_size } as i64;
        zipfile_entry_free(unsafe { (*p_csr).p_current });
        unsafe { (*p_csr).p_current = core::ptr::null_mut() };
        if unsafe { (*p_csr).i_next_off } >= i_eof {
            unsafe { (*p_csr).b_eof = 1 as u8 };
        } else {
            let mut p: *mut ZipfileEntry = core::ptr::null_mut();
            let p_tab: *mut ZipfileTab =
                unsafe { (*cur).p_vtab } as *mut ZipfileTab;
            rc =
                zipfile_get_entry(p_tab, core::ptr::null(), 0 as i64,
                    unsafe { (*p_csr).p_file }, unsafe { (*p_csr).i_next_off },
                    &mut p);
            if rc == 0 {
                unsafe { (*p_csr).i_next_off += 46 as i64 };
                unsafe {
                    (*p_csr).i_next_off +=
                        (unsafe { (*p).cds.n_extra } as i32 +
                                    unsafe { (*p).cds.n_file } as i32 +
                                unsafe { (*p).cds.n_comment } as i32) as i64
                };
            }
            unsafe { (*p_csr).p_current = p };
        }
    } else {
        if (unsafe { (*p_csr).b_noop } == 0) as i32 != 0 {
            unsafe {
                (*p_csr).p_current =
                    unsafe { (*unsafe { (*p_csr).p_current }).p_next }
            };
        }
        if unsafe { (*p_csr).p_current } == core::ptr::null_mut() {
            unsafe { (*p_csr).b_eof = 1 as u8 };
        }
    }
    unsafe { (*p_csr).b_noop = 0 as u8 };
    return rc;
}
extern "C" fn zipfile_free(p: *mut ()) -> () { unsafe { sqlite3_free(p) }; }
extern "C" fn zipfile_inflate(p_ctx_1: *mut sqlite3_context,
    a_in_1: *const u8, n_in_1: i32, n_out_1: i32) -> () {
    let mut a_res: *mut u8 =
        unsafe { sqlite3_malloc64(n_out_1 as sqlite3_uint64) } as *mut u8;
    if a_res == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(p_ctx_1) };
    } else {
        let mut err: i32 = 0;
        let mut str: z_stream = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut str as *mut (), 0,
                core::mem::size_of::<z_stream>() as u64)
        };
        str.next_in = a_in_1 as *mut Byte;
        str.avail_in = n_in_1 as uInt;
        str.next_out = a_res as *mut Byte;
        str.avail_out = n_out_1 as uInt;
        err =
            unsafe {
                inflateInit2_(&mut str, -15,
                    c"1.2.12".as_ptr() as *mut i8 as *const i8,
                    core::mem::size_of::<z_stream>() as i32)
            };
        if err != 0 {
            unsafe {
                zipfile_ctx_error_msg(p_ctx_1,
                    c"inflateInit2() failed (%d)".as_ptr() as *mut i8 as
                        *const i8, err)
            };
        } else {
            err = unsafe { inflate(&mut str, 0) };
            if err != 1 {
                unsafe {
                    zipfile_ctx_error_msg(p_ctx_1,
                        c"inflate() failed (%d)".as_ptr() as *mut i8 as *const i8,
                        err)
                };
            } else {
                unsafe {
                    sqlite3_result_blob(p_ctx_1, a_res as *const (),
                        str.total_out as i32, Some(zipfile_free))
                };
                a_res = core::ptr::null_mut();
            }
        }
        unsafe { sqlite3_free(a_res as *mut ()) };
        unsafe { inflateEnd(&mut str) };
    }
}
extern "C" fn zipfile_deflate(a_in_1: *const u8, n_in_1: i32,
    pp_out_1: &mut *mut u8, pn_out_1: &mut i32, pz_err_1: &mut *mut i8)
    -> i32 {
    let mut rc: i32 = 0;
    let mut n_alloc: sqlite3_int64 = 0 as sqlite3_int64;
    let mut str: z_stream = unsafe { core::mem::zeroed() };
    let mut a_out: *mut u8 = core::ptr::null_mut();
    unsafe {
        memset(&raw mut str as *mut (), 0,
            core::mem::size_of::<z_stream>() as u64)
    };
    str.next_in = a_in_1 as *mut Bytef;
    str.avail_in = n_in_1 as uInt;
    unsafe {
        deflateInit2_(&mut str, 9, 8, -15, 8, 0,
            c"1.2.12".as_ptr() as *mut i8 as *const i8,
            core::mem::size_of::<z_stream>() as i32)
    };
    n_alloc =
        unsafe { deflateBound(&mut str, n_in_1 as uLong) } as sqlite3_int64;
    a_out = unsafe { sqlite3_malloc64(n_alloc as sqlite3_uint64) } as *mut u8;
    if a_out == core::ptr::null_mut() {
        rc = 7;
    } else {
        let mut res: i32 = 0;
        str.next_out = a_out;
        str.avail_out = n_alloc as uInt;
        res = unsafe { deflate(&mut str, 4) };
        if res == 1 {
            *pp_out_1 = a_out;
            *pn_out_1 = str.total_out as i32;
        } else {
            unsafe { sqlite3_free(a_out as *mut ()) };
            *pz_err_1 =
                unsafe {
                    sqlite3_mprintf(c"zipfile: deflate() error".as_ptr() as
                                *mut i8 as *const i8)
                };
            rc = 1;
        }
        unsafe { deflateEnd(&mut str) };
    }
    return rc;
}
extern "C" fn zipfile_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_csr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
    let p_cds: *const ZipfileCDS =
        unsafe { &raw mut (*unsafe { (*p_csr).p_current }).cds } as
            *const ZipfileCDS;
    let mut rc: i32 = 0;
    '__s6:
        {
        match i {
            0 => {
                unsafe {
                    sqlite3_result_text(ctx,
                        unsafe { (*p_cds).z_file } as *const i8, -1,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
            }
            1 => {
                unsafe {
                    sqlite3_result_int(ctx,
                        (unsafe { (*p_cds).i_external_attr } >> 16) as i32)
                };
            }
            2 => {
                {
                    unsafe {
                        sqlite3_result_int64(ctx,
                            unsafe { (*unsafe { (*p_csr).p_current }).m_unix_time } as
                                sqlite3_int64)
                    };
                    break '__s6;
                }
                {
                    if unsafe { sqlite3_vtab_nochange(ctx) } == 0 {
                        unsafe {
                            sqlite3_result_int64(ctx,
                                unsafe { (*p_cds).sz_uncompressed } as sqlite3_int64)
                        };
                    }
                    break '__s6;
                }
                if unsafe { sqlite3_vtab_nochange(ctx) } != 0 { break '__s6; }
                {
                    if i == 4 || unsafe { (*p_cds).i_compression } as i32 == 0
                            || unsafe { (*p_cds).i_compression } as i32 == 8 {
                        let sz: i32 = unsafe { (*p_cds).sz_compressed } as i32;
                        let sz_final: i32 =
                            unsafe { (*p_cds).sz_uncompressed } as i32;
                        if sz_final > 0 {
                            let mut a_buf: *mut u8 = core::ptr::null_mut();
                            let mut a_free: *mut u8 = core::ptr::null_mut();
                            if !(unsafe {
                                                (*unsafe { (*p_csr).p_current }).a_data
                                            }).is_null() {
                                a_buf = unsafe { (*unsafe { (*p_csr).p_current }).a_data };
                            } else {
                                a_buf =
                                    {
                                        a_free =
                                            unsafe { sqlite3_malloc64(sz as sqlite3_uint64) } as
                                                *mut u8;
                                        a_free
                                    };
                                if a_buf == core::ptr::null_mut() {
                                    rc = 7;
                                } else {
                                    let mut p_file: *mut FILE = unsafe { (*p_csr).p_file };
                                    if p_file == core::ptr::null_mut() {
                                        p_file =
                                            unsafe {
                                                (*(unsafe { (*p_csr).base.p_vtab } as
                                                                *mut ZipfileTab)).p_write_fd
                                            };
                                    }
                                    rc =
                                        zipfile_read_data(p_file, a_buf, sz as i64,
                                            unsafe { (*unsafe { (*p_csr).p_current }).i_data_off },
                                            unsafe {
                                                &mut (*unsafe { (*p_csr).base.p_vtab }).z_err_msg
                                            });
                                }
                            }
                            if rc == 0 {
                                if i == 5 && unsafe { (*p_cds).i_compression } != 0 {
                                    zipfile_inflate(ctx, a_buf as *const u8, sz, sz_final);
                                } else {
                                    unsafe {
                                        sqlite3_result_blob(ctx, a_buf as *const (), sz,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                }
                            }
                            unsafe { sqlite3_free(a_free as *mut ()) };
                        } else {
                            let mode: u32 = unsafe { (*p_cds).i_external_attr } >> 16;
                            if (mode & 16384 as u32 == 0) as i32 != 0 &&
                                        unsafe { (*p_cds).n_file } as i32 >= 1 &&
                                    unsafe {
                                                *unsafe {
                                                        (*p_cds).z_file.offset((unsafe { (*p_cds).n_file } as i32 -
                                                                    1) as isize)
                                                    }
                                            } as i32 != '/' as i32 {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        c"".as_ptr() as *mut i8 as *const (), 0, None)
                                };
                            }
                        }
                    }
                    break '__s6;
                }
                unsafe {
                    sqlite3_result_int(ctx,
                        unsafe { (*p_cds).i_compression } as i32)
                };
            }
            3 => {
                {
                    if unsafe { sqlite3_vtab_nochange(ctx) } == 0 {
                        unsafe {
                            sqlite3_result_int64(ctx,
                                unsafe { (*p_cds).sz_uncompressed } as sqlite3_int64)
                        };
                    }
                    break '__s6;
                }
                if unsafe { sqlite3_vtab_nochange(ctx) } != 0 { break '__s6; }
                {
                    if i == 4 || unsafe { (*p_cds).i_compression } as i32 == 0
                            || unsafe { (*p_cds).i_compression } as i32 == 8 {
                        let sz: i32 = unsafe { (*p_cds).sz_compressed } as i32;
                        let sz_final: i32 =
                            unsafe { (*p_cds).sz_uncompressed } as i32;
                        if sz_final > 0 {
                            let mut a_buf: *mut u8 = core::ptr::null_mut();
                            let mut a_free: *mut u8 = core::ptr::null_mut();
                            if !(unsafe {
                                                (*unsafe { (*p_csr).p_current }).a_data
                                            }).is_null() {
                                a_buf = unsafe { (*unsafe { (*p_csr).p_current }).a_data };
                            } else {
                                a_buf =
                                    {
                                        a_free =
                                            unsafe { sqlite3_malloc64(sz as sqlite3_uint64) } as
                                                *mut u8;
                                        a_free
                                    };
                                if a_buf == core::ptr::null_mut() {
                                    rc = 7;
                                } else {
                                    let mut p_file: *mut FILE = unsafe { (*p_csr).p_file };
                                    if p_file == core::ptr::null_mut() {
                                        p_file =
                                            unsafe {
                                                (*(unsafe { (*p_csr).base.p_vtab } as
                                                                *mut ZipfileTab)).p_write_fd
                                            };
                                    }
                                    rc =
                                        zipfile_read_data(p_file, a_buf, sz as i64,
                                            unsafe { (*unsafe { (*p_csr).p_current }).i_data_off },
                                            unsafe {
                                                &mut (*unsafe { (*p_csr).base.p_vtab }).z_err_msg
                                            });
                                }
                            }
                            if rc == 0 {
                                if i == 5 && unsafe { (*p_cds).i_compression } != 0 {
                                    zipfile_inflate(ctx, a_buf as *const u8, sz, sz_final);
                                } else {
                                    unsafe {
                                        sqlite3_result_blob(ctx, a_buf as *const (), sz,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                }
                            }
                            unsafe { sqlite3_free(a_free as *mut ()) };
                        } else {
                            let mode: u32 = unsafe { (*p_cds).i_external_attr } >> 16;
                            if (mode & 16384 as u32 == 0) as i32 != 0 &&
                                        unsafe { (*p_cds).n_file } as i32 >= 1 &&
                                    unsafe {
                                                *unsafe {
                                                        (*p_cds).z_file.offset((unsafe { (*p_cds).n_file } as i32 -
                                                                    1) as isize)
                                                    }
                                            } as i32 != '/' as i32 {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        c"".as_ptr() as *mut i8 as *const (), 0, None)
                                };
                            }
                        }
                    }
                    break '__s6;
                }
                unsafe {
                    sqlite3_result_int(ctx,
                        unsafe { (*p_cds).i_compression } as i32)
                };
            }
            4 => {
                if unsafe { sqlite3_vtab_nochange(ctx) } != 0 { break '__s6; }
                {
                    if i == 4 || unsafe { (*p_cds).i_compression } as i32 == 0
                            || unsafe { (*p_cds).i_compression } as i32 == 8 {
                        let sz: i32 = unsafe { (*p_cds).sz_compressed } as i32;
                        let sz_final: i32 =
                            unsafe { (*p_cds).sz_uncompressed } as i32;
                        if sz_final > 0 {
                            let mut a_buf: *mut u8 = core::ptr::null_mut();
                            let mut a_free: *mut u8 = core::ptr::null_mut();
                            if !(unsafe {
                                                (*unsafe { (*p_csr).p_current }).a_data
                                            }).is_null() {
                                a_buf = unsafe { (*unsafe { (*p_csr).p_current }).a_data };
                            } else {
                                a_buf =
                                    {
                                        a_free =
                                            unsafe { sqlite3_malloc64(sz as sqlite3_uint64) } as
                                                *mut u8;
                                        a_free
                                    };
                                if a_buf == core::ptr::null_mut() {
                                    rc = 7;
                                } else {
                                    let mut p_file: *mut FILE = unsafe { (*p_csr).p_file };
                                    if p_file == core::ptr::null_mut() {
                                        p_file =
                                            unsafe {
                                                (*(unsafe { (*p_csr).base.p_vtab } as
                                                                *mut ZipfileTab)).p_write_fd
                                            };
                                    }
                                    rc =
                                        zipfile_read_data(p_file, a_buf, sz as i64,
                                            unsafe { (*unsafe { (*p_csr).p_current }).i_data_off },
                                            unsafe {
                                                &mut (*unsafe { (*p_csr).base.p_vtab }).z_err_msg
                                            });
                                }
                            }
                            if rc == 0 {
                                if i == 5 && unsafe { (*p_cds).i_compression } != 0 {
                                    zipfile_inflate(ctx, a_buf as *const u8, sz, sz_final);
                                } else {
                                    unsafe {
                                        sqlite3_result_blob(ctx, a_buf as *const (), sz,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                }
                            }
                            unsafe { sqlite3_free(a_free as *mut ()) };
                        } else {
                            let mode: u32 = unsafe { (*p_cds).i_external_attr } >> 16;
                            if (mode & 16384 as u32 == 0) as i32 != 0 &&
                                        unsafe { (*p_cds).n_file } as i32 >= 1 &&
                                    unsafe {
                                                *unsafe {
                                                        (*p_cds).z_file.offset((unsafe { (*p_cds).n_file } as i32 -
                                                                    1) as isize)
                                                    }
                                            } as i32 != '/' as i32 {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        c"".as_ptr() as *mut i8 as *const (), 0, None)
                                };
                            }
                        }
                    }
                    break '__s6;
                }
                unsafe {
                    sqlite3_result_int(ctx,
                        unsafe { (*p_cds).i_compression } as i32)
                };
            }
            5 => {
                {
                    if i == 4 || unsafe { (*p_cds).i_compression } as i32 == 0
                            || unsafe { (*p_cds).i_compression } as i32 == 8 {
                        let sz: i32 = unsafe { (*p_cds).sz_compressed } as i32;
                        let sz_final: i32 =
                            unsafe { (*p_cds).sz_uncompressed } as i32;
                        if sz_final > 0 {
                            let mut a_buf: *mut u8 = core::ptr::null_mut();
                            let mut a_free: *mut u8 = core::ptr::null_mut();
                            if !(unsafe {
                                                (*unsafe { (*p_csr).p_current }).a_data
                                            }).is_null() {
                                a_buf = unsafe { (*unsafe { (*p_csr).p_current }).a_data };
                            } else {
                                a_buf =
                                    {
                                        a_free =
                                            unsafe { sqlite3_malloc64(sz as sqlite3_uint64) } as
                                                *mut u8;
                                        a_free
                                    };
                                if a_buf == core::ptr::null_mut() {
                                    rc = 7;
                                } else {
                                    let mut p_file: *mut FILE = unsafe { (*p_csr).p_file };
                                    if p_file == core::ptr::null_mut() {
                                        p_file =
                                            unsafe {
                                                (*(unsafe { (*p_csr).base.p_vtab } as
                                                                *mut ZipfileTab)).p_write_fd
                                            };
                                    }
                                    rc =
                                        zipfile_read_data(p_file, a_buf, sz as i64,
                                            unsafe { (*unsafe { (*p_csr).p_current }).i_data_off },
                                            unsafe {
                                                &mut (*unsafe { (*p_csr).base.p_vtab }).z_err_msg
                                            });
                                }
                            }
                            if rc == 0 {
                                if i == 5 && unsafe { (*p_cds).i_compression } != 0 {
                                    zipfile_inflate(ctx, a_buf as *const u8, sz, sz_final);
                                } else {
                                    unsafe {
                                        sqlite3_result_blob(ctx, a_buf as *const (), sz,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                }
                            }
                            unsafe { sqlite3_free(a_free as *mut ()) };
                        } else {
                            let mode: u32 = unsafe { (*p_cds).i_external_attr } >> 16;
                            if (mode & 16384 as u32 == 0) as i32 != 0 &&
                                        unsafe { (*p_cds).n_file } as i32 >= 1 &&
                                    unsafe {
                                                *unsafe {
                                                        (*p_cds).z_file.offset((unsafe { (*p_cds).n_file } as i32 -
                                                                    1) as isize)
                                                    }
                                            } as i32 != '/' as i32 {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        c"".as_ptr() as *mut i8 as *const (), 0, None)
                                };
                            }
                        }
                    }
                    break '__s6;
                }
                unsafe {
                    sqlite3_result_int(ctx,
                        unsafe { (*p_cds).i_compression } as i32)
                };
            }
            6 => {
                unsafe {
                    sqlite3_result_int(ctx,
                        unsafe { (*p_cds).i_compression } as i32)
                };
            }
            _ => {
                if !(i == 7) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"zipfileColumn".as_ptr() as *const i8,
                            c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1169,
                            c"i==7".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                unsafe {
                    sqlite3_result_int64(ctx, unsafe { (*p_csr).i_id })
                };
            }
        }
    }
    return rc;
}
extern "C" fn zipfile_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *const ZipfileCsr =
        cur as *mut ZipfileCsr as *const ZipfileCsr;
    return unsafe { (*p_csr).b_eof } as i32;
}
extern "C" fn zipfile_read_eocd(p_tab_1: *mut ZipfileTab, a_blob_1: *const u8,
    n_blob_1: i64, p_file_1: *mut FILE, p_eocd_1: *mut ZipfileEOCD) -> i32 {
    let mut a_read: *mut u8 = unsafe { (*p_tab_1).a_buffer };
    let mut n_read: i64 = 0 as i64;
    let mut rc: i32 = 0;
    unsafe {
        memset(p_eocd_1 as *mut (), 0,
            core::mem::size_of::<ZipfileEOCD>() as u64)
    };
    if a_blob_1 == core::ptr::null() {
        let mut i_off: i64 = 0 as i64;
        let mut sz_file: i64 = 0 as i64;
        unsafe { fseek(p_file_1, 0 as i64, 2) };
        sz_file = unsafe { ftell(p_file_1) } as i64;
        if sz_file == 0 as i64 { return 0; }
        n_read =
            if sz_file < (200 * 1024) as i64 {
                        sz_file
                    } else { (200 * 1024) as i64 } as i32 as i64;
        i_off = sz_file - n_read;
        rc =
            zipfile_read_data(p_file_1, a_read, n_read, i_off,
                unsafe { &mut (*p_tab_1).base.z_err_msg });
    } else {
        n_read =
            if n_blob_1 < (200 * 1024) as i64 {
                        n_blob_1
                    } else { (200 * 1024) as i64 } as i32 as i64;
        a_read =
            unsafe {
                    &raw const *a_blob_1.offset((n_blob_1 - n_read) as isize)
                } as *mut u8;
    }
    if rc == 0 {
        let mut i: i64 = 0 as i64;
        {
            i = n_read - 20 as i64;
            '__b7: loop {
                if !(i >= 0 as i64) { break '__b7; }
                '__c7: loop {
                    if unsafe { *a_read.offset(i as isize) } as i32 == 80 &&
                                    unsafe { *a_read.offset((i + 1 as i64) as isize) } as i32 ==
                                        75 &&
                                unsafe { *a_read.offset((i + 2 as i64) as isize) } as i32 ==
                                    5 &&
                            unsafe { *a_read.offset((i + 3 as i64) as isize) } as i32 ==
                                6 {
                        break '__b7;
                    }
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        if i < 0 as i64 {
            unsafe {
                zipfile_table_err(unsafe { &mut *p_tab_1 },
                    c"cannot find end of central directory record".as_ptr() as
                            *mut i8 as *const i8)
            };
            return 1;
        }
        {
            let __n = i + 4 as i64;
            let __p = &mut a_read;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        unsafe {
            (*p_eocd_1).i_disk =
                {
                    {
                        let __n = 2;
                        let __p = &mut a_read;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                            *const u8)
                }
        };
        unsafe {
            (*p_eocd_1).i_first_disk =
                {
                    {
                        let __n = 2;
                        let __p = &mut a_read;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                            *const u8)
                }
        };
        unsafe {
            (*p_eocd_1).n_entry =
                {
                    {
                        let __n = 2;
                        let __p = &mut a_read;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                            *const u8)
                }
        };
        unsafe {
            (*p_eocd_1).n_entry_total =
                {
                    {
                        let __n = 2;
                        let __p = &mut a_read;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    zipfile_get_u16(unsafe { a_read.offset(-(2 as isize)) } as
                            *const u8)
                }
        };
        unsafe {
            (*p_eocd_1).n_size =
                {
                    {
                        let __n = 4;
                        let __p = &mut a_read;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                            *const u8)
                }
        };
        unsafe {
            (*p_eocd_1).i_offset =
                {
                    {
                        let __n = 4;
                        let __p = &mut a_read;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    zipfile_get_u32(unsafe { a_read.offset(-(4 as isize)) } as
                            *const u8)
                }
        };
    }
    return rc;
}
extern "C" fn zipfile_add_entry(p_tab_1: &mut ZipfileTab,
    p_before_1: *mut ZipfileEntry, p_new_1: *mut ZipfileEntry) -> () {
    if !(((*p_tab_1).p_first_entry == core::ptr::null_mut()) as i32 ==
                            ((*p_tab_1).p_last_entry == core::ptr::null_mut()) as i32)
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileAddEntry".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1262,
                c"(pTab->pFirstEntry==0)==(pTab->pLastEntry==0)".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*p_new_1).p_next } == core::ptr::null_mut()) as i32 as i64
            != 0 {
        unsafe {
            __assert_rtn(c"zipfileAddEntry".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1263,
                c"pNew->pNext==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if p_before_1 == core::ptr::null_mut() {
        if (*p_tab_1).p_first_entry == core::ptr::null_mut() {
            (*p_tab_1).p_first_entry =
                {
                    (*p_tab_1).p_last_entry = p_new_1;
                    (*p_tab_1).p_last_entry
                };
        } else {
            if !(unsafe { (*(*p_tab_1).p_last_entry).p_next } ==
                                    core::ptr::null_mut()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"zipfileAddEntry".as_ptr() as *const i8,
                        c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1268,
                        c"pTab->pLastEntry->pNext==0".as_ptr() as *mut i8 as
                            *const i8)
                }
            } else { { let _ = 0; } };
            unsafe { (*(*p_tab_1).p_last_entry).p_next = p_new_1 };
            (*p_tab_1).p_last_entry = p_new_1;
        }
    } else {
        let mut pp: *mut *mut ZipfileEntry = core::ptr::null_mut();
        {
            pp = &mut (*p_tab_1).p_first_entry;
            '__b8: loop {
                if !(unsafe { *pp } != p_before_1) { break '__b8; }
                '__c8: loop { break '__c8; }
                pp = unsafe { &mut (*unsafe { *pp }).p_next };
            }
        }
        unsafe { (*p_new_1).p_next = p_before_1 };
        unsafe { *pp = p_new_1 };
    }
}
extern "C" fn zipfile_load_directory(p_tab_1: *mut ZipfileTab,
    a_blob_1: *const u8, n_blob_1: i64) -> i32 {
    let mut eocd: ZipfileEOCD = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut i_off: i64 = 0 as i64;
    rc =
        zipfile_read_eocd(p_tab_1, a_blob_1, n_blob_1,
            unsafe { (*p_tab_1).p_write_fd }, &mut eocd);
    i_off = eocd.i_offset as i64;
    {
        i = 0;
        '__b9: loop {
            if !(rc == 0 && i < eocd.n_entry as i32) { break '__b9; }
            '__c9: loop {
                let mut p_new: *mut ZipfileEntry = core::ptr::null_mut();
                rc =
                    zipfile_get_entry(p_tab_1, a_blob_1, n_blob_1,
                        unsafe { (*p_tab_1).p_write_fd }, i_off, &mut p_new);
                if rc == 0 {
                    zipfile_add_entry(unsafe { &mut *p_tab_1 },
                        core::ptr::null_mut(), p_new);
                    i_off += 46 as i64;
                    i_off +=
                        (unsafe { (*p_new).cds.n_extra } as i32 +
                                    unsafe { (*p_new).cds.n_file } as i32 +
                                unsafe { (*p_new).cds.n_comment } as i32) as i64;
                }
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return rc;
}
extern "C" fn zipfile_filter(cur: *mut sqlite3_vtab_cursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut sqlite3_value) -> i32 {
    let p_tab: *mut ZipfileTab = unsafe { (*cur).p_vtab } as *mut ZipfileTab;
    let p_csr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
    let mut z_file: *const i8 = core::ptr::null();
    let mut rc: i32 = 0;
    let mut b_in_memory: i32 = 0;
    { let _ = idx_str_1; };
    { let _ = argc; };
    zipfile_reset_cursor(unsafe { &mut *p_csr });
    if !(unsafe { (*p_tab).z_file }).is_null() {
        z_file = unsafe { (*p_tab).z_file } as *const i8;
    } else if idx_num_1 == 0 {
        unsafe {
            zipfile_cursor_err(unsafe { &*p_csr },
                c"zipfile() function requires an argument".as_ptr() as *mut i8
                    as *const i8)
        };
        return 1;
    } else if unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } == 4 {
        let mut a_blob: *const u8 =
            unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) }
                as *const u8;
        let mut n_blob: i64 =
            unsafe {
                    sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                } as i64;
        if !(unsafe { (*p_tab).p_first_entry } == core::ptr::null_mut()) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"zipfileFilter".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1329,
                    c"pTab->pFirstEntry==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if a_blob == core::ptr::null() {
            a_blob = &a_empty_blob;
            n_blob = 0 as i64;
        }
        rc = zipfile_load_directory(p_tab, a_blob, n_blob);
        unsafe { (*p_csr).p_free_entry = unsafe { (*p_tab).p_first_entry } };
        unsafe {
            (*p_tab).p_first_entry =
                {
                    unsafe { (*p_tab).p_last_entry = core::ptr::null_mut() };
                    unsafe { (*p_tab).p_last_entry }
                }
        };
        if rc != 0 { return rc; }
        b_in_memory = 1;
    } else {
        z_file =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
    }
    if core::ptr::null_mut() == unsafe { (*p_tab).p_write_fd } &&
            0 == b_in_memory {
        unsafe {
            (*p_csr).p_file =
                if !(z_file).is_null() {
                    unsafe {
                        fopen(z_file, c"rb".as_ptr() as *mut i8 as *const i8)
                    }
                } else { core::ptr::null_mut() }
        };
        if unsafe { (*p_csr).p_file } == core::ptr::null_mut() {
            unsafe {
                zipfile_cursor_err(unsafe { &*p_csr },
                    c"cannot open file: %s".as_ptr() as *mut i8 as *const i8,
                    z_file)
            };
            rc = 1;
        } else {
            rc =
                zipfile_read_eocd(p_tab, core::ptr::null(), 0 as i64,
                    unsafe { (*p_csr).p_file }, unsafe { &mut (*p_csr).eocd });
            if rc == 0 {
                if unsafe { (*p_csr).eocd.n_entry } as i32 == 0 {
                    unsafe { (*p_csr).b_eof = 1 as u8 };
                } else {
                    unsafe {
                        (*p_csr).i_next_off =
                            unsafe { (*p_csr).eocd.i_offset } as i64
                    };
                    rc = zipfile_next(cur);
                }
            }
        }
    } else {
        unsafe { (*p_csr).b_noop = 1 as u8 };
        unsafe {
            (*p_csr).p_current =
                if !(unsafe { (*p_csr).p_free_entry }).is_null() {
                    unsafe { (*p_csr).p_free_entry }
                } else { unsafe { (*p_tab).p_first_entry } }
        };
        rc = zipfile_next(cur);
    }
    return rc;
}
extern "C" fn zipfile_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i: i32 = 0;
    let mut idx: i32 = -1;
    let mut unusable: i32 = 0;
    { let _ = tab; };
    {
        i = 0;
        '__b10: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b10;
            }
            '__c10: loop {
                let p_cons: *const sqlite3_index_constraint =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_idx_info_1).a_constraint.offset(i as isize)
                                    }
                        } as *const sqlite3_index_constraint;
                if unsafe { (*p_cons).i_column } as i32 != 7 { break '__c10; }
                if unsafe { (*p_cons).usable } as i32 == 0 {
                    unusable = 1;
                } else if unsafe { (*p_cons).op } as i32 == 2 { idx = i; }
                break '__c10;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p_idx_info_1).estimated_cost = 1000.0 };
    if idx >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(idx as isize)
                        }).argv_index = 1
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(idx as isize)
                        }).omit = 1 as u8
        };
        unsafe { (*p_idx_info_1).idx_num = 1 };
    } else if unusable != 0 { return 19; }
    return 0;
}
extern "C" fn zipfile_new_entry(z_path_1: *const i8) -> *mut ZipfileEntry {
    let mut p_new: *mut ZipfileEntry = core::ptr::null_mut();
    p_new =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<ZipfileEntry>() as
                        sqlite3_uint64)
            } as *mut ZipfileEntry;
    if !(p_new).is_null() {
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<ZipfileEntry>() as u64)
        };
        unsafe {
            (*p_new).cds.z_file =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_path_1)
                }
        };
        if unsafe { (*p_new).cds.z_file } == core::ptr::null_mut() {
            unsafe { sqlite3_free(p_new as *mut ()) };
            p_new = core::ptr::null_mut();
        }
    }
    return p_new;
}
extern "C" fn zipfile_serialize_lfh(p_entry_1: &mut ZipfileEntry,
    a_buf_1: *mut u8) -> i32 {
    let p_cds: *mut ZipfileCDS = &mut (*p_entry_1).cds;
    let mut a: *mut u8 = a_buf_1;
    unsafe { (*p_cds).n_extra = 9 as u16 };
    {
        zipfile_put_u32(a, 67324752 as u32);
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_version_extract });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).flags });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_compression });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).m_time });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).m_date });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).crc32 });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).sz_compressed });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).sz_uncompressed });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).n_file } as u16);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).n_extra });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    if !(a == unsafe { a_buf_1.offset(30 as isize) }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileSerializeLFH".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1432,
                c"a==&aBuf[ZIPFILE_LFH_FIXED_SZ]".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        memcpy(a as *mut (), unsafe { (*p_cds).z_file } as *const (),
            unsafe { (*p_cds).n_file } as i32 as u64)
    };
    {
        let __n = unsafe { (*p_cds).n_file } as i32;
        let __p = &mut a;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    {
        zipfile_put_u16(a, 21589 as u16);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, 5 as u16);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    unsafe {
        *{
                    let __p = &mut a;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = 1 as u8
    };
    {
        zipfile_put_u32(a, (*p_entry_1).m_unix_time);
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return unsafe { a.offset_from(a_buf_1) } as i64 as i32;
}
extern "C" fn zipfile_append_entry(p_tab_1: *mut ZipfileTab,
    p_entry_1: *mut ZipfileEntry, p_data_1: *const u8, n_data_1: i32) -> i32 {
    let a_buf: *mut u8 = unsafe { (*p_tab_1).a_buffer };
    let mut n_buf: i32 = 0;
    let mut rc: i32 = 0;
    n_buf = zipfile_serialize_lfh(unsafe { &mut *p_entry_1 }, a_buf);
    rc = zipfile_append_data(p_tab_1, a_buf as *const u8, n_buf);
    if rc == 0 {
        unsafe { (*p_entry_1).i_data_off = unsafe { (*p_tab_1).sz_current } };
        rc = zipfile_append_data(p_tab_1, p_data_1, n_data_1);
    }
    return rc;
}
extern "C" fn zipfile_get_mode(p_val_1: *mut sqlite3_value, b_is_dir_1: i32,
    p_mode_1: &mut u32, pz_err_1: &mut *mut i8) -> i32 {
    let mut z: *const i8 = core::ptr::null();
    let mut mode: u32 = 0 as u32;
    let z_template: [i8; 11] =
        [45 as i8, 114 as i8, 119 as i8, 120 as i8, 114 as i8, 119 as i8,
                120 as i8, 114 as i8, 119 as i8, 120 as i8, 0 as i8];
    let mut i: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s12:
            {
            match __state {
                0 => {
                    z = unsafe { sqlite3_value_text(p_val_1) } as *const i8;
                    __state = 3;
                }
                2 => {
                    *pz_err_1 =
                        unsafe {
                            sqlite3_mprintf(c"zipfile: parse error in mode: %s".as_ptr()
                                        as *mut i8 as *const i8, z)
                        };
                    __state = 37;
                }
                3 => { mode = 0 as u32; __state = 4; }
                4 => {
                    if z == core::ptr::null() {
                        __state = 6;
                    } else { __state = 7; }
                }
                5 => {
                    if (mode & 16384 as u32 == 0 as u32) as i32 == b_is_dir_1 {
                        __state = 33;
                    } else { __state = 32; }
                }
                6 => {
                    mode =
                        if b_is_dir_1 != 0 { 16384 + 493 } else { 32768 + 420 } as
                            u32;
                    __state = 5;
                }
                7 => {
                    if unsafe { *z.offset(0 as isize) } as i32 >= '0' as i32 &&
                            unsafe { *z.offset(0 as isize) } as i32 <= '9' as i32 {
                        __state = 8;
                    } else { __state = 9; }
                }
                8 => {
                    mode = unsafe { sqlite3_value_int(p_val_1) } as u32;
                    __state = 5;
                }
                9 => { __state = 10; }
                10 => { __state = 11; }
                11 => {
                    if unsafe { strlen(z) } != 10 as u64 {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => {
                    '__s13:
                        {
                        match unsafe { *z.offset(0 as isize) } {
                            45 => { __state = 15; }
                            100 => { __state = 16; }
                            108 => { __state = 17; }
                            _ => { __state = 18; }
                        }
                    }
                }
                13 => { __state = 2; }
                14 => { i = 1; __state = 26; }
                15 => { mode |= 32768 as u32; __state = 20; }
                16 => { mode |= 16384 as u32; __state = 22; }
                17 => { mode |= 40960 as u32; __state = 24; }
                18 => { __state = 2; }
                19 => { __state = 15; }
                20 => { __state = 14; }
                21 => { __state = 16; }
                22 => { __state = 14; }
                23 => { __state = 17; }
                24 => { __state = 14; }
                25 => { __state = 18; }
                26 => { if i < 10 { __state = 27; } else { __state = 5; } }
                27 => {
                    if unsafe { *z.offset(i as isize) } as i32 ==
                            z_template[i as usize] as i32 {
                        __state = 29;
                    } else { __state = 30; }
                }
                28 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 26;
                }
                29 => { mode |= (1 << 9 - i) as u32; __state = 28; }
                30 => {
                    if unsafe { *z.offset(i as isize) } as i32 != '-' as i32 {
                        __state = 31;
                    } else { __state = 28; }
                }
                31 => { __state = 2; }
                32 => { *p_mode_1 = mode; __state = 35; }
                33 => {
                    *pz_err_1 =
                        unsafe {
                            sqlite3_mprintf(c"zipfile: mode does not match data".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    __state = 34;
                }
                34 => { return 19; }
                35 => { return 0; }
                36 => { __state = 2; }
                37 => { return 1; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn zipfile_compare_path(z_a_1: *const i8, z_b_1: *const i8,
    mut n_b_1: i32) -> i32 {
    let mut n_a: i32 = unsafe { strlen(z_a_1) } as i32;
    if n_a > 0 &&
            unsafe { *z_a_1.offset((n_a - 1) as isize) } as i32 == '/' as i32
        {
        { let __p = &mut n_a; let __t = *__p; *__p -= 1; __t };
    }
    if n_b_1 > 0 &&
            unsafe { *z_b_1.offset((n_b_1 - 1) as isize) } as i32 ==
                '/' as i32 {
        { let __p = &mut n_b_1; let __t = *__p; *__p -= 1; __t };
    }
    if n_a == n_b_1 &&
            unsafe {
                    memcmp(z_a_1 as *const (), z_b_1 as *const (), n_a as u64)
                } == 0 {
        return 0;
    }
    return 1;
}
extern "C" fn zipfile_begin(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p_tab: *mut ZipfileTab = p_vtab_1 as *mut ZipfileTab;
    let mut rc: i32 = 0;
    if !(unsafe { (*p_tab).p_write_fd } == core::ptr::null_mut()) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileBegin".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1524,
                c"pTab->pWriteFd==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_tab).z_file } == core::ptr::null_mut() ||
            unsafe { *unsafe { (*p_tab).z_file.offset(0 as isize) } } as i32
                == 0 {
        unsafe {
            zipfile_table_err(unsafe { &mut *p_tab },
                c"zipfile: missing filename".as_ptr() as *mut i8 as *const i8)
        };
        return 1;
    }
    unsafe {
        (*p_tab).p_write_fd =
            unsafe {
                fopen(unsafe { (*p_tab).z_file } as *const i8,
                    c"ab+".as_ptr() as *mut i8 as *const i8)
            }
    };
    if unsafe { (*p_tab).p_write_fd } == core::ptr::null_mut() {
        unsafe {
            zipfile_table_err(unsafe { &mut *p_tab },
                c"zipfile: failed to open file %s for writing".as_ptr() as
                        *mut i8 as *const i8, unsafe { (*p_tab).z_file })
        };
        rc = 1;
    } else {
        unsafe { fseek(unsafe { (*p_tab).p_write_fd }, 0 as i64, 2) };
        unsafe {
            (*p_tab).sz_current =
                {
                    unsafe {
                        (*p_tab).sz_orig =
                            unsafe { ftell(unsafe { (*p_tab).p_write_fd }) } as i64
                    };
                    unsafe { (*p_tab).sz_orig }
                }
        };
        rc = zipfile_load_directory(p_tab, core::ptr::null(), 0 as i64);
    }
    if rc != 0 { zipfile_cleanup_transaction(unsafe { &mut *p_tab }); }
    return rc;
}
extern "C" fn zipfile_time() -> u32 {
    let p_vfs: *mut sqlite3_vfs =
        unsafe { sqlite3_vfs_find(core::ptr::null()) };
    let mut ret: u32 = 0 as u32;
    if p_vfs == core::ptr::null_mut() { return 0 as u32; }
    if unsafe { (*p_vfs).i_version } >= 2 &&
            unsafe { (*p_vfs).x_current_time_int64.is_some() } {
        let mut ms: i64 = 0 as i64;
        unsafe {
            (unsafe {
                    (*p_vfs).x_current_time_int64.unwrap()
                })(p_vfs, &mut ms)
        };
        ret = (ms / 1000 as i64 - 24405875 as i64 * 8640 as i64) as u32;
    } else {
        let mut day: f64 = 0.0;
        unsafe {
            (unsafe { (*p_vfs).x_current_time.unwrap() })(p_vfs, &mut day)
        };
        ret = ((day - 2440587.5) * 86400 as f64) as u32;
    }
    return ret;
}
extern "C" fn zipfile_get_time(p_val_1: *mut sqlite3_value) -> u32 {
    if p_val_1 == core::ptr::null_mut() ||
            unsafe { sqlite3_value_type(p_val_1) } == 5 {
        return zipfile_time();
    }
    return unsafe { sqlite3_value_int64(p_val_1) } as u32;
}
extern "C" fn zipfile_remove_entry_from_list(p_tab_1: &mut ZipfileTab,
    p_old_1: *mut ZipfileEntry) -> () {
    if !(p_old_1).is_null() {
        if (*p_tab_1).p_first_entry == p_old_1 {
            (*p_tab_1).p_first_entry = unsafe { (*p_old_1).p_next };
            if (*p_tab_1).p_last_entry == p_old_1 {
                (*p_tab_1).p_last_entry = core::ptr::null_mut();
            }
        } else {
            let mut p: *mut ZipfileEntry = core::ptr::null_mut();
            {
                p = (*p_tab_1).p_first_entry;
                '__b14: loop {
                    if !(!(p).is_null()) { break '__b14; }
                    '__c14: loop {
                        if unsafe { (*p).p_next } == p_old_1 {
                            unsafe { (*p).p_next = unsafe { (*p_old_1).p_next } };
                            if (*p_tab_1).p_last_entry == p_old_1 {
                                (*p_tab_1).p_last_entry = p;
                            }
                            break '__b14;
                        }
                        break '__c14;
                    }
                    p = unsafe { (*p).p_next };
                }
            }
        }
        zipfile_entry_free(p_old_1);
    }
}
extern "C" fn zipfile_update(p_vtab_1: *mut sqlite3_vtab, n_val_1: i32,
    ap_val_1: *mut *mut sqlite3_value, p_rowid_1: *mut sqlite_int64) -> i32 {
    let mut p_tab: *mut ZipfileTab = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_new: *mut ZipfileEntry = core::ptr::null_mut();
    let mut mode: u32 = 0 as u32;
    let mut m_time: u32 = 0 as u32;
    let mut sz: i64 = 0 as i64;
    let mut z_path: *const i8 = core::ptr::null();
    let mut n_path: i32 = 0;
    let mut p_data: *const u8 = core::ptr::null();
    let mut n_data: i32 = 0;
    let mut i_method: i32 = 0;
    let mut p_free: *mut u8 = core::ptr::null_mut();
    let mut z_free: *mut i8 = core::ptr::null_mut();
    let mut p_old: *mut ZipfileEntry = core::ptr::null_mut();
    let mut p_old2: *mut ZipfileEntry = core::ptr::null_mut();
    let mut b_update: i32 = 0;
    let mut b_is_dir: i32 = 0;
    let mut i_crc32: u32 = 0 as u32;
    let mut z_delete: *const i8 = core::ptr::null();
    let mut n_delete: i32 = 0;
    let mut z_update: *const i8 = core::ptr::null();
    let mut a_in: *const u8 = core::ptr::null();
    let mut n_in: i32 = 0;
    let mut b_auto: i32 = 0;
    let mut n_cmp: i32 = 0;
    let mut p: *mut ZipfileEntry = core::ptr::null_mut();
    let mut p_csr: *mut ZipfileCsr = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s16:
            {
            match __state {
                0 => { p_tab = p_vtab_1 as *mut ZipfileTab; __state = 3; }
                2 => {
                    unsafe { sqlite3_free(p_free as *mut ()) };
                    __state = 132;
                }
                3 => { rc = 0; __state = 4; }
                4 => { p_new = core::ptr::null_mut(); __state = 5; }
                5 => { mode = 0 as u32; __state = 6; }
                6 => { m_time = 0 as u32; __state = 7; }
                7 => { sz = 0 as i64; __state = 8; }
                8 => { z_path = core::ptr::null(); __state = 9; }
                9 => { n_path = 0; __state = 10; }
                10 => { p_data = core::ptr::null(); __state = 11; }
                11 => { n_data = 0; __state = 12; }
                12 => { i_method = 0; __state = 13; }
                13 => { p_free = core::ptr::null_mut(); __state = 14; }
                14 => { z_free = core::ptr::null_mut(); __state = 15; }
                15 => { p_old = core::ptr::null_mut(); __state = 16; }
                16 => { p_old2 = core::ptr::null_mut(); __state = 17; }
                17 => { b_update = 0; __state = 18; }
                18 => { b_is_dir = 0; __state = 19; }
                19 => { i_crc32 = 0 as u32; __state = 20; }
                20 => { { let _ = p_rowid_1; }; __state = 21; }
                21 => {
                    if unsafe { (*p_tab).p_write_fd } == core::ptr::null_mut() {
                        __state = 23;
                    } else { __state = 22; }
                }
                22 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *ap_val_1.offset(0 as isize) })
                            } != 5 {
                        __state = 27;
                    } else { __state = 26; }
                }
                23 => { rc = zipfile_begin(p_vtab_1); __state = 24; }
                24 => { if rc != 0 { __state = 25; } else { __state = 22; } }
                25 => { return rc; }
                26 => {
                    if n_val_1 > 1 { __state = 40; } else { __state = 39; }
                }
                27 => {
                    z_delete =
                        unsafe {
                                sqlite3_value_text(unsafe { *ap_val_1.offset(0 as isize) })
                            } as *const i8;
                    __state = 28;
                }
                28 => {
                    n_delete = unsafe { strlen(z_delete) } as i32;
                    __state = 29;
                }
                29 => {
                    if n_val_1 > 1 { __state = 31; } else { __state = 30; }
                }
                30 => {
                    p_old = unsafe { (*p_tab).p_first_entry };
                    __state = 34;
                }
                31 => {
                    z_update =
                        unsafe {
                                sqlite3_value_text(unsafe { *ap_val_1.offset(1 as isize) })
                            } as *const i8;
                    __state = 32;
                }
                32 => {
                    if !(z_update).is_null() &&
                            zipfile_compare_path(z_update, z_delete, n_delete) != 0 {
                        __state = 33;
                    } else { __state = 30; }
                }
                33 => { b_update = 1; __state = 30; }
                34 => { if 1 != 0 { __state = 35; } else { __state = 26; } }
                35 => {
                    if zipfile_compare_path(unsafe { (*p_old).cds.z_file } as
                                    *const i8, z_delete, n_delete) == 0 {
                        __state = 38;
                    } else { __state = 37; }
                }
                36 => { p_old = unsafe { (*p_old).p_next }; __state = 34; }
                37 => {
                    if (unsafe { (*p_old).p_next }).is_null() as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"zipfileUpdate".as_ptr() as *const i8,
                                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1660,
                                c"pOld->pNext".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 36;
                }
                38 => { __state = 26; }
                39 => {
                    if rc == 0 && (!(p_old).is_null() || !(p_old2).is_null()) {
                        __state = 123;
                    } else { __state = 122; }
                }
                40 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *ap_val_1.offset(5 as isize) })
                            } != 5 {
                        __state = 42;
                    } else { __state = 41; }
                }
                41 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *ap_val_1.offset(6 as isize) })
                            } != 5 {
                        __state = 45;
                    } else { __state = 44; }
                }
                42 => {
                    unsafe {
                        zipfile_table_err(unsafe { &mut *p_tab },
                            c"sz must be NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    __state = 43;
                }
                43 => { rc = 19; __state = 41; }
                44 => { if rc == 0 { __state = 48; } else { __state = 47; } }
                45 => {
                    unsafe {
                        zipfile_table_err(unsafe { &mut *p_tab },
                            c"rawdata must be NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    __state = 46;
                }
                46 => { rc = 19; __state = 44; }
                47 => { if rc == 0 { __state = 70; } else { __state = 69; } }
                48 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *ap_val_1.offset(7 as isize) })
                            } == 5 {
                        __state = 49;
                    } else { __state = 50; }
                }
                49 => { b_is_dir = 1; __state = 47; }
                50 => {
                    a_in =
                        unsafe {
                                sqlite3_value_blob(unsafe { *ap_val_1.offset(7 as isize) })
                            } as *const u8;
                    __state = 51;
                }
                51 => {
                    n_in =
                        unsafe {
                            sqlite3_value_bytes(unsafe { *ap_val_1.offset(7 as isize) })
                        };
                    __state = 52;
                }
                52 => {
                    b_auto =
                        (unsafe {
                                    sqlite3_value_type(unsafe { *ap_val_1.offset(8 as isize) })
                                } == 5) as i32;
                    __state = 53;
                }
                53 => {
                    i_method =
                        unsafe {
                            sqlite3_value_int(unsafe { *ap_val_1.offset(8 as isize) })
                        };
                    __state = 54;
                }
                54 => { sz = n_in as i64; __state = 55; }
                55 => { p_data = a_in; __state = 56; }
                56 => { n_data = n_in; __state = 57; }
                57 => {
                    if i_method != 0 && i_method != 8 {
                        __state = 58;
                    } else { __state = 59; }
                }
                58 => {
                    unsafe {
                        zipfile_table_err(unsafe { &mut *p_tab },
                            c"unknown compression method: %d".as_ptr() as *mut i8 as
                                *const i8, i_method)
                    };
                    __state = 60;
                }
                59 => {
                    if b_auto != 0 || i_method != 0 {
                        __state = 62;
                    } else { __state = 61; }
                }
                60 => { rc = 19; __state = 47; }
                61 => {
                    i_crc32 =
                        unsafe { crc32(0 as uLong, a_in, n_in as uInt) } as u32;
                    __state = 47;
                }
                62 => { __state = 63; }
                63 => {
                    rc =
                        zipfile_deflate(a_in, n_in, &mut p_free, &mut n_cmp,
                            unsafe { &mut (*p_tab).base.z_err_msg });
                    __state = 64;
                }
                64 => { if rc == 0 { __state = 65; } else { __state = 61; } }
                65 => {
                    if i_method != 0 || n_cmp < n_in {
                        __state = 66;
                    } else { __state = 61; }
                }
                66 => { i_method = 8; __state = 67; }
                67 => { p_data = p_free as *const u8; __state = 68; }
                68 => { n_data = n_cmp; __state = 61; }
                69 => { if rc == 0 { __state = 72; } else { __state = 71; } }
                70 => {
                    rc =
                        zipfile_get_mode(unsafe { *ap_val_1.offset(3 as isize) },
                            b_is_dir, &mut mode,
                            unsafe { &mut (*p_tab).base.z_err_msg });
                    __state = 69;
                }
                71 => {
                    if rc == 0 && b_is_dir != 0 {
                        __state = 81;
                    } else { __state = 80; }
                }
                72 => {
                    z_path =
                        unsafe {
                                sqlite3_value_text(unsafe { *ap_val_1.offset(2 as isize) })
                            } as *const i8;
                    __state = 73;
                }
                73 => {
                    if z_path == core::ptr::null() {
                        __state = 75;
                    } else { __state = 74; }
                }
                74 => {
                    n_path = unsafe { strlen(z_path) } as i32;
                    __state = 76;
                }
                75 => {
                    z_path = c"".as_ptr() as *mut i8 as *const i8;
                    __state = 74;
                }
                76 => {
                    if n_path > 250 { __state = 78; } else { __state = 77; }
                }
                77 => {
                    m_time =
                        zipfile_get_time(unsafe { *ap_val_1.offset(4 as isize) });
                    __state = 71;
                }
                78 => {
                    unsafe {
                        zipfile_table_err(unsafe { &mut *p_tab },
                            c"filename too long; max: %d bytes".as_ptr() as *mut i8 as
                                *const i8, 250)
                    };
                    __state = 79;
                }
                79 => { rc = 19; __state = 77; }
                80 => {
                    if (p_old == core::ptr::null_mut() || b_update != 0) &&
                            rc == 0 {
                        __state = 89;
                    } else { __state = 88; }
                }
                81 => {
                    if n_path <= 0 ||
                            unsafe { *z_path.offset((n_path - 1) as isize) } as i32 !=
                                '/' as i32 {
                        __state = 82;
                    } else { __state = 80; }
                }
                82 => {
                    z_free =
                        unsafe {
                            sqlite3_mprintf(c"%s/".as_ptr() as *mut i8 as *const i8,
                                z_path)
                        };
                    __state = 83;
                }
                83 => { z_path = z_free as *const i8; __state = 84; }
                84 => {
                    if z_free == core::ptr::null_mut() {
                        __state = 85;
                    } else { __state = 86; }
                }
                85 => { rc = 7; __state = 87; }
                86 => {
                    n_path = unsafe { strlen(z_path) } as i32;
                    __state = 80;
                }
                87 => { n_path = 0; __state = 80; }
                88 => { if rc == 0 { __state = 105; } else { __state = 39; } }
                89 => { __state = 90; }
                90 => { p = unsafe { (*p_tab).p_first_entry }; __state = 91; }
                91 => {
                    if !(p).is_null() { __state = 92; } else { __state = 88; }
                }
                92 => {
                    if zipfile_compare_path(unsafe { (*p).cds.z_file } as
                                    *const i8, z_path, n_path) == 0 {
                        __state = 94;
                    } else { __state = 93; }
                }
                93 => { p = unsafe { (*p).p_next }; __state = 91; }
                94 => {
                    '__s17:
                        {
                        match unsafe {
                                sqlite3_vtab_on_conflict(unsafe { (*p_tab).db })
                            } {
                            2 => { __state = 96; }
                            5 => { __state = 97; }
                            _ => { __state = 98; }
                        }
                    }
                }
                95 => { __state = 88; }
                96 => { __state = 2; }
                97 => { p_old2 = p; __state = 102; }
                98 => {
                    unsafe {
                        zipfile_table_err(unsafe { &mut *p_tab },
                            c"duplicate name: \"%s\"".as_ptr() as *mut i8 as *const i8,
                            z_path)
                    };
                    __state = 103;
                }
                99 => { __state = 96; }
                100 => { __state = 97; }
                101 => { __state = 98; }
                102 => { __state = 95; }
                103 => { rc = 19; __state = 104; }
                104 => { __state = 95; }
                105 => { p_new = zipfile_new_entry(z_path); __state = 106; }
                106 => {
                    if p_new == core::ptr::null_mut() {
                        __state = 107;
                    } else { __state = 108; }
                }
                107 => { rc = 7; __state = 39; }
                108 => {
                    unsafe {
                        (*p_new).cds.i_version_made_by = ((3 << 8) + 30) as u16
                    };
                    __state = 109;
                }
                109 => {
                    unsafe { (*p_new).cds.i_version_extract = 20 as u16 };
                    __state = 110;
                }
                110 => {
                    unsafe { (*p_new).cds.flags = 2048 as u16 };
                    __state = 111;
                }
                111 => {
                    unsafe { (*p_new).cds.i_compression = i_method as u16 };
                    __state = 112;
                }
                112 => {
                    zipfile_mtime_to_dos(unsafe { &mut (*p_new).cds }, m_time);
                    __state = 113;
                }
                113 => {
                    unsafe { (*p_new).cds.crc32 = i_crc32 };
                    __state = 114;
                }
                114 => {
                    unsafe { (*p_new).cds.sz_compressed = n_data as u32 };
                    __state = 115;
                }
                115 => {
                    unsafe { (*p_new).cds.sz_uncompressed = sz as u32 };
                    __state = 116;
                }
                116 => {
                    unsafe { (*p_new).cds.i_external_attr = mode << 16 };
                    __state = 117;
                }
                117 => {
                    unsafe {
                        (*p_new).cds.i_offset =
                            unsafe { (*p_tab).sz_current } as u32
                    };
                    __state = 118;
                }
                118 => {
                    unsafe { (*p_new).cds.n_file = n_path as u16 };
                    __state = 119;
                }
                119 => {
                    unsafe { (*p_new).m_unix_time = m_time as u32 };
                    __state = 120;
                }
                120 => {
                    rc = zipfile_append_entry(p_tab, p_new, p_data, n_data);
                    __state = 121;
                }
                121 => {
                    zipfile_add_entry(unsafe { &mut *p_tab }, p_old, p_new);
                    __state = 39;
                }
                122 => { __state = 2; }
                123 => { __state = 124; }
                124 => {
                    p_csr = unsafe { (*p_tab).p_csr_list };
                    __state = 126;
                }
                125 => {
                    zipfile_remove_entry_from_list(unsafe { &mut *p_tab },
                        p_old);
                    __state = 131;
                }
                126 => {
                    if !(p_csr).is_null() {
                        __state = 127;
                    } else { __state = 125; }
                }
                127 => {
                    if !(unsafe { (*p_csr).p_current }).is_null() &&
                            (unsafe { (*p_csr).p_current } == p_old ||
                                unsafe { (*p_csr).p_current } == p_old2) {
                        __state = 129;
                    } else { __state = 128; }
                }
                128 => {
                    p_csr = unsafe { (*p_csr).p_csr_next };
                    __state = 126;
                }
                129 => {
                    unsafe {
                        (*p_csr).p_current =
                            unsafe { (*unsafe { (*p_csr).p_current }).p_next }
                    };
                    __state = 130;
                }
                130 => {
                    unsafe { (*p_csr).b_noop = 1 as u8 };
                    __state = 128;
                }
                131 => {
                    zipfile_remove_entry_from_list(unsafe { &mut *p_tab },
                        p_old2);
                    __state = 122;
                }
                132 => {
                    unsafe { sqlite3_free(z_free as *mut ()) };
                    __state = 133;
                }
                133 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn zipfile_serialize_eocd(p: &ZipfileEOCD, a_buf_1: *mut u8)
    -> i32 {
    let mut a: *mut u8 = a_buf_1;
    {
        zipfile_put_u32(a, 101010256 as u32);
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, (*p).i_disk);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, (*p).i_first_disk);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, (*p).n_entry);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, (*p).n_entry_total);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, (*p).n_size);
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, (*p).i_offset);
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, 0 as u16);
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    return unsafe { a.offset_from(a_buf_1) } as i64 as i32;
}
extern "C" fn zipfile_append_eocd(p_tab_1: *mut ZipfileTab,
    p: *mut ZipfileEOCD) -> i32 {
    let n_buf: i32 =
        zipfile_serialize_eocd(unsafe { &*p },
            unsafe { (*p_tab_1).a_buffer });
    if !(n_buf == 22) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileAppendEOCD".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1827,
                c"nBuf==ZIPFILE_EOCD_FIXED_SZ".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    return zipfile_append_data(p_tab_1,
            unsafe { (*p_tab_1).a_buffer } as *const u8, n_buf);
}
extern "C" fn zipfile_serialize_cds(p_entry_1: &mut ZipfileEntry,
    a_buf_1: *mut u8) -> i32 {
    let mut a: *mut u8 = a_buf_1;
    let p_cds: *mut ZipfileCDS = &mut (*p_entry_1).cds;
    if (*p_entry_1).a_extra == core::ptr::null_mut() {
        unsafe { (*p_cds).n_extra = 9 as u16 };
    }
    {
        zipfile_put_u32(a, 33639248 as u32);
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_version_made_by });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_version_extract });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).flags });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_compression });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).m_time });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).m_date });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).crc32 });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).sz_compressed });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).sz_uncompressed });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    if !(a == unsafe { a_buf_1.offset(28 as isize) }) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileSerializeCDS".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1853,
                c"a==&aBuf[ZIPFILE_CDS_NFILE_OFF]".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    {
        zipfile_put_u16(a, unsafe { (*p_cds).n_file });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).n_extra });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).n_comment });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_disk_start });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u16(a, unsafe { (*p_cds).i_internal_attr });
        {
            let __n = 2;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).i_external_attr });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    {
        zipfile_put_u32(a, unsafe { (*p_cds).i_offset });
        {
            let __n = 4;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    }
    unsafe {
        memcpy(a as *mut (), unsafe { (*p_cds).z_file } as *const (),
            unsafe { (*p_cds).n_file } as u64)
    };
    {
        let __n = unsafe { (*p_cds).n_file };
        let __p = &mut a;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    if !((*p_entry_1).a_extra).is_null() {
        let n: i32 =
            unsafe { (*p_cds).n_extra } as i32 +
                unsafe { (*p_cds).n_comment } as i32;
        unsafe {
            memcpy(a as *mut (), (*p_entry_1).a_extra as *const (), n as u64)
        };
        {
            let __n = n;
            let __p = &mut a;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
    } else {
        if !(unsafe { (*p_cds).n_extra } as i32 == 9) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"zipfileSerializeCDS".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1870,
                    c"pCDS->nExtra==9".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        {
            zipfile_put_u16(a, 21589 as u16);
            {
                let __n = 2;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        {
            zipfile_put_u16(a, 5 as u16);
            {
                let __n = 2;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        unsafe {
            *{
                        let __p = &mut a;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = 1 as u8
        };
        {
            zipfile_put_u32(a, (*p_entry_1).m_unix_time);
            {
                let __n = 4;
                let __p = &mut a;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
    }
    return unsafe { a.offset_from(a_buf_1) } as i64 as i32;
}
extern "C" fn zipfile_commit(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p_tab: *mut ZipfileTab = p_vtab_1 as *mut ZipfileTab;
    let mut rc: i32 = 0;
    if !(unsafe { (*p_tab).p_write_fd }).is_null() {
        let i_offset: i64 = unsafe { (*p_tab).sz_current };
        let mut p: *mut ZipfileEntry = core::ptr::null_mut();
        let mut eocd: ZipfileEOCD = unsafe { core::mem::zeroed() };
        let mut n_entry: i32 = 0;
        {
            p = unsafe { (*p_tab).p_first_entry };
            '__b18: loop {
                if !(rc == 0 && !(p).is_null()) { break '__b18; }
                '__c18: loop {
                    let n: i32 =
                        zipfile_serialize_cds(unsafe { &mut *p },
                            unsafe { (*p_tab).a_buffer });
                    rc =
                        zipfile_append_data(p_tab,
                            unsafe { (*p_tab).a_buffer } as *const u8, n);
                    { let __p = &mut n_entry; let __t = *__p; *__p += 1; __t };
                    break '__c18;
                }
                p = unsafe { (*p).p_next };
            }
        }
        eocd.i_disk = 0 as u16;
        eocd.i_first_disk = 0 as u16;
        eocd.n_entry = n_entry as u16;
        eocd.n_entry_total = n_entry as u16;
        eocd.n_size = (unsafe { (*p_tab).sz_current } - i_offset) as u32;
        eocd.i_offset = i_offset as u32;
        rc = zipfile_append_eocd(p_tab, &mut eocd);
        zipfile_cleanup_transaction(unsafe { &mut *p_tab });
    }
    return rc;
}
extern "C" fn zipfile_rollback(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    return zipfile_commit(p_vtab_1);
}
extern "C" fn zipfile_find_cursor(p_tab_1: &ZipfileTab, i_id_1: i64)
    -> *mut ZipfileCsr {
    let mut p_csr: *mut ZipfileCsr = core::ptr::null_mut();
    {
        p_csr = (*p_tab_1).p_csr_list;
        '__b19: loop {
            if !(!(p_csr).is_null()) { break '__b19; }
            '__c19: loop {
                if i_id_1 == unsafe { (*p_csr).i_id } { break '__b19; }
                break '__c19;
            }
            p_csr = unsafe { (*p_csr).p_csr_next };
        }
    }
    return p_csr;
}
extern "C" fn zipfile_function_cds(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p_csr: *const ZipfileCsr = core::ptr::null();
    let p_tab: *mut ZipfileTab =
        unsafe { sqlite3_user_data(context) } as *mut ZipfileTab;
    if !(argc > 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"zipfileFunctionCds".as_ptr() as *const i8,
                c"zipfile.c".as_ptr() as *mut i8 as *const i8, 1929,
                c"argc>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_csr =
        zipfile_find_cursor(unsafe { &*p_tab },
            unsafe {
                sqlite3_value_int64(unsafe { *argv.offset(0 as isize) })
            });
    if !(p_csr).is_null() {
        let p: *const ZipfileCDS =
            unsafe { &raw mut (*unsafe { (*p_csr).p_current }).cds } as
                *const ZipfileCDS;
        let z_res: *mut i8 =
            unsafe {
                sqlite3_mprintf(c"{\"version-made-by\" : %u, \"version-to-extract\" : %u, \"flags\" : %u, \"compression\" : %u, \"time\" : %u, \"date\" : %u, \"crc32\" : %u, \"compressed-size\" : %u, \"uncompressed-size\" : %u, \"file-name-length\" : %u, \"extra-field-length\" : %u, \"file-comment-length\" : %u, \"disk-number-start\" : %u, \"internal-attr\" : %u, \"external-attr\" : %u, \"offset\" : %u }".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { (*p).i_version_made_by } as u32,
                    unsafe { (*p).i_version_extract } as u32,
                    unsafe { (*p).flags } as u32,
                    unsafe { (*p).i_compression } as u32,
                    unsafe { (*p).m_time } as u32,
                    unsafe { (*p).m_date } as u32, unsafe { (*p).crc32 } as u32,
                    unsafe { (*p).sz_compressed } as u32,
                    unsafe { (*p).sz_uncompressed } as u32,
                    unsafe { (*p).n_file } as u32,
                    unsafe { (*p).n_extra } as u32,
                    unsafe { (*p).n_comment } as u32,
                    unsafe { (*p).i_disk_start } as u32,
                    unsafe { (*p).i_internal_attr } as u32,
                    unsafe { (*p).i_external_attr } as u32,
                    unsafe { (*p).i_offset } as u32)
            };
        if z_res == core::ptr::null_mut() {
            unsafe { sqlite3_result_error_nomem(context) };
        } else {
            unsafe {
                sqlite3_result_text(context, z_res as *const i8, -1,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
            unsafe { sqlite3_free(z_res as *mut ()) };
        }
    }
}
extern "C" fn zipfile_find_function(p_vtab_1: *mut sqlite3_vtab, n_arg_1: i32,
    z_name_1: *const i8,
    px_func_1:
        *mut unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> (), pp_arg_1: *mut *mut ()) -> i32 {
    { let _ = n_arg_1; };
    if unsafe {
                sqlite3_stricmp(c"zipfile_cds".as_ptr() as *mut i8 as
                        *const i8, z_name_1)
            } == 0 {
        unsafe { *px_func_1 = zipfile_function_cds };
        unsafe { *pp_arg_1 = p_vtab_1 as *mut () };
        return 1;
    }
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileBuffer {
    a: *mut u8,
    n: i32,
    n_alloc: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ZipfileCtx {
    n_entry: i32,
    body: ZipfileBuffer,
    cds: ZipfileBuffer,
}
extern "C" fn zipfile_buffer_grow(p_buf_1: &mut ZipfileBuffer, n_byte_1: i64)
    -> i32 {
    if (((*p_buf_1).n_alloc - (*p_buf_1).n) as i64) < n_byte_1 {
        let mut a_new: *mut u8 = core::ptr::null_mut();
        let mut n_new: i64 =
            if (*p_buf_1).n != 0 {
                (*p_buf_1).n as i64 * 2 as i64
            } else { 512 as i64 };
        let n_req: i64 = (*p_buf_1).n as i64 + n_byte_1;
        while n_new < n_req { n_new = n_new * 2 as i64; }
        a_new =
            unsafe {
                    sqlite3_realloc64((*p_buf_1).a as *mut (),
                        n_new as sqlite3_uint64)
                } as *mut u8;
        if a_new == core::ptr::null_mut() { return 7; }
        (*p_buf_1).a = a_new;
        (*p_buf_1).n_alloc = n_new as i32;
    }
    return 0;
}
extern "C" fn zipfile_step(p_ctx_1: *mut sqlite3_context, n_val_1: i32,
    ap_val_1: *mut *mut sqlite3_value) -> () {
    let mut p: *mut ZipfileCtx = core::ptr::null_mut();
    let mut e: ZipfileEntry = unsafe { core::mem::zeroed() };
    let mut p_name: *mut sqlite3_value = core::ptr::null_mut();
    let mut p_mode: *mut sqlite3_value = core::ptr::null_mut();
    let mut p_mtime: *mut sqlite3_value = core::ptr::null_mut();
    let mut p_data: *mut sqlite3_value = core::ptr::null_mut();
    let mut p_method: *mut sqlite3_value = core::ptr::null_mut();
    let mut b_is_dir: i32 = 0;
    let mut mode: u32 = 0 as u32;
    let mut rc: i32 = 0;
    let mut z_err: *mut i8 = core::ptr::null_mut();
    let mut i_method: i32 = 0;
    let mut a_data: *const u8 = core::ptr::null();
    let mut n_data: i32 = 0;
    let mut sz_uncompressed: i32 = 0;
    let mut a_free: *mut u8 = core::ptr::null_mut();
    let mut i_crc32: u32 = 0 as u32;
    let mut z_name: *mut i8 = core::ptr::null_mut();
    let mut n_name: i32 = 0;
    let mut z_free: *mut i8 = core::ptr::null_mut();
    let mut n_byte: i64 = 0 as i64;
    let mut n_out: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s22:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe { sqlite3_free(a_free as *mut ()) };
                    __state = 115;
                }
                3 => { __state = 4; }
                4 => { p_name = core::ptr::null_mut(); __state = 5; }
                5 => { p_mode = core::ptr::null_mut(); __state = 6; }
                6 => { p_mtime = core::ptr::null_mut(); __state = 7; }
                7 => { p_data = core::ptr::null_mut(); __state = 8; }
                8 => { p_method = core::ptr::null_mut(); __state = 9; }
                9 => { b_is_dir = 0; __state = 10; }
                10 => { __state = 11; }
                11 => { rc = 0; __state = 12; }
                12 => { z_err = core::ptr::null_mut(); __state = 13; }
                13 => { i_method = -1; __state = 14; }
                14 => { a_data = core::ptr::null(); __state = 15; }
                15 => { n_data = 0; __state = 16; }
                16 => { sz_uncompressed = 0; __state = 17; }
                17 => { a_free = core::ptr::null_mut(); __state = 18; }
                18 => { i_crc32 = 0 as u32; __state = 19; }
                19 => { z_name = core::ptr::null_mut(); __state = 20; }
                20 => { n_name = 0; __state = 21; }
                21 => { z_free = core::ptr::null_mut(); __state = 22; }
                22 => { __state = 23; }
                23 => {
                    unsafe {
                        memset(&raw mut e as *mut (), 0,
                            core::mem::size_of::<ZipfileEntry>() as u64)
                    };
                    __state = 24;
                }
                24 => {
                    p =
                        unsafe {
                                sqlite3_aggregate_context(p_ctx_1,
                                    core::mem::size_of::<ZipfileCtx>() as i32)
                            } as *mut ZipfileCtx;
                    __state = 25;
                }
                25 => {
                    if p == core::ptr::null_mut() {
                        __state = 27;
                    } else { __state = 26; }
                }
                26 => {
                    if n_val_1 != 2 && n_val_1 != 4 && n_val_1 != 5 {
                        __state = 29;
                    } else { __state = 28; }
                }
                27 => { return; }
                28 => {
                    p_name = unsafe { *ap_val_1.offset(0 as isize) };
                    __state = 32;
                }
                29 => {
                    z_err =
                        unsafe {
                            sqlite3_mprintf(c"wrong number of arguments to function zipfile()".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    __state = 30;
                }
                30 => { rc = 1; __state = 31; }
                31 => { __state = 2; }
                32 => {
                    if n_val_1 == 2 { __state = 34; } else { __state = 35; }
                }
                33 => {
                    z_name = unsafe { sqlite3_value_text(p_name) } as *mut i8;
                    __state = 40;
                }
                34 => {
                    p_data = unsafe { *ap_val_1.offset(1 as isize) };
                    __state = 33;
                }
                35 => {
                    p_mode = unsafe { *ap_val_1.offset(1 as isize) };
                    __state = 36;
                }
                36 => {
                    p_mtime = unsafe { *ap_val_1.offset(2 as isize) };
                    __state = 37;
                }
                37 => {
                    p_data = unsafe { *ap_val_1.offset(3 as isize) };
                    __state = 38;
                }
                38 => {
                    if n_val_1 == 5 { __state = 39; } else { __state = 33; }
                }
                39 => {
                    p_method = unsafe { *ap_val_1.offset(4 as isize) };
                    __state = 33;
                }
                40 => {
                    n_name = unsafe { sqlite3_value_bytes(p_name) };
                    __state = 41;
                }
                41 => {
                    if z_name == core::ptr::null_mut() {
                        __state = 43;
                    } else { __state = 42; }
                }
                42 => {
                    if n_name > 250 { __state = 47; } else { __state = 46; }
                }
                43 => {
                    z_err =
                        unsafe {
                            sqlite3_mprintf(c"first argument to zipfile() must be non-NULL".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    __state = 44;
                }
                44 => { rc = 1; __state = 45; }
                45 => { __state = 2; }
                46 => {
                    if !(p_method).is_null() &&
                            5 != unsafe { sqlite3_value_type(p_method) } {
                        __state = 51;
                    } else { __state = 50; }
                }
                47 => {
                    z_err =
                        unsafe {
                            sqlite3_mprintf(c"filename argument to zipfile() too big; max: %d bytes".as_ptr()
                                        as *mut i8 as *const i8, 250)
                        };
                    __state = 48;
                }
                48 => { rc = 1; __state = 49; }
                49 => { __state = 2; }
                50 => {
                    if unsafe { sqlite3_value_type(p_data) } == 5 {
                        __state = 57;
                    } else { __state = 58; }
                }
                51 => {
                    i_method = unsafe { sqlite3_value_int64(p_method) } as i32;
                    __state = 52;
                }
                52 => {
                    if i_method != 0 && i_method != 8 {
                        __state = 53;
                    } else { __state = 50; }
                }
                53 => {
                    z_err =
                        unsafe {
                            sqlite3_mprintf(c"illegal method value: %d".as_ptr() as
                                        *mut i8 as *const i8, i_method)
                        };
                    __state = 54;
                }
                54 => { rc = 1; __state = 55; }
                55 => { __state = 2; }
                56 => {
                    rc =
                        zipfile_get_mode(p_mode, b_is_dir, &mut mode, &mut z_err);
                    __state = 72;
                }
                57 => { b_is_dir = 1; __state = 59; }
                58 => {
                    a_data = unsafe { sqlite3_value_blob(p_data) } as *const u8;
                    __state = 60;
                }
                59 => { i_method = 0; __state = 56; }
                60 => {
                    sz_uncompressed =
                        { n_data = unsafe { sqlite3_value_bytes(p_data) }; n_data };
                    __state = 61;
                }
                61 => {
                    i_crc32 =
                        unsafe { crc32(0 as uLong, a_data, n_data as uInt) } as u32;
                    __state = 62;
                }
                62 => {
                    if i_method < 0 || i_method == 8 {
                        __state = 63;
                    } else { __state = 56; }
                }
                63 => { n_out = 0; __state = 64; }
                64 => {
                    rc =
                        zipfile_deflate(a_data, n_data, &mut a_free, &mut n_out,
                            &mut z_err);
                    __state = 65;
                }
                65 => { if rc != 0 { __state = 67; } else { __state = 66; } }
                66 => {
                    if i_method == 8 || n_out < n_data {
                        __state = 68;
                    } else { __state = 69; }
                }
                67 => { __state = 2; }
                68 => { a_data = a_free as *const u8; __state = 70; }
                69 => { i_method = 0; __state = 56; }
                70 => { n_data = n_out; __state = 71; }
                71 => { i_method = 8; __state = 56; }
                72 => { if rc != 0 { __state = 74; } else { __state = 73; } }
                73 => {
                    e.m_unix_time = zipfile_get_time(p_mtime);
                    __state = 75;
                }
                74 => { __state = 2; }
                75 => {
                    if b_is_dir == 0 { __state = 77; } else { __state = 78; }
                }
                76 => {
                    e.cds.i_version_made_by = ((3 << 8) + 30) as u16;
                    __state = 89;
                }
                77 => {
                    if n_name > 0 &&
                            unsafe { *z_name.offset((n_name - 1) as isize) } as i32 ==
                                '/' as i32 {
                        __state = 79;
                    } else { __state = 76; }
                }
                78 => {
                    if n_name == 0 ||
                            unsafe { *z_name.offset((n_name - 1) as isize) } as i32 !=
                                '/' as i32 {
                        __state = 82;
                    } else { __state = 83; }
                }
                79 => {
                    z_err =
                        unsafe {
                            sqlite3_mprintf(c"non-directory name must not end with /".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    __state = 80;
                }
                80 => { rc = 1; __state = 81; }
                81 => { __state = 2; }
                82 => {
                    z_name =
                        {
                            z_free =
                                unsafe {
                                    sqlite3_mprintf(c"%s/".as_ptr() as *mut i8 as *const i8,
                                        z_name)
                                };
                            z_free
                        };
                    __state = 84;
                }
                83 => {
                    if n_name > 1 &&
                            unsafe { *z_name.offset((n_name - 2) as isize) } as i32 ==
                                '/' as i32 {
                        __state = 88;
                    } else { __state = 76; }
                }
                84 => {
                    if z_name == core::ptr::null_mut() {
                        __state = 86;
                    } else { __state = 85; }
                }
                85 => {
                    n_name = unsafe { strlen(z_name as *const i8) } as i32;
                    __state = 76;
                }
                86 => { rc = 7; __state = 87; }
                87 => { __state = 2; }
                88 => {
                    { let __p = &mut n_name; let __t = *__p; *__p -= 1; __t };
                    __state = 83;
                }
                89 => { e.cds.i_version_extract = 20 as u16; __state = 90; }
                90 => { e.cds.flags = 2048 as u16; __state = 91; }
                91 => { e.cds.i_compression = i_method as u16; __state = 92; }
                92 => {
                    zipfile_mtime_to_dos(&mut e.cds, e.m_unix_time as u32);
                    __state = 93;
                }
                93 => { e.cds.crc32 = i_crc32; __state = 94; }
                94 => { e.cds.sz_compressed = n_data as u32; __state = 95; }
                95 => {
                    e.cds.sz_uncompressed = sz_uncompressed as u32;
                    __state = 96;
                }
                96 => { e.cds.i_external_attr = mode << 16; __state = 97; }
                97 => {
                    e.cds.i_offset = unsafe { (*p).body.n } as u32;
                    __state = 98;
                }
                98 => { e.cds.n_file = n_name as u16; __state = 99; }
                99 => { e.cds.z_file = z_name; __state = 100; }
                100 => {
                    n_byte = (30 + e.cds.n_file as i32 + 9) as i64;
                    __state = 101;
                }
                101 => {
                    if {
                                rc = zipfile_buffer_grow(unsafe { &mut (*p).body }, n_byte);
                                rc
                            } != 0 {
                        __state = 103;
                    } else { __state = 102; }
                }
                102 => {
                    unsafe {
                        (*p).body.n +=
                            zipfile_serialize_lfh(&mut e,
                                unsafe {
                                    &mut *unsafe {
                                                (*p).body.a.offset(unsafe { (*p).body.n } as isize)
                                            }
                                })
                    };
                    __state = 104;
                }
                103 => { __state = 2; }
                104 => {
                    if n_data > 0 { __state = 106; } else { __state = 105; }
                }
                105 => {
                    n_byte = (46 + e.cds.n_file as i32 + 9) as i64;
                    __state = 110;
                }
                106 => {
                    if {
                                rc =
                                    zipfile_buffer_grow(unsafe { &mut (*p).body },
                                        n_data as i64);
                                rc
                            } != 0 {
                        __state = 108;
                    } else { __state = 107; }
                }
                107 => {
                    unsafe {
                        memcpy(unsafe {
                                    &raw mut *unsafe {
                                                (*p).body.a.offset(unsafe { (*p).body.n } as isize)
                                            }
                                } as *mut (), a_data as *const (), n_data as u64)
                    };
                    __state = 109;
                }
                108 => { __state = 2; }
                109 => { unsafe { (*p).body.n += n_data }; __state = 105; }
                110 => {
                    if {
                                rc = zipfile_buffer_grow(unsafe { &mut (*p).cds }, n_byte);
                                rc
                            } != 0 {
                        __state = 112;
                    } else { __state = 111; }
                }
                111 => {
                    unsafe {
                        (*p).cds.n +=
                            zipfile_serialize_cds(&mut e,
                                unsafe {
                                    &mut *unsafe {
                                                (*p).cds.a.offset(unsafe { (*p).cds.n } as isize)
                                            }
                                })
                    };
                    __state = 113;
                }
                112 => { __state = 2; }
                113 => {
                    {
                        let __p = unsafe { &mut (*p).n_entry };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 114;
                }
                114 => { __state = 2; }
                115 => {
                    unsafe { sqlite3_free(z_free as *mut ()) };
                    __state = 116;
                }
                116 => {
                    if rc != 0 { __state = 118; } else { __state = 117; }
                }
                117 => {
                    unsafe { sqlite3_free(z_err as *mut ()) };
                    __state = 1;
                }
                118 => {
                    if !(z_err).is_null() {
                        __state = 119;
                    } else { __state = 120; }
                }
                119 => {
                    unsafe {
                        sqlite3_result_error(p_ctx_1, z_err as *const i8, -1)
                    };
                    __state = 117;
                }
                120 => {
                    unsafe { sqlite3_result_error_code(p_ctx_1, rc) };
                    __state = 117;
                }
                _ => {}
            }
        }
    }
}
extern "C" fn zipfile_final(p_ctx_1: *mut sqlite3_context) -> () {
    let mut p: *const ZipfileCtx = core::ptr::null();
    let mut eocd: ZipfileEOCD = unsafe { core::mem::zeroed() };
    let mut n_zip: sqlite3_int64 = 0 as sqlite3_int64;
    let mut a_zip: *mut u8 = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<ZipfileCtx>() as i32)
            } as *mut ZipfileCtx;
    if p == core::ptr::null_mut() { return; }
    if unsafe { (*p).n_entry } > 0 {
        unsafe {
            memset(&raw mut eocd as *mut (), 0,
                core::mem::size_of::<ZipfileEOCD>() as u64)
        };
        eocd.n_entry = unsafe { (*p).n_entry } as u16;
        eocd.n_entry_total = unsafe { (*p).n_entry } as u16;
        eocd.n_size = unsafe { (*p).cds.n } as u32;
        eocd.i_offset = unsafe { (*p).body.n } as u32;
        n_zip =
            unsafe { (*p).body.n } as i64 + unsafe { (*p).cds.n } as i64 +
                22 as i64;
        a_zip =
            unsafe { sqlite3_malloc64(n_zip as sqlite3_uint64) } as *mut u8;
        if a_zip == core::ptr::null_mut() {
            unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        } else {
            unsafe {
                memcpy(a_zip as *mut (), unsafe { (*p).body.a } as *const (),
                    unsafe { (*p).body.n } as u64)
            };
            unsafe {
                memcpy(unsafe {
                            &raw mut *a_zip.offset(unsafe { (*p).body.n } as isize)
                        } as *mut (), unsafe { (*p).cds.a } as *const (),
                    unsafe { (*p).cds.n } as u64)
            };
            zipfile_serialize_eocd(&eocd,
                unsafe {
                    &mut *a_zip.offset((unsafe { (*p).body.n } +
                                        unsafe { (*p).cds.n }) as isize)
                });
            unsafe {
                sqlite3_result_blob(p_ctx_1, a_zip as *const (), n_zip as i32,
                    Some(zipfile_free))
            };
        }
    }
    unsafe { sqlite3_free(unsafe { (*p).body.a } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p).cds.a } as *mut ()) };
}
extern "C" fn zipfile_register(db: *mut sqlite3) -> i32 {
    unsafe {
        let mut rc: i32 =
            unsafe {
                sqlite3_create_module(db,
                    c"zipfile".as_ptr() as *mut i8 as *const i8,
                    &raw mut zipfile_module as *const sqlite3_module,
                    core::ptr::null_mut())
            };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_overload_function(db,
                        c"zipfile_cds".as_ptr() as *mut i8 as *const i8, -1)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_function(db,
                        c"zipfile".as_ptr() as *mut i8 as *const i8, -1, 1,
                        core::ptr::null_mut(), None, Some(zipfile_step),
                        Some(zipfile_final))
                };
        }
        if !(core::mem::size_of::<i64>() as u64 == 8 as u64) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"zipfileRegister".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 2279,
                    c"sizeof(i64)==8".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(core::mem::size_of::<u32>() as u64 == 4 as u64) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"zipfileRegister".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 2280,
                    c"sizeof(u32)==4".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(core::mem::size_of::<u16>() as u64 == 2 as u64) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"zipfileRegister".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 2281,
                    c"sizeof(u16)==2".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(core::mem::size_of::<u8>() as u64 == 1 as u64) as i32 as i64 != 0
            {
            unsafe {
                __assert_rtn(c"zipfileRegister".as_ptr() as *const i8,
                    c"zipfile.c".as_ptr() as *mut i8 as *const i8, 2282,
                    c"sizeof(u8)==1".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_zipfile_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };
    return zipfile_register(db);
}
static sz_fix: i32 = 30 as i32;
static a_empty_blob: u8 = 0 as u8;
static mut zipfile_module: sqlite3_module =
    sqlite3_module {
        i_version: 1,
        x_create: Some(zipfile_connect),
        x_connect: Some(zipfile_connect),
        x_best_index: Some(zipfile_best_index),
        x_disconnect: Some(zipfile_disconnect),
        x_destroy: Some(zipfile_disconnect),
        x_open: Some(zipfile_open),
        x_close: Some(zipfile_close),
        x_filter: Some(zipfile_filter),
        x_next: Some(zipfile_next),
        x_eof: Some(zipfile_eof),
        x_column: Some(zipfile_column),
        x_rowid: None,
        x_update: Some(zipfile_update),
        x_begin: Some(zipfile_begin),
        x_sync: None,
        x_commit: Some(zipfile_commit),
        x_rollback: Some(zipfile_rollback),
        x_find_function: Some(zipfile_find_function),
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
    fn strlen(__s: *const i8)
    -> u64;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fclose(_: *mut FILE)
    -> i32;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn inflateInit2_(strm: z_streamp, windowBits: i32, version: *const i8,
    stream_size: i32)
    -> i32;
    fn inflate(strm: z_streamp, flush: i32)
    -> i32;
    fn inflateEnd(strm: z_streamp)
    -> i32;
    fn deflateInit2_(strm: z_streamp, level: i32, method: i32,
    windowBits: i32, memLevel: i32, strategy: i32, version: *const i8,
    stream_size: i32)
    -> i32;
    fn deflateBound(strm: z_streamp, sourceLen: uLong)
    -> uLong;
    fn deflate(strm: z_streamp, flush: i32)
    -> i32;
    fn deflateEnd(strm: z_streamp)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt)
    -> uLong;
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
#[repr(C)]
#[derive(Copy, Clone)]
struct internal_state {
    _opaque: [u8; 0],
}