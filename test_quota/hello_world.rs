type __darwin_size_t = u64;
type __int64_t = i64;
type __darwin_off_t = __int64_t;
type off_t = __darwin_off_t;
type __darwin_time_t = i64;
type time_t = __darwin_time_t;
type __int32_t = i32;
type __darwin_dev_t = __int32_t;
type dev_t = __darwin_dev_t;
type __uint16_t = u16;
type __darwin_mode_t = __uint16_t;
type mode_t = __darwin_mode_t;
type nlink_t = __uint16_t;
type __uint64_t = u64;
type __darwin_ino64_t = __uint64_t;
type __uint32_t = u32;
type __darwin_uid_t = __uint32_t;
type uid_t = __darwin_uid_t;
type __darwin_gid_t = __uint32_t;
type gid_t = __darwin_gid_t;
type __darwin_blkcnt_t = __int64_t;
type blkcnt_t = __darwin_blkcnt_t;
type __darwin_blksize_t = __int32_t;
type blksize_t = __darwin_blksize_t;
#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atimespec: timespec,
    st_mtimespec: timespec,
    st_ctimespec: timespec,
    st_birthtimespec: timespec,
    st_size: i64,
    st_blocks: i64,
    st_blksize: i32,
    st_flags: u32,
    st_gen: u32,
    st_lspare: i32,
    st_qspare: [i64; 2],
}
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
struct anon_s0 {
    p_orig_vfs: *mut sqlite3_vfs,
    s_this_vfs: sqlite3_vfs,
    s_io_methods_v1: sqlite3_io_methods,
    s_io_methods_v2: sqlite3_io_methods,
    is_initialized: i32,
    p_mutex: *mut sqlite3_mutex,
    p_group: *mut quotaGroup,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct quotaGroup {
    z_pattern: *const i8,
    i_limit: sqlite3_int64,
    i_size: sqlite3_int64,
    x_callback: Option<unsafe extern "C" fn(*const i8, *mut i64, i64, *mut ())
        -> ()>,
    p_arg: *mut (),
    x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_next: *mut quotaGroup,
    pp_prev: *mut *mut quotaGroup,
    p_files: *mut quotaFile,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct quotaFile {
    z_filename: *mut i8,
    p_group: *mut quotaGroup,
    i_size: sqlite3_int64,
    n_ref: i32,
    delete_on_close: i32,
    p_next: *mut quotaFile,
    pp_prev: *mut *mut quotaFile,
}
static mut g_quota: anon_s0 = unsafe { core::mem::zeroed() };
#[repr(C)]
#[derive(Copy, Clone)]
struct quotaConn {
    base: sqlite3_file,
    p_file: *mut quotaFile,
}
extern "C" fn quota_enter() -> () {
    unsafe { unsafe { sqlite3_mutex_enter(g_quota.p_mutex) }; }
}
extern "C" fn quota_strglob(mut z_glob_1: *const i8, mut z: *const i8)
    -> i32 {
    let mut c: i32 = 0;
    let mut c2: i32 = 0;
    let mut cx: i32 = 0;
    let mut invert: i32 = 0;
    let mut seen: i32 = 0;
    while {
                c =
                    unsafe {
                            *{
                                    let __p = &mut z_glob_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32;
                c
            } != 0 {
        if c == '*' as i32 {
            while {
                            c =
                                unsafe {
                                        *{
                                                let __p = &mut z_glob_1;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                    } as i32;
                            c
                        } == '*' as i32 || c == '?' as i32 {
                if c == '?' as i32 &&
                        unsafe {
                                    *{
                                            let __p = &mut z;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as i32 == 0 {
                    return 0;
                }
            }
            if c == 0 {
                return 1;
            } else if c == '[' as i32 {
                while unsafe { *z } != 0 &&
                        quota_strglob(unsafe { z_glob_1.offset(-(1 as isize)) }, z)
                            == 0 {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                return (unsafe { *z } as i32 != 0) as i32;
            }
            cx = if c == '/' as i32 { '\\' as i32 } else { c };
            while {
                        c2 =
                            unsafe {
                                    *{
                                            let __p = &mut z;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as i32;
                        c2
                    } != 0 {
                while c2 != c && c2 != cx {
                    c2 =
                        unsafe {
                                *{
                                        let __p = &mut z;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as i32;
                    if c2 == 0 { return 0; }
                }
                if quota_strglob(z_glob_1, z) != 0 { return 1; }
            }
            return 0;
        } else if c == '?' as i32 {
            if unsafe {
                            *{
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32 == 0 {
                return 0;
            }
        } else if c == '[' as i32 {
            let mut prior_c: i32 = 0;
            seen = 0;
            invert = 0;
            c =
                unsafe {
                        *{
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as i32;
            if c == 0 { return 0; }
            c2 =
                unsafe {
                        *{
                                let __p = &mut z_glob_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as i32;
            if c2 == '^' as i32 {
                invert = 1;
                c2 =
                    unsafe {
                            *{
                                    let __p = &mut z_glob_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32;
            }
            if c2 == ']' as i32 {
                if c == ']' as i32 { seen = 1; }
                c2 =
                    unsafe {
                            *{
                                    let __p = &mut z_glob_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32;
            }
            while c2 != 0 && c2 != ']' as i32 {
                if c2 == '-' as i32 &&
                                unsafe { *z_glob_1.offset(0 as isize) } as i32 != ']' as i32
                            && unsafe { *z_glob_1.offset(0 as isize) } as i32 != 0 &&
                        prior_c > 0 {
                    c2 =
                        unsafe {
                                *{
                                        let __p = &mut z_glob_1;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as i32;
                    if c >= prior_c && c <= c2 { seen = 1; }
                    prior_c = 0;
                } else { if c == c2 { seen = 1; } prior_c = c2; }
                c2 =
                    unsafe {
                            *{
                                    let __p = &mut z_glob_1;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32;
            }
            if c2 == 0 || seen ^ invert == 0 { return 0; }
        } else if c == '/' as i32 {
            if unsafe { *z.offset(0 as isize) } as i32 != '/' as i32 &&
                    unsafe { *z.offset(0 as isize) } as i32 != '\\' as i32 {
                return 0;
            }
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else {
            if c !=
                    unsafe {
                            *{
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32 {
                return 0;
            }
        }
    }
    return (unsafe { *z } as i32 == 0) as i32;
}
extern "C" fn quota_group_find(z_filename_1: *const i8) -> *mut quotaGroup {
    unsafe {
        let mut p: *mut quotaGroup = core::ptr::null_mut();
        {
            p = g_quota.p_group;
            '__b6: loop {
                if !(!(p).is_null() &&
                                quota_strglob(unsafe { (*p).z_pattern }, z_filename_1) == 0)
                    {
                    break '__b6;
                }
                '__c6: loop { break '__c6; }
                p = unsafe { (*p).p_next };
            }
        }
        return p;
    }
}
extern "C" fn quota_sub_open(p_conn_1: *mut sqlite3_file)
    -> *mut sqlite3_file {
    let p: *mut quotaConn = p_conn_1 as *mut quotaConn;
    return unsafe { &raw mut *p.offset(1 as isize) } as *mut sqlite3_file;
}
extern "C" fn quota_find_file(p_group_1: *mut quotaGroup, z_name_1: *const i8,
    create_flag_1: i32) -> *mut quotaFile {
    let mut p_file: *mut quotaFile = unsafe { (*p_group_1).p_files };
    while !(p_file).is_null() &&
            unsafe {
                    strcmp(unsafe { (*p_file).z_filename } as *const i8,
                        z_name_1)
                } != 0 {
        p_file = unsafe { (*p_file).p_next };
    }
    if p_file == core::ptr::null_mut() && create_flag_1 != 0 {
        let n_name: i32 =
            (unsafe { strlen(z_name_1) } & 1073741823 as u64) as i32;
        p_file =
            unsafe {
                    sqlite3_malloc((core::mem::size_of::<quotaFile>() as u64 +
                                    n_name as u64 + 1 as u64) as i32)
                } as *mut quotaFile;
        if !(p_file).is_null() {
            unsafe {
                memset(p_file as *mut (), 0,
                    core::mem::size_of::<quotaFile>() as u64)
            };
            unsafe {
                (*p_file).z_filename =
                    unsafe { &raw mut *p_file.offset(1 as isize) } as *mut i8
            };
            unsafe {
                memcpy(unsafe { (*p_file).z_filename } as *mut (),
                    z_name_1 as *const (), (n_name + 1) as u64)
            };
            unsafe { (*p_file).p_next = unsafe { (*p_group_1).p_files } };
            if !(unsafe { (*p_group_1).p_files }).is_null() {
                unsafe {
                    (*unsafe { (*p_group_1).p_files }).pp_prev =
                        unsafe { &mut (*p_file).p_next }
                };
            }
            unsafe {
                (*p_file).pp_prev = unsafe { &mut (*p_group_1).p_files }
            };
            unsafe { (*p_group_1).p_files = p_file };
            unsafe { (*p_file).p_group = p_group_1 };
        }
    }
    return p_file;
}
extern "C" fn quota_leave() -> () {
    unsafe { unsafe { sqlite3_mutex_leave(g_quota.p_mutex) }; }
}
extern "C" fn quota_open(p_vfs_1: *mut sqlite3_vfs, z_name_1: *const i8,
    p_conn_1: *mut sqlite3_file, flags: i32, p_out_flags_1: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_quota_open: *mut quotaConn = core::ptr::null_mut();
        let mut p_file: *mut quotaFile = core::ptr::null_mut();
        let mut p_group: *mut quotaGroup = core::ptr::null_mut();
        let mut p_sub_open: *mut sqlite3_file = core::ptr::null_mut();
        let p_orig_vfs: *mut sqlite3_vfs = g_quota.p_orig_vfs;
        if flags & (256 | 524288) == 0 {
            return unsafe {
                    (unsafe {
                            (*p_orig_vfs).x_open.unwrap()
                        })(p_orig_vfs, z_name_1, p_conn_1, flags, p_out_flags_1)
                };
        }
        quota_enter();
        p_group = quota_group_find(z_name_1);
        if p_group == core::ptr::null_mut() {
            rc =
                unsafe {
                    (unsafe {
                            (*p_orig_vfs).x_open.unwrap()
                        })(p_orig_vfs, z_name_1, p_conn_1, flags, p_out_flags_1)
                };
        } else {
            p_quota_open = p_conn_1 as *mut quotaConn;
            p_sub_open = quota_sub_open(p_conn_1);
            rc =
                unsafe {
                    (unsafe {
                            (*p_orig_vfs).x_open.unwrap()
                        })(p_orig_vfs, z_name_1, p_sub_open, flags, p_out_flags_1)
                };
            if rc == 0 {
                p_file = quota_find_file(p_group, z_name_1, 1);
                if p_file == core::ptr::null_mut() {
                    quota_leave();
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_sub_open).p_methods }).x_close.unwrap()
                            })(p_sub_open)
                    };
                    return 7;
                }
                unsafe {
                    (*p_file).delete_on_close = (flags & 8 != 0) as i32
                };
                {
                    let __p = unsafe { &mut (*p_file).n_ref };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe { (*p_quota_open).p_file = p_file };
                if unsafe { (*unsafe { (*p_sub_open).p_methods }).i_version }
                            as i32 == 1 {
                    unsafe {
                        (*p_quota_open).base.p_methods =
                            &raw mut g_quota.s_io_methods_v1 as
                                *const sqlite3_io_methods
                    };
                } else {
                    unsafe {
                        (*p_quota_open).base.p_methods =
                            &raw mut g_quota.s_io_methods_v2 as
                                *const sqlite3_io_methods
                    };
                }
            }
        }
        quota_leave();
        return rc;
    }
}
extern "C" fn quota_remove_file(p_file_1: *mut quotaFile) -> () {
    let p_group: *mut quotaGroup = unsafe { (*p_file_1).p_group };
    unsafe { (*p_group).i_size -= unsafe { (*p_file_1).i_size } };
    unsafe {
        *unsafe { (*p_file_1).pp_prev } = unsafe { (*p_file_1).p_next }
    };
    if !(unsafe { (*p_file_1).p_next }).is_null() {
        unsafe {
            (*unsafe { (*p_file_1).p_next }).pp_prev =
                unsafe { (*p_file_1).pp_prev }
        };
    }
    unsafe { sqlite3_free(p_file_1 as *mut ()) };
}
extern "C" fn quota_group_open_file_count(p_group_1: &quotaGroup) -> i32 {
    let mut n: i32 = 0;
    let mut p_file: *const quotaFile =
        (*p_group_1).p_files as *const quotaFile;
    while !(p_file).is_null() {
        if unsafe { (*p_file).n_ref } != 0 {
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        }
        p_file = unsafe { (*p_file).p_next };
    }
    return n;
}
extern "C" fn quota_remove_all_files(p_group_1: &quotaGroup) -> () {
    while !((*p_group_1).p_files).is_null() {
        if !(unsafe { (*(*p_group_1).p_files).n_ref } == 0) as i32 as i64 != 0
            {
            unsafe {
                __assert_rtn(c"quotaRemoveAllFiles".as_ptr() as *const i8,
                    c"test_quota.c".as_ptr() as *mut i8 as *const i8, 218,
                    c"pGroup->pFiles->nRef==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        quota_remove_file((*p_group_1).p_files);
    }
}
extern "C" fn quota_group_deref(p_group_1: *mut quotaGroup) -> () {
    if unsafe { (*p_group_1).i_limit } == 0 as i64 &&
            quota_group_open_file_count(unsafe { &*p_group_1 }) == 0 {
        quota_remove_all_files(unsafe { &*p_group_1 });
        unsafe {
            *unsafe { (*p_group_1).pp_prev } = unsafe { (*p_group_1).p_next }
        };
        if !(unsafe { (*p_group_1).p_next }).is_null() {
            unsafe {
                (*unsafe { (*p_group_1).p_next }).pp_prev =
                    unsafe { (*p_group_1).pp_prev }
            };
        }
        if unsafe { (*p_group_1).x_destroy.is_some() } {
            unsafe {
                (unsafe {
                        (*p_group_1).x_destroy.unwrap()
                    })(unsafe { (*p_group_1).p_arg })
            };
        }
        unsafe { sqlite3_free(p_group_1 as *mut ()) };
    }
}
extern "C" fn quota_delete(p_vfs_1: *mut sqlite3_vfs, z_name_1: *const i8,
    sync_dir_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_file: *mut quotaFile = core::ptr::null_mut();
        let mut p_group: *mut quotaGroup = core::ptr::null_mut();
        let p_orig_vfs: *mut sqlite3_vfs = g_quota.p_orig_vfs;
        rc =
            unsafe {
                (unsafe {
                        (*p_orig_vfs).x_delete.unwrap()
                    })(p_orig_vfs, z_name_1, sync_dir_1)
            };
        if rc == 0 {
            quota_enter();
            p_group = quota_group_find(z_name_1);
            if !(p_group).is_null() {
                p_file = quota_find_file(p_group, z_name_1, 0);
                if !(p_file).is_null() {
                    if unsafe { (*p_file).n_ref } != 0 {
                        unsafe { (*p_file).delete_on_close = 1 };
                    } else {
                        quota_remove_file(p_file);
                        quota_group_deref(p_group);
                    }
                }
            }
            quota_leave();
        }
        return rc;
    }
}
extern "C" fn quota_close(p_conn_1: *mut sqlite3_file) -> i32 {
    unsafe {
        let p: *const quotaConn =
            p_conn_1 as *mut quotaConn as *const quotaConn;
        let p_file: *mut quotaFile = unsafe { (*p).p_file };
        let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
        let mut rc: i32 = 0;
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_sub_open).p_methods }).x_close.unwrap()
                    })(p_sub_open)
            };
        quota_enter();
        {
            let __p = unsafe { &mut (*p_file).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p_file).n_ref } == 0 {
            let p_group: *mut quotaGroup = unsafe { (*p_file).p_group };
            if unsafe { (*p_file).delete_on_close } != 0 {
                unsafe {
                    (unsafe {
                            (*g_quota.p_orig_vfs).x_delete.unwrap()
                        })(g_quota.p_orig_vfs,
                        unsafe { (*p_file).z_filename } as *const i8, 0)
                };
                quota_remove_file(p_file);
            }
            quota_group_deref(p_group);
        }
        quota_leave();
        return rc;
    }
}
extern "C" fn quota_read(p_conn_1: *mut sqlite3_file, p_buf_1: *mut (),
    i_amt_1: i32, i_ofst_1: sqlite3_int64) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_read.unwrap()
                })(p_sub_open, p_buf_1, i_amt_1, i_ofst_1)
        };
}
extern "C" fn quota_write(p_conn_1: *mut sqlite3_file, p_buf_1: *const (),
    i_amt_1: i32, i_ofst_1: sqlite3_int64) -> i32 {
    let p: *const quotaConn = p_conn_1 as *mut quotaConn as *const quotaConn;
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    let i_end: sqlite3_int64 = i_ofst_1 + i_amt_1 as sqlite3_int64;
    let mut p_group: *mut quotaGroup = core::ptr::null_mut();
    let p_file: *mut quotaFile = unsafe { (*p).p_file };
    let mut sz_new: sqlite3_int64 = 0 as sqlite3_int64;
    if unsafe { (*p_file).i_size } < i_end {
        p_group = unsafe { (*p_file).p_group };
        quota_enter();
        sz_new =
            unsafe { (*p_group).i_size } - unsafe { (*p_file).i_size } +
                i_end;
        if sz_new > unsafe { (*p_group).i_limit } &&
                unsafe { (*p_group).i_limit } > 0 as i64 {
            if unsafe { (*p_group).x_callback.is_some() } {
                unsafe {
                    (unsafe {
                            (*p_group).x_callback.unwrap()
                        })(unsafe { (*p_file).z_filename } as *const i8,
                        unsafe { &mut (*p_group).i_limit }, sz_new,
                        unsafe { (*p_group).p_arg })
                };
            }
            if sz_new > unsafe { (*p_group).i_limit } &&
                    unsafe { (*p_group).i_limit } > 0 as i64 {
                quota_leave();
                return 13;
            }
        }
        unsafe { (*p_group).i_size = sz_new };
        unsafe { (*p_file).i_size = i_end };
        quota_leave();
    }
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_write.unwrap()
                })(p_sub_open, p_buf_1, i_amt_1, i_ofst_1)
        };
}
extern "C" fn quota_truncate(p_conn_1: *mut sqlite3_file, size: sqlite3_int64)
    -> i32 {
    let p: *const quotaConn = p_conn_1 as *mut quotaConn as *const quotaConn;
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    let rc: i32 =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_truncate.unwrap()
                })(p_sub_open, size)
        };
    let p_file: *mut quotaFile = unsafe { (*p).p_file };
    let mut p_group: *mut quotaGroup = core::ptr::null_mut();
    if rc == 0 {
        quota_enter();
        p_group = unsafe { (*p_file).p_group };
        unsafe { (*p_group).i_size -= unsafe { (*p_file).i_size } };
        unsafe { (*p_file).i_size = size };
        unsafe { (*p_group).i_size += size };
        quota_leave();
    }
    return rc;
}
extern "C" fn quota_sync(p_conn_1: *mut sqlite3_file, flags: i32) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_sync.unwrap()
                })(p_sub_open, flags)
        };
}
extern "C" fn quota_file_size(p_conn_1: *mut sqlite3_file,
    p_size_1: *mut sqlite3_int64) -> i32 {
    let p: *const quotaConn = p_conn_1 as *mut quotaConn as *const quotaConn;
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    let p_file: *mut quotaFile = unsafe { (*p).p_file };
    let mut p_group: *mut quotaGroup = core::ptr::null_mut();
    let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
    let mut rc: i32 = 0;
    rc =
        unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_file_size.unwrap()
                })(p_sub_open, &mut sz)
        };
    if rc == 0 {
        quota_enter();
        p_group = unsafe { (*p_file).p_group };
        unsafe { (*p_group).i_size -= unsafe { (*p_file).i_size } };
        unsafe { (*p_file).i_size = sz };
        unsafe { (*p_group).i_size += sz };
        quota_leave();
        unsafe { *p_size_1 = sz };
    }
    return rc;
}
extern "C" fn quota_lock(p_conn_1: *mut sqlite3_file, lock: i32) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_lock.unwrap()
                })(p_sub_open, lock)
        };
}
extern "C" fn quota_unlock(p_conn_1: *mut sqlite3_file, lock: i32) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_unlock.unwrap()
                })(p_sub_open, lock)
        };
}
extern "C" fn quota_check_reserved_lock(p_conn_1: *mut sqlite3_file,
    p_res_out_1: *mut i32) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_sub_open).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(p_sub_open, p_res_out_1)
        };
}
extern "C" fn quota_file_control(p_conn_1: *mut sqlite3_file, op: i32,
    p_arg_1: *mut ()) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    let rc: i32 =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_sub_open).p_methods
                                    }).x_file_control.unwrap()
                })(p_sub_open, op, p_arg_1)
        };
    if op == 12 && rc == 0 {
        unsafe {
            *(p_arg_1 as *mut *mut i8) =
                unsafe {
                    sqlite3_mprintf(c"quota/%z".as_ptr() as *mut i8 as
                            *const i8, unsafe { *(p_arg_1 as *mut *mut i8) })
                }
        };
    }
    return rc;
}
extern "C" fn quota_sector_size(p_conn_1: *mut sqlite3_file) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_sector_size.unwrap()
                })(p_sub_open)
        };
}
extern "C" fn quota_device_characteristics(p_conn_1: *mut sqlite3_file)
    -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*p_sub_open).p_methods
                                    }).x_device_characteristics.unwrap()
                })(p_sub_open)
        };
}
extern "C" fn quota_shm_map(p_conn_1: *mut sqlite3_file, i_region_1: i32,
    sz_region_1: i32, b_extend_1: i32, pp: *mut *mut ()) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_shm_map.unwrap()
                })(p_sub_open, i_region_1, sz_region_1, b_extend_1, pp)
        };
}
extern "C" fn quota_shm_lock(p_conn_1: *mut sqlite3_file, ofst: i32, n: i32,
    flags: i32) -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_shm_lock.unwrap()
                })(p_sub_open, ofst, n, flags)
        };
}
extern "C" fn quota_shm_barrier(p_conn_1: *mut sqlite3_file) -> () {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    unsafe {
        (unsafe {
                (*unsafe { (*p_sub_open).p_methods }).x_shm_barrier.unwrap()
            })(p_sub_open)
    };
}
extern "C" fn quota_shm_unmap(p_conn_1: *mut sqlite3_file, delete_flag_1: i32)
    -> i32 {
    let p_sub_open: *mut sqlite3_file = quota_sub_open(p_conn_1);
    return unsafe {
            (unsafe {
                    (*unsafe { (*p_sub_open).p_methods }).x_shm_unmap.unwrap()
                })(p_sub_open, delete_flag_1)
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_initialize(z_orig_vfs_name: *const i8,
    make_default: i32) -> i32 {
    unsafe {
        let mut p_orig_vfs: *mut sqlite3_vfs = core::ptr::null_mut();
        if g_quota.is_initialized != 0 { return 21; }
        p_orig_vfs = unsafe { sqlite3_vfs_find(z_orig_vfs_name) };
        if p_orig_vfs == core::ptr::null_mut() { return 1; }
        if !(p_orig_vfs != &raw mut g_quota.s_this_vfs as *mut sqlite3_vfs) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3_quota_initialize".as_ptr() as
                        *const i8, c"test_quota.c".as_ptr() as *mut i8 as *const i8,
                    752,
                    c"pOrigVfs!=&gQuota.sThisVfs".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        g_quota.p_mutex = unsafe { sqlite3_mutex_alloc(0) };
        if (g_quota.p_mutex).is_null() as i32 != 0 { return 7; }
        g_quota.is_initialized = 1;
        g_quota.p_orig_vfs = p_orig_vfs;
        g_quota.s_this_vfs = unsafe { core::ptr::read(p_orig_vfs) };
        g_quota.s_this_vfs.x_open = Some(quota_open);
        g_quota.s_this_vfs.x_delete = Some(quota_delete);
        g_quota.s_this_vfs.sz_os_file +=
            core::mem::size_of::<quotaConn>() as i32;
        g_quota.s_this_vfs.z_name = c"quota".as_ptr() as *mut i8 as *const i8;
        g_quota.s_io_methods_v1.i_version = 1;
        g_quota.s_io_methods_v1.x_close = Some(quota_close);
        g_quota.s_io_methods_v1.x_read = Some(quota_read);
        g_quota.s_io_methods_v1.x_write = Some(quota_write);
        g_quota.s_io_methods_v1.x_truncate = Some(quota_truncate);
        g_quota.s_io_methods_v1.x_sync = Some(quota_sync);
        g_quota.s_io_methods_v1.x_file_size = Some(quota_file_size);
        g_quota.s_io_methods_v1.x_lock = Some(quota_lock);
        g_quota.s_io_methods_v1.x_unlock = Some(quota_unlock);
        g_quota.s_io_methods_v1.x_check_reserved_lock =
            Some(quota_check_reserved_lock);
        g_quota.s_io_methods_v1.x_file_control = Some(quota_file_control);
        g_quota.s_io_methods_v1.x_sector_size = Some(quota_sector_size);
        g_quota.s_io_methods_v1.x_device_characteristics =
            Some(quota_device_characteristics);
        g_quota.s_io_methods_v2 = g_quota.s_io_methods_v1;
        g_quota.s_io_methods_v2.i_version = 2;
        g_quota.s_io_methods_v2.x_shm_map = Some(quota_shm_map);
        g_quota.s_io_methods_v2.x_shm_lock = Some(quota_shm_lock);
        g_quota.s_io_methods_v2.x_shm_barrier = Some(quota_shm_barrier);
        g_quota.s_io_methods_v2.x_shm_unmap = Some(quota_shm_unmap);
        unsafe {
            sqlite3_vfs_register(&mut g_quota.s_this_vfs, make_default)
        };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_shutdown() -> i32 {
    unsafe {
        let mut p_group: *mut quotaGroup = core::ptr::null_mut();
        if g_quota.is_initialized == 0 { return 21; }
        {
            p_group = g_quota.p_group;
            '__b10: loop {
                if !(!(p_group).is_null()) { break '__b10; }
                '__c10: loop {
                    if quota_group_open_file_count(unsafe { &*p_group }) > 0 {
                        return 21;
                    }
                    break '__c10;
                }
                p_group = unsafe { (*p_group).p_next };
            }
        }
        while !(g_quota.p_group).is_null() {
            p_group = g_quota.p_group;
            g_quota.p_group = unsafe { (*p_group).p_next };
            unsafe { (*p_group).i_limit = 0 as sqlite3_int64 };
            if !(quota_group_open_file_count(unsafe { &*p_group }) == 0) as
                            i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3_quota_shutdown".as_ptr() as
                            *const i8, c"test_quota.c".as_ptr() as *mut i8 as *const i8,
                        806,
                        c"quotaGroupOpenFileCount(pGroup)==0".as_ptr() as *mut i8 as
                            *const i8)
                }
            } else { { let _ = 0; } };
            quota_group_deref(p_group);
        }
        g_quota.is_initialized = 0;
        unsafe { sqlite3_mutex_free(g_quota.p_mutex) };
        unsafe { sqlite3_vfs_unregister(&mut g_quota.s_this_vfs) };
        unsafe { memset(&raw mut g_quota as *mut (), 0, 504) };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_set(z_pattern: *const i8,
    i_limit: sqlite3_int64,
    x_callback:
        Option<unsafe extern "C" fn(*const i8, *mut i64, i64, *mut ()) -> ()>,
    p_arg: *mut (), x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32 {
    unsafe {
        let mut p_group: *mut quotaGroup = core::ptr::null_mut();
        quota_enter();
        p_group = g_quota.p_group;
        while !(p_group).is_null() &&
                unsafe { strcmp(unsafe { (*p_group).z_pattern }, z_pattern) }
                    != 0 {
            p_group = unsafe { (*p_group).p_next };
        }
        if p_group == core::ptr::null_mut() {
            let n_pattern: i32 =
                (unsafe { strlen(z_pattern) } & 1073741823 as u64) as i32;
            if i_limit <= 0 as i64 { quota_leave(); return 0; }
            p_group =
                unsafe {
                        sqlite3_malloc((core::mem::size_of::<quotaGroup>() as u64 +
                                        n_pattern as u64 + 1 as u64) as i32)
                    } as *mut quotaGroup;
            if p_group == core::ptr::null_mut() { quota_leave(); return 7; }
            unsafe {
                memset(p_group as *mut (), 0,
                    core::mem::size_of::<quotaGroup>() as u64)
            };
            unsafe {
                (*p_group).z_pattern =
                    unsafe { &raw mut *p_group.offset(1 as isize) } as *mut i8
                        as *const i8
            };
            unsafe {
                memcpy(unsafe { (*p_group).z_pattern } as *mut i8 as *mut (),
                    z_pattern as *const (), (n_pattern + 1) as u64)
            };
            if !(g_quota.p_group).is_null() {
                unsafe {
                    (*g_quota.p_group).pp_prev =
                        unsafe { &mut (*p_group).p_next }
                };
            }
            unsafe { (*p_group).p_next = g_quota.p_group };
            unsafe { (*p_group).pp_prev = &mut g_quota.p_group };
            g_quota.p_group = p_group;
        }
        unsafe { (*p_group).i_limit = i_limit };
        unsafe { (*p_group).x_callback = x_callback };
        if unsafe { (*p_group).x_destroy.is_some() } &&
                unsafe { (*p_group).p_arg } != p_arg {
            unsafe {
                (unsafe {
                        (*p_group).x_destroy.unwrap()
                    })(unsafe { (*p_group).p_arg })
            };
        }
        unsafe { (*p_group).p_arg = p_arg };
        unsafe { (*p_group).x_destroy = x_destroy };
        quota_group_deref(p_group);
        quota_leave();
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_file(z_filename: *const i8) -> i32 {
    unsafe {
        let mut z_full: *mut i8 = core::ptr::null_mut();
        let mut fd: *mut sqlite3_file = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut out_flags: i32 = 0;
        let mut i_size: sqlite3_int64 = 0 as sqlite3_int64;
        let n_alloc: i32 =
            g_quota.s_this_vfs.sz_os_file + g_quota.s_this_vfs.mx_pathname +
                2;
        fd = unsafe { sqlite3_malloc(n_alloc) } as *mut sqlite3_file;
        if fd == core::ptr::null_mut() {
            rc = 7;
        } else {
            z_full =
                unsafe {
                    (fd as
                            *mut i8).offset(g_quota.s_this_vfs.sz_os_file as isize)
                };
            rc =
                unsafe {
                    (unsafe {
                            (*g_quota.p_orig_vfs).x_full_pathname.unwrap()
                        })(g_quota.p_orig_vfs, z_filename,
                        g_quota.s_this_vfs.mx_pathname + 1, z_full)
                };
        }
        if rc == 0 {
            unsafe {
                *z_full.add((unsafe { strlen(z_full as *const i8) } +
                                    1 as u64) as usize) = '\u{0}' as i32 as i8
            };
            rc =
                quota_open(&mut g_quota.s_this_vfs, z_full as *const i8, fd,
                    1 | 256, &mut out_flags);
            if rc == 0 {
                unsafe {
                    (unsafe {
                            (*unsafe { (*fd).p_methods }).x_file_size.unwrap()
                        })(fd, &mut i_size)
                };
                unsafe {
                    (unsafe {
                            (*unsafe { (*fd).p_methods }).x_close.unwrap()
                        })(fd)
                };
            } else if rc == 14 {
                let mut p_group: *mut quotaGroup = core::ptr::null_mut();
                let mut p_file: *mut quotaFile = core::ptr::null_mut();
                quota_enter();
                p_group = quota_group_find(z_full as *const i8);
                if !(p_group).is_null() {
                    p_file = quota_find_file(p_group, z_full as *const i8, 0);
                    if !(p_file).is_null() { quota_remove_file(p_file); }
                }
                quota_leave();
            }
        }
        unsafe { sqlite3_free(fd as *mut ()) };
        return rc;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct quota_FILE {
    f: *mut FILE,
    i_ofst: sqlite3_int64,
    p_file: *mut quotaFile,
}
extern "C" fn quota_mbcs_free(z_old_1: *const i8) -> () {}
extern "C" fn quota_utf8_to_mbcs(z_utf8_1: *const i8) -> *mut i8 {
    return z_utf8_1 as *mut i8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_fopen(z_filename_1: *const i8,
    z_mode_1: *const i8) -> *mut quota_FILE {
    unsafe {
        let mut p: *mut quota_FILE = core::ptr::null_mut();
        let mut z_full: *mut i8 = core::ptr::null_mut();
        let mut z_full_translated: *mut i8 = core::ptr::null_mut();
        '__b13: loop {
            '__c13: loop {
                let mut rc: i32 = 0;
                let mut p_group: *mut quotaGroup = core::ptr::null_mut();
                let mut p_file: *mut quotaFile = core::ptr::null_mut();
                z_full =
                    unsafe {
                            sqlite3_malloc(g_quota.s_this_vfs.mx_pathname + 1)
                        } as *mut i8;
                if z_full == core::ptr::null_mut() {
                    return core::ptr::null_mut();
                }
                rc =
                    unsafe {
                        (unsafe {
                                (*g_quota.p_orig_vfs).x_full_pathname.unwrap()
                            })(g_quota.p_orig_vfs, z_filename_1,
                            g_quota.s_this_vfs.mx_pathname + 1, z_full)
                    };
                if rc != 0 { break '__b13; }
                p =
                    unsafe {
                            sqlite3_malloc(core::mem::size_of::<quota_FILE>() as i32)
                        } as *mut quota_FILE;
                if p == core::ptr::null_mut() { break '__b13; }
                unsafe {
                    memset(p as *mut (), 0,
                        core::mem::size_of::<quota_FILE>() as u64)
                };
                z_full_translated = quota_utf8_to_mbcs(z_full as *const i8);
                if z_full_translated == core::ptr::null_mut() {
                    break '__b13;
                }
                unsafe {
                    (*p).f =
                        unsafe { fopen(z_full_translated as *const i8, z_mode_1) }
                };
                if unsafe { (*p).f } == core::ptr::null_mut() {
                    break '__b13;
                }
                quota_enter();
                p_group = quota_group_find(z_full as *const i8);
                if !(p_group).is_null() {
                    p_file = quota_find_file(p_group, z_full as *const i8, 1);
                    if p_file == core::ptr::null_mut() {
                        quota_leave();
                        break '__b13;
                    }
                    {
                        let __p = unsafe { &mut (*p_file).n_ref };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    unsafe { (*p).p_file = p_file };
                }
                quota_leave();
                unsafe { sqlite3_free(z_full as *mut ()) };
                return p;
                break '__c13;
            }
            if !(false) { break '__b13; }
        }
        quota_mbcs_free(z_full_translated as *const i8);
        unsafe { sqlite3_free(z_full as *mut ()) };
        if !(p).is_null() && !(unsafe { (*p).f }).is_null() {
            unsafe { fclose(unsafe { (*p).f }) };
        }
        unsafe { sqlite3_free(p as *mut ()) };
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_fread(p_buf_1: *mut (), size: u64, nmemb: u64,
    p: &quota_FILE) -> u64 {
    return unsafe { fread(p_buf_1, size, nmemb, (*p).f) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_fwrite(p_buf_1: *const (), size: u64,
    mut nmemb: u64, p: &quota_FILE) -> u64 {
    let mut i_ofst: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i_end: sqlite3_int64 = 0 as sqlite3_int64;
    let mut sz_new: sqlite3_int64 = 0 as sqlite3_int64;
    let mut p_file: *mut quotaFile = core::ptr::null_mut();
    let mut rc: u64 = 0 as u64;
    i_ofst = unsafe { ftell((*p).f) } as sqlite3_int64;
    i_end = (i_ofst as u64 + (size * nmemb) as u64) as sqlite3_int64;
    p_file = (*p).p_file;
    if !(p_file).is_null() && unsafe { (*p_file).i_size } < i_end {
        let p_group: *mut quotaGroup = unsafe { (*p_file).p_group };
        quota_enter();
        sz_new =
            unsafe { (*p_group).i_size } - unsafe { (*p_file).i_size } +
                i_end;
        if sz_new > unsafe { (*p_group).i_limit } &&
                unsafe { (*p_group).i_limit } > 0 as i64 {
            if unsafe { (*p_group).x_callback.is_some() } {
                unsafe {
                    (unsafe {
                            (*p_group).x_callback.unwrap()
                        })(unsafe { (*p_file).z_filename } as *const i8,
                        unsafe { &mut (*p_group).i_limit }, sz_new,
                        unsafe { (*p_group).p_arg })
                };
            }
            if sz_new > unsafe { (*p_group).i_limit } &&
                    unsafe { (*p_group).i_limit } > 0 as i64 {
                i_end =
                    unsafe { (*p_group).i_limit } - unsafe { (*p_group).i_size }
                        + unsafe { (*p_file).i_size };
                nmemb = ((i_end - i_ofst) as u64 / size as u64) as u64;
                i_end =
                    (i_ofst as u64 + (size * nmemb) as u64) as sqlite3_int64;
                sz_new =
                    unsafe { (*p_group).i_size } - unsafe { (*p_file).i_size } +
                        i_end;
            }
        }
        unsafe { (*p_group).i_size = sz_new };
        unsafe { (*p_file).i_size = i_end };
        quota_leave();
    } else { p_file = core::ptr::null_mut(); }
    rc = unsafe { fwrite(p_buf_1, size, nmemb, (*p).f) };
    if rc < nmemb && !(p_file).is_null() {
        let n_written: u64 = rc;
        let mut i_new_end: sqlite3_int64 =
            (i_ofst as u64 + (size * n_written) as u64) as sqlite3_int64;
        if i_new_end < i_end { i_new_end = i_end; }
        quota_enter();
        unsafe {
            (*unsafe { (*p_file).p_group }).i_size +=
                i_new_end - unsafe { (*p_file).i_size }
        };
        unsafe { (*p_file).i_size = i_new_end };
        quota_leave();
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_fflush(p: &quota_FILE, do_fsync_1: i32)
    -> i32 {
    let mut rc: i32 = 0;
    rc = unsafe { fflush((*p).f) };
    if rc == 0 && do_fsync_1 != 0 {
        rc = unsafe { fsync(unsafe { fileno((*p).f) }) };
    }
    return (rc != 0) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_fclose(p: *mut quota_FILE) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_file: *mut quotaFile = core::ptr::null_mut();
        rc = unsafe { fclose(unsafe { (*p).f }) };
        p_file = unsafe { (*p).p_file };
        if !(p_file).is_null() {
            quota_enter();
            {
                let __p = unsafe { &mut (*p_file).n_ref };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if unsafe { (*p_file).n_ref } == 0 {
                let p_group: *mut quotaGroup = unsafe { (*p_file).p_group };
                if unsafe { (*p_file).delete_on_close } != 0 {
                    unsafe {
                        (unsafe {
                                (*g_quota.p_orig_vfs).x_delete.unwrap()
                            })(g_quota.p_orig_vfs,
                            unsafe { (*p_file).z_filename } as *const i8, 0)
                    };
                    quota_remove_file(p_file);
                }
                quota_group_deref(p_group);
            }
            quota_leave();
        }
        unsafe { sqlite3_free(p as *mut ()) };
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_fseek(p: &quota_FILE, offset: i64,
    whence: i32) -> i32 {
    return unsafe { fseek((*p).f, offset, whence) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_rewind(p: &quota_FILE) -> () {
    unsafe { rewind((*p).f) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_ftell(p: &quota_FILE) -> i64 {
    return unsafe { ftell((*p).f) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_ferror(p: &quota_FILE) -> i32 {
    return unsafe { ferror((*p).f) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_ftruncate(p: &quota_FILE,
    sz_new_1: sqlite3_int64) -> i32 {
    let mut p_file: *mut quotaFile = (*p).p_file;
    let mut rc: i32 = 0;
    if { p_file = (*p).p_file; p_file } != core::ptr::null_mut() &&
            unsafe { (*p_file).i_size } < sz_new_1 {
        let mut p_group: *mut quotaGroup = core::ptr::null_mut();
        if unsafe { (*p_file).i_size } < sz_new_1 { return -1; }
        p_group = unsafe { (*p_file).p_group };
        quota_enter();
        unsafe {
            (*p_group).i_size += sz_new_1 - unsafe { (*p_file).i_size }
        };
        quota_leave();
    }
    rc = unsafe { ftruncate(unsafe { fileno((*p).f) }, sz_new_1) };
    if !(p_file).is_null() && rc == 0 {
        let p_group_1: *mut quotaGroup = unsafe { (*p_file).p_group };
        quota_enter();
        unsafe {
            (*p_group_1).i_size += sz_new_1 - unsafe { (*p_file).i_size }
        };
        unsafe { (*p_file).i_size = sz_new_1 };
        quota_leave();
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_file_mtime(p: &quota_FILE,
    p_time_1: *mut time_t) -> i32 {
    let mut rc: i32 = 0;
    let mut buf: stat = unsafe { core::mem::zeroed() };
    rc = unsafe { fstat(unsafe { fileno((*p).f) }, &mut buf) };
    if rc == 0 { unsafe { *p_time_1 = buf.st_mtimespec.tv_sec }; }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_file_size(p: &quota_FILE) -> sqlite3_int64 {
    return if !((*p).p_file).is_null() {
            unsafe { (*(*p).p_file).i_size }
        } else { -1 as sqlite3_int64 };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_file_truesize(p: &quota_FILE)
    -> sqlite3_int64 {
    let mut rc: i32 = 0;
    let mut buf: stat = unsafe { core::mem::zeroed() };
    rc = unsafe { fstat(unsafe { fileno((*p).f) }, &mut buf) };
    return if rc == 0 { buf.st_size } else { -1 as off_t };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_file_available(p: &quota_FILE) -> i64 {
    let f: *mut FILE = (*p).f;
    let mut pos1: i64 = 0 as i64;
    let mut pos2: i64 = 0 as i64;
    let mut rc: i32 = 0;
    pos1 = unsafe { ftell(f) };
    if pos1 < 0 as i64 { return -1 as i64; }
    rc = unsafe { fseek(f, 0 as i64, 2) };
    if rc != 0 { return -1 as i64; }
    pos2 = unsafe { ftell(f) };
    if pos2 < 0 as i64 { return -1 as i64; }
    rc = unsafe { fseek(f, pos1, 0) };
    if rc != 0 { return -1 as i64; }
    return pos2 - pos1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_quota_remove(z_filename: *const i8) -> i32 {
    unsafe {
        let mut z_full: *mut i8 = core::ptr::null_mut();
        let mut n_full: u64 = 0 as u64;
        let mut rc: i32 = 0;
        let mut p_group: *mut quotaGroup = core::ptr::null_mut();
        let mut p_file: *mut quotaFile = core::ptr::null_mut();
        let mut p_next_file: *mut quotaFile = core::ptr::null_mut();
        let mut diff: i32 = 0;
        let mut c: i8 = 0 as i8;
        z_full =
            unsafe { sqlite3_malloc(g_quota.s_this_vfs.mx_pathname + 1) } as
                *mut i8;
        if z_full == core::ptr::null_mut() { return 7; }
        rc =
            unsafe {
                (unsafe {
                        (*g_quota.p_orig_vfs).x_full_pathname.unwrap()
                    })(g_quota.p_orig_vfs, z_filename,
                    g_quota.s_this_vfs.mx_pathname + 1, z_full)
            };
        if rc != 0 { unsafe { sqlite3_free(z_full as *mut ()) }; return rc; }
        n_full = unsafe { strlen(z_full as *const i8) };
        if n_full > 0 as u64 &&
                (unsafe { *z_full.add((n_full - 1 as u64) as usize) } as i32
                        == '/' as i32 ||
                    unsafe { *z_full.add((n_full - 1 as u64) as usize) } as i32
                        == '\\' as i32) {
            { let __p = &mut n_full; let __t = *__p; *__p -= 1; __t };
            unsafe { *z_full.add(n_full as usize) = 0 as i8 };
        }
        quota_enter();
        p_group = quota_group_find(z_full as *const i8);
        if !(p_group).is_null() {
            {
                p_file = unsafe { (*p_group).p_files };
                '__b14: loop {
                    if !(!(p_file).is_null() && rc == 0) { break '__b14; }
                    '__c14: loop {
                        p_next_file = unsafe { (*p_file).p_next };
                        diff =
                            unsafe {
                                strncmp(z_full as *const i8,
                                    unsafe { (*p_file).z_filename } as *const i8, n_full)
                            };
                        if diff == 0 &&
                                ({
                                                    c =
                                                        unsafe {
                                                            *unsafe { (*p_file).z_filename.add(n_full as usize) }
                                                        };
                                                    c
                                                } as i32 == 0 || c as i32 == '/' as i32 ||
                                    c as i32 == '\\' as i32) {
                            if unsafe { (*p_file).n_ref } != 0 {
                                unsafe { (*p_file).delete_on_close = 1 };
                            } else {
                                rc =
                                    unsafe {
                                        (unsafe {
                                                (*g_quota.p_orig_vfs).x_delete.unwrap()
                                            })(g_quota.p_orig_vfs,
                                            unsafe { (*p_file).z_filename } as *const i8, 0)
                                    };
                                quota_remove_file(p_file);
                                quota_group_deref(p_group);
                            }
                        }
                        break '__c14;
                    }
                    p_file = p_next_file;
                }
            }
        }
        quota_leave();
        unsafe { sqlite3_free(z_full as *mut ()) };
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
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fclose(_: *mut FILE)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn ftell(_: *mut FILE)
    -> i64;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn fflush(_: *mut FILE)
    -> i32;
    fn fileno(_: *mut FILE)
    -> i32;
    fn fsync(_: i32)
    -> i32;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn rewind(_: *mut FILE)
    -> ();
    fn ferror(_: *mut FILE)
    -> i32;
    fn ftruncate(_: i32, _: off_t)
    -> i32;
    fn fstat(_: i32, _: *mut stat)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;