//!* Compile this program against an SQLite library of unknown version
//!* and then run this program, and it will print out the SQLite version
//!* information.

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        printf(c"SQLite version %s\n".as_ptr() as *mut i8 as *const i8,
            unsafe { sqlite3_libversion() })
    };
    unsafe {
        printf(c"SQLite source  %s\n".as_ptr() as *mut i8 as *const i8,
            unsafe { sqlite3_sourceid() })
    };
    return Ok(());
}

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn sqlite3_libversion()
    -> *const i8;
    fn sqlite3_sourceid()
    -> *const i8;
    fn printf(_: *const i8, ...)
    -> i32;
}
