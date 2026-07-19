use super::*;
use crate::sqlite_int_h::Sqlite3;

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

///* CAPI3REF: File Name
///*
///* Type [sqlite3_filename] is used by SQLite to pass filenames to the
///* xOpen method of a [VFS]. It may be cast to (const char*) and treated
///* as a normal, nul-terminated, UTF-8 buffer containing the filename, but
///* may also be passed to special APIs such as:
///*
///* <ul>
///* <li>  sqlite3_filename_database()
///* <li>  sqlite3_filename_journal()
///* <li>  sqlite3_filename_wal()
///* <li>  sqlite3_uri_parameter()
///* <li>  sqlite3_uri_boolean()
///* <li>  sqlite3_uri_int64()
///* <li>  sqlite3_uri_key()
///* </ul>
pub(crate) type Sqlite3Filename = *const i8;

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

pub(crate) type SqliteInt64 = i64;

pub(crate) type Sqlite3Int64 = SqliteInt64;

pub(crate) type Sqlite3SyscallPtr = unsafe extern "C" fn() -> ();

///* CAPI3REF: Mutex Handle
///*
///* The mutex module within SQLite defines [sqlite3_mutex] to be an
///* abstract type for a mutex object.  The SQLite core never looks
///* at the internal representation of an [sqlite3_mutex].  It only
///* deals with pointers to the [sqlite3_mutex] object.
///*
///* Mutexes are created using [sqlite3_mutex_alloc()].
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Mutex {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type SqliteUint64 = u64;

///* CAPI3REF: SQL Function Context Object
///*
///* The context in which an SQL function executes is stored in an
///* sqlite3_context object.  ^A pointer to an sqlite3_context object
///* is always the first parameter to [application-defined SQL functions].
///* The application-defined SQL function implementation will pass this
///* pointer through into calls to [sqlite3_result_int | sqlite3_result()],
///* [sqlite3_aggregate_context()], [sqlite3_user_data()],
///* [sqlite3_context_db_handle()], [sqlite3_get_auxdata()],
///* and/or [sqlite3_set_auxdata()].
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Context {
    pub(crate) _opaque: [u8; 0],
}

///* CAPI3REF: Dynamically Typed Value Object
///* KEYWORDS: {protected sqlite3_value} {unprotected sqlite3_value}
///*
///* SQLite uses the sqlite3_value object to represent all values
///* that can be stored in a database table. SQLite uses dynamic typing
///* for the values it stores.  ^Values stored in sqlite3_value objects
///* can be integers, floating point values, strings, BLOBs, or NULL.
///*
///* An sqlite3_value object may be either "protected" or "unprotected".
///* Some interfaces require a protected sqlite3_value.  Other interfaces
///* will accept either a protected or an unprotected sqlite3_value.
///* Every interface that accepts sqlite3_value arguments specifies
///* whether or not it requires a protected sqlite3_value.  The
///* [sqlite3_value_dup()] interface can be used to construct a new
///* protected sqlite3_value from an unprotected sqlite3_value.
///*
///* The terms "protected" and "unprotected" refer to whether or not
///* a mutex is held.  An internal mutex is held for a protected
///* sqlite3_value object but no mutex is held for an unprotected
///* sqlite3_value object.  If SQLite is compiled to be single-threaded
///* (with [SQLITE_THREADSAFE=0] and with [sqlite3_threadsafe()] returning 0)
///* or if SQLite is run in one of reduced mutex modes
///* [SQLITE_CONFIG_SINGLETHREAD] or [SQLITE_CONFIG_MULTITHREAD]
///* then there is no distinction between protected and unprotected
///* sqlite3_value objects and they can be used interchangeably.  However,
///* for maximum code portability it is recommended that applications
///* still make the distinction between protected and unprotected
///* sqlite3_value objects even when not strictly required.
///*
///* ^The sqlite3_value objects that are passed as parameters into the
///* implementation of [application-defined SQL functions] are protected.
///* ^The sqlite3_value objects returned by [sqlite3_vtab_rhs_value()]
///* are protected.
///* ^The sqlite3_value object returned by
///* [sqlite3_column_value()] is unprotected.
///* Unprotected sqlite3_value objects may only be used as arguments
///* to [sqlite3_result_value()], [sqlite3_bind_value()], and
///* [sqlite3_value_dup()].
///* The [sqlite3_value_blob | sqlite3_value_type()] family of
///* interfaces require protected sqlite3_value objects.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Value {
    pub(crate) _opaque: [u8; 0],
}

///* CAPI3REF: Virtual Table Object
///* KEYWORDS: sqlite3_module {virtual table module}
///*
///* This structure, sometimes called a "virtual table module",
///* defines the implementation of a [virtual table].
///* This structure consists mostly of methods for the module.
///*
///* ^A virtual table module is created by filling in a persistent
///* instance of this structure and passing a pointer to that instance
///* to [sqlite3_create_module()] or [sqlite3_create_module_v2()].
///* ^The registration remains valid until it is replaced by a different
///* module or until the [database connection] closes.  The content
///* of this structure must not change while it is registered with
///* any database connection.
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

///* CAPI3REF: Virtual Table Instance Object
///* KEYWORDS: sqlite3_vtab
///*
///* Every [virtual table module] implementation uses a subclass
///* of this object to describe a particular instance
///* of the [virtual table].  Each subclass will
///* be tailored to the specific needs of the module implementation.
///* The purpose of this superclass is to define certain fields that are
///* common to all module implementations.
///*
///* ^Virtual tables methods can set an error message by assigning a
///* string obtained from [sqlite3_mprintf()] to zErrMsg.  The method should
///* take care that any prior string is freed by a call to [sqlite3_free()]
///* prior to assigning a new string to zErrMsg.  ^After the error message
///* is delivered up to the client application, the string will be automatically
///* freed by sqlite3_free() and the zErrMsg field will be zeroed.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Vtab {
    pub(crate) p_module: *const Sqlite3Module,
    pub(crate) n_ref: i32,
    pub(crate) z_err_msg: *mut i8,
}

///* CAPI3REF: Virtual Table Indexing Information
///* KEYWORDS: sqlite3_index_info
///*
///* The sqlite3_index_info structure and its substructures is used as part
///* of the [virtual table] interface to
///* pass information into and receive the reply from the [xBestIndex]
///* method of a [virtual table module].  The fields under **Inputs** are the
///* inputs to xBestIndex and are read-only.  xBestIndex inserts its
///* results into the **Outputs** fields.
///*
///* ^(The aConstraint[] array records WHERE clause constraints of the form:
///*
///* <blockquote>column OP expr</blockquote>
///*
///* where OP is =, &lt;, &lt;=, &gt;, or &gt;=.)^  ^(The particular operator is
///* stored in aConstraint[].op using one of the
///* [SQLITE_INDEX_CONSTRAINT_EQ | SQLITE_INDEX_CONSTRAINT_ values].)^
///* ^(The index of the column is stored in
///* aConstraint[].iColumn.)^  ^(aConstraint[].usable is TRUE if the
///* expr on the right-hand side can be evaluated (and thus the constraint
///* is usable) and false if it cannot.)^
///*
///* ^The optimizer automatically inverts terms of the form "expr OP column"
///* and makes other simplifications to the WHERE clause in an attempt to
///* get as many WHERE clause terms into the form shown above as possible.
///* ^The aConstraint[] array only reports WHERE clause terms that are
///* relevant to the particular virtual table being queried.
///*
///* ^Information about the ORDER BY clause is stored in aOrderBy[].
///* ^Each term of aOrderBy records a column of the ORDER BY clause.
///*
///* The colUsed field indicates which columns of the virtual table may be
///* required by the current scan. Virtual table columns are numbered from
///* zero in the order in which they appear within the CREATE TABLE statement
///* passed to sqlite3_declare_vtab(). For the first 63 columns (columns 0-62),
///* the corresponding bit is set within the colUsed mask if the column may be
///* required by SQLite. If the table has at least 64 columns and any column
///* to the right of the first 63 is required, then bit 63 of colUsed is also
///* set. In other words, column iCol may be required if the expression
///* (colUsed & ((sqlite3_uint64)1 << (iCol>=63 ? 63 : iCol))) evaluates to
///* non-zero.
///*
///* The [xBestIndex] method must fill aConstraintUsage[] with information
///* about what parameters to pass to xFilter.  ^If argvIndex>0 then
///* the right-hand side of the corresponding aConstraint[] is evaluated
///* and becomes the argvIndex-th entry in argv.  ^(If aConstraintUsage[].omit
///* is true, then the constraint is assumed to be fully handled by the
///* virtual table and might not be checked again by the byte code.)^ ^(The
///* aConstraintUsage[].omit flag is an optimization hint. When the omit flag
///* is left in its default setting of false, the constraint will always be
///* checked separately in byte code.  If the omit flag is changed to true, then
///* the constraint may or may not be checked in byte code.  In other words,
///* when the omit flag is true there is no guarantee that the constraint will
///* not be checked again using byte code.)^
///*
///* ^The idxNum and idxStr values are recorded and passed into the
///* [xFilter] method.
///* ^[sqlite3_free()] is used to free idxStr if and only if
///* needToFreeIdxStr is true.
///*
///* ^The orderByConsumed means that output from [xFilter]/[xNext] will occur in
///* the correct order to satisfy the ORDER BY clause so that no separate
///* sorting step is required.
///*
///* ^The estimatedCost value is an estimate of the cost of a particular
///* strategy. A cost of N indicates that the cost of the strategy is similar
///* to a linear scan of an SQLite table with N rows. A cost of log(N)
///* indicates that the expense of the operation is similar to that of a
///* binary search on a unique indexed field of an SQLite table with N rows.
///*
///* ^The estimatedRows value is an estimate of the number of rows that
///* will be returned by the strategy.
///*
///* The xBestIndex method may optionally populate the idxFlags field with a
///* mask of SQLITE_INDEX_SCAN_* flags. One such flag is
///* [SQLITE_INDEX_SCAN_HEX], which if set causes the [EXPLAIN QUERY PLAN]
///* output to show the idxNum as hex instead of as decimal.  Another flag is
///* SQLITE_INDEX_SCAN_UNIQUE, which if set indicates that the query plan will
///* return at most one row.
///*
///* Additionally, if xBestIndex sets the SQLITE_INDEX_SCAN_UNIQUE flag, then
///* SQLite also assumes that if a call to the xUpdate() method is made as
///* part of the same statement to delete or update a virtual table row and the
///* implementation returns SQLITE_CONSTRAINT, then there is no need to rollback
///* any database changes. In other words, if the xUpdate() returns
///* SQLITE_CONSTRAINT, the database contents must be exactly as they were
///* before xUpdate was called. By contrast, if SQLITE_INDEX_SCAN_UNIQUE is not
///* set and xUpdate returns SQLITE_CONSTRAINT, any database changes made by
///* the xUpdate method are automatically rolled back by SQLite.
///*
///* IMPORTANT: The estimatedRows field was added to the sqlite3_index_info
///* structure for SQLite [version 3.8.2] ([dateof:3.8.2]).
///* If a virtual table extension is
///* used with an SQLite version earlier than 3.8.2, the results of attempting
///* to read or write the estimatedRows field are undefined (but are likely
///* to include crashing the application). The estimatedRows field should
///* therefore only be used if [sqlite3_libversion_number()] returns a
///* value greater than or equal to 3008002. Similarly, the idxFlags field
///* was added for [version 3.9.0] ([dateof:3.9.0]).
///* It may therefore only be used if
///* sqlite3_libversion_number() returns a value greater than or equal to
///* 3009000.
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

pub(crate) type Sqlite3Uint64 = SqliteUint64;

///* CAPI3REF: Virtual Table Cursor Object
///* KEYWORDS: sqlite3_vtab_cursor {virtual table cursor}
///*
///* Every [virtual table module] implementation uses a subclass of the
///* following structure to describe cursors that point into the
///* [virtual table] and are used
///* to loop through the virtual table.  Cursors are created using the
///* [sqlite3_module.xOpen | xOpen] method of the module and are destroyed
///* by the [sqlite3_module.xClose | xClose] method.  Cursors are used
///* by the [xFilter], [xNext], [xEof], [xColumn], and [xRowid] methods
///* of the module.  Each module implementation will define
///* the content of a cursor structure to suit its own needs.
///*
///* This superclass exists in order to define fields of the cursor that
///* are common to all implementations.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3VtabCursor {
    pub(crate) p_vtab: *mut Sqlite3Vtab,
}

///* The type for a callback function.
///* This is legacy and deprecated.  It is included for historical
///* compatibility and is not documented.
pub(crate) type Sqlite3Callback =
    unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32;

///* CAPI3REF: Loadable Extension Thunk
///*
///* A pointer to the opaque sqlite3_api_routines structure is passed as
///* the third parameter to entry points of [loadable extensions].  This
///* structure must be typedefed in order to work around compiler warnings
///* on some platforms.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3ApiRoutines {
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

///* CAPI3REF: Prepared Statement Object
///* KEYWORDS: {prepared statement} {prepared statements}
///*
///* An instance of this object represents a single SQL statement that
///* has been compiled into binary form and is ready to be evaluated.
///*
///* Think of each SQL statement as a separate computer program.  The
///* original SQL text is source code.  A prepared statement object
///* is the compiled object code.  All SQL must be converted into a
///* prepared statement before it can be run.
///*
///* The life-cycle of a prepared statement object usually goes like this:
///*
///* <ol>
///* <li> Create the prepared statement object using [sqlite3_prepare_v2()].
///* <li> Bind values to [parameters] using the sqlite3_bind_*()
///*      interfaces.
///* <li> Run the SQL by calling [sqlite3_step()] one or more times.
///* <li> Reset the prepared statement using [sqlite3_reset()] then go back
///*      to step 2.  Do this zero or more times.
///* <li> Destroy the object using [sqlite3_finalize()].
///* </ol>
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Stmt {
    pub(crate) _opaque: [u8; 0],
}

///* CAPI3REF: Constants Defining Special Destructor Behavior
///*
///* These are special values for the destructor that is passed in as the
///* final argument to routines like [sqlite3_result_blob()].  ^If the destructor
///* argument is SQLITE_STATIC, it means that the content pointer is constant
///* and will never change.  It does not need to be destroyed.  ^The
///* SQLITE_TRANSIENT value means that the content will likely change in
///* the near future and that SQLite should make its own private copy of
///* the content before returning.
///*
///* The typedef is necessary to work around problems in certain
///* C++ compilers.
pub(crate) type Sqlite3DestructorType = unsafe extern "C" fn(*mut ()) -> ();

///* CAPI3REF: A Handle To An Open BLOB
///* KEYWORDS: {BLOB handle} {BLOB handles}
///*
///* An instance of this object represents an open BLOB on which
///* [sqlite3_blob_open | incremental BLOB I/O] can be performed.
///* ^Objects of this type are created by [sqlite3_blob_open()]
///* and destroyed by [sqlite3_blob_close()].
///* ^The [sqlite3_blob_read()] and [sqlite3_blob_write()] interfaces
///* can be used to read or write small subsections of the BLOB.
///* ^The [sqlite3_blob_bytes()] interface returns the size of the BLOB in bytes.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Blob {
    pub(crate) _opaque: [u8; 0],
}

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

///* CAPI3REF: Custom Page Cache Object
///*
///* The sqlite3_pcache type is opaque.  It is implemented by
///* the pluggable module.  The SQLite core has no knowledge of
///* its size or internal structure and never deals with the
///* sqlite3_pcache object except by holding and passing pointers
///* to the object.
///*
///* See [sqlite3_pcache_methods2] for additional information.
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

///* CAPI3REF: Online Backup Object
///*
///* The sqlite3_backup object records state information about an ongoing
///* online backup operation.  ^The sqlite3_backup object is created by
///* a call to [sqlite3_backup_init()] and is destroyed by a call to
///* [sqlite3_backup_finish()].
///*
///* See Also: [Using the SQLite Online Backup API]
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Backup {
    pub(crate) _opaque: [u8; 0],
}

///* CAPI3REF: Database Snapshot
///* KEYWORDS: {snapshot} {sqlite3_snapshot}
///*
///* An instance of the snapshot object records the state of a [WAL mode]
///* database for some specific point in history.
///*
///* In [WAL mode], multiple [database connections] that are open on the
///* same database file can each be reading a different historical version
///* of the database file.  When a [database connection] begins a read
///* transaction, that connection sees an unchanging copy of the database
///* as it existed for the point in time when the transaction first started.
///* Subsequent changes to the database from other connections are not seen
///* by the reader until a new read transaction is started.
///*
///* The sqlite3_snapshot object records state information about an historical
///* version of the database file so that it is possible to later open a new read
///* transaction that sees that historical version of the database rather than
///* the most recent version.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Snapshot {
    pub(crate) hidden: [u8; 48],
}

///* A pointer to a structure of the following type is passed as the first
///* argument to callbacks registered using rtree_geometry_callback().
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

///* A pointer to a structure of the following type is passed as the
///* argument to scored geometry callback registered using
///* sqlite3_rtree_query_callback().
///*
///* Note that the first 5 fields of this structure are identical to
///* sqlite3_rtree_geometry.  This structure is a subclass of
///* sqlite3_rtree_geometry.
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

///* EXTENSION API FUNCTIONS
///*
///* xUserData(pFts):
///*   Return a copy of the pUserData pointer passed to the xCreateFunction()
///*   API when the extension function was registered.
///*
///* xColumnTotalSize(pFts, iCol, pnToken):
///*   If parameter iCol is less than zero, set output variable *pnToken
///*   to the total number of tokens in the FTS5 table. Or, if iCol is
///*   non-negative but less than the number of columns in the table, return
///*   the total number of tokens in column iCol, considering all rows in
///*   the FTS5 table.
///*
///*   If parameter iCol is greater than or equal to the number of columns
///*   in the table, SQLITE_RANGE is returned. Or, if an error occurs (e.g.
///*   an OOM condition or IO error), an appropriate SQLite error code is
///*   returned.
///*
///* xColumnCount(pFts):
///*   Return the number of columns in the table.
///*
///* xColumnSize(pFts, iCol, pnToken):
///*   If parameter iCol is less than zero, set output variable *pnToken
///*   to the total number of tokens in the current row. Or, if iCol is
///*   non-negative but less than the number of columns in the table, set
///*   *pnToken to the number of tokens in column iCol of the current row.
///*
///*   If parameter iCol is greater than or equal to the number of columns
///*   in the table, SQLITE_RANGE is returned. Or, if an error occurs (e.g.
///*   an OOM condition or IO error), an appropriate SQLite error code is
///*   returned.
///*
///*   This function may be quite inefficient if used with an FTS5 table
///*   created with the "columnsize=0" option.
///*
///* xColumnText:
///*   If parameter iCol is less than zero, or greater than or equal to the
///*   number of columns in the table, SQLITE_RANGE is returned.
///*
///*   Otherwise, this function attempts to retrieve the text of column iCol of
///*   the current document. If successful, (*pz) is set to point to a buffer
///*   containing the text in utf-8 encoding, (*pn) is set to the size in bytes
///*   (not characters) of the buffer and SQLITE_OK is returned. Otherwise,
///*   if an error occurs, an SQLite error code is returned and the final values
///*   of (*pz) and (*pn) are undefined.
///*
///* xPhraseCount:
///*   Returns the number of phrases in the current query expression.
///*
///* xPhraseSize:
///*   If parameter iCol is less than zero, or greater than or equal to the
///*   number of phrases in the current query, as returned by xPhraseCount,
///*   0 is returned. Otherwise, this function returns the number of tokens in
///*   phrase iPhrase of the query. Phrases are numbered starting from zero.
///*
///* xInstCount:
///*   Set *pnInst to the total number of occurrences of all phrases within
///*   the query within the current row. Return SQLITE_OK if successful, or
///*   an error code (i.e. SQLITE_NOMEM) if an error occurs.
///*
///*   This API can be quite slow if used with an FTS5 table created with the
///*   "detail=none" or "detail=column" option. If the FTS5 table is created
///*   with either "detail=none" or "detail=column" and "content=" option
///*   (i.e. if it is a contentless table), then this API always returns 0.
///*
///* xInst:
///*   Query for the details of phrase match iIdx within the current row.
///*   Phrase matches are numbered starting from zero, so the iIdx argument
///*   should be greater than or equal to zero and smaller than the value
///*   output by xInstCount(). If iIdx is less than zero or greater than
///*   or equal to the value returned by xInstCount(), SQLITE_RANGE is returned.
///*
///*   Otherwise, output parameter *piPhrase is set to the phrase number, *piCol
///*   to the column in which it occurs and *piOff the token offset of the
///*   first token of the phrase. SQLITE_OK is returned if successful, or an
///*   error code (i.e. SQLITE_NOMEM) if an error occurs.
///*
///*   This API can be quite slow if used with an FTS5 table created with the
///*   "detail=none" or "detail=column" option.
///*
///* xRowid:
///*   Returns the rowid of the current row.
///*
///* xTokenize:
///*   Tokenize text using the tokenizer belonging to the FTS5 table.
///*
///* xQueryPhrase(pFts5, iPhrase, pUserData, xCallback):
///*   This API function is used to query the FTS table for phrase iPhrase
///*   of the current query. Specifically, a query equivalent to:
///*
///*       ... FROM ftstable WHERE ftstable MATCH $p ORDER BY rowid
///*
///*   with $p set to a phrase equivalent to the phrase iPhrase of the
///*   current query is executed. Any column filter that applies to
///*   phrase iPhrase of the current query is included in $p. For each
///*   row visited, the callback function passed as the fourth argument
///*   is invoked. The context and API objects passed to the callback
///*   function may be used to access the properties of each matched row.
///*   Invoking Api.xUserData() returns a copy of the pointer passed as
///*   the third argument to pUserData.
///*
///*   If parameter iPhrase is less than zero, or greater than or equal to
///*   the number of phrases in the query, as returned by xPhraseCount(),
///*   this function returns SQLITE_RANGE.
///*
///*   If the callback function returns any value other than SQLITE_OK, the
///*   query is abandoned and the xQueryPhrase function returns immediately.
///*   If the returned value is SQLITE_DONE, xQueryPhrase returns SQLITE_OK.
///*   Otherwise, the error code is propagated upwards.
///*
///*   If the query runs to completion without incident, SQLITE_OK is returned.
///*   Or, if some error occurs before the query completes or is aborted by
///*   the callback, an SQLite error code is returned.
///*
///*
///* xSetAuxdata(pFts5, pAux, xDelete)
///*
///*   Save the pointer passed as the second argument as the extension function's
///*   "auxiliary data". The pointer may then be retrieved by the current or any
///*   future invocation of the same fts5 extension function made as part of
///*   the same MATCH query using the xGetAuxdata() API.
///*
///*   Each extension function is allocated a single auxiliary data slot for
///*   each FTS query (MATCH expression). If the extension function is invoked
///*   more than once for a single FTS query, then all invocations share a
///*   single auxiliary data context.
///*
///*   If there is already an auxiliary data pointer when this function is
///*   invoked, then it is replaced by the new pointer. If an xDelete callback
///*   was specified along with the original pointer, it is invoked at this
///*   point.
///*
///*   The xDelete callback, if one is specified, is also invoked on the
///*   auxiliary data pointer after the FTS5 query has finished.
///*
///*   If an error (e.g. an OOM condition) occurs within this function,
///*   the auxiliary data is set to NULL and an error code returned. If the
///*   xDelete parameter was not NULL, it is invoked on the auxiliary data
///*   pointer before returning.
///*
///*
///* xGetAuxdata(pFts5, bClear)
///*
///*   Returns the current auxiliary data pointer for the fts5 extension
///*   function. See the xSetAuxdata() method for details.
///*
///*   If the bClear argument is non-zero, then the auxiliary data is cleared
///*   (set to NULL) before this function returns. In this case the xDelete,
///*   if any, is not invoked.
///*
///*
///* xRowCount(pFts5, pnRow)
///*
///*   This function is used to retrieve the total number of rows in the table.
///*   In other words, the same value that would be returned by:
///*
///*        SELECT count(*) FROM ftstable;
///*
///* xPhraseFirst()
///*   This function is used, along with type Fts5PhraseIter and the xPhraseNext
///*   method, to iterate through all instances of a single query phrase within
///*   the current row. This is the same information as is accessible via the
///*   xInstCount/xInst APIs. While the xInstCount/xInst APIs are more convenient
///*   to use, this API may be faster under some circumstances. To iterate
///*   through instances of phrase iPhrase, use the following code:
///*
///*       Fts5PhraseIter iter;
///*       int iCol, iOff;
///*       for(pApi->xPhraseFirst(pFts, iPhrase, &iter, &iCol, &iOff);
///*           iCol>=0;
///*           pApi->xPhraseNext(pFts, &iter, &iCol, &iOff)
///*       ){
///*         // An instance of phrase iPhrase at offset iOff of column iCol
///*       }
///*
///*   The Fts5PhraseIter structure is defined above. Applications should not
///*   modify this structure directly - it should only be used as shown above
///*   with the xPhraseFirst() and xPhraseNext() API methods (and by
///*   xPhraseFirstColumn() and xPhraseNextColumn() as illustrated below).
///*
///*   This API can be quite slow if used with an FTS5 table created with the
///*   "detail=none" or "detail=column" option. If the FTS5 table is created
///*   with either "detail=none" or "detail=column" and "content=" option
///*   (i.e. if it is a contentless table), then this API always iterates
///*   through an empty set (all calls to xPhraseFirst() set iCol to -1).
///*
///*   In all cases, matches are visited in (column ASC, offset ASC) order.
///*   i.e. all those in column 0, sorted by offset, followed by those in
///*   column 1, etc.
///*
///* xPhraseNext()
///*   See xPhraseFirst above.
///*
///* xPhraseFirstColumn()
///*   This function and xPhraseNextColumn() are similar to the xPhraseFirst()
///*   and xPhraseNext() APIs described above. The difference is that instead
///*   of iterating through all instances of a phrase in the current row, these
///*   APIs are used to iterate through the set of columns in the current row
///*   that contain one or more instances of a specified phrase. For example:
///*
///*       Fts5PhraseIter iter;
///*       int iCol;
///*       for(pApi->xPhraseFirstColumn(pFts, iPhrase, &iter, &iCol);
///*           iCol>=0;
///*           pApi->xPhraseNextColumn(pFts, &iter, &iCol)
///*       ){
///*         // Column iCol contains at least one instance of phrase iPhrase
///*       }
///*
///*   This API can be quite slow if used with an FTS5 table created with the
///*   "detail=none" option. If the FTS5 table is created with either
///*   "detail=none" "content=" option (i.e. if it is a contentless table),
///*   then this API always iterates through an empty set (all calls to
///*   xPhraseFirstColumn() set iCol to -1).
///*
///*   The information accessed using this API and its companion
///*   xPhraseFirstColumn() may also be obtained using xPhraseFirst/xPhraseNext
///*   (or xInst/xInstCount). The chief advantage of this API is that it is
///*   significantly more efficient than those alternatives when used with
///*   "detail=column" tables.
///*
///* xPhraseNextColumn()
///*   See xPhraseFirstColumn above.
///*
///* xQueryToken(pFts5, iPhrase, iToken, ppToken, pnToken)
///*   This is used to access token iToken of phrase iPhrase of the current
///*   query. Before returning, output parameter *ppToken is set to point
///*   to a buffer containing the requested token, and *pnToken to the
///*   size of this buffer in bytes.
///*
///*   If iPhrase or iToken are less than zero, or if iPhrase is greater than
///*   or equal to the number of phrases in the query as reported by
///*   xPhraseCount(), or if iToken is equal to or greater than the number of
///*   tokens in the phrase, SQLITE_RANGE is returned and *ppToken and *pnToken
///     are both zeroed.
///*
///*   The output text is not a copy of the query text that specified the
///*   token. It is the output of the tokenizer module. For tokendata=1
///*   tables, this includes any embedded 0x00 and trailing data.
///*
///* xInstToken(pFts5, iIdx, iToken, ppToken, pnToken)
///*   This is used to access token iToken of phrase hit iIdx within the
///*   current row. If iIdx is less than zero or greater than or equal to the
///*   value returned by xInstCount(), SQLITE_RANGE is returned.  Otherwise,
///*   output variable (*ppToken) is set to point to a buffer containing the
///*   matching document token, and (*pnToken) to the size of that buffer in
///*   bytes.
///*
///*   The output text is not a copy of the document text that was tokenized.
///*   It is the output of the tokenizer module. For tokendata=1 tables, this
///*   includes any embedded 0x00 and trailing data.
///*
///*   This API may be slow in some cases if the token identified by parameters
///*   iIdx and iToken matched a prefix token in the query. In most cases, the
///*   first call to this API for each prefix token in the query is forced
///*   to scan the portion of the full-text index that matches the prefix
///*   token to collect the extra data required by this API. If the prefix
///*   token matches a large number of token instances in the document set,
///*   this may be a performance problem.
///*
///*   If the user knows in advance that a query may use this API for a
///*   prefix token, FTS5 may be configured to collect all required data as part
///*   of the initial querying of the full-text index, avoiding the second scan
///*   entirely. This also causes prefix queries that do not use this API to
///*   run more slowly and use more memory. FTS5 may be configured in this way
///*   either on a per-table basis using the [FTS5 insttoken | 'insttoken']
///*   option, or on a per-query basis using the
///*   [fts5_insttoken | fts5_insttoken()] user function.
///*
///*   This API can be quite slow if used with an FTS5 table created with the
///*   "detail=none" or "detail=column" option.
///*
///* xColumnLocale(pFts5, iIdx, pzLocale, pnLocale)
///*   If parameter iCol is less than zero, or greater than or equal to the
///*   number of columns in the table, SQLITE_RANGE is returned.
///*
///*   Otherwise, this function attempts to retrieve the locale associated
///*   with column iCol of the current row. Usually, there is no associated
///*   locale, and output parameters (*pzLocale) and (*pnLocale) are set
///*   to NULL and 0, respectively. However, if the fts5_locale() function
///*   was used to associate a locale with the value when it was inserted
///*   into the fts5 table, then (*pzLocale) is set to point to a nul-terminated
///*   buffer containing the name of the locale in utf-8 encoding. (*pnLocale)
///*   is set to the size in bytes of the buffer, not including the
///*   nul-terminator.
///*
///*   If successful, SQLITE_OK is returned. Or, if an error occurs, an
///*   SQLite error code is returned. The final value of the output parameters
///*   is undefined in this case.
///*
///* xTokenize_v2:
///*   Tokenize text using the tokenizer belonging to the FTS5 table. This
///*   API is the same as the xTokenize() API, except that it allows a tokenizer
///*   locale to be specified.
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

///**********************************************************************
///* CUSTOM TOKENIZERS
///*
///* Applications may also register custom tokenizer types. A tokenizer
///* is registered by providing fts5 with a populated instance of the
///* following structure. All structure methods must be defined, setting
///* any member of the fts5_tokenizer struct to NULL leads to undefined
///* behaviour. The structure methods are expected to function as follows:
///*
///* xCreate:
///*   This function is used to allocate and initialize a tokenizer instance.
///*   A tokenizer instance is required to actually tokenize text.
///*
///*   The first argument passed to this function is a copy of the (void*)
///*   pointer provided by the application when the fts5_tokenizer_v2 object
///*   was registered with FTS5 (the third argument to xCreateTokenizer()).
///*   The second and third arguments are an array of nul-terminated strings
///*   containing the tokenizer arguments, if any, specified following the
///*   tokenizer name as part of the CREATE VIRTUAL TABLE statement used
///*   to create the FTS5 table.
///*
///*   The final argument is an output variable. If successful, (*ppOut)
///*   should be set to point to the new tokenizer handle and SQLITE_OK
///*   returned. If an error occurs, some value other than SQLITE_OK should
///*   be returned. In this case, fts5 assumes that the final value of *ppOut
///*   is undefined.
///*
///* xDelete:
///*   This function is invoked to delete a tokenizer handle previously
///*   allocated using xCreate(). Fts5 guarantees that this function will
///*   be invoked exactly once for each successful call to xCreate().
///*
///* xTokenize:
///*   This function is expected to tokenize the nText byte string indicated
///*   by argument pText. pText may or may not be nul-terminated. The first
///*   argument passed to this function is a pointer to an Fts5Tokenizer object
///*   returned by an earlier call to xCreate().
///*
///*   The third argument indicates the reason that FTS5 is requesting
///*   tokenization of the supplied text. This is always one of the following
///*   four values:
///*
///*   <ul><li> <b>FTS5_TOKENIZE_DOCUMENT</b> - A document is being inserted into
///*            or removed from the FTS table. The tokenizer is being invoked to
///*            determine the set of tokens to add to (or delete from) the
///*            FTS index.
///*
///*       <li> <b>FTS5_TOKENIZE_QUERY</b> - A MATCH query is being executed
///*            against the FTS index. The tokenizer is being called to tokenize
///*            a bareword or quoted string specified as part of the query.
///*
///*       <li> <b>(FTS5_TOKENIZE_QUERY | FTS5_TOKENIZE_PREFIX)</b> - Same as
///*            FTS5_TOKENIZE_QUERY, except that the bareword or quoted string is
///*            followed by a "*" character, indicating that the last token
///*            returned by the tokenizer will be treated as a token prefix.
///*
///*       <li> <b>FTS5_TOKENIZE_AUX</b> - The tokenizer is being invoked to
///*            satisfy an fts5_api.xTokenize() request made by an auxiliary
///*            function. Or an fts5_api.xColumnSize() request made by the same
///*            on a columnsize=0 database.
///*   </ul>
///*
///*   The sixth and seventh arguments passed to xTokenize() - pLocale and
///*   nLocale - are a pointer to a buffer containing the locale to use for
///*   tokenization (e.g. "en_US") and its size in bytes, respectively. The
///*   pLocale buffer is not nul-terminated. pLocale may be passed NULL (in
///*   which case nLocale is always 0) to indicate that the tokenizer should
///*   use its default locale.
///*
///*   For each token in the input string, the supplied callback xToken() must
///*   be invoked. The first argument to it should be a copy of the pointer
///*   passed as the second argument to xTokenize(). The third and fourth
///*   arguments are a pointer to a buffer containing the token text, and the
///*   size of the token in bytes. The 4th and 5th arguments are the byte offsets
///*   of the first byte of and first byte immediately following the text from
///*   which the token is derived within the input.
///*
///*   The second argument passed to the xToken() callback ("tflags") should
///*   normally be set to 0. The exception is if the tokenizer supports
///*   synonyms. In this case see the discussion below for details.
///*
///*   FTS5 assumes the xToken() callback is invoked for each token in the
///*   order that they occur within the input text.
///*
///*   If an xToken() callback returns any value other than SQLITE_OK, then
///*   the tokenization should be abandoned and the xTokenize() method should
///*   immediately return a copy of the xToken() return value. Or, if the
///*   input buffer is exhausted, xTokenize() should return SQLITE_OK. Finally,
///*   if an error occurs with the xTokenize() implementation itself, it
///*   may abandon the tokenization and return any error code other than
///*   SQLITE_OK or SQLITE_DONE.
///*
///*   If the tokenizer is registered using an fts5_tokenizer_v2 object,
///*   then the xTokenize() method has two additional arguments - pLocale
///*   and nLocale. These specify the locale that the tokenizer should use
///*   for the current request. If pLocale and nLocale are both 0, then the
///*   tokenizer should use its default locale. Otherwise, pLocale points to
///*   an nLocale byte buffer containing the name of the locale to use as utf-8
///*   text. pLocale is not nul-terminated.
///*
///* FTS5_TOKENIZER
///*
///* There is also an fts5_tokenizer object. This is an older, deprecated,
///* version of fts5_tokenizer_v2. It is similar except that:
///*
///*  <ul>
///*    <li> There is no "iVersion" field, and
///*    <li> The xTokenize() method does not take a locale argument.
///*  </ul>
///*
///* Legacy fts5_tokenizer tokenizers must be registered using the
///* legacy xCreateTokenizer() function, instead of xCreateTokenizer_v2().
///*
///* Tokenizer implementations registered using either API may be retrieved
///* using both xFindTokenizer() and xFindTokenizer_v2().
///*
///* SYNONYM SUPPORT
///*
///*   Custom tokenizers may also support synonyms. Consider a case in which a
///*   user wishes to query for a phrase such as "first place". Using the
///*   built-in tokenizers, the FTS5 query 'first + place' will match instances
///*   of "first place" within the document set, but not alternative forms
///*   such as "1st place". In some applications, it would be better to match
///*   all instances of "first place" or "1st place" regardless of which form
///*   the user specified in the MATCH query text.
///*
///*   There are several ways to approach this in FTS5:
///*
///*   <ol><li> By mapping all synonyms to a single token. In this case, using
///*            the above example, this means that the tokenizer returns the
///*            same token for inputs "first" and "1st". Say that token is in
///*            fact "first", so that when the user inserts the document "I won
///*            1st place" entries are added to the index for tokens "i", "won",
///*            "first" and "place". If the user then queries for '1st + place',
///*            the tokenizer substitutes "first" for "1st" and the query works
///*            as expected.
///*
///*       <li> By querying the index for all synonyms of each query term
///*            separately. In this case, when tokenizing query text, the
///*            tokenizer may provide multiple synonyms for a single term
///*            within the document. FTS5 then queries the index for each
///*            synonym individually. For example, faced with the query:
///*
///*   <codeblock>
///*     ... MATCH 'first place'</codeblock>
///*
///*            the tokenizer offers both "1st" and "first" as synonyms for the
///*            first token in the MATCH query and FTS5 effectively runs a query
///*            similar to:
///*
///*   <codeblock>
///*     ... MATCH '(first OR 1st) place'</codeblock>
///*
///*            except that, for the purposes of auxiliary functions, the query
///*            still appears to contain just two phrases - "(first OR 1st)"
///*            being treated as a single phrase.
///*
///*       <li> By adding multiple synonyms for a single term to the FTS index.
///*            Using this method, when tokenizing document text, the tokenizer
///*            provides multiple synonyms for each token. So that when a
///*            document such as "I won first place" is tokenized, entries are
///*            added to the FTS index for "i", "won", "first", "1st" and
///*            "place".
///*
///*            This way, even if the tokenizer does not provide synonyms
///*            when tokenizing query text (it should not - to do so would be
///*            inefficient), it doesn't matter if the user queries for
///*            'first + place' or '1st + place', as there are entries in the
///*            FTS index corresponding to both forms of the first token.
///*   </ol>
///*
///*   Whether it is parsing document or query text, any call to xToken that
///*   specifies a <i>tflags</i> argument with the FTS5_TOKEN_COLOCATED bit
///*   is considered to supply a synonym for the previous token. For example,
///*   when parsing the document "I won first place", a tokenizer that supports
///*   synonyms would call xToken() 5 times, as follows:
///*
///*   <codeblock>
///*       xToken(pCtx, 0, "i",                      1,  0,  1);
///*       xToken(pCtx, 0, "won",                    3,  2,  5);
///*       xToken(pCtx, 0, "first",                  5,  6, 11);
///*       xToken(pCtx, FTS5_TOKEN_COLOCATED, "1st", 3,  6, 11);
///*       xToken(pCtx, 0, "place",                  5, 12, 17);
///*</codeblock>
///*
///*   It is an error to specify the FTS5_TOKEN_COLOCATED flag the first time
///*   xToken() is called. Multiple synonyms may be specified for a single token
///*   by making multiple calls to xToken(FTS5_TOKEN_COLOCATED) in sequence.
///*   There is no limit to the number of synonyms that may be provided for a
///*   single token.
///*
///*   In many cases, method (1) above is the best approach. It does not add
///*   extra data to the FTS index or require FTS5 to query for multiple terms,
///*   so it is efficient in terms of disk space and query speed. However, it
///*   does not support prefix queries very well. If, as suggested above, the
///*   token "first" is substituted for "1st" by the tokenizer, then the query:
///*
///*   <codeblock>
///*     ... MATCH '1s*'</codeblock>
///*
///*   will not match documents that contain the token "1st" (as the tokenizer
///*   will probably not map "1s" to any prefix of "first").
///*
///*   For full prefix support, method (3) may be preferred. In this case,
///*   because the index contains entries for both "first" and "1st", prefix
///*   queries such as 'fi*' or '1s*' will match correctly. However, because
///*   extra entries are added to the FTS index, this method uses more space
///*   within the database.
///*
///*   Method (2) offers a midpoint between (1) and (3). Using this method,
///*   a query such as '1s*' will match documents that contain the literal
///*   token "1st", but not "first" (assuming the tokenizer is not able to
///*   provide synonyms for prefixes). However, a non-prefix query like '1st'
///*   will match against "1st" and "first". This method does not require
///*   extra disk space, as no extra entries are added to the FTS index.
///*   On the other hand, it may require more CPU cycles to run MATCH queries,
///*   as separate queries of the FTS index are required for each synonym.
///*
///*   When using methods (2) or (3), it is important that the tokenizer only
///*   provide synonyms when tokenizing document text (method (3)) or query
///*   text (method (2)), not both. Doing so will not cause any errors, but is
///*   inefficient.
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

///**********************************************************************
///* FTS5 EXTENSION REGISTRATION API
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
