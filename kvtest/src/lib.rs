#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;

type DarwinSizeT = u64;

type Int64T = i64;

type DarwinOffT = Int64T;

type OffT = DarwinOffT;

type Uint16T = u16;

type DarwinModeT = Uint16T;

type ModeT = DarwinModeT;

type Int32T = i32;

type DarwinDevT = Int32T;

type DevT = DarwinDevT;

type NlinkT = Uint16T;

type Uint64T = u64;

type DarwinIno64T = Uint64T;

type Uint32T = u32;

type DarwinUidT = Uint32T;

type UidT = DarwinUidT;

type DarwinGidT = Uint32T;

type GidT = DarwinGidT;

type DarwinBlkcntT = Int64T;

type BlkcntT = DarwinBlkcntT;

type DarwinBlksizeT = Int32T;

type BlksizeT = DarwinBlksizeT;

type DarwinTimeT = i64;

#[repr(C)]
#[derive(Copy, Clone)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Stat {
    st_dev: i32,
    st_mode: u16,
    st_nlink: u16,
    st_ino: u64,
    st_uid: u32,
    st_gid: u32,
    st_rdev: i32,
    st_atimespec: Timespec,
    st_mtimespec: Timespec,
    st_ctimespec: Timespec,
    st_birthtimespec: Timespec,
    st_size: i64,
    st_blocks: i64,
    st_blksize: i32,
    st_flags: u32,
    st_gen: u32,
    st_lspare: i32,
    st_qspare: [i64; 2],
}

static z_help: [i8; 2265] =
    [85 as i8, 115 as i8, 97 as i8, 103 as i8, 101 as i8, 58 as i8, 32 as i8,
            107 as i8, 118 as i8, 116 as i8, 101 as i8, 115 as i8, 116 as i8,
            32 as i8, 67 as i8, 79 as i8, 77 as i8, 77 as i8, 65 as i8,
            78 as i8, 68 as i8, 32 as i8, 65 as i8, 82 as i8, 71 as i8,
            83 as i8, 46 as i8, 46 as i8, 46 as i8, 10 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 107 as i8, 118 as i8, 116 as i8,
            101 as i8, 115 as i8, 116 as i8, 32 as i8, 105 as i8, 110 as i8,
            105 as i8, 116 as i8, 32 as i8, 68 as i8, 66 as i8, 70 as i8,
            73 as i8, 76 as i8, 69 as i8, 32 as i8, 45 as i8, 45 as i8,
            99 as i8, 111 as i8, 117 as i8, 110 as i8, 116 as i8, 32 as i8,
            78 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8, 105 as i8,
            122 as i8, 101 as i8, 32 as i8, 77 as i8, 32 as i8, 45 as i8,
            45 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8, 115 as i8,
            105 as i8, 122 as i8, 101 as i8, 32 as i8, 88 as i8, 10 as i8,
            10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 71 as i8, 101 as i8, 110 as i8,
            101 as i8, 114 as i8, 97 as i8, 116 as i8, 101 as i8, 32 as i8,
            97 as i8, 32 as i8, 110 as i8, 101 as i8, 119 as i8, 32 as i8,
            116 as i8, 101 as i8, 115 as i8, 116 as i8, 32 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 32 as i8, 102 as i8, 105 as i8, 108 as i8, 101 as i8,
            32 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8, 100 as i8,
            32 as i8, 68 as i8, 66 as i8, 70 as i8, 73 as i8, 76 as i8,
            69 as i8, 32 as i8, 99 as i8, 111 as i8, 110 as i8, 116 as i8,
            97 as i8, 105 as i8, 110 as i8, 105 as i8, 110 as i8, 103 as i8,
            32 as i8, 78 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 66 as i8,
            76 as i8, 79 as i8, 66 as i8, 115 as i8, 32 as i8, 101 as i8,
            97 as i8, 99 as i8, 104 as i8, 32 as i8, 111 as i8, 102 as i8,
            32 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8,
            77 as i8, 32 as i8, 98 as i8, 121 as i8, 116 as i8, 101 as i8,
            115 as i8, 46 as i8, 32 as i8, 32 as i8, 84 as i8, 104 as i8,
            101 as i8, 32 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8,
            32 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8,
            111 as i8, 102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 110 as i8, 101 as i8, 119 as i8, 32 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 102 as i8, 105 as i8,
            108 as i8, 101 as i8, 32 as i8, 119 as i8, 105 as i8, 108 as i8,
            108 as i8, 32 as i8, 98 as i8, 101 as i8, 32 as i8, 88 as i8,
            46 as i8, 32 as i8, 32 as i8, 65 as i8, 100 as i8, 100 as i8,
            105 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8, 97 as i8,
            108 as i8, 32 as i8, 111 as i8, 112 as i8, 116 as i8, 105 as i8,
            111 as i8, 110 as i8, 115 as i8, 58 as i8, 10 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 118 as i8, 97 as i8, 114 as i8, 105 as i8, 97 as i8,
            110 as i8, 99 as i8, 101 as i8, 32 as i8, 86 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 82 as i8, 97 as i8,
            110 as i8, 100 as i8, 111 as i8, 109 as i8, 108 as i8, 121 as i8,
            32 as i8, 118 as i8, 97 as i8, 114 as i8, 121 as i8, 32 as i8,
            77 as i8, 32 as i8, 98 as i8, 121 as i8, 32 as i8, 112 as i8,
            108 as i8, 117 as i8, 115 as i8, 32 as i8, 111 as i8, 114 as i8,
            32 as i8, 109 as i8, 105 as i8, 110 as i8, 117 as i8, 115 as i8,
            32 as i8, 86 as i8, 10 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 107 as i8, 118 as i8, 116 as i8, 101 as i8, 115 as i8,
            116 as i8, 32 as i8, 101 as i8, 120 as i8, 112 as i8, 111 as i8,
            114 as i8, 116 as i8, 32 as i8, 68 as i8, 66 as i8, 70 as i8,
            73 as i8, 76 as i8, 69 as i8, 32 as i8, 68 as i8, 73 as i8,
            82 as i8, 69 as i8, 67 as i8, 84 as i8, 79 as i8, 82 as i8,
            89 as i8, 32 as i8, 91 as i8, 45 as i8, 45 as i8, 116 as i8,
            114 as i8, 101 as i8, 101 as i8, 93 as i8, 10 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 69 as i8, 120 as i8, 112 as i8, 111 as i8,
            114 as i8, 116 as i8, 32 as i8, 97 as i8, 108 as i8, 108 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 115 as i8, 32 as i8, 105 as i8,
            110 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            107 as i8, 118 as i8, 32 as i8, 116 as i8, 97 as i8, 98 as i8,
            108 as i8, 101 as i8, 32 as i8, 111 as i8, 102 as i8, 32 as i8,
            68 as i8, 66 as i8, 70 as i8, 73 as i8, 76 as i8, 69 as i8,
            32 as i8, 105 as i8, 110 as i8, 116 as i8, 111 as i8, 32 as i8,
            115 as i8, 101 as i8, 112 as i8, 97 as i8, 114 as i8, 97 as i8,
            116 as i8, 101 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 115 as i8, 32 as i8, 105 as i8,
            110 as i8, 32 as i8, 68 as i8, 73 as i8, 82 as i8, 69 as i8,
            67 as i8, 84 as i8, 79 as i8, 82 as i8, 89 as i8, 46 as i8,
            32 as i8, 32 as i8, 68 as i8, 73 as i8, 82 as i8, 69 as i8,
            67 as i8, 84 as i8, 79 as i8, 82 as i8, 89 as i8, 32 as i8,
            105 as i8, 115 as i8, 32 as i8, 99 as i8, 114 as i8, 101 as i8,
            97 as i8, 116 as i8, 101 as i8, 100 as i8, 32 as i8, 105 as i8,
            102 as i8, 32 as i8, 105 as i8, 116 as i8, 32 as i8, 100 as i8,
            111 as i8, 101 as i8, 115 as i8, 32 as i8, 110 as i8, 111 as i8,
            116 as i8, 32 as i8, 112 as i8, 114 as i8, 101 as i8, 118 as i8,
            105 as i8, 111 as i8, 117 as i8, 115 as i8, 108 as i8, 121 as i8,
            10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 101 as i8, 120 as i8, 105 as i8,
            115 as i8, 116 as i8, 46 as i8, 32 as i8, 32 as i8, 73 as i8,
            102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            45 as i8, 45 as i8, 116 as i8, 114 as i8, 101 as i8, 101 as i8,
            32 as i8, 111 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8,
            110 as i8, 32 as i8, 105 as i8, 115 as i8, 32 as i8, 117 as i8,
            115 as i8, 101 as i8, 100 as i8, 44 as i8, 32 as i8, 116 as i8,
            104 as i8, 101 as i8, 110 as i8, 32 as i8, 116 as i8, 104 as i8,
            101 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8,
            115 as i8, 32 as i8, 97 as i8, 114 as i8, 101 as i8, 32 as i8,
            119 as i8, 114 as i8, 105 as i8, 116 as i8, 116 as i8, 101 as i8,
            110 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 105 as i8, 110 as i8,
            116 as i8, 111 as i8, 32 as i8, 97 as i8, 32 as i8, 104 as i8,
            105 as i8, 101 as i8, 114 as i8, 97 as i8, 114 as i8, 99 as i8,
            104 as i8, 121 as i8, 32 as i8, 111 as i8, 102 as i8, 32 as i8,
            100 as i8, 105 as i8, 114 as i8, 101 as i8, 99 as i8, 116 as i8,
            111 as i8, 114 as i8, 105 as i8, 101 as i8, 115 as i8, 44 as i8,
            32 as i8, 117 as i8, 115 as i8, 105 as i8, 110 as i8, 103 as i8,
            32 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8, 115 as i8,
            32 as i8, 108 as i8, 105 as i8, 107 as i8, 101 as i8, 32 as i8,
            48 as i8, 48 as i8, 47 as i8, 48 as i8, 48 as i8, 47 as i8,
            48 as i8, 48 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            48 as i8, 48 as i8, 47 as i8, 48 as i8, 48 as i8, 47 as i8,
            48 as i8, 49 as i8, 44 as i8, 32 as i8, 48 as i8, 48 as i8,
            47 as i8, 48 as i8, 48 as i8, 47 as i8, 48 as i8, 50 as i8,
            44 as i8, 32 as i8, 97 as i8, 110 as i8, 100 as i8, 32 as i8,
            115 as i8, 111 as i8, 32 as i8, 102 as i8, 111 as i8, 114 as i8,
            116 as i8, 104 as i8, 46 as i8, 32 as i8, 32 as i8, 87 as i8,
            105 as i8, 116 as i8, 104 as i8, 111 as i8, 117 as i8, 116 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 45 as i8,
            45 as i8, 116 as i8, 114 as i8, 101 as i8, 101 as i8, 32 as i8,
            111 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            44 as i8, 32 as i8, 97 as i8, 108 as i8, 108 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 102 as i8, 105 as i8, 108 as i8, 101 as i8,
            115 as i8, 32 as i8, 97 as i8, 114 as i8, 101 as i8, 32 as i8,
            105 as i8, 110 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 116 as i8, 111 as i8, 112 as i8, 45 as i8, 108 as i8,
            101 as i8, 118 as i8, 101 as i8, 108 as i8, 32 as i8, 100 as i8,
            105 as i8, 114 as i8, 101 as i8, 99 as i8, 116 as i8, 111 as i8,
            114 as i8, 121 as i8, 32 as i8, 119 as i8, 105 as i8, 116 as i8,
            104 as i8, 32 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8,
            115 as i8, 32 as i8, 108 as i8, 105 as i8, 107 as i8, 101 as i8,
            32 as i8, 48 as i8, 48 as i8, 48 as i8, 48 as i8, 48 as i8,
            48 as i8, 44 as i8, 32 as i8, 48 as i8, 48 as i8, 48 as i8,
            48 as i8, 48 as i8, 49 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 48 as i8, 48 as i8, 48 as i8, 48 as i8, 48 as i8,
            50 as i8, 44 as i8, 32 as i8, 97 as i8, 110 as i8, 100 as i8,
            32 as i8, 115 as i8, 111 as i8, 32 as i8, 102 as i8, 111 as i8,
            114 as i8, 116 as i8, 104 as i8, 46 as i8, 10 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 107 as i8, 118 as i8, 116 as i8,
            101 as i8, 115 as i8, 116 as i8, 32 as i8, 115 as i8, 116 as i8,
            97 as i8, 116 as i8, 32 as i8, 68 as i8, 66 as i8, 70 as i8,
            73 as i8, 76 as i8, 69 as i8, 32 as i8, 91 as i8, 111 as i8,
            112 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8, 115 as i8,
            93 as i8, 10 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 68 as i8,
            105 as i8, 115 as i8, 112 as i8, 108 as i8, 97 as i8, 121 as i8,
            32 as i8, 115 as i8, 117 as i8, 109 as i8, 109 as i8, 97 as i8,
            114 as i8, 121 as i8, 32 as i8, 105 as i8, 110 as i8, 102 as i8,
            111 as i8, 114 as i8, 109 as i8, 97 as i8, 116 as i8, 105 as i8,
            111 as i8, 110 as i8, 32 as i8, 97 as i8, 98 as i8, 111 as i8,
            117 as i8, 116 as i8, 32 as i8, 68 as i8, 66 as i8, 70 as i8,
            73 as i8, 76 as i8, 69 as i8, 46 as i8, 32 as i8, 32 as i8,
            79 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            115 as i8, 58 as i8, 10 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 118 as i8,
            97 as i8, 99 as i8, 117 as i8, 117 as i8, 109 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 82 as i8, 117 as i8, 110 as i8, 32 as i8,
            86 as i8, 65 as i8, 67 as i8, 85 as i8, 85 as i8, 77 as i8,
            32 as i8, 111 as i8, 110 as i8, 32 as i8, 116 as i8, 104 as i8,
            101 as i8, 32 as i8, 100 as i8, 97 as i8, 116 as i8, 97 as i8,
            98 as i8, 97 as i8, 115 as i8, 101 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 10 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 107 as i8, 118 as i8, 116 as i8, 101 as i8,
            115 as i8, 116 as i8, 32 as i8, 114 as i8, 117 as i8, 110 as i8,
            32 as i8, 68 as i8, 66 as i8, 70 as i8, 73 as i8, 76 as i8,
            69 as i8, 32 as i8, 91 as i8, 111 as i8, 112 as i8, 116 as i8,
            105 as i8, 111 as i8, 110 as i8, 115 as i8, 93 as i8, 10 as i8,
            10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 82 as i8, 117 as i8, 110 as i8,
            32 as i8, 97 as i8, 32 as i8, 112 as i8, 101 as i8, 114 as i8,
            102 as i8, 111 as i8, 114 as i8, 109 as i8, 97 as i8, 110 as i8,
            99 as i8, 101 as i8, 32 as i8, 116 as i8, 101 as i8, 115 as i8,
            116 as i8, 46 as i8, 32 as i8, 32 as i8, 68 as i8, 66 as i8,
            70 as i8, 73 as i8, 76 as i8, 69 as i8, 32 as i8, 99 as i8,
            97 as i8, 110 as i8, 32 as i8, 98 as i8, 101 as i8, 32 as i8,
            101 as i8, 105 as i8, 116 as i8, 104 as i8, 101 as i8, 114 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 32 as i8, 111 as i8, 102 as i8,
            32 as i8, 97 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 32 as i8, 111 as i8, 114 as i8, 32 as i8, 97 as i8,
            32 as i8, 100 as i8, 105 as i8, 114 as i8, 101 as i8, 99 as i8,
            116 as i8, 111 as i8, 114 as i8, 121 as i8, 32 as i8, 99 as i8,
            111 as i8, 110 as i8, 116 as i8, 97 as i8, 105 as i8, 110 as i8,
            105 as i8, 110 as i8, 103 as i8, 32 as i8, 115 as i8, 97 as i8,
            109 as i8, 112 as i8, 108 as i8, 101 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 115 as i8, 46 as i8, 32 as i8,
            32 as i8, 79 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8,
            110 as i8, 115 as i8, 58 as i8, 10 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            97 as i8, 115 as i8, 99 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 82 as i8, 101 as i8, 97 as i8,
            100 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8,
            115 as i8, 32 as i8, 105 as i8, 110 as i8, 32 as i8, 97 as i8,
            115 as i8, 99 as i8, 101 as i8, 110 as i8, 100 as i8, 105 as i8,
            110 as i8, 103 as i8, 32 as i8, 111 as i8, 114 as i8, 100 as i8,
            101 as i8, 114 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 98 as i8, 108 as i8,
            111 as i8, 98 as i8, 45 as i8, 97 as i8, 112 as i8, 105 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8, 116 as i8,
            104 as i8, 101 as i8, 32 as i8, 66 as i8, 76 as i8, 79 as i8,
            66 as i8, 32 as i8, 65 as i8, 80 as i8, 73 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8, 101 as i8,
            45 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8,
            78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 68 as i8, 97 as i8,
            116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8, 101 as i8,
            32 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8, 101 as i8,
            32 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 99 as i8, 111 as i8, 117 as i8, 110 as i8, 116 as i8,
            32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 82 as i8, 101 as i8,
            97 as i8, 100 as i8, 32 as i8, 78 as i8, 32 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 115 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            100 as i8, 101 as i8, 115 as i8, 99 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 82 as i8, 101 as i8, 97 as i8,
            100 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8,
            115 as i8, 32 as i8, 105 as i8, 110 as i8, 32 as i8, 100 as i8,
            101 as i8, 115 as i8, 99 as i8, 101 as i8, 110 as i8, 100 as i8,
            105 as i8, 110 as i8, 103 as i8, 32 as i8, 111 as i8, 114 as i8,
            100 as i8, 101 as i8, 114 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 102 as i8,
            115 as i8, 121 as i8, 110 as i8, 99 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 83 as i8, 121 as i8, 110 as i8, 99 as i8,
            104 as i8, 114 as i8, 111 as i8, 110 as i8, 111 as i8, 117 as i8,
            115 as i8, 32 as i8, 102 as i8, 105 as i8, 108 as i8, 101 as i8,
            32 as i8, 119 as i8, 114 as i8, 105 as i8, 116 as i8, 101 as i8,
            115 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 105 as i8, 110 as i8, 116 as i8,
            101 as i8, 103 as i8, 114 as i8, 105 as i8, 116 as i8, 121 as i8,
            45 as i8, 99 as i8, 104 as i8, 101 as i8, 99 as i8, 107 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            82 as i8, 117 as i8, 110 as i8, 32 as i8, 34 as i8, 80 as i8,
            82 as i8, 65 as i8, 71 as i8, 77 as i8, 65 as i8, 32 as i8,
            105 as i8, 110 as i8, 116 as i8, 101 as i8, 103 as i8, 114 as i8,
            105 as i8, 116 as i8, 121 as i8, 95 as i8, 99 as i8, 104 as i8,
            101 as i8, 99 as i8, 107 as i8, 34 as i8, 32 as i8, 97 as i8,
            102 as i8, 116 as i8, 101 as i8, 114 as i8, 32 as i8, 116 as i8,
            101 as i8, 115 as i8, 116 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 109 as i8,
            97 as i8, 120 as i8, 45 as i8, 105 as i8, 100 as i8, 32 as i8,
            78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 77 as i8, 97 as i8, 120 as i8, 105 as i8,
            109 as i8, 117 as i8, 109 as i8, 32 as i8, 98 as i8, 108 as i8,
            111 as i8, 98 as i8, 32 as i8, 107 as i8, 101 as i8, 121 as i8,
            32 as i8, 116 as i8, 111 as i8, 32 as i8, 117 as i8, 115 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 109 as i8, 109 as i8, 97 as i8,
            112 as i8, 32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            77 as i8, 109 as i8, 97 as i8, 112 as i8, 32 as i8, 97 as i8,
            115 as i8, 32 as i8, 109 as i8, 117 as i8, 99 as i8, 104 as i8,
            32 as i8, 97 as i8, 115 as i8, 32 as i8, 78 as i8, 32 as i8,
            98 as i8, 121 as i8, 116 as i8, 101 as i8, 115 as i8, 32 as i8,
            111 as i8, 102 as i8, 32 as i8, 68 as i8, 66 as i8, 70 as i8,
            73 as i8, 76 as i8, 69 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 109 as i8,
            117 as i8, 108 as i8, 116 as i8, 105 as i8, 116 as i8, 114 as i8,
            97 as i8, 110 as i8, 115 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 69 as i8, 97 as i8, 99 as i8, 104 as i8,
            32 as i8, 114 as i8, 101 as i8, 97 as i8, 100 as i8, 32 as i8,
            111 as i8, 114 as i8, 32 as i8, 119 as i8, 114 as i8, 105 as i8,
            116 as i8, 101 as i8, 32 as i8, 105 as i8, 110 as i8, 32 as i8,
            105 as i8, 116 as i8, 115 as i8, 32 as i8, 111 as i8, 119 as i8,
            110 as i8, 32 as i8, 116 as i8, 114 as i8, 97 as i8, 110 as i8,
            115 as i8, 97 as i8, 99 as i8, 116 as i8, 105 as i8, 111 as i8,
            110 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 110 as i8, 111 as i8, 99 as i8,
            104 as i8, 101 as i8, 99 as i8, 107 as i8, 112 as i8, 111 as i8,
            105 as i8, 110 as i8, 116 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            79 as i8, 109 as i8, 105 as i8, 116 as i8, 32 as i8, 116 as i8,
            104 as i8, 101 as i8, 32 as i8, 99 as i8, 104 as i8, 101 as i8,
            99 as i8, 107 as i8, 112 as i8, 111 as i8, 105 as i8, 110 as i8,
            116 as i8, 32 as i8, 111 as i8, 110 as i8, 32 as i8, 87 as i8,
            65 as i8, 76 as i8, 32 as i8, 109 as i8, 111 as i8, 100 as i8,
            101 as i8, 32 as i8, 119 as i8, 114 as i8, 105 as i8, 116 as i8,
            101 as i8, 115 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 110 as i8, 111 as i8,
            115 as i8, 121 as i8, 110 as i8, 99 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 83 as i8, 101 as i8, 116 as i8, 32 as i8, 34 as i8,
            80 as i8, 82 as i8, 65 as i8, 71 as i8, 77 as i8, 65 as i8,
            32 as i8, 115 as i8, 121 as i8, 110 as i8, 99 as i8, 104 as i8,
            114 as i8, 111 as i8, 110 as i8, 111 as i8, 117 as i8, 115 as i8,
            61 as i8, 79 as i8, 70 as i8, 70 as i8, 34 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 106 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8,
            32 as i8, 77 as i8, 79 as i8, 68 as i8, 69 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8,
            116 as i8, 32 as i8, 77 as i8, 79 as i8, 68 as i8, 69 as i8,
            32 as i8, 106 as i8, 111 as i8, 117 as i8, 114 as i8, 110 as i8,
            97 as i8, 108 as i8, 32 as i8, 109 as i8, 111 as i8, 100 as i8,
            101 as i8, 32 as i8, 112 as i8, 114 as i8, 105 as i8, 111 as i8,
            114 as i8, 32 as i8, 116 as i8, 111 as i8, 32 as i8, 115 as i8,
            116 as i8, 97 as i8, 114 as i8, 116 as i8, 105 as i8, 110 as i8,
            103 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 114 as i8, 97 as i8, 110 as i8,
            100 as i8, 111 as i8, 109 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            82 as i8, 101 as i8, 97 as i8, 100 as i8, 32 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 115 as i8, 32 as i8, 105 as i8,
            110 as i8, 32 as i8, 97 as i8, 32 as i8, 114 as i8, 97 as i8,
            110 as i8, 100 as i8, 111 as i8, 109 as i8, 32 as i8, 111 as i8,
            114 as i8, 100 as i8, 101 as i8, 114 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            115 as i8, 116 as i8, 97 as i8, 114 as i8, 116 as i8, 32 as i8,
            78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 83 as i8, 116 as i8, 97 as i8,
            114 as i8, 116 as i8, 32 as i8, 114 as i8, 101 as i8, 97 as i8,
            100 as i8, 105 as i8, 110 as i8, 103 as i8, 32 as i8, 119 as i8,
            105 as i8, 116 as i8, 104 as i8, 32 as i8, 116 as i8, 104 as i8,
            105 as i8, 115 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8,
            98 as i8, 32 as i8, 107 as i8, 101 as i8, 121 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8, 115 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 79 as i8, 117 as i8,
            116 as i8, 112 as i8, 117 as i8, 116 as i8, 32 as i8, 111 as i8,
            112 as i8, 101 as i8, 114 as i8, 97 as i8, 116 as i8, 105 as i8,
            110 as i8, 103 as i8, 32 as i8, 115 as i8, 116 as i8, 97 as i8,
            116 as i8, 115 as i8, 32 as i8, 98 as i8, 101 as i8, 102 as i8,
            111 as i8, 114 as i8, 101 as i8, 32 as i8, 101 as i8, 120 as i8,
            105 as i8, 116 as i8, 105 as i8, 110 as i8, 103 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 117 as i8, 112 as i8, 100 as i8, 97 as i8, 116 as i8,
            101 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 68 as i8, 111 as i8,
            32 as i8, 97 as i8, 110 as i8, 32 as i8, 111 as i8, 118 as i8,
            101 as i8, 114 as i8, 119 as i8, 114 as i8, 105 as i8, 116 as i8,
            101 as i8, 32 as i8, 116 as i8, 101 as i8, 115 as i8, 116 as i8,
            10 as i8, 0 as i8];

extern "C" fn show_help() -> () {
    unsafe {
        unsafe {
            fprintf(__stdoutp, c"%s".as_ptr() as *mut i8 as *const i8,
                &raw const z_help[0 as usize] as *const i8)
        };
        unsafe { exit(1) };
    }
}

unsafe extern "C" fn fatal_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe {
            fprintf(__stdoutp, c"ERROR: ".as_ptr() as *mut i8 as *const i8)
        };
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stdoutp, z_format_1, ap) };
        ();
        unsafe { fprintf(__stdoutp, c"\n".as_ptr() as *mut i8 as *const i8) };
        unsafe { exit(1) };
    }
}

extern "C" fn hex_digit_value(c: i8) -> i32 {
    if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
        return c as i32 - '0' as i32;
    }
    if c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32 {
        return c as i32 - 'a' as i32 + 10;
    }
    if c as i32 >= 'A' as i32 && c as i32 <= 'F' as i32 {
        return c as i32 - 'A' as i32 + 10;
    }
    return -1;
}

extern "C" fn integer_value(mut z_arg_1: *const i8) -> i32 {
    unsafe {
        let mut v: i32 = 0;
        let mut i: i32 = 0;
        let mut is_neg: i32 = 0;
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '-' as i32 {
            is_neg = 1;
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '+' as i32
            {
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '0' as i32 &&
                unsafe { *z_arg_1.offset(1 as isize) } as i32 == 'x' as i32 {
            let mut x: i32 = 0;
            {
                let __n = 2;
                let __p = &mut z_arg_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            while {
                        x = hex_digit_value(unsafe { *z_arg_1.offset(0 as isize) });
                        x
                    } >= 0 {
                v = (v << 4) + x;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else {
            while unsafe { *z_arg_1.offset(0 as isize) } as i32 >= '0' as i32
                    &&
                    unsafe { *z_arg_1.offset(0 as isize) } as i32 <= '9' as i32
                {
                v =
                    v * 10 + unsafe { *z_arg_1.offset(0 as isize) } as i32 -
                        '0' as i32;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        {
            i = 0;
            '__b2: loop {
                if !((i as u64) <
                                core::mem::size_of::<[IntegerValueS0N16integerValueS0; 9]>()
                                        as u64 / 16) {
                    break '__b2;
                }
                '__c2: loop {
                    if unsafe {
                                sqlite3_stricmp(a_mult[i as usize].z_suffix as *const i8,
                                    z_arg_1)
                            } == 0 {
                        v *= a_mult[i as usize].i_mult as i32;
                        break '__b2;
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return if is_neg != 0 { -v } else { v };
    }
}

extern "C" fn path_type(z_path_1: *const i8) -> i32 {
    let mut x: Stat = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    if unsafe { access(z_path_1, 1 << 2) } != 0 { return 0; }
    unsafe {
        memset(&raw mut x as *mut (), 0, core::mem::size_of::<Stat>() as u64)
    };
    rc = unsafe { stat(z_path_1, &mut x) };
    if rc < 0 { return 99; }
    if x.st_mode as i32 & 61440 == 16384 {
        let z_layer1: *mut i8 =
            unsafe {
                sqlite3_mprintf(c"%s/00".as_ptr() as *mut i8 as *const i8,
                    z_path_1)
            };
        unsafe {
            memset(&raw mut x as *mut (), 0,
                core::mem::size_of::<Stat>() as u64)
        };
        rc = unsafe { stat(z_layer1 as *const i8, &mut x) };
        unsafe { sqlite3_free(z_layer1 as *mut ()) };
        if rc < 0 { return 1; }
        if x.st_mode as i32 & 61440 == 16384 { return 2; }
        return 1;
    }
    if x.st_size % 512 as OffT == 0 as i64 { return 3; }
    return 99;
}

extern "C" fn file_size(z_path_1: *const i8) -> Sqlite3Int64 {
    let mut x: Stat = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    unsafe {
        memset(&raw mut x as *mut (), 0, core::mem::size_of::<Stat>() as u64)
    };
    rc = unsafe { stat(z_path_1, &mut x) };
    if rc < 0 { return -1 as Sqlite3Int64; }
    if !(x.st_mode as i32 & 61440 == 32768) as i32 != 0 {
        return -1 as Sqlite3Int64;
    }
    return x.st_size;
}

extern "C" fn rand_int() -> u32 {
    unsafe {
        x_1 = x_1 >> 1 ^ 1 as u32 + !(x_1 & 1 as u32) & 3489660929u32;
        y = y * 1103515245 as u32 + 12345 as u32;
        return x_1 ^ y;
    }
}

extern "C" fn init_main(argc: i32, argv: *const *mut i8) -> i32 {
    let mut z_db: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut n_count: i32 = 1000;
    let mut sz: i32 = 10000;
    let mut i_variance: i32 = 0;
    let mut pgsz: i32 = 4096;
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let mut z_err_msg: *mut i8 = core::ptr::null_mut();
    if !(unsafe {
                                strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                                    c"init".as_ptr() as *mut i8 as *const i8)
                            } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"initMain".as_ptr() as *const i8,
                c"kvtest.c".as_ptr() as *mut i8 as *const i8, 325,
                c"strcmp(argv[1],\"init\")==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(argc >= 3) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"initMain".as_ptr() as *const i8,
                c"kvtest.c".as_ptr() as *mut i8 as *const i8, 326,
                c"argc>=3".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    z_db = unsafe { *argv.offset(2 as isize) };
    {
        i = 3;
        '__b3: loop {
            if !(i < argc) { break '__b3; }
            '__c3: loop {
                let mut z: *mut i8 = unsafe { *argv.offset(i as isize) };
                if unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 {
                    unsafe {
                        fatal_error(c"unknown argument: \"%s\"".as_ptr() as *mut i8
                                as *const i8, z)
                    };
                }
                if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-count".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    n_count =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    if n_count < 1 {
                        unsafe {
                            fatal_error(c"the --count must be positive".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                    }
                    break '__c3;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-size".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    sz =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    if sz < 1 {
                        unsafe {
                            fatal_error(c"the --size must be positive".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                    }
                    break '__c3;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-variance".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    i_variance =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    break '__c3;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-pagesize".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    pgsz =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    if pgsz < 512 || pgsz > 65536 || pgsz - 1 & pgsz != 0 {
                        unsafe {
                            fatal_error(c"the --pagesize must be power of 2 between 512 and 65536".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    }
                    break '__c3;
                }
                unsafe {
                    fatal_error(c"unknown option: \"%s\"".as_ptr() as *mut i8 as
                            *const i8, unsafe { *argv.offset(i as isize) })
                };
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    rc = unsafe { sqlite3_open(z_db as *const i8, &mut db) };
    if rc != 0 {
        unsafe {
            fatal_error(c"cannot open database \"%s\": %s".as_ptr() as *mut i8
                    as *const i8, z_db, unsafe { sqlite3_errmsg(db) })
        };
    }
    z_sql =
        unsafe {
            sqlite3_mprintf(c"DROP TABLE IF EXISTS kv;\nPRAGMA page_size=%d;\nVACUUM;\nBEGIN;\nCREATE TABLE kv(k INTEGER PRIMARY KEY, v BLOB);\nWITH RECURSIVE c(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM c WHERE x<%d) INSERT INTO kv(k,v) SELECT x, randomblob(%d+(random()%%(%d))) FROM c;\nCOMMIT;\n".as_ptr()
                        as *mut i8 as *const i8, pgsz, n_count, sz, i_variance + 1)
        };
    rc =
        unsafe {
            sqlite3_exec(db, z_sql as *const i8, None, core::ptr::null_mut(),
                &mut z_err_msg)
        };
    if rc != 0 {
        unsafe {
            fatal_error(c"database create failed: %s".as_ptr() as *mut i8 as
                    *const i8, z_err_msg)
        };
    }
    unsafe { sqlite3_free(z_sql as *mut ()) };
    unsafe { sqlite3_close(db) };
    return 0;
}

extern "C" fn stat_main(argc: i32, argv: *const *mut i8) -> i32 {
    unsafe {
        let mut z_db: *mut i8 = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut do_vacuum: i32 = 0;
        if !(unsafe {
                                    strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                                        c"stat".as_ptr() as *mut i8 as *const i8)
                                } == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"statMain".as_ptr() as *const i8,
                    c"kvtest.c".as_ptr() as *mut i8 as *const i8, 392,
                    c"strcmp(argv[1],\"stat\")==0".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !(argc >= 3) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"statMain".as_ptr() as *const i8,
                    c"kvtest.c".as_ptr() as *mut i8 as *const i8, 393,
                    c"argc>=3".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        z_db = unsafe { *argv.offset(2 as isize) };
        {
            i = 3;
            '__b4: loop {
                if !(i < argc) { break '__b4; }
                '__c4: loop {
                    let mut z: *mut i8 = unsafe { *argv.offset(i as isize) };
                    if unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 {
                        unsafe {
                            fatal_error(c"unknown argument: \"%s\"".as_ptr() as *mut i8
                                    as *const i8, z)
                        };
                    }
                    if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe {
                                strcmp(z as *const i8,
                                    c"-vacuum".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        do_vacuum = 1;
                        break '__c4;
                    }
                    unsafe {
                        fatal_error(c"unknown option: \"%s\"".as_ptr() as *mut i8 as
                                *const i8, unsafe { *argv.offset(i as isize) })
                    };
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        rc = unsafe { sqlite3_open(z_db as *const i8, &mut db) };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot open database \"%s\": %s".as_ptr() as
                            *mut i8 as *const i8, z_db, unsafe { sqlite3_errmsg(db) })
            };
        }
        if do_vacuum != 0 {
            unsafe {
                printf(c"Vacuuming....".as_ptr() as *mut i8 as *const i8)
            };
            unsafe { fflush(__stdoutp) };
            unsafe {
                sqlite3_exec(db, c"VACUUM".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe {
                printf(c"       done\n".as_ptr() as *mut i8 as *const i8)
            };
        }
        z_sql =
            unsafe {
                sqlite3_mprintf(c"SELECT count(*), min(length(v)), max(length(v)), avg(length(v))  FROM kv".as_ptr()
                            as *mut i8 as *const i8)
            };
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot prepare SQL [%s]: %s".as_ptr() as *mut i8
                        as *const i8, z_sql, unsafe { sqlite3_errmsg(db) })
            };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            unsafe {
                printf(c"Number of entries:  %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 0) })
            };
            unsafe {
                printf(c"Average value size: %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 3) })
            };
            unsafe {
                printf(c"Minimum value size: %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 1) })
            };
            unsafe {
                printf(c"Maximum value size: %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 2) })
            };
        } else {
            unsafe { printf(c"No rows\n".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        z_sql =
            unsafe {
                sqlite3_mprintf(c"PRAGMA page_size".as_ptr() as *mut i8 as
                        *const i8)
            };
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot prepare SQL [%s]: %s".as_ptr() as *mut i8
                        as *const i8, z_sql, unsafe { sqlite3_errmsg(db) })
            };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            unsafe {
                printf(c"Page-size:          %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 0) })
            };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        z_sql =
            unsafe {
                sqlite3_mprintf(c"PRAGMA page_count".as_ptr() as *mut i8 as
                        *const i8)
            };
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot prepare SQL [%s]: %s".as_ptr() as *mut i8
                        as *const i8, z_sql, unsafe { sqlite3_errmsg(db) })
            };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            unsafe {
                printf(c"Page-count:         %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 0) })
            };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        z_sql =
            unsafe {
                sqlite3_mprintf(c"PRAGMA freelist_count".as_ptr() as *mut i8
                        as *const i8)
            };
        rc =
            unsafe {
                sqlite3_prepare_v2(db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot prepare SQL [%s]: %s".as_ptr() as *mut i8
                        as *const i8, z_sql, unsafe { sqlite3_errmsg(db) })
            };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            unsafe {
                printf(c"Freelist-count:     %8d\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_int(p_stmt, 0) })
            };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        rc =
            unsafe {
                sqlite3_prepare_v2(db,
                    c"PRAGMA integrity_check(10)".as_ptr() as *mut i8 as
                        *const i8, -1, &mut p_stmt, core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot prepare integrity check: %s".as_ptr() as
                            *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
            };
        }
        while unsafe { sqlite3_step(p_stmt) } == 100 {
            unsafe {
                printf(c"Integrity-check:    %s\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_column_text(p_stmt, 0) })
            };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        unsafe { sqlite3_close(db) };
        return 0;
    }
}

extern "C" fn remember_func(p_ctx_1: *mut Sqlite3Context, argc: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut v: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut ptr: Sqlite3Int64 = 0 as Sqlite3Int64;
    if !(argc == 2) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"rememberFunc".as_ptr() as *const i8,
                c"kvtest.c".as_ptr() as *mut i8 as *const i8, 477,
                c"argc==2".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    v = unsafe { sqlite3_value_int64(unsafe { *argv.offset(0 as isize) }) };
    ptr = unsafe { sqlite3_value_int64(unsafe { *argv.offset(1 as isize) }) };
    unsafe { *(ptr as i64 as *mut () as *mut Sqlite3Int64) = v };
    unsafe { sqlite3_result_int64(p_ctx_1, v) };
}

extern "C" fn kvtest_mkdir(z_dir_1: *const i8) -> () {
    { let _ = unsafe { mkdir(z_dir_1, 493 as ModeT) }; };
}

extern "C" fn export_main(argc: i32, argv: *const *mut i8) -> i32 {
    unsafe {
        let mut z_db: *mut i8 = core::ptr::null_mut();
        let mut z_dir: *mut i8 = core::ptr::null_mut();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut e_path_type: i32 = 0;
        let mut n_fn: i32 = 0;
        let mut z_fn: *mut i8 = core::ptr::null_mut();
        let mut z_tail: *mut i8 = core::ptr::null_mut();
        let mut n_wrote: u64 = 0 as u64;
        let mut i: i32 = 0;
        if !(unsafe {
                                    strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                                        c"export".as_ptr() as *mut i8 as *const i8)
                                } == 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"exportMain".as_ptr() as *const i8,
                    c"kvtest.c".as_ptr() as *mut i8 as *const i8, 511,
                    c"strcmp(argv[1],\"export\")==0".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if !(argc >= 3) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"exportMain".as_ptr() as *const i8,
                    c"kvtest.c".as_ptr() as *mut i8 as *const i8, 512,
                    c"argc>=3".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if argc < 4 {
            unsafe {
                fatal_error(c"Usage: kvtest export DATABASE DIRECTORY [OPTIONS]".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        z_db = unsafe { *argv.offset(2 as isize) };
        z_dir = unsafe { *argv.offset(3 as isize) };
        kvtest_mkdir(z_dir as *const i8);
        {
            i = 4;
            '__b6: loop {
                if !(i < argc) { break '__b6; }
                '__c6: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 &&
                            unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    if unsafe {
                                strcmp(z, c"-tree".as_ptr() as *mut i8 as *const i8)
                            } == 0 {
                        z_fn =
                            unsafe {
                                sqlite3_mprintf(c"%s/00".as_ptr() as *mut i8 as *const i8,
                                    z_dir)
                            };
                        kvtest_mkdir(z_fn as *const i8);
                        unsafe { sqlite3_free(z_fn as *mut ()) };
                        break '__c6;
                    }
                    unsafe {
                        fatal_error(c"unknown argument: \"%s\"\n".as_ptr() as
                                    *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                    };
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        e_path_type = path_type(z_dir as *const i8);
        if e_path_type != 1 && e_path_type != 2 {
            unsafe {
                fatal_error(c"object \"%s\" is not a directory".as_ptr() as
                            *mut i8 as *const i8, z_dir)
            };
        }
        rc = unsafe { sqlite3_open(z_db as *const i8, &mut db) };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot open database \"%s\": %s".as_ptr() as
                            *mut i8 as *const i8, z_db, unsafe { sqlite3_errmsg(db) })
            };
        }
        rc =
            unsafe {
                sqlite3_prepare_v2(db,
                    c"SELECT k, v FROM kv ORDER BY k".as_ptr() as *mut i8 as
                        *const i8, -1, &mut p_stmt, core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                fatal_error(c"prepare_v2 failed: %s\n".as_ptr() as *mut i8 as
                        *const i8, unsafe { sqlite3_errmsg(db) })
            };
        }
        n_fn = unsafe { strlen(z_dir as *const i8) } as i32;
        z_fn =
            unsafe {
                sqlite3_mprintf(c"%s/00/00/00.extra---------------------".as_ptr()
                            as *mut i8 as *const i8, z_dir)
            };
        if z_fn == core::ptr::null_mut() {
            unsafe {
                fatal_error(c"malloc failed\n".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        z_tail =
            unsafe {
                unsafe { z_fn.offset(n_fn as isize).offset(1 as isize) }
            };
        while unsafe { sqlite3_step(p_stmt) } == 100 {
            let i_key: i32 = unsafe { sqlite3_column_int(p_stmt, 0) };
            let n_data: Sqlite3Int64 =
                unsafe { sqlite3_column_bytes(p_stmt, 1) } as Sqlite3Int64;
            let p_data: *const () = unsafe { sqlite3_column_blob(p_stmt, 1) };
            let mut out: *mut FILE = core::ptr::null_mut();
            if e_path_type == 1 {
                unsafe {
                    sqlite3_snprintf(20, z_tail,
                        c"%06d".as_ptr() as *mut i8 as *const i8, i_key)
                };
            } else {
                unsafe {
                    sqlite3_snprintf(20, z_tail,
                        c"%02d".as_ptr() as *mut i8 as *const i8, i_key / 10000)
                };
                kvtest_mkdir(z_fn as *const i8);
                unsafe {
                    sqlite3_snprintf(20, z_tail,
                        c"%02d/%02d".as_ptr() as *mut i8 as *const i8,
                        i_key / 10000, i_key / 100 % 100)
                };
                kvtest_mkdir(z_fn as *const i8);
                unsafe {
                    sqlite3_snprintf(20, z_tail,
                        c"%02d/%02d/%02d".as_ptr() as *mut i8 as *const i8,
                        i_key / 10000, i_key / 100 % 100, i_key % 100)
                };
            }
            out =
                unsafe {
                    fopen(z_fn as *const i8,
                        c"wb".as_ptr() as *mut i8 as *const i8)
                };
            n_wrote = unsafe { fwrite(p_data, 1 as u64, n_data as u64, out) };
            unsafe { fclose(out) };
            unsafe {
                printf(c"\r%s   ".as_ptr() as *mut i8 as *const i8, z_tail)
            };
            unsafe { fflush(__stdoutp) };
            if n_wrote != n_data as u64 {
                unsafe {
                    fatal_error(c"Wrote only %d of %d bytes to %s\n".as_ptr() as
                                *mut i8 as *const i8, n_wrote as i32, n_data, z_fn)
                };
            }
        }
        unsafe { sqlite3_finalize(p_stmt) };
        unsafe { sqlite3_close(db) };
        unsafe { sqlite3_free(z_fn as *mut ()) };
        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
        return 0;
    }
}

extern "C" fn read_file(z_name_1: *const i8, pn_byte_1: *mut Sqlite3Int64)
    -> *mut u8 {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut n_in: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_read: u64 = 0 as u64;
    let mut p_buf: *mut u8 = core::ptr::null_mut();
    n_in = file_size(z_name_1);
    if n_in < 0 as i64 { return core::ptr::null_mut(); }
    in_ = unsafe { fopen(z_name_1, c"rb".as_ptr() as *mut i8 as *const i8) };
    if in_ == core::ptr::null_mut() { return core::ptr::null_mut(); }
    p_buf = unsafe { sqlite3_malloc64(n_in as Sqlite3Uint64) } as *mut u8;
    if p_buf == core::ptr::null_mut() { return core::ptr::null_mut(); }
    n_read = unsafe { fread(p_buf as *mut (), n_in as u64, 1 as u64, in_) };
    unsafe { fclose(in_) };
    if n_read != 1 as u64 {
        unsafe { sqlite3_free(p_buf as *mut ()) };
        return core::ptr::null_mut();
    }
    if !(pn_byte_1).is_null() { unsafe { *pn_byte_1 = n_in }; }
    return p_buf;
}

extern "C" fn update_file(z_name_1: *const i8, pn_byte_1: &mut Sqlite3Int64,
    do_fsync_1: i32) -> () {
    let mut out: *mut FILE = core::ptr::null_mut();
    let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_written: u64 = 0 as u64;
    let mut p_buf: *mut u8 = core::ptr::null_mut();
    let z_mode: *const i8 = c"wb".as_ptr() as *mut i8 as *const i8;
    sz = file_size(z_name_1);
    if sz < 0 as i64 {
        unsafe {
            fatal_error(c"No such file: \"%s\"".as_ptr() as *mut i8 as
                    *const i8, z_name_1)
        };
    }
    *pn_byte_1 = sz;
    if sz == 0 as i64 { return; }
    p_buf = unsafe { sqlite3_malloc64(sz as Sqlite3Uint64) } as *mut u8;
    if p_buf == core::ptr::null_mut() {
        unsafe {
            fatal_error(c"Cannot allocate %lld bytes\n".as_ptr() as *mut i8 as
                    *const i8, sz)
        };
    }
    unsafe { sqlite3_randomness(sz as i32, p_buf as *mut ()) };
    out = unsafe { fopen(z_name_1, z_mode) };
    if out == core::ptr::null_mut() {
        unsafe {
            fatal_error(c"Cannot open \"%s\" for writing\n".as_ptr() as
                        *mut i8 as *const i8, z_name_1)
        };
    }
    n_written =
        unsafe { fwrite(p_buf as *const (), 1 as u64, sz as u64, out) };
    if do_fsync_1 != 0 { unsafe { fsync(unsafe { fileno(out) }) }; }
    unsafe { fclose(out) };
    if n_written != sz as u64 {
        unsafe {
            fatal_error(c"Wrote only %d of %d bytes to \"%s\"\n".as_ptr() as
                        *mut i8 as *const i8, n_written as i32, sz as i32, z_name_1)
        };
    }
    unsafe { sqlite3_free(p_buf as *mut ()) };
}

extern "C" fn time_of_day() -> Sqlite3Int64 {
    unsafe {
        let mut t: Sqlite3Int64 = 0 as Sqlite3Int64;
        if clock_vfs == core::ptr::null_mut() {
            clock_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        }
        if unsafe { (*clock_vfs).i_version } >= 2 &&
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

extern "C" fn display_stats(db: *mut Sqlite3, b_reset_1: i32) -> i32 {
    unsafe {
        let mut i_cur: i32 = 0;
        let mut i_hiwtr: i32 = 0;
        let out: *mut FILE = __stdoutp;
        unsafe { fprintf(out, c"\n".as_ptr() as *mut i8 as *const i8) };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_status(0, &mut i_cur, &mut i_hiwtr, b_reset_1) };
        unsafe {
            fprintf(out,
                c"Memory Used:                         %d (max %d) bytes\n".as_ptr()
                        as *mut i8 as *const i8, i_cur, i_hiwtr)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_status(9, &mut i_cur, &mut i_hiwtr, b_reset_1) };
        unsafe {
            fprintf(out,
                c"Number of Outstanding Allocations:   %d (max %d)\n".as_ptr()
                        as *mut i8 as *const i8, i_cur, i_hiwtr)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_status(1, &mut i_cur, &mut i_hiwtr, b_reset_1) };
        unsafe {
            fprintf(out,
                c"Number of Pcache Pages Used:         %d (max %d) pages\n".as_ptr()
                        as *mut i8 as *const i8, i_cur, i_hiwtr)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_status(2, &mut i_cur, &mut i_hiwtr, b_reset_1) };
        unsafe {
            fprintf(out,
                c"Number of Pcache Overflow Bytes:     %d (max %d) bytes\n".as_ptr()
                        as *mut i8 as *const i8, i_cur, i_hiwtr)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_status(5, &mut i_cur, &mut i_hiwtr, b_reset_1) };
        unsafe {
            fprintf(out,
                c"Largest Allocation:                  %d bytes\n".as_ptr() as
                        *mut i8 as *const i8, i_hiwtr)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_status(7, &mut i_cur, &mut i_hiwtr, b_reset_1) };
        unsafe {
            fprintf(out,
                c"Largest Pcache Allocation:           %d bytes\n".as_ptr() as
                        *mut i8 as *const i8, i_hiwtr)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe {
            sqlite3_db_status(db, 1, &mut i_cur, &mut i_hiwtr, b_reset_1)
        };
        unsafe {
            fprintf(out,
                c"Pager Heap Usage:                    %d bytes\n".as_ptr() as
                        *mut i8 as *const i8, i_cur)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_db_status(db, 7, &mut i_cur, &mut i_hiwtr, 1) };
        unsafe {
            fprintf(out,
                c"Page cache hits:                     %d\n".as_ptr() as
                        *mut i8 as *const i8, i_cur)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_db_status(db, 8, &mut i_cur, &mut i_hiwtr, 1) };
        unsafe {
            fprintf(out,
                c"Page cache misses:                   %d\n".as_ptr() as
                        *mut i8 as *const i8, i_cur)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        unsafe { sqlite3_db_status(db, 9, &mut i_cur, &mut i_hiwtr, 1) };
        unsafe {
            fprintf(out,
                c"Page cache writes:                   %d\n".as_ptr() as
                        *mut i8 as *const i8, i_cur)
        };
        i_hiwtr = { i_cur = -1; i_cur };
        return 0;
    }
}

extern "C" fn run_main(argc: i32, argv: *const *mut i8) -> i32 {
    let mut e_type: i32 = 0;
    let mut z_db: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    let mut n_count: i32 = 1000;
    let mut n_extra: i32 = 0;
    let mut i_key: i32 = 1;
    let mut i_max: i32 = 0;
    let mut i_pagesize: i32 = 0;
    let mut i_cache: i32 = 1000;
    let mut b_blob_api: i32 = 0;
    let mut b_stats: i32 = 0;
    let mut e_order: i32 = 1;
    let mut is_update_test: i32 = 0;
    let mut do_integrity_ck: i32 = 0;
    let mut no_sync: i32 = 0;
    let mut do_fsync: i32 = 0;
    let mut do_multi_trans: i32 = 0;
    let mut no_checkpoint: i32 = 0;
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    let mut p_blob: *mut Sqlite3Blob = core::ptr::null_mut();
    let mut tm_start: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut tm_elapsed: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut mmap_size: i32 = 0;
    let mut n_data: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut n_total: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut p_data: *mut u8 = core::ptr::null_mut();
    let mut n_alloc: Sqlite3Int64 = 0 as Sqlite3Int64;
    let mut z_j_mode: *const i8 = core::ptr::null();
    if !(unsafe {
                                strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                                    c"run".as_ptr() as *mut i8 as *const i8)
                            } == 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"runMain".as_ptr() as *const i8,
                c"kvtest.c".as_ptr() as *mut i8 as *const i8, 818,
                c"strcmp(argv[1],\"run\")==0".as_ptr() as *mut i8 as
                    *const i8)
        }
    } else { { let _ = 0; } };
    if !(argc >= 3) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"runMain".as_ptr() as *const i8,
                c"kvtest.c".as_ptr() as *mut i8 as *const i8, 819,
                c"argc>=3".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    z_db = unsafe { *argv.offset(2 as isize) };
    e_type = path_type(z_db as *const i8);
    if e_type == 99 {
        unsafe {
            fatal_error(c"unknown object type: \"%s\"".as_ptr() as *mut i8 as
                    *const i8, z_db)
        };
    }
    if e_type == 0 {
        unsafe {
            fatal_error(c"object does not exist: \"%s\"".as_ptr() as *mut i8
                    as *const i8, z_db)
        };
    }
    {
        i = 3;
        '__b8: loop {
            if !(i < argc) { break '__b8; }
            '__c8: loop {
                let mut z: *mut i8 = unsafe { *argv.offset(i as isize) };
                if unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 {
                    unsafe {
                        fatal_error(c"unknown argument: \"%s\"".as_ptr() as *mut i8
                                as *const i8, z)
                    };
                }
                if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-asc".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    e_order = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-blob-api".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    b_blob_api = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-cache-size".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    i_cache =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-count".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    n_count =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    if n_count < 1 {
                        unsafe {
                            fatal_error(c"the --count must be positive".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                    }
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-desc".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    e_order = 2;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-fsync".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    do_fsync = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-integrity-check".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    do_integrity_ck = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-jmode".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    z_j_mode =
                        unsafe {
                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                            } as *const i8;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-mmap".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    mmap_size =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    if n_count < 0 {
                        unsafe {
                            fatal_error(c"the --mmap must be non-negative".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                    }
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-max-id".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    i_max =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-multitrans".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    do_multi_trans = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-nocheckpoint".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    no_checkpoint = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-nosync".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    no_sync = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-random".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    e_order = 3;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-start".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            fatal_error(c"missing argument on \"%s\"".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                    }
                    i_key =
                        integer_value(unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                } as *const i8);
                    if i_key < 1 {
                        unsafe {
                            fatal_error(c"the --start must be positive".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                    }
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-stats".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    b_stats = 1;
                    break '__c8;
                }
                if unsafe {
                            strcmp(z as *const i8,
                                c"-update".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    is_update_test = 1;
                    break '__c8;
                }
                unsafe {
                    fatal_error(c"unknown option: \"%s\"".as_ptr() as *mut i8 as
                            *const i8, unsafe { *argv.offset(i as isize) })
                };
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if e_type == 3 {
        unsafe { sqlite3_open(z_db as *const i8, &mut db) };
        unsafe {
            sqlite3_exec(db,
                c"SELECT rowid FROM sqlite_schema LIMIT 1".as_ptr() as *mut i8
                    as *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
        unsafe { sqlite3_close(db) };
        db = core::ptr::null_mut();
    }
    tm_start = time_of_day();
    if e_type == 3 {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        rc = unsafe { sqlite3_open(z_db as *const i8, &mut db) };
        if rc != 0 {
            unsafe {
                fatal_error(c"cannot open database \"%s\": %s".as_ptr() as
                            *mut i8 as *const i8, z_db, unsafe { sqlite3_errmsg(db) })
            };
        }
        z_sql =
            unsafe {
                sqlite3_mprintf(c"PRAGMA mmap_size=%d".as_ptr() as *mut i8 as
                        *const i8, mmap_size)
            };
        unsafe {
            sqlite3_exec(db, z_sql as *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        z_sql =
            unsafe {
                sqlite3_mprintf(c"PRAGMA cache_size=%d".as_ptr() as *mut i8 as
                        *const i8, i_cache)
            };
        unsafe {
            sqlite3_exec(db, z_sql as *const i8, None, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if no_sync != 0 {
            unsafe {
                sqlite3_exec(db,
                    c"PRAGMA synchronous=OFF".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
        p_stmt = core::ptr::null_mut();
        unsafe {
            sqlite3_prepare_v2(db,
                c"PRAGMA page_size".as_ptr() as *mut i8 as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            i_pagesize = unsafe { sqlite3_column_int(p_stmt, 0) };
        }
        unsafe { sqlite3_finalize(p_stmt) };
        unsafe {
            sqlite3_prepare_v2(db,
                c"PRAGMA cache_size".as_ptr() as *mut i8 as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            i_cache = unsafe { sqlite3_column_int(p_stmt, 0) };
        } else { i_cache = 0; }
        unsafe { sqlite3_finalize(p_stmt) };
        p_stmt = core::ptr::null_mut();
        if !(z_j_mode).is_null() {
            z_sql =
                unsafe {
                    sqlite3_mprintf(c"PRAGMA journal_mode=%Q".as_ptr() as
                                *mut i8 as *const i8, z_j_mode)
                };
            unsafe {
                sqlite3_exec(db, z_sql as *const i8, None,
                    core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe { sqlite3_free(z_sql as *mut ()) };
            if no_checkpoint != 0 {
                unsafe {
                    sqlite3_exec(db,
                        c"PRAGMA wal_autocheckpoint=0".as_ptr() as *mut i8 as
                            *const i8, None, core::ptr::null_mut(),
                        core::ptr::null_mut())
                };
            }
        }
        unsafe {
            sqlite3_prepare_v2(db,
                c"PRAGMA journal_mode".as_ptr() as *mut i8 as *const i8, -1,
                &mut p_stmt, core::ptr::null_mut())
        };
        if unsafe { sqlite3_step(p_stmt) } == 100 {
            z_j_mode =
                unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_column_text(p_stmt, 0) })
                    } as *const i8;
        } else { z_j_mode = c"???".as_ptr() as *mut i8 as *const i8; }
        unsafe { sqlite3_finalize(p_stmt) };
        if i_max <= 0 {
            unsafe {
                sqlite3_prepare_v2(db,
                    c"SELECT max(k) FROM kv".as_ptr() as *mut i8 as *const i8,
                    -1, &mut p_stmt, core::ptr::null_mut())
            };
            if unsafe { sqlite3_step(p_stmt) } == 100 {
                i_max = unsafe { sqlite3_column_int(p_stmt, 0) };
            }
            unsafe { sqlite3_finalize(p_stmt) };
        }
        p_stmt = core::ptr::null_mut();
        if (do_multi_trans == 0) as i32 != 0 {
            unsafe {
                sqlite3_exec(db, c"BEGIN".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
    }
    if i_max <= 0 { i_max = 1000; }
    {
        i = 0;
        '__b9: loop {
            if !(i < n_count) { break '__b9; }
            '__c9: loop {
                if e_type == 1 || e_type == 2 {
                    let mut z_key: *mut i8 = core::ptr::null_mut();
                    if e_type == 1 {
                        z_key =
                            unsafe {
                                sqlite3_mprintf(c"%s/%06d".as_ptr() as *mut i8 as *const i8,
                                    z_db, i_key)
                            };
                    } else {
                        z_key =
                            unsafe {
                                sqlite3_mprintf(c"%s/%02d/%02d/%02d".as_ptr() as *mut i8 as
                                        *const i8, z_db, i_key / 10000, i_key / 100 % 100,
                                    i_key % 100)
                            };
                    }
                    n_data = 0 as Sqlite3Int64;
                    if is_update_test != 0 {
                        update_file(z_key as *const i8, &mut n_data, do_fsync);
                    } else {
                        p_data = read_file(z_key as *const i8, &mut n_data);
                        unsafe { sqlite3_free(p_data as *mut ()) };
                    }
                    unsafe { sqlite3_free(z_key as *mut ()) };
                } else if b_blob_api != 0 {
                    if p_blob == core::ptr::null_mut() {
                        rc =
                            unsafe {
                                sqlite3_blob_open(db,
                                    c"main".as_ptr() as *mut i8 as *const i8,
                                    c"kv".as_ptr() as *mut i8 as *const i8,
                                    c"v".as_ptr() as *mut i8 as *const i8,
                                    i_key as Sqlite3Int64, is_update_test, &mut p_blob)
                            };
                        if rc != 0 {
                            unsafe {
                                fatal_error(c"could not open sqlite3_blob handle: %s".as_ptr()
                                            as *mut i8 as *const i8, unsafe { sqlite3_errmsg(db) })
                            };
                        }
                    } else {
                        rc =
                            unsafe {
                                sqlite3_blob_reopen(p_blob, i_key as Sqlite3Int64)
                            };
                    }
                    if rc == 0 {
                        n_data =
                            unsafe { sqlite3_blob_bytes(p_blob) } as Sqlite3Int64;
                        if n_alloc < n_data + 1 as Sqlite3Int64 {
                            n_alloc = n_data + 100 as Sqlite3Int64;
                            p_data =
                                unsafe {
                                        sqlite3_realloc64(p_data as *mut (),
                                            n_alloc as Sqlite3Uint64)
                                    } as *mut u8;
                        }
                        if p_data == core::ptr::null_mut() {
                            unsafe {
                                fatal_error(c"cannot allocate %d bytes".as_ptr() as *mut i8
                                        as *const i8, n_data + 1 as Sqlite3Int64)
                            };
                        }
                        if is_update_test != 0 {
                            unsafe {
                                sqlite3_randomness(n_data as i32, p_data as *mut ())
                            };
                            rc =
                                unsafe {
                                    sqlite3_blob_write(p_blob, p_data as *const (),
                                        n_data as i32, 0)
                                };
                            if rc != 0 {
                                unsafe {
                                    fatal_error(c"could not write the blob at %d: %s".as_ptr()
                                                as *mut i8 as *const i8, i_key,
                                        unsafe { sqlite3_errmsg(db) })
                                };
                            }
                        } else {
                            rc =
                                unsafe {
                                    sqlite3_blob_read(p_blob, p_data as *mut (), n_data as i32,
                                        0)
                                };
                            if rc != 0 {
                                unsafe {
                                    fatal_error(c"could not read the blob at %d: %s".as_ptr() as
                                                *mut i8 as *const i8, i_key, unsafe { sqlite3_errmsg(db) })
                                };
                            }
                        }
                    }
                } else {
                    if p_stmt == core::ptr::null_mut() {
                        if is_update_test != 0 {
                            unsafe {
                                sqlite3_create_function(db,
                                    c"remember".as_ptr() as *mut i8 as *const i8, 2, 1,
                                    core::ptr::null_mut(), Some(remember_func), None, None)
                            };
                            rc =
                                unsafe {
                                    sqlite3_prepare_v2(db,
                                        c"UPDATE kv SET v=randomblob(remember(length(v),?2)) WHERE k=?1".as_ptr()
                                                as *mut i8 as *const i8, -1, &mut p_stmt,
                                        core::ptr::null_mut())
                                };
                            unsafe {
                                sqlite3_bind_int64(p_stmt, 2,
                                    &raw mut n_data as i64 as Sqlite3Int64)
                            };
                        } else {
                            rc =
                                unsafe {
                                    sqlite3_prepare_v2(db,
                                        c"SELECT v FROM kv WHERE k=?1".as_ptr() as *mut i8 as
                                            *const i8, -1, &mut p_stmt, core::ptr::null_mut())
                                };
                        }
                        if rc != 0 {
                            unsafe {
                                fatal_error(c"cannot prepare query: %s".as_ptr() as *mut i8
                                        as *const i8, unsafe { sqlite3_errmsg(db) })
                            };
                        }
                    } else { unsafe { sqlite3_reset(p_stmt) }; }
                    unsafe { sqlite3_bind_int(p_stmt, 1, i_key) };
                    n_data = 0 as Sqlite3Int64;
                    rc = unsafe { sqlite3_step(p_stmt) };
                    if rc == 100 {
                        n_data =
                            unsafe { sqlite3_column_bytes(p_stmt, 0) } as Sqlite3Int64;
                        p_data =
                            unsafe { sqlite3_column_blob(p_stmt, 0) } as *mut u8;
                    }
                }
                if e_order == 1 {
                    { let __p = &mut i_key; let __t = *__p; *__p += 1; __t };
                    if i_key > i_max { i_key = 1; }
                } else if e_order == 2 {
                    { let __p = &mut i_key; let __t = *__p; *__p -= 1; __t };
                    if i_key <= 0 { i_key = i_max; }
                } else {
                    i_key = (rand_int() % i_max as u32 + 1 as u32) as i32;
                }
                n_total += n_data;
                if n_data == 0 as i64 {
                    { let __p = &mut n_count; let __t = *__p; *__p += 1; __t };
                    { let __p = &mut n_extra; let __t = *__p; *__p += 1; __t };
                }
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if n_alloc != 0 { unsafe { sqlite3_free(p_data as *mut ()) }; }
    if !(p_stmt).is_null() { unsafe { sqlite3_finalize(p_stmt) }; }
    if !(p_blob).is_null() { unsafe { sqlite3_blob_close(p_blob) }; }
    if b_stats != 0 { display_stats(db, 0); }
    if !(db).is_null() {
        if (do_multi_trans == 0) as i32 != 0 {
            unsafe {
                sqlite3_exec(db, c"COMMIT".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
        if (no_checkpoint == 0) as i32 != 0 {
            unsafe { sqlite3_close(db) };
            db = core::ptr::null_mut();
        }
    }
    tm_elapsed = time_of_day() - tm_start;
    if !(db).is_null() && no_checkpoint != 0 {
        unsafe { sqlite3_close(db) };
        db = core::ptr::null_mut();
    }
    if n_extra != 0 {
        unsafe {
            printf(c"%d cycles due to %d misses\n".as_ptr() as *mut i8 as
                    *const i8, n_count, n_extra)
        };
    }
    if e_type == 3 {
        unsafe {
            printf(c"SQLite version: %s\n".as_ptr() as *mut i8 as *const i8,
                unsafe { sqlite3_libversion() })
        };
        if do_integrity_ck != 0 {
            unsafe { sqlite3_open(z_db as *const i8, &mut db) };
            unsafe {
                sqlite3_prepare_v2(db,
                    c"PRAGMA integrity_check".as_ptr() as *mut i8 as *const i8,
                    -1, &mut p_stmt, core::ptr::null_mut())
            };
            while unsafe { sqlite3_step(p_stmt) } == 100 {
                unsafe {
                    printf(c"integrity-check: %s\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { sqlite3_column_text(p_stmt, 0) })
                };
            }
            unsafe { sqlite3_finalize(p_stmt) };
            unsafe { sqlite3_close(db) };
            db = core::ptr::null_mut();
        }
    }
    unsafe {
        printf(c"--count %d --max-id %d".as_ptr() as *mut i8 as *const i8,
            n_count - n_extra, i_max)
    };
    '__s11:
        {
        match e_order {
            3 => {
                unsafe {
                    printf(c" --random\n".as_ptr() as *mut i8 as *const i8)
                };
            }
            2 => {
                unsafe {
                    printf(c" --desc\n".as_ptr() as *mut i8 as *const i8)
                };
            }
            _ => {
                unsafe {
                    printf(c" --asc\n".as_ptr() as *mut i8 as *const i8)
                };
            }
        }
    }
    if e_type == 3 {
        unsafe {
            printf(c"--cache-size %d --jmode %s\n".as_ptr() as *mut i8 as
                    *const i8, i_cache, z_j_mode)
        };
        unsafe {
            printf(c"--mmap %d%s\n".as_ptr() as *mut i8 as *const i8,
                mmap_size,
                if b_blob_api != 0 {
                    c" --blob-api".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 })
        };
        if no_sync != 0 {
            unsafe { printf(c"--nosync\n".as_ptr() as *mut i8 as *const i8) };
        }
    }
    if i_pagesize != 0 {
        unsafe {
            printf(c"Database page size: %d\n".as_ptr() as *mut i8 as
                    *const i8, i_pagesize)
        };
    }
    unsafe {
        printf(c"Total elapsed time: %.3f\n".as_ptr() as *mut i8 as *const i8,
            tm_elapsed as f64 / 1000.0)
    };
    if is_update_test != 0 {
        unsafe {
            printf(c"Microseconds per BLOB write: %.3f\n".as_ptr() as *mut i8
                    as *const i8, tm_elapsed as f64 * 1000.0 / n_count as f64)
        };
        unsafe {
            printf(c"Content write rate: %.1f MB/s\n".as_ptr() as *mut i8 as
                    *const i8, n_total as f64 / (1000.0 * tm_elapsed as f64))
        };
    } else {
        unsafe {
            printf(c"Microseconds per BLOB read: %.3f\n".as_ptr() as *mut i8
                    as *const i8, tm_elapsed as f64 * 1000.0 / n_count as f64)
        };
        unsafe {
            printf(c"Content read rate: %.1f MB/s\n".as_ptr() as *mut i8 as
                    *const i8, n_total as f64 / (1000.0 * tm_elapsed as f64))
        };
    }
    return 0;
}

extern "C" fn __main_inner(argc: i32, argv: *mut *mut i8) -> Result<(), i32> {
    if argc < 3 { show_help(); }
    if unsafe {
                strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"init".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return Err(init_main(argc, argv as *const *mut i8));
    }
    if unsafe {
                strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"export".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return Err(export_main(argc, argv as *const *mut i8));
    }
    if unsafe {
                strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"run".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return Err(run_main(argc, argv as *const *mut i8));
    }
    if unsafe {
                strcmp(unsafe { *argv.offset(1 as isize) } as *const i8,
                    c"stat".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return Err(stat_main(argc, argv as *const *mut i8));
    }
    show_help();
    return Ok(());
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IntegerValueS0N16integerValueS0 {
    z_suffix: *mut i8,
    i_mult: i32,
}

static mut a_mult: [IntegerValueS0N16integerValueS0; 9] =
    [IntegerValueS0N16integerValueS0 {
                z_suffix: c"KiB".as_ptr() as *mut i8,
                i_mult: 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"MiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"GiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024 * 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"KB".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"MB".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"GB".as_ptr() as *mut i8,
                i_mult: 1000000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"K".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"M".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"G".as_ptr() as *mut i8,
                i_mult: 1000000000,
            }];

static mut x_1: u32 = 859444173 as u32;

static mut y: u32 = 3971132906u32;

static mut clock_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn access(_: *const i8, _: i32)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn stat(_: *const i8, _: *mut Stat)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn mkdir(_: *const i8, _: ModeT)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fileno(_: *mut FILE)
    -> i32;
    fn fsync(_: i32)
    -> i32;
    static mut __stdoutp: *mut FILE;
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
