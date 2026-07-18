#![feature(c_variadic)]
type __darwin_pthread_t = *mut _opaque_pthread_t;
type pthread_t = __darwin_pthread_t;
type __darwin_size_t = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct _opaque_pthread_mutex_t {
    __sig: i64,
    __opaque: [i8; 56],
}
type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
type pthread_mutex_t = __darwin_pthread_mutex_t;
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
    _opaque: [u8; 0],
}
type sqlite3_filename = *const i8;
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
type sqlite3_syscall_ptr = unsafe extern "C" fn() -> ();
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
struct sqlite3_context {
    _opaque: [u8; 0],
}
type sqlite3_destructor_type = unsafe extern "C" fn(*mut ()) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vtab {
    p_module: *const sqlite3_module,
    n_ref: i32,
    z_err_msg: *mut i8,
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
struct sqlite3_str {
    _opaque: [u8; 0],
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
struct sqlite3_backup {
    _opaque: [u8; 0],
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
#[repr(C)]
#[derive(Copy, Clone)]
struct WorkerInfo {
    tid: i32,
    n_worker: i32,
    wkr_flags: u32,
    main_db: *mut sqlite3,
    db: *mut sqlite3,
    n_err: i32,
    n_test: i32,
    z_msg: *mut i8,
    id: pthread_t,
    p_wr_mutex: *mut pthread_mutex_t,
}
extern "C" fn check_oom(x: *mut ()) -> () {
    if x == core::ptr::null_mut() {
        eprintln!("out of memory");
        unsafe { exit(1) };
    }
}
extern "C" fn safe_malloc(sz: i32) -> *mut () {
    let x: *mut () = unsafe { sqlite3_malloc(if sz > 0 { sz } else { 1 }) };
    check_oom(x);
    return x;
}
unsafe extern "C" fn worker_trace(p: &WorkerInfo, z_format_1: *const i8,
    mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_msg: *mut i8 = core::ptr::null_mut();
        if (*p).wkr_flags & 4 as u32 == 0 as u32 { return; }
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_msg = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        check_oom(z_msg as *mut ());
        ();
        unsafe {
            fprintf(__stderrp,
                c"TRACE(%02d): %s\n".as_ptr() as *mut i8 as *const i8,
                (*p).tid, z_msg)
        };
        unsafe { sqlite3_free(z_msg as *mut ()) };
    }
}
unsafe extern "C" fn prep_sql(db: *mut sqlite3, z_format_1: *const i8,
    mut __va0: ...) -> *mut sqlite3_stmt {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        check_oom(z_sql as *mut ());
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"SQL error (%d,%d): %s\nWhile preparing: [%s]\n".as_ptr()
                            as *mut i8 as *const i8, rc,
                    unsafe { sqlite3_extended_errcode(db) },
                    unsafe { sqlite3_errmsg(db) }, z_sql)
            };
            unsafe { exit(1) };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        return p_stmt;
    }
}
unsafe extern "C" fn run_sql(p: *mut WorkerInfo, z_format_1: *const i8,
    mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut n_retry: i32 = 0;
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        check_oom(z_sql as *mut ());
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8, -1,
                    &mut p_stmt, core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"SQL error (%d,%d): %s\nWhile preparing: [%s]\n".as_ptr()
                            as *mut i8 as *const i8, rc,
                    unsafe { sqlite3_extended_errcode(unsafe { (*p).db }) },
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) }, z_sql)
            };
            unsafe { exit(1) };
        }
        unsafe {
            worker_trace(unsafe { &*p },
                c"running [%s]".as_ptr() as *mut i8 as *const i8, z_sql)
        };
        while { rc = unsafe { sqlite3_step(p_stmt) }; rc } != 101 {
            if rc & 255 == 5 || rc & 255 == 6 {
                unsafe { sqlite3_reset(p_stmt) };
                { let __p = &mut n_retry; let __t = *__p; *__p += 1; __t };
                if n_retry < 10 {
                    unsafe {
                        worker_trace(unsafe { &*p },
                            c"retry %d for [%s]".as_ptr() as *mut i8 as *const i8,
                            n_retry, z_sql)
                    };
                    unsafe { sched_yield() };
                    continue;
                } else {
                    unsafe {
                        fprintf(__stderrp,
                            c"Deadlock in thread %d while running [%s]\n".as_ptr() as
                                    *mut i8 as *const i8, unsafe { (*p).tid }, z_sql)
                    };
                    unsafe { exit(1) };
                }
            }
            if rc != 100 {
                unsafe {
                    fprintf(__stderrp,
                        c"SQL error (%d,%d): %s\nWhile running [%s]\n".as_ptr() as
                                *mut i8 as *const i8, rc,
                        unsafe { sqlite3_extended_errcode(unsafe { (*p).db }) },
                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) }, z_sql)
                };
                unsafe { exit(1) };
            }
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        unsafe { sqlite3_finalize(p_stmt) };
    }
}
extern "C" fn worker_open_connection(p: *mut WorkerInfo, i_cnt_1: i32) -> () {
    unsafe {
        let mut z_file: *mut i8 = core::ptr::null_mut();
        let mut x: i32 = 0;
        let mut rc: i32 = 0;
        x = (unsafe { (*p).tid } + i_cnt_1) % 6;
        z_file =
            unsafe {
                sqlite3_mprintf(c"tt4-test%d.db".as_ptr() as *mut i8 as
                        *const i8, a_order[x as usize][0 as usize] as i32)
            };
        check_oom(z_file as *mut ());
        unsafe {
            worker_trace(unsafe { &*p },
                c"open %s".as_ptr() as *mut i8 as *const i8, z_file)
        };
        rc =
            unsafe {
                sqlite3_open_v2(z_file as *const i8, unsafe { &mut (*p).db },
                    2 | 131072, core::ptr::null())
            };
        if rc != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"sqlite_open_v2(%s) failed on thread %d\n".as_ptr() as
                            *mut i8 as *const i8, z_file, unsafe { (*p).tid })
            };
            unsafe { exit(1) };
        }
        unsafe { sqlite3_free(z_file as *mut ()) };
        unsafe {
            run_sql(p,
                c"PRAGMA read_uncommitted=ON;".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe { sqlite3_busy_timeout(unsafe { (*p).db }, 10000) };
        unsafe {
            run_sql(p,
                c"PRAGMA synchronous=OFF;".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            run_sql(p,
                c"ATTACH \'tt4-test%d.db\' AS aux1".as_ptr() as *mut i8 as
                    *const i8, a_order[x as usize][1 as usize] as i32)
        };
        unsafe {
            run_sql(p,
                c"ATTACH \'tt4-test%d.db\' AS aux2".as_ptr() as *mut i8 as
                    *const i8, a_order[x as usize][2 as usize] as i32)
        };
    }
}
extern "C" fn worker_close_connection(p: *mut WorkerInfo) -> () {
    if !(unsafe { (*p).db }).is_null() {
        unsafe {
            worker_trace(unsafe { &*p },
                c"close".as_ptr() as *mut i8 as *const i8)
        };
        unsafe { sqlite3_close(unsafe { (*p).db }) };
        unsafe { (*p).db = core::ptr::null_mut() };
    }
}
extern "C" fn worker_delete_all_content(p: *mut WorkerInfo, in_trans_1: i32)
    -> () {
    if in_trans_1 != 0 {
        unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
        unsafe { run_sql(p, c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            run_sql(p,
                c"DELETE FROM t1 WHERE tid=%d".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).tid })
        };
        unsafe {
            run_sql(p,
                c"DELETE FROM t2 WHERE tid=%d".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).tid })
        };
        unsafe {
            run_sql(p,
                c"DELETE FROM t3 WHERE tid=%d".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).tid })
        };
        unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8 as *const i8) };
        unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
        {
            let __p = unsafe { &mut (*p).n_test };
            let __t = *__p;
            *__p += 1;
            __t
        };
    } else {
        unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
        unsafe {
            run_sql(p,
                c"DELETE FROM t1 WHERE tid=%d".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).tid })
        };
        unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
        {
            let __p = unsafe { &mut (*p).n_test };
            let __t = *__p;
            *__p += 1;
            __t
        };
        unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
        unsafe {
            run_sql(p,
                c"DELETE FROM t2 WHERE tid=%d".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).tid })
        };
        unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
        {
            let __p = unsafe { &mut (*p).n_test };
            let __t = *__p;
            *__p += 1;
            __t
        };
        unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
        unsafe {
            run_sql(p,
                c"DELETE FROM t3 WHERE tid=%d".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).tid })
        };
        unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
        {
            let __p = unsafe { &mut (*p).n_test };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}
extern "C" fn worker_add_content(p: *mut WorkerInfo, mn: i32, mx: i32,
    i_tab_1: i32) -> () {
    let mut z_tab_def: *mut i8 = core::ptr::null_mut();
    '__s1:
        {
        match i_tab_1 {
            1 => { z_tab_def = c"t1(tid,sp,a,b,c)".as_ptr() as *mut i8; }
            2 => { z_tab_def = c"t2(tid,sp,d,e,f)".as_ptr() as *mut i8; }
            3 => { z_tab_def = c"t3(tid,sp,x,y,z)".as_ptr() as *mut i8; }
            _ => {}
        }
    }
    unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
    unsafe {
        run_sql(p,
            c"WITH RECURSIVE\n c(i) AS (VALUES(%d) UNION ALL SELECT i+1 FROM c WHERE i<%d)\nINSERT INTO %s SELECT %d, zeroblob(3000), i, printf(\'%%d\',i), i FROM c;".as_ptr()
                    as *mut i8 as *const i8, mn, mx, z_tab_def,
            unsafe { (*p).tid })
    };
    unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
    { let __p = unsafe { &mut (*p).n_test }; let __t = *__p; *__p += 1; __t };
}
unsafe extern "C" fn worker_error(p: &mut WorkerInfo, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    { let __p = &mut (*p).n_err; let __t = *__p; *__p += 1; __t };
    unsafe { sqlite3_free((*p).z_msg as *mut ()) };
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    (*p).z_msg = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
}
extern "C" fn worker_thread(p_arg_1: *mut ()) -> *mut () {
    unsafe {
        let p: *mut WorkerInfo = p_arg_1 as *mut WorkerInfo;
        let mut i_outer: i32 = 0;
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        unsafe {
            printf(c"worker %d startup\n".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p).tid })
        };
        unsafe { fflush(__stdoutp) };
        {
            i_outer = 1;
            '__b2: loop {
                if !(i_outer <= unsafe { (*p).n_worker }) { break '__b2; }
                '__c2: loop {
                    worker_open_connection(p, i_outer);
                    {
                        i = 0;
                        '__b3: loop {
                            if !(i < 4) { break '__b3; }
                            '__c3: loop {
                                worker_add_content(p, i * 100 + 1, (i + 1) * 100,
                                    (unsafe { (*p).tid } + i_outer) % 3 + 1);
                                worker_add_content(p, i * 100 + 1, (i + 1) * 100,
                                    (unsafe { (*p).tid } + i_outer + 1) % 3 + 1);
                                worker_add_content(p, i * 100 + 1, (i + 1) * 100,
                                    (unsafe { (*p).tid } + i_outer + 2) % 3 + 1);
                                break '__c3;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    p_stmt =
                        unsafe {
                            prep_sql(unsafe { (*p).db },
                                c"SELECT count(a) FROM t1 WHERE tid=%d".as_ptr() as *mut i8
                                    as *const i8, unsafe { (*p).tid })
                        };
                    unsafe {
                        worker_trace(unsafe { &*p },
                            c"query [%s]".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_sql(p_stmt) })
                    };
                    rc = unsafe { sqlite3_step(p_stmt) };
                    if rc != 100 {
                        unsafe {
                            worker_error(unsafe { &mut *p },
                                c"Failed to step: %s".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_sql(p_stmt) })
                        };
                    } else if unsafe { sqlite3_column_int(p_stmt, 0) } != 400 {
                        unsafe {
                            worker_error(unsafe { &mut *p },
                                c"Wrong result: %d".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_column_int(p_stmt, 0) })
                        };
                    }
                    unsafe { sqlite3_finalize(p_stmt) };
                    if unsafe { (*p).n_err } != 0 { break '__b2; }
                    if (i_outer + unsafe { (*p).tid }) % 3 == 0 {
                        unsafe { sqlite3_db_release_memory(unsafe { (*p).db }) };
                        {
                            let __p = unsafe { &mut (*p).n_test };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                    unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
                    unsafe {
                        run_sql(p, c"BEGIN;".as_ptr() as *mut i8 as *const i8)
                    };
                    unsafe {
                        run_sql(p,
                            c"UPDATE t1 SET c=NULL WHERE a=55".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    unsafe {
                        run_sql(p,
                            c"UPDATE t2 SET f=NULL WHERE d=42".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    unsafe {
                        run_sql(p,
                            c"UPDATE t3 SET z=NULL WHERE x=31".as_ptr() as *mut i8 as
                                *const i8)
                    };
                    unsafe {
                        run_sql(p, c"ROLLBACK;".as_ptr() as *mut i8 as *const i8)
                    };
                    {
                        let __p = unsafe { &mut (*p).n_test };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
                    if i_outer == unsafe { (*p).tid } {
                        unsafe { pthread_mutex_lock(unsafe { (*p).p_wr_mutex }) };
                        unsafe {
                            run_sql(p, c"VACUUM".as_ptr() as *mut i8 as *const i8)
                        };
                        unsafe { pthread_mutex_unlock(unsafe { (*p).p_wr_mutex }) };
                    }
                    p_stmt =
                        unsafe {
                            prep_sql(unsafe { (*p).db },
                                c"SELECT t1.rowid, t2.rowid, t3.rowid  FROM t1, t2, t3 WHERE t1.tid=%d AND t2.tid=%d AND t3.tid=%d   AND t1.a<>t2.d AND t2.d<>t3.x ORDER BY 1, 2, 3".as_ptr()
                                        as *mut i8 as *const i8, unsafe { (*p).tid },
                                unsafe { (*p).tid }, unsafe { (*p).tid })
                        };
                    unsafe {
                        worker_trace(unsafe { &*p },
                            c"query [%s]".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_sql(p_stmt) })
                    };
                    {
                        i = 0;
                        '__b4: loop {
                            if !(i < unsafe { (*p).n_worker }) { break '__b4; }
                            '__c4: loop {
                                rc = unsafe { sqlite3_step(p_stmt) };
                                if rc != 100 {
                                    unsafe {
                                        worker_error(unsafe { &mut *p },
                                            c"Failed to step: %s".as_ptr() as *mut i8 as *const i8,
                                            unsafe { sqlite3_sql(p_stmt) })
                                    };
                                    break '__b4;
                                }
                                unsafe { sched_yield() };
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { sqlite3_finalize(p_stmt) };
                    if unsafe { (*p).n_err } != 0 { break '__b2; }
                    worker_delete_all_content(p,
                        (unsafe { (*p).tid } + i_outer) % 2);
                    worker_close_connection(p);
                    unsafe { (*p).db = core::ptr::null_mut() };
                    break '__c2;
                }
                { let __p = &mut i_outer; let __t = *__p; *__p += 1; __t };
            }
        }
        worker_close_connection(p);
        unsafe {
            printf(c"worker %d finished\n".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p).tid })
        };
        unsafe { fflush(__stdoutp) };
        return core::ptr::null_mut();
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut n_worker: i32 = 0;
        let mut i: i32 = 0;
        let mut a_info: *mut WorkerInfo = core::ptr::null_mut();
        let mut wkr_flags: u32 = 0 as u32;
        let mut n_err: i32 = 0;
        let mut n_test: i32 = 0;
        let mut rc: i32 = 0;
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut wr_mutex: pthread_mutex_t = unsafe { core::mem::zeroed() };
        let mut info_top: WorkerInfo = unsafe { core::mem::zeroed() };
        let mut p: *mut WorkerInfo = core::ptr::null_mut();
        unsafe { sqlite3_config(2) };
        {
            i = 1;
            '__b5: loop {
                if !(i < argc) { break '__b5; }
                '__c5: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 &&
                                unsafe { *z.offset(2 as isize) } as i32 != 0 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        if unsafe {
                                    strcmp(z, c"-multithread".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            unsafe { sqlite3_config(2) };
                            wkr_flags &= !1 as u32;
                        } else if unsafe {
                                    strcmp(z, c"-serialized".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            unsafe { sqlite3_config(3) };
                            wkr_flags |= 1 as u32;
                        } else if unsafe {
                                    strcmp(z, c"-wal".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            wkr_flags |= 2 as u32;
                        } else if unsafe {
                                    strcmp(z, c"-trace".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            wkr_flags |= 4 as u32;
                        } else {
                            unsafe {
                                fprintf(__stderrp,
                                    c"unknown command-line option: %s\n".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(i as isize) })
                            };
                            unsafe { exit(1) };
                        }
                    } else if unsafe { *z.offset(0 as isize) } as i32 >=
                                    '1' as i32 &&
                                unsafe { *z.offset(0 as isize) } as i32 <= '9' as i32 &&
                            n_worker == 0 {
                        n_worker = unsafe { atoi(z) };
                        if n_worker < 2 {
                            eprintln!("minimum of 2 threads");
                            unsafe { exit(1) };
                        }
                    } else {
                        unsafe {
                            fprintf(__stderrp,
                                c"extra command-line argument: \"%s\"\n".as_ptr() as *mut i8
                                    as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        unsafe { exit(1) };
                    }
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_worker == 0 {
            unsafe {
                fprintf(__stderrp,
                    c"usage:  %s ?OPTIONS? N\nN is the number of threads and must be at least 2.\nOptions:\n  --serialized\n  --multithread\n  --wal\n  --trace\n".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) })
            };
            unsafe { exit(1) };
        }
        if (unsafe { sqlite3_threadsafe() } == 0) as i32 != 0 {
            eprintln!("requires a threadsafe build of SQLite");
            unsafe { exit(1) };
        }
        unsafe { sqlite3_initialize() };
        unsafe { sqlite3_enable_shared_cache(1) };
        unsafe { pthread_mutex_init(&mut wr_mutex, core::ptr::null()) };
        {
            let _ =
                unsafe {
                    unlink(c"tt4-test1.db".as_ptr() as *mut i8 as *const i8)
                };
        };
        {
            let _ =
                unsafe {
                    unlink(c"tt4-test2.db".as_ptr() as *mut i8 as *const i8)
                };
        };
        {
            let _ =
                unsafe {
                    unlink(c"tt4-test3.db".as_ptr() as *mut i8 as *const i8)
                };
        };
        rc =
            unsafe {
                sqlite3_open(c"tt4-test1.db".as_ptr() as *mut i8 as *const i8,
                    &mut db)
            };
        if rc != 0 {
            eprintln!("Unable to open test database: tt4-test2.db");
            unsafe { exit(1) };
        }
        unsafe {
            memset(&raw mut info_top as *mut (), 0,
                core::mem::size_of::<WorkerInfo>() as u64)
        };
        info_top.db = db;
        info_top.wkr_flags = wkr_flags;
        p = &mut info_top;
        if wkr_flags & 2 as u32 != 0 {
            unsafe {
                run_sql(p,
                    c"PRAGMA journal_mode=WAL".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            run_sql(p,
                c"PRAGMA synchronous=OFF".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE TABLE IF NOT EXISTS t1(tid INTEGER, sp, a, b, c)".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE INDEX t1tid ON t1(tid)".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE INDEX t1ab ON t1(a,b)".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"ATTACH \'tt4-test2.db\' AS \'test2\'".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE TABLE IF NOT EXISTS test2.t2(tid INTEGER, sp, d, e, f)".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE INDEX test2.t2tid ON t2(tid)".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE INDEX test2.t2de ON t2(d,e)".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"ATTACH \'tt4-test3.db\' AS \'test3\'".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE TABLE IF NOT EXISTS test3.t3(tid INTEGER, sp, x, y, z)".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE INDEX test3.t3tid ON t3(tid)".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            run_sql(p,
                c"CREATE INDEX test3.t3xy ON t3(x,y)".as_ptr() as *mut i8 as
                    *const i8)
        };
        a_info =
            safe_malloc((core::mem::size_of::<WorkerInfo>() as u64 *
                            n_worker as u64) as i32) as *mut WorkerInfo;
        unsafe {
            memset(a_info as *mut (), 0,
                core::mem::size_of::<WorkerInfo>() as u64 * n_worker as u64)
        };
        {
            i = 0;
            '__b6: loop {
                if !(i < n_worker) { break '__b6; }
                '__c6: loop {
                    unsafe { (*a_info.offset(i as isize)).tid = i + 1 };
                    unsafe { (*a_info.offset(i as isize)).n_worker = n_worker };
                    unsafe {
                        (*a_info.offset(i as isize)).wkr_flags = wkr_flags
                    };
                    unsafe { (*a_info.offset(i as isize)).main_db = db };
                    unsafe {
                        (*a_info.offset(i as isize)).p_wr_mutex = &mut wr_mutex
                    };
                    rc =
                        unsafe {
                            pthread_create(unsafe {
                                    &mut (*a_info.offset(i as isize)).id
                                }, core::ptr::null(), worker_thread,
                                unsafe { &raw mut *a_info.offset(i as isize) } as *mut ())
                        };
                    if rc != 0 {
                        eprintln!("thread creation failed for thread {}", (i + 1) as i32);
                        unsafe { exit(1) };
                    }
                    unsafe { sched_yield() };
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 0;
            '__b7: loop {
                if !(i < n_worker) { break '__b7; }
                '__c7: loop {
                    unsafe {
                        pthread_join(unsafe { (*a_info.offset(i as isize)).id },
                            core::ptr::null_mut())
                    };
                    unsafe {
                        printf(c"Joined thread %d: %d errors in %d tests".as_ptr()
                                    as *mut i8 as *const i8,
                            unsafe { (*a_info.offset(i as isize)).tid },
                            unsafe { (*a_info.offset(i as isize)).n_err },
                            unsafe { (*a_info.offset(i as isize)).n_test })
                    };
                    if !(unsafe {
                                        (*a_info.offset(i as isize)).z_msg
                                    }).is_null() {
                        unsafe {
                            printf(c": %s\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*a_info.offset(i as isize)).z_msg })
                        };
                    } else {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                    }
                    n_err += unsafe { (*a_info.offset(i as isize)).n_err };
                    n_test += unsafe { (*a_info.offset(i as isize)).n_test };
                    unsafe { fflush(__stdoutp) };
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_close(db) };
        unsafe { sqlite3_free(a_info as *mut ()) };
        unsafe {
            printf(c"Total %d errors in %d tests\n".as_ptr() as *mut i8 as
                    *const i8, n_err, n_test)
        };
        return Err(n_err);
    }
}
static a_order: [[u8; 3]; 6] =
    [[1 as u8, 2 as u8, 3 as u8], [1 as u8, 3 as u8, 2 as u8],
            [2 as u8, 1 as u8, 3 as u8], [2 as u8, 3 as u8, 1 as u8],
            [3 as u8, 1 as u8, 2 as u8], [3 as u8, 2 as u8, 1 as u8]];
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
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
    fn exit(_: i32)
    -> ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn sched_yield()
    -> i32;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t)
    -> i32;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn atoi(_: *const i8)
    -> i32;
    fn pthread_mutex_init(_: *mut pthread_mutex_t,
    _: *const pthread_mutexattr_t)
    -> i32;
    fn unlink(_: *const i8)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn pthread_create(_: *mut pthread_t, _: *const pthread_attr_t,
    _: unsafe extern "C" fn(*mut ()) -> *mut (), _: *mut ())
    -> i32;
    fn pthread_join(_: pthread_t, _: *mut *mut ())
    -> i32;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;
#[repr(C)]
#[derive(Copy, Clone)]
struct _opaque_pthread_t {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct _opaque_pthread_attr_t {
    _opaque: [u8; 0],
}
type pthread_attr_t = _opaque_pthread_attr_t;
#[repr(C)]
#[derive(Copy, Clone)]
struct _opaque_pthread_mutexattr_t {
    _opaque: [u8; 0],
}
type pthread_mutexattr_t = _opaque_pthread_mutexattr_t;