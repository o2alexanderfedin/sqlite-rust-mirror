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
struct HighlightContext {
    i_range_start: i32,
    i_range_end: i32,
    z_open: *const i8,
    z_close: *const i8,
    z_in: *const i8,
    n_in: i32,
    iter: CInstIter,
    i_pos: i32,
    i_off: i32,
    b_open: i32,
    z_out: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CInstIter {
    p_api: *const Fts5ExtensionApi,
    p_fts: *mut Fts5Context,
    i_col: i32,
    i_inst: i32,
    n_inst: i32,
    i_start: i32,
    i_end: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5SFinder {
    i_pos: i32,
    n_first_alloc: i32,
    n_first: i32,
    a_first: *mut i32,
    z_doc: *const i8,
}

extern "C" fn fts5_value_to_text(p_val_1: *mut Sqlite3Value) -> *const i8 {
    let z_ret: *const i8 =
        unsafe { sqlite3_value_text(p_val_1) } as *const i8;
    return if !(z_ret).is_null() {
            z_ret
        } else { c"".as_ptr() as *mut i8 as *const i8 };
}

extern "C" fn fts5_sentence_finder_add(p: &mut Fts5SFinder, i_add_1: i32)
    -> i32 {
    if (*p).n_first_alloc == (*p).n_first {
        let n_new: i32 =
            if (*p).n_first_alloc != 0 { (*p).n_first_alloc * 2 } else { 64 };
        let mut a_new: *mut i32 = core::ptr::null_mut();
        a_new =
            unsafe {
                    sqlite3_realloc64((*p).a_first as *mut (),
                        n_new as u64 * core::mem::size_of::<i32>() as u64)
                } as *mut i32;
        if a_new == core::ptr::null_mut() { return 7; }
        (*p).a_first = a_new;
        (*p).n_first_alloc = n_new;
    }
    unsafe {
        *(*p).a_first.offset({
                            let __p = &mut (*p).n_first;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        } as isize) = i_add_1
    };
    return 0;
}

extern "C" fn fts5_sentence_finder_cb(p_context_1: *mut (), tflags: i32,
    p_token_1: *const i8, n_token_1: i32, i_start_off_1: i32,
    i_end_off_1: i32) -> i32 {
    let mut rc: i32 = 0;
    { { let _ = p_token_1; }; { let _ = n_token_1; } };
    { let _ = i_end_off_1; };
    if tflags & 1 == 0 {
        let p: *mut Fts5SFinder = p_context_1 as *mut Fts5SFinder;
        if unsafe { (*p).i_pos } > 0 {
            let mut i: i32 = 0;
            let mut c: i8 = 0 as i8;
            {
                i = i_start_off_1 - 1;
                '__b0: loop {
                    if !(i >= 0) { break '__b0; }
                    '__c0: loop {
                        c =
                            unsafe { *unsafe { (*p).z_doc.offset(i as isize) } } as i8;
                        if c as i32 != ' ' as i32 && c as i32 != '\t' as i32 &&
                                    c as i32 != '\n' as i32 && c as i32 != '\r' as i32 {
                            break '__b0;
                        }
                        break '__c0;
                    }
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                }
            }
            if i != i_start_off_1 - 1 &&
                    (c as i32 == '.' as i32 || c as i32 == ':' as i32) {
                rc =
                    fts5_sentence_finder_add(unsafe { &mut *p },
                        unsafe { (*p).i_pos });
            }
        } else { rc = fts5_sentence_finder_add(unsafe { &mut *p }, 0); }
        {
            let __p = unsafe { &mut (*p).i_pos };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    return rc;
}

extern "C" fn fts5_snippet_score(p_api_1: &Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, n_docsize_1: i32, a_seen_1: *mut u8,
    i_col_1: i32, i_pos_1: i32, n_token_1: i32, pn_score_1: &mut i32,
    pi_pos_1: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut ip: i32 = 0;
    let mut ic: i32 = 0;
    let mut i_off: i32 = 0;
    let mut i_first: i32 = -1;
    let mut n_inst: i32 = 0;
    let mut n_score: i32 = 0;
    let mut i_last: i32 = 0;
    let i_end: Sqlite3Int64 =
        i_pos_1 as Sqlite3Int64 + n_token_1 as Sqlite3Int64;
    rc = unsafe { (*p_api_1).x_inst_count.unwrap()(p_fts_1, &mut n_inst) };
    {
        i = 0;
        '__b1: loop {
            if !(i < n_inst && rc == 0) { break '__b1; }
            '__c1: loop {
                rc =
                    unsafe {
                        (*p_api_1).x_inst.unwrap()(p_fts_1, i, &mut ip, &mut ic,
                            &mut i_off)
                    };
                if rc == 0 && ic == i_col_1 && i_off >= i_pos_1 &&
                        (i_off as Sqlite3Int64) < i_end {
                    n_score +=
                        if unsafe { *a_seen_1.offset(ip as isize) } != 0 {
                            1
                        } else { 1000 };
                    unsafe { *a_seen_1.offset(ip as isize) = 1 as u8 };
                    if i_first < 0 { i_first = i_off; }
                    i_last =
                        i_off +
                            unsafe { (*p_api_1).x_phrase_size.unwrap()(p_fts_1, ip) };
                }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    *pn_score_1 = n_score;
    if !(pi_pos_1).is_null() {
        let mut i_adj: Sqlite3Int64 =
            (i_first - (n_token_1 - (i_last - i_first)) / 2) as Sqlite3Int64;
        if i_adj + n_token_1 as Sqlite3Int64 > n_docsize_1 as i64 {
            i_adj = (n_docsize_1 - n_token_1) as Sqlite3Int64;
        }
        if i_adj < 0 as i64 { i_adj = 0 as Sqlite3Int64; }
        unsafe { *pi_pos_1 = i_adj as i32 };
    }
    return rc;
}

extern "C" fn fts5_c_inst_iter_next(p_iter_1: &mut CInstIter) -> i32 {
    let mut rc: i32 = 0;
    (*p_iter_1).i_start = -1;
    (*p_iter_1).i_end = -1;
    while rc == 0 && (*p_iter_1).i_inst < (*p_iter_1).n_inst {
        let mut ip: i32 = 0;
        let mut ic: i32 = 0;
        let mut io: i32 = 0;
        rc =
            unsafe {
                (unsafe {
                        (*(*p_iter_1).p_api).x_inst.unwrap()
                    })((*p_iter_1).p_fts, (*p_iter_1).i_inst, &mut ip, &mut ic,
                    &mut io)
            };
        if rc == 0 {
            if ic == (*p_iter_1).i_col {
                let i_end: i32 =
                    io - 1 +
                        unsafe {
                            (unsafe {
                                    (*(*p_iter_1).p_api).x_phrase_size.unwrap()
                                })((*p_iter_1).p_fts, ip)
                        };
                if (*p_iter_1).i_start < 0 {
                    (*p_iter_1).i_start = io;
                    (*p_iter_1).i_end = i_end;
                } else if io <= (*p_iter_1).i_end {
                    if i_end > (*p_iter_1).i_end { (*p_iter_1).i_end = i_end; }
                } else { break; }
            }
            {
                let __p = &mut (*p_iter_1).i_inst;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
    return rc;
}

extern "C" fn fts5_c_inst_iter_init(p_api_1: *const Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, i_col_1: i32, p_iter_1: *mut CInstIter)
    -> i32 {
    let mut rc: i32 = 0;
    unsafe {
        memset(p_iter_1 as *mut (), 0,
            core::mem::size_of::<CInstIter>() as u64)
    };
    unsafe { (*p_iter_1).p_api = p_api_1 };
    unsafe { (*p_iter_1).p_fts = p_fts_1 };
    unsafe { (*p_iter_1).i_col = i_col_1 };
    rc =
        unsafe {
            (unsafe {
                    (*p_api_1).x_inst_count.unwrap()
                })(p_fts_1, unsafe { &mut (*p_iter_1).n_inst })
        };
    if rc == 0 { rc = fts5_c_inst_iter_next(unsafe { &mut *p_iter_1 }); }
    return rc;
}

extern "C" fn fts5_highlight_append(p_rc_1: &mut i32,
    p: *mut HighlightContext, z: *const i8, mut n: i32) -> () {
    if *p_rc_1 == 0 && !(z).is_null() {
        if n < 0 { n = unsafe { strlen(z) } as i32; }
        unsafe {
            (*p).z_out =
                unsafe {
                    sqlite3_mprintf(c"%z%.*s".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p).z_out }, n, z)
                }
        };
        if unsafe { (*p).z_out } == core::ptr::null_mut() { *p_rc_1 = 7; }
    }
}

extern "C" fn fts5_highlight_cb(p_context_1: *mut (), tflags: i32,
    p_token_1: *const i8, n_token_1: i32, i_start_off_1: i32,
    i_end_off_1: i32) -> i32 {
    let p: *mut HighlightContext = p_context_1 as *mut HighlightContext;
    let mut rc: i32 = 0;
    let mut i_pos: i32 = 0;
    { { let _ = p_token_1; }; { let _ = n_token_1; } };
    if tflags & 1 != 0 { return 0; }
    i_pos =
        {
            let __p = unsafe { &mut (*p).i_pos };
            let __t = *__p;
            *__p += 1;
            __t
        };
    if unsafe { (*p).i_range_end } >= 0 {
        if i_pos < unsafe { (*p).i_range_start } ||
                i_pos > unsafe { (*p).i_range_end } {
            return 0;
        }
        if unsafe { (*p).i_range_start } != 0 &&
                i_pos == unsafe { (*p).i_range_start } {
            unsafe { (*p).i_off = i_start_off_1 };
        }
    }
    if unsafe { (*p).b_open } != 0 &&
                (i_pos <= unsafe { (*p).iter.i_start } ||
                    unsafe { (*p).iter.i_start } < 0) &&
            i_start_off_1 > unsafe { (*p).i_off } {
        fts5_highlight_append(&mut rc, p, unsafe { (*p).z_close }, -1);
        unsafe { (*p).b_open = 0 };
    }
    if i_pos == unsafe { (*p).iter.i_start } && unsafe { (*p).b_open } == 0 {
        fts5_highlight_append(&mut rc, p,
            unsafe {
                &*unsafe { (*p).z_in.offset(unsafe { (*p).i_off } as isize) }
            }, i_start_off_1 - unsafe { (*p).i_off });
        fts5_highlight_append(&mut rc, p, unsafe { (*p).z_open }, -1);
        unsafe { (*p).i_off = i_start_off_1 };
        unsafe { (*p).b_open = 1 };
    }
    if i_pos == unsafe { (*p).iter.i_end } {
        if unsafe { (*p).b_open } == 0 {
            if !(unsafe { (*p).i_range_end } >= 0) as i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"fts5HighlightCb".as_ptr() as *const i8,
                        c"fts5_aux.c".as_ptr() as *mut i8 as *const i8, 201,
                        c"p->iRangeEnd>=0".as_ptr() as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
            fts5_highlight_append(&mut rc, p, unsafe { (*p).z_open }, -1);
            unsafe { (*p).b_open = 1 };
        }
        fts5_highlight_append(&mut rc, p,
            unsafe {
                &*unsafe { (*p).z_in.offset(unsafe { (*p).i_off } as isize) }
            }, i_end_off_1 - unsafe { (*p).i_off });
        unsafe { (*p).i_off = i_end_off_1 };
        if rc == 0 { rc = fts5_c_inst_iter_next(unsafe { &mut (*p).iter }); }
    }
    if i_pos == unsafe { (*p).i_range_end } {
        if unsafe { (*p).b_open } != 0 {
            if unsafe { (*p).iter.i_start } >= 0 &&
                    i_pos >= unsafe { (*p).iter.i_start } {
                fts5_highlight_append(&mut rc, p,
                    unsafe {
                        &*unsafe {
                                    (*p).z_in.offset(unsafe { (*p).i_off } as isize)
                                }
                    }, i_end_off_1 - unsafe { (*p).i_off });
                unsafe { (*p).i_off = i_end_off_1 };
            }
            fts5_highlight_append(&mut rc, p, unsafe { (*p).z_close }, -1);
            unsafe { (*p).b_open = 0 };
        }
        fts5_highlight_append(&mut rc, p,
            unsafe {
                &*unsafe { (*p).z_in.offset(unsafe { (*p).i_off } as isize) }
            }, i_end_off_1 - unsafe { (*p).i_off });
        unsafe { (*p).i_off = i_end_off_1 };
    }
    return rc;
}

extern "C" fn fts5_snippet_function(p_api_1: *const Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, p_ctx_1: *mut Sqlite3Context, n_val_1: i32,
    ap_val_1: *mut *mut Sqlite3Value) -> () {
    let mut ctx: HighlightContext = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut i_col: i32 = 0;
    let mut z_ellips: *const i8 = core::ptr::null();
    let mut n_token: i64 = 0 as i64;
    let mut n_inst: i32 = 0;
    let mut i: i32 = 0;
    let mut n_phrase: i32 = 0;
    let mut a_seen: *mut u8 = core::ptr::null_mut();
    let mut i_best_col: i32 = 0;
    let mut i_best_start: i32 = 0;
    let mut n_best_score: i32 = 0;
    let mut n_col_size: i32 = 0;
    let mut s_finder: Fts5SFinder = unsafe { core::mem::zeroed() };
    let mut n_col: i32 = 0;
    if n_val_1 != 5 {
        let z_err: *const i8 =
            c"wrong number of arguments to function snippet()".as_ptr() as
                    *mut i8 as *const i8;
        unsafe { sqlite3_result_error(p_ctx_1, z_err, -1) };
        return;
    }
    n_col =
        unsafe { (unsafe { (*p_api_1).x_column_count.unwrap() })(p_fts_1) };
    unsafe {
        memset(&raw mut ctx as *mut (), 0,
            core::mem::size_of::<HighlightContext>() as u64)
    };
    i_col =
        unsafe { sqlite3_value_int(unsafe { *ap_val_1.offset(0 as isize) }) };
    ctx.z_open = fts5_value_to_text(unsafe { *ap_val_1.offset(1 as isize) });
    ctx.z_close = fts5_value_to_text(unsafe { *ap_val_1.offset(2 as isize) });
    ctx.i_range_end = -1;
    z_ellips = fts5_value_to_text(unsafe { *ap_val_1.offset(3 as isize) });
    n_token =
        if if unsafe {
                                    sqlite3_value_int64(unsafe { *ap_val_1.offset(4 as isize) })
                                } > 0 as i64 {
                            unsafe {
                                sqlite3_value_int64(unsafe { *ap_val_1.offset(4 as isize) })
                            }
                        } else { 0 as Sqlite3Int64 } < 64 as i64 {
                    if unsafe {
                                sqlite3_value_int64(unsafe { *ap_val_1.offset(4 as isize) })
                            } > 0 as i64 {
                        unsafe {
                            sqlite3_value_int64(unsafe { *ap_val_1.offset(4 as isize) })
                        }
                    } else { 0 as Sqlite3Int64 }
                } else { 64 as Sqlite3Int64 } as i32 as i64;
    i_best_col = if i_col >= 0 { i_col } else { 0 };
    n_phrase =
        unsafe { (unsafe { (*p_api_1).x_phrase_count.unwrap() })(p_fts_1) };
    a_seen =
        unsafe { sqlite3_malloc64(n_phrase as Sqlite3Uint64) } as *mut u8;
    if a_seen == core::ptr::null_mut() { rc = 7; }
    if rc == 0 {
        rc =
            unsafe {
                (unsafe {
                        (*p_api_1).x_inst_count.unwrap()
                    })(p_fts_1, &mut n_inst)
            };
    }
    unsafe {
        memset(&raw mut s_finder as *mut (), 0,
            core::mem::size_of::<Fts5SFinder>() as u64)
    };
    {
        i = 0;
        '__b3: loop {
            if !(i < n_col) { break '__b3; }
            '__c3: loop {
                if i_col < 0 || i_col == i {
                    let mut p_loc: *const i8 = core::ptr::null();
                    let mut n_loc: i32 = 0;
                    let mut n_doc: i32 = 0;
                    let mut n_docsize: i32 = 0;
                    let mut ii: i32 = 0;
                    s_finder.i_pos = 0;
                    s_finder.n_first = 0;
                    rc =
                        unsafe {
                            (unsafe {
                                    (*p_api_1).x_column_text.unwrap()
                                })(p_fts_1, i, &mut s_finder.z_doc, &mut n_doc)
                        };
                    if rc != 0 { break '__b3; }
                    rc =
                        unsafe {
                            (unsafe {
                                    (*p_api_1).x_column_locale.unwrap()
                                })(p_fts_1, i, &mut p_loc, &mut n_loc)
                        };
                    if rc != 0 { break '__b3; }
                    rc =
                        unsafe {
                            (unsafe {
                                    (*p_api_1).x_tokenize_v2.unwrap()
                                })(p_fts_1, s_finder.z_doc, n_doc, p_loc, n_loc,
                                &raw mut s_finder as *mut (), fts5_sentence_finder_cb)
                        };
                    if rc != 0 { break '__b3; }
                    rc =
                        unsafe {
                            (unsafe {
                                    (*p_api_1).x_column_size.unwrap()
                                })(p_fts_1, i, &mut n_docsize)
                        };
                    if rc != 0 { break '__b3; }
                    {
                        ii = 0;
                        '__b4: loop {
                            if !(rc == 0 && ii < n_inst) { break '__b4; }
                            '__c4: loop {
                                let mut ip: i32 = 0;
                                let mut ic: i32 = 0;
                                let mut io: i32 = 0;
                                let mut i_adj: i32 = 0;
                                let mut n_score: i32 = 0;
                                let mut jj: i32 = 0;
                                rc =
                                    unsafe {
                                        (unsafe {
                                                (*p_api_1).x_inst.unwrap()
                                            })(p_fts_1, ii, &mut ip, &mut ic, &mut io)
                                    };
                                if ic != i { break '__c4; }
                                if io > n_docsize { rc = 11 | 1 << 8; }
                                if rc != 0 { break '__c4; }
                                unsafe { memset(a_seen as *mut (), 0, n_phrase as u64) };
                                rc =
                                    fts5_snippet_score(unsafe { &*p_api_1 }, p_fts_1, n_docsize,
                                        a_seen, i, io, n_token as i32, &mut n_score, &mut i_adj);
                                if rc == 0 && n_score > n_best_score {
                                    n_best_score = n_score;
                                    i_best_col = i;
                                    i_best_start = i_adj;
                                    n_col_size = n_docsize;
                                }
                                if rc == 0 && s_finder.n_first != 0 &&
                                        n_docsize as i64 > n_token {
                                    {
                                        jj = 0;
                                        '__b5: loop {
                                            if !(jj < s_finder.n_first - 1) { break '__b5; }
                                            '__c5: loop {
                                                if unsafe { *s_finder.a_first.offset((jj + 1) as isize) } >
                                                        io {
                                                    break '__b5;
                                                }
                                                break '__c5;
                                            }
                                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    if unsafe { *s_finder.a_first.offset(jj as isize) } < io {
                                        unsafe { memset(a_seen as *mut (), 0, n_phrase as u64) };
                                        rc =
                                            fts5_snippet_score(unsafe { &*p_api_1 }, p_fts_1, n_docsize,
                                                a_seen, i, unsafe { *s_finder.a_first.offset(jj as isize) },
                                                n_token as i32, &mut n_score, core::ptr::null_mut());
                                        n_score +=
                                            if unsafe { *s_finder.a_first.offset(jj as isize) } == 0 {
                                                120
                                            } else { 100 };
                                        if rc == 0 && n_score > n_best_score {
                                            n_best_score = n_score;
                                            i_best_col = i;
                                            i_best_start =
                                                unsafe { *s_finder.a_first.offset(jj as isize) };
                                            n_col_size = n_docsize;
                                        }
                                    }
                                }
                                break '__c4;
                            }
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 {
        rc =
            unsafe {
                (unsafe {
                        (*p_api_1).x_column_text.unwrap()
                    })(p_fts_1, i_best_col, &mut ctx.z_in, &mut ctx.n_in)
            };
    }
    if rc == 0 && n_col_size == 0 {
        rc =
            unsafe {
                (unsafe {
                        (*p_api_1).x_column_size.unwrap()
                    })(p_fts_1, i_best_col, &mut n_col_size)
            };
    }
    if !(ctx.z_in).is_null() {
        let mut p_loc_1: *const i8 = core::ptr::null();
        let mut n_loc_1: i32 = 0;
        if rc == 0 {
            rc =
                fts5_c_inst_iter_init(p_api_1, p_fts_1, i_best_col,
                    &mut ctx.iter);
        }
        ctx.i_range_start = i_best_start;
        ctx.i_range_end = (i_best_start as i64 + n_token - 1 as i64) as i32;
        if i_best_start > 0 {
            fts5_highlight_append(&mut rc, &mut ctx, z_ellips, -1);
        }
        while ctx.iter.i_start >= 0 && ctx.iter.i_start < i_best_start &&
                rc == 0 {
            rc = fts5_c_inst_iter_next(&mut ctx.iter);
        }
        if rc == 0 {
            rc =
                unsafe {
                    (unsafe {
                            (*p_api_1).x_column_locale.unwrap()
                        })(p_fts_1, i_best_col, &mut p_loc_1, &mut n_loc_1)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    (unsafe {
                            (*p_api_1).x_tokenize_v2.unwrap()
                        })(p_fts_1, ctx.z_in, ctx.n_in, p_loc_1, n_loc_1,
                        &raw mut ctx as *mut (), fts5_highlight_cb)
                };
        }
        if ctx.b_open != 0 {
            fts5_highlight_append(&mut rc, &mut ctx, ctx.z_close, -1);
        }
        if ctx.i_range_end >= n_col_size - 1 {
            fts5_highlight_append(&mut rc, &mut ctx,
                unsafe { &*ctx.z_in.offset(ctx.i_off as isize) },
                ctx.n_in - ctx.i_off);
        } else { fts5_highlight_append(&mut rc, &mut ctx, z_ellips, -1); }
    }
    if rc == 0 {
        unsafe {
            sqlite3_result_text(p_ctx_1, ctx.z_out as *const i8, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    } else { unsafe { sqlite3_result_error_code(p_ctx_1, rc) }; }
    unsafe { sqlite3_free(ctx.z_out as *mut ()) };
    unsafe { sqlite3_free(a_seen as *mut ()) };
    unsafe { sqlite3_free(s_finder.a_first as *mut ()) };
}

extern "C" fn fts5_highlight_function(p_api_1: *const Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, p_ctx_1: *mut Sqlite3Context, n_val_1: i32,
    ap_val_1: *mut *mut Sqlite3Value) -> () {
    let mut ctx: HighlightContext = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut i_col: i32 = 0;
    if n_val_1 != 3 {
        let z_err: *const i8 =
            c"wrong number of arguments to function highlight()".as_ptr() as
                    *mut i8 as *const i8;
        unsafe { sqlite3_result_error(p_ctx_1, z_err, -1) };
        return;
    }
    i_col =
        unsafe { sqlite3_value_int(unsafe { *ap_val_1.offset(0 as isize) }) };
    unsafe {
        memset(&raw mut ctx as *mut (), 0,
            core::mem::size_of::<HighlightContext>() as u64)
    };
    ctx.z_open =
        unsafe { sqlite3_value_text(unsafe { *ap_val_1.offset(1 as isize) }) }
            as *const i8;
    ctx.z_close =
        unsafe { sqlite3_value_text(unsafe { *ap_val_1.offset(2 as isize) }) }
            as *const i8;
    ctx.i_range_end = -1;
    rc =
        unsafe {
            (unsafe {
                    (*p_api_1).x_column_text.unwrap()
                })(p_fts_1, i_col, &mut ctx.z_in, &mut ctx.n_in)
        };
    if rc == 25 {
        unsafe {
            sqlite3_result_text(p_ctx_1, c"".as_ptr() as *mut i8 as *const i8,
                -1, None)
        };
        rc = 0;
    } else if !(ctx.z_in).is_null() {
        let mut p_loc: *const i8 = core::ptr::null();
        let mut n_loc: i32 = 0;
        if rc == 0 {
            rc =
                fts5_c_inst_iter_init(p_api_1, p_fts_1, i_col, &mut ctx.iter);
        }
        if rc == 0 {
            rc =
                unsafe {
                    (unsafe {
                            (*p_api_1).x_column_locale.unwrap()
                        })(p_fts_1, i_col, &mut p_loc, &mut n_loc)
                };
        }
        if rc == 0 {
            rc =
                unsafe {
                    (unsafe {
                            (*p_api_1).x_tokenize_v2.unwrap()
                        })(p_fts_1, ctx.z_in, ctx.n_in, p_loc, n_loc,
                        &raw mut ctx as *mut (), fts5_highlight_cb)
                };
        }
        if ctx.b_open != 0 {
            fts5_highlight_append(&mut rc, &mut ctx, ctx.z_close, -1);
        }
        fts5_highlight_append(&mut rc, &mut ctx,
            unsafe { &*ctx.z_in.offset(ctx.i_off as isize) },
            ctx.n_in - ctx.i_off);
        if rc == 0 {
            unsafe {
                sqlite3_result_text(p_ctx_1, ctx.z_out as *const i8, -1,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
        }
        unsafe { sqlite3_free(ctx.z_out as *mut ()) };
    }
    if rc != 0 { unsafe { sqlite3_result_error_code(p_ctx_1, rc) }; }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Bm25Data {
    n_phrase: i32,
    avgdl: f64,
    a_idf: *mut f64,
    a_freq: *mut f64,
}

extern "C" fn fts5_count_cb(p_api_1: *const Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, p_user_data_1: *mut ()) -> i32 {
    let pn: *mut Sqlite3Int64 = p_user_data_1 as *mut Sqlite3Int64;
    { { let _ = p_api_1; }; { let _ = p_fts_1; } };
    { let __p = unsafe { &mut *pn }; let __t = *__p; *__p += 1; __t };
    return 0;
}

extern "C" fn fts5_bm25_get_data(p_api_1: &Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, pp_data_1: &mut *mut Fts5Bm25Data) -> i32 {
    let mut rc: i32 = 0;
    let mut p: *mut Fts5Bm25Data = core::ptr::null_mut();
    p =
        unsafe { (*p_api_1).x_get_auxdata.unwrap()(p_fts_1, 0) } as
            *mut Fts5Bm25Data;
    if p == core::ptr::null_mut() {
        let mut n_phrase: i32 = 0;
        let mut n_row: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut n_token: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut n_byte: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i: i32 = 0;
        n_phrase = unsafe { (*p_api_1).x_phrase_count.unwrap()(p_fts_1) };
        n_byte =
            (core::mem::size_of::<Fts5Bm25Data>() as u64 +
                    (n_phrase * 2) as u64 * core::mem::size_of::<f64>() as u64)
                as Sqlite3Int64;
        p =
            unsafe { sqlite3_malloc64(n_byte as Sqlite3Uint64) } as
                *mut Fts5Bm25Data;
        if p == core::ptr::null_mut() {
            rc = 7;
        } else {
            unsafe { memset(p as *mut (), 0, n_byte as u64) };
            unsafe { (*p).n_phrase = n_phrase };
            unsafe {
                (*p).a_idf =
                    unsafe { &raw mut *p.offset(1 as isize) } as *mut f64
            };
            unsafe {
                (*p).a_freq =
                    unsafe { unsafe { (*p).a_idf.offset(n_phrase as isize) } }
            };
        }
        if rc == 0 {
            rc =
                unsafe {
                    (*p_api_1).x_row_count.unwrap()(p_fts_1, &mut n_row)
                };
        }
        if !(rc != 0 || n_row > 0 as i64) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fts5Bm25GetData".as_ptr() as *const i8,
                    c"fts5_aux.c".as_ptr() as *mut i8 as *const i8, 651,
                    c"rc!=SQLITE_OK || nRow>0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if rc == 0 {
            rc =
                unsafe {
                    (*p_api_1).x_column_total_size.unwrap()(p_fts_1, -1,
                        &mut n_token)
                };
        }
        if rc == 0 { unsafe { (*p).avgdl = n_token as f64 / n_row as f64 }; }
        {
            i = 0;
            '__b7: loop {
                if !(rc == 0 && i < n_phrase) { break '__b7; }
                '__c7: loop {
                    let mut n_hit: Sqlite3Int64 = 0 as Sqlite3Int64;
                    rc =
                        unsafe {
                            (*p_api_1).x_query_phrase.unwrap()(p_fts_1, i,
                                &raw mut n_hit as *mut (), fts5_count_cb)
                        };
                    if rc == 0 {
                        let mut idf: f64 =
                            unsafe {
                                log(((n_row - n_hit) as f64 + 0.5) / (n_hit as f64 + 0.5))
                            };
                        if idf <= 0.0 { idf = 1e-6; }
                        unsafe { *unsafe { (*p).a_idf.offset(i as isize) } = idf };
                    }
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc != 0 {
            unsafe { sqlite3_free(p as *mut ()) };
        } else {
            rc =
                unsafe {
                    (*p_api_1).x_set_auxdata.unwrap()(p_fts_1, p as *mut (),
                        sqlite3_free)
                };
        }
        if rc != 0 { p = core::ptr::null_mut(); }
    }
    *pp_data_1 = p;
    return rc;
}

extern "C" fn fts5_bm25_function(p_api_1: *const Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, p_ctx_1: *mut Sqlite3Context, n_val_1: i32,
    ap_val_1: *mut *mut Sqlite3Value) -> () {
    let k1: f64 = 1.2 as f64;
    let b: f64 = 0.75 as f64;
    let mut rc: i32 = 0;
    let mut score: f64 = 0.0;
    let mut p_data: *mut Fts5Bm25Data = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut n_inst: i32 = 0;
    let mut d: f64 = 0.0;
    let mut a_freq: *mut f64 = core::ptr::null_mut();
    rc = fts5_bm25_get_data(unsafe { &*p_api_1 }, p_fts_1, &mut p_data);
    if rc == 0 {
        a_freq = unsafe { (*p_data).a_freq };
        unsafe {
            memset(a_freq as *mut (), 0,
                core::mem::size_of::<f64>() as u64 *
                    unsafe { (*p_data).n_phrase } as u64)
        };
        rc =
            unsafe {
                (unsafe {
                        (*p_api_1).x_inst_count.unwrap()
                    })(p_fts_1, &mut n_inst)
            };
    }
    {
        i = 0;
        '__b8: loop {
            if !(rc == 0 && i < n_inst) { break '__b8; }
            '__c8: loop {
                let mut ip: i32 = 0;
                let mut ic: i32 = 0;
                let mut io: i32 = 0;
                rc =
                    unsafe {
                        (unsafe {
                                (*p_api_1).x_inst.unwrap()
                            })(p_fts_1, i, &mut ip, &mut ic, &mut io)
                    };
                if rc == 0 {
                    let w: f64 =
                        if n_val_1 > ic {
                            unsafe {
                                sqlite3_value_double(unsafe {
                                        *ap_val_1.offset(ic as isize)
                                    })
                            }
                        } else { 1.0 };
                    unsafe { *a_freq.offset(ip as isize) += w };
                }
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 {
        let mut n_tok: i32 = 0;
        rc =
            unsafe {
                (unsafe {
                        (*p_api_1).x_column_size.unwrap()
                    })(p_fts_1, -1, &mut n_tok)
            };
        d = n_tok as f64;
    }
    if rc == 0 {
        {
            i = 0;
            '__b9: loop {
                if !(i < unsafe { (*p_data).n_phrase }) { break '__b9; }
                '__c9: loop {
                    score +=
                        unsafe { *unsafe { (*p_data).a_idf.offset(i as isize) } } *
                            (unsafe { *a_freq.offset(i as isize) } * (k1 + 1.0) /
                                (unsafe { *a_freq.offset(i as isize) } +
                                    k1 *
                                        (1 as f64 - b as f64 +
                                            b * d / unsafe { (*p_data).avgdl })));
                    break '__c9;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_result_double(p_ctx_1, -1.0 * score) };
    } else { unsafe { sqlite3_result_error_code(p_ctx_1, rc) }; }
}

extern "C" fn fts5_get_locale_function(p_api_1: *const Fts5ExtensionApi,
    p_fts_1: *mut Fts5Context, p_ctx_1: *mut Sqlite3Context, n_val_1: i32,
    ap_val_1: *mut *mut Sqlite3Value) -> () {
    let mut i_col: i32 = 0;
    let mut e_type: i32 = 0;
    let mut rc: i32 = 0;
    let mut z_locale: *const i8 = core::ptr::null();
    let mut n_locale: i32 = 0;
    if !(unsafe { (*p_api_1).i_version } as i32 >= 4) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fts5GetLocaleFunction".as_ptr() as *const i8,
                c"fts5_aux.c".as_ptr() as *mut i8 as *const i8, 766,
                c"pApi->iVersion>=4".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if n_val_1 != 1 {
        let z: *const i8 =
            c"wrong number of arguments to function fts5_get_locale()".as_ptr()
                    as *mut i8 as *const i8;
        unsafe { sqlite3_result_error(p_ctx_1, z, -1) };
        return;
    }
    e_type =
        unsafe {
            sqlite3_value_numeric_type(unsafe {
                    *ap_val_1.offset(0 as isize)
                })
        };
    if e_type != 1 {
        let z: *const i8 =
            c"non-integer argument passed to function fts5_get_locale()".as_ptr()
                    as *mut i8 as *const i8;
        unsafe { sqlite3_result_error(p_ctx_1, z, -1) };
        return;
    }
    i_col =
        unsafe { sqlite3_value_int(unsafe { *ap_val_1.offset(0 as isize) }) };
    if i_col < 0 ||
            i_col >=
                unsafe {
                    (unsafe { (*p_api_1).x_column_count.unwrap() })(p_fts_1)
                } {
        unsafe { sqlite3_result_error_code(p_ctx_1, 25) };
        return;
    }
    rc =
        unsafe {
            (unsafe {
                    (*p_api_1).x_column_locale.unwrap()
                })(p_fts_1, i_col, &mut z_locale, &mut n_locale)
        };
    if rc != 0 { unsafe { sqlite3_result_error_code(p_ctx_1, rc) }; return; }
    unsafe {
        sqlite3_result_text(p_ctx_1, z_locale, n_locale,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fts5_aux_init(p_api: *mut Fts5Api) -> i32 {
    let a_builtin: [BuiltinN7Builtin; 4] =
        [BuiltinN7Builtin {
                    z_func: c"snippet".as_ptr() as *const i8,
                    p_user_data: core::ptr::null_mut(),
                    x_func: Some(fts5_snippet_function),
                    x_destroy: None,
                },
                BuiltinN7Builtin {
                    z_func: c"highlight".as_ptr() as *const i8,
                    p_user_data: core::ptr::null_mut(),
                    x_func: Some(fts5_highlight_function),
                    x_destroy: None,
                },
                BuiltinN7Builtin {
                    z_func: c"bm25".as_ptr() as *const i8,
                    p_user_data: core::ptr::null_mut(),
                    x_func: Some(fts5_bm25_function),
                    x_destroy: None,
                },
                BuiltinN7Builtin {
                    z_func: c"fts5_get_locale".as_ptr() as *const i8,
                    p_user_data: core::ptr::null_mut(),
                    x_func: Some(fts5_get_locale_function),
                    x_destroy: None,
                }];
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b10: loop {
            if !(rc == 0 &&
                            i <
                                (core::mem::size_of::<[BuiltinN7Builtin; 4]>() as u64 /
                                        core::mem::size_of::<BuiltinN7Builtin>() as u64) as i32) {
                break '__b10;
            }
            '__c10: loop {
                rc =
                    unsafe {
                        (unsafe {
                                (*p_api).x_create_function.unwrap()
                            })(p_api, a_builtin[i as usize].z_func,
                            a_builtin[i as usize].p_user_data,
                            a_builtin[i as
                                            usize].x_func.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*const Fts5ExtensionApi,
                                                *mut Fts5Context, *mut Sqlite3Context, i32,
                                                *mut *mut Sqlite3Value) -> ()>(0 as *const ())
                                }),
                            a_builtin[i as
                                            usize].x_destroy.unwrap_or(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ()) -> ()>(0 as *const ())
                                }))
                    };
                break '__c10;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct BuiltinN7Builtin {
    z_func: *const i8,
    p_user_data: *mut (),
    x_func: Option<unsafe extern "C" fn(*const Fts5ExtensionApi,
        *mut Fts5Context, *mut Sqlite3Context, i32, *mut *mut Sqlite3Value)
        -> ()>,
    x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>,
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
    fn sqlite3_fts5_expr_new(p_config_1: *mut Fts5Config,
    b_phrase_to_and_1: i32, i_col_1: i32, z_expr_1: *const i8,
    pp_new_1: *mut *mut Fts5Expr, pz_err_1: *mut *mut i8)
    -> i32;
    fn sqlite3_fts5_expr_pattern(p_config_1: *mut Fts5Config, b_glob_1: i32,
    i_col_1: i32, z_text_1: *const i8, pp: *mut *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_first(_: *mut Fts5Expr, p_idx_1: *mut Fts5Index,
    i_min_1: i64, _: i64, b_desc_1: i32)
    -> i32;
    fn sqlite3_fts5_expr_next(_: *mut Fts5Expr, i_max_1: i64)
    -> i32;
    fn sqlite3_fts5_expr_eof(_: *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_rowid(_: *mut Fts5Expr)
    -> i64;
    fn sqlite3_fts5_expr_free(_: *mut Fts5Expr)
    -> ();
    fn sqlite3_fts5_expr_and(pp1: *mut *mut Fts5Expr, p2: *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_init(_: *mut Fts5Global, _: *mut Sqlite3)
    -> i32;
    fn sqlite3_fts5_expr_phrase_count(_: *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_phrase_size(_: *mut Fts5Expr, i_phrase_1: i32)
    -> i32;
    fn sqlite3_fts5_expr_poslist(_: *mut Fts5Expr, _: i32, _: *mut *const u8)
    -> i32;
    fn sqlite3_fts5_expr_clear_poslists(_: *mut Fts5Expr, _: i32)
    -> *mut Fts5PoslistPopulator;
    fn sqlite3_fts5_expr_populate_poslists(_: *mut Fts5Config,
    _: *mut Fts5Expr, _: *mut Fts5PoslistPopulator, _: i32, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_fts5_expr_check_poslists(_: *mut Fts5Expr, _: i64)
    -> ();
    fn sqlite3_fts5_expr_clone_phrase(_: *mut Fts5Expr, _: i32,
    _: *mut *mut Fts5Expr)
    -> i32;
    fn sqlite3_fts5_expr_phrase_collist(_: *mut Fts5Expr, _: i32,
    _: *mut *const u8, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_expr_query_token(_: *mut Fts5Expr, _: i32, _: i32,
    _: *mut *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_expr_inst_token(_: *mut Fts5Expr, _: i64, _: i32, _: i32,
    _: i32, _: i32, _: *mut *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_fts5_expr_clear_tokens(_: *mut Fts5Expr)
    -> ();
    fn sqlite3_fts5_parse_error(p_parse_1: *mut Fts5Parse, z_fmt_1: *const i8,
    ...)
    -> ();
    fn sqlite3_fts5_parse_node(p_parse_1: *mut Fts5Parse, e_type_1: i32,
    p_left_1: *mut Fts5ExprNode, p_right_1: *mut Fts5ExprNode,
    p_near_1: *mut Fts5ExprNearset)
    -> *mut Fts5ExprNode;
    fn sqlite3_fts5_parse_implicit_and(p_parse_1: *mut Fts5Parse,
    p_left_1: *mut Fts5ExprNode, p_right_1: *mut Fts5ExprNode)
    -> *mut Fts5ExprNode;
    fn sqlite3_fts5_parse_term(p_parse_1: *mut Fts5Parse,
    p_phrase_1: *mut Fts5ExprPhrase, p_token_1: *mut Fts5Token,
    b_prefix_1: i32)
    -> *mut Fts5ExprPhrase;
    fn sqlite3_fts5_parse_set_caret(_: *mut Fts5ExprPhrase)
    -> ();
    fn sqlite3_fts5_parse_nearset(_: *mut Fts5Parse, _: *mut Fts5ExprNearset,
    _: *mut Fts5ExprPhrase)
    -> *mut Fts5ExprNearset;
    fn sqlite3_fts5_parse_colset(_: *mut Fts5Parse, _: *mut Fts5Colset,
    _: *mut Fts5Token)
    -> *mut Fts5Colset;
    fn sqlite3_fts5_parse_phrase_free(_: *mut Fts5ExprPhrase)
    -> ();
    fn sqlite3_fts5_parse_nearset_free(_: *mut Fts5ExprNearset)
    -> ();
    fn sqlite3_fts5_parse_node_free(_: *mut Fts5ExprNode)
    -> ();
    fn sqlite3_fts5_parse_set_distance(_: *mut Fts5Parse,
    _: *mut Fts5ExprNearset, _: *mut Fts5Token)
    -> ();
    fn sqlite3_fts5_parse_set_colset(_: *mut Fts5Parse, _: *mut Fts5ExprNode,
    _: *mut Fts5Colset)
    -> ();
    fn sqlite3_fts5_parse_colset_invert(_: *mut Fts5Parse, _: *mut Fts5Colset)
    -> *mut Fts5Colset;
    fn sqlite3_fts5_parse_finished(p_parse_1: *mut Fts5Parse,
    p: *mut Fts5ExprNode)
    -> ();
    fn sqlite3_fts5_parse_near(p_parse_1: *mut Fts5Parse, _: *mut Fts5Token)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn log(_: f64)
    -> f64;
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
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
