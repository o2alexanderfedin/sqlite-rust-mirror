use super::*;

///* An instance of the sqlite3_recover object represents a recovery
///* operation in progress.
///*
///* Constructors:
///*
///*    sqlite3_recover_init()
///*    sqlite3_recover_init_sql()
///*
///* Destructor:
///*
///*    sqlite3_recover_finish()
///*
///* Methods:
///*
///*    sqlite3_recover_config()
///*    sqlite3_recover_errcode()
///*    sqlite3_recover_errmsg()
///*    sqlite3_recover_run()
///*    sqlite3_recover_step()
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Recover {
    pub(crate) _opaque: [u8; 0],
}
