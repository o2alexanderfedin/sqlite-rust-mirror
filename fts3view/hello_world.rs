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
#[unsafe(no_mangle)]
pub static mut n_extra: i32 = unsafe { core::mem::zeroed() };
#[unsafe(no_mangle)]
pub static mut az_extra: *mut *mut i8 = unsafe { core::mem::zeroed() };
#[unsafe(no_mangle)]
pub extern "C" fn find_option(z_name_1: *const i8, has_arg_1: i32,
    z_default_1: *const i8) -> *const i8 {
    unsafe {
        let mut i: i32 = 0;
        let mut z_result: *const i8 = z_default_1;
        {
            i = 0;
            '__b0: loop {
                if !(i < n_extra) { break '__b0; }
                '__c0: loop {
                    let mut z: *const i8 =
                        unsafe { *az_extra.offset(i as isize) } as *const i8;
                    while unsafe { *z.offset(0 as isize) } as i32 == '-' as i32
                        {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe { strcmp(z, z_name_1) } == 0 {
                        let mut j: i32 = 1;
                        if has_arg_1 == 0 || i == n_extra - 1 { j = 0; }
                        z_result =
                            unsafe { *az_extra.offset((i + j) as isize) } as *const i8;
                        while i + j < n_extra {
                            unsafe {
                                *az_extra.offset(i as isize) =
                                    unsafe { *az_extra.offset((i + j + 1) as isize) }
                            };
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        break '__b0;
                    }
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return z_result;
    }
}
unsafe extern "C" fn prepare(db: *mut sqlite3, z_format_1: *const i8,
    mut __va0: ...) -> *mut sqlite3_stmt {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"Error: %s\nSQL: %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(db) }, z_sql)
            };
            unsafe { exit(1) };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        return p_stmt;
    }
}
unsafe extern "C" fn run_sql(db: *mut sqlite3, z_format_1: *const i8,
    mut __va0: ...) -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_sql: *const i8 = core::ptr::null();
    let mut rc: i32 = 0;
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    rc =
        unsafe {
            sqlite3_exec(db, z_sql as *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
    ();
    return rc;
}
extern "C" fn show_schema(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT sql FROM sqlite_schema WHERE name LIKE \'%q%%\' ORDER BY 1".as_ptr()
                        as *mut i8 as *const i8, z_tab_1)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        unsafe {
            printf(c"%s;\n".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_column_text(p_stmt, 0) })
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    p_stmt =
        unsafe {
            prepare(db, c"PRAGMA page_size".as_ptr() as *mut i8 as *const i8)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        unsafe {
            printf(c"PRAGMA page_size=%s;\n".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_column_text(p_stmt, 0) })
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    p_stmt =
        unsafe {
            prepare(db,
                c"PRAGMA journal_mode".as_ptr() as *mut i8 as *const i8)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        unsafe {
            printf(c"PRAGMA journal_mode=%s;\n".as_ptr() as *mut i8 as
                    *const i8, unsafe { sqlite3_column_text(p_stmt, 0) })
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    p_stmt =
        unsafe {
            prepare(db,
                c"PRAGMA auto_vacuum".as_ptr() as *mut i8 as *const i8)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        let mut z_type: *const i8 = c"???".as_ptr() as *mut i8 as *const i8;
        '__s7:
            {
            match unsafe { sqlite3_column_int(p_stmt, 0) } {
                0 => { z_type = c"OFF".as_ptr() as *mut i8 as *const i8; }
                1 => { z_type = c"FULL".as_ptr() as *mut i8 as *const i8; }
                2 => {
                    z_type = c"INCREMENTAL".as_ptr() as *mut i8 as *const i8;
                }
                _ => {}
            }
        }
        unsafe {
            printf(c"PRAGMA auto_vacuum=%s;\n".as_ptr() as *mut i8 as
                    *const i8, z_type)
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    p_stmt =
        unsafe {
            prepare(db, c"PRAGMA encoding".as_ptr() as *mut i8 as *const i8)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        unsafe {
            printf(c"PRAGMA encoding=%s;\n".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_column_text(p_stmt, 0) })
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
}
#[unsafe(no_mangle)]
pub extern "C" fn get_varint(p: *const u8, v: &mut sqlite_int64) -> i32 {
    let mut q: *const u8 = p;
    let mut x: sqlite_uint64 = 0 as sqlite_uint64;
    let mut y: sqlite_uint64 = 1 as sqlite_uint64;
    while unsafe { *q } as i32 & 128 == 128 &&
            (unsafe { q.offset_from(p as *mut u8) } as i64) < 9 as i64 {
        x +=
            y *
                (unsafe {
                                *{
                                        let __p = &mut q;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as i32 & 127) as sqlite_uint64;
        y <<= 7 as sqlite_uint64;
    }
    x +=
        y *
            unsafe {
                    *{
                            let __p = &mut q;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                } as sqlite_uint64;
    *v = x as sqlite_int64;
    return unsafe { q.offset_from(p as *mut u8) } as i64 as i32;
}
extern "C" fn show_stat(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT id, value FROM \'%q_stat\'".as_ptr() as *mut i8 as
                    *const i8, z_tab_1)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        unsafe {
            printf(c"stat[%d] =".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_column_int(p_stmt, 0) })
        };
        '__s11:
            {
            match unsafe { sqlite3_column_type(p_stmt, 1) } {
                1 => {
                    {
                        unsafe {
                            printf(c" %d\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_column_int(p_stmt, 1) })
                        };
                        break '__s11;
                    }
                    {
                        let x: *const u8 =
                            unsafe { sqlite3_column_blob(p_stmt, 1) } as *mut u8 as
                                *const u8;
                        let len: i32 = unsafe { sqlite3_column_bytes(p_stmt, 1) };
                        let mut i: i32 = 0;
                        let mut v: sqlite3_int64 = 0 as sqlite3_int64;
                        while i < len {
                            i += get_varint(x as *const u8, &mut v);
                            unsafe {
                                printf(c" %lld".as_ptr() as *mut i8 as *const i8, v)
                            };
                        }
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        break '__s11;
                    }
                }
                4 => {
                    {
                        let x: *const u8 =
                            unsafe { sqlite3_column_blob(p_stmt, 1) } as *mut u8 as
                                *const u8;
                        let len: i32 = unsafe { sqlite3_column_bytes(p_stmt, 1) };
                        let mut i: i32 = 0;
                        let mut v: sqlite3_int64 = 0 as sqlite3_int64;
                        while i < len {
                            i += get_varint(x as *const u8, &mut v);
                            unsafe {
                                printf(c" %lld".as_ptr() as *mut i8 as *const i8, v)
                            };
                        }
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        break '__s11;
                    }
                }
                _ => {}
            }
        }
    }
    unsafe { sqlite3_finalize(p_stmt) };
}
extern "C" fn show_vocabulary(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    let mut z_aux: *mut i8 = core::ptr::null_mut();
    let mut r: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut n_doc: i32 = 0;
    let mut n_token: i32 = 0;
    let mut n_occurrence: i32 = 0;
    let mut n_top: i32 = 0;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s14:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe {
                        run_sql(db, c"ROLLBACK".as_ptr() as *mut i8 as *const i8)
                    };
                    __state = 64;
                }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { n_doc = 0; __state = 6; }
                6 => { n_token = 0; __state = 7; }
                7 => { n_occurrence = 0; __state = 8; }
                8 => { __state = 9; }
                9 => { __state = 10; }
                10 => {
                    unsafe {
                        sqlite3_randomness(core::mem::size_of::<sqlite3_uint64>() as
                                i32, &raw mut r as *mut ())
                    };
                    __state = 11;
                }
                11 => {
                    z_aux =
                        unsafe {
                            sqlite3_mprintf(c"viewer_%llx".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1, r)
                        };
                    __state = 12;
                }
                12 => {
                    unsafe {
                        run_sql(db, c"BEGIN".as_ptr() as *mut i8 as *const i8)
                    };
                    __state = 13;
                }
                13 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT count(*) FROM %Q".as_ptr() as *mut i8 as *const i8,
                                z_tab_1)
                        };
                    __state = 14;
                }
                14 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 16;
                    } else { __state = 15; }
                }
                15 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 17; }
                16 => {
                    n_doc = unsafe { sqlite3_column_int(p_stmt, 0) };
                    __state = 14;
                }
                17 => {
                    unsafe {
                        printf(c"Number of documents...................... %9d\n".as_ptr()
                                    as *mut i8 as *const i8, n_doc)
                    };
                    __state = 18;
                }
                18 => {
                    unsafe {
                        run_sql(db,
                            c"CREATE VIRTUAL TABLE %s USING fts4aux(%Q)".as_ptr() as
                                    *mut i8 as *const i8, z_aux, z_tab_1)
                    };
                    __state = 19;
                }
                19 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT count(*), sum(occurrences) FROM %s WHERE col=\'*\'".as_ptr()
                                        as *mut i8 as *const i8, z_aux)
                        };
                    __state = 20;
                }
                20 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 24; }
                22 => {
                    n_token = unsafe { sqlite3_column_int(p_stmt, 0) };
                    __state = 23;
                }
                23 => {
                    n_occurrence = unsafe { sqlite3_column_int(p_stmt, 1) };
                    __state = 20;
                }
                24 => {
                    unsafe {
                        printf(c"Total tokens in all documents............ %9d\n".as_ptr()
                                    as *mut i8 as *const i8, n_occurrence)
                    };
                    __state = 25;
                }
                25 => {
                    unsafe {
                        printf(c"Total number of distinct tokens.......... %9d\n".as_ptr()
                                    as *mut i8 as *const i8, n_token)
                    };
                    __state = 26;
                }
                26 => {
                    if n_token == 0 { __state = 28; } else { __state = 27; }
                }
                27 => { n = 0; __state = 29; }
                28 => { __state = 2; }
                29 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT count(*) FROM %s WHERE col=\'*\' AND occurrences==1".as_ptr()
                                        as *mut i8 as *const i8, z_aux)
                        };
                    __state = 30;
                }
                30 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 32;
                    } else { __state = 31; }
                }
                31 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 33; }
                32 => {
                    n = unsafe { sqlite3_column_int(p_stmt, 0) };
                    __state = 30;
                }
                33 => {
                    unsafe {
                        printf(c"Tokens used exactly once................. %9d %5.2f%%\n".as_ptr()
                                    as *mut i8 as *const i8, n,
                            n as f64 * 100.0 / n_token as f64)
                    };
                    __state = 34;
                }
                34 => { n = 0; __state = 35; }
                35 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT count(*) FROM %s WHERE col=\'*\' AND documents==1".as_ptr()
                                        as *mut i8 as *const i8, z_aux)
                        };
                    __state = 36;
                }
                36 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 38;
                    } else { __state = 37; }
                }
                37 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 39; }
                38 => {
                    n = unsafe { sqlite3_column_int(p_stmt, 0) };
                    __state = 36;
                }
                39 => {
                    unsafe {
                        printf(c"Tokens used in only one document......... %9d %5.2f%%\n".as_ptr()
                                    as *mut i8 as *const i8, n,
                            n as f64 * 100.0 / n_token as f64)
                    };
                    __state = 40;
                }
                40 => {
                    if n_doc >= 2000 { __state = 42; } else { __state = 41; }
                }
                41 => {
                    if n_doc >= 200 { __state = 49; } else { __state = 48; }
                }
                42 => { n = 0; __state = 43; }
                43 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT count(*) FROM %s WHERE col=\'*\' AND occurrences<=%d".as_ptr()
                                        as *mut i8 as *const i8, z_aux, n_doc / 1000)
                        };
                    __state = 44;
                }
                44 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 46;
                    } else { __state = 45; }
                }
                45 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 47; }
                46 => {
                    n = unsafe { sqlite3_column_int(p_stmt, 0) };
                    __state = 44;
                }
                47 => {
                    unsafe {
                        printf(c"Tokens used in 0.1%% or less of docs...... %9d %5.2f%%\n".as_ptr()
                                    as *mut i8 as *const i8, n,
                            n as f64 * 100.0 / n_token as f64)
                    };
                    __state = 41;
                }
                48 => {
                    n_top =
                        unsafe {
                            atoi(find_option(c"top".as_ptr() as *mut i8 as *const i8, 1,
                                    c"25".as_ptr() as *mut i8 as *const i8))
                        };
                    __state = 55;
                }
                49 => { n = 0; __state = 50; }
                50 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT count(*) FROM %s WHERE col=\'*\' AND occurrences<=%d".as_ptr()
                                        as *mut i8 as *const i8, z_aux, n_doc / 100)
                        };
                    __state = 51;
                }
                51 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 53;
                    } else { __state = 52; }
                }
                52 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 54; }
                53 => {
                    n = unsafe { sqlite3_column_int(p_stmt, 0) };
                    __state = 51;
                }
                54 => {
                    unsafe {
                        printf(c"Tokens used in 1%% or less of docs........ %9d %5.2f%%\n".as_ptr()
                                    as *mut i8 as *const i8, n,
                            n as f64 * 100.0 / n_token as f64)
                    };
                    __state = 48;
                }
                55 => {
                    unsafe {
                        printf(c"The %d most common tokens:\n".as_ptr() as *mut i8
                                as *const i8, n_top)
                    };
                    __state = 56;
                }
                56 => {
                    p_stmt =
                        unsafe {
                            prepare(db,
                                c"SELECT term, documents FROM %s WHERE col=\'*\' ORDER BY documents DESC, term LIMIT %d".as_ptr()
                                        as *mut i8 as *const i8, z_aux, n_top)
                        };
                    __state = 57;
                }
                57 => { i = 0; __state = 58; }
                58 => {
                    if unsafe { sqlite3_step(p_stmt) } == 100 {
                        __state = 60;
                    } else { __state = 59; }
                }
                59 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 63; }
                60 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 61;
                }
                61 => {
                    n = unsafe { sqlite3_column_int(p_stmt, 1) };
                    __state = 62;
                }
                62 => {
                    unsafe {
                        printf(c"  %2d. %-30s %9d docs %5.2f%%\n".as_ptr() as
                                    *mut i8 as *const i8, i,
                            unsafe { sqlite3_column_text(p_stmt, 0) }, n,
                            n as f64 * 100.0 / n_doc as f64)
                    };
                    __state = 58;
                }
                63 => { __state = 2; }
                64 => {
                    unsafe { sqlite3_free(z_aux as *mut ()) };
                    __state = 1;
                }
                _ => {}
            }
        }
    }
}
extern "C" fn show_segment_stats(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut n_seg: i32 = 0;
    let mut sz_seg: sqlite3_int64 = 0 as sqlite3_int64;
    let mut mx_seg: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_idx: i32 = 0;
    let mut sz_idx: sqlite3_int64 = 0 as sqlite3_int64;
    let mut mx_idx: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_root: i32 = 0;
    let mut sz_root: sqlite3_int64 = 0 as sqlite3_int64;
    let mut mx_root: sqlite3_int64 = 0 as sqlite3_int64;
    let mut mx: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_leaf: i32 = 0;
    let mut n: i32 = 0;
    let mut pgsz: i32 = 0;
    let mut mx_level: i32 = 0;
    let mut i: i32 = 0;
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT count(*), sum(length(block)), max(length(block)) FROM \'%q_segments\'".as_ptr()
                        as *mut i8 as *const i8, z_tab_1)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        n_seg = unsafe { sqlite3_column_int(p_stmt, 0) };
        sz_seg = unsafe { sqlite3_column_int64(p_stmt, 1) };
        mx_seg = unsafe { sqlite3_column_int64(p_stmt, 2) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT count(*), sum(length(block)), max(length(block))  FROM \'%q_segments\' a JOIN \'%q_segdir\' b WHERE a.blockid BETWEEN b.leaves_end_block+1 AND b.end_block".as_ptr()
                        as *mut i8 as *const i8, z_tab_1, z_tab_1)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        n_idx = unsafe { sqlite3_column_int(p_stmt, 0) };
        sz_idx = unsafe { sqlite3_column_int64(p_stmt, 1) };
        mx_idx = unsafe { sqlite3_column_int64(p_stmt, 2) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT count(*), sum(length(root)), max(length(root))  FROM \'%q_segdir\'".as_ptr()
                        as *mut i8 as *const i8, z_tab_1)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        n_root = unsafe { sqlite3_column_int(p_stmt, 0) };
        sz_root = unsafe { sqlite3_column_int64(p_stmt, 1) };
        mx_root = unsafe { sqlite3_column_int64(p_stmt, 2) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    unsafe {
        printf(c"Number of segments....................... %9d\n".as_ptr() as
                    *mut i8 as *const i8, n_seg + n_root)
    };
    unsafe {
        printf(c"Number of leaf segments.................. %9d\n".as_ptr() as
                    *mut i8 as *const i8, n_seg - n_idx)
    };
    unsafe {
        printf(c"Number of index segments................. %9d\n".as_ptr() as
                    *mut i8 as *const i8, n_idx)
    };
    unsafe {
        printf(c"Number of root segments.................. %9d\n".as_ptr() as
                    *mut i8 as *const i8, n_root)
    };
    unsafe {
        printf(c"Total size of all segments............... %9lld\n".as_ptr()
                    as *mut i8 as *const i8, sz_seg + sz_root)
    };
    unsafe {
        printf(c"Total size of all leaf segments.......... %9lld\n".as_ptr()
                    as *mut i8 as *const i8, sz_seg - sz_idx)
    };
    unsafe {
        printf(c"Total size of all index segments......... %9lld\n".as_ptr()
                    as *mut i8 as *const i8, sz_idx)
    };
    unsafe {
        printf(c"Total size of all root segments.......... %9lld\n".as_ptr()
                    as *mut i8 as *const i8, sz_root)
    };
    if n_seg > 0 {
        unsafe {
            printf(c"Average size of all segments............. %11.1f\n".as_ptr()
                        as *mut i8 as *const i8,
                (sz_seg + sz_root) as f64 / (n_seg + n_root) as f64)
        };
        unsafe {
            printf(c"Average size of leaf segments............ %11.1f\n".as_ptr()
                        as *mut i8 as *const i8,
                (sz_seg - sz_idx) as f64 / (n_seg - n_idx) as f64)
        };
    }
    if n_idx > 0 {
        unsafe {
            printf(c"Average size of index segments........... %11.1f\n".as_ptr()
                        as *mut i8 as *const i8, sz_idx as f64 / n_idx as f64)
        };
    }
    if n_root > 0 {
        unsafe {
            printf(c"Average size of root segments............ %11.1f\n".as_ptr()
                        as *mut i8 as *const i8, sz_root as f64 / n_root as f64)
        };
    }
    mx = mx_seg;
    if mx < mx_root { mx = mx_root; }
    unsafe {
        printf(c"Maximum segment size..................... %9lld\n".as_ptr()
                    as *mut i8 as *const i8, mx)
    };
    unsafe {
        printf(c"Maximum index segment size............... %9lld\n".as_ptr()
                    as *mut i8 as *const i8, mx_idx)
    };
    unsafe {
        printf(c"Maximum root segment size................ %9lld\n".as_ptr()
                    as *mut i8 as *const i8, mx_root)
    };
    p_stmt =
        unsafe {
            prepare(db, c"PRAGMA page_size".as_ptr() as *mut i8 as *const i8)
        };
    pgsz = 1024;
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        pgsz = unsafe { sqlite3_column_int(p_stmt, 0) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    unsafe {
        printf(c"Database page size....................... %9d\n".as_ptr() as
                    *mut i8 as *const i8, pgsz)
    };
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT count(*)  FROM \'%q_segments\' a JOIN \'%q_segdir\' b WHERE a.blockid BETWEEN b.start_block AND b.leaves_end_block   AND length(a.block)>%d".as_ptr()
                        as *mut i8 as *const i8, z_tab_1, z_tab_1, pgsz - 45)
        };
    n = 0;
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        n = unsafe { sqlite3_column_int(p_stmt, 0) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    n_leaf = n_seg - n_idx;
    unsafe {
        printf(c"Leaf segments larger than %5d bytes.... %9d   %5.2f%%\n".as_ptr()
                    as *mut i8 as *const i8, pgsz - 45, n,
            if n_leaf > 0 { n as f64 * 100.0 / n_leaf as f64 } else { 0.0 })
    };
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT max(level%%1024) FROM \'%q_segdir\'".as_ptr() as
                        *mut i8 as *const i8, z_tab_1)
        };
    mx_level = 0;
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        mx_level = unsafe { sqlite3_column_int(p_stmt, 0) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    {
        i = 0;
        '__b21: loop {
            if !(i <= mx_level) { break '__b21; }
            '__c21: loop {
                p_stmt =
                    unsafe {
                        prepare(db,
                            c"SELECT count(*), sum(len), avg(len), max(len), sum(len>%d),       count(distinct idx)  FROM (SELECT length(a.block) AS len, idx          FROM \'%q_segments\' a JOIN \'%q_segdir\' b         WHERE (a.blockid BETWEEN b.start_block AND b.leaves_end_block)           AND (b.level%%1024)==%d)".as_ptr()
                                    as *mut i8 as *const i8, pgsz - 45, z_tab_1, z_tab_1, i)
                    };
                if unsafe { sqlite3_step(p_stmt) } == 100 &&
                        {
                                n_leaf = unsafe { sqlite3_column_int(p_stmt, 0) };
                                n_leaf
                            } > 0 {
                    let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
                    n_idx = unsafe { sqlite3_column_int(p_stmt, 5) };
                    unsafe {
                        printf(c"For level %d:\n".as_ptr() as *mut i8 as *const i8,
                            i)
                    };
                    unsafe {
                        printf(c"  Number of indexes...................... %9d\n".as_ptr()
                                    as *mut i8 as *const i8, n_idx)
                    };
                    unsafe {
                        printf(c"  Number of leaf segments................ %9d\n".as_ptr()
                                    as *mut i8 as *const i8, n_leaf)
                    };
                    if n_idx > 1 {
                        unsafe {
                            printf(c"  Average leaf segments per index........ %11.1f\n".as_ptr()
                                        as *mut i8 as *const i8, n_leaf as f64 / n_idx as f64)
                        };
                    }
                    unsafe {
                        printf(c"  Total size of all leaf segments........ %9lld\n".as_ptr()
                                    as *mut i8 as *const i8,
                            { sz = unsafe { sqlite3_column_int64(p_stmt, 1) }; sz })
                    };
                    unsafe {
                        printf(c"  Average size of leaf segments.......... %11.1f\n".as_ptr()
                                    as *mut i8 as *const i8,
                            unsafe { sqlite3_column_double(p_stmt, 2) })
                    };
                    if n_idx > 1 {
                        unsafe {
                            printf(c"  Average leaf segment size per index.... %11.1f\n".as_ptr()
                                        as *mut i8 as *const i8, sz as f64 / n_idx as f64)
                        };
                    }
                    unsafe {
                        printf(c"  Maximum leaf segment size.............. %9lld\n".as_ptr()
                                    as *mut i8 as *const i8,
                            unsafe { sqlite3_column_int64(p_stmt, 3) })
                    };
                    n = unsafe { sqlite3_column_int(p_stmt, 4) };
                    unsafe {
                        printf(c"  Leaf segments larger than %5d bytes.. %9d   %5.2f%%\n".as_ptr()
                                    as *mut i8 as *const i8, pgsz - 45, n,
                            n as f64 * 100.0 / n_leaf as f64)
                    };
                }
                unsafe { sqlite3_finalize(p_stmt) };
                break '__c21;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn print_tree_line(i_lower_1: sqlite3_int64,
    i_upper_1: sqlite3_int64) -> () {
    unsafe {
        printf(c"                 tree   %9lld".as_ptr() as *mut i8 as
                *const i8, i_lower_1)
    };
    if i_upper_1 > i_lower_1 {
        unsafe {
            printf(c" thru %9lld  (%lld blocks)".as_ptr() as *mut i8 as
                    *const i8, i_upper_1,
                i_upper_1 - i_lower_1 + 1 as sqlite3_int64)
        };
    }
    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
}
extern "C" fn is_null_segment(db: *mut sqlite3, z_tab_1: *const i8,
    i_block_id_1: sqlite3_int64) -> i32 {
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut rc: i32 = 1;
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT block IS NULL FROM \'%q_segments\' WHERE blockid=%lld".as_ptr()
                        as *mut i8 as *const i8, z_tab_1, i_block_id_1)
        };
    if unsafe { sqlite3_step(p_stmt) } == 100 {
        rc = unsafe { sqlite3_column_int(p_stmt, 0) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    return rc;
}
extern "C" fn show_segdir_map(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    let mut mx_index: i32 = 0;
    let mut i_index: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut p_stmt2: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut prev_level: i32 = 0;
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT max(level/1024) FROM \'%q_segdir\'".as_ptr() as
                        *mut i8 as *const i8, z_tab_1)
        };
    if unsafe { sqlite3_step(p_stmt) } == 100 {
        mx_index = unsafe { sqlite3_column_int(p_stmt, 0) };
    } else { mx_index = 0; }
    unsafe { sqlite3_finalize(p_stmt) };
    unsafe {
        printf(c"Number of inverted indices............... %3d\n".as_ptr() as
                    *mut i8 as *const i8, mx_index + 1)
    };
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT level, idx, start_block, leaves_end_block, end_block, rowid  FROM \'%q_segdir\' WHERE level/1024==? ORDER BY level DESC, idx".as_ptr()
                        as *mut i8 as *const i8, z_tab_1)
        };
    p_stmt2 =
        unsafe {
            prepare(db,
                c"SELECT blockid FROM \'%q_segments\' WHERE blockid BETWEEN ? AND ? ORDER BY blockid".as_ptr()
                        as *mut i8 as *const i8, z_tab_1)
        };
    {
        i_index = 0;
        '__b22: loop {
            if !(i_index <= mx_index) { break '__b22; }
            '__c22: loop {
                if mx_index > 0 {
                    unsafe {
                        printf(c"**************************** Index %d ****************************\n".as_ptr()
                                    as *mut i8 as *const i8, i_index)
                    };
                }
                unsafe { sqlite3_bind_int(p_stmt, 1, i_index) };
                prev_level = -1;
                while unsafe { sqlite3_step(p_stmt) } == 100 {
                    let i_level: i32 =
                        unsafe { sqlite3_column_int(p_stmt, 0) } % 1024;
                    let i_idx: i32 = unsafe { sqlite3_column_int(p_stmt, 1) };
                    let i_start: sqlite3_int64 =
                        unsafe { sqlite3_column_int64(p_stmt, 2) };
                    let i_l_end: sqlite3_int64 =
                        unsafe { sqlite3_column_int64(p_stmt, 3) };
                    let i_end: sqlite3_int64 =
                        unsafe { sqlite3_column_int64(p_stmt, 4) };
                    let mut rtag: [i8; 20] = [0; 20];
                    if i_level != prev_level {
                        unsafe {
                            printf(c"level %2d idx %2d".as_ptr() as *mut i8 as
                                    *const i8, i_level, i_idx)
                        };
                        prev_level = i_level;
                    } else {
                        unsafe {
                            printf(c"         idx %2d".as_ptr() as *mut i8 as *const i8,
                                i_idx)
                        };
                    }
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 20]>() as i32,
                            &raw mut rtag[0 as usize] as *mut i8,
                            c"r%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_column_int64(p_stmt, 5) })
                    };
                    unsafe {
                        printf(c"  root   %9s\n".as_ptr() as *mut i8 as *const i8,
                            &raw mut rtag[0 as usize] as *mut i8)
                    };
                    if i_l_end > i_start {
                        let mut i_lower: sqlite3_int64 = 0 as sqlite3_int64;
                        let mut i_prev: sqlite3_int64 = 0 as sqlite3_int64;
                        let mut i_x: sqlite3_int64 = 0 as sqlite3_int64;
                        if i_l_end + 1 as sqlite3_int64 <= i_end {
                            unsafe {
                                sqlite3_bind_int64(p_stmt2, 1, i_l_end + 1 as sqlite3_int64)
                            };
                            unsafe { sqlite3_bind_int64(p_stmt2, 2, i_end) };
                            i_lower = -1 as sqlite3_int64;
                            while unsafe { sqlite3_step(p_stmt2) } == 100 {
                                i_x = unsafe { sqlite3_column_int64(p_stmt2, 0) };
                                if i_lower < 0 as i64 {
                                    i_lower = { i_prev = i_x; i_prev };
                                } else if i_x == i_prev + 1 as sqlite3_int64 {
                                    i_prev = i_x;
                                } else {
                                    print_tree_line(i_lower, i_prev);
                                    i_lower = { i_prev = i_x; i_prev };
                                }
                            }
                            unsafe { sqlite3_reset(p_stmt2) };
                            if i_lower >= 0 as i64 {
                                if i_lower == i_prev && i_lower == i_end &&
                                        is_null_segment(db, z_tab_1, i_lower) != 0 {
                                    unsafe {
                                        printf(c"                 null   %9lld\n".as_ptr() as
                                                    *mut i8 as *const i8, i_lower)
                                    };
                                } else { print_tree_line(i_lower, i_prev); }
                            }
                        }
                        unsafe {
                            printf(c"                 leaves %9lld thru %9lld  (%lld blocks)\n".as_ptr()
                                        as *mut i8 as *const i8, i_start, i_l_end,
                                i_l_end - i_start + 1 as sqlite3_int64)
                        };
                    }
                }
                unsafe { sqlite3_reset(p_stmt) };
                break '__c22;
            }
            { let __p = &mut i_index; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_finalize(p_stmt) };
    unsafe { sqlite3_finalize(p_stmt2) };
}
extern "C" fn decode_segment(a_data_1: *const u8, n_data_1: i32) -> () {
    let mut i_child: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_prefix: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n_term: sqlite3_int64 = 0 as sqlite3_int64;
    let mut n: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_docsz: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_height: i32 = 0;
    let mut i: sqlite3_int64 = 0 as sqlite3_int64;
    let mut cnt: i32 = 0;
    let mut z_term: [i8; 1000] = [0; 1000];
    i += get_varint(a_data_1, &mut n) as sqlite3_int64;
    i_height = n as i32;
    unsafe {
        printf(c"height: %d\n".as_ptr() as *mut i8 as *const i8, i_height)
    };
    if i_height > 0 {
        i +=
            get_varint(unsafe { a_data_1.offset(i as isize) }, &mut i_child)
                as sqlite3_int64;
        unsafe {
            printf(c"left-child: %lld\n".as_ptr() as *mut i8 as *const i8,
                i_child)
        };
    }
    while i < n_data_1 as i64 {
        if { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } > 0 {
            i +=
                get_varint(unsafe { a_data_1.offset(i as isize) },
                        &mut i_prefix) as sqlite3_int64;
        } else { i_prefix = 0 as sqlite3_int64; }
        i +=
            get_varint(unsafe { a_data_1.offset(i as isize) }, &mut n_term) as
                sqlite3_int64;
        if (i_prefix + n_term + 1 as sqlite3_int64) as u64 >=
                core::mem::size_of::<[i8; 1000]>() as u64 {
            eprintln!("term to long");
            unsafe { exit(1) };
        }
        unsafe {
            memcpy(unsafe {
                        (&raw mut z_term[0 as usize] as
                                *mut i8).offset(i_prefix as isize)
                    } as *mut (),
                unsafe { a_data_1.offset(i as isize) } as *const (),
                n_term as u64)
        };
        z_term[(i_prefix + n_term) as usize] = 0 as i8;
        i += n_term;
        if i_height == 0 {
            i +=
                get_varint(unsafe { a_data_1.offset(i as isize) },
                        &mut i_docsz) as sqlite3_int64;
            unsafe {
                printf(c"term: %-25s doclist %7lld bytes offset %lld\n".as_ptr()
                            as *mut i8 as *const i8,
                    &raw mut z_term[0 as usize] as *mut i8, i_docsz, i)
            };
            i += i_docsz;
        } else {
            unsafe {
                printf(c"term: %-25s child %lld\n".as_ptr() as *mut i8 as
                        *const i8, &raw mut z_term[0 as usize] as *mut i8,
                    { let __p = &mut i_child; *__p += 1; *__p })
            };
        }
    }
}
extern "C" fn print_blob(a_data_1: &[u8]) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut z_ofst_fmt: *const i8 = core::ptr::null();
        let per_line: i32 = 16 as i32;
        if a_data_1.len() as i32 & !4095 == 0 {
            z_ofst_fmt = c" %03x: ".as_ptr() as *mut i8 as *const i8;
        } else if a_data_1.len() as i32 & !65535 == 0 {
            z_ofst_fmt = c" %04x: ".as_ptr() as *mut i8 as *const i8;
        } else if a_data_1.len() as i32 & !1048575 == 0 {
            z_ofst_fmt = c" %05x: ".as_ptr() as *mut i8 as *const i8;
        } else if a_data_1.len() as i32 & !16777215 == 0 {
            z_ofst_fmt = c" %06x: ".as_ptr() as *mut i8 as *const i8;
        } else { z_ofst_fmt = c" %08x: ".as_ptr() as *mut i8 as *const i8; }
        {
            i = 0;
            '__b26: loop {
                if !(i < a_data_1.len() as i32) { break '__b26; }
                '__c26: loop {
                    unsafe { fprintf(__stdoutp, z_ofst_fmt, i) };
                    {
                        j = 0;
                        '__b27: loop {
                            if !(j < per_line) { break '__b27; }
                            '__c27: loop {
                                if i + j > a_data_1.len() as i32 {
                                    unsafe {
                                        fprintf(__stdoutp, c"   ".as_ptr() as *mut i8 as *const i8)
                                    };
                                } else {
                                    unsafe {
                                        fprintf(__stdoutp,
                                            c"%02x ".as_ptr() as *mut i8 as *const i8,
                                            a_data_1[(i + j) as usize] as i32)
                                    };
                                }
                                break '__c27;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    {
                        j = 0;
                        '__b28: loop {
                            if !(j < per_line) { break '__b28; }
                            '__c28: loop {
                                if i + j > a_data_1.len() as i32 {
                                    unsafe {
                                        fprintf(__stdoutp, c" ".as_ptr() as *mut i8 as *const i8)
                                    };
                                } else {
                                    unsafe {
                                        fprintf(__stdoutp, c"%c".as_ptr() as *mut i8 as *const i8,
                                            if unsafe { isprint(a_data_1[(i + j) as usize] as i32) } !=
                                                    0 {
                                                a_data_1[(i + j) as usize] as i32
                                            } else { '.' as i32 })
                                    };
                                }
                                break '__c28;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        fprintf(__stdoutp, c"\n".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__c26;
                }
                i += per_line as i32;
            }
        }
    }
}
extern "C" fn atoi64(mut z: *const i8) -> sqlite3_int64 {
    let mut v: sqlite3_int64 = 0 as sqlite3_int64;
    while unsafe { *z.offset(0 as isize) } as i32 >= '0' as i32 &&
            unsafe { *z.offset(0 as isize) } as i32 <= '9' as i32 {
        v =
            v * 10 as sqlite3_int64 +
                    unsafe { *z.offset(0 as isize) } as sqlite3_int64 -
                '0' as i32 as sqlite3_int64;
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return v;
}
extern "C" fn prepare_to_get_segment(db: *mut sqlite3, z_tab_1: *const i8,
    z_id_1: *const i8) -> *mut sqlite3_stmt {
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    if unsafe { *z_id_1.offset(0 as isize) } as i32 == 'r' as i32 {
        p_stmt =
            unsafe {
                prepare(db,
                    c"SELECT root FROM \'%q_segdir\' WHERE rowid=%lld".as_ptr()
                            as *mut i8 as *const i8, z_tab_1,
                    atoi64(unsafe { z_id_1.offset(1 as isize) }))
            };
    } else {
        p_stmt =
            unsafe {
                prepare(db,
                    c"SELECT block FROM \'%q_segments\' WHERE blockid=%lld".as_ptr()
                            as *mut i8 as *const i8, z_tab_1, atoi64(z_id_1))
            };
    }
    return p_stmt;
}
extern "C" fn show_segment(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    unsafe {
        let mut a_data: *const u8 = core::ptr::null();
        let mut n_data: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        p_stmt =
            prepare_to_get_segment(db, z_tab_1,
                unsafe { *az_extra.offset(0 as isize) } as *const i8);
        if unsafe { sqlite3_step(p_stmt) } != 100 {
            unsafe { sqlite3_finalize(p_stmt) };
            return;
        }
        n_data = unsafe { sqlite3_column_bytes(p_stmt, 0) };
        a_data = unsafe { sqlite3_column_blob(p_stmt, 0) } as *const u8;
        unsafe {
            printf(c"Segment %s of size %d bytes:\n".as_ptr() as *mut i8 as
                    *const i8, unsafe { *az_extra.offset(0 as isize) }, n_data)
        };
        if find_option(c"raw".as_ptr() as *mut i8 as *const i8, 0,
                    core::ptr::null()) != core::ptr::null() {
            print_blob(unsafe {
                    let __p = a_data as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, n_data as usize) }
                });
        } else { decode_segment(a_data, n_data); }
        unsafe { sqlite3_finalize(p_stmt) };
    }
}
extern "C" fn decode_doclist(a_data_1: *const u8, n_data_1: i32) -> () {
    let mut i_prev_docid: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_docid: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_pos: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_prev_pos: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_col: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i: i32 = 0;
    while i < n_data_1 {
        i += get_varint(unsafe { a_data_1.offset(i as isize) }, &mut i_docid);
        unsafe {
            printf(c"docid %lld col0".as_ptr() as *mut i8 as *const i8,
                i_docid + i_prev_docid)
        };
        i_prev_docid += i_docid;
        i_prev_pos = 0 as sqlite3_int64;
        loop {
            i +=
                get_varint(unsafe { a_data_1.offset(i as isize) },
                    &mut i_pos);
            if i_pos == 1 as i64 {
                i +=
                    get_varint(unsafe { a_data_1.offset(i as isize) },
                        &mut i_col);
                unsafe {
                    printf(c" col%lld".as_ptr() as *mut i8 as *const i8, i_col)
                };
                i_prev_pos = 0 as sqlite3_int64;
            } else if i_pos == 0 as i64 {
                unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                break;
            } else {
                i_prev_pos += i_pos - 2 as sqlite3_int64;
                unsafe {
                    printf(c" %lld".as_ptr() as *mut i8 as *const i8,
                        i_prev_pos)
                };
            }
        }
    }
}
extern "C" fn show_doclist(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    unsafe {
        let mut a_data: *const u8 = core::ptr::null();
        let mut offset: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_data: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        offset = atoi64(unsafe { *az_extra.offset(1 as isize) } as *const i8);
        n_data =
            unsafe {
                atoi(unsafe { *az_extra.offset(2 as isize) } as *const i8)
            };
        p_stmt =
            prepare_to_get_segment(db, z_tab_1,
                unsafe { *az_extra.offset(0 as isize) } as *const i8);
        if unsafe { sqlite3_step(p_stmt) } != 100 {
            unsafe { sqlite3_finalize(p_stmt) };
            return;
        }
        a_data = unsafe { sqlite3_column_blob(p_stmt, 0) } as *const u8;
        unsafe {
            printf(c"Doclist at %s offset %lld of size %d bytes:\n".as_ptr()
                        as *mut i8 as *const i8,
                unsafe { *az_extra.offset(0 as isize) }, offset, n_data)
        };
        if find_option(c"raw".as_ptr() as *mut i8 as *const i8, 0,
                    core::ptr::null()) != core::ptr::null() {
            print_blob(unsafe {
                    let __p =
                        unsafe { a_data.offset(offset as isize) } as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, n_data as usize) }
                });
        } else {
            decode_doclist(unsafe { a_data.offset(offset as isize) }, n_data);
        }
        unsafe { sqlite3_finalize(p_stmt) };
    }
}
extern "C" fn list_big_segments(db: *mut sqlite3, z_tab_1: *const i8) -> () {
    let mut n_top: i32 = 0;
    let mut i: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
    let mut id: sqlite3_int64 = 0 as sqlite3_int64;
    n_top =
        unsafe {
            atoi(find_option(c"top".as_ptr() as *mut i8 as *const i8, 1,
                    c"25".as_ptr() as *mut i8 as *const i8))
        };
    unsafe {
        printf(c"The %d largest segments:\n".as_ptr() as *mut i8 as *const i8,
            n_top)
    };
    p_stmt =
        unsafe {
            prepare(db,
                c"SELECT blockid, length(block) AS len FROM \'%q_segments\' ORDER BY 2 DESC, 1 LIMIT %d".as_ptr()
                        as *mut i8 as *const i8, z_tab_1, n_top)
        };
    i = 0;
    while unsafe { sqlite3_step(p_stmt) } == 100 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        id = unsafe { sqlite3_column_int64(p_stmt, 0) };
        sz = unsafe { sqlite3_column_int64(p_stmt, 1) };
        unsafe {
            printf(c"  %2d. %9lld size %lld\n".as_ptr() as *mut i8 as
                    *const i8, i, id, sz)
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
}
extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        unsafe {
            fprintf(__stderrp,
                c"Usage: %s DATABASE\n   or: %s DATABASE FTS3TABLE ARGS...\n".as_ptr()
                        as *mut i8 as *const i8, argv0, argv0)
        };
        eprintln!("ARGS:\n  big-segments [--top N]                    show the largest segments\n  doclist BLOCKID OFFSET SIZE [--raw]       Decode a doclist\n  schema                                    FTS table schema\n  segdir                                    directory of segments\n  segment BLOCKID [--raw]                   content of a segment\n  segment-stats                             info on segment sizes\n  stat                                      the %_stat table\n  vocabulary [--top N]                      document vocabulary");
        unsafe { exit(1) };
    }
}
extern "C" fn __main_inner(argc: i32, argv: *mut *mut i8) -> Result<(), i32> {
    unsafe {
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut z_tab: *const i8 = core::ptr::null();
        let mut z_cmd: *const i8 = core::ptr::null();
        if argc < 2 {
            usage(unsafe { *argv.offset(0 as isize) } as *const i8);
        }
        rc =
            unsafe {
                sqlite3_open(unsafe { *argv.offset(1 as isize) } as *const i8,
                    &mut db)
            };
        if rc != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"Cannot open %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) })
            };
            unsafe { exit(1) };
        }
        if argc == 2 {
            let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
            let mut cnt: i32 = 0;
            p_stmt =
                unsafe {
                    prepare(db,
                        c"SELECT b.sql  FROM sqlite_schema a, sqlite_schema b WHERE a.name GLOB \'*_segdir\'   AND b.name=substr(a.name,1,length(a.name)-7) ORDER BY 1".as_ptr()
                                as *mut i8 as *const i8)
                };
            while unsafe { sqlite3_step(p_stmt) } == 100 {
                { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                unsafe {
                    printf(c"%s;\n".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_column_text(p_stmt, 0) })
                };
            }
            unsafe { sqlite3_finalize(p_stmt) };
            if cnt == 0 {
                unsafe {
                    printf(c"/* No FTS3/4 tables found in database %s */\n".as_ptr()
                                as *mut i8 as *const i8,
                        unsafe { *argv.offset(1 as isize) })
                };
            }
            return Ok(());
        }
        if argc < 4 {
            usage(unsafe { *argv.offset(0 as isize) } as *const i8);
        }
        z_tab = unsafe { *argv.offset(2 as isize) } as *const i8;
        z_cmd = unsafe { *argv.offset(3 as isize) } as *const i8;
        n_extra = argc - 4;
        az_extra = unsafe { argv.offset(4 as isize) };
        if unsafe {
                    strcmp(z_cmd,
                        c"big-segments".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            list_big_segments(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd, c"doclist".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            if argc < 7 {
                usage(unsafe { *argv.offset(0 as isize) } as *const i8);
            }
            show_doclist(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd, c"schema".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            show_schema(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd, c"segdir".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            show_segdir_map(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd, c"segment".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            if argc < 5 {
                usage(unsafe { *argv.offset(0 as isize) } as *const i8);
            }
            show_segment(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd,
                        c"segment-stats".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            show_segment_stats(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd, c"stat".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            show_stat(db, z_tab);
        } else if unsafe {
                    strcmp(z_cmd,
                        c"vocabulary".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            show_vocabulary(db, z_tab);
        } else { usage(unsafe { *argv.offset(0 as isize) } as *const i8); }
        return Ok(());
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
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
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn atoi(_: *const i8)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn isprint(_c: i32)
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