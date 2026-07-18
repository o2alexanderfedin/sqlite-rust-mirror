type __darwin_size_t = u64;
static mut page_size: i32 = 1024;
static mut sector_size: i32 = 512;
static mut db: *mut FILE = core::ptr::null_mut();
static mut file_size: i32 = 0;
static mut cksum_nonce: u32 = 0 as u32;
extern "C" fn out_of_memory() -> () {
    eprintln!("Out of memory...");
    unsafe { exit(1) };
}
extern "C" fn read_content(n_1: i32, i_ofst_1: i32) -> *mut u8 {
    unsafe {
        let mut got: i32 = 0;
        let p_buf: *mut u8 = unsafe { malloc(n_1 as u64) } as *mut u8;
        if p_buf == core::ptr::null_mut() { out_of_memory(); }
        unsafe { fseek(db, i_ofst_1 as i64, 0) };
        got =
            unsafe { fread(p_buf as *mut (), 1 as u64, n_1 as u64, db) } as
                i32;
        if got < 0 {
            eprintln!("I/O error reading {} bytes from {}", n_1 as i32, i_ofst_1 as i32);
            unsafe { memset(p_buf as *mut (), 0, n_1 as u64) };
        } else if got < n_1 {
            eprintln!("Short read: got only {} of {} bytes from {}", got as i32, n_1 as i32, i_ofst_1 as i32);
            unsafe {
                memset(unsafe { &raw mut *p_buf.offset(got as isize) } as
                        *mut (), 0, (n_1 - got) as u64)
            };
        }
        return p_buf;
    }
}
extern "C" fn print_decode_line(a_data_1: *const u8, ofst: i32, n_byte_1: i32,
    z_msg_1: *const i8) -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut val: u32 = unsafe { *a_data_1.offset(ofst as isize) } as u32;
    let mut z_buf: [i8; 100] = [0; 100];
    unsafe {
        sprintf(&raw mut z_buf[0 as usize] as *mut i8,
            c" %05x: %02x".as_ptr() as *mut i8 as *const i8, ofst,
            unsafe { *a_data_1.offset(ofst as isize) } as i32)
    };
    i =
        unsafe { strlen(&raw mut z_buf[0 as usize] as *mut i8 as *const i8) }
            as i32;
    {
        j = 1;
        '__b0: loop {
            if !(j < 4) { break '__b0; }
            '__c0: loop {
                if j >= n_byte_1 {
                    unsafe {
                        sprintf(&mut z_buf[i as usize],
                            c"   ".as_ptr() as *mut i8 as *const i8)
                    };
                } else {
                    unsafe {
                        sprintf(&mut z_buf[i as usize],
                            c" %02x".as_ptr() as *mut i8 as *const i8,
                            unsafe { *a_data_1.offset((ofst + j) as isize) } as i32)
                    };
                    val =
                        val * 256 as u32 +
                            unsafe { *a_data_1.offset((ofst + j) as isize) } as u32;
                }
                i +=
                    unsafe { strlen(&raw mut z_buf[i as usize] as *const i8) }
                        as i32;
                break '__c0;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        sprintf(&mut z_buf[i as usize],
            c"   %10u".as_ptr() as *mut i8 as *const i8, val)
    };
    unsafe {
        printf(c"%s  %s\n".as_ptr() as *mut i8 as *const i8,
            &raw mut z_buf[0 as usize] as *mut i8, z_msg_1)
    };
    return val;
}
extern "C" fn decode_journal_header(i_ofst_1: i32) -> u32 {
    unsafe {
        let p_hdr: *mut u8 = read_content(64, i_ofst_1);
        let mut n_page: u32 = 0 as u32;
        unsafe {
            printf(c"Header at offset %d:\n".as_ptr() as *mut i8 as *const i8,
                i_ofst_1)
        };
        print_decode_line(p_hdr as *const u8, 0, 4,
            c"Header part 1 (3654616569)".as_ptr() as *mut i8 as *const i8);
        print_decode_line(p_hdr as *const u8, 4, 4,
            c"Header part 2 (547447767)".as_ptr() as *mut i8 as *const i8);
        n_page =
            print_decode_line(p_hdr as *const u8, 8, 4,
                c"page count".as_ptr() as *mut i8 as *const i8);
        cksum_nonce =
            print_decode_line(p_hdr as *const u8, 12, 4,
                c"chksum nonce".as_ptr() as *mut i8 as *const i8);
        print_decode_line(p_hdr as *const u8, 16, 4,
            c"initial database size in pages".as_ptr() as *mut i8 as
                *const i8);
        sector_size =
            print_decode_line(p_hdr as *const u8, 20, 4,
                    c"sector size".as_ptr() as *mut i8 as *const i8) as i32;
        page_size =
            print_decode_line(p_hdr as *const u8, 24, 4,
                    c"page size".as_ptr() as *mut i8 as *const i8) as i32;
        print_decode_line(p_hdr as *const u8, 28, 4,
            c"zero".as_ptr() as *mut i8 as *const i8);
        print_decode_line(p_hdr as *const u8, 32, 4,
            c"zero".as_ptr() as *mut i8 as *const i8);
        print_decode_line(p_hdr as *const u8, 36, 4,
            c"zero".as_ptr() as *mut i8 as *const i8);
        print_decode_line(p_hdr as *const u8, 40, 4,
            c"zero".as_ptr() as *mut i8 as *const i8);
        unsafe { free(p_hdr as *mut ()) };
        return n_page;
    }
}
extern "C" fn print_page(i_ofst_1: i32) -> () {
    unsafe {
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut z_title: [i8; 50] = [0; 50];
        a_data = read_content(page_size + 8, i_ofst_1);
        unsafe {
            sprintf(&raw mut z_title[0 as usize] as *mut i8,
                c"page number for page at offset %d".as_ptr() as *mut i8 as
                    *const i8, i_ofst_1)
        };
        print_decode_line(unsafe { a_data.offset(-(i_ofst_1 as isize)) } as
                *const u8, i_ofst_1, 4,
            &raw mut z_title[0 as usize] as *mut i8 as *const i8);
        unsafe { free(a_data as *mut ()) };
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut n_page: i32 = 0;
        let mut cnt: i32 = 0;
        let mut i_ofst: i32 = 0;
        if argc != 2 {
            unsafe {
                fprintf(__stderrp,
                    c"Usage: %s FILENAME\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) })
            };
            unsafe { exit(1) };
        }
        db =
            unsafe {
                fopen(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if db == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"%s: can\'t open %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { *argv.offset(0 as isize) },
                    unsafe { *argv.offset(1 as isize) })
            };
            unsafe { exit(1) };
        }
        unsafe { fseek(db, 0 as i64, 2) };
        file_size = unsafe { ftell(db) } as i32;
        unsafe {
            printf(c"journal file size: %d bytes\n".as_ptr() as *mut i8 as
                    *const i8, file_size)
        };
        unsafe { fseek(db, 0 as i64, 0) };
        i_ofst = 0;
        while i_ofst < file_size {
            cnt = { n_page = decode_journal_header(i_ofst) as i32; n_page };
            if cnt == 0 { cnt = (file_size - sector_size) / (page_size + 8); }
            i_ofst += sector_size;
            while cnt != 0 && i_ofst < file_size {
                print_page(i_ofst);
                i_ofst += page_size + 8;
            }
            i_ofst = (i_ofst / sector_size + 1) * sector_size;
        }
        unsafe { fclose(db) };
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
    fn exit(_: i32)
    -> ();
    fn malloc(__size: u64)
    -> *mut ();
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sprintf(_: *mut i8, _: *const i8, ...)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn free(_: *mut ())
    -> ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn ftell(_: *mut FILE)
    -> i64;
    fn fclose(_: *mut FILE)
    -> i32;
    static mut __stderrp: *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct __sFILE {
    _opaque: [u8; 0],
}
type FILE = __sFILE;