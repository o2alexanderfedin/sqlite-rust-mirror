use super::*;
use crate::sqlite3_h::{Sqlite3Int64, Sqlite3Value};

/// An instance of the BtreePayload object describes the content of a single
///* entry in either an index or table btree.
///*
///* Index btrees (used for indexes and also WITHOUT ROWID tables) contain
///* an arbitrary key and no data.  These btrees have pKey,nKey set to the
///* key and the pData,nData,nZero fields are uninitialized.  The aMem,nMem
///* fields give an array of Mem objects that are a decomposition of the key.
///* The nMem field might be zero, indicating that no decomposition is available.
///*
///* Table btrees (used for rowid tables) contain an integer rowid used as
///* the key and passed in the nKey field.  The pKey field is zero.  
///* pData,nData hold the content of the new entry.  nZero extra zero bytes
///* are appended to the end of the content when constructing the entry.
///* The aMem,nMem fields are uninitialized for table btrees.
///*
///* Field usage summary:
///*
///*               Table BTrees                   Index Btrees
///*
///*   pKey        always NULL                    encoded key
///*   nKey        the ROWID                      length of pKey
///*   pData       data                           not used
///*   aMem        not used                       decomposed key value
///*   nMem        not used                       entries in aMem
///*   nData       length of pData                not used
///*   nZero       extra zeros after pData        not used
///*
///* This object is used to pass information into sqlite3BtreeInsert().  The
///* same information used to be passed as five separate parameters.  But placing
///* the information into this object helps to keep the interface more 
///* organized and understandable, and it also helps the resulting code to
///* run a little faster by using fewer registers for parameter passing.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtreePayload {
    pub(crate) p_key: *const (),
    pub(crate) n_key: Sqlite3Int64,
    pub(crate) p_data: *const (),
    pub(crate) a_mem: *mut Sqlite3Value,
    pub(crate) n_mem: u16,
    pub(crate) n_data: i32,
    pub(crate) n_zero: i32,
}
