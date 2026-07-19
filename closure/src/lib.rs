#![allow(unused_imports, dead_code)]

mod sqlite3_h;
mod sqlite3ext_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File,
    Sqlite3Filename, Sqlite3IndexConstraint, Sqlite3IndexInfo, Sqlite3Int64,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor, SqliteInt64,
};
use crate::sqlite3ext_h::Sqlite3ApiRoutines;

type DarwinSizeT = u64;

/// 
///* A closure virtual-table object
#[repr(C)]
#[derive(Copy, Clone)]
struct ClosureVtab {
    base: Sqlite3Vtab,
    z_db: *mut i8,
    z_self: *mut i8,
    z_table_name: *mut i8,
    z_id_column: *mut i8,
    z_parent_column: *mut i8,
    db: *mut Sqlite3,
    n_cursor: i32,
}

/// A closure cursor object
#[repr(C)]
#[derive(Copy, Clone)]
struct ClosureCursor {
    base: Sqlite3VtabCursor,
    p_vtab: *mut ClosureVtab,
    z_table_name: *mut i8,
    z_id_column: *mut i8,
    z_parent_column: *mut i8,
    p_current: *mut ClosureAvl,
    p_closure: *mut ClosureAvl,
}

///**************************************************************************
///* AVL Tree implementation
////
////*
///* Objects that want to be members of the AVL tree should embedded an
///* instance of this structure.
#[repr(C)]
#[derive(Copy, Clone)]
struct ClosureAvl {
    id: Sqlite3Int64,
    i_generation: i32,
    p_list: *mut ClosureAvl,
    p_before: *mut ClosureAvl,
    p_after: *mut ClosureAvl,
    p_up: *mut ClosureAvl,
    height: i16,
    imbalance: i16,
}

/// A queue of AVL nodes
#[repr(C)]
#[derive(Copy, Clone)]
struct ClosureQueue {
    p_first: *mut ClosureAvl,
    p_last: *mut ClosureAvl,
}

/// Recompute the closure_avl.height and closure_avl.imbalance fields for p.
///* Assume that the children of p have correct heights.
#[allow(unused_doc_comments)]
extern "C" fn closure_avl_recompute_height(p: &mut ClosureAvl) -> () {
    let h_before: i16 =
        if !((*p).p_before).is_null() {
                (unsafe { (*(*p).p_before).height }) as i32
            } else { 0 } as i16;
    let h_after: i16 =
        if !((*p).p_after).is_null() {
                (unsafe { (*(*p).p_after).height }) as i32
            } else { 0 } as i16;
    (*p).imbalance = (h_before as i32 - h_after as i32) as i16;

    /// -: pAfter higher.  +: pBefore higher
    ((*p).height =
        (if h_before as i32 > h_after as i32 {
                    h_before as i32
                } else { h_after as i32 } + 1) as i16);
}

///*     P                B
///*    / \              / \
///*   B   Z    ==>     X   P
///*  / \                  / \
///* X   Y                Y   Z
///*
extern "C" fn closure_avl_rotate_before(p_p_1: *mut ClosureAvl)
    -> *mut ClosureAvl {
    let p_b: *mut ClosureAvl = unsafe { (*p_p_1).p_before };
    let p_y: *mut ClosureAvl = unsafe { (*p_b).p_after };
    unsafe { (*p_b).p_up = unsafe { (*p_p_1).p_up } };
    unsafe { (*p_b).p_after = p_p_1 };
    unsafe { (*p_p_1).p_up = p_b };
    unsafe { (*p_p_1).p_before = p_y };
    if !(p_y).is_null() { unsafe { (*p_y).p_up = p_p_1 }; }
    closure_avl_recompute_height(unsafe { &mut *p_p_1 });
    closure_avl_recompute_height(unsafe { &mut *p_b });
    return p_b;
}

///*     P                A
///*    / \              / \
///*   X   A    ==>     P   Z
///*      / \          / \
///*     Y   Z        X   Y
///*
extern "C" fn closure_avl_rotate_after(p_p_1: *mut ClosureAvl)
    -> *mut ClosureAvl {
    let p_a: *mut ClosureAvl = unsafe { (*p_p_1).p_after };
    let p_y: *mut ClosureAvl = unsafe { (*p_a).p_before };
    unsafe { (*p_a).p_up = unsafe { (*p_p_1).p_up } };
    unsafe { (*p_a).p_before = p_p_1 };
    unsafe { (*p_p_1).p_up = p_a };
    unsafe { (*p_p_1).p_after = p_y };
    if !(p_y).is_null() { unsafe { (*p_y).p_up = p_p_1 }; }
    closure_avl_recompute_height(unsafe { &mut *p_p_1 });
    closure_avl_recompute_height(unsafe { &mut *p_a });
    return p_a;
}

///* Return a pointer to the pBefore or pAfter pointer in the parent
///* of p that points to p.  Or if p is the root node, return pp.
extern "C" fn closure_avl_from_ptr(p: *mut ClosureAvl,
    pp: *mut *mut ClosureAvl) -> *mut *mut ClosureAvl {
    let p_up: *mut ClosureAvl = unsafe { (*p).p_up };
    if p_up == core::ptr::null_mut() { return pp; }
    if unsafe { (*p_up).p_after } == p {
        return unsafe { &mut (*p_up).p_after };
    }
    return unsafe { &mut (*p_up).p_before };
}

///* Rebalance all nodes starting with p and working up to the root.
///* Return the new root.
extern "C" fn closure_avl_balance(mut p: *mut ClosureAvl) -> *mut ClosureAvl {
    let mut p_top: *mut ClosureAvl = p;
    let mut pp: *mut *mut ClosureAvl = core::ptr::null_mut();
    while !(p).is_null() {
        closure_avl_recompute_height(unsafe { &mut *p });
        if unsafe { (*p).imbalance } as i32 >= 2 {
            let p_b: *mut ClosureAvl = unsafe { (*p).p_before };
            if (unsafe { (*p_b).imbalance } as i32) < 0 {
                unsafe { (*p).p_before = closure_avl_rotate_after(p_b) };
            }
            pp = closure_avl_from_ptr(p, &mut p);
            p =
                {
                    unsafe { *pp = closure_avl_rotate_before(p) };
                    unsafe { *pp }
                };
        } else if unsafe { (*p).imbalance } as i32 <= -2 {
            let p_a: *mut ClosureAvl = unsafe { (*p).p_after };
            if unsafe { (*p_a).imbalance } as i32 > 0 {
                unsafe { (*p).p_after = closure_avl_rotate_before(p_a) };
            }
            pp = closure_avl_from_ptr(p, &mut p);
            p =
                {
                    unsafe { *pp = closure_avl_rotate_after(p) };
                    unsafe { *pp }
                };
        }
        p_top = p;
        p = unsafe { (*p).p_up };
    }
    return p_top;
}

/// Search the tree rooted at p for an entry with id.  Return a pointer
///* to the entry or return NULL.
extern "C" fn closure_avl_search(mut p: *mut ClosureAvl, id: Sqlite3Int64)
    -> *mut ClosureAvl {
    while !(p).is_null() && id != unsafe { (*p).id } {
        p =
            if id < unsafe { (*p).id } {
                unsafe { (*p).p_before }
            } else { unsafe { (*p).p_after } };
    }
    return p;
}

/// Find the first node (the one with the smallest key).
extern "C" fn closure_avl_first(mut p: *mut ClosureAvl) -> *mut ClosureAvl {
    if !(p).is_null() {
        while !(unsafe { (*p).p_before }).is_null() {
            p = unsafe { (*p).p_before };
        }
    }
    return p;
}

/// Return the node with the next larger key after p.
#[unsafe(no_mangle)]
pub extern "C" fn closure_avl_next(mut p: *mut ClosureAvl)
    -> *mut ClosureAvl {
    let mut p_prev: *mut ClosureAvl = core::ptr::null_mut();
    while !(p).is_null() && unsafe { (*p).p_after } == p_prev {
        p_prev = p;
        p = unsafe { (*p).p_up };
    }
    if !(p).is_null() && p_prev == core::ptr::null_mut() {
        p = closure_avl_first(unsafe { (*p).p_after });
    }
    return p;
}

/// Insert a new node pNew.  Return NULL on success.  If the key is not
///* unique, then do not perform the insert but instead leave pNew unchanged
///* and return a pointer to an existing node with the same key.
extern "C" fn closure_avl_insert(pp_head_1: &mut *mut ClosureAvl,
    p_new_1: *mut ClosureAvl) -> *mut ClosureAvl {
    let mut p: *mut ClosureAvl = *pp_head_1;
    if p == core::ptr::null_mut() {
        p = p_new_1;
        unsafe { (*p_new_1).p_up = core::ptr::null_mut() };
    } else {
        while !(p).is_null() {
            if unsafe { (*p_new_1).id } < unsafe { (*p).id } {
                if !(unsafe { (*p).p_before }).is_null() {
                    p = unsafe { (*p).p_before };
                } else {
                    unsafe { (*p).p_before = p_new_1 };
                    unsafe { (*p_new_1).p_up = p };
                    break;
                }
            } else if unsafe { (*p_new_1).id } > unsafe { (*p).id } {
                if !(unsafe { (*p).p_after }).is_null() {
                    p = unsafe { (*p).p_after };
                } else {
                    unsafe { (*p).p_after = p_new_1 };
                    unsafe { (*p_new_1).p_up = p };
                    break;
                }
            } else { return p; }
        }
    }
    unsafe { (*p_new_1).p_before = core::ptr::null_mut() };
    unsafe { (*p_new_1).p_after = core::ptr::null_mut() };
    unsafe { (*p_new_1).height = 1 as i16 };
    unsafe { (*p_new_1).imbalance = 0 as i16 };
    *pp_head_1 = closure_avl_balance(p);
    return core::ptr::null_mut();
}

/// Walk the tree can call xDestroy on each node
extern "C" fn closure_avl_destroy(p: *mut ClosureAvl,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ClosureAvl) -> ()>) -> () {
    if !(p).is_null() {
        closure_avl_destroy(unsafe { (*p).p_before }, x_destroy_1);
        closure_avl_destroy(unsafe { (*p).p_after }, x_destroy_1);
        unsafe { x_destroy_1.unwrap()(p) };
    }
}

///* Add a node to the end of the queue
extern "C" fn queue_push(p_queue_1: &mut ClosureQueue,
    p_node_1: *mut ClosureAvl) -> () {
    unsafe { (*p_node_1).p_list = core::ptr::null_mut() };
    if !((*p_queue_1).p_last).is_null() {
        unsafe { (*(*p_queue_1).p_last).p_list = p_node_1 };
    } else { (*p_queue_1).p_first = p_node_1; }
    (*p_queue_1).p_last = p_node_1;
}

///* Extract the oldest element (the front element) from the queue.
extern "C" fn queue_pull(p_queue_1: &mut ClosureQueue) -> *mut ClosureAvl {
    let p: *mut ClosureAvl = (*p_queue_1).p_first;
    if !(p).is_null() {
        (*p_queue_1).p_first = unsafe { (*p).p_list };
        if (*p_queue_1).p_first == core::ptr::null_mut() {
            (*p_queue_1).p_last = core::ptr::null_mut();
        }
    }
    return p;
}

///* This function converts an SQL quoted string into an unquoted string
///* and returns a pointer to a buffer allocated using sqlite3_malloc() 
///* containing the result. The caller should eventually free this buffer
///* using sqlite3_free.
///*
///* Examples:
///*
///*     "abc"   becomes   abc
///*     'xyz'   becomes   xyz
///*     [pqr]   becomes   pqr
///*     `mno`   becomes   mno
#[allow(unused_doc_comments)]
extern "C" fn closure_dequote(z_in_1: *const i8) -> *mut i8 {
    let mut n_in: Sqlite3Int64 = 0 as Sqlite3Int64;
    /// Size of input string, in bytes
    let mut z_out: *mut i8 = core::ptr::null_mut();

    /// Output (dequoted) string
    (n_in = unsafe { strlen(z_in_1) } as Sqlite3Int64);
    z_out =
        unsafe {
                sqlite3_malloc64((n_in + 1 as Sqlite3Int64) as Sqlite3Uint64)
            } as *mut i8;
    if !(z_out).is_null() {
        let mut q: i8 = unsafe { *z_in_1.offset(0 as isize) } as i8;
        if q as i32 != '[' as i32 && q as i32 != '\'' as i32 &&
                    q as i32 != '\"' as i32 && q as i32 != '`' as i32 {
            unsafe {
                memcpy(z_out as *mut (), z_in_1 as *const (),
                    (n_in + 1 as Sqlite3Int64) as u64)
            };
        } else {
            let mut i_out: i32 = 0;
            /// Index of next byte to write to output
            let mut i_in: i32 = 0;
            if q as i32 == '[' as i32 { q = ']' as i32 as i8; }
            {
                i_in = 1;
                '__b5: loop {
                    if !((i_in as Sqlite3Int64) < n_in) { break '__b5; }
                    '__c5: loop {
                        if unsafe { *z_in_1.offset(i_in as isize) } as i32 ==
                                q as i32 {
                            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                        }
                        unsafe {
                            *z_out.offset({
                                                let __p = &mut i_out;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = unsafe { *z_in_1.offset(i_in as isize) } as i8
                        };
                        break '__c5;
                    }
                    { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { *z_out.offset(i_out as isize) = 0 as i8 };
        }
        if !(unsafe { strlen(z_out as *const i8) } as i32 as Sqlite3Int64 <=
                                n_in) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"closureDequote".as_ptr() as *const i8,
                    c"closure.c".as_ptr() as *mut i8 as *const i8, 458,
                    c"(int)strlen(zOut)<=nIn".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    return z_out;
}

///* Deallocate an closure_vtab object
extern "C" fn closure_free(p: *mut ClosureVtab) -> () {
    if !(p).is_null() {
        unsafe { sqlite3_free(unsafe { (*p).z_db } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_self } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_table_name } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_id_column } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_parent_column } as *mut ()) };
        unsafe {
            memset(p as *mut (), 0,
                core::mem::size_of::<ClosureVtab>() as u64)
        };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

///* xDisconnect/xDestroy method for the closure module.
extern "C" fn closure_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut ClosureVtab = p_vtab_1 as *mut ClosureVtab;
    if !(unsafe { (*p).n_cursor } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"closureDisconnect".as_ptr() as *const i8,
                c"closure.c".as_ptr() as *mut i8 as *const i8, 483,
                c"p->nCursor==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    closure_free(p);
    return 0;
}

///* Check to see if the argument is of the form:
///*
///*       KEY = VALUE
///*
///* If it is, return a pointer to the first character of VALUE.
///* If not, return NULL.  Spaces around the = are ignored.
extern "C" fn closure_value_of_key(z_key_1: *const i8, z_str_1: *const i8)
    -> *const i8 {
    let n_key: i32 = unsafe { strlen(z_key_1) } as i32;
    let n_str: i32 = unsafe { strlen(z_str_1) } as i32;
    let mut i: i32 = 0;
    if n_str < n_key + 1 { return core::ptr::null(); }
    if unsafe {
                memcmp(z_str_1 as *const (), z_key_1 as *const (),
                    n_key as u64)
            } != 0 {
        return core::ptr::null();
    }
    {
        i = n_key;
        '__b6: loop {
            if !(unsafe {
                                isspace(unsafe { *z_str_1.offset(i as isize) } as u8 as i32)
                            } != 0) {
                break '__b6;
            }
            '__c6: loop { break '__c6; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if unsafe { *z_str_1.offset(i as isize) } as i32 != '=' as i32 {
        return core::ptr::null();
    }
    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    while unsafe {
                isspace(unsafe { *z_str_1.offset(i as isize) } as u8 as i32)
            } != 0 {
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    }
    return unsafe { z_str_1.offset(i as isize) };
}

///* xConnect/xCreate method for the closure module. Arguments are:
///*
///*   argv[0]    -> module name  ("transitive_closure")
///*   argv[1]    -> database name
///*   argv[2]    -> table name
///*   argv[3...] -> arguments
#[allow(unused_doc_comments)]
extern "C" fn closure_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut p_new: *mut ClosureVtab = core::ptr::null_mut();
    /// New virtual table
    let mut z_db: *const i8 = core::ptr::null();
    let mut z_val: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s9:
            {
            match __state {
                0 => { rc = 0; __state = 3; }
                2 => { closure_free(p_new); __state = 55; }
                3 => { p_new = core::ptr::null_mut(); __state = 4; }
                4 => {
                    z_db = unsafe { *argv.offset(1 as isize) };
                    __state = 5;
                }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { { let _ = p_aux_1; }; __state = 8; }
                8 => {
                    unsafe { *pp_vtab_1 = core::ptr::null_mut() };
                    __state = 9;
                }
                9 => {
                    p_new =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<ClosureVtab>() as
                                        Sqlite3Uint64)
                            } as *mut ClosureVtab;
                    __state = 10;
                }
                10 => {
                    if p_new == core::ptr::null_mut() {
                        __state = 12;
                    } else { __state = 11; }
                }
                11 => { rc = 7; __state = 13; }
                12 => { return 7; }
                13 => {
                    unsafe {
                        memset(p_new as *mut (), 0,
                            core::mem::size_of::<ClosureVtab>() as u64)
                    };
                    __state = 14;
                }
                14 => { unsafe { (*p_new).db = db }; __state = 15; }
                15 => {
                    unsafe {
                        (*p_new).z_db =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    z_db)
                            }
                    };
                    __state = 16;
                }
                16 => {
                    if unsafe { (*p_new).z_db } == core::ptr::null_mut() {
                        __state = 18;
                    } else { __state = 17; }
                }
                17 => {
                    unsafe {
                        (*p_new).z_self =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(2 as isize) })
                            }
                    };
                    __state = 19;
                }
                18 => { __state = 2; }
                19 => {
                    if unsafe { (*p_new).z_self } == core::ptr::null_mut() {
                        __state = 21;
                    } else { __state = 20; }
                }
                20 => { i = 3; __state = 23; }
                21 => { __state = 2; }
                22 => {
                    rc =
                        unsafe {
                            sqlite3_declare_vtab(db,
                                c"CREATE TABLE x(id,depth,root HIDDEN,tablename HIDDEN,idcolumn HIDDEN,parentcolumn HIDDEN)".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    __state = 50;
                }
                23 => { if i < argc { __state = 24; } else { __state = 22; } }
                24 => {
                    z_val =
                        closure_value_of_key(c"tablename".as_ptr() as *mut i8 as
                                *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 26;
                }
                25 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 23;
                }
                26 => {
                    if !(z_val).is_null() {
                        __state = 28;
                    } else { __state = 27; }
                }
                27 => {
                    z_val =
                        closure_value_of_key(c"idcolumn".as_ptr() as *mut i8 as
                                *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 33;
                }
                28 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_table_name } as *mut ())
                    };
                    __state = 29;
                }
                29 => {
                    unsafe { (*p_new).z_table_name = closure_dequote(z_val) };
                    __state = 30;
                }
                30 => {
                    if unsafe { (*p_new).z_table_name } == core::ptr::null_mut()
                        {
                        __state = 32;
                    } else { __state = 31; }
                }
                31 => { __state = 25; }
                32 => { __state = 2; }
                33 => {
                    if !(z_val).is_null() {
                        __state = 35;
                    } else { __state = 34; }
                }
                34 => {
                    z_val =
                        closure_value_of_key(c"parentcolumn".as_ptr() as *mut i8 as
                                *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 40;
                }
                35 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_id_column } as *mut ())
                    };
                    __state = 36;
                }
                36 => {
                    unsafe { (*p_new).z_id_column = closure_dequote(z_val) };
                    __state = 37;
                }
                37 => {
                    if unsafe { (*p_new).z_id_column } == core::ptr::null_mut()
                        {
                        __state = 39;
                    } else { __state = 38; }
                }
                38 => { __state = 25; }
                39 => { __state = 2; }
                40 => {
                    if !(z_val).is_null() {
                        __state = 42;
                    } else { __state = 41; }
                }
                41 => {
                    unsafe {
                        *pz_err_1 =
                            unsafe {
                                sqlite3_mprintf(c"unrecognized argument: [%s]\n".as_ptr() as
                                            *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                            }
                    };
                    __state = 47;
                }
                42 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_parent_column } as *mut ())
                    };
                    __state = 43;
                }
                43 => {
                    unsafe {
                        (*p_new).z_parent_column = closure_dequote(z_val)
                    };
                    __state = 44;
                }
                44 => {
                    if unsafe { (*p_new).z_parent_column } ==
                            core::ptr::null_mut() {
                        __state = 46;
                    } else { __state = 45; }
                }
                45 => { __state = 25; }
                46 => { __state = 2; }
                47 => { closure_free(p_new); __state = 48; }
                48 => {
                    unsafe { *pp_vtab_1 = core::ptr::null_mut() };
                    __state = 49;
                }
                49 => { return 1; }
                50 => { if rc != 0 { __state = 52; } else { __state = 51; } }
                51 => {
                    unsafe { *pp_vtab_1 = unsafe { &mut (*p_new).base } };
                    __state = 53;
                }
                52 => { closure_free(p_new); __state = 51; }
                53 => { return rc; }
                54 => { __state = 2; }
                55 => { return rc; }
                _ => {}
            }
        }
    }

    /// Return code
    /// New virtual table
    unreachable!();
}

///* Open a new closure cursor.
extern "C" fn closure_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p: *mut ClosureVtab = p_v_tab_1 as *mut ClosureVtab;
    let mut p_cur: *mut ClosureCursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<ClosureCursor>() as
                        Sqlite3Uint64)
            } as *mut ClosureCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<ClosureCursor>() as u64)
    };
    unsafe { (*p_cur).p_vtab = p };
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    {
        let __p = unsafe { &mut (*p).n_cursor };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return 0;
}

///* Wrapper around sqlite3_free
extern "C" fn closure_mem_free(p: *mut ClosureAvl) -> () {
    unsafe { sqlite3_free(p as *mut ()) };
}

///* Free up all the memory allocated by a cursor.  Set it rLimit to 0
///* to indicate that it is at EOF.
extern "C" fn closure_clear_cursor(p_cur_1: &mut ClosureCursor) -> () {
    closure_avl_destroy((*p_cur_1).p_closure, Some(closure_mem_free));
    unsafe { sqlite3_free((*p_cur_1).z_table_name as *mut ()) };
    unsafe { sqlite3_free((*p_cur_1).z_id_column as *mut ()) };
    unsafe { sqlite3_free((*p_cur_1).z_parent_column as *mut ()) };
    (*p_cur_1).z_table_name = core::ptr::null_mut();
    (*p_cur_1).z_id_column = core::ptr::null_mut();
    (*p_cur_1).z_parent_column = core::ptr::null_mut();
    (*p_cur_1).p_current = core::ptr::null_mut();
    (*p_cur_1).p_closure = core::ptr::null_mut();
}

///* Close a closure cursor.
extern "C" fn closure_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut ClosureCursor = cur as *mut ClosureCursor;
    closure_clear_cursor(unsafe { &mut *p_cur });
    {
        let __p = unsafe { &mut (*unsafe { (*p_cur).p_vtab }).n_cursor };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}

///* Advance a cursor to its next row of output
extern "C" fn closure_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut ClosureCursor = cur as *mut ClosureCursor;
    unsafe {
        (*p_cur).p_current = closure_avl_next(unsafe { (*p_cur).p_current })
    };
    return 0;
}

///* Allocate and insert a node
extern "C" fn closure_insert_node(p_queue_1: *mut ClosureQueue,
    p_cur_1: &mut ClosureCursor, id: Sqlite3Int64, i_generation_1: i32)
    -> i32 {
    let p_new: *mut ClosureAvl =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<ClosureAvl>() as
                        Sqlite3Uint64)
            } as *mut ClosureAvl;
    if p_new == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_new as *mut (), 0, core::mem::size_of::<ClosureAvl>() as u64)
    };
    unsafe { (*p_new).id = id };
    unsafe { (*p_new).i_generation = i_generation_1 };
    closure_avl_insert(&mut (*p_cur_1).p_closure, p_new);
    queue_push(unsafe { &mut *p_queue_1 }, p_new);
    return 0;
}

///* Called to "rewind" a cursor back to the beginning so that
///* it starts its output over again.  Always called at least once
///* prior to any closureColumn, closureRowid, or closureEof call.
///*
///* This routine actually computes the closure.
///*
///* See the comment at the beginning of closureBestIndex() for a 
///* description of the meaning of idxNum.  The idxStr parameter is
///* not used.
#[allow(unused_doc_comments)]
extern "C" fn closure_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut ClosureCursor = p_vtab_cursor_1 as *mut ClosureCursor;
    let p_vtab: *mut ClosureVtab = unsafe { (*p_cur).p_vtab };
    let mut i_root: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut mx_gen: i32 = 999999999;
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_avl: *const ClosureAvl = core::ptr::null();
    let mut rc: i32 = 0;
    let mut z_table_name: *const i8 =
        unsafe { (*p_vtab).z_table_name } as *const i8;
    let mut z_id_column: *const i8 =
        unsafe { (*p_vtab).z_id_column } as *const i8;
    let mut z_parent_column: *const i8 =
        unsafe { (*p_vtab).z_parent_column } as *const i8;
    let mut s_queue: ClosureQueue = unsafe { core::mem::zeroed() };
    { let _ = idx_str_1; };

    /// Unused parameter
    { let _ = argc; };

    /// Unused parameter
    /// Unused parameter
    closure_clear_cursor(unsafe { &mut *p_cur });
    unsafe {
        memset(&raw mut s_queue as *mut (), 0,
            core::mem::size_of::<ClosureQueue>() as u64)
    };
    if idx_num_1 & 1 == 0 {

        /// No root=$root in the WHERE clause.  Return an empty set
        return 0;
    }
    i_root =
        unsafe { sqlite3_value_int64(unsafe { *argv.offset(0 as isize) }) };
    if idx_num_1 & 240 != 0 {
        mx_gen =
            unsafe {
                sqlite3_value_int(unsafe {
                        *argv.offset((idx_num_1 >> 4 & 15) as isize)
                    })
            };
        if idx_num_1 & 2 != 0 {
            { let __p = &mut mx_gen; let __t = *__p; *__p -= 1; __t };
        }
    }
    if idx_num_1 & 3840 != 0 {
        z_table_name =
            unsafe {
                    sqlite3_value_text(unsafe {
                            *argv.offset((idx_num_1 >> 8 & 15) as isize)
                        })
                } as *const i8;
        unsafe {
            (*p_cur).z_table_name =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_table_name)
                }
        };
    }
    if idx_num_1 & 61440 != 0 {
        z_id_column =
            unsafe {
                    sqlite3_value_text(unsafe {
                            *argv.offset((idx_num_1 >> 12 & 15) as isize)
                        })
                } as *const i8;
        unsafe {
            (*p_cur).z_id_column =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_id_column)
                }
        };
    }
    if idx_num_1 & 983040 != 0 {
        z_parent_column =
            unsafe {
                    sqlite3_value_text(unsafe {
                            *argv.offset((idx_num_1 >> 16 & 15) as isize)
                        })
                } as *const i8;
        unsafe {
            (*p_cur).z_parent_column =
                unsafe {
                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                        z_parent_column)
                }
        };
    }
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT \"%w\".\"%w\" FROM \"%w\" WHERE \"%w\".\"%w\"=?1".as_ptr()
                        as *mut i8 as *const i8, z_table_name, z_id_column,
                z_table_name, z_table_name, z_parent_column)
        };
    if z_sql == core::ptr::null_mut() {
        return 7;
    } else {
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p_vtab).db },
                    z_sql as *const i8, -1, &mut p_stmt, core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if rc != 0 {
            unsafe {
                sqlite3_free(unsafe { (*p_vtab).base.z_err_msg } as *mut ())
            };
            unsafe {
                (*p_vtab).base.z_err_msg =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(unsafe { (*p_vtab).db }) })
                    }
            };
            return rc;
        }
    }
    if rc == 0 {
        rc =
            closure_insert_node(&mut s_queue, unsafe { &mut *p_cur }, i_root,
                0);
    }
    while { p_avl = queue_pull(&mut s_queue); p_avl } != core::ptr::null_mut()
        {
        if unsafe { (*p_avl).i_generation } >= mx_gen { continue; }
        unsafe { sqlite3_bind_int64(p_stmt, 1, unsafe { (*p_avl).id }) };
        while rc == 0 && unsafe { sqlite3_step(p_stmt) } == 100 {
            if unsafe { sqlite3_column_type(p_stmt, 0) } == 1 {
                let i_new: Sqlite3Int64 =
                    unsafe { sqlite3_column_int64(p_stmt, 0) };
                if closure_avl_search(unsafe { (*p_cur).p_closure }, i_new) ==
                        core::ptr::null_mut() {
                    rc =
                        closure_insert_node(&mut s_queue, unsafe { &mut *p_cur },
                            i_new, unsafe { (*p_avl).i_generation } + 1);
                }
            }
        }
        unsafe { sqlite3_reset(p_stmt) };
    }
    unsafe { sqlite3_finalize(p_stmt) };
    if rc == 0 {
        unsafe {
            (*p_cur).p_current =
                closure_avl_first(unsafe { (*p_cur).p_closure })
        };
    }
    return rc;
}

///* Only the word and distance columns have values.  All other columns
///* return NULL
extern "C" fn closure_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const ClosureCursor =
        cur as *mut ClosureCursor as *const ClosureCursor;
    '__s12:
        {
        match i {
            0 => {
                {
                    unsafe {
                        sqlite3_result_int64(ctx,
                            unsafe { (*unsafe { (*p_cur).p_current }).id })
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe { (*unsafe { (*p_cur).p_current }).i_generation })
                    };
                    break '__s12;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s12; }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_table_name }).is_null() {
                                    unsafe { (*p_cur).z_table_name }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_table_name }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_id_column }).is_null() {
                                    unsafe { (*p_cur).z_id_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_id_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_parent_column }).is_null() {
                                    unsafe { (*p_cur).z_parent_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_parent_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe { (*unsafe { (*p_cur).p_current }).i_generation })
                    };
                    break '__s12;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s12; }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_table_name }).is_null() {
                                    unsafe { (*p_cur).z_table_name }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_table_name }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_id_column }).is_null() {
                                    unsafe { (*p_cur).z_id_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_id_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_parent_column }).is_null() {
                                    unsafe { (*p_cur).z_parent_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_parent_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
            }
            2 => {
                { unsafe { sqlite3_result_null(ctx) }; break '__s12; }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_table_name }).is_null() {
                                    unsafe { (*p_cur).z_table_name }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_table_name }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_id_column }).is_null() {
                                    unsafe { (*p_cur).z_id_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_id_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_parent_column }).is_null() {
                                    unsafe { (*p_cur).z_parent_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_parent_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
            }
            3 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_table_name }).is_null() {
                                    unsafe { (*p_cur).z_table_name }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_table_name }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_id_column }).is_null() {
                                    unsafe { (*p_cur).z_id_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_id_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_parent_column }).is_null() {
                                    unsafe { (*p_cur).z_parent_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_parent_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
            }
            4 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_id_column }).is_null() {
                                    unsafe { (*p_cur).z_id_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_id_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_parent_column }).is_null() {
                                    unsafe { (*p_cur).z_parent_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_parent_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
            }
            5 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            if !(unsafe { (*p_cur).z_parent_column }).is_null() {
                                    unsafe { (*p_cur).z_parent_column }
                                } else {
                                    unsafe { (*unsafe { (*p_cur).p_vtab }).z_parent_column }
                                } as *const i8, -1,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__s12;
                }
            }
            _ => {}
        }
    }
    return 0;
}

///* The rowid.  For the closure table, this is the same as the "id" column.
extern "C" fn closure_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const ClosureCursor =
        cur as *mut ClosureCursor as *const ClosureCursor;
    unsafe { *p_rowid_1 = unsafe { (*unsafe { (*p_cur).p_current }).id } };
    return 0;
}

///* EOF indicator
extern "C" fn closure_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const ClosureCursor =
        cur as *mut ClosureCursor as *const ClosureCursor;
    return (unsafe { (*p_cur).p_current } == core::ptr::null_mut()) as i32;
}

///* Search for terms of these forms:
///*
///*   (A)    root = $root
///*   (B1)   depth < $depth
///*   (B2)   depth <= $depth
///*   (B3)   depth = $depth
///*   (C)    tablename = $tablename
///*   (D)    idcolumn = $idcolumn
///*   (E)    parentcolumn = $parentcolumn
///*
///* 
///*
///*   idxNum       meaning
///*   ----------   ------------------------------------------------------
///*   0x00000001   Term of the form (A) found
///*   0x00000002   The term of bit-2 is like (B1)
///*   0x000000f0   Index in filter.argv[] of $depth.  0 if not used.
///*   0x00000f00   Index in filter.argv[] of $tablename.  0 if not used.
///*   0x0000f000   Index in filter.argv[] of $idcolumn.  0 if not used
///*   0x000f0000   Index in filter.argv[] of $parentcolumn.  0 if not used.
///*
///* There must be a term of type (A).  If there is not, then the index type
///* is 0 and the query will return an empty set.
#[allow(unused_doc_comments)]
extern "C" fn closure_best_index(p_tab_1: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i_plan: i32 = 0;
    let mut i: i32 = 0;
    let mut idx: i32 = 1;
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();
    let p_vtab: *const ClosureVtab =
        p_tab_1 as *mut ClosureVtab as *const ClosureVtab;
    let mut r_cost: f64 = 10000000.0;
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b13: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b13;
            }
            '__c13: loop {
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c13;
                }
                if i_plan & 1 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 2 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 1;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = 1
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                    r_cost /= 100.0;
                }
                if i_plan & 240 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 1 &&
                        (unsafe { (*p_constraint).op } as i32 == 16 ||
                                unsafe { (*p_constraint).op } as i32 == 8 ||
                            unsafe { (*p_constraint).op } as i32 == 2) {
                    i_plan |= idx << 4;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = { let __p = &mut idx; *__p += 1; *__p }
                    };
                    if unsafe { (*p_constraint).op } as i32 == 16 {
                        i_plan |= 2;
                    }
                    r_cost /= 5.0;
                }
                if i_plan & 3840 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 3 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= idx << 8;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = { let __p = &mut idx; *__p += 1; *__p }
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                    r_cost /= 5.0;
                }
                if i_plan & 61440 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 4 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= idx << 12;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = { let __p = &mut idx; *__p += 1; *__p }
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                }
                if i_plan & 983040 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 5 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= idx << 16;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = { let __p = &mut idx; *__p += 1; *__p }
                    };
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                }
                break '__c13;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_constraint;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if unsafe { (*p_vtab).z_table_name } == core::ptr::null_mut() &&
                    i_plan & 3840 == 0 ||
                unsafe { (*p_vtab).z_id_column } == core::ptr::null_mut() &&
                    i_plan & 61440 == 0 ||
            unsafe { (*p_vtab).z_parent_column } == core::ptr::null_mut() &&
                i_plan & 983040 == 0 {

        /// All of tablename, idcolumn, and parentcolumn must be specified
        ///* in either the CREATE VIRTUAL TABLE or in the WHERE clause constraints
        ///* or else the result is an empty set.
        (i_plan = 0);
    }
    if i_plan & 1 == 0 {

        /// If there is no usable "root=?" term, then set the index-type to 0.
        ///* Also clear any argvIndex variables already set. This is necessary
        ///* to prevent the core from throwing an "xBestIndex malfunction error"
        ///* error (because the argvIndex values are not contiguously assigned
        ///* starting from 1).
        (r_cost *= 1e30);
        {
            i = 0;
            '__b14: loop {
                if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                    break '__b14;
                }
                '__c14: loop {
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).argv_index = 0
                    };
                    break '__c14;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_constraint;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        i_plan = 0;
    }
    unsafe { (*p_idx_info_1).idx_num = i_plan };
    if unsafe { (*p_idx_info_1).n_order_by } == 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 0 &&
            unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } as i32 == 0 {
        unsafe { (*p_idx_info_1).order_by_consumed = 1 };
    }
    unsafe { (*p_idx_info_1).estimated_cost = r_cost };
    return 0;
}

///* A virtual table module that implements the "transitive_closure".
static mut closure_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: Some(closure_connect),
        x_connect: Some(closure_connect),
        x_best_index: Some(closure_best_index),
        x_disconnect: Some(closure_disconnect),
        x_destroy: Some(closure_disconnect),
        x_open: Some(closure_open),
        x_close: Some(closure_close),
        x_filter: Some(closure_filter),
        x_next: Some(closure_next),
        x_eof: Some(closure_eof),
        x_column: Some(closure_column),
        x_rowid: Some(closure_rowid),
        x_update: None,
        x_begin: None,
        x_sync: None,
        x_commit: None,
        x_rollback: None,
        x_find_function: None,
        x_rename: None,
        x_savepoint: None,
        x_release: None,
        x_rollback_to: None,
        x_shadow_name: None,
        x_integrity: None,
    };

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_closure_init(db: *const Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    let rc: i32 = 0;
    { let _ = p_api_1; };
    { let _ = pz_err_msg_1; };

    /// SQLITE_TEST && !SQLITE_OMIT_VIRTUALTABLE
    return rc;
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
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn isspace(_c: i32)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
