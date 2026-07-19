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
    Sqlite3IndexInfo, Sqlite3Int64, Sqlite3IoMethods, Sqlite3Module,
    Sqlite3Mutex, Sqlite3MutexMethods, Sqlite3PcachePage,
    Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt,
    Sqlite3Uint64, Sqlite3Value, Sqlite3Vfs, Sqlite3Vtab,
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

type Int32T = i32;

type DarwinDevT = Int32T;

type DevT = DarwinDevT;

type Uint16T = u16;

type DarwinModeT = Uint16T;

type ModeT = DarwinModeT;

type Int64T = i64;

type DarwinOffT = Int64T;

type OffT = DarwinOffT;

type DarwinPidT = Int32T;

type PidT = DarwinPidT;

type DarwinSizeT = u64;

type DarwinSsizeT = i64;

type DarwinTimeT = i64;

type DarwinUuidT = [u8; 16];

type UuidT = DarwinUuidT;

type Uint32T = u32;

type DarwinUidT = Uint32T;

type UidT = DarwinUidT;

type DarwinGidT = Uint32T;

type GidT = DarwinGidT;

type TimeT = DarwinTimeT;

type NlinkT = Uint16T;

type Uint64T = u64;

type DarwinIno64T = Uint64T;

type DarwinBlkcntT = Int64T;

type BlkcntT = DarwinBlkcntT;

type DarwinBlksizeT = Int32T;

type BlksizeT = DarwinBlksizeT;

type DarwinSusecondsT = Int32T;

#[repr(C)]
#[derive(Copy, Clone)]
struct Fsid {
    val: [i32; 2],
}

type FsidT = Fsid;

#[repr(C)]
#[derive(Copy, Clone)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atimespec: Timespec,
    st_mtimespec: Timespec,
    st_ctimespec: Timespec,
    st_birthtimespec: Timespec,
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
struct Flock {
    l_start: i64,
    l_len: i64,
    l_pid: i32,
    l_type: i16,
    l_whence: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Statfs {
    f_bsize: u32,
    f_iosize: i32,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: Fsid,
    f_owner: u32,
    f_type: u32,
    f_flags: u32,
    f_fssubtype: u32,
    f_fstypename: [i8; 16],
    f_mntonname: [i8; 1024],
    f_mntfromname: [i8; 1024],
    f_flags_ext: u32,
    f_reserved: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i32,
}

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

#[repr(C)]
#[derive(Copy, Clone)]
struct UnixFile {
    p_method: *const Sqlite3IoMethods,
    p_vfs: *mut Sqlite3Vfs,
    p_inode: *mut UnixInodeInfo,
    h: i32,
    e_file_lock: u8,
    ctrl_flags: u16,
    last_errno: i32,
    locking_context: *mut (),
    p_preallocated_unused: *mut UnixUnusedFd,
    z_path: *const i8,
    p_shm: *mut UnixShm,
    sz_chunk: i32,
    n_fetch_out: i32,
    mmap_size: Sqlite3Int64,
    mmap_size_actual: Sqlite3Int64,
    mmap_size_max: Sqlite3Int64,
    p_map_region: *mut (),
    sector_size: i32,
    device_characteristics: i32,
    open_flags: i32,
    fs_flags: u32,
}

///* An instance of the following structure is allocated for each open
///* inode.
///*
///* A single inode can have multiple file descriptors, so each unixFile
///* structure contains a pointer to an instance of this object and this
///* object keeps a count of the number of unixFile pointing to it.
///*
///* Mutex rules:
///*
///*  (1) Only the pLockMutex mutex must be held in order to read or write
///*      any of the locking fields:
///*          nShared, nLock, eFileLock, bProcessLock, pUnused
///*
///*  (2) When nRef>0, then the following fields are unchanging and can
///*      be read (but not written) without holding any mutex:
///*          fileId, pLockMutex
///*
///*  (3) With the exceptions above, all the fields may only be read
///*      or written while holding the global unixBigLock mutex.
///*
///* Deadlock prevention:  The global unixBigLock mutex may not
///* be acquired while holding the pLockMutex mutex.  If both unixBigLock
///* and pLockMutex are needed, then unixBigLock must be acquired first.
#[repr(C)]
#[derive(Copy, Clone)]
struct UnixInodeInfo {
    file_id: UnixFileId,
    p_lock_mutex: *mut Sqlite3Mutex,
    n_shared: i32,
    n_lock: i32,
    e_file_lock: u8,
    b_process_lock: u8,
    p_unused: *mut UnixUnusedFd,
    n_ref: i32,
    p_shm_node: *mut UnixShmNode,
    p_next: *mut UnixInodeInfo,
    p_prev: *mut UnixInodeInfo,
    shared_byte: u64,
}

///* An instance of the following structure serves as the key used
///* to locate a particular unixInodeInfo object.
#[repr(C)]
#[derive(Copy, Clone)]
struct UnixFileId {
    dev: DevT,
    ino: u64,
}

///* Sometimes, after a file handle is closed by SQLite, the file descriptor
///* cannot be closed immediately. In these cases, instances of the following
///* structure are used to store the file descriptor while waiting for an
///* opportunity to either close or reuse it.
#[repr(C)]
#[derive(Copy, Clone)]
struct UnixUnusedFd {
    fd: i32,
    flags: i32,
    p_next: *mut UnixUnusedFd,
}

///* Object used to represent an shared memory buffer.
///*
///* When multiple threads all reference the same wal-index, each thread
///* has its own unixShm object, but they all point to a single instance
///* of this unixShmNode object.  In other words, each wal-index is opened
///* only once per process.
///*
///* Each unixShmNode object is connected to a single unixInodeInfo object.
///* We could coalesce this object into unixInodeInfo, but that would mean
///* every open file that does not use shared memory (in other words, most
///* open files) would have to carry around this extra information.  So
///* the unixInodeInfo object contains a pointer to this unixShmNode object
///* and the unixShmNode object is created only when needed.
///*
///* unixMutexHeld() must be true when creating or destroying
///* this object or while reading or writing the following fields:
///*
///*      nRef
///*
///* The following fields are read-only after the object is created:
///*
///*      hShm
///*      zFilename
///*
///* Either unixShmNode.pShmMutex must be held or unixShmNode.nRef==0 and
///* unixMutexHeld() is true when reading or writing any other field
///* in this structure.
///*
///* aLock[SQLITE_SHM_NLOCK]:
///*   This array records the various locks held by clients on each of the
///*   SQLITE_SHM_NLOCK slots. If the aLock[] entry is set to 0, then no
///*   locks are held by the process on this slot. If it is set to -1, then
///*   some client holds an EXCLUSIVE lock on the locking slot. If the aLock[]
///*   value is set to a positive value, then it is the number of shared 
///*   locks currently held on the slot.
///*
///* aMutex[SQLITE_SHM_NLOCK]:
///*   Normally, when SQLITE_ENABLE_SETLK_TIMEOUT is not defined, mutex 
///*   pShmMutex is used to protect the aLock[] array and the right to
///*   call fcntl() on unixShmNode.hShm to obtain or release locks.
///*
///*   If SQLITE_ENABLE_SETLK_TIMEOUT is defined though, we use an array
///*   of mutexes - one for each locking slot. To read or write locking
///*   slot aLock[iSlot], the caller must hold the corresponding mutex
///*   aMutex[iSlot]. Similarly, to call fcntl() to obtain or release a
///*   lock corresponding to slot iSlot, mutex aMutex[iSlot] must be held.
#[repr(C)]
#[derive(Copy, Clone)]
struct UnixShmNode {
    p_inode: *mut UnixInodeInfo,
    p_shm_mutex: *mut Sqlite3Mutex,
    z_filename: *mut i8,
    h_shm: i32,
    sz_region: i32,
    n_region: u16,
    is_readonly: u8,
    is_unlocked: u8,
    ap_region: *mut *mut i8,
    n_ref: i32,
    p_first: *mut UnixShm,
    a_lock: [i32; 8],
}

///* Structure used internally by this VFS to record the state of an
///* open shared memory connection.
///*
///* The following fields are initialized when this object is created and
///* are read-only thereafter:
///*
///*    unixShm.pShmNode
///*    unixShm.id
///*
///* All other fields are read/write.  The unixShm.pShmNode->pShmMutex must
///* be held while accessing any read/write fields.
#[repr(C)]
#[derive(Copy, Clone)]
struct UnixShm {
    p_shm_node: *mut UnixShmNode,
    p_next: *mut UnixShm,
    has_mutex: u8,
    id: u8,
    shared_mask: u16,
    excl_mask: u16,
}

///* Many system calls are accessed through pointer-to-functions so that
///* they may be overridden at runtime to facilitate fault injection during
///* testing and sandboxing.  The following array holds the names and pointers
///* to all overrideable system calls.
#[repr(C)]
#[derive(Copy, Clone)]
struct UnixSyscall {
    z_name: *const i8,
    p_current: Option<unsafe extern "C" fn() -> ()>,
    p_default: Option<unsafe extern "C" fn() -> ()>,
}

///* Different Unix systems declare open() in different ways.  Same use
///* open(const char*,int,mode_t).  Others use open(const char*,int,...).
///* The difference is important when using a pointer to the function.
///*
///* The safest way to deal with the problem is to always use this wrapper
///* which always has the same well-defined interface.
extern "C" fn posix_open(z_file_1: *const i8, flags: i32, mode: i32) -> i32 {
    return unsafe { open(z_file_1, flags, mode) };
}

///* Invoke open().  Do so multiple times, until it either succeeds or
///* fails for some reason other than EINTR.
///*
///* If the file creation mode "m" is 0 then set it to the default for
///* SQLite.  The default is SQLITE_DEFAULT_FILE_PERMISSIONS (normally
///* 0644) as modified by the system umask.  If m is not 0, then
///* make the file creation mode be exactly m ignoring the umask.
///*
///* The m parameter will be non-zero only when creating -wal, -journal,
///* and -shm files.  We want those files to have *exactly* the same
///* permissions as their original database, unadulterated by the umask.
///* In that way, if a database file is -rw-rw-rw or -rw-rw-r-, and a
///* transaction crashes and leaves behind hot journals, then any
///* process that is able to write to the database will also be able to
///* recover the hot journals.
extern "C" fn robust_open(z: *const i8, f: i32, m: ModeT) -> i32 {
    unsafe {
        let mut fd: i32 = 0;
        let m2: ModeT = if m != 0 { m as i32 } else { 420 } as ModeT;
        loop {
            fd =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8, i32, i32)
                                        ->
                                            i32>(a_syscall[0 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(z, f | 16777216, m2 as i32)
                };
            if fd < 0 {
                if unsafe { *unsafe { __error() } } == 4 { continue; }
                break;
            }
            if fd >= 3 { break; }
            if f & (2048 | 512) == 2048 | 512 {
                {
                    let _ =
                        unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*const i8)
                                                ->
                                                    i32>(a_syscall[16 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(z)
                        };
                };
            }
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(i32)
                                    ->
                                        i32>(a_syscall[1 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })(fd)
            };
            unsafe {
                sqlite3_log(28,
                    c"attempt to open \"%s\" as file descriptor %d".as_ptr() as
                            *mut i8 as *const i8, z, fd)
            };
            fd = -1;
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*const i8, i32, i32)
                                            ->
                                                i32>(a_syscall[0 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(c"/dev/null".as_ptr() as *mut i8 as *const i8, 0,
                            m as i32)
                    } < 0 {
                break;
            }
        }
        if fd >= 0 {
            if m as i32 != 0 {
                let mut statbuf: Stat = unsafe { core::mem::zeroed() };
                if unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(i32, *mut Stat)
                                                        ->
                                                            i32>(a_syscall[5 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(fd, &mut statbuf)
                                } == 0 && statbuf.st_size == 0 as i64 &&
                        statbuf.st_mode as i32 & 511 != m as i32 {
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, u16)
                                            ->
                                                i32>(a_syscall[14 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(fd, m)
                    };
                }
            }
        }
        return fd;
    }
}

#[allow(unused_doc_comments)]
extern "C" fn unix_log_error_at_line(errcode: i32, z_func_1: *const i8,
    mut z_path_1: *const i8, i_line_1: i32) -> i32 {
    let mut z_err: *mut i8 = core::ptr::null_mut();
    /// Message from strerror() or equivalent
    let i_errno: i32 = unsafe { *unsafe { __error() } };

    /// Saved syscall error number
    /// If this is not a threadsafe build (SQLITE_THREADSAFE==0), then use
    ///* the strerror() function to obtain the human-readable error message
    ///* equivalent to errno. Otherwise, use strerror_r().
    /// This is a threadsafe build, but strerror_r() is not available.
    (z_err = c"".as_ptr() as *mut i8);
    if z_path_1 == core::ptr::null() {
        z_path_1 = c"".as_ptr() as *mut i8 as *const i8;
    }
    unsafe {
        sqlite3_log(errcode,
            c"os_unix.c:%d: (%d) %s(%s) - %s".as_ptr() as *mut i8 as
                *const i8, i_line_1, i_errno, z_func_1, z_path_1, z_err)
    };
    return errcode;
}

/// Forward reference
extern "C" fn open_directory(z_filename: *const i8, p_fd: *mut i32) -> i32 {
    let mut ii: i32 = 0;
    let mut fd: i32 = -1;
    let mut z_dirname: [i8; 513] = [0; 513];
    unsafe {
        sqlite3_snprintf(512, &raw mut z_dirname[0 as usize] as *mut i8,
            c"%s".as_ptr() as *mut i8 as *const i8, z_filename)
    };
    {
        ii =
            unsafe {
                    strlen(&raw mut z_dirname[0 as usize] as *mut i8 as
                            *const i8)
                } as i32;
        '__b1: loop {
            if !(ii > 0 && z_dirname[ii as usize] as i32 != '/' as i32) {
                break '__b1;
            }
            '__c1: loop { break '__c1; }
            { let __p = &mut ii; let __t = *__p; *__p -= 1; __t };
        }
    }
    if ii > 0 {
        z_dirname[ii as usize] = '\u{0}' as i32 as i8;
    } else {
        if z_dirname[0 as usize] as i32 != '/' as i32 {
            z_dirname[0 as usize] = '.' as i32 as i8;
        }
        z_dirname[1 as usize] = 0 as i8;
    }
    fd =
        robust_open(&raw mut z_dirname[0 as usize] as *mut i8 as *const i8,
            0 | 0, 0 as ModeT);
    if fd >= 0 {}
    unsafe { *p_fd = fd };
    if fd >= 0 { return 0; }
    return unix_log_error_at_line(unsafe { sqlite3_cantopen_error(3893) },
            c"openDirectory".as_ptr() as *mut i8 as *const i8,
            &raw mut z_dirname[0 as usize] as *mut i8 as *const i8, 3893);
}

///* Return the system page size.
///*
///* This function should not be called directly by other code in this file.
///* Instead, it should be called via macro osGetpagesize().
extern "C" fn unix_getpagesize() -> i32 {
    return unsafe { sysconf(29) } as i32;
}

static mut a_syscall: [UnixSyscall; 29] =
    [UnixSyscall {
                z_name: c"open".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(posix_open as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"close".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(close as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"access".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(access as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"getcwd".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(getcwd as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"stat".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(stat as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"fstat".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(fstat as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"ftruncate".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(ftruncate as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"fcntl".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(fcntl as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"read".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(read as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"pread".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(pread as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"pread64".as_ptr() as *const i8,
                p_current: None,
                p_default: None,
            },
            UnixSyscall {
                z_name: c"write".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(write as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"pwrite".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(pwrite as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"pwrite64".as_ptr() as *const i8,
                p_current: None,
                p_default: None,
            },
            UnixSyscall {
                z_name: c"fchmod".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(fchmod as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"fallocate".as_ptr() as *const i8,
                p_current: None,
                p_default: None,
            },
            UnixSyscall {
                z_name: c"unlink".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(unlink as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"openDirectory".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(open_directory as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"mkdir".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(mkdir as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"rmdir".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(rmdir as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"fchown".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(fchown as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"geteuid".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(geteuid as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"mmap".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(mmap as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"munmap".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(munmap as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"mremap".as_ptr() as *const i8,
                p_current: None,
                p_default: None,
            },
            UnixSyscall {
                z_name: c"getpagesize".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(unix_getpagesize as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"readlink".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(readlink as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"lstat".as_ptr() as *const i8,
                p_current: Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn() -> ()>(lstat as *const ())
                    }),
                p_default: None,
            },
            UnixSyscall {
                z_name: c"ioctl".as_ptr() as *const i8,
                p_current: None,
                p_default: None,
            }];

///* Return TRUE if pFile has been renamed or unlinked since it was first opened.
extern "C" fn file_has_moved(p_file_1: &UnixFile) -> i32 {
    unsafe {
        let mut buf: Stat = unsafe { core::mem::zeroed() };
        return ((*p_file_1).p_inode != core::ptr::null_mut() &&
                    (unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*const i8, *mut Stat)
                                                    ->
                                                        i32>(a_syscall[4 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })((*p_file_1).z_path, &mut buf)
                            } != 0 ||
                        buf.st_ino as u64 !=
                            unsafe { (*(*p_file_1).p_inode).file_id.ino })) as i32;
    }
}

///* Check a unixFile that is a database.  Verify the following:
///*
///* (1) There is exactly one hard link on the file
///* (2) The file is not a symbolic link
///* (3) The file has not been renamed or unlinked
///*
///* Issue sqlite3_log(SQLITE_WARNING,...) messages if anything is not right.
extern "C" fn verify_db_file(p_file_1: *mut UnixFile) -> () {
    unsafe {
        let mut buf: Stat = unsafe { core::mem::zeroed() };
        let mut rc: i32 = 0;
        if unsafe { (*p_file_1).ctrl_flags } as i32 & 128 != 0 { return; }
        rc =
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(i32, *mut Stat)
                                    ->
                                        i32>(a_syscall[5 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })(unsafe { (*p_file_1).h }, &mut buf)
            };
        if rc != 0 {
            unsafe {
                sqlite3_log(28,
                    c"cannot fstat db file %s".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_file_1).z_path })
            };
            return;
        }
        if buf.st_nlink as i32 == 0 {
            unsafe {
                sqlite3_log(28,
                    c"file unlinked while open: %s".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_file_1).z_path })
            };
            return;
        }
        if buf.st_nlink as i32 > 1 {
            unsafe {
                sqlite3_log(28,
                    c"multiple links to file: %s".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_file_1).z_path })
            };
            return;
        }
        if file_has_moved(unsafe { &*p_file_1 }) != 0 {
            unsafe {
                sqlite3_log(28,
                    c"file renamed while open: %s".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_file_1).z_path })
            };
            return;
        }
    }
}

///* Attempt to set a system-lock on the file pFile.  The lock is
///* described by pLock.
///*
///* If the pFile was opened read/write from unix-excl, then the only lock
///* ever obtained is an exclusive lock, and it is obtained exactly once
///* the first time any lock is attempted.  All subsequent system locking
///* operations become no-ops.  Locking operations still happen internally,
///* in order to coordinate access between separate database connections
///* within this process, but all of that is handled in memory and the
///* operating system does not participate.
///*
///* This function is a pass-through to fcntl(F_SETLK) if pFile is using
///* any VFS other than "unix-excl" or if pFile is opened on "unix-excl"
///* and is read-only.
///*
///* Zero is returned if the call completes successfully, or -1 if a call
///* to fcntl() fails. In this case, errno is set appropriately (by fcntl()).
#[allow(unused_doc_comments)]
extern "C" fn unix_file_lock(p_file_1: &UnixFile, p_lock_1: *mut Flock)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_inode: *mut UnixInodeInfo = (*p_file_1).p_inode;
        { let _ = 0; };
        { let _ = 0; };
        if (*p_file_1).ctrl_flags as i32 & (1 | 2) == 1 {
            if unsafe { (*p_inode).b_process_lock } as i32 == 0 {
                let mut lock: Flock = unsafe { core::mem::zeroed() };

                /// assert( pInode->nLock==0 ); <-- Not true if unix-excl READONLY used
                (lock.l_whence = 0 as i16);
                lock.l_start = (sqlite3_pending_byte + 2) as OffT;
                lock.l_len = 510 as OffT;
                lock.l_type = 3 as i16;
                rc =
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, i32, ...)
                                            ->
                                                i32>(a_syscall[7 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })((*p_file_1).h, 8, &mut lock)
                    };
                if rc < 0 { return rc; }
                unsafe { (*p_inode).b_process_lock = 1 as u8 };
                {
                    let __p = unsafe { &mut (*p_inode).n_lock };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            } else { rc = 0; }
        } else {
            rc =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, i32, ...)
                                        ->
                                            i32>(a_syscall[7 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })((*p_file_1).h, 8, p_lock_1)
                };
        }
        return rc;
    }
}

///* Set the pFile->lastErrno.  Do this in a subroutine as that provides
///* a convenient place to set a breakpoint.
extern "C" fn store_last_errno(p_file_1: &mut UnixFile, error: i32) -> () {
    (*p_file_1).last_errno = error;
}

///* This routine translates a standard POSIX errno code into something
///* useful to the clients of the sqlite3 functions.  Specifically, it is
///* intended to translate a variety of "try again" errors into SQLITE_BUSY
///* and a variety of "please close the file descriptor NOW" errors into
///* SQLITE_IOERR
///*
///* Errors during initialization of locks, or file system support for locks,
///* should handle ENOLCK, ENOTSUP, EOPNOTSUPP separately.
extern "C" fn sqlite_error_from_posix_error(posix_error_1: i32,
    sqlite_io_err_1: i32) -> i32 {
    { let _ = 0; };
    '__s2:
        {
        match posix_error_1 {
            13 => { return 5; return 3; }
            35 => { return 5; return 3; }
            60 => { return 5; return 3; }
            16 => { return 5; return 3; }
            4 => { return 5; return 3; }
            77 => { return 5; return 3; }
            1 => { return 3; }
            _ => { return sqlite_io_err_1; }
        }
    }
}

///* Close a file descriptor.
///*
///* We assume that close() almost always works, since it is only in a
///* very sick application or on a very sick platform that it might fail.
///* If it does fail, simply leak the file descriptor, but do log the
///* error.
///*
///* Note that it is not safe to retry close() after EINTR since the
///* file descriptor might have already been reused by another thread.
///* So we don't even try to recover from an EINTR.  Just log the error
///* and move on.
extern "C" fn robust_close(p_file_1: *const UnixFile, h: i32, lineno: i32)
    -> () {
    unsafe {
        if unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32)
                                        ->
                                            i32>(a_syscall[1 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(h)
                } != 0 {
            unix_log_error_at_line(10 | 16 << 8,
                c"close".as_ptr() as *mut i8 as *const i8,
                if !(p_file_1).is_null() {
                    unsafe { (*p_file_1).z_path }
                } else { core::ptr::null() }, lineno);
        }
    }
}

///* Close all file descriptors accumulated in the unixInodeInfo->pUnused list.
extern "C" fn close_pending_fds(p_file_1: *mut UnixFile) -> () {
    let p_inode: *mut UnixInodeInfo = unsafe { (*p_file_1).p_inode };
    let mut p: *mut UnixUnusedFd = core::ptr::null_mut();
    let mut p_next: *mut UnixUnusedFd = core::ptr::null_mut();
    { let _ = 0; };
    {
        p = unsafe { (*p_inode).p_unused };
        '__b3: loop {
            if !(!(p).is_null()) { break '__b3; }
            '__c3: loop {
                p_next = unsafe { (*p).p_next };
                robust_close(p_file_1 as *const UnixFile, unsafe { (*p).fd },
                    1478);
                unsafe { sqlite3_free(p as *mut ()) };
                break '__c3;
            }
            p = p_next;
        }
    }
    unsafe { (*p_inode).p_unused = core::ptr::null_mut() };
}

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
///*
///* If handleNFSUnlock is true, then on downgrading an EXCLUSIVE_LOCK to SHARED
///* the byte range is divided into 2 parts and the first part is unlocked then
///* set to a read lock, then the other part is simply unlocked.  This works
///* around a bug in BSD NFS lockd (also seen on MacOSX 10.3+) that fails to
///* remove the write lock on a region when a read lock is set.
#[allow(unused_doc_comments)]
extern "C" fn posix_unlock(id: *mut Sqlite3File, e_file_lock_1: i32,
    handle_nfs_unlock_1: i32) -> i32 {
    unsafe {
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let mut p_inode: *mut UnixInodeInfo = core::ptr::null_mut();
        let mut rc: i32 = 0;
        '__b4: loop {
            '__c4: loop {
                let mut lock: Flock = unsafe { core::mem::zeroed() };
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_file).e_file_lock } as i32 <= e_file_lock_1 {
                    return 0;
                }
                p_inode = unsafe { (*p_file).p_inode };
                unsafe {
                    sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex })
                };
                { let _ = 0; };
                if unsafe { (*p_file).e_file_lock } as i32 > 1 {
                    { let _ = 0; };
                    if e_file_lock_1 == 1 {
                        if handle_nfs_unlock_1 != 0 {
                            let mut t_errno: i32 = 0;
                            /// Error code from system call errors
                            let div_size: OffT = (510 - 1) as OffT;
                            lock.l_type = 2 as i16;
                            lock.l_whence = 0 as i16;
                            lock.l_start = (sqlite3_pending_byte + 2) as OffT;
                            lock.l_len = div_size;
                            if unix_file_lock(unsafe { &*p_file }, &mut lock) == -1 {
                                t_errno = unsafe { *unsafe { __error() } };
                                rc = 10 | 8 << 8;
                                store_last_errno(unsafe { &mut *p_file }, t_errno);
                                break '__b4;
                            }
                            lock.l_type = 1 as i16;
                            lock.l_whence = 0 as i16;
                            lock.l_start = (sqlite3_pending_byte + 2) as OffT;
                            lock.l_len = div_size;
                            if unix_file_lock(unsafe { &*p_file }, &mut lock) == -1 {
                                t_errno = unsafe { *unsafe { __error() } };
                                rc = sqlite_error_from_posix_error(t_errno, 10 | 9 << 8);
                                if rc != 0 && rc != 5 {
                                    store_last_errno(unsafe { &mut *p_file }, t_errno);
                                }
                                break '__b4;
                            }
                            lock.l_type = 2 as i16;
                            lock.l_whence = 0 as i16;
                            lock.l_start =
                                (sqlite3_pending_byte + 2) as OffT + div_size;
                            lock.l_len = 510 as OffT - div_size;
                            if unix_file_lock(unsafe { &*p_file }, &mut lock) == -1 {
                                t_errno = unsafe { *unsafe { __error() } };
                                rc = 10 | 8 << 8;
                                store_last_errno(unsafe { &mut *p_file }, t_errno);
                                break '__b4;
                            }
                        } else {
                            lock.l_type = 1 as i16;
                            lock.l_whence = 0 as i16;
                            lock.l_start = (sqlite3_pending_byte + 2) as OffT;
                            lock.l_len = 510 as OffT;
                            if unix_file_lock(unsafe { &*p_file }, &mut lock) != 0 {

                                /// In theory, the call to unixFileLock() cannot fail because another
                                ///* process is holding an incompatible lock. If it does, this
                                ///* indicates that the other process is not following the locking
                                ///* protocol. If this happens, return SQLITE_IOERR_RDLOCK. Returning
                                ///* SQLITE_BUSY would confuse the upper layer (in practice it causes
                                ///* an assert to fail).
                                (rc = 10 | 9 << 8);
                                store_last_errno(unsafe { &mut *p_file },
                                    unsafe { *unsafe { __error() } });
                                break '__b4;
                            }
                        }
                    }
                    lock.l_type = 2 as i16;
                    lock.l_whence = 0 as i16;
                    lock.l_start = sqlite3_pending_byte as OffT;
                    lock.l_len = 2 as OffT;
                    { let _ = 0; };
                    if unix_file_lock(unsafe { &*p_file }, &mut lock) == 0 {
                        unsafe { (*p_inode).e_file_lock = 1 as u8 };
                    } else {
                        rc = 10 | 8 << 8;
                        store_last_errno(unsafe { &mut *p_file },
                            unsafe { *unsafe { __error() } });
                        break '__b4;
                    }
                }
                if e_file_lock_1 == 0 {

                    /// Decrement the shared lock counter.  Release the lock using an
                    ///* OS call only when all threads in this same process have released
                    ///* the lock.
                    {
                        let __p = unsafe { &mut (*p_inode).n_shared };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    if unsafe { (*p_inode).n_shared } == 0 {
                        lock.l_type = 2 as i16;
                        lock.l_whence = 0 as i16;
                        lock.l_start = { lock.l_len = 0 as OffT; lock.l_len };
                        if unix_file_lock(unsafe { &*p_file }, &mut lock) == 0 {
                            unsafe { (*p_inode).e_file_lock = 0 as u8 };
                        } else {
                            rc = 10 | 8 << 8;
                            store_last_errno(unsafe { &mut *p_file },
                                unsafe { *unsafe { __error() } });
                            unsafe { (*p_inode).e_file_lock = 0 as u8 };
                            unsafe { (*p_file).e_file_lock = 0 as u8 };
                        }
                    }

                    /// Decrement the count of locks against this same file.  When the
                    ///* count reaches zero, close any other file descriptors whose close
                    ///* was deferred because of outstanding locks.
                    {
                        let __p = unsafe { &mut (*p_inode).n_lock };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    { let _ = 0; };
                    if unsafe { (*p_inode).n_lock } == 0 {
                        close_pending_fds(p_file);
                    }
                }
                break '__c4;
            }
            if !(false) { break '__b4; }
        }

        /// downgrading to a shared lock on NFS involves clearing the write lock
        ///* before establishing the readlock - to avoid a race condition we downgrade
        ///* the lock in 2 blocks, so that part of the range will be covered by a
        ///* write lock until the rest is covered by a read lock:
        ///*  1:   [WWWWW]
        ///*  2:   [....W]
        ///*  3:   [RRRRW]
        ///*  4:   [RRRR.]
        /// Error code from system call errors
        /// defined(__APPLE__) && SQLITE_ENABLE_LOCKING_STYLE
        /// In theory, the call to unixFileLock() cannot fail because another
        ///* process is holding an incompatible lock. If it does, this
        ///* indicates that the other process is not following the locking
        ///* protocol. If this happens, return SQLITE_IOERR_RDLOCK. Returning
        ///* SQLITE_BUSY would confuse the upper layer (in practice it causes
        ///* an assert to fail).
        /// Decrement the shared lock counter.  Release the lock using an
        ///* OS call only when all threads in this same process have released
        ///* the lock.
        /// Decrement the count of locks against this same file.  When the
        ///* count reaches zero, close any other file descriptors whose close
        ///* was deferred because of outstanding locks.
        unsafe { sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex }) };
        if rc == 0 { unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 }; }
        return rc;
    }
}

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
extern "C" fn unix_unlock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    { let _ = 0; };
    return posix_unlock(id, e_file_lock_1, 0);
}

///* Helper functions to obtain and relinquish the global mutex. The
///* global mutex is used to protect the unixInodeInfo objects used by
///* this file, all of which may be shared by multiple threads.
///*
///* Function unixMutexHeld() is used to assert() that the global mutex
///* is held when required. This function is only used as part of assert()
///* statements. e.g.
///*
///*   unixEnterMutex()
///*     assert( unixMutexHeld() );
///*   unixEnterLeave()
///*
///* To prevent deadlock, the global unixBigLock must must be acquired
///* before the unixInodeInfo.pLockMutex mutex, if both are held.  It is
///* OK to get the pLockMutex without holding unixBigLock first, but if
///* that happens, the unixBigLock mutex must not be acquired until after
///* pLockMutex is released.
///*
///*      OK:     enter(unixBigLock),  enter(pLockInfo)
///*      OK:     enter(unixBigLock)
///*      OK:     enter(pLockInfo)
///*   ERROR:     enter(pLockInfo), enter(unixBigLock)
static mut unix_big_lock: *mut Sqlite3Mutex = core::ptr::null_mut();

#[allow(unused_doc_comments)]
extern "C" fn unix_enter_mutex() -> () {
    unsafe {
        { let _ = 0; };

        /// Not a recursive mutex
        unsafe { sqlite3_mutex_enter(unix_big_lock) };
    }
}

///* Add the file descriptor used by file handle pFile to the corresponding
///* pUnused list.
extern "C" fn set_pending_fd(p_file_1: &mut UnixFile) -> () {
    let p_inode: *mut UnixInodeInfo = (*p_file_1).p_inode;
    let p: *mut UnixUnusedFd = (*p_file_1).p_preallocated_unused;
    { let _ = 0; };
    unsafe { (*p).p_next = unsafe { (*p_inode).p_unused } };
    unsafe { (*p_inode).p_unused = p };
    (*p_file_1).h = -1;
    (*p_file_1).p_preallocated_unused = core::ptr::null_mut();
}

/// All unixInodeInfo objects
static mut inode_list: *mut UnixInodeInfo = core::ptr::null_mut();

///* Release a unixInodeInfo structure previously allocated by findInodeInfo().
///*
///* The global mutex must be held when this routine is called, but the mutex
///* on the inode being deleted must NOT be held.
extern "C" fn release_inode_info(p_file_1: *mut UnixFile) -> () {
    unsafe {
        let p_inode: *mut UnixInodeInfo = unsafe { (*p_file_1).p_inode };
        { let _ = 0; };
        { let _ = 0; };
        if !(p_inode).is_null() {
            {
                let __p = unsafe { &mut (*p_inode).n_ref };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if unsafe { (*p_inode).n_ref } == 0 {
                { let _ = 0; };
                unsafe {
                    sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex })
                };
                close_pending_fds(p_file_1);
                unsafe {
                    sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex })
                };
                if !(unsafe { (*p_inode).p_prev }).is_null() {
                    { let _ = 0; };
                    unsafe {
                        (*unsafe { (*p_inode).p_prev }).p_next =
                            unsafe { (*p_inode).p_next }
                    };
                } else {
                    { let _ = 0; };
                    inode_list = unsafe { (*p_inode).p_next };
                }
                if !(unsafe { (*p_inode).p_next }).is_null() {
                    { let _ = 0; };
                    unsafe {
                        (*unsafe { (*p_inode).p_next }).p_prev =
                            unsafe { (*p_inode).p_prev }
                    };
                }
                unsafe {
                    sqlite3_mutex_free(unsafe { (*p_inode).p_lock_mutex })
                };
                unsafe { sqlite3_free(p_inode as *mut ()) };
            }
        }
    }
}

///* If it is currently memory mapped, unmap file pFd.
extern "C" fn unix_unmapfile(p_fd_1: &mut UnixFile) -> () {
    unsafe {
        { let _ = 0; };
        if !((*p_fd_1).p_map_region).is_null() {
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut (), u64)
                                    ->
                                        i32>(a_syscall[23 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })((*p_fd_1).p_map_region,
                    (*p_fd_1).mmap_size_actual as u64)
            };
            (*p_fd_1).p_map_region = core::ptr::null_mut();
            (*p_fd_1).mmap_size = 0 as Sqlite3Int64;
            (*p_fd_1).mmap_size_actual = 0 as Sqlite3Int64;
        }
    }
}

///* This function performs the parts of the "close file" operation
///* common to all locking schemes. It closes the directory and file
///* handles, if they are valid, and sets all fields of the unixFile
///* structure to 0.
///*
///* It is *not* necessary to hold the mutex when this routine is called,
///* even on VxWorks.  A mutex will be acquired on VxWorks by the
///* vxworksReleaseFileId() routine.
extern "C" fn close_unix_file(id: *mut Sqlite3File) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
    if unsafe { (*p_file).h } >= 0 {
        robust_close(p_file as *const UnixFile, unsafe { (*p_file).h }, 2312);
        unsafe { (*p_file).h = -1 };
    }
    unsafe {
        sqlite3_free(unsafe { (*p_file).p_preallocated_unused } as *mut ())
    };
    unsafe {
        memset(p_file as *mut (), 0, core::mem::size_of::<UnixFile>() as u64)
    };
    return 0;
}

extern "C" fn unix_leave_mutex() -> () {
    unsafe { { let _ = 0; }; unsafe { sqlite3_mutex_leave(unix_big_lock) }; }
}

///* Close a file.
#[allow(unused_doc_comments)]
extern "C" fn unix_close(id: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let p_inode: *const UnixInodeInfo =
        unsafe { (*p_file).p_inode } as *const UnixInodeInfo;
    { let _ = 0; };
    verify_db_file(p_file);
    unix_unlock(id, 0);
    { let _ = 0; };
    unix_enter_mutex();

    /// unixFile.pInode is always valid here. Otherwise, a different close
    ///* routine (e.g. nolockClose()) would be called instead.
    { let _ = 0; };
    unsafe { sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex }) };
    if unsafe { (*p_inode).n_lock } != 0 {

        /// If there are outstanding locks, do not actually close the file just
        ///* yet because that would clear those locks.  Instead, add the file
        ///* descriptor to pInode->pUnused list.  It will be automatically closed
        ///* when the last lock is cleared.
        set_pending_fd(unsafe { &mut *p_file });
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex }) };
    release_inode_info(p_file);
    { let _ = 0; };
    rc = close_unix_file(id);
    unix_leave_mutex();
    return rc;
}

///* Seek to the offset passed as the second argument, then read cnt
///* bytes into pBuf. Return the number of bytes actually read.
///*
///* To avoid stomping the errno value on a failed read the lastErrno value
///* is set before returning.
extern "C" fn seek_and_read(id: *mut UnixFile, mut offset: Sqlite3Int64,
    mut p_buf_1: *mut (), mut cnt: i32) -> i32 {
    unsafe {
        let mut got: i32 = 0;
        let mut prior: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        '__b5: loop {
            '__c5: loop {
                got =
                    unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(i32, *mut (), u64, i64)
                                                ->
                                                    i64>(a_syscall[9 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(unsafe { (*id).h }, p_buf_1, cnt as u64, offset)
                        } as i32;
                if got == cnt { break '__b5; }
                if got < 0 {
                    if unsafe { *unsafe { __error() } } == 4 {
                        got = 1;
                        break '__c5;
                    }
                    prior = 0;
                    store_last_errno(unsafe { &mut *(id as *mut UnixFile) },
                        unsafe { *unsafe { __error() } });
                    break '__b5;
                } else if got > 0 {
                    cnt -= got;
                    offset += got as Sqlite3Int64;
                    prior += got;
                    p_buf_1 =
                        unsafe { (p_buf_1 as *mut i8).offset(got as isize) } as
                            *mut ();
                }
                break '__c5;
            }
            if !(got > 0) { break '__b5; }
        }
        return got + prior;
    }
}

///* Read data from a file into a buffer.  Return SQLITE_OK if all
///* bytes were read successfully and SQLITE_IOERR if anything goes
///* wrong.
#[allow(unused_doc_comments)]
extern "C" fn unix_read(id: *mut Sqlite3File, mut p_buf_1: *mut (),
    mut amt: i32, mut offset: Sqlite3Int64) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let mut got: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if offset < unsafe { (*p_file).mmap_size } {
        if offset + amt as Sqlite3Int64 <= unsafe { (*p_file).mmap_size } {
            unsafe {
                memcpy(p_buf_1,
                    unsafe {
                            &raw mut *(unsafe { (*p_file).p_map_region } as
                                            *mut u8).offset(offset as isize)
                        } as *const (), amt as u64)
            };
            return 0;
        } else {
            let n_copy: i32 =
                (unsafe { (*p_file).mmap_size } - offset) as i32;
            unsafe {
                memcpy(p_buf_1,
                    unsafe {
                            &raw mut *(unsafe { (*p_file).p_map_region } as
                                            *mut u8).offset(offset as isize)
                        } as *const (), n_copy as u64)
            };
            p_buf_1 =
                unsafe { (p_buf_1 as *mut u8).offset(n_copy as isize) } as
                    *mut ();
            amt -= n_copy;
            offset += n_copy as Sqlite3Int64;
        }
    }
    got = seek_and_read(p_file, offset, p_buf_1, amt);
    if got == amt {
        return 0;
    } else if got < 0 {
        '__s6:
            {
            match unsafe { (*p_file).last_errno } {
                34 => { return 10 | 33 << 8; }
                5 => { return 10 | 33 << 8; }
                6 => { return 10 | 33 << 8; }
                83 => { return 10 | 33 << 8; }
                _ => {}
            }
        }
        return 10 | 1 << 8;
    } else {
        store_last_errno(unsafe { &mut *p_file }, 0);

        /// not a system error */
        ///    /* Unread parts of the buffer must be zero-filled
        unsafe {
            memset(unsafe {
                        &raw mut *(p_buf_1 as *mut i8).offset(got as isize)
                    } as *mut (), 0, (amt - got) as u64)
        };
        return 10 | 2 << 8;
    }
}

///* Attempt to seek the file-descriptor passed as the first argument to
///* absolute offset iOff, then attempt to write nBuf bytes of data from
///* pBuf to it. If an error occurs, return -1 and set *piErrno. Otherwise,
///* return the actual number of bytes written (which may be less than
///* nBuf).
#[allow(unused_doc_comments)]
extern "C" fn seek_and_write_fd(fd: i32, i_off_1: i64, p_buf_1: *const (),
    mut n_buf_1: i32, pi_errno_1: &mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;

        /// Value returned by system call
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        n_buf_1 &= 131071;
        '__b7: loop {
            '__c7: loop {
                rc =
                    unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(i32, *const (), u64, i64)
                                                ->
                                                    i64>(a_syscall[12 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(fd, p_buf_1, n_buf_1 as u64, i_off_1)
                        } as i32;
                break '__c7;
            }
            if !(rc < 0 && unsafe { *unsafe { __error() } } == 4) {
                break '__b7;
            }
        }
        if rc < 0 { *pi_errno_1 = unsafe { *unsafe { __error() } }; }
        return rc;
    }
}

///* Seek to the offset in id->offset then read cnt bytes into pBuf.
///* Return the number of bytes actually read.  Update the offset.
///*
///* To avoid stomping the errno value on a failed write the lastErrno value
///* is set before returning.
extern "C" fn seek_and_write(id: &mut UnixFile, offset: i64,
    p_buf_1: *const (), cnt: i32) -> i32 {
    return seek_and_write_fd((*id).h, offset, p_buf_1, cnt,
            &mut (*id).last_errno);
}

///* Write data from a buffer into a file.  Return SQLITE_OK on success
///* or some other error code on failure.
#[allow(unused_doc_comments)]
extern "C" fn unix_write(id: *mut Sqlite3File, mut p_buf_1: *const (),
    mut amt: i32, mut offset: Sqlite3Int64) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let mut wrote: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    while {
                    wrote =
                        seek_and_write(unsafe { &mut *p_file }, offset, p_buf_1,
                            amt);
                    wrote
                } < amt && wrote > 0 {
        amt -= wrote;
        offset += wrote as Sqlite3Int64;
        p_buf_1 =
            unsafe { (p_buf_1 as *mut i8).offset(wrote as isize) } as
                *const ();
    }
    if amt > wrote {
        if wrote < 0 && unsafe { (*p_file).last_errno } != 28 {

            /// lastErrno set by seekAndWrite
            return 10 | 3 << 8;
        } else {
            store_last_errno(unsafe { &mut *p_file }, 0);

            /// not a system error
            return 13;
        }
    }
    return 0;
}

///* Retry ftruncate() calls that fail due to EINTR
///*
///* All calls to ftruncate() within this file should be made through
///* this wrapper.  On the Android platform, bypassing the logic below
///* could lead to a corrupt database.
extern "C" fn robust_ftruncate(h: i32, sz: Sqlite3Int64) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        '__b9: loop {
            '__c9: loop {
                rc =
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, i64)
                                            ->
                                                i32>(a_syscall[6 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(h, sz)
                    };
                break '__c9;
            }
            if !(rc < 0 && unsafe { *unsafe { __error() } } == 4) {
                break '__b9;
            }
        }
        return rc;
    }
}

///* Truncate an open file to a specified size
extern "C" fn unix_truncate(id: *mut Sqlite3File, mut n_byte_1: i64) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let mut rc: i32 = 0;
    { let _ = 0; };
    if unsafe { (*p_file).sz_chunk } > 0 {
        n_byte_1 =
            (n_byte_1 + unsafe { (*p_file).sz_chunk } as i64 - 1 as i64) /
                    unsafe { (*p_file).sz_chunk } as i64 *
                unsafe { (*p_file).sz_chunk } as i64;
    }
    rc = robust_ftruncate(unsafe { (*p_file).h }, n_byte_1);
    if rc != 0 {
        store_last_errno(unsafe { &mut *p_file },
            unsafe { *unsafe { __error() } });
        return unix_log_error_at_line(10 | 6 << 8,
                c"ftruncate".as_ptr() as *mut i8 as *const i8,
                unsafe { (*p_file).z_path }, 3979);
    } else {
        if n_byte_1 < unsafe { (*p_file).mmap_size } {
            unsafe { (*p_file).mmap_size = n_byte_1 };
        }
        return 0;
    }
}

///* The fsync() system call does not work as advertised on many
///* unix systems.  The following procedure is an attempt to make
///* it work better.
///*
///* The SQLITE_NO_SYNC macro disables all fsync()s.  This is useful
///* for testing when we want to run through the test suite quickly.
///* You are strongly advised *not* to deploy with SQLITE_NO_SYNC
///* enabled, however, since with SQLITE_NO_SYNC enabled, an OS crash
///* or power failure will likely corrupt the database file.
///*
///* SQLite sets the dataOnly flag if the size of the file is unchanged.
///* The idea behind dataOnly is that it should only write the file content
///* to disk, not the inode.  We only set dataOnly if the file size is
///* unchanged since the file size is part of the inode.  However,
///* Ted Ts'o tells us that fdatasync() will also write the inode if the
///* file size has changed.  The only real difference between fdatasync()
///* and fsync(), Ted tells us, is that fdatasync() will not flush the
///* inode if the mtime or owner or other inode attributes have changed.
///* We only care about the file size, not the other file attributes, so
///* as far as SQLite is concerned, an fdatasync() is always adequate.
///* So, we always use fdatasync() if it is available, regardless of
///* the value of the dataOnly flag.
#[allow(unused_doc_comments)]
extern "C" fn full_fsync(fd: i32, full_sync_1: i32, data_only_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;

        /// The following "ifdef/elif/else/" block has the same structure as
        ///* the one below. It is replicated here solely to avoid cluttering
        ///* up the real code with the UNUSED_PARAMETER() macros.
        { let _ = data_only_1; };
        if full_sync_1 != 0 {
            rc =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, i32, ...)
                                        ->
                                            i32>(a_syscall[7 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(fd, 51, 0)
                };
        } else { rc = 1; }
        if rc != 0 { rc = unsafe { fsync(fd) }; }
        if 0 != 0 && rc != -1 { rc = 0; }
        return rc;
    }
}

///* Make sure all writes to a particular file are committed to disk.
///*
///* If dataOnly==0 then both the file itself and its metadata (file
///* size, access time, etc) are synced.  If dataOnly!=0 then only the
///* file data is synced.
///*
///* Under Unix, also make sure that the directory entry for the file
///* has been created by fsync-ing the directory that contains the file.
///* If we do not do this and we encounter a power failure, the directory
///* entry for the journal might not exist after we reboot.  The next
///* SQLite to access the file will not know that the journal exists (because
///* the directory entry for the journal was never created) and the transaction
///* will not roll back - possibly leading to database corruption.
#[allow(unused_doc_comments)]
extern "C" fn unix_sync(id: *mut Sqlite3File, flags: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let is_data_only: i32 = flags & 16;
        let is_fullsync: i32 = (flags & 15 == 3) as i32;

        /// Check that one of SQLITE_SYNC_NORMAL or FULL was passed
        { let _ = 0; };
        { let _ = 0; };
        rc = full_fsync(unsafe { (*p_file).h }, is_fullsync, is_data_only);
        if rc != 0 {
            store_last_errno(unsafe { &mut *p_file },
                unsafe { *unsafe { __error() } });
            return unix_log_error_at_line(10 | 4 << 8,
                    c"full_fsync".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_file).z_path }, 3934);
        }
        if unsafe { (*p_file).ctrl_flags } as i32 & 8 != 0 {
            let mut dirfd: i32 = 0;
            rc =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8, *mut i32)
                                        ->
                                            i32>(a_syscall[17 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(unsafe { (*p_file).z_path }, &mut dirfd)
                };
            if rc == 0 {
                full_fsync(dirfd, 0, 0);
                robust_close(p_file as *const UnixFile, dirfd, 3948);
            } else { { let _ = 0; }; rc = 0; }
            unsafe { (*p_file).ctrl_flags &= !8 as u16 };
        }
        return rc;
    }
}

///* Determine the current size of a file in bytes
extern "C" fn unix_file_size(id: *mut Sqlite3File, p_size_1: *mut i64)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut buf: Stat = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        rc =
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(i32, *mut Stat)
                                    ->
                                        i32>(a_syscall[5 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })(unsafe { (*(id as *mut UnixFile)).h }, &mut buf)
            };
        if rc != 0 {
            store_last_errno(unsafe { &mut *(id as *mut UnixFile) },
                unsafe { *unsafe { __error() } });
            return 10 | 7 << 8;
        }
        unsafe { *p_size_1 = buf.st_size };
        if unsafe { *p_size_1 } == 1 as i64 {
            unsafe { *p_size_1 = 0 as i64 };
        }
        return 0;
    }
}

/// Forward reference
extern "C" fn unix_is_sharing_shm_node(p_file_1: &UnixFile) -> i32 {
    unsafe {
        let mut p_shm_node: *const UnixShmNode = core::ptr::null();
        let mut lock: Flock = unsafe { core::mem::zeroed() };
        if (*p_file_1).p_shm == core::ptr::null_mut() { return 0; }
        if (*p_file_1).ctrl_flags as i32 & 1 != 0 { return 0; }
        p_shm_node = unsafe { (*(*p_file_1).p_shm).p_shm_node };
        { let _ = 0; };
        unsafe {
            memset(&raw mut lock as *mut (), 0,
                core::mem::size_of::<Flock>() as u64)
        };
        lock.l_whence = 0 as i16;
        lock.l_start = ((22 + 8) * 4 + 8) as OffT;
        lock.l_len = 1 as OffT;
        lock.l_type = 3 as i16;
        unsafe {
            (unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(i32, i32, ...)
                                ->
                                    i32>(a_syscall[7 as
                                            usize].p_current.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                }) as *const ())
                })(unsafe { (*p_shm_node).h_shm }, 7, &mut lock)
        };
        return (lock.l_type as i32 != 2) as i32;
    }
}

///* Lock the file with the lock specified by parameter eFileLock - one
///* of the following:
///*
///*     (1) SHARED_LOCK
///*     (2) RESERVED_LOCK
///*     (3) PENDING_LOCK
///*     (4) EXCLUSIVE_LOCK
///*
///* Sometimes when requesting one lock state, additional lock states
///* are inserted in between.  The locking might fail on one of the later
///* transitions leaving the lock state different from what it started but
///* still short of its goal.  The following chart shows the allowed
///* transitions and the inserted intermediate states:
///*
///*    UNLOCKED -> SHARED
///*    SHARED -> RESERVED
///*    SHARED -> EXCLUSIVE
///*    RESERVED -> (PENDING) -> EXCLUSIVE
///*    PENDING -> EXCLUSIVE
///*
///* This routine will only increase a lock.  Use the sqlite3OsUnlock()
///* routine to lower a locking level.
#[allow(unused_doc_comments)]
extern "C" fn unix_lock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    unsafe {
        /// The following describes the implementation of the various locks and
        ///* lock transitions in terms of the POSIX advisory shared and exclusive
        ///* lock primitives (called read-locks and write-locks below, to avoid
        ///* confusion with SQLite lock names). The algorithms are complicated
        ///* slightly in order to be compatible with Windows95 systems simultaneously
        ///* accessing the same database file, in case that is ever required.
        ///*
        ///* Symbols defined in os.h identify the 'pending byte' and the 'reserved
        ///* byte', each single bytes at well known offsets, and the 'shared byte
        ///* range', a range of 510 bytes at a well known offset.
        ///*
        ///* To obtain a SHARED lock, a read-lock is obtained on the 'pending
        ///* byte'.  If this is successful, 'shared byte range' is read-locked
        ///* and the lock on the 'pending byte' released.  (Legacy note:  When
        ///* SQLite was first developed, Windows95 systems were still very common,
        ///* and Windows95 lacks a shared-lock capability.  So on Windows95, a
        ///* single randomly selected by from the 'shared byte range' is locked.
        ///* Windows95 is now pretty much extinct, but this work-around for the
        ///* lack of shared-locks on Windows95 lives on, for backwards
        ///* compatibility.)
        ///*
        ///* A process may only obtain a RESERVED lock after it has a SHARED lock.
        ///* A RESERVED lock is implemented by grabbing a write-lock on the
        ///* 'reserved byte'.
        ///*
        ///* An EXCLUSIVE lock may only be requested after either a SHARED or
        ///* RESERVED lock is held. An EXCLUSIVE lock is implemented by obtaining
        ///* a write-lock on the entire 'shared byte range'. Since all other locks
        ///* require a read-lock on one of the bytes within this range, this ensures
        ///* that no other locks are held on the database.
        ///*
        ///* If a process that holds a RESERVED lock requests an EXCLUSIVE, then
        ///* a PENDING lock is obtained first. A PENDING lock is implemented by
        ///* obtaining a write-lock on the 'pending byte'. This ensures that no new
        ///* SHARED locks can be obtained, but existing SHARED locks are allowed to
        ///* persist. If the call to this function fails to obtain the EXCLUSIVE
        ///* lock in this case, it holds the PENDING lock instead. The client may
        ///* then re-attempt the EXCLUSIVE lock later on, after existing SHARED
        ///* locks have cleared.
        let mut rc: i32 = 0;
        let mut p_inode: *mut UnixInodeInfo = core::ptr::null_mut();
        '__b10: loop {
            '__c10: loop {
                /// The following describes the implementation of the various locks and
                ///* lock transitions in terms of the POSIX advisory shared and exclusive
                ///* lock primitives (called read-locks and write-locks below, to avoid
                ///* confusion with SQLite lock names). The algorithms are complicated
                ///* slightly in order to be compatible with Windows95 systems simultaneously
                ///* accessing the same database file, in case that is ever required.
                ///*
                ///* Symbols defined in os.h identify the 'pending byte' and the 'reserved
                ///* byte', each single bytes at well known offsets, and the 'shared byte
                ///* range', a range of 510 bytes at a well known offset.
                ///*
                ///* To obtain a SHARED lock, a read-lock is obtained on the 'pending
                ///* byte'.  If this is successful, 'shared byte range' is read-locked
                ///* and the lock on the 'pending byte' released.  (Legacy note:  When
                ///* SQLite was first developed, Windows95 systems were still very common,
                ///* and Windows95 lacks a shared-lock capability.  So on Windows95, a
                ///* single randomly selected by from the 'shared byte range' is locked.
                ///* Windows95 is now pretty much extinct, but this work-around for the
                ///* lack of shared-locks on Windows95 lives on, for backwards
                ///* compatibility.)
                ///*
                ///* A process may only obtain a RESERVED lock after it has a SHARED lock.
                ///* A RESERVED lock is implemented by grabbing a write-lock on the
                ///* 'reserved byte'.
                ///*
                ///* An EXCLUSIVE lock may only be requested after either a SHARED or
                ///* RESERVED lock is held. An EXCLUSIVE lock is implemented by obtaining
                ///* a write-lock on the entire 'shared byte range'. Since all other locks
                ///* require a read-lock on one of the bytes within this range, this ensures
                ///* that no other locks are held on the database.
                ///*
                ///* If a process that holds a RESERVED lock requests an EXCLUSIVE, then
                ///* a PENDING lock is obtained first. A PENDING lock is implemented by
                ///* obtaining a write-lock on the 'pending byte'. This ensures that no new
                ///* SHARED locks can be obtained, but existing SHARED locks are allowed to
                ///* persist. If the call to this function fails to obtain the EXCLUSIVE
                ///* lock in this case, it holds the PENDING lock instead. The client may
                ///* then re-attempt the EXCLUSIVE lock later on, after existing SHARED
                ///* locks have cleared.
                let p_file: *mut UnixFile = id as *mut UnixFile;
                let mut lock: Flock = unsafe { core::mem::zeroed() };
                let mut t_errno: i32 = 0;
                { let _ = 0; };
                if unsafe { (*p_file).e_file_lock } as i32 >= e_file_lock_1 {
                    return 0;
                }

                /// Make sure the locking sequence is correct.
                ///*  (1) We never move from unlocked to anything higher than shared lock.
                ///*  (2) SQLite never explicitly requests a pending lock.
                ///*  (3) A shared lock is always held when a reserve lock is requested.
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };

                /// This mutex is needed because pFile->pInode is shared across threads
                (p_inode = unsafe { (*p_file).p_inode });
                unsafe {
                    sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex })
                };
                if unsafe { (*p_file).e_file_lock } as i32 !=
                            unsafe { (*p_inode).e_file_lock } as i32 &&
                        (unsafe { (*p_inode).e_file_lock } as i32 >= 3 ||
                            e_file_lock_1 > 1) {
                    rc = 5;
                    break '__b10;
                }
                if e_file_lock_1 == 1 &&
                        (unsafe { (*p_inode).e_file_lock } as i32 == 1 ||
                            unsafe { (*p_inode).e_file_lock } as i32 == 2) {
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe { (*p_file).e_file_lock = 1 as u8 };
                    {
                        let __p = unsafe { &mut (*p_inode).n_shared };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    {
                        let __p = unsafe { &mut (*p_inode).n_lock };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    break '__b10;
                }

                /// A PENDING lock is needed before acquiring a SHARED lock and before
                ///* acquiring an EXCLUSIVE lock.  For the SHARED lock, the PENDING will
                ///* be released.
                (lock.l_len = 1 as OffT);
                lock.l_whence = 0 as i16;
                if e_file_lock_1 == 1 ||
                        e_file_lock_1 == 4 &&
                            unsafe { (*p_file).e_file_lock } as i32 == 2 {
                    lock.l_type = if e_file_lock_1 == 1 { 1 } else { 3 } as i16;
                    lock.l_start = sqlite3_pending_byte as OffT;
                    if unix_file_lock(unsafe { &*p_file }, &mut lock) != 0 {
                        t_errno = unsafe { *unsafe { __error() } };
                        rc = sqlite_error_from_posix_error(t_errno, 10 | 15 << 8);
                        if rc != 5 {
                            store_last_errno(unsafe { &mut *p_file }, t_errno);
                        }
                        break '__b10;
                    } else if e_file_lock_1 == 4 {
                        unsafe { (*p_file).e_file_lock = 3 as u8 };
                        unsafe { (*p_inode).e_file_lock = 3 as u8 };
                    }
                }
                if e_file_lock_1 == 1 {
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };

                    /// Now get the read-lock
                    (lock.l_start = (sqlite3_pending_byte + 2) as OffT);
                    lock.l_len = 510 as OffT;
                    if unix_file_lock(unsafe { &*p_file }, &mut lock) != 0 {
                        t_errno = unsafe { *unsafe { __error() } };
                        rc = sqlite_error_from_posix_error(t_errno, 10 | 15 << 8);
                    }

                    /// Drop the temporary PENDING lock
                    (lock.l_start = sqlite3_pending_byte as OffT);
                    lock.l_len = 1 as OffT;
                    lock.l_type = 2 as i16;
                    if unix_file_lock(unsafe { &*p_file }, &mut lock) != 0 &&
                            rc == 0 {

                        /// This could happen with a network mount
                        (t_errno = unsafe { *unsafe { __error() } });
                        rc = 10 | 8 << 8;
                    }
                    if rc != 0 {
                        if rc != 5 {
                            store_last_errno(unsafe { &mut *p_file }, t_errno);
                        }
                        break '__b10;
                    } else {
                        unsafe { (*p_file).e_file_lock = 1 as u8 };
                        {
                            let __p = unsafe { &mut (*p_inode).n_lock };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe { (*p_inode).n_shared = 1 };
                    }
                } else if e_file_lock_1 == 4 &&
                        unsafe { (*p_inode).n_shared } > 1 {

                    /// We are trying for an exclusive lock but another thread in this
                    ///* same process is still holding a shared lock.
                    (rc = 5);
                } else if unsafe {
                            unix_is_sharing_shm_node(unsafe { &*p_file })
                        } != 0 {

                    /// We are in WAL mode and attempting to delete the SHM and WAL
                    ///* files due to closing the connection or changing out of WAL mode,
                    ///* but another process still holds locks on the SHM file, thus
                    ///* indicating that database locks have been broken, perhaps due
                    ///* to a rogue close(open(dbFile)) or similar.
                    (rc = 5);
                } else {

                    /// The request was for a RESERVED or EXCLUSIVE lock.  It is
                    ///* assumed that there is a SHARED or greater lock on the file
                    ///* already.
                    { let _ = 0; };
                    lock.l_type = 3 as i16;
                    { let _ = 0; };
                    if e_file_lock_1 == 2 {
                        lock.l_start = (sqlite3_pending_byte + 1) as OffT;
                        lock.l_len = 1 as OffT;
                    } else {
                        lock.l_start = (sqlite3_pending_byte + 2) as OffT;
                        lock.l_len = 510 as OffT;
                    }
                    if unix_file_lock(unsafe { &*p_file }, &mut lock) != 0 {
                        t_errno = unsafe { *unsafe { __error() } };
                        rc = sqlite_error_from_posix_error(t_errno, 10 | 15 << 8);
                        if rc != 5 {
                            store_last_errno(unsafe { &mut *p_file }, t_errno);
                        }
                    }
                }
                if rc == 0 {
                    unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };
                    unsafe { (*p_inode).e_file_lock = e_file_lock_1 as u8 };
                }
                break '__c10;
            }
            if !(false) { break '__b10; }
        }

        /// The following describes the implementation of the various locks and
        ///* lock transitions in terms of the POSIX advisory shared and exclusive
        ///* lock primitives (called read-locks and write-locks below, to avoid
        ///* confusion with SQLite lock names). The algorithms are complicated
        ///* slightly in order to be compatible with Windows95 systems simultaneously
        ///* accessing the same database file, in case that is ever required.
        ///*
        ///* Symbols defined in os.h identify the 'pending byte' and the 'reserved
        ///* byte', each single bytes at well known offsets, and the 'shared byte
        ///* range', a range of 510 bytes at a well known offset.
        ///*
        ///* To obtain a SHARED lock, a read-lock is obtained on the 'pending
        ///* byte'.  If this is successful, 'shared byte range' is read-locked
        ///* and the lock on the 'pending byte' released.  (Legacy note:  When
        ///* SQLite was first developed, Windows95 systems were still very common,
        ///* and Windows95 lacks a shared-lock capability.  So on Windows95, a
        ///* single randomly selected by from the 'shared byte range' is locked.
        ///* Windows95 is now pretty much extinct, but this work-around for the
        ///* lack of shared-locks on Windows95 lives on, for backwards
        ///* compatibility.)
        ///*
        ///* A process may only obtain a RESERVED lock after it has a SHARED lock.
        ///* A RESERVED lock is implemented by grabbing a write-lock on the
        ///* 'reserved byte'.
        ///*
        ///* An EXCLUSIVE lock may only be requested after either a SHARED or
        ///* RESERVED lock is held. An EXCLUSIVE lock is implemented by obtaining
        ///* a write-lock on the entire 'shared byte range'. Since all other locks
        ///* require a read-lock on one of the bytes within this range, this ensures
        ///* that no other locks are held on the database.
        ///*
        ///* If a process that holds a RESERVED lock requests an EXCLUSIVE, then
        ///* a PENDING lock is obtained first. A PENDING lock is implemented by
        ///* obtaining a write-lock on the 'pending byte'. This ensures that no new
        ///* SHARED locks can be obtained, but existing SHARED locks are allowed to
        ///* persist. If the call to this function fails to obtain the EXCLUSIVE
        ///* lock in this case, it holds the PENDING lock instead. The client may
        ///* then re-attempt the EXCLUSIVE lock later on, after existing SHARED
        ///* locks have cleared.
        /// If there is already a lock of this type or more restrictive on the
        ///* unixFile, do nothing. Don't use the end_lock: exit path, as
        ///* unixEnterMutex() hasn't been called yet.
        /// Make sure the locking sequence is correct.
        ///*  (1) We never move from unlocked to anything higher than shared lock.
        ///*  (2) SQLite never explicitly requests a pending lock.
        ///*  (3) A shared lock is always held when a reserve lock is requested.
        /// This mutex is needed because pFile->pInode is shared across threads
        /// If some thread using this PID has a lock via a different unixFile*
        ///* handle that precludes the requested lock, return BUSY.
        /// If a SHARED lock is requested, and some thread using this PID already
        ///* has a SHARED or RESERVED lock, then increment reference counts and
        ///* return SQLITE_OK.
        /// A PENDING lock is needed before acquiring a SHARED lock and before
        ///* acquiring an EXCLUSIVE lock.  For the SHARED lock, the PENDING will
        ///* be released.
        /// If control gets to this point, then actually go ahead and make
        ///* operating system calls for the specified lock.
        /// Now get the read-lock
        /// Drop the temporary PENDING lock
        /// This could happen with a network mount
        /// We are trying for an exclusive lock but another thread in this
        ///* same process is still holding a shared lock.
        /// We are in WAL mode and attempting to delete the SHM and WAL
        ///* files due to closing the connection or changing out of WAL mode,
        ///* but another process still holds locks on the SHM file, thus
        ///* indicating that database locks have been broken, perhaps due
        ///* to a rogue close(open(dbFile)) or similar.
        /// The request was for a RESERVED or EXCLUSIVE lock.  It is
        ///* assumed that there is a SHARED or greater lock on the file
        ///* already.
        unsafe { sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex }) };
        return rc;
    }
}

///* This routine checks if there is a RESERVED lock held on the specified
///* file by this or any other process. If such a lock is held, set *pResOut
///* to a non-zero value otherwise *pResOut is set to zero.  The return value
///* is set to SQLITE_OK unless an I/O error occurs during lock checking.
extern "C" fn unix_check_reserved_lock(id: *mut Sqlite3File,
    p_res_out_1: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut reserved: i32 = 0;
        let p_file: *mut UnixFile = id as *mut UnixFile;
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_mutex_enter(unsafe {
                    (*unsafe { (*p_file).p_inode }).p_lock_mutex
                })
        };
        if unsafe { (*unsafe { (*p_file).p_inode }).e_file_lock } as i32 > 1 {
            reserved = 1;
        }
        if (reserved == 0) as i32 != 0 &&
                (unsafe { (*unsafe { (*p_file).p_inode }).b_process_lock } ==
                            0) as i32 != 0 {
            let mut lock: Flock = unsafe { core::mem::zeroed() };
            lock.l_whence = 0 as i16;
            lock.l_start = (sqlite3_pending_byte + 1) as OffT;
            lock.l_len = 1 as OffT;
            lock.l_type = 3 as i16;
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, i32, ...)
                                            ->
                                                i32>(a_syscall[7 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(unsafe { (*p_file).h }, 7, &mut lock)
                    } != 0 {
                rc = 10 | 14 << 8;
                store_last_errno(unsafe { &mut *p_file },
                    unsafe { *unsafe { __error() } });
            } else if lock.l_type as i32 != 2 { reserved = 1; }
        }
        unsafe {
            sqlite3_mutex_leave(unsafe {
                    (*unsafe { (*p_file).p_inode }).p_lock_mutex
                })
        };
        unsafe { *p_res_out_1 = reserved };
        return rc;
    }
}

///* Attempt to set the size of the memory mapping maintained by file
///* descriptor pFd to nNew bytes. Any existing mapping is discarded.
///*
///* If successful, this function sets the following variables:
///*
///*       unixFile.pMapRegion
///*       unixFile.mmapSize
///*       unixFile.mmapSizeActual
///*
///* If unsuccessful, an error message is logged via sqlite3_log() and
///* the three variables above are zeroed. In this case SQLite should
///* continue accessing the database using the xRead() and xWrite()
///* methods.
#[allow(unused_doc_comments)]
extern "C" fn unix_remapfile(p_fd_1: &mut UnixFile, mut n_new_1: i64) -> () {
    unsafe {
        let z_err: *const i8 = c"mmap".as_ptr() as *mut i8 as *const i8;
        let h: i32 = (*p_fd_1).h;
        /// File descriptor open on db file
        let p_orig: *mut u8 = (*p_fd_1).p_map_region as *mut u8;
        /// Pointer to current file mapping
        let n_orig: i64 = (*p_fd_1).mmap_size_actual;
        /// Size of pOrig region in bytes
        let mut p_new: *mut u8 = core::ptr::null_mut();
        /// Location of new mapping
        let flags: i32 = 1;

        /// Flags to pass to mmap()
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if !(p_orig).is_null() {
            let sz_syspage: i32 =
                unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn()
                                            ->
                                                i32>(a_syscall[25 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })()
                    } as i32;
            let n_reuse: i64 =
                (*p_fd_1).mmap_size & !(sz_syspage - 1) as Sqlite3Int64;
            let p_req: *mut u8 =
                unsafe { &mut *p_orig.offset(n_reuse as isize) };
            if n_reuse != n_orig {
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut (), u64)
                                        ->
                                            i32>(a_syscall[23 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(p_req as *mut (), (n_orig - n_reuse) as u64)
                };
            }
            p_new =
                unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut (), u64, i32, i32, i32, i64)
                                            ->
                                                *mut ()>(a_syscall[22 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(p_req as *mut (), (n_new_1 - n_reuse) as u64, flags, 1,
                            h, n_reuse)
                    } as *mut u8;
            if p_new as *mut () != -1i32 as *mut () {
                if p_new != p_req {
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut (), u64)
                                            ->
                                                i32>(a_syscall[23 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(p_new as *mut (), (n_new_1 - n_reuse) as u64)
                    };
                    p_new = core::ptr::null_mut();
                } else { p_new = p_orig; }
            }
            if p_new as *mut () == -1i32 as *mut () ||
                    p_new == core::ptr::null_mut() {
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut (), u64)
                                        ->
                                            i32>(a_syscall[23 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(p_orig as *mut (), n_reuse as u64)
                };
            }
        }
        if p_new == core::ptr::null_mut() {
            p_new =
                unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut (), u64, i32, i32, i32, i64)
                                            ->
                                                *mut ()>(a_syscall[22 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(core::ptr::null_mut(), n_new_1 as u64, flags, 1, h, 0)
                    } as *mut u8;
        }
        if p_new as *mut () == -1i32 as *mut () {
            p_new = core::ptr::null_mut();
            n_new_1 = 0 as i64;
            unix_log_error_at_line(0, z_err, (*p_fd_1).z_path, 5650);

            /// If the mmap() above failed, assume that all subsequent mmap() calls
            ///* will probably fail too. Fall back to using xRead/xWrite exclusively
            ///* in this case.
            ((*p_fd_1).mmap_size_max = 0 as Sqlite3Int64);
        }
        (*p_fd_1).p_map_region = p_new as *mut ();
        (*p_fd_1).mmap_size =
            {
                (*p_fd_1).mmap_size_actual = n_new_1;
                (*p_fd_1).mmap_size_actual
            };
    }
}

///* Memory map or remap the file opened by file-descriptor pFd (if the file
///* is already mapped, the existing mapping is replaced by the new). Or, if
///* there already exists a mapping for this file, and there are still
///* outstanding xFetch() references to it, this function is a no-op.
///*
///* If parameter nByte is non-negative, then it is the requested size of
///* the mapping to create. Otherwise, if nByte is less than zero, then the
///* requested size is the size of the file on disk. The actual size of the
///* created mapping is either the requested size or the value configured
///* using SQLITE_FCNTL_MMAP_LIMIT, whichever is smaller.
///*
///* SQLITE_OK is returned if no error occurs (even if the mapping is not
///* recreated as a result of outstanding references) or an SQLite error
///* code otherwise.
extern "C" fn unix_mapfile(p_fd_1: *mut UnixFile, mut n_map_1: i64) -> i32 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_fd_1).n_fetch_out } > 0 { return 0; }
        if n_map_1 < 0 as i64 {
            let mut statbuf: Stat = unsafe { core::mem::zeroed() };
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, *mut Stat)
                                            ->
                                                i32>(a_syscall[5 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(unsafe { (*p_fd_1).h }, &mut statbuf)
                    } != 0 {
                return 10 | 7 << 8;
            }
            n_map_1 = statbuf.st_size;
        }
        if n_map_1 > unsafe { (*p_fd_1).mmap_size_max } {
            n_map_1 = unsafe { (*p_fd_1).mmap_size_max };
        }
        { let _ = 0; };
        if n_map_1 != unsafe { (*p_fd_1).mmap_size } {
            unix_remapfile(unsafe { &mut *p_fd_1 }, n_map_1);
        }
        return 0;
    }
}

///* This function is called to handle the SQLITE_FCNTL_SIZE_HINT
///* file-control operation.  Enlarge the database to nBytes in size
///* (rounded up to the next chunk-size).  If the database is already
///* nBytes or larger, this routine is a no-op.
#[allow(unused_doc_comments)]
extern "C" fn fcntl_size_hint(p_file_1: *mut UnixFile, n_byte_1: i64) -> i32 {
    unsafe {
        if unsafe { (*p_file_1).sz_chunk } > 0 {
            let mut n_size: i64 = 0 as i64;
            /// Required file size
            let mut buf: Stat = unsafe { core::mem::zeroed() };
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, *mut Stat)
                                            ->
                                                i32>(a_syscall[5 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(unsafe { (*p_file_1).h }, &mut buf)
                    } != 0 {
                return 10 | 7 << 8;
            }
            n_size =
                (n_byte_1 + unsafe { (*p_file_1).sz_chunk } as i64 - 1 as i64)
                        / unsafe { (*p_file_1).sz_chunk } as i64 *
                    unsafe { (*p_file_1).sz_chunk } as i64;
            if n_size > buf.st_size as i64 {
                /// If the OS does not have posix_fallocate(), fake it. Write a
                ///* single byte to the last byte in each block that falls entirely
                ///* within the extended region. Then, if required, a single byte
                ///* at offset (nSize-1), to set the size of the file correctly.
                ///* This is a similar technique to that used by glibc on systems
                ///* that do not have a real fallocate() call.
                let n_blk: i32 = buf.st_blksize;
                /// File-system block size
                let mut n_write: i32 = 0;
                /// Number of bytes written by seekAndWrite
                let mut i_write: i64 = 0 as i64;

                /// Next offset to write to
                (i_write =
                    buf.st_size / n_blk as OffT * n_blk as OffT + n_blk as OffT
                        - 1 as OffT);
                { let _ = 0; };
                { let _ = 0; };
                {
                    '__b11: loop {
                        if !(i_write < n_size + n_blk as i64 - 1 as i64) {
                            break '__b11;
                        }
                        '__c11: loop {
                            if i_write >= n_size { i_write = n_size - 1 as i64; }
                            n_write =
                                seek_and_write(unsafe { &mut *p_file_1 }, i_write,
                                    c"".as_ptr() as *mut i8 as *const (), 1);
                            if n_write != 1 { return 10 | 3 << 8; }
                            break '__c11;
                        }
                        i_write += n_blk as i64;
                    }
                }
            }
        }
        if unsafe { (*p_file_1).mmap_size_max } > 0 as i64 &&
                n_byte_1 > unsafe { (*p_file_1).mmap_size } {
            let mut rc: i32 = 0;
            if unsafe { (*p_file_1).sz_chunk } <= 0 {
                if robust_ftruncate(unsafe { (*p_file_1).h }, n_byte_1) != 0 {
                    store_last_errno(unsafe { &mut *p_file_1 },
                        unsafe { *unsafe { __error() } });
                    return unix_log_error_at_line(10 | 6 << 8,
                            c"ftruncate".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_file_1).z_path }, 4100);
                }
            }
            rc = unsafe { unix_mapfile(p_file_1, n_byte_1) };
            return rc;
        }
        return 0;
    }
}

///* If *pArg is initially negative then this is a query.  Set *pArg to
///* 1 or 0 depending on whether or not bit mask of pFile->ctrlFlags is set.
///*
///* If *pArg is 0 or 1, then clear or set the mask bit of pFile->ctrlFlags.
extern "C" fn unix_mode_bit(p_file_1: &mut UnixFile, mask: u8,
    p_arg_1: &mut i32) -> () {
    if *p_arg_1 < 0 {
        *p_arg_1 = ((*p_file_1).ctrl_flags as i32 & mask as i32 != 0) as i32;
    } else if *p_arg_1 == 0 {
        (*p_file_1).ctrl_flags &= !mask as u16;
    } else { (*p_file_1).ctrl_flags |= mask as i32 as u16; }
}

///* Directories to consider for temp files.
static mut az_temp_dirs: [*const i8; 6] =
    [core::ptr::null(), core::ptr::null(), c"/var/tmp".as_ptr() as *const i8,
            c"/usr/tmp".as_ptr() as *const i8, c"/tmp".as_ptr() as *const i8,
            c".".as_ptr() as *const i8];

///* Return the name of a directory in which to put temporary files.
///* If no suitable temporary file directory can be found, return NULL.
extern "C" fn unix_temp_file_dir() -> *const i8 {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut buf: Stat = unsafe { core::mem::zeroed() };
        let mut z_dir: *const i8 = sqlite3_temp_directory as *const i8;
        loop {
            if z_dir != core::ptr::null() &&
                            unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*const i8, *mut Stat)
                                                        ->
                                                            i32>(a_syscall[4 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(z_dir, &mut buf)
                                } == 0 && buf.st_mode as i32 & 61440 == 16384 &&
                    unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*const i8, i32)
                                                ->
                                                    i32>(a_syscall[2 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(z_dir, 3)
                        } == 0 {
                return z_dir;
            }
            if i as u64 >=
                    core::mem::size_of::<[*const i8; 6]>() as u64 /
                        core::mem::size_of::<*const i8>() as u64 {
                break;
            }
            z_dir =
                az_temp_dirs[{
                            let __p = &mut i;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        } as usize];
        }
        return core::ptr::null();
    }
}

/// Forward declaration
#[allow(unused_doc_comments)]
extern "C" fn unix_get_tempname(n_buf: i32, z_buf: *mut i8) -> i32 {
    unsafe {
        let mut z_dir: *const i8 = core::ptr::null();
        let mut i_limit: i32 = 0;
        let mut rc: i32 = 0;

        /// It's odd to simulate an io-error here, but really this is just
        ///* using the io-error infrastructure to test that SQLite handles this
        ///* function failing.
        unsafe { *z_buf.offset(0 as isize) = 0 as i8 };
        unsafe { sqlite3_mutex_enter(unsafe { sqlite3MutexAlloc(11) }) };
        z_dir = unix_temp_file_dir();
        if z_dir == core::ptr::null() {
            rc = 10 | 25 << 8;
        } else {
            '__b13: loop {
                '__c13: loop {
                    let mut r: u64 = 0 as u64;
                    unsafe {
                        sqlite3_randomness(core::mem::size_of::<u64>() as i32,
                            &raw mut r as *mut ())
                    };
                    { let _ = 0; };
                    unsafe { *z_buf.offset((n_buf - 2) as isize) = 0 as i8 };
                    unsafe {
                        sqlite3_snprintf(n_buf, z_buf,
                            c"%s/etilqs_%llx%c".as_ptr() as *mut i8 as *const i8, z_dir,
                            r, 0)
                    };
                    if unsafe { *z_buf.offset((n_buf - 2) as isize) } as i32 !=
                                0 ||
                            { let __p = &mut i_limit; let __t = *__p; *__p += 1; __t } >
                                10 {
                        rc = 1;
                        break '__b13;
                    }
                    break '__c13;
                }
                if !(unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*const i8, i32)
                                                        ->
                                                            i32>(a_syscall[2 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(z_buf as *const i8, 0)
                                } == 0) {
                    break '__b13;
                }
            }
        }
        unsafe { sqlite3_mutex_leave(unsafe { sqlite3MutexAlloc(11) }) };
        return rc;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ProxyLockingContext {
    conch_file: *mut UnixFile,
    conch_file_path: *mut i8,
    lock_proxy: *mut UnixFile,
    lock_proxy_path: *mut i8,
    db_path: *mut i8,
    conch_held: i32,
    n_fails: i32,
    old_locking_context: *mut (),
    p_old_method: *const Sqlite3IoMethods,
}

///* If pFile holds a lock on a conch file, then release that lock.
#[allow(unused_doc_comments)]
extern "C" fn proxy_release_conch(p_file_1: &mut UnixFile) -> i32 {
    let mut rc: i32 = 0;
    /// Subroutine return code
    let mut p_ctx: *mut ProxyLockingContext = core::ptr::null_mut();
    /// The locking context for the proxy lock
    let mut conch_file: *mut UnixFile = core::ptr::null_mut();

    /// Name of the conch file
    (p_ctx = (*p_file_1).locking_context as *mut ProxyLockingContext);

    /// Name of the conch file
    (conch_file = unsafe { (*p_ctx).conch_file });
    if unsafe { (*p_ctx).conch_held } > 0 {
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*conch_file).p_method }).x_unlock.unwrap()
                    })(conch_file as *mut Sqlite3File, 0)
            };
    }
    unsafe { (*p_ctx).conch_held = 0 };
    return rc;
}

///* Close a file that uses proxy locks.
#[allow(unused_doc_comments)]
extern "C" fn proxy_close(id: *mut Sqlite3File) -> i32 {
    if !(id).is_null() {
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let p_ctx: *mut ProxyLockingContext =
            unsafe { (*p_file).locking_context } as *mut ProxyLockingContext;
        let lock_proxy: *mut UnixFile = unsafe { (*p_ctx).lock_proxy };
        let conch_file: *mut UnixFile = unsafe { (*p_ctx).conch_file };
        let mut rc: i32 = 0;
        if !(lock_proxy).is_null() {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*lock_proxy).p_method }).x_unlock.unwrap()
                        })(lock_proxy as *mut Sqlite3File, 0)
                };
            if rc != 0 { return rc; }
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*lock_proxy).p_method }).x_close.unwrap()
                        })(lock_proxy as *mut Sqlite3File)
                };
            if rc != 0 { return rc; }
            unsafe { sqlite3_free(lock_proxy as *mut ()) };
            unsafe { (*p_ctx).lock_proxy = core::ptr::null_mut() };
        }
        if !(conch_file).is_null() {
            if unsafe { (*p_ctx).conch_held } != 0 {
                rc = proxy_release_conch(unsafe { &mut *p_file });
                if rc != 0 { return rc; }
            }
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*conch_file).p_method }).x_close.unwrap()
                        })(conch_file as *mut Sqlite3File)
                };
            if rc != 0 { return rc; }
            unsafe { sqlite3_free(conch_file as *mut ()) };
        }
        unsafe {
            sqlite3_db_free(core::ptr::null_mut(),
                unsafe { (*p_ctx).lock_proxy_path } as *mut ())
        };
        unsafe {
            sqlite3_free(unsafe { (*p_ctx).conch_file_path } as *mut ())
        };
        unsafe {
            sqlite3_db_free(core::ptr::null_mut(),
                unsafe { (*p_ctx).db_path } as *mut ())
        };

        /// restore the original locking context and pMethod then close it
        unsafe {
            (*p_file).locking_context =
                unsafe { (*p_ctx).old_locking_context }
        };
        unsafe { (*p_file).p_method = unsafe { (*p_ctx).p_old_method } };
        unsafe { sqlite3_free(p_ctx as *mut ()) };
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_file).p_method }).x_close.unwrap()
                    })(id)
            };
    }
    return 0;
}

/// get the host ID via gethostuuid(), pHostID must point to PROXY_HOSTIDLEN
///* bytes of writable memory.
extern "C" fn proxy_get_host_id(p_host_id_1: *mut u8, p_error_1: *mut i32)
    -> i32 {
    { let _ = 0; };
    unsafe { memset(p_host_id_1 as *mut (), 0, 16 as u64) };
    {
        let mut timeout: Timespec =
            Timespec { tv_sec: 1 as i64, tv_nsec: 0 as i64 };
        if unsafe {
                    gethostuuid(p_host_id_1,
                        &raw mut timeout as *const Timespec)
                } != 0 {
            let err: i32 = unsafe { *unsafe { __error() } };
            if !(p_error_1).is_null() { unsafe { *p_error_1 = err }; }
            return 10;
        }
    }
    return 0;
}

/// Forward declaration
#[allow(unused_doc_comments)]
extern "C" fn unix_sleep(not_used: *mut Sqlite3Vfs, microseconds: i32)
    -> i32 {
    let mut sp: Timespec = unsafe { core::mem::zeroed() };
    sp.tv_sec = (microseconds / 1000000) as DarwinTimeT;
    sp.tv_nsec = (microseconds % 1000000 * 1000) as i64;

    /// Almost all modern unix systems support nanosleep().  But if you are
    ///* compiling for one of the rare exceptions, you can use
    ///* -DHAVE_NANOSLEEP=0 (perhaps in conjunction with -DHAVE_USLEEP if
    ///* usleep() is available) in order to bypass the use of nanosleep()
    unsafe {
        nanosleep(&raw mut sp as *const Timespec,
            0 as *mut () as *mut Timespec)
    };
    { let _ = not_used; };
    return microseconds;
}

///* Takes an open conch file, copies the contents to a new path and then moves
///* it back.  The newly created file's file descriptor is assigned to the
///* conch file structure and finally the original conch file descriptor is
///* closed.  Returns zero if successful.
#[allow(unused_doc_comments)]
extern "C" fn proxy_break_conch_lock(p_file_1: *mut UnixFile,
    my_host_id_1: *const u8) -> i32 {
    unsafe {
        let p_ctx: *const ProxyLockingContext =
            unsafe { (*p_file_1).locking_context } as *mut ProxyLockingContext
                as *const ProxyLockingContext;
        let mut t_path: [i8; 1024] = [0; 1024];
        let c_path: *mut i8 = unsafe { (*p_ctx).conch_file_path };
        let mut errmsg: [i8; 64] =
            [0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
                    0 as i8, 0 as i8, 0 as i8];
        let mut fd: i32 = -1;
        let mut rc: i32 = -1;
        '__b14: loop {
            '__c14: loop {
                let conch_file: *mut UnixFile =
                    unsafe { (*p_ctx).conch_file };
                let mut buf: [i8; 1041] = [0; 1041];
                let mut read_len: u64 = 0 as u64;
                let mut path_len: u64 = 0 as u64;
                { let _ = my_host_id_1; };

                /// create a new path by replace the trailing '-conch' with '-break'
                (path_len =
                    unsafe {
                        strlcpy(&raw mut t_path[0 as usize] as *mut i8,
                            c_path as *const i8, 1024 as u64)
                    });
                if path_len > 1024 as u64 || path_len < 6 as u64 ||
                        unsafe {
                                strlcpy(&mut t_path[(path_len - 5 as u64) as usize],
                                    c"break".as_ptr() as *mut i8 as *const i8, 6 as u64)
                            } != 5 as u64 {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 64]>() as i32,
                            &raw mut errmsg[0 as usize] as *mut i8,
                            c"path error (len %d)".as_ptr() as *mut i8 as *const i8,
                            path_len as i32)
                    };
                    break '__b14;
                }

                /// read the conch content
                (read_len =
                    unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(i32, *mut (), u64, i64)
                                                ->
                                                    i64>(a_syscall[9 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(unsafe { (*conch_file).h },
                                &raw mut buf[0 as usize] as *mut i8 as *mut (),
                                (1 + 16 + 1024) as u64, 0)
                        } as u64);
                if read_len < (1 + 16) as u64 {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 64]>() as i32,
                            &raw mut errmsg[0 as usize] as *mut i8,
                            c"read error (len %d)".as_ptr() as *mut i8 as *const i8,
                            read_len as i32)
                    };
                    break '__b14;
                }

                /// write it out to the temporary break file
                (fd =
                    robust_open(&raw mut t_path[0 as usize] as *mut i8 as
                            *const i8, 2 | 512 | 2048 | 256, 0 as ModeT));
                if fd < 0 {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 64]>() as i32,
                            &raw mut errmsg[0 as usize] as *mut i8,
                            c"create failed (%d)".as_ptr() as *mut i8 as *const i8,
                            unsafe { *unsafe { __error() } })
                    };
                    break '__b14;
                }
                if unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(i32, *const (), u64, i64)
                                                ->
                                                    i64>(a_syscall[12 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(fd, &raw mut buf[0 as usize] as *mut i8 as *const (),
                                read_len, 0)
                        } != read_len as i64 {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 64]>() as i32,
                            &raw mut errmsg[0 as usize] as *mut i8,
                            c"write failed (%d)".as_ptr() as *mut i8 as *const i8,
                            unsafe { *unsafe { __error() } })
                    };
                    break '__b14;
                }
                if unsafe {
                            rename(&raw mut t_path[0 as usize] as *mut i8 as *const i8,
                                c_path as *const i8)
                        } != 0 {
                    unsafe {
                        sqlite3_snprintf(core::mem::size_of::<[i8; 64]>() as i32,
                            &raw mut errmsg[0 as usize] as *mut i8,
                            c"rename failed (%d)".as_ptr() as *mut i8 as *const i8,
                            unsafe { *unsafe { __error() } })
                    };
                    break '__b14;
                }
                rc = 0;
                unsafe {
                    fprintf(__stderrp,
                        c"broke stale lock on %s\n".as_ptr() as *mut i8 as
                            *const i8, c_path)
                };
                robust_close(p_file_1 as *const UnixFile,
                    unsafe { (*conch_file).h }, 7707);
                unsafe { (*conch_file).h = fd };
                unsafe { (*conch_file).open_flags = 2 | 512 };
                break '__c14;
            }
            if !(false) { break '__b14; }
        }
        if rc != 0 {
            if fd >= 0 {
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8)
                                        ->
                                            i32>(a_syscall[16 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(&raw mut t_path[0 as usize] as *mut i8 as *const i8)
                };
                robust_close(p_file_1 as *const UnixFile, fd, 7715);
            }
            unsafe {
                fprintf(__stderrp,
                    c"failed to break stale lock on %s, %s\n".as_ptr() as
                            *mut i8 as *const i8, c_path,
                    &raw mut errmsg[0 as usize] as *mut i8)
            };
        }
        return rc;
    }
}

/// Take the requested lock on the conch file and break a stale lock if the
///* host id matches.
#[allow(unused_doc_comments)]
extern "C" fn proxy_conch_lock(p_file_1: *mut UnixFile, my_host_id_1: *mut u8,
    lock_type_1: i32) -> i32 {
    unsafe {
        let p_ctx: *const ProxyLockingContext =
            unsafe { (*p_file_1).locking_context } as *mut ProxyLockingContext
                as *const ProxyLockingContext;
        let conch_file: *mut UnixFile = unsafe { (*p_ctx).conch_file };
        let mut rc: i32 = 0;
        let mut n_tries: i32 = 0;
        let mut conch_mod_time: Timespec = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut conch_mod_time as *mut (), 0,
                core::mem::size_of::<Timespec>() as u64)
        };
        '__b15: loop {
            '__c15: loop {
                rc =
                    unsafe {
                        (unsafe {
                                (*unsafe { (*conch_file).p_method }).x_lock.unwrap()
                            })(conch_file as *mut Sqlite3File, lock_type_1)
                    };
                { let __p = &mut n_tries; let __t = *__p; *__p += 1; __t };
                if rc == 5 {
                    /// If the lock failed (busy):
                    ///1st try: get the mod time of the conch, wait 0.5s and try again.
                    ///2nd try: fail if the mod time changed or host id is different, wait
                    ///          10 sec and try again
                    ///3rd try: break the lock unless the mod time has changed.
                    let mut buf: Stat = unsafe { core::mem::zeroed() };
                    if unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(i32, *mut Stat)
                                                    ->
                                                        i32>(a_syscall[5 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(unsafe { (*conch_file).h }, &mut buf)
                            } != 0 {
                        store_last_errno(unsafe { &mut *p_file_1 },
                            unsafe { *unsafe { __error() } });
                        return 10 | 15 << 8;
                    }
                    if n_tries == 1 {
                        conch_mod_time = buf.st_mtimespec;
                        unix_sleep(core::ptr::null_mut(), 500000);

                        /// wait 0.5 sec and try the lock again
                        break '__c15;
                    }
                    { let _ = 0; };
                    if conch_mod_time.tv_sec != buf.st_mtimespec.tv_sec ||
                            conch_mod_time.tv_nsec != buf.st_mtimespec.tv_nsec {
                        return 5;
                    }
                    if n_tries == 2 {
                        let mut t_buf: [i8; 1041] = [0; 1041];
                        let len: i32 =
                            unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(i32, *mut (), u64, i64)
                                                        ->
                                                            i64>(a_syscall[9 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(unsafe { (*conch_file).h },
                                        &raw mut t_buf[0 as usize] as *mut i8 as *mut (),
                                        (1 + 16 + 1024) as u64, 0)
                                } as i32;
                        if len < 0 {
                            store_last_errno(unsafe { &mut *p_file_1 },
                                unsafe { *unsafe { __error() } });
                            return 10 | 15 << 8;
                        }
                        if len > 1 + 16 &&
                                t_buf[0 as usize] as i32 == 2 as i8 as i32 {
                            if 0 !=
                                    unsafe {
                                        memcmp(&raw mut t_buf[1 as usize] as *const (),
                                            my_host_id_1 as *const (), 16 as u64)
                                    } {
                                return 5;
                            }
                        } else {

                            /// don't break the lock on short read or a version mismatch
                            return 5;
                        }
                        unix_sleep(core::ptr::null_mut(), 10000000);

                        /// wait 10 sec and try the lock again
                        break '__c15;
                    }
                    { let _ = 0; };
                    if 0 ==
                            proxy_break_conch_lock(p_file_1, my_host_id_1 as *const u8)
                        {
                        rc = 0;
                        if lock_type_1 == 4 {
                            rc =
                                unsafe {
                                    (unsafe {
                                            (*unsafe { (*conch_file).p_method }).x_lock.unwrap()
                                        })(conch_file as *mut Sqlite3File, 1)
                                };
                        }
                        if (rc == 0) as i32 != 0 {
                            rc =
                                unsafe {
                                    (unsafe {
                                            (*unsafe { (*conch_file).p_method }).x_lock.unwrap()
                                        })(conch_file as *mut Sqlite3File, lock_type_1)
                                };
                        }
                    }
                }
                break '__c15;
            }
            if !(rc == 5 && n_tries < 3) { break '__b15; }
        }
        return rc;
    }
}

///* The proxy lock file path for the database at dbPath is written into lPath,
///* which must point to valid, writable memory large enough for a maxLen length
///* file path.
#[allow(unused_doc_comments)]
extern "C" fn proxy_get_lock_path(db_path_1: *const i8, l_path_1: *mut i8,
    max_len_1: u64) -> i32 {
    let mut len: i32 = 0;
    let mut db_len: i32 = 0;
    let mut i: i32 = 0;
    {
        if (unsafe { confstr(65537, l_path_1, max_len_1) } == 0) as i32 != 0 {
            return 10 | 15 << 8;
        }
        len =
            unsafe {
                    strlcat(l_path_1,
                        c"sqliteplocks".as_ptr() as *mut i8 as *const i8, max_len_1)
                } as i32;
    }
    if unsafe { *l_path_1.offset((len - 1) as isize) } as i32 != '/' as i32 {
        len =
            unsafe {
                    strlcat(l_path_1, c"/".as_ptr() as *mut i8 as *const i8,
                        max_len_1)
                } as i32;
    }

    /// transform the db path to a unique cache name
    (db_len = unsafe { strlen(db_path_1) } as i32);
    {
        i = 0;
        '__b16: loop {
            if !(i < db_len && i + len + 7 < max_len_1 as i32) {
                break '__b16;
            }
            '__c16: loop {
                let c: i8 = unsafe { *db_path_1.offset(i as isize) } as i8;
                unsafe {
                    *l_path_1.offset((i + len) as isize) =
                        if c as i32 == '/' as i32 { '_' as i32 } else { c as i32 }
                            as i8
                };
                break '__c16;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *l_path_1.offset((i + len) as isize) = '\u{0}' as i32 as i8 };
    unsafe {
        strlcat(l_path_1, c":auto:".as_ptr() as *mut i8 as *const i8,
            max_len_1)
    };
    return 0;
}

///* Search for an unused file descriptor that was opened on the database
///* file (not a journal or super-journal file) identified by pathname
///* zPath with SQLITE_OPEN_XXX flags matching those passed as the second
///* argument to this function.
///*
///* Such a file descriptor may exist if a database connection was closed
///* but the associated file descriptor could not be closed because some
///* other file descriptor open on the same file is holding a file-lock.
///* Refer to comments in the unixClose() function and the lengthy comment
///* describing "Posix Advisory Locking" at the start of this file for
///* further details. Also, ticket #4018.
///*
///* If a suitable file descriptor is found, then it is returned. If no
///* such file descriptor is located, -1 is returned.
#[allow(unused_doc_comments)]
extern "C" fn find_reusable_fd(z_path_1: *const i8, mut flags: i32)
    -> *mut UnixUnusedFd {
    unsafe {
        let mut p_unused: *mut UnixUnusedFd = core::ptr::null_mut();
        /// Do not search for an unused file descriptor on vxworks. Not because
        ///* vxworks would not benefit from the change (it might, we're not sure),
        ///* but because no way to test it is currently available. It is better
        ///* not to risk breaking vxworks support for the sake of such an obscure
        ///* feature.
        let mut s_stat: Stat = unsafe { core::mem::zeroed() };

        /// Results of stat() call
        unix_enter_mutex();
        if inode_list != core::ptr::null_mut() &&
                0 ==
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*const i8, *mut Stat)
                                            ->
                                                i32>(a_syscall[4 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(z_path_1, &mut s_stat)
                    } {
            let mut p_inode: *mut UnixInodeInfo = core::ptr::null_mut();
            p_inode = inode_list;
            while !(p_inode).is_null() &&
                    (unsafe { (*p_inode).file_id.dev } != s_stat.st_dev ||
                        unsafe { (*p_inode).file_id.ino } != s_stat.st_ino as u64) {
                p_inode = unsafe { (*p_inode).p_next };
            }
            if !(p_inode).is_null() {
                let mut pp: *mut *mut UnixUnusedFd = core::ptr::null_mut();
                { let _ = 0; };
                unsafe {
                    sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex })
                };
                flags &= 1 | 2;
                {
                    pp = unsafe { &mut (*p_inode).p_unused };
                    '__b18: loop {
                        if !(!(unsafe { *pp }).is_null() &&
                                        unsafe { (*unsafe { *pp }).flags } != flags) {
                            break '__b18;
                        }
                        '__c18: loop { break '__c18; }
                        pp = unsafe { &mut (*unsafe { *pp }).p_next };
                    }
                }
                p_unused = unsafe { *pp };
                if !(p_unused).is_null() {
                    unsafe { *pp = unsafe { (*p_unused).p_next } };
                }
                unsafe {
                    sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex })
                };
            }
        }
        unix_leave_mutex();

        /// if !OS_VXWORKS
        return p_unused;
    }
}

///* Creates the lock file and any missing directories in lockPath
#[allow(unused_doc_comments)]
extern "C" fn proxy_create_lock_path(lock_path_1: *const i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut buf: [i8; 1024] = [0; 1024];
        let mut start: i32 = 0;
        { let _ = 0; };

        /// try to create all the intermediate directories
        (len = unsafe { strlen(lock_path_1) } as i32);

        /// try to create all the intermediate directories
        (buf[0 as usize] = unsafe { *lock_path_1.offset(0 as isize) } as i8);
        {
            i = 1;
            '__b19: loop {
                if !(i < len) { break '__b19; }
                '__c19: loop {
                    if unsafe { *lock_path_1.offset(i as isize) } as i32 ==
                                '/' as i32 && i - start > 0 {
                        if i - start > 2 ||
                                    i - start == 1 && buf[start as usize] as i32 != '.' as i32
                                        && buf[start as usize] as i32 != '/' as i32 ||
                                i - start == 2 && buf[start as usize] as i32 != '.' as i32
                                    && buf[(start + 1) as usize] as i32 != '.' as i32 {
                            buf[i as usize] = '\u{0}' as i32 as i8;
                            if unsafe {
                                        (unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*const i8, u16)
                                                            ->
                                                                i32>(a_syscall[18 as
                                                                        usize].p_current.unwrap_or(unsafe {
                                                                core::mem::transmute::<*const (),
                                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                                            }) as *const ())
                                            })(&raw mut buf[0 as usize] as *mut i8 as *const i8, 493)
                                    } != 0 {
                                let err: i32 = unsafe { *unsafe { __error() } };
                                if err != 17 { return err; }
                            }
                        }
                        start = i + 1;
                    }
                    buf[i as usize] =
                        unsafe { *lock_path_1.offset(i as isize) } as i8;
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}

///* Close the file.
extern "C" fn nolock_close(id: *mut Sqlite3File) -> i32 {
    return close_unix_file(id);
}

extern "C" fn nolock_lock(not_used_1: *mut Sqlite3File, not_used2_1: i32)
    -> i32 {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    return 0;
}

extern "C" fn nolock_unlock(not_used_1: *mut Sqlite3File, not_used2_1: i32)
    -> i32 {
    { { let _ = not_used_1; }; { let _ = not_used2_1; } };
    return 0;
}

///***************************************************************************
///***************************** No-op Locking **********************************
///*
///* Of the various locking implementations available, this is by far the
///* simplest:  locking is ignored.  No attempt is made to lock the database
///* file for reading or writing.
///*
///* This locking mode is appropriate for use on read-only databases
///* (ex: databases that are burned into CD-ROM, for example.)  It can
///* also be used if the application employs some external mechanism to
///* prevent simultaneous access of the same database by two or more
///* database connections.  But there is a serious risk of database
///* corruption if this locking mode is used in situations where multiple
///* database connections are accessing the same database file at the same
///* time and one or more of those connections are writing.
extern "C" fn nolock_check_reserved_lock(not_used_1: *mut Sqlite3File,
    p_res_out_1: *mut i32) -> i32 {
    { let _ = not_used_1; };
    unsafe { *p_res_out_1 = 0 };
    return 0;
}

extern "C" fn set_device_characteristics(p_fd_1: &mut UnixFile) -> () {
    { let _ = 0; };
    if (*p_fd_1).sector_size == 0 {
        if (*p_fd_1).ctrl_flags as i32 & 16 != 0 {
            (*p_fd_1).device_characteristics |= 4096;
        }
        (*p_fd_1).device_characteristics |= 32768;
        (*p_fd_1).sector_size = 4096;
    }
}

///* Return the sector size in bytes of the underlying block device for
///* the specified file. This is almost always 512 bytes, but may be
///* larger for some devices.
///*
///* SQLite code assumes this function cannot fail. It also assumes that
///* if two files are created in the same file-system directory (i.e.
///* a database and its journal file) that the sector size will be the
///* same for both.
extern "C" fn unix_sector_size(id: *mut Sqlite3File) -> i32 {
    let p_fd: *mut UnixFile = id as *mut UnixFile;
    set_device_characteristics(unsafe { &mut *p_fd });
    return unsafe { (*p_fd).sector_size };
}

///* Return the device characteristics for the file.
///*
///* This VFS is set up to return SQLITE_IOCAP_POWERSAFE_OVERWRITE by default.
///* However, that choice is controversial since technically the underlying
///* file system does not always provide powersafe overwrites.  (In other
///* words, after a power-loss event, parts of the file that were never
///* written might end up being altered.)  However, non-PSOW behavior is very,
///* very rare.  And asserting PSOW makes a large reduction in the amount
///* of required I/O for journaling, since a lot of padding is eliminated.
///*  Hence, while POWERSAFE_OVERWRITE is on by default, there is a file-control
///* available to turn it off and URI query parameter available to turn it off.
extern "C" fn unix_device_characteristics(id: *mut Sqlite3File) -> i32 {
    let p_fd: *mut UnixFile = id as *mut UnixFile;
    set_device_characteristics(unsafe { &mut *p_fd });
    return unsafe { (*p_fd).device_characteristics };
}

///* Apply posix advisory locks for all bytes from ofst through ofst+n-1.
///*
///* Locks block if the mask is exactly UNIX_SHM_C and are non-blocking
///* otherwise.
#[allow(unused_doc_comments)]
extern "C" fn unix_shm_system_lock(p_file_1: &UnixFile, lock_type_1: i32,
    ofst: i32, n: i32) -> i32 {
    unsafe {
        let mut p_shm_node: *const UnixShmNode = core::ptr::null();
        /// Apply locks to this open shared-memory segment
        let mut f: Flock = unsafe { core::mem::zeroed() };
        /// The posix advisory locking structure
        let mut rc: i32 = 0;

        /// Result code form fcntl()
        (p_shm_node = unsafe { (*(*p_file_1).p_inode).p_shm_node });

        /// Assert that the parameters are within expected range and that the
        ///* correct mutex or mutexes are held.
        { let _ = 0; };
        { let _ = 0; };
        if ofst == (22 + 8) * 4 + 8 {
            { let _ = 0; };
            { let _ = 0; };
        } else { { let _ = 0; }; { let _ = 0; }; }

        /// Shared locks never span more than one byte
        { let _ = 0; };

        /// Locks are within range
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_shm_node).h_shm } >= 0 {
            let mut res: i32 = 0;

            /// Initialize the locking parameters
            (f.l_type = lock_type_1 as i16);
            f.l_whence = 0 as i16;
            f.l_start = ofst as OffT;
            f.l_len = n as OffT;
            res =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, i32, ...)
                                        ->
                                            i32>(a_syscall[7 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(unsafe { (*p_shm_node).h_shm }, 8, &mut f)
                };
            if res == -1 { rc = 5; }
        }

        /// Do debug tracing
        return rc;
    }
}

///* Change the lock state for a shared-memory segment.
///*
///* Note that the relationship between SHARED and EXCLUSIVE locks is a little
///* different here than in posix.  In xShmLock(), one can go from unlocked
///* to shared and back or from unlocked to exclusive and back.  But one may
///* not go from shared to exclusive or from exclusive to shared.
#[allow(unused_doc_comments)]
extern "C" fn unix_shm_lock(fd: *mut Sqlite3File, ofst: i32, n: i32,
    flags: i32) -> i32 {
    let p_db_fd: *mut UnixFile = fd as *mut UnixFile;
    /// Connection holding shared memory
    let mut p: *mut UnixShm = core::ptr::null_mut();
    /// The shared memory being locked
    let mut p_shm_node: *mut UnixShmNode = core::ptr::null_mut();
    /// The underlying file iNode
    let mut rc: i32 = 0;
    /// Result code
    let mask: u16 = ((1 << ofst + n) - (1 << ofst)) as u16;
    /// Mask of locks to take or release
    let mut a_lock: *mut i32 = core::ptr::null_mut();
    p = unsafe { (*p_db_fd).p_shm };
    if p == core::ptr::null_mut() { return 10 | 20 << 8; }
    p_shm_node = unsafe { (*p).p_shm_node };
    if p_shm_node == core::ptr::null_mut() { return 10 | 20 << 8; }
    a_lock = unsafe { &raw mut (*p_shm_node).a_lock[0 as usize] } as *mut i32;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };

    /// Check that, if this to be a blocking lock, no locks that occur later
    ///* in the following list than the lock being obtained are already held:
    ///*
    ///*   1. Recovery lock (ofst==2).
    ///*   2. Checkpointer lock (ofst==1).
    ///*   3. Write lock (ofst==0).
    ///*   4. Read locks (ofst>=3 && ofst<SQLITE_SHM_NLOCK).
    ///*
    ///* In other words, if this is a blocking lock, none of the locks that
    ///* occur later in the above list than the lock being obtained may be
    ///* held.
    /// Check if there is any work to do. There are three cases:
    ///*
    ///*    a) An unlock operation where there are locks to unlock,
    ///*    b) An shared lock where the requested lock is not already held
    ///*    c) An exclusive lock where the requested lock is not already held
    ///*
    ///* The SQLite core never requests an exclusive lock that it already holds.
    ///* This is assert()ed below.
    { let _ = 0; };
    if flags & 1 != 0 &&
                    (unsafe { (*p).excl_mask } as i32 |
                                unsafe { (*p).shared_mask } as i32) & mask as i32 != 0 ||
                flags == 4 | 2 &&
                    0 == unsafe { (*p).shared_mask } as i32 & mask as i32 ||
            flags == 8 | 2 {

        /// Take the required mutexes. In SETLK_TIMEOUT mode (blocking locks), if
        ///* this is an attempt on an exclusive lock use sqlite3_mutex_try(). If any
        ///* other thread is holding this mutex, then it is either holding or about
        ///* to hold a lock exclusive to the one being requested, and we may
        ///* therefore return SQLITE_BUSY to the caller.
        ///*
        ///* Doing this prevents some deadlock scenarios. For example, thread 1 may
        ///* be a checkpointer blocked waiting on the WRITER lock. And thread 2
        ///* may be a normal SQL client upgrading to a write transaction. In this
        ///* case thread 2 does a non-blocking request for the WRITER lock. But -
        ///* if it were to use sqlite3_mutex_enter() then it would effectively
        ///* become a (doomed) blocking request, as thread 2 would block until thread
        ///* 1 obtained WRITER and released the mutex. Since thread 2 already holds
        ///* a lock on a read-locking slot at this point, this breaks the
        ///* anti-deadlock rules (see above).
        unsafe { sqlite3_mutex_enter(unsafe { (*p_shm_node).p_shm_mutex }) };
        if rc == 0 {
            if flags & 1 != 0 {
                /// Case (a) - unlock.
                let mut b_unlock: i32 = 1;
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if flags & 4 != 0 {
                    { let _ = 0; };
                    { let _ = 0; };
                    if unsafe { *a_lock.offset(ofst as isize) } > 1 {
                        b_unlock = 0;
                        {
                            let __p = unsafe { &mut *a_lock.offset(ofst as isize) };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { (*p).shared_mask &= !mask as u16 };
                    }
                }
                if b_unlock != 0 {
                    rc =
                        unix_shm_system_lock(unsafe { &*p_db_fd }, 2,
                            ofst + (22 + 8) * 4, n);
                    if rc == 0 {
                        unsafe {
                            memset(unsafe { &raw mut *a_lock.offset(ofst as isize) } as
                                    *mut (), 0, core::mem::size_of::<i32>() as u64 * n as u64)
                        };
                        unsafe { (*p).shared_mask &= !mask as u16 };
                        unsafe { (*p).excl_mask &= !mask as u16 };
                    }
                }
            } else if flags & 4 != 0 {
                if unsafe { *a_lock.offset(ofst as isize) } < 0 {

                    /// An exclusive lock is held by some other connection. BUSY.
                    (rc = 5);
                } else if unsafe { *a_lock.offset(ofst as isize) } == 0 {
                    rc =
                        unix_shm_system_lock(unsafe { &*p_db_fd }, 1,
                            ofst + (22 + 8) * 4, n);
                }
                if rc == 0 {
                    unsafe { (*p).shared_mask |= mask as i32 as u16 };
                    {
                        let __p = unsafe { &mut *a_lock.offset(ofst as isize) };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
            } else {
                /// Case (c) - an exclusive lock.
                let mut ii: i32 = 0;
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                {
                    ii = ofst;
                    '__b20: loop {
                        if !(ii < ofst + n) { break '__b20; }
                        '__c20: loop {
                            if unsafe { *a_lock.offset(ii as isize) } != 0 {
                                rc = 5;
                                break '__b20;
                            }
                            break '__c20;
                        }
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    }
                }
                if rc == 0 {
                    rc =
                        unix_shm_system_lock(unsafe { &*p_db_fd }, 3,
                            ofst + (22 + 8) * 4, n);
                    if rc == 0 {
                        unsafe { (*p).excl_mask |= mask as i32 as u16 };
                        {
                            ii = ofst;
                            '__b21: loop {
                                if !(ii < ofst + n) { break '__b21; }
                                '__c21: loop {
                                    unsafe { *a_lock.offset(ii as isize) = -1 };
                                    break '__c21;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                }
            }
            { let _ = 0; };
        }

        /// Drop the mutexes acquired above.
        unsafe { sqlite3_mutex_leave(unsafe { (*p_shm_node).p_shm_mutex }) };
    }
    return rc;
}

///* Implement a memory barrier or memory fence on shared memory.
///*
///* All loads and stores begun before the barrier must complete before
///* any load or store begun after the barrier.
#[allow(unused_doc_comments)]
extern "C" fn unix_shm_barrier(fd: *mut Sqlite3File) -> () {
    { let _ = fd; };
    unsafe { sqlite3_memory_barrier() };

    /// compiler-defined memory barrier
    { let _ = 0; };
    unix_enter_mutex();

    /// Also mutex, for redundancy
    unix_leave_mutex();
}

///* Return the minimum number of 32KB shm regions that should be mapped at
///* a time, assuming that each mapping must be an integer multiple of the
///* current system page-size.
///*
///* Usually, this is 1. The exception seems to be systems that are configured
///* to use 64KB pages - in this case each mapping must cover at least two
///* shm regions.
#[allow(unused_doc_comments)]
extern "C" fn unix_shm_region_per_map() -> i32 {
    unsafe {
        let shmsz: i32 = 32 * 1024;
        /// SHM region size
        let pgsz: i32 =
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn()
                                    ->
                                        i32>(a_syscall[25 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })()
            };

        /// System page size
        { let _ = 0; };
        if pgsz < shmsz { return 1; }
        return pgsz / shmsz;
    }
}

///* Purge the unixShmNodeList list of all entries with unixShmNode.nRef==0.
///*
///* This is not a VFS shared-memory method; it is a utility function called
///* by VFS shared-memory methods.
extern "C" fn unix_shm_purge(p_fd_1: *mut UnixFile) -> () {
    unsafe {
        let p: *mut UnixShmNode =
            unsafe { (*unsafe { (*p_fd_1).p_inode }).p_shm_node };
        { let _ = 0; };
        if !(p).is_null() && unsafe { (*p).n_ref } == 0 {
            let n_shm_per_map: i32 = unix_shm_region_per_map();
            let mut i: i32 = 0;
            { let _ = 0; };
            unsafe { sqlite3_mutex_free(unsafe { (*p).p_shm_mutex }) };
            {
                i = 0;
                '__b22: loop {
                    if !(i < unsafe { (*p).n_region } as i32) { break '__b22; }
                    '__c22: loop {
                        if unsafe { (*p).h_shm } >= 0 {
                            unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut (), u64)
                                                    ->
                                                        i32>(a_syscall[23 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(unsafe { *unsafe { (*p).ap_region.offset(i as isize) } }
                                        as *mut (), unsafe { (*p).sz_region } as u64)
                            };
                        } else {
                            unsafe {
                                sqlite3_free(unsafe {
                                            *unsafe { (*p).ap_region.offset(i as isize) }
                                        } as *mut ())
                            };
                        }
                        break '__c22;
                    }
                    i += n_shm_per_map;
                }
            }
            unsafe { sqlite3_free(unsafe { (*p).ap_region } as *mut ()) };
            if unsafe { (*p).h_shm } >= 0 {
                robust_close(p_fd_1 as *const UnixFile, unsafe { (*p).h_shm },
                    4833);
                unsafe { (*p).h_shm = -1 };
            }
            unsafe {
                (*unsafe { (*p).p_inode }).p_shm_node = core::ptr::null_mut()
            };
            unsafe { sqlite3_free(p as *mut ()) };
        }
    }
}

///* Close a connection to shared-memory.  Delete the underlying
///* storage if deleteFlag is true.
///*
///* If there is no shared memory associated with the connection then this
///* routine is a harmless no-op.
#[allow(unused_doc_comments)]
extern "C" fn unix_shm_unmap(fd: *mut Sqlite3File, delete_flag_1: i32)
    -> i32 {
    unsafe {
        let mut p: *mut UnixShm = core::ptr::null_mut();
        /// The connection to be closed
        let mut p_shm_node: *mut UnixShmNode = core::ptr::null_mut();
        /// The underlying shared-memory file
        let mut pp: *mut *mut UnixShm = core::ptr::null_mut();
        /// For looping over sibling connections
        let mut p_db_fd: *mut UnixFile = core::ptr::null_mut();

        /// The underlying database file
        (p_db_fd = fd as *mut UnixFile);

        /// The underlying database file
        (p = unsafe { (*p_db_fd).p_shm });
        if p == core::ptr::null_mut() { return 0; }
        p_shm_node = unsafe { (*p).p_shm_node };
        { let _ = 0; };
        { let _ = 0; };

        /// Remove connection p from the set of connections associated
        ///* with pShmNode
        unsafe { sqlite3_mutex_enter(unsafe { (*p_shm_node).p_shm_mutex }) };
        {
            pp = unsafe { &mut (*p_shm_node).p_first };
            '__b23: loop {
                if !(unsafe { *pp } != p) { break '__b23; }
                '__c23: loop { break '__c23; }
                pp = unsafe { &mut (*unsafe { *pp }).p_next };
            }
        }
        unsafe { *pp = unsafe { (*p).p_next } };

        /// Free the connection p
        unsafe { sqlite3_free(p as *mut ()) };
        unsafe { (*p_db_fd).p_shm = core::ptr::null_mut() };
        unsafe { sqlite3_mutex_leave(unsafe { (*p_shm_node).p_shm_mutex }) };

        /// If pShmNode->nRef has reached 0, then close the underlying
        ///* shared-memory file, too
        { let _ = 0; };
        unix_enter_mutex();
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p_shm_node).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p_shm_node).n_ref } == 0 {
            if delete_flag_1 != 0 && unsafe { (*p_shm_node).h_shm } >= 0 {
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8)
                                        ->
                                            i32>(a_syscall[16 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(unsafe { (*p_shm_node).z_filename } as *const i8)
                };
            }
            unix_shm_purge(p_db_fd);
        }
        unix_leave_mutex();
        return 0;
    }
}

///* If possible, return a pointer to a mapping of file fd starting at offset
///* iOff. The mapping must be valid for at least nAmt bytes.
///*
///* If such a pointer can be obtained, store it in *pp and return SQLITE_OK.
///* Or, if one cannot but no error occurs, set *pp to 0 and return SQLITE_OK.
///* Finally, if an error does occur, return an SQLite error code. The final
///* value of *pp is undefined in this case.
///*
///* If this function does return a pointer, the caller must eventually
///* release the reference by calling unixUnfetch().
#[allow(unused_doc_comments)]
extern "C" fn unix_fetch(fd: *mut Sqlite3File, i_off_1: i64, n_amt_1: i32,
    pp: *mut *mut ()) -> i32 {
    let p_fd: *mut UnixFile = fd as *mut UnixFile;

    /// The underlying database file
    unsafe { *pp = core::ptr::null_mut() };
    if unsafe { (*p_fd).mmap_size_max } > 0 as i64 {
        /// Ensure that there is always at least a 256 byte buffer of addressable
        ///* memory following the returned page. If the database is corrupt,
        ///* SQLite may overread the page slightly (in practice only a few bytes,
        ///* but 256 is safe, round, number).
        let n_eof_buffer: i32 = 256 as i32;
        if unsafe { (*p_fd).p_map_region } == core::ptr::null_mut() {
            let rc: i32 = unix_mapfile(p_fd, -1 as i64);
            if rc != 0 { return rc; }
        }
        if unsafe { (*p_fd).mmap_size } >=
                i_off_1 + n_amt_1 as i64 + n_eof_buffer as i64 {
            unsafe {
                *pp =
                    unsafe {
                            (unsafe { (*p_fd).p_map_region } as
                                    *mut u8).offset(i_off_1 as isize)
                        } as *mut ()
            };
            {
                let __p = unsafe { &mut (*p_fd).n_fetch_out };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return 0;
}

///* If the third argument is non-NULL, then this function releases a
///* reference obtained by an earlier call to unixFetch(). The second
///* argument passed to this function must be the same as the corresponding
///* argument that was passed to the unixFetch() invocation.
///*
///* Or, if the third argument is NULL, then this function is being called
///* to inform the VFS layer that, according to POSIX, any existing mapping
///* may now be invalid and should be unmapped.
#[allow(unused_doc_comments)]
extern "C" fn unix_unfetch(fd: *mut Sqlite3File, i_off_1: i64, p: *mut ())
    -> i32 {
    let p_fd: *mut UnixFile = fd as *mut UnixFile;

    /// The underlying database file
    { let _ = i_off_1; };

    /// The underlying database file
    /// If p==0 (unmap the entire file) then there must be no outstanding
    ///* xFetch references. Or, if p!=0 (meaning it is an xFetch reference),
    ///* then there must be at least one outstanding.
    { let _ = 0; };

    /// If p!=0, it must match the iOff value.
    { let _ = 0; };
    if !(p).is_null() {
        {
            let __p = unsafe { &mut (*p_fd).n_fetch_out };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    } else { unix_unmapfile(unsafe { &mut *p_fd }); }
    { let _ = 0; };
    return 0;
}

static nolock_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 3,
        x_close: Some(nolock_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(nolock_lock),
        x_unlock: Some(nolock_unlock),
        x_check_reserved_lock: Some(nolock_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: None,
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

///* An abstract type for a pointer to an IO method finder function:
type FinderType =
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods;

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
extern "C" fn nfs_unlock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    return posix_unlock(id, e_file_lock_1, 1);
}

static nfs_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(unix_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(unix_lock),
        x_unlock: Some(nfs_unlock),
        x_check_reserved_lock: Some(unix_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: None,
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

///* Given a file descriptor, locate the unixInodeInfo object that
///* describes that file descriptor.  Create a new one if necessary.  The
///* return value might be uninitialized if an error occurs.
///*
///* The global mutex must held when calling this routine.
///*
///* Return an appropriate error code.
#[allow(unused_doc_comments)]
extern "C" fn find_inode_info(p_file_1: *mut UnixFile,
    pp_inode_1: &mut *mut UnixInodeInfo) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        /// System call return code
        let mut fd: i32 = 0;
        /// The file descriptor for pFile
        let mut file_id: UnixFileId = unsafe { core::mem::zeroed() };
        /// Lookup key for the unixInodeInfo
        let mut statbuf: Stat = unsafe { core::mem::zeroed() };
        /// Low-level file information
        let mut p_inode: *mut UnixInodeInfo = core::ptr::null_mut();

        /// Candidate unixInodeInfo object
        { let _ = 0; };

        /// Get low-level information about the file that we can used to
        ///* create a unique name for the file.
        (fd = unsafe { (*p_file_1).h });
        rc =
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(i32, *mut Stat)
                                    ->
                                        i32>(a_syscall[5 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })(fd, &mut statbuf)
            };
        if rc != 0 {
            store_last_errno(unsafe { &mut *p_file_1 },
                unsafe { *unsafe { __error() } });
            return 10;
        }
        if statbuf.st_size == 0 as i64 &&
                unsafe { (*p_file_1).fs_flags } & 1 as u32 != 0 as u32 {
            '__b24: loop {
                '__c24: loop {
                    rc =
                        unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(i32, *const (), u64)
                                                    ->
                                                        i64>(a_syscall[11 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(fd, c"S".as_ptr() as *mut i8 as *const (), 1)
                            } as i32;
                    break '__c24;
                }
                if !(rc < 0 && unsafe { *unsafe { __error() } } == 4) {
                    break '__b24;
                }
            }
            if rc != 1 {
                store_last_errno(unsafe { &mut *p_file_1 },
                    unsafe { *unsafe { __error() } });
                return 10;
            }
            if unsafe { fsync(fd) } != 0 {
                store_last_errno(unsafe { &mut *p_file_1 },
                    unsafe { *unsafe { __error() } });
                return 10 | 4 << 8;
            }
            rc =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, *mut Stat)
                                        ->
                                            i32>(a_syscall[5 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(fd, &mut statbuf)
                };
            if rc != 0 {
                store_last_errno(unsafe { &mut *p_file_1 },
                    unsafe { *unsafe { __error() } });
                return 10;
            }
        }
        unsafe {
            memset(&raw mut file_id as *mut (), 0,
                core::mem::size_of::<UnixFileId>() as u64)
        };
        file_id.dev = statbuf.st_dev;
        file_id.ino = statbuf.st_ino as u64;
        { let _ = 0; };
        p_inode = inode_list;
        while !(p_inode).is_null() &&
                unsafe {
                        memcmp(&raw mut file_id as *const (),
                            unsafe { &raw mut (*p_inode).file_id } as *const (),
                            core::mem::size_of::<UnixFileId>() as u64)
                    } != 0 {
            p_inode = unsafe { (*p_inode).p_next };
        }
        if p_inode == core::ptr::null_mut() {
            p_inode =
                unsafe {
                        sqlite3_malloc64(core::mem::size_of::<UnixInodeInfo>() as
                                Sqlite3Uint64)
                    } as *mut UnixInodeInfo;
            if p_inode == core::ptr::null_mut() { return 7; }
            unsafe {
                memset(p_inode as *mut (), 0,
                    core::mem::size_of::<UnixInodeInfo>() as u64)
            };
            unsafe {
                memcpy(unsafe { &raw mut (*p_inode).file_id } as *mut (),
                    &raw mut file_id as *const (),
                    core::mem::size_of::<UnixFileId>() as u64)
            };
            if sqlite3Config.b_core_mutex != 0 {
                unsafe {
                    (*p_inode).p_lock_mutex = unsafe { sqlite3_mutex_alloc(0) }
                };
                if unsafe { (*p_inode).p_lock_mutex } == core::ptr::null_mut()
                    {
                    unsafe { sqlite3_free(p_inode as *mut ()) };
                    return 7;
                }
            }
            unsafe { (*p_inode).n_ref = 1 };
            { let _ = 0; };
            unsafe { (*p_inode).p_next = inode_list };
            unsafe { (*p_inode).p_prev = core::ptr::null_mut() };
            if !(inode_list).is_null() {
                unsafe { (*inode_list).p_prev = p_inode };
            }
            inode_list = p_inode;
        } else {
            {
                let __p = unsafe { &mut (*p_inode).n_ref };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        *pp_inode_1 = p_inode;
        return 0;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct AfpLockingContext {
    reserved: i32,
    db_path: *const i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ByteRangeLockPB2 {
    offset: u64,
    length: u64,
    ret_range_start: u64,
    un_lock_flag: u8,
    start_end_flag: u8,
    fd: i32,
}

///* This is a utility for setting or clearing a bit-range lock on an
///* AFP filesystem.
///*
///* Return SQLITE_OK on success, SQLITE_BUSY on failure.
extern "C" fn afp_set_lock(path: *const i8, p_file_1: *mut UnixFile,
    offset: u64, length: u64, set_lock_flag_1: i32) -> i32 {
    let mut pb: ByteRangeLockPB2 = unsafe { core::mem::zeroed() };
    let mut err: i32 = 0;
    pb.un_lock_flag = if set_lock_flag_1 != 0 { 0 } else { 1 } as u8;
    pb.start_end_flag = 0 as u8;
    pb.offset = offset;
    pb.length = length;
    pb.fd = unsafe { (*p_file_1).h };
    err =
        unsafe {
            fsctl(path,
                (2147483648u32 as Uint32T | 1073741824 as Uint32T) as u64 |
                            (core::mem::size_of::<ByteRangeLockPB2>() as u64 &
                                        8191 as u64) << 16 | (('z' as i32) << 8) as u64 | 23 as u64,
                &raw mut pb as *mut (), 0 as u32)
        };
    if err == -1 {
        let mut rc: i32 = 0;
        let t_errno: i32 = unsafe { *unsafe { __error() } };
        rc =
            sqlite_error_from_posix_error(t_errno,
                if set_lock_flag_1 != 0 {
                    10 | 15 << 8
                } else { 10 | 8 << 8 });
        if rc != 0 && rc != 5 {
            store_last_errno(unsafe { &mut *p_file_1 }, t_errno);
        }
        return rc;
    } else { return 0; }
}

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
#[allow(unused_doc_comments)]
extern "C" fn afp_unlock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let mut p_inode: *mut UnixInodeInfo = core::ptr::null_mut();
        let context: *mut AfpLockingContext =
            unsafe { (*p_file).locking_context } as *mut AfpLockingContext;
        let mut skip_shared: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_file).e_file_lock } as i32 <= e_file_lock_1 {
            return 0;
        }
        p_inode = unsafe { (*p_file).p_inode };
        unsafe { sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex }) };
        { let _ = 0; };
        if unsafe { (*p_file).e_file_lock } as i32 > 1 {
            { let _ = 0; };
            if unsafe { (*p_file).e_file_lock } as i32 == 4 {
                rc =
                    afp_set_lock(unsafe { (*context).db_path }, p_file,
                        (sqlite3_pending_byte + 2) as u64, 510 as u64, 0);
                if rc == 0 &&
                        (e_file_lock_1 == 1 || unsafe { (*p_inode).n_shared } > 1) {
                    /// only re-establish the shared lock if necessary
                    let shared_lock_byte: i32 =
                        ((sqlite3_pending_byte + 2) as u64 +
                                unsafe { (*p_inode).shared_byte }) as i32;
                    rc =
                        afp_set_lock(unsafe { (*context).db_path }, p_file,
                            shared_lock_byte as u64, 1 as u64, 1);
                } else { skip_shared = 1; }
            }
            if rc == 0 && unsafe { (*p_file).e_file_lock } as i32 >= 3 {
                rc =
                    afp_set_lock(unsafe { (*context).db_path }, p_file,
                        sqlite3_pending_byte as u64, 1 as u64, 0);
            }
            if rc == 0 && unsafe { (*p_file).e_file_lock } as i32 >= 2 &&
                    unsafe { (*context).reserved } != 0 {
                rc =
                    afp_set_lock(unsafe { (*context).db_path }, p_file,
                        (sqlite3_pending_byte + 1) as u64, 1 as u64, 0);
                if (rc == 0) as i32 != 0 {
                    unsafe { (*context).reserved = 0 };
                }
            }
            if rc == 0 &&
                    (e_file_lock_1 == 1 || unsafe { (*p_inode).n_shared } > 1) {
                unsafe { (*p_inode).e_file_lock = 1 as u8 };
            }
        }
        if rc == 0 && e_file_lock_1 == 0 {
            /// Decrement the shared lock counter.  Release the lock using an
            ///* OS call only when all threads in this same process have released
            ///* the lock.
            let shared_lock_byte_1: u64 =
                (sqlite3_pending_byte + 2) as u64 +
                    unsafe { (*p_inode).shared_byte };
            {
                let __p = unsafe { &mut (*p_inode).n_shared };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if unsafe { (*p_inode).n_shared } == 0 {
                if (skip_shared == 0) as i32 != 0 {
                    rc =
                        afp_set_lock(unsafe { (*context).db_path }, p_file,
                            shared_lock_byte_1, 1 as u64, 0);
                }
                if (rc == 0) as i32 != 0 {
                    unsafe { (*p_inode).e_file_lock = 0 as u8 };
                    unsafe { (*p_file).e_file_lock = 0 as u8 };
                }
            }
            if rc == 0 {
                {
                    let __p = unsafe { &mut (*p_inode).n_lock };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
                { let _ = 0; };
                if unsafe { (*p_inode).n_lock } == 0 {
                    close_pending_fds(p_file);
                }
            }
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex }) };
        if rc == 0 { unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 }; }
        return rc;
    }
}

///* Close a file & cleanup AFP specific locking context
#[allow(unused_doc_comments)]
extern "C" fn afp_close(id: *mut Sqlite3File) -> i32 {
    let mut rc: i32 = 0;
    let p_file: *mut UnixFile = id as *mut UnixFile;
    { let _ = 0; };
    afp_unlock(id, 0);
    { let _ = 0; };
    unix_enter_mutex();
    if !(unsafe { (*p_file).p_inode }).is_null() {
        let p_inode: *const UnixInodeInfo =
            unsafe { (*p_file).p_inode } as *const UnixInodeInfo;
        unsafe { sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex }) };
        if unsafe { (*p_inode).n_lock } != 0 {

            /// If there are outstanding locks, do not actually close the file just
            ///* yet because that would clear those locks.  Instead, add the file
            ///* descriptor to pInode->aPending.  It will be automatically closed when
            ///* the last lock is cleared.
            set_pending_fd(unsafe { &mut *p_file });
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex }) };
    }
    release_inode_info(p_file);
    unsafe { sqlite3_free(unsafe { (*p_file).locking_context }) };
    rc = close_unix_file(id);
    unix_leave_mutex();
    return rc;
}

///* Lock the file with the lock specified by parameter eFileLock - one
///* of the following:
///*
///*     (1) SHARED_LOCK
///*     (2) RESERVED_LOCK
///*     (3) PENDING_LOCK
///*     (4) EXCLUSIVE_LOCK
///*
///* Sometimes when requesting one lock state, additional lock states
///* are inserted in between.  The locking might fail on one of the later
///* transitions leaving the lock state different from what it started but
///* still short of its goal.  The following chart shows the allowed
///* transitions and the inserted intermediate states:
///*
///*    UNLOCKED -> SHARED
///*    SHARED -> RESERVED
///*    SHARED -> (PENDING) -> EXCLUSIVE
///*    RESERVED -> (PENDING) -> EXCLUSIVE
///*    PENDING -> EXCLUSIVE
///*
///* This routine will only increase a lock.  Use the sqlite3OsUnlock()
///* routine to lower a locking level.
#[allow(unused_doc_comments)]
extern "C" fn afp_lock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let mut p_inode: *mut UnixInodeInfo = unsafe { (*p_file).p_inode };
        '__b26: loop {
            '__c26: loop {
                let context: *mut AfpLockingContext =
                    unsafe { (*p_file).locking_context } as
                        *mut AfpLockingContext;
                { let _ = 0; };
                if unsafe { (*p_file).e_file_lock } as i32 >= e_file_lock_1 {
                    return 0;
                }

                /// Make sure the locking sequence is correct
                ///*  (1) We never move from unlocked to anything higher than shared lock.
                ///*  (2) SQLite never explicitly requests a pending lock.
                ///*  (3) A shared lock is always held when a reserve lock is requested.
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };

                /// This mutex is needed because pFile->pInode is shared across threads
                (p_inode = unsafe { (*p_file).p_inode });
                unsafe {
                    sqlite3_mutex_enter(unsafe { (*p_inode).p_lock_mutex })
                };
                if unsafe { (*p_file).e_file_lock } as i32 !=
                            unsafe { (*p_inode).e_file_lock } as i32 &&
                        (unsafe { (*p_inode).e_file_lock } as i32 >= 3 ||
                            e_file_lock_1 > 1) {
                    rc = 5;
                    break '__b26;
                }
                if e_file_lock_1 == 1 &&
                        (unsafe { (*p_inode).e_file_lock } as i32 == 1 ||
                            unsafe { (*p_inode).e_file_lock } as i32 == 2) {
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe { (*p_file).e_file_lock = 1 as u8 };
                    {
                        let __p = unsafe { &mut (*p_inode).n_shared };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    {
                        let __p = unsafe { &mut (*p_inode).n_lock };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    break '__b26;
                }
                if e_file_lock_1 == 1 ||
                        e_file_lock_1 == 4 &&
                            (unsafe { (*p_file).e_file_lock } as i32) < 3 {
                    let mut failed: i32 = 0;
                    failed =
                        afp_set_lock(unsafe { (*context).db_path }, p_file,
                            sqlite3_pending_byte as u64, 1 as u64, 1);
                    if failed != 0 { rc = failed; break '__b26; }
                }
                if e_file_lock_1 == 1 {
                    let mut lrc1: i32 = 0;
                    let mut lrc2: i32 = 0;
                    let mut lrc1_errno: i32 = 0;
                    let mut lk: i64 = 0 as i64;
                    let mut mask: i64 = 0 as i64;
                    { let _ = 0; };
                    { let _ = 0; };
                    mask =
                        if core::mem::size_of::<i64>() as u64 == 8 as u64 {
                                4294967295u32 as i64 | (2147483647 as i64) << 32
                            } else { 2147483647 as i64 } as i64;

                    /// Now get the read-lock SHARED_LOCK */
                    ///    /* note that the quality of the randomness doesn't matter that much
                    (lk = unsafe { random() });
                    unsafe {
                        (*p_inode).shared_byte =
                            ((lk & mask) % (510 - 1) as i64) as u64
                    };
                    lrc1 =
                        afp_set_lock(unsafe { (*context).db_path }, p_file,
                            (sqlite3_pending_byte + 2) as u64 +
                                unsafe { (*p_inode).shared_byte }, 1 as u64, 1);
                    if lrc1 != 0 && lrc1 != 5 {
                        lrc1_errno = unsafe { (*p_file).last_errno };
                    }

                    /// Drop the temporary PENDING lock
                    (lrc2 =
                        afp_set_lock(unsafe { (*context).db_path }, p_file,
                            sqlite3_pending_byte as u64, 1 as u64, 0));
                    if lrc1 != 0 && lrc1 != 5 {
                        store_last_errno(unsafe { &mut *p_file }, lrc1_errno);
                        rc = lrc1;
                        break '__b26;
                    } else if lrc2 != 0 && lrc2 != 5 {
                        rc = lrc2;
                        break '__b26;
                    } else if lrc1 != 0 {
                        rc = lrc1;
                    } else {
                        unsafe { (*p_file).e_file_lock = 1 as u8 };
                        {
                            let __p = unsafe { &mut (*p_inode).n_lock };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe { (*p_inode).n_shared = 1 };
                    }
                } else if e_file_lock_1 == 4 &&
                        unsafe { (*p_inode).n_shared } > 1 {

                    /// We are trying for an exclusive lock but another thread in this
                    ///* same process is still holding a shared lock.
                    (rc = 5);
                } else {
                    /// The request was for a RESERVED or EXCLUSIVE lock.  It is
                    ///* assumed that there is a SHARED or greater lock on the file
                    ///* already.
                    let mut failed: i32 = 0;
                    { let _ = 0; };
                    if e_file_lock_1 >= 2 &&
                            (unsafe { (*p_file).e_file_lock } as i32) < 2 {

                        /// Acquire a RESERVED lock
                        (failed =
                            afp_set_lock(unsafe { (*context).db_path }, p_file,
                                (sqlite3_pending_byte + 1) as u64, 1 as u64, 1));
                        if (failed == 0) as i32 != 0 {
                            unsafe { (*context).reserved = 1 };
                        }
                    }
                    if (failed == 0) as i32 != 0 && e_file_lock_1 == 4 {
                        if ({
                                            failed =
                                                afp_set_lock(unsafe { (*context).db_path }, p_file,
                                                    (sqlite3_pending_byte + 2) as u64 +
                                                        unsafe { (*p_inode).shared_byte }, 1 as u64, 0);
                                            failed
                                        } == 0) as i32 != 0 {
                            let mut failed2: i32 = 0;

                            /// now attempt to get the exclusive lock range
                            (failed =
                                afp_set_lock(unsafe { (*context).db_path }, p_file,
                                    (sqlite3_pending_byte + 2) as u64, 510 as u64, 1));
                            if failed != 0 &&
                                    {
                                            failed2 =
                                                afp_set_lock(unsafe { (*context).db_path }, p_file,
                                                    (sqlite3_pending_byte + 2) as u64 +
                                                        unsafe { (*p_inode).shared_byte }, 1 as u64, 1);
                                            failed2
                                        } != 0 {

                                /// Can't reestablish the shared lock.  Sqlite can't deal, this is
                                ///* a critical I/O error
                                (rc =
                                    if failed & 255 == 10 { failed2 } else { 10 | 15 << 8 });
                                break '__b26;
                            }
                        } else { rc = failed; }
                    }
                    if failed != 0 { rc = failed; }
                }
                if rc == 0 {
                    unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };
                    unsafe { (*p_inode).e_file_lock = e_file_lock_1 as u8 };
                } else if e_file_lock_1 == 4 {
                    unsafe { (*p_file).e_file_lock = 3 as u8 };
                    unsafe { (*p_inode).e_file_lock = 3 as u8 };
                }
                break '__c26;
            }
            if !(false) { break '__b26; }
        }

        /// If there is already a lock of this type or more restrictive on the
        ///* unixFile, do nothing. Don't use the afp_end_lock: exit path, as
        ///* unixEnterMutex() hasn't been called yet.
        /// Make sure the locking sequence is correct
        ///*  (1) We never move from unlocked to anything higher than shared lock.
        ///*  (2) SQLite never explicitly requests a pending lock.
        ///*  (3) A shared lock is always held when a reserve lock is requested.
        /// This mutex is needed because pFile->pInode is shared across threads
        /// If some thread using this PID has a lock via a different unixFile*
        ///* handle that precludes the requested lock, return BUSY.
        /// If a SHARED lock is requested, and some thread using this PID already
        ///* has a SHARED or RESERVED lock, then increment reference counts and
        ///* return SQLITE_OK.
        /// A PENDING lock is needed before acquiring a SHARED lock and before
        ///* acquiring an EXCLUSIVE lock.  For the SHARED lock, the PENDING will
        ///* be released.
        /// If control gets to this point, then actually go ahead and make
        ///* operating system calls for the specified lock.
        /// Now get the read-lock SHARED_LOCK */
        ///    /* note that the quality of the randomness doesn't matter that much
        /// Drop the temporary PENDING lock
        /// We are trying for an exclusive lock but another thread in this
        ///* same process is still holding a shared lock.
        /// The request was for a RESERVED or EXCLUSIVE lock.  It is
        ///* assumed that there is a SHARED or greater lock on the file
        ///* already.
        /// Acquire a RESERVED lock
        /// Acquire an EXCLUSIVE lock
        /// Remove the shared lock before trying the range.  we'll need to
        ///* reestablish the shared lock if we can't get the  afpUnlock
        /// now attempt to get the exclusive lock range
        /// Can't reestablish the shared lock.  Sqlite can't deal, this is
        ///* a critical I/O error
        unsafe { sqlite3_mutex_leave(unsafe { (*p_inode).p_lock_mutex }) };
        return rc;
    }
}

///* This routine checks if there is a RESERVED lock held on the specified
///* file by this or any other process. If such a lock is held, set *pResOut
///* to a non-zero value otherwise *pResOut is set to zero.  The return value
///* is set to SQLITE_OK unless an I/O error occurs during lock checking.
#[allow(unused_doc_comments)]
extern "C" fn afp_check_reserved_lock(id: *mut Sqlite3File,
    p_res_out_1: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut reserved: i32 = 0;
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let mut context: *const AfpLockingContext = core::ptr::null();
        { let _ = 0; };
        context =
            unsafe { (*p_file).locking_context } as *mut AfpLockingContext;
        if unsafe { (*context).reserved } != 0 {
            unsafe { *p_res_out_1 = 1 };
            return 0;
        }
        unsafe {
            sqlite3_mutex_enter(unsafe {
                    (*unsafe { (*p_file).p_inode }).p_lock_mutex
                })
        };
        if unsafe { (*unsafe { (*p_file).p_inode }).e_file_lock } as i32 > 1 {
            reserved = 1;
        }
        if (reserved == 0) as i32 != 0 {
            /// lock the RESERVED byte
            let mut lrc: i32 =
                afp_set_lock(unsafe { (*context).db_path }, p_file,
                    (sqlite3_pending_byte + 1) as u64, 1 as u64, 1);
            if 0 == lrc {

                /// if we succeeded in taking the reserved lock, unlock it to restore
                ///* the original state
                (lrc =
                    afp_set_lock(unsafe { (*context).db_path }, p_file,
                        (sqlite3_pending_byte + 1) as u64, 1 as u64, 0));
            } else {

                /// if we failed to get the lock then someone else must have it
                (reserved = 1);
            }
            if lrc != 0 && lrc != 5 { rc = lrc; }
        }
        unsafe {
            sqlite3_mutex_leave(unsafe {
                    (*unsafe { (*p_file).p_inode }).p_lock_mutex
                })
        };
        unsafe { *p_res_out_1 = reserved };
        return rc;
    }
}

static afp_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(afp_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(afp_lock),
        x_unlock: Some(afp_unlock),
        x_check_reserved_lock: Some(afp_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: None,
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
///*
///* When the locking level reaches NO_LOCK, delete the lock file.
#[allow(unused_doc_comments)]
extern "C" fn dotlock_unlock(id: *mut Sqlite3File, e_file_lock_1: i32)
    -> i32 {
    unsafe {
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let z_lock_file: *mut i8 =
            unsafe { (*p_file).locking_context } as *mut i8;
        let mut rc: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_file).e_file_lock } as i32 == e_file_lock_1 {
            return 0;
        }
        if e_file_lock_1 == 1 {
            unsafe { (*p_file).e_file_lock = 1 as u8 };
            return 0;
        }

        /// To fully unlock the database, delete the lock file
        { let _ = 0; };
        rc =
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*const i8)
                                    ->
                                        i32>(a_syscall[19 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })(z_lock_file as *const i8)
            };
        if rc < 0 {
            let t_errno: i32 = unsafe { *unsafe { __error() } };
            if t_errno == 2 {
                rc = 0;
            } else {
                rc = 10 | 8 << 8;
                store_last_errno(unsafe { &mut *p_file }, t_errno);
            }
            return rc;
        }
        unsafe { (*p_file).e_file_lock = 0 as u8 };
        return 0;
    }
}

///* Close a file.  Make sure the lock has been released before closing.
extern "C" fn dotlock_close(id: *mut Sqlite3File) -> i32 {
    let p_file: *const UnixFile = id as *mut UnixFile as *const UnixFile;
    { let _ = 0; };
    dotlock_unlock(id, 0);
    unsafe { sqlite3_free(unsafe { (*p_file).locking_context }) };
    return close_unix_file(id);
}

///* Lock the file with the lock specified by parameter eFileLock - one
///* of the following:
///*
///*     (1) SHARED_LOCK
///*     (2) RESERVED_LOCK
///*     (3) PENDING_LOCK
///*     (4) EXCLUSIVE_LOCK
///*
///* Sometimes when requesting one lock state, additional lock states
///* are inserted in between.  The locking might fail on one of the later
///* transitions leaving the lock state different from what it started but
///* still short of its goal.  The following chart shows the allowed
///* transitions and the inserted intermediate states:
///*
///*    UNLOCKED -> SHARED
///*    SHARED -> RESERVED
///*    SHARED -> (PENDING) -> EXCLUSIVE
///*    RESERVED -> (PENDING) -> EXCLUSIVE
///*    PENDING -> EXCLUSIVE
///*
///* This routine will only increase a lock.  Use the sqlite3OsUnlock()
///* routine to lower a locking level.
///*
///* With dotfile locking, we really only support state (4): EXCLUSIVE.
///* But we track the other locking levels internally.
#[allow(unused_doc_comments)]
extern "C" fn dotlock_lock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    unsafe {
        let p_file: *mut UnixFile = id as *mut UnixFile;
        let z_lock_file: *mut i8 =
            unsafe { (*p_file).locking_context } as *mut i8;
        let mut rc: i32 = 0;
        if unsafe { (*p_file).e_file_lock } as i32 > 0 {
            unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };

            /// Always update the timestamp on the old file
            unsafe {
                utime(z_lock_file as *const i8,
                    0 as *mut () as *const Utimbuf)
            };
            return 0;
        }

        /// grab an exclusive lock
        (rc =
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*const i8, u16)
                                    ->
                                        i32>(a_syscall[18 as
                                                usize].p_current.unwrap_or(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    }) as *const ())
                    })(z_lock_file as *const i8, 511)
            });
        if rc < 0 {
            /// failed to open/create the lock directory
            let t_errno: i32 = unsafe { *unsafe { __error() } };
            if 17 == t_errno {
                rc = 5;
            } else {
                rc = sqlite_error_from_posix_error(t_errno, 10 | 15 << 8);
                if rc != 5 {
                    store_last_errno(unsafe { &mut *p_file }, t_errno);
                }
            }
            return rc;
        }

        /// got it, set the type and return ok
        unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };
        return rc;
    }
}

///* This routine checks if there is a RESERVED lock held on the specified
///* file by this or any other process. If the caller holds a SHARED
///* or greater lock when it is called, then it is assumed that no other
///* client may hold RESERVED. Or, if the caller holds no lock, then it
///* is assumed another client holds RESERVED if the lock-file exists.
extern "C" fn dotlock_check_reserved_lock(id: *mut Sqlite3File,
    p_res_out_1: *mut i32) -> i32 {
    unsafe {
        let p_file: *const UnixFile = id as *mut UnixFile as *const UnixFile;
        if unsafe { (*p_file).e_file_lock } as i32 >= 1 {
            unsafe { *p_res_out_1 = 0 };
        } else {
            unsafe {
                *p_res_out_1 =
                    (unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*const i8, i32)
                                                    ->
                                                        i32>(a_syscall[2 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(unsafe { (*p_file).locking_context } as *const i8, 0)
                            } == 0) as i32
            };
        }
        return 0;
    }
}

static dotlock_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(dotlock_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(dotlock_lock),
        x_unlock: Some(dotlock_unlock),
        x_check_reserved_lock: Some(dotlock_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: None,
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

///* Initialize the contents of the unixFile structure pointed to by pId.
#[allow(unused_doc_comments)]
extern "C" fn fill_in_unix_file(p_vfs_1: *mut Sqlite3Vfs, mut h: i32,
    p_id_1: *mut Sqlite3File, z_filename_1: *const i8, ctrl_flags_1: i32)
    -> i32 {
    unsafe {
        let mut p_locking_style: *const Sqlite3IoMethods = core::ptr::null();
        let p_new: *mut UnixFile = p_id_1 as *mut UnixFile;
        let mut rc: i32 = 0;
        { let _ = 0; };

        /// No locking occurs in temporary files
        { let _ = 0; };
        unsafe { (*p_new).h = h };
        unsafe { (*p_new).p_vfs = p_vfs_1 };
        unsafe { (*p_new).z_path = z_filename_1 };
        unsafe { (*p_new).ctrl_flags = ctrl_flags_1 as u8 as u16 };
        unsafe { (*p_new).mmap_size_max = sqlite3Config.sz_mmap };
        if unsafe {
                    sqlite3_uri_boolean(if ctrl_flags_1 & 64 != 0 {
                            z_filename_1
                        } else { core::ptr::null() },
                        c"psow".as_ptr() as *mut i8 as *const i8, 1)
                } != 0 {
            unsafe { (*p_new).ctrl_flags |= 16 as u16 };
        }
        if unsafe {
                    strcmp(unsafe { (*p_vfs_1).z_name },
                        c"unix-excl".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            unsafe { (*p_new).ctrl_flags |= 1 as u16 };
        }
        if ctrl_flags_1 & 128 != 0 {
            p_locking_style = &nolock_io_methods;
        } else {
            p_locking_style =
                unsafe {
                    (unsafe {
                            *unsafe {
                                    core::mem::transmute::<*const (),
                                            *mut unsafe extern "C" fn(*const i8, *mut UnixFile)
                                                ->
                                                    *const Sqlite3IoMethods>(unsafe { (*p_vfs_1).p_app_data } as
                                            *const ())
                                }
                        })(z_filename_1, p_new)
                };

            /// Cache zFilename in the locking context (AFP and dotlock override) for
            ///* proxyLock activation is possible (remote proxy is based on db name)
            ///* zFilename remains valid until file is closed, to support
            unsafe { (*p_new).locking_context = z_filename_1 as *mut () };
        }
        if p_locking_style ==
                    &raw const posix_io_methods as *const Sqlite3IoMethods ||
                p_locking_style ==
                    &raw const nfs_io_methods as *const Sqlite3IoMethods {
            unix_enter_mutex();
            rc = find_inode_info(p_new, unsafe { &mut (*p_new).p_inode });
            if rc != 0 {

                /// If an error occurred in findInodeInfo(), close the file descriptor
                ///* immediately, before releasing the mutex. findInodeInfo() may fail
                ///* in two scenarios:
                ///*
                ///*   (a) A call to fstat() failed.
                ///*   (b) A malloc failed.
                ///*
                ///* Scenario (b) may only occur if the process is holding no other
                ///* file descriptors open on the same file. If there were other file
                ///* descriptors on this file, then no malloc would be required by
                ///* findInodeInfo(). If this is the case, it is quite safe to close
                ///* handle h - as it is guaranteed that no posix locks will be released
                ///* by doing so.
                ///*
                ///* If scenario (a) caused the error then things are not so safe. The
                ///* implicit assumption here is that if fstat() fails, things are in
                ///* such bad shape that dropping a lock or two doesn't matter much.
                robust_close(p_new as *const UnixFile, h, 6158);
                h = -1;
            }
            unix_leave_mutex();
        } else if p_locking_style ==
                &raw const afp_io_methods as *const Sqlite3IoMethods {
            /// AFP locking uses the file path so it needs to be included in
            ///* the afpLockingContext.
            let mut p_ctx: *mut AfpLockingContext = core::ptr::null_mut();
            unsafe {
                (*p_new).locking_context =
                    {
                            p_ctx =
                                unsafe {
                                        sqlite3_malloc64(core::mem::size_of::<AfpLockingContext>()
                                                as Sqlite3Uint64)
                                    } as *mut AfpLockingContext;
                            p_ctx
                        } as *mut ()
            };
            if p_ctx == core::ptr::null_mut() {
                rc = 7;
            } else {

                /// NB: zFilename exists and remains valid until the file is closed
                ///* according to requirement F11141.  So we do not need to make a
                ///* copy of the filename.
                unsafe { (*p_ctx).db_path = z_filename_1 };
                unsafe { (*p_ctx).reserved = 0 };
                unsafe { srandomdev() };
                unix_enter_mutex();
                rc = find_inode_info(p_new, unsafe { &mut (*p_new).p_inode });
                if rc != 0 {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).locking_context })
                    };
                    robust_close(p_new as *const UnixFile, h, 6184);
                    h = -1;
                }
                unix_leave_mutex();
            }
        } else if p_locking_style ==
                &raw const dotlock_io_methods as *const Sqlite3IoMethods {
            /// Dotfile locking uses the file path so it needs to be included in
            ///* the dotlockLockingContext
            let mut z_lock_file: *mut i8 = core::ptr::null_mut();
            let mut n_filename: i32 = 0;
            { let _ = 0; };
            n_filename = unsafe { strlen(z_filename_1) } as i32 + 6;
            z_lock_file =
                unsafe { sqlite3_malloc64(n_filename as Sqlite3Uint64) } as
                    *mut i8;
            if z_lock_file == core::ptr::null_mut() {
                rc = 7;
            } else {
                unsafe {
                    sqlite3_snprintf(n_filename, z_lock_file,
                        c"%s.lock".as_ptr() as *mut i8 as *const i8, z_filename_1)
                };
            }
            unsafe { (*p_new).locking_context = z_lock_file as *mut () };
        }
        store_last_errno(unsafe { &mut *p_new }, 0);
        if rc != 0 {
            if h >= 0 { robust_close(p_new as *const UnixFile, h, 6250); }
        } else {
            unsafe { (*p_id_1).p_methods = p_locking_style };
            verify_db_file(p_new);
        }
        return rc;
    }
}

///* Create a new VFS file descriptor (stored in memory obtained from
///* sqlite3_malloc) and open the file named "path" in the file descriptor.
///*
///* The caller is responsible not only for closing the file descriptor
///* but also for freeing the memory associated with the file descriptor.
#[allow(unused_doc_comments)]
extern "C" fn proxy_create_unix_file(path: *const i8,
    pp_file_1: &mut *mut UnixFile, islockfile: i32) -> i32 {
    let mut fd: i32 = 0;
    let mut p_new: *mut UnixFile = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut open_flags: i32 = 0;
    let mut dummy_vfs: Sqlite3Vfs = unsafe { core::mem::zeroed() };
    let mut terrno: i32 = 0;
    let mut p_unused: *mut UnixUnusedFd = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s28:
            {
            match __state {
                0 => { fd = -1; __state = 3; }
                2 => {
                    robust_close(p_new as *const UnixFile, fd, 7604);
                    __state = 52;
                }
                3 => { __state = 4; }
                4 => { rc = 0; __state = 5; }
                5 => { open_flags = 2 | 512 | 256; __state = 6; }
                6 => { __state = 7; }
                7 => { terrno = 0; __state = 8; }
                8 => {
                    p_unused = 0 as *mut () as *mut UnixUnusedFd;
                    __state = 9;
                }
                9 => {
                    p_unused = find_reusable_fd(path, open_flags);
                    __state = 10;
                }
                10 => {
                    if !(p_unused).is_null() {
                        __state = 12;
                    } else { __state = 13; }
                }
                11 => { if fd < 0 { __state = 17; } else { __state = 16; } }
                12 => { fd = unsafe { (*p_unused).fd }; __state = 11; }
                13 => {
                    p_unused =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<UnixUnusedFd>() as
                                        Sqlite3Uint64)
                            } as *mut UnixUnusedFd;
                    __state = 14;
                }
                14 => {
                    if (p_unused).is_null() as i32 != 0 {
                        __state = 15;
                    } else { __state = 11; }
                }
                15 => { return 7; }
                16 => { if fd < 0 { __state = 23; } else { __state = 22; } }
                17 => {
                    fd = robust_open(path, open_flags, 0 as ModeT);
                    __state = 18;
                }
                18 => {
                    terrno = unsafe { *unsafe { __error() } };
                    __state = 19;
                }
                19 => {
                    if fd < 0 && unsafe { *unsafe { __error() } } == 2 &&
                            islockfile != 0 {
                        __state = 20;
                    } else { __state = 16; }
                }
                20 => {
                    if proxy_create_lock_path(path) == 0 {
                        __state = 21;
                    } else { __state = 16; }
                }
                21 => {
                    fd = robust_open(path, open_flags, 0 as ModeT);
                    __state = 16;
                }
                22 => { if fd < 0 { __state = 27; } else { __state = 26; } }
                23 => { open_flags = 0 | 256; __state = 24; }
                24 => {
                    fd = robust_open(path, open_flags, 0 as ModeT);
                    __state = 25;
                }
                25 => {
                    terrno = unsafe { *unsafe { __error() } };
                    __state = 22;
                }
                26 => {
                    p_new =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<UnixFile>() as
                                        Sqlite3Uint64)
                            } as *mut UnixFile;
                    __state = 36;
                }
                27 => {
                    if islockfile != 0 { __state = 29; } else { __state = 28; }
                }
                28 => {
                    '__s29:
                        {
                        match terrno {
                            13 => { __state = 30; }
                            5 => { __state = 31; }
                            _ => { __state = 32; }
                        }
                    }
                }
                29 => { return 5; }
                30 => { return 3; }
                31 => { return 10 | 15 << 8; }
                32 => { return unsafe { sqlite3_cantopen_error(7580) }; }
                33 => { __state = 30; }
                34 => { __state = 31; }
                35 => { __state = 32; }
                36 => {
                    if p_new as *mut () == 0 as *mut () {
                        __state = 38;
                    } else { __state = 37; }
                }
                37 => {
                    unsafe {
                        memset(p_new as *mut (), 0,
                            core::mem::size_of::<UnixFile>() as u64)
                    };
                    __state = 40;
                }
                38 => { rc = 7; __state = 39; }
                39 => { __state = 2; }
                40 => {
                    unsafe { (*p_new).open_flags = open_flags };
                    __state = 41;
                }
                41 => {
                    unsafe {
                        memset(&raw mut dummy_vfs as *mut (), 0,
                            core::mem::size_of::<Sqlite3Vfs>() as u64)
                    };
                    __state = 42;
                }
                42 => {
                    dummy_vfs.p_app_data =
                        &raw const autolock_io_finder as *mut ();
                    __state = 43;
                }
                43 => {
                    dummy_vfs.z_name =
                        c"dummy".as_ptr() as *mut i8 as *const i8;
                    __state = 44;
                }
                44 => { unsafe { (*p_unused).fd = fd }; __state = 45; }
                45 => {
                    unsafe { (*p_unused).flags = open_flags };
                    __state = 46;
                }
                46 => {
                    unsafe { (*p_new).p_preallocated_unused = p_unused };
                    __state = 47;
                }
                47 => {
                    rc =
                        fill_in_unix_file(&mut dummy_vfs, fd,
                            p_new as *mut Sqlite3File, path, 0);
                    __state = 48;
                }
                48 => { if rc == 0 { __state = 50; } else { __state = 49; } }
                49 => { __state = 2; }
                50 => { *pp_file_1 = p_new; __state = 51; }
                51 => { return 0; }
                52 => {
                    unsafe { sqlite3_free(p_new as *mut ()) };
                    __state = 53;
                }
                53 => {
                    unsafe { sqlite3_free(p_unused as *mut ()) };
                    __state = 54;
                }
                54 => { return rc; }
                _ => {}
            }
        }
    }

    /// 1. first try to open/create the file
    ///* 2. if that fails, and this is a lock file (not-conch), try creating
    ///* the parent directories and then try again.
    ///* 3. if that fails, try to open the file read-only
    ///* otherwise return BUSY (if lock file) or CANTOPEN for the conch file
    /// even though it is the conch
    unreachable!();
}

/// Takes the conch by taking a shared lock and read the contents conch, if
///* lockPath is non-NULL, the host ID and lock file path must match.  A NULL
///* lockPath means that the lockPath in the conch file will be used if the
///* host IDs match, or a new lock path will be generated automatically
///* and written to the conch file.
#[allow(unused_doc_comments)]
extern "C" fn proxy_take_conch(p_file_1: *mut UnixFile) -> i32 {
    unsafe {
        let p_ctx: *mut ProxyLockingContext =
            unsafe { (*p_file_1).locking_context } as
                *mut ProxyLockingContext;
        if unsafe { (*p_ctx).conch_held } != 0 {
            return 0;
        } else {
            let mut conch_file: *mut UnixFile = core::ptr::null_mut();
            let mut my_host_id: UuidT = [0; 16];
            let mut p_error: i32 = 0;
            let mut read_buf: [i8; 1041] = [0; 1041];
            let mut lock_path: [i8; 1024] = [0; 1024];
            let mut temp_lock_path: *mut i8 = core::ptr::null_mut();
            let mut rc: i32 = 0;
            let mut create_conch: i32 = 0;
            let mut host_id_match: i32 = 0;
            let mut read_len: i32 = 0;
            let mut try_old_lock_path: i32 = 0;
            let mut force_new_lock_path: i32 = 0;
            /// read the existing conch file
            /// I/O error: lastErrno set by seekAndRead
            /// a short read or version format mismatch means we need to create a new
            ///* conch file.
            /// if the host id matches and the lock path already exists in the conch
            ///* we'll try to use the path there, if we can't open that path, we'll
            ///* retry with a new auto-generated path
            /// in case we need to try again for an :auto: named lock file
            /// if the conch has data compare the contents
            /// for auto-named local lock file, just check the host ID and we'll
            ///* use the local lock file path that's already in there
            let mut path_len: u64 = 0 as u64;
            /// create a copy of the lock path if the conch is taken
            /// conch host and lock path match
            /// if the conch isn't writable and doesn't match, we can't take it
            /// either the conch didn't match or we need to create a new one
            /// create a copy of the lock path _only_ if the conch is taken
            /// update conch with host and path (this will fail if other process
            ///* has a shared lock already), if the host id matches, use the big
            ///* stick.
            /// We are trying for an exclusive lock but another thread in this
            ///* same process is still holding a shared lock.
            let mut write_buffer: [i8; 1041] = [0; 1041];
            let mut write_size: i32 = 0;
            /// If we created a new conch file (not just updated the contents of a
            ///* valid conch file), try to match the permissions of the database
            let mut buf: Stat = unsafe { core::mem::zeroed() };
            let mut err: i32 = 0;
            let mut cmode: ModeT = 0 as ModeT;
            /// try to match the database file R/W permissions, ignore failure
            let mut fd: i32 = 0;
            /// SQLITE_BUSY? proxyTakeConch called
            ///           during locking
            let mut path: *const i8 = core::ptr::null();
            /// we couldn't create the proxy lock file with the old lock file path
            ///* so try again via auto-naming
            /// go back to the do {} while start point, try again
            /// Need to make a copy of path if we extracted the value
            ///* from the conch file or the path was allocated on the stack
            let mut afp_ctx: *mut AfpLockingContext = core::ptr::null_mut();
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s31:
                    {
                    match __state {
                        0 => {
                            conch_file = unsafe { (*p_ctx).conch_file };
                            __state = 3;
                        }
                        2 => { __state = 76; }
                        3 => { __state = 4; }
                        4 => { p_error = 0; __state = 5; }
                        5 => { __state = 6; }
                        6 => { __state = 7; }
                        7 => {
                            temp_lock_path = 0 as *mut () as *mut i8;
                            __state = 8;
                        }
                        8 => { rc = 0; __state = 9; }
                        9 => { create_conch = 0; __state = 10; }
                        10 => { host_id_match = 0; __state = 11; }
                        11 => { read_len = 0; __state = 12; }
                        12 => { try_old_lock_path = 0; __state = 13; }
                        13 => { force_new_lock_path = 0; __state = 14; }
                        14 => { __state = 15; }
                        15 => {
                            rc =
                                proxy_get_host_id(&raw mut my_host_id[0 as usize] as
                                        *mut u8, &mut p_error);
                            __state = 16;
                        }
                        16 => {
                            if rc & 255 == 10 { __state = 18; } else { __state = 17; }
                        }
                        17 => {
                            rc =
                                proxy_conch_lock(p_file_1,
                                    &raw mut my_host_id[0 as usize] as *mut u8, 1);
                            __state = 20;
                        }
                        18 => {
                            store_last_errno(unsafe { &mut *p_file_1 }, p_error);
                            __state = 19;
                        }
                        19 => { __state = 2; }
                        20 => {
                            if rc != 0 { __state = 22; } else { __state = 21; }
                        }
                        21 => {
                            read_len =
                                seek_and_read(conch_file as *mut UnixFile,
                                    0 as Sqlite3Int64,
                                    &raw mut read_buf[0 as usize] as *mut i8 as *mut (),
                                    1 + 16 + 1024);
                            __state = 23;
                        }
                        22 => { __state = 2; }
                        23 => {
                            if read_len < 0 { __state = 25; } else { __state = 26; }
                        }
                        24 => {
                            if (create_conch == 0) as i32 != 0 &&
                                    (force_new_lock_path == 0) as i32 != 0 {
                                __state = 32;
                            } else { __state = 31; }
                        }
                        25 => {
                            store_last_errno(unsafe { &mut *p_file_1 },
                                unsafe { (*conch_file).last_errno });
                            __state = 27;
                        }
                        26 => {
                            if read_len <= 1 + 16 ||
                                    read_buf[0 as usize] as i32 != 2 as i8 as i32 {
                                __state = 29;
                            } else { __state = 24; }
                        }
                        27 => { rc = 10 | 1 << 8; __state = 28; }
                        28 => { __state = 2; }
                        29 => { create_conch = 1; __state = 24; }
                        30 => { if 1 != 0 { __state = 24; } else { __state = 1; } }
                        31 => {
                            if unsafe { (*conch_file).open_flags } & 2 == 0 {
                                __state = 46;
                            } else { __state = 45; }
                        }
                        32 => {
                            host_id_match =
                                (unsafe {
                                                memcmp(&raw mut read_buf[1 as usize] as *const (),
                                                    &raw mut my_host_id[0 as usize] as *mut u8 as *const (),
                                                    16 as u64)
                                            } == 0) as i32 as i32;
                            __state = 33;
                        }
                        33 => {
                            if (unsafe { (*p_ctx).lock_proxy_path }).is_null() as i32 !=
                                    0 {
                                __state = 34;
                            } else { __state = 35; }
                        }
                        34 => {
                            if host_id_match != 0 {
                                __state = 36;
                            } else { __state = 31; }
                        }
                        35 => {
                            if host_id_match != 0 &&
                                    (unsafe {
                                                    strncmp(unsafe { (*p_ctx).lock_proxy_path } as *const i8,
                                                        &raw mut read_buf[(1 + 16) as usize] as *const i8,
                                                        (read_len - (1 + 16)) as u64)
                                                } == 0) as i32 != 0 {
                                __state = 44;
                            } else { __state = 31; }
                        }
                        36 => {
                            path_len = (read_len - (1 + 16)) as u64;
                            __state = 37;
                        }
                        37 => {
                            if path_len >= 1024 as u64 {
                                __state = 39;
                            } else { __state = 38; }
                        }
                        38 => {
                            unsafe {
                                memcpy(&raw mut lock_path[0 as usize] as *mut i8 as *mut (),
                                    &raw mut read_buf[(1 + 16) as usize] as *const (), path_len)
                            };
                            __state = 40;
                        }
                        39 => { path_len = (1024 - 1) as u64; __state = 38; }
                        40 => {
                            lock_path[path_len as usize] = 0 as i8;
                            __state = 41;
                        }
                        41 => {
                            temp_lock_path = &raw mut lock_path[0 as usize] as *mut i8;
                            __state = 42;
                        }
                        42 => { try_old_lock_path = 1; __state = 43; }
                        43 => { __state = 2; }
                        44 => { __state = 2; }
                        45 => {
                            if (unsafe { (*p_ctx).lock_proxy_path }).is_null() as i32 !=
                                    0 {
                                __state = 49;
                            } else { __state = 48; }
                        }
                        46 => { rc = 5; __state = 47; }
                        47 => { __state = 2; }
                        48 => {
                            unsafe {
                                futimes(unsafe { (*conch_file).h },
                                    0 as *mut () as *const Timeval)
                            };
                            __state = 51;
                        }
                        49 => {
                            proxy_get_lock_path(unsafe { (*p_ctx).db_path } as
                                    *const i8, &raw mut lock_path[0 as usize] as *mut i8,
                                1024 as u64);
                            __state = 50;
                        }
                        50 => {
                            temp_lock_path = &raw mut lock_path[0 as usize] as *mut i8;
                            __state = 48;
                        }
                        51 => {
                            if host_id_match != 0 && (create_conch == 0) as i32 != 0 {
                                __state = 53;
                            } else { __state = 54; }
                        }
                        52 => {
                            if rc == 0 { __state = 58; } else { __state = 57; }
                        }
                        53 => {
                            if !(unsafe { (*conch_file).p_inode }).is_null() &&
                                    unsafe { (*unsafe { (*conch_file).p_inode }).n_shared } > 1
                                {
                                __state = 55;
                            } else { __state = 56; }
                        }
                        54 => {
                            rc =
                                proxy_conch_lock(p_file_1,
                                    &raw mut my_host_id[0 as usize] as *mut u8, 4);
                            __state = 52;
                        }
                        55 => { rc = 5; __state = 52; }
                        56 => {
                            rc =
                                proxy_conch_lock(p_file_1,
                                    &raw mut my_host_id[0 as usize] as *mut u8, 4);
                            __state = 52;
                        }
                        57 => {
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*conch_file).p_method }).x_unlock.unwrap()
                                    })(conch_file as *mut Sqlite3File, 1)
                            };
                            __state = 75;
                        }
                        58 => { __state = 59; }
                        59 => { write_size = 0; __state = 60; }
                        60 => { write_buffer[0 as usize] = 2 as i8; __state = 61; }
                        61 => {
                            unsafe {
                                memcpy(&raw mut write_buffer[1 as usize] as *mut (),
                                    &raw mut my_host_id[0 as usize] as *mut u8 as *const (),
                                    16 as u64)
                            };
                            __state = 62;
                        }
                        62 => {
                            if unsafe { (*p_ctx).lock_proxy_path } as *mut () !=
                                    0 as *mut () {
                                __state = 64;
                            } else { __state = 65; }
                        }
                        63 => {
                            write_size =
                                ((1 + 16) as u64 +
                                        unsafe {
                                            strlen(&raw mut write_buffer[(1 + 16) as usize] as
                                                    *const i8)
                                        }) as i32;
                            __state = 66;
                        }
                        64 => {
                            unsafe {
                                strlcpy(&mut write_buffer[(1 + 16) as usize],
                                    unsafe { (*p_ctx).lock_proxy_path } as *const i8,
                                    1024 as u64)
                            };
                            __state = 63;
                        }
                        65 => {
                            unsafe {
                                strlcpy(&mut write_buffer[(1 + 16) as usize],
                                    temp_lock_path as *const i8, 1024 as u64)
                            };
                            __state = 63;
                        }
                        66 => {
                            robust_ftruncate(unsafe { (*conch_file).h },
                                write_size as Sqlite3Int64);
                            __state = 67;
                        }
                        67 => {
                            rc =
                                unix_write(conch_file as *mut Sqlite3File,
                                    &raw mut write_buffer[0 as usize] as *mut i8 as *const (),
                                    write_size, 0 as Sqlite3Int64);
                            __state = 68;
                        }
                        68 => {
                            full_fsync(unsafe { (*conch_file).h }, 0, 0);
                            __state = 69;
                        }
                        69 => {
                            if rc == 0 && create_conch != 0 {
                                __state = 70;
                            } else { __state = 57; }
                        }
                        70 => { __state = 71; }
                        71 => {
                            err =
                                unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(i32, *mut Stat)
                                                        ->
                                                            i32>(a_syscall[5 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(unsafe { (*p_file_1).h }, &mut buf)
                                };
                            __state = 72;
                        }
                        72 => {
                            if err == 0 { __state = 73; } else { __state = 57; }
                        }
                        73 => {
                            cmode =
                                (buf.st_mode as i32 & (256 | 128 | 32 | 16 | 4 | 2)) as
                                    ModeT;
                            __state = 74;
                        }
                        74 => {
                            unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(i32, u16)
                                                    ->
                                                        i32>(a_syscall[14 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(unsafe { (*conch_file).h }, cmode)
                            };
                            __state = 57;
                        }
                        75 => { __state = 2; }
                        76 => {
                            if rc == 0 && unsafe { (*p_file_1).open_flags } != 0 {
                                __state = 78;
                            } else { __state = 77; }
                        }
                        77 => {
                            if rc == 0 &&
                                    (unsafe { (*p_ctx).lock_proxy }).is_null() as i32 != 0 {
                                __state = 88;
                            } else { __state = 87; }
                        }
                        78 => { __state = 79; }
                        79 => {
                            if unsafe { (*p_file_1).h } >= 0 {
                                __state = 81;
                            } else { __state = 80; }
                        }
                        80 => { unsafe { (*p_file_1).h = -1 }; __state = 82; }
                        81 => {
                            robust_close(p_file_1 as *const UnixFile,
                                unsafe { (*p_file_1).h }, 7968);
                            __state = 80;
                        }
                        82 => {
                            fd =
                                robust_open(unsafe { (*p_ctx).db_path } as *const i8,
                                    unsafe { (*p_file_1).open_flags }, 0 as ModeT);
                            __state = 83;
                        }
                        83 => { __state = 84; }
                        84 => {
                            if fd >= 0 { __state = 85; } else { __state = 86; }
                        }
                        85 => { unsafe { (*p_file_1).h = fd }; __state = 77; }
                        86 => {
                            rc = unsafe { sqlite3_cantopen_error(7976) };
                            __state = 77;
                        }
                        87 => {
                            if rc == 0 { __state = 95; } else { __state = 94; }
                        }
                        88 => {
                            path =
                                if !(temp_lock_path).is_null() {
                                        temp_lock_path
                                    } else { unsafe { (*p_ctx).lock_proxy_path } } as *const i8;
                            __state = 89;
                        }
                        89 => {
                            rc =
                                proxy_create_unix_file(path as *const i8,
                                    unsafe { &mut (*p_ctx).lock_proxy }, 1);
                            __state = 90;
                        }
                        90 => {
                            if rc != 0 && rc != 7 && try_old_lock_path != 0 {
                                __state = 91;
                            } else { __state = 87; }
                        }
                        91 => { force_new_lock_path = 1; __state = 92; }
                        92 => { try_old_lock_path = 0; __state = 93; }
                        93 => { __state = 30; }
                        94 => {
                            if rc == 0 { __state = 100; } else { __state = 101; }
                        }
                        95 => {
                            if !(temp_lock_path).is_null() {
                                __state = 96;
                            } else { __state = 94; }
                        }
                        96 => {
                            unsafe {
                                (*p_ctx).lock_proxy_path =
                                    unsafe {
                                        sqlite3_db_str_dup(core::ptr::null_mut(),
                                            temp_lock_path as *const i8)
                                    }
                            };
                            __state = 97;
                        }
                        97 => {
                            if (unsafe { (*p_ctx).lock_proxy_path }).is_null() as i32 !=
                                    0 {
                                __state = 98;
                            } else { __state = 94; }
                        }
                        98 => { rc = 7; __state = 94; }
                        99 => { __state = 106; }
                        100 => {
                            unsafe { (*p_ctx).conch_held = 1 };
                            __state = 102;
                        }
                        101 => {
                            unsafe {
                                (unsafe {
                                        (*unsafe { (*conch_file).p_method }).x_unlock.unwrap()
                                    })(conch_file as *mut Sqlite3File, 0)
                            };
                            __state = 99;
                        }
                        102 => {
                            if unsafe { (*unsafe { (*p_ctx).lock_proxy }).p_method } ==
                                    &raw const afp_io_methods as *const Sqlite3IoMethods {
                                __state = 103;
                            } else { __state = 99; }
                        }
                        103 => { __state = 104; }
                        104 => {
                            afp_ctx =
                                unsafe { (*unsafe { (*p_ctx).lock_proxy }).locking_context }
                                    as *mut AfpLockingContext;
                            __state = 105;
                        }
                        105 => {
                            unsafe {
                                (*afp_ctx).db_path =
                                    unsafe { (*p_ctx).lock_proxy_path } as *const i8
                            };
                            __state = 99;
                        }
                        106 => { return rc; }
                        _ => {}
                    }
                }
            }

            /// read the existing conch file
            /// I/O error: lastErrno set by seekAndRead
            /// a short read or version format mismatch means we need to create a new
            ///* conch file.
            /// if the host id matches and the lock path already exists in the conch
            ///* we'll try to use the path there, if we can't open that path, we'll
            ///* retry with a new auto-generated path
            /// in case we need to try again for an :auto: named lock file
            /// if the conch has data compare the contents
            /// for auto-named local lock file, just check the host ID and we'll
            ///* use the local lock file path that's already in there
            /// create a copy of the lock path if the conch is taken
            /// conch host and lock path match
            /// if the conch isn't writable and doesn't match, we can't take it
            /// either the conch didn't match or we need to create a new one
            /// create a copy of the lock path _only_ if the conch is taken
            /// update conch with host and path (this will fail if other process
            ///* has a shared lock already), if the host id matches, use the big
            ///* stick.
            /// We are trying for an exclusive lock but another thread in this
            ///* same process is still holding a shared lock.
            /// If we created a new conch file (not just updated the contents of a
            ///* valid conch file), try to match the permissions of the database
            /// try to match the database file R/W permissions, ignore failure
            /// SQLITE_BUSY? proxyTakeConch called
            ///           during locking
            /// we couldn't create the proxy lock file with the old lock file path
            ///* so try again via auto-naming
            /// go back to the do {} while start point, try again
            /// Need to make a copy of path if we extracted the value
            ///* from the conch file or the path was allocated on the stack
            /// in case we need to retry the :auto: lock file -
            ///* we should never get here except via the 'continue' call.
            unreachable!();
        }
    }
}

///* Lock the file with the lock specified by parameter eFileLock - one
///* of the following:
///*
///*     (1) SHARED_LOCK
///*     (2) RESERVED_LOCK
///*     (3) PENDING_LOCK
///*     (4) EXCLUSIVE_LOCK
///*
///* Sometimes when requesting one lock state, additional lock states
///* are inserted in between.  The locking might fail on one of the later
///* transitions leaving the lock state different from what it started but
///* still short of its goal.  The following chart shows the allowed
///* transitions and the inserted intermediate states:
///*
///*    UNLOCKED -> SHARED
///*    SHARED -> RESERVED
///*    SHARED -> (PENDING) -> EXCLUSIVE
///*    RESERVED -> (PENDING) -> EXCLUSIVE
///*    PENDING -> EXCLUSIVE
///*
///* This routine will only increase a lock.  Use the sqlite3OsUnlock()
///* routine to lower a locking level.
extern "C" fn proxy_lock(id: *mut Sqlite3File, e_file_lock: i32) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let mut rc: i32 = proxy_take_conch(p_file);
    if rc == 0 {
        let p_ctx: *const ProxyLockingContext =
            unsafe { (*p_file).locking_context } as *mut ProxyLockingContext
                as *const ProxyLockingContext;
        if unsafe { (*p_ctx).conch_held } > 0 {
            let proxy: *mut UnixFile = unsafe { (*p_ctx).lock_proxy };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*proxy).p_method }).x_lock.unwrap()
                        })(proxy as *mut Sqlite3File, e_file_lock)
                };
            unsafe {
                (*p_file).e_file_lock = unsafe { (*proxy).e_file_lock }
            };
        } else {}
    }
    return rc;
}

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
extern "C" fn proxy_unlock(id: *mut Sqlite3File, e_file_lock: i32) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let mut rc: i32 = proxy_take_conch(p_file);
    if rc == 0 {
        let p_ctx: *const ProxyLockingContext =
            unsafe { (*p_file).locking_context } as *mut ProxyLockingContext
                as *const ProxyLockingContext;
        if unsafe { (*p_ctx).conch_held } > 0 {
            let proxy: *mut UnixFile = unsafe { (*p_ctx).lock_proxy };
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*proxy).p_method }).x_unlock.unwrap()
                        })(proxy as *mut Sqlite3File, e_file_lock)
                };
            unsafe {
                (*p_file).e_file_lock = unsafe { (*proxy).e_file_lock }
            };
        } else {}
    }
    return rc;
}

///* This routine checks if there is a RESERVED lock held on the specified
///* file by this or any other process. If such a lock is held, set *pResOut
///* to a non-zero value otherwise *pResOut is set to zero.  The return value
///* is set to SQLITE_OK unless an I/O error occurs during lock checking.
#[allow(unused_doc_comments)]
extern "C" fn proxy_check_reserved_lock(id: *mut Sqlite3File,
    mut p_res_out: *mut i32) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    let rc: i32 = proxy_take_conch(p_file);
    if rc == 0 {
        let p_ctx: *const ProxyLockingContext =
            unsafe { (*p_file).locking_context } as *mut ProxyLockingContext
                as *const ProxyLockingContext;
        if unsafe { (*p_ctx).conch_held } > 0 {
            let proxy: *mut UnixFile = unsafe { (*p_ctx).lock_proxy };
            return unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*proxy).p_method
                                            }).x_check_reserved_lock.unwrap()
                        })(proxy as *mut Sqlite3File, p_res_out)
                };
        } else {

            /// conchHeld < 0 is lockless
            (p_res_out = core::ptr::null_mut());
        }
    }
    return rc;
}

static proxy_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(proxy_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(proxy_lock),
        x_unlock: Some(proxy_unlock),
        x_check_reserved_lock: Some(proxy_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: None,
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

/// Takes a fully configured proxy locking-style unix file and switches
///* the local lock file path
extern "C" fn switch_lock_proxy_path(p_file_1: &mut UnixFile, path: *const i8)
    -> i32 {
    let p_ctx: *mut ProxyLockingContext =
        (*p_file_1).locking_context as *mut ProxyLockingContext;
    let old_path: *mut i8 = unsafe { (*p_ctx).lock_proxy_path };
    let mut rc: i32 = 0;
    if (*p_file_1).e_file_lock as i32 != 0 { return 5; }
    if (path).is_null() as i32 != 0 ||
                    unsafe { *path.offset(0 as isize) } as i32 == '\u{0}' as i32
                ||
                (unsafe {
                                strcmp(path, c":auto:".as_ptr() as *mut i8 as *const i8)
                            } == 0) as i32 != 0 ||
            !(old_path).is_null() &&
                (unsafe { strncmp(old_path as *const i8, path, 1024 as u64) }
                            == 0) as i32 != 0 {
        return 0;
    } else {
        let lock_proxy: *mut UnixFile = unsafe { (*p_ctx).lock_proxy };
        unsafe { (*p_ctx).lock_proxy = 0 as *mut () as *mut UnixFile };
        unsafe { (*p_ctx).conch_held = 0 };
        if lock_proxy as *mut () != 0 as *mut () {
            rc =
                unsafe {
                    (unsafe {
                            (*unsafe { (*lock_proxy).p_method }).x_close.unwrap()
                        })(lock_proxy as *mut Sqlite3File)
                };
            if rc != 0 { return rc; }
            unsafe { sqlite3_free(lock_proxy as *mut ()) };
        }
        unsafe { sqlite3_free(old_path as *mut ()) };
        unsafe {
            (*p_ctx).lock_proxy_path =
                unsafe { sqlite3_db_str_dup(core::ptr::null_mut(), path) }
        };
    }
    return rc;
}

///* pFile is a file that has been opened by a prior xOpen call.  dbPath
///* is a string buffer at least MAXPATHLEN+1 characters in size.
///*
///* This routine find the filename associated with pFile and writes it
///* int dbPath.
#[allow(unused_doc_comments)]
extern "C" fn proxy_get_db_path_for_unix_file(p_file_1: &mut UnixFile,
    db_path_1: *mut i8) -> i32 {
    if (*p_file_1).p_method ==
            &raw const afp_io_methods as *const Sqlite3IoMethods {

        /// afp style keeps a reference to the db path in the filePath field
        ///* of the struct
        { let _ = 0; };
        unsafe {
            strlcpy(db_path_1,
                unsafe {
                    (*((*p_file_1).locking_context as
                                    *mut AfpLockingContext)).db_path
                }, 1024 as u64)
        };
    } else if (*p_file_1).p_method ==
            &raw const dotlock_io_methods as *const Sqlite3IoMethods {
        /// dot lock style uses the locking context to store the dot lock
        ///* file path
        let len: i32 =
            (unsafe {
                        strlen((*p_file_1).locking_context as *mut i8 as *const i8)
                    } -
                    unsafe {
                        strlen(c".lock".as_ptr() as *mut i8 as *const i8)
                    }) as i32;
        unsafe {
            memcpy(db_path_1 as *mut (),
                (*p_file_1).locking_context as *mut i8 as *const (),
                (len + 1) as u64)
        };
    } else {

        /// all other styles use the locking context to store the db file path
        { let _ = 0; };
        unsafe {
            strlcpy(db_path_1,
                (*p_file_1).locking_context as *mut i8 as *const i8,
                1024 as u64)
        };
    }
    return 0;
}

///* Given the name of a database file, compute the name of its conch file.
///* Store the conch filename in memory obtained from sqlite3_malloc64().
///* Make *pConchPath point to the new name.  Return SQLITE_OK on success
///* or SQLITE_NOMEM if unable to obtain memory.
///*
///* The caller is responsible for ensuring that the allocated memory
///* space is eventually freed.
///*
///* *pConchPath is set to NULL if a memory allocation error occurs.
#[allow(unused_doc_comments)]
extern "C" fn proxy_create_conch_pathname(db_path_1: *const i8,
    p_conch_path_1: &mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    /// Loop counter
    let len: i32 = unsafe { strlen(db_path_1 as *const i8) } as i32;
    /// Length of database filename - dbPath
    let mut conch_path: *mut i8 = core::ptr::null_mut();

    /// buffer in which to construct conch name
    /// Allocate space for the conch filename and initialize the name to
    ///* the name of the original database file.
    (*p_conch_path_1 =
        {
            conch_path =
                unsafe { sqlite3_malloc64((len + 8) as Sqlite3Uint64) } as
                    *mut i8;
            conch_path
        });
    if conch_path == core::ptr::null_mut() { return 7; }
    unsafe {
        memcpy(conch_path as *mut (), db_path_1 as *const (),
            (len + 1) as u64)
    };
    {
        i = len - 1;
        '__b32: loop {
            if !(i >= 0) { break '__b32; }
            '__c32: loop {
                if unsafe { *conch_path.offset(i as isize) } as i32 ==
                        '/' as i32 {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    break '__b32;
                }
                break '__c32;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    unsafe { *conch_path.offset(i as isize) = '.' as i32 as i8 };
    while i < len {
        unsafe {
            *conch_path.offset((i + 1) as isize) =
                unsafe { *db_path_1.offset(i as isize) }
        };
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }

    /// append the "-conch" suffix to the file
    unsafe {
        memcpy(unsafe { &raw mut *conch_path.offset((i + 1) as isize) } as
                *mut (), c"-conch".as_ptr() as *mut i8 as *const (), 7 as u64)
    };
    { let _ = 0; };
    return 0;
}

///* Routine to transform a unixFile into a proxy-locking unixFile.
///* Implementation in the proxy-lock division, but used by unixOpen()
///* if SQLITE_PREFER_PROXY_LOCKING is defined.
#[allow(unused_doc_comments)]
extern "C" fn proxy_transform_unix_file(p_file_1: *mut UnixFile,
    path: *const i8) -> i32 {
    unsafe {
        let mut p_ctx: *mut ProxyLockingContext = core::ptr::null_mut();
        let mut db_path: [i8; 1025] = [0; 1025];
        /// Name of the database file
        let mut lock_path: *const i8 = 0 as *mut () as *const i8;
        let mut rc: i32 = 0;
        if unsafe { (*p_file_1).e_file_lock } as i32 != 0 { return 5; }
        proxy_get_db_path_for_unix_file(unsafe { &mut *p_file_1 },
            &raw mut db_path[0 as usize] as *mut i8);
        if (path).is_null() as i32 != 0 ||
                    unsafe { *path.offset(0 as isize) } as i32 == '\u{0}' as i32
                ||
                (unsafe {
                                strcmp(path, c":auto:".as_ptr() as *mut i8 as *const i8)
                            } == 0) as i32 != 0 {
            lock_path = 0 as *mut () as *mut i8;
        } else { lock_path = path as *mut i8; }
        p_ctx =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<ProxyLockingContext>()
                            as Sqlite3Uint64)
                } as *mut ProxyLockingContext;
        if p_ctx == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_ctx as *mut (), 0,
                core::mem::size_of::<ProxyLockingContext>() as u64)
        };
        rc =
            proxy_create_conch_pathname(&raw mut db_path[0 as usize] as
                        *mut i8 as *const i8,
                unsafe { &mut (*p_ctx).conch_file_path });
        if rc == 0 {
            rc =
                proxy_create_unix_file(unsafe { (*p_ctx).conch_file_path } as
                        *const i8, unsafe { &mut (*p_ctx).conch_file }, 0);
            if rc == 14 && unsafe { (*p_file_1).open_flags } & 2 == 0 {
                /// if (a) the open flags are not O_RDWR, (b) the conch isn't there, and
                ///* (c) the file system is read-only, then enable no-locking access.
                ///* Ugh, since O_RDONLY==0x0000 we test for !O_RDWR since unixOpen asserts
                ///* that openFlags will have only one of O_RDONLY or O_RDWR.
                let mut fs_info: Statfs = unsafe { core::mem::zeroed() };
                let mut conch_info: Stat = unsafe { core::mem::zeroed() };
                let mut go_lockless: i32 = 0;
                if unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*const i8, *mut Stat)
                                                ->
                                                    i32>(a_syscall[4 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(unsafe { (*p_ctx).conch_file_path } as *const i8,
                                &mut conch_info)
                        } == -1 {
                    let err: i32 = unsafe { *unsafe { __error() } };
                    if err == 2 &&
                            unsafe {
                                    statfs(&raw mut db_path[0 as usize] as *mut i8 as *const i8,
                                        &mut fs_info)
                                } != -1 {
                        go_lockless =
                            (fs_info.f_flags & 1 as u32 == 1 as u32) as i32;
                    }
                }
                if go_lockless != 0 {
                    unsafe { (*p_ctx).conch_held = -1 };

                    /// read only FS/ lockless
                    (rc = 0);
                }
            }
        }
        if rc == 0 && !(lock_path).is_null() {
            unsafe {
                (*p_ctx).lock_proxy_path =
                    unsafe {
                        sqlite3_db_str_dup(core::ptr::null_mut(),
                            lock_path as *const i8)
                    }
            };
        }
        if rc == 0 {
            unsafe {
                (*p_ctx).db_path =
                    unsafe {
                        sqlite3_db_str_dup(core::ptr::null_mut(),
                            &raw mut db_path[0 as usize] as *mut i8 as *const i8)
                    }
            };
            if unsafe { (*p_ctx).db_path } as *mut () == 0 as *mut () {
                rc = 7;
            }
        }
        if rc == 0 {

            /// all memory is allocated, proxys are created and assigned,
            ///* switch the locking context and pMethod then return.
            unsafe {
                (*p_ctx).old_locking_context =
                    unsafe { (*p_file_1).locking_context }
            };
            unsafe { (*p_file_1).locking_context = p_ctx as *mut () };
            unsafe {
                (*p_ctx).p_old_method = unsafe { (*p_file_1).p_method }
            };
            unsafe { (*p_file_1).p_method = &proxy_io_methods };
        } else {
            if !(unsafe { (*p_ctx).conch_file }).is_null() {
                unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*unsafe { (*p_ctx).conch_file }).p_method
                                            }).x_close.unwrap()
                        })(unsafe { (*p_ctx).conch_file } as *mut Sqlite3File)
                };
                unsafe {
                    sqlite3_free(unsafe { (*p_ctx).conch_file } as *mut ())
                };
            }
            unsafe {
                sqlite3_db_free(core::ptr::null_mut(),
                    unsafe { (*p_ctx).lock_proxy_path } as *mut ())
            };
            unsafe {
                sqlite3_free(unsafe { (*p_ctx).conch_file_path } as *mut ())
            };
            unsafe { sqlite3_free(p_ctx as *mut ()) };
        }
        return rc;
    }
}

///* Handler for proxy-locking file-control verbs.  Defined below in the
///* proxying locking division.
#[allow(unused_doc_comments)]
extern "C" fn proxy_file_control(id: *mut Sqlite3File, op: i32,
    p_arg: *mut ()) -> i32 {
    '__s34:
        {
        match op {
            2 => {
                {
                    let p_file: *mut UnixFile = id as *mut UnixFile;
                    if unsafe { (*p_file).p_method } ==
                            &raw const proxy_io_methods as *const Sqlite3IoMethods {
                        let p_ctx: *const ProxyLockingContext =
                            unsafe { (*p_file).locking_context } as
                                    *mut ProxyLockingContext as *const ProxyLockingContext;
                        proxy_take_conch(p_file);
                        if !(unsafe { (*p_ctx).lock_proxy_path }).is_null() {
                            unsafe {
                                *(p_arg as *mut *const i8) =
                                    unsafe { (*p_ctx).lock_proxy_path } as *const i8
                            };
                        } else {
                            unsafe {
                                *(p_arg as *mut *const i8) =
                                    c":auto: (not held)".as_ptr() as *mut i8 as *const i8
                            };
                        }
                    } else {
                        unsafe {
                            *(p_arg as *mut *const i8) = 0 as *mut () as *const i8
                        };
                    }
                    return 0;
                }
                {
                    let p_file_1: *mut UnixFile = id as *mut UnixFile;
                    let mut rc: i32 = 0;
                    let is_proxy_style: i32 =
                        (unsafe { (*p_file_1).p_method } ==
                                &raw const proxy_io_methods as *const Sqlite3IoMethods) as
                            i32;
                    if p_arg == 0 as *mut () ||
                            p_arg as *const i8 == core::ptr::null() {
                        if is_proxy_style != 0 {

                            /// turn off proxy locking - not supported.  If support is added for
                            ///* switching proxy locking mode off then it will need to fail if
                            ///* the journal mode is WAL mode.
                            (rc = 1);
                        } else {

                            /// turn off proxy locking - already off - NOOP
                            (rc = 0);
                        }
                    } else {
                        let proxy_path: *const i8 = p_arg as *const i8;
                        if is_proxy_style != 0 {
                            let p_ctx_1: *const ProxyLockingContext =
                                unsafe { (*p_file_1).locking_context } as
                                        *mut ProxyLockingContext as *const ProxyLockingContext;
                            if (unsafe {
                                                    strcmp(p_arg as *const i8,
                                                        c":auto:".as_ptr() as *mut i8 as *const i8)
                                                } == 0) as i32 != 0 ||
                                    !(unsafe { (*p_ctx_1).lock_proxy_path }).is_null() &&
                                        (unsafe {
                                                        strncmp(unsafe { (*p_ctx_1).lock_proxy_path } as *const i8,
                                                            proxy_path, 1024 as u64)
                                                    } == 0) as i32 != 0 {
                                rc = 0;
                            } else {
                                rc =
                                    switch_lock_proxy_path(unsafe { &mut *p_file_1 },
                                        proxy_path);
                            }
                        } else {

                            /// turn on proxy file locking
                            (rc = proxy_transform_unix_file(p_file_1, proxy_path));
                        }
                    }
                    return rc;
                }
                { { let _ = 0; }; }
            }
            3 => {
                {
                    let p_file_1: *mut UnixFile = id as *mut UnixFile;
                    let mut rc: i32 = 0;
                    let is_proxy_style: i32 =
                        (unsafe { (*p_file_1).p_method } ==
                                &raw const proxy_io_methods as *const Sqlite3IoMethods) as
                            i32;
                    if p_arg == 0 as *mut () ||
                            p_arg as *const i8 == core::ptr::null() {
                        if is_proxy_style != 0 {

                            /// turn off proxy locking - not supported.  If support is added for
                            ///* switching proxy locking mode off then it will need to fail if
                            ///* the journal mode is WAL mode.
                            (rc = 1);
                        } else {

                            /// turn off proxy locking - already off - NOOP
                            (rc = 0);
                        }
                    } else {
                        let proxy_path: *const i8 = p_arg as *const i8;
                        if is_proxy_style != 0 {
                            let p_ctx_1: *const ProxyLockingContext =
                                unsafe { (*p_file_1).locking_context } as
                                        *mut ProxyLockingContext as *const ProxyLockingContext;
                            if (unsafe {
                                                    strcmp(p_arg as *const i8,
                                                        c":auto:".as_ptr() as *mut i8 as *const i8)
                                                } == 0) as i32 != 0 ||
                                    !(unsafe { (*p_ctx_1).lock_proxy_path }).is_null() &&
                                        (unsafe {
                                                        strncmp(unsafe { (*p_ctx_1).lock_proxy_path } as *const i8,
                                                            proxy_path, 1024 as u64)
                                                    } == 0) as i32 != 0 {
                                rc = 0;
                            } else {
                                rc =
                                    switch_lock_proxy_path(unsafe { &mut *p_file_1 },
                                        proxy_path);
                            }
                        } else {

                            /// turn on proxy file locking
                            (rc = proxy_transform_unix_file(p_file_1, proxy_path));
                        }
                    }
                    return rc;
                }
                { { let _ = 0; }; }
            }
            _ => { { { let _ = 0; }; } }
        }
    }

    ///NOTREACHED
    { let _ = 0; };
    return 1;
}

///* Use F_GETLK to check whether or not there are any readers with open
///* wal-mode transactions in other processes on database file pFile. If
///* no error occurs, return SQLITE_OK and set (*piOut) to 1 if there are
///* such transactions, or 0 otherwise. If an error occurs, return an
///* SQLite error code. The final value of *piOut is undefined in this
///* case.
extern "C" fn unix_fcntl_external_reader(p_file_1: &UnixFile,
    pi_out_1: &mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        *pi_out_1 = 0;
        if !((*p_file_1).p_shm).is_null() {
            let p_shm_node: *const UnixShmNode =
                unsafe { (*(*p_file_1).p_shm).p_shm_node } as
                    *const UnixShmNode;
            let mut f: Flock = unsafe { core::mem::zeroed() };
            unsafe {
                memset(&raw mut f as *mut (), 0,
                    core::mem::size_of::<Flock>() as u64)
            };
            f.l_type = 3 as i16;
            f.l_whence = 0 as i16;
            f.l_start = ((22 + 8) * 4 + 3) as OffT;
            f.l_len = (8 - 3) as OffT;
            unsafe {
                sqlite3_mutex_enter(unsafe { (*p_shm_node).p_shm_mutex })
            };
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(i32, i32, ...)
                                            ->
                                                i32>(a_syscall[7 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(unsafe { (*p_shm_node).h_shm }, 7, &mut f)
                    } < 0 {
                rc = 10 | 15 << 8;
            } else { *pi_out_1 = (f.l_type as i32 != 2) as i32; }
            unsafe {
                sqlite3_mutex_leave(unsafe { (*p_shm_node).p_shm_mutex })
            };
        }
        return rc;
    }
}

///* Information and control of an open file handle.
extern "C" fn unix_file_control(id: *mut Sqlite3File, op: i32,
    p_arg_1: *mut ()) -> i32 {
    unsafe {
        let p_file: *mut UnixFile = id as *mut UnixFile;
        '__s35:
            {
            match op {
                43 => {
                    {
                        unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(i32)
                                                ->
                                                    i32>(a_syscall[1 as
                                                            usize].p_current.unwrap_or(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                                }) as *const ())
                                })(unsafe { (*p_file).h })
                        };
                        unsafe { (*p_file).h = -1 };
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) =
                                unsafe { (*p_file).e_file_lock } as i32
                        };
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = unsafe { (*p_file).last_errno }
                        };
                        return 0;
                    }
                    {
                        unsafe {
                            (*p_file).sz_chunk = unsafe { *(p_arg_1 as *mut i32) }
                        };
                        return 0;
                    }
                    {
                        let mut rc: i32 = 0;
                        rc =
                            fcntl_size_hint(p_file, unsafe { *(p_arg_1 as *mut i64) });
                        return rc;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 4 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                1 => {
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) =
                                unsafe { (*p_file).e_file_lock } as i32
                        };
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = unsafe { (*p_file).last_errno }
                        };
                        return 0;
                    }
                    {
                        unsafe {
                            (*p_file).sz_chunk = unsafe { *(p_arg_1 as *mut i32) }
                        };
                        return 0;
                    }
                    {
                        let mut rc: i32 = 0;
                        rc =
                            fcntl_size_hint(p_file, unsafe { *(p_arg_1 as *mut i64) });
                        return rc;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 4 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                4 => {
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = unsafe { (*p_file).last_errno }
                        };
                        return 0;
                    }
                    {
                        unsafe {
                            (*p_file).sz_chunk = unsafe { *(p_arg_1 as *mut i32) }
                        };
                        return 0;
                    }
                    {
                        let mut rc: i32 = 0;
                        rc =
                            fcntl_size_hint(p_file, unsafe { *(p_arg_1 as *mut i64) });
                        return rc;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 4 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                6 => {
                    {
                        unsafe {
                            (*p_file).sz_chunk = unsafe { *(p_arg_1 as *mut i32) }
                        };
                        return 0;
                    }
                    {
                        let mut rc: i32 = 0;
                        rc =
                            fcntl_size_hint(p_file, unsafe { *(p_arg_1 as *mut i64) });
                        return rc;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 4 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                5 => {
                    {
                        let mut rc: i32 = 0;
                        rc =
                            fcntl_size_hint(p_file, unsafe { *(p_arg_1 as *mut i64) });
                        return rc;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 4 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                10 => {
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 4 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                13 => {
                    {
                        unix_mode_bit(unsafe { &mut *p_file }, 16 as u8,
                            unsafe { &mut *(p_arg_1 as *mut i32) });
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                12 => {
                    {
                        unsafe {
                            *(p_arg_1 as *mut *mut i8) =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*unsafe { (*p_file).p_vfs }).z_name })
                                }
                        };
                        return 0;
                    }
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                16 => {
                    {
                        let z_t_file: *mut i8 =
                            unsafe {
                                    sqlite3_malloc64(unsafe {
                                                (*unsafe { (*p_file).p_vfs }).mx_pathname
                                            } as Sqlite3Uint64)
                                } as *mut i8;
                        if !(z_t_file).is_null() {
                            unix_get_tempname(unsafe {
                                    (*unsafe { (*p_file).p_vfs }).mx_pathname
                                }, z_t_file);
                            unsafe { *(p_arg_1 as *mut *mut i8) = z_t_file };
                        }
                        return 0;
                    }
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                20 => {
                    {
                        unsafe {
                            *(p_arg_1 as *mut i32) = file_has_moved(unsafe { &*p_file })
                        };
                        return 0;
                    }
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                18 => {
                    {
                        let mut new_limit: i64 = unsafe { *(p_arg_1 as *mut i64) };
                        let mut rc: i32 = 0;
                        if new_limit > sqlite3Config.mx_mmap {
                            new_limit = sqlite3Config.mx_mmap;
                        }
                        if new_limit > 0 as i64 &&
                                (core::mem::size_of::<u64>() as u64) < 8 as u64 {
                            new_limit = new_limit & 2147483647 as i64;
                        }
                        unsafe {
                            *(p_arg_1 as *mut i64) = unsafe { (*p_file).mmap_size_max }
                        };
                        if new_limit >= 0 as i64 &&
                                    new_limit != unsafe { (*p_file).mmap_size_max } &&
                                unsafe { (*p_file).n_fetch_out } == 0 {
                            unsafe { (*p_file).mmap_size_max = new_limit };
                            if unsafe { (*p_file).mmap_size } > 0 as i64 {
                                unsafe { unix_unmapfile(unsafe { &mut *p_file }) };
                                rc = unsafe { unix_mapfile(p_file, -1 as i64) };
                            }
                        }
                        return rc;
                    }
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                3 => {
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                2 => {
                    { return proxy_file_control(id, op, p_arg_1); }
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                40 => {
                    {
                        return unsafe {
                                unix_fcntl_external_reader(unsafe {
                                        &*(id as *mut UnixFile)
                                    }, unsafe { &mut *(p_arg_1 as *mut i32) })
                            };
                    }
                }
                _ => {}
            }
        }
        return 12;
    }
}

///* On some systems, calls to fchown() will trigger a message in a security
///* log if they come from non-root processes.  So avoid calling fchown() if
///* we are not running as root.
extern "C" fn robust_fchown(fd: i32, uid: UidT, gid: GidT) -> i32 {
    unsafe {
        return if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn()
                                            ->
                                                u32>(a_syscall[21 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })()
                    } != 0 {
                0
            } else {
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, u32, u32)
                                        ->
                                            i32>(a_syscall[20 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(fd, uid, gid)
                }
            };
    }
}

///* The DMS lock has not yet been taken on shm file pShmNode. Attempt to
///* take it now. Return SQLITE_OK if successful, or an SQLite error
///* code otherwise.
///*
///* If the DMS cannot be locked because this is a readonly_shm=1
///* connection and no other process already holds a lock, return
///* SQLITE_READONLY_CANTINIT and set pShmNode->isUnlocked=1.
#[allow(unused_doc_comments)]
extern "C" fn unix_lock_shared_memory(p_db_fd_1: *mut UnixFile,
    p_shm_node_1: &mut UnixShmNode) -> i32 {
    unsafe {
        let mut lock: Flock = unsafe { core::mem::zeroed() };
        let mut rc: i32 = 0;

        /// Use F_GETLK to determine the locks other processes are holding
        ///* on the DMS byte. If it indicates that another process is holding
        ///* a SHARED lock, then this process may also take a SHARED lock
        ///* and proceed with opening the *-shm file.
        ///*
        ///* Or, if no other process is holding any lock, then this process
        ///* is the first to open it. In this case take an EXCLUSIVE lock on the
        ///* DMS byte and truncate the *-shm file to zero bytes in size. Then
        ///* downgrade to a SHARED lock on the DMS byte.
        ///*
        ///* If another process is holding an EXCLUSIVE lock on the DMS byte,
        ///* return SQLITE_BUSY to the caller (it will try again). An earlier
        ///* version of this code attempted the SHARED lock at this point. But
        ///* this introduced a subtle race condition: if the process holding
        ///* EXCLUSIVE failed just before truncating the *-shm file, then this
        ///* process might open and use the *-shm file without truncating it.
        ///* And if the *-shm file has been corrupted by a power failure or
        ///* system crash, the database itself may also become corrupt.
        (lock.l_whence = 0 as i16);
        lock.l_start = ((22 + 8) * 4 + 8) as OffT;
        lock.l_len = 1 as OffT;
        lock.l_type = 3 as i16;
        if unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, i32, ...)
                                        ->
                                            i32>(a_syscall[7 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })((*p_shm_node_1).h_shm, 7, &mut lock)
                } != 0 {
            rc = 10 | 15 << 8;
        } else if lock.l_type as i32 == 2 {
            if (*p_shm_node_1).is_readonly != 0 {
                (*p_shm_node_1).is_unlocked = 1 as u8;
                rc = 8 | 5 << 8;
            } else {
                rc =
                    unix_shm_system_lock(unsafe { &*p_db_fd_1 }, 3,
                        (22 + 8) * 4 + 8, 1);
                if rc == 0 &&
                        robust_ftruncate((*p_shm_node_1).h_shm, 3 as Sqlite3Int64)
                            != 0 {
                    rc =
                        unix_log_error_at_line(10 | 18 << 8,
                            c"ftruncate".as_ptr() as *mut i8 as *const i8,
                            (*p_shm_node_1).z_filename as *const i8, 4903);
                }
            }
        } else if lock.l_type as i32 == 3 { rc = 5; }
        if rc == 0 {
            { let _ = 0; };
            rc =
                unix_shm_system_lock(unsafe { &*p_db_fd_1 }, 1,
                    (22 + 8) * 4 + 8, 1);
        }
        return rc;
    }
}

///* Open a shared-memory area associated with open database file pDbFd.
///* This particular implementation uses mmapped files.
///*
///* The file used to implement shared-memory is in the same directory
///* as the open database file and has the same name as the open database
///* file with the "-shm" suffix added.  For example, if the database file
///* is "/home/user1/config.db" then the file that is created and mmapped
///* for shared memory will be called "/home/user1/config.db-shm".
///*
///* Another approach to is to use files in /dev/shm or /dev/tmp or an
///* some other tmpfs mount. But if a file in a different directory
///* from the database file is used, then differing access permissions
///* or a chroot() might cause two different processes on the same
///* database to end up using different files for shared memory -
///* meaning that their memory would not really be shared - resulting
///* in database corruption.  Nevertheless, this tmpfs file usage
///* can be enabled at compile-time using -DSQLITE_SHM_DIRECTORY="/dev/shm"
///* or the equivalent.  The use of the SQLITE_SHM_DIRECTORY compile-time
///* option results in an incompatible build of SQLite;  builds of SQLite
///* that with differing SQLITE_SHM_DIRECTORY settings attempt to use the
///* same database file at the same time, database corruption will likely
///* result. The SQLITE_SHM_DIRECTORY compile-time option is considered
///* "unsupported" and may go away in a future SQLite release.
///*
///* When opening a new shared-memory file, if no other instances of that
///* file are currently open, in this process or in other processes, then
///* the file must be truncated to zero length or have its header cleared.
///*
///* If the original database file (pDbFd) is using the "unix-excl" VFS
///* that means that an exclusive lock is held on the database file and
///* that no other processes are able to read or write the database.  In
///* that case, we do not really need shared memory.  No shared memory
///* file is created.  The shared memory will be simulated with heap memory.
#[allow(unused_doc_comments)]
extern "C" fn unix_open_shared_memory(p_db_fd_1: *mut UnixFile) -> i32 {
    unsafe {
        let mut p: *mut UnixShm = core::ptr::null_mut();
        /// The connection to be opened
        /// The underlying mmapped file
        let mut rc: i32 = 0;
        '__b36: loop {
            '__c36: loop {
                /// The connection to be opened
                let mut p_shm_node: *mut UnixShmNode = core::ptr::null_mut();
                /// The underlying mmapped file
                /// Result code
                let mut p_inode: *const UnixInodeInfo = core::ptr::null();
                /// The inode of fd
                let mut z_shm: *mut i8 = core::ptr::null_mut();
                /// Name of the file used for SHM
                let mut n_shm_filename: i32 = 0;

                /// Size of the SHM filename in bytes
                /// Allocate space for the new unixShm object.
                (p =
                    unsafe {
                            sqlite3_malloc64(core::mem::size_of::<UnixShm>() as
                                    Sqlite3Uint64)
                        } as *mut UnixShm);
                if p == core::ptr::null_mut() { return 7; }
                unsafe {
                    memset(p as *mut (), 0,
                        core::mem::size_of::<UnixShm>() as u64)
                };
                { let _ = 0; };

                /// Check to see if a unixShmNode object already exists. Reuse an existing
                ///* one if present. Create a new one if necessary.
                { let _ = 0; };
                unix_enter_mutex();
                p_inode = unsafe { (*p_db_fd_1).p_inode };
                p_shm_node = unsafe { (*p_inode).p_shm_node };
                if p_shm_node == core::ptr::null_mut() {
                    let mut s_stat: Stat = unsafe { core::mem::zeroed() };
                    /// fstat() info for database file
                    let z_base_path: *const i8 = unsafe { (*p_db_fd_1).z_path };
                    if unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(i32, *mut Stat)
                                                    ->
                                                        i32>(a_syscall[5 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(unsafe { (*p_db_fd_1).h }, &mut s_stat)
                            } != 0 {
                        rc = 10 | 7 << 8;
                        break '__b36;
                    }
                    n_shm_filename = 6 + unsafe { strlen(z_base_path) } as i32;
                    p_shm_node =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<UnixShmNode>() as u64
                                        + n_shm_filename as u64)
                            } as *mut UnixShmNode;
                    if p_shm_node == core::ptr::null_mut() {
                        rc = 7;
                        break '__b36;
                    }
                    unsafe {
                        memset(p_shm_node as *mut (), 0,
                            core::mem::size_of::<UnixShmNode>() as u64 +
                                n_shm_filename as u64)
                    };
                    z_shm =
                        {
                            unsafe {
                                (*p_shm_node).z_filename =
                                    unsafe { &raw mut *p_shm_node.offset(1 as isize) } as
                                        *mut i8
                            };
                            unsafe { (*p_shm_node).z_filename }
                        };
                    unsafe {
                        sqlite3_snprintf(n_shm_filename, z_shm,
                            c"%s-shm".as_ptr() as *mut i8 as *const i8, z_base_path)
                    };
                    unsafe { (*p_shm_node).h_shm = -1 };
                    unsafe {
                        (*unsafe { (*p_db_fd_1).p_inode }).p_shm_node = p_shm_node
                    };
                    unsafe {
                        (*p_shm_node).p_inode = unsafe { (*p_db_fd_1).p_inode }
                    };
                    if sqlite3Config.b_core_mutex != 0 {
                        unsafe {
                            (*p_shm_node).p_shm_mutex =
                                unsafe { sqlite3_mutex_alloc(0) }
                        };
                        if unsafe { (*p_shm_node).p_shm_mutex } ==
                                core::ptr::null_mut() {
                            rc = 7;
                            break '__b36;
                        }
                    }
                    if unsafe { (*p_inode).b_process_lock } as i32 == 0 {
                        if 0 ==
                                unsafe {
                                    sqlite3_uri_boolean(unsafe { (*p_db_fd_1).z_path },
                                        c"readonly_shm".as_ptr() as *mut i8 as *const i8, 0)
                                } {
                            unsafe {
                                (*p_shm_node).h_shm =
                                    robust_open(z_shm as *const i8, 2 | 512 | 256,
                                        (s_stat.st_mode as i32 & 511) as ModeT)
                            };
                        }
                        if unsafe { (*p_shm_node).h_shm } < 0 {
                            unsafe {
                                (*p_shm_node).h_shm =
                                    robust_open(z_shm as *const i8, 0 | 256,
                                        (s_stat.st_mode as i32 & 511) as ModeT)
                            };
                            if unsafe { (*p_shm_node).h_shm } < 0 {
                                rc =
                                    unix_log_error_at_line(unsafe {
                                            sqlite3_cantopen_error(5040)
                                        }, c"open".as_ptr() as *mut i8 as *const i8,
                                        z_shm as *const i8, 5040);
                                break '__b36;
                            }
                            unsafe { (*p_shm_node).is_readonly = 1 as u8 };
                        }

                        /// If this process is running as root, make sure that the SHM file
                        ///* is owned by the same user that owns the original database.  Otherwise,
                        ///* the original owner will not be able to connect.
                        robust_fchown(unsafe { (*p_shm_node).h_shm }, s_stat.st_uid,
                            s_stat.st_gid);
                        rc =
                            unix_lock_shared_memory(p_db_fd_1,
                                unsafe { &mut *p_shm_node });
                        if rc != 0 && rc != 8 | 5 << 8 { break '__b36; }
                    }
                }

                /// Make the new connection a child of the unixShmNode
                unsafe { (*p).p_shm_node = p_shm_node };
                {
                    let __p = unsafe { &mut (*p_shm_node).n_ref };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe { (*p_db_fd_1).p_shm = p };
                unix_leave_mutex();

                /// The reference count on pShmNode has already been incremented under
                ///* the cover of the unixEnterMutex() mutex and the pointer from the
                ///* new (struct unixShm) object to the pShmNode has been set. All that is
                ///* left to do is to link the new object into the linked list starting
                ///* at pShmNode->pFirst. This must be done while holding the
                ///* pShmNode->pShmMutex.
                unsafe {
                    sqlite3_mutex_enter(unsafe { (*p_shm_node).p_shm_mutex })
                };
                unsafe { (*p).p_next = unsafe { (*p_shm_node).p_first } };
                unsafe { (*p_shm_node).p_first = p };
                unsafe {
                    sqlite3_mutex_leave(unsafe { (*p_shm_node).p_shm_mutex })
                };
                return rc;
                break '__c36;
            }
            if !(false) { break '__b36; }
        }

        /// The connection to be opened
        /// The underlying mmapped file
        /// Result code
        /// The inode of fd
        /// Name of the file used for SHM
        /// Size of the SHM filename in bytes
        /// Allocate space for the new unixShm object.
        /// Check to see if a unixShmNode object already exists. Reuse an existing
        ///* one if present. Create a new one if necessary.
        /// fstat() info for database file
        /// Call fstat() to figure out the permissions on the database file. If
        ///* a new *-shm file is created, an attempt will be made to create it
        ///* with the same permissions.
        /// If this process is running as root, make sure that the SHM file
        ///* is owned by the same user that owns the original database.  Otherwise,
        ///* the original owner will not be able to connect.
        /// Make the new connection a child of the unixShmNode
        /// The reference count on pShmNode has already been incremented under
        ///* the cover of the unixEnterMutex() mutex and the pointer from the
        ///* new (struct unixShm) object to the pShmNode has been set. All that is
        ///* left to do is to link the new object into the linked list starting
        ///* at pShmNode->pFirst. This must be done while holding the
        ///* pShmNode->pShmMutex.
        /// Jump here on any error
        unix_shm_purge(p_db_fd_1);

        /// This call frees pShmNode if required
        unsafe { sqlite3_free(p as *mut ()) };
        unix_leave_mutex();
        return rc;
    }
}

///* This function is called to obtain a pointer to region iRegion of the
///* shared-memory associated with the database file fd. Shared-memory regions
///* are numbered starting from zero. Each shared-memory region is szRegion
///* bytes in size.
///*
///* If an error occurs, an error code is returned and *pp is set to NULL.
///*
///* Otherwise, if the bExtend parameter is 0 and the requested shared-memory
///* region has not been allocated (by any client, including one running in a
///* separate process), then *pp is set to NULL and SQLITE_OK returned. If
///* bExtend is non-zero and the requested shared-memory region has not yet
///* been allocated, it is allocated by this function.
///*
///* If the shared-memory region has already been allocated or is allocated by
///* this call as described above, then it is mapped into this processes
///* address space (if it is not already), *pp is set to point to the mapped
///* memory and SQLITE_OK returned.
#[allow(unused_doc_comments)]
extern "C" fn unix_shm_map(fd: *mut Sqlite3File, i_region_1: i32,
    sz_region_1: i32, b_extend_1: i32, pp: *mut *mut ()) -> i32 {
    unsafe {
        let mut p_db_fd: *mut UnixFile = core::ptr::null_mut();
        let mut p: *const UnixShm = core::ptr::null();
        let mut p_shm_node: *mut UnixShmNode = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut n_shm_per_map: i32 = 0;
        let mut n_req_region: i32 = 0;
        /// If the shared-memory file has not yet been opened, open it now.
        /// Minimum number of regions required to be mapped.
        let mut ap_new: *mut *mut i8 = core::ptr::null_mut();
        /// New apRegion[] array
        let mut n_byte: i64 = 0 as i64;
        /// Minimum required file size
        let mut s_stat: Stat = unsafe { core::mem::zeroed() };
        let mut i_pg: i64 = 0 as i64;
        /// Write to the last byte of each newly allocated or extended page
        let mut x: i32 = 0;
        let mut z_file: *const i8 = core::ptr::null();
        /// Map the requested memory region into this processes address space.
        let mut n_map: i64 = 0 as i64;
        let mut i: i64 = 0 as i64;
        let mut p_mem: *mut () = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s38:
                {
                match __state {
                    0 => { p_db_fd = fd as *mut UnixFile; __state = 3; }
                    2 => {
                        if unsafe { (*p_shm_node).n_region } as i32 > i_region_1 {
                            __state = 74;
                        } else { __state = 75; }
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { rc = 0; __state = 6; }
                    6 => {
                        n_shm_per_map = unix_shm_region_per_map();
                        __state = 7;
                    }
                    7 => { __state = 8; }
                    8 => {
                        if unsafe { (*p_db_fd).p_shm } == core::ptr::null_mut() {
                            __state = 10;
                        } else { __state = 9; }
                    }
                    9 => { p = unsafe { (*p_db_fd).p_shm }; __state = 13; }
                    10 => {
                        rc = unix_open_shared_memory(p_db_fd);
                        __state = 11;
                    }
                    11 => { if rc != 0 { __state = 12; } else { __state = 9; } }
                    12 => { return rc; }
                    13 => {
                        p_shm_node = unsafe { (*p).p_shm_node };
                        __state = 14;
                    }
                    14 => {
                        unsafe {
                            sqlite3_mutex_enter(unsafe { (*p_shm_node).p_shm_mutex })
                        };
                        __state = 15;
                    }
                    15 => {
                        if unsafe { (*p_shm_node).is_unlocked } != 0 {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => { { let _ = 0; }; __state = 21; }
                    17 => {
                        rc =
                            unix_lock_shared_memory(p_db_fd,
                                unsafe { &mut *p_shm_node });
                        __state = 18;
                    }
                    18 => {
                        if rc != 0 { __state = 20; } else { __state = 19; }
                    }
                    19 => {
                        unsafe { (*p_shm_node).is_unlocked = 0 as u8 };
                        __state = 16;
                    }
                    20 => { __state = 2; }
                    21 => { { let _ = 0; }; __state = 22; }
                    22 => { { let _ = 0; }; __state = 23; }
                    23 => { { let _ = 0; }; __state = 24; }
                    24 => {
                        n_req_region =
                            (i_region_1 + n_shm_per_map) / n_shm_per_map *
                                n_shm_per_map;
                        __state = 25;
                    }
                    25 => {
                        if (unsafe { (*p_shm_node).n_region } as i32) < n_req_region
                            {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => { __state = 2; }
                    27 => { __state = 28; }
                    28 => {
                        n_byte = n_req_region as i64 * sz_region_1 as i64;
                        __state = 29;
                    }
                    29 => { __state = 30; }
                    30 => {
                        unsafe { (*p_shm_node).sz_region = sz_region_1 };
                        __state = 31;
                    }
                    31 => {
                        if unsafe { (*p_shm_node).h_shm } >= 0 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        ap_new =
                            unsafe {
                                    sqlite3_realloc64(unsafe { (*p_shm_node).ap_region } as
                                            *mut (),
                                        n_req_region as u64 *
                                            core::mem::size_of::<*mut i8>() as u64)
                                } as *mut *mut i8;
                        __state = 50;
                    }
                    33 => {
                        if unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(i32, *mut Stat)
                                                        ->
                                                            i32>(a_syscall[5 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(unsafe { (*p_shm_node).h_shm }, &mut s_stat)
                                } != 0 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    34 => {
                        if s_stat.st_size < n_byte {
                            __state = 37;
                        } else { __state = 32; }
                    }
                    35 => { rc = 10 | 19 << 8; __state = 36; }
                    36 => { __state = 2; }
                    37 => {
                        if (b_extend_1 == 0) as i32 != 0 {
                            __state = 38;
                        } else { __state = 39; }
                    }
                    38 => { __state = 2; }
                    39 => { __state = 40; }
                    40 => { __state = 41; }
                    41 => { { let _ = 0; }; __state = 42; }
                    42 => {
                        i_pg = s_stat.st_size / pgsz_1 as OffT;
                        __state = 43;
                    }
                    43 => {
                        if i_pg < n_byte / pgsz_1 as i64 {
                            __state = 44;
                        } else { __state = 32; }
                    }
                    44 => { x = 0; __state = 46; }
                    45 => {
                        { let __p = &mut i_pg; let __t = *__p; *__p += 1; __t };
                        __state = 43;
                    }
                    46 => {
                        if seek_and_write_fd(unsafe { (*p_shm_node).h_shm },
                                    i_pg * pgsz_1 as i64 + pgsz_1 as i64 - 1 as i64,
                                    c"".as_ptr() as *mut i8 as *const (), 1, &mut x) != 1 {
                            __state = 47;
                        } else { __state = 45; }
                    }
                    47 => {
                        z_file = unsafe { (*p_shm_node).z_filename } as *const i8;
                        __state = 48;
                    }
                    48 => {
                        rc =
                            unix_log_error_at_line(10 | 19 << 8,
                                c"write".as_ptr() as *mut i8 as *const i8, z_file, 5184);
                        __state = 49;
                    }
                    49 => { __state = 2; }
                    50 => {
                        if (ap_new).is_null() as i32 != 0 {
                            __state = 52;
                        } else { __state = 51; }
                    }
                    51 => {
                        unsafe { (*p_shm_node).ap_region = ap_new };
                        __state = 54;
                    }
                    52 => { rc = 10 | 12 << 8; __state = 53; }
                    53 => { __state = 2; }
                    54 => {
                        if (unsafe { (*p_shm_node).n_region } as i32) < n_req_region
                            {
                            __state = 55;
                        } else { __state = 26; }
                    }
                    55 => {
                        n_map = sz_region_1 as i64 * n_shm_per_map as i64;
                        __state = 56;
                    }
                    56 => { __state = 57; }
                    57 => { __state = 58; }
                    58 => {
                        if unsafe { (*p_shm_node).h_shm } >= 0 {
                            __state = 60;
                        } else { __state = 61; }
                    }
                    59 => { i = 0 as i64; __state = 70; }
                    60 => {
                        p_mem =
                            unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut (), u64, i32, i32, i32, i64)
                                                    ->
                                                        *mut ()>(a_syscall[22 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(core::ptr::null_mut(), n_map as u64,
                                    if unsafe { (*p_shm_node).is_readonly } != 0 {
                                        1
                                    } else { 1 | 2 }, 1, unsafe { (*p_shm_node).h_shm },
                                    sz_region_1 as i64 *
                                        unsafe { (*p_shm_node).n_region } as i64)
                            };
                        __state = 62;
                    }
                    61 => {
                        p_mem = unsafe { sqlite3_malloc64(n_map as Sqlite3Uint64) };
                        __state = 65;
                    }
                    62 => {
                        if p_mem == -1i32 as *mut () {
                            __state = 63;
                        } else { __state = 59; }
                    }
                    63 => {
                        rc =
                            unix_log_error_at_line(10 | 21 << 8,
                                c"mmap".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_shm_node).z_filename } as *const i8, 5211);
                        __state = 64;
                    }
                    64 => { __state = 2; }
                    65 => {
                        if p_mem == core::ptr::null_mut() {
                            __state = 67;
                        } else { __state = 66; }
                    }
                    66 => {
                        unsafe { memset(p_mem, 0, n_map as u64) };
                        __state = 59;
                    }
                    67 => { rc = 7; __state = 68; }
                    68 => { __state = 2; }
                    69 => {
                        unsafe { (*p_shm_node).n_region += n_shm_per_map as u16 };
                        __state = 54;
                    }
                    70 => {
                        if i < n_shm_per_map as i64 {
                            __state = 71;
                        } else { __state = 69; }
                    }
                    71 => {
                        unsafe {
                            *unsafe {
                                        (*p_shm_node).ap_region.offset((unsafe {
                                                            (*p_shm_node).n_region
                                                        } as i64 + i) as isize)
                                    } =
                                unsafe {
                                    (p_mem as *mut i8).offset((sz_region_1 as i64 * i) as isize)
                                }
                        };
                        __state = 72;
                    }
                    72 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 70;
                    }
                    73 => {
                        if unsafe { (*p_shm_node).is_readonly } != 0 && rc == 0 {
                            __state = 77;
                        } else { __state = 76; }
                    }
                    74 => {
                        unsafe {
                            *pp =
                                unsafe {
                                        *unsafe {
                                                (*p_shm_node).ap_region.offset(i_region_1 as isize)
                                            }
                                    } as *mut ()
                        };
                        __state = 73;
                    }
                    75 => {
                        unsafe { *pp = core::ptr::null_mut() };
                        __state = 73;
                    }
                    76 => {
                        unsafe {
                            sqlite3_mutex_leave(unsafe { (*p_shm_node).p_shm_mutex })
                        };
                        __state = 78;
                    }
                    77 => { rc = 8; __state = 76; }
                    78 => { return rc; }
                    _ => {}
                }
            }
        }

        /// If the shared-memory file has not yet been opened, open it now.
        /// Minimum number of regions required to be mapped.
        /// New apRegion[] array
        /// Minimum required file size
        /// Used by fstat()
        /// The requested region is not mapped into this processes address space.
        ///* Check to see if it has been allocated (i.e. if the wal-index file is
        ///* large enough to contain the requested region).
        /// The requested memory region does not exist. If bExtend is set to
        ///* false, exit early. *pp will be set to NULL and SQLITE_OK returned.
        /// Alternatively, if bExtend is true, extend the file. Do this by
        ///* writing a single byte to the end of each (OS) page being
        ///* allocated or extended. Technically, we need only write to the
        ///* last page in order to extend the file. But writing to all new
        ///* pages forces the OS to allocate them immediately, which reduces
        ///* the chances of SIGBUS while accessing the mapped region later on.
        /// Write to the last byte of each newly allocated or extended page
        /// Map the requested memory region into this processes address space.
        unreachable!();
    }
}

///* Here are all of the sqlite3_io_methods objects for each of the
///* locking strategies.  Functions that return pointers to these methods
///* are also created.
static posix_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 3,
        x_close: Some(unix_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(unix_lock),
        x_unlock: Some(unix_unlock),
        x_check_reserved_lock: Some(unix_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: Some(unix_shm_map),
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

///* This "finder" function attempts to determine the best locking strategy
///* for the database file "filePath".  It then returns the sqlite3_io_methods
///* object that implements that strategy.
///*
///* This is for MacOSX only.
#[allow(unused_doc_comments)]
extern "C" fn autolock_io_finder_impl(file_path_1: *const i8,
    p_new_1: *mut UnixFile) -> *const Sqlite3IoMethods {
    unsafe {
        /// Filesystem type name
        /// Appropriate locking method
        let mut i: i32 = 0;
        let mut fs_info: Statfs = unsafe { core::mem::zeroed() };
        let mut lock_info: Flock = unsafe { core::mem::zeroed() };
        if (file_path_1).is_null() as i32 != 0 {

            /// If filePath==NULL that means we are dealing with a transient file
            ///* that does not need to be locked.
            return &nolock_io_methods;
        }
        if unsafe { statfs(file_path_1, &mut fs_info) } != -1 {
            if fs_info.f_flags & 1 as u32 != 0 { return &nolock_io_methods; }
            {
                i = 0;
                '__b39: loop {
                    if !(!(a_map[i as usize].z_filesystem).is_null()) {
                        break '__b39;
                    }
                    '__c39: loop {
                        if unsafe {
                                    strcmp(&raw mut fs_info.f_fstypename[0 as usize] as *mut i8
                                            as *const i8, a_map[i as usize].z_filesystem)
                                } == 0 {
                            return a_map[i as usize].p_methods;
                        }
                        break '__c39;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }

        /// Default case. Handles, amongst others, "nfs".
        ///* Test byte-range lock using fcntl(). If the call succeeds,
        ///* assume that the file-system supports POSIX style locks.
        (lock_info.l_len = 1 as OffT);
        lock_info.l_start = 0 as OffT;
        lock_info.l_whence = 0 as i16;
        lock_info.l_type = 1 as i16;
        if unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(i32, i32, ...)
                                        ->
                                            i32>(a_syscall[7 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(unsafe { (*p_new_1).h }, 7, &mut lock_info)
                } != -1 {
            if unsafe {
                        strcmp(&raw mut fs_info.f_fstypename[0 as usize] as *mut i8
                                as *const i8, c"nfs".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
                return &nfs_io_methods;
            } else { return &posix_io_methods; }
        } else { return &dotlock_io_methods; }
    }
}

static autolock_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    autolock_io_finder_impl;

/// This variable holds the process id (pid) from when the xRandomness()
///* method was called.  If xOpen() is called from a different process id,
///* indicating that a fork() has occurred, the PRNG will be reset.
static mut randomness_pid: PidT = 0;

///* Find the mode, uid and gid of file zFile.
#[allow(unused_doc_comments)]
extern "C" fn get_file_mode(z_file_1: *const i8, p_mode_1: *mut ModeT,
    p_uid_1: *mut UidT, p_gid_1: *mut GidT) -> i32 {
    unsafe {
        let mut s_stat: Stat = unsafe { core::mem::zeroed() };
        /// Output of stat() on database file
        let mut rc: i32 = 0;
        if 0 ==
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8, *mut Stat)
                                        ->
                                            i32>(a_syscall[4 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(z_file_1, &mut s_stat)
                } {
            unsafe { *p_mode_1 = (s_stat.st_mode as i32 & 511) as ModeT };
            unsafe { *p_uid_1 = s_stat.st_uid };
            unsafe { *p_gid_1 = s_stat.st_gid };
        } else { rc = 10 | 7 << 8; }
        return rc;
    }
}

///* This function is called by unixOpen() to determine the unix permissions
///* to create new files with. If no error occurs, then SQLITE_OK is returned
///* and a value suitable for passing as the third argument to open(2) is
///* written to *pMode. If an IO error occurs, an SQLite error code is
///* returned and the value of *pMode is not modified.
///*
///* In most cases, this routine sets *pMode to 0, which will become
///* an indication to robust_open() to create the file using
///* SQLITE_DEFAULT_FILE_PERMISSIONS adjusted by the umask.
///* But if the file being opened is a WAL or regular journal file, then
///* this function queries the file-system for the permissions on the
///* corresponding database file and sets *pMode to this value. Whenever
///* possible, WAL and journal files are created using the same permissions
///* as the associated database file.
///*
///* If the SQLITE_ENABLE_8_3_NAMES option is enabled, then the
///* original filename is unavailable.  But 8_3_NAMES is only used for
///* FAT filesystems and permissions do not matter there, so just use
///* the default permissions.  In 8_3_NAMES mode, leave *pMode set to zero.
#[allow(unused_doc_comments)]
extern "C" fn find_create_file_mode(z_path_1: *const i8, flags: i32,
    p_mode_1: *mut ModeT, p_uid_1: *mut UidT, p_gid_1: *mut GidT) -> i32 {
    let mut rc: i32 = 0;

    /// Return Code
    unsafe { *p_mode_1 = 0 as ModeT };
    unsafe { *p_uid_1 = 0 as UidT };
    unsafe { *p_gid_1 = 0 as GidT };
    if flags & (524288 | 2048) != 0 {
        let mut z_db: [i8; 513] = [0; 513];
        /// Database file path
        let mut n_db: i32 = 0;

        /// Number of valid bytes in zDb
        /// zPath is a path to a WAL or journal file. The following block derives
        ///* the path to the associated database file from zPath. This block handles
        ///* the following naming conventions:
        ///*
        ///*   "<path to db>-journal"
        ///*   "<path to db>-wal"
        ///*   "<path to db>-journalNN"
        ///*   "<path to db>-walNN"
        ///*
        ///* where NN is a decimal number. The NN naming schemes are
        ///* used by the test_multiplex.c module.
        ///*
        ///* In normal operation, the journal file name will always contain
        ///* a '-' character.  However in 8+3 filename mode, or if a corrupt
        ///* rollback journal specifies a super-journal with a goofy name, then
        ///* the '-' might be missing or the '-' might be the first character in
        ///* the filename.  In that case, just return SQLITE_OK with *pMode==0.
        (n_db = unsafe { sqlite3_strlen30(z_path_1) } - 1);
        while n_db > 0 &&
                unsafe { *z_path_1.offset(n_db as isize) } as i32 !=
                    '.' as i32 {
            if unsafe { *z_path_1.offset(n_db as isize) } as i32 == '-' as i32
                {
                unsafe {
                    memcpy(&raw mut z_db[0 as usize] as *mut i8 as *mut (),
                        z_path_1 as *const (), n_db as u64)
                };
                z_db[n_db as usize] = '\u{0}' as i32 as i8;
                rc =
                    get_file_mode(&raw mut z_db[0 as usize] as *mut i8 as
                            *const i8, p_mode_1, p_uid_1, p_gid_1);
                break;
            }
            { let __p = &mut n_db; let __t = *__p; *__p -= 1; __t };
        }
    } else if flags & 8 != 0 {
        unsafe { *p_mode_1 = 384 as ModeT };
    } else if flags & 64 != 0 {
        /// If this is a main database file and the file was opened using a URI
        ///* filename, check for the "modeof" parameter. If present, interpret
        ///* its value as a filename and try to copy the mode, uid and gid from
        ///* that file.
        let z: *const i8 =
            unsafe {
                sqlite3_uri_parameter(z_path_1,
                    c"modeof".as_ptr() as *mut i8 as *const i8)
            };
        if !(z).is_null() {
            rc = get_file_mode(z, p_mode_1, p_uid_1, p_gid_1);
        }
    }
    return rc;
}

///* Open the file zPath.
///*
///* Previously, the SQLite OS layer used three functions in place of this
///* one:
///*
///*     sqlite3OsOpenReadWrite();
///*     sqlite3OsOpenReadOnly();
///*     sqlite3OsOpenExclusive();
///*
///* These calls correspond to the following combinations of flags:
///*
///*     ReadWrite() ->     (READWRITE | CREATE)
///*     ReadOnly()  ->     (READONLY)
///*     OpenExclusive() -> (READWRITE | CREATE | EXCLUSIVE)
///*
///* The old OpenExclusive() accepted a boolean argument - "delFlag". If
///* true, the file was configured to be automatically deleted when the
///* file handle closed. To achieve the same effect using this new
///* interface, add the DELETEONCLOSE flag to those specified above for
///* OpenExclusive().
#[allow(unused_doc_comments)]
extern "C" fn unix_open(p_vfs_1: *mut Sqlite3Vfs, z_path_1: *const i8,
    p_file_1: *mut Sqlite3File, mut flags: i32, p_out_flags_1: *mut i32)
    -> i32 {
    unsafe {
        let p: *mut UnixFile = p_file_1 as *mut UnixFile;
        /// File descriptor returned by open()
        /// Flags to pass to open()
        /// Type of file to open
        /// True to omit locking primitives
        let mut rc: i32 = 0;
        '__b41: loop {
            '__c41: loop {
                let mut fd: i32 = -1;
                /// File descriptor returned by open()
                let mut open_flags: i32 = 0;
                /// Flags to pass to open()
                let e_type: i32 = flags & 1048320;
                /// Type of file to open
                let mut no_lock: i32 = 0;
                /// True to omit locking primitives
                /// Function Return Code
                let mut ctrl_flags: i32 = 0;
                /// UNIXFILE_* flags
                let is_exclusive: i32 = flags & 16;
                let is_delete: i32 = flags & 8;
                let is_create: i32 = flags & 4;
                let mut is_readonly: i32 = flags & 1;
                let is_read_write: i32 = flags & 2;
                let is_auto_proxy: i32 = flags & 32;
                let mut fs_info: Statfs = unsafe { core::mem::zeroed() };
                /// If creating a super- or main-file journal, this function will open
                ///* a file-descriptor on the directory too. The first time unixSync()
                ///* is called the directory file descriptor will be fsync()ed and close()d.
                let is_new_jrnl: i32 =
                    (is_create != 0 &&
                            (e_type == 16384 || e_type == 2048 || e_type == 524288)) as
                        i32;
                /// If argument zPath is a NULL pointer, this function is required to open
                ///* a temporary file. Use this buffer to store the file name in.
                let mut z_tmpname: [i8; 514] = [0; 514];
                let mut z_name: *const i8 = z_path_1;

                /// Check the following statements are true:
                ///*
                ///*   (a) Exactly one of the READWRITE and READONLY flags must be set, and
                ///*   (b) if CREATE is set, then READWRITE must also be set, and
                ///*   (c) if EXCLUSIVE is set, then CREATE must also be set.
                ///*   (d) if DELETEONCLOSE is set, then CREATE must also be set.
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };

                /// The main DB, main journal, WAL file and super-journal are never
                ///* automatically deleted. Nor are they ever temporary files.
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };

                /// Assert that the upper layer has set one of the "file-type" flags.
                { let _ = 0; };
                if randomness_pid != unsafe { getpid() } as PidT {
                    randomness_pid = unsafe { getpid() } as PidT;
                    unsafe { sqlite3_randomness(0, core::ptr::null_mut()) };
                }
                unsafe {
                    memset(p as *mut (), 0,
                        core::mem::size_of::<UnixFile>() as u64)
                };
                if e_type == 256 {
                    let mut p_unused: *mut UnixUnusedFd = core::ptr::null_mut();
                    p_unused = find_reusable_fd(z_name, flags);
                    if !(p_unused).is_null() {
                        fd = unsafe { (*p_unused).fd };
                    } else {
                        p_unused =
                            unsafe {
                                    sqlite3_malloc64(core::mem::size_of::<UnixUnusedFd>() as
                                            Sqlite3Uint64)
                                } as *mut UnixUnusedFd;
                        if (p_unused).is_null() as i32 != 0 { return 7; }
                    }
                    unsafe { (*p).p_preallocated_unused = p_unused };

                    /// Database filenames are double-zero terminated if they are not
                    ///* URIs with parameters.  Hence, they can always be passed into
                    ///* sqlite3_uri_parameter().
                    { let _ = 0; };
                } else if (z_name).is_null() as i32 != 0 {

                    /// If zName is NULL, the upper layer is requesting a temp file.
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    rc =
                        unix_get_tempname(unsafe { (*p_vfs_1).mx_pathname },
                            &raw mut z_tmpname[0 as usize] as *mut i8);
                    if rc != 0 { return rc; }
                    z_name =
                        &raw mut z_tmpname[0 as usize] as *mut i8 as *const i8;

                    /// Generated temporary filenames are always double-zero terminated
                    ///* for use by sqlite3_uri_parameter().
                    { let _ = 0; };
                }
                if is_readonly != 0 { open_flags |= 0; }
                if is_read_write != 0 { open_flags |= 2; }
                if is_create != 0 { open_flags |= 512; }
                if is_exclusive != 0 { open_flags |= 2048 | 256; }
                open_flags |= 0 | 0 | 256;
                if fd < 0 {
                    let mut open_mode: ModeT = 0 as ModeT;
                    /// Permissions to create file with
                    let mut uid: UidT = 0 as UidT;
                    /// Userid for the file
                    let mut gid: GidT = 0 as GidT;

                    /// Groupid for the file
                    (rc =
                        find_create_file_mode(z_name, flags, &mut open_mode,
                            &mut uid, &mut gid));
                    if rc != 0 { { let _ = 0; }; { let _ = 0; }; return rc; }
                    fd = robust_open(z_name, open_flags, open_mode);
                    { let _ = 0; };
                    if fd < 0 {
                        if is_new_jrnl != 0 &&
                                    unsafe { *unsafe { __error() } } == 13 &&
                                unsafe {
                                        (unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*const i8, i32)
                                                            ->
                                                                i32>(a_syscall[2 as
                                                                        usize].p_current.unwrap_or(unsafe {
                                                                core::mem::transmute::<*const (),
                                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                                            }) as *const ())
                                            })(z_name, 0)
                                    } != 0 {

                            /// If unable to create a journal because the directory is not
                            ///* writable, change the error code to indicate that.
                            (rc = 8 | 6 << 8);
                        } else if unsafe { *unsafe { __error() } } != 21 &&
                                is_read_write != 0 {
                            /// Failed to open the file for read/write access. Try read-only.
                            let mut p_readonly: *mut UnixUnusedFd =
                                core::ptr::null_mut();
                            flags &= !(2 | 4);
                            open_flags &= !(2 | 512);
                            flags |= 1;
                            open_flags |= 0;
                            is_readonly = 1;
                            p_readonly = find_reusable_fd(z_name, flags);
                            if !(p_readonly).is_null() {
                                fd = unsafe { (*p_readonly).fd };
                                unsafe { sqlite3_free(p_readonly as *mut ()) };
                            } else { fd = robust_open(z_name, open_flags, open_mode); }
                        }
                    }
                    if fd < 0 {
                        let rc2: i32 =
                            unix_log_error_at_line(unsafe {
                                    sqlite3_cantopen_error(6722)
                                }, c"open".as_ptr() as *mut i8 as *const i8, z_name, 6722);
                        if rc == 0 { rc = rc2; }
                        break '__b41;
                    }
                    if open_mode != 0 && flags & (524288 | 2048) != 0 {
                        robust_fchown(fd, uid, gid);
                    }
                }
                { let _ = 0; };
                if !(p_out_flags_1).is_null() {
                    unsafe { *p_out_flags_1 = flags };
                }
                if !(unsafe { (*p).p_preallocated_unused }).is_null() {
                    unsafe { (*unsafe { (*p).p_preallocated_unused }).fd = fd };
                    unsafe {
                        (*unsafe { (*p).p_preallocated_unused }).flags =
                            flags & (1 | 2)
                    };
                }
                if is_delete != 0 {
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*const i8)
                                            ->
                                                i32>(a_syscall[16 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(z_name)
                    };
                } else { unsafe { (*p).open_flags = open_flags }; }
                if unsafe { fstatfs(fd, &mut fs_info) } == -1 {
                    store_last_errno(unsafe { &mut *p },
                        unsafe { *unsafe { __error() } });
                    robust_close(p as *const UnixFile, fd, 6776);
                    return 10 | 13 << 8;
                }
                if 0 ==
                        unsafe {
                            strncmp(c"msdos".as_ptr() as *mut i8 as *const i8,
                                &raw mut fs_info.f_fstypename[0 as usize] as *mut i8 as
                                    *const i8, 5 as u64)
                        } {
                    unsafe {
                        (*(p_file_1 as *mut UnixFile)).fs_flags |= 1 as u32
                    };
                }
                if 0 ==
                        unsafe {
                            strncmp(c"exfat".as_ptr() as *mut i8 as *const i8,
                                &raw mut fs_info.f_fstypename[0 as usize] as *mut i8 as
                                    *const i8, 5 as u64)
                        } {
                    unsafe {
                        (*(p_file_1 as *mut UnixFile)).fs_flags |= 1 as u32
                    };
                }
                if is_delete != 0 { ctrl_flags |= 32; }
                if is_readonly != 0 { ctrl_flags |= 2; }
                no_lock = (e_type != 256) as i32;
                if no_lock != 0 { ctrl_flags |= 128; }
                if is_new_jrnl != 0 { ctrl_flags |= 8; }
                if flags & 64 != 0 { ctrl_flags |= 64; }
                if is_auto_proxy != 0 && z_path_1 as *mut () != 0 as *mut ()
                            && (no_lock == 0) as i32 != 0 &&
                        unsafe { (*p_vfs_1).x_open.is_some() } {
                    let envforce: *mut i8 =
                        unsafe {
                            getenv(c"SQLITE_FORCE_PROXY_LOCKING".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                    let mut use_proxy: i32 = 0;
                    if envforce as *mut () != 0 as *mut () {
                        use_proxy =
                            (unsafe { atoi(envforce as *const i8) } > 0) as i32;
                    } else {
                        use_proxy =
                            (fs_info.f_flags & 4096 as u32 == 0) as i32 as i32;
                    }
                    if use_proxy != 0 {
                        rc =
                            fill_in_unix_file(p_vfs_1, fd, p_file_1, z_path_1,
                                ctrl_flags);
                        if rc == 0 {
                            rc =
                                unsafe {
                                    proxy_transform_unix_file(p_file_1 as *mut UnixFile,
                                        c":auto:".as_ptr() as *mut i8 as *const i8)
                                };
                            if rc != 0 {

                                /// Use unixClose to clean up the resources added in fillInUnixFile
                                ///* and clear all the structure's references.  Specifically,
                                ///* pFile->pMethods will be NULL so sqlite3OsClose will be a no-op
                                unix_close(p_file_1);
                                return rc;
                            }
                        }
                        break '__b41;
                    }
                }
                { let _ = 0; };
                rc =
                    fill_in_unix_file(p_vfs_1, fd, p_file_1, z_path_1,
                        ctrl_flags);
                break '__c41;
            }
            if !(false) { break '__b41; }
        }
        if rc != 0 {
            unsafe {
                sqlite3_free(unsafe { (*p).p_preallocated_unused } as *mut ())
            };
        }
        return rc;
    }
}

///* Delete the file at zPath. If the dirSync argument is true, fsync()
///* the directory after deleting the file.
extern "C" fn unix_delete(not_used_1: *mut Sqlite3Vfs, z_path_1: *const i8,
    dir_sync_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = not_used_1; };
        if unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8)
                                        ->
                                            i32>(a_syscall[16 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(z_path_1)
                } == -1 {
            if unsafe { *unsafe { __error() } } == 2 {
                rc = 10 | 23 << 8;
            } else {
                rc =
                    unix_log_error_at_line(10 | 10 << 8,
                        c"unlink".as_ptr() as *mut i8 as *const i8, z_path_1, 6864);
            }
            return rc;
        }
        if dir_sync_1 & 1 != 0 {
            let mut fd: i32 = 0;
            rc =
                unsafe {
                    (unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*const i8, *mut i32)
                                        ->
                                            i32>(a_syscall[17 as
                                                    usize].p_current.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        }) as *const ())
                        })(z_path_1, &mut fd)
                };
            if rc == 0 {
                if full_fsync(fd, 0, 0) != 0 {
                    rc =
                        unix_log_error_at_line(10 | 5 << 8,
                            c"fsync".as_ptr() as *mut i8 as *const i8, z_path_1, 6874);
                }
                robust_close(core::ptr::null(), fd, 6876);
            } else { { let _ = 0; }; rc = 0; }
        }
        return rc;
    }
}

///* Test the existence of or access permissions of file zPath. The
///* test performed depends on the value of flags:
///*
///*     SQLITE_ACCESS_EXISTS: Return 1 if the file exists
///*     SQLITE_ACCESS_READWRITE: Return 1 if the file is read and writable.
///*     SQLITE_ACCESS_READONLY: Return 1 if the file is readable.
///*
///* Otherwise return 0.
#[allow(unused_doc_comments)]
extern "C" fn unix_access(not_used_1: *mut Sqlite3Vfs, z_path_1: *const i8,
    flags: i32, p_res_out_1: *mut i32) -> i32 {
    unsafe {
        { let _ = not_used_1; };
        { let _ = 0; };

        /// The spec says there are three possible values for flags.  But only
        ///* two of them are actually used
        { let _ = 0; };
        if flags == 0 {
            let mut buf: Stat = unsafe { core::mem::zeroed() };
            unsafe {
                *p_res_out_1 =
                    (0 ==
                                unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*const i8, *mut Stat)
                                                        ->
                                                            i32>(a_syscall[4 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(z_path_1, &mut buf)
                                } &&
                            (!(buf.st_mode as i32 & 61440 == 32768) as i32 != 0 ||
                                buf.st_size > 0 as i64)) as i32
            };
        } else {
            unsafe {
                *p_res_out_1 =
                    (unsafe {
                                (unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*const i8, i32)
                                                    ->
                                                        i32>(a_syscall[2 as
                                                                usize].p_current.unwrap_or(unsafe {
                                                        core::mem::transmute::<*const (),
                                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                                    }) as *const ())
                                    })(z_path_1, 1 << 1 | 1 << 2)
                            } == 0) as i32
            };
        }
        return 0;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DbPath {
    rc: i32,
    n_symlink: i32,
    z_out: *mut i8,
    n_out: i32,
    n_used: i32,
}

///* Append a single path element to the DbPath under construction
extern "C" fn append_one_path_element(p_path_1: *mut DbPath,
    z_name_1: *const i8, n_name_1: i32) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { *z_name_1.offset(0 as isize) } as i32 == '.' as i32 {
            if n_name_1 == 1 { return; }
            if unsafe { *z_name_1.offset(1 as isize) } as i32 == '.' as i32 &&
                    n_name_1 == 2 {
                if unsafe { (*p_path_1).n_used } > 1 {
                    { let _ = 0; };
                    while unsafe {
                                    *unsafe {
                                            (*p_path_1).z_out.offset({
                                                        let __p = unsafe { &mut (*p_path_1).n_used };
                                                        *__p -= 1;
                                                        *__p
                                                    } as isize)
                                        }
                                } as i32 != '/' as i32 {}
                }
                return;
            }
        }
        if unsafe { (*p_path_1).n_used } + n_name_1 + 2 >=
                unsafe { (*p_path_1).n_out } {
            unsafe { (*p_path_1).rc = 1 };
            return;
        }
        unsafe {
            *unsafe {
                        (*p_path_1).z_out.offset({
                                    let __p = unsafe { &mut (*p_path_1).n_used };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                    } = '/' as i32 as i8
        };
        unsafe {
            memcpy(unsafe {
                        &raw mut *unsafe {
                                    (*p_path_1).z_out.offset(unsafe { (*p_path_1).n_used } as
                                            isize)
                                }
                    } as *mut (), z_name_1 as *const (), n_name_1 as u64)
        };
        unsafe { (*p_path_1).n_used += n_name_1 };
        if unsafe { (*p_path_1).rc } == 0 {
            let mut z_in: *const i8 = core::ptr::null();
            let mut buf: Stat = unsafe { core::mem::zeroed() };
            unsafe {
                *unsafe {
                            (*p_path_1).z_out.offset(unsafe { (*p_path_1).n_used } as
                                    isize)
                        } = 0 as i8
            };
            z_in = unsafe { (*p_path_1).z_out } as *const i8;
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*const i8, *mut Stat)
                                            ->
                                                i32>(a_syscall[27 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(z_in, &mut buf)
                    } != 0 {
                if unsafe { *unsafe { __error() } } != 2 {
                    unsafe {
                        (*p_path_1).rc =
                            unix_log_error_at_line(unsafe {
                                    sqlite3_cantopen_error(6970)
                                }, c"lstat".as_ptr() as *mut i8 as *const i8, z_in, 6970)
                    };
                }
            } else if buf.st_mode as i32 & 61440 == 40960 {
                let mut got: i64 = 0 as i64;
                let mut z_lnk: [i8; 1026] = [0; 1026];
                if {
                            let __p = unsafe { &mut (*p_path_1).n_symlink };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        } > 200 {
                    unsafe {
                        (*p_path_1).rc = unsafe { sqlite3_cantopen_error(6976) }
                    };
                    return;
                }
                got =
                    unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*const i8, *mut i8, u64)
                                            ->
                                                i64>(a_syscall[26 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(z_in, &raw mut z_lnk[0 as usize] as *mut i8,
                            core::mem::size_of::<[i8; 1026]>() as u64 - 2 as u64)
                    };
                if got <= 0 as i64 ||
                        got >= core::mem::size_of::<[i8; 1026]>() as i64 - 2 as i64
                    {
                    unsafe {
                        (*p_path_1).rc =
                            unix_log_error_at_line(unsafe {
                                    sqlite3_cantopen_error(6981)
                                }, c"readlink".as_ptr() as *mut i8 as *const i8, z_in, 6981)
                    };
                    return;
                }
                z_lnk[got as usize] = 0 as i8;
                if z_lnk[0 as usize] as i32 == '/' as i32 {
                    unsafe { (*p_path_1).n_used = 0 };
                } else { unsafe { (*p_path_1).n_used -= n_name_1 + 1 }; }
                unsafe {
                    append_all_path_elements(p_path_1,
                        &raw mut z_lnk[0 as usize] as *mut i8 as *const i8)
                };
            }
        }
    }
}

/// Forward reference
extern "C" fn append_all_path_elements(p_path_1: *mut DbPath,
    z_path_1: *const i8) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    '__b43: loop {
        '__c43: loop {
            while unsafe { *z_path_1.offset(i as isize) } != 0 &&
                    unsafe { *z_path_1.offset(i as isize) } as i32 != '/' as i32
                {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
            if i > j {
                append_one_path_element(p_path_1,
                    unsafe { &*z_path_1.offset(j as isize) }, i - j);
            }
            j = i + 1;
            break '__c43;
        }
        if !(unsafe {
                            *z_path_1.offset({
                                            let __p = &mut i;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize)
                        } != 0) {
            break '__b43;
        }
    }
}

///* Turn a relative pathname into a full pathname. The relative path
///* is stored as a nul-terminated string in the buffer pointed to by
///* zPath.
///*
///* zOut points to a buffer of at least sqlite3_vfs.mxPathname bytes
///* (in this case, MAX_PATHNAME bytes). The full-path is written to
///* this buffer before returning.
extern "C" fn unix_full_pathname(p_vfs_1: *mut Sqlite3Vfs,
    z_path_1: *const i8, n_out_1: i32, z_out_1: *mut i8) -> i32 {
    unsafe {
        let mut path: DbPath = unsafe { core::mem::zeroed() };
        { let _ = p_vfs_1; };
        path.rc = 0;
        path.n_used = 0;
        path.n_symlink = 0;
        path.n_out = n_out_1;
        path.z_out = z_out_1;
        if unsafe { *z_path_1.offset(0 as isize) } as i32 != '/' as i32 {
            let mut z_pwd: [i8; 1026] = [0; 1026];
            if unsafe {
                        (unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut i8, u64)
                                            ->
                                                *mut i8>(a_syscall[3 as
                                                        usize].p_current.unwrap_or(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn() -> ()>(0 as *const ())
                                            }) as *const ())
                            })(&raw mut z_pwd[0 as usize] as *mut i8,
                            core::mem::size_of::<[i8; 1026]>() as u64 - 2 as u64)
                    } == core::ptr::null_mut() {
                return unix_log_error_at_line(unsafe {
                            sqlite3_cantopen_error(7039)
                        }, c"getcwd".as_ptr() as *mut i8 as *const i8, z_path_1,
                        7039);
            }
            append_all_path_elements(&mut path,
                &raw mut z_pwd[0 as usize] as *mut i8 as *const i8);
        }
        append_all_path_elements(&mut path, z_path_1);
        unsafe { *z_out_1.offset(path.n_used as isize) = 0 as i8 };
        if path.rc != 0 || path.n_used < 2 {
            return unsafe { sqlite3_cantopen_error(7045) };
        }
        if path.n_symlink != 0 { return 0 | 2 << 8; }
        return 0;
    }
}

extern "C" fn unix_dl_open(not_used_1: *mut Sqlite3Vfs,
    z_filename_1: *const i8) -> *mut () {
    { let _ = not_used_1; };
    return unsafe { dlopen(z_filename_1, 2 | 8) };
}

///* SQLite calls this function immediately after a call to unixDlSym() or
///* unixDlOpen() fails (returns a null pointer). If a more detailed error
///* message is available, it is written to zBufOut. If no error message
///* is available, zBufOut is left unmodified and SQLite uses a default
///* error message.
extern "C" fn unix_dl_error(not_used_1: *mut Sqlite3Vfs, n_buf_1: i32,
    z_buf_out_1: *mut i8) -> () {
    let mut z_err: *const i8 = core::ptr::null();
    { let _ = not_used_1; };
    unix_enter_mutex();
    z_err = unsafe { dlerror() } as *const i8;
    if !(z_err).is_null() {
        unsafe {
            sqlite3_snprintf(n_buf_1, z_buf_out_1,
                c"%s".as_ptr() as *mut i8 as *const i8, z_err)
        };
    }
    unix_leave_mutex();
}

#[allow(unused_doc_comments)]
extern "C" fn unix_dl_sym(not_used_1: *mut Sqlite3Vfs, p: *mut (),
    z_sym_1: *const i8) -> unsafe extern "C" fn() -> () {
    ///* GCC with -pedantic-errors says that C90 does not allow a void* to be
    ///* cast into a pointer to a function.  And yet the library dlsym() routine
    ///* returns a void* which is really a pointer to a function.  So how do we
    ///* use dlsym() with -pedantic-errors?
    ///*
    ///* Variable x below is defined to be a pointer to a function taking
    ///* parameters void* and const char* and returning a pointer to a function.
    ///* We initialize x by assigning it a pointer to the dlsym() function.
    ///* (That assignment requires a cast.)  Then we call the function that
    ///* x points to.
    ///*
    ///* This work-around is unlikely to work correctly on any system where
    ///* you really cannot cast a function pointer into void*.  But then, on the
    ///* other hand, dlsym() will not work on such a system either, so we have
    ///* not really lost anything.
    let mut x:
            Option<unsafe extern "C" fn(*mut (), *const i8)
                -> unsafe extern "C" fn() -> ()> = None;
    { let _ = not_used_1; };
    x =
        Some(unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), *const i8)
                            -> unsafe extern "C" fn() -> ()>(dlsym as *const ())
            });
    return unsafe { x.unwrap()(p, z_sym_1) };
}

extern "C" fn unix_dl_close(not_used_1: *mut Sqlite3Vfs, p_handle_1: *mut ())
    -> () {
    { let _ = not_used_1; };
    unsafe { dlclose(p_handle_1) };
}

///* Write nBuf bytes of random data to the supplied buffer zBuf.
#[allow(unused_doc_comments)]
extern "C" fn unix_randomness(not_used_1: *mut Sqlite3Vfs, mut n_buf_1: i32,
    z_buf_1: *mut i8) -> i32 {
    unsafe {
        { let _ = not_used_1; };
        { let _ = 0; };

        /// We have to initialize zBuf to prevent valgrind from reporting
        ///* errors.  The reports issued by valgrind are incorrect - we would
        ///* prefer that the randomness be increased by making use of the
        ///* uninitialized space in zBuf - but valgrind errors tend to worry
        ///* some users.  Rather than argue, it seems easier just to initialize
        ///* the whole array and silence valgrind, even if that means less randomness
        ///* in the random seed.
        ///*
        ///* When testing, initializing zBuf[] to zero is all we do.  That means
        ///* that we always use the same random number sequence.  This makes the
        ///* tests repeatable.
        unsafe { memset(z_buf_1 as *mut (), 0, n_buf_1 as u64) };
        randomness_pid = unsafe { getpid() } as PidT;
        {
            let mut fd: i32 = 0;
            let mut got: i32 = 0;
            fd =
                robust_open(c"/dev/urandom".as_ptr() as *mut i8 as *const i8,
                    0, 0 as ModeT);
            if fd < 0 {
                let mut t: TimeT = 0 as TimeT;
                unsafe { time(&mut t) };
                unsafe {
                    memcpy(z_buf_1 as *mut (), &raw mut t as *const (),
                        core::mem::size_of::<TimeT>() as u64)
                };
                unsafe {
                    memcpy(unsafe {
                                &raw mut *z_buf_1.add(core::mem::size_of::<TimeT>() as
                                                usize)
                            } as *mut (), &raw mut randomness_pid as *const (),
                        core::mem::size_of::<PidT>() as u64)
                };
                { let _ = 0; };
                n_buf_1 =
                    (core::mem::size_of::<TimeT>() as u64 +
                            core::mem::size_of::<PidT>() as u64) as i32;
            } else {
                '__b45: loop {
                    '__c45: loop {
                        got =
                            unsafe {
                                    (unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(i32, *mut (), u64)
                                                        ->
                                                            i64>(a_syscall[8 as
                                                                    usize].p_current.unwrap_or(unsafe {
                                                            core::mem::transmute::<*const (),
                                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                                        }) as *const ())
                                        })(fd, z_buf_1 as *mut (), n_buf_1 as u64)
                                } as i32;
                        break '__c45;
                    }
                    if !(got < 0 && unsafe { *unsafe { __error() } } == 4) {
                        break '__b45;
                    }
                }
                robust_close(core::ptr::null(), fd, 7146);
            }
        }
        return n_buf_1;
    }
}

///* Find the current time (in Universal Coordinated Time).  Write into *piNow
///* the current time and date as a Julian Day number times 86_400_000.  In
///* other words, write into *piNow the number of milliseconds since the Julian
///* epoch of noon in Greenwich on November 24, 4714 B.C according to the
///* proleptic Gregorian calendar.
///*
///* On success, return SQLITE_OK.  Return SQLITE_ERROR if the time and date
///* cannot be found.
#[allow(unused_doc_comments)]
extern "C" fn unix_current_time_int64(not_used_1: *mut Sqlite3Vfs,
    pi_now_1: *mut Sqlite3Int64) -> i32 {
    let rc: i32 = 0;
    let mut s_now: Timeval = unsafe { core::mem::zeroed() };
    { let _ = unsafe { gettimeofday(&mut s_now, core::ptr::null_mut()) }; };

    /// Cannot fail given valid arguments
    unsafe {
        *pi_now_1 =
            unix_epoch + 1000 as Sqlite3Int64 * s_now.tv_sec as Sqlite3Int64 +
                (s_now.tv_usec / 1000) as Sqlite3Int64
    };
    { let _ = not_used_1; };
    return rc;
}

///* Find the current time (in Universal Coordinated Time).  Write the
///* current time and date as a Julian Day number into *prNow and
///* return 0.  Return 1 if the time and date cannot be found.
extern "C" fn unix_current_time(not_used_1: *mut Sqlite3Vfs,
    pr_now_1: *mut f64) -> i32 {
    let mut i: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut rc: i32 = 0;
    { let _ = not_used_1; };
    rc = unix_current_time_int64(core::ptr::null_mut(), &mut i);
    unsafe { *pr_now_1 = i as f64 / 86400000.0 };
    return rc;
}

///* The xGetLastError() method is designed to return a better
///* low-level error message when operating-system problems come up
///* during SQLite operation.  Only the integer return code is currently
///* used.
extern "C" fn unix_get_last_error(not_used_1: *mut Sqlite3Vfs,
    not_used2_1: i32, not_used3_1: *mut i8) -> i32 {
    { let _ = not_used_1; };
    { let _ = not_used2_1; };
    { let _ = not_used3_1; };
    return unsafe { *unsafe { __error() } };
}

///* This is the xSetSystemCall() method of sqlite3_vfs for all of the
///* "unix" VFSes.  Return SQLITE_OK upon successfully updating the
///* system call pointer, or SQLITE_NOTFOUND if there is no configurable
///* system call named zName.
#[allow(unused_doc_comments)]
extern "C" fn unix_set_system_call(p_not_used_1: *mut Sqlite3Vfs,
    z_name_1: *const i8, mut p_new_func_1: unsafe extern "C" fn() -> ())
    -> i32 {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut rc: i32 = 12;
        { let _ = p_not_used_1; };
        if z_name_1 == core::ptr::null() {

            /// If no zName is given, restore all system calls to their default
            ///* settings and return NULL
            (rc = 0);
            {
                i = 0 as u32;
                '__b46: loop {
                    if !((i as u64) <
                                    core::mem::size_of::<[UnixSyscall; 29]>() as u64 /
                                        core::mem::size_of::<UnixSyscall>() as u64) {
                        break '__b46;
                    }
                    '__c46: loop {
                        if a_syscall[i as usize].p_default.is_some() {
                            a_syscall[i as usize].p_current =
                                a_syscall[i as usize].p_default;
                        }
                        break '__c46;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else {
            {
                i = 0 as u32;
                '__b47: loop {
                    if !((i as u64) <
                                    core::mem::size_of::<[UnixSyscall; 29]>() as u64 /
                                        core::mem::size_of::<UnixSyscall>() as u64) {
                        break '__b47;
                    }
                    '__c47: loop {
                        if unsafe { strcmp(z_name_1, a_syscall[i as usize].z_name) }
                                == 0 {
                            if !a_syscall[i as usize].p_default.is_some() as i32 != 0 {
                                a_syscall[i as usize].p_default =
                                    a_syscall[i as usize].p_current;
                            }
                            rc = 0;
                            if p_new_func_1 ==
                                    unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn() -> ()>(0 as *const ())
                                    } {
                                p_new_func_1 =
                                    a_syscall[i as
                                                    usize].p_default.unwrap_or(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        });
                            }
                            a_syscall[i as usize].p_current =
                                if p_new_func_1 !=
                                        unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn() -> ()>(0 as *const ())
                                        } {
                                    Some(p_new_func_1)
                                } else { None };
                            break '__b47;
                        }
                        break '__c47;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return rc;
    }
}

///* Return the value of a system call.  Return NULL if zName is not a
///* recognized system call name.  NULL is also returned if the system call
///* is currently undefined.
extern "C" fn unix_get_system_call(p_not_used_1: *mut Sqlite3Vfs,
    z_name_1: *const i8) -> unsafe extern "C" fn() -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        { let _ = p_not_used_1; };
        {
            i = 0 as u32;
            '__b48: loop {
                if !((i as u64) <
                                core::mem::size_of::<[UnixSyscall; 29]>() as u64 /
                                    core::mem::size_of::<UnixSyscall>() as u64) {
                    break '__b48;
                }
                '__c48: loop {
                    if unsafe { strcmp(z_name_1, a_syscall[i as usize].z_name) }
                            == 0 {
                        return a_syscall[i as
                                            usize].p_current.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn() -> ()>(0 as *const ())
                                });
                    }
                    break '__c48;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn() -> ()>(0 as *const ())
            };
    }
}

///* Return the name of the first system call after zName.  If zName==NULL
///* then return the name of the first system call.  Return NULL if zName
///* is the last system call or if zName is not the name of a valid
///* system call.
extern "C" fn unix_next_system_call(p: *mut Sqlite3Vfs, z_name_1: *const i8)
    -> *const i8 {
    unsafe {
        let mut i: i32 = -1;
        { let _ = p; };
        if !(z_name_1).is_null() {
            {
                i = 0;
                '__b49: loop {
                    if !(i <
                                    (core::mem::size_of::<[UnixSyscall; 29]>() as u64 /
                                                core::mem::size_of::<UnixSyscall>() as u64) as i32 - 1) {
                        break '__b49;
                    }
                    '__c49: loop {
                        if unsafe { strcmp(z_name_1, a_syscall[i as usize].z_name) }
                                == 0 {
                            break '__b49;
                        }
                        break '__c49;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        {
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            '__b50: loop {
                if !(i <
                                (core::mem::size_of::<[UnixSyscall; 29]>() as u64 /
                                        core::mem::size_of::<UnixSyscall>() as u64) as i32) {
                    break '__b50;
                }
                '__c50: loop {
                    if a_syscall[i as usize].p_current.is_some() {
                        return a_syscall[i as usize].z_name;
                    }
                    break '__c50;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return core::ptr::null();
    }
}

extern "C" fn nolock_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &nolock_io_methods;
}

static nolock_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    nolock_io_finder_impl;

extern "C" fn dotlock_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &dotlock_io_methods;
}

static dotlock_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    dotlock_io_finder_impl;

///* Here are all of the sqlite3_io_methods objects for each of the
///* locking strategies.  Functions that return pointers to these methods
///* are also created.
extern "C" fn posix_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &posix_io_methods;
}

///* Here are all of the sqlite3_io_methods objects for each of the
///* locking strategies.  Functions that return pointers to these methods
///* are also created.
static posix_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    posix_io_finder_impl;

extern "C" fn robust_flock(fd: i32, op: i32) -> i32 {
    let mut rc: i32 = 0;
    '__b51: loop {
        '__c51: loop { rc = unsafe { flock(fd, op) }; break '__c51; }
        if !(rc < 0 && unsafe { *unsafe { __error() } } == 4) {
            break '__b51;
        }
    }
    return rc;
}

///* Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
///* must be either NO_LOCK or SHARED_LOCK.
///*
///* If the locking level of the file descriptor is already at or below
///* the requested locking level, this routine is a no-op.
#[allow(unused_doc_comments)]
extern "C" fn flock_unlock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    let p_file: *mut UnixFile = id as *mut UnixFile;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_file).e_file_lock } as i32 == e_file_lock_1 { return 0; }
    if e_file_lock_1 == 1 {
        unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };
        return 0;
    }
    if robust_flock(unsafe { (*p_file).h }, 8) != 0 {

        /// SQLITE_IGNORE_FLOCK_LOCK_ERRORS
        return 10 | 8 << 8;
    } else { unsafe { (*p_file).e_file_lock = 0 as u8 }; return 0; }
}

///* Close a file.
extern "C" fn flock_close(id: *mut Sqlite3File) -> i32 {
    { let _ = 0; };
    flock_unlock(id, 0);
    return close_unix_file(id);
}

///* Lock the file with the lock specified by parameter eFileLock - one
///* of the following:
///*
///*     (1) SHARED_LOCK
///*     (2) RESERVED_LOCK
///*     (3) PENDING_LOCK
///*     (4) EXCLUSIVE_LOCK
///*
///* Sometimes when requesting one lock state, additional lock states
///* are inserted in between.  The locking might fail on one of the later
///* transitions leaving the lock state different from what it started but
///* still short of its goal.  The following chart shows the allowed
///* transitions and the inserted intermediate states:
///*
///*    UNLOCKED -> SHARED
///*    SHARED -> RESERVED
///*    SHARED -> (PENDING) -> EXCLUSIVE
///*    RESERVED -> (PENDING) -> EXCLUSIVE
///*    PENDING -> EXCLUSIVE
///*
///* flock() only really support EXCLUSIVE locks.  We track intermediate
///* lock states in the sqlite3_file structure, but all locks SHARED or
///* above are really EXCLUSIVE locks and exclude all other processes from
///* access the file.
///*
///* This routine will only increase a lock.  Use the sqlite3OsUnlock()
///* routine to lower a locking level.
#[allow(unused_doc_comments)]
extern "C" fn flock_lock(id: *mut Sqlite3File, e_file_lock_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let p_file: *mut UnixFile = id as *mut UnixFile;
    { let _ = 0; };
    if unsafe { (*p_file).e_file_lock } as i32 > 0 {
        unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };
        return 0;
    }
    if robust_flock(unsafe { (*p_file).h }, 2 | 4) != 0 {
        let t_errno: i32 = unsafe { *unsafe { __error() } };

        /// didn't get, must be busy
        (rc = sqlite_error_from_posix_error(t_errno, 10 | 15 << 8));
        if rc != 0 && rc != 5 {
            store_last_errno(unsafe { &mut *p_file }, t_errno);
        }
    } else {

        /// got it, set the type and return ok
        unsafe { (*p_file).e_file_lock = e_file_lock_1 as u8 };
    }

    /// SQLITE_IGNORE_FLOCK_LOCK_ERRORS
    return rc;
}

///* This routine checks if there is a RESERVED lock held on the specified
///* file by this or any other process. If such a lock is held, set *pResOut
///* to a non-zero value otherwise *pResOut is set to zero.  The return value
///* is set to SQLITE_OK unless an I/O error occurs during lock checking.
#[allow(unused_doc_comments)]
extern "C" fn flock_check_reserved_lock(id: *mut Sqlite3File,
    p_res_out_1: *mut i32) -> i32 {
    { let _ = id; };
    { let _ = 0; };
    { let _ = 0; };

    /// The flock VFS only ever takes exclusive locks (see function flockLock).
    ///* Therefore, if this connection is holding any lock at all, no other
    ///* connection may be holding a RESERVED lock. So set *pResOut to 0
    ///* in this case.
    ///*
    ///* Or, this connection may be holding no lock. In that case, set *pResOut to
    ///* 0 as well. The caller will then attempt to take an EXCLUSIVE lock on the
    ///* db in order to roll the hot journal back. If there is another connection
    ///* holding a lock, that attempt will fail and an SQLITE_BUSY returned to
    ///* the user. With other VFS, we try to avoid this, in order to allow a reader
    ///* to proceed while a writer is preparing its transaction. But that won't
    ///* work with the flock VFS - as it always takes EXCLUSIVE locks - so it is
    ///* not a problem in this case.
    unsafe { *p_res_out_1 = 0 };
    return 0;
}

static flock_io_methods: Sqlite3IoMethods =
    Sqlite3IoMethods {
        i_version: 1,
        x_close: Some(flock_close),
        x_read: Some(unix_read),
        x_write: Some(unix_write),
        x_truncate: Some(unix_truncate),
        x_sync: Some(unix_sync),
        x_file_size: Some(unix_file_size),
        x_lock: Some(flock_lock),
        x_unlock: Some(flock_unlock),
        x_check_reserved_lock: Some(flock_check_reserved_lock),
        x_file_control: Some(unix_file_control),
        x_sector_size: Some(unix_sector_size),
        x_device_characteristics: Some(unix_device_characteristics),
        x_shm_map: None,
        x_shm_lock: Some(unix_shm_lock),
        x_shm_barrier: Some(unix_shm_barrier),
        x_shm_unmap: Some(unix_shm_unmap),
        x_fetch: Some(unix_fetch),
        x_unfetch: Some(unix_unfetch),
    };

extern "C" fn flock_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &flock_io_methods;
}

static flock_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    flock_io_finder_impl;

extern "C" fn afp_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &afp_io_methods;
}

static afp_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    afp_io_finder_impl;

extern "C" fn nfs_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &nfs_io_methods;
}

static nfs_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    nfs_io_finder_impl;

extern "C" fn proxy_io_finder_impl(z: *const i8, p: *mut UnixFile)
    -> *const Sqlite3IoMethods {
    { let _ = z; };
    { let _ = p; };
    return &proxy_io_methods;
}

static proxy_io_finder:
    unsafe extern "C" fn(*const i8, *mut UnixFile) -> *const Sqlite3IoMethods
    =
    proxy_io_finder_impl;

///* Initialize first two members of azTempDirs[] array.
extern "C" fn unix_temp_file_init() -> () {
    unsafe {
        az_temp_dirs[0 as usize] =
            unsafe {
                    getenv(c"SQLITE_TMPDIR".as_ptr() as *mut i8 as *const i8)
                } as *const i8;
        az_temp_dirs[1 as usize] =
            unsafe { getenv(c"TMPDIR".as_ptr() as *mut i8 as *const i8) } as
                *const i8;
    }
}

///* Initialize the operating system interface.
///*
///* This routine registers all VFS implementations for unix-like operating
///* systems.  This routine, and the sqlite3_os_end() routine that follows,
///* should be the only routines in this file that are visible from other
///* files.
///*
///* This routine is called once during SQLite initialization and by a
///* single thread.  The memory allocation and mutex subsystems have not
///* necessarily been initialized when this routine is called, and so they
///* should not be used.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_os_init() -> i32 {
    unsafe {
        let mut i: u32 = 0 as u32;

        /// Loop counter
        /// Double-check that the aSyscall[] array has been constructed
        ///* correctly.  See ticket [bb3a86e890c8e96ab]
        { let _ = 0; };
        {
            i = 0 as u32;
            '__b52: loop {
                if !((i as u64) <
                                core::mem::size_of::<[Sqlite3Vfs; 9]>() as u64 /
                                    core::mem::size_of::<Sqlite3Vfs>() as u64) {
                    break '__b52;
                }
                '__c52: loop {
                    unsafe {
                        sqlite3_vfs_register(&mut a_vfs[i as usize],
                            (i == 0 as u32) as i32)
                    };
                    break '__c52;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unix_big_lock = unsafe { sqlite3MutexAlloc(11) };

        /// Validate lock assumptions
        { let _ = 0; };

        /// Number of available locks
        { let _ = 0; };

        /// Start of locking area */
        ///  /* Locks:
        ///*    WRITE       UNIX_SHM_BASE      120
        ///*    CKPT        UNIX_SHM_BASE+1    121
        ///*    RECOVER     UNIX_SHM_BASE+2    122
        ///*    READ-0      UNIX_SHM_BASE+3    123
        ///*    READ-1      UNIX_SHM_BASE+4    124
        ///*    READ-2      UNIX_SHM_BASE+5    125
        ///*    READ-3      UNIX_SHM_BASE+6    126
        ///*    READ-4      UNIX_SHM_BASE+7    127
        ///*    DMS         UNIX_SHM_BASE+8    128
        { let _ = 0; };

        /// Byte offset of the deadman-switch
        /// Initialize temp file dir array.
        unix_temp_file_init();
        return 0;
    }
}

///* Shutdown the operating system interface.
///*
///* Some operating systems might need to do some cleanup in this routine,
///* to release dynamically allocated objects.  But not on unix.
///* This routine is a no-op for unix.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_os_end() -> i32 {
    unsafe { unix_big_lock = core::ptr::null_mut(); return 0; }
}

///***************************************************************************
///***************** Begin Unique File ID Utility Used By VxWorks ***************
///*
///* On most versions of unix, we can get a unique ID for a file by concatenating
///* the device number and the inode number.  But this does not work on VxWorks.
///* On VxWorks, a unique file id must be based on the canonical filename.
///*
///* A pointer to an instance of the following structure can be used as a
///* unique file ID in VxWorks.  Each instance of this structure contains
///* a copy of the canonical filename.  There is also a reference count.
///* The structure is reclaimed when the number of pointers to it drops to
///* zero.
///*
///* There are never very many files open at one time and lookups are not
///* a performance-critical path, so it is sufficient to put these
///* structures on a linked list.
#[repr(C)]
#[derive(Copy, Clone)]
struct VxworksFileId {
    p_next: *mut VxworksFileId,
    n_ref: i32,
    n_name: i32,
    z_canonical_name: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MappingN7Mapping {
    z_filesystem: *const i8,
    p_methods: *const Sqlite3IoMethods,
}

static pgsz_1: i32 = 4096 as i32;

static mut a_map: [MappingN7Mapping; 6] =
    [MappingN7Mapping {
                z_filesystem: c"hfs".as_ptr() as *const i8,
                p_methods: &posix_io_methods,
            },
            MappingN7Mapping {
                z_filesystem: c"ufs".as_ptr() as *const i8,
                p_methods: &posix_io_methods,
            },
            MappingN7Mapping {
                z_filesystem: c"afpfs".as_ptr() as *const i8,
                p_methods: &afp_io_methods,
            },
            MappingN7Mapping {
                z_filesystem: c"smbfs".as_ptr() as *const i8,
                p_methods: &afp_io_methods,
            },
            MappingN7Mapping {
                z_filesystem: c"webdav".as_ptr() as *const i8,
                p_methods: &nolock_io_methods,
            },
            MappingN7Mapping {
                z_filesystem: core::ptr::null(),
                p_methods: core::ptr::null(),
            }];

static unix_epoch: Sqlite3Int64 =
    (24405875 as Sqlite3Int64 * 8640000 as Sqlite3Int64) as Sqlite3Int64;

static mut a_vfs: [Sqlite3Vfs; 9] =
    [Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix".as_ptr() as *const i8,
                p_app_data: &raw const autolock_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-none".as_ptr() as *const i8,
                p_app_data: &raw const nolock_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-dotfile".as_ptr() as *const i8,
                p_app_data: &raw const dotlock_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-excl".as_ptr() as *const i8,
                p_app_data: &raw const posix_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-posix".as_ptr() as *const i8,
                p_app_data: &raw const posix_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-flock".as_ptr() as *const i8,
                p_app_data: &raw const flock_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-afp".as_ptr() as *const i8,
                p_app_data: &raw const afp_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-nfs".as_ptr() as *const i8,
                p_app_data: &raw const nfs_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            },
            Sqlite3Vfs {
                i_version: 3,
                sz_os_file: core::mem::size_of::<UnixFile>() as i32,
                mx_pathname: 512,
                p_next: core::ptr::null_mut(),
                z_name: c"unix-proxy".as_ptr() as *const i8,
                p_app_data: &raw const proxy_io_finder as *mut (),
                x_open: Some(unix_open),
                x_delete: Some(unix_delete),
                x_access: Some(unix_access),
                x_full_pathname: Some(unix_full_pathname),
                x_dl_open: Some(unix_dl_open),
                x_dl_error: Some(unix_dl_error),
                x_dl_sym: Some(unix_dl_sym),
                x_dl_close: Some(unix_dl_close),
                x_randomness: Some(unix_randomness),
                x_sleep: Some(unix_sleep),
                x_current_time: Some(unix_current_time),
                x_get_last_error: Some(unix_get_last_error),
                x_current_time_int64: Some(unix_current_time_int64),
                x_set_system_call: Some(unix_set_system_call),
                x_get_system_call: Some(unix_get_system_call),
                x_next_system_call: Some(unix_next_system_call),
            }];

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
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn sqlite3_snprintf(_: i32, _: *mut i8, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_log(i_err_code_1: i32, z_format_1: *const i8, ...)
    -> ();
    fn __error()
    -> *mut i32;
    fn sqlite3_cantopen_error(_: i32)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn sysconf(_: i32)
    -> i64;
    fn sqlite3_mutex_enter(_: *mut Sqlite3Mutex)
    -> ();
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_mutex_leave(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_mutex_free(_: *mut Sqlite3Mutex)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fsync(_: i32)
    -> i32;
    fn sqlite3_mprintf(_: *const i8, ...)
    -> *mut i8;
    fn sqlite3_malloc64(_: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3MutexAlloc(_: i32)
    -> *mut Sqlite3Mutex;
    static mut sqlite3_temp_directory: *mut i8;
    fn sqlite3_randomness(n_1: i32, p_1: *mut ())
    -> ();
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn gethostuuid(id: *mut u8, wait: *const Timespec)
    -> i32;
    fn nanosleep(__rqtp: *const Timespec, __rmtp: *mut Timespec)
    -> i32;
    fn strlcpy(__dst: *mut i8, __source: *const i8, __size: u64)
    -> u64;
    fn rename(__old: *const i8, __new: *const i8)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn confstr(_: i32, _: *mut i8, __len: u64)
    -> u64;
    fn strlcat(__dst: *mut i8, __source: *const i8, __size: u64)
    -> u64;
    fn sqlite3_uri_boolean(z: Sqlite3Filename, z_param_1: *const i8,
    b_default_1: i32)
    -> i32;
    fn sqlite3_memory_barrier()
    -> ();
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn fsctl(_: *const i8, _: u64, _: *mut (), _: u32)
    -> i32;
    fn random()
    -> i64;
    fn utime(_: *const i8, _: *const Utimbuf)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn srandomdev()
    -> ();
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn futimes(_: i32, _: *const Timeval)
    -> i32;
    fn statfs(_: *const i8, _: *mut Statfs)
    -> i32;
    fn sqlite3_realloc64(_: *mut (), _: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3_uri_parameter(z: Sqlite3Filename, z_param_1: *const i8)
    -> *const i8;
    fn getpid()
    -> PidT;
    fn fstatfs(_: i32, _: *mut Statfs)
    -> i32;
    fn getenv(_: *const i8)
    -> *mut i8;
    fn atoi(_: *const i8)
    -> i32;
    fn dlopen(__path: *const i8, __mode: i32)
    -> *mut ();
    fn dlerror()
    -> *mut i8;
    fn dlclose(__handle: *mut ())
    -> i32;
    fn time(_: *mut TimeT)
    -> TimeT;
    fn gettimeofday(_: *mut Timeval, _: *mut ())
    -> i32;
    fn flock(_: i32, _: i32)
    -> i32;
    fn sqlite3_vfs_register(_: *mut Sqlite3Vfs, make_dflt_1: i32)
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
    fn sqlite3_vmprintf(_: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_vsnprintf(_: i32, _: *mut i8, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_malloc(_: i32)
    -> *mut ();
    fn sqlite3_realloc(_: *mut (), _: i32)
    -> *mut ();
    fn sqlite3_msize(_: *mut ())
    -> Sqlite3Uint64;
    fn sqlite3_memory_used()
    -> Sqlite3Int64;
    fn sqlite3_memory_highwater(reset_flag_1: i32)
    -> Sqlite3Int64;
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
    fn sqlite3_vfs_unregister(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_mutex_try(_: *mut Sqlite3Mutex)
    -> i32;
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
    fn sqlite3_is_id_char(_: u8)
    -> i32;
    fn sqlite3_str_i_cmp(_: *const i8, _: *const i8)
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
    fn sqlite3_mutex_init()
    -> i32;
    fn sqlite3_mutex_end()
    -> i32;
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
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: Sqlite3Str;
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
    fn __builtin_unreachable()
    -> ();
    static mut __stderrp: *mut FILE;
    fn close(_: i32)
    -> i32;
    fn access(_: *const i8, _: i32)
    -> i32;
    fn getcwd(_: *mut i8, __size: u64)
    -> *mut i8;
    fn stat(_: *const i8, _: *mut Stat)
    -> i32;
    fn fstat(_: i32, _: *mut Stat)
    -> i32;
    fn ftruncate(_: i32, _: OffT)
    -> i32;
    fn fcntl(_: i32, _: i32, ...)
    -> i32;
    fn read(_: i32, _: *mut (), __nbyte: u64)
    -> i64;
    fn pread(__fd: i32, __buf: *mut (), __nbyte: u64, __offset: OffT)
    -> i64;
    fn write(__fd: i32, __buf: *const (), __nbyte: u64)
    -> i64;
    fn pwrite(__fd: i32, __buf: *const (), __nbyte: u64, __offset: OffT)
    -> i64;
    fn fchmod(_: i32, _: ModeT)
    -> i32;
    fn unlink(_: *const i8)
    -> i32;
    fn mkdir(_: *const i8, _: ModeT)
    -> i32;
    fn rmdir(_: *const i8)
    -> i32;
    fn fchown(_: i32, _: UidT, _: GidT)
    -> i32;
    fn geteuid()
    -> UidT;
    fn mmap(_: *mut (), _: u64, _: i32, _: i32, _: i32, _: OffT)
    -> *mut ();
    fn munmap(_: *mut (), _: u64)
    -> i32;
    fn readlink(_: *const i8, _: *mut i8, __bufsize: u64)
    -> i64;
    fn lstat(_: *const i8, _: *mut Stat)
    -> i32;
    fn dlsym(__handle: *mut (), __symbol: *const i8)
    -> *mut ();
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
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;

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

#[repr(C)]
#[derive(Copy, Clone)]
struct Utimbuf {
    _opaque: [u8; 0],
}
