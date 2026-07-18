#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
static z_help: [i8; 1078] =
    [85 as i8, 115 as i8, 97 as i8, 103 as i8, 101 as i8, 58 as i8, 32 as i8,
            119 as i8, 111 as i8, 114 as i8, 100 as i8, 99 as i8, 111 as i8,
            117 as i8, 110 as i8, 116 as i8, 32 as i8, 91 as i8, 79 as i8,
            80 as i8, 84 as i8, 73 as i8, 79 as i8, 78 as i8, 83 as i8,
            93 as i8, 32 as i8, 68 as i8, 65 as i8, 84 as i8, 65 as i8,
            66 as i8, 65 as i8, 83 as i8, 69 as i8, 32 as i8, 91 as i8,
            73 as i8, 78 as i8, 80 as i8, 85 as i8, 84 as i8, 93 as i8,
            10 as i8, 32 as i8, 45 as i8, 45 as i8, 97 as i8, 108 as i8,
            108 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 82 as i8,
            101 as i8, 112 as i8, 101 as i8, 97 as i8, 116 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 116 as i8, 101 as i8,
            115 as i8, 116 as i8, 32 as i8, 102 as i8, 111 as i8, 114 as i8,
            32 as i8, 97 as i8, 108 as i8, 108 as i8, 32 as i8, 116 as i8,
            101 as i8, 115 as i8, 116 as i8, 32 as i8, 109 as i8, 111 as i8,
            100 as i8, 101 as i8, 115 as i8, 10 as i8, 32 as i8, 45 as i8,
            45 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8, 101 as i8,
            115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8, 78 as i8,
            78 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8,
            97 as i8, 32 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8,
            101 as i8, 32 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8,
            32 as i8, 111 as i8, 102 as i8, 32 as i8, 78 as i8, 78 as i8,
            78 as i8, 10 as i8, 32 as i8, 45 as i8, 45 as i8, 99 as i8,
            111 as i8, 109 as i8, 109 as i8, 105 as i8, 116 as i8, 32 as i8,
            78 as i8, 78 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            67 as i8, 111 as i8, 109 as i8, 109 as i8, 105 as i8, 116 as i8,
            32 as i8, 97 as i8, 102 as i8, 116 as i8, 101 as i8, 114 as i8,
            32 as i8, 101 as i8, 118 as i8, 101 as i8, 114 as i8, 121 as i8,
            32 as i8, 78 as i8, 78 as i8, 78 as i8, 32 as i8, 111 as i8,
            112 as i8, 101 as i8, 114 as i8, 97 as i8, 116 as i8, 105 as i8,
            111 as i8, 110 as i8, 115 as i8, 10 as i8, 32 as i8, 45 as i8,
            45 as i8, 100 as i8, 101 as i8, 108 as i8, 101 as i8, 116 as i8,
            101 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8,
            68 as i8, 69 as i8, 76 as i8, 69 as i8, 84 as i8, 69 as i8,
            32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8, 10 as i8,
            32 as i8, 45 as i8, 45 as i8, 105 as i8, 110 as i8, 115 as i8,
            101 as i8, 114 as i8, 116 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8,
            101 as i8, 32 as i8, 73 as i8, 78 as i8, 83 as i8, 69 as i8,
            82 as i8, 84 as i8, 32 as i8, 109 as i8, 111 as i8, 100 as i8,
            101 as i8, 32 as i8, 40 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 100 as i8, 101 as i8, 102 as i8, 97 as i8, 117 as i8,
            108 as i8, 116 as i8, 41 as i8, 10 as i8, 32 as i8, 45 as i8,
            45 as i8, 106 as i8, 111 as i8, 117 as i8, 114 as i8, 110 as i8,
            97 as i8, 108 as i8, 32 as i8, 77 as i8, 77 as i8, 77 as i8,
            77 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8,
            80 as i8, 82 as i8, 65 as i8, 71 as i8, 77 as i8, 65 as i8,
            32 as i8, 106 as i8, 111 as i8, 117 as i8, 114 as i8, 110 as i8,
            97 as i8, 108 as i8, 95 as i8, 109 as i8, 111 as i8, 100 as i8,
            101 as i8, 61 as i8, 77 as i8, 77 as i8, 77 as i8, 77 as i8,
            10 as i8, 32 as i8, 45 as i8, 45 as i8, 110 as i8, 111 as i8,
            99 as i8, 97 as i8, 115 as i8, 101 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 65 as i8,
            100 as i8, 100 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 78 as i8, 79 as i8, 67 as i8, 65 as i8, 83 as i8,
            69 as i8, 32 as i8, 99 as i8, 111 as i8, 108 as i8, 108 as i8,
            97 as i8, 116 as i8, 105 as i8, 110 as i8, 103 as i8, 32 as i8,
            115 as i8, 101 as i8, 113 as i8, 117 as i8, 101 as i8, 110 as i8,
            99 as i8, 101 as i8, 32 as i8, 116 as i8, 111 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 119 as i8, 111 as i8,
            114 as i8, 100 as i8, 115 as i8, 46 as i8, 10 as i8, 32 as i8,
            45 as i8, 45 as i8, 110 as i8, 111 as i8, 115 as i8, 121 as i8,
            110 as i8, 99 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8,
            32 as i8, 80 as i8, 82 as i8, 65 as i8, 71 as i8, 77 as i8,
            65 as i8, 32 as i8, 115 as i8, 121 as i8, 110 as i8, 99 as i8,
            104 as i8, 114 as i8, 111 as i8, 110 as i8, 111 as i8, 117 as i8,
            115 as i8, 61 as i8, 79 as i8, 70 as i8, 70 as i8, 10 as i8,
            32 as i8, 45 as i8, 45 as i8, 112 as i8, 97 as i8, 103 as i8,
            101 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8,
            78 as i8, 78 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8,
            101 as i8, 32 as i8, 97 as i8, 32 as i8, 112 as i8, 97 as i8,
            103 as i8, 101 as i8, 32 as i8, 115 as i8, 105 as i8, 122 as i8,
            101 as i8, 32 as i8, 111 as i8, 102 as i8, 32 as i8, 78 as i8,
            78 as i8, 78 as i8, 10 as i8, 32 as i8, 45 as i8, 45 as i8,
            113 as i8, 117 as i8, 101 as i8, 114 as i8, 121 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8, 81 as i8,
            85 as i8, 69 as i8, 82 as i8, 89 as i8, 32 as i8, 109 as i8,
            111 as i8, 100 as i8, 101 as i8, 10 as i8, 32 as i8, 45 as i8,
            45 as i8, 114 as i8, 101 as i8, 112 as i8, 108 as i8, 97 as i8,
            99 as i8, 101 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8,
            82 as i8, 69 as i8, 80 as i8, 76 as i8, 65 as i8, 67 as i8,
            69 as i8, 32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8,
            10 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8, 101 as i8,
            108 as i8, 101 as i8, 99 as i8, 116 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 85 as i8,
            115 as i8, 101 as i8, 32 as i8, 83 as i8, 69 as i8, 76 as i8,
            69 as i8, 67 as i8, 84 as i8, 32 as i8, 109 as i8, 111 as i8,
            100 as i8, 101 as i8, 10 as i8, 32 as i8, 45 as i8, 45 as i8,
            115 as i8, 116 as i8, 97 as i8, 116 as i8, 115 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 83 as i8, 104 as i8, 111 as i8, 119 as i8, 32 as i8,
            115 as i8, 113 as i8, 108 as i8, 105 as i8, 116 as i8, 101 as i8,
            51 as i8, 95 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8,
            117 as i8, 115 as i8, 40 as i8, 41 as i8, 32 as i8, 114 as i8,
            101 as i8, 115 as i8, 117 as i8, 108 as i8, 116 as i8, 115 as i8,
            32 as i8, 97 as i8, 116 as i8, 32 as i8, 116 as i8, 104 as i8,
            101 as i8, 32 as i8, 101 as i8, 110 as i8, 100 as i8, 46 as i8,
            10 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8, 117 as i8,
            109 as i8, 109 as i8, 97 as i8, 114 as i8, 121 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8,
            104 as i8, 111 as i8, 119 as i8, 32 as i8, 115 as i8, 117 as i8,
            109 as i8, 109 as i8, 97 as i8, 114 as i8, 121 as i8, 32 as i8,
            105 as i8, 110 as i8, 102 as i8, 111 as i8, 114 as i8, 109 as i8,
            97 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8, 32 as i8,
            111 as i8, 110 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 99 as i8, 111 as i8, 108 as i8, 108 as i8, 101 as i8,
            99 as i8, 116 as i8, 101 as i8, 100 as i8, 32 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 46 as i8, 10 as i8, 32 as i8,
            45 as i8, 45 as i8, 116 as i8, 97 as i8, 103 as i8, 32 as i8,
            78 as i8, 65 as i8, 77 as i8, 69 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 84 as i8, 97 as i8, 103 as i8,
            32 as i8, 97 as i8, 108 as i8, 108 as i8, 32 as i8, 111 as i8,
            117 as i8, 116 as i8, 112 as i8, 117 as i8, 116 as i8, 32 as i8,
            117 as i8, 115 as i8, 105 as i8, 110 as i8, 103 as i8, 32 as i8,
            78 as i8, 65 as i8, 77 as i8, 69 as i8, 46 as i8, 32 as i8,
            32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8, 111 as i8,
            110 as i8, 108 as i8, 121 as i8, 32 as i8, 115 as i8, 116 as i8,
            100 as i8, 111 as i8, 117 as i8, 116 as i8, 46 as i8, 10 as i8,
            32 as i8, 45 as i8, 45 as i8, 116 as i8, 105 as i8, 109 as i8,
            101 as i8, 114 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 84 as i8, 105 as i8,
            109 as i8, 101 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 111 as i8, 112 as i8, 101 as i8, 114 as i8, 97 as i8,
            116 as i8, 105 as i8, 111 as i8, 110 as i8, 32 as i8, 111 as i8,
            102 as i8, 32 as i8, 116 as i8, 104 as i8, 105 as i8, 115 as i8,
            32 as i8, 112 as i8, 114 as i8, 111 as i8, 103 as i8, 114 as i8,
            97 as i8, 109 as i8, 10 as i8, 32 as i8, 45 as i8, 45 as i8,
            116 as i8, 114 as i8, 97 as i8, 99 as i8, 101 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 69 as i8, 110 as i8, 97 as i8, 98 as i8, 108 as i8,
            101 as i8, 32 as i8, 115 as i8, 113 as i8, 108 as i8, 105 as i8,
            116 as i8, 101 as i8, 51 as i8, 95 as i8, 116 as i8, 114 as i8,
            97 as i8, 99 as i8, 101 as i8, 40 as i8, 41 as i8, 32 as i8,
            111 as i8, 117 as i8, 116 as i8, 112 as i8, 117 as i8, 116 as i8,
            46 as i8, 10 as i8, 32 as i8, 45 as i8, 45 as i8, 117 as i8,
            112 as i8, 100 as i8, 97 as i8, 116 as i8, 101 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            85 as i8, 115 as i8, 101 as i8, 32 as i8, 85 as i8, 80 as i8,
            68 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 109 as i8,
            111 as i8, 100 as i8, 101 as i8, 10 as i8, 32 as i8, 45 as i8,
            45 as i8, 117 as i8, 112 as i8, 115 as i8, 101 as i8, 114 as i8,
            116 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8,
            85 as i8, 80 as i8, 83 as i8, 69 as i8, 82 as i8, 84 as i8,
            32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8, 10 as i8,
            32 as i8, 45 as i8, 45 as i8, 119 as i8, 105 as i8, 116 as i8,
            104 as i8, 111 as i8, 117 as i8, 116 as i8, 45 as i8, 114 as i8,
            111 as i8, 119 as i8, 105 as i8, 100 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8,
            101 as i8, 32 as i8, 97 as i8, 32 as i8, 87 as i8, 73 as i8,
            84 as i8, 72 as i8, 79 as i8, 85 as i8, 84 as i8, 32 as i8,
            82 as i8, 79 as i8, 87 as i8, 73 as i8, 68 as i8, 32 as i8,
            116 as i8, 97 as i8, 98 as i8, 108 as i8, 101 as i8, 32 as i8,
            116 as i8, 111 as i8, 32 as i8, 115 as i8, 116 as i8, 111 as i8,
            114 as i8, 101 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 119 as i8, 111 as i8, 114 as i8, 100 as i8, 115 as i8,
            46 as i8, 10 as i8, 0 as i8];
#[unsafe(no_mangle)]
pub static mut z_tag: *mut i8 = c"--".as_ptr() as *mut i8;
extern "C" fn real_time() -> Sqlite3Int64 {
    unsafe {
        let mut t: Sqlite3Int64 = 0 as Sqlite3Int64;
        if clock_vfs == core::ptr::null_mut() {
            clock_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        }
        if unsafe { (*clock_vfs).i_version } >= 1 &&
                unsafe { (*clock_vfs).x_current_time_int64.is_some() } {
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time_int64.unwrap()
                    })(clock_vfs, &mut t)
            };
        } else {
            let mut r: f64 = 0.0;
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time.unwrap()
                    })(clock_vfs, &mut r)
            };
            t = (r * 86400000.0) as Sqlite3Int64;
        }
        return t;
    }
}
unsafe extern "C" fn fatal_error(z_msg_1: *const i8, mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_msg_1, ap) };
        ();
        unsafe { exit(1) };
    }
}
extern "C" fn usage() -> () {
    unsafe {
        printf(c"%s".as_ptr() as *mut i8 as *const i8,
            &raw const z_help[0 as usize] as *const i8)
    };
    unsafe { exit(0) };
}
extern "C" fn trace_callback(not_used_1: *mut (), z_sql_1: *const i8) -> () {
    unsafe { printf(c"%s;\n".as_ptr() as *mut i8 as *const i8, z_sql_1) };
}
extern "C" fn print_result(not_used_1: *mut (), n_arg_1: i32,
    az_arg_1: *mut *mut i8, az_nm_1: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        unsafe { printf(c"%s".as_ptr() as *mut i8 as *const i8, z_tag) };
        {
            i = 0;
            '__b0: loop {
                if !(i < n_arg_1) { break '__b0; }
                '__c0: loop {
                    unsafe {
                        printf(c" %s".as_ptr() as *mut i8 as *const i8,
                            if !(unsafe { *az_arg_1.offset(i as isize) }).is_null() {
                                unsafe { *az_arg_1.offset(i as isize) }
                            } else { c"(null)".as_ptr() as *mut i8 })
                    };
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
        return 0;
    }
}
extern "C" fn add_char_to_hash(a: *mut u32, x: u8) -> () {
    if unsafe { *a.offset(0 as isize) } < 4 as u32 {
        unsafe {
            *a.offset(1 as isize) =
                unsafe { *a.offset(1 as isize) } << 8 | x as u32
        };
        {
            let __p = unsafe { &mut *a.offset(0 as isize) };
            let __t = *__p;
            *__p += 1;
            __t
        };
    } else {
        unsafe {
            *a.offset(2 as isize) =
                unsafe { *a.offset(2 as isize) } << 8 | x as u32
        };
        {
            let __p = unsafe { &mut *a.offset(0 as isize) };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if unsafe { *a.offset(0 as isize) } == 8 as u32 {
            unsafe {
                *a.offset(3 as isize) +=
                    unsafe { *a.offset(1 as isize) } +
                        unsafe { *a.offset(4 as isize) }
            };
            unsafe {
                *a.offset(4 as isize) +=
                    unsafe { *a.offset(2 as isize) } +
                        unsafe { *a.offset(3 as isize) }
            };
            unsafe {
                *a.offset(0 as isize) =
                    {
                        unsafe {
                            *a.offset(1 as isize) =
                                {
                                    unsafe { *a.offset(2 as isize) = 0 as u32 };
                                    unsafe { *a.offset(2 as isize) }
                                }
                        };
                        unsafe { *a.offset(1 as isize) }
                    }
            };
        }
    }
}
extern "C" fn final_hash(a: *mut u32, z: *mut i8) -> () {
    unsafe {
        *a.offset(3 as isize) +=
            unsafe { *a.offset(1 as isize) } +
                    unsafe { *a.offset(4 as isize) } +
                unsafe { *a.offset(0 as isize) }
    };
    unsafe {
        *a.offset(4 as isize) +=
            unsafe { *a.offset(2 as isize) } +
                unsafe { *a.offset(3 as isize) }
    };
    unsafe {
        sqlite3_snprintf(17, z, c"%08x%08x".as_ptr() as *mut i8 as *const i8,
            unsafe { *a.offset(3 as isize) },
            unsafe { *a.offset(4 as isize) })
    };
}
extern "C" fn checksum_step(context: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_val: *const u8 = core::ptr::null();
    let mut n_val: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut a: *mut u32 = core::ptr::null_mut();
    a =
        unsafe {
                sqlite3_aggregate_context(context,
                    (core::mem::size_of::<u32>() as u64 * 5 as u64) as i32)
            } as *mut u32;
    if !(a).is_null() {
        {
            i = 0;
            '__b1: loop {
                if !(i < argc) { break '__b1; }
                '__c1: loop {
                    n_val =
                        unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(i as isize) })
                        };
                    z_val =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const u8;
                    if !(z_val).is_null() {
                        {
                            j = 0;
                            '__b2: loop {
                                if !(j < n_val) { break '__b2; }
                                '__c2: loop {
                                    add_char_to_hash(a, unsafe { *z_val.offset(j as isize) });
                                    break '__c2;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    add_char_to_hash(a, '|' as i32 as u8);
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        add_char_to_hash(a, '\n' as i32 as u8);
    }
}
extern "C" fn checksum_finalize(context: *mut Sqlite3Context) -> () {
    let mut a: *mut u32 = core::ptr::null_mut();
    let mut z_result: [i8; 24] = [0; 24];
    a = unsafe { sqlite3_aggregate_context(context, 0) } as *mut u32;
    if !(a).is_null() {
        final_hash(a, &raw mut z_result[0 as usize] as *mut i8);
        unsafe {
            sqlite3_result_text(context,
                &raw mut z_result[0 as usize] as *mut i8 as *const i8, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
    }
}
static mut az_mode: [*const i8; 7] =
    [c"--insert".as_ptr() as *const i8, c"--replace".as_ptr() as *const i8,
            c"--upsert".as_ptr() as *const i8,
            c"--select".as_ptr() as *const i8,
            c"--update".as_ptr() as *const i8,
            c"--delete".as_ptr() as *const i8,
            c"--query".as_ptr() as *const i8];
extern "C" fn all_loop(i_mode_1: i32, pi_loop_cnt_1: &mut i32,
    pi_mode2_1: &mut i32, p_use_without_rowid_1: &mut i32) -> i32 {
    let mut i: i32 = 0;
    if i_mode_1 != -1 {
        if *pi_loop_cnt_1 != 0 { return 0; }
        *pi_mode2_1 = i_mode_1;
        *pi_loop_cnt_1 = 1;
        return 1;
    }
    if *pi_loop_cnt_1 >= 7 * 2 { return 0; }
    i = { let __p = &mut *pi_loop_cnt_1; let __t = *__p; *__p += 1; __t };
    *p_use_without_rowid_1 = i & 1;
    *pi_mode2_1 = i >> 1;
    return 1;
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut z_file_to_read: *const i8 = core::ptr::null();
        let mut z_db_name: *const i8 = core::ptr::null();
        let mut use_without_rowid: i32 = 0;
        let mut i_mode: i32 = 0;
        let mut i_mode2: i32 = 0;
        let mut i_loop_cnt: i32 = 0;
        let mut use_nocase: i32 = 0;
        let mut do_trace: i32 = 0;
        let mut show_stats: i32 = 0;
        let mut show_summary: i32 = 0;
        let mut show_timer: i32 = 0;
        let mut cache_size: i32 = 0;
        let mut page_size: i32 = 0;
        let mut commit_interval: i32 = 0;
        let mut no_sync: i32 = 0;
        let mut z_j_mode: *const i8 = core::ptr::null();
        let mut n_op: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut p_insert: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut p_update: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut p_select: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut p_delete: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut i_cur: i32 = 0;
        let mut i_hiwtr: i32 = 0;
        let mut p_timer: *mut FILE = __stderrp;
        let mut sum_cnt: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut start_time: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut total_time: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut z_input: [i8; 2000] = [0; 2000];
        {
            i = 1;
            '__b3: loop {
                if !(i < argc) { break '__b3; }
                '__c3: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        '__b4: loop {
                            '__c4: loop {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                break '__c4;
                            }
                            if !(unsafe { *z.offset(0 as isize) } as i32 == '-' as i32)
                                {
                                break '__b4;
                            }
                        }
                        if unsafe {
                                    strcmp(z, c"without-rowid".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            use_without_rowid = 1;
                        } else if unsafe {
                                    strcmp(z, c"replace".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 1;
                        } else if unsafe {
                                    strcmp(z, c"upsert".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 2;
                        } else if unsafe {
                                    strcmp(z, c"select".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 3;
                        } else if unsafe {
                                    strcmp(z, c"insert".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 0;
                        } else if unsafe {
                                    strcmp(z, c"update".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 4;
                        } else if unsafe {
                                    strcmp(z, c"delete".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 5;
                        } else if unsafe {
                                    strcmp(z, c"query".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = 6;
                        } else if unsafe {
                                    strcmp(z, c"all".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            i_mode = -1;
                            show_timer = -99;
                        } else if unsafe {
                                    strcmp(z, c"nocase".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            use_nocase = 1;
                        } else if unsafe {
                                    strcmp(z, c"trace".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            do_trace = 1;
                        } else if unsafe {
                                    strcmp(z, c"nosync".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            no_sync = 1;
                        } else if unsafe {
                                    strcmp(z, c"stats".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            show_stats = 1;
                        } else if unsafe {
                                    strcmp(z, c"summary".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            show_summary = 1;
                        } else if unsafe {
                                    strcmp(z, c"timer".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            show_timer = i;
                        } else if unsafe {
                                        strcmp(z, c"cachesize".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i < argc - 1 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            cache_size =
                                unsafe {
                                    atoi(unsafe { *argv.offset(i as isize) } as *const i8)
                                };
                        } else if unsafe {
                                        strcmp(z, c"pagesize".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i < argc - 1 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            page_size =
                                unsafe {
                                    atoi(unsafe { *argv.offset(i as isize) } as *const i8)
                                };
                        } else if unsafe {
                                        strcmp(z, c"commit".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i < argc - 1 {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            commit_interval =
                                unsafe {
                                    atoi(unsafe { *argv.offset(i as isize) } as *const i8)
                                };
                        } else if unsafe {
                                        strcmp(z, c"journal".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i < argc - 1 {
                            z_j_mode =
                                unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8;
                        } else if unsafe {
                                        strcmp(z, c"tag".as_ptr() as *mut i8 as *const i8)
                                    } == 0 && i < argc - 1 {
                            z_tag =
                                unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                };
                            p_timer = __stdoutp;
                        } else if unsafe {
                                        strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe { strcmp(z, c"?".as_ptr() as *mut i8 as *const i8) }
                                    == 0 {
                            usage();
                        } else {
                            unsafe {
                                fatal_error(c"unknown option: \"%s\"\nUse --help for a list of options\n".as_ptr()
                                            as *mut i8 as *const i8,
                                    unsafe { *argv.offset(i as isize) })
                            };
                        }
                    } else if z_db_name == core::ptr::null() {
                        z_db_name =
                            unsafe { *argv.offset(i as isize) } as *const i8;
                    } else if z_file_to_read == core::ptr::null() {
                        z_file_to_read =
                            unsafe { *argv.offset(i as isize) } as *const i8;
                    } else {
                        unsafe {
                            fatal_error(c"surplus argument: \"%s\"\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if z_db_name == core::ptr::null() { usage(); }
        start_time = real_time();
        if unsafe { *z_db_name.offset(0 as isize) } != 0 &&
                unsafe {
                        strcmp(z_db_name,
                            c":memory:".as_ptr() as *mut i8 as *const i8)
                    } != 0 {
            unsafe { unlink(z_db_name) };
        }
        if unsafe { sqlite3_open(z_db_name, &mut db) } != 0 {
            unsafe {
                fatal_error(c"Cannot open database file: %s\n".as_ptr() as
                            *mut i8 as *const i8, z_db_name)
            };
        }
        if !(z_file_to_read).is_null() {
            in_ =
                unsafe {
                    fopen(z_file_to_read,
                        c"rb".as_ptr() as *mut i8 as *const i8)
                };
            if in_ == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"Could not open input file \"%s\"\n".as_ptr()
                                as *mut i8 as *const i8, z_file_to_read)
                };
            }
        } else {
            if i_mode == -1 {
                unsafe {
                    fatal_error(c"The --all mode cannot be used with stdin\n".as_ptr()
                                as *mut i8 as *const i8)
                };
            }
            in_ = __stdinp;
        }
        if do_trace != 0 {
            unsafe {
                sqlite3_trace(db, Some(trace_callback), core::ptr::null_mut())
            };
        }
        if page_size != 0 {
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"PRAGMA page_size=%d".as_ptr() as *mut i8
                            as *const i8, page_size)
                };
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
        }
        if cache_size != 0 {
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"PRAGMA cache_size=%d".as_ptr() as *mut i8
                            as *const i8, cache_size)
                };
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
        }
        if no_sync != 0 {
            unsafe {
                sqlite3_exec(db,
                    c"PRAGMA synchronous=OFF".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
        if !(z_j_mode).is_null() {
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"PRAGMA journal_mode=%s".as_ptr() as
                                *mut i8 as *const i8, z_j_mode)
                };
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
        }
        i_loop_cnt = 0;
        while all_loop(i_mode, &mut i_loop_cnt, &mut i_mode2,
                    &mut use_without_rowid) != 0 {
            if i_mode == -1 {
                if unsafe {
                            sqlite3_exec(db,
                                c"DROP TABLE IF EXISTS wordcount; VACUUM;".as_ptr() as
                                        *mut i8 as *const i8, None, core::ptr::null_mut(),
                                core::ptr::null_mut())
                        } != 0 {
                    unsafe {
                        fatal_error(c"Could not clean up prior iteration\n".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                }
                start_time = real_time();
                unsafe { rewind(in_) };
            }
            if unsafe {
                        sqlite3_exec(db,
                            c"BEGIN IMMEDIATE".as_ptr() as *mut i8 as *const i8, None,
                            core::ptr::null_mut(), core::ptr::null_mut())
                    } != 0 {
                unsafe {
                    fatal_error(c"Could not start a transaction\n".as_ptr() as
                                *mut i8 as *const i8)
                };
            }
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"CREATE TABLE IF NOT EXISTS wordcount(\n  word TEXT PRIMARY KEY COLLATE %s,\n  cnt INTEGER\n)%s".as_ptr()
                                as *mut i8 as *const i8,
                        if use_nocase != 0 {
                            c"nocase".as_ptr() as *mut i8
                        } else { c"binary".as_ptr() as *mut i8 },
                        if use_without_rowid != 0 {
                            c" WITHOUT ROWID".as_ptr() as *mut i8
                        } else { c"".as_ptr() as *mut i8 })
                };
            if z_sql == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"out of memory\n".as_ptr() as *mut i8 as
                            *const i8)
                };
            }
            rc =
                unsafe {
                    sqlite3_exec(db, z_sql as *const i8, None,
                        core::ptr::null_mut(), core::ptr::null_mut())
                };
            if rc != 0 {
                unsafe {
                    fatal_error(c"Could not create the wordcount table: %s.\n".as_ptr()
                                as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                };
            }
            unsafe { sqlite3_free(z_sql as *mut ()) };
            if i_mode2 == 6 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"SELECT cnt FROM wordcount WHERE word=?1".as_ptr() as
                                    *mut i8 as *const i8, -1, &mut p_select,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the SELECT statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 3 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"SELECT 1 FROM wordcount WHERE word=?1".as_ptr() as *mut i8
                                as *const i8, -1, &mut p_select, core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the SELECT statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"INSERT INTO wordcount(word,cnt) VALUES(?1,1)".as_ptr() as
                                    *mut i8 as *const i8, -1, &mut p_insert,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the INSERT statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 3 || i_mode2 == 4 || i_mode2 == 0 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"UPDATE wordcount SET cnt=cnt+1 WHERE word=?1".as_ptr() as
                                    *mut i8 as *const i8, -1, &mut p_update,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the UPDATE statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 0 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"INSERT OR IGNORE INTO wordcount(word,cnt) VALUES(?1,1)".as_ptr()
                                    as *mut i8 as *const i8, -1, &mut p_insert,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the INSERT statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 4 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"INSERT OR IGNORE INTO wordcount(word,cnt) VALUES(?1,0)".as_ptr()
                                    as *mut i8 as *const i8, -1, &mut p_insert,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the INSERT statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 1 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"REPLACE INTO wordcount(word,cnt)VALUES(?1,coalesce((SELECT cnt FROM wordcount WHERE word=?1),0)+1)".as_ptr()
                                    as *mut i8 as *const i8, -1, &mut p_insert,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the REPLACE statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 2 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"INSERT INTO wordcount(word,cnt) VALUES(?1,1) ON CONFLICT(word) DO UPDATE SET cnt=cnt+1".as_ptr()
                                    as *mut i8 as *const i8, -1, &mut p_insert,
                            core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the UPSERT statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            if i_mode2 == 5 {
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"DELETE FROM wordcount WHERE word=?1".as_ptr() as *mut i8
                                as *const i8, -1, &mut p_delete, core::ptr::null_mut())
                    };
                if rc != 0 {
                    unsafe {
                        fatal_error(c"Could not prepare the DELETE statement: %s\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                    };
                }
            }
            while !(unsafe {
                                fgets(&raw mut z_input[0 as usize] as *mut i8,
                                    core::mem::size_of::<[i8; 2000]>() as i32, in_)
                            }).is_null() {
                {
                    i = 0;
                    '__b7: loop {
                        if !(z_input[i as usize] != 0) { break '__b7; }
                        '__c7: loop {
                            if (unsafe { isalpha(z_input[i as usize] as u8 as i32) } ==
                                            0) as i32 != 0 {
                                break '__c7;
                            }
                            {
                                j = i + 1;
                                '__b8: loop {
                                    if !(unsafe { isalpha(z_input[j as usize] as u8 as i32) } !=
                                                    0) {
                                        break '__b8;
                                    }
                                    '__c8: loop { break '__c8; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if i_mode2 == 5 {
                                unsafe {
                                    sqlite3_bind_text(p_delete, 1,
                                        unsafe {
                                                (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                            } as *const i8, j - i, None)
                                };
                                if unsafe { sqlite3_step(p_delete) } != 101 {
                                    unsafe {
                                        fatal_error(c"DELETE failed: %s\n".as_ptr() as *mut i8 as
                                                *const i8, unsafe { sqlite3_errmsg(db) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_delete) };
                            } else if i_mode2 == 3 {
                                unsafe {
                                    sqlite3_bind_text(p_select, 1,
                                        unsafe {
                                                (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                            } as *const i8, j - i, None)
                                };
                                rc = unsafe { sqlite3_step(p_select) };
                                unsafe { sqlite3_reset(p_select) };
                                if rc == 100 {
                                    unsafe {
                                        sqlite3_bind_text(p_update, 1,
                                            unsafe {
                                                    (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                                } as *const i8, j - i, None)
                                    };
                                    if unsafe { sqlite3_step(p_update) } != 101 {
                                        unsafe {
                                            fatal_error(c"UPDATE failed: %s\n".as_ptr() as *mut i8 as
                                                    *const i8, unsafe { sqlite3_errmsg(db) })
                                        };
                                    }
                                    unsafe { sqlite3_reset(p_update) };
                                } else if rc == 101 {
                                    unsafe {
                                        sqlite3_bind_text(p_insert, 1,
                                            unsafe {
                                                    (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                                } as *const i8, j - i, None)
                                    };
                                    if unsafe { sqlite3_step(p_insert) } != 101 {
                                        unsafe {
                                            fatal_error(c"Insert failed: %s\n".as_ptr() as *mut i8 as
                                                    *const i8, unsafe { sqlite3_errmsg(db) })
                                        };
                                    }
                                    unsafe { sqlite3_reset(p_insert) };
                                } else {
                                    unsafe {
                                        fatal_error(c"SELECT failed: %s\n".as_ptr() as *mut i8 as
                                                *const i8, unsafe { sqlite3_errmsg(db) })
                                    };
                                }
                            } else if i_mode2 == 6 {
                                unsafe {
                                    sqlite3_bind_text(p_select, 1,
                                        unsafe {
                                                (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                            } as *const i8, j - i, None)
                                };
                                if unsafe { sqlite3_step(p_select) } == 100 {
                                    sum_cnt += unsafe { sqlite3_column_int64(p_select, 0) };
                                }
                                unsafe { sqlite3_reset(p_select) };
                            } else {
                                unsafe {
                                    sqlite3_bind_text(p_insert, 1,
                                        unsafe {
                                                (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                            } as *const i8, j - i, None)
                                };
                                if unsafe { sqlite3_step(p_insert) } != 101 {
                                    unsafe {
                                        fatal_error(c"INSERT failed: %s\n".as_ptr() as *mut i8 as
                                                *const i8, unsafe { sqlite3_errmsg(db) })
                                    };
                                }
                                unsafe { sqlite3_reset(p_insert) };
                                if i_mode2 == 4 ||
                                        i_mode2 == 0 && unsafe { sqlite3_changes(db) } == 0 {
                                    unsafe {
                                        sqlite3_bind_text(p_update, 1,
                                            unsafe {
                                                    (&raw mut z_input[0 as usize] as *mut i8).offset(i as isize)
                                                } as *const i8, j - i, None)
                                    };
                                    if unsafe { sqlite3_step(p_update) } != 101 {
                                        unsafe {
                                            fatal_error(c"UPDATE failed: %s\n".as_ptr() as *mut i8 as
                                                    *const i8, unsafe { sqlite3_errmsg(db) })
                                        };
                                    }
                                    unsafe { sqlite3_reset(p_update) };
                                }
                            }
                            i = j - 1;
                            { let __p = &mut n_op; let __t = *__p; *__p += 1; __t };
                            if commit_interval > 0 && n_op % commit_interval == 0 {
                                unsafe {
                                    sqlite3_exec(db,
                                        c"COMMIT; BEGIN IMMEDIATE".as_ptr() as *mut i8 as *const i8,
                                        None, core::ptr::null_mut(), core::ptr::null_mut())
                                };
                            }
                            break '__c7;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            unsafe {
                sqlite3_exec(db, c"COMMIT".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_finalize(p_insert) };
            p_insert = core::ptr::null_mut();
            unsafe { sqlite3_finalize(p_update) };
            p_update = core::ptr::null_mut();
            unsafe { sqlite3_finalize(p_select) };
            p_select = core::ptr::null_mut();
            unsafe { sqlite3_finalize(p_delete) };
            p_delete = core::ptr::null_mut();
            if i_mode2 == 6 && i_mode != -1 {
                unsafe {
                    printf(c"%s sum of cnt: %lld\n".as_ptr() as *mut i8 as
                            *const i8, z_tag, sum_cnt)
                };
                rc =
                    unsafe {
                        sqlite3_prepare_v2(db,
                            c"SELECT sum(cnt*cnt) FROM wordcount".as_ptr() as *mut i8 as
                                *const i8, -1, &mut p_select, core::ptr::null_mut())
                    };
                if rc == 0 && unsafe { sqlite3_step(p_select) } == 100 {
                    unsafe {
                        printf(c"%s double-check: %lld\n".as_ptr() as *mut i8 as
                                *const i8, z_tag,
                            unsafe { sqlite3_column_int64(p_select, 0) })
                    };
                }
                unsafe { sqlite3_finalize(p_select) };
            }
            if show_timer != 0 {
                let elapse_time: Sqlite3Int64 = real_time() - start_time;
                total_time += elapse_time;
                unsafe {
                    fprintf(p_timer,
                        c"%3d.%03d wordcount".as_ptr() as *mut i8 as *const i8,
                        (elapse_time / 1000 as Sqlite3Int64) as i32,
                        (elapse_time % 1000 as Sqlite3Int64) as i32)
                };
                if i_mode == -1 {
                    unsafe {
                        fprintf(p_timer,
                            c" %s%s\n".as_ptr() as *mut i8 as *const i8,
                            az_mode[i_mode2 as usize],
                            if use_without_rowid != 0 {
                                c" --without-rowid".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 })
                    };
                } else {
                    {
                        i = 1;
                        '__b9: loop {
                            if !(i < argc) { break '__b9; }
                            '__c9: loop {
                                if i != show_timer {
                                    unsafe {
                                        fprintf(p_timer, c" %s".as_ptr() as *mut i8 as *const i8,
                                            unsafe { *argv.offset(i as isize) })
                                    };
                                }
                                break '__c9;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        fprintf(p_timer, c"\n".as_ptr() as *mut i8 as *const i8)
                    };
                }
            }
            if show_summary != 0 {
                unsafe {
                    sqlite3_create_function(db,
                        c"checksum".as_ptr() as *mut i8 as *const i8, -1, 1,
                        core::ptr::null_mut(), None, Some(checksum_step),
                        Some(checksum_finalize))
                };
                unsafe {
                    sqlite3_exec(db,
                        c"SELECT \'count(*):  \', count(*) FROM wordcount;\nSELECT \'sum(cnt):  \', sum(cnt) FROM wordcount;\nSELECT \'max(cnt):  \', max(cnt) FROM wordcount;\nSELECT \'avg(cnt):  \', avg(cnt) FROM wordcount;\nSELECT \'sum(cnt=1):\', sum(cnt=1) FROM wordcount;\nSELECT \'top 10:    \', group_concat(word, \', \') FROM (SELECT word FROM wordcount ORDER BY cnt DESC, word LIMIT 10);\nSELECT \'checksum:  \', checksum(word, cnt) FROM (SELECT word, cnt FROM wordcount ORDER BY word);\nPRAGMA integrity_check;\n".as_ptr()
                                as *mut i8 as *const i8, Some(print_result),
                        core::ptr::null_mut(), core::ptr::null_mut())
                };
            }
        }
        if !(z_file_to_read).is_null() { unsafe { fclose(in_) }; }
        if i_mode == -1 && show_timer != 0 {
            unsafe {
                fprintf(p_timer,
                    c"%3d.%03d wordcount --all\n".as_ptr() as *mut i8 as
                        *const i8, (total_time / 1000 as Sqlite3Int64) as i32,
                    (total_time % 1000 as Sqlite3Int64) as i32)
            };
        }
        if show_stats != 0 {
            unsafe { sqlite3_db_status(db, 0, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Lookaside Slots Used:        %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur, i_hiwtr)
            };
            unsafe { sqlite3_db_status(db, 4, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Successful lookasides:       %d\n".as_ptr() as
                            *mut i8 as *const i8, z_tag, i_hiwtr)
            };
            unsafe { sqlite3_db_status(db, 5, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Lookaside size faults:       %d\n".as_ptr() as
                            *mut i8 as *const i8, z_tag, i_hiwtr)
            };
            unsafe { sqlite3_db_status(db, 6, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Lookaside OOM faults:        %d\n".as_ptr() as
                            *mut i8 as *const i8, z_tag, i_hiwtr)
            };
            unsafe { sqlite3_db_status(db, 1, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Pager Heap Usage:            %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur)
            };
            unsafe { sqlite3_db_status(db, 7, &mut i_cur, &mut i_hiwtr, 1) };
            unsafe {
                printf(c"%s Page cache hits:             %d\n".as_ptr() as
                            *mut i8 as *const i8, z_tag, i_cur)
            };
            unsafe { sqlite3_db_status(db, 8, &mut i_cur, &mut i_hiwtr, 1) };
            unsafe {
                printf(c"%s Page cache misses:           %d\n".as_ptr() as
                            *mut i8 as *const i8, z_tag, i_cur)
            };
            unsafe { sqlite3_db_status(db, 9, &mut i_cur, &mut i_hiwtr, 1) };
            unsafe {
                printf(c"%s Page cache writes:           %d\n".as_ptr() as
                            *mut i8 as *const i8, z_tag, i_cur)
            };
            unsafe { sqlite3_db_status(db, 2, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Schema Heap Usage:           %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur)
            };
            unsafe { sqlite3_db_status(db, 3, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Statement Heap Usage:        %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur)
            };
        }
        unsafe { sqlite3_close(db) };
        if show_stats != 0 {
            unsafe { sqlite3_status(0, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Memory Used (bytes):         %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur, i_hiwtr)
            };
            unsafe { sqlite3_status(9, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Outstanding Allocations:     %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur, i_hiwtr)
            };
            unsafe { sqlite3_status(2, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Pcache Overflow Bytes:       %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_cur, i_hiwtr)
            };
            unsafe { sqlite3_status(5, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Largest Allocation:          %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_hiwtr)
            };
            unsafe { sqlite3_status(7, &mut i_cur, &mut i_hiwtr, 0) };
            unsafe {
                printf(c"%s Largest Pcache Allocation:   %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, z_tag, i_hiwtr)
            };
        }
        return Ok(());
    }
}
static mut clock_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
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
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn atoi(_: *const i8)
    -> i32;
    fn unlink(_: *const i8)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn rewind(_: *mut FILE)
    -> ();
    fn fgets(_: *mut i8, __size: i32, _: *mut FILE)
    -> *mut i8;
    fn isalpha(_c: i32)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn fclose(_: *mut FILE)
    -> i32;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stdinp: *mut FILE;
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