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
struct VLogLog {
    p_next: *mut VLogLog,
    pp_prev: *mut *mut VLogLog,
    n_ref: i32,
    n_filename: i32,
    z_filename: *mut i8,
    out: *mut FILE,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VLogVfs {
    base: sqlite3_vfs,
    p_vfs: *mut sqlite3_vfs,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VLogFile {
    base: sqlite3_file,
    p_real: *mut sqlite3_file,
    p_log: *mut VLogLog,
}
extern "C" fn vlog_time() -> sqlite3_uint64 { return 0 as sqlite3_uint64; }
extern "C" fn vlog_log_print(p_log_1: *const VLogLog,
    t_start_1: sqlite3_int64, t_elapse_1: sqlite3_int64, z_op_1: *const i8,
    i_arg1_1: sqlite3_int64, i_arg2_1: sqlite3_int64, z_arg3_1: *const i8,
    i_res_1: i32) -> () {
    let mut z1: [i8; 40] = [0; 40];
    let mut z2: [i8; 40] = [0; 40];
    let mut z3: [i8; 2000] = [0; 2000];
    if p_log_1 == core::ptr::null_mut() { return; }
    if i_arg1_1 >= 0 as i64 {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 40]>() as i32,
                &raw mut z1[0 as usize] as *mut i8,
                c"%lld".as_ptr() as *mut i8 as *const i8, i_arg1_1)
        };
    } else { z1[0 as usize] = 0 as i8; }
    if i_arg2_1 >= 0 as i64 {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 40]>() as i32,
                &raw mut z2[0 as usize] as *mut i8,
                c"%lld".as_ptr() as *mut i8 as *const i8, i_arg2_1)
        };
    } else { z2[0 as usize] = 0 as i8; }
    if !(z_arg3_1).is_null() {
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 2000]>() as i32,
                &raw mut z3[0 as usize] as *mut i8,
                c"\"%.*w\"".as_ptr() as *mut i8 as *const i8,
                core::mem::size_of::<[i8; 2000]>() as u64 - 4 as u64,
                z_arg3_1)
        };
    } else { z3[0 as usize] = 0 as i8; }
    unsafe {
        fprintf(unsafe { (*p_log_1).out },
            c"%lld,%lld,%s,%d,%s,%s,%s,%d\n".as_ptr() as *mut i8 as *const i8,
            t_start_1, t_elapse_1, z_op_1,
            (unsafe { (*p_log_1).z_filename } == core::ptr::null_mut()) as
                i32, &raw mut z1[0 as usize] as *mut i8,
            &raw mut z2[0 as usize] as *mut i8,
            &raw mut z3[0 as usize] as *mut i8, i_res_1)
    };
}
extern "C" fn vlog_log_close(p: *mut VLogLog) -> () {
    if !(p).is_null() {
        let mut p_mutex: *mut sqlite3_mutex = core::ptr::null_mut();
        {
            let __p = unsafe { &mut (*p).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p).n_ref } > 0 ||
                unsafe { (*p).z_filename } == core::ptr::null_mut() {
            return;
        }
        p_mutex = unsafe { sqlite3_mutex_alloc(2) };
        unsafe { sqlite3_mutex_enter(p_mutex) };
        unsafe { *unsafe { (*p).pp_prev } = unsafe { (*p).p_next } };
        if !(unsafe { (*p).p_next }).is_null() {
            unsafe {
                (*unsafe { (*p).p_next }).pp_prev = unsafe { (*p).pp_prev }
            };
        }
        unsafe { sqlite3_mutex_leave(p_mutex) };
        unsafe { fclose(unsafe { (*p).out }) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}
extern "C" fn vlog_close(p_file: *mut sqlite3_file) -> i32 {
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut rc: i32 = 0;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    if !(unsafe { (*unsafe { (*p).p_real }).p_methods }).is_null() {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*unsafe { (*p).p_real }).p_methods
                                        }).x_close.unwrap()
                    })(unsafe { (*p).p_real })
            };
    }
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"CLOSE".as_ptr() as *mut i8 as *const i8, -1 as sqlite3_int64,
        -1 as sqlite3_int64, core::ptr::null(), rc);
    vlog_log_close(unsafe { (*p).p_log });
    return rc;
}
extern "C" fn vlog_signature(p: *mut u8, n: i32, z_cksum_1: *mut i8) -> () {
    let mut s0: u32 = 0 as u32;
    let mut s1: u32 = 0 as u32;
    let mut p_i: *const u32 = core::ptr::null();
    let mut i: i32 = 0;
    if n <= 16 {
        {
            i = 0;
            '__b0: loop {
                if !(i < n) { break '__b0; }
                '__c0: loop {
                    unsafe {
                        sqlite3_snprintf(3,
                            unsafe { z_cksum_1.offset((i * 2) as isize) },
                            c"%02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *p.offset(i as isize) } as i32)
                    };
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    } else {
        p_i = p as *mut u32;
        {
            i = 0;
            '__b1: loop {
                if !(i < n - 7) { break '__b1; }
                '__c1: loop {
                    s0 += unsafe { *p_i.offset(0 as isize) } + s1;
                    s1 += unsafe { *p_i.offset(1 as isize) } + s0;
                    {
                        let __n = 2;
                        let __p = &mut p_i;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    break '__c1;
                }
                i += 8;
            }
        }
        {
            i = 0;
            '__b2: loop {
                if !(i < 8) { break '__b2; }
                '__c2: loop {
                    unsafe {
                        sqlite3_snprintf(3,
                            unsafe { z_cksum_1.offset((i * 2) as isize) },
                            c"%02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *p.offset(i as isize) } as i32)
                    };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            sqlite3_snprintf(18,
                unsafe { z_cksum_1.offset((i * 2) as isize) },
                c"-%08x%08x".as_ptr() as *mut i8 as *const i8, s0, s1)
        };
    }
}
extern "C" fn big_to_native(x: *const u8) -> i32 {
    return ((unsafe { *x.offset(0 as isize) } as i32) << 24) +
                    ((unsafe { *x.offset(1 as isize) } as i32) << 16) +
                ((unsafe { *x.offset(2 as isize) } as i32) << 8) +
            unsafe { *x.offset(3 as isize) } as i32;
}
extern "C" fn vlog_read(p_file: *mut sqlite3_file, z_buf: *mut (), i_amt: i32,
    i_ofst: sqlite3_int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    let mut z_sig: [i8; 40] = [0; 40];
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_read.unwrap()
                })(unsafe { (*p).p_real }, z_buf, i_amt, i_ofst)
        };
    t_elapse = vlog_time() - t_start;
    if rc == 0 {
        vlog_signature(z_buf as *mut u8, i_amt,
            &raw mut z_sig[0 as usize] as *mut i8);
    } else { z_sig[0 as usize] = 0 as i8; }
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"READ".as_ptr() as *mut i8 as *const i8, i_amt as sqlite3_int64,
        i_ofst, &raw mut z_sig[0 as usize] as *mut i8 as *const i8, rc);
    if rc == 0 && !(unsafe { (*p).p_log }).is_null() &&
                    !(unsafe { (*unsafe { (*p).p_log }).z_filename }).is_null()
                && i_ofst <= 24 as i64 &&
            i_ofst + i_amt as sqlite_int64 >= 28 as i64 {
        let x: *const u8 =
            unsafe {
                    (z_buf as
                            *mut u8).offset((24 as sqlite_int64 - i_ofst) as isize)
                } as *const u8;
        let mut i_ctr: u32 = 0 as u32;
        let mut n_free: u32 = -1i32 as u32;
        let mut z_free: *const i8 = core::ptr::null();
        let mut z_str: [i8; 12] = [0; 12];
        i_ctr = big_to_native(x as *const u8) as u32;
        if i_ofst + i_amt as sqlite_int64 >= 40 as i64 {
            z_free = &raw mut z_str[0 as usize] as *mut i8;
            unsafe {
                sqlite3_snprintf(core::mem::size_of::<[i8; 12]>() as i32,
                    &raw mut z_str[0 as usize] as *mut i8,
                    c"%d".as_ptr() as *mut i8 as *const i8,
                    big_to_native(unsafe { x.offset(8 as isize) } as *const u8))
            };
            n_free =
                big_to_native(unsafe { x.offset(12 as isize) } as *const u8)
                    as u32;
        }
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, 0 as sqlite3_int64,
            c"CHNGCTR-READ".as_ptr() as *mut i8 as *const i8,
            i_ctr as sqlite3_int64, n_free as sqlite3_int64,
            z_free as *const i8, 0);
    }
    return rc;
}
extern "C" fn vlog_write(p_file: *mut sqlite3_file, z: *const (), i_amt: i32,
    i_ofst: sqlite3_int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    let mut z_sig: [i8; 40] = [0; 40];
    t_start = vlog_time();
    vlog_signature(z as *mut u8, i_amt,
        &raw mut z_sig[0 as usize] as *mut i8);
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_write.unwrap()
                })(unsafe { (*p).p_real }, z, i_amt, i_ofst)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"WRITE".as_ptr() as *mut i8 as *const i8, i_amt as sqlite3_int64,
        i_ofst, &raw mut z_sig[0 as usize] as *mut i8 as *const i8, rc);
    if rc == 0 && !(unsafe { (*p).p_log }).is_null() &&
                    !(unsafe { (*unsafe { (*p).p_log }).z_filename }).is_null()
                && i_ofst <= 24 as i64 &&
            i_ofst + i_amt as sqlite_int64 >= 28 as i64 {
        let x: *const u8 =
            unsafe {
                    (z as
                            *mut u8).offset((24 as sqlite_int64 - i_ofst) as isize)
                } as *const u8;
        let mut i_ctr: u32 = 0 as u32;
        let mut n_free: u32 = -1i32 as u32;
        let mut z_free: *const i8 = core::ptr::null();
        let mut z_str: [i8; 12] = [0; 12];
        i_ctr = big_to_native(x as *const u8) as u32;
        if i_ofst + i_amt as sqlite_int64 >= 40 as i64 {
            z_free = &raw mut z_str[0 as usize] as *mut i8;
            unsafe {
                sqlite3_snprintf(core::mem::size_of::<[i8; 12]>() as i32,
                    &raw mut z_str[0 as usize] as *mut i8,
                    c"%d".as_ptr() as *mut i8 as *const i8,
                    big_to_native(unsafe { x.offset(8 as isize) } as *const u8))
            };
            n_free =
                big_to_native(unsafe { x.offset(12 as isize) } as *const u8)
                    as u32;
        }
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, 0 as sqlite3_int64,
            c"CHNGCTR-WRITE".as_ptr() as *mut i8 as *const i8,
            i_ctr as sqlite3_int64, n_free as sqlite3_int64,
            z_free as *const i8, 0);
    }
    return rc;
}
extern "C" fn vlog_truncate(p_file: *mut sqlite3_file, size: sqlite3_int64)
    -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_truncate.unwrap()
                })(unsafe { (*p).p_real }, size)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"TRUNCATE".as_ptr() as *mut i8 as *const i8, size,
        -1 as sqlite3_int64, core::ptr::null(), rc);
    return rc;
}
extern "C" fn vlog_sync(p_file: *mut sqlite3_file, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_sync.unwrap()
                })(unsafe { (*p).p_real }, flags)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"SYNC".as_ptr() as *mut i8 as *const i8, flags as sqlite3_int64,
        -1 as sqlite3_int64, core::ptr::null(), rc);
    return rc;
}
extern "C" fn vlog_file_size(p_file: *mut sqlite3_file,
    p_size: *mut sqlite3_int64) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_file_size.unwrap()
                })(unsafe { (*p).p_real }, p_size)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"FILESIZE".as_ptr() as *mut i8 as *const i8, unsafe { *p_size },
        -1 as sqlite3_int64, core::ptr::null(), rc);
    return rc;
}
extern "C" fn vlog_lock(p_file: *mut sqlite3_file, e_lock: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_lock.unwrap()
                })(unsafe { (*p).p_real }, e_lock)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"LOCK".as_ptr() as *mut i8 as *const i8, e_lock as sqlite3_int64,
        -1 as sqlite3_int64, core::ptr::null(), rc);
    return rc;
}
extern "C" fn vlog_unlock(p_file: *mut sqlite3_file, e_lock: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, 0 as sqlite3_int64,
        c"UNLOCK".as_ptr() as *mut i8 as *const i8, e_lock as sqlite3_int64,
        -1 as sqlite3_int64, core::ptr::null(), 0);
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_unlock.unwrap()
                })(unsafe { (*p).p_real }, e_lock)
        };
    return rc;
}
extern "C" fn vlog_check_reserved_lock(p_file: *mut sqlite3_file,
    p_res_out: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_check_reserved_lock.unwrap()
                })(unsafe { (*p).p_real }, p_res_out)
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"CHECKRESERVEDLOCK".as_ptr() as *mut i8 as *const i8,
        unsafe { *p_res_out } as sqlite3_int64, -1 as sqlite3_int64,
        c"".as_ptr() as *mut i8 as *const i8, rc);
    return rc;
}
extern "C" fn vlog_file_control(p_file: *mut sqlite3_file, op: i32,
    p_arg: *mut ()) -> i32 {
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut rc: i32 = 0;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_file_control.unwrap()
                })(unsafe { (*p).p_real }, op, p_arg)
        };
    if op == 12 && rc == 0 {
        unsafe {
            *(p_arg as *mut *mut i8) =
                unsafe {
                    sqlite3_mprintf(c"vlog/%z".as_ptr() as *mut i8 as *const i8,
                        unsafe { *(p_arg as *mut *mut i8) })
                }
        };
    }
    t_elapse = vlog_time() - t_start;
    if op == 19 {
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, t_elapse as sqlite3_int64,
            c"TRACE".as_ptr() as *mut i8 as *const i8, op as sqlite3_int64,
            -1 as sqlite3_int64, p_arg as *const i8, rc);
    } else if op == 14 {
        let az_arg: *const *const i8 =
            p_arg as *mut *const i8 as *const *const i8;
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, t_elapse as sqlite3_int64,
            c"FILECONTROL".as_ptr() as *mut i8 as *const i8,
            op as sqlite3_int64, -1 as sqlite3_int64,
            unsafe { *az_arg.offset(1 as isize) }, rc);
    } else if op == 5 {
        let sz: sqlite3_int64 =
            unsafe { core::ptr::read(p_arg as *mut sqlite3_int64) };
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, t_elapse as sqlite3_int64,
            c"FILECONTROL".as_ptr() as *mut i8 as *const i8,
            op as sqlite3_int64, sz, core::ptr::null(), rc);
    } else {
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, t_elapse as sqlite3_int64,
            c"FILECONTROL".as_ptr() as *mut i8 as *const i8,
            op as sqlite3_int64, -1 as sqlite3_int64, core::ptr::null(), rc);
    }
    return rc;
}
extern "C" fn vlog_sector_size(p_file: *mut sqlite3_file) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_sector_size.unwrap()
                })(unsafe { (*p).p_real })
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"SECTORSIZE".as_ptr() as *mut i8 as *const i8, -1 as sqlite3_int64,
        -1 as sqlite3_int64, core::ptr::null(), rc);
    return rc;
}
extern "C" fn vlog_device_characteristics(p_file: *mut sqlite3_file) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let p: *const VLogFile = p_file as *mut VLogFile as *const VLogFile;
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*unsafe { (*p).p_real }).p_methods
                                    }).x_device_characteristics.unwrap()
                })(unsafe { (*p).p_real })
        };
    t_elapse = vlog_time() - t_start;
    vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
        t_start as sqlite3_int64, t_elapse as sqlite3_int64,
        c"DEVCHAR".as_ptr() as *mut i8 as *const i8, -1 as sqlite3_int64,
        -1 as sqlite3_int64, core::ptr::null(), rc);
    return rc;
}
static mut all_logs: *mut VLogLog = core::ptr::null_mut();
extern "C" fn vlog_log_open(z_filename_1: *const i8) -> *mut VLogLog {
    unsafe {
        let mut n_name: i32 = unsafe { strlen(z_filename_1) } as i32;
        let mut is_journal: i32 = 0;
        let mut p_mutex: *mut sqlite3_mutex = core::ptr::null_mut();
        let mut p_log: *mut VLogLog = core::ptr::null_mut();
        let mut p_temp: *mut VLogLog = core::ptr::null_mut();
        let mut t_now: sqlite3_int64 = 0 as sqlite3_int64;
        if n_name > 4 &&
                unsafe {
                        strcmp(unsafe {
                                unsafe {
                                    z_filename_1.offset(n_name as isize).offset(-(4 as isize))
                                }
                            }, c"-wal".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
            return core::ptr::null_mut();
        } else if n_name > 8 &&
                unsafe {
                        strcmp(unsafe {
                                unsafe {
                                    z_filename_1.offset(n_name as isize).offset(-(8 as isize))
                                }
                            }, c"-journal".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
            n_name -= 8;
            is_journal = 1;
        } else if n_name > 12 &&
                unsafe {
                        sqlite3_strglob(c"-mj??????9??".as_ptr() as *mut i8 as
                                *const i8,
                            unsafe {
                                unsafe {
                                    z_filename_1.offset(n_name as isize).offset(-(12 as isize))
                                }
                            })
                    } == 0 {
            return core::ptr::null_mut();
        }
        p_temp =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<VLogLog>() as u64 *
                                    2 as u64 + n_name as u64 + 60 as u64)
                } as *mut VLogLog;
        if p_temp == core::ptr::null_mut() { return core::ptr::null_mut(); }
        p_mutex = unsafe { sqlite3_mutex_alloc(2) };
        unsafe { sqlite3_mutex_enter(p_mutex) };
        {
            p_log = all_logs;
            '__b3: loop {
                if !(!(p_log).is_null()) { break '__b3; }
                '__c3: loop {
                    if unsafe { (*p_log).n_filename } == n_name &&
                            (unsafe {
                                            memcmp(unsafe { (*p_log).z_filename } as *const (),
                                                z_filename_1 as *const (), n_name as u64)
                                        } == 0) as i32 != 0 {
                        break '__b3;
                    }
                    break '__c3;
                }
                p_log = unsafe { (*p_log).p_next };
            }
        }
        if p_log == core::ptr::null_mut() {
            p_log = p_temp;
            p_temp = core::ptr::null_mut();
            unsafe {
                memset(p_log as *mut (), 0,
                    core::mem::size_of::<VLogLog>() as u64 * 2 as u64)
            };
            unsafe {
                (*p_log).z_filename =
                    unsafe { &raw mut *p_log.offset(2 as isize) } as *mut i8
            };
            t_now = vlog_time() as sqlite3_int64;
            unsafe {
                sqlite3_snprintf(n_name + 60, unsafe { (*p_log).z_filename },
                    c"%.*s-debuglog-%lld".as_ptr() as *mut i8 as *const i8,
                    n_name, z_filename_1, t_now)
            };
            unsafe {
                (*p_log).out =
                    unsafe {
                        fopen(unsafe { (*p_log).z_filename } as *const i8,
                            c"a".as_ptr() as *mut i8 as *const i8)
                    }
            };
            if unsafe { (*p_log).out } == core::ptr::null_mut() {
                unsafe { sqlite3_mutex_leave(p_mutex) };
                unsafe { sqlite3_free(p_log as *mut ()) };
                return core::ptr::null_mut();
            }
            unsafe { (*p_log).n_filename = n_name };
            unsafe {
                (*p_log.offset(1 as isize)).out =
                    unsafe { (*p_log.offset(0 as isize)).out }
            };
            unsafe { (*p_log).pp_prev = &mut all_logs };
            if !(all_logs).is_null() {
                unsafe {
                    (*all_logs).pp_prev = unsafe { &mut (*p_log).p_next }
                };
            }
            unsafe { (*p_log).p_next = all_logs };
            all_logs = p_log;
        }
        unsafe { sqlite3_mutex_leave(p_mutex) };
        if !(p_temp).is_null() {
            unsafe { sqlite3_free(p_temp as *mut ()) };
        } else {}
        if !(p_log).is_null() && is_journal != 0 {
            {
                let __p = &mut p_log;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        {
            let __p = unsafe { &mut (*p_log).n_ref };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return p_log;
    }
}
static mut vlog_io_methods: sqlite3_io_methods =
    sqlite3_io_methods {
        i_version: 1,
        x_close: Some(vlog_close),
        x_read: Some(vlog_read),
        x_write: Some(vlog_write),
        x_truncate: Some(vlog_truncate),
        x_sync: Some(vlog_sync),
        x_file_size: Some(vlog_file_size),
        x_lock: Some(vlog_lock),
        x_unlock: Some(vlog_unlock),
        x_check_reserved_lock: Some(vlog_check_reserved_lock),
        x_file_control: Some(vlog_file_control),
        x_sector_size: Some(vlog_sector_size),
        x_device_characteristics: Some(vlog_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };
extern "C" fn vlog_open(p_vfs: *mut sqlite3_vfs, z_name: *const i8,
    p_file: *mut sqlite3_file, flags: i32, p_out_flags: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut i_arg2: sqlite3_int64 = 0 as sqlite3_int64;
        let p: *mut VLogFile = p_file as *mut VLogFile;
        unsafe {
            (*p).p_real =
                unsafe { &raw mut *p.offset(1 as isize) } as *mut sqlite3_file
        };
        if flags & (256 | 2048) != 0 {
            unsafe { (*p).p_log = vlog_log_open(z_name) };
        } else { unsafe { (*p).p_log = core::ptr::null_mut() }; }
        t_start = vlog_time();
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*(p_vfs as *mut VLogVfs)).p_vfs
                                        }).x_open.unwrap()
                    })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_name,
                    unsafe { (*p).p_real }, flags, p_out_flags)
            };
        t_elapse = vlog_time() - t_start;
        i_arg2 =
            if !(p_out_flags).is_null() {
                    unsafe { *p_out_flags }
                } else { -1 } as sqlite3_int64;
        vlog_log_print(unsafe { (*p).p_log } as *const VLogLog,
            t_start as sqlite3_int64, t_elapse as sqlite3_int64,
            c"OPEN".as_ptr() as *mut i8 as *const i8, flags as sqlite3_int64,
            i_arg2, core::ptr::null(), rc);
        if rc == 0 {
            unsafe {
                (*p_file).p_methods =
                    &raw mut vlog_io_methods as *const sqlite3_io_methods
            };
        } else {
            if !(unsafe { (*p).p_log }).is_null() {
                vlog_log_close(unsafe { (*p).p_log });
            }
            unsafe { (*p).p_log = core::ptr::null_mut() };
        }
        return rc;
    }
}
extern "C" fn vlog_delete(p_vfs: *mut sqlite3_vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut p_log: *mut VLogLog = core::ptr::null_mut();
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_delete.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path,
                dir_sync)
        };
    t_elapse = vlog_time() - t_start;
    p_log = vlog_log_open(z_path);
    vlog_log_print(p_log as *const VLogLog, t_start as sqlite3_int64,
        t_elapse as sqlite3_int64, c"DELETE".as_ptr() as *mut i8 as *const i8,
        dir_sync as sqlite3_int64, -1 as sqlite3_int64, core::ptr::null(),
        rc);
    vlog_log_close(p_log);
    return rc;
}
extern "C" fn vlog_access(p_vfs: *mut sqlite3_vfs, z_path: *const i8,
    flags: i32, p_res_out: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut t_start: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut t_elapse: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut p_log: *mut VLogLog = core::ptr::null_mut();
    t_start = vlog_time();
    rc =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_access.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path, flags,
                p_res_out)
        };
    t_elapse = vlog_time() - t_start;
    p_log = vlog_log_open(z_path);
    vlog_log_print(p_log as *const VLogLog, t_start as sqlite3_int64,
        t_elapse as sqlite3_int64, c"ACCESS".as_ptr() as *mut i8 as *const i8,
        flags as sqlite3_int64, unsafe { *p_res_out } as sqlite3_int64,
        core::ptr::null(), rc);
    vlog_log_close(p_log);
    return rc;
}
extern "C" fn vlog_full_pathname(p_vfs: *mut sqlite3_vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_full_pathname.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path, n_out,
                z_out)
        };
}
extern "C" fn vlog_dl_open(p_vfs: *mut sqlite3_vfs, z_path: *const i8)
    -> *mut () {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_dl_open.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, z_path)
        };
}
extern "C" fn vlog_dl_error(p_vfs: *mut sqlite3_vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*(p_vfs as *mut VLogVfs)).p_vfs
                                }).x_dl_error.unwrap()
            })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, n_byte, z_err_msg)
    };
}
extern "C" fn vlog_dl_sym(p_vfs: *mut sqlite3_vfs, p: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_dl_sym.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p, z_sym)
        };
}
extern "C" fn vlog_dl_close(p_vfs: *mut sqlite3_vfs, p_handle: *mut ())
    -> () {
    unsafe {
        (unsafe {
                (*unsafe {
                                    (*(p_vfs as *mut VLogVfs)).p_vfs
                                }).x_dl_close.unwrap()
            })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p_handle)
    };
}
extern "C" fn vlog_randomness(p_vfs: *mut sqlite3_vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_randomness.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, n_byte,
                z_buf_out)
        };
}
extern "C" fn vlog_sleep(p_vfs: *mut sqlite3_vfs, n_micro: i32) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_sleep.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, n_micro)
        };
}
extern "C" fn vlog_current_time(p_vfs: *mut sqlite3_vfs, p_time_out: *mut f64)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_current_time.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p_time_out)
        };
}
extern "C" fn vlog_get_last_error(p_vfs: *mut sqlite3_vfs, a: i32, b: *mut i8)
    -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_get_last_error.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, a, b)
        };
}
extern "C" fn vlog_current_time_int64(p_vfs: *mut sqlite3_vfs,
    p: *mut sqlite3_int64) -> i32 {
    return unsafe {
            (unsafe {
                    (*unsafe {
                                        (*(p_vfs as *mut VLogVfs)).p_vfs
                                    }).x_current_time_int64.unwrap()
                })(unsafe { (*(p_vfs as *mut VLogVfs)).p_vfs }, p)
        };
}
static mut vlog_vfs: VLogVfs =
    VLogVfs {
        base: sqlite3_vfs {
            i_version: 1,
            sz_os_file: 0,
            mx_pathname: 1024,
            p_next: core::ptr::null_mut(),
            z_name: c"vfslog".as_ptr() as *const i8,
            p_app_data: core::ptr::null_mut(),
            x_open: Some(vlog_open),
            x_delete: Some(vlog_delete),
            x_access: Some(vlog_access),
            x_full_pathname: Some(vlog_full_pathname),
            x_dl_open: Some(vlog_dl_open),
            x_dl_error: Some(vlog_dl_error),
            x_dl_sym: Some(vlog_dl_sym),
            x_dl_close: Some(vlog_dl_close),
            x_randomness: Some(vlog_randomness),
            x_sleep: Some(vlog_sleep),
            x_current_time: Some(vlog_current_time),
            x_get_last_error: Some(vlog_get_last_error),
            x_current_time_int64: Some(vlog_current_time_int64),
            x_set_system_call: None,
            x_get_system_call: None,
            x_next_system_call: None,
        },
        p_vfs: core::ptr::null_mut(),
    };
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_vfslog(z_arg_1: *const i8) -> i32 {
    unsafe {
        vlog_vfs.p_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        if vlog_vfs.p_vfs == core::ptr::null_mut() { return 1; }
        vlog_vfs.base.sz_os_file =
            (core::mem::size_of::<VLogFile>() as u64 +
                    unsafe { (*vlog_vfs.p_vfs).sz_os_file } as u64) as i32;
        return unsafe { sqlite3_vfs_register(&mut vlog_vfs.base, 1) };
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn fclose(_: *mut FILE)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;