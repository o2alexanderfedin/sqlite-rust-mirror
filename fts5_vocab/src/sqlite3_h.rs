use super::*;

///* CAPI3REF: Database Connection Handle
///* KEYWORDS: {database connection} {database connections}
///*
///* Each open SQLite database is represented by a pointer to an instance of
///* the opaque structure named "sqlite3".  It is useful to think of an sqlite3
///* pointer as an object.  The [sqlite3_open()], [sqlite3_open16()], and
///* [sqlite3_open_v2()] interfaces are its constructors, and [sqlite3_close()]
///* and [sqlite3_close_v2()] are its destructors.  There are many other
///* interfaces (such as
///* [sqlite3_prepare_v2()], [sqlite3_create_function()], and
///* [sqlite3_busy_timeout()] to name but three) that are methods on an
///* sqlite3 object.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3 {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type SqliteInt64 = i64;

pub(crate) type SqliteUint64 = u64;

pub(crate) type Sqlite3Int64 = SqliteInt64;

pub(crate) type Sqlite3Uint64 = SqliteUint64;

///* The type for a callback function.
///* This is legacy and deprecated.  It is included for historical
///* compatibility and is not documented.
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

pub(crate) type Sqlite3SyscallPtr = unsafe extern "C" fn() -> ();

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

///* CAPI3REF: Dynamic String Object
///* KEYWORDS: {dynamic string}
///*
///* An instance of the sqlite3_str object contains a dynamically-sized
///* string under construction.
///*
///* The lifecycle of an sqlite3_str object is as follows:
///* <ol>
///* <li> ^The sqlite3_str object is created using [sqlite3_str_new()].
///* <li> ^Text is appended to the sqlite3_str object using various
///* methods, such as [sqlite3_str_appendf()].
///* <li> The sqlite3_str object is destroyed and the string it created
///* is returned using [sqlite3_str_finish()] or [sqlite3_result_str()].
///* </ol>
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
