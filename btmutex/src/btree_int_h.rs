use super::*;
use crate::pager_h::{DbPage, Pager, Pgno};
use crate::sqlite3_h::Sqlite3Mutex;
use crate::sqlite_int_h::{Bitvec, KeyInfo, Sqlite3, StrAccum};

/// A Btree handle
///*
///* A database connection contains a pointer to an instance of
///* this object for every database file that it has open.  This structure
///* is opaque to the database connection.  The database connection cannot
///* see the internals of this structure and only deals with pointers to
///* this structure.
///*
///* For some database files, the same underlying database cache might be 
///* shared between multiple connections.  In that case, each connection
///* has it own instance of this object.  But each instance of this object
///* points to the same BtShared object.  The database cache and the
///* schema associated with the database file are all contained within
///* the BtShared object.
///*
///* All fields in this structure are accessed under sqlite3.mutex.
///* The pBt pointer itself may not be changed while there exists cursors 
///* in the referenced BtShared that point back to this Btree since those
///* cursors have to go through this Btree to find their BtShared and
///* they often do so without holding sqlite3.mutex.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Btree {
    pub(crate) db: *mut Sqlite3,
    pub(crate) p_bt: *mut BtShared,
    pub(crate) in_trans: u8,
    pub(crate) sharable: u8,
    pub(crate) locked: u8,
    pub(crate) has_incrblob_cur: u8,
    pub(crate) want_to_lock: i32,
    pub(crate) n_backup: i32,
    pub(crate) i_b_data_version: u32,
    pub(crate) p_next: *mut Btree,
    pub(crate) p_prev: *mut Btree,
    pub(crate) lock: BtLock,
}

///* An instance of this object represents a single database file.
///* 
///* A single database file can be in use at the same time by two
///* or more database connections.  When two or more connections are
///* sharing the same database file, each connection has it own
///* private Btree object for the file and each of those Btrees points
///* to this one BtShared object.  BtShared.nRef is the number of
///* connections currently sharing this database file.
///*
///* Fields in this structure are accessed under the BtShared.mutex
///* mutex, except for nRef and pNext which are accessed under the
///* global SQLITE_MUTEX_STATIC_MAIN mutex.  The pPager field
///* may not be modified once it is initially set as long as nRef>0.
///* The pSchema field may be set once under BtShared.mutex and
///* thereafter is unchanged as long as nRef>0.
///*
///* isPending:
///*
///*   If a BtShared client fails to obtain a write-lock on a database
///*   table (because there exists one or more read-locks on the table),
///*   the shared-cache enters 'pending-lock' state and isPending is
///*   set to true.
///*
///*   The shared-cache leaves the 'pending lock' state when either of
///*   the following occur:
///*
///*     1) The current writer (BtShared.pWriter) concludes its transaction, OR
///*     2) The number of locks held by other connections drops to zero.
///*
///*   while in the 'pending-lock' state, no connection may start a new
///*   transaction.
///*
///*   This feature is included to help prevent writer-starvation.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtShared {
    pub(crate) p_pager: *mut Pager,
    pub(crate) db: *mut Sqlite3,
    pub(crate) p_cursor: *mut BtCursor,
    pub(crate) p_page1: *mut MemPage,
    pub(crate) open_flags: u8,
    pub(crate) auto_vacuum: u8,
    pub(crate) incr_vacuum: u8,
    pub(crate) b_do_truncate: u8,
    pub(crate) in_transaction: u8,
    pub(crate) max1byte_payload: u8,
    pub(crate) n_reserve_wanted: u8,
    pub(crate) bts_flags: u16,
    pub(crate) max_local: u16,
    pub(crate) min_local: u16,
    pub(crate) max_leaf: u16,
    pub(crate) min_leaf: u16,
    pub(crate) page_size: u32,
    pub(crate) usable_size: u32,
    pub(crate) n_transaction: i32,
    pub(crate) n_page: u32,
    pub(crate) p_schema: *mut (),
    pub(crate) x_free_schema: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) mutex: *mut Sqlite3Mutex,
    pub(crate) p_has_content: *mut Bitvec,
    pub(crate) n_ref: i32,
    pub(crate) p_next: *mut BtShared,
    pub(crate) p_lock: *mut BtLock,
    pub(crate) p_writer: *mut Btree,
    pub(crate) p_tmp_space: *mut u8,
    pub(crate) n_preformat_size: i32,
}

///* A cursor is a pointer to a particular entry within a particular
///* b-tree within a database file.
///*
///* The entry is identified by its MemPage and the index in
///* MemPage.aCell[] of the entry.
///*
///* A single database file can be shared by two more database connections,
///* but cursors cannot be shared.  Each cursor is associated with a
///* particular database connection identified BtCursor.pBtree.db.
///*
///* Fields in this structure are accessed under the BtShared.mutex
///* found at self->pBt->mutex. 
///*
///* skipNext meaning:
///* The meaning of skipNext depends on the value of eState:
///*
///*   eState            Meaning of skipNext
///*   VALID             skipNext is meaningless and is ignored
///*   INVALID           skipNext is meaningless and is ignored
///*   SKIPNEXT          sqlite3BtreeNext() is a no-op if skipNext>0 and
///*                     sqlite3BtreePrevious() is no-op if skipNext<0.
///*   REQUIRESEEK       restoreCursorPosition() restores the cursor to
///*                     eState=SKIPNEXT if skipNext!=0
///*   FAULT             skipNext holds the cursor fault error code.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtCursor {
    pub(crate) e_state: u8,
    pub(crate) cur_flags: u8,
    pub(crate) cur_pager_flags: u8,
    pub(crate) hints: u8,
    pub(crate) skip_next: i32,
    pub(crate) p_btree: *mut Btree,
    pub(crate) a_overflow: *mut Pgno,
    pub(crate) p_key: *mut (),
    pub(crate) p_bt: *mut BtShared,
    pub(crate) p_next: *mut BtCursor,
    pub(crate) info: CellInfo,
    pub(crate) n_key: i64,
    pub(crate) pgno_root: Pgno,
    pub(crate) i_page: i8,
    pub(crate) cur_int_key: u8,
    pub(crate) ix: u16,
    pub(crate) ai_idx: [u16; 19],
    pub(crate) p_key_info: *mut KeyInfo,
    pub(crate) p_page: *mut MemPage,
    pub(crate) ap_page: [*mut MemPage; 19],
}

///* An instance of the following structure is used to hold information
///* about a cell.  The parseCellPtr() function fills in this structure
///* based on information extract from the raw disk page.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CellInfo {
    pub(crate) n_key: i64,
    pub(crate) p_payload: *mut u8,
    pub(crate) n_payload: u32,
    pub(crate) n_local: u16,
    pub(crate) n_size: u16,
}

///* An instance of this object stores information about each a single database
///* page that has been loaded into memory.  The information in this object
///* is derived from the raw on-disk page content.
///*
///* As each database page is loaded into memory, the pager allocates an
///* instance of this object and zeros the first 8 bytes.  (This is the
///* "extra" information associated with each page of the pager.)
///*
///* Access to all fields of this structure is controlled by the mutex
///* stored in MemPage.pBt->mutex.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct MemPage {
    pub(crate) is_init: u8,
    pub(crate) int_key: u8,
    pub(crate) int_key_leaf: u8,
    pub(crate) pgno: Pgno,
    pub(crate) leaf: u8,
    pub(crate) hdr_offset: u8,
    pub(crate) child_ptr_size: u8,
    pub(crate) max1byte_payload: u8,
    pub(crate) n_overflow: u8,
    pub(crate) max_local: u16,
    pub(crate) min_local: u16,
    pub(crate) cell_offset: u16,
    pub(crate) n_free: i32,
    pub(crate) n_cell: u16,
    pub(crate) mask_page: u16,
    pub(crate) ai_ovfl: [u16; 4],
    pub(crate) ap_ovfl: [*mut u8; 4],
    pub(crate) p_bt: *mut BtShared,
    pub(crate) a_data: *mut u8,
    pub(crate) a_data_end: *mut u8,
    pub(crate) a_cell_idx: *mut u8,
    pub(crate) a_data_ofst: *mut u8,
    pub(crate) p_db_page: *mut DbPage,
    pub(crate) x_cell_size: Option<unsafe extern "C" fn(*mut MemPage, *mut u8)
        -> u16>,
    pub(crate) x_parse_cell: Option<unsafe extern "C" fn(*mut MemPage,
        *mut u8, *mut CellInfo) -> ()>,
}

///* A linked list of the following structures is stored at BtShared.pLock.
///* Locks are added (or upgraded from READ_LOCK to WRITE_LOCK) when a cursor 
///* is opened on the table with root page BtShared.iTable. Locks are removed
///* from this list when a transaction is committed or rolled back, or when
///* a btree handle is closed.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtLock {
    pub(crate) p_btree: *mut Btree,
    pub(crate) i_table: Pgno,
    pub(crate) e_lock: u8,
    pub(crate) p_next: *mut BtLock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IntegrityCk {
    pub(crate) p_bt: *mut BtShared,
    pub(crate) p_pager: *mut Pager,
    pub(crate) a_pg_ref: *mut u8,
    pub(crate) n_ck_page: Pgno,
    pub(crate) mx_err: i32,
    pub(crate) n_err: i32,
    pub(crate) rc: i32,
    pub(crate) n_step: u32,
    pub(crate) z_pfx: *const i8,
    pub(crate) v0: Pgno,
    pub(crate) v1: Pgno,
    pub(crate) v2: i32,
    pub(crate) err_msg: StrAccum,
    pub(crate) heap: *mut u32,
    pub(crate) db: *mut Sqlite3,
    pub(crate) n_row: i64,
}
