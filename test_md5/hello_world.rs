type __darwin_size_t = u64;
type Tcl_Command = *mut Tcl_Command_;
type ClientData = *mut ();
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
struct MD5Context {
    is_init: i32,
    buf: [u32; 4],
    bits: [u32; 2],
    in_: [u8; 64],
}
extern "C" fn byte_reverse(mut buf: *mut u8, mut longs: u32) -> () {
    let mut t: u32 = 0 as u32;
    '__b0: loop {
        '__c0: loop {
            t =
                (((unsafe { *buf.offset(3 as isize) } as u32) << 8 |
                                    unsafe { *buf.offset(2 as isize) } as u32) as u32) << 16 |
                    ((unsafe { *buf.offset(1 as isize) } as u32) << 8 |
                        unsafe { *buf.offset(0 as isize) } as u32);
            unsafe { *(buf as *mut u32) = t };
            {
                let __n = 4;
                let __p = &mut buf;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            break '__c0;
        }
        if !({ let __p = &mut longs; *__p -= 1; *__p } != 0) { break '__b0; }
    }
}
extern "C" fn md5_transform(buf: *mut u32, in__1: *const u32) -> () {
    let mut a: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    let mut c: u32 = 0 as u32;
    let mut d: u32 = 0 as u32;
    a = unsafe { *buf.offset(0 as isize) };
    b = unsafe { *buf.offset(1 as isize) };
    c = unsafe { *buf.offset(2 as isize) };
    d = unsafe { *buf.offset(3 as isize) };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(0 as isize) } as u32 + 3614090360u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(1 as isize) } as u32 + 3905402710u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(2 as isize) } as u32 +
                    606105819 as u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(3 as isize) } as u32 + 3250441966u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(4 as isize) } as u32 + 4118548399u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(5 as isize) } as u32 +
                    1200080426 as u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(6 as isize) } as u32 + 2821735955u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(7 as isize) } as u32 + 4249261313u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(8 as isize) } as u32 +
                    1770035416 as u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(9 as isize) } as u32 + 2336552879u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(10 as isize) } as u32 +
                    4294925233u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(11 as isize) } as u32 +
                    2304563134u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (d ^ b & (c ^ d)) +
                        unsafe { *in__1.offset(12 as isize) } as u32 +
                    1804603682 as u32;
            a = a << 7 | a >> 32 - 7
        };
        a += b
    };
    {
        {
            d +=
                (c ^ a & (b ^ c)) +
                        unsafe { *in__1.offset(13 as isize) } as u32 +
                    4254626195u32;
            d = d << 12 | d >> 32 - 12
        };
        d += a
    };
    {
        {
            c +=
                (b ^ d & (a ^ b)) +
                        unsafe { *in__1.offset(14 as isize) } as u32 +
                    2792965006u32;
            c = c << 17 | c >> 32 - 17
        };
        c += d
    };
    {
        {
            b +=
                (a ^ c & (d ^ a)) +
                        unsafe { *in__1.offset(15 as isize) } as u32 +
                    1236535329 as u32;
            b = b << 22 | b >> 32 - 22
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(1 as isize) } as u32 + 4129170786u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(6 as isize) } as u32 + 3225465664u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(11 as isize) } as u32 +
                    643717713 as u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(0 as isize) } as u32 + 3921069994u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(5 as isize) } as u32 + 3593408605u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(10 as isize) } as u32 +
                    38016083 as u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(15 as isize) } as u32 +
                    3634488961u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(4 as isize) } as u32 + 3889429448u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(9 as isize) } as u32 +
                    568446438 as u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(14 as isize) } as u32 +
                    3275163606u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(3 as isize) } as u32 + 4107603335u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(8 as isize) } as u32 +
                    1163531501 as u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (c ^ d & (b ^ c)) +
                        unsafe { *in__1.offset(13 as isize) } as u32 +
                    2850285829u32;
            a = a << 5 | a >> 32 - 5
        };
        a += b
    };
    {
        {
            d +=
                (b ^ c & (a ^ b)) +
                        unsafe { *in__1.offset(2 as isize) } as u32 + 4243563512u32;
            d = d << 9 | d >> 32 - 9
        };
        d += a
    };
    {
        {
            c +=
                (a ^ b & (d ^ a)) +
                        unsafe { *in__1.offset(7 as isize) } as u32 +
                    1735328473 as u32;
            c = c << 14 | c >> 32 - 14
        };
        c += d
    };
    {
        {
            b +=
                (d ^ a & (c ^ d)) +
                        unsafe { *in__1.offset(12 as isize) } as u32 +
                    2368359562u32;
            b = b << 20 | b >> 32 - 20
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(5 as isize) } as u32 +
                    4294588738u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(8 as isize) } as u32 +
                    2272392833u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(11 as isize) } as u32 +
                    1839030562 as u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(14 as isize) } as u32 +
                    4259657740u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(1 as isize) } as u32 +
                    2763975236u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(4 as isize) } as u32 +
                    1272893353 as u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(7 as isize) } as u32 +
                    4139469664u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(10 as isize) } as u32 +
                    3200236656u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(13 as isize) } as u32 +
                    681279174 as u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(0 as isize) } as u32 +
                    3936430074u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(3 as isize) } as u32 +
                    3572445317u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(6 as isize) } as u32 +
                    76029189 as u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (b ^ c ^ d) + unsafe { *in__1.offset(9 as isize) } as u32 +
                    3654602809u32;
            a = a << 4 | a >> 32 - 4
        };
        a += b
    };
    {
        {
            d +=
                (a ^ b ^ c) + unsafe { *in__1.offset(12 as isize) } as u32 +
                    3873151461u32;
            d = d << 11 | d >> 32 - 11
        };
        d += a
    };
    {
        {
            c +=
                (d ^ a ^ b) + unsafe { *in__1.offset(15 as isize) } as u32 +
                    530742520 as u32;
            c = c << 16 | c >> 32 - 16
        };
        c += d
    };
    {
        {
            b +=
                (c ^ d ^ a) + unsafe { *in__1.offset(2 as isize) } as u32 +
                    3299628645u32;
            b = b << 23 | b >> 32 - 23
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(0 as isize) } as u32 +
                    4096336452u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(7 as isize) } as u32 +
                    1126891415 as u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(14 as isize) } as u32
                    + 2878612391u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(5 as isize) } as u32 +
                    4237533241u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(12 as isize) } as u32
                    + 1700485571 as u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(3 as isize) } as u32 +
                    2399980690u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(10 as isize) } as u32
                    + 4293915773u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(1 as isize) } as u32 +
                    2240044497u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(8 as isize) } as u32 +
                    1873313359 as u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(15 as isize) } as u32
                    + 4264355552u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(6 as isize) } as u32 +
                    2734768916u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(13 as isize) } as u32
                    + 1309151649 as u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    {
        {
            a +=
                (c ^ (b | !d)) + unsafe { *in__1.offset(4 as isize) } as u32 +
                    4149444226u32;
            a = a << 6 | a >> 32 - 6
        };
        a += b
    };
    {
        {
            d +=
                (b ^ (a | !c)) + unsafe { *in__1.offset(11 as isize) } as u32
                    + 3174756917u32;
            d = d << 10 | d >> 32 - 10
        };
        d += a
    };
    {
        {
            c +=
                (a ^ (d | !b)) + unsafe { *in__1.offset(2 as isize) } as u32 +
                    718787259 as u32;
            c = c << 15 | c >> 32 - 15
        };
        c += d
    };
    {
        {
            b +=
                (d ^ (c | !a)) + unsafe { *in__1.offset(9 as isize) } as u32 +
                    3951481745u32;
            b = b << 21 | b >> 32 - 21
        };
        b += c
    };
    unsafe { *buf.offset(0 as isize) += a };
    unsafe { *buf.offset(1 as isize) += b };
    unsafe { *buf.offset(2 as isize) += c };
    unsafe { *buf.offset(3 as isize) += d };
}
extern "C" fn md5_init_1(ctx: &mut MD5Context) -> () {
    (*ctx).is_init = 1;
    (*ctx).buf[0 as usize] = 1732584193 as u32;
    (*ctx).buf[1 as usize] = 4023233417u32;
    (*ctx).buf[2 as usize] = 2562383102u32;
    (*ctx).buf[3 as usize] = 271733878 as u32;
    (*ctx).bits[0 as usize] = 0 as u32;
    (*ctx).bits[1 as usize] = 0 as u32;
}
extern "C" fn md5_update(ctx: &mut MD5Context, mut buf: *const u8,
    mut len: u32) -> () {
    let mut t: u32 = 0 as u32;
    t = (*ctx).bits[0 as usize];
    if {
                (*ctx).bits[0 as usize] = t + ((len as u32) << 3);
                (*ctx).bits[0 as usize]
            } < t {
        {
            let __p = &mut (*ctx).bits[1 as usize];
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    (*ctx).bits[1 as usize] += len >> 29;
    t = t >> 3 & 63 as u32;
    if t != 0 {
        let p: *mut u8 =
            unsafe {
                (&raw mut (*ctx).in_[0 as usize] as *mut u8).add(t as usize)
            };
        t = 64 as u32 - t;
        if len < t {
            unsafe { memcpy(p as *mut (), buf as *const (), len as u64) };
            return;
        }
        unsafe { memcpy(p as *mut (), buf as *const (), t as u64) };
        byte_reverse(&raw mut (*ctx).in_[0 as usize] as *mut u8, 16 as u32);
        md5_transform(&raw mut (*ctx).buf[0 as usize] as *mut u32,
            &raw mut (*ctx).in_[0 as usize] as *mut u32 as *const u32);
        {
            let __n = t;
            let __p = &mut buf;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        len -= t;
    }
    while len >= 64 as u32 {
        unsafe {
            memcpy(&raw mut (*ctx).in_[0 as usize] as *mut u8 as *mut (),
                buf as *const (), 64 as u64)
        };
        byte_reverse(&raw mut (*ctx).in_[0 as usize] as *mut u8, 16 as u32);
        md5_transform(&raw mut (*ctx).buf[0 as usize] as *mut u32,
            &raw mut (*ctx).in_[0 as usize] as *mut u32 as *const u32);
        {
            let __n = 64;
            let __p = &mut buf;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        len -= 64 as u32;
    }
    unsafe {
        memcpy(&raw mut (*ctx).in_[0 as usize] as *mut u8 as *mut (),
            buf as *const (), len as u64)
    };
}
extern "C" fn md5_final(digest: *mut u8, ctx: &mut MD5Context) -> () {
    let mut count: u32 = 0 as u32;
    let mut p: *mut u8 = core::ptr::null_mut();
    count = (*ctx).bits[0 as usize] >> 3 & 63 as u32;
    p =
        unsafe {
            (&raw mut (*ctx).in_[0 as usize] as *mut u8).add(count as usize)
        };
    unsafe {
        *{
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = 128 as u8
    };
    count = (64 - 1) as u32 - count;
    if count < 8 as u32 {
        unsafe { memset(p as *mut (), 0, count as u64) };
        byte_reverse(&raw mut (*ctx).in_[0 as usize] as *mut u8, 16 as u32);
        md5_transform(&raw mut (*ctx).buf[0 as usize] as *mut u32,
            &raw mut (*ctx).in_[0 as usize] as *mut u32 as *const u32);
        unsafe {
            memset(&raw mut (*ctx).in_[0 as usize] as *mut u8 as *mut (), 0,
                56 as u64)
        };
    } else { unsafe { memset(p as *mut (), 0, (count - 8 as u32) as u64) }; }
    byte_reverse(&raw mut (*ctx).in_[0 as usize] as *mut u8, 14 as u32);
    unsafe {
        memcpy(unsafe {
                    (&raw mut (*ctx).in_[0 as usize] as
                            *mut u8).offset((14 * 4) as isize)
                } as *mut (),
            &raw mut (*ctx).bits[0 as usize] as *mut u32 as *const (),
            8 as u64)
    };
    md5_transform(&raw mut (*ctx).buf[0 as usize] as *mut u32,
        &raw mut (*ctx).in_[0 as usize] as *mut u32 as *const u32);
    byte_reverse(&raw mut (*ctx).buf[0 as usize] as *mut u8, 4 as u32);
    unsafe {
        memcpy(digest as *mut (),
            &raw mut (*ctx).buf[0 as usize] as *mut u32 as *const (),
            16 as u64)
    };
}
extern "C" fn md5_digest_to_base16(digest: *mut u8, z_buf_1: *mut i8) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = { i = 0; i };
        '__b2: loop {
            if !(i < 16) { break '__b2; }
            '__c2: loop {
                let a: i32 = unsafe { *digest.offset(i as isize) } as i32;
                unsafe {
                    *z_buf_1.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = z_encode[(a >> 4 & 15) as usize] as i8
                };
                unsafe {
                    *z_buf_1.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = z_encode[(a & 15) as usize] as i8
                };
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z_buf_1.offset(j as isize) = 0 as i8 };
}
extern "C" fn md5_digest_to_base10x8(digest: *mut u8, z_digest_1: *mut i8)
    -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: u32 = 0 as u32;
    {
        i = { j = 0; j };
        '__b3: loop {
            if !(i < 16) { break '__b3; }
            '__c3: loop {
                x =
                    (unsafe { *digest.offset(i as isize) } as i32 * 256 +
                            unsafe { *digest.offset((i + 1) as isize) } as i32) as u32;
                if i > 0 {
                    unsafe {
                        *z_digest_1.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = '-' as i32 as i8
                    };
                }
                unsafe {
                    sqlite3_snprintf(50 - j,
                        unsafe { &mut *z_digest_1.offset(j as isize) },
                        c"%05u".as_ptr() as *mut i8 as *const i8, x)
                };
                j += 5;
                break '__c3;
            }
            i += 2;
        }
    }
    unsafe { *z_digest_1.offset(j as isize) = 0 as i8 };
}
extern "C" fn md5_cmd(cd: *mut (), interp: *mut Tcl_Interp, argc: i32,
    argv: *mut *const i8) -> i32 {
    let mut ctx: MD5Context = unsafe { core::mem::zeroed() };
    let mut digest: [u8; 16] = [0; 16];
    let mut z_buf: [i8; 50] = [0; 50];
    let mut converter: Option<unsafe extern "C" fn(*mut u8, *mut i8) -> ()> =
        None;
    if argc != 2 {
        unsafe {
            Tcl_AppendResult(interp,
                c"wrong # args: should be \"".as_ptr() as *mut i8,
                unsafe { *argv.offset(0 as isize) },
                c" TEXT\"".as_ptr() as *mut i8, 0 as *mut i8)
        };
        return 1;
    }
    md5_init_1(&mut ctx);
    md5_update(&mut ctx,
        unsafe { *argv.offset(1 as isize) } as *mut u8 as *const u8,
        unsafe { strlen(unsafe { *argv.offset(1 as isize) }) } as u32);
    md5_final(&raw mut digest[0 as usize] as *mut u8, &mut ctx);
    converter =
        Some(unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut u8, *mut i8)
                            -> ()>(cd as *const ())
            });
    unsafe {
        converter.unwrap()(&raw mut digest[0 as usize] as *mut u8,
            &raw mut z_buf[0 as usize] as *mut i8)
    };
    unsafe {
        Tcl_AppendResult(interp, &raw mut z_buf[0 as usize] as *mut i8,
            0 as *mut i8)
    };
    return 0;
}
extern "C" fn md5file_cmd(cd: *mut (), interp: *mut Tcl_Interp, argc: i32,
    argv: *mut *const i8) -> i32 {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut ofst: i32 = 0;
    let mut amt: i32 = 0;
    let mut ctx: MD5Context = unsafe { core::mem::zeroed() };
    let mut converter: Option<unsafe extern "C" fn(*mut u8, *mut i8) -> ()> =
        None;
    let mut digest: [u8; 16] = [0; 16];
    let mut z_buf: [i8; 10240] = [0; 10240];
    if argc != 2 && argc != 4 {
        unsafe {
            Tcl_AppendResult(interp,
                c"wrong # args: should be \"".as_ptr() as *mut i8,
                unsafe { *argv.offset(0 as isize) },
                c" FILENAME [OFFSET AMT]\"".as_ptr() as *mut i8, 0 as *mut i8)
        };
        return 1;
    }
    if argc == 4 {
        ofst = unsafe { atoi(unsafe { *argv.offset(2 as isize) }) };
        amt = unsafe { atoi(unsafe { *argv.offset(3 as isize) }) };
    } else { ofst = 0; amt = 2147483647; }
    in_ =
        unsafe {
            fopen(unsafe { *argv.offset(1 as isize) },
                c"rb".as_ptr() as *mut i8 as *const i8)
        };
    if in_ == core::ptr::null_mut() {
        unsafe {
            Tcl_AppendResult(interp,
                c"unable to open file \"".as_ptr() as *mut i8,
                unsafe { *argv.offset(1 as isize) },
                c"\" for reading".as_ptr() as *mut i8, 0 as *mut i8)
        };
        return 1;
    }
    unsafe { fseek(in_, ofst as i64, 0) };
    md5_init_1(&mut ctx);
    while amt > 0 {
        let mut n: i32 = 0;
        n =
            unsafe {
                    fread(&raw mut z_buf[0 as usize] as *mut i8 as *mut (),
                        1 as u64,
                        if core::mem::size_of::<[i8; 10240]>() as u64 <= amt as u64
                            {
                            core::mem::size_of::<[i8; 10240]>() as u64
                        } else { amt as u64 }, in_)
                } as i32;
        if n <= 0 { break; }
        md5_update(&mut ctx,
            &raw mut z_buf[0 as usize] as *mut u8 as *const u8, n as u32);
        amt -= n;
    }
    unsafe { fclose(in_) };
    md5_final(&raw mut digest[0 as usize] as *mut u8, &mut ctx);
    converter =
        Some(unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut u8, *mut i8)
                            -> ()>(cd as *const ())
            });
    unsafe {
        converter.unwrap()(&raw mut digest[0 as usize] as *mut u8,
            &raw mut z_buf[0 as usize] as *mut i8)
    };
    unsafe {
        Tcl_AppendResult(interp, &raw mut z_buf[0 as usize] as *mut i8,
            0 as *mut i8)
    };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn md5_init(interp: *mut Tcl_Interp) -> i32 {
    unsafe {
        Tcl_CreateCommand(interp, c"md5".as_ptr() as *mut i8 as *const i8,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), *mut Tcl_Interp, i32,
                            *mut *const i8) -> i32>(md5_cmd as *const ())
            }, md5_digest_to_base16 as ClientData,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
            })
    };
    unsafe {
        Tcl_CreateCommand(interp,
            c"md5-10x8".as_ptr() as *mut i8 as *const i8,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), *mut Tcl_Interp, i32,
                            *mut *const i8) -> i32>(md5_cmd as *const ())
            }, md5_digest_to_base10x8 as ClientData,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
            })
    };
    unsafe {
        Tcl_CreateCommand(interp, c"md5file".as_ptr() as *mut i8 as *const i8,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), *mut Tcl_Interp, i32,
                            *mut *const i8) -> i32>(md5file_cmd as *const ())
            }, md5_digest_to_base16 as ClientData,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
            })
    };
    unsafe {
        Tcl_CreateCommand(interp,
            c"md5file-10x8".as_ptr() as *mut i8 as *const i8,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), *mut Tcl_Interp, i32,
                            *mut *const i8) -> i32>(md5file_cmd as *const ())
            }, md5_digest_to_base10x8 as ClientData,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
            })
    };
    return 0;
}
extern "C" fn md5step(context: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut MD5Context = core::ptr::null_mut();
    let mut i: i32 = 0;
    if argc < 1 { return; }
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<MD5Context>() as i32)
            } as *mut MD5Context;
    if p == core::ptr::null_mut() { return; }
    if (unsafe { (*p).is_init } == 0) as i32 != 0 {
        md5_init_1(unsafe { &mut *p });
    }
    {
        i = 0;
        '__b5: loop {
            if !(i < argc) { break '__b5; }
            '__c5: loop {
                let z_data: *const i8 =
                    unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *mut i8 as *const i8;
                if !(z_data).is_null() {
                    md5_update(unsafe { &mut *p },
                        z_data as *mut u8 as *const u8,
                        unsafe { strlen(z_data) } as i32 as u32);
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn md5finalize(context: *mut sqlite3_context) -> () {
    let mut p: *mut MD5Context = core::ptr::null_mut();
    let mut digest: [u8; 16] = [0; 16];
    let mut z_buf: [i8; 33] = [0; 33];
    p =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<MD5Context>() as i32)
            } as *mut MD5Context;
    md5_final(&raw mut digest[0 as usize] as *mut u8, unsafe { &mut *p });
    md5_digest_to_base16(&raw mut digest[0 as usize] as *mut u8,
        &raw mut z_buf[0 as usize] as *mut i8);
    unsafe {
        sqlite3_result_text(context,
            &raw mut z_buf[0 as usize] as *mut i8 as *const i8, -1,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}
#[unsafe(no_mangle)]
pub extern "C" fn md5_register(db: *mut sqlite3, pz_err_msg_1: *const *mut i8,
    p_thunk_1: *const sqlite3_api_routines) -> i32 {
    let rc: i32 =
        unsafe {
            sqlite3_create_function(db,
                c"md5sum".as_ptr() as *mut i8 as *const i8, -1, 1,
                core::ptr::null_mut(), None, Some(md5step), Some(md5finalize))
        };
    unsafe {
        sqlite3_overload_function(db,
            c"md5sum".as_ptr() as *mut i8 as *const i8, -1)
    };
    return rc;
}
static z_encode: [i8; 17] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 97 as i8, 98 as i8, 99 as i8,
            100 as i8, 101 as i8, 102 as i8, 0 as i8];
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
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn atoi(_: *const i8)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn Tcl_CreateCommand(interp: *mut Tcl_Interp, cmdName: *const i8,
    proc:
        unsafe extern "C" fn(*mut (), *mut Tcl_Interp, i32, *mut *const i8)
            -> i32, clientData: ClientData,
    deleteProc: unsafe extern "C" fn(*mut ()) -> ())
    -> Tcl_Command;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;
#[repr(C)]
#[derive(Copy, Clone)]
struct Tcl_Command_ {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Tcl_Interp {
    _opaque: [u8; 0],
}