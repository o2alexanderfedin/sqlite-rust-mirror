use super::*;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3IndexInfo, Sqlite3Module, Sqlite3Mutex, Sqlite3Stmt, Sqlite3Str,
    Sqlite3Value, Sqlite3Vfs,
};

///* The following structure holds pointers to all of the SQLite API
///* routines.
///*
///* WARNING:  In order to maintain backwards compatibility, add new
///* interfaces to the end of this structure only.  If you insert new
///* interfaces in the middle of this structure, then older different
///* versions of SQLite will not be able to load each other's shared
///* libraries!
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3ApiRoutines {
    pub(crate) aggregate_context: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32) -> *mut ()>,
    pub(crate) aggregate_count: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> i32>,
    pub(crate) bind_blob: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) bind_double: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        f64) -> i32>,
    pub(crate) bind_int: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        i32) -> i32>,
    pub(crate) bind_int64: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        i64) -> i32>,
    pub(crate) bind_null: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32)
        -> i32>,
    pub(crate) bind_parameter_count: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) bind_parameter_index: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        *const i8) -> i32>,
    pub(crate) bind_parameter_name: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const i8>,
    pub(crate) bind_text: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *const i8, i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) bind_text16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) bind_value: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *const Sqlite3Value) -> i32>,
    pub(crate) busy_handler: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), i32) -> i32, *mut ()) -> i32>,
    pub(crate) busy_timeout: Option<unsafe extern "C" fn(*mut Sqlite3, i32)
        -> i32>,
    pub(crate) changes: Option<unsafe extern "C" fn(*mut Sqlite3) -> i32>,
    pub(crate) close: Option<unsafe extern "C" fn(*mut Sqlite3) -> i32>,
    pub(crate) collation_needed: Option<unsafe extern "C" fn(*mut Sqlite3,
        *mut (),
        unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const i8) -> ())
        -> i32>,
    pub(crate) collation_needed16: Option<unsafe extern "C" fn(*mut Sqlite3,
        *mut (),
        unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const ()) -> ())
        -> i32>,
    pub(crate) column_blob: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32)
        -> *const ()>,
    pub(crate) column_bytes: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> i32>,
    pub(crate) column_bytes16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> i32>,
    pub(crate) column_count: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) column_database_name: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const i8>,
    pub(crate) column_database_name16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const ()>,
    pub(crate) column_decltype: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const i8>,
    pub(crate) column_decltype16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const ()>,
    pub(crate) column_double: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> f64>,
    pub(crate) column_int: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32)
        -> i32>,
    pub(crate) column_int64: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> i64>,
    pub(crate) column_name: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32)
        -> *const i8>,
    pub(crate) column_name16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const ()>,
    pub(crate) column_origin_name: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const i8>,
    pub(crate) column_origin_name16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const ()>,
    pub(crate) column_table_name: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const i8>,
    pub(crate) column_table_name16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const ()>,
    pub(crate) column_text: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32)
        -> *const u8>,
    pub(crate) column_text16: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *const ()>,
    pub(crate) column_type: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32)
        -> i32>,
    pub(crate) column_value: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> *mut Sqlite3Value>,
    pub(crate) commit_hook: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut ()) -> i32, *mut ()) -> *mut ()>,
    pub(crate) complete: Option<unsafe extern "C" fn(*const i8) -> i32>,
    pub(crate) complete16: Option<unsafe extern "C" fn(*const ()) -> i32>,
    pub(crate) create_collation: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ()) -> i32)
        -> i32>,
    pub(crate) create_collation16: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const (), i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ()) -> i32)
        -> i32>,
    pub(crate) create_function: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, i32, *mut (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (), unsafe extern "C" fn(*mut Sqlite3Context) -> ())
        -> i32>,
    pub(crate) create_function16: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const (), i32, i32, *mut (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (), unsafe extern "C" fn(*mut Sqlite3Context) -> ())
        -> i32>,
    pub(crate) create_module: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *const Sqlite3Module, *mut ()) -> i32>,
    pub(crate) data_count: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) db_handle: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> *mut Sqlite3>,
    pub(crate) declare_vtab: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8) -> i32>,
    pub(crate) enable_shared_cache: Option<unsafe extern "C" fn(i32) -> i32>,
    pub(crate) errcode: Option<unsafe extern "C" fn(*mut Sqlite3) -> i32>,
    pub(crate) errmsg: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> *const i8>,
    pub(crate) errmsg16: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> *const ()>,
    pub(crate) exec: Option<unsafe extern "C" fn(*mut Sqlite3, *const i8,
        unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        *mut (), *mut *mut i8) -> i32>,
    pub(crate) expired: Option<unsafe extern "C" fn(*mut Sqlite3Stmt) -> i32>,
    pub(crate) finalize: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) free: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) free_table: Option<unsafe extern "C" fn(*mut *mut i8) -> ()>,
    pub(crate) get_autocommit: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) get_auxdata: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32) -> *mut ()>,
    pub(crate) get_table: Option<unsafe extern "C" fn(*mut Sqlite3, *const i8,
        *mut *mut *mut i8, *mut i32, *mut i32, *mut *mut i8) -> i32>,
    pub(crate) global_recover: Option<unsafe extern "C" fn() -> i32>,
    pub(crate) interruptx: Option<unsafe extern "C" fn(*mut Sqlite3) -> ()>,
    pub(crate) last_insert_rowid: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i64>,
    pub(crate) libversion: Option<unsafe extern "C" fn() -> *const i8>,
    pub(crate) libversion_number: Option<unsafe extern "C" fn() -> i32>,
    pub(crate) malloc: Option<unsafe extern "C" fn(i32) -> *mut ()>,
    pub(crate) mprintf: Option<unsafe extern "C" fn(*const i8, ...)
        -> *mut i8>,
    pub(crate) open: Option<unsafe extern "C" fn(*const i8, *mut *mut Sqlite3)
        -> i32>,
    pub(crate) open16: Option<unsafe extern "C" fn(*const (),
        *mut *mut Sqlite3) -> i32>,
    pub(crate) prepare: Option<unsafe extern "C" fn(*mut Sqlite3, *const i8,
        i32, *mut *mut Sqlite3Stmt, *mut *const i8) -> i32>,
    pub(crate) prepare16: Option<unsafe extern "C" fn(*mut Sqlite3, *const (),
        i32, *mut *mut Sqlite3Stmt, *mut *const ()) -> i32>,
    pub(crate) profile: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), *const i8, u64) -> (), *mut ())
        -> *mut ()>,
    pub(crate) progress_handler: Option<unsafe extern "C" fn(*mut Sqlite3,
        i32, unsafe extern "C" fn(*mut ()) -> i32, *mut ()) -> ()>,
    pub(crate) realloc: Option<unsafe extern "C" fn(*mut (), i32) -> *mut ()>,
    pub(crate) reset: Option<unsafe extern "C" fn(*mut Sqlite3Stmt) -> i32>,
    pub(crate) result_blob: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) result_double: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        f64) -> ()>,
    pub(crate) result_error: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const i8, i32) -> ()>,
    pub(crate) result_error16: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const (), i32) -> ()>,
    pub(crate) result_int: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32) -> ()>,
    pub(crate) result_int64: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i64) -> ()>,
    pub(crate) result_null: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> ()>,
    pub(crate) result_text: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const i8, i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) result_text16: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) result_text16be: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) result_text16le: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const (), i32, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) result_value: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *mut Sqlite3Value) -> ()>,
    pub(crate) rollback_hook: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut ()) -> (), *mut ()) -> *mut ()>,
    pub(crate) set_authorizer: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, *const i8,
                *const i8) -> i32, *mut ()) -> i32>,
    pub(crate) set_auxdata: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32, *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) xsnprintf: Option<unsafe extern "C" fn(i32, *mut i8, *const i8,
        ...) -> *mut i8>,
    pub(crate) step: Option<unsafe extern "C" fn(*mut Sqlite3Stmt) -> i32>,
    pub(crate) table_column_metadata: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *const i8, *const i8, *mut *const i8, *mut *const i8,
        *mut i32, *mut i32, *mut i32) -> i32>,
    pub(crate) thread_cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub(crate) total_changes: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) trace: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), *const i8) -> (), *mut ()) -> *mut ()>,
    pub(crate) transfer_bindings: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        *mut Sqlite3Stmt) -> i32>,
    pub(crate) update_hook: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64) -> (),
        *mut ()) -> *mut ()>,
    pub(crate) user_data: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> *mut ()>,
    pub(crate) value_blob: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> *const ()>,
    pub(crate) value_bytes: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) value_bytes16: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) value_double: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> f64>,
    pub(crate) value_int: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) value_int64: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i64>,
    pub(crate) value_numeric_type: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) value_text: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> *const u8>,
    pub(crate) value_text16: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> *const ()>,
    pub(crate) value_text16be: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> *const ()>,
    pub(crate) value_text16le: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> *const ()>,
    pub(crate) value_type: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) vmprintf: Option<unsafe extern "C" fn(*const i8, *mut i8)
        -> *mut i8>,
    pub(crate) overload_function: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32) -> i32>,
    pub(crate) prepare_v2: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, *mut *mut Sqlite3Stmt, *mut *const i8) -> i32>,
    pub(crate) prepare16_v2: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const (), i32, *mut *mut Sqlite3Stmt, *mut *const ()) -> i32>,
    pub(crate) clear_bindings: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) create_module_v2: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *const Sqlite3Module, *mut (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) bind_zeroblob: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32, i32) -> i32>,
    pub(crate) blob_bytes: Option<unsafe extern "C" fn(*mut Sqlite3Blob)
        -> i32>,
    pub(crate) blob_close: Option<unsafe extern "C" fn(*mut Sqlite3Blob)
        -> i32>,
    pub(crate) blob_open: Option<unsafe extern "C" fn(*mut Sqlite3, *const i8,
        *const i8, *const i8, i64, i32, *mut *mut Sqlite3Blob) -> i32>,
    pub(crate) blob_read: Option<unsafe extern "C" fn(*mut Sqlite3Blob,
        *mut (), i32, i32) -> i32>,
    pub(crate) blob_write: Option<unsafe extern "C" fn(*mut Sqlite3Blob,
        *const (), i32, i32) -> i32>,
    pub(crate) create_collation_v2: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) file_control: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, *mut ()) -> i32>,
    pub(crate) memory_highwater: Option<unsafe extern "C" fn(i32) -> i64>,
    pub(crate) memory_used: Option<unsafe extern "C" fn() -> i64>,
    pub(crate) mutex_alloc: Option<unsafe extern "C" fn(i32)
        -> *mut Sqlite3Mutex>,
    pub(crate) mutex_enter: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> ()>,
    pub(crate) mutex_free: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> ()>,
    pub(crate) mutex_leave: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> ()>,
    pub(crate) mutex_try: Option<unsafe extern "C" fn(*mut Sqlite3Mutex)
        -> i32>,
    pub(crate) open_v2: Option<unsafe extern "C" fn(*const i8,
        *mut *mut Sqlite3, i32, *const i8) -> i32>,
    pub(crate) release_memory: Option<unsafe extern "C" fn(i32) -> i32>,
    pub(crate) result_error_nomem: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> ()>,
    pub(crate) result_error_toobig: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> ()>,
    pub(crate) sleep: Option<unsafe extern "C" fn(i32) -> i32>,
    pub(crate) soft_heap_limit: Option<unsafe extern "C" fn(i32) -> ()>,
    pub(crate) vfs_find: Option<unsafe extern "C" fn(*const i8)
        -> *mut Sqlite3Vfs>,
    pub(crate) vfs_register: Option<unsafe extern "C" fn(*mut Sqlite3Vfs, i32)
        -> i32>,
    pub(crate) vfs_unregister: Option<unsafe extern "C" fn(*mut Sqlite3Vfs)
        -> i32>,
    pub(crate) xthreadsafe: Option<unsafe extern "C" fn() -> i32>,
    pub(crate) result_zeroblob: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32) -> ()>,
    pub(crate) result_error_code: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32) -> ()>,
    pub(crate) test_control: Option<unsafe extern "C" fn(i32, ...) -> i32>,
    pub(crate) randomness: Option<unsafe extern "C" fn(i32, *mut ()) -> ()>,
    pub(crate) context_db_handle: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> *mut Sqlite3>,
    pub(crate) extended_result_codes: Option<unsafe extern "C" fn(*mut Sqlite3,
        i32) -> i32>,
    pub(crate) limit: Option<unsafe extern "C" fn(*mut Sqlite3, i32, i32)
        -> i32>,
    pub(crate) next_stmt: Option<unsafe extern "C" fn(*mut Sqlite3,
        *mut Sqlite3Stmt) -> *mut Sqlite3Stmt>,
    pub(crate) sql: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> *const i8>,
    pub(crate) status: Option<unsafe extern "C" fn(i32, *mut i32, *mut i32,
        i32) -> i32>,
    pub(crate) backup_finish: Option<unsafe extern "C" fn(*mut Sqlite3Backup)
        -> i32>,
    pub(crate) backup_init: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *mut Sqlite3, *const i8) -> *mut Sqlite3Backup>,
    pub(crate) backup_pagecount: Option<unsafe extern "C" fn(*mut Sqlite3Backup)
        -> i32>,
    pub(crate) backup_remaining: Option<unsafe extern "C" fn(*mut Sqlite3Backup)
        -> i32>,
    pub(crate) backup_step: Option<unsafe extern "C" fn(*mut Sqlite3Backup,
        i32) -> i32>,
    pub(crate) compileoption_get: Option<unsafe extern "C" fn(i32)
        -> *const i8>,
    pub(crate) compileoption_used: Option<unsafe extern "C" fn(*const i8)
        -> i32>,
    pub(crate) create_function_v2: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, i32, *mut (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (), unsafe extern "C" fn(*mut Sqlite3Context) -> (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) db_config: Option<unsafe extern "C" fn(*mut Sqlite3, i32, ...)
        -> i32>,
    pub(crate) db_mutex: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> *mut Sqlite3Mutex>,
    pub(crate) db_status: Option<unsafe extern "C" fn(*mut Sqlite3, i32,
        *mut i32, *mut i32, i32) -> i32>,
    pub(crate) extended_errcode: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) log: Option<unsafe extern "C" fn(i32, *const i8, ...) -> ()>,
    pub(crate) soft_heap_limit64: Option<unsafe extern "C" fn(i64) -> i64>,
    pub(crate) sourceid: Option<unsafe extern "C" fn() -> *const i8>,
    pub(crate) stmt_status: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        i32) -> i32>,
    pub(crate) strnicmp: Option<unsafe extern "C" fn(*const i8, *const i8,
        i32) -> i32>,
    pub(crate) unlock_notify: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut *mut (), i32) -> (), *mut ()) -> i32>,
    pub(crate) wal_autocheckpoint: Option<unsafe extern "C" fn(*mut Sqlite3,
        i32) -> i32>,
    pub(crate) wal_checkpoint: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8) -> i32>,
    pub(crate) wal_hook: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32) -> i32,
        *mut ()) -> *mut ()>,
    pub(crate) blob_reopen: Option<unsafe extern "C" fn(*mut Sqlite3Blob, i64)
        -> i32>,
    pub(crate) vtab_config: Option<unsafe extern "C" fn(*mut Sqlite3, i32,
        ...) -> i32>,
    pub(crate) vtab_on_conflict: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) close_v2: Option<unsafe extern "C" fn(*mut Sqlite3) -> i32>,
    pub(crate) db_filename: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8) -> *const i8>,
    pub(crate) db_readonly: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8) -> i32>,
    pub(crate) db_release_memory: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) errstr: Option<unsafe extern "C" fn(i32) -> *const i8>,
    pub(crate) stmt_busy: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) stmt_readonly: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) stricmp: Option<unsafe extern "C" fn(*const i8, *const i8)
        -> i32>,
    pub(crate) uri_boolean: Option<unsafe extern "C" fn(*const i8, *const i8,
        i32) -> i32>,
    pub(crate) uri_int64: Option<unsafe extern "C" fn(*const i8, *const i8,
        i64) -> i64>,
    pub(crate) uri_parameter: Option<unsafe extern "C" fn(*const i8,
        *const i8) -> *const i8>,
    pub(crate) xvsnprintf: Option<unsafe extern "C" fn(i32, *mut i8,
        *const i8, *mut i8) -> *mut i8>,
    pub(crate) wal_checkpoint_v2: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, *mut i32, *mut i32) -> i32>,
    pub(crate) auto_extension: Option<unsafe extern "C" fn(unsafe extern "C" fn()
                -> ()) -> i32>,
    pub(crate) bind_blob64: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *const (), u64, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) bind_text64: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *const i8, u64, unsafe extern "C" fn(*mut ()) -> (), u8) -> i32>,
    pub(crate) cancel_auto_extension: Option<unsafe extern "C" fn(unsafe extern "C" fn()
                -> ()) -> i32>,
    pub(crate) load_extension: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *const i8, *mut *mut i8) -> i32>,
    pub(crate) malloc64: Option<unsafe extern "C" fn(u64) -> *mut ()>,
    pub(crate) msize: Option<unsafe extern "C" fn(*mut ()) -> u64>,
    pub(crate) realloc64: Option<unsafe extern "C" fn(*mut (), u64)
        -> *mut ()>,
    pub(crate) reset_auto_extension: Option<unsafe extern "C" fn() -> ()>,
    pub(crate) result_blob64: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const (), u64, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) result_text64: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *const i8, u64, unsafe extern "C" fn(*mut ()) -> (), u8) -> ()>,
    pub(crate) strglob: Option<unsafe extern "C" fn(*const i8, *const i8)
        -> i32>,
    pub(crate) value_dup: Option<unsafe extern "C" fn(*const Sqlite3Value)
        -> *mut Sqlite3Value>,
    pub(crate) value_free: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> ()>,
    pub(crate) result_zeroblob64: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        u64) -> i32>,
    pub(crate) bind_zeroblob64: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32, u64) -> i32>,
    pub(crate) value_subtype: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> u32>,
    pub(crate) result_subtype: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        u32) -> ()>,
    pub(crate) status64: Option<unsafe extern "C" fn(i32, *mut i64, *mut i64,
        i32) -> i32>,
    pub(crate) strlike: Option<unsafe extern "C" fn(*const i8, *const i8, u32)
        -> i32>,
    pub(crate) db_cacheflush: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) system_errno: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) trace_v2: Option<unsafe extern "C" fn(*mut Sqlite3, u32,
        unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32, *mut ())
        -> i32>,
    pub(crate) expanded_sql: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> *mut i8>,
    pub(crate) set_last_insert_rowid: Option<unsafe extern "C" fn(*mut Sqlite3,
        i64) -> ()>,
    pub(crate) prepare_v3: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, u32, *mut *mut Sqlite3Stmt, *mut *const i8) -> i32>,
    pub(crate) prepare16_v3: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const (), i32, u32, *mut *mut Sqlite3Stmt, *mut *const ()) -> i32>,
    pub(crate) bind_pointer: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32, *mut (), *const i8, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) result_pointer: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *mut (), *const i8, unsafe extern "C" fn(*mut ()) -> ()) -> ()>,
    pub(crate) value_pointer: Option<unsafe extern "C" fn(*mut Sqlite3Value,
        *const i8) -> *mut ()>,
    pub(crate) vtab_nochange: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> i32>,
    pub(crate) value_nochange: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) vtab_collation: Option<unsafe extern "C" fn(*mut Sqlite3IndexInfo,
        i32) -> *const i8>,
    pub(crate) keyword_count: Option<unsafe extern "C" fn() -> i32>,
    pub(crate) keyword_name: Option<unsafe extern "C" fn(i32, *mut *const i8,
        *mut i32) -> i32>,
    pub(crate) keyword_check: Option<unsafe extern "C" fn(*const i8, i32)
        -> i32>,
    pub(crate) str_new: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> *mut Sqlite3Str>,
    pub(crate) str_finish: Option<unsafe extern "C" fn(*mut Sqlite3Str)
        -> *mut i8>,
    pub(crate) str_appendf: Option<unsafe extern "C" fn(*mut Sqlite3Str,
        *const i8, ...) -> ()>,
    pub(crate) str_vappendf: Option<unsafe extern "C" fn(*mut Sqlite3Str,
        *const i8, *mut i8) -> ()>,
    pub(crate) str_append: Option<unsafe extern "C" fn(*mut Sqlite3Str,
        *const i8, i32) -> ()>,
    pub(crate) str_appendall: Option<unsafe extern "C" fn(*mut Sqlite3Str,
        *const i8) -> ()>,
    pub(crate) str_appendchar: Option<unsafe extern "C" fn(*mut Sqlite3Str,
        i32, i8) -> ()>,
    pub(crate) str_reset: Option<unsafe extern "C" fn(*mut Sqlite3Str) -> ()>,
    pub(crate) str_errcode: Option<unsafe extern "C" fn(*mut Sqlite3Str)
        -> i32>,
    pub(crate) str_length: Option<unsafe extern "C" fn(*mut Sqlite3Str)
        -> i32>,
    pub(crate) str_value: Option<unsafe extern "C" fn(*mut Sqlite3Str)
        -> *mut i8>,
    pub(crate) create_window_function: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, i32, i32, *mut (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (), unsafe extern "C" fn(*mut Sqlite3Context) -> (),
        unsafe extern "C" fn(*mut Sqlite3Context) -> (),
        unsafe extern "C" fn(*mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
                -> (), unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) normalized_sql: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> *const i8>,
    pub(crate) stmt_isexplain: Option<unsafe extern "C" fn(*mut Sqlite3Stmt)
        -> i32>,
    pub(crate) value_frombind: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) drop_modules: Option<unsafe extern "C" fn(*mut Sqlite3,
        *mut *const i8) -> i32>,
    pub(crate) hard_heap_limit64: Option<unsafe extern "C" fn(i64) -> i64>,
    pub(crate) uri_key: Option<unsafe extern "C" fn(*const i8, i32)
        -> *const i8>,
    pub(crate) filename_database: Option<unsafe extern "C" fn(*const i8)
        -> *const i8>,
    pub(crate) filename_journal: Option<unsafe extern "C" fn(*const i8)
        -> *const i8>,
    pub(crate) filename_wal: Option<unsafe extern "C" fn(*const i8)
        -> *const i8>,
    pub(crate) create_filename: Option<unsafe extern "C" fn(*const i8,
        *const i8, *const i8, i32, *mut *const i8) -> *const i8>,
    pub(crate) free_filename: Option<unsafe extern "C" fn(*const i8) -> ()>,
    pub(crate) database_file_object: Option<unsafe extern "C" fn(*const i8)
        -> *mut Sqlite3File>,
    pub(crate) txn_state: Option<unsafe extern "C" fn(*mut Sqlite3, *const i8)
        -> i32>,
    pub(crate) changes64: Option<unsafe extern "C" fn(*mut Sqlite3) -> i64>,
    pub(crate) total_changes64: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i64>,
    pub(crate) autovacuum_pages: Option<unsafe extern "C" fn(*mut Sqlite3,
        unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32,
        *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) error_offset: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) vtab_rhs_value: Option<unsafe extern "C" fn(*mut Sqlite3IndexInfo,
        i32, *mut *mut Sqlite3Value) -> i32>,
    pub(crate) vtab_distinct: Option<unsafe extern "C" fn(*mut Sqlite3IndexInfo)
        -> i32>,
    pub(crate) vtab_in: Option<unsafe extern "C" fn(*mut Sqlite3IndexInfo,
        i32, i32) -> i32>,
    pub(crate) vtab_in_first: Option<unsafe extern "C" fn(*mut Sqlite3Value,
        *mut *mut Sqlite3Value) -> i32>,
    pub(crate) vtab_in_next: Option<unsafe extern "C" fn(*mut Sqlite3Value,
        *mut *mut Sqlite3Value) -> i32>,
    pub(crate) deserialize: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *mut u8, i64, i64, u32) -> i32>,
    pub(crate) serialize: Option<unsafe extern "C" fn(*mut Sqlite3, *const i8,
        *mut i64, u32) -> *mut u8>,
    pub(crate) db_name: Option<unsafe extern "C" fn(*mut Sqlite3, i32)
        -> *const i8>,
    pub(crate) value_encoding: Option<unsafe extern "C" fn(*mut Sqlite3Value)
        -> i32>,
    pub(crate) is_interrupted: Option<unsafe extern "C" fn(*mut Sqlite3)
        -> i32>,
    pub(crate) stmt_explain: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32) -> i32>,
    pub(crate) get_clientdata: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8) -> *mut ()>,
    pub(crate) set_clientdata: Option<unsafe extern "C" fn(*mut Sqlite3,
        *const i8, *mut (), unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) setlk_timeout: Option<unsafe extern "C" fn(*mut Sqlite3, i32,
        i32) -> i32>,
    pub(crate) set_errmsg: Option<unsafe extern "C" fn(*mut Sqlite3, i32,
        *const i8) -> i32>,
    pub(crate) db_status64: Option<unsafe extern "C" fn(*mut Sqlite3, i32,
        *mut i64, *mut i64, i32) -> i32>,
    pub(crate) str_truncate: Option<unsafe extern "C" fn(*mut Sqlite3Str, i32)
        -> ()>,
    pub(crate) str_free: Option<unsafe extern "C" fn(*mut Sqlite3Str) -> ()>,
    pub(crate) carray_bind: Option<unsafe extern "C" fn(*mut Sqlite3Stmt, i32,
        *mut (), i32, i32, unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    pub(crate) carray_bind_v2: Option<unsafe extern "C" fn(*mut Sqlite3Stmt,
        i32, *mut (), i32, i32, unsafe extern "C" fn(*mut ()) -> (), *mut ())
        -> i32>,
    pub(crate) incomplete: Option<unsafe extern "C" fn(*const i8) -> i64>,
    pub(crate) result_str: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        *mut Sqlite3Str, i32) -> ()>,
}

///* This is the function signature used for all extension entry points.  It
///* is also defined in the file "loadext.c".
pub(crate) type Sqlite3LoadextEntry =
    unsafe extern "C" fn(*mut Sqlite3, *mut *mut i8,
        *const Sqlite3ApiRoutines) -> i32;
