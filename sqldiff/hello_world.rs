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
#[repr(C)]
#[derive(Copy, Clone)]
struct GlobalVars {
    z_argv0: *const i8,
    b_schema_only: i32,
    b_schema_pk: i32,
    b_handle_vtab: i32,
    f_debug: u32,
    b_schema_compare: i32,
    db: *mut sqlite3,
}
#[unsafe(no_mangle)]
pub static mut g: GlobalVars = unsafe { core::mem::zeroed() };
extern "C" fn str_free(p_str_1: *mut sqlite3_str) -> () {
    unsafe {
        sqlite3_free(unsafe { sqlite3_str_finish(p_str_1) } as *mut ())
    };
}
unsafe extern "C" fn cmdline_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let p_out: *mut sqlite3_str =
            unsafe { sqlite3_str_new(core::ptr::null_mut()) };
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { sqlite3_str_vappendf(p_out, z_format_1, ap) };
        ();
        unsafe {
            fprintf(__stderrp, c"%s: %s\n".as_ptr() as *mut i8 as *const i8,
                g.z_argv0, unsafe { sqlite3_str_value(p_out) })
        };
        str_free(p_out);
        unsafe {
            fprintf(__stderrp,
                c"\"%s --help\" for more help\n".as_ptr() as *mut i8 as
                    *const i8, g.z_argv0)
        };
        unsafe { exit(1) };
    }
}
unsafe extern "C" fn runtime_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let p_out: *mut sqlite3_str =
            unsafe { sqlite3_str_new(core::ptr::null_mut()) };
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { sqlite3_str_vappendf(p_out, z_format_1, ap) };
        ();
        unsafe {
            fprintf(__stderrp, c"%s: %s\n".as_ptr() as *mut i8 as *const i8,
                g.z_argv0, unsafe { sqlite3_str_value(p_out) })
        };
        str_free(p_out);
        unsafe { exit(1) };
    }
}
extern "C" fn safe_id(z_id_1: *const i8) -> *mut i8 {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut c: i8 = 0 as i8;
    if unsafe { *z_id_1.offset(0 as isize) } as i32 == 0 {
        return unsafe {
                sqlite3_mprintf(c"\"\"".as_ptr() as *mut i8 as *const i8)
            };
    }
    {
        i = { x = 0; x };
        '__b0: loop {
            if !({ c = unsafe { *z_id_1.offset(i as isize) } as i8; c } as i32
                            != 0) {
                break '__b0;
            }
            '__c0: loop {
                if (unsafe { isalpha(c as i32) } == 0) as i32 != 0 &&
                        c as i32 != '_' as i32 {
                    if i > 0 && unsafe { isdigit(c as i32) } != 0 {
                        { let __p = &mut x; let __t = *__p; *__p += 1; __t };
                    } else {
                        return unsafe {
                                sqlite3_mprintf(c"\"%w\"".as_ptr() as *mut i8 as *const i8,
                                    z_id_1)
                            };
                    }
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if x != 0 ||
            (unsafe { sqlite3_keyword_check(z_id_1, i) } == 0) as i32 != 0 {
        return unsafe {
                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                    z_id_1)
            };
    }
    return unsafe {
            sqlite3_mprintf(c"\"%w\"".as_ptr() as *mut i8 as *const i8,
                z_id_1)
        };
}
extern "C" fn db_vprepare(z_format_1: *const i8, ap: *mut i8)
    -> *mut sqlite3_stmt {
    unsafe {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        if z_sql == core::ptr::null_mut() {
            unsafe {
                runtime_error(c"out of memory".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        rc =
            unsafe {
                sqlite3_prepare_v2(g.db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                runtime_error(c"SQL statement error: %s\n\"%s\"".as_ptr() as
                            *mut i8 as *const i8, unsafe { sqlite3_errmsg(g.db) },
                    z_sql)
            };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        return p_stmt;
    }
}
unsafe extern "C" fn db_prepare(z_format_1: *const i8, mut __va0: ...)
    -> *mut sqlite3_stmt {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = db_vprepare(z_format_1, ap);
    ();
    return p_stmt;
}
extern "C" fn namelist_free(az: *mut *mut i8) -> () {
    if !(az).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b1: loop {
                if !(!(unsafe { *az.offset(i as isize) }).is_null()) {
                    break '__b1;
                }
                '__c1: loop {
                    unsafe {
                        sqlite3_free(unsafe { *az.offset(i as isize) } as *mut ())
                    };
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(az as *mut ()) };
    }
}
extern "C" fn column_names(z_db_1: *const i8, z_tab_1: *const i8,
    pn_p_key_1: &mut i32, pb_rowid_1: *mut i32) -> *mut *mut i8 {
    unsafe {
        let mut az: *mut *mut i8 = core::ptr::null_mut();
        let mut naz: i64 = 0 as i64;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_pk_idx_name: *mut i8 = core::ptr::null_mut();
        let mut true_pk: i32 = 0;
        let mut n_pk: i64 = 0 as i64;
        let mut i: i64 = 0 as i64;
        let mut j: i64 = 0 as i64;
        if g.b_schema_pk == 0 {
            p_stmt =
                unsafe {
                    db_prepare(c"PRAGMA %s.index_list=%Q".as_ptr() as *mut i8 as
                            *const i8, z_db_1, z_tab_1)
                };
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                if unsafe {
                            sqlite3_stricmp(unsafe { sqlite3_column_text(p_stmt, 3) } as
                                    *const i8, c"pk".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    z_pk_idx_name =
                        unsafe {
                            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_column_text(p_stmt, 1) })
                        };
                    break;
                }
            }
            unsafe { sqlite3_finalize(p_stmt) };
            if !(z_pk_idx_name).is_null() {
                let mut n_key: i32 = 0;
                let mut n_col: i32 = 0;
                true_pk = 0;
                p_stmt =
                    unsafe {
                        db_prepare(c"PRAGMA %s.index_xinfo=%Q".as_ptr() as *mut i8
                                as *const i8, z_db_1, z_pk_idx_name)
                    };
                while 100 == unsafe { sqlite3_step(p_stmt) } {
                    { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                    if unsafe { sqlite3_column_int(p_stmt, 5) } != 0 {
                        { let __p = &mut n_key; let __t = *__p; *__p += 1; __t };
                        continue;
                    }
                    if unsafe { sqlite3_column_int(p_stmt, 1) } >= 0 {
                        true_pk = 1;
                    }
                }
                if n_col == n_key { true_pk = 1; }
                if true_pk != 0 {
                    n_pk = n_key as i64;
                } else { n_pk = 1 as i64; }
                unsafe { sqlite3_finalize(p_stmt) };
                unsafe { sqlite3_free(z_pk_idx_name as *mut ()) };
            } else { true_pk = 1; n_pk = 1 as i64; }
            p_stmt =
                unsafe {
                    db_prepare(c"PRAGMA %s.table_info=%Q".as_ptr() as *mut i8 as
                            *const i8, z_db_1, z_tab_1)
                };
        } else {
            n_pk = 0 as i64;
            p_stmt =
                unsafe {
                    db_prepare(c"PRAGMA %s.table_info=%Q".as_ptr() as *mut i8 as
                            *const i8, z_db_1, z_tab_1)
                };
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                if unsafe { sqlite3_column_int(p_stmt, 5) } > 0 {
                    { let __p = &mut n_pk; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { sqlite3_reset(p_stmt) };
            if n_pk == 0 as i64 { n_pk = 1 as i64; }
            true_pk = 1;
        }
        if g.b_schema_compare != 0 {
            if !(unsafe {
                                            sqlite3_stricmp(z_tab_1,
                                                c"sqlite_schema".as_ptr() as *mut i8 as *const i8)
                                        } == 0 ||
                                    unsafe {
                                            sqlite3_stricmp(z_tab_1,
                                                c"sqlite_master".as_ptr() as *mut i8 as *const i8)
                                        } == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"columnNames".as_ptr() as *const i8,
                        c"sqldiff.c".as_ptr() as *mut i8 as *const i8, 270,
                        c"sqlite3_stricmp(zTab,\"sqlite_schema\")==0 || sqlite3_stricmp(zTab,\"sqlite_master\")==0".as_ptr()
                                as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            n_pk = 2 as i64;
            true_pk = 0;
        }
        *pn_p_key_1 = n_pk as i32;
        naz = n_pk;
        az =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<*mut i8>() as u64 *
                            (n_pk + 1 as i64) as u64)
                } as *mut *mut i8;
        if az == core::ptr::null_mut() {
            unsafe {
                runtime_error(c"out of memory".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        unsafe {
            memset(az as *mut (), 0,
                core::mem::size_of::<*mut i8>() as u64 *
                    (n_pk + 1 as i64) as u64)
        };
        if g.b_schema_compare != 0 {
            unsafe {
                *az.offset(0 as isize) =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            c"type".as_ptr() as *mut i8)
                    }
            };
            unsafe {
                *az.offset(1 as isize) =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            c"name".as_ptr() as *mut i8)
                    }
            };
        }
        while 100 == unsafe { sqlite3_step(p_stmt) } {
            let sid: *mut i8 =
                safe_id(unsafe { sqlite3_column_text(p_stmt, 1) } as *mut i8
                        as *const i8);
            let mut i_p_key: i32 = 0;
            if true_pk != 0 &&
                    {
                            i_p_key = unsafe { sqlite3_column_int(p_stmt, 5) };
                            i_p_key
                        } > 0 {
                unsafe { *az.offset((i_p_key - 1) as isize) = sid };
            } else {
                if (g.b_schema_compare == 0) as i32 != 0 ||
                        !(unsafe {
                                                    strcmp(sid as *const i8,
                                                        c"rootpage".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                            unsafe {
                                                    strcmp(sid as *const i8,
                                                        c"name".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                        unsafe {
                                                strcmp(sid as *const i8,
                                                    c"type".as_ptr() as *mut i8 as *const i8)
                                            } == 0) as i32 != 0 {
                    az =
                        unsafe {
                                sqlite3_realloc64(az as *mut (),
                                    core::mem::size_of::<*mut i8>() as u64 *
                                        (naz + 2 as i64) as u64)
                            } as *mut *mut i8;
                    if az == core::ptr::null_mut() {
                        unsafe {
                            runtime_error(c"out of memory".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                    }
                    unsafe {
                        *az.offset({
                                            let __p = &mut naz;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = sid
                    };
                }
            }
        }
        unsafe { sqlite3_finalize(p_stmt) };
        if !(az).is_null() {
            unsafe { *az.offset(naz as isize) = core::ptr::null_mut() };
        }
        if !(pb_rowid_1).is_null() {
            unsafe {
                *pb_rowid_1 =
                    (unsafe { *az.offset(0 as isize) } == core::ptr::null_mut())
                        as i32
            };
        }
        if unsafe { *az.offset(0 as isize) } == core::ptr::null_mut() {
            let az_rowid: [*const i8; 3] =
                [c"rowid".as_ptr() as *const i8,
                        c"_rowid_".as_ptr() as *const i8,
                        c"oid".as_ptr() as *const i8];
            {
                i = 0 as i64;
                '__b6: loop {
                    if !((i as u64) <
                                    core::mem::size_of::<[*const i8; 3]>() as u64 /
                                        core::mem::size_of::<*const i8>() as u64) {
                        break '__b6;
                    }
                    '__c6: loop {
                        {
                            j = 1 as i64;
                            '__b7: loop {
                                if !(j < naz) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                sqlite3_stricmp(unsafe { *az.offset(j as isize) } as
                                                        *const i8, az_rowid[i as usize])
                                            } == 0 {
                                        break '__b7;
                                    }
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if j >= naz {
                            unsafe {
                                *az.offset(0 as isize) =
                                    unsafe {
                                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                            az_rowid[i as usize])
                                    }
                            };
                            break '__b6;
                        }
                        break '__c6;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if unsafe { *az.offset(0 as isize) } == core::ptr::null_mut() {
                {
                    i = 1 as i64;
                    '__b8: loop {
                        if !(i < naz) { break '__b8; }
                        '__c8: loop {
                            unsafe {
                                sqlite3_free(unsafe { *az.offset(i as isize) } as *mut ())
                            };
                            break '__c8;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { sqlite3_free(az as *mut ()) };
                az = core::ptr::null_mut();
            }
        }
        return az;
    }
}
extern "C" fn print_quoted(out: *mut FILE, x_1: *mut sqlite3_value) -> () {
    '__s9:
        {
        match unsafe { sqlite3_value_type(x_1) } {
            2 => {
                {
                    let mut r1: f64 = 0.0;
                    let mut z_buf: [i8; 50] = [0; 50];
                    r1 = unsafe { sqlite3_value_double(x_1) };
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 50]>() as i32,
                            &raw mut z_buf[0 as usize] as *mut i8,
                            c"%!.15g".as_ptr() as *mut i8 as *const i8, r1)
                    };
                    unsafe {
                        fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8,
                            &raw mut z_buf[0 as usize] as *mut i8)
                    };
                    break '__s9;
                }
                {
                    unsafe {
                        fprintf(out, c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_int64(x_1) })
                    };
                    break '__s9;
                }
                {
                    let z_blob: *const u8 =
                        unsafe { sqlite3_value_blob(x_1) } as *const u8;
                    let n_blob: i32 = unsafe { sqlite3_value_bytes(x_1) };
                    if !(z_blob).is_null() {
                        let mut i: i32 = 0;
                        unsafe {
                            fprintf(out, c"x\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = 0;
                            '__b10: loop {
                                if !(i < n_blob) { break '__b10; }
                                '__c10: loop {
                                    unsafe {
                                        fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *z_blob.offset(i as isize) } as i32)
                                    };
                                    break '__c10;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        unsafe {
                            fprintf(out, c"X\'\'".as_ptr() as *mut i8 as *const i8)
                        };
                    }
                    break '__s9;
                }
                {
                    let z_arg: *const u8 = unsafe { sqlite3_value_text(x_1) };
                    if z_arg == core::ptr::null() {
                        unsafe {
                            fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        let mut inctl: i32 = 0;
                        let mut i: i32 = 0;
                        let mut j: i32 = 0;
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = { j = 0; j };
                            '__b11: loop {
                                if !(unsafe { *z_arg.offset(i as isize) } != 0) {
                                    break '__b11;
                                }
                                '__c11: loop {
                                    let c: i8 = unsafe { *z_arg.offset(i as isize) } as i8;
                                    let ctl: i32 = unsafe { iscntrl(c as u8 as i32) };
                                    if ctl > inctl {
                                        inctl = ctl;
                                        unsafe {
                                            fprintf(out,
                                                c"%.*s\'||X\'%02x".as_ptr() as *mut i8 as *const i8, i - j,
                                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                                    *const u8, c as i32)
                                        };
                                        j = i + 1;
                                    } else if ctl != 0 {
                                        unsafe {
                                            fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                                c as i32)
                                        };
                                        j = i + 1;
                                    } else {
                                        if inctl != 0 {
                                            inctl = 0;
                                            unsafe {
                                                fprintf(out, c"\'\n||\'".as_ptr() as *mut i8 as *const i8)
                                            };
                                        }
                                        if c as i32 == '\'' as i32 {
                                            unsafe {
                                                fprintf(out, c"%.*s\'".as_ptr() as *mut i8 as *const i8,
                                                    i - j + 1,
                                                    unsafe { &raw const *z_arg.offset(j as isize) } as
                                                        *const u8)
                                            };
                                            j = i + 1;
                                        }
                                    }
                                    break '__c11;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"%s\'".as_ptr() as *mut i8 as *const i8,
                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                    *const u8)
                        };
                    }
                    break '__s9;
                }
                {
                    unsafe {
                        fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s9;
                }
            }
            1 => {
                {
                    unsafe {
                        fprintf(out, c"%lld".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_value_int64(x_1) })
                    };
                    break '__s9;
                }
                {
                    let z_blob: *const u8 =
                        unsafe { sqlite3_value_blob(x_1) } as *const u8;
                    let n_blob: i32 = unsafe { sqlite3_value_bytes(x_1) };
                    if !(z_blob).is_null() {
                        let mut i: i32 = 0;
                        unsafe {
                            fprintf(out, c"x\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = 0;
                            '__b10: loop {
                                if !(i < n_blob) { break '__b10; }
                                '__c10: loop {
                                    unsafe {
                                        fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *z_blob.offset(i as isize) } as i32)
                                    };
                                    break '__c10;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        unsafe {
                            fprintf(out, c"X\'\'".as_ptr() as *mut i8 as *const i8)
                        };
                    }
                    break '__s9;
                }
                {
                    let z_arg: *const u8 = unsafe { sqlite3_value_text(x_1) };
                    if z_arg == core::ptr::null() {
                        unsafe {
                            fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        let mut inctl: i32 = 0;
                        let mut i: i32 = 0;
                        let mut j: i32 = 0;
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = { j = 0; j };
                            '__b11: loop {
                                if !(unsafe { *z_arg.offset(i as isize) } != 0) {
                                    break '__b11;
                                }
                                '__c11: loop {
                                    let c: i8 = unsafe { *z_arg.offset(i as isize) } as i8;
                                    let ctl: i32 = unsafe { iscntrl(c as u8 as i32) };
                                    if ctl > inctl {
                                        inctl = ctl;
                                        unsafe {
                                            fprintf(out,
                                                c"%.*s\'||X\'%02x".as_ptr() as *mut i8 as *const i8, i - j,
                                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                                    *const u8, c as i32)
                                        };
                                        j = i + 1;
                                    } else if ctl != 0 {
                                        unsafe {
                                            fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                                c as i32)
                                        };
                                        j = i + 1;
                                    } else {
                                        if inctl != 0 {
                                            inctl = 0;
                                            unsafe {
                                                fprintf(out, c"\'\n||\'".as_ptr() as *mut i8 as *const i8)
                                            };
                                        }
                                        if c as i32 == '\'' as i32 {
                                            unsafe {
                                                fprintf(out, c"%.*s\'".as_ptr() as *mut i8 as *const i8,
                                                    i - j + 1,
                                                    unsafe { &raw const *z_arg.offset(j as isize) } as
                                                        *const u8)
                                            };
                                            j = i + 1;
                                        }
                                    }
                                    break '__c11;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"%s\'".as_ptr() as *mut i8 as *const i8,
                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                    *const u8)
                        };
                    }
                    break '__s9;
                }
                {
                    unsafe {
                        fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s9;
                }
            }
            4 => {
                {
                    let z_blob: *const u8 =
                        unsafe { sqlite3_value_blob(x_1) } as *const u8;
                    let n_blob: i32 = unsafe { sqlite3_value_bytes(x_1) };
                    if !(z_blob).is_null() {
                        let mut i: i32 = 0;
                        unsafe {
                            fprintf(out, c"x\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = 0;
                            '__b10: loop {
                                if !(i < n_blob) { break '__b10; }
                                '__c10: loop {
                                    unsafe {
                                        fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *z_blob.offset(i as isize) } as i32)
                                    };
                                    break '__c10;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        unsafe {
                            fprintf(out, c"X\'\'".as_ptr() as *mut i8 as *const i8)
                        };
                    }
                    break '__s9;
                }
                {
                    let z_arg: *const u8 = unsafe { sqlite3_value_text(x_1) };
                    if z_arg == core::ptr::null() {
                        unsafe {
                            fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        let mut inctl: i32 = 0;
                        let mut i: i32 = 0;
                        let mut j: i32 = 0;
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = { j = 0; j };
                            '__b11: loop {
                                if !(unsafe { *z_arg.offset(i as isize) } != 0) {
                                    break '__b11;
                                }
                                '__c11: loop {
                                    let c: i8 = unsafe { *z_arg.offset(i as isize) } as i8;
                                    let ctl: i32 = unsafe { iscntrl(c as u8 as i32) };
                                    if ctl > inctl {
                                        inctl = ctl;
                                        unsafe {
                                            fprintf(out,
                                                c"%.*s\'||X\'%02x".as_ptr() as *mut i8 as *const i8, i - j,
                                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                                    *const u8, c as i32)
                                        };
                                        j = i + 1;
                                    } else if ctl != 0 {
                                        unsafe {
                                            fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                                c as i32)
                                        };
                                        j = i + 1;
                                    } else {
                                        if inctl != 0 {
                                            inctl = 0;
                                            unsafe {
                                                fprintf(out, c"\'\n||\'".as_ptr() as *mut i8 as *const i8)
                                            };
                                        }
                                        if c as i32 == '\'' as i32 {
                                            unsafe {
                                                fprintf(out, c"%.*s\'".as_ptr() as *mut i8 as *const i8,
                                                    i - j + 1,
                                                    unsafe { &raw const *z_arg.offset(j as isize) } as
                                                        *const u8)
                                            };
                                            j = i + 1;
                                        }
                                    }
                                    break '__c11;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"%s\'".as_ptr() as *mut i8 as *const i8,
                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                    *const u8)
                        };
                    }
                    break '__s9;
                }
                {
                    unsafe {
                        fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s9;
                }
            }
            3 => {
                {
                    let z_arg: *const u8 = unsafe { sqlite3_value_text(x_1) };
                    if z_arg == core::ptr::null() {
                        unsafe {
                            fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                        };
                    } else {
                        let mut inctl: i32 = 0;
                        let mut i: i32 = 0;
                        let mut j: i32 = 0;
                        unsafe {
                            fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                        };
                        {
                            i = { j = 0; j };
                            '__b11: loop {
                                if !(unsafe { *z_arg.offset(i as isize) } != 0) {
                                    break '__b11;
                                }
                                '__c11: loop {
                                    let c: i8 = unsafe { *z_arg.offset(i as isize) } as i8;
                                    let ctl: i32 = unsafe { iscntrl(c as u8 as i32) };
                                    if ctl > inctl {
                                        inctl = ctl;
                                        unsafe {
                                            fprintf(out,
                                                c"%.*s\'||X\'%02x".as_ptr() as *mut i8 as *const i8, i - j,
                                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                                    *const u8, c as i32)
                                        };
                                        j = i + 1;
                                    } else if ctl != 0 {
                                        unsafe {
                                            fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                                c as i32)
                                        };
                                        j = i + 1;
                                    } else {
                                        if inctl != 0 {
                                            inctl = 0;
                                            unsafe {
                                                fprintf(out, c"\'\n||\'".as_ptr() as *mut i8 as *const i8)
                                            };
                                        }
                                        if c as i32 == '\'' as i32 {
                                            unsafe {
                                                fprintf(out, c"%.*s\'".as_ptr() as *mut i8 as *const i8,
                                                    i - j + 1,
                                                    unsafe { &raw const *z_arg.offset(j as isize) } as
                                                        *const u8)
                                            };
                                            j = i + 1;
                                        }
                                    }
                                    break '__c11;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            fprintf(out, c"%s\'".as_ptr() as *mut i8 as *const i8,
                                unsafe { &raw const *z_arg.offset(j as isize) } as
                                    *const u8)
                        };
                    }
                    break '__s9;
                }
                {
                    unsafe {
                        fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s9;
                }
            }
            5 => {
                {
                    unsafe {
                        fprintf(out, c"NULL".as_ptr() as *mut i8 as *const i8)
                    };
                    break '__s9;
                }
            }
            _ => {}
        }
    }
}
extern "C" fn dump_table(z_tab_1: *const i8, out: *mut FILE) -> () {
    unsafe {
        let z_id: *mut i8 = safe_id(z_tab_1);
        let mut az: *mut *mut i8 = core::ptr::null_mut();
        let mut n_pk: i32 = 0;
        let mut n_col: i32 = 0;
        let mut i: i32 = 0;
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_sep: *const i8 = core::ptr::null();
        let mut p_ins: *mut sqlite3_str = core::ptr::null_mut();
        p_stmt =
            unsafe {
                db_prepare(c"SELECT sql FROM aux.sqlite_schema WHERE name=%Q".as_ptr()
                            as *mut i8 as *const i8, z_tab_1)
            };
        if 100 == unsafe { sqlite3_step(p_stmt) } {
            unsafe {
                fprintf(out, c"%s;\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_column_text(p_stmt, 0) })
            };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        if (g.b_schema_only == 0) as i32 != 0 {
            az =
                column_names(c"aux".as_ptr() as *mut i8 as *const i8, z_tab_1,
                    &mut n_pk, core::ptr::null_mut());
            p_ins = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
            if az == core::ptr::null_mut() {
                p_stmt =
                    unsafe {
                        db_prepare(c"SELECT * FROM aux.%s".as_ptr() as *mut i8 as
                                *const i8, z_id)
                    };
                unsafe {
                    sqlite3_str_appendf(p_ins,
                        c"INSERT INTO %s VALUES".as_ptr() as *mut i8 as *const i8,
                        z_id)
                };
            } else {
                let p_sql: *mut sqlite3_str =
                    unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                z_sep = c"SELECT".as_ptr() as *mut i8 as *const i8;
                {
                    i = 0;
                    '__b12: loop {
                        if !(!(unsafe { *az.offset(i as isize) }).is_null()) {
                            break '__b12;
                        }
                        '__c12: loop {
                            unsafe {
                                sqlite3_str_appendf(p_sql,
                                    c"%s %s".as_ptr() as *mut i8 as *const i8, z_sep,
                                    unsafe { *az.offset(i as isize) })
                            };
                            z_sep = c",".as_ptr() as *mut i8 as *const i8;
                            break '__c12;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    sqlite3_str_appendf(p_sql,
                        c" FROM aux.%s".as_ptr() as *mut i8 as *const i8, z_id)
                };
                z_sep = c" ORDER BY".as_ptr() as *mut i8 as *const i8;
                {
                    i = 1;
                    '__b13: loop {
                        if !(i <= n_pk) { break '__b13; }
                        '__c13: loop {
                            unsafe {
                                sqlite3_str_appendf(p_sql,
                                    c"%s %d".as_ptr() as *mut i8 as *const i8, z_sep, i)
                            };
                            z_sep = c",".as_ptr() as *mut i8 as *const i8;
                            break '__c13;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                p_stmt =
                    unsafe {
                        db_prepare(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_str_value(p_sql) })
                    };
                str_free(p_sql);
                unsafe {
                    sqlite3_str_appendf(p_ins,
                        c"INSERT INTO %s".as_ptr() as *mut i8 as *const i8, z_id)
                };
                z_sep = c"(".as_ptr() as *mut i8 as *const i8;
                {
                    i = 0;
                    '__b14: loop {
                        if !(!(unsafe { *az.offset(i as isize) }).is_null()) {
                            break '__b14;
                        }
                        '__c14: loop {
                            unsafe {
                                sqlite3_str_appendf(p_ins,
                                    c"%s%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                    unsafe { *az.offset(i as isize) })
                            };
                            z_sep = c",".as_ptr() as *mut i8 as *const i8;
                            break '__c14;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    sqlite3_str_appendf(p_ins,
                        c") VALUES".as_ptr() as *mut i8 as *const i8)
                };
                namelist_free(az);
            }
            n_col = unsafe { sqlite3_column_count(p_stmt) };
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                unsafe {
                    fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_str_value(p_ins) })
                };
                z_sep = c"(".as_ptr() as *mut i8 as *const i8;
                {
                    i = 0;
                    '__b16: loop {
                        if !(i < n_col) { break '__b16; }
                        '__c16: loop {
                            unsafe {
                                fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8, z_sep)
                            };
                            print_quoted(out,
                                unsafe { sqlite3_column_value(p_stmt, i) });
                            z_sep = c",".as_ptr() as *mut i8 as *const i8;
                            break '__c16;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    fprintf(out, c");\n".as_ptr() as *mut i8 as *const i8)
                };
            }
            unsafe { sqlite3_finalize(p_stmt) };
            str_free(p_ins);
        }
        p_stmt =
            unsafe {
                db_prepare(c"SELECT sql FROM aux.sqlite_schema WHERE type=\'index\' AND tbl_name=%Q AND sql IS NOT NULL".as_ptr()
                            as *mut i8 as *const i8, z_tab_1)
            };
        while 100 == unsafe { sqlite3_step(p_stmt) } {
            unsafe {
                fprintf(out, c"%s;\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_column_text(p_stmt, 0) })
            };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        unsafe { sqlite3_free(z_id as *mut ()) };
    }
}
extern "C" fn diff_one_table(z_tab_1: *const i8, out: *mut FILE) -> () {
    unsafe {
        let mut z_id: *mut i8 = core::ptr::null_mut();
        let mut az: *mut *mut i8 = core::ptr::null_mut();
        let mut az2: *mut *mut i8 = core::ptr::null_mut();
        let mut n_pk: i32 = 0;
        let mut n_pk2: i32 = 0;
        let mut n: i32 = 0;
        let mut n2: i32 = 0;
        let mut n_q: i32 = 0;
        let mut i: i32 = 0;
        let mut z_sep: *const i8 = core::ptr::null();
        let mut p_sql: *mut sqlite3_str = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_lead: *const i8 = core::ptr::null();
        let mut z_n_tab: *mut i8 = core::ptr::null_mut();
        let mut z: *mut i8 = core::ptr::null_mut();
        let mut i_type: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s19:
                {
                match __state {
                    0 => { z_id = safe_id(z_tab_1); __state = 3; }
                    2 => { str_free(p_sql); __state = 194; }
                    3 => { az = core::ptr::null_mut(); __state = 4; }
                    4 => { az2 = core::ptr::null_mut(); __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { n = 0; __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => {
                        z_lead =
                            if g.b_schema_compare != 0 {
                                    c"-- ".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 } as *const i8;
                        __state = 15;
                    }
                    15 => {
                        p_sql = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        __state = 16;
                    }
                    16 => {
                        if g.f_debug == 1 as u32 {
                            __state = 18;
                        } else { __state = 17; }
                    }
                    17 => {
                        if unsafe {
                                    sqlite3_table_column_metadata(g.db,
                                        c"aux".as_ptr() as *mut i8 as *const i8, z_tab_1,
                                        core::ptr::null(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                } != 0 {
                            __state = 31;
                        } else { __state = 30; }
                    }
                    18 => {
                        az =
                            column_names(c"aux".as_ptr() as *mut i8 as *const i8,
                                z_tab_1, &mut n_pk, core::ptr::null_mut());
                        __state = 19;
                    }
                    19 => {
                        if az == core::ptr::null_mut() {
                            __state = 21;
                        } else { __state = 22; }
                    }
                    20 => { __state = 2; }
                    21 => {
                        unsafe {
                            fprintf(__stdoutp,
                                c"Rowid not accessible for %s\n".as_ptr() as *mut i8 as
                                    *const i8, z_id)
                        };
                        __state = 20;
                    }
                    22 => {
                        unsafe {
                            fprintf(__stdoutp, c"%s:".as_ptr() as *mut i8 as *const i8,
                                z_id)
                        };
                        __state = 23;
                    }
                    23 => { i = 0; __state = 25; }
                    24 => {
                        unsafe {
                            fprintf(__stdoutp, c"\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 20;
                    }
                    25 => {
                        if !(unsafe { *az.offset(i as isize) }).is_null() {
                            __state = 26;
                        } else { __state = 24; }
                    }
                    26 => {
                        unsafe {
                            fprintf(__stdoutp, c" %s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 28;
                    }
                    27 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 25;
                    }
                    28 => {
                        if i + 1 == n_pk { __state = 29; } else { __state = 27; }
                    }
                    29 => {
                        unsafe {
                            fprintf(__stdoutp, c" *".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 27;
                    }
                    30 => {
                        if unsafe {
                                    sqlite3_table_column_metadata(g.db,
                                        c"main".as_ptr() as *mut i8 as *const i8, z_tab_1,
                                        core::ptr::null(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                } != 0 {
                            __state = 37;
                        } else { __state = 36; }
                    }
                    31 => {
                        if (unsafe {
                                            sqlite3_table_column_metadata(g.db,
                                                c"main".as_ptr() as *mut i8 as *const i8, z_tab_1,
                                                core::ptr::null(), core::ptr::null_mut(),
                                                core::ptr::null_mut(), core::ptr::null_mut(),
                                                core::ptr::null_mut(), core::ptr::null_mut())
                                        } == 0) as i32 != 0 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => { __state = 2; }
                    33 => {
                        if g.b_schema_compare != 0 {
                            __state = 34;
                        } else { __state = 35; }
                    }
                    34 => {
                        unsafe {
                            fprintf(out,
                                c"-- 2nd DB has no %s table\n".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1)
                        };
                        __state = 32;
                    }
                    35 => {
                        unsafe {
                            fprintf(out,
                                c"DROP TABLE %s;\n".as_ptr() as *mut i8 as *const i8, z_id)
                        };
                        __state = 32;
                    }
                    36 => {
                        az =
                            column_names(c"main".as_ptr() as *mut i8 as *const i8,
                                z_tab_1, &mut n_pk, core::ptr::null_mut());
                        __state = 41;
                    }
                    37 => {
                        if g.b_schema_compare != 0 {
                            __state = 39;
                        } else { __state = 40; }
                    }
                    38 => { __state = 2; }
                    39 => {
                        unsafe {
                            fprintf(out,
                                c"-- 1st DB has no %s table\n".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1)
                        };
                        __state = 38;
                    }
                    40 => { dump_table(z_tab_1, out); __state = 38; }
                    41 => {
                        az2 =
                            column_names(c"aux".as_ptr() as *mut i8 as *const i8,
                                z_tab_1, &mut n_pk2, core::ptr::null_mut());
                        __state = 42;
                    }
                    42 => {
                        if !(az).is_null() && !(az2).is_null() {
                            __state = 44;
                        } else { __state = 43; }
                    }
                    43 => {
                        if az == core::ptr::null_mut() ||
                                        az2 == core::ptr::null_mut() || n_pk != n_pk2 ||
                                !(unsafe { *az.offset(n as isize) }).is_null() {
                            __state = 50;
                        } else { __state = 49; }
                    }
                    44 => { n = 0; __state = 45; }
                    45 => {
                        if !(unsafe { *az.offset(n as isize) }).is_null() &&
                                !(unsafe { *az2.offset(n as isize) }).is_null() {
                            __state = 46;
                        } else { __state = 43; }
                    }
                    46 => {
                        if unsafe {
                                    sqlite3_stricmp(unsafe { *az.offset(n as isize) } as
                                            *const i8, unsafe { *az2.offset(n as isize) } as *const i8)
                                } != 0 {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    47 => {
                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        __state = 45;
                    }
                    48 => { __state = 43; }
                    49 => { n2 = n; __state = 54; }
                    50 => {
                        unsafe {
                            fprintf(out,
                                c"%sDROP TABLE %s; -- due to schema mismatch\n".as_ptr() as
                                        *mut i8 as *const i8, z_lead, z_id)
                        };
                        __state = 51;
                    }
                    51 => { dump_table(z_tab_1, out); __state = 52; }
                    52 => { __state = 2; }
                    53 => { n_q = n_pk2 + 1 + 2 * (n2 - n_pk2); __state = 59; }
                    54 => {
                        if !(unsafe { *az2.offset(n2 as isize) }).is_null() {
                            __state = 55;
                        } else { __state = 53; }
                    }
                    55 => {
                        z_n_tab =
                            safe_id(unsafe { *az2.offset(n2 as isize) } as *const i8);
                        __state = 57;
                    }
                    56 => {
                        { let __p = &mut n2; let __t = *__p; *__p += 1; __t };
                        __state = 54;
                    }
                    57 => {
                        unsafe {
                            fprintf(out,
                                c"ALTER TABLE %s ADD COLUMN %s;\n".as_ptr() as *mut i8 as
                                    *const i8, z_id, z_n_tab)
                        };
                        __state = 58;
                    }
                    58 => {
                        unsafe { sqlite3_free(z_n_tab as *mut ()) };
                        __state = 56;
                    }
                    59 => {
                        if n2 > n_pk2 { __state = 61; } else { __state = 60; }
                    }
                    60 => {
                        z_sep = c"SELECT ".as_ptr() as *mut i8 as *const i8;
                        __state = 91;
                    }
                    61 => {
                        z_sep = c"SELECT ".as_ptr() as *mut i8 as *const i8;
                        __state = 62;
                    }
                    62 => { i = 0; __state = 64; }
                    63 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", 1 /* changed row */".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 68;
                    }
                    64 => {
                        if i < n_pk { __state = 65; } else { __state = 63; }
                    }
                    65 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sB.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 67;
                    }
                    66 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 64;
                    }
                    67 => {
                        z_sep = c", ".as_ptr() as *mut i8 as *const i8;
                        __state = 66;
                    }
                    68 => {
                        if !(unsafe { *az.offset(i as isize) }).is_null() {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        if !(unsafe { *az2.offset(i as isize) }).is_null() {
                            __state = 73;
                        } else { __state = 72; }
                    }
                    70 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", A.%s IS NOT B.%s, B.%s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *az.offset(i as isize) },
                                unsafe { *az2.offset(i as isize) },
                                unsafe { *az2.offset(i as isize) })
                        };
                        __state = 71;
                    }
                    71 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 68;
                    }
                    72 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"\n  FROM main.%s A, aux.%s B\n".as_ptr() as *mut i8 as
                                    *const i8, z_id, z_id)
                        };
                        __state = 75;
                    }
                    73 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", B.%s IS NOT NULL, B.%s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *az2.offset(i as isize) },
                                unsafe { *az2.offset(i as isize) })
                        };
                        __state = 74;
                    }
                    74 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 69;
                    }
                    75 => {
                        z_sep = c" WHERE".as_ptr() as *mut i8 as *const i8;
                        __state = 76;
                    }
                    76 => { i = 0; __state = 78; }
                    77 => {
                        z_sep = c"\n   AND (".as_ptr() as *mut i8 as *const i8;
                        __state = 82;
                    }
                    78 => {
                        if i < n_pk { __state = 79; } else { __state = 77; }
                    }
                    79 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 81;
                    }
                    80 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 78;
                    }
                    81 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 80;
                    }
                    82 => {
                        if !(unsafe { *az.offset(i as isize) }).is_null() {
                            __state = 84;
                        } else { __state = 83; }
                    }
                    83 => {
                        if !(unsafe { *az2.offset(i as isize) }).is_null() {
                            __state = 88;
                        } else { __state = 87; }
                    }
                    84 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sA.%s IS NOT B.%s%s\n".as_ptr() as *mut i8 as *const i8,
                                z_sep, unsafe { *az.offset(i as isize) },
                                unsafe { *az2.offset(i as isize) },
                                if unsafe { *az2.offset((i + 1) as isize) } ==
                                        core::ptr::null_mut() {
                                    c")".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 })
                        };
                        __state = 85;
                    }
                    85 => {
                        z_sep = c"        OR ".as_ptr() as *mut i8 as *const i8;
                        __state = 86;
                    }
                    86 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 82;
                    }
                    87 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" UNION ALL\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 60;
                    }
                    88 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sB.%s IS NOT NULL%s\n".as_ptr() as *mut i8 as *const i8,
                                z_sep, unsafe { *az2.offset(i as isize) },
                                if unsafe { *az2.offset((i + 1) as isize) } ==
                                        core::ptr::null_mut() {
                                    c")".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 })
                        };
                        __state = 89;
                    }
                    89 => {
                        z_sep = c"        OR ".as_ptr() as *mut i8 as *const i8;
                        __state = 90;
                    }
                    90 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 83;
                    }
                    91 => { i = 0; __state = 93; }
                    92 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", 2 /* deleted row */".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 97;
                    }
                    93 => {
                        if i < n_pk { __state = 94; } else { __state = 92; }
                    }
                    94 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sA.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 96;
                    }
                    95 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 93;
                    }
                    96 => {
                        z_sep = c", ".as_ptr() as *mut i8 as *const i8;
                        __state = 95;
                    }
                    97 => {
                        if !(unsafe { *az2.offset(i as isize) }).is_null() {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    98 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"\n  FROM main.%s A\n".as_ptr() as *mut i8 as *const i8,
                                z_id)
                        };
                        __state = 101;
                    }
                    99 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", NULL, NULL".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 100;
                    }
                    100 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 97;
                    }
                    101 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" WHERE NOT EXISTS(SELECT 1 FROM aux.%s B\n".as_ptr() as
                                        *mut i8 as *const i8, z_id)
                        };
                        __state = 102;
                    }
                    102 => {
                        z_sep =
                            c"                   WHERE".as_ptr() as *mut i8 as
                                *const i8;
                        __state = 103;
                    }
                    103 => { i = 0; __state = 105; }
                    104 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 109;
                    }
                    105 => {
                        if i < n_pk { __state = 106; } else { __state = 104; }
                    }
                    106 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 108;
                    }
                    107 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 105;
                    }
                    108 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 107;
                    }
                    109 => {
                        z_sep =
                            c" UNION ALL\nSELECT ".as_ptr() as *mut i8 as *const i8;
                        __state = 110;
                    }
                    110 => { i = 0; __state = 112; }
                    111 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", 3 /* inserted row */".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 116;
                    }
                    112 => {
                        if i < n_pk { __state = 113; } else { __state = 111; }
                    }
                    113 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sB.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 115;
                    }
                    114 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 112;
                    }
                    115 => {
                        z_sep = c", ".as_ptr() as *mut i8 as *const i8;
                        __state = 114;
                    }
                    116 => {
                        if !(unsafe { *az2.offset(i as isize) }).is_null() {
                            __state = 118;
                        } else { __state = 117; }
                    }
                    117 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"\n  FROM aux.%s B\n".as_ptr() as *mut i8 as *const i8,
                                z_id)
                        };
                        __state = 120;
                    }
                    118 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", 1, B.%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az2.offset(i as isize) })
                        };
                        __state = 119;
                    }
                    119 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 116;
                    }
                    120 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" WHERE NOT EXISTS(SELECT 1 FROM main.%s A\n".as_ptr() as
                                        *mut i8 as *const i8, z_id)
                        };
                        __state = 121;
                    }
                    121 => {
                        z_sep =
                            c"                   WHERE".as_ptr() as *mut i8 as
                                *const i8;
                        __state = 122;
                    }
                    122 => { i = 0; __state = 124; }
                    123 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n ORDER BY".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 128;
                    }
                    124 => {
                        if i < n_pk { __state = 125; } else { __state = 123; }
                    }
                    125 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 127;
                    }
                    126 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 124;
                    }
                    127 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 126;
                    }
                    128 => {
                        z_sep = c" ".as_ptr() as *mut i8 as *const i8;
                        __state = 129;
                    }
                    129 => { i = 1; __state = 131; }
                    130 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c";\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 135;
                    }
                    131 => {
                        if i <= n_pk { __state = 132; } else { __state = 130; }
                    }
                    132 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s%d".as_ptr() as *mut i8 as *const i8, z_sep, i)
                        };
                        __state = 134;
                    }
                    133 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 131;
                    }
                    134 => {
                        z_sep = c", ".as_ptr() as *mut i8 as *const i8;
                        __state = 133;
                    }
                    135 => {
                        if g.f_debug & 2 as u32 != 0 {
                            __state = 137;
                        } else { __state = 136; }
                    }
                    136 => {
                        p_stmt =
                            unsafe {
                                db_prepare(c"SELECT name FROM main.sqlite_schema WHERE type=\'index\' AND tbl_name=%Q   AND sql IS NOT NULL   AND sql NOT IN (SELECT sql FROM aux.sqlite_schema                    WHERE type=\'index\' AND tbl_name=%Q                      AND sql IS NOT NULL)".as_ptr()
                                            as *mut i8 as *const i8, z_tab_1, z_tab_1)
                            };
                        __state = 139;
                    }
                    137 => {
                        unsafe {
                            printf(c"SQL for %s:\n%s\n".as_ptr() as *mut i8 as
                                    *const i8, z_id, unsafe { sqlite3_str_value(p_sql) })
                        };
                        __state = 138;
                    }
                    138 => { __state = 2; }
                    139 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 141;
                        } else { __state = 140; }
                    }
                    140 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 144;
                    }
                    141 => {
                        z =
                            safe_id(unsafe { sqlite3_column_text(p_stmt, 0) } as
                                    *const i8);
                        __state = 142;
                    }
                    142 => {
                        unsafe {
                            fprintf(out,
                                c"DROP INDEX %s;\n".as_ptr() as *mut i8 as *const i8, z)
                        };
                        __state = 143;
                    }
                    143 => {
                        unsafe { sqlite3_free(z as *mut ()) };
                        __state = 139;
                    }
                    144 => {
                        if (g.b_schema_only == 0) as i32 != 0 {
                            __state = 146;
                        } else { __state = 145; }
                    }
                    145 => {
                        p_stmt =
                            unsafe {
                                db_prepare(c"SELECT sql FROM aux.sqlite_schema WHERE type=\'index\' AND tbl_name=%Q   AND sql IS NOT NULL   AND sql NOT IN (SELECT sql FROM main.sqlite_schema                    WHERE type=\'index\' AND tbl_name=%Q                      AND sql IS NOT NULL)".as_ptr()
                                            as *mut i8 as *const i8, z_tab_1, z_tab_1)
                            };
                        __state = 190;
                    }
                    146 => {
                        p_stmt =
                            unsafe {
                                db_prepare(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { sqlite3_str_value(p_sql) })
                            };
                        __state = 147;
                    }
                    147 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 149;
                        } else { __state = 148; }
                    }
                    148 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 145;
                    }
                    149 => {
                        i_type = unsafe { sqlite3_column_int(p_stmt, n_pk) };
                        __state = 150;
                    }
                    150 => {
                        if i_type == 1 || i_type == 2 {
                            __state = 151;
                        } else { __state = 152; }
                    }
                    151 => {
                        if i_type == 1 { __state = 154; } else { __state = 155; }
                    }
                    152 => {
                        unsafe {
                            fprintf(out,
                                c"%sINSERT INTO %s(%s".as_ptr() as *mut i8 as *const i8,
                                z_lead, z_id, unsafe { *az2.offset(0 as isize) })
                        };
                        __state = 172;
                    }
                    153 => {
                        z_sep = c" WHERE".as_ptr() as *mut i8 as *const i8;
                        __state = 165;
                    }
                    154 => {
                        unsafe {
                            fprintf(out,
                                c"%sUPDATE %s".as_ptr() as *mut i8 as *const i8, z_lead,
                                z_id)
                        };
                        __state = 156;
                    }
                    155 => {
                        unsafe {
                            fprintf(out,
                                c"%sDELETE FROM %s".as_ptr() as *mut i8 as *const i8,
                                z_lead, z_id)
                        };
                        __state = 153;
                    }
                    156 => {
                        z_sep = c" SET".as_ptr() as *mut i8 as *const i8;
                        __state = 157;
                    }
                    157 => { i = n_pk + 1; __state = 158; }
                    158 => {
                        if i < n_q { __state = 159; } else { __state = 153; }
                    }
                    159 => {
                        if unsafe { sqlite3_column_int(p_stmt, i) } == 0 {
                            __state = 162;
                        } else { __state = 161; }
                    }
                    160 => { i += 2; __state = 158; }
                    161 => {
                        unsafe {
                            fprintf(out, c"%s %s=".as_ptr() as *mut i8 as *const i8,
                                z_sep,
                                unsafe { *az2.offset(((i + n_pk - 1) / 2) as isize) })
                        };
                        __state = 163;
                    }
                    162 => { __state = 160; }
                    163 => {
                        z_sep = c",".as_ptr() as *mut i8 as *const i8;
                        __state = 164;
                    }
                    164 => {
                        print_quoted(out,
                            unsafe { sqlite3_column_value(p_stmt, i + 1) });
                        __state = 160;
                    }
                    165 => { i = 0; __state = 167; }
                    166 => {
                        unsafe {
                            fprintf(out, c";\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 147;
                    }
                    167 => {
                        if i < n_pk { __state = 168; } else { __state = 166; }
                    }
                    168 => {
                        unsafe {
                            fprintf(out, c"%s %s=".as_ptr() as *mut i8 as *const i8,
                                z_sep, unsafe { *az2.offset(i as isize) })
                        };
                        __state = 170;
                    }
                    169 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 167;
                    }
                    170 => {
                        print_quoted(out,
                            unsafe { sqlite3_column_value(p_stmt, i) });
                        __state = 171;
                    }
                    171 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 169;
                    }
                    172 => { i = 1; __state = 174; }
                    173 => {
                        unsafe {
                            fprintf(out, c") VALUES".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 177;
                    }
                    174 => {
                        if !(unsafe { *az2.offset(i as isize) }).is_null() {
                            __state = 175;
                        } else { __state = 173; }
                    }
                    175 => {
                        unsafe {
                            fprintf(out, c",%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az2.offset(i as isize) })
                        };
                        __state = 176;
                    }
                    176 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 174;
                    }
                    177 => {
                        z_sep = c"(".as_ptr() as *mut i8 as *const i8;
                        __state = 178;
                    }
                    178 => { i = 0; __state = 180; }
                    179 => { i = n_pk2 + 2; __state = 186; }
                    180 => {
                        if i < n_pk2 { __state = 181; } else { __state = 179; }
                    }
                    181 => {
                        unsafe {
                            fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8, z_sep)
                        };
                        __state = 183;
                    }
                    182 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 180;
                    }
                    183 => {
                        z_sep = c",".as_ptr() as *mut i8 as *const i8;
                        __state = 184;
                    }
                    184 => {
                        print_quoted(out,
                            unsafe { sqlite3_column_value(p_stmt, i) });
                        __state = 182;
                    }
                    185 => {
                        unsafe {
                            fprintf(out, c");\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 147;
                    }
                    186 => {
                        if i < n_q { __state = 187; } else { __state = 185; }
                    }
                    187 => {
                        unsafe {
                            fprintf(out, c",".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 189;
                    }
                    188 => { i += 2; __state = 186; }
                    189 => {
                        print_quoted(out,
                            unsafe { sqlite3_column_value(p_stmt, i) });
                        __state = 188;
                    }
                    190 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 192;
                        } else { __state = 191; }
                    }
                    191 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 193;
                    }
                    192 => {
                        unsafe {
                            fprintf(out, c"%s;\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_column_text(p_stmt, 0) })
                        };
                        __state = 190;
                    }
                    193 => { __state = 2; }
                    194 => {
                        unsafe { sqlite3_free(z_id as *mut ()) };
                        __state = 195;
                    }
                    195 => { namelist_free(az); __state = 196; }
                    196 => { namelist_free(az2); __state = 197; }
                    197 => { return; }
                    _ => {}
                }
            }
        }
    }
}
extern "C" fn check_schemas_match(z_tab_1: *const i8) -> () {
    let p_stmt: *mut sqlite3_stmt =
        unsafe {
            db_prepare(c"SELECT A.sql=B.sql FROM main.sqlite_schema A, aux.sqlite_schema B WHERE A.name=%Q AND B.name=%Q".as_ptr()
                        as *mut i8 as *const i8, z_tab_1, z_tab_1)
        };
    if 100 == unsafe { sqlite3_step(p_stmt) } {
        if unsafe { sqlite3_column_int(p_stmt, 0) } == 0 {
            unsafe {
                runtime_error(c"schema changes for table %s".as_ptr() as
                            *mut i8 as *const i8, safe_id(z_tab_1))
            };
        }
    } else {
        unsafe {
            runtime_error(c"table %s missing from one or both databases".as_ptr()
                        as *mut i8 as *const i8, safe_id(z_tab_1))
        };
    }
    unsafe { sqlite3_finalize(p_stmt) };
}
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
    a = { b = 0 as u16; b };
    {
        i = 0 as u16;
        '__b20: loop {
            if !((i as i32) < 16) { break '__b20; }
            '__c20: loop {
                a += unsafe { *z.add(i as usize) } as i32 as u16;
                b +=
                    ((16 - i as i32) * unsafe { *z.add(i as usize) } as i32) as
                        u16;
                (*p_hash_1).z[i as usize] =
                    unsafe { *z.add(i as usize) } as i8;
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_hash_1).a = (a as i32 & 65535) as u16;
    (*p_hash_1).b = (b as i32 & 65535) as u16;
    (*p_hash_1).i = 0 as u16;
}
extern "C" fn hash_next(p_hash_1: &mut hash, c: i32) -> () {
    let old: u16 = (*p_hash_1).z[(*p_hash_1).i as usize] as u16;
    (*p_hash_1).z[(*p_hash_1).i as usize] = c as i8;
    (*p_hash_1).i = ((*p_hash_1).i as i32 + 1 & 16 - 1) as u16;
    (*p_hash_1).a =
        ((*p_hash_1).a as i32 - old as i32 + c as i8 as i32) as u16;
    (*p_hash_1).b =
        ((*p_hash_1).b as i32 - 16 * old as i32 + (*p_hash_1).a as i32) as
            u16;
}
extern "C" fn hash_32bit(p_hash_1: &hash) -> u32 {
    return ((*p_hash_1).a as i32 & 65535) as u32 |
            (((*p_hash_1).b as i32 & 65535) as u32) << 16;
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
        '__b21: loop {
            if !(v > 0 as u32) { break '__b21; }
            '__c21: loop {
                z_buf[i as usize] = z_digits[(v & 63 as u32) as usize] as i8;
                break '__c21;
            }
            {
                ({ let __p = &mut i; let __t = *__p; *__p += 1; __t }) as u32;
                v >>= 6 as u32
            };
        }
    }
    {
        j = i - 1;
        '__b22: loop {
            if !(j >= 0) { break '__b22; }
            '__c22: loop {
                unsafe {
                    *{
                                let __p = &mut *pz;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = z_buf[j as usize]
                };
                break '__c22;
            }
            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
        }
    }
}
extern "C" fn digit_count(v: i32) -> i32 {
    let mut i: u32 = 0 as u32;
    let mut x: u32 = 0 as u32;
    {
        { i = 1 as u32; x = 64 as u32 };
        '__b23: loop {
            if !(v as u32 >= x) { break '__b23; }
            '__c23: loop { break '__c23; }
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
    let mut sum0: u32 = 0 as u32;
    let mut sum1: u32 = 0 as u32;
    let mut sum2: u32 = 0 as u32;
    let mut sum3: u32 = 0 as u32;
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
        sum3 +=
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
        sum3 += unsafe { *z.offset(3 as isize) } as u32;
        {
            let __n = 4;
            let __p = &mut z;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        n_1 -= 4 as u64;
    }
    sum3 += (sum2 << 8) + (sum1 << 16) + (sum0 << 24);
    '__s26:
        {
        match n_1 {
            3 => {
                sum3 +=
                    ((unsafe { *z.offset(2 as isize) } as i32) << 8) as u32;
                sum3 +=
                    ((unsafe { *z.offset(1 as isize) } as i32) << 16) as u32;
                sum3 +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            2 => {
                sum3 +=
                    ((unsafe { *z.offset(1 as isize) } as i32) << 16) as u32;
                sum3 +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            1 => {
                sum3 +=
                    ((unsafe { *z.offset(0 as isize) } as i32) << 24) as u32;
            }
            _ => {}
        }
    }
    return sum3;
}
extern "C" fn rbu_delta_create(z_src_1: *const i8, len_src_1: u32,
    z_out_1: *const i8, len_out_1: u32, mut z_delta_1: *mut i8) -> i32 {
    let mut i: u32 = 0 as u32;
    let mut base: u32 = 0 as u32;
    let z_orig_delta: *const i8 = z_delta_1 as *const i8;
    let mut h: hash = unsafe { core::mem::zeroed() };
    let mut n_hash: i64 = 0 as i64;
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
    n_hash = (len_src_1 / 16 as u32) as i64;
    collide =
        unsafe {
                sqlite3_malloc64((n_hash * 2 as i64) as u64 *
                        core::mem::size_of::<i32>() as u64)
            } as *mut i32;
    if collide == core::ptr::null_mut() { return 0; }
    landmark = unsafe { collide.offset(n_hash as isize) };
    unsafe {
        memset(landmark as *mut (), -1,
            n_hash as u64 * core::mem::size_of::<i32>() as u64)
    };
    unsafe {
        memset(collide as *mut (), -1,
            n_hash as u64 * core::mem::size_of::<i32>() as u64)
    };
    {
        i = 0 as u32;
        '__b27: loop {
            if !(i < len_src_1 - 16 as u32) { break '__b27; }
            '__c27: loop {
                let mut hv: i32 = 0;
                hash_init(&mut h, unsafe { &*z_src_1.add(i as usize) });
                hv = (hash_32bit(&h) as i64 % n_hash) as i32;
                unsafe {
                    *collide.add((i / 16 as u32) as usize) =
                        unsafe { *landmark.offset(hv as isize) }
                };
                unsafe {
                    *landmark.offset(hv as isize) = (i / 16 as u32) as i32
                };
                break '__c27;
            }
            i += 16 as u32;
        }
    }
    base = 0 as u32;
    while (base + 16 as u32) < len_out_1 {
        let mut i_src: i32 = 0;
        let mut i_block: i32 = 0;
        let mut best_cnt: i32 = 0;
        let mut best_ofst: i32 = 0;
        let mut best_litsz: i32 = 0;
        hash_init(&mut h, unsafe { &*z_out_1.add(base as usize) });
        i = 0 as u32;
        best_cnt = 0;
        loop {
            let mut hv: i32 = 0;
            let mut limit: i32 = 250;
            hv = (hash_32bit(&h) as i64 % n_hash) as i32;
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
                i_src = i_block * 16;
                {
                    { { j = 0; x = i_src }; y = (base + i) as i32 };
                    '__b31: loop {
                        if !((x as u32) < len_src_1 && (y as u32) < len_out_1) {
                            break '__b31;
                        }
                        '__c31: loop {
                            if unsafe { *z_src_1.offset(x as isize) } as i32 !=
                                    unsafe { *z_out_1.offset(y as isize) } as i32 {
                                break '__b31;
                            }
                            break '__c31;
                        }
                        {
                            {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                { let __p = &mut x; let __t = *__p; *__p += 1; __t }
                            };
                            { let __p = &mut y; let __t = *__p; *__p += 1; __t }
                        };
                    }
                }
                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                {
                    k = 1;
                    '__b32: loop {
                        if !(k < i_src && k as u32 <= i) { break '__b32; }
                        '__c32: loop {
                            if unsafe { *z_src_1.offset((i_src - k) as isize) } as i32
                                    !=
                                    unsafe { *z_out_1.add((base + i - k as u32) as usize) } as
                                        i32 {
                                break '__b32;
                            }
                            break '__c32;
                        }
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    }
                }
                { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                ofst = i_src - k;
                cnt = j + k + 1;
                litsz = (i - k as u32) as i32;
                sz =
                    digit_count((i - k as u32) as i32) + digit_count(cnt) +
                            digit_count(ofst) + 3;
                if cnt >= sz && cnt > best_cnt {
                    best_cnt = cnt;
                    best_ofst = i_src - k;
                    best_litsz = litsz;
                }
                i_block = unsafe { *collide.offset(i_block as isize) };
            }
            if best_cnt > 0 {
                if best_litsz > 0 {
                    put_int(best_litsz as u32, &mut z_delta_1);
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
                            unsafe { &raw const *z_out_1.add(base as usize) } as
                                *const (), best_litsz as u64)
                    };
                    {
                        let __n = best_litsz;
                        let __p = &mut z_delta_1;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    base += best_litsz as u32;
                }
                base += best_cnt as u32;
                put_int(best_cnt as u32, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = '@' as i32 as i8
                };
                put_int(best_ofst as u32, &mut z_delta_1);
                unsafe {
                    *{
                                let __p = &mut z_delta_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = ',' as i32 as i8
                };
                if best_ofst + best_cnt - 1 > last_read {
                    last_read = best_ofst + best_cnt - 1;
                }
                best_cnt = 0;
                break;
            }
            if base + i + 16 as u32 >= len_out_1 {
                put_int(len_out_1 - base, &mut z_delta_1);
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
                        unsafe { &raw const *z_out_1.add(base as usize) } as
                            *const (), (len_out_1 - base) as u64)
                };
                {
                    let __n = len_out_1 - base;
                    let __p = &mut z_delta_1;
                    *__p = unsafe { (*__p).add(__n as usize) };
                };
                base = len_out_1;
                break;
            }
            hash_next(&mut h,
                unsafe { *z_out_1.add((base + i + 16 as u32) as usize) } as
                    i32);
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if base < len_out_1 {
        put_int(len_out_1 - base, &mut z_delta_1);
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
                unsafe { &raw const *z_out_1.add(base as usize) } as
                    *const (), (len_out_1 - base) as u64)
        };
        {
            let __n = len_out_1 - base;
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
extern "C" fn str_printf_array(p_str_1: *mut sqlite3_str, z_sep_1: *const i8,
    z_fmt_1: *const i8, az: &[*mut i8]) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b33: loop {
            if !(!(az[i as usize]).is_null() &&
                            (i < az.len() as i32 || (az.len() as i32) < 0)) {
                break '__b33;
            }
            '__c33: loop {
                if i != 0 {
                    unsafe {
                        sqlite3_str_appendf(p_str_1,
                            c"%s".as_ptr() as *mut i8 as *const i8, z_sep_1)
                    };
                }
                unsafe {
                    sqlite3_str_appendf(p_str_1, z_fmt_1, az[i as usize],
                        az[i as usize], az[i as usize])
                };
                break '__c33;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn get_rbudiff_query(z_tab_1: *const i8, az_col_1: *mut *mut i8,
    n_pk_1: i32, b_ota_rowid_1: i32, p_sql_1: *mut sqlite3_str) -> () {
    let mut i: i32 = 0;
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"SELECT ".as_ptr() as *mut i8 as *const i8)
    };
    str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
        c"%s".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
        });
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c", 0, ".as_ptr() as *mut i8 as *const i8)
    };
    str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
        c"NULL".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
        });
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c" FROM aux.%Q AS n WHERE NOT EXISTS (\n".as_ptr() as *mut i8 as
                *const i8, z_tab_1)
    };
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"    SELECT 1 FROM ".as_ptr() as *mut i8 as *const i8, z_tab_1)
    };
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c" main.%Q AS o WHERE ".as_ptr() as *mut i8 as *const i8, z_tab_1)
    };
    str_printf_array(p_sql_1, c" AND ".as_ptr() as *mut i8 as *const i8,
        c"(n.%Q = o.%Q)".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
        });
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"\n) AND ".as_ptr() as *mut i8 as *const i8)
    };
    str_printf_array(p_sql_1, c" AND ".as_ptr() as *mut i8 as *const i8,
        c"(n.%Q IS NOT NULL)".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
        });
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"\nUNION ALL\nSELECT ".as_ptr() as *mut i8 as *const i8)
    };
    str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
        c"%s".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
        });
    if !(unsafe { *az_col_1.offset(n_pk_1 as isize) }).is_null() {
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c", ".as_ptr() as *mut i8 as *const i8)
        };
        str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
            c"NULL".as_ptr() as *mut i8 as *const i8,
            unsafe {
                let __p =
                    unsafe { &mut *az_col_1.offset(n_pk_1 as isize) } as
                        *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
            });
    }
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c", 1, ".as_ptr() as *mut i8 as *const i8)
    };
    str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
        c"NULL".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
        });
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c" FROM main.%Q AS n WHERE NOT EXISTS (\n".as_ptr() as *mut i8 as
                *const i8, z_tab_1)
    };
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"    SELECT 1 FROM ".as_ptr() as *mut i8 as *const i8, z_tab_1)
    };
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c" aux.%Q AS o WHERE ".as_ptr() as *mut i8 as *const i8, z_tab_1)
    };
    str_printf_array(p_sql_1, c" AND ".as_ptr() as *mut i8 as *const i8,
        c"(n.%Q = o.%Q)".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
        });
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"\n) AND ".as_ptr() as *mut i8 as *const i8)
    };
    str_printf_array(p_sql_1, c" AND ".as_ptr() as *mut i8 as *const i8,
        c"(n.%Q IS NOT NULL)".as_ptr() as *mut i8 as *const i8,
        unsafe {
            let __p = az_col_1 as *const *mut i8;
            if __p.is_null() {
                &[]
            } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
        });
    if !(unsafe { *az_col_1.offset(n_pk_1 as isize) }).is_null() {
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c"\nUNION ALL\nSELECT ".as_ptr() as *mut i8 as *const i8)
        };
        str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
            c"n.%s".as_ptr() as *mut i8 as *const i8,
            unsafe {
                let __p = az_col_1 as *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c",\n".as_ptr() as *mut i8 as *const i8)
        };
        str_printf_array(p_sql_1, c" ,\n".as_ptr() as *mut i8 as *const i8,
            c"    CASE WHEN n.%s IS o.%s THEN NULL ELSE n.%s END".as_ptr() as
                    *mut i8 as *const i8,
            unsafe {
                let __p =
                    unsafe { &mut *az_col_1.offset(n_pk_1 as isize) } as
                        *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
            });
        if b_ota_rowid_1 == 0 {
            unsafe {
                sqlite3_str_appendf(p_sql_1,
                    c", \'".as_ptr() as *mut i8 as *const i8)
            };
            str_printf_array(p_sql_1, c"".as_ptr() as *mut i8 as *const i8,
                c".".as_ptr() as *mut i8 as *const i8,
                unsafe {
                    let __p = az_col_1 as *const *mut i8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
                });
            unsafe {
                sqlite3_str_appendf(p_sql_1,
                    c"\' ||\n".as_ptr() as *mut i8 as *const i8)
            };
        } else {
            unsafe {
                sqlite3_str_appendf(p_sql_1,
                    c",\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        str_printf_array(p_sql_1, c" ||\n".as_ptr() as *mut i8 as *const i8,
            c"    CASE WHEN n.%s IS o.%s THEN \'.\' ELSE \'x\' END".as_ptr()
                    as *mut i8 as *const i8,
            unsafe {
                let __p =
                    unsafe { &mut *az_col_1.offset(n_pk_1 as isize) } as
                        *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c"\nAS ota_control, ".as_ptr() as *mut i8 as *const i8)
        };
        str_printf_array(p_sql_1, c", ".as_ptr() as *mut i8 as *const i8,
            c"NULL".as_ptr() as *mut i8 as *const i8,
            unsafe {
                let __p = az_col_1 as *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c",\n".as_ptr() as *mut i8 as *const i8)
        };
        str_printf_array(p_sql_1, c" ,\n".as_ptr() as *mut i8 as *const i8,
            c"    CASE WHEN n.%s IS o.%s THEN NULL ELSE o.%s END".as_ptr() as
                    *mut i8 as *const i8,
            unsafe {
                let __p =
                    unsafe { &mut *az_col_1.offset(n_pk_1 as isize) } as
                        *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c"\nFROM main.%Q AS o, aux.%Q AS n\nWHERE ".as_ptr() as
                        *mut i8 as *const i8, z_tab_1, z_tab_1)
        };
        str_printf_array(p_sql_1, c" AND ".as_ptr() as *mut i8 as *const i8,
            c"(n.%Q = o.%Q)".as_ptr() as *mut i8 as *const i8,
            unsafe {
                let __p = az_col_1 as *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_pk_1 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_sql_1,
                c" AND ota_control LIKE \'%%x%%\'".as_ptr() as *mut i8 as
                    *const i8)
        };
    }
    unsafe {
        sqlite3_str_appendf(p_sql_1,
            c"\nORDER BY ".as_ptr() as *mut i8 as *const i8)
    };
    {
        i = 1;
        '__b34: loop {
            if !(i <= n_pk_1) { break '__b34; }
            '__c34: loop {
                unsafe {
                    sqlite3_str_appendf(p_sql_1,
                        c"%s%d".as_ptr() as *mut i8 as *const i8,
                        if i > 1 {
                            c", ".as_ptr() as *mut i8
                        } else { c"".as_ptr() as *mut i8 }, i)
                };
                break '__c34;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn rbudiff_one_table(z_tab_1: *const i8, out: *mut FILE) -> () {
    unsafe {
        let mut b_ota_rowid: i32 = 0;
        let mut n_pk: i32 = 0;
        let mut az_col: *mut *mut i8 = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut n_col: i32 = 0;
        let mut p_ct: *mut sqlite3_str = core::ptr::null_mut();
        let mut p_sql: *mut sqlite3_str = core::ptr::null_mut();
        let mut p_insert: *mut sqlite3_str = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut n_row: i32 = 0;
        g.b_schema_pk = 1;
        p_ct = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
        p_sql = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
        p_insert = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
        check_schemas_match(z_tab_1);
        az_col =
            column_names(c"main".as_ptr() as *mut i8 as *const i8, z_tab_1,
                &mut n_pk, &mut b_ota_rowid);
        if az_col == core::ptr::null_mut() {
            unsafe {
                runtime_error(c"table %s has no usable PK columns".as_ptr() as
                            *mut i8 as *const i8, z_tab_1)
            };
        }
        {
            n_col = 0;
            '__b35: loop {
                if !(!(unsafe { *az_col.offset(n_col as isize) }).is_null()) {
                    break '__b35;
                }
                '__c35: loop { break '__c35; }
                { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_str_appendf(p_ct,
                c"CREATE TABLE IF NOT EXISTS \'data_%q\'(".as_ptr() as *mut i8
                    as *const i8, z_tab_1)
        };
        if b_ota_rowid != 0 {
            unsafe {
                sqlite3_str_appendf(p_ct,
                    c"rbu_rowid, ".as_ptr() as *mut i8 as *const i8)
            };
        }
        str_printf_array(p_ct, c", ".as_ptr() as *mut i8 as *const i8,
            c"%s".as_ptr() as *mut i8 as *const i8,
            unsafe {
                let __p =
                    unsafe { &mut *az_col.offset(b_ota_rowid as isize) } as
                        *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_ct,
                c", rbu_control);".as_ptr() as *mut i8 as *const i8)
        };
        get_rbudiff_query(z_tab_1, az_col, n_pk, b_ota_rowid, p_sql);
        unsafe {
            sqlite3_str_appendf(p_insert,
                c"INSERT INTO \'data_%q\' (".as_ptr() as *mut i8 as *const i8,
                z_tab_1)
        };
        if b_ota_rowid != 0 {
            unsafe {
                sqlite3_str_appendf(p_insert,
                    c"rbu_rowid, ".as_ptr() as *mut i8 as *const i8)
            };
        }
        str_printf_array(p_insert, c", ".as_ptr() as *mut i8 as *const i8,
            c"%s".as_ptr() as *mut i8 as *const i8,
            unsafe {
                let __p =
                    unsafe { &mut *az_col.offset(b_ota_rowid as isize) } as
                        *const *mut i8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, -1i32 as usize) }
            });
        unsafe {
            sqlite3_str_appendf(p_insert,
                c", rbu_control) VALUES(".as_ptr() as *mut i8 as *const i8)
        };
        p_stmt =
            unsafe {
                db_prepare(c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_str_value(p_sql) })
            };
        while unsafe { sqlite3_step(p_stmt) } == 100 {
            if unsafe { sqlite3_str_length(p_ct) } != 0 {
                unsafe {
                    fprintf(out, c"%s\n".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_str_value(p_ct) })
                };
                unsafe { sqlite3_str_reset(p_ct) };
            }
            unsafe {
                fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_str_value(p_insert) })
            };
            { let __p = &mut n_row; let __t = *__p; *__p += 1; __t };
            if unsafe { sqlite3_column_type(p_stmt, n_col) } == 1 {
                {
                    i = 0;
                    '__b37: loop {
                        if !(i <= n_col) { break '__b37; }
                        '__c37: loop {
                            if i > 0 {
                                unsafe {
                                    fprintf(out, c", ".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            print_quoted(out,
                                unsafe { sqlite3_column_value(p_stmt, i) });
                            break '__c37;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            } else {
                let mut z_ota_control: *mut i8 = core::ptr::null_mut();
                let n_ota_control: i64 =
                    unsafe { sqlite3_column_bytes(p_stmt, n_col) } as i64;
                z_ota_control =
                    unsafe {
                            sqlite3_malloc64((n_ota_control + 1 as i64) as
                                    sqlite3_uint64)
                        } as *mut i8;
                unsafe {
                    memcpy(z_ota_control as *mut (),
                        unsafe { sqlite3_column_text(p_stmt, n_col) } as *const (),
                        (n_ota_control + 1 as i64) as u64)
                };
                {
                    i = 0;
                    '__b38: loop {
                        if !(i < n_col) { break '__b38; }
                        '__c38: loop {
                            let mut b_done: i32 = 0;
                            if i >= n_pk &&
                                        unsafe { sqlite3_column_type(p_stmt, i) } == 4 &&
                                    unsafe { sqlite3_column_type(p_stmt, n_col + 1 + i) } == 4 {
                                let a_src: *const i8 =
                                    unsafe { sqlite3_column_blob(p_stmt, n_col + 1 + i) } as
                                        *const i8;
                                let n_src: i32 =
                                    unsafe { sqlite3_column_bytes(p_stmt, n_col + 1 + i) };
                                let a_final: *const i8 =
                                    unsafe { sqlite3_column_blob(p_stmt, i) } as *const i8;
                                let n_final: i64 =
                                    unsafe { sqlite3_column_bytes(p_stmt, i) } as i64;
                                let mut a_delta: *mut i8 = core::ptr::null_mut();
                                let mut n_delta: i32 = 0;
                                a_delta =
                                    unsafe {
                                            sqlite3_malloc64((n_final + 60 as i64) as sqlite3_uint64)
                                        } as *mut i8;
                                n_delta =
                                    rbu_delta_create(a_src, n_src as u32, a_final,
                                        n_final as u32, a_delta);
                                if n_delta > 0 && (n_delta as i64) < n_final {
                                    let mut j: i32 = 0;
                                    unsafe {
                                        fprintf(out, c"x\'".as_ptr() as *mut i8 as *const i8)
                                    };
                                    {
                                        j = 0;
                                        '__b39: loop {
                                            if !(j < n_delta) { break '__b39; }
                                            '__c39: loop {
                                                unsafe {
                                                    fprintf(out, c"%02x".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { *a_delta.offset(j as isize) } as u8 as i32)
                                                };
                                                break '__c39;
                                            }
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    unsafe {
                                        fprintf(out, c"\'".as_ptr() as *mut i8 as *const i8)
                                    };
                                    unsafe {
                                        *z_ota_control.offset((i - b_ota_rowid) as isize) =
                                            'f' as i32 as i8
                                    };
                                    b_done = 1;
                                }
                                unsafe { sqlite3_free(a_delta as *mut ()) };
                            }
                            if b_done == 0 {
                                print_quoted(out,
                                    unsafe { sqlite3_column_value(p_stmt, i) });
                            }
                            unsafe {
                                fprintf(out, c", ".as_ptr() as *mut i8 as *const i8)
                            };
                            break '__c38;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    fprintf(out, c"\'%s\'".as_ptr() as *mut i8 as *const i8,
                        z_ota_control)
                };
                unsafe { sqlite3_free(z_ota_control as *mut ()) };
            }
            unsafe { fprintf(out, c");\n".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        if n_row > 0 {
            let p_cnt: *mut sqlite3_str =
                unsafe { sqlite3_str_new(core::ptr::null_mut()) };
            unsafe {
                sqlite3_str_appendf(p_cnt,
                    c"INSERT INTO rbu_count VALUES(\'data_%q\', %d);".as_ptr()
                            as *mut i8 as *const i8, z_tab_1, n_row)
            };
            unsafe {
                fprintf(out, c"%s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_str_value(p_cnt) })
            };
            str_free(p_cnt);
        }
        str_free(p_ct);
        str_free(p_sql);
        str_free(p_insert);
    }
}
extern "C" fn summarize_one_table(z_tab_1: *const i8, out: *mut FILE) -> () {
    unsafe {
        let mut z_id: *mut i8 = core::ptr::null_mut();
        let mut az: *mut *mut i8 = core::ptr::null_mut();
        let mut az2: *mut *mut i8 = core::ptr::null_mut();
        let mut n_pk: i32 = 0;
        let mut n_pk2: i32 = 0;
        let mut n: i32 = 0;
        let mut n2: i32 = 0;
        let mut i: i32 = 0;
        let mut z_sep: *const i8 = core::ptr::null();
        let mut p_sql: *mut sqlite3_str = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut n_update: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_unchanged: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_delete: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_insert: sqlite3_int64 = 0 as sqlite3_int64;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s41:
                {
                match __state {
                    0 => { z_id = safe_id(z_tab_1); __state = 3; }
                    2 => { str_free(p_sql); __state = 102; }
                    3 => { az = core::ptr::null_mut(); __state = 4; }
                    4 => { az2 = core::ptr::null_mut(); __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { n = 0; __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => {
                        p_sql = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        __state = 18;
                    }
                    18 => {
                        if unsafe {
                                    sqlite3_table_column_metadata(g.db,
                                        c"aux".as_ptr() as *mut i8 as *const i8, z_tab_1,
                                        core::ptr::null(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                } != 0 {
                            __state = 20;
                        } else { __state = 19; }
                    }
                    19 => {
                        if unsafe {
                                    sqlite3_table_column_metadata(g.db,
                                        c"main".as_ptr() as *mut i8 as *const i8, z_tab_1,
                                        core::ptr::null(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut())
                                } != 0 {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    20 => {
                        if (unsafe {
                                            sqlite3_table_column_metadata(g.db,
                                                c"main".as_ptr() as *mut i8 as *const i8, z_tab_1,
                                                core::ptr::null(), core::ptr::null_mut(),
                                                core::ptr::null_mut(), core::ptr::null_mut(),
                                                core::ptr::null_mut(), core::ptr::null_mut())
                                        } == 0) as i32 != 0 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => { __state = 2; }
                    22 => {
                        unsafe {
                            fprintf(out,
                                c"%s: missing from second database\n".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1)
                        };
                        __state = 21;
                    }
                    23 => {
                        az =
                            column_names(c"main".as_ptr() as *mut i8 as *const i8,
                                z_tab_1, &mut n_pk, core::ptr::null_mut());
                        __state = 26;
                    }
                    24 => {
                        unsafe {
                            fprintf(out,
                                c"%s: missing from first database\n".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1)
                        };
                        __state = 25;
                    }
                    25 => { __state = 2; }
                    26 => {
                        az2 =
                            column_names(c"aux".as_ptr() as *mut i8 as *const i8,
                                z_tab_1, &mut n_pk2, core::ptr::null_mut());
                        __state = 27;
                    }
                    27 => {
                        if !(az).is_null() && !(az2).is_null() {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    28 => {
                        if az == core::ptr::null_mut() ||
                                        az2 == core::ptr::null_mut() || n_pk != n_pk2 ||
                                !(unsafe { *az.offset(n as isize) }).is_null() {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    29 => { n = 0; __state = 30; }
                    30 => {
                        if !(unsafe { *az.offset(n as isize) }).is_null() {
                            __state = 31;
                        } else { __state = 28; }
                    }
                    31 => {
                        if unsafe {
                                    sqlite3_stricmp(unsafe { *az.offset(n as isize) } as
                                            *const i8, unsafe { *az2.offset(n as isize) } as *const i8)
                                } != 0 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        __state = 30;
                    }
                    33 => { __state = 28; }
                    34 => { n2 = n; __state = 38; }
                    35 => {
                        unsafe {
                            fprintf(out,
                                c"%s: incompatible schema\n".as_ptr() as *mut i8 as
                                    *const i8, z_tab_1)
                        };
                        __state = 36;
                    }
                    36 => { __state = 2; }
                    37 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"SELECT 1, count(*)".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 41;
                    }
                    38 => {
                        if !(unsafe { *az.offset(n2 as isize) }).is_null() {
                            __state = 39;
                        } else { __state = 37; }
                    }
                    39 => { __state = 40; }
                    40 => {
                        { let __p = &mut n2; let __t = *__p; *__p += 1; __t };
                        __state = 38;
                    }
                    41 => {
                        if n2 == n_pk2 { __state = 43; } else { __state = 44; }
                    }
                    42 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"  FROM main.%s A, aux.%s B\n".as_ptr() as *mut i8 as
                                    *const i8, z_id, z_id)
                        };
                        __state = 51;
                    }
                    43 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c", 0\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 42;
                    }
                    44 => {
                        z_sep = c", sum(".as_ptr() as *mut i8 as *const i8;
                        __state = 45;
                    }
                    45 => { i = n_pk; __state = 47; }
                    46 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 42;
                    }
                    47 => {
                        if !(unsafe { *az.offset(i as isize) }).is_null() {
                            __state = 48;
                        } else { __state = 46; }
                    }
                    48 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sA.%s IS NOT B.%s".as_ptr() as *mut i8 as *const i8,
                                z_sep, unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 50;
                    }
                    49 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 47;
                    }
                    50 => {
                        z_sep = c" OR ".as_ptr() as *mut i8 as *const i8;
                        __state = 49;
                    }
                    51 => {
                        z_sep = c" WHERE".as_ptr() as *mut i8 as *const i8;
                        __state = 52;
                    }
                    52 => { i = 0; __state = 54; }
                    53 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" UNION ALL\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 58;
                    }
                    54 => {
                        if i < n_pk { __state = 55; } else { __state = 53; }
                    }
                    55 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 57;
                    }
                    56 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 54;
                    }
                    57 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 56;
                    }
                    58 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"SELECT 2, count(*), 0\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 59;
                    }
                    59 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"  FROM main.%s A\n".as_ptr() as *mut i8 as *const i8,
                                z_id)
                        };
                        __state = 60;
                    }
                    60 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" WHERE NOT EXISTS(SELECT 1 FROM aux.%s B ".as_ptr() as
                                        *mut i8 as *const i8, z_id)
                        };
                        __state = 61;
                    }
                    61 => {
                        z_sep = c"WHERE".as_ptr() as *mut i8 as *const i8;
                        __state = 62;
                    }
                    62 => { i = 0; __state = 64; }
                    63 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 68;
                    }
                    64 => {
                        if i < n_pk { __state = 65; } else { __state = 63; }
                    }
                    65 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 67;
                    }
                    66 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 64;
                    }
                    67 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 66;
                    }
                    68 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" UNION ALL\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 69;
                    }
                    69 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"SELECT 3, count(*), 0\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 70;
                    }
                    70 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"  FROM aux.%s B\n".as_ptr() as *mut i8 as *const i8, z_id)
                        };
                        __state = 71;
                    }
                    71 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" WHERE NOT EXISTS(SELECT 1 FROM main.%s A ".as_ptr() as
                                        *mut i8 as *const i8, z_id)
                        };
                        __state = 72;
                    }
                    72 => {
                        z_sep = c"WHERE".as_ptr() as *mut i8 as *const i8;
                        __state = 73;
                    }
                    73 => { i = 0; __state = 75; }
                    74 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n ORDER BY 1;\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 79;
                    }
                    75 => {
                        if i < n_pk { __state = 76; } else { __state = 74; }
                    }
                    76 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *az.offset(i as isize) },
                                unsafe { *az.offset(i as isize) })
                        };
                        __state = 78;
                    }
                    77 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 75;
                    }
                    78 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 77;
                    }
                    79 => {
                        if g.f_debug & 2 as u32 != 0 as u32 {
                            __state = 81;
                        } else { __state = 80; }
                    }
                    80 => {
                        p_stmt =
                            unsafe {
                                db_prepare(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { sqlite3_str_value(p_sql) })
                            };
                        __state = 83;
                    }
                    81 => {
                        unsafe {
                            fprintf(__stdoutp,
                                c"SQL for %s:\n%s\n".as_ptr() as *mut i8 as *const i8, z_id,
                                unsafe { sqlite3_str_value(p_sql) })
                        };
                        __state = 82;
                    }
                    82 => { __state = 2; }
                    83 => { n_update = 0 as sqlite3_int64; __state = 84; }
                    84 => { n_insert = 0 as sqlite3_int64; __state = 85; }
                    85 => { n_delete = 0 as sqlite3_int64; __state = 86; }
                    86 => { n_unchanged = 0 as sqlite3_int64; __state = 87; }
                    87 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 89;
                        } else { __state = 88; }
                    }
                    88 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 100;
                    }
                    89 => {
                        '__s42:
                            {
                            match unsafe { sqlite3_column_int(p_stmt, 0) } {
                                1 => { __state = 90; }
                                2 => { __state = 91; }
                                3 => { __state = 92; }
                                _ => { __state = 87; }
                            }
                        }
                    }
                    90 => {
                        n_update = unsafe { sqlite3_column_int64(p_stmt, 2) };
                        __state = 94;
                    }
                    91 => {
                        n_delete = unsafe { sqlite3_column_int64(p_stmt, 1) };
                        __state = 97;
                    }
                    92 => {
                        n_insert = unsafe { sqlite3_column_int64(p_stmt, 1) };
                        __state = 99;
                    }
                    93 => { __state = 90; }
                    94 => {
                        n_unchanged =
                            unsafe { sqlite3_column_int64(p_stmt, 1) } - n_update;
                        __state = 95;
                    }
                    95 => { __state = 87; }
                    96 => { __state = 91; }
                    97 => { __state = 87; }
                    98 => { __state = 92; }
                    99 => { __state = 87; }
                    100 => {
                        unsafe {
                            fprintf(out,
                                c"%s: %lld changes, %lld inserts, %lld deletes, %lld unchanged\n".as_ptr()
                                        as *mut i8 as *const i8, z_tab_1, n_update, n_insert,
                                n_delete, n_unchanged)
                        };
                        __state = 101;
                    }
                    101 => { __state = 2; }
                    102 => {
                        unsafe { sqlite3_free(z_id as *mut ()) };
                        __state = 103;
                    }
                    103 => { namelist_free(az); __state = 104; }
                    104 => { namelist_free(az2); __state = 105; }
                    105 => { return; }
                    _ => {}
                }
            }
        }
    }
}
extern "C" fn puts_varint(out: *mut FILE, mut v: sqlite3_uint64) -> () {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut p: [u8; 12] = [0; 12];
    if v & (4278190080u32 as sqlite3_uint64) << 32 != 0 {
        p[8 as usize] = v as u8;
        v >>= 8 as sqlite3_uint64;
        {
            i = 7;
            '__b43: loop {
                if !(i >= 0) { break '__b43; }
                '__c43: loop {
                    p[i as usize] =
                        (v & 127 as sqlite3_uint64 | 128 as sqlite3_uint64) as u8;
                    v >>= 7 as sqlite3_uint64;
                    break '__c43;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        unsafe {
            fwrite(&raw mut p[0 as usize] as *mut u8 as *const (), 8 as u64,
                1 as u64, out)
        };
    } else {
        n = 9;
        '__b44: loop {
            '__c44: loop {
                p[{ let __p = &mut n; let __t = *__p; *__p -= 1; __t } as
                            usize] =
                    (v & 127 as sqlite3_uint64 | 128 as sqlite3_uint64) as u8;
                v >>= 7 as sqlite3_uint64;
                break '__c44;
            }
            if !(v != 0 as u64) { break '__b44; }
        }
        p[9 as usize] &= 127 as u8;
        unsafe {
            fwrite(unsafe {
                        unsafe {
                            (&raw mut p[0 as usize] as
                                        *mut u8).offset(n as isize).offset(1 as isize)
                        }
                    } as *const (), (9 - n) as u64, 1 as u64, out)
        };
    }
}
extern "C" fn put_value(out: *mut FILE, p_stmt_1: *mut sqlite3_stmt, k: i32)
    -> () {
    let i_d_type: i32 = unsafe { sqlite3_column_type(p_stmt_1, k) };
    let mut i_x: sqlite3_int64 = 0 as sqlite3_int64;
    let mut r_x: f64 = 0.0;
    let mut u_x: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut j: i32 = 0;
    unsafe { putc(i_d_type, out) };
    '__s45:
        {
        match i_d_type {
            1 => {
                i_x = unsafe { sqlite3_column_int64(p_stmt_1, k) };
                unsafe {
                    memcpy(&raw mut u_x as *mut (), &raw mut i_x as *const (),
                        8 as u64)
                };
                {
                    j = 56;
                    '__b46: loop {
                        if !(j >= 0) { break '__b46; }
                        '__c46: loop {
                            unsafe {
                                putc((u_x >> j & 255 as sqlite3_uint64) as i32, out)
                            };
                            break '__c46;
                        }
                        j -= 8;
                    }
                }
            }
            2 => {
                r_x = unsafe { sqlite3_column_double(p_stmt_1, k) };
                unsafe {
                    memcpy(&raw mut u_x as *mut (), &raw mut r_x as *const (),
                        8 as u64)
                };
                {
                    j = 56;
                    '__b47: loop {
                        if !(j >= 0) { break '__b47; }
                        '__c47: loop {
                            unsafe {
                                putc((u_x >> j & 255 as sqlite3_uint64) as i32, out)
                            };
                            break '__c47;
                        }
                        j -= 8;
                    }
                }
            }
            3 => {
                i_x =
                    unsafe { sqlite3_column_bytes(p_stmt_1, k) } as
                        sqlite3_int64;
                puts_varint(out, i_x as sqlite3_uint64);
                unsafe {
                    fwrite(unsafe { sqlite3_column_text(p_stmt_1, k) } as
                            *const (), 1 as u64, i_x as u64, out)
                };
            }
            4 => {
                i_x =
                    unsafe { sqlite3_column_bytes(p_stmt_1, k) } as
                        sqlite3_int64;
                puts_varint(out, i_x as sqlite3_uint64);
                unsafe {
                    fwrite(unsafe { sqlite3_column_blob(p_stmt_1, k) },
                        1 as u64, i_x as u64, out)
                };
            }
            5 => {}
            _ => {}
        }
    }
}
extern "C" fn changeset_one_table(z_tab_1: *const i8, out: *mut FILE) -> () {
    unsafe {
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_id: *mut i8 = core::ptr::null_mut();
        let mut az_col: *mut *mut i8 = core::ptr::null_mut();
        let mut n_col: i64 = 0 as i64;
        let mut ai_flg: *mut i32 = core::ptr::null_mut();
        let mut ai_pk: *mut i32 = core::ptr::null_mut();
        let mut n_pk: i64 = 0 as i64;
        let mut p_sql: *mut sqlite3_str = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        let mut z_sep: *const i8 = core::ptr::null();
        let mut i_type: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s49:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        if n_col > 0 as i64 {
                            __state = 163;
                        } else { __state = 162; }
                    }
                    3 => { z_id = safe_id(z_tab_1); __state = 4; }
                    4 => { az_col = core::ptr::null_mut(); __state = 5; }
                    5 => { n_col = 0 as i64; __state = 6; }
                    6 => { ai_flg = core::ptr::null_mut(); __state = 7; }
                    7 => { ai_pk = core::ptr::null_mut(); __state = 8; }
                    8 => { n_pk = 0 as i64; __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { check_schemas_match(z_tab_1); __state = 13; }
                    13 => {
                        p_sql = unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        __state = 14;
                    }
                    14 => {
                        p_stmt =
                            unsafe {
                                db_prepare(c"PRAGMA main.table_info=%Q".as_ptr() as *mut i8
                                        as *const i8, z_tab_1)
                            };
                        __state = 15;
                    }
                    15 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 33; }
                    17 => {
                        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                        __state = 18;
                    }
                    18 => {
                        az_col =
                            unsafe {
                                    sqlite3_realloc64(az_col as *mut (),
                                        core::mem::size_of::<*mut i8>() as u64 * n_col as u64)
                                } as *mut *mut i8;
                        __state = 19;
                    }
                    19 => {
                        if az_col == core::ptr::null_mut() {
                            __state = 21;
                        } else { __state = 20; }
                    }
                    20 => {
                        ai_flg =
                            unsafe {
                                    sqlite3_realloc64(ai_flg as *mut (),
                                        core::mem::size_of::<i32>() as u64 * n_col as u64)
                                } as *mut i32;
                        __state = 22;
                    }
                    21 => {
                        unsafe {
                            runtime_error(c"out of memory".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 20;
                    }
                    22 => {
                        if ai_flg == core::ptr::null_mut() {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    23 => {
                        unsafe {
                            *az_col.offset((n_col - 1 as i64) as isize) =
                                safe_id(unsafe { sqlite3_column_text(p_stmt, 1) } as
                                        *const i8)
                        };
                        __state = 25;
                    }
                    24 => {
                        unsafe {
                            runtime_error(c"out of memory".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 23;
                    }
                    25 => {
                        unsafe {
                            *ai_flg.offset((n_col - 1 as i64) as isize) =
                                { i = unsafe { sqlite3_column_int(p_stmt, 5) }; i }
                        };
                        __state = 26;
                    }
                    26 => { if i > 0 { __state = 27; } else { __state = 15; } }
                    27 => {
                        if i as i64 > n_pk { __state = 29; } else { __state = 28; }
                    }
                    28 => {
                        unsafe {
                            *ai_pk.offset((i - 1) as isize) = (n_col - 1 as i64) as i32
                        };
                        __state = 15;
                    }
                    29 => { n_pk = i as i64; __state = 30; }
                    30 => {
                        ai_pk =
                            unsafe {
                                    sqlite3_realloc64(ai_pk as *mut (),
                                        core::mem::size_of::<i32>() as u64 * n_pk as u64)
                                } as *mut i32;
                        __state = 31;
                    }
                    31 => {
                        if ai_pk == core::ptr::null_mut() {
                            __state = 32;
                        } else { __state = 28; }
                    }
                    32 => {
                        unsafe {
                            runtime_error(c"out of memory".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 28;
                    }
                    33 => {
                        if n_pk == 0 as i64 { __state = 35; } else { __state = 34; }
                    }
                    34 => {
                        if n_col > n_pk { __state = 37; } else { __state = 36; }
                    }
                    35 => { __state = 2; }
                    36 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"SELECT %d".as_ptr() as *mut i8 as *const i8, 9)
                        };
                        __state = 60;
                    }
                    37 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"SELECT %d".as_ptr() as *mut i8 as *const i8, 23)
                        };
                        __state = 38;
                    }
                    38 => { i = 0; __state = 40; }
                    39 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"\n  FROM main.%s A, aux.%s B\n".as_ptr() as *mut i8 as
                                    *const i8, z_id, z_id)
                        };
                        __state = 45;
                    }
                    40 => {
                        if (i as i64) < n_col {
                            __state = 41;
                        } else { __state = 39; }
                    }
                    41 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 43;
                        } else { __state = 44; }
                    }
                    42 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 40;
                    }
                    43 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c",\n       A.%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 42;
                    }
                    44 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c",\n       A.%s IS NOT B.%s, A.%s, B.%s".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *az_col.offset(i as isize) },
                                unsafe { *az_col.offset(i as isize) },
                                unsafe { *az_col.offset(i as isize) },
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 42;
                    }
                    45 => {
                        z_sep = c" WHERE".as_ptr() as *mut i8 as *const i8;
                        __state = 46;
                    }
                    46 => { i = 0; __state = 48; }
                    47 => {
                        z_sep = c"\n   AND (".as_ptr() as *mut i8 as *const i8;
                        __state = 52;
                    }
                    48 => {
                        if (i as i64) < n_pk {
                            __state = 49;
                        } else { __state = 47; }
                    }
                    49 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe {
                                    *az_col.offset(unsafe { *ai_pk.offset(i as isize) } as
                                                isize)
                                },
                                unsafe {
                                    *az_col.offset(unsafe { *ai_pk.offset(i as isize) } as
                                                isize)
                                })
                        };
                        __state = 51;
                    }
                    50 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 48;
                    }
                    51 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 50;
                    }
                    52 => { i = 0; __state = 54; }
                    53 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n UNION ALL\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 36;
                    }
                    54 => {
                        if (i as i64) < n_col {
                            __state = 55;
                        } else { __state = 53; }
                    }
                    55 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 58;
                        } else { __state = 57; }
                    }
                    56 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 54;
                    }
                    57 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%sA.%s IS NOT B.%s".as_ptr() as *mut i8 as *const i8,
                                z_sep, unsafe { *az_col.offset(i as isize) },
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 59;
                    }
                    58 => { __state = 56; }
                    59 => {
                        z_sep = c" OR\n        ".as_ptr() as *mut i8 as *const i8;
                        __state = 56;
                    }
                    60 => { i = 0; __state = 62; }
                    61 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"\n  FROM main.%s A\n".as_ptr() as *mut i8 as *const i8,
                                z_id)
                        };
                        __state = 67;
                    }
                    62 => {
                        if (i as i64) < n_col {
                            __state = 63;
                        } else { __state = 61; }
                    }
                    63 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 65;
                        } else { __state = 66; }
                    }
                    64 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 62;
                    }
                    65 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c",\n       A.%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 64;
                    }
                    66 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c",\n       1, A.%s, NULL".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 64;
                    }
                    67 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" WHERE NOT EXISTS(SELECT 1 FROM aux.%s B\n".as_ptr() as
                                        *mut i8 as *const i8, z_id)
                        };
                        __state = 68;
                    }
                    68 => {
                        z_sep =
                            c"                   WHERE".as_ptr() as *mut i8 as
                                *const i8;
                        __state = 69;
                    }
                    69 => { i = 0; __state = 71; }
                    70 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n UNION ALL\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 75;
                    }
                    71 => {
                        if (i as i64) < n_pk {
                            __state = 72;
                        } else { __state = 70; }
                    }
                    72 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe {
                                    *az_col.offset(unsafe { *ai_pk.offset(i as isize) } as
                                                isize)
                                },
                                unsafe {
                                    *az_col.offset(unsafe { *ai_pk.offset(i as isize) } as
                                                isize)
                                })
                        };
                        __state = 74;
                    }
                    73 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 71;
                    }
                    74 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 73;
                    }
                    75 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"SELECT %d".as_ptr() as *mut i8 as *const i8, 18)
                        };
                        __state = 76;
                    }
                    76 => { i = 0; __state = 78; }
                    77 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"\n  FROM aux.%s B\n".as_ptr() as *mut i8 as *const i8,
                                z_id)
                        };
                        __state = 83;
                    }
                    78 => {
                        if (i as i64) < n_col {
                            __state = 79;
                        } else { __state = 77; }
                    }
                    79 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 81;
                        } else { __state = 82; }
                    }
                    80 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 78;
                    }
                    81 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c",\n       B.%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 80;
                    }
                    82 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c",\n       1, NULL, B.%s".as_ptr() as *mut i8 as *const i8,
                                unsafe { *az_col.offset(i as isize) })
                        };
                        __state = 80;
                    }
                    83 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" WHERE NOT EXISTS(SELECT 1 FROM main.%s A\n".as_ptr() as
                                        *mut i8 as *const i8, z_id)
                        };
                        __state = 84;
                    }
                    84 => {
                        z_sep =
                            c"                   WHERE".as_ptr() as *mut i8 as
                                *const i8;
                        __state = 85;
                    }
                    85 => { i = 0; __state = 87; }
                    86 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c")\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 91;
                    }
                    87 => {
                        if (i as i64) < n_pk {
                            __state = 88;
                        } else { __state = 86; }
                    }
                    88 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s A.%s=B.%s".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe {
                                    *az_col.offset(unsafe { *ai_pk.offset(i as isize) } as
                                                isize)
                                },
                                unsafe {
                                    *az_col.offset(unsafe { *ai_pk.offset(i as isize) } as
                                                isize)
                                })
                        };
                        __state = 90;
                    }
                    89 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 87;
                    }
                    90 => {
                        z_sep = c" AND".as_ptr() as *mut i8 as *const i8;
                        __state = 89;
                    }
                    91 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c" ORDER BY".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 92;
                    }
                    92 => {
                        z_sep = c" ".as_ptr() as *mut i8 as *const i8;
                        __state = 93;
                    }
                    93 => { i = 0; __state = 95; }
                    94 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c";\n".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 99;
                    }
                    95 => {
                        if (i as i64) < n_pk {
                            __state = 96;
                        } else { __state = 94; }
                    }
                    96 => {
                        unsafe {
                            sqlite3_str_appendf(p_sql,
                                c"%s %d".as_ptr() as *mut i8 as *const i8, z_sep,
                                unsafe { *ai_pk.offset(i as isize) } + 2)
                        };
                        __state = 98;
                    }
                    97 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 95;
                    }
                    98 => {
                        z_sep = c",".as_ptr() as *mut i8 as *const i8;
                        __state = 97;
                    }
                    99 => {
                        if g.f_debug & 2 as u32 != 0 {
                            __state = 101;
                        } else { __state = 100; }
                    }
                    100 => { unsafe { putc('T' as i32, out) }; __state = 103; }
                    101 => {
                        unsafe {
                            fprintf(__stdoutp,
                                c"SQL for %s:\n%s\n".as_ptr() as *mut i8 as *const i8, z_id,
                                unsafe { sqlite3_str_value(p_sql) })
                        };
                        __state = 102;
                    }
                    102 => { __state = 2; }
                    103 => {
                        puts_varint(out, n_col as sqlite3_uint64);
                        __state = 104;
                    }
                    104 => { i = 0; __state = 106; }
                    105 => {
                        unsafe {
                            fwrite(z_tab_1 as *const (), 1 as u64,
                                unsafe { strlen(z_tab_1) }, out)
                        };
                        __state = 109;
                    }
                    106 => {
                        if (i as i64) < n_col {
                            __state = 107;
                        } else { __state = 105; }
                    }
                    107 => {
                        unsafe { putc(unsafe { *ai_flg.offset(i as isize) }, out) };
                        __state = 108;
                    }
                    108 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 106;
                    }
                    109 => { unsafe { putc(0, out) }; __state = 110; }
                    110 => {
                        p_stmt =
                            unsafe {
                                db_prepare(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { sqlite3_str_value(p_sql) })
                            };
                        __state = 111;
                    }
                    111 => {
                        if 100 == unsafe { sqlite3_step(p_stmt) } {
                            __state = 113;
                        } else { __state = 112; }
                    }
                    112 => {
                        unsafe { sqlite3_finalize(p_stmt) };
                        __state = 161;
                    }
                    113 => {
                        i_type = unsafe { sqlite3_column_int(p_stmt, 0) };
                        __state = 114;
                    }
                    114 => { unsafe { putc(i_type, out) }; __state = 115; }
                    115 => { unsafe { putc(0, out) }; __state = 116; }
                    116 => {
                        '__s50:
                            {
                            match unsafe { sqlite3_column_int(p_stmt, 0) } {
                                23 => { __state = 117; }
                                18 => { __state = 118; }
                                9 => { __state = 119; }
                                _ => { __state = 111; }
                            }
                        }
                    }
                    117 => { { k = 1; i = 0 }; __state = 123; }
                    118 => { { k = 1; i = 0 }; __state = 146; }
                    119 => { { k = 1; i = 0 }; __state = 154; }
                    120 => { __state = 117; }
                    121 => { __state = 118; }
                    122 => { { k = 1; i = 0 }; __state = 134; }
                    123 => {
                        if (i as i64) < n_col {
                            __state = 124;
                        } else { __state = 122; }
                    }
                    124 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 126;
                        } else { __state = 127; }
                    }
                    125 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 123;
                    }
                    126 => { put_value(out, p_stmt, k); __state = 128; }
                    127 => {
                        if unsafe { sqlite3_column_int(p_stmt, k) } != 0 {
                            __state = 129;
                        } else { __state = 130; }
                    }
                    128 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 125;
                    }
                    129 => { put_value(out, p_stmt, k + 1); __state = 131; }
                    130 => { unsafe { putc(0, out) }; __state = 132; }
                    131 => { k += 3; __state = 125; }
                    132 => { k += 3; __state = 125; }
                    133 => { __state = 111; }
                    134 => {
                        if (i as i64) < n_col {
                            __state = 135;
                        } else { __state = 133; }
                    }
                    135 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 137;
                        } else { __state = 138; }
                    }
                    136 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 134;
                    }
                    137 => { unsafe { putc(0, out) }; __state = 139; }
                    138 => {
                        if unsafe { sqlite3_column_int(p_stmt, k) } != 0 {
                            __state = 140;
                        } else { __state = 141; }
                    }
                    139 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 136;
                    }
                    140 => { put_value(out, p_stmt, k + 2); __state = 142; }
                    141 => { unsafe { putc(0, out) }; __state = 143; }
                    142 => { k += 3; __state = 136; }
                    143 => { k += 3; __state = 136; }
                    144 => { __state = 119; }
                    145 => { __state = 111; }
                    146 => {
                        if (i as i64) < n_col {
                            __state = 147;
                        } else { __state = 145; }
                    }
                    147 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 149;
                        } else { __state = 150; }
                    }
                    148 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 146;
                    }
                    149 => { put_value(out, p_stmt, k); __state = 151; }
                    150 => { put_value(out, p_stmt, k + 2); __state = 152; }
                    151 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 148;
                    }
                    152 => { k += 3; __state = 148; }
                    153 => { __state = 111; }
                    154 => {
                        if (i as i64) < n_col {
                            __state = 155;
                        } else { __state = 153; }
                    }
                    155 => {
                        if unsafe { *ai_flg.offset(i as isize) } != 0 {
                            __state = 157;
                        } else { __state = 158; }
                    }
                    156 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 154;
                    }
                    157 => { put_value(out, p_stmt, k); __state = 159; }
                    158 => { put_value(out, p_stmt, k + 1); __state = 160; }
                    159 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 156;
                    }
                    160 => { k += 3; __state = 156; }
                    161 => { __state = 2; }
                    162 => {
                        unsafe { sqlite3_free(az_col as *mut ()) };
                        __state = 164;
                    }
                    163 => {
                        unsafe {
                            sqlite3_free(unsafe {
                                        *az_col.offset({ let __p = &mut n_col; *__p -= 1; *__p } as
                                                    isize)
                                    } as *mut ())
                        };
                        __state = 2;
                    }
                    164 => {
                        unsafe { sqlite3_free(ai_pk as *mut ()) };
                        __state = 165;
                    }
                    165 => {
                        unsafe { sqlite3_free(z_id as *mut ()) };
                        __state = 166;
                    }
                    166 => {
                        unsafe { sqlite3_free(ai_flg as *mut ()) };
                        __state = 167;
                    }
                    167 => { str_free(p_sql); __state = 1; }
                    _ => {}
                }
            }
        }
    }
}
extern "C" fn is_whitespace(x: i8) -> i32 {
    return (x as i32 == ' ' as i32 || x as i32 == '\t' as i32 ||
                    x as i32 == '\n' as i32 || x as i32 == '\r' as i32) as i32;
}
extern "C" fn gobble_token(z_in_1: *const i8, z_buf_1: *mut i8, n_buf_1: i32)
    -> *const i8 {
    let mut p: *const i8 = z_in_1;
    let mut p_out: *mut i8 = z_buf_1;
    let p_end: *mut i8 =
        unsafe { &mut *p_out.offset((n_buf_1 - 1) as isize) };
    let mut q: i8 = 0 as i8;
    if p == core::ptr::null() { return core::ptr::null(); }
    while is_whitespace(unsafe { *p }) != 0 {
        {
            let __p = &mut p;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    '__s52:
        {
        match unsafe { *p } {
            34 => { q = '\"' as i32 as i8; }
            39 => { q = '\'' as i32 as i8; }
            96 => { q = '`' as i32 as i8; }
            91 => { q = ']' as i32 as i8; }
            _ => {}
        }
    }
    if q != 0 {
        {
            let __p = &mut p;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        while unsafe { *p } != 0 && p_out < p_end {
            if unsafe { *p } as i32 == q as i32 {
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                if unsafe { *p } as i32 != q as i32 { break; }
            }
            if p_out < p_end {
                unsafe {
                    *{
                                let __p = &mut p_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = unsafe { *p } as i8
                };
            }
            {
                let __p = &mut p;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    } else {
        while unsafe { *p } != 0 &&
                    (is_whitespace(unsafe { *p }) == 0) as i32 != 0 &&
                unsafe { *p } as i32 != '(' as i32 {
            if p_out < p_end {
                unsafe {
                    *{
                                let __p = &mut p_out;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } = unsafe { *p } as i8
                };
            }
            {
                let __p = &mut p;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
    unsafe { *p_out = '\u{0}' as i32 as i8 };
    return p;
}
extern "C" fn module_name_func(p_ctx_1: *mut sqlite3_context, n_val_1: i32,
    ap_val_1: *mut *mut sqlite3_value) -> () {
    let mut z_sql: *const i8 = core::ptr::null();
    let mut z_token: [i8; 32] = [0; 32];
    if !(n_val_1 == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"module_name_func".as_ptr() as *const i8,
                c"sqldiff.c".as_ptr() as *mut i8 as *const i8, 1804,
                c"nVal==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    z_sql =
        unsafe { sqlite3_value_text(unsafe { *ap_val_1.offset(0 as isize) }) }
            as *const i8;
    z_sql =
        gobble_token(z_sql, &raw mut z_token[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 32]>() as i32);
    if z_sql == core::ptr::null() ||
            unsafe {
                    sqlite3_stricmp(&raw mut z_token[0 as usize] as *mut i8 as
                            *const i8, c"create".as_ptr() as *mut i8 as *const i8)
                } != 0 {
        return;
    }
    z_sql =
        gobble_token(z_sql, &raw mut z_token[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 32]>() as i32);
    if z_sql == core::ptr::null() ||
            unsafe {
                    sqlite3_stricmp(&raw mut z_token[0 as usize] as *mut i8 as
                            *const i8, c"virtual".as_ptr() as *mut i8 as *const i8)
                } != 0 {
        return;
    }
    z_sql =
        gobble_token(z_sql, &raw mut z_token[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 32]>() as i32);
    if z_sql == core::ptr::null() ||
            unsafe {
                    sqlite3_stricmp(&raw mut z_token[0 as usize] as *mut i8 as
                            *const i8, c"table".as_ptr() as *mut i8 as *const i8)
                } != 0 {
        return;
    }
    z_sql =
        gobble_token(z_sql, &raw mut z_token[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 32]>() as i32);
    if z_sql == core::ptr::null() { return; }
    z_sql =
        gobble_token(z_sql, &raw mut z_token[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 32]>() as i32);
    if z_sql == core::ptr::null() ||
            unsafe {
                    sqlite3_stricmp(&raw mut z_token[0 as usize] as *mut i8 as
                            *const i8, c"using".as_ptr() as *mut i8 as *const i8)
                } != 0 {
        return;
    }
    z_sql =
        gobble_token(z_sql, &raw mut z_token[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 32]>() as i32);
    unsafe {
        sqlite3_result_text(p_ctx_1,
            &raw mut z_token[0 as usize] as *mut i8 as *const i8, -1,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}
#[unsafe(no_mangle)]
pub extern "C" fn all_tables_sql() -> *const i8 {
    unsafe {
        if g.b_handle_vtab != 0 {
            let mut rc: i32 = 0;
            rc =
                unsafe {
                    sqlite3_exec(g.db,
                        c"CREATE TEMP TABLE tblmap(module COLLATE nocase, postfix);INSERT INTO temp.tblmap VALUES(\'fts3\', \'_content\'), (\'fts3\', \'_segments\'), (\'fts3\', \'_segdir\'),(\'fts4\', \'_content\'), (\'fts4\', \'_segments\'), (\'fts4\', \'_segdir\'),(\'fts4\', \'_docsize\'), (\'fts4\', \'_stat\'),(\'fts5\', \'_data\'), (\'fts5\', \'_idx\'), (\'fts5\', \'_content\'),(\'fts5\', \'_docsize\'), (\'fts5\', \'_config\'),(\'rtree\', \'_node\'), (\'rtree\', \'_rowid\'), (\'rtree\', \'_parent\');".as_ptr()
                                as *mut i8 as *const i8, None, core::ptr::null_mut(),
                        core::ptr::null_mut())
                };
            if !(rc == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"all_tables_sql".as_ptr() as *const i8,
                        c"sqldiff.c".as_ptr() as *mut i8 as *const i8, 1844,
                        c"rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            rc =
                unsafe {
                    sqlite3_create_function(g.db,
                        c"module_name".as_ptr() as *mut i8 as *const i8, 1, 1,
                        core::ptr::null_mut(), Some(module_name_func), None, None)
                };
            if !(rc == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"all_tables_sql".as_ptr() as *const i8,
                        c"sqldiff.c".as_ptr() as *mut i8 as *const i8, 1849,
                        c"rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            return c"SELECT name FROM main.sqlite_schema\n WHERE type=\'table\' AND (\n    module_name(sql) IS NULL OR \n    module_name(sql) IN (SELECT module FROM temp.tblmap)\n ) AND name NOT IN (\n  SELECT a.name || b.postfix \nFROM main.sqlite_schema AS a, temp.tblmap AS b \nWHERE module_name(a.sql) = b.module\n )\nUNION \nSELECT name FROM aux.sqlite_schema\n WHERE type=\'table\' AND (\n    module_name(sql) IS NULL OR \n    module_name(sql) IN (SELECT module FROM temp.tblmap)\n ) AND name NOT IN (\n  SELECT a.name || b.postfix \nFROM aux.sqlite_schema AS a, temp.tblmap AS b \nWHERE module_name(a.sql) = b.module\n )\n ORDER BY name".as_ptr()
                        as *mut i8 as *const i8;
        } else {
            return c"SELECT name FROM main.sqlite_schema\n WHERE type=\'table\' AND sql NOT LIKE \'CREATE VIRTUAL%%\'\n UNION\nSELECT name FROM aux.sqlite_schema\n WHERE type=\'table\' AND sql NOT LIKE \'CREATE VIRTUAL%%\'\n ORDER BY name".as_ptr()
                        as *mut i8 as *const i8;
        }
    }
}
extern "C" fn show_help() -> () {
    unsafe {
        unsafe {
            fprintf(__stdoutp,
                c"Usage: %s [options] DB1 DB2\n".as_ptr() as *mut i8 as
                    *const i8, g.z_argv0)
        };
        unsafe {
            fprintf(__stdoutp,
                c"Output SQL text that would transform DB1 into DB2.\nOptions:\n  --changeset FILE      Write a CHANGESET into FILE\n  -L|--lib LIBRARY      Load an SQLite extension library\n  --primarykey          Use schema-defined PRIMARY KEYs\n  --rbu                 Output SQL to create/populate RBU table(s)\n  --schema              Show only differences in the schema\n  --summary             Show only a summary of the differences\n  --table TAB           Show only differences in table TAB\n  --transaction         Show SQL output inside a transaction\n  --vtab                Handle fts3, fts4, fts5 and rtree tables\nSee https://sqlite.org/sqldiff.html for detailed explanation.\n".as_ptr()
                        as *mut i8 as *const i8)
        };
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut z_db1: *const i8 = core::ptr::null();
        let mut z_db2: *const i8 = core::ptr::null();
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut z_err_msg: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_tab: *mut i8 = core::ptr::null_mut();
        let mut out: *mut FILE = __stdoutp;
        let mut x_diff:
                Option<unsafe extern "C" fn(*const i8, *mut __sFILE) -> ()> =
            Some(diff_one_table);
        let mut n_ext: i64 = 0 as i64;
        let mut az_ext: *mut *mut i8 = core::ptr::null_mut();
        let mut use_transaction: i32 = 0;
        let mut never_use_transaction: i32 = 0;
        g.z_argv0 = unsafe { *argv.offset(0 as isize) } as *const i8;
        unsafe { sqlite3_config(1) };
        {
            i = 1;
            '__b55: loop {
                if !(i < argc) { break '__b55; }
                '__c55: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        if unsafe {
                                    strcmp(z, c"changeset".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i == argc - 1 {
                                unsafe {
                                    cmdline_error(c"missing argument to %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            out =
                                unsafe {
                                    fopen(unsafe {
                                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                            } as *const i8, c"wb".as_ptr() as *mut i8 as *const i8)
                                };
                            if out == core::ptr::null_mut() {
                                unsafe {
                                    cmdline_error(c"cannot open: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            x_diff = Some(changeset_one_table);
                            never_use_transaction = 1;
                        } else if unsafe {
                                    strcmp(z, c"debug".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i == argc - 1 {
                                unsafe {
                                    cmdline_error(c"missing argument to %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            g.f_debug =
                                unsafe {
                                        strtol(unsafe {
                                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                                } as *const i8, core::ptr::null_mut(), 0)
                                    } as u32;
                        } else if unsafe {
                                    strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            show_help();
                            return Ok(());
                        } else if unsafe {
                                        strcmp(z, c"lib".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe { strcmp(z, c"L".as_ptr() as *mut i8 as *const i8) }
                                    == 0 {
                            if i == argc - 1 {
                                unsafe {
                                    cmdline_error(c"missing argument to %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            az_ext =
                                unsafe {
                                        realloc(az_ext as *mut (),
                                            core::mem::size_of::<*mut i8>() as u64 *
                                                (n_ext + 1 as i64) as u64)
                                    } as *mut *mut i8;
                            if az_ext == core::ptr::null_mut() {
                                unsafe {
                                    cmdline_error(c"out of memory".as_ptr() as *mut i8 as
                                            *const i8)
                                };
                            }
                            unsafe {
                                *az_ext.offset({
                                                    let __p = &mut n_ext;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) =
                                    unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    }
                            };
                        } else if unsafe {
                                    strcmp(z, c"primarykey".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_schema_pk = 1;
                        } else if unsafe {
                                    strcmp(z, c"rbu".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            x_diff = Some(rbudiff_one_table);
                        } else if unsafe {
                                    strcmp(z, c"schema".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_schema_only = 1;
                        } else if unsafe {
                                    strcmp(z, c"summary".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            x_diff = Some(summarize_one_table);
                        } else if unsafe {
                                    strcmp(z, c"table".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i == argc - 1 {
                                unsafe {
                                    cmdline_error(c"missing argument to %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            z_tab =
                                unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                };
                            g.b_schema_compare =
                                (unsafe {
                                                sqlite3_stricmp(z_tab as *const i8,
                                                    c"sqlite_schema".as_ptr() as *mut i8 as *const i8)
                                            } == 0 ||
                                        unsafe {
                                                sqlite3_stricmp(z_tab as *const i8,
                                                    c"sqlite_master".as_ptr() as *mut i8 as *const i8)
                                            } == 0) as i32;
                        } else if unsafe {
                                    strcmp(z, c"transaction".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            use_transaction = 1;
                        } else if unsafe {
                                    strcmp(z, c"vtab".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_handle_vtab = 1;
                        } else {
                            unsafe {
                                cmdline_error(c"unknown option: %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(i as isize) })
                            };
                        }
                    } else if z_db1 == core::ptr::null() {
                        z_db1 = unsafe { *argv.offset(i as isize) } as *const i8;
                    } else if z_db2 == core::ptr::null() {
                        z_db2 = unsafe { *argv.offset(i as isize) } as *const i8;
                    } else {
                        unsafe {
                            cmdline_error(c"unknown argument: %s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    break '__c55;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if z_db2 == core::ptr::null() {
            unsafe {
                cmdline_error(c"two database arguments required".as_ptr() as
                            *mut i8 as *const i8)
            };
        }
        if g.b_schema_only != 0 && g.b_schema_compare != 0 {
            unsafe {
                cmdline_error(c"The --schema option is useless with --table %s .".as_ptr()
                            as *mut i8 as *const i8, z_tab)
            };
        }
        rc =
            unsafe {
                sqlite3_open_v2(z_db1, &mut g.db, 1, core::ptr::null())
            };
        if rc != 0 {
            unsafe {
                cmdline_error(c"cannot open database file \"%s\"".as_ptr() as
                            *mut i8 as *const i8, z_db1)
            };
        }
        rc =
            unsafe {
                sqlite3_exec(g.db,
                    c"SELECT * FROM sqlite_schema".as_ptr() as *mut i8 as
                        *const i8, None, core::ptr::null_mut(), &mut z_err_msg)
            };
        if rc != 0 || !(z_err_msg).is_null() {
            unsafe {
                cmdline_error(c"\"%s\" does not appear to be a valid SQLite database".as_ptr()
                            as *mut i8 as *const i8, z_db1)
            };
        }
        {
            let mut db2: *mut sqlite3 = core::ptr::null_mut();
            if unsafe {
                        sqlite3_open_v2(z_db2, &mut db2, 1, core::ptr::null())
                    } != 0 {
                unsafe {
                    cmdline_error(c"cannot open database file \"%s\"".as_ptr()
                                as *mut i8 as *const i8, z_db2)
                };
            }
            unsafe { sqlite3_close(db2) };
        }
        unsafe { sqlite3_enable_load_extension(g.db, 1) };
        {
            i = 0;
            '__b56: loop {
                if !((i as i64) < n_ext) { break '__b56; }
                '__c56: loop {
                    rc =
                        unsafe {
                            sqlite3_load_extension(g.db,
                                unsafe { *az_ext.offset(i as isize) } as *const i8,
                                core::ptr::null(), &mut z_err_msg)
                        };
                    if rc != 0 || !(z_err_msg).is_null() {
                        unsafe {
                            cmdline_error(c"error loading %s: %s".as_ptr() as *mut i8 as
                                    *const i8, unsafe { *az_ext.offset(i as isize) }, z_err_msg)
                        };
                    }
                    break '__c56;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { free(az_ext as *mut ()) };
        z_sql =
            unsafe {
                sqlite3_mprintf(c"ATTACH %Q as aux;".as_ptr() as *mut i8 as
                        *const i8, z_db2)
            };
        rc =
            unsafe {
                sqlite3_exec(g.db, z_sql as *const i8, None,
                    core::ptr::null_mut(), &mut z_err_msg)
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        z_sql = core::ptr::null_mut();
        if rc != 0 || !(z_err_msg).is_null() {
            unsafe {
                cmdline_error(c"cannot attach database \"%s\"".as_ptr() as
                            *mut i8 as *const i8, z_db2)
            };
        }
        rc =
            unsafe {
                sqlite3_exec(g.db,
                    c"SELECT * FROM aux.sqlite_schema".as_ptr() as *mut i8 as
                        *const i8, None, core::ptr::null_mut(), &mut z_err_msg)
            };
        if rc != 0 || !(z_err_msg).is_null() {
            unsafe {
                cmdline_error(c"\"%s\" does not appear to be a valid SQLite database".as_ptr()
                            as *mut i8 as *const i8, z_db2)
            };
        }
        if never_use_transaction != 0 { use_transaction = 0; }
        if use_transaction != 0 {
            unsafe {
                fprintf(out,
                    c"BEGIN TRANSACTION;\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        if x_diff == Some(rbudiff_one_table) {
            unsafe {
                fprintf(out,
                    c"CREATE TABLE IF NOT EXISTS rbu_count(tbl TEXT PRIMARY KEY COLLATE NOCASE, cnt INTEGER) WITHOUT ROWID;\n".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        if !(z_tab).is_null() {
            unsafe {
                x_diff.unwrap()(z_tab as *const i8, out as *mut __sFILE)
            };
        } else {
            p_stmt =
                unsafe {
                    db_prepare(c"%s".as_ptr() as *mut i8 as *const i8,
                        all_tables_sql())
                };
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                unsafe {
                    x_diff.unwrap()(unsafe { sqlite3_column_text(p_stmt, 0) } as
                            *const i8, out as *mut __sFILE)
                };
            }
            unsafe { sqlite3_finalize(p_stmt) };
        }
        if use_transaction != 0 {
            unsafe {
                fprintf(__stdoutp,
                    c"COMMIT;\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe { sqlite3_close(g.db) };
        return Ok(());
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn isalpha(_c: i32)
    -> i32;
    fn isdigit(_c: i32)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn iscntrl(_c: i32)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn putc(_: i32, _: *mut FILE)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    fn realloc(__ptr: *mut (), __size: u64)
    -> *mut ();
    fn free(_: *mut ())
    -> ();
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
type FILE = __sFILE;
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}