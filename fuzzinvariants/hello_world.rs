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
extern "C" fn bind_debug_parameters(p_stmt_1: *mut sqlite3_stmt) -> () {
    let n_var: i32 = unsafe { sqlite3_bind_parameter_count(p_stmt_1) };
    let mut i: i32 = 0;
    {
        i = 1;
        '__b0: loop {
            if !(i <= n_var) { break '__b0; }
            '__c0: loop {
                let z_var: *const i8 =
                    unsafe { sqlite3_bind_parameter_name(p_stmt_1, i) };
                if z_var == core::ptr::null() { break '__c0; }
                if unsafe {
                            strncmp(z_var, c"$int_".as_ptr() as *mut i8 as *const i8,
                                5 as u64)
                        } == 0 {
                    unsafe {
                        sqlite3_bind_int(p_stmt_1, i,
                            unsafe { atoi(unsafe { &*z_var.offset(5 as isize) }) })
                    };
                } else if unsafe {
                            strncmp(z_var, c"$text_".as_ptr() as *mut i8 as *const i8,
                                6 as u64)
                        } == 0 {
                    let sz_var: u64 = unsafe { strlen(z_var) };
                    let z_buf: *mut i8 =
                        unsafe { sqlite3_malloc64(sz_var - 5 as u64) } as *mut i8;
                    if !(z_buf).is_null() {
                        unsafe {
                            memcpy(z_buf as *mut (),
                                unsafe { &raw const *z_var.offset(6 as isize) } as
                                    *const (), sz_var - 5 as u64)
                        };
                        unsafe {
                            sqlite3_bind_text64(p_stmt_1, i, z_buf as *const i8,
                                sz_var - 6 as u64, Some(sqlite3_free), 1 as u8)
                        };
                    }
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn fuzz_invariant_sql(p_stmt: *mut sqlite3_stmt, mut i_cnt: i32)
    -> *mut i8 {
    let mut z_in: *const i8 = core::ptr::null();
    let mut n_in: u64 = 0 as u64;
    let mut z_and: *const i8 = c"WHERE".as_ptr() as *mut i8 as *const i8;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p_test: *mut sqlite3_str = core::ptr::null_mut();
    let mut p_base: *mut sqlite3_stmt = core::ptr::null_mut();
    let db: *mut sqlite3 = unsafe { sqlite3_db_handle(p_stmt) };
    let mut rc: i32 = 0;
    let n_col: i32 = unsafe { sqlite3_column_count(p_stmt) };
    let mut mx_cnt: i32 = 0;
    let mut b_distinct: i32 = 0;
    let mut b_order_by: i32 = 0;
    let n_param: i32 = unsafe { sqlite3_bind_parameter_count(p_stmt) };
    let mut has_group_by: i32 = 0;
    '__s1:
        {
        match i_cnt % 4 {
            1 => { b_distinct = 1; }
            2 => { b_order_by = 1; }
            3 => { b_distinct = { b_order_by = 1; b_order_by }; }
            _ => {}
        }
    }
    i_cnt /= 4;
    mx_cnt = n_col;
    if i_cnt < 0 || i_cnt > mx_cnt { return core::ptr::null_mut(); }
    z_in = unsafe { sqlite3_sql(p_stmt) };
    if z_in == core::ptr::null() { return core::ptr::null_mut(); }
    n_in = unsafe { strlen(z_in) };
    while n_in > 0 as u64 &&
            (unsafe {
                        isspace(unsafe { *z_in.add((n_in - 1 as u64) as usize) } as
                                i32)
                    } != 0 ||
                unsafe { *z_in.add((n_in - 1 as u64) as usize) } as i32 ==
                    ';' as i32) {
        { let __p = &mut n_in; let __t = *__p; *__p -= 1; __t };
    }
    if !(unsafe { strchr(z_in, '?' as i32) }).is_null() {
        return core::ptr::null_mut();
    }
    p_test = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
    unsafe {
        sqlite3_str_appendf(p_test,
            c"SELECT %s* FROM (".as_ptr() as *mut i8 as *const i8,
            if b_distinct != 0 {
                c"DISTINCT ".as_ptr() as *mut i8
            } else { c"".as_ptr() as *mut i8 })
    };
    unsafe { sqlite3_str_append(p_test, z_in, n_in as i32) };
    unsafe {
        sqlite3_str_append(p_test, c")".as_ptr() as *mut i8 as *const i8, 1)
    };
    rc =
        unsafe {
            sqlite3_prepare_v2(db,
                unsafe { sqlite3_str_value(p_test) } as *const i8, -1,
                &mut p_base, core::ptr::null_mut())
        };
    if rc != 0 { unsafe { sqlite3_finalize(p_base) }; p_base = p_stmt; }
    has_group_by =
        (unsafe {
                    sqlite3_strlike(c"%GROUP BY%".as_ptr() as *mut i8 as
                            *const i8, z_in, 0 as u32)
                } == 0) as i32;
    bind_debug_parameters(p_base);
    {
        i = 0;
        '__b3: loop {
            if !(i < unsafe { sqlite3_column_count(p_stmt) }) { break '__b3; }
            '__c3: loop {
                let z_col_name: *const i8 =
                    unsafe { sqlite3_column_name(p_base, i) };
                let z_suffix: *const i8 =
                    if !(z_col_name).is_null() {
                            unsafe { strrchr(z_col_name, ':' as i32) }
                        } else { core::ptr::null_mut() } as *const i8;
                if !(z_suffix).is_null() &&
                            unsafe {
                                    isdigit(unsafe { *z_suffix.offset(1 as isize) } as i32)
                                } != 0 &&
                        (unsafe { *z_suffix.offset(1 as isize) } as i32 > '3' as i32
                            ||
                            unsafe {
                                    isdigit(unsafe { *z_suffix.offset(2 as isize) } as i32)
                                } != 0) {
                    break '__c3;
                }
                {
                    j = 0;
                    '__b4: loop {
                        if !(j < i) { break '__b4; }
                        '__c4: loop {
                            let z_prior: *const i8 =
                                unsafe { sqlite3_column_name(p_base, j) };
                            if unsafe { sqlite3_stricmp(z_prior, z_col_name) } == 0 {
                                break '__b4;
                            }
                            break '__c4;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                if j < i { break '__c3; }
                if i_cnt == 0 { break '__c3; }
                if i_cnt > 1 && i + 2 != i_cnt { break '__c3; }
                if z_col_name == core::ptr::null() { break '__c3; }
                if unsafe { sqlite3_column_type(p_stmt, i) } == 5 {
                    let z_plus: *const i8 =
                        if has_group_by != 0 {
                                c"+".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 } as *const i8;
                    unsafe {
                        sqlite3_str_appendf(p_test,
                            c" %s %s\"%w\" ISNULL".as_ptr() as *mut i8 as *const i8,
                            z_and, z_plus, z_col_name)
                    };
                } else {
                    unsafe {
                        sqlite3_str_appendf(p_test,
                            c" %s \"%w\"=?%d".as_ptr() as *mut i8 as *const i8, z_and,
                            z_col_name, i + 1 + n_param)
                    };
                }
                z_and = c"AND".as_ptr() as *mut i8 as *const i8;
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if p_base != p_stmt { unsafe { sqlite3_finalize(p_base) }; }
    if b_order_by != 0 {
        unsafe {
            sqlite3_str_appendf(p_test,
                c" ORDER BY %d".as_ptr() as *mut i8 as *const i8,
                if i_cnt > 2 { i_cnt - 1 } else { 1 })
        };
    }
    return unsafe { sqlite3_str_finish(p_test) };
}
extern "C" fn same_value(p_s1: *mut sqlite3_stmt, i1: i32,
    p_s2: *mut sqlite3_stmt, i2: i32, p_test_compare: *mut sqlite3_stmt)
    -> i32 {
    let mut x: i32 = 1;
    let t1: i32 = unsafe { sqlite3_column_type(p_s1, i1) };
    let t2: i32 = unsafe { sqlite3_column_type(p_s2, i2) };
    if t1 != t2 {
        if t1 == 1 && t2 == 2 || t1 == 2 && t2 == 1 {} else { return 0; }
    }
    '__s5:
        {
        match unsafe { sqlite3_column_type(p_s1, i1) } {
            1 => {
                {
                    x =
                        (unsafe { sqlite3_column_int64(p_s1, i1) } ==
                                unsafe { sqlite3_column_int64(p_s2, i2) }) as i32;
                    break '__s5;
                }
                {
                    x =
                        (unsafe { sqlite3_column_double(p_s1, i1) } ==
                                unsafe { sqlite3_column_double(p_s2, i2) }) as i32;
                    break '__s5;
                }
                {
                    let e1: i32 =
                        unsafe {
                            sqlite3_value_encoding(unsafe {
                                    sqlite3_column_value(p_s1, i1)
                                })
                        };
                    let e2: i32 =
                        unsafe {
                            sqlite3_value_encoding(unsafe {
                                    sqlite3_column_value(p_s2, i2)
                                })
                        };
                    if e1 != e2 {
                        let z1: *const i8 =
                            unsafe { sqlite3_column_text(p_s1, i1) } as *const i8;
                        let z2: *const i8 =
                            unsafe { sqlite3_column_text(p_s2, i2) } as *const i8;
                        x =
                            (z1 == core::ptr::null() && z2 == core::ptr::null() ||
                                    z1 != core::ptr::null() && z2 != core::ptr::null() &&
                                        unsafe { strcmp(z1, z1) } == 0) as i32;
                        unsafe {
                            printf(c"Encodings differ.  %d on left and %d on right\n".as_ptr()
                                        as *mut i8 as *const i8, e1, e2)
                        };
                        unsafe { abort() };
                    }
                    if !(p_test_compare).is_null() {
                        unsafe {
                            sqlite3_bind_value(p_test_compare, 1,
                                unsafe { sqlite3_column_value(p_s1, i1) } as
                                    *const sqlite3_value)
                        };
                        unsafe {
                            sqlite3_bind_value(p_test_compare, 2,
                                unsafe { sqlite3_column_value(p_s2, i2) } as
                                    *const sqlite3_value)
                        };
                        x =
                            (unsafe { sqlite3_step(p_test_compare) } == 100 &&
                                    unsafe { sqlite3_column_int(p_test_compare, 0) } != 0) as
                                i32;
                        unsafe { sqlite3_reset(p_test_compare) };
                        break '__s5;
                    }
                    if e1 != 1 {
                        let len1: i32 = unsafe { sqlite3_column_bytes16(p_s1, i1) };
                        let b1: *const u8 =
                            unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                        let len2: i32 = unsafe { sqlite3_column_bytes16(p_s2, i2) };
                        let b2: *const u8 =
                            unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                        if len1 != len2 {
                            x = 0;
                        } else if len1 == 0 {
                            x = 1;
                        } else {
                            x =
                                (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                        unsafe {
                                                memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                            } == 0) as i32;
                        }
                        break '__s5;
                    }
                }
                {
                    let len1: i32 = unsafe { sqlite3_column_bytes(p_s1, i1) };
                    let b1: *const u8 =
                        unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                    let len2: i32 = unsafe { sqlite3_column_bytes(p_s2, i2) };
                    let b2: *const u8 =
                        unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                    if len1 != len2 {
                        x = 0;
                    } else if len1 == 0 {
                        x = 1;
                    } else {
                        x =
                            (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                    unsafe {
                                            memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                        } == 0) as i32;
                    }
                    break '__s5;
                }
            }
            2 => {
                {
                    x =
                        (unsafe { sqlite3_column_double(p_s1, i1) } ==
                                unsafe { sqlite3_column_double(p_s2, i2) }) as i32;
                    break '__s5;
                }
                {
                    let e1: i32 =
                        unsafe {
                            sqlite3_value_encoding(unsafe {
                                    sqlite3_column_value(p_s1, i1)
                                })
                        };
                    let e2: i32 =
                        unsafe {
                            sqlite3_value_encoding(unsafe {
                                    sqlite3_column_value(p_s2, i2)
                                })
                        };
                    if e1 != e2 {
                        let z1: *const i8 =
                            unsafe { sqlite3_column_text(p_s1, i1) } as *const i8;
                        let z2: *const i8 =
                            unsafe { sqlite3_column_text(p_s2, i2) } as *const i8;
                        x =
                            (z1 == core::ptr::null() && z2 == core::ptr::null() ||
                                    z1 != core::ptr::null() && z2 != core::ptr::null() &&
                                        unsafe { strcmp(z1, z1) } == 0) as i32;
                        unsafe {
                            printf(c"Encodings differ.  %d on left and %d on right\n".as_ptr()
                                        as *mut i8 as *const i8, e1, e2)
                        };
                        unsafe { abort() };
                    }
                    if !(p_test_compare).is_null() {
                        unsafe {
                            sqlite3_bind_value(p_test_compare, 1,
                                unsafe { sqlite3_column_value(p_s1, i1) } as
                                    *const sqlite3_value)
                        };
                        unsafe {
                            sqlite3_bind_value(p_test_compare, 2,
                                unsafe { sqlite3_column_value(p_s2, i2) } as
                                    *const sqlite3_value)
                        };
                        x =
                            (unsafe { sqlite3_step(p_test_compare) } == 100 &&
                                    unsafe { sqlite3_column_int(p_test_compare, 0) } != 0) as
                                i32;
                        unsafe { sqlite3_reset(p_test_compare) };
                        break '__s5;
                    }
                    if e1 != 1 {
                        let len1: i32 = unsafe { sqlite3_column_bytes16(p_s1, i1) };
                        let b1: *const u8 =
                            unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                        let len2: i32 = unsafe { sqlite3_column_bytes16(p_s2, i2) };
                        let b2: *const u8 =
                            unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                        if len1 != len2 {
                            x = 0;
                        } else if len1 == 0 {
                            x = 1;
                        } else {
                            x =
                                (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                        unsafe {
                                                memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                            } == 0) as i32;
                        }
                        break '__s5;
                    }
                }
                {
                    let len1: i32 = unsafe { sqlite3_column_bytes(p_s1, i1) };
                    let b1: *const u8 =
                        unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                    let len2: i32 = unsafe { sqlite3_column_bytes(p_s2, i2) };
                    let b2: *const u8 =
                        unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                    if len1 != len2 {
                        x = 0;
                    } else if len1 == 0 {
                        x = 1;
                    } else {
                        x =
                            (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                    unsafe {
                                            memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                        } == 0) as i32;
                    }
                    break '__s5;
                }
            }
            3 => {
                {
                    let e1: i32 =
                        unsafe {
                            sqlite3_value_encoding(unsafe {
                                    sqlite3_column_value(p_s1, i1)
                                })
                        };
                    let e2: i32 =
                        unsafe {
                            sqlite3_value_encoding(unsafe {
                                    sqlite3_column_value(p_s2, i2)
                                })
                        };
                    if e1 != e2 {
                        let z1: *const i8 =
                            unsafe { sqlite3_column_text(p_s1, i1) } as *const i8;
                        let z2: *const i8 =
                            unsafe { sqlite3_column_text(p_s2, i2) } as *const i8;
                        x =
                            (z1 == core::ptr::null() && z2 == core::ptr::null() ||
                                    z1 != core::ptr::null() && z2 != core::ptr::null() &&
                                        unsafe { strcmp(z1, z1) } == 0) as i32;
                        unsafe {
                            printf(c"Encodings differ.  %d on left and %d on right\n".as_ptr()
                                        as *mut i8 as *const i8, e1, e2)
                        };
                        unsafe { abort() };
                    }
                    if !(p_test_compare).is_null() {
                        unsafe {
                            sqlite3_bind_value(p_test_compare, 1,
                                unsafe { sqlite3_column_value(p_s1, i1) } as
                                    *const sqlite3_value)
                        };
                        unsafe {
                            sqlite3_bind_value(p_test_compare, 2,
                                unsafe { sqlite3_column_value(p_s2, i2) } as
                                    *const sqlite3_value)
                        };
                        x =
                            (unsafe { sqlite3_step(p_test_compare) } == 100 &&
                                    unsafe { sqlite3_column_int(p_test_compare, 0) } != 0) as
                                i32;
                        unsafe { sqlite3_reset(p_test_compare) };
                        break '__s5;
                    }
                    if e1 != 1 {
                        let len1: i32 = unsafe { sqlite3_column_bytes16(p_s1, i1) };
                        let b1: *const u8 =
                            unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                        let len2: i32 = unsafe { sqlite3_column_bytes16(p_s2, i2) };
                        let b2: *const u8 =
                            unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                        if len1 != len2 {
                            x = 0;
                        } else if len1 == 0 {
                            x = 1;
                        } else {
                            x =
                                (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                        unsafe {
                                                memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                            } == 0) as i32;
                        }
                        break '__s5;
                    }
                }
                {
                    let len1: i32 = unsafe { sqlite3_column_bytes(p_s1, i1) };
                    let b1: *const u8 =
                        unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                    let len2: i32 = unsafe { sqlite3_column_bytes(p_s2, i2) };
                    let b2: *const u8 =
                        unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                    if len1 != len2 {
                        x = 0;
                    } else if len1 == 0 {
                        x = 1;
                    } else {
                        x =
                            (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                    unsafe {
                                            memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                        } == 0) as i32;
                    }
                    break '__s5;
                }
            }
            4 => {
                {
                    let len1: i32 = unsafe { sqlite3_column_bytes(p_s1, i1) };
                    let b1: *const u8 =
                        unsafe { sqlite3_column_blob(p_s1, i1) } as *const u8;
                    let len2: i32 = unsafe { sqlite3_column_bytes(p_s2, i2) };
                    let b2: *const u8 =
                        unsafe { sqlite3_column_blob(p_s2, i2) } as *const u8;
                    if len1 != len2 {
                        x = 0;
                    } else if len1 == 0 {
                        x = 1;
                    } else {
                        x =
                            (b1 != core::ptr::null() && b2 != core::ptr::null() &&
                                    unsafe {
                                            memcmp(b1 as *const (), b2 as *const (), len1 as u64)
                                        } == 0) as i32;
                    }
                    break '__s5;
                }
            }
            _ => {}
        }
    }
    return x;
}
extern "C" fn print_hex(a: &[u8], mx: i32) -> () {
    let mut j: i32 = 0;
    {
        j = 0;
        '__b6: loop {
            if !(j < mx && j < a.len() as i32) { break '__b6; }
            '__c6: loop {
                unsafe {
                    printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                        a[j as usize] as i32)
                };
                break '__c6;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    if j < a.len() as i32 {
        unsafe { printf(c"...".as_ptr() as *mut i8 as *const i8) };
    }
}
extern "C" fn print_row(p_stmt_1: *mut sqlite3_stmt, i_row_1: i32) -> () {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut n_col: i32 = 0;
    let mut data: *const u8 = core::ptr::null();
    n_col = unsafe { sqlite3_column_count(p_stmt_1) };
    {
        i = 0;
        '__b7: loop {
            if !(i < n_col) { break '__b7; }
            '__c7: loop {
                unsafe {
                    printf(c"row%d.col%d = ".as_ptr() as *mut i8 as *const i8,
                        i_row_1, i)
                };
                '__s8:
                    {
                    match unsafe { sqlite3_column_type(p_stmt_1, i) } {
                        5 => {
                            {
                                unsafe {
                                    printf(c"NULL\n".as_ptr() as *mut i8 as *const i8)
                                };
                                break '__s8;
                            }
                            {
                                unsafe {
                                    printf(c"(integer) %lld\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_int64(p_stmt_1, i) })
                                };
                                break '__s8;
                            }
                            {
                                unsafe {
                                    printf(c"(float) %f\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_double(p_stmt_1, i) })
                                };
                                break '__s8;
                            }
                            {
                                '__s9:
                                    {
                                    match unsafe {
                                            sqlite3_value_encoding(unsafe {
                                                    sqlite3_column_value(p_stmt_1, i)
                                                })
                                        } {
                                        1 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf8) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        3 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        2 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        _ => {
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                    }
                                }
                                break '__s8;
                            }
                            {
                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                data =
                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                unsafe {
                                    printf(c"(blob %d bytes) x\'".as_ptr() as *mut i8 as
                                            *const i8, n)
                                };
                                print_hex(unsafe {
                                        let __p = data as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                    }, 35);
                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                break '__s8;
                            }
                        }
                        1 => {
                            {
                                unsafe {
                                    printf(c"(integer) %lld\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_int64(p_stmt_1, i) })
                                };
                                break '__s8;
                            }
                            {
                                unsafe {
                                    printf(c"(float) %f\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_double(p_stmt_1, i) })
                                };
                                break '__s8;
                            }
                            {
                                '__s9:
                                    {
                                    match unsafe {
                                            sqlite3_value_encoding(unsafe {
                                                    sqlite3_column_value(p_stmt_1, i)
                                                })
                                        } {
                                        1 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf8) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        3 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        2 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        _ => {
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                    }
                                }
                                break '__s8;
                            }
                            {
                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                data =
                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                unsafe {
                                    printf(c"(blob %d bytes) x\'".as_ptr() as *mut i8 as
                                            *const i8, n)
                                };
                                print_hex(unsafe {
                                        let __p = data as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                    }, 35);
                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                break '__s8;
                            }
                        }
                        2 => {
                            {
                                unsafe {
                                    printf(c"(float) %f\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_double(p_stmt_1, i) })
                                };
                                break '__s8;
                            }
                            {
                                '__s9:
                                    {
                                    match unsafe {
                                            sqlite3_value_encoding(unsafe {
                                                    sqlite3_column_value(p_stmt_1, i)
                                                })
                                        } {
                                        1 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf8) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        3 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        2 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        _ => {
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                    }
                                }
                                break '__s8;
                            }
                            {
                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                data =
                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                unsafe {
                                    printf(c"(blob %d bytes) x\'".as_ptr() as *mut i8 as
                                            *const i8, n)
                                };
                                print_hex(unsafe {
                                        let __p = data as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                    }, 35);
                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                break '__s8;
                            }
                        }
                        3 => {
                            {
                                '__s9:
                                    {
                                    match unsafe {
                                            sqlite3_value_encoding(unsafe {
                                                    sqlite3_column_value(p_stmt_1, i)
                                                })
                                        } {
                                        1 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf8) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        3 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16be) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        2 => {
                                            {
                                                unsafe {
                                                    printf(c"(utf16le) x\'".as_ptr() as *mut i8 as *const i8)
                                                };
                                                n = unsafe { sqlite3_column_bytes16(p_stmt_1, i) };
                                                data =
                                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                                print_hex(unsafe {
                                                        let __p = data as *const u8;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                                    }, 35);
                                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                                break '__s9;
                                            }
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                        _ => {
                                            {
                                                unsafe {
                                                    printf(c"Illegal return from sqlite3_value_encoding(): %d\n".as_ptr()
                                                                as *mut i8 as *const i8,
                                                        unsafe {
                                                            sqlite3_value_encoding(unsafe {
                                                                    sqlite3_column_value(p_stmt_1, i)
                                                                })
                                                        })
                                                };
                                                unsafe { abort() };
                                            }
                                        }
                                    }
                                }
                                break '__s8;
                            }
                            {
                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                data =
                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                unsafe {
                                    printf(c"(blob %d bytes) x\'".as_ptr() as *mut i8 as
                                            *const i8, n)
                                };
                                print_hex(unsafe {
                                        let __p = data as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                    }, 35);
                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                break '__s8;
                            }
                        }
                        4 => {
                            {
                                n = unsafe { sqlite3_column_bytes(p_stmt_1, i) };
                                data =
                                    unsafe { sqlite3_column_blob(p_stmt_1, i) } as *const u8;
                                unsafe {
                                    printf(c"(blob %d bytes) x\'".as_ptr() as *mut i8 as
                                            *const i8, n)
                                };
                                print_hex(unsafe {
                                        let __p = data as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, n as usize) }
                                    }, 35);
                                unsafe { printf(c"\'\n".as_ptr() as *mut i8 as *const i8) };
                                break '__s8;
                            }
                        }
                        _ => {}
                    }
                }
                break '__c7;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn report_invariant_failed(p_orig: *mut sqlite3_stmt,
    p_test: *mut sqlite3_stmt, i_row: i32, db_opt: u32, no_opt: i32) -> () {
    let mut i_test_row: i32 = 0;
    unsafe {
        printf(c"Invariant check failed on row %d.\n".as_ptr() as *mut i8 as
                *const i8, i_row)
    };
    unsafe {
        printf(c"Original query (opt-flags: 0x%08x) --------------------------\n".as_ptr()
                    as *mut i8 as *const i8, db_opt)
    };
    unsafe {
        printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
            unsafe { sqlite3_expanded_sql(p_orig) })
    };
    unsafe {
        printf(c"Alternative query (opt-flags: 0x%08x) -----------------------\n".as_ptr()
                    as *mut i8 as *const i8,
            if no_opt != 0 { !db_opt } else { db_opt })
    };
    unsafe {
        printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
            unsafe { sqlite3_expanded_sql(p_test) })
    };
    unsafe {
        printf(c"Result row that is missing from the alternative -----------------\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    print_row(p_orig, i_row);
    unsafe {
        printf(c"Complete results from the alternative query ---------------------\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe { sqlite3_reset(p_test) };
    while unsafe { sqlite3_step(p_test) } == 100 {
        { let __p = &mut i_test_row; let __t = *__p; *__p += 1; __t };
        print_row(p_test, i_test_row);
    }
    unsafe { sqlite3_finalize(p_test) };
    unsafe { abort() };
}
#[unsafe(no_mangle)]
pub extern "C" fn fuzz_invariant(db: *mut sqlite3,
    p_stmt_1: *mut sqlite3_stmt, i_cnt_1: i32, i_row_1: i32, n_row_1: i32,
    pb_corrupt_1: &mut i32, e_verbosity_1: i32, db_opt_1: u32) -> i32 {
    let mut z_test: *mut i8 = core::ptr::null_mut();
    let mut p_test_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut n_col: i32 = 0;
    let mut n_param: i32 = 0;
    let mut no_opt: i32 = 0;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p_ck: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut i_orig_rso: i32 = 0;
    let mut z_sql_1: *mut i8 = core::ptr::null_mut();
    let mut z_sql_2: *mut i8 = core::ptr::null_mut();
    let mut z_sql_3: *mut i8 = core::ptr::null_mut();
    let mut z_sql_4: *mut i8 = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s12:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe { sqlite3_finalize(p_test_stmt) };
                    __state = 123;
                }
                3 => { p_test_stmt = core::ptr::null_mut(); __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { no_opt = (i_cnt_1 % 3 == 0) as i32; __state = 9; }
                9 => {
                    if *pb_corrupt_1 != 0 {
                        __state = 11;
                    } else { __state = 10; }
                }
                10 => {
                    n_param = unsafe { sqlite3_bind_parameter_count(p_stmt_1) };
                    __state = 12;
                }
                11 => { return 101; }
                12 => {
                    if n_param > 100 { __state = 14; } else { __state = 13; }
                }
                13 => {
                    z_test = fuzz_invariant_sql(p_stmt_1, i_cnt_1);
                    __state = 15;
                }
                14 => { return 101; }
                15 => {
                    if z_test == core::ptr::null_mut() {
                        __state = 17;
                    } else { __state = 16; }
                }
                16 => {
                    if no_opt != 0 { __state = 19; } else { __state = 18; }
                }
                17 => { return 101; }
                18 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db, z_test as *const i8, -1,
                                &mut p_test_stmt, core::ptr::null_mut())
                        };
                    __state = 20;
                }
                19 => {
                    unsafe { sqlite3_test_control(15, db, !db_opt_1) };
                    __state = 18;
                }
                20 => {
                    if no_opt != 0 { __state = 22; } else { __state = 21; }
                }
                21 => { if rc != 0 { __state = 24; } else { __state = 23; } }
                22 => {
                    unsafe { sqlite3_test_control(15, db, db_opt_1) };
                    __state = 21;
                }
                23 => {
                    unsafe { sqlite3_free(z_test as *mut ()) };
                    __state = 29;
                }
                24 => {
                    if e_verbosity_1 != 0 {
                        __state = 26;
                    } else { __state = 25; }
                }
                25 => {
                    unsafe { sqlite3_free(z_test as *mut ()) };
                    __state = 27;
                }
                26 => {
                    unsafe {
                        printf(c"invariant compile failed: %s\n%s\n".as_ptr() as
                                    *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) }, z_test)
                    };
                    __state = 25;
                }
                27 => {
                    unsafe { sqlite3_finalize(p_test_stmt) };
                    __state = 28;
                }
                28 => { return rc; }
                29 => { bind_debug_parameters(p_test_stmt); __state = 30; }
                30 => {
                    n_col = unsafe { sqlite3_column_count(p_stmt_1) };
                    __state = 31;
                }
                31 => { i = 0; __state = 33; }
                32 => {
                    if e_verbosity_1 >= 2 {
                        __state = 40;
                    } else { __state = 39; }
                }
                33 => {
                    if i < n_col { __state = 34; } else { __state = 32; }
                }
                34 => {
                    rc =
                        unsafe {
                            sqlite3_bind_value(p_test_stmt, i + 1 + n_param,
                                unsafe { sqlite3_column_value(p_stmt_1, i) } as
                                    *const sqlite3_value)
                        };
                    __state = 36;
                }
                35 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 33;
                }
                36 => {
                    if rc != 0 && rc != 25 {
                        __state = 37;
                    } else { __state = 35; }
                }
                37 => {
                    unsafe { sqlite3_finalize(p_test_stmt) };
                    __state = 38;
                }
                38 => { return rc; }
                39 => {
                    if { rc = unsafe { sqlite3_step(p_test_stmt) }; rc } == 100
                        {
                        __state = 44;
                    } else { __state = 43; }
                }
                40 => {
                    z_sql = unsafe { sqlite3_expanded_sql(p_test_stmt) };
                    __state = 41;
                }
                41 => {
                    unsafe {
                        printf(c"invariant-sql row=%d #%d:\n%s\n".as_ptr() as
                                    *mut i8 as *const i8, i_row_1, i_cnt_1, z_sql)
                    };
                    __state = 42;
                }
                42 => {
                    unsafe { sqlite3_free(z_sql as *mut ()) };
                    __state = 39;
                }
                43 => {
                    if rc == 101 { __state = 52; } else { __state = 51; }
                }
                44 => { i = 0; __state = 46; }
                45 => {
                    if i >= n_col { __state = 50; } else { __state = 39; }
                }
                46 => {
                    if i < n_col { __state = 47; } else { __state = 45; }
                }
                47 => {
                    if (same_value(p_stmt_1, i, p_test_stmt, i,
                                        core::ptr::null_mut()) == 0) as i32 != 0 {
                        __state = 49;
                    } else { __state = 48; }
                }
                48 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 46;
                }
                49 => { __state = 45; }
                50 => { __state = 43; }
                51 => { __state = 2; }
                52 => { p_ck = core::ptr::null_mut(); __state = 53; }
                53 => { __state = 54; }
                54 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"PRAGMA integrity_check".as_ptr() as *mut i8 as *const i8,
                                -1, &mut p_ck, core::ptr::null_mut())
                        };
                    __state = 55;
                }
                55 => { if rc != 0 { __state = 57; } else { __state = 56; } }
                56 => {
                    if e_verbosity_1 >= 2 {
                        __state = 61;
                    } else { __state = 60; }
                }
                57 => { unsafe { sqlite3_finalize(p_ck) }; __state = 58; }
                58 => {
                    unsafe { sqlite3_finalize(p_test_stmt) };
                    __state = 59;
                }
                59 => { return rc; }
                60 => { rc = unsafe { sqlite3_step(p_ck) }; __state = 64; }
                61 => {
                    z_sql_1 = unsafe { sqlite3_expanded_sql(p_ck) };
                    __state = 62;
                }
                62 => {
                    unsafe {
                        printf(c"invariant-validity-check #1:\n%s\n".as_ptr() as
                                    *mut i8 as *const i8, z_sql_1)
                    };
                    __state = 63;
                }
                63 => {
                    unsafe { sqlite3_free(z_sql_1 as *mut ()) };
                    __state = 60;
                }
                64 => {
                    if rc != 100 ||
                                unsafe { sqlite3_column_text(p_ck, 0) } == core::ptr::null()
                            ||
                            unsafe {
                                    strcmp(unsafe { sqlite3_column_text(p_ck, 0) } as *const i8,
                                        c"ok".as_ptr() as *mut i8 as *const i8)
                                } != 0 {
                        __state = 66;
                    } else { __state = 65; }
                }
                65 => { unsafe { sqlite3_finalize(p_ck) }; __state = 70; }
                66 => { *pb_corrupt_1 = 1; __state = 67; }
                67 => { unsafe { sqlite3_finalize(p_ck) }; __state = 68; }
                68 => {
                    unsafe { sqlite3_finalize(p_test_stmt) };
                    __state = 69;
                }
                69 => { return 11; }
                70 => {
                    unsafe {
                        sqlite3_db_config(db, 1019, -1,
                            &raw mut i_orig_rso as *mut i32)
                    };
                    __state = 71;
                }
                71 => {
                    unsafe {
                        sqlite3_db_config(db, 1019, (i_orig_rso == 0) as i32 as i32,
                            0)
                    };
                    __state = 72;
                }
                72 => {
                    unsafe {
                        sqlite3_prepare_v2(db, unsafe { sqlite3_sql(p_stmt_1) }, -1,
                            &mut p_ck, core::ptr::null_mut())
                    };
                    __state = 73;
                }
                73 => {
                    unsafe { sqlite3_db_config(db, 1019, i_orig_rso, 0) };
                    __state = 74;
                }
                74 => {
                    if e_verbosity_1 >= 2 {
                        __state = 76;
                    } else { __state = 75; }
                }
                75 => { bind_debug_parameters(p_ck); __state = 79; }
                76 => {
                    z_sql_2 = unsafe { sqlite3_expanded_sql(p_ck) };
                    __state = 77;
                }
                77 => {
                    unsafe {
                        printf(c"invariant-validity-check #2:\n%s\n".as_ptr() as
                                    *mut i8 as *const i8, z_sql_2)
                    };
                    __state = 78;
                }
                78 => {
                    unsafe { sqlite3_free(z_sql_2 as *mut ()) };
                    __state = 75;
                }
                79 => {
                    if { rc = unsafe { sqlite3_step(p_ck) }; rc } == 100 {
                        __state = 81;
                    } else { __state = 80; }
                }
                80 => { unsafe { sqlite3_finalize(p_ck) }; __state = 88; }
                81 => { i = 0; __state = 83; }
                82 => {
                    if i >= n_col { __state = 87; } else { __state = 79; }
                }
                83 => {
                    if i < n_col { __state = 84; } else { __state = 82; }
                }
                84 => {
                    if (same_value(p_stmt_1, i, p_test_stmt, i,
                                        core::ptr::null_mut()) == 0) as i32 != 0 {
                        __state = 86;
                    } else { __state = 85; }
                }
                85 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 83;
                }
                86 => { __state = 82; }
                87 => { __state = 80; }
                88 => {
                    if rc == 101 { __state = 90; } else { __state = 89; }
                }
                89 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT ?1=?2 OR ?1=?2 COLLATE nocase OR ?1=?2 COLLATE rtrim".as_ptr()
                                        as *mut i8 as *const i8, -1, &mut p_ck,
                                core::ptr::null_mut())
                        };
                    __state = 92;
                }
                90 => {
                    unsafe { sqlite3_finalize(p_test_stmt) };
                    __state = 91;
                }
                91 => { return 101; }
                92 => { if rc == 0 { __state = 94; } else { __state = 93; } }
                93 => { unsafe { sqlite3_finalize(p_ck) }; __state = 109; }
                94 => {
                    if e_verbosity_1 >= 2 {
                        __state = 96;
                    } else { __state = 95; }
                }
                95 => { unsafe { sqlite3_reset(p_test_stmt) }; __state = 99; }
                96 => {
                    z_sql_3 = unsafe { sqlite3_expanded_sql(p_ck) };
                    __state = 97;
                }
                97 => {
                    unsafe {
                        printf(c"invariant-validity-check #3:\n%s\n".as_ptr() as
                                    *mut i8 as *const i8, z_sql_3)
                    };
                    __state = 98;
                }
                98 => {
                    unsafe { sqlite3_free(z_sql_3 as *mut ()) };
                    __state = 95;
                }
                99 => { bind_debug_parameters(p_ck); __state = 100; }
                100 => {
                    if { rc = unsafe { sqlite3_step(p_test_stmt) }; rc } == 100
                        {
                        __state = 101;
                    } else { __state = 93; }
                }
                101 => { i = 0; __state = 103; }
                102 => {
                    if i >= n_col { __state = 107; } else { __state = 100; }
                }
                103 => {
                    if i < n_col { __state = 104; } else { __state = 102; }
                }
                104 => {
                    if (same_value(p_stmt_1, i, p_test_stmt, i, p_ck) == 0) as
                                i32 != 0 {
                        __state = 106;
                    } else { __state = 105; }
                }
                105 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 103;
                }
                106 => { __state = 102; }
                107 => { unsafe { sqlite3_finalize(p_ck) }; __state = 108; }
                108 => { __state = 2; }
                109 => {
                    rc =
                        unsafe {
                            sqlite3_prepare_v2(db,
                                c"SELECT 1 FROM bytecode(?1) WHERE opcode=\'VOpen\' OR      (opcode=\'Explain\' AND p4 GLOB \'SCALAR SUBQUERY*\')".as_ptr()
                                        as *mut i8 as *const i8, -1, &mut p_ck,
                                core::ptr::null_mut())
                        };
                    __state = 110;
                }
                110 => {
                    if rc == 0 { __state = 112; } else { __state = 111; }
                }
                111 => { unsafe { sqlite3_finalize(p_ck) }; __state = 118; }
                112 => {
                    if e_verbosity_1 >= 2 {
                        __state = 114;
                    } else { __state = 113; }
                }
                113 => {
                    unsafe {
                        sqlite3_bind_pointer(p_ck, 1, p_stmt_1 as *mut (),
                            c"stmt-pointer".as_ptr() as *mut i8 as *const i8, None)
                    };
                    __state = 117;
                }
                114 => {
                    z_sql_4 = unsafe { sqlite3_expanded_sql(p_ck) };
                    __state = 115;
                }
                115 => {
                    unsafe {
                        printf(c"invariant-validity-check #4:\n%s\n".as_ptr() as
                                    *mut i8 as *const i8, z_sql_4)
                    };
                    __state = 116;
                }
                116 => {
                    unsafe { sqlite3_free(z_sql_4 as *mut ()) };
                    __state = 113;
                }
                117 => { rc = unsafe { sqlite3_step(p_ck) }; __state = 111; }
                118 => {
                    if rc == 101 { __state = 119; } else { __state = 120; }
                }
                119 => {
                    report_invariant_failed(p_stmt_1, p_test_stmt, i_row_1,
                        db_opt_1, no_opt);
                    __state = 121;
                }
                120 => {
                    if e_verbosity_1 > 0 {
                        __state = 122;
                    } else { __state = 51; }
                }
                121 => { return 2; }
                122 => {
                    unsafe {
                        printf(c"invariant-error ignored due to the use of virtual tables\n".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    __state = 51;
                }
                123 => { return 0; }
                _ => {}
            }
        }
    }
    unreachable!();
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
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn atoi(_: *const i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn isspace(_c: i32)
    -> i32;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn strrchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn isdigit(_c: i32)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn abort()
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn __builtin_unreachable()
    -> ();
}