use super::*;

/// A complete hash table is an instance of the following structure.
///* The internals of this structure are intended to be opaque -- client
///* code should not attempt to access or modify the fields of this structure
///* directly.  Change this structure only by using the routines below.
///* However, some of the "procedures" and "functions" for modifying and
///* accessing this structure are really macros, so we can't really make
///* this structure opaque.
///*
///* All elements of the hash table are on a single doubly-linked list.
///* Hash.first points to the head of this list.
///*
///* There are Hash.htsize buckets.  Each bucket points to a spot in
///* the global doubly-linked list.  The contents of the bucket are the
///* element pointed to plus the next _ht.count-1 elements in the list.
///*
///* Hash.htsize and Hash.ht may be zero.  In that case lookup is done
///* by a linear search of the global list.  For small tables, the 
///* Hash.ht table is never allocated because if there are few elements
///* in the table, it is faster to do a linear search than to manage
///* the hash table.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Hash {
    pub(crate) htsize: u32,
    pub(crate) count: u32,
    pub(crate) first: *mut HashElem,
    pub(crate) ht: *mut Ht,
}

/// Each element in the hash table is an instance of the following 
///* structure.  All elements are stored on a single doubly-linked list.
///*
///* Again, this structure is intended to be opaque, but it can't really
///* be opaque because it is used by macros.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct HashElem {
    pub(crate) next: *mut HashElem,
    pub(crate) prev: *mut HashElem,
    pub(crate) data: *mut (),
    pub(crate) p_key: *const i8,
    pub(crate) h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Ht {
    pub(crate) count: u32,
    pub(crate) chain: *mut HashElem,
}
