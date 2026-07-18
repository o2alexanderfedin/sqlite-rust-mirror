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
static ai_class: [u8; 256] =
    [27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8,
            27 as u8, 27 as u8, 7 as u8, 7 as u8, 27 as u8, 7 as u8, 7 as u8,
            27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8,
            27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8,
            27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8,
            7 as u8, 15 as u8, 8 as u8, 5 as u8, 4 as u8, 22 as u8, 24 as u8,
            8 as u8, 17 as u8, 18 as u8, 21 as u8, 20 as u8, 23 as u8,
            11 as u8, 26 as u8, 16 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8,
            3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 3 as u8, 5 as u8,
            19 as u8, 12 as u8, 14 as u8, 13 as u8, 6 as u8, 5 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 0 as u8, 1 as u8, 1 as u8, 9 as u8, 27 as u8, 27 as u8,
            27 as u8, 1 as u8, 8 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8,
            1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0 as u8, 1 as u8,
            1 as u8, 27 as u8, 10 as u8, 27 as u8, 25 as u8, 27 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8];
static sqlite3_upper_to_lower: [u8; 256] =
    [0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8,
            8 as u8, 9 as u8, 10 as u8, 11 as u8, 12 as u8, 13 as u8,
            14 as u8, 15 as u8, 16 as u8, 17 as u8, 18 as u8, 19 as u8,
            20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
            26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8,
            32 as u8, 33 as u8, 34 as u8, 35 as u8, 36 as u8, 37 as u8,
            38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
            44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8,
            50 as u8, 51 as u8, 52 as u8, 53 as u8, 54 as u8, 55 as u8,
            56 as u8, 57 as u8, 58 as u8, 59 as u8, 60 as u8, 61 as u8,
            62 as u8, 63 as u8, 64 as u8, 97 as u8, 98 as u8, 99 as u8,
            100 as u8, 101 as u8, 102 as u8, 103 as u8, 104 as u8, 105 as u8,
            106 as u8, 107 as u8, 108 as u8, 109 as u8, 110 as u8, 111 as u8,
            112 as u8, 113 as u8, 114 as u8, 115 as u8, 116 as u8, 117 as u8,
            118 as u8, 119 as u8, 120 as u8, 121 as u8, 122 as u8, 91 as u8,
            92 as u8, 93 as u8, 94 as u8, 95 as u8, 96 as u8, 97 as u8,
            98 as u8, 99 as u8, 100 as u8, 101 as u8, 102 as u8, 103 as u8,
            104 as u8, 105 as u8, 106 as u8, 107 as u8, 108 as u8, 109 as u8,
            110 as u8, 111 as u8, 112 as u8, 113 as u8, 114 as u8, 115 as u8,
            116 as u8, 117 as u8, 118 as u8, 119 as u8, 120 as u8, 121 as u8,
            122 as u8, 123 as u8, 124 as u8, 125 as u8, 126 as u8, 127 as u8,
            128 as u8, 129 as u8, 130 as u8, 131 as u8, 132 as u8, 133 as u8,
            134 as u8, 135 as u8, 136 as u8, 137 as u8, 138 as u8, 139 as u8,
            140 as u8, 141 as u8, 142 as u8, 143 as u8, 144 as u8, 145 as u8,
            146 as u8, 147 as u8, 148 as u8, 149 as u8, 150 as u8, 151 as u8,
            152 as u8, 153 as u8, 154 as u8, 155 as u8, 156 as u8, 157 as u8,
            158 as u8, 159 as u8, 160 as u8, 161 as u8, 162 as u8, 163 as u8,
            164 as u8, 165 as u8, 166 as u8, 167 as u8, 168 as u8, 169 as u8,
            170 as u8, 171 as u8, 172 as u8, 173 as u8, 174 as u8, 175 as u8,
            176 as u8, 177 as u8, 178 as u8, 179 as u8, 180 as u8, 181 as u8,
            182 as u8, 183 as u8, 184 as u8, 185 as u8, 186 as u8, 187 as u8,
            188 as u8, 189 as u8, 190 as u8, 191 as u8, 192 as u8, 193 as u8,
            194 as u8, 195 as u8, 196 as u8, 197 as u8, 198 as u8, 199 as u8,
            200 as u8, 201 as u8, 202 as u8, 203 as u8, 204 as u8, 205 as u8,
            206 as u8, 207 as u8, 208 as u8, 209 as u8, 210 as u8, 211 as u8,
            212 as u8, 213 as u8, 214 as u8, 215 as u8, 216 as u8, 217 as u8,
            218 as u8, 219 as u8, 220 as u8, 221 as u8, 222 as u8, 223 as u8,
            224 as u8, 225 as u8, 226 as u8, 227 as u8, 228 as u8, 229 as u8,
            230 as u8, 231 as u8, 232 as u8, 233 as u8, 234 as u8, 235 as u8,
            236 as u8, 237 as u8, 238 as u8, 239 as u8, 240 as u8, 241 as u8,
            242 as u8, 243 as u8, 244 as u8, 245 as u8, 246 as u8, 247 as u8,
            248 as u8, 249 as u8, 250 as u8, 251 as u8, 252 as u8, 253 as u8,
            254 as u8, 255 as u8];
static sqlite3_ctype_map: [u8; 256] =
    [0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 1 as u8, 0 as u8, 128 as u8, 0 as u8,
            64 as u8, 0 as u8, 0 as u8, 128 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 10 as u8, 10 as u8, 10 as u8, 10 as u8,
            10 as u8, 10 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 128 as u8, 0 as u8, 0 as u8, 0 as u8, 64 as u8,
            128 as u8, 42 as u8, 42 as u8, 42 as u8, 42 as u8, 42 as u8,
            42 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8,
            34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8,
            34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8,
            34 as u8, 34 as u8, 34 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8];
extern "C" fn sqlite3_get_token(z: *const u8, token_type_1: &mut i32)
    -> sqlite3_int64 {
    let mut i: sqlite3_int64 = 0 as sqlite3_int64;
    let mut c: i32 = 0;
    '__s0:
        {
        match ai_class[unsafe { *z } as usize] {
            7 => {
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b1: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 1 != 0) {
                                break '__b1;
                            }
                            '__c1: loop { break '__c1; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = 0;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                        {
                            i = 2 as sqlite3_int64;
                            '__b2: loop {
                                if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                && c != '\n' as i32) {
                                    break '__b2;
                                }
                                '__c2: loop { break '__c2; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        *token_type_1 = 0;
                        return i;
                    }
                    *token_type_1 = 3;
                    return 1 as sqlite3_int64;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            11 => {
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                        {
                            i = 2 as sqlite3_int64;
                            '__b2: loop {
                                if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                && c != '\n' as i32) {
                                    break '__b2;
                                }
                                '__c2: loop { break '__c2; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        *token_type_1 = 0;
                        return i;
                    }
                    *token_type_1 = 3;
                    return 1 as sqlite3_int64;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            17 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            18 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            19 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            20 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            21 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            16 => {
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '*' as i32 ||
                            unsafe { *z.offset(2 as isize) } as i32 == 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                    {
                        {
                            ({ i = 3 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(2 as isize) } as i32
                        };
                        '__b3: loop {
                            if !((c != '*' as i32 ||
                                                unsafe { *z.offset(i as isize) } as i32 != '/' as i32) &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    *token_type_1 = 0;
                    return i;
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            22 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            14 => {
                {
                    *token_type_1 = 3;
                    return (1 +
                                (unsafe { *z.offset(1 as isize) } as i32 == '=' as i32) as
                                    i32) as sqlite3_int64;
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            12 => {
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '<' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            13 => {
                {
                    if { c = unsafe { *z.offset(1 as isize) } as i32; c } ==
                            '=' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else if c == '>' as i32 {
                        *token_type_1 = 3;
                        return 2 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 1 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            15 => {
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '=' as i32 {
                        *token_type_1 = 4;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            10 => {
                {
                    if unsafe { *z.offset(1 as isize) } as i32 != '|' as i32 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    } else { *token_type_1 = 3; return 2 as sqlite3_int64; }
                }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            23 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            24 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            25 => {
                { *token_type_1 = 3; return 1 as sqlite3_int64; }
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            8 => {
                {
                    let delim: i32 = unsafe { *z.offset(0 as isize) } as i32;
                    {
                        i = 1 as sqlite3_int64;
                        '__b4: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                if c == delim {
                                    if unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) }
                                                as i32 == delim {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { break '__b4; }
                                }
                                break '__c4;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if c == '\'' as i32 {
                        *token_type_1 = 2;
                        return i + 1 as sqlite3_int64;
                    } else if c != 0 {
                        *token_type_1 = 1;
                        return i + 1 as sqlite3_int64;
                    } else { *token_type_1 = 4; return i; }
                }
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            26 => {
                {
                    if (sqlite3_ctype_map[unsafe { *z.offset(1 as isize) } as u8
                                                    as usize] as i32 & 4 == 0) as i32 != 0 {
                        *token_type_1 = 3;
                        return 1 as sqlite3_int64;
                    }
                }
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            3 => {
                {
                    *token_type_1 = 2;
                    if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) &&
                            sqlite3_ctype_map[unsafe { *z.offset(2 as isize) } as u8 as
                                                usize] as i32 & 8 != 0 {
                        {
                            i = 3 as sqlite3_int64;
                            '__b5: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b5;
                                }
                                '__c5: loop { break '__c5; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        return i;
                    }
                    {
                        i = 0 as sqlite3_int64;
                        '__b6: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b6;
                            }
                            '__c6: loop { break '__c6; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '.' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    if (unsafe { *z.offset(i as isize) } as i32 == 'e' as i32 ||
                                unsafe { *z.offset(i as isize) } as i32 == 'E' as i32) &&
                            (sqlite3_ctype_map[unsafe {
                                                            *z.offset((i + 1 as sqlite3_int64) as isize)
                                                        } as u8 as usize] as i32 & 4 != 0 ||
                                (unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '+' as i32 ||
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == '-' as i32) &&
                                    sqlite3_ctype_map[unsafe {
                                                                *z.offset((i + 2 as sqlite3_int64) as isize)
                                                            } as u8 as usize] as i32 & 4 != 0) {
                        i += 2 as sqlite3_int64;
                        while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                    u8 as usize] as i32 & 4 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        *token_type_1 = 2;
                    }
                    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                u8 as usize] as i32 & 70 != 0 {
                        *token_type_1 = 4;
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    return i;
                }
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            9 => {
                {
                    {
                        {
                            ({ i = 1 as sqlite3_int64; i }) as i32;
                            c = unsafe { *z.offset(0 as isize) } as i32
                        };
                        '__b10: loop {
                            if !(c != ']' as i32 &&
                                            { c = unsafe { *z.offset(i as isize) } as i32; c } != 0) {
                                break '__b10;
                            }
                            '__c10: loop { break '__c10; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    *token_type_1 = if c == ']' as i32 { 1 } else { 4 };
                    return i;
                }
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            6 => {
                {
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b11: loop {
                            if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                u8 as usize] as i32 & 4 != 0) {
                                break '__b11;
                            }
                            '__c11: loop { break '__c11; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return i;
                }
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            4 => {
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            5 => {
                {
                    let mut n: i32 = 0;
                    *token_type_1 = 2;
                    {
                        i = 1 as sqlite3_int64;
                        '__b12: loop {
                            if !({ c = unsafe { *z.offset(i as isize) } as i32; c } !=
                                            0) {
                                break '__b12;
                            }
                            '__c12: loop {
                                if sqlite3_ctype_map[c as u8 as usize] as i32 & 70 != 0 {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                } else if c == '(' as i32 && n > 0 {
                                    '__b13: loop {
                                        '__c13: loop {
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            break '__c13;
                                        }
                                        if !({ c = unsafe { *z.offset(i as isize) } as i32; c } != 0
                                                            &&
                                                            (sqlite3_ctype_map[c as u8 as usize] as i32 & 1 == 0) as i32
                                                                != 0 && c != ')' as i32) {
                                            break '__b13;
                                        }
                                    }
                                    if c == ')' as i32 {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    } else { *token_type_1 = 4; }
                                    break '__b12;
                                } else if c == ':' as i32 &&
                                        unsafe { *z.offset((i + 1 as sqlite3_int64) as isize) } as
                                                i32 == ':' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else { break '__b12; }
                                break '__c12;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == 0 { *token_type_1 = 4; }
                    return i;
                }
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            1 => {
                {
                    {
                        i = 1 as sqlite3_int64;
                        '__b14: loop {
                            if !(ai_class[unsafe { *z.offset(i as isize) } as usize] as
                                                i32 <= 1) {
                                break '__b14;
                            }
                            '__c14: loop { break '__c14; }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        break '__s0;
                    }
                    *token_type_1 = 1;
                    return i;
                }
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            0 => {
                {
                    if unsafe { *z.offset(1 as isize) } as i32 == '\'' as i32 {
                        *token_type_1 = 2;
                        {
                            i = 2 as sqlite3_int64;
                            '__b15: loop {
                                if !(sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as
                                                                    u8 as usize] as i32 & 8 != 0) {
                                    break '__b15;
                                }
                                '__c15: loop { break '__c15; }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 ||
                                i % 2 as sqlite3_int64 != 0 {
                            *token_type_1 = 4;
                            while unsafe { *z.offset(i as isize) } != 0 &&
                                    unsafe { *z.offset(i as isize) } as i32 != '\'' as i32 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if unsafe { *z.offset(i as isize) } != 0 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        return i;
                    }
                }
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            2 => {
                { i = 1 as sqlite3_int64; break '__s0; }
                { *token_type_1 = 4; return 1 as sqlite3_int64; }
            }
            _ => { { *token_type_1 = 4; return 1 as sqlite3_int64; } }
        }
    }
    while sqlite3_ctype_map[unsafe { *z.offset(i as isize) } as u8 as usize]
                    as i32 & 70 != 0 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    *token_type_1 = 1;
    return i;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_normalize(z_sql_1: *const i8) -> *mut i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut n_z: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_sql: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut token_type: i32 = 0;
    let mut n: sqlite3_int64 = 0 as sqlite3_int64;
    let mut k: i32 = 0;
    n_sql = unsafe { strlen(z_sql_1) } as sqlite3_int64;
    n_z = n_sql;
    z =
        unsafe {
                sqlite3_malloc64((n_z + 2 as sqlite3_int64) as sqlite3_uint64)
            } as *mut i8;
    if z == core::ptr::null_mut() { return core::ptr::null_mut(); }
    {
        i = { j = 0; j };
        '__b18: loop {
            if !(unsafe { *z_sql_1.offset(i as isize) } != 0) {
                break '__b18;
            }
            '__c18: loop {
                n =
                    sqlite3_get_token(unsafe {
                                (z_sql_1 as *mut u8).offset(i as isize)
                            } as *const u8, &mut token_type);
                '__s19:
                    {
                    match token_type {
                        0 => {
                            { break '__s19; }
                            {
                                unsafe { sqlite3_free(z as *mut ()) };
                                return core::ptr::null_mut();
                            }
                            {
                                unsafe {
                                    *z.offset({
                                                        let __p = &mut j;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = '?' as i32 as i8
                                };
                                break '__s19;
                            }
                            {
                                if n == 4 as i64 &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { z_sql_1.offset(i as isize) },
                                                    c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                                            } == 0 {
                                    if j >= 3 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(2 as isize)) }
                                                                    } as *const i8, c"is".as_ptr() as *mut i8 as *const i8,
                                                                2 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 3) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0 ||
                                            j >= 4 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(3 as isize)) }
                                                                    } as *const i8, c"not".as_ptr() as *mut i8 as *const i8,
                                                                3 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 4) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0
                                        {} else {
                                        unsafe {
                                            *z.offset({
                                                                let __p = &mut j;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize) = '?' as i32 as i8
                                        };
                                        break '__s19;
                                    }
                                }
                                if j > 0 &&
                                            sqlite3_ctype_map[unsafe { *z.offset((j - 1) as isize) } as
                                                                    u8 as usize] as i32 & 70 != 0 &&
                                        sqlite3_ctype_map[unsafe { *z_sql_1.offset(i as isize) } as
                                                                u8 as usize] as i32 & 70 != 0 {
                                    unsafe {
                                        *z.offset({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize) = ' ' as i32 as i8
                                    };
                                }
                                {
                                    k = 0;
                                    '__b20: loop {
                                        if !((k as sqlite3_int64) < n) { break '__b20; }
                                        '__c20: loop {
                                            unsafe {
                                                *z.offset({
                                                                    let __p = &mut j;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize) =
                                                    sqlite3_upper_to_lower[unsafe {
                                                                        *z_sql_1.offset((i + k) as isize)
                                                                    } as u8 as usize] as i8
                                            };
                                            break '__c20;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__s19;
                            }
                        }
                        4 => {
                            {
                                unsafe { sqlite3_free(z as *mut ()) };
                                return core::ptr::null_mut();
                            }
                            {
                                unsafe {
                                    *z.offset({
                                                        let __p = &mut j;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = '?' as i32 as i8
                                };
                                break '__s19;
                            }
                            {
                                if n == 4 as i64 &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { z_sql_1.offset(i as isize) },
                                                    c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                                            } == 0 {
                                    if j >= 3 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(2 as isize)) }
                                                                    } as *const i8, c"is".as_ptr() as *mut i8 as *const i8,
                                                                2 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 3) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0 ||
                                            j >= 4 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(3 as isize)) }
                                                                    } as *const i8, c"not".as_ptr() as *mut i8 as *const i8,
                                                                3 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 4) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0
                                        {} else {
                                        unsafe {
                                            *z.offset({
                                                                let __p = &mut j;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize) = '?' as i32 as i8
                                        };
                                        break '__s19;
                                    }
                                }
                                if j > 0 &&
                                            sqlite3_ctype_map[unsafe { *z.offset((j - 1) as isize) } as
                                                                    u8 as usize] as i32 & 70 != 0 &&
                                        sqlite3_ctype_map[unsafe { *z_sql_1.offset(i as isize) } as
                                                                u8 as usize] as i32 & 70 != 0 {
                                    unsafe {
                                        *z.offset({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize) = ' ' as i32 as i8
                                    };
                                }
                                {
                                    k = 0;
                                    '__b20: loop {
                                        if !((k as sqlite3_int64) < n) { break '__b20; }
                                        '__c20: loop {
                                            unsafe {
                                                *z.offset({
                                                                    let __p = &mut j;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize) =
                                                    sqlite3_upper_to_lower[unsafe {
                                                                        *z_sql_1.offset((i + k) as isize)
                                                                    } as u8 as usize] as i8
                                            };
                                            break '__c20;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__s19;
                            }
                        }
                        2 => {
                            {
                                unsafe {
                                    *z.offset({
                                                        let __p = &mut j;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = '?' as i32 as i8
                                };
                                break '__s19;
                            }
                            {
                                if n == 4 as i64 &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { z_sql_1.offset(i as isize) },
                                                    c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                                            } == 0 {
                                    if j >= 3 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(2 as isize)) }
                                                                    } as *const i8, c"is".as_ptr() as *mut i8 as *const i8,
                                                                2 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 3) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0 ||
                                            j >= 4 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(3 as isize)) }
                                                                    } as *const i8, c"not".as_ptr() as *mut i8 as *const i8,
                                                                3 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 4) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0
                                        {} else {
                                        unsafe {
                                            *z.offset({
                                                                let __p = &mut j;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize) = '?' as i32 as i8
                                        };
                                        break '__s19;
                                    }
                                }
                                if j > 0 &&
                                            sqlite3_ctype_map[unsafe { *z.offset((j - 1) as isize) } as
                                                                    u8 as usize] as i32 & 70 != 0 &&
                                        sqlite3_ctype_map[unsafe { *z_sql_1.offset(i as isize) } as
                                                                u8 as usize] as i32 & 70 != 0 {
                                    unsafe {
                                        *z.offset({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize) = ' ' as i32 as i8
                                    };
                                }
                                {
                                    k = 0;
                                    '__b20: loop {
                                        if !((k as sqlite3_int64) < n) { break '__b20; }
                                        '__c20: loop {
                                            unsafe {
                                                *z.offset({
                                                                    let __p = &mut j;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize) =
                                                    sqlite3_upper_to_lower[unsafe {
                                                                        *z_sql_1.offset((i + k) as isize)
                                                                    } as u8 as usize] as i8
                                            };
                                            break '__c20;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__s19;
                            }
                        }
                        3 => {
                            {
                                if n == 4 as i64 &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { z_sql_1.offset(i as isize) },
                                                    c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                                            } == 0 {
                                    if j >= 3 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(2 as isize)) }
                                                                    } as *const i8, c"is".as_ptr() as *mut i8 as *const i8,
                                                                2 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 3) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0 ||
                                            j >= 4 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(3 as isize)) }
                                                                    } as *const i8, c"not".as_ptr() as *mut i8 as *const i8,
                                                                3 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 4) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0
                                        {} else {
                                        unsafe {
                                            *z.offset({
                                                                let __p = &mut j;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize) = '?' as i32 as i8
                                        };
                                        break '__s19;
                                    }
                                }
                                if j > 0 &&
                                            sqlite3_ctype_map[unsafe { *z.offset((j - 1) as isize) } as
                                                                    u8 as usize] as i32 & 70 != 0 &&
                                        sqlite3_ctype_map[unsafe { *z_sql_1.offset(i as isize) } as
                                                                u8 as usize] as i32 & 70 != 0 {
                                    unsafe {
                                        *z.offset({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize) = ' ' as i32 as i8
                                    };
                                }
                                {
                                    k = 0;
                                    '__b20: loop {
                                        if !((k as sqlite3_int64) < n) { break '__b20; }
                                        '__c20: loop {
                                            unsafe {
                                                *z.offset({
                                                                    let __p = &mut j;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize) =
                                                    sqlite3_upper_to_lower[unsafe {
                                                                        *z_sql_1.offset((i + k) as isize)
                                                                    } as u8 as usize] as i8
                                            };
                                            break '__c20;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__s19;
                            }
                        }
                        1 => {
                            {
                                if n == 4 as i64 &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { z_sql_1.offset(i as isize) },
                                                    c"NULL".as_ptr() as *mut i8 as *const i8, 4)
                                            } == 0 {
                                    if j >= 3 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(2 as isize)) }
                                                                    } as *const i8, c"is".as_ptr() as *mut i8 as *const i8,
                                                                2 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 3) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0 ||
                                            j >= 4 &&
                                                    unsafe {
                                                            strncmp(unsafe {
                                                                        unsafe { z.offset(j as isize).offset(-(3 as isize)) }
                                                                    } as *const i8, c"not".as_ptr() as *mut i8 as *const i8,
                                                                3 as u64)
                                                        } == 0 &&
                                                !(sqlite3_ctype_map[unsafe { *z.offset((j - 4) as isize) }
                                                                                    as u8 as usize] as i32 & 70 != 0) as i32 != 0
                                        {} else {
                                        unsafe {
                                            *z.offset({
                                                                let __p = &mut j;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize) = '?' as i32 as i8
                                        };
                                        break '__s19;
                                    }
                                }
                                if j > 0 &&
                                            sqlite3_ctype_map[unsafe { *z.offset((j - 1) as isize) } as
                                                                    u8 as usize] as i32 & 70 != 0 &&
                                        sqlite3_ctype_map[unsafe { *z_sql_1.offset(i as isize) } as
                                                                u8 as usize] as i32 & 70 != 0 {
                                    unsafe {
                                        *z.offset({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize) = ' ' as i32 as i8
                                    };
                                }
                                {
                                    k = 0;
                                    '__b20: loop {
                                        if !((k as sqlite3_int64) < n) { break '__b20; }
                                        '__c20: loop {
                                            unsafe {
                                                *z.offset({
                                                                    let __p = &mut j;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize) =
                                                    sqlite3_upper_to_lower[unsafe {
                                                                        *z_sql_1.offset((i + k) as isize)
                                                                    } as u8 as usize] as i8
                                            };
                                            break '__c20;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__s19;
                            }
                        }
                        _ => {}
                    }
                }
                break '__c18;
            }
            i += n as i32;
        }
    }
    while j > 0 && unsafe { *z.offset((j - 1) as isize) } as i32 == ' ' as i32
        {
        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
    }
    if j > 0 && unsafe { *z.offset((j - 1) as isize) } as i32 != ';' as i32 {
        unsafe {
            *z.offset({ let __p = &mut j; let __t = *__p; *__p += 1; __t } as
                            isize) = ';' as i32 as i8
        };
    }
    unsafe { *z.offset(j as isize) = 0 as i8 };
    {
        i = 0;
        '__b22: loop {
            if !(i < j) { break '__b22; }
            '__c22: loop {
                let z_in: *const i8 =
                    unsafe {
                            strstr(unsafe { z.offset(i as isize) } as *const i8,
                                c"in(".as_ptr() as *mut i8 as *const i8)
                        } as *const i8;
                let mut n_paren: i32 = 0;
                if z_in == core::ptr::null_mut() { break '__b22; }
                n =
                    (unsafe { z_in.offset_from(z) } as i64 as i32 + 3) as
                        sqlite3_int64;
                if n != 0 &&
                        sqlite3_ctype_map[unsafe { *z_in.offset(-1 as isize) } as u8
                                            as usize] as i32 & 70 != 0 {
                    break '__c22;
                }
                if unsafe {
                                strncmp(z_in as *const i8,
                                    c"in(select".as_ptr() as *mut i8 as *const i8, 9 as u64)
                            } == 0 &&
                        !(sqlite3_ctype_map[unsafe { *z_in.offset(9 as isize) } as
                                                            u8 as usize] as i32 & 70 != 0) as i32 != 0 {
                    break '__c22;
                }
                if unsafe {
                                strncmp(z_in as *const i8,
                                    c"in(with".as_ptr() as *mut i8 as *const i8, 7 as u64)
                            } == 0 &&
                        !(sqlite3_ctype_map[unsafe { *z_in.offset(7 as isize) } as
                                                            u8 as usize] as i32 & 70 != 0) as i32 != 0 {
                    break '__c22;
                }
                {
                    { n_paren = 1; k = 0 };
                    '__b23: loop {
                        if !(unsafe { *z.offset((n + k as sqlite3_int64) as isize) }
                                        != 0) {
                            break '__b23;
                        }
                        '__c23: loop {
                            if unsafe { *z.offset((n + k as sqlite3_int64) as isize) }
                                        as i32 == '(' as i32 {
                                { let __p = &mut n_paren; let __t = *__p; *__p += 1; __t };
                            }
                            if unsafe { *z.offset((n + k as sqlite3_int64) as isize) }
                                        as i32 == ')' as i32 {
                                { let __p = &mut n_paren; let __t = *__p; *__p -= 1; __t };
                                if n_paren == 0 { break '__b23; }
                            }
                            break '__c23;
                        }
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    }
                }
                if k < 5 {
                    z =
                        unsafe {
                                sqlite3_realloc64(z as *mut (),
                                    (j + (5 - k) + 1) as sqlite3_uint64)
                            } as *mut i8;
                    if z == core::ptr::null_mut() {
                        return core::ptr::null_mut();
                    }
                    unsafe {
                        memmove(unsafe {
                                    unsafe { z.offset(n as isize).offset(5 as isize) }
                                } as *mut (),
                            unsafe {
                                    unsafe { z.offset(n as isize).offset(k as isize) }
                                } as *const (),
                            (j as sqlite3_int64 - (n + k as sqlite3_int64)) as u64)
                    };
                } else if k > 5 {
                    unsafe {
                        memmove(unsafe {
                                    unsafe { z.offset(n as isize).offset(5 as isize) }
                                } as *mut (),
                            unsafe {
                                    unsafe { z.offset(n as isize).offset(k as isize) }
                                } as *const (),
                            (j as sqlite3_int64 - (n + k as sqlite3_int64)) as u64)
                    };
                }
                j = j - k + 5;
                unsafe { *z.offset(j as isize) = 0 as i8 };
                unsafe {
                    memcpy(unsafe { z.offset(n as isize) } as *mut (),
                        c"?,?,?".as_ptr() as *mut i8 as *const (), 5 as u64)
                };
                break '__c22;
            }
            i = n as i32;
        }
    }
    return z;
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
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
}