#[repr(C)]
#[derive(Copy, Clone)]
struct AnonS0 {
    input: *mut FILE,
    output: *mut FILE,
    rc: i32,
    keep_first: i32,
}

#[unsafe(no_mangle)]
pub static mut app: AnonS0 =
    AnonS0 {
        input: core::ptr::null_mut(),
        output: core::ptr::null_mut(),
        rc: 0,
        keep_first: 0,
    };

#[unsafe(no_mangle)]
pub extern "C" fn do_it_all() -> () {
    unsafe {
        let mut ch: i32 = 0;
        let mut prev: i32 = -1;
        let out: *mut FILE = app.output;
        let slash: i32 = '/' as i32 as i32;
        let star: i32 = '*' as i32 as i32;
        let mut line: i32 = 1;
        let mut col: i32 = 0;
        let mut state: u32 = S_NONE;
        let mut elide: i32 = 0;
        let mut state3_col: i32 = -99;
        {
            '__b0: loop {
                if !(-1 != { ch = unsafe { fgetc(app.input) }; ch }) {
                    break '__b0;
                }
                '__c0: loop {
                    '__s1:
                        {
                        match state {
                            S_NONE => {
                                if '\'' as i32 == ch || '\"' as i32 == ch ||
                                        '`' as i32 == ch {
                                    let quote: i32 = ch as i32;
                                    let start_line: i32 = line as i32;
                                    let start_col: i32 = col as i32;
                                    let mut ch2: i32 = 0;
                                    let mut escaped: i32 = 0;
                                    let mut end_of_string: i32 = 0;
                                    unsafe { fputc(ch, out) };
                                    {
                                        { let __p = &mut col; *__p += 1; *__p };
                                        '__b2: loop {
                                            if !((end_of_string == 0) as i32 != 0 &&
                                                            -1 != { ch2 = unsafe { fgetc(app.input) }; ch2 }) {
                                                break '__b2;
                                            }
                                            '__c2: loop {
                                                '__s3:
                                                    {
                                                    match ch2 {
                                                        92 => { escaped = (escaped == 0) as i32 as i32; }
                                                        96 => {
                                                            if (escaped == 0) as i32 != 0 && quote as i32 == ch2 {
                                                                end_of_string = 1;
                                                            }
                                                            escaped = 0;
                                                        }
                                                        39 => {
                                                            if (escaped == 0) as i32 != 0 && quote as i32 == ch2 {
                                                                end_of_string = 1;
                                                            }
                                                            escaped = 0;
                                                        }
                                                        34 => {
                                                            if (escaped == 0) as i32 != 0 && quote as i32 == ch2 {
                                                                end_of_string = 1;
                                                            }
                                                            escaped = 0;
                                                        }
                                                        _ => { escaped = 0; }
                                                    }
                                                }
                                                if '\n' as i32 == ch2 {
                                                    { let __p = &mut line; *__p += 1; *__p };
                                                    col = 0;
                                                }
                                                unsafe { fputc(ch2, out) };
                                                break '__c2;
                                            }
                                            { let __p = &mut col; *__p += 1; *__p };
                                        }
                                    }
                                    if -1 == ch2 {
                                        unsafe {
                                            fprintf(__stderrp,
                                                c"Unexpected EOF while reading %s literal on line %d column %d.\n".as_ptr()
                                                        as *mut i8 as *const i8,
                                                if '\'' as i32 == ch {
                                                    c"char".as_ptr() as *mut i8
                                                } else { c"string".as_ptr() as *mut i8 }, start_line,
                                                start_col)
                                        };
                                        app.rc = 1;
                                        return;
                                    }
                                    break '__s1;
                                } else if slash as i32 == ch {
                                    if '\\' as i32 == prev {
                                        unsafe { fputc(ch, out) };
                                    } else { state = S_SLASH1; }
                                    break '__s1;
                                }
                                unsafe { fputc(ch, out) };
                            }
                            S_SLASH1 => {
                                '__s4:
                                    {
                                    match ch {
                                        42 => {
                                            if app.keep_first > 0 {
                                                elide = 0;
                                                { let __p = &mut app.keep_first; *__p -= 1; *__p };
                                            } else { elide = 1; }
                                            state = S_C;
                                            state3_col = col - 1;
                                            if (elide == 0) as i32 != 0 {
                                                unsafe { fputc(prev, out) };
                                                unsafe { fputc(ch, out) };
                                            }
                                        }
                                        47 => {
                                            if app.keep_first > 0 {
                                                elide = 0;
                                                { let __p = &mut app.keep_first; *__p -= 1; *__p };
                                            } else { elide = 1; }
                                            state = S_CPP;
                                            if (elide == 0) as i32 != 0 {
                                                unsafe { fputc(prev, out) };
                                                unsafe { fputc(ch, out) };
                                            }
                                        }
                                        _ => {
                                            state = S_NONE;
                                            if (elide == 0) as i32 != 0 {
                                                unsafe { fputc(prev, out) };
                                                unsafe { fputc(ch, out) };
                                            }
                                        }
                                    }
                                }
                            }
                            S_CPP => {
                                if '\n' as i32 == ch { state = S_NONE; elide = 0; }
                                if (elide == 0) as i32 != 0 { unsafe { fputc(ch, out) }; }
                            }
                            S_C => {
                                if (elide == 0) as i32 != 0 { unsafe { fputc(ch, out) }; }
                                if slash as i32 == ch {
                                    if star as i32 == prev {
                                        if col != state3_col + 2 {
                                            state = S_NONE;
                                            elide = 0;
                                            state3_col = -99;
                                        }
                                    }
                                }
                            }
                            _ => {
                                if (0 == 0) as i32 as i64 != 0 {
                                    unsafe {
                                        __assert_rtn(c"do_it_all".as_ptr() as *const i8,
                                            c"stripccomments.c".as_ptr() as *mut i8 as *const i8, 211,
                                            c"!\"impossible!\"".as_ptr() as *mut i8 as *const i8)
                                    }
                                } else { { let _ = 0; } };
                            }
                        }
                    }
                    if '\n' as i32 == ch {
                        { let __p = &mut line; *__p += 1; *__p };
                        col = 0;
                        state3_col = -99;
                    }
                    break '__c0;
                }
                { prev = ch; { let __p = &mut col; *__p += 1; *__p } };
            }
        }
    }
}

extern "C" fn usage(z_app_name_1: *const i8) -> () {
    unsafe {
        eprintln!("Strips C- and C++-style comments from stdin and sends the results to stdout.");
        unsafe {
            fprintf(__stderrp,
                c"Usage: %s [--keep-first|-k] < input > output\n".as_ptr() as
                        *mut i8 as *const i8, z_app_name_1)
        };
    }
}

extern "C" fn __main_inner(argc: i32, argv: *const *const i8)
    -> Result<(), i32> {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 1;
            '__b5: loop {
                if !(i < argc) { break '__b5; }
                '__c5: loop {
                    let mut z_arg: *const i8 =
                        unsafe { *argv.offset(i as isize) };
                    while '-' as i32 == unsafe { *z_arg } as i32 {
                        {
                            let __p = &mut z_arg;
                            *__p = unsafe { (*__p).offset(1) };
                            *__p
                        };
                    }
                    if 0 ==
                                unsafe {
                                    strcmp(z_arg, c"k".as_ptr() as *mut i8 as *const i8)
                                } ||
                            0 ==
                                unsafe {
                                    strcmp(z_arg,
                                        c"keep-first".as_ptr() as *mut i8 as *const i8)
                                } {
                        { let __p = &mut app.keep_first; *__p += 1; *__p };
                    } else {
                        usage(unsafe { *argv.offset(0 as isize) });
                        return Err(1);
                    }
                    break '__c5;
                }
                { let __p = &mut i; *__p += 1; *__p };
            }
        }
        app.input = __stdinp;
        app.output = __stdoutp;
        do_it_all();
        return Err(if app.rc != 0 { 1 } else { 0 });
    }
}

const S_NONE: u32 = 0;

const S_SLASH1: u32 = 1;

const S_C: u32 = 3;

const S_CPP: u32 = 2;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn fgetc(_: *mut FILE)
    -> i32;
    fn fputc(_: i32, _: *mut FILE)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    static mut __stderrp: *mut FILE;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
