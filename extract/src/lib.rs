type DarwinSizeT = u64;
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut f: *mut FILE = core::ptr::null_mut();
        let mut z_buf: *mut i8 = core::ptr::null_mut();
        let mut ofst: i32 = 0;
        let mut n: i32 = 0;
        let mut got: u64 = 0 as u64;
        if argc != 4 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s FILENAME OFFSET AMOUNT\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { *argv })
            };
            return Err(1);
        }
        f =
            unsafe {
                fopen(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if f == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot open \"%s\"\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) })
            };
            return Err(1);
        }
        ofst =
            unsafe { atoi(unsafe { *argv.offset(2 as isize) } as *const i8) };
        n = unsafe { atoi(unsafe { *argv.offset(3 as isize) } as *const i8) };
        z_buf = unsafe { malloc(n as u64) } as *mut i8;
        if z_buf == core::ptr::null_mut() {
            eprintln!("out of memory");
            return Err(1);
        }
        unsafe { fseek(f, ofst as i64, 0) };
        got = unsafe { fread(z_buf as *mut (), 1 as u64, n as u64, f) };
        unsafe { fclose(f) };
        if got < n as u64 {
            eprintln!("got only {} of {} bytes", got as i32, n as i32);
            return Err(1);
        } else {
            unsafe {
                fwrite(z_buf as *const (), 1 as u64, n as u64, __stdoutp)
            };
        }
        return Ok(());
    }
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn atoi(_: *const i8)
    -> i32;
    fn malloc(__size: u64)
    -> *mut ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;