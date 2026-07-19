use super::*;
use crate::fts5_h::{Fts5Tokenizer, Fts5TokenizerV2, fts5_tokenizer};
use crate::sqlite3_h::{Sqlite3, Sqlite3Vtab};

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Global {
    pub(crate) _opaque: [u8; 0],
}

/// If a NEAR() clump or phrase may only match a specific set of columns, 
///* then an object of the following type is used to record the set of columns.
///* Each entry in the aiCol[] array is a column that may be matched.
///*
///* This object is used by fts5_expr.c and fts5_index.c.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Colset {
    pub(crate) n_col: i32,
    pub(crate) ai_col: [i32; 0],
}

///* An instance of the following structure encodes all information that can
///* be gleaned from the CREATE VIRTUAL TABLE statement.
///*
///* And all information loaded from the %_config table.
///*
///* nAutomerge:
///*   The minimum number of segments that an auto-merge operation should
///*   attempt to merge together. A value of 1 sets the object to use the 
///*   compile time default. Zero disables auto-merge altogether.
///*
///* bContentlessDelete:
///*   True if the contentless_delete option was present in the CREATE 
///*   VIRTUAL TABLE statement.
///*
///* zContent:
///*
///* zContentRowid:
///*   The value of the content_rowid= option, if one was specified. Or 
///*   the string "rowid" otherwise. This text is not quoted - if it is
///*   used as part of an SQL statement it needs to be quoted appropriately.
///*
///* zContentExprlist:
///*
///* pzErrmsg:
///*   This exists in order to allow the fts5_index.c module to return a 
///*   decent error message if it encounters a file-format version it does
///*   not understand.
///*
///* bColumnsize:
///*   True if the %_docsize table is created.
///*
///* bPrefixIndex:
///*   This is only used for debugging. If set to false, any prefix indexes
///*   are ignored. This value is configured using:
///*
///*       INSERT INTO tbl(tbl, rank) VALUES('prefix-index', $bPrefixIndex);
///*
///* bLocale:
///*   Set to true if locale=1 was specified when the table was created.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Config {
    pub(crate) db: *mut Sqlite3,
    pub(crate) p_global: *mut Fts5Global,
    pub(crate) z_db: *mut i8,
    pub(crate) z_name: *mut i8,
    pub(crate) n_col: i32,
    pub(crate) az_col: *mut *mut i8,
    pub(crate) ab_unindexed: *mut u8,
    pub(crate) n_prefix: i32,
    pub(crate) a_prefix: *mut i32,
    pub(crate) e_content: i32,
    pub(crate) b_contentless_delete: i32,
    pub(crate) b_contentless_unindexed: i32,
    pub(crate) z_content: *mut i8,
    pub(crate) z_content_rowid: *mut i8,
    pub(crate) b_columnsize: i32,
    pub(crate) b_tokendata: i32,
    pub(crate) b_locale: i32,
    pub(crate) e_detail: i32,
    pub(crate) z_content_exprlist: *mut i8,
    pub(crate) t: Fts5TokenizerConfig,
    pub(crate) b_lock: i32,
    pub(crate) i_version: i32,
    pub(crate) i_cookie: i32,
    pub(crate) pgsz: i32,
    pub(crate) n_automerge: i32,
    pub(crate) n_crisis_merge: i32,
    pub(crate) n_usermerge: i32,
    pub(crate) n_hash_size: i32,
    pub(crate) z_rank: *mut i8,
    pub(crate) z_rank_args: *mut i8,
    pub(crate) b_secure_delete: i32,
    pub(crate) n_delete_merge: i32,
    pub(crate) b_prefix_insttoken: i32,
    pub(crate) pz_errmsg: *mut *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5TokenizerConfig {
    pub(crate) p_tok: *mut Fts5Tokenizer,
    pub(crate) p_api2: *mut Fts5TokenizerV2,
    pub(crate) p_api1: *mut fts5_tokenizer,
    pub(crate) az_arg: *mut *const i8,
    pub(crate) n_arg: i32,
    pub(crate) e_pattern: i32,
    pub(crate) p_locale: *const i8,
    pub(crate) n_locale: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Buffer {
    pub(crate) p: *mut u8,
    pub(crate) n: i32,
    pub(crate) n_space: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5PoslistReader {
    pub(crate) a: *const u8,
    pub(crate) n: i32,
    pub(crate) i: i32,
    pub(crate) b_flag: u8,
    pub(crate) b_eof: u8,
    pub(crate) i_pos: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5PoslistWriter {
    pub(crate) i_prev: i64,
}

///***********************************************************************
///* Interface to code in fts5_index.c. fts5_index.c contains contains code
///* to access the data stored in the %_data table.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Index {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5IndexIter {
    pub(crate) i_rowid: i64,
    pub(crate) p_data: *const u8,
    pub(crate) n_data: i32,
    pub(crate) b_eof: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Table {
    pub(crate) base: Sqlite3Vtab,
    pub(crate) p_config: *mut Fts5Config,
    pub(crate) p_index: *mut Fts5Index,
}

///***********************************************************************
///* Interface to code in fts5_hash.c.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Hash {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Storage {
    pub(crate) _opaque: [u8; 0],
}

///***********************************************************************
///* Interface to code in fts5_expr.c.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Expr {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5ExprNode {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Parse {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Token {
    pub(crate) p: *const i8,
    pub(crate) n: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5ExprPhrase {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5ExprNearset {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5PoslistPopulator {
    pub(crate) _opaque: [u8; 0],
}
