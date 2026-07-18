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
    s_orig_mem: sqlite3_mem_methods,
    s_oom_mem: sqlite3_mem_methods,
    i_oom_cntdown: i32,
    n_oom_fault: i32,
    b_oom_once: i32,
    b_oom_enable: i32,
    n_oom_brkpt: i32,
    z_test_name: [i8; 100],
}
#[unsafe(no_mangle)]
pub static mut g: GlobalVars = unsafe { core::mem::zeroed() };
extern "C" fn oom_fault() -> () {
    unsafe {
        { let __p = &mut g.n_oom_brkpt; let __t = *__p; *__p += 1; __t };
    }
}
extern "C" fn oom_malloc(n_byte_1: i32) -> *mut () {
    unsafe {
        if n_byte_1 > 0 && g.b_oom_enable != 0 && g.i_oom_cntdown > 0 {
            {
                let __p = &mut g.i_oom_cntdown;
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if g.i_oom_cntdown == 0 {
                if g.n_oom_fault == 0 { oom_fault(); }
                {
                    let __p = &mut g.n_oom_fault;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                if (g.b_oom_once == 0) as i32 != 0 { g.i_oom_cntdown = 1; }
                return core::ptr::null_mut();
            }
        }
        return unsafe { g.s_orig_mem.x_malloc.unwrap()(n_byte_1) };
    }
}
extern "C" fn oom_realloc(p_old_1: *mut (), n_byte_1: i32) -> *mut () {
    unsafe {
        if n_byte_1 > 0 && g.b_oom_enable != 0 && g.i_oom_cntdown > 0 {
            {
                let __p = &mut g.i_oom_cntdown;
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if g.i_oom_cntdown == 0 {
                if g.n_oom_fault == 0 { oom_fault(); }
                {
                    let __p = &mut g.n_oom_fault;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                if (g.b_oom_once == 0) as i32 != 0 { g.i_oom_cntdown = 1; }
                return core::ptr::null_mut();
            }
        }
        return unsafe { g.s_orig_mem.x_realloc.unwrap()(p_old_1, n_byte_1) };
    }
}
unsafe extern "C" fn abend_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        if g.z_test_name[0 as usize] != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"%s (%s): ".as_ptr() as *mut i8 as *const i8, g.z_argv0,
                    &raw mut g.z_test_name[0 as usize] as *mut i8)
            };
        } else {
            unsafe {
                fprintf(__stderrp, c"%s: ".as_ptr() as *mut i8 as *const i8,
                    g.z_argv0)
            };
        }
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_format_1, ap) };
        ();
        eprintln!("");
        unsafe { abort() };
    }
}
unsafe extern "C" fn fatal_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        if g.z_test_name[0 as usize] != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"%s (%s): ".as_ptr() as *mut i8 as *const i8, g.z_argv0,
                    &raw mut g.z_test_name[0 as usize] as *mut i8)
            };
        } else {
            unsafe {
                fprintf(__stderrp, c"%s: ".as_ptr() as *mut i8 as *const i8,
                    g.z_argv0)
            };
        }
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_format_1, ap) };
        ();
        eprintln!("");
        unsafe { exit(1) };
    }
}
unsafe extern "C" fn sqlexec(db: *mut sqlite3, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut z_err_msg: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    rc =
        unsafe {
            sqlite3_exec(db, z_sql as *const i8, None, core::ptr::null_mut(),
                &mut z_err_msg)
        };
    if rc != 0 {
        unsafe {
            abend_error(c"failed sql [%s]: %s".as_ptr() as *mut i8 as
                    *const i8, z_sql, z_err_msg)
        };
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
}
extern "C" fn shell_log(p_not_used_1: *mut (), i_err_code_1: i32,
    z_msg_1: *const i8) -> () {
    unsafe {
        unsafe {
            printf(c"LOG: (%d) %s\n".as_ptr() as *mut i8 as *const i8,
                i_err_code_1, z_msg_1)
        };
        unsafe { fflush(__stdoutp) };
    }
}
extern "C" fn shell_log_noop(p_not_used_1: *mut (), i_err_code_1: i32,
    z_msg_1: *const i8) -> () {
    return;
}
extern "C" fn exec_callback(not_used_1: *mut (), argc: i32,
    argv: *mut *mut i8, colv: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        unsafe {
            printf(c"ROW #%u:\n".as_ptr() as *mut i8 as *const i8,
                { let __p = &mut cnt; *__p += 1; *__p })
        };
        if !(argv).is_null() {
            {
                i = 0;
                '__b0: loop {
                    if !(i < argc) { break '__b0; }
                    '__c0: loop {
                        unsafe {
                            printf(c" %s=".as_ptr() as *mut i8 as *const i8,
                                unsafe { *colv.offset(i as isize) })
                        };
                        if !(unsafe { *argv.offset(i as isize) }).is_null() {
                            unsafe {
                                printf(c"[%s]\n".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(i as isize) })
                            };
                        } else {
                            unsafe {
                                printf(c"NULL\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        break '__c0;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        unsafe { fflush(__stdoutp) };
        return 0;
    }
}
extern "C" fn exec_noop(not_used_1: *mut (), argc: i32, argv: *mut *mut i8,
    colv: *mut *mut i8) -> i32 {
    return 0;
}
extern "C" fn trace_callback(not_used_1: *mut (), z_msg_1: *const i8) -> () {
    unsafe {
        unsafe {
            printf(c"TRACE: %s\n".as_ptr() as *mut i8 as *const i8, z_msg_1)
        };
        unsafe { fflush(__stdoutp) };
    }
}
extern "C" fn trace_noop(not_used_1: *mut (), z_msg_1: *const i8) -> () {
    return;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Str {
    z: *mut i8,
    n: sqlite3_uint64,
    n_alloc: sqlite3_uint64,
    oom_err: i32,
}
extern "C" fn str_init(p: *mut Str) -> () {
    unsafe { memset(p as *mut (), 0, core::mem::size_of::<Str>() as u64) };
}
extern "C" fn str_append(p: *mut Str, z: *const i8) -> () {
    let n: sqlite3_uint64 = unsafe { strlen(z) } as sqlite3_uint64;
    if unsafe { (*p).n } + n >= unsafe { (*p).n_alloc } {
        let mut z_new: *mut i8 = core::ptr::null_mut();
        let mut n_new: sqlite3_uint64 = 0 as sqlite3_uint64;
        if unsafe { (*p).oom_err } != 0 { return; }
        n_new =
            unsafe { (*p).n_alloc } * 2 as sqlite3_uint64 +
                    100 as sqlite3_uint64 + n;
        z_new =
            unsafe {
                    sqlite3_realloc(unsafe { (*p).z } as *mut (), n_new as i32)
                } as *mut i8;
        if z_new == core::ptr::null_mut() {
            unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
            unsafe {
                memset(p as *mut (), 0, core::mem::size_of::<Str>() as u64)
            };
            unsafe { (*p).oom_err = 1 };
            return;
        }
        unsafe { (*p).z = z_new };
        unsafe { (*p).n_alloc = n_new };
    }
    unsafe {
        memcpy(unsafe { unsafe { (*p).z.add(unsafe { (*p).n } as usize) } } as
                *mut (), z as *const (), n as u64)
    };
    unsafe { (*p).n += n };
    unsafe { *unsafe { (*p).z.add(unsafe { (*p).n } as usize) } = 0 as i8 };
}
extern "C" fn str_str(p: &Str) -> *mut i8 { return (*p).z; }
extern "C" fn str_free(p: *mut Str) -> () {
    unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
    str_init(p);
}
#[repr(C)]
#[derive(Copy, Clone)]
struct EvalResult {
    z: *mut i8,
    z_sep: *const i8,
    sz_sep: i32,
    n_alloc: sqlite3_int64,
    n_used: sqlite3_int64,
}
extern "C" fn callback(p_ctx_1: *mut (), argc: i32, argv: *mut *mut i8,
    colnames: *mut *mut i8) -> i32 {
    let p: *mut EvalResult = p_ctx_1 as *mut EvalResult;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b1: loop {
            if !(i < argc) { break '__b1; }
            '__c1: loop {
                let z: *const i8 =
                    if !(unsafe { *argv.offset(i as isize) }).is_null() {
                            unsafe { *argv.offset(i as isize) }
                        } else { c"".as_ptr() as *mut i8 } as *const i8;
                let sz: u64 = unsafe { strlen(z) };
                if sz as sqlite3_int64 + unsafe { (*p).n_used } +
                                unsafe { (*p).sz_sep } as sqlite3_int64 + 1 as sqlite3_int64
                        > unsafe { (*p).n_alloc } {
                    let mut z_new: *mut i8 = core::ptr::null_mut();
                    unsafe {
                        (*p).n_alloc =
                            ((unsafe { (*p).n_alloc } * 2 as sqlite3_int64) as u64 +
                                            sz as u64 + unsafe { (*p).sz_sep } as u64 + 1 as u64) as
                                sqlite3_int64
                    };
                    z_new =
                        if unsafe { (*p).n_alloc } <= 2147483647 as i64 {
                                unsafe {
                                    sqlite3_realloc(unsafe { (*p).z } as *mut (),
                                        unsafe { (*p).n_alloc } as i32)
                                }
                            } else { core::ptr::null_mut() } as *mut i8;
                    if z_new == core::ptr::null_mut() {
                        unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
                        unsafe {
                            memset(p as *mut (), 0,
                                core::mem::size_of::<EvalResult>() as u64)
                        };
                        return 1;
                    }
                    unsafe { (*p).z = z_new };
                }
                if unsafe { (*p).n_used } > 0 as i64 {
                    unsafe {
                        memcpy(unsafe {
                                    &raw mut *unsafe {
                                                (*p).z.offset(unsafe { (*p).n_used } as isize)
                                            }
                                } as *mut (), unsafe { (*p).z_sep } as *const (),
                            unsafe { (*p).sz_sep } as u64)
                    };
                    unsafe {
                        (*p).n_used += unsafe { (*p).sz_sep } as sqlite3_int64
                    };
                }
                unsafe {
                    memcpy(unsafe {
                                &raw mut *unsafe {
                                            (*p).z.offset(unsafe { (*p).n_used } as isize)
                                        }
                            } as *mut (), z as *const (), sz)
                };
                unsafe { (*p).n_used += sz as u64 as sqlite3_int64 };
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
extern "C" fn sql_eval_func(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut z_sql: *const i8 = core::ptr::null();
    let mut db: *mut sqlite3 = core::ptr::null_mut();
    let mut z_err: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut x: EvalResult = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut x as *mut (), 0,
            core::mem::size_of::<EvalResult>() as u64)
    };
    x.z_sep = c" ".as_ptr() as *mut i8 as *const i8;
    z_sql =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    if z_sql == core::ptr::null() { return; }
    if argc > 1 {
        x.z_sep =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8;
        if x.z_sep == core::ptr::null() { return; }
    }
    x.sz_sep = unsafe { strlen(x.z_sep) } as i32;
    db = unsafe { sqlite3_context_db_handle(context) };
    rc =
        unsafe {
            sqlite3_exec(db, z_sql, Some(callback), &raw mut x as *mut (),
                &mut z_err)
        };
    if rc != 0 {
        unsafe { sqlite3_result_error(context, z_err as *const i8, -1) };
        unsafe { sqlite3_free(z_err as *mut ()) };
    } else if x.z_sep == core::ptr::null() {
        unsafe { sqlite3_result_error_nomem(context) };
        unsafe { sqlite3_free(x.z as *mut ()) };
    } else {
        unsafe {
            sqlite3_result_text(context, x.z as *const i8, x.n_used as i32,
                Some(sqlite3_free))
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct series_cursor {
    base: sqlite3_vtab_cursor,
    is_desc: i32,
    i_rowid: sqlite3_int64,
    i_value: sqlite3_int64,
    mn_value: sqlite3_int64,
    mx_value: sqlite3_int64,
    i_step: sqlite3_int64,
}
extern "C" fn series_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut sqlite3_vtab = core::ptr::null_mut();
    let mut rc: i32 = 0;
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
                                sqlite3_malloc(core::mem::size_of::<sqlite3_vtab>() as i32)
                            } as *mut sqlite3_vtab
                };
                unsafe { *pp_vtab_1 }
            };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_new as *mut (), 0,
                core::mem::size_of::<sqlite3_vtab>() as u64)
        };
    }
    return rc;
}
extern "C" fn series_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    return 0;
}
extern "C" fn series_open(p: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let mut p_cur: *mut series_cursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc(core::mem::size_of::<series_cursor>() as i32)
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
    if unsafe { (*p_cur).is_desc } != 0 {
        unsafe { (*p_cur).i_value -= unsafe { (*p_cur).i_step } };
    } else { unsafe { (*p_cur).i_value += unsafe { (*p_cur).i_step } }; }
    {
        let __p = unsafe { &mut (*p_cur).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return 0;
}
extern "C" fn series_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_cur: *const series_cursor =
        cur as *mut series_cursor as *const series_cursor;
    let mut x: sqlite3_int64 = 0 as sqlite3_int64;
    '__s2:
        {
        match i {
            1 => { x = unsafe { (*p_cur).mn_value }; }
            2 => { x = unsafe { (*p_cur).mx_value }; }
            3 => { x = unsafe { (*p_cur).i_step }; }
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
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}
extern "C" fn series_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_cur: *const series_cursor =
        cur as *mut series_cursor as *const series_cursor;
    if unsafe { (*p_cur).is_desc } != 0 {
        return (unsafe { (*p_cur).i_value } < unsafe { (*p_cur).mn_value }) as
                i32;
    } else {
        return (unsafe { (*p_cur).i_value } > unsafe { (*p_cur).mx_value }) as
                i32;
    }
}
extern "C" fn series_filter(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let p_cur: *mut series_cursor = p_vtab_cursor_1 as *mut series_cursor;
    let mut i: i32 = 0;
    if idx_num_1 & 1 != 0 {
        unsafe {
            (*p_cur).mn_value =
                unsafe {
                    sqlite3_value_int64(unsafe {
                            *argv.offset({
                                            let __p = &mut i;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        })
                }
        };
    } else { unsafe { (*p_cur).mn_value = 0 as sqlite3_int64 }; }
    if idx_num_1 & 2 != 0 {
        unsafe {
            (*p_cur).mx_value =
                unsafe {
                    sqlite3_value_int64(unsafe {
                            *argv.offset({
                                            let __p = &mut i;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        })
                }
        };
    } else { unsafe { (*p_cur).mx_value = 4294967295u32 as sqlite3_int64 }; }
    if idx_num_1 & 4 != 0 {
        unsafe {
            (*p_cur).i_step =
                unsafe {
                    sqlite3_value_int64(unsafe {
                            *argv.offset({
                                            let __p = &mut i;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        })
                }
        };
        if unsafe { (*p_cur).i_step } < 1 as i64 {
            unsafe { (*p_cur).i_step = 1 as sqlite3_int64 };
        }
    } else { unsafe { (*p_cur).i_step = 1 as sqlite3_int64 }; }
    if idx_num_1 & 8 != 0 {
        unsafe { (*p_cur).is_desc = 1 };
        unsafe { (*p_cur).i_value = unsafe { (*p_cur).mx_value } };
        if unsafe { (*p_cur).i_step } > 0 as i64 {
            unsafe {
                (*p_cur).i_value -=
                    (unsafe { (*p_cur).mx_value } -
                            unsafe { (*p_cur).mn_value }) % unsafe { (*p_cur).i_step }
            };
        }
    } else {
        unsafe { (*p_cur).is_desc = 0 };
        unsafe { (*p_cur).i_value = unsafe { (*p_cur).mn_value } };
    }
    unsafe { (*p_cur).i_rowid = 1 as sqlite3_int64 };
    return 0;
}
extern "C" fn series_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i: i32 = 0;
    let mut idx_num: i32 = 0;
    let mut start_idx: i32 = -1;
    let mut stop_idx: i32 = -1;
    let mut step_idx: i32 = -1;
    let mut n_arg: i32 = 0;
    let mut p_constraint: *const sqlite3_index_constraint = core::ptr::null();
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const sqlite3_index_constraint;
    {
        i = 0;
        '__b3: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) { break '__b3; }
            '__c3: loop {
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c3;
                }
                if unsafe { (*p_constraint).op } as i32 != 2 { break '__c3; }
                '__s4:
                    {
                    match unsafe { (*p_constraint).i_column } {
                        1 => { start_idx = i; idx_num |= 1; }
                        2 => { stop_idx = i; idx_num |= 2; }
                        3 => { step_idx = i; idx_num |= 4; }
                        _ => {}
                    }
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
    if start_idx >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(start_idx as
                                    isize)
                        }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(start_idx as
                                    isize)
                        }).omit = (0 == 0) as i32 as u8
        };
    }
    if stop_idx >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(stop_idx as isize)
                        }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(stop_idx as isize)
                        }).omit = (0 == 0) as i32 as u8
        };
    }
    if step_idx >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(step_idx as isize)
                        }).argv_index = { let __p = &mut n_arg; *__p += 1; *__p }
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(step_idx as isize)
                        }).omit = (0 == 0) as i32 as u8
        };
    }
    if idx_num & 3 == 3 {
        unsafe {
            (*p_idx_info_1).estimated_cost =
                (2 - (idx_num & 4 != 0) as i32) as f64
        };
        unsafe { (*p_idx_info_1).estimated_rows = 1000 as sqlite3_int64 };
        if unsafe { (*p_idx_info_1).n_order_by } == 1 {
            if unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } != 0 {
                idx_num |= 8;
            }
            unsafe { (*p_idx_info_1).order_by_consumed = 1 };
        }
    } else {
        unsafe { (*p_idx_info_1).estimated_cost = 2147483647 as f64 };
        unsafe {
            (*p_idx_info_1).estimated_rows = 2147483647 as sqlite3_int64
        };
    }
    unsafe { (*p_idx_info_1).idx_num = idx_num };
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
extern "C" fn show_help() -> () {
    unsafe {
        unsafe {
            printf(c"Usage: %s [options] ?FILE...?\n".as_ptr() as *mut i8 as
                    *const i8, g.z_argv0)
        };
        unsafe {
            printf(c"Read SQL text from FILE... (or from standard input if FILE... is omitted)\nand then evaluate each block of SQL contained therein.\nOptions:\n  --autovacuum          Enable AUTOVACUUM mode\n  --database FILE       Use database FILE instead of an in-memory database\n  --disable-lookaside   Turn off lookaside memory\n  --heap SZ MIN         Memory allocator uses SZ bytes & min allocation MIN\n  --help                Show this help text\n  --lookaside N SZ      Configure lookaside for N slots of SZ bytes each\n  --oom                 Run each test multiple times in a simulated OOM loop\n  --pagesize N          Set the page size to N\n  --pcache N SZ         Configure N pages of pagecache each of size SZ bytes\n  -q                    Reduced output\n  --quiet               Reduced output\n  --scratch N SZ        Configure scratch memory for N slots of SZ bytes each\n  --unique-cases FILE   Write all unique test cases to FILE\n  --utf16be             Set text encoding to UTF-16BE\n  --utf16le             Set text encoding to UTF-16LE\n  -v                    Increased output\n  --verbose             Increased output\n".as_ptr()
                        as *mut i8 as *const i8)
        };
    }
}
extern "C" fn hex_digit_value(c: i8) -> i32 {
    if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
        return c as i32 - '0' as i32;
    }
    if c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32 {
        return c as i32 - 'a' as i32 + 10;
    }
    if c as i32 >= 'A' as i32 && c as i32 <= 'F' as i32 {
        return c as i32 - 'A' as i32 + 10;
    }
    return -1;
}
extern "C" fn integer_value(mut z_arg_1: *const i8) -> i32 {
    unsafe {
        let mut v: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i: i32 = 0;
        let mut is_neg: i32 = 0;
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '-' as i32 {
            is_neg = 1;
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '+' as i32
            {
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '0' as i32 &&
                unsafe { *z_arg_1.offset(1 as isize) } as i32 == 'x' as i32 {
            let mut x: i32 = 0;
            {
                let __n = 2;
                let __p = &mut z_arg_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            while {
                        x = hex_digit_value(unsafe { *z_arg_1.offset(0 as isize) });
                        x
                    } >= 0 {
                v = (v << 4) + x as sqlite3_int64;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else {
            while unsafe {
                        isdigit(unsafe { *z_arg_1.offset(0 as isize) } as u8 as i32)
                    } != 0 {
                v =
                    v * 10 as sqlite3_int64 +
                            unsafe { *z_arg_1.offset(0 as isize) } as sqlite3_int64 -
                        '0' as i32 as sqlite3_int64;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        {
            i = 0;
            '__b7: loop {
                if !((i as u64) <
                                core::mem::size_of::<[integer_value_s0_N16integer_value_s0; 9]>()
                                        as u64 / 16) {
                    break '__b7;
                }
                '__c7: loop {
                    if unsafe {
                                sqlite3_stricmp(a_mult[i as usize].z_suffix as *const i8,
                                    z_arg_1)
                            } == 0 {
                        v *= a_mult[i as usize].i_mult as sqlite3_int64;
                        break '__b7;
                    }
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if v > 2147483647 as i64 {
            unsafe {
                abend_error(c"parameter too large - max 2147483648".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        return if is_neg != 0 { -v } else { v } as i32;
    }
}
extern "C" fn time_of_day() -> sqlite3_int64 {
    unsafe {
        let mut t: sqlite3_int64 = 0 as sqlite3_int64;
        if clock_vfs == core::ptr::null_mut() {
            clock_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        }
        if unsafe { (*clock_vfs).i_version } >= 1 &&
                unsafe { (*clock_vfs).x_current_time_int64.is_some() } {
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time_int64.unwrap()
                    })(clock_vfs, &mut t)
            };
        } else {
            let mut r: f64 = 0.0;
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time.unwrap()
                    })(clock_vfs, &mut r)
            };
            t = (r * 86400000.0) as sqlite3_int64;
        }
        return t;
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut z_in: *mut i8 = core::ptr::null_mut();
        let mut n_alloc: i32 = 0;
        let mut n_in: i32 = 0;
        let mut got: u64 = 0 as u64;
        let mut rc: i32 = 0;
        let mut i: i32 = 0;
        let mut i_next: i32 = 0;
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut z_err_msg: *mut i8 = core::ptr::null_mut();
        let mut z_encoding: *const i8 = core::ptr::null();
        let mut n_heap: i32 = 0;
        let mut mn_heap: i32 = 0;
        let mut n_look: i32 = 0;
        let mut sz_look: i32 = 0;
        let mut n_p_cache: i32 = 0;
        let mut sz_p_cache: i32 = 0;
        let mut n_scratch: i32 = 0;
        let mut sz_scratch: i32 = 0;
        let mut page_size: i32 = 0;
        let mut p_heap: *mut () = core::ptr::null_mut();
        let mut p_look: *mut () = core::ptr::null_mut();
        let mut p_p_cache: *mut () = core::ptr::null_mut();
        let mut p_scratch: *mut () = core::ptr::null_mut();
        let mut do_autovac: i32 = 0;
        let mut z_sql: *const i8 = core::ptr::null();
        let mut z_to_free: *mut i8 = core::ptr::null_mut();
        let mut verbose_flag: i32 = 0;
        let mut quiet_flag: i32 = 0;
        let mut n_test: i32 = 0;
        let mut multi_test: i32 = 0;
        let mut last_pct: i32 = -1;
        let mut data_db: *mut sqlite3 = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_data_out: *const i8 = core::ptr::null();
        let mut n_header: i32 = 0;
        let mut oom_flag: i32 = 0;
        let mut oom_cnt: i32 = 0;
        let mut z_err_buf: [i8; 200] = [0; 200];
        let mut z_fail_code: *const i8 = core::ptr::null();
        let mut z_prompt: *const i8 = core::ptr::null();
        let mut n_in_file: i32 = 0;
        let mut az_in_file: *mut *mut i8 = core::ptr::null_mut();
        let mut jj: i32 = 0;
        let mut i_begin: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i_start: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i_end: sqlite3_int64 = 0 as sqlite3_int64;
        let mut z_db_name: *const i8 = core::ptr::null();
        i_begin = time_of_day();
        unsafe { sqlite3_shutdown() };
        z_fail_code =
            unsafe {
                    getenv(c"TEST_FAILURE".as_ptr() as *mut i8 as *const i8)
                } as *const i8;
        g.z_argv0 = unsafe { *argv.offset(0 as isize) } as *const i8;
        z_prompt = c"<stdin>".as_ptr() as *mut i8 as *const i8;
        {
            i = 1;
            '__b8: loop {
                if !(i < argc) { break '__b8; }
                '__c8: loop {
                    let mut z: *const i8 = core::ptr::null();
                    let mut __state: i32 = 0;
                    loop {
                        if __state == 1 { break; }
                        '__s10:
                            {
                            match __state {
                                0 => {
                                    z = unsafe { *argv.offset(i as isize) } as *const i8;
                                    __state = 3;
                                }
                                2 => {
                                    {
                                        let __p = &mut n_in_file;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                    __state = 71;
                                }
                                3 => {
                                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                                        __state = 4;
                                    } else { __state = 5; }
                                }
                                4 => {
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                    __state = 6;
                                }
                                5 => { __state = 2; }
                                6 => {
                                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                                        __state = 8;
                                    } else { __state = 7; }
                                }
                                7 => {
                                    if unsafe {
                                                strcmp(z, c"autovacuum".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 9;
                                    } else { __state = 10; }
                                }
                                8 => {
                                    {
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                    __state = 7;
                                }
                                9 => { do_autovac = 1; __state = 1; }
                                10 => {
                                    if unsafe {
                                                strcmp(z, c"database".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 11;
                                    } else { __state = 12; }
                                }
                                11 => {
                                    if i >= argc - 1 { __state = 14; } else { __state = 13; }
                                }
                                12 => {
                                    if unsafe {
                                                strcmp(z,
                                                    c"disable-lookaside".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 16;
                                    } else { __state = 17; }
                                }
                                13 => {
                                    z_db_name =
                                        unsafe { *argv.offset((i + 1) as isize) } as *const i8;
                                    __state = 15;
                                }
                                14 => {
                                    unsafe {
                                        abend_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                                as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 13;
                                }
                                15 => { i += 1; __state = 1; }
                                16 => { n_look = 1; __state = 18; }
                                17 => {
                                    if unsafe {
                                                    strcmp(z, c"f".as_ptr() as *mut i8 as *const i8)
                                                } == 0 && i + 1 < argc {
                                        __state = 19;
                                    } else { __state = 20; }
                                }
                                18 => { sz_look = 0; __state = 1; }
                                19 => {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    __state = 21;
                                }
                                20 => {
                                    if unsafe {
                                                strcmp(z, c"heap".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 22;
                                    } else { __state = 23; }
                                }
                                21 => { __state = 2; }
                                22 => {
                                    if i >= argc - 2 { __state = 25; } else { __state = 24; }
                                }
                                23 => {
                                    if unsafe {
                                                strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 28;
                                    } else { __state = 29; }
                                }
                                24 => {
                                    n_heap =
                                        integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                                *const i8);
                                    __state = 26;
                                }
                                25 => {
                                    unsafe {
                                        abend_error(c"missing arguments on %s\n".as_ptr() as *mut i8
                                                as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 24;
                                }
                                26 => {
                                    mn_heap =
                                        integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                                *const i8);
                                    __state = 27;
                                }
                                27 => { i += 2; __state = 1; }
                                28 => { show_help(); __state = 30; }
                                29 => {
                                    if unsafe {
                                                strcmp(z, c"lookaside".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 31;
                                    } else { __state = 32; }
                                }
                                30 => { return Ok(()); }
                                31 => {
                                    if i >= argc - 2 { __state = 34; } else { __state = 33; }
                                }
                                32 => {
                                    if unsafe {
                                                strcmp(z, c"oom".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 37;
                                    } else { __state = 38; }
                                }
                                33 => {
                                    n_look =
                                        integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                                *const i8);
                                    __state = 35;
                                }
                                34 => {
                                    unsafe {
                                        abend_error(c"missing arguments on %s".as_ptr() as *mut i8
                                                as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 33;
                                }
                                35 => {
                                    sz_look =
                                        integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                                *const i8);
                                    __state = 36;
                                }
                                36 => { i += 2; __state = 1; }
                                37 => { oom_flag = 1; __state = 1; }
                                38 => {
                                    if unsafe {
                                                strcmp(z, c"pagesize".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 39;
                                    } else { __state = 40; }
                                }
                                39 => {
                                    if i >= argc - 1 { __state = 42; } else { __state = 41; }
                                }
                                40 => {
                                    if unsafe {
                                                strcmp(z, c"pcache".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 43;
                                    } else { __state = 44; }
                                }
                                41 => {
                                    page_size =
                                        integer_value(unsafe {
                                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                                } as *const i8);
                                    __state = 1;
                                }
                                42 => {
                                    unsafe {
                                        abend_error(c"missing argument on %s".as_ptr() as *mut i8 as
                                                *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 41;
                                }
                                43 => {
                                    if i >= argc - 2 { __state = 46; } else { __state = 45; }
                                }
                                44 => {
                                    if unsafe {
                                                    strcmp(z, c"quiet".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                            unsafe { strcmp(z, c"q".as_ptr() as *mut i8 as *const i8) }
                                                == 0 {
                                        __state = 49;
                                    } else { __state = 50; }
                                }
                                45 => {
                                    n_p_cache =
                                        integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                                *const i8);
                                    __state = 47;
                                }
                                46 => {
                                    unsafe {
                                        abend_error(c"missing arguments on %s".as_ptr() as *mut i8
                                                as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 45;
                                }
                                47 => {
                                    sz_p_cache =
                                        integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                                *const i8);
                                    __state = 48;
                                }
                                48 => { i += 2; __state = 1; }
                                49 => { quiet_flag = 1; __state = 51; }
                                50 => {
                                    if unsafe {
                                                strcmp(z, c"scratch".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 52;
                                    } else { __state = 53; }
                                }
                                51 => { verbose_flag = 0; __state = 1; }
                                52 => {
                                    if i >= argc - 2 { __state = 55; } else { __state = 54; }
                                }
                                53 => {
                                    if unsafe {
                                                strcmp(z, c"unique-cases".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 58;
                                    } else { __state = 59; }
                                }
                                54 => {
                                    n_scratch =
                                        integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                                *const i8);
                                    __state = 56;
                                }
                                55 => {
                                    unsafe {
                                        abend_error(c"missing arguments on %s".as_ptr() as *mut i8
                                                as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 54;
                                }
                                56 => {
                                    sz_scratch =
                                        integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                                *const i8);
                                    __state = 57;
                                }
                                57 => { i += 2; __state = 1; }
                                58 => {
                                    if i >= argc - 1 { __state = 61; } else { __state = 60; }
                                }
                                59 => {
                                    if unsafe {
                                                strcmp(z, c"utf16le".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 64;
                                    } else { __state = 65; }
                                }
                                60 => {
                                    if !(z_data_out).is_null() {
                                        __state = 63;
                                    } else { __state = 62; }
                                }
                                61 => {
                                    unsafe {
                                        abend_error(c"missing arguments on %s".as_ptr() as *mut i8
                                                as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 60;
                                }
                                62 => {
                                    z_data_out =
                                        unsafe {
                                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                            } as *const i8;
                                    __state = 1;
                                }
                                63 => {
                                    unsafe {
                                        abend_error(c"only one --minimize allowed".as_ptr() as
                                                    *mut i8 as *const i8)
                                    };
                                    __state = 62;
                                }
                                64 => {
                                    z_encoding = c"utf16le".as_ptr() as *mut i8 as *const i8;
                                    __state = 1;
                                }
                                65 => {
                                    if unsafe {
                                                strcmp(z, c"utf16be".as_ptr() as *mut i8 as *const i8)
                                            } == 0 {
                                        __state = 66;
                                    } else { __state = 67; }
                                }
                                66 => {
                                    z_encoding = c"utf16be".as_ptr() as *mut i8 as *const i8;
                                    __state = 1;
                                }
                                67 => {
                                    if unsafe {
                                                    strcmp(z, c"verbose".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                            unsafe { strcmp(z, c"v".as_ptr() as *mut i8 as *const i8) }
                                                == 0 {
                                        __state = 68;
                                    } else { __state = 69; }
                                }
                                68 => { quiet_flag = 0; __state = 70; }
                                69 => {
                                    unsafe {
                                        abend_error(c"unknown option: %s".as_ptr() as *mut i8 as
                                                *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                    __state = 1;
                                }
                                70 => { verbose_flag = 1; __state = 1; }
                                71 => {
                                    az_in_file =
                                        unsafe {
                                                realloc(az_in_file as *mut (),
                                                    core::mem::size_of::<*mut i8>() as u64 * n_in_file as u64)
                                            } as *mut *mut i8;
                                    __state = 72;
                                }
                                72 => {
                                    if az_in_file == core::ptr::null_mut() {
                                        __state = 74;
                                    } else { __state = 73; }
                                }
                                73 => {
                                    unsafe {
                                        *az_in_file.offset((n_in_file - 1) as isize) =
                                            unsafe { *argv.offset(i as isize) }
                                    };
                                    __state = 1;
                                }
                                74 => {
                                    unsafe {
                                        abend_error(c"out of memory".as_ptr() as *mut i8 as
                                                *const i8)
                                    };
                                    __state = 73;
                                }
                                _ => {}
                            }
                        }
                    }
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_config(16,
                if verbose_flag != 0 { shell_log } else { shell_log_noop }, 0)
        };
        if n_heap > 0 {
            p_heap = unsafe { malloc(n_heap as u64) };
            if p_heap == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"cannot allocate %d-byte heap\n".as_ptr() as
                                *mut i8 as *const i8, n_heap)
                };
            }
            rc = unsafe { sqlite3_config(8, p_heap, n_heap, mn_heap) };
            if rc != 0 {
                unsafe {
                    abend_error(c"heap configuration failed: %d\n".as_ptr() as
                                *mut i8 as *const i8, rc)
                };
            }
        }
        if oom_flag != 0 {
            unsafe {
                sqlite3_config(5,
                    &raw mut g.s_orig_mem as *mut sqlite3_mem_methods)
            };
            g.s_oom_mem = g.s_orig_mem;
            g.s_oom_mem.x_malloc = Some(oom_malloc);
            g.s_oom_mem.x_realloc = Some(oom_realloc);
            unsafe {
                sqlite3_config(4,
                    &raw mut g.s_oom_mem as *mut sqlite3_mem_methods)
            };
        }
        if n_look > 0 {
            unsafe { sqlite3_config(13, 0, 0) };
            if sz_look > 0 {
                p_look = unsafe { malloc((n_look * sz_look) as u64) };
                if p_look == core::ptr::null_mut() {
                    unsafe {
                        fatal_error(c"out of memory".as_ptr() as *mut i8 as
                                *const i8)
                    };
                }
            }
        }
        if n_scratch > 0 && sz_scratch > 0 {
            p_scratch =
                unsafe {
                    malloc((n_scratch as sqlite3_int64 *
                                sz_scratch as sqlite3_int64) as u64)
                };
            if p_scratch == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"cannot allocate %lld-byte scratch".as_ptr() as
                                *mut i8 as *const i8,
                        n_scratch as sqlite3_int64 * sz_scratch as sqlite3_int64)
                };
            }
            rc =
                unsafe {
                    sqlite3_config(6, p_scratch, sz_scratch, n_scratch)
                };
            if rc != 0 {
                unsafe {
                    abend_error(c"scratch configuration failed: %d\n".as_ptr()
                                as *mut i8 as *const i8, rc)
                };
            }
        }
        if n_p_cache > 0 && sz_p_cache > 0 {
            p_p_cache =
                unsafe {
                    malloc((n_p_cache as sqlite3_int64 *
                                sz_p_cache as sqlite3_int64) as u64)
                };
            if p_p_cache == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"cannot allocate %lld-byte pcache".as_ptr() as
                                *mut i8 as *const i8,
                        n_p_cache as sqlite3_int64 * sz_p_cache as sqlite3_int64)
                };
            }
            rc =
                unsafe {
                    sqlite3_config(7, p_p_cache, sz_p_cache, n_p_cache)
                };
            if rc != 0 {
                unsafe {
                    abend_error(c"pcache configuration failed: %d".as_ptr() as
                                *mut i8 as *const i8, rc)
                };
            }
        }
        if !(z_data_out).is_null() {
            rc =
                unsafe {
                    sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                        &mut data_db)
                };
            if rc != 0 {
                unsafe {
                    abend_error(c"cannot open :memory: database".as_ptr() as
                                *mut i8 as *const i8)
                };
            }
            rc =
                unsafe {
                    sqlite3_exec(data_db,
                        c"CREATE TABLE testcase(sql BLOB PRIMARY KEY, tm) WITHOUT ROWID;".as_ptr()
                                as *mut i8 as *const i8, None, core::ptr::null_mut(),
                        core::ptr::null_mut())
                };
            if rc != 0 {
                unsafe {
                    abend_error(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_errmsg(data_db) })
                };
            }
            rc =
                unsafe {
                    sqlite3_prepare_v2(data_db,
                        c"INSERT OR IGNORE INTO testcase(sql,tm)VALUES(?1,?2)".as_ptr()
                                as *mut i8 as *const i8, -1, &mut p_stmt,
                        core::ptr::null_mut())
                };
            if rc != 0 {
                unsafe {
                    abend_error(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_errmsg(data_db) })
                };
            }
        }
        if n_in_file == 0 { n_in_file = 1; }
        n_alloc = 1000;
        z_in = unsafe { malloc(n_alloc as u64) } as *mut i8;
        if z_in == core::ptr::null_mut() {
            unsafe {
                fatal_error(c"out of memory".as_ptr() as *mut i8 as *const i8)
            };
        }
        {
            jj = 0;
            '__b11: loop {
                if !(jj < n_in_file) { break '__b11; }
                '__c11: loop {
                    let mut in_: *mut FILE = core::ptr::null_mut();
                    if !(az_in_file).is_null() {
                        let mut j: i32 = 0;
                        let mut k: i32 = 0;
                        in_ =
                            unsafe {
                                fopen(unsafe { *az_in_file.offset(jj as isize) } as
                                        *const i8, c"rb".as_ptr() as *mut i8 as *const i8)
                            };
                        if in_ == core::ptr::null_mut() {
                            unsafe {
                                abend_error(c"cannot open %s for reading".as_ptr() as
                                            *mut i8 as *const i8,
                                    unsafe { *az_in_file.offset(jj as isize) })
                            };
                        }
                        z_prompt =
                            unsafe { *az_in_file.offset(jj as isize) } as *const i8;
                        {
                            j = { k = 0; k };
                            '__b12: loop {
                                if !(unsafe { *z_prompt.offset(j as isize) } != 0) {
                                    break '__b12;
                                }
                                '__c12: loop {
                                    if unsafe { *z_prompt.offset(j as isize) } as i32 ==
                                            '/' as i32 {
                                        k = j + 1;
                                    }
                                    break '__c12;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        {
                            let __n = k;
                            let __p = &mut z_prompt;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                    } else {
                        in_ = __stdinp;
                        z_prompt = c"<stdin>".as_ptr() as *mut i8 as *const i8;
                    }
                    while (unsafe { feof(in_) } == 0) as i32 != 0 {
                        got =
                            unsafe {
                                fread(unsafe { z_in.offset(n_in as isize) } as *mut (),
                                    1 as u64, (n_alloc - n_in - 1) as u64, in_)
                            };
                        n_in += got as i32;
                        unsafe { *z_in.offset(n_in as isize) = 0 as i8 };
                        if got == 0 as u64 { break; }
                        if n_alloc - n_in - 1 < 100 {
                            n_alloc += n_alloc + 1000;
                            z_in =
                                unsafe { realloc(z_in as *mut (), n_alloc as u64) } as
                                    *mut i8;
                            if z_in == core::ptr::null_mut() {
                                unsafe {
                                    fatal_error(c"out of memory".as_ptr() as *mut i8 as
                                            *const i8)
                                };
                            }
                        }
                    }
                    if in_ != __stdinp { unsafe { fclose(in_) }; }
                    last_pct = -1;
                    {
                        i = 0;
                        '__b14: loop {
                            if !(i < n_in) { break '__b14; }
                            '__c14: loop {
                                if unsafe { *z_in.offset(i as isize) } as i32 != '#' as i32
                                    {
                                    break '__b14;
                                }
                                {
                                    i_next = i + 1;
                                    '__b15: loop {
                                        if !(i_next < n_in &&
                                                        unsafe { *z_in.offset(i_next as isize) } as i32 !=
                                                            '\n' as i32) {
                                            break '__b15;
                                        }
                                        '__c15: loop { break '__c15; }
                                        { let __p = &mut i_next; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__c14;
                            }
                            i = i_next + 1;
                        }
                    }
                    n_header = i;
                    {
                        '__b16: loop {
                            if !(i < n_in) { break '__b16; }
                            '__c16: loop {
                                let mut c_saved: i8 = 0 as i8;
                                if unsafe {
                                            strncmp(unsafe { &raw mut *z_in.offset(i as isize) } as
                                                    *const i8, c"/****<".as_ptr() as *mut i8 as *const i8,
                                                6 as u64)
                                        } == 0 {
                                    let mut z: *mut i8 =
                                        unsafe {
                                            strstr(unsafe { &raw mut *z_in.offset(i as isize) } as
                                                    *const i8, c">****/".as_ptr() as *mut i8 as *const i8)
                                        };
                                    if !(z).is_null() {
                                        {
                                            let __n = 6;
                                            let __p = &mut z;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        unsafe {
                                            sqlite3_snprintf(core::mem::size_of::<[i8; 100]>() as i32,
                                                &raw mut g.z_test_name[0 as usize] as *mut i8,
                                                c"%.*s".as_ptr() as *mut i8 as *const i8,
                                                unsafe { z.offset_from(unsafe { z_in.offset(i as isize) }) }
                                                            as i64 as i32 - 12,
                                                unsafe { &raw mut *z_in.offset((i + 6) as isize) } as
                                                    *mut i8)
                                        };
                                        if verbose_flag != 0 {
                                            unsafe {
                                                printf(c"%.*s\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { z.offset_from(unsafe { z_in.offset(i as isize) }) }
                                                            as i64 as i32,
                                                    unsafe { &raw mut *z_in.offset(i as isize) } as *mut i8)
                                            };
                                            unsafe { fflush(__stdoutp) };
                                        }
                                        i +=
                                            unsafe { z.offset_from(unsafe { z_in.offset(i as isize) }) }
                                                    as i64 as i32;
                                        multi_test = 1;
                                    }
                                }
                                {
                                    i_next = i;
                                    '__b17: loop {
                                        if !(i_next < n_in &&
                                                        unsafe {
                                                                strncmp(unsafe { &raw mut *z_in.offset(i_next as isize) } as
                                                                        *const i8, c"/****<".as_ptr() as *mut i8 as *const i8,
                                                                    6 as u64)
                                                            } != 0) {
                                            break '__b17;
                                        }
                                        '__c17: loop { break '__c17; }
                                        { let __p = &mut i_next; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                c_saved = unsafe { *z_in.offset(i_next as isize) };
                                unsafe { *z_in.offset(i_next as isize) = 0 as i8 };
                                z_sql = unsafe { z_in.offset(i as isize) };
                                if verbose_flag != 0 {
                                    unsafe {
                                        printf(c"INPUT (offset: %d, size: %d): [%s]\n".as_ptr() as
                                                    *mut i8 as *const i8, i,
                                            unsafe {
                                                    strlen(unsafe { &raw mut *z_in.offset(i as isize) } as
                                                            *const i8)
                                                } as i32,
                                            unsafe { &raw mut *z_in.offset(i as isize) } as *mut i8)
                                    };
                                } else if multi_test != 0 && (quiet_flag == 0) as i32 != 0 {
                                    if oom_flag != 0 {
                                        unsafe {
                                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                                                &raw mut g.z_test_name[0 as usize] as *mut i8)
                                        };
                                    } else {
                                        let pct: i32 = 10 * i_next / n_in;
                                        if pct != last_pct {
                                            if last_pct < 0 {
                                                unsafe {
                                                    printf(c"%s:".as_ptr() as *mut i8 as *const i8, z_prompt)
                                                };
                                            }
                                            unsafe {
                                                printf(c" %d%%".as_ptr() as *mut i8 as *const i8, pct * 10)
                                            };
                                            last_pct = pct;
                                        }
                                    }
                                } else if n_in_file > 1 {
                                    unsafe {
                                        printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_prompt)
                                    };
                                }
                                unsafe { fflush(__stdoutp) };
                                if oom_flag != 0 {
                                    oom_cnt = { g.i_oom_cntdown = 1; g.i_oom_cntdown };
                                    g.n_oom_fault = 0;
                                    g.b_oom_once = 1;
                                    if verbose_flag != 0 {
                                        unsafe {
                                            printf(c"Once.%d\n".as_ptr() as *mut i8 as *const i8,
                                                oom_cnt)
                                        };
                                        unsafe { fflush(__stdoutp) };
                                    }
                                } else { oom_cnt = 0; }
                                '__b18: loop {
                                    '__c18: loop {
                                        let mut sql: Str = unsafe { core::mem::zeroed() };
                                        str_init(&mut sql);
                                        if !(z_db_name).is_null() {
                                            rc =
                                                unsafe {
                                                    sqlite3_open_v2(z_db_name, &mut db, 2, core::ptr::null())
                                                };
                                            if rc != 0 {
                                                unsafe {
                                                    abend_error(c"Cannot open database file %s".as_ptr() as
                                                                *mut i8 as *const i8, z_db_name)
                                                };
                                            }
                                        } else {
                                            rc =
                                                unsafe {
                                                    sqlite3_open_v2(c"main.db".as_ptr() as *mut i8 as *const i8,
                                                        &mut db, 2 | 4 | 128, core::ptr::null())
                                                };
                                            if rc != 0 {
                                                unsafe {
                                                    abend_error(c"Unable to open the in-memory database".as_ptr()
                                                                as *mut i8 as *const i8)
                                                };
                                            }
                                        }
                                        if !(p_look).is_null() {
                                            rc =
                                                unsafe {
                                                    sqlite3_db_config(db, 1001, p_look, sz_look, n_look)
                                                };
                                            if rc != 0 {
                                                unsafe {
                                                    abend_error(c"lookaside configuration filed: %d".as_ptr() as
                                                                *mut i8 as *const i8, rc)
                                                };
                                            }
                                        }
                                        unsafe {
                                            sqlite3_trace(db,
                                                Some(if verbose_flag != 0 {
                                                        trace_callback
                                                    } else { trace_noop }), core::ptr::null_mut())
                                        };
                                        unsafe {
                                            sqlite3_create_function(db,
                                                c"eval".as_ptr() as *mut i8 as *const i8, 1, 1,
                                                core::ptr::null_mut(), Some(sql_eval_func), None, None)
                                        };
                                        unsafe {
                                            sqlite3_create_function(db,
                                                c"eval".as_ptr() as *mut i8 as *const i8, 2, 1,
                                                core::ptr::null_mut(), Some(sql_eval_func), None, None)
                                        };
                                        unsafe {
                                            sqlite3_create_module(db,
                                                c"generate_series".as_ptr() as *mut i8 as *const i8,
                                                &raw mut series_module as *const sqlite3_module,
                                                core::ptr::null_mut())
                                        };
                                        unsafe { sqlite3_limit(db, 0, 1000000) };
                                        if !(z_encoding).is_null() {
                                            unsafe {
                                                sqlexec(db,
                                                    c"PRAGMA encoding=%s".as_ptr() as *mut i8 as *const i8,
                                                    z_encoding)
                                            };
                                        }
                                        if page_size != 0 {
                                            unsafe {
                                                sqlexec(db,
                                                    c"PRAGMA pagesize=%d".as_ptr() as *mut i8 as *const i8,
                                                    page_size)
                                            };
                                        }
                                        if do_autovac != 0 {
                                            unsafe {
                                                sqlexec(db,
                                                    c"PRAGMA auto_vacuum=FULL".as_ptr() as *mut i8 as *const i8)
                                            };
                                        }
                                        i_start = time_of_day();
                                        if unsafe {
                                                    sqlite3_table_column_metadata(db, core::ptr::null(),
                                                        c"autoexec".as_ptr() as *mut i8 as *const i8,
                                                        c"sql".as_ptr() as *mut i8 as *const i8,
                                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                                        core::ptr::null_mut())
                                                } == 0 {
                                            let mut p_stmt2: *mut sqlite3_stmt = core::ptr::null_mut();
                                            rc =
                                                unsafe {
                                                    sqlite3_prepare_v2(db,
                                                        c"SELECT sql FROM autoexec".as_ptr() as *mut i8 as
                                                            *const i8, -1, &mut p_stmt2, core::ptr::null_mut())
                                                };
                                            if rc == 0 {
                                                while unsafe { sqlite3_step(p_stmt2) } == 100 {
                                                    str_append(&mut sql,
                                                        unsafe { sqlite3_column_text(p_stmt2, 0) } as *const i8);
                                                    str_append(&mut sql,
                                                        c"\n".as_ptr() as *mut i8 as *const i8);
                                                }
                                            }
                                            unsafe { sqlite3_finalize(p_stmt2) };
                                            z_sql = str_str(&sql);
                                        }
                                        g.b_oom_enable = 1;
                                        if verbose_flag != 0 {
                                            z_err_msg = core::ptr::null_mut();
                                            rc =
                                                unsafe {
                                                    sqlite3_exec(db, z_sql as *const i8, Some(exec_callback),
                                                        core::ptr::null_mut(), &mut z_err_msg)
                                                };
                                            if !(z_err_msg).is_null() {
                                                unsafe {
                                                    sqlite3_snprintf(core::mem::size_of::<[i8; 200]>() as i32,
                                                        &raw mut z_err_buf[0 as usize] as *mut i8,
                                                        c"%z".as_ptr() as *mut i8 as *const i8, z_err_msg)
                                                };
                                                z_err_msg = core::ptr::null_mut();
                                            }
                                        } else {
                                            rc =
                                                unsafe {
                                                    sqlite3_exec(db, z_sql as *const i8, Some(exec_noop),
                                                        core::ptr::null_mut(), core::ptr::null_mut())
                                                };
                                        }
                                        g.b_oom_enable = 0;
                                        i_end = time_of_day();
                                        str_free(&mut sql);
                                        rc = unsafe { sqlite3_close(db) };
                                        if rc != 0 {
                                            unsafe {
                                                abend_error(c"sqlite3_close() failed with rc=%d".as_ptr() as
                                                            *mut i8 as *const i8, rc)
                                            };
                                        }
                                        if (z_data_out).is_null() as i32 != 0 &&
                                                unsafe { sqlite3_memory_used() } > 0 as i64 {
                                            unsafe {
                                                abend_error(c"memory in use after close: %lld bytes".as_ptr()
                                                            as *mut i8 as *const i8, unsafe { sqlite3_memory_used() })
                                            };
                                        }
                                        if oom_flag != 0 {
                                            if g.n_oom_fault == 0 || oom_cnt > 625 {
                                                if g.b_oom_once != 0 && oom_cnt <= 625 * 2 / 3 {
                                                    oom_cnt = { g.i_oom_cntdown = 1; g.i_oom_cntdown };
                                                    g.b_oom_once = 0;
                                                } else { oom_cnt = 0; }
                                            } else {
                                                g.i_oom_cntdown =
                                                    { let __p = &mut oom_cnt; *__p += 1; *__p };
                                                g.n_oom_fault = 0;
                                            }
                                            if oom_cnt != 0 {
                                                if verbose_flag != 0 {
                                                    unsafe {
                                                        printf(c"%s.%d\n".as_ptr() as *mut i8 as *const i8,
                                                            if g.b_oom_once != 0 {
                                                                c"Once".as_ptr() as *mut i8
                                                            } else { c"Multi".as_ptr() as *mut i8 }, oom_cnt)
                                                    };
                                                    unsafe { fflush(__stdoutp) };
                                                }
                                                { let __p = &mut n_test; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                        break '__c18;
                                    }
                                    if !(oom_cnt > 0) { break '__b18; }
                                }
                                if !(z_data_out).is_null() {
                                    unsafe {
                                        sqlite3_bind_blob(p_stmt, 1,
                                            unsafe { &raw mut *z_in.offset(i as isize) } as *const (),
                                            i_next - i, None)
                                    };
                                    unsafe { sqlite3_bind_int64(p_stmt, 2, i_end - i_start) };
                                    rc = unsafe { sqlite3_step(p_stmt) };
                                    if rc != 101 {
                                        unsafe {
                                            abend_error(c"%s".as_ptr() as *mut i8 as *const i8,
                                                unsafe { sqlite3_errmsg(data_db) })
                                        };
                                    }
                                    unsafe { sqlite3_reset(p_stmt) };
                                }
                                if !(z_to_free).is_null() {
                                    unsafe { sqlite3_free(z_to_free as *mut ()) };
                                    z_to_free = core::ptr::null_mut();
                                }
                                unsafe { *z_in.offset(i_next as isize) = c_saved };
                                if verbose_flag != 0 {
                                    unsafe {
                                        printf(c"RESULT-CODE: %d\n".as_ptr() as *mut i8 as
                                                *const i8, rc)
                                    };
                                    if !(z_err_msg).is_null() {
                                        unsafe {
                                            printf(c"ERROR-MSG: [%s]\n".as_ptr() as *mut i8 as
                                                    *const i8, &raw mut z_err_buf[0 as usize] as *mut i8)
                                        };
                                    }
                                    unsafe { fflush(__stdoutp) };
                                }
                                if !(z_fail_code).is_null() {
                                    if unsafe { *z_fail_code.offset(0 as isize) } as i32 ==
                                                '5' as i32 &&
                                            unsafe { *z_fail_code.offset(1 as isize) } as i32 == 0 {
                                        unsafe {
                                            abend_error(c"simulated failure".as_ptr() as *mut i8 as
                                                    *const i8)
                                        };
                                    } else if unsafe { *z_fail_code.offset(0 as isize) } as i32
                                            != 0 {
                                        unsafe {
                                            printf(c"\nExit early due to TEST_FAILURE being set".as_ptr()
                                                        as *mut i8 as *const i8)
                                        };
                                        break '__b16;
                                    }
                                }
                                break '__c16;
                            }
                            {
                                ({
                                        i = i_next;
                                        { let __p = &mut n_test; let __t = *__p; *__p += 1; __t }
                                    }) as i8;
                                g.z_test_name[0 as usize] = 0 as i8
                            };
                        }
                    }
                    if (verbose_flag == 0) as i32 != 0 && multi_test != 0 &&
                                (quiet_flag == 0) as i32 != 0 && (oom_flag == 0) as i32 != 0
                        {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                    }
                    break '__c11;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_test > 1 && (quiet_flag == 0) as i32 != 0 {
            let i_elapse: sqlite3_int64 = time_of_day() - i_begin;
            unsafe {
                printf(c"%s: 0 errors out of %d tests in %d.%03d seconds\nSQLite %s %s\n".as_ptr()
                            as *mut i8 as *const i8, g.z_argv0, n_test,
                    (i_elapse / 1000 as sqlite3_int64) as i32,
                    (i_elapse % 1000 as sqlite3_int64) as i32,
                    unsafe { sqlite3_libversion() },
                    unsafe { sqlite3_sourceid() })
            };
        }
        if !(z_data_out).is_null() {
            let mut n: i32 = 0;
            let out: *mut FILE =
                unsafe {
                    fopen(z_data_out, c"wb".as_ptr() as *mut i8 as *const i8)
                };
            if out == core::ptr::null_mut() {
                unsafe {
                    abend_error(c"cannot open %s for writing".as_ptr() as
                                *mut i8 as *const i8, z_data_out)
                };
            }
            if n_header > 0 {
                unsafe {
                    fwrite(z_in as *const (), n_header as u64, 1 as u64, out)
                };
            }
            unsafe { sqlite3_finalize(p_stmt) };
            rc =
                unsafe {
                    sqlite3_prepare_v2(data_db,
                        c"SELECT sql, tm FROM testcase ORDER BY tm, sql".as_ptr() as
                                *mut i8 as *const i8, -1, &mut p_stmt,
                        core::ptr::null_mut())
                };
            if rc != 0 {
                unsafe {
                    abend_error(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_errmsg(data_db) })
                };
            }
            while unsafe { sqlite3_step(p_stmt) } == 100 {
                unsafe {
                    fprintf(out,
                        c"/****<%d:%dms>****/".as_ptr() as *mut i8 as *const i8,
                        { let __p = &mut n; *__p += 1; *__p },
                        unsafe { sqlite3_column_int(p_stmt, 1) })
                };
                unsafe {
                    fwrite(unsafe { sqlite3_column_blob(p_stmt, 0) },
                        unsafe { sqlite3_column_bytes(p_stmt, 0) } as u64, 1 as u64,
                        out)
                };
            }
            unsafe { fclose(out) };
            unsafe { sqlite3_finalize(p_stmt) };
            unsafe { sqlite3_close(data_db) };
        }
        unsafe { free(az_in_file as *mut ()) };
        unsafe { free(z_in as *mut ()) };
        unsafe { free(p_heap) };
        unsafe { free(p_look) };
        unsafe { free(p_scratch) };
        unsafe { free(p_p_cache) };
        return Ok(());
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct integer_value_s0_N16integer_value_s0 {
    z_suffix: *mut i8,
    i_mult: i32,
}
static mut cnt: u32 = 0 as u32;
static mut a_mult: [integer_value_s0_N16integer_value_s0; 9] =
    [integer_value_s0_N16integer_value_s0 {
                z_suffix: c"KiB".as_ptr() as *mut i8,
                i_mult: 1024,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"MiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"GiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024 * 1024,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"KB".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"MB".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"GB".as_ptr() as *mut i8,
                i_mult: 1000000000,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"K".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"M".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            integer_value_s0_N16integer_value_s0 {
                z_suffix: c"G".as_ptr() as *mut i8,
                i_mult: 1000000000,
            }];
static mut clock_vfs: *mut sqlite3_vfs = core::ptr::null_mut();
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
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn abort()
    -> ();
    fn exit(_: i32)
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn isdigit(_c: i32)
    -> i32;
    fn getenv(_: *const i8)
    -> *mut i8;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn realloc(__ptr: *mut (), __size: u64)
    -> *mut ();
    fn malloc(__size: u64)
    -> *mut ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn feof(_: *mut FILE)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn free(_: *mut ())
    -> ();
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stdinp: *mut FILE;
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