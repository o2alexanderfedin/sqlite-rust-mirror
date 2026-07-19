#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
use crate::sqlite3_h::{
    Sqlite3, Sqlite3ApiRoutines, Sqlite3Backup, Sqlite3Blob, Sqlite3Context,
    Sqlite3File, Sqlite3Filename, Sqlite3IndexInfo, Sqlite3Int64,
    Sqlite3Module, Sqlite3Mutex, Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo,
    Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Str, Sqlite3Uint64, Sqlite3Value,
    Sqlite3Vfs,
};

type Int32T = i32;

type DarwinPidT = Int32T;

type PidT = DarwinPidT;

type DarwinSizeT = u64;

static z_usage: [i8; 757] =
    [115 as i8, 113 as i8, 108 as i8, 105 as i8, 116 as i8, 101 as i8,
            51 as i8, 95 as i8, 114 as i8, 115 as i8, 121 as i8, 110 as i8,
            99 as i8, 32 as i8, 79 as i8, 82 as i8, 73 as i8, 71 as i8,
            73 as i8, 78 as i8, 32 as i8, 82 as i8, 69 as i8, 80 as i8,
            76 as i8, 73 as i8, 67 as i8, 65 as i8, 32 as i8, 63 as i8,
            79 as i8, 80 as i8, 84 as i8, 73 as i8, 79 as i8, 78 as i8,
            83 as i8, 63 as i8, 10 as i8, 10 as i8, 79 as i8, 110 as i8,
            101 as i8, 32 as i8, 111 as i8, 102 as i8, 32 as i8, 79 as i8,
            82 as i8, 73 as i8, 71 as i8, 73 as i8, 78 as i8, 32 as i8,
            111 as i8, 114 as i8, 32 as i8, 82 as i8, 69 as i8, 80 as i8,
            76 as i8, 73 as i8, 67 as i8, 65 as i8, 32 as i8, 105 as i8,
            115 as i8, 32 as i8, 97 as i8, 32 as i8, 112 as i8, 97 as i8,
            116 as i8, 104 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8,
            32 as i8, 116 as i8, 111 as i8, 32 as i8, 97 as i8, 32 as i8,
            100 as i8, 97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8,
            115 as i8, 101 as i8, 32 as i8, 111 as i8, 110 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 108 as i8, 111 as i8,
            99 as i8, 97 as i8, 108 as i8, 10 as i8, 109 as i8, 97 as i8,
            99 as i8, 104 as i8, 105 as i8, 110 as i8, 101 as i8, 32 as i8,
            97 as i8, 110 as i8, 100 as i8, 32 as i8, 116 as i8, 104 as i8,
            101 as i8, 32 as i8, 111 as i8, 116 as i8, 104 as i8, 101 as i8,
            114 as i8, 32 as i8, 105 as i8, 115 as i8, 32 as i8, 111 as i8,
            102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            102 as i8, 111 as i8, 114 as i8, 109 as i8, 32 as i8, 34 as i8,
            85 as i8, 83 as i8, 69 as i8, 82 as i8, 64 as i8, 72 as i8,
            79 as i8, 83 as i8, 84 as i8, 58 as i8, 80 as i8, 65 as i8,
            84 as i8, 72 as i8, 34 as i8, 32 as i8, 100 as i8, 101 as i8,
            115 as i8, 99 as i8, 114 as i8, 105 as i8, 98 as i8, 105 as i8,
            110 as i8, 103 as i8, 10 as i8, 97 as i8, 32 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 32 as i8, 111 as i8, 110 as i8, 32 as i8, 97 as i8,
            32 as i8, 114 as i8, 101 as i8, 109 as i8, 111 as i8, 116 as i8,
            101 as i8, 32 as i8, 109 as i8, 97 as i8, 99 as i8, 104 as i8,
            105 as i8, 110 as i8, 101 as i8, 46 as i8, 32 as i8, 32 as i8,
            84 as i8, 104 as i8, 105 as i8, 115 as i8, 32 as i8, 117 as i8,
            116 as i8, 105 as i8, 108 as i8, 105 as i8, 116 as i8, 121 as i8,
            32 as i8, 109 as i8, 97 as i8, 107 as i8, 101 as i8, 115 as i8,
            32 as i8, 82 as i8, 69 as i8, 80 as i8, 76 as i8, 73 as i8,
            67 as i8, 65 as i8, 32 as i8, 105 as i8, 110 as i8, 116 as i8,
            111 as i8, 32 as i8, 97 as i8, 10 as i8, 99 as i8, 111 as i8,
            112 as i8, 121 as i8, 32 as i8, 111 as i8, 102 as i8, 32 as i8,
            79 as i8, 82 as i8, 73 as i8, 71 as i8, 73 as i8, 78 as i8,
            10 as i8, 10 as i8, 79 as i8, 80 as i8, 84 as i8, 73 as i8,
            79 as i8, 78 as i8, 83 as i8, 58 as i8, 10 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 101 as i8,
            120 as i8, 101 as i8, 32 as i8, 80 as i8, 65 as i8, 84 as i8,
            72 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 78 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8,
            111 as i8, 102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 115 as i8, 113 as i8, 108 as i8, 105 as i8, 116 as i8,
            101 as i8, 51 as i8, 95 as i8, 114 as i8, 115 as i8, 121 as i8,
            110 as i8, 99 as i8, 32 as i8, 112 as i8, 114 as i8, 111 as i8,
            103 as i8, 114 as i8, 97 as i8, 109 as i8, 32 as i8, 111 as i8,
            110 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            114 as i8, 101 as i8, 109 as i8, 111 as i8, 116 as i8, 101 as i8,
            32 as i8, 115 as i8, 105 as i8, 100 as i8, 101 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 104 as i8,
            101 as i8, 108 as i8, 112 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 83 as i8, 104 as i8, 111 as i8, 119 as i8, 32 as i8,
            116 as i8, 104 as i8, 105 as i8, 115 as i8, 32 as i8, 104 as i8,
            101 as i8, 108 as i8, 112 as i8, 32 as i8, 115 as i8, 99 as i8,
            114 as i8, 101 as i8, 101 as i8, 110 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 112 as i8, 124 as i8, 45 as i8,
            45 as i8, 112 as i8, 111 as i8, 114 as i8, 116 as i8, 32 as i8,
            80 as i8, 79 as i8, 82 as i8, 84 as i8, 32 as i8, 32 as i8,
            82 as i8, 117 as i8, 110 as i8, 32 as i8, 83 as i8, 83 as i8,
            72 as i8, 32 as i8, 111 as i8, 118 as i8, 101 as i8, 114 as i8,
            32 as i8, 84 as i8, 67 as i8, 80 as i8, 32 as i8, 112 as i8,
            111 as i8, 114 as i8, 116 as i8, 32 as i8, 80 as i8, 79 as i8,
            82 as i8, 84 as i8, 32 as i8, 105 as i8, 110 as i8, 115 as i8,
            116 as i8, 101 as i8, 97 as i8, 100 as i8, 32 as i8, 111 as i8,
            102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            100 as i8, 101 as i8, 102 as i8, 97 as i8, 117 as i8, 108 as i8,
            116 as i8, 32 as i8, 50 as i8, 50 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 112 as i8, 114 as i8,
            111 as i8, 116 as i8, 111 as i8, 99 as i8, 111 as i8, 108 as i8,
            32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            85 as i8, 115 as i8, 101 as i8, 32 as i8, 115 as i8, 121 as i8,
            110 as i8, 99 as i8, 32 as i8, 112 as i8, 114 as i8, 111 as i8,
            116 as i8, 111 as i8, 99 as i8, 111 as i8, 108 as i8, 32 as i8,
            118 as i8, 101 as i8, 114 as i8, 115 as i8, 105 as i8, 111 as i8,
            110 as i8, 32 as i8, 78 as i8, 46 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8, 115 as i8,
            104 as i8, 32 as i8, 80 as i8, 65 as i8, 84 as i8, 72 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            78 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 111 as i8,
            102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            83 as i8, 83 as i8, 72 as i8, 32 as i8, 112 as i8, 114 as i8,
            111 as i8, 103 as i8, 114 as i8, 97 as i8, 109 as i8, 32 as i8,
            117 as i8, 115 as i8, 101 as i8, 100 as i8, 32 as i8, 116 as i8,
            111 as i8, 32 as i8, 114 as i8, 101 as i8, 97 as i8, 99 as i8,
            104 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            114 as i8, 101 as i8, 109 as i8, 111 as i8, 116 as i8, 101 as i8,
            32 as i8, 115 as i8, 105 as i8, 100 as i8, 101 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 118 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 86 as i8, 101 as i8, 114 as i8, 98 as i8, 111 as i8,
            115 as i8, 101 as i8, 46 as i8, 32 as i8, 32 as i8, 77 as i8,
            117 as i8, 108 as i8, 116 as i8, 105 as i8, 112 as i8, 108 as i8,
            101 as i8, 32 as i8, 118 as i8, 39 as i8, 115 as i8, 32 as i8,
            102 as i8, 111 as i8, 114 as i8, 32 as i8, 105 as i8, 110 as i8,
            99 as i8, 114 as i8, 101 as i8, 97 as i8, 115 as i8, 105 as i8,
            110 as i8, 103 as i8, 32 as i8, 111 as i8, 117 as i8, 116 as i8,
            112 as i8, 117 as i8, 116 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 118 as i8, 101 as i8, 114 as i8,
            115 as i8, 105 as i8, 111 as i8, 110 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8,
            104 as i8, 111 as i8, 119 as i8, 32 as i8, 100 as i8, 101 as i8,
            116 as i8, 97 as i8, 105 as i8, 108 as i8, 101 as i8, 100 as i8,
            32 as i8, 118 as i8, 101 as i8, 114 as i8, 115 as i8, 105 as i8,
            111 as i8, 110 as i8, 32 as i8, 105 as i8, 110 as i8, 102 as i8,
            111 as i8, 114 as i8, 109 as i8, 97 as i8, 116 as i8, 105 as i8,
            111 as i8, 110 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 119 as i8, 97 as i8, 108 as i8, 45 as i8,
            111 as i8, 110 as i8, 108 as i8, 121 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 68 as i8, 111 as i8,
            32 as i8, 110 as i8, 111 as i8, 116 as i8, 32 as i8, 115 as i8,
            121 as i8, 110 as i8, 99 as i8, 32 as i8, 117 as i8, 110 as i8,
            108 as i8, 101 as i8, 115 as i8, 115 as i8, 32 as i8, 98 as i8,
            111 as i8, 116 as i8, 104 as i8, 32 as i8, 100 as i8, 97 as i8,
            116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8, 101 as i8,
            115 as i8, 32 as i8, 97 as i8, 114 as i8, 101 as i8, 32 as i8,
            105 as i8, 110 as i8, 32 as i8, 87 as i8, 65 as i8, 76 as i8,
            32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8, 10 as i8,
            0 as i8];

#[repr(C)]
#[derive(Copy, Clone)]
struct SQLiteRsync {
    z_origin: *const i8,
    z_replica: *const i8,
    z_err_file: *const i8,
    z_debug_file: *const i8,
    p_out: *mut FILE,
    p_in: *mut FILE,
    p_log: *mut FILE,
    p_debug: *mut FILE,
    db: *mut Sqlite3,
    n_err: i32,
    n_wr_err: i32,
    e_verbose: u8,
    b_comm_check: u8,
    is_remote: u8,
    is_replica: u8,
    i_protocol: u8,
    wrong_encoding: u8,
    b_wal_only: u8,
    n_out: Sqlite3Uint64,
    n_in: Sqlite3Uint64,
    n_page: u32,
    sz_page: u32,
    n_hash_sent: u64,
    n_round: u32,
    n_page_sent: u32,
}

///* Create a child process running shell command "zCmd".  *ppOut is
///* a FILE that becomes the standard input of the child process.
///* (The caller writes to *ppOut in order to send text to the child.)
///* *ppIn is stdout from the child process.  (The caller
///* reads from *ppIn in order to receive input from the child.)
///* Note that *ppIn is an unbuffered file descriptor, not a FILE.
///* The process ID of the child is written into *pChildPid.
///*
///* Return the number of errors.
#[allow(unused_doc_comments)]
extern "C" fn popen2(z_cmd_1: *const i8, pp_in_1: &mut *mut FILE,
    pp_out_1: &mut *mut FILE, p_child_pid_1: &mut i32, b_direct_1: i32)
    -> i32 {
    let mut pin: [i32; 2] = [0; 2];
    let mut pout: [i32; 2] = [0; 2];
    *pp_in_1 = core::ptr::null_mut();
    *pp_out_1 = core::ptr::null_mut();
    *p_child_pid_1 = 0;
    if unsafe { pipe(&raw mut pin[0 as usize] as *mut i32) } < 0 { return 1; }
    if unsafe { pipe(&raw mut pout[0 as usize] as *mut i32) } < 0 {
        unsafe { close(pin[0 as usize]) };
        unsafe { close(pin[1 as usize]) };
        return 1;
    }
    *p_child_pid_1 = unsafe { fork() };
    if *p_child_pid_1 < 0 {
        unsafe { close(pin[0 as usize]) };
        unsafe { close(pin[1 as usize]) };
        unsafe { close(pout[0 as usize]) };
        unsafe { close(pout[1 as usize]) };
        *p_child_pid_1 = 0;
        return 1;
    }
    unsafe {
        signal(13,
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(i32) -> ()>(1 as *const ())
            })
    };
    if *p_child_pid_1 == 0 {
        let mut fd: i32 = 0;

        /// This is the child process
        unsafe { close(0) };
        fd = unsafe { dup(pout[0 as usize]) };
        if fd != 0 {
            eprint!("popen2() failed to open file descriptor 0");
            unsafe { exit(1) };
        }
        unsafe { close(pout[0 as usize]) };
        unsafe { close(pout[1 as usize]) };
        unsafe { close(1) };
        fd = unsafe { dup(pin[1 as usize]) };
        if fd != 1 {
            eprint!("popen() failed to open file descriptor 1");
            unsafe { exit(1) };
        }
        unsafe { close(pin[0 as usize]) };
        unsafe { close(pin[1 as usize]) };
        if b_direct_1 != 0 {
            unsafe { execl(z_cmd_1, z_cmd_1, 0 as *mut i8) };
        } else {
            unsafe {
                execl(c"/bin/sh".as_ptr() as *mut i8 as *const i8,
                    c"/bin/sh".as_ptr() as *mut i8 as *const i8,
                    c"-c".as_ptr() as *mut i8, z_cmd_1, 0 as *mut i8)
            };
        }
        return 1;
    } else {

        /// This is the parent process
        unsafe { close(pin[1 as usize]) };
        *pp_in_1 =
            unsafe {
                fdopen(pin[0 as usize], c"r".as_ptr() as *mut i8 as *const i8)
            };
        unsafe { close(pout[0 as usize]) };
        *pp_out_1 =
            unsafe {
                fdopen(pout[1 as usize],
                    c"w".as_ptr() as *mut i8 as *const i8)
            };
        return 0;
    }
}

///* Close the connection to a child process previously created using
///* popen2().
extern "C" fn pclose2(p_in_1: *mut FILE, p_out_1: *mut FILE, child_pid_1: i32)
    -> i32 {
    let mut wp: i32 = 0;
    let mut rc: i32 = 0;
    unsafe { fclose(p_in_1) };
    unsafe { fclose(p_out_1) };
    '__b0: loop {
        '__c0: loop {
            wp = unsafe { waitpid(0, &mut rc, 1) };
            if wp > 0 {
                if unsafe { *(&raw mut rc as *mut i32) } & 127 == 0 {
                    rc = unsafe { *(&raw mut rc as *mut i32) } >> 8 & 255;
                } else if unsafe { *(&raw mut rc as *mut i32) } & 127 != 127
                        && unsafe { *(&raw mut rc as *mut i32) } & 127 != 0 {
                    rc = unsafe { *(&raw mut rc as *mut i32) } & 127;
                } else { rc = 0; }
            }
            break '__c0;
        }
        if !(wp > 0) { break '__b0; }
    }
    return rc;
}

///* Meanings for bytes in a filename:
///*
///*    0      Ordinary character.  No encoding required
///*    1      Needs to be escaped
///*    2      Illegal character.  Do not allow in a filename
///*    3      First byte of a 2-byte UTF-8
///*    4      First byte of a 3-byte UTF-8
///*    5      First byte of a 4-byte UTF-8
static a_safe_char: [i8; 256] =
    [2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 1 as i8, 1 as i8, 0 as i8, 1 as i8, 1 as i8,
            1 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 0 as i8, 1 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 1 as i8, 1 as i8, 1 as i8, 0 as i8,
            1 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8, 2 as i8,
            2 as i8, 2 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8,
            3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8,
            3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8,
            3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8,
            3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 3 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8, 4 as i8,
            4 as i8, 5 as i8, 5 as i8, 5 as i8, 5 as i8, 5 as i8, 5 as i8,
            5 as i8, 5 as i8, 5 as i8, 5 as i8, 5 as i8, 5 as i8, 5 as i8,
            5 as i8, 5 as i8, 5 as i8];

///* pStr is a shell command under construction.  This routine safely
///* appends filename argument zIn.  It returns 0 on success or non-zero
///* on any error.
///*
///* The argument is escaped if it contains white space or other characters
///* that need to be escaped for the shell.  If zIn contains characters
///* that cannot be safely escaped, then throw a fatal error.
///*
///* If the isFilename argument is true, then the argument is expected
///* to be a filename.  As shell commands commonly have command-line
///* options that begin with "-" and since we do not want an attacker
///* to be able to invoke these switches using filenames that begin
///* with "-", if zIn begins with "-", prepend an additional "./"
///* (or ".\\" on Windows).
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn append_escaped_arg(p_str_1: *mut Sqlite3Str,
    z_in_1: *const i8, is_filename_1: i32) -> i32 {
    let mut i: i32 = 0;
    let mut c: u8 = 0 as u8;
    let mut need_escape: i32 = 0;
    let n: i32 = unsafe { sqlite3_str_length(p_str_1) };
    let z: *const i8 = unsafe { sqlite3_str_value(p_str_1) } as *const i8;
    {
        i = 0;
        '__b1: loop {
            if !({ c = unsafe { *z_in_1.offset(i as isize) } as u8; c } as i32
                            != 0) {
                break '__b1;
            }
            '__c1: loop {
                if a_safe_char[c as usize] != 0 {
                    let x: u8 = a_safe_char[c as usize] as u8;
                    need_escape = 1;
                    if x as i32 == 2 {

                        /// Bad ASCII character
                        return 1;
                    } else if x as i32 > 2 {
                        if unsafe { *z_in_1.offset((i + 1) as isize) } as i32 & 192
                                        != 128 ||
                                    x as i32 >= 4 &&
                                        unsafe { *z_in_1.offset((i + 2) as isize) } as i32 & 192 !=
                                            128 ||
                                x as i32 == 5 &&
                                    unsafe { *z_in_1.offset((i + 3) as isize) } as i32 & 192 !=
                                        128 {

                            /// Bad UTF8 character
                            return 1;
                        }
                        i += x as i32 - 2;
                    }
                }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n > 0 &&
            (unsafe { isspace(unsafe { *z.offset((n - 1) as isize) } as i32) }
                        == 0) as i32 != 0 {
        unsafe { sqlite3_str_appendchar(p_str_1, 1, ' ' as i32 as i8) };
    }
    if (need_escape == 0) as i32 != 0 {
        if is_filename_1 != 0 &&
                unsafe { *z_in_1.offset(0 as isize) } as i32 == '-' as i32 {
            unsafe { sqlite3_str_appendchar(p_str_1, 1, '.' as i32 as i8) };
            unsafe { sqlite3_str_appendchar(p_str_1, 1, '/' as i32 as i8) };
        }
        unsafe { sqlite3_str_appendall(p_str_1, z_in_1) };
    } else {
        if !(unsafe { strchr(z_in_1, '\'' as i32) }).is_null() {
            if is_filename_1 != 0 &&
                    unsafe { *z_in_1.offset(0 as isize) } as i32 == '-' as i32 {
                unsafe {
                    sqlite3_str_appendchar(p_str_1, 1, '.' as i32 as i8)
                };
                unsafe {
                    sqlite3_str_appendchar(p_str_1, 1, '/' as i32 as i8)
                };
            }
            {
                i = 0;
                '__b2: loop {
                    if !({ c = unsafe { *z_in_1.offset(i as isize) } as u8; c }
                                        as i32 != 0) {
                        break '__b2;
                    }
                    '__c2: loop {
                        if a_safe_char[c as usize] != 0 &&
                                a_safe_char[c as usize] as i32 != 2 {
                            unsafe {
                                sqlite3_str_appendchar(p_str_1, 1, '\\' as i32 as i8)
                            };
                        }
                        unsafe { sqlite3_str_appendchar(p_str_1, 1, c as i8) };
                        break '__c2;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else {
            unsafe { sqlite3_str_appendchar(p_str_1, 1, '\'' as i32 as i8) };
            if is_filename_1 != 0 &&
                    unsafe { *z_in_1.offset(0 as isize) } as i32 == '-' as i32 {
                unsafe {
                    sqlite3_str_appendchar(p_str_1, 1, '.' as i32 as i8)
                };
                unsafe {
                    sqlite3_str_appendchar(p_str_1, 1, '/' as i32 as i8)
                };
            }
            unsafe { sqlite3_str_appendall(p_str_1, z_in_1) };
            unsafe { sqlite3_str_appendchar(p_str_1, 1, '\'' as i32 as i8) };
        }
    }
    return 0;
}

/// Add an approprate PATH= argument to the SSH command under construction
///* in pStr
///*
///* About This Feature
///* ==================
///*
///* On some ssh servers (Macs in particular are guilty of this) the PATH
///* variable in the shell that runs the command that is sent to the remote
///* host contains a limited number of read-only system directories:
///*
///*      /usr/bin:/bin:/usr/sbin:/sbin
///*
///* The sqlite3_rsync executable cannot be installed into any of those
///* directories because they are locked down, and so the "sqlite3_rsync"
///* command cannot run.
///*
///* To work around this, the sqlite3_rsync command is prefixed with a PATH=
///* argument, inserted by this function, to augment the PATH with additional
///* directories in which the sqlite3_rsync executable can be installed.
///*
///* But other ssh servers are confused by this initial PATH= argument.
///* Some ssh servers have a list of programs that they are allowed to run
///* and will fail if the first argument is not on that list, and PATH=....
///* is not on that list.
///*
///* So that sqlite3_rsync can invoke itself on a remote system using ssh
///* on a variety of platforms, the following algorithm is used:
///*
///*   *  First try running the sqlite3_rsync without any PATH= argument.
///*      If that works (and it does on a majority of systems) then we are
///*      done.
///*
///*   *  If the first attempt fails, then try again after adding the
///*      PATH= prefix argument.  (This function is what adds that
///*      argument.)
///*
///* A consequence of this is that if the remote system is a Mac, the
///* "ssh" command always ends up being invoked twice.  If anybody knows a
///* way around that problem, please bring it to the attention of the
///* developers.
#[unsafe(no_mangle)]
pub extern "C" fn add_path_argument(p_str_1: *mut Sqlite3Str) -> () {
    append_escaped_arg(p_str_1,
        c"PATH=$HOME/bin:/usr/local/bin:/opt/homebrew/bin:/opt/local/bin:$PATH".as_ptr()
                as *mut i8 as *const i8, 0);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct HashContext {
    u: HashContextU0,
    n_rate: u32,
    n_loaded: u32,
    ix_mask: u32,
    i_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
union HashContextU0 {
    s: [u64; 25],
    x: [u8; 1600],
}

///* A single step of the Keccak mixing function for a 1600-bit state
extern "C" fn keccak_f1600_step(p: &mut HashContext) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut b0: u64 = 0 as u64;
        let mut b1: u64 = 0 as u64;
        let mut b2: u64 = 0 as u64;
        let mut b3: u64 = 0 as u64;
        let mut b4: u64 = 0 as u64;
        let mut c0: u64 = 0 as u64;
        let mut c1: u64 = 0 as u64;
        let mut c2: u64 = 0 as u64;
        let mut c3: u64 = 0 as u64;
        let mut c4: u64 = 0 as u64;
        let mut d0: u64 = 0 as u64;
        let mut d1: u64 = 0 as u64;
        let mut d2: u64 = 0 as u64;
        let mut d3: u64 = 0 as u64;
        let mut d4: u64 = 0 as u64;
        {
            i = 0;
            '__b3: loop {
                if !(i < 6) { break '__b3; }
                '__c3: loop {
                    c0 =
                        (*p).u.s[0 as usize] ^ (*p).u.s[5 as usize] ^
                                    (*p).u.s[10 as usize] ^ (*p).u.s[15 as usize] ^
                            (*p).u.s[20 as usize];
                    c1 =
                        (*p).u.s[1 as usize] ^ (*p).u.s[6 as usize] ^
                                    (*p).u.s[11 as usize] ^ (*p).u.s[16 as usize] ^
                            (*p).u.s[21 as usize];
                    c2 =
                        (*p).u.s[2 as usize] ^ (*p).u.s[7 as usize] ^
                                    (*p).u.s[12 as usize] ^ (*p).u.s[17 as usize] ^
                            (*p).u.s[22 as usize];
                    c3 =
                        (*p).u.s[3 as usize] ^ (*p).u.s[8 as usize] ^
                                    (*p).u.s[13 as usize] ^ (*p).u.s[18 as usize] ^
                            (*p).u.s[23 as usize];
                    c4 =
                        (*p).u.s[4 as usize] ^ (*p).u.s[9 as usize] ^
                                    (*p).u.s[14 as usize] ^ (*p).u.s[19 as usize] ^
                            (*p).u.s[24 as usize];
                    d0 = c4 ^ (c1 << 1 | c1 >> 64 - 1);
                    d1 = c0 ^ (c2 << 1 | c2 >> 64 - 1);
                    d2 = c1 ^ (c3 << 1 | c3 >> 64 - 1);
                    d3 = c2 ^ (c4 << 1 | c4 >> 64 - 1);
                    d4 = c3 ^ (c0 << 1 | c0 >> 64 - 1);
                    b0 = (*p).u.s[0 as usize] ^ d0;
                    b1 =
                        ((*p).u.s[6 as usize] ^ d1) << 44 |
                            ((*p).u.s[6 as usize] ^ d1) >> 64 - 44;
                    b2 =
                        ((*p).u.s[12 as usize] ^ d2) << 43 |
                            ((*p).u.s[12 as usize] ^ d2) >> 64 - 43;
                    b3 =
                        ((*p).u.s[18 as usize] ^ d3) << 21 |
                            ((*p).u.s[18 as usize] ^ d3) >> 64 - 21;
                    b4 =
                        ((*p).u.s[24 as usize] ^ d4) << 14 |
                            ((*p).u.s[24 as usize] ^ d4) >> 64 - 14;
                    (*p).u.s[0 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[0 as usize] ^= rc_1[i as usize] as u64;
                    (*p).u.s[6 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[12 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[18 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[24 as usize] = b4 ^ !b0 & b1;
                    b2 =
                        ((*p).u.s[10 as usize] ^ d0) << 3 |
                            ((*p).u.s[10 as usize] ^ d0) >> 64 - 3;
                    b3 =
                        ((*p).u.s[16 as usize] ^ d1) << 45 |
                            ((*p).u.s[16 as usize] ^ d1) >> 64 - 45;
                    b4 =
                        ((*p).u.s[22 as usize] ^ d2) << 61 |
                            ((*p).u.s[22 as usize] ^ d2) >> 64 - 61;
                    b0 =
                        ((*p).u.s[3 as usize] ^ d3) << 28 |
                            ((*p).u.s[3 as usize] ^ d3) >> 64 - 28;
                    b1 =
                        ((*p).u.s[9 as usize] ^ d4) << 20 |
                            ((*p).u.s[9 as usize] ^ d4) >> 64 - 20;
                    (*p).u.s[10 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[16 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[22 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[3 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[9 as usize] = b4 ^ !b0 & b1;
                    b4 =
                        ((*p).u.s[20 as usize] ^ d0) << 18 |
                            ((*p).u.s[20 as usize] ^ d0) >> 64 - 18;
                    b0 =
                        ((*p).u.s[1 as usize] ^ d1) << 1 |
                            ((*p).u.s[1 as usize] ^ d1) >> 64 - 1;
                    b1 =
                        ((*p).u.s[7 as usize] ^ d2) << 6 |
                            ((*p).u.s[7 as usize] ^ d2) >> 64 - 6;
                    b2 =
                        ((*p).u.s[13 as usize] ^ d3) << 25 |
                            ((*p).u.s[13 as usize] ^ d3) >> 64 - 25;
                    b3 =
                        ((*p).u.s[19 as usize] ^ d4) << 8 |
                            ((*p).u.s[19 as usize] ^ d4) >> 64 - 8;
                    (*p).u.s[20 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[1 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[7 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[13 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[19 as usize] = b4 ^ !b0 & b1;
                    b1 =
                        ((*p).u.s[5 as usize] ^ d0) << 36 |
                            ((*p).u.s[5 as usize] ^ d0) >> 64 - 36;
                    b2 =
                        ((*p).u.s[11 as usize] ^ d1) << 10 |
                            ((*p).u.s[11 as usize] ^ d1) >> 64 - 10;
                    b3 =
                        ((*p).u.s[17 as usize] ^ d2) << 15 |
                            ((*p).u.s[17 as usize] ^ d2) >> 64 - 15;
                    b4 =
                        ((*p).u.s[23 as usize] ^ d3) << 56 |
                            ((*p).u.s[23 as usize] ^ d3) >> 64 - 56;
                    b0 =
                        ((*p).u.s[4 as usize] ^ d4) << 27 |
                            ((*p).u.s[4 as usize] ^ d4) >> 64 - 27;
                    (*p).u.s[5 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[11 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[17 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[23 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[4 as usize] = b4 ^ !b0 & b1;
                    b3 =
                        ((*p).u.s[15 as usize] ^ d0) << 41 |
                            ((*p).u.s[15 as usize] ^ d0) >> 64 - 41;
                    b4 =
                        ((*p).u.s[21 as usize] ^ d1) << 2 |
                            ((*p).u.s[21 as usize] ^ d1) >> 64 - 2;
                    b0 =
                        ((*p).u.s[2 as usize] ^ d2) << 62 |
                            ((*p).u.s[2 as usize] ^ d2) >> 64 - 62;
                    b1 =
                        ((*p).u.s[8 as usize] ^ d3) << 55 |
                            ((*p).u.s[8 as usize] ^ d3) >> 64 - 55;
                    b2 =
                        ((*p).u.s[14 as usize] ^ d4) << 39 |
                            ((*p).u.s[14 as usize] ^ d4) >> 64 - 39;
                    (*p).u.s[15 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[21 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[2 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[8 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[14 as usize] = b4 ^ !b0 & b1;
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

///* Initialize a new hash.  iSize determines the size of the hash
///* in bits and should be one of 224, 256, 384, or 512.  Or iSize
///* can be zero to use the default hash size of 256 bits.
#[allow(unused_doc_comments)]
extern "C" fn hash_init(p: *mut HashContext, i_size_1: i32) -> () {
    unsafe {
        unsafe {
            memset(p as *mut (), 0,
                core::mem::size_of::<HashContext>() as u64)
        };
        unsafe { (*p).i_size = i_size_1 as u32 };
        if i_size_1 >= 128 && i_size_1 <= 512 {
            unsafe {
                (*p).n_rate = ((1600 - (i_size_1 + 31 & !31) * 2) / 8) as u32
            };
        } else { unsafe { (*p).n_rate = ((1600 - 2 * 256) / 8) as u32 }; }
        {
            if 1 == unsafe { *(&raw mut one as *mut u8) } as i32 {

                /// Little endian.  No byte swapping.
                unsafe { (*p).ix_mask = 0 as u32 };
            } else {

                /// Big endian.  Byte swap.
                unsafe { (*p).ix_mask = 7 as u32 };
            }
        }
    }
}

///* Make consecutive calls to the HashUpdate function to add new content
///* to the hash
extern "C" fn hash_update(p: *mut HashContext, a_data_1: *const u8,
    n_data_1: u32) -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        if a_data_1 == core::ptr::null() { return; }
        {
            '__b4: loop {
                if !(i < n_data_1) { break '__b4; }
                '__c4: loop {
                    unsafe {
                        (*p).u.x[(unsafe { (*p).n_loaded } ^
                                        unsafe { (*p).ix_mask }) as usize] ^=
                            unsafe { *a_data_1.add(i as usize) } as i32 as u8
                    };
                    {
                        let __p = unsafe { &mut (*p).n_loaded };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    if unsafe { (*p).n_loaded } == unsafe { (*p).n_rate } {
                        keccak_f1600_step(unsafe { &mut *p });
                        unsafe { (*p).n_loaded = 0 as u32 };
                    }
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

///* After all content has been added, invoke HashFinal() to compute
///* the final hash.  The function returns a pointer to the binary
///* hash value.
extern "C" fn hash_final(p: *mut HashContext) -> *mut u8 {
    unsafe {
        let mut i: u32 = 0 as u32;
        if unsafe { (*p).n_loaded } == unsafe { (*p).n_rate } - 1 as u32 {
            let c1: u8 = 134 as u8;
            hash_update(p, &c1, 1 as u32);
        } else {
            let c2: u8 = 6 as u8;
            let c3: u8 = 128 as u8;
            hash_update(p, &c2, 1 as u32);
            unsafe { (*p).n_loaded = unsafe { (*p).n_rate } - 1 as u32 };
            hash_update(p, &c3, 1 as u32);
        }
        {
            i = 0 as u32;
            '__b5: loop {
                if !(i < unsafe { (*p).n_rate }) { break '__b5; }
                '__c5: loop {
                    unsafe {
                        (*p).u.x[(i + unsafe { (*p).n_rate }) as usize] =
                            unsafe { (*p).u.x[(i ^ unsafe { (*p).ix_mask }) as usize] }
                    };
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return unsafe { &mut (*p).u.x[unsafe { (*p).n_rate } as usize] };
    }
}

///* Implementation of the hash(X) function.
///*
///* Return a 160-bit BLOB which is the hash of X.
extern "C" fn hash_func(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut cx: HashContext = unsafe { core::mem::zeroed() };
    let e_type: i32 =
        unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) };
    let n_byte: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    if e_type == 5 { return; }
    hash_init(&mut cx, 160);
    if e_type == 4 {
        hash_update(&mut cx,
            unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) }
                as *const u8, n_byte as u32);
    } else {
        hash_update(&mut cx,
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
            }, n_byte as u32);
    }
    unsafe {
        sqlite3_result_blob(context, hash_final(&mut cx) as *const (),
            160 / 8,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

///* Implementation of the agghash(X) function.
///*
///* Return a 160-bit BLOB which is the hash of the concatenation
///* of all X inputs.
extern "C" fn agghash_step(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut p_cx: *mut HashContext = core::ptr::null_mut();
    let e_type: i32 =
        unsafe { sqlite3_value_type(unsafe { *argv.offset(0 as isize) }) };
    let n_byte: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) }) };
    if e_type == 5 { return; }
    p_cx =
        unsafe {
                sqlite3_aggregate_context(context,
                    core::mem::size_of::<HashContext>() as i32)
            } as *mut HashContext;
    if p_cx == core::ptr::null_mut() { return; }
    if unsafe { (*p_cx).i_size } == 0 as u32 { hash_init(p_cx, 160); }
    if e_type == 4 {
        hash_update(p_cx,
            unsafe { sqlite3_value_blob(unsafe { *argv.offset(0 as isize) }) }
                as *const u8, n_byte as u32);
    } else {
        hash_update(p_cx,
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
            }, n_byte as u32);
    }
}

extern "C" fn agghash_final(context: *mut Sqlite3Context) -> () {
    let p_cx: *mut HashContext =
        unsafe { sqlite3_aggregate_context(context, 0) } as *mut HashContext;
    if !(p_cx).is_null() {
        unsafe {
            sqlite3_result_blob(context, hash_final(p_cx) as *const (),
                160 / 8,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
}

/// Register the hash function
extern "C" fn hash_register(db: *mut Sqlite3) -> i32 {
    let mut rc: i32 = 0;
    rc =
        unsafe {
            sqlite3_create_function(db,
                c"hash".as_ptr() as *mut i8 as *const i8, 1,
                1 | 2097152 | 2048, core::ptr::null_mut(), Some(hash_func),
                None, None)
        };
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_create_function(db,
                    c"agghash".as_ptr() as *mut i8 as *const i8, 1,
                    1 | 2097152 | 2048, core::ptr::null_mut(), None,
                    Some(agghash_step), Some(agghash_final))
            };
    }
    return rc;
}

///* Return the tail of a file pathname.  The tail is the last component
///* of the path.  For example, the tail of "/a/b/c.d" is "c.d".
#[unsafe(no_mangle)]
pub extern "C" fn file_tail(mut z: *const i8) -> *const i8 {
    let mut z_tail: *const i8 = z;
    if (z_tail).is_null() as i32 != 0 { return core::ptr::null(); }
    while unsafe { *z.offset(0 as isize) } != 0 {
        if unsafe { *z.offset(0 as isize) } as i32 == '/' as i32 {
            z_tail = unsafe { z.offset(1 as isize) };
        }
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return z_tail;
}

///* Append error message text to the error file, if an error file is
///* specified.  In any case, increment the error count.
unsafe extern "C" fn log_error(p: &mut SQLiteRsync, z_format_1: *const i8,
    mut __va0: ...) -> () {
    if !((*p).z_err_file).is_null() {
        let p_err: *mut FILE =
            unsafe {
                fopen((*p).z_err_file, c"a".as_ptr() as *mut i8 as *const i8)
            };
        if !(p_err).is_null() {
            let mut ap: *mut i8 = core::ptr::null_mut();
            unsafe { ap = core::mem::transmute_copy(&__va0) };
            unsafe { vfprintf(p_err, z_format_1, ap) };
            ();
            unsafe { fclose(p_err) };
        }
    }
    { let __p = &mut (*p).n_err; let __t = *__p; *__p += 1; __t };
}

///* Append text to the debugging mesage file, if an that file is
///* specified.
unsafe extern "C" fn debug_message(p: &mut SQLiteRsync, z_format_1: *const i8,
    mut __va0: ...) -> () {
    if !((*p).z_debug_file).is_null() {
        if (*p).p_debug == core::ptr::null_mut() {
            (*p).p_debug =
                unsafe {
                    fopen((*p).z_debug_file,
                        c"wb".as_ptr() as *mut i8 as *const i8)
                };
        }
        if !((*p).p_debug).is_null() {
            let mut ap: *mut i8 = core::ptr::null_mut();
            unsafe { ap = core::mem::transmute_copy(&__va0) };
            unsafe { vfprintf((*p).p_debug, z_format_1, ap) };
            ();
            unsafe { fflush((*p).p_debug) };
        }
    }
}

/// Read a single big-endian 32-bit unsigned integer from the input
///* stream.  Return 0 on success and 1 if there are any errors.
extern "C" fn read_uint32(p: *mut SQLiteRsync, p_u_1: &mut u32) -> i32 {
    let mut buf: [u8; 4] = [0; 4];
    if unsafe {
                fread(&raw mut buf[0 as usize] as *mut u8 as *mut (),
                    core::mem::size_of::<[u8; 4]>() as u64, 1 as u64,
                    unsafe { (*p).p_in })
            } == 1 as u64 {
        *p_u_1 =
            ((buf[0 as usize] as i32) << 24 | (buf[1 as usize] as i32) << 16 |
                        (buf[2 as usize] as i32) << 8 | buf[3 as usize] as i32) as
                u32;
        unsafe { (*p).n_in += 4 as Sqlite3Uint64 };
        return 0;
    } else {
        unsafe {
            log_error(unsafe { &mut *p },
                c"failed to read a 32-bit integer\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        return 1;
    }
}

/// Write a single big-endian 32-bit unsigned integer to the output stream.
///* Return 0 on success and 1 if there are any errors.
extern "C" fn write_uint32(p: *mut SQLiteRsync, mut x: u32) -> i32 {
    let mut buf: [u8; 4] = [0; 4];
    buf[3 as usize] = (x & 255 as u32) as u8;
    x >>= 8 as u32;
    buf[2 as usize] = (x & 255 as u32) as u8;
    x >>= 8 as u32;
    buf[1 as usize] = (x & 255 as u32) as u8;
    x >>= 8 as u32;
    buf[0 as usize] = x as u8;
    if !(unsafe { (*p).p_log }).is_null() {
        unsafe {
            fwrite(&raw mut buf[0 as usize] as *mut u8 as *const (),
                core::mem::size_of::<[u8; 4]>() as u64, 1 as u64,
                unsafe { (*p).p_log })
        };
    }
    if unsafe {
                fwrite(&raw mut buf[0 as usize] as *mut u8 as *const (),
                    core::mem::size_of::<[u8; 4]>() as u64, 1 as u64,
                    unsafe { (*p).p_out })
            } != 1 as u64 {
        unsafe {
            log_error(unsafe { &mut *p },
                c"failed to write 32-bit integer 0x%x\n".as_ptr() as *mut i8
                    as *const i8, x)
        };
        {
            let __p = unsafe { &mut (*p).n_wr_err };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return 1;
    }
    unsafe { (*p).n_out += 4 as Sqlite3Uint64 };
    return 0;
}

/// Read a single byte from the wire.
#[unsafe(no_mangle)]
pub extern "C" fn read_byte(p: &mut SQLiteRsync) -> i32 {
    let c: i32 = unsafe { fgetc((*p).p_in) };
    if c != -1 {
        { let __p = &mut (*p).n_in; let __t = *__p; *__p += 1; __t };
    }
    return c;
}

/// Write a single byte into the wire.
#[unsafe(no_mangle)]
pub extern "C" fn write_byte(p: &mut SQLiteRsync, c: i32) -> () {
    if !((*p).p_log).is_null() { unsafe { fputc(c, (*p).p_log) }; }
    unsafe { fputc(c, (*p).p_out) };
    { let __p = &mut (*p).n_out; let __t = *__p; *__p += 1; __t };
}

/// Read a power of two encoded as a single byte.
#[unsafe(no_mangle)]
pub extern "C" fn read_pow2(p: *mut SQLiteRsync) -> i32 {
    let x: i32 = read_byte(unsafe { &mut *p });
    if x < 0 || x >= 32 {
        unsafe {
            log_error(unsafe { &mut *p },
                c"read invalid page size %d\n".as_ptr() as *mut i8 as
                    *const i8, x)
        };
        return 0;
    }
    return 1 << x;
}

/// Write a power-of-two value onto the wire as a single byte.
#[unsafe(no_mangle)]
pub extern "C" fn write_pow2(p: *mut SQLiteRsync, mut c: i32) -> () {
    let mut n: i32 = 0;
    if c < 0 || c & c - 1 != 0 {
        unsafe {
            log_error(unsafe { &mut *p },
                c"trying to read invalid page size %d\n".as_ptr() as *mut i8
                    as *const i8, c)
        };
    }
    {
        n = 0;
        '__b7: loop {
            if !(c > 1) { break '__b7; }
            '__c7: loop { c /= 2; break '__c7; }
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        }
    }
    write_byte(unsafe { &mut *p }, n);
}

/// Read an array of bytes from the wire.
#[unsafe(no_mangle)]
pub extern "C" fn read_bytes(p: *mut SQLiteRsync, p_data_1: &mut [u8]) -> () {
    if unsafe {
                fread(p_data_1.as_ptr() as *mut (), 1 as u64,
                    p_data_1.len() as i32 as u64, unsafe { (*p).p_in })
            } == p_data_1.len() as i32 as u64 {
        unsafe { (*p).n_in += p_data_1.len() as i32 as Sqlite3Uint64 };
    } else {
        unsafe {
            log_error(unsafe { &mut *p },
                c"failed to read %d bytes\n".as_ptr() as *mut i8 as *const i8,
                p_data_1.len() as i32)
        };
    }
}

/// Write an array of bytes onto the wire.
#[unsafe(no_mangle)]
pub extern "C" fn write_bytes(p: *mut SQLiteRsync, p_data_1: &[u8]) -> () {
    if !(unsafe { (*p).p_log }).is_null() {
        unsafe {
            fwrite(p_data_1.as_ptr() as *const (), 1 as u64,
                p_data_1.len() as i32 as u64, unsafe { (*p).p_log })
        };
    }
    if unsafe {
                fwrite(p_data_1.as_ptr() as *const (), 1 as u64,
                    p_data_1.len() as i32 as u64, unsafe { (*p).p_out })
            } == p_data_1.len() as i32 as u64 {
        unsafe { (*p).n_out += p_data_1.len() as i32 as Sqlite3Uint64 };
    } else {
        unsafe {
            log_error(unsafe { &mut *p },
                c"failed to write %d bytes\n".as_ptr() as *mut i8 as
                    *const i8, p_data_1.len() as i32)
        };
        {
            let __p = unsafe { &mut (*p).n_wr_err };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}

/// Report an error.
///*
///* If this happens on the remote side, we send back a *_ERROR
///* message.  On the local side, the error message goes to stderr.
unsafe extern "C" fn report_error(p: *mut SQLiteRsync, z_format_1: *const i8,
    mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_msg: *mut i8 = core::ptr::null_mut();
        let mut n_msg: u32 = 0 as u32;
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_msg = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        n_msg =
            if !(z_msg).is_null() {
                (unsafe { strlen(z_msg as *const i8) }) as u32
            } else { 0 as u32 };
        if unsafe { (*p).is_remote } != 0 {
            if unsafe { (*p).is_replica } != 0 {
                unsafe { putc(98, unsafe { (*p).p_out }) };
            } else { unsafe { putc(67, unsafe { (*p).p_out }) }; }
            write_uint32(p, n_msg);
            write_bytes(p,
                unsafe {
                    let __p = z_msg as *const u8 as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, n_msg as usize) }
                });
            unsafe { fflush(unsafe { (*p).p_out }) };
        } else {
            unsafe {
                fprintf(__stderrp, c"%s\n".as_ptr() as *mut i8 as *const i8,
                    z_msg)
            };
        }
        unsafe {
            log_error(unsafe { &mut *p },
                c"%s\n".as_ptr() as *mut i8 as *const i8, z_msg)
        };
        unsafe { sqlite3_free(z_msg as *mut ()) };
    }
}

/// Send an informational message.
///*
///* If this happens on the remote side, we send back a *_MSG 
///* message.  On the local side, the message goes to stdout.
unsafe extern "C" fn info_msg(p: *mut SQLiteRsync, z_format_1: *const i8,
    mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    let mut n_msg: u32 = 0 as u32;
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg = unsafe { sqlite3_vmprintf(z_format_1, ap) };
    ();
    n_msg =
        if !(z_msg).is_null() {
            (unsafe { strlen(z_msg as *const i8) }) as u32
        } else { 0 as u32 };
    if unsafe { (*p).is_remote } != 0 {
        if unsafe { (*p).is_replica } != 0 {
            unsafe { putc(102, unsafe { (*p).p_out }) };
        } else { unsafe { putc(70, unsafe { (*p).p_out }) }; }
        write_uint32(p, n_msg);
        write_bytes(p,
            unsafe {
                let __p = z_msg as *const u8 as *const u8;
                if __p.is_null() {
                    &[]
                } else { core::slice::from_raw_parts(__p, n_msg as usize) }
            });
        unsafe { fflush(unsafe { (*p).p_out }) };
    } else {
        unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_msg) };
    }
    unsafe { sqlite3_free(z_msg as *mut ()) };
}

/// Receive and report an error message coming from the other side.
extern "C" fn read_and_display_message(p: *mut SQLiteRsync, c: i32) -> () {
    unsafe {
        let mut n: u32 = 0 as u32;
        let mut z_msg: *mut i8 = core::ptr::null_mut();
        let mut z_prefix: *const i8 = core::ptr::null();
        if c == 67 || c == 98 {
            z_prefix = c"ERROR: ".as_ptr() as *mut i8 as *const i8;
        } else { z_prefix = c"".as_ptr() as *mut i8 as *const i8; }
        read_uint32(p, &mut n);
        if n == 0 as u32 {
            eprintln!("ERROR: unknown (possibly out-of-memory)");
        } else {
            z_msg =
                unsafe { sqlite3_malloc64((n + 1 as u32) as Sqlite3Uint64) }
                    as *mut i8;
            if z_msg == core::ptr::null_mut() {
                eprintln!("ERROR: out-of-memory");
                return;
            }
            unsafe { memset(z_msg as *mut (), 0, (n + 1 as u32) as u64) };
            read_bytes(p,
                unsafe {
                    let __p = z_msg as *mut u8 as *mut u8;
                    if __p.is_null() {
                        &mut []
                    } else { core::slice::from_raw_parts_mut(__p, n as usize) }
                });
            unsafe {
                fprintf(__stderrp, c"%s%s\n".as_ptr() as *mut i8 as *const i8,
                    z_prefix, z_msg)
            };
            if unsafe { *z_prefix.offset(0 as isize) } != 0 {
                unsafe {
                    log_error(unsafe { &mut *p },
                        c"%s%s\n".as_ptr() as *mut i8 as *const i8, z_prefix, z_msg)
                };
            }
            unsafe { sqlite3_free(z_msg as *mut ()) };
        }
    }
}

/// Construct a new prepared statement.  Report an error and return NULL
///* if anything goes wrong.
extern "C" fn prepare_stmt_va(p: *mut SQLiteRsync, z_format_1: *mut i8,
    ap: *mut i8) -> *mut Sqlite3Stmt {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut z_to_free: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if !(unsafe { strchr(z_format_1 as *const i8, '%' as i32) }).is_null() {
        z_sql = unsafe { sqlite3_vmprintf(z_format_1 as *const i8, ap) };
        if z_sql == core::ptr::null_mut() {
            unsafe {
                report_error(p,
                    c"out-of-memory".as_ptr() as *mut i8 as *const i8)
            };
            return core::ptr::null_mut();
        } else { z_to_free = z_sql; }
    } else { z_sql = z_format_1; }
    rc =
        unsafe {
            sqlite3_prepare_v2(unsafe { (*p).db }, z_sql as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
    if rc != 0 || p_stmt == core::ptr::null_mut() {
        unsafe {
            report_error(p,
                c"unable to prepare SQL [%s]: %s".as_ptr() as *mut i8 as
                    *const i8, z_sql,
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
        };
        unsafe { sqlite3_finalize(p_stmt) };
        p_stmt = core::ptr::null_mut();
    }
    if !(z_to_free).is_null() {
        unsafe { sqlite3_free(z_to_free as *mut ()) };
    }
    return p_stmt;
}

unsafe extern "C" fn prepare_stmt(p: *mut SQLiteRsync, z_format_1: *mut i8,
    mut __va0: ...) -> *mut Sqlite3Stmt {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = prepare_stmt_va(p, z_format_1, ap);
    ();
    return p_stmt;
}

/// Run a single SQL statement.  Report an error if something goes
///* wrong.
///*
///* As a special case, if the statement starts with "ATTACH" (but not
///* "Attach") and if the error message is about an incorrect encoding,
///* then do not report the error, but instead set the wrongEncoding flag.
///* This is a kludgy work-around to the problem of attaching a database
///* with a non-UTF8 encoding to the empty :memory: database that is
///* opened on the replica.
unsafe extern "C" fn run_sql(p: *mut SQLiteRsync, z_sql_1: *mut i8,
    mut __va0: ...) -> () {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = prepare_stmt_va(p, z_sql_1, ap);
    ();
    if !(p_stmt).is_null() {
        let mut rc: i32 = unsafe { sqlite3_step(p_stmt) };
        if rc == 100 { rc = unsafe { sqlite3_step(p_stmt) }; }
        if rc != 0 && rc != 101 {
            let z_err: *const i8 =
                unsafe { sqlite3_errmsg(unsafe { (*p).db }) };
            if unsafe {
                            strncmp(z_sql_1 as *const i8,
                                c"ATTACH ".as_ptr() as *mut i8 as *const i8, 7 as u64)
                        } == 0 &&
                    unsafe {
                            strstr(z_err,
                                c"must use the same text encoding".as_ptr() as *mut i8 as
                                    *const i8)
                        } != core::ptr::null_mut() {
                unsafe { (*p).wrong_encoding = 1 as u8 };
            } else {
                unsafe {
                    report_error(p,
                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                            *const i8, z_sql_1,
                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                };
            }
        }
        unsafe { sqlite3_finalize(p_stmt) };
    }
}

/// Run an SQL statement that returns a single unsigned 32-bit integer result
unsafe extern "C" fn run_sql_return_u_int(p: *mut SQLiteRsync,
    p_res_1: &mut u32, z_sql_1: *mut i8, mut __va0: ...) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut res: i32 = 0;
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = prepare_stmt_va(p, z_sql_1, ap);
    ();
    if p_stmt == core::ptr::null_mut() {
        res = 1;
    } else {
        let rc: i32 = unsafe { sqlite3_step(p_stmt) };
        if rc == 100 {
            *p_res_1 =
                (unsafe { sqlite3_column_int64(p_stmt, 0) } &
                        4294967295u32 as Sqlite3Int64) as u32;
        } else {
            unsafe {
                report_error(p,
                    c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                        *const i8, z_sql_1,
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
            };
            res = 1;
        }
        unsafe { sqlite3_finalize(p_stmt) };
    }
    return res;
}

/// Run an SQL statement that returns a single TEXT value that is no more
///* than 99 bytes in length.
unsafe extern "C" fn run_sql_return_text(p: *mut SQLiteRsync,
    p_res_1: *mut i8, z_sql_1: *mut i8, mut __va0: ...) -> i32 {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut res: i32 = 0;
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = prepare_stmt_va(p, z_sql_1, ap);
    ();
    unsafe { *p_res_1.offset(0 as isize) = 0 as i8 };
    if p_stmt == core::ptr::null_mut() {
        res = 1;
    } else {
        let rc: i32 = unsafe { sqlite3_step(p_stmt) };
        if rc == 100 {
            let a: *const u8 = unsafe { sqlite3_column_text(p_stmt, 0) };
            let mut n: i32 = 0;
            if a == core::ptr::null() {
                unsafe { *p_res_1.offset(0 as isize) = 0 as i8 };
            } else {
                n = unsafe { sqlite3_column_bytes(p_stmt, 0) };
                if n > 99 { n = 99; }
                unsafe {
                    memcpy(p_res_1 as *mut (), a as *const (), n as u64)
                };
                unsafe { *p_res_1.offset(n as isize) = 0 as i8 };
            }
        } else {
            unsafe {
                report_error(p,
                    c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                        *const i8, z_sql_1,
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
            };
            res = 1;
        }
        unsafe { sqlite3_finalize(p_stmt) };
    }
    return res;
}

/// Close the database connection associated with p
extern "C" fn close_db(p: &mut SQLiteRsync) -> () {
    if !((*p).db).is_null() {
        unsafe { sqlite3_close((*p).db) };
        (*p).db = core::ptr::null_mut();
    }
}

///* Run the origin-side protocol.
///*
///* Begin by sending the ORIGIN_BEGIN message with two arguments,
///* nPage, and szPage.  Then enter a loop responding to message from
///* the replica:
///*
///*    REPLICA_BEGIN  iProtocol
///*
///*         An optional message sent by the replica in response to the
///*         prior ORIGIN_BEGIN with a counter-proposal for the protocol
///*         level.  If seen, try to reduce the protocol level to what is
///*         requested and send a new ORGIN_BEGIN.
///*
///*    REPLICA_ERROR  size  text
///*
///*         Report an error from the replica and quit
///*
///*    REPLICA_END
///*
///*         The replica is terminating.  Stop processing now.
///*
///*    REPLICA_HASH  hash
///*
///*         The argument is the 20-byte SHA1 hash for the next page or
///*         block of pages.  Hashes appear in sequential order with no gaps,
///*         unless there is an intervening REPLICA_CONFIG message.
///*
///*    REPLICA_CONFIG   pgno   cnt
///*
///*         Set counters used by REPLICA_HASH.  The next hash will start
///*         on page pgno and all subsequent hashes will cover cnt pages
///*         each.  Note that for a multi-page hash, the hash value is
///*         actually a hash of the individual page hashes.
///*
///*    REPLICA_READY
///*
///*         The replica has sent all the hashes that it intends to send.
///*         This side (the origin) can now start responding with page
///*         content for pages that do not have a matching hash or with
///*         ORIGIN_DETAIL messages with requests for more detail.
#[allow(unused_doc_comments)]
extern "C" fn origin_side(p: *mut SQLiteRsync) -> () {
    let mut rc: i32 = 0;
    let mut c: i32 = 0;
    let mut n_page: u32 = 0 as u32;
    let mut i_hash: u32 = 1 as u32;
    /// Pgno for next hash to receive
    let mut n_hash: u32 = 1 as u32;
    /// Number of pages per hash received
    let mut mx_hash: u32 = 0 as u32;
    /// Maximum hash value received
    let mut lock_byte_page: u32 = 0 as u32;
    let mut sz_pg: u32 = 0 as u32;
    let mut p_ck_hash: *mut Sqlite3Stmt = core::ptr::null_mut();
    /// Verify hash on a single page
    let mut p_ck_hash_n: *mut Sqlite3Stmt = core::ptr::null_mut();
    /// Verify a multi-page hash
    let mut p_ins_hash: *mut Sqlite3Stmt = core::ptr::null_mut();
    /// Record a bad hash
    let mut buf: [i8; 200] = [0; 200];
    unsafe { (*p).is_replica = 0 as u8 };
    if unsafe { (*p).b_comm_check } != 0 {
        unsafe {
            info_msg(p,
                c"origin  zOrigin=%Q zReplica=%Q isRemote=%d protocol=%d".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_origin },
                unsafe { (*p).z_replica }, unsafe { (*p).is_remote } as i32,
                unsafe { (*p).i_protocol } as i32)
        };
        write_byte(unsafe { &mut *p }, 66);
        unsafe { fflush(unsafe { (*p).p_out }) };
    } else {

        /// Open the ORIGIN database.
        (rc =
            unsafe {
                sqlite3_open_v2(unsafe { (*p).z_origin },
                    unsafe { &mut (*p).db }, 2, core::ptr::null())
            });
        if rc != 0 {
            unsafe {
                report_error(p,
                    c"cannot open origin \"%s\": %s".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p).z_origin },
                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
            };
            close_db(unsafe { &mut *p });
            return;
        }
        hash_register(unsafe { (*p).db });
        unsafe { run_sql(p, c"BEGIN".as_ptr() as *mut i8) };
        if unsafe { (*p).b_wal_only } != 0 {
            unsafe {
                run_sql_return_text(p, &raw mut buf[0 as usize] as *mut i8,
                    c"PRAGMA journal_mode".as_ptr() as *mut i8)
            };
            if unsafe {
                        sqlite3_stricmp(&raw mut buf[0 as usize] as *mut i8 as
                                *const i8, c"wal".as_ptr() as *mut i8 as *const i8)
                    } != 0 {
                unsafe {
                    report_error(p,
                        c"Origin database is not in WAL mode".as_ptr() as *mut i8 as
                            *const i8)
                };
            }
        }
        unsafe {
            run_sql_return_u_int(p, &mut n_page,
                c"PRAGMA page_count".as_ptr() as *mut i8)
        };
        unsafe {
            run_sql_return_u_int(p, &mut sz_pg,
                c"PRAGMA page_size".as_ptr() as *mut i8)
        };
        if unsafe { (*p).n_err } == 0 {

            /// Send the ORIGIN_BEGIN message
            write_byte(unsafe { &mut *p }, 65);
            write_byte(unsafe { &mut *p }, unsafe { (*p).i_protocol } as i32);
            write_pow2(p, sz_pg as i32);
            write_uint32(p, n_page);
            unsafe { fflush(unsafe { (*p).p_out }) };
            if !(unsafe { (*p).z_debug_file }).is_null() {
                unsafe {
                    debug_message(unsafe { &mut *p },
                        c"-> ORIGIN_BEGIN %u %u %u\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { (*p).i_protocol } as i32, sz_pg, n_page)
                };
            }
            unsafe { (*p).n_page = n_page };
            unsafe { (*p).sz_page = sz_pg };
            lock_byte_page = (1 << 30) as u32 / sz_pg + 1 as u32;
        }
    }
    while unsafe { (*p).n_err } <= unsafe { (*p).n_wr_err } &&
                { c = read_byte(unsafe { &mut *p }); c } != -1 && c != 99 {
        '__s9:
            {
            match c {
                97 => {
                    {
                        /// This message is only sent if the replica received an origin-protocol
                        ///* that is larger than what it knows about.  The replica sends back
                        ///* a counter-proposal of an earlier protocol which the origin can
                        ///* accept by resending a new ORIGIN_BEGIN.
                        let new_protocol: u8 = read_byte(unsafe { &mut *p }) as u8;
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_BEGIN %d\n".as_ptr() as *mut i8 as *const i8,
                                    new_protocol as i32)
                            };
                        }
                        if (new_protocol as i32) < unsafe { (*p).i_protocol } as i32
                            {
                            unsafe { (*p).i_protocol = new_protocol };
                            write_byte(unsafe { &mut *p }, 65);
                            write_byte(unsafe { &mut *p },
                                unsafe { (*p).i_protocol } as i32);
                            write_pow2(p, unsafe { (*p).sz_page } as i32);
                            write_uint32(p, unsafe { (*p).n_page });
                            unsafe { fflush(unsafe { (*p).p_out }) };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_BEGIN %d %d %u\n".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p).i_protocol } as i32,
                                        unsafe { (*p).sz_page }, unsafe { (*p).n_page })
                                };
                            }
                        } else {
                            unsafe {
                                report_error(p,
                                    c"Invalid REPLICA_BEGIN reply".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                        }
                        break '__s9;
                    }
                    { read_and_display_message(p, c); break '__s9; }
                    {
                        read_uint32(p, &mut i_hash);
                        read_uint32(p, &mut n_hash);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_CONFIG %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash)
                            };
                        }
                        break '__s9;
                    }
                    {
                        let mut b_match: i32 = 0;
                        if p_ck_hash == core::ptr::null_mut() {
                            unsafe {
                                run_sql(p,
                                    c"CREATE TEMP TABLE badHash( pgno INTEGER PRIMARY KEY, sz INT)".as_ptr()
                                        as *mut i8)
                            };
                            p_ck_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT hash(data)==?3 FROM sqlite_dbpage(\'main\') WHERE pgno=?1".as_ptr()
                                            as *mut i8)
                                };
                            if p_ck_hash == core::ptr::null_mut() { break '__s9; }
                            p_ins_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO badHash VALUES(?1,?2)".as_ptr() as *mut i8)
                                };
                            if p_ins_hash == core::ptr::null_mut() { break '__s9; }
                        }
                        {
                            let __p = unsafe { &mut (*p).n_hash_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else { core::slice::from_raw_parts_mut(__p, 20 as usize) }
                            });
                        if n_hash > 1 as u32 {
                            if p_ck_hash_n == core::ptr::null_mut() {
                                p_ck_hash_n =
                                    unsafe {
                                        prepare_stmt(p,
                                            c"WITH c(n) AS   (VALUES(?1) UNION ALL SELECT n+1 FROM c WHERE n<?2)SELECT agghash(hash(data))==?3 FROM c CROSS JOIN sqlite_dbpage(\'main\') ON pgno=n".as_ptr()
                                                as *mut i8)
                                    };
                                if p_ck_hash_n == core::ptr::null_mut() { break '__s9; }
                            }
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 2,
                                    (i_hash + n_hash - 1 as u32) as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash_n, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash_n) };
                            if rc == 100 {
                                b_match = unsafe { sqlite3_column_int(p_ck_hash_n, 0) };
                            } else if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash_n) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ck_hash_n) };
                        } else {
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash) };
                            if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            } else if rc == 100 &&
                                    unsafe { sqlite3_column_int(p_ck_hash, 0) } != 0 {
                                b_match = 1;
                            }
                            unsafe { sqlite3_reset(p_ck_hash) };
                        }
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_HASH %u %u %s %08x...\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash,
                                    if b_match != 0 {
                                        c"match".as_ptr() as *mut i8
                                    } else { c"fail".as_ptr() as *mut i8 },
                                    unsafe { *(&raw mut buf[0 as usize] as *mut u32) })
                            };
                        }
                        if (b_match == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 2, n_hash as Sqlite3Int64)
                            };
                            rc = unsafe { sqlite3_step(p_ins_hash) };
                            if rc != 101 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ins_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ins_hash) };
                        }
                        if i_hash + n_hash > mx_hash { mx_hash = i_hash + n_hash; }
                        i_hash += n_hash;
                        break '__s9;
                    }
                    {
                        let mut n_multi: i32 = 0;
                        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p).n_round };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        p_stmt =
                            unsafe {
                                prepare_stmt(p,
                                    c"SELECT pgno, sz FROM badHash WHERE sz>1".as_ptr() as
                                        *mut i8)
                            };
                        if p_stmt == core::ptr::null_mut() { break '__s9; }
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                            let cnt: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
                            write_byte(unsafe { &mut *p }, 71);
                            write_uint32(p, pgno);
                            write_uint32(p, cnt);
                            { let __p = &mut n_multi; let __t = *__p; *__p += 1; __t };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                            *const i8, pgno, cnt)
                                };
                            }
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                        if n_multi != 0 {
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE sz>1".as_ptr() as *mut i8)
                            };
                            write_byte(unsafe { &mut *p }, 72);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                        } else {
                            unsafe { sqlite3_finalize(p_ck_hash) };
                            unsafe { sqlite3_finalize(p_ck_hash_n) };
                            unsafe { sqlite3_finalize(p_ins_hash) };
                            p_ck_hash = core::ptr::null_mut();
                            p_ins_hash = core::ptr::null_mut();
                            if mx_hash <= unsafe { (*p).n_page } {
                                unsafe {
                                    run_sql(p,
                                        c"WITH RECURSIVE c(n) AS (VALUES(%d) UNION ALL SELECT n+1 FROM c WHERE n<%d) INSERT INTO badHash SELECT n, 1 FROM c".as_ptr()
                                            as *mut i8, mx_hash, unsafe { (*p).n_page })
                                };
                            }
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE pgno=%d".as_ptr() as *mut i8,
                                    lock_byte_page)
                            };
                            p_stmt =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT pgno, data  FROM badHash JOIN sqlite_dbpage(\'main\') USING(pgno)".as_ptr()
                                            as *mut i8)
                                };
                            if p_stmt == core::ptr::null_mut() { break '__s9; }
                            while unsafe { sqlite3_step(p_stmt) } == 100 &&
                                        unsafe { (*p).n_err } == 0 && unsafe { (*p).n_wr_err } == 0
                                {
                                let pgno: u32 =
                                    unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                                let p_content: *const () =
                                    unsafe { sqlite3_column_blob(p_stmt, 1) };
                                write_byte(unsafe { &mut *p }, 68);
                                write_uint32(p, pgno);
                                write_bytes(p,
                                    unsafe {
                                        let __p = p_content as *const u8 as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, sz_pg as usize) }
                                    });
                                {
                                    let __p = unsafe { &mut (*p).n_page_sent };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                if !(unsafe { (*p).z_debug_file }).is_null() {
                                    unsafe {
                                        debug_message(unsafe { &mut *p },
                                            c"-> ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                            pgno)
                                    };
                                }
                            }
                            unsafe { sqlite3_finalize(p_stmt) };
                            write_byte(unsafe { &mut *p }, 69);
                            write_uint32(p, n_page);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                        n_page)
                                };
                            }
                            write_byte(unsafe { &mut *p }, 66);
                        }
                        unsafe { fflush(unsafe { (*p).p_out }) };
                        break '__s9;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
                102 => {
                    { read_and_display_message(p, c); break '__s9; }
                    {
                        read_uint32(p, &mut i_hash);
                        read_uint32(p, &mut n_hash);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_CONFIG %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash)
                            };
                        }
                        break '__s9;
                    }
                    {
                        let mut b_match: i32 = 0;
                        if p_ck_hash == core::ptr::null_mut() {
                            unsafe {
                                run_sql(p,
                                    c"CREATE TEMP TABLE badHash( pgno INTEGER PRIMARY KEY, sz INT)".as_ptr()
                                        as *mut i8)
                            };
                            p_ck_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT hash(data)==?3 FROM sqlite_dbpage(\'main\') WHERE pgno=?1".as_ptr()
                                            as *mut i8)
                                };
                            if p_ck_hash == core::ptr::null_mut() { break '__s9; }
                            p_ins_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO badHash VALUES(?1,?2)".as_ptr() as *mut i8)
                                };
                            if p_ins_hash == core::ptr::null_mut() { break '__s9; }
                        }
                        {
                            let __p = unsafe { &mut (*p).n_hash_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else { core::slice::from_raw_parts_mut(__p, 20 as usize) }
                            });
                        if n_hash > 1 as u32 {
                            if p_ck_hash_n == core::ptr::null_mut() {
                                p_ck_hash_n =
                                    unsafe {
                                        prepare_stmt(p,
                                            c"WITH c(n) AS   (VALUES(?1) UNION ALL SELECT n+1 FROM c WHERE n<?2)SELECT agghash(hash(data))==?3 FROM c CROSS JOIN sqlite_dbpage(\'main\') ON pgno=n".as_ptr()
                                                as *mut i8)
                                    };
                                if p_ck_hash_n == core::ptr::null_mut() { break '__s9; }
                            }
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 2,
                                    (i_hash + n_hash - 1 as u32) as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash_n, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash_n) };
                            if rc == 100 {
                                b_match = unsafe { sqlite3_column_int(p_ck_hash_n, 0) };
                            } else if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash_n) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ck_hash_n) };
                        } else {
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash) };
                            if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            } else if rc == 100 &&
                                    unsafe { sqlite3_column_int(p_ck_hash, 0) } != 0 {
                                b_match = 1;
                            }
                            unsafe { sqlite3_reset(p_ck_hash) };
                        }
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_HASH %u %u %s %08x...\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash,
                                    if b_match != 0 {
                                        c"match".as_ptr() as *mut i8
                                    } else { c"fail".as_ptr() as *mut i8 },
                                    unsafe { *(&raw mut buf[0 as usize] as *mut u32) })
                            };
                        }
                        if (b_match == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 2, n_hash as Sqlite3Int64)
                            };
                            rc = unsafe { sqlite3_step(p_ins_hash) };
                            if rc != 101 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ins_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ins_hash) };
                        }
                        if i_hash + n_hash > mx_hash { mx_hash = i_hash + n_hash; }
                        i_hash += n_hash;
                        break '__s9;
                    }
                    {
                        let mut n_multi: i32 = 0;
                        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p).n_round };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        p_stmt =
                            unsafe {
                                prepare_stmt(p,
                                    c"SELECT pgno, sz FROM badHash WHERE sz>1".as_ptr() as
                                        *mut i8)
                            };
                        if p_stmt == core::ptr::null_mut() { break '__s9; }
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                            let cnt: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
                            write_byte(unsafe { &mut *p }, 71);
                            write_uint32(p, pgno);
                            write_uint32(p, cnt);
                            { let __p = &mut n_multi; let __t = *__p; *__p += 1; __t };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                            *const i8, pgno, cnt)
                                };
                            }
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                        if n_multi != 0 {
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE sz>1".as_ptr() as *mut i8)
                            };
                            write_byte(unsafe { &mut *p }, 72);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                        } else {
                            unsafe { sqlite3_finalize(p_ck_hash) };
                            unsafe { sqlite3_finalize(p_ck_hash_n) };
                            unsafe { sqlite3_finalize(p_ins_hash) };
                            p_ck_hash = core::ptr::null_mut();
                            p_ins_hash = core::ptr::null_mut();
                            if mx_hash <= unsafe { (*p).n_page } {
                                unsafe {
                                    run_sql(p,
                                        c"WITH RECURSIVE c(n) AS (VALUES(%d) UNION ALL SELECT n+1 FROM c WHERE n<%d) INSERT INTO badHash SELECT n, 1 FROM c".as_ptr()
                                            as *mut i8, mx_hash, unsafe { (*p).n_page })
                                };
                            }
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE pgno=%d".as_ptr() as *mut i8,
                                    lock_byte_page)
                            };
                            p_stmt =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT pgno, data  FROM badHash JOIN sqlite_dbpage(\'main\') USING(pgno)".as_ptr()
                                            as *mut i8)
                                };
                            if p_stmt == core::ptr::null_mut() { break '__s9; }
                            while unsafe { sqlite3_step(p_stmt) } == 100 &&
                                        unsafe { (*p).n_err } == 0 && unsafe { (*p).n_wr_err } == 0
                                {
                                let pgno: u32 =
                                    unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                                let p_content: *const () =
                                    unsafe { sqlite3_column_blob(p_stmt, 1) };
                                write_byte(unsafe { &mut *p }, 68);
                                write_uint32(p, pgno);
                                write_bytes(p,
                                    unsafe {
                                        let __p = p_content as *const u8 as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, sz_pg as usize) }
                                    });
                                {
                                    let __p = unsafe { &mut (*p).n_page_sent };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                if !(unsafe { (*p).z_debug_file }).is_null() {
                                    unsafe {
                                        debug_message(unsafe { &mut *p },
                                            c"-> ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                            pgno)
                                    };
                                }
                            }
                            unsafe { sqlite3_finalize(p_stmt) };
                            write_byte(unsafe { &mut *p }, 69);
                            write_uint32(p, n_page);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                        n_page)
                                };
                            }
                            write_byte(unsafe { &mut *p }, 66);
                        }
                        unsafe { fflush(unsafe { (*p).p_out }) };
                        break '__s9;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
                98 => {
                    { read_and_display_message(p, c); break '__s9; }
                    {
                        read_uint32(p, &mut i_hash);
                        read_uint32(p, &mut n_hash);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_CONFIG %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash)
                            };
                        }
                        break '__s9;
                    }
                    {
                        let mut b_match: i32 = 0;
                        if p_ck_hash == core::ptr::null_mut() {
                            unsafe {
                                run_sql(p,
                                    c"CREATE TEMP TABLE badHash( pgno INTEGER PRIMARY KEY, sz INT)".as_ptr()
                                        as *mut i8)
                            };
                            p_ck_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT hash(data)==?3 FROM sqlite_dbpage(\'main\') WHERE pgno=?1".as_ptr()
                                            as *mut i8)
                                };
                            if p_ck_hash == core::ptr::null_mut() { break '__s9; }
                            p_ins_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO badHash VALUES(?1,?2)".as_ptr() as *mut i8)
                                };
                            if p_ins_hash == core::ptr::null_mut() { break '__s9; }
                        }
                        {
                            let __p = unsafe { &mut (*p).n_hash_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else { core::slice::from_raw_parts_mut(__p, 20 as usize) }
                            });
                        if n_hash > 1 as u32 {
                            if p_ck_hash_n == core::ptr::null_mut() {
                                p_ck_hash_n =
                                    unsafe {
                                        prepare_stmt(p,
                                            c"WITH c(n) AS   (VALUES(?1) UNION ALL SELECT n+1 FROM c WHERE n<?2)SELECT agghash(hash(data))==?3 FROM c CROSS JOIN sqlite_dbpage(\'main\') ON pgno=n".as_ptr()
                                                as *mut i8)
                                    };
                                if p_ck_hash_n == core::ptr::null_mut() { break '__s9; }
                            }
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 2,
                                    (i_hash + n_hash - 1 as u32) as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash_n, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash_n) };
                            if rc == 100 {
                                b_match = unsafe { sqlite3_column_int(p_ck_hash_n, 0) };
                            } else if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash_n) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ck_hash_n) };
                        } else {
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash) };
                            if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            } else if rc == 100 &&
                                    unsafe { sqlite3_column_int(p_ck_hash, 0) } != 0 {
                                b_match = 1;
                            }
                            unsafe { sqlite3_reset(p_ck_hash) };
                        }
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_HASH %u %u %s %08x...\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash,
                                    if b_match != 0 {
                                        c"match".as_ptr() as *mut i8
                                    } else { c"fail".as_ptr() as *mut i8 },
                                    unsafe { *(&raw mut buf[0 as usize] as *mut u32) })
                            };
                        }
                        if (b_match == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 2, n_hash as Sqlite3Int64)
                            };
                            rc = unsafe { sqlite3_step(p_ins_hash) };
                            if rc != 101 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ins_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ins_hash) };
                        }
                        if i_hash + n_hash > mx_hash { mx_hash = i_hash + n_hash; }
                        i_hash += n_hash;
                        break '__s9;
                    }
                    {
                        let mut n_multi: i32 = 0;
                        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p).n_round };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        p_stmt =
                            unsafe {
                                prepare_stmt(p,
                                    c"SELECT pgno, sz FROM badHash WHERE sz>1".as_ptr() as
                                        *mut i8)
                            };
                        if p_stmt == core::ptr::null_mut() { break '__s9; }
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                            let cnt: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
                            write_byte(unsafe { &mut *p }, 71);
                            write_uint32(p, pgno);
                            write_uint32(p, cnt);
                            { let __p = &mut n_multi; let __t = *__p; *__p += 1; __t };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                            *const i8, pgno, cnt)
                                };
                            }
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                        if n_multi != 0 {
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE sz>1".as_ptr() as *mut i8)
                            };
                            write_byte(unsafe { &mut *p }, 72);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                        } else {
                            unsafe { sqlite3_finalize(p_ck_hash) };
                            unsafe { sqlite3_finalize(p_ck_hash_n) };
                            unsafe { sqlite3_finalize(p_ins_hash) };
                            p_ck_hash = core::ptr::null_mut();
                            p_ins_hash = core::ptr::null_mut();
                            if mx_hash <= unsafe { (*p).n_page } {
                                unsafe {
                                    run_sql(p,
                                        c"WITH RECURSIVE c(n) AS (VALUES(%d) UNION ALL SELECT n+1 FROM c WHERE n<%d) INSERT INTO badHash SELECT n, 1 FROM c".as_ptr()
                                            as *mut i8, mx_hash, unsafe { (*p).n_page })
                                };
                            }
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE pgno=%d".as_ptr() as *mut i8,
                                    lock_byte_page)
                            };
                            p_stmt =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT pgno, data  FROM badHash JOIN sqlite_dbpage(\'main\') USING(pgno)".as_ptr()
                                            as *mut i8)
                                };
                            if p_stmt == core::ptr::null_mut() { break '__s9; }
                            while unsafe { sqlite3_step(p_stmt) } == 100 &&
                                        unsafe { (*p).n_err } == 0 && unsafe { (*p).n_wr_err } == 0
                                {
                                let pgno: u32 =
                                    unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                                let p_content: *const () =
                                    unsafe { sqlite3_column_blob(p_stmt, 1) };
                                write_byte(unsafe { &mut *p }, 68);
                                write_uint32(p, pgno);
                                write_bytes(p,
                                    unsafe {
                                        let __p = p_content as *const u8 as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, sz_pg as usize) }
                                    });
                                {
                                    let __p = unsafe { &mut (*p).n_page_sent };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                if !(unsafe { (*p).z_debug_file }).is_null() {
                                    unsafe {
                                        debug_message(unsafe { &mut *p },
                                            c"-> ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                            pgno)
                                    };
                                }
                            }
                            unsafe { sqlite3_finalize(p_stmt) };
                            write_byte(unsafe { &mut *p }, 69);
                            write_uint32(p, n_page);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                        n_page)
                                };
                            }
                            write_byte(unsafe { &mut *p }, 66);
                        }
                        unsafe { fflush(unsafe { (*p).p_out }) };
                        break '__s9;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
                103 => {
                    {
                        read_uint32(p, &mut i_hash);
                        read_uint32(p, &mut n_hash);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_CONFIG %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash)
                            };
                        }
                        break '__s9;
                    }
                    {
                        let mut b_match: i32 = 0;
                        if p_ck_hash == core::ptr::null_mut() {
                            unsafe {
                                run_sql(p,
                                    c"CREATE TEMP TABLE badHash( pgno INTEGER PRIMARY KEY, sz INT)".as_ptr()
                                        as *mut i8)
                            };
                            p_ck_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT hash(data)==?3 FROM sqlite_dbpage(\'main\') WHERE pgno=?1".as_ptr()
                                            as *mut i8)
                                };
                            if p_ck_hash == core::ptr::null_mut() { break '__s9; }
                            p_ins_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO badHash VALUES(?1,?2)".as_ptr() as *mut i8)
                                };
                            if p_ins_hash == core::ptr::null_mut() { break '__s9; }
                        }
                        {
                            let __p = unsafe { &mut (*p).n_hash_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else { core::slice::from_raw_parts_mut(__p, 20 as usize) }
                            });
                        if n_hash > 1 as u32 {
                            if p_ck_hash_n == core::ptr::null_mut() {
                                p_ck_hash_n =
                                    unsafe {
                                        prepare_stmt(p,
                                            c"WITH c(n) AS   (VALUES(?1) UNION ALL SELECT n+1 FROM c WHERE n<?2)SELECT agghash(hash(data))==?3 FROM c CROSS JOIN sqlite_dbpage(\'main\') ON pgno=n".as_ptr()
                                                as *mut i8)
                                    };
                                if p_ck_hash_n == core::ptr::null_mut() { break '__s9; }
                            }
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 2,
                                    (i_hash + n_hash - 1 as u32) as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash_n, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash_n) };
                            if rc == 100 {
                                b_match = unsafe { sqlite3_column_int(p_ck_hash_n, 0) };
                            } else if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash_n) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ck_hash_n) };
                        } else {
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash) };
                            if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            } else if rc == 100 &&
                                    unsafe { sqlite3_column_int(p_ck_hash, 0) } != 0 {
                                b_match = 1;
                            }
                            unsafe { sqlite3_reset(p_ck_hash) };
                        }
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_HASH %u %u %s %08x...\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash,
                                    if b_match != 0 {
                                        c"match".as_ptr() as *mut i8
                                    } else { c"fail".as_ptr() as *mut i8 },
                                    unsafe { *(&raw mut buf[0 as usize] as *mut u32) })
                            };
                        }
                        if (b_match == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 2, n_hash as Sqlite3Int64)
                            };
                            rc = unsafe { sqlite3_step(p_ins_hash) };
                            if rc != 101 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ins_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ins_hash) };
                        }
                        if i_hash + n_hash > mx_hash { mx_hash = i_hash + n_hash; }
                        i_hash += n_hash;
                        break '__s9;
                    }
                    {
                        let mut n_multi: i32 = 0;
                        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p).n_round };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        p_stmt =
                            unsafe {
                                prepare_stmt(p,
                                    c"SELECT pgno, sz FROM badHash WHERE sz>1".as_ptr() as
                                        *mut i8)
                            };
                        if p_stmt == core::ptr::null_mut() { break '__s9; }
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                            let cnt: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
                            write_byte(unsafe { &mut *p }, 71);
                            write_uint32(p, pgno);
                            write_uint32(p, cnt);
                            { let __p = &mut n_multi; let __t = *__p; *__p += 1; __t };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                            *const i8, pgno, cnt)
                                };
                            }
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                        if n_multi != 0 {
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE sz>1".as_ptr() as *mut i8)
                            };
                            write_byte(unsafe { &mut *p }, 72);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                        } else {
                            unsafe { sqlite3_finalize(p_ck_hash) };
                            unsafe { sqlite3_finalize(p_ck_hash_n) };
                            unsafe { sqlite3_finalize(p_ins_hash) };
                            p_ck_hash = core::ptr::null_mut();
                            p_ins_hash = core::ptr::null_mut();
                            if mx_hash <= unsafe { (*p).n_page } {
                                unsafe {
                                    run_sql(p,
                                        c"WITH RECURSIVE c(n) AS (VALUES(%d) UNION ALL SELECT n+1 FROM c WHERE n<%d) INSERT INTO badHash SELECT n, 1 FROM c".as_ptr()
                                            as *mut i8, mx_hash, unsafe { (*p).n_page })
                                };
                            }
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE pgno=%d".as_ptr() as *mut i8,
                                    lock_byte_page)
                            };
                            p_stmt =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT pgno, data  FROM badHash JOIN sqlite_dbpage(\'main\') USING(pgno)".as_ptr()
                                            as *mut i8)
                                };
                            if p_stmt == core::ptr::null_mut() { break '__s9; }
                            while unsafe { sqlite3_step(p_stmt) } == 100 &&
                                        unsafe { (*p).n_err } == 0 && unsafe { (*p).n_wr_err } == 0
                                {
                                let pgno: u32 =
                                    unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                                let p_content: *const () =
                                    unsafe { sqlite3_column_blob(p_stmt, 1) };
                                write_byte(unsafe { &mut *p }, 68);
                                write_uint32(p, pgno);
                                write_bytes(p,
                                    unsafe {
                                        let __p = p_content as *const u8 as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, sz_pg as usize) }
                                    });
                                {
                                    let __p = unsafe { &mut (*p).n_page_sent };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                if !(unsafe { (*p).z_debug_file }).is_null() {
                                    unsafe {
                                        debug_message(unsafe { &mut *p },
                                            c"-> ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                            pgno)
                                    };
                                }
                            }
                            unsafe { sqlite3_finalize(p_stmt) };
                            write_byte(unsafe { &mut *p }, 69);
                            write_uint32(p, n_page);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                        n_page)
                                };
                            }
                            write_byte(unsafe { &mut *p }, 66);
                        }
                        unsafe { fflush(unsafe { (*p).p_out }) };
                        break '__s9;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
                100 => {
                    {
                        let mut b_match: i32 = 0;
                        if p_ck_hash == core::ptr::null_mut() {
                            unsafe {
                                run_sql(p,
                                    c"CREATE TEMP TABLE badHash( pgno INTEGER PRIMARY KEY, sz INT)".as_ptr()
                                        as *mut i8)
                            };
                            p_ck_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT hash(data)==?3 FROM sqlite_dbpage(\'main\') WHERE pgno=?1".as_ptr()
                                            as *mut i8)
                                };
                            if p_ck_hash == core::ptr::null_mut() { break '__s9; }
                            p_ins_hash =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO badHash VALUES(?1,?2)".as_ptr() as *mut i8)
                                };
                            if p_ins_hash == core::ptr::null_mut() { break '__s9; }
                        }
                        {
                            let __p = unsafe { &mut (*p).n_hash_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else { core::slice::from_raw_parts_mut(__p, 20 as usize) }
                            });
                        if n_hash > 1 as u32 {
                            if p_ck_hash_n == core::ptr::null_mut() {
                                p_ck_hash_n =
                                    unsafe {
                                        prepare_stmt(p,
                                            c"WITH c(n) AS   (VALUES(?1) UNION ALL SELECT n+1 FROM c WHERE n<?2)SELECT agghash(hash(data))==?3 FROM c CROSS JOIN sqlite_dbpage(\'main\') ON pgno=n".as_ptr()
                                                as *mut i8)
                                    };
                                if p_ck_hash_n == core::ptr::null_mut() { break '__s9; }
                            }
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash_n, 2,
                                    (i_hash + n_hash - 1 as u32) as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash_n, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash_n) };
                            if rc == 100 {
                                b_match = unsafe { sqlite3_column_int(p_ck_hash_n, 0) };
                            } else if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash_n) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ck_hash_n) };
                        } else {
                            unsafe {
                                sqlite3_bind_int64(p_ck_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_blob(p_ck_hash, 3,
                                    &raw mut buf[0 as usize] as *mut i8 as *const (), 20, None)
                            };
                            rc = unsafe { sqlite3_step(p_ck_hash) };
                            if rc == 1 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ck_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            } else if rc == 100 &&
                                    unsafe { sqlite3_column_int(p_ck_hash, 0) } != 0 {
                                b_match = 1;
                            }
                            unsafe { sqlite3_reset(p_ck_hash) };
                        }
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_HASH %u %u %s %08x...\n".as_ptr() as *mut i8 as
                                        *const i8, i_hash, n_hash,
                                    if b_match != 0 {
                                        c"match".as_ptr() as *mut i8
                                    } else { c"fail".as_ptr() as *mut i8 },
                                    unsafe { *(&raw mut buf[0 as usize] as *mut u32) })
                            };
                        }
                        if (b_match == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 1, i_hash as Sqlite3Int64)
                            };
                            unsafe {
                                sqlite3_bind_int64(p_ins_hash, 2, n_hash as Sqlite3Int64)
                            };
                            rc = unsafe { sqlite3_step(p_ins_hash) };
                            if rc != 101 {
                                unsafe {
                                    report_error(p,
                                        c"SQL statement [%s] failed: %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { sqlite3_sql(p_ins_hash) },
                                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                };
                            }
                            unsafe { sqlite3_reset(p_ins_hash) };
                        }
                        if i_hash + n_hash > mx_hash { mx_hash = i_hash + n_hash; }
                        i_hash += n_hash;
                        break '__s9;
                    }
                    {
                        let mut n_multi: i32 = 0;
                        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p).n_round };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        p_stmt =
                            unsafe {
                                prepare_stmt(p,
                                    c"SELECT pgno, sz FROM badHash WHERE sz>1".as_ptr() as
                                        *mut i8)
                            };
                        if p_stmt == core::ptr::null_mut() { break '__s9; }
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                            let cnt: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
                            write_byte(unsafe { &mut *p }, 71);
                            write_uint32(p, pgno);
                            write_uint32(p, cnt);
                            { let __p = &mut n_multi; let __t = *__p; *__p += 1; __t };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                            *const i8, pgno, cnt)
                                };
                            }
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                        if n_multi != 0 {
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE sz>1".as_ptr() as *mut i8)
                            };
                            write_byte(unsafe { &mut *p }, 72);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                        } else {
                            unsafe { sqlite3_finalize(p_ck_hash) };
                            unsafe { sqlite3_finalize(p_ck_hash_n) };
                            unsafe { sqlite3_finalize(p_ins_hash) };
                            p_ck_hash = core::ptr::null_mut();
                            p_ins_hash = core::ptr::null_mut();
                            if mx_hash <= unsafe { (*p).n_page } {
                                unsafe {
                                    run_sql(p,
                                        c"WITH RECURSIVE c(n) AS (VALUES(%d) UNION ALL SELECT n+1 FROM c WHERE n<%d) INSERT INTO badHash SELECT n, 1 FROM c".as_ptr()
                                            as *mut i8, mx_hash, unsafe { (*p).n_page })
                                };
                            }
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE pgno=%d".as_ptr() as *mut i8,
                                    lock_byte_page)
                            };
                            p_stmt =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT pgno, data  FROM badHash JOIN sqlite_dbpage(\'main\') USING(pgno)".as_ptr()
                                            as *mut i8)
                                };
                            if p_stmt == core::ptr::null_mut() { break '__s9; }
                            while unsafe { sqlite3_step(p_stmt) } == 100 &&
                                        unsafe { (*p).n_err } == 0 && unsafe { (*p).n_wr_err } == 0
                                {
                                let pgno: u32 =
                                    unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                                let p_content: *const () =
                                    unsafe { sqlite3_column_blob(p_stmt, 1) };
                                write_byte(unsafe { &mut *p }, 68);
                                write_uint32(p, pgno);
                                write_bytes(p,
                                    unsafe {
                                        let __p = p_content as *const u8 as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, sz_pg as usize) }
                                    });
                                {
                                    let __p = unsafe { &mut (*p).n_page_sent };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                if !(unsafe { (*p).z_debug_file }).is_null() {
                                    unsafe {
                                        debug_message(unsafe { &mut *p },
                                            c"-> ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                            pgno)
                                    };
                                }
                            }
                            unsafe { sqlite3_finalize(p_stmt) };
                            write_byte(unsafe { &mut *p }, 69);
                            write_uint32(p, n_page);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                        n_page)
                                };
                            }
                            write_byte(unsafe { &mut *p }, 66);
                        }
                        unsafe { fflush(unsafe { (*p).p_out }) };
                        break '__s9;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
                101 => {
                    {
                        let mut n_multi: i32 = 0;
                        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- REPLICA_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        {
                            let __p = unsafe { &mut (*p).n_round };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        p_stmt =
                            unsafe {
                                prepare_stmt(p,
                                    c"SELECT pgno, sz FROM badHash WHERE sz>1".as_ptr() as
                                        *mut i8)
                            };
                        if p_stmt == core::ptr::null_mut() { break '__s9; }
                        while unsafe { sqlite3_step(p_stmt) } == 100 {
                            let pgno: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                            let cnt: u32 =
                                unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
                            write_byte(unsafe { &mut *p }, 71);
                            write_uint32(p, pgno);
                            write_uint32(p, cnt);
                            { let __p = &mut n_multi; let __t = *__p; *__p += 1; __t };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                            *const i8, pgno, cnt)
                                };
                            }
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                        if n_multi != 0 {
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE sz>1".as_ptr() as *mut i8)
                            };
                            write_byte(unsafe { &mut *p }, 72);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                        } else {
                            unsafe { sqlite3_finalize(p_ck_hash) };
                            unsafe { sqlite3_finalize(p_ck_hash_n) };
                            unsafe { sqlite3_finalize(p_ins_hash) };
                            p_ck_hash = core::ptr::null_mut();
                            p_ins_hash = core::ptr::null_mut();
                            if mx_hash <= unsafe { (*p).n_page } {
                                unsafe {
                                    run_sql(p,
                                        c"WITH RECURSIVE c(n) AS (VALUES(%d) UNION ALL SELECT n+1 FROM c WHERE n<%d) INSERT INTO badHash SELECT n, 1 FROM c".as_ptr()
                                            as *mut i8, mx_hash, unsafe { (*p).n_page })
                                };
                            }
                            unsafe {
                                run_sql(p,
                                    c"DELETE FROM badHash WHERE pgno=%d".as_ptr() as *mut i8,
                                    lock_byte_page)
                            };
                            p_stmt =
                                unsafe {
                                    prepare_stmt(p,
                                        c"SELECT pgno, data  FROM badHash JOIN sqlite_dbpage(\'main\') USING(pgno)".as_ptr()
                                            as *mut i8)
                                };
                            if p_stmt == core::ptr::null_mut() { break '__s9; }
                            while unsafe { sqlite3_step(p_stmt) } == 100 &&
                                        unsafe { (*p).n_err } == 0 && unsafe { (*p).n_wr_err } == 0
                                {
                                let pgno: u32 =
                                    unsafe { sqlite3_column_int64(p_stmt, 0) } as u32;
                                let p_content: *const () =
                                    unsafe { sqlite3_column_blob(p_stmt, 1) };
                                write_byte(unsafe { &mut *p }, 68);
                                write_uint32(p, pgno);
                                write_bytes(p,
                                    unsafe {
                                        let __p = p_content as *const u8 as *const u8;
                                        if __p.is_null() {
                                            &[]
                                        } else { core::slice::from_raw_parts(__p, sz_pg as usize) }
                                    });
                                {
                                    let __p = unsafe { &mut (*p).n_page_sent };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                if !(unsafe { (*p).z_debug_file }).is_null() {
                                    unsafe {
                                        debug_message(unsafe { &mut *p },
                                            c"-> ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                            pgno)
                                    };
                                }
                            }
                            unsafe { sqlite3_finalize(p_stmt) };
                            write_byte(unsafe { &mut *p }, 69);
                            write_uint32(p, n_page);
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                        n_page)
                                };
                            }
                            write_byte(unsafe { &mut *p }, 66);
                        }
                        unsafe { fflush(unsafe { (*p).p_out }) };
                        break '__s9;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
                _ => {
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s9;
                    }
                }
            }
        }
    }
    if !(p_ck_hash).is_null() { unsafe { sqlite3_finalize(p_ck_hash) }; }
    if !(p_ins_hash).is_null() { unsafe { sqlite3_finalize(p_ins_hash) }; }
    close_db(unsafe { &mut *p });
}

///* Send a REPLICA_HASH message for each entry in the sendHash table.
///* The sendHash table looks like this:
///*
///*   CREATE TABLE sendHash(
///*      fpg INTEGER PRIMARY KEY,   -- Page number of the hash
///*      npg INT                    -- Number of pages in this hash
///*   );
///*
///* If iHash is page number for the next page that the origin will
///* be expecting, and nHash is the number of pages that the origin will
///* be expecting in the hash that follows.  Send a REPLICA_CONFIG message
///* if either of these values if not correct.
extern "C" fn send_hash_messages(p: *mut SQLiteRsync, mut i_hash_1: u32,
    mut n_hash_1: u32) -> () {
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    p_stmt =
        unsafe {
            prepare_stmt(p,
                c"SELECT if(npg==1,  (SELECT hash(data) FROM sqlite_dbpage(\'replica\') WHERE pgno=fpg),  (WITH RECURSIVE c(n) AS     (SELECT fpg UNION ALL SELECT n+1 FROM c WHERE n<fpg+npg-1)   SELECT agghash(hash(data))     FROM c CROSS JOIN sqlite_dbpage(\'replica\') ON pgno=n)) AS hash,  fpg,  npg FROM sendHash ORDER BY fpg".as_ptr()
                    as *mut i8)
        };
    while unsafe { sqlite3_step(p_stmt) } == 100 && unsafe { (*p).n_err } == 0
            && unsafe { (*p).n_wr_err } == 0 {
        let a: *const u8 =
            unsafe { sqlite3_column_blob(p_stmt, 0) } as *const u8;
        let pgno: u32 = unsafe { sqlite3_column_int64(p_stmt, 1) } as u32;
        let npg: u32 = unsafe { sqlite3_column_int64(p_stmt, 2) } as u32;
        if pgno != i_hash_1 || npg != n_hash_1 {
            write_byte(unsafe { &mut *p }, 103);
            write_uint32(p, pgno);
            write_uint32(p, npg);
            if !(unsafe { (*p).z_debug_file }).is_null() {
                unsafe {
                    debug_message(unsafe { &mut *p },
                        c"-> REPLICA_CONFIG %u %u\n".as_ptr() as *mut i8 as
                            *const i8, pgno, npg)
                };
            }
        }
        if a == core::ptr::null() {
            if !(unsafe { (*p).z_debug_file }).is_null() {
                unsafe {
                    debug_message(unsafe { &mut *p },
                        c"# Oops: No hash for %u %u\n".as_ptr() as *mut i8 as
                            *const i8, pgno, npg)
                };
            }
        } else {
            write_byte(unsafe { &mut *p }, 100);
            write_bytes(p,
                unsafe {
                    let __p = a as *const u8 as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, 20 as usize) }
                });
            if !(unsafe { (*p).z_debug_file }).is_null() {
                unsafe {
                    debug_message(unsafe { &mut *p },
                        c"-> REPLICA_HASH %u %u (%08x...)\n".as_ptr() as *mut i8 as
                            *const i8, pgno, npg, unsafe { *(a as *mut u32) })
                };
            }
        }
        {
            let __p = unsafe { &mut (*p).n_hash_sent };
            let __t = *__p;
            *__p += 1;
            __t
        };
        i_hash_1 = pgno + npg;
        n_hash_1 = npg;
    }
    unsafe { sqlite3_finalize(p_stmt) };
    unsafe { run_sql(p, c"DELETE FROM sendHash".as_ptr() as *mut i8) };
    write_byte(unsafe { &mut *p }, 101);
    unsafe { fflush(unsafe { (*p).p_out }) };
    {
        let __p = unsafe { &mut (*p).n_round };
        let __t = *__p;
        *__p += 1;
        __t
    };
    if !(unsafe { (*p).z_debug_file }).is_null() {
        unsafe {
            debug_message(unsafe { &mut *p },
                c"-> REPLICA_READY\n".as_ptr() as *mut i8 as *const i8,
                i_hash_1)
        };
    }
}

///* Make entries in the sendHash table to send hashes for
///* npg (mnemonic: Number of PaGes) pages starting with fpg
///* (mnemonic: First PaGe).
#[allow(unused_doc_comments)]
extern "C" fn subdivide_hash_range(p: *mut SQLiteRsync, fpg: u32, npg: u32)
    -> () {
    let mut n_chunk: u32 = 0 as u32;
    /// How many pages to request per hash
    let mut i_end: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    if npg <= 30 as u32 {
        n_chunk = 1 as u32;
    } else if npg <= 1000 as u32 {
        n_chunk = 30 as u32;
    } else { n_chunk = 1000 as u32; }
    i_end = fpg as Sqlite3Uint64;
    i_end += npg as Sqlite3Uint64;
    unsafe {
        run_sql(p,
            c"WITH RECURSIVE c(n) AS  (VALUES(%u) UNION ALL SELECT n+%u FROM c WHERE n<%llu)REPLACE INTO sendHash(fpg,npg) SELECT n, min(%llu-n,%u) FROM c".as_ptr()
                as *mut i8, fpg, n_chunk, i_end - n_chunk as Sqlite3Uint64,
            i_end, n_chunk)
    };
}

///* Run the replica-side protocol.  The protocol is passive in the sense
///* that it only response to message from the origin side.
///*
///*    ORIGIN_BEGIN  idProtocol szPage nPage
///*
///*         The origin is reporting the protocol version number, the size of
///*         each page in the origin database (sent as a single-byte power-of-2),
///*         and the number of pages in the origin database.
///*         This procedure checks compatibility, and if everything is ok,
///*         it starts sending hashes back to the origin using REPLICA_HASH
///*         and/or REPLICA_CONFIG message, followed by a single REPLICA_READY.
///*         REPLICA_CONFIG is only sent if the protocol is 2 or greater.
///*
///*    ORIGIN_ERROR  size  text
///*
///*         Report an error and quit.
///*
///*    ORIGIN_DETAIL  pgno  cnt
///*
///*         The origin reports that a multi-page hash starting at pgno and
///*         spanning cnt pages failed to match.  The origin is requesting
///*         details (more REPLICA_HASH message with a smaller cnt).  The
///*         replica must wait on ORIGIN_READY before sending its reply.
///*
///*    ORIGIN_READY
///*
///*         After sending one or more ORIGIN_DETAIL messages, the ORIGIN_READY
///*         is sent by the origin to indicate that it has finished sending
///*         requests for detail and is ready for the replicate to reply
///*         with a new round of REPLICA_CONFIG and REPLICA_HASH messages.
///*
///*    ORIGIN_PAGE  pgno  content
///*
///*         Once the origin believes it knows exactly which pages need to be
///*         updated in the replica, it starts sending those pages using these
///*         messages.  These messages will only appear immediately after
///*         REPLICA_READY.  The origin never mixes ORIGIN_DETAIL and
///*         ORIGIN_PAGE messages in the same batch.
///*
///*    ORIGIN_TXN   pgno
///*
///*         Close the update transaction.  The total database size is pgno
///*         pages.
///*
///*    ORIGIN_END
///*
///*         Expect no more transmissions from the origin.
#[allow(unused_doc_comments)]
extern "C" fn replica_side(p: *mut SQLiteRsync) -> () {
    let mut c: i32 = 0;
    let mut p_ins: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut sz_o_page: u32 = 0 as u32;
    let mut e_j_mode: i8 = 0 as i8;
    /// Journal mode prior to sync
    let mut buf: [i8; 65536] = [0; 65536];
    unsafe { (*p).is_replica = 1 as u8 };
    if unsafe { (*p).b_comm_check } != 0 {
        unsafe {
            info_msg(p,
                c"replica zOrigin=%Q zReplica=%Q isRemote=%d protocol=%d".as_ptr()
                        as *mut i8 as *const i8, unsafe { (*p).z_origin },
                unsafe { (*p).z_replica }, unsafe { (*p).is_remote } as i32,
                unsafe { (*p).i_protocol } as i32)
        };
        write_byte(unsafe { &mut *p }, 99);
        unsafe { fflush(unsafe { (*p).p_out }) };
    }
    if unsafe { (*p).i_protocol } as i32 <= 0 {
        unsafe { (*p).i_protocol = 2 as u8 };
    }
    while unsafe { (*p).n_err } <= unsafe { (*p).n_wr_err } &&
                { c = read_byte(unsafe { &mut *p }); c } != -1 && c != 66 {
        '__s14:
            {
            match c {
                70 => {
                    { read_and_display_message(p, c); break '__s14; }
                    {
                        let mut n_o_page: u32 = 0 as u32;
                        let mut n_r_page: u32 = 0 as u32;
                        let mut sz_r_page: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        let mut i_protocol: u8 = 0 as u8;
                        close_db(unsafe { &mut *p });
                        i_protocol = read_byte(unsafe { &mut *p }) as u8;
                        sz_o_page = read_pow2(p) as u32;
                        read_uint32(p, &mut n_o_page);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_BEGIN %d %d %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_protocol as i32, sz_o_page, n_o_page)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if i_protocol as i32 > unsafe { (*p).i_protocol } as i32 {

                            /// If the protocol version on the origin side is larger, send back
                            ///* a REPLICA_BEGIN message with the protocol version number of the
                            ///* replica side.  This gives the origin an opportunity to resend
                            ///* a new ORIGIN_BEGIN with a reduced protocol version.
                            write_byte(unsafe { &mut *p }, 97);
                            write_byte(unsafe { &mut *p },
                                unsafe { (*p).i_protocol } as i32);
                            unsafe { fflush(unsafe { (*p).p_out }) };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> REPLICA_BEGIN %u\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).i_protocol } as i32)
                                };
                            }
                            break '__s14;
                        }
                        unsafe { (*p).i_protocol = i_protocol };
                        unsafe { (*p).n_page = n_o_page };
                        unsafe { (*p).sz_page = sz_o_page };
                        rc =
                            unsafe {
                                sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                                    unsafe { &mut (*p).db })
                            };
                        if rc != 0 {
                            unsafe {
                                report_error(p,
                                    c"cannot open in-memory database: %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                            close_db(unsafe { &mut *p });
                            break '__s14;
                        }
                        unsafe {
                            sqlite3_db_config(unsafe { (*p).db }, 1011, 1, 0)
                        };
                        unsafe {
                            run_sql(p, c"ATTACH %Q AS \'replica\'".as_ptr() as *mut i8,
                                unsafe { (*p).z_replica })
                        };
                        if unsafe { (*p).wrong_encoding } != 0 {
                            unsafe { (*p).wrong_encoding = 0 as u8 };
                            unsafe {
                                run_sql(p, c"PRAGMA encoding=utf16le".as_ptr() as *mut i8)
                            };
                            unsafe {
                                run_sql(p, c"ATTACH %Q AS \'replica\'".as_ptr() as *mut i8,
                                    unsafe { (*p).z_replica })
                            };
                            if unsafe { (*p).wrong_encoding } != 0 {
                                unsafe { (*p).wrong_encoding = 0 as u8 };
                                unsafe {
                                    run_sql(p, c"PRAGMA encoding=utf16be".as_ptr() as *mut i8)
                                };
                                unsafe {
                                    run_sql(p, c"Attach %Q AS \'replica\'".as_ptr() as *mut i8,
                                        unsafe { (*p).z_replica })
                                };
                            }
                        }
                        if unsafe { (*p).n_err } != 0 {
                            close_db(unsafe { &mut *p });
                            break '__s14;
                        }
                        unsafe {
                            run_sql(p,
                                c"CREATE TABLE sendHash(  fpg INTEGER PRIMARY KEY,  npg INT)".as_ptr()
                                    as *mut i8)
                        };
                        hash_register(unsafe { (*p).db });
                        if unsafe {
                                    run_sql_return_u_int(p, &mut n_r_page,
                                        c"PRAGMA replica.page_count".as_ptr() as *mut i8)
                                } != 0 {
                            break '__s14;
                        }
                        if n_r_page == 0 as u32 {
                            unsafe {
                                run_sql(p,
                                    c"PRAGMA replica.page_size=%u".as_ptr() as *mut i8,
                                    sz_o_page)
                            };
                            unsafe {
                                run_sql(p,
                                    c"SELECT * FROM replica.sqlite_schema".as_ptr() as *mut i8)
                            };
                        }
                        unsafe {
                            run_sql(p, c"BEGIN IMMEDIATE".as_ptr() as *mut i8)
                        };
                        unsafe {
                            run_sql_return_text(p, &raw mut buf[0 as usize] as *mut i8,
                                c"PRAGMA replica.journal_mode".as_ptr() as *mut i8)
                        };
                        if unsafe {
                                    strcmp(&raw mut buf[0 as usize] as *mut i8 as *const i8,
                                        c"wal".as_ptr() as *mut i8 as *const i8)
                                } != 0 {
                            if unsafe { (*p).b_wal_only } != 0 && n_r_page > 0 as u32 {
                                unsafe {
                                    report_error(p,
                                        c"replica is not in WAL mode".as_ptr() as *mut i8 as
                                            *const i8)
                                };
                                break '__s14;
                            }
                            e_j_mode = 1 as i8;
                        } else { e_j_mode = 2 as i8; }
                        unsafe {
                            run_sql_return_u_int(p, &mut n_r_page,
                                c"PRAGMA replica.page_count".as_ptr() as *mut i8)
                        };
                        unsafe {
                            run_sql_return_u_int(p, &mut sz_r_page,
                                c"PRAGMA replica.page_size".as_ptr() as *mut i8)
                        };
                        if sz_r_page != sz_o_page {
                            unsafe {
                                report_error(p,
                                    c"page size mismatch; origin is %d bytes and replica is %d bytes".as_ptr()
                                            as *mut i8 as *const i8, sz_o_page, sz_r_page)
                            };
                            break '__s14;
                        }
                        if (unsafe { (*p).i_protocol } as i32) < 2 ||
                                n_r_page <= 100 as u32 {
                            unsafe {
                                run_sql(p,
                                    c"WITH RECURSIVE c(n) AS(VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<%d)INSERT INTO sendHash(fpg, npg) SELECT n, 1 FROM c".as_ptr()
                                        as *mut i8, n_r_page)
                            };
                        } else {
                            unsafe {
                                run_sql(p,
                                    c"INSERT INTO sendHash VALUES(1,1)".as_ptr() as *mut i8)
                            };
                            subdivide_hash_range(p, 2 as u32, n_r_page - 1 as u32);
                        }
                        send_hash_messages(p, 1 as u32, 1 as u32);
                        unsafe {
                            run_sql(p, c"PRAGMA writable_schema=ON".as_ptr() as *mut i8)
                        };
                        break '__s14;
                    }
                    {
                        let mut fpg: u32 = 0 as u32;
                        let mut npg: u32 = 0 as u32;
                        read_uint32(p, &mut fpg);
                        read_uint32(p, &mut npg);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, fpg, npg)
                            };
                        }
                        subdivide_hash_range(p, fpg, npg);
                        break '__s14;
                    }
                    {
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        send_hash_messages(p, 0 as u32, 0 as u32);
                        break '__s14;
                    }
                    {
                        let mut n_o_page_1: u32 = 0 as u32;
                        read_uint32(p, &mut n_o_page_1);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                    n_o_page_1)
                            };
                        }
                        if p_ins == core::ptr::null_mut() {

                            /// Nothing has changed
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        } else if unsafe { (*p).n_err } != 0 {
                            unsafe { run_sql(p, c"ROLLBACK".as_ptr() as *mut i8) };
                        } else {
                            if n_o_page_1 < 4294967295u32 {
                                let mut rc: i32 = 0;
                                unsafe {
                                    sqlite3_bind_int64(p_ins, 1,
                                        (n_o_page_1 + 1 as u32) as Sqlite3Int64)
                                };
                                unsafe { sqlite3_bind_null(p_ins, 2) };
                                rc = unsafe { sqlite3_step(p_ins) };
                                if rc != 101 {
                                    unsafe {
                                        report_error(p,
                                            c"SQL statement [%s] failed (pgno=%u, data=NULL): %s".as_ptr()
                                                    as *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) },
                                            n_o_page_1, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_ins) };
                            }
                            unsafe { (*p).n_page = n_o_page_1 };
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        }
                        break '__s14;
                    }
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                67 => {
                    { read_and_display_message(p, c); break '__s14; }
                    {
                        let mut n_o_page: u32 = 0 as u32;
                        let mut n_r_page: u32 = 0 as u32;
                        let mut sz_r_page: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        let mut i_protocol: u8 = 0 as u8;
                        close_db(unsafe { &mut *p });
                        i_protocol = read_byte(unsafe { &mut *p }) as u8;
                        sz_o_page = read_pow2(p) as u32;
                        read_uint32(p, &mut n_o_page);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_BEGIN %d %d %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_protocol as i32, sz_o_page, n_o_page)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if i_protocol as i32 > unsafe { (*p).i_protocol } as i32 {

                            /// If the protocol version on the origin side is larger, send back
                            ///* a REPLICA_BEGIN message with the protocol version number of the
                            ///* replica side.  This gives the origin an opportunity to resend
                            ///* a new ORIGIN_BEGIN with a reduced protocol version.
                            write_byte(unsafe { &mut *p }, 97);
                            write_byte(unsafe { &mut *p },
                                unsafe { (*p).i_protocol } as i32);
                            unsafe { fflush(unsafe { (*p).p_out }) };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> REPLICA_BEGIN %u\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).i_protocol } as i32)
                                };
                            }
                            break '__s14;
                        }
                        unsafe { (*p).i_protocol = i_protocol };
                        unsafe { (*p).n_page = n_o_page };
                        unsafe { (*p).sz_page = sz_o_page };
                        rc =
                            unsafe {
                                sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                                    unsafe { &mut (*p).db })
                            };
                        if rc != 0 {
                            unsafe {
                                report_error(p,
                                    c"cannot open in-memory database: %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                            close_db(unsafe { &mut *p });
                            break '__s14;
                        }
                        unsafe {
                            sqlite3_db_config(unsafe { (*p).db }, 1011, 1, 0)
                        };
                        unsafe {
                            run_sql(p, c"ATTACH %Q AS \'replica\'".as_ptr() as *mut i8,
                                unsafe { (*p).z_replica })
                        };
                        if unsafe { (*p).wrong_encoding } != 0 {
                            unsafe { (*p).wrong_encoding = 0 as u8 };
                            unsafe {
                                run_sql(p, c"PRAGMA encoding=utf16le".as_ptr() as *mut i8)
                            };
                            unsafe {
                                run_sql(p, c"ATTACH %Q AS \'replica\'".as_ptr() as *mut i8,
                                    unsafe { (*p).z_replica })
                            };
                            if unsafe { (*p).wrong_encoding } != 0 {
                                unsafe { (*p).wrong_encoding = 0 as u8 };
                                unsafe {
                                    run_sql(p, c"PRAGMA encoding=utf16be".as_ptr() as *mut i8)
                                };
                                unsafe {
                                    run_sql(p, c"Attach %Q AS \'replica\'".as_ptr() as *mut i8,
                                        unsafe { (*p).z_replica })
                                };
                            }
                        }
                        if unsafe { (*p).n_err } != 0 {
                            close_db(unsafe { &mut *p });
                            break '__s14;
                        }
                        unsafe {
                            run_sql(p,
                                c"CREATE TABLE sendHash(  fpg INTEGER PRIMARY KEY,  npg INT)".as_ptr()
                                    as *mut i8)
                        };
                        hash_register(unsafe { (*p).db });
                        if unsafe {
                                    run_sql_return_u_int(p, &mut n_r_page,
                                        c"PRAGMA replica.page_count".as_ptr() as *mut i8)
                                } != 0 {
                            break '__s14;
                        }
                        if n_r_page == 0 as u32 {
                            unsafe {
                                run_sql(p,
                                    c"PRAGMA replica.page_size=%u".as_ptr() as *mut i8,
                                    sz_o_page)
                            };
                            unsafe {
                                run_sql(p,
                                    c"SELECT * FROM replica.sqlite_schema".as_ptr() as *mut i8)
                            };
                        }
                        unsafe {
                            run_sql(p, c"BEGIN IMMEDIATE".as_ptr() as *mut i8)
                        };
                        unsafe {
                            run_sql_return_text(p, &raw mut buf[0 as usize] as *mut i8,
                                c"PRAGMA replica.journal_mode".as_ptr() as *mut i8)
                        };
                        if unsafe {
                                    strcmp(&raw mut buf[0 as usize] as *mut i8 as *const i8,
                                        c"wal".as_ptr() as *mut i8 as *const i8)
                                } != 0 {
                            if unsafe { (*p).b_wal_only } != 0 && n_r_page > 0 as u32 {
                                unsafe {
                                    report_error(p,
                                        c"replica is not in WAL mode".as_ptr() as *mut i8 as
                                            *const i8)
                                };
                                break '__s14;
                            }
                            e_j_mode = 1 as i8;
                        } else { e_j_mode = 2 as i8; }
                        unsafe {
                            run_sql_return_u_int(p, &mut n_r_page,
                                c"PRAGMA replica.page_count".as_ptr() as *mut i8)
                        };
                        unsafe {
                            run_sql_return_u_int(p, &mut sz_r_page,
                                c"PRAGMA replica.page_size".as_ptr() as *mut i8)
                        };
                        if sz_r_page != sz_o_page {
                            unsafe {
                                report_error(p,
                                    c"page size mismatch; origin is %d bytes and replica is %d bytes".as_ptr()
                                            as *mut i8 as *const i8, sz_o_page, sz_r_page)
                            };
                            break '__s14;
                        }
                        if (unsafe { (*p).i_protocol } as i32) < 2 ||
                                n_r_page <= 100 as u32 {
                            unsafe {
                                run_sql(p,
                                    c"WITH RECURSIVE c(n) AS(VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<%d)INSERT INTO sendHash(fpg, npg) SELECT n, 1 FROM c".as_ptr()
                                        as *mut i8, n_r_page)
                            };
                        } else {
                            unsafe {
                                run_sql(p,
                                    c"INSERT INTO sendHash VALUES(1,1)".as_ptr() as *mut i8)
                            };
                            subdivide_hash_range(p, 2 as u32, n_r_page - 1 as u32);
                        }
                        send_hash_messages(p, 1 as u32, 1 as u32);
                        unsafe {
                            run_sql(p, c"PRAGMA writable_schema=ON".as_ptr() as *mut i8)
                        };
                        break '__s14;
                    }
                    {
                        let mut fpg: u32 = 0 as u32;
                        let mut npg: u32 = 0 as u32;
                        read_uint32(p, &mut fpg);
                        read_uint32(p, &mut npg);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, fpg, npg)
                            };
                        }
                        subdivide_hash_range(p, fpg, npg);
                        break '__s14;
                    }
                    {
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        send_hash_messages(p, 0 as u32, 0 as u32);
                        break '__s14;
                    }
                    {
                        let mut n_o_page_1: u32 = 0 as u32;
                        read_uint32(p, &mut n_o_page_1);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                    n_o_page_1)
                            };
                        }
                        if p_ins == core::ptr::null_mut() {

                            /// Nothing has changed
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        } else if unsafe { (*p).n_err } != 0 {
                            unsafe { run_sql(p, c"ROLLBACK".as_ptr() as *mut i8) };
                        } else {
                            if n_o_page_1 < 4294967295u32 {
                                let mut rc: i32 = 0;
                                unsafe {
                                    sqlite3_bind_int64(p_ins, 1,
                                        (n_o_page_1 + 1 as u32) as Sqlite3Int64)
                                };
                                unsafe { sqlite3_bind_null(p_ins, 2) };
                                rc = unsafe { sqlite3_step(p_ins) };
                                if rc != 101 {
                                    unsafe {
                                        report_error(p,
                                            c"SQL statement [%s] failed (pgno=%u, data=NULL): %s".as_ptr()
                                                    as *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) },
                                            n_o_page_1, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_ins) };
                            }
                            unsafe { (*p).n_page = n_o_page_1 };
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        }
                        break '__s14;
                    }
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                65 => {
                    {
                        let mut n_o_page: u32 = 0 as u32;
                        let mut n_r_page: u32 = 0 as u32;
                        let mut sz_r_page: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        let mut i_protocol: u8 = 0 as u8;
                        close_db(unsafe { &mut *p });
                        i_protocol = read_byte(unsafe { &mut *p }) as u8;
                        sz_o_page = read_pow2(p) as u32;
                        read_uint32(p, &mut n_o_page);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_BEGIN %d %d %u\n".as_ptr() as *mut i8 as
                                        *const i8, i_protocol as i32, sz_o_page, n_o_page)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if i_protocol as i32 > unsafe { (*p).i_protocol } as i32 {

                            /// If the protocol version on the origin side is larger, send back
                            ///* a REPLICA_BEGIN message with the protocol version number of the
                            ///* replica side.  This gives the origin an opportunity to resend
                            ///* a new ORIGIN_BEGIN with a reduced protocol version.
                            write_byte(unsafe { &mut *p }, 97);
                            write_byte(unsafe { &mut *p },
                                unsafe { (*p).i_protocol } as i32);
                            unsafe { fflush(unsafe { (*p).p_out }) };
                            if !(unsafe { (*p).z_debug_file }).is_null() {
                                unsafe {
                                    debug_message(unsafe { &mut *p },
                                        c"-> REPLICA_BEGIN %u\n".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p).i_protocol } as i32)
                                };
                            }
                            break '__s14;
                        }
                        unsafe { (*p).i_protocol = i_protocol };
                        unsafe { (*p).n_page = n_o_page };
                        unsafe { (*p).sz_page = sz_o_page };
                        rc =
                            unsafe {
                                sqlite3_open(c":memory:".as_ptr() as *mut i8 as *const i8,
                                    unsafe { &mut (*p).db })
                            };
                        if rc != 0 {
                            unsafe {
                                report_error(p,
                                    c"cannot open in-memory database: %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                            close_db(unsafe { &mut *p });
                            break '__s14;
                        }
                        unsafe {
                            sqlite3_db_config(unsafe { (*p).db }, 1011, 1, 0)
                        };
                        unsafe {
                            run_sql(p, c"ATTACH %Q AS \'replica\'".as_ptr() as *mut i8,
                                unsafe { (*p).z_replica })
                        };
                        if unsafe { (*p).wrong_encoding } != 0 {
                            unsafe { (*p).wrong_encoding = 0 as u8 };
                            unsafe {
                                run_sql(p, c"PRAGMA encoding=utf16le".as_ptr() as *mut i8)
                            };
                            unsafe {
                                run_sql(p, c"ATTACH %Q AS \'replica\'".as_ptr() as *mut i8,
                                    unsafe { (*p).z_replica })
                            };
                            if unsafe { (*p).wrong_encoding } != 0 {
                                unsafe { (*p).wrong_encoding = 0 as u8 };
                                unsafe {
                                    run_sql(p, c"PRAGMA encoding=utf16be".as_ptr() as *mut i8)
                                };
                                unsafe {
                                    run_sql(p, c"Attach %Q AS \'replica\'".as_ptr() as *mut i8,
                                        unsafe { (*p).z_replica })
                                };
                            }
                        }
                        if unsafe { (*p).n_err } != 0 {
                            close_db(unsafe { &mut *p });
                            break '__s14;
                        }
                        unsafe {
                            run_sql(p,
                                c"CREATE TABLE sendHash(  fpg INTEGER PRIMARY KEY,  npg INT)".as_ptr()
                                    as *mut i8)
                        };
                        hash_register(unsafe { (*p).db });
                        if unsafe {
                                    run_sql_return_u_int(p, &mut n_r_page,
                                        c"PRAGMA replica.page_count".as_ptr() as *mut i8)
                                } != 0 {
                            break '__s14;
                        }
                        if n_r_page == 0 as u32 {
                            unsafe {
                                run_sql(p,
                                    c"PRAGMA replica.page_size=%u".as_ptr() as *mut i8,
                                    sz_o_page)
                            };
                            unsafe {
                                run_sql(p,
                                    c"SELECT * FROM replica.sqlite_schema".as_ptr() as *mut i8)
                            };
                        }
                        unsafe {
                            run_sql(p, c"BEGIN IMMEDIATE".as_ptr() as *mut i8)
                        };
                        unsafe {
                            run_sql_return_text(p, &raw mut buf[0 as usize] as *mut i8,
                                c"PRAGMA replica.journal_mode".as_ptr() as *mut i8)
                        };
                        if unsafe {
                                    strcmp(&raw mut buf[0 as usize] as *mut i8 as *const i8,
                                        c"wal".as_ptr() as *mut i8 as *const i8)
                                } != 0 {
                            if unsafe { (*p).b_wal_only } != 0 && n_r_page > 0 as u32 {
                                unsafe {
                                    report_error(p,
                                        c"replica is not in WAL mode".as_ptr() as *mut i8 as
                                            *const i8)
                                };
                                break '__s14;
                            }
                            e_j_mode = 1 as i8;
                        } else { e_j_mode = 2 as i8; }
                        unsafe {
                            run_sql_return_u_int(p, &mut n_r_page,
                                c"PRAGMA replica.page_count".as_ptr() as *mut i8)
                        };
                        unsafe {
                            run_sql_return_u_int(p, &mut sz_r_page,
                                c"PRAGMA replica.page_size".as_ptr() as *mut i8)
                        };
                        if sz_r_page != sz_o_page {
                            unsafe {
                                report_error(p,
                                    c"page size mismatch; origin is %d bytes and replica is %d bytes".as_ptr()
                                            as *mut i8 as *const i8, sz_o_page, sz_r_page)
                            };
                            break '__s14;
                        }
                        if (unsafe { (*p).i_protocol } as i32) < 2 ||
                                n_r_page <= 100 as u32 {
                            unsafe {
                                run_sql(p,
                                    c"WITH RECURSIVE c(n) AS(VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<%d)INSERT INTO sendHash(fpg, npg) SELECT n, 1 FROM c".as_ptr()
                                        as *mut i8, n_r_page)
                            };
                        } else {
                            unsafe {
                                run_sql(p,
                                    c"INSERT INTO sendHash VALUES(1,1)".as_ptr() as *mut i8)
                            };
                            subdivide_hash_range(p, 2 as u32, n_r_page - 1 as u32);
                        }
                        send_hash_messages(p, 1 as u32, 1 as u32);
                        unsafe {
                            run_sql(p, c"PRAGMA writable_schema=ON".as_ptr() as *mut i8)
                        };
                        break '__s14;
                    }
                    {
                        let mut fpg: u32 = 0 as u32;
                        let mut npg: u32 = 0 as u32;
                        read_uint32(p, &mut fpg);
                        read_uint32(p, &mut npg);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, fpg, npg)
                            };
                        }
                        subdivide_hash_range(p, fpg, npg);
                        break '__s14;
                    }
                    {
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        send_hash_messages(p, 0 as u32, 0 as u32);
                        break '__s14;
                    }
                    {
                        let mut n_o_page_1: u32 = 0 as u32;
                        read_uint32(p, &mut n_o_page_1);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                    n_o_page_1)
                            };
                        }
                        if p_ins == core::ptr::null_mut() {

                            /// Nothing has changed
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        } else if unsafe { (*p).n_err } != 0 {
                            unsafe { run_sql(p, c"ROLLBACK".as_ptr() as *mut i8) };
                        } else {
                            if n_o_page_1 < 4294967295u32 {
                                let mut rc: i32 = 0;
                                unsafe {
                                    sqlite3_bind_int64(p_ins, 1,
                                        (n_o_page_1 + 1 as u32) as Sqlite3Int64)
                                };
                                unsafe { sqlite3_bind_null(p_ins, 2) };
                                rc = unsafe { sqlite3_step(p_ins) };
                                if rc != 101 {
                                    unsafe {
                                        report_error(p,
                                            c"SQL statement [%s] failed (pgno=%u, data=NULL): %s".as_ptr()
                                                    as *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) },
                                            n_o_page_1, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_ins) };
                            }
                            unsafe { (*p).n_page = n_o_page_1 };
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        }
                        break '__s14;
                    }
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                71 => {
                    {
                        let mut fpg: u32 = 0 as u32;
                        let mut npg: u32 = 0 as u32;
                        read_uint32(p, &mut fpg);
                        read_uint32(p, &mut npg);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_DETAIL %u %u\n".as_ptr() as *mut i8 as
                                        *const i8, fpg, npg)
                            };
                        }
                        subdivide_hash_range(p, fpg, npg);
                        break '__s14;
                    }
                    {
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        send_hash_messages(p, 0 as u32, 0 as u32);
                        break '__s14;
                    }
                    {
                        let mut n_o_page_1: u32 = 0 as u32;
                        read_uint32(p, &mut n_o_page_1);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                    n_o_page_1)
                            };
                        }
                        if p_ins == core::ptr::null_mut() {

                            /// Nothing has changed
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        } else if unsafe { (*p).n_err } != 0 {
                            unsafe { run_sql(p, c"ROLLBACK".as_ptr() as *mut i8) };
                        } else {
                            if n_o_page_1 < 4294967295u32 {
                                let mut rc: i32 = 0;
                                unsafe {
                                    sqlite3_bind_int64(p_ins, 1,
                                        (n_o_page_1 + 1 as u32) as Sqlite3Int64)
                                };
                                unsafe { sqlite3_bind_null(p_ins, 2) };
                                rc = unsafe { sqlite3_step(p_ins) };
                                if rc != 101 {
                                    unsafe {
                                        report_error(p,
                                            c"SQL statement [%s] failed (pgno=%u, data=NULL): %s".as_ptr()
                                                    as *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) },
                                            n_o_page_1, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_ins) };
                            }
                            unsafe { (*p).n_page = n_o_page_1 };
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        }
                        break '__s14;
                    }
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                72 => {
                    {
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_READY\n".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        send_hash_messages(p, 0 as u32, 0 as u32);
                        break '__s14;
                    }
                    {
                        let mut n_o_page_1: u32 = 0 as u32;
                        read_uint32(p, &mut n_o_page_1);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                    n_o_page_1)
                            };
                        }
                        if p_ins == core::ptr::null_mut() {

                            /// Nothing has changed
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        } else if unsafe { (*p).n_err } != 0 {
                            unsafe { run_sql(p, c"ROLLBACK".as_ptr() as *mut i8) };
                        } else {
                            if n_o_page_1 < 4294967295u32 {
                                let mut rc: i32 = 0;
                                unsafe {
                                    sqlite3_bind_int64(p_ins, 1,
                                        (n_o_page_1 + 1 as u32) as Sqlite3Int64)
                                };
                                unsafe { sqlite3_bind_null(p_ins, 2) };
                                rc = unsafe { sqlite3_step(p_ins) };
                                if rc != 101 {
                                    unsafe {
                                        report_error(p,
                                            c"SQL statement [%s] failed (pgno=%u, data=NULL): %s".as_ptr()
                                                    as *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) },
                                            n_o_page_1, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_ins) };
                            }
                            unsafe { (*p).n_page = n_o_page_1 };
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        }
                        break '__s14;
                    }
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                69 => {
                    {
                        let mut n_o_page_1: u32 = 0 as u32;
                        read_uint32(p, &mut n_o_page_1);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_TXN %u\n".as_ptr() as *mut i8 as *const i8,
                                    n_o_page_1)
                            };
                        }
                        if p_ins == core::ptr::null_mut() {

                            /// Nothing has changed
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        } else if unsafe { (*p).n_err } != 0 {
                            unsafe { run_sql(p, c"ROLLBACK".as_ptr() as *mut i8) };
                        } else {
                            if n_o_page_1 < 4294967295u32 {
                                let mut rc: i32 = 0;
                                unsafe {
                                    sqlite3_bind_int64(p_ins, 1,
                                        (n_o_page_1 + 1 as u32) as Sqlite3Int64)
                                };
                                unsafe { sqlite3_bind_null(p_ins, 2) };
                                rc = unsafe { sqlite3_step(p_ins) };
                                if rc != 101 {
                                    unsafe {
                                        report_error(p,
                                            c"SQL statement [%s] failed (pgno=%u, data=NULL): %s".as_ptr()
                                                    as *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) },
                                            n_o_page_1, unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_ins) };
                            }
                            unsafe { (*p).n_page = n_o_page_1 };
                            unsafe { run_sql(p, c"COMMIT".as_ptr() as *mut i8) };
                        }
                        break '__s14;
                    }
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                68 => {
                    {
                        let mut pgno: u32 = 0 as u32;
                        let mut rc: i32 = 0;
                        read_uint32(p, &mut pgno);
                        if !(unsafe { (*p).z_debug_file }).is_null() {
                            unsafe {
                                debug_message(unsafe { &mut *p },
                                    c"<- ORIGIN_PAGE %u\n".as_ptr() as *mut i8 as *const i8,
                                    pgno)
                            };
                        }
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if p_ins == core::ptr::null_mut() {
                            p_ins =
                                unsafe {
                                    prepare_stmt(p,
                                        c"INSERT INTO sqlite_dbpage(pgno,data,schema)VALUES(?1,?2,\'replica\')".as_ptr()
                                            as *mut i8)
                                };
                            if p_ins == core::ptr::null_mut() { break '__s14; }
                        }
                        read_bytes(p,
                            unsafe {
                                let __p = &raw mut buf[0 as usize] as *mut u8 as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p, sz_o_page as usize)
                                }
                            });
                        if unsafe { (*p).n_err } != 0 { break '__s14; }
                        if pgno == 1 as u32 && e_j_mode as i32 == 2 &&
                                buf[18 as usize] as i32 == 1 {

                            /// Do not switch the replica out of WAL mode if it started in 
                            ///* WAL mode
                            (buf[18 as usize] =
                                { buf[19 as usize] = 2 as i8; buf[19 as usize] });
                        }
                        {
                            let __p = unsafe { &mut (*p).n_page_sent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        unsafe {
                            sqlite3_bind_int64(p_ins, 1, pgno as Sqlite3Int64)
                        };
                        unsafe {
                            sqlite3_bind_blob(p_ins, 2,
                                &raw mut buf[0 as usize] as *mut i8 as *const (),
                                sz_o_page as i32, None)
                        };
                        rc = unsafe { sqlite3_step(p_ins) };
                        if rc != 101 {
                            unsafe {
                                report_error(p,
                                    c"SQL statement [%s] failed (pgno=%u): %s".as_ptr() as
                                            *mut i8 as *const i8, unsafe { sqlite3_sql(p_ins) }, pgno,
                                    unsafe { sqlite3_errmsg(unsafe { (*p).db }) })
                            };
                        }
                        unsafe { sqlite3_reset(p_ins) };
                        break '__s14;
                    }
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
                _ => {
                    {
                        unsafe {
                            report_error(p,
                                c"Unknown message 0x%02x %lld bytes into conversation".as_ptr()
                                        as *mut i8 as *const i8, c, unsafe { (*p).n_in })
                        };
                        break '__s14;
                    }
                }
            }
        }
    }
    if !(p_ins).is_null() { unsafe { sqlite3_finalize(p_ins) }; }
    close_db(unsafe { &mut *p });
}

///* The argument might be -vvv...vv with any number of "v"s.  Return
///* the number of "v"s.  Return 0 if the argument is not a -vvv...v.
extern "C" fn num_vs(mut z: *const i8) -> i32 {
    let mut n: i32 = 0;
    if unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 { return 0; }
    {
        let __p = &mut z;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    while unsafe { *z.offset(0 as isize) } as i32 == 'v' as i32 {
        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if unsafe { *z.offset(0 as isize) } as i32 == 0 { return n; }
    return 0;
}

///* Get the argument to an --option.  Throw an error and die if no argument
///* is available.
extern "C" fn cmdline_option_value(argc: i32, argv: *const *const i8, i: i32)
    -> *const i8 {
    unsafe {
        if i == argc {
            unsafe {
                fprintf(__stderrp,
                    c"%s: Error: missing argument to %s\n".as_ptr() as *mut i8
                        as *const i8, unsafe { *argv.offset(0 as isize) },
                    unsafe { *argv.offset((argc - 1) as isize) })
            };
            unsafe { exit(1) };
        }
        return unsafe { *argv.offset(i as isize) };
    }
}

///* Return the current time in milliseconds since the Julian epoch.
#[unsafe(no_mangle)]
pub extern "C" fn current_time() -> Sqlite3Int64 {
    let mut now: Sqlite3Int64 = 0 as Sqlite3Int64;
    let p_vfs: *mut Sqlite3Vfs =
        unsafe { sqlite3_vfs_find(core::ptr::null()) };
    if !(p_vfs).is_null() && unsafe { (*p_vfs).i_version } >= 2 &&
            unsafe { (*p_vfs).x_current_time_int64.is_some() } {
        unsafe {
            (unsafe {
                    (*p_vfs).x_current_time_int64.unwrap()
                })(p_vfs, &mut now)
        };
    }
    return now;
}

///* Input string zIn might be in any of these formats:
///*
///*    (1) PATH
///*    (2) HOST:PATH
///*    (3) USER@HOST:PATH
///*
///* For format 1, return NULL.  For formats 2 and 3, return
///* a pointer to the ':' character that separates the hostname
///* from the path.
extern "C" fn host_separator(mut z_in_1: *const i8) -> *mut i8 {
    let z_path: *mut i8 = unsafe { strchr(z_in_1, ':' as i32) };
    if z_path == core::ptr::null_mut() { return core::ptr::null_mut(); }
    while z_in_1 < z_path as *const i8 {
        if unsafe { *z_in_1.offset(0 as isize) } as i32 == '/' as i32 {
            return core::ptr::null_mut();
        }
        if unsafe { *z_in_1.offset(0 as isize) } as i32 == '\\' as i32 {
            return core::ptr::null_mut();
        }
        {
            let __p = &mut z_in_1;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    return z_path;
}

///* Parse command-line arguments.  Dispatch subroutines to do the
///* requested work.
///*
///* Input formats:
///*
///*  (1)    sqlite3_rsync  FILENAME1  USER@HOST:FILENAME2
///*
///*  (2)    sqlite3_rsync  USER@HOST:FILENAME1  FILENAME2
///*
///*  (3)    sqlite3_rsync --origin FILENAME1
///*
///*  (4)    sqlite3_rsync --replica FILENAME2
///*
///* The user types (1) or (2).  SSH launches (3) or (4).
///*
///* If (1) is seen then popen2 is used launch (4) on the remote and
///* originSide() is called locally.
///*
///* If (2) is seen, then popen2() is used to launch (3) on the remote
///* and replicaSide() is run locally.
///*
///* If (3) is seen, call originSide() on stdin and stdout.
///*
///q** If (4) is seen, call replicaSide() on stdin and stdout.
#[allow(unused_doc_comments)]
extern "C" fn __main_inner(argc: i32, argv: *const *const i8)
    -> Result<(), i32> {
    unsafe {
        let mut is_origin: i32 = 0;
        let mut is_replica: i32 = 0;
        let mut i: i32 = 0;
        let mut ctx: SQLiteRsync = unsafe { core::mem::zeroed() };
        let mut z_div: *mut i8 = core::ptr::null_mut();
        let p_in: *mut FILE = core::ptr::null_mut();
        let p_out: *mut FILE = core::ptr::null_mut();
        let mut child_pid: i32 = 0;
        let mut z_ssh: *const i8 = c"ssh".as_ptr() as *mut i8 as *const i8;
        let mut i_port: i32 = 0;
        let mut z_exe: *const i8 =
            c"sqlite3_rsync".as_ptr() as *mut i8 as *const i8;
        let mut z_cmd: *mut i8 = core::ptr::null_mut();
        let mut tm_start: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut tm_end: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut tm_elapse: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut z_remote_err_file: *const i8 = core::ptr::null();
        let mut z_remote_debug_file: *const i8 = core::ptr::null();
        unsafe {
            memset(&raw mut ctx as *mut (), 0,
                core::mem::size_of::<SQLiteRsync>() as u64)
        };
        ctx.i_protocol = 2 as u8;
        unsafe { sqlite3_initialize() };
        {
            i = 1;
            '__b17: loop {
                if !(i < argc) { break '__b17; }
                '__c17: loop {
                    let mut z: *const i8 = unsafe { *argv.offset(i as isize) };
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 &&
                                unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 &&
                            unsafe { *z.offset(2 as isize) } as i32 != 0 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe {
                                strcmp(z, c"-origin".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        is_origin = 1;
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-replica".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        is_replica = 1;
                        break '__c17;
                    }
                    if num_vs(z) != 0 {
                        ctx.e_verbose += num_vs(z) as u8;
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-protocol".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        ctx.i_protocol =
                            unsafe {
                                    atoi(cmdline_option_value(argc, argv,
                                            { let __p = &mut i; *__p += 1; *__p }))
                                } as u8;
                        if (ctx.i_protocol as i32) < 1 {
                            ctx.i_protocol = 1 as u8;
                        } else if ctx.i_protocol as i32 > 2 {
                            ctx.i_protocol = 2 as u8;
                        }
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-ssh".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        z_ssh =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p });
                        break '__c17;
                    }
                    if unsafe {
                                    strcmp(z, c"-port".as_ptr() as *mut i8 as *const i8)
                                } == 0 ||
                            unsafe { strcmp(z, c"-p".as_ptr() as *mut i8 as *const i8) }
                                == 0 {
                        let z_port: *const i8 =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p });
                        i_port = unsafe { atoi(z_port) };
                        if i_port < 1 || i_port > 65535 {
                            unsafe {
                                fprintf(__stderrp,
                                    c"invalid TCP port number: \"%s\"\n".as_ptr() as *mut i8 as
                                        *const i8, z_port)
                            };
                            return Err(1);
                        }
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-exe".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        z_exe =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p });
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-wal-only".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        ctx.b_wal_only = 1 as u8;
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-version".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        unsafe {
                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_sourceid() })
                        };
                        return Ok(());
                    }
                    if unsafe {
                                        strcmp(z, c"-help".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe {
                                        strcmp(z, c"--help".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                            unsafe { strcmp(z, c"-?".as_ptr() as *mut i8 as *const i8) }
                                == 0 {
                        unsafe {
                            printf(c"%s".as_ptr() as *mut i8 as *const i8,
                                &raw const z_usage[0 as usize] as *const i8)
                        };
                        return Ok(());
                    }
                    if unsafe {
                                strcmp(z, c"-logfile".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        /// DEBUG OPTION:  --logfile FILENAME
                        ///* Cause all local output traffic to be duplicated in FILENAME
                        let z_log: *const i8 =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p });
                        if !(ctx.p_log).is_null() { unsafe { fclose(ctx.p_log) }; }
                        ctx.p_log =
                            unsafe {
                                fopen(z_log, c"wb".as_ptr() as *mut i8 as *const i8)
                            };
                        if ctx.p_log == core::ptr::null_mut() {
                            unsafe {
                                fprintf(__stderrp,
                                    c"cannot open \"%s\" for writing\n".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(i as isize) })
                            };
                            return Err(1);
                        }
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-errorfile".as_ptr() as *mut i8 as *const i8)
                            } == 0 {

                        /// DEBUG OPTION:  --errorfile FILENAME
                        ///* Error messages on the local side are written into FILENAME
                        (ctx.z_err_file =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p }));
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z,
                                    c"-remote-errorfile".as_ptr() as *mut i8 as *const i8)
                            } == 0 {

                        /// DEBUG OPTION:  --remote-errorfile FILENAME
                        ///* Error messages on the remote side are written into FILENAME on
                        ///* the remote side.
                        (z_remote_err_file =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p }));
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-debugfile".as_ptr() as *mut i8 as *const i8)
                            } == 0 {

                        /// DEBUG OPTION:  --debugfile FILENAME
                        ///* Debugging messages on the local side are written into FILENAME
                        (ctx.z_debug_file =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p }));
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z,
                                    c"-remote-debugfile".as_ptr() as *mut i8 as *const i8)
                            } == 0 {

                        /// DEBUG OPTION:  --remote-debugfile FILENAME
                        ///* Error messages on the remote side are written into FILENAME on
                        ///* the remote side.
                        (z_remote_debug_file =
                            cmdline_option_value(argc, argv,
                                { let __p = &mut i; *__p += 1; *__p }));
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z, c"-commcheck".as_ptr() as *mut i8 as *const i8)
                            } == 0 {

                        /// DEBUG ONLY */
                        ///      /* Run a communication check with the remote side.  Do not attempt
                        ///* to exchange any database connection
                        (ctx.b_comm_check = 1 as u8);
                        break '__c17;
                    }
                    if unsafe {
                                strcmp(z,
                                    c"-arg-escape-check".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        /// DEBUG ONLY */
                        ///      /* Test the append_escaped_arg() routine by using it to render a
                        ///* copy of the input command-line, assuming all arguments except
                        ///* this one are filenames.
                        let p_str: *mut Sqlite3Str =
                            unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        let mut k: i32 = 0;
                        {
                            k = 0;
                            '__b18: loop {
                                if !(k < argc) { break '__b18; }
                                '__c18: loop {
                                    append_escaped_arg(p_str,
                                        unsafe { *argv.offset(k as isize) }, (i != k) as i32);
                                    break '__c18;
                                }
                                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_str_value(p_str) })
                        };
                        return Ok(());
                    }
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        unsafe {
                            fprintf(__stderrp,
                                c"unknown option: \"%s\". Use --help for more detail.\n".as_ptr()
                                        as *mut i8 as *const i8, z)
                        };
                        return Err(1);
                    }
                    if ctx.z_origin == core::ptr::null() {
                        ctx.z_origin = z;
                    } else if ctx.z_replica == core::ptr::null() {
                        ctx.z_replica = z;
                    } else {
                        unsafe {
                            fprintf(__stderrp,
                                c"Unknown argument: \"%s\"\n".as_ptr() as *mut i8 as
                                    *const i8, z)
                        };
                        return Err(1);
                    }
                    break '__c17;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if ctx.z_origin == core::ptr::null() {
            eprintln!("missing ORIGIN database filename");
            return Err(1);
        }
        if ctx.z_replica == core::ptr::null() {
            eprintln!("missing REPLICA database filename");
            return Err(1);
        }
        if is_origin != 0 && is_replica != 0 {
            eprintln!("bad option combination");
            return Err(1);
        }
        if is_origin != 0 {
            ctx.p_in = __stdinp;
            ctx.p_out = __stdoutp;
            ctx.is_remote = 1 as u8;
            origin_side(&mut ctx);
            return Ok(());
        }
        if is_replica != 0 {
            ctx.p_in = __stdinp;
            ctx.p_out = __stdoutp;
            ctx.is_remote = 1 as u8;
            replica_side(&mut ctx);
            return Ok(());
        }
        if ctx.z_replica == core::ptr::null() {
            eprintln!("missing REPLICA database filename");
            return Err(1);
        }
        tm_start = current_time();
        z_div = host_separator(ctx.z_origin);
        if !(z_div).is_null() {
            let mut i_retry: i32 = 0;
            if host_separator(ctx.z_replica) != core::ptr::null_mut() {
                eprintln!("At least one of ORIGIN and REPLICA must be a local database\nYou provided two remote databases.");
                return Err(1);
            }
            unsafe {
                *{
                            let __p = &mut z_div;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        } = 0 as i8
            };
            {
                i_retry = 0;
                '__b19: loop {
                    if !(1 != 0) { break '__b19; }
                    '__c19: loop {
                        let p_str_1: *mut Sqlite3Str =
                            unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        append_escaped_arg(p_str_1, z_ssh, 1);
                        if i_port > 0 {
                            unsafe {
                                sqlite3_str_appendf(p_str_1,
                                    c" -p %d".as_ptr() as *mut i8 as *const i8, i_port)
                            };
                        }
                        unsafe {
                            sqlite3_str_appendf(p_str_1,
                                c" -e none".as_ptr() as *mut i8 as *const i8)
                        };
                        append_escaped_arg(p_str_1, ctx.z_origin, 1);
                        if i_retry != 0 { add_path_argument(p_str_1); }
                        append_escaped_arg(p_str_1, z_exe, 1);
                        append_escaped_arg(p_str_1,
                            c"--origin".as_ptr() as *mut i8 as *const i8, 0);
                        if ctx.b_comm_check != 0 {
                            append_escaped_arg(p_str_1,
                                c"--commcheck".as_ptr() as *mut i8 as *const i8, 0);
                            if ctx.e_verbose as i32 == 0 { ctx.e_verbose = 1 as u8; }
                        }
                        if !(z_remote_err_file).is_null() {
                            append_escaped_arg(p_str_1,
                                c"--errorfile".as_ptr() as *mut i8 as *const i8, 0);
                            append_escaped_arg(p_str_1, z_remote_err_file, 1);
                        }
                        if !(z_remote_debug_file).is_null() {
                            append_escaped_arg(p_str_1,
                                c"--debugfile".as_ptr() as *mut i8 as *const i8, 0);
                            append_escaped_arg(p_str_1, z_remote_debug_file, 1);
                        }
                        if ctx.b_wal_only != 0 {
                            append_escaped_arg(p_str_1,
                                c"--wal-only".as_ptr() as *mut i8 as *const i8, 0);
                        }
                        append_escaped_arg(p_str_1, z_div as *const i8, 1);
                        append_escaped_arg(p_str_1, file_tail(ctx.z_replica), 1);
                        if (ctx.e_verbose as i32) < 2 && i_retry == 0 {
                            append_escaped_arg(p_str_1,
                                c"2>/dev/null".as_ptr() as *mut i8 as *const i8, 0);
                        }
                        z_cmd = unsafe { sqlite3_str_finish(p_str_1) };
                        if ctx.e_verbose as i32 >= 2 {
                            unsafe {
                                printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_cmd)
                            };
                        }
                        if popen2(z_cmd as *const i8, &mut ctx.p_in, &mut ctx.p_out,
                                    &mut child_pid, 0) != 0 {
                            if i_retry >= 1 {
                                unsafe {
                                    fprintf(__stderrp,
                                        c"Could not start auxiliary process: %s\n".as_ptr() as
                                                *mut i8 as *const i8, z_cmd)
                                };
                                return Err(1);
                            }
                            if ctx.e_verbose as i32 >= 2 {
                                unsafe {
                                    printf(c"ssh FAILED.  Retry with a PATH= argument...\n".as_ptr()
                                                as *mut i8 as *const i8)
                                };
                            }
                            break '__c19;
                        }
                        replica_side(&mut ctx);
                        if ctx.n_hash_sent == 0 as u64 && i_retry == 0 {
                            break '__c19;
                        }
                        break '__b19;
                        break '__c19;
                    }
                    { let __p = &mut i_retry; let __t = *__p; *__p += 1; __t };
                }
            }
        } else if { z_div = host_separator(ctx.z_replica); z_div } !=
                core::ptr::null_mut() {
            /// Local ORIGIN and remote REPLICA
            let mut i_retry_1: i32 = 0;
            unsafe {
                *{
                            let __p = &mut z_div;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        } = 0 as i8
            };
            {
                i_retry_1 = 0;
                '__b20: loop {
                    if !(1 != 0) { break '__b20; }
                    '__c20: loop {
                        let p_str_2: *mut Sqlite3Str =
                            unsafe { sqlite3_str_new(core::ptr::null_mut()) };
                        append_escaped_arg(p_str_2, z_ssh, 1);
                        if i_port > 0 {
                            unsafe {
                                sqlite3_str_appendf(p_str_2,
                                    c" -p %d".as_ptr() as *mut i8 as *const i8, i_port)
                            };
                        }
                        unsafe {
                            sqlite3_str_appendf(p_str_2,
                                c" -e none".as_ptr() as *mut i8 as *const i8)
                        };
                        append_escaped_arg(p_str_2, ctx.z_replica, 1);
                        if i_retry_1 == 1 { add_path_argument(p_str_2); }
                        append_escaped_arg(p_str_2, z_exe, 1);
                        append_escaped_arg(p_str_2,
                            c"--replica".as_ptr() as *mut i8 as *const i8, 0);
                        if ctx.b_comm_check != 0 {
                            append_escaped_arg(p_str_2,
                                c"--commcheck".as_ptr() as *mut i8 as *const i8, 0);
                            if ctx.e_verbose as i32 == 0 { ctx.e_verbose = 1 as u8; }
                        }
                        if !(z_remote_err_file).is_null() {
                            append_escaped_arg(p_str_2,
                                c"--errorfile".as_ptr() as *mut i8 as *const i8, 0);
                            append_escaped_arg(p_str_2, z_remote_err_file, 1);
                        }
                        if !(z_remote_debug_file).is_null() {
                            append_escaped_arg(p_str_2,
                                c"--debugfile".as_ptr() as *mut i8 as *const i8, 0);
                            append_escaped_arg(p_str_2, z_remote_debug_file, 1);
                        }
                        if ctx.b_wal_only != 0 {
                            append_escaped_arg(p_str_2,
                                c"--wal-only".as_ptr() as *mut i8 as *const i8, 0);
                        }
                        append_escaped_arg(p_str_2, file_tail(ctx.z_origin), 1);
                        append_escaped_arg(p_str_2, z_div as *const i8, 1);
                        if (ctx.e_verbose as i32) < 2 && i_retry_1 == 0 {
                            append_escaped_arg(p_str_2,
                                c"2>/dev/null".as_ptr() as *mut i8 as *const i8, 0);
                        }
                        z_cmd = unsafe { sqlite3_str_finish(p_str_2) };
                        if ctx.e_verbose as i32 >= 2 {
                            unsafe {
                                printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_cmd)
                            };
                        }
                        if popen2(z_cmd as *const i8, &mut ctx.p_in, &mut ctx.p_out,
                                    &mut child_pid, 0) != 0 {
                            if i_retry_1 >= 1 {
                                unsafe {
                                    fprintf(__stderrp,
                                        c"Could not start auxiliary process: %s\n".as_ptr() as
                                                *mut i8 as *const i8, z_cmd)
                                };
                                return Err(1);
                            } else if ctx.e_verbose as i32 >= 2 {
                                unsafe {
                                    printf(c"ssh FAILED.  Retry with a PATH= argument...\n".as_ptr()
                                                as *mut i8 as *const i8)
                                };
                            }
                            break '__c20;
                        }
                        origin_side(&mut ctx);
                        if ctx.n_hash_sent == 0 as u64 && i_retry_1 == 0 {
                            break '__c20;
                        }
                        break '__b20;
                        break '__c20;
                    }
                    {
                        let __p = &mut i_retry_1;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
            }
        } else {
            /// Local ORIGIN and REPLICA
            let p_str_3: *mut Sqlite3Str =
                unsafe { sqlite3_str_new(core::ptr::null_mut()) };
            append_escaped_arg(p_str_3, unsafe { *argv.offset(0 as isize) },
                1);
            append_escaped_arg(p_str_3,
                c"--replica".as_ptr() as *mut i8 as *const i8, 0);
            if ctx.b_comm_check != 0 {
                append_escaped_arg(p_str_3,
                    c"--commcheck".as_ptr() as *mut i8 as *const i8, 0);
            }
            if !(z_remote_err_file).is_null() {
                append_escaped_arg(p_str_3,
                    c"--errorfile".as_ptr() as *mut i8 as *const i8, 0);
                append_escaped_arg(p_str_3, z_remote_err_file, 1);
            }
            if !(z_remote_debug_file).is_null() {
                append_escaped_arg(p_str_3,
                    c"--debugfile".as_ptr() as *mut i8 as *const i8, 0);
                append_escaped_arg(p_str_3, z_remote_debug_file, 1);
            }
            append_escaped_arg(p_str_3, ctx.z_origin, 1);
            append_escaped_arg(p_str_3, ctx.z_replica, 1);
            z_cmd = unsafe { sqlite3_str_finish(p_str_3) };
            if ctx.e_verbose as i32 >= 2 {
                unsafe {
                    printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_cmd)
                };
            }
            if popen2(z_cmd as *const i8, &mut ctx.p_in, &mut ctx.p_out,
                        &mut child_pid, 0) != 0 {
                unsafe {
                    fprintf(__stderrp,
                        c"Could not start auxiliary process: %s\n".as_ptr() as
                                *mut i8 as *const i8, z_cmd)
                };
                return Err(1);
            }
            origin_side(&mut ctx);
        }
        ctx.n_err +=
            ((pclose2(ctx.p_in, ctx.p_out, child_pid) == 0) as i32 == 0) as
                    i32 as i32;
        if !(ctx.p_log).is_null() { unsafe { fclose(ctx.p_log) }; }
        tm_end = current_time();
        tm_elapse = tm_end - tm_start;
        if ctx.n_err != 0 {
            unsafe {
                printf(c"Databases were not synced due to errors\n".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        if ctx.e_verbose as i32 >= 1 {
            let mut z_msg: *mut i8 = core::ptr::null_mut();
            let sz_total: Sqlite3Int64 =
                ctx.n_page as Sqlite3Int64 * ctx.sz_page as Sqlite3Int64;
            let n_io: Sqlite3Int64 = (ctx.n_out + ctx.n_in) as Sqlite3Int64;
            z_msg =
                unsafe {
                    sqlite3_mprintf(c"sent %,lld bytes, received %,lld bytes".as_ptr()
                                as *mut i8 as *const i8, ctx.n_out, ctx.n_in)
                };
            unsafe { printf(c"%s".as_ptr() as *mut i8 as *const i8, z_msg) };
            unsafe { sqlite3_free(z_msg as *mut ()) };
            if tm_elapse > 0 as i64 {
                z_msg =
                    unsafe {
                        sqlite3_mprintf(c", %,.2f bytes/sec".as_ptr() as *mut i8 as
                                *const i8, 1000.0 * n_io as f64 / tm_elapse as f64)
                    };
                unsafe {
                    printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_msg)
                };
                unsafe { sqlite3_free(z_msg as *mut ()) };
            } else {
                unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
            }
            if ctx.n_err == 0 {
                if n_io <= sz_total && n_io > 0 as i64 {
                    z_msg =
                        unsafe {
                            sqlite3_mprintf(c"total size %,lld  speedup is %.2f".as_ptr()
                                        as *mut i8 as *const i8, sz_total,
                                sz_total as f64 / n_io as f64)
                        };
                } else {
                    z_msg =
                        unsafe {
                            sqlite3_mprintf(c"total size %,lld".as_ptr() as *mut i8 as
                                    *const i8, sz_total)
                        };
                }
                unsafe {
                    printf(c"%s\n".as_ptr() as *mut i8 as *const i8, z_msg)
                };
                unsafe { sqlite3_free(z_msg as *mut ()) };
            }
            if ctx.e_verbose as i32 >= 3 {
                unsafe {
                    printf(c"hashes: %lld  hash-rounds: %d  page updates: %d  protocol: %d\n".as_ptr()
                                as *mut i8 as *const i8, ctx.n_hash_sent, ctx.n_round,
                        ctx.n_page_sent, ctx.i_protocol as i32)
                };
            }
        }
        unsafe { sqlite3_free(z_cmd as *mut ()) };
        if p_in != core::ptr::null_mut() && p_out != core::ptr::null_mut() {
            pclose2(p_in, p_out, child_pid);
        }
        unsafe { sqlite3_shutdown() };
        return Err(ctx.n_err);
    }
}

static rc_1: [u64; 24] =
    [1, 32898, 9223372036854808714u64, 9223372039002292224u64, 32907,
            2147483649u64, 9223372039002292353u64, 9223372036854808585u64,
            138, 136, 2147516425u64, 2147483658u64, 2147516555u64,
            9223372036854775947u64, 9223372036854808713u64,
            9223372036854808579u64, 9223372036854808578u64,
            9223372036854775936u64, 32778, 9223372039002259466u64,
            9223372039002292353u64, 9223372036854808704u64, 2147483649u64,
            9223372039002292232u64];

static mut one: u32 = 1 as u32;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
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
    fn sqlite3_sha_init(db: *mut Sqlite3, pz_err_msg_1: *mut *mut i8,
    p_api_1: *const Sqlite3ApiRoutines)
    -> i32;
    fn pipe(_: *mut i32)
    -> i32;
    fn close(_: i32)
    -> i32;
    fn fork()
    -> PidT;
    fn signal(_: i32, _: unsafe extern "C" fn(i32) -> ())
    -> unsafe extern "C" fn(i32) -> ();
    fn dup(_: i32)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn execl(__path: *const i8, __arg0: *const i8, ...)
    -> i32;
    fn fdopen(_: i32, _: *const i8)
    -> *mut FILE;
    fn fclose(_: *mut FILE)
    -> i32;
    fn waitpid(_: PidT, _: *mut i32, _: i32)
    -> PidT;
    fn isspace(_c: i32)
    -> i32;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn fgetc(_: *mut FILE)
    -> i32;
    fn fputc(_: i32, _: *mut FILE)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn putc(_: i32, _: *mut FILE)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn atoi(_: *const i8)
    -> i32;
    static mut __stderrp: *mut FILE;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
