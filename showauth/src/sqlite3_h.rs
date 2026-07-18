use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3 {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type SqliteInt64 = i64;

pub(crate) type SqliteUint64 = u64;

pub(crate) type Sqlite3Int64 = SqliteInt64;

pub(crate) type Sqlite3Uint64 = SqliteUint64;

pub(crate) type Sqlite3Callback =
    unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3File {
    pub(crate) p_methods: *const Sqlite3IoMethods,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3IoMethods {
    pub(crate) i_version: i32,
    pub(crate) x_close: Option<unsafe extern "C" fn(*mut Sqlite3File) -> i32>,
    pub(crate) x_read: Option<unsafe extern "C" fn(*mut Sqlite3File, *mut (),
        i32, i64) -> i32>,
    pub(crate) x_write: Option<unsafe extern "C" fn(*mut Sqlite3File,
        *const (), i32, i64) -> i32>,
    pub(crate) x_truncate: Option<unsafe extern "C" fn(*mut Sqlite3File, i64)
        -> i32>,
    pub(crate) x_sync: Option<unsafe extern "C" fn(*mut Sqlite3File, i32)
        -> i32>,
    pub(crate) x_file_size: Option<unsafe extern "C" fn(*mut Sqlite3File,
        *mut i64) -> i32>,
    pub(crate) x_lock: Option<unsafe extern "C" fn(*mut Sqlite3File, i32)
        -> i32>,
    pub(crate) x_unlock: Option<unsafe extern "C" fn(*mut Sqlite3File, i32)
        -> i32>,
    pub(crate) x_check_reserved_lock: Option<unsafe extern "C" fn(*mut Sqlite3File,
        *mut i32) -> i32>,
    pub(crate) x_file_control: Option<unsafe extern "C" fn(*mut Sqlite3File,
        i32, *mut ()) -> i32>,
    pub(crate) x_sector_size: Option<unsafe extern "C" fn(*mut Sqlite3File)
        -> i32>,
    pub(crate) x_device_characteristics: Option<unsafe extern "C" fn(*mut Sqlite3File)
        -> i32>,
    pub(crate) x_shm_map: Option<unsafe extern "C" fn(*mut Sqlite3File, i32,
        i32, i32, *mut *mut ()) -> i32>,
    pub(crate) x_shm_lock: Option<unsafe extern "C" fn(*mut Sqlite3File, i32,
        i32, i32) -> i32>,
    pub(crate) x_shm_barrier: Option<unsafe extern "C" fn(*mut Sqlite3File)
        -> ()>,
    pub(crate) x_shm_unmap: Option<unsafe extern "C" fn(*mut Sqlite3File, i32)
        -> i32>,
    pub(crate) x_fetch: Option<unsafe extern "C" fn(*mut Sqlite3File, i64,
        i32, *mut *mut ()) -> i32>,
    pub(crate) x_unfetch: Option<unsafe extern "C" fn(*mut Sqlite3File, i64,
        *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Mutex {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Context {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Stmt {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Value {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Module {
    pub(crate) i_version: i32,
    pub(crate) x_create: Option<unsafe extern "C" fn(*mut Sqlite3, *mut (),
        i32, *const *const i8, *mut *mut Sqlite3Vtab, *mut *mut i8) -> i32>,
    pub(crate) x_connect: Option<unsafe extern "C" fn(*mut Sqlite3, *mut (),
        i32, *const *const i8, *mut *mut Sqlite3Vtab, *mut *mut i8) -> i32>,
    pub(crate) x_best_index: Option<unsafe extern "C" fn(*mut Sqlite3Vtab,
        *mut Sqlite3IndexInfo) -> i32>,
    pub(crate) x_disconnect: Option<unsafe extern "C" fn(*mut Sqlite3Vtab)
        -> i32>,
    pub(crate) x_destroy: Option<unsafe extern "C" fn(*mut Sqlite3Vtab)
        -> i32>,
    pub(crate) x_open: Option<unsafe extern "C" fn(*mut Sqlite3Vtab,
        *mut *mut Sqlite3VtabCursor) -> i32>,
    pub(crate) x_close: Option<unsafe extern "C" fn(*mut Sqlite3VtabCursor)
        -> i32>,
    pub(crate) x_filter: Option<unsafe extern "C" fn(*mut Sqlite3VtabCursor,
        i32, *const i8, i32, *mut *mut Sqlite3Value) -> i32>,
    pub(crate) x_next: Option<unsafe extern "C" fn(*mut Sqlite3VtabCursor)
        -> i32>,
    pub(crate) x_eof: Option<unsafe extern "C" fn(*mut Sqlite3VtabCursor)
        -> i32>,
    pub(crate) x_column: Option<unsafe extern "C" fn(*mut Sqlite3VtabCursor,
        *mut Sqlite3Context, i32) -> i32>,
    pub(crate) x_rowid: Option<unsafe extern "C" fn(*mut Sqlite3VtabCursor,
        *mut i64) -> i32>,
    pub(crate) x_update: Option<unsafe extern "C" fn(*mut Sqlite3Vtab, i32,
        *mut *mut Sqlite3Value, *mut i64) -> i32>,
    pub(crate) x_begin: Option<unsafe extern "C" fn(*mut Sqlite3Vtab) -> i32>,
    pub(crate) x_sync: Option<unsafe extern "C" fn(*mut Sqlite3Vtab) -> i32>,
    pub(crate) x_commit: Option<unsafe extern "C" fn(*mut Sqlite3Vtab)
        -> i32>,
    pub(crate) x_rollback: Option<unsafe extern "C" fn(*mut Sqlite3Vtab)
        -> i32>,
    pub(crate) x_find_function: Option<unsafe extern "C" fn(*mut Sqlite3Vtab,
        i32, *const i8,
        *mut unsafe extern "C" fn(*mut Sqlite3Context, i32,
                *mut *mut Sqlite3Value) -> (), *mut *mut ()) -> i32>,
    pub(crate) x_rename: Option<unsafe extern "C" fn(*mut Sqlite3Vtab,
        *const i8) -> i32>,
    pub(crate) x_savepoint: Option<unsafe extern "C" fn(*mut Sqlite3Vtab, i32)
        -> i32>,
    pub(crate) x_release: Option<unsafe extern "C" fn(*mut Sqlite3Vtab, i32)
        -> i32>,
    pub(crate) x_rollback_to: Option<unsafe extern "C" fn(*mut Sqlite3Vtab,
        i32) -> i32>,
    pub(crate) x_shadow_name: Option<unsafe extern "C" fn(*const i8) -> i32>,
    pub(crate) x_integrity: Option<unsafe extern "C" fn(*mut Sqlite3Vtab,
        *const i8, *const i8, i32, *mut *mut i8) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Vtab {
    pub(crate) p_module: *const Sqlite3Module,
    pub(crate) n_ref: i32,
    pub(crate) z_err_msg: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3IndexInfo {
    pub(crate) n_constraint: i32,
    pub(crate) a_constraint: *mut Sqlite3IndexConstraint,
    pub(crate) n_order_by: i32,
    pub(crate) a_order_by: *mut Sqlite3IndexOrderby,
    pub(crate) a_constraint_usage: *mut Sqlite3IndexConstraintUsage,
    pub(crate) idx_num: i32,
    pub(crate) idx_str: *mut i8,
    pub(crate) need_to_free_idx_str: i32,
    pub(crate) order_by_consumed: i32,
    pub(crate) estimated_cost: f64,
    pub(crate) estimated_rows: Sqlite3Int64,
    pub(crate) idx_flags: i32,
    pub(crate) col_used: Sqlite3Uint64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3IndexConstraint {
    pub(crate) i_column: i32,
    pub(crate) op: u8,
    pub(crate) usable: u8,
    pub(crate) i_term_offset: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3IndexOrderby {
    pub(crate) i_column: i32,
    pub(crate) desc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3IndexConstraintUsage {
    pub(crate) argv_index: i32,
    pub(crate) omit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3VtabCursor {
    pub(crate) p_vtab: *mut Sqlite3Vtab,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Blob {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Vfs {
    pub(crate) i_version: i32,
    pub(crate) sz_os_file: i32,
    pub(crate) mx_pathname: i32,
    pub(crate) p_next: *mut Sqlite3Vfs,
    pub(crate) z_name: *const i8,
    pub(crate) p_app_data: *mut (),
    pub(crate) x_open: Option<unsafe extern "C" fn(*mut Sqlite3Vfs, *const i8,
        *mut Sqlite3File, i32, *mut i32) -> i32>,
    pub(crate) x_delete: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8, i32) -> i32>,
    pub(crate) x_access: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8, i32, *mut i32) -> i32>,
    pub(crate) x_full_pathname: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8, i32, *mut i8) -> i32>,
    pub(crate) x_dl_open: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8) -> *mut ()>,
    pub(crate) x_dl_error: Option<unsafe extern "C" fn(*mut Sqlite3Vfs, i32,
        *mut i8) -> ()>,
    pub(crate) x_dl_sym: Option<unsafe extern "C" fn(*mut Sqlite3Vfs, *mut (),
        *const i8) -> unsafe extern "C" fn() -> ()>,
    pub(crate) x_dl_close: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *mut ()) -> ()>,
    pub(crate) x_randomness: Option<unsafe extern "C" fn(*mut Sqlite3Vfs, i32,
        *mut i8) -> i32>,
    pub(crate) x_sleep: Option<unsafe extern "C" fn(*mut Sqlite3Vfs, i32)
        -> i32>,
    pub(crate) x_current_time: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *mut f64) -> i32>,
    pub(crate) x_get_last_error: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        i32, *mut i8) -> i32>,
    pub(crate) x_current_time_int64: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *mut i64) -> i32>,
    pub(crate) x_set_system_call: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8, unsafe extern "C" fn() -> ()) -> i32>,
    pub(crate) x_get_system_call: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8) -> unsafe extern "C" fn() -> ()>,
    pub(crate) x_next_system_call: Option<unsafe extern "C" fn(*mut Sqlite3Vfs,
        *const i8) -> *const i8>,
}

pub(crate) type Sqlite3Filename = *const i8;

pub(crate) type Sqlite3SyscallPtr = unsafe extern "C" fn() -> ();

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Backup {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Str {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3MemMethods {
    pub(crate) x_malloc: Option<unsafe extern "C" fn(i32) -> *mut ()>,
    pub(crate) x_free: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) x_realloc: Option<unsafe extern "C" fn(*mut (), i32)
        -> *mut ()>,
    pub(crate) x_size: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    pub(crate) x_roundup: Option<unsafe extern "C" fn(i32) -> i32>,
    pub(crate) x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    pub(crate) x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) p_app_data: *mut (),
}

pub(crate) type Sqlite3DestructorType = unsafe extern "C" fn(*mut ()) -> ();

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3MutexMethods {
    pub(crate) x_mutex_init: Option<unsafe extern "C" fn() -> i32>,
    pub(crate) x_mutex_end: Option<unsafe extern "C" fn() -> i32>,
    pub(crate) x_mutex_alloc: Option<unsafe extern "C" fn(i32)
        -> *mut Sqlite3Mutex>,
    pub(crate) x_mutex_free: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> ()>,
    pub(crate) x_mutex_enter: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> ()>,
    pub(crate) x_mutex_try: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> i32>,
    pub(crate) x_mutex_leave: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> ()>,
    pub(crate) x_mutex_held: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> i32>,
    pub(crate) x_mutex_notheld: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Pcache {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3PcachePage {
    pub(crate) p_buf: *mut (),
    pub(crate) p_extra: *mut (),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3PcacheMethods2 {
    pub(crate) i_version: i32,
    pub(crate) p_arg: *mut (),
    pub(crate) x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    pub(crate) x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) x_create: Option<unsafe extern "C" fn(i32, i32, i32)
        -> *mut Sqlite3Pcache>,
    pub(crate) x_cachesize: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        i32) -> ()>,
    pub(crate) x_pagecount: Option<unsafe extern "C" fn(*mut Sqlite3Pcache)
        -> i32>,
    pub(crate) x_fetch: Option<unsafe extern "C" fn(*mut Sqlite3Pcache, u32,
        i32) -> *mut Sqlite3PcachePage>,
    pub(crate) x_unpin: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        *mut Sqlite3PcachePage, i32) -> ()>,
    pub(crate) x_rekey: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        *mut Sqlite3PcachePage, u32, u32) -> ()>,
    pub(crate) x_truncate: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        u32) -> ()>,
    pub(crate) x_destroy: Option<unsafe extern "C" fn(*mut Sqlite3Pcache)
        -> ()>,
    pub(crate) x_shrink: Option<unsafe extern "C" fn(*mut Sqlite3Pcache)
        -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3PcacheMethods {
    pub(crate) p_arg: *mut (),
    pub(crate) x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    pub(crate) x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) x_create: Option<unsafe extern "C" fn(i32, i32)
        -> *mut Sqlite3Pcache>,
    pub(crate) x_cachesize: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        i32) -> ()>,
    pub(crate) x_pagecount: Option<unsafe extern "C" fn(*mut Sqlite3Pcache)
        -> i32>,
    pub(crate) x_fetch: Option<unsafe extern "C" fn(*mut Sqlite3Pcache, u32,
        i32) -> *mut ()>,
    pub(crate) x_unpin: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        *mut (), i32) -> ()>,
    pub(crate) x_rekey: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        *mut (), u32, u32) -> ()>,
    pub(crate) x_truncate: Option<unsafe extern "C" fn(*mut Sqlite3Pcache,
        u32) -> ()>,
    pub(crate) x_destroy: Option<unsafe extern "C" fn(*mut Sqlite3Pcache)
        -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Snapshot {
    pub(crate) hidden: [u8; 48],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3RtreeGeometry {
    pub(crate) p_context: *mut (),
    pub(crate) n_param: i32,
    pub(crate) a_param: *mut Sqlite3RtreeDbl,
    pub(crate) p_user: *mut (),
    pub(crate) x_del_user: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}

pub(crate) type Sqlite3RtreeDbl = f64;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3RtreeQueryInfo {
    pub(crate) p_context: *mut (),
    pub(crate) n_param: i32,
    pub(crate) a_param: *mut Sqlite3RtreeDbl,
    pub(crate) p_user: *mut (),
    pub(crate) x_del_user: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) a_coord: *mut Sqlite3RtreeDbl,
    pub(crate) an_queue: *mut u32,
    pub(crate) n_coord: i32,
    pub(crate) i_level: i32,
    pub(crate) mx_level: i32,
    pub(crate) i_rowid: Sqlite3Int64,
    pub(crate) r_parent_score: Sqlite3RtreeDbl,
    pub(crate) e_parent_within: i32,
    pub(crate) e_within: i32,
    pub(crate) r_score: Sqlite3RtreeDbl,
    pub(crate) ap_sql_param: *mut *mut Sqlite3Value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5ExtensionApi {
    pub(crate) i_version: i32,
    pub(crate) x_user_data: Option<unsafe extern "C" fn(*mut Fts5Context)
        -> *mut ()>,
    pub(crate) x_column_count: Option<unsafe extern "C" fn(*mut Fts5Context)
        -> i32>,
    pub(crate) x_row_count: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut i64) -> i32>,
    pub(crate) x_column_total_size: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut i64) -> i32>,
    pub(crate) x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Context,
        *const i8, i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
    pub(crate) x_phrase_count: Option<unsafe extern "C" fn(*mut Fts5Context)
        -> i32>,
    pub(crate) x_phrase_size: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32) -> i32>,
    pub(crate) x_inst_count: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut i32) -> i32>,
    pub(crate) x_inst: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut i32, *mut i32, *mut i32) -> i32>,
    pub(crate) x_rowid: Option<unsafe extern "C" fn(*mut Fts5Context) -> i64>,
    pub(crate) x_column_text: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut *const i8, *mut i32) -> i32>,
    pub(crate) x_column_size: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut i32) -> i32>,
    pub(crate) x_query_phrase: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut (),
        unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
                *mut ()) -> i32) -> i32>,
    pub(crate) x_set_auxdata: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) x_get_auxdata: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32) -> *mut ()>,
    pub(crate) x_phrase_first: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut Fts5PhraseIter, *mut i32, *mut i32) -> i32>,
    pub(crate) x_phrase_next: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut Fts5PhraseIter, *mut i32, *mut i32) -> ()>,
    pub(crate) x_phrase_first_column: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut Fts5PhraseIter, *mut i32) -> i32>,
    pub(crate) x_phrase_next_column: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut Fts5PhraseIter, *mut i32) -> ()>,
    pub(crate) x_query_token: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, i32, *mut *const i8, *mut i32) -> i32>,
    pub(crate) x_inst_token: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, i32, *mut *const i8, *mut i32) -> i32>,
    pub(crate) x_column_locale: Option<unsafe extern "C" fn(*mut Fts5Context,
        i32, *mut *const i8, *mut i32) -> i32>,
    pub(crate) x_tokenize_v2: Option<unsafe extern "C" fn(*mut Fts5Context,
        *const i8, i32, *const i8, i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Context {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5PhraseIter {
    pub(crate) a: *const u8,
    pub(crate) b: *const u8,
}

pub(crate) type Fts5ExtensionFunction =
    unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
        *mut Sqlite3Context, i32, *mut *mut Sqlite3Value) -> ();

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Tokenizer {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5TokenizerV2 {
    pub(crate) i_version: i32,
    pub(crate) x_create: Option<unsafe extern "C" fn(*mut (), *mut *const i8,
        i32, *mut *mut Fts5Tokenizer) -> i32>,
    pub(crate) x_delete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer)
        -> ()>,
    pub(crate) x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Tokenizer,
        *mut (), i32, *const i8, i32, *const i8, i32,
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct fts5_tokenizer {
    pub(crate) x_create: Option<unsafe extern "C" fn(*mut (), *mut *const i8,
        i32, *mut *mut Fts5Tokenizer) -> i32>,
    pub(crate) x_delete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer)
        -> ()>,
    pub(crate) x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Tokenizer,
        *mut (), i32, *const i8, i32,
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Api {
    pub(crate) i_version: i32,
    pub(crate) x_create_tokenizer: Option<unsafe extern "C" fn(*mut Fts5Api,
        *const i8, *mut (), *mut fts5_tokenizer,
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) x_find_tokenizer: Option<unsafe extern "C" fn(*mut Fts5Api,
        *const i8, *mut *mut (), *mut fts5_tokenizer) -> i32>,
    pub(crate) x_create_function: Option<unsafe extern "C" fn(*mut Fts5Api,
        *const i8, *mut (),
        unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
                *mut Sqlite3Context, i32, *mut *mut Sqlite3Value) -> (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) x_create_tokenizer_v2: Option<unsafe extern "C" fn(*mut Fts5Api,
        *const i8, *mut (), *mut Fts5TokenizerV2,
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) x_find_tokenizer_v2: Option<unsafe extern "C" fn(*mut Fts5Api,
        *const i8, *mut *mut (), *mut *mut Fts5TokenizerV2) -> i32>,
}
