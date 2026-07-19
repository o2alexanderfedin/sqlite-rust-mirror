type Int64T = i64;

type DarwinOffT = Int64T;

type OffT = DarwinOffT;

type DarwinSizeT = u64;

type Int32T = i32;

type DarwinPidT = Int32T;

type PidT = DarwinPidT;

#[repr(C)]
#[derive(Copy, Clone)]
struct Flock {
    l_start: i64,
    l_len: i64,
    l_pid: i32,
    l_type: i16,
    l_whence: i16,
}

///* Print all locks on the inode of "fd" that occur in between
///* lwr and upr, inclusive.
extern "C" fn show_locks_in_range(fd: i32, mut lwr: OffT, mut upr: OffT)
    -> i32 {
    unsafe {
        let mut cnt: i32 = 0;
        let mut x: Flock = unsafe { core::mem::zeroed() };
        let mut a_pending: *mut LockRangeN9lockRange = core::ptr::null_mut();
        let mut n_alloc: i32 = 1;
        let mut n_pending: i32 = 0;
        let mut n_done: i32 = 0;
        n_pending = 1;
        a_pending =
            unsafe {
                    malloc(core::mem::size_of::<LockRangeN9lockRange>() as u64)
                } as *mut LockRangeN9lockRange;
        if a_pending == core::ptr::null_mut() {
            eprintln!("out of memory");
            unsafe { exit(1) };
        }
        unsafe { (*a_pending.offset(0 as isize)).lwr = lwr };
        unsafe { (*a_pending.offset(0 as isize)).upr = upr };
        {
            n_done = 0;
            '__b0: loop {
                if !(n_done < n_pending) { break '__b0; }
                '__c0: loop {
                    lwr = unsafe { (*a_pending.offset(n_done as isize)).lwr };
                    upr = unsafe { (*a_pending.offset(n_done as isize)).upr };
                    if lwr >= upr { break '__c0; }
                    x.l_type = 3 as i16;
                    x.l_whence = 0 as i16;
                    x.l_start = lwr;
                    x.l_len = upr - lwr;
                    unsafe { fcntl(fd, 7, &raw mut x as *mut Flock) };
                    if x.l_type as i32 == 2 { break '__c0; }
                    unsafe {
                        printf(c"start: %-12d len: %-5d pid: %-5d type: %s\n".as_ptr()
                                    as *mut i8 as *const i8, x.l_start as i32, x.l_len as i32,
                            x.l_pid,
                            if x.l_type as i32 == 3 {
                                c"WRLCK".as_ptr() as *mut i8
                            } else { c"RDLCK".as_ptr() as *mut i8 })
                    };
                    { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                    if n_pending + 2 > n_alloc {
                        n_alloc = n_alloc * 2 + 2;
                        a_pending =
                            unsafe {
                                    realloc(a_pending as *mut (),
                                        core::mem::size_of::<LockRangeN9lockRange>() as u64 *
                                            n_alloc as u64)
                                } as *mut LockRangeN9lockRange;
                    }
                    if a_pending == core::ptr::null_mut() {
                        unsafe {
                            fprintf(__stderrp,
                                c"unable to realloc for %d bytes\n".as_ptr() as *mut i8 as
                                    *const i8,
                                core::mem::size_of::<LockRangeN9lockRange>() as i32 *
                                    (n_pending + 2))
                        };
                        unsafe { exit(1) };
                    }
                    if lwr < x.l_start {
                        unsafe {
                            (*a_pending.offset(n_pending as isize)).lwr = lwr
                        };
                        unsafe {
                            (*a_pending.offset(n_pending as isize)).upr = x.l_start
                        };
                        {
                            let __p = &mut n_pending;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                    if x.l_start + x.l_len <= upr {
                        unsafe {
                            (*a_pending.offset(n_pending as isize)).lwr =
                                x.l_start + x.l_len
                        };
                        unsafe {
                            (*a_pending.offset(n_pending as isize)).upr = upr
                        };
                        {
                            let __p = &mut n_pending;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                    break '__c0;
                }
                { let __p = &mut n_done; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { free(a_pending as *mut ()) };
        return cnt;
    }
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut fd: i32 = 0;
        let mut cnt: i32 = 0;
        if argc != 2 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s FILENAME\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) })
            };
            return Err(1);
        }
        fd =
            unsafe {
                open(unsafe { *argv.offset(1 as isize) } as *const i8, 2, 0)
            };
        if fd < 0 {
            unsafe {
                fprintf(__stderrp,
                    c"%s: cannot open %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) },
                    unsafe { *argv.offset(1 as isize) })
            };
            return Err(1);
        }
        cnt = show_locks_in_range(fd, 0 as OffT, 2147483647 as OffT);
        if cnt == 0 {
            unsafe { printf(c"no locks\n".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe { close(fd) };
        return Ok(());
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct LockRangeN9lockRange {
    lwr: OffT,
    upr: OffT,
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
    fn malloc(__size: u64)
    -> *mut ();
    fn exit(_: i32)
    -> ();
    fn fcntl(_: i32, _: i32, ...)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn realloc(__ptr: *mut (), __size: u64)
    -> *mut ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn free(_: *mut ())
    -> ();
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn close(_: i32)
    -> i32;
    static mut __stderrp: *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
