#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod fts5_h;
pub(crate) use crate::fts5_h::*;
mod fts5_int_h;
pub(crate) use crate::fts5_int_h::*;
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite3ext_h;
pub(crate) use crate::sqlite3ext_h::*;

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Expr {
    p_index: *mut Fts5Index,
    p_config: *mut Fts5Config,
    p_root: *mut Fts5ExprNode,
    b_desc: i32,
    n_phrase: i32,
    ap_expr_phrase: *mut *mut Fts5ExprPhrase,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExprNode {
    e_type: i32,
    b_eof: i32,
    b_nomatch: i32,
    i_height: i32,
    x_next: Option<unsafe extern "C" fn(*mut Fts5Expr, *mut Fts5ExprNode, i32,
        i64) -> i32>,
    i_rowid: i64,
    p_near: *mut Fts5ExprNearset,
    n_child: i32,
    ap_child: [*mut Fts5ExprNode; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExprNearset {
    n_near: i32,
    p_colset: *mut Fts5Colset,
    n_phrase: i32,
    ap_phrase: [*mut Fts5ExprPhrase; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExprPhrase {
    p_node: *mut Fts5ExprNode,
    poslist: Fts5Buffer,
    n_term: i32,
    a_term: [Fts5ExprTerm; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExprTerm {
    b_prefix: u8,
    b_first: u8,
    p_term: *mut i8,
    n_query_term: i32,
    n_full_term: i32,
    p_iter: *mut Fts5IndexIter,
    p_synonym: *mut Fts5ExprTerm,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Parse {
    p_config: *mut Fts5Config,
    z_err: *mut i8,
    rc: i32,
    n_phrase: i32,
    ap_phrase: *mut *mut Fts5ExprPhrase,
    p_expr: *mut Fts5ExprNode,
    b_phrase_to_and: i32,
}

extern "C" fn fts5_parse_alloc(t: u64) -> *mut () {
    return unsafe { sqlite3_malloc64(t as Sqlite3Int64 as Sqlite3Uint64) };
}

extern "C" fn fts5_expr_isspace(t: i8) -> i32 {
    return (t as i32 == ' ' as i32 || t as i32 == '\t' as i32 ||
                    t as i32 == '\n' as i32 || t as i32 == '\r' as i32) as i32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_fts5_parse_error(p_parse: &mut Fts5Parse,
    z_fmt: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    if (*p_parse).rc == 0 {
        if !((*p_parse).z_err == core::ptr::null_mut()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseError".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 190,
                    c"pParse->zErr==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        (*p_parse).z_err = unsafe { sqlite3_vmprintf(z_fmt, ap) };
        (*p_parse).rc = 1;
    }
    ();
}

extern "C" fn fts5_expr_get_token(p_parse_1: *mut Fts5Parse,
    pz: &mut *const i8, p_token_1: &mut Fts5Token) -> i32 {
    let mut z: *const i8 = *pz;
    let mut tok: i32 = 0;
    while fts5_expr_isspace(unsafe { *z }) != 0 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    (*p_token_1).p = z;
    (*p_token_1).n = 1;
    '__s1:
        {
        match unsafe { *z } {
            40 => { tok = 10; }
            41 => { tok = 11; }
            123 => { tok = 7; }
            125 => { tok = 8; }
            58 => { tok = 5; }
            44 => { tok = 13; }
            43 => { tok = 14; }
            42 => { tok = 15; }
            45 => { tok = 6; }
            94 => { tok = 12; }
            0 => { tok = 0; }
            34 => {
                {
                    let mut z2: *const i8 = core::ptr::null();
                    tok = 9;
                    {
                        z2 = unsafe { z.offset(1 as isize) };
                        '__b2: loop {
                            if !(1 != 0) { break '__b2; }
                            '__c2: loop {
                                if unsafe { *z2.offset(0 as isize) } as i32 == '\"' as i32 {
                                    {
                                        let __p = &mut z2;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    };
                                    if unsafe { *z2.offset(0 as isize) } as i32 != '\"' as i32 {
                                        break '__b2;
                                    }
                                }
                                if unsafe { *z2.offset(0 as isize) } as i32 ==
                                        '\u{0}' as i32 {
                                    unsafe {
                                        sqlite3_fts5_parse_error(unsafe { &mut *p_parse_1 },
                                            c"unterminated string".as_ptr() as *mut i8 as *const i8)
                                    };
                                    return 0;
                                }
                                break '__c2;
                            }
                            {
                                let __p = &mut z2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    (*p_token_1).n = unsafe { z2.offset_from(z) } as i64 as i32;
                    break '__s1;
                }
                {
                    let mut z2: *const i8 = core::ptr::null();
                    if unsafe {
                                sqlite3_fts5_is_bareword(unsafe { *z.offset(0 as isize) })
                            } == 0 {
                        unsafe {
                            sqlite3_fts5_parse_error(unsafe { &mut *p_parse_1 },
                                c"fts5: syntax error near \"%.1s\"".as_ptr() as *mut i8 as
                                    *const i8, z)
                        };
                        return 0;
                    }
                    tok = 9;
                    {
                        z2 = unsafe { z.offset(1 as isize) };
                        '__b3: loop {
                            if !(unsafe { sqlite3_fts5_is_bareword(unsafe { *z2 }) } !=
                                            0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            {
                                let __p = &mut z2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    (*p_token_1).n = unsafe { z2.offset_from(z) } as i64 as i32;
                    if (*p_token_1).n == 2 &&
                            unsafe {
                                    memcmp((*p_token_1).p as *const (),
                                        c"OR".as_ptr() as *mut i8 as *const (), 2 as u64)
                                } == 0 {
                        tok = 1;
                    }
                    if (*p_token_1).n == 3 &&
                            unsafe {
                                    memcmp((*p_token_1).p as *const (),
                                        c"NOT".as_ptr() as *mut i8 as *const (), 3 as u64)
                                } == 0 {
                        tok = 3;
                    }
                    if (*p_token_1).n == 3 &&
                            unsafe {
                                    memcmp((*p_token_1).p as *const (),
                                        c"AND".as_ptr() as *mut i8 as *const (), 3 as u64)
                                } == 0 {
                        tok = 2;
                    }
                    break '__s1;
                }
            }
            _ => {
                {
                    let mut z2: *const i8 = core::ptr::null();
                    if unsafe {
                                sqlite3_fts5_is_bareword(unsafe { *z.offset(0 as isize) })
                            } == 0 {
                        unsafe {
                            sqlite3_fts5_parse_error(unsafe { &mut *p_parse_1 },
                                c"fts5: syntax error near \"%.1s\"".as_ptr() as *mut i8 as
                                    *const i8, z)
                        };
                        return 0;
                    }
                    tok = 9;
                    {
                        z2 = unsafe { z.offset(1 as isize) };
                        '__b3: loop {
                            if !(unsafe { sqlite3_fts5_is_bareword(unsafe { *z2 }) } !=
                                            0) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            {
                                let __p = &mut z2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    (*p_token_1).n = unsafe { z2.offset_from(z) } as i64 as i32;
                    if (*p_token_1).n == 2 &&
                            unsafe {
                                    memcmp((*p_token_1).p as *const (),
                                        c"OR".as_ptr() as *mut i8 as *const (), 2 as u64)
                                } == 0 {
                        tok = 1;
                    }
                    if (*p_token_1).n == 3 &&
                            unsafe {
                                    memcmp((*p_token_1).p as *const (),
                                        c"NOT".as_ptr() as *mut i8 as *const (), 3 as u64)
                                } == 0 {
                        tok = 3;
                    }
                    if (*p_token_1).n == 3 &&
                            unsafe {
                                    memcmp((*p_token_1).p as *const (),
                                        c"AND".as_ptr() as *mut i8 as *const (), 3 as u64)
                                } == 0 {
                        tok = 2;
                    }
                    break '__s1;
                }
            }
        }
    }
    *pz = unsafe { (*p_token_1).p.offset((*p_token_1).n as isize) };
    return tok;
}

extern "C" fn fts5_parse_free(p: *mut ()) -> () {
    unsafe { sqlite3_free(p) };
}

extern "C" fn assert_expr_depth_ok(rc: i32, p: &Fts5ExprNode) -> () {
    if rc == 0 {
        if (*p).e_type == 4 || (*p).e_type == 9 || (*p).e_type == 0 {
            if !((*p).i_height == 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"assert_expr_depth_ok".as_ptr() as *const i8,
                        c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 169,
                        c"p->iHeight==0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
        } else {
            let mut ii: i32 = 0;
            let mut i_max_child: i32 = 0;
            {
                ii = 0;
                '__b4: loop {
                    if !(ii < (*p).n_child) { break '__b4; }
                    '__c4: loop {
                        let p_child: *mut Fts5ExprNode =
                            unsafe {
                                *((*p).ap_child.as_ptr() as
                                            *mut *mut Fts5ExprNode).offset(ii as isize)
                            };
                        i_max_child =
                            if i_max_child > unsafe { (*p_child).i_height } {
                                i_max_child
                            } else { unsafe { (*p_child).i_height } };
                        assert_expr_depth_ok(0, unsafe { &*p_child });
                        break '__c4;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            if !((*p).i_height == i_max_child + 1) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"assert_expr_depth_ok".as_ptr() as *const i8,
                        c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 178,
                        c"p->iHeight==iMaxChild+1".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
        }
    }
}

extern "C" fn fts5_merge_colset(p_colset_1: &mut Fts5Colset,
    p_merge_1: &Fts5Colset) -> () {
    let mut i_in: i32 = 0;
    let mut i_merge: i32 = 0;
    let mut i_out: i32 = 0;
    while i_in < (*p_colset_1).n_col && i_merge < (*p_merge_1).n_col {
        let i_diff: i32 =
            unsafe {
                    *((*p_colset_1).ai_col.as_ptr() as
                                *mut i32).offset(i_in as isize)
                } -
                unsafe {
                    *((*p_merge_1).ai_col.as_ptr() as
                                *mut i32).offset(i_merge as isize)
                };
        if i_diff == 0 {
            unsafe {
                *((*p_colset_1).ai_col.as_ptr() as
                                *mut i32).offset({
                                    let __p = &mut i_out;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize) =
                    unsafe {
                        *((*p_merge_1).ai_col.as_ptr() as
                                    *mut i32).offset(i_merge as isize)
                    }
            };
            { let __p = &mut i_merge; let __t = *__p; *__p += 1; __t };
            { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
        } else if i_diff > 0 {
            { let __p = &mut i_merge; let __t = *__p; *__p += 1; __t };
        } else { { let __p = &mut i_in; let __t = *__p; *__p += 1; __t }; }
    }
    (*p_colset_1).n_col = i_out;
}

extern "C" fn fts5_clone_colset(p_rc_1: *mut i32, p_orig_1: *const Fts5Colset)
    -> *mut Fts5Colset {
    let mut p_ret: *mut Fts5Colset = core::ptr::null_mut();
    if !(p_orig_1).is_null() {
        let n_byte: Sqlite3Int64 =
            (core::mem::size_of::<i64>() as u64 *
                    ((unsafe { (*p_orig_1).n_col } + 2) / 2) as u64) as
                Sqlite3Int64;
        p_ret =
            unsafe { sqlite3_fts5_malloc_zero(p_rc_1, n_byte) } as
                *mut Fts5Colset;
        if !(p_ret).is_null() {
            unsafe {
                memcpy(p_ret as *mut (), p_orig_1 as *const (), n_byte as u64)
            };
        }
    } else { p_ret = core::ptr::null_mut(); }
    return p_ret;
}

extern "C" fn fts5_parse_set_colset(p_parse_1: *mut Fts5Parse,
    p_node_1: &mut Fts5ExprNode, p_colset_1: *mut Fts5Colset,
    pp_free_1: *mut *mut Fts5Colset) -> () {
    if unsafe { (*p_parse_1).rc } == 0 {
        if !((*p_node_1).e_type == 4 || (*p_node_1).e_type == 9 ||
                                            (*p_node_1).e_type == 2 || (*p_node_1).e_type == 1 ||
                                    (*p_node_1).e_type == 3 || (*p_node_1).e_type == 0) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ParseSetColset".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2203,
                    c"pNode->eType==FTS5_TERM || pNode->eType==FTS5_STRING || pNode->eType==FTS5_AND || pNode->eType==FTS5_OR || pNode->eType==FTS5_NOT || pNode->eType==FTS5_EOF".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if (*p_node_1).e_type == 9 || (*p_node_1).e_type == 4 {
            let p_near: *mut Fts5ExprNearset = (*p_node_1).p_near;
            if !(unsafe { (*p_near).p_colset }).is_null() {
                fts5_merge_colset(unsafe {
                        &mut *unsafe { (*p_near).p_colset }
                    }, unsafe { &*p_colset_1 });
                if unsafe { (*unsafe { (*p_near).p_colset }).n_col } == 0 {
                    (*p_node_1).e_type = 0;
                    (*p_node_1).x_next = None;
                }
            } else if !(unsafe { *pp_free_1 }).is_null() {
                unsafe { (*p_near).p_colset = p_colset_1 };
                unsafe { *pp_free_1 = core::ptr::null_mut() };
            } else {
                unsafe {
                    (*p_near).p_colset =
                        fts5_clone_colset(unsafe { &mut (*p_parse_1).rc },
                            p_colset_1 as *const Fts5Colset)
                };
            }
        } else {
            let mut i: i32 = 0;
            if !((*p_node_1).e_type != 0 || (*p_node_1).n_child == 0) as i32
                        as i64 != 0 {
                unsafe {
                    __assert_rtn(c"fts5ParseSetColset".as_ptr() as *const i8,
                        c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2220,
                        c"pNode->eType!=FTS5_EOF || pNode->nChild==0".as_ptr() as
                                *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            {
                i = 0;
                '__b6: loop {
                    if !(i < (*p_node_1).n_child) { break '__b6; }
                    '__c6: loop {
                        fts5_parse_set_colset(p_parse_1,
                            unsafe {
                                &mut *unsafe {
                                            *((*p_node_1).ap_child.as_ptr() as
                                                        *mut *mut Fts5ExprNode).offset(i as isize)
                                        }
                            }, p_colset_1, pp_free_1);
                        break '__c6;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_set_colset(p_parse: *mut Fts5Parse,
    p_expr: *mut Fts5ExprNode, p_colset: *mut Fts5Colset) -> () {
    let mut p_free: *mut Fts5Colset = p_colset;
    if unsafe { (*unsafe { (*p_parse).p_config }).e_detail } == 1 {
        unsafe {
            sqlite3_fts5_parse_error(unsafe { &mut *p_parse },
                c"fts5: column queries are not supported (detail=none)".as_ptr()
                        as *mut i8 as *const i8)
        };
    } else {
        fts5_parse_set_colset(p_parse, unsafe { &mut *p_expr }, p_colset,
            &mut p_free);
    }
    unsafe { sqlite3_free(p_free as *mut ()) };
}

extern "C" fn fts5_expr_phrase_free(p_phrase_1: *mut Fts5ExprPhrase) -> () {
    if !(p_phrase_1).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b7: loop {
                if !(i < unsafe { (*p_phrase_1).n_term }) { break '__b7; }
                '__c7: loop {
                    let mut p_syn: *mut Fts5ExprTerm = core::ptr::null_mut();
                    let mut p_next: *mut Fts5ExprTerm = core::ptr::null_mut();
                    let p_term: *const Fts5ExprTerm =
                        unsafe {
                                &raw mut *(unsafe { (*p_phrase_1).a_term.as_ptr() } as
                                                *mut Fts5ExprTerm).offset(i as isize)
                            } as *const Fts5ExprTerm;
                    unsafe {
                        sqlite3_free(unsafe { (*p_term).p_term } as *mut ())
                    };
                    unsafe {
                        sqlite3_fts5_iter_close(unsafe { (*p_term).p_iter })
                    };
                    {
                        p_syn = unsafe { (*p_term).p_synonym };
                        '__b8: loop {
                            if !(!(p_syn).is_null()) { break '__b8; }
                            '__c8: loop {
                                p_next = unsafe { (*p_syn).p_synonym };
                                unsafe {
                                    sqlite3_fts5_iter_close(unsafe { (*p_syn).p_iter })
                                };
                                unsafe {
                                    sqlite3_fts5_buffer_free(unsafe {
                                                &raw mut *p_syn.offset(1 as isize)
                                            } as *mut Fts5Buffer)
                                };
                                unsafe { sqlite3_free(p_syn as *mut ()) };
                                break '__c8;
                            }
                            p_syn = p_next;
                        }
                    }
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_phrase_1).poslist.n_space } > 0 {
            unsafe {
                sqlite3_fts5_buffer_free(unsafe {
                        &mut (*p_phrase_1).poslist
                    })
            };
        }
        unsafe { sqlite3_free(p_phrase_1 as *mut ()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_nearset_free(p_near:
        *mut Fts5ExprNearset) -> () {
    if !(p_near).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b9: loop {
                if !(i < unsafe { (*p_near).n_phrase }) { break '__b9; }
                '__c9: loop {
                    fts5_expr_phrase_free(unsafe {
                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                        *mut *mut Fts5ExprPhrase).offset(i as isize)
                        });
                    break '__c9;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_free(unsafe { (*p_near).p_colset } as *mut ()) };
        unsafe { sqlite3_free(p_near as *mut ()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_node_free(p: *mut Fts5ExprNode) -> () {
    if !(p).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b10: loop {
                if !(i < unsafe { (*p).n_child }) { break '__b10; }
                '__c10: loop {
                    sqlite3_fts5_parse_node_free(unsafe {
                            *(unsafe { (*p).ap_child.as_ptr() } as
                                        *mut *mut Fts5ExprNode).offset(i as isize)
                        });
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        sqlite3_fts5_parse_nearset_free(unsafe { (*p).p_near });
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_new(p_config: *mut Fts5Config,
    b_phrase_to_and: i32, i_col: i32, z_expr: *const i8,
    pp_new: &mut *mut Fts5Expr, pz_err: &mut *mut i8) -> i32 {
    let mut s_parse: Fts5Parse = unsafe { core::mem::zeroed() };
    let mut token: Fts5Token = unsafe { core::mem::zeroed() };
    let mut z: *const i8 = z_expr;
    let mut t: i32 = 0;
    let mut p_engine: *mut () = core::ptr::null_mut();
    let mut p_new: *mut Fts5Expr = core::ptr::null_mut();
    *pp_new = core::ptr::null_mut();
    *pz_err = core::ptr::null_mut();
    unsafe {
        memset(&raw mut s_parse as *mut (), 0,
            core::mem::size_of::<Fts5Parse>() as u64)
    };
    s_parse.b_phrase_to_and = b_phrase_to_and;
    p_engine = unsafe { sqlite3_fts5_parser_alloc(Some(fts5_parse_alloc)) };
    if p_engine == core::ptr::null_mut() { return 7; }
    s_parse.p_config = p_config;
    '__b11: loop {
        '__c11: loop {
            t = fts5_expr_get_token(&mut s_parse, &mut z, &mut token);
            unsafe { sqlite3_fts5_parser(p_engine, t, token, &mut s_parse) };
            break '__c11;
        }
        if !(s_parse.rc == 0 && t != 0) { break '__b11; }
    }
    unsafe { sqlite3_fts5_parser_free(p_engine, Some(fts5_parse_free)) };
    if !(!(s_parse.p_expr).is_null() || s_parse.rc != 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ExprNew".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 300,
                c"sParse.pExpr || sParse.rc!=SQLITE_OK".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    assert_expr_depth_ok(s_parse.rc, unsafe { &*s_parse.p_expr });
    if s_parse.rc == 0 && i_col < unsafe { (*p_config).n_col } {
        let n: i32 =
            (core::mem::size_of::<i64>() as u64 * ((1 + 2) / 2) as u64) as
                i32;
        let p_colset: *mut Fts5Colset =
            unsafe {
                    sqlite3_fts5_malloc_zero(&mut s_parse.rc, n as Sqlite3Int64)
                } as *mut Fts5Colset;
        if !(p_colset).is_null() {
            unsafe { (*p_colset).n_col = 1 };
            unsafe {
                *(unsafe { (*p_colset).ai_col.as_ptr() } as
                                *mut i32).offset(0 as isize) = i_col
            };
            sqlite3_fts5_parse_set_colset(&mut s_parse, s_parse.p_expr,
                p_colset);
        }
    }
    if !(s_parse.rc != 0 || s_parse.z_err == core::ptr::null_mut()) as i32 as
                i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ExprNew".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 315,
                c"sParse.rc!=SQLITE_OK || sParse.zErr==0".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    if s_parse.rc == 0 {
        *pp_new =
            {
                p_new =
                    unsafe {
                            sqlite3_malloc64(core::mem::size_of::<Fts5Expr>() as
                                    Sqlite3Uint64)
                        } as *mut Fts5Expr;
                p_new
            };
        if p_new == core::ptr::null_mut() {
            s_parse.rc = 7;
            sqlite3_fts5_parse_node_free(s_parse.p_expr);
        } else {
            unsafe { (*p_new).p_root = s_parse.p_expr };
            unsafe { (*p_new).p_index = core::ptr::null_mut() };
            unsafe { (*p_new).p_config = p_config };
            unsafe { (*p_new).ap_expr_phrase = s_parse.ap_phrase };
            unsafe { (*p_new).n_phrase = s_parse.n_phrase };
            unsafe { (*p_new).b_desc = 0 };
            s_parse.ap_phrase = core::ptr::null_mut();
        }
    } else { sqlite3_fts5_parse_node_free(s_parse.p_expr); }
    unsafe { sqlite3_free(s_parse.ap_phrase as *mut ()) };
    if core::ptr::null_mut() == *pz_err {
        *pz_err = s_parse.z_err;
    } else { unsafe { sqlite3_free(s_parse.z_err as *mut ()) }; }
    return s_parse.rc;
}

extern "C" fn fts5_expr_count_char(z: &[i8]) -> i32 {
    let mut n_ret: i32 = 0;
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b12: loop {
            if !(ii < z.len() as i32) { break '__b12; }
            '__c12: loop {
                if z[ii as usize] as i32 & 192 != 128 {
                    { let __p = &mut n_ret; let __t = *__p; *__p += 1; __t };
                }
                break '__c12;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return n_ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_pattern(p_config: *mut Fts5Config,
    b_glob: i32, mut i_col: i32, z_text: *const i8, pp: *mut *mut Fts5Expr)
    -> i32 {
    let n_text: i64 = unsafe { strlen(z_text) } as i64;
    let z_expr: *mut i8 =
        unsafe {
                sqlite3_malloc64((n_text * 4 as i64 + 1 as i64) as
                        Sqlite3Uint64)
            } as *mut i8;
    let mut rc: i32 = 0;
    if z_expr == core::ptr::null_mut() {
        rc = 7;
    } else {
        let mut a_spec: [i8; 3] = [0; 3];
        let mut i_out: i32 = 0;
        let mut i: i32 = 0;
        let mut i_first: i32 = 0;
        if b_glob == 0 {
            a_spec[0 as usize] = '_' as i32 as i8;
            a_spec[1 as usize] = '%' as i32 as i8;
            a_spec[2 as usize] = 0 as i8;
        } else {
            a_spec[0 as usize] = '*' as i32 as i8;
            a_spec[1 as usize] = '?' as i32 as i8;
            a_spec[2 as usize] = '[' as i32 as i8;
        }
        while i as i64 <= n_text {
            if i as i64 == n_text ||
                            unsafe { *z_text.offset(i as isize) } as i32 ==
                                a_spec[0 as usize] as i32 ||
                        unsafe { *z_text.offset(i as isize) } as i32 ==
                            a_spec[1 as usize] as i32 ||
                    unsafe { *z_text.offset(i as isize) } as i32 ==
                        a_spec[2 as usize] as i32 {
                if fts5_expr_count_char(unsafe {
                                let __p =
                                    unsafe { &*z_text.offset(i_first as isize) } as *const i8;
                                if __p.is_null() {
                                    &[]
                                } else {
                                    core::slice::from_raw_parts(__p, (i - i_first) as usize)
                                }
                            }) >= 3 {
                    let mut jj: i32 = 0;
                    unsafe {
                        *z_expr.offset({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = '\"' as i32 as i8
                    };
                    {
                        jj = i_first;
                        '__b14: loop {
                            if !(jj < i) { break '__b14; }
                            '__c14: loop {
                                unsafe {
                                    *z_expr.offset({
                                                        let __p = &mut i_out;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = unsafe { *z_text.offset(jj as isize) } as i8
                                };
                                if unsafe { *z_text.offset(jj as isize) } as i32 ==
                                        '\"' as i32 {
                                    unsafe {
                                        *z_expr.offset({
                                                            let __p = &mut i_out;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize) = '\"' as i32 as i8
                                    };
                                }
                                break '__c14;
                            }
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        *z_expr.offset({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = '\"' as i32 as i8
                    };
                    unsafe {
                        *z_expr.offset({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = ' ' as i32 as i8
                    };
                }
                if unsafe { *z_text.offset(i as isize) } as i32 ==
                        a_spec[2 as usize] as i32 {
                    i += 2;
                    if unsafe { *z_text.offset((i - 1) as isize) } as i32 ==
                            '^' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                    while (i as i64) < n_text &&
                            unsafe { *z_text.offset(i as isize) } as i32 != ']' as i32 {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                i_first = i + 1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
        if i_out > 0 {
            let mut b_and: i32 = 0;
            if unsafe { (*p_config).e_detail } != 0 {
                b_and = 1;
                if unsafe { (*p_config).e_detail } == 1 {
                    i_col = unsafe { (*p_config).n_col };
                }
            }
            unsafe { *z_expr.offset(i_out as isize) = '\u{0}' as i32 as i8 };
            rc =
                sqlite3_fts5_expr_new(p_config, b_and, i_col,
                    z_expr as *const i8, unsafe { &mut *pp },
                    unsafe { &mut *unsafe { (*p_config).pz_errmsg } });
        } else { unsafe { *pp = core::ptr::null_mut() }; }
        unsafe { sqlite3_free(z_expr as *mut ()) };
    }
    return rc;
}

extern "C" fn fts5_expr_near_init_all(p_expr_1: &Fts5Expr,
    p_node_1: &mut Fts5ExprNode) -> i32 {
    let p_near: *const Fts5ExprNearset =
        (*p_node_1).p_near as *const Fts5ExprNearset;
    let mut i: i32 = 0;
    if !((*p_node_1).b_nomatch == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNearInitAll".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 964,
                c"pNode->bNomatch==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        i = 0;
        '__b16: loop {
            if !(i < unsafe { (*p_near).n_phrase }) { break '__b16; }
            '__c16: loop {
                let p_phrase: *mut Fts5ExprPhrase =
                    unsafe {
                        *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                    *mut *mut Fts5ExprPhrase).offset(i as isize)
                    };
                if unsafe { (*p_phrase).n_term } == 0 {
                    (*p_node_1).b_eof = 1;
                    return 0;
                } else {
                    let mut j: i32 = 0;
                    {
                        j = 0;
                        '__b17: loop {
                            if !(j < unsafe { (*p_phrase).n_term }) { break '__b17; }
                            '__c17: loop {
                                let p_term: *mut Fts5ExprTerm =
                                    unsafe {
                                        &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                        *mut Fts5ExprTerm).offset(j as isize)
                                    };
                                let mut p: *mut Fts5ExprTerm = core::ptr::null_mut();
                                let mut b_hit: i32 = 0;
                                {
                                    p = p_term;
                                    '__b18: loop {
                                        if !(!(p).is_null()) { break '__b18; }
                                        '__c18: loop {
                                            let mut rc: i32 = 0;
                                            if !(unsafe { (*p).p_iter }).is_null() {
                                                unsafe { sqlite3_fts5_iter_close(unsafe { (*p).p_iter }) };
                                                unsafe { (*p).p_iter = core::ptr::null_mut() };
                                            }
                                            rc =
                                                unsafe {
                                                    sqlite3_fts5_index_query((*p_expr_1).p_index,
                                                        unsafe { (*p).p_term } as *const i8,
                                                        unsafe { (*p).n_query_term },
                                                        if unsafe { (*p_term).b_prefix } != 0 { 1 } else { 0 } |
                                                            if (*p_expr_1).b_desc != 0 { 2 } else { 0 },
                                                        unsafe { (*p_near).p_colset }, unsafe { &mut (*p).p_iter })
                                                };
                                            if !((rc == 0) as i32 ==
                                                                    (unsafe { (*p).p_iter } != core::ptr::null_mut()) as i32) as
                                                            i32 as i64 != 0 {
                                                unsafe {
                                                    __assert_rtn(c"fts5ExprNearInitAll".as_ptr() as *const i8,
                                                        c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 990,
                                                        c"(rc==SQLITE_OK)==(p->pIter!=0)".as_ptr() as *mut i8 as
                                                            *const i8)
                                                }
                                            } else { { let _ = 0; } };
                                            if rc != 0 { return rc; }
                                            if 0 == unsafe { (*unsafe { (*p).p_iter }).b_eof } as i32 {
                                                b_hit = 1;
                                            }
                                            break '__c18;
                                        }
                                        p = unsafe { (*p).p_synonym };
                                    }
                                }
                                if b_hit == 0 { (*p_node_1).b_eof = 1; return 0; }
                                break '__c17;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                break '__c16;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_node_1).b_eof = 0;
    return 0;
}

extern "C" fn fts5_expr_set_eof(p_node_1: &mut Fts5ExprNode) -> () {
    let mut i: i32 = 0;
    (*p_node_1).b_eof = 1;
    (*p_node_1).b_nomatch = 0;
    {
        i = 0;
        '__b19: loop {
            if !(i < (*p_node_1).n_child) { break '__b19; }
            '__c19: loop {
                fts5_expr_set_eof(unsafe {
                        &mut *unsafe {
                                    *((*p_node_1).ap_child.as_ptr() as
                                                *mut *mut Fts5ExprNode).offset(i as isize)
                                }
                    });
                break '__c19;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn fts5_expr_synonym_rowid(p_term_1: *mut Fts5ExprTerm,
    b_desc_1: i32, pb_eof_1: *mut i32) -> i64 {
    let mut i_ret: i64 = 0 as i64;
    let mut b_ret_valid: i32 = 0;
    let mut p: *const Fts5ExprTerm = core::ptr::null();
    if (p_term_1).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprSynonymRowid".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 502,
                c"pTerm".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if (unsafe { (*p_term_1).p_synonym }).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprSynonymRowid".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 503,
                c"pTerm->pSynonym".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(b_desc_1 == 0 || b_desc_1 == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprSynonymRowid".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 504,
                c"bDesc==0 || bDesc==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    {
        p = p_term_1;
        '__b20: loop {
            if !(!(p).is_null()) { break '__b20; }
            '__c20: loop {
                if 0 == unsafe { (*unsafe { (*p).p_iter }).b_eof } as i32 {
                    let i_rowid: i64 =
                        unsafe { (*unsafe { (*p).p_iter }).i_rowid };
                    if b_ret_valid == 0 || b_desc_1 != (i_rowid < i_ret) as i32
                        {
                        i_ret = i_rowid;
                        b_ret_valid = 1;
                    }
                }
                break '__c20;
            }
            p = unsafe { (*p).p_synonym };
        }
    }
    if !(pb_eof_1).is_null() && b_ret_valid == 0 { unsafe { *pb_eof_1 = 1 }; }
    return i_ret;
}

extern "C" fn fts5_expr_synonym_advanceto(p_term_1: *mut Fts5ExprTerm,
    b_desc_1: i32, pi_last_1: &mut i64, p_rc_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    let i_last: i64 = *pi_last_1;
    let mut p: *const Fts5ExprTerm = core::ptr::null();
    let mut b_eof: i32 = 0;
    {
        p = p_term_1;
        '__b21: loop {
            if !(rc == 0 && !(p).is_null()) { break '__b21; }
            '__c21: loop {
                if unsafe { (*unsafe { (*p).p_iter }).b_eof } as i32 == 0 {
                    let i_rowid: i64 =
                        unsafe { (*unsafe { (*p).p_iter }).i_rowid };
                    if b_desc_1 == 0 && i_last > i_rowid ||
                            b_desc_1 != 0 && i_last < i_rowid {
                        rc =
                            unsafe {
                                sqlite3_fts5_iter_next_from(unsafe { (*p).p_iter }, i_last)
                            };
                    }
                }
                break '__c21;
            }
            p = unsafe { (*p).p_synonym };
        }
    }
    if rc != 0 {
        *p_rc_1 = rc;
        b_eof = 1;
    } else {
        *pi_last_1 = fts5_expr_synonym_rowid(p_term_1, b_desc_1, &mut b_eof);
    }
    return b_eof;
}

extern "C" fn fts5_expr_advanceto(p_iter_1: *mut Fts5IndexIter, b_desc_1: i32,
    pi_last_1: &mut i64, p_rc_1: &mut i32, pb_eof_1: &mut i32) -> i32 {
    let i_last: i64 = *pi_last_1;
    let mut i_rowid: i64 = 0 as i64;
    i_rowid = unsafe { (*p_iter_1).i_rowid };
    if b_desc_1 == 0 && i_last > i_rowid || b_desc_1 != 0 && i_last < i_rowid
        {
        let rc: i32 =
            unsafe { sqlite3_fts5_iter_next_from(p_iter_1, i_last) };
        if rc != 0 || unsafe { (*p_iter_1).b_eof } != 0 {
            *p_rc_1 = rc;
            *pb_eof_1 = 1;
            return 1;
        }
        i_rowid = unsafe { (*p_iter_1).i_rowid };
        if !(b_desc_1 == 0 && i_rowid >= i_last ||
                                b_desc_1 == 1 && i_rowid <= i_last) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ExprAdvanceto".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 861,
                    c"(bDesc==0 && iRowid>=iLast) || (bDesc==1 && iRowid<=iLast)".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    *pi_last_1 = i_rowid;
    return 0;
}

extern "C" fn fts5_expr_synonym_list(p_term_1: *mut Fts5ExprTerm,
    i_rowid_1: i64, p_buf_1: *mut Fts5Buffer, pa: &mut *mut u8, pn: &mut i32)
    -> i32 {
    let mut a_static: [Fts5PoslistReader; 4] = unsafe { core::mem::zeroed() };
    let mut a_iter: *mut Fts5PoslistReader = core::ptr::null_mut();
    let mut n_iter: i32 = 0;
    let mut n_alloc: i32 = 0;
    let mut rc: i32 = 0;
    let mut p: *const Fts5ExprTerm = core::ptr::null();
    let mut p_iter: *const Fts5IndexIter = core::ptr::null();
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut a_new: *mut Fts5PoslistReader = core::ptr::null_mut();
    let mut writer: Fts5PoslistWriter = unsafe { core::mem::zeroed() };
    let mut i_prev: i64 = 0 as i64;
    let mut i: i32 = 0;
    let mut i_min: i64 = 0 as i64;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s23:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    if a_iter !=
                            &raw mut a_static[0 as usize] as *mut Fts5PoslistReader {
                        __state = 57;
                    } else { __state = 56; }
                }
                3 => {
                    a_iter =
                        &raw mut a_static[0 as usize] as *mut Fts5PoslistReader;
                    __state = 4;
                }
                4 => { n_iter = 0; __state = 5; }
                5 => { n_alloc = 4; __state = 6; }
                6 => { rc = 0; __state = 7; }
                7 => { __state = 8; }
                8 => {
                    if (unsafe { (*p_term_1).p_synonym }).is_null() as i32 as
                                i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5ExprSynonymList".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 535,
                                c"pTerm->pSynonym".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 9;
                }
                9 => { p = p_term_1; __state = 11; }
                10 => {
                    if n_iter == 1 { __state = 32; } else { __state = 33; }
                }
                11 => {
                    if !(p).is_null() { __state = 12; } else { __state = 10; }
                }
                12 => {
                    p_iter = unsafe { (*p).p_iter } as *const Fts5IndexIter;
                    __state = 14;
                }
                13 => { p = unsafe { (*p).p_synonym }; __state = 11; }
                14 => {
                    if unsafe { (*p_iter).b_eof } as i32 == 0 &&
                            unsafe { (*p_iter).i_rowid } == i_rowid_1 {
                        __state = 15;
                    } else { __state = 13; }
                }
                15 => {
                    if unsafe { (*p_iter).n_data } == 0 {
                        __state = 17;
                    } else { __state = 16; }
                }
                16 => {
                    if n_iter == n_alloc {
                        __state = 19;
                    } else { __state = 18; }
                }
                17 => { __state = 13; }
                18 => {
                    unsafe {
                        sqlite3_fts5_poslist_reader_init(unsafe {
                                (*p_iter).p_data
                            }, unsafe { (*p_iter).n_data },
                            unsafe { &mut *a_iter.offset(n_iter as isize) })
                    };
                    __state = 29;
                }
                19 => {
                    n_byte =
                        (core::mem::size_of::<Fts5PoslistReader>() as u64 *
                                    n_alloc as u64 * 2 as u64) as Sqlite3Int64;
                    __state = 20;
                }
                20 => {
                    a_new =
                        unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                            *mut Fts5PoslistReader;
                    __state = 21;
                }
                21 => {
                    if a_new == core::ptr::null_mut() {
                        __state = 23;
                    } else { __state = 22; }
                }
                22 => {
                    unsafe {
                        memcpy(a_new as *mut (), a_iter as *const (),
                            core::mem::size_of::<Fts5PoslistReader>() as u64 *
                                n_iter as u64)
                    };
                    __state = 25;
                }
                23 => { rc = 7; __state = 24; }
                24 => { __state = 2; }
                25 => { n_alloc = n_alloc * 2; __state = 26; }
                26 => {
                    if a_iter !=
                            &raw mut a_static[0 as usize] as *mut Fts5PoslistReader {
                        __state = 28;
                    } else { __state = 27; }
                }
                27 => { a_iter = a_new; __state = 18; }
                28 => {
                    unsafe { sqlite3_free(a_iter as *mut ()) };
                    __state = 27;
                }
                29 => {
                    if !(unsafe { (*a_iter.offset(n_iter as isize)).b_eof } as
                                                i32 == 0) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5ExprSynonymList".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 553,
                                c"aIter[nIter].bEof==0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 30;
                }
                30 => {
                    { let __p = &mut n_iter; let __t = *__p; *__p += 1; __t };
                    __state = 13;
                }
                31 => { __state = 2; }
                32 => {
                    *pa = unsafe { (*a_iter.offset(0 as isize)).a } as *mut u8;
                    __state = 34;
                }
                33 => {
                    writer = Fts5PoslistWriter { i_prev: 0 };
                    __state = 35;
                }
                34 => {
                    *pn = unsafe { (*a_iter.offset(0 as isize)).n };
                    __state = 31;
                }
                35 => { i_prev = -1 as i64; __state = 36; }
                36 => {
                    unsafe { sqlite3_fts5_buffer_zero(p_buf_1) };
                    __state = 37;
                }
                37 => { if 1 != 0 { __state = 39; } else { __state = 38; } }
                38 => { if rc == 0 { __state = 54; } else { __state = 31; } }
                39 => { __state = 40; }
                40 => {
                    i_min = 4294967295u32 as i64 | (2147483647 as i64) << 32;
                    __state = 41;
                }
                41 => { i = 0; __state = 43; }
                42 => {
                    if i_min == 4294967295u32 as i64 | (2147483647 as i64) << 32
                            || rc != 0 {
                        __state = 52;
                    } else { __state = 51; }
                }
                43 => {
                    if i < n_iter { __state = 44; } else { __state = 42; }
                }
                44 => {
                    if unsafe { (*a_iter.offset(i as isize)).b_eof } as i32 == 0
                        {
                        __state = 46;
                    } else { __state = 45; }
                }
                45 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 43;
                }
                46 => {
                    if unsafe { (*a_iter.offset(i as isize)).i_pos } == i_prev {
                        __state = 48;
                    } else { __state = 47; }
                }
                47 => {
                    if unsafe { (*a_iter.offset(i as isize)).i_pos } < i_min {
                        __state = 50;
                    } else { __state = 45; }
                }
                48 => {
                    if unsafe {
                                sqlite3_fts5_poslist_reader_next(unsafe {
                                        &mut *a_iter.offset(i as isize)
                                    })
                            } != 0 {
                        __state = 49;
                    } else { __state = 47; }
                }
                49 => { __state = 45; }
                50 => {
                    i_min = unsafe { (*a_iter.offset(i as isize)).i_pos };
                    __state = 45;
                }
                51 => {
                    rc =
                        unsafe {
                            sqlite3_fts5_poslist_writer_append(p_buf_1, &mut writer,
                                i_min)
                        };
                    __state = 53;
                }
                52 => { __state = 38; }
                53 => { i_prev = i_min; __state = 37; }
                54 => { *pa = unsafe { (*p_buf_1).p }; __state = 55; }
                55 => { *pn = unsafe { (*p_buf_1).n }; __state = 31; }
                56 => { return rc; }
                57 => {
                    unsafe { sqlite3_free(a_iter as *mut ()) };
                    __state = 56;
                }
                _ => {}
            }
        }
    }
    unreachable!();
}

extern "C" fn fts5_expr_phrase_is_match(p_node_1: &Fts5ExprNode,
    p_phrase_1: &mut Fts5ExprPhrase, pb_match_1: &mut i32) -> i32 {
    let mut writer: Fts5PoslistWriter = unsafe { core::mem::zeroed() };
    let mut a_static: [Fts5PoslistReader; 4] = unsafe { core::mem::zeroed() };
    let mut a_iter: *mut Fts5PoslistReader = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut b_first: i32 = 0;
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut p_term: *mut Fts5ExprTerm = core::ptr::null_mut();
    let mut n: i32 = 0;
    let mut b_flag: i32 = 0;
    let mut a: *mut u8 = core::ptr::null_mut();
    let mut buf: Fts5Buffer = unsafe { core::mem::zeroed() };
    let mut b_match: i32 = 0;
    let mut i_pos: i64 = 0 as i64;
    let mut p_pos: *mut Fts5PoslistReader = core::ptr::null_mut();
    let mut i_adj: i64 = 0 as i64;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s25:
            {
            match __state {
                0 => {
                    writer = Fts5PoslistWriter { i_prev: 0 };
                    __state = 3;
                }
                2 => {
                    *pb_match_1 = ((*p_phrase_1).poslist.n > 0) as i32;
                    __state = 63;
                }
                3 => { __state = 4; }
                4 => {
                    a_iter =
                        &raw mut a_static[0 as usize] as *mut Fts5PoslistReader;
                    __state = 5;
                }
                5 => { __state = 6; }
                6 => { rc = 0; __state = 7; }
                7 => {
                    b_first =
                        unsafe {
                                (*((*p_phrase_1).a_term.as_ptr() as
                                                *mut Fts5ExprTerm).offset(0 as isize)).b_first
                            } as i32;
                    __state = 8;
                }
                8 => {
                    unsafe {
                        sqlite3_fts5_buffer_zero(&mut (*p_phrase_1).poslist)
                    };
                    __state = 9;
                }
                9 => {
                    if (*p_phrase_1).n_term >
                            (core::mem::size_of::<[Fts5PoslistReader; 4]>() as u64 /
                                    core::mem::size_of::<Fts5PoslistReader>() as u64) as i32 {
                        __state = 11;
                    } else { __state = 10; }
                }
                10 => {
                    unsafe {
                        memset(a_iter as *mut (), 0,
                            core::mem::size_of::<Fts5PoslistReader>() as u64 *
                                (*p_phrase_1).n_term as u64)
                    };
                    __state = 15;
                }
                11 => {
                    n_byte =
                        (core::mem::size_of::<Fts5PoslistReader>() as u64 *
                                (*p_phrase_1).n_term as u64) as Sqlite3Int64;
                    __state = 12;
                }
                12 => {
                    a_iter =
                        unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                            *mut Fts5PoslistReader;
                    __state = 13;
                }
                13 => {
                    if (a_iter).is_null() as i32 != 0 {
                        __state = 14;
                    } else { __state = 10; }
                }
                14 => { return 7; }
                15 => { i = 0; __state = 17; }
                16 => { if 1 != 0 { __state = 38; } else { __state = 37; } }
                17 => {
                    if i < (*p_phrase_1).n_term {
                        __state = 18;
                    } else { __state = 16; }
                }
                18 => {
                    p_term =
                        unsafe {
                            &mut *((*p_phrase_1).a_term.as_ptr() as
                                            *mut Fts5ExprTerm).offset(i as isize)
                        };
                    __state = 20;
                }
                19 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 17;
                }
                20 => { n = 0; __state = 21; }
                21 => { b_flag = 0; __state = 22; }
                22 => { a = core::ptr::null_mut(); __state = 23; }
                23 => {
                    if !(unsafe { (*p_term).p_synonym }).is_null() {
                        __state = 25;
                    } else { __state = 26; }
                }
                24 => {
                    unsafe {
                        sqlite3_fts5_poslist_reader_init(a as *const u8, n,
                            unsafe { &mut *a_iter.offset(i as isize) })
                    };
                    __state = 34;
                }
                25 => {
                    buf =
                        Fts5Buffer { p: core::ptr::null_mut(), n: 0, n_space: 0 };
                    __state = 27;
                }
                26 => {
                    a =
                        unsafe { (*unsafe { (*p_term).p_iter }).p_data } as *mut u8;
                    __state = 33;
                }
                27 => {
                    rc =
                        fts5_expr_synonym_list(p_term, (*p_node_1).i_rowid,
                            &mut buf, &mut a, &mut n);
                    __state = 28;
                }
                28 => { if rc != 0 { __state = 30; } else { __state = 29; } }
                29 => {
                    if a == buf.p { __state = 32; } else { __state = 24; }
                }
                30 => { unsafe { sqlite3_free(a as *mut ()) }; __state = 31; }
                31 => { __state = 2; }
                32 => { b_flag = 1; __state = 24; }
                33 => {
                    n = unsafe { (*unsafe { (*p_term).p_iter }).n_data };
                    __state = 24;
                }
                34 => {
                    unsafe {
                        (*a_iter.offset(i as isize)).b_flag = b_flag as u8
                    };
                    __state = 35;
                }
                35 => {
                    if unsafe { (*a_iter.offset(i as isize)).b_eof } != 0 {
                        __state = 36;
                    } else { __state = 19; }
                }
                36 => { __state = 2; }
                37 => { __state = 2; }
                38 => { __state = 39; }
                39 => {
                    i_pos = unsafe { (*a_iter.offset(0 as isize)).i_pos };
                    __state = 40;
                }
                40 => { b_match = 1; __state = 43; }
                41 => {
                    if b_first == 0 || (i_pos & 2147483647 as i64) as i32 == 0 {
                        __state = 56;
                    } else { __state = 55; }
                }
                42 => {
                    if b_match == 0 { __state = 40; } else { __state = 41; }
                }
                43 => { i = 0; __state = 44; }
                44 => {
                    if i < (*p_phrase_1).n_term {
                        __state = 45;
                    } else { __state = 42; }
                }
                45 => {
                    p_pos = unsafe { a_iter.offset(i as isize) };
                    __state = 47;
                }
                46 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 44;
                }
                47 => { i_adj = i_pos + i as i64; __state = 48; }
                48 => {
                    if unsafe { (*p_pos).i_pos } != i_adj {
                        __state = 49;
                    } else { __state = 46; }
                }
                49 => { b_match = 0; __state = 50; }
                50 => {
                    if unsafe { (*p_pos).i_pos } < i_adj {
                        __state = 52;
                    } else { __state = 51; }
                }
                51 => {
                    if unsafe { (*p_pos).i_pos } > i_adj {
                        __state = 54;
                    } else { __state = 46; }
                }
                52 => {
                    if unsafe { sqlite3_fts5_poslist_reader_next(p_pos) } != 0 {
                        __state = 53;
                    } else { __state = 50; }
                }
                53 => { __state = 2; }
                54 => {
                    i_pos = unsafe { (*p_pos).i_pos } - i as i64;
                    __state = 46;
                }
                55 => { i = 0; __state = 59; }
                56 => {
                    rc =
                        unsafe {
                            sqlite3_fts5_poslist_writer_append(&mut (*p_phrase_1).poslist,
                                &mut writer, i_pos)
                        };
                    __state = 57;
                }
                57 => { if rc != 0 { __state = 58; } else { __state = 55; } }
                58 => { __state = 2; }
                59 => {
                    if i < (*p_phrase_1).n_term {
                        __state = 60;
                    } else { __state = 16; }
                }
                60 => {
                    if unsafe {
                                sqlite3_fts5_poslist_reader_next(unsafe {
                                        &mut *a_iter.offset(i as isize)
                                    })
                            } != 0 {
                        __state = 62;
                    } else { __state = 61; }
                }
                61 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 59;
                }
                62 => { __state = 2; }
                63 => { i = 0; __state = 65; }
                64 => {
                    if a_iter !=
                            &raw mut a_static[0 as usize] as *mut Fts5PoslistReader {
                        __state = 70;
                    } else { __state = 69; }
                }
                65 => {
                    if i < (*p_phrase_1).n_term {
                        __state = 66;
                    } else { __state = 64; }
                }
                66 => {
                    if unsafe { (*a_iter.offset(i as isize)).b_flag } != 0 {
                        __state = 68;
                    } else { __state = 67; }
                }
                67 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 65;
                }
                68 => {
                    unsafe {
                        sqlite3_free(unsafe { (*a_iter.offset(i as isize)).a } as
                                    *mut u8 as *mut ())
                    };
                    __state = 67;
                }
                69 => { return rc; }
                70 => {
                    unsafe { sqlite3_free(a_iter as *mut ()) };
                    __state = 69;
                }
                _ => {}
            }
        }
    }
    unreachable!();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5NearTrimmer {
    reader: Fts5LookaheadReader,
    writer: Fts5PoslistWriter,
    p_out: *mut Fts5Buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5LookaheadReader {
    a: *const u8,
    n: i32,
    i: i32,
    i_pos: i64,
    i_lookahead: i64,
}

extern "C" fn fts5_lookahead_reader_next(p: &mut Fts5LookaheadReader) -> i32 {
    (*p).i_pos = (*p).i_lookahead;
    if unsafe {
                sqlite3_fts5_poslist_next64((*p).a, (*p).n, &mut (*p).i,
                    &mut (*p).i_lookahead)
            } != 0 {
        (*p).i_lookahead = (1 as i64) << 62;
    }
    return ((*p).i_pos == (1 as i64) << 62) as i32;
}

extern "C" fn fts5_lookahead_reader_init(a: *const u8, n: i32,
    p: *mut Fts5LookaheadReader) -> i32 {
    unsafe {
        memset(p as *mut (), 0,
            core::mem::size_of::<Fts5LookaheadReader>() as u64)
    };
    unsafe { (*p).a = a };
    unsafe { (*p).n = n };
    fts5_lookahead_reader_next(unsafe { &mut *p });
    return fts5_lookahead_reader_next(unsafe { &mut *p });
}

extern "C" fn fts5_expr_near_is_match(p_rc_1: &mut i32,
    p_near_1: &Fts5ExprNearset) -> i32 {
    let mut a_static: [Fts5NearTrimmer; 4] = unsafe { core::mem::zeroed() };
    let mut a: *mut Fts5NearTrimmer = core::ptr::null_mut();
    let mut ap_phrase: *const *mut Fts5ExprPhrase = core::ptr::null();
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut b_match: i32 = 0;
    let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut p_poslist: *mut Fts5Buffer = core::ptr::null_mut();
    let mut i_adv: i32 = 0;
    let mut i_min: i64 = 0 as i64;
    let mut i_max: i64 = 0 as i64;
    let mut p_pos: *mut Fts5LookaheadReader = core::ptr::null_mut();
    let mut i_pos: i64 = 0 as i64;
    let mut p_writer: *mut Fts5PoslistWriter = core::ptr::null_mut();
    let mut b_ret: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s27:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    b_ret =
                        (unsafe { (*unsafe { (*a.offset(0 as isize)).p_out }).n } >
                                0) as i32;
                    __state = 60;
                }
                3 => {
                    a = &raw mut a_static[0 as usize] as *mut Fts5NearTrimmer;
                    __state = 4;
                }
                4 => {
                    ap_phrase =
                        (*p_near_1).ap_phrase.as_ptr() as *mut *mut Fts5ExprPhrase
                            as *const *mut Fts5ExprPhrase;
                    __state = 5;
                }
                5 => { __state = 6; }
                6 => { rc = *p_rc_1; __state = 7; }
                7 => { __state = 8; }
                8 => {
                    if !((*p_near_1).n_phrase > 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5ExprNearIsMatch".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 752,
                                c"pNear->nPhrase>1".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    __state = 9;
                }
                9 => {
                    if (*p_near_1).n_phrase >
                            (core::mem::size_of::<[Fts5NearTrimmer; 4]>() as u64 /
                                    core::mem::size_of::<Fts5NearTrimmer>() as u64) as i32 {
                        __state = 11;
                    } else { __state = 12; }
                }
                10 => { if rc != 0 { __state = 15; } else { __state = 14; } }
                11 => {
                    n_byte =
                        (core::mem::size_of::<Fts5NearTrimmer>() as u64 *
                                (*p_near_1).n_phrase as u64) as Sqlite3Int64;
                    __state = 13;
                }
                12 => {
                    unsafe {
                        memset(&raw mut a_static[0 as usize] as *mut Fts5NearTrimmer
                                as *mut (), 0,
                            core::mem::size_of::<[Fts5NearTrimmer; 4]>() as u64)
                    };
                    __state = 10;
                }
                13 => {
                    a =
                        unsafe { sqlite3_fts5_malloc_zero(&mut rc, n_byte) } as
                            *mut Fts5NearTrimmer;
                    __state = 10;
                }
                14 => { i = 0; __state = 18; }
                15 => { *p_rc_1 = rc; __state = 16; }
                16 => { return 0; }
                17 => { if 1 != 0 { __state = 25; } else { __state = 24; } }
                18 => {
                    if i < (*p_near_1).n_phrase {
                        __state = 19;
                    } else { __state = 17; }
                }
                19 => {
                    p_poslist =
                        unsafe {
                            &mut (*unsafe { *ap_phrase.offset(i as isize) }).poslist
                        };
                    __state = 21;
                }
                20 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 18;
                }
                21 => {
                    fts5_lookahead_reader_init(unsafe { (*p_poslist).p } as
                            *const u8, unsafe { (*p_poslist).n },
                        unsafe { &mut (*a.offset(i as isize)).reader });
                    __state = 22;
                }
                22 => { unsafe { (*p_poslist).n = 0 }; __state = 23; }
                23 => {
                    unsafe { (*a.offset(i as isize)).p_out = p_poslist };
                    __state = 20;
                }
                24 => { __state = 2; }
                25 => { __state = 26; }
                26 => { __state = 27; }
                27 => { __state = 28; }
                28 => {
                    i_max = unsafe { (*a.offset(0 as isize)).reader.i_pos };
                    __state = 29;
                }
                29 => { b_match = 1; __state = 32; }
                30 => { i = 0; __state = 45; }
                31 => {
                    if b_match == 0 { __state = 29; } else { __state = 30; }
                }
                32 => { i = 0; __state = 33; }
                33 => {
                    if i < (*p_near_1).n_phrase {
                        __state = 34;
                    } else { __state = 31; }
                }
                34 => {
                    p_pos = unsafe { &mut (*a.offset(i as isize)).reader };
                    __state = 36;
                }
                35 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 33;
                }
                36 => {
                    i_min =
                        i_max -
                                unsafe {
                                        (*unsafe {
                                                        *((*p_near_1).ap_phrase.as_ptr() as
                                                                    *mut *mut Fts5ExprPhrase).offset(i as isize)
                                                    }).n_term
                                    } as i64 - (*p_near_1).n_near as i64;
                    __state = 37;
                }
                37 => {
                    if unsafe { (*p_pos).i_pos } < i_min ||
                            unsafe { (*p_pos).i_pos } > i_max {
                        __state = 38;
                    } else { __state = 35; }
                }
                38 => { b_match = 0; __state = 39; }
                39 => {
                    if unsafe { (*p_pos).i_pos } < i_min {
                        __state = 41;
                    } else { __state = 40; }
                }
                40 => {
                    if unsafe { (*p_pos).i_pos } > i_max {
                        __state = 43;
                    } else { __state = 35; }
                }
                41 => {
                    if fts5_lookahead_reader_next(unsafe { &mut *p_pos }) != 0 {
                        __state = 42;
                    } else { __state = 39; }
                }
                42 => { __state = 2; }
                43 => { i_max = unsafe { (*p_pos).i_pos }; __state = 35; }
                44 => { i_adv = 0; __state = 51; }
                45 => {
                    if i < (*p_near_1).n_phrase {
                        __state = 46;
                    } else { __state = 44; }
                }
                46 => {
                    i_pos = unsafe { (*a.offset(i as isize)).reader.i_pos };
                    __state = 48;
                }
                47 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 45;
                }
                48 => {
                    p_writer = unsafe { &mut (*a.offset(i as isize)).writer };
                    __state = 49;
                }
                49 => {
                    if unsafe { (*unsafe { (*a.offset(i as isize)).p_out }).n }
                                == 0 || i_pos != unsafe { (*p_writer).i_prev } {
                        __state = 50;
                    } else { __state = 47; }
                }
                50 => {
                    unsafe {
                        sqlite3_fts5_poslist_safe_append(unsafe {
                                (*a.offset(i as isize)).p_out
                            }, unsafe { &mut (*p_writer).i_prev }, i_pos)
                    };
                    __state = 47;
                }
                51 => {
                    i_min =
                        unsafe { (*a.offset(0 as isize)).reader.i_lookahead };
                    __state = 52;
                }
                52 => { i = 0; __state = 54; }
                53 => {
                    if fts5_lookahead_reader_next(unsafe {
                                    &mut (*a.offset(i_adv as isize)).reader
                                }) != 0 {
                        __state = 59;
                    } else { __state = 17; }
                }
                54 => {
                    if i < (*p_near_1).n_phrase {
                        __state = 55;
                    } else { __state = 53; }
                }
                55 => {
                    if unsafe { (*a.offset(i as isize)).reader.i_lookahead } <
                            i_min {
                        __state = 57;
                    } else { __state = 56; }
                }
                56 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 54;
                }
                57 => {
                    i_min =
                        unsafe { (*a.offset(i as isize)).reader.i_lookahead };
                    __state = 58;
                }
                58 => { i_adv = i; __state = 56; }
                59 => { __state = 2; }
                60 => { *p_rc_1 = rc; __state = 61; }
                61 => {
                    if a !=
                            &raw mut a_static[0 as usize] as *mut Fts5NearTrimmer {
                        __state = 63;
                    } else { __state = 62; }
                }
                62 => { return b_ret; }
                63 => { unsafe { sqlite3_free(a as *mut ()) }; __state = 62; }
                _ => {}
            }
        }
    }
    unreachable!();
}

extern "C" fn fts5_expr_near_test(p_rc_1: *mut i32, p_expr_1: &Fts5Expr,
    p_node_1: *mut Fts5ExprNode) -> i32 {
    let p_near: *mut Fts5ExprNearset = unsafe { (*p_node_1).p_near };
    let mut rc: i32 = unsafe { *p_rc_1 };
    if unsafe { (*(*p_expr_1).p_config).e_detail } != 0 {
        let mut p_term: *const Fts5ExprTerm = core::ptr::null();
        let p_phrase: *mut Fts5ExprPhrase =
            unsafe {
                *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                            *mut *mut Fts5ExprPhrase).offset(0 as isize)
            };
        unsafe { (*p_phrase).poslist.n = 0 };
        {
            p_term =
                unsafe {
                    &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                                    *mut Fts5ExprTerm).offset(0 as isize)
                };
            '__b28: loop {
                if !(!(p_term).is_null()) { break '__b28; }
                '__c28: loop {
                    let p_iter: *const Fts5IndexIter =
                        unsafe { (*p_term).p_iter } as *const Fts5IndexIter;
                    if unsafe { (*p_iter).b_eof } as i32 == 0 {
                        if unsafe { (*p_iter).i_rowid } ==
                                    unsafe { (*p_node_1).i_rowid } &&
                                unsafe { (*p_iter).n_data } > 0 {
                            unsafe { (*p_phrase).poslist.n = 1 };
                        }
                    }
                    break '__c28;
                }
                p_term = unsafe { (*p_term).p_synonym };
            }
        }
        return unsafe { (*p_phrase).poslist.n };
    } else {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b29: loop {
                if !(rc == 0 && i < unsafe { (*p_near).n_phrase }) {
                    break '__b29;
                }
                '__c29: loop {
                    let p_phrase_1: *mut Fts5ExprPhrase =
                        unsafe {
                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                        *mut *mut Fts5ExprPhrase).offset(i as isize)
                        };
                    if unsafe { (*p_phrase_1).n_term } > 1 ||
                                    !(unsafe {
                                                    (*(unsafe { (*p_phrase_1).a_term.as_ptr() } as
                                                                    *mut Fts5ExprTerm).offset(0 as isize)).p_synonym
                                                }).is_null() || !(unsafe { (*p_near).p_colset }).is_null()
                            ||
                            unsafe {
                                    (*(unsafe { (*p_phrase_1).a_term.as_ptr() } as
                                                    *mut Fts5ExprTerm).offset(0 as isize)).b_first
                                } != 0 {
                        let mut b_match: i32 = 0;
                        rc =
                            fts5_expr_phrase_is_match(unsafe { &*p_node_1 },
                                unsafe { &mut *p_phrase_1 }, &mut b_match);
                        if b_match == 0 { break '__b29; }
                    } else {
                        let p_iter_1: *const Fts5IndexIter =
                            unsafe {
                                    (*(unsafe { (*p_phrase_1).a_term.as_ptr() } as
                                                    *mut Fts5ExprTerm).offset(0 as isize)).p_iter
                                } as *const Fts5IndexIter;
                        unsafe {
                            sqlite3_fts5_buffer_set(&mut rc,
                                unsafe { &mut (*p_phrase_1).poslist },
                                unsafe { (*p_iter_1).n_data },
                                unsafe { (*p_iter_1).p_data })
                        };
                    }
                    break '__c29;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { *p_rc_1 = rc };
        if i == unsafe { (*p_near).n_phrase } &&
                (i == 1 ||
                    fts5_expr_near_is_match(unsafe { &mut *p_rc_1 },
                            unsafe { &*p_near }) != 0) {
            return 1;
        }
        return 0;
    }
}

extern "C" fn fts5_expr_node_test_string(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode) -> i32 {
    let p_near: *const Fts5ExprNearset =
        unsafe { (*p_node_1).p_near } as *const Fts5ExprNearset;
    let p_left: *mut Fts5ExprPhrase =
        unsafe {
            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                        *mut *mut Fts5ExprPhrase).offset(0 as isize)
        };
    let mut rc: i32 = 0;
    let mut i_last: i64 = 0 as i64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut b_match: i32 = 0;
    let b_desc: i32 = unsafe { (*p_expr_1).b_desc } as i32;
    if !(unsafe { (*p_near).n_phrase } > 1 ||
                                    unsafe {
                                            (*unsafe {
                                                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                        *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                        }).n_term
                                        } > 1 ||
                                !(unsafe {
                                                (*(unsafe {
                                                                    (*unsafe {
                                                                                        *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                                                    *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                                                    }).a_term.as_ptr()
                                                                } as *mut Fts5ExprTerm).offset(0 as isize)).p_synonym
                                            }).is_null() ||
                            unsafe {
                                    (*(unsafe {
                                                        (*unsafe {
                                                                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                                        *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                                        }).a_term.as_ptr()
                                                    } as *mut Fts5ExprTerm).offset(0 as isize)).b_first
                                } != 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_STRING".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1111,
                c"pNear->nPhrase>1 || pNear->apPhrase[0]->nTerm>1 || pNear->apPhrase[0]->aTerm[0].pSynonym || pNear->apPhrase[0]->aTerm[0].bFirst".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                        (*(unsafe { (*p_left).a_term.as_ptr() } as
                                        *mut Fts5ExprTerm).offset(0 as isize)).p_synonym
                    }).is_null() {
        i_last =
            fts5_expr_synonym_rowid(unsafe {
                    &mut *(unsafe { (*p_left).a_term.as_ptr() } as
                                    *mut Fts5ExprTerm).offset(0 as isize)
                }, b_desc, core::ptr::null_mut());
    } else {
        i_last =
            unsafe {
                (*unsafe {
                                (*(unsafe { (*p_left).a_term.as_ptr() } as
                                                *mut Fts5ExprTerm).offset(0 as isize)).p_iter
                            }).i_rowid
            };
    }
    '__b30: loop {
        '__c30: loop {
            b_match = 1;
            {
                i = 0;
                '__b31: loop {
                    if !(i < unsafe { (*p_near).n_phrase }) { break '__b31; }
                    '__c31: loop {
                        let p_phrase: *mut Fts5ExprPhrase =
                            unsafe {
                                *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                            *mut *mut Fts5ExprPhrase).offset(i as isize)
                            };
                        {
                            j = 0;
                            '__b32: loop {
                                if !(j < unsafe { (*p_phrase).n_term }) { break '__b32; }
                                '__c32: loop {
                                    let p_term: *mut Fts5ExprTerm =
                                        unsafe {
                                            &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                            *mut Fts5ExprTerm).offset(j as isize)
                                        };
                                    if !(unsafe { (*p_term).p_synonym }).is_null() {
                                        let i_rowid: i64 =
                                            fts5_expr_synonym_rowid(p_term, b_desc,
                                                core::ptr::null_mut());
                                        if i_rowid == i_last { break '__c32; }
                                        b_match = 0;
                                        if fts5_expr_synonym_advanceto(p_term, b_desc, &mut i_last,
                                                    &mut rc) != 0 {
                                            unsafe { (*p_node_1).b_nomatch = 0 };
                                            unsafe { (*p_node_1).b_eof = 1 };
                                            return rc;
                                        }
                                    } else {
                                        let p_iter: *mut Fts5IndexIter =
                                            unsafe {
                                                (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                                *mut Fts5ExprTerm).offset(j as isize)).p_iter
                                            };
                                        if unsafe { (*p_iter).i_rowid } == i_last { break '__c32; }
                                        b_match = 0;
                                        if fts5_expr_advanceto(p_iter, b_desc, &mut i_last, &mut rc,
                                                    unsafe { &mut (*p_node_1).b_eof }) != 0 {
                                            return rc;
                                        }
                                    }
                                    break '__c32;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        break '__c31;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            break '__c30;
        }
        if !(b_match == 0) { break '__b30; }
    }
    unsafe { (*p_node_1).i_rowid = i_last };
    unsafe {
        (*p_node_1).b_nomatch =
            (0 ==
                        fts5_expr_near_test(&mut rc, unsafe { &*p_expr_1 },
                            p_node_1) && rc == 0) as i32
    };
    if !(unsafe { (*p_node_1).b_eof } == 0 ||
                            unsafe { (*p_node_1).b_nomatch } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_STRING".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1152,
                c"pNode->bEof==0 || pNode->bNomatch==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    return rc;
}

extern "C" fn fts5_expr_node_test_term(p_expr_1: &Fts5Expr,
    p_node_1: &mut Fts5ExprNode) -> i32 {
    let p_phrase: *mut Fts5ExprPhrase =
        unsafe {
            *(unsafe { (*(*p_node_1).p_near).ap_phrase.as_ptr() } as
                        *mut *mut Fts5ExprPhrase).offset(0 as isize)
        };
    let p_iter: *mut Fts5IndexIter =
        unsafe {
            (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                            *mut Fts5ExprTerm).offset(0 as isize)).p_iter
        };
    if !((*p_node_1).e_type == 4) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_TERM".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1241,
                c"pNode->eType==FTS5_TERM".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*(*p_node_1).p_near).n_phrase } == 1 &&
                            unsafe { (*p_phrase).n_term } == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_TERM".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1242,
                c"pNode->pNear->nPhrase==1 && pPhrase->nTerm==1".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe {
                                (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                *mut Fts5ExprTerm).offset(0 as isize)).p_synonym
                            } == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_TERM".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1243,
                c"pPhrase->aTerm[0].pSynonym==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    unsafe { (*p_phrase).poslist.n = unsafe { (*p_iter).n_data } };
    if unsafe { (*(*p_expr_1).p_config).e_detail } == 0 {
        unsafe {
            (*p_phrase).poslist.p = unsafe { (*p_iter).p_data } as *mut u8
        };
    }
    (*p_node_1).i_rowid = unsafe { (*p_iter).i_rowid };
    (*p_node_1).b_nomatch = (unsafe { (*p_phrase).poslist.n } == 0) as i32;
    return 0;
}

extern "C" fn fts5_rowid_cmp(p_expr_1: &Fts5Expr, i_lhs_1: i64, i_rhs_1: i64)
    -> i32 {
    if !((*p_expr_1).b_desc == 0 || (*p_expr_1).b_desc == 1) as i32 as i64 !=
            0 {
        unsafe {
            __assert_rtn(c"fts5RowidCmp".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1024,
                c"pExpr->bDesc==0 || pExpr->bDesc==1".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if (*p_expr_1).b_desc == 0 {
        if i_lhs_1 < i_rhs_1 { return -1; }
        return (i_lhs_1 > i_rhs_1) as i32;
    } else {
        if i_lhs_1 > i_rhs_1 { return -1; }
        return (i_lhs_1 < i_rhs_1) as i32;
    }
}

extern "C" fn fts5_expr_node_zero_poslist(p_node_1: &Fts5ExprNode) -> () {
    if (*p_node_1).e_type == 9 || (*p_node_1).e_type == 4 {
        let p_near: *const Fts5ExprNearset =
            (*p_node_1).p_near as *const Fts5ExprNearset;
        let mut i: i32 = 0;
        {
            i = 0;
            '__b33: loop {
                if !(i < unsafe { (*p_near).n_phrase }) { break '__b33; }
                '__c33: loop {
                    let p_phrase: *mut Fts5ExprPhrase =
                        unsafe {
                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                        *mut *mut Fts5ExprPhrase).offset(i as isize)
                        };
                    unsafe { (*p_phrase).poslist.n = 0 };
                    break '__c33;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    } else {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b34: loop {
                if !(i < (*p_node_1).n_child) { break '__b34; }
                '__c34: loop {
                    fts5_expr_node_zero_poslist(unsafe {
                            &*unsafe {
                                        *((*p_node_1).ap_child.as_ptr() as
                                                    *mut *mut Fts5ExprNode).offset(i as isize)
                                    }
                        });
                    break '__c34;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn fts5_expr_node_test_and(p_expr_1: *mut Fts5Expr,
    p_and_1: *mut Fts5ExprNode) -> i32 {
    let mut i_child: i32 = 0;
    let mut i_last: i64 = unsafe { (*p_and_1).i_rowid };
    let mut rc: i32 = 0;
    let mut b_match: i32 = 0;
    if !(unsafe { (*p_and_1).b_eof } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_AND".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1341,
                c"pAnd->bEof==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    '__b35: loop {
        '__c35: loop {
            unsafe { (*p_and_1).b_nomatch = 0 };
            b_match = 1;
            {
                i_child = 0;
                '__b36: loop {
                    if !(i_child < unsafe { (*p_and_1).n_child }) {
                        break '__b36;
                    }
                    '__c36: loop {
                        let p_child: *mut Fts5ExprNode =
                            unsafe {
                                *(unsafe { (*p_and_1).ap_child.as_ptr() } as
                                            *mut *mut Fts5ExprNode).offset(i_child as isize)
                            };
                        let cmp: i32 =
                            fts5_rowid_cmp(unsafe { &*p_expr_1 }, i_last,
                                unsafe { (*p_child).i_rowid });
                        if cmp > 0 {
                            rc =
                                unsafe {
                                    (unsafe {
                                            (*p_child).x_next.unwrap()
                                        })(p_expr_1, p_child, 1, i_last)
                                };
                            if rc != 0 {
                                unsafe { (*p_and_1).b_nomatch = 0 };
                                return rc;
                            }
                        }
                        if !(unsafe { (*p_child).b_eof } != 0 ||
                                                fts5_rowid_cmp(unsafe { &*p_expr_1 }, i_last,
                                                        unsafe { (*p_child).i_rowid }) <= 0) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"fts5ExprNodeTest_AND".as_ptr() as *const i8,
                                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1361,
                                    c"pChild->bEof || fts5RowidCmp(pExpr, iLast, pChild->iRowid)<=0".as_ptr()
                                            as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        if unsafe { (*p_child).b_eof } != 0 {
                            fts5_expr_set_eof(unsafe { &mut *p_and_1 });
                            b_match = 1;
                            break '__b36;
                        } else if i_last != unsafe { (*p_child).i_rowid } {
                            b_match = 0;
                            i_last = unsafe { (*p_child).i_rowid };
                        }
                        if unsafe { (*p_child).b_nomatch } != 0 {
                            unsafe { (*p_and_1).b_nomatch = 1 };
                        }
                        break '__c36;
                    }
                    { let __p = &mut i_child; let __t = *__p; *__p += 1; __t };
                }
            }
            break '__c35;
        }
        if !(b_match == 0) { break '__b35; }
    }
    if unsafe { (*p_and_1).b_nomatch } != 0 &&
            p_and_1 != unsafe { (*p_expr_1).p_root } {
        fts5_expr_node_zero_poslist(unsafe { &*p_and_1 });
    }
    unsafe { (*p_and_1).i_rowid = i_last };
    return 0;
}

extern "C" fn fts5_node_compare(p_expr_1: *mut Fts5Expr, p1: &Fts5ExprNode,
    p2: &Fts5ExprNode) -> i32 {
    if (*p2).b_eof != 0 { return -1; }
    if (*p1).b_eof != 0 { return 1; }
    return fts5_rowid_cmp(unsafe { &*p_expr_1 }, (*p1).i_rowid,
            (*p2).i_rowid);
}

extern "C" fn fts5_expr_node_test_or(p_expr_1: *mut Fts5Expr,
    p_node_1: &mut Fts5ExprNode) -> () {
    let mut p_next: *mut Fts5ExprNode =
        unsafe {
            *((*p_node_1).ap_child.as_ptr() as
                        *mut *mut Fts5ExprNode).offset(0 as isize)
        };
    let mut i: i32 = 0;
    {
        i = 1;
        '__b37: loop {
            if !(i < (*p_node_1).n_child) { break '__b37; }
            '__c37: loop {
                let p_child: *mut Fts5ExprNode =
                    unsafe {
                        *((*p_node_1).ap_child.as_ptr() as
                                    *mut *mut Fts5ExprNode).offset(i as isize)
                    };
                let cmp: i32 =
                    fts5_node_compare(p_expr_1, unsafe { &*p_next },
                        unsafe { &*p_child });
                if cmp > 0 || cmp == 0 && unsafe { (*p_child).b_nomatch } == 0
                    {
                    p_next = p_child;
                }
                break '__c37;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_node_1).i_rowid = unsafe { (*p_next).i_rowid };
    (*p_node_1).b_eof = unsafe { (*p_next).b_eof };
    (*p_node_1).b_nomatch = unsafe { (*p_next).b_nomatch };
}

extern "C" fn fts5_expr_node_test_not(p_expr_1: *mut Fts5Expr,
    p_node_1: &mut Fts5ExprNode) -> i32 {
    let mut rc: i32 = 0;
    let p1: *mut Fts5ExprNode =
        unsafe {
            *((*p_node_1).ap_child.as_ptr() as
                        *mut *mut Fts5ExprNode).offset(0 as isize)
        };
    let p2: *mut Fts5ExprNode =
        unsafe {
            *((*p_node_1).ap_child.as_ptr() as
                        *mut *mut Fts5ExprNode).offset(1 as isize)
        };
    if !((*p_node_1).n_child == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeTest_NOT".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1406,
                c"pNode->nChild==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    while rc == 0 && unsafe { (*p1).b_eof } == 0 {
        let mut cmp: i32 =
            fts5_node_compare(p_expr_1, unsafe { &*p1 }, unsafe { &*p2 });
        if cmp > 0 {
            rc =
                unsafe {
                    (unsafe {
                            (*p2).x_next.unwrap()
                        })(p_expr_1, p2, 1, unsafe { (*p1).i_rowid })
                };
            cmp =
                fts5_node_compare(p_expr_1, unsafe { &*p1 }, unsafe { &*p2 });
        }
        if !(rc != 0 || cmp <= 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ExprNodeTest_NOT".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1414,
                    c"rc!=SQLITE_OK || cmp<=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if cmp != 0 || unsafe { (*p2).b_nomatch } != 0 { break; }
        rc =
            unsafe { (unsafe { (*p1).x_next.unwrap() })(p_expr_1, p1, 0, 0) };
    }
    (*p_node_1).b_eof = unsafe { (*p1).b_eof };
    (*p_node_1).b_nomatch = unsafe { (*p1).b_nomatch };
    (*p_node_1).i_rowid = unsafe { (*p1).i_rowid };
    if unsafe { (*p1).b_eof } != 0 {
        fts5_expr_node_zero_poslist(unsafe { &*p2 });
    }
    return rc;
}

extern "C" fn fts5_expr_node_test(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p_node_1).b_eof } == 0 {
        '__s39:
            {
            match unsafe { (*p_node_1).e_type } {
                9 => {
                    {
                        rc = fts5_expr_node_test_string(p_expr_1, p_node_1);
                        break '__s39;
                    }
                    {
                        rc =
                            fts5_expr_node_test_term(unsafe { &*p_expr_1 },
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                    {
                        rc = fts5_expr_node_test_and(p_expr_1, p_node_1);
                        break '__s39;
                    }
                    {
                        fts5_expr_node_test_or(p_expr_1, unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeTest".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1476,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    {
                        rc =
                            fts5_expr_node_test_not(p_expr_1,
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                }
                4 => {
                    {
                        rc =
                            fts5_expr_node_test_term(unsafe { &*p_expr_1 },
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                    {
                        rc = fts5_expr_node_test_and(p_expr_1, p_node_1);
                        break '__s39;
                    }
                    {
                        fts5_expr_node_test_or(p_expr_1, unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeTest".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1476,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    {
                        rc =
                            fts5_expr_node_test_not(p_expr_1,
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                }
                2 => {
                    {
                        rc = fts5_expr_node_test_and(p_expr_1, p_node_1);
                        break '__s39;
                    }
                    {
                        fts5_expr_node_test_or(p_expr_1, unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeTest".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1476,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    {
                        rc =
                            fts5_expr_node_test_not(p_expr_1,
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                }
                1 => {
                    {
                        fts5_expr_node_test_or(p_expr_1, unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeTest".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1476,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    {
                        rc =
                            fts5_expr_node_test_not(p_expr_1,
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                }
                _ => {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeTest".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1476,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    {
                        rc =
                            fts5_expr_node_test_not(p_expr_1,
                                unsafe { &mut *p_node_1 });
                        break '__s39;
                    }
                }
            }
        }
    }
    return rc;
}

extern "C" fn fts5_expr_node_first(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode) -> i32 {
    let mut rc: i32 = 0;
    unsafe { (*p_node_1).b_eof = 0 };
    unsafe { (*p_node_1).b_nomatch = 0 };
    if unsafe { (*p_node_1).e_type } == 4 ||
            unsafe { (*p_node_1).e_type } == 9 {
        rc =
            fts5_expr_near_init_all(unsafe { &*p_expr_1 },
                unsafe { &mut *p_node_1 });
    } else if !unsafe { (*p_node_1).x_next.is_some() } as i32 != 0 {
        unsafe { (*p_node_1).b_eof = 1 };
    } else {
        let mut i: i32 = 0;
        let mut n_eof: i32 = 0;
        {
            i = 0;
            '__b40: loop {
                if !(i < unsafe { (*p_node_1).n_child } && rc == 0) {
                    break '__b40;
                }
                '__c40: loop {
                    let p_child: *const Fts5ExprNode =
                        unsafe {
                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                            *mut *mut Fts5ExprNode).offset(i as isize)
                            } as *const Fts5ExprNode;
                    rc =
                        fts5_expr_node_first(p_expr_1,
                            unsafe {
                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                            *mut *mut Fts5ExprNode).offset(i as isize)
                            });
                    if !(unsafe { (*p_child).b_eof } == 0 ||
                                            unsafe { (*p_child).b_eof } == 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeFirst".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1509,
                                c"pChild->bEof==0 || pChild->bEof==1".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    n_eof += unsafe { (*p_child).b_eof };
                    break '__c40;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            (*p_node_1).i_rowid =
                unsafe {
                    (*unsafe {
                                    *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                *mut *mut Fts5ExprNode).offset(0 as isize)
                                }).i_rowid
                }
        };
        '__s41:
            {
            match unsafe { (*p_node_1).e_type } {
                2 => {
                    if n_eof > 0 {
                        fts5_expr_set_eof(unsafe { &mut *p_node_1 });
                    }
                }
                1 => {
                    if unsafe { (*p_node_1).n_child } == n_eof {
                        fts5_expr_set_eof(unsafe { &mut *p_node_1 });
                    }
                }
                _ => {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprNodeFirst".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1524,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe {
                        (*p_node_1).b_eof =
                            unsafe {
                                (*unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(0 as isize)
                                            }).b_eof
                            }
                    };
                }
            }
        }
    }
    if rc == 0 { rc = fts5_expr_node_test(p_expr_1, p_node_1); }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_first(p: *mut Fts5Expr,
    p_idx: *mut Fts5Index, i_first: i64, i_last: i64, b_desc: i32) -> i32 {
    let p_root: *mut Fts5ExprNode = unsafe { (*p).p_root };
    let mut rc: i32 = 0;
    unsafe { (*p).p_index = p_idx };
    unsafe { (*p).b_desc = b_desc };
    rc = fts5_expr_node_first(p, p_root);
    if rc == 0 && 0 == unsafe { (*p_root).b_eof } &&
            fts5_rowid_cmp(unsafe { &*p }, unsafe { (*p_root).i_rowid },
                    i_first) < 0 {
        rc =
            unsafe {
                (unsafe { (*p_root).x_next.unwrap() })(p, p_root, 1, i_first)
            };
    }
    while unsafe { (*p_root).b_nomatch } != 0 && rc == 0 {
        if !(unsafe { (*p_root).b_eof } == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ExprFirst".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1577,
                    c"pRoot->bEof==0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        rc =
            unsafe {
                (unsafe { (*p_root).x_next.unwrap() })(p, p_root, 0, 0)
            };
    }
    if fts5_rowid_cmp(unsafe { &*p }, unsafe { (*p_root).i_rowid }, i_last) >
            0 {
        unsafe { (*p_root).b_eof = 1 };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_next(p: *mut Fts5Expr, i_last: i64)
    -> i32 {
    let mut rc: i32 = 0;
    let p_root: *mut Fts5ExprNode = unsafe { (*p).p_root };
    if !(unsafe { (*p_root).b_eof } == 0 &&
                            unsafe { (*p_root).b_nomatch } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ExprNext".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1595,
                c"pRoot->bEof==0 && pRoot->bNomatch==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    '__b43: loop {
        '__c43: loop {
            rc =
                unsafe {
                    (unsafe { (*p_root).x_next.unwrap() })(p, p_root, 0, 0)
                };
            if !(unsafe { (*p_root).b_nomatch } == 0 ||
                                    rc == 0 && unsafe { (*p_root).b_eof } == 0) as i32 as i64 !=
                    0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ExprNext".as_ptr() as *const i8,
                        c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1598,
                        c"pRoot->bNomatch==0 || (rc==SQLITE_OK && pRoot->bEof==0)".as_ptr()
                                as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            break '__c43;
        }
        if !(unsafe { (*p_root).b_nomatch } != 0) { break '__b43; }
    }
    if fts5_rowid_cmp(unsafe { &*p }, unsafe { (*p_root).i_rowid }, i_last) >
            0 {
        unsafe { (*p_root).b_eof = 1 };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_eof(p: &Fts5Expr) -> i32 {
    return unsafe { (*(*p).p_root).b_eof };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_rowid(p: &Fts5Expr) -> i64 {
    return unsafe { (*(*p).p_root).i_rowid };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_free(p: *mut Fts5Expr) -> () {
    if !(p).is_null() {
        sqlite3_fts5_parse_node_free(unsafe { (*p).p_root });
        unsafe { sqlite3_free(unsafe { (*p).ap_expr_phrase } as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

extern "C" fn fts5_expr_node_next_term(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode, b_from_valid_1: i32, i_from_1: i64) -> i32 {
    let mut rc: i32 = 0;
    let p_iter: *mut Fts5IndexIter =
        unsafe {
            (*(unsafe {
                                (*unsafe {
                                                    *(unsafe {
                                                                    (*unsafe { (*p_node_1).p_near }).ap_phrase.as_ptr()
                                                                } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                }).a_term.as_ptr()
                            } as *mut Fts5ExprTerm).offset(0 as isize)).p_iter
        };
    if !(unsafe { (*p_node_1).b_eof } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ExprNodeNext_TERM".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1266,
                c"pNode->bEof==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if b_from_valid_1 != 0 {
        rc = unsafe { sqlite3_fts5_iter_next_from(p_iter, i_from_1) };
    } else { rc = unsafe { sqlite3_fts5_iter_next(p_iter) }; }
    if rc == 0 && unsafe { (*p_iter).b_eof } as i32 == 0 {
        rc =
            fts5_expr_node_test_term(unsafe { &*p_expr_1 },
                unsafe { &mut *p_node_1 });
    } else {
        unsafe { (*p_node_1).b_eof = 1 };
        unsafe { (*p_node_1).b_nomatch = 0 };
    }
    return rc;
}

extern "C" fn fts5_expr_node_next_string(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode, b_from_valid_1: i32, i_from_1: i64) -> i32 {
    let p_term: *mut Fts5ExprTerm =
        unsafe {
            &mut *(unsafe {
                                (*unsafe {
                                                    *(unsafe {
                                                                    (*unsafe { (*p_node_1).p_near }).ap_phrase.as_ptr()
                                                                } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                }).a_term.as_ptr()
                            } as *mut Fts5ExprTerm).offset(0 as isize)
        };
    let mut rc: i32 = 0;
    unsafe { (*p_node_1).b_nomatch = 0 };
    if !(unsafe { (*p_term).p_synonym }).is_null() {
        let mut b_eof: i32 = 1;
        let mut p: *const Fts5ExprTerm = core::ptr::null();
        let i_rowid: i64 =
            fts5_expr_synonym_rowid(p_term, unsafe { (*p_expr_1).b_desc },
                core::ptr::null_mut());
        {
            p = p_term;
            '__b44: loop {
                if !(!(p).is_null()) { break '__b44; }
                '__c44: loop {
                    if unsafe { (*unsafe { (*p).p_iter }).b_eof } as i32 == 0 {
                        let ii: i64 = unsafe { (*unsafe { (*p).p_iter }).i_rowid };
                        if ii == i_rowid ||
                                b_from_valid_1 != 0 && ii != i_from_1 &&
                                    (ii > i_from_1) as i32 == unsafe { (*p_expr_1).b_desc } {
                            if b_from_valid_1 != 0 {
                                rc =
                                    unsafe {
                                        sqlite3_fts5_iter_next_from(unsafe { (*p).p_iter },
                                            i_from_1)
                                    };
                            } else {
                                rc =
                                    unsafe { sqlite3_fts5_iter_next(unsafe { (*p).p_iter }) };
                            }
                            if rc != 0 { break '__b44; }
                            if unsafe { (*unsafe { (*p).p_iter }).b_eof } as i32 == 0 {
                                b_eof = 0;
                            }
                        } else { b_eof = 0; }
                    }
                    break '__c44;
                }
                p = unsafe { (*p).p_synonym };
            }
        }
        unsafe { (*p_node_1).b_eof = (rc != 0 || b_eof != 0) as i32 };
    } else {
        let p_iter: *mut Fts5IndexIter = unsafe { (*p_term).p_iter };
        if !(unsafe { (*p_node_1).e_type } == 4 ||
                                unsafe { (*p_node_1).e_type } == 9) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ExprNodeNext_STRING".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1210,
                    c"Fts5NodeIsString(pNode)".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if b_from_valid_1 != 0 {
            rc = unsafe { sqlite3_fts5_iter_next_from(p_iter, i_from_1) };
        } else { rc = unsafe { sqlite3_fts5_iter_next(p_iter) }; }
        unsafe {
            (*p_node_1).b_eof =
                (rc != 0 || unsafe { (*p_iter).b_eof } != 0) as i32
        };
    }
    if unsafe { (*p_node_1).b_eof } == 0 {
        if !(rc == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5ExprNodeNext_STRING".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1221,
                    c"rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        rc = fts5_expr_node_test_string(p_expr_1, p_node_1);
    }
    return rc;
}

extern "C" fn fts5_expr_node_next_or(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode, b_from_valid_1: i32, i_from_1: i64) -> i32 {
    let mut i: i32 = 0;
    let i_last: i64 = unsafe { (*p_node_1).i_rowid };
    {
        i = 0;
        '__b45: loop {
            if !(i < unsafe { (*p_node_1).n_child }) { break '__b45; }
            '__c45: loop {
                let p1: *mut Fts5ExprNode =
                    unsafe {
                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                    *mut *mut Fts5ExprNode).offset(i as isize)
                    };
                if !(unsafe { (*p1).b_eof } != 0 ||
                                        fts5_rowid_cmp(unsafe { &*p_expr_1 },
                                                unsafe { (*p1).i_rowid }, i_last) >= 0) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"fts5ExprNodeNext_OR".as_ptr() as *const i8,
                            c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1311,
                            c"p1->bEof || fts5RowidCmp(pExpr, p1->iRowid, iLast)>=0".as_ptr()
                                    as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                if unsafe { (*p1).b_eof } == 0 {
                    if unsafe { (*p1).i_rowid } == i_last ||
                            b_from_valid_1 != 0 &&
                                fts5_rowid_cmp(unsafe { &*p_expr_1 },
                                        unsafe { (*p1).i_rowid }, i_from_1) < 0 {
                        let rc: i32 =
                            unsafe {
                                (unsafe {
                                        (*p1).x_next.unwrap()
                                    })(p_expr_1, p1, b_from_valid_1, i_from_1)
                            };
                        if rc != 0 {
                            unsafe { (*p_node_1).b_nomatch = 0 };
                            return rc;
                        }
                    }
                }
                break '__c45;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    fts5_expr_node_test_or(p_expr_1, unsafe { &mut *p_node_1 });
    return 0;
}

extern "C" fn fts5_expr_node_next_and(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode, b_from_valid_1: i32, i_from_1: i64) -> i32 {
    let mut rc: i32 =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }).x_next.unwrap()
                })(p_expr_1,
                unsafe {
                    *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                *mut *mut Fts5ExprNode).offset(0 as isize)
                }, b_from_valid_1, i_from_1)
        };
    if rc == 0 {
        rc = fts5_expr_node_test_and(p_expr_1, p_node_1);
    } else { unsafe { (*p_node_1).b_nomatch = 0 }; }
    return rc;
}

extern "C" fn fts5_expr_node_next_not(p_expr_1: *mut Fts5Expr,
    p_node_1: *mut Fts5ExprNode, b_from_valid_1: i32, i_from_1: i64) -> i32 {
    let mut rc: i32 =
        unsafe {
            (unsafe {
                    (*unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }).x_next.unwrap()
                })(p_expr_1,
                unsafe {
                    *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                *mut *mut Fts5ExprNode).offset(0 as isize)
                }, b_from_valid_1, i_from_1)
        };
    if rc == 0 {
        rc = fts5_expr_node_test_not(p_expr_1, unsafe { &mut *p_node_1 });
    }
    if rc != 0 { unsafe { (*p_node_1).b_nomatch = 0 }; }
    return rc;
}

extern "C" fn fts5_expr_assign_x_next(p_node_1: &mut Fts5ExprNode) -> () {
    '__s46:
        {
        match (*p_node_1).e_type {
            9 => {
                {
                    let p_near: *const Fts5ExprNearset =
                        (*p_node_1).p_near as *const Fts5ExprNearset;
                    if unsafe { (*p_near).n_phrase } == 1 &&
                                    unsafe {
                                            (*unsafe {
                                                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                        *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                        }).n_term
                                        } == 1 &&
                                unsafe {
                                        (*(unsafe {
                                                            (*unsafe {
                                                                                *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                                            *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                                            }).a_term.as_ptr()
                                                        } as *mut Fts5ExprTerm).offset(0 as isize)).p_synonym
                                    } == core::ptr::null_mut() &&
                            unsafe {
                                        (*(unsafe {
                                                            (*unsafe {
                                                                                *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                                            *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                                            }).a_term.as_ptr()
                                                        } as *mut Fts5ExprTerm).offset(0 as isize)).b_first
                                    } as i32 == 0 {
                        (*p_node_1).e_type = 4;
                        (*p_node_1).x_next = Some(fts5_expr_node_next_term);
                    } else {
                        (*p_node_1).x_next = Some(fts5_expr_node_next_string);
                    }
                    break '__s46;
                }
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_or);
                    break '__s46;
                }
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_and);
                    break '__s46;
                }
                if !((*p_node_1).e_type == 3) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"fts5ExprAssignXNext".as_ptr() as *const i8,
                            c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2273,
                            c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_not);
                    break '__s46;
                }
            }
            1 => {
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_or);
                    break '__s46;
                }
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_and);
                    break '__s46;
                }
                if !((*p_node_1).e_type == 3) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"fts5ExprAssignXNext".as_ptr() as *const i8,
                            c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2273,
                            c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_not);
                    break '__s46;
                }
            }
            2 => {
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_and);
                    break '__s46;
                }
                if !((*p_node_1).e_type == 3) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"fts5ExprAssignXNext".as_ptr() as *const i8,
                            c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2273,
                            c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_not);
                    break '__s46;
                }
            }
            _ => {
                if !((*p_node_1).e_type == 3) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"fts5ExprAssignXNext".as_ptr() as *const i8,
                            c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2273,
                            c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                {
                    (*p_node_1).x_next = Some(fts5_expr_node_next_not);
                    break '__s46;
                }
            }
        }
    }
}

extern "C" fn parse_grow_phrase_array(p_parse_1: &mut Fts5Parse) -> i32 {
    if (*p_parse_1).n_phrase % 8 == 0 {
        let n_byte: Sqlite3Int64 =
            (core::mem::size_of::<*mut Fts5ExprPhrase>() as u64 *
                    ((*p_parse_1).n_phrase + 8) as u64) as Sqlite3Int64;
        let mut ap_new: *mut *mut Fts5ExprPhrase = core::ptr::null_mut();
        ap_new =
            unsafe {
                    sqlite3_realloc64((*p_parse_1).ap_phrase as *mut (),
                        n_byte as Sqlite3Uint64)
                } as *mut *mut Fts5ExprPhrase;
        if ap_new == core::ptr::null_mut() { (*p_parse_1).rc = 7; return 7; }
        (*p_parse_1).ap_phrase = ap_new;
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_phrase_free(p_phrase:
        *mut Fts5ExprPhrase) -> () {
    fts5_expr_phrase_free(p_phrase);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_nearset(p_parse: *mut Fts5Parse,
    p_near: *mut Fts5ExprNearset, mut p_phrase: *mut Fts5ExprPhrase)
    -> *mut Fts5ExprNearset {
    let szalloc: i32 = 8 as i32;
    let mut p_ret: *mut Fts5ExprNearset = core::ptr::null_mut();
    if unsafe { (*p_parse).rc } == 0 {
        if p_near == core::ptr::null_mut() {
            let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
            n_byte =
                (core::mem::offset_of!(Fts5ExprNearset, ap_phrase) as u64 +
                        (szalloc + 1) as u64 *
                            core::mem::size_of::<*mut Fts5ExprPhrase>() as u64) as
                    Sqlite3Int64;
            p_ret =
                unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                    *mut Fts5ExprNearset;
            if p_ret == core::ptr::null_mut() {
                unsafe { (*p_parse).rc = 7 };
            } else { unsafe { memset(p_ret as *mut (), 0, n_byte as u64) }; }
        } else if unsafe { (*p_near).n_phrase } % szalloc as i32 == 0 {
            let n_new: i32 = unsafe { (*p_near).n_phrase } + szalloc as i32;
            let mut n_byte_1: Sqlite3Int64 = 0 as Sqlite3Int64;
            n_byte_1 =
                (core::mem::offset_of!(Fts5ExprNearset, ap_phrase) as u64 +
                        (n_new + 1) as u64 *
                            core::mem::size_of::<*mut Fts5ExprPhrase>() as u64) as
                    Sqlite3Int64;
            p_ret =
                unsafe {
                        sqlite3_realloc64(p_near as *mut (),
                            n_byte_1 as Sqlite3Uint64)
                    } as *mut Fts5ExprNearset;
            if p_ret == core::ptr::null_mut() {
                unsafe { (*p_parse).rc = 7 };
            }
        } else { p_ret = p_near; }
    }
    if p_ret == core::ptr::null_mut() {
        if !(unsafe { (*p_parse).rc } != 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseNearset".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1695,
                    c"pParse->rc!=SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        sqlite3_fts5_parse_nearset_free(p_near);
        sqlite3_fts5_parse_phrase_free(p_phrase);
    } else {
        if unsafe { (*p_ret).n_phrase } > 0 {
            let p_last: *mut Fts5ExprPhrase =
                unsafe {
                    *(unsafe { (*p_ret).ap_phrase.as_ptr() } as
                                *mut *mut Fts5ExprPhrase).offset((unsafe {
                                        (*p_ret).n_phrase
                                    } - 1) as isize)
                };
            if !(p_parse != core::ptr::null_mut()) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseNearset".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        1701, c"pParse!=0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if !(unsafe { (*p_parse).ap_phrase } != core::ptr::null_mut()) as
                            i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseNearset".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        1702,
                        c"pParse->apPhrase!=0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if !(unsafe { (*p_parse).n_phrase } >= 2) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseNearset".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        1703,
                        c"pParse->nPhrase>=2".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if !(p_last ==
                                    unsafe {
                                        *unsafe {
                                                (*p_parse).ap_phrase.offset((unsafe { (*p_parse).n_phrase }
                                                            - 2) as isize)
                                            }
                                    }) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseNearset".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        1704,
                        c"pLast==pParse->apPhrase[pParse->nPhrase-2]".as_ptr() as
                                *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if unsafe { (*p_phrase).n_term } == 0 {
                fts5_expr_phrase_free(p_phrase);
                {
                    let __p = unsafe { &mut (*p_ret).n_phrase };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
                {
                    let __p = unsafe { &mut (*p_parse).n_phrase };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
                p_phrase = p_last;
            } else if unsafe { (*p_last).n_term } == 0 {
                fts5_expr_phrase_free(p_last);
                unsafe {
                    *unsafe {
                                (*p_parse).ap_phrase.offset((unsafe { (*p_parse).n_phrase }
                                            - 2) as isize)
                            } = p_phrase
                };
                {
                    let __p = unsafe { &mut (*p_parse).n_phrase };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
                {
                    let __p = unsafe { &mut (*p_ret).n_phrase };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
        }
        unsafe {
            *(unsafe { (*p_ret).ap_phrase.as_ptr() } as
                            *mut *mut Fts5ExprPhrase).offset({
                                let __p = unsafe { &mut (*p_ret).n_phrase };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = p_phrase
        };
    }
    return p_ret;
}

extern "C" fn fts5_parse_phrase_to_and(p_parse_1: *mut Fts5Parse,
    p_near_1: *mut Fts5ExprNearset) -> *mut Fts5ExprNode {
    let n_term: i32 =
        unsafe {
            (*unsafe {
                            *(unsafe { (*p_near_1).ap_phrase.as_ptr() } as
                                        *mut *mut Fts5ExprPhrase).offset(0 as isize)
                        }).n_term
        };
    let mut ii: i32 = 0;
    let mut n_byte: i32 = 0;
    let mut p_ret: *mut Fts5ExprNode = core::ptr::null_mut();
    if !(unsafe { (*p_near_1).n_phrase } == 1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ParsePhraseToAnd".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2318,
                c"pNear->nPhrase==1".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if (unsafe { (*p_parse_1).b_phrase_to_and } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ParsePhraseToAnd".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2319,
                c"pParse->bPhraseToAnd".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n_byte =
        (core::mem::offset_of!(Fts5ExprNode, ap_child) as u64 +
                (n_term + 1) as u64 *
                    core::mem::size_of::<*mut Fts5ExprNode>() as u64) as i32;
    p_ret =
        unsafe {
                sqlite3_fts5_malloc_zero(unsafe { &mut (*p_parse_1).rc },
                    n_byte as Sqlite3Int64)
            } as *mut Fts5ExprNode;
    if !(p_ret).is_null() {
        unsafe { (*p_ret).e_type = 2 };
        unsafe { (*p_ret).n_child = n_term };
        unsafe { (*p_ret).i_height = 1 };
        fts5_expr_assign_x_next(unsafe { &mut *p_ret });
        {
            let __p = unsafe { &mut (*p_parse_1).n_phrase };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        {
            ii = 0;
            '__b47: loop {
                if !(ii < n_term) { break '__b47; }
                '__c47: loop {
                    let p_phrase: *mut Fts5ExprPhrase =
                        unsafe {
                                sqlite3_fts5_malloc_zero(unsafe { &mut (*p_parse_1).rc },
                                    (core::mem::offset_of!(Fts5ExprPhrase, a_term) as u64 +
                                            1 as u64 * core::mem::size_of::<Fts5ExprTerm>() as u64) as
                                        Sqlite3Int64)
                            } as *mut Fts5ExprPhrase;
                    if !(p_phrase).is_null() {
                        if parse_grow_phrase_array(unsafe { &mut *p_parse_1 }) != 0
                            {
                            fts5_expr_phrase_free(p_phrase);
                        } else {
                            let p: *const Fts5ExprTerm =
                                unsafe {
                                        &raw mut *(unsafe {
                                                            (*unsafe {
                                                                                *(unsafe { (*p_near_1).ap_phrase.as_ptr() } as
                                                                                            *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                                            }).a_term.as_ptr()
                                                        } as *mut Fts5ExprTerm).offset(ii as isize)
                                    } as *const Fts5ExprTerm;
                            let p_to: *mut Fts5ExprTerm =
                                unsafe {
                                    &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                    *mut Fts5ExprTerm).offset(0 as isize)
                                };
                            unsafe {
                                *unsafe {
                                            (*p_parse_1).ap_phrase.offset({
                                                        let __p = unsafe { &mut (*p_parse_1).n_phrase };
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                        } = p_phrase
                            };
                            unsafe { (*p_phrase).n_term = 1 };
                            unsafe {
                                (*p_to).p_term =
                                    unsafe {
                                        sqlite3_fts5_strndup(unsafe { &mut (*p_parse_1).rc },
                                            unsafe { (*p).p_term } as *const i8,
                                            unsafe { (*p).n_full_term })
                                    }
                            };
                            unsafe {
                                (*p_to).n_query_term = unsafe { (*p).n_query_term }
                            };
                            unsafe {
                                (*p_to).n_full_term = unsafe { (*p).n_full_term }
                            };
                            unsafe {
                                *(unsafe { (*p_ret).ap_child.as_ptr() } as
                                                *mut *mut Fts5ExprNode).offset(ii as isize) =
                                    sqlite3_fts5_parse_node(p_parse_1, 9, core::ptr::null_mut(),
                                        core::ptr::null_mut(),
                                        sqlite3_fts5_parse_nearset(p_parse_1, core::ptr::null_mut(),
                                            p_phrase))
                            };
                        }
                    }
                    break '__c47;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_parse_1).rc } != 0 {
            sqlite3_fts5_parse_node_free(p_ret);
            p_ret = core::ptr::null_mut();
        } else { sqlite3_fts5_parse_nearset_free(p_near_1); }
    }
    return p_ret;
}

extern "C" fn fts5_expr_add_children(p: &mut Fts5ExprNode,
    p_sub_1: *mut Fts5ExprNode) -> () {
    let mut ii: i32 = (*p).n_child;
    if (*p).e_type != 3 && unsafe { (*p_sub_1).e_type } == (*p).e_type {
        let n_byte: i32 =
            (core::mem::size_of::<*mut Fts5ExprNode>() as u64 *
                    unsafe { (*p_sub_1).n_child } as u64) as i32;
        unsafe {
            memcpy(unsafe {
                        &raw mut *((*p).ap_child.as_ptr() as
                                        *mut *mut Fts5ExprNode).offset((*p).n_child as isize)
                    } as *mut (),
                unsafe { (*p_sub_1).ap_child.as_ptr() } as
                        *mut *mut Fts5ExprNode as *const (), n_byte as u64)
        };
        (*p).n_child += unsafe { (*p_sub_1).n_child };
        unsafe { sqlite3_free(p_sub_1 as *mut ()) };
    } else {
        unsafe {
            *((*p).ap_child.as_ptr() as
                            *mut *mut Fts5ExprNode).offset({
                                let __p = &mut (*p).n_child;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = p_sub_1
        };
    }
    {
        '__b48: loop {
            if !(ii < (*p).n_child) { break '__b48; }
            '__c48: loop {
                (*p).i_height =
                    if (*p).i_height >
                            unsafe {
                                    (*unsafe {
                                                    *((*p).ap_child.as_ptr() as
                                                                *mut *mut Fts5ExprNode).offset(ii as isize)
                                                }).i_height
                                } + 1 {
                        (*p).i_height
                    } else {
                        (unsafe {
                                (*unsafe {
                                                *((*p).ap_child.as_ptr() as
                                                            *mut *mut Fts5ExprNode).offset(ii as isize)
                                            }).i_height
                            }) + 1
                    };
                break '__c48;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_node(p_parse: *mut Fts5Parse,
    e_type: i32, mut p_left: *mut Fts5ExprNode,
    mut p_right: *mut Fts5ExprNode, mut p_near: *mut Fts5ExprNearset)
    -> *mut Fts5ExprNode {
    let mut p_ret: *mut Fts5ExprNode = core::ptr::null_mut();
    if unsafe { (*p_parse).rc } == 0 {
        let mut n_child: i32 = 0;
        let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
        if !(e_type != 9 && (p_near).is_null() as i32 != 0 ||
                                e_type == 9 && (p_left).is_null() as i32 != 0 &&
                                    (p_right).is_null() as i32 != 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseNode".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2381,
                    c"(eType!=FTS5_STRING && !pNear) || (eType==FTS5_STRING && !pLeft && !pRight)".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if e_type == 9 && p_near == core::ptr::null_mut() {
            return core::ptr::null_mut();
        }
        if e_type != 9 && p_left == core::ptr::null_mut() { return p_right; }
        if e_type != 9 && p_right == core::ptr::null_mut() { return p_left; }
        if e_type == 9 && unsafe { (*p_parse).b_phrase_to_and } != 0 &&
                unsafe {
                        (*unsafe {
                                        *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                    *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                    }).n_term
                    } > 1 {
            p_ret = fts5_parse_phrase_to_and(p_parse, p_near);
        } else {
            if e_type == 3 {
                n_child = 2;
            } else if e_type == 2 || e_type == 1 {
                n_child = 2;
                if unsafe { (*p_left).e_type } == e_type {
                    n_child += unsafe { (*p_left).n_child } - 1;
                }
                if unsafe { (*p_right).e_type } == e_type {
                    n_child += unsafe { (*p_right).n_child } - 1;
                }
            }
            n_byte =
                (core::mem::offset_of!(Fts5ExprNode, ap_child) as u64 +
                        n_child as u64 *
                            core::mem::size_of::<*mut Fts5ExprNode>() as u64) as
                    Sqlite3Int64;
            p_ret =
                unsafe {
                        sqlite3_fts5_malloc_zero(unsafe { &mut (*p_parse).rc },
                            n_byte)
                    } as *mut Fts5ExprNode;
            if !(p_ret).is_null() {
                unsafe { (*p_ret).e_type = e_type };
                unsafe { (*p_ret).p_near = p_near };
                fts5_expr_assign_x_next(unsafe { &mut *p_ret });
                if e_type == 9 {
                    let mut i_phrase: i32 = 0;
                    {
                        i_phrase = 0;
                        '__b49: loop {
                            if !(i_phrase < unsafe { (*p_near).n_phrase }) {
                                break '__b49;
                            }
                            '__c49: loop {
                                unsafe {
                                    (*unsafe {
                                                        *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                    *mut *mut Fts5ExprPhrase).offset(i_phrase as isize)
                                                    }).p_node = p_ret
                                };
                                if unsafe {
                                            (*unsafe {
                                                            *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                                        *mut *mut Fts5ExprPhrase).offset(i_phrase as isize)
                                                        }).n_term
                                        } == 0 {
                                    unsafe { (*p_ret).x_next = None };
                                    unsafe { (*p_ret).e_type = 0 };
                                }
                                break '__c49;
                            }
                            { let __p = &mut i_phrase; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { (*unsafe { (*p_parse).p_config }).e_detail } !=
                            0 {
                        let p_phrase: *const Fts5ExprPhrase =
                            unsafe {
                                    *(unsafe { (*p_near).ap_phrase.as_ptr() } as
                                                *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                } as *const Fts5ExprPhrase;
                        if unsafe { (*p_near).n_phrase } != 1 ||
                                    unsafe { (*p_phrase).n_term } > 1 ||
                                unsafe { (*p_phrase).n_term } > 0 &&
                                    unsafe {
                                            (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                            *mut Fts5ExprTerm).offset(0 as isize)).b_first
                                        } != 0 {
                            unsafe {
                                sqlite3_fts5_parse_error(unsafe { &mut *p_parse },
                                    c"fts5: %s queries are not supported (detail!=full)".as_ptr()
                                            as *mut i8 as *const i8,
                                    if unsafe { (*p_near).n_phrase } == 1 {
                                        c"phrase".as_ptr() as *mut i8
                                    } else { c"NEAR".as_ptr() as *mut i8 })
                            };
                            sqlite3_fts5_parse_node_free(p_ret);
                            p_ret = core::ptr::null_mut();
                            p_near = core::ptr::null_mut();
                            if !(p_left == core::ptr::null_mut() &&
                                                    p_right == core::ptr::null_mut()) as i32 as i64 != 0 {
                                unsafe {
                                    __assert_rtn(c"sqlite3Fts5ParseNode".as_ptr() as *const i8,
                                        c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2430,
                                        c"pLeft==0 && pRight==0".as_ptr() as *mut i8 as *const i8)
                                }
                            } else { { let _ = 0; } };
                        }
                    }
                } else {
                    if !(p_near == core::ptr::null_mut()) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"sqlite3Fts5ParseNode".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2434,
                                c"pNear==0".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    fts5_expr_add_children(unsafe { &mut *p_ret }, p_left);
                    fts5_expr_add_children(unsafe { &mut *p_ret }, p_right);
                    p_left = { p_right = core::ptr::null_mut(); p_right };
                    if unsafe { (*p_ret).i_height } > 256 {
                        unsafe {
                            sqlite3_fts5_parse_error(unsafe { &mut *p_parse },
                                c"fts5 expression tree is too large (maximum depth %d)".as_ptr()
                                        as *mut i8 as *const i8, 256)
                        };
                        sqlite3_fts5_parse_node_free(p_ret);
                        p_ret = core::ptr::null_mut();
                    }
                }
            }
        }
    }
    if p_ret == core::ptr::null_mut() {
        if !(unsafe { (*p_parse).rc } != 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseNode".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2452,
                    c"pParse->rc!=SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        sqlite3_fts5_parse_node_free(p_left);
        sqlite3_fts5_parse_node_free(p_right);
        sqlite3_fts5_parse_nearset_free(p_near);
    }
    return p_ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_and(pp1: &mut *mut Fts5Expr,
    p2: *mut Fts5Expr) -> i32 {
    let mut s_parse: Fts5Parse = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut s_parse as *mut (), 0,
            core::mem::size_of::<Fts5Parse>() as u64)
    };
    if !(*pp1).is_null() && !(p2).is_null() {
        let p1: *mut Fts5Expr = *pp1;
        let n_phrase: i32 =
            unsafe { (*p1).n_phrase } + unsafe { (*p2).n_phrase };
        unsafe {
            (*p1).p_root =
                sqlite3_fts5_parse_node(&mut s_parse, 2,
                    unsafe { (*p1).p_root }, unsafe { (*p2).p_root },
                    core::ptr::null_mut())
        };
        unsafe { (*p2).p_root = core::ptr::null_mut() };
        if s_parse.rc == 0 {
            let ap: *mut *mut Fts5ExprPhrase =
                unsafe {
                        sqlite3_realloc64(unsafe { (*p1).ap_expr_phrase } as
                                *mut (),
                            n_phrase as u64 *
                                core::mem::size_of::<*mut Fts5ExprPhrase>() as u64)
                    } as *mut *mut Fts5ExprPhrase;
            if ap == core::ptr::null_mut() {
                s_parse.rc = 7;
            } else {
                let mut i: i32 = 0;
                unsafe {
                    memmove(unsafe {
                                &raw mut *ap.offset(unsafe { (*p2).n_phrase } as isize)
                            } as *mut (), ap as *const (),
                        unsafe { (*p1).n_phrase } as u64 *
                            core::mem::size_of::<*mut Fts5ExprPhrase>() as u64)
                };
                {
                    i = 0;
                    '__b50: loop {
                        if !(i < unsafe { (*p2).n_phrase }) { break '__b50; }
                        '__c50: loop {
                            unsafe {
                                *ap.offset(i as isize) =
                                    unsafe {
                                        *unsafe { (*p2).ap_expr_phrase.offset(i as isize) }
                                    }
                            };
                            break '__c50;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { (*p1).n_phrase = n_phrase };
                unsafe { (*p1).ap_expr_phrase = ap };
            }
        }
        unsafe { sqlite3_free(unsafe { (*p2).ap_expr_phrase } as *mut ()) };
        unsafe { sqlite3_free(p2 as *mut ()) };
    } else if !(p2).is_null() { *pp1 = p2; }
    return s_parse.rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_init(p_global: *mut Fts5Global,
    db: *mut Sqlite3) -> i32 {
    let rc: i32 = 0;
    { { let _ = p_global; }; { let _ = db; } };
    { let _ = sqlite3_fts5_parser_trace; };
    { let _ = sqlite3_fts5_parser_fallback; };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_phrase_count(p_expr: *mut Fts5Expr)
    -> i32 {
    return if !(p_expr).is_null() {
            unsafe { (*p_expr).n_phrase }
        } else { 0 };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_phrase_size(p_expr: &Fts5Expr,
    i_phrase: i32) -> i32 {
    if i_phrase < 0 || i_phrase >= (*p_expr).n_phrase { return 0; }
    return unsafe {
            (*unsafe {
                            *(*p_expr).ap_expr_phrase.offset(i_phrase as isize)
                        }).n_term
        };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_poslist(p_expr: &Fts5Expr, i_phrase: i32,
    pa: &mut *const u8) -> i32 {
    let mut n_ret: i32 = 0;
    let p_phrase: *const Fts5ExprPhrase =
        unsafe { *(*p_expr).ap_expr_phrase.offset(i_phrase as isize) } as
            *const Fts5ExprPhrase;
    let p_node: *const Fts5ExprNode =
        unsafe { (*p_phrase).p_node } as *const Fts5ExprNode;
    if unsafe { (*p_node).b_eof } == 0 &&
            unsafe { (*p_node).i_rowid } ==
                unsafe { (*(*p_expr).p_root).i_rowid } {
        *pa = unsafe { (*p_phrase).poslist.p } as *const u8;
        n_ret = unsafe { (*p_phrase).poslist.n };
    } else { *pa = core::ptr::null(); n_ret = 0; }
    return n_ret;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5PoslistPopulator {
    writer: Fts5PoslistWriter,
    b_ok: i32,
    b_miss: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_clear_poslists(p_expr: &Fts5Expr,
    b_live: i32) -> *mut Fts5PoslistPopulator {
    let mut p_ret: *mut Fts5PoslistPopulator = core::ptr::null_mut();
    p_ret =
        unsafe {
                sqlite3_malloc64(core::mem::size_of::<Fts5PoslistPopulator>()
                            as u64 * (*p_expr).n_phrase as u64)
            } as *mut Fts5PoslistPopulator;
    if !(p_ret).is_null() {
        let mut i: i32 = 0;
        unsafe {
            memset(p_ret as *mut (), 0,
                core::mem::size_of::<Fts5PoslistPopulator>() as u64 *
                    (*p_expr).n_phrase as u64)
        };
        {
            i = 0;
            '__b51: loop {
                if !(i < (*p_expr).n_phrase) { break '__b51; }
                '__c51: loop {
                    let p_buf: *mut Fts5Buffer =
                        unsafe {
                            &mut (*unsafe {
                                                *(*p_expr).ap_expr_phrase.offset(i as isize)
                                            }).poslist
                        };
                    let p_node: *const Fts5ExprNode =
                        unsafe {
                                (*unsafe {
                                                *(*p_expr).ap_expr_phrase.offset(i as isize)
                                            }).p_node
                            } as *const Fts5ExprNode;
                    if !(unsafe {
                                                (*unsafe {
                                                                *(*p_expr).ap_expr_phrase.offset(i as isize)
                                                            }).n_term
                                            } <= 1) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"sqlite3Fts5ExprClearPoslists".as_ptr() as
                                    *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                                2996,
                                c"pExpr->apExprPhrase[i]->nTerm<=1".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    if b_live != 0 &&
                            (unsafe { (*p_buf).n } == 0 ||
                                    unsafe { (*p_node).i_rowid } !=
                                        unsafe { (*(*p_expr).p_root).i_rowid } ||
                                unsafe { (*p_node).b_eof } != 0) {
                        unsafe { (*p_ret.offset(i as isize)).b_miss = 1 };
                    } else { unsafe { (*p_buf).n = 0 }; }
                    break '__c51;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return p_ret;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExprCtx {
    p_expr: *mut Fts5Expr,
    a_populator: *mut Fts5PoslistPopulator,
    i_off: i64,
}

extern "C" fn fts5_expr_colset_test(p_colset_1: &Fts5Colset, i_col_1: i32)
    -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b52: loop {
            if !(i < (*p_colset_1).n_col) { break '__b52; }
            '__c52: loop {
                if unsafe {
                            *((*p_colset_1).ai_col.as_ptr() as
                                        *mut i32).offset(i as isize)
                        } == i_col_1 {
                    return 1;
                }
                break '__c52;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

extern "C" fn fts5_query_term(p_token_1: &[i8]) -> i32 {
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b53: loop {
            if !(ii < p_token_1.len() as i32 && p_token_1[ii as usize] != 0) {
                break '__b53;
            }
            '__c53: loop { break '__c53; }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return ii;
}

extern "C" fn fts5_expr_populate_poslists_cb(p_ctx_1: *mut (), tflags: i32,
    p_token_1: *const i8, n_token_1: i32, i_unused1_1: i32, i_unused2_1: i32)
    -> i32 {
    let p: *mut Fts5ExprCtx = p_ctx_1 as *mut Fts5ExprCtx;
    let p_expr: *const Fts5Expr = unsafe { (*p).p_expr } as *const Fts5Expr;
    let mut i: i32 = 0;
    let mut n_query: i32 = n_token_1;
    let i_rowid: i64 = unsafe { (*unsafe { (*p_expr).p_root }).i_rowid };
    { { let _ = i_unused1_1; }; { let _ = i_unused2_1; } };
    if n_query > 32768 { n_query = 32768; }
    if unsafe { (*unsafe { (*p_expr).p_config }).b_tokendata } != 0 {
        n_query =
            fts5_query_term(unsafe {
                    let __p = p_token_1 as *const i8;
                    if __p.is_null() {
                        &[]
                    } else {
                        core::slice::from_raw_parts(__p, n_query as usize)
                    }
                });
    }
    if tflags & 1 == 0 {
        {
            let __p = unsafe { &mut (*p).i_off };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    {
        i = 0;
        '__b54: loop {
            if !(i < unsafe { (*p_expr).n_phrase }) { break '__b54; }
            '__c54: loop {
                let mut p_t: *const Fts5ExprTerm = core::ptr::null();
                if unsafe {
                            (*unsafe { (*p).a_populator.offset(i as isize) }).b_ok
                        } == 0 {
                    break '__c54;
                }
                {
                    p_t =
                        unsafe {
                            &mut *(unsafe {
                                                (*unsafe {
                                                                    *unsafe { (*p_expr).ap_expr_phrase.offset(i as isize) }
                                                                }).a_term.as_ptr()
                                            } as *mut Fts5ExprTerm).offset(0 as isize)
                        };
                    '__b55: loop {
                        if !(!(p_t).is_null()) { break '__b55; }
                        '__c55: loop {
                            if (unsafe { (*p_t).n_query_term } == n_query ||
                                        unsafe { (*p_t).n_query_term } < n_query &&
                                            unsafe { (*p_t).b_prefix } != 0) &&
                                    unsafe {
                                            memcmp(unsafe { (*p_t).p_term } as *const (),
                                                p_token_1 as *const (),
                                                unsafe { (*p_t).n_query_term } as u64)
                                        } == 0 {
                                let mut rc: i32 =
                                    unsafe {
                                        sqlite3_fts5_poslist_writer_append(unsafe {
                                                &mut (*unsafe {
                                                                    *unsafe { (*p_expr).ap_expr_phrase.offset(i as isize) }
                                                                }).poslist
                                            },
                                            unsafe {
                                                &mut (*unsafe {
                                                                (*p).a_populator.offset(i as isize)
                                                            }).writer
                                            }, unsafe { (*p).i_off })
                                    };
                                if rc == 0 &&
                                        (unsafe { (*unsafe { (*p_expr).p_config }).b_tokendata } !=
                                                0 || unsafe { (*p_t).b_prefix } != 0) {
                                    let i_col: i32 = (unsafe { (*p).i_off } >> 32) as i32;
                                    let i_tok_off: i32 =
                                        (unsafe { (*p).i_off } & 2147483647 as i64) as i32;
                                    rc =
                                        unsafe {
                                            sqlite3_fts5_index_iter_write_tokendata(unsafe {
                                                    (*p_t).p_iter
                                                }, p_token_1, n_token_1, i_rowid, i_col, i_tok_off)
                                        };
                                }
                                if rc != 0 { return rc; }
                                break '__b55;
                            }
                            break '__c55;
                        }
                        p_t = unsafe { (*p_t).p_synonym };
                    }
                }
                break '__c54;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_populate_poslists(p_config:
        *mut Fts5Config, p_expr: *mut Fts5Expr,
    a_populator: *mut Fts5PoslistPopulator, i_col: i32, z: *const i8, n: i32)
    -> i32 {
    let mut i: i32 = 0;
    let mut s_ctx: Fts5ExprCtx = unsafe { core::mem::zeroed() };
    s_ctx.p_expr = p_expr;
    s_ctx.a_populator = a_populator;
    s_ctx.i_off = ((i_col as i64) << 32) - 1 as i64;
    {
        i = 0;
        '__b56: loop {
            if !(i < unsafe { (*p_expr).n_phrase }) { break '__b56; }
            '__c56: loop {
                let p_node: *const Fts5ExprNode =
                    unsafe {
                            (*unsafe {
                                            *unsafe { (*p_expr).ap_expr_phrase.offset(i as isize) }
                                        }).p_node
                        } as *const Fts5ExprNode;
                let p_colset: *mut Fts5Colset =
                    unsafe { (*unsafe { (*p_node).p_near }).p_colset };
                if !(p_colset).is_null() &&
                            0 == fts5_expr_colset_test(unsafe { &*p_colset }, i_col) ||
                        unsafe { (*a_populator.offset(i as isize)).b_miss } != 0 {
                    unsafe { (*a_populator.offset(i as isize)).b_ok = 0 };
                } else {
                    unsafe { (*a_populator.offset(i as isize)).b_ok = 1 };
                }
                break '__c56;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return unsafe {
            sqlite3_fts5_tokenize(p_config, 4, z, n,
                &raw mut s_ctx as *mut (),
                Some(fts5_expr_populate_poslists_cb))
        };
}

extern "C" fn fts5_expr_clear_poslists(p_node_1: &Fts5ExprNode) -> () {
    if (*p_node_1).e_type == 4 || (*p_node_1).e_type == 9 {
        unsafe {
            (*unsafe {
                                    *(unsafe { (*(*p_node_1).p_near).ap_phrase.as_ptr() } as
                                                *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                }).poslist.n = 0
        };
    } else {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b57: loop {
                if !(i < (*p_node_1).n_child) { break '__b57; }
                '__c57: loop {
                    fts5_expr_clear_poslists(unsafe {
                            &*unsafe {
                                        *((*p_node_1).ap_child.as_ptr() as
                                                    *mut *mut Fts5ExprNode).offset(i as isize)
                                    }
                        });
                    break '__c57;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn fts5_expr_check_poslists(p_node_1: *mut Fts5ExprNode,
    i_rowid_1: i64) -> i32 {
    unsafe { (*p_node_1).i_rowid = i_rowid_1 };
    unsafe { (*p_node_1).b_eof = 0 };
    '__s58:
        {
        match unsafe { (*p_node_1).e_type } {
            0 => {
                return (unsafe {
                                (*unsafe {
                                                    *(unsafe {
                                                                    (*unsafe { (*p_node_1).p_near }).ap_phrase.as_ptr()
                                                                } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                }).poslist.n
                            } > 0) as i32;
                {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b59: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b59; }
                            '__c59: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) == 0 {
                                    fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                                    return 0;
                                }
                                break '__c59;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s58;
                }
                {
                    let mut i: i32 = 0;
                    let mut b_ret: i32 = 0;
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b60; }
                            '__c60: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) != 0 {
                                    b_ret = 1;
                                }
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return b_ret;
                }
                {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprCheckPoslists".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 3157,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if 0 ==
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }, i_rowid_1) ||
                            0 !=
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(1 as isize)
                                    }, i_rowid_1) {
                        fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                        return 0;
                    }
                    break '__s58;
                }
            }
            4 => {
                return (unsafe {
                                (*unsafe {
                                                    *(unsafe {
                                                                    (*unsafe { (*p_node_1).p_near }).ap_phrase.as_ptr()
                                                                } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                }).poslist.n
                            } > 0) as i32;
                {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b59: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b59; }
                            '__c59: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) == 0 {
                                    fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                                    return 0;
                                }
                                break '__c59;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s58;
                }
                {
                    let mut i: i32 = 0;
                    let mut b_ret: i32 = 0;
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b60; }
                            '__c60: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) != 0 {
                                    b_ret = 1;
                                }
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return b_ret;
                }
                {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprCheckPoslists".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 3157,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if 0 ==
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }, i_rowid_1) ||
                            0 !=
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(1 as isize)
                                    }, i_rowid_1) {
                        fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                        return 0;
                    }
                    break '__s58;
                }
            }
            9 => {
                return (unsafe {
                                (*unsafe {
                                                    *(unsafe {
                                                                    (*unsafe { (*p_node_1).p_near }).ap_phrase.as_ptr()
                                                                } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                                }).poslist.n
                            } > 0) as i32;
                {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b59: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b59; }
                            '__c59: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) == 0 {
                                    fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                                    return 0;
                                }
                                break '__c59;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s58;
                }
                {
                    let mut i: i32 = 0;
                    let mut b_ret: i32 = 0;
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b60; }
                            '__c60: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) != 0 {
                                    b_ret = 1;
                                }
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return b_ret;
                }
                {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprCheckPoslists".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 3157,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if 0 ==
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }, i_rowid_1) ||
                            0 !=
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(1 as isize)
                                    }, i_rowid_1) {
                        fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                        return 0;
                    }
                    break '__s58;
                }
            }
            2 => {
                {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b59: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b59; }
                            '__c59: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) == 0 {
                                    fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                                    return 0;
                                }
                                break '__c59;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__s58;
                }
                {
                    let mut i: i32 = 0;
                    let mut b_ret: i32 = 0;
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b60; }
                            '__c60: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) != 0 {
                                    b_ret = 1;
                                }
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return b_ret;
                }
                {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprCheckPoslists".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 3157,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if 0 ==
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }, i_rowid_1) ||
                            0 !=
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(1 as isize)
                                    }, i_rowid_1) {
                        fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                        return 0;
                    }
                    break '__s58;
                }
            }
            1 => {
                {
                    let mut i: i32 = 0;
                    let mut b_ret: i32 = 0;
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < unsafe { (*p_node_1).n_child }) { break '__b60; }
                            '__c60: loop {
                                if fts5_expr_check_poslists(unsafe {
                                                *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                            *mut *mut Fts5ExprNode).offset(i as isize)
                                            }, i_rowid_1) != 0 {
                                    b_ret = 1;
                                }
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return b_ret;
                }
                {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprCheckPoslists".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 3157,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if 0 ==
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }, i_rowid_1) ||
                            0 !=
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(1 as isize)
                                    }, i_rowid_1) {
                        fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                        return 0;
                    }
                    break '__s58;
                }
            }
            _ => {
                {
                    if !(unsafe { (*p_node_1).e_type } == 3) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"fts5ExprCheckPoslists".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 3157,
                                c"pNode->eType==FTS5_NOT".as_ptr() as *mut i8 as *const i8)
                        }
                    } else { { let _ = 0; } };
                    if 0 ==
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(0 as isize)
                                    }, i_rowid_1) ||
                            0 !=
                                fts5_expr_check_poslists(unsafe {
                                        *(unsafe { (*p_node_1).ap_child.as_ptr() } as
                                                    *mut *mut Fts5ExprNode).offset(1 as isize)
                                    }, i_rowid_1) {
                        fts5_expr_clear_poslists(unsafe { &*p_node_1 });
                        return 0;
                    }
                    break '__s58;
                }
            }
        }
    }
    return 1;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_check_poslists(p_expr: &Fts5Expr,
    i_rowid: i64) -> () {
    fts5_expr_check_poslists((*p_expr).p_root, i_rowid);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TokenCtx {
    p_phrase: *mut Fts5ExprPhrase,
    p_config: *mut Fts5Config,
    rc: i32,
}

extern "C" fn fts5_parse_tokenize(p_context_1: *mut (), tflags: i32,
    p_token_1: *const i8, mut n_token_1: i32, i_unused1_1: i32,
    i_unused2_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let szalloc: i32 = 8 as i32;
    let p_ctx: *mut TokenCtx = p_context_1 as *mut TokenCtx;
    let mut p_phrase: *mut Fts5ExprPhrase = unsafe { (*p_ctx).p_phrase };
    { { let _ = i_unused1_1; }; { let _ = i_unused2_1; } };
    if unsafe { (*p_ctx).rc } != 0 { return unsafe { (*p_ctx).rc }; }
    if n_token_1 > 32768 { n_token_1 = 32768; }
    if !(p_phrase).is_null() && unsafe { (*p_phrase).n_term } > 0 &&
            tflags & 1 != 0 {
        let mut p_syn: *mut Fts5ExprTerm = core::ptr::null_mut();
        let n_byte: Sqlite3Int64 =
            (core::mem::size_of::<Fts5ExprTerm>() as u64 +
                            core::mem::size_of::<Fts5Buffer>() as u64 + n_token_1 as u64
                    + 1 as u64) as Sqlite3Int64;
        p_syn =
            unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                *mut Fts5ExprTerm;
        if p_syn == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe { memset(p_syn as *mut (), 0, n_byte as u64) };
            unsafe {
                (*p_syn).p_term =
                    unsafe {
                        unsafe {
                            (p_syn as
                                        *mut i8).add(core::mem::size_of::<Fts5ExprTerm>() as
                                        usize).add(core::mem::size_of::<Fts5Buffer>() as usize)
                        }
                    }
            };
            unsafe {
                (*p_syn).n_full_term =
                    {
                        unsafe { (*p_syn).n_query_term = n_token_1 };
                        unsafe { (*p_syn).n_query_term }
                    }
            };
            unsafe {
                memcpy(unsafe { (*p_syn).p_term } as *mut (),
                    p_token_1 as *const (), n_token_1 as u64)
            };
            if unsafe { (*unsafe { (*p_ctx).p_config }).b_tokendata } != 0 {
                unsafe {
                    (*p_syn).n_query_term =
                        unsafe { strlen(unsafe { (*p_syn).p_term } as *const i8) }
                            as i32
                };
            }
            unsafe {
                (*p_syn).p_synonym =
                    unsafe {
                        (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                        *mut Fts5ExprTerm).offset((unsafe { (*p_phrase).n_term } -
                                            1) as isize)).p_synonym
                    }
            };
            unsafe {
                (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                    *mut Fts5ExprTerm).offset((unsafe { (*p_phrase).n_term } -
                                        1) as isize)).p_synonym = p_syn
            };
        }
    } else {
        let mut p_term: *mut Fts5ExprTerm = core::ptr::null_mut();
        if p_phrase == core::ptr::null_mut() ||
                unsafe { (*p_phrase).n_term } % szalloc as i32 == 0 {
            let mut p_new: *mut Fts5ExprPhrase = core::ptr::null_mut();
            let n_new: i32 =
                szalloc +
                    if !(p_phrase).is_null() {
                        unsafe { (*p_phrase).n_term }
                    } else { 0 };
            p_new =
                unsafe {
                        sqlite3_realloc64(p_phrase as *mut (),
                            core::mem::offset_of!(Fts5ExprPhrase, a_term) as u64 +
                                (n_new + 1) as u64 *
                                    core::mem::size_of::<Fts5ExprTerm>() as u64)
                    } as *mut Fts5ExprPhrase;
            if p_new == core::ptr::null_mut() {
                rc = 7;
            } else {
                if p_phrase == core::ptr::null_mut() {
                    unsafe {
                        memset(p_new as *mut (), 0,
                            core::mem::offset_of!(Fts5ExprPhrase, a_term) as u64 +
                                1 as u64 * core::mem::size_of::<Fts5ExprTerm>() as u64)
                    };
                }
                unsafe { (*p_ctx).p_phrase = { p_phrase = p_new; p_phrase } };
                unsafe { (*p_new).n_term = n_new - szalloc as i32 };
            }
        }
        if rc == 0 {
            p_term =
                unsafe {
                    &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                                    *mut Fts5ExprTerm).offset({
                                        let __p = unsafe { &mut (*p_phrase).n_term };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                };
            unsafe {
                memset(p_term as *mut (), 0,
                    core::mem::size_of::<Fts5ExprTerm>() as u64)
            };
            unsafe {
                (*p_term).p_term =
                    unsafe {
                        sqlite3_fts5_strndup(&mut rc, p_token_1, n_token_1)
                    }
            };
            unsafe {
                (*p_term).n_full_term =
                    {
                        unsafe { (*p_term).n_query_term = n_token_1 };
                        unsafe { (*p_term).n_query_term }
                    }
            };
            if unsafe { (*unsafe { (*p_ctx).p_config }).b_tokendata } != 0 &&
                    rc == 0 {
                unsafe {
                    (*p_term).n_query_term =
                        unsafe { strlen(unsafe { (*p_term).p_term } as *const i8) }
                            as i32
                };
            }
        }
    }
    unsafe { (*p_ctx).rc = rc };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_clone_phrase(p_expr: *mut Fts5Expr,
    i_phrase: i32, pp_new: &mut *mut Fts5Expr) -> i32 {
    let mut rc: i32 = 0;
    let mut p_orig: *mut Fts5ExprPhrase = core::ptr::null_mut();
    let mut p_new: *mut Fts5Expr = core::ptr::null_mut();
    let mut s_ctx: TokenCtx =
        TokenCtx {
            p_phrase: core::ptr::null_mut(),
            p_config: core::ptr::null_mut(),
            rc: 0,
        };
    if (p_expr).is_null() as i32 != 0 || i_phrase < 0 ||
            i_phrase >= unsafe { (*p_expr).n_phrase } {
        rc = 25;
    } else {
        p_orig =
            unsafe {
                *unsafe { (*p_expr).ap_expr_phrase.offset(i_phrase as isize) }
            };
        p_new =
            unsafe {
                    sqlite3_fts5_malloc_zero(&mut rc,
                        core::mem::size_of::<Fts5Expr>() as Sqlite3Int64)
                } as *mut Fts5Expr;
    }
    if rc == 0 {
        unsafe {
            (*p_new).ap_expr_phrase =
                unsafe {
                        sqlite3_fts5_malloc_zero(&mut rc,
                            core::mem::size_of::<*mut Fts5ExprPhrase>() as Sqlite3Int64)
                    } as *mut *mut Fts5ExprPhrase
        };
    }
    if rc == 0 {
        unsafe {
            (*p_new).p_root =
                unsafe {
                        sqlite3_fts5_malloc_zero(&mut rc,
                            (core::mem::offset_of!(Fts5ExprNode, ap_child) as u64 +
                                    1 as u64 * core::mem::size_of::<*mut Fts5ExprNode>() as u64)
                                as Sqlite3Int64)
                    } as *mut Fts5ExprNode
        };
    }
    if rc == 0 {
        unsafe {
            (*unsafe { (*p_new).p_root }).p_near =
                unsafe {
                        sqlite3_fts5_malloc_zero(&mut rc,
                            (core::mem::offset_of!(Fts5ExprNearset, ap_phrase) as u64 +
                                    2 as u64 *
                                        core::mem::size_of::<*mut Fts5ExprPhrase>() as u64) as
                                Sqlite3Int64)
                    } as *mut Fts5ExprNearset
        };
    }
    if rc == 0 &&
            if p_orig != core::ptr::null_mut() {
                    1
                } else {
                    {
                        if (0 == 0) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"sqlite3Fts5ExprClonePhrase".as_ptr() as
                                        *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                                    1929, c"0".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        0
                    }
                } != 0 {
        let p_colset_orig: *const Fts5Colset =
            unsafe {
                    (*unsafe { (*unsafe { (*p_orig).p_node }).p_near }).p_colset
                } as *const Fts5Colset;
        if !(p_colset_orig).is_null() {
            let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
            let mut p_colset: *mut Fts5Colset = core::ptr::null_mut();
            n_byte =
                (core::mem::size_of::<i64>() as u64 *
                        ((unsafe { (*p_colset_orig).n_col } + 2) / 2) as u64) as
                    Sqlite3Int64;
            p_colset =
                unsafe { sqlite3_fts5_malloc_zero(&mut rc, n_byte) } as
                    *mut Fts5Colset;
            if !(p_colset).is_null() {
                unsafe {
                    memcpy(p_colset as *mut (), p_colset_orig as *const (),
                        n_byte as u64)
                };
            }
            unsafe {
                (*unsafe { (*unsafe { (*p_new).p_root }).p_near }).p_colset =
                    p_colset
            };
        }
    }
    if rc == 0 {
        if unsafe { (*p_orig).n_term } != 0 {
            let mut i: i32 = 0;
            s_ctx.p_config = unsafe { (*p_expr).p_config };
            {
                i = 0;
                '__b61: loop {
                    if !(rc == 0 && i < unsafe { (*p_orig).n_term }) {
                        break '__b61;
                    }
                    '__c61: loop {
                        let mut tflags: i32 = 0;
                        let mut p: *const Fts5ExprTerm = core::ptr::null();
                        {
                            p =
                                unsafe {
                                    &mut *(unsafe { (*p_orig).a_term.as_ptr() } as
                                                    *mut Fts5ExprTerm).offset(i as isize)
                                };
                            '__b62: loop {
                                if !(!(p).is_null() && rc == 0) { break '__b62; }
                                '__c62: loop {
                                    rc =
                                        fts5_parse_tokenize(&raw mut s_ctx as *mut (), tflags,
                                            unsafe { (*p).p_term } as *const i8,
                                            unsafe { (*p).n_full_term }, 0, 0);
                                    tflags = 1;
                                    break '__c62;
                                }
                                p = unsafe { (*p).p_synonym };
                            }
                        }
                        if rc == 0 {
                            unsafe {
                                (*(unsafe { (*s_ctx.p_phrase).a_term.as_ptr() } as
                                                    *mut Fts5ExprTerm).offset(i as isize)).b_prefix =
                                    unsafe {
                                        (*(unsafe { (*p_orig).a_term.as_ptr() } as
                                                        *mut Fts5ExprTerm).offset(i as isize)).b_prefix
                                    }
                            };
                            unsafe {
                                (*(unsafe { (*s_ctx.p_phrase).a_term.as_ptr() } as
                                                    *mut Fts5ExprTerm).offset(i as isize)).b_first =
                                    unsafe {
                                        (*(unsafe { (*p_orig).a_term.as_ptr() } as
                                                        *mut Fts5ExprTerm).offset(i as isize)).b_first
                                    }
                            };
                        }
                        break '__c61;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else {
            s_ctx.p_phrase =
                unsafe {
                        sqlite3_fts5_malloc_zero(&mut rc,
                            (core::mem::offset_of!(Fts5ExprPhrase, a_term) as u64 +
                                    1 as u64 * core::mem::size_of::<Fts5ExprTerm>() as u64) as
                                Sqlite3Int64)
                    } as *mut Fts5ExprPhrase;
        }
    }
    if rc == 0 &&
            if !(s_ctx.p_phrase).is_null() {
                    1
                } else {
                    {
                        if (0 == 0) as i32 as i64 != 0 {
                            unsafe {
                                __assert_rtn(c"sqlite3Fts5ExprClonePhrase".as_ptr() as
                                        *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                                    1966, c"0".as_ptr() as *mut i8 as *const i8)
                            }
                        } else { { let _ = 0; } };
                        0
                    }
                } != 0 {
        unsafe { (*p_new).p_index = unsafe { (*p_expr).p_index } };
        unsafe { (*p_new).p_config = unsafe { (*p_expr).p_config } };
        unsafe { (*p_new).n_phrase = 1 };
        unsafe {
            *unsafe { (*p_new).ap_expr_phrase.offset(0 as isize) } =
                s_ctx.p_phrase
        };
        unsafe {
            *(unsafe {
                                (*unsafe {
                                                    (*unsafe { (*p_new).p_root }).p_near
                                                }).ap_phrase.as_ptr()
                            } as *mut *mut Fts5ExprPhrase).offset(0 as isize) =
                s_ctx.p_phrase
        };
        unsafe {
            (*unsafe { (*unsafe { (*p_new).p_root }).p_near }).n_phrase = 1
        };
        unsafe { (*s_ctx.p_phrase).p_node = unsafe { (*p_new).p_root } };
        if unsafe { (*p_orig).n_term } == 1 &&
                    unsafe {
                            (*(unsafe { (*p_orig).a_term.as_ptr() } as
                                            *mut Fts5ExprTerm).offset(0 as isize)).p_synonym
                        } == core::ptr::null_mut() &&
                unsafe {
                            (*(unsafe { (*p_orig).a_term.as_ptr() } as
                                            *mut Fts5ExprTerm).offset(0 as isize)).b_first
                        } as i32 == 0 {
            unsafe { (*unsafe { (*p_new).p_root }).e_type = 4 };
            unsafe {
                (*unsafe { (*p_new).p_root }).x_next =
                    Some(fts5_expr_node_next_term)
            };
        } else {
            unsafe { (*unsafe { (*p_new).p_root }).e_type = 9 };
            unsafe {
                (*unsafe { (*p_new).p_root }).x_next =
                    Some(fts5_expr_node_next_string)
            };
        }
    } else {
        sqlite3_fts5_expr_free(p_new);
        fts5_expr_phrase_free(s_ctx.p_phrase);
        p_new = core::ptr::null_mut();
    }
    *pp_new = p_new;
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_phrase_collist(p_expr: &Fts5Expr,
    i_phrase: i32, pp_collist: *mut *const u8, pn_collist: *mut i32) -> i32 {
    let p_phrase: *mut Fts5ExprPhrase =
        unsafe { *(*p_expr).ap_expr_phrase.offset(i_phrase as isize) };
    let p_node: *const Fts5ExprNode =
        unsafe { (*p_phrase).p_node } as *const Fts5ExprNode;
    let mut rc: i32 = 0;
    if !(i_phrase >= 0 && i_phrase < (*p_expr).n_phrase) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ExprPhraseCollist".as_ptr() as
                    *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                3187,
                c"iPhrase>=0 && iPhrase<pExpr->nPhrase".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(unsafe { (*(*p_expr).p_config).e_detail } == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ExprPhraseCollist".as_ptr() as
                    *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                3188,
                c"pExpr->pConfig->eDetail==FTS5_DETAIL_COLUMNS".as_ptr() as
                        *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if unsafe { (*p_node).b_eof } == 0 &&
                unsafe { (*p_node).i_rowid } ==
                    unsafe { (*(*p_expr).p_root).i_rowid } &&
            unsafe { (*p_phrase).poslist.n } > 0 {
        let p_term: *mut Fts5ExprTerm =
            unsafe {
                &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                                *mut Fts5ExprTerm).offset(0 as isize)
            };
        if !(unsafe { (*p_term).p_synonym }).is_null() {
            let p_buf: *mut Fts5Buffer =
                unsafe {
                        &raw mut *unsafe { (*p_term).p_synonym.offset(1 as isize) }
                    } as *mut Fts5Buffer;
            rc =
                fts5_expr_synonym_list(p_term, unsafe { (*p_node).i_rowid },
                    p_buf, unsafe { &mut *(pp_collist as *mut *mut u8) },
                    unsafe { &mut *pn_collist });
        } else {
            unsafe {
                *pp_collist =
                    unsafe {
                        (*unsafe {
                                        (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                        *mut Fts5ExprTerm).offset(0 as isize)).p_iter
                                    }).p_data
                    }
            };
            unsafe {
                *pn_collist =
                    unsafe {
                        (*unsafe {
                                        (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                                        *mut Fts5ExprTerm).offset(0 as isize)).p_iter
                                    }).n_data
                    }
            };
        }
    } else {
        unsafe { *pp_collist = core::ptr::null() };
        unsafe { *pn_collist = 0 };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_query_token(p_expr: &Fts5Expr,
    i_phrase: i32, i_token: i32, pp_out: &mut *const i8, pn_out: &mut i32)
    -> i32 {
    let mut p_phrase: *const Fts5ExprPhrase = core::ptr::null();
    if i_phrase < 0 || i_phrase >= (*p_expr).n_phrase { return 25; }
    p_phrase = unsafe { *(*p_expr).ap_expr_phrase.offset(i_phrase as isize) };
    if i_token < 0 || i_token >= unsafe { (*p_phrase).n_term } { return 25; }
    *pp_out =
        unsafe {
                (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                *mut Fts5ExprTerm).offset(i_token as isize)).p_term
            } as *const i8;
    *pn_out =
        unsafe {
            (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                            *mut Fts5ExprTerm).offset(i_token as isize)).n_full_term
        };
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_inst_token(p_expr: &Fts5Expr,
    i_rowid: i64, i_phrase: i32, i_col: i32, i_off: i32, i_token: i32,
    pp_out: *mut *const i8, pn_out: *mut i32) -> i32 {
    let mut p_phrase: *mut Fts5ExprPhrase = core::ptr::null_mut();
    let mut p_term: *const Fts5ExprTerm = core::ptr::null();
    let mut rc: i32 = 0;
    if i_phrase < 0 || i_phrase >= (*p_expr).n_phrase { return 25; }
    p_phrase = unsafe { *(*p_expr).ap_expr_phrase.offset(i_phrase as isize) };
    if i_token < 0 || i_token >= unsafe { (*p_phrase).n_term } { return 25; }
    p_term =
        unsafe {
            &mut *(unsafe { (*p_phrase).a_term.as_ptr() } as
                            *mut Fts5ExprTerm).offset(i_token as isize)
        };
    if unsafe { (*(*p_expr).p_config).b_tokendata } != 0 ||
            unsafe { (*p_term).b_prefix } != 0 {
        rc =
            unsafe {
                sqlite3_fts5_iter_token(unsafe { (*p_term).p_iter },
                    unsafe { (*p_term).p_term } as *const i8,
                    unsafe { (*p_term).n_query_term }, i_rowid, i_col,
                    i_off + i_token, pp_out, pn_out)
            };
    } else {
        unsafe { *pp_out = unsafe { (*p_term).p_term } as *const i8 };
        unsafe { *pn_out = unsafe { (*p_term).n_full_term } };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_expr_clear_tokens(p_expr: &Fts5Expr) -> () {
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b63: loop {
            if !(ii < (*p_expr).n_phrase) { break '__b63; }
            '__c63: loop {
                let mut p_t: *const Fts5ExprTerm = core::ptr::null();
                {
                    p_t =
                        unsafe {
                            &mut *(unsafe {
                                                (*unsafe {
                                                                    *(*p_expr).ap_expr_phrase.offset(ii as isize)
                                                                }).a_term.as_ptr()
                                            } as *mut Fts5ExprTerm).offset(0 as isize)
                        };
                    '__b64: loop {
                        if !(!(p_t).is_null()) { break '__b64; }
                        '__c64: loop {
                            unsafe {
                                sqlite3_fts5_index_iter_clear_tokendata(unsafe {
                                        (*p_t).p_iter
                                    })
                            };
                            break '__c64;
                        }
                        p_t = unsafe { (*p_t).p_synonym };
                    }
                }
                break '__c63;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_implicit_and(p_parse: *mut Fts5Parse,
    p_left: *mut Fts5ExprNode, p_right: *mut Fts5ExprNode)
    -> *mut Fts5ExprNode {
    let mut p_ret: *mut Fts5ExprNode = core::ptr::null_mut();
    let mut p_prev: *mut Fts5ExprNode = core::ptr::null_mut();
    if unsafe { (*p_parse).rc } != 0 {
        sqlite3_fts5_parse_node_free(p_left);
        sqlite3_fts5_parse_node_free(p_right);
    } else {
        if !(unsafe { (*p_left).e_type } == 9 ||
                                        unsafe { (*p_left).e_type } == 4 ||
                                    unsafe { (*p_left).e_type } == 0 ||
                                unsafe { (*p_left).e_type } == 2) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                        *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                    2477,
                    c"pLeft->eType==FTS5_STRING || pLeft->eType==FTS5_TERM || pLeft->eType==FTS5_EOF || pLeft->eType==FTS5_AND".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(unsafe { (*p_right).e_type } == 9 ||
                                        unsafe { (*p_right).e_type } == 4 ||
                                    unsafe { (*p_right).e_type } == 0 ||
                                unsafe { (*p_right).e_type } == 2 &&
                                    unsafe { (*p_parse).b_phrase_to_and } != 0) as i32 as i64 !=
                0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                        *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                    2482,
                    c"pRight->eType==FTS5_STRING || pRight->eType==FTS5_TERM || pRight->eType==FTS5_EOF || (pRight->eType==FTS5_AND && pParse->bPhraseToAnd)".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if unsafe { (*p_left).e_type } == 2 {
            p_prev =
                unsafe {
                    *(unsafe { (*p_left).ap_child.as_ptr() } as
                                *mut *mut Fts5ExprNode).offset((unsafe { (*p_left).n_child }
                                    - 1) as isize)
                };
        } else { p_prev = p_left; }
        if !(unsafe { (*p_prev).e_type } == 9 ||
                                    unsafe { (*p_prev).e_type } == 4 ||
                                unsafe { (*p_prev).e_type } == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                        *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                    2492,
                    c"pPrev->eType==FTS5_STRING || pPrev->eType==FTS5_TERM || pPrev->eType==FTS5_EOF".as_ptr()
                            as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if unsafe { (*p_right).e_type } == 0 {
            if !(unsafe { (*p_parse).ap_phrase } != core::ptr::null_mut()) as
                            i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        2495,
                        c"pParse->apPhrase!=0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if !(unsafe { (*p_parse).n_phrase } > 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        2496, c"pParse->nPhrase>0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            if !(unsafe {
                                        *unsafe {
                                                (*p_parse).ap_phrase.offset((unsafe { (*p_parse).n_phrase }
                                                            - 1) as isize)
                                            }
                                    } ==
                                    unsafe {
                                        *(unsafe {
                                                        (*unsafe { (*p_right).p_near }).ap_phrase.as_ptr()
                                                    } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                    }) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        2497,
                        c"pParse->apPhrase[pParse->nPhrase-1]==pRight->pNear->apPhrase[0]".as_ptr()
                                as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            sqlite3_fts5_parse_node_free(p_right);
            p_ret = p_left;
            {
                let __p = unsafe { &mut (*p_parse).n_phrase };
                let __t = *__p;
                *__p -= 1;
                __t
            };
        } else if unsafe { (*p_prev).e_type } == 0 {
            let mut ap: *mut *mut Fts5ExprPhrase = core::ptr::null_mut();
            if p_prev == p_left {
                p_ret = p_right;
            } else {
                unsafe {
                    *(unsafe { (*p_left).ap_child.as_ptr() } as
                                    *mut *mut Fts5ExprNode).offset((unsafe { (*p_left).n_child }
                                        - 1) as isize) = p_right
                };
                p_ret = p_left;
            }
            ap =
                unsafe {
                    unsafe {
                        (*p_parse).ap_phrase.offset((unsafe { (*p_parse).n_phrase }
                                        - 1 - unsafe { (*unsafe { (*p_right).p_near }).n_phrase })
                                as isize)
                    }
                };
            if !(unsafe { *ap.offset(0 as isize) } ==
                                    unsafe {
                                        *(unsafe {
                                                        (*unsafe { (*p_prev).p_near }).ap_phrase.as_ptr()
                                                    } as *mut *mut Fts5ExprPhrase).offset(0 as isize)
                                    }) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"sqlite3Fts5ParseImplicitAnd".as_ptr() as
                            *const i8, c"fts5_expr.c".as_ptr() as *mut i8 as *const i8,
                        2513,
                        c"ap[0]==pPrev->pNear->apPhrase[0]".as_ptr() as *mut i8 as
                            *const i8)
                }
            } else { { let _ = 0; } };
            unsafe {
                memmove(ap as *mut (),
                    unsafe { &raw mut *ap.offset(1 as isize) } as *const (),
                    core::mem::size_of::<*mut Fts5ExprPhrase>() as u64 *
                        unsafe { (*unsafe { (*p_right).p_near }).n_phrase } as u64)
            };
            {
                let __p = unsafe { &mut (*p_parse).n_phrase };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            sqlite3_fts5_parse_node_free(p_prev);
        } else {
            p_ret =
                sqlite3_fts5_parse_node(p_parse, 2, p_left, p_right,
                    core::ptr::null_mut());
        }
    }
    return p_ret;
}

extern "C" fn fts5_parse_string_from_token(p_token_1: &Fts5Token,
    pz: &mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    *pz =
        unsafe {
            sqlite3_fts5_strndup(&mut rc, (*p_token_1).p, (*p_token_1).n)
        };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_term(p_parse: *mut Fts5Parse,
    p_append: *mut Fts5ExprPhrase, p_token: *mut Fts5Token, b_prefix: i32)
    -> *mut Fts5ExprPhrase {
    let p_config: *mut Fts5Config = unsafe { (*p_parse).p_config };
    let mut s_ctx: TokenCtx = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut z: *mut i8 = core::ptr::null_mut();
    unsafe {
        memset(&raw mut s_ctx as *mut (), 0,
            core::mem::size_of::<TokenCtx>() as u64)
    };
    s_ctx.p_phrase = p_append;
    s_ctx.p_config = p_config;
    rc = fts5_parse_string_from_token(unsafe { &*p_token }, &mut z);
    if rc == 0 {
        let flags: i32 = 1 | if b_prefix != 0 { 2 } else { 0 };
        let mut n: i32 = 0;
        unsafe { sqlite3_fts5_dequote(z) };
        n = unsafe { strlen(z as *const i8) } as i32;
        rc =
            unsafe {
                sqlite3_fts5_tokenize(p_config, flags, z as *const i8, n,
                    &raw mut s_ctx as *mut (), Some(fts5_parse_tokenize))
            };
    }
    unsafe { sqlite3_free(z as *mut ()) };
    if rc != 0 || { rc = s_ctx.rc; rc } != 0 {
        unsafe { (*p_parse).rc = rc };
        fts5_expr_phrase_free(s_ctx.p_phrase);
        s_ctx.p_phrase = core::ptr::null_mut();
    } else {
        if p_append == core::ptr::null_mut() {
            if parse_grow_phrase_array(unsafe { &mut *p_parse }) != 0 {
                fts5_expr_phrase_free(s_ctx.p_phrase);
                return core::ptr::null_mut();
            }
            {
                let __p = unsafe { &mut (*p_parse).n_phrase };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        if s_ctx.p_phrase == core::ptr::null_mut() {
            s_ctx.p_phrase =
                unsafe {
                        sqlite3_fts5_malloc_zero(unsafe { &mut (*p_parse).rc },
                            (core::mem::offset_of!(Fts5ExprPhrase, a_term) as u64 +
                                    1 as u64 * core::mem::size_of::<Fts5ExprTerm>() as u64) as
                                Sqlite3Int64)
                    } as *mut Fts5ExprPhrase;
        } else if unsafe { (*s_ctx.p_phrase).n_term } != 0 {
            unsafe {
                (*(unsafe { (*s_ctx.p_phrase).a_term.as_ptr() } as
                                    *mut Fts5ExprTerm).offset((unsafe {
                                            (*s_ctx.p_phrase).n_term
                                        } - 1) as isize)).b_prefix = b_prefix as u8
            };
        }
        if !(unsafe { (*p_parse).ap_phrase } != core::ptr::null_mut()) as i32
                    as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseTerm".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1892,
                    c"pParse->apPhrase!=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            *unsafe {
                        (*p_parse).ap_phrase.offset((unsafe { (*p_parse).n_phrase }
                                    - 1) as isize)
                    } = s_ctx.p_phrase
        };
    }
    return s_ctx.p_phrase;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_set_caret(p_phrase: *mut Fts5ExprPhrase)
    -> () {
    if !(p_phrase).is_null() && unsafe { (*p_phrase).n_term } != 0 {
        unsafe {
            (*(unsafe { (*p_phrase).a_term.as_ptr() } as
                                *mut Fts5ExprTerm).offset(0 as isize)).b_first = 1 as u8
        };
    }
}

extern "C" fn fts5_parse_colset(p_parse_1: &mut Fts5Parse, p: *mut Fts5Colset,
    i_col_1: i32) -> *mut Fts5Colset {
    let n_col: i32 = if !(p).is_null() { unsafe { (*p).n_col } } else { 0 };
    let mut p_new: *mut Fts5Colset = core::ptr::null_mut();
    if !((*p_parse_1).rc == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ParseColset".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2054,
                c"pParse->rc==SQLITE_OK".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if !(i_col_1 >= 0 && i_col_1 < unsafe { (*(*p_parse_1).p_config).n_col })
                    as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5ParseColset".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2055,
                c"iCol>=0 && iCol<pParse->pConfig->nCol".as_ptr() as *mut i8
                    as *const i8)
        }
    } else { { let _ = 0; } };
    p_new =
        unsafe {
                sqlite3_realloc64(p as *mut (),
                    core::mem::size_of::<i64>() as u64 *
                        ((n_col + 1 + 2) / 2) as u64)
            } as *mut Fts5Colset;
    if p_new == core::ptr::null_mut() {
        (*p_parse_1).rc = 7;
    } else {
        let ai_col: *mut i32 =
            unsafe { (*p_new).ai_col.as_ptr() } as *mut i32;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        {
            i = 0;
            '__b65: loop {
                if !(i < n_col) { break '__b65; }
                '__c65: loop {
                    if unsafe { *ai_col.offset(i as isize) } == i_col_1 {
                        return p_new;
                    }
                    if unsafe { *ai_col.offset(i as isize) } > i_col_1 {
                        break '__b65;
                    }
                    break '__c65;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            j = n_col;
            '__b66: loop {
                if !(j > i) { break '__b66; }
                '__c66: loop {
                    unsafe {
                        *ai_col.offset(j as isize) =
                            unsafe { *ai_col.offset((j - 1) as isize) }
                    };
                    break '__c66;
                }
                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
            }
        }
        unsafe { *ai_col.offset(i as isize) = i_col_1 };
        unsafe { (*p_new).n_col = n_col + 1 };
        {
            i = 1;
            '__b67: loop {
                if !(i < unsafe { (*p_new).n_col }) { break '__b67; }
                '__c67: loop {
                    if !(unsafe {
                                                *(unsafe { (*p_new).ai_col.as_ptr() } as
                                                            *mut i32).offset(i as isize)
                                            } >
                                            unsafe {
                                                *(unsafe { (*p_new).ai_col.as_ptr() } as
                                                            *mut i32).offset((i - 1) as isize)
                                            }) as i32 as i64 != 0 {
                        unsafe {
                            __assert_rtn(c"fts5ParseColset".as_ptr() as *const i8,
                                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2075,
                                c"pNew->aiCol[i]>pNew->aiCol[i-1]".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    break '__c67;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return p_new;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_colset(p_parse: *mut Fts5Parse,
    p_colset: *mut Fts5Colset, p: &Fts5Token) -> *mut Fts5Colset {
    let mut p_ret: *mut Fts5Colset = core::ptr::null_mut();
    let mut i_col: i32 = 0;
    let mut z: *mut i8 = core::ptr::null_mut();
    z =
        unsafe {
            sqlite3_fts5_strndup(unsafe { &mut (*p_parse).rc }, (*p).p,
                (*p).n)
        };
    if unsafe { (*p_parse).rc } == 0 {
        let p_config: *const Fts5Config =
            unsafe { (*p_parse).p_config } as *const Fts5Config;
        unsafe { sqlite3_fts5_dequote(z) };
        {
            i_col = 0;
            '__b68: loop {
                if !(i_col < unsafe { (*p_config).n_col }) { break '__b68; }
                '__c68: loop {
                    if 0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe {
                                            *unsafe { (*p_config).az_col.offset(i_col as isize) }
                                        } as *const i8, z as *const i8)
                            } {
                        break '__b68;
                    }
                    break '__c68;
                }
                { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
            }
        }
        if i_col == unsafe { (*p_config).n_col } {
            unsafe {
                sqlite3_fts5_parse_error(unsafe { &mut *p_parse },
                    c"no such column: %s".as_ptr() as *mut i8 as *const i8, z)
            };
        } else {
            p_ret =
                fts5_parse_colset(unsafe { &mut *p_parse }, p_colset, i_col);
        }
        unsafe { sqlite3_free(z as *mut ()) };
    }
    if p_ret == core::ptr::null_mut() {
        if !(unsafe { (*p_parse).rc } != 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"sqlite3Fts5ParseColset".as_ptr() as *const i8,
                    c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 2135,
                    c"pParse->rc!=SQLITE_OK".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { sqlite3_free(p_colset as *mut ()) };
    }
    return p_ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_set_distance(p_parse: *mut Fts5Parse,
    p_near: *mut Fts5ExprNearset, p: &Fts5Token) -> () {
    if !(p_near).is_null() {
        let mut n_near: i32 = 0;
        let mut i: i32 = 0;
        if (*p).n != 0 {
            {
                i = 0;
                '__b69: loop {
                    if !(i < (*p).n) { break '__b69; }
                    '__c69: loop {
                        let c: i8 = unsafe { *(*p).p.offset(i as isize) } as i8;
                        if (c as i32) < '0' as i32 || c as i32 > '9' as i32 {
                            unsafe {
                                sqlite3_fts5_parse_error(unsafe { &mut *p_parse },
                                    c"expected integer, got \"%.*s\"".as_ptr() as *mut i8 as
                                        *const i8, (*p).n, (*p).p)
                            };
                            return;
                        }
                        if n_near < 214748363 {
                            n_near =
                                n_near * 10 +
                                    (unsafe { *(*p).p.offset(i as isize) } as i32 - '0' as i32);
                        }
                        break '__c69;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else { n_near = 10; }
        unsafe { (*p_near).n_near = n_near };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_colset_invert(p_parse: &mut Fts5Parse,
    p: *mut Fts5Colset) -> *mut Fts5Colset {
    let mut p_ret: *mut Fts5Colset = core::ptr::null_mut();
    let n_col: i32 = unsafe { (*(*p_parse).p_config).n_col };
    p_ret =
        unsafe {
                sqlite3_fts5_malloc_zero(&mut (*p_parse).rc,
                    (core::mem::size_of::<i64>() as u64 *
                            ((n_col + 1 + 2) / 2) as u64) as Sqlite3Int64)
            } as *mut Fts5Colset;
    if !(p_ret).is_null() {
        let mut i: i32 = 0;
        let mut i_old: i32 = 0;
        {
            i = 0;
            '__b70: loop {
                if !(i < n_col) { break '__b70; }
                '__c70: loop {
                    if i_old >= unsafe { (*p).n_col } ||
                            unsafe {
                                    *(unsafe { (*p).ai_col.as_ptr() } as
                                                *mut i32).offset(i_old as isize)
                                } != i {
                        unsafe {
                            *(unsafe { (*p_ret).ai_col.as_ptr() } as
                                            *mut i32).offset({
                                                let __p = unsafe { &mut (*p_ret).n_col };
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = i
                        };
                    } else {
                        { let __p = &mut i_old; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c70;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    unsafe { sqlite3_free(p as *mut ()) };
    return p_ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_finished(p_parse: &mut Fts5Parse,
    p: *mut Fts5ExprNode) -> () {
    if !((*p_parse).p_expr == core::ptr::null_mut()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"sqlite3Fts5ParseFinished".as_ptr() as *const i8,
                c"fts5_expr.c".as_ptr() as *mut i8 as *const i8, 1824,
                c"pParse->pExpr==0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    (*p_parse).p_expr = p;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_parse_near(p_parse: *mut Fts5Parse,
    p_tok: &Fts5Token) -> () {
    if (*p_tok).n != 4 ||
            unsafe {
                    memcmp(c"NEAR".as_ptr() as *mut i8 as *const (),
                        (*p_tok).p as *const (), 4 as u64)
                } != 0 {
        unsafe {
            sqlite3_fts5_parse_error(unsafe { &mut *p_parse },
                c"fts5: syntax error near \"%.*s\"".as_ptr() as *mut i8 as
                    *const i8, (*p_tok).n, (*p_tok).p)
        };
    }
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
    fn sqlite3_fts5_config_parse(_: *mut Fts5Global, _: *mut Sqlite3, _: i32,
    _: *mut *const i8, _: *mut *mut Fts5Config, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_config_free(_: *mut Fts5Config)
    -> ();
    fn sqlite3_fts5_config_declare_vtab(p_config_1: *mut Fts5Config)
    -> i32;
    fn sqlite3_fts5_tokenize(p_config_1: *mut Fts5Config, flags: i32,
    p_text_1: *const i8, n_text_1: i32, p_ctx_1: *mut (),
    x_token_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32)
            -> i32>)
    -> i32;
    fn sqlite3_fts5_dequote(z: *mut i8)
    -> ();
    fn sqlite3_fts5_config_load(_: *mut Fts5Config, _: i32)
    -> i32;
    fn sqlite3_fts5_config_set_value(_: *mut Fts5Config, _: *const i8,
    _: *mut Sqlite3Value, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_config_parse_rank(_: *const i8, _: *mut *mut i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_config_errmsg(p_config_1: *mut Fts5Config,
    z_fmt_1: *const i8, ...)
    -> ();
    fn sqlite3_fts5_buffer_size(_: *mut i32, _: *mut Fts5Buffer, _: u32)
    -> i32;
    fn sqlite3_fts5_buffer_append_varint(_: *mut i32, _: *mut Fts5Buffer,
    _: i64)
    -> ();
    fn sqlite3_fts5_buffer_append_blob(_: *mut i32, _: *mut Fts5Buffer,
    _: u32, _: *const u8)
    -> ();
    fn sqlite3_fts5_buffer_append_string(_: *mut i32, _: *mut Fts5Buffer,
    _: *const i8)
    -> ();
    fn sqlite3_fts5_buffer_free(_: *mut Fts5Buffer)
    -> ();
    fn sqlite3_fts5_buffer_zero(_: *mut Fts5Buffer)
    -> ();
    fn sqlite3_fts5_buffer_set(_: *mut i32, _: *mut Fts5Buffer, _: i32,
    _: *const u8)
    -> ();
    fn sqlite3_fts5_buffer_append_printf(_: *mut i32, _: *mut Fts5Buffer,
    z_fmt_1: *mut i8, ...)
    -> ();
    fn sqlite3_fts5_mprintf(p_rc_1: *mut i32, z_fmt_1: *const i8, ...)
    -> *mut i8;
    fn sqlite3_fts5_put32(_: *mut u8, _: i32)
    -> ();
    fn sqlite3_fts5_get32(_: *const u8)
    -> i32;
    fn sqlite3_fts5_poslist_reader_init(a: *const u8, n: i32,
    p_iter_1: *mut Fts5PoslistReader)
    -> i32;
    fn sqlite3_fts5_poslist_reader_next(_: *mut Fts5PoslistReader)
    -> i32;
    fn sqlite3_fts5_poslist_writer_append(_: *mut Fts5Buffer,
    _: *mut Fts5PoslistWriter, _: i64)
    -> i32;
    fn sqlite3_fts5_poslist_safe_append(_: *mut Fts5Buffer, _: *mut i64,
    _: i64)
    -> ();
    fn sqlite3_fts5_poslist_next64(a: *const u8, n: i32, pi: *mut i32,
    pi_off_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_malloc_zero(p_rc_1: *mut i32, n_byte_1: Sqlite3Int64)
    -> *mut ();
    fn sqlite3_fts5_strndup(p_rc_1: *mut i32, p_in_1: *const i8, n_in_1: i32)
    -> *mut i8;
    fn sqlite3_fts5_is_bareword(t: i8)
    -> i32;
    fn sqlite3_fts5_termset_new(_: *mut *mut Fts5Termset)
    -> i32;
    fn sqlite3_fts5_termset_add(_: *mut Fts5Termset, _: i32, _: *const i8,
    _: i32, pb_present_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_termset_free(_: *mut Fts5Termset)
    -> ();
    fn sqlite3_fts5_index_open(p_config_1: *mut Fts5Config, b_create_1: i32,
    _: *mut *mut Fts5Index, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_index_close(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_entry_cksum(i_rowid_1: i64, i_col_1: i32,
    i_pos_1: i32, i_idx_1: i32, p_term_1: *const i8, n_term_1: i32)
    -> u64;
    fn sqlite3_fts5_index_charlen_to_bytelen(p: *const i8, n_byte_1: i32,
    n_char_1: i32)
    -> i32;
    fn sqlite3_fts5_index_query(p: *mut Fts5Index, p_token_1: *const i8,
    n_token_1: i32, flags: i32, p_colset_1: *mut Fts5Colset,
    pp_iter_1: *mut *mut Fts5IndexIter)
    -> i32;
    fn sqlite3_fts5_iter_next(_: *mut Fts5IndexIter)
    -> i32;
    fn sqlite3_fts5_iter_next_from(_: *mut Fts5IndexIter, i_match_1: i64)
    -> i32;
    fn sqlite3_fts5_iter_close(_: *mut Fts5IndexIter)
    -> ();
    fn sqlite3_fts5_index_close_reader(_: *mut Fts5Index)
    -> ();
    fn sqlite3_fts5_iter_term(_: *mut Fts5IndexIter, _: *mut i32)
    -> *const i8;
    fn sqlite3_fts5_iter_next_scan(_: *mut Fts5IndexIter)
    -> i32;
    fn sqlite3_fts5_structure_ref(_: *mut Fts5Index)
    -> *mut ();
    fn sqlite3_fts5_structure_release(_: *mut ())
    -> ();
    fn sqlite3_fts5_structure_test(_: *mut Fts5Index, _: *mut ())
    -> i32;
    fn sqlite3_fts5_iter_token(p_index_iter_1: *mut Fts5IndexIter,
    p_token_1: *const i8, n_token_1: i32, i_rowid_1: i64, i_col_1: i32,
    i_off_1: i32, pp_out_1: *mut *const i8, pn_out_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_index_write(p: *mut Fts5Index, i_col_1: i32, i_pos_1: i32,
    p_token_1: *const i8, n_token_1: i32)
    -> i32;
    fn sqlite3_fts5_index_begin_write(p: *mut Fts5Index, b_delete_1: i32,
    i_docid_1: i64)
    -> i32;
    fn sqlite3_fts5_index_sync(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_rollback(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_get_averages(p: *mut Fts5Index, pn_row_1: *mut i64,
    an_size_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_index_set_averages(p: *mut Fts5Index, _: *const u8,
    _: i32)
    -> i32;
    fn sqlite3_fts5_index_integrity_check(_: *mut Fts5Index, cksum: u64,
    b_use_cksum_1: i32)
    -> i32;
    fn sqlite3_fts5_index_init(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_fts5_index_set_cookie(_: *mut Fts5Index, _: i32)
    -> i32;
    fn sqlite3_fts5_index_reads(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_reinit(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_optimize(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_merge(p: *mut Fts5Index, n_merge_1: i32)
    -> i32;
    fn sqlite3_fts5_index_reset(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_load_config(p: *mut Fts5Index)
    -> i32;
    fn sqlite3_fts5_index_get_origin(p: *mut Fts5Index, pi_origin_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_index_contentless_delete(p: *mut Fts5Index,
    i_origin_1: i64, i_rowid_1: i64)
    -> i32;
    fn sqlite3_fts5_index_iter_clear_tokendata(_: *mut Fts5IndexIter)
    -> ();
    fn sqlite3_fts5_index_iter_write_tokendata(_: *mut Fts5IndexIter,
    _: *const i8, _: i32, i_rowid_1: i64, i_col_1: i32, i_off_1: i32)
    -> i32;
    fn sqlite3_fts5_get_varint32(p: *const u8, v: *mut u32)
    -> i32;
    fn sqlite3_fts5_get_varint_len(i_val_1: u32)
    -> i32;
    fn sqlite3_fts5_get_varint(_: *const u8, _: *mut u64)
    -> u8;
    fn sqlite3_fts5_put_varint(p: *mut u8, v: u64)
    -> i32;
    fn sqlite3_fts5_load_tokenizer(p_config_1: *mut Fts5Config)
    -> i32;
    fn sqlite3_fts5_table_from_csrid(_: *mut Fts5Global, _: i64)
    -> *mut Fts5Table;
    fn sqlite3_fts5_flush_to_disk(_: *mut Fts5Table)
    -> i32;
    fn sqlite3_fts5_clear_locale(p_config_1: *mut Fts5Config)
    -> ();
    fn sqlite3_fts5_set_locale(p_config_1: *mut Fts5Config,
    p_loc_1: *const i8, n_loc_1: i32)
    -> ();
    fn sqlite3_fts5_is_locale_value(p_config_1: *mut Fts5Config,
    p_val_1: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_fts5_decode_locale_value(p_val_1: *mut Sqlite3Value,
    pp_text_1: *mut *const i8, pn_text_1: *mut i32, pp_loc_1: *mut *const i8,
    pn_loc_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_hash_new(_: *mut Fts5Config, _: *mut *mut Fts5Hash,
    pn_size_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_hash_free(_: *mut Fts5Hash)
    -> ();
    fn sqlite3_fts5_hash_write(_: *mut Fts5Hash, i_rowid_1: i64, i_col_1: i32,
    i_pos_1: i32, b_byte_1: i8, p_token_1: *const i8, n_token_1: i32)
    -> i32;
    fn sqlite3_fts5_hash_clear(_: *mut Fts5Hash)
    -> ();
    fn sqlite3_fts5_hash_is_empty(_: *mut Fts5Hash)
    -> i32;
    fn sqlite3_fts5_hash_query(_: *mut Fts5Hash, n_pre_1: i32,
    p_term_1: *const i8, n_term_1: i32, pp_obj_1: *mut *mut (),
    pn_doclist_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_hash_scan_init(_: *mut Fts5Hash, p_term_1: *const i8,
    n_term_1: i32)
    -> i32;
    fn sqlite3_fts5_hash_scan_next(_: *mut Fts5Hash)
    -> ();
    fn sqlite3_fts5_hash_scan_eof(_: *mut Fts5Hash)
    -> i32;
    fn sqlite3_fts5_hash_scan_entry(_: *mut Fts5Hash,
    pz_term_1: *mut *const i8, pn_term_1: *mut i32,
    pp_doclist_1: *mut *const u8, pn_doclist_1: *mut i32)
    -> ();
    fn sqlite3_fts5_storage_open(_: *mut Fts5Config, _: *mut Fts5Index,
    _: i32, _: *mut *mut Fts5Storage, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_storage_close(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_rename(_: *mut Fts5Storage, z_name_1: *const i8)
    -> i32;
    fn sqlite3_fts5_drop_all(_: *mut Fts5Config)
    -> i32;
    fn sqlite3_fts5_create_table(_: *mut Fts5Config, _: *const i8,
    _: *const i8, _: i32, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_storage_delete(p: *mut Fts5Storage, _: i64,
    _: *mut *mut Sqlite3Value, _: i32)
    -> i32;
    fn sqlite3_fts5_storage_content_insert(p: *mut Fts5Storage, _: i32,
    _: *mut *mut Sqlite3Value, _: *mut i64)
    -> i32;
    fn sqlite3_fts5_storage_index_insert(p: *mut Fts5Storage,
    _: *mut *mut Sqlite3Value, _: i64)
    -> i32;
    fn sqlite3_fts5_storage_integrity(p: *mut Fts5Storage, i_arg_1: i32)
    -> i32;
    fn sqlite3_fts5_storage_stmt(p: *mut Fts5Storage, e_stmt_1: i32,
    _: *mut *mut Sqlite3Stmt, _: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_storage_stmt_release(p: *mut Fts5Storage, e_stmt_1: i32,
    _: *mut Sqlite3Stmt)
    -> ();
    fn sqlite3_fts5_storage_docsize(p: *mut Fts5Storage, i_rowid_1: i64,
    a_col_1: *mut i32)
    -> i32;
    fn sqlite3_fts5_storage_size(p: *mut Fts5Storage, i_col_1: i32,
    pn_avg_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_storage_row_count(p: *mut Fts5Storage, pn_row_1: *mut i64)
    -> i32;
    fn sqlite3_fts5_storage_sync(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_rollback(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_config_value(p: *mut Fts5Storage, _: *const i8,
    _: *mut Sqlite3Value, _: i32)
    -> i32;
    fn sqlite3_fts5_storage_delete_all(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_rebuild(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_optimize(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_merge(p: *mut Fts5Storage, n_merge_1: i32)
    -> i32;
    fn sqlite3_fts5_storage_reset(p: *mut Fts5Storage)
    -> i32;
    fn sqlite3_fts5_storage_release_delete_row(_: *mut Fts5Storage)
    -> ();
    fn sqlite3_fts5_storage_find_delete_row(p: *mut Fts5Storage, i_del_1: i64)
    -> i32;
    fn sqlite3_fts5_parser_alloc(malloc_proc_1:
        Option<unsafe extern "C" fn(u64) -> *mut ()>)
    -> *mut ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_fts5_parser(_: *mut (), _: i32, _: Fts5Token,
    _: *mut Fts5Parse)
    -> ();
    fn sqlite3_fts5_parser_free(_: *mut (),
    free_proc_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn sqlite3_fts5_parser_trace(_: *mut FILE, _: *mut i8)
    -> ();
    fn sqlite3_fts5_parser_fallback(_: i32)
    -> i32;
    fn sqlite3_fts5_aux_init(_: *mut Fts5Api)
    -> i32;
    fn sqlite3_fts5_tokenizer_init(_: *mut Fts5Api)
    -> i32;
    fn sqlite3_fts5_tokenizer_pattern(x_create_1:
        Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
            *mut *mut Fts5Tokenizer) -> i32>, p_tok_1: *mut Fts5Tokenizer)
    -> i32;
    fn sqlite3_fts5_tokenizer_preload(_: *mut Fts5TokenizerConfig)
    -> i32;
    fn sqlite3_fts5_vocab_init(_: *mut Fts5Global, _: *mut Sqlite3)
    -> i32;
    fn sqlite3_fts5_unicode_isdiacritic(c: i32)
    -> i32;
    fn sqlite3_fts5_unicode_fold(c: i32, b_remove_diacritic_1: i32)
    -> i32;
    fn sqlite3_fts5_unicode_cat_parse(_: *const i8, _: *mut u8)
    -> i32;
    fn sqlite3_fts5_unicode_category(i_code_1: u32)
    -> i32;
    fn sqlite3_fts5_unicode_ascii(_: *mut u8, _: *mut u8)
    -> ();
    fn __builtin_unreachable()
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
