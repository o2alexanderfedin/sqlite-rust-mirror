use super::*;#[repr(C)]
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