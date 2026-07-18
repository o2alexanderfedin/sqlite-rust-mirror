use super::*;#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Global {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Colset {
    pub(crate) n_col: i32,
    pub(crate) ai_col: [i32; 0],
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Termset {
    pub(crate) _opaque: [u8; 0],
}
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
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Fts5Token {
    pub(crate) p: *const i8,
    pub(crate) n: i32,
}