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
struct DState {
    db: *mut sqlite3,
    n_err: i32,
    rc: i32,
    writable_schema: i32,
    x_callback: Option<unsafe extern "C" fn(*const i8, *mut ()) -> i32>,
    p_arg: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DText {
    z: *mut i8,
    n: sqlite3_int64,
    n_alloc: sqlite3_int64,
}
extern "C" fn init_text(p: *mut DText) -> () {
    unsafe { memset(p as *mut (), 0, core::mem::size_of::<DText>() as u64) };
}
extern "C" fn free_text(p: *mut DText) -> () {
    unsafe { sqlite3_free(unsafe { (*p).z } as *mut ()) };
    init_text(p);
}
extern "C" fn append_text(p: *mut DText, z_append_1: *const i8, quote: i8)
    -> () {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let n_append: i32 =
        (unsafe { strlen(z_append_1) } & 1073741823 as u64) as i32;
    len =
        (n_append as sqlite3_int64 + unsafe { (*p).n } + 1 as sqlite3_int64)
            as i32;
    if quote != 0 {
        len += 2;
        {
            i = 0;
            '__b0: loop {
                if !(i < n_append) { break '__b0; }
                '__c0: loop {
                    if unsafe { *z_append_1.offset(i as isize) } as i32 ==
                            quote as i32 {
                        { let __p = &mut len; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if unsafe { (*p).n } + len as sqlite3_int64 >= unsafe { (*p).n_alloc } {
        let mut z_new: *mut i8 = core::ptr::null_mut();
        unsafe {
            (*p).n_alloc =
                unsafe { (*p).n_alloc } * 2 as sqlite3_int64 +
                        len as sqlite3_int64 + 20 as sqlite3_int64
        };
        z_new =
            unsafe {
                    sqlite3_realloc64(unsafe { (*p).z } as *mut (),
                        unsafe { (*p).n_alloc } as sqlite3_uint64)
                } as *mut i8;
        if z_new == core::ptr::null_mut() { free_text(p); return; }
        unsafe { (*p).z = z_new };
    }
    if quote != 0 {
        let mut z_csr: *mut i8 =
            unsafe { unsafe { (*p).z.offset(unsafe { (*p).n } as isize) } };
        unsafe {
            *{
                        let __p = &mut z_csr;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = quote
        };
        {
            i = 0;
            '__b1: loop {
                if !(i < n_append) { break '__b1; }
                '__c1: loop {
                    unsafe {
                        *{
                                    let __p = &mut z_csr;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = unsafe { *z_append_1.offset(i as isize) } as i8
                    };
                    if unsafe { *z_append_1.offset(i as isize) } as i32 ==
                            quote as i32 {
                        unsafe {
                            *{
                                        let __p = &mut z_csr;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = quote
                        };
                    }
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            *{
                        let __p = &mut z_csr;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } = quote
        };
        unsafe {
            (*p).n =
                unsafe { z_csr.offset_from(unsafe { (*p).z }) } as i64 as i32
                    as sqlite3_int64
        };
        unsafe { *z_csr = '\u{0}' as i32 as i8 };
    } else {
        unsafe {
            memcpy(unsafe {
                        unsafe { (*p).z.offset(unsafe { (*p).n } as isize) }
                    } as *mut (), z_append_1 as *const (), n_append as u64)
        };
        unsafe { (*p).n += n_append as sqlite3_int64 };
        unsafe {
            *unsafe { (*p).z.offset(unsafe { (*p).n } as isize) } =
                '\u{0}' as i32 as i8
        };
    }
}
extern "C" fn quote_char(z_name_1: *const i8) -> i8 {
    let mut i: i32 = 0;
    if (unsafe {
                            isalpha(unsafe { *z_name_1.offset(0 as isize) } as u8 as
                                    i32)
                        } == 0) as i32 != 0 &&
            unsafe { *z_name_1.offset(0 as isize) } as i32 != '_' as i32 {
        return '\"' as i32 as i8;
    }
    {
        i = 0;
        '__b2: loop {
            if !(unsafe { *z_name_1.offset(i as isize) } != 0) {
                break '__b2;
            }
            '__c2: loop {
                if (unsafe {
                                        isalnum(unsafe { *z_name_1.offset(i as isize) } as u8 as
                                                i32)
                                    } == 0) as i32 != 0 &&
                        unsafe { *z_name_1.offset(i as isize) } as i32 != '_' as i32
                    {
                    return '\"' as i32 as i8;
                }
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return if unsafe { sqlite3_keyword_check(z_name_1, i) } != 0 {
                '\"' as i32
            } else { 0 } as i8;
}
extern "C" fn free_column_list(az_col_1: *mut *mut i8) -> () {
    let mut i: i32 = 0;
    {
        i = 1;
        '__b3: loop {
            if !(!(unsafe { *az_col_1.offset(i as isize) }).is_null()) {
                break '__b3;
            }
            '__c3: loop {
                unsafe {
                    sqlite3_free(unsafe { *az_col_1.offset(i as isize) } as
                            *mut ())
                };
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(az_col_1 as *mut ()) };
}
extern "C" fn table_column_list(p: &mut DState, z_tab_1: *const i8)
    -> *mut *mut i8 {
    unsafe {
        let mut az_col: *mut *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut n_col: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_alloc: sqlite3_int64 = 0 as sqlite3_int64;
        let mut n_pk: i32 = 0;
        let mut is_ipk: i32 = 0;
        let mut preserve_rowid: i32 = 0;
        let mut rc: i32 = 0;
        let mut az_new: *mut *mut i8 = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s5:
                {
                match __state {
                    0 => { az_col = core::ptr::null_mut(); __state = 3; }
                    2 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 69; }
                    3 => { p_stmt = core::ptr::null_mut(); __state = 4; }
                    4 => { __state = 5; }
                    5 => { n_col = 0 as sqlite3_int64; __state = 6; }
                    6 => { n_alloc = 0 as sqlite3_int64; __state = 7; }
                    7 => { n_pk = 0; __state = 8; }
                    8 => { is_ipk = 0; __state = 9; }
                    9 => { preserve_rowid = 1; __state = 10; }
                    10 => { __state = 11; }
                    11 => {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"PRAGMA table_info=%Q".as_ptr() as *mut i8
                                        as *const i8, z_tab_1)
                            };
                        __state = 12;
                    }
                    12 => {
                        if z_sql == core::ptr::null_mut() {
                            __state = 14;
                        } else { __state = 13; }
                    }
                    13 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2((*p).db, z_sql as *const i8, -1,
                                    &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 15;
                    }
                    14 => { return core::ptr::null_mut(); }
                    15 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 16;
                    }
                    16 => {
                        if rc != 0 { __state = 18; } else { __state = 17; }
                    }
                    17 => {
                        if unsafe { sqlite3_step(p_stmt) } == 100 {
                            __state = 20;
                        } else { __state = 19; }
                    }
                    18 => { return core::ptr::null_mut(); }
                    19 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 36; }
                    20 => {
                        if n_col >= n_alloc - 2 as sqlite3_int64 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        unsafe {
                            *az_col.offset({ let __p = &mut n_col; *__p += 1; *__p } as
                                            isize) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { sqlite3_column_text(p_stmt, 1) })
                                }
                        };
                        __state = 29;
                    }
                    22 => { __state = 23; }
                    23 => {
                        n_alloc =
                            n_alloc * 2 as sqlite3_int64 + n_col + 10 as sqlite3_int64;
                        __state = 24;
                    }
                    24 => {
                        az_new =
                            unsafe {
                                    sqlite3_realloc64(az_col as *mut (),
                                        n_alloc as u64 * core::mem::size_of::<*mut i8>() as u64)
                                } as *mut *mut i8;
                        __state = 25;
                    }
                    25 => {
                        if az_new == core::ptr::null_mut() {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => { az_col = az_new; __state = 28; }
                    27 => { __state = 2; }
                    28 => {
                        unsafe {
                            *az_col.offset(0 as isize) = core::ptr::null_mut()
                        };
                        __state = 21;
                    }
                    29 => {
                        if unsafe { *az_col.offset(n_col as isize) } ==
                                core::ptr::null_mut() {
                            __state = 31;
                        } else { __state = 30; }
                    }
                    30 => {
                        if unsafe { sqlite3_column_int(p_stmt, 5) } != 0 {
                            __state = 32;
                        } else { __state = 17; }
                    }
                    31 => { __state = 2; }
                    32 => {
                        { let __p = &mut n_pk; let __t = *__p; *__p += 1; __t };
                        __state = 33;
                    }
                    33 => {
                        if n_pk == 1 &&
                                unsafe {
                                        sqlite3_stricmp(unsafe { sqlite3_column_text(p_stmt, 2) } as
                                                *const i8, c"INTEGER".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                            __state = 34;
                        } else { __state = 35; }
                    }
                    34 => { is_ipk = 1; __state = 17; }
                    35 => { is_ipk = 0; __state = 17; }
                    36 => { p_stmt = core::ptr::null_mut(); __state = 37; }
                    37 => {
                        unsafe {
                            *az_col.offset((n_col + 1 as sqlite3_int64) as isize) =
                                core::ptr::null_mut()
                        };
                        __state = 38;
                    }
                    38 => {
                        if is_ipk != 0 { __state = 40; } else { __state = 39; }
                    }
                    39 => {
                        if preserve_rowid != 0 {
                            __state = 53;
                        } else { __state = 52; }
                    }
                    40 => {
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"SELECT 1 FROM pragma_index_list(%Q) WHERE origin=\'pk\'".as_ptr()
                                            as *mut i8 as *const i8, z_tab_1)
                            };
                        __state = 41;
                    }
                    41 => {
                        if z_sql == core::ptr::null_mut() {
                            __state = 43;
                        } else { __state = 42; }
                    }
                    42 => {
                        rc =
                            unsafe {
                                sqlite3_prepare_v2((*p).db, z_sql as *const i8, -1,
                                    &mut p_stmt, core::ptr::null_mut())
                            };
                        __state = 44;
                    }
                    43 => { __state = 2; }
                    44 => {
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        __state = 45;
                    }
                    45 => {
                        if rc != 0 { __state = 47; } else { __state = 46; }
                    }
                    46 => {
                        rc = unsafe { sqlite3_step(p_stmt) };
                        __state = 49;
                    }
                    47 => { free_column_list(az_col); __state = 48; }
                    48 => { return core::ptr::null_mut(); }
                    49 => { unsafe { sqlite3_finalize(p_stmt) }; __state = 50; }
                    50 => { p_stmt = core::ptr::null_mut(); __state = 51; }
                    51 => { preserve_rowid = (rc == 100) as i32; __state = 39; }
                    52 => { return az_col; }
                    53 => { __state = 54; }
                    54 => { __state = 55; }
                    55 => { j = 0; __state = 56; }
                    56 => { if j < 3 { __state = 57; } else { __state = 52; } }
                    57 => { i = 1; __state = 60; }
                    58 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 56;
                    }
                    59 => {
                        if i as sqlite3_int64 > n_col {
                            __state = 64;
                        } else { __state = 58; }
                    }
                    60 => {
                        if i as sqlite3_int64 <= n_col {
                            __state = 61;
                        } else { __state = 59; }
                    }
                    61 => {
                        if unsafe {
                                    sqlite3_stricmp(az_rowid[j as usize] as *const i8,
                                        unsafe { *az_col.offset(i as isize) } as *const i8)
                                } == 0 {
                            __state = 63;
                        } else { __state = 62; }
                    }
                    62 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 60;
                    }
                    63 => { __state = 59; }
                    64 => {
                        rc =
                            unsafe {
                                sqlite3_table_column_metadata((*p).db, core::ptr::null(),
                                    z_tab_1, az_rowid[j as usize] as *const i8,
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        __state = 65;
                    }
                    65 => {
                        if rc == 0 { __state = 67; } else { __state = 66; }
                    }
                    66 => { __state = 52; }
                    67 => {
                        unsafe {
                            *az_col.offset(0 as isize) = az_rowid[j as usize]
                        };
                        __state = 66;
                    }
                    68 => { __state = 2; }
                    69 => { free_column_list(az_col); __state = 70; }
                    70 => {
                        {
                            let __p = &mut (*p).n_err;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 71;
                    }
                    71 => { (*p).rc = 7; __state = 72; }
                    72 => { return core::ptr::null_mut(); }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
unsafe extern "C" fn output_formatted(p: &DState, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    unsafe { (*p).x_callback.unwrap()(z as *const i8, (*p).p_arg) };
    unsafe { sqlite3_free(z as *mut ()) };
}
extern "C" fn unused_string(z: *const i8, z_a_1: *const i8, z_b_1: *const i8,
    z_buf_1: *mut i8) -> *const i8 {
    let mut i: u32 = 0 as u32;
    if unsafe { strstr(z, z_a_1) } == core::ptr::null_mut() { return z_a_1; }
    if unsafe { strstr(z, z_b_1) } == core::ptr::null_mut() { return z_b_1; }
    '__b6: loop {
        '__c6: loop {
            unsafe {
                sqlite3_snprintf(20, z_buf_1,
                    c"(%s%u)".as_ptr() as *mut i8 as *const i8, z_a_1,
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t })
            };
            break '__c6;
        }
        if !(unsafe { strstr(z, z_buf_1 as *const i8) } !=
                        core::ptr::null_mut()) {
            break '__b6;
        }
    }
    return z_buf_1 as *const i8;
}
extern "C" fn output_quoted_escaped_string(p: *mut DState, mut z: *const i8)
    -> () {
    let mut i: i32 = 0;
    let mut c: i8 = 0 as i8;
    {
        i = 0;
        '__b7: loop {
            if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as i32 != 0
                                    && c as i32 != '\'' as i32 && c as i32 != '\n' as i32 &&
                            c as i32 != '\r' as i32) {
                break '__b7;
            }
            '__c7: loop { break '__c7; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if c as i32 == 0 {
        unsafe {
            output_formatted(unsafe { &*p },
                c"\'%s\'".as_ptr() as *mut i8 as *const i8, z)
        };
    } else {
        let mut z_nl: *const i8 = core::ptr::null();
        let mut z_cr: *const i8 = core::ptr::null();
        let mut n_nl: i32 = 0;
        let mut n_cr: i32 = 0;
        let mut z_buf1: [i8; 20] = [0; 20];
        let mut z_buf2: [i8; 20] = [0; 20];
        {
            i = 0;
            '__b8: loop {
                if !(unsafe { *z.offset(i as isize) } != 0) { break '__b8; }
                '__c8: loop {
                    if unsafe { *z.offset(i as isize) } as i32 == '\n' as i32 {
                        { let __p = &mut n_nl; let __t = *__p; *__p += 1; __t };
                    }
                    if unsafe { *z.offset(i as isize) } as i32 == '\r' as i32 {
                        { let __p = &mut n_cr; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_nl != 0 {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(c"replace(".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).p_arg })
            };
            z_nl =
                unused_string(z, c"\\n".as_ptr() as *mut i8 as *const i8,
                    c"\\012".as_ptr() as *mut i8 as *const i8,
                    &raw mut z_buf1[0 as usize] as *mut i8);
        }
        if n_cr != 0 {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(c"replace(".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).p_arg })
            };
            z_cr =
                unused_string(z, c"\\r".as_ptr() as *mut i8 as *const i8,
                    c"\\015".as_ptr() as *mut i8 as *const i8,
                    &raw mut z_buf2[0 as usize] as *mut i8);
        }
        unsafe {
            (unsafe {
                    (*p).x_callback.unwrap()
                })(c"\'".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p).p_arg })
        };
        while unsafe { *z } != 0 {
            {
                i = 0;
                '__b10: loop {
                    if !({ c = unsafe { *z.offset(i as isize) } as i8; c } as
                                                    i32 != 0 && c as i32 != '\n' as i32 &&
                                        c as i32 != '\r' as i32 && c as i32 != '\'' as i32) {
                        break '__b10;
                    }
                    '__c10: loop { break '__c10; }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if c as i32 == '\'' as i32 {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
            if i != 0 {
                unsafe {
                    output_formatted(unsafe { &*p },
                        c"%.*s".as_ptr() as *mut i8 as *const i8, i, z)
                };
                {
                    let __n = i;
                    let __p = &mut z;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            }
            if c as i32 == '\'' as i32 {
                unsafe {
                    (unsafe {
                            (*p).x_callback.unwrap()
                        })(c"\'".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p).p_arg })
                };
                continue;
            }
            if c as i32 == 0 { break; }
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            if c as i32 == '\n' as i32 {
                unsafe {
                    (unsafe {
                            (*p).x_callback.unwrap()
                        })(z_nl, unsafe { (*p).p_arg })
                };
                continue;
            }
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(z_cr, unsafe { (*p).p_arg })
            };
        }
        unsafe {
            (unsafe {
                    (*p).x_callback.unwrap()
                })(c"\'".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p).p_arg })
        };
        if n_cr != 0 {
            unsafe {
                output_formatted(unsafe { &*p },
                    c",\'%s\',char(13))".as_ptr() as *mut i8 as *const i8, z_cr)
            };
        }
        if n_nl != 0 {
            unsafe {
                output_formatted(unsafe { &*p },
                    c",\'%s\',char(10))".as_ptr() as *mut i8 as *const i8, z_nl)
            };
        }
    }
}
extern "C" fn dump_callback(p_arg_1: *mut (), n_arg_1: i32,
    az_arg_1: *mut *mut i8, az_col_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    let mut z_table: *const i8 = core::ptr::null();
    let mut z_type: *const i8 = core::ptr::null();
    let mut z_sql: *const i8 = core::ptr::null();
    let p: *mut DState = p_arg_1 as *mut DState;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    { let _ = az_col_1; };
    if n_arg_1 != 3 { return 1; }
    z_table = unsafe { *az_arg_1.offset(0 as isize) } as *const i8;
    z_type = unsafe { *az_arg_1.offset(1 as isize) } as *const i8;
    z_sql = unsafe { *az_arg_1.offset(2 as isize) } as *const i8;
    if unsafe {
                strcmp(z_table,
                    c"sqlite_sequence".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        unsafe {
            (unsafe {
                    (*p).x_callback.unwrap()
                })(c"DELETE FROM sqlite_sequence;\n".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).p_arg })
        };
    } else if unsafe {
                sqlite3_strglob(c"sqlite_stat?".as_ptr() as *mut i8 as
                        *const i8, z_table)
            } == 0 {
        unsafe {
            (unsafe {
                    (*p).x_callback.unwrap()
                })(c"ANALYZE sqlite_schema;\n".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).p_arg })
        };
    } else if unsafe {
                strncmp(z_table, c"sqlite_".as_ptr() as *mut i8 as *const i8,
                    7 as u64)
            } == 0 {
        return 0;
    } else if unsafe {
                strncmp(z_sql,
                    c"CREATE VIRTUAL TABLE".as_ptr() as *mut i8 as *const i8,
                    20 as u64)
            } == 0 {
        if (unsafe { (*p).writable_schema } == 0) as i32 != 0 {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(c"PRAGMA writable_schema=ON;\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p).p_arg })
            };
            unsafe { (*p).writable_schema = 1 };
        }
        unsafe {
            output_formatted(unsafe { &*p },
                c"INSERT INTO sqlite_schema(type,name,tbl_name,rootpage,sql)VALUES(\'table\',\'%q\',\'%q\',0,\'%q\');".as_ptr()
                        as *mut i8 as *const i8, z_table, z_table, z_sql)
        };
        return 0;
    } else {
        if unsafe {
                    sqlite3_strglob(c"CREATE TABLE [\'\"]*".as_ptr() as *mut i8
                            as *const i8, z_sql)
                } == 0 {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(c"CREATE TABLE IF NOT EXISTS ".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p).p_arg })
            };
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(unsafe { z_sql.offset(13 as isize) },
                    unsafe { (*p).p_arg })
            };
        } else {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(z_sql, unsafe { (*p).p_arg })
            };
        }
        unsafe {
            (unsafe {
                    (*p).x_callback.unwrap()
                })(c";\n".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p).p_arg })
        };
    }
    if unsafe { strcmp(z_type, c"table".as_ptr() as *mut i8 as *const i8) } ==
            0 {
        let mut s_select: DText = unsafe { core::mem::zeroed() };
        let mut s_table: DText = unsafe { core::mem::zeroed() };
        let mut az_t_col: *mut *mut i8 = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut n_col: i32 = 0;
        az_t_col = table_column_list(unsafe { &mut *p }, z_table);
        if az_t_col == core::ptr::null_mut() { return 0; }
        init_text(&mut s_table);
        append_text(&mut s_table,
            c"INSERT INTO ".as_ptr() as *mut i8 as *const i8, 0 as i8);
        append_text(&mut s_table, z_table, quote_char(z_table));
        if !(unsafe { *az_t_col.offset(0 as isize) }).is_null() {
            append_text(&mut s_table, c"(".as_ptr() as *mut i8 as *const i8,
                0 as i8);
            append_text(&mut s_table,
                unsafe { *az_t_col.offset(0 as isize) } as *const i8,
                0 as i8);
            {
                i = 1;
                '__b11: loop {
                    if !(!(unsafe { *az_t_col.offset(i as isize) }).is_null()) {
                        break '__b11;
                    }
                    '__c11: loop {
                        append_text(&mut s_table,
                            c",".as_ptr() as *mut i8 as *const i8, 0 as i8);
                        append_text(&mut s_table,
                            unsafe { *az_t_col.offset(i as isize) } as *const i8,
                            quote_char(unsafe { *az_t_col.offset(i as isize) } as
                                    *const i8));
                        break '__c11;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            append_text(&mut s_table, c")".as_ptr() as *mut i8 as *const i8,
                0 as i8);
        }
        append_text(&mut s_table,
            c" VALUES(".as_ptr() as *mut i8 as *const i8, 0 as i8);
        init_text(&mut s_select);
        append_text(&mut s_select,
            c"SELECT ".as_ptr() as *mut i8 as *const i8, 0 as i8);
        if !(unsafe { *az_t_col.offset(0 as isize) }).is_null() {
            append_text(&mut s_select,
                unsafe { *az_t_col.offset(0 as isize) } as *const i8,
                0 as i8);
            append_text(&mut s_select, c",".as_ptr() as *mut i8 as *const i8,
                0 as i8);
        }
        {
            i = 1;
            '__b12: loop {
                if !(!(unsafe { *az_t_col.offset(i as isize) }).is_null()) {
                    break '__b12;
                }
                '__c12: loop {
                    append_text(&mut s_select,
                        unsafe { *az_t_col.offset(i as isize) } as *const i8,
                        quote_char(unsafe { *az_t_col.offset(i as isize) } as
                                *const i8));
                    if !(unsafe {
                                        *az_t_col.offset((i + 1) as isize)
                                    }).is_null() {
                        append_text(&mut s_select,
                            c",".as_ptr() as *mut i8 as *const i8, 0 as i8);
                    }
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        n_col = i;
        if unsafe { *az_t_col.offset(0 as isize) } == core::ptr::null_mut() {
            { let __p = &mut n_col; let __t = *__p; *__p -= 1; __t };
        }
        free_column_list(az_t_col);
        append_text(&mut s_select, c" FROM ".as_ptr() as *mut i8 as *const i8,
            0 as i8);
        append_text(&mut s_select, z_table, quote_char(z_table));
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p).db },
                    s_select.z as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            {
                let __p = unsafe { &mut (*p).n_err };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p).rc } == 0 { unsafe { (*p).rc = rc }; }
        } else {
            while 100 == unsafe { sqlite3_step(p_stmt) } {
                unsafe {
                    (unsafe {
                            (*p).x_callback.unwrap()
                        })(s_table.z as *const i8, unsafe { (*p).p_arg })
                };
                {
                    i = 0;
                    '__b14: loop {
                        if !(i < n_col) { break '__b14; }
                        '__c14: loop {
                            if i != 0 {
                                unsafe {
                                    (unsafe {
                                            (*p).x_callback.unwrap()
                                        })(c",".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).p_arg })
                                };
                            }
                            '__s15:
                                {
                                match unsafe { sqlite3_column_type(p_stmt, i) } {
                                    1 => {
                                        {
                                            unsafe {
                                                output_formatted(unsafe { &*p },
                                                    c"%lld".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_int64(p_stmt, i) })
                                            };
                                            break '__s15;
                                        }
                                        {
                                            let mut r: f64 =
                                                unsafe { sqlite3_column_double(p_stmt, i) };
                                            let mut ur: sqlite3_uint64 = 0 as sqlite3_uint64;
                                            unsafe {
                                                memcpy(&raw mut ur as *mut (), &raw mut r as *const (),
                                                    core::mem::size_of::<f64>() as u64)
                                            };
                                            if ur == 9218868437227405312i64 as u64 {
                                                unsafe {
                                                    (unsafe {
                                                            (*p).x_callback.unwrap()
                                                        })(c"1e999".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { (*p).p_arg })
                                                };
                                            } else if ur == 18442240474082181120u64 {
                                                unsafe {
                                                    (unsafe {
                                                            (*p).x_callback.unwrap()
                                                        })(c"-1e999".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { (*p).p_arg })
                                                };
                                            } else {
                                                unsafe {
                                                    output_formatted(unsafe { &*p },
                                                        c"%!.20g".as_ptr() as *mut i8 as *const i8, r)
                                                };
                                            }
                                            break '__s15;
                                        }
                                        {
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"NULL".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                        {
                                            output_quoted_escaped_string(p,
                                                unsafe { sqlite3_column_text(p_stmt, i) } as *const i8);
                                            break '__s15;
                                        }
                                        {
                                            let n_byte: i32 =
                                                unsafe { sqlite3_column_bytes(p_stmt, i) };
                                            let a: *const u8 =
                                                unsafe { sqlite3_column_blob(p_stmt, i) } as *mut u8 as
                                                    *const u8;
                                            let mut j: i32 = 0;
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"x\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            {
                                                j = 0;
                                                '__b16: loop {
                                                    if !(j < n_byte) { break '__b16; }
                                                    '__c16: loop {
                                                        let mut z_word: [i8; 3] = [0; 3];
                                                        z_word[0 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 >>
                                                                                    4 & 15) as isize)
                                                            };
                                                        z_word[1 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 &
                                                                                15) as isize)
                                                            };
                                                        z_word[2 as usize] = 0 as i8;
                                                        unsafe {
                                                            (unsafe {
                                                                    (*p).x_callback.unwrap()
                                                                })(&raw mut z_word[0 as usize] as *mut i8 as *const i8,
                                                                unsafe { (*p).p_arg })
                                                        };
                                                        break '__c16;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                    }
                                    2 => {
                                        {
                                            let mut r: f64 =
                                                unsafe { sqlite3_column_double(p_stmt, i) };
                                            let mut ur: sqlite3_uint64 = 0 as sqlite3_uint64;
                                            unsafe {
                                                memcpy(&raw mut ur as *mut (), &raw mut r as *const (),
                                                    core::mem::size_of::<f64>() as u64)
                                            };
                                            if ur == 9218868437227405312i64 as u64 {
                                                unsafe {
                                                    (unsafe {
                                                            (*p).x_callback.unwrap()
                                                        })(c"1e999".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { (*p).p_arg })
                                                };
                                            } else if ur == 18442240474082181120u64 {
                                                unsafe {
                                                    (unsafe {
                                                            (*p).x_callback.unwrap()
                                                        })(c"-1e999".as_ptr() as *mut i8 as *const i8,
                                                        unsafe { (*p).p_arg })
                                                };
                                            } else {
                                                unsafe {
                                                    output_formatted(unsafe { &*p },
                                                        c"%!.20g".as_ptr() as *mut i8 as *const i8, r)
                                                };
                                            }
                                            break '__s15;
                                        }
                                        {
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"NULL".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                        {
                                            output_quoted_escaped_string(p,
                                                unsafe { sqlite3_column_text(p_stmt, i) } as *const i8);
                                            break '__s15;
                                        }
                                        {
                                            let n_byte: i32 =
                                                unsafe { sqlite3_column_bytes(p_stmt, i) };
                                            let a: *const u8 =
                                                unsafe { sqlite3_column_blob(p_stmt, i) } as *mut u8 as
                                                    *const u8;
                                            let mut j: i32 = 0;
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"x\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            {
                                                j = 0;
                                                '__b16: loop {
                                                    if !(j < n_byte) { break '__b16; }
                                                    '__c16: loop {
                                                        let mut z_word: [i8; 3] = [0; 3];
                                                        z_word[0 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 >>
                                                                                    4 & 15) as isize)
                                                            };
                                                        z_word[1 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 &
                                                                                15) as isize)
                                                            };
                                                        z_word[2 as usize] = 0 as i8;
                                                        unsafe {
                                                            (unsafe {
                                                                    (*p).x_callback.unwrap()
                                                                })(&raw mut z_word[0 as usize] as *mut i8 as *const i8,
                                                                unsafe { (*p).p_arg })
                                                        };
                                                        break '__c16;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                    }
                                    5 => {
                                        {
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"NULL".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                        {
                                            output_quoted_escaped_string(p,
                                                unsafe { sqlite3_column_text(p_stmt, i) } as *const i8);
                                            break '__s15;
                                        }
                                        {
                                            let n_byte: i32 =
                                                unsafe { sqlite3_column_bytes(p_stmt, i) };
                                            let a: *const u8 =
                                                unsafe { sqlite3_column_blob(p_stmt, i) } as *mut u8 as
                                                    *const u8;
                                            let mut j: i32 = 0;
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"x\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            {
                                                j = 0;
                                                '__b16: loop {
                                                    if !(j < n_byte) { break '__b16; }
                                                    '__c16: loop {
                                                        let mut z_word: [i8; 3] = [0; 3];
                                                        z_word[0 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 >>
                                                                                    4 & 15) as isize)
                                                            };
                                                        z_word[1 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 &
                                                                                15) as isize)
                                                            };
                                                        z_word[2 as usize] = 0 as i8;
                                                        unsafe {
                                                            (unsafe {
                                                                    (*p).x_callback.unwrap()
                                                                })(&raw mut z_word[0 as usize] as *mut i8 as *const i8,
                                                                unsafe { (*p).p_arg })
                                                        };
                                                        break '__c16;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                    }
                                    3 => {
                                        {
                                            output_quoted_escaped_string(p,
                                                unsafe { sqlite3_column_text(p_stmt, i) } as *const i8);
                                            break '__s15;
                                        }
                                        {
                                            let n_byte: i32 =
                                                unsafe { sqlite3_column_bytes(p_stmt, i) };
                                            let a: *const u8 =
                                                unsafe { sqlite3_column_blob(p_stmt, i) } as *mut u8 as
                                                    *const u8;
                                            let mut j: i32 = 0;
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"x\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            {
                                                j = 0;
                                                '__b16: loop {
                                                    if !(j < n_byte) { break '__b16; }
                                                    '__c16: loop {
                                                        let mut z_word: [i8; 3] = [0; 3];
                                                        z_word[0 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 >>
                                                                                    4 & 15) as isize)
                                                            };
                                                        z_word[1 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 &
                                                                                15) as isize)
                                                            };
                                                        z_word[2 as usize] = 0 as i8;
                                                        unsafe {
                                                            (unsafe {
                                                                    (*p).x_callback.unwrap()
                                                                })(&raw mut z_word[0 as usize] as *mut i8 as *const i8,
                                                                unsafe { (*p).p_arg })
                                                        };
                                                        break '__c16;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                    }
                                    4 => {
                                        {
                                            let n_byte: i32 =
                                                unsafe { sqlite3_column_bytes(p_stmt, i) };
                                            let a: *const u8 =
                                                unsafe { sqlite3_column_blob(p_stmt, i) } as *mut u8 as
                                                    *const u8;
                                            let mut j: i32 = 0;
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"x\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            {
                                                j = 0;
                                                '__b16: loop {
                                                    if !(j < n_byte) { break '__b16; }
                                                    '__c16: loop {
                                                        let mut z_word: [i8; 3] = [0; 3];
                                                        z_word[0 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 >>
                                                                                    4 & 15) as isize)
                                                            };
                                                        z_word[1 as usize] =
                                                            unsafe {
                                                                *(c"0123456789abcdef".as_ptr() as
                                                                            *mut i8).offset((unsafe { *a.offset(j as isize) } as i32 &
                                                                                15) as isize)
                                                            };
                                                        z_word[2 as usize] = 0 as i8;
                                                        unsafe {
                                                            (unsafe {
                                                                    (*p).x_callback.unwrap()
                                                                })(&raw mut z_word[0 as usize] as *mut i8 as *const i8,
                                                                unsafe { (*p).p_arg })
                                                        };
                                                        break '__c16;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            unsafe {
                                                (unsafe {
                                                        (*p).x_callback.unwrap()
                                                    })(c"\'".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { (*p).p_arg })
                                            };
                                            break '__s15;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            break '__c14;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    (unsafe {
                            (*p).x_callback.unwrap()
                        })(c");\n".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p).p_arg })
                };
            }
        }
        unsafe { sqlite3_finalize(p_stmt) };
        free_text(&mut s_table);
        free_text(&mut s_select);
    }
    return 0;
}
unsafe extern "C" fn output_sql_from_query(p: *mut DState,
    z_select_1: *const i8, mut __va0: ...) -> () {
    let mut p_select: *mut sqlite3_stmt = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut n_result: i32 = 0;
    let mut i: i32 = 0;
    let mut z: *const i8 = core::ptr::null();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vmprintf(z_select_1, ap) };
    ();
    if z_sql == core::ptr::null_mut() {
        unsafe { (*p).rc = 7 };
        {
            let __p = unsafe { &mut (*p).n_err };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return;
    }
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8, -1,
                &mut p_select, core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc != 0 || (p_select).is_null() as i32 != 0 {
        unsafe {
            output_formatted(unsafe { &*p },
                c"/**** ERROR: (%d) %s *****/\n".as_ptr() as *mut i8 as
                    *const i8, rc,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
        };
        {
            let __p = unsafe { &mut (*p).n_err };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return;
    }
    rc = unsafe { sqlite3_step(p_select) };
    n_result = unsafe { sqlite3_column_count(p_select) };
    while rc == 100 {
        z = unsafe { sqlite3_column_text(p_select, 0) } as *const i8;
        unsafe {
            (unsafe { (*p).x_callback.unwrap() })(z, unsafe { (*p).p_arg })
        };
        {
            i = 1;
            '__b18: loop {
                if !(i < n_result) { break '__b18; }
                '__c18: loop {
                    unsafe {
                        (unsafe {
                                (*p).x_callback.unwrap()
                            })(c",".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).p_arg })
                    };
                    unsafe {
                        (unsafe {
                                (*p).x_callback.unwrap()
                            })(unsafe { sqlite3_column_text(p_select, i) } as *const i8,
                            unsafe { (*p).p_arg })
                    };
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if z == core::ptr::null() {
            z = c"".as_ptr() as *mut i8 as *const i8;
        }
        while unsafe { *z.offset(0 as isize) } != 0 &&
                (unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 ||
                    unsafe { *z.offset(1 as isize) } as i32 != '-' as i32) {
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z.offset(0 as isize) } != 0 {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(c"\n;\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).p_arg })
            };
        } else {
            unsafe {
                (unsafe {
                        (*p).x_callback.unwrap()
                    })(c";\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p).p_arg })
            };
        }
        rc = unsafe { sqlite3_step(p_select) };
    }
    rc = unsafe { sqlite3_finalize(p_select) };
    if rc != 0 {
        unsafe {
            output_formatted(unsafe { &*p },
                c"/**** ERROR: (%d) %s *****/\n".as_ptr() as *mut i8 as
                    *const i8, rc,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
        };
        if rc & 255 != 11 {
            {
                let __p = unsafe { &mut (*p).n_err };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}
unsafe extern "C" fn run_schema_dump_query(p: *mut DState,
    z_query_1: *const i8, mut __va0: ...) -> () {
    let mut z_err: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z = unsafe { sqlite3_vmprintf(z_query_1, ap) };
    ();
    unsafe {
        sqlite3_exec(unsafe { (*p).db }, z as *const i8, Some(dump_callback),
            p as *mut (), &mut z_err)
    };
    unsafe { sqlite3_free(z as *mut ()) };
    if !(z_err).is_null() {
        unsafe {
            output_formatted(unsafe { &*p },
                c"/****** %s ******/\n".as_ptr() as *mut i8 as *const i8,
                z_err)
        };
        unsafe { sqlite3_free(z_err as *mut ()) };
        {
            let __p = unsafe { &mut (*p).n_err };
            let __t = *__p;
            *__p += 1;
            __t
        };
        z_err = core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_dump(db: *mut sqlite3, z_schema_1: *const i8,
    z_table_1: *const i8,
    x_callback_1: Option<unsafe extern "C" fn(*const i8, *mut ()) -> i32>,
    p_arg_1: *mut ()) -> i32 {
    let mut x: DState = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut x as *mut (), 0,
            core::mem::size_of::<DState>() as u64)
    };
    x.rc =
        unsafe {
            sqlite3_exec(db, c"BEGIN".as_ptr() as *mut i8 as *const i8, None,
                core::ptr::null_mut(), core::ptr::null_mut())
        };
    if x.rc != 0 { return x.rc; }
    x.db = db;
    x.x_callback = x_callback_1;
    x.p_arg = p_arg_1;
    unsafe {
        x_callback_1.unwrap()(c"PRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION;\n".as_ptr()
                    as *mut i8 as *const i8, p_arg_1)
    };
    if z_table_1 == core::ptr::null() {
        unsafe {
            run_schema_dump_query(&mut x,
                c"SELECT name, type, sql FROM \"%w\".sqlite_schema WHERE sql NOT NULL AND type==\'table\' AND name!=\'sqlite_sequence\'".as_ptr()
                        as *mut i8 as *const i8, z_schema_1)
        };
        unsafe {
            run_schema_dump_query(&mut x,
                c"SELECT name, type, sql FROM \"%w\".sqlite_schema WHERE name==\'sqlite_sequence\'".as_ptr()
                        as *mut i8 as *const i8, z_schema_1)
        };
        unsafe {
            output_sql_from_query(&mut x,
                c"SELECT sql FROM sqlite_schema WHERE sql NOT NULL AND type IN (\'index\',\'trigger\',\'view\')".as_ptr()
                        as *mut i8 as *const i8, 0)
        };
    } else {
        unsafe {
            run_schema_dump_query(&mut x,
                c"SELECT name, type, sql FROM \"%w\".sqlite_schema WHERE tbl_name=%Q COLLATE nocase AND type==\'table\'  AND sql NOT NULL".as_ptr()
                        as *mut i8 as *const i8, z_schema_1, z_table_1)
        };
        unsafe {
            output_sql_from_query(&mut x,
                c"SELECT sql FROM \"%w\".sqlite_schema WHERE sql NOT NULL  AND type IN (\'index\',\'trigger\',\'view\')  AND tbl_name=%Q COLLATE nocase".as_ptr()
                        as *mut i8 as *const i8, z_schema_1, z_table_1)
        };
    }
    if x.writable_schema != 0 {
        unsafe {
            x_callback_1.unwrap()(c"PRAGMA writable_schema=OFF;\n".as_ptr() as
                        *mut i8 as *const i8, p_arg_1)
        };
    }
    unsafe {
        x_callback_1.unwrap()(if x.n_err != 0 {
                    c"ROLLBACK; -- due to errors\n".as_ptr() as *mut i8
                } else { c"COMMIT;\n".as_ptr() as *mut i8 } as *const i8,
            p_arg_1)
    };
    unsafe {
        sqlite3_exec(db, c"COMMIT".as_ptr() as *mut i8 as *const i8, None,
            core::ptr::null_mut(), core::ptr::null_mut())
    };
    return x.rc;
}
static mut az_rowid: [*mut i8; 3] =
    [c"rowid".as_ptr() as *mut i8, c"_rowid_".as_ptr() as *mut i8,
            c"oid".as_ptr() as *mut i8];
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
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn isalpha(_c: i32)
    -> i32;
    fn isalnum(_c: i32)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}