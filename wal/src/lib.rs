#![allow(unused_imports, dead_code)]

mod btree_h;
mod hash_h;
mod pager_h;
mod pcache_h;
mod sqlite3_h;
mod sqlite_int_h;
mod vdbe_h;
use crate::btree_h::{BtCursor, Btree, BtreePayload};
use crate::hash_h::Hash;
use crate::pager_h::{DbPage, Pager, Pgno};
use crate::pcache_h::{PCache, PgHdr};
use crate::sqlite3_h::{
    Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File, Sqlite3Filename,
    Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module, Sqlite3Mutex,
    Sqlite3MutexMethods, Sqlite3PcachePage, Sqlite3RtreeGeometry,
    Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Uint64,
    Sqlite3Value, Sqlite3Vfs, Sqlite3Vtab,
};
use crate::sqlite_int_h::{
    AuthContext, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte, DbFixer,
    Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode, FuncDef,
    FuncDefHash, FuncDestructor, IdList, Index, KeyInfo, LogEst, Module,
    NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema, Select,
    SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str, SrcItem,
    SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With,
};
use crate::vdbe_h::{Mem, SubProgram, Vdbe, VdbeOp, VdbeOpList};

type DarwinSizeT = u64;

impl Column {
    fn not_null(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0xfu32) as i32 }
    fn set_not_null(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0xfu32) | ((val & 0xfu32) << 0u32);
    }
    fn e_c_type(&self) -> i32 { ((self._bitfield_1 >> 4u32) & 0xfu32) as i32 }
    fn set_e_c_type(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0xfu32 << 4u32)) | ((val & 0xfu32) << 4u32);
    }
}

impl Index {
    fn idx_type(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x3u32) as i32 }
    fn set_idx_type(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x3u32) | ((val & 0x3u32) << 0u32);
    }
    fn b_unordered(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_b_unordered(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn uniq_not_null(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_uniq_not_null(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn is_resized(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_is_resized(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn is_covering(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_is_covering(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn no_skip_scan(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_no_skip_scan(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn has_stat1(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_has_stat1(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn b_no_query(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_b_no_query(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
    fn b_asc_key_bug(&self) -> i32 {
        ((self._bitfield_1 >> 9u32) & 0x1u32) as i32
    }
    fn set_b_asc_key_bug(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 9u32)) | ((val & 0x1u32) << 9u32);
    }
    fn b_has_v_col(&self) -> i32 {
        ((self._bitfield_1 >> 10u32) & 0x1u32) as i32
    }
    fn set_b_has_v_col(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 10u32)) |
                ((val & 0x1u32) << 10u32);
    }
    fn b_has_expr(&self) -> i32 {
        ((self._bitfield_1 >> 11u32) & 0x1u32) as i32
    }
    fn set_b_has_expr(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 11u32)) |
                ((val & 0x1u32) << 11u32);
    }
}

impl ExprListItemS0 {
    fn e_e_name(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x3u32) as i32 }
    fn set_e_e_name(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x3u32) | ((val & 0x3u32) << 0u32);
    }
    fn done(&self) -> i32 { ((self._bitfield_1 >> 2u32) & 0x1u32) as i32 }
    fn set_done(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn reusable(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_reusable(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn b_sorter_ref(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_b_sorter_ref(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn b_nulls(&self) -> i32 { ((self._bitfield_1 >> 5u32) & 0x1u32) as i32 }
    fn set_b_nulls(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn b_used(&self) -> i32 { ((self._bitfield_1 >> 6u32) & 0x1u32) as i32 }
    fn set_b_used(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn b_using_term(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_b_using_term(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn b_no_expand(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_b_no_expand(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
}

impl SrcItemS0 {
    fn not_indexed(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_not_indexed(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn is_indexed_by(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_is_indexed_by(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn is_subquery(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_is_subquery(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn is_tab_func(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_is_tab_func(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn is_correlated(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_is_correlated(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn is_materialized(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_is_materialized(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn via_coroutine(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_via_coroutine(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn is_recursive(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_is_recursive(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn from_ddl(&self) -> i32 { ((self._bitfield_1 >> 8u32) & 0x1u32) as i32 }
    fn set_from_ddl(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
    fn is_cte(&self) -> i32 { ((self._bitfield_1 >> 9u32) & 0x1u32) as i32 }
    fn set_is_cte(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 9u32)) | ((val & 0x1u32) << 9u32);
    }
    fn not_cte(&self) -> i32 { ((self._bitfield_1 >> 10u32) & 0x1u32) as i32 }
    fn set_not_cte(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 10u32)) |
                ((val & 0x1u32) << 10u32);
    }
    fn is_using(&self) -> i32 {
        ((self._bitfield_1 >> 11u32) & 0x1u32) as i32
    }
    fn set_is_using(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 11u32)) |
                ((val & 0x1u32) << 11u32);
    }
    fn is_on(&self) -> i32 { ((self._bitfield_1 >> 12u32) & 0x1u32) as i32 }
    fn set_is_on(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 12u32)) |
                ((val & 0x1u32) << 12u32);
    }
    fn is_synth_using(&self) -> i32 {
        ((self._bitfield_1 >> 13u32) & 0x1u32) as i32
    }
    fn set_is_synth_using(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 13u32)) |
                ((val & 0x1u32) << 13u32);
    }
    fn is_nested_from(&self) -> i32 {
        ((self._bitfield_1 >> 14u32) & 0x1u32) as i32
    }
    fn set_is_nested_from(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 14u32)) |
                ((val & 0x1u32) << 14u32);
    }
    fn rowid_used(&self) -> i32 {
        ((self._bitfield_1 >> 15u32) & 0x1u32) as i32
    }
    fn set_rowid_used(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 15u32)) |
                ((val & 0x1u32) << 15u32);
    }
    fn fixed_schema(&self) -> i32 {
        ((self._bitfield_1 >> 16u32) & 0x1u32) as i32
    }
    fn set_fixed_schema(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 16u32)) |
                ((val & 0x1u32) << 16u32);
    }
    fn had_schema(&self) -> i32 {
        ((self._bitfield_1 >> 17u32) & 0x1u32) as i32
    }
    fn set_had_schema(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 17u32)) |
                ((val & 0x1u32) << 17u32);
    }
    fn from_exists(&self) -> i32 {
        ((self._bitfield_1 >> 18u32) & 0x1u32) as i32
    }
    fn set_from_exists(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 18u32)) |
                ((val & 0x1u32) << 18u32);
    }
}

impl Sqlite3InitInfo {
    fn orphan_trigger(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_orphan_trigger(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn imposter_table(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x3u32) as i32
    }
    fn set_imposter_table(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x3u32 << 1u32)) | ((val & 0x3u32) << 1u32);
    }
    fn reopen_memdb(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_reopen_memdb(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
}

impl Parse {
    fn disable_triggers(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_disable_triggers(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn may_abort(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_may_abort(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn has_compound(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_has_compound(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn b_returning(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_b_returning(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn b_has_exists(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_b_has_exists(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn col_names_set(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_col_names_set(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn b_has_with(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_b_has_with(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn ok_const_factor(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_ok_const_factor(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn check_schema(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_check_schema(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
    fn uses_ainc(&self) -> i32 {
        ((self._bitfield_1 >> 9u32) & 0x1u32) as i32
    }
    fn set_uses_ainc(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 9u32)) | ((val & 0x1u32) << 9u32);
    }
}

///* An open write-ahead log file is represented by an instance of the
///* following object.
///*
///* writeLock:
///*   This is usually set to 1 whenever the WRITER lock is held. However,
///*   if it is set to 2, then the WRITER lock is held but must be released
///*   by walHandleException() if a SEH exception is thrown.
#[repr(C)]
#[derive(Copy, Clone)]
struct Wal {
    p_vfs: *mut Sqlite3Vfs,
    p_db_fd: *mut Sqlite3File,
    p_wal_fd: *mut Sqlite3File,
    i_callback: u32,
    mx_wal_size: i64,
    n_wi_data: i32,
    sz_first_block: i32,
    ap_wi_data: *mut *mut u32,
    sz_page: u32,
    read_lock: i16,
    sync_flags: u8,
    exclusive_mode: u8,
    write_lock: u8,
    ckpt_lock: u8,
    read_only: u8,
    truncate_on_commit: u8,
    sync_header: u8,
    pad_to_sector_boundary: u8,
    b_shm_unreliable: u8,
    hdr: WalIndexHdr,
    min_frame: u32,
    i_re_cksum: u32,
    z_wal_name: *const i8,
    n_ckpt: u32,
}

///* The following object holds a copy of the wal-index header content.
///*
///* The actual header in the wal-index consists of two copies of this
///* object followed by one instance of the WalCkptInfo object.
///* For all versions of SQLite through 3.10.0 and probably beyond,
///* the locking bytes (WalCkptInfo.aLock) start at offset 120 and
///* the total header size is 136 bytes.
///*
///* The szPage value can be any power of 2 between 512 and 32768, inclusive.
///* Or it can be 1 to represent a 65536-byte page.  The latter case was
///* added in 3.7.1 when support for 64K pages was added.
#[repr(C)]
#[derive(Copy, Clone)]
struct WalIndexHdr {
    i_version: u32,
    unused: u32,
    i_change: u32,
    is_init: u8,
    big_end_cksum: u8,
    sz_page: u16,
    mx_frame: u32,
    n_page: u32,
    a_frame_cksum: [u32; 2],
    a_salt: [u32; 2],
    a_cksum: [u32; 2],
}

///* Close an open wal-index.
extern "C" fn wal_index_close(p_wal_1: &Wal, is_delete_1: i32) -> () {
    if (*p_wal_1).exclusive_mode as i32 == 2 ||
            (*p_wal_1).b_shm_unreliable != 0 {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b0: loop {
                if !(i < (*p_wal_1).n_wi_data) { break '__b0; }
                '__c0: loop {
                    unsafe {
                        sqlite3_free(unsafe {
                                    *(*p_wal_1).ap_wi_data.offset(i as isize)
                                } as *mut ())
                    };
                    unsafe {
                        *(*p_wal_1).ap_wi_data.offset(i as isize) =
                            core::ptr::null_mut()
                    };
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if (*p_wal_1).exclusive_mode as i32 != 2 {
        unsafe { sqlite3_os_shm_unmap((*p_wal_1).p_db_fd, is_delete_1) };
    }
}

/// Open and close a connection to a write-ahead log.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_open(p_vfs: *mut Sqlite3Vfs,
    p_db_fd: *mut Sqlite3File, z_wal_name: *const i8, b_no_shm: i32,
    mx_wal_size: i64, pp_wal: &mut *mut Wal) -> i32 {
    let mut rc: i32 = 0;
    /// Return Code
    let mut p_ret: *mut Wal = core::ptr::null_mut();
    /// Object to allocate and return
    let mut flags: i32 = 0;

    /// Flags passed to OsOpen()
    { let _ = 0; };
    { let _ = 0; };

    /// Verify the values of various constants.  Any changes to the values
    ///* of these constants would result in an incompatible on-disk format
    ///* for the -shm file.  Any change that causes one of these asserts to
    ///* fail is a backward compatibility problem, even if the change otherwise
    ///* works.
    ///*
    ///* This table also serves as a helpful cross-reference when trying to
    ///* interpret hex dumps of the -shm file.
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };

    /// In the amalgamation, the os_unix.c and os_win.c source files come before
    ///* this source file.  Verify that the #defines of the locking byte offsets
    ///* in os_unix.c and os_win.c agree with the WALINDEX_LOCK_OFFSET value.
    ///* For that matter, if the lock offset ever changes from its initial design
    ///* value of 120, we need to know that so there is an assert() to check it.
    /// Allocate an instance of struct Wal to return.
    (*pp_wal = core::ptr::null_mut());
    p_ret =
        unsafe {
                sqlite3_malloc_zero(core::mem::size_of::<Wal>() as u64 +
                        unsafe { (*p_vfs).sz_os_file } as u64)
            } as *mut Wal;
    if (p_ret).is_null() as i32 != 0 { return 7; }
    unsafe { (*p_ret).p_vfs = p_vfs };
    unsafe {
        (*p_ret).p_wal_fd =
            unsafe { &raw mut *p_ret.offset(1 as isize) } as *mut Sqlite3File
    };
    unsafe { (*p_ret).p_db_fd = p_db_fd };
    unsafe { (*p_ret).read_lock = -1 as i16 };
    unsafe { (*p_ret).mx_wal_size = mx_wal_size };
    unsafe { (*p_ret).z_wal_name = z_wal_name };
    unsafe { (*p_ret).sync_header = 1 as u8 };
    unsafe { (*p_ret).pad_to_sector_boundary = 1 as u8 };
    unsafe {
        (*p_ret).exclusive_mode = if b_no_shm != 0 { 2 } else { 0 } as u8
    };

    /// Open file handle on the write-ahead log file.
    (flags = 2 | 4 | 524288);
    rc =
        unsafe {
            sqlite3_os_open(p_vfs, z_wal_name, unsafe { (*p_ret).p_wal_fd },
                flags, &mut flags)
        };
    if rc == 0 && flags & 1 != 0 { unsafe { (*p_ret).read_only = 1 as u8 }; }
    if rc != 0 {
        wal_index_close(unsafe { &*p_ret }, 0);
        unsafe { sqlite3_os_close(unsafe { (*p_ret).p_wal_fd }) };
        unsafe { sqlite3_free(p_ret as *mut ()) };
    } else {
        let i_dc: i32 = unsafe { sqlite3_os_device_characteristics(p_db_fd) };
        if i_dc & 1024 != 0 { unsafe { (*p_ret).sync_header = 0 as u8 }; }
        if i_dc & 4096 != 0 {
            unsafe { (*p_ret).pad_to_sector_boundary = 0 as u8 };
        }
        *pp_wal = p_ret;
    }
    return rc;
}

extern "C" fn wal_lock_exclusive(p_wal_1: &Wal, lock_idx_1: i32, n: i32)
    -> i32 {
    let mut rc: i32 = 0;
    if (*p_wal_1).exclusive_mode != 0 { return 0; }
    rc =
        unsafe {
            sqlite3_os_shm_lock((*p_wal_1).p_db_fd, lock_idx_1, n, 2 | 8)
        };
    return rc;
}

///* Attempt to obtain the exclusive WAL lock defined by parameters lockIdx and
///* n. If the attempt fails and parameter xBusy is not NULL, then it is a
///* busy-handler function. Invoke it and retry the lock until either the
///* lock is successfully obtained or the busy-handler returns 0.
extern "C" fn wal_busy_lock(p_wal_1: *mut Wal,
    x_busy_1: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_busy_arg_1: *mut (), lock_idx_1: i32, n: i32) -> i32 {
    let mut rc: i32 = 0;
    '__b1: loop {
        '__c1: loop {
            rc = wal_lock_exclusive(unsafe { &*p_wal_1 }, lock_idx_1, n);
            break '__c1;
        }
        if !(x_busy_1.is_some() && rc == 5 &&
                        unsafe { x_busy_1.unwrap()(p_busy_arg_1) } != 0) {
            break '__b1;
        }
    }
    return rc;
}

///* Each page of the wal-index mapping contains a hash-table made up of
///* an array of HASHTABLE_NSLOT elements of the following type.
type HtSlot = u16;

///* Obtain a pointer to the iPage'th page of the wal-index. The wal-index
///* is broken into pages of WALINDEX_PGSZ bytes. Wal-index pages are
///* numbered from zero.
///*
///* If the wal-index is currently smaller the iPage pages then the size
///* of the wal-index might be increased, but only if it is safe to do
///* so.  It is safe to enlarge the wal-index if pWal->writeLock is true
///* or pWal->exclusiveMode==WAL_HEAPMEMORY_MODE.
///*
///* Three possible result scenarios:
///*
///*   (1)  rc==SQLITE_OK    and *ppPage==Requested-Wal-Index-Page
///*   (2)  rc>=SQLITE_ERROR and *ppPage==NULL
///*   (3)  rc==SQLITE_OK    and *ppPage==NULL  // only if iPage==0
///*
///* Scenario (3) can only occur when pWal->writeLock is false and iPage==0
#[allow(unused_doc_comments)]
extern "C" fn wal_index_page_realloc(p_wal_1: &mut Wal, i_page_1: i32,
    pp_page_1: &mut *mut u32) -> i32 {
    let mut rc: i32 = 0;
    if (*p_wal_1).n_wi_data <= i_page_1 {
        let n_byte: Sqlite3Int64 =
            (core::mem::size_of::<*mut u32>() as u64 *
                    (1 as i64 + i_page_1 as i64) as u64) as Sqlite3Int64;
        let mut ap_new: *mut *mut u32 = core::ptr::null_mut();
        ap_new =
            unsafe {
                    sqlite3Realloc((*p_wal_1).ap_wi_data as *mut (),
                        n_byte as u64)
                } as *mut *mut u32;
        if (ap_new).is_null() as i32 != 0 {
            *pp_page_1 = core::ptr::null_mut();
            return 7;
        }
        unsafe {
            memset(unsafe {
                        &raw mut *ap_new.offset((*p_wal_1).n_wi_data as isize)
                    } as *mut (), 0,
                core::mem::size_of::<*mut u32>() as u64 *
                    (i_page_1 + 1 - (*p_wal_1).n_wi_data) as u64)
        };
        (*p_wal_1).ap_wi_data = ap_new;
        (*p_wal_1).n_wi_data = i_page_1 + 1;
    }

    /// Request a pointer to the required page from the VFS
    { let _ = 0; };
    if (*p_wal_1).exclusive_mode as i32 == 2 {
        unsafe {
            *(*p_wal_1).ap_wi_data.offset(i_page_1 as isize) =
                unsafe {
                        sqlite3_malloc_zero(core::mem::size_of::<HtSlot>() as u64 *
                                    (4096 * 2) as u64 +
                                4096 as u64 * core::mem::size_of::<u32>() as u64)
                    } as *mut u32
        };
        if (unsafe {
                                *(*p_wal_1).ap_wi_data.offset(i_page_1 as isize)
                            }).is_null() as i32 != 0 {
            rc = 7;
        }
    } else {
        rc =
            unsafe {
                sqlite3_os_shm_map((*p_wal_1).p_db_fd, i_page_1,
                    (core::mem::size_of::<HtSlot>() as u64 * (4096 * 2) as u64 +
                            4096 as u64 * core::mem::size_of::<u32>() as u64) as i32,
                    (*p_wal_1).write_lock as i32,
                    unsafe {
                            &raw mut *(*p_wal_1).ap_wi_data.offset(i_page_1 as isize)
                        } as *mut *mut ())
            };
        { let _ = 0; };
        if rc == 0 {
            if i_page_1 > 0 && unsafe { sqlite3_fault_sim(600) } != 0 {
                rc = 7;
            }
        } else if rc & 255 == 8 {
            (*p_wal_1).read_only |= 2 as u8;
            if rc == 8 { rc = 0; }
        }
    }
    *pp_page_1 = unsafe { *(*p_wal_1).ap_wi_data.offset(i_page_1 as isize) };
    { let _ = 0; };
    return rc;
}

extern "C" fn wal_index_page(p_wal_1: *mut Wal, i_page_1: i32,
    pp_page_1: *mut *mut u32) -> i32 {
    { let _ = 0; };
    if unsafe { (*p_wal_1).n_wi_data } <= i_page_1 ||
            {
                    let __v =
                        unsafe {
                            *unsafe { (*p_wal_1).ap_wi_data.offset(i_page_1 as isize) }
                        };
                    unsafe { *pp_page_1 = __v };
                    __v
                } == core::ptr::null_mut() {
        return wal_index_page_realloc(unsafe { &mut *p_wal_1 }, i_page_1,
                unsafe { &mut *pp_page_1 });
    }
    return 0;
}

///* Return a pointer to the WalIndexHdr structure in the wal-index.
extern "C" fn wal_index_hdr(p_wal_1: &Wal) -> *mut WalIndexHdr {
    { let _ = 0; };
    { let _ = 0; };
    return unsafe { *(*p_wal_1).ap_wi_data.offset(0 as isize) } as
            *mut WalIndexHdr;
}

///* If there is the possibility of concurrent access to the SHM file
///* from multiple threads and/or processes, then do a memory barrier.
extern "C" fn wal_shm_barrier(p_wal_1: &Wal) -> () {
    if (*p_wal_1).exclusive_mode as i32 != 2 {
        unsafe { sqlite3_os_shm_barrier((*p_wal_1).p_db_fd) };
    }
}

///* Generate or extend an 8 byte checksum based on the data in
///* array aByte[] and the initial values of aIn[0] and aIn[1] (or
///* initial values of 0 and 0 if aIn==NULL).
///*
///* The checksum is written back into aOut[] before returning.
///*
///* nByte must be a positive multiple of 8.
#[allow(unused_doc_comments)]
extern "C" fn wal_checksum_bytes(native_cksum_1: i32, a: *mut u8,
    n_byte_1: i32, a_in_1: *const u32, a_out_1: *mut u32) -> () {
    let mut s1: u32 = 0 as u32;
    let mut s2: u32 = 0 as u32;
    let mut a_data: *mut u32 = a as *mut u32;
    let a_end: *mut u32 =
        unsafe { &raw mut *a.offset(n_byte_1 as isize) } as *mut u32;
    if !(a_in_1).is_null() {
        s1 = unsafe { *a_in_1.offset(0 as isize) } as u32;
        s2 = unsafe { *a_in_1.offset(1 as isize) } as u32;
    } else { s1 = { s2 = 0 as u32; s2 }; }

    /// nByte is a multiple of 8 between 8 and 65536
    { let _ = 0; };
    if (native_cksum_1 == 0) as i32 != 0 {
        '__b2: loop {
            '__c2: loop {
                s1 +=
                    ((unsafe { *a_data.offset(0 as isize) } & 255 as u32) << 24)
                                    +
                                    ((unsafe { *a_data.offset(0 as isize) } & 65280 as u32) <<
                                        8) +
                                ((unsafe { *a_data.offset(0 as isize) } & 16711680 as u32)
                                    >> 8) +
                            ((unsafe { *a_data.offset(0 as isize) } & 4278190080u32) >>
                                24) + s2;
                s2 +=
                    ((unsafe { *a_data.offset(1 as isize) } & 255 as u32) << 24)
                                    +
                                    ((unsafe { *a_data.offset(1 as isize) } & 65280 as u32) <<
                                        8) +
                                ((unsafe { *a_data.offset(1 as isize) } & 16711680 as u32)
                                    >> 8) +
                            ((unsafe { *a_data.offset(1 as isize) } & 4278190080u32) >>
                                24) + s1;
                {
                    let __n = 2;
                    let __p = &mut a_data;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                break '__c2;
            }
            if !(a_data < a_end) { break '__b2; }
        }
    } else if n_byte_1 % 64 == 0 {
        '__b3: loop {
            '__c3: loop {
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                break '__c3;
            }
            if !(a_data < a_end) { break '__b3; }
        }
    } else {
        '__b4: loop {
            '__c4: loop {
                s1 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s2;
                s2 +=
                    unsafe {
                            *{
                                    let __p = &mut a_data;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } + s1;
                break '__c4;
            }
            if !(a_data < a_end) { break '__b4; }
        }
    }
    { let _ = 0; };
    unsafe { *a_out_1.offset(0 as isize) = s1 };
    unsafe { *a_out_1.offset(1 as isize) = s2 };
}

///* Try to read the wal-index header.  Return 0 on success and 1 if
///* there is a problem.
///*
///* The wal-index is in shared memory.  Another thread or process might
///* be writing the header at the same time this procedure is trying to
///* read it, which might result in inconsistency.  A dirty read is detected
///* by verifying that both copies of the header are the same and also by
///* a checksum on the header.
///*
///* If and only if the read is consistent and the header is different from
///* pWal->hdr, then pWal->hdr is updated to the content of the new header
///* and *pChanged is set to 1.
///*
///* If the checksum cannot be verified return non-zero. If the header
///* is read successfully and the checksum verified, return zero.
#[allow(unused_doc_comments)]
extern "C" fn wal_index_try_hdr(p_wal_1: *mut Wal, p_changed_1: &mut i32)
    -> i32 {
    let mut a_cksum: [u32; 2] = [0; 2];
    /// Checksum on the header content
    let mut h1: WalIndexHdr = unsafe { core::mem::zeroed() };
    let mut h2: WalIndexHdr = unsafe { core::mem::zeroed() };
    /// Two copies of the header content
    let mut a_hdr: *mut WalIndexHdr = core::ptr::null_mut();

    /// Header in shared memory
    /// The first page of the wal-index must be mapped at this point.
    { let _ = 0; };

    /// Read the header. This might happen concurrently with a write to the
    ///* same area of shared memory on a different CPU in a SMP,
    ///* meaning it is possible that an inconsistent snapshot is read
    ///* from the file. If this happens, return non-zero.
    ///*
    ///* tag-20200519-1:
    ///* There are two copies of the header at the beginning of the wal-index.
    ///* When reading, read [0] first then [1].  Writes are in the reverse order.
    ///* Memory barriers are used to prevent the compiler or the hardware from
    ///* reordering the reads and writes.  TSAN and similar tools can sometimes
    ///* give false-positive warnings about these accesses because the tools do not
    ///* account for the double-read and the memory barrier. The use of mutexes
    ///* here would be problematic as the memory being accessed is potentially
    ///* shared among multiple processes and not all mutex implementations work
    ///* reliably in that environment.
    (a_hdr = wal_index_hdr(unsafe { &*p_wal_1 }));
    unsafe {
        memcpy(&raw mut h1 as *mut (),
            unsafe { &raw mut *a_hdr.offset(0 as isize) } as *mut () as
                *const (), core::mem::size_of::<WalIndexHdr>() as u64)
    };

    /// Possible TSAN false-positive
    wal_shm_barrier(unsafe { &*p_wal_1 });
    unsafe {
        memcpy(&raw mut h2 as *mut (),
            unsafe { &raw mut *a_hdr.offset(1 as isize) } as *mut () as
                *const (), core::mem::size_of::<WalIndexHdr>() as u64)
    };
    if unsafe {
                memcmp(&raw mut h1 as *const (), &raw mut h2 as *const (),
                    core::mem::size_of::<WalIndexHdr>() as u64)
            } != 0 {
        return 1;
    }
    if h1.is_init as i32 == 0 { return 1; }
    wal_checksum_bytes(1, &raw mut h1 as *mut u8,
        (core::mem::size_of::<WalIndexHdr>() as u64 -
                core::mem::size_of::<[u32; 2]>() as u64) as i32,
        core::ptr::null(), &raw mut a_cksum[0 as usize] as *mut u32);
    if a_cksum[0 as usize] != h1.a_cksum[0 as usize] ||
            a_cksum[1 as usize] != h1.a_cksum[1 as usize] {
        return 1;
    }
    if unsafe {
                memcmp(unsafe { &raw mut (*p_wal_1).hdr } as *const (),
                    &raw mut h1 as *const (),
                    core::mem::size_of::<WalIndexHdr>() as u64)
            } != 0 {
        *p_changed_1 = 1;
        unsafe {
            memcpy(unsafe { &raw mut (*p_wal_1).hdr } as *mut (),
                &raw mut h1 as *const (),
                core::mem::size_of::<WalIndexHdr>() as u64)
        };
        unsafe {
            (*p_wal_1).sz_page =
                ((unsafe { (*p_wal_1).hdr.sz_page } as i32 & 65024) +
                        ((unsafe { (*p_wal_1).hdr.sz_page } as i32 & 1) << 16)) as
                    u32
        };
    }

    /// The header was successfully read. Return zero.
    return 0;
}

///* Set or release locks on the WAL.  Locks are either shared or exclusive.
///* A lock cannot be moved directly between shared and exclusive - it must go
///* through the unlocked state first.
///*
///* In locking_mode=EXCLUSIVE, all of these routines become no-ops.
extern "C" fn wal_lock_shared(p_wal_1: &Wal, lock_idx_1: i32) -> i32 {
    let mut rc: i32 = 0;
    if (*p_wal_1).exclusive_mode != 0 { return 0; }
    rc =
        unsafe {
            sqlite3_os_shm_lock((*p_wal_1).p_db_fd, lock_idx_1, 1, 2 | 4)
        };
    return rc;
}

extern "C" fn wal_unlock_shared(p_wal_1: &Wal, lock_idx_1: i32) -> () {
    if (*p_wal_1).exclusive_mode != 0 { return; }
    {
        let _ =
            unsafe {
                sqlite3_os_shm_lock((*p_wal_1).p_db_fd, lock_idx_1, 1, 1 | 4)
            };
    };
}

///* A copy of the following object occurs in the wal-index immediately
///* following the second copy of the WalIndexHdr.  This object stores
///* information used by checkpoint.
///*
///* nBackfill is the number of frames in the WAL that have been written
///* back into the database. (We call the act of moving content from WAL to
///* database "backfilling".)  The nBackfill number is never greater than
///* WalIndexHdr.mxFrame.  nBackfill can only be increased by threads
///* holding the WAL_CKPT_LOCK lock (which includes a recovery thread).
///* However, a WAL_WRITE_LOCK thread can move the value of nBackfill from
///* mxFrame back to zero when the WAL is reset.
///*
///* nBackfillAttempted is the largest value of nBackfill that a checkpoint
///* has attempted to achieve.  Normally nBackfill==nBackfillAtempted, however
///* the nBackfillAttempted is set before any backfilling is done and the
///* nBackfill is only set after all backfilling completes.  So if a checkpoint
///* crashes, nBackfillAttempted might be larger than nBackfill.  The
///* WalIndexHdr.mxFrame must never be less than nBackfillAttempted.
///*
///* The aLock[] field is a set of bytes used for locking.  These bytes should
///* never be read or written.
///*
///* There is one entry in aReadMark[] for each reader lock.  If a reader
///* holds read-lock K, then the value in aReadMark[K] is no greater than
///* the mxFrame for that reader.  The value READMARK_NOT_USED (0xffffffff)
///* for any aReadMark[] means that entry is unused.  aReadMark[0] is
///* a special case; its value is never used and it exists as a place-holder
///* to avoid having to offset aReadMark[] indexes by one.  Readers holding
///* WAL_READ_LOCK(0) always ignore the entire WAL and read all content
///* directly from the database.
///*
///* The value of aReadMark[K] may only be changed by a thread that
///* is holding an exclusive lock on WAL_READ_LOCK(K).  Thus, the value of
///* aReadMark[K] cannot changed while there is a reader is using that mark
///* since the reader will be holding a shared lock on WAL_READ_LOCK(K).
///*
///* The checkpointer may only transfer frames from WAL to database where
///* the frame numbers are less than or equal to every aReadMark[] that is
///* in use (that is, every aReadMark[j] for which there is a corresponding
///* WAL_READ_LOCK(j)).  New readers (usually) pick the aReadMark[] with the
///* largest value and will increase an unused aReadMark[] to mxFrame if there
///* is not already an aReadMark[] equal to mxFrame.  The exception to the
///* previous sentence is when nBackfill equals mxFrame (meaning that everything
///* in the WAL has been backfilled into the database) then new readers
///* will choose aReadMark[0] which has value 0 and hence such reader will
///* get all their all content directly from the database file and ignore
///* the WAL.
///*
///* Writers normally append new frames to the end of the WAL.  However,
///* if nBackfill equals mxFrame (meaning that all WAL content has been
///* written back into the database) and if no readers are using the WAL
///* (in other words, if there are no WAL_READ_LOCK(i) where i>0) then
///* the writer will first "reset" the WAL back to the beginning and start
///* writing new content beginning at frame 1.
///*
///* We assume that 32-bit loads are atomic and so no locks are needed in
///* order to read from any aReadMark[] entries.
#[repr(C)]
#[derive(Copy, Clone)]
struct WalCkptInfo {
    n_backfill: u32,
    a_read_mark: [u32; 5],
    a_lock: [u8; 8],
    n_backfill_attempted: u32,
    not_used0: u32,
}

///* Write the header information in pWal->hdr into the wal-index.
///*
///* The checksum on pWal->hdr is updated before it is written.
#[allow(unused_doc_comments)]
extern "C" fn wal_index_write_hdr(p_wal_1: *mut Wal) -> () {
    let a_hdr: *mut WalIndexHdr = wal_index_hdr(unsafe { &*p_wal_1 });
    let n_cksum: i32 = core::mem::offset_of!(WalIndexHdr, a_cksum) as i32;
    { let _ = 0; };
    unsafe { (*p_wal_1).hdr.is_init = 1 as u8 };
    unsafe { (*p_wal_1).hdr.i_version = 3007000 as u32 };
    wal_checksum_bytes(1, unsafe { &raw mut (*p_wal_1).hdr } as *mut u8,
        n_cksum, core::ptr::null(),
        unsafe { &raw mut (*p_wal_1).hdr.a_cksum[0 as usize] } as *mut u32);

    /// Possible TSAN false-positive.  See tag-20200519-1
    unsafe {
        memcpy(unsafe { &raw mut *a_hdr.offset(1 as isize) } as *mut (),
            unsafe { &raw mut (*p_wal_1).hdr } as *const (),
            core::mem::size_of::<WalIndexHdr>() as u64)
    };
    wal_shm_barrier(unsafe { &*p_wal_1 });
    unsafe {
        memcpy(unsafe { &raw mut *a_hdr.offset(0 as isize) } as *mut (),
            unsafe { &raw mut (*p_wal_1).hdr } as *const (),
            core::mem::size_of::<WalIndexHdr>() as u64)
    };
}

///* Return a pointer to the WalCkptInfo structure in the wal-index.
extern "C" fn wal_ckpt_info(p_wal_1: &Wal) -> *mut WalCkptInfo {
    { let _ = 0; };
    { let _ = 0; };
    return unsafe {
                &raw mut *unsafe {
                            (*(*p_wal_1).ap_wi_data.offset(0 as
                                            isize)).add((core::mem::size_of::<WalIndexHdr>() as u64 /
                                        2 as u64) as usize)
                        }
            } as *mut WalCkptInfo;
}

extern "C" fn wal_unlock_exclusive(p_wal_1: &Wal, lock_idx_1: i32, n: i32)
    -> () {
    if (*p_wal_1).exclusive_mode != 0 { return; }
    {
        let _ =
            unsafe {
                sqlite3_os_shm_lock((*p_wal_1).p_db_fd, lock_idx_1, n, 1 | 8)
            };
    };
}

///* Return the number of the wal-index page that contains the hash-table
///* and page-number array that contain entries corresponding to WAL frame
///* iFrame. The wal-index is broken up into 32KB pages. Wal-index pages
///* are numbered starting from 0.
extern "C" fn wal_frame_page(i_frame_1: u32) -> i32 {
    let i_hash: i32 =
        (((i_frame_1 + 4096 as u32) as u64 -
                        (4096 as u64 -
                            (core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                    core::mem::size_of::<WalCkptInfo>() as u64) /
                                core::mem::size_of::<u32>() as u64) - 1 as u64) /
                4096 as u64) as i32;
    { let _ = 0; };
    { let _ = 0; };
    return i_hash;
}

///* Check to see if the frame with header in aFrame[] and content
///* in aData[] is valid.  If it is a valid frame, fill *piPage and
///* *pnTruncate and return true.  Return if the frame is not valid.
#[allow(unused_doc_comments)]
extern "C" fn wal_decode_frame(p_wal_1: &mut Wal, pi_page_1: &mut u32,
    pn_truncate_1: &mut u32, a_data_1: *mut u8, a_frame_1: *mut u8) -> i32 {
    let mut native_cksum: i32 = 0;
    /// True for native byte-order checksums
    let a_cksum: *mut u32 =
        &raw mut (*p_wal_1).hdr.a_frame_cksum[0 as usize] as *mut u32;
    let mut pgno: u32 = 0 as u32;

    /// Page number of the frame
    { let _ = 0; };
    if unsafe {
                memcmp(&raw mut (*p_wal_1).hdr.a_salt as *const (),
                    unsafe { &raw mut *a_frame_1.offset(8 as isize) } as
                        *const (), 8 as u64)
            } != 0 {
        return 0;
    }

    /// A frame is only valid if the page number is greater than zero.
    (pgno =
        unsafe {
            sqlite3_get4byte(unsafe { &raw mut *a_frame_1.offset(0 as isize) }
                    as *const u8)
        });
    if pgno == 0 as u32 { return 0; }
    if ((*p_wal_1).sz_page == 0) as i32 != 0 { return 0; }

    /// A frame is only valid if a checksum of the WAL header,
    ///* all prior frames, the first 16 bytes of this frame-header,
    ///* and the frame-data matches the checksum in the last 8
    ///* bytes of this frame-header.
    (native_cksum = ((*p_wal_1).hdr.big_end_cksum as i32 == 0) as i32);
    wal_checksum_bytes(native_cksum, a_frame_1, 8, a_cksum as *const u32,
        a_cksum);
    wal_checksum_bytes(native_cksum, a_data_1, (*p_wal_1).sz_page as i32,
        a_cksum as *const u32, a_cksum);
    if unsafe { *a_cksum.offset(0 as isize) } !=
                unsafe {
                    sqlite3_get4byte(unsafe {
                                &raw mut *a_frame_1.offset(16 as isize)
                            } as *const u8)
                } ||
            unsafe { *a_cksum.offset(1 as isize) } !=
                unsafe {
                    sqlite3_get4byte(unsafe {
                                &raw mut *a_frame_1.offset(20 as isize)
                            } as *const u8)
                } {

        /// Checksum failed.
        return 0;
    }

    /// If we reach this point, the frame is valid.  Return the page number
    ///* and the new database size.
    (*pi_page_1 = pgno);
    *pn_truncate_1 =
        unsafe {
            sqlite3_get4byte(unsafe { &raw mut *a_frame_1.offset(4 as isize) }
                    as *const u8)
        };
    return 1;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WalHashLoc {
    a_hash: *mut HtSlot,
    a_pgno: *mut u32,
    i_zero: u32,
}

///* Return pointers to the hash table and page number array stored on
///* page iHash of the wal-index. The wal-index is broken into 32KB pages
///* numbered starting from 0.
///*
///* Set output variable pLoc->aHash to point to the start of the hash table
///* in the wal-index file. Set pLoc->iZero to one less than the frame
///* number of the first frame indexed by this hash table. If a
///* slot in the hash table is set to N, it refers to frame number
///* (pLoc->iZero+N) in the log.
///*
///* Finally, set pLoc->aPgno so that pLoc->aPgno[0] is the page number of the
///* first frame indexed by the hash table, frame (pLoc->iZero).
#[allow(unused_doc_comments)]
extern "C" fn wal_hash_get(p_wal_1: *mut Wal, i_hash_1: i32,
    p_loc_1: &mut WalHashLoc) -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    (rc = wal_index_page(p_wal_1, i_hash_1, &mut (*p_loc_1).a_pgno));
    { let _ = 0; };
    if !((*p_loc_1).a_pgno).is_null() {
        (*p_loc_1).a_hash =
            unsafe { &raw mut *(*p_loc_1).a_pgno.offset(4096 as isize) } as
                *mut HtSlot;
        if i_hash_1 == 0 {
            (*p_loc_1).a_pgno =
                unsafe {
                    (*p_loc_1).a_pgno.add(((core::mem::size_of::<WalIndexHdr>()
                                            as u64 * 2 as u64 +
                                    core::mem::size_of::<WalCkptInfo>() as u64) /
                                core::mem::size_of::<u32>() as u64) as usize)
                };
            (*p_loc_1).i_zero = 0 as u32;
        } else {
            (*p_loc_1).i_zero =
                (4096 as u64 -
                            (core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                    core::mem::size_of::<WalCkptInfo>() as u64) /
                                core::mem::size_of::<u32>() as u64 +
                        ((i_hash_1 - 1) * 4096) as u64) as u32;
        }
    } else if rc == 0 { rc = 1; }
    return rc;
}

///* Remove entries from the hash table that point to WAL slots greater
///* than pWal->hdr.mxFrame.
///*
///* This function is called whenever pWal->hdr.mxFrame is decreased due
///* to a rollback or savepoint.
///*
///* At most only the hash table containing pWal->hdr.mxFrame needs to be
///* updated.  Any later hash tables will be automatically cleared when
///* pWal->hdr.mxFrame advances to the point where those hash tables are
///* actually needed.
#[allow(unused_doc_comments)]
extern "C" fn wal_cleanup_hash(p_wal_1: *mut Wal) -> () {
    let mut s_loc: WalHashLoc = unsafe { core::mem::zeroed() };
    /// Hash table location
    let mut i_limit: i32 = 0;
    /// Zero values greater than this
    let mut n_byte: i32 = 0;
    /// Number of bytes to zero in aPgno[]
    let mut i: i32 = 0;

    /// Used to iterate through aHash[]
    { let _ = 0; };
    if unsafe { (*p_wal_1).hdr.mx_frame } == 0 as u32 { return; }

    /// Obtain pointers to the hash-table and page-number array containing
    ///* the entry that corresponds to frame pWal->hdr.mxFrame. It is guaranteed
    ///* that the page said hash-table and array reside on is already mapped.(1)
    { let _ = 0; };
    { let _ = 0; };
    i =
        wal_hash_get(p_wal_1,
            wal_frame_page(unsafe { (*p_wal_1).hdr.mx_frame }), &mut s_loc);
    if i != 0 { return; }

    /// Defense-in-depth, in case (1) above is wrong
    /// Zero all hash-table entries that correspond to frame numbers greater
    ///* than pWal->hdr.mxFrame.
    (i_limit = (unsafe { (*p_wal_1).hdr.mx_frame } - s_loc.i_zero) as i32);
    { let _ = 0; };
    {
        i = 0;
        '__b5: loop {
            if !(i < 4096 * 2) { break '__b5; }
            '__c5: loop {
                if unsafe { *s_loc.a_hash.offset(i as isize) } as i32 >
                        i_limit {
                    unsafe { *s_loc.a_hash.offset(i as isize) = 0 as HtSlot };
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }

    /// Zero the entries in the aPgno array that correspond to frames with
    ///* frame numbers greater than pWal->hdr.mxFrame.
    (n_byte =
        unsafe {
                    (s_loc.a_hash as
                            *mut i8).offset_from(unsafe {
                                &raw mut *s_loc.a_pgno.offset(i_limit as isize)
                            } as *mut i8)
                } as i64 as i32);

    /// Zero the entries in the aPgno array that correspond to frames with
    ///* frame numbers greater than pWal->hdr.mxFrame.
    { let _ = 0; };
    unsafe {
        memset(unsafe { &raw mut *s_loc.a_pgno.offset(i_limit as isize) } as
                *mut (), 0, n_byte as u64)
    };
}

///* Compute a hash on a page number.  The resulting hash value must land
///* between 0 and (HASHTABLE_NSLOT-1).  The walNextHash() function advances
///* the hash to the next value in the event of a collision.
extern "C" fn wal_hash(i_page_1: u32) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    return (i_page_1 * 383 as u32 & (4096 * 2 - 1) as u32) as i32;
}

extern "C" fn wal_next_hash(i_prior_hash_1: i32) -> i32 {
    return i_prior_hash_1 + 1 & 4096 * 2 - 1;
}

///* Set an entry in the wal-index that will map database page number
///* pPage into WAL frame iFrame.
#[allow(unused_doc_comments)]
extern "C" fn wal_index_append(p_wal_1: *mut Wal, i_frame_1: u32,
    i_page_1: u32) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut s_loc: WalHashLoc = unsafe { core::mem::zeroed() };

    /// Wal-index hash table location
    (rc = wal_hash_get(p_wal_1, wal_frame_page(i_frame_1), &mut s_loc));
    if rc == 0 {
        let mut i_key: i32 = 0;
        /// Hash table key
        let mut idx: i32 = 0;
        /// Value to write to hash-table slot
        let mut n_collide: i32 = 0;

        /// Number of hash collisions
        (idx = (i_frame_1 - s_loc.i_zero) as i32);
        { let _ = 0; };
        if idx == 1 {
            let n_byte: i32 =
                unsafe {
                            (unsafe {
                                        &raw mut *s_loc.a_hash.offset((4096 * 2) as isize)
                                    } as *mut u8).offset_from(s_loc.a_pgno as *mut u8)
                        } as i64 as i32;
            { let _ = 0; };
            unsafe { memset(s_loc.a_pgno as *mut (), 0, n_byte as u64) };
        }
        if unsafe { *s_loc.a_pgno.offset((idx - 1) as isize) } != 0 {
            wal_cleanup_hash(p_wal_1);
            { let _ = 0; };
        }

        /// Write the aPgno[] array entry and the hash-table slot.
        (n_collide = idx);
        {
            i_key = wal_hash(i_page_1);
            '__b6: loop {
                if !(unsafe { *s_loc.a_hash.offset(i_key as isize) } != 0) {
                    break '__b6;
                }
                '__c6: loop {
                    if {
                                let __p = &mut n_collide;
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            } == 0 {
                        return unsafe { sqlite3_corrupt_error(1341) };
                    }
                    break '__c6;
                }
                i_key = wal_next_hash(i_key);
            }
        }
        unsafe {
            *s_loc.a_pgno.offset((idx - 1 & 4096 - 1) as isize) = i_page_1
        };
        unsafe {
            std::sync::atomic::AtomicU16::from_ptr(unsafe {
                            &raw mut *s_loc.a_hash.offset(i_key as isize)
                        } as
                        *mut u16).store(idx as HtSlot as u16,
                std::sync::atomic::Ordering::Relaxed)
        };
    }
    return rc;
}

///* Recover the wal-index by reading the write-ahead log file.
///*
///* This routine first tries to establish an exclusive lock on the
///* wal-index to prevent other threads/processes from doing anything
///* with the WAL or wal-index while recovery is running.  The
///* WAL_RECOVER_LOCK is also held so that other threads will know
///* that this thread is running recovery.  If unable to establish
///* the necessary locks, this routine returns SQLITE_BUSY.
#[allow(unused_doc_comments)]
extern "C" fn wal_index_recover(p_wal_1: *mut Wal) -> i32 {
    let mut rc: i32 = 0;
    /// Return Code
    let mut n_size: i64 = 0 as i64;
    /// Size of log file
    let mut a_frame_cksum: [u32; 2] = [0 as u32, 0 as u32];
    let mut i_lock: i32 = 0;
    /// Lock offset to lock for checkpoint
    /// Obtain an exclusive lock on all byte in the locking range not already
    ///* locked by the caller. The caller is guaranteed to have locked the
    ///* WAL_WRITE_LOCK byte, and may have also locked the WAL_CKPT_LOCK byte.
    ///* If successful, the same bytes that are locked here are unlocked before
    ///* this function returns.
    let mut a_buf: [u8; 32] = [0; 32];
    /// Buffer to load WAL header into
    let mut a_private: *mut u32 = core::ptr::null_mut();
    /// Heap copy of *-shm hash being populated
    let mut a_frame: *mut u8 = core::ptr::null_mut();
    /// Malloc'd buffer to load entire frame
    let mut sz_frame: i32 = 0;
    /// Number of bytes in buffer aFrame[]
    let mut a_data: *mut u8 = core::ptr::null_mut();
    /// Pointer to data part of aFrame buffer
    let mut sz_page: i32 = 0;
    /// Page size according to the log
    let mut magic: u32 = 0 as u32;
    /// Magic value read from WAL header
    let mut version: u32 = 0 as u32;
    /// Magic value read from WAL header
    let mut is_valid: i32 = 0;
    /// True if this frame is valid
    let mut i_pg: u32 = 0 as u32;
    /// Current 32KB wal-index page
    let mut i_last_frame: u32 = 0 as u32;
    /// Last frame in wal, based on nSize alone
    /// Read in the WAL header.
    /// If the database page size is not a power of two, or is greater than
    ///* SQLITE_MAX_PAGE_SIZE, conclude that the WAL file contains no valid
    ///* data. Similarly, if the 'magic' value is invalid, ignore the whole
    ///* WAL file.
    /// Verify that the WAL header checksum is correct
    /// Verify that the version number on the WAL format is one that
    ///* are able to understand
    /// Malloc a buffer to read frames into.
    /// Read all frames from the log file.
    let mut a_share: *mut u32 = core::ptr::null_mut();
    let mut i_frame: u32 = 0 as u32;
    /// Index of last frame read
    let mut i_last: u32 = 0 as u32;
    let mut i_first: u32 = 0 as u32;
    let mut n_hdr: u32 = 0 as u32;
    let mut n_hdr32: u32 = 0 as u32;
    let mut i_offset: i64 = 0 as i64;
    let mut pgno: u32 = 0 as u32;
    /// Database page number for frame
    let mut n_truncate: u32 = 0 as u32;
    /// dbsize field from frame header
    /// Read and decode the next log frame.
    /// If nTruncate is non-zero, this is a commit record.
    /// Memcpy() should work fine here, on all reasonable implementations.
    ///* Technically, memcpy() might change the destination to some
    ///* intermediate value before setting to the final value, and that might
    ///* cause a concurrent reader to malfunction.  Memcpy() is allowed to
    ///* do that, according to the spec, but no memcpy() implementation that
    ///* we know of actually does that, which is why we say that memcpy()
    ///* is safe for this.  Memcpy() is certainly a lot faster.
    let mut p_info: *mut WalCkptInfo = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s8:
            {
            match __state {
                0 => { __state = 4; }
                2 => { if rc == 0 { __state = 109; } else { __state = 108; } }
                3 => { __state = 133; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { { let _ = 0; }; __state = 8; }
                8 => { { let _ = 0; }; __state = 9; }
                9 => { { let _ = 0; }; __state = 10; }
                10 => { { let _ = 0; }; __state = 11; }
                11 => {
                    i_lock = 1 + unsafe { (*p_wal_1).ckpt_lock } as i32;
                    __state = 12;
                }
                12 => {
                    rc =
                        wal_lock_exclusive(unsafe { &*p_wal_1 }, i_lock,
                            3 + 0 - i_lock);
                    __state = 13;
                }
                13 => { if rc != 0 { __state = 15; } else { __state = 14; } }
                14 => { __state = 16; }
                15 => { return rc; }
                16 => {
                    unsafe {
                        memset(unsafe { &raw mut (*p_wal_1).hdr } as *mut (), 0,
                            core::mem::size_of::<WalIndexHdr>() as u64)
                    };
                    __state = 17;
                }
                17 => {
                    rc =
                        unsafe {
                            sqlite3_os_file_size(unsafe { (*p_wal_1).p_wal_fd },
                                &mut n_size)
                        };
                    __state = 18;
                }
                18 => { if rc != 0 { __state = 20; } else { __state = 19; } }
                19 => {
                    if n_size > 32 as i64 {
                        __state = 22;
                    } else { __state = 21; }
                }
                20 => { __state = 3; }
                21 => { __state = 2; }
                22 => { __state = 23; }
                23 => { a_private = core::ptr::null_mut(); __state = 24; }
                24 => { a_frame = core::ptr::null_mut(); __state = 25; }
                25 => { __state = 26; }
                26 => { __state = 27; }
                27 => { __state = 28; }
                28 => { __state = 29; }
                29 => { __state = 30; }
                30 => { __state = 31; }
                31 => { __state = 32; }
                32 => { __state = 33; }
                33 => {
                    rc =
                        unsafe {
                            sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd },
                                &raw mut a_buf[0 as usize] as *mut u8 as *mut (), 32,
                                0 as i64)
                        };
                    __state = 34;
                }
                34 => { if rc != 0 { __state = 36; } else { __state = 35; } }
                35 => {
                    magic =
                        unsafe {
                            sqlite3_get4byte(&raw mut a_buf[0 as usize] as *const u8)
                        };
                    __state = 37;
                }
                36 => { __state = 3; }
                37 => {
                    sz_page =
                        unsafe {
                                sqlite3_get4byte(&raw mut a_buf[8 as usize] as *const u8)
                            } as i32;
                    __state = 38;
                }
                38 => {
                    if magic & 4294967294u32 != 931071618 as u32 ||
                                    sz_page & sz_page - 1 != 0 || sz_page > 65536 ||
                            sz_page < 512 {
                        __state = 40;
                    } else { __state = 39; }
                }
                39 => {
                    unsafe {
                        (*p_wal_1).hdr.big_end_cksum = (magic & 1 as u32) as u8
                    };
                    __state = 41;
                }
                40 => { __state = 2; }
                41 => {
                    unsafe { (*p_wal_1).sz_page = sz_page as u32 };
                    __state = 42;
                }
                42 => {
                    unsafe {
                        (*p_wal_1).n_ckpt =
                            unsafe {
                                sqlite3_get4byte(&raw mut a_buf[12 as usize] as *const u8)
                            }
                    };
                    __state = 43;
                }
                43 => {
                    unsafe {
                        memcpy(unsafe { &raw mut (*p_wal_1).hdr.a_salt } as *mut (),
                            &raw mut a_buf[16 as usize] as *const (), 8 as u64)
                    };
                    __state = 44;
                }
                44 => {
                    wal_checksum_bytes((unsafe { (*p_wal_1).hdr.big_end_cksum }
                                    as i32 == 0) as i32, &raw mut a_buf[0 as usize] as *mut u8,
                        32 - 2 * 4, core::ptr::null(),
                        unsafe { &raw mut (*p_wal_1).hdr.a_frame_cksum[0 as usize] }
                            as *mut u32);
                    __state = 45;
                }
                45 => {
                    if unsafe { (*p_wal_1).hdr.a_frame_cksum[0 as usize] } !=
                                unsafe {
                                    sqlite3_get4byte(&raw mut a_buf[24 as usize] as *const u8)
                                } ||
                            unsafe { (*p_wal_1).hdr.a_frame_cksum[1 as usize] } !=
                                unsafe {
                                    sqlite3_get4byte(&raw mut a_buf[28 as usize] as *const u8)
                                } {
                        __state = 47;
                    } else { __state = 46; }
                }
                46 => {
                    version =
                        unsafe {
                            sqlite3_get4byte(&raw mut a_buf[4 as usize] as *const u8)
                        };
                    __state = 48;
                }
                47 => { __state = 2; }
                48 => {
                    if version != 3007000 as u32 {
                        __state = 50;
                    } else { __state = 49; }
                }
                49 => { sz_frame = sz_page + 24; __state = 52; }
                50 => {
                    rc = unsafe { sqlite3_cantopen_error(1473) };
                    __state = 51;
                }
                51 => { __state = 2; }
                52 => {
                    a_frame =
                        unsafe {
                                sqlite3_malloc64(sz_frame as u64 +
                                        (core::mem::size_of::<HtSlot>() as u64 * (4096 * 2) as u64 +
                                            4096 as u64 * core::mem::size_of::<u32>() as u64))
                            } as *mut u8;
                    __state = 53;
                }
                53 => { __state = 54; }
                54 => {
                    if (a_frame).is_null() as i32 != 0 {
                        __state = 56;
                    } else { __state = 55; }
                }
                55 => {
                    a_data = unsafe { a_frame.offset(24 as isize) };
                    __state = 58;
                }
                56 => { rc = 7; __state = 57; }
                57 => { __state = 3; }
                58 => {
                    a_private =
                        unsafe { &raw mut *a_data.offset(sz_page as isize) } as
                            *mut u32;
                    __state = 59;
                }
                59 => {
                    i_last_frame =
                        ((n_size - 32 as i64) / sz_frame as i64) as u32;
                    __state = 60;
                }
                60 => { i_pg = 0 as u32; __state = 62; }
                61 => { __state = 107; }
                62 => {
                    if i_pg <= wal_frame_page(i_last_frame) as u32 {
                        __state = 63;
                    } else { __state = 61; }
                }
                63 => { __state = 65; }
                64 => {
                    { let __p = &mut i_pg; let __t = *__p; *__p += 1; __t };
                    __state = 62;
                }
                65 => { __state = 66; }
                66 => {
                    i_last =
                        if (i_last_frame as u64) <
                                    4096 as u64 -
                                            (core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                                    core::mem::size_of::<WalCkptInfo>() as u64) /
                                                core::mem::size_of::<u32>() as u64 +
                                        (i_pg * 4096 as u32) as u64 {
                                i_last_frame as u64
                            } else {
                                4096 as u64 -
                                        (core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                                core::mem::size_of::<WalCkptInfo>() as u64) /
                                            core::mem::size_of::<u32>() as u64 +
                                    (i_pg * 4096 as u32) as u64
                            } as u32;
                    __state = 67;
                }
                67 => {
                    i_first =
                        (1 as u64 +
                                if i_pg == 0 as u32 {
                                    0 as u64
                                } else {
                                    4096 as u64 -
                                            (core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                                    core::mem::size_of::<WalCkptInfo>() as u64) /
                                                core::mem::size_of::<u32>() as u64 +
                                        ((i_pg - 1 as u32) * 4096 as u32) as u64
                                }) as u32;
                    __state = 68;
                }
                68 => { __state = 69; }
                69 => {
                    rc =
                        wal_index_page(p_wal_1, i_pg as i32,
                            &raw mut a_share as *mut *mut u32);
                    __state = 70;
                }
                70 => { { let _ = 0; }; __state = 71; }
                71 => {
                    if a_share == core::ptr::null_mut() {
                        __state = 73;
                    } else { __state = 72; }
                }
                72 => { __state = 74; }
                73 => { __state = 61; }
                74 => {
                    unsafe {
                        *unsafe { (*p_wal_1).ap_wi_data.add(i_pg as usize) } =
                            a_private
                    };
                    __state = 75;
                }
                75 => { i_frame = i_first; __state = 77; }
                76 => {
                    unsafe {
                        *unsafe { (*p_wal_1).ap_wi_data.add(i_pg as usize) } =
                            a_share
                    };
                    __state = 99;
                }
                77 => {
                    if i_frame <= i_last {
                        __state = 78;
                    } else { __state = 76; }
                }
                78 => {
                    i_offset =
                        32 as i64 +
                            (i_frame - 1 as u32) as i64 * (sz_page + 24) as i64;
                    __state = 80;
                }
                79 => {
                    { let __p = &mut i_frame; let __t = *__p; *__p += 1; __t };
                    __state = 77;
                }
                80 => { __state = 81; }
                81 => { __state = 82; }
                82 => {
                    rc =
                        unsafe {
                            sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd },
                                a_frame as *mut (), sz_frame, i_offset)
                        };
                    __state = 83;
                }
                83 => { if rc != 0 { __state = 85; } else { __state = 84; } }
                84 => {
                    is_valid =
                        wal_decode_frame(unsafe { &mut *p_wal_1 }, &mut pgno,
                            &mut n_truncate, a_data, a_frame);
                    __state = 86;
                }
                85 => { __state = 76; }
                86 => {
                    if (is_valid == 0) as i32 != 0 {
                        __state = 88;
                    } else { __state = 87; }
                }
                87 => {
                    rc = wal_index_append(p_wal_1, i_frame, pgno);
                    __state = 89;
                }
                88 => { __state = 76; }
                89 => { if rc != 0 { __state = 91; } else { __state = 90; } }
                90 => {
                    if n_truncate != 0 { __state = 92; } else { __state = 79; }
                }
                91 => { __state = 76; }
                92 => {
                    unsafe { (*p_wal_1).hdr.mx_frame = i_frame };
                    __state = 93;
                }
                93 => {
                    unsafe { (*p_wal_1).hdr.n_page = n_truncate };
                    __state = 94;
                }
                94 => {
                    unsafe {
                        (*p_wal_1).hdr.sz_page =
                            (sz_page & 65280 | sz_page >> 16) as u16
                    };
                    __state = 95;
                }
                95 => { __state = 96; }
                96 => { __state = 97; }
                97 => {
                    a_frame_cksum[0 as usize] =
                        unsafe { (*p_wal_1).hdr.a_frame_cksum[0 as usize] };
                    __state = 98;
                }
                98 => {
                    a_frame_cksum[1 as usize] =
                        unsafe { (*p_wal_1).hdr.a_frame_cksum[1 as usize] };
                    __state = 79;
                }
                99 => { __state = 100; }
                100 => {
                    n_hdr =
                        if i_pg == 0 as u32 {
                                core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                    core::mem::size_of::<WalCkptInfo>() as u64
                            } else { 0 as u64 } as u32;
                    __state = 101;
                }
                101 => {
                    n_hdr32 =
                        (n_hdr as u64 / core::mem::size_of::<u32>() as u64) as u32;
                    __state = 102;
                }
                102 => {
                    unsafe {
                        memcpy(unsafe { &raw mut *a_share.add(n_hdr32 as usize) } as
                                *mut (),
                            unsafe { &raw mut *a_private.add(n_hdr32 as usize) } as
                                *const (),
                            core::mem::size_of::<HtSlot>() as u64 * (4096 * 2) as u64 +
                                    4096 as u64 * core::mem::size_of::<u32>() as u64 -
                                n_hdr as u64)
                    };
                    __state = 103;
                }
                103 => { { let _ = 0; }; __state = 104; }
                104 => { __state = 105; }
                105 => {
                    if i_frame <= i_last {
                        __state = 106;
                    } else { __state = 64; }
                }
                106 => { __state = 61; }
                107 => {
                    unsafe { sqlite3_free(a_frame as *mut ()) };
                    __state = 21;
                }
                108 => { __state = 3; }
                109 => { __state = 110; }
                110 => { __state = 111; }
                111 => {
                    unsafe {
                        (*p_wal_1).hdr.a_frame_cksum[0 as usize] =
                            a_frame_cksum[0 as usize]
                    };
                    __state = 112;
                }
                112 => {
                    unsafe {
                        (*p_wal_1).hdr.a_frame_cksum[1 as usize] =
                            a_frame_cksum[1 as usize]
                    };
                    __state = 113;
                }
                113 => { wal_index_write_hdr(p_wal_1); __state = 114; }
                114 => {
                    p_info = wal_ckpt_info(unsafe { &*p_wal_1 });
                    __state = 115;
                }
                115 => {
                    unsafe { (*p_info).n_backfill = 0 as u32 };
                    __state = 116;
                }
                116 => {
                    unsafe {
                        (*p_info).n_backfill_attempted =
                            unsafe { (*p_wal_1).hdr.mx_frame }
                    };
                    __state = 117;
                }
                117 => {
                    unsafe { (*p_info).a_read_mark[0 as usize] = 0 as u32 };
                    __state = 118;
                }
                118 => { i = 1; __state = 120; }
                119 => {
                    if unsafe { (*p_wal_1).hdr.n_page } != 0 {
                        __state = 132;
                    } else { __state = 108; }
                }
                120 => {
                    if i < 8 - 3 { __state = 121; } else { __state = 119; }
                }
                121 => {
                    rc = wal_lock_exclusive(unsafe { &*p_wal_1 }, 3 + i, 1);
                    __state = 123;
                }
                122 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 120;
                }
                123 => {
                    if rc == 0 { __state = 124; } else { __state = 125; }
                }
                124 => {
                    if i == 1 && unsafe { (*p_wal_1).hdr.mx_frame } != 0 {
                        __state = 127;
                    } else { __state = 128; }
                }
                125 => {
                    if rc != 5 { __state = 131; } else { __state = 122; }
                }
                126 => { { let _ = 0; }; __state = 129; }
                127 => {
                    unsafe {
                        (*p_info).a_read_mark[i as usize] =
                            unsafe { (*p_wal_1).hdr.mx_frame }
                    };
                    __state = 126;
                }
                128 => {
                    unsafe {
                        (*p_info).a_read_mark[i as usize] = 4294967295u32
                    };
                    __state = 126;
                }
                129 => { __state = 130; }
                130 => {
                    wal_unlock_exclusive(unsafe { &*p_wal_1 }, 3 + i, 1);
                    __state = 122;
                }
                131 => { __state = 3; }
                132 => {
                    unsafe {
                        sqlite3_log(27 | 1 << 8,
                            c"recovered %d frames from WAL file %s".as_ptr() as *mut i8
                                as *const i8, unsafe { (*p_wal_1).hdr.mx_frame },
                            unsafe { (*p_wal_1).z_wal_name })
                    };
                    __state = 108;
                }
                133 => {
                    wal_unlock_exclusive(unsafe { &*p_wal_1 }, i_lock,
                        3 + 0 - i_lock);
                    __state = 134;
                }
                134 => { return rc; }
                _ => {}
            }
        }
    }

    /// Return Code
    /// Size of log file
    /// Lock offset to lock for checkpoint
    /// Obtain an exclusive lock on all byte in the locking range not already
    ///* locked by the caller. The caller is guaranteed to have locked the
    ///* WAL_WRITE_LOCK byte, and may have also locked the WAL_CKPT_LOCK byte.
    ///* If successful, the same bytes that are locked here are unlocked before
    ///* this function returns.
    /// Buffer to load WAL header into
    /// Heap copy of *-shm hash being populated
    /// Malloc'd buffer to load entire frame
    /// Number of bytes in buffer aFrame[]
    /// Pointer to data part of aFrame buffer
    /// Page size according to the log
    /// Magic value read from WAL header
    /// Magic value read from WAL header
    /// True if this frame is valid
    /// Current 32KB wal-index page
    /// Last frame in wal, based on nSize alone
    /// Read in the WAL header.
    /// If the database page size is not a power of two, or is greater than
    ///* SQLITE_MAX_PAGE_SIZE, conclude that the WAL file contains no valid
    ///* data. Similarly, if the 'magic' value is invalid, ignore the whole
    ///* WAL file.
    /// Verify that the WAL header checksum is correct
    /// Verify that the version number on the WAL format is one that
    ///* are able to understand
    /// Malloc a buffer to read frames into.
    /// Read all frames from the log file.
    /// Index of last frame read
    /// Database page number for frame
    /// dbsize field from frame header
    /// Read and decode the next log frame.
    /// If nTruncate is non-zero, this is a commit record.
    /// Memcpy() should work fine here, on all reasonable implementations.
    ///* Technically, memcpy() might change the destination to some
    ///* intermediate value before setting to the final value, and that might
    ///* cause a concurrent reader to malfunction.  Memcpy() is allowed to
    ///* do that, according to the spec, but no memcpy() implementation that
    ///* we know of actually does that, which is why we say that memcpy()
    ///* is safe for this.  Memcpy() is certainly a lot faster.
    /// Reset the checkpoint-header. This is safe because this thread is
    ///* currently holding locks that exclude all other writers and
    ///* checkpointers. Then set the values of read-mark slots 1 through N.
    /// If more than one frame was recovered from the log file, report an
    ///* event via sqlite3_log(). This is to help with identifying performance
    ///* problems caused by applications routinely shutting down without
    ///* checkpointing the log file.
    unreachable!();
}

///* Read the wal-index header from the wal-index and into pWal->hdr.
///* If the wal-header appears to be corrupt, try to reconstruct the
///* wal-index from the WAL before returning.
///*
///* Set *pChanged to 1 if the wal-index header value in pWal->hdr is
///* changed by this operation.  If pWal->hdr is unchanged, set *pChanged
///* to 0.
///*
///* If the wal-index header is successfully read, return SQLITE_OK.
///* Otherwise an SQLite error code.
#[allow(unused_doc_comments)]
extern "C" fn wal_index_read_hdr(p_wal_1: *mut Wal, p_changed_1: *mut i32)
    -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut bad_hdr: i32 = 0;
    /// True if a header read failed
    let mut page0: *mut u32 = core::ptr::null_mut();

    /// Chunk of wal-index containing header
    /// Ensure that page 0 of the wal-index (the page that contains the
    ///* wal-index header) is mapped. Return early if an error occurs here.
    { let _ = 0; };
    rc = wal_index_page(p_wal_1, 0, &mut page0);
    if rc != 0 {
        { let _ = 0; };
        if rc == 8 | 5 << 8 {

            /// The SQLITE_READONLY_CANTINIT return means that the shared-memory
            ///* was openable but is not writable, and this thread is unable to
            ///* confirm that another write-capable connection has the shared-memory
            ///* open, and hence the content of the shared-memory is unreliable,
            ///* since the shared-memory might be inconsistent with the WAL file
            ///* and there is no writer on hand to fix it.
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p_wal_1).b_shm_unreliable = 1 as u8 };
            unsafe { (*p_wal_1).exclusive_mode = 2 as u8 };
            unsafe { *p_changed_1 = 1 };
        } else { return rc; }
    } else {}
    { let _ = 0; };

    /// If the first page of the wal-index has been mapped, try to read the
    ///* wal-index header immediately, without holding any lock. This usually
    ///* works, but may fail if the wal-index header is corrupt or currently
    ///* being modified by another thread or process.
    (bad_hdr =
        if !(page0).is_null() {
            wal_index_try_hdr(p_wal_1, unsafe { &mut *p_changed_1 })
        } else { 1 });
    if bad_hdr != 0 {
        if unsafe { (*p_wal_1).b_shm_unreliable } as i32 == 0 &&
                unsafe { (*p_wal_1).read_only } as i32 & 2 != 0 {
            if 0 == { rc = wal_lock_shared(unsafe { &*p_wal_1 }, 0); rc } {
                wal_unlock_shared(unsafe { &*p_wal_1 }, 0);
                rc = 8 | 1 << 8;
            }
        } else {
            let b_write_lock: i32 = unsafe { (*p_wal_1).write_lock } as i32;
            if b_write_lock != 0 ||
                    0 ==
                        { rc = wal_lock_exclusive(unsafe { &*p_wal_1 }, 0, 1); rc }
                {
                if (b_write_lock == 0) as i32 != 0 {
                    unsafe { (*p_wal_1).write_lock = 2 as u8 };
                }
                if 0 == { rc = wal_index_page(p_wal_1, 0, &mut page0); rc } {
                    bad_hdr =
                        wal_index_try_hdr(p_wal_1, unsafe { &mut *p_changed_1 });
                    if bad_hdr != 0 {
                        rc = wal_index_recover(p_wal_1);
                        unsafe { *p_changed_1 = 1 };
                    }
                }
                if b_write_lock == 0 {
                    unsafe { (*p_wal_1).write_lock = 0 as u8 };
                    wal_unlock_exclusive(unsafe { &*p_wal_1 }, 0, 1);
                }
            }
        }
    }
    if bad_hdr == 0 && unsafe { (*p_wal_1).hdr.i_version } != 3007000 as u32 {
        rc = unsafe { sqlite3_cantopen_error(2747) };
    }
    if unsafe { (*p_wal_1).b_shm_unreliable } != 0 {
        if rc != 0 {
            wal_index_close(unsafe { &*p_wal_1 }, 0);
            unsafe { (*p_wal_1).b_shm_unreliable = 0 as u8 };
            { let _ = 0; };
            if rc == 10 | 2 << 8 { rc = -1; }
        }
        unsafe { (*p_wal_1).exclusive_mode = 0 as u8 };
    }
    return rc;
}

///* The cache of the wal-index header must be valid to call this function.
///* Return the page-size in bytes used by the database.
extern "C" fn wal_pagesize(p_wal_1: &Wal) -> i32 {
    return ((*p_wal_1).hdr.sz_page as i32 & 65024) +
            (((*p_wal_1).hdr.sz_page as i32 & 1) << 16);
}

///* This structure is used to implement an iterator that loops through
///* all frames in the WAL in database page order. Where two or more frames
///* correspond to the same database page, the iterator visits only the
///* frame most recently written to the WAL (in other words, the frame with
///* the largest index).
///*
///* The internals of this structure are only accessed by:
///*
///*   walIteratorInit() - Create a new iterator,
///*   walIteratorNext() - Step an iterator,
///*   walIteratorFree() - Free an iterator.
///*
///* This functionality is used by the checkpoint code (see walCheckpoint()).
#[repr(C)]
#[derive(Copy, Clone)]
struct WalIterator {
    i_prior: u32,
    n_segment: i32,
    a_segment: [WalSegment; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WalSegment {
    i_next: i32,
    a_index: *mut HtSlot,
    a_pgno: *mut u32,
    n_entry: i32,
    i_zero: i32,
}

///* This function merges two sorted lists into a single sorted list.
///*
///* aLeft[] and aRight[] are arrays of indices.  The sort key is
///* aContent[aLeft[]] and aContent[aRight[]].  Upon entry, the following
///* is guaranteed for all J<K:
///*
///*        aContent[aLeft[J]] < aContent[aLeft[K]]
///*        aContent[aRight[J]] < aContent[aRight[K]]
///*
///* This routine overwrites aRight[] with a new (probably longer) sequence
///* of indices such that the aRight[] contains every index that appears in
///* either aLeft[] or the old aRight[] and such that the second condition
///* above is still met.
///*
///* The aContent[aLeft[X]] values will be unique for all X.  And the
///* aContent[aRight[X]] values will be unique too.  But there might be
///* one or more combinations of X and Y such that
///*
///*      aLeft[X]!=aRight[Y]  &&  aContent[aLeft[X]] == aContent[aRight[Y]]
///*
///* When that happens, omit the aLeft[X] and use the aRight[Y] index.
#[allow(unused_doc_comments)]
extern "C" fn wal_merge(a_content_1: *const u32, a_left_1: *mut HtSlot,
    n_left_1: i32, pa_right_1: &mut *mut HtSlot, pn_right_1: &mut i32,
    a_tmp_1: *mut HtSlot) -> () {
    let mut i_left: i32 = 0;
    /// Current index in aLeft
    let mut i_right: i32 = 0;
    /// Current index in aRight
    let mut i_out: i32 = 0;
    /// Current index in output buffer
    let n_right: i32 = *pn_right_1;
    let a_right: *const HtSlot = *pa_right_1 as *const HtSlot;
    { let _ = 0; };
    while i_right < n_right || i_left < n_left_1 {
        let mut logpage: HtSlot = 0 as HtSlot;
        let mut dbpage: Pgno = 0 as Pgno;
        if i_left < n_left_1 &&
                (i_right >= n_right ||
                    (unsafe {
                                    *a_content_1.add(unsafe {
                                                    *a_left_1.offset(i_left as isize)
                                                } as usize)
                                } as u32) <
                        unsafe {
                            *a_content_1.add(unsafe {
                                            *a_right.offset(i_right as isize)
                                        } as usize)
                        }) {
            logpage =
                unsafe {
                    *a_left_1.offset({
                                    let __p = &mut i_left;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                };
        } else {
            logpage =
                unsafe {
                    *a_right.offset({
                                    let __p = &mut i_right;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                };
        }
        dbpage = unsafe { *a_content_1.add(logpage as usize) } as Pgno;
        unsafe {
            *a_tmp_1.offset({
                                let __p = &mut i_out;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = logpage
        };
        if i_left < n_left_1 &&
                unsafe {
                            *a_content_1.add(unsafe {
                                            *a_left_1.offset(i_left as isize)
                                        } as usize)
                        } as u32 == dbpage {
            { let __p = &mut i_left; let __t = *__p; *__p += 1; __t };
        }
        { let _ = 0; };
        { let _ = 0; };
    }
    *pa_right_1 = a_left_1;
    *pn_right_1 = i_out;
    unsafe {
        memcpy(a_left_1 as *mut (), a_tmp_1 as *const (),
            core::mem::size_of::<HtSlot>() as u64 * i_out as u64)
    };
}

///* Sort the elements in list aList using aContent[] as the sort key.
///* Remove elements with duplicate keys, preferring to keep the
///* larger aList[] values.
///*
///* The aList[] entries are indices into aContent[].  The values in
///* aList[] are to be sorted so that for all J<K:
///*
///*      aContent[aList[J]] < aContent[aList[K]]
///*
///* For any X and Y such that
///*
///*      aContent[aList[X]] == aContent[aList[Y]]
///*
///* Keep the larger of the two values aList[X] and aList[Y] and discard
///* the smaller.
#[allow(unused_doc_comments)]
extern "C" fn wal_mergesort(a_content_1: *const u32, a_buffer_1: *mut HtSlot,
    a_list_1: *mut HtSlot, pn_list_1: &mut i32) -> () {
    /// Number of elements in aList
    /// Pointer to sub-list content
    let n_list: i32 = *pn_list_1 as i32;
    /// Size of input list
    let mut n_merge: i32 = 0;
    /// Number of elements in list aMerge
    let mut a_merge: *mut HtSlot = core::ptr::null_mut();
    /// List to be merged
    let mut i_list: i32 = 0;
    /// Index into input list
    let mut i_sub: u32 = 0 as u32;
    /// Index into aSub array
    let mut a_sub: [SublistN7Sublist; 13] = unsafe { core::mem::zeroed() };

    /// Array of sub-lists
    unsafe {
        memset(&raw mut a_sub[0 as usize] as *mut SublistN7Sublist as *mut (),
            0, core::mem::size_of::<[SublistN7Sublist; 13]>() as u64)
    };
    { let _ = 0; };
    { let _ = 0; };
    {
        i_list = 0;
        '__b10: loop {
            if !(i_list < n_list) { break '__b10; }
            '__c10: loop {
                n_merge = 1;
                a_merge = unsafe { a_list_1.offset(i_list as isize) };
                {
                    i_sub = 0 as u32;
                    '__b11: loop {
                        if !(i_list & 1 << i_sub != 0) { break '__b11; }
                        '__c11: loop {
                            let mut p: *const SublistN7Sublist = core::ptr::null();
                            { let _ = 0; };
                            p = &mut a_sub[i_sub as usize];
                            { let _ = 0; };
                            { let _ = 0; };
                            wal_merge(a_content_1, unsafe { (*p).a_list },
                                unsafe { (*p).n_list }, &mut a_merge, &mut n_merge,
                                a_buffer_1);
                            break '__c11;
                        }
                        { let __p = &mut i_sub; let __t = *__p; *__p += 1; __t };
                    }
                }
                a_sub[i_sub as usize].a_list = a_merge;
                a_sub[i_sub as usize].n_list = n_merge;
                break '__c10;
            }
            { let __p = &mut i_list; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        { let __p = &mut i_sub; let __t = *__p; *__p += 1; __t };
        '__b12: loop {
            if !(i_sub <
                            (core::mem::size_of::<[SublistN7Sublist; 13]>() as u64 /
                                        core::mem::size_of::<SublistN7Sublist>() as u64) as i32 as
                                u32) {
                break '__b12;
            }
            '__c12: loop {
                if n_list & 1 << i_sub != 0 {
                    let mut p: *const SublistN7Sublist = core::ptr::null();
                    { let _ = 0; };
                    p = &mut a_sub[i_sub as usize];
                    { let _ = 0; };
                    { let _ = 0; };
                    wal_merge(a_content_1, unsafe { (*p).a_list },
                        unsafe { (*p).n_list }, &mut a_merge, &mut n_merge,
                        a_buffer_1);
                }
                break '__c12;
            }
            { let __p = &mut i_sub; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    *pn_list_1 = n_merge;
}

///* Free an iterator allocated by walIteratorInit().
extern "C" fn wal_iterator_free(p: *mut WalIterator) -> () {
    unsafe { sqlite3_free(p as *mut ()) };
}

///* Construct a WalInterator object that can be used to loop over all
///* pages in the WAL following frame nBackfill in ascending order. Frames
///* nBackfill or earlier may be included - excluding them is an optimization
///* only. The caller must hold the checkpoint lock.
///*
///* On success, make *pp point to the newly allocated WalInterator object
///* return SQLITE_OK. Otherwise, return an error code. If this routine
///* returns an error, the value of *pp is undefined.
///*
///* The calling routine should invoke walIteratorFree() to destroy the
///* WalIterator object when it has finished with it.
#[allow(unused_doc_comments)]
extern "C" fn wal_iterator_init(p_wal_1: *mut Wal, n_backfill_1: u32,
    pp: &mut *mut WalIterator) -> i32 {
    let mut p: *mut WalIterator = core::ptr::null_mut();
    /// Return value
    let mut n_segment: i32 = 0;
    /// Number of segments to merge
    let mut i_last: u32 = 0 as u32;
    /// Last frame in log
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Number of bytes to allocate
    let mut i: i32 = 0;
    /// Iterator variable
    let mut a_tmp: *mut HtSlot = core::ptr::null_mut();
    /// Temp space used by merge-sort
    let mut rc: i32 = 0;

    /// Return Code
    /// This routine only runs while holding the checkpoint lock. And
    ///* it only runs if there is actually content in the log (mxFrame>0).
    { let _ = 0; };
    i_last = unsafe { (*p_wal_1).hdr.mx_frame };

    /// Allocate space for the WalIterator object.
    (n_segment = wal_frame_page(i_last) + 1);
    n_byte =
        (core::mem::offset_of!(WalIterator, a_segment) as u64 +
                    n_segment as u64 * core::mem::size_of::<WalSegment>() as u64
                + i_last as u64 * core::mem::size_of::<HtSlot>() as u64) as
            Sqlite3Int64;
    p =
        unsafe {
                sqlite3_malloc64(n_byte as u64 +
                        (core::mem::size_of::<HtSlot>() as u64 *
                                if i_last > 4096 as u32 { 4096 as u32 } else { i_last } as
                                    u64) as u64)
            } as *mut WalIterator;
    if (p).is_null() as i32 != 0 { return 7; }
    unsafe { memset(p as *mut (), 0, n_byte as u64) };
    unsafe { (*p).n_segment = n_segment };
    a_tmp =
        unsafe { &raw mut *(p as *mut u8).offset(n_byte as isize) } as
            *mut HtSlot;
    {
        i = wal_frame_page(n_backfill_1 + 1 as u32);
        '__b13: loop {
            if !(rc == 0 && i < n_segment) { break '__b13; }
            '__c13: loop {
                let mut s_loc: WalHashLoc = unsafe { core::mem::zeroed() };
                rc = wal_hash_get(p_wal_1, i, &mut s_loc);
                if rc == 0 {
                    let mut j: i32 = 0;
                    /// Counter variable
                    let mut n_entry: i32 = 0;
                    /// Number of entries in this segment
                    let mut a_index: *mut HtSlot = core::ptr::null_mut();
                    if i + 1 == n_segment {
                        n_entry = (i_last - s_loc.i_zero) as i32;
                    } else {
                        n_entry =
                            unsafe {
                                        (s_loc.a_hash as
                                                *mut u32).offset_from(s_loc.a_pgno as *mut u32)
                                    } as i64 as i32;
                    }
                    a_index =
                        unsafe {
                            (unsafe {
                                        &raw mut *(unsafe { (*p).a_segment.as_ptr() } as
                                                        *mut WalSegment).offset(unsafe { (*p).n_segment } as isize)
                                    } as *mut HtSlot).add(s_loc.i_zero as usize)
                        };
                    {
                        let __p = &mut s_loc.i_zero;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    {
                        j = 0;
                        '__b14: loop {
                            if !(j < n_entry) { break '__b14; }
                            '__c14: loop {
                                unsafe { *a_index.offset(j as isize) = j as HtSlot };
                                break '__c14;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    wal_mergesort(s_loc.a_pgno as *mut u32 as *const u32, a_tmp,
                        a_index, &mut n_entry);
                    unsafe {
                        (*(unsafe { (*p).a_segment.as_ptr() } as
                                            *mut WalSegment).offset(i as isize)).i_zero =
                            s_loc.i_zero as i32
                    };
                    unsafe {
                        (*(unsafe { (*p).a_segment.as_ptr() } as
                                            *mut WalSegment).offset(i as isize)).n_entry = n_entry
                    };
                    unsafe {
                        (*(unsafe { (*p).a_segment.as_ptr() } as
                                            *mut WalSegment).offset(i as isize)).a_index = a_index
                    };
                    unsafe {
                        (*(unsafe { (*p).a_segment.as_ptr() } as
                                            *mut WalSegment).offset(i as isize)).a_pgno =
                            s_loc.a_pgno as *mut u32
                    };
                }
                break '__c13;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc != 0 { wal_iterator_free(p); p = core::ptr::null_mut(); }
    *pp = p;
    return rc;
}

///* Find the smallest page number out of all pages held in the WAL that
///* has not been returned by any prior invocation of this method on the
///* same WalIterator object.   Write into *piFrame the frame index where
///* that page was last written into the WAL.  Write into *piPage the page
///* number.
///*
///* Return 0 on success.  If there are no pages in the WAL with a page
///* number larger than *piPage, then return 1.
#[allow(unused_doc_comments)]
extern "C" fn wal_iterator_next(p: &mut WalIterator, pi_page_1: &mut u32,
    pi_frame_1: &mut u32) -> i32 {
    let mut i_min: u32 = 0 as u32;
    /// Result pgno must be greater than iMin
    let mut i_ret: u32 = 4294967295u32;
    /// 0xffffffff is never a valid page number
    let mut i: i32 = 0;

    /// For looping through segments
    (i_min = (*p).i_prior);
    { let _ = 0; };
    {
        i = (*p).n_segment - 1;
        '__b15: loop {
            if !(i >= 0) { break '__b15; }
            '__c15: loop {
                let p_segment: *mut WalSegment =
                    unsafe {
                        &mut *((*p).a_segment.as_ptr() as
                                        *mut WalSegment).offset(i as isize)
                    };
                while unsafe { (*p_segment).i_next } <
                        unsafe { (*p_segment).n_entry } {
                    let i_pg: u32 =
                        unsafe {
                            *unsafe {
                                    (*p_segment).a_pgno.add(unsafe {
                                                *unsafe {
                                                        (*p_segment).a_index.offset(unsafe { (*p_segment).i_next }
                                                                as isize)
                                                    }
                                            } as usize)
                                }
                        };
                    if i_pg > i_min {
                        if i_pg < i_ret {
                            i_ret = i_pg;
                            *pi_frame_1 =
                                (unsafe { (*p_segment).i_zero } +
                                        unsafe {
                                                *unsafe {
                                                        (*p_segment).a_index.offset(unsafe { (*p_segment).i_next }
                                                                as isize)
                                                    }
                                            } as i32) as u32;
                        }
                        break;
                    }
                    {
                        let __p = unsafe { &mut (*p_segment).i_next };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                break '__c15;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    *pi_page_1 = { (*p).i_prior = i_ret; (*p).i_prior };
    return (i_ret == 4294967295u32) as i32;
}

///* The following is guaranteed when this function is called:
///*
///*   a) the WRITER lock is held,
///*   b) the entire log file has been checkpointed, and
///*   c) any existing readers are reading exclusively from the database
///*      file - there are no readers that may attempt to read a frame from
///*      the log file.
///*
///* This function updates the shared-memory structures so that the next
///* client to write to the database (which may be this one) does so by
///* writing frames into the start of the log file.
///*
///* The value of parameter salt1 is used as the aSalt[1] value in the
///* new wal-index header. It should be passed a pseudo-random value (i.e.
///* one obtained from sqlite3_randomness()).
#[allow(unused_doc_comments)]
extern "C" fn wal_restart_hdr(p_wal_1: *mut Wal, mut salt1: u32) -> () {
    let p_info: *mut WalCkptInfo = wal_ckpt_info(unsafe { &*p_wal_1 });
    let mut i: i32 = 0;
    /// Loop counter
    let a_salt: *mut u32 =
        unsafe { &raw mut (*p_wal_1).hdr.a_salt[0 as usize] } as *mut u32;

    /// Big-endian salt values
    {
        let __p = unsafe { &mut (*p_wal_1).n_ckpt };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { (*p_wal_1).hdr.mx_frame = 0 as u32 };
    unsafe {
        sqlite3_put4byte(unsafe { &raw mut *a_salt.offset(0 as isize) } as
                *mut u8,
            1 as u32 +
                unsafe {
                    sqlite3_get4byte(unsafe {
                                    &raw mut *a_salt.offset(0 as isize)
                                } as *mut u8 as *const u8)
                })
    };
    unsafe {
        memcpy(unsafe { &raw mut (*p_wal_1).hdr.a_salt[1 as usize] } as
                *mut (), &raw mut salt1 as *const (), 4 as u64)
    };
    wal_index_write_hdr(p_wal_1);
    unsafe {
        std::sync::atomic::AtomicU32::from_ptr(unsafe {
                        &raw mut (*p_info).n_backfill
                    } as
                    *mut u32).store(0 as u32,
            std::sync::atomic::Ordering::Relaxed)
    };
    unsafe { (*p_info).n_backfill_attempted = 0 as u32 };
    unsafe { (*p_info).a_read_mark[1 as usize] = 0 as u32 };
    {
        i = 2;
        '__b17: loop {
            if !(i < 8 - 3) { break '__b17; }
            '__c17: loop {
                unsafe { (*p_info).a_read_mark[i as usize] = 4294967295u32 };
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
}

///* Copy as much content as we can from the WAL back into the database file
///* in response to an sqlite3_wal_checkpoint() request or the equivalent.
///*
///* The amount of information copies from WAL to database might be limited
///* by active readers.  This routine will never overwrite a database page
///* that a concurrent reader might be using.
///*
///* All I/O barrier operations (a.k.a fsyncs) occur in this routine when
///* SQLite is in WAL-mode in synchronous=NORMAL.  That means that if
///* checkpoints are always run by a background thread or background
///* process, foreground threads will never block on a lengthy fsync call.
///*
///* Fsync is called on the WAL before writing content out of the WAL and
///* into the database.  This ensures that if the new content is persistent
///* in the WAL and can be recovered following a power-loss or hard reset.
///*
///* Fsync is also called on the database file if (and only if) the entire
///* WAL content is copied into the database file.  This second fsync makes
///* it safe to delete the WAL since the new content will persist in the
///* database file.
///*
///* This routine uses and updates the nBackfill field of the wal-index header.
///* This is the only routine that will increase the value of nBackfill.
///* (A WAL reset or recovery will revert nBackfill to zero, but not increase
///* its value.)
///*
///* The caller must be holding sufficient locks to ensure that no other
///* checkpoint is running (in any other thread or process) at the same
///* time.
#[allow(unused_doc_comments)]
extern "C" fn wal_checkpoint(p_wal_1: *mut Wal, db: &mut Sqlite3,
    e_mode_1: i32, mut x_busy_1: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_busy_arg_1: *mut (), sync_flags: i32, z_buf_1: *mut u8) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        /// Return code
        let mut sz_page: i32 = 0;
        /// Database page-size
        let mut p_iter: *mut WalIterator = core::ptr::null_mut();
        /// Wal iterator context
        let mut i_dbpage: u32 = 0 as u32;
        /// Next database page to write
        let mut i_frame: u32 = 0 as u32;
        /// Wal frame containing data for iDbpage
        let mut mx_safe_frame: u32 = 0 as u32;
        /// Max frame that can be backfilled
        let mut mx_page: u32 = 0 as u32;
        /// Max database page to write
        let mut i: i32 = 0;
        /// Loop counter
        let mut p_info: *mut WalCkptInfo = core::ptr::null_mut();
        /// The checkpoint status information
        /// EVIDENCE-OF: R-62920-47450 The busy-handler callback is never invoked
        ///* in the SQLITE_CHECKPOINT_PASSIVE mode.
        /// Compute in mxSafeFrame the index of the last frame of the WAL that is
        ///* safe to write into the database.  Frames beyond mxSafeFrame might
        ///* overwrite database pages that are in use by active readers and thus
        ///* cannot be backfilled from the WAL.
        let mut y: u32 = 0 as u32;
        let mut i_mark: u32 = 0 as u32;
        /// Allocate the iterator
        let mut n_backfill: u32 = 0 as u32;
        let mut p_live: *const WalIndexHdr = core::ptr::null();
        /// Now that read-lock slot 0 is locked, check that the wal has not been
        ///* wrapped since the header was read for this checkpoint. If it was, then
        ///* there was no work to do anyway.  In this case the
        ///* (pInfo->nBackfill<pWal->hdr.mxFrame) test above only passed because
        ///* pInfo->nBackfill had already been set to 0 by the writer that wrapped
        ///* the wal file. It would also be dangerous to proceed, as there may be
        ///* fewer than pWal->hdr.mxFrame valid frames in the wal file.
        let mut b_chg: i32 = 0;
        /// Sync the WAL to disk
        /// If the database may grow as a result of this checkpoint, hint
        ///* about the eventual size of the db file to the VFS layer.
        let mut n_req: i64 = 0 as i64;
        let mut n_size: i64 = 0 as i64;
        /// Current size of database file
        /// If the size of the final database is larger than the current
        ///* database plus the amount of data in the wal file, plus the
        ///* maximum size of the pending-byte page (65536 bytes), then
        ///* must be corruption somewhere.
        /// Iterate through the contents of the WAL, copying data to the 
        ///* db file
        let mut i_offset: i64 = 0 as i64;
        /// testcase( IS_BIG_INT(iOffset) ); // requires a 4GiB WAL file
        /// If work was actually accomplished...
        let mut sz_db: i64 = 0 as i64;
        /// Release the reader lock held while backfilling
        /// Reset the return code so as not to report a checkpoint failure
        ///* just because there are active readers.
        /// If this is an SQLITE_CHECKPOINT_RESTART or TRUNCATE operation, and the
        ///* entire wal file has been copied into the database file, then block
        ///* until all readers have finished using the wal file. This ensures that
        ///* the next process to write to the database restarts the wal file.
        let mut salt1: u32 = 0 as u32;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s19:
                {
                match __state {
                    0 => { rc = 0; __state = 3; }
                    2 => { __state = 110; }
                    3 => { __state = 4; }
                    4 => { p_iter = core::ptr::null_mut(); __state = 5; }
                    5 => { i_dbpage = 0 as u32; __state = 6; }
                    6 => { i_frame = 0 as u32; __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => {
                        sz_page = wal_pagesize(unsafe { &*p_wal_1 });
                        __state = 12;
                    }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => {
                        p_info = wal_ckpt_info(unsafe { &*p_wal_1 });
                        __state = 15;
                    }
                    15 => {
                        if (unsafe { (*p_info).n_backfill } as u32) <
                                unsafe { (*p_wal_1).hdr.mx_frame } {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => {
                        if rc == 0 && e_mode_1 != 0 {
                            __state = 95;
                        } else { __state = 94; }
                    }
                    17 => { { let _ = 0; }; __state = 18; }
                    18 => {
                        mx_safe_frame = unsafe { (*p_wal_1).hdr.mx_frame };
                        __state = 19;
                    }
                    19 => {
                        mx_page = unsafe { (*p_wal_1).hdr.n_page };
                        __state = 20;
                    }
                    20 => { i = 1; __state = 22; }
                    21 => {
                        if (unsafe { (*p_info).n_backfill } as u32) < mx_safe_frame
                            {
                            __state = 41;
                        } else { __state = 40; }
                    }
                    22 => {
                        if i < 8 - 3 { __state = 23; } else { __state = 21; }
                    }
                    23 => {
                        y =
                            unsafe {
                                std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                                (unsafe { &raw mut (*p_info).a_read_mark[0 as usize] } as
                                                        *mut u32).offset(i as isize)
                                            } as *mut u32).load(std::sync::atomic::Ordering::Relaxed)
                            };
                        __state = 25;
                    }
                    24 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 22;
                    }
                    25 => { { let _ = 0; }; __state = 26; }
                    26 => { __state = 27; }
                    27 => {
                        if mx_safe_frame > y {
                            __state = 28;
                        } else { __state = 24; }
                    }
                    28 => { { let _ = 0; }; __state = 29; }
                    29 => {
                        rc =
                            wal_busy_lock(p_wal_1, x_busy_1, p_busy_arg_1, 3 + i, 1);
                        __state = 30;
                    }
                    30 => {
                        if rc == 0 { __state = 31; } else { __state = 32; }
                    }
                    31 => {
                        i_mark = if i == 1 { mx_safe_frame } else { 4294967295u32 };
                        __state = 33;
                    }
                    32 => {
                        if rc == 5 { __state = 37; } else { __state = 38; }
                    }
                    33 => {
                        unsafe {
                            std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                            (unsafe { &raw mut (*p_info).a_read_mark[0 as usize] } as
                                                    *mut u32).offset(i as isize)
                                        } as
                                        *mut u32).store(i_mark as u32,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        __state = 34;
                    }
                    34 => { { let _ = 0; }; __state = 35; }
                    35 => { __state = 36; }
                    36 => {
                        wal_unlock_exclusive(unsafe { &*p_wal_1 }, 3 + i, 1);
                        __state = 24;
                    }
                    37 => { mx_safe_frame = y; __state = 39; }
                    38 => { __state = 2; }
                    39 => { x_busy_1 = None; __state = 24; }
                    40 => {
                        if !(p_iter).is_null() &&
                                {
                                        rc =
                                            wal_busy_lock(p_wal_1, x_busy_1, p_busy_arg_1, 3 + 0, 1);
                                        rc
                                    } == 0 {
                            __state = 44;
                        } else { __state = 43; }
                    }
                    41 => {
                        rc =
                            wal_iterator_init(p_wal_1, unsafe { (*p_info).n_backfill },
                                &mut p_iter);
                        __state = 42;
                    }
                    42 => { { let _ = 0; }; __state = 40; }
                    43 => {
                        if rc == 5 { __state = 93; } else { __state = 16; }
                    }
                    44 => {
                        n_backfill = unsafe { (*p_info).n_backfill } as u32;
                        __state = 45;
                    }
                    45 => {
                        p_live =
                            wal_index_hdr(unsafe { &*p_wal_1 }) as *mut WalIndexHdr as
                                *const WalIndexHdr;
                        __state = 46;
                    }
                    46 => {
                        b_chg =
                            unsafe {
                                memcmp(unsafe { &raw const (*p_live).a_salt[0 as usize] } as
                                            *mut u32 as *const (),
                                    unsafe { &raw mut (*p_wal_1).hdr.a_salt[0 as usize] } as
                                            *mut u32 as *const (),
                                    core::mem::size_of::<[u32; 2]>() as u64)
                            };
                        __state = 47;
                    }
                    47 => {
                        if 0 == b_chg { __state = 49; } else { __state = 48; }
                    }
                    48 => {
                        wal_unlock_exclusive(unsafe { &*p_wal_1 }, 3 + 0, 1);
                        __state = 43;
                    }
                    49 => {
                        unsafe { (*p_info).n_backfill_attempted = mx_safe_frame };
                        __state = 50;
                    }
                    50 => { { let _ = 0; }; __state = 51; }
                    51 => { __state = 52; }
                    52 => {
                        rc =
                            unsafe {
                                sqlite3_os_sync(unsafe { (*p_wal_1).p_wal_fd },
                                    sync_flags >> 2 & 3)
                            };
                        __state = 53;
                    }
                    53 => {
                        if rc == 0 { __state = 55; } else { __state = 54; }
                    }
                    54 => {
                        if rc == 0 &&
                                0 ==
                                    wal_iterator_next(unsafe { &mut *p_iter }, &mut i_dbpage,
                                        &mut i_frame) {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    55 => {
                        n_req = mx_page as i64 * sz_page as i64;
                        __state = 56;
                    }
                    56 => { __state = 57; }
                    57 => {
                        unsafe {
                            sqlite3_os_file_control(unsafe { (*p_wal_1).p_db_fd }, 39,
                                core::ptr::null_mut())
                        };
                        __state = 58;
                    }
                    58 => {
                        rc =
                            unsafe {
                                sqlite3_os_file_size(unsafe { (*p_wal_1).p_db_fd },
                                    &mut n_size)
                            };
                        __state = 59;
                    }
                    59 => {
                        if rc == 0 && n_size < n_req {
                            __state = 60;
                        } else { __state = 54; }
                    }
                    60 => {
                        if (n_size + 65536 as i64 +
                                        unsafe { (*p_wal_1).hdr.mx_frame } as i64 * sz_page as i64)
                                < n_req {
                            __state = 61;
                        } else { __state = 62; }
                    }
                    61 => {
                        rc = unsafe { sqlite3_corrupt_error(2293) };
                        __state = 54;
                    }
                    62 => {
                        unsafe {
                            sqlite3_os_file_control_hint(unsafe { (*p_wal_1).p_db_fd },
                                5, &raw mut n_req as *mut ())
                        };
                        __state = 54;
                    }
                    63 => {
                        unsafe {
                            sqlite3_os_file_control(unsafe { (*p_wal_1).p_db_fd }, 37,
                                core::ptr::null_mut())
                        };
                        __state = 82;
                    }
                    64 => { __state = 65; }
                    65 => { { let _ = 0; }; __state = 66; }
                    66 => { { let _ = 0; }; __state = 67; }
                    67 => { __state = 68; }
                    68 => {
                        if unsafe {
                                    std::sync::atomic::AtomicI32::from_ptr(&raw mut (*db).u1.is_interrupted
                                                as *mut i32).load(std::sync::atomic::Ordering::Relaxed)
                                } != 0 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        if i_frame <= n_backfill || i_frame > mx_safe_frame ||
                                i_dbpage > mx_page {
                            __state = 73;
                        } else { __state = 72; }
                    }
                    70 => {
                        rc = if (*db).malloc_failed != 0 { 7 } else { 9 };
                        __state = 71;
                    }
                    71 => { __state = 63; }
                    72 => {
                        i_offset =
                            32 as i64 +
                                    (i_frame - 1 as u32) as i64 * (sz_page + 24) as i64 +
                                24 as i64;
                        __state = 74;
                    }
                    73 => { __state = 54; }
                    74 => {
                        rc =
                            unsafe {
                                sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd },
                                    z_buf_1 as *mut (), sz_page, i_offset)
                            };
                        __state = 75;
                    }
                    75 => {
                        if rc != 0 { __state = 77; } else { __state = 76; }
                    }
                    76 => {
                        i_offset = (i_dbpage - 1 as u32) as i64 * sz_page as i64;
                        __state = 78;
                    }
                    77 => { __state = 63; }
                    78 => { __state = 79; }
                    79 => {
                        rc =
                            unsafe {
                                sqlite3_os_write(unsafe { (*p_wal_1).p_db_fd },
                                    z_buf_1 as *const (), sz_page, i_offset)
                            };
                        __state = 80;
                    }
                    80 => {
                        if rc != 0 { __state = 81; } else { __state = 54; }
                    }
                    81 => { __state = 63; }
                    82 => {
                        if rc == 0 { __state = 83; } else { __state = 48; }
                    }
                    83 => {
                        if mx_safe_frame ==
                                unsafe { (*wal_index_hdr(unsafe { &*p_wal_1 })).mx_frame } {
                            __state = 85;
                        } else { __state = 84; }
                    }
                    84 => {
                        if rc == 0 { __state = 90; } else { __state = 48; }
                    }
                    85 => {
                        sz_db =
                            unsafe { (*p_wal_1).hdr.n_page } as i64 * sz_page as i64;
                        __state = 86;
                    }
                    86 => { __state = 87; }
                    87 => {
                        rc =
                            unsafe {
                                sqlite3_os_truncate(unsafe { (*p_wal_1).p_db_fd }, sz_db)
                            };
                        __state = 88;
                    }
                    88 => {
                        if rc == 0 { __state = 89; } else { __state = 84; }
                    }
                    89 => {
                        rc =
                            unsafe {
                                sqlite3_os_sync(unsafe { (*p_wal_1).p_db_fd },
                                    sync_flags >> 2 & 3)
                            };
                        __state = 84;
                    }
                    90 => {
                        unsafe {
                            std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                            &raw mut (*p_info).n_backfill
                                        } as
                                        *mut u32).store(mx_safe_frame as u32,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        __state = 91;
                    }
                    91 => { { let _ = 0; }; __state = 92; }
                    92 => { __state = 48; }
                    93 => { rc = 0; __state = 16; }
                    94 => { __state = 2; }
                    95 => { { let _ = 0; }; __state = 96; }
                    96 => { { let _ = 0; }; __state = 97; }
                    97 => { __state = 98; }
                    98 => {
                        if (unsafe { (*p_info).n_backfill } as u32) <
                                unsafe { (*p_wal_1).hdr.mx_frame } {
                            __state = 99;
                        } else { __state = 100; }
                    }
                    99 => { rc = 5; __state = 94; }
                    100 => {
                        if e_mode_1 >= 2 { __state = 101; } else { __state = 94; }
                    }
                    101 => { __state = 102; }
                    102 => {
                        unsafe { sqlite3_randomness(4, &raw mut salt1 as *mut ()) };
                        __state = 103;
                    }
                    103 => { { let _ = 0; }; __state = 104; }
                    104 => {
                        rc =
                            wal_busy_lock(p_wal_1, x_busy_1, p_busy_arg_1, 3 + 1,
                                8 - 3 - 1);
                        __state = 105;
                    }
                    105 => {
                        if rc == 0 { __state = 106; } else { __state = 94; }
                    }
                    106 => {
                        if e_mode_1 == 3 { __state = 108; } else { __state = 107; }
                    }
                    107 => {
                        wal_unlock_exclusive(unsafe { &*p_wal_1 }, 3 + 1,
                            8 - 3 - 1);
                        __state = 94;
                    }
                    108 => { wal_restart_hdr(p_wal_1, salt1); __state = 109; }
                    109 => {
                        rc =
                            unsafe {
                                sqlite3_os_truncate(unsafe { (*p_wal_1).p_wal_fd },
                                    0 as i64)
                            };
                        __state = 107;
                    }
                    110 => { wal_iterator_free(p_iter); __state = 111; }
                    111 => { return rc; }
                    _ => {}
                }
            }
        }

        /// Return code
        /// Database page-size
        /// Wal iterator context
        /// Next database page to write
        /// Wal frame containing data for iDbpage
        /// Max frame that can be backfilled
        /// Max database page to write
        /// Loop counter
        /// The checkpoint status information
        /// EVIDENCE-OF: R-62920-47450 The busy-handler callback is never invoked
        ///* in the SQLITE_CHECKPOINT_PASSIVE mode.
        /// Compute in mxSafeFrame the index of the last frame of the WAL that is
        ///* safe to write into the database.  Frames beyond mxSafeFrame might
        ///* overwrite database pages that are in use by active readers and thus
        ///* cannot be backfilled from the WAL.
        /// Allocate the iterator
        /// Now that read-lock slot 0 is locked, check that the wal has not been
        ///* wrapped since the header was read for this checkpoint. If it was, then
        ///* there was no work to do anyway.  In this case the
        ///* (pInfo->nBackfill<pWal->hdr.mxFrame) test above only passed because
        ///* pInfo->nBackfill had already been set to 0 by the writer that wrapped
        ///* the wal file. It would also be dangerous to proceed, as there may be
        ///* fewer than pWal->hdr.mxFrame valid frames in the wal file.
        /// Sync the WAL to disk
        /// If the database may grow as a result of this checkpoint, hint
        ///* about the eventual size of the db file to the VFS layer.
        /// Current size of database file
        /// If the size of the final database is larger than the current
        ///* database plus the amount of data in the wal file, plus the
        ///* maximum size of the pending-byte page (65536 bytes), then
        ///* must be corruption somewhere.
        /// Iterate through the contents of the WAL, copying data to the 
        ///* db file
        /// testcase( IS_BIG_INT(iOffset) ); // requires a 4GiB WAL file
        /// If work was actually accomplished...
        /// Release the reader lock held while backfilling
        /// Reset the return code so as not to report a checkpoint failure
        ///* just because there are active readers.
        /// If this is an SQLITE_CHECKPOINT_RESTART or TRUNCATE operation, and the
        ///* entire wal file has been copied into the database file, then block
        ///* until all readers have finished using the wal file. This ensures that
        ///* the next process to write to the database restarts the wal file.
        /// IMPLEMENTATION-OF: R-44699-57140 This mode works the same way as
        ///* SQLITE_CHECKPOINT_RESTART with the addition that it also
        ///* truncates the log file to zero bytes just prior to a
        ///* successful return.
        ///*
        ///* In theory, it might be safe to do this without updating the
        ///* wal-index header in shared memory, as all subsequent reader or
        ///* writer clients should see that the entire log file has been
        ///* checkpointed and behave accordingly. This seems unsafe though,
        ///* as it would leave the system in a state where the contents of
        ///* the wal-index header do not match the contents of the
        ///* file-system. To avoid this, update the wal-index header to
        ///* indicate that the log file contains zero valid frames.
        unreachable!();
    }
}

///* End a write transaction.  The commit has already been done.  This
///* routine merely releases the lock.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_end_write_transaction(p_wal: *mut Wal) -> i32 {
    if unsafe { (*p_wal).write_lock } != 0 {
        wal_unlock_exclusive(unsafe { &*p_wal }, 0, 1);
        unsafe { (*p_wal).write_lock = 0 as u8 };
        unsafe { (*p_wal).i_re_cksum = 0 as u32 };
        unsafe { (*p_wal).truncate_on_commit = 0 as u8 };
    }
    return 0;
}

/// Copy pages from the log to the database file
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3WalCheckpoint(p_wal: *mut Wal, db: *mut Sqlite3,
    e_mode: i32, x_busy: unsafe extern "C" fn(*mut ()) -> i32,
    p_busy_arg: *mut (), sync_flags: i32, n_buf: i32, z_buf: *mut u8,
    pn_log: *mut i32, pn_ckpt: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut is_changed: i32 = 0;
    /// True if a new wal-index header is loaded
    let mut e_mode2: i32 = e_mode;
    /// Mode to pass to walCheckpoint()
    let mut x_busy2: Option<unsafe extern "C" fn(*mut ()) -> i32> =
        Some(x_busy);

    /// Busy handler for eMode2
    { let _ = 0; };
    { let _ = 0; };

    /// EVIDENCE-OF: R-62920-47450 The busy-handler callback is never invoked
    ///* in the SQLITE_CHECKPOINT_PASSIVE mode.
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_wal).read_only } != 0 { return 8; }
    if x_busy2.is_some() { { let _ = 0; }; }
    if e_mode != -1 {
        rc = wal_lock_exclusive(unsafe { &*p_wal }, 1, 1);
        if rc == 0 {
            unsafe { (*p_wal).ckpt_lock = 1 as u8 };
            if e_mode != 0 {
                rc = wal_busy_lock(p_wal, x_busy2, p_busy_arg, 0, 1);
                if rc == 0 {
                    unsafe { (*p_wal).write_lock = 1 as u8 };
                } else if rc == 5 { e_mode2 = 0; x_busy2 = None; rc = 0; }
            }
        }
    } else { rc = 0; }
    {
        if rc == 0 {
            rc = wal_index_read_hdr(p_wal, &mut is_changed);
            if e_mode2 > 0 { { let _ = 0; }; }
            if is_changed != 0 &&
                    unsafe {
                                (*unsafe {
                                                (*unsafe { (*p_wal).p_db_fd }).p_methods
                                            }).i_version
                            } as i32 >= 3 {
                unsafe {
                    sqlite3_os_unfetch(unsafe { (*p_wal).p_db_fd }, 0 as i64,
                        core::ptr::null_mut())
                };
            }
        }
        if rc == 0 {
            unsafe { sqlite3_fault_sim(660) };
            if unsafe { (*p_wal).hdr.mx_frame } != 0 &&
                    wal_pagesize(unsafe { &*p_wal }) != n_buf {
                rc = unsafe { sqlite3_corrupt_error(4393) };
            } else if e_mode2 != -1 {
                rc =
                    wal_checkpoint(p_wal, unsafe { &mut *db }, e_mode2, x_busy2,
                        p_busy_arg, sync_flags, z_buf);
            }
            if rc == 0 || rc == 5 {
                if !(pn_log).is_null() {
                    unsafe {
                        *pn_log = unsafe { (*p_wal).hdr.mx_frame } as i32
                    };
                }
                { let _ = 0; };
                if !(pn_ckpt).is_null() {
                    unsafe {
                        *pn_ckpt =
                            unsafe { (*wal_ckpt_info(unsafe { &*p_wal })).n_backfill }
                                as i32
                    };
                }
            }
        }
    }
    { let _ = 0; };
    if is_changed != 0 {

        /// If a new wal-index header was loaded before the checkpoint was
        ///* performed, then the pager-cache associated with pWal is now
        ///* out of date. So zero the cached wal-index header to ensure that
        ///* next time the pager opens a snapshot on this database it knows that
        ///* the cache needs to be reset.
        unsafe {
            memset(unsafe { &raw mut (*p_wal).hdr } as *mut (), 0,
                core::mem::size_of::<WalIndexHdr>() as u64)
        };
    }

    /// Release the locks.
    { let _ = sqlite3_wal_end_write_transaction(p_wal); };
    if unsafe { (*p_wal).ckpt_lock } != 0 {
        wal_unlock_exclusive(unsafe { &*p_wal }, 1, 1);
        unsafe { (*p_wal).ckpt_lock = 0 as u8 };
    }
    return if rc == 0 && e_mode != e_mode2 { 5 } else { rc };
}

///* If the WAL file is currently larger than nMax bytes in size, truncate
///* it to exactly nMax bytes. If an error occurs while doing so, ignore it.
extern "C" fn wal_limit_size(p_wal_1: &Wal, n_max_1: i64) -> () {
    let mut sz: i64 = 0 as i64;
    let mut rx: i32 = 0;
    unsafe { sqlite3_begin_benign_malloc() };
    rx = unsafe { sqlite3_os_file_size((*p_wal_1).p_wal_fd, &mut sz) };
    if rx == 0 && sz > n_max_1 {
        rx = unsafe { sqlite3_os_truncate((*p_wal_1).p_wal_fd, n_max_1) };
    }
    unsafe { sqlite3_end_benign_malloc() };
    if rx != 0 {
        unsafe {
            sqlite3_log(rx,
                c"cannot limit WAL size: %s".as_ptr() as *mut i8 as *const i8,
                (*p_wal_1).z_wal_name)
        };
    }
}

///* Close a connection to a log file.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_close(p_wal: *mut Wal, db: *mut Sqlite3,
    sync_flags: i32, n_buf: i32, z_buf: *mut u8) -> i32 {
    let mut rc: i32 = 0;
    if !(p_wal).is_null() {
        let mut is_delete: i32 = 0;

        /// True to unlink wal and wal-index files
        { let _ = 0; };
        if z_buf != core::ptr::null_mut() &&
                0 ==
                    {
                        rc =
                            unsafe { sqlite3_os_lock(unsafe { (*p_wal).p_db_fd }, 4) };
                        rc
                    } {
            if unsafe { (*p_wal).exclusive_mode } as i32 == 0 {
                unsafe { (*p_wal).exclusive_mode = 1 as u8 };
            }
            rc =
                sqlite3WalCheckpoint(p_wal, db, 0,
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ()) -> i32>(0 as *const ())
                    }, core::ptr::null_mut(), sync_flags, n_buf, z_buf,
                    core::ptr::null_mut(), core::ptr::null_mut());
            if rc == 0 {
                let mut b_persist: i32 = -1;
                unsafe {
                    sqlite3_os_file_control_hint(unsafe { (*p_wal).p_db_fd },
                        10, &raw mut b_persist as *mut ())
                };
                if b_persist != 1 {

                    /// Try to delete the WAL file if the checkpoint completed and
                    ///* fsynced (rc==SQLITE_OK) and if we are not in persistent-wal
                    ///* mode (!bPersist)
                    (is_delete = 1);
                } else if unsafe { (*p_wal).mx_wal_size } >= 0 as i64 {

                    /// Try to truncate the WAL file to zero bytes if the checkpoint
                    ///* completed and fsynced (rc==SQLITE_OK) and we are in persistent
                    ///* WAL mode (bPersist) and if the PRAGMA journal_size_limit is a
                    ///* non-negative value (pWal->mxWalSize>=0).  Note that we truncate
                    ///* to zero bytes as truncating to the journal_size_limit might
                    ///* leave a corrupt WAL file on disk.
                    wal_limit_size(unsafe { &*p_wal }, 0 as i64);
                }
            }
        }
        wal_index_close(unsafe { &*p_wal }, is_delete);
        unsafe { sqlite3_os_close(unsafe { (*p_wal).p_wal_fd }) };
        if is_delete != 0 {
            unsafe { sqlite3_begin_benign_malloc() };
            unsafe {
                sqlite3_os_delete(unsafe { (*p_wal).p_vfs },
                    unsafe { (*p_wal).z_wal_name }, 0)
            };
            unsafe { sqlite3_end_benign_malloc() };
        }
        unsafe { sqlite3_free(unsafe { (*p_wal).ap_wi_data } as *mut ()) };
        unsafe { sqlite3_free(p_wal as *mut ()) };
    }
    return rc;
}

/// Set the limiting size of a WAL file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_limit(p_wal: *mut Wal, i_limit: i64) -> () {
    if !(p_wal).is_null() { unsafe { (*p_wal).mx_wal_size = i_limit }; }
}

///* Finish with a read transaction.  All this does is release the
///* read-lock.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_end_read_transaction(p_wal: *mut Wal) -> () {
    { let _ = 0; };
    if unsafe { (*p_wal).read_lock } as i32 >= 0 {
        { let _ = sqlite3_wal_end_write_transaction(p_wal); };
        wal_unlock_shared(unsafe { &*p_wal },
            3 + unsafe { (*p_wal).read_lock } as i32);
        unsafe { (*p_wal).read_lock = -1 as i16 };
    }
}

///* Open a transaction in a connection where the shared-memory is read-only
///* and where we cannot verify that there is a separate write-capable connection
///* on hand to keep the shared-memory up-to-date with the WAL file.
///*
///* This can happen, for example, when the shared-memory is implemented by
///* memory-mapping a *-shm file, where a prior writer has shut down and
///* left the *-shm file on disk, and now the present connection is trying
///* to use that database but lacks write permission on the *-shm file.
///* Other scenarios are also possible, depending on the VFS implementation.
///*
///* Precondition:
///*
///*    The *-wal file has been read and an appropriate wal-index has been
///*    constructed in pWal->apWiData[] using heap memory instead of shared
///*    memory.
///*
///* If this function returns SQLITE_OK, then the read transaction has
///* been successfully opened. In this case output variable (*pChanged)
///* is set to true before returning if the caller should discard the
///* contents of the page cache before proceeding. Or, if it returns
///* WAL_RETRY, then the heap memory wal-index has been discarded and
///* the caller should retry opening the read transaction from the
///* beginning (including attempting to map the *-shm file).
///*
///* If an error occurs, an SQLite error code is returned.
#[allow(unused_doc_comments)]
extern "C" fn wal_begin_shm_unreliable(p_wal_1: *mut Wal,
    p_changed_1: &mut i32) -> i32 {
    let mut sz_wal: i64 = 0 as i64;
    /// Size of wal file on disk in bytes
    let mut i_offset: i64 = 0 as i64;
    /// Current offset when reading wal file
    let mut a_buf: [u8; 32] = [0; 32];
    /// Buffer to load WAL header into
    let mut a_frame: *mut u8 = core::ptr::null_mut();
    /// Malloc'd buffer to load entire frame
    let mut sz_frame: i32 = 0;
    /// Number of bytes in buffer aFrame[]
    let mut a_data: *mut u8 = core::ptr::null_mut();
    /// Pointer to data part of aFrame buffer
    let mut p_dummy: *mut () = core::ptr::null_mut();
    /// Dummy argument for xShmMap
    let mut rc: i32 = 0;
    /// Return code
    let mut a_save_cksum: [u32; 2] = [0; 2];
    /// Saved copy of pWal->hdr.aFrameCksum
    /// Take WAL_READ_LOCK(0). This has the effect of preventing any
    ///* writers from running a checkpoint, but does not stop them
    ///* from running recovery.
    /// Check to see if a separate writer has attached to the shared-memory area,
    ///* thus making the shared-memory "reliable" again.  Do this by invoking
    ///* the xShmMap() routine of the VFS and looking to see if the return
    ///* is SQLITE_READONLY instead of SQLITE_READONLY_CANTINIT.
    ///*
    ///* If the shared-memory is now "reliable" return WAL_RETRY, which will
    ///* cause the heap-memory WAL-index to be discarded and the actual
    ///* shared memory to be used in its place.
    ///*
    ///* This step is important because, even though this connection is holding
    ///* the WAL_READ_LOCK(0) which prevents a checkpoint, a writer might
    ///* have already checkpointed the WAL file and, while the current
    ///* is active, wrap the WAL and start overwriting frames that this
    ///* process wants to use.
    ///*
    ///* Once sqlite3OsShmMap() has been called for an sqlite3_file and has
    ///* returned any SQLITE_READONLY value, it must return only SQLITE_READONLY
    ///* or SQLITE_READONLY_CANTINIT or some error for all subsequent invocations,
    ///* even if some external agent does a "chmod" to make the shared-memory
    ///* writable by us, until sqlite3OsShmUnmap() has been called.
    ///* This is a requirement on the VFS implementation.
    /// SQLITE_OK not possible for read-only connection
    /// We reach this point only if the real shared-memory is still unreliable.
    ///* Assume the in-memory WAL-index substitute is correct and load it
    ///* into pWal->hdr.
    /// Make sure some writer hasn't come in and changed the WAL file out
    ///* from under us, then disconnected, while we were not looking.
    /// If the wal file is too small to contain a wal-header and the
    ///* wal-index header has mxFrame==0, then it must be safe to proceed
    ///* reading the database file only. However, the page cache cannot
    ///* be trusted, as a read/write connection may have connected, written
    ///* the db, run a checkpoint, truncated the wal file and disconnected
    ///* since this client's last read transaction.
    /// Check the salt keys at the start of the wal file still match.
    /// Some writer has wrapped the WAL file while we were not looking.
    ///* Return WAL_RETRY which will cause the in-memory WAL-index to be
    ///* rebuilt.
    /// Allocate a buffer to read frames into
    /// Check to see if a complete transaction has been appended to the
    ///* wal file since the heap-memory wal-index was created. If so, the
    ///* heap-memory wal-index is discarded and WAL_RETRY returned to
    ///* the caller.
    let mut pgno: u32 = 0 as u32;
    /// Database page number for frame
    let mut n_truncate: u32 = 0 as u32;
    /// dbsize field from frame header
    /// Read and decode the next log frame.
    /// If nTruncate is non-zero, then a complete transaction has been
    ///* appended to this wal file. Set rc to WAL_RETRY and break out of
    ///* the loop.
    let mut i: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s21:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe { sqlite3_free(a_frame as *mut ()) };
                    __state = 65;
                }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { a_frame = core::ptr::null_mut(); __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { __state = 9; }
                9 => { __state = 10; }
                10 => { __state = 11; }
                11 => { { let _ = 0; }; __state = 12; }
                12 => { { let _ = 0; }; __state = 13; }
                13 => { { let _ = 0; }; __state = 14; }
                14 => {
                    rc = wal_lock_shared(unsafe { &*p_wal_1 }, 3 + 0);
                    __state = 15;
                }
                15 => { if rc != 0 { __state = 17; } else { __state = 16; } }
                16 => {
                    unsafe { (*p_wal_1).read_lock = 0 as i16 };
                    __state = 20;
                }
                17 => { if rc == 5 { __state = 19; } else { __state = 18; } }
                18 => { __state = 2; }
                19 => { rc = -1; __state = 18; }
                20 => {
                    rc =
                        unsafe {
                            sqlite3_os_shm_map(unsafe { (*p_wal_1).p_db_fd }, 0,
                                (core::mem::size_of::<HtSlot>() as u64 * (4096 * 2) as u64 +
                                        4096 as u64 * core::mem::size_of::<u32>() as u64) as i32, 0,
                                &mut p_dummy)
                        };
                    __state = 21;
                }
                21 => { { let _ = 0; }; __state = 22; }
                22 => {
                    if rc != 8 | 5 << 8 { __state = 24; } else { __state = 23; }
                }
                23 => {
                    unsafe {
                        memcpy(unsafe { &raw mut (*p_wal_1).hdr } as *mut (),
                            wal_index_hdr(unsafe { &*p_wal_1 }) as *mut () as *const (),
                            core::mem::size_of::<WalIndexHdr>() as u64)
                    };
                    __state = 26;
                }
                24 => { rc = if rc == 8 { -1 } else { rc }; __state = 25; }
                25 => { __state = 2; }
                26 => {
                    rc =
                        unsafe {
                            sqlite3_os_file_size(unsafe { (*p_wal_1).p_wal_fd },
                                &mut sz_wal)
                        };
                    __state = 27;
                }
                27 => { if rc != 0 { __state = 29; } else { __state = 28; } }
                28 => {
                    if sz_wal < 32 as i64 {
                        __state = 31;
                    } else { __state = 30; }
                }
                29 => { __state = 2; }
                30 => {
                    rc =
                        unsafe {
                            sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd },
                                &raw mut a_buf[0 as usize] as *mut u8 as *mut (), 32,
                                0 as i64)
                        };
                    __state = 34;
                }
                31 => { *p_changed_1 = 1; __state = 32; }
                32 => {
                    rc =
                        if unsafe { (*p_wal_1).hdr.mx_frame } == 0 as u32 {
                            0
                        } else { -1 };
                    __state = 33;
                }
                33 => { __state = 2; }
                34 => { if rc != 0 { __state = 36; } else { __state = 35; } }
                35 => {
                    if unsafe {
                                memcmp(unsafe { &raw mut (*p_wal_1).hdr.a_salt } as
                                        *const (), &raw mut a_buf[16 as usize] as *const (),
                                    8 as u64)
                            } != 0 {
                        __state = 38;
                    } else { __state = 37; }
                }
                36 => { __state = 2; }
                37 => { { let _ = 0; }; __state = 40; }
                38 => { rc = -1; __state = 39; }
                39 => { __state = 2; }
                40 => { { let _ = 0; }; __state = 41; }
                41 => {
                    sz_frame =
                        (unsafe { (*p_wal_1).sz_page } + 24 as u32) as i32;
                    __state = 42;
                }
                42 => {
                    a_frame =
                        unsafe { sqlite3_malloc64(sz_frame as Sqlite3Uint64) } as
                            *mut u8;
                    __state = 43;
                }
                43 => {
                    if a_frame == core::ptr::null_mut() {
                        __state = 45;
                    } else { __state = 44; }
                }
                44 => {
                    a_data = unsafe { a_frame.offset(24 as isize) };
                    __state = 47;
                }
                45 => { rc = 7; __state = 46; }
                46 => { __state = 2; }
                47 => {
                    a_save_cksum[0 as usize] =
                        unsafe { (*p_wal_1).hdr.a_frame_cksum[0 as usize] };
                    __state = 48;
                }
                48 => {
                    a_save_cksum[1 as usize] =
                        unsafe { (*p_wal_1).hdr.a_frame_cksum[1 as usize] };
                    __state = 49;
                }
                49 => {
                    i_offset =
                        32 as i64 +
                            (unsafe { (*p_wal_1).hdr.mx_frame } + 1 as u32 - 1 as u32)
                                    as i64 * (unsafe { (*p_wal_1).sz_page } + 24 as u32) as i64;
                    __state = 51;
                }
                50 => {
                    unsafe {
                        (*p_wal_1).hdr.a_frame_cksum[0 as usize] =
                            a_save_cksum[0 as usize]
                    };
                    __state = 63;
                }
                51 => {
                    if i_offset + sz_frame as i64 <= sz_wal {
                        __state = 52;
                    } else { __state = 50; }
                }
                52 => { __state = 54; }
                53 => { i_offset += sz_frame as i64; __state = 51; }
                54 => { __state = 55; }
                55 => {
                    rc =
                        unsafe {
                            sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd },
                                a_frame as *mut (), sz_frame, i_offset)
                        };
                    __state = 56;
                }
                56 => { if rc != 0 { __state = 58; } else { __state = 57; } }
                57 => {
                    if (wal_decode_frame(unsafe { &mut *p_wal_1 }, &mut pgno,
                                        &mut n_truncate, a_data, a_frame) == 0) as i32 != 0 {
                        __state = 60;
                    } else { __state = 59; }
                }
                58 => { __state = 50; }
                59 => {
                    if n_truncate != 0 { __state = 61; } else { __state = 53; }
                }
                60 => { __state = 50; }
                61 => { rc = -1; __state = 62; }
                62 => { __state = 50; }
                63 => {
                    unsafe {
                        (*p_wal_1).hdr.a_frame_cksum[1 as usize] =
                            a_save_cksum[1 as usize]
                    };
                    __state = 64;
                }
                64 => { __state = 2; }
                65 => { if rc != 0 { __state = 67; } else { __state = 66; } }
                66 => { return rc; }
                67 => { __state = 68; }
                68 => { i = 0; __state = 70; }
                69 => {
                    unsafe { (*p_wal_1).b_shm_unreliable = 0 as u8 };
                    __state = 74;
                }
                70 => {
                    if i < unsafe { (*p_wal_1).n_wi_data } {
                        __state = 71;
                    } else { __state = 69; }
                }
                71 => {
                    unsafe {
                        sqlite3_free(unsafe {
                                    *unsafe { (*p_wal_1).ap_wi_data.offset(i as isize) }
                                } as *mut ())
                    };
                    __state = 73;
                }
                72 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 70;
                }
                73 => {
                    unsafe {
                        *unsafe { (*p_wal_1).ap_wi_data.offset(i as isize) } =
                            core::ptr::null_mut()
                    };
                    __state = 72;
                }
                74 => {
                    sqlite3_wal_end_read_transaction(p_wal_1);
                    __state = 75;
                }
                75 => { *p_changed_1 = 1; __state = 66; }
                _ => {}
            }
        }
    }

    /// Size of wal file on disk in bytes
    /// Current offset when reading wal file
    /// Buffer to load WAL header into
    /// Malloc'd buffer to load entire frame
    /// Number of bytes in buffer aFrame[]
    /// Pointer to data part of aFrame buffer
    /// Dummy argument for xShmMap
    /// Return code
    /// Saved copy of pWal->hdr.aFrameCksum
    /// Take WAL_READ_LOCK(0). This has the effect of preventing any
    ///* writers from running a checkpoint, but does not stop them
    ///* from running recovery.
    /// Check to see if a separate writer has attached to the shared-memory area,
    ///* thus making the shared-memory "reliable" again.  Do this by invoking
    ///* the xShmMap() routine of the VFS and looking to see if the return
    ///* is SQLITE_READONLY instead of SQLITE_READONLY_CANTINIT.
    ///*
    ///* If the shared-memory is now "reliable" return WAL_RETRY, which will
    ///* cause the heap-memory WAL-index to be discarded and the actual
    ///* shared memory to be used in its place.
    ///*
    ///* This step is important because, even though this connection is holding
    ///* the WAL_READ_LOCK(0) which prevents a checkpoint, a writer might
    ///* have already checkpointed the WAL file and, while the current
    ///* is active, wrap the WAL and start overwriting frames that this
    ///* process wants to use.
    ///*
    ///* Once sqlite3OsShmMap() has been called for an sqlite3_file and has
    ///* returned any SQLITE_READONLY value, it must return only SQLITE_READONLY
    ///* or SQLITE_READONLY_CANTINIT or some error for all subsequent invocations,
    ///* even if some external agent does a "chmod" to make the shared-memory
    ///* writable by us, until sqlite3OsShmUnmap() has been called.
    ///* This is a requirement on the VFS implementation.
    /// SQLITE_OK not possible for read-only connection
    /// We reach this point only if the real shared-memory is still unreliable.
    ///* Assume the in-memory WAL-index substitute is correct and load it
    ///* into pWal->hdr.
    /// Make sure some writer hasn't come in and changed the WAL file out
    ///* from under us, then disconnected, while we were not looking.
    /// If the wal file is too small to contain a wal-header and the
    ///* wal-index header has mxFrame==0, then it must be safe to proceed
    ///* reading the database file only. However, the page cache cannot
    ///* be trusted, as a read/write connection may have connected, written
    ///* the db, run a checkpoint, truncated the wal file and disconnected
    ///* since this client's last read transaction.
    /// Check the salt keys at the start of the wal file still match.
    /// Some writer has wrapped the WAL file while we were not looking.
    ///* Return WAL_RETRY which will cause the in-memory WAL-index to be
    ///* rebuilt.
    /// Allocate a buffer to read frames into
    /// Check to see if a complete transaction has been appended to the
    ///* wal file since the heap-memory wal-index was created. If so, the
    ///* heap-memory wal-index is discarded and WAL_RETRY returned to
    ///* the caller.
    /// Database page number for frame
    /// dbsize field from frame header
    /// Read and decode the next log frame.
    /// If nTruncate is non-zero, then a complete transaction has been
    ///* appended to this wal file. Set rc to WAL_RETRY and break out of
    ///* the loop.
    unreachable!();
}

///* Attempt to start a read transaction.  This might fail due to a race or
///* other transient condition.  When that happens, it returns WAL_RETRY to
///* indicate to the caller that it is safe to retry immediately.
///*
///* On success return SQLITE_OK.  On a permanent failure (such an
///* I/O error or an SQLITE_BUSY because another process is running
///* recovery) return a positive error code.
///*
///* The useWal parameter is true to force the use of the WAL and disable
///* the case where the WAL is bypassed because it has been completely
///* checkpointed.  If useWal==0 then this routine calls walIndexReadHdr()
///* to make a copy of the wal-index header into pWal->hdr.  If the
///* wal-index header has changed, *pChanged is set to 1 (as an indication
///* to the caller that the local page cache is obsolete and needs to be
///* flushed.)  When useWal==1, the wal-index header is assumed to already
///* be loaded and the pChanged parameter is unused.
///*
///* The caller must set the cnt parameter to the number of prior calls to
///* this routine during the current read attempt that returned WAL_RETRY.
///* This routine will start taking more aggressive measures to clear the
///* race conditions after multiple WAL_RETRY returns, and after an excessive
///* number of errors will ultimately return SQLITE_PROTOCOL.  The
///* SQLITE_PROTOCOL return indicates that some other process has gone rogue
///* and is not honoring the locking protocol.  There is a vanishingly small
///* chance that SQLITE_PROTOCOL could be returned because of a run of really
///* bad luck when there is lots of contention for the wal-index, but that
///* possibility is so small that it can be safely neglected, we believe.
///*
///* On success, this routine obtains a read lock on
///* WAL_READ_LOCK(pWal->readLock).  The pWal->readLock integer is
///* in the range 0 <= pWal->readLock < WAL_NREADER.  If pWal->readLock==(-1)
///* that means the Wal does not hold any read lock.  The reader must not
///* access any database page that is modified by a WAL frame up to and
///* including frame number aReadMark[pWal->readLock].  The reader will
///* use WAL frames up to and including pWal->hdr.mxFrame if pWal->readLock>0
///* Or if pWal->readLock==0, then the reader will ignore the WAL
///* completely and get all content directly from the database file.
///* If the useWal parameter is 1 then the WAL will never be ignored and
///* this routine will always set pWal->readLock>0 on success.
///* When the read transaction is completed, the caller must release the
///* lock on WAL_READ_LOCK(pWal->readLock) and set pWal->readLock to -1.
///*
///* This routine uses the nBackfill and aReadMark[] fields of the header
///* to select a particular WAL_READ_LOCK() that strives to let the
///* checkpoint process do as much work as possible.  This routine might
///* update values of the aReadMark[] array in the header, but if it does
///* so it takes care to hold an exclusive lock on the corresponding
///* WAL_READ_LOCK() while changing values.
#[allow(unused_doc_comments)]
extern "C" fn wal_try_begin_read(p_wal_1: *mut Wal, p_changed_1: *mut i32,
    use_wal_1: i32, p_cnt_1: &mut i32) -> i32 {
    let mut p_info: *mut WalCkptInfo = core::ptr::null_mut();
    /// Checkpoint information in wal-index
    let mut rc: i32 = 0;

    /// Return code
    { let _ = 0; };

    /// Not currently locked
    /// useWal may only be set for read/write connections
    { let _ = 0; };

    /// Take steps to avoid spinning forever if there is a protocol error.
    ///*
    ///* Circumstances that cause a RETRY should only last for the briefest
    ///* instances of time.  No I/O or other system calls are done while the
    ///* locks are held, so the locks should not be held for very long. But
    ///* if we are unlucky, another process that is holding a lock might get
    ///* paged out or take a page-fault that is time-consuming to resolve,
    ///* during the few nanoseconds that it is holding the lock.  In that case,
    ///* it might take longer than normal for the lock to free.
    ///*
    ///* After 5 RETRYs, we begin calling sqlite3OsSleep().  The first few
    ///* calls to sqlite3OsSleep() have a delay of 1 microsecond.  Really this
    ///* is more of a scheduler yield than an actual delay.  But on the 10th
    ///* an subsequent retries, the delays start becoming longer and longer,
    ///* so that on the 100th (and last) RETRY we delay for 323 milliseconds.
    ///* The total delay time before giving up is less than 10 seconds.
    { let __p = &mut *p_cnt_1; let __t = *__p; *__p += 1; __t };
    if *p_cnt_1 > 5 {
        let mut n_delay: i32 = 1;
        /// Pause time in microseconds
        let cnt: i32 = *p_cnt_1 & !0;
        if cnt > 100 { return 15; }
        if *p_cnt_1 >= 10 { n_delay = (cnt - 9) * (cnt - 9) * 39; }
        unsafe { sqlite3_os_sleep(unsafe { (*p_wal_1).p_vfs }, n_delay) };
        *p_cnt_1 &= !0;
    }
    if (use_wal_1 == 0) as i32 != 0 {
        { let _ = 0; };
        if unsafe { (*p_wal_1).b_shm_unreliable } as i32 == 0 {
            rc = wal_index_read_hdr(p_wal_1, p_changed_1);
        }
        if rc == 5 {

            /// If there is not a recovery running in another thread or process
            ///* then convert BUSY errors to WAL_RETRY.  If recovery is known to
            ///* be running, convert BUSY to BUSY_RECOVERY.  There is a race here
            ///* which might cause WAL_RETRY to be returned even if BUSY_RECOVERY
            ///* would be technically correct.  But the race is benign since with
            ///* WAL_RETRY this routine will be called again and will probably be
            ///* right on the second iteration.
            { let _ = 0; };
            if unsafe { *unsafe { (*p_wal_1).ap_wi_data.offset(0 as isize) } }
                    == core::ptr::null_mut() {

                /// This branch is taken when the xShmMap() method returns SQLITE_BUSY.
                ///* We assume this is a transient condition, so return WAL_RETRY. The
                ///* xShmMap() implementation used by the default unix and win32 VFS
                ///* modules may return SQLITE_BUSY due to a race condition in the
                ///* code that determines whether or not the shared-memory region
                ///* must be zeroed before the requested page is returned.
                (rc = -1);
            } else if 0 ==
                    { rc = wal_lock_shared(unsafe { &*p_wal_1 }, 2); rc } {
                wal_unlock_shared(unsafe { &*p_wal_1 }, 2);
                rc = -1;
            } else if rc == 5 { rc = 5 | 1 << 8; }
        }
        if rc != 0 {
            return rc;
        } else if unsafe { (*p_wal_1).b_shm_unreliable } != 0 {
            return wal_begin_shm_unreliable(p_wal_1,
                    unsafe { &mut *p_changed_1 });
        }
    }
    { let _ = 0; };
    { let _ = 0; };
    p_info = wal_ckpt_info(unsafe { &*p_wal_1 });
    { let _ = 0; };
    {
        let mut mx_read_mark: u32 = 0 as u32;
        /// Largest aReadMark[] value
        let mut mx_i: i32 = 0;
        /// Index of largest aReadMark[] value
        let mut i: i32 = 0;
        /// Loop counter
        let mut mx_frame: u32 = 0 as u32;
        if (use_wal_1 == 0) as i32 != 0 &&
                unsafe {
                        std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                        &raw mut (*p_info).n_backfill
                                    } as *mut u32).load(std::sync::atomic::Ordering::Relaxed)
                    } == unsafe { (*p_wal_1).hdr.mx_frame } {

            /// The WAL has been completely backfilled (or it is empty).
            ///* and can be safely ignored.
            (rc = wal_lock_shared(unsafe { &*p_wal_1 }, 3 + 0));
            wal_shm_barrier(unsafe { &*p_wal_1 });
            if rc == 0 {
                if unsafe {
                            memcmp(wal_index_hdr(unsafe { &*p_wal_1 }) as *mut () as
                                    *const (), unsafe { &raw mut (*p_wal_1).hdr } as *const (),
                                core::mem::size_of::<WalIndexHdr>() as u64)
                        } != 0 {

                    /// It is not safe to allow the reader to continue here if frames
                    ///* may have been appended to the log before READ_LOCK(0) was obtained.
                    ///* When holding READ_LOCK(0), the reader ignores the entire log file,
                    ///* which implies that the database file contains a trustworthy
                    ///* snapshot. Since holding READ_LOCK(0) prevents a checkpoint from
                    ///* happening, this is usually correct.
                    ///*
                    ///* However, if frames have been appended to the log (or if the log
                    ///* is wrapped and written for that matter) before the READ_LOCK(0)
                    ///* is obtained, that is not necessarily true. A checkpointer may
                    ///* have started to backfill the appended frames but crashed before
                    ///* it finished. Leaving a corrupt image in the database file.
                    wal_unlock_shared(unsafe { &*p_wal_1 }, 3 + 0);
                    return -1;
                }
                unsafe { (*p_wal_1).read_lock = 0 as i16 };
                return 0;
            } else if rc != 5 { return rc; }
        }

        /// If we get this far, it means that the reader will want to use
        ///* the WAL to get at content from recent commits.  The job now is
        ///* to select one of the aReadMark[] entries that is closest to
        ///* but not exceeding pWal->hdr.mxFrame and lock that entry.
        (mx_read_mark = 0 as u32);
        mx_i = 0;
        mx_frame = unsafe { (*p_wal_1).hdr.mx_frame };
        {
            i = 1;
            '__b22: loop {
                if !(i < 8 - 3) { break '__b22; }
                '__c22: loop {
                    let this_mark: u32 =
                        unsafe {
                            std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                            (unsafe { &raw mut (*p_info).a_read_mark[0 as usize] } as
                                                    *mut u32).offset(i as isize)
                                        } as *mut u32).load(std::sync::atomic::Ordering::Relaxed)
                        };
                    { let _ = 0; };
                    if mx_read_mark <= this_mark && this_mark <= mx_frame {
                        { let _ = 0; };
                        mx_read_mark = this_mark;
                        mx_i = i;
                    }
                    break '__c22;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_wal_1).read_only } as i32 & 2 == 0 &&
                (mx_read_mark < mx_frame || mx_i == 0) {
            {
                i = 1;
                '__b23: loop {
                    if !(i < 8 - 3) { break '__b23; }
                    '__c23: loop {
                        rc = wal_lock_exclusive(unsafe { &*p_wal_1 }, 3 + i, 1);
                        if rc == 0 {
                            unsafe {
                                std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                                (unsafe { &raw mut (*p_info).a_read_mark[0 as usize] } as
                                                        *mut u32).offset(i as isize)
                                            } as
                                            *mut u32).store(mx_frame as u32,
                                    std::sync::atomic::Ordering::Relaxed)
                            };
                            mx_read_mark = mx_frame;
                            mx_i = i;
                            wal_unlock_exclusive(unsafe { &*p_wal_1 }, 3 + i, 1);
                            break '__b23;
                        } else if rc != 5 { return rc; }
                        break '__c23;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if mx_i == 0 {
            { let _ = 0; };
            return if rc == 5 { -1 } else { 8 | 5 << 8 };
        }
        { let _ = 0; };
        rc = wal_lock_shared(unsafe { &*p_wal_1 }, 3 + mx_i);
        if rc != 0 {
            { let _ = 0; };
            { let _ = 0; };
            return if rc & 255 == 5 { -1 } else { rc };
        }

        /// Now that the read-lock has been obtained, check that neither the
        ///* value in the aReadMark[] array or the contents of the wal-index
        ///* header have changed.
        ///*
        ///* It is necessary to check that the wal-index header did not change
        ///* between the time it was read and when the shared-lock was obtained
        ///* on WAL_READ_LOCK(mxI) was obtained to account for the possibility
        ///* that the log file may have been wrapped by a writer, or that frames
        ///* that occur later in the log than pWal->hdr.mxFrame may have been
        ///* copied into the database by a checkpointer. If either of these things
        ///* happened, then reading the database with the current value of
        ///* pWal->hdr.mxFrame risks reading a corrupted snapshot. So, retry
        ///* instead.
        ///*
        ///* Before checking that the live wal-index header has not changed
        ///* since it was read, set Wal.minFrame to the first frame in the wal
        ///* file that has not yet been checkpointed. This client will not need
        ///* to read any frames earlier than minFrame from the wal file - they
        ///* can be safely read directly from the database file.
        ///*
        ///* Because a ShmBarrier() call is made between taking the copy of
        ///* nBackfill and checking that the wal-header in shared-memory still
        ///* matches the one cached in pWal->hdr, it is guaranteed that the
        ///* checkpointer that set nBackfill was not working with a wal-index
        ///* header newer than that cached in pWal->hdr. If it were, that could
        ///* cause a problem. The checkpointer could omit to checkpoint
        ///* a version of page X that lies before pWal->minFrame (call that version
        ///* A) on the basis that there is a newer version (version B) of the same
        ///* page later in the wal file. But if version B happens to like past
        ///* frame pWal->hdr.mxFrame - then the client would incorrectly assume
        ///* that it can read version A from the database file. However, since
        ///* we can guarantee that the checkpointer that set nBackfill could not
        ///* see any pages past pWal->hdr.mxFrame, this problem does not come up.
        unsafe {
            (*p_wal_1).min_frame =
                unsafe {
                        std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                        &raw mut (*p_info).n_backfill
                                    } as *mut u32).load(std::sync::atomic::Ordering::Relaxed)
                    } + 1 as u32
        };
        { let _ = 0; };
        wal_shm_barrier(unsafe { &*p_wal_1 });
        if unsafe {
                        std::sync::atomic::AtomicU32::from_ptr(unsafe {
                                        (unsafe { &raw mut (*p_info).a_read_mark[0 as usize] } as
                                                *mut u32).offset(mx_i as isize)
                                    } as *mut u32).load(std::sync::atomic::Ordering::Relaxed)
                    } != mx_read_mark ||
                unsafe {
                        memcmp(wal_index_hdr(unsafe { &*p_wal_1 }) as *mut () as
                                *const (), unsafe { &raw mut (*p_wal_1).hdr } as *const (),
                            core::mem::size_of::<WalIndexHdr>() as u64)
                    } != 0 {
            wal_unlock_shared(unsafe { &*p_wal_1 }, 3 + mx_i);
            return -1;
        } else {
            { let _ = 0; };
            unsafe { (*p_wal_1).read_lock = mx_i as i16 };
        }
    }
    return rc;
}

///* This function does the work of sqlite3WalBeginReadTransaction() (see 
///* below). That function simply calls this one inside an SEH_TRY{...} block.
#[allow(unused_doc_comments)]
extern "C" fn wal_begin_read_transaction(p_wal_1: *mut Wal,
    p_changed_1: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut cnt: i32 = 0;

    /// Number of TryBeginRead attempts
    { let _ = 0; };
    { let _ = 0; };
    '__b24: loop {
        '__c24: loop {
            rc = wal_try_begin_read(p_wal_1, p_changed_1, 0, &mut cnt);
            break '__c24;
        }
        if !(rc == -1) { break '__b24; }
    }
    return rc;
}

/// Used by readers to open (lock) and close (unlock) a snapshot.  A 
///* snapshot is like a read-transaction.  It is the state of the database
///* at an instant in time.  sqlite3WalOpenSnapshot gets a read lock and
///* preserves the current state even if the other threads or processes
///* write to or checkpoint the WAL.  sqlite3WalCloseSnapshot() closes the
///* transaction and releases the lock.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_begin_read_transaction(p_wal: *mut Wal,
    p_changed: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    { rc = wal_begin_read_transaction(p_wal, p_changed); }
    { let _ = 0; };
    return rc;
}

///* Search the wal file for page pgno. If found, set *piRead to the frame that
///* contains the page. Otherwise, if pgno is not in the wal file, set *piRead
///* to zero.
///*
///* Return SQLITE_OK if successful, or an error code if an error occurs. If an
///* error does occur, the final value of *piRead is undefined.
#[allow(unused_doc_comments)]
extern "C" fn wal_find_frame(p_wal_1: *mut Wal, pgno: Pgno,
    pi_read_1: &mut u32) -> i32 {
    let mut i_read: u32 = 0 as u32;
    /// If !=0, WAL frame to return data from
    let i_last: u32 = unsafe { (*p_wal_1).hdr.mx_frame };
    /// Last page in WAL for this reader
    let mut i_hash: i32 = 0;
    /// Used to loop through N hash tables
    let mut i_min_hash: i32 = 0;

    /// This routine is only be called from within a read transaction.
    { let _ = 0; };
    if i_last == 0 as u32 ||
            unsafe { (*p_wal_1).read_lock } as i32 == 0 &&
                unsafe { (*p_wal_1).b_shm_unreliable } as i32 == 0 {
        *pi_read_1 = 0 as u32;
        return 0;
    }

    /// Search the hash table or tables for an entry matching page number
    ///* pgno. Each iteration of the following for() loop searches one
    ///* hash table (each hash table indexes up to HASHTABLE_NPAGE frames).
    ///*
    ///* This code might run concurrently to the code in walIndexAppend()
    ///* that adds entries to the wal-index (and possibly to this hash
    ///* table). This means the value just read from the hash
    ///* slot (aHash[iKey]) may have been added before or after the
    ///* current read transaction was opened. Values added after the
    ///* read transaction was opened may have been written incorrectly -
    ///* i.e. these slots may contain garbage data. However, we assume
    ///* that any slots written before the current read transaction was
    ///* opened remain unmodified.
    ///*
    ///* For the reasons above, the if(...) condition featured in the inner
    ///* loop of the following block is more stringent that would be required
    ///* if we had exclusive access to the hash-table:
    ///*
    ///*   (aPgno[iFrame]==pgno):
    ///*     This condition filters out normal hash-table collisions.
    ///*
    ///*   (iFrame<=iLast):
    ///*     This condition filters out entries that were added to the hash
    ///*     table after the current read-transaction had started.
    (i_min_hash = wal_frame_page(unsafe { (*p_wal_1).min_frame }));
    {
        i_hash = wal_frame_page(i_last);
        '__b25: loop {
            if !(i_hash >= i_min_hash) { break '__b25; }
            '__c25: loop {
                let mut s_loc: WalHashLoc = unsafe { core::mem::zeroed() };
                /// Hash table location
                let mut i_key: i32 = 0;
                /// Hash slot index
                let mut n_collide: i32 = 0;
                /// Number of hash collisions remaining
                let mut rc: i32 = 0;
                /// Error code
                let mut i_h: u32 = 0 as u32;
                rc = wal_hash_get(p_wal_1, i_hash, &mut s_loc);
                if rc != 0 { return rc; }
                n_collide = 4096 * 2;
                i_key = wal_hash(pgno);
                { let _ = 0; };
                while {
                            i_h =
                                unsafe {
                                        std::sync::atomic::AtomicU16::from_ptr(unsafe {
                                                        &raw mut *s_loc.a_hash.offset(i_key as isize)
                                                    } as *mut u16).load(std::sync::atomic::Ordering::Relaxed)
                                    } as u32;
                            i_h
                        } != 0 as u32 {
                    let i_frame: u32 = i_h + s_loc.i_zero;
                    if i_frame <= i_last &&
                                i_frame >= unsafe { (*p_wal_1).min_frame } &&
                            unsafe {
                                        *s_loc.a_pgno.add((i_h - 1 as u32 & (4096 - 1) as u32) as
                                                    usize)
                                    } as u32 == pgno {
                        { let _ = 0; };
                        i_read = i_frame;
                    }
                    if {
                                let __p = &mut n_collide;
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            } == 0 {
                        *pi_read_1 = 0 as u32;
                        return unsafe { sqlite3_corrupt_error(3600) };
                    }
                    i_key = wal_next_hash(i_key);
                }
                if i_read != 0 { break '__b25; }
                break '__c25;
            }
            { let __p = &mut i_hash; let __t = *__p; *__p -= 1; __t };
        }
    }
    *pi_read_1 = i_read;
    return 0;
}

/// Read a page from the write-ahead log, if it is present.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_find_frame(p_wal: *mut Wal, pgno: Pgno,
    pi_read: *mut u32) -> i32 {
    let mut rc: i32 = 0;
    { rc = wal_find_frame(p_wal, pgno, unsafe { &mut *pi_read }); }
    { let _ = 0; };
    return rc;
}

///* Read the contents of frame iRead from the wal file into buffer pOut
///* (which is nOut bytes in size). Return SQLITE_OK if successful, or an
///* error code otherwise.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_read_frame(p_wal: &Wal, i_read: u32, n_out: i32,
    p_out: *mut u8) -> i32 {
    let mut sz: i32 = 0;
    let mut i_offset: i64 = 0 as i64;
    sz = (*p_wal).hdr.sz_page as i32;
    sz = (sz & 65024) + ((sz & 1) << 16);
    i_offset =
        32 as i64 + (i_read - 1 as u32) as i64 * (sz + 24) as i64 + 24 as i64;

    /// testcase( IS_BIG_INT(iOffset) ); // requires a 4GiB WAL
    return unsafe {
            sqlite3_os_read((*p_wal).p_wal_fd, p_out as *mut (),
                if n_out > sz { sz } else { n_out }, i_offset)
        };
}

/// If the WAL is not empty, return the size of the database.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_dbsize(p_wal: *mut Wal) -> Pgno {
    if !(p_wal).is_null() && unsafe { (*p_wal).read_lock } as i32 >= 0 {
        return unsafe { (*p_wal).hdr.n_page };
    }
    return 0 as Pgno;
}

/// Obtain or release the WRITER lock.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_begin_write_transaction(p_wal: *mut Wal)
    -> i32 {
    let mut rc: i32 = 0;

    /// Cannot start a write transaction without first holding a read
    ///* transaction.
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_wal).read_only } != 0 { return 8; }

    /// Only one writer allowed at a time.  Get the write lock.  Return
    ///* SQLITE_BUSY if unable.
    (rc = wal_lock_exclusive(unsafe { &*p_wal }, 0, 1));
    if rc != 0 { return rc; }
    unsafe { (*p_wal).write_lock = 1 as u8 };
    {
        if unsafe {
                    memcmp(unsafe { &raw mut (*p_wal).hdr } as *const (),
                        wal_index_hdr(unsafe { &*p_wal }) as *mut () as *const (),
                        core::mem::size_of::<WalIndexHdr>() as u64)
                } != 0 {
            rc = 5 | 2 << 8;
        }
    }
    { let _ = 0; };
    if rc != 0 {
        wal_unlock_exclusive(unsafe { &*p_wal }, 0, 1);
        unsafe { (*p_wal).write_lock = 0 as u8 };
    }
    return rc;
}

///* Return the page number associated with frame iFrame in this WAL.
extern "C" fn wal_frame_pgno(p_wal_1: &Wal, i_frame_1: u32) -> u32 {
    let i_hash: i32 = wal_frame_page(i_frame_1);
    { let _ = 0; };
    if i_hash == 0 {
        return unsafe {
                    *unsafe {
                            (*(*p_wal_1).ap_wi_data.offset(0 as
                                            isize)).add(((core::mem::size_of::<WalIndexHdr>() as u64 *
                                                        2 as u64 + core::mem::size_of::<WalCkptInfo>() as u64) /
                                                core::mem::size_of::<u32>() as u64 + i_frame_1 as u64 -
                                        1 as u64) as usize)
                        }
                } as u32;
    }
    return unsafe {
                *unsafe {
                        (*(*p_wal_1).ap_wi_data.offset(i_hash as
                                        isize)).add((((i_frame_1 - 1 as u32) as u64 -
                                        (4096 as u64 -
                                            (core::mem::size_of::<WalIndexHdr>() as u64 * 2 as u64 +
                                                    core::mem::size_of::<WalCkptInfo>() as u64) /
                                                core::mem::size_of::<u32>() as u64)) % 4096 as u64) as
                                usize)
                    }
            } as u32;
}

/// Undo any frames written (but not committed) to the log
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_undo(p_wal: *mut Wal,
    x_undo: Option<unsafe extern "C" fn(*mut (), u32) -> i32>,
    p_undo_ctx: *mut ()) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p_wal).write_lock } != 0 {
        let i_max: Pgno = unsafe { (*p_wal).hdr.mx_frame };
        let mut i_frame: Pgno = 0 as Pgno;
        {

            /// Restore the clients cache of the wal-index header to the state it
            ///* was in before the client began writing to the database.
            unsafe {
                memcpy(unsafe { &raw mut (*p_wal).hdr } as *mut (),
                    wal_index_hdr(unsafe { &*p_wal }) as *mut () as *const (),
                    core::mem::size_of::<WalIndexHdr>() as u64)
            };
            {
                i_frame = unsafe { (*p_wal).hdr.mx_frame } + 1 as u32;
                '__b27: loop {
                    if !(rc == 0 && i_frame <= i_max) { break '__b27; }
                    '__c27: loop {

                        /// This call cannot fail. Unless the page for which the page number
                        ///* is passed as the second argument is (a) in the cache and
                        ///* (b) has an outstanding reference, then xUndo is either a no-op
                        ///* (if (a) is false) or simply expels the page from the cache (if (b)
                        ///* is false).
                        ///*
                        ///* If the upper layer is doing a rollback, it is guaranteed that there
                        ///* are no outstanding references to any page other than page 1. And
                        ///* page 1 is never written to the log until the transaction is
                        ///* committed. As a result, the call to xUndo may not fail.
                        { let _ = 0; };
                        rc =
                            unsafe {
                                x_undo.unwrap()(p_undo_ctx,
                                    wal_frame_pgno(unsafe { &*p_wal }, i_frame))
                            };
                        break '__c27;
                    }
                    { let __p = &mut i_frame; let __t = *__p; *__p += 1; __t };
                }
            }
            if i_max != unsafe { (*p_wal).hdr.mx_frame } {
                wal_cleanup_hash(p_wal);
            }
        }
        { let _ = 0; };
        unsafe { (*p_wal).i_re_cksum = 0 as u32 };
    }
    return rc;
}

/// Return an integer that records the current (uncommitted) write
///* position in the WAL
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_savepoint(p_wal: &Wal, a_wal_data: *mut u32)
    -> () {
    { let _ = 0; };
    unsafe { *a_wal_data.offset(0 as isize) = (*p_wal).hdr.mx_frame };
    unsafe {
        *a_wal_data.offset(1 as isize) =
            (*p_wal).hdr.a_frame_cksum[0 as usize]
    };
    unsafe {
        *a_wal_data.offset(2 as isize) =
            (*p_wal).hdr.a_frame_cksum[1 as usize]
    };
    unsafe { *a_wal_data.offset(3 as isize) = (*p_wal).n_ckpt };
}

/// Move the write position of the WAL back to iFrame.  Called in
///* response to a ROLLBACK TO command.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_savepoint_undo(p_wal: *mut Wal,
    a_wal_data: *mut u32) -> i32 {
    let rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { *a_wal_data.offset(3 as isize) } != unsafe { (*p_wal).n_ckpt }
        {

        /// This savepoint was opened immediately after the write-transaction
        ///* was started. Right after that, the writer decided to wrap around
        ///* to the start of the log. Update the savepoint values to match.
        unsafe { *a_wal_data.offset(0 as isize) = 0 as u32 };
        unsafe {
            *a_wal_data.offset(3 as isize) = unsafe { (*p_wal).n_ckpt }
        };
    }
    if unsafe { *a_wal_data.offset(0 as isize) } <
            unsafe { (*p_wal).hdr.mx_frame } {
        unsafe {
            (*p_wal).hdr.mx_frame = unsafe { *a_wal_data.offset(0 as isize) }
        };
        unsafe {
            (*p_wal).hdr.a_frame_cksum[0 as usize] =
                unsafe { *a_wal_data.offset(1 as isize) }
        };
        unsafe {
            (*p_wal).hdr.a_frame_cksum[1 as usize] =
                unsafe { *a_wal_data.offset(2 as isize) }
        };
        { wal_cleanup_hash(p_wal); }
        { let _ = 0; };
        if unsafe { (*p_wal).i_re_cksum } > unsafe { (*p_wal).hdr.mx_frame } {
            unsafe { (*p_wal).i_re_cksum = 0 as u32 };
        }
    }
    return rc;
}

///* Information about the current state of the WAL file and where
///* the next fsync should occur - passed from sqlite3WalFrames() into
///* walWriteToLog().
#[repr(C)]
#[derive(Copy, Clone)]
struct WalWriter {
    p_wal: *mut Wal,
    p_fd: *mut Sqlite3File,
    i_sync_point: Sqlite3Int64,
    sync_flags: i32,
    sz_page: i32,
}

///* This function is called just before writing a set of frames to the log
///* file (see sqlite3WalFrames()). It checks to see if, instead of appending
///* to the current log file, it is possible to overwrite the start of the
///* existing log file with the new frames (i.e. "reset" the log). If so,
///* it sets pWal->hdr.mxFrame to 0. Otherwise, pWal->hdr.mxFrame is left
///* unchanged.
///*
///* SQLITE_OK is returned if no error is encountered (regardless of whether
///* or not pWal->hdr.mxFrame is modified). An SQLite error code is returned
///* if an error occurs.
#[allow(unused_doc_comments)]
extern "C" fn wal_restart_log(p_wal_1: *mut Wal) -> i32 {
    let mut rc: i32 = 0;
    let mut cnt: i32 = 0;
    if unsafe { (*p_wal_1).read_lock } as i32 == 0 {
        let p_info: *const WalCkptInfo =
            wal_ckpt_info(unsafe { &*p_wal_1 }) as *const WalCkptInfo;
        { let _ = 0; };
        if unsafe { (*p_info).n_backfill } as u32 > 0 as u32 {
            let mut salt1: u32 = 0 as u32;
            unsafe { sqlite3_randomness(4, &raw mut salt1 as *mut ()) };
            rc = wal_lock_exclusive(unsafe { &*p_wal_1 }, 3 + 1, 8 - 3 - 1);
            if rc == 0 {

                /// If all readers are using WAL_READ_LOCK(0) (in other words if no
                ///* readers are currently using the WAL), then the transactions
                ///* frames will overwrite the start of the existing log. Update the
                ///* wal-index header to reflect this.
                ///*
                ///* In theory it would be Ok to update the cache of the header only
                ///* at this point. But updating the actual wal-index header is also
                ///* safe and means there is no special case for sqlite3WalUndo()
                ///* to handle if this transaction is rolled back.
                wal_restart_hdr(p_wal_1, salt1);
                wal_unlock_exclusive(unsafe { &*p_wal_1 }, 3 + 1, 8 - 3 - 1);
            } else if rc != 5 { return rc; }
        }
        wal_unlock_shared(unsafe { &*p_wal_1 }, 3 + 0);
        unsafe { (*p_wal_1).read_lock = -1 as i16 };
        cnt = 0;
        '__b28: loop {
            '__c28: loop {
                let mut not_used: i32 = 0;
                rc = wal_try_begin_read(p_wal_1, &mut not_used, 1, &mut cnt);
                break '__c28;
            }
            if !(rc == -1) { break '__b28; }
        }
        { let _ = 0; };
    }
    return rc;
}

///* This function encodes a single frame header and writes it to a buffer
///* supplied by the caller. A frame-header is made up of a series of
///* 4-byte big-endian integers, as follows:
///*
///*     0: Page number.
///*     4: For commit records, the size of the database image in pages
///*        after the commit. For all other records, zero.
///*     8: Salt-1 (copied from the wal-header)
///*    12: Salt-2 (copied from the wal-header)
///*    16: Checksum-1.
///*    20: Checksum-2.
#[allow(unused_doc_comments)]
extern "C" fn wal_encode_frame(p_wal_1: &mut Wal, i_page_1: u32,
    n_truncate_1: u32, a_data_1: *mut u8, a_frame_1: *mut u8) -> () {
    let mut native_cksum: i32 = 0;
    /// True for native byte-order checksums
    let a_cksum: *mut u32 =
        &raw mut (*p_wal_1).hdr.a_frame_cksum[0 as usize] as *mut u32;
    { let _ = 0; };
    unsafe {
        sqlite3_put4byte(unsafe { &mut *a_frame_1.offset(0 as isize) },
            i_page_1)
    };
    unsafe {
        sqlite3_put4byte(unsafe { &mut *a_frame_1.offset(4 as isize) },
            n_truncate_1)
    };
    if (*p_wal_1).i_re_cksum == 0 as u32 {
        unsafe {
            memcpy(unsafe { &raw mut *a_frame_1.offset(8 as isize) } as
                    *mut (),
                &raw mut (*p_wal_1).hdr.a_salt[0 as usize] as *mut u32 as
                    *const (), 8 as u64)
        };
        native_cksum = ((*p_wal_1).hdr.big_end_cksum as i32 == 0) as i32;
        wal_checksum_bytes(native_cksum, a_frame_1, 8, a_cksum as *const u32,
            a_cksum);
        wal_checksum_bytes(native_cksum, a_data_1, (*p_wal_1).sz_page as i32,
            a_cksum as *const u32, a_cksum);
        unsafe {
            sqlite3_put4byte(unsafe { &mut *a_frame_1.offset(16 as isize) },
                unsafe { *a_cksum.offset(0 as isize) })
        };
        unsafe {
            sqlite3_put4byte(unsafe { &mut *a_frame_1.offset(20 as isize) },
                unsafe { *a_cksum.offset(1 as isize) })
        };
    } else {
        unsafe {
            memset(unsafe { &raw mut *a_frame_1.offset(8 as isize) } as
                    *mut (), 0, 16 as u64)
        };
    }
}

///* Write iAmt bytes of content into the WAL file beginning at iOffset.
///* Do a sync when crossing the p->iSyncPoint boundary.
///*
///* In other words, if iSyncPoint is in between iOffset and iOffset+iAmt,
///* first write the part before iSyncPoint, then sync, then write the
///* rest.
extern "C" fn wal_write_to_log(p: &WalWriter, mut p_content_1: *mut (),
    mut i_amt_1: i32, mut i_offset_1: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    if i_offset_1 < (*p).i_sync_point &&
            i_offset_1 + i_amt_1 as Sqlite3Int64 >= (*p).i_sync_point {
        let i_first_amt: i32 = ((*p).i_sync_point - i_offset_1) as i32;
        rc =
            unsafe {
                sqlite3_os_write((*p).p_fd, p_content_1 as *const (),
                    i_first_amt, i_offset_1)
            };
        if rc != 0 { return rc; }
        i_offset_1 += i_first_amt as Sqlite3Int64;
        i_amt_1 -= i_first_amt;
        p_content_1 =
            unsafe { (p_content_1 as *mut i8).offset(i_first_amt as isize) }
                as *mut ();
        { let _ = 0; };
        rc = unsafe { sqlite3_os_sync((*p).p_fd, (*p).sync_flags & 3) };
        if i_amt_1 == 0 || rc != 0 { return rc; }
    }
    rc =
        unsafe {
            sqlite3_os_write((*p).p_fd, p_content_1 as *const (), i_amt_1,
                i_offset_1)
        };
    return rc;
}

///* Write out a single frame of the WAL
#[allow(unused_doc_comments)]
extern "C" fn wal_write_one_frame(p: *mut WalWriter, p_page_1: &PgHdr,
    n_truncate_1: i32, i_offset_1: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    /// Result code from subfunctions
    let mut p_data: *mut () = core::ptr::null_mut();
    /// Data actually written
    let mut a_frame: [u8; 24] = [0; 24];

    /// Buffer to assemble frame-header in
    (p_data = (*p_page_1).p_data);
    wal_encode_frame(unsafe { &mut *unsafe { (*p).p_wal } }, (*p_page_1).pgno,
        n_truncate_1 as u32, p_data as *mut u8,
        &raw mut a_frame[0 as usize] as *mut u8);
    rc =
        wal_write_to_log(unsafe { &*p },
            &raw mut a_frame[0 as usize] as *mut u8 as *mut (),
            core::mem::size_of::<[u8; 24]>() as i32, i_offset_1);
    if rc != 0 { return rc; }

    /// Write the page data
    (rc =
        wal_write_to_log(unsafe { &*p }, p_data, unsafe { (*p).sz_page },
            (i_offset_1 as u64 + core::mem::size_of::<[u8; 24]>() as u64) as
                Sqlite3Int64));
    return rc;
}

///* This function is called as part of committing a transaction within which
///* one or more frames have been overwritten. It updates the checksums for
///* all frames written to the wal file by the current transaction starting
///* with the earliest to have been overwritten.
///*
///* SQLITE_OK is returned if successful, or an SQLite error code otherwise.
#[allow(unused_doc_comments)]
extern "C" fn wal_rewrite_checksums(p_wal_1: *mut Wal, i_last_1: u32) -> i32 {
    let sz_page: i32 = unsafe { (*p_wal_1).sz_page } as i32;
    /// Database page size
    let mut rc: i32 = 0;
    /// Return code
    let mut a_buf: *mut u8 = core::ptr::null_mut();
    /// Buffer to load data from wal file into
    let mut a_frame: [u8; 24] = [0; 24];
    /// Buffer to assemble frame-headers in
    let mut i_read: u32 = 0 as u32;
    /// Next frame to read from wal file
    let mut i_cksum_off: i64 = 0 as i64;
    a_buf = unsafe { sqlite3_malloc(sz_page + 24) } as *mut u8;
    if a_buf == core::ptr::null_mut() { return 7; }

    /// Find the checksum values to use as input for the recalculating the
    ///* first checksum. If the first frame is frame 1 (implying that the current
    ///* transaction restarted the wal file), these values must be read from the
    ///* wal-file header. Otherwise, read them from the frame header of the
    ///* previous frame.
    { let _ = 0; };
    if unsafe { (*p_wal_1).i_re_cksum } == 1 as u32 {
        i_cksum_off = 24 as i64;
    } else {
        i_cksum_off =
            32 as i64 +
                    (unsafe { (*p_wal_1).i_re_cksum } - 1 as u32 - 1 as u32) as
                            i64 * (sz_page + 24) as i64 + 16 as i64;
    }
    rc =
        unsafe {
            sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd }, a_buf as *mut (),
                (core::mem::size_of::<u32>() as u64 * 2 as u64) as i32,
                i_cksum_off)
        };
    unsafe {
        (*p_wal_1).hdr.a_frame_cksum[0 as usize] =
            unsafe { sqlite3_get4byte(a_buf as *const u8) }
    };
    unsafe {
        (*p_wal_1).hdr.a_frame_cksum[1 as usize] =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *a_buf.add(core::mem::size_of::<u32>() as usize)
                        } as *const u8)
            }
    };
    i_read = unsafe { (*p_wal_1).i_re_cksum };
    unsafe { (*p_wal_1).i_re_cksum = 0 as u32 };
    {
        '__b29: loop {
            if !(rc == 0 && i_read <= i_last_1) { break '__b29; }
            '__c29: loop {
                let i_off: i64 =
                    32 as i64 +
                        (i_read - 1 as u32) as i64 * (sz_page + 24) as i64;
                rc =
                    unsafe {
                        sqlite3_os_read(unsafe { (*p_wal_1).p_wal_fd },
                            a_buf as *mut (), sz_page + 24, i_off)
                    };
                if rc == 0 {
                    let mut i_pgno: u32 = 0 as u32;
                    let mut n_db_size: u32 = 0 as u32;
                    i_pgno = unsafe { sqlite3_get4byte(a_buf as *const u8) };
                    n_db_size =
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        &raw mut *a_buf.offset(4 as isize)
                                    } as *const u8)
                        };
                    wal_encode_frame(unsafe { &mut *p_wal_1 }, i_pgno,
                        n_db_size, unsafe { &mut *a_buf.offset(24 as isize) },
                        &raw mut a_frame[0 as usize] as *mut u8);
                    rc =
                        unsafe {
                            sqlite3_os_write(unsafe { (*p_wal_1).p_wal_fd },
                                &raw mut a_frame[0 as usize] as *mut u8 as *const (),
                                core::mem::size_of::<[u8; 24]>() as i32, i_off)
                        };
                }
                break '__c29;
            }
            { let __p = &mut i_read; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_free(a_buf as *mut ()) };
    return rc;
}

///* Write a set of frames to the log. The caller must hold the write-lock
///* on the log file (obtained using sqlite3WalBeginWriteTransaction()).
#[allow(unused_doc_comments)]
extern "C" fn wal_frames(p_wal_1: *mut Wal, sz_page_1: i32,
    p_list_1: *mut PgHdr, n_truncate_1: Pgno, is_commit_1: i32,
    sync_flags: i32) -> i32 {
    let mut rc: i32 = 0;
    /// Used to catch return codes
    let mut i_frame: u32 = 0 as u32;
    /// Next frame address
    let mut p: *mut PgHdr = core::ptr::null_mut();
    /// Iterator to run through pList with.
    let mut p_last: *mut PgHdr = core::ptr::null_mut();
    /// Last frame in list
    let mut n_extra: i32 = 0;
    /// Number of extra copies of last page
    let mut sz_frame: i32 = 0;
    /// The size of a single frame
    let mut i_offset: i64 = 0 as i64;
    /// Next byte to write in WAL file
    let mut w: WalWriter = unsafe { core::mem::zeroed() };
    /// The writer
    let mut i_first: u32 = 0 as u32;
    /// First frame that may be overwritten
    let mut p_live: *mut WalIndexHdr = core::ptr::null_mut();

    /// Pointer to shared header
    { let _ = 0; };
    { let _ = 0; };

    /// If this frame set completes a transaction, then nTruncate>0.  If
    ///* nTruncate==0 then this frame set does not complete the transaction.
    { let _ = 0; };
    p_live = wal_index_hdr(unsafe { &*p_wal_1 }) as *mut WalIndexHdr;
    if unsafe {
                memcmp(unsafe { &raw mut (*p_wal_1).hdr } as *const (),
                    p_live as *mut () as *const (),
                    core::mem::size_of::<WalIndexHdr>() as u64)
            } != 0 {
        i_first = unsafe { (*p_live).mx_frame } + 1 as u32;
    }
    if 0 != { rc = wal_restart_log(p_wal_1); rc } { return rc; }

    /// If this is the first frame written into the log, write the WAL
    ///* header to the start of the WAL file. See comments at the top of
    ///* this source file for a description of the WAL header format.
    (i_frame = unsafe { (*p_wal_1).hdr.mx_frame });
    if i_frame == 0 as u32 {
        let mut a_wal_hdr: [u8; 32] = [0; 32];
        /// Buffer to assemble wal-header in
        let mut a_cksum: [u32; 2] = [0; 2];

        /// Checksum for wal-header
        unsafe {
            sqlite3_put4byte(&mut a_wal_hdr[0 as usize],
                (931071618 | 0) as u32)
        };
        unsafe {
            sqlite3_put4byte(&mut a_wal_hdr[4 as usize], 3007000 as u32)
        };
        unsafe {
            sqlite3_put4byte(&mut a_wal_hdr[8 as usize], sz_page_1 as u32)
        };
        unsafe {
            sqlite3_put4byte(&mut a_wal_hdr[12 as usize],
                unsafe { (*p_wal_1).n_ckpt })
        };
        if unsafe { (*p_wal_1).n_ckpt } == 0 as u32 {
            unsafe {
                sqlite3_randomness(8,
                    unsafe { &raw mut (*p_wal_1).hdr.a_salt[0 as usize] } as
                            *mut u32 as *mut ())
            };
        }
        unsafe {
            memcpy(&raw mut a_wal_hdr[16 as usize] as *mut (),
                unsafe { &raw mut (*p_wal_1).hdr.a_salt[0 as usize] } as
                        *mut u32 as *const (), 8 as u64)
        };
        wal_checksum_bytes(1, &raw mut a_wal_hdr[0 as usize] as *mut u8,
            32 - 2 * 4, core::ptr::null(),
            &raw mut a_cksum[0 as usize] as *mut u32);
        unsafe {
            sqlite3_put4byte(&mut a_wal_hdr[24 as usize], a_cksum[0 as usize])
        };
        unsafe {
            sqlite3_put4byte(&mut a_wal_hdr[28 as usize], a_cksum[1 as usize])
        };
        unsafe { (*p_wal_1).sz_page = sz_page_1 as u32 };
        unsafe { (*p_wal_1).hdr.big_end_cksum = 0 as u8 };
        unsafe {
            (*p_wal_1).hdr.a_frame_cksum[0 as usize] = a_cksum[0 as usize]
        };
        unsafe {
            (*p_wal_1).hdr.a_frame_cksum[1 as usize] = a_cksum[1 as usize]
        };
        unsafe { (*p_wal_1).truncate_on_commit = 1 as u8 };
        rc =
            unsafe {
                sqlite3_os_write(unsafe { (*p_wal_1).p_wal_fd },
                    &raw mut a_wal_hdr[0 as usize] as *mut u8 as *const (),
                    core::mem::size_of::<[u8; 32]>() as i32, 0 as i64)
            };
        if rc != 0 { return rc; }
        if unsafe { (*p_wal_1).sync_header } != 0 {
            rc =
                unsafe {
                    sqlite3_os_sync(unsafe { (*p_wal_1).p_wal_fd },
                        sync_flags >> 2 & 3)
                };
            if rc != 0 { return rc; }
        }
    }
    if unsafe { (*p_wal_1).sz_page } as i32 != sz_page_1 {
        return unsafe { sqlite3_corrupt_error(4127) };
    }

    /// Setup information needed to write frames into the WAL
    (w.p_wal = p_wal_1);
    w.p_fd = unsafe { (*p_wal_1).p_wal_fd };
    w.i_sync_point = 0 as Sqlite3Int64;
    w.sync_flags = sync_flags;
    w.sz_page = sz_page_1;
    i_offset =
        32 as i64 +
            (i_frame + 1 as u32 - 1 as u32) as i64 * (sz_page_1 + 24) as i64;
    sz_frame = sz_page_1 + 24;
    {
        p = p_list_1;
        '__b30: loop {
            if !(!(p).is_null()) { break '__b30; }
            '__c30: loop {
                let mut n_db_size: i32 = 0;
                if i_first != 0 &&
                        (!(unsafe { (*p).p_dirty }).is_null() || is_commit_1 == 0) {
                    let mut i_write: u32 = 0 as u32;
                    wal_find_frame(p_wal_1, unsafe { (*p).pgno }, &mut i_write);
                    { let _ = 0; };
                    if i_write >= i_first {
                        let i_off: i64 =
                            32 as i64 +
                                    (i_write - 1 as u32) as i64 * (sz_page_1 + 24) as i64 +
                                24 as i64;
                        let mut p_data: *mut () = core::ptr::null_mut();
                        if unsafe { (*p_wal_1).i_re_cksum } == 0 as u32 ||
                                i_write < unsafe { (*p_wal_1).i_re_cksum } {
                            unsafe { (*p_wal_1).i_re_cksum = i_write };
                        }
                        p_data = unsafe { (*p).p_data };
                        rc =
                            unsafe {
                                sqlite3_os_write(unsafe { (*p_wal_1).p_wal_fd },
                                    p_data as *const (), sz_page_1, i_off)
                            };
                        if rc != 0 { return rc; }
                        unsafe { (*p).flags &= !64 as u16 };
                        break '__c30;
                    }
                }
                { let __p = &mut i_frame; let __t = *__p; *__p += 1; __t };
                { let _ = 0; };
                n_db_size =
                    if is_commit_1 != 0 &&
                                unsafe { (*p).p_dirty } == core::ptr::null_mut() {
                            n_truncate_1
                        } else { 0 as Pgno } as i32;
                rc =
                    wal_write_one_frame(&mut w, unsafe { &*p }, n_db_size,
                        i_offset);
                if rc != 0 { return rc; }
                p_last = p;
                i_offset += sz_frame as i64;
                unsafe { (*p).flags |= 64 as u16 };
                break '__c30;
            }
            p = unsafe { (*p).p_dirty };
        }
    }
    if is_commit_1 != 0 && unsafe { (*p_wal_1).i_re_cksum } != 0 {
        rc = wal_rewrite_checksums(p_wal_1, i_frame);
        if rc != 0 { return rc; }
    }
    if is_commit_1 != 0 && sync_flags & 3 != 0 {
        let mut b_sync: i32 = 1;
        if unsafe { (*p_wal_1).pad_to_sector_boundary } != 0 {
            let sector_size: i32 =
                unsafe {
                    sqlite3_sector_size(unsafe { (*p_wal_1).p_wal_fd })
                };
            w.i_sync_point =
                (i_offset + sector_size as i64 - 1 as i64) /
                        sector_size as i64 * sector_size as i64;
            b_sync = (w.i_sync_point == i_offset) as i32;
            while i_offset < w.i_sync_point {
                rc =
                    wal_write_one_frame(&mut w, unsafe { &*p_last },
                        n_truncate_1 as i32, i_offset);
                if rc != 0 { return rc; }
                i_offset += sz_frame as i64;
                { let __p = &mut n_extra; let __t = *__p; *__p += 1; __t };
                { let _ = 0; };
            }
        }
        if b_sync != 0 {
            { let _ = 0; };
            rc = unsafe { sqlite3_os_sync(w.p_fd, sync_flags & 3) };
        }
    }
    if is_commit_1 != 0 && unsafe { (*p_wal_1).truncate_on_commit } != 0 &&
            unsafe { (*p_wal_1).mx_wal_size } >= 0 as i64 {
        let mut sz: i64 = unsafe { (*p_wal_1).mx_wal_size };
        if 32 as i64 +
                    (i_frame + n_extra as u32 + 1 as u32 - 1 as u32) as i64 *
                        (sz_page_1 + 24) as i64 > unsafe { (*p_wal_1).mx_wal_size }
            {
            sz =
                32 as i64 +
                    (i_frame + n_extra as u32 + 1 as u32 - 1 as u32) as i64 *
                        (sz_page_1 + 24) as i64;
        }
        wal_limit_size(unsafe { &*p_wal_1 }, sz);
        unsafe { (*p_wal_1).truncate_on_commit = 0 as u8 };
    }

    /// Append data to the wal-index. It is not necessary to lock the
    ///* wal-index to do this as the SQLITE_SHM_WRITE lock held on the wal-index
    ///* guarantees that there are no other writers, and no data that may
    ///* be in use by existing readers is being overwritten.
    (i_frame = unsafe { (*p_wal_1).hdr.mx_frame });
    {
        p = p_list_1;
        '__b32: loop {
            if !(!(p).is_null() && rc == 0) { break '__b32; }
            '__c32: loop {
                if unsafe { (*p).flags } as i32 & 64 == 0 { break '__c32; }
                { let __p = &mut i_frame; let __t = *__p; *__p += 1; __t };
                rc = wal_index_append(p_wal_1, i_frame, unsafe { (*p).pgno });
                break '__c32;
            }
            p = unsafe { (*p).p_dirty };
        }
    }
    { let _ = 0; };
    while rc == 0 && n_extra > 0 {
        { let __p = &mut i_frame; let __t = *__p; *__p += 1; __t };
        { let __p = &mut n_extra; let __t = *__p; *__p -= 1; __t };
        rc = wal_index_append(p_wal_1, i_frame, unsafe { (*p_last).pgno });
    }
    if rc == 0 {
        unsafe {

            /// Update the private copy of the header.
            ((*p_wal_1).hdr.sz_page =
                (sz_page_1 & 65280 | sz_page_1 >> 16) as u16)
        };
        unsafe { (*p_wal_1).hdr.mx_frame = i_frame };
        if is_commit_1 != 0 {
            {
                let __p = unsafe { &mut (*p_wal_1).hdr.i_change };
                let __t = *__p;
                *__p += 1;
                __t
            };
            unsafe { (*p_wal_1).hdr.n_page = n_truncate_1 };
        }
        if is_commit_1 != 0 {
            wal_index_write_hdr(p_wal_1);
            unsafe { (*p_wal_1).i_callback = i_frame };
        }
    }
    return rc;
}

/// Write a frame or frames to the log.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_frames(p_wal: *mut Wal, sz_page: i32,
    p_list: *mut PgHdr, n_truncate: Pgno, is_commit: i32, sync_flags: i32)
    -> i32 {
    let mut rc: i32 = 0;
    {
        rc =
            wal_frames(p_wal, sz_page, p_list, n_truncate, is_commit,
                sync_flags);
    }
    { let _ = 0; };
    return rc;
}

/// Return the value to pass to a sqlite3_wal_hook callback, the
///* number of frames in the WAL at the point of the last commit since
///* sqlite3WalCallback() was called.  If no commits have occurred since
///* the last call, then return 0.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_callback(p_wal: *mut Wal) -> i32 {
    let mut ret: u32 = 0 as u32;
    if !(p_wal).is_null() {
        ret = unsafe { (*p_wal).i_callback };
        unsafe { (*p_wal).i_callback = 0 as u32 };
    }
    return ret as i32;
}

/// Tell the wal layer that an EXCLUSIVE lock has been obtained (or released)
///* by the pager layer on the database file.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_wal_exclusive_mode(p_wal: *mut Wal, op: i32)
    -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };

    /// pWal->readLock is usually set, but might be -1 if there was a
    ///* prior error while attempting to acquire are read-lock. This cannot
    ///* happen if the connection is actually in exclusive mode (as no xShmLock
    ///* locks are taken in this case). Nor should the pager attempt to
    ///* upgrade to exclusive-mode following such an error.
    { let _ = 0; };
    { let _ = 0; };
    if op == 0 {
        if unsafe { (*p_wal).exclusive_mode } as i32 != 0 {
            unsafe { (*p_wal).exclusive_mode = 0 as u8 };
            if wal_lock_shared(unsafe { &*p_wal },
                        3 + unsafe { (*p_wal).read_lock } as i32) != 0 {
                unsafe { (*p_wal).exclusive_mode = 1 as u8 };
            }
            rc = (unsafe { (*p_wal).exclusive_mode } as i32 == 0) as i32;
        } else {

            /// Already in locking_mode=NORMAL
            (rc = 0);
        }
    } else if op > 0 {
        { let _ = 0; };
        { let _ = 0; };
        wal_unlock_shared(unsafe { &*p_wal },
            3 + unsafe { (*p_wal).read_lock } as i32);
        unsafe { (*p_wal).exclusive_mode = 1 as u8 };
        rc = 1;
    } else { rc = (unsafe { (*p_wal).exclusive_mode } as i32 == 0) as i32; }
    return rc;
}

/// Return true if the argument is non-NULL and the WAL module is using
///* heap-memory for the wal-index. Otherwise, if the argument is NULL or the
///* WAL module is using shared-memory, return false.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_heap_memory(p_wal: *mut Wal) -> i32 {
    return (!(p_wal).is_null() &&
                unsafe { (*p_wal).exclusive_mode } as i32 == 2) as i32;
}

/// Return the sqlite3_file object for the WAL file
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_wal_file(p_wal: &Wal) -> *mut Sqlite3File {
    return (*p_wal).p_wal_fd;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SublistN7Sublist {
    n_list: i32,
    a_list: *mut HtSlot,
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
    fn sqlite3_close(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_close_v2(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_exec(_: *mut Sqlite3, sql: *const i8,
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
    fn sqlite3_db_config(_: *mut Sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_extended_result_codes(_: *mut Sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_last_insert_rowid(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_set_last_insert_rowid(_: *mut Sqlite3, _: Sqlite3Int64)
    -> ();
    fn sqlite3_changes(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_changes64(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_total_changes(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_total_changes64(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_interrupt(_: *mut Sqlite3)
    -> ();
    fn sqlite3_is_interrupted(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_complete(sql: *const i8)
    -> i32;
    fn sqlite3_complete16(sql: *const ())
    -> i32;
    fn sqlite3_incomplete(sql: *const i8)
    -> Sqlite3Int64;
    fn sqlite3_busy_handler(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), i32) -> i32>, _: *mut ())
    -> i32;
    fn sqlite3_busy_timeout(_: *mut Sqlite3, ms: i32)
    -> i32;
    fn sqlite3_setlk_timeout(_: *mut Sqlite3, ms: i32, flags: i32)
    -> i32;
    fn sqlite3_get_table(db: *mut Sqlite3, z_sql_1: *const i8,
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
    fn sqlite3_malloc64(_: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_realloc(_: *mut (), _: i32)
    -> *mut ();
    fn sqlite3_realloc64(_: *mut (), _: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_msize(_: *mut ())
    -> Sqlite3Uint64;
    fn sqlite3_memory_used()
    -> Sqlite3Int64;
    fn sqlite3_memory_highwater(reset_flag_1: i32)
    -> Sqlite3Int64;
    fn sqlite3_randomness(n_1: i32, p_1: *mut ())
    -> ();
    fn sqlite3_set_authorizer(_: *mut Sqlite3,
    x_auth_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
            *const i8, *const i8) -> i32>, p_user_data_1: *mut ())
    -> i32;
    fn sqlite3_trace(_: *mut Sqlite3,
    x_trace_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_profile(_: *mut Sqlite3,
    x_profile_1: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_trace_v2(_: *mut Sqlite3, u_mask_1: u32,
    x_callback_1:
        Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    p_ctx_1: *mut ())
    -> i32;
    fn sqlite3_progress_handler(_: *mut Sqlite3, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_open(filename: *const i8, pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_open16(filename: *const (), pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_open_v2(filename: *const i8, pp_db_1: *mut *mut Sqlite3,
    flags: i32, z_vfs_1: *const i8)
    -> i32;
    fn sqlite3_uri_parameter(z: Sqlite3Filename, z_param_1: *const i8)
    -> *const i8;
    fn sqlite3_uri_boolean(z: Sqlite3Filename, z_param_1: *const i8,
    b_default_1: i32)
    -> i32;
    fn sqlite3_uri_int64(_: Sqlite3Filename, _: *const i8, _: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_uri_key(z: Sqlite3Filename, n_1: i32)
    -> *const i8;
    fn sqlite3_filename_database(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_filename_journal(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_filename_wal(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_database_file_object(_: *const i8)
    -> *mut Sqlite3File;
    fn sqlite3_create_filename(z_database_1: *const i8,
    z_journal_1: *const i8, z_wal_1: *const i8, n_param_1: i32,
    az_param_1: *mut *const i8)
    -> Sqlite3Filename;
    fn sqlite3_free_filename(_: Sqlite3Filename)
    -> ();
    fn sqlite3_errcode(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_extended_errcode(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_errmsg(_: *mut Sqlite3)
    -> *const i8;
    fn sqlite3_errmsg16(_: *mut Sqlite3)
    -> *const ();
    fn sqlite3_errstr(_: i32)
    -> *const i8;
    fn sqlite3_error_offset(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_set_errmsg(db: *mut Sqlite3, errcode: i32,
    z_err_msg_1: *const i8)
    -> i32;
    fn sqlite3_limit(_: *mut Sqlite3, id: i32, new_val_1: i32)
    -> i32;
    fn sqlite3_prepare(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v2(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v3(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    prep_flags_1: u32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare16(db: *mut Sqlite3, z_sql_1: *const (), n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v2(db: *mut Sqlite3, z_sql_1: *const (),
    n_byte_1: i32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v3(db: *mut Sqlite3, z_sql_1: *const (),
    n_byte_1: i32, prep_flags_1: u32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_sql(p_stmt_1: *mut Sqlite3Stmt)
    -> *const i8;
    fn sqlite3_expanded_sql(p_stmt_1: *mut Sqlite3Stmt)
    -> *mut i8;
    fn sqlite3_stmt_readonly(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_stmt_isexplain(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_stmt_explain(p_stmt_1: *mut Sqlite3Stmt, e_mode_1: i32)
    -> i32;
    fn sqlite3_stmt_busy(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_bind_blob(_: *mut Sqlite3Stmt, _: i32, _: *const (), n: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_blob64(_: *mut Sqlite3Stmt, _: i32, _: *const (),
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_double(_: *mut Sqlite3Stmt, _: i32, _: f64)
    -> i32;
    fn sqlite3_bind_int(_: *mut Sqlite3Stmt, _: i32, _: i32)
    -> i32;
    fn sqlite3_bind_int64(_: *mut Sqlite3Stmt, _: i32, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_bind_null(_: *mut Sqlite3Stmt, _: i32)
    -> i32;
    fn sqlite3_bind_text(_: *mut Sqlite3Stmt, _: i32, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text16(_: *mut Sqlite3Stmt, _: i32, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text64(_: *mut Sqlite3Stmt, _: i32, _: *const i8,
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> i32;
    fn sqlite3_bind_value(_: *mut Sqlite3Stmt, _: i32, _: *const Sqlite3Value)
    -> i32;
    fn sqlite3_bind_pointer(_: *mut Sqlite3Stmt, _: i32, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_zeroblob(_: *mut Sqlite3Stmt, _: i32, n: i32)
    -> i32;
    fn sqlite3_bind_zeroblob64(_: *mut Sqlite3Stmt, _: i32, _: Sqlite3Uint64)
    -> i32;
    fn sqlite3_bind_parameter_count(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_bind_parameter_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_bind_parameter_index(_: *mut Sqlite3Stmt, z_name_1: *const i8)
    -> i32;
    fn sqlite3_clear_bindings(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_count(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_name(_: *mut Sqlite3Stmt, n_1: i32)
    -> *const i8;
    fn sqlite3_column_name16(_: *mut Sqlite3Stmt, n_1: i32)
    -> *const ();
    fn sqlite3_column_database_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_database_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_table_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_table_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_origin_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_origin_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_decltype(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_decltype16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_step(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_data_count(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_blob(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_double(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> f64;
    fn sqlite3_column_int(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_int64(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> Sqlite3Int64;
    fn sqlite3_column_text(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const u8;
    fn sqlite3_column_text16(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_value(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *mut Sqlite3Value;
    fn sqlite3_column_bytes(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_bytes16(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_type(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_finalize(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_reset(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_create_function(db: *mut Sqlite3, z_function_name_1: *const i8,
    n_arg_1: i32, e_text_rep_1: i32, p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32;
    fn sqlite3_create_function16(db: *mut Sqlite3,
    z_function_name_1: *const (), n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32;
    fn sqlite3_create_function_v2(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_window_function(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_aggregate_count(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_expired(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_transfer_bindings(_: *mut Sqlite3Stmt, _: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_global_recover()
    -> i32;
    fn sqlite3_thread_cleanup()
    -> ();
    fn sqlite3_memory_alarm(_:
        Option<unsafe extern "C" fn(*mut (), i64, i32) -> ()>, _: *mut (),
    _: Sqlite3Int64)
    -> i32;
    fn sqlite3_value_blob(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_double(_: *mut Sqlite3Value)
    -> f64;
    fn sqlite3_value_int(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_int64(_: *mut Sqlite3Value)
    -> Sqlite3Int64;
    fn sqlite3_value_pointer(_: *mut Sqlite3Value, _: *const i8)
    -> *mut ();
    fn sqlite3_value_text(_: *mut Sqlite3Value)
    -> *const u8;
    fn sqlite3_value_text16(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_text16le(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_text16be(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_bytes(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_bytes16(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_numeric_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_nochange(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_frombind(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_encoding(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_subtype(_: *mut Sqlite3Value)
    -> u32;
    fn sqlite3_value_dup(_: *const Sqlite3Value)
    -> *mut Sqlite3Value;
    fn sqlite3_value_free(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_aggregate_context(_: *mut Sqlite3Context, n_bytes_1: i32)
    -> *mut ();
    fn sqlite3_user_data(_: *mut Sqlite3Context)
    -> *mut ();
    fn sqlite3_context_db_handle(_: *mut Sqlite3Context)
    -> *mut Sqlite3;
    fn sqlite3_get_auxdata(_: *mut Sqlite3Context, n_1: i32)
    -> *mut ();
    fn sqlite3_set_auxdata(_: *mut Sqlite3Context, n_1: i32, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_get_clientdata(_: *mut Sqlite3, _: *const i8)
    -> *mut ();
    fn sqlite3_set_clientdata(_: *mut Sqlite3, _: *const i8, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_result_blob(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_blob64(_: *mut Sqlite3Context, _: *const (),
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_double(_: *mut Sqlite3Context, _: f64)
    -> ();
    fn sqlite3_result_error(_: *mut Sqlite3Context, _: *const i8, _: i32)
    -> ();
    fn sqlite3_result_error16(_: *mut Sqlite3Context, _: *const (), _: i32)
    -> ();
    fn sqlite3_result_error_toobig(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_error_nomem(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_error_code(_: *mut Sqlite3Context, _: i32)
    -> ();
    fn sqlite3_result_int(_: *mut Sqlite3Context, _: i32)
    -> ();
    fn sqlite3_result_int64(_: *mut Sqlite3Context, _: Sqlite3Int64)
    -> ();
    fn sqlite3_result_null(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_text(_: *mut Sqlite3Context, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text64(_: *mut Sqlite3Context, z: *const i8,
    n: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> ();
    fn sqlite3_result_text16(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16le(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16be(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_value(_: *mut Sqlite3Context, _: *mut Sqlite3Value)
    -> ();
    fn sqlite3_result_pointer(_: *mut Sqlite3Context, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_zeroblob(_: *mut Sqlite3Context, n: i32)
    -> ();
    fn sqlite3_result_zeroblob64(_: *mut Sqlite3Context, n: Sqlite3Uint64)
    -> i32;
    fn sqlite3_result_subtype(_: *mut Sqlite3Context, _: u32)
    -> ();
    fn sqlite3_create_collation(_: *mut Sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_create_collation_v2(_: *mut Sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_collation16(_: *mut Sqlite3, z_name_1: *const (),
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_collation_needed(_: *mut Sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const i8)
            -> ()>)
    -> i32;
    fn sqlite3_collation_needed16(_: *mut Sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const ())
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
    fn sqlite3_get_autocommit(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_db_handle(_: *mut Sqlite3Stmt)
    -> *mut Sqlite3;
    fn sqlite3_db_name(db: *mut Sqlite3, n_1: i32)
    -> *const i8;
    fn sqlite3_db_filename(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> Sqlite3Filename;
    fn sqlite3_db_readonly(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> i32;
    fn sqlite3_txn_state(_: *mut Sqlite3, z_schema_1: *const i8)
    -> i32;
    fn sqlite3_next_stmt(p_db_1: *mut Sqlite3, p_stmt_1: *mut Sqlite3Stmt)
    -> *mut Sqlite3Stmt;
    fn sqlite3_commit_hook(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_rollback_hook(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_autovacuum_pages(db: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    _: *mut (), _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_update_hook(_: *mut Sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64)
            -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_enable_shared_cache(_: i32)
    -> i32;
    fn sqlite3_release_memory(_: i32)
    -> i32;
    fn sqlite3_db_release_memory(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_soft_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_hard_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_soft_heap_limit(n_1: i32)
    -> ();
    fn sqlite3_table_column_metadata(db: *mut Sqlite3, z_db_name_1: *const i8,
    z_table_name_1: *const i8, z_column_name_1: *const i8,
    pz_data_type_1: *mut *const i8, pz_coll_seq_1: *mut *const i8,
    p_not_null_1: *mut i32, p_primary_key_1: *mut i32, p_autoinc_1: *mut i32)
    -> i32;
    fn sqlite3_load_extension(db: *mut Sqlite3, z_file_1: *const i8,
    z_proc_1: *const i8, pz_err_msg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_enable_load_extension(db: *mut Sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_cancel_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_reset_auto_extension()
    -> ();
    fn sqlite3_create_module(db: *mut Sqlite3, z_name_1: *const i8,
    p: *const Sqlite3Module, p_client_data_1: *mut ())
    -> i32;
    fn sqlite3_create_module_v2(db: *mut Sqlite3, z_name_1: *const i8,
    p: *const Sqlite3Module, p_client_data_1: *mut (),
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_drop_modules(db: *mut Sqlite3, az_keep_1: *mut *const i8)
    -> i32;
    fn sqlite3_declare_vtab(_: *mut Sqlite3, z_sql_1: *const i8)
    -> i32;
    fn sqlite3_overload_function(_: *mut Sqlite3, z_func_name_1: *const i8,
    n_arg_1: i32)
    -> i32;
    fn sqlite3_blob_open(_: *mut Sqlite3, z_db_1: *const i8,
    z_table_1: *const i8, z_column_1: *const i8, i_row_1: Sqlite3Int64,
    flags: i32, pp_blob_1: *mut *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_reopen(_: *mut Sqlite3Blob, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_blob_close(_: *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_bytes(_: *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_read(_: *mut Sqlite3Blob, z_1: *mut (), n_1: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_blob_write(_: *mut Sqlite3Blob, z: *const (), n: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_vfs_find(z_vfs_name_1: *const i8)
    -> *mut Sqlite3Vfs;
    fn sqlite3_vfs_register(_: *mut Sqlite3Vfs, make_dflt_1: i32)
    -> i32;
    fn sqlite3_vfs_unregister(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_mutex_free(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_enter(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_try(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_leave(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_held(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_notheld(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_db_mutex(_: *mut Sqlite3)
    -> *mut Sqlite3Mutex;
    fn sqlite3_file_control(_: *mut Sqlite3, z_db_name_1: *const i8, op: i32,
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
    fn sqlite3_str_new(_: *mut Sqlite3)
    -> *mut Sqlite3Str;
    fn sqlite3_str_finish(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_str_free(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_result_str(_: *mut Sqlite3Context, _: *mut Sqlite3Str, _: i32)
    -> ();
    fn sqlite3_str_appendf(_: *mut Sqlite3Str, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_str_vappendf(_: *mut Sqlite3Str, z_format_1: *const i8,
    _: *mut i8)
    -> ();
    fn sqlite3_str_append(_: *mut Sqlite3Str, z_in_1: *const i8, n_1: i32)
    -> ();
    fn sqlite3_str_appendall(_: *mut Sqlite3Str, z_in_1: *const i8)
    -> ();
    fn sqlite3_str_appendchar(_: *mut Sqlite3Str, n_1: i32, c_1: i8)
    -> ();
    fn sqlite3_str_reset(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_str_truncate(_: *mut Sqlite3Str, n_1: i32)
    -> ();
    fn sqlite3_str_errcode(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_length(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_value(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_status(op: i32, p_current_1: *mut i32, p_highwater_1: *mut i32,
    reset_flag_1: i32)
    -> i32;
    fn sqlite3_status64(op: i32, p_current_1: *mut Sqlite3Int64,
    p_highwater_1: *mut Sqlite3Int64, reset_flag_1: i32)
    -> i32;
    fn sqlite3_db_status(_: *mut Sqlite3, op: i32, p_cur_1: *mut i32,
    p_hiwtr_1: *mut i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_db_status64(_: *mut Sqlite3, _: i32, _: *mut Sqlite3Int64,
    _: *mut Sqlite3Int64, _: i32)
    -> i32;
    fn sqlite3_stmt_status(_: *mut Sqlite3Stmt, op: i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_backup_init(p_dest_1: *mut Sqlite3, z_dest_name_1: *const i8,
    p_source_1: *mut Sqlite3, z_source_name_1: *const i8)
    -> *mut Sqlite3Backup;
    fn sqlite3_backup_step(p: *mut Sqlite3Backup, n_page_1: i32)
    -> i32;
    fn sqlite3_backup_finish(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_backup_remaining(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_backup_pagecount(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_unlock_notify(p_blocked_1: *mut Sqlite3,
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
    fn sqlite3_wal_hook(_: *mut Sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32)
            -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_wal_autocheckpoint(db: *mut Sqlite3, n_1: i32)
    -> i32;
    fn sqlite3_wal_checkpoint(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_wal_checkpoint_v2(db: *mut Sqlite3, z_db_1: *const i8,
    e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32;
    fn sqlite3_vtab_config(_: *mut Sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_vtab_on_conflict(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_nochange(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_vtab_collation(_: *mut Sqlite3IndexInfo, _: i32)
    -> *const i8;
    fn sqlite3_vtab_distinct(_: *mut Sqlite3IndexInfo)
    -> i32;
    fn sqlite3_vtab_in(_: *mut Sqlite3IndexInfo, i_cons_1: i32,
    b_handle_1: i32)
    -> i32;
    fn sqlite3_vtab_in_first(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_in_next(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_rhs_value(_: *mut Sqlite3IndexInfo, _: i32,
    pp_val_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_stmt_scanstatus(p_stmt_1: *mut Sqlite3Stmt, idx: i32,
    i_scan_status_op_1: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_v2(p_stmt_1: *mut Sqlite3Stmt, idx: i32,
    i_scan_status_op_1: i32, flags: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_reset(_: *mut Sqlite3Stmt)
    -> ();
    fn sqlite3_db_cacheflush(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_system_errno(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_snapshot_get(db: *mut Sqlite3, z_schema_1: *const i8,
    pp_snapshot_1: *mut *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_open(db: *mut Sqlite3, z_schema_1: *const i8,
    p_snapshot_1: *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_free(_: *mut Sqlite3Snapshot)
    -> ();
    fn sqlite3_snapshot_cmp(p1: *mut Sqlite3Snapshot,
    p2: *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_recover(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_serialize(db: *mut Sqlite3, z_schema_1: *const i8,
    pi_size_1: *mut Sqlite3Int64, m_flags_1: u32)
    -> *mut u8;
    fn sqlite3_deserialize(db: *mut Sqlite3, z_schema_1: *const i8,
    p_data_1: *mut u8, sz_db_1: Sqlite3Int64, sz_buf_1: Sqlite3Int64,
    m_flags_1: u32)
    -> i32;
    fn sqlite3_carray_bind_v2(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, p_del_1: *mut ())
    -> i32;
    fn sqlite3_carray_bind(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_rtree_geometry_callback(db: *mut Sqlite3, z_geom_1: *const i8,
    x_geom_1:
        Option<unsafe extern "C" fn(*mut Sqlite3RtreeGeometry, i32, *mut f64,
            *mut i32) -> i32>, p_context_1: *mut ())
    -> i32;
    fn sqlite3_rtree_query_callback(db: *mut Sqlite3,
    z_query_func_1: *const i8,
    x_query_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3RtreeQueryInfo) -> i32>,
    p_context_1: *mut (),
    x_destructor_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_hash_init(_: *mut Hash)
    -> ();
    fn sqlite3_hash_insert(_: *mut Hash, p_key_1: *const i8,
    p_data_1: *mut ())
    -> *mut ();
    fn sqlite3_hash_find(_: *const Hash, p_key_1: *const i8)
    -> *mut ();
    fn sqlite3_hash_clear(_: *mut Hash)
    -> ();
    static mut sqlite3_tree_trace: u32;
    static mut sqlite3_where_trace: u32;
    fn sqlite3OsInit()
    -> i32;
    fn sqlite3_os_close(_: *mut Sqlite3File)
    -> ();
    fn sqlite3_os_read(_: *mut Sqlite3File, _: *mut (), amt: i32, offset: i64)
    -> i32;
    fn sqlite3_os_write(_: *mut Sqlite3File, _: *const (), amt: i32,
    offset: i64)
    -> i32;
    fn sqlite3_os_truncate(_: *mut Sqlite3File, size: i64)
    -> i32;
    fn sqlite3_os_sync(_: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_file_size(_: *mut Sqlite3File, p_size_1: *mut i64)
    -> i32;
    fn sqlite3_os_lock(_: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_unlock(_: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_check_reserved_lock(id: *mut Sqlite3File,
    p_res_out_1: *mut i32)
    -> i32;
    fn sqlite3_os_file_control(_: *mut Sqlite3File, _: i32, _: *mut ())
    -> i32;
    fn sqlite3_os_file_control_hint(_: *mut Sqlite3File, _: i32, _: *mut ())
    -> ();
    fn sqlite3_os_sector_size(id: *mut Sqlite3File)
    -> i32;
    fn sqlite3_os_device_characteristics(id: *mut Sqlite3File)
    -> i32;
    fn sqlite3_os_shm_map(_: *mut Sqlite3File, _: i32, _: i32, _: i32,
    _: *mut *mut ())
    -> i32;
    fn sqlite3_os_shm_lock(id: *mut Sqlite3File, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_os_shm_barrier(id: *mut Sqlite3File)
    -> ();
    fn sqlite3_os_shm_unmap(id: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_fetch(id: *mut Sqlite3File, _: i64, _: i32, _: *mut *mut ())
    -> i32;
    fn sqlite3_os_unfetch(_: *mut Sqlite3File, _: i64, _: *mut ())
    -> i32;
    fn sqlite3_os_open(_: *mut Sqlite3Vfs, _: *const i8, _: *mut Sqlite3File,
    _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_os_delete(_: *mut Sqlite3Vfs, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_os_access(_: *mut Sqlite3Vfs, _: *const i8, _: i32,
    p_res_out_1: *mut i32)
    -> i32;
    fn sqlite3_os_full_pathname(_: *mut Sqlite3Vfs, _: *const i8, _: i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_os_dl_open(_: *mut Sqlite3Vfs, _: *const i8)
    -> *mut ();
    fn sqlite3_os_dl_error(_: *mut Sqlite3Vfs, _: i32, _: *mut i8)
    -> ();
    fn sqlite3_os_dl_sym(_: *mut Sqlite3Vfs, _: *mut (), _: *const i8)
    -> unsafe extern "C" fn() -> ();
    fn sqlite3_os_dl_close(_: *mut Sqlite3Vfs, _: *mut ())
    -> ();
    fn sqlite3_os_randomness(_: *mut Sqlite3Vfs, _: i32, _: *mut i8)
    -> i32;
    fn sqlite3_os_sleep(_: *mut Sqlite3Vfs, _: i32)
    -> i32;
    fn sqlite3_os_get_last_error(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_os_current_time_int64(_: *mut Sqlite3Vfs, _: *mut Sqlite3Int64)
    -> i32;
    fn sqlite3_os_open_malloc(_: *mut Sqlite3Vfs, _: *const i8,
    _: *mut *mut Sqlite3File, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_os_close_free(_: *mut Sqlite3File)
    -> ();
    fn sqlite3_pager_open(_: *mut Sqlite3Vfs, pp_pager_1: *mut *mut Pager,
    _: *const i8, _: i32, _: i32, _: i32,
    _: Option<unsafe extern "C" fn(*mut PgHdr) -> ()>)
    -> i32;
    fn sqlite3_pager_close(p_pager_1: *mut Pager, _: *mut Sqlite3)
    -> i32;
    fn sqlite3_pager_read_fileheader(_: *mut Pager, _: i32, _: *mut u8)
    -> i32;
    fn sqlite3_pager_set_busy_handler(_: *mut Pager,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_pager_set_pagesize(_: *mut Pager, _: *mut u32, _: i32)
    -> i32;
    fn sqlite3_pager_max_page_count(_: *mut Pager, _: Pgno)
    -> Pgno;
    fn sqlite3_pager_set_cachesize(_: *mut Pager, _: i32)
    -> ();
    fn sqlite3_pager_set_spillsize(_: *mut Pager, _: i32)
    -> i32;
    fn sqlite3_pager_set_mmap_limit(_: *mut Pager, _: Sqlite3Int64)
    -> ();
    fn sqlite3_pager_shrink(_: *mut Pager)
    -> ();
    fn sqlite3_pager_set_flags(_: *mut Pager, _: u32)
    -> ();
    fn sqlite3_pager_locking_mode(_: *mut Pager, _: i32)
    -> i32;
    fn sqlite3_pager_set_journal_mode(_: *mut Pager, _: i32)
    -> i32;
    fn sqlite3_pager_get_journal_mode(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_ok_to_change_journal_mode(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_journal_size_limit(_: *mut Pager, _: i64)
    -> i64;
    fn sqlite3_pager_backup_ptr(_: *mut Pager)
    -> *mut *mut Sqlite3Backup;
    fn sqlite3_pager_flush(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_get(p_pager_1: *mut Pager, pgno: Pgno,
    pp_page_1: *mut *mut DbPage, clr_flag_1: i32)
    -> i32;
    fn sqlite3_pager_lookup(p_pager_1: *mut Pager, pgno: Pgno)
    -> *mut DbPage;
    fn sqlite3_pager_ref(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_unref(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_unref_not_null(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_unref_page_one(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_write(_: *mut DbPage)
    -> i32;
    fn sqlite3_pager_dont_write(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_movepage(_: *mut Pager, _: *mut DbPage, _: Pgno, _: i32)
    -> i32;
    fn sqlite3_pager_page_refcount(_: *mut DbPage)
    -> i32;
    fn sqlite3_pager_get_data(_: *mut DbPage)
    -> *mut ();
    fn sqlite3_pager_get_extra(_: *mut DbPage)
    -> *mut ();
    fn sqlite3_pager_pagecount(_: *mut Pager, _: *mut i32)
    -> ();
    fn sqlite3_pager_begin(_: *mut Pager, ex_flag_1: i32, _: i32)
    -> i32;
    fn sqlite3_pager_commit_phase_one(_: *mut Pager, z_super_1: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_pager_exclusive_lock(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_sync(p_pager_1: *mut Pager, z_super_1: *const i8)
    -> i32;
    fn sqlite3_pager_commit_phase_two(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_rollback(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_open_savepoint(p_pager_1: *mut Pager, n: i32)
    -> i32;
    fn sqlite3_pager_savepoint(p_pager_1: *mut Pager, op: i32,
    i_savepoint_1: i32)
    -> i32;
    fn sqlite3_pager_shared_lock(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_checkpoint(p_pager_1: *mut Pager, _: *mut Sqlite3,
    _: i32, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_pager_wal_supported(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_wal_callback(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_open_wal(p_pager_1: *mut Pager, pis_open_1: *mut i32)
    -> i32;
    fn sqlite3_pager_close_wal(p_pager_1: *mut Pager, _: *mut Sqlite3)
    -> i32;
    fn sqlite3_pager_direct_read_ok(p_pager_1: *mut Pager, pgno: Pgno)
    -> i32;
    fn sqlite3_pager_isreadonly(_: *mut Pager)
    -> u8;
    fn sqlite3_pager_data_version(_: *mut Pager)
    -> u32;
    fn sqlite3_pager_mem_used(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_filename(_: *const Pager, _: i32)
    -> *const i8;
    fn sqlite3_pager_vfs(_: *mut Pager)
    -> *mut Sqlite3Vfs;
    fn sqlite3_pager_file(_: *mut Pager)
    -> *mut Sqlite3File;
    fn sqlite3_pager_jrnl_file(_: *mut Pager)
    -> *mut Sqlite3File;
    fn sqlite3_pager_journalname(_: *mut Pager)
    -> *const i8;
    fn sqlite3_pager_temp_space(_: *mut Pager)
    -> *mut ();
    fn sqlite3_pager_is_memdb(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_cache_stat(_: *mut Pager, _: i32, _: i32, _: *mut u64)
    -> ();
    fn sqlite3_pager_clear_cache(_: *mut Pager)
    -> ();
    fn sqlite3_sector_size(_: *mut Sqlite3File)
    -> i32;
    fn sqlite3_pager_truncate_image(_: *mut Pager, _: Pgno)
    -> ();
    fn sqlite3_pager_rekey(_: *mut DbPage, _: Pgno, _: u16)
    -> ();
    fn sqlite3_btree_open(p_vfs_1: *mut Sqlite3Vfs, z_filename_1: *const i8,
    db: *mut Sqlite3, pp_btree_1: *mut *mut Btree, flags: i32,
    vfs_flags_1: i32)
    -> i32;
    fn sqlite3_btree_close(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_set_cache_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_spill_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_mmap_limit(_: *mut Btree, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_btree_set_pager_flags(_: *mut Btree, _: u32)
    -> i32;
    fn sqlite3_btree_set_page_size(p: *mut Btree, n_pagesize_1: i32,
    n_reserve_1: i32, e_fix_1: i32)
    -> i32;
    fn sqlite3_btree_get_page_size(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_max_page_count(_: *mut Btree, _: Pgno)
    -> Pgno;
    fn sqlite3_btree_last_page(_: *mut Btree)
    -> Pgno;
    fn sqlite3_btree_secure_delete(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_get_requested_reserve(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_get_reserve_no_mutex(p: *mut Btree)
    -> i32;
    fn sqlite3_btree_set_auto_vacuum(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_get_auto_vacuum(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_begin_trans(_: *mut Btree, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_btree_commit_phase_one(_: *mut Btree, _: *const i8)
    -> i32;
    fn sqlite3_btree_commit_phase_two(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_commit(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_rollback(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_btree_begin_stmt(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_create_table(_: *mut Btree, _: *mut Pgno, flags: i32)
    -> i32;
    fn sqlite3_btree_txn_state(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_is_in_backup(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_schema(_: *mut Btree, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut ();
    fn sqlite3_btree_schema_locked(p_btree_1: *mut Btree)
    -> i32;
    fn sqlite3_btree_lock_table(p_btree_1: *mut Btree, i_tab_1: i32,
    is_write_lock_1: u8)
    -> i32;
    fn sqlite3_btree_savepoint(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_btree_checkpoint(_: *mut Btree, _: i32, _: *mut i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_btree_get_filename(_: *mut Btree)
    -> *const i8;
    fn sqlite3_btree_get_journalname(_: *mut Btree)
    -> *const i8;
    fn sqlite3_btree_copy_file(_: *mut Btree, _: *mut Btree)
    -> i32;
    fn sqlite3_btree_incr_vacuum(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_drop_table(_: *mut Btree, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_btree_clear_table(_: *mut Btree, _: i32, _: *mut i64)
    -> i32;
    fn sqlite3_btree_clear_table_of_cursor(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_trip_all_cursors(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_btree_get_meta(p_btree_1: *mut Btree, idx: i32,
    p_value_1: *mut u32)
    -> ();
    fn sqlite3_btree_update_meta(_: *mut Btree, idx: i32, value: u32)
    -> i32;
    fn sqlite3_btree_new_db(p: *mut Btree)
    -> i32;
    fn sqlite3_btree_cursor(_: *mut Btree, i_table_1: Pgno, wr_flag_1: i32,
    _: *mut KeyInfo, p_cursor_1: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_fake_valid_cursor()
    -> *mut BtCursor;
    fn sqlite3_btree_cursor_size()
    -> i32;
    fn sqlite3_btree_cursor_zero(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_cursor_hint_flags(_: *mut BtCursor, _: u32)
    -> ();
    fn sqlite3_btree_close_cursor(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_table_moveto(_: *mut BtCursor, int_key_1: i64, bias: i32,
    p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_index_moveto(_: *mut BtCursor,
    p_un_key_1: *mut UnpackedRecord, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_cursor_has_moved(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_cursor_restore(_: *mut BtCursor, _: *mut i32)
    -> i32;
    fn sqlite3_btree_delete(_: *mut BtCursor, flags: u8)
    -> i32;
    fn sqlite3_btree_insert(_: *mut BtCursor,
    p_payload_1: *const BtreePayload, flags: i32, seek_result_1: i32)
    -> i32;
    fn sqlite3_btree_first(_: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_is_empty(p_cur_1: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_last(_: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_next(_: *mut BtCursor, flags: i32)
    -> i32;
    fn sqlite3_btree_eof(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_previous(_: *mut BtCursor, flags: i32)
    -> i32;
    fn sqlite3_btree_integer_key(_: *mut BtCursor)
    -> i64;
    fn sqlite3_btree_cursor_pin(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_cursor_unpin(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_offset(_: *mut BtCursor)
    -> i64;
    fn sqlite3_btree_payload(_: *mut BtCursor, offset: u32, amt: u32,
    _: *mut ())
    -> i32;
    fn sqlite3_btree_payload_fetch(_: *mut BtCursor, p_amt_1: *mut u32)
    -> *const ();
    fn sqlite3_btree_payload_size(_: *mut BtCursor)
    -> u32;
    fn sqlite3_btree_max_record_size(_: *mut BtCursor)
    -> Sqlite3Int64;
    fn sqlite3_btree_integrity_check(db: *mut Sqlite3, p: *mut Btree,
    a_root_1: *mut Pgno, a_cnt_1: *mut Sqlite3Value, n_root_1: i32,
    mx_err_1: i32, pn_err_1: *mut i32, pz_out_1: *mut *mut i8)
    -> i32;
    fn sqlite3_btree_pager(_: *mut Btree)
    -> *mut Pager;
    fn sqlite3_btree_row_count_est(_: *mut BtCursor)
    -> i64;
    fn sqlite3_btree_payload_checked(_: *mut BtCursor, offset: u32, amt: u32,
    _: *mut ())
    -> i32;
    fn sqlite3_btree_put_data(_: *mut BtCursor, offset: u32, amt: u32,
    _: *mut ())
    -> i32;
    fn sqlite3_btree_incrblob_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_clear_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_set_version(p_bt_1: *mut Btree, i_version_1: i32)
    -> i32;
    fn sqlite3_btree_cursor_has_hint(_: *mut BtCursor, mask: u32)
    -> i32;
    fn sqlite3_btree_is_readonly(p_bt_1: *mut Btree)
    -> i32;
    fn sqlite3_header_size_btree()
    -> i32;
    fn sqlite3_btree_cursor_is_valid_nn(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_count(_: *mut Sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_btree_transfer_row(_: *mut BtCursor, _: *mut BtCursor, _: i64)
    -> i32;
    fn sqlite3_btree_clear_cache(_: *mut Btree)
    -> ();
    fn sqlite3_btree_enter(_: *mut Btree)
    -> ();
    fn sqlite3_btree_enter_all(_: *mut Sqlite3)
    -> ();
    fn sqlite3_btree_sharable(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_enter_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_connection_count(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_leave(_: *mut Btree)
    -> ();
    fn sqlite3_btree_leave_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_leave_all(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vdbe_create(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_vdbe_parser(_: *mut Vdbe)
    -> *mut Parse;
    fn sqlite3_vdbe_add_op0(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op1(_: *mut Vdbe, _: i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op2(_: *mut Vdbe, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_goto(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_load_string(_: *mut Vdbe, _: i32, _: *const i8)
    -> i32;
    fn sqlite3_vdbe_multi_load(_: *mut Vdbe, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_vdbe_add_op3(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op4(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32,
    z_p4_1: *const i8, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op4_dup8(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32,
    _: *const u8, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op4_int(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32,
    _: i32)
    -> i32;
    fn sqlite3_vdbe_add_function_call(_: *mut Parse, _: i32, _: i32, _: i32,
    _: i32, _: *const FuncDef, _: i32)
    -> i32;
    fn sqlite3_vdbe_end_coroutine(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_add_op_list(_: *mut Vdbe, n_op_1: i32,
    a_op_1: *const VdbeOpList, i_lineno_1: i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_explain(_: *mut Parse, _: u8, _: *const i8, ...)
    -> i32;
    fn sqlite3_vdbe_explain_pop(_: *mut Parse)
    -> ();
    fn sqlite3_vdbe_explain_parent(_: *mut Parse)
    -> i32;
    fn sqlite3_vdbe_add_parse_schema_op(_: *mut Vdbe, _: i32, _: *mut i8,
    _: u16)
    -> ();
    fn sqlite3_vdbe_change_opcode(_: *mut Vdbe, addr: i32, _: u8)
    -> ();
    fn sqlite3_vdbe_change_p1(_: *mut Vdbe, addr: i32, p1_1: i32)
    -> ();
    fn sqlite3_vdbe_change_p2(_: *mut Vdbe, addr: i32, p2_1: i32)
    -> ();
    fn sqlite3_vdbe_change_p3(_: *mut Vdbe, addr: i32, p3_1: i32)
    -> ();
    fn sqlite3_vdbe_change_p5(_: *mut Vdbe, p5_1: u16)
    -> ();
    fn sqlite3_vdbe_typeof_column(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_jump_here(_: *mut Vdbe, addr: i32)
    -> ();
    fn sqlite3_vdbe_jump_here_or_pop_inst(_: *mut Vdbe, addr: i32)
    -> ();
    fn sqlite3_vdbe_change_to_noop(_: *mut Vdbe, addr: i32)
    -> i32;
    fn sqlite3_vdbe_delete_prior_opcode(_: *mut Vdbe, op: u8)
    -> i32;
    fn sqlite3_vdbe_change_p4(_: *mut Vdbe, addr: i32, z_p4_1: *const i8,
    n_1: i32)
    -> ();
    fn sqlite3_vdbe_append_p4(_: *mut Vdbe, p_p4_1: *mut (), p4type: i32)
    -> ();
    fn sqlite3_vdbe_set_p4_key_info(_: *mut Parse, _: *mut Index)
    -> ();
    fn sqlite3_vdbe_uses_btree(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_get_op(_: *mut Vdbe, _: i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_get_last_op(_: *mut Vdbe)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_make_label(_: *mut Parse)
    -> i32;
    fn sqlite3_vdbe_run_only_once(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_reusable(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_delete(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_make_ready(_: *mut Vdbe, _: *mut Parse)
    -> ();
    fn sqlite3_vdbe_finalize(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_resolve_label(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_current_addr(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_reset_step_result(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_rewind(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_reset(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_set_num_cols(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_set_col_name(_: *mut Vdbe, _: i32, _: i32, _: *const i8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_count_changes(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_db(_: *mut Vdbe)
    -> *mut Sqlite3;
    fn sqlite3_vdbe_prepare_flags(_: *mut Vdbe)
    -> u8;
    fn sqlite3_vdbe_set_sql(_: *mut Vdbe, z: *const i8, n: i32, _: u8)
    -> ();
    fn sqlite3_vdbe_swap(_: *mut Vdbe, _: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_take_op_array(_: *mut Vdbe, _: *mut i32, _: *mut i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_get_bound_value(_: *mut Vdbe, _: i32, _: u8)
    -> *mut Sqlite3Value;
    fn sqlite3_vdbe_set_varmask(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_expand_sql(_: *mut Vdbe, _: *const i8)
    -> *mut i8;
    fn sqlite3_mem_compare(_: *const Mem, _: *const Mem, _: *const CollSeq)
    -> i32;
    fn sqlite3_blob_compare(_: *const Mem, _: *const Mem)
    -> i32;
    fn sqlite3_vdbe_func_name(_: *const Sqlite3Context)
    -> *const i8;
    fn sqlite3_vdbe_record_unpack(_: i32, _: *const (),
    _: *mut UnpackedRecord)
    -> ();
    fn sqlite3_vdbe_record_compare(_: i32, _: *const (),
    _: *mut UnpackedRecord)
    -> i32;
    fn sqlite3_vdbe_record_compare_with_skip(_: i32, _: *const (),
    _: *mut UnpackedRecord, _: i32)
    -> i32;
    fn sqlite3_vdbe_alloc_unpacked_record(_: *mut KeyInfo)
    -> *mut UnpackedRecord;
    fn sqlite3_vdbe_find_compare(_: *mut UnpackedRecord)
    -> unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32;
    fn sqlite3_vdbe_link_sub_program(_: *mut Vdbe, _: *mut SubProgram)
    -> ();
    fn sqlite3_vdbe_has_sub_program(_: *mut Vdbe)
    -> i32;
    fn sqlite3_mem_set_array_int64(a_mem_1: *mut Sqlite3Value, i_idx_1: i32,
    val: i64)
    -> ();
    fn sqlite3_not_pure_func(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_pcache_initialize()
    -> i32;
    fn sqlite3_pcache_shutdown()
    -> ();
    fn sqlite3_p_cache_buffer_setup(_: *mut (), sz: i32, n: i32)
    -> ();
    fn sqlite3_pcache_open(sz_page_1: i32, sz_extra_1: i32,
    b_purgeable_1: i32,
    x_stress_1: Option<unsafe extern "C" fn(*mut (), *mut PgHdr) -> i32>,
    p_stress_1: *mut (), p_to_init_1: *mut PCache)
    -> i32;
    fn sqlite3_pcache_set_page_size(_: *mut PCache, _: i32)
    -> i32;
    fn sqlite3_pcache_size()
    -> i32;
    fn sqlite3_pcache_fetch(_: *mut PCache, _: Pgno, create_flag_1: i32)
    -> *mut Sqlite3PcachePage;
    fn sqlite3_pcache_fetch_stress(_: *mut PCache, _: Pgno,
    _: *mut *mut Sqlite3PcachePage)
    -> i32;
    fn sqlite3_pcache_fetch_finish(_: *mut PCache, _: Pgno,
    p_page_1: *mut Sqlite3PcachePage)
    -> *mut PgHdr;
    fn sqlite3_pcache_release(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_drop(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_make_dirty(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_make_clean(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_clean_all(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear_writable(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_move(_: *mut PgHdr, _: Pgno)
    -> ();
    fn sqlite3_pcache_truncate(_: *mut PCache, x: Pgno)
    -> ();
    fn sqlite3_pcache_dirty_list(_: *mut PCache)
    -> *mut PgHdr;
    fn sqlite3_pcache_close(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear_sync_flags(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_ref_count(_: *mut PCache)
    -> i64;
    fn sqlite3_pcache_ref(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_page_refcount(_: *mut PgHdr)
    -> i64;
    fn sqlite3_pcache_pagecount(_: *mut PCache)
    -> i32;
    fn sqlite3_pcache_set_cachesize(_: *mut PCache, _: i32)
    -> ();
    fn sqlite3_pcache_set_spillsize(_: *mut PCache, _: i32)
    -> i32;
    fn sqlite3_pcache_shrink(_: *mut PCache)
    -> ();
    fn sqlite3_p_cache_set_default()
    -> ();
    fn sqlite3_header_size_pcache()
    -> i32;
    fn sqlite3_header_size_pcache1()
    -> i32;
    fn sqlite3_p_cache_percent_dirty(_: *mut PCache)
    -> i32;
    fn sqlite3_p_cache_is_dirty(p_cache_1: *mut PCache)
    -> i32;
    fn sqlite3_walk_expr(_: *mut Walker, _: *mut Expr)
    -> i32;
    fn sqlite3_walk_expr_nn(_: *mut Walker, _: *mut Expr)
    -> i32;
    fn sqlite3_walk_expr_list(_: *mut Walker, _: *mut ExprList)
    -> i32;
    fn sqlite3_walk_select(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walk_select_expr(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walk_select_from(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_expr_walk_noop(_: *mut Walker, _: *mut Expr)
    -> i32;
    fn sqlite3_select_walk_noop(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_select_walk_fail(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walker_depth_increase(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walker_depth_decrease(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_walk_win_defn_dummy_callback(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_select_pop_with(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_multi_values(p_parse_1: *mut Parse, p_left_1: *mut Select,
    p_row_1: *mut ExprList)
    -> *mut Select;
    fn sqlite3_multi_values_end(p_parse_1: *mut Parse, p_val_1: *mut Select)
    -> ();
    fn sqlite3_window_delete(_: *mut Sqlite3, _: *mut Window)
    -> ();
    fn sqlite3_window_unlink_from_select(_: *mut Window)
    -> ();
    fn sqlite3_window_list_delete(db: *mut Sqlite3, p: *mut Window)
    -> ();
    fn sqlite3_window_alloc(_: *mut Parse, _: i32, _: i32, _: *mut Expr,
    _: i32, _: *mut Expr, _: u8)
    -> *mut Window;
    fn sqlite3_window_attach(_: *mut Parse, _: *mut Expr, _: *mut Window)
    -> ();
    fn sqlite3_window_link(p_sel_1: *mut Select, p_win_1: *mut Window)
    -> ();
    fn sqlite3_window_compare(_: *const Parse, _: *const Window,
    _: *const Window, _: i32)
    -> i32;
    fn sqlite3_window_code_init(_: *mut Parse, _: *mut Select)
    -> ();
    fn sqlite3_window_code_step(_: *mut Parse, _: *mut Select,
    _: *mut WhereInfo, _: i32, _: i32)
    -> ();
    fn sqlite3_window_rewrite(_: *mut Parse, _: *mut Select)
    -> i32;
    fn sqlite3_window_update(_: *mut Parse, _: *mut Window, _: *mut Window,
    _: *mut FuncDef)
    -> ();
    fn sqlite3_window_dup(db: *mut Sqlite3, p_owner_1: *mut Expr,
    p: *mut Window)
    -> *mut Window;
    fn sqlite3_window_list_dup(db: *mut Sqlite3, p: *mut Window)
    -> *mut Window;
    fn sqlite3_window_functions()
    -> ();
    fn sqlite3_window_chain(_: *mut Parse, _: *mut Window, _: *mut Window)
    -> ();
    fn sqlite3_window_assemble(_: *mut Parse, _: *mut Window,
    _: *mut ExprList, _: *mut ExprList, _: *mut Token)
    -> *mut Window;
    fn sqlite3_report_error(i_err_1: i32, lineno: i32, z_type_1: *const i8)
    -> i32;
    fn sqlite3_corrupt_error(_: i32)
    -> i32;
    fn sqlite3_misuse_error(_: i32)
    -> i32;
    fn sqlite3_cantopen_error(_: i32)
    -> i32;
    fn sqlite3_is_id_char(_: u8)
    -> i32;
    fn sqlite3_str_i_cmp(_: *const i8, _: *const i8)
    -> i32;
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3ColumnType(_: *mut Column, _: *mut i8)
    -> *mut i8;
    fn sqlite3_malloc_init()
    -> i32;
    fn sqlite3_malloc_end()
    -> ();
    fn sqlite3Malloc(_: u64)
    -> *mut ();
    fn sqlite3_malloc_zero(_: u64)
    -> *mut ();
    fn sqlite3_db_malloc_zero(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw_nn(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_str_n_dup(_: *mut Sqlite3, _: *const i8, _: u64)
    -> *mut i8;
    fn sqlite3_db_span_dup(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut i8;
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc_or_free(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_db_malloc_size(_: *mut Sqlite3, _: *const ())
    -> i32;
    fn sqlite3_page_malloc(_: i32)
    -> *mut ();
    fn sqlite3_page_free(_: *mut ())
    -> ();
    fn sqlite3_mem_set_default()
    -> ();
    fn sqlite3_benign_malloc_hooks(_: Option<unsafe extern "C" fn() -> ()>,
    _: Option<unsafe extern "C" fn() -> ()>)
    -> ();
    fn sqlite3_heap_nearly_full()
    -> i32;
    fn sqlite3_default_mutex()
    -> *const Sqlite3MutexMethods;
    fn sqlite3_noop_mutex()
    -> *const Sqlite3MutexMethods;
    fn sqlite3MutexAlloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_mutex_init()
    -> i32;
    fn sqlite3_mutex_end()
    -> i32;
    fn sqlite3_memory_barrier()
    -> ();
    fn sqlite3_status_value(_: i32)
    -> Sqlite3Int64;
    fn sqlite3_status_up(_: i32, _: i32)
    -> ();
    fn sqlite3_status_down(_: i32, _: i32)
    -> ();
    fn sqlite3_status_highwater(_: i32, _: i32)
    -> ();
    fn sqlite3_lookaside_used(_: *mut Sqlite3, _: *mut i32)
    -> i32;
    fn sqlite3_pcache1_mutex()
    -> *mut Sqlite3Mutex;
    fn sqlite3_malloc_mutex()
    -> *mut Sqlite3Mutex;
    fn sqlite3_is_na_n(_: f64)
    -> i32;
    fn sqlite3_is_overflow(_: f64)
    -> i32;
    fn sqlite3_fp_decode(_: *mut FpDecode, _: f64, _: i32, _: i32)
    -> ();
    fn sqlite3_m_printf(_: *mut Sqlite3, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vm_printf(_: *mut Sqlite3, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_set_string(_: *mut *mut i8, _: *mut Sqlite3, _: *const i8)
    -> ();
    fn sqlite3_progress_check(_: *mut Parse)
    -> ();
    fn sqlite3_error_msg(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_error_to_parser(_: *mut Sqlite3, _: i32)
    -> i32;
    fn sqlite3_dequote(_: *mut i8)
    -> ();
    fn sqlite3_dequote_expr(_: *mut Expr)
    -> ();
    fn sqlite3_dequote_token(_: *mut Token)
    -> ();
    fn sqlite3_dequote_number(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_token_init(_: *mut Token, _: *mut i8)
    -> ();
    fn sqlite3_keyword_code(_: *const u8, _: i32)
    -> i32;
    fn sqlite3_run_parser(_: *mut Parse, _: *const i8)
    -> i32;
    fn sqlite3_finish_coding(_: *mut Parse)
    -> ();
    fn sqlite3_get_temp_reg(_: *mut Parse)
    -> i32;
    fn sqlite3_release_temp_reg(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_get_temp_range(_: *mut Parse, _: i32)
    -> i32;
    fn sqlite3_release_temp_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_clear_temp_reg_cache(_: *mut Parse)
    -> ();
    fn sqlite3_touch_register(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_expr_alloc(_: *mut Sqlite3, _: i32, _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr(_: *mut Sqlite3, _: i32, _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_int32(_: *mut Sqlite3, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_attach_subtrees(_: *mut Sqlite3, _: *mut Expr,
    _: *mut Expr, _: *mut Expr)
    -> ();
    fn sqlite3_p_expr(_: *mut Parse, _: i32, _: *mut Expr, _: *mut Expr)
    -> *mut Expr;
    fn sqlite3_p_expr_add_select(_: *mut Parse, _: *mut Expr, _: *mut Select)
    -> ();
    fn sqlite3_expr_and(_: *mut Parse, _: *mut Expr, _: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_simplified_and_or(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_function(_: *mut Parse, _: *mut ExprList, _: *const Token,
    _: i32)
    -> *mut Expr;
    fn sqlite3_expr_add_function_order_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> ();
    fn sqlite3_expr_order_by_aggregate_error(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_function_usable(_: *mut Parse, _: *const Expr,
    _: *const FuncDef)
    -> ();
    fn sqlite3_expr_assign_var_number(_: *mut Parse, _: *mut Expr, _: u32)
    -> ();
    fn sqlite3_expr_delete(_: *mut Sqlite3, _: *mut Expr)
    -> ();
    fn sqlite3_expr_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_expr_deferred_delete(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_expr_unmap_and_delete(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_list_append(_: *mut Parse, _: *mut ExprList, _: *mut Expr)
    -> *mut ExprList;
    fn sqlite3_expr_list_append_vector(_: *mut Parse, _: *mut ExprList,
    _: *mut IdList, _: *mut Expr)
    -> *mut ExprList;
    fn sqlite3_expr_list_to_values(_: *mut Parse, _: i32, _: *mut ExprList)
    -> *mut Select;
    fn sqlite3_expr_list_set_sort_order(_: *mut ExprList, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_list_set_name(_: *mut Parse, _: *mut ExprList,
    _: *const Token, _: i32)
    -> ();
    fn sqlite3_expr_list_set_span(_: *mut Parse, _: *mut ExprList,
    _: *const i8, _: *const i8)
    -> ();
    fn sqlite3_expr_list_delete(_: *mut Sqlite3, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_list_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_expr_list_flags(_: *const ExprList)
    -> u32;
    fn sqlite3_index_has_duplicate_root_page(_: *mut Index)
    -> i32;
    fn sqlite3_init(_: *mut Sqlite3, _: *mut *mut i8)
    -> i32;
    fn sqlite3_init_callback(_: *mut (), _: i32, _: *mut *mut i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_init_one(_: *mut Sqlite3, _: i32, _: *mut *mut i8, _: u32)
    -> i32;
    fn sqlite3_pragma(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: i32)
    -> ();
    fn sqlite3_pragma_vtab_register(_: *mut Sqlite3, z_name_1: *const i8)
    -> *mut Module;
    fn sqlite3_reset_all_schemas_of_connection(_: *mut Sqlite3)
    -> ();
    fn sqlite3_reset_one_schema(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_collapse_database_array(_: *mut Sqlite3)
    -> ();
    fn sqlite3_commit_internal_changes(_: *mut Sqlite3)
    -> ();
    fn sqlite3_column_set_expr(_: *mut Parse, _: *mut Table, _: *mut Column,
    _: *mut Expr)
    -> ();
    fn sqlite3_column_expr(_: *mut Table, _: *mut Column)
    -> *mut Expr;
    fn sqlite3_column_set_coll(_: *mut Sqlite3, _: *mut Column,
    z_coll_1: *const i8)
    -> ();
    fn sqlite3_column_coll(_: *mut Column)
    -> *const i8;
    fn sqlite3_delete_column_names(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_generate_column_names(p_parse_1: *mut Parse,
    p_select_1: *mut Select)
    -> ();
    fn sqlite3_columns_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: *mut i16, _: *mut *mut Column)
    -> i32;
    fn sqlite3_subquery_column_types(_: *mut Parse, _: *mut Table,
    _: *mut Select, _: i8)
    -> ();
    fn sqlite3_result_set_of_select(_: *mut Parse, _: *mut Select, _: i8)
    -> *mut Table;
    fn sqlite3_open_schema_table(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_primary_key_index(_: *mut Table)
    -> *mut Index;
    fn sqlite3_table_column_to_index(_: *mut Index, _: i32)
    -> i32;
    fn sqlite3_table_column_to_storage(_: *mut Table, _: i16)
    -> i16;
    fn sqlite3_storage_column_to_table(_: *mut Table, _: i16)
    -> i16;
    fn sqlite3_start_table(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: i32, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_add_column(_: *mut Parse, _: Token, _: Token)
    -> ();
    fn sqlite3_add_not_null(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_add_primary_key(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: i32)
    -> ();
    fn sqlite3_add_check_constraint(_: *mut Parse, _: *mut Expr, _: *const i8,
    _: *const i8)
    -> ();
    fn sqlite3_add_default_value(_: *mut Parse, _: *mut Expr, _: *const i8,
    _: *const i8)
    -> ();
    fn sqlite3_add_collate_type(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_add_generated(_: *mut Parse, _: *mut Expr, _: *mut Token)
    -> ();
    fn sqlite3_end_table(_: *mut Parse, _: *mut Token, _: *mut Token, _: u32,
    _: *mut Select)
    -> ();
    fn sqlite3_add_returning(_: *mut Parse, _: *mut ExprList)
    -> ();
    fn sqlite3_parse_uri(_: *const i8, _: *const i8, _: *mut u32,
    _: *mut *mut Sqlite3Vfs, _: *mut *mut i8, _: *mut *mut i8)
    -> i32;
    fn sqlite3_db_name_to_btree(_: *mut Sqlite3, _: *const i8)
    -> *mut Btree;
    fn sqlite3_fault_sim(_: i32)
    -> i32;
    fn sqlite3_bitvec_create(_: u32)
    -> *mut Bitvec;
    fn sqlite3_bitvec_test(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_test_not_null(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_set(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_clear(_: *mut Bitvec, _: u32, _: *mut ())
    -> ();
    fn sqlite3_bitvec_destroy(_: *mut Bitvec)
    -> ();
    fn sqlite3_bitvec_size(_: *mut Bitvec)
    -> u32;
    fn sqlite3_bitvec_builtin_test(_: i32, _: *mut i32)
    -> i32;
    fn sqlite3_row_set_init(_: *mut Sqlite3)
    -> *mut RowSet;
    fn sqlite3_row_set_delete(_: *mut ())
    -> ();
    fn sqlite3_row_set_clear(_: *mut ())
    -> ();
    fn sqlite3_row_set_insert(_: *mut RowSet, _: i64)
    -> ();
    fn sqlite3_row_set_test(_: *mut RowSet, i_batch_1: i32, _: i64)
    -> i32;
    fn sqlite3_row_set_next(_: *mut RowSet, _: *mut i64)
    -> i32;
    fn sqlite3_create_view(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: *mut ExprList, _: *mut Select, _: i32, _: i32)
    -> ();
    fn sqlite3_view_get_column_names(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_drop_table(_: *mut Parse, _: *mut SrcList, _: i32, _: i32)
    -> ();
    fn sqlite3_code_drop_table(_: *mut Parse, _: *mut Table, _: i32, _: i32)
    -> ();
    fn sqlite3_delete_table(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_delete_table_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_free_index(_: *mut Sqlite3, _: *mut Index)
    -> ();
    fn sqlite3_autoincrement_begin(p_parse_1: *mut Parse)
    -> ();
    fn sqlite3_autoincrement_end(p_parse_1: *mut Parse)
    -> ();
    fn sqlite3_insert(_: *mut Parse, _: *mut SrcList, _: *mut Select,
    _: *mut IdList, _: i32, _: *mut Upsert)
    -> ();
    fn sqlite3_compute_generated_columns(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    fn sqlite3_array_allocate(_: *mut Sqlite3, _: *mut (), _: i32,
    _: *mut i32, _: *mut i32)
    -> *mut ();
    fn sqlite3_id_list_append(_: *mut Parse, _: *mut IdList, _: *mut Token)
    -> *mut IdList;
    fn sqlite3_id_list_index(_: *mut IdList, _: *const i8)
    -> i32;
    fn sqlite3_src_list_enlarge(_: *mut Parse, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut SrcList;
    fn sqlite3_src_list_append_list(p_parse_1: *mut Parse, p1: *mut SrcList,
    p2: *mut SrcList)
    -> *mut SrcList;
    fn sqlite3_src_list_append(_: *mut Parse, _: *mut SrcList, _: *mut Token,
    _: *mut Token)
    -> *mut SrcList;
    fn sqlite3_subquery_delete(_: *mut Sqlite3, _: *mut Subquery)
    -> ();
    fn sqlite3_subquery_detach(_: *mut Sqlite3, _: *mut SrcItem)
    -> *mut Select;
    fn sqlite3_src_item_attach_subquery(_: *mut Parse, _: *mut SrcItem,
    _: *mut Select, _: i32)
    -> i32;
    fn sqlite3_src_list_append_from_term(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token, _: *mut Token, _: *mut Select,
    _: *mut OnOrUsing)
    -> *mut SrcList;
    fn sqlite3_src_list_indexed_by(_: *mut Parse, _: *mut SrcList,
    _: *mut Token)
    -> ();
    fn sqlite3_src_list_func_args(_: *mut Parse, _: *mut SrcList,
    _: *mut ExprList)
    -> ();
    fn sqlite3_indexed_by_lookup(_: *mut Parse, _: *mut SrcItem)
    -> i32;
    fn sqlite3_src_list_shift_join_type(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_src_list_assign_cursors(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_id_list_delete(_: *mut Sqlite3, _: *mut IdList)
    -> ();
    fn sqlite3_clear_on_or_using(_: *mut Sqlite3, _: *mut OnOrUsing)
    -> ();
    fn sqlite3_src_list_delete(_: *mut Sqlite3, _: *mut SrcList)
    -> ();
    fn sqlite3_allocate_index_object(_: *mut Sqlite3, _: i32, _: i32,
    _: *mut *mut i8)
    -> *mut Index;
    fn sqlite3_create_index(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut SrcList, _: *mut ExprList, _: i32, _: *mut Token, _: *mut Expr,
    _: i32, _: i32, _: u8)
    -> ();
    fn sqlite3_drop_index(_: *mut Parse, _: *mut SrcList, _: i32)
    -> ();
    fn sqlite3_select(_: *mut Parse, _: *mut Select, _: *mut SelectDest)
    -> i32;
    fn sqlite3_select_new(_: *mut Parse, _: *mut ExprList, _: *mut SrcList,
    _: *mut Expr, _: *mut ExprList, _: *mut Expr, _: *mut ExprList, _: u32,
    _: *mut Expr)
    -> *mut Select;
    fn sqlite3_select_delete(_: *mut Sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_select_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_select_check_on_clauses(p_parse_1: *mut Parse,
    p_select_1: *mut Select)
    -> ();
    fn sqlite3_src_list_lookup(_: *mut Parse, _: *mut SrcList)
    -> *mut Table;
    fn sqlite3_is_read_only(_: *mut Parse, _: *mut Table, _: *mut Trigger)
    -> i32;
    fn sqlite3_open_table(_: *mut Parse, i_cur_1: i32, i_db_1: i32,
    _: *mut Table, _: i32)
    -> ();
    fn sqlite3_code_change_count(_: *mut Vdbe, _: i32, _: *const i8)
    -> ();
    fn sqlite3_delete_from(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr)
    -> ();
    fn sqlite3_update(_: *mut Parse, _: *mut SrcList, _: *mut ExprList,
    _: *mut Expr, _: i32, _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> ();
    fn sqlite3_where_begin(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut ExprList, _: *mut Select, _: u16, _: i32)
    -> *mut WhereInfo;
    fn sqlite3_where_end(_: *mut WhereInfo)
    -> ();
    fn sqlite3_where_output_row_count(_: *mut WhereInfo)
    -> LogEst;
    fn sqlite3_where_is_distinct(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_is_ordered(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_order_by_limit_opt_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_min_max_opt_early_out(_: *mut Vdbe, _: *mut WhereInfo)
    -> ();
    fn sqlite3_where_is_sorted(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_continue_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_break_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_ok_one_pass(_: *mut WhereInfo, _: *mut i32)
    -> i32;
    fn sqlite3_where_uses_deferred_seek(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_expr_code_load_index_column(_: *mut Parse, _: *mut Index,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_get_column(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_code_get_column_of_table(_: *mut Vdbe, _: *mut Table,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_move(_: *mut Parse, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_to_register(p_expr_1: *mut Expr, i_reg_1: i32)
    -> ();
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_generated_column(_: *mut Parse, _: *mut Table,
    _: *mut Column, _: i32)
    -> ();
    fn sqlite3_expr_code_copy(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_factorable(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_run_just_once(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_null_register_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_temp(_: *mut Parse, _: *mut Expr, _: *mut i32)
    -> i32;
    fn sqlite3_expr_code_target(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_code_expr_list(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_if_true(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_if_false_dup(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_find_table(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut Table;
    fn sqlite3_locate_table(_: *mut Parse, flags: u32, _: *const i8,
    _: *const i8)
    -> *mut Table;
    fn sqlite3_preferred_table_name(_: *const i8)
    -> *const i8;
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_find_index(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut Index;
    fn sqlite3_unlink_and_delete_table(_: *mut Sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_unlink_and_delete_index(_: *mut Sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_vacuum(_: *mut Parse, _: *mut Token, _: *mut Expr)
    -> ();
    fn sqlite3_run_vacuum(_: *mut *mut i8, _: *mut Sqlite3, _: i32,
    _: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_name_from_token(_: *mut Sqlite3, _: *const Token)
    -> *mut i8;
    fn sqlite3_expr_compare(_: *const Parse, _: *const Expr, _: *const Expr,
    _: i32)
    -> i32;
    fn sqlite3_expr_compare_skip(_: *mut Expr, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn sqlite3_expr_implies_expr(_: *const Parse, _: *const Expr,
    _: *const Expr, _: i32)
    -> i32;
    fn sqlite3_expr_implies_non_null_row(_: *mut Expr, _: i32, _: i32)
    -> i32;
    fn sqlite3_agg_info_persist_walker_init(_: *mut Walker, _: *mut Parse)
    -> ();
    fn sqlite3_expr_analyze_aggregates(_: *mut NameContext, _: *mut Expr)
    -> ();
    fn sqlite3_expr_analyze_agg_list(_: *mut NameContext, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_covered_by_index(_: *mut Expr, i_cur_1: i32,
    p_idx_1: *mut Index)
    -> i32;
    fn sqlite3_references_src_list(_: *mut Parse, _: *mut Expr,
    _: *mut SrcList)
    -> i32;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_rollback_all(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_code_verify_named_schema(_: *mut Parse, z_db_1: *const i8)
    -> ();
    fn sqlite3_begin_transaction(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_end_transaction(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_savepoint(_: *mut Parse, _: i32, _: *mut Token)
    -> ();
    fn sqlite3_close_savepoints(_: *mut Sqlite3)
    -> ();
    fn sqlite3_leave_mutex_and_close_zombie(_: *mut Sqlite3)
    -> ();
    fn sqlite3_is_true_or_false(_: *const i8)
    -> u32;
    fn sqlite3_expr_id_to_true_false(_: *mut Expr)
    -> i32;
    fn sqlite3_expr_truth_value(_: *const Expr)
    -> i32;
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_is_constant_or_group_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> i32;
    fn sqlite3_expr_is_single_table_constraint(_: *mut Expr,
    _: *const SrcList, _: i32, _: i32)
    -> i32;
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_expr_can_be_null(_: *const Expr)
    -> i32;
    fn sqlite3_expr_needs_no_affinity_change(_: *const Expr, _: i8)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
    -> i32;
    fn sqlite3_rowid_alias(p_tab_1: *mut Table)
    -> *const i8;
    fn sqlite3_generate_row_delete(_: *mut Parse, _: *mut Table,
    _: *mut Trigger, _: i32, _: i32, _: i32, _: i16, _: u8, _: u8, _: u8,
    _: i32)
    -> ();
    fn sqlite3_generate_row_index_delete(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    fn sqlite3_resolve_part_idx_label(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_expr_references_updated_column(_: *mut Expr, _: *mut i32,
    _: i32)
    -> i32;
    fn sqlite3_generate_constraint_checks(_: *mut Parse, _: *mut Table,
    _: *mut i32, _: i32, _: i32, _: i32, _: i32, _: u8, _: u8, _: i32,
    _: *mut i32, _: *mut i32, _: *mut Upsert)
    -> ();
    fn sqlite3_complete_insertion(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: *mut i32, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_open_table_and_indices(_: *mut Parse, _: *mut Table, _: i32,
    _: u8, _: i32, _: *mut u8, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_begin_write_operation(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_multi_write(_: *mut Parse)
    -> ();
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    fn sqlite3_halt_constraint(_: *mut Parse, _: i32, _: i32, _: *mut i8,
    _: i8, _: u8)
    -> ();
    fn sqlite3_unique_constraint(_: *mut Parse, _: i32, _: *mut Index)
    -> ();
    fn sqlite3_rowid_constraint(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_list_dup(_: *mut Sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_src_list_dup(_: *mut Sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
    fn sqlite3_id_list_dup(_: *mut Sqlite3, _: *const IdList)
    -> *mut IdList;
    fn sqlite3_select_dup(_: *mut Sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_function_search(_: i32, _: *const i8)
    -> *mut FuncDef;
    fn sqlite3_insert_builtin_funcs(_: *mut FuncDef, _: i32)
    -> ();
    fn sqlite3_find_function(_: *mut Sqlite3, _: *const i8, _: i32, _: u8,
    _: u8)
    -> *mut FuncDef;
    fn sqlite3_quote_value(_: *mut StrAccum, _: *mut Sqlite3Value, _: i32)
    -> ();
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn sqlite3_register_builtin_functions()
    -> ();
    fn sqlite3_register_date_time_functions()
    -> ();
    fn sqlite3_register_json_functions()
    -> ();
    fn sqlite3_register_per_connection_builtin_functions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_json_vtab_register(_: *mut Sqlite3, _: *const i8)
    -> *mut Module;
    fn sqlite3_safety_check_ok(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_safety_check_sick_or_ok(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_change_cookie(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_with_dup(db: *mut Sqlite3, p: *mut With)
    -> *mut With;
    fn sqlite3_materialize_view(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_begin_trigger(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: i32, _: i32, _: *mut IdList, _: *mut SrcList, _: *mut Expr, _: i32,
    _: i32)
    -> ();
    fn sqlite3_finish_trigger(_: *mut Parse, _: *mut TriggerStep,
    _: *mut Token)
    -> ();
    fn sqlite3_drop_trigger(_: *mut Parse, _: *mut SrcList, _: i32)
    -> ();
    fn sqlite3_drop_trigger_ptr(_: *mut Parse, _: *mut Trigger)
    -> ();
    fn sqlite3_triggers_exist(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut ExprList, p_mask_1: *mut i32)
    -> *mut Trigger;
    fn sqlite3_trigger_list(_: *mut Parse, _: *mut Table)
    -> *mut Trigger;
    fn sqlite3_code_row_trigger(_: *mut Parse, _: *mut Trigger, _: i32,
    _: *mut ExprList, _: i32, _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_code_row_trigger_direct(_: *mut Parse, _: *mut Trigger,
    _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite_view_triggers(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: i32, _: *mut ExprList)
    -> ();
    fn sqlite3_delete_trigger_step(_: *mut Sqlite3, _: *mut TriggerStep)
    -> ();
    fn sqlite3_trigger_select_step(_: *mut Sqlite3, _: *mut Select,
    _: *const i8, _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_trigger_insert_step(_: *mut Parse, _: *mut SrcList,
    _: *mut IdList, _: *mut Select, _: u8, _: *mut Upsert, _: *const i8,
    _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_trigger_update_step(_: *mut Parse, _: *mut SrcList,
    _: *mut SrcList, _: *mut ExprList, _: *mut Expr, _: u8, _: *const i8,
    _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_trigger_delete_step(_: *mut Parse, _: *mut SrcList,
    _: *mut Expr, _: *const i8, _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_delete_trigger(_: *mut Sqlite3, _: *mut Trigger)
    -> ();
    fn sqlite3_unlink_and_delete_trigger(_: *mut Sqlite3, _: i32,
    _: *const i8)
    -> ();
    fn sqlite3_trigger_colmask(_: *mut Parse, _: *mut Trigger,
    _: *mut ExprList, _: i32, _: i32, _: *mut Table, _: i32)
    -> u32;
    fn sqlite3_join_type(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token)
    -> i32;
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
    fn sqlite3_src_item_column_used(_: *mut SrcItem, _: i32)
    -> ();
    fn sqlite3_set_join_expr(_: *mut Expr, _: i32, _: u32)
    -> ();
    fn sqlite3_create_foreign_key(_: *mut Parse, _: *mut ExprList,
    _: *mut Token, _: *mut ExprList, _: i32)
    -> ();
    fn sqlite3_defer_foreign_key(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_auth_read(_: *mut Parse, _: *mut Expr, _: *mut Schema,
    _: *mut SrcList)
    -> ();
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_auth_context_push(_: *mut Parse, _: *mut AuthContext,
    _: *const i8)
    -> ();
    fn sqlite3_auth_context_pop(_: *mut AuthContext)
    -> ();
    fn sqlite3_auth_read_col(_: *mut Parse, _: *const i8, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_db_is_named(db: *mut Sqlite3, i_db_1: i32, z_name_1: *const i8)
    -> i32;
    fn sqlite3_attach(_: *mut Parse, _: *mut Expr, _: *mut Expr, _: *mut Expr)
    -> ();
    fn sqlite3_detach(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_fix_init(_: *mut DbFixer, _: *mut Parse, _: i32, _: *const i8,
    _: *const Token)
    -> ();
    fn sqlite3_fix_src_list(_: *mut DbFixer, _: *mut SrcList)
    -> i32;
    fn sqlite3_fix_select(_: *mut DbFixer, _: *mut Select)
    -> i32;
    fn sqlite3_fix_expr(_: *mut DbFixer, _: *mut Expr)
    -> i32;
    fn sqlite3_fix_trigger_step(_: *mut DbFixer, _: *mut TriggerStep)
    -> i32;
    fn sqlite3_real_same_as_int(_: f64, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_real_to_i64(_: f64)
    -> i64;
    fn sqlite3_int64_to_text(_: i64, _: *mut i8)
    -> i32;
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
    -> i32;
    fn sqlite3_get_int32(_: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_get_u_int32(_: *const i8, _: *mut u32)
    -> i32;
    fn sqlite3_atoi(_: *const i8)
    -> i32;
    fn sqlite3_utf16_byte_len(p_data_1: *const (), n_byte_1: i32,
    n_char_1: i32)
    -> i32;
    fn sqlite3_utf8_char_len(p_data_1: *const i8, n_byte_1: i32)
    -> i32;
    fn sqlite3_utf8_read(_: *mut *const u8)
    -> u32;
    fn sqlite3_utf8_read_limited(_: *const u8, _: i32, _: *mut u32)
    -> i32;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_log_est_add(_: LogEst, _: LogEst)
    -> LogEst;
    fn sqlite3_log_est_from_double(_: f64)
    -> LogEst;
    fn sqlite3_log_est_to_int(_: LogEst)
    -> u64;
    fn sqlite3_v_list_add(_: *mut Sqlite3, _: *mut VList, _: *const i8,
    _: i32, _: i32)
    -> *mut VList;
    fn sqlite3_v_list_num_to_name(_: *mut VList, _: i32)
    -> *const i8;
    fn sqlite3_v_list_name_to_num(_: *mut VList, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_put_varint(_: *mut u8, _: u64)
    -> i32;
    fn sqlite3_get_varint(_: *const u8, _: *mut u64)
    -> u8;
    fn sqlite3_get_varint32(_: *const u8, _: *mut u32)
    -> u8;
    fn sqlite3_varint_len(v: u64)
    -> i32;
    fn sqlite3_index_affinity_str(_: *mut Sqlite3, _: *mut Index)
    -> *const i8;
    fn sqlite3_table_affinity_str(_: *mut Sqlite3, _: *const Table)
    -> *mut i8;
    fn sqlite3_table_affinity(_: *mut Vdbe, _: *mut Table, _: i32)
    -> ();
    fn sqlite3_compare_affinity(p_expr_1: *const Expr, aff2: i8)
    -> i8;
    fn sqlite3_index_affinity_ok(p_expr_1: *const Expr, idx_affinity: i8)
    -> i32;
    fn sqlite3_table_column_affinity(_: *const Table, _: i32)
    -> i8;
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite3_expr_data_type(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_atoi64(_: *const i8, _: *mut i64, _: i32, _: u8)
    -> i32;
    fn sqlite3_dec_or_hex_to_i64(_: *const i8, _: *mut i64)
    -> i32;
    fn sqlite3_error_with_msg(_: *mut Sqlite3, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_error_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_system_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_hex_to_blob(_: *mut Sqlite3, z: *const i8, n: i32)
    -> *mut ();
    fn sqlite3_hex_to_int(h: i32)
    -> u8;
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
    fn sqlite3_memdb_init()
    -> i32;
    fn sqlite3_is_memdb(_: *const Sqlite3Vfs)
    -> i32;
    fn sqlite3_err_str(_: i32)
    -> *const i8;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_find_coll_seq(_: *mut Sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut Sqlite3, _: u8)
    -> ();
    fn sqlite3_expr_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_nn_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_coll_seq_match(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> i32;
    fn sqlite3_expr_add_collate_token(p_parse_1: *const Parse, _: *mut Expr,
    _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_add_collate_string(_: *const Parse, _: *mut Expr,
    _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_skip_collate(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_check_coll_seq(_: *mut Parse, _: *mut CollSeq)
    -> i32;
    fn sqlite3_writable_schema(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_check_object_name(_: *mut Parse, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_vdbe_set_changes(_: *mut Sqlite3, _: i64)
    -> ();
    fn sqlite3_add_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_sub_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_mul_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_abs_int32(_: i32)
    -> i32;
    fn sqlite3_get_boolean(z: *const i8, _: u8)
    -> u8;
    fn sqlite3ValueText(_: *mut Sqlite3Value, _: u8)
    -> *const ();
    fn sqlite3_value_is_of_class(_: *const Sqlite3Value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3ValueBytes(_: *mut Sqlite3Value, _: u8)
    -> i32;
    fn sqlite3_value_set_str(_: *mut Sqlite3Value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_value_set_null(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_result_int_real(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_value_new(_: *mut Sqlite3)
    -> *mut Sqlite3Value;
    fn sqlite3_utf16to8(_: *mut Sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_apply_affinity(_: *mut Sqlite3Value, _: u8, _: u8)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    static sqlite3_str_binary: [i8; 0];
    static sqlite3_std_type_len: [u8; 0];
    static sqlite3_std_type_affinity: [i8; 0];
    static mut sqlite3_std_type: [*const i8; 0];
    static sqlite3_upper_to_lower: [u8; 0];
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
    static sqlite3_ctype_map: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: Sqlite3Str;
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_root_page_moved(_: *mut Sqlite3, _: i32, _: Pgno, _: Pgno)
    -> ();
    fn sqlite3_reindex(_: *mut Parse, _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_alter_functions()
    -> ();
    fn sqlite3_alter_rename_table(_: *mut Parse, _: *mut SrcList,
    _: *mut Token)
    -> ();
    fn sqlite3_alter_rename_column(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_alter_drop_constraint(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_alter_add_constraint(p_parse_1: *mut Parse,
    p_src_1: *mut SrcList, p_first_1: *mut Token, p_name_1: *mut Token,
    z_expr_1: *const i8, n_expr_1: i32, p_expr_1: *mut Expr)
    -> ();
    fn sqlite3_alter_set_not_null(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_get_token(_: *const u8, _: *mut i32)
    -> i64;
    fn sqlite3_nested_parse(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_expire_prepared_statements(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_code_rhs_of_in(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_code_subselect(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_select_prep(_: *mut Parse, _: *mut Select, _: *mut NameContext)
    -> ();
    fn sqlite3_expand_subquery(_: *mut Parse, _: *mut SrcItem)
    -> i32;
    fn sqlite3_select_wrong_num_terms_error(p_parse_1: *mut Parse,
    p: *mut Select)
    -> ();
    fn sqlite3_match_e_name(_: *const ExprListItem, _: *const i8,
    _: *const i8, _: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_expr_col_used(_: *mut Expr)
    -> Bitmask;
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_select_names(_: *mut Parse, _: *mut Select,
    _: *mut NameContext)
    -> ();
    fn sqlite3_resolve_self_reference(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut Expr, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_order_group_by(_: *mut Parse, _: *mut Select,
    _: *mut ExprList, _: *const i8)
    -> i32;
    fn sqlite3_column_default(_: *mut Vdbe, _: *mut Table, _: i32, _: i32)
    -> ();
    fn sqlite3_alter_finish_add_column(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_alter_begin_add_column(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_alter_drop_column(_: *mut Parse, _: *mut SrcList,
    _: *const Token)
    -> ();
    fn sqlite3_rename_token_map(_: *mut Parse, _: *const (), _: *const Token)
    -> *const ();
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_rename_expr_unmap(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_rename_exprlist_unmap(_: *mut Parse, _: *mut ExprList)
    -> ();
    fn sqlite3_get_coll_seq(_: *mut Parse, _: u8, _: *mut CollSeq,
    _: *const i8)
    -> *mut CollSeq;
    fn sqlite3_affinity_type(_: *const i8, _: *mut Column)
    -> i8;
    fn sqlite3_analyze(_: *mut Parse, _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_invoke_busy_handler(_: *mut BusyHandler)
    -> i32;
    fn sqlite3_find_db(_: *mut Sqlite3, _: *mut Token)
    -> i32;
    fn sqlite3_find_db_name(_: *mut Sqlite3, _: *const i8)
    -> i32;
    fn sqlite3_analysis_load(_: *mut Sqlite3, i_db_1: i32)
    -> i32;
    fn sqlite3_delete_index_samples(_: *mut Sqlite3, _: *mut Index)
    -> ();
    fn sqlite3_default_row_est(_: *mut Index)
    -> ();
    fn sqlite3_register_like_functions(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_is_like_function(_: *mut Sqlite3, _: *mut Expr, _: *mut i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_schema_clear(_: *mut ())
    -> ();
    fn sqlite3_schema_get(_: *mut Sqlite3, _: *mut Btree)
    -> *mut Schema;
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_key_info_alloc(_: *mut Sqlite3, _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_key_info_unref(_: *mut KeyInfo)
    -> ();
    fn sqlite3_key_info_ref(_: *mut KeyInfo)
    -> *mut KeyInfo;
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_key_info_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_select_op_name(_: i32)
    -> *const i8;
    fn sqlite3_has_explicit_nulls(_: *mut Parse, _: *mut ExprList)
    -> i32;
    fn sqlite3_create_func(_: *mut Sqlite3, _: *const i8, _: i32, _: i32,
    _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    _:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    _: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    _: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    _:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    p_destructor_1: *mut FuncDestructor)
    -> i32;
    fn sqlite3_noop_destructor(_: *mut ())
    -> ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_oom_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_api_exit(db: *mut Sqlite3, _: i32)
    -> i32;
    fn sqlite3_open_temp_database(_: *mut Parse)
    -> i32;
    fn sqlite3_rc_str_ref(_: *mut i8)
    -> *mut i8;
    fn sqlite3_rc_str_unref(_: *mut ())
    -> ();
    fn sqlite3_rc_str_new(_: u64)
    -> *mut i8;
    fn sqlite3_rc_str_resize(_: *mut i8, _: u64)
    -> *mut i8;
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut Sqlite3, _: *mut i8,
    _: i32, _: i32)
    -> ();
    fn sqlite3_str_accum_enlarge(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_enlarge_if_needed(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_finish(_: *mut StrAccum)
    -> *mut i8;
    fn sqlite3_str_accum_set_error(_: *mut StrAccum, _: u8)
    -> ();
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_create_column_expr(_: *mut Sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_record_error_byte_offset(_: *mut Sqlite3, _: *const i8)
    -> ();
    fn sqlite3_record_error_offset_of_expr(_: *mut Sqlite3, _: *const Expr)
    -> ();
    fn sqlite3_backup_restart(_: *mut Sqlite3Backup)
    -> ();
    fn sqlite3_backup_update(_: *mut Sqlite3Backup, _: Pgno, _: *const u8)
    -> ();
    fn sqlite3_expr_check_in(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_parser_alloc(_: Option<unsafe extern "C" fn(u64) -> *mut ()>,
    _: *mut Parse)
    -> *mut ();
    fn sqlite3_parser_free(_: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_parser(_: *mut (), _: i32, _: Token)
    -> ();
    fn sqlite3_parser_fallback(_: i32)
    -> i32;
    fn sqlite3_auto_load_extensions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_close_extensions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_vtab_clear(db: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_disconnect(db: *mut Sqlite3, p: *mut Table)
    -> ();
    fn sqlite3_vtab_sync(db: *mut Sqlite3, _: *mut Vdbe)
    -> i32;
    fn sqlite3_vtab_rollback(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_commit(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_lock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_unlock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_module_unref(_: *mut Sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_unlock_list(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vtab_savepoint(_: *mut Sqlite3, _: i32, _: i32)
    -> i32;
    fn sqlite3_vtab_import_errmsg(_: *mut Vdbe, _: *mut Sqlite3Vtab)
    -> ();
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_vtab_create_module(_: *mut Sqlite3, _: *const i8,
    _: *const Sqlite3Module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
    fn sqlite3_read_only_shadow_tables(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_shadow_table_name(db: *mut Sqlite3, z_name_1: *const i8)
    -> i32;
    fn sqlite3_is_shadow_table_of(_: *mut Sqlite3, _: *mut Table,
    _: *const i8)
    -> i32;
    fn sqlite3_mark_all_shadow_tables_of(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_eponymous_table_init(_: *mut Parse, _: *mut Module)
    -> i32;
    fn sqlite3_vtab_eponymous_table_clear(_: *mut Sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_make_writable(_: *mut Parse, _: *mut Table)
    -> ();
    fn sqlite3_vtab_begin_parse(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: i32)
    -> ();
    fn sqlite3_vtab_finish_parse(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_vtab_arg_init(_: *mut Parse)
    -> ();
    fn sqlite3_vtab_arg_extend(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_vtab_call_create(_: *mut Sqlite3, _: i32, _: *const i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_vtab_call_connect(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_vtab_call_destroy(_: *mut Sqlite3, _: i32, _: *const i8)
    -> i32;
    fn sqlite3_vtab_begin(_: *mut Sqlite3, _: *mut VTable)
    -> i32;
    fn sqlite3_vtab_overload_function(_: *mut Sqlite3, _: *mut FuncDef,
    n_arg_1: i32, _: *mut Expr)
    -> *mut FuncDef;
    fn sqlite3_vtab_uses_all_schemas(_: *mut Parse)
    -> ();
    fn sqlite3_stmt_current_time(_: *mut Sqlite3Context)
    -> Sqlite3Int64;
    fn sqlite3_vdbe_parameter_index(_: *mut Vdbe, _: *const i8, _: i32)
    -> i32;
    fn sqlite3TransferBindings(_: *mut Sqlite3Stmt, _: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_parse_object_init(_: *mut Parse, _: *mut Sqlite3)
    -> ();
    fn sqlite3_parse_object_reset(_: *mut Parse)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
    fn sqlite3_expr_compare_coll_seq(_: *mut Parse, _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_binary_compare_coll_seq(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_temp_in_memory(_: *const Sqlite3)
    -> i32;
    fn sqlite3_journal_modename(_: i32)
    -> *const i8;
    fn sqlite3_checkpoint(_: *mut Sqlite3, _: i32, _: i32, _: *mut i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_wal_default_hook(_: *mut (), _: *mut Sqlite3, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_cte_new(_: *mut Parse, _: *mut Token, _: *mut ExprList,
    _: *mut Select, _: u8)
    -> *mut Cte;
    fn sqlite3_cte_delete(_: *mut Sqlite3, _: *mut Cte)
    -> ();
    fn sqlite3_with_add(_: *mut Parse, _: *mut With, _: *mut Cte)
    -> *mut With;
    fn sqlite3_with_delete(_: *mut Sqlite3, _: *mut With)
    -> ();
    fn sqlite3_with_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_with_push(_: *mut Parse, _: *mut With, _: u8)
    -> *mut With;
    fn sqlite3_upsert_new(_: *mut Sqlite3, _: *mut ExprList, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_upsert_delete(_: *mut Sqlite3, _: *mut Upsert)
    -> ();
    fn sqlite3_upsert_dup(_: *mut Sqlite3, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_upsert_analyze_target(_: *mut Parse, _: *mut SrcList,
    _: *mut Upsert, _: *mut Upsert)
    -> i32;
    fn sqlite3_upsert_do_update(_: *mut Parse, _: *mut Upsert, _: *mut Table,
    _: *mut Index, _: i32)
    -> ();
    fn sqlite3_upsert_of_index(_: *mut Upsert, _: *mut Index)
    -> *mut Upsert;
    fn sqlite3_upsert_next_is_ipk(_: *mut Upsert)
    -> i32;
    fn sqlite3_fk_check(_: *mut Parse, _: *mut Table, _: i32, _: i32,
    _: *mut i32, _: i32)
    -> ();
    fn sqlite3_fk_drop_table(_: *mut Parse, _: *mut SrcList, _: *mut Table)
    -> ();
    fn sqlite3_fk_actions(_: *mut Parse, _: *mut Table, _: *mut ExprList,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_fk_required(_: *mut Parse, _: *mut Table, _: *mut i32, _: i32)
    -> i32;
    fn sqlite3_fk_oldmask(_: *mut Parse, _: *mut Table)
    -> u32;
    fn sqlite3_fk_references(_: *mut Table)
    -> *mut FKey;
    fn sqlite3_fk_clear_trigger_cache(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_fk_delete(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_fk_locate_index(_: *mut Parse, _: *mut Table, _: *mut FKey,
    _: *mut *mut Index, _: *mut *mut i32)
    -> i32;
    fn sqlite3_begin_benign_malloc()
    -> ();
    fn sqlite3_end_benign_malloc()
    -> ();
    fn sqlite3_find_in_index(_: *mut Parse, _: *mut Expr, _: u32, _: *mut i32,
    _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_journal_open(_: *mut Sqlite3Vfs, _: *const i8,
    _: *mut Sqlite3File, _: i32, _: i32)
    -> i32;
    fn sqlite3_journal_size(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_journal_is_in_memory(p: *mut Sqlite3File)
    -> i32;
    fn sqlite3_mem_journal_open(_: *mut Sqlite3File)
    -> ();
    fn sqlite3_expr_set_height_and_flags(p_parse_1: *mut Parse, p: *mut Expr)
    -> ();
    fn sqlite3_select_expr_height(_: *const Select)
    -> i32;
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
    -> i32;
    fn sqlite3_expr_set_error_offset(_: *mut Expr, _: i32)
    -> ();
    fn sqlite3_get4byte(_: *const u8)
    -> u32;
    fn sqlite3_put4byte(_: *mut u8, _: u32)
    -> ();
    fn sqlite3_thread_create(_: *mut *mut SQLiteThread,
    _: Option<unsafe extern "C" fn(*mut ()) -> *mut ()>, _: *mut ())
    -> i32;
    fn sqlite3_thread_join(_: *mut SQLiteThread, _: *mut *mut ())
    -> i32;
    fn sqlite3_expr_vector_size(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_expr_is_vector(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_vector_field_subexpr(_: *mut Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_for_vector_field(_: *mut Parse, _: *mut Expr, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_vector_error_msg(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_compile_options(pn_opt_1: *mut i32)
    -> *mut *const i8;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn __builtin_unreachable()
    -> ();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CCurHint {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CheckOnCtx {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CoveringIndexCheck {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IdxCover {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RefSrcList {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RenameCtx {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WhereConst {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WindowRewrite {
    _opaque: [u8; 0],
}
