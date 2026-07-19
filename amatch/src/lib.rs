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
///* A amatch virtual-table object
#[repr(C)]
#[derive(Copy, Clone)]
struct AmatchVtab {
    base: Sqlite3Vtab,
    z_class_name: *mut i8,
    z_db: *mut i8,
    z_self: *mut i8,
    z_cost_tab: *mut i8,
    z_vocab_tab: *mut i8,
    z_vocab_word: *mut i8,
    z_vocab_lang: *mut i8,
    p_rule: *mut AmatchRule,
    r_ins: AmatchCost,
    r_del: AmatchCost,
    r_sub: AmatchCost,
    db: *mut Sqlite3,
    p_v_check: *mut Sqlite3Stmt,
    n_cursor: i32,
}

///* Each transformation rule is stored as an instance of this object.
///* All rules are kept on a linked list sorted by rCost.
#[repr(C)]
#[derive(Copy, Clone)]
struct AmatchRule {
    p_next: *mut AmatchRule,
    z_from: *mut i8,
    r_cost: AmatchCost,
    i_lang: AmatchLangid,
    n_from: AmatchLen,
    n_to: AmatchLen,
    z_to: [i8; 4],
}

///* Various types.
///*
///* amatch_cost is the "cost" of an edit operation.
///*
///* amatch_len is the length of a matching string.  
///*
///* amatch_langid is an ruleset identifier.
type AmatchCost = i32;

type AmatchLangid = i32;

type AmatchLen = i8;

/// A amatch cursor object
#[repr(C)]
#[derive(Copy, Clone)]
struct AmatchCursor {
    base: Sqlite3VtabCursor,
    i_rowid: Sqlite3Int64,
    i_lang: AmatchLangid,
    r_limit: AmatchCost,
    n_buf: Sqlite3Int64,
    oom_err: i32,
    n_word: i32,
    z_buf: *mut i8,
    z_input: *mut i8,
    p_vtab: *mut AmatchVtab,
    p_all_words: *mut AmatchWord,
    p_current: *mut AmatchWord,
    p_cost: *mut AmatchAvl,
    p_word: *mut AmatchAvl,
}

///* A match or partial match
#[repr(C)]
#[derive(Copy, Clone)]
struct AmatchWord {
    p_next: *mut AmatchWord,
    s_cost: AmatchAvl,
    s_word: AmatchAvl,
    r_cost: AmatchCost,
    i_seq: i32,
    n_match: i32,
    z_cost: [i8; 10],
    z_word: [i8; 4],
}

///**************************************************************************
///* AVL Tree implementation
////
////*
///* Objects that want to be members of the AVL tree should embedded an
///* instance of this structure.
#[repr(C)]
#[derive(Copy, Clone)]
struct AmatchAvl {
    p_word: *mut AmatchWord,
    z_key: *mut i8,
    p_before: *mut AmatchAvl,
    p_after: *mut AmatchAvl,
    p_up: *mut AmatchAvl,
    height: i32,
    imbalance: i32,
}

/// Recompute the amatch_avl.height and amatch_avl.imbalance fields for p.
///* Assume that the children of p have correct heights.
#[allow(unused_doc_comments)]
extern "C" fn amatch_avl_recompute_height(p: &mut AmatchAvl) -> () {
    let h_before: i32 =
        if !((*p).p_before).is_null() {
            unsafe { (*(*p).p_before).height }
        } else { 0 };
    let h_after: i32 =
        if !((*p).p_after).is_null() {
            unsafe { (*(*p).p_after).height }
        } else { 0 };
    (*p).imbalance = h_before - h_after;

    /// -: pAfter higher.  +: pBefore higher
    ((*p).height = if h_before > h_after { h_before } else { h_after } + 1);
}

///*     P                B
///*    / \              / \
///*   B   Z    ==>     X   P
///*  / \                  / \
///* X   Y                Y   Z
///*
extern "C" fn amatch_avl_rotate_before(p_p_1: *mut AmatchAvl)
    -> *mut AmatchAvl {
    let p_b: *mut AmatchAvl = unsafe { (*p_p_1).p_before };
    let p_y: *mut AmatchAvl = unsafe { (*p_b).p_after };
    unsafe { (*p_b).p_up = unsafe { (*p_p_1).p_up } };
    unsafe { (*p_b).p_after = p_p_1 };
    unsafe { (*p_p_1).p_up = p_b };
    unsafe { (*p_p_1).p_before = p_y };
    if !(p_y).is_null() { unsafe { (*p_y).p_up = p_p_1 }; }
    amatch_avl_recompute_height(unsafe { &mut *p_p_1 });
    amatch_avl_recompute_height(unsafe { &mut *p_b });
    return p_b;
}

///*     P                A
///*    / \              / \
///*   X   A    ==>     P   Z
///*      / \          / \
///*     Y   Z        X   Y
///*
extern "C" fn amatch_avl_rotate_after(p_p_1: *mut AmatchAvl)
    -> *mut AmatchAvl {
    let p_a: *mut AmatchAvl = unsafe { (*p_p_1).p_after };
    let p_y: *mut AmatchAvl = unsafe { (*p_a).p_before };
    unsafe { (*p_a).p_up = unsafe { (*p_p_1).p_up } };
    unsafe { (*p_a).p_before = p_p_1 };
    unsafe { (*p_p_1).p_up = p_a };
    unsafe { (*p_p_1).p_after = p_y };
    if !(p_y).is_null() { unsafe { (*p_y).p_up = p_p_1 }; }
    amatch_avl_recompute_height(unsafe { &mut *p_p_1 });
    amatch_avl_recompute_height(unsafe { &mut *p_a });
    return p_a;
}

///* Return a pointer to the pBefore or pAfter pointer in the parent
///* of p that points to p.  Or if p is the root node, return pp.
extern "C" fn amatch_avl_from_ptr(p: *mut AmatchAvl, pp: *mut *mut AmatchAvl)
    -> *mut *mut AmatchAvl {
    let p_up: *mut AmatchAvl = unsafe { (*p).p_up };
    if p_up == core::ptr::null_mut() { return pp; }
    if unsafe { (*p_up).p_after } == p {
        return unsafe { &mut (*p_up).p_after };
    }
    return unsafe { &mut (*p_up).p_before };
}

///* Rebalance all nodes starting with p and working up to the root.
///* Return the new root.
extern "C" fn amatch_avl_balance(mut p: *mut AmatchAvl) -> *mut AmatchAvl {
    let mut p_top: *mut AmatchAvl = p;
    let mut pp: *mut *mut AmatchAvl = core::ptr::null_mut();
    while !(p).is_null() {
        amatch_avl_recompute_height(unsafe { &mut *p });
        if unsafe { (*p).imbalance } >= 2 {
            let p_b: *mut AmatchAvl = unsafe { (*p).p_before };
            if unsafe { (*p_b).imbalance } < 0 {
                unsafe { (*p).p_before = amatch_avl_rotate_after(p_b) };
            }
            pp = amatch_avl_from_ptr(p, &mut p);
            p =
                {
                    unsafe { *pp = amatch_avl_rotate_before(p) };
                    unsafe { *pp }
                };
        } else if unsafe { (*p).imbalance } <= -2 {
            let p_a: *mut AmatchAvl = unsafe { (*p).p_after };
            if unsafe { (*p_a).imbalance } > 0 {
                unsafe { (*p).p_after = amatch_avl_rotate_before(p_a) };
            }
            pp = amatch_avl_from_ptr(p, &mut p);
            p =
                {
                    unsafe { *pp = amatch_avl_rotate_after(p) };
                    unsafe { *pp }
                };
        }
        p_top = p;
        p = unsafe { (*p).p_up };
    }
    return p_top;
}

/// Search the tree rooted at p for an entry with zKey.  Return a pointer
///* to the entry or return NULL.
extern "C" fn amatch_avl_search(mut p: *mut AmatchAvl, z_key_1: *const i8)
    -> *mut AmatchAvl {
    let mut c: i32 = 0;
    while !(p).is_null() &&
            {
                    c =
                        unsafe {
                            strcmp(z_key_1, unsafe { (*p).z_key } as *const i8)
                        };
                    c
                } != 0 {
        p =
            if c < 0 {
                unsafe { (*p).p_before }
            } else { unsafe { (*p).p_after } };
    }
    return p;
}

/// Find the first node (the one with the smallest key).
extern "C" fn amatch_avl_first(mut p: *mut AmatchAvl) -> *mut AmatchAvl {
    if !(p).is_null() {
        while !(unsafe { (*p).p_before }).is_null() {
            p = unsafe { (*p).p_before };
        }
    }
    return p;
}

/// Insert a new node pNew.  Return NULL on success.  If the key is not
///* unique, then do not perform the insert but instead leave pNew unchanged
///* and return a pointer to an existing node with the same key.
#[allow(unused_doc_comments)]
extern "C" fn amatch_avl_insert(pp_head_1: &mut *mut AmatchAvl,
    p_new_1: *mut AmatchAvl) -> *mut AmatchAvl {
    let mut c: i32 = 0;
    let mut p: *mut AmatchAvl = *pp_head_1;
    if p == core::ptr::null_mut() {
        p = p_new_1;
        unsafe { (*p_new_1).p_up = core::ptr::null_mut() };
    } else {
        while !(p).is_null() {
            c =
                unsafe {
                    strcmp(unsafe { (*p_new_1).z_key } as *const i8,
                        unsafe { (*p).z_key } as *const i8)
                };
            if c < 0 {
                if !(unsafe { (*p).p_before }).is_null() {
                    p = unsafe { (*p).p_before };
                } else {
                    unsafe { (*p).p_before = p_new_1 };
                    unsafe { (*p_new_1).p_up = p };
                    break;
                }
            } else if c > 0 {
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
    unsafe { (*p_new_1).height = 1 };
    unsafe { (*p_new_1).imbalance = 0 };
    *pp_head_1 = amatch_avl_balance(p);

    /// assert( amatchAvlIntegrity(*ppHead) ); */
    ///  /* assert( amatchAvlIntegrity2(*ppHead) );
    return core::ptr::null_mut();
}

/// Remove node pOld from the tree.  pOld must be an element of the tree or
///* the AVL tree will become corrupt.
#[allow(unused_doc_comments)]
extern "C" fn amatch_avl_remove(pp_head_1: *mut *mut AmatchAvl,
    p_old_1: *mut AmatchAvl) -> () {
    let mut pp_parent: *mut *mut AmatchAvl = core::ptr::null_mut();
    let mut p_balance: *mut AmatchAvl = core::ptr::null_mut();

    /// assert( amatchAvlSearch(*ppHead, pOld->zKey)==pOld );
    (pp_parent = amatch_avl_from_ptr(p_old_1, pp_head_1));
    if unsafe { (*p_old_1).p_before } == core::ptr::null_mut() &&
            unsafe { (*p_old_1).p_after } == core::ptr::null_mut() {
        unsafe { *pp_parent = core::ptr::null_mut() };
        p_balance = unsafe { (*p_old_1).p_up };
    } else if !(unsafe { (*p_old_1).p_before }).is_null() &&
            !(unsafe { (*p_old_1).p_after }).is_null() {
        let mut p_x: *mut AmatchAvl = core::ptr::null_mut();
        let mut p_y: *mut AmatchAvl = core::ptr::null_mut();
        p_x = amatch_avl_first(unsafe { (*p_old_1).p_after });
        unsafe {
            *amatch_avl_from_ptr(p_x, core::ptr::null_mut()) =
                unsafe { (*p_x).p_after }
        };
        if !(unsafe { (*p_x).p_after }).is_null() {
            unsafe {
                (*unsafe { (*p_x).p_after }).p_up = unsafe { (*p_x).p_up }
            };
        }
        p_balance = unsafe { (*p_x).p_up };
        unsafe { (*p_x).p_after = unsafe { (*p_old_1).p_after } };
        if !(unsafe { (*p_x).p_after }).is_null() {
            unsafe { (*unsafe { (*p_x).p_after }).p_up = p_x };
        } else {
            if !(p_balance == p_old_1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"amatchAvlRemove".as_ptr() as *const i8,
                        c"amatch.c".as_ptr() as *mut i8 as *const i8, 417,
                        c"pBalance==pOld".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            p_balance = p_x;
        }
        unsafe {
            (*p_x).p_before = { p_y = unsafe { (*p_old_1).p_before }; p_y }
        };
        if !(p_y).is_null() { unsafe { (*p_y).p_up = p_x }; }
        unsafe { (*p_x).p_up = unsafe { (*p_old_1).p_up } };
        unsafe { *pp_parent = p_x };
    } else if unsafe { (*p_old_1).p_before } == core::ptr::null_mut() {
        unsafe {
            *pp_parent =
                { p_balance = unsafe { (*p_old_1).p_after }; p_balance }
        };
        unsafe { (*p_balance).p_up = unsafe { (*p_old_1).p_up } };
    } else if unsafe { (*p_old_1).p_after } == core::ptr::null_mut() {
        unsafe {
            *pp_parent =
                { p_balance = unsafe { (*p_old_1).p_before }; p_balance }
        };
        unsafe { (*p_balance).p_up = unsafe { (*p_old_1).p_up } };
    }
    unsafe { *pp_head_1 = amatch_avl_balance(p_balance) };
    unsafe { (*p_old_1).p_up = core::ptr::null_mut() };
    unsafe { (*p_old_1).p_before = core::ptr::null_mut() };
    unsafe { (*p_old_1).p_after = core::ptr::null_mut() };
}

///* The two input rule lists are both sorted in order of increasing
///* cost.  Merge them together into a single list, sorted by cost, and
///* return a pointer to the head of that list.
extern "C" fn amatch_merge_rules(mut p_a_1: *mut AmatchRule,
    mut p_b_1: *mut AmatchRule) -> *mut AmatchRule {
    let mut head: AmatchRule = unsafe { core::mem::zeroed() };
    let mut p_tail: *mut AmatchRule = core::ptr::null_mut();
    p_tail = &mut head;
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if unsafe { (*p_a_1).r_cost } <= unsafe { (*p_b_1).r_cost } {
            unsafe { (*p_tail).p_next = p_a_1 };
            p_tail = p_a_1;
            p_a_1 = unsafe { (*p_a_1).p_next };
        } else {
            unsafe { (*p_tail).p_next = p_b_1 };
            p_tail = p_b_1;
            p_b_1 = unsafe { (*p_b_1).p_next };
        }
    }
    if p_a_1 == core::ptr::null_mut() {
        unsafe { (*p_tail).p_next = p_b_1 };
    } else { unsafe { (*p_tail).p_next = p_a_1 }; }
    return head.p_next;
}

///* Statement pStmt currently points to a row in the amatch data table. This
///* function allocates and populates a amatch_rule structure according to
///* the content of the row.
///*
///* If successful, *ppRule is set to point to the new object and SQLITE_OK
///* is returned. Otherwise, *ppRule is zeroed, *pzErr may be set to point
///* to an error message and an SQLite error code returned.
#[allow(unused_doc_comments)]
extern "C" fn amatch_load_one_rule(p: &mut AmatchVtab,
    p_stmt_1: *mut Sqlite3Stmt, pp_rule_1: &mut *mut AmatchRule,
    pz_err_1: &mut *mut i8) -> i32 {
    let i_lang: Sqlite3Int64 = unsafe { sqlite3_column_int64(p_stmt_1, 0) };
    let mut z_from: *const i8 =
        unsafe { sqlite3_column_text(p_stmt_1, 1) } as *const i8;
    let mut z_to: *const i8 =
        unsafe { sqlite3_column_text(p_stmt_1, 2) } as *const i8;
    let r_cost: AmatchCost = unsafe { sqlite3_column_int(p_stmt_1, 3) };
    let mut rc: i32 = 0;
    /// Return code
    let mut n_from: i32 = 0;
    /// Size of string zFrom, in bytes
    let mut n_to: i32 = 0;
    /// Size of string zTo, in bytes
    let mut p_rule: *mut AmatchRule = core::ptr::null_mut();
    if z_from == core::ptr::null() {
        z_from = c"".as_ptr() as *mut i8 as *const i8;
    }
    if z_to == core::ptr::null() {
        z_to = c"".as_ptr() as *mut i8 as *const i8;
    }
    n_from = unsafe { strlen(z_from) } as i32;
    n_to = unsafe { strlen(z_to) } as i32;
    if unsafe { strcmp(z_from, z_to) } == 0 {
        if unsafe { *z_from.offset(0 as isize) } as i32 == '?' as i32 &&
                unsafe { *z_from.offset(1 as isize) } as i32 == 0 {
            if (*p).r_sub == 0 || (*p).r_sub > r_cost { (*p).r_sub = r_cost; }
        }
        *pp_rule_1 = core::ptr::null_mut();
        return 0;
    }
    if r_cost <= 0 || r_cost > 1000 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: cost must be between 1 and %d".as_ptr()
                            as *mut i8 as *const i8, (*p).z_class_name, 1000)
            };
        rc = 1;
    } else if n_from > 50 || n_to > 50 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: maximum string length is %d".as_ptr() as
                            *mut i8 as *const i8, (*p).z_class_name, 50)
            };
        rc = 1;
    } else if i_lang < 0 as i64 || i_lang > 2147483647 as i64 {
        *pz_err_1 =
            unsafe {
                sqlite3_mprintf(c"%s: iLang must be between 0 and %d".as_ptr()
                            as *mut i8 as *const i8, (*p).z_class_name, 2147483647)
            };
        rc = 1;
    } else if unsafe { strcmp(z_from, c"".as_ptr() as *mut i8 as *const i8) }
                == 0 &&
            unsafe { strcmp(z_to, c"?".as_ptr() as *mut i8 as *const i8) } ==
                0 {
        if (*p).r_ins == 0 || (*p).r_ins > r_cost { (*p).r_ins = r_cost; }
    } else if unsafe { strcmp(z_from, c"?".as_ptr() as *mut i8 as *const i8) }
                == 0 &&
            unsafe { strcmp(z_to, c"".as_ptr() as *mut i8 as *const i8) } == 0
        {
        if (*p).r_del == 0 || (*p).r_del > r_cost { (*p).r_del = r_cost; }
    } else {
        p_rule =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<AmatchRule>() as u64 +
                                n_from as u64 + n_to as u64)
                } as *mut AmatchRule;
        if p_rule == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe {
                memset(p_rule as *mut (), 0,
                    core::mem::size_of::<AmatchRule>() as u64)
            };
            unsafe {
                (*p_rule).z_from =
                    unsafe { &mut (*p_rule).z_to[(n_to + 1) as usize] }
            };
            unsafe { (*p_rule).n_from = n_from as AmatchLen };
            unsafe {
                memcpy(unsafe { (*p_rule).z_from } as *mut (),
                    z_from as *const (), (n_from + 1) as u64)
            };
            unsafe {
                memcpy(unsafe { &raw mut (*p_rule).z_to[0 as usize] } as
                            *mut i8 as *mut (), z_to as *const (), (n_to + 1) as u64)
            };
            unsafe { (*p_rule).n_to = n_to as AmatchLen };
            unsafe { (*p_rule).r_cost = r_cost };
            unsafe { (*p_rule).i_lang = i_lang as i32 };
        }
    }
    *pp_rule_1 = p_rule;
    return rc;
}

///* Free all the content in the edit-cost-table
extern "C" fn amatch_free_rules(p: &mut AmatchVtab) -> () {
    while !((*p).p_rule).is_null() {
        let p_rule: *mut AmatchRule = (*p).p_rule;
        (*p).p_rule = unsafe { (*p_rule).p_next };
        unsafe { sqlite3_free(p_rule as *mut ()) };
    }
    (*p).p_rule = core::ptr::null_mut();
}

///* Load the content of the amatch data table into memory.
#[allow(unused_doc_comments)]
extern "C" fn amatch_load_rules(db: *mut Sqlite3, p: *mut AmatchVtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    /// SELECT used to read from rules table
    let mut p_head: *mut AmatchRule = core::ptr::null_mut();
    z_sql =
        unsafe {
            sqlite3_mprintf(c"SELECT * FROM %Q.%Q".as_ptr() as *mut i8 as
                    *const i8, unsafe { (*p).z_db }, unsafe { (*p).z_cost_tab })
        };
    if z_sql == core::ptr::null_mut() {
        rc = 7;
    } else {
        let mut rc2: i32 = 0;
        /// finalize() return code
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s: %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).z_class_name }, unsafe { sqlite3_errmsg(db) })
                    }
            };
        } else if unsafe { sqlite3_column_count(p_stmt) } != 4 {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s: %s has %d columns, expected 4".as_ptr()
                                    as *mut i8 as *const i8, unsafe { (*p).z_class_name },
                            unsafe { (*p).z_cost_tab },
                            unsafe { sqlite3_column_count(p_stmt) })
                    }
            };
            rc = 1;
        } else {
            while rc == 0 && 100 == unsafe { sqlite3_step(p_stmt) } {
                let mut p_rule: *mut AmatchRule = core::ptr::null_mut();
                rc =
                    amatch_load_one_rule(unsafe { &mut *p }, p_stmt,
                        &mut p_rule, unsafe { &mut *pz_err_1 });
                if !(p_rule).is_null() {
                    unsafe { (*p_rule).p_next = p_head };
                    p_head = p_rule;
                }
            }
        }
        rc2 = unsafe { sqlite3_finalize(p_stmt) };
        if rc == 0 { rc = rc2; }
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    if rc == 0 {
        let mut i: u32 = 0 as u32;
        let mut p_x: *mut AmatchRule = core::ptr::null_mut();
        let mut a: [*mut AmatchRule; 15] = [core::ptr::null_mut(); 15];
        {
            i = 0 as u32;
            '__b7: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*mut AmatchRule; 15]>() as u64 /
                                    core::mem::size_of::<*mut AmatchRule>() as u64) {
                    break '__b7;
                }
                '__c7: loop {
                    a[i as usize] = core::ptr::null_mut();
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        while { p_x = p_head; p_x } != core::ptr::null_mut() {
            p_head = unsafe { (*p_x).p_next };
            unsafe { (*p_x).p_next = core::ptr::null_mut() };
            {
                i = 0 as u32;
                '__b9: loop {
                    if !(!(a[i as usize]).is_null() &&
                                    (i as u64) <
                                        core::mem::size_of::<[*mut AmatchRule; 15]>() as u64 /
                                                core::mem::size_of::<*mut AmatchRule>() as u64 - 1 as u64) {
                        break '__b9;
                    }
                    '__c9: loop {
                        p_x = amatch_merge_rules(a[i as usize], p_x);
                        a[i as usize] = core::ptr::null_mut();
                        break '__c9;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            a[i as usize] = amatch_merge_rules(a[i as usize], p_x);
        }
        {
            { p_x = a[0 as usize]; i = 1 as u32 };
            '__b10: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*mut AmatchRule; 15]>() as u64 /
                                    core::mem::size_of::<*mut AmatchRule>() as u64) {
                    break '__b10;
                }
                '__c10: loop {
                    p_x = amatch_merge_rules(a[i as usize], p_x);
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            (*p).p_rule = amatch_merge_rules(unsafe { (*p).p_rule }, p_x)
        };
    } else {

        /// An error has occurred. Setting p->pRule to point to the head of the
        ///* allocated list ensures that the list will be cleaned up in this case.
        if !(unsafe { (*p).p_rule } == core::ptr::null_mut()) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"amatchLoadRules".as_ptr() as *const i8,
                    c"amatch.c".as_ptr() as *mut i8 as *const i8, 722,
                    c"p->pRule==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { (*p).p_rule = p_head };
    }
    return rc;
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
extern "C" fn amatch_dequote(z_in_1: *const i8) -> *mut i8 {
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
                '__b11: loop {
                    if !((i_in as Sqlite3Int64) < n_in) { break '__b11; }
                    '__c11: loop {
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
                        break '__c11;
                    }
                    { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if !(unsafe { strlen(z_out as *const i8) } as i32 as Sqlite3Int64 <=
                                n_in) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"amatchDequote".as_ptr() as *const i8,
                    c"amatch.c".as_ptr() as *mut i8 as *const i8, 763,
                    c"(int)strlen(zOut)<=nIn".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    return z_out;
}

///* Deallocate the pVCheck prepared statement.
extern "C" fn amatch_v_check_clear(p: &mut AmatchVtab) -> () {
    if !((*p).p_v_check).is_null() {
        unsafe { sqlite3_finalize((*p).p_v_check) };
        (*p).p_v_check = core::ptr::null_mut();
    }
}

///* Deallocate an amatch_vtab object
extern "C" fn amatch_free(p: *mut AmatchVtab) -> () {
    if !(p).is_null() {
        amatch_free_rules(unsafe { &mut *p });
        amatch_v_check_clear(unsafe { &mut *p });
        unsafe { sqlite3_free(unsafe { (*p).z_class_name } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_db } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_cost_tab } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_vocab_tab } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_vocab_word } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_vocab_lang } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p).z_self } as *mut ()) };
        unsafe {
            memset(p as *mut (), 0, core::mem::size_of::<AmatchVtab>() as u64)
        };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

///* xDisconnect/xDestroy method for the amatch module.
extern "C" fn amatch_disconnect(p_vtab_1: *mut Sqlite3Vtab) -> i32 {
    let p: *mut AmatchVtab = p_vtab_1 as *mut AmatchVtab;
    if !(unsafe { (*p).n_cursor } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"amatchDisconnect".as_ptr() as *const i8,
                c"amatch.c".as_ptr() as *mut i8 as *const i8, 802,
                c"p->nCursor==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    amatch_free(p);
    return 0;
}

///* Check to see if the argument is of the form:
///*
///*       KEY = VALUE
///*
///* If it is, return a pointer to the first character of VALUE.
///* If not, return NULL.  Spaces around the = are ignored.
extern "C" fn amatch_value_of_key(z_key_1: *const i8, z_str_1: *const i8)
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
        '__b12: loop {
            if !(unsafe {
                                isspace(unsafe { *z_str_1.offset(i as isize) } as u8 as i32)
                            } != 0) {
                break '__b12;
            }
            '__c12: loop { break '__c12; }
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

///* xConnect/xCreate method for the amatch module. Arguments are:
///*
///*   argv[0]    -> module name  ("approximate_match")
///*   argv[1]    -> database name
///*   argv[2]    -> table name
///*   argv[3...] -> arguments
#[allow(unused_doc_comments)]
extern "C" fn amatch_connect(db: *mut Sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut Sqlite3Vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut p_new: *mut AmatchVtab = core::ptr::null_mut();
    /// New virtual table
    let mut z_module: *const i8 = core::ptr::null();
    let mut z_db: *const i8 = core::ptr::null();
    let mut z_val: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s15:
            {
            match __state {
                0 => { rc = 0; __state = 3; }
                2 => { amatch_free(p_new); __state = 73; }
                3 => { p_new = core::ptr::null_mut(); __state = 4; }
                4 => {
                    z_module = unsafe { *argv.offset(0 as isize) };
                    __state = 5;
                }
                5 => {
                    z_db = unsafe { *argv.offset(1 as isize) };
                    __state = 6;
                }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { { let _ = p_aux_1; }; __state = 9; }
                9 => {
                    unsafe { *pp_vtab_1 = core::ptr::null_mut() };
                    __state = 10;
                }
                10 => {
                    p_new =
                        unsafe {
                                sqlite3_malloc64(core::mem::size_of::<AmatchVtab>() as
                                        Sqlite3Uint64)
                            } as *mut AmatchVtab;
                    __state = 11;
                }
                11 => {
                    if p_new == core::ptr::null_mut() {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => { rc = 7; __state = 14; }
                13 => { return 7; }
                14 => {
                    unsafe {
                        memset(p_new as *mut (), 0,
                            core::mem::size_of::<AmatchVtab>() as u64)
                    };
                    __state = 15;
                }
                15 => { unsafe { (*p_new).db = db }; __state = 16; }
                16 => {
                    unsafe {
                        (*p_new).z_class_name =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    z_module)
                            }
                    };
                    __state = 17;
                }
                17 => {
                    if unsafe { (*p_new).z_class_name } == core::ptr::null_mut()
                        {
                        __state = 19;
                    } else { __state = 18; }
                }
                18 => {
                    unsafe {
                        (*p_new).z_db =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    z_db)
                            }
                    };
                    __state = 20;
                }
                19 => { __state = 2; }
                20 => {
                    if unsafe { (*p_new).z_db } == core::ptr::null_mut() {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => {
                    unsafe {
                        (*p_new).z_self =
                            unsafe {
                                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(2 as isize) })
                            }
                    };
                    __state = 23;
                }
                22 => { __state = 2; }
                23 => {
                    if unsafe { (*p_new).z_self } == core::ptr::null_mut() {
                        __state = 25;
                    } else { __state = 24; }
                }
                24 => { i = 3; __state = 27; }
                25 => { __state = 2; }
                26 => { rc = 0; __state = 61; }
                27 => { if i < argc { __state = 28; } else { __state = 26; } }
                28 => {
                    z_val =
                        amatch_value_of_key(c"vocabulary_table".as_ptr() as *mut i8
                                as *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 30;
                }
                29 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 27;
                }
                30 => {
                    if !(z_val).is_null() {
                        __state = 32;
                    } else { __state = 31; }
                }
                31 => {
                    z_val =
                        amatch_value_of_key(c"vocabulary_word".as_ptr() as *mut i8
                                as *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 37;
                }
                32 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_vocab_tab } as *mut ())
                    };
                    __state = 33;
                }
                33 => {
                    unsafe { (*p_new).z_vocab_tab = amatch_dequote(z_val) };
                    __state = 34;
                }
                34 => {
                    if unsafe { (*p_new).z_vocab_tab } == core::ptr::null_mut()
                        {
                        __state = 36;
                    } else { __state = 35; }
                }
                35 => { __state = 29; }
                36 => { __state = 2; }
                37 => {
                    if !(z_val).is_null() {
                        __state = 39;
                    } else { __state = 38; }
                }
                38 => {
                    z_val =
                        amatch_value_of_key(c"vocabulary_language".as_ptr() as
                                    *mut i8 as *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 44;
                }
                39 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_vocab_word } as *mut ())
                    };
                    __state = 40;
                }
                40 => {
                    unsafe { (*p_new).z_vocab_word = amatch_dequote(z_val) };
                    __state = 41;
                }
                41 => {
                    if unsafe { (*p_new).z_vocab_word } == core::ptr::null_mut()
                        {
                        __state = 43;
                    } else { __state = 42; }
                }
                42 => { __state = 29; }
                43 => { __state = 2; }
                44 => {
                    if !(z_val).is_null() {
                        __state = 46;
                    } else { __state = 45; }
                }
                45 => {
                    z_val =
                        amatch_value_of_key(c"edit_distances".as_ptr() as *mut i8 as
                                *const i8, unsafe { *argv.offset(i as isize) });
                    __state = 51;
                }
                46 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_vocab_lang } as *mut ())
                    };
                    __state = 47;
                }
                47 => {
                    unsafe { (*p_new).z_vocab_lang = amatch_dequote(z_val) };
                    __state = 48;
                }
                48 => {
                    if unsafe { (*p_new).z_vocab_lang } == core::ptr::null_mut()
                        {
                        __state = 50;
                    } else { __state = 49; }
                }
                49 => { __state = 29; }
                50 => { __state = 2; }
                51 => {
                    if !(z_val).is_null() {
                        __state = 53;
                    } else { __state = 52; }
                }
                52 => {
                    unsafe {
                        *pz_err_1 =
                            unsafe {
                                sqlite3_mprintf(c"unrecognized argument: [%s]\n".as_ptr() as
                                            *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                            }
                    };
                    __state = 58;
                }
                53 => {
                    unsafe {
                        sqlite3_free(unsafe { (*p_new).z_cost_tab } as *mut ())
                    };
                    __state = 54;
                }
                54 => {
                    unsafe { (*p_new).z_cost_tab = amatch_dequote(z_val) };
                    __state = 55;
                }
                55 => {
                    if unsafe { (*p_new).z_cost_tab } == core::ptr::null_mut() {
                        __state = 57;
                    } else { __state = 56; }
                }
                56 => { __state = 29; }
                57 => { __state = 2; }
                58 => { amatch_free(p_new); __state = 59; }
                59 => {
                    unsafe { *pp_vtab_1 = core::ptr::null_mut() };
                    __state = 60;
                }
                60 => { return 1; }
                61 => {
                    if unsafe { (*p_new).z_cost_tab } == core::ptr::null_mut() {
                        __state = 63;
                    } else { __state = 64; }
                }
                62 => { if rc == 0 { __state = 67; } else { __state = 66; } }
                63 => {
                    unsafe {
                        *pz_err_1 =
                            unsafe {
                                sqlite3_mprintf(c"no edit_distances table specified".as_ptr()
                                            as *mut i8 as *const i8)
                            }
                    };
                    __state = 65;
                }
                64 => {
                    rc = amatch_load_rules(db, p_new, pz_err_1);
                    __state = 62;
                }
                65 => { rc = 1; __state = 62; }
                66 => { if rc != 0 { __state = 70; } else { __state = 69; } }
                67 => { unsafe { sqlite3_vtab_config(db, 2) }; __state = 68; }
                68 => {
                    rc =
                        unsafe {
                            sqlite3_declare_vtab(db,
                                c"CREATE TABLE x(word,distance,language,command HIDDEN,nword HIDDEN)".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    __state = 66;
                }
                69 => {
                    unsafe { *pp_vtab_1 = unsafe { &mut (*p_new).base } };
                    __state = 71;
                }
                70 => { amatch_free(p_new); __state = 69; }
                71 => { return rc; }
                72 => { __state = 2; }
                73 => { return rc; }
                _ => {}
            }
        }
    }

    /// Return code
    /// New virtual table
    unreachable!();
}

///* Open a new amatch cursor.
extern "C" fn amatch_open(p_v_tab_1: *mut Sqlite3Vtab,
    pp_cursor_1: *mut *mut Sqlite3VtabCursor) -> i32 {
    let p: *mut AmatchVtab = p_v_tab_1 as *mut AmatchVtab;
    let mut p_cur: *mut AmatchCursor = core::ptr::null_mut();
    p_cur =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<AmatchCursor>() as
                        Sqlite3Uint64)
            } as *mut AmatchCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe {
        memset(p_cur as *mut (), 0,
            core::mem::size_of::<AmatchCursor>() as u64)
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

///* Free up all the memory allocated by a cursor.  Set it rLimit to 0
///* to indicate that it is at EOF.
extern "C" fn amatch_clear_cursor(p_cur_1: &mut AmatchCursor) -> () {
    let mut p_word: *mut AmatchWord = core::ptr::null_mut();
    let mut p_next_word: *mut AmatchWord = core::ptr::null_mut();
    {
        p_word = (*p_cur_1).p_all_words;
        '__b16: loop {
            if !(!(p_word).is_null()) { break '__b16; }
            '__c16: loop {
                p_next_word = unsafe { (*p_word).p_next };
                unsafe { sqlite3_free(p_word as *mut ()) };
                break '__c16;
            }
            p_word = p_next_word;
        }
    }
    (*p_cur_1).p_all_words = core::ptr::null_mut();
    unsafe { sqlite3_free((*p_cur_1).z_input as *mut ()) };
    (*p_cur_1).z_input = core::ptr::null_mut();
    unsafe { sqlite3_free((*p_cur_1).z_buf as *mut ()) };
    (*p_cur_1).z_buf = core::ptr::null_mut();
    (*p_cur_1).n_buf = 0 as Sqlite3Int64;
    (*p_cur_1).p_cost = core::ptr::null_mut();
    (*p_cur_1).p_word = core::ptr::null_mut();
    (*p_cur_1).p_current = core::ptr::null_mut();
    (*p_cur_1).r_limit = 1000000;
    (*p_cur_1).i_lang = 0;
    (*p_cur_1).n_word = 0;
}

///* Close a amatch cursor.
extern "C" fn amatch_close(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut AmatchCursor = cur as *mut AmatchCursor;
    amatch_clear_cursor(unsafe { &mut *p_cur });
    {
        let __p = unsafe { &mut (*unsafe { (*p_cur).p_vtab }).n_cursor };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    unsafe { sqlite3_free(p_cur as *mut ()) };
    return 0;
}

///* Render a 24-bit unsigned integer as a 4-byte base-64 number.
extern "C" fn amatch_encode_int(x: i32, z: *mut i8) -> () {
    unsafe { *z.offset(0 as isize) = a_1[(x >> 18 & 63) as usize] as i8 };
    unsafe { *z.offset(1 as isize) = a_1[(x >> 12 & 63) as usize] as i8 };
    unsafe { *z.offset(2 as isize) = a_1[(x >> 6 & 63) as usize] as i8 };
    unsafe { *z.offset(3 as isize) = a_1[(x & 63) as usize] as i8 };
}

///* Write the zCost[] field for a amatch_word object
extern "C" fn amatch_write_cost(p_word_1: &mut AmatchWord) -> () {
    amatch_encode_int((*p_word_1).r_cost,
        &raw mut (*p_word_1).z_cost[0 as usize] as *mut i8);
    amatch_encode_int((*p_word_1).i_seq,
        unsafe {
            (&raw mut (*p_word_1).z_cost[0 as usize] as
                    *mut i8).offset(4 as isize)
        });
    (*p_word_1).z_cost[8 as usize] = 0 as i8;
}

/// Circumvent compiler warnings about the use of strcpy() by supplying
///* our own implementation.
extern "C" fn amatch_strcpy(mut dest: *mut i8, mut src: *const i8) -> () {
    while {
                    let __v =
                        unsafe {
                                *{
                                        let __p = &mut src;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as i8;
                    unsafe {
                        *{
                                    let __p = &mut dest;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                } = __v
                    };
                    __v
                } as i32 != 0 {}
}

extern "C" fn amatch_strcat(mut dest: *mut i8, src: *const i8) -> () {
    while unsafe { *dest } != 0 {
        {
            let __p = &mut dest;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    amatch_strcpy(dest, src);
}

///* Add a new amatch_word object to the queue.
///*
///* If a prior amatch_word object with the same zWord, and nMatch
///* already exists, update its rCost (if the new rCost is less) but
///* otherwise leave it unchanged.  Do not add a duplicate.
///*
///* Do nothing if the cost exceeds threshold.
extern "C" fn amatch_add_word(p_cur_1: &mut AmatchCursor,
    r_cost_1: AmatchCost, n_match_1: i32, z_word_base_1: *const i8,
    z_word_tail_1: *const i8) -> () {
    let mut p_word: *mut AmatchWord = core::ptr::null_mut();
    let mut p_node: *const AmatchAvl = core::ptr::null();
    let mut p_other: *const AmatchAvl = core::ptr::null();
    let mut n_base: i32 = 0;
    let mut n_tail: i32 = 0;
    let mut z_buf: [i8; 4] = [0; 4];
    if r_cost_1 > (*p_cur_1).r_limit { return; }
    n_base = unsafe { strlen(z_word_base_1) } as i32;
    n_tail = unsafe { strlen(z_word_tail_1) } as i32;
    if (n_base + n_tail + 3) as Sqlite3Int64 > (*p_cur_1).n_buf {
        (*p_cur_1).n_buf = (n_base + n_tail + 100) as Sqlite3Int64;
        (*p_cur_1).z_buf =
            unsafe {
                    sqlite3_realloc64((*p_cur_1).z_buf as *mut (),
                        (*p_cur_1).n_buf as Sqlite3Uint64)
                } as *mut i8;
        if (*p_cur_1).z_buf == core::ptr::null_mut() {
            (*p_cur_1).n_buf = 0 as Sqlite3Int64;
            return;
        }
    }
    amatch_encode_int(n_match_1, &raw mut z_buf[0 as usize] as *mut i8);
    unsafe {
        memcpy((*p_cur_1).z_buf as *mut (),
            unsafe {
                    (&raw mut z_buf[0 as usize] as *mut i8).offset(2 as isize)
                } as *const (), 2 as u64)
    };
    unsafe {
        memcpy(unsafe { (*p_cur_1).z_buf.offset(2 as isize) } as *mut (),
            z_word_base_1 as *const (), n_base as u64)
    };
    unsafe {
        memcpy(unsafe {
                    unsafe {
                        (*p_cur_1).z_buf.offset(2 as isize).offset(n_base as isize)
                    }
                } as *mut (), z_word_tail_1 as *const (), (n_tail + 1) as u64)
    };
    p_node =
        amatch_avl_search((*p_cur_1).p_word, (*p_cur_1).z_buf as *const i8);
    if !(p_node).is_null() {
        p_word = unsafe { (*p_node).p_word };
        if unsafe { (*p_word).r_cost } > r_cost_1 {
            amatch_avl_remove(&mut (*p_cur_1).p_cost,
                unsafe { &mut (*p_word).s_cost });
            unsafe { (*p_word).r_cost = r_cost_1 };
            amatch_write_cost(unsafe { &mut *p_word });
            p_other =
                amatch_avl_insert(&mut (*p_cur_1).p_cost,
                    unsafe { &mut (*p_word).s_cost });
            if !(p_other == core::ptr::null_mut()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"amatchAddWord".as_ptr() as *const i8,
                        c"amatch.c".as_ptr() as *mut i8 as *const i8, 1071,
                        c"pOther==0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            { let _ = p_other; };
        }
        return;
    }
    p_word =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<AmatchWord>() as u64 +
                                n_base as u64 + n_tail as u64 - 1 as u64)
            } as *mut AmatchWord;
    if p_word == core::ptr::null_mut() { return; }
    unsafe {
        memset(p_word as *mut (), 0,
            core::mem::size_of::<AmatchWord>() as u64)
    };
    unsafe { (*p_word).r_cost = r_cost_1 };
    unsafe {
        (*p_word).i_seq =
            {
                let __p = &mut (*p_cur_1).n_word;
                let __t = *__p;
                *__p += 1;
                __t
            }
    };
    amatch_write_cost(unsafe { &mut *p_word });
    unsafe { (*p_word).n_match = n_match_1 };
    unsafe { (*p_word).p_next = (*p_cur_1).p_all_words };
    (*p_cur_1).p_all_words = p_word;
    unsafe {
        (*p_word).s_cost.z_key =
            unsafe { &raw mut (*p_word).z_cost[0 as usize] } as *mut i8
    };
    unsafe { (*p_word).s_cost.p_word = p_word };
    p_other =
        amatch_avl_insert(&mut (*p_cur_1).p_cost,
            unsafe { &mut (*p_word).s_cost });
    if !(p_other == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"amatchAddWord".as_ptr() as *const i8,
                c"amatch.c".as_ptr() as *mut i8 as *const i8, 1087,
                c"pOther==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    { let _ = p_other; };
    unsafe {
        (*p_word).s_word.z_key =
            unsafe { &raw mut (*p_word).z_word[0 as usize] } as *mut i8
    };
    unsafe { (*p_word).s_word.p_word = p_word };
    amatch_strcpy(unsafe { &raw mut (*p_word).z_word[0 as usize] } as *mut i8,
        (*p_cur_1).z_buf as *const i8);
    p_other =
        amatch_avl_insert(&mut (*p_cur_1).p_word,
            unsafe { &mut (*p_word).s_word });
    if !(p_other == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"amatchAddWord".as_ptr() as *const i8,
                c"amatch.c".as_ptr() as *mut i8 as *const i8, 1092,
                c"pOther==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    { let _ = p_other; };
}

///* Advance a cursor to its next row of output
extern "C" fn amatch_next(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *mut AmatchCursor = cur as *mut AmatchCursor;
    let mut p_word: *mut AmatchWord = core::ptr::null_mut();
    let mut p_node: *const AmatchAvl = core::ptr::null();
    let mut is_match: i32 = 0;
    let p: *mut AmatchVtab = unsafe { (*p_cur).p_vtab };
    let mut n_word: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut z_w: *const i8 = core::ptr::null();
    let mut p_rule: *const AmatchRule = core::ptr::null();
    let mut z_buf: *mut i8 = core::ptr::null_mut();
    let mut n_buf: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut z_next: [i8; 8] = [0; 8];
    let mut z_next_in: [i8; 8] = [0; 8];
    let mut n_next_in: i32 = 0;
    if unsafe { (*p).p_v_check } == core::ptr::null_mut() {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        if !(unsafe { (*p).z_vocab_lang }).is_null() &&
                unsafe { *unsafe { (*p).z_vocab_lang.offset(0 as isize) } } !=
                    0 {
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"SELECT \"%w\" FROM \"%w\"".as_ptr() as
                                *mut i8 as *const i8,
                        c" WHERE \"%w\">=?1 AND \"%w\"=?2 ORDER BY 1".as_ptr() as
                            *mut i8, unsafe { (*p).z_vocab_word },
                        unsafe { (*p).z_vocab_tab }, unsafe { (*p).z_vocab_word },
                        unsafe { (*p).z_vocab_lang })
                };
        } else {
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"SELECT \"%w\" FROM \"%w\" WHERE \"%w\">=?1 ORDER BY 1".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p).z_vocab_word },
                        unsafe { (*p).z_vocab_tab }, unsafe { (*p).z_vocab_word })
                };
        }
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8, -1,
                    unsafe { &mut (*p).p_v_check }, core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if rc != 0 { return rc; }
    }
    unsafe {
        sqlite3_bind_int(unsafe { (*p).p_v_check }, 2,
            unsafe { (*p_cur).i_lang })
    };
    '__b19: loop {
        '__c19: loop {
            p_node = amatch_avl_first(unsafe { (*p_cur).p_cost });
            if p_node == core::ptr::null_mut() {
                p_word = core::ptr::null_mut();
                break '__b19;
            }
            p_word = unsafe { (*p_node).p_word };
            amatch_avl_remove(unsafe { &mut (*p_cur).p_cost },
                unsafe { &mut (*p_word).s_cost });
            n_word =
                unsafe {
                            strlen(unsafe {
                                        (unsafe { &raw mut (*p_word).z_word[0 as usize] } as
                                                *mut i8).offset(2 as isize)
                                    } as *const i8)
                        } as i32 as Sqlite3Int64;
            if n_word + 20 as Sqlite3Int64 > n_buf {
                n_buf = n_word + 100 as Sqlite3Int64;
                z_buf =
                    unsafe {
                            sqlite3_realloc64(z_buf as *mut (), n_buf as Sqlite3Uint64)
                        } as *mut i8;
                if z_buf == core::ptr::null_mut() { return 7; }
            }
            amatch_strcpy(z_buf,
                unsafe {
                        (unsafe { &raw mut (*p_word).z_word[0 as usize] } as
                                *mut i8).offset(2 as isize)
                    } as *const i8);
            z_next[0 as usize] = 0 as i8;
            z_next_in[0 as usize] =
                unsafe {
                    *unsafe {
                            (*p_cur).z_input.offset(unsafe { (*p_word).n_match } as
                                    isize)
                        }
                };
            if z_next_in[0 as usize] != 0 {
                {
                    i = 1;
                    '__b20: loop {
                        if !(i <= 4 &&
                                        unsafe {
                                                        *unsafe {
                                                                (*p_cur).z_input.offset((unsafe { (*p_word).n_match } + i)
                                                                        as isize)
                                                            }
                                                    } as i32 & 192 == 128) {
                            break '__b20;
                        }
                        '__c20: loop {
                            z_next_in[i as usize] =
                                unsafe {
                                    *unsafe {
                                            (*p_cur).z_input.offset((unsafe { (*p_word).n_match } + i)
                                                    as isize)
                                        }
                                };
                            break '__c20;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                z_next_in[i as usize] = 0 as i8;
                n_next_in = i;
            } else { n_next_in = 0; }
            if z_next_in[0 as usize] != 0 &&
                    z_next_in[0 as usize] as i32 != '*' as i32 {
                unsafe { sqlite3_reset(unsafe { (*p).p_v_check }) };
                amatch_strcat(z_buf,
                    &raw mut z_next_in[0 as usize] as *mut i8 as *const i8);
                unsafe {
                    sqlite3_bind_text(unsafe { (*p).p_v_check }, 1,
                        z_buf as *const i8,
                        (n_word + n_next_in as Sqlite3Int64) as i32, None)
                };
                rc = unsafe { sqlite3_step(unsafe { (*p).p_v_check }) };
                if rc == 100 {
                    z_w =
                        unsafe { sqlite3_column_text(unsafe { (*p).p_v_check }, 0) }
                            as *const i8;
                    if unsafe {
                                strncmp(z_buf as *const i8, z_w,
                                    (n_word + n_next_in as Sqlite3Int64) as u64)
                            } == 0 {
                        amatch_add_word(unsafe { &mut *p_cur },
                            unsafe { (*p_word).r_cost },
                            unsafe { (*p_word).n_match } + n_next_in,
                            z_buf as *const i8, c"".as_ptr() as *mut i8 as *const i8);
                    }
                }
                unsafe { *z_buf.offset(n_word as isize) = 0 as i8 };
            }
            loop {
                amatch_strcpy(unsafe { z_buf.offset(n_word as isize) },
                    &raw mut z_next[0 as usize] as *mut i8 as *const i8);
                unsafe { sqlite3_reset(unsafe { (*p).p_v_check }) };
                unsafe {
                    sqlite3_bind_text(unsafe { (*p).p_v_check }, 1,
                        z_buf as *const i8, -1,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
                rc = unsafe { sqlite3_step(unsafe { (*p).p_v_check }) };
                if rc != 100 { break; }
                z_w =
                    unsafe { sqlite3_column_text(unsafe { (*p).p_v_check }, 0) }
                        as *const i8;
                amatch_strcpy(unsafe { z_buf.offset(n_word as isize) },
                    &raw mut z_next[0 as usize] as *mut i8 as *const i8);
                if unsafe { strncmp(z_w, z_buf as *const i8, n_word as u64) }
                        != 0 {
                    break;
                }
                if z_next_in[0 as usize] as i32 == '*' as i32 &&
                            z_next_in[1 as usize] as i32 == 0 ||
                        z_next_in[0 as usize] as i32 == 0 &&
                            unsafe { *z_w.offset(n_word as isize) } as i32 == 0 {
                    is_match = 1;
                    z_next_in[0 as usize] = 0 as i8;
                    n_next_in = 0;
                    break;
                }
                z_next[0 as usize] =
                    unsafe { *z_w.offset(n_word as isize) } as i8;
                {
                    i = 1;
                    '__b22: loop {
                        if !(i <= 4 &&
                                        unsafe {
                                                        *z_w.offset((n_word + i as Sqlite3Int64) as isize)
                                                    } as i32 & 192 == 128) {
                            break '__b22;
                        }
                        '__c22: loop {
                            z_next[i as usize] =
                                unsafe {
                                        *z_w.offset((n_word + i as Sqlite3Int64) as isize)
                                    } as i8;
                            break '__c22;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                z_next[i as usize] = 0 as i8;
                unsafe { *z_buf.offset(n_word as isize) = 0 as i8 };
                if unsafe { (*p).r_ins } > 0 {
                    amatch_add_word(unsafe { &mut *p_cur },
                        unsafe { (*p_word).r_cost } + unsafe { (*p).r_ins },
                        unsafe { (*p_word).n_match }, z_buf as *const i8,
                        &raw mut z_next[0 as usize] as *mut i8 as *const i8);
                }
                if unsafe { (*p).r_sub } > 0 {
                    amatch_add_word(unsafe { &mut *p_cur },
                        unsafe { (*p_word).r_cost } + unsafe { (*p).r_sub },
                        unsafe { (*p_word).n_match } + n_next_in,
                        z_buf as *const i8,
                        &raw mut z_next[0 as usize] as *mut i8 as *const i8);
                }
                if unsafe { (*p).r_ins } < 0 && unsafe { (*p).r_sub } < 0 {
                    break;
                }
                {
                    let __p = &mut z_next[(i - 1) as usize];
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            unsafe { sqlite3_reset(unsafe { (*p).p_v_check }) };
            if unsafe { (*p).r_del } > 0 {
                unsafe { *z_buf.offset(n_word as isize) = 0 as i8 };
                amatch_add_word(unsafe { &mut *p_cur },
                    unsafe { (*p_word).r_cost } + unsafe { (*p).r_del },
                    unsafe { (*p_word).n_match } + n_next_in,
                    z_buf as *const i8, c"".as_ptr() as *mut i8 as *const i8);
            }
            {
                p_rule = unsafe { (*p).p_rule };
                '__b23: loop {
                    if !(!(p_rule).is_null()) { break '__b23; }
                    '__c23: loop {
                        if unsafe { (*p_rule).i_lang } != unsafe { (*p_cur).i_lang }
                            {
                            break '__c23;
                        }
                        if unsafe {
                                    strncmp(unsafe { (*p_rule).z_from } as *const i8,
                                        unsafe {
                                                unsafe {
                                                    (*p_cur).z_input.offset(unsafe { (*p_word).n_match } as
                                                            isize)
                                                }
                                            } as *const i8, unsafe { (*p_rule).n_from } as u64)
                                } == 0 {
                            amatch_add_word(unsafe { &mut *p_cur },
                                unsafe { (*p_word).r_cost } + unsafe { (*p_rule).r_cost },
                                unsafe { (*p_word).n_match } +
                                    unsafe { (*p_rule).n_from } as i32,
                                unsafe {
                                        (unsafe { &raw mut (*p_word).z_word[0 as usize] } as
                                                *mut i8).offset(2 as isize)
                                    } as *const i8,
                                unsafe { &raw const (*p_rule).z_to[0 as usize] } as *mut i8
                                    as *const i8);
                        }
                        break '__c23;
                    }
                    p_rule = unsafe { (*p_rule).p_next };
                }
            }
            break '__c19;
        }
        if !((is_match == 0) as i32 != 0) { break '__b19; }
    }
    unsafe { (*p_cur).p_current = p_word };
    unsafe { sqlite3_free(z_buf as *mut ()) };
    return 0;
}

///* Called to "rewind" a cursor back to the beginning so that
///* it starts its output over again.  Always called at least once
///* prior to any amatchColumn, amatchRowid, or amatchEof call.
extern "C" fn amatch_filter(p_vtab_cursor_1: *mut Sqlite3VtabCursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut Sqlite3Value) -> i32 {
    let p_cur: *mut AmatchCursor = p_vtab_cursor_1 as *mut AmatchCursor;
    let mut z_word: *const i8 = c"*".as_ptr() as *mut i8 as *const i8;
    let mut idx: i32 = 0;
    let mut rc: i32 = 0;
    amatch_clear_cursor(unsafe { &mut *p_cur });
    idx = 0;
    if idx_num_1 & 1 != 0 {
        z_word =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        if z_word == core::ptr::null() {
            z_word = c"".as_ptr() as *mut i8 as *const i8;
        }
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    if idx_num_1 & 2 != 0 {
        unsafe {
            (*p_cur).r_limit =
                unsafe {
                        sqlite3_value_int(unsafe { *argv.offset(idx as isize) })
                    } as AmatchCost
        };
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    if idx_num_1 & 4 != 0 {
        unsafe {
            (*p_cur).i_lang =
                unsafe {
                        sqlite3_value_int(unsafe { *argv.offset(idx as isize) })
                    } as AmatchCost
        };
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
    }
    unsafe {
        (*p_cur).z_input =
            unsafe {
                sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                    z_word)
            }
    };
    if unsafe { (*p_cur).z_input } == core::ptr::null_mut() { return 7; }
    if unsafe { strlen(unsafe { (*p_cur).z_input } as *const i8) } >
            1000 as u64 {
        unsafe {
            *unsafe { (*p_cur).z_input.offset(1000 as isize) } = 0 as i8
        };
        rc = 18;
    }
    amatch_add_word(unsafe { &mut *p_cur }, 0, 0,
        c"".as_ptr() as *mut i8 as *const i8,
        c"".as_ptr() as *mut i8 as *const i8);
    amatch_next(p_vtab_cursor_1);
    return rc;
}

///* Only the word and distance columns have values.  All other columns
///* return NULL
extern "C" fn amatch_column(cur: *mut Sqlite3VtabCursor,
    ctx: *mut Sqlite3Context, i: i32) -> i32 {
    let p_cur: *const AmatchCursor =
        cur as *mut AmatchCursor as *const AmatchCursor;
    '__s24:
        {
        match i {
            0 => {
                {
                    unsafe {
                        sqlite3_result_text(ctx,
                            unsafe {
                                    (unsafe {
                                                &raw mut (*unsafe { (*p_cur).p_current }).z_word[0 as usize]
                                            } as *mut i8).offset(2 as isize)
                                } as *const i8, -1, None)
                    };
                    break '__s24;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe { (*unsafe { (*p_cur).p_current }).r_cost })
                    };
                    break '__s24;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s24;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_word })
                    };
                    break '__s24;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s24; }
            }
            1 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx,
                            unsafe { (*unsafe { (*p_cur).p_current }).r_cost })
                    };
                    break '__s24;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s24;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_word })
                    };
                    break '__s24;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s24; }
            }
            2 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).i_lang })
                    };
                    break '__s24;
                }
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_word })
                    };
                    break '__s24;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s24; }
            }
            4 => {
                {
                    unsafe {
                        sqlite3_result_int(ctx, unsafe { (*p_cur).n_word })
                    };
                    break '__s24;
                }
                { unsafe { sqlite3_result_null(ctx) }; break '__s24; }
            }
            _ => { { unsafe { sqlite3_result_null(ctx) }; break '__s24; } }
        }
    }
    return 0;
}

///* The rowid.
extern "C" fn amatch_rowid(cur: *mut Sqlite3VtabCursor,
    p_rowid_1: *mut SqliteInt64) -> i32 {
    let p_cur: *const AmatchCursor =
        cur as *mut AmatchCursor as *const AmatchCursor;
    unsafe { *p_rowid_1 = unsafe { (*p_cur).i_rowid } };
    return 0;
}

///* EOF indicator
extern "C" fn amatch_eof(cur: *mut Sqlite3VtabCursor) -> i32 {
    let p_cur: *const AmatchCursor =
        cur as *mut AmatchCursor as *const AmatchCursor;
    return (unsafe { (*p_cur).p_current } == core::ptr::null_mut()) as i32;
}

///* Search for terms of these forms:
///*
///*   (A)    word MATCH $str
///*   (B1)   distance < $value
///*   (B2)   distance <= $value
///*   (C)    language == $language
///*
///* The distance< and distance<= are both treated as distance<=.
///* The query plan number is a bit vector:
///*
///*   bit 1:   Term of the form (A) found
///*   bit 2:   Term like (B1) or (B2) found
///*   bit 3:   Term like (C) found
///*
///* If bit-1 is set, $str is always in filter.argv[0].  If bit-2 is set
///* then $value is in filter.argv[0] if bit-1 is clear and is in 
///* filter.argv[1] if bit-1 is set.  If bit-3 is set, then $ruleid is
///* in filter.argv[0] if bit-1 and bit-2 are both zero, is in
///* filter.argv[1] if exactly one of bit-1 and bit-2 are set, and is in
///* filter.argv[2] if both bit-1 and bit-2 are set.
extern "C" fn amatch_best_index(tab: *mut Sqlite3Vtab,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> i32 {
    let mut i_plan: i32 = 0;
    let mut i_dist_term: i32 = -1;
    let mut i_lang_term: i32 = -1;
    let mut i: i32 = 0;
    let mut p_constraint: *const Sqlite3IndexConstraint = core::ptr::null();
    { let _ = tab; };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const Sqlite3IndexConstraint;
    {
        i = 0;
        '__b25: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b25;
            }
            '__c25: loop {
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    break '__c25;
                }
                if i_plan & 1 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 0 &&
                        unsafe { (*p_constraint).op } as i32 == 64 {
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
                }
                if i_plan & 2 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 1 &&
                        (unsafe { (*p_constraint).op } as i32 == 16 ||
                            unsafe { (*p_constraint).op } as i32 == 8) {
                    i_plan |= 2;
                    i_dist_term = i;
                }
                if i_plan & 4 == 0 &&
                            unsafe { (*p_constraint).i_column } as i32 == 2 &&
                        unsafe { (*p_constraint).op } as i32 == 2 {
                    i_plan |= 4;
                    unsafe {
                        (*unsafe {
                                        (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                                    }).omit = 1 as u8
                    };
                    i_lang_term = i;
                }
                break '__c25;
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
    if i_plan & 2 != 0 {
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_dist_term as
                                    isize)
                        }).argv_index = 1 + (i_plan & 1 != 0) as i32
        };
    }
    if i_plan & 4 != 0 {
        let mut idx: i32 = 1;
        if i_plan & 1 != 0 {
            { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        }
        if i_plan & 2 != 0 {
            { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        }
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i_lang_term as
                                    isize)
                        }).argv_index = idx
        };
    }
    unsafe { (*p_idx_info_1).idx_num = i_plan };
    if unsafe { (*p_idx_info_1).n_order_by } == 1 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } == 1 &&
            unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } as i32 == 0 {
        unsafe { (*p_idx_info_1).order_by_consumed = 1 };
    }
    unsafe { (*p_idx_info_1).estimated_cost = 10000 as f64 };
    return 0;
}

///* The xUpdate() method.  
///*
///* This implementation disallows DELETE and UPDATE.  The only thing
///* allowed is INSERT into the "command" column.
extern "C" fn amatch_update(p_v_tab_1: *mut Sqlite3Vtab, argc: i32,
    argv: *mut *mut Sqlite3Value, p_rowid_1: *mut SqliteInt64) -> i32 {
    let p: *const AmatchVtab =
        p_v_tab_1 as *mut AmatchVtab as *const AmatchVtab;
    let mut z_cmd: *const u8 = core::ptr::null();
    { let _ = p_rowid_1; };
    if argc == 1 {
        unsafe {
            (*p_v_tab_1).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"DELETE from %s is not allowed".as_ptr() as
                                *mut i8 as *const i8, unsafe { (*p).z_self })
                }
        };
        return 1;
    }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) } != 5
        {
        unsafe {
            (*p_v_tab_1).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"UPDATE of %s is not allowed".as_ptr() as
                                *mut i8 as *const i8, unsafe { (*p).z_self })
                }
        };
        return 1;
    }
    if unsafe {
                        sqlite3_value_type(unsafe {
                                *argv.offset((2 + 0) as isize)
                            })
                    } != 5 ||
                unsafe {
                        sqlite3_value_type(unsafe {
                                *argv.offset((2 + 1) as isize)
                            })
                    } != 5 ||
            unsafe {
                    sqlite3_value_type(unsafe {
                            *argv.offset((2 + 2) as isize)
                        })
                } != 5 {
        unsafe {
            (*p_v_tab_1).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"INSERT INTO %s allowed for column [command] only".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p).z_self })
                }
        };
        return 1;
    }
    z_cmd =
        unsafe {
            sqlite3_value_text(unsafe { *argv.offset((2 + 3) as isize) })
        };
    if z_cmd == core::ptr::null() { return 0; }
    return 0;
}

///* A virtual table module that implements the "approximate_match".
static mut amatch_module: Sqlite3Module =
    Sqlite3Module {
        i_version: 0,
        x_create: Some(amatch_connect),
        x_connect: Some(amatch_connect),
        x_best_index: Some(amatch_best_index),
        x_disconnect: Some(amatch_disconnect),
        x_destroy: Some(amatch_disconnect),
        x_open: Some(amatch_open),
        x_close: Some(amatch_close),
        x_filter: Some(amatch_filter),
        x_next: Some(amatch_next),
        x_eof: Some(amatch_eof),
        x_column: Some(amatch_column),
        x_rowid: Some(amatch_rowid),
        x_update: Some(amatch_update),
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
pub extern "C" fn sqlite3_amatch_init(db: *mut Sqlite3,
    pz_err_msg_1: *const *mut i8, p_api_1: *const Sqlite3ApiRoutines) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = p_api_1; };
        { let _ = pz_err_msg_1; };

        /// Not used
        (rc =
            unsafe {
                sqlite3_create_module(db,
                    c"approximate_match".as_ptr() as *mut i8 as *const i8,
                    &raw mut amatch_module as *const Sqlite3Module,
                    core::ptr::null_mut())
            });

        /// SQLITE_OMIT_VIRTUALTABLE
        return rc;
    }
}

static a_1: [i8; 65] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 65 as i8, 66 as i8, 67 as i8,
            68 as i8, 69 as i8, 70 as i8, 71 as i8, 72 as i8, 73 as i8,
            74 as i8, 75 as i8, 76 as i8, 77 as i8, 78 as i8, 79 as i8,
            80 as i8, 81 as i8, 82 as i8, 83 as i8, 84 as i8, 85 as i8,
            86 as i8, 87 as i8, 88 as i8, 89 as i8, 90 as i8, 94 as i8,
            97 as i8, 98 as i8, 99 as i8, 100 as i8, 101 as i8, 102 as i8,
            103 as i8, 104 as i8, 105 as i8, 106 as i8, 107 as i8, 108 as i8,
            109 as i8, 110 as i8, 111 as i8, 112 as i8, 113 as i8, 114 as i8,
            115 as i8, 116 as i8, 117 as i8, 118 as i8, 119 as i8, 120 as i8,
            121 as i8, 122 as i8, 126 as i8, 0 as i8];

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
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn isspace(_c: i32)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
