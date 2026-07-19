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
    AuthContext, Bft, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte,
    DbFixer, Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode,
    FuncDef, FuncDefHash, FuncDestructor, IdList, Index, KeyInfo, LogEst,
    Module, NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema,
    Select, SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str,
    SrcItem, SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With, YnVar,
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

/// The state of the parser is completely contained in an instance of
///* the following structure
#[repr(C)]
#[derive(Copy, Clone)]
struct YyParser {
    yytos: *mut YyStackEntry,
    p_parse: *mut Parse,
    yystack_end: *mut YyStackEntry,
    yystack: *mut YyStackEntry,
    yystk0: [YyStackEntry; 50],
}

/// The following structure represents a single element of the
///* parser's stack.  Information stored includes:
///*
///*   +  The state number for the parser at this level of the stack.
///*
///*   +  The value of the token stored at this level of the stack.
///*      (In other words, the "major" token.)
///*
///*   +  The semantic value stored at this level of the stack.  This is
///*      the information used by the action routines in the grammar.
///*      It is sometimes called the "minor" token.
///*
///* After the "shift" half of a SHIFTREDUCE action, the stateno field
///* actually contains the reduce action for the second half of the
///* SHIFTREDUCE.
#[repr(C)]
#[derive(Copy, Clone)]
struct YyStackEntry {
    stateno: u16,
    major: u16,
    minor: YYMINORTYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
union YYMINORTYPE {
    yyinit: i32,
    yy0: Token,
    yy14: *mut ExprList,
    yy59: *mut With,
    yy67: *mut Cte,
    yy122: *mut Upsert,
    yy132: *mut IdList,
    yy144: i32,
    yy168: *const i8,
    yy203: *mut SrcList,
    yy211: *mut Window,
    yy269: OnOrUsing,
    yy286: TrigEvent,
    yy383: YYMINORTYPES0,
    yy391: u32,
    yy427: *mut TriggerStep,
    yy454: *mut Expr,
    yy462: u8,
    yy509: FrameBound,
    yy555: *mut Select,
}

///* An instance of the following structure describes the event of a
///* TRIGGER.  "a" is the event type, one of TK_UPDATE, TK_INSERT,
///* TK_DELETE, or TK_INSTEAD.  If the event is of the form
///*
///*      UPDATE ON (a,b,c)
///*
///* Then the "b" IdList records the list "a,b,c".
#[repr(C)]
#[derive(Copy, Clone)]
struct TrigEvent {
    a: i32,
    b: *mut IdList,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct YYMINORTYPES0 {
    value: i32,
    mask: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FrameBound {
    e_type: i32,
    p_expr: *mut Expr,
}

/// Initialize a new parser that has already been allocated.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_parser_init(yyp_raw_parser_1: *mut (),
    p_parse_1: *mut Parse) -> () {
    let yyp_parser: *mut YyParser = yyp_raw_parser_1 as *mut YyParser;
    unsafe { (*yyp_parser).p_parse = p_parse_1 };
    unsafe {
        (*yyp_parser).yystack =
            unsafe { &raw mut (*yyp_parser).yystk0[0 as usize] } as
                *mut YyStackEntry
    };
    unsafe {
        (*yyp_parser).yystack_end =
            unsafe {
                unsafe { (*yyp_parser).yystack.offset((50 - 1) as isize) }
            }
    };
    unsafe { (*yyp_parser).yytos = unsafe { (*yyp_parser).yystack } };
    unsafe {
        (*unsafe { (*yyp_parser).yystack.offset(0 as isize) }).stateno =
            0 as u16
    };
    unsafe {
        (*unsafe { (*yyp_parser).yystack.offset(0 as isize) }).major =
            0 as u16
    };
}

/// 
///* This function allocates a new parser.
///* The only argument is a pointer to a function which works like
///* malloc.
///*
///* Inputs:
///* A pointer to the function used to allocate memory.
///*
///* Outputs:
///* A pointer to a parser.  This pointer is used in subsequent calls
///* to sqlite3Parser and sqlite3ParserFree.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_parser_alloc(malloc_proc:
        Option<unsafe extern "C" fn(u64) -> *mut ()>, p_parse: *mut Parse)
    -> *mut () {
    let mut yyp_parser: *mut YyParser = core::ptr::null_mut();
    yyp_parser =
        unsafe {
                malloc_proc.unwrap()(core::mem::size_of::<YyParser>() as u64)
            } as *mut YyParser;
    if !(yyp_parser).is_null() {
        unsafe { (*yyp_parser).p_parse = p_parse };
        sqlite3_parser_init(yyp_parser as *mut (), p_parse);
    }
    return yyp_parser as *mut ();
}

/// The following function deletes the "minor type" or semantic value
///* associated with a symbol.  The symbol can be either a terminal
///* or nonterminal. "yymajor" is the symbol code, and "yypminor" is
///* a pointer to the value to be deleted.  The code used to do the 
///* deletions is derived from the %destructor and/or %token_destructor
///* directives of the input grammar.
extern "C" fn yy_destructor(yyp_parser_1: &YyParser, yymajor: u16,
    yypminor: &YYMINORTYPE) -> () {
    unsafe {
        let p_parse: *const Parse = (*yyp_parser_1).p_parse as *const Parse;
        '__s0:
            {
            match yymajor {
                206 => {
                    {
                        unsafe {
                            sqlite3_select_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy555)
                        };
                    }
                }
                241 => {
                    {
                        unsafe {
                            sqlite3_select_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy555)
                        };
                    }
                }
                242 => {
                    {
                        unsafe {
                            sqlite3_select_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy555)
                        };
                    }
                }
                254 => {
                    {
                        unsafe {
                            sqlite3_select_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy555)
                        };
                    }
                }
                256 => {
                    {
                        unsafe {
                            sqlite3_select_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy555)
                        };
                    }
                }
                218 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                219 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                248 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                250 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                270 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                281 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                283 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                286 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                293 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                297 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                314 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy454)
                        };
                    }
                }
                223 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                233 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                234 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                246 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                249 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                251 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                255 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                257 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                264 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                271 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                280 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                282 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                313 => {
                    {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy14)
                        };
                    }
                }
                240 => {
                    {
                        unsafe {
                            sqlite3_src_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy203)
                        };
                    }
                }
                247 => {
                    {
                        unsafe {
                            sqlite3_src_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy203)
                        };
                    }
                }
                259 => {
                    {
                        unsafe {
                            sqlite3_src_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy203)
                        };
                    }
                }
                260 => {
                    {
                        unsafe {
                            sqlite3_src_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy203)
                        };
                    }
                }
                265 => {
                    {
                        unsafe {
                            sqlite3_src_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy203)
                        };
                    }
                }
                243 => {
                    {
                        unsafe {
                            sqlite3_with_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy59)
                        };
                    }
                }
                253 => {
                    {
                        unsafe {
                            sqlite3_window_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                309 => {
                    {
                        unsafe {
                            sqlite3_window_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                266 => {
                    {
                        unsafe {
                            sqlite3_id_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy132)
                        };
                    }
                }
                273 => {
                    {
                        unsafe {
                            sqlite3_id_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy132)
                        };
                    }
                }
                276 => {
                    {
                        unsafe {
                            sqlite3_window_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                310 => {
                    {
                        unsafe {
                            sqlite3_window_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                311 => {
                    {
                        unsafe {
                            sqlite3_window_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                312 => {
                    {
                        unsafe {
                            sqlite3_window_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                315 => {
                    {
                        unsafe {
                            sqlite3_window_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy211)
                        };
                    }
                }
                289 => {
                    {
                        unsafe {
                            sqlite3_delete_trigger_step(unsafe { (*p_parse).db },
                                (*yypminor).yy427)
                        };
                    }
                }
                294 => {
                    {
                        unsafe {
                            sqlite3_delete_trigger_step(unsafe { (*p_parse).db },
                                (*yypminor).yy427)
                        };
                    }
                }
                291 => {
                    {
                        unsafe {
                            sqlite3_id_list_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy286.b)
                        };
                    }
                }
                317 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy509.p_expr)
                        };
                    }
                }
                318 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy509.p_expr)
                        };
                    }
                }
                319 => {
                    {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                (*yypminor).yy509.p_expr)
                        };
                    }
                }
                _ => {}
            }
        }
    }
}

extern "C" fn parser_stack_free(p_old_1: *mut (), p_parse_1: *const Parse)
    -> () {
    { let _ = p_parse_1; };
    unsafe { sqlite3_free(p_old_1) };
}

///* Clear all secondary memory allocations from the parser
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_parser_finalize(p: *mut ()) -> () {
    let p_parser: *mut YyParser = p as *mut YyParser;
    /// In-lined version of calling yy_pop_parser_stack() for each
    ///* element left in the stack
    let mut yytos: *mut YyStackEntry = unsafe { (*p_parser).yytos };
    while yytos > unsafe { (*p_parser).yystack } {
        if unsafe { (*yytos).major } as i32 >= 206 {
            yy_destructor(unsafe { &*p_parser }, unsafe { (*yytos).major },
                unsafe { &(*yytos).minor });
        }
        {
            let __p = &mut yytos;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(-1) };
            __t
        };
    }
    if unsafe { (*p_parser).yystack } !=
            unsafe { &raw mut (*p_parser).yystk0[0 as usize] } as
                *mut YyStackEntry {
        parser_stack_free(unsafe { (*p_parser).yystack } as *mut (),
            unsafe { (*p_parser).p_parse } as *const Parse);
    }
}

/// 
///* Deallocate and destroy a parser.  Destructors are called for
///* all stack elements before shutting the parser down.
///*
///* If the YYPARSEFREENEVERNULL macro exists (for example because it
///* is defined in a %include section of the input grammar) then it is
///* assumed that the input pointer is never NULL.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_parser_free(p: *mut (),
    free_proc: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    sqlite3_parser_finalize(p);
    unsafe { free_proc.unwrap()(p) };
}

static yy_shift_ofst: [u16; 600] =
    [2201 as u16, 1973 as u16, 2215 as u16, 1552 as u16, 1552 as u16,
            33 as u16, 368 as u16, 1668 as u16, 1741 as u16, 1814 as u16,
            726 as u16, 726 as u16, 726 as u16, 265 as u16, 33 as u16,
            33 as u16, 33 as u16, 33 as u16, 33 as u16, 0 as u16, 0 as u16,
            216 as u16, 1349 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 272 as u16, 272 as u16, 111 as u16,
            111 as u16, 316 as u16, 365 as u16, 516 as u16, 867 as u16,
            867 as u16, 916 as u16, 916 as u16, 916 as u16, 916 as u16,
            40 as u16, 112 as u16, 260 as u16, 364 as u16, 408 as u16,
            512 as u16, 617 as u16, 661 as u16, 765 as u16, 809 as u16,
            913 as u16, 957 as u16, 1061 as u16, 1081 as u16, 1195 as u16,
            1215 as u16, 1329 as u16, 1349 as u16, 1349 as u16, 1349 as u16,
            1349 as u16, 1349 as u16, 1349 as u16, 1349 as u16, 1349 as u16,
            1349 as u16, 1349 as u16, 1349 as u16, 1349 as u16, 1349 as u16,
            1349 as u16, 1349 as u16, 1349 as u16, 1349 as u16, 1349 as u16,
            1369 as u16, 1349 as u16, 1473 as u16, 1493 as u16, 1493 as u16,
            473 as u16, 1974 as u16, 2082 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            726 as u16, 726 as u16, 726 as u16, 726 as u16, 726 as u16,
            138 as u16, 232 as u16, 232 as u16, 232 as u16, 232 as u16,
            232 as u16, 232 as u16, 232 as u16, 188 as u16, 99 as u16,
            242 as u16, 718 as u16, 416 as u16, 1159 as u16, 867 as u16,
            867 as u16, 940 as u16, 940 as u16, 867 as u16, 1103 as u16,
            417 as u16, 574 as u16, 574 as u16, 574 as u16, 611 as u16,
            139 as u16, 139 as u16, 2379 as u16, 2379 as u16, 1026 as u16,
            1026 as u16, 1026 as u16, 536 as u16, 466 as u16, 466 as u16,
            466 as u16, 466 as u16, 1017 as u16, 1017 as u16, 849 as u16,
            718 as u16, 971 as u16, 1060 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 261 as u16, 712 as u16, 712 as u16,
            867 as u16, 108 as u16, 1142 as u16, 1142 as u16, 977 as u16,
            1108 as u16, 1108 as u16, 977 as u16, 977 as u16, 1243 as u16,
            2379 as u16, 2379 as u16, 2379 as u16, 2379 as u16, 2379 as u16,
            2379 as u16, 2379 as u16, 641 as u16, 789 as u16, 789 as u16,
            635 as u16, 366 as u16, 721 as u16, 673 as u16, 782 as u16,
            494 as u16, 787 as u16, 829 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 959 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 820 as u16,
            820 as u16, 820 as u16, 867 as u16, 867 as u16, 867 as u16,
            1136 as u16, 867 as u16, 867 as u16, 867 as u16, 1119 as u16,
            1007 as u16, 867 as u16, 1169 as u16, 867 as u16, 867 as u16,
            867 as u16, 867 as u16, 867 as u16, 867 as u16, 867 as u16,
            867 as u16, 1225 as u16, 1153 as u16, 869 as u16, 196 as u16,
            618 as u16, 618 as u16, 618 as u16, 618 as u16, 1491 as u16,
            196 as u16, 196 as u16, 91 as u16, 339 as u16, 1326 as u16,
            1386 as u16, 383 as u16, 1163 as u16, 1364 as u16, 1426 as u16,
            1364 as u16, 1538 as u16, 903 as u16, 1163 as u16, 1163 as u16,
            903 as u16, 1163 as u16, 1426 as u16, 1538 as u16, 1018 as u16,
            1535 as u16, 1241 as u16, 1528 as u16, 1528 as u16, 1528 as u16,
            1394 as u16, 1394 as u16, 1394 as u16, 1394 as u16, 762 as u16,
            762 as u16, 1403 as u16, 1466 as u16, 1475 as u16, 1551 as u16,
            1746 as u16, 1805 as u16, 1746 as u16, 1746 as u16, 1729 as u16,
            1729 as u16, 1840 as u16, 1840 as u16, 1729 as u16, 1730 as u16,
            1732 as u16, 1859 as u16, 1842 as u16, 1870 as u16, 1870 as u16,
            1870 as u16, 1870 as u16, 1729 as u16, 1876 as u16, 1751 as u16,
            1732 as u16, 1732 as u16, 1751 as u16, 1859 as u16, 1842 as u16,
            1751 as u16, 1842 as u16, 1751 as u16, 1729 as u16, 1876 as u16,
            1760 as u16, 1857 as u16, 1729 as u16, 1876 as u16, 1906 as u16,
            1729 as u16, 1876 as u16, 1729 as u16, 1876 as u16, 1906 as u16,
            1746 as u16, 1746 as u16, 1746 as u16, 1873 as u16, 1922 as u16,
            1922 as u16, 1906 as u16, 1746 as u16, 1822 as u16, 1746 as u16,
            1873 as u16, 1746 as u16, 1746 as u16, 1786 as u16, 1929 as u16,
            1843 as u16, 1843 as u16, 1906 as u16, 1729 as u16, 1872 as u16,
            1872 as u16, 1894 as u16, 1894 as u16, 1831 as u16, 1836 as u16,
            1966 as u16, 1729 as u16, 1833 as u16, 1831 as u16, 1851 as u16,
            1860 as u16, 1751 as u16, 1983 as u16, 1996 as u16, 1996 as u16,
            2009 as u16, 2009 as u16, 2009 as u16, 2379 as u16, 2379 as u16,
            2379 as u16, 2379 as u16, 2379 as u16, 2379 as u16, 2379 as u16,
            2379 as u16, 2379 as u16, 2379 as u16, 2379 as u16, 2379 as u16,
            2379 as u16, 2379 as u16, 2379 as u16, 136 as u16, 1063 as u16,
            1196 as u16, 530 as u16, 636 as u16, 1274 as u16, 1300 as u16,
            1443 as u16, 1598 as u16, 1495 as u16, 1479 as u16, 967 as u16,
            1083 as u16, 1602 as u16, 463 as u16, 1625 as u16, 1638 as u16,
            1670 as u16, 1541 as u16, 1671 as u16, 1689 as u16, 1696 as u16,
            1277 as u16, 1432 as u16, 1693 as u16, 808 as u16, 1700 as u16,
            1607 as u16, 1657 as u16, 1587 as u16, 1704 as u16, 1707 as u16,
            1631 as u16, 1708 as u16, 1733 as u16, 1608 as u16, 1611 as u16,
            1743 as u16, 1747 as u16, 1620 as u16, 1592 as u16, 2026 as u16,
            2030 as u16, 2013 as u16, 1914 as u16, 2018 as u16, 1916 as u16,
            2020 as u16, 2019 as u16, 2021 as u16, 1915 as u16, 2027 as u16,
            2029 as u16, 2024 as u16, 2025 as u16, 1909 as u16, 1899 as u16,
            1923 as u16, 2028 as u16, 2028 as u16, 1913 as u16, 2037 as u16,
            1917 as u16, 2042 as u16, 2059 as u16, 1918 as u16, 1932 as u16,
            2028 as u16, 1933 as u16, 2003 as u16, 2031 as u16, 2028 as u16,
            1919 as u16, 2012 as u16, 2015 as u16, 2016 as u16, 2022 as u16,
            1936 as u16, 1956 as u16, 2040 as u16, 2053 as u16, 2077 as u16,
            2074 as u16, 2058 as u16, 1967 as u16, 1924 as u16, 2034 as u16,
            2060 as u16, 2036 as u16, 2008 as u16, 2046 as u16, 1946 as u16,
            1978 as u16, 2066 as u16, 2075 as u16, 2078 as u16, 1968 as u16,
            1972 as u16, 2084 as u16, 2041 as u16, 2086 as u16, 2088 as u16,
            2076 as u16, 2089 as u16, 2048 as u16, 2054 as u16, 2093 as u16,
            2023 as u16, 2091 as u16, 2099 as u16, 2055 as u16, 2085 as u16,
            2101 as u16, 2092 as u16, 1975 as u16, 2105 as u16, 2110 as u16,
            2111 as u16, 2112 as u16, 2115 as u16, 2113 as u16, 2043 as u16,
            1997 as u16, 2117 as u16, 2120 as u16, 2032 as u16, 2114 as u16,
            2122 as u16, 2001 as u16, 2121 as u16, 2116 as u16, 2118 as u16,
            2119 as u16, 2124 as u16, 2062 as u16, 2071 as u16, 2068 as u16,
            2123 as u16, 2080 as u16, 2065 as u16, 2126 as u16, 2138 as u16,
            2140 as u16, 2139 as u16, 2141 as u16, 2143 as u16, 2130 as u16,
            2033 as u16, 2035 as u16, 2142 as u16, 2121 as u16, 2146 as u16,
            2147 as u16, 2148 as u16, 2150 as u16, 2149 as u16, 2152 as u16,
            2156 as u16, 2151 as u16, 2164 as u16, 2158 as u16, 2159 as u16,
            2161 as u16, 2162 as u16, 2160 as u16, 2165 as u16, 2163 as u16,
            2050 as u16, 2049 as u16, 2051 as u16, 2052 as u16, 2167 as u16,
            2172 as u16, 2181 as u16, 2197 as u16, 2204 as u16];

static yy_lookahead: [u16; 2566] =
    [277 as u16, 278 as u16, 279 as u16, 241 as u16, 242 as u16, 225 as u16,
            195 as u16, 227 as u16, 195 as u16, 312 as u16, 195 as u16,
            218 as u16, 195 as u16, 316 as u16, 195 as u16, 235 as u16,
            254 as u16, 195 as u16, 256 as u16, 19 as u16, 297 as u16,
            277 as u16, 278 as u16, 279 as u16, 218 as u16, 206 as u16,
            213 as u16, 214 as u16, 206 as u16, 218 as u16, 219 as u16,
            31 as u16, 206 as u16, 218 as u16, 219 as u16, 218 as u16,
            219 as u16, 218 as u16, 219 as u16, 39 as u16, 218 as u16,
            219 as u16, 195 as u16, 43 as u16, 44 as u16, 45 as u16,
            195 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16,
            52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16,
            58 as u16, 19 as u16, 241 as u16, 242 as u16, 195 as u16,
            241 as u16, 242 as u16, 195 as u16, 255 as u16, 241 as u16,
            242 as u16, 195 as u16, 255 as u16, 237 as u16, 238 as u16,
            254 as u16, 255 as u16, 256 as u16, 254 as u16, 255 as u16,
            256 as u16, 264 as u16, 254 as u16, 207 as u16, 256 as u16,
            43 as u16, 44 as u16, 45 as u16, 264 as u16, 47 as u16, 48 as u16,
            49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 251 as u16,
            287 as u16, 253 as u16, 215 as u16, 103 as u16, 104 as u16,
            105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16,
            110 as u16, 111 as u16, 112 as u16, 113 as u16, 114 as u16,
            82 as u16, 265 as u16, 195 as u16, 271 as u16, 11 as u16,
            187 as u16, 188 as u16, 189 as u16, 190 as u16, 191 as u16,
            192 as u16, 190 as u16, 87 as u16, 192 as u16, 89 as u16,
            197 as u16, 19 as u16, 199 as u16, 197 as u16, 317 as u16,
            199 as u16, 319 as u16, 25 as u16, 271 as u16, 206 as u16,
            218 as u16, 219 as u16, 206 as u16, 103 as u16, 104 as u16,
            105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16,
            110 as u16, 111 as u16, 112 as u16, 113 as u16, 114 as u16,
            43 as u16, 44 as u16, 45 as u16, 195 as u16, 47 as u16, 48 as u16,
            49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 60 as u16, 139 as u16,
            140 as u16, 241 as u16, 242 as u16, 289 as u16, 241 as u16,
            242 as u16, 309 as u16, 310 as u16, 294 as u16, 70 as u16,
            47 as u16, 48 as u16, 49 as u16, 50 as u16, 254 as u16, 77 as u16,
            256 as u16, 254 as u16, 195 as u16, 256 as u16, 55 as u16,
            56 as u16, 57 as u16, 58 as u16, 59 as u16, 221 as u16, 88 as u16,
            109 as u16, 90 as u16, 269 as u16, 240 as u16, 93 as u16,
            269 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
            111 as u16, 112 as u16, 113 as u16, 114 as u16, 215 as u16,
            103 as u16, 104 as u16, 105 as u16, 106 as u16, 107 as u16,
            108 as u16, 109 as u16, 110 as u16, 111 as u16, 112 as u16,
            113 as u16, 114 as u16, 136 as u16, 117 as u16, 118 as u16,
            119 as u16, 298 as u16, 141 as u16, 300 as u16, 298 as u16,
            19 as u16, 300 as u16, 129 as u16, 130 as u16, 317 as u16,
            318 as u16, 103 as u16, 104 as u16, 105 as u16, 106 as u16,
            107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 114 as u16, 277 as u16,
            278 as u16, 279 as u16, 146 as u16, 122 as u16, 43 as u16,
            44 as u16, 45 as u16, 195 as u16, 47 as u16, 48 as u16, 49 as u16,
            50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16,
            56 as u16, 57 as u16, 58 as u16, 218 as u16, 277 as u16,
            278 as u16, 279 as u16, 19 as u16, 19 as u16, 195 as u16,
            286 as u16, 23 as u16, 68 as u16, 218 as u16, 219 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 43 as u16, 44 as u16, 45 as u16, 232 as u16,
            47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
            53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 58 as u16,
            103 as u16, 104 as u16, 105 as u16, 106 as u16, 107 as u16,
            108 as u16, 109 as u16, 110 as u16, 111 as u16, 112 as u16,
            113 as u16, 114 as u16, 135 as u16, 60 as u16, 137 as u16,
            138 as u16, 103 as u16, 104 as u16, 105 as u16, 106 as u16,
            107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 82 as u16, 281 as u16,
            206 as u16, 195 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 195 as u16, 195 as u16,
            195 as u16, 205 as u16, 22 as u16, 207 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 195 as u16, 60 as u16, 116 as u16, 117 as u16,
            107 as u16, 108 as u16, 218 as u16, 219 as u16, 19 as u16,
            241 as u16, 242 as u16, 121 as u16, 23 as u16, 116 as u16,
            117 as u16, 118 as u16, 119 as u16, 306 as u16, 121 as u16,
            308 as u16, 206 as u16, 234 as u16, 254 as u16, 15 as u16,
            256 as u16, 195 as u16, 129 as u16, 259 as u16, 260 as u16,
            139 as u16, 140 as u16, 145 as u16, 43 as u16, 44 as u16,
            45 as u16, 200 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16,
            51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16,
            57 as u16, 58 as u16, 218 as u16, 219 as u16, 60 as u16,
            154 as u16, 19 as u16, 156 as u16, 265 as u16, 241 as u16,
            242 as u16, 24 as u16, 117 as u16, 118 as u16, 119 as u16,
            120 as u16, 21 as u16, 73 as u16, 123 as u16, 124 as u16,
            125 as u16, 74 as u16, 254 as u16, 61 as u16, 256 as u16,
            107 as u16, 108 as u16, 221 as u16, 133 as u16, 82 as u16,
            43 as u16, 44 as u16, 45 as u16, 195 as u16, 47 as u16, 48 as u16,
            49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 195 as u16, 317 as u16, 318 as u16, 117 as u16,
            118 as u16, 119 as u16, 22 as u16, 120 as u16, 195 as u16,
            22 as u16, 123 as u16, 124 as u16, 125 as u16, 19 as u16,
            20 as u16, 284 as u16, 22 as u16, 128 as u16, 81 as u16,
            288 as u16, 133 as u16, 195 as u16, 195 as u16, 218 as u16,
            219 as u16, 277 as u16, 278 as u16, 279 as u16, 139 as u16,
            140 as u16, 36 as u16, 195 as u16, 103 as u16, 104 as u16,
            105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16,
            110 as u16, 111 as u16, 112 as u16, 113 as u16, 114 as u16,
            218 as u16, 219 as u16, 62 as u16, 60 as u16, 195 as u16,
            241 as u16, 242 as u16, 271 as u16, 19 as u16, 240 as u16,
            60 as u16, 189 as u16, 190 as u16, 191 as u16, 192 as u16,
            233 as u16, 255 as u16, 124 as u16, 254 as u16, 197 as u16,
            256 as u16, 199 as u16, 72 as u16, 129 as u16, 130 as u16,
            264 as u16, 195 as u16, 195 as u16, 206 as u16, 22 as u16,
            23 as u16, 60 as u16, 43 as u16, 44 as u16, 45 as u16, 206 as u16,
            47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
            53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 58 as u16,
            195 as u16, 218 as u16, 219 as u16, 101 as u16, 195 as u16,
            60 as u16, 271 as u16, 162 as u16, 195 as u16, 107 as u16,
            108 as u16, 109 as u16, 117 as u16, 118 as u16, 119 as u16,
            241 as u16, 242 as u16, 115 as u16, 73 as u16, 117 as u16,
            118 as u16, 119 as u16, 241 as u16, 242 as u16, 122 as u16,
            60 as u16, 195 as u16, 266 as u16, 254 as u16, 312 as u16,
            256 as u16, 218 as u16, 219 as u16, 316 as u16, 203 as u16,
            254 as u16, 195 as u16, 256 as u16, 255 as u16, 208 as u16,
            117 as u16, 118 as u16, 119 as u16, 269 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 154 as u16, 155 as u16, 156 as u16, 157 as u16,
            158 as u16, 102 as u16, 117 as u16, 118 as u16, 119 as u16,
            19 as u16, 242 as u16, 144 as u16, 255 as u16, 23 as u16,
            206 as u16, 24 as u16, 298 as u16, 195 as u16, 300 as u16,
            206 as u16, 195 as u16, 264 as u16, 254 as u16, 206 as u16,
            256 as u16, 240 as u16, 117 as u16, 118 as u16, 119 as u16,
            183 as u16, 22 as u16, 22 as u16, 23 as u16, 43 as u16, 44 as u16,
            45 as u16, 151 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16,
            51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16,
            57 as u16, 58 as u16, 241 as u16, 242 as u16, 60 as u16,
            195 as u16, 19 as u16, 241 as u16, 242 as u16, 195 as u16,
            23 as u16, 241 as u16, 242 as u16, 195 as u16, 152 as u16,
            254 as u16, 310 as u16, 256 as u16, 243 as u16, 312 as u16,
            254 as u16, 60 as u16, 256 as u16, 316 as u16, 254 as u16,
            206 as u16, 256 as u16, 60 as u16, 218 as u16, 219 as u16,
            43 as u16, 44 as u16, 45 as u16, 272 as u16, 47 as u16, 48 as u16,
            49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 240 as u16, 60 as u16, 241 as u16, 242 as u16,
            118 as u16, 25 as u16, 102 as u16, 255 as u16, 166 as u16,
            167 as u16, 101 as u16, 22 as u16, 26 as u16, 19 as u16,
            20 as u16, 254 as u16, 22 as u16, 256 as u16, 139 as u16,
            140 as u16, 117 as u16, 118 as u16, 119 as u16, 306 as u16,
            195 as u16, 308 as u16, 117 as u16, 118 as u16, 237 as u16,
            238 as u16, 36 as u16, 122 as u16, 103 as u16, 104 as u16,
            105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16,
            110 as u16, 111 as u16, 112 as u16, 113 as u16, 114 as u16,
            195 as u16, 195 as u16, 60 as u16, 218 as u16, 219 as u16,
            60 as u16, 109 as u16, 195 as u16, 19 as u16, 217 as u16,
            60 as u16, 25 as u16, 23 as u16, 77 as u16, 117 as u16,
            118 as u16, 119 as u16, 225 as u16, 233 as u16, 154 as u16,
            155 as u16, 156 as u16, 72 as u16, 312 as u16, 218 as u16,
            219 as u16, 90 as u16, 316 as u16, 22 as u16, 93 as u16,
            303 as u16, 304 as u16, 43 as u16, 44 as u16, 45 as u16,
            195 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16,
            52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16,
            58 as u16, 183 as u16, 195 as u16, 195 as u16, 101 as u16,
            19 as u16, 213 as u16, 214 as u16, 243 as u16, 23 as u16,
            107 as u16, 108 as u16, 117 as u16, 118 as u16, 119 as u16,
            117 as u16, 118 as u16, 119 as u16, 115 as u16, 60 as u16,
            117 as u16, 118 as u16, 119 as u16, 195 as u16, 60 as u16,
            122 as u16, 218 as u16, 219 as u16, 22 as u16, 43 as u16,
            44 as u16, 45 as u16, 35 as u16, 47 as u16, 48 as u16, 49 as u16,
            50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16,
            56 as u16, 57 as u16, 58 as u16, 103 as u16, 104 as u16,
            105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16,
            110 as u16, 111 as u16, 112 as u16, 113 as u16, 114 as u16,
            154 as u16, 155 as u16, 156 as u16, 157 as u16, 158 as u16,
            195 as u16, 255 as u16, 67 as u16, 195 as u16, 60 as u16,
            101 as u16, 240 as u16, 311 as u16, 312 as u16, 306 as u16,
            75 as u16, 308 as u16, 316 as u16, 29 as u16, 117 as u16,
            118 as u16, 119 as u16, 33 as u16, 287 as u16, 117 as u16,
            118 as u16, 119 as u16, 118 as u16, 146 as u16, 183 as u16,
            195 as u16, 122 as u16, 103 as u16, 104 as u16, 105 as u16,
            106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
            111 as u16, 112 as u16, 113 as u16, 114 as u16, 215 as u16,
            195 as u16, 77 as u16, 60 as u16, 25 as u16, 195 as u16,
            122 as u16, 144 as u16, 19 as u16, 218 as u16, 219 as u16,
            66 as u16, 23 as u16, 88 as u16, 246 as u16, 90 as u16,
            132 as u16, 25 as u16, 93 as u16, 154 as u16, 155 as u16,
            156 as u16, 117 as u16, 118 as u16, 119 as u16, 257 as u16,
            195 as u16, 131 as u16, 218 as u16, 219 as u16, 195 as u16,
            265 as u16, 43 as u16, 44 as u16, 45 as u16, 195 as u16,
            47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
            53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 58 as u16,
            183 as u16, 218 as u16, 219 as u16, 195 as u16, 19 as u16,
            218 as u16, 219 as u16, 195 as u16, 23 as u16, 195 as u16,
            218 as u16, 219 as u16, 117 as u16, 118 as u16, 119 as u16,
            195 as u16, 233 as u16, 255 as u16, 195 as u16, 195 as u16,
            233 as u16, 22 as u16, 23 as u16, 146 as u16, 25 as u16,
            233 as u16, 218 as u16, 219 as u16, 43 as u16, 44 as u16,
            45 as u16, 294 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16,
            51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16,
            57 as u16, 58 as u16, 103 as u16, 104 as u16, 105 as u16,
            106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
            111 as u16, 112 as u16, 113 as u16, 114 as u16, 195 as u16,
            12 as u16, 234 as u16, 195 as u16, 240 as u16, 74 as u16,
            195 as u16, 255 as u16, 195 as u16, 60 as u16, 243 as u16,
            262 as u16, 263 as u16, 311 as u16, 312 as u16, 25 as u16,
            27 as u16, 19 as u16, 316 as u16, 107 as u16, 108 as u16,
            265 as u16, 24 as u16, 265 as u16, 195 as u16, 150 as u16,
            195 as u16, 139 as u16, 140 as u16, 218 as u16, 219 as u16,
            42 as u16, 103 as u16, 104 as u16, 105 as u16, 106 as u16,
            107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 233 as u16, 102 as u16,
            67 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            243 as u16, 19 as u16, 64 as u16, 22 as u16, 23 as u16, 23 as u16,
            25 as u16, 195 as u16, 128 as u16, 129 as u16, 130 as u16,
            233 as u16, 74 as u16, 233 as u16, 86 as u16, 154 as u16,
            118 as u16, 156 as u16, 130 as u16, 265 as u16, 208 as u16,
            19 as u16, 306 as u16, 95 as u16, 308 as u16, 43 as u16,
            44 as u16, 45 as u16, 266 as u16, 47 as u16, 48 as u16, 49 as u16,
            50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16,
            56 as u16, 57 as u16, 58 as u16, 153 as u16, 230 as u16,
            96 as u16, 232 as u16, 43 as u16, 44 as u16, 45 as u16, 19 as u16,
            47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
            53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 58 as u16,
            114 as u16, 22 as u16, 306 as u16, 24 as u16, 308 as u16,
            127 as u16, 120 as u16, 121 as u16, 122 as u16, 123 as u16,
            124 as u16, 125 as u16, 126 as u16, 195 as u16, 147 as u16,
            212 as u16, 213 as u16, 214 as u16, 132 as u16, 23 as u16,
            195 as u16, 25 as u16, 102 as u16, 100 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 218 as u16, 219 as u16, 19 as u16, 60 as u16,
            195 as u16, 12 as u16, 210 as u16, 211 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 27 as u16, 134 as u16, 195 as u16, 195 as u16,
            195 as u16, 210 as u16, 211 as u16, 218 as u16, 219 as u16,
            195 as u16, 47 as u16, 195 as u16, 212 as u16, 213 as u16,
            214 as u16, 42 as u16, 16 as u16, 130 as u16, 19 as u16,
            112 as u16, 113 as u16, 114 as u16, 23 as u16, 77 as u16,
            195 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            117 as u16, 163 as u16, 164 as u16, 218 as u16, 219 as u16,
            218 as u16, 219 as u16, 90 as u16, 64 as u16, 19 as u16,
            93 as u16, 153 as u16, 118 as u16, 43 as u16, 44 as u16,
            45 as u16, 160 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16,
            51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16,
            57 as u16, 58 as u16, 195 as u16, 119 as u16, 272 as u16,
            276 as u16, 43 as u16, 44 as u16, 45 as u16, 195 as u16,
            47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
            53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 58 as u16,
            78 as u16, 116 as u16, 80 as u16, 218 as u16, 219 as u16,
            116 as u16, 144 as u16, 128 as u16, 129 as u16, 130 as u16,
            218 as u16, 219 as u16, 61 as u16, 195 as u16, 47 as u16,
            195 as u16, 16 as u16, 132 as u16, 195 as u16, 263 as u16,
            195 as u16, 314 as u16, 315 as u16, 267 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            151 as u16, 218 as u16, 219 as u16, 195 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 210 as u16, 211 as u16, 195 as u16, 7 as u16,
            8 as u16, 9 as u16, 195 as u16, 60 as u16, 195 as u16, 312 as u16,
            218 as u16, 219 as u16, 195 as u16, 316 as u16, 195 as u16,
            120 as u16, 195 as u16, 263 as u16, 19 as u16, 195 as u16,
            125 as u16, 267 as u16, 78 as u16, 24 as u16, 80 as u16,
            218 as u16, 219 as u16, 116 as u16, 162 as u16, 218 as u16,
            219 as u16, 218 as u16, 219 as u16, 301 as u16, 302 as u16,
            218 as u16, 219 as u16, 195 as u16, 19 as u16, 218 as u16,
            219 as u16, 276 as u16, 43 as u16, 44 as u16, 45 as u16,
            160 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16,
            52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16,
            58 as u16, 19 as u16, 146 as u16, 218 as u16, 219 as u16,
            43 as u16, 44 as u16, 45 as u16, 118 as u16, 47 as u16, 48 as u16,
            49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 165 as u16,
            314 as u16, 315 as u16, 276 as u16, 43 as u16, 44 as u16,
            45 as u16, 266 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16,
            51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16,
            57 as u16, 58 as u16, 128 as u16, 129 as u16, 130 as u16,
            195 as u16, 103 as u16, 104 as u16, 105 as u16, 106 as u16,
            107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 195 as u16, 228 as u16,
            195 as u16, 61 as u16, 195 as u16, 314 as u16, 315 as u16,
            25 as u16, 103 as u16, 104 as u16, 105 as u16, 106 as u16,
            107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 195 as u16, 22 as u16,
            195 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            195 as u16, 103 as u16, 104 as u16, 105 as u16, 106 as u16,
            107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
            112 as u16, 113 as u16, 114 as u16, 195 as u16, 195 as u16,
            246 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            25 as u16, 19 as u16, 246 as u16, 218 as u16, 219 as u16,
            246 as u16, 257 as u16, 259 as u16, 260 as u16, 195 as u16,
            22 as u16, 266 as u16, 60 as u16, 257 as u16, 195 as u16,
            120 as u16, 257 as u16, 218 as u16, 219 as u16, 116 as u16,
            195 as u16, 19 as u16, 195 as u16, 150 as u16, 151 as u16,
            25 as u16, 44 as u16, 45 as u16, 266 as u16, 47 as u16, 48 as u16,
            49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
            55 as u16, 56 as u16, 57 as u16, 58 as u16, 195 as u16, 54 as u16,
            218 as u16, 219 as u16, 218 as u16, 219 as u16, 45 as u16,
            145 as u16, 47 as u16, 48 as u16, 49 as u16, 50 as u16, 51 as u16,
            52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16,
            58 as u16, 246 as u16, 121 as u16, 122 as u16, 218 as u16,
            219 as u16, 19 as u16, 23 as u16, 31 as u16, 25 as u16,
            118 as u16, 159 as u16, 257 as u16, 161 as u16, 24 as u16,
            195 as u16, 39 as u16, 195 as u16, 143 as u16, 195 as u16,
            19 as u16, 20 as u16, 22 as u16, 22 as u16, 24 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 36 as u16, 218 as u16, 219 as u16, 218 as u16,
            219 as u16, 218 as u16, 219 as u16, 195 as u16, 103 as u16,
            104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
            109 as u16, 110 as u16, 111 as u16, 112 as u16, 113 as u16,
            114 as u16, 195 as u16, 143 as u16, 119 as u16, 136 as u16,
            60 as u16, 195 as u16, 22 as u16, 195 as u16, 141 as u16,
            195 as u16, 218 as u16, 219 as u16, 195 as u16, 23 as u16,
            195 as u16, 25 as u16, 72 as u16, 23 as u16, 131 as u16,
            25 as u16, 195 as u16, 134 as u16, 23 as u16, 218 as u16,
            219 as u16, 195 as u16, 82 as u16, 144 as u16, 218 as u16,
            219 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            195 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            60 as u16, 23 as u16, 195 as u16, 25 as u16, 218 as u16,
            219 as u16, 101 as u16, 195 as u16, 117 as u16, 218 as u16,
            219 as u16, 195 as u16, 107 as u16, 108 as u16, 23 as u16,
            195 as u16, 25 as u16, 195 as u16, 218 as u16, 219 as u16,
            115 as u16, 228 as u16, 117 as u16, 118 as u16, 119 as u16,
            218 as u16, 219 as u16, 122 as u16, 195 as u16, 19 as u16,
            218 as u16, 219 as u16, 195 as u16, 60 as u16, 218 as u16,
            219 as u16, 142 as u16, 195 as u16, 218 as u16, 219 as u16,
            19 as u16, 20 as u16, 195 as u16, 22 as u16, 139 as u16,
            140 as u16, 23 as u16, 23 as u16, 25 as u16, 25 as u16,
            195 as u16, 218 as u16, 219 as u16, 7 as u16, 8 as u16,
            218 as u16, 219 as u16, 36 as u16, 118 as u16, 154 as u16,
            155 as u16, 156 as u16, 157 as u16, 158 as u16, 195 as u16,
            23 as u16, 195 as u16, 25 as u16, 84 as u16, 85 as u16, 49 as u16,
            195 as u16, 23 as u16, 195 as u16, 25 as u16, 195 as u16,
            23 as u16, 195 as u16, 25 as u16, 195 as u16, 23 as u16,
            60 as u16, 25 as u16, 23 as u16, 23 as u16, 25 as u16, 25 as u16,
            142 as u16, 183 as u16, 218 as u16, 219 as u16, 118 as u16,
            195 as u16, 72 as u16, 218 as u16, 219 as u16, 218 as u16,
            219 as u16, 218 as u16, 219 as u16, 218 as u16, 219 as u16,
            218 as u16, 219 as u16, 195 as u16, 195 as u16, 146 as u16,
            86 as u16, 98 as u16, 23 as u16, 195 as u16, 25 as u16, 91 as u16,
            19 as u16, 20 as u16, 154 as u16, 22 as u16, 156 as u16,
            154 as u16, 23 as u16, 156 as u16, 25 as u16, 101 as u16,
            23 as u16, 195 as u16, 25 as u16, 195 as u16, 195 as u16,
            107 as u16, 108 as u16, 36 as u16, 195 as u16, 195 as u16,
            195 as u16, 195 as u16, 228 as u16, 115 as u16, 195 as u16,
            117 as u16, 118 as u16, 119 as u16, 195 as u16, 195 as u16,
            122 as u16, 261 as u16, 195 as u16, 321 as u16, 195 as u16,
            195 as u16, 195 as u16, 258 as u16, 238 as u16, 195 as u16,
            195 as u16, 60 as u16, 299 as u16, 291 as u16, 195 as u16,
            195 as u16, 258 as u16, 195 as u16, 195 as u16, 195 as u16,
            290 as u16, 244 as u16, 216 as u16, 72 as u16, 245 as u16,
            193 as u16, 258 as u16, 258 as u16, 299 as u16, 258 as u16,
            299 as u16, 274 as u16, 154 as u16, 155 as u16, 156 as u16,
            157 as u16, 158 as u16, 86 as u16, 247 as u16, 295 as u16,
            248 as u16, 295 as u16, 91 as u16, 19 as u16, 20 as u16,
            270 as u16, 22 as u16, 274 as u16, 270 as u16, 248 as u16,
            274 as u16, 222 as u16, 101 as u16, 227 as u16, 274 as u16,
            221 as u16, 231 as u16, 221 as u16, 107 as u16, 108 as u16,
            36 as u16, 183 as u16, 262 as u16, 247 as u16, 221 as u16,
            283 as u16, 115 as u16, 262 as u16, 117 as u16, 118 as u16,
            119 as u16, 198 as u16, 116 as u16, 122 as u16, 220 as u16,
            262 as u16, 61 as u16, 220 as u16, 220 as u16, 251 as u16,
            247 as u16, 142 as u16, 251 as u16, 245 as u16, 60 as u16,
            202 as u16, 299 as u16, 202 as u16, 38 as u16, 262 as u16,
            202 as u16, 22 as u16, 152 as u16, 151 as u16, 296 as u16,
            43 as u16, 72 as u16, 236 as u16, 18 as u16, 239 as u16,
            202 as u16, 239 as u16, 239 as u16, 239 as u16, 18 as u16,
            154 as u16, 155 as u16, 156 as u16, 157 as u16, 158 as u16,
            86 as u16, 150 as u16, 201 as u16, 248 as u16, 275 as u16,
            91 as u16, 248 as u16, 273 as u16, 236 as u16, 248 as u16,
            275 as u16, 275 as u16, 273 as u16, 236 as u16, 248 as u16,
            101 as u16, 286 as u16, 202 as u16, 201 as u16, 159 as u16,
            63 as u16, 107 as u16, 108 as u16, 296 as u16, 183 as u16,
            293 as u16, 202 as u16, 201 as u16, 22 as u16, 115 as u16,
            202 as u16, 117 as u16, 118 as u16, 119 as u16, 292 as u16,
            223 as u16, 122 as u16, 201 as u16, 65 as u16, 202 as u16,
            201 as u16, 223 as u16, 220 as u16, 220 as u16, 22 as u16,
            220 as u16, 226 as u16, 226 as u16, 229 as u16, 127 as u16,
            223 as u16, 220 as u16, 166 as u16, 24 as u16, 285 as u16,
            220 as u16, 222 as u16, 114 as u16, 315 as u16, 285 as u16,
            220 as u16, 202 as u16, 220 as u16, 307 as u16, 92 as u16,
            320 as u16, 320 as u16, 229 as u16, 154 as u16, 155 as u16,
            156 as u16, 157 as u16, 158 as u16, 0 as u16, 1 as u16, 2 as u16,
            223 as u16, 83 as u16, 5 as u16, 268 as u16, 149 as u16,
            268 as u16, 146 as u16, 10 as u16, 11 as u16, 12 as u16,
            13 as u16, 14 as u16, 22 as u16, 280 as u16, 17 as u16,
            202 as u16, 159 as u16, 19 as u16, 20 as u16, 251 as u16,
            22 as u16, 183 as u16, 282 as u16, 148 as u16, 252 as u16,
            252 as u16, 250 as u16, 30 as u16, 249 as u16, 32 as u16,
            248 as u16, 147 as u16, 25 as u16, 13 as u16, 36 as u16,
            204 as u16, 196 as u16, 40 as u16, 196 as u16, 6 as u16,
            302 as u16, 194 as u16, 194 as u16, 194 as u16, 209 as u16,
            215 as u16, 209 as u16, 215 as u16, 215 as u16, 215 as u16,
            224 as u16, 224 as u16, 216 as u16, 209 as u16, 4 as u16,
            216 as u16, 215 as u16, 3 as u16, 60 as u16, 22 as u16,
            122 as u16, 19 as u16, 122 as u16, 19 as u16, 125 as u16,
            22 as u16, 15 as u16, 22 as u16, 71 as u16, 16 as u16, 72 as u16,
            23 as u16, 23 as u16, 140 as u16, 305 as u16, 152 as u16,
            79 as u16, 25 as u16, 131 as u16, 82 as u16, 143 as u16,
            20 as u16, 16 as u16, 305 as u16, 1 as u16, 143 as u16,
            145 as u16, 131 as u16, 131 as u16, 62 as u16, 54 as u16,
            131 as u16, 37 as u16, 54 as u16, 54 as u16, 152 as u16,
            99 as u16, 117 as u16, 34 as u16, 101 as u16, 54 as u16,
            24 as u16, 1 as u16, 5 as u16, 22 as u16, 107 as u16, 108 as u16,
            116 as u16, 76 as u16, 25 as u16, 162 as u16, 41 as u16,
            142 as u16, 115 as u16, 24 as u16, 117 as u16, 118 as u16,
            119 as u16, 116 as u16, 20 as u16, 122 as u16, 19 as u16,
            126 as u16, 23 as u16, 132 as u16, 19 as u16, 20 as u16,
            69 as u16, 22 as u16, 69 as u16, 22 as u16, 134 as u16, 22 as u16,
            68 as u16, 22 as u16, 22 as u16, 139 as u16, 140 as u16,
            60 as u16, 141 as u16, 68 as u16, 24 as u16, 36 as u16, 28 as u16,
            97 as u16, 22 as u16, 37 as u16, 68 as u16, 23 as u16, 150 as u16,
            34 as u16, 22 as u16, 154 as u16, 155 as u16, 156 as u16,
            157 as u16, 158 as u16, 23 as u16, 23 as u16, 22 as u16,
            163 as u16, 25 as u16, 23 as u16, 142 as u16, 23 as u16,
            98 as u16, 60 as u16, 23 as u16, 22 as u16, 144 as u16, 25 as u16,
            76 as u16, 34 as u16, 117 as u16, 34 as u16, 89 as u16, 34 as u16,
            34 as u16, 72 as u16, 87 as u16, 76 as u16, 183 as u16, 34 as u16,
            94 as u16, 34 as u16, 23 as u16, 22 as u16, 24 as u16, 34 as u16,
            23 as u16, 25 as u16, 44 as u16, 25 as u16, 23 as u16, 23 as u16,
            23 as u16, 22 as u16, 22 as u16, 25 as u16, 11 as u16, 143 as u16,
            25 as u16, 143 as u16, 23 as u16, 22 as u16, 22 as u16, 22 as u16,
            101 as u16, 23 as u16, 23 as u16, 136 as u16, 22 as u16,
            25 as u16, 107 as u16, 108 as u16, 142 as u16, 25 as u16,
            142 as u16, 142 as u16, 23 as u16, 15 as u16, 115 as u16,
            1 as u16, 117 as u16, 118 as u16, 119 as u16, 1 as u16, 2 as u16,
            122 as u16, 1 as u16, 5 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 10 as u16, 11 as u16, 12 as u16,
            13 as u16, 14 as u16, 322 as u16, 322 as u16, 17 as u16,
            322 as u16, 5 as u16, 322 as u16, 322 as u16, 141 as u16,
            322 as u16, 10 as u16, 11 as u16, 12 as u16, 13 as u16, 14 as u16,
            322 as u16, 30 as u16, 17 as u16, 32 as u16, 322 as u16,
            322 as u16, 154 as u16, 155 as u16, 156 as u16, 157 as u16,
            158 as u16, 40 as u16, 322 as u16, 322 as u16, 322 as u16,
            30 as u16, 322 as u16, 32 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            40 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            183 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 71 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            79 as u16, 322 as u16, 322 as u16, 82 as u16, 322 as u16,
            322 as u16, 71 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 79 as u16,
            322 as u16, 322 as u16, 82 as u16, 322 as u16, 322 as u16,
            99 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 99 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            134 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            139 as u16, 140 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 134 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 139 as u16,
            140 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 163 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 163 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 322 as u16, 322 as u16,
            322 as u16, 322 as u16, 322 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16, 187 as u16, 187 as u16, 187 as u16, 187 as u16,
            187 as u16];

static yy_fallback: [u16; 187] =
    [0 as u16, 0 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 0 as u16,
            60 as u16, 60 as u16, 60 as u16, 0 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 0 as u16, 0 as u16, 0 as u16, 60 as u16,
            0 as u16, 0 as u16, 60 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 60 as u16, 60 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16,
            60 as u16, 60 as u16, 60 as u16, 60 as u16, 60 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16,
            0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16];

static yy_action: [u16; 2379] =
    [134 as u16, 131 as u16, 238 as u16, 290 as u16, 290 as u16, 1353 as u16,
            593 as u16, 1332 as u16, 478 as u16, 1606 as u16, 593 as u16,
            1315 as u16, 593 as u16, 7 as u16, 593 as u16, 1353 as u16,
            590 as u16, 593 as u16, 579 as u16, 424 as u16, 1566 as u16,
            134 as u16, 131 as u16, 238 as u16, 1318 as u16, 541 as u16,
            478 as u16, 477 as u16, 575 as u16, 84 as u16, 84 as u16,
            1005 as u16, 303 as u16, 84 as u16, 84 as u16, 51 as u16,
            51 as u16, 63 as u16, 63 as u16, 1006 as u16, 84 as u16,
            84 as u16, 498 as u16, 141 as u16, 142 as u16, 93 as u16,
            442 as u16, 1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16,
            1075 as u16, 1075 as u16, 139 as u16, 139 as u16, 140 as u16,
            140 as u16, 140 as u16, 140 as u16, 424 as u16, 296 as u16,
            296 as u16, 498 as u16, 296 as u16, 296 as u16, 567 as u16,
            553 as u16, 296 as u16, 296 as u16, 1306 as u16, 574 as u16,
            1358 as u16, 1358 as u16, 590 as u16, 542 as u16, 579 as u16,
            590 as u16, 574 as u16, 579 as u16, 548 as u16, 590 as u16,
            1304 as u16, 579 as u16, 141 as u16, 142 as u16, 93 as u16,
            576 as u16, 1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16,
            1075 as u16, 1075 as u16, 139 as u16, 139 as u16, 140 as u16,
            140 as u16, 140 as u16, 140 as u16, 399 as u16, 478 as u16,
            395 as u16, 6 as u16, 138 as u16, 138 as u16, 138 as u16,
            138 as u16, 137 as u16, 137 as u16, 136 as u16, 136 as u16,
            136 as u16, 135 as u16, 132 as u16, 463 as u16, 44 as u16,
            342 as u16, 593 as u16, 305 as u16, 1127 as u16, 1280 as u16,
            1 as u16, 1 as u16, 599 as u16, 2 as u16, 1284 as u16, 598 as u16,
            1200 as u16, 1284 as u16, 1200 as u16, 330 as u16, 424 as u16,
            158 as u16, 330 as u16, 1613 as u16, 158 as u16, 390 as u16,
            116 as u16, 308 as u16, 1366 as u16, 51 as u16, 51 as u16,
            1366 as u16, 138 as u16, 138 as u16, 138 as u16, 138 as u16,
            137 as u16, 137 as u16, 136 as u16, 136 as u16, 136 as u16,
            135 as u16, 132 as u16, 463 as u16, 141 as u16, 142 as u16,
            93 as u16, 515 as u16, 1254 as u16, 1254 as u16, 1085 as u16,
            1088 as u16, 1075 as u16, 1075 as u16, 139 as u16, 139 as u16,
            140 as u16, 140 as u16, 140 as u16, 140 as u16, 1230 as u16,
            329 as u16, 584 as u16, 296 as u16, 296 as u16, 212 as u16,
            296 as u16, 296 as u16, 568 as u16, 568 as u16, 488 as u16,
            143 as u16, 1072 as u16, 1072 as u16, 1086 as u16, 1089 as u16,
            590 as u16, 1195 as u16, 579 as u16, 590 as u16, 340 as u16,
            579 as u16, 140 as u16, 140 as u16, 140 as u16, 140 as u16,
            133 as u16, 392 as u16, 564 as u16, 536 as u16, 1195 as u16,
            250 as u16, 425 as u16, 1195 as u16, 250 as u16, 137 as u16,
            137 as u16, 136 as u16, 136 as u16, 136 as u16, 135 as u16,
            132 as u16, 463 as u16, 291 as u16, 138 as u16, 138 as u16,
            138 as u16, 138 as u16, 137 as u16, 137 as u16, 136 as u16,
            136 as u16, 136 as u16, 135 as u16, 132 as u16, 463 as u16,
            966 as u16, 1230 as u16, 1231 as u16, 1230 as u16, 412 as u16,
            965 as u16, 467 as u16, 412 as u16, 424 as u16, 467 as u16,
            489 as u16, 357 as u16, 1611 as u16, 391 as u16, 138 as u16,
            138 as u16, 138 as u16, 138 as u16, 137 as u16, 137 as u16,
            136 as u16, 136 as u16, 136 as u16, 135 as u16, 132 as u16,
            463 as u16, 463 as u16, 134 as u16, 131 as u16, 238 as u16,
            555 as u16, 1076 as u16, 141 as u16, 142 as u16, 93 as u16,
            593 as u16, 1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16,
            1075 as u16, 1075 as u16, 139 as u16, 139 as u16, 140 as u16,
            140 as u16, 140 as u16, 140 as u16, 1317 as u16, 134 as u16,
            131 as u16, 238 as u16, 424 as u16, 549 as u16, 1597 as u16,
            1531 as u16, 333 as u16, 97 as u16, 83 as u16, 83 as u16,
            140 as u16, 140 as u16, 140 as u16, 140 as u16, 138 as u16,
            138 as u16, 138 as u16, 138 as u16, 137 as u16, 137 as u16,
            136 as u16, 136 as u16, 136 as u16, 135 as u16, 132 as u16,
            463 as u16, 141 as u16, 142 as u16, 93 as u16, 1657 as u16,
            1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16,
            1075 as u16, 139 as u16, 139 as u16, 140 as u16, 140 as u16,
            140 as u16, 140 as u16, 138 as u16, 138 as u16, 138 as u16,
            138 as u16, 137 as u16, 137 as u16, 136 as u16, 136 as u16,
            136 as u16, 135 as u16, 132 as u16, 463 as u16, 591 as u16,
            1230 as u16, 958 as u16, 958 as u16, 138 as u16, 138 as u16,
            138 as u16, 138 as u16, 137 as u16, 137 as u16, 136 as u16,
            136 as u16, 136 as u16, 135 as u16, 132 as u16, 463 as u16,
            44 as u16, 398 as u16, 547 as u16, 1306 as u16, 136 as u16,
            136 as u16, 136 as u16, 135 as u16, 132 as u16, 463 as u16,
            386 as u16, 593 as u16, 442 as u16, 595 as u16, 145 as u16,
            595 as u16, 138 as u16, 138 as u16, 138 as u16, 138 as u16,
            137 as u16, 137 as u16, 136 as u16, 136 as u16, 136 as u16,
            135 as u16, 132 as u16, 463 as u16, 500 as u16, 1230 as u16,
            112 as u16, 550 as u16, 460 as u16, 459 as u16, 51 as u16,
            51 as u16, 424 as u16, 296 as u16, 296 as u16, 479 as u16,
            334 as u16, 1259 as u16, 1230 as u16, 1231 as u16, 1230 as u16,
            1599 as u16, 1261 as u16, 388 as u16, 312 as u16, 444 as u16,
            590 as u16, 246 as u16, 579 as u16, 546 as u16, 1260 as u16,
            271 as u16, 235 as u16, 329 as u16, 584 as u16, 551 as u16,
            141 as u16, 142 as u16, 93 as u16, 429 as u16, 1254 as u16,
            1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16,
            139 as u16, 139 as u16, 140 as u16, 140 as u16, 140 as u16,
            140 as u16, 22 as u16, 22 as u16, 1230 as u16, 1262 as u16,
            424 as u16, 1262 as u16, 216 as u16, 296 as u16, 296 as u16,
            98 as u16, 1230 as u16, 1231 as u16, 1230 as u16, 264 as u16,
            884 as u16, 45 as u16, 528 as u16, 525 as u16, 524 as u16,
            1041 as u16, 590 as u16, 1269 as u16, 579 as u16, 421 as u16,
            420 as u16, 393 as u16, 523 as u16, 44 as u16, 141 as u16,
            142 as u16, 93 as u16, 498 as u16, 1254 as u16, 1254 as u16,
            1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16, 139 as u16,
            139 as u16, 140 as u16, 140 as u16, 140 as u16, 140 as u16,
            138 as u16, 138 as u16, 138 as u16, 138 as u16, 137 as u16,
            137 as u16, 136 as u16, 136 as u16, 136 as u16, 135 as u16,
            132 as u16, 463 as u16, 593 as u16, 1611 as u16, 561 as u16,
            1230 as u16, 1231 as u16, 1230 as u16, 23 as u16, 264 as u16,
            515 as u16, 200 as u16, 528 as u16, 525 as u16, 524 as u16,
            127 as u16, 585 as u16, 509 as u16, 4 as u16, 355 as u16,
            487 as u16, 506 as u16, 523 as u16, 593 as u16, 498 as u16,
            84 as u16, 84 as u16, 134 as u16, 131 as u16, 238 as u16,
            329 as u16, 584 as u16, 588 as u16, 1627 as u16, 138 as u16,
            138 as u16, 138 as u16, 138 as u16, 137 as u16, 137 as u16,
            136 as u16, 136 as u16, 136 as u16, 135 as u16, 132 as u16,
            463 as u16, 19 as u16, 19 as u16, 435 as u16, 1230 as u16,
            1460 as u16, 297 as u16, 297 as u16, 311 as u16, 424 as u16,
            1565 as u16, 464 as u16, 1631 as u16, 599 as u16, 2 as u16,
            1284 as u16, 437 as u16, 574 as u16, 1107 as u16, 590 as u16,
            330 as u16, 579 as u16, 158 as u16, 582 as u16, 489 as u16,
            357 as u16, 573 as u16, 593 as u16, 592 as u16, 1366 as u16,
            409 as u16, 1274 as u16, 1230 as u16, 141 as u16, 142 as u16,
            93 as u16, 1364 as u16, 1254 as u16, 1254 as u16, 1085 as u16,
            1088 as u16, 1075 as u16, 1075 as u16, 139 as u16, 139 as u16,
            140 as u16, 140 as u16, 140 as u16, 140 as u16, 389 as u16,
            84 as u16, 84 as u16, 1062 as u16, 567 as u16, 1230 as u16,
            313 as u16, 1523 as u16, 593 as u16, 125 as u16, 125 as u16,
            970 as u16, 1230 as u16, 1231 as u16, 1230 as u16, 296 as u16,
            296 as u16, 126 as u16, 46 as u16, 464 as u16, 594 as u16,
            464 as u16, 296 as u16, 296 as u16, 1050 as u16, 1230 as u16,
            218 as u16, 439 as u16, 590 as u16, 1604 as u16, 579 as u16,
            84 as u16, 84 as u16, 7 as u16, 403 as u16, 590 as u16,
            515 as u16, 579 as u16, 325 as u16, 417 as u16, 1230 as u16,
            1231 as u16, 1230 as u16, 250 as u16, 138 as u16, 138 as u16,
            138 as u16, 138 as u16, 137 as u16, 137 as u16, 136 as u16,
            136 as u16, 136 as u16, 135 as u16, 132 as u16, 463 as u16,
            1050 as u16, 1050 as u16, 1052 as u16, 1053 as u16, 35 as u16,
            1275 as u16, 1230 as u16, 1231 as u16, 1230 as u16, 424 as u16,
            1370 as u16, 993 as u16, 574 as u16, 371 as u16, 414 as u16,
            274 as u16, 412 as u16, 1597 as u16, 467 as u16, 1302 as u16,
            552 as u16, 451 as u16, 590 as u16, 543 as u16, 579 as u16,
            1530 as u16, 1230 as u16, 1231 as u16, 1230 as u16, 1214 as u16,
            201 as u16, 409 as u16, 1174 as u16, 141 as u16, 142 as u16,
            93 as u16, 223 as u16, 1254 as u16, 1254 as u16, 1085 as u16,
            1088 as u16, 1075 as u16, 1075 as u16, 139 as u16, 139 as u16,
            140 as u16, 140 as u16, 140 as u16, 140 as u16, 296 as u16,
            296 as u16, 1250 as u16, 593 as u16, 424 as u16, 296 as u16,
            296 as u16, 236 as u16, 529 as u16, 296 as u16, 296 as u16,
            515 as u16, 100 as u16, 590 as u16, 1600 as u16, 579 as u16,
            48 as u16, 1605 as u16, 590 as u16, 1230 as u16, 579 as u16,
            7 as u16, 590 as u16, 577 as u16, 579 as u16, 904 as u16,
            84 as u16, 84 as u16, 141 as u16, 142 as u16, 93 as u16,
            496 as u16, 1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16,
            1075 as u16, 1075 as u16, 139 as u16, 139 as u16, 140 as u16,
            140 as u16, 140 as u16, 140 as u16, 138 as u16, 138 as u16,
            138 as u16, 138 as u16, 137 as u16, 137 as u16, 136 as u16,
            136 as u16, 136 as u16, 135 as u16, 132 as u16, 463 as u16,
            1365 as u16, 1230 as u16, 296 as u16, 296 as u16, 1250 as u16,
            115 as u16, 1275 as u16, 326 as u16, 233 as u16, 539 as u16,
            1062 as u16, 40 as u16, 282 as u16, 127 as u16, 585 as u16,
            590 as u16, 4 as u16, 579 as u16, 329 as u16, 584 as u16,
            1230 as u16, 1231 as u16, 1230 as u16, 1598 as u16, 593 as u16,
            388 as u16, 904 as u16, 1051 as u16, 1356 as u16, 1356 as u16,
            588 as u16, 1050 as u16, 138 as u16, 138 as u16, 138 as u16,
            138 as u16, 137 as u16, 137 as u16, 136 as u16, 136 as u16,
            136 as u16, 135 as u16, 132 as u16, 463 as u16, 185 as u16,
            593 as u16, 1230 as u16, 19 as u16, 19 as u16, 1230 as u16,
            971 as u16, 1597 as u16, 424 as u16, 1651 as u16, 464 as u16,
            129 as u16, 908 as u16, 1195 as u16, 1230 as u16, 1231 as u16,
            1230 as u16, 1325 as u16, 443 as u16, 1050 as u16, 1050 as u16,
            1052 as u16, 582 as u16, 1603 as u16, 149 as u16, 149 as u16,
            1195 as u16, 7 as u16, 5 as u16, 1195 as u16, 1687 as u16,
            410 as u16, 141 as u16, 142 as u16, 93 as u16, 1536 as u16,
            1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16,
            1075 as u16, 139 as u16, 139 as u16, 140 as u16, 140 as u16,
            140 as u16, 140 as u16, 1214 as u16, 397 as u16, 593 as u16,
            1062 as u16, 424 as u16, 1536 as u16, 1538 as u16, 50 as u16,
            901 as u16, 125 as u16, 125 as u16, 1230 as u16, 1231 as u16,
            1230 as u16, 1230 as u16, 1231 as u16, 1230 as u16, 126 as u16,
            1230 as u16, 464 as u16, 594 as u16, 464 as u16, 515 as u16,
            1230 as u16, 1050 as u16, 84 as u16, 84 as u16, 3 as u16,
            141 as u16, 142 as u16, 93 as u16, 924 as u16, 1254 as u16,
            1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16,
            139 as u16, 139 as u16, 140 as u16, 140 as u16, 140 as u16,
            140 as u16, 138 as u16, 138 as u16, 138 as u16, 138 as u16,
            137 as u16, 137 as u16, 136 as u16, 136 as u16, 136 as u16,
            135 as u16, 132 as u16, 463 as u16, 1050 as u16, 1050 as u16,
            1052 as u16, 1053 as u16, 35 as u16, 442 as u16, 457 as u16,
            532 as u16, 433 as u16, 1230 as u16, 1062 as u16, 1361 as u16,
            540 as u16, 540 as u16, 1598 as u16, 925 as u16, 388 as u16,
            7 as u16, 1129 as u16, 1230 as u16, 1231 as u16, 1230 as u16,
            1129 as u16, 1536 as u16, 1230 as u16, 1231 as u16, 1230 as u16,
            1051 as u16, 570 as u16, 1214 as u16, 593 as u16, 1050 as u16,
            138 as u16, 138 as u16, 138 as u16, 138 as u16, 137 as u16,
            137 as u16, 136 as u16, 136 as u16, 136 as u16, 135 as u16,
            132 as u16, 463 as u16, 6 as u16, 185 as u16, 1195 as u16,
            1230 as u16, 231 as u16, 593 as u16, 382 as u16, 992 as u16,
            424 as u16, 151 as u16, 151 as u16, 510 as u16, 1213 as u16,
            557 as u16, 482 as u16, 1195 as u16, 381 as u16, 160 as u16,
            1195 as u16, 1050 as u16, 1050 as u16, 1052 as u16, 1230 as u16,
            1231 as u16, 1230 as u16, 422 as u16, 593 as u16, 447 as u16,
            84 as u16, 84 as u16, 593 as u16, 217 as u16, 141 as u16,
            142 as u16, 93 as u16, 593 as u16, 1254 as u16, 1254 as u16,
            1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16, 139 as u16,
            139 as u16, 140 as u16, 140 as u16, 140 as u16, 140 as u16,
            1214 as u16, 19 as u16, 19 as u16, 593 as u16, 424 as u16,
            19 as u16, 19 as u16, 442 as u16, 1063 as u16, 442 as u16,
            19 as u16, 19 as u16, 1230 as u16, 1231 as u16, 1230 as u16,
            515 as u16, 445 as u16, 458 as u16, 1597 as u16, 386 as u16,
            315 as u16, 1175 as u16, 1685 as u16, 556 as u16, 1685 as u16,
            450 as u16, 84 as u16, 84 as u16, 141 as u16, 142 as u16,
            93 as u16, 505 as u16, 1254 as u16, 1254 as u16, 1085 as u16,
            1088 as u16, 1075 as u16, 1075 as u16, 139 as u16, 139 as u16,
            140 as u16, 140 as u16, 140 as u16, 140 as u16, 138 as u16,
            138 as u16, 138 as u16, 138 as u16, 137 as u16, 137 as u16,
            136 as u16, 136 as u16, 136 as u16, 135 as u16, 132 as u16,
            463 as u16, 442 as u16, 1147 as u16, 454 as u16, 1597 as u16,
            362 as u16, 1041 as u16, 593 as u16, 462 as u16, 1460 as u16,
            1233 as u16, 47 as u16, 1393 as u16, 324 as u16, 565 as u16,
            565 as u16, 115 as u16, 1148 as u16, 449 as u16, 7 as u16,
            460 as u16, 459 as u16, 307 as u16, 375 as u16, 354 as u16,
            593 as u16, 113 as u16, 593 as u16, 329 as u16, 584 as u16,
            19 as u16, 19 as u16, 1149 as u16, 138 as u16, 138 as u16,
            138 as u16, 138 as u16, 137 as u16, 137 as u16, 136 as u16,
            136 as u16, 136 as u16, 135 as u16, 132 as u16, 463 as u16,
            209 as u16, 1173 as u16, 563 as u16, 19 as u16, 19 as u16,
            19 as u16, 19 as u16, 49 as u16, 424 as u16, 944 as u16,
            1175 as u16, 1686 as u16, 1046 as u16, 1686 as u16, 218 as u16,
            355 as u16, 484 as u16, 343 as u16, 210 as u16, 945 as u16,
            569 as u16, 562 as u16, 1262 as u16, 1233 as u16, 1262 as u16,
            490 as u16, 314 as u16, 423 as u16, 424 as u16, 1598 as u16,
            1206 as u16, 388 as u16, 141 as u16, 142 as u16, 93 as u16,
            440 as u16, 1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16,
            1075 as u16, 1075 as u16, 139 as u16, 139 as u16, 140 as u16,
            140 as u16, 140 as u16, 140 as u16, 352 as u16, 316 as u16,
            531 as u16, 316 as u16, 141 as u16, 142 as u16, 93 as u16,
            549 as u16, 1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16,
            1075 as u16, 1075 as u16, 139 as u16, 139 as u16, 140 as u16,
            140 as u16, 140 as u16, 140 as u16, 446 as u16, 10 as u16,
            1598 as u16, 274 as u16, 388 as u16, 915 as u16, 281 as u16,
            299 as u16, 383 as u16, 534 as u16, 378 as u16, 533 as u16,
            269 as u16, 593 as u16, 1206 as u16, 587 as u16, 587 as u16,
            587 as u16, 374 as u16, 293 as u16, 1579 as u16, 991 as u16,
            1173 as u16, 302 as u16, 138 as u16, 138 as u16, 138 as u16,
            138 as u16, 137 as u16, 137 as u16, 136 as u16, 136 as u16,
            136 as u16, 135 as u16, 132 as u16, 463 as u16, 53 as u16,
            53 as u16, 520 as u16, 1250 as u16, 593 as u16, 1147 as u16,
            1576 as u16, 431 as u16, 138 as u16, 138 as u16, 138 as u16,
            138 as u16, 137 as u16, 137 as u16, 136 as u16, 136 as u16,
            136 as u16, 135 as u16, 132 as u16, 463 as u16, 1148 as u16,
            301 as u16, 593 as u16, 1577 as u16, 593 as u16, 1307 as u16,
            431 as u16, 54 as u16, 54 as u16, 593 as u16, 268 as u16,
            593 as u16, 461 as u16, 461 as u16, 461 as u16, 1149 as u16,
            347 as u16, 492 as u16, 424 as u16, 135 as u16, 132 as u16,
            463 as u16, 1146 as u16, 1195 as u16, 474 as u16, 68 as u16,
            68 as u16, 69 as u16, 69 as u16, 550 as u16, 332 as u16,
            287 as u16, 21 as u16, 21 as u16, 55 as u16, 55 as u16,
            1195 as u16, 581 as u16, 424 as u16, 1195 as u16, 309 as u16,
            1250 as u16, 141 as u16, 142 as u16, 93 as u16, 119 as u16,
            1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16,
            1075 as u16, 139 as u16, 139 as u16, 140 as u16, 140 as u16,
            140 as u16, 140 as u16, 593 as u16, 237 as u16, 480 as u16,
            1476 as u16, 141 as u16, 142 as u16, 93 as u16, 593 as u16,
            1254 as u16, 1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16,
            1075 as u16, 139 as u16, 139 as u16, 140 as u16, 140 as u16,
            140 as u16, 140 as u16, 344 as u16, 430 as u16, 346 as u16,
            70 as u16, 70 as u16, 494 as u16, 991 as u16, 1132 as u16,
            1132 as u16, 512 as u16, 56 as u16, 56 as u16, 1269 as u16,
            593 as u16, 268 as u16, 593 as u16, 369 as u16, 374 as u16,
            593 as u16, 481 as u16, 215 as u16, 384 as u16, 1624 as u16,
            481 as u16, 138 as u16, 138 as u16, 138 as u16, 138 as u16,
            137 as u16, 137 as u16, 136 as u16, 136 as u16, 136 as u16,
            135 as u16, 132 as u16, 463 as u16, 71 as u16, 71 as u16,
            72 as u16, 72 as u16, 225 as u16, 73 as u16, 73 as u16,
            593 as u16, 138 as u16, 138 as u16, 138 as u16, 138 as u16,
            137 as u16, 137 as u16, 136 as u16, 136 as u16, 136 as u16,
            135 as u16, 132 as u16, 463 as u16, 586 as u16, 431 as u16,
            593 as u16, 872 as u16, 873 as u16, 874 as u16, 593 as u16,
            911 as u16, 593 as u16, 1602 as u16, 74 as u16, 74 as u16,
            593 as u16, 7 as u16, 1460 as u16, 242 as u16, 593 as u16,
            306 as u16, 424 as u16, 1578 as u16, 472 as u16, 306 as u16,
            364 as u16, 219 as u16, 367 as u16, 75 as u16, 75 as u16,
            430 as u16, 345 as u16, 57 as u16, 57 as u16, 58 as u16,
            58 as u16, 432 as u16, 187 as u16, 59 as u16, 59 as u16,
            593 as u16, 424 as u16, 61 as u16, 61 as u16, 1475 as u16,
            141 as u16, 142 as u16, 93 as u16, 123 as u16, 1254 as u16,
            1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16,
            139 as u16, 139 as u16, 140 as u16, 140 as u16, 140 as u16,
            140 as u16, 424 as u16, 570 as u16, 62 as u16, 62 as u16,
            141 as u16, 142 as u16, 93 as u16, 911 as u16, 1254 as u16,
            1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16,
            139 as u16, 139 as u16, 140 as u16, 140 as u16, 140 as u16,
            140 as u16, 161 as u16, 384 as u16, 1624 as u16, 1474 as u16,
            141 as u16, 130 as u16, 93 as u16, 441 as u16, 1254 as u16,
            1254 as u16, 1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16,
            139 as u16, 139 as u16, 140 as u16, 140 as u16, 140 as u16,
            140 as u16, 267 as u16, 266 as u16, 265 as u16, 1460 as u16,
            138 as u16, 138 as u16, 138 as u16, 138 as u16, 137 as u16,
            137 as u16, 136 as u16, 136 as u16, 136 as u16, 135 as u16,
            132 as u16, 463 as u16, 593 as u16, 1336 as u16, 593 as u16,
            1269 as u16, 1460 as u16, 384 as u16, 1624 as u16, 231 as u16,
            138 as u16, 138 as u16, 138 as u16, 138 as u16, 137 as u16,
            137 as u16, 136 as u16, 136 as u16, 136 as u16, 135 as u16,
            132 as u16, 463 as u16, 593 as u16, 163 as u16, 593 as u16,
            76 as u16, 76 as u16, 77 as u16, 77 as u16, 593 as u16,
            138 as u16, 138 as u16, 138 as u16, 138 as u16, 137 as u16,
            137 as u16, 136 as u16, 136 as u16, 136 as u16, 135 as u16,
            132 as u16, 463 as u16, 475 as u16, 593 as u16, 483 as u16,
            78 as u16, 78 as u16, 20 as u16, 20 as u16, 1249 as u16,
            424 as u16, 491 as u16, 79 as u16, 79 as u16, 495 as u16,
            422 as u16, 295 as u16, 235 as u16, 1574 as u16, 38 as u16,
            511 as u16, 896 as u16, 422 as u16, 335 as u16, 240 as u16,
            422 as u16, 147 as u16, 147 as u16, 112 as u16, 593 as u16,
            424 as u16, 593 as u16, 101 as u16, 222 as u16, 991 as u16,
            142 as u16, 93 as u16, 455 as u16, 1254 as u16, 1254 as u16,
            1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16, 139 as u16,
            139 as u16, 140 as u16, 140 as u16, 140 as u16, 140 as u16,
            593 as u16, 39 as u16, 148 as u16, 148 as u16, 80 as u16,
            80 as u16, 93 as u16, 551 as u16, 1254 as u16, 1254 as u16,
            1085 as u16, 1088 as u16, 1075 as u16, 1075 as u16, 139 as u16,
            139 as u16, 140 as u16, 140 as u16, 140 as u16, 140 as u16,
            328 as u16, 923 as u16, 922 as u16, 64 as u16, 64 as u16,
            502 as u16, 1656 as u16, 1005 as u16, 933 as u16, 896 as u16,
            124 as u16, 422 as u16, 121 as u16, 254 as u16, 593 as u16,
            1006 as u16, 593 as u16, 226 as u16, 593 as u16, 127 as u16,
            585 as u16, 164 as u16, 4 as u16, 16 as u16, 138 as u16,
            138 as u16, 138 as u16, 138 as u16, 137 as u16, 137 as u16,
            136 as u16, 136 as u16, 136 as u16, 135 as u16, 132 as u16,
            463 as u16, 588 as u16, 81 as u16, 81 as u16, 65 as u16,
            65 as u16, 82 as u16, 82 as u16, 593 as u16, 138 as u16,
            138 as u16, 138 as u16, 138 as u16, 137 as u16, 137 as u16,
            136 as u16, 136 as u16, 136 as u16, 135 as u16, 132 as u16,
            463 as u16, 593 as u16, 226 as u16, 237 as u16, 966 as u16,
            464 as u16, 593 as u16, 298 as u16, 593 as u16, 965 as u16,
            593 as u16, 66 as u16, 66 as u16, 593 as u16, 1170 as u16,
            593 as u16, 411 as u16, 582 as u16, 353 as u16, 469 as u16,
            115 as u16, 593 as u16, 471 as u16, 169 as u16, 173 as u16,
            173 as u16, 593 as u16, 44 as u16, 991 as u16, 174 as u16,
            174 as u16, 89 as u16, 89 as u16, 67 as u16, 67 as u16,
            593 as u16, 85 as u16, 85 as u16, 150 as u16, 150 as u16,
            1114 as u16, 1043 as u16, 593 as u16, 273 as u16, 86 as u16,
            86 as u16, 1062 as u16, 593 as u16, 503 as u16, 171 as u16,
            171 as u16, 593 as u16, 125 as u16, 125 as u16, 497 as u16,
            593 as u16, 273 as u16, 336 as u16, 152 as u16, 152 as u16,
            126 as u16, 1335 as u16, 464 as u16, 594 as u16, 464 as u16,
            146 as u16, 146 as u16, 1050 as u16, 593 as u16, 545 as u16,
            172 as u16, 172 as u16, 593 as u16, 1054 as u16, 165 as u16,
            165 as u16, 256 as u16, 339 as u16, 156 as u16, 156 as u16,
            127 as u16, 585 as u16, 1586 as u16, 4 as u16, 329 as u16,
            584 as u16, 499 as u16, 358 as u16, 273 as u16, 115 as u16,
            348 as u16, 155 as u16, 155 as u16, 930 as u16, 931 as u16,
            153 as u16, 153 as u16, 588 as u16, 1114 as u16, 1050 as u16,
            1050 as u16, 1052 as u16, 1053 as u16, 35 as u16, 1554 as u16,
            521 as u16, 593 as u16, 270 as u16, 1008 as u16, 1009 as u16,
            9 as u16, 593 as u16, 372 as u16, 593 as u16, 115 as u16,
            593 as u16, 168 as u16, 593 as u16, 115 as u16, 593 as u16,
            1110 as u16, 464 as u16, 270 as u16, 996 as u16, 964 as u16,
            273 as u16, 129 as u16, 1645 as u16, 1214 as u16, 154 as u16,
            154 as u16, 1054 as u16, 1404 as u16, 582 as u16, 88 as u16,
            88 as u16, 90 as u16, 90 as u16, 87 as u16, 87 as u16, 52 as u16,
            52 as u16, 60 as u16, 60 as u16, 1405 as u16, 504 as u16,
            537 as u16, 559 as u16, 1179 as u16, 961 as u16, 507 as u16,
            129 as u16, 558 as u16, 127 as u16, 585 as u16, 1126 as u16,
            4 as u16, 1126 as u16, 1125 as u16, 894 as u16, 1125 as u16,
            162 as u16, 1062 as u16, 963 as u16, 359 as u16, 129 as u16,
            1401 as u16, 363 as u16, 125 as u16, 125 as u16, 588 as u16,
            366 as u16, 368 as u16, 370 as u16, 1349 as u16, 1334 as u16,
            126 as u16, 1333 as u16, 464 as u16, 594 as u16, 464 as u16,
            377 as u16, 387 as u16, 1050 as u16, 1391 as u16, 1414 as u16,
            1618 as u16, 1459 as u16, 1387 as u16, 1399 as u16, 208 as u16,
            580 as u16, 1464 as u16, 1314 as u16, 464 as u16, 243 as u16,
            516 as u16, 1305 as u16, 1293 as u16, 1384 as u16, 1292 as u16,
            1294 as u16, 1638 as u16, 288 as u16, 170 as u16, 228 as u16,
            582 as u16, 12 as u16, 408 as u16, 321 as u16, 322 as u16,
            241 as u16, 323 as u16, 245 as u16, 1446 as u16, 1050 as u16,
            1050 as u16, 1052 as u16, 1053 as u16, 35 as u16, 559 as u16,
            304 as u16, 350 as u16, 351 as u16, 501 as u16, 560 as u16,
            127 as u16, 585 as u16, 1441 as u16, 4 as u16, 1451 as u16,
            1434 as u16, 310 as u16, 1450 as u16, 526 as u16, 1062 as u16,
            1332 as u16, 415 as u16, 380 as u16, 232 as u16, 1527 as u16,
            125 as u16, 125 as u16, 588 as u16, 1214 as u16, 1396 as u16,
            356 as u16, 1526 as u16, 583 as u16, 126 as u16, 1397 as u16,
            464 as u16, 594 as u16, 464 as u16, 1641 as u16, 535 as u16,
            1050 as u16, 1581 as u16, 1395 as u16, 1269 as u16, 1583 as u16,
            1582 as u16, 213 as u16, 402 as u16, 277 as u16, 214 as u16,
            227 as u16, 464 as u16, 1573 as u16, 239 as u16, 1571 as u16,
            1266 as u16, 1394 as u16, 434 as u16, 198 as u16, 100 as u16,
            224 as u16, 96 as u16, 183 as u16, 582 as u16, 191 as u16,
            485 as u16, 193 as u16, 486 as u16, 194 as u16, 195 as u16,
            196 as u16, 519 as u16, 1050 as u16, 1050 as u16, 1052 as u16,
            1053 as u16, 35 as u16, 559 as u16, 113 as u16, 252 as u16,
            413 as u16, 1447 as u16, 558 as u16, 493 as u16, 13 as u16,
            1455 as u16, 416 as u16, 1453 as u16, 1452 as u16, 14 as u16,
            202 as u16, 1521 as u16, 1062 as u16, 1532 as u16, 508 as u16,
            258 as u16, 106 as u16, 514 as u16, 125 as u16, 125 as u16,
            99 as u16, 1214 as u16, 1543 as u16, 289 as u16, 260 as u16,
            206 as u16, 126 as u16, 365 as u16, 464 as u16, 594 as u16,
            464 as u16, 361 as u16, 517 as u16, 1050 as u16, 261 as u16,
            448 as u16, 1295 as u16, 262 as u16, 418 as u16, 1352 as u16,
            1351 as u16, 108 as u16, 1350 as u16, 1655 as u16, 1654 as u16,
            1343 as u16, 915 as u16, 419 as u16, 1322 as u16, 233 as u16,
            452 as u16, 319 as u16, 379 as u16, 1321 as u16, 453 as u16,
            1623 as u16, 320 as u16, 1320 as u16, 275 as u16, 1653 as u16,
            544 as u16, 276 as u16, 1609 as u16, 1608 as u16, 1342 as u16,
            1050 as u16, 1050 as u16, 1052 as u16, 1053 as u16, 35 as u16,
            1630 as u16, 1218 as u16, 466 as u16, 385 as u16, 456 as u16,
            300 as u16, 1419 as u16, 144 as u16, 1418 as u16, 570 as u16,
            407 as u16, 407 as u16, 406 as u16, 284 as u16, 404 as u16,
            11 as u16, 1508 as u16, 881 as u16, 396 as u16, 120 as u16,
            127 as u16, 585 as u16, 394 as u16, 4 as u16, 1214 as u16,
            327 as u16, 114 as u16, 1375 as u16, 1374 as u16, 220 as u16,
            247 as u16, 400 as u16, 338 as u16, 401 as u16, 554 as u16,
            42 as u16, 1224 as u16, 588 as u16, 596 as u16, 283 as u16,
            337 as u16, 285 as u16, 286 as u16, 188 as u16, 597 as u16,
            1290 as u16, 1285 as u16, 175 as u16, 1558 as u16, 176 as u16,
            1559 as u16, 1557 as u16, 1556 as u16, 159 as u16, 317 as u16,
            229 as u16, 177 as u16, 868 as u16, 230 as u16, 91 as u16,
            465 as u16, 464 as u16, 221 as u16, 331 as u16, 468 as u16,
            1165 as u16, 470 as u16, 473 as u16, 94 as u16, 244 as u16,
            95 as u16, 249 as u16, 189 as u16, 582 as u16, 1124 as u16,
            1122 as u16, 341 as u16, 427 as u16, 190 as u16, 178 as u16,
            1249 as u16, 179 as u16, 43 as u16, 192 as u16, 947 as u16,
            349 as u16, 428 as u16, 1138 as u16, 197 as u16, 251 as u16,
            180 as u16, 181 as u16, 436 as u16, 102 as u16, 182 as u16,
            438 as u16, 103 as u16, 104 as u16, 199 as u16, 248 as u16,
            1140 as u16, 253 as u16, 1062 as u16, 105 as u16, 255 as u16,
            1137 as u16, 166 as u16, 24 as u16, 125 as u16, 125 as u16,
            257 as u16, 1264 as u16, 273 as u16, 360 as u16, 513 as u16,
            259 as u16, 126 as u16, 15 as u16, 464 as u16, 594 as u16,
            464 as u16, 204 as u16, 883 as u16, 1050 as u16, 518 as u16,
            263 as u16, 373 as u16, 381 as u16, 92 as u16, 585 as u16,
            1130 as u16, 4 as u16, 203 as u16, 205 as u16, 426 as u16,
            107 as u16, 522 as u16, 25 as u16, 26 as u16, 329 as u16,
            584 as u16, 913 as u16, 572 as u16, 527 as u16, 376 as u16,
            588 as u16, 926 as u16, 530 as u16, 109 as u16, 184 as u16,
            318 as u16, 167 as u16, 110 as u16, 27 as u16, 538 as u16,
            1050 as u16, 1050 as u16, 1052 as u16, 1053 as u16, 35 as u16,
            1211 as u16, 1091 as u16, 17 as u16, 476 as u16, 111 as u16,
            1181 as u16, 234 as u16, 292 as u16, 1180 as u16, 464 as u16,
            294 as u16, 207 as u16, 994 as u16, 129 as u16, 1201 as u16,
            272 as u16, 1000 as u16, 28 as u16, 1197 as u16, 29 as u16,
            30 as u16, 582 as u16, 1199 as u16, 1205 as u16, 1214 as u16,
            31 as u16, 1204 as u16, 32 as u16, 1186 as u16, 41 as u16,
            566 as u16, 33 as u16, 1105 as u16, 211 as u16, 8 as u16,
            115 as u16, 1092 as u16, 1090 as u16, 1094 as u16, 34 as u16,
            278 as u16, 578 as u16, 1095 as u16, 117 as u16, 122 as u16,
            118 as u16, 1145 as u16, 36 as u16, 18 as u16, 128 as u16,
            1062 as u16, 1055 as u16, 895 as u16, 957 as u16, 37 as u16,
            589 as u16, 125 as u16, 125 as u16, 279 as u16, 186 as u16,
            280 as u16, 1646 as u16, 157 as u16, 405 as u16, 126 as u16,
            1220 as u16, 464 as u16, 594 as u16, 464 as u16, 1218 as u16,
            466 as u16, 1050 as u16, 1219 as u16, 300 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 407 as u16, 407 as u16,
            406 as u16, 284 as u16, 404 as u16, 1281 as u16, 1281 as u16,
            881 as u16, 1281 as u16, 300 as u16, 1281 as u16, 1281 as u16,
            571 as u16, 1281 as u16, 407 as u16, 407 as u16, 406 as u16,
            284 as u16, 404 as u16, 1281 as u16, 247 as u16, 881 as u16,
            338 as u16, 1281 as u16, 1281 as u16, 1050 as u16, 1050 as u16,
            1052 as u16, 1053 as u16, 35 as u16, 337 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 247 as u16, 1281 as u16, 338 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 337 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1214 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 249 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 178 as u16, 1281 as u16, 1281 as u16,
            43 as u16, 1281 as u16, 1281 as u16, 249 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 178 as u16, 1281 as u16, 1281 as u16, 43 as u16,
            1281 as u16, 1281 as u16, 248 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 248 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 426 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 329 as u16, 584 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 426 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 329 as u16, 584 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 476 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16, 1281 as u16,
            476 as u16];

static yy_default: [u16; 600] =
    [1691 as u16, 1691 as u16, 1691 as u16, 1516 as u16, 1279 as u16,
            1392 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1516 as u16, 1516 as u16, 1516 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1422 as u16,
            1422 as u16, 1568 as u16, 1312 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1515 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1607 as u16, 1607 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1592 as u16, 1591 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1431 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1438 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1517 as u16, 1518 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1567 as u16, 1569 as u16,
            1533 as u16, 1445 as u16, 1444 as u16, 1443 as u16, 1442 as u16,
            1551 as u16, 1410 as u16, 1436 as u16, 1429 as u16, 1433 as u16,
            1512 as u16, 1513 as u16, 1511 as u16, 1670 as u16, 1518 as u16,
            1517 as u16, 1279 as u16, 1432 as u16, 1480 as u16, 1496 as u16,
            1479 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1488 as u16, 1495 as u16, 1494 as u16, 1493 as u16,
            1502 as u16, 1492 as u16, 1489 as u16, 1482 as u16, 1481 as u16,
            1483 as u16, 1484 as u16, 1303 as u16, 1300 as u16, 1354 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1485 as u16, 1312 as u16, 1473 as u16, 1472 as u16, 1471 as u16,
            1279 as u16, 1499 as u16, 1486 as u16, 1498 as u16, 1497 as u16,
            1575 as u16, 1644 as u16, 1643 as u16, 1534 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1607 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1412 as u16, 1607 as u16,
            1607 as u16, 1279 as u16, 1312 as u16, 1607 as u16, 1607 as u16,
            1308 as u16, 1413 as u16, 1413 as u16, 1308 as u16, 1308 as u16,
            1416 as u16, 1587 as u16, 1383 as u16, 1383 as u16, 1383 as u16,
            1383 as u16, 1392 as u16, 1383 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1572 as u16, 1570 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1388 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1637 as u16, 1683 as u16, 1279 as u16, 1546 as u16,
            1368 as u16, 1388 as u16, 1388 as u16, 1388 as u16, 1388 as u16,
            1390 as u16, 1369 as u16, 1367 as u16, 1382 as u16, 1313 as u16,
            1286 as u16, 1683 as u16, 1683 as u16, 1448 as u16, 1437 as u16,
            1389 as u16, 1437 as u16, 1680 as u16, 1435 as u16, 1448 as u16,
            1448 as u16, 1435 as u16, 1448 as u16, 1389 as u16, 1680 as u16,
            1329 as u16, 1659 as u16, 1324 as u16, 1422 as u16, 1422 as u16,
            1422 as u16, 1412 as u16, 1412 as u16, 1412 as u16, 1412 as u16,
            1416 as u16, 1416 as u16, 1514 as u16, 1389 as u16, 1382 as u16,
            1279 as u16, 1355 as u16, 1683 as u16, 1355 as u16, 1355 as u16,
            1398 as u16, 1398 as u16, 1682 as u16, 1682 as u16, 1398 as u16,
            1534 as u16, 1667 as u16, 1457 as u16, 1357 as u16, 1363 as u16,
            1363 as u16, 1363 as u16, 1363 as u16, 1398 as u16, 1297 as u16,
            1435 as u16, 1667 as u16, 1667 as u16, 1435 as u16, 1457 as u16,
            1357 as u16, 1435 as u16, 1357 as u16, 1435 as u16, 1398 as u16,
            1297 as u16, 1550 as u16, 1678 as u16, 1398 as u16, 1297 as u16,
            1524 as u16, 1398 as u16, 1297 as u16, 1398 as u16, 1297 as u16,
            1524 as u16, 1355 as u16, 1355 as u16, 1355 as u16, 1344 as u16,
            1279 as u16, 1279 as u16, 1524 as u16, 1355 as u16, 1329 as u16,
            1355 as u16, 1344 as u16, 1355 as u16, 1355 as u16, 1625 as u16,
            1279 as u16, 1528 as u16, 1528 as u16, 1524 as u16, 1398 as u16,
            1617 as u16, 1617 as u16, 1425 as u16, 1425 as u16, 1430 as u16,
            1416 as u16, 1519 as u16, 1398 as u16, 1279 as u16, 1430 as u16,
            1428 as u16, 1426 as u16, 1435 as u16, 1347 as u16, 1640 as u16,
            1640 as u16, 1636 as u16, 1636 as u16, 1636 as u16, 1688 as u16,
            1688 as u16, 1587 as u16, 1652 as u16, 1312 as u16, 1312 as u16,
            1312 as u16, 1312 as u16, 1652 as u16, 1331 as u16, 1331 as u16,
            1313 as u16, 1313 as u16, 1312 as u16, 1652 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1647 as u16, 1279 as u16, 1279 as u16, 1535 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1402 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1593 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1462 as u16,
            1279 as u16, 1282 as u16, 1584 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1439 as u16, 1440 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1454 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1449 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1403 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1549 as u16, 1548 as u16, 1279 as u16, 1279 as u16,
            1400 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1327 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1427 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1622 as u16, 1417 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1671 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1377 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16, 1279 as u16,
            1663 as u16, 1371 as u16, 1463 as u16, 1279 as u16, 1466 as u16,
            1301 as u16, 1279 as u16, 1291 as u16, 1279 as u16, 1279 as u16];

///* Find the appropriate action for a parser given the terminal
///* look-ahead token iLookAhead.
#[allow(unused_doc_comments)]
extern "C" fn yy_find_shift_action(mut i_look_ahead_1: u16, stateno: u16)
    -> u16 {
    let mut i: i32 = 0;
    if stateno as i32 > 599 { return stateno; }
    { let _ = 0; };
    '__b2: loop {
        '__c2: loop {
            i = yy_shift_ofst[stateno as usize] as i32;
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            i += i_look_ahead_1 as i32;
            { let _ = 0; };
            if yy_lookahead[i as usize] as i32 != i_look_ahead_1 as i32 {
                let mut i_fallback: u16 = 0 as u16;

                /// Fallback token
                { let _ = 0; };
                i_fallback = yy_fallback[i_look_ahead_1 as usize] as u16;
                if i_fallback as i32 != 0 {
                    { let _ = 0; };

                    /// Fallback loop must terminate
                    (i_look_ahead_1 = i_fallback);
                    break '__c2;
                }
                {
                    let j: i32 = i - i_look_ahead_1 as i32 + 102;
                    { let _ = 0; };
                    if yy_lookahead[j as usize] as i32 == 102 &&
                            i_look_ahead_1 as i32 > 0 {

                        /// NDEBUG
                        return yy_action[j as usize] as u16;
                    }
                }

                /// YYWILDCARD
                return yy_default[stateno as usize] as u16;
            } else { { let _ = 0; }; return yy_action[i as usize] as u16; }
            break '__c2;
        }
    }
}

/// For rule J, yyRuleInfoNRhs[J] contains the negative of the number
///* of symbols on the right-hand side of that rule.
static yy_rule_info_n_rhs: [i8; 412] =
    [-1 as i8, -3 as i8, -1 as i8, -3 as i8, 0 as i8, -1 as i8, -1 as i8,
            -1 as i8, -2 as i8, -2 as i8, -2 as i8, -3 as i8, -5 as i8,
            -6 as i8, -1 as i8, 0 as i8, -3 as i8, -1 as i8, 0 as i8,
            -5 as i8, -2 as i8, 0 as i8, -3 as i8, -2 as i8, -1 as i8,
            -2 as i8, 0 as i8, -4 as i8, -6 as i8, -2 as i8, 0 as i8, 0 as i8,
            -2 as i8, -3 as i8, -4 as i8, -4 as i8, -4 as i8, -3 as i8,
            -3 as i8, -5 as i8, -2 as i8, -4 as i8, -4 as i8, -1 as i8,
            -2 as i8, -3 as i8, -4 as i8, 0 as i8, -1 as i8, 0 as i8,
            -2 as i8, -2 as i8, -3 as i8, -3 as i8, -3 as i8, -2 as i8,
            -2 as i8, -1 as i8, -1 as i8, -2 as i8, -3 as i8, -2 as i8,
            0 as i8, -2 as i8, -2 as i8, 0 as i8, -1 as i8, -2 as i8,
            -7 as i8, -5 as i8, -5 as i8, -10 as i8, 0 as i8, 0 as i8,
            -3 as i8, 0 as i8, -2 as i8, -1 as i8, -1 as i8, -4 as i8,
            -2 as i8, 0 as i8, -9 as i8, -4 as i8, -1 as i8, -3 as i8,
            -4 as i8, -1 as i8, -3 as i8, -1 as i8, -2 as i8, -1 as i8,
            -9 as i8, -10 as i8, -4 as i8, -1 as i8, -5 as i8, -5 as i8,
            -1 as i8, -1 as i8, 0 as i8, 0 as i8, -5 as i8, -3 as i8,
            -5 as i8, -2 as i8, 0 as i8, 0 as i8, -2 as i8, -2 as i8, 0 as i8,
            -5 as i8, -6 as i8, -8 as i8, -6 as i8, -6 as i8, 0 as i8,
            -2 as i8, -1 as i8, -3 as i8, -1 as i8, -3 as i8, -3 as i8,
            -5 as i8, -1 as i8, -2 as i8, -3 as i8, -4 as i8, -2 as i8,
            -4 as i8, 0 as i8, 0 as i8, -3 as i8, -2 as i8, 0 as i8, -3 as i8,
            -5 as i8, -3 as i8, -1 as i8, -1 as i8, 0 as i8, -2 as i8,
            -2 as i8, 0 as i8, 0 as i8, -3 as i8, 0 as i8, -2 as i8, 0 as i8,
            -2 as i8, -4 as i8, -4 as i8, -6 as i8, 0 as i8, -2 as i8,
            0 as i8, -2 as i8, -2 as i8, -4 as i8, -9 as i8, -5 as i8,
            -7 as i8, -3 as i8, -5 as i8, -7 as i8, -8 as i8, 0 as i8,
            -2 as i8, -12 as i8, -9 as i8, -5 as i8, -8 as i8, -2 as i8,
            -2 as i8, -1 as i8, 0 as i8, -3 as i8, -3 as i8, -1 as i8,
            -3 as i8, -1 as i8, -3 as i8, -5 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -3 as i8, -6 as i8, -5 as i8, -8 as i8,
            -4 as i8, -6 as i8, -9 as i8, -5 as i8, -1 as i8, -5 as i8,
            -3 as i8, -3 as i8, -3 as i8, -3 as i8, -3 as i8, -3 as i8,
            -3 as i8, -3 as i8, -2 as i8, -3 as i8, -5 as i8, -2 as i8,
            -3 as i8, -3 as i8, -4 as i8, -6 as i8, -5 as i8, -2 as i8,
            -2 as i8, -2 as i8, -3 as i8, -1 as i8, -2 as i8, -5 as i8,
            -1 as i8, -2 as i8, -5 as i8, -3 as i8, -5 as i8, -5 as i8,
            -4 as i8, -5 as i8, -5 as i8, -4 as i8, -2 as i8, 0 as i8,
            0 as i8, 0 as i8, -3 as i8, -1 as i8, 0 as i8, -3 as i8,
            -12 as i8, -1 as i8, 0 as i8, 0 as i8, -3 as i8, -5 as i8,
            -3 as i8, 0 as i8, -2 as i8, -4 as i8, -2 as i8, -3 as i8,
            -2 as i8, 0 as i8, -3 as i8, -5 as i8, -6 as i8, -5 as i8,
            -6 as i8, -2 as i8, -2 as i8, -5 as i8, -11 as i8, -1 as i8,
            -2 as i8, 0 as i8, -1 as i8, -1 as i8, -3 as i8, 0 as i8,
            -2 as i8, -3 as i8, -2 as i8, -3 as i8, -2 as i8, -9 as i8,
            -8 as i8, -6 as i8, -3 as i8, -4 as i8, -6 as i8, -1 as i8,
            -1 as i8, -1 as i8, -4 as i8, -6 as i8, -3 as i8, 0 as i8,
            -2 as i8, -1 as i8, -3 as i8, -1 as i8, -3 as i8, -6 as i8,
            -2 as i8, -7 as i8, -6 as i8, -8 as i8, -6 as i8, -9 as i8,
            -10 as i8, -11 as i8, -9 as i8, -1 as i8, -4 as i8, -8 as i8,
            0 as i8, -1 as i8, -3 as i8, -1 as i8, -2 as i8, -3 as i8,
            -1 as i8, -2 as i8, -3 as i8, -6 as i8, -1 as i8, -1 as i8,
            -3 as i8, -3 as i8, -5 as i8, -5 as i8, -6 as i8, -4 as i8,
            -5 as i8, -2 as i8, 0 as i8, -3 as i8, -6 as i8, -1 as i8,
            -1 as i8, -2 as i8, -1 as i8, -2 as i8, -2 as i8, -2 as i8,
            0 as i8, -2 as i8, -2 as i8, -2 as i8, -1 as i8, -2 as i8,
            -2 as i8, -1 as i8, -1 as i8, -4 as i8, -2 as i8, -5 as i8,
            -1 as i8, -1 as i8, -2 as i8, -1 as i8, -1 as i8, -2 as i8,
            -3 as i8, 0 as i8, -1 as i8, -2 as i8, -1 as i8, 0 as i8,
            -2 as i8, -1 as i8, -4 as i8, -2 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -2 as i8, 0 as i8,
            -2 as i8, -4 as i8, -2 as i8, -2 as i8, -3 as i8, -1 as i8,
            0 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -2 as i8,
            -1 as i8, -1 as i8, 0 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
            -1 as i8, 0 as i8, -3 as i8, 0 as i8, -1 as i8, 0 as i8, 0 as i8,
            -1 as i8, -1 as i8, -3 as i8, -2 as i8, 0 as i8, -4 as i8,
            -2 as i8, 0 as i8, -1 as i8, -1 as i8];

/// Return an integer that is the maximum allowed stack size
extern "C" fn parser_stack_size_limit(p_parse_1: &Parse) -> i32 {
    return unsafe { (*(*p_parse_1).db).a_limit[12 as usize] };
}

/// Memory allocator for parser stack resizing.  This is a thin wrapper around
///* sqlite3_realloc() that includes a call to sqlite3FaultSim() to facilitate
///* testing.
extern "C" fn parser_stack_realloc(p_old_1: *mut (),
    new_size_1: Sqlite3Uint64, p_parse_1: &Parse) -> *mut () {
    let p: *mut () =
        if unsafe { sqlite3_fault_sim(700) } != 0 {
            core::ptr::null_mut()
        } else { unsafe { sqlite3_realloc(p_old_1, new_size_1 as i32) } };
    if p == core::ptr::null_mut() {
        unsafe { sqlite3_oom_fault((*p_parse_1).db) };
    }
    return p;
}

///* Try to increase the size of the parser stack.  Return the number
///* of errors.  Return 0 on success.
extern "C" fn yy_grow_stack(p: &mut YyParser) -> i32 {
    let old_size: i32 =
        1 +
            unsafe { (*p).yystack_end.offset_from((*p).yystack) } as i64 as
                i32;
    let mut new_size: i32 = 0;
    let mut idx: i32 = 0;
    let mut p_new: *mut YyStackEntry = core::ptr::null_mut();
    let n_limit: i32 = parser_stack_size_limit(unsafe { &*(*p).p_parse });
    new_size = old_size * 2 + 100;
    if new_size > n_limit {
        new_size = n_limit;
        if new_size <= old_size { return 1; }
    }
    idx = unsafe { (*p).yytos.offset_from((*p).yystack) } as i64 as i32;
    if (*p).yystack == &raw mut (*p).yystk0[0 as usize] as *mut YyStackEntry {
        p_new =
            parser_stack_realloc(core::ptr::null_mut(),
                    new_size as u64 *
                        core::mem::size_of::<YyStackEntry>() as u64,
                    unsafe { &*(*p).p_parse }) as *mut YyStackEntry;
        if p_new == core::ptr::null_mut() { return 1; }
        unsafe {
            memcpy(p_new as *mut (), (*p).yystack as *const (),
                old_size as u64 * core::mem::size_of::<YyStackEntry>() as u64)
        };
    } else {
        p_new =
            parser_stack_realloc((*p).yystack as *mut (),
                    new_size as u64 *
                        core::mem::size_of::<YyStackEntry>() as u64,
                    unsafe { &*(*p).p_parse }) as *mut YyStackEntry;
        if p_new == core::ptr::null_mut() { return 1; }
    }
    (*p).yystack = p_new;
    (*p).yytos = unsafe { (*p).yystack.offset(idx as isize) };
    (*p).yystack_end =
        unsafe { (*p).yystack.offset((new_size - 1) as isize) };
    return 0;
}

///* Pop the parser's stack once.
///*
///* If there is a destructor routine associated with the token which
///* is popped from the stack, then call it.
extern "C" fn yy_pop_parser_stack(p_parser_1: *mut YyParser) -> () {
    let mut yytos: *mut YyStackEntry = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    yytos =
        {
            let __p = unsafe { &mut (*p_parser_1).yytos };
            let __t = *__p;
            *__p = unsafe { (*__p).offset(-1) };
            __t
        };
    yy_destructor(unsafe { &*p_parser_1 }, unsafe { (*yytos).major },
        unsafe { &(*yytos).minor });
}

///* The following routine is called if the stack overflows.
#[allow(unused_doc_comments)]
extern "C" fn yy_stack_overflow(yyp_parser_1: *mut YyParser) -> () {
    let p_parse: *mut Parse = unsafe { (*yyp_parser_1).p_parse };
    while unsafe { (*yyp_parser_1).yytos } >
            unsafe { (*yyp_parser_1).yystack } {
        yy_pop_parser_stack(yyp_parser_1);
    }
    if unsafe { (*p_parse).n_err } == 0 {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"Recursion limit".as_ptr() as *mut i8 as *const i8)
        };
    }

    ///***** End %stack_overflow code *******************************************
    /// Suppress warning about unused %extra_argument var
    unsafe { (*yyp_parser_1).p_parse = p_parse };
}

///* Disable lookaside memory allocation for objects that might be
///* shared across database connections.
extern "C" fn disable_lookaside(p_parse_1: &mut Parse) -> () {
    unsafe {
        let db: *mut Sqlite3 = (*p_parse_1).db;
        {
            let __p = &mut (*p_parse_1).disable_lookaside;
            let __t = *__p;
            *__p += 1;
            __t
        };
        unsafe { memset(&raw mut (*p_parse_1).u1.cr as *mut (), 0, 32) };
        {
            let __p = unsafe { &mut (*db).lookaside.b_disable };
            let __t = *__p;
            *__p += 1;
            __t
        };
        unsafe { (*db).lookaside.sz = 0 as u16 };
    }
}

/// Construct a new Expr object from a single token
#[allow(unused_doc_comments)]
extern "C" fn token_expr(p_parse_1: *mut Parse, op: i32, mut t: Token)
    -> *mut Expr {
    unsafe {
        unsafe {
            let p: *mut Expr =
                unsafe {
                        sqlite3_db_malloc_raw_nn(unsafe { (*p_parse_1).db },
                            core::mem::size_of::<Expr>() as u64 + t.n as u64 + 1 as u64)
                    } as *mut Expr;
            if !(p).is_null() {

                /// memset(p, 0, sizeof(Expr));
                unsafe { (*p).op = op as u8 };

                /// memset(p, 0, sizeof(Expr));
                unsafe { (*p).aff_expr = 0 as i8 };
                unsafe { (*p).flags = 8388608 as u32 };

                /// p->iAgg = -1; // Not required
                unsafe {
                    (*p).p_left =
                        {
                            unsafe { (*p).p_right = core::ptr::null_mut() };
                            unsafe { (*p).p_right }
                        }
                };
                unsafe { (*p).p_agg_info = core::ptr::null_mut() };
                unsafe {
                    memset(unsafe { &raw mut (*p).x } as *mut (), 0, 8)
                };
                unsafe {
                    memset(unsafe { &raw mut (*p).y } as *mut (), 0, 8)
                };
                unsafe { (*p).op2 = 0 as u8 };
                unsafe { (*p).i_table = 0 };
                unsafe { (*p).i_column = 0 as YnVar };
                unsafe {
                    (*p).u.z_token =
                        unsafe { &raw mut *p.offset(1 as isize) } as *mut i8
                };
                unsafe {
                    memcpy(unsafe { (*p).u.z_token } as *mut (),
                        t.z as *const (), t.n as u64)
                };
                unsafe {
                    *unsafe { (*p).u.z_token.add(t.n as usize) } = 0 as i8
                };
                unsafe {
                    (*p).w.i_ofst =
                        unsafe { t.z.offset_from(unsafe { (*p_parse_1).z_tail }) }
                                as i64 as i32
                };
                if unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe {
                                                        *unsafe { (*p).u.z_token.offset(0 as isize) }
                                                    } as u8 as usize)
                                } as i32 & 128 != 0 {
                    unsafe { sqlite3_dequote_expr(p) };
                }
                unsafe { (*p).n_height = 1 };
                if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                    return unsafe {
                                sqlite3_rename_token_map(p_parse_1,
                                    p as *mut () as *const (), &raw mut t as *const Token)
                            } as *mut Expr;
                }
            }
            return p;
        }
    }
}

///* For a compound SELECT statement, make sure p->pPrior->pNext==p for
///* all elements in the list.  And make sure list length does not exceed
///* SQLITE_LIMIT_COMPOUND_SELECT.
extern "C" fn parser_double_link_select(p_parse_1: *mut Parse, p: *mut Select)
    -> () {
    { let _ = 0; };
    if !(unsafe { (*p).p_prior }).is_null() {
        let mut p_next: *mut Select = core::ptr::null_mut();
        let mut p_loop: *mut Select = p;
        let mut mx_select: i32 = 0;
        let mut cnt: i32 = 1;
        loop {
            unsafe { (*p_loop).p_next = p_next };
            unsafe { (*p_loop).sel_flags |= 256 as u32 };
            p_next = p_loop;
            p_loop = unsafe { (*p_loop).p_prior };
            if p_loop == core::ptr::null_mut() { break; }
            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
            if !(unsafe { (*p_loop).p_order_by }).is_null() ||
                    !(unsafe { (*p_loop).p_limit }).is_null() {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"%s clause should come after %s not before".as_ptr() as
                                *mut i8 as *const i8,
                        if unsafe { (*p_loop).p_order_by } != core::ptr::null_mut()
                            {
                            c"ORDER BY".as_ptr() as *mut i8
                        } else { c"LIMIT".as_ptr() as *mut i8 },
                        unsafe {
                            sqlite3_select_op_name(unsafe { (*p_next).op } as i32)
                        })
                };
                break;
            }
        }
        if unsafe { (*p).sel_flags } & (1024 | 512) as u32 == 0 as u32 &&
                    {
                            mx_select =
                                unsafe {
                                    (*unsafe { (*p_parse_1).db }).a_limit[4 as usize]
                                };
                            mx_select
                        } > 0 && cnt > mx_select {
            unsafe {
                sqlite3_error_msg(p_parse_1,
                    c"too many terms in compound SELECT".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
    }
}

/// Attach a With object describing the WITH clause to a Select
///* object describing the query for which the WITH clause is a prefix.
extern "C" fn attach_with_to_select(p_parse_1: *mut Parse,
    p_select_1: *mut Select, p_with_1: *mut With) -> *mut Select {
    if !(p_select_1).is_null() {
        unsafe { (*p_select_1).p_with = p_with_1 };
        parser_double_link_select(p_parse_1, p_select_1);
    } else {
        unsafe { sqlite3_with_delete(unsafe { (*p_parse_1).db }, p_with_1) };
    }
    return p_select_1;
}

///* Generate a syntax error
extern "C" fn parser_syntax_error(p_parse_1: *mut Parse, p: *mut Token)
    -> () {
    unsafe {
        sqlite3_error_msg(p_parse_1,
            c"near \"%T\": syntax error".as_ptr() as *mut i8 as *const i8, p)
    };
}

/// Create a TK_ISNULL or TK_NOTNULL expression, perhaps optimized to
///* to TK_TRUEFALSE, if possible
extern "C" fn sqlite3_p_expr_is_null(p_parse_1: *mut Parse, op: i32,
    p_left_1: *mut Expr) -> *mut Expr {
    let mut p: *const Expr = p_left_1 as *const Expr;
    { let _ = 0; };
    { let _ = 0; };
    while unsafe { (*p).op } as i32 == 173 || unsafe { (*p).op } as i32 == 174
        {
        p = unsafe { (*p).p_left };
        { let _ = 0; };
    }
    '__s6:
        {
        match unsafe { (*p).op } {
            156 => {
                unsafe { sqlite3_expr_deferred_delete(p_parse_1, p_left_1) };
                return unsafe {
                        sqlite3_expr_int32(unsafe { (*p_parse_1).db },
                            (op == 52) as i32)
                    };
            }
            118 => {
                unsafe { sqlite3_expr_deferred_delete(p_parse_1, p_left_1) };
                return unsafe {
                        sqlite3_expr_int32(unsafe { (*p_parse_1).db },
                            (op == 52) as i32)
                    };
            }
            154 => {
                unsafe { sqlite3_expr_deferred_delete(p_parse_1, p_left_1) };
                return unsafe {
                        sqlite3_expr_int32(unsafe { (*p_parse_1).db },
                            (op == 52) as i32)
                    };
            }
            155 => {
                unsafe { sqlite3_expr_deferred_delete(p_parse_1, p_left_1) };
                return unsafe {
                        sqlite3_expr_int32(unsafe { (*p_parse_1).db },
                            (op == 52) as i32)
                    };
            }
            _ => {}
        }
    }
    return unsafe {
            sqlite3_p_expr(p_parse_1, op, p_left_1, core::ptr::null_mut())
        };
}

/// Create a TK_IS or TK_ISNOT operator, perhaps optimized to
///* TK_ISNULL or TK_NOTNULL or TK_TRUEFALSE.
extern "C" fn sqlite3_p_expr_is(p_parse_1: *mut Parse, op: i32,
    p_left_1: *mut Expr, p_right_1: *mut Expr) -> *mut Expr {
    if !(p_right_1).is_null() && unsafe { (*p_right_1).op } as i32 == 122 {
        unsafe { sqlite3_expr_deferred_delete(p_parse_1, p_right_1) };
        return sqlite3_p_expr_is_null(p_parse_1,
                if op == 45 { 51 } else { 52 }, p_left_1);
    }
    return unsafe { sqlite3_p_expr(p_parse_1, op, p_left_1, p_right_1) };
}

/// Add a single new term to an ExprList that is used to store a
///* list of identifiers.  Report an error if the ID list contains
///* a COLLATE clause or an ASC or DESC keyword, except ignore the
///* error while parsing a legacy schema.
extern "C" fn parser_add_expr_id_list_term(p_parse_1: *mut Parse,
    p_prior_1: *mut ExprList, p_id_token_1: *const Token, has_collate_1: i32,
    sort_order_1: i32) -> *mut ExprList {
    unsafe {
        let p: *mut ExprList =
            unsafe {
                sqlite3_expr_list_append(p_parse_1, p_prior_1,
                    core::ptr::null_mut())
            };
        if (has_collate_1 != 0 || sort_order_1 != -1) &&
                unsafe { (*unsafe { (*p_parse_1).db }).init.busy } as i32 == 0
            {
            unsafe {
                sqlite3_error_msg(p_parse_1,
                    c"syntax error after column name \"%.*s\"".as_ptr() as
                            *mut i8 as *const i8, unsafe { (*p_id_token_1).n },
                    unsafe { (*p_id_token_1).z })
            };
        }
        unsafe {
            sqlite3_expr_list_set_name(p_parse_1, p,
                p_id_token_1 as *const Token, 1)
        };
        return p;
    }
}

/// For rule J, yyRuleInfoLhs[J] contains the symbol on the left-hand side
///* of that rule
static yy_rule_info_lhs: [u16; 412] =
    [191 as u16, 191 as u16, 190 as u16, 192 as u16, 193 as u16, 193 as u16,
            193 as u16, 193 as u16, 192 as u16, 192 as u16, 192 as u16,
            192 as u16, 192 as u16, 197 as u16, 199 as u16, 201 as u16,
            201 as u16, 200 as u16, 200 as u16, 198 as u16, 198 as u16,
            205 as u16, 205 as u16, 207 as u16, 207 as u16, 208 as u16,
            210 as u16, 210 as u16, 210 as u16, 211 as u16, 215 as u16,
            216 as u16, 217 as u16, 217 as u16, 217 as u16, 217 as u16,
            217 as u16, 217 as u16, 217 as u16, 217 as u16, 217 as u16,
            217 as u16, 217 as u16, 217 as u16, 217 as u16, 226 as u16,
            226 as u16, 222 as u16, 222 as u16, 224 as u16, 224 as u16,
            227 as u16, 227 as u16, 227 as u16, 227 as u16, 228 as u16,
            228 as u16, 228 as u16, 228 as u16, 228 as u16, 225 as u16,
            225 as u16, 229 as u16, 229 as u16, 229 as u16, 204 as u16,
            231 as u16, 232 as u16, 232 as u16, 232 as u16, 232 as u16,
            232 as u16, 235 as u16, 220 as u16, 220 as u16, 236 as u16,
            236 as u16, 237 as u16, 237 as u16, 192 as u16, 239 as u16,
            239 as u16, 192 as u16, 192 as u16, 192 as u16, 206 as u16,
            206 as u16, 206 as u16, 241 as u16, 244 as u16, 244 as u16,
            244 as u16, 242 as u16, 242 as u16, 254 as u16, 242 as u16,
            256 as u16, 256 as u16, 245 as u16, 245 as u16, 245 as u16,
            257 as u16, 246 as u16, 246 as u16, 246 as u16, 258 as u16,
            258 as u16, 247 as u16, 247 as u16, 260 as u16, 260 as u16,
            259 as u16, 259 as u16, 259 as u16, 259 as u16, 259 as u16,
            202 as u16, 202 as u16, 240 as u16, 240 as u16, 265 as u16,
            265 as u16, 265 as u16, 265 as u16, 261 as u16, 261 as u16,
            261 as u16, 261 as u16, 262 as u16, 262 as u16, 262 as u16,
            267 as u16, 263 as u16, 263 as u16, 251 as u16, 251 as u16,
            233 as u16, 233 as u16, 221 as u16, 221 as u16, 221 as u16,
            268 as u16, 268 as u16, 268 as u16, 249 as u16, 249 as u16,
            250 as u16, 250 as u16, 252 as u16, 252 as u16, 252 as u16,
            252 as u16, 192 as u16, 248 as u16, 248 as u16, 270 as u16,
            270 as u16, 270 as u16, 270 as u16, 192 as u16, 271 as u16,
            271 as u16, 271 as u16, 271 as u16, 192 as u16, 192 as u16,
            274 as u16, 274 as u16, 274 as u16, 274 as u16, 274 as u16,
            274 as u16, 275 as u16, 272 as u16, 272 as u16, 273 as u16,
            273 as u16, 266 as u16, 266 as u16, 219 as u16, 219 as u16,
            219 as u16, 219 as u16, 218 as u16, 218 as u16, 218 as u16,
            219 as u16, 219 as u16, 219 as u16, 219 as u16, 219 as u16,
            219 as u16, 219 as u16, 219 as u16, 219 as u16, 218 as u16,
            219 as u16, 219 as u16, 219 as u16, 219 as u16, 219 as u16,
            219 as u16, 219 as u16, 219 as u16, 219 as u16, 277 as u16,
            219 as u16, 219 as u16, 219 as u16, 219 as u16, 219 as u16,
            219 as u16, 219 as u16, 219 as u16, 219 as u16, 219 as u16,
            219 as u16, 219 as u16, 278 as u16, 278 as u16, 219 as u16,
            279 as u16, 279 as u16, 219 as u16, 219 as u16, 219 as u16,
            219 as u16, 219 as u16, 219 as u16, 282 as u16, 282 as u16,
            283 as u16, 283 as u16, 281 as u16, 264 as u16, 255 as u16,
            255 as u16, 280 as u16, 280 as u16, 192 as u16, 284 as u16,
            284 as u16, 223 as u16, 223 as u16, 234 as u16, 234 as u16,
            285 as u16, 285 as u16, 192 as u16, 192 as u16, 192 as u16,
            286 as u16, 286 as u16, 192 as u16, 192 as u16, 192 as u16,
            192 as u16, 192 as u16, 213 as u16, 214 as u16, 192 as u16,
            288 as u16, 290 as u16, 290 as u16, 290 as u16, 291 as u16,
            291 as u16, 291 as u16, 293 as u16, 293 as u16, 289 as u16,
            289 as u16, 295 as u16, 295 as u16, 294 as u16, 294 as u16,
            294 as u16, 294 as u16, 219 as u16, 219 as u16, 238 as u16,
            238 as u16, 238 as u16, 192 as u16, 192 as u16, 192 as u16,
            297 as u16, 297 as u16, 192 as u16, 192 as u16, 192 as u16,
            192 as u16, 192 as u16, 192 as u16, 298 as u16, 192 as u16,
            192 as u16, 192 as u16, 192 as u16, 192 as u16, 192 as u16,
            192 as u16, 192 as u16, 192 as u16, 300 as u16, 302 as u16,
            303 as u16, 303 as u16, 304 as u16, 269 as u16, 269 as u16,
            307 as u16, 307 as u16, 307 as u16, 306 as u16, 308 as u16,
            243 as u16, 243 as u16, 309 as u16, 310 as u16, 311 as u16,
            311 as u16, 311 as u16, 311 as u16, 311 as u16, 312 as u16,
            312 as u16, 312 as u16, 316 as u16, 318 as u16, 318 as u16,
            319 as u16, 319 as u16, 317 as u16, 317 as u16, 320 as u16,
            320 as u16, 321 as u16, 321 as u16, 321 as u16, 253 as u16,
            276 as u16, 276 as u16, 276 as u16, 315 as u16, 315 as u16,
            314 as u16, 218 as u16, 187 as u16, 188 as u16, 188 as u16,
            189 as u16, 189 as u16, 189 as u16, 194 as u16, 194 as u16,
            194 as u16, 196 as u16, 196 as u16, 192 as u16, 205 as u16,
            203 as u16, 203 as u16, 195 as u16, 195 as u16, 210 as u16,
            211 as u16, 212 as u16, 212 as u16, 209 as u16, 209 as u16,
            217 as u16, 217 as u16, 217 as u16, 204 as u16, 230 as u16,
            230 as u16, 231 as u16, 235 as u16, 237 as u16, 241 as u16,
            242 as u16, 257 as u16, 258 as u16, 267 as u16, 275 as u16,
            219 as u16, 277 as u16, 281 as u16, 264 as u16, 287 as u16,
            287 as u16, 287 as u16, 287 as u16, 287 as u16, 213 as u16,
            292 as u16, 292 as u16, 295 as u16, 296 as u16, 296 as u16,
            299 as u16, 299 as u16, 301 as u16, 301 as u16, 302 as u16,
            305 as u16, 305 as u16, 305 as u16, 269 as u16, 309 as u16,
            311 as u16];

static yy_reduce_ofst: [i16; 424] =
    [-67 as i16, 345 as i16, -64 as i16, -178 as i16, -181 as i16, 143 as i16,
            435 as i16, -78 as i16, -183 as i16, 163 as i16, -185 as i16,
            284 as i16, 384 as i16, -174 as i16, 189 as i16, 352 as i16,
            440 as i16, 444 as i16, 493 as i16, -23 as i16, 227 as i16,
            -277 as i16, -1 as i16, 305 as i16, 561 as i16, 755 as i16,
            759 as i16, 764 as i16, -189 as i16, 839 as i16, 857 as i16,
            354 as i16, 484 as i16, 859 as i16, 631 as i16, 67 as i16,
            734 as i16, 780 as i16, -187 as i16, 616 as i16, 581 as i16,
            730 as i16, 891 as i16, 449 as i16, 588 as i16, 795 as i16,
            836 as i16, -238 as i16, 287 as i16, -238 as i16, 287 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            205 as i16, 582 as i16, 715 as i16, 958 as i16, 985 as i16,
            1003 as i16, 1005 as i16, 1010 as i16, 1012 as i16, 1059 as i16,
            1066 as i16, 1092 as i16, 1094 as i16, 1097 as i16, 1122 as i16,
            1137 as i16, 1141 as i16, 1143 as i16, 1147 as i16, 1151 as i16,
            1172 as i16, 1249 as i16, 1251 as i16, 1269 as i16, 1271 as i16,
            1276 as i16, 1290 as i16, 1316 as i16, 1318 as i16, 1337 as i16,
            1371 as i16, 1373 as i16, 1375 as i16, 1400 as i16, 1413 as i16,
            1418 as i16, 1420 as i16, 1422 as i16, 1425 as i16, 1427 as i16,
            1433 as i16, 1438 as i16, 1447 as i16, 1454 as i16, 1459 as i16,
            1463 as i16, 1467 as i16, 1480 as i16, 1484 as i16, 1518 as i16,
            1523 as i16, 1525 as i16, 1527 as i16, 1529 as i16, 1531 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, -256 as i16,
            -256 as i16, 155 as i16, 210 as i16, -220 as i16, 86 as i16,
            -130 as i16, 943 as i16, 996 as i16, 402 as i16, -256 as i16,
            -113 as i16, 981 as i16, 1095 as i16, 1135 as i16, 395 as i16,
            -256 as i16, -256 as i16, -256 as i16, -256 as i16, 568 as i16,
            568 as i16, 568 as i16, -4 as i16, -153 as i16, -133 as i16,
            259 as i16, 306 as i16, -166 as i16, 523 as i16, -303 as i16,
            -126 as i16, 503 as i16, 503 as i16, -37 as i16, -149 as i16,
            164 as i16, 690 as i16, 292 as i16, 412 as i16, 492 as i16,
            651 as i16, 784 as i16, 332 as i16, 786 as i16, 841 as i16,
            1149 as i16, 833 as i16, 1236 as i16, 792 as i16, 162 as i16,
            796 as i16, 1253 as i16, 777 as i16, 288 as i16, 381 as i16,
            380 as i16, 709 as i16, 487 as i16, 1027 as i16, 972 as i16,
            1030 as i16, 1084 as i16, 991 as i16, 1120 as i16, -152 as i16,
            1062 as i16, 692 as i16, 1240 as i16, 1247 as i16, 1250 as i16,
            1239 as i16, 1306 as i16, -207 as i16, -194 as i16, 57 as i16,
            180 as i16, 74 as i16, 315 as i16, 355 as i16, 376 as i16,
            452 as i16, 488 as i16, 630 as i16, 693 as i16, 965 as i16,
            1004 as i16, 1025 as i16, 1099 as i16, 1154 as i16, 1289 as i16,
            1305 as i16, 1310 as i16, 1469 as i16, 1489 as i16, 984 as i16,
            1494 as i16, 1502 as i16, 1516 as i16, 1544 as i16, 1556 as i16,
            1557 as i16, 1562 as i16, 1576 as i16, 1578 as i16, 1579 as i16,
            1583 as i16, 1584 as i16, 1585 as i16, 1586 as i16, 1217 as i16,
            1440 as i16, 1554 as i16, 1589 as i16, 1593 as i16, 1594 as i16,
            1530 as i16, 1597 as i16, 1599 as i16, 1600 as i16, 1539 as i16,
            1472 as i16, 1601 as i16, 1560 as i16, 1604 as i16, 355 as i16,
            1605 as i16, 1609 as i16, 1610 as i16, 1612 as i16, 1613 as i16,
            1614 as i16, 1503 as i16, 1512 as i16, 1520 as i16, 1567 as i16,
            1548 as i16, 1558 as i16, 1559 as i16, 1561 as i16, 1530 as i16,
            1567 as i16, 1567 as i16, 1569 as i16, 1596 as i16, 1622 as i16,
            1519 as i16, 1521 as i16, 1547 as i16, 1565 as i16, 1581 as i16,
            1568 as i16, 1534 as i16, 1582 as i16, 1563 as i16, 1566 as i16,
            1591 as i16, 1570 as i16, 1606 as i16, 1536 as i16, 1619 as i16,
            1615 as i16, 1616 as i16, 1624 as i16, 1626 as i16, 1633 as i16,
            1590 as i16, 1595 as i16, 1603 as i16, 1617 as i16, 1618 as i16,
            1621 as i16, 1572 as i16, 1623 as i16, 1628 as i16, 1663 as i16,
            1644 as i16, 1577 as i16, 1647 as i16, 1648 as i16, 1673 as i16,
            1675 as i16, 1588 as i16, 1627 as i16, 1678 as i16, 1630 as i16,
            1629 as i16, 1634 as i16, 1651 as i16, 1650 as i16, 1652 as i16,
            1653 as i16, 1654 as i16, 1688 as i16, 1701 as i16, 1655 as i16,
            1635 as i16, 1636 as i16, 1658 as i16, 1639 as i16, 1672 as i16,
            1661 as i16, 1677 as i16, 1666 as i16, 1715 as i16, 1717 as i16,
            1632 as i16, 1642 as i16, 1724 as i16, 1726 as i16, 1712 as i16,
            1728 as i16, 1736 as i16, 1737 as i16, 1739 as i16, 1718 as i16,
            1722 as i16, 1723 as i16, 1725 as i16, 1719 as i16, 1720 as i16,
            1721 as i16, 1727 as i16, 1731 as i16, 1734 as i16, 1735 as i16,
            1738 as i16, 1740 as i16, 1742 as i16, 1643 as i16, 1656 as i16,
            1669 as i16, 1674 as i16, 1753 as i16, 1759 as i16, 1645 as i16,
            1646 as i16, 1711 as i16, 1713 as i16, 1748 as i16, 1744 as i16,
            1709 as i16, 1789 as i16, 1716 as i16, 1749 as i16, 1752 as i16,
            1755 as i16, 1758 as i16, 1807 as i16, 1816 as i16, 1818 as i16,
            1823 as i16, 1824 as i16, 1825 as i16, 1745 as i16, 1754 as i16,
            1714 as i16, 1811 as i16, 1806 as i16, 1808 as i16, 1809 as i16,
            1810 as i16, 1813 as i16, 1802 as i16, 1803 as i16, 1812 as i16,
            1815 as i16, 1817 as i16, 1820 as i16];

///* Find the appropriate action for a parser given the non-terminal
///* look-ahead token iLookAhead.
extern "C" fn yy_find_reduce_action(stateno: u16, i_look_ahead_1: u16)
    -> u16 {
    let mut i: i32 = 0;
    { let _ = 0; };
    i = yy_reduce_ofst[stateno as usize] as i32;
    { let _ = 0; };
    i += i_look_ahead_1 as i32;
    { let _ = 0; };
    { let _ = 0; };
    return yy_action[i as usize] as u16;
}

///* Perform a reduce action and the shift that must immediately
///* follow the reduce.
///*
///* The yyLookahead and yyLookaheadToken parameters provide reduce actions
///* access to the lookahead token (if any).  The yyLookahead will be YYNOCODE
///* if the lookahead token has already been consumed.  As this procedure is
///* only called from one place, optimizing compilers will in-line it, which
///* means that the extra parameters have no performance impact.
#[allow(unused_doc_comments)]
extern "C" fn yy_reduce(yyp_parser_1: &mut YyParser, yyruleno: u32,
    yy_lookahead_1: i32, yy_lookahead_token_1: Token, p_parse_1: *mut Parse)
    -> u16 {
    unsafe {
        unsafe {
            let mut yygoto: i32 = 0;
            /// The next state
            let mut yyact: u16 = 0 as u16;
            /// The next action
            let mut yymsp: *mut YyStackEntry = core::ptr::null_mut();
            /// The top of the parser's stack
            let mut yysize: i32 = 0;

            /// Amount to pop the stack
            { let _ = yy_lookahead_1; };

            /// Amount to pop the stack
            { let _ = yy_lookahead_token_1; };

            /// Amount to pop the stack
            (yymsp = (*yyp_parser_1).yytos);
            /// Beginning here are the reduction cases.  A typical example
            ///* follows:
            ///*   case 0:
            ///*  #line <lineno> <grammarfile>
            ///*     { ... }           // User supplied code
            ///*  #line <lineno> <thisfile>
            ///*     break;
            ////
            ////********** Begin reduce actions *********************************************
            let mut yylhsminor: YYMINORTYPE = unsafe { core::mem::zeroed() };
            '__s7:
                {
                match yyruleno {
                    0 => {
                        {
                            if unsafe { (*p_parse_1).p_reprepare } ==
                                    core::ptr::null_mut() {
                                unsafe { (*p_parse_1).explain = 1 as u8 };
                            }
                        }
                    }
                    1 => {
                        {
                            if unsafe { (*p_parse_1).p_reprepare } ==
                                    core::ptr::null_mut() {
                                unsafe { (*p_parse_1).explain = 2 as u8 };
                            }
                        }
                    }
                    2 => { { unsafe { sqlite3_finish_coding(p_parse_1) }; } }
                    3 => {
                        {
                            unsafe {
                                sqlite3_begin_transaction(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 })
                            };
                        }
                    }
                    4 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 7 }; }
                    }
                    5 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    6 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    7 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    328 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    8 => {
                        {
                            unsafe {
                                sqlite3_end_transaction(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).major } as i32)
                            };
                        }
                    }
                    9 => {
                        {
                            unsafe {
                                sqlite3_end_transaction(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).major } as i32)
                            };
                        }
                    }
                    10 => {
                        {
                            unsafe {
                                sqlite3_savepoint(p_parse_1, 0,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    11 => {
                        {
                            unsafe {
                                sqlite3_savepoint(p_parse_1, 1,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    12 => {
                        {
                            unsafe {
                                sqlite3_savepoint(p_parse_1, 2,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    13 => {
                        {
                            unsafe {
                                sqlite3_start_table(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy144 }, 0, 0,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 })
                            };
                        }
                    }
                    14 => { { disable_lookaside(unsafe { &mut *p_parse_1 }); } }
                    15 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    18 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    47 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    62 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    72 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    81 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    100 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    246 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    16 => {
                        { unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 = 1 }; }
                    }
                    17 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    (unsafe { (*unsafe { (*p_parse_1).db }).init.busy } as i32
                                            == 0) as i32
                            };
                        }
                    }
                    19 => {
                        {
                            unsafe {
                                sqlite3_end_table(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy391 },
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    20 => {
                        {
                            unsafe {
                                sqlite3_end_table(p_parse_1, core::ptr::null_mut(),
                                    core::ptr::null_mut(), 0 as u32,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy555 })
                            };
                            unsafe {
                                sqlite3_select_delete(unsafe { (*p_parse_1).db },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy555 })
                            };
                        }
                    }
                    21 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy391 = 0 as u32
                            };
                        }
                    }
                    22 => {
                        {
                            yylhsminor.yy391 =
                                unsafe { (*yymsp.offset(-2 as isize)).minor.yy391 } |
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy391 };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy391 = yylhsminor.yy391
                        };
                    }
                    23 => {
                        {
                            if unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n } ==
                                        5 as u32 &&
                                    unsafe {
                                            sqlite3_strnicmp(unsafe {
                                                    (*yymsp.offset(0 as isize)).minor.yy0.z
                                                }, c"rowid".as_ptr() as *mut i8 as *const i8, 5)
                                        } == 0 {
                                unsafe {
                                    (*yymsp.offset(-1 as isize)).minor.yy391 =
                                        (128 | 512) as u32
                                };
                            } else {
                                unsafe {
                                    (*yymsp.offset(-1 as isize)).minor.yy391 = 0 as u32
                                };
                                unsafe {
                                    sqlite3_error_msg(p_parse_1,
                                        c"unknown table option: %.*s".as_ptr() as *mut i8 as
                                            *const i8,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0.z })
                                };
                            }
                        }
                    }
                    24 => {
                        {
                            if unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n } ==
                                        6 as u32 &&
                                    unsafe {
                                            sqlite3_strnicmp(unsafe {
                                                    (*yymsp.offset(0 as isize)).minor.yy0.z
                                                }, c"strict".as_ptr() as *mut i8 as *const i8, 6)
                                        } == 0 {
                                yylhsminor.yy391 = 65536 as u32;
                            } else {
                                yylhsminor.yy391 = 0 as u32;
                                unsafe {
                                    sqlite3_error_msg(p_parse_1,
                                        c"unknown table option: %.*s".as_ptr() as *mut i8 as
                                            *const i8,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0.z })
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy391 = yylhsminor.yy391
                        };
                    }
                    25 => {
                        {
                            unsafe {
                                sqlite3_add_column(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    26 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as u32
                            };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.z = core::ptr::null()
                            };
                        }
                    }
                    65 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as u32
                            };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.z = core::ptr::null()
                            };
                        }
                    }
                    106 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as u32
                            };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.z = core::ptr::null()
                            };
                        }
                    }
                    27 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy0.n =
                                    unsafe {
                                                    unsafe {
                                                        unsafe {
                                                            (*yymsp.offset(0 as
                                                                                            isize)).minor.yy0.z.add(unsafe {
                                                                            (*yymsp.offset(0 as isize)).minor.yy0.n
                                                                        } as
                                                                        usize).offset_from(unsafe {
                                                                    (*yymsp.offset(-3 as isize)).minor.yy0.z
                                                                })
                                                        }
                                                    }
                                                } as i64 as i32 as u32
                            };
                        }
                    }
                    28 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy0.n =
                                    unsafe {
                                                    unsafe {
                                                        unsafe {
                                                            (*yymsp.offset(0 as
                                                                                            isize)).minor.yy0.z.add(unsafe {
                                                                            (*yymsp.offset(0 as isize)).minor.yy0.n
                                                                        } as
                                                                        usize).offset_from(unsafe {
                                                                    (*yymsp.offset(-5 as isize)).minor.yy0.z
                                                                })
                                                        }
                                                    }
                                                } as i64 as i32 as u32
                            };
                        }
                    }
                    29 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0.n =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n } +
                                        unsafe {
                                                        unsafe {
                                                            (*yymsp.offset(0 as
                                                                                        isize)).minor.yy0.z.offset_from(unsafe {
                                                                    (*yymsp.offset(-1 as isize)).minor.yy0.z
                                                                })
                                                        }
                                                    } as i64 as i32 as u32
                            };
                        }
                    }
                    30 => {
                        {
                            { let _ = 0; };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy168 =
                                    yy_lookahead_token_1.z
                            };
                        }
                    }
                    31 => {
                        {
                            { let _ = 0; };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0 = yy_lookahead_token_1
                            };
                        }
                    }
                    32 => {
                        {
                            { let _ = 0; };
                            unsafe {
                                (*p_parse_1).u1.cr.constraint_name =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    67 => {
                        {
                            { let _ = 0; };
                            unsafe {
                                (*p_parse_1).u1.cr.constraint_name =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    33 => {
                        {
                            unsafe {
                                sqlite3_add_default_value(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy0.z },
                                    unsafe {
                                        &*unsafe {
                                                    (*yymsp.offset(-1 as
                                                                                isize)).minor.yy0.z.add(unsafe {
                                                                (*yymsp.offset(-1 as isize)).minor.yy0.n
                                                            } as usize)
                                                }
                                    })
                            };
                        }
                    }
                    34 => {
                        {
                            unsafe {
                                sqlite3_add_default_value(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                    unsafe {
                                        unsafe {
                                            (*yymsp.offset(-2 as isize)).minor.yy0.z.offset(1 as isize)
                                        }
                                    }, unsafe { (*yymsp.offset(0 as isize)).minor.yy0.z })
                            };
                        }
                    }
                    35 => {
                        {
                            unsafe {
                                sqlite3_add_default_value(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy0.z },
                                    unsafe {
                                        &*unsafe {
                                                    (*yymsp.offset(-1 as
                                                                                isize)).minor.yy0.z.add(unsafe {
                                                                (*yymsp.offset(-1 as isize)).minor.yy0.n
                                                            } as usize)
                                                }
                                    })
                            };
                        }
                    }
                    36 => {
                        {
                            let mut p: *mut Expr =
                                unsafe {
                                    sqlite3_p_expr(p_parse_1, 174,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                        core::ptr::null_mut())
                                };
                            unsafe {
                                sqlite3_add_default_value(p_parse_1, p,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy0.z },
                                    unsafe {
                                        &*unsafe {
                                                    (*yymsp.offset(-1 as
                                                                                isize)).minor.yy0.z.add(unsafe {
                                                                (*yymsp.offset(-1 as isize)).minor.yy0.n
                                                            } as usize)
                                                }
                                    })
                            };
                        }
                    }
                    37 => {
                        {
                            let mut p: *mut Expr =
                                token_expr(p_parse_1, 118,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 });
                            if !(p).is_null() {
                                unsafe { sqlite3_expr_id_to_true_false(p) };
                            }
                            unsafe {
                                sqlite3_add_default_value(p_parse_1, p,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0.z },
                                    unsafe {
                                        unsafe {
                                            (*yymsp.offset(0 as
                                                                        isize)).minor.yy0.z.add(unsafe {
                                                        (*yymsp.offset(0 as isize)).minor.yy0.n
                                                    } as usize)
                                        }
                                    })
                            };
                        }
                    }
                    38 => {
                        {
                            unsafe {
                                sqlite3_add_not_null(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    39 => {
                        {
                            unsafe {
                                sqlite3_add_primary_key(p_parse_1, core::ptr::null_mut(),
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 })
                            };
                        }
                    }
                    40 => {
                        {
                            unsafe {
                                sqlite3_create_index(p_parse_1, core::ptr::null_mut(),
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    core::ptr::null_mut(),
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 },
                                    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, 1 as u8)
                            };
                        }
                    }
                    41 => {
                        {
                            unsafe {
                                sqlite3_add_check_constraint(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy0.z },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0.z })
                            };
                        }
                    }
                    42 => {
                        {
                            unsafe {
                                sqlite3_create_foreign_key(p_parse_1, core::ptr::null_mut(),
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    43 => {
                        {
                            unsafe {
                                sqlite3_defer_foreign_key(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    44 => {
                        {
                            unsafe {
                                sqlite3_add_collate_type(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    45 => {
                        {
                            unsafe {
                                sqlite3_add_generated(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    46 => {
                        {
                            unsafe {
                                sqlite3_add_generated(p_parse_1,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    48 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 1 }; }
                    }
                    49 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy144 = 0 * 257
                            };
                        }
                    }
                    50 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 } &
                                            !unsafe { (*yymsp.offset(0 as isize)).minor.yy383.mask } |
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy383.value }
                            };
                        }
                    }
                    51 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy383.value = 0
                            };
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy383.mask = 0
                            };
                        }
                    }
                    52 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy383.value = 0
                            };
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy383.mask = 0
                            };
                        }
                    }
                    53 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy383.value =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 }
                            };
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy383.mask = 255
                            };
                        }
                    }
                    54 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy383.value =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 } << 8
                            };
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy383.mask = 65280
                            };
                        }
                    }
                    55 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 8 }; }
                    }
                    56 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 9 }; }
                    }
                    57 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 10 }; }
                    }
                    58 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 7 }; }
                    }
                    59 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 0 }; }
                    }
                    60 => {
                        { unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 = 0 }; }
                    }
                    61 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 }
                            };
                        }
                    }
                    76 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 }
                            };
                        }
                    }
                    173 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 }
                            };
                        }
                    }
                    63 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 1 }; }
                    }
                    80 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 1 }; }
                    }
                    219 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 1 }; }
                    }
                    222 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 1 }; }
                    }
                    247 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 1 }; }
                    }
                    64 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 0 }; }
                    }
                    66 => {
                        {
                            { let _ = 0; };
                            unsafe { (*p_parse_1).u1.cr.constraint_name.n = 0 as u32 };
                        }
                    }
                    68 => {
                        {
                            unsafe {
                                sqlite3_add_primary_key(p_parse_1,
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 }, 0)
                            };
                        }
                    }
                    69 => {
                        {
                            unsafe {
                                sqlite3_create_index(p_parse_1, core::ptr::null_mut(),
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 },
                                    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, 1 as u8)
                            };
                        }
                    }
                    70 => {
                        {
                            unsafe {
                                sqlite3_add_check_constraint(p_parse_1,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy0.z },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy0.z })
                            };
                        }
                    }
                    71 => {
                        {
                            unsafe {
                                sqlite3_create_foreign_key(p_parse_1,
                                    unsafe { (*yymsp.offset(-6 as isize)).minor.yy14 },
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 })
                            };
                            unsafe {
                                sqlite3_defer_foreign_key(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    73 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 11 }; }
                    }
                    75 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 11 }; }
                    }
                    74 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 }
                            };
                        }
                    }
                    77 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 4 }; }
                    }
                    78 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 5 }; }
                    }
                    174 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 5 }; }
                    }
                    79 => {
                        {
                            unsafe {
                                sqlite3_drop_table(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy203 }, 0,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 })
                            };
                        }
                    }
                    82 => {
                        {
                            unsafe {
                                sqlite3_create_view(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-8 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy555 },
                                    unsafe { (*yymsp.offset(-7 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy144 })
                            };
                        }
                    }
                    83 => {
                        {
                            unsafe {
                                sqlite3_drop_table(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy203 }, 1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 })
                            };
                        }
                    }
                    84 => {
                        {
                            let mut dest: SelectDest =
                                SelectDest {
                                    e_dest: 7 as u8,
                                    i_sd_parm: 0,
                                    i_sd_parm2: 0,
                                    i_sdst: 0,
                                    n_sdst: 0,
                                    z_aff_sdst: core::ptr::null_mut(),
                                    p_order_by: core::ptr::null_mut(),
                                };
                            if unsafe { (*unsafe { (*p_parse_1).db }).m_db_flags } &
                                            64 as u32 != 0 as u32 ||
                                    unsafe { sqlite3_read_schema(p_parse_1) } == 0 {
                                unsafe {
                                    sqlite3_select(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy555 },
                                        &mut dest)
                                };
                            }
                            unsafe {
                                sqlite3_select_delete(unsafe { (*p_parse_1).db },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy555 })
                            };
                        }
                    }
                    85 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy555 =
                                    attach_with_to_select(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy555 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy59 })
                            };
                        }
                    }
                    86 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy555 =
                                    attach_with_to_select(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy555 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy59 })
                            };
                        }
                    }
                    87 => {
                        {
                            let mut p: *mut Select =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy555 };
                            if !(p).is_null() {
                                parser_double_link_select(p_parse_1, p);
                            }
                        }
                    }
                    88 => {
                        {
                            let mut p_rhs: *mut Select =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy555 };
                            let p_lhs: *mut Select =
                                unsafe { (*yymsp.offset(-2 as isize)).minor.yy555 };
                            if !(p_rhs).is_null() &&
                                    !(unsafe { (*p_rhs).p_prior }).is_null() {
                                let mut p_from: *mut SrcList = core::ptr::null_mut();
                                let mut x: Token = unsafe { core::mem::zeroed() };
                                x.n = 0 as u32;
                                parser_double_link_select(p_parse_1, p_rhs);
                                p_from =
                                    unsafe {
                                        sqlite3_src_list_append_from_term(p_parse_1,
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut(), &mut x, p_rhs, core::ptr::null_mut())
                                    };
                                p_rhs =
                                    unsafe {
                                        sqlite3_select_new(p_parse_1, core::ptr::null_mut(), p_from,
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut(), core::ptr::null_mut(), 0 as u32,
                                            core::ptr::null_mut())
                                    };
                            }
                            if !(p_rhs).is_null() {
                                unsafe {
                                    (*p_rhs).op =
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 } as u8
                                };
                                unsafe { (*p_rhs).p_prior = p_lhs };
                                if !(p_lhs).is_null() {
                                    unsafe { (*p_lhs).sel_flags &= !(1024 as u32) };
                                }
                                unsafe { (*p_rhs).sel_flags &= !(1024 as u32) };
                                if unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 } !=
                                        136 {
                                    unsafe { (*p_parse_1).set_has_compound(1 as Bft as u32) };
                                }
                            } else {
                                unsafe {
                                    sqlite3_select_delete(unsafe { (*p_parse_1).db }, p_lhs)
                                };
                            }
                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy555 = p_rhs };
                        }
                    }
                    89 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    91 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    90 => {
                        {
                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 136 };
                        }
                    }
                    92 => {
                        {
                            unsafe {
                                (*yymsp.offset(-8 as isize)).minor.yy555 =
                                    unsafe {
                                        sqlite3_select_new(p_parse_1,
                                            unsafe { (*yymsp.offset(-6 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-7 as isize)).minor.yy144 } as u32,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    93 => {
                        {
                            unsafe {
                                (*yymsp.offset(-9 as isize)).minor.yy555 =
                                    unsafe {
                                        sqlite3_select_new(p_parse_1,
                                            unsafe { (*yymsp.offset(-7 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-6 as isize)).minor.yy203 },
                                            unsafe { (*yymsp.offset(-5 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-8 as isize)).minor.yy144 } as u32,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-9 as isize)).minor.yy555
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                        (*yymsp.offset(-9 as isize)).minor.yy555
                                                    }).p_win_defn =
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy211 }
                                };
                            } else {
                                unsafe {
                                    sqlite3_window_list_delete(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy211 })
                                };
                            }
                        }
                    }
                    94 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy555 =
                                    unsafe {
                                        sqlite3_select_new(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut(), 512 as u32, core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    95 => {
                        {
                            unsafe {
                                sqlite3_multi_values_end(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy555 })
                            };
                        }
                    }
                    96 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy555 =
                                    unsafe {
                                        sqlite3_multi_values(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy555 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                                    }
                            };
                        }
                    }
                    97 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy555 =
                                    unsafe {
                                        sqlite3_multi_values(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy555 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                                    }
                            };
                        }
                    }
                    98 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 1 }; }
                    }
                    99 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 2 }; }
                    }
                    101 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy14 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    134 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy14 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    144 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy14 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    234 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy14 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    237 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy14 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    242 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy14 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    102 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                    }
                            };
                            if unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n } >
                                    0 as u32 {
                                unsafe {
                                    sqlite3_expr_list_set_name(p_parse_1,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                        unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                            *const Token, 1)
                                };
                            }
                            unsafe {
                                sqlite3_expr_list_set_span(p_parse_1,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy168 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy168 })
                            };
                        }
                    }
                    103 => {
                        {
                            let mut p: *mut Expr =
                                unsafe {
                                    sqlite3_expr(unsafe { (*p_parse_1).db }, 180,
                                        core::ptr::null())
                                };
                            unsafe {
                                sqlite3_expr_set_error_offset(p,
                                    unsafe {
                                                unsafe {
                                                    (*yymsp.offset(0 as
                                                                                isize)).minor.yy0.z.offset_from(unsafe {
                                                            (*p_parse_1).z_tail
                                                        })
                                                }
                                            } as i64 as i32)
                            };
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 }, p)
                                    }
                            };
                        }
                    }
                    104 => {
                        {
                            let mut p_right: *mut Expr = core::ptr::null_mut();
                            let mut p_left: *mut Expr = core::ptr::null_mut();
                            let mut p_dot: *mut Expr = core::ptr::null_mut();
                            p_right =
                                unsafe {
                                    sqlite3_p_expr(p_parse_1, 180, core::ptr::null_mut(),
                                        core::ptr::null_mut())
                                };
                            unsafe {
                                sqlite3_expr_set_error_offset(p_right,
                                    unsafe {
                                                unsafe {
                                                    (*yymsp.offset(0 as
                                                                                isize)).minor.yy0.z.offset_from(unsafe {
                                                            (*p_parse_1).z_tail
                                                        })
                                                }
                                            } as i64 as i32)
                            };
                            p_left =
                                token_expr(p_parse_1, 60,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy0 });
                            p_dot =
                                unsafe { sqlite3_p_expr(p_parse_1, 142, p_left, p_right) };
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 }, p_dot)
                                    }
                            };
                        }
                    }
                    105 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    117 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    258 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    259 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    107 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy203 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    110 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy203 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    108 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy203 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy203 }
                            };
                            unsafe {
                                sqlite3_src_list_shift_join_type(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy203 })
                            };
                        }
                    }
                    109 => {
                        {
                            if !(unsafe {
                                                    (*yymsp.offset(-1 as isize)).minor.yy203
                                                }).is_null() &&
                                    unsafe {
                                            (*unsafe { (*yymsp.offset(-1 as isize)).minor.yy203 }).n_src
                                        } > 0 {
                                unsafe {
                                    (*(unsafe {
                                                                (*unsafe {
                                                                                    (*yymsp.offset(-1 as isize)).minor.yy203
                                                                                }).a.as_ptr()
                                                            } as
                                                            *mut SrcItem).offset((unsafe {
                                                                    (*unsafe { (*yymsp.offset(-1 as isize)).minor.yy203 }).n_src
                                                                } - 1) as isize)).fg.jointype =
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy144 } as u8
                                };
                            }
                        }
                    }
                    111 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy203 =
                                    unsafe {
                                        sqlite3_src_list_append_from_term(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy203 },
                                            unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                            core::ptr::null_mut(),
                                            unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy269 })
                                    }
                            };
                        }
                    }
                    112 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy203 =
                                    unsafe {
                                        sqlite3_src_list_append_from_term(p_parse_1,
                                            unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                            unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                            core::ptr::null_mut(),
                                            unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy269 })
                                    }
                            };
                            unsafe {
                                sqlite3_src_list_indexed_by(p_parse_1,
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 })
                            };
                        }
                    }
                    113 => {
                        {
                            unsafe {
                                (*yymsp.offset(-7 as isize)).minor.yy203 =
                                    unsafe {
                                        sqlite3_src_list_append_from_term(p_parse_1,
                                            unsafe { (*yymsp.offset(-7 as isize)).minor.yy203 },
                                            unsafe { &mut (*yymsp.offset(-6 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-5 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                            core::ptr::null_mut(),
                                            unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy269 })
                                    }
                            };
                            unsafe {
                                sqlite3_src_list_func_args(p_parse_1,
                                    unsafe { (*yymsp.offset(-7 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy14 })
                            };
                        }
                    }
                    114 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy203 =
                                    unsafe {
                                        sqlite3_src_list_append_from_term(p_parse_1,
                                            unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy555 },
                                            unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy269 })
                                    }
                            };
                        }
                    }
                    115 => {
                        {
                            if unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 } ==
                                                core::ptr::null_mut() &&
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy0.n } ==
                                                0 as u32 &&
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy269.p_on } ==
                                            core::ptr::null_mut() &&
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy269.p_using }
                                        == core::ptr::null_mut() {
                                unsafe {
                                    (*yymsp.offset(-5 as isize)).minor.yy203 =
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 }
                                };
                            } else if unsafe {
                                            (*yymsp.offset(-3 as isize)).minor.yy203
                                        } != core::ptr::null_mut() &&
                                    unsafe {
                                            (*unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 }).n_src
                                        } == 1 {
                                unsafe {
                                    (*yymsp.offset(-5 as isize)).minor.yy203 =
                                        unsafe {
                                            sqlite3_src_list_append_from_term(p_parse_1,
                                                unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                                core::ptr::null_mut(), core::ptr::null_mut(),
                                                unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                                core::ptr::null_mut(),
                                                unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy269 })
                                        }
                                };
                                if !(unsafe {
                                                    (*yymsp.offset(-5 as isize)).minor.yy203
                                                }).is_null() {
                                    let p_new: *mut SrcItem =
                                        unsafe {
                                            &mut *(unsafe {
                                                                (*unsafe {
                                                                                    (*yymsp.offset(-5 as isize)).minor.yy203
                                                                                }).a.as_ptr()
                                                            } as
                                                            *mut SrcItem).offset((unsafe {
                                                                    (*unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 }).n_src
                                                                } - 1) as isize)
                                        };
                                    let p_old: *mut SrcItem =
                                        unsafe {
                                                (*unsafe {
                                                                    (*yymsp.offset(-3 as isize)).minor.yy203
                                                                }).a.as_ptr()
                                            } as *mut SrcItem;
                                    { let _ = 0; };
                                    unsafe { (*p_new).z_name = unsafe { (*p_old).z_name } };
                                    { let _ = 0; };
                                    if unsafe { (*p_old).fg.is_subquery() } != 0 {
                                        unsafe { (*p_new).fg.set_is_subquery(1 as u32 as u32) };
                                        unsafe {
                                            (*p_new).u4.p_subq = unsafe { (*p_old).u4.p_subq }
                                        };
                                        unsafe { (*p_old).u4.p_subq = core::ptr::null_mut() };
                                        unsafe { (*p_old).fg.set_is_subquery(0 as u32 as u32) };
                                        { let _ = 0; };
                                        if unsafe {
                                                        (*unsafe {
                                                                        (*unsafe { (*p_new).u4.p_subq }).p_select
                                                                    }).sel_flags
                                                    } & 2048 as u32 != 0 as u32 {
                                            unsafe { (*p_new).fg.set_is_nested_from(1 as u32 as u32) };
                                        }
                                    } else {
                                        unsafe {
                                            (*p_new).u4.z_database = unsafe { (*p_old).u4.z_database }
                                        };
                                        unsafe { (*p_old).u4.z_database = core::ptr::null_mut() };
                                    }
                                    if unsafe { (*p_old).fg.is_tab_func() } != 0 {
                                        unsafe {
                                            (*p_new).u1.p_func_arg = unsafe { (*p_old).u1.p_func_arg }
                                        };
                                        unsafe { (*p_old).u1.p_func_arg = core::ptr::null_mut() };
                                        unsafe { (*p_old).fg.set_is_tab_func(0 as u32 as u32) };
                                        unsafe { (*p_new).fg.set_is_tab_func(1 as u32 as u32) };
                                    }
                                    unsafe { (*p_old).z_name = core::ptr::null_mut() };
                                }
                                unsafe {
                                    sqlite3_src_list_delete(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 })
                                };
                            } else {
                                let mut p_subquery: *mut Select = core::ptr::null_mut();
                                unsafe {
                                    sqlite3_src_list_shift_join_type(p_parse_1,
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 })
                                };
                                p_subquery =
                                    unsafe {
                                        sqlite3_select_new(p_parse_1, core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 },
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut(), core::ptr::null_mut(), 2048 as u32,
                                            core::ptr::null_mut())
                                    };
                                unsafe {
                                    (*yymsp.offset(-5 as isize)).minor.yy203 =
                                        unsafe {
                                            sqlite3_src_list_append_from_term(p_parse_1,
                                                unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                                core::ptr::null_mut(), core::ptr::null_mut(),
                                                unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                                p_subquery,
                                                unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy269 })
                                        }
                                };
                            }
                        }
                    }
                    116 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.z = core::ptr::null()
                            };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as u32
                            };
                        }
                    }
                    131 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.z = core::ptr::null()
                            };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as u32
                            };
                        }
                    }
                    118 => {
                        {
                            yylhsminor.yy203 =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 },
                                        core::ptr::null_mut())
                                };
                            if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                    !(yylhsminor.yy203).is_null() {
                                unsafe {
                                    sqlite3_rename_token_map(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).z_name
                                            } as *const (),
                                        unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                            *const Token)
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy203 = yylhsminor.yy203
                        };
                    }
                    120 => {
                        {
                            yylhsminor.yy203 =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 },
                                        core::ptr::null_mut())
                                };
                            if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                    !(yylhsminor.yy203).is_null() {
                                unsafe {
                                    sqlite3_rename_token_map(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).z_name
                                            } as *const (),
                                        unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                            *const Token)
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy203 = yylhsminor.yy203
                        };
                    }
                    119 => {
                        {
                            yylhsminor.yy203 =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                        unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                                };
                            if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                    !(yylhsminor.yy203).is_null() {
                                unsafe {
                                    sqlite3_rename_token_map(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).z_name
                                            } as *const (),
                                        unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                            *const Token)
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy203 = yylhsminor.yy203
                        };
                    }
                    121 => {
                        {
                            yylhsminor.yy203 =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                        unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                                };
                            if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                    !(yylhsminor.yy203).is_null() {
                                unsafe {
                                    sqlite3_rename_token_map(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).z_name
                                            } as *const (),
                                        unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                            *const Token)
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy203 = yylhsminor.yy203
                        };
                    }
                    122 => {
                        {
                            yylhsminor.yy203 =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                        core::ptr::null_mut())
                                };
                            if !(yylhsminor.yy203).is_null() {
                                if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                                    unsafe {
                                        sqlite3_rename_token_map(p_parse_1,
                                            unsafe {
                                                    (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).z_name
                                                } as *const (),
                                            unsafe { &raw mut (*yymsp.offset(-2 as isize)).minor.yy0 }
                                                as *const Token)
                                    };
                                } else {
                                    unsafe {
                                        (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).z_alias =
                                            unsafe {
                                                sqlite3_name_from_token(unsafe { (*p_parse_1).db },
                                                    unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                                        *const Token)
                                            }
                                    };
                                }
                            }
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy203 = yylhsminor.yy203
                        };
                    }
                    123 => {
                        {
                            yylhsminor.yy203 =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                        unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 })
                                };
                            if !(yylhsminor.yy203).is_null() {
                                if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                                    unsafe {
                                        sqlite3_rename_token_map(p_parse_1,
                                            unsafe {
                                                    (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).z_name
                                                } as *const (),
                                            unsafe { &raw mut (*yymsp.offset(-2 as isize)).minor.yy0 }
                                                as *const Token)
                                    };
                                } else {
                                    unsafe {
                                        (*(unsafe { (*yylhsminor.yy203).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).z_alias =
                                            unsafe {
                                                sqlite3_name_from_token(unsafe { (*p_parse_1).db },
                                                    unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                                        *const Token)
                                            }
                                    };
                                }
                            }
                        }
                        unsafe {
                            (*yymsp.offset(-4 as isize)).minor.yy203 = yylhsminor.yy203
                        };
                    }
                    124 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 1 }; }
                    }
                    125 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy144 =
                                    unsafe {
                                        sqlite3_join_type(p_parse_1,
                                            unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                            core::ptr::null_mut(), core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    126 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy144 =
                                    unsafe {
                                        sqlite3_join_type(p_parse_1,
                                            unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    127 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy144 =
                                    unsafe {
                                        sqlite3_join_type(p_parse_1,
                                            unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                            unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 })
                                    }
                            };
                        }
                    }
                    128 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy269.p_on =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy269.p_using =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    129 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy269.p_on =
                                    core::ptr::null_mut()
                            };
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy269.p_using =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy132 }
                            };
                        }
                    }
                    130 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy269.p_on =
                                    core::ptr::null_mut()
                            };
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy269.p_using =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    132 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                        }
                    }
                    133 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0.z = core::ptr::null()
                            };
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0.n = 1 as u32
                            };
                        }
                    }
                    135 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy14 }
                            };
                        }
                    }
                    145 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy14 }
                            };
                        }
                    }
                    136 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                    }
                            };
                            unsafe {
                                sqlite3_expr_list_set_sort_order(unsafe {
                                        (*yymsp.offset(-4 as isize)).minor.yy14
                                    }, unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    137 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                    }
                            };

                            ///A-overwrites-Y
                            unsafe {
                                sqlite3_expr_list_set_sort_order(unsafe {
                                        (*yymsp.offset(-2 as isize)).minor.yy14
                                    }, unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    138 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 0 }; }
                    }
                    139 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 1 }; }
                    }
                    140 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = -1 }; }
                    }
                    143 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = -1 }; }
                    }
                    141 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 0 }; }
                    }
                    142 => {
                        { unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 1 }; }
                    }
                    146 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    148 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    153 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    155 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    232 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    233 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    252 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    147 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    154 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    156 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    231 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    251 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    149 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 149,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    150 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 149,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    151 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 149,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    152 => {
                        {
                            unsafe {
                                sqlite3_src_list_indexed_by(p_parse_1,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 })
                            };
                            unsafe {
                                sqlite3_delete_from(p_parse_1,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                    core::ptr::null_mut(), core::ptr::null_mut())
                            };
                        }
                    }
                    157 => {
                        {
                            unsafe {
                                sqlite3_add_returning(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy14 })
                            };
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    158 => {
                        {
                            unsafe {
                                sqlite3_add_returning(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy14 })
                            };
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 }
                            };
                        }
                    }
                    159 => {
                        {
                            unsafe {
                                sqlite3_src_list_indexed_by(p_parse_1,
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 })
                            };
                            unsafe {
                                sqlite3_expr_list_check_length(p_parse_1,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                    c"set list".as_ptr() as *mut i8 as *const i8)
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-1 as isize)).minor.yy203
                                            }).is_null() {
                                let mut p_from_clause: *mut SrcList =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy203 };
                                if unsafe { (*p_from_clause).n_src } > 1 {
                                    let mut p_subquery_1: *mut Select = core::ptr::null_mut();
                                    let mut as_: Token = unsafe { core::mem::zeroed() };
                                    p_subquery_1 =
                                        unsafe {
                                            sqlite3_select_new(p_parse_1, core::ptr::null_mut(),
                                                p_from_clause, core::ptr::null_mut(), core::ptr::null_mut(),
                                                core::ptr::null_mut(), core::ptr::null_mut(), 2048 as u32,
                                                core::ptr::null_mut())
                                        };
                                    as_.n = 0 as u32;
                                    as_.z = core::ptr::null();
                                    p_from_clause =
                                        unsafe {
                                            sqlite3_src_list_append_from_term(p_parse_1,
                                                core::ptr::null_mut(), core::ptr::null_mut(),
                                                core::ptr::null_mut(), &mut as_, p_subquery_1,
                                                core::ptr::null_mut())
                                        };
                                }
                                unsafe {
                                    (*yymsp.offset(-5 as isize)).minor.yy203 =
                                        unsafe {
                                            sqlite3_src_list_append_list(p_parse_1,
                                                unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                                p_from_clause)
                                        }
                                };
                            }
                            unsafe {
                                sqlite3_update(p_parse_1,
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-6 as isize)).minor.yy144 },
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    160 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                            unsafe {
                                sqlite3_expr_list_set_name(p_parse_1,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                    unsafe { &raw mut (*yymsp.offset(-2 as isize)).minor.yy0 }
                                        as *const Token, 1)
                            };
                        }
                    }
                    161 => {
                        {
                            unsafe {
                                (*yymsp.offset(-6 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append_vector(p_parse_1,
                                            unsafe { (*yymsp.offset(-6 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy132 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    162 => {
                        {
                            yylhsminor.yy14 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                };
                            unsafe {
                                sqlite3_expr_list_set_name(p_parse_1, yylhsminor.yy14,
                                    unsafe { &raw mut (*yymsp.offset(-2 as isize)).minor.yy0 }
                                        as *const Token, 1)
                            };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy14 = yylhsminor.yy14
                        };
                    }
                    163 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append_vector(p_parse_1,
                                            core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy132 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    164 => {
                        {
                            unsafe {
                                sqlite3_insert(p_parse_1,
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy555 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy132 },
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy122 })
                            };
                        }
                    }
                    165 => {
                        {
                            unsafe {
                                sqlite3_insert(p_parse_1,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy203 },
                                    core::ptr::null_mut(),
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy132 },
                                    unsafe { (*yymsp.offset(-6 as isize)).minor.yy144 },
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    166 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy122 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    167 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy122 =
                                    core::ptr::null_mut()
                            };
                            unsafe {
                                sqlite3_add_returning(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy14 })
                            };
                        }
                    }
                    168 => {
                        {
                            unsafe {
                                (*yymsp.offset(-11 as isize)).minor.yy122 =
                                    unsafe {
                                        sqlite3_upsert_new(unsafe { (*p_parse_1).db },
                                            unsafe { (*yymsp.offset(-8 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-6 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy122 })
                                    }
                            };
                        }
                    }
                    169 => {
                        {
                            unsafe {
                                (*yymsp.offset(-8 as isize)).minor.yy122 =
                                    unsafe {
                                        sqlite3_upsert_new(unsafe { (*p_parse_1).db },
                                            unsafe { (*yymsp.offset(-5 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 },
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy122 })
                                    }
                            };
                        }
                    }
                    170 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy122 =
                                    unsafe {
                                        sqlite3_upsert_new(unsafe { (*p_parse_1).db },
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    171 => {
                        {
                            unsafe {
                                (*yymsp.offset(-7 as isize)).minor.yy122 =
                                    unsafe {
                                        sqlite3_upsert_new(unsafe { (*p_parse_1).db },
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    172 => {
                        {
                            unsafe {
                                sqlite3_add_returning(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy14 })
                            };
                        }
                    }
                    175 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy132 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    176 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy132 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy132 }
                            };
                        }
                    }
                    177 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy132 =
                                    unsafe {
                                        sqlite3_id_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy132 },
                                            unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                                    }
                            };
                        }
                    }
                    178 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy132 =
                                    unsafe {
                                        sqlite3_id_list_append(p_parse_1, core::ptr::null_mut(),
                                            unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                                    }
                            };
                        }
                    }
                    179 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 }
                            };
                        }
                    }
                    180 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy454 =
                                    token_expr(p_parse_1, 60,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    181 => {
                        {
                            let temp1: *mut Expr =
                                token_expr(p_parse_1, 60,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy0 });
                            let temp2: *mut Expr =
                                token_expr(p_parse_1, 60,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 });
                            yylhsminor.yy454 =
                                unsafe { sqlite3_p_expr(p_parse_1, 142, temp1, temp2) };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    182 => {
                        {
                            let temp1: *mut Expr =
                                token_expr(p_parse_1, 60,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy0 });
                            let temp2: *mut Expr =
                                token_expr(p_parse_1, 60,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy0 });
                            let temp3: *mut Expr =
                                token_expr(p_parse_1, 60,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 });
                            let temp4: *mut Expr =
                                unsafe { sqlite3_p_expr(p_parse_1, 142, temp2, temp3) };
                            if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse_1, core::ptr::null(),
                                        temp1 as *const ())
                                };
                            }
                            yylhsminor.yy454 =
                                unsafe { sqlite3_p_expr(p_parse_1, 142, temp1, temp4) };
                        }
                        unsafe {
                            (*yymsp.offset(-4 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    183 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy454 =
                                    token_expr(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).major } as i32,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    184 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy454 =
                                    token_expr(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).major } as i32,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    185 => {
                        {
                            let mut i_value: i32 = 0;
                            if unsafe {
                                        sqlite3_get_int32(unsafe {
                                                (*yymsp.offset(0 as isize)).minor.yy0.z
                                            }, &mut i_value)
                                    } == 0 {
                                yylhsminor.yy454 =
                                    unsafe {
                                        sqlite3_expr_alloc(unsafe { (*p_parse_1).db }, 156,
                                            unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                                *const Token, 0)
                                    };
                            } else {
                                yylhsminor.yy454 =
                                    unsafe {
                                        sqlite3_expr_int32(unsafe { (*p_parse_1).db }, i_value)
                                    };
                            }
                            if !(yylhsminor.yy454).is_null() {
                                unsafe {
                                    (*yylhsminor.yy454).w.i_ofst =
                                        unsafe {
                                                    unsafe {
                                                        (*yymsp.offset(0 as
                                                                                    isize)).minor.yy0.z.offset_from(unsafe {
                                                                (*p_parse_1).z_tail
                                                            })
                                                    }
                                                } as i64 as i32
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    186 => {
                        {
                            if !(unsafe {
                                                            *unsafe {
                                                                    (*yymsp.offset(0 as isize)).minor.yy0.z.offset(0 as isize)
                                                                }
                                                        } as i32 == '#' as i32 &&
                                                unsafe {
                                                                *(sqlite3_ctype_map.as_ptr() as
                                                                            *const u8).add(unsafe {
                                                                                    *unsafe {
                                                                                            (*yymsp.offset(0 as isize)).minor.yy0.z.offset(1 as isize)
                                                                                        }
                                                                                } as u8 as usize)
                                                            } as i32 & 4 != 0) as i32 != 0 {
                                let n: u32 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n };
                                unsafe {
                                    (*yymsp.offset(0 as isize)).minor.yy454 =
                                        token_expr(p_parse_1, 157,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy0 })
                                };
                                unsafe {
                                    sqlite3_expr_assign_var_number(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }, n)
                                };
                            } else {
                                /// When doing a nested parse, one can include terms in an expression
                                ///* that look like this:   #1 #2 ...  These terms refer to registers
                                ///* in the virtual machine.  #N is the N-th register.
                                let mut t: Token =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 };

                                ///A-overwrites-X
                                { let _ = 0; };
                                if unsafe { (*p_parse_1).nested } as i32 == 0 {
                                    parser_syntax_error(p_parse_1, &mut t);
                                    unsafe {
                                        (*yymsp.offset(0 as isize)).minor.yy454 =
                                            core::ptr::null_mut()
                                    };
                                } else {
                                    unsafe {
                                        (*yymsp.offset(0 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1, 176, core::ptr::null_mut(),
                                                    core::ptr::null_mut())
                                            }
                                    };
                                    if !(unsafe {
                                                        (*yymsp.offset(0 as isize)).minor.yy454
                                                    }).is_null() {
                                        unsafe {
                                            sqlite3_get_int32(unsafe { &*t.z.offset(1 as isize) },
                                                unsafe {
                                                    &mut (*unsafe {
                                                                        (*yymsp.offset(0 as isize)).minor.yy454
                                                                    }).i_table
                                                })
                                        };
                                    }
                                }
                            }
                        }
                    }
                    187 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_expr_add_collate_token(p_parse_1 as *const Parse,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                                *const Token, 1)
                                    }
                            };
                        }
                    }
                    188 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_expr_alloc(unsafe { (*p_parse_1).db }, 36,
                                            unsafe { &raw mut (*yymsp.offset(-1 as isize)).minor.yy0 }
                                                as *const Token, 1)
                                    }
                            };
                            unsafe {
                                sqlite3_expr_attach_subtrees(unsafe { (*p_parse_1).db },
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 },
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    189 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1,
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                        unsafe { &raw mut (*yymsp.offset(-4 as isize)).minor.yy0 }
                                            as *const Token,
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-4 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    190 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                        unsafe { &raw mut (*yymsp.offset(-7 as isize)).minor.yy0 }
                                            as *const Token,
                                        unsafe { (*yymsp.offset(-5 as isize)).minor.yy144 })
                                };
                            unsafe {
                                sqlite3_expr_add_function_order_by(p_parse_1,
                                    yylhsminor.yy454,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                            };
                        }
                        unsafe {
                            (*yymsp.offset(-7 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    191 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &raw mut (*yymsp.offset(-3 as isize)).minor.yy0 }
                                            as *const Token, 0)
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-3 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    192 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1,
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                        unsafe { &raw mut (*yymsp.offset(-5 as isize)).minor.yy0 }
                                            as *const Token,
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 })
                                };
                            unsafe {
                                sqlite3_window_attach(p_parse_1, yylhsminor.yy454,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy211 })
                            };
                        }
                        unsafe {
                            (*yymsp.offset(-5 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    193 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1,
                                        unsafe { (*yymsp.offset(-5 as isize)).minor.yy14 },
                                        unsafe { &raw mut (*yymsp.offset(-8 as isize)).minor.yy0 }
                                            as *const Token,
                                        unsafe { (*yymsp.offset(-6 as isize)).minor.yy144 })
                                };
                            unsafe {
                                sqlite3_window_attach(p_parse_1, yylhsminor.yy454,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy211 })
                            };
                            unsafe {
                                sqlite3_expr_add_function_order_by(p_parse_1,
                                    yylhsminor.yy454,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 })
                            };
                        }
                        unsafe {
                            (*yymsp.offset(-8 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    194 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &raw mut (*yymsp.offset(-4 as isize)).minor.yy0 }
                                            as *const Token, 0)
                                };
                            unsafe {
                                sqlite3_window_attach(p_parse_1, yylhsminor.yy454,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy211 })
                            };
                        }
                        unsafe {
                            (*yymsp.offset(-4 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    195 => {
                        {
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                            *const Token, 0)
                                };
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    196 => {
                        {
                            let p_list: *mut ExprList =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1,
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy14 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 })
                                };
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 177, core::ptr::null_mut(),
                                            core::ptr::null_mut())
                                    }
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-4 as isize)).minor.yy454
                                            }).is_null() {
                                let mut i: i32 = 0;
                                unsafe {
                                    (*unsafe {
                                                            (*yymsp.offset(-4 as isize)).minor.yy454
                                                        }).x.p_list = p_list
                                };
                                {
                                    i = 0;
                                    '__b8: loop {
                                        if !(i < unsafe { (*p_list).n_expr }) { break '__b8; }
                                        '__c8: loop {
                                            { let _ = 0; };
                                            unsafe {
                                                (*unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 }).flags
                                                    |=
                                                    unsafe {
                                                            (*unsafe {
                                                                            (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                                                        }).flags
                                                        } & (512 | 4194304 | 8) as u32
                                            };
                                            break '__c8;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            } else {
                                unsafe {
                                    sqlite3_expr_list_delete(unsafe { (*p_parse_1).db }, p_list)
                                };
                            }
                        }
                    }
                    197 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_expr_and(p_parse_1,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    198 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    199 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    200 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    201 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    202 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    203 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    204 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    205 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 }
                            };
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0.n |= 2147483648u32
                            };
                        }
                    }
                    206 => {
                        {
                            let mut p_list_1: *mut ExprList = core::ptr::null_mut();
                            let b_not: i32 =
                                (unsafe { (*yymsp.offset(-1 as isize)).minor.yy0.n } &
                                        2147483648u32) as i32;
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0.n &=
                                    2147483647 as u32
                            };
                            p_list_1 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                };
                            p_list_1 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, p_list_1,
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                };
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_expr_function(p_parse_1, p_list_1,
                                            unsafe { &raw mut (*yymsp.offset(-1 as isize)).minor.yy0 }
                                                as *const Token, 0)
                                    }
                            };
                            if b_not != 0 {
                                unsafe {
                                    (*yymsp.offset(-2 as isize)).minor.yy454 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, 19,
                                                unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                                core::ptr::null_mut())
                                        }
                                };
                            }
                            if !(unsafe {
                                                (*yymsp.offset(-2 as isize)).minor.yy454
                                            }).is_null() {
                                unsafe {
                                    (*unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 }).flags
                                        |= 256 as u32
                                };
                            }
                        }
                    }
                    207 => {
                        {
                            let mut p_list_2: *mut ExprList = core::ptr::null_mut();
                            let b_not_1: i32 =
                                (unsafe { (*yymsp.offset(-3 as isize)).minor.yy0.n } &
                                        2147483648u32) as i32;
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy0.n &=
                                    2147483647 as u32
                            };
                            p_list_2 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                };
                            p_list_2 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, p_list_2,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 })
                                };
                            p_list_2 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, p_list_2,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                };
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_expr_function(p_parse_1, p_list_2,
                                            unsafe { &raw mut (*yymsp.offset(-3 as isize)).minor.yy0 }
                                                as *const Token, 0)
                                    }
                            };
                            if b_not_1 != 0 {
                                unsafe {
                                    (*yymsp.offset(-4 as isize)).minor.yy454 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, 19,
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                core::ptr::null_mut())
                                        }
                                };
                            }
                            if !(unsafe {
                                                (*yymsp.offset(-4 as isize)).minor.yy454
                                            }).is_null() {
                                unsafe {
                                    (*unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 }).flags
                                        |= 256 as u32
                                };
                            }
                        }
                    }
                    208 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    sqlite3_p_expr_is_null(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).major } as i32,
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 })
                            };
                        }
                    }
                    209 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    sqlite3_p_expr_is_null(p_parse_1, 52,
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                            };
                        }
                    }
                    210 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    sqlite3_p_expr_is(p_parse_1, 45,
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    211 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy454 =
                                    sqlite3_p_expr_is(p_parse_1, 46,
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    212 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy454 =
                                    sqlite3_p_expr_is(p_parse_1, 45,
                                        unsafe { (*yymsp.offset(-5 as isize)).minor.yy454 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    213 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    sqlite3_p_expr_is(p_parse_1, 46,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    214 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    215 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1,
                                            unsafe { (*yymsp.offset(-1 as isize)).major } as i32,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    216 => {
                        {
                            let mut p: *mut Expr =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy454 };
                            let op: u8 =
                                (unsafe { (*yymsp.offset(-1 as isize)).major } as i32 +
                                        (173 - 107)) as u8;
                            { let _ = 0; };
                            { let _ = 0; };
                            if !(p).is_null() && unsafe { (*p).op } as i32 == 173 {
                                unsafe { (*p).op = op };
                                unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 = p };
                            } else {
                                unsafe {
                                    (*yymsp.offset(-1 as isize)).minor.yy454 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, op as i32, p,
                                                core::ptr::null_mut())
                                        }
                                };
                            }
                        }
                    }
                    217 => {
                        {
                            let mut p_list_3: *mut ExprList =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                };
                            p_list_3 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, p_list_3,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                };
                            yylhsminor.yy454 =
                                unsafe {
                                    sqlite3_expr_function(p_parse_1, p_list_3,
                                        unsafe { &raw mut (*yymsp.offset(-1 as isize)).minor.yy0 }
                                            as *const Token, 0)
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    218 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 0 }; }
                    }
                    221 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 0 }; }
                    }
                    220 => {
                        {
                            let mut p_list_4: *mut ExprList =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                };
                            p_list_4 =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse_1, p_list_4,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                };
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 49,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-4 as isize)).minor.yy454
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                            (*yymsp.offset(-4 as isize)).minor.yy454
                                                        }).x.p_list = p_list_4
                                };
                                unsafe {
                                    sqlite3_expr_set_height_and_flags(p_parse_1,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 })
                                };
                            } else {
                                unsafe {
                                    sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                        p_list_4)
                                };
                            }
                            if unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } != 0
                                {
                                unsafe {
                                    (*yymsp.offset(-4 as isize)).minor.yy454 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, 19,
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                core::ptr::null_mut())
                                        }
                                };
                            }
                        }
                    }
                    223 => {
                        {
                            if unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 } ==
                                    core::ptr::null_mut() {
                                /// Expressions of the form
                                ///*
                                ///*      expr1 IN ()
                                ///*      expr1 NOT IN ()
                                ///*
                                ///* simplify to constants 0 (false) and 1 (true), respectively.
                                ///*
                                ///* Except, do not apply this optimization if expr1 contains a function
                                ///* because that function might be an aggregate (we don't know yet whether
                                ///* it is or not) and if it is an aggregate, that could change the meaning
                                ///* of the whole query.
                                let p_b: *mut Expr =
                                    unsafe {
                                        sqlite3_expr(unsafe { (*p_parse_1).db }, 118,
                                            if unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } != 0
                                                    {
                                                    c"true".as_ptr() as *mut i8
                                                } else { c"false".as_ptr() as *mut i8 } as *const i8)
                                    };
                                if !(p_b).is_null() {
                                    unsafe { sqlite3_expr_id_to_true_false(p_b) };
                                }
                                if !(unsafe {
                                                            (*unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 }).flags
                                                        } & 8 as u32 != 0 as u32) as i32 != 0 {
                                    unsafe {
                                        sqlite3_expr_unmap_and_delete(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 })
                                    };
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 = p_b };
                                } else {
                                    unsafe {
                                        (*yymsp.offset(-4 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1,
                                                    if unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } != 0
                                                        {
                                                        43
                                                    } else { 44 }, p_b,
                                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 })
                                            }
                                    };
                                }
                            } else {
                                let mut p_rhs_1: *mut Expr =
                                    unsafe {
                                        (*(unsafe {
                                                            (*unsafe {
                                                                                (*yymsp.offset(-1 as isize)).minor.yy14
                                                                            }).a.as_ptr()
                                                        } as *mut ExprListItem).offset(0 as isize)).p_expr
                                    };
                                if unsafe {
                                                    (*unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 }).n_expr
                                                } == 1 &&
                                            unsafe { sqlite3_expr_is_constant(p_parse_1, p_rhs_1) } != 0
                                        &&
                                        unsafe {
                                                    (*unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 }).op
                                                } as i32 != 177 {
                                    unsafe {
                                        (*(unsafe {
                                                                (*unsafe {
                                                                                    (*yymsp.offset(-1 as isize)).minor.yy14
                                                                                }).a.as_ptr()
                                                            } as *mut ExprListItem).offset(0 as isize)).p_expr =
                                            core::ptr::null_mut()
                                    };
                                    unsafe {
                                        sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                                    };
                                    p_rhs_1 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, 173, p_rhs_1,
                                                core::ptr::null_mut())
                                        };
                                    unsafe {
                                        (*yymsp.offset(-4 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1, 54,
                                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                    p_rhs_1)
                                            }
                                    };
                                } else if unsafe {
                                                (*unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 }).n_expr
                                            } == 1 && unsafe { (*p_rhs_1).op } as i32 == 139 {
                                    unsafe {
                                        (*yymsp.offset(-4 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1, 50,
                                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                    core::ptr::null_mut())
                                            }
                                    };
                                    unsafe {
                                        sqlite3_p_expr_add_select(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                            unsafe { (*p_rhs_1).x.p_select })
                                    };
                                    unsafe { (*p_rhs_1).x.p_select = core::ptr::null_mut() };
                                    unsafe {
                                        sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                                    };
                                } else {
                                    unsafe {
                                        (*yymsp.offset(-4 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1, 50,
                                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                    core::ptr::null_mut())
                                            }
                                    };
                                    if unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 } ==
                                            core::ptr::null_mut() {
                                        unsafe {
                                            sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                                unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                                        };
                                    } else if unsafe {
                                                    (*unsafe {
                                                                    (*unsafe {
                                                                                    (*yymsp.offset(-4 as isize)).minor.yy454
                                                                                }).p_left
                                                                }).op
                                                } as i32 == 177 {
                                        let n_expr: i32 =
                                            unsafe {
                                                (*unsafe {
                                                                (*unsafe {
                                                                                    (*unsafe {
                                                                                                    (*yymsp.offset(-4 as isize)).minor.yy454
                                                                                                }).p_left
                                                                                }).x.p_list
                                                            }).n_expr
                                            };
                                        let p_select_rhs: *mut Select =
                                            unsafe {
                                                sqlite3_expr_list_to_values(p_parse_1, n_expr,
                                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 })
                                            };
                                        if !(p_select_rhs).is_null() {
                                            parser_double_link_select(p_parse_1, p_select_rhs);
                                            unsafe {
                                                sqlite3_p_expr_add_select(p_parse_1,
                                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                    p_select_rhs)
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            (*unsafe {
                                                                    (*yymsp.offset(-4 as isize)).minor.yy454
                                                                }).x.p_list =
                                                unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 }
                                        };
                                        unsafe {
                                            sqlite3_expr_set_height_and_flags(p_parse_1,
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 })
                                        };
                                    }
                                }
                                if unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } != 0
                                    {
                                    unsafe {
                                        (*yymsp.offset(-4 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1, 19,
                                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                    core::ptr::null_mut())
                                            }
                                    };
                                }
                            }
                        }
                    }
                    224 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 139, core::ptr::null_mut(),
                                            core::ptr::null_mut())
                                    }
                            };
                            unsafe {
                                sqlite3_p_expr_add_select(p_parse_1,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy555 })
                            };
                        }
                    }
                    225 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 50,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                            unsafe {
                                sqlite3_p_expr_add_select(p_parse_1,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy555 })
                            };
                            if unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } != 0
                                {
                                unsafe {
                                    (*yymsp.offset(-4 as isize)).minor.yy454 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, 19,
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                core::ptr::null_mut())
                                        }
                                };
                            }
                        }
                    }
                    226 => {
                        {
                            let p_src: *mut SrcList =
                                unsafe {
                                    sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                        unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 })
                                };
                            let p_select: *mut Select =
                                unsafe {
                                    sqlite3_select_new(p_parse_1, core::ptr::null_mut(), p_src,
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(), 0 as u32,
                                        core::ptr::null_mut())
                                };
                            if !(unsafe {
                                                (*yymsp.offset(0 as isize)).minor.yy14
                                            }).is_null() {
                                unsafe {
                                    sqlite3_src_list_func_args(p_parse_1,
                                        if !(p_select).is_null() {
                                            p_src
                                        } else { core::ptr::null_mut() },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy14 })
                                };
                            }
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 50,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                            unsafe {
                                sqlite3_p_expr_add_select(p_parse_1,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                    p_select)
                            };
                            if unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } != 0
                                {
                                unsafe {
                                    (*yymsp.offset(-4 as isize)).minor.yy454 =
                                        unsafe {
                                            sqlite3_p_expr(p_parse_1, 19,
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 },
                                                core::ptr::null_mut())
                                        }
                                };
                            }
                        }
                    }
                    227 => {
                        {
                            let mut p: *mut Expr = core::ptr::null_mut();
                            p =
                                {
                                    unsafe {
                                        (*yymsp.offset(-3 as isize)).minor.yy454 =
                                            unsafe {
                                                sqlite3_p_expr(p_parse_1, 20, core::ptr::null_mut(),
                                                    core::ptr::null_mut())
                                            }
                                    };
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 }
                                };
                            unsafe {
                                sqlite3_p_expr_add_select(p_parse_1, p,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy555 })
                            };
                        }
                    }
                    228 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 158,
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-4 as isize)).minor.yy454
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                            (*yymsp.offset(-4 as isize)).minor.yy454
                                                        }).x.p_list =
                                        if !(unsafe {
                                                            (*yymsp.offset(-1 as isize)).minor.yy454
                                                        }).is_null() {
                                            unsafe {
                                                sqlite3_expr_list_append(p_parse_1,
                                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 })
                                            }
                                        } else {
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 }
                                        }
                                };
                                unsafe {
                                    sqlite3_expr_set_height_and_flags(p_parse_1,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy454 })
                                };
                            } else {
                                unsafe {
                                    sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 })
                                };
                                unsafe {
                                    sqlite3_expr_delete(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 })
                                };
                            }
                        }
                    }
                    229 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                    }
                            };
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    230 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                                    }
                            };
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    235 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    236 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy14 =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                    }
                            };
                        }
                    }
                    238 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 }
                            };
                        }
                    }
                    243 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 }
                            };
                        }
                    }
                    239 => {
                        {
                            unsafe {
                                sqlite3_create_index(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-7 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-6 as isize)).minor.yy0 },
                                    unsafe {
                                        sqlite3_src_list_append(p_parse_1, core::ptr::null_mut(),
                                            unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                            core::ptr::null_mut())
                                    }, unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                    unsafe { (*yymsp.offset(-10 as isize)).minor.yy144 },
                                    unsafe { &mut (*yymsp.offset(-11 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }, 0,
                                    unsafe { (*yymsp.offset(-8 as isize)).minor.yy144 },
                                    0 as u8)
                            };
                            if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                    !(unsafe { (*p_parse_1).p_new_index }).is_null() {
                                unsafe {
                                    sqlite3_rename_token_map(p_parse_1,
                                        unsafe { (*unsafe { (*p_parse_1).p_new_index }).z_name } as
                                            *const (),
                                        unsafe { &raw mut (*yymsp.offset(-4 as isize)).minor.yy0 }
                                            as *const Token)
                                };
                            }
                        }
                    }
                    240 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 2 }; }
                    }
                    281 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 2 }; }
                    }
                    241 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 0 }; }
                    }
                    244 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy14 =
                                    parser_add_expr_id_list_term(p_parse_1,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                        unsafe { &raw mut (*yymsp.offset(-2 as isize)).minor.yy0 }
                                            as *const Token,
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    245 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy14 =
                                    parser_add_expr_id_list_term(p_parse_1,
                                        core::ptr::null_mut(),
                                        unsafe { &raw mut (*yymsp.offset(-2 as isize)).minor.yy0 }
                                            as *const Token,
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy144 })
                            };
                        }
                    }
                    248 => {
                        {
                            unsafe {
                                sqlite3_drop_index(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 })
                            };
                        }
                    }
                    249 => {
                        {
                            unsafe {
                                sqlite3_vacuum(p_parse_1, core::ptr::null_mut(),
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    250 => {
                        {
                            unsafe {
                                sqlite3_vacuum(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    253 => {
                        {
                            unsafe {
                                sqlite3_pragma(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 },
                                    core::ptr::null_mut(), 0)
                            };
                        }
                    }
                    254 => {
                        {
                            unsafe {
                                sqlite3_pragma(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 }, 0)
                            };
                        }
                    }
                    255 => {
                        {
                            unsafe {
                                sqlite3_pragma(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 }, 0)
                            };
                        }
                    }
                    256 => {
                        {
                            unsafe {
                                sqlite3_pragma(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 }, 1)
                            };
                        }
                    }
                    257 => {
                        {
                            unsafe {
                                sqlite3_pragma(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 }, 1)
                            };
                        }
                    }
                    260 => {
                        {
                            let mut all: Token = unsafe { core::mem::zeroed() };
                            all.z = unsafe { (*yymsp.offset(-3 as isize)).minor.yy0.z };
                            all.n =
                                unsafe {
                                                    unsafe {
                                                        (*yymsp.offset(0 as
                                                                                    isize)).minor.yy0.z.offset_from(unsafe {
                                                                (*yymsp.offset(-3 as isize)).minor.yy0.z
                                                            })
                                                    }
                                                } as i64 as i32 as u32 +
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n };
                            unsafe {
                                sqlite3_finish_trigger(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy427 },
                                    &mut all)
                            };
                        }
                    }
                    261 => {
                        {
                            unsafe {
                                sqlite3_begin_trigger(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-7 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-6 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy286.a },
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy286.b },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-10 as isize)).minor.yy144 },
                                    unsafe { (*yymsp.offset(-8 as isize)).minor.yy144 })
                            };
                            unsafe {
                                (*yymsp.offset(-10 as isize)).minor.yy0 =
                                    if unsafe { (*yymsp.offset(-6 as isize)).minor.yy0.n } ==
                                            0 as u32 {
                                        unsafe { (*yymsp.offset(-7 as isize)).minor.yy0 }
                                    } else { unsafe { (*yymsp.offset(-6 as isize)).minor.yy0 } }
                            };
                        }
                    }
                    262 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy144 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                        }
                    }
                    263 => {
                        {
                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 = 66 };
                        }
                    }
                    264 => {
                        { unsafe { (*yymsp.offset(1 as isize)).minor.yy144 = 33 }; }
                    }
                    265 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy286.a =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                            unsafe {

                                ///A-overwrites-X
                                ((*yymsp.offset(0 as isize)).minor.yy286.b =
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    266 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy286.a =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32
                            };
                            unsafe {

                                ///A-overwrites-X
                                ((*yymsp.offset(0 as isize)).minor.yy286.b =
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    267 => {
                        {
                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy286.a = 130 };
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy286.b =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy132 }
                            };
                        }
                    }
                    268 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    286 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy454 =
                                    core::ptr::null_mut()
                            };
                        }
                    }
                    269 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    287 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                            };
                        }
                    }
                    270 => {
                        {
                            unsafe {
                                (*unsafe {
                                                    (*unsafe {
                                                                    (*yymsp.offset(-2 as isize)).minor.yy427
                                                                }).p_last
                                                }).p_next =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy427 }
                            };
                            unsafe {
                                (*unsafe {
                                                    (*yymsp.offset(-2 as isize)).minor.yy427
                                                }).p_last =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy427 }
                            };
                        }
                    }
                    271 => {
                        {
                            unsafe {
                                (*unsafe {
                                                    (*yymsp.offset(-1 as isize)).minor.yy427
                                                }).p_last =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy427 }
                            };
                        }
                    }
                    272 => {
                        {
                            unsafe {
                                sqlite3_error_msg(p_parse_1,
                                    c"the INDEXED BY clause is not allowed on UPDATE or DELETE statements within triggers".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                        }
                    }
                    273 => {
                        {
                            unsafe {
                                sqlite3_error_msg(p_parse_1,
                                    c"the NOT INDEXED clause is not allowed on UPDATE or DELETE statements within triggers".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                        }
                    }
                    274 => {
                        {
                            yylhsminor.yy427 =
                                unsafe {
                                    sqlite3_trigger_update_step(p_parse_1,
                                        unsafe { (*yymsp.offset(-6 as isize)).minor.yy203 },
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy203 },
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy14 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                        unsafe { (*yymsp.offset(-7 as isize)).minor.yy144 } as u8,
                                        unsafe { (*yymsp.offset(-8 as isize)).minor.yy0.z },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy168 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-8 as isize)).minor.yy427 = yylhsminor.yy427
                        };
                    }
                    275 => {
                        {
                            yylhsminor.yy427 =
                                unsafe {
                                    sqlite3_trigger_insert_step(p_parse_1,
                                        unsafe { (*yymsp.offset(-4 as isize)).minor.yy203 },
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy132 },
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy555 },
                                        unsafe { (*yymsp.offset(-6 as isize)).minor.yy144 } as u8,
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy122 },
                                        unsafe { (*yymsp.offset(-7 as isize)).minor.yy168 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy168 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-7 as isize)).minor.yy427 = yylhsminor.yy427
                        };
                    }
                    276 => {
                        {
                            yylhsminor.yy427 =
                                unsafe {
                                    sqlite3_trigger_delete_step(p_parse_1,
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                        unsafe { (*yymsp.offset(-5 as isize)).minor.yy0.z },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy168 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-5 as isize)).minor.yy427 = yylhsminor.yy427
                        };
                    }
                    277 => {
                        {
                            yylhsminor.yy427 =
                                unsafe {
                                    sqlite3_trigger_select_step(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy555 },
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy168 },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy168 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy427 = yylhsminor.yy427
                        };
                    }
                    278 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 72, core::ptr::null_mut(),
                                            core::ptr::null_mut())
                                    }
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-3 as isize)).minor.yy454
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                        (*yymsp.offset(-3 as isize)).minor.yy454
                                                    }).aff_expr = 4 as i8
                                };
                            }
                        }
                    }
                    279 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy454 =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 72,
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                            core::ptr::null_mut())
                                    }
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-5 as isize)).minor.yy454
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                        (*yymsp.offset(-5 as isize)).minor.yy454
                                                    }).aff_expr =
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy144 } as i8
                                };
                            }
                        }
                    }
                    280 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 1 }; }
                    }
                    282 => {
                        { unsafe { (*yymsp.offset(0 as isize)).minor.yy144 = 3 }; }
                    }
                    283 => {
                        {
                            unsafe {
                                sqlite3_drop_trigger(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy203 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy144 })
                            };
                        }
                    }
                    284 => {
                        {
                            unsafe {
                                sqlite3_attach(p_parse_1,
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    285 => {
                        {
                            unsafe {
                                sqlite3_detach(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                            };
                        }
                    }
                    288 => {
                        {
                            unsafe {
                                sqlite3_reindex(p_parse_1, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    289 => {
                        {
                            unsafe {
                                sqlite3_reindex(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    290 => {
                        {
                            unsafe {
                                sqlite3_analyze(p_parse_1, core::ptr::null_mut(),
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    291 => {
                        {
                            unsafe {
                                sqlite3_analyze(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    292 => {
                        {
                            unsafe {
                                sqlite3_alter_rename_table(p_parse_1,
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    293 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy0.n =
                                    unsafe {
                                                        unsafe {
                                                            (*p_parse_1).s_last_token.z.offset_from(unsafe {
                                                                    (*yymsp.offset(-1 as isize)).minor.yy0.z
                                                                })
                                                        }
                                                    } as i64 as i32 as u32 +
                                        unsafe { (*p_parse_1).s_last_token.n }
                            };
                            unsafe {
                                sqlite3_alter_finish_add_column(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 })
                            };
                        }
                    }
                    294 => {
                        {
                            disable_lookaside(unsafe { &mut *p_parse_1 });
                            unsafe {
                                sqlite3_alter_begin_add_column(p_parse_1,
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy203 })
                            };
                            unsafe {
                                sqlite3_add_column(p_parse_1,
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                            unsafe {
                                (*yymsp.offset(-6 as isize)).minor.yy0 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy0 }
                            };
                        }
                    }
                    295 => {
                        {
                            unsafe {
                                sqlite3_alter_drop_column(p_parse_1,
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 },
                                    unsafe { &raw mut (*yymsp.offset(0 as isize)).minor.yy0 } as
                                        *const Token)
                            };
                        }
                    }
                    296 => {
                        {
                            unsafe {
                                sqlite3_alter_rename_column(p_parse_1,
                                    unsafe { (*yymsp.offset(-5 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    297 => {
                        {
                            unsafe {
                                sqlite3_alter_drop_constraint(p_parse_1,
                                    unsafe { (*yymsp.offset(-3 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 },
                                    core::ptr::null_mut())
                            };
                        }
                    }
                    298 => {
                        {
                            unsafe {
                                sqlite3_alter_drop_constraint(p_parse_1,
                                    unsafe { (*yymsp.offset(-6 as isize)).minor.yy203 },
                                    core::ptr::null_mut(),
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 })
                            };
                        }
                    }
                    299 => {
                        {
                            unsafe {
                                sqlite3_alter_set_not_null(p_parse_1,
                                    unsafe { (*yymsp.offset(-7 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 })
                            };
                        }
                    }
                    300 => {
                        {
                            unsafe {
                                sqlite3_alter_add_constraint(p_parse_1,
                                    unsafe { (*yymsp.offset(-8 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-6 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-5 as isize)).minor.yy0 },
                                    unsafe {
                                        unsafe {
                                            (*yymsp.offset(-3 as isize)).minor.yy0.z.offset(1 as isize)
                                        }
                                    },
                                    (unsafe {
                                                    unsafe {
                                                        (*yymsp.offset(-1 as
                                                                                    isize)).minor.yy0.z.offset_from(unsafe {
                                                                (*yymsp.offset(-3 as isize)).minor.yy0.z
                                                            })
                                                    }
                                                } as i64 - 1 as i64) as i32,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                            };
                        }
                    }
                    301 => {
                        {
                            unsafe {
                                sqlite3_alter_add_constraint(p_parse_1,
                                    unsafe { (*yymsp.offset(-6 as isize)).minor.yy203 },
                                    unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 },
                                    core::ptr::null_mut(),
                                    unsafe {
                                        unsafe {
                                            (*yymsp.offset(-3 as isize)).minor.yy0.z.offset(1 as isize)
                                        }
                                    },
                                    (unsafe {
                                                    unsafe {
                                                        (*yymsp.offset(-1 as
                                                                                    isize)).minor.yy0.z.offset_from(unsafe {
                                                                (*yymsp.offset(-3 as isize)).minor.yy0.z
                                                            })
                                                    }
                                                } as i64 - 1 as i64) as i32,
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy454 })
                            };
                        }
                    }
                    302 => {
                        {
                            unsafe {
                                sqlite3_vtab_finish_parse(p_parse_1, core::ptr::null_mut())
                            };
                        }
                    }
                    303 => {
                        {
                            unsafe {
                                sqlite3_vtab_finish_parse(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    304 => {
                        {
                            unsafe {
                                sqlite3_vtab_begin_parse(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(-3 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(-2 as isize)).minor.yy0 },
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 },
                                    unsafe { (*yymsp.offset(-4 as isize)).minor.yy144 })
                            };
                        }
                    }
                    305 => { { unsafe { sqlite3_vtab_arg_init(p_parse_1) }; } }
                    306 => {
                        {
                            unsafe {
                                sqlite3_vtab_arg_extend(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    307 => {
                        {
                            unsafe {
                                sqlite3_vtab_arg_extend(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    308 => {
                        {
                            unsafe {
                                sqlite3_vtab_arg_extend(p_parse_1,
                                    unsafe { &mut (*yymsp.offset(0 as isize)).minor.yy0 })
                            };
                        }
                    }
                    309 => {
                        {
                            unsafe {
                                sqlite3_with_push(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy59 }, 1 as u8)
                            };
                        }
                    }
                    310 => {
                        {
                            unsafe {
                                sqlite3_with_push(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy59 }, 1 as u8)
                            };
                        }
                    }
                    311 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy462 = 1 as u8
                            };
                        }
                    }
                    312 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy462 = 0 as u8
                            };
                        }
                    }
                    313 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy462 = 2 as u8
                            };
                        }
                    }
                    314 => {
                        {
                            unsafe {
                                (*yymsp.offset(-5 as isize)).minor.yy67 =
                                    unsafe {
                                        sqlite3_cte_new(p_parse_1,
                                            unsafe { &mut (*yymsp.offset(-5 as isize)).minor.yy0 },
                                            unsafe { (*yymsp.offset(-4 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy555 },
                                            unsafe { (*yymsp.offset(-3 as isize)).minor.yy462 })
                                    }
                            };
                        }
                    }
                    315 => {
                        { unsafe { (*p_parse_1).set_b_has_with(1 as Bft as u32) }; }
                    }
                    316 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy59 =
                                    unsafe {
                                        sqlite3_with_add(p_parse_1, core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy67 })
                                    }
                            };
                        }
                    }
                    317 => {
                        {
                            unsafe {
                                (*yymsp.offset(-2 as isize)).minor.yy59 =
                                    unsafe {
                                        sqlite3_with_add(p_parse_1,
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy59 },
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy67 })
                                    }
                            };
                        }
                    }
                    318 => {
                        {
                            { let _ = 0; };
                            unsafe {
                                sqlite3_window_chain(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy211 },
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy211 })
                            };
                            unsafe {
                                (*unsafe {
                                                    (*yymsp.offset(0 as isize)).minor.yy211
                                                }).p_next_win =
                                    unsafe { (*yymsp.offset(-2 as isize)).minor.yy211 }
                            };
                            yylhsminor.yy211 =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy211 };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    319 => {
                        {
                            if !(unsafe {
                                                (*yymsp.offset(-1 as isize)).minor.yy211
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                        (*yymsp.offset(-1 as isize)).minor.yy211
                                                    }).z_name =
                                        unsafe {
                                            sqlite3_db_str_n_dup(unsafe { (*p_parse_1).db },
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy0.z },
                                                unsafe { (*yymsp.offset(-4 as isize)).minor.yy0.n } as u64)
                                        }
                                };
                            }
                            yylhsminor.yy211 =
                                unsafe { (*yymsp.offset(-1 as isize)).minor.yy211 };
                        }
                        unsafe {
                            (*yymsp.offset(-4 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    320 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy211 =
                                    unsafe {
                                        sqlite3_window_assemble(p_parse_1,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy211 },
                                            unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    321 => {
                        {
                            yylhsminor.yy211 =
                                unsafe {
                                    sqlite3_window_assemble(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy211 },
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy14 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                        unsafe { &mut (*yymsp.offset(-5 as isize)).minor.yy0 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-5 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    322 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy211 =
                                    unsafe {
                                        sqlite3_window_assemble(p_parse_1,
                                            unsafe { (*yymsp.offset(0 as isize)).minor.yy211 },
                                            core::ptr::null_mut(),
                                            unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                            core::ptr::null_mut())
                                    }
                            };
                        }
                    }
                    323 => {
                        {
                            yylhsminor.yy211 =
                                unsafe {
                                    sqlite3_window_assemble(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy211 },
                                        core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy14 },
                                        unsafe { &mut (*yymsp.offset(-4 as isize)).minor.yy0 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-4 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    324 => {
                        {
                            yylhsminor.yy211 =
                                unsafe {
                                    sqlite3_window_assemble(p_parse_1,
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy211 },
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        unsafe { &mut (*yymsp.offset(-1 as isize)).minor.yy0 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-1 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    325 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy211 =
                                    unsafe {
                                        sqlite3_window_alloc(p_parse_1, 0, 91,
                                            core::ptr::null_mut(), 86, core::ptr::null_mut(), 0 as u8)
                                    }
                            };
                        }
                    }
                    326 => {
                        {
                            yylhsminor.yy211 =
                                unsafe {
                                    sqlite3_window_alloc(p_parse_1,
                                        unsafe { (*yymsp.offset(-2 as isize)).minor.yy144 },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy509.e_type },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy509.p_expr },
                                        86, core::ptr::null_mut(),
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy462 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-2 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    327 => {
                        {
                            yylhsminor.yy211 =
                                unsafe {
                                    sqlite3_window_alloc(p_parse_1,
                                        unsafe { (*yymsp.offset(-5 as isize)).minor.yy144 },
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy509.e_type },
                                        unsafe { (*yymsp.offset(-3 as isize)).minor.yy509.p_expr },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy509.e_type },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy509.p_expr },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy462 })
                                };
                        }
                        unsafe {
                            (*yymsp.offset(-5 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    329 => {
                        {
                            yylhsminor.yy509 =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy509 };
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy509 = yylhsminor.yy509
                        };
                    }
                    331 => {
                        {
                            yylhsminor.yy509 =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy509 };
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy509 = yylhsminor.yy509
                        };
                    }
                    330 => {
                        {
                            yylhsminor.yy509.e_type =
                                unsafe { (*yymsp.offset(-1 as isize)).major } as i32;
                            yylhsminor.yy509.p_expr = core::ptr::null_mut();
                        }
                        unsafe {
                            (*yymsp.offset(-1 as isize)).minor.yy509 = yylhsminor.yy509
                        };
                    }
                    332 => {
                        {
                            yylhsminor.yy509.e_type =
                                unsafe { (*yymsp.offset(-1 as isize)).major } as i32;
                            yylhsminor.yy509.p_expr = core::ptr::null_mut();
                        }
                        unsafe {
                            (*yymsp.offset(-1 as isize)).minor.yy509 = yylhsminor.yy509
                        };
                    }
                    334 => {
                        {
                            yylhsminor.yy509.e_type =
                                unsafe { (*yymsp.offset(-1 as isize)).major } as i32;
                            yylhsminor.yy509.p_expr = core::ptr::null_mut();
                        }
                        unsafe {
                            (*yymsp.offset(-1 as isize)).minor.yy509 = yylhsminor.yy509
                        };
                    }
                    333 => {
                        {
                            yylhsminor.yy509.e_type =
                                unsafe { (*yymsp.offset(0 as isize)).major } as i32;
                            yylhsminor.yy509.p_expr =
                                unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 };
                        }
                        unsafe {
                            (*yymsp.offset(-1 as isize)).minor.yy509 = yylhsminor.yy509
                        };
                    }
                    335 => {
                        {
                            unsafe {
                                (*yymsp.offset(1 as isize)).minor.yy462 = 0 as u8
                            };
                        }
                    }
                    336 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy462 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy462 }
                            };
                        }
                    }
                    337 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy462 =
                                    unsafe { (*yymsp.offset(-1 as isize)).major } as u8
                            };
                        }
                    }
                    338 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy462 =
                                    unsafe { (*yymsp.offset(-1 as isize)).major } as u8
                            };
                        }
                    }
                    339 => {
                        {
                            unsafe {
                                (*yymsp.offset(0 as isize)).minor.yy462 =
                                    unsafe { (*yymsp.offset(0 as isize)).major } as u8
                            };
                        }
                    }
                    340 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy211 =
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy211 }
                            };
                        }
                    }
                    341 => {
                        {
                            if !(unsafe {
                                                (*yymsp.offset(0 as isize)).minor.yy211
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                        (*yymsp.offset(0 as isize)).minor.yy211
                                                    }).p_filter =
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 }
                                };
                            } else {
                                unsafe {
                                    sqlite3_expr_delete(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 })
                                };
                            }
                            yylhsminor.yy211 =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy211 };
                        }
                        unsafe {
                            (*yymsp.offset(-1 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    342 => {
                        {
                            yylhsminor.yy211 =
                                unsafe { (*yymsp.offset(0 as isize)).minor.yy211 };
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    343 => {
                        {
                            yylhsminor.yy211 =
                                unsafe {
                                        sqlite3_db_malloc_zero(unsafe { (*p_parse_1).db },
                                            core::mem::size_of::<Window>() as u64)
                                    } as *mut Window;
                            if !(yylhsminor.yy211).is_null() {
                                unsafe { (*yylhsminor.yy211).e_frm_type = 167 as u8 };
                                unsafe {
                                    (*yylhsminor.yy211).p_filter =
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 }
                                };
                            } else {
                                unsafe {
                                    sqlite3_expr_delete(unsafe { (*p_parse_1).db },
                                        unsafe { (*yymsp.offset(0 as isize)).minor.yy454 })
                                };
                            }
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy211 = yylhsminor.yy211
                        };
                    }
                    344 => {
                        {
                            unsafe {
                                (*yymsp.offset(-3 as isize)).minor.yy211 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy211 }
                            };
                            { let _ = 0; };
                        }
                    }
                    345 => {
                        {
                            unsafe {
                                (*yymsp.offset(-1 as isize)).minor.yy211 =
                                    unsafe {
                                            sqlite3_db_malloc_zero(unsafe { (*p_parse_1).db },
                                                core::mem::size_of::<Window>() as u64)
                                        } as *mut Window
                            };
                            if !(unsafe {
                                                (*yymsp.offset(-1 as isize)).minor.yy211
                                            }).is_null() {
                                unsafe {
                                    (*unsafe {
                                                        (*yymsp.offset(-1 as isize)).minor.yy211
                                                    }).z_name =
                                        unsafe {
                                            sqlite3_db_str_n_dup(unsafe { (*p_parse_1).db },
                                                unsafe { (*yymsp.offset(0 as isize)).minor.yy0.z },
                                                unsafe { (*yymsp.offset(0 as isize)).minor.yy0.n } as u64)
                                        }
                                };
                            }
                        }
                    }
                    346 => {
                        {
                            unsafe {
                                (*yymsp.offset(-4 as isize)).minor.yy454 =
                                    unsafe { (*yymsp.offset(-1 as isize)).minor.yy454 }
                            };
                        }
                    }
                    347 => {
                        {
                            yylhsminor.yy454 =
                                token_expr(p_parse_1,
                                    unsafe { (*yymsp.offset(0 as isize)).major } as i32,
                                    unsafe { (*yymsp.offset(0 as isize)).minor.yy0 });
                            unsafe {
                                sqlite3_dequote_number(p_parse_1, yylhsminor.yy454)
                            };
                        }
                        unsafe {
                            (*yymsp.offset(0 as isize)).minor.yy454 = yylhsminor.yy454
                        };
                    }
                    _ => {
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
                    }
                }
            }
            { let _ = 0; };
            yygoto = yy_rule_info_lhs[yyruleno as usize] as i32;
            yysize = yy_rule_info_n_rhs[yyruleno as usize] as i32;
            yyact =
                yy_find_reduce_action(unsafe {
                        (*yymsp.offset(yysize as isize)).stateno
                    }, yygoto as u16);

            /// There are no SHIFTREDUCE actions on nonterminals because the table
            ///* generator has simplified them to pure REDUCE actions.
            { let _ = 0; };

            /// It is not possible for a REDUCE to be followed by an error
            { let _ = 0; };
            {
                let __n = yysize + 1;
                let __p = &mut yymsp;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            (*yyp_parser_1).yytos = yymsp;
            unsafe { (*yymsp).stateno = yyact as u16 };
            unsafe { (*yymsp).major = yygoto as u16 };
            return yyact;
        }
    }
}

///* Perform a shift action.
extern "C" fn yy_shift(yyp_parser_1: *mut YyParser, mut yy_new_state_1: u16,
    yy_major_1: u16, yy_minor_1: Token) -> () {
    unsafe {
        let mut yytos: *mut YyStackEntry = core::ptr::null_mut();
        {
            let __p = unsafe { &mut (*yyp_parser_1).yytos };
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        yytos = unsafe { (*yyp_parser_1).yytos };
        if yytos > unsafe { (*yyp_parser_1).yystack_end } {
            if yy_grow_stack(unsafe { &mut *yyp_parser_1 }) != 0 {
                {
                    let __p = unsafe { &mut (*yyp_parser_1).yytos };
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(-1) };
                    __t
                };
                yy_stack_overflow(yyp_parser_1);
                return;
            }
            yytos = unsafe { (*yyp_parser_1).yytos };
            { let _ = 0; };
        }
        if yy_new_state_1 as i32 > 599 {
            yy_new_state_1 += (1282 - 867) as u16;
        }
        unsafe { (*yytos).stateno = yy_new_state_1 };
        unsafe { (*yytos).major = yy_major_1 };
        unsafe { (*yytos).minor.yy0 = yy_minor_1 };
    }
}

///* The following is executed when the parser accepts
#[allow(unused_doc_comments)]
extern "C" fn yy_accept(yyp_parser_1: &mut YyParser) -> () {
    let p_parse: *mut Parse = (*yyp_parser_1).p_parse;
    { let _ = 0; };

    /// Here code is inserted which will be executed whenever the
    ///* parser accepts */
    ////*********** Begin %parse_accept code *****************************************/
    ////*********** End %parse_accept code ******************************************
    /// Suppress warning about unused %extra_argument variable
    ((*yyp_parser_1).p_parse = p_parse);
}

///* The following code executes when a syntax error first occurs.
#[allow(unused_doc_comments)]
extern "C" fn yy_syntax_error(yyp_parser_1: &mut YyParser, yymajor: i32,
    mut yyminor: Token) -> () {
    unsafe {
        let p_parse: *mut Parse = (*yyp_parser_1).p_parse;

        ///********* Begin %syntax_error code ***************************************
        { let _ = yymajor; };
        if unsafe { *yyminor.z.offset(0 as isize) } != 0 {
            parser_syntax_error(p_parse, &mut yyminor);
        } else {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"incomplete input".as_ptr() as *mut i8 as *const i8)
            };
        }

        ///********* End %syntax_error code *****************************************
        /// Suppress warning about unused %extra_argument variable
        ((*yyp_parser_1).p_parse = p_parse);
    }
}

/// The main parser program.
///* The first argument is a pointer to a structure obtained from
///* "sqlite3ParserAlloc" which describes the current state of the parser.
///* The second argument is the major token number.  The third is
///* the minor token.  The fourth optional argument is whatever the
///* user wants (and specified in the grammar) and is available for
///* use by the action routines.
///*
///* Inputs:
///* <ul>
///* <li> A pointer to the parser (an opaque structure.)
///* <li> The major token number.
///* <li> The minor token number.
///* <li> An option argument of a grammar-specified type.
///* </ul>
///*
///* Outputs:
///* None.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_parser(yyp: *mut (), yymajor: i32, yyminor: Token)
    -> () {
    unsafe {
        let mut yyminorunion: YYMINORTYPE = unsafe { core::mem::zeroed() };
        let mut yyact: u16 = 0 as u16;
        /// The parser action.
        let yyp_parser: *mut YyParser = yyp as *mut YyParser;
        /// The parser
        let p_parse: *mut Parse = unsafe { (*yyp_parser).p_parse };
        { let _ = 0; };
        yyact = unsafe { (*unsafe { (*yyp_parser).yytos }).stateno };
        loop {

            /// Exit by "break"
            { let _ = 0; };
            { let _ = 0; };
            yyact = yy_find_shift_action(yymajor as u16, yyact);
            if yyact as i32 >= 1282 {
                let yyruleno: u32 = (yyact as i32 - 1282) as u32;
                if yy_rule_info_n_rhs[yyruleno as usize] as i32 == 0 {
                    if unsafe { (*yyp_parser).yytos } >=
                            unsafe { (*yyp_parser).yystack_end } {
                        if yy_grow_stack(unsafe { &mut *yyp_parser }) != 0 {
                            yy_stack_overflow(yyp_parser);
                            break;
                        }
                    }
                }
                yyact =
                    yy_reduce(unsafe { &mut *yyp_parser }, yyruleno, yymajor,
                        yyminor, p_parse);
            } else if yyact as i32 <= 1278 {
                yy_shift(yyp_parser, yyact, yymajor as u16, yyminor);
                break;
            } else if yyact as i32 == 1280 {
                {
                    let __p = unsafe { &mut (*yyp_parser).yytos };
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(-1) };
                    __t
                };
                yy_accept(unsafe { &mut *yyp_parser });
                return;
            } else {
                { let _ = 0; };
                yyminorunion.yy0 = yyminor;

                /// If the YYNOERRORRECOVERY macro is defined, then do not attempt to
                ///* do any kind of error recovery.  Instead, simply invoke the syntax
                ///* error routine and continue going as if nothing had happened.
                ///*
                ///* Applications can set this macro (for example inside %include) if
                ///* they intend to abandon the parse upon the first syntax error seen.
                yy_syntax_error(unsafe { &mut *yyp_parser }, yymajor,
                    yyminor);
                yy_destructor(unsafe { &*yyp_parser }, yymajor as u16,
                    &yyminorunion);
                break;
            }
        }
        return;
    }
}

///* Return the fallback token corresponding to canonical token iToken, or
///* 0 if iToken has no fallback.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_parser_fallback(i_token: i32) -> i32 {
    { let _ = 0; };
    return yy_fallback[i_token as usize] as i32;
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
    fn sqlite3_with_delete(_: *mut Sqlite3, _: *mut With)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_expr_set_error_offset(_: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
    fn sqlite3_upsert_new(_: *mut Sqlite3, _: *mut ExprList, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_expr_set_height_and_flags(p_parse_1: *mut Parse, p: *mut Expr)
    -> ();
    fn sqlite3_vtab_finish_parse(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_vtab_begin_parse(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: i32)
    -> ();
    fn sqlite3_vtab_arg_init(_: *mut Parse)
    -> ();
    fn sqlite3_vtab_arg_extend(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_with_push(_: *mut Parse, _: *mut With, _: u8)
    -> *mut With;
    fn sqlite3_cte_new(_: *mut Parse, _: *mut Token, _: *mut ExprList,
    _: *mut Select, _: u8)
    -> *mut Cte;
    fn sqlite3_with_add(_: *mut Parse, _: *mut With, _: *mut Cte)
    -> *mut With;
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
    fn sqlite3_cte_delete(_: *mut Sqlite3, _: *mut Cte)
    -> ();
    fn sqlite3_with_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
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
    fn sqlite3_select_expr_height(_: *const Select)
    -> i32;
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
    -> i32;
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
