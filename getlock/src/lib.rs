type DarwinSizeT = u64;

type Int64T = i64;

type DarwinOffT = Int64T;

type OffT = DarwinOffT;

type DarwinSsizeT = i64;

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

extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        unsafe {
            fprintf(__stderrp,
                c"Usage: %s database\n".as_ptr() as *mut i8 as *const i8,
                argv0)
        };
        unsafe { exit(1) };
    }
}

extern "C" fn is_locked(h: i32, type__1: i32, i_ofst_1: u32, i_cnt_1: u32,
    z_type_1: *const i8) -> i32 {
    unsafe {
        let mut lk: Flock = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut lk as *mut (), 0,
                core::mem::size_of::<Flock>() as u64)
        };
        lk.l_type = type__1 as i16;
        lk.l_whence = 0 as i16;
        lk.l_start = i_ofst_1 as OffT;
        lk.l_len = i_cnt_1 as OffT;
        if unsafe { fcntl(h, 7, &raw mut lk as *mut Flock) } == -1 {
            unsafe {
                fprintf(__stderrp,
                    c"fcntl(%d) failed: errno=%d\n".as_ptr() as *mut i8 as
                        *const i8, h, unsafe { *unsafe { __error() } })
            };
            unsafe { exit(1) };
        }
        if lk.l_type as i32 == 2 { return 0; }
        unsafe {
            printf(c"%s lock held by %d\n".as_ptr() as *mut i8 as *const i8,
                z_type_1, lk.l_pid as i32)
        };
        return 1;
    }
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut h_db: i32 = 0;
        let mut h_shm: i32 = 0;
        let mut z_shm: *mut i8 = core::ptr::null_mut();
        let mut got: i64 = 0 as i64;
        let mut is_wal: i32 = 0;
        let mut n_name: i32 = 0;
        let mut a_hdr: [u8; 100] = [0; 100];
        let mut n_lock: i32 = 0;
        let mut i: i32 = 0;
        if argc != 2 {
            usage(unsafe { *argv.offset(0 as isize) } as *const i8);
        }
        h_db =
            unsafe {
                open(unsafe { *argv.offset(1 as isize) } as *const i8, 0, 0)
            };
        if h_db < 0 {
            unsafe {
                fprintf(__stderrp,
                    c"cannot open %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(1 as isize) })
            };
            return Err(1);
        }
        got =
            unsafe {
                read(h_db, &raw mut a_hdr[0 as usize] as *mut u8 as *mut (),
                    100 as u64)
            };
        if got != 100 as i64 ||
                unsafe {
                        memcmp(&raw mut a_hdr[0 as usize] as *mut u8 as *const (),
                            c"SQLite format 3".as_ptr() as *mut i8 as *const (),
                            16 as u64)
                    } != 0 {
            unsafe {
                fprintf(__stderrp,
                    c"not an SQLite database: %s\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { *argv.offset(1 as isize) })
            };
            unsafe { exit(1) };
        }
        if is_locked(h_db, 1, (1073741824 + 2) as u32, 510 as u32,
                    c"EXCLUSIVE".as_ptr() as *mut i8 as *const i8) != 0 {
            return Ok(());
        }
        is_wal = (a_hdr[18 as usize] as i32 == 2) as i32;
        if is_wal == 0 {
            if is_locked(h_db, 1, 1073741824 as u32, 1 as u32,
                        c"PENDING".as_ptr() as *mut i8 as *const i8) != 0 {
                return Ok(());
            }
            if is_locked(h_db, 1, (1073741824 + 1) as u32, 1 as u32,
                        c"RESERVED".as_ptr() as *mut i8 as *const i8) != 0 {
                return Ok(());
            }
            if is_locked(h_db, 3, (1073741824 + 2) as u32, 510 as u32,
                        c"SHARED".as_ptr() as *mut i8 as *const i8) != 0 {
                return Ok(());
            }
        } else {
            n_name =
                unsafe {
                        strlen(unsafe { *argv.offset(1 as isize) } as *const i8)
                    } as i32;
            z_shm = unsafe { malloc((n_name + 100) as u64) } as *mut i8;
            if z_shm == core::ptr::null_mut() {
                eprintln!("out of memory");
                unsafe { exit(1) };
            }
            unsafe {
                memcpy(z_shm as *mut (),
                    unsafe { *argv.offset(1 as isize) } as *const (),
                    n_name as u64)
            };
            unsafe {
                memcpy(unsafe { &raw mut *z_shm.offset(n_name as isize) } as
                        *mut (), c"-shm".as_ptr() as *mut i8 as *const (), 5 as u64)
            };
            h_shm = unsafe { open(z_shm as *const i8, 0, 0) };
            if h_shm < 0 {
                unsafe {
                    fprintf(__stderrp,
                        c"cannot open %s\n".as_ptr() as *mut i8 as *const i8, z_shm)
                };
                return Err(1);
            }
            if is_locked(h_shm, 1, (120 + 2) as u32, 1 as u32,
                        c"WAL-RECOVERY".as_ptr() as *mut i8 as *const i8) != 0 {
                return Ok(());
            }
            n_lock +=
                is_locked(h_shm, 1, (120 + 1) as u32, 1 as u32,
                    c"WAL-CHECKPOINT".as_ptr() as *mut i8 as *const i8);
            n_lock +=
                is_locked(h_shm, 1, 120 as u32, 1 as u32,
                    c"WAL-WRITE".as_ptr() as *mut i8 as *const i8);
            {
                i = 0;
                '__b0: loop {
                    if !(i < 5) { break '__b0; }
                    '__c0: loop {
                        n_lock +=
                            is_locked(h_shm, 3, (120 + 3 + i) as u32, 1 as u32,
                                c"WAL-READ".as_ptr() as *mut i8 as *const i8);
                        break '__c0;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if n_lock == 0 {
            unsafe {
                printf(c"file is not locked\n".as_ptr() as *mut i8 as
                        *const i8)
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
    fn exit(_: i32)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn fcntl(_: i32, _: i32, ...)
    -> i32;
    fn __error()
    -> *mut i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn open(_: *const i8, _: i32, ...)
    -> i32;
    fn read(_: i32, _: *mut (), __nbyte: u64)
    -> i64;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn malloc(__size: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    static mut __stderrp: *mut FILE;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
