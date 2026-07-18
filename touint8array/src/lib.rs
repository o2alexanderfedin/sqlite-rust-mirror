extern "C" fn __main_inner(argc: i32, argv: *const *const i8)
    -> Result<(), i32> {
    unsafe {
        let mut i: i32 = 0;
        let rc: i32 = 0;
        let col_width: i32 = 30;
        let mut ch: i32 = 0;
        unsafe { printf(c"[".as_ptr() as *mut i8 as *const i8) };
        {
            i = 0;
            '__b0: loop {
                if !(-1 != { ch = unsafe { fgetc(__stdinp) }; ch }) {
                    break '__b0;
                }
                '__c0: loop {
                    if 0 != i {
                        unsafe { printf(c",".as_ptr() as *mut i8 as *const i8) };
                    }
                    if i != 0 && 0 == i % col_width {
                        unsafe { puts(c"".as_ptr() as *mut i8 as *const i8) };
                    }
                    unsafe {
                        printf(c"%d".as_ptr() as *mut i8 as *const i8, ch)
                    };
                    break '__c0;
                }
                { let __p = &mut i; *__p += 1; *__p };
            }
        }
        unsafe { printf(c"]".as_ptr() as *mut i8 as *const i8) };
        return Err(rc);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn printf(_: *const i8, ...)
    -> i32;
    fn fgetc(_: *mut FILE)
    -> i32;
    fn puts(_: *const i8)
    -> i32;
    static mut __stdinp: *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
