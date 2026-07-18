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
struct DbdataTable {
    base: sqlite3_vtab,
    db: *mut sqlite3,
    p_stmt: *mut sqlite3_stmt,
    b_ptr: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DbdataCursor {
    base: sqlite3_vtab_cursor,
    p_stmt: *mut sqlite3_stmt,
    i_pgno: i32,
    a_page: *mut u8,
    n_page: i32,
    n_cell: i32,
    i_cell: i32,
    b_one_page: i32,
    sz_db: i32,
    i_rowid: sqlite3_int64,
    rec: DbdataBuffer,
    n_rec: sqlite3_int64,
    n_hdr: sqlite3_int64,
    i_field: i32,
    p_hdr_ptr: *mut u8,
    p_ptr: *mut u8,
    enc: u32,
    i_intkey: sqlite3_int64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DbdataBuffer {
    a_buf: *mut u8,
    n_buf: sqlite3_int64,
}
extern "C" fn dbdata_buffer_size(p_buf_1: &mut DbdataBuffer,
    n_min_1: sqlite3_int64) -> i32 {
    if n_min_1 > (*p_buf_1).n_buf {
        let n_new: sqlite3_int64 = n_min_1 + 16384 as sqlite3_int64;
        let a_new: *mut u8 =
            unsafe {
                    sqlite3_realloc64((*p_buf_1).a_buf as *mut (),
                        n_new as sqlite3_uint64)
                } as *mut u8;
        if a_new == core::ptr::null_mut() { return 7; }
        (*p_buf_1).a_buf = a_new;
        (*p_buf_1).n_buf = n_new;
    }
    return 0;
}
extern "C" fn dbdata_buffer_free(p_buf_1: *mut DbdataBuffer) -> () {
    unsafe { sqlite3_free(unsafe { (*p_buf_1).a_buf } as *mut ()) };
    unsafe {
        memset(p_buf_1 as *mut (), 0,
            core::mem::size_of::<DbdataBuffer>() as u64)
    };
}
extern "C" fn dbdata_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_tab: *mut DbdataTable = core::ptr::null_mut();
    let mut rc: i32 =
        unsafe {
            sqlite3_declare_vtab(db,
                if !(p_aux_1).is_null() {
                        c"CREATE TABLE x(  pgno INTEGER,  child INTEGER,  schema TEXT HIDDEN)".as_ptr()
                            as *mut i8
                    } else {
                        c"CREATE TABLE x(  pgno INTEGER,  cell INTEGER,  field INTEGER,  value ANY,  schema TEXT HIDDEN)".as_ptr()
                            as *mut i8
                    } as *const i8)
        };
    { let _ = argc; };
    { let _ = argv; };
    { let _ = pz_err_1; };
    unsafe { sqlite3_vtab_config(db, 4) };
    if rc == 0 {
        p_tab =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<DbdataTable>() as
                            sqlite3_uint64)
                } as *mut DbdataTable;
        if p_tab == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe {
                memset(p_tab as *mut (), 0,
                    core::mem::size_of::<DbdataTable>() as u64)
            };
            unsafe { (*p_tab).db = db };
            unsafe {
                (*p_tab).b_ptr = (p_aux_1 != core::ptr::null_mut()) as i32
            };
        }
    }
    unsafe { *pp_vtab_1 = p_tab as *mut sqlite3_vtab };
    return rc;
}
extern "C" fn dbdata_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p_tab: *const DbdataTable =
        p_vtab_1 as *mut DbdataTable as *const DbdataTable;
    if !(p_tab).is_null() {
        unsafe { sqlite3_finalize(unsafe { (*p_tab).p_stmt }) };
        unsafe { sqlite3_free(p_vtab_1 as *mut ()) };
    }
    return 0;
}
extern "C" fn dbdata_best_index(tab: *mut sqlite3_vtab,
    p_idx_1: *mut sqlite3_index_info) -> i32 {
    let p_tab: *const DbdataTable =
        tab as *mut DbdataTable as *const DbdataTable;
    let mut i: i32 = 0;
    let mut i_schema: i32 = -1;
    let mut i_pgno: i32 = -1;
    let col_schema: i32 = if unsafe { (*p_tab).b_ptr } != 0 { 2 } else { 4 };
    {
        i = 0;
        '__b0: loop {
            if !(i < unsafe { (*p_idx_1).n_constraint }) { break '__b0; }
            '__c0: loop {
                let p: *const sqlite3_index_constraint =
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_idx_1).a_constraint.offset(i as isize)
                                    }
                        } as *const sqlite3_index_constraint;
                if unsafe { (*p).op } as i32 == 2 {
                    if unsafe { (*p).i_column } == col_schema {
                        if unsafe { (*p).usable } as i32 == 0 { return 19; }
                        i_schema = i;
                    }
                    if unsafe { (*p).i_column } == 0 &&
                            unsafe { (*p).usable } != 0 {
                        i_pgno = i;
                    }
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i_schema >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_1).a_constraint_usage.offset(i_schema as isize)
                        }).argv_index = 1
        };
        unsafe {
            (*unsafe {
                            (*p_idx_1).a_constraint_usage.offset(i_schema as isize)
                        }).omit = 1 as u8
        };
    }
    if i_pgno >= 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_1).a_constraint_usage.offset(i_pgno as isize)
                        }).argv_index = 1 + (i_schema >= 0) as i32
        };
        unsafe {
            (*unsafe {
                            (*p_idx_1).a_constraint_usage.offset(i_pgno as isize)
                        }).omit = 1 as u8
        };
        unsafe { (*p_idx_1).estimated_cost = 100 as f64 };
        unsafe { (*p_idx_1).estimated_rows = 50 as sqlite3_int64 };
        if unsafe { (*p_tab).b_ptr } == 0 &&
                    unsafe { (*p_idx_1).n_order_by } != 0 &&
                unsafe {
                            (*unsafe { (*p_idx_1).a_order_by.offset(0 as isize) }).desc
                        } as i32 == 0 {
            let i_col: i32 =
                unsafe {
                    (*unsafe {
                                (*p_idx_1).a_order_by.offset(0 as isize)
                            }).i_column
                };
            if unsafe { (*p_idx_1).n_order_by } == 1 {
                unsafe {
                    (*p_idx_1).order_by_consumed =
                        (i_col == 0 || i_col == 1) as i32
                };
            } else if unsafe { (*p_idx_1).n_order_by } == 2 &&
                        unsafe {
                                    (*unsafe { (*p_idx_1).a_order_by.offset(1 as isize) }).desc
                                } as i32 == 0 && i_col == 0 {
                unsafe {
                    (*p_idx_1).order_by_consumed =
                        (unsafe {
                                    (*unsafe {
                                                (*p_idx_1).a_order_by.offset(1 as isize)
                                            }).i_column
                                } == 1) as i32
                };
            }
        }
    } else {
        unsafe { (*p_idx_1).estimated_cost = 100000000 as f64 };
        unsafe { (*p_idx_1).estimated_rows = 1000000000 as sqlite3_int64 };
    }
    unsafe {
        (*p_idx_1).idx_num =
            if i_schema >= 0 { 1 } else { 0 } |
                if i_pgno >= 0 { 2 } else { 0 }
    };
    return 0;
}
extern "C" fn dbdata_open(p_v_tab_1: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let mut p_csr: *mut DbdataCursor = core::ptr::null_mut();
    p_csr =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<DbdataCursor>() as
                        sqlite3_uint64)
            } as *mut DbdataCursor;
    if p_csr == core::ptr::null_mut() {
        return 7;
    } else {
        unsafe {
            memset(p_csr as *mut (), 0,
                core::mem::size_of::<DbdataCursor>() as u64)
        };
        unsafe { (*p_csr).base.p_vtab = p_v_tab_1 };
    }
    unsafe { *pp_cursor_1 = p_csr as *mut sqlite3_vtab_cursor };
    return 0;
}
extern "C" fn dbdata_reset_cursor(p_csr_1: &mut DbdataCursor) -> () {
    let p_tab: *mut DbdataTable = (*p_csr_1).base.p_vtab as *mut DbdataTable;
    if unsafe { (*p_tab).p_stmt } == core::ptr::null_mut() {
        unsafe { (*p_tab).p_stmt = (*p_csr_1).p_stmt };
    } else { unsafe { sqlite3_finalize((*p_csr_1).p_stmt) }; }
    (*p_csr_1).p_stmt = core::ptr::null_mut();
    (*p_csr_1).i_pgno = 1;
    (*p_csr_1).i_cell = 0;
    (*p_csr_1).i_field = 0;
    (*p_csr_1).b_one_page = 0;
    unsafe { sqlite3_free((*p_csr_1).a_page as *mut ()) };
    dbdata_buffer_free(&mut (*p_csr_1).rec);
    (*p_csr_1).a_page = core::ptr::null_mut();
    (*p_csr_1).n_rec = 0 as sqlite3_int64;
}
extern "C" fn dbdata_close(p_cursor_1: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *mut DbdataCursor = p_cursor_1 as *mut DbdataCursor;
    dbdata_reset_cursor(unsafe { &mut *p_csr });
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}
extern "C" fn get_uint16(a: *const u8) -> u32 {
    return ((unsafe { *a.offset(0 as isize) } as i32) << 8 |
                unsafe { *a.offset(1 as isize) } as i32) as u32;
}
extern "C" fn get_uint32(a: *const u8) -> u32 {
    return (unsafe { *a.offset(0 as isize) } as u32) << 24 |
                    (unsafe { *a.offset(1 as isize) } as u32) << 16 |
                (unsafe { *a.offset(2 as isize) } as u32) << 8 |
            unsafe { *a.offset(3 as isize) } as u32;
}
extern "C" fn dbdata_load_page(p_csr_1: &DbdataCursor, pgno: u32,
    pp_page_1: &mut *mut u8, pn_page_1: &mut i32) -> i32 {
    let mut rc2: i32 = 0;
    let mut rc: i32 = 0;
    let p_stmt: *mut sqlite3_stmt = (*p_csr_1).p_stmt;
    *pp_page_1 = core::ptr::null_mut();
    *pn_page_1 = 0;
    if pgno > 0 as u32 {
        unsafe { sqlite3_bind_int64(p_stmt, 2, pgno as sqlite3_int64) };
        if 100 == unsafe { sqlite3_step(p_stmt) } {
            let n_copy: i32 = unsafe { sqlite3_column_bytes(p_stmt, 0) };
            if n_copy > 0 {
                let mut p_page: *mut u8 = core::ptr::null_mut();
                p_page =
                    unsafe {
                            sqlite3_malloc64((n_copy + 100) as sqlite3_uint64)
                        } as *mut u8;
                if p_page == core::ptr::null_mut() {
                    rc = 7;
                } else {
                    let p_copy: *const u8 =
                        unsafe { sqlite3_column_blob(p_stmt, 0) } as *const u8;
                    unsafe {
                        memcpy(p_page as *mut (), p_copy as *const (),
                            n_copy as u64)
                    };
                    unsafe {
                        memset(unsafe { &raw mut *p_page.offset(n_copy as isize) }
                                as *mut (), 0, 100 as u64)
                    };
                }
                *pp_page_1 = p_page;
                *pn_page_1 = n_copy;
            }
        }
        rc2 = unsafe { sqlite3_reset(p_stmt) };
        if rc == 0 { rc = rc2; }
    }
    return rc;
}
extern "C" fn dbdata_get_varint(z: *const u8, p_val_1: &mut sqlite3_int64)
    -> i32 {
    let mut u: sqlite3_uint64 = 0 as sqlite3_uint64;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b1: loop {
            if !(i < 8) { break '__b1; }
            '__c1: loop {
                u =
                    (u << 7) +
                        (unsafe { *z.offset(i as isize) } as i32 & 127) as
                            sqlite3_uint64;
                if unsafe { *z.offset(i as isize) } as i32 & 128 == 0 {
                    *p_val_1 = u as sqlite3_int64;
                    return i + 1;
                }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    u =
        (u << 8) +
            (unsafe { *z.offset(i as isize) } as i32 & 255) as sqlite3_uint64;
    *p_val_1 = u as sqlite3_int64;
    return 9;
}
extern "C" fn dbdata_get_varint_u32(z: *const u8, p_val_1: &mut sqlite3_int64)
    -> i32 {
    let mut val: sqlite3_int64 = 0 as sqlite3_int64;
    let n_ret: i32 = dbdata_get_varint(z, &mut val);
    if val < 0 as i64 || val > 4294967295u32 as i64 {
        val = 0 as sqlite3_int64;
    }
    *p_val_1 = val;
    return n_ret;
}
extern "C" fn dbdata_value_bytes(e_type_1: i32) -> i32 {
    '__s2:
        {
        match e_type_1 {
            0 => { return 0; return 1; }
            8 => { return 0; return 1; }
            9 => { return 0; return 1; }
            10 => { return 0; return 1; }
            11 => { return 0; return 1; }
            1 => { return 1; }
            2 => { return 2; }
            3 => { return 3; }
            4 => { return 4; }
            5 => { return 6; }
            6 => {
                return 8;
                if e_type_1 > 0 { return (e_type_1 - 12) / 2; }
                return 0;
            }
            7 => {
                return 8;
                if e_type_1 > 0 { return (e_type_1 - 12) / 2; }
                return 0;
            }
            _ => { if e_type_1 > 0 { return (e_type_1 - 12) / 2; } return 0; }
        }
    }
}
extern "C" fn dbdata_value(p_ctx_1: *mut sqlite3_context, enc: u32,
    e_type_1: i32, mut p_data_1: *mut u8, n_data_1: sqlite3_int64) -> () {
    if e_type_1 >= 0 {
        if dbdata_value_bytes(e_type_1) as sqlite3_int64 <= n_data_1 {
            '__s3:
                {
                match e_type_1 {
                    0 => { unsafe { sqlite3_result_null(p_ctx_1) }; }
                    10 => { unsafe { sqlite3_result_null(p_ctx_1) }; }
                    11 => { unsafe { sqlite3_result_null(p_ctx_1) }; }
                    8 => { unsafe { sqlite3_result_int(p_ctx_1, 0) }; }
                    9 => { unsafe { sqlite3_result_int(p_ctx_1, 1) }; }
                    1 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    2 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    3 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    4 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    5 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    6 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    7 => {
                        {
                            let mut v: sqlite3_uint64 =
                                unsafe { *p_data_1.offset(0 as isize) } as i8 as
                                    sqlite3_uint64;
                            {
                                let __p = &mut p_data_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            '__s4:
                                {
                                match e_type_1 {
                                    7 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    6 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    5 => {
                                        v =
                                            (v << 16) +
                                                    ((unsafe { *p_data_1.offset(0 as isize) } as i32) << 8) as
                                                        sqlite3_uint64 +
                                                unsafe { *p_data_1.offset(1 as isize) } as sqlite3_uint64;
                                        {
                                            let __n = 2;
                                            let __p = &mut p_data_1;
                                            *__p = unsafe { (*__p).offset(__n as isize) };
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    4 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    3 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    2 => {
                                        v =
                                            (v << 8) +
                                                unsafe { *p_data_1.offset(0 as isize) } as sqlite3_uint64;
                                        {
                                            let __p = &mut p_data_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            if e_type_1 == 7 {
                                let mut r: f64 = 0.0;
                                unsafe {
                                    memcpy(&raw mut r as *mut (), &raw mut v as *const (),
                                        core::mem::size_of::<f64>() as u64)
                                };
                                unsafe { sqlite3_result_double(p_ctx_1, r) };
                            } else {
                                unsafe {
                                    sqlite3_result_int64(p_ctx_1, v as sqlite3_int64)
                                };
                            }
                            break '__s3;
                        }
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                    _ => {
                        {
                            let n: i32 = (e_type_1 - 12) / 2;
                            if e_type_1 % 2 != 0 {
                                '__s5:
                                    {
                                    match enc {
                                        3 => {
                                            unsafe {
                                                sqlite3_result_text16be(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        2 => {
                                            unsafe {
                                                sqlite3_result_text16le(p_ctx_1,
                                                    p_data_1 as *mut () as *const (), n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                        _ => {
                                            unsafe {
                                                sqlite3_result_text(p_ctx_1,
                                                    p_data_1 as *mut i8 as *const i8, n,
                                                    Some(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn(*mut ())
                                                                        -> ()>(-1 as isize as *const ())
                                                        }))
                                            };
                                        }
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_result_blob(p_ctx_1, p_data_1 as *const (), n,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                        }
                    }
                }
            }
        } else {
            if e_type_1 == 7 {
                unsafe { sqlite3_result_double(p_ctx_1, 0.0) };
            } else if e_type_1 < 7 {
                unsafe { sqlite3_result_int(p_ctx_1, 0) };
            } else if e_type_1 % 2 != 0 {
                unsafe {
                    sqlite3_result_text(p_ctx_1,
                        c"".as_ptr() as *mut i8 as *const i8, 0, None)
                };
            } else {
                unsafe {
                    sqlite3_result_blob(p_ctx_1,
                        c"".as_ptr() as *mut i8 as *const (), 0, None)
                };
            }
        }
    }
}
extern "C" fn dbdata_next(p_cursor_1: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *mut DbdataCursor = p_cursor_1 as *mut DbdataCursor;
    let p_tab: *const DbdataTable =
        unsafe { (*p_cursor_1).p_vtab } as *mut DbdataTable as
            *const DbdataTable;
    {
        let __p = unsafe { &mut (*p_csr).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    loop {
        let mut rc: i32 = 0;
        let mut i_off: i32 =
            if unsafe { (*p_csr).i_pgno } == 1 { 100 } else { 0 };
        let mut b_next_page: i32 = 0;
        if unsafe { (*p_csr).a_page } == core::ptr::null_mut() {
            loop {
                if unsafe { (*p_csr).b_one_page } == 0 &&
                        unsafe { (*p_csr).i_pgno } > unsafe { (*p_csr).sz_db } {
                    return 0;
                }
                rc =
                    dbdata_load_page(unsafe { &*p_csr },
                        unsafe { (*p_csr).i_pgno } as u32,
                        unsafe { &mut (*p_csr).a_page },
                        unsafe { &mut (*p_csr).n_page });
                if rc != 0 { return rc; }
                if !(unsafe { (*p_csr).a_page }).is_null() &&
                        unsafe { (*p_csr).n_page } >= 256 {
                    break;
                }
                unsafe {
                    sqlite3_free(unsafe { (*p_csr).a_page } as *mut ())
                };
                unsafe { (*p_csr).a_page = core::ptr::null_mut() };
                if unsafe { (*p_csr).b_one_page } != 0 { return 0; }
                {
                    let __p = unsafe { &mut (*p_csr).i_pgno };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            if !(i_off + 3 + 2 <= unsafe { (*p_csr).n_page }) as i32 as i64 !=
                    0 {
                unsafe {
                    __assert_rtn(c"dbdataNext".as_ptr() as *const i8,
                        c"dbdata.c".as_ptr() as *mut i8 as *const i8, 577,
                        c"iOff+3+2<=pCsr->nPage".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            unsafe {
                (*p_csr).i_cell =
                    if unsafe { (*p_tab).b_ptr } != 0 { -2 } else { 0 }
            };
            unsafe {
                (*p_csr).n_cell =
                    get_uint16(unsafe {
                                    &raw mut *unsafe {
                                                (*p_csr).a_page.offset((i_off + 3) as isize)
                                            }
                                } as *const u8) as i32
            };
            if unsafe { (*p_csr).n_cell } >
                    (unsafe { (*p_csr).n_page } - 8) / 6 {
                unsafe {
                    (*p_csr).n_cell = (unsafe { (*p_csr).n_page } - 8) / 6
                };
            }
        }
        if unsafe { (*p_tab).b_ptr } != 0 {
            if unsafe { *unsafe { (*p_csr).a_page.offset(i_off as isize) } }
                            as i32 != 2 &&
                    unsafe {
                                *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                            } as i32 != 5 {
                unsafe { (*p_csr).i_cell = unsafe { (*p_csr).n_cell } };
            }
            {
                let __p = unsafe { &mut (*p_csr).i_cell };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p_csr).i_cell } >= unsafe { (*p_csr).n_cell } {
                unsafe {
                    sqlite3_free(unsafe { (*p_csr).a_page } as *mut ())
                };
                unsafe { (*p_csr).a_page = core::ptr::null_mut() };
                if unsafe { (*p_csr).b_one_page } != 0 { return 0; }
                {
                    let __p = unsafe { &mut (*p_csr).i_pgno };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            } else { return 0; }
        } else {
            if !(unsafe { (*p_csr).rec.a_buf } != core::ptr::null_mut() ||
                                    unsafe { (*p_csr).n_rec } == 0 as i64) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"dbdataNext".as_ptr() as *const i8,
                        c"dbdata.c".as_ptr() as *mut i8 as *const i8, 600,
                        c"pCsr->rec.aBuf!=0 || pCsr->nRec==0".as_ptr() as *mut i8 as
                            *const i8)
                }
            } else { { let _ = 0; } };
            if unsafe { (*p_csr).n_rec } == 0 as i64 {
                let mut b_has_rowid: i32 = 0;
                let mut n_pointer: i32 = 0;
                let mut n_payload: sqlite3_int64 = 0 as sqlite3_int64;
                let mut n_hdr: sqlite3_int64 = 0 as sqlite3_int64;
                let mut i_hdr: i32 = 0;
                let mut u: i32 = 0;
                let mut x: i32 = 0;
                let mut n_local: i32 = 0;
                '__s8:
                    {
                    match unsafe {
                            *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                        } {
                        2 => { n_pointer = 4; }
                        10 => {}
                        13 => { b_has_rowid = 1; }
                        _ => {
                            unsafe { (*p_csr).i_cell = unsafe { (*p_csr).n_cell } };
                        }
                    }
                }
                if unsafe { (*p_csr).i_cell } >= unsafe { (*p_csr).n_cell } {
                    b_next_page = 1;
                } else {
                    let i_cell_ptr: i32 =
                        i_off + 8 + n_pointer + unsafe { (*p_csr).i_cell } * 2;
                    if i_cell_ptr > unsafe { (*p_csr).n_page } {
                        b_next_page = 1;
                    } else {
                        i_off =
                            get_uint16(unsafe {
                                            &raw mut *unsafe {
                                                        (*p_csr).a_page.offset(i_cell_ptr as isize)
                                                    }
                                        } as *const u8) as i32;
                    }
                    i_off += n_pointer;
                    if b_next_page != 0 || i_off > unsafe { (*p_csr).n_page } ||
                            i_off <= i_cell_ptr {
                        b_next_page = 1;
                    } else {
                        i_off +=
                            dbdata_get_varint_u32(unsafe {
                                        &raw mut *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                                    } as *const u8, &mut n_payload);
                        if n_payload > 2147483392 as i64 {
                            n_payload &= 16383 as sqlite3_int64;
                        }
                        if n_payload == 0 as i64 { n_payload = 1 as sqlite3_int64; }
                    }
                    if b_has_rowid != 0 && (b_next_page == 0) as i32 != 0 &&
                            i_off < unsafe { (*p_csr).n_page } {
                        i_off +=
                            dbdata_get_varint(unsafe {
                                        &raw mut *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                                    } as *const u8, unsafe { &mut (*p_csr).i_intkey });
                    }
                    u = unsafe { (*p_csr).n_page };
                    if b_has_rowid != 0 {
                        x = u - 35;
                    } else { x = (u - 12) * 64 / 255 - 23; }
                    if n_payload <= x as i64 {
                        n_local = n_payload as i32;
                    } else {
                        let mut m: i32 = 0;
                        let mut k: i32 = 0;
                        m = (u - 12) * 32 / 255 - 23;
                        k =
                            (m as sqlite3_int64 +
                                    (n_payload - m as sqlite3_int64) % (u - 4) as sqlite3_int64)
                                as i32;
                        if k <= x { n_local = k; } else { n_local = m; }
                    }
                    if b_next_page != 0 ||
                            n_local + i_off > unsafe { (*p_csr).n_page } {
                        b_next_page = 1;
                    } else {
                        rc =
                            dbdata_buffer_size(unsafe { &mut (*p_csr).rec },
                                n_payload + 100 as sqlite3_int64);
                        if rc != 0 { return rc; }
                        if !(unsafe { (*p_csr).rec.a_buf } != core::ptr::null_mut())
                                        as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"dbdataNext".as_ptr() as *const i8,
                                    c"dbdata.c".as_ptr() as *mut i8 as *const i8, 682,
                                    c"pCsr->rec.aBuf!=0".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        if !(n_payload != 0 as i64) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"dbdataNext".as_ptr() as *const i8,
                                    c"dbdata.c".as_ptr() as *mut i8 as *const i8, 683,
                                    c"nPayload!=0".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        unsafe {
                            memcpy(unsafe { (*p_csr).rec.a_buf } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                                    } as *const (), n_local as u64)
                        };
                        i_off += n_local;
                        if n_payload > n_local as i64 {
                            let mut n_rem: sqlite3_int64 =
                                n_payload - n_local as sqlite3_int64;
                            let mut pgno_ovfl: u32 =
                                get_uint32(unsafe {
                                            &raw mut *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                                        } as *const u8);
                            while n_rem > 0 as i64 {
                                let mut a_ovfl: *mut u8 = core::ptr::null_mut();
                                let mut n_ovfl: i32 = 0;
                                let mut n_copy: i32 = 0;
                                rc =
                                    dbdata_load_page(unsafe { &*p_csr }, pgno_ovfl, &mut a_ovfl,
                                        &mut n_ovfl);
                                if !(rc != 0 || a_ovfl == core::ptr::null_mut() ||
                                                        n_ovfl == unsafe { (*p_csr).n_page }) as i32 as i64 != 0 {
                                    unsafe {
                                        __assert_rtn(c"dbdataNext".as_ptr() as *const i8,
                                            c"dbdata.c".as_ptr() as *mut i8 as *const i8, 698,
                                            c"rc!=SQLITE_OK || aOvfl==0 || nOvfl==pCsr->nPage".as_ptr()
                                                    as *mut i8 as *const i8)
                                    }
                                } else { { let _ = 0; } };
                                if rc != 0 { return rc; }
                                if a_ovfl == core::ptr::null_mut() { break; }
                                n_copy = u - 4;
                                if n_copy as sqlite3_int64 > n_rem {
                                    n_copy = n_rem as i32;
                                }
                                unsafe {
                                    memcpy(unsafe {
                                                &raw mut *unsafe {
                                                            (*p_csr).rec.a_buf.offset((n_payload - n_rem) as isize)
                                                        }
                                            } as *mut (),
                                        unsafe { &raw mut *a_ovfl.offset(4 as isize) } as *const (),
                                        n_copy as u64)
                                };
                                n_rem -= n_copy as sqlite3_int64;
                                pgno_ovfl = get_uint32(a_ovfl as *const u8);
                                unsafe { sqlite3_free(a_ovfl as *mut ()) };
                            }
                            n_payload -= n_rem;
                        }
                        unsafe {
                            memset(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_csr).rec.a_buf.offset(n_payload as isize)
                                                }
                                    } as *mut (), 0, 100 as u64)
                        };
                        unsafe { (*p_csr).n_rec = n_payload };
                        i_hdr =
                            dbdata_get_varint_u32(unsafe { (*p_csr).rec.a_buf } as
                                    *const u8, &mut n_hdr);
                        if n_hdr > n_payload { n_hdr = 0 as sqlite3_int64; }
                        unsafe { (*p_csr).n_hdr = n_hdr };
                        unsafe {
                            (*p_csr).p_hdr_ptr =
                                unsafe {
                                    unsafe { (*p_csr).rec.a_buf.offset(i_hdr as isize) }
                                }
                        };
                        unsafe {
                            (*p_csr).p_ptr =
                                unsafe {
                                    unsafe {
                                        (*p_csr).rec.a_buf.offset(unsafe { (*p_csr).n_hdr } as
                                                isize)
                                    }
                                }
                        };
                        unsafe {
                            (*p_csr).i_field = if b_has_rowid != 0 { -1 } else { 0 }
                        };
                    }
                }
            } else {
                {
                    let __p = unsafe { &mut (*p_csr).i_field };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                if unsafe { (*p_csr).i_field } > 0 {
                    let mut i_type: sqlite3_int64 = 0 as sqlite3_int64;
                    if unsafe { (*p_csr).p_hdr_ptr } >=
                                unsafe {
                                    unsafe {
                                        (*p_csr).rec.a_buf.offset(unsafe { (*p_csr).n_rec } as
                                                isize)
                                    }
                                } || unsafe { (*p_csr).i_field } >= 32676 {
                        b_next_page = 1;
                    } else {
                        let mut sz_field: i32 = 0;
                        unsafe {
                            {
                                let __n =
                                    dbdata_get_varint_u32(unsafe { (*p_csr).p_hdr_ptr } as
                                            *const u8, &mut i_type);
                                let __p = &mut (*p_csr).p_hdr_ptr;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            }
                        };
                        sz_field = dbdata_value_bytes(i_type as i32);
                        if (unsafe { (*p_csr).n_rec } -
                                        unsafe {
                                                    unsafe {
                                                        (*p_csr).p_ptr.offset_from(unsafe { (*p_csr).rec.a_buf })
                                                    }
                                                } as i64 as sqlite3_int64) < sz_field as i64 {
                            unsafe {
                                (*p_csr).p_ptr =
                                    unsafe {
                                        unsafe {
                                            (*p_csr).rec.a_buf.offset(unsafe { (*p_csr).n_rec } as
                                                    isize)
                                        }
                                    }
                            };
                        } else {
                            unsafe {
                                {
                                    let __n = sz_field;
                                    let __p = &mut (*p_csr).p_ptr;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                }
                            };
                        }
                    }
                }
            }
            if b_next_page != 0 {
                unsafe {
                    sqlite3_free(unsafe { (*p_csr).a_page } as *mut ())
                };
                unsafe { (*p_csr).a_page = core::ptr::null_mut() };
                unsafe { (*p_csr).n_rec = 0 as sqlite3_int64 };
                if unsafe { (*p_csr).b_one_page } != 0 { return 0; }
                {
                    let __p = unsafe { &mut (*p_csr).i_pgno };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            } else {
                if unsafe { (*p_csr).i_field } < 0 ||
                        unsafe { (*p_csr).p_hdr_ptr } <
                            unsafe {
                                unsafe {
                                    (*p_csr).rec.a_buf.offset(unsafe { (*p_csr).n_hdr } as
                                            isize)
                                }
                            } {
                    return 0;
                }
                unsafe { (*p_csr).n_rec = 0 as sqlite3_int64 };
                {
                    let __p = unsafe { &mut (*p_csr).i_cell };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
        }
    }
    if (0 == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dbdataNext".as_ptr() as *const i8,
                c"dbdata.c".as_ptr() as *mut i8 as *const i8, 763,
                c"!\"can\'t get here\"".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return 0;
}
extern "C" fn dbdata_eof(p_cursor_1: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *const DbdataCursor =
        p_cursor_1 as *mut DbdataCursor as *const DbdataCursor;
    return (unsafe { (*p_csr).a_page } == core::ptr::null_mut()) as i32;
}
extern "C" fn dbdata_is_function(z_schema_1: *const i8) -> i32 {
    let n: u64 = unsafe { strlen(z_schema_1) };
    if n > 2 as u64 &&
                unsafe { *z_schema_1.add((n - 2 as u64) as usize) } as i32 ==
                    '(' as i32 &&
            unsafe { *z_schema_1.add((n - 1 as u64) as usize) } as i32 ==
                ')' as i32 {
        return n as i32 - 2;
    }
    return 0;
}
extern "C" fn dbdata_dbsize(p_csr_1: &mut DbdataCursor, z_schema_1: *const i8)
    -> i32 {
    let p_tab: *const DbdataTable =
        (*p_csr_1).base.p_vtab as *mut DbdataTable as *const DbdataTable;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut rc2: i32 = 0;
    let mut n_func: i32 = 0;
    let mut p_stmt: *mut sqlite3_stmt = core::ptr::null_mut();
    if { n_func = dbdata_is_function(z_schema_1); n_func } > 0 {
        z_sql =
            unsafe {
                sqlite3_mprintf(c"SELECT %.*s(0)".as_ptr() as *mut i8 as
                        *const i8, n_func, z_schema_1)
            };
    } else {
        z_sql =
            unsafe {
                sqlite3_mprintf(c"PRAGMA %Q.page_count".as_ptr() as *mut i8 as
                        *const i8, z_schema_1)
            };
    }
    if z_sql == core::ptr::null_mut() { return 7; }
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p_tab).db }, z_sql as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc == 0 && unsafe { sqlite3_step(p_stmt) } == 100 {
        (*p_csr_1).sz_db = unsafe { sqlite3_column_int(p_stmt, 0) };
    }
    rc2 = unsafe { sqlite3_finalize(p_stmt) };
    if rc == 0 { rc = rc2; }
    return rc;
}
extern "C" fn dbdata_get_encoding(p_csr_1: *mut DbdataCursor) -> i32 {
    let mut rc: i32 = 0;
    let mut n_pg1: i32 = 0;
    let mut a_pg1: *mut u8 = core::ptr::null_mut();
    rc =
        dbdata_load_page(unsafe { &*p_csr_1 }, 1 as u32, &mut a_pg1,
            &mut n_pg1);
    if rc == 0 && n_pg1 >= 56 + 4 {
        unsafe {
            (*p_csr_1).enc =
                get_uint32(unsafe { &raw mut *a_pg1.offset(56 as isize) } as
                        *const u8)
        };
    }
    unsafe { sqlite3_free(a_pg1 as *mut ()) };
    return rc;
}
extern "C" fn dbdata_filter(p_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    let p_csr: *mut DbdataCursor = p_cursor_1 as *mut DbdataCursor;
    let p_tab: *mut DbdataTable =
        unsafe { (*p_cursor_1).p_vtab } as *mut DbdataTable;
    let mut rc: i32 = 0;
    let mut z_schema: *const i8 = c"main".as_ptr() as *mut i8 as *const i8;
    { let _ = idx_str_1; };
    { let _ = argc; };
    dbdata_reset_cursor(unsafe { &mut *p_csr });
    if !(unsafe { (*p_csr).i_pgno } == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"dbdataFilter".as_ptr() as *const i8,
                c"dbdata.c".as_ptr() as *mut i8 as *const i8, 851,
                c"pCsr->iPgno==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if idx_num_1 & 1 != 0 {
        z_schema =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        if z_schema == core::ptr::null() {
            z_schema = c"".as_ptr() as *mut i8 as *const i8;
        }
    }
    if idx_num_1 & 2 != 0 {
        unsafe {
            (*p_csr).i_pgno =
                unsafe {
                    sqlite3_value_int(unsafe {
                            *argv.offset((idx_num_1 & 1) as isize)
                        })
                }
        };
        unsafe { (*p_csr).b_one_page = 1 };
    } else { rc = dbdata_dbsize(unsafe { &mut *p_csr }, z_schema); }
    if rc == 0 {
        let mut n_func: i32 = 0;
        if !(unsafe { (*p_tab).p_stmt }).is_null() {
            unsafe { (*p_csr).p_stmt = unsafe { (*p_tab).p_stmt } };
            unsafe { (*p_tab).p_stmt = core::ptr::null_mut() };
        } else if { n_func = dbdata_is_function(z_schema); n_func } > 0 {
            let z_sql: *mut i8 =
                unsafe {
                    sqlite3_mprintf(c"SELECT %.*s(?2)".as_ptr() as *mut i8 as
                            *const i8, n_func, z_schema)
                };
            if z_sql == core::ptr::null_mut() {
                rc = 7;
            } else {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(unsafe { (*p_tab).db },
                            z_sql as *const i8, -1, unsafe { &mut (*p_csr).p_stmt },
                            core::ptr::null_mut())
                    };
                unsafe { sqlite3_free(z_sql as *mut ()) };
            }
        } else {
            rc =
                unsafe {
                    sqlite3_prepare_v2(unsafe { (*p_tab).db },
                        c"SELECT data FROM sqlite_dbpage(?) WHERE pgno=?".as_ptr()
                                as *mut i8 as *const i8, -1,
                        unsafe { &mut (*p_csr).p_stmt }, core::ptr::null_mut())
                };
        }
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_bind_text(unsafe { (*p_csr).p_stmt }, 1, z_schema, -1,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
    }
    if rc == 0 { rc = dbdata_get_encoding(p_csr); }
    if rc != 0 {
        unsafe {
            (*p_tab).base.z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_errmsg(unsafe { (*p_tab).db }) })
                }
        };
    }
    if rc == 0 { rc = dbdata_next(p_cursor_1); }
    return rc;
}
extern "C" fn dbdata_column(p_cursor_1: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    let p_csr: *const DbdataCursor =
        p_cursor_1 as *mut DbdataCursor as *const DbdataCursor;
    let p_tab: *const DbdataTable =
        unsafe { (*p_cursor_1).p_vtab } as *mut DbdataTable as
            *const DbdataTable;
    if unsafe { (*p_tab).b_ptr } != 0 {
        '__s10:
            {
            match i {
                0 => {
                    unsafe {
                        sqlite3_result_int64(ctx,
                            unsafe { (*p_csr).i_pgno } as sqlite3_int64)
                    };
                }
                1 => {
                    {
                        let mut i_off: i32 =
                            if unsafe { (*p_csr).i_pgno } == 1 { 100 } else { 0 };
                        if unsafe { (*p_csr).i_cell } < 0 {
                            i_off += 8;
                        } else {
                            i_off += 12 + unsafe { (*p_csr).i_cell } * 2;
                            if i_off > unsafe { (*p_csr).n_page } { return 0; }
                            i_off =
                                get_uint16(unsafe {
                                                &raw mut *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                                            } as *const u8) as i32;
                        }
                        if i_off <= unsafe { (*p_csr).n_page } {
                            unsafe {
                                sqlite3_result_int64(ctx,
                                    get_uint32(unsafe {
                                                    &raw mut *unsafe { (*p_csr).a_page.offset(i_off as isize) }
                                                } as *const u8) as sqlite3_int64)
                            };
                        }
                        break '__s10;
                    }
                }
                _ => {}
            }
        }
    } else {
        '__s11:
            {
            match i {
                0 => {
                    unsafe {
                        sqlite3_result_int64(ctx,
                            unsafe { (*p_csr).i_pgno } as sqlite3_int64)
                    };
                }
                1 => {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).i_cell })
                    };
                }
                2 => {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_csr).i_field })
                    };
                }
                3 => {
                    {
                        if unsafe { (*p_csr).i_field } < 0 {
                            unsafe {
                                sqlite3_result_int64(ctx, unsafe { (*p_csr).i_intkey })
                            };
                        } else if unsafe {
                                    unsafe {
                                        (*p_csr).rec.a_buf.offset(unsafe { (*p_csr).n_rec } as
                                                isize)
                                    }
                                } >= unsafe { (*p_csr).p_ptr } {
                            let mut i_type: sqlite3_int64 = 0 as sqlite3_int64;
                            dbdata_get_varint_u32(unsafe { (*p_csr).p_hdr_ptr } as
                                    *const u8, &mut i_type);
                            dbdata_value(ctx, unsafe { (*p_csr).enc }, i_type as i32,
                                unsafe { (*p_csr).p_ptr },
                                unsafe {
                                        unsafe {
                                            unsafe {
                                                (*p_csr).rec.a_buf.offset(unsafe { (*p_csr).n_rec } as
                                                            isize).offset_from(unsafe { (*p_csr).p_ptr })
                                            }
                                        }
                                    } as i64);
                        }
                        break '__s11;
                    }
                }
                _ => {}
            }
        }
    }
    return 0;
}
extern "C" fn dbdata_rowid(p_cursor_1: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p_csr: *const DbdataCursor =
        p_cursor_1 as *mut DbdataCursor as *const DbdataCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_csr).i_rowid } };
    return 0;
}
extern "C" fn sqlite3_dbdata_register(db: *mut sqlite3) -> i32 {
    unsafe {
        let mut rc: i32 =
            unsafe {
                sqlite3_create_module(db,
                    c"sqlite_dbdata".as_ptr() as *mut i8 as *const i8,
                    &raw mut dbdata_module as *const sqlite3_module,
                    core::ptr::null_mut())
            };
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_create_module(db,
                        c"sqlite_dbptr".as_ptr() as *mut i8 as *const i8,
                        &raw mut dbdata_module as *const sqlite3_module,
                        1 as *mut ())
                };
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_dbdata_init(db: *mut sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const sqlite3_api_routines)
    -> i32 {
    { let _ = pz_err_msg_1; };
    return sqlite3_dbdata_register(db);
}
static mut dbdata_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: None,
        x_connect: Some(dbdata_connect),
        x_best_index: Some(dbdata_best_index),
        x_disconnect: Some(dbdata_disconnect),
        x_destroy: None,
        x_open: Some(dbdata_open),
        x_close: Some(dbdata_close),
        x_filter: Some(dbdata_filter),
        x_next: Some(dbdata_next),
        x_eof: Some(dbdata_eof),
        x_column: Some(dbdata_column),
        x_rowid: Some(dbdata_rowid),
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
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}