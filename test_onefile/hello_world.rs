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
struct fs_real_file {
    p_file: *mut sqlite3_file,
    z_name: *const i8,
    n_database: i32,
    n_journal: i32,
    n_blob: i32,
    n_ref: i32,
    p_next: *mut fs_real_file,
    pp_this: *mut *mut fs_real_file,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fs_file {
    base: sqlite3_file,
    e_type: i32,
    p_real: *mut fs_real_file,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct tmp_file {
    base: sqlite3_file,
    n_size: i32,
    n_alloc: i32,
    z_alloc: *mut i8,
}
extern "C" fn fs_close(p_file: *mut sqlite3_file) -> i32 {
    let mut rc: i32 = 0;
    let p: *const fs_file = p_file as *mut fs_file as *const fs_file;
    let p_real: *mut fs_real_file = unsafe { (*p).p_real };
    {
        let __p = unsafe { &mut (*p_real).n_ref };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    if !(unsafe { (*p_real).n_ref } >= 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsClose".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 382,
                c"pReal->nRef>=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_real).n_ref } == 0 {
        unsafe {
            *unsafe { (*p_real).pp_this } = unsafe { (*p_real).p_next }
        };
        if !(unsafe { (*p_real).p_next }).is_null() {
            unsafe {
                (*unsafe { (*p_real).p_next }).pp_this =
                    unsafe { (*p_real).pp_this }
            };
        }
        rc =
            unsafe {
                (unsafe {
                        (*unsafe {
                                            (*unsafe { (*p_real).p_file }).p_methods
                                        }).x_close.unwrap()
                    })(unsafe { (*p_real).p_file })
            };
        unsafe { sqlite3_free(p_real as *mut ()) };
    }
    return rc;
}
extern "C" fn fs_read(p_file: *mut sqlite3_file, z_buf: *mut (), i_amt: i32,
    i_ofst: sqlite3_int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *const fs_file = p_file as *mut fs_file as *const fs_file;
    let p_real: *const fs_real_file =
        unsafe { (*p).p_real } as *const fs_real_file;
    let p_f: *mut sqlite3_file = unsafe { (*p_real).p_file };
    if unsafe { (*p).e_type } == 1 &&
                i_amt as sqlite_int64 + i_ofst >
                    unsafe { (*p_real).n_database } as i64 ||
            unsafe { (*p).e_type } == 2 &&
                i_amt as sqlite_int64 + i_ofst >
                    unsafe { (*p_real).n_journal } as i64 {
        rc = 10 | 2 << 8;
    } else if unsafe { (*p).e_type } == 1 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_f).p_methods }).x_read.unwrap()
                    })(p_f, z_buf, i_amt, i_ofst + 512 as sqlite_int64)
            };
    } else {
        let mut i_rem: i32 = i_amt;
        let mut i_buf: i32 = 0;
        let mut ii: i32 = i_ofst as i32;
        while i_rem > 0 && rc == 0 {
            let i_real_off: i32 =
                unsafe { (*p_real).n_blob } - 512 * (ii / 512 + 1) + ii % 512;
            let i_real_amt: i32 =
                if i_rem < 512 - i_real_off % 512 {
                    i_rem
                } else { 512 - i_real_off % 512 };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_f).p_methods }).x_read.unwrap()
                        })(p_f,
                        unsafe {
                                &raw mut *(z_buf as *mut i8).offset(i_buf as isize)
                            } as *mut (), i_real_amt, i_real_off as i64)
                };
            ii += i_real_amt;
            i_buf += i_real_amt;
            i_rem -= i_real_amt;
        }
    }
    return rc;
}
extern "C" fn fs_write(p_file: *mut sqlite3_file, z_buf: *const (),
    i_amt: i32, i_ofst: sqlite3_int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *const fs_file = p_file as *mut fs_file as *const fs_file;
    let p_real: *mut fs_real_file = unsafe { (*p).p_real };
    let p_f: *mut sqlite3_file = unsafe { (*p_real).p_file };
    if unsafe { (*p).e_type } == 1 {
        if i_amt as sqlite_int64 + i_ofst + 512 as sqlite_int64 >
                (unsafe { (*p_real).n_blob } - unsafe { (*p_real).n_journal })
                    as i64 {
            rc = 13;
        } else {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*p_f).p_methods }).x_write.unwrap()
                        })(p_f, z_buf, i_amt, i_ofst + 512 as sqlite_int64)
                };
            if rc == 0 {
                unsafe {
                    (*p_real).n_database =
                        if unsafe { (*p_real).n_database } as sqlite_int64 >
                                    i_amt as sqlite_int64 + i_ofst {
                                (unsafe { (*p_real).n_database }) as sqlite_int64
                            } else { i_amt as sqlite_int64 + i_ofst } as i32
                };
            }
        }
    } else {
        let mut i_rem: i32 = i_amt;
        let mut i_buf: i32 = 0;
        let mut ii: i32 = i_ofst as i32;
        while i_rem > 0 && rc == 0 {
            let i_real_off: i32 =
                unsafe { (*p_real).n_blob } - 512 * (ii / 512 + 1) + ii % 512;
            let i_real_amt: i32 =
                if i_rem < 512 - i_real_off % 512 {
                    i_rem
                } else { 512 - i_real_off % 512 };
            if i_real_off < unsafe { (*p_real).n_database } + 512 {
                rc = 13;
            } else {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*p_f).p_methods }).x_write.unwrap()
                            })(p_f,
                            unsafe {
                                    &raw mut *(z_buf as *mut i8).offset(i_buf as isize)
                                } as *const (), i_real_amt, i_real_off as i64)
                    };
                ii += i_real_amt;
                i_buf += i_real_amt;
                i_rem -= i_real_amt;
            }
        }
        if rc == 0 {
            unsafe {
                (*p_real).n_journal =
                    if unsafe { (*p_real).n_journal } as sqlite_int64 >
                                i_amt as sqlite_int64 + i_ofst {
                            (unsafe { (*p_real).n_journal }) as sqlite_int64
                        } else { i_amt as sqlite_int64 + i_ofst } as i32
            };
        }
    }
    return rc;
}
extern "C" fn fs_truncate(p_file: *mut sqlite3_file, size: sqlite3_int64)
    -> i32 {
    let p: *const fs_file = p_file as *mut fs_file as *const fs_file;
    let p_real: *mut fs_real_file = unsafe { (*p).p_real };
    if unsafe { (*p).e_type } == 1 {
        unsafe {
            (*p_real).n_database =
                if (unsafe { (*p_real).n_database } as sqlite_int64) < size {
                        (unsafe { (*p_real).n_database }) as sqlite_int64
                    } else { size } as i32
        };
    } else {
        unsafe {
            (*p_real).n_journal =
                if (unsafe { (*p_real).n_journal } as sqlite_int64) < size {
                        (unsafe { (*p_real).n_journal }) as sqlite_int64
                    } else { size } as i32
        };
    }
    return 0;
}
extern "C" fn fs_sync(p_file: *mut sqlite3_file, flags: i32) -> i32 {
    let p: *const fs_file = p_file as *mut fs_file as *const fs_file;
    let p_real: *const fs_real_file =
        unsafe { (*p).p_real } as *const fs_real_file;
    let p_real_file: *mut sqlite3_file = unsafe { (*p_real).p_file };
    let mut rc: i32 = 0;
    if unsafe { (*p).e_type } == 1 {
        let mut z_size: [u8; 4] = [0; 4];
        z_size[0 as usize] =
            ((unsafe { (*p_real).n_database } as u32 & 4278190080u32) >> 24)
                as u8;
        z_size[1 as usize] =
            ((unsafe { (*p_real).n_database } & 16711680) >> 16) as u8;
        z_size[2 as usize] =
            ((unsafe { (*p_real).n_database } & 65280) >> 8) as u8;
        z_size[3 as usize] = (unsafe { (*p_real).n_database } & 255) as u8;
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_real_file).p_methods }).x_write.unwrap()
                    })(p_real_file,
                    &raw mut z_size[0 as usize] as *mut u8 as *const (), 4, 0)
            };
    }
    if rc == 0 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_real_file).p_methods }).x_sync.unwrap()
                    })(p_real_file, flags & !16)
            };
    }
    return rc;
}
extern "C" fn fs_file_size(p_file: *mut sqlite3_file,
    p_size: *mut sqlite3_int64) -> i32 {
    let p: *const fs_file = p_file as *mut fs_file as *const fs_file;
    let p_real: *const fs_real_file =
        unsafe { (*p).p_real } as *const fs_real_file;
    if unsafe { (*p).e_type } == 1 {
        unsafe { *p_size = unsafe { (*p_real).n_database } as sqlite_int64 };
    } else {
        unsafe { *p_size = unsafe { (*p_real).n_journal } as sqlite_int64 };
    }
    return 0;
}
extern "C" fn fs_lock(p_file: *mut sqlite3_file, e_lock: i32) -> i32 {
    return 0;
}
extern "C" fn fs_unlock(p_file: *mut sqlite3_file, e_lock: i32) -> i32 {
    return 0;
}
extern "C" fn fs_check_reserved_lock(p_file: *mut sqlite3_file,
    p_res_out: *mut i32) -> i32 {
    unsafe { *p_res_out = 0 };
    return 0;
}
extern "C" fn fs_file_control(p_file: *mut sqlite3_file, op: i32,
    p_arg: *mut ()) -> i32 {
    if op == 14 { return 12; }
    return 0;
}
extern "C" fn fs_sector_size(p_file: *mut sqlite3_file) -> i32 { return 512; }
extern "C" fn fs_device_characteristics(p_file: *mut sqlite3_file) -> i32 {
    return 0;
}
extern "C" fn tmp_close(p_file: *mut sqlite3_file) -> i32 {
    let p_tmp: *const tmp_file = p_file as *mut tmp_file as *const tmp_file;
    unsafe { sqlite3_free(unsafe { (*p_tmp).z_alloc } as *mut ()) };
    return 0;
}
extern "C" fn tmp_read(p_file: *mut sqlite3_file, z_buf: *mut (), i_amt: i32,
    i_ofst: sqlite3_int64) -> i32 {
    let p_tmp: *const tmp_file = p_file as *mut tmp_file as *const tmp_file;
    if i_amt as sqlite_int64 + i_ofst > unsafe { (*p_tmp).n_size } as i64 {
        return 10 | 2 << 8;
    }
    unsafe {
        memcpy(z_buf,
            unsafe {
                    &raw mut *unsafe {
                                (*p_tmp).z_alloc.offset(i_ofst as isize)
                            }
                } as *const (), i_amt as u64)
    };
    return 0;
}
extern "C" fn tmp_write(p_file: *mut sqlite3_file, z_buf: *const (),
    i_amt: i32, i_ofst: sqlite3_int64) -> i32 {
    let p_tmp: *mut tmp_file = p_file as *mut tmp_file;
    if i_amt as sqlite_int64 + i_ofst > unsafe { (*p_tmp).n_alloc } as i64 {
        let n_new: i32 =
            (2 as sqlite_int64 *
                    (i_amt as sqlite_int64 + i_ofst +
                        unsafe { (*p_tmp).n_alloc } as sqlite_int64)) as i32;
        let z_new: *mut i8 =
            unsafe {
                    sqlite3_realloc(unsafe { (*p_tmp).z_alloc } as *mut (),
                        n_new)
                } as *mut i8;
        if (z_new).is_null() as i32 != 0 { return 7; }
        unsafe { (*p_tmp).z_alloc = z_new };
        unsafe { (*p_tmp).n_alloc = n_new };
    }
    unsafe {
        memcpy(unsafe {
                    &raw mut *unsafe {
                                (*p_tmp).z_alloc.offset(i_ofst as isize)
                            }
                } as *mut (), z_buf, i_amt as u64)
    };
    unsafe {
        (*p_tmp).n_size =
            if unsafe { (*p_tmp).n_size } as sqlite_int64 >
                        i_ofst + i_amt as sqlite_int64 {
                    (unsafe { (*p_tmp).n_size }) as sqlite_int64
                } else { i_ofst + i_amt as sqlite_int64 } as i32
    };
    return 0;
}
extern "C" fn tmp_truncate(p_file: *mut sqlite3_file, size: sqlite3_int64)
    -> i32 {
    let p_tmp: *mut tmp_file = p_file as *mut tmp_file;
    unsafe {
        (*p_tmp).n_size =
            if (unsafe { (*p_tmp).n_size } as sqlite_int64) < size {
                    (unsafe { (*p_tmp).n_size }) as sqlite_int64
                } else { size } as i32
    };
    return 0;
}
extern "C" fn tmp_sync(p_file: *mut sqlite3_file, flags: i32) -> i32 {
    return 0;
}
extern "C" fn tmp_file_size(p_file: *mut sqlite3_file,
    p_size: *mut sqlite3_int64) -> i32 {
    let p_tmp: *const tmp_file = p_file as *mut tmp_file as *const tmp_file;
    unsafe { *p_size = unsafe { (*p_tmp).n_size } as sqlite_int64 };
    return 0;
}
extern "C" fn tmp_lock(p_file: *mut sqlite3_file, e_lock: i32) -> i32 {
    return 0;
}
extern "C" fn tmp_unlock(p_file: *mut sqlite3_file, e_lock: i32) -> i32 {
    return 0;
}
extern "C" fn tmp_check_reserved_lock(p_file: *mut sqlite3_file,
    p_res_out: *mut i32) -> i32 {
    unsafe { *p_res_out = 0 };
    return 0;
}
extern "C" fn tmp_file_control(p_file: *mut sqlite3_file, op: i32,
    p_arg: *mut ()) -> i32 {
    return 0;
}
extern "C" fn tmp_sector_size(p_file: *mut sqlite3_file) -> i32 { return 0; }
extern "C" fn tmp_device_characteristics(p_file: *mut sqlite3_file) -> i32 {
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fs_vfs_t {
    base: sqlite3_vfs,
    p_file_list: *mut fs_real_file,
    p_parent: *mut sqlite3_vfs,
}
static mut tmp_io_methods: sqlite3_io_methods =
    sqlite3_io_methods {
        i_version: 1,
        x_close: Some(tmp_close),
        x_read: Some(tmp_read),
        x_write: Some(tmp_write),
        x_truncate: Some(tmp_truncate),
        x_sync: Some(tmp_sync),
        x_file_size: Some(tmp_file_size),
        x_lock: Some(tmp_lock),
        x_unlock: Some(tmp_unlock),
        x_check_reserved_lock: Some(tmp_check_reserved_lock),
        x_file_control: Some(tmp_file_control),
        x_sector_size: Some(tmp_sector_size),
        x_device_characteristics: Some(tmp_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };
static mut fs_io_methods: sqlite3_io_methods =
    sqlite3_io_methods {
        i_version: 1,
        x_close: Some(fs_close),
        x_read: Some(fs_read),
        x_write: Some(fs_write),
        x_truncate: Some(fs_truncate),
        x_sync: Some(fs_sync),
        x_file_size: Some(fs_file_size),
        x_lock: Some(fs_lock),
        x_unlock: Some(fs_unlock),
        x_check_reserved_lock: Some(fs_check_reserved_lock),
        x_file_control: Some(fs_file_control),
        x_sector_size: Some(fs_sector_size),
        x_device_characteristics: Some(fs_device_characteristics),
        x_shm_map: None,
        x_shm_lock: None,
        x_shm_barrier: None,
        x_shm_unmap: None,
        x_fetch: None,
        x_unfetch: None,
    };
extern "C" fn fs_open(p_vfs: *mut sqlite3_vfs, z_name: *const i8,
    p_file: *mut sqlite3_file, flags: i32, p_out_flags: *mut i32) -> i32 {
    unsafe {
        let mut p_fs_vfs: *mut fs_vfs_t = core::ptr::null_mut();
        let mut p: *mut fs_file = core::ptr::null_mut();
        let mut p_real: *mut fs_real_file = core::ptr::null_mut();
        let mut e_type: i32 = 0;
        let mut n_name: i32 = 0;
        let mut rc: i32 = 0;
        let mut p2: *mut tmp_file = core::ptr::null_mut();
        let mut real_flags: i32 = 0;
        let mut size: sqlite3_int64 = 0 as sqlite3_int64;
        let mut p_real_file: *mut sqlite3_file = core::ptr::null_mut();
        let mut p_parent: *mut sqlite3_vfs = core::ptr::null_mut();
        let mut z_s: [u8; 4] = [0; 4];
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s3:
                {
                match __state {
                    0 => { p_fs_vfs = p_vfs as *mut fs_vfs_t; __state = 3; }
                    2 => {
                        if !(p_real).is_null() {
                            __state = 62;
                        } else { __state = 61; }
                    }
                    3 => { p = p_file as *mut fs_file; __state = 4; }
                    4 => { p_real = core::ptr::null_mut(); __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { rc = 0; __state = 8; }
                    8 => {
                        if 0 == flags & (256 | 2048) {
                            __state = 10;
                        } else { __state = 9; }
                    }
                    9 => {
                        e_type = if flags & 256 != 0 { 1 } else { 2 };
                        __state = 14;
                    }
                    10 => { p2 = p_file as *mut tmp_file; __state = 11; }
                    11 => {
                        unsafe {
                            memset(p2 as *mut (), 0,
                                core::mem::size_of::<tmp_file>() as u64)
                        };
                        __state = 12;
                    }
                    12 => {
                        unsafe {
                            (*p2).base.p_methods =
                                &raw mut tmp_io_methods as *const sqlite3_io_methods
                        };
                        __state = 13;
                    }
                    13 => { return 0; }
                    14 => {
                        unsafe {
                            (*p).base.p_methods =
                                &raw mut fs_io_methods as *const sqlite3_io_methods
                        };
                        __state = 15;
                    }
                    15 => { unsafe { (*p).e_type = e_type }; __state = 16; }
                    16 => {
                        if !(unsafe {
                                                    strlen(c"-journal".as_ptr() as *mut i8 as *const i8)
                                                } == 8 as u64) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fsOpen".as_ptr() as *const i8,
                                    c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 609,
                                    c"strlen(\"-journal\")==8".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 17;
                    }
                    17 => {
                        n_name =
                            unsafe { strlen(z_name) } as i32 -
                                if e_type == 2 { 8 } else { 0 };
                        __state = 18;
                    }
                    18 => {
                        p_real = unsafe { (*p_fs_vfs).p_file_list };
                        __state = 19;
                    }
                    19 => { __state = 21; }
                    20 => {
                        if (p_real).is_null() as i32 != 0 {
                            __state = 25;
                        } else { __state = 24; }
                    }
                    21 => {
                        if !(p_real).is_null() &&
                                unsafe {
                                        strncmp(unsafe { (*p_real).z_name }, z_name, n_name as u64)
                                    } != 0 {
                            __state = 22;
                        } else { __state = 20; }
                    }
                    22 => { __state = 23; }
                    23 => {
                        p_real = unsafe { (*p_real).p_next };
                        __state = 21;
                    }
                    24 => { __state = 2; }
                    25 => { real_flags = flags & !256 | 512; __state = 26; }
                    26 => { __state = 27; }
                    27 => { __state = 28; }
                    28 => {
                        p_parent = unsafe { (*p_fs_vfs).p_parent };
                        __state = 29;
                    }
                    29 => {
                        if !(e_type == 1) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fsOpen".as_ptr() as *const i8,
                                    c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 619,
                                    c"eType==DATABASE_FILE".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        __state = 30;
                    }
                    30 => {
                        p_real =
                            unsafe {
                                    sqlite3_malloc((core::mem::size_of::<fs_real_file>() as u64
                                                + unsafe { (*p_parent).sz_os_file } as u64) as i32)
                                } as *mut fs_real_file;
                        __state = 31;
                    }
                    31 => {
                        if (p_real).is_null() as i32 != 0 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        unsafe {
                            memset(p_real as *mut (), 0,
                                core::mem::size_of::<fs_real_file>() as u64 +
                                    unsafe { (*p_parent).sz_os_file } as u64)
                        };
                        __state = 35;
                    }
                    33 => { rc = 7; __state = 34; }
                    34 => { __state = 2; }
                    35 => {
                        unsafe { (*p_real).z_name = z_name };
                        __state = 36;
                    }
                    36 => {
                        unsafe {
                            (*p_real).p_file =
                                unsafe { &raw mut *p_real.offset(1 as isize) } as
                                    *mut sqlite3_file
                        };
                        __state = 37;
                    }
                    37 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*p_parent).x_open.unwrap()
                                    })(p_parent, z_name, unsafe { (*p_real).p_file },
                                    real_flags, p_out_flags)
                            };
                        __state = 38;
                    }
                    38 => {
                        if rc != 0 { __state = 40; } else { __state = 39; }
                    }
                    39 => {
                        p_real_file = unsafe { (*p_real).p_file };
                        __state = 41;
                    }
                    40 => { __state = 2; }
                    41 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_file_size.unwrap()
                                    })(p_real_file, &mut size)
                            };
                        __state = 42;
                    }
                    42 => {
                        if rc != 0 { __state = 44; } else { __state = 43; }
                    }
                    43 => {
                        if size == 0 as i64 { __state = 46; } else { __state = 47; }
                    }
                    44 => { __state = 2; }
                    45 => {
                        if rc == 0 { __state = 56; } else { __state = 24; }
                    }
                    46 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_write.unwrap()
                                    })(p_real_file,
                                    b"\x00\x00".as_ptr() as *mut i8 as *const (), 1,
                                    (10485760 - 1) as i64)
                            };
                        __state = 48;
                    }
                    47 => { __state = 49; }
                    48 => {
                        unsafe { (*p_real).n_blob = 10485760 };
                        __state = 45;
                    }
                    49 => {
                        unsafe { (*p_real).n_blob = size as i32 };
                        __state = 50;
                    }
                    50 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_read.unwrap()
                                    })(p_real_file,
                                    &raw mut z_s[0 as usize] as *mut u8 as *mut (), 4, 0)
                            };
                        __state = 51;
                    }
                    51 => {
                        unsafe {
                            (*p_real).n_database =
                                ((z_s[0 as usize] as i32) << 24) +
                                            ((z_s[1 as usize] as i32) << 16) +
                                        ((z_s[2 as usize] as i32) << 8) + z_s[3 as usize] as i32
                        };
                        __state = 52;
                    }
                    52 => {
                        if rc == 0 { __state = 53; } else { __state = 45; }
                    }
                    53 => {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*p_real_file).p_methods }).x_read.unwrap()
                                    })(p_real_file,
                                    &raw mut z_s[0 as usize] as *mut u8 as *mut (), 4,
                                    (unsafe { (*p_real).n_blob } - 4) as i64)
                            };
                        __state = 54;
                    }
                    54 => {
                        if z_s[0 as usize] != 0 || z_s[1 as usize] != 0 ||
                                    z_s[2 as usize] != 0 || z_s[3 as usize] != 0 {
                            __state = 55;
                        } else { __state = 45; }
                    }
                    55 => {
                        unsafe {
                            (*p_real).n_journal = unsafe { (*p_real).n_blob }
                        };
                        __state = 45;
                    }
                    56 => {
                        unsafe {
                            (*p_real).p_next = unsafe { (*p_fs_vfs).p_file_list }
                        };
                        __state = 57;
                    }
                    57 => {
                        if !(unsafe { (*p_real).p_next }).is_null() {
                            __state = 59;
                        } else { __state = 58; }
                    }
                    58 => {
                        unsafe {
                            (*p_real).pp_this = unsafe { &mut (*p_fs_vfs).p_file_list }
                        };
                        __state = 60;
                    }
                    59 => {
                        unsafe {
                            (*unsafe { (*p_real).p_next }).pp_this =
                                unsafe { &mut (*p_real).p_next }
                        };
                        __state = 58;
                    }
                    60 => {
                        unsafe { (*p_fs_vfs).p_file_list = p_real };
                        __state = 24;
                    }
                    61 => { return rc; }
                    62 => {
                        if rc == 0 { __state = 63; } else { __state = 64; }
                    }
                    63 => { unsafe { (*p).p_real = p_real }; __state = 65; }
                    64 => {
                        if !(unsafe {
                                            (*unsafe { (*p_real).p_file }).p_methods
                                        }).is_null() {
                            __state = 67;
                        } else { __state = 66; }
                    }
                    65 => {
                        {
                            let __p = unsafe { &mut (*p_real).n_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 61;
                    }
                    66 => {
                        unsafe { sqlite3_free(p_real as *mut ()) };
                        __state = 61;
                    }
                    67 => {
                        unsafe {
                            (unsafe {
                                    (*unsafe {
                                                        (*unsafe { (*p_real).p_file }).p_methods
                                                    }).x_close.unwrap()
                                })(unsafe { (*p_real).p_file })
                        };
                        __state = 66;
                    }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn fs_delete(p_vfs: *mut sqlite3_vfs, z_path: *const i8,
    dir_sync: i32) -> i32 {
    let mut rc: i32 = 0;
    let p_fs_vfs: *const fs_vfs_t = p_vfs as *mut fs_vfs_t as *const fs_vfs_t;
    let mut p_real: *mut fs_real_file = core::ptr::null_mut();
    let mut p_f: *mut sqlite3_file = core::ptr::null_mut();
    let n_name: i32 = unsafe { strlen(z_path) } as i32 - 8;
    if !(unsafe { strlen(c"-journal".as_ptr() as *mut i8 as *const i8) } ==
                            8 as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsDelete".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 693,
                c"strlen(\"-journal\")==8".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                                strcmp(c"-journal".as_ptr() as *mut i8 as *const i8,
                                    unsafe { &*z_path.offset(n_name as isize) })
                            } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsDelete".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 694,
                c"strcmp(\"-journal\", &zPath[nName])==0".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    p_real = unsafe { (*p_fs_vfs).p_file_list };
    {
        '__b4: loop {
            if !(!(p_real).is_null() &&
                            unsafe {
                                    strncmp(unsafe { (*p_real).z_name }, z_path, n_name as u64)
                                } != 0) {
                break '__b4;
            }
            '__c4: loop { break '__c4; }
            p_real = unsafe { (*p_real).p_next };
        }
    }
    if !(p_real).is_null() {
        p_f = unsafe { (*p_real).p_file };
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_f).p_methods }).x_write.unwrap()
                    })(p_f,
                    b"\x00\x00\x00\x00\x00".as_ptr() as *mut i8 as *const (), 4,
                    (unsafe { (*p_real).n_blob } - 512) as i64)
            };
        if rc == 0 { unsafe { (*p_real).n_journal = 0 }; }
    }
    return rc;
}
extern "C" fn fs_access(p_vfs: *mut sqlite3_vfs, z_path: *const i8,
    flags: i32, p_res_out: *mut i32) -> i32 {
    let p_fs_vfs: *const fs_vfs_t = p_vfs as *mut fs_vfs_t as *const fs_vfs_t;
    let mut p_real: *const fs_real_file = core::ptr::null();
    let mut is_journal: i32 = 0;
    let mut n_name: i32 = unsafe { strlen(z_path) } as i32;
    if flags != 0 {
        let p_parent: *mut sqlite3_vfs =
            unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
        return unsafe {
                (unsafe {
                        (*p_parent).x_access.unwrap()
                    })(p_parent, z_path, flags, p_res_out)
            };
    }
    if !(unsafe { strlen(c"-journal".as_ptr() as *mut i8 as *const i8) } ==
                            8 as u64) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fsAccess".as_ptr() as *const i8,
                c"test_onefile.c".as_ptr() as *mut i8 as *const i8, 728,
                c"strlen(\"-journal\")==8".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if n_name > 8 &&
            unsafe {
                    strcmp(c"-journal".as_ptr() as *mut i8 as *const i8,
                        unsafe { &*z_path.offset((n_name - 8) as isize) })
                } == 0 {
        n_name -= 8;
        is_journal = 1;
    }
    p_real = unsafe { (*p_fs_vfs).p_file_list };
    {
        '__b5: loop {
            if !(!(p_real).is_null() &&
                            unsafe {
                                    strncmp(unsafe { (*p_real).z_name }, z_path, n_name as u64)
                                } != 0) {
                break '__b5;
            }
            '__c5: loop { break '__c5; }
            p_real = unsafe { (*p_real).p_next };
        }
    }
    unsafe {
        *p_res_out =
            (!(p_real).is_null() &&
                    ((is_journal == 0) as i32 != 0 ||
                        unsafe { (*p_real).n_journal } > 0)) as i32
    };
    return 0;
}
extern "C" fn fs_full_pathname(p_vfs: *mut sqlite3_vfs, z_path: *const i8,
    n_out: i32, z_out: *mut i8) -> i32 {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    return unsafe {
            (unsafe {
                    (*p_parent).x_full_pathname.unwrap()
                })(p_parent, z_path, n_out, z_out)
        };
}
extern "C" fn fs_dl_open(p_vfs: *mut sqlite3_vfs, z_path: *const i8)
    -> *mut () {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    return unsafe {
            (unsafe { (*p_parent).x_dl_open.unwrap() })(p_parent, z_path)
        };
}
extern "C" fn fs_dl_error(p_vfs: *mut sqlite3_vfs, n_byte: i32,
    z_err_msg: *mut i8) -> () {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    unsafe {
        (unsafe {
                (*p_parent).x_dl_error.unwrap()
            })(p_parent, n_byte, z_err_msg)
    };
}
extern "C" fn fs_dl_sym(p_vfs: *mut sqlite3_vfs, p_h: *mut (),
    z_sym: *const i8) -> unsafe extern "C" fn() -> () {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    return unsafe {
            (unsafe { (*p_parent).x_dl_sym.unwrap() })(p_parent, p_h, z_sym)
        };
}
extern "C" fn fs_dl_close(p_vfs: *mut sqlite3_vfs, p_handle: *mut ()) -> () {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    unsafe {
        (unsafe { (*p_parent).x_dl_close.unwrap() })(p_parent, p_handle)
    };
}
extern "C" fn fs_randomness(p_vfs: *mut sqlite3_vfs, n_byte: i32,
    z_buf_out: *mut i8) -> i32 {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    return unsafe {
            (unsafe {
                    (*p_parent).x_randomness.unwrap()
                })(p_parent, n_byte, z_buf_out)
        };
}
extern "C" fn fs_sleep(p_vfs: *mut sqlite3_vfs, n_micro: i32) -> i32 {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    return unsafe {
            (unsafe { (*p_parent).x_sleep.unwrap() })(p_parent, n_micro)
        };
}
extern "C" fn fs_current_time(p_vfs: *mut sqlite3_vfs, p_time_out: *mut f64)
    -> i32 {
    let p_parent: *mut sqlite3_vfs =
        unsafe { (*(p_vfs as *mut fs_vfs_t)).p_parent };
    return unsafe {
            (unsafe {
                    (*p_parent).x_current_time.unwrap()
                })(p_parent, p_time_out)
        };
}
static mut fs_vfs: fs_vfs_t =
    fs_vfs_t {
        base: sqlite3_vfs {
            i_version: 1,
            sz_os_file: 0,
            mx_pathname: 0,
            p_next: core::ptr::null_mut(),
            z_name: c"fs".as_ptr() as *const i8,
            p_app_data: core::ptr::null_mut(),
            x_open: Some(fs_open),
            x_delete: Some(fs_delete),
            x_access: Some(fs_access),
            x_full_pathname: Some(fs_full_pathname),
            x_dl_open: Some(fs_dl_open),
            x_dl_error: Some(fs_dl_error),
            x_dl_sym: Some(fs_dl_sym),
            x_dl_close: Some(fs_dl_close),
            x_randomness: Some(fs_randomness),
            x_sleep: Some(fs_sleep),
            x_current_time: Some(fs_current_time),
            x_get_last_error: None,
            x_current_time_int64: None,
            x_set_system_call: None,
            x_get_system_call: None,
            x_next_system_call: None,
        },
        p_file_list: core::ptr::null_mut(),
        p_parent: core::ptr::null_mut(),
    };
#[unsafe(no_mangle)]
pub extern "C" fn fs_register() -> i32 {
    unsafe {
        if !(fs_vfs.p_parent).is_null() { return 0; }
        fs_vfs.p_parent = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        fs_vfs.base.mx_pathname = unsafe { (*fs_vfs.p_parent).mx_pathname };
        fs_vfs.base.sz_os_file =
            if core::mem::size_of::<tmp_file>() as u64 >
                        core::mem::size_of::<fs_file>() as u64 {
                    core::mem::size_of::<tmp_file>() as u64
                } else { core::mem::size_of::<fs_file>() as u64 } as i32;
        return unsafe { sqlite3_vfs_register(&mut fs_vfs.base, 0) };
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
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}