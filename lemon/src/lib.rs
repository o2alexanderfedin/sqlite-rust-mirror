#![feature(c_variadic)]

type DarwinSizeT = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct State {
    bp: *mut Config,
    cfp: *mut Config,
    statenum: i32,
    ap: *mut Action,
    n_tkn_act: i32,
    n_nt_act: i32,
    i_tkn_ofst: i32,
    i_nt_ofst: i32,
    i_dflt_reduce: i32,
    p_dflt_reduce: *mut Rule,
    auto_reduce: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Config {
    rp: *mut Rule,
    dot: i32,
    fws: *mut i8,
    fplp: *mut Plink,
    bplp: *mut Plink,
    status: u32,
    next: *mut Config,
    bp: *mut Config,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Rule {
    lhs: *mut Symbol,
    lhsalias: *const i8,
    lhs_start: i32,
    ruleline: i32,
    nrhs: i32,
    rhs: *mut *mut Symbol,
    rhsalias: *mut *const i8,
    line: i32,
    code: *const i8,
    code_prefix: *const i8,
    code_suffix: *const i8,
    precsym: *mut Symbol,
    index: i32,
    i_rule: i32,
    no_code: Boolean,
    code_emitted: Boolean,
    can_reduce: Boolean,
    does_reduce: Boolean,
    never_reduce: Boolean,
    nextlhs: *mut Rule,
    next: *mut Rule,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Symbol {
    name: *const i8,
    index: i32,
    type_: u32,
    fallback: *mut Symbol,
    prec: i32,
    assoc: u32,
    firstset: *mut i8,
    lambda: Boolean,
    use_cnt: i32,
    destructor: *mut i8,
    dest_lineno: i32,
    datatype: *mut i8,
    dtnum: i32,
    b_content: i32,
    nsubsym: i32,
    subsym: *mut *mut Symbol,
}

const TERMINAL: u32 = 0;

const NONTERMINAL: u32 = 1;

const MULTITERMINAL: u32 = 2;

const LEFT: u32 = 0;

const RIGHT: u32 = 1;

const NONE: u32 = 2;

const UNK: u32 = 3;

const LEMON_FALSE: u32 = 0;

const LEMON_TRUE: u32 = 1;

type Boolean = u32;

#[repr(C)]
#[derive(Copy, Clone)]
struct Plink {
    cfp: *mut Config,
    next: *mut Plink,
}

const COMPLETE: u32 = 0;

const INCOMPLETE: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
struct Action {
    sp: *mut Symbol,
    type_: u32,
    x: ActionU0,
    sp_opt: *mut Symbol,
    next: *mut Action,
    collide: *mut Action,
}

const REDUCE: u32 = 2;

const SHIFTREDUCE: u32 = 10;

const SHIFT: u32 = 0;

const ACCEPT: u32 = 1;

const ERROR: u32 = 3;

const SSCONFLICT: u32 = 4;

const SRCONFLICT: u32 = 5;

const RRCONFLICT: u32 = 6;

const SH_RESOLVED: u32 = 7;

const RD_RESOLVED: u32 = 8;

const NOT_USED: u32 = 9;

#[repr(C)]
#[derive(Copy, Clone)]
union ActionU0 {
    rp: *mut Rule,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Lemon {
    sorted: *mut *mut State,
    rule: *mut Rule,
    start_rule: *mut Rule,
    nstate: i32,
    nxstate: i32,
    nrule: i32,
    nrule_with_action: i32,
    nsymbol: i32,
    nterminal: i32,
    min_shift_reduce: i32,
    err_action: i32,
    acc_action: i32,
    no_action: i32,
    min_reduce: i32,
    max_action: i32,
    symbols: *mut *mut Symbol,
    errorcnt: i32,
    errsym: *mut Symbol,
    wildcard: *mut Symbol,
    name: *mut i8,
    arg: *mut i8,
    ctx: *mut i8,
    tokentype: *mut i8,
    vartype: *mut i8,
    start: *mut i8,
    stacksize: *mut i8,
    include: *mut i8,
    error: *mut i8,
    overflow: *mut i8,
    failure: *mut i8,
    accept: *mut i8,
    extracode: *mut i8,
    tokendest: *mut i8,
    vardest: *mut i8,
    filename: *mut i8,
    outname: *mut i8,
    tokenprefix: *mut i8,
    stack_size_limit: *mut i8,
    realloc_func: *mut i8,
    free_func: *mut i8,
    nconflict: i32,
    nactiontab: i32,
    nlookaheadtab: i32,
    tablesize: i32,
    basisflag: i32,
    print_preprocessed: i32,
    has_fallback: i32,
    nolinenosflag: i32,
    argc: i32,
    argv: *mut *mut i8,
}

static mut basis: *mut Config = core::ptr::null_mut();

extern "C" fn merge(mut a: *mut i8, mut b: *mut i8,
    cmp: Option<unsafe extern "C" fn(*const i8, *const i8) -> i32>,
    offset: i32) -> *mut i8 {
    let mut ptr: *mut i8 = core::ptr::null_mut();
    let mut head: *mut i8 = core::ptr::null_mut();
    if a == core::ptr::null_mut() {
        head = b;
    } else if b == core::ptr::null_mut() {
        head = a;
    } else {
        if unsafe { cmp.unwrap()(a as *const i8, b as *const i8) } <= 0 {
            ptr = a;
            a =
                unsafe {
                    *(unsafe { (a as *mut i8).offset(offset as isize) } as
                            *mut *mut i8)
                };
        } else {
            ptr = b;
            b =
                unsafe {
                    *(unsafe { (b as *mut i8).offset(offset as isize) } as
                            *mut *mut i8)
                };
        }
        head = ptr;
        while !(a).is_null() && !(b).is_null() {
            if unsafe { cmp.unwrap()(a as *const i8, b as *const i8) } <= 0 {
                unsafe {
                    *(unsafe { (ptr as *mut i8).offset(offset as isize) } as
                                *mut *mut i8) = a
                };
                ptr = a;
                a =
                    unsafe {
                        *(unsafe { (a as *mut i8).offset(offset as isize) } as
                                *mut *mut i8)
                    };
            } else {
                unsafe {
                    *(unsafe { (ptr as *mut i8).offset(offset as isize) } as
                                *mut *mut i8) = b
                };
                ptr = b;
                b =
                    unsafe {
                        *(unsafe { (b as *mut i8).offset(offset as isize) } as
                                *mut *mut i8)
                    };
            }
        }
        if !(a).is_null() {
            unsafe {
                *(unsafe { (ptr as *mut i8).offset(offset as isize) } as
                            *mut *mut i8) = a
            };
        } else {
            unsafe {
                *(unsafe { (ptr as *mut i8).offset(offset as isize) } as
                            *mut *mut i8) = b
            };
        }
    }
    return head;
}

extern "C" fn msort(mut list: *mut i8, next: *mut *mut i8,
    cmp: Option<unsafe extern "C" fn(*const i8, *const i8) -> i32>)
    -> *mut i8 {
    let mut offset: u64 = 0 as u64;
    let mut ep: *mut i8 = core::ptr::null_mut();
    let mut set: [*mut i8; 30] = [core::ptr::null_mut(); 30];
    let mut i: i32 = 0;
    offset =
        unsafe { (next as *mut i8).offset_from(list as *mut i8) } as i64 as
            u64;
    {
        i = 0;
        '__b1: loop {
            if !(i < 30) { break '__b1; }
            '__c1: loop {
                set[i as usize] = core::ptr::null_mut();
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    while !(list).is_null() {
        ep = list;
        list =
            unsafe {
                *(unsafe { (list as *mut i8).add(offset as usize) } as
                        *mut *mut i8)
            };
        unsafe {
            *(unsafe { (ep as *mut i8).add(offset as usize) } as *mut *mut i8)
                = core::ptr::null_mut()
        };
        {
            i = 0;
            '__b3: loop {
                if !(i < 30 - 1 && set[i as usize] != core::ptr::null_mut()) {
                    break '__b3;
                }
                '__c3: loop {
                    ep = merge(set[i as usize], ep, cmp, offset as i32);
                    set[i as usize] = core::ptr::null_mut();
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        set[i as usize] = merge(set[i as usize], ep, cmp, offset as i32);
    }
    ep = core::ptr::null_mut();
    {
        i = 0;
        '__b4: loop {
            if !(i < 30) { break '__b4; }
            '__c4: loop {
                if !(set[i as usize]).is_null() {
                    ep = merge(set[i as usize], ep, cmp, offset as i32);
                }
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return ep;
}

static mut current: *mut Config = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn configcmp(_a: *const i8, _b: *const i8) -> i32 {
    unsafe {
        let a: *const Config = _a as *mut Config as *const Config;
        let b: *const Config = _b as *mut Config as *const Config;
        let mut x: i32 = 0;
        x =
            unsafe { (*unsafe { (*a).rp }).index } -
                unsafe { (*unsafe { (*b).rp }).index };
        if x == 0 { x = unsafe { (*a).dot } - unsafe { (*b).dot } as i32; }
        return x;
    }
}

static mut basisend: *mut *mut Config = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn configlist_sortbasis() -> () {
    unsafe {
        basis =
            msort(current as *mut i8,
                    unsafe { &raw mut (*current).bp } as *mut *mut i8,
                    Some(configcmp)) as *mut Config;
        basisend = core::ptr::null_mut();
        return;
    }
}

static mut show_precedence_conflict: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
struct MemChunk {
    p_next: *mut MemChunk,
    sz: u64,
}

static mut mem_chunk_list: *mut MemChunk = core::ptr::null_mut();

extern "C" fn lemon_malloc(n_byte_1: u64) -> *mut () {
    unsafe {
        let mut p: *mut MemChunk = core::ptr::null_mut();
        if n_byte_1 < 0 as u64 { return core::ptr::null_mut(); }
        p =
            unsafe {
                    malloc(n_byte_1 + core::mem::size_of::<MemChunk>() as u64)
                } as *mut MemChunk;
        if p == core::ptr::null_mut() {
            eprintln!("Out of memory.  Failed to allocate {} bytes.", n_byte_1 as i64 as i64);
            unsafe { exit(1) };
        }
        unsafe { (*p).p_next = mem_chunk_list };
        unsafe { (*p).sz = n_byte_1 };
        mem_chunk_list = p;
        return unsafe { &raw mut *p.offset(1 as isize) } as *mut ();
    }
}

extern "C" fn lemon_calloc(n_elem_1: u64, sz: u64) -> *mut () {
    let p: *mut () = lemon_malloc(n_elem_1 * sz);
    unsafe { memset(p, 0, n_elem_1 * sz) };
    return p;
}

extern "C" fn lemon_free(p_old_1: *mut ()) -> () {
    if !(p_old_1).is_null() {
        let mut p: *const MemChunk =
            p_old_1 as *mut MemChunk as *const MemChunk;
        {
            let __p = &mut p;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(-1) };
            __t
        };
        unsafe { memset(p_old_1, 0, unsafe { (*p).sz }) };
    }
}

extern "C" fn lemon_realloc(p_old_1: *mut (), n_new_1: u64) -> *mut () {
    let mut p_new: *mut () = core::ptr::null_mut();
    let mut p: *const MemChunk = core::ptr::null();
    if p_old_1 == core::ptr::null_mut() { return lemon_malloc(n_new_1); }
    p = p_old_1 as *mut MemChunk;
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(-1) };
        __t
    };
    if unsafe { (*p).sz } >= n_new_1 { return p_old_1; }
    p_new = lemon_malloc(n_new_1);
    unsafe { memcpy(p_new, p_old_1 as *const (), unsafe { (*p).sz }) };
    return p_new;
}

extern "C" fn lemon_free_all() -> () {
    unsafe {
        while !(mem_chunk_list).is_null() {
            let p_next: *mut MemChunk = unsafe { (*mem_chunk_list).p_next };
            unsafe { free(mem_chunk_list as *mut ()) };
            mem_chunk_list = p_next;
        }
    }
}

extern "C" fn lemon_addtext(z_buf_1: *mut i8, pn_used_1: &mut i32,
    z_in_1: *const i8, mut n_in_1: i32, mut i_width_1: i32) -> () {
    if n_in_1 < 0 {
        {
            n_in_1 = 0;
            '__b6: loop {
                if !(unsafe { *z_in_1.offset(n_in_1 as isize) } != 0) {
                    break '__b6;
                }
                '__c6: loop { break '__c6; }
                { let __p = &mut n_in_1; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    while i_width_1 > n_in_1 {
        unsafe {
            *z_buf_1.offset({
                                let __p = &mut *pn_used_1;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = ' ' as i32 as i8
        };
        { let __p = &mut i_width_1; let __t = *__p; *__p -= 1; __t };
    }
    if n_in_1 == 0 { return; }
    unsafe {
        memcpy(unsafe { &raw mut *z_buf_1.offset(*pn_used_1 as isize) } as
                *mut (), z_in_1 as *const (), n_in_1 as u64)
    };
    *pn_used_1 += n_in_1;
    while -i_width_1 > n_in_1 {
        unsafe {
            *z_buf_1.offset({
                                let __p = &mut *pn_used_1;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = ' ' as i32 as i8
        };
        { let __p = &mut i_width_1; let __t = *__p; *__p += 1; __t };
    }
    unsafe { *z_buf_1.offset(*pn_used_1 as isize) = 0 as i8 };
}

extern "C" fn lemon_vsprintf(str: *mut i8, z_format_1: *const i8,
    mut ap: *const i8) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut c: i32 = 0;
    let mut n_used: i32 = 0;
    let mut z: *const i8 = core::ptr::null();
    let mut z_temp: [i8; 50] = [0; 50];
    unsafe { *str.offset(0 as isize) = 0 as i8 };
    {
        i = { j = 0; j };
        '__b9: loop {
            if !({ c = unsafe { *z_format_1.offset(i as isize) } as i32; c }
                            != 0) {
                break '__b9;
            }
            '__c9: loop {
                if c == '%' as i32 {
                    let mut i_width: i32 = 0;
                    lemon_addtext(str, &mut n_used,
                        unsafe { &*z_format_1.offset(j as isize) }, i - j, 0);
                    c =
                        unsafe {
                                *z_format_1.offset({ let __p = &mut i; *__p += 1; *__p } as
                                            isize)
                            } as i32;
                    if unsafe { isdigit(c as u8 as i32) } != 0 ||
                            c == '-' as i32 &&
                                unsafe {
                                        isdigit(unsafe { *z_format_1.offset((i + 1) as isize) } as
                                                    u8 as i32)
                                    } != 0 {
                        if c == '-' as i32 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                        while unsafe {
                                    isdigit(unsafe { *z_format_1.offset(i as isize) } as u8 as
                                            i32)
                                } != 0 {
                            i_width =
                                i_width * 10 +
                                        unsafe {
                                                *z_format_1.offset({
                                                                let __p = &mut i;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize)
                                            } as i32 - '0' as i32;
                        }
                        if c == '-' as i32 { i_width = -i_width; }
                        c = unsafe { *z_format_1.offset(i as isize) } as i32;
                    }
                    if c == 'd' as i32 {
                        let mut v: i32 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        if v < 0 {
                            lemon_addtext(str, &mut n_used,
                                c"-".as_ptr() as *mut i8 as *const i8, 1, i_width);
                            v = -v;
                        } else if v == 0 {
                            lemon_addtext(str, &mut n_used,
                                c"0".as_ptr() as *mut i8 as *const i8, 1, i_width);
                        }
                        k = 0;
                        while v > 0 {
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            z_temp[(core::mem::size_of::<[i8; 50]>() as u64 - k as u64)
                                        as usize] = (v % 10 + '0' as i32) as i8;
                            v /= 10;
                        }
                        lemon_addtext(str, &mut n_used,
                            &raw mut z_temp[(core::mem::size_of::<[i8; 50]>() as u64 -
                                                k as u64) as usize] as *const i8, k, i_width);
                    } else if c == 's' as i32 {
                        z =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                *(__va_p as *const *const i8)
                            };
                        lemon_addtext(str, &mut n_used, z, -1, i_width);
                    } else if c == '.' as i32 &&
                            unsafe {
                                    memcmp(unsafe { &raw const *z_format_1.offset(i as isize) }
                                            as *const (), c".*s".as_ptr() as *mut i8 as *const (),
                                        3 as u64)
                                } == 0 {
                        i += 2;
                        k =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                *(__va_p as *const i32)
                            };
                        z =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                                *(__va_p as *const *const i8)
                            };
                        lemon_addtext(str, &mut n_used, z, k, i_width);
                    } else if c == '%' as i32 {
                        lemon_addtext(str, &mut n_used,
                            c"%".as_ptr() as *mut i8 as *const i8, 1, 0);
                    } else { eprintln!("illegal format"); unsafe { exit(1) }; }
                    j = i + 1;
                }
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    lemon_addtext(str, &mut n_used,
        unsafe { &*z_format_1.offset(j as isize) }, i - j, 0);
    return n_used;
}

unsafe extern "C" fn lemon_sprintf(str: *mut i8, format: *const i8,
    mut __va0: ...) -> i32 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    rc = lemon_vsprintf(str, format, ap as *const i8);
    ();
    return rc;
}

extern "C" fn lemon_strcpy(mut dest: *mut i8, mut src: *const i8) -> () {
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

extern "C" fn lemon_strcat(mut dest: *mut i8, src: *const i8) -> () {
    while unsafe { *dest } != 0 {
        {
            let __p = &mut dest;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    lemon_strcpy(dest, src);
}

extern "C" fn action_new() -> *mut Action {
    unsafe {
        let mut newaction: *mut Action = core::ptr::null_mut();
        if actionfreelist == core::ptr::null_mut() {
            let mut i: i32 = 0;
            let amt: i32 = 100;
            actionfreelist =
                lemon_calloc(amt as u64,
                        core::mem::size_of::<Action>() as u64) as *mut Action;
            if actionfreelist == core::ptr::null_mut() {
                eprint!("Unable to allocate memory for a new parser action.");
                unsafe { exit(1) };
            }
            {
                i = 0;
                '__b14: loop {
                    if !(i < amt - 1) { break '__b14; }
                    '__c14: loop {
                        unsafe {
                            (*actionfreelist.offset(i as isize)).next =
                                unsafe { actionfreelist.offset((i + 1) as isize) }
                        };
                        break '__c14;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe {
                (*actionfreelist.offset((amt - 1) as isize)).next =
                    core::ptr::null_mut()
            };
        }
        newaction = actionfreelist;
        actionfreelist = unsafe { (*actionfreelist).next };
        return newaction;
    }
}

extern "C" fn actioncmp(ap1: *mut Action, ap2: *mut Action) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        rc =
            unsafe { (*unsafe { (*ap1).sp }).index } -
                unsafe { (*unsafe { (*ap2).sp }).index };
        if rc == 0 {
            rc =
                unsafe { (*ap1).type_ } as i32 -
                    unsafe { (*ap2).type_ } as i32;
        }
        if rc == 0 &&
                (unsafe { (*ap1).type_ } == REDUCE ||
                    unsafe { (*ap1).type_ } == SHIFTREDUCE) {
            rc =
                unsafe { (*unsafe { (*ap1).x.rp }).index } -
                    unsafe { (*unsafe { (*ap2).x.rp }).index };
        }
        if rc == 0 { rc = unsafe { ap2.offset_from(ap1) } as i64 as i32; }
        return rc;
    }
}

extern "C" fn action_sort(mut ap: *mut Action) -> *mut Action {
    ap =
        msort(ap as *mut i8, unsafe { &raw mut (*ap).next } as *mut *mut i8,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*const i8, *const i8)
                                    -> i32>(actioncmp as *const ())
                    })) as *mut Action;
    return ap;
}

static mut currentend: *mut *mut Config = core::ptr::null_mut();

#[repr(C)]
#[derive(Copy, Clone)]
struct SX4 {
    size: i32,
    count: i32,
    tbl: *mut SX4node,
    ht: *mut *mut SX4node,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SX4node {
    data: *mut Config,
    next: *mut SX4node,
    from: *mut *mut SX4node,
}

static mut x4a: *mut SX4 = unsafe { core::mem::zeroed() };

type X4node = SX4node;

#[unsafe(no_mangle)]
pub extern "C" fn configtable_init() -> () {
    unsafe {
        if !(x4a).is_null() { return; }
        x4a = lemon_malloc(core::mem::size_of::<SX4>() as u64) as *mut SX4;
        if !(x4a).is_null() {
            unsafe { (*x4a).size = 64 };
            unsafe { (*x4a).count = 0 };
            unsafe {
                (*x4a).tbl =
                    lemon_calloc(64 as u64,
                                core::mem::size_of::<X4node>() as u64 +
                                    core::mem::size_of::<*mut X4node>() as u64) as *mut X4node
                        as *mut SX4node
            };
            if unsafe { (*x4a).tbl } == core::ptr::null_mut() {
                lemon_free(x4a as *mut ());
                x4a = core::ptr::null_mut();
            } else {
                let mut i: i32 = 0;
                unsafe {
                    (*x4a).ht =
                        unsafe {
                                    &raw mut *unsafe { (*x4a).tbl.offset(64 as isize) }
                                } as *mut *mut X4node as *mut *mut SX4node
                };
                {
                    i = 0;
                    '__b15: loop {
                        if !(i < 64) { break '__b15; }
                        '__c15: loop {
                            unsafe {
                                *unsafe { (*x4a).ht.offset(i as isize) } =
                                    core::ptr::null_mut()
                            };
                            break '__c15;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configlist_init() -> () {
    unsafe {
        current = core::ptr::null_mut();
        currentend = &mut current;
        basis = core::ptr::null_mut();
        basisend = &mut basis;
        configtable_init();
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configlist_sort() -> () {
    unsafe {
        current =
            msort(current as *mut i8,
                    unsafe { &raw mut (*current).next } as *mut *mut i8,
                    Some(configcmp)) as *mut Config;
        currentend = core::ptr::null_mut();
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configtable_clear(f:
        Option<unsafe extern "C" fn(*mut Config) -> i32>) -> () {
    unsafe {
        let mut i: i32 = 0;
        if x4a == core::ptr::null_mut() || unsafe { (*x4a).count } == 0 {
            return;
        }
        if f.is_some() {
            {
                i = 0;
                '__b16: loop {
                    if !(i < unsafe { (*x4a).count }) { break '__b16; }
                    '__c16: loop {
                        unsafe {
                            f.unwrap()(unsafe {
                                    (*unsafe { (*x4a).tbl.offset(i as isize) }).data
                                })
                        };
                        break '__c16;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        {
            i = 0;
            '__b17: loop {
                if !(i < unsafe { (*x4a).size }) { break '__b17; }
                '__c17: loop {
                    unsafe {
                        *unsafe { (*x4a).ht.offset(i as isize) } =
                            core::ptr::null_mut()
                    };
                    break '__c17;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*x4a).count = 0 };
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configlist_reset() -> () {
    unsafe {
        current = core::ptr::null_mut();
        currentend = &mut current;
        basis = core::ptr::null_mut();
        basisend = &mut basis;
        unsafe { configtable_clear(None) };
        return;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn error_msg(filename: *const i8, lineno: i32,
    format: *const i8, mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe {
            fprintf(__stderrp, c"%s:%d: ".as_ptr() as *mut i8 as *const i8,
                filename, lineno)
        };
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, format, ap) };
        ();
        eprintln!("");
    }
}

const OPT_FLAG: u32 = 1;

const OPT_INT: u32 = 2;

const OPT_DBL: u32 = 3;

const OPT_STR: u32 = 4;

const OPT_FFLAG: u32 = 5;

const OPT_FINT: u32 = 6;

const OPT_FDBL: u32 = 7;

const OPT_FSTR: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
struct SOptions {
    type_: u32,
    label: *const i8,
    arg: *mut i8,
    message: *const i8,
}

static mut g_argv: *mut *mut i8 = unsafe { core::mem::zeroed() };

static mut op: *mut SOptions = unsafe { core::mem::zeroed() };

static mut errstream: *mut FILE = unsafe { core::mem::zeroed() };

static mut emsg: [i8; 28] =
    [67 as i8, 111 as i8, 109 as i8, 109 as i8, 97 as i8, 110 as i8,
            100 as i8, 32 as i8, 108 as i8, 105 as i8, 110 as i8, 101 as i8,
            32 as i8, 115 as i8, 121 as i8, 110 as i8, 116 as i8, 97 as i8,
            120 as i8, 32 as i8, 101 as i8, 114 as i8, 114 as i8, 111 as i8,
            114 as i8, 58 as i8, 32 as i8, 0 as i8];

extern "C" fn errline(n: i32, k: i32, err: *mut FILE) -> () {
    unsafe {
        let mut spcnt: i32 = 0;
        let mut i: i32 = 0;
        if !(unsafe { *g_argv.offset(0 as isize) }).is_null() {
            unsafe {
                fprintf(err, c"%s".as_ptr() as *mut i8 as *const i8,
                    unsafe { *g_argv.offset(0 as isize) })
            };
            spcnt =
                unsafe {
                            strlen(unsafe { *g_argv.offset(0 as isize) } as *const i8)
                        } as i32 + 1;
        } else { spcnt = 0; }
        {
            i = 1;
            '__b18: loop {
                if !(i < n &&
                                !(unsafe { *g_argv.offset(i as isize) }).is_null()) {
                    break '__b18;
                }
                '__c18: loop {
                    unsafe {
                        fprintf(err, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { *g_argv.offset(i as isize) })
                    };
                    spcnt +=
                        unsafe {
                                    strlen(unsafe { *g_argv.offset(i as isize) } as *const i8)
                                } as i32 + 1;
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        spcnt += k;
        {
            '__b19: loop {
                if !(!(unsafe { *g_argv.offset(i as isize) }).is_null()) {
                    break '__b19;
                }
                '__c19: loop {
                    unsafe {
                        fprintf(err, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { *g_argv.offset(i as isize) })
                    };
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if spcnt < 20 {
            unsafe {
                fprintf(err,
                    c"\n%*s^-- here\n".as_ptr() as *mut i8 as *const i8, spcnt,
                    c"".as_ptr() as *mut i8)
            };
        } else {
            unsafe {
                fprintf(err,
                    c"\n%*shere --^\n".as_ptr() as *mut i8 as *const i8,
                    spcnt - 7, c"".as_ptr() as *mut i8)
            };
        }
    }
}

extern "C" fn handleflags(i: i32, err: *mut FILE) -> i32 {
    unsafe {
        let mut v: i32 = 0;
        let mut errcnt: i32 = 0;
        let mut j: i32 = 0;
        {
            j = 0;
            '__b20: loop {
                if !(!(unsafe { (*op.offset(j as isize)).label }).is_null()) {
                    break '__b20;
                }
                '__c20: loop {
                    if unsafe {
                                strncmp(unsafe {
                                            &raw mut *unsafe {
                                                        (*g_argv.offset(i as isize)).offset(1 as isize)
                                                    }
                                        } as *const i8, unsafe { (*op.offset(j as isize)).label },
                                    unsafe { strlen(unsafe { (*op.offset(j as isize)).label }) }
                                            as i32 as u64)
                            } == 0 {
                        break '__b20;
                    }
                    break '__c20;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        v =
            if unsafe {
                            *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                        } as i32 == '-' as i32 {
                1
            } else { 0 };
        if unsafe { (*op.offset(j as isize)).label } == core::ptr::null() {
            if !(err).is_null() {
                unsafe {
                    fprintf(err,
                        c"%sundefined option.\n".as_ptr() as *mut i8 as *const i8,
                        &raw mut emsg[0 as usize] as *mut i8)
                };
                errline(i, 1, err);
            }
            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
        } else if unsafe { (*op.offset(j as isize)).arg } ==
                core::ptr::null_mut()
            {} else if unsafe { (*op.offset(j as isize)).type_ } == OPT_FLAG {
            unsafe {
                *(unsafe { (*op.offset(j as isize)).arg } as *mut i32) = v
            };
        } else if unsafe { (*op.offset(j as isize)).type_ } == OPT_FFLAG {
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(i32)
                                    -> ()>(unsafe { (*op.offset(j as isize)).arg } as *const ())
                    })(v)
            };
        } else if unsafe { (*op.offset(j as isize)).type_ } == OPT_FSTR {
            unsafe {
                (unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut i8)
                                    -> ()>(unsafe { (*op.offset(j as isize)).arg } as *const ())
                    })(unsafe {
                        &mut *unsafe {
                                    (*g_argv.offset(i as isize)).offset(2 as isize)
                                }
                    })
            };
        } else {
            if !(err).is_null() {
                unsafe {
                    fprintf(err,
                        c"%smissing argument on switch.\n".as_ptr() as *mut i8 as
                            *const i8, &raw mut emsg[0 as usize] as *mut i8)
                };
                errline(i, 1, err);
            }
            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
        }
        return errcnt;
    }
}

extern "C" fn handleswitch(i: i32, err: *mut FILE) -> i32 {
    unsafe {
        let mut lv: i32 = 0;
        let mut dv: f64 = 0.0;
        let mut sv: *mut i8 = core::ptr::null_mut();
        let mut end: *mut i8 = core::ptr::null_mut();
        let mut cp: *mut i8 = core::ptr::null_mut();
        let mut j: i32 = 0;
        let mut errcnt: i32 = 0;
        cp =
            unsafe {
                strchr(unsafe { *g_argv.offset(i as isize) } as *const i8,
                    '=' as i32)
            };
        if !(cp != core::ptr::null_mut()) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"handleswitch".as_ptr() as *const i8,
                    c"lemon.c".as_ptr() as *mut i8 as *const i8, 2139,
                    c"cp!=0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe { *cp = 0 as i8 };
        {
            j = 0;
            '__b21: loop {
                if !(!(unsafe { (*op.offset(j as isize)).label }).is_null()) {
                    break '__b21;
                }
                '__c21: loop {
                    if unsafe {
                                strcmp(unsafe { *g_argv.offset(i as isize) } as *const i8,
                                    unsafe { (*op.offset(j as isize)).label })
                            } == 0 {
                        break '__b21;
                    }
                    break '__c21;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { *cp = '=' as i32 as i8 };
        if unsafe { (*op.offset(j as isize)).label } == core::ptr::null() {
            if !(err).is_null() {
                unsafe {
                    fprintf(err,
                        c"%sundefined option.\n".as_ptr() as *mut i8 as *const i8,
                        &raw mut emsg[0 as usize] as *mut i8)
                };
                errline(i, 0, err);
            }
            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
        } else {
            {
                let __p = &mut cp;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            '__s22:
                {
                match unsafe { (*op.offset(j as isize)).type_ } {
                    OPT_FLAG => {
                        if !(err).is_null() {
                            unsafe {
                                fprintf(err,
                                    c"%soption requires an argument.\n".as_ptr() as *mut i8 as
                                        *const i8, &raw mut emsg[0 as usize] as *mut i8)
                            };
                            errline(i, 0, err);
                        }
                        { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
                    }
                    OPT_FFLAG => {
                        if !(err).is_null() {
                            unsafe {
                                fprintf(err,
                                    c"%soption requires an argument.\n".as_ptr() as *mut i8 as
                                        *const i8, &raw mut emsg[0 as usize] as *mut i8)
                            };
                            errline(i, 0, err);
                        }
                        { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
                    }
                    OPT_DBL => {
                        dv = unsafe { strtod(cp as *const i8, &mut end) };
                        if unsafe { *end } != 0 {
                            if !(err).is_null() {
                                unsafe {
                                    fprintf(err,
                                        c"%sillegal character in floating-point argument.\n".as_ptr()
                                                as *mut i8 as *const i8,
                                        &raw mut emsg[0 as usize] as *mut i8)
                                };
                                errline(i,
                                    unsafe {
                                                (end as
                                                        *mut i8).offset_from(unsafe { *g_argv.offset(i as isize) }
                                                        as *mut i8)
                                            } as i64 as i32, err);
                            }
                            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    OPT_FDBL => {
                        dv = unsafe { strtod(cp as *const i8, &mut end) };
                        if unsafe { *end } != 0 {
                            if !(err).is_null() {
                                unsafe {
                                    fprintf(err,
                                        c"%sillegal character in floating-point argument.\n".as_ptr()
                                                as *mut i8 as *const i8,
                                        &raw mut emsg[0 as usize] as *mut i8)
                                };
                                errline(i,
                                    unsafe {
                                                (end as
                                                        *mut i8).offset_from(unsafe { *g_argv.offset(i as isize) }
                                                        as *mut i8)
                                            } as i64 as i32, err);
                            }
                            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    OPT_INT => {
                        lv = unsafe { strtol(cp as *const i8, &mut end, 0) } as i32;
                        if unsafe { *end } != 0 {
                            if !(err).is_null() {
                                unsafe {
                                    fprintf(err,
                                        c"%sillegal character in integer argument.\n".as_ptr() as
                                                *mut i8 as *const i8, &raw mut emsg[0 as usize] as *mut i8)
                                };
                                errline(i,
                                    unsafe {
                                                (end as
                                                        *mut i8).offset_from(unsafe { *g_argv.offset(i as isize) }
                                                        as *mut i8)
                                            } as i64 as i32, err);
                            }
                            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    OPT_FINT => {
                        lv = unsafe { strtol(cp as *const i8, &mut end, 0) } as i32;
                        if unsafe { *end } != 0 {
                            if !(err).is_null() {
                                unsafe {
                                    fprintf(err,
                                        c"%sillegal character in integer argument.\n".as_ptr() as
                                                *mut i8 as *const i8, &raw mut emsg[0 as usize] as *mut i8)
                                };
                                errline(i,
                                    unsafe {
                                                (end as
                                                        *mut i8).offset_from(unsafe { *g_argv.offset(i as isize) }
                                                        as *mut i8)
                                            } as i64 as i32, err);
                            }
                            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    OPT_STR => { sv = cp; }
                    OPT_FSTR => { sv = cp; }
                    _ => {}
                }
            }
            '__s23:
                {
                match unsafe { (*op.offset(j as isize)).type_ } {
                    OPT_FLAG => {
                        break '__s23;
                        unsafe {
                            *(unsafe { (*op.offset(j as isize)).arg } as *mut f64) = dv
                        };
                    }
                    OPT_FFLAG => {
                        break '__s23;
                        unsafe {
                            *(unsafe { (*op.offset(j as isize)).arg } as *mut f64) = dv
                        };
                    }
                    OPT_DBL => {
                        unsafe {
                            *(unsafe { (*op.offset(j as isize)).arg } as *mut f64) = dv
                        };
                    }
                    OPT_FDBL => {
                        unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(f64)
                                                -> ()>(unsafe { (*op.offset(j as isize)).arg } as *const ())
                                })(dv)
                        };
                    }
                    OPT_INT => {
                        unsafe {
                            *(unsafe { (*op.offset(j as isize)).arg } as *mut i32) = lv
                        };
                    }
                    OPT_FINT => {
                        unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(i32)
                                                -> ()>(unsafe { (*op.offset(j as isize)).arg } as *const ())
                                })(lv as i32)
                        };
                    }
                    OPT_STR => {
                        unsafe {
                            *(unsafe { (*op.offset(j as isize)).arg } as *mut *mut i8) =
                                sv
                        };
                    }
                    OPT_FSTR => {
                        unsafe {
                            (unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut i8)
                                                -> ()>(unsafe { (*op.offset(j as isize)).arg } as *const ())
                                })(sv)
                        };
                    }
                    _ => {}
                }
            }
        }
        return errcnt;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn opt_print() -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut max: i32 = 0;
        let mut len: i32 = 0;
        max = 0;
        {
            i = 0;
            '__b24: loop {
                if !(!(unsafe { (*op.offset(i as isize)).label }).is_null()) {
                    break '__b24;
                }
                '__c24: loop {
                    len =
                        unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                as i32 + 1;
                    '__s25:
                        {
                        match unsafe { (*op.offset(i as isize)).type_ } {
                            OPT_FLAG => { break '__s25; len += 9; }
                            OPT_FFLAG => { break '__s25; len += 9; }
                            OPT_INT => { len += 9; }
                            OPT_FINT => { len += 9; }
                            OPT_DBL => { len += 6; }
                            OPT_FDBL => { len += 6; }
                            OPT_STR => { len += 8; }
                            OPT_FSTR => { len += 8; }
                            _ => {}
                        }
                    }
                    if len > max { max = len; }
                    break '__c24;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 0;
            '__b26: loop {
                if !(!(unsafe { (*op.offset(i as isize)).label }).is_null()) {
                    break '__b26;
                }
                '__c26: loop {
                    '__s27:
                        {
                        match unsafe { (*op.offset(i as isize)).type_ } {
                            OPT_FLAG => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%-*s  %s\n".as_ptr() as *mut i8 as *const i8, max,
                                        unsafe { (*op.offset(i as isize)).label },
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_FFLAG => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%-*s  %s\n".as_ptr() as *mut i8 as *const i8, max,
                                        unsafe { (*op.offset(i as isize)).label },
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_INT => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%s<integer>%*s  %s\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*op.offset(i as isize)).label },
                                        (max -
                                                    unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                                        as i32 - 9) as i32, c"".as_ptr() as *mut i8,
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_FINT => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%s<integer>%*s  %s\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*op.offset(i as isize)).label },
                                        (max -
                                                    unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                                        as i32 - 9) as i32, c"".as_ptr() as *mut i8,
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_DBL => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%s<real>%*s  %s\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*op.offset(i as isize)).label },
                                        (max -
                                                    unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                                        as i32 - 6) as i32, c"".as_ptr() as *mut i8,
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_FDBL => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%s<real>%*s  %s\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*op.offset(i as isize)).label },
                                        (max -
                                                    unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                                        as i32 - 6) as i32, c"".as_ptr() as *mut i8,
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_STR => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%s<string>%*s  %s\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*op.offset(i as isize)).label },
                                        (max -
                                                    unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                                        as i32 - 8) as i32, c"".as_ptr() as *mut i8,
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            OPT_FSTR => {
                                unsafe {
                                    fprintf(errstream,
                                        c"  -%s<string>%*s  %s\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*op.offset(i as isize)).label },
                                        (max -
                                                    unsafe { strlen(unsafe { (*op.offset(i as isize)).label }) }
                                                        as i32 - 8) as i32, c"".as_ptr() as *mut i8,
                                        unsafe { (*op.offset(i as isize)).message })
                                };
                            }
                            _ => {}
                        }
                    }
                    break '__c26;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn opt_init(a: *mut *mut i8, o: *mut SOptions, err: *mut FILE)
    -> i32 {
    unsafe {
        let mut errcnt: i32 = 0;
        g_argv = a;
        op = o;
        errstream = err;
        if !(g_argv).is_null() && !(unsafe { *g_argv }).is_null() &&
                !(op).is_null() {
            let mut i: i32 = 0;
            {
                i = 1;
                '__b28: loop {
                    if !(!(unsafe { *g_argv.offset(i as isize) }).is_null()) {
                        break '__b28;
                    }
                    '__c28: loop {
                        if unsafe {
                                            *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                                        } as i32 == '+' as i32 ||
                                unsafe {
                                            *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                                        } as i32 == '-' as i32 {
                            errcnt += handleflags(i, err);
                        } else if !(unsafe {
                                            strchr(unsafe { *g_argv.offset(i as isize) } as *const i8,
                                                '=' as i32)
                                        }).is_null() {
                            errcnt += handleswitch(i, err);
                        }
                        break '__c28;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if errcnt > 0 {
            unsafe {
                fprintf(err,
                    c"Valid command line options for \"%s\" are:\n".as_ptr() as
                            *mut i8 as *const i8, unsafe { *a })
            };
            opt_print();
            unsafe { exit(1) };
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn opt_n_args() -> i32 {
    unsafe {
        let mut cnt: i32 = 0;
        let mut dashdash: i32 = 0;
        let mut i: i32 = 0;
        if g_argv != core::ptr::null_mut() &&
                unsafe { *g_argv.offset(0 as isize) } != core::ptr::null_mut()
            {
            {
                i = 1;
                '__b29: loop {
                    if !(!(unsafe { *g_argv.offset(i as isize) }).is_null()) {
                        break '__b29;
                    }
                    '__c29: loop {
                        if dashdash != 0 ||
                                !(unsafe {
                                                                *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                                                            } as i32 == '-' as i32 ||
                                                    unsafe {
                                                                *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                                                            } as i32 == '+' as i32 ||
                                                unsafe {
                                                        strchr(unsafe { *g_argv.offset(i as isize) } as *const i8,
                                                            '=' as i32)
                                                    } != core::ptr::null_mut()) as i32 != 0 {
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe {
                                    strcmp(unsafe { *g_argv.offset(i as isize) } as *const i8,
                                        c"--".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            dashdash = 1;
                        }
                        break '__c29;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return cnt;
    }
}

extern "C" fn argindex(mut n: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut dashdash: i32 = 0;
        if g_argv != core::ptr::null_mut() &&
                unsafe { *g_argv } != core::ptr::null_mut() {
            {
                i = 1;
                '__b30: loop {
                    if !(!(unsafe { *g_argv.offset(i as isize) }).is_null()) {
                        break '__b30;
                    }
                    '__c30: loop {
                        if dashdash != 0 ||
                                !(unsafe {
                                                                *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                                                            } as i32 == '-' as i32 ||
                                                    unsafe {
                                                                *unsafe { (*g_argv.offset(i as isize)).offset(0 as isize) }
                                                            } as i32 == '+' as i32 ||
                                                unsafe {
                                                        strchr(unsafe { *g_argv.offset(i as isize) } as *const i8,
                                                            '=' as i32)
                                                    } != core::ptr::null_mut()) as i32 != 0 {
                            if n == 0 { return i; }
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        }
                        if unsafe {
                                    strcmp(unsafe { *g_argv.offset(i as isize) } as *const i8,
                                        c"--".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            dashdash = 1;
                        }
                        break '__c30;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return -1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn opt_arg(n: i32) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        i = argindex(n);
        return if i >= 0 {
                unsafe { *g_argv.offset(i as isize) }
            } else { core::ptr::null_mut() };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn opt_err(n: i32) -> () {
    unsafe {
        let mut i: i32 = 0;
        i = argindex(n);
        if i >= 0 { errline(i, 0, errstream); }
    }
}

static mut plink_freelist: *mut Plink = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn plink_new() -> *mut Plink {
    unsafe {
        let mut newlink: *mut Plink = core::ptr::null_mut();
        if plink_freelist == core::ptr::null_mut() {
            let mut i: i32 = 0;
            let amt: i32 = 100;
            plink_freelist =
                lemon_calloc(amt as u64, core::mem::size_of::<Plink>() as u64)
                    as *mut Plink;
            if plink_freelist == core::ptr::null_mut() {
                eprintln!("Unable to allocate memory for a new follow-set propagation link.");
                unsafe { exit(1) };
            }
            {
                i = 0;
                '__b31: loop {
                    if !(i < amt - 1) { break '__b31; }
                    '__c31: loop {
                        unsafe {
                            (*plink_freelist.offset(i as isize)).next =
                                unsafe { plink_freelist.offset((i + 1) as isize) }
                        };
                        break '__c31;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe {
                (*plink_freelist.offset((amt - 1) as isize)).next =
                    core::ptr::null_mut()
            };
        }
        newlink = plink_freelist;
        plink_freelist = unsafe { (*plink_freelist).next };
        return newlink;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn plink_copy(to: &mut *mut Plink, mut from: *mut Plink)
    -> () {
    let mut nextpl: *mut Plink = core::ptr::null_mut();
    while !(from).is_null() {
        nextpl = unsafe { (*from).next };
        unsafe { (*from).next = *to };
        *to = from;
        from = nextpl;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn plink_delete(mut plp: *mut Plink) -> () {
    unsafe {
        let mut nextpl: *mut Plink = core::ptr::null_mut();
        while !(plp).is_null() {
            nextpl = unsafe { (*plp).next };
            unsafe { (*plp).next = plink_freelist };
            plink_freelist = plp;
            plp = nextpl;
        }
    }
}

static mut size: i32 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn set_size(n: i32) -> () { unsafe { size = n + 1; } }

#[unsafe(no_mangle)]
pub extern "C" fn set_free(s: *mut i8) -> () { lemon_free(s as *mut ()); }

#[unsafe(no_mangle)]
pub extern "C" fn set_add(s: *mut i8, e: i32) -> i32 {
    unsafe {
        let mut rv: i32 = 0;
        if !(e >= 0 && e < size) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"SetAdd".as_ptr() as *const i8,
                    c"lemon.c".as_ptr() as *mut i8 as *const i8, 5384,
                    c"e>=0 && e<size".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        rv = unsafe { *s.offset(e as isize) } as i32;
        unsafe { *s.offset(e as isize) = 1 as i8 };
        return (rv == 0) as i32 as i32;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_union(s1: *mut i8, s2: *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut progress: i32 = 0;
        progress = 0;
        {
            i = 0;
            '__b34: loop {
                if !(i < size) { break '__b34; }
                '__c34: loop {
                    if unsafe { *s2.offset(i as isize) } as i32 == 0 {
                        break '__c34;
                    }
                    if unsafe { *s1.offset(i as isize) } as i32 == 0 {
                        progress = 1;
                        unsafe { *s1.offset(i as isize) = 1 as i8 };
                    }
                    break '__c34;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return progress;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SX1node {
    data: *const i8,
    next: *mut SX1node,
    from: *mut *mut SX1node,
}

type X1node = SX1node;

#[repr(C)]
#[derive(Copy, Clone)]
struct SX1 {
    size: i32,
    count: i32,
}

static mut x1a: *mut SX1 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
pub extern "C" fn strhash(mut x: *const i8) -> u32 {
    let mut h: u32 = 0 as u32;
    while unsafe { *x } != 0 {
        h =
            h * 13 as u32 +
                unsafe {
                        *{
                                let __p = &mut x;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as u32;
    }
    return h;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SX2node {
    data: *mut Symbol,
    key: *const i8,
    next: *mut SX2node,
    from: *mut *mut SX2node,
}

type X2node = SX2node;

#[repr(C)]
#[derive(Copy, Clone)]
struct SX2 {
    size: i32,
    count: i32,
}

static mut x2a: *mut SX2 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
pub extern "C" fn symbolcmpp(_a: *const (), _b: *const ()) -> i32 {
    let a: *const Symbol = unsafe { *(_a as *mut *const Symbol) };
    let b: *const Symbol = unsafe { *(_b as *mut *const Symbol) };
    let i1: i32 =
        if unsafe { (*a).type_ } == MULTITERMINAL {
            3
        } else {
            if unsafe { *unsafe { (*a).name.offset(0 as isize) } } as i32 >
                    'Z' as i32 {
                2
            } else { 1 }
        };
    let i2: i32 =
        if unsafe { (*b).type_ } == MULTITERMINAL {
            3
        } else {
            if unsafe { *unsafe { (*b).name.offset(0 as isize) } } as i32 >
                    'Z' as i32 {
                2
            } else { 1 }
        };
    return if i1 == i2 {
            (unsafe { (*a).index }) - unsafe { (*b).index } as i32
        } else { i1 - i2 };
}

#[unsafe(no_mangle)]
pub extern "C" fn symbol_count() -> i32 {
    unsafe {
        return if !(x2a).is_null() { unsafe { (*x2a).count } } else { 0 };
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SX3 {
    size: i32,
    count: i32,
    tbl: *mut SX3node,
    ht: *mut *mut SX3node,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SX3node {
    data: *mut State,
    key: *mut Config,
    next: *mut SX3node,
    from: *mut *mut SX3node,
}

static mut x3a: *mut SX3 = unsafe { core::mem::zeroed() };

type X3node = SX3node;

#[unsafe(no_mangle)]
pub extern "C" fn state_init() -> () {
    unsafe {
        if !(x3a).is_null() { return; }
        x3a = lemon_malloc(core::mem::size_of::<SX3>() as u64) as *mut SX3;
        if !(x3a).is_null() {
            unsafe { (*x3a).size = 128 };
            unsafe { (*x3a).count = 0 };
            unsafe {
                (*x3a).tbl =
                    lemon_calloc(128 as u64,
                                core::mem::size_of::<X3node>() as u64 +
                                    core::mem::size_of::<*mut X3node>() as u64) as *mut X3node
                        as *mut SX3node
            };
            if unsafe { (*x3a).tbl } == core::ptr::null_mut() {
                lemon_free(x3a as *mut ());
                x3a = core::ptr::null_mut();
            } else {
                let mut i: i32 = 0;
                unsafe {
                    (*x3a).ht =
                        unsafe {
                                    &raw mut *unsafe { (*x3a).tbl.offset(128 as isize) }
                                } as *mut *mut X3node as *mut *mut SX3node
                };
                {
                    i = 0;
                    '__b36: loop {
                        if !(i < 128) { break '__b36; }
                        '__c36: loop {
                            unsafe {
                                *unsafe { (*x3a).ht.offset(i as isize) } =
                                    core::ptr::null_mut()
                            };
                            break '__c36;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn statehash(mut a: *const Config) -> u32 {
    unsafe {
        let mut h: u32 = 0 as u32;
        while !(a).is_null() {
            h =
                h * 571 as u32 +
                        (unsafe { (*unsafe { (*a).rp }).index } * 37) as u32 +
                    unsafe { (*a).dot } as u32;
            a = unsafe { (*a).bp };
        }
        return h;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn statecmp(mut a: *const Config, mut b: *const Config)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        {
            rc = 0;
            '__b38: loop {
                if !(rc == 0 && !(a).is_null() && !(b).is_null()) {
                    break '__b38;
                }
                '__c38: loop {
                    rc =
                        unsafe { (*unsafe { (*a).rp }).index } -
                            unsafe { (*unsafe { (*b).rp }).index };
                    if rc == 0 {
                        rc = unsafe { (*a).dot } - unsafe { (*b).dot };
                    }
                    break '__c38;
                }
                { a = unsafe { (*a).bp }; b = unsafe { (*b).bp } };
            }
        }
        if rc == 0 {
            if !(a).is_null() { rc = 1; }
            if !(b).is_null() { rc = -1; }
        }
        return rc;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn state_insert(data: *mut State, key: *mut Config) -> i32 {
    unsafe {
        let mut np: *mut X3node = core::ptr::null_mut();
        let mut h: u32 = 0 as u32;
        let mut ph: u32 = 0 as u32;
        if x3a == core::ptr::null_mut() { return 0; }
        ph = statehash(key as *const Config);
        h = ph & (unsafe { (*x3a).size } - 1) as u32;
        np = unsafe { *unsafe { (*x3a).ht.add(h as usize) } } as *mut X3node;
        while !(np).is_null() {
            if statecmp(unsafe { (*np).key } as *const Config,
                        key as *const Config) == 0 {
                return 0;
            }
            np = unsafe { (*np).next } as *mut X3node;
        }
        if unsafe { (*x3a).count } >= unsafe { (*x3a).size } {
            let mut i: i32 = 0;
            let mut arr_size: i32 = 0;
            let mut array: SX3 = unsafe { core::mem::zeroed() };
            array.size = { arr_size = unsafe { (*x3a).size } * 2; arr_size };
            array.count = unsafe { (*x3a).count };
            array.tbl =
                lemon_calloc(arr_size as u64,
                            core::mem::size_of::<X3node>() as u64 +
                                core::mem::size_of::<*mut X3node>() as u64) as *mut X3node
                    as *mut SX3node;
            if array.tbl == core::ptr::null_mut() { return 0; }
            array.ht =
                unsafe { &raw mut *array.tbl.offset(arr_size as isize) } as
                        *mut *mut X3node as *mut *mut SX3node;
            {
                i = 0;
                '__b40: loop {
                    if !(i < arr_size) { break '__b40; }
                    '__c40: loop {
                        unsafe {
                            *array.ht.offset(i as isize) = core::ptr::null_mut()
                        };
                        break '__c40;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                i = 0;
                '__b41: loop {
                    if !(i < unsafe { (*x3a).count }) { break '__b41; }
                    '__c41: loop {
                        let mut oldnp: *const X3node = core::ptr::null();
                        let mut newnp: *mut X3node = core::ptr::null_mut();
                        oldnp =
                            unsafe { unsafe { (*x3a).tbl.offset(i as isize) } } as
                                *mut X3node;
                        h =
                            statehash(unsafe { (*oldnp).key } as *const Config) &
                                (arr_size - 1) as u32;
                        newnp =
                            unsafe { array.tbl.offset(i as isize) } as *mut X3node;
                        if !(unsafe { *array.ht.add(h as usize) }).is_null() {
                            unsafe {
                                (*unsafe { *array.ht.add(h as usize) }).from =
                                    unsafe { &mut (*newnp).next }
                            };
                        }
                        unsafe {
                            (*newnp).next = unsafe { *array.ht.add(h as usize) }
                        };
                        unsafe { (*newnp).key = unsafe { (*oldnp).key } };
                        unsafe { (*newnp).data = unsafe { (*oldnp).data } };
                        unsafe {
                            (*newnp).from = unsafe { array.ht.add(h as usize) }
                        };
                        unsafe {
                            *array.ht.add(h as usize) = newnp as *mut SX3node
                        };
                        break '__c41;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            lemon_free(unsafe { (*x3a).tbl } as *mut ());
            unsafe { *x3a = array };
        }
        h = ph & (unsafe { (*x3a).size } - 1) as u32;
        np =
            unsafe {
                    unsafe {
                        (*x3a).tbl.offset({
                                    let __p = unsafe { &mut (*x3a).count };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                    }
                } as *mut X3node;
        unsafe { (*np).key = key };
        unsafe { (*np).data = data };
        if !(unsafe { *unsafe { (*x3a).ht.add(h as usize) } }).is_null() {
            unsafe {
                (*unsafe { *unsafe { (*x3a).ht.add(h as usize) } }).from =
                    unsafe { &mut (*np).next }
            };
        }
        unsafe {
            (*np).next = unsafe { *unsafe { (*x3a).ht.add(h as usize) } }
        };
        unsafe { *unsafe { (*x3a).ht.add(h as usize) } = np as *mut SX3node };
        unsafe {
            (*np).from = unsafe { unsafe { (*x3a).ht.add(h as usize) } }
        };
        return 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn state_find(key: *mut Config) -> *mut State {
    unsafe {
        let mut h: u32 = 0 as u32;
        let mut np: *const X3node = core::ptr::null();
        if x3a == core::ptr::null_mut() { return core::ptr::null_mut(); }
        h =
            statehash(key as *const Config) &
                (unsafe { (*x3a).size } - 1) as u32;
        np = unsafe { *unsafe { (*x3a).ht.add(h as usize) } } as *mut X3node;
        while !(np).is_null() {
            if statecmp(unsafe { (*np).key } as *const Config,
                        key as *const Config) == 0 {
                break;
            }
            np = unsafe { (*np).next } as *mut X3node;
        }
        return if !(np).is_null() {
                unsafe { (*np).data }
            } else { core::ptr::null_mut() };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn state_arrayof() -> *mut *mut State {
    unsafe {
        let mut array: *mut *mut State = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut arr_size: i32 = 0;
        if x3a == core::ptr::null_mut() { return core::ptr::null_mut(); }
        arr_size = unsafe { (*x3a).count };
        array =
            lemon_calloc(arr_size as u64,
                    core::mem::size_of::<*mut State>() as u64) as
                *mut *mut State;
        if !(array).is_null() {
            {
                i = 0;
                '__b43: loop {
                    if !(i < arr_size) { break '__b43; }
                    '__c43: loop {
                        unsafe {
                            *array.offset(i as isize) =
                                unsafe { (*unsafe { (*x3a).tbl.offset(i as isize) }).data }
                        };
                        break '__c43;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return array;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn confighash(a: &Config) -> u32 {
    unsafe {
        let mut h: u32 = 0 as u32;
        h = (unsafe { (*(*a).rp).index } * 37 + (*a).dot) as u32;
        return h;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configtable_insert(data: *mut Config) -> i32 {
    unsafe {
        let mut np: *mut X4node = core::ptr::null_mut();
        let mut h: u32 = 0 as u32;
        let mut ph: u32 = 0 as u32;
        if x4a == core::ptr::null_mut() { return 0; }
        ph = confighash(unsafe { &*data });
        h = ph & (unsafe { (*x4a).size } - 1) as u32;
        np = unsafe { *unsafe { (*x4a).ht.add(h as usize) } } as *mut X4node;
        while !(np).is_null() {
            if configcmp(unsafe { (*np).data } as *const i8,
                        data as *const i8) == 0 {
                return 0;
            }
            np = unsafe { (*np).next } as *mut X4node;
        }
        if unsafe { (*x4a).count } >= unsafe { (*x4a).size } {
            let mut i: i32 = 0;
            let mut arr_size: i32 = 0;
            let mut array: SX4 = unsafe { core::mem::zeroed() };
            array.size = { arr_size = unsafe { (*x4a).size } * 2; arr_size };
            array.count = unsafe { (*x4a).count };
            array.tbl =
                lemon_calloc(arr_size as u64,
                            core::mem::size_of::<X4node>() as u64 +
                                core::mem::size_of::<*mut X4node>() as u64) as *mut X4node
                    as *mut SX4node;
            if array.tbl == core::ptr::null_mut() { return 0; }
            array.ht =
                unsafe { &raw mut *array.tbl.offset(arr_size as isize) } as
                        *mut *mut X4node as *mut *mut SX4node;
            {
                i = 0;
                '__b45: loop {
                    if !(i < arr_size) { break '__b45; }
                    '__c45: loop {
                        unsafe {
                            *array.ht.offset(i as isize) = core::ptr::null_mut()
                        };
                        break '__c45;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                i = 0;
                '__b46: loop {
                    if !(i < unsafe { (*x4a).count }) { break '__b46; }
                    '__c46: loop {
                        let mut oldnp: *const X4node = core::ptr::null();
                        let mut newnp: *mut X4node = core::ptr::null_mut();
                        oldnp =
                            unsafe { unsafe { (*x4a).tbl.offset(i as isize) } } as
                                *mut X4node;
                        h =
                            confighash(unsafe { &*unsafe { (*oldnp).data } }) &
                                (arr_size - 1) as u32;
                        newnp =
                            unsafe { array.tbl.offset(i as isize) } as *mut X4node;
                        if !(unsafe { *array.ht.add(h as usize) }).is_null() {
                            unsafe {
                                (*unsafe { *array.ht.add(h as usize) }).from =
                                    unsafe { &mut (*newnp).next }
                            };
                        }
                        unsafe {
                            (*newnp).next = unsafe { *array.ht.add(h as usize) }
                        };
                        unsafe { (*newnp).data = unsafe { (*oldnp).data } };
                        unsafe {
                            (*newnp).from = unsafe { array.ht.add(h as usize) }
                        };
                        unsafe {
                            *array.ht.add(h as usize) = newnp as *mut SX4node
                        };
                        break '__c46;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { *x4a = array };
        }
        h = ph & (unsafe { (*x4a).size } - 1) as u32;
        np =
            unsafe {
                    unsafe {
                        (*x4a).tbl.offset({
                                    let __p = unsafe { &mut (*x4a).count };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                    }
                } as *mut X4node;
        unsafe { (*np).data = data };
        if !(unsafe { *unsafe { (*x4a).ht.add(h as usize) } }).is_null() {
            unsafe {
                (*unsafe { *unsafe { (*x4a).ht.add(h as usize) } }).from =
                    unsafe { &mut (*np).next }
            };
        }
        unsafe {
            (*np).next = unsafe { *unsafe { (*x4a).ht.add(h as usize) } }
        };
        unsafe { *unsafe { (*x4a).ht.add(h as usize) } = np as *mut SX4node };
        unsafe {
            (*np).from = unsafe { unsafe { (*x4a).ht.add(h as usize) } }
        };
        return 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn configtable_find(key: *mut Config) -> *mut Config {
    unsafe {
        let mut h: i32 = 0;
        let mut np: *const X4node = core::ptr::null();
        if x4a == core::ptr::null_mut() { return core::ptr::null_mut(); }
        h =
            (confighash(unsafe { &*key }) &
                    (unsafe { (*x4a).size } - 1) as u32) as i32;
        np =
            unsafe { *unsafe { (*x4a).ht.offset(h as isize) } } as
                *mut X4node;
        while !(np).is_null() {
            if configcmp(unsafe { (*np).data } as *const i8, key as *const i8)
                    == 0 {
                break;
            }
            np = unsafe { (*np).next } as *mut X4node;
        }
        return if !(np).is_null() {
                unsafe { (*np).data }
            } else { core::ptr::null_mut() };
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct LookaheadAction {
    lookahead: i32,
    action: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Acttab {
    n_action: i32,
    n_action_alloc: i32,
    a_action: *mut LookaheadAction,
    a_lookahead: *mut LookaheadAction,
    mn_lookahead: i32,
    mn_action: i32,
    mx_lookahead: i32,
    n_lookahead: i32,
    n_lookahead_alloc: i32,
    nterminal: i32,
    nsymbol: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn acttab_free(p: *mut Acttab) -> () {
    lemon_free(unsafe { (*p).a_action } as *mut ());
    lemon_free(unsafe { (*p).a_lookahead } as *mut ());
    lemon_free(p as *mut ());
}

#[unsafe(no_mangle)]
pub extern "C" fn acttab_alloc(nsymbol: i32, nterminal: i32) -> *mut Acttab {
    let p: *mut Acttab =
        lemon_calloc(1 as u64, core::mem::size_of::<Acttab>() as u64) as
            *mut Acttab;
    if p == core::ptr::null_mut() {
        eprint!("Unable to allocate memory for a new acttab.");
        unsafe { exit(1) };
    }
    unsafe { memset(p as *mut (), 0, core::mem::size_of::<Acttab>() as u64) };
    unsafe { (*p).nsymbol = nsymbol };
    unsafe { (*p).nterminal = nterminal };
    return p;
}

#[unsafe(no_mangle)]
pub extern "C" fn acttab_action(p: &mut Acttab, lookahead: i32, action: i32)
    -> () {
    if (*p).n_lookahead >= (*p).n_lookahead_alloc {
        (*p).n_lookahead_alloc += 25;
        (*p).a_lookahead =
            lemon_realloc((*p).a_lookahead as *mut (),
                    core::mem::size_of::<LookaheadAction>() as u64 *
                        (*p).n_lookahead_alloc as u64) as *mut LookaheadAction;
        if (*p).a_lookahead == core::ptr::null_mut() {
            eprintln!("malloc failed");
            unsafe { exit(1) };
        }
    }
    if (*p).n_lookahead == 0 {
        (*p).mx_lookahead = lookahead;
        (*p).mn_lookahead = lookahead;
        (*p).mn_action = action;
    } else {
        if (*p).mx_lookahead < lookahead { (*p).mx_lookahead = lookahead; }
        if (*p).mn_lookahead > lookahead {
            (*p).mn_lookahead = lookahead;
            (*p).mn_action = action;
        }
    }
    unsafe {
        (*(*p).a_lookahead.offset((*p).n_lookahead as isize)).lookahead =
            lookahead
    };
    unsafe {
        (*(*p).a_lookahead.offset((*p).n_lookahead as isize)).action = action
    };
    { let __p = &mut (*p).n_lookahead; let __t = *__p; *__p += 1; __t };
}

#[unsafe(no_mangle)]
pub extern "C" fn acttab_insert(p: &mut Acttab, make_it_safe_1: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut end: i32 = 0;
    if !((*p).n_lookahead > 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"acttab_insert".as_ptr() as *const i8,
                c"lemon.c".as_ptr() as *mut i8 as *const i8, 762,
                c"p->nLookahead>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    n = (*p).nsymbol + 1;
    if (*p).n_action + n >= (*p).n_action_alloc {
        let old_alloc: i32 = (*p).n_action_alloc;
        (*p).n_action_alloc = (*p).n_action + n + (*p).n_action_alloc + 20;
        (*p).a_action =
            lemon_realloc((*p).a_action as *mut (),
                    core::mem::size_of::<LookaheadAction>() as u64 *
                        (*p).n_action_alloc as u64) as *mut LookaheadAction;
        if (*p).a_action == core::ptr::null_mut() {
            eprintln!("malloc failed");
            unsafe { exit(1) };
        }
        {
            i = old_alloc;
            '__b48: loop {
                if !(i < (*p).n_action_alloc) { break '__b48; }
                '__c48: loop {
                    unsafe {
                        (*(*p).a_action.offset(i as isize)).lookahead = -1
                    };
                    unsafe { (*(*p).a_action.offset(i as isize)).action = -1 };
                    break '__c48;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    end = if make_it_safe_1 != 0 { (*p).mn_lookahead } else { 0 };
    {
        i = (*p).n_action - 1;
        '__b49: loop {
            if !(i >= end) { break '__b49; }
            '__c49: loop {
                if unsafe { (*(*p).a_action.offset(i as isize)).lookahead } ==
                        (*p).mn_lookahead {
                    if unsafe { (*(*p).a_action.offset(i as isize)).action } !=
                            (*p).mn_action {
                        break '__c49;
                    }
                    {
                        j = 0;
                        '__b50: loop {
                            if !(j < (*p).n_lookahead) { break '__b50; }
                            '__c50: loop {
                                k =
                                    unsafe { (*(*p).a_lookahead.offset(j as isize)).lookahead }
                                            - (*p).mn_lookahead + i;
                                if k < 0 || k >= (*p).n_action { break '__b50; }
                                if unsafe {
                                            (*(*p).a_lookahead.offset(j as isize)).lookahead
                                        } !=
                                        unsafe { (*(*p).a_action.offset(k as isize)).lookahead } {
                                    break '__b50;
                                }
                                if unsafe { (*(*p).a_lookahead.offset(j as isize)).action }
                                        != unsafe { (*(*p).a_action.offset(k as isize)).action } {
                                    break '__b50;
                                }
                                break '__c50;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if j < (*p).n_lookahead { break '__c49; }
                    n = 0;
                    {
                        j = 0;
                        '__b51: loop {
                            if !(j < (*p).n_action) { break '__b51; }
                            '__c51: loop {
                                if unsafe { (*(*p).a_action.offset(j as isize)).lookahead }
                                        < 0 {
                                    break '__c51;
                                }
                                if unsafe { (*(*p).a_action.offset(j as isize)).lookahead }
                                        == j + (*p).mn_lookahead - i {
                                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                }
                                break '__c51;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n == (*p).n_lookahead { break '__b49; }
                }
                break '__c49;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    if i < end {
        i = if make_it_safe_1 != 0 { (*p).mn_lookahead } else { 0 };
        {
            '__b52: loop {
                if !(i < (*p).n_action_alloc - (*p).mx_lookahead) {
                    break '__b52;
                }
                '__c52: loop {
                    if unsafe { (*(*p).a_action.offset(i as isize)).lookahead }
                            < 0 {
                        {
                            j = 0;
                            '__b53: loop {
                                if !(j < (*p).n_lookahead) { break '__b53; }
                                '__c53: loop {
                                    k =
                                        unsafe { (*(*p).a_lookahead.offset(j as isize)).lookahead }
                                                - (*p).mn_lookahead + i;
                                    if k < 0 { break '__b53; }
                                    if unsafe { (*(*p).a_action.offset(k as isize)).lookahead }
                                            >= 0 {
                                        break '__b53;
                                    }
                                    break '__c53;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if j < (*p).n_lookahead { break '__c52; }
                        {
                            j = 0;
                            '__b54: loop {
                                if !(j < (*p).n_action) { break '__b54; }
                                '__c54: loop {
                                    if unsafe { (*(*p).a_action.offset(j as isize)).lookahead }
                                            == j + (*p).mn_lookahead - i {
                                        break '__b54;
                                    }
                                    break '__c54;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if j == (*p).n_action { break '__b52; }
                    }
                    break '__c52;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    {
        j = 0;
        '__b55: loop {
            if !(j < (*p).n_lookahead) { break '__b55; }
            '__c55: loop {
                k =
                    unsafe { (*(*p).a_lookahead.offset(j as isize)).lookahead }
                            - (*p).mn_lookahead + i;
                unsafe {
                    *(*p).a_action.offset(k as isize) =
                        unsafe { *(*p).a_lookahead.offset(j as isize) }
                };
                if k >= (*p).n_action { (*p).n_action = k + 1; }
                break '__c55;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    if make_it_safe_1 != 0 && i + (*p).nterminal >= (*p).n_action {
        (*p).n_action = i + (*p).nterminal + 1;
    }
    (*p).n_lookahead = 0;
    return i - (*p).mn_lookahead;
}

#[unsafe(no_mangle)]
pub extern "C" fn acttab_action_size(p: &Acttab) -> i32 {
    let mut n: i32 = (*p).n_action;
    while n > 0 &&
            unsafe { (*(*p).a_action.offset((n - 1) as isize)).lookahead } < 0
        {
        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    }
    return n;
}

#[unsafe(no_mangle)]
pub extern "C" fn same_symbol(a: *mut Symbol, b: *mut Symbol) -> i32 {
    let mut i: i32 = 0;
    if a == b { return 1; }
    if unsafe { (*a).type_ } != MULTITERMINAL { return 0; }
    if unsafe { (*b).type_ } != MULTITERMINAL { return 0; }
    if unsafe { (*a).nsubsym } != unsafe { (*b).nsubsym } { return 0; }
    {
        i = 0;
        '__b57: loop {
            if !(i < unsafe { (*a).nsubsym }) { break '__b57; }
            '__c57: loop {
                if unsafe { *unsafe { (*a).subsym.offset(i as isize) } } !=
                        unsafe { *unsafe { (*b).subsym.offset(i as isize) } } {
                    return 0;
                }
                break '__c57;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}

extern "C" fn resolve_conflict(apx: &mut Action, apy: &mut Action) -> i32 {
    unsafe {
        let mut spx: *const Symbol = core::ptr::null();
        let mut spy: *const Symbol = core::ptr::null();
        let mut errcnt: i32 = 0;
        if !((*apx).sp == (*apy).sp) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"resolve_conflict".as_ptr() as *const i8,
                    c"lemon.c".as_ptr() as *mut i8 as *const i8, 1325,
                    c"apx->sp==apy->sp".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if (*apx).type_ == SHIFT && (*apy).type_ == SHIFT {
            (*apy).type_ = SSCONFLICT;
            { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
        }
        if (*apx).type_ == SHIFT && (*apy).type_ == REDUCE {
            spx = (*apx).sp;
            spy = unsafe { (*(*apy).x.rp).precsym };
            if spy == core::ptr::null_mut() || unsafe { (*spx).prec } < 0 ||
                    unsafe { (*spy).prec } < 0 {
                (*apy).type_ = SRCONFLICT;
                { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
            } else if unsafe { (*spx).prec } > unsafe { (*spy).prec } {
                (*apy).type_ = RD_RESOLVED;
            } else if unsafe { (*spx).prec } < unsafe { (*spy).prec } {
                (*apx).type_ = SH_RESOLVED;
            } else if unsafe { (*spx).prec } == unsafe { (*spy).prec } &&
                    unsafe { (*spx).assoc } == RIGHT {
                (*apy).type_ = RD_RESOLVED;
            } else if unsafe { (*spx).prec } == unsafe { (*spy).prec } &&
                    unsafe { (*spx).assoc } == LEFT {
                (*apx).type_ = SH_RESOLVED;
            } else {
                if !(unsafe { (*spx).prec } == unsafe { (*spy).prec } &&
                                        unsafe { (*spx).assoc } == NONE) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"resolve_conflict".as_ptr() as *const i8,
                            c"lemon.c".as_ptr() as *mut i8 as *const i8, 1346,
                            c"spx->prec==spy->prec && spx->assoc==NONE".as_ptr() as
                                    *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
                (*apx).type_ = ERROR;
            }
        } else if (*apx).type_ == REDUCE && (*apy).type_ == REDUCE {
            spx = unsafe { (*(*apx).x.rp).precsym };
            spy = unsafe { (*(*apy).x.rp).precsym };
            if spx == core::ptr::null_mut() || spy == core::ptr::null_mut() ||
                            unsafe { (*spx).prec } < 0 || unsafe { (*spy).prec } < 0 ||
                    unsafe { (*spx).prec } == unsafe { (*spy).prec } {
                (*apy).type_ = RRCONFLICT;
                { let __p = &mut errcnt; let __t = *__p; *__p += 1; __t };
            } else if unsafe { (*spx).prec } > unsafe { (*spy).prec } {
                (*apy).type_ = RD_RESOLVED;
            } else if unsafe { (*spx).prec } < unsafe { (*spy).prec } {
                (*apx).type_ = RD_RESOLVED;
            }
        } else {
            if !((*apx).type_ == SH_RESOLVED || (*apx).type_ == RD_RESOLVED ||
                                                                (*apx).type_ == SSCONFLICT || (*apx).type_ == SRCONFLICT ||
                                                        (*apx).type_ == RRCONFLICT || (*apy).type_ == SH_RESOLVED ||
                                                (*apy).type_ == RD_RESOLVED || (*apy).type_ == SSCONFLICT ||
                                        (*apy).type_ == SRCONFLICT || (*apy).type_ == RRCONFLICT) as
                            i32 as i64 != 0 {
                unsafe {
                    __assert_rtn(c"resolve_conflict".as_ptr() as *const i8,
                        c"lemon.c".as_ptr() as *mut i8 as *const i8, 1373,
                        c"apx->type==SH_RESOLVED || apx->type==RD_RESOLVED || apx->type==SSCONFLICT || apx->type==SRCONFLICT || apx->type==RRCONFLICT || apy->type==SH_RESOLVED || apy->type==RD_RESOLVED || apy->type==SSCONFLICT || apy->type==SRCONFLICT || apy->type==RRCONFLICT".as_ptr()
                                as *mut i8 as *const i8)
                }
            } else { { let _ = 0; } };
        }
        return errcnt;
    }
}

static mut freelist: *mut Config = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn newconfig() -> *mut Config {
    return lemon_calloc(1 as u64, core::mem::size_of::<Config>() as u64) as
            *mut Config;
}

#[unsafe(no_mangle)]
pub extern "C" fn deleteconfig(old: *mut Config) -> () {
    unsafe { unsafe { (*old).next = freelist }; freelist = old; }
}

static mut n_define: i32 = 0;

static mut n_define_used: i32 = 0;

static mut az_define: *mut *mut i8 = core::ptr::null_mut();

static mut b_define_used: *mut i8 = core::ptr::null_mut();

extern "C" fn handle_d_option_1(mut z: *mut i8) -> () {
    unsafe {
        let mut paz: *mut *mut i8 = core::ptr::null_mut();
        { let __p = &mut n_define; let __t = *__p; *__p += 1; __t };
        az_define =
            lemon_realloc(az_define as *mut (),
                    core::mem::size_of::<*mut i8>() as u64 * n_define as u64) as
                *mut *mut i8;
        if az_define == core::ptr::null_mut() {
            eprintln!("out of memory");
            unsafe { exit(1) };
        }
        b_define_used =
            lemon_realloc(b_define_used as *mut (), n_define as u64) as
                *mut i8;
        if b_define_used == core::ptr::null_mut() {
            eprintln!("out of memory");
            unsafe { exit(1) };
        }
        unsafe { *b_define_used.offset((n_define - 1) as isize) = 0 as i8 };
        paz = unsafe { az_define.offset((n_define - 1) as isize) };
        unsafe {
            *paz =
                lemon_malloc((unsafe { strlen(z as *const i8) } as i32 + 1) as
                            u64) as *mut i8
        };
        if unsafe { *paz } == core::ptr::null_mut() {
            eprintln!("out of memory");
            unsafe { exit(1) };
        }
        lemon_strcpy(unsafe { *paz }, z as *const i8);
        {
            z = unsafe { *paz };
            '__b58: loop {
                if !(unsafe { *z } != 0 && unsafe { *z } as i32 != '=' as i32)
                    {
                    break '__b58;
                }
                '__c58: loop { break '__c58; }
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        unsafe { *z = 0 as i8 };
    }
}

extern "C" fn handle_u_option(z: *const i8) -> () {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b59: loop {
                if !(i < n_define) { break '__b59; }
                '__c59: loop {
                    if unsafe {
                                strcmp(unsafe { *az_define.offset(i as isize) } as
                                        *const i8, z as *const i8)
                            } == 0 {
                        { let __p = &mut n_define; let __t = *__p; *__p -= 1; __t };
                        if i < n_define {
                            unsafe {
                                *az_define.offset(i as isize) =
                                    unsafe { *az_define.offset(n_define as isize) }
                            };
                            unsafe {
                                *b_define_used.offset(i as isize) =
                                    unsafe { *b_define_used.offset(n_define as isize) }
                            };
                        }
                        break '__b59;
                    }
                    break '__c59;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

static mut output_dir: *mut i8 = 0 as *mut () as *mut i8;

extern "C" fn handle_d_option(z: *const i8) -> () {
    unsafe {
        output_dir =
            lemon_malloc((unsafe { strlen(z as *const i8) } as i32 + 1) as
                        u64) as *mut i8;
        if output_dir == core::ptr::null_mut() {
            eprintln!("out of memory");
            unsafe { exit(1) };
        }
        lemon_strcpy(output_dir, z as *const i8);
    }
}

static mut user_templatename: *mut i8 = 0 as *mut () as *mut i8;

extern "C" fn rule_merge(mut p_a_1: *mut Rule, mut p_b_1: *mut Rule)
    -> *mut Rule {
    let mut p_first: *mut Rule = core::ptr::null_mut();
    let mut pp_prev: *mut *mut Rule = &mut p_first;
    while !(p_a_1).is_null() && !(p_b_1).is_null() {
        if unsafe { (*p_a_1).i_rule } < unsafe { (*p_b_1).i_rule } {
            unsafe { *pp_prev = p_a_1 };
            pp_prev = unsafe { &mut (*p_a_1).next };
            p_a_1 = unsafe { (*p_a_1).next };
        } else {
            unsafe { *pp_prev = p_b_1 };
            pp_prev = unsafe { &mut (*p_b_1).next };
            p_b_1 = unsafe { (*p_b_1).next };
        }
    }
    if !(p_a_1).is_null() {
        unsafe { *pp_prev = p_a_1 };
    } else { unsafe { *pp_prev = p_b_1 }; }
    return p_first;
}

extern "C" fn rule_sort(mut rp: *mut Rule) -> *mut Rule {
    let mut i: u32 = 0 as u32;
    let mut p_next: *mut Rule = core::ptr::null_mut();
    let mut x: [*mut Rule; 32] = [core::ptr::null_mut(); 32];
    unsafe {
        memset(&raw mut x[0 as usize] as *mut *mut Rule as *mut (), 0,
            core::mem::size_of::<[*mut Rule; 32]>() as u64)
    };
    while !(rp).is_null() {
        p_next = unsafe { (*rp).next };
        unsafe { (*rp).next = core::ptr::null_mut() };
        {
            i = 0 as u32;
            '__b62: loop {
                if !((i as u64) <
                                    core::mem::size_of::<[*mut Rule; 32]>() as u64 /
                                            core::mem::size_of::<*mut Rule>() as u64 - 1 as u64 &&
                                !(x[i as usize]).is_null()) {
                    break '__b62;
                }
                '__c62: loop {
                    rp = rule_merge(x[i as usize], rp);
                    x[i as usize] = core::ptr::null_mut();
                    break '__c62;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        x[i as usize] = rp;
        rp = p_next;
    }
    rp = core::ptr::null_mut();
    {
        i = 0 as u32;
        '__b63: loop {
            if !((i as u64) <
                            core::mem::size_of::<[*mut Rule; 32]>() as u64 /
                                core::mem::size_of::<*mut Rule>() as u64) {
                break '__b63;
            }
            '__c63: loop { rp = rule_merge(x[i as usize], rp); break '__c63; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return rp;
}

extern "C" fn minimum_size_type(lwr: i32, upr: i32, pn_byte: *mut i32)
    -> *const i8 {
    let mut z_type: *const i8 = c"int".as_ptr() as *mut i8 as *const i8;
    let mut n_byte: i32 = 4;
    if lwr >= 0 {
        if upr <= 255 {
            z_type = c"unsigned char".as_ptr() as *mut i8 as *const i8;
            n_byte = 1;
        } else if upr < 65535 {
            z_type = c"unsigned short int".as_ptr() as *mut i8 as *const i8;
            n_byte = 2;
        } else {
            z_type = c"unsigned int".as_ptr() as *mut i8 as *const i8;
            n_byte = 4;
        }
    } else if lwr >= -127 && upr <= 127 {
        z_type = c"signed char".as_ptr() as *mut i8 as *const i8;
        n_byte = 1;
    } else if lwr >= -32767 && upr < 32767 {
        z_type = c"short".as_ptr() as *mut i8 as *const i8;
        n_byte = 2;
    }
    if !(pn_byte).is_null() { unsafe { *pn_byte = n_byte }; }
    return z_type;
}

extern "C" fn stats_line(z_label_1: *const i8, i_value_1: i32) -> () {
    let n_label: i32 = unsafe { strlen(z_label_1) } as i32;
    unsafe {
        printf(c"  %s%.*s %5d\n".as_ptr() as *mut i8 as *const i8, z_label_1,
            35 - n_label,
            c"................................".as_ptr() as *mut i8,
            i_value_1)
    };
}

extern "C" fn define_cmp(p_a_1: *const (), p_b_1: *const ()) -> i32 {
    let z_a: *const i8 = unsafe { *(p_a_1 as *mut *const i8) };
    let z_b: *const i8 = unsafe { *(p_b_1 as *mut *const i8) };
    return unsafe { strcmp(z_a, z_b) };
}

const INITIALIZE: u32 = 0;

const WAITING_FOR_DECL_OR_RULE: u32 = 1;

const WAITING_FOR_DECL_KEYWORD: u32 = 2;

const WAITING_FOR_DECL_ARG: u32 = 3;

const WAITING_FOR_PRECEDENCE_SYMBOL: u32 = 4;

const WAITING_FOR_ARROW: u32 = 5;

const IN_RHS: u32 = 6;

const LHS_ALIAS_1: u32 = 7;

const LHS_ALIAS_2: u32 = 8;

const LHS_ALIAS_3: u32 = 9;

const RHS_ALIAS_1: u32 = 10;

const RHS_ALIAS_2: u32 = 11;

const PRECEDENCE_MARK_1: u32 = 12;

const PRECEDENCE_MARK_2: u32 = 13;

const RESYNC_AFTER_RULE_ERROR: u32 = 14;

const RESYNC_AFTER_DECL_ERROR: u32 = 15;

const WAITING_FOR_DESTRUCTOR_SYMBOL: u32 = 16;

const WAITING_FOR_DATATYPE_SYMBOL: u32 = 17;

const WAITING_FOR_FALLBACK_ID: u32 = 18;

const WAITING_FOR_WILDCARD_ID: u32 = 19;

const WAITING_FOR_CLASS_ID: u32 = 20;

const WAITING_FOR_CLASS_TOKEN: u32 = 21;

const WAITING_FOR_TOKEN_NAME: u32 = 22;

#[repr(C)]
#[derive(Copy, Clone)]
struct Pstate {
    filename: *mut i8,
    tokenlineno: i32,
    errorcnt: i32,
    tokenstart: *mut i8,
    gp: *mut Lemon,
    state: u32,
    fallback: *mut Symbol,
    tkclass: *mut Symbol,
    lhs: *mut Symbol,
    lhsalias: *const i8,
    nrhs: i32,
    rhs: [*mut Symbol; 1000],
    alias: [*const i8; 1000],
    prevrule: *mut Rule,
    declkeyword: *const i8,
    declargslot: *mut *mut i8,
    insert_line_macro: i32,
    decllinenoslot: *mut i32,
    declassoc: u32,
    preccounter: i32,
    firstrule: *mut Rule,
    lastrule: *mut Rule,
}

extern "C" fn eval_preprocessor_boolean(z: *mut i8, lineno: i32) -> i32 {
    unsafe {
        let mut neg: i32 = 0;
        let mut res: i32 = 0;
        let mut ok_term: i32 = 0;
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        let mut n: i32 = 0;
        let mut j: i32 = 0;
        let mut k__1: i32 = 0;
        let mut n__1: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s65:
                {
                match __state {
                    0 => { neg = 0; __state = 3; }
                    2 => {
                        if lineno > 0 { __state = 89; } else { __state = 90; }
                    }
                    3 => { res = 0; __state = 4; }
                    4 => { ok_term = 1; __state = 5; }
                    5 => { __state = 6; }
                    6 => { i = 0; __state = 8; }
                    7 => { return res; }
                    8 => {
                        if unsafe { *z.offset(i as isize) } as i32 != 0 {
                            __state = 9;
                        } else { __state = 7; }
                    }
                    9 => {
                        if unsafe {
                                    isspace(unsafe { *z.offset(i as isize) } as u8 as i32)
                                } != 0 {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    10 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 8;
                    }
                    11 => {
                        if unsafe { *z.offset(i as isize) } as i32 == '!' as i32 {
                            __state = 14;
                        } else { __state = 13; }
                    }
                    12 => { __state = 10; }
                    13 => {
                        if unsafe { *z.offset(i as isize) } as i32 == '|' as i32 &&
                                unsafe { *z.offset((i + 1) as isize) } as i32 == '|' as i32
                            {
                            __state = 19;
                        } else { __state = 18; }
                    }
                    14 => {
                        if (ok_term == 0) as i32 != 0 {
                            __state = 16;
                        } else { __state = 15; }
                    }
                    15 => { neg = (neg == 0) as i32 as i32; __state = 17; }
                    16 => { __state = 2; }
                    17 => { __state = 10; }
                    18 => {
                        if unsafe { *z.offset(i as isize) } as i32 == '&' as i32 &&
                                unsafe { *z.offset((i + 1) as isize) } as i32 == '&' as i32
                            {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    19 => {
                        if ok_term != 0 { __state = 21; } else { __state = 20; }
                    }
                    20 => {
                        if res != 0 { __state = 23; } else { __state = 22; }
                    }
                    21 => { __state = 2; }
                    22 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 24;
                    }
                    23 => { return 1; }
                    24 => { ok_term = 1; __state = 25; }
                    25 => { __state = 10; }
                    26 => {
                        if unsafe { *z.offset(i as isize) } as i32 == '(' as i32 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    27 => {
                        if ok_term != 0 { __state = 29; } else { __state = 28; }
                    }
                    28 => {
                        if (res == 0) as i32 != 0 {
                            __state = 31;
                        } else { __state = 30; }
                    }
                    29 => { __state = 2; }
                    30 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 32;
                    }
                    31 => { return 0; }
                    32 => { ok_term = 1; __state = 33; }
                    33 => { __state = 10; }
                    34 => {
                        if unsafe {
                                    isalpha(unsafe { *z.offset(i as isize) } as u8 as i32)
                                } != 0 {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    35 => { __state = 36; }
                    36 => { n = 1; __state = 37; }
                    37 => {
                        if (ok_term == 0) as i32 != 0 {
                            __state = 39;
                        } else { __state = 38; }
                    }
                    38 => { k = i + 1; __state = 41; }
                    39 => { __state = 2; }
                    40 => {
                        if neg != 0 { __state = 60; } else { __state = 59; }
                    }
                    41 => {
                        if unsafe { *z.offset(k as isize) } != 0 {
                            __state = 42;
                        } else { __state = 40; }
                    }
                    42 => {
                        if unsafe { *z.offset(k as isize) } as i32 == ')' as i32 {
                            __state = 44;
                        } else { __state = 45; }
                    }
                    43 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 41;
                    }
                    44 => {
                        { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        __state = 46;
                    }
                    45 => {
                        if unsafe { *z.offset(k as isize) } as i32 == '(' as i32 {
                            __state = 55;
                        } else { __state = 56; }
                    }
                    46 => { if n == 0 { __state = 47; } else { __state = 43; } }
                    47 => {
                        unsafe { *z.offset(k as isize) = 0 as i8 };
                        __state = 48;
                    }
                    48 => {
                        res =
                            eval_preprocessor_boolean(unsafe {
                                    &mut *z.offset((i + 1) as isize)
                                }, -1);
                        __state = 49;
                    }
                    49 => {
                        unsafe { *z.offset(k as isize) = ')' as i32 as i8 };
                        __state = 50;
                    }
                    50 => {
                        if res < 0 { __state = 52; } else { __state = 51; }
                    }
                    51 => { i = k; __state = 54; }
                    52 => { i = i - res; __state = 53; }
                    53 => { __state = 2; }
                    54 => { __state = 40; }
                    55 => {
                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        __state = 43;
                    }
                    56 => {
                        if unsafe { *z.offset(k as isize) } as i32 == 0 {
                            __state = 57;
                        } else { __state = 43; }
                    }
                    57 => { i = k; __state = 58; }
                    58 => { __state = 2; }
                    59 => { ok_term = 0; __state = 62; }
                    60 => { res = (res == 0) as i32 as i32; __state = 61; }
                    61 => { neg = 0; __state = 59; }
                    62 => { __state = 10; }
                    63 => { __state = 2; }
                    64 => { __state = 65; }
                    65 => {
                        if (ok_term == 0) as i32 != 0 {
                            __state = 67;
                        } else { __state = 66; }
                    }
                    66 => { k__1 = i + 1; __state = 69; }
                    67 => { __state = 2; }
                    68 => { n__1 = k__1 - i; __state = 72; }
                    69 => {
                        if unsafe {
                                        isalnum(unsafe { *z.offset(k__1 as isize) } as u8 as i32)
                                    } != 0 ||
                                unsafe { *z.offset(k__1 as isize) } as i32 == '_' as i32 {
                            __state = 70;
                        } else { __state = 68; }
                    }
                    70 => { __state = 71; }
                    71 => {
                        { let __p = &mut k__1; let __t = *__p; *__p += 1; __t };
                        __state = 69;
                    }
                    72 => { res = 0; __state = 73; }
                    73 => { j = 0; __state = 75; }
                    74 => { i = k__1 - 1; __state = 83; }
                    75 => {
                        if j < n_define { __state = 76; } else { __state = 74; }
                    }
                    76 => {
                        if unsafe {
                                        strncmp(unsafe { *az_define.offset(j as isize) } as
                                                *const i8,
                                            unsafe { &raw mut *z.offset(i as isize) } as *const i8,
                                            n__1 as u64)
                                    } == 0 &&
                                unsafe {
                                            *unsafe {
                                                    (*az_define.offset(j as isize)).offset(n__1 as isize)
                                                }
                                        } as i32 == 0 {
                            __state = 78;
                        } else { __state = 77; }
                    }
                    77 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 75;
                    }
                    78 => {
                        if (unsafe { *b_define_used.offset(j as isize) } == 0) as
                                    i32 != 0 {
                            __state = 80;
                        } else { __state = 79; }
                    }
                    79 => { res = 1; __state = 82; }
                    80 => {
                        unsafe { *b_define_used.offset(j as isize) = 1 as i8 };
                        __state = 81;
                    }
                    81 => {
                        {
                            let __p = &mut n_define_used;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 79;
                    }
                    82 => { __state = 74; }
                    83 => {
                        if neg != 0 { __state = 85; } else { __state = 84; }
                    }
                    84 => { ok_term = 0; __state = 87; }
                    85 => { res = (res == 0) as i32 as i32; __state = 86; }
                    86 => { neg = 0; __state = 84; }
                    87 => { __state = 10; }
                    88 => { __state = 2; }
                    89 => {
                        eprintln!("%if syntax error on line {}.", lineno as i32);
                        __state = 91;
                    }
                    90 => { return -(i + 1); }
                    91 => {
                        unsafe {
                            fprintf(__stderrp,
                                c"  %.*s <-- syntax error here\n".as_ptr() as *mut i8 as
                                    *const i8, i + 1, z)
                        };
                        __state = 92;
                    }
                    92 => { unsafe { exit(1) }; __state = 1; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}

extern "C" fn preprocess_input(z: *mut i8) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut exclude: i32 = 0;
    let mut start: i32 = 0;
    let mut lineno: i32 = 1;
    let mut start_lineno: i32 = 1;
    {
        i = 0;
        '__b66: loop {
            if !(unsafe { *z.offset(i as isize) } != 0) { break '__b66; }
            '__c66: loop {
                if unsafe { *z.offset(i as isize) } as i32 == '\n' as i32 {
                    { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
                }
                if unsafe { *z.offset(i as isize) } as i32 != '%' as i32 ||
                        i > 0 &&
                            unsafe { *z.offset((i - 1) as isize) } as i32 != '\n' as i32
                    {
                    break '__c66;
                }
                if unsafe {
                                strncmp(unsafe { &raw mut *z.offset(i as isize) } as
                                        *const i8, c"%endif".as_ptr() as *mut i8 as *const i8,
                                    6 as u64)
                            } == 0 &&
                        unsafe {
                                isspace(unsafe { *z.offset((i + 6) as isize) } as u8 as i32)
                            } != 0 {
                    if exclude != 0 {
                        { let __p = &mut exclude; let __t = *__p; *__p -= 1; __t };
                        if exclude == 0 {
                            {
                                j = start;
                                '__b67: loop {
                                    if !(j < i) { break '__b67; }
                                    '__c67: loop {
                                        if unsafe { *z.offset(j as isize) } as i32 != '\n' as i32 {
                                            unsafe { *z.offset(j as isize) = ' ' as i32 as i8 };
                                        }
                                        break '__c67;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                    }
                    {
                        j = i;
                        '__b68: loop {
                            if !(unsafe { *z.offset(j as isize) } != 0 &&
                                            unsafe { *z.offset(j as isize) } as i32 != '\n' as i32) {
                                break '__b68;
                            }
                            '__c68: loop {
                                unsafe { *z.offset(j as isize) = ' ' as i32 as i8 };
                                break '__c68;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                } else if unsafe {
                                strncmp(unsafe { &raw mut *z.offset(i as isize) } as
                                        *const i8, c"%else".as_ptr() as *mut i8 as *const i8,
                                    5 as u64)
                            } == 0 &&
                        unsafe {
                                isspace(unsafe { *z.offset((i + 5) as isize) } as u8 as i32)
                            } != 0 {
                    if exclude == 1 {
                        exclude = 0;
                        {
                            j = start;
                            '__b69: loop {
                                if !(j < i) { break '__b69; }
                                '__c69: loop {
                                    if unsafe { *z.offset(j as isize) } as i32 != '\n' as i32 {
                                        unsafe { *z.offset(j as isize) = ' ' as i32 as i8 };
                                    }
                                    break '__c69;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    } else if exclude == 0 {
                        exclude = 1;
                        start = i;
                        start_lineno = lineno;
                    }
                    {
                        j = i;
                        '__b70: loop {
                            if !(unsafe { *z.offset(j as isize) } != 0 &&
                                            unsafe { *z.offset(j as isize) } as i32 != '\n' as i32) {
                                break '__b70;
                            }
                            '__c70: loop {
                                unsafe { *z.offset(j as isize) = ' ' as i32 as i8 };
                                break '__c70;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                } else if unsafe {
                                    strncmp(unsafe { &raw mut *z.offset(i as isize) } as
                                            *const i8, c"%ifdef ".as_ptr() as *mut i8 as *const i8,
                                        7 as u64)
                                } == 0 ||
                            unsafe {
                                    strncmp(unsafe { &raw mut *z.offset(i as isize) } as
                                            *const i8, c"%if ".as_ptr() as *mut i8 as *const i8,
                                        4 as u64)
                                } == 0 ||
                        unsafe {
                                strncmp(unsafe { &raw mut *z.offset(i as isize) } as
                                        *const i8, c"%ifndef ".as_ptr() as *mut i8 as *const i8,
                                    8 as u64)
                            } == 0 {
                    if exclude != 0 {
                        { let __p = &mut exclude; let __t = *__p; *__p += 1; __t };
                    } else {
                        let mut is_not: i32 = 0;
                        let mut i_bool: i32 = 0;
                        {
                            j = i;
                            '__b71: loop {
                                if !(unsafe { *z.offset(j as isize) } != 0 &&
                                                (unsafe {
                                                                isspace(unsafe { *z.offset(j as isize) } as u8 as i32)
                                                            } == 0) as i32 != 0) {
                                    break '__b71;
                                }
                                '__c71: loop { break '__c71; }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        i_bool = j;
                        is_not = (j == i + 7) as i32;
                        while unsafe { *z.offset(j as isize) } != 0 &&
                                unsafe { *z.offset(j as isize) } as i32 != '\n' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        k = unsafe { *z.offset(j as isize) } as i32;
                        unsafe { *z.offset(j as isize) = 0 as i8 };
                        exclude =
                            eval_preprocessor_boolean(unsafe {
                                    &mut *z.offset(i_bool as isize)
                                }, lineno);
                        unsafe { *z.offset(j as isize) = k as i8 };
                        if (is_not == 0) as i32 != 0 {
                            exclude = (exclude == 0) as i32 as i32;
                        }
                        if exclude != 0 { start = i; start_lineno = lineno; }
                    }
                    {
                        j = i;
                        '__b73: loop {
                            if !(unsafe { *z.offset(j as isize) } != 0 &&
                                            unsafe { *z.offset(j as isize) } as i32 != '\n' as i32) {
                                break '__b73;
                            }
                            '__c73: loop {
                                unsafe { *z.offset(j as isize) = ' ' as i32 as i8 };
                                break '__c73;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                break '__c66;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if exclude != 0 {
        eprintln!("unterminated %ifdef starting on line {}", start_lineno as i32);
        unsafe { exit(1) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_makename(lemp: &Lemon, suffix: *const i8) -> *mut i8 {
    unsafe {
        let mut name: *mut i8 = core::ptr::null_mut();
        let mut cp: *mut i8 = core::ptr::null_mut();
        let mut filename: *const i8 = (*lemp).filename as *const i8;
        let mut sz: i32 = 0;
        if !(output_dir).is_null() {
            cp = unsafe { strrchr(filename as *const i8, '/' as i32) };
            if !(cp).is_null() {
                filename = unsafe { cp.offset(1 as isize) };
            }
        }
        sz = unsafe { strlen(filename as *const i8) } as i32;
        sz += unsafe { strlen(suffix) } as i32;
        if !(output_dir).is_null() {
            sz += unsafe { strlen(output_dir as *const i8) } as i32 + 1;
        }
        sz += 5;
        name = lemon_malloc(sz as u64) as *mut i8;
        if name == core::ptr::null_mut() {
            eprintln!("Can't allocate space for a filename.");
            unsafe { exit(1) };
        }
        unsafe { *name.offset(0 as isize) = 0 as i8 };
        if !(output_dir).is_null() {
            lemon_strcpy(name, output_dir as *const i8);
            lemon_strcat(name, c"/".as_ptr() as *mut i8 as *const i8);
        }
        lemon_strcat(name, filename as *const i8);
        cp = unsafe { strrchr(name as *const i8, '.' as i32) };
        if !(cp).is_null() { unsafe { *cp = 0 as i8 }; }
        lemon_strcat(name, suffix);
        return name;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn file_open(lemp: *mut Lemon, suffix: *const i8,
    mode: *const i8) -> *mut FILE {
    unsafe {
        let mut fp: *mut FILE = core::ptr::null_mut();
        if !(unsafe { (*lemp).outname }).is_null() {
            lemon_free(unsafe { (*lemp).outname } as *mut ());
        }
        unsafe { (*lemp).outname = file_makename(unsafe { &*lemp }, suffix) };
        fp = unsafe { fopen(unsafe { (*lemp).outname } as *const i8, mode) };
        if fp == core::ptr::null_mut() &&
                unsafe { *mode } as i32 == 'w' as i32 {
            unsafe {
                fprintf(__stderrp,
                    c"Can\'t open file \"%s\".\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*lemp).outname })
            };
            {
                let __p = unsafe { &mut (*lemp).errorcnt };
                let __t = *__p;
                *__p += 1;
                __t
            };
            return core::ptr::null_mut();
        }
        return fp;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rule_print(out: *mut FILE, rp: &Rule) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    unsafe {
        fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8,
            unsafe { (*(*rp).lhs).name })
    };
    unsafe { fprintf(out, c" ::=".as_ptr() as *mut i8 as *const i8) };
    {
        i = 0;
        '__b74: loop {
            if !(i < (*rp).nrhs) { break '__b74; }
            '__c74: loop {
                let sp: *const Symbol =
                    unsafe { *(*rp).rhs.offset(i as isize) } as *const Symbol;
                if unsafe { (*sp).type_ } == MULTITERMINAL {
                    unsafe {
                        fprintf(out, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                (*unsafe {
                                                *unsafe { (*sp).subsym.offset(0 as isize) }
                                            }).name
                            })
                    };
                    {
                        j = 1;
                        '__b75: loop {
                            if !(j < unsafe { (*sp).nsubsym }) { break '__b75; }
                            '__c75: loop {
                                unsafe {
                                    fprintf(out, c"|%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe {
                                            (*unsafe {
                                                            *unsafe { (*sp).subsym.offset(j as isize) }
                                                        }).name
                                        })
                                };
                                break '__c75;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                } else {
                    unsafe {
                        fprintf(out, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*sp).name })
                    };
                }
                break '__c74;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RulePrint(fp: *mut FILE, rp: &Rule, i_cursor_1: i32) -> () {
    let mut sp: *const Symbol = core::ptr::null();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    unsafe {
        fprintf(fp, c"%s ::=".as_ptr() as *mut i8 as *const i8,
            unsafe { (*(*rp).lhs).name })
    };
    {
        i = 0;
        '__b76: loop {
            if !(i <= (*rp).nrhs) { break '__b76; }
            '__c76: loop {
                if i == i_cursor_1 {
                    unsafe {
                        fprintf(fp, c" *".as_ptr() as *mut i8 as *const i8)
                    };
                }
                if i == (*rp).nrhs { break '__b76; }
                sp = unsafe { *(*rp).rhs.offset(i as isize) };
                if unsafe { (*sp).type_ } == MULTITERMINAL {
                    unsafe {
                        fprintf(fp, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                (*unsafe {
                                                *unsafe { (*sp).subsym.offset(0 as isize) }
                                            }).name
                            })
                    };
                    {
                        j = 1;
                        '__b77: loop {
                            if !(j < unsafe { (*sp).nsubsym }) { break '__b77; }
                            '__c77: loop {
                                unsafe {
                                    fprintf(fp, c"|%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe {
                                            (*unsafe {
                                                            *unsafe { (*sp).subsym.offset(j as isize) }
                                                        }).name
                                        })
                                };
                                break '__c77;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                } else {
                    unsafe {
                        fprintf(fp, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*sp).name })
                    };
                }
                break '__c76;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn config_print(fp: *mut FILE, cfp: &Config) -> () {
    unsafe { RulePrint(fp, unsafe { &*(*cfp).rp }, (*cfp).dot); }
}

#[unsafe(no_mangle)]
pub extern "C" fn pathsearch(argv0: *mut i8, name: *mut i8, modemask: i32)
    -> *mut i8 {
    let mut pathlist: *const i8 = core::ptr::null();
    let mut pathbufptr: *mut i8 = core::ptr::null_mut();
    let mut pathbuf: *mut i8 = core::ptr::null_mut();
    let mut path: *mut i8 = core::ptr::null_mut();
    let mut cp: *mut i8 = core::ptr::null_mut();
    let mut c: i8 = 0 as i8;
    cp = unsafe { strrchr(argv0 as *const i8, '/' as i32) };
    if !(cp).is_null() {
        c = unsafe { *cp };
        unsafe { *cp = 0 as i8 };
        path =
            lemon_malloc((unsafe { strlen(argv0 as *const i8) } as i32 +
                                unsafe { strlen(name as *const i8) } as i32 + 2) as u64) as
                *mut i8;
        if !(path).is_null() {
            unsafe {
                lemon_sprintf(path, c"%s/%s".as_ptr() as *mut i8 as *const i8,
                    argv0, name)
            };
        }
        unsafe { *cp = c };
    } else {
        pathlist =
            unsafe { getenv(c"PATH".as_ptr() as *mut i8 as *const i8) } as
                *const i8;
        if pathlist == core::ptr::null() {
            pathlist = c".:/bin:/usr/bin".as_ptr() as *mut i8 as *const i8;
        }
        pathbuf =
            lemon_malloc((unsafe { strlen(pathlist) } as i32 + 1) as u64) as
                *mut i8;
        path =
            lemon_malloc((unsafe { strlen(pathlist) } as i32 +
                                unsafe { strlen(name as *const i8) } as i32 + 2) as u64) as
                *mut i8;
        if pathbuf != core::ptr::null_mut() && path != core::ptr::null_mut() {
            pathbufptr = pathbuf;
            lemon_strcpy(pathbuf, pathlist);
            while unsafe { *pathbuf } != 0 {
                cp = unsafe { strchr(pathbuf as *const i8, ':' as i32) };
                if cp == core::ptr::null_mut() {
                    cp =
                        unsafe {
                            pathbuf.offset(unsafe { strlen(pathbuf as *const i8) } as
                                        i32 as isize)
                        };
                }
                c = unsafe { *cp };
                unsafe { *cp = 0 as i8 };
                unsafe {
                    lemon_sprintf(path,
                        c"%s/%s".as_ptr() as *mut i8 as *const i8, pathbuf, name)
                };
                unsafe { *cp = c };
                if c as i32 == 0 {
                    unsafe { *pathbuf.offset(0 as isize) = 0 as i8 };
                } else { pathbuf = unsafe { cp.offset(1 as isize) }; }
                if unsafe { access(path as *const i8, modemask) } == 0 {
                    break;
                }
            }
        }
        lemon_free(pathbufptr as *mut ());
    }
    return path;
}

#[unsafe(no_mangle)]
pub extern "C" fn tplt_xfer(name: *mut i8, in__1: *mut FILE, out: *mut FILE,
    lineno: &mut i32) -> () {
    let mut i: i32 = 0;
    let mut i_start: i32 = 0;
    let mut line: [i8; 10000] = [0; 10000];
    while !(unsafe {
                            fgets(&raw mut line[0 as usize] as *mut i8, 10000, in__1)
                        }).is_null() &&
            (line[0 as usize] as i32 != '%' as i32 ||
                line[1 as usize] as i32 != '%' as i32) {
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
        i_start = 0;
        if !(name).is_null() {
            {
                i = 0;
                '__b80: loop {
                    if !(line[i as usize] != 0) { break '__b80; }
                    '__c80: loop {
                        if line[i as usize] as i32 == 'P' as i32 &&
                                    unsafe {
                                            strncmp(&raw mut line[i as usize] as *const i8,
                                                c"Parse".as_ptr() as *mut i8 as *const i8, 5 as u64)
                                        } == 0 &&
                                (i == 0 ||
                                    (unsafe { isalpha(line[(i - 1) as usize] as u8 as i32) } ==
                                                0) as i32 != 0) {
                            if i > i_start {
                                unsafe {
                                    fprintf(out, c"%.*s".as_ptr() as *mut i8 as *const i8,
                                        i - i_start, &raw mut line[i_start as usize] as *mut i8)
                                };
                            }
                            unsafe {
                                fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8, name)
                            };
                            i += 4;
                            i_start = i + 1;
                        }
                        break '__c80;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        unsafe {
            fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8,
                &raw mut line[i_start as usize] as *mut i8)
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tplt_skip_header(in__1: *mut FILE) -> () {
    let mut line: [i8; 10000] = [0; 10000];
    while !(unsafe {
                            fgets(&raw mut line[0 as usize] as *mut i8, 10000, in__1)
                        }).is_null() &&
            (line[0 as usize] as i32 != '%' as i32 ||
                line[1 as usize] as i32 != '%' as i32) {}
}

#[unsafe(no_mangle)]
pub extern "C" fn tplt_open(lemp: &mut Lemon) -> *mut FILE {
    unsafe {
        let mut buf: [i8; 1000] = [0; 1000];
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut tpltname: *mut i8 = core::ptr::null_mut();
        let mut to_free: *mut i8 = core::ptr::null_mut();
        let mut cp: *mut i8 = core::ptr::null_mut();
        if user_templatename != core::ptr::null_mut() {
            if unsafe { access(user_templatename as *const i8, 4) } == -1 {
                unsafe {
                    fprintf(__stderrp,
                        c"Can\'t find the parser driver template file \"%s\".\n".as_ptr()
                                as *mut i8 as *const i8, user_templatename)
                };
                {
                    let __p = &mut (*lemp).errorcnt;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                return core::ptr::null_mut();
            }
            in_ =
                unsafe {
                    fopen(user_templatename as *const i8,
                        c"rb".as_ptr() as *mut i8 as *const i8)
                };
            if in_ == core::ptr::null_mut() {
                unsafe {
                    fprintf(__stderrp,
                        c"Can\'t open the template file \"%s\".\n".as_ptr() as
                                *mut i8 as *const i8, user_templatename)
                };
                {
                    let __p = &mut (*lemp).errorcnt;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                return core::ptr::null_mut();
            }
            return in_;
        }
        cp = unsafe { strrchr((*lemp).filename as *const i8, '.' as i32) };
        if !(cp).is_null() {
            unsafe {
                lemon_sprintf(&raw mut buf[0 as usize] as *mut i8,
                    c"%.*s.lt".as_ptr() as *mut i8 as *const i8,
                    unsafe { cp.offset_from((*lemp).filename) } as i64 as i32,
                    (*lemp).filename)
            };
        } else {
            unsafe {
                lemon_sprintf(&raw mut buf[0 as usize] as *mut i8,
                    c"%s.lt".as_ptr() as *mut i8 as *const i8, (*lemp).filename)
            };
        }
        if unsafe {
                    access(&raw mut buf[0 as usize] as *mut i8 as *const i8, 4)
                } == 0 {
            tpltname = &raw mut buf[0 as usize] as *mut i8;
        } else if unsafe {
                    access(&raw mut templatename[0 as usize] as *mut i8 as
                            *const i8, 4)
                } == 0 {
            tpltname = &raw mut templatename[0 as usize] as *mut i8;
        } else {
            to_free =
                {
                    tpltname =
                        pathsearch(unsafe { *(*lemp).argv.offset(0 as isize) },
                            &raw mut templatename[0 as usize] as *mut i8, 0);
                    tpltname
                };
        }
        if tpltname == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"Can\'t find the parser driver template file \"%s\".\n".as_ptr()
                            as *mut i8 as *const i8,
                    &raw mut templatename[0 as usize] as *mut i8)
            };
            {
                let __p = &mut (*lemp).errorcnt;
                let __t = *__p;
                *__p += 1;
                __t
            };
            return core::ptr::null_mut();
        }
        in_ =
            unsafe {
                fopen(tpltname as *const i8,
                    c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if in_ == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"Can\'t open the template file \"%s\".\n".as_ptr() as
                            *mut i8 as *const i8, tpltname)
            };
            {
                let __p = &mut (*lemp).errorcnt;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        lemon_free(to_free as *mut ());
        return in_;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tplt_linedir(out: *mut FILE, lineno: i32,
    mut filename: *const i8) -> () {
    unsafe {
        fprintf(out, c"#line %d \"".as_ptr() as *mut i8 as *const i8, lineno)
    };
    while unsafe { *filename } != 0 {
        if unsafe { *filename } as i32 == '\\' as i32 {
            unsafe { putc('\\' as i32, out) };
        }
        unsafe { putc(unsafe { *filename } as i32, out) };
        {
            let __p = &mut filename;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    unsafe { fprintf(out, c"\"\n".as_ptr() as *mut i8 as *const i8) };
    unsafe { fflush(out) };
}

#[unsafe(no_mangle)]
pub extern "C" fn tplt_print(out: *mut FILE, lemp: &Lemon, mut str: *const i8,
    lineno: &mut i32) -> () {
    if str == core::ptr::null_mut() { return; }
    unsafe { fflush(out) };
    while unsafe { *str } != 0 {
        unsafe { putc(unsafe { *str } as i32, out) };
        if unsafe { *str } as i32 == '\n' as i32 {
            { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
        }
        {
            let __p = &mut str;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if unsafe { *str.offset(-1 as isize) } as i32 != '\n' as i32 {
        unsafe { putc('\n' as i32, out) };
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
    }
    if ((*lemp).nolinenosflag == 0) as i32 != 0 {
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
        tplt_linedir(out, *lineno, (*lemp).outname as *const i8);
    }
    unsafe { fflush(out) };
    return;
}

#[unsafe(no_mangle)]
pub extern "C" fn emit_destructor_code(out: *mut FILE, sp: &Symbol,
    lemp: &Lemon, lineno: &mut i32) -> () {
    let mut cp: *const i8 = core::ptr::null();
    if (*sp).type_ == TERMINAL {
        cp = (*lemp).tokendest;
        if cp == core::ptr::null_mut() { return; }
        unsafe { fprintf(out, c"{\n".as_ptr() as *mut i8 as *const i8) };
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
    } else if !((*sp).destructor).is_null() {
        cp = (*sp).destructor;
        unsafe { fprintf(out, c"{\n".as_ptr() as *mut i8 as *const i8) };
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
        if ((*lemp).nolinenosflag == 0) as i32 != 0 {
            { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
            tplt_linedir(out, (*sp).dest_lineno,
                (*lemp).filename as *const i8);
        }
    } else if !((*lemp).vardest).is_null() {
        cp = (*lemp).vardest;
        if cp == core::ptr::null_mut() { return; }
        unsafe { fprintf(out, c"{\n".as_ptr() as *mut i8 as *const i8) };
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
    } else {
        if (0 == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"emit_destructor_code".as_ptr() as *const i8,
                    c"lemon.c".as_ptr() as *mut i8 as *const i8, 3881,
                    c"0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
    }
    {
        '__b84: loop {
            if !(unsafe { *cp } != 0) { break '__b84; }
            '__c84: loop {
                if unsafe { *cp } as i32 == '$' as i32 &&
                        unsafe { *cp.offset(1 as isize) } as i32 == '$' as i32 {
                    unsafe {
                        fprintf(out,
                            c"(yypminor->yy%d)".as_ptr() as *mut i8 as *const i8,
                            (*sp).dtnum)
                    };
                    {
                        let __p = &mut cp;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    break '__c84;
                }
                if unsafe { *cp } as i32 == '\n' as i32 {
                    { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
                }
                unsafe { fputc(unsafe { *cp } as i32, out) };
                break '__c84;
            }
            {
                let __p = &mut cp;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
    unsafe { fprintf(out, c"\n".as_ptr() as *mut i8 as *const i8) };
    { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
    if ((*lemp).nolinenosflag == 0) as i32 != 0 {
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
        tplt_linedir(out, *lineno, (*lemp).outname as *const i8);
    }
    unsafe { fprintf(out, c"}\n".as_ptr() as *mut i8 as *const i8) };
    { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
    return;
}

#[unsafe(no_mangle)]
pub extern "C" fn has_destructor(sp: &Symbol, lemp: &Lemon) -> i32 {
    let mut ret: i32 = 0;
    if (*sp).type_ == TERMINAL {
        ret = ((*lemp).tokendest != core::ptr::null_mut()) as i32;
    } else {
        ret =
            ((*lemp).vardest != core::ptr::null_mut() ||
                    (*sp).destructor != core::ptr::null_mut()) as i32;
    }
    return ret;
}

#[unsafe(no_mangle)]
pub extern "C" fn append_str(mut z_text_1: *const i8, mut n: i32, mut p1: i32,
    p2: i32) -> *mut i8 {
    unsafe {
        let mut c: i32 = 0;
        let mut z_int: [i8; 40] = [0; 40];
        if z_text_1 == core::ptr::null() {
            if used == 0 && z_1 != core::ptr::null_mut() {
                unsafe { *z_1.offset(0 as isize) = 0 as i8 };
            }
            used = 0;
            return z_1;
        }
        if n <= 0 {
            if n < 0 {
                used += n;
                if !(used >= 0) as i32 as i64 != 0 {
                    unsafe {
                        __assert_rtn(c"append_str".as_ptr() as *const i8,
                            c"lemon.c".as_ptr() as *mut i8 as *const i8, 3941,
                            c"used>=0".as_ptr() as *mut i8 as *const i8)
                    }
                } else { { let _ = 0; } };
            }
            n = unsafe { strlen(z_text_1) } as i32;
        }
        if (n as u64 + core::mem::size_of::<[i8; 40]>() as u64 * 2 as u64 +
                        used as u64) as i32 >= alloced {
            alloced =
                (n as u64 + core::mem::size_of::<[i8; 40]>() as u64 * 2 as u64
                            + used as u64 + 200 as u64) as i32;
            z_1 = lemon_realloc(z_1 as *mut (), alloced as u64) as *mut i8;
        }
        if z_1 == core::ptr::null_mut() {
            return &raw mut empty[0 as usize] as *mut i8;
        }
        while { let __p = &mut n; let __t = *__p; *__p -= 1; __t } > 0 {
            c =
                unsafe {
                        *{
                                let __p = &mut z_text_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as i32;
            if c == '%' as i32 && n > 0 &&
                    unsafe { *z_text_1.offset(0 as isize) } as i32 == 'd' as i32
                {
                unsafe {
                    lemon_sprintf(&raw mut z_int[0 as usize] as *mut i8,
                        c"%d".as_ptr() as *mut i8 as *const i8, p1)
                };
                p1 = p2;
                lemon_strcpy(unsafe { &mut *z_1.offset(used as isize) },
                    &raw mut z_int[0 as usize] as *mut i8 as *const i8);
                used +=
                    unsafe {
                            strlen(unsafe { &raw mut *z_1.offset(used as isize) } as
                                    *const i8)
                        } as i32;
                {
                    let __p = &mut z_text_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
            } else {
                unsafe {
                    *z_1.offset({
                                        let __p = &mut used;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = c as i8
                };
            }
        }
        unsafe { *z_1.offset(used as isize) = 0 as i8 };
        return z_1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn emit_code(out: *mut FILE, rp: &Rule, lemp: &Lemon,
    lineno: &mut i32) -> () {
    let mut cp: *const i8 = core::ptr::null();
    if !((*rp).code_prefix).is_null() &&
            unsafe { *(*rp).code_prefix.offset(0 as isize) } != 0 {
        unsafe {
            fprintf(out, c"{%s".as_ptr() as *mut i8 as *const i8,
                (*rp).code_prefix)
        };
        {
            cp = (*rp).code_prefix;
            '__b86: loop {
                if !(unsafe { *cp } != 0) { break '__b86; }
                '__c86: loop {
                    if unsafe { *cp } as i32 == '\n' as i32 {
                        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c86;
                }
                {
                    let __p = &mut cp;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
    }
    if !((*rp).code).is_null() {
        if ((*lemp).nolinenosflag == 0) as i32 != 0 {
            { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
            tplt_linedir(out, (*rp).line, (*lemp).filename as *const i8);
        }
        unsafe {
            fprintf(out, c"{%s".as_ptr() as *mut i8 as *const i8, (*rp).code)
        };
        {
            cp = (*rp).code;
            '__b87: loop {
                if !(unsafe { *cp } != 0) { break '__b87; }
                '__c87: loop {
                    if unsafe { *cp } as i32 == '\n' as i32 {
                        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c87;
                }
                {
                    let __p = &mut cp;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        unsafe { fprintf(out, c"}\n".as_ptr() as *mut i8 as *const i8) };
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
        if ((*lemp).nolinenosflag == 0) as i32 != 0 {
            { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
            tplt_linedir(out, *lineno, (*lemp).outname as *const i8);
        }
    }
    if !((*rp).code_suffix).is_null() &&
            unsafe { *(*rp).code_suffix.offset(0 as isize) } != 0 {
        unsafe {
            fprintf(out, c"%s".as_ptr() as *mut i8 as *const i8,
                (*rp).code_suffix)
        };
        {
            cp = (*rp).code_suffix;
            '__b88: loop {
                if !(unsafe { *cp } != 0) { break '__b88; }
                '__c88: loop {
                    if unsafe { *cp } as i32 == '\n' as i32 {
                        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c88;
                }
                {
                    let __p = &mut cp;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
    }
    if !((*rp).code_prefix).is_null() {
        unsafe { fprintf(out, c"}\n".as_ptr() as *mut i8 as *const i8) };
        { let __p = &mut *lineno; let __t = *__p; *__p += 1; __t };
    }
    return;
}

#[unsafe(no_mangle)]
pub extern "C" fn print_stack_union(out: *mut FILE, lemp: &Lemon,
    plineno: &mut i32, mhflag: i32) -> () {
    let mut lineno: i32 = 0;
    let mut types: *mut *mut i8 = core::ptr::null_mut();
    let mut arraysize: i32 = 0;
    let mut maxdtlength: i32 = 0;
    let mut stddt: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut hash: u32 = 0 as u32;
    let mut name: *const i8 = core::ptr::null();
    arraysize = (*lemp).nsymbol * 2;
    types =
        lemon_calloc(arraysize as u64, core::mem::size_of::<*mut i8>() as u64)
            as *mut *mut i8;
    if types == core::ptr::null_mut() {
        eprintln!("Out of memory.");
        unsafe { exit(1) };
    }
    {
        i = 0;
        '__b89: loop {
            if !(i < arraysize) { break '__b89; }
            '__c89: loop {
                unsafe { *types.offset(i as isize) = core::ptr::null_mut() };
                break '__c89;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    maxdtlength = 0;
    if !((*lemp).vartype).is_null() {
        maxdtlength = unsafe { strlen((*lemp).vartype as *const i8) } as i32;
    }
    {
        i = 0;
        '__b90: loop {
            if !(i < (*lemp).nsymbol) { break '__b90; }
            '__c90: loop {
                let mut len: i32 = 0;
                let sp: *const Symbol =
                    unsafe { *(*lemp).symbols.offset(i as isize) } as
                        *const Symbol;
                if unsafe { (*sp).datatype } == core::ptr::null_mut() {
                    break '__c90;
                }
                len =
                    unsafe { strlen(unsafe { (*sp).datatype } as *const i8) } as
                        i32;
                if len > maxdtlength { maxdtlength = len; }
                break '__c90;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    stddt = lemon_malloc((maxdtlength * 2 + 1) as u64) as *mut i8;
    if stddt == core::ptr::null_mut() {
        eprintln!("Out of memory.");
        unsafe { exit(1) };
    }
    {
        i = 0;
        '__b91: loop {
            if !(i < (*lemp).nsymbol) { break '__b91; }
            '__c91: loop {
                let sp: *mut Symbol =
                    unsafe { *(*lemp).symbols.offset(i as isize) };
                let mut cp: *const i8 = core::ptr::null();
                if sp == (*lemp).errsym {
                    unsafe { (*sp).dtnum = arraysize + 1 };
                    break '__c91;
                }
                if unsafe { (*sp).type_ } != NONTERMINAL ||
                        unsafe { (*sp).datatype } == core::ptr::null_mut() &&
                            (*lemp).vartype == core::ptr::null_mut() {
                    unsafe { (*sp).dtnum = 0 };
                    break '__c91;
                }
                cp = unsafe { (*sp).datatype };
                if cp == core::ptr::null_mut() { cp = (*lemp).vartype; }
                j = 0;
                while unsafe { isspace(unsafe { *cp } as u8 as i32) } != 0 {
                    {
                        let __p = &mut cp;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                while unsafe { *cp } != 0 {
                    unsafe {
                        *stddt.offset({
                                            let __p = &mut j;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) =
                            unsafe {
                                *{
                                        let __p = &mut cp;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            }
                    };
                }
                while j > 0 &&
                        unsafe {
                                isspace(unsafe { *stddt.offset((j - 1) as isize) } as u8 as
                                        i32)
                            } != 0 {
                    { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                }
                unsafe { *stddt.offset(j as isize) = 0 as i8 };
                if !((*lemp).tokentype).is_null() &&
                        unsafe {
                                strcmp(stddt as *const i8, (*lemp).tokentype as *const i8)
                            } == 0 {
                    unsafe { (*sp).dtnum = 0 };
                    break '__c91;
                }
                hash = 0 as u32;
                {
                    j = 0;
                    '__b95: loop {
                        if !(unsafe { *stddt.offset(j as isize) } != 0) {
                            break '__b95;
                        }
                        '__c95: loop {
                            hash =
                                hash * 53 as u32 +
                                    unsafe { *stddt.offset(j as isize) } as u32;
                            break '__c95;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                hash = (hash & 2147483647 as u32) % arraysize as u32;
                while !(unsafe { *types.add(hash as usize) }).is_null() {
                    if unsafe {
                                strcmp(unsafe { *types.add(hash as usize) } as *const i8,
                                    stddt as *const i8)
                            } == 0 {
                        unsafe { (*sp).dtnum = (hash + 1 as u32) as i32 };
                        break;
                    }
                    { let __p = &mut hash; let __t = *__p; *__p += 1; __t };
                    if hash >= arraysize as u32 { hash = 0 as u32; }
                }
                if unsafe { *types.add(hash as usize) } ==
                        core::ptr::null_mut() {
                    unsafe { (*sp).dtnum = (hash + 1 as u32) as i32 };
                    unsafe {
                        *types.add(hash as usize) =
                            lemon_malloc((unsafe { strlen(stddt as *const i8) } as i32 +
                                            1) as u64) as *mut i8
                    };
                    if unsafe { *types.add(hash as usize) } ==
                            core::ptr::null_mut() {
                        eprintln!("Out of memory.");
                        unsafe { exit(1) };
                    }
                    lemon_strcpy(unsafe { *types.add(hash as usize) },
                        stddt as *const i8);
                }
                break '__c91;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    name =
        if !((*lemp).name).is_null() {
                (*lemp).name
            } else { c"Parse".as_ptr() as *mut i8 } as *const i8;
    lineno = *plineno;
    if mhflag != 0 {
        unsafe {
            fprintf(out, c"#if INTERFACE\n".as_ptr() as *mut i8 as *const i8)
        };
        { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    }
    unsafe {
        fprintf(out,
            c"#define %sTOKENTYPE %s\n".as_ptr() as *mut i8 as *const i8,
            name,
            if !((*lemp).tokentype).is_null() {
                (*lemp).tokentype
            } else { c"void*".as_ptr() as *mut i8 })
    };
    { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    if mhflag != 0 {
        unsafe { fprintf(out, c"#endif\n".as_ptr() as *mut i8 as *const i8) };
        { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    }
    unsafe {
        fprintf(out, c"typedef union {\n".as_ptr() as *mut i8 as *const i8)
    };
    { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    unsafe {
        fprintf(out, c"  int yyinit;\n".as_ptr() as *mut i8 as *const i8)
    };
    { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    unsafe {
        fprintf(out, c"  %sTOKENTYPE yy0;\n".as_ptr() as *mut i8 as *const i8,
            name)
    };
    { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    {
        i = 0;
        '__b97: loop {
            if !(i < arraysize) { break '__b97; }
            '__c97: loop {
                if unsafe { *types.offset(i as isize) } ==
                        core::ptr::null_mut() {
                    break '__c97;
                }
                unsafe {
                    fprintf(out,
                        c"  %s yy%d;\n".as_ptr() as *mut i8 as *const i8,
                        unsafe { *types.offset(i as isize) }, i + 1)
                };
                { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
                lemon_free(unsafe { *types.offset(i as isize) } as *mut ());
                break '__c97;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if !((*lemp).errsym).is_null() &&
            unsafe { (*(*lemp).errsym).use_cnt } != 0 {
        unsafe {
            fprintf(out, c"  int yy%d;\n".as_ptr() as *mut i8 as *const i8,
                unsafe { (*(*lemp).errsym).dtnum })
        };
        { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    }
    lemon_free(stddt as *mut ());
    lemon_free(types as *mut ());
    unsafe {
        fprintf(out, c"} YYMINORTYPE;\n".as_ptr() as *mut i8 as *const i8)
    };
    { let __p = &mut lineno; let __t = *__p; *__p += 1; __t };
    *plineno = lineno;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Axset {
    stp: *mut State,
    is_tkn: i32,
    n_action: i32,
    i_order: i32,
}

extern "C" fn axset_compare(a: *const (), b: *const ()) -> i32 {
    let p1: *mut Axset = a as *mut Axset;
    let p2: *mut Axset = b as *mut Axset;
    let mut c: i32 = 0;
    c = unsafe { (*p2).n_action } - unsafe { (*p1).n_action };
    if c == 0 { c = unsafe { (*p1).i_order } - unsafe { (*p2).i_order }; }
    if !(c != 0 || p1 == p2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"axset_compare".as_ptr() as *const i8,
                c"lemon.c".as_ptr() as *mut i8 as *const i8, 4390,
                c"c!=0 || p1==p2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return c;
}

extern "C" fn write_rule_text(out: *mut FILE, rp: &Rule) -> () {
    let mut j: i32 = 0;
    unsafe {
        fprintf(out, c"%s ::=".as_ptr() as *mut i8 as *const i8,
            unsafe { (*(*rp).lhs).name })
    };
    {
        j = 0;
        '__b98: loop {
            if !(j < (*rp).nrhs) { break '__b98; }
            '__c98: loop {
                let sp: *const Symbol =
                    unsafe { *(*rp).rhs.offset(j as isize) } as *const Symbol;
                if unsafe { (*sp).type_ } != MULTITERMINAL {
                    unsafe {
                        fprintf(out, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*sp).name })
                    };
                } else {
                    let mut k: i32 = 0;
                    unsafe {
                        fprintf(out, c" %s".as_ptr() as *mut i8 as *const i8,
                            unsafe {
                                (*unsafe {
                                                *unsafe { (*sp).subsym.offset(0 as isize) }
                                            }).name
                            })
                    };
                    {
                        k = 1;
                        '__b99: loop {
                            if !(k < unsafe { (*sp).nsubsym }) { break '__b99; }
                            '__c99: loop {
                                unsafe {
                                    fprintf(out, c"|%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe {
                                            (*unsafe {
                                                            *unsafe { (*sp).subsym.offset(k as isize) }
                                                        }).name
                                        })
                                };
                                break '__c99;
                            }
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                break '__c98;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn notnull(z: *const i8) -> i32 {
    return (!(z).is_null() && unsafe { *z.offset(0 as isize) } != 0) as i32;
}

extern "C" fn state_resort_compare(a: *const (), b: *const ()) -> i32 {
    let p_a: *const State = unsafe { *(a as *mut *const State) };
    let p_b: *const State = unsafe { *(b as *mut *const State) };
    let mut n: i32 = 0;
    n = unsafe { (*p_b).n_nt_act } - unsafe { (*p_a).n_nt_act } as i32;
    if n == 0 {
        n = unsafe { (*p_b).n_tkn_act } - unsafe { (*p_a).n_tkn_act } as i32;
        if n == 0 {
            n =
                unsafe { (*p_b).statenum } -
                    unsafe { (*p_a).statenum } as i32;
        }
    }
    if !(n != 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"stateResortCompare".as_ptr() as *const i8,
                c"lemon.c".as_ptr() as *mut i8 as *const i8, 5303,
                c"n!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return n;
}

static mut actionfreelist: *mut Action = core::ptr::null_mut();

static mut templatename: [i8; 9] =
    [108 as i8, 101 as i8, 109 as i8, 112 as i8, 97 as i8, 114 as i8,
            46 as i8, 99 as i8, 0 as i8];

static mut empty: [i8; 1] = [0 as i8];

static mut z_1: *mut i8 = core::ptr::null_mut();

static mut alloced: i32 = 0;

static mut used: i32 = 0;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn memory_error()
    -> ();
    fn getstate(_: *mut Lemon)
    -> *mut State;
    fn malloc(__size: u64)
    -> *mut ();
    fn exit(_: i32)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn free(_: *mut ())
    -> ();
    fn isdigit(_c: i32)
    -> i32;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strtod(_: *const i8, _: *mut *mut i8)
    -> f64;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    fn set_new()
    -> *mut i8;
    fn strsafe(_: *const i8)
    -> *const i8;
    fn strsafe_find(_: *const i8)
    -> *const i8;
    fn strsafe_init()
    -> ();
    fn strsafe_insert(_: *const i8)
    -> i32;
    fn symbol_new(_: *const i8)
    -> *mut Symbol;
    fn symbol_find(_: *const i8)
    -> *mut Symbol;
    fn symbol_init()
    -> ();
    fn symbol_insert(_: *mut Symbol, _: *const i8)
    -> i32;
    fn symbol_nth(_: i32)
    -> *mut Symbol;
    fn symbol_arrayof()
    -> *mut *mut Symbol;
    fn state_new()
    -> *mut State;
    fn buildshifts(_: *mut Lemon, _: *mut State)
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn isspace(_c: i32)
    -> i32;
    fn isalpha(_c: i32)
    -> i32;
    fn isalnum(_c: i32)
    -> i32;
    fn strrchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn getenv(_: *const i8)
    -> *mut i8;
    fn access(_: *const i8, _: i32)
    -> i32;
    fn fgets(_: *mut i8, __size: i32, _: *mut FILE)
    -> *mut i8;
    fn putc(_: i32, _: *mut FILE)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn fputc(_: i32, _: *mut FILE)
    -> i32;
    fn __builtin_unreachable()
    -> ();
    static mut __stderrp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
